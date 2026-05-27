use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed aseptic tool-surface bioburden sampling station.
//
// Intent:
// - Hold reusable workcell tools in repeatable lanes for pre/post surface
//   sampling without opening the aseptic envelope.
// - Keep swab/contact-plate media, neutralizer vials, camera evidence, custody
//   tracking, suspect quarantine, robot handles, and fiducials physically
//   represented in the CAD.
// - Export each functional module as a separate STL plus one complete assembly
//   for integration layout review.
//
// This is mechanical architecture CAD only. It is not a microbiology method,
// sterilization validation, sampling plan, or release criterion.

const OUTPUTS: [&str; 12] = [
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_base_tray.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_tool_nest_lanes.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_swab_contact_plate_holders.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_neutralizer_vial_rack.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_before_after_camera_bridge.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_sterile_cover_envelope.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_sample_custody_lane.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_quarantine_bin.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_robot_grippable_handles.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_fiducial_datum_targets.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_service_keepouts.stl",
    "output/closed_aseptic_tool_surface_bioburden_sampling_station_assembly.stl",
];

const DECK_X: f64 = 1380.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SUMP_X: f64 = 1200.0;
const SUMP_Y: f64 = 700.0;
const SUMP_DEPTH: f64 = 7.0;
const DRAIN_PORT_D: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;

const TOOL_CENTER: (f64, f64) = (-305.0, 74.0);
const TOOL_PANEL_X: f64 = 650.0;
const TOOL_PANEL_Y: f64 = 512.0;
const TOOL_PANEL_Z: f64 = 44.0;
const TOOL_LANES: usize = 4;
const TOOL_LANE_PITCH_Y: f64 = 106.0;
const TOOL_LANE_LENGTH: f64 = 570.0;
const TOOL_LANE_CLEARANCE_D: f64 = 24.0;
const TOOL_CLAMPS_PER_LANE: usize = 3;
const TOOL_SAMPLING_WINDOWS_PER_LANE: usize = 2;

const HOLDER_CENTER: (f64, f64) = (430.0, 240.0);
const HOLDER_PANEL_X: f64 = 424.0;
const HOLDER_PANEL_Y: f64 = 238.0;
const HOLDER_PANEL_Z: f64 = 36.0;
const SWAB_WELLS: usize = 12;
const SWAB_WELL_D: f64 = 13.5;
const CONTACT_PLATE_WELLS: usize = 8;
const CONTACT_PLATE_D: f64 = 55.0;

const VIAL_RACK_CENTER: (f64, f64) = (420.0, -10.0);
const VIAL_RACK_X: f64 = 384.0;
const VIAL_RACK_Y: f64 = 230.0;
const VIAL_RACK_Z: f64 = 48.0;
const NEUTRALIZER_VIALS: usize = 16;
const VIAL_COLS: usize = 4;
const VIAL_D: f64 = 18.0;
const VIAL_PITCH: f64 = 42.0;
const VIAL_RETENTION_RAILS: usize = 4;

const CAMERA_CENTER: (f64, f64) = (-220.0, 96.0);
const CAMERA_BRIDGE_X: f64 = 780.0;
const CAMERA_BRIDGE_Y: f64 = 610.0;
const CAMERA_BRIDGE_Z: f64 = 238.0;
const CAMERA_BEAM_Z: f64 = 34.0;
const CAMERA_PODS: usize = 4;
const CAMERA_PITCH_X: f64 = 172.0;
const LED_BARS: usize = 4;

const COVER_CENTER: (f64, f64) = (0.0, 40.0);
const COVER_X: f64 = 1320.0;
const COVER_Y: f64 = 720.0;
const COVER_Z: f64 = 260.0;
const COVER_WALL_T: f64 = 10.0;
const COVER_GASKET_W: f64 = 12.0;
const COVER_LATCHES: usize = 10;
const TRANSFER_SLOT_X: f64 = 420.0;
const TRANSFER_SLOT_Z: f64 = 82.0;

const CUSTODY_CENTER: (f64, f64) = (-130.0, -328.0);
const CUSTODY_LANE_X: f64 = 760.0;
const CUSTODY_LANE_Y: f64 = 150.0;
const CUSTODY_LANE_Z: f64 = 30.0;
const CUSTODY_POSITIONS: usize = 8;
const CUSTODY_POSITION_PITCH_X: f64 = 84.0;
const CUSTODY_BARCODE_LANDS: usize = 8;
const TAMPER_SEAL_SLOTS: usize = 4;

const QUARANTINE_CENTER: (f64, f64) = (520.0, -270.0);
const QUARANTINE_BIN_X: f64 = 270.0;
const QUARANTINE_BIN_Y: f64 = 270.0;
const QUARANTINE_BIN_Z: f64 = 126.0;
const QUARANTINE_LID_Z: f64 = 20.0;
const QUARANTINE_SAMPLE_SLOTS: usize = 6;

const ROBOT_HANDLES: usize = 6;
const HANDLE_GRIP_X: f64 = 118.0;
const HANDLE_GRIP_D: f64 = 18.0;
const HANDLE_STANDOFF_Z: f64 = 58.0;

const FIDUCIALS: usize = 10;
const DATUM_PINS: usize = 6;
const FIDUCIAL_D: f64 = 28.0;
const DATUM_PIN_D: f64 = 8.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 460.0;
const REAR_MEDIA_SERVICE_KEEP_OUT_Y: f64 = 230.0;
const RIGHT_QUARANTINE_SERVICE_KEEP_OUT_X: f64 = 250.0;
const TOP_COVER_LIFT_KEEP_OUT_Z: f64 = 325.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let lanes = tool_nest_lanes();
    export(OUTPUTS[1], &lanes);

    let holders = swab_contact_plate_holders();
    export(OUTPUTS[2], &holders);

    let vials = neutralizer_vial_rack();
    export(OUTPUTS[3], &vials);

    let camera = before_after_evidence_camera_bridge();
    export(OUTPUTS[4], &camera);

    let cover = sterile_cover_envelope();
    export(OUTPUTS[5], &cover);

    let custody = sample_custody_lane();
    export(OUTPUTS[6], &custody);

    let quarantine = quarantine_bin();
    export(OUTPUTS[7], &quarantine);

    let handles = robot_grippable_handles();
    export(OUTPUTS[8], &handles);

    let fiducials = fiducial_datum_targets();
    export(OUTPUTS[9], &fiducials);

    let keepouts = service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + lanes
        + holders
        + vials
        + camera
        + cover
        + custody
        + quarantine
        + handles
        + fiducials
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!(
        "Closed aseptic tool surface bioburden sampling station: {DECK_X:.0}mm x {DECK_Y:.0}mm deck, {TOOL_LANES} tool nest lanes, {SWAB_WELLS} swab wells, {CONTACT_PLATE_WELLS} contact plate wells, {NEUTRALIZER_VIALS} neutralizer vials, and {OUTPUTS_LEN} STL outputs.",
        OUTPUTS_LEN = OUTPUTS.len()
    );
    println!(
        "Evidence and custody controls: {CAMERA_PODS} before/after camera pods, {LED_BARS} LED bars, {CUSTODY_POSITIONS} custody positions, {CUSTODY_BARCODE_LANDS} barcode lands, {TAMPER_SEAL_SLOTS} tamper-seal slots, {QUARANTINE_SAMPLE_SLOTS} quarantine sample slots."
    );
    println!(
        "Automation controls: {ROBOT_HANDLES} robot-grippable handles, {FIDUCIALS} fiducial targets, {DATUM_PINS} datum pins, {FRONT_ROBOT_KEEP_OUT_Y:.0}mm front robot keepout, {TOP_COVER_LIFT_KEEP_OUT_Z:.0}mm top cover lift keepout."
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "bioburden_sampling_station_closed_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let sump = centered_cube(
        "bioburden_sampling_station_recessed_wet_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        "bioburden_sampling_station_low_point_sump_drain",
        DRAIN_PORT_D / 2.0,
        RIM_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 14.0, DECK_Z - 5.0);

    deck - sump - drain - base_mount_holes()
        + deck_perimeter_curbs()
        + workflow_zone_witness_grooves()
        + media_drip_channels()
}

