use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/reservoir mixing homogeneity validation station.
//
// Intent:
// - Package a closed mechanical station for checking reservoir mixing
//   homogeneity, concentration-gradient sampling, and residual hold-up before
//   automated seeding or perfusion runs.
// - Keep sealed reservoir/bag cradles, gentle rocker placeholders, top/middle/
//   bottom sampling ports, tracer coupon lands, gravimetric pads, temperature
//   logging, bubble/degas witnesses, traceability, disposition lanes, clean/used
//   segregation, camera evidence, and keepout gauges mechanically explicit.
// - This is validation packaging/interface CAD only. It is not a media
//   formulation, biological acceptance protocol, release method, or sterility
//   claim.

const OUTPUTS: [&str; 15] = [
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_base_leak_tray_deck.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_sealed_reservoir_bag_cradle.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_gentle_rocker_envelope_placeholder.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_top_middle_bottom_sampling_loop_ports.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_tracer_dye_reference_coupon_lands.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_load_cell_gravimetric_pad_placeholder.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_temperature_logger_pocket.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_bubble_degas_witness_features.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_barcode_certificate_lands.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_release_hold_reject_lanes.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_clean_used_segregation.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_holdup_volume_witness_loop_cartridge.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_evidence_camera_bridge.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_robot_service_keepout_gauges.stl",
    "output/closed_media_reservoir_mixing_homogeneity_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "base_leak_tray_deck",
    "sealed_reservoir_bag_cradle",
    "gentle_rocker_envelope_placeholder",
    "top_middle_bottom_sampling_loop_ports",
    "tracer_dye_reference_coupon_lands",
    "load_cell_gravimetric_pad_placeholder",
    "temperature_logger_pocket",
    "bubble_degas_witness_features",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "holdup_volume_witness_loop_cartridge",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.8;
const DRAIN_PORT_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_SENSOR_WELLS: usize = 6;
const DATUM_TARGETS: usize = 4;

const CRADLE_CENTER: (f64, f64) = (-410.0, 230.0);
const CRADLE_X: f64 = 420.0;
const CRADLE_Y: f64 = 230.0;
const CRADLE_Z: f64 = 58.0;
const RESERVOIR_BAGS: usize = 2;
const BAG_RECESS_X: f64 = 168.0;
const BAG_RECESS_Y: f64 = 138.0;
const BAG_RECESS_DEPTH: f64 = 22.0;
const BAG_PORT_COMBS: usize = 6;
const BAG_CLAMP_TABS: usize = 4;

const ROCKER_CENTER: (f64, f64) = (20.0, 230.0);
const ROCKER_X: f64 = 360.0;
const ROCKER_Y: f64 = 230.0;
const ROCKER_Z: f64 = 126.0;
const ROCKER_PIVOT_D: f64 = 24.0;
const ROCKER_TILT_DEG: f64 = 8.0;
const ROCKER_LIMIT_STOPS: usize = 4;

const SAMPLE_CENTER: (f64, f64) = (430.0, 230.0);
const SAMPLE_X: f64 = 300.0;
const SAMPLE_Y: f64 = 230.0;
const SAMPLE_Z: f64 = 54.0;
const SAMPLING_LEVELS: usize = 3;
const PORTS_PER_LEVEL: usize = 3;
const SAMPLE_PORT_D: f64 = 14.0;
const SAMPLE_ROW_PITCH_Y: f64 = 66.0;
const SAMPLE_PORT_PITCH_X: f64 = 58.0;
const SAMPLE_TUBE_OD: f64 = 4.8;

const TRACER_CENTER: (f64, f64) = (-440.0, -25.0);
const TRACER_X: f64 = 350.0;
const TRACER_Y: f64 = 180.0;
const TRACER_Z: f64 = 22.0;
const TRACER_COUPONS: usize = 8;
const DYE_SWATCHES: usize = 6;
const REFERENCE_CARD_SLOTS: usize = 3;

const LOAD_CENTER: (f64, f64) = (-45.0, -25.0);
const LOAD_X: f64 = 360.0;
const LOAD_Y: f64 = 180.0;
const LOAD_Z: f64 = 34.0;
const LOAD_PADS: usize = 4;
const LOAD_PAD_X: f64 = 120.0;
const LOAD_PAD_Y: f64 = 58.0;
const LOAD_PAD_Z: f64 = 6.0;
const SCALE_CABLE_CHANNEL_W: f64 = 12.0;

const LOGGER_CENTER: (f64, f64) = (380.0, -25.0);
const LOGGER_X: f64 = 290.0;
const LOGGER_Y: f64 = 180.0;
const LOGGER_Z: f64 = 44.0;
const LOGGER_SLOT_X: f64 = 178.0;
const LOGGER_SLOT_Y: f64 = 72.0;
const LOGGER_SLOT_Z: f64 = 24.0;
const TEMP_PROBE_POCKETS: usize = 5;
const LOGGER_CABLE_CLIPS: usize = 4;

const BUBBLE_CENTER: (f64, f64) = (-440.0, -260.0);
const BUBBLE_X: f64 = 350.0;
const BUBBLE_Y: f64 = 150.0;
const BUBBLE_Z: f64 = 36.0;
const SIGHT_TUBE_CHANNELS: usize = 3;
const BUBBLE_LADDER_MARKS: usize = 9;
const DEGAS_WITNESS_PORTS: usize = 3;

const TRACE_CENTER: (f64, f64) = (-80.0, -260.0);
const TRACE_X: f64 = 330.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;
const RFID_LANDS: usize = 2;

const STATUS_CENTER: (f64, f64) = (245.0, -260.0);
const STATUS_X: f64 = 250.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_PITCH_X: f64 = 76.0;

const SEG_CENTER: (f64, f64) = (520.0, -260.0);
const SEG_X: f64 = 210.0;
const SEG_Y: f64 = 150.0;
const SEG_Z: f64 = 44.0;
const CLEAN_SLOTS: usize = 4;
const USED_SLOTS: usize = 4;
const SEGREGATION_DIVIDER_Z: f64 = 76.0;

const HOLDUP_CENTER: (f64, f64) = (65.0, -150.0);
const HOLDUP_X: f64 = 510.0;
const HOLDUP_Y: f64 = 58.0;
const HOLDUP_Z: f64 = 28.0;
const HOLDUP_LOOP_CHANNELS: usize = 4;
const HOLDUP_WITNESS_WINDOWS: usize = 8;
const HOLDUP_LOOP_PITCH_X: f64 = 92.0;

const CAMERA_CENTER: (f64, f64) = (0.0, -28.0);
const CAMERA_BRIDGE_X: f64 = 1120.0;
const CAMERA_BRIDGE_Y: f64 = 62.0;
const CAMERA_BRIDGE_Z: f64 = 216.0;
const CAMERA_POST_X: f64 = 30.0;
const CAMERA_POST_Y: f64 = 48.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const EVIDENCE_CAMERAS: usize = 4;
const LIGHT_BARS: usize = 3;

const ROBOT_KEEP_OUT_X: f64 = 920.0;
const ROBOT_KEEP_OUT_Y: f64 = 575.0;
const ROBOT_KEEP_OUT_Z: f64 = 156.0;
const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_BAG_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_SAMPLE_SERVICE_CLEARANCE: f64 = 210.0;
const ROCKER_LIFT_CLEARANCE_Z: f64 = 310.0;
const KEEP_OUT_RAIL: f64 = 7.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let cradle = sealed_reservoir_bag_cradle();
    export(OUTPUTS[1], &cradle);

    let rocker = gentle_rocker_envelope_placeholder();
    export(OUTPUTS[2], &rocker);

    let sampling = top_middle_bottom_sampling_loop_ports();
    export(OUTPUTS[3], &sampling);

    let tracer = tracer_dye_reference_coupon_lands();
    export(OUTPUTS[4], &tracer);

    let load_cell = load_cell_gravimetric_pad_placeholder();
    export(OUTPUTS[5], &load_cell);

    let logger = temperature_logger_pocket();
    export(OUTPUTS[6], &logger);

    let bubbles = bubble_degas_witness_features();
    export(OUTPUTS[7], &bubbles);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[8], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[9], &status);

    let segregation = clean_used_segregation();
    export(OUTPUTS[10], &segregation);

    let holdup = holdup_volume_witness_loop_cartridge();
    export(OUTPUTS[11], &holdup);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[12], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[13], &keepouts);

    let assembly = station_assembly(
        base,
        cradle,
        rocker,
        sampling,
        tracer,
        load_cell,
        logger,
        bubbles,
        traceability,
        status,
        segregation,
        holdup,
        camera,
        keepouts,
    );
    export(OUTPUTS[14], &assembly);

    println!();
    println!("Closed media/reservoir mixing homogeneity validation station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm closed leak-tray deck"
    );
    println!(
        "  Closed reservoir handling: {RESERVOIR_BAGS} sealed bag cradles, {BAG_PORT_COMBS} port combs, and a gentle rocker envelope with +/-{ROCKER_TILT_DEG:.0} degree limit gauges"
    );
    println!(
        "  Gradient sampling:         top/middle/bottom closed-loop port rows with {PORTS_PER_LEVEL} ports per row and {SAMPLE_TUBE_OD:.1}mm OD tube assumptions"
    );
    println!(
        "  Evidence interfaces:       {TRACER_COUPONS} tracer coupon lands, {DYE_SWATCHES} dye/reference swatches, {LOAD_PADS} gravimetric pad placeholders, and {TEMP_PROBE_POCKETS} temperature pockets"
    );
    println!(
        "  Hold-up and gas witness:   {HOLDUP_LOOP_CHANNELS} hold-up loop channels, {HOLDUP_WITNESS_WINDOWS} witness windows, {SIGHT_TUBE_CHANNELS} sight tubes, and {DEGAS_WITNESS_PORTS} degas witness ports"
    );
    println!(
        "  Traceability and custody:  {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, released/hold/reject lanes, clean/used segregation, {EVIDENCE_CAMERAS} evidence cameras"
    );
    println!(
        "  Access gauges:             front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, rocker lift {ROCKER_LIFT_CLEARANCE_Z:.0}mm"
    );
    println!("  Feature groups covered:    {}", REQUIRED_FEATURES.len());
    println!("  Limitation:                mechanical validation packaging only; no formulation or biological acceptance protocol is defined.");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly(
    base: Part,
    cradle: Part,
    rocker: Part,
    sampling: Part,
    tracer: Part,
    load_cell: Part,
    logger: Part,
    bubbles: Part,
    traceability: Part,
    status: Part,
    segregation: Part,
    holdup: Part,
    camera: Part,
    keepouts: Part,
) -> Part {
    base + cradle.translate(CRADLE_CENTER.0, CRADLE_CENTER.1, part_z(CRADLE_Z))
        + rocker.translate(ROCKER_CENTER.0, ROCKER_CENTER.1, part_z(ROCKER_Z))
        + sampling.translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, part_z(SAMPLE_Z))
        + tracer.translate(TRACER_CENTER.0, TRACER_CENTER.1, part_z(TRACER_Z))
        + load_cell.translate(LOAD_CENTER.0, LOAD_CENTER.1, part_z(LOAD_Z))
        + logger.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, part_z(LOGGER_Z))
        + bubbles.translate(BUBBLE_CENTER.0, BUBBLE_CENTER.1, part_z(BUBBLE_Z))
        + traceability.translate(TRACE_CENTER.0, TRACE_CENTER.1, part_z(TRACE_Z))
        + status.translate(STATUS_CENTER.0, STATUS_CENTER.1, part_z(STATUS_Z))
        + segregation.translate(SEG_CENTER.0, SEG_CENTER.1, part_z(SEG_Z))
        + holdup.translate(HOLDUP_CENTER.0, HOLDUP_CENTER.1, part_z(HOLDUP_Z))
        + camera.translate(CAMERA_CENTER.0, CAMERA_CENTER.1, part_z(CAMERA_BRIDGE_Z))
        + keepouts
}

