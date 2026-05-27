use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed optical focus drift calibration station for the culture imaging module.
//
// Intent:
// - Validate focus repeatability, flat-field response, fiducial registration, and
//   transmitted/epi illumination without live cells.
// - Keep reference slides, focus phantoms, dust covers, certificates, and status
//   lanes physically segregated so calibration evidence can be handled as a
//   closed, traceable workflow.
// - Model mechanical envelopes and robot/service keepouts only. Optical
//   acceptance thresholds, certificate content, and instrument algorithms remain
//   process controls outside this CAD placeholder.

const BIN_PREFIX: &str = "closed_optical_focus_drift_calibration_station";

const OUTPUTS: &[&str] = &[
    "output/closed_optical_focus_drift_calibration_station_base_enclosure.stl",
    "output/closed_optical_focus_drift_calibration_station_calibration_slide_nests.stl",
    "output/closed_optical_focus_drift_calibration_station_focus_step_phantom_blocks.stl",
    "output/closed_optical_focus_drift_calibration_station_flat_field_target_carrier.stl",
    "output/closed_optical_focus_drift_calibration_station_fiducial_grid_plate.stl",
    "output/closed_optical_focus_drift_calibration_station_illumination_reference_lands.stl",
    "output/closed_optical_focus_drift_calibration_station_temperature_logger_pocket.stl",
    "output/closed_optical_focus_drift_calibration_station_barcode_certificate_lands.stl",
    "output/closed_optical_focus_drift_calibration_station_released_hold_reject_lanes.stl",
    "output/closed_optical_focus_drift_calibration_station_dust_cover_cassette.stl",
    "output/closed_optical_focus_drift_calibration_station_evidence_camera_bridge.stl",
    "output/closed_optical_focus_drift_calibration_station_robot_service_keepouts.stl",
    "output/closed_optical_focus_drift_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "calibration_slide_nests",
    "focus_step_phantom_blocks",
    "flat_field_target_carrier",
    "fiducial_grid_plate",
    "transmitted_epi_illumination_reference_lands",
    "temperature_logger_pocket",
    "barcode_certificate_lands",
    "released_hold_reject_lanes",
    "dust_cover_cassette",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "assembly_export",
];

const STATUS_LANE_NAMES: &[&str] = &["released", "hold", "reject"];

const STATION_X: f64 = 1040.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 24.0;
const PERIMETER_RIM_W: f64 = 18.0;
const PERIMETER_RIM_Z: f64 = 36.0;
const GASKET_RAIL_W: f64 = 12.0;
const GASKET_RAIL_Z: f64 = 8.0;
const LIGHT_TRAP_Z: f64 = 32.0;

const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const SLIDE_NEST_COUNT: usize = 6;
const SLIDE_NEST_COLS: usize = 3;
const SLIDE_NEST_ROWS: usize = 2;
const SLIDE_RACK_X: f64 = 430.0;
const SLIDE_RACK_Y: f64 = 214.0;
const SLIDE_RACK_Z: f64 = 22.0;
const SLIDE_RACK_CENTER_X: f64 = -280.0;
const SLIDE_RACK_CENTER_Y: f64 = 185.0;
const SLIDE_SLOT_X: f64 = 82.0;
const SLIDE_SLOT_Y: f64 = 34.0;
const SLIDE_SLOT_DEPTH: f64 = 14.0;
const SLIDE_PITCH_X: f64 = 118.0;
const SLIDE_PITCH_Y: f64 = 82.0;

const FOCUS_STEP_COUNT: usize = 9;
const FOCUS_BLOCK_X: f64 = 392.0;
const FOCUS_BLOCK_Y: f64 = 164.0;
const FOCUS_BLOCK_Z: f64 = 18.0;
const FOCUS_BLOCK_CENTER_X: f64 = 270.0;
const FOCUS_BLOCK_CENTER_Y: f64 = 208.0;
const FOCUS_STEP_X: f64 = 34.0;
const FOCUS_STEP_Y: f64 = 104.0;
const FOCUS_STEP_PITCH_X: f64 = 39.0;
const FOCUS_STEP_START_Z: f64 = 3.0;
const FOCUS_STEP_DELTA_Z: f64 = 1.25;
const FOCUS_ACCEPTANCE_SPAN_Z: f64 = 10.0;

const FLAT_FIELD_TARGET_COUNT: usize = 4;
const FLAT_FIELD_CARRIER_X: f64 = 426.0;
const FLAT_FIELD_CARRIER_Y: f64 = 132.0;
const FLAT_FIELD_CARRIER_Z: f64 = 20.0;
const FLAT_FIELD_CENTER_X: f64 = -280.0;
const FLAT_FIELD_CENTER_Y: f64 = -48.0;
const FLAT_FIELD_WINDOW_D: f64 = 54.0;
const FLAT_FIELD_PITCH_X: f64 = 86.0;

const GRID_COLS: usize = 9;
const GRID_ROWS: usize = 7;
const GRID_PLATE_X: f64 = 364.0;
const GRID_PLATE_Y: f64 = 272.0;
const GRID_PLATE_Z: f64 = 14.0;
const GRID_CENTER_X: f64 = 286.0;
const GRID_CENTER_Y: f64 = -40.0;
const GRID_PITCH_X: f64 = 36.0;
const GRID_PITCH_Y: f64 = 32.0;
const GRID_MARK_D: f64 = 5.0;

const ILLUMINATION_MODE_COUNT: usize = 2;
const ILLUMINATION_REFERENCE_LANDS_PER_MODE: usize = 4;
const ILLUMINATION_LAND_COUNT: usize =
    ILLUMINATION_MODE_COUNT * ILLUMINATION_REFERENCE_LANDS_PER_MODE;
const ILLUMINATION_BAR_X: f64 = 840.0;
const ILLUMINATION_BAR_Y: f64 = 74.0;
const ILLUMINATION_BAR_Z: f64 = 14.0;
const ILLUMINATION_CENTER_Y: f64 = -250.0;
const ILLUMINATION_LAND_X: f64 = 74.0;
const ILLUMINATION_LAND_Y: f64 = 42.0;
const ILLUMINATION_PITCH_X: f64 = 92.0;