fn deck_perimeter_curbs() -> Part {
    let front = centered_cube(
        "bioburden_sampling_station_front_closed_tray_curb",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "bioburden_sampling_station_rear_closed_tray_curb",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "bioburden_sampling_station_left_closed_tray_curb",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "bioburden_sampling_station_right_closed_tray_curb",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn base_mount_holes() -> Part {
    let mut holes = Part::empty("bioburden_sampling_station_base_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let round = centered_cylinder(
            format!("bioburden_sampling_station_m6_mount_round_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("bioburden_sampling_station_m6_mount_slot_{i}"),
            28.0,
            MOUNT_HOLE_D + 0.6,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        holes = holes + round + slot;
    }
    holes
}

fn workflow_zone_witness_grooves() -> Part {
    let clean_boundary = centered_cube(
        "bioburden_sampling_station_clean_tool_zone_witness_groove",
        8.0,
        DECK_Y - 116.0,
        6.0,
    )
    .translate(122.0, 0.0, DECK_Z + 1.0);
    let sample_boundary = centered_cube(
        "bioburden_sampling_station_sample_custody_zone_witness_groove",
        CUSTODY_LANE_X + 52.0,
        8.0,
        6.0,
    )
    .translate(CUSTODY_CENTER.0, -245.0, DECK_Z + 1.0);
    let quarantine_boundary = centered_cube(
        "bioburden_sampling_station_quarantine_zone_witness_groove",
        8.0,
        290.0,
        6.0,
    )
    .translate(378.0, -130.0, DECK_Z + 1.0);

    clean_boundary + sample_boundary + quarantine_boundary
}

fn media_drip_channels() -> Part {
    let mut channels = Part::empty("bioburden_sampling_station_media_drip_channels");
    for (i, y) in [-258.0, -156.0, -54.0, 48.0, 150.0, 252.0]
        .iter()
        .enumerate()
    {
        channels = channels
            + centered_cube(
                format!("bioburden_sampling_station_sump_flow_channel_{i}"),
                DECK_X - 190.0,
                5.0,
                6.0,
            )
            .translate(0.0, *y, DECK_Z + 2.0);
    }
    channels
}

fn tool_nest_lanes() -> Part {
    let panel = centered_cube(
        "bioburden_sampling_station_tool_lane_panel",
        TOOL_PANEL_X,
        TOOL_PANEL_Y,
        TOOL_PANEL_Z,
    )
    .translate(TOOL_CENTER.0, TOOL_CENTER.1, DECK_Z + TOOL_PANEL_Z / 2.0);

    let mut lane_cuts = Part::empty("bioburden_sampling_station_tool_lane_clearances");
    let mut rails = Part::empty("bioburden_sampling_station_tool_lane_raised_rails");
    for lane in 0..TOOL_LANES {
        let y = tool_lane_y(lane);
        let trough = centered_cylinder(
            format!("bioburden_sampling_station_tool_lane_trough_{lane}"),
            TOOL_LANE_CLEARANCE_D / 2.0,
            TOOL_LANE_LENGTH,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            TOOL_CENTER.0,
            TOOL_CENTER.1 + y,
            DECK_Z + TOOL_PANEL_Z - 8.0,
        );
        lane_cuts = lane_cuts + trough;

        let left_rail = centered_cube(
            format!("bioburden_sampling_station_tool_lane_left_rail_{lane}"),
            TOOL_LANE_LENGTH,
            8.0,
            18.0,
        )
        .translate(
            TOOL_CENTER.0,
            TOOL_CENTER.1 + y - 24.0,
            DECK_Z + TOOL_PANEL_Z + 9.0,
        );
        let right_rail = centered_cube(
            format!("bioburden_sampling_station_tool_lane_right_rail_{lane}"),
            TOOL_LANE_LENGTH,
            8.0,
            18.0,
        )
        .translate(
            TOOL_CENTER.0,
            TOOL_CENTER.1 + y + 24.0,
            DECK_Z + TOOL_PANEL_Z + 9.0,
        );
        rails = rails + left_rail + right_rail + tool_lane_clamps(lane) + sampling_windows(lane);
    }

    panel - lane_cuts + rails + tool_lane_end_stops() + tool_lane_identity_lands()
}

fn tool_lane_clamps(lane: usize) -> Part {
    let mut clamps = Part::empty(format!(
        "bioburden_sampling_station_tool_lane_{lane}_retention_clamps"
    ));
    let y = TOOL_CENTER.1 + tool_lane_y(lane);
    for clamp in 0..TOOL_CLAMPS_PER_LANE {
        let x = TOOL_CENTER.0 - 210.0 + clamp as f64 * 210.0;
        let bridge = centered_cube(
            format!("bioburden_sampling_station_tool_lane_{lane}_clamp_bridge_{clamp}"),
            50.0,
            72.0,
            10.0,
        )
        .translate(x, y, DECK_Z + TOOL_PANEL_Z + 34.0);
        let relief = centered_cube(
            format!("bioburden_sampling_station_tool_lane_{lane}_clamp_tool_relief_{clamp}"),
            32.0,
            42.0,
            12.0,
        )
        .translate(x, y, DECK_Z + TOOL_PANEL_Z + 34.0);
        clamps = clamps + (bridge - relief);
    }
    clamps
}

fn sampling_windows(lane: usize) -> Part {
    let mut windows = Part::empty(format!(
        "bioburden_sampling_station_tool_lane_{lane}_sampling_windows"
    ));
    let y = TOOL_CENTER.1 + tool_lane_y(lane);
    for window in 0..TOOL_SAMPLING_WINDOWS_PER_LANE {
        let x = TOOL_CENTER.0 - 110.0 + window as f64 * 220.0;
        windows = windows
            + centered_cube(
                format!("bioburden_sampling_station_tool_lane_{lane}_sample_window_{window}"),
                82.0,
                58.0,
                6.0,
            )
            .translate(x, y, DECK_Z + TOOL_PANEL_Z + 5.0);
    }
    windows
}

fn tool_lane_end_stops() -> Part {
    let left = centered_cube(
        "bioburden_sampling_station_tool_lane_left_end_stop",
        18.0,
        TOOL_PANEL_Y - 42.0,
        54.0,
    )
    .translate(
        TOOL_CENTER.0 - TOOL_LANE_LENGTH / 2.0 - 18.0,
        TOOL_CENTER.1,
        DECK_Z + TOOL_PANEL_Z + 27.0,
    );
    let right = centered_cube(
        "bioburden_sampling_station_tool_lane_right_end_stop",
        18.0,
        TOOL_PANEL_Y - 42.0,
        54.0,
    )
    .translate(
        TOOL_CENTER.0 + TOOL_LANE_LENGTH / 2.0 + 18.0,
        TOOL_CENTER.1,
        DECK_Z + TOOL_PANEL_Z + 27.0,
    );
    left + right
}

fn tool_lane_identity_lands() -> Part {
    let mut lands = Part::empty("bioburden_sampling_station_tool_lane_identity_lands");
    for lane in 0..TOOL_LANES {
        let y = TOOL_CENTER.1 + tool_lane_y(lane);
        lands = lands
            + centered_cube(
                format!("bioburden_sampling_station_before_sample_identity_land_{lane}"),
                78.0,
                20.0,
                8.0,
            )
            .translate(
                TOOL_CENTER.0 - TOOL_LANE_LENGTH / 2.0 + 82.0,
                y - 46.0,
                DECK_Z + TOOL_PANEL_Z + 8.0,
            )
            + centered_cube(
                format!("bioburden_sampling_station_after_sample_identity_land_{lane}"),
                78.0,
                20.0,
                8.0,
            )
            .translate(
                TOOL_CENTER.0 + TOOL_LANE_LENGTH / 2.0 - 82.0,
                y + 46.0,
                DECK_Z + TOOL_PANEL_Z + 8.0,
            );
    }
    lands
}

fn swab_contact_plate_holders() -> Part {
    let tray = centered_cube(
        "bioburden_sampling_station_swab_contact_plate_holder_block",
        HOLDER_PANEL_X,
        HOLDER_PANEL_Y,
        HOLDER_PANEL_Z,
    )
    .translate(
        HOLDER_CENTER.0,
        HOLDER_CENTER.1,
        DECK_Z + HOLDER_PANEL_Z / 2.0,
    );
    let raised_backstop = centered_cube(
        "bioburden_sampling_station_contact_plate_rear_backstop",
        HOLDER_PANEL_X - 42.0,
        16.0,
        42.0,
    )
    .translate(
        HOLDER_CENTER.0,
        HOLDER_CENTER.1 + HOLDER_PANEL_Y / 2.0 - 20.0,
        DECK_Z + HOLDER_PANEL_Z + 21.0,
    );

    tray - swab_holder_cutouts() - contact_plate_cutouts()
        + raised_backstop
        + swab_retainer_lips()
        + contact_plate_lid_parking_lands()
}

fn swab_holder_cutouts() -> Part {
    let mut cutouts = Part::empty("bioburden_sampling_station_swab_holder_cutouts");
    for i in 0..SWAB_WELLS {
        let (x, y) = swab_well_position(i);
        cutouts = cutouts
            + centered_cylinder(
                format!("bioburden_sampling_station_swab_tube_well_cut_{i}"),
                SWAB_WELL_D / 2.0,
                HOLDER_PANEL_Z + 4.0,
                24,
            )
            .translate(x, y, DECK_Z + HOLDER_PANEL_Z / 2.0);
    }
    cutouts
}

fn contact_plate_cutouts() -> Part {
    let mut cutouts = Part::empty("bioburden_sampling_station_contact_plate_cutouts");
    for i in 0..CONTACT_PLATE_WELLS {
        let (x, y) = contact_plate_position(i);
        let well = centered_cylinder(
            format!("bioburden_sampling_station_contact_plate_well_cut_{i}"),
            CONTACT_PLATE_D / 2.0,
            HOLDER_PANEL_Z + 4.0,
            42,
        )
        .translate(x, y, DECK_Z + HOLDER_PANEL_Z / 2.0);
        let finger_relief = centered_cube(
            format!("bioburden_sampling_station_contact_plate_finger_relief_{i}"),
            14.0,
            CONTACT_PLATE_D + 12.0,
            10.0,
        )
        .translate(x + CONTACT_PLATE_D / 2.0 + 2.0, y, DECK_Z + HOLDER_PANEL_Z);
        cutouts = cutouts + well + finger_relief;
    }
    cutouts
}

fn swab_retainer_lips() -> Part {
    let mut lips = Part::empty("bioburden_sampling_station_swab_retainer_lips");
    for row in 0..2 {
        let y = HOLDER_CENTER.1 - 88.0 + row as f64 * 38.0;
        lips = lips
            + centered_cube(
                format!("bioburden_sampling_station_swab_retainer_lip_{row}"),
                322.0,
                6.0,
                10.0,
            )
            .translate(
                HOLDER_CENTER.0 - 20.0,
                y + 18.0,
                DECK_Z + HOLDER_PANEL_Z + 5.0,
            );
    }
    lips
}

fn contact_plate_lid_parking_lands() -> Part {
    let mut lands = Part::empty("bioburden_sampling_station_contact_plate_lid_parking_lands");
    for i in 0..4 {
        let x = HOLDER_CENTER.0 - 156.0 + i as f64 * 104.0;
        lands = lands
            + centered_cube(
                format!("bioburden_sampling_station_contact_plate_lid_parking_land_{i}"),
                70.0,
                16.0,
                8.0,
            )
            .translate(x, HOLDER_CENTER.1 + 94.0, DECK_Z + HOLDER_PANEL_Z + 4.0);
    }
    lands
}

fn neutralizer_vial_rack() -> Part {
    let rack = centered_cube(
        "bioburden_sampling_station_neutralizer_vial_rack",
        VIAL_RACK_X,
        VIAL_RACK_Y,
        VIAL_RACK_Z,
    )
    .translate(
        VIAL_RACK_CENTER.0,
        VIAL_RACK_CENTER.1,
        DECK_Z + VIAL_RACK_Z / 2.0,
    );
    let rear_retainer = centered_cube(
        "bioburden_sampling_station_neutralizer_vial_rear_retainer",
        VIAL_RACK_X - 36.0,
        14.0,
        54.0,
    )
    .translate(
        VIAL_RACK_CENTER.0,
        VIAL_RACK_CENTER.1 + VIAL_RACK_Y / 2.0 - 18.0,
        DECK_Z + VIAL_RACK_Z + 27.0,
    );

    rack - neutralizer_vial_cutouts()
        + rear_retainer
        + vial_retention_rails()
        + neutralizer_lot_lands()
}

fn neutralizer_vial_cutouts() -> Part {
    let mut cutouts = Part::empty("bioburden_sampling_station_neutralizer_vial_cutouts");
    for i in 0..NEUTRALIZER_VIALS {
        let (x, y) = vial_position(i);
        let well = centered_cylinder(
            format!("bioburden_sampling_station_neutralizer_vial_well_{i}"),
            VIAL_D / 2.0,
            VIAL_RACK_Z + 4.0,
            28,
        )
        .translate(x, y, DECK_Z + VIAL_RACK_Z / 2.0);
        let key_flat = centered_cube(
            format!("bioburden_sampling_station_neutralizer_vial_key_flat_{i}"),
            VIAL_D + 8.0,
            4.0,
            VIAL_RACK_Z + 4.0,
        )
        .translate(x, y + VIAL_D / 2.0, DECK_Z + VIAL_RACK_Z / 2.0);
        cutouts = cutouts + well + key_flat;
    }
    cutouts
}

fn vial_retention_rails() -> Part {
    let mut rails = Part::empty("bioburden_sampling_station_neutralizer_vial_retention_rails");
    for row in 0..VIAL_RETENTION_RAILS {
        rails = rails
            + centered_cube(
                format!("bioburden_sampling_station_vial_retention_rail_{row}"),
                VIAL_RACK_X - 78.0,
                6.0,
                10.0,
            )
            .translate(
                VIAL_RACK_CENTER.0,
                VIAL_RACK_CENTER.1 - 65.0 + row as f64 * 42.0,
                DECK_Z + VIAL_RACK_Z + 5.0,
            );
    }
    rails
}

fn neutralizer_lot_lands() -> Part {
    let mut lands = Part::empty("bioburden_sampling_station_neutralizer_lot_lands");
    for i in 0..4 {
        lands = lands
            + centered_cube(
                format!("bioburden_sampling_station_neutralizer_lot_land_{i}"),
                62.0,
                18.0,
                8.0,
            )
            .translate(
                VIAL_RACK_CENTER.0 - 132.0 + i as f64 * 88.0,
                VIAL_RACK_CENTER.1 + VIAL_RACK_Y / 2.0 - 48.0,
                DECK_Z + VIAL_RACK_Z + 4.0,
            );
    }
    lands
}

fn before_after_evidence_camera_bridge() -> Part {
    let mut posts = Part::empty("bioburden_sampling_station_camera_bridge_posts");
    for (i, (x, y)) in camera_post_positions().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("bioburden_sampling_station_camera_bridge_post_{i}"),
                24.0,
                24.0,
                CAMERA_BRIDGE_Z,
            )
            .translate(
                CAMERA_CENTER.0 + *x,
                CAMERA_CENTER.1 + *y,
                DECK_Z + CAMERA_BRIDGE_Z / 2.0,
            );
    }

    let front_beam = centered_cube(
        "bioburden_sampling_station_camera_bridge_front_beam",
        CAMERA_BRIDGE_X,
        28.0,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1 - CAMERA_BRIDGE_Y / 2.0 + 14.0,
        DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        "bioburden_sampling_station_camera_bridge_rear_beam",
        CAMERA_BRIDGE_X,
        28.0,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 - 14.0,
        DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );
    let cross_beam = centered_cube(
        "bioburden_sampling_station_camera_bridge_center_cross_beam",
        36.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1,
        DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );

    posts + front_beam + rear_beam + cross_beam + camera_pods() + led_bars() + evidence_scale_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("bioburden_sampling_station_before_after_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = CAMERA_CENTER.0 - 1.5 * CAMERA_PITCH_X + i as f64 * CAMERA_PITCH_X;
        let y = if i < 2 {
            CAMERA_CENTER.1 - 146.0
        } else {
            CAMERA_CENTER.1 + 146.0
        };
        let body = centered_cube(
            format!("bioburden_sampling_station_evidence_camera_pod_{i}"),
            68.0,
            50.0,
            34.0,
        )
        .translate(x, y, DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 22.0);
        let lens = centered_cylinder(
            format!("bioburden_sampling_station_evidence_camera_lens_{i}"),
            14.0,
            18.0,
            32,
        )
        .translate(x, y, DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 44.0);
        pods = pods + body + lens;
    }
    pods
}

fn led_bars() -> Part {
    let mut bars = Part::empty("bioburden_sampling_station_evidence_led_bars");
    for i in 0..LED_BARS {
        let x = CAMERA_CENTER.0 - 258.0 + i as f64 * 172.0;
        bars =
            bars + centered_cube(
                format!("bioburden_sampling_station_evidence_led_bar_front_{i}"),
                90.0,
                10.0,
                12.0,
            )
            .translate(
                x,
                CAMERA_CENTER.1 - 212.0,
                DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 20.0,
            ) + centered_cube(
                format!("bioburden_sampling_station_evidence_led_bar_rear_{i}"),
                90.0,
                10.0,
                12.0,
            )
            .translate(
                x,
                CAMERA_CENTER.1 + 212.0,
                DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 20.0,
            );
    }
    bars
}

fn evidence_scale_bars() -> Part {
    let before = centered_cube(
        "bioburden_sampling_station_before_image_scale_bar",
        210.0,
        8.0,
        6.0,
    )
    .translate(
        TOOL_CENTER.0 - 160.0,
        TOOL_CENTER.1 - 212.0,
        DECK_Z + TOOL_PANEL_Z + 8.0,
    );
    let after = centered_cube(
        "bioburden_sampling_station_after_image_scale_bar",
        210.0,
        8.0,
        6.0,
    )
    .translate(
        TOOL_CENTER.0 + 160.0,
        TOOL_CENTER.1 + 212.0,
        DECK_Z + TOOL_PANEL_Z + 8.0,
    );
    before + after
}

fn sterile_cover_envelope() -> Part {
    let outer = centered_cube(
        "bioburden_sampling_station_sterile_cover_outer_envelope",
        COVER_X,
        COVER_Y,
        COVER_Z,
    )
    .translate(COVER_CENTER.0, COVER_CENTER.1, DECK_Z + COVER_Z / 2.0 + 4.0);
    let inner = centered_cube(
        "bioburden_sampling_station_sterile_cover_internal_clearance",
        COVER_X - 2.0 * COVER_WALL_T,
        COVER_Y - 2.0 * COVER_WALL_T,
        COVER_Z - COVER_WALL_T,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z / 2.0 - COVER_WALL_T / 2.0,
    );
    let front_transfer_slot = centered_cube(
        "bioburden_sampling_station_sterile_cover_front_transfer_slot",
        TRANSFER_SLOT_X,
        COVER_WALL_T + 8.0,
        TRANSFER_SLOT_Z,
    )
    .translate(
        COVER_CENTER.0 + 278.0,
        COVER_CENTER.1 - COVER_Y / 2.0,
        DECK_Z + 88.0,
    );
    let rear_media_slot = centered_cube(
        "bioburden_sampling_station_sterile_cover_rear_media_service_slot",
        260.0,
        COVER_WALL_T + 8.0,
        72.0,
    )
    .translate(
        HOLDER_CENTER.0 - 30.0,
        COVER_CENTER.1 + COVER_Y / 2.0,
        DECK_Z + 104.0,
    );

    outer - inner - front_transfer_slot - rear_media_slot
        + cover_gasket_path()
        + cover_latch_pads()
        + aseptic_sleeve_ports()
        + cover_filter_ports()
}

fn cover_gasket_path() -> Part {
    let front = centered_cube(
        "bioburden_sampling_station_sterile_cover_front_gasket_land",
        COVER_X,
        COVER_GASKET_W,
        10.0,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 - COVER_Y / 2.0,
        DECK_Z + COVER_GASKET_W / 2.0,
    );
    let rear = centered_cube(
        "bioburden_sampling_station_sterile_cover_rear_gasket_land",
        COVER_X,
        COVER_GASKET_W,
        10.0,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 + COVER_Y / 2.0,
        DECK_Z + COVER_GASKET_W / 2.0,
    );
    let left = centered_cube(
        "bioburden_sampling_station_sterile_cover_left_gasket_land",
        COVER_GASKET_W,
        COVER_Y,
        10.0,
    )
    .translate(
        COVER_CENTER.0 - COVER_X / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_GASKET_W / 2.0,
    );
    let right = centered_cube(
        "bioburden_sampling_station_sterile_cover_right_gasket_land",
        COVER_GASKET_W,
        COVER_Y,
        10.0,
    )
    .translate(
        COVER_CENTER.0 + COVER_X / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_GASKET_W / 2.0,
    );
    front + rear + left + right
}

fn cover_latch_pads() -> Part {
    let mut pads = Part::empty("bioburden_sampling_station_sterile_cover_latch_pads");
    for i in 0..COVER_LATCHES {
        let (x, y) = cover_latch_position(i);
        pads = pads
            + centered_cube(
                format!("bioburden_sampling_station_sterile_cover_latch_pad_{i}"),
                56.0,
                24.0,
                14.0,
            )
            .translate(x, y, DECK_Z + 18.0);
    }
    pads
}

fn aseptic_sleeve_ports() -> Part {
    let mut ports = Part::empty("bioburden_sampling_station_aseptic_sleeve_ports");
    for i in 0..2 {
        let x = TOOL_CENTER.0 - 150.0 + i as f64 * 300.0;
        let rim = centered_cylinder(
            format!("bioburden_sampling_station_aseptic_sleeve_port_rim_{i}"),
            54.0,
            16.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, COVER_CENTER.1 - COVER_Y / 2.0 - 7.0, DECK_Z + 128.0);
        let bore = centered_cylinder(
            format!("bioburden_sampling_station_aseptic_sleeve_port_bore_{i}"),
            42.0,
            18.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, COVER_CENTER.1 - COVER_Y / 2.0 - 7.0, DECK_Z + 128.0);
        ports = ports + (rim - bore);
    }
    ports
}

fn cover_filter_ports() -> Part {
    let mut ports = Part::empty("bioburden_sampling_station_sterile_cover_filter_ports");
    for i in 0..3 {
        ports = ports
            + centered_cube(
                format!("bioburden_sampling_station_sterile_cover_hepa_vhp_filter_land_{i}"),
                96.0,
                64.0,
                10.0,
            )
            .translate(
                COVER_CENTER.0 - 210.0 + i as f64 * 210.0,
                COVER_CENTER.1 + COVER_Y / 2.0 - 54.0,
                DECK_Z + COVER_Z + 12.0,
            );
    }
    ports
}

fn sample_custody_lane() -> Part {
    let lane = centered_cube(
        "bioburden_sampling_station_sample_custody_lane_panel",
        CUSTODY_LANE_X,
        CUSTODY_LANE_Y,
        CUSTODY_LANE_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        DECK_Z + CUSTODY_LANE_Z / 2.0,
    );
    let lane_dividers = custody_lane_dividers();
    let custody_wells = custody_sample_wells();
    let barcode_lands = custody_barcode_lands();
    let tamper_slots = tamper_seal_slots();

    lane - custody_wells + lane_dividers + barcode_lands + tamper_slots + custody_direction_arrow()
}

fn custody_sample_wells() -> Part {
    let mut wells = Part::empty("bioburden_sampling_station_sample_custody_wells");
    for i in 0..CUSTODY_POSITIONS {
        let x = custody_x(i);
        wells = wells
            + centered_cylinder(
                format!("bioburden_sampling_station_custody_sample_vial_well_{i}"),
                12.0,
                CUSTODY_LANE_Z + 4.0,
                28,
            )
            .translate(x, CUSTODY_CENTER.1 - 28.0, DECK_Z + CUSTODY_LANE_Z / 2.0);
    }
    wells
}

fn custody_lane_dividers() -> Part {
    let mut dividers = Part::empty("bioburden_sampling_station_sample_custody_lane_dividers");
    for i in 0..=CUSTODY_POSITIONS {
        dividers = dividers
            + centered_cube(
                format!("bioburden_sampling_station_custody_lane_divider_{i}"),
                4.0,
                CUSTODY_LANE_Y - 26.0,
                18.0,
            )
            .translate(
                CUSTODY_CENTER.0 - (CUSTODY_POSITIONS as f64 * CUSTODY_POSITION_PITCH_X) / 2.0
                    + i as f64 * CUSTODY_POSITION_PITCH_X,
                CUSTODY_CENTER.1,
                DECK_Z + CUSTODY_LANE_Z + 9.0,
            );
    }
    dividers
}

fn custody_barcode_lands() -> Part {
    let mut lands = Part::empty("bioburden_sampling_station_sample_custody_barcode_lands");
    for i in 0..CUSTODY_BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("bioburden_sampling_station_custody_barcode_land_{i}"),
                54.0,
                18.0,
                7.0,
            )
            .translate(
                custody_x(i),
                CUSTODY_CENTER.1 + 42.0,
                DECK_Z + CUSTODY_LANE_Z + 3.5,
            );
    }
    lands
}