fn part_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_position(
    index: usize,
    cols: usize,
    count: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    let rows = count.div_ceil(cols);
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn assert_layout() {
    for (name, center, width, depth) in deck_module_specs() {
        assert!(
            fits_on_station(center, width, depth, 30.0),
            "{name} exceeds closed media mixing station deck envelope"
        );
    }

    let specs = deck_module_specs();
    for i in 0..specs.len() {
        for j in (i + 1)..specs.len() {
            assert!(
                !rects_overlap(
                    rect(specs[i].1, specs[i].2, specs[i].3),
                    rect(specs[j].1, specs[j].2, specs[j].3)
                ),
                "{} overlaps {} on station deck",
                specs[i].0,
                specs[j].0
            );
        }
    }

    assert_eq!(
        SAMPLING_LEVELS, 3,
        "top/middle/bottom sample rows are required"
    );
    assert_eq!(STATUS_LANES, 3, "release/hold/reject lanes are required");
    assert!(
        SAMPLE_ROW_PITCH_Y * (SAMPLING_LEVELS as f64 - 1.0) + SAMPLE_PORT_D < SAMPLE_Y - 52.0,
        "sampling level rows exceed sample panel"
    );
    assert!(
        HOLDUP_LOOP_PITCH_X * (HOLDUP_LOOP_CHANNELS as f64 - 1.0) + 38.0 < HOLDUP_X - 60.0,
        "hold-up witness loop channels exceed cartridge width"
    );
    assert!(
        wet_to_dry_gap() >= 18.0,
        "wet witness features are too close to dry traceability lands"
    );
}

fn deck_module_specs() -> [(&'static str, (f64, f64), f64, f64); 11] {
    [
        (
            "sealed_reservoir_bag_cradle",
            CRADLE_CENTER,
            CRADLE_X,
            CRADLE_Y,
        ),
        (
            "gentle_rocker_envelope_placeholder",
            ROCKER_CENTER,
            ROCKER_X,
            ROCKER_Y,
        ),
        (
            "top_middle_bottom_sampling_loop_ports",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        (
            "tracer_dye_reference_coupon_lands",
            TRACER_CENTER,
            TRACER_X,
            TRACER_Y,
        ),
        (
            "load_cell_gravimetric_pad_placeholder",
            LOAD_CENTER,
            LOAD_X,
            LOAD_Y,
        ),
        (
            "temperature_logger_pocket",
            LOGGER_CENTER,
            LOGGER_X,
            LOGGER_Y,
        ),
        (
            "bubble_degas_witness_features",
            BUBBLE_CENTER,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        ("barcode_certificate_lands", TRACE_CENTER, TRACE_X, TRACE_Y),
        (
            "release_hold_reject_lanes",
            STATUS_CENTER,
            STATUS_X,
            STATUS_Y,
        ),
        ("clean_used_segregation", SEG_CENTER, SEG_X, SEG_Y),
        (
            "holdup_volume_witness_loop_cartridge",
            HOLDUP_CENTER,
            HOLDUP_X,
            HOLDUP_Y,
        ),
    ]
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

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.1 && a.1 > b.0 && a.2 < b.3 && a.3 > b.2
}

fn wet_to_dry_gap() -> f64 {
    TRACE_CENTER.0 - TRACE_X / 2.0 - (BUBBLE_CENTER.0 + BUBBLE_X / 2.0)
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_media_mixing_homogeneity_station_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        "closed_media_mixing_homogeneity_station_washdown_basin_recess",
        STATION_X - 118.0,
        STATION_Y - 108.0,
        7.0,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - 3.5);
    let drain = centered_cylinder(
        "closed_media_mixing_homogeneity_station_front_drain_port",
        DRAIN_PORT_D / 2.0,
        56.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - basin - drain - deck_module_sockets() - deck_mount_slots()
        + perimeter_rims()
        + wet_dry_dividers()
        + leak_sensor_wells()
        + deck_datum_targets()
}

fn deck_module_sockets() -> Part {
    let mut sockets = Part::empty("closed_media_mixing_homogeneity_station_deck_module_sockets");
    for (name, center, width, depth) in deck_module_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_{name}_socket"),
                width + 10.0,
                depth + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(center.0, center.1, DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("closed_media_mixing_homogeneity_station_mount_slots");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 48.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 48.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, -STATION_Y / 2.0 + 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
    ]
    .into_iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_media_mixing_homogeneity_station_m6_mount_bore_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                24,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_m6_mount_slot_{index}"),
                26.0,
                7.2,
                DECK_Z + 6.0,
            )
            .translate(x, y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_media_mixing_homogeneity_station_left_leak_rim",
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_mixing_homogeneity_station_right_leak_rim",
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_media_mixing_homogeneity_station_rear_leak_rim",
        STATION_X - 58.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_media_mixing_homogeneity_station_front_low_leak_lip",
        STATION_X - 190.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, DECK_Z / 2.0 + 12.0);

    left + right + rear + front_low_lip
}

fn wet_dry_dividers() -> Part {
    let upper_lower = centered_cube(
        "closed_media_mixing_homogeneity_station_upper_lower_row_divider",
        STATION_X - 152.0,
        9.0,
        26.0,
    )
    .translate(0.0, 92.0, DECK_Z / 2.0 + 13.0);
    let middle_bottom = centered_cube(
        "closed_media_mixing_homogeneity_station_middle_bottom_row_divider",
        STATION_X - 166.0,
        9.0,
        24.0,
    )
    .translate(0.0, -180.0, DECK_Z / 2.0 + 12.0);
    let wet_trace_split = centered_cube(
        "closed_media_mixing_homogeneity_station_wet_traceability_split",
        9.0,
        168.0,
        24.0,
    )
    .translate(-260.0, -260.0, DECK_Z / 2.0 + 12.0);
    let status_seg_split = centered_cube(
        "closed_media_mixing_homogeneity_station_status_clean_used_split",
        9.0,
        168.0,
        24.0,
    )
    .translate(390.0, -260.0, DECK_Z / 2.0 + 12.0);

    upper_lower + middle_bottom + wet_trace_split + status_seg_split
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_media_mixing_homogeneity_station_leak_sensor_wells");
    for index in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(index, LEAK_SENSOR_WELLS, 186.0);
        let boss = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_leak_sensor_boss_{index}"),
            15.0,
            5.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 64.0, DECK_Z / 2.0 + 2.5);
        let pocket = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_leak_sensor_pocket_{index}"),
            7.0,
            7.0,
            28,
        )
        .translate(x, -STATION_Y / 2.0 + 64.0, DECK_Z / 2.0 + 3.0);
        wells = wells + (boss - pocket);
    }
    wells
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("closed_media_mixing_homogeneity_station_robot_datum_targets");
    let positions = [
        (-STATION_X / 2.0 + 88.0, STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 88.0, STATION_Y / 2.0 - 82.0),
        (-STATION_X / 2.0 + 88.0, -STATION_Y / 2.0 + 90.0),
        (STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 + 90.0),
    ];
    assert_eq!(positions.len(), DATUM_TARGETS);
    for (index, (x, y)) in positions.into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "closed_media_mixing_homogeneity_station_deck_datum_{index}"
            ))
            .translate(x, y, DECK_Z / 2.0 + 2.0);
    }
    targets
}