const TEMPERATURE_LOGGER_COUNT: usize = 2;
const LOGGER_BLOCK_X: f64 = 184.0;
const LOGGER_BLOCK_Y: f64 = 150.0;
const LOGGER_BLOCK_Z: f64 = 44.0;
const LOGGER_CENTER_X: f64 = -390.0;
const LOGGER_CENTER_Y: f64 = -150.0;
const LOGGER_POCKET_X: f64 = 132.0;
const LOGGER_POCKET_Y: f64 = 46.0;
const LOGGER_POCKET_Z: f64 = 34.0;
const LOGGER_PITCH_Y: f64 = 58.0;

const BARCODE_LAND_COUNT: usize = 8;
const CERTIFICATE_LAND_COUNT: usize = 2;
const TRACEABILITY_LAND_COUNT: usize = BARCODE_LAND_COUNT + CERTIFICATE_LAND_COUNT;
const TRACEABILITY_BAR_X: f64 = 880.0;
const TRACEABILITY_BAR_Y: f64 = 82.0;
const TRACEABILITY_BAR_Z: f64 = 10.0;
const TRACEABILITY_CENTER_Y: f64 = 318.0;

const STATUS_LANE_COUNT: usize = 3;
const STATUS_LANE_X: f64 = 246.0;
const STATUS_LANE_Y: f64 = 118.0;
const STATUS_LANE_Z: f64 = 26.0;
const STATUS_LANE_CENTER_Y: f64 = -295.0;
const STATUS_LANE_PITCH_X: f64 = 278.0;
const CLEAN_REFERENCE_CENTER_Y: f64 = 286.0;
const CLEAN_REFERENCE_Y: f64 = 64.0;
const USED_STATUS_CENTER_Y: f64 = STATUS_LANE_CENTER_Y;
const USED_STATUS_Y: f64 = STATUS_LANE_Y;
const CLEAN_USED_MIN_GAP: f64 = 430.0;
const STATUS_DIVIDER_Z: f64 = 60.0;

const DUST_COVER_SLOT_COUNT: usize = 3;
const DUST_CASSETTE_X: f64 = 226.0;
const DUST_CASSETTE_Y: f64 = 246.0;
const DUST_CASSETTE_Z: f64 = 58.0;
const DUST_CASSETTE_CENTER_X: f64 = 380.0;
const DUST_CASSETTE_CENTER_Y: f64 = 104.0;
const DUST_COVER_SLOT_X: f64 = 154.0;
const DUST_COVER_SLOT_Y: f64 = 26.0;
const DUST_COVER_SLOT_Z: f64 = 42.0;

const CAMERA_BRIDGE_SPAN_X: f64 = 850.0;
const CAMERA_BRIDGE_DEPTH_Y: f64 = 58.0;
const CAMERA_BRIDGE_CLEAR_Z: f64 = 190.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 32.0;
const CAMERA_BRIDGE_CENTER_Y: f64 = -16.0;
const CAMERA_WINDOW_X: f64 = 260.0;
const CAMERA_WINDOW_Y: f64 = 42.0;