fn tamper_seal_slots() -> Part {
    let mut slots = Part::empty("bioburden_sampling_station_tamper_evidence_slots");
    for i in 0..TAMPER_SEAL_SLOTS {
        slots = slots
            + centered_cube(
                format!("bioburden_sampling_station_tamper_evidence_slot_{i}"),
                120.0,
                8.0,
                12.0,
            )
            .translate(
                CUSTODY_CENTER.0 - 252.0 + i as f64 * 168.0,
                CUSTODY_CENTER.1 + CUSTODY_LANE_Y / 2.0 - 20.0,
                DECK_Z + CUSTODY_LANE_Z + 6.0,
            );
    }
    slots
}

fn custody_direction_arrow() -> Part {
    let shaft = centered_cube(
        "bioburden_sampling_station_custody_lane_direction_arrow_shaft",
        CUSTODY_LANE_X - 110.0,
        5.0,
        7.0,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1 - 60.0,
        DECK_Z + CUSTODY_LANE_Z + 3.5,
    );
    let head = centered_cube(
        "bioburden_sampling_station_custody_lane_direction_arrow_head",
        22.0,
        22.0,
        7.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        CUSTODY_CENTER.0 + CUSTODY_LANE_X / 2.0 - 52.0,
        CUSTODY_CENTER.1 - 60.0,
        DECK_Z + CUSTODY_LANE_Z + 3.5,
    );
    shaft + head
}