fn sealed_reservoir_bag_cradle() -> Part {
    let body = centered_cube(
        "closed_media_mixing_homogeneity_station_bag_cradle_body",
        CRADLE_X,
        CRADLE_Y,
        CRADLE_Z,
    );
    let rear_fence = centered_cube(
        "closed_media_mixing_homogeneity_station_bag_closed_port_rear_fence",
        CRADLE_X,
        14.0,
        CRADLE_Z + 36.0,
    )
    .translate(0.0, CRADLE_Y / 2.0 - 7.0, 18.0);
    let front_pull_lip = centered_cube(
        "closed_media_mixing_homogeneity_station_bag_front_robot_pull_lip",
        CRADLE_X - 42.0,
        12.0,
        24.0,
    )
    .translate(0.0, -CRADLE_Y / 2.0 + 10.0, CRADLE_Z / 2.0 + 12.0);

    body - bag_recesses()
        + rear_fence
        + front_pull_lip
        + bag_port_comb_features()
        + bag_clamp_tabs()
        + module_latch_tabs("bag_cradle", CRADLE_X, CRADLE_Y, CRADLE_Z)
}

fn bag_recesses() -> Part {
    let mut recesses = Part::empty("closed_media_mixing_homogeneity_station_bag_recesses");
    for index in 0..RESERVOIR_BAGS {
        let x = centered_index(index, RESERVOIR_BAGS, 190.0);
        recesses = recesses
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_sealed_bag_basin_{index}"),
                BAG_RECESS_X,
                BAG_RECESS_Y,
                BAG_RECESS_DEPTH + 0.4,
            )
            .translate(x, -8.0, CRADLE_Z / 2.0 - BAG_RECESS_DEPTH / 2.0 + 0.2)
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_bag_neck_keyway_{index}"),
                40.0,
                32.0,
                BAG_RECESS_DEPTH + 4.0,
            )
            .translate(x + 45.0, CRADLE_Y / 2.0 - 20.0, CRADLE_Z / 2.0 - 8.0);
    }
    recesses
}

