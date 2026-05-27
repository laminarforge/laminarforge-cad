use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robot end-effector sterility parking station for the clean cabinet.
//
// Intent:
// - Give each robot end effector a covered, drainable parking nest inside the
//   closed clean cabinet instead of returning tools to an open bench.
// - Keep clean, used, and quarantine lanes physically separated with keyed
//   robot approach datums and explicit evidence lands for barcode/RFID scans.
// - Reserve sterile cap/cover holders, UV/VHP-compatible exposure windows,
//   coupon pockets, and service keepouts as validation placeholders.
//
// This is architecture packaging CAD. UV/VHP windows, coupon pockets, and
// evidence lands are placeholders for later materials selection, cycle
// validation, and robot teach-point qualification.

const OUTPUTS: &[&str] = &[
    "output/closed_robot_end_effector_sterility_parking_station_base_lane_tray.stl",
    "output/closed_robot_end_effector_sterility_parking_station_covered_tool_nests.stl",
    "output/closed_robot_end_effector_sterility_parking_station_sterile_cap_cover_holders.stl",
    "output/closed_robot_end_effector_sterility_parking_station_uv_vhp_exposure_windows.stl",
    "output/closed_robot_end_effector_sterility_parking_station_contact_coupon_pockets.stl",
    "output/closed_robot_end_effector_sterility_parking_station_barcode_rfid_evidence_lands.stl",
    "output/closed_robot_end_effector_sterility_parking_station_clean_used_quarantine_lanes.stl",
    "output/closed_robot_end_effector_sterility_parking_station_robot_alignment_datums.stl",
    "output/closed_robot_end_effector_sterility_parking_station_drip_vhp_capture_sump.stl",
    "output/closed_robot_end_effector_sterility_parking_station_service_keepouts.stl",
    "output/closed_robot_end_effector_sterility_parking_station_assembly.stl",
];

const STATION_X: f64 = 940.0;
const STATION_Y: f64 = 620.0;
const STATION_Z: f64 = 318.0;
const BASE_Z: f64 = 34.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 58.0;
const LANE_COUNT: usize = 3;
const TOOL_COUNT: usize = 4;
const CAP_COLUMNS: usize = 8;
const COVER_SLOTS: usize = 4;
const COUPON_ROWS: usize = 2;
const COUPON_COLUMNS: usize = 6;
const BARCODE_LANDS: usize = TOOL_COUNT * LANE_COUNT;
const RFID_LANDS: usize = TOOL_COUNT * LANE_COUNT;

const CLEAN_LANE_Y: f64 = 184.0;
const USED_LANE_Y: f64 = 0.0;
const QUARANTINE_LANE_Y: f64 = -184.0;
const LANE_WIDTH: f64 = 138.0;
const LANE_BARRIER_W: f64 = 16.0;
const LANE_BARRIER_Z: f64 = 148.0;
const LANE_AIR_GAP_MIN: f64 = 42.0;

const TOOL_NEST_Z: f64 = 88.0;
const TOOL_COVER_Z: f64 = 52.0;
const TOOL_PITCH_X: f64 = 210.0;
const TOOL_START_X: f64 = -315.0;
const USED_SADDLE_Z: f64 = 38.0;
const QUARANTINE_WELL_Z: f64 = 50.0;

const CAP_HOLDER_X: f64 = 330.0;
const CAP_HOLDER_Y: f64 = 76.0;
const CAP_HOLDER_Z: f64 = 42.0;
const CAP_WELL_D: f64 = 18.0;
const COVER_SLOT_X: f64 = 52.0;
const COVER_SLOT_Y: f64 = 18.0;
const COVER_SLOT_Z: f64 = 28.0;

const WINDOW_FRAME_X: f64 = 812.0;
const WINDOW_FRAME_Y: f64 = 18.0;
const WINDOW_FRAME_Z: f64 = 126.0;
const WINDOW_CLEAR_X: f64 = 178.0;
const WINDOW_CLEAR_Z: f64 = 78.0;
const VHP_NOZZLE_CLEARANCE: f64 = 86.0;

const COUPON_PANEL_X: f64 = 392.0;
const COUPON_PANEL_Y: f64 = 28.0;
const COUPON_PANEL_Z: f64 = 118.0;
const COUPON_SLOT_X: f64 = 42.0;
const COUPON_SLOT_Z: f64 = 24.0;
const COUPON_MIN_COUNT: usize = 12;