const ROBOT_KEEP_OUT_X: f64 = 940.0;
const ROBOT_KEEP_OUT_Y: f64 = 132.0;
const ROBOT_KEEP_OUT_Z: f64 = 158.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_SCANNER_SERVICE_CLEARANCE: f64 = 190.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_enclosure();
    export(OUTPUTS[0], &base);

    let slide_nests = calibration_slide_nests();
    export(OUTPUTS[1], &slide_nests);

    let focus_phantoms = focus_step_phantom_blocks();
    export(OUTPUTS[2], &focus_phantoms);

    let flat_field = flat_field_target_carrier();
    export(OUTPUTS[3], &flat_field);

    let fiducials = fiducial_grid_plate();
    export(OUTPUTS[4], &fiducials);

    let illumination = illumination_reference_lands();
    export(OUTPUTS[5], &illumination);

    let logger = temperature_logger_pocket();
    export(OUTPUTS[6], &logger);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let status_lanes = released_hold_reject_lanes();
    export(OUTPUTS[8], &status_lanes);

    let dust_cover = dust_cover_cassette();
    export(OUTPUTS[9], &dust_cover);

    let evidence_bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &evidence_bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + slide_nests
        + focus_phantoms
        + flat_field
        + fiducials
        + illumination
        + logger
        + traceability
        + status_lanes
        + dust_cover
        + evidence_bridge
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed optical focus drift calibration station:");
    println!("  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm");
    println!(
        "  Reference handling:        {SLIDE_NEST_COUNT} calibration slide nests, {DUST_COVER_SLOT_COUNT} dust cover slots, {TRACEABILITY_LAND_COUNT} barcode/certificate lands"
    );
    println!(
        "  Optical references:        {FOCUS_STEP_COUNT} focus steps over {:.1}mm, {FLAT_FIELD_TARGET_COUNT} flat-field targets, {GRID_COLS}x{GRID_ROWS} fiducial grid, {ILLUMINATION_LAND_COUNT} transmitted/epi lands",
        focus_step_span_z()
    );
    println!(
        "  Evidence and custody:      {TEMPERATURE_LOGGER_COUNT} logger pockets, {} status lanes, {:.0}mm evidence bridge clearance",
        STATUS_LANE_NAMES.len(),
        CAMERA_BRIDGE_CLEAR_Z
    );
    println!(
        "  Keepouts:                  {:.0}mm front service, {:.0}mm rear service, {:.0}mm scanner/service side clearance, {ROBOT_KEEP_OUT_WINDOWS} robot sweep windows",
        FRONT_SERVICE_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        RIGHT_SCANNER_SERVICE_CLEARANCE
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_enclosure() -> Part {
    let deck = centered_cube(
        format!("{BIN_PREFIX}_base_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_floor = centered_cube(
        format!("{BIN_PREFIX}_closed_workflow_recessed_floor"),
        STATION_X - 112.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z - 3.2);
    let front_transfer_slot = centered_cube(
        format!("{BIN_PREFIX}_front_robot_transfer_slot"),
        562.0,
        PERIMETER_RIM_W + 8.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 10.0, BASE_Z + 9.0);
    let certificate_pass_slot = centered_cube(
        format!("{BIN_PREFIX}_certificate_pass_through_slot"),
        170.0,
        PERIMETER_RIM_W + 8.0,
        15.0,
    )
    .translate(
        -STATION_X / 2.0 + 174.0,
        STATION_Y / 2.0 - 10.0,
        BASE_Z + 7.5,
    );

    deck - recessed_floor - front_transfer_slot - certificate_pass_slot - mounting_slots()
        + perimeter_rim()
        + gasket_frame_xy(
            format!("{BIN_PREFIX}_closed_lid_gasket_frame"),
            STATION_X - 92.0,
            STATION_Y - 96.0,
            GASKET_RAIL_W,
            GASKET_RAIL_Z,
        )
        .translate(0.0, 0.0, BASE_Z + GASKET_RAIL_Z / 2.0)
        + light_trap_labyrinth()
        + datum_targets()
}

fn perimeter_rim() -> Part {
    let left = centered_cube(
        format!("{BIN_PREFIX}_left_light_tight_rim"),
        PERIMETER_RIM_W,
        STATION_Y,
        PERIMETER_RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + PERIMETER_RIM_W / 2.0,
        0.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{BIN_PREFIX}_right_light_tight_rim"),
        PERIMETER_RIM_W,
        STATION_Y,
        PERIMETER_RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - PERIMETER_RIM_W / 2.0,
        0.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{BIN_PREFIX}_rear_light_tight_rim"),
        STATION_X,
        PERIMETER_RIM_W,
        PERIMETER_RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - PERIMETER_RIM_W / 2.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let front_left = centered_cube(
        format!("{BIN_PREFIX}_front_left_low_transfer_lip"),
        205.0,
        PERIMETER_RIM_W,
        21.0,
    )
    .translate(
        -STATION_X / 2.0 + 102.5,
        -STATION_Y / 2.0 + PERIMETER_RIM_W / 2.0,
        BASE_Z + 10.5,
    );
    let front_right = centered_cube(
        format!("{BIN_PREFIX}_front_right_low_transfer_lip"),
        205.0,
        PERIMETER_RIM_W,
        21.0,
    )
    .translate(
        STATION_X / 2.0 - 102.5,
        -STATION_Y / 2.0 + PERIMETER_RIM_W / 2.0,
        BASE_Z + 10.5,
    );

    left + right + rear + front_left + front_right
}

fn light_trap_labyrinth() -> Part {
    let rear_baffle = centered_cube(
        format!("{BIN_PREFIX}_rear_lid_light_trap_baffle"),
        STATION_X - 146.0,
        10.0,
        LIGHT_TRAP_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 58.0, BASE_Z + LIGHT_TRAP_Z / 2.0);
    let left_baffle = centered_cube(
        format!("{BIN_PREFIX}_left_lid_light_trap_baffle"),
        10.0,
        STATION_Y - 170.0,
        LIGHT_TRAP_Z,
    )
    .translate(-STATION_X / 2.0 + 56.0, 0.0, BASE_Z + LIGHT_TRAP_Z / 2.0);
    let right_baffle = centered_cube(
        format!("{BIN_PREFIX}_right_lid_light_trap_baffle"),
        10.0,
        STATION_Y - 170.0,
        LIGHT_TRAP_Z,
    )
    .translate(STATION_X / 2.0 - 56.0, 0.0, BASE_Z + LIGHT_TRAP_Z / 2.0);
    let front_baffle = centered_cube(
        format!("{BIN_PREFIX}_front_lid_light_trap_baffle"),
        STATION_X - 250.0,
        10.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 64.0, BASE_Z + 10.0);

    rear_baffle + left_baffle + right_baffle + front_baffle
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty(format!("{BIN_PREFIX}_m6_mounting_slots"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let bore = centered_cylinder(
            format!("{BIN_PREFIX}_m6_mount_bore_{i}"),
            6.6 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("{BIN_PREFIX}_m6_mount_slot_relief_{i}"),
            24.0,
            7.0,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        slots = slots + bore + slot;
    }
    slots
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{BIN_PREFIX}_robot_datum_targets"));
    for (i, (x, y)) in datum_points().iter().enumerate() {
        let target =
            centered_cylinder(format!("{BIN_PREFIX}_datum_target_disc_{i}"), 11.0, 3.0, 42)
                .translate(*x, *y, BASE_Z + 1.5);
        let center = centered_cylinder(
            format!("{BIN_PREFIX}_datum_target_center_{i}"),
            2.2,
            4.0,
            20,
        )
        .translate(*x, *y, BASE_Z + 1.5);
        targets = targets + (target - center);
    }
    targets
}

fn calibration_slide_nests() -> Part {
    let rack = centered_cube(
        format!("{BIN_PREFIX}_calibration_slide_nest_rack"),
        SLIDE_RACK_X,
        SLIDE_RACK_Y,
        SLIDE_RACK_Z,
    )
    .translate(
        SLIDE_RACK_CENTER_X,
        SLIDE_RACK_CENTER_Y,
        BASE_Z + SLIDE_RACK_Z / 2.0,
    );

    let mut pockets = Part::empty(format!("{BIN_PREFIX}_calibration_slide_nest_pockets"));
    let mut rails = Part::empty(format!("{BIN_PREFIX}_calibration_slide_nest_rails"));
    for index in 0..SLIDE_NEST_COUNT {
        let (x, y) = slide_nest_position(index);
        pockets = pockets
            + centered_cube(
                format!("{BIN_PREFIX}_calibration_slide_recess_{index}"),
                SLIDE_SLOT_X,
                SLIDE_SLOT_Y,
                SLIDE_SLOT_DEPTH + 2.0,
            )
            .translate(x, y, BASE_Z + SLIDE_RACK_Z - SLIDE_SLOT_DEPTH / 2.0);
        rails = rails
            + centered_cube(
                format!("{BIN_PREFIX}_slide_nest_left_datum_rail_{index}"),
                8.0,
                SLIDE_SLOT_Y + 18.0,
                18.0,
            )
            .translate(x - SLIDE_SLOT_X / 2.0 - 8.0, y, BASE_Z + SLIDE_RACK_Z + 9.0)
            + centered_cube(
                format!("{BIN_PREFIX}_slide_nest_rear_stop_{index}"),
                SLIDE_SLOT_X + 18.0,
                8.0,
                18.0,
            )
            .translate(x, y + SLIDE_SLOT_Y / 2.0 + 8.0, BASE_Z + SLIDE_RACK_Z + 9.0)
            + centered_cube(
                format!("{BIN_PREFIX}_slide_nest_id_land_{index}"),
                54.0,
                12.0,
                4.0,
            )
            .translate(
                x,
                y - SLIDE_SLOT_Y / 2.0 - 15.0,
                BASE_Z + SLIDE_RACK_Z + 2.0,
            );
    }

    rack - pockets + rails + clean_reference_pack_land()
}

fn clean_reference_pack_land() -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_clean_unused_reference_pack_lane"),
        338.0,
        CLEAN_REFERENCE_Y,
        9.0,
    )
    .translate(
        SLIDE_RACK_CENTER_X,
        CLEAN_REFERENCE_CENTER_Y,
        BASE_Z + SLIDE_RACK_Z + 4.5,
    )
}

fn focus_step_phantom_blocks() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_focus_step_phantom_base"),
        FOCUS_BLOCK_X,
        FOCUS_BLOCK_Y,
        FOCUS_BLOCK_Z,
    )
    .translate(
        FOCUS_BLOCK_CENTER_X,
        FOCUS_BLOCK_CENTER_Y,
        BASE_Z + FOCUS_BLOCK_Z / 2.0,
    );
    let mut steps = Part::empty(format!("{BIN_PREFIX}_focus_step_phantom_blocks"));
    for index in 0..FOCUS_STEP_COUNT {
        let step_z = focus_step_height(index);
        let x = lane_x(index, FOCUS_STEP_COUNT, FOCUS_STEP_PITCH_X) + FOCUS_BLOCK_CENTER_X;
        let y = FOCUS_BLOCK_CENTER_Y - 8.0;
        steps = steps
            + centered_cube(
                format!("{BIN_PREFIX}_focus_step_height_reference_{index}"),
                FOCUS_STEP_X,
                FOCUS_STEP_Y,
                step_z,
            )
            .translate(x, y, BASE_Z + FOCUS_BLOCK_Z + step_z / 2.0)
            + centered_cylinder(
                format!("{BIN_PREFIX}_focus_step_specular_dot_{index}"),
                5.0,
                1.8,
                24,
            )
            .translate(
                x,
                y - FOCUS_STEP_Y / 2.0 + 20.0,
                BASE_Z + FOCUS_BLOCK_Z + step_z + 0.9,
            )
            + centered_cube(
                format!("{BIN_PREFIX}_focus_step_label_land_{index}"),
                FOCUS_STEP_X,
                12.0,
                3.0,
            )
            .translate(
                x,
                y + FOCUS_STEP_Y / 2.0 + 15.0,
                BASE_Z + FOCUS_BLOCK_Z + 1.5,
            );
    }

    base + steps + focus_zero_datum()
}

fn focus_zero_datum() -> Part {
    let datum = centered_cube(
        format!("{BIN_PREFIX}_focus_zero_height_reference_bar"),
        FOCUS_BLOCK_X - 62.0,
        10.0,
        5.0,
    )
    .translate(
        FOCUS_BLOCK_CENTER_X,
        FOCUS_BLOCK_CENTER_Y - FOCUS_BLOCK_Y / 2.0 + 20.0,
        BASE_Z + FOCUS_BLOCK_Z + 2.5,
    );
    let thermal_anchor = centered_cylinder(
        format!("{BIN_PREFIX}_focus_phantom_thermal_anchor_bore"),
        7.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        FOCUS_BLOCK_CENTER_X - FOCUS_BLOCK_X / 2.0 + 48.0,
        FOCUS_BLOCK_CENTER_Y - FOCUS_BLOCK_Y / 2.0 + 20.0,
        BASE_Z + FOCUS_BLOCK_Z / 2.0,
    );

    datum - thermal_anchor
}

fn flat_field_target_carrier() -> Part {
    let carrier = centered_cube(
        format!("{BIN_PREFIX}_flat_field_target_carrier"),
        FLAT_FIELD_CARRIER_X,
        FLAT_FIELD_CARRIER_Y,
        FLAT_FIELD_CARRIER_Z,
    )
    .translate(
        FLAT_FIELD_CENTER_X,
        FLAT_FIELD_CENTER_Y,
        BASE_Z + FLAT_FIELD_CARRIER_Z / 2.0,
    );
    let mut windows = Part::empty(format!("{BIN_PREFIX}_flat_field_diffuser_windows"));
    let mut lands = Part::empty(format!("{BIN_PREFIX}_flat_field_reference_lands"));
    for index in 0..FLAT_FIELD_TARGET_COUNT {
        let x = FLAT_FIELD_CENTER_X + lane_x(index, FLAT_FIELD_TARGET_COUNT, FLAT_FIELD_PITCH_X);
        windows = windows
            + centered_cylinder(
                format!("{BIN_PREFIX}_flat_field_target_window_{index}"),
                FLAT_FIELD_WINDOW_D / 2.0,
                FLAT_FIELD_CARRIER_Z + 6.0,
                48,
            )
            .translate(x, FLAT_FIELD_CENTER_Y, BASE_Z + FLAT_FIELD_CARRIER_Z / 2.0);
        lands = lands
            + centered_cube(
                format!("{BIN_PREFIX}_flat_field_target_diffuser_land_{index}"),
                FLAT_FIELD_WINDOW_D + 18.0,
                FLAT_FIELD_WINDOW_D + 18.0,
                4.0,
            )
            .translate(x, FLAT_FIELD_CENTER_Y, BASE_Z + FLAT_FIELD_CARRIER_Z + 2.0)
            + centered_cube(
                format!("{BIN_PREFIX}_flat_field_density_label_{index}"),
                56.0,
                12.0,
                3.0,
            )
            .translate(
                x,
                FLAT_FIELD_CENTER_Y - FLAT_FIELD_CARRIER_Y / 2.0 + 14.0,
                BASE_Z + FLAT_FIELD_CARRIER_Z + 1.5,
            );
    }

    carrier - windows + lands
}

fn fiducial_grid_plate() -> Part {
    let plate = centered_cube(
        format!("{BIN_PREFIX}_fiducial_grid_plate"),
        GRID_PLATE_X,
        GRID_PLATE_Y,
        GRID_PLATE_Z,
    )
    .translate(GRID_CENTER_X, GRID_CENTER_Y, BASE_Z + GRID_PLATE_Z / 2.0);
    let mut marks = Part::empty(format!("{BIN_PREFIX}_fiducial_grid_marks"));
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let (x, y) = grid_mark_position(row, col);
            let radius = if row == 0 && col == 0 {
                GRID_MARK_D
            } else {
                GRID_MARK_D / 2.0
            };
            marks = marks
                + centered_cylinder(
                    format!("{BIN_PREFIX}_fiducial_grid_mark_{row}_{col}"),
                    radius,
                    2.4,
                    22,
                )
                .translate(x, y, BASE_Z + GRID_PLATE_Z + 1.2);
        }
    }

    plate + marks + fiducial_grid_edge_codes()
}