fn bag_port_comb_features() -> Part {
    let mut combs = Part::empty("closed_media_mixing_homogeneity_station_bag_port_combs");
    for index in 0..BAG_PORT_COMBS {
        let x = centered_index(index, BAG_PORT_COMBS, 58.0);
        combs = combs
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_port_clip_bridge_{index}"),
                34.0,
                12.0,
                20.0,
            )
            .translate(x, CRADLE_Y / 2.0 + 14.0, CRADLE_Z / 2.0 + 10.0)
            + centered_cylinder(
                format!("closed_media_mixing_homogeneity_station_capped_septum_guard_{index}"),
                10.0,
                5.0,
                28,
            )
            .translate(x, CRADLE_Y / 2.0 + 34.0, CRADLE_Z / 2.0 + 2.5);
    }
    combs
}

fn bag_clamp_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_mixing_homogeneity_station_bag_clamp_tabs");
    for index in 0..BAG_CLAMP_TABS {
        let x = centered_index(index, BAG_CLAMP_TABS, 94.0);
        tabs = tabs
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_bag_soft_clamp_tab_{index}"),
                42.0,
                18.0,
                12.0,
            )
            .translate(x, -CRADLE_Y / 2.0 + 36.0, CRADLE_Z / 2.0 + 6.0);
    }
    tabs
}

fn gentle_rocker_envelope_placeholder() -> Part {
    let base = centered_cube(
        "closed_media_mixing_homogeneity_station_rocker_base_placeholder",
        ROCKER_X,
        ROCKER_Y,
        22.0,
    )
    .translate(0.0, 0.0, -ROCKER_Z / 2.0 + 11.0);
    let envelope = centered_cube(
        "closed_media_mixing_homogeneity_station_rocker_swept_envelope",
        ROCKER_X - 42.0,
        ROCKER_Y - 54.0,
        68.0,
    )
    .translate(0.0, 0.0, -8.0);
    let pivot_left = centered_cylinder(
        "closed_media_mixing_homogeneity_station_rocker_left_pivot_axis",
        ROCKER_PIVOT_D / 2.0,
        ROCKER_Y + 24.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-ROCKER_X / 2.0 + 42.0, 0.0, 0.0);
    let pivot_right = centered_cylinder(
        "closed_media_mixing_homogeneity_station_rocker_right_pivot_axis",
        ROCKER_PIVOT_D / 2.0,
        ROCKER_Y + 24.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(ROCKER_X / 2.0 - 42.0, 0.0, 0.0);
    let tilted_min = centered_cube(
        "closed_media_mixing_homogeneity_station_rocker_minus_tilt_plane",
        ROCKER_X - 70.0,
        12.0,
        8.0,
    )
    .rotate(ROCKER_TILT_DEG, 0.0, 0.0)
    .translate(0.0, -ROCKER_Y / 2.0 + 38.0, 28.0);
    let tilted_max = centered_cube(
        "closed_media_mixing_homogeneity_station_rocker_plus_tilt_plane",
        ROCKER_X - 70.0,
        12.0,
        8.0,
    )
    .rotate(-ROCKER_TILT_DEG, 0.0, 0.0)
    .translate(0.0, ROCKER_Y / 2.0 - 38.0, 28.0);

    base + envelope + pivot_left + pivot_right + tilted_min + tilted_max + rocker_limit_stops()
        - rocker_service_clearance_void()
}

fn rocker_limit_stops() -> Part {
    let mut stops = Part::empty("closed_media_mixing_homogeneity_station_rocker_limit_stops");
    let positions = [
        (-ROCKER_X / 2.0 + 34.0, -ROCKER_Y / 2.0 + 34.0),
        (ROCKER_X / 2.0 - 34.0, -ROCKER_Y / 2.0 + 34.0),
        (-ROCKER_X / 2.0 + 34.0, ROCKER_Y / 2.0 - 34.0),
        (ROCKER_X / 2.0 - 34.0, ROCKER_Y / 2.0 - 34.0),
    ];
    assert_eq!(positions.len(), ROCKER_LIMIT_STOPS);
    for (index, (x, y)) in positions.into_iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_rocker_limit_stop_{index}"),
                28.0,
                24.0,
                46.0,
            )
            .translate(x, y, -ROCKER_Z / 2.0 + 22.0);
    }
    stops
}

fn rocker_service_clearance_void() -> Part {
    centered_cube(
        "closed_media_mixing_homogeneity_station_rocker_motor_service_placeholder_void",
        84.0,
        ROCKER_Y - 72.0,
        38.0,
    )
    .translate(0.0, 0.0, -ROCKER_Z / 2.0 + 24.0)
}

fn top_middle_bottom_sampling_loop_ports() -> Part {
    let panel = centered_cube(
        "closed_media_mixing_homogeneity_station_sampling_panel_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let gasket_land = centered_cube(
        "closed_media_mixing_homogeneity_station_sampling_panel_gasket_land",
        SAMPLE_X - 38.0,
        SAMPLE_Y - 38.0,
        6.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0 + 3.0);

    panel - sample_port_bores()
        + gasket_land
        + sample_port_collars()
        + sample_level_name_lands()
        + module_latch_tabs("sampling_panel", SAMPLE_X, SAMPLE_Y, SAMPLE_Z)
}