fn quarantine_bin() -> Part {
    let outer = centered_cube(
        "bioburden_sampling_station_quarantine_bin_outer",
        QUARANTINE_BIN_X,
        QUARANTINE_BIN_Y,
        QUARANTINE_BIN_Z,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        DECK_Z + QUARANTINE_BIN_Z / 2.0,
    );
    let cavity = centered_cube(
        "bioburden_sampling_station_quarantine_bin_cavity",
        QUARANTINE_BIN_X - 34.0,
        QUARANTINE_BIN_Y - 34.0,
        QUARANTINE_BIN_Z - 20.0,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        DECK_Z + QUARANTINE_BIN_Z / 2.0 + 10.0,
    );
    let lid = centered_cube(
        "bioburden_sampling_station_quarantine_bin_sliding_lid",
        QUARANTINE_BIN_X + 22.0,
        QUARANTINE_BIN_Y + 18.0,
        QUARANTINE_LID_Z,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        DECK_Z + QUARANTINE_BIN_Z + QUARANTINE_LID_Z / 2.0 + 6.0,
    );

    (outer - cavity)
        + lid
        + quarantine_sample_slot_rack()
        + quarantine_status_lands()
        + quarantine_interlock_tabs()
}

fn quarantine_sample_slot_rack() -> Part {
    let mut rack = Part::empty("bioburden_sampling_station_quarantine_sample_slot_rack");
    for i in 0..QUARANTINE_SAMPLE_SLOTS {
        rack = rack
            + centered_cube(
                format!("bioburden_sampling_station_quarantine_sample_slot_{i}"),
                36.0,
                112.0,
                16.0,
            )
            .translate(
                QUARANTINE_CENTER.0 - 90.0 + i as f64 * 36.0,
                QUARANTINE_CENTER.1,
                DECK_Z + 32.0,
            );
    }
    rack
}