fn fiducial_grid_edge_codes() -> Part {
    let mut codes = Part::empty(format!("{BIN_PREFIX}_fiducial_grid_edge_codes"));
    for col in 0..GRID_COLS {
        let x = GRID_CENTER_X + lane_x(col, GRID_COLS, GRID_PITCH_X);
        codes = codes
            + centered_cube(
                format!("{BIN_PREFIX}_fiducial_grid_column_code_{col}"),
                12.0,
                6.0 + col as f64,
                2.5,
            )
            .translate(
                x,
                GRID_CENTER_Y + GRID_PLATE_Y / 2.0 - 18.0,
                BASE_Z + GRID_PLATE_Z + 1.25,
            );
    }
    codes
}

fn illumination_reference_lands() -> Part {
    let bar = centered_cube(
        format!("{BIN_PREFIX}_illumination_reference_bar"),
        ILLUMINATION_BAR_X,
        ILLUMINATION_BAR_Y,
        ILLUMINATION_BAR_Z,
    )
    .translate(
        0.0,
        ILLUMINATION_CENTER_Y,
        BASE_Z + ILLUMINATION_BAR_Z / 2.0,
    );
    let mut lands = Part::empty(format!(
        "{BIN_PREFIX}_transmitted_epi_illumination_reference_lands"
    ));
    for mode in 0..ILLUMINATION_MODE_COUNT {
        for index in 0..ILLUMINATION_REFERENCE_LANDS_PER_MODE {
            let global = mode * ILLUMINATION_REFERENCE_LANDS_PER_MODE + index;
            let mode_offset = if mode == 0 { -230.0 } else { 230.0 };
            let x = mode_offset
                + lane_x(
                    index,
                    ILLUMINATION_REFERENCE_LANDS_PER_MODE,
                    ILLUMINATION_PITCH_X,
                );
            let name = if mode == 0 { "transmitted" } else { "epi" };
            lands = lands
                + centered_cube(
                    format!("{BIN_PREFIX}_{name}_illumination_reference_land_{index}"),
                    ILLUMINATION_LAND_X,
                    ILLUMINATION_LAND_Y,
                    5.0,
                )
                .translate(
                    x,
                    ILLUMINATION_CENTER_Y,
                    BASE_Z + ILLUMINATION_BAR_Z + 2.5,
                )
                - centered_cylinder(
                    format!("{BIN_PREFIX}_{name}_illumination_aperture_{index}"),
                    7.0 + global as f64,
                    ILLUMINATION_BAR_Z + 9.0,
                    32,
                )
                .translate(
                    x,
                    ILLUMINATION_CENTER_Y,
                    BASE_Z + ILLUMINATION_BAR_Z / 2.0,
                );
        }
    }
    let separator = centered_cube(
        format!("{BIN_PREFIX}_illumination_mode_separator"),
        12.0,
        ILLUMINATION_BAR_Y + 18.0,
        30.0,
    )
    .translate(
        0.0,
        ILLUMINATION_CENTER_Y,
        BASE_Z + ILLUMINATION_BAR_Z + 15.0,
    );

    bar + lands + separator
}