fn sample_port_bores() -> Part {
    let mut bores = Part::empty("closed_media_mixing_homogeneity_station_sample_port_bores");
    for level in 0..SAMPLING_LEVELS {
        let y = centered_index(level, SAMPLING_LEVELS, SAMPLE_ROW_PITCH_Y);
        for port in 0..PORTS_PER_LEVEL {
            let x = centered_index(port, PORTS_PER_LEVEL, SAMPLE_PORT_PITCH_X);
            bores = bores
                + centered_cylinder(
                    format!(
                        "closed_media_mixing_homogeneity_station_sample_bore_level_{level}_{port}"
                    ),
                    SAMPLE_PORT_D / 2.0,
                    SAMPLE_Z + 8.0,
                    36,
                )
                .translate(x, y, 0.0)
                + centered_cylinder(
                    format!(
                        "closed_media_mixing_homogeneity_station_sample_tube_side_bore_{level}_{port}"
                    ),
                    SAMPLE_TUBE_OD / 2.0,
                    SAMPLE_X + 18.0,
                    20,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(x, y, 0.0);
        }
    }
    bores
}

fn sample_port_collars() -> Part {
    let mut collars = Part::empty("closed_media_mixing_homogeneity_station_sample_port_collars");
    for level in 0..SAMPLING_LEVELS {
        let y = centered_index(level, SAMPLING_LEVELS, SAMPLE_ROW_PITCH_Y);
        for port in 0..PORTS_PER_LEVEL {
            let x = centered_index(port, PORTS_PER_LEVEL, SAMPLE_PORT_PITCH_X);
            let outer = centered_cylinder(
                format!(
                    "closed_media_mixing_homogeneity_station_sample_port_collar_{level}_{port}"
                ),
                SAMPLE_PORT_D / 2.0 + 5.0,
                6.0,
                36,
            )
            .translate(x, y, SAMPLE_Z / 2.0 + 3.0);
            let inner = centered_cylinder(
                format!(
                    "closed_media_mixing_homogeneity_station_sample_port_collar_open_{level}_{port}"
                ),
                SAMPLE_PORT_D / 2.0 + 0.7,
                6.4,
                36,
            )
            .translate(x, y, SAMPLE_Z / 2.0 + 3.0);
            collars = collars + (outer - inner);
        }
    }
    collars
}

fn sample_level_name_lands() -> Part {
    let mut lands = Part::empty("closed_media_mixing_homogeneity_station_sample_level_lands");
    for (level, (label, y)) in [
        ("top", SAMPLE_ROW_PITCH_Y),
        ("middle", 0.0),
        ("bottom", -SAMPLE_ROW_PITCH_Y),
    ]
    .into_iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!(
                    "closed_media_mixing_homogeneity_station_{level}_{label}_sample_label_land"
                ),
                56.0,
                16.0,
                3.0,
            )
            .translate(-SAMPLE_X / 2.0 + 46.0, y, SAMPLE_Z / 2.0 + 1.5);
    }
    lands
}

fn tracer_dye_reference_coupon_lands() -> Part {
    let panel = centered_cube(
        "closed_media_mixing_homogeneity_station_tracer_coupon_panel",
        TRACER_X,
        TRACER_Y,
        TRACER_Z,
    );
    let drip_recess = centered_cube(
        "closed_media_mixing_homogeneity_station_tracer_panel_drip_recess",
        TRACER_X - 34.0,
        TRACER_Y - 34.0,
        6.0,
    )
    .translate(0.0, 0.0, TRACER_Z / 2.0 - 2.8);

    panel - drip_recess
        + tracer_coupon_pockets()
        + dye_reference_swatches()
        + reference_card_slots()
        + module_latch_tabs("tracer_panel", TRACER_X, TRACER_Y, TRACER_Z)
}

fn tracer_coupon_pockets() -> Part {
    let mut pockets = Part::empty("closed_media_mixing_homogeneity_station_tracer_coupon_pockets");
    for index in 0..TRACER_COUPONS {
        let (x, y) = grid_position(index, 4, TRACER_COUPONS, 68.0, 50.0);
        let land = centered_cube(
            format!("closed_media_mixing_homogeneity_station_tracer_coupon_land_{index}"),
            48.0,
            26.0,
            4.0,
        )
        .translate(x - 24.0, y + 14.0, TRACER_Z / 2.0 + 2.0);
        let retention_rail = centered_cube(
            format!("closed_media_mixing_homogeneity_station_tracer_coupon_retainer_{index}"),
            54.0,
            5.0,
            11.0,
        )
        .translate(x - 24.0, y - 6.0, TRACER_Z / 2.0 + 5.5);
        pockets = pockets + land + retention_rail;
    }
    pockets
}

fn dye_reference_swatches() -> Part {
    let mut swatches =
        Part::empty("closed_media_mixing_homogeneity_station_dye_reference_swatches");
    for index in 0..DYE_SWATCHES {
        let x = centered_index(index, DYE_SWATCHES, 38.0);
        swatches = swatches
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_dye_swatch_land_{index}"),
                26.0,
                22.0,
                3.0,
            )
            .translate(x, -TRACER_Y / 2.0 + 26.0, TRACER_Z / 2.0 + 1.5);
    }
    swatches
}

fn reference_card_slots() -> Part {
    let mut slots = Part::empty("closed_media_mixing_homogeneity_station_reference_card_slots");
    for index in 0..REFERENCE_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_reference_card_land_{index}"),
                84.0,
                18.0,
                4.0,
            )
            .translate(
                TRACER_X / 2.0 - 58.0,
                centered_index(index, REFERENCE_CARD_SLOTS, 38.0),
                TRACER_Z / 2.0 + 2.0,
            );
    }
    slots
}

fn load_cell_gravimetric_pad_placeholder() -> Part {
    let base = centered_cube(
        "closed_media_mixing_homogeneity_station_load_cell_base",
        LOAD_X,
        LOAD_Y,
        LOAD_Z,
    );
    let isolation_recess = centered_cube(
        "closed_media_mixing_homogeneity_station_load_cell_isolation_recess",
        LOAD_X - 42.0,
        LOAD_Y - 44.0,
        8.0,
    )
    .translate(0.0, 0.0, LOAD_Z / 2.0 - 3.8);
    let cable_channel = centered_cube(
        "closed_media_mixing_homogeneity_station_scale_cable_channel",
        LOAD_X - 56.0,
        SCALE_CABLE_CHANNEL_W,
        15.0,
    )
    .translate(0.0, -LOAD_Y / 2.0 + 26.0, -4.0);

    base - isolation_recess - cable_channel
        + load_cell_pad_lands()
        + load_cell_guard_rails()
        + module_latch_tabs("load_cell", LOAD_X, LOAD_Y, LOAD_Z)
}

fn load_cell_pad_lands() -> Part {
    let mut pads = Part::empty("closed_media_mixing_homogeneity_station_load_cell_pads");
    for index in 0..LOAD_PADS {
        let (x, y) = grid_position(index, 2, LOAD_PADS, 148.0, 78.0);
        pads = pads
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_gravimetric_pad_{index}"),
                LOAD_PAD_X,
                LOAD_PAD_Y,
                LOAD_PAD_Z,
            )
            .translate(x, y, LOAD_Z / 2.0 + LOAD_PAD_Z / 2.0)
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_load_cell_datum_slot_{index}"),
                38.0,
                5.0,
                LOAD_PAD_Z + 3.0,
            )
            .translate(
                x,
                y + LOAD_PAD_Y / 2.0 - 9.0,
                LOAD_Z / 2.0 + LOAD_PAD_Z / 2.0,
            );
    }
    pads
}

fn load_cell_guard_rails() -> Part {
    let rear = centered_cube(
        "closed_media_mixing_homogeneity_station_load_cell_rear_guard_rail",
        LOAD_X - 36.0,
        8.0,
        18.0,
    )
    .translate(0.0, LOAD_Y / 2.0 - 12.0, LOAD_Z / 2.0 + 9.0);
    let side = centered_cube(
        "closed_media_mixing_homogeneity_station_load_cell_service_side_rail",
        8.0,
        LOAD_Y - 36.0,
        18.0,
    )
    .translate(LOAD_X / 2.0 - 12.0, 0.0, LOAD_Z / 2.0 + 9.0);
    rear + side
}