fn quarantine_status_lands() -> Part {
    let hold = centered_cube(
        "bioburden_sampling_station_quarantine_hold_status_land",
        86.0,
        22.0,
        8.0,
    )
    .translate(
        QUARANTINE_CENTER.0 - 52.0,
        QUARANTINE_CENTER.1 - QUARANTINE_BIN_Y / 2.0 - 12.0,
        DECK_Z + 52.0,
    );
    let reject = centered_cube(
        "bioburden_sampling_station_quarantine_reject_status_land",
        86.0,
        22.0,
        8.0,
    )
    .translate(
        QUARANTINE_CENTER.0 + 52.0,
        QUARANTINE_CENTER.1 - QUARANTINE_BIN_Y / 2.0 - 12.0,
        DECK_Z + 52.0,
    );
    hold + reject
}

fn quarantine_interlock_tabs() -> Part {
    let left = centered_cube(
        "bioburden_sampling_station_quarantine_left_interlock_tab",
        42.0,
        16.0,
        22.0,
    )
    .translate(
        QUARANTINE_CENTER.0 - QUARANTINE_BIN_X / 2.0 + 48.0,
        QUARANTINE_CENTER.1 + QUARANTINE_BIN_Y / 2.0 + 4.0,
        DECK_Z + QUARANTINE_BIN_Z + 18.0,
    );
    let right = centered_cube(
        "bioburden_sampling_station_quarantine_right_interlock_tab",
        42.0,
        16.0,
        22.0,
    )
    .translate(
        QUARANTINE_CENTER.0 + QUARANTINE_BIN_X / 2.0 - 48.0,
        QUARANTINE_CENTER.1 + QUARANTINE_BIN_Y / 2.0 + 4.0,
        DECK_Z + QUARANTINE_BIN_Z + 18.0,
    );
    left + right
}