fn temperature_logger_pocket() -> Part {
    let block = centered_cube(
        format!("{BIN_PREFIX}_temperature_logger_block"),
        LOGGER_BLOCK_X,
        LOGGER_BLOCK_Y,
        LOGGER_BLOCK_Z,
    )
    .translate(
        LOGGER_CENTER_X,
        LOGGER_CENTER_Y,
        BASE_Z + LOGGER_BLOCK_Z / 2.0,
    );
    let mut pockets = Part::empty(format!("{BIN_PREFIX}_temperature_logger_pockets"));
    let mut retention = Part::empty(format!("{BIN_PREFIX}_temperature_logger_retention"));
    for index in 0..TEMPERATURE_LOGGER_COUNT {
        let y = LOGGER_CENTER_Y + lane_x(index, TEMPERATURE_LOGGER_COUNT, LOGGER_PITCH_Y);
        pockets = pockets
            + centered_cube(
                format!("{BIN_PREFIX}_temperature_logger_pocket_{index}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_Z,
            )
            .translate(
                LOGGER_CENTER_X,
                y,
                BASE_Z + LOGGER_BLOCK_Z - LOGGER_POCKET_Z / 2.0,
            );
        retention = retention
            + centered_cube(
                format!("{BIN_PREFIX}_temperature_logger_clip_{index}"),
                LOGGER_POCKET_X + 18.0,
                8.0,
                16.0,
            )
            .translate(
                LOGGER_CENTER_X,
                y + LOGGER_POCKET_Y / 2.0 + 7.0,
                BASE_Z + 36.0,
            );
    }
    let cable_gland = centered_cylinder(
        format!("{BIN_PREFIX}_logger_cable_gland"),
        8.0,
        LOGGER_BLOCK_X + 18.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        LOGGER_CENTER_X,
        LOGGER_CENTER_Y - LOGGER_BLOCK_Y / 2.0 + 18.0,
        BASE_Z + 20.0,
    );
    let witness_thermowell = centered_cylinder(
        format!("{BIN_PREFIX}_logger_reference_thermowell"),
        5.0,
        LOGGER_BLOCK_Y + 18.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        LOGGER_CENTER_X + LOGGER_BLOCK_X / 2.0 - 24.0,
        LOGGER_CENTER_Y,
        BASE_Z + 25.0,
    );

    block - pockets - cable_gland - witness_thermowell + retention
}

fn barcode_certificate_lands() -> Part {
    let bar = centered_cube(
        format!("{BIN_PREFIX}_barcode_certificate_land_bar"),
        TRACEABILITY_BAR_X,
        TRACEABILITY_BAR_Y,
        TRACEABILITY_BAR_Z,
    )
    .translate(
        0.0,
        TRACEABILITY_CENTER_Y,
        BASE_Z + TRACEABILITY_BAR_Z / 2.0,
    );
    let mut lands = Part::empty(format!("{BIN_PREFIX}_barcode_certificate_lands"));
    for index in 0..BARCODE_LAND_COUNT {
        let x = -TRACEABILITY_BAR_X / 2.0 + 58.0 + index as f64 * 72.0;
        lands = lands
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_land_{index}"),
                58.0,
                28.0,
                3.0,
            )
            .translate(
                x,
                TRACEABILITY_CENTER_Y - 15.0,
                BASE_Z + TRACEABILITY_BAR_Z + 1.5,
            );
    }
    for index in 0..CERTIFICATE_LAND_COUNT {
        let x = TRACEABILITY_BAR_X / 2.0 - 184.0 + index as f64 * 128.0;
        lands = lands
            + centered_cube(
                format!("{BIN_PREFIX}_certificate_land_{index}"),
                108.0,
                54.0,
                4.0,
            )
            .translate(
                x,
                TRACEABILITY_CENTER_Y + 5.0,
                BASE_Z + TRACEABILITY_BAR_Z + 2.0,
            )
            + centered_cylinder(
                format!("{BIN_PREFIX}_certificate_corner_datum_{index}"),
                5.0,
                2.0,
                24,
            )
            .translate(
                x - 42.0,
                TRACEABILITY_CENTER_Y + 24.0,
                BASE_Z + TRACEABILITY_BAR_Z + 5.0,
            );
    }
    bar + lands
}