const BARCODE_X: f64 = 58.0;
const BARCODE_Y: f64 = 22.0;
const RFID_D: f64 = 20.0;
const DATUM_PIN_D: f64 = 8.0;
const FIDUCIAL_D: f64 = 16.0;
const ROBOT_APPROACH_Z_CLEARANCE: f64 = 252.0;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const SIDE_SERVICE_CLEARANCE: f64 = 160.0;

#[derive(Clone, Copy)]
struct ToolNest {
    slug: &'static str,
    x: f64,
    body_x: f64,
    body_y: f64,
    cavity_x: f64,
    cavity_y: f64,
    cavity_z: f64,
}

const TOOL_NESTS: [ToolNest; TOOL_COUNT] = [
    ToolNest {
        slug: "cassette_gripper",
        x: TOOL_START_X,
        body_x: 168.0,
        body_y: 120.0,
        cavity_x: 118.0,
        cavity_y: 78.0,
        cavity_z: 58.0,
    },
    ToolNest {
        slug: "scanner_camera",
        x: TOOL_START_X + TOOL_PITCH_X,
        body_x: 150.0,
        body_y: 108.0,
        cavity_x: 94.0,
        cavity_y: 66.0,
        cavity_z: 50.0,
    },
    ToolNest {
        slug: "connector_driver",
        x: TOOL_START_X + 2.0 * TOOL_PITCH_X,
        body_x: 158.0,
        body_y: 126.0,
        cavity_x: 104.0,
        cavity_y: 86.0,
        cavity_z: 62.0,
    },
    ToolNest {
        slug: "probe_swab_tool",
        x: TOOL_START_X + 3.0 * TOOL_PITCH_X,
        body_x: 136.0,
        body_y: 112.0,
        cavity_x: 62.0,
        cavity_y: 84.0,
        cavity_z: 60.0,
    },
];

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_sanity();

    let base = base_lane_tray();
    export(OUTPUTS[0], &base);

    let nests = covered_tool_nests();
    export(OUTPUTS[1], &nests);

    let cap_holders = sterile_cap_cover_holders();
    export(OUTPUTS[2], &cap_holders);

    let windows = uv_vhp_exposure_windows();
    export(OUTPUTS[3], &windows);

    let coupons = contact_coupon_pockets();
    export(OUTPUTS[4], &coupons);

    let evidence = barcode_rfid_evidence_lands();
    export(OUTPUTS[5], &evidence);

    let lanes = clean_used_quarantine_lanes();
    export(OUTPUTS[6], &lanes);

    let datums = robot_alignment_datums();
    export(OUTPUTS[7], &datums);

    let sump = drip_vhp_capture_sump();
    export(OUTPUTS[8], &sump);

    let keepouts = service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + nests
        + cap_holders
        + windows
        + coupons
        + evidence
        + lanes
        + datums
        + sump
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed robot end-effector sterility parking station:");
    println!(
        "  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm x {STATION_Z:.0}mm envelope"
    );
    println!(
        "  Tool parking:               {TOOL_COUNT} covered clean nests, used saddles, and quarantine wells"
    );
    println!("  Sterile cap/cover storage:  {CAP_COLUMNS} cap wells and {COVER_SLOTS} cover slots");
    println!(
        "  Validation controls:        {} contact coupon pockets, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands",
        coupon_count()
    );
    println!(
        "  Segregation:                clean/used/quarantine lanes with {:.0}mm minimum open air gap",
        lane_air_gap()
    );
    println!(
        "  Exposure placeholders:      UV/VHP window frames with {VHP_NOZZLE_CLEARANCE:.0}mm nozzle/flow clearance"
    );
    println!(
        "  Robot controls:             three-point datum pins, fiducials, rear hard-stop rail, {:.0}mm approach Z clearance",
        ROBOT_APPROACH_Z_CLEARANCE
    );
    println!(
        "  Service keepouts:           front {FRONT_SERVICE_CLEARANCE:.0}mm, side {SIDE_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_sanity() {
    assert_eq!(TOOL_NESTS.len(), TOOL_COUNT, "unexpected tool nest count");
    assert_eq!(
        LANE_COUNT, 3,
        "station must model clean, used, and quarantine lanes"
    );
    assert!(
        lane_air_gap() >= LANE_AIR_GAP_MIN,
        "clean/used/quarantine lane gap is too small"
    );
    assert!(
        coupon_count() >= COUPON_MIN_COUNT,
        "not enough coupon pockets for challenge/contact evidence"
    );
    assert!(
        covered_nest_span_x() < STATION_X - 2.0 * RIM_W,
        "covered nest bank exceeds tray inner width"
    );
    assert!(
        TOOL_NEST_Z + TOOL_COVER_Z + BASE_Z < ROBOT_APPROACH_Z_CLEARANCE,
        "covered nests intrude into robot approach clearance"
    );
    assert!(
        station_fits_closed_clean_cabinet(),
        "station exceeds closed clean cabinet deck allowance"
    );
}