fn robot_grippable_handles() -> Part {
    let mut handles = Part::empty("bioburden_sampling_station_robot_grippable_handles");
    for i in 0..ROBOT_HANDLES {
        let (x, y, z, rotation) = robot_handle_pose(i);
        let handle = robot_handle(&format!("bioburden_sampling_station_robot_handle_{i}"));
        handles = handles + handle.rotate(0.0, 0.0, rotation).translate(x, y, z);
    }
    handles
}

fn robot_handle(name: &str) -> Part {
    let left_post = centered_cube(
        format!("{name}_left_standoff"),
        18.0,
        18.0,
        HANDLE_STANDOFF_Z,
    )
    .translate(-HANDLE_GRIP_X / 2.0, 0.0, HANDLE_STANDOFF_Z / 2.0);
    let right_post = centered_cube(
        format!("{name}_right_standoff"),
        18.0,
        18.0,
        HANDLE_STANDOFF_Z,
    )
    .translate(HANDLE_GRIP_X / 2.0, 0.0, HANDLE_STANDOFF_Z / 2.0);
    let grip = centered_cylinder(
        format!("{name}_round_robot_grip"),
        HANDLE_GRIP_D / 2.0,
        HANDLE_GRIP_X + 18.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, HANDLE_STANDOFF_Z);
    left_post + right_post + grip
}