fn temperature_logger_pocket() -> Part {
    let body = centered_cube(
        "closed_media_mixing_homogeneity_station_temperature_logger_body",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let logger_slot = centered_cube(
        "closed_media_mixing_homogeneity_station_temperature_logger_slot",
        LOGGER_SLOT_X,
        LOGGER_SLOT_Y,
        LOGGER_SLOT_Z + 0.4,
    )
    .translate(0.0, 14.0, LOGGER_Z / 2.0 - LOGGER_SLOT_Z / 2.0 + 0.2);
    let cable_exit = centered_cube(
        "closed_media_mixing_homogeneity_station_temperature_logger_cable_exit",
        70.0,
        16.0,
        18.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 8.0, -3.0);

    body - logger_slot - cable_exit
        + temperature_probe_pockets()
        + logger_cable_clips()
        + module_latch_tabs("temperature_logger", LOGGER_X, LOGGER_Y, LOGGER_Z)
}

fn temperature_probe_pockets() -> Part {
    let mut pockets =
        Part::empty("closed_media_mixing_homogeneity_station_temperature_probe_pockets");
    for index in 0..TEMP_PROBE_POCKETS {
        let x = centered_index(index, TEMP_PROBE_POCKETS, 38.0);
        let clip = centered_cube(
            format!("closed_media_mixing_homogeneity_station_temperature_probe_clip_{index}"),
            28.0,
            12.0,
            9.0,
        )
        .translate(x, -LOGGER_Y / 2.0 + 34.0, LOGGER_Z / 2.0 + 4.5);
        let bore = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_temperature_probe_bore_{index}"),
            2.0,
            38.0,
            16,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -LOGGER_Y / 2.0 + 34.0, LOGGER_Z / 2.0 + 4.5);
        pockets = pockets + (clip - bore);
    }
    pockets
}

fn logger_cable_clips() -> Part {
    let mut clips = Part::empty("closed_media_mixing_homogeneity_station_logger_cable_clips");
    for index in 0..LOGGER_CABLE_CLIPS {
        clips = clips
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_logger_cable_clip_{index}"),
                28.0,
                10.0,
                8.0,
            )
            .translate(
                -LOGGER_X / 2.0 + 42.0 + index as f64 * 46.0,
                -LOGGER_Y / 2.0 + 22.0,
                LOGGER_Z / 2.0 + 4.0,
            );
    }
    clips
}

fn bubble_degas_witness_features() -> Part {
    let base = centered_cube(
        "closed_media_mixing_homogeneity_station_bubble_degas_body",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );
    let spill_recess = centered_cube(
        "closed_media_mixing_homogeneity_station_bubble_degas_spill_recess",
        BUBBLE_X - 38.0,
        BUBBLE_Y - 34.0,
        6.0,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0 - 2.8);

    base - spill_recess - sight_tube_channels()
        + sight_tube_caps()
        + bubble_ladder_marks()
        + degas_witness_ports()
        + module_latch_tabs("bubble_degas", BUBBLE_X, BUBBLE_Y, BUBBLE_Z)
}

fn sight_tube_channels() -> Part {
    let mut channels = Part::empty("closed_media_mixing_homogeneity_station_sight_tube_channels");
    for index in 0..SIGHT_TUBE_CHANNELS {
        let y = centered_index(index, SIGHT_TUBE_CHANNELS, 42.0);
        channels = channels
            + centered_cylinder(
                format!("closed_media_mixing_homogeneity_station_sight_tube_channel_{index}"),
                7.0,
                BUBBLE_X - 70.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-18.0, y, BUBBLE_Z / 2.0 - 8.0);
    }
    channels
}

fn sight_tube_caps() -> Part {
    let mut caps = Part::empty("closed_media_mixing_homogeneity_station_sight_tube_caps");
    for index in 0..SIGHT_TUBE_CHANNELS {
        let y = centered_index(index, SIGHT_TUBE_CHANNELS, 42.0);
        for (end, x) in [
            ("left", -BUBBLE_X / 2.0 + 36.0),
            ("right", BUBBLE_X / 2.0 - 36.0),
        ] {
            caps = caps
                + centered_cube(
                    format!("closed_media_mixing_homogeneity_station_sight_tube_{end}_cap_{index}"),
                    18.0,
                    26.0,
                    18.0,
                )
                .translate(x, y, BUBBLE_Z / 2.0 + 9.0);
        }
    }
    caps
}

fn bubble_ladder_marks() -> Part {
    let mut marks = Part::empty("closed_media_mixing_homogeneity_station_bubble_ladder_marks");
    for index in 0..BUBBLE_LADDER_MARKS {
        marks = marks
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_bubble_ladder_mark_{index}"),
                18.0 + (index % 3) as f64 * 6.0,
                4.0,
                4.0,
            )
            .translate(
                BUBBLE_X / 2.0 - 58.0,
                centered_index(index, BUBBLE_LADDER_MARKS, 12.0),
                BUBBLE_Z / 2.0 + 2.0,
            );
    }
    marks
}

fn degas_witness_ports() -> Part {
    let mut ports = Part::empty("closed_media_mixing_homogeneity_station_degas_witness_ports");
    for index in 0..DEGAS_WITNESS_PORTS {
        let x = centered_index(index, DEGAS_WITNESS_PORTS, 52.0);
        let collar = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_degas_port_collar_{index}"),
            13.0,
            7.0,
            32,
        )
        .translate(x, BUBBLE_Y / 2.0 - 28.0, BUBBLE_Z / 2.0 + 3.5);
        let bore = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_degas_port_bore_{index}"),
            6.0,
            7.4,
            28,
        )
        .translate(x, BUBBLE_Y / 2.0 - 28.0, BUBBLE_Z / 2.0 + 3.5);
        ports = ports + (collar - bore);
    }
    ports
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_media_mixing_homogeneity_station_barcode_certificate_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );

    panel
        + barcode_lands()
        + certificate_lands()
        + rfid_lands()
        + module_latch_tabs("traceability_panel", TRACE_X, TRACE_Y, TRACE_Z)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_media_mixing_homogeneity_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let (x, y) = grid_position(index, 4, BARCODE_LANDS, 72.0, 42.0);
        lands = lands
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_barcode_land_{index}"),
                56.0,
                22.0,
                3.0,
            )
            .translate(x - 22.0, y + 18.0, TRACE_Z / 2.0 + 1.5);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("closed_media_mixing_homogeneity_station_certificate_lands");
    for index in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_certificate_card_land_{index}"),
                86.0,
                26.0,
                4.0,
            )
            .translate(
                TRACE_X / 2.0 - 60.0,
                centered_index(index, CERTIFICATE_LANDS, 42.0),
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("closed_media_mixing_homogeneity_station_rfid_lands");
    for index in 0..RFID_LANDS {
        let disc = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_rfid_antenna_land_{index}"),
            21.0,
            3.0,
            36,
        )
        .translate(
            -TRACE_X / 2.0 + 42.0,
            centered_index(index, RFID_LANDS, 54.0),
            TRACE_Z / 2.0 + 1.5,
        );
        lands = lands + disc;
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_media_mixing_homogeneity_station_release_hold_reject_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    body - status_lane_recesses()
        + status_lane_gates()
        + status_token_posts()
        + module_latch_tabs("status_lanes", STATUS_X, STATUS_Y, STATUS_Z)
}

