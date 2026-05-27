use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reference particle / tracer flow calibration station for optical-flow
// and blockage-detection checks.
//
// Intent:
// - Hold traceable reference-particle vials with lot/certificate custody before
//   they enter any closed fluid path.
// - Keep particles suspended through a rocking cradle and staged loop parking so
//   settling is physically discouraged before injection.
// - Package a sterile injection loop, optical window target, blockage standard,
//   flush/waste routing, release/hold/reject lanes, and robot/service keepouts
//   as visible mechanical interfaces.
//
// This is packaging/interface CAD only. It does not specify particle chemistry,
// sterile barrier validation, optical thresholds, calibration equations, or
// release criteria.

const OUTPUTS: &[&str] = &[
    "output/closed_reference_particle_flow_calibration_station_base_leak_tray.stl",
    "output/closed_reference_particle_flow_calibration_station_reference_particle_vial_custody.stl",
    "output/closed_reference_particle_flow_calibration_station_mixing_settling_prevention_cradle.stl",
    "output/closed_reference_particle_flow_calibration_station_sterile_injection_loop.stl",
    "output/closed_reference_particle_flow_calibration_station_optical_window_calibration_target.stl",
    "output/closed_reference_particle_flow_calibration_station_flow_blockage_detection_panel.stl",
    "output/closed_reference_particle_flow_calibration_station_waste_flush_route.stl",
    "output/closed_reference_particle_flow_calibration_station_barcode_certificate_lands.stl",
    "output/closed_reference_particle_flow_calibration_station_release_hold_reject_lanes.stl",
    "output/closed_reference_particle_flow_calibration_station_particle_trap_and_clean_used_segregation.stl",
    "output/closed_reference_particle_flow_calibration_station_robot_service_keepouts.stl",
    "output/closed_reference_particle_flow_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "reference_particle_vial_custody",
    "mixing_settling_prevention_cradle",
    "sterile_injection_loop",
    "optical_window_calibration_target",
    "flow_blockage_detection_panel",
    "waste_flush_route",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "particle_trap_and_clean_used_segregation",
    "base_leak_tray",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;

const VIAL_RACK_X: f64 = 310.0;
const VIAL_RACK_Y: f64 = 178.0;
const VIAL_RACK_Z: f64 = 58.0;
const VIAL_POS: (f64, f64) = (-398.0, 224.0);
const VIAL_ROWS: usize = 2;
const VIAL_COLS: usize = 5;
const VIAL_COUNT: usize = VIAL_ROWS * VIAL_COLS;
const VIAL_PITCH_X: f64 = 48.0;
const VIAL_PITCH_Y: f64 = 56.0;
const VIAL_BORE_D: f64 = 17.5;
const CUSTODY_SEAL_LANDS: usize = 10;

const MIX_CRADLE_X: f64 = 350.0;
const MIX_CRADLE_Y: f64 = 186.0;
const MIX_CRADLE_Z: f64 = 70.0;
const MIX_POS: (f64, f64) = (-30.0, 226.0);
const ROCKING_ROLLERS: usize = 4;
const CRADLE_VIAL_SLOTS: usize = 8;
const CRADLE_ROLLER_PITCH: f64 = 68.0;
const MIXING_TILT_STOPS: usize = 3;
const SETTLING_PAUSE_MAX_MIN: f64 = 2.0;

const INJECTION_X: f64 = 430.0;
const INJECTION_Y: f64 = 150.0;
const INJECTION_Z: f64 = 54.0;
const INJECTION_POS: (f64, f64) = (-342.0, 28.0);
const STERILE_LOOP_COUNT: usize = 4;
const INJECTION_PORT_COUNT: usize = 8;
const LOOP_CLAMP_COUNT: usize = 8;
const LOOP_PITCH_X: f64 = 88.0;
const LUER_CLEARANCE_D: f64 = 10.4;
const LOOP_CHANNEL_D: f64 = 5.6;

const OPTICAL_TARGET_X: f64 = 330.0;
const OPTICAL_TARGET_Y: f64 = 186.0;
const OPTICAL_TARGET_Z: f64 = 78.0;
const OPTICAL_POS: (f64, f64) = (388.0, 226.0);
const OPTICAL_WINDOW_COUNT: usize = 4;
const OPTICAL_TARGET_MARKS: usize = 12;
const WINDOW_X: f64 = 54.0;
const WINDOW_Y: f64 = 30.0;
const CAMERA_DATUM_COUNT: usize = 4;

const FLOW_PANEL_X: f64 = 382.0;
const FLOW_PANEL_Y: f64 = 158.0;
const FLOW_PANEL_Z: f64 = 42.0;
const FLOW_POS: (f64, f64) = (122.0, 28.0);
const FLOW_LANES: usize = 4;
const BLOCKAGE_STANDARDS: usize = 4;
const PRESSURE_TAP_COUNT: usize = 8;
const FLOW_LANE_PITCH: f64 = 68.0;
const BLOCKAGE_APERTURE_D: f64 = 3.2;

const WASTE_ROUTE_X: f64 = 260.0;
const WASTE_ROUTE_Y: f64 = 150.0;
const WASTE_ROUTE_Z: f64 = 52.0;
const WASTE_POS: (f64, f64) = (430.0, 24.0);
const FLUSH_PORT_COUNT: usize = 6;
const WASTE_CHANNEL_COUNT: usize = 6;
const PARTICLE_TRAP_COUNT: usize = 3;
const FLUSH_TUBE_BORE_D: f64 = 6.0;

const TRACE_PANEL_X: f64 = 320.0;
const TRACE_PANEL_Y: f64 = 106.0;
const TRACE_PANEL_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (-402.0, -222.0);
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 4;
const VIAL_RETAIN_LANDS: usize = 2;

const STATUS_LANES_X: f64 = 432.0;
const STATUS_LANES_Y: f64 = 142.0;
const STATUS_LANES_Z: f64 = 44.0;
const STATUS_POS: (f64, f64) = (8.0, -220.0);
const RELEASE_LANES: usize = 3;
const SLOTS_PER_STATUS_LANE: usize = 6;
const STATUS_SLOT_PITCH_X: f64 = 54.0;
const STATUS_LANE_PITCH_Y: f64 = 42.0;

const SEGREGATION_X: f64 = 260.0;
const SEGREGATION_Y: f64 = 124.0;
const SEGREGATION_Z: f64 = 48.0;
const SEGREGATION_POS: (f64, f64) = (434.0, -224.0);
const CLEAN_USED_MIN_GAP: f64 = 32.0;
const CAP_PARK_POSTS: usize = 12;
const USED_PARTICLE_CUPS: usize = 6;

const LEAK_CHANNEL_COUNT: usize = 5;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 360.0;
const REAR_VIAL_SERVICE_CLEARANCE: f64 = 240.0;
const RIGHT_OPTICAL_SERVICE_CLEARANCE: f64 = 180.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let vial_custody = reference_particle_vial_custody();
    export(OUTPUTS[1], &vial_custody);

    let mixing = mixing_settling_prevention_cradle();
    export(OUTPUTS[2], &mixing);

    let injection = sterile_injection_loop();
    export(OUTPUTS[3], &injection);

    let optical = optical_window_calibration_target();
    export(OUTPUTS[4], &optical);

    let flow = flow_blockage_detection_panel();
    export(OUTPUTS[5], &flow);

    let waste = waste_flush_route();
    export(OUTPUTS[6], &waste);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[7], &trace);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[8], &status);

    let segregation = particle_trap_and_clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + vial_custody.translate(VIAL_POS.0, VIAL_POS.1, insert_z(VIAL_RACK_Z))
        + mixing.translate(MIX_POS.0, MIX_POS.1, insert_z(MIX_CRADLE_Z))
        + injection.translate(INJECTION_POS.0, INJECTION_POS.1, insert_z(INJECTION_Z))
        + optical.translate(OPTICAL_POS.0, OPTICAL_POS.1, insert_z(OPTICAL_TARGET_Z))
        + flow.translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_PANEL_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_ROUTE_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z))
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_LANES_Z))
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + 3.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed reference particle flow calibration station:");
    println!(
        "  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm contained leak tray"
    );
    println!(
        "  Custody:                    {VIAL_COUNT} reference-particle vial wells, {CUSTODY_SEAL_LANDS} custody seal lands, {BARCODE_LANDS} barcode lands, and {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Anti-settling:              {ROCKING_ROLLERS} rocking rollers, {CRADLE_VIAL_SLOTS} cradle slots, {MIXING_TILT_STOPS} tilt stops, max staged pause {SETTLING_PAUSE_MAX_MIN:.0} min"
    );
    println!(
        "  Closed-path calibration:    {STERILE_LOOP_COUNT} sterile injection loops, {INJECTION_PORT_COUNT} injection ports, {FLOW_LANES} optical-flow lanes, {BLOCKAGE_STANDARDS} blockage standards"
    );
    println!(
        "  Optical/waste controls:     {OPTICAL_WINDOW_COUNT} optical windows, {OPTICAL_TARGET_MARKS} target marks, {FLUSH_PORT_COUNT} flush ports, {WASTE_CHANNEL_COUNT} waste channels, {PARTICLE_TRAP_COUNT} particle traps"
    );
    println!(
        "  Disposition and access:     release/hold/reject lanes with {SLOTS_PER_STATUS_LANE} slots each, {KEEP_OUT_ZONE_COUNT} robot/service keepouts, and {} required feature groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
    assert_eq!(VIAL_COUNT, VIAL_ROWS * VIAL_COLS);
    assert_eq!(INJECTION_PORT_COUNT, STERILE_LOOP_COUNT * 2);
    assert_eq!(LOOP_CLAMP_COUNT, STERILE_LOOP_COUNT * 2);
    assert_eq!(PRESSURE_TAP_COUNT, FLOW_LANES * 2);
    assert_eq!(CAMERA_DATUM_COUNT, OPTICAL_WINDOW_COUNT);
    assert_eq!(WASTE_CHANNEL_COUNT, FLUSH_PORT_COUNT);
    assert!(CLEAN_USED_MIN_GAP >= 32.0);
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "reference_particle_vial_custody",
            VIAL_POS,
            VIAL_RACK_X,
            VIAL_RACK_Y,
        ),
        (
            "mixing_settling_prevention_cradle",
            MIX_POS,
            MIX_CRADLE_X,
            MIX_CRADLE_Y,
        ),
        (
            "sterile_injection_loop",
            INJECTION_POS,
            INJECTION_X,
            INJECTION_Y,
        ),
        (
            "optical_window_calibration_target",
            OPTICAL_POS,
            OPTICAL_TARGET_X,
            OPTICAL_TARGET_Y,
        ),
        (
            "flow_blockage_detection_panel",
            FLOW_POS,
            FLOW_PANEL_X,
            FLOW_PANEL_Y,
        ),
        ("waste_flush_route", WASTE_POS, WASTE_ROUTE_X, WASTE_ROUTE_Y),
        (
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            STATUS_POS,
            STATUS_LANES_X,
            STATUS_LANES_Y,
        ),
        (
            "particle_trap_and_clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_reference_particle_flow_calibration_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_reference_particle_flow_calibration_station_washdown_recess",
        STATION_X - 110.0,
        STATION_Y - 104.0,
        7.0,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - 3.5);
    let wet_sump = centered_cube(
        "closed_reference_particle_flow_calibration_station_wet_path_sump",
        780.0,
        178.0,
        8.0,
    )
    .translate(96.0, 25.0, BASE_Z / 2.0 - 4.0);
    let disposition_sump = centered_cube(
        "closed_reference_particle_flow_calibration_station_disposition_sump",
        STATION_X - 180.0,
        100.0,
        8.0,
    )
    .translate(0.0, -224.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "closed_reference_particle_flow_calibration_station_front_drain",
        8.0 / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 74.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess
        - wet_sump
        - disposition_sump
        - front_drain
        - insert_sockets()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + rear_service_bulkheads()
        + leak_witness_channels()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets =
        Part::empty("closed_reference_particle_flow_calibration_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_reference_particle_flow_calibration_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots =
        Part::empty("closed_reference_particle_flow_calibration_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_reference_particle_flow_calibration_station_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_reference_particle_flow_calibration_station_mount_slot_relief_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes =
        Part::empty("closed_reference_particle_flow_calibration_station_datum_pin_holes");
    for (i, (x, y)) in [
        (-530.0, 328.0),
        (530.0, 328.0),
        (-530.0, -328.0),
        (530.0, -328.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!(
                    "closed_reference_particle_flow_calibration_station_datum_pin_clearance_{i}"
                ),
                5.0 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_reference_particle_flow_calibration_station_left_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_reference_particle_flow_calibration_station_right_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_reference_particle_flow_calibration_station_rear_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_reference_particle_flow_calibration_station_front_low_lip",
        STATION_X - 170.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let custody_row = centered_cube(
        "closed_reference_particle_flow_calibration_station_custody_to_wet_path_divider",
        STATION_X - 142.0,
        10.0,
        28.0,
    )
    .translate(0.0, 124.0, BASE_Z / 2.0 + 14.0);
    let wet_to_disposition = centered_cube(
        "closed_reference_particle_flow_calibration_station_wet_path_to_disposition_divider",
        STATION_X - 160.0,
        10.0,
        26.0,
    )
    .translate(0.0, -116.0, BASE_Z / 2.0 + 13.0);
    let vial_to_mixing = centered_cube(
        "closed_reference_particle_flow_calibration_station_vial_to_mixing_barrier",
        10.0,
        210.0,
        28.0,
    )
    .translate(-208.0, 226.0, BASE_Z / 2.0 + 14.0);
    let mixing_to_optical = centered_cube(
        "closed_reference_particle_flow_calibration_station_mixing_to_optical_barrier",
        10.0,
        210.0,
        28.0,
    )
    .translate(176.0, 226.0, BASE_Z / 2.0 + 14.0);
    let injection_to_flow = centered_cube(
        "closed_reference_particle_flow_calibration_station_injection_to_flow_barrier",
        10.0,
        170.0,
        24.0,
    )
    .translate(-106.0, 28.0, BASE_Z / 2.0 + 12.0);
    let flow_to_waste = centered_cube(
        "closed_reference_particle_flow_calibration_station_flow_to_waste_barrier",
        10.0,
        170.0,
        24.0,
    )
    .translate(306.0, 28.0, BASE_Z / 2.0 + 12.0);

    custody_row
        + wet_to_disposition
        + vial_to_mixing
        + mixing_to_optical
        + injection_to_flow
        + flow_to_waste
}

fn rear_service_bulkheads() -> Part {
    let mut tabs = Part::empty("closed_reference_particle_flow_calibration_station_rear_bulkheads");
    for (i, x) in [-450.0, -300.0, -150.0, 0.0, 150.0, 300.0, 450.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("closed_reference_particle_flow_calibration_station_rear_bulkhead_{i}"),
            58.0,
            20.0,
            28.0,
        )
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 14.0);
        let bore = centered_cylinder(
            format!("closed_reference_particle_flow_calibration_station_rear_tube_bore_{i}"),
            FLUSH_TUBE_BORE_D / 2.0,
            28.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 14.0);
        tabs = tabs + (tab - bore);
    }
    tabs
}

fn leak_witness_channels() -> Part {
    let mut channels =
        Part::empty("closed_reference_particle_flow_calibration_station_leak_witness_channels");
    for i in 0..LEAK_CHANNEL_COUNT {
        let x = centered_index(i, LEAK_CHANNEL_COUNT, 204.0);
        channels = channels
            + centered_cube(
                format!("closed_reference_particle_flow_calibration_station_leak_witness_rib_{i}"),
                126.0,
                6.0,
                7.0,
            )
            .translate(x, -326.0, BASE_Z / 2.0 + 3.5);
    }
    channels
}

fn robot_fiducials() -> Part {
    let mut targets =
        Part::empty("closed_reference_particle_flow_calibration_station_robot_fiducials");
    for (i, (x, y)) in [(-516.0, 316.0), (516.0, 316.0), (-516.0, -316.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_reference_particle_flow_calibration_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    targets
}

fn reference_particle_vial_custody() -> Part {
    let body = centered_cube(
        "closed_reference_particle_vial_custody_rack_body",
        VIAL_RACK_X,
        VIAL_RACK_Y,
        VIAL_RACK_Z,
    );
    let rear_fence = centered_cube(
        "closed_reference_particle_vial_custody_rear_chain_of_custody_fence",
        VIAL_RACK_X,
        14.0,
        VIAL_RACK_Z + 34.0,
    )
    .translate(0.0, VIAL_RACK_Y / 2.0 - 7.0, 17.0);
    let front_seal_bar = centered_cube(
        "closed_reference_particle_vial_custody_front_tamper_seal_bar",
        VIAL_RACK_X - 24.0,
        12.0,
        18.0,
    )
    .translate(0.0, -VIAL_RACK_Y / 2.0 + 10.0, VIAL_RACK_Z / 2.0 + 9.0);

    let mut vial_wells = Part::empty("closed_reference_particle_vial_custody_well_cuts");
    let mut custody_keys = Part::empty("closed_reference_particle_vial_custody_keyed_lands");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let index = row * VIAL_COLS + col;
            let x = centered_index(col, VIAL_COLS, VIAL_PITCH_X);
            let y = centered_index(row, VIAL_ROWS, VIAL_PITCH_Y) - 2.0;
            vial_wells = vial_wells
                + centered_cylinder(
                    format!("closed_reference_particle_vial_custody_reference_vial_well_{index}"),
                    VIAL_BORE_D / 2.0,
                    VIAL_RACK_Z + 8.0,
                    32,
                )
                .translate(x, y, 4.0)
                + centered_cube(
                    format!("closed_reference_particle_vial_custody_finger_relief_{index}"),
                    14.0,
                    26.0,
                    22.0,
                )
                .translate(x, y - 14.0, VIAL_RACK_Z / 2.0 - 6.0);
            custody_keys = custody_keys
                + centered_cube(
                    format!("closed_reference_particle_vial_custody_lot_key_land_{index}"),
                    28.0,
                    7.0,
                    5.0,
                )
                .translate(x, y + 26.0, VIAL_RACK_Z / 2.0 + 2.5);
        }
    }

    let mut seals = Part::empty("closed_reference_particle_vial_custody_seal_lands");
    for i in 0..CUSTODY_SEAL_LANDS {
        seals = seals
            + centered_cube(
                format!("closed_reference_particle_vial_custody_tamper_seal_land_{i}"),
                18.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LANDS, 26.0),
                -VIAL_RACK_Y / 2.0 + 22.0,
                VIAL_RACK_Z / 2.0 + 2.0,
            );
    }

    body + rear_fence + front_seal_bar + custody_keys + seals - vial_wells
        + gripper_fiducials("vial_custody", VIAL_RACK_X / 2.0 - 34.0)
}

fn mixing_settling_prevention_cradle() -> Part {
    let base = centered_cube(
        "closed_reference_particle_mixing_cradle_body",
        MIX_CRADLE_X,
        MIX_CRADLE_Y,
        MIX_CRADLE_Z,
    );
    let lower_recess = centered_cube(
        "closed_reference_particle_mixing_cradle_rocking_tray_recess",
        MIX_CRADLE_X - 42.0,
        MIX_CRADLE_Y - 44.0,
        30.0,
    )
    .translate(0.0, 0.0, MIX_CRADLE_Z / 2.0 - 15.0);
    let rear_motor_land = centered_cube(
        "closed_reference_particle_mixing_cradle_rear_motor_land",
        88.0,
        26.0,
        42.0,
    )
    .translate(0.0, MIX_CRADLE_Y / 2.0 - 17.0, MIX_CRADLE_Z / 2.0 + 21.0);

    let mut rollers = Part::empty("closed_reference_particle_mixing_cradle_anti_settling_rollers");
    for i in 0..ROCKING_ROLLERS {
        let x = centered_index(i, ROCKING_ROLLERS, CRADLE_ROLLER_PITCH);
        rollers = rollers
            + centered_cylinder(
                format!("closed_reference_particle_mixing_cradle_roller_{i}"),
                9.0,
                MIX_CRADLE_Y - 42.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, MIX_CRADLE_Z / 2.0 + 12.0);
    }

    let mut slot_cuts = Part::empty("closed_reference_particle_mixing_cradle_vial_slot_cuts");
    for i in 0..CRADLE_VIAL_SLOTS {
        let x = centered_index(i % 4, 4, 68.0);
        let y = centered_index(i / 4, 2, 52.0);
        slot_cuts = slot_cuts
            + centered_cube(
                format!("closed_reference_particle_mixing_cradle_vial_saddle_{i}"),
                42.0,
                22.0,
                20.0,
            )
            .translate(x, y, MIX_CRADLE_Z / 2.0 + 6.0);
    }

    let mut tilt_stops = Part::empty("closed_reference_particle_mixing_cradle_tilt_stops");
    for i in 0..MIXING_TILT_STOPS {
        tilt_stops = tilt_stops
            + centered_cube(
                format!("closed_reference_particle_mixing_cradle_settling_pause_tilt_stop_{i}"),
                20.0,
                18.0,
                34.0,
            )
            .translate(
                centered_index(i, MIXING_TILT_STOPS, 120.0),
                -MIX_CRADLE_Y / 2.0 + 20.0,
                MIX_CRADLE_Z / 2.0 + 17.0,
            );
    }

    base - lower_recess - slot_cuts
        + rear_motor_land
        + rollers
        + tilt_stops
        + gripper_fiducials("mixing_cradle", MIX_CRADLE_X / 2.0 - 40.0)
}

fn sterile_injection_loop() -> Part {
    let body = centered_cube(
        "closed_reference_particle_sterile_injection_loop_body",
        INJECTION_X,
        INJECTION_Y,
        INJECTION_Z,
    );
    let cover_land = centered_cube(
        "closed_reference_particle_sterile_injection_loop_clear_cover_land",
        INJECTION_X - 34.0,
        INJECTION_Y - 28.0,
        8.0,
    )
    .translate(0.0, 0.0, INJECTION_Z / 2.0 + 4.0);

    let mut loop_channels =
        Part::empty("closed_reference_particle_sterile_injection_loop_channel_cuts");
    let mut clamps = Part::empty("closed_reference_particle_sterile_injection_loop_clamps");
    for i in 0..STERILE_LOOP_COUNT {
        let x = centered_index(i, STERILE_LOOP_COUNT, LOOP_PITCH_X);
        loop_channels = loop_channels
            + centered_cylinder(
                format!("closed_reference_particle_sterile_injection_loop_upstream_port_{i}"),
                LUER_CLEARANCE_D / 2.0,
                INJECTION_Y + 10.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -2.0, 3.0)
            + centered_cylinder(
                format!("closed_reference_particle_sterile_injection_loop_downstream_port_{i}"),
                LOOP_CHANNEL_D / 2.0,
                INJECTION_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, centered_index(i, STERILE_LOOP_COUNT, 28.0), -8.0)
            + centered_cube(
                format!("closed_reference_particle_sterile_injection_loop_loop_pocket_{i}"),
                62.0,
                42.0,
                18.0,
            )
            .translate(x, 22.0, INJECTION_Z / 2.0 - 9.0);
        clamps = clamps
            + centered_cube(
                format!("closed_reference_particle_sterile_injection_loop_left_clamp_{i}"),
                12.0,
                28.0,
                20.0,
            )
            .translate(x - 34.0, -28.0, INJECTION_Z / 2.0 + 10.0)
            + centered_cube(
                format!("closed_reference_particle_sterile_injection_loop_right_clamp_{i}"),
                12.0,
                28.0,
                20.0,
            )
            .translate(x + 34.0, -28.0, INJECTION_Z / 2.0 + 10.0);
    }

    let septum_guard = centered_cube(
        "closed_reference_particle_sterile_injection_loop_septum_guard_rail",
        INJECTION_X - 52.0,
        12.0,
        34.0,
    )
    .translate(0.0, -INJECTION_Y / 2.0 + 18.0, INJECTION_Z / 2.0 + 17.0);
    let pinch_valve_relief = centered_cube(
        "closed_reference_particle_sterile_injection_loop_pinch_valve_relief",
        INJECTION_X - 80.0,
        20.0,
        22.0,
    )
    .translate(0.0, INJECTION_Y / 2.0 - 28.0, INJECTION_Z / 2.0 - 6.0);

    body + cover_land + clamps + septum_guard - loop_channels - pinch_valve_relief
        + gripper_fiducials("sterile_injection_loop", INJECTION_X / 2.0 - 42.0)
}

fn optical_window_calibration_target() -> Part {
    let frame = centered_cube(
        "closed_reference_particle_optical_window_target_frame",
        OPTICAL_TARGET_X,
        OPTICAL_TARGET_Y,
        OPTICAL_TARGET_Z,
    );
    let center_opening = centered_cube(
        "closed_reference_particle_optical_window_target_center_opening",
        OPTICAL_TARGET_X - 58.0,
        OPTICAL_TARGET_Y - 52.0,
        OPTICAL_TARGET_Z - 18.0,
    )
    .translate(0.0, 0.0, 6.0);
    let glass_land = centered_cube(
        "closed_reference_particle_optical_window_target_glass_land",
        OPTICAL_TARGET_X - 36.0,
        OPTICAL_TARGET_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, -OPTICAL_TARGET_Z / 2.0 + 14.0);

    let mut windows = Part::empty("closed_reference_particle_optical_window_target_windows");
    let mut target_marks =
        Part::empty("closed_reference_particle_optical_window_target_fiducial_marks");
    for i in 0..OPTICAL_WINDOW_COUNT {
        let x = centered_index(i, OPTICAL_WINDOW_COUNT, 66.0);
        windows = windows
            + centered_cube(
                format!("closed_reference_particle_optical_window_viewport_{i}"),
                WINDOW_X,
                WINDOW_Y,
                OPTICAL_TARGET_Z + 6.0,
            )
            .translate(x, 2.0, 0.0);
        target_marks = target_marks
            + centered_cube(
                format!("closed_reference_particle_optical_window_horizontal_line_pair_{i}"),
                WINDOW_X - 8.0,
                3.0,
                4.0,
            )
            .translate(x, -WINDOW_Y / 2.0 - 16.0, OPTICAL_TARGET_Z / 2.0 + 2.0)
            + centered_cube(
                format!("closed_reference_particle_optical_window_vertical_line_pair_{i}"),
                3.0,
                WINDOW_Y + 26.0,
                4.0,
            )
            .translate(x, -WINDOW_Y / 2.0 - 16.0, OPTICAL_TARGET_Z / 2.0 + 2.0);
    }

    let mut camera_datums = Part::empty("closed_reference_particle_optical_window_camera_datums");
    for (i, (x, y)) in [
        (
            -OPTICAL_TARGET_X / 2.0 + 32.0,
            -OPTICAL_TARGET_Y / 2.0 + 28.0,
        ),
        (
            OPTICAL_TARGET_X / 2.0 - 32.0,
            -OPTICAL_TARGET_Y / 2.0 + 28.0,
        ),
        (
            -OPTICAL_TARGET_X / 2.0 + 32.0,
            OPTICAL_TARGET_Y / 2.0 - 28.0,
        ),
        (OPTICAL_TARGET_X / 2.0 - 32.0, OPTICAL_TARGET_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        camera_datums = camera_datums
            + fiducial_disc(&format!(
                "closed_reference_particle_optical_window_camera_datum_{i}"
            ))
            .translate(*x, *y, OPTICAL_TARGET_Z / 2.0 + 3.0);
    }

    frame + glass_land + target_marks + camera_datums - center_opening - windows
}

fn flow_blockage_detection_panel() -> Part {
    let plate = centered_cube(
        "closed_reference_particle_flow_blockage_detection_panel_body",
        FLOW_PANEL_X,
        FLOW_PANEL_Y,
        FLOW_PANEL_Z,
    );
    let cover_land = centered_cube(
        "closed_reference_particle_flow_blockage_detection_panel_clear_cover_land",
        FLOW_PANEL_X - 30.0,
        FLOW_PANEL_Y - 28.0,
        7.0,
    )
    .translate(0.0, 0.0, FLOW_PANEL_Z / 2.0 + 3.5);

    let mut channels = Part::empty("closed_reference_particle_flow_blockage_lane_cuts");
    let mut aperture_standards =
        Part::empty("closed_reference_particle_flow_blockage_aperture_standards");
    let mut pressure_taps = Part::empty("closed_reference_particle_flow_pressure_taps");
    for i in 0..FLOW_LANES {
        let y = centered_index(i, FLOW_LANES, FLOW_LANE_PITCH);
        channels = channels
            + centered_cube(
                format!("closed_reference_particle_flow_observation_channel_{i}"),
                FLOW_PANEL_X - 58.0,
                10.0,
                16.0,
            )
            .translate(0.0, y, FLOW_PANEL_Z / 2.0 - 8.0)
            + centered_cube(
                format!("closed_reference_particle_flow_clear_window_over_lane_{i}"),
                FLOW_PANEL_X - 108.0,
                24.0,
                8.0,
            )
            .translate(0.0, y, FLOW_PANEL_Z / 2.0 + 2.0);
        aperture_standards = aperture_standards
            + centered_cylinder(
                format!("closed_reference_particle_blockage_standard_aperture_{i}"),
                BLOCKAGE_APERTURE_D / 2.0 + i as f64 * 0.55,
                18.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(92.0, y, FLOW_PANEL_Z / 2.0 + 11.0);
        for tap_index in 0..2 {
            pressure_taps = pressure_taps
                + centered_cylinder(
                    format!("closed_reference_particle_flow_lane_{i}_pressure_tap_{tap_index}"),
                    3.0,
                    FLOW_PANEL_Z + 8.0,
                    20,
                )
                .translate(
                    if tap_index == 0 { -142.0 } else { 142.0 },
                    y,
                    FLOW_PANEL_Z / 2.0,
                );
        }
    }

    let inlet_header = centered_cube(
        "closed_reference_particle_flow_blockage_inlet_header",
        24.0,
        FLOW_PANEL_Y - 26.0,
        30.0,
    )
    .translate(-FLOW_PANEL_X / 2.0 + 25.0, 0.0, FLOW_PANEL_Z / 2.0 + 15.0);
    let outlet_header = centered_cube(
        "closed_reference_particle_flow_blockage_outlet_header",
        24.0,
        FLOW_PANEL_Y - 26.0,
        30.0,
    )
    .translate(FLOW_PANEL_X / 2.0 - 25.0, 0.0, FLOW_PANEL_Z / 2.0 + 15.0);

    plate + cover_land + aperture_standards + inlet_header + outlet_header
        - channels
        - pressure_taps
        + gripper_fiducials("flow_blockage_panel", FLOW_PANEL_X / 2.0 - 40.0)
}

fn waste_flush_route() -> Part {
    let body = centered_cube(
        "closed_reference_particle_waste_flush_route_body",
        WASTE_ROUTE_X,
        WASTE_ROUTE_Y,
        WASTE_ROUTE_Z,
    );
    let rear_fence = centered_cube(
        "closed_reference_particle_waste_flush_route_rear_splash_fence",
        WASTE_ROUTE_X,
        12.0,
        WASTE_ROUTE_Z + 28.0,
    )
    .translate(0.0, WASTE_ROUTE_Y / 2.0 - 6.0, 14.0);

    let mut ports = Part::empty("closed_reference_particle_waste_flush_route_port_cuts");
    let mut route_labels = Part::empty("closed_reference_particle_waste_flush_route_labels");
    for i in 0..FLUSH_PORT_COUNT {
        let x = centered_index(i, FLUSH_PORT_COUNT, 40.0);
        ports = ports
            + centered_cylinder(
                format!("closed_reference_particle_flush_input_port_{i}"),
                FLUSH_TUBE_BORE_D / 2.0,
                WASTE_ROUTE_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -10.0, 0.0)
            + centered_cube(
                format!("closed_reference_particle_waste_channel_trench_{i}"),
                24.0,
                WASTE_ROUTE_Y - 38.0,
                12.0,
            )
            .translate(x, 0.0, WASTE_ROUTE_Z / 2.0 - 6.0);
        route_labels = route_labels
            + centered_cube(
                format!("closed_reference_particle_flush_route_label_land_{i}"),
                24.0,
                8.0,
                4.0,
            )
            .translate(x, -WASTE_ROUTE_Y / 2.0 + 18.0, WASTE_ROUTE_Z / 2.0 + 2.0);
    }

    let waste_header = centered_cylinder(
        "closed_reference_particle_waste_flush_route_waste_header_bore",
        8.0 / 2.0,
        WASTE_ROUTE_X + 12.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, WASTE_ROUTE_Y / 2.0 - 32.0, 2.0);
    let spent_trap_land = centered_cube(
        "closed_reference_particle_waste_flush_route_particle_trap_land",
        102.0,
        42.0,
        18.0,
    )
    .translate(0.0, 14.0, WASTE_ROUTE_Z / 2.0 + 9.0);

    body + rear_fence + route_labels + spent_trap_land - ports - waste_header
        + gripper_fiducials("waste_flush_route", WASTE_ROUTE_X / 2.0 - 34.0)
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_reference_particle_barcode_certificate_lands_plate",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_reference_particle_barcode_certificate_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_particle_barcode_land_{i}"),
                42.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(i % 5, 5, 58.0),
                24.0 - (i / 5) as f64 * 32.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_particle_calibration_certificate_land_{i}"),
                62.0,
                28.0,
                4.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LANDS, 76.0),
                -TRACE_PANEL_Y / 2.0 + 18.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for i in 0..VIAL_RETAIN_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_particle_vial_retain_chain_of_custody_land_{i}"),
                92.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, VIAL_RETAIN_LANDS, 128.0),
                TRACE_PANEL_Y / 2.0 - 16.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    let scanner_relief = centered_cube(
        "closed_reference_particle_barcode_certificate_scanner_relief",
        TRACE_PANEL_X - 42.0,
        12.0,
        TRACE_PANEL_Z + 4.0,
    )
    .translate(0.0, 0.0, 0.0);

    plate + lands - scanner_relief
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_reference_particle_release_hold_reject_lanes_body",
        STATUS_LANES_X,
        STATUS_LANES_Y,
        STATUS_LANES_Z,
    );
    let mut slot_cuts =
        Part::empty("closed_reference_particle_release_hold_reject_status_slot_cuts");
    let mut lane_keys = Part::empty("closed_reference_particle_release_hold_reject_lane_keys");
    for lane in 0..RELEASE_LANES {
        let y = centered_index(lane, RELEASE_LANES, STATUS_LANE_PITCH_Y);
        let lane_name = match lane {
            0 => "release",
            1 => "hold",
            _ => "reject",
        };
        lane_keys = lane_keys
            + centered_cube(
                format!("closed_reference_particle_{lane_name}_lane_key"),
                78.0,
                10.0,
                12.0,
            )
            .translate(-STATUS_LANES_X / 2.0 + 54.0, y, STATUS_LANES_Z / 2.0 + 6.0);
        for slot in 0..SLOTS_PER_STATUS_LANE {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_reference_particle_{lane_name}_lane_slot_{slot}"),
                    34.0,
                    24.0,
                    22.0,
                )
                .translate(
                    -STATUS_LANES_X / 2.0 + 128.0 + slot as f64 * STATUS_SLOT_PITCH_X,
                    y,
                    STATUS_LANES_Z / 2.0 - 8.0,
                );
        }
    }
    let separator_a = centered_cube(
        "closed_reference_particle_release_hold_lane_divider",
        STATUS_LANES_X - 34.0,
        5.0,
        18.0,
    )
    .translate(0.0, -STATUS_LANE_PITCH_Y / 2.0, STATUS_LANES_Z / 2.0 + 9.0);
    let separator_b = centered_cube(
        "closed_reference_particle_hold_reject_lane_divider",
        STATUS_LANES_X - 34.0,
        5.0,
        18.0,
    )
    .translate(0.0, STATUS_LANE_PITCH_Y / 2.0, STATUS_LANES_Z / 2.0 + 9.0);

    body + lane_keys + separator_a + separator_b - slot_cuts
        + gripper_fiducials("release_hold_reject_lanes", STATUS_LANES_X / 2.0 - 40.0)
}