fn base_lane_tray() -> Part {
    let deck = centered_cube(
        "closed_end_effector_parking_base_lane_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    deck - base_recesses() - deck_drain_slots() - base_mount_holes()
        + perimeter_rim()
        + lane_floor_markers()
        + rear_cabinet_dock_tongue()
}

fn base_recesses() -> Part {
    let central_recess = centered_cube(
        "closed_end_effector_parking_base_washdown_recess",
        STATION_X - 108.0,
        STATION_Y - 106.0,
        12.0,
    )
    .translate(0.0, 0.0, BASE_Z - 6.0);
    let front_gutter = centered_cube(
        "closed_end_effector_parking_front_drain_gutter",
        STATION_X - 154.0,
        18.0,
        20.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 54.0), BASE_Z - 10.0);

    central_recess + front_gutter
}

fn deck_drain_slots() -> Part {
    let mut slots = Part::empty("closed_end_effector_parking_deck_drain_slots");
    for (i, y) in [CLEAN_LANE_Y, USED_LANE_Y, QUARANTINE_LANE_Y]
        .iter()
        .enumerate()
    {
        slots = slots
            + centered_cube(
                format!("closed_end_effector_parking_lane_drain_slot_{i}"),
                STATION_X - 184.0,
                10.0,
                16.0,
            )
            .translate(0.0, *y - LANE_WIDTH / 2.0 + 18.0, BASE_Z - 7.0);
    }
    slots
}