fn fiducial_datum_targets() -> Part {
    let mut targets = Part::empty("bioburden_sampling_station_fiducial_targets");
    for i in 0..FIDUCIALS {
        let (x, y) = fiducial_position(i);
        targets = targets
            + fiducial_target(&format!("bioburden_sampling_station_fiducial_{i}")).translate(
                x,
                y,
                DECK_Z + 8.0,
            );
    }
    for i in 0..DATUM_PINS {
        let (x, y) = datum_pin_position(i);
        targets = targets
            + centered_cylinder(
                format!("bioburden_sampling_station_robot_datum_pin_{i}"),
                DATUM_PIN_D / 2.0,
                22.0,
                28,
            )
            .translate(x, y, DECK_Z + 11.0);
    }
    targets
}

fn fiducial_target(name: &str) -> Part {
    let disk = centered_cylinder(format!("{name}_disk"), FIDUCIAL_D / 2.0, 4.0, 42);
    let center = centered_cylinder(format!("{name}_center_dot_cut"), 4.0, 6.0, 24);
    let horizontal = centered_cube(format!("{name}_horizontal_bar"), FIDUCIAL_D, 4.0, 5.0);
    let vertical = centered_cube(format!("{name}_vertical_bar"), 4.0, FIDUCIAL_D, 5.0);
    (disk - center) + horizontal + vertical
}