fn status_lane_recesses() -> Part {
    let mut recesses = Part::empty("closed_media_mixing_homogeneity_station_status_lane_recesses");
    for lane in 0..STATUS_LANES {
        let x = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_status_lane_recess_{lane}"),
                56.0,
                STATUS_Y - 44.0,
                17.0,
            )
            .translate(x, 0.0, STATUS_Z / 2.0 - 8.0);
    }
    recesses
}

fn status_lane_gates() -> Part {
    let mut gates = Part::empty("closed_media_mixing_homogeneity_station_status_lane_gates");
    for (lane, label) in ["release", "hold", "reject"].into_iter().enumerate() {
        let x = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_X);
        gates = gates
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_{label}_gate_bar"),
                58.0,
                8.0,
                42.0,
            )
            .translate(x, STATUS_Y / 2.0 - 18.0, STATUS_Z / 2.0 + 21.0);
    }
    gates
}

fn status_token_posts() -> Part {
    let mut posts = Part::empty("closed_media_mixing_homogeneity_station_status_token_posts");
    for lane in 0..STATUS_LANES {
        let x = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_X);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            posts = posts
                + centered_cylinder(
                    format!(
                        "closed_media_mixing_homogeneity_station_status_token_post_{lane}_{slot}"
                    ),
                    5.0,
                    8.0,
                    20,
                )
                .translate(
                    x,
                    centered_index(slot, STATUS_SLOTS_PER_LANE, 24.0) - 10.0,
                    STATUS_Z / 2.0 + 4.0,
                );
        }
    }
    posts
}

fn clean_used_segregation() -> Part {
    let body = centered_cube(
        "closed_media_mixing_homogeneity_station_clean_used_body",
        SEG_X,
        SEG_Y,
        SEG_Z,
    );
    let clean_recess = centered_cube(
        "closed_media_mixing_homogeneity_station_clean_lane_recess",
        SEG_X / 2.0 - 24.0,
        SEG_Y - 42.0,
        18.0,
    )
    .translate(-SEG_X / 4.0, 0.0, SEG_Z / 2.0 - 8.0);
    let used_recess = centered_cube(
        "closed_media_mixing_homogeneity_station_used_lane_recess",
        SEG_X / 2.0 - 24.0,
        SEG_Y - 42.0,
        18.0,
    )
    .translate(SEG_X / 4.0, 0.0, SEG_Z / 2.0 - 8.0);
    let divider = centered_cube(
        "closed_media_mixing_homogeneity_station_clean_used_high_divider",
        10.0,
        SEG_Y - 20.0,
        SEGREGATION_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEG_Z / 2.0 + SEGREGATION_DIVIDER_Z / 2.0);

    body - clean_recess - used_recess
        + divider
        + clean_used_slots()
        + module_latch_tabs("clean_used", SEG_X, SEG_Y, SEG_Z)
}

fn clean_used_slots() -> Part {
    let mut slots = Part::empty("closed_media_mixing_homogeneity_station_clean_used_slots");
    for index in 0..CLEAN_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_clean_cap_slot_{index}"),
                28.0,
                12.0,
                8.0,
            )
            .translate(
                -SEG_X / 4.0,
                centered_index(index, CLEAN_SLOTS, 24.0),
                SEG_Z / 2.0 + 4.0,
            );
    }
    for index in 0..USED_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_used_cap_slot_{index}"),
                28.0,
                12.0,
                8.0,
            )
            .translate(
                SEG_X / 4.0,
                centered_index(index, USED_SLOTS, 24.0),
                SEG_Z / 2.0 + 4.0,
            );
    }
    slots
}

fn holdup_volume_witness_loop_cartridge() -> Part {
    let body = centered_cube(
        "closed_media_mixing_homogeneity_station_holdup_loop_body",
        HOLDUP_X,
        HOLDUP_Y,
        HOLDUP_Z,
    );
    let open_window = centered_cube(
        "closed_media_mixing_homogeneity_station_holdup_loop_long_window",
        HOLDUP_X - 60.0,
        HOLDUP_Y - 28.0,
        7.0,
    )
    .translate(0.0, 0.0, HOLDUP_Z / 2.0 - 3.0);

    body - open_window - holdup_loop_channels()
        + holdup_witness_windows()
        + holdup_reference_marks()
        + module_latch_tabs("holdup_loop", HOLDUP_X, HOLDUP_Y, HOLDUP_Z)
}

fn holdup_loop_channels() -> Part {
    let mut channels = Part::empty("closed_media_mixing_homogeneity_station_holdup_loop_channels");
    for index in 0..HOLDUP_LOOP_CHANNELS {
        let x = centered_index(index, HOLDUP_LOOP_CHANNELS, HOLDUP_LOOP_PITCH_X);
        channels = channels
            + centered_cylinder(
                format!("closed_media_mixing_homogeneity_station_holdup_channel_{index}"),
                5.0,
                HOLDUP_Y + 10.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
    }
    channels
}

fn holdup_witness_windows() -> Part {
    let mut windows = Part::empty("closed_media_mixing_homogeneity_station_holdup_witness_windows");
    for index in 0..HOLDUP_WITNESS_WINDOWS {
        let (x, y) = grid_position(index, 4, HOLDUP_WITNESS_WINDOWS, 92.0, 26.0);
        windows = windows
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_holdup_window_land_{index}"),
                46.0,
                9.0,
                3.0,
            )
            .translate(x, y, HOLDUP_Z / 2.0 + 1.5);
    }
    windows
}

fn holdup_reference_marks() -> Part {
    let mut marks = Part::empty("closed_media_mixing_homogeneity_station_holdup_reference_marks");
    for index in 0..=HOLDUP_LOOP_CHANNELS {
        let x = centered_index(index, HOLDUP_LOOP_CHANNELS + 1, HOLDUP_LOOP_PITCH_X);
        marks = marks
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_holdup_reference_mark_{index}"),
                5.0,
                HOLDUP_Y - 14.0,
                4.0,
            )
            .translate(x, 0.0, HOLDUP_Z / 2.0 + 2.0);
    }
    marks
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_media_mixing_homogeneity_station_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 34.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_media_mixing_homogeneity_station_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 34.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_media_mixing_homogeneity_station_camera_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - CAMERA_BEAM_Z / 2.0);

    left_post + right_post + beam + evidence_camera_blocks() + evidence_light_bars()
}