fn base_mount_holes() -> Part {
    let mut holes = Part::empty("closed_end_effector_parking_base_mount_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 70.0), -(STATION_Y / 2.0 - 62.0)),
        (STATION_X / 2.0 - 70.0, -(STATION_Y / 2.0 - 62.0)),
        (-(STATION_X / 2.0 - 70.0), STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_end_effector_parking_mount_hole_{i}"),
                7.0 / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "closed_end_effector_parking_front_raised_washdown_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -(STATION_Y / 2.0 - RIM_W / 2.0), RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_end_effector_parking_rear_cabinet_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, RIM_Z / 2.0);
    let left = centered_cube(
        "closed_end_effector_parking_left_side_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, RIM_Z / 2.0);
    let right = centered_cube(
        "closed_end_effector_parking_right_side_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, RIM_Z / 2.0);

    front + rear + left + right
}

fn lane_floor_markers() -> Part {
    let mut markers = Part::empty("closed_end_effector_parking_lane_floor_markers");
    for (i, y) in [CLEAN_LANE_Y, USED_LANE_Y, QUARANTINE_LANE_Y]
        .iter()
        .enumerate()
    {
        markers = markers
            + centered_cube(
                format!("closed_end_effector_parking_lane_floor_marker_{i}"),
                STATION_X - 170.0,
                LANE_WIDTH - 36.0,
                4.0,
            )
            .translate(0.0, *y, BASE_Z + 2.0);
    }
    markers
}

fn rear_cabinet_dock_tongue() -> Part {
    let tongue = centered_cube(
        "closed_end_effector_parking_rear_cabinet_dock_tongue",
        430.0,
        36.0,
        32.0,
    )
    .translate(0.0, STATION_Y / 2.0 + 18.0, BASE_Z + 16.0);
    let key_slot = centered_cube(
        "closed_end_effector_parking_dock_tongue_key_slot",
        86.0,
        38.0,
        10.0,
    )
    .translate(0.0, STATION_Y / 2.0 + 18.0, BASE_Z + 26.0);

    tongue - key_slot
}

fn covered_tool_nests() -> Part {
    let mut nests = Part::empty("closed_end_effector_parking_covered_tool_nests");
    for nest in TOOL_NESTS {
        nests = nests
            + clean_covered_nest(nest).translate(nest.x, CLEAN_LANE_Y, BASE_Z)
            + used_return_saddle(nest).translate(nest.x, USED_LANE_Y, BASE_Z)
            + quarantine_well(nest).translate(nest.x, QUARANTINE_LANE_Y, BASE_Z);
    }
    nests
}

fn clean_covered_nest(nest: ToolNest) -> Part {
    let body = centered_cube(
        format!("closed_end_effector_{}_clean_nest_body", nest.slug),
        nest.body_x,
        nest.body_y,
        TOOL_NEST_Z,
    )
    .translate(0.0, 0.0, TOOL_NEST_Z / 2.0);
    let cavity = centered_cube(
        format!("closed_end_effector_{}_clean_tool_recess", nest.slug),
        nest.cavity_x,
        nest.cavity_y,
        nest.cavity_z + 8.0,
    )
    .translate(0.0, -4.0, TOOL_NEST_Z - nest.cavity_z / 2.0 + 4.0);
    let front_loading_slot = centered_cube(
        format!("closed_end_effector_{}_front_loading_slot", nest.slug),
        nest.cavity_x + 34.0,
        24.0,
        34.0,
    )
    .translate(0.0, -(nest.body_y / 2.0 - 14.0), TOOL_NEST_Z - 14.0);
    let drain = centered_cube(
        format!("closed_end_effector_{}_nest_drain_channel", nest.slug),
        nest.cavity_x * 0.62,
        9.0,
        18.0,
    )
    .translate(0.0, -(nest.body_y / 2.0 - 18.0), 15.0);

    let shell = body - cavity - front_loading_slot - drain;
    shell + nest_hard_stops(nest) + hinged_cover_placeholder(nest) + nest_shadow_fiducials(nest)
}

fn nest_hard_stops(nest: ToolNest) -> Part {
    let rear_stop = centered_cube(
        format!("closed_end_effector_{}_rear_hard_stop", nest.slug),
        nest.cavity_x + 22.0,
        12.0,
        30.0,
    )
    .translate(0.0, nest.body_y / 2.0 - 16.0, TOOL_NEST_Z - 15.0);
    let left_rail = centered_cube(
        format!("closed_end_effector_{}_left_locating_rail", nest.slug),
        12.0,
        nest.body_y - 38.0,
        26.0,
    )
    .translate(-(nest.cavity_x / 2.0 + 14.0), -2.0, TOOL_NEST_Z - 13.0);
    let right_rail = centered_cube(
        format!("closed_end_effector_{}_right_locating_rail", nest.slug),
        12.0,
        nest.body_y - 38.0,
        26.0,
    )
    .translate(nest.cavity_x / 2.0 + 14.0, -2.0, TOOL_NEST_Z - 13.0);

    rear_stop + left_rail + right_rail
}

fn hinged_cover_placeholder(nest: ToolNest) -> Part {
    let cover = centered_cube(
        format!("closed_end_effector_{}_sterile_cover_canopy", nest.slug),
        nest.body_x + 24.0,
        nest.body_y + 18.0,
        TOOL_COVER_Z,
    )
    .translate(0.0, 2.0, TOOL_NEST_Z + TOOL_COVER_Z / 2.0 + 10.0);
    let window_cut = centered_cube(
        format!("closed_end_effector_{}_uv_window_opening", nest.slug),
        nest.body_x - 40.0,
        nest.body_y - 42.0,
        TOOL_COVER_Z + 4.0,
    )
    .translate(0.0, 2.0, TOOL_NEST_Z + TOOL_COVER_Z / 2.0 + 10.0);
    let hinge_bar = centered_cylinder(
        format!("closed_end_effector_{}_rear_cover_hinge_bar", nest.slug),
        8.0 / 2.0,
        nest.body_x + 38.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, nest.body_y / 2.0 + 16.0, TOOL_NEST_Z + 16.0);

    (cover - window_cut) + hinge_bar
}

fn nest_shadow_fiducials(nest: ToolNest) -> Part {
    let mut fiducials = Part::empty(format!(
        "closed_end_effector_{}_nest_shadow_fiducials",
        nest.slug
    ));
    for (i, (x, y)) in [
        (-(nest.body_x / 2.0 - 24.0), -(nest.body_y / 2.0 - 18.0)),
        (nest.body_x / 2.0 - 24.0, -(nest.body_y / 2.0 - 18.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_end_effector_{}_fiducial_boss_{i}", nest.slug),
                6.0,
                5.0,
                24,
            )
            .translate(*x, *y, TOOL_NEST_Z + TOOL_COVER_Z + 16.0);
    }
    fiducials
}

fn used_return_saddle(nest: ToolNest) -> Part {
    let saddle = centered_cube(
        format!("closed_end_effector_{}_used_return_saddle", nest.slug),
        nest.body_x - 16.0,
        nest.body_y - 34.0,
        USED_SADDLE_Z,
    )
    .translate(0.0, 0.0, USED_SADDLE_Z / 2.0);
    let trough = centered_cylinder(
        format!("closed_end_effector_{}_used_saddle_open_trough", nest.slug),
        (nest.body_y - 56.0) / 2.0,
        nest.body_x - 36.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, USED_SADDLE_Z);
    let warning_tab = centered_cube(
        format!("closed_end_effector_{}_used_lane_status_tab", nest.slug),
        48.0,
        10.0,
        18.0,
    )
    .translate(0.0, -(nest.body_y / 2.0 - 14.0), USED_SADDLE_Z + 8.0);

    saddle - trough + warning_tab
}

fn quarantine_well(nest: ToolNest) -> Part {
    let well = centered_cube(
        format!("closed_end_effector_{}_quarantine_well_body", nest.slug),
        nest.body_x - 10.0,
        nest.body_y - 10.0,
        QUARANTINE_WELL_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_WELL_Z / 2.0);
    let pocket = centered_cube(
        format!("closed_end_effector_{}_quarantine_deep_recess", nest.slug),
        nest.body_x - 44.0,
        nest.body_y - 44.0,
        QUARANTINE_WELL_Z - 10.0,
    )
    .translate(0.0, 0.0, QUARANTINE_WELL_Z / 2.0 + 10.0);
    let red_tag_land = centered_cube(
        format!(
            "closed_end_effector_{}_quarantine_evidence_tag_land",
            nest.slug
        ),
        58.0,
        14.0,
        18.0,
    )
    .translate(0.0, -(nest.body_y / 2.0 - 18.0), QUARANTINE_WELL_Z + 8.0);

    well - pocket + red_tag_land
}

fn sterile_cap_cover_holders() -> Part {
    let body = centered_cube(
        "closed_end_effector_sterile_cap_cover_holder_body",
        CAP_HOLDER_X,
        CAP_HOLDER_Y,
        CAP_HOLDER_Z,
    )
    .translate(
        -250.0,
        -(STATION_Y / 2.0 - 74.0),
        BASE_Z + CAP_HOLDER_Z / 2.0,
    );

    let mut cap_wells = Part::empty("closed_end_effector_sterile_cap_well_cuts");
    for i in 0..CAP_COLUMNS {
        let x = -250.0 + lane_position(i, CAP_COLUMNS, 34.0);
        cap_wells = cap_wells
            + centered_cylinder(
                format!("closed_end_effector_sterile_cap_well_cut_{i}"),
                CAP_WELL_D / 2.0,
                CAP_HOLDER_Z + 8.0,
                32,
            )
            .translate(
                x,
                -(STATION_Y / 2.0 - 74.0),
                BASE_Z + CAP_HOLDER_Z / 2.0 + 5.0,
            );
    }

    let cover_rack = centered_cube(
        "closed_end_effector_sterile_cover_slot_rack",
        250.0,
        84.0,
        66.0,
    )
    .translate(235.0, -(STATION_Y / 2.0 - 76.0), BASE_Z + 33.0);
    let mut cover_slots = Part::empty("closed_end_effector_sterile_cover_slot_cuts");
    for i in 0..COVER_SLOTS {
        cover_slots = cover_slots
            + centered_cube(
                format!("closed_end_effector_sterile_cover_slot_cut_{i}"),
                COVER_SLOT_X,
                COVER_SLOT_Y,
                COVER_SLOT_Z,
            )
            .translate(
                235.0 + lane_position(i, COVER_SLOTS, 52.0),
                -(STATION_Y / 2.0 - 76.0),
                BASE_Z + 46.0,
            );
    }

    (body - cap_wells) + (cover_rack - cover_slots) + cap_holder_status_tabs()
}

fn cap_holder_status_tabs() -> Part {
    let clean_tab = centered_cube(
        "closed_end_effector_cap_holder_clean_status_land",
        82.0,
        14.0,
        20.0,
    )
    .translate(-394.0, -(STATION_Y / 2.0 - 28.0), BASE_Z + 44.0);
    let cover_tab = centered_cube(
        "closed_end_effector_cover_holder_lot_status_land",
        92.0,
        14.0,
        20.0,
    )
    .translate(376.0, -(STATION_Y / 2.0 - 30.0), BASE_Z + 52.0);

    clean_tab + cover_tab
}

fn uv_vhp_exposure_windows() -> Part {
    let mut frames = Part::empty("closed_end_effector_uv_vhp_exposure_window_frames");
    for (i, y) in [CLEAN_LANE_Y, USED_LANE_Y, QUARANTINE_LANE_Y]
        .iter()
        .enumerate()
    {
        let frame = centered_cube(
            format!("closed_end_effector_lane_{i}_uv_vhp_window_frame"),
            WINDOW_FRAME_X,
            WINDOW_FRAME_Y,
            WINDOW_FRAME_Z,
        )
        .translate(0.0, *y + LANE_WIDTH / 2.0 - 18.0, BASE_Z + 164.0);

        let mut cuts = Part::empty(format!("closed_end_effector_lane_{i}_uv_window_cuts"));
        for tool in TOOL_NESTS {
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_end_effector_{}_lane_{i}_uv_window_clear_opening",
                        tool.slug
                    ),
                    WINDOW_CLEAR_X,
                    WINDOW_FRAME_Y + 4.0,
                    WINDOW_CLEAR_Z,
                )
                .translate(tool.x, *y + LANE_WIDTH / 2.0 - 18.0, BASE_Z + 164.0);
        }

        frames = frames + (frame - cuts);
    }

    frames + vhp_nozzle_placeholder_rails()
}

fn vhp_nozzle_placeholder_rails() -> Part {
    let rear_rail = centered_cube(
        "closed_end_effector_rear_vhp_nozzle_clearance_rail",
        STATION_X - 146.0,
        12.0,
        16.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 46.0,
        BASE_Z + STATION_Z - VHP_NOZZLE_CLEARANCE,
    );
    let side_rail = centered_cube(
        "closed_end_effector_side_uv_light_service_clearance_rail",
        12.0,
        STATION_Y - 120.0,
        16.0,
    )
    .translate(
        STATION_X / 2.0 - 52.0,
        0.0,
        BASE_Z + STATION_Z - VHP_NOZZLE_CLEARANCE,
    );

    rear_rail + side_rail
}

fn contact_coupon_pockets() -> Part {
    let panel = centered_cube(
        "closed_end_effector_contact_coupon_pocket_panel",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    )
    .translate(-(STATION_X / 2.0 - 70.0), 0.0, BASE_Z + 104.0);

    let mut slots = Part::empty("closed_end_effector_contact_coupon_slot_cuts");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLUMNS {
            let i = row * COUPON_COLUMNS + col;
            slots = slots
                + centered_cube(
                    format!("closed_end_effector_contact_coupon_slot_cut_{i}"),
                    COUPON_SLOT_X,
                    COUPON_PANEL_Y + 6.0,
                    COUPON_SLOT_Z,
                )
                .translate(
                    -(STATION_X / 2.0 - 70.0),
                    lane_position(col, COUPON_COLUMNS, 44.0),
                    BASE_Z + 82.0 + row as f64 * 38.0,
                );
        }
    }

    let witness_strip = centered_cube(
        "closed_end_effector_coupon_witness_strip_label_land",
        16.0,
        STATION_Y - 160.0,
        34.0,
    )
    .translate(-(STATION_X / 2.0 - 46.0), 0.0, BASE_Z + 52.0);

    (panel - slots) + witness_strip
}

fn barcode_rfid_evidence_lands() -> Part {
    let mut lands = Part::empty("closed_end_effector_barcode_rfid_evidence_lands");
    for (lane_i, y) in [CLEAN_LANE_Y, USED_LANE_Y, QUARANTINE_LANE_Y]
        .iter()
        .enumerate()
    {
        for (tool_i, tool) in TOOL_NESTS.iter().enumerate() {
            let barcode = centered_cube(
                format!("closed_end_effector_lane_{lane_i}_tool_{tool_i}_barcode_land"),
                BARCODE_X,
                BARCODE_Y,
                5.0,
            )
            .translate(tool.x - 28.0, *y - LANE_WIDTH / 2.0 + 18.0, BASE_Z + 6.0);
            let rfid = centered_cylinder(
                format!("closed_end_effector_lane_{lane_i}_tool_{tool_i}_rfid_puck_land"),
                RFID_D / 2.0,
                6.0,
                32,
            )
            .translate(tool.x + 48.0, *y - LANE_WIDTH / 2.0 + 18.0, BASE_Z + 6.0);
            lands = lands + barcode + rfid;
        }
    }
    lands + chain_of_custody_land_bank()
}

fn chain_of_custody_land_bank() -> Part {
    let lot_land = centered_cube(
        "closed_end_effector_station_lot_chain_of_custody_land",
        220.0,
        28.0,
        6.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 30.0), BASE_Z + 6.0);
    let quarantine_release_land = centered_cube(
        "closed_end_effector_quarantine_release_evidence_land",
        210.0,
        28.0,
        6.0,
    )
    .translate(
        0.0,
        QUARANTINE_LANE_Y - LANE_WIDTH / 2.0 + 18.0,
        BASE_Z + 8.0,
    );

    lot_land + quarantine_release_land
}

fn clean_used_quarantine_lanes() -> Part {
    let clean_used_barrier = lane_barrier(
        "closed_end_effector_clean_used_lane_barrier",
        (CLEAN_LANE_Y + USED_LANE_Y) / 2.0,
    );
    let used_quarantine_barrier = lane_barrier(
        "closed_end_effector_used_quarantine_lane_barrier",
        (USED_LANE_Y + QUARANTINE_LANE_Y) / 2.0,
    );
    let clean_entry_gate = centered_cube(
        "closed_end_effector_clean_lane_robot_entry_gate",
        96.0,
        12.0,
        76.0,
    )
    .translate(-(STATION_X / 2.0 - 110.0), CLEAN_LANE_Y, BASE_Z + 38.0);
    let quarantine_lock_tab = centered_cube(
        "closed_end_effector_quarantine_lane_lockout_tab",
        108.0,
        12.0,
        84.0,
    )
    .translate(STATION_X / 2.0 - 108.0, QUARANTINE_LANE_Y, BASE_Z + 42.0);

    clean_used_barrier + used_quarantine_barrier + clean_entry_gate + quarantine_lock_tab
}

fn lane_barrier(name: &str, y: f64) -> Part {
    let wall = centered_cube(name, STATION_X - 128.0, LANE_BARRIER_W, LANE_BARRIER_Z).translate(
        0.0,
        y,
        BASE_Z + LANE_BARRIER_Z / 2.0,
    );
    let robot_crossing_notch = centered_cube(
        format!("{name}_robot_wrist_crossing_notch"),
        122.0,
        LANE_BARRIER_W + 4.0,
        76.0,
    )
    .translate(0.0, y, BASE_Z + LANE_BARRIER_Z - 30.0);
    let bottom_wash_gap = centered_cube(
        format!("{name}_bottom_wash_gap"),
        STATION_X - 188.0,
        LANE_BARRIER_W + 4.0,
        18.0,
    )
    .translate(0.0, y, BASE_Z + 9.0);

    wall - robot_crossing_notch - bottom_wash_gap
}

fn robot_alignment_datums() -> Part {
    let rear_rail = centered_cube(
        "closed_end_effector_robot_rear_hard_stop_datum_rail",
        STATION_X - 160.0,
        18.0,
        46.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 78.0, BASE_Z + 23.0);

    let mut datums = Part::empty("closed_end_effector_robot_alignment_datum_features");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 92.0), STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
        (0.0, -(STATION_Y / 2.0 - 92.0)),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("closed_end_effector_robot_alignment_datum_pad_{i}"),
            19.0,
            12.0,
            40,
        )
        .translate(*x, *y, BASE_Z + 6.0);
        let pin = centered_cylinder(
            format!("closed_end_effector_robot_alignment_pin_clearance_{i}"),
            DATUM_PIN_D / 2.0,
            16.0,
            28,
        )
        .translate(*x, *y, BASE_Z + 6.0);
        datums = datums + (pad - pin);
    }

    rear_rail + datums + robot_fiducial_grid()
}