fn released_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty(format!("{BIN_PREFIX}_released_hold_reject_lanes"));
    for (index, name) in STATUS_LANE_NAMES.iter().enumerate() {
        let x = status_lane_x(index);
        let lane = centered_cube(
            format!("{BIN_PREFIX}_{name}_status_lane_tray"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(x, STATUS_LANE_CENTER_Y, BASE_Z + STATUS_LANE_Z / 2.0);
        let pocket = centered_cube(
            format!("{BIN_PREFIX}_{name}_status_lane_cassette_recess"),
            STATUS_LANE_X - 52.0,
            STATUS_LANE_Y - 38.0,
            13.0,
        )
        .translate(x, STATUS_LANE_CENTER_Y, BASE_Z + STATUS_LANE_Z - 13.0 / 2.0);
        let barcode = centered_cube(
            format!("{BIN_PREFIX}_{name}_status_lane_barcode_land"),
            88.0,
            18.0,
            4.0,
        )
        .translate(
            x,
            STATUS_LANE_CENTER_Y - STATUS_LANE_Y / 2.0 + 18.0,
            BASE_Z + STATUS_LANE_Z + 2.0,
        );
        lanes = lanes + (lane - pocket) + barcode;
    }

    lanes + status_lane_dividers() + used_return_lane()
}

fn status_lane_dividers() -> Part {
    let left_divider = centered_cube(
        format!("{BIN_PREFIX}_released_hold_status_divider"),
        10.0,
        STATUS_LANE_Y + 28.0,
        STATUS_DIVIDER_Z,
    )
    .translate(
        (status_lane_x(0) + status_lane_x(1)) / 2.0,
        STATUS_LANE_CENTER_Y,
        BASE_Z + STATUS_DIVIDER_Z / 2.0,
    );
    let right_divider = centered_cube(
        format!("{BIN_PREFIX}_hold_reject_status_divider"),
        10.0,
        STATUS_LANE_Y + 28.0,
        STATUS_DIVIDER_Z,
    )
    .translate(
        (status_lane_x(1) + status_lane_x(2)) / 2.0,
        STATUS_LANE_CENTER_Y,
        BASE_Z + STATUS_DIVIDER_Z / 2.0,
    );
    let clean_used_wall = centered_cube(
        format!("{BIN_PREFIX}_clean_used_workflow_segregation_wall"),
        STATION_X - 132.0,
        12.0,
        54.0,
    )
    .translate(0.0, -184.0, BASE_Z + 27.0);

    left_divider + right_divider + clean_used_wall
}

fn used_return_lane() -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_used_reference_return_lane"),
        332.0,
        46.0,
        8.0,
    )
    .translate(
        0.0,
        USED_STATUS_CENTER_Y + 86.0,
        BASE_Z + STATUS_LANE_Z + 4.0,
    )
}

fn dust_cover_cassette() -> Part {
    let cassette = centered_cube(
        format!("{BIN_PREFIX}_dust_cover_cassette_body"),
        DUST_CASSETTE_X,
        DUST_CASSETTE_Y,
        DUST_CASSETTE_Z,
    )
    .translate(
        DUST_CASSETTE_CENTER_X,
        DUST_CASSETTE_CENTER_Y,
        BASE_Z + DUST_CASSETTE_Z / 2.0,
    );
    let mut slots = Part::empty(format!("{BIN_PREFIX}_dust_cover_cassette_slots"));
    let mut handles = Part::empty(format!("{BIN_PREFIX}_dust_cover_cassette_handles"));
    for index in 0..DUST_COVER_SLOT_COUNT {
        let y = DUST_CASSETTE_CENTER_Y + lane_x(index, DUST_COVER_SLOT_COUNT, 66.0);
        slots = slots
            + centered_cube(
                format!("{BIN_PREFIX}_dust_cover_slot_{index}"),
                DUST_COVER_SLOT_X,
                DUST_COVER_SLOT_Y,
                DUST_COVER_SLOT_Z,
            )
            .translate(
                DUST_CASSETTE_CENTER_X,
                y,
                BASE_Z + DUST_CASSETTE_Z - DUST_COVER_SLOT_Z / 2.0,
            );
        handles = handles
            + centered_cube(
                format!("{BIN_PREFIX}_dust_cover_pull_tab_{index}"),
                92.0,
                9.0,
                20.0,
            )
            .translate(
                DUST_CASSETTE_CENTER_X + DUST_CASSETTE_X / 2.0 - 18.0,
                y,
                BASE_Z + DUST_CASSETTE_Z + 10.0,
            );
    }
    let dust_sensor_land = centered_cube(
        format!("{BIN_PREFIX}_dust_cover_particle_witness_land"),
        92.0,
        42.0,
        4.0,
    )
    .translate(
        DUST_CASSETTE_CENTER_X,
        DUST_CASSETTE_CENTER_Y + DUST_CASSETTE_Y / 2.0 - 38.0,
        BASE_Z + DUST_CASSETTE_Z + 2.0,
    );

    cassette - slots + handles + dust_sensor_land
}