fn particle_trap_and_clean_used_segregation() -> Part {
    let body = centered_cube(
        "closed_reference_particle_trap_clean_used_segregation_body",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let divider = centered_cube(
        "closed_reference_particle_clean_used_physical_divider",
        12.0,
        SEGREGATION_Y,
        SEGREGATION_Z + 34.0,
    )
    .translate(0.0, 0.0, 17.0);

    let clean_bin = centered_cube(
        "closed_reference_particle_clean_cap_return_bin_cut",
        92.0,
        SEGREGATION_Y - 30.0,
        SEGREGATION_Z - 8.0,
    )
    .translate(-SEGREGATION_X / 4.0, 0.0, 2.0);
    let used_bin = centered_cube(
        "closed_reference_particle_used_particle_quarantine_bin_cut",
        92.0,
        SEGREGATION_Y - 30.0,
        SEGREGATION_Z - 8.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, 2.0);

    let mut cap_posts = Part::empty("closed_reference_particle_clean_cap_park_posts");
    for i in 0..CAP_PARK_POSTS {
        cap_posts = cap_posts
            + centered_cylinder(
                format!("closed_reference_particle_clean_cap_park_post_{i}"),
                4.5,
                20.0,
                22,
            )
            .translate(
                -SEGREGATION_X / 4.0 + centered_index(i % 6, 6, 16.0),
                SEGREGATION_Y / 2.0 - 20.0 - (i / 6) as f64 * 24.0,
                SEGREGATION_Z / 2.0 + 10.0,
            );
    }

    let mut cups = Part::empty("closed_reference_particle_used_particle_trap_cups");
    for i in 0..USED_PARTICLE_CUPS {
        cups = cups
            + centered_cylinder(
                format!("closed_reference_particle_used_particle_trap_cup_{i}"),
                8.0,
                SEGREGATION_Z + 6.0,
                24,
            )
            .translate(
                SEGREGATION_X / 4.0 + centered_index(i % 3, 3, 28.0),
                centered_index(i / 3, 2, 38.0),
                SEGREGATION_Z / 2.0 - 4.0,
            );
    }

    body + divider + cap_posts - clean_bin - used_bin - cups
}

fn robot_service_keepouts() -> Part {
    let mut zones =
        Part::empty("closed_reference_particle_flow_calibration_station_robot_service_keepouts");
    for (i, (name, x, y, width, depth)) in [
        (
            "front_robot_pick_sweep",
            0.0,
            -(STATION_Y / 2.0 + FRONT_ROBOT_SWEEP_CLEARANCE / 2.0),
            STATION_X - 150.0,
            FRONT_ROBOT_SWEEP_CLEARANCE,
        ),
        (
            "rear_vial_service_lane",
            0.0,
            STATION_Y / 2.0 + REAR_VIAL_SERVICE_CLEARANCE / 2.0,
            STATION_X - 190.0,
            REAR_VIAL_SERVICE_CLEARANCE,
        ),
        (
            "right_optical_service_lane",
            STATION_X / 2.0 + RIGHT_OPTICAL_SERVICE_CLEARANCE / 2.0,
            148.0,
            RIGHT_OPTICAL_SERVICE_CLEARANCE,
            430.0,
        ),
        (
            "left_injection_loop_service_lane",
            -(STATION_X / 2.0 + 132.0 / 2.0),
            -34.0,
            132.0,
            360.0,
        ),
        (
            "mixing_cradle_lift_envelope",
            MIX_POS.0,
            MIX_POS.1,
            MIX_CRADLE_X + 42.0,
            MIX_CRADLE_Y + 36.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        zones = zones
            + centered_cube(
                format!("closed_reference_particle_keepout_{i}_{name}"),
                *width,
                *depth,
                6.0,
            )
            .translate(*x, *y, 0.0);
    }
    let crosshair = centered_cube(
        "closed_reference_particle_keepout_station_centerline_x",
        STATION_X - 160.0,
        4.0,
        6.0,
    ) + centered_cube(
        "closed_reference_particle_keepout_station_centerline_y",
        4.0,
        STATION_Y - 160.0,
        6.0,
    );
    zones + crosshair
}

fn gripper_fiducials(name: &str, x_offset: f64) -> Part {
    let mut fiducials = Part::empty(format!(
        "closed_reference_particle_{name}_gripper_fiducials"
    ));
    for (i, x) in [-x_offset, x_offset].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!("closed_reference_particle_{name}_fiducial_{i}"))
                .translate(*x, 0.0, 4.0);
    }
    fiducials
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32)
        - centered_cylinder(format!("{name}_center"), 1.2, 3.0, 18)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_reference_particle_flow_calibration_station_"),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn required_features_cover_requested_station_interfaces() {
        for feature in [
            "reference_particle_vial_custody",
            "mixing_settling_prevention_cradle",
            "sterile_injection_loop",
            "optical_window_calibration_target",
            "waste_flush_route",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert!(REQUIRED_FEATURES.contains(&"flow_blockage_detection_panel"));
        assert_eq!(REQUIRED_FEATURES.len(), 12);
    }

    #[test]
    fn all_insert_modules_fit_inside_station_rims() {
        assert_layout();
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
    }

    #[test]
    fn custody_and_traceability_capacity_match_reference_vial_batch() {
        assert_eq!(VIAL_COUNT, 10);
        assert_eq!(VIAL_ROWS * VIAL_COLS, VIAL_COUNT);
        assert_eq!(CUSTODY_SEAL_LANDS, VIAL_COUNT);
        assert!(BARCODE_LANDS >= VIAL_COUNT);
        assert!(CERTIFICATE_LANDS >= RELEASE_LANES);
        assert_eq!(VIAL_RETAIN_LANDS, 2);
    }

    #[test]
    fn anti_settling_and_sterile_injection_are_explicit() {
        assert!(ROCKING_ROLLERS >= 4);
        assert_eq!(CRADLE_VIAL_SLOTS, 8);
        assert!(MIXING_TILT_STOPS >= 3);
        assert!(SETTLING_PAUSE_MAX_MIN <= 2.0);
        assert_eq!(STERILE_LOOP_COUNT, 4);
        assert_eq!(INJECTION_PORT_COUNT, STERILE_LOOP_COUNT * 2);
        assert_eq!(LOOP_CLAMP_COUNT, STERILE_LOOP_COUNT * 2);
    }

    #[test]
    fn optical_flow_and_blockage_detection_are_represented() {
        assert_eq!(OPTICAL_WINDOW_COUNT, FLOW_LANES);
        assert_eq!(BLOCKAGE_STANDARDS, FLOW_LANES);
        assert_eq!(PRESSURE_TAP_COUNT, FLOW_LANES * 2);
        assert_eq!(CAMERA_DATUM_COUNT, 4);
        assert!(OPTICAL_TARGET_MARKS >= OPTICAL_WINDOW_COUNT * 3);
        assert!(BLOCKAGE_APERTURE_D < LOOP_CHANNEL_D);
    }

    #[test]
    fn flush_waste_and_particle_traps_are_balanced() {
        assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
        assert!(PARTICLE_TRAP_COUNT >= 3);
        assert_eq!(USED_PARTICLE_CUPS, FLUSH_PORT_COUNT);
        assert!(FLUSH_TUBE_BORE_D > LOOP_CHANNEL_D);
        assert_eq!(LEAK_CHANNEL_COUNT, 5);
    }

    #[test]
    fn disposition_lanes_and_clean_used_gap_are_sufficient() {
        assert_eq!(RELEASE_LANES, 3);
        assert_eq!(SLOTS_PER_STATUS_LANE, 6);
        assert_eq!(RELEASE_LANES * SLOTS_PER_STATUS_LANE, 18);
        let clean_right_edge = SEGREGATION_POS.0 - SEGREGATION_X / 4.0 + 92.0 / 2.0;
        let used_left_edge = SEGREGATION_POS.0 + SEGREGATION_X / 4.0 - 92.0 / 2.0;
        assert!(used_left_edge - clean_right_edge >= CLEAN_USED_MIN_GAP);
    }

    #[test]
    fn robot_and_service_keepouts_are_declared_large_enough() {
        assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
        assert!(FRONT_ROBOT_SWEEP_CLEARANCE >= 340.0);
        assert!(REAR_VIAL_SERVICE_CLEARANCE >= 220.0);
        assert!(RIGHT_OPTICAL_SERVICE_CLEARANCE >= 170.0);
        assert!(STATION_X >= 1100.0);
        assert!(STATION_Y >= 720.0);
    }
}