fn service_keepouts() -> Part {
    let front_robot = centered_cube(
        "bioburden_sampling_station_front_robot_approach_keepout",
        DECK_X,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0),
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_media = centered_cube(
        "bioburden_sampling_station_rear_media_service_keepout",
        DECK_X - 180.0,
        REAR_MEDIA_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -20.0,
        DECK_Y / 2.0 + REAR_MEDIA_SERVICE_KEEP_OUT_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right_quarantine = centered_cube(
        "bioburden_sampling_station_right_quarantine_pull_keepout",
        RIGHT_QUARANTINE_SERVICE_KEEP_OUT_X,
        DECK_Y - 120.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_QUARANTINE_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top_lift = centered_cube(
        "bioburden_sampling_station_top_sterile_cover_lift_keepout",
        COVER_X,
        COVER_Y,
        TOP_COVER_LIFT_KEEP_OUT_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z + TOP_COVER_LIFT_KEEP_OUT_Z / 2.0,
    );

    front_robot + rear_media + right_quarantine + top_lift
}

fn mount_points() -> [(f64, f64); 10] {
    [
        (-620.0, -380.0),
        (-310.0, -380.0),
        (0.0, -380.0),
        (310.0, -380.0),
        (620.0, -380.0),
        (-620.0, 380.0),
        (-310.0, 380.0),
        (0.0, 380.0),
        (310.0, 380.0),
        (620.0, 380.0),
    ]
}

fn tool_lane_y(lane: usize) -> f64 {
    -((TOOL_LANES as f64 - 1.0) * TOOL_LANE_PITCH_Y) / 2.0 + lane as f64 * TOOL_LANE_PITCH_Y
}

fn swab_well_position(index: usize) -> (f64, f64) {
    let row = index / 6;
    let col = index % 6;
    (
        HOLDER_CENTER.0 - 140.0 + col as f64 * 56.0,
        HOLDER_CENTER.1 - 84.0 + row as f64 * 38.0,
    )
}

fn contact_plate_position(index: usize) -> (f64, f64) {
    let row = index / 4;
    let col = index % 4;
    (
        HOLDER_CENTER.0 - 150.0 + col as f64 * 100.0,
        HOLDER_CENTER.1 + 18.0 + row as f64 * 70.0,
    )
}

fn vial_position(index: usize) -> (f64, f64) {
    let row = index / VIAL_COLS;
    let col = index % VIAL_COLS;
    (
        VIAL_RACK_CENTER.0 - 1.5 * VIAL_PITCH + col as f64 * VIAL_PITCH,
        VIAL_RACK_CENTER.1 - 1.5 * VIAL_PITCH + row as f64 * VIAL_PITCH,
    )
}

fn camera_post_positions() -> [(f64, f64); 4] {
    [
        (-CAMERA_BRIDGE_X / 2.0 + 18.0, -CAMERA_BRIDGE_Y / 2.0 + 18.0),
        (CAMERA_BRIDGE_X / 2.0 - 18.0, -CAMERA_BRIDGE_Y / 2.0 + 18.0),
        (-CAMERA_BRIDGE_X / 2.0 + 18.0, CAMERA_BRIDGE_Y / 2.0 - 18.0),
        (CAMERA_BRIDGE_X / 2.0 - 18.0, CAMERA_BRIDGE_Y / 2.0 - 18.0),
    ]
}

fn cover_latch_position(index: usize) -> (f64, f64) {
    if index < 4 {
        (
            COVER_CENTER.0 - 360.0 + index as f64 * 240.0,
            COVER_CENTER.1 - COVER_Y / 2.0 - 8.0,
        )
    } else if index < 8 {
        (
            COVER_CENTER.0 - 360.0 + (index - 4) as f64 * 240.0,
            COVER_CENTER.1 + COVER_Y / 2.0 + 8.0,
        )
    } else if index == 8 {
        (COVER_CENTER.0 - COVER_X / 2.0 - 8.0, COVER_CENTER.1)
    } else {
        (COVER_CENTER.0 + COVER_X / 2.0 + 8.0, COVER_CENTER.1)
    }
}

fn custody_x(index: usize) -> f64 {
    CUSTODY_CENTER.0 - ((CUSTODY_POSITIONS as f64 - 1.0) * CUSTODY_POSITION_PITCH_X) / 2.0
        + index as f64 * CUSTODY_POSITION_PITCH_X
}

fn robot_handle_pose(index: usize) -> (f64, f64, f64, f64) {
    match index {
        0 => (
            COVER_CENTER.0 - COVER_X / 2.0 + 140.0,
            COVER_CENTER.1,
            DECK_Z + COVER_Z + 12.0,
            90.0,
        ),
        1 => (
            COVER_CENTER.0 + COVER_X / 2.0 - 140.0,
            COVER_CENTER.1,
            DECK_Z + COVER_Z + 12.0,
            90.0,
        ),
        2 => (
            COVER_CENTER.0,
            COVER_CENTER.1 - COVER_Y / 2.0 + 100.0,
            DECK_Z + COVER_Z + 12.0,
            0.0,
        ),
        3 => (
            COVER_CENTER.0,
            COVER_CENTER.1 + COVER_Y / 2.0 - 100.0,
            DECK_Z + COVER_Z + 12.0,
            0.0,
        ),
        4 => (
            QUARANTINE_CENTER.0,
            QUARANTINE_CENTER.1,
            DECK_Z + QUARANTINE_BIN_Z + QUARANTINE_LID_Z + 10.0,
            0.0,
        ),
        _ => (
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            DECK_Z + CUSTODY_LANE_Z + 10.0,
            0.0,
        ),
    }
}

fn fiducial_position(index: usize) -> (f64, f64) {
    let positions = [
        (-640.0, -390.0),
        (-320.0, -390.0),
        (0.0, -390.0),
        (320.0, -390.0),
        (640.0, -390.0),
        (-640.0, 390.0),
        (-320.0, 390.0),
        (0.0, 390.0),
        (320.0, 390.0),
        (640.0, 390.0),
    ];
    positions[index]
}

fn datum_pin_position(index: usize) -> (f64, f64) {
    let positions = [
        (TOOL_CENTER.0 - 286.0, TOOL_CENTER.1 - 226.0),
        (TOOL_CENTER.0 + 286.0, TOOL_CENTER.1 - 226.0),
        (TOOL_CENTER.0 - 286.0, TOOL_CENTER.1 + 226.0),
        (TOOL_CENTER.0 + 286.0, TOOL_CENTER.1 + 226.0),
        (VIAL_RACK_CENTER.0 - 156.0, VIAL_RACK_CENTER.1 - 88.0),
        (VIAL_RACK_CENTER.0 + 156.0, VIAL_RACK_CENTER.1 + 88.0),
    ];
    positions[index]
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected output count");
    assert!(fits_on_deck(TOOL_CENTER, TOOL_PANEL_X, TOOL_PANEL_Y, 24.0));
    assert!(fits_on_deck(
        HOLDER_CENTER,
        HOLDER_PANEL_X,
        HOLDER_PANEL_Y,
        24.0
    ));
    assert!(fits_on_deck(
        VIAL_RACK_CENTER,
        VIAL_RACK_X,
        VIAL_RACK_Y,
        24.0
    ));
    assert!(fits_on_deck(
        CUSTODY_CENTER,
        CUSTODY_LANE_X,
        CUSTODY_LANE_Y,
        24.0
    ));
    assert!(fits_on_deck(
        QUARANTINE_CENTER,
        QUARANTINE_BIN_X,
        QUARANTINE_BIN_Y,
        24.0
    ));
    assert!(fits_on_deck(COVER_CENTER, COVER_X, COVER_Y, 22.0));
    assert!(covers_rect(
        COVER_CENTER,
        COVER_X,
        COVER_Y,
        TOOL_CENTER,
        TOOL_PANEL_X,
        TOOL_PANEL_Y
    ));
    assert!(covers_rect(
        COVER_CENTER,
        COVER_X,
        COVER_Y,
        HOLDER_CENTER,
        HOLDER_PANEL_X,
        HOLDER_PANEL_Y
    ));
    assert!(covers_rect(
        COVER_CENTER,
        COVER_X,
        COVER_Y,
        VIAL_RACK_CENTER,
        VIAL_RACK_X,
        VIAL_RACK_Y
    ));
    assert!(CAMERA_BRIDGE_Z > COVER_Z - 40.0);
    assert!(CAMERA_PODS >= TOOL_SAMPLING_WINDOWS_PER_LANE * 2);
    assert!(NEUTRALIZER_VIALS >= TOOL_LANES * 4);
    assert!(
        !rects_overlap(
            rect(CUSTODY_CENTER, CUSTODY_LANE_X, CUSTODY_LANE_Y),
            rect(QUARANTINE_CENTER, QUARANTINE_BIN_X, QUARANTINE_BIN_Y)
        ),
        "custody lane and quarantine bin collide"
    );
    assert!(
        !rects_overlap(
            rect(VIAL_RACK_CENTER, VIAL_RACK_X, VIAL_RACK_Y),
            rect(QUARANTINE_CENTER, QUARANTINE_BIN_X, QUARANTINE_BIN_Y)
        ),
        "neutralizer rack and quarantine bin collide"
    );
    assert!(
        !rects_overlap(
            rect(HOLDER_CENTER, HOLDER_PANEL_X, HOLDER_PANEL_Y),
            rect(VIAL_RACK_CENTER, VIAL_RACK_X, VIAL_RACK_Y)
        ),
        "media holder and neutralizer vial rack collide"
    );
}

fn fits_on_deck(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + width / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= DECK_Y / 2.0 - margin
}

fn covers_rect(
    cover_center: (f64, f64),
    cover_width: f64,
    cover_depth: f64,
    target_center: (f64, f64),
    target_width: f64,
    target_depth: f64,
) -> bool {
    target_center.0 - target_width / 2.0 >= cover_center.0 - cover_width / 2.0 + COVER_WALL_T
        && target_center.0 + target_width / 2.0 <= cover_center.0 + cover_width / 2.0 - COVER_WALL_T
        && target_center.1 - target_depth / 2.0 >= cover_center.1 - cover_depth / 2.0 + COVER_WALL_T
        && target_center.1 + target_depth / 2.0 <= cover_center.1 + cover_depth / 2.0 - COVER_WALL_T
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output
                .starts_with("output/closed_aseptic_tool_surface_bioburden_sampling_station_"));
            assert!(output.ends_with(".stl"));
        }
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
    }

    #[test]
    fn station_layout_fits_on_the_deck_and_under_cover() {
        assert_layout();
    }

    #[test]
    fn tool_sampling_features_match_required_workflow() {
        assert_eq!(TOOL_LANES, 4);
        assert_eq!(TOOL_CLAMPS_PER_LANE, 3);
        assert_eq!(TOOL_SAMPLING_WINDOWS_PER_LANE, 2);
        assert!(TOOL_LANE_LENGTH >= 560.0);
        assert!(TOOL_LANE_CLEARANCE_D >= 24.0);
    }

    #[test]
    fn media_and_neutralizer_holders_are_sized_for_parallel_sampling() {
        assert_eq!(SWAB_WELLS, 12);
        assert_eq!(CONTACT_PLATE_WELLS, 8);
        assert_eq!(NEUTRALIZER_VIALS, 16);
        assert_eq!(VIAL_COLS, 4);
        assert!(NEUTRALIZER_VIALS >= TOOL_LANES * 4);
    }

    #[test]
    fn camera_evidence_and_custody_controls_are_explicit() {
        assert_eq!(CAMERA_PODS, 4);
        assert_eq!(LED_BARS, 4);
        assert_eq!(CUSTODY_POSITIONS, 8);
        assert_eq!(CUSTODY_BARCODE_LANDS, 8);
        assert_eq!(TAMPER_SEAL_SLOTS, 4);
        assert!(CAMERA_BRIDGE_Z >= 230.0);
    }

    #[test]
    fn quarantine_and_robot_controls_are_counted() {
        assert_eq!(QUARANTINE_SAMPLE_SLOTS, 6);
        assert_eq!(ROBOT_HANDLES, 6);
        assert_eq!(FIDUCIALS, 10);
        assert_eq!(DATUM_PINS, 6);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 440.0);
        assert!(TOP_COVER_LIFT_KEEP_OUT_Z >= COVER_Z);
    }
}