fn robot_fiducial_grid() -> Part {
    let mut grid = Part::empty("closed_end_effector_robot_camera_fiducial_grid");
    for (i, (x, y)) in [
        (TOOL_START_X - 68.0, CLEAN_LANE_Y + 72.0),
        (
            TOOL_START_X + 3.0 * TOOL_PITCH_X + 68.0,
            CLEAN_LANE_Y + 72.0,
        ),
        (TOOL_START_X - 68.0, QUARANTINE_LANE_Y - 72.0),
        (
            TOOL_START_X + 3.0 * TOOL_PITCH_X + 68.0,
            QUARANTINE_LANE_Y - 72.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let outer = centered_cylinder(
            format!("closed_end_effector_robot_fiducial_outer_{i}"),
            FIDUCIAL_D / 2.0,
            4.0,
            32,
        )
        .translate(*x, *y, BASE_Z + 8.0);
        let center = centered_cylinder(
            format!("closed_end_effector_robot_fiducial_center_{i}"),
            3.0,
            5.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 8.0);
        grid = grid + (outer - center);
    }
    grid
}

fn drip_vhp_capture_sump() -> Part {
    let sump = centered_cube(
        "closed_end_effector_drip_vhp_capture_secondary_sump",
        STATION_X - 190.0,
        76.0,
        28.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 108.0), BASE_Z + 14.0);
    let drain_port = centered_cylinder(
        "closed_end_effector_drip_vhp_capture_drain_port",
        18.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -(STATION_Y / 2.0 - 72.0),
        BASE_Z + 14.0,
    );
    let witness_pocket = centered_cube(
        "closed_end_effector_vhp_condensate_witness_coupon_pocket",
        126.0,
        22.0,
        18.0,
    )
    .translate(
        -(STATION_X / 2.0 - 138.0),
        -(STATION_Y / 2.0 - 72.0),
        BASE_Z + 38.0,
    );

    sump - drain_port + witness_pocket
}