fn evidence_camera_blocks() -> Part {
    let mut cameras = Part::empty("closed_media_mixing_homogeneity_station_evidence_cameras");
    for index in 0..EVIDENCE_CAMERAS {
        let x = centered_index(index, EVIDENCE_CAMERAS, 240.0);
        let body = centered_cube(
            format!("closed_media_mixing_homogeneity_station_camera_body_{index}"),
            44.0,
            38.0,
            32.0,
        )
        .translate(
            x,
            -CAMERA_BRIDGE_Y / 2.0 - 22.0,
            CAMERA_BRIDGE_Z / 2.0 - 44.0,
        );
        let lens = centered_cylinder(
            format!("closed_media_mixing_homogeneity_station_camera_lens_{index}"),
            9.0,
            8.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            -CAMERA_BRIDGE_Y / 2.0 - 45.0,
            CAMERA_BRIDGE_Z / 2.0 - 44.0,
        );
        cameras = cameras + body + lens;
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("closed_media_mixing_homogeneity_station_evidence_light_bars");
    for index in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("closed_media_mixing_homogeneity_station_evidence_light_bar_{index}"),
                180.0,
                8.0,
                10.0,
            )
            .translate(
                centered_index(index, LIGHT_BARS, 320.0),
                CAMERA_BRIDGE_Y / 2.0 + 12.0,
                CAMERA_BRIDGE_Z / 2.0 - 50.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let robot_window = keepout_box(
        "closed_media_mixing_homogeneity_station_robot_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
        (0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0),
    );
    let front_gauge = centered_cube(
        "closed_media_mixing_homogeneity_station_front_robot_clearance_gauge",
        STATION_X - 160.0,
        KEEP_OUT_RAIL,
        30.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        DECK_Z / 2.0 + 15.0,
    );
    let rear_gauge = centered_cube(
        "closed_media_mixing_homogeneity_station_rear_service_clearance_gauge",
        STATION_X - 190.0,
        KEEP_OUT_RAIL,
        30.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE,
        DECK_Z / 2.0 + 15.0,
    );
    let left_service = centered_cube(
        "closed_media_mixing_homogeneity_station_left_bag_service_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 160.0,
        30.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_BAG_SERVICE_CLEARANCE,
        0.0,
        DECK_Z / 2.0 + 15.0,
    );
    let right_service = centered_cube(
        "closed_media_mixing_homogeneity_station_right_sample_service_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 160.0,
        30.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SAMPLE_SERVICE_CLEARANCE,
        0.0,
        DECK_Z / 2.0 + 15.0,
    );
    let rocker_lift = keepout_box(
        "closed_media_mixing_homogeneity_station_rocker_lift_keepout",
        ROCKER_X + 70.0,
        ROCKER_Y + 70.0,
        ROCKER_LIFT_CLEARANCE_Z,
        (
            ROCKER_CENTER.0,
            ROCKER_CENTER.1,
            DECK_Z / 2.0 + ROCKER_LIFT_CLEARANCE_Z / 2.0,
        ),
    );

    robot_window + front_gauge + rear_gauge + left_service + right_service + rocker_lift
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64, center: (f64, f64, f64)) -> Part {
    let front = centered_cube(
        format!("{name}_front_rail"),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(center.0, center.1 - y / 2.0, center.2 + z / 2.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, KEEP_OUT_RAIL, KEEP_OUT_RAIL)
        .translate(center.0, center.1 + y / 2.0, center.2 + z / 2.0);
    let left = centered_cube(format!("{name}_left_rail"), KEEP_OUT_RAIL, y, KEEP_OUT_RAIL)
        .translate(center.0 - x / 2.0, center.1, center.2 + z / 2.0);
    let right = centered_cube(
        format!("{name}_right_rail"),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(center.0 + x / 2.0, center.1, center.2 + z / 2.0);
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (index, (px, py)) in [
        (center.0 - x / 2.0, center.1 - y / 2.0),
        (center.0 + x / 2.0, center.1 - y / 2.0),
        (center.0 - x / 2.0, center.1 + y / 2.0),
        (center.0 + x / 2.0, center.1 + y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{name}_corner_post_{index}"),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                z,
            )
            .translate(px, py, center.2);
    }
    front + rear + left + right + posts
}

fn module_latch_tabs(prefix: &str, width: f64, depth: f64, height: f64) -> Part {
    let rear = centered_cube(
        format!("closed_media_mixing_homogeneity_station_{prefix}_rear_latch_tab"),
        width - 42.0,
        10.0,
        14.0,
    )
    .translate(0.0, depth / 2.0 - 14.0, height / 2.0 + 7.0);
    let front_left = centered_cube(
        format!("closed_media_mixing_homogeneity_station_{prefix}_front_left_latch_tab"),
        64.0,
        10.0,
        14.0,
    )
    .translate(-width / 2.0 + 58.0, -depth / 2.0 + 14.0, height / 2.0 + 7.0);
    let front_right = centered_cube(
        format!("closed_media_mixing_homogeneity_station_{prefix}_front_right_latch_tab"),
        64.0,
        10.0,
        14.0,
    )
    .translate(width / 2.0 - 58.0, -depth / 2.0 + 14.0, height / 2.0 + 7.0);
    rear + front_left + front_right
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 13.0, 4.0, 40);
    let inner = centered_cylinder(format!("{name}_inner"), 5.0, 4.6, 28);
    outer - inner
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_stable_and_prefixed() {
        assert_eq!(OUTPUTS.len(), 15);
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_media_reservoir_mixing_homogeneity_validation_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[0].ends_with("_base_leak_tray_deck.stl"));
        assert!(OUTPUTS[14].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_validation_packaging_features_are_declared() {
        assert_eq!(REQUIRED_FEATURES.len(), 14);
        assert!(REQUIRED_FEATURES.contains(&"top_middle_bottom_sampling_loop_ports"));
        assert!(REQUIRED_FEATURES.contains(&"holdup_volume_witness_loop_cartridge"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepout_gauges"));
    }

    #[test]
    fn geometry_assumptions_fit_the_station_deck() {
        assert_layout();
        assert!(STATION_X >= 1200.0);
        assert!(STATION_Y >= 800.0);
        assert!(CRADLE_X + ROCKER_X + SAMPLE_X < STATION_X - 120.0);
        assert_eq!(RESERVOIR_BAGS, 2);
        assert_eq!(SAMPLING_LEVELS, 3);
        assert_eq!(PORTS_PER_LEVEL, 3);
        assert!(ROCKER_LIFT_CLEARANCE_Z > ROCKER_Z);
    }

    #[test]
    fn custody_and_evidence_capacity_are_explicit() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert!(BARCODE_LANDS >= 6);
        assert!(CERTIFICATE_LANDS >= 2);
        assert!(EVIDENCE_CAMERAS >= 3);
        assert!(CLEAN_SLOTS == USED_SLOTS);
    }

    #[test]
    fn hold_up_and_gradient_features_are_conservative_placeholders() {
        assert!(HOLDUP_LOOP_CHANNELS >= SAMPLING_LEVELS);
        assert!(HOLDUP_WITNESS_WINDOWS >= HOLDUP_LOOP_CHANNELS * 2);
        assert!(SIGHT_TUBE_CHANNELS >= SAMPLING_LEVELS);
        assert!(DEGAS_WITNESS_PORTS >= SAMPLING_LEVELS);
        assert!(SAMPLE_TUBE_OD > 0.0);
    }
}