fn evidence_camera_bridge() -> Part {
    let left_upright = centered_cube(
        format!("{BIN_PREFIX}_evidence_bridge_left_upright"),
        32.0,
        CAMERA_BRIDGE_DEPTH_Y,
        CAMERA_BRIDGE_CLEAR_Z,
    )
    .translate(
        -CAMERA_BRIDGE_SPAN_X / 2.0,
        CAMERA_BRIDGE_CENTER_Y,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z / 2.0,
    );
    let right_upright = centered_cube(
        format!("{BIN_PREFIX}_evidence_bridge_right_upright"),
        32.0,
        CAMERA_BRIDGE_DEPTH_Y,
        CAMERA_BRIDGE_CLEAR_Z,
    )
    .translate(
        CAMERA_BRIDGE_SPAN_X / 2.0,
        CAMERA_BRIDGE_CENTER_Y,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z / 2.0,
    );
    let top_beam = centered_cube(
        format!("{BIN_PREFIX}_evidence_camera_scanner_bridge_beam"),
        CAMERA_BRIDGE_SPAN_X + 64.0,
        CAMERA_BRIDGE_DEPTH_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_CENTER_Y,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let camera_window = centered_cube(
        format!("{BIN_PREFIX}_evidence_camera_window_cutout"),
        CAMERA_WINDOW_X,
        CAMERA_WINDOW_Y,
        CAMERA_BRIDGE_BEAM_Z + 4.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_CENTER_Y,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let scanner_rail = centered_cube(
        format!("{BIN_PREFIX}_scanner_linear_reference_rail"),
        CAMERA_BRIDGE_SPAN_X - 160.0,
        12.0,
        18.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_CENTER_Y + CAMERA_BRIDGE_DEPTH_Y / 2.0 + 12.0,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z + 9.0,
    );
    let evidence_light_shield = centered_cube(
        format!("{BIN_PREFIX}_evidence_bridge_light_shield"),
        CAMERA_BRIDGE_SPAN_X - 120.0,
        10.0,
        64.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_CENTER_Y - CAMERA_BRIDGE_DEPTH_Y / 2.0 - 9.0,
        BASE_Z + CAMERA_BRIDGE_CLEAR_Z - 32.0,
    );

    left_upright + right_upright + (top_beam - camera_window) + scanner_rail + evidence_light_shield
}

fn robot_service_keepouts() -> Part {
    let sweep = centered_cube(
        format!("{BIN_PREFIX}_robot_pick_place_sweep_keepout"),
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(0.0, 40.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    let front_service = centered_cube(
        format!("{BIN_PREFIX}_front_status_lane_service_keepout"),
        STATION_X - 140.0,
        FRONT_SERVICE_CLEARANCE,
        86.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 43.0,
    );
    let rear_service = centered_cube(
        format!("{BIN_PREFIX}_rear_certificate_service_keepout"),
        STATION_X - 210.0,
        REAR_SERVICE_CLEARANCE,
        84.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 42.0,
    );
    let side_service = centered_cube(
        format!("{BIN_PREFIX}_right_scanner_bridge_service_keepout"),
        RIGHT_SCANNER_SERVICE_CLEARANCE,
        STATION_Y - 170.0,
        110.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SCANNER_SERVICE_CLEARANCE / 2.0,
        6.0,
        BASE_Z + 55.0,
    );

    sweep - keepout_windows() + front_service + rear_service + side_service
}

fn keepout_windows() -> Part {
    let mut windows = Part::empty(format!("{BIN_PREFIX}_robot_keepout_windows"));
    for index in 0..ROBOT_KEEP_OUT_WINDOWS {
        windows = windows
            + centered_cube(
                format!("{BIN_PREFIX}_robot_keepout_window_{index}"),
                154.0,
                ROBOT_KEEP_OUT_Y + 8.0,
                74.0,
            )
            .translate(
                lane_x(index, ROBOT_KEEP_OUT_WINDOWS, 202.0),
                40.0,
                BASE_Z + ROBOT_KEEP_OUT_Z / 2.0,
            );
    }
    windows
}

fn gasket_frame_xy(name: String, x: f64, y: f64, rail: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear =
        centered_cube(format!("{name}_rear"), x, rail, z).translate(0.0, y / 2.0 - rail / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(
        -x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(
        x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );

    front + rear + left + right
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 52.0),
        (-178.0, -STATION_Y / 2.0 + 52.0),
        (178.0, -STATION_Y / 2.0 + 52.0),
        (-178.0, STATION_Y / 2.0 - 52.0),
        (178.0, STATION_Y / 2.0 - 52.0),
    ]
}

fn datum_points() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 82.0, STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 86.0),
        (-STATION_X / 2.0 + 82.0, -STATION_Y / 2.0 + 86.0),
        (STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 + 86.0),
    ]
}

fn slide_nest_position(index: usize) -> (f64, f64) {
    let col = index % SLIDE_NEST_COLS;
    let row = index / SLIDE_NEST_COLS;
    (
        SLIDE_RACK_CENTER_X + lane_x(col, SLIDE_NEST_COLS, SLIDE_PITCH_X),
        SLIDE_RACK_CENTER_Y + lane_x(row, SLIDE_NEST_ROWS, SLIDE_PITCH_Y),
    )
}

fn grid_mark_position(row: usize, col: usize) -> (f64, f64) {
    (
        GRID_CENTER_X + lane_x(col, GRID_COLS, GRID_PITCH_X),
        GRID_CENTER_Y + lane_x(row, GRID_ROWS, GRID_PITCH_Y),
    )
}

fn focus_step_height(index: usize) -> f64 {
    FOCUS_STEP_START_Z + index as f64 * FOCUS_STEP_DELTA_Z
}

fn focus_step_span_z() -> f64 {
    focus_step_height(FOCUS_STEP_COUNT - 1) - focus_step_height(0)
}

fn status_lane_x(index: usize) -> f64 {
    lane_x(index, STATUS_LANE_COUNT, STATUS_LANE_PITCH_X)
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(SLIDE_NEST_COUNT, SLIDE_NEST_COLS * SLIDE_NEST_ROWS);
    assert_eq!(
        ILLUMINATION_LAND_COUNT,
        ILLUMINATION_MODE_COUNT * ILLUMINATION_REFERENCE_LANDS_PER_MODE
    );
    assert_eq!(STATUS_LANE_COUNT, STATUS_LANE_NAMES.len());
    assert_eq!(
        TRACEABILITY_LAND_COUNT,
        BARCODE_LAND_COUNT + CERTIFICATE_LAND_COUNT
    );
    assert!(focus_step_span_z() >= FOCUS_ACCEPTANCE_SPAN_Z);

    let slide_edge_x = SLIDE_RACK_CENTER_X.abs() + SLIDE_RACK_X / 2.0;
    let focus_edge_x = FOCUS_BLOCK_CENTER_X.abs() + FOCUS_BLOCK_X / 2.0;
    let flat_edge_x = FLAT_FIELD_CENTER_X.abs() + FLAT_FIELD_CARRIER_X / 2.0;
    let grid_edge_x = GRID_CENTER_X.abs() + GRID_PLATE_X / 2.0;
    let dust_edge_x = DUST_CASSETTE_CENTER_X.abs() + DUST_CASSETTE_X / 2.0;
    assert!(slide_edge_x < STATION_X / 2.0 - PERIMETER_RIM_W);
    assert!(focus_edge_x < STATION_X / 2.0 - PERIMETER_RIM_W);
    assert!(flat_edge_x < STATION_X / 2.0 - PERIMETER_RIM_W);
    assert!(grid_edge_x < STATION_X / 2.0 - PERIMETER_RIM_W);
    assert!(dust_edge_x < STATION_X / 2.0 - PERIMETER_RIM_W);

    let traceability_top = TRACEABILITY_CENTER_Y + TRACEABILITY_BAR_Y / 2.0;
    let status_bottom = STATUS_LANE_CENTER_Y - STATUS_LANE_Y / 2.0;
    let clean_front_edge = CLEAN_REFERENCE_CENTER_Y - CLEAN_REFERENCE_Y / 2.0;
    let used_rear_edge = USED_STATUS_CENTER_Y + USED_STATUS_Y / 2.0;
    assert!(traceability_top < STATION_Y / 2.0 - PERIMETER_RIM_W);
    assert!(status_bottom > -STATION_Y / 2.0 + PERIMETER_RIM_W);
    assert!(clean_front_edge - used_rear_edge >= CLEAN_USED_MIN_GAP);
    assert!(CAMERA_BRIDGE_CLEAR_Z > BASE_Z + LOGGER_BLOCK_Z + 80.0);
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
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with(&format!("output/{BIN_PREFIX}_")));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_groups_cover_design_intent() {
        let features: BTreeSet<&str> = REQUIRED_FEATURES.iter().copied().collect();
        for required in [
            "calibration_slide_nests",
            "focus_step_phantom_blocks",
            "flat_field_target_carrier",
            "fiducial_grid_plate",
            "transmitted_epi_illumination_reference_lands",
            "temperature_logger_pocket",
            "barcode_certificate_lands",
            "released_hold_reject_lanes",
            "dust_cover_cassette",
            "evidence_camera_bridge",
            "robot_service_keepouts",
            "assembly_export",
        ] {
            assert!(features.contains(required));
        }
        assert_eq!(features.len(), REQUIRED_FEATURES.len());
    }

    #[test]
    fn optical_reference_capacity_matches_focus_drift_workflow() {
        assert_eq!(SLIDE_NEST_COUNT, 6);
        assert_eq!(FOCUS_STEP_COUNT, 9);
        assert_eq!(FLAT_FIELD_TARGET_COUNT, 4);
        assert_eq!(GRID_COLS * GRID_ROWS, 63);
        assert_eq!(ILLUMINATION_LAND_COUNT, 8);
        assert!(focus_step_height(0) < focus_step_height(FOCUS_STEP_COUNT - 1));
        assert!(focus_step_span_z() >= FOCUS_ACCEPTANCE_SPAN_Z);
    }

    #[test]
    fn major_dimensions_fit_inside_closed_station() {
        assert_layout();
        assert!(STATION_X >= 1000.0);
        assert!(STATION_Y >= 720.0);
        assert!(CAMERA_BRIDGE_SPAN_X < STATION_X - 120.0);
        assert!(CAMERA_BRIDGE_CLEAR_Z >= 180.0);
        assert!(DUST_CASSETTE_CENTER_X + DUST_CASSETTE_X / 2.0 < STATION_X / 2.0 - 18.0);
        assert!(LOGGER_CENTER_X - LOGGER_BLOCK_X / 2.0 > -STATION_X / 2.0 + 18.0);
    }

    #[test]
    fn clean_used_and_status_lanes_are_physically_separated() {
        assert_eq!(STATUS_LANE_COUNT, 3);
        assert_eq!(STATUS_LANE_NAMES, &["released", "hold", "reject"]);
        assert!(status_lane_x(0) < status_lane_x(1));
        assert!(status_lane_x(1) < status_lane_x(2));

        let clean_front_edge = CLEAN_REFERENCE_CENTER_Y - CLEAN_REFERENCE_Y / 2.0;
        let used_rear_edge = USED_STATUS_CENTER_Y + USED_STATUS_Y / 2.0;
        assert!(clean_front_edge - used_rear_edge >= CLEAN_USED_MIN_GAP);
        assert!(STATUS_DIVIDER_Z > STATUS_LANE_Z);
    }

    #[test]
    fn traceability_and_environmental_evidence_are_explicit() {
        assert_eq!(TEMPERATURE_LOGGER_COUNT, 2);
        assert_eq!(BARCODE_LAND_COUNT, 8);
        assert_eq!(CERTIFICATE_LAND_COUNT, 2);
        assert!(TRACEABILITY_LAND_COUNT >= SLIDE_NEST_COUNT + CERTIFICATE_LAND_COUNT);
        assert_eq!(DUST_COVER_SLOT_COUNT, STATUS_LANE_COUNT);
    }

    #[test]
    fn robot_and_service_keepouts_are_large_enough_for_module_validation() {
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 4);
        assert!(ROBOT_KEEP_OUT_X < STATION_X);
        assert!(ROBOT_KEEP_OUT_Z >= 150.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 340.0);
        assert!(REAR_SERVICE_CLEARANCE >= 240.0);
        assert!(RIGHT_SCANNER_SERVICE_CLEARANCE >= 180.0);
    }
}