fn service_keepouts() -> Part {
    let front_pull = centered_cube(
        "closed_end_effector_front_tool_pull_service_keepout",
        STATION_X - 120.0,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE),
        BASE_Z + 22.0,
    );
    let left_clearance = centered_cube(
        "closed_end_effector_left_side_service_keepout",
        18.0,
        STATION_Y - 96.0,
        22.0,
    )
    .translate(
        -(STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE),
        0.0,
        BASE_Z + 22.0,
    );
    let robot_wrist_envelope = centered_cube(
        "closed_end_effector_robot_wrist_approach_envelope",
        STATION_X - 176.0,
        STATION_Y - 144.0,
        16.0,
    )
    .translate(0.0, 0.0, ROBOT_APPROACH_Z_CLEARANCE);

    front_pull + left_clearance + robot_wrist_envelope
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn coupon_count() -> usize {
    COUPON_ROWS * COUPON_COLUMNS
}

fn covered_nest_span_x() -> f64 {
    let first = TOOL_NESTS.first().unwrap();
    let last = TOOL_NESTS.last().unwrap();
    (last.x + last.body_x / 2.0) - (first.x - first.body_x / 2.0)
}

fn lane_air_gap() -> f64 {
    (CLEAN_LANE_Y - USED_LANE_Y).abs() - LANE_WIDTH
}

fn station_fits_closed_clean_cabinet() -> bool {
    const CLOSED_CABINET_DECK_X: f64 = 1220.0;
    const CLOSED_CABINET_DECK_Y: f64 = 820.0;

    STATION_X <= CLOSED_CABINET_DECK_X && STATION_Y <= CLOSED_CABINET_DECK_Y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_required_lane_segregation() {
        assert_eq!(LANE_COUNT, 3);
        assert!(lane_air_gap() >= LANE_AIR_GAP_MIN);
    }

    #[test]
    fn covers_fit_robot_approach_clearance() {
        assert!(TOOL_NEST_Z + TOOL_COVER_Z + BASE_Z < ROBOT_APPROACH_Z_CLEARANCE);
    }

    #[test]
    fn evidence_features_cover_every_tool_lane_position() {
        assert_eq!(BARCODE_LANDS, TOOL_COUNT * LANE_COUNT);
        assert_eq!(RFID_LANDS, TOOL_COUNT * LANE_COUNT);
        assert!(coupon_count() >= COUPON_MIN_COUNT);
    }

    #[test]
    fn nest_bank_stays_inside_base_rim() {
        assert!(covered_nest_span_x() < STATION_X - 2.0 * RIM_W);
    }
}
