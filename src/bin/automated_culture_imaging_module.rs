use std::fs;

use laminarforge_cad::{
    GT2_BELT_WIDTH, REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT, VSLOT_2040_H,
    VSLOT_2040_W,
};
use vcad::{centered_cube, centered_cylinder, Part};

// Enclosed imaging and inspection module for closed cell-culture monitoring.
//
// Intent:
// - Keep the culture cassette inside a dark, clean, gasketed inspection volume.
// - Preserve the closed isolator/cassette architecture: datum nest, service bulkhead,
//   filtered purge regions, sealed cable transit, and transfer-panel geometry.
// - Reserve an XY gantry camera path with a focus Z stage, objective turret/digital
//   microscope bay, transmitted illumination, epi illumination, calibration target,
//   and serviceable clean/dark enclosure panels.
//
// This is an architecture CAD bin for envelope, interface, and clearance planning.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const NEST_X: f64 = CASSETTE_X + 160.0;
const NEST_Y: f64 = CASSETTE_Y + 150.0;
const TRAY_Z: f64 = 26.0;
const RAIL_Z: f64 = 28.0;
const DATUM_RAIL_W: f64 = 18.0;

const ENC_X: f64 = 1120.0;
const ENC_Y: f64 = 820.0;
const ENC_Z: f64 = 540.0;
const WALL: f64 = 28.0;
const FLOOR_Z: f64 = 32.0;
const CEILING_Z: f64 = 42.0;
const INNER_X: f64 = ENC_X - WALL * 2.0;
const INNER_Y: f64 = ENC_Y - WALL * 2.0;
const INNER_Z: f64 = ENC_Z - FLOOR_Z - CEILING_Z;

const NEST_BASE_Z: f64 = FLOOR_Z + 8.0;
const CASSETTE_DATUM_Z: f64 = NEST_BASE_Z + TRAY_Z + RAIL_Z;
const CASSETTE_TOP_Z: f64 = CASSETTE_DATUM_Z + REVC_TOTAL_HEIGHT;

const X_TRAVEL: f64 = 780.0;
const Y_TRAVEL: f64 = 640.0;
const X_BEAM_LEN: f64 = X_TRAVEL + 120.0;
const Y_RAIL_LEN: f64 = Y_TRAVEL + 70.0;
const Y_RAIL_SPACING: f64 = 880.0;
const GANTRY_BEAM_Z: f64 = 426.0;

const FOCUS_TRAVEL_Z: f64 = 75.0;
const OBJECTIVE_TIP_Z: f64 = CASSETTE_TOP_Z + 66.0;
const OBJECTIVE_TURRET_Z: f64 = OBJECTIVE_TIP_Z + 39.0;
const OPTICS_BODY_Z: f64 = OBJECTIVE_TURRET_Z + 54.0;

const CAL_TARGET_X: f64 = 118.0;
const CAL_TARGET_Y: f64 = 92.0;
const CAL_TARGET_Z: f64 = 5.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    assert_fit_clearances();

    let enclosure = enclosure_shell();
    export(
        "output/automated_culture_imaging_module_enclosure.stl",
        &enclosure,
    );

    let nest = cassette_datum_nest();
    export(
        "output/automated_culture_imaging_module_datum_nest.stl",
        &nest,
    );

    let gantry = xy_gantry();
    export(
        "output/automated_culture_imaging_module_motion_gantry.stl",
        &gantry,
    );

    let optics = optics_carriage();
    export(
        "output/automated_culture_imaging_module_optics_carriage.stl",
        &optics,
    );

    let illumination = illumination_regions();
    export(
        "output/automated_culture_imaging_module_illumination.stl",
        &illumination,
    );

    let cable_chain = cable_chain();
    export(
        "output/automated_culture_imaging_module_cable_chain.stl",
        &cable_chain,
    );

    let service = service_panels();
    export(
        "output/automated_culture_imaging_module_service_panels.stl",
        &service,
    );

    let calibration = calibration_target();
    export(
        "output/automated_culture_imaging_module_calibration_target.stl",
        &calibration,
    );

    let assembly = enclosure
        + nest
        + gantry
        + optics
        + illumination
        + cable_chain
        + service
        + calibration
        + cassette_keepout();
    export(
        "output/automated_culture_imaging_module_assembly.stl",
        &assembly,
    );

    println!();
    println!("Automated culture imaging module:");
    println!("  Enclosure envelope:       {ENC_X:.0}mm x {ENC_Y:.0}mm x {ENC_Z:.0}mm");
    println!("  Dark/clean inner volume:  {INNER_X:.0}mm x {INNER_Y:.0}mm x {INNER_Z:.0}mm");
    println!("  Cassette datum nest:      {NEST_X:.0}mm x {NEST_Y:.0}mm");
    println!("  4x5 Rev C cassette span:  {CASSETTE_X:.0}mm x {CASSETTE_Y:.0}mm");
    println!("  XY travel placeholder:    {X_TRAVEL:.0}mm x {Y_TRAVEL:.0}mm");
    println!("  Focus Z travel:           {FOCUS_TRAVEL_Z:.0}mm");
    println!(
        "  Objective working gap:    {:.0}mm from cassette top to objective tip",
        objective_working_gap()
    );
    println!("  Illumination:             transmitted panel plus epi/ring/darkfield regions");
    println!(
        "  Closed-boundary features: gasket transfer frame, cable gland wall, purge filters, service panels"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_fit_clearances() {
    assert!(
        NEST_X < INNER_X - 80.0,
        "cassette nest does not leave side service margin"
    );
    assert!(
        NEST_Y < INNER_Y - 70.0,
        "cassette nest does not leave front/rear service margin"
    );
    assert!(
        X_BEAM_LEN / 2.0 + 35.0 < INNER_X / 2.0,
        "X gantry beam exceeds dark enclosure width"
    );
    assert!(
        Y_RAIL_LEN / 2.0 + 20.0 < INNER_Y / 2.0,
        "Y gantry rails exceed dark enclosure depth"
    );
    assert!(
        GANTRY_BEAM_Z + VSLOT_2040_H / 2.0 + 22.0 < ENC_Z - CEILING_Z,
        "gantry rail stack collides with enclosure ceiling"
    );
    assert!(
        objective_working_gap() >= 60.0,
        "objective tip too close to cassette top"
    );
    assert!(
        OBJECTIVE_TURRET_Z + 90.0 < GANTRY_BEAM_Z - 22.0,
        "optics carriage collides with gantry beam envelope"
    );
}

fn objective_working_gap() -> f64 {
    OBJECTIVE_TIP_Z - CASSETTE_TOP_Z
}

fn enclosure_shell() -> Part {
    let outer = centered_cube("imaging_module_outer_dark_clean_shell", ENC_X, ENC_Y, ENC_Z)
        .translate(0.0, 0.0, ENC_Z / 2.0);
    let inner = centered_cube(
        "imaging_module_inner_dark_chamber",
        INNER_X,
        INNER_Y,
        INNER_Z,
    )
    .translate(0.0, 0.0, FLOOR_Z + INNER_Z / 2.0);

    let front_transfer_cut = centered_cube(
        "imaging_module_front_transfer_cut",
        NEST_X + 80.0,
        WALL + 8.0,
        235.0,
    )
    .translate(0.0, -ENC_Y / 2.0, NEST_BASE_Z + 132.0);
    let rear_service_cut = centered_cube(
        "imaging_module_rear_service_bulkhead_cut",
        520.0,
        WALL + 8.0,
        280.0,
    )
    .translate(0.0, ENC_Y / 2.0, 215.0);
    let right_panel_cut = centered_cube(
        "imaging_module_right_service_panel_cut",
        WALL + 8.0,
        360.0,
        315.0,
    )
    .translate(ENC_X / 2.0, 80.0, 245.0);
    let top_lid_service_cut = centered_cube(
        "imaging_module_top_lid_service_cut",
        720.0,
        510.0,
        CEILING_Z + 4.0,
    )
    .translate(0.0, 0.0, ENC_Z - CEILING_Z / 2.0);

    let shell = outer
        - inner
        - front_transfer_cut
        - rear_service_cut
        - right_panel_cut
        - top_lid_service_cut;

    shell
        + gasket_frame_on_y(
            "imaging_module_front_transfer_gasket_frame",
            NEST_X + 150.0,
            285.0,
            22.0,
            18.0,
        )
        .translate(0.0, -ENC_Y / 2.0 - 10.0, NEST_BASE_Z + 132.0)
        + gasket_frame_on_y(
            "imaging_module_rear_service_gasket_frame",
            600.0,
            320.0,
            18.0,
            18.0,
        )
        .translate(0.0, ENC_Y / 2.0 + 10.0, 215.0)
        + top_lid_frame()
        + dark_baffle_ribs()
        + floor_drain_and_curb()
}

fn top_lid_frame() -> Part {
    let frame = rect_frame_xy(
        "imaging_module_lift_off_top_lid_frame",
        760.0,
        550.0,
        20.0,
        16.0,
    )
    .translate(0.0, 0.0, ENC_Z - 12.0);
    let handle_a = centered_cube("imaging_module_top_lid_handle_a", 130.0, 18.0, 22.0).translate(
        -180.0,
        -235.0,
        ENC_Z + 10.0,
    );
    let handle_b = centered_cube("imaging_module_top_lid_handle_b", 130.0, 18.0, 22.0).translate(
        180.0,
        -235.0,
        ENC_Z + 10.0,
    );
    let light_trap = centered_cube(
        "imaging_module_top_lid_light_trap_labyrinth",
        720.0,
        18.0,
        28.0,
    )
    .translate(0.0, 235.0, ENC_Z - 16.0);
    frame + handle_a + handle_b + light_trap
}

fn dark_baffle_ribs() -> Part {
    let mut ribs = Part::empty("imaging_module_internal_dark_baffle_ribs");
    for (i, y) in [-315.0, -245.0, 245.0, 315.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("imaging_module_floor_wall_baffle_{i}"),
                INNER_X - 80.0,
                12.0,
                48.0,
            )
            .translate(0.0, *y, FLOOR_Z + 24.0);
    }
    for (i, x) in [-505.0, 505.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("imaging_module_side_light_trap_{i}"),
                16.0,
                INNER_Y - 130.0,
                64.0,
            )
            .translate(*x, 0.0, 320.0);
    }
    ribs
}

fn floor_drain_and_curb() -> Part {
    let curb = rect_frame_xy(
        "imaging_module_containment_floor_curb",
        INNER_X - 95.0,
        INNER_Y - 90.0,
        16.0,
        18.0,
    )
    .translate(0.0, 0.0, FLOOR_Z + 10.0);
    let drain_sump = centered_cube("imaging_module_floor_drain_sump_land", 120.0, 58.0, 8.0)
        .translate(INNER_X / 2.0 - 130.0, -INNER_Y / 2.0 + 82.0, FLOOR_Z + 4.0);
    let drain_stub = centered_cylinder("imaging_module_floor_drain_stub", 16.0 / 2.0, 38.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(INNER_X / 2.0 - 130.0, -ENC_Y / 2.0 - 3.0, FLOOR_Z + 5.0);
    curb + drain_sump + drain_stub
}

fn cassette_datum_nest() -> Part {
    let tray_outer = centered_cube(
        "imaging_module_nest_leak_tray_outer",
        NEST_X,
        NEST_Y,
        TRAY_Z,
    )
    .translate(0.0, 0.0, NEST_BASE_Z + TRAY_Z / 2.0);
    let tray_basin = centered_cube(
        "imaging_module_nest_recessed_basin",
        CASSETTE_X + 72.0,
        CASSETTE_Y + 62.0,
        TRAY_Z + 2.0,
    )
    .translate(0.0, 0.0, NEST_BASE_Z + TRAY_Z / 2.0 + 6.0);
    let transmitted_window = centered_cube(
        "imaging_module_nest_transmitted_light_window",
        CASSETTE_X - 48.0,
        CASSETTE_Y - 48.0,
        TRAY_Z + 3.0,
    )
    .translate(0.0, 0.0, NEST_BASE_Z + TRAY_Z / 2.0);

    let left_tube_clearance = centered_cube(
        "imaging_module_nest_left_harness_clearance",
        34.0,
        NEST_Y + 2.0,
        TRAY_Z + 3.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 45.0), 0.0, NEST_BASE_Z + TRAY_Z / 2.0);
    let right_tube_clearance = centered_cube(
        "imaging_module_nest_right_harness_clearance",
        34.0,
        NEST_Y + 2.0,
        TRAY_Z + 3.0,
    )
    .translate(CASSETTE_X / 2.0 + 45.0, 0.0, NEST_BASE_Z + TRAY_Z / 2.0);
    let rear_harness_clearance = centered_cube(
        "imaging_module_nest_rear_harness_clearance",
        CASSETTE_X + 110.0,
        44.0,
        TRAY_Z + 3.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 52.0, NEST_BASE_Z + TRAY_Z / 2.0);

    let tray = tray_outer
        - tray_basin
        - transmitted_window
        - left_tube_clearance
        - right_tube_clearance
        - rear_harness_clearance
        - nest_mount_holes();

    tray + datum_rails_and_latches() + kinematic_datum_posts()
}

fn datum_rails_and_latches() -> Part {
    let z = CASSETTE_DATUM_Z - RAIL_Z / 2.0;
    let back_stop = centered_cube(
        "imaging_module_nest_back_y_hard_datum",
        CASSETTE_X + 42.0,
        DATUM_RAIL_W,
        RAIL_Z,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 13.0, z);
    let left_stop = centered_cube(
        "imaging_module_nest_left_x_hard_datum",
        DATUM_RAIL_W,
        CASSETTE_Y + 42.0,
        RAIL_Z,
    )
    .translate(-(CASSETTE_X / 2.0 + 13.0), 0.0, z);
    let right_spring_rail = centered_cube(
        "imaging_module_nest_right_compliance_rail",
        DATUM_RAIL_W,
        CASSETTE_Y + 42.0,
        RAIL_Z * 0.72,
    )
    .translate(CASSETTE_X / 2.0 + 13.0, 0.0, z - RAIL_Z * 0.14);
    let front_low_lip = centered_cube(
        "imaging_module_nest_front_low_light_stop_lip",
        CASSETTE_X + 42.0,
        12.0,
        RAIL_Z * 0.52,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 10.0), z - RAIL_Z * 0.24);

    let mut latches = Part::empty("imaging_module_nest_swing_latches");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 50.0), -(CASSETTE_Y / 2.0 - 42.0)),
        (CASSETTE_X / 2.0 - 50.0, -(CASSETTE_Y / 2.0 - 42.0)),
        (-(CASSETTE_X / 2.0 - 50.0), CASSETTE_Y / 2.0 - 42.0),
        (CASSETTE_X / 2.0 - 50.0, CASSETTE_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        let block = centered_cube(
            format!("imaging_module_nest_latch_block_{i}"),
            52.0,
            30.0,
            22.0,
        )
        .translate(*x, *y, CASSETTE_DATUM_Z + 10.0);
        let cam = centered_cylinder(
            format!("imaging_module_nest_latch_cam_bore_{i}"),
            8.0 / 2.0,
            56.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, *y, CASSETTE_DATUM_Z + 10.0);
        latches = latches + (block - cam);
    }

    back_stop + left_stop + right_spring_rail + front_low_lip + latches
}

fn kinematic_datum_posts() -> Part {
    let mut posts = Part::empty("imaging_module_nest_kinematic_datum_posts");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 34.0), CASSETTE_Y / 2.0 - 34.0),
        (CASSETTE_X / 2.0 - 34.0, CASSETTE_Y / 2.0 - 34.0),
        (-(CASSETTE_X / 2.0 - 34.0), -(CASSETTE_Y / 2.0 - 34.0)),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("imaging_module_nest_datum_boss_{i}"),
            12.0,
            10.0,
            36,
        )
        .translate(*x, *y, CASSETTE_DATUM_Z + 5.0);
        let bore = centered_cylinder(
            format!("imaging_module_nest_datum_pin_bore_{i}"),
            3.0 / 2.0,
            12.0,
            20,
        )
        .translate(*x, *y, CASSETTE_DATUM_Z + 5.0);
        posts = posts + (boss - bore);
    }
    posts + nest_fiducials()
}

fn nest_fiducials() -> Part {
    let mut targets = Part::empty("imaging_module_nest_fiducials");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 45.0), NEST_Y / 2.0 - 45.0),
        (NEST_X / 2.0 - 45.0, NEST_Y / 2.0 - 45.0),
        (-(NEST_X / 2.0 - 45.0), -(NEST_Y / 2.0 - 45.0)),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("imaging_module_nest_fiducial_{i}")).translate(
                *x,
                *y,
                CASSETTE_DATUM_Z + 16.0,
            );
    }
    targets
}

fn nest_mount_holes() -> Part {
    let mut holes = Part::empty("imaging_module_nest_mount_holes");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 36.0), -(NEST_Y / 2.0 - 36.0)),
        (NEST_X / 2.0 - 36.0, -(NEST_Y / 2.0 - 36.0)),
        (-(NEST_X / 2.0 - 36.0), NEST_Y / 2.0 - 36.0),
        (NEST_X / 2.0 - 36.0, NEST_Y / 2.0 - 36.0),
        (0.0, -(NEST_Y / 2.0 - 36.0)),
        (0.0, NEST_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("imaging_module_nest_m6_mount_{i}"),
                6.6 / 2.0,
                TRAY_Z + 4.0,
                24,
            )
            .translate(*x, *y, NEST_BASE_Z + TRAY_Z / 2.0);
    }
    holes
}

fn xy_gantry() -> Part {
    let rail_x = Y_RAIL_SPACING / 2.0;
    let mut gantry = Part::empty("imaging_module_xy_gantry");

    for (i, sx) in [-1.0f64, 1.0].iter().enumerate() {
        gantry = gantry
            + y_axis_rail(&format!("imaging_module_y_axis_{i}")).translate(
                sx * rail_x,
                0.0,
                GANTRY_BEAM_Z,
            )
            + y_axis_motor_pod(&format!("imaging_module_y_motor_{i}")).translate(
                sx * rail_x,
                -Y_RAIL_LEN / 2.0 - 32.0,
                GANTRY_BEAM_Z,
            )
            + y_axis_idler(&format!("imaging_module_y_idler_{i}")).translate(
                sx * rail_x,
                Y_RAIL_LEN / 2.0 + 22.0,
                GANTRY_BEAM_Z,
            );
    }

    gantry
        + x_axis_bridge().translate(0.0, 0.0, GANTRY_BEAM_Z - 18.0)
        + x_carriage_plate().translate(0.0, 0.0, GANTRY_BEAM_Z - 48.0)
        + gantry_limit_targets()
}

fn y_axis_rail(name: &str) -> Part {
    let extrusion = centered_cube(
        format!("{name}_2040_extrusion"),
        VSLOT_2040_W,
        Y_RAIL_LEN,
        VSLOT_2040_H,
    );
    let linear_rail = centered_cube(format!("{name}_mgn12_rail"), 12.0, Y_RAIL_LEN - 36.0, 8.0)
        .translate(0.0, 0.0, VSLOT_2040_H / 2.0 + 4.0);
    let belt = centered_cube(
        format!("{name}_gt2_belt_placeholder"),
        GT2_BELT_WIDTH,
        Y_RAIL_LEN - 70.0,
        4.0,
    )
    .translate(0.0, 0.0, -VSLOT_2040_H / 2.0 - 4.0);
    extrusion + linear_rail + belt
}

fn y_axis_motor_pod(name: &str) -> Part {
    let body = centered_cube(format!("{name}_sealed_stepper_pod"), 74.0, 56.0, 64.0);
    let gland = centered_cylinder(format!("{name}_shaft_gland"), 18.0 / 2.0, 60.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 25.0, 0.0);
    let service_cap =
        centered_cube(format!("{name}_service_cap"), 58.0, 4.0, 48.0).translate(0.0, -30.0, 0.0);
    body + gland + service_cap
}

fn y_axis_idler(name: &str) -> Part {
    let bracket = centered_cube(format!("{name}_idler_bracket"), 54.0, 28.0, 50.0);
    let pulley = centered_cylinder(format!("{name}_idler_pulley"), 17.0 / 2.0, 10.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 0.0);
    bracket + pulley
}

fn x_axis_bridge() -> Part {
    let extrusion = centered_cube(
        "imaging_module_x_bridge_2040_extrusion",
        X_BEAM_LEN,
        VSLOT_2040_W,
        VSLOT_2040_H,
    );
    let rail = centered_cube(
        "imaging_module_x_bridge_mgn12_rail",
        X_BEAM_LEN - 70.0,
        12.0,
        8.0,
    )
    .translate(0.0, 0.0, VSLOT_2040_H / 2.0 + 4.0);
    let belt = centered_cube(
        "imaging_module_x_bridge_gt2_belt_placeholder",
        X_BEAM_LEN - 95.0,
        GT2_BELT_WIDTH,
        4.0,
    )
    .translate(0.0, 0.0, -VSLOT_2040_H / 2.0 - 4.0);
    let left_end = centered_cube("imaging_module_x_bridge_left_end_plate", 8.0, 62.0, 54.0)
        .translate(-X_BEAM_LEN / 2.0 - 4.0, 0.0, 0.0);
    let right_end = centered_cube("imaging_module_x_bridge_right_end_plate", 8.0, 62.0, 54.0)
        .translate(X_BEAM_LEN / 2.0 + 4.0, 0.0, 0.0);
    extrusion + rail + belt + left_end + right_end
}

fn x_carriage_plate() -> Part {
    let plate = centered_cube("imaging_module_x_carriage_plate", 126.0, 94.0, 14.0);
    let drop_bore = centered_cylinder(
        "imaging_module_x_carriage_z_stage_bore",
        24.0 / 2.0,
        16.0,
        32,
    );
    let belt_clamp = centered_cube(
        "imaging_module_x_carriage_belt_clamp_slot",
        44.0,
        10.0,
        16.0,
    )
    .translate(0.0, 42.0, 0.0);
    let mut holes = Part::empty("imaging_module_x_carriage_m3_holes");
    for (i, (x, y)) in [(-38.0, -26.0), (38.0, -26.0), (-38.0, 26.0), (38.0, 26.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("imaging_module_x_carriage_m3_{i}"),
                3.3 / 2.0,
                16.0,
                20,
            )
            .translate(*x, *y, 0.0);
    }
    plate - drop_bore - belt_clamp - holes
}

fn gantry_limit_targets() -> Part {
    let mut targets = Part::empty("imaging_module_gantry_limit_targets");
    for (i, (x, y)) in [
        (-(X_TRAVEL / 2.0), -(Y_TRAVEL / 2.0)),
        (X_TRAVEL / 2.0, -(Y_TRAVEL / 2.0)),
        (-(X_TRAVEL / 2.0), Y_TRAVEL / 2.0),
        (X_TRAVEL / 2.0, Y_TRAVEL / 2.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + centered_cube(format!("imaging_module_xy_limit_flag_{i}"), 26.0, 8.0, 24.0)
                .translate(*x, *y, GANTRY_BEAM_Z - 78.0);
    }
    targets
}

fn optics_carriage() -> Part {
    focus_z_stage()
        + digital_microscope_bay()
        + objective_turret()
        + objective_barrels()
        + focus_bellows()
}

fn focus_z_stage() -> Part {
    let back_plate = centered_cube("imaging_module_focus_z_backplate", 118.0, 16.0, 172.0)
        .translate(0.0, 58.0, OPTICS_BODY_Z - 8.0);
    let slide = centered_cube("imaging_module_focus_z_moving_slide", 82.0, 18.0, 118.0).translate(
        0.0,
        44.0,
        OPTICS_BODY_Z - 18.0,
    );
    let lead_screw = centered_cylinder("imaging_module_focus_z_leadscrew", 5.0 / 2.0, 176.0, 24)
        .translate(0.0, 31.0, OPTICS_BODY_Z - 8.0);

    let mut rails = Part::empty("imaging_module_focus_z_linear_rails");
    for (i, x) in [-32.0, 32.0].iter().enumerate() {
        rails = rails
            + centered_cube(format!("imaging_module_focus_z_rail_{i}"), 10.0, 8.0, 160.0)
                .translate(*x, 30.0, OPTICS_BODY_Z - 8.0);
    }

    let motor = centered_cube(
        "imaging_module_focus_z_stepper_placeholder",
        48.0,
        42.0,
        42.0,
    )
    .translate(0.0, 68.0, OPTICS_BODY_Z + 90.0);
    let limit_flag = centered_cube("imaging_module_focus_z_limit_flag", 38.0, 4.0, 18.0).translate(
        52.0,
        29.0,
        OPTICS_BODY_Z + FOCUS_TRAVEL_Z / 2.0,
    );

    back_plate + slide + lead_screw + rails + motor + limit_flag
}

fn digital_microscope_bay() -> Part {
    let bay = centered_cube("imaging_module_digital_microscope_bay", 96.0, 88.0, 72.0).translate(
        0.0,
        0.0,
        OPTICS_BODY_Z,
    );
    let sensor_pocket = centered_cube("imaging_module_camera_sensor_pocket", 58.0, 56.0, 28.0)
        .translate(0.0, -8.0, OPTICS_BODY_Z + 10.0);
    let usb_cable_exit = centered_cylinder("imaging_module_usb3_cable_exit", 16.0 / 2.0, 92.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 35.0, OPTICS_BODY_Z + 20.0);
    let top_heat_sink = centered_cube("imaging_module_camera_heat_spreader", 84.0, 70.0, 10.0)
        .translate(0.0, 0.0, OPTICS_BODY_Z + 46.0);
    bay - sensor_pocket - usb_cable_exit + top_heat_sink
}

fn objective_turret() -> Part {
    let turret = centered_cylinder("imaging_module_objective_turret_disk", 96.0 / 2.0, 12.0, 64)
        .translate(0.0, 0.0, OBJECTIVE_TURRET_Z);
    let bearing = centered_cylinder(
        "imaging_module_objective_turret_bearing_bore",
        24.0 / 2.0,
        14.0,
        48,
    )
    .translate(0.0, 0.0, OBJECTIVE_TURRET_Z);

    let mut objective_holes = Part::empty("imaging_module_objective_turret_holes");
    for i in 0..4 {
        let theta = i as f64 * 90.0_f64.to_radians();
        objective_holes = objective_holes
            + centered_cylinder(
                format!("imaging_module_objective_turret_port_{i}"),
                18.0 / 2.0,
                14.0,
                36,
            )
            .translate(31.0 * theta.cos(), 31.0 * theta.sin(), OBJECTIVE_TURRET_Z);
    }

    turret - bearing - objective_holes
}

fn objective_barrels() -> Part {
    let mut barrels = Part::empty("imaging_module_objective_barrels");
    for (i, (r, h)) in [(8.0, 52.0), (7.0, 48.0), (6.5, 42.0), (7.5, 50.0)]
        .iter()
        .enumerate()
    {
        let theta = i as f64 * 90.0_f64.to_radians();
        let x = 31.0 * theta.cos();
        let y = 31.0 * theta.sin();
        barrels = barrels
            + centered_cylinder(format!("imaging_module_objective_barrel_{i}"), *r, *h, 40)
                .translate(x, y, OBJECTIVE_TIP_Z + h / 2.0);
    }
    barrels
}

fn focus_bellows() -> Part {
    let mut bellows = Part::empty("imaging_module_focus_bellows");
    for i in 0..7 {
        bellows = bellows
            + centered_cube(
                format!("imaging_module_focus_bellows_fold_{i}"),
                76.0 - i as f64 * 2.0,
                62.0 - i as f64 * 1.5,
                4.0,
            )
            .translate(0.0, 0.0, OBJECTIVE_TURRET_Z + 12.0 + i as f64 * 7.0);
    }
    bellows
}

fn illumination_regions() -> Part {
    transmitted_illumination() + epi_illumination() + darkfield_side_bars()
}

fn transmitted_illumination() -> Part {
    let tray = centered_cube(
        "imaging_module_transmitted_led_panel",
        CASSETTE_X - 38.0,
        CASSETTE_Y - 38.0,
        16.0,
    )
    .translate(0.0, 0.0, NEST_BASE_Z + 7.0);
    let diffuser = centered_cube(
        "imaging_module_transmitted_diffuser_region",
        CASSETTE_X - 62.0,
        CASSETTE_Y - 62.0,
        3.0,
    )
    .translate(0.0, 0.0, NEST_BASE_Z + 20.0);

    let mut led_rows = Part::empty("imaging_module_transmitted_led_rows");
    for (i, y) in [-185.0, -95.0, -5.0, 85.0, 175.0].iter().enumerate() {
        led_rows = led_rows
            + centered_cube(
                format!("imaging_module_transmitted_led_strip_{i}"),
                CASSETTE_X - 120.0,
                8.0,
                5.0,
            )
            .translate(0.0, *y, NEST_BASE_Z + 27.0);
    }

    tray + diffuser + led_rows
}

fn epi_illumination() -> Part {
    let ring_outer = centered_cylinder("imaging_module_epi_ring_outer", 62.0 / 2.0, 8.0, 64)
        .translate(0.0, 0.0, OBJECTIVE_TIP_Z + 34.0);
    let ring_inner = centered_cylinder(
        "imaging_module_epi_ring_clear_aperture",
        28.0 / 2.0,
        10.0,
        64,
    )
    .translate(0.0, 0.0, OBJECTIVE_TIP_Z + 34.0);

    let mut emitters = Part::empty("imaging_module_epi_led_emitters");
    for i in 0..8 {
        let theta = i as f64 * 45.0_f64.to_radians();
        emitters = emitters
            + centered_cylinder(
                format!("imaging_module_epi_led_lenslet_{i}"),
                5.0 / 2.0,
                4.0,
                20,
            )
            .translate(
                24.0 * theta.cos(),
                24.0 * theta.sin(),
                OBJECTIVE_TIP_Z + 28.0,
            );
    }

    (ring_outer - ring_inner) + emitters
}

fn darkfield_side_bars() -> Part {
    let left = centered_cube(
        "imaging_module_darkfield_left_bar",
        12.0,
        CASSETTE_Y - 80.0,
        18.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 34.0), 0.0, CASSETTE_TOP_Z + 36.0);
    let right = centered_cube(
        "imaging_module_darkfield_right_bar",
        12.0,
        CASSETTE_Y - 80.0,
        18.0,
    )
    .translate(CASSETTE_X / 2.0 + 34.0, 0.0, CASSETTE_TOP_Z + 36.0);
    let front = centered_cube(
        "imaging_module_darkfield_front_bar",
        CASSETTE_X - 90.0,
        12.0,
        18.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 34.0), CASSETTE_TOP_Z + 36.0);
    left + right + front
}

fn cable_chain() -> Part {
    let mut chain = Part::empty("imaging_module_cable_chain");
    let x = -(Y_RAIL_SPACING / 2.0 - 72.0);
    let z = GANTRY_BEAM_Z + 32.0;

    for i in 0..18 {
        chain = chain
            + cable_chain_link(&format!("imaging_module_horizontal_chain_link_{i}")).translate(
                x,
                -300.0 + i as f64 * 31.0,
                z,
            );
    }

    for i in 0..8 {
        chain = chain
            + cable_chain_link(&format!("imaging_module_vertical_drop_chain_link_{i}"))
                .rotate(0.0, 90.0, 0.0)
                .translate(
                    -70.0 + i as f64 * 16.0,
                    -8.0,
                    GANTRY_BEAM_Z - 35.0 - i as f64 * 18.0,
                );
    }

    let fixed_anchor = centered_cube(
        "imaging_module_chain_fixed_bulkhead_anchor",
        82.0,
        36.0,
        54.0,
    )
    .translate(x, -330.0, z);
    let moving_anchor = centered_cube(
        "imaging_module_chain_moving_carriage_anchor",
        68.0,
        30.0,
        44.0,
    )
    .translate(-4.0, -8.0, GANTRY_BEAM_Z - 70.0);

    chain + fixed_anchor + moving_anchor
}

fn cable_chain_link(name: &str) -> Part {
    let outer = centered_cube(format!("{name}_outer"), 28.0, 18.0, 16.0);
    let inner = centered_cube(format!("{name}_inner_opening"), 18.0, 20.0, 8.0);
    outer - inner
}

fn service_panels() -> Part {
    let front_door = service_door_panel(
        "imaging_module_front_cassette_transfer_panel",
        NEST_X + 130.0,
        18.0,
        285.0,
        2,
    )
    .translate(0.0, -ENC_Y / 2.0 - 22.0, NEST_BASE_Z + 132.0);

    let rear_bulkhead = rear_service_bulkhead().translate(0.0, ENC_Y / 2.0 + 22.0, 215.0);
    let right_panel = side_electronics_panel().translate(ENC_X / 2.0 + 22.0, 80.0, 245.0);
    let purge = purge_filter_regions();

    front_door + rear_bulkhead + right_panel + purge
}

fn service_door_panel(name: &str, x: f64, y: f64, z: f64, handles: usize) -> Part {
    let panel = centered_cube(format!("{name}_body"), x, y, z);
    let window_land = rect_frame_xz(
        &format!("{name}_dark_window_frame"),
        x - 120.0,
        z - 120.0,
        16.0,
        y + 2.0,
    );
    let mut handle_parts = Part::empty(format!("{name}_handles"));
    for i in 0..handles {
        let offset = (i as f64 - (handles as f64 - 1.0) / 2.0) * 190.0;
        handle_parts = handle_parts
            + centered_cube(format!("{name}_handle_{i}"), 88.0, 16.0, 26.0).translate(
                offset,
                -y / 2.0 - 8.0,
                0.0,
            );
    }
    panel + window_land + handle_parts + screw_pattern_xz(name, x - 55.0, z - 45.0, y + 3.0)
}

fn rear_service_bulkhead() -> Part {
    let body = centered_cube(
        "imaging_module_rear_service_bulkhead_body",
        600.0,
        20.0,
        320.0,
    );
    let mut cuts = Part::empty("imaging_module_rear_service_bulkhead_cuts");

    for (i, x) in [-230.0, -195.0, -160.0, -125.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("imaging_module_rear_gas_purge_port_{i}"),
                9.0 / 2.0,
                24.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 70.0);
    }
    for (i, x) in [-55.0, -25.0, 5.0, 35.0, 65.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("imaging_module_rear_aux_tube_port_{i}"),
                7.0 / 2.0,
                24.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 26.0);
    }
    for (i, x) in [145.0, 182.0, 219.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("imaging_module_rear_cable_gland_{i}"),
                18.0 / 2.0,
                24.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 32.0);
    }

    let power_slot = centered_cube("imaging_module_rear_power_filter_inlet", 90.0, 24.0, 34.0)
        .translate(238.0, 0.0, -82.0);
    let label = centered_cube("imaging_module_rear_service_label_land", 500.0, 4.0, 10.0)
        .translate(0.0, -12.0, 132.0);
    body - cuts - power_slot + label + receiver_alignment_pins()
}

fn receiver_alignment_pins() -> Part {
    let left = centered_cylinder("imaging_module_rear_alignment_pin_left", 7.0, 18.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-270.0, -18.0, -118.0);
    let right = centered_cylinder("imaging_module_rear_alignment_pin_right", 7.0, 18.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(270.0, -18.0, -118.0);
    left + right
}

fn side_electronics_panel() -> Part {
    let panel = centered_cube(
        "imaging_module_right_electronics_service_panel_body",
        20.0,
        360.0,
        315.0,
    );
    let frame = rect_frame_yz(
        "imaging_module_right_electronics_service_panel_frame",
        300.0,
        255.0,
        16.0,
        22.0,
    );
    let handle = centered_cube(
        "imaging_module_right_electronics_service_panel_handle",
        22.0,
        90.0,
        16.0,
    )
    .translate(-20.0, -132.0, 0.0);
    let controller = centered_cube("imaging_module_right_controller_plate", 18.0, 250.0, 150.0)
        .translate(0.0, 0.0, -25.0);
    let fan_filter = centered_cube(
        "imaging_module_right_clean_fan_filter_land",
        20.0,
        110.0,
        110.0,
    )
    .translate(0.0, 90.0, 72.0);
    panel
        + frame
        + handle
        + controller
        + fan_filter
        + screw_pattern_yz(
            "imaging_module_right_electronics_service_panel",
            310.0,
            260.0,
            24.0,
        )
}

fn purge_filter_regions() -> Part {
    let left_filter = filter_grille("imaging_module_left_hepa_purge_return").translate(
        -ENC_X / 2.0 - 8.0,
        250.0,
        390.0,
    );
    let right_filter = filter_grille("imaging_module_right_hepa_purge_supply").translate(
        ENC_X / 2.0 + 8.0,
        250.0,
        390.0,
    );
    left_filter + right_filter
}

fn filter_grille(name: &str) -> Part {
    let frame = centered_cube(format!("{name}_frame"), 18.0, 180.0, 88.0);
    let mut slats = Part::empty(format!("{name}_slats"));
    for i in 0..5 {
        slats = slats
            + centered_cube(format!("{name}_slat_{i}"), 20.0, 150.0, 6.0).translate(
                0.0,
                0.0,
                -30.0 + i as f64 * 15.0,
            );
    }
    frame + slats
}

fn calibration_target() -> Part {
    let target_x = NEST_X / 2.0 - 96.0;
    let target_y = -(NEST_Y / 2.0 - 86.0);
    let base_z = CASSETTE_DATUM_Z + CAL_TARGET_Z / 2.0 + 4.0;

    let tile = centered_cube(
        "imaging_module_calibration_ceramic_tile",
        CAL_TARGET_X,
        CAL_TARGET_Y,
        CAL_TARGET_Z,
    )
    .translate(target_x, target_y, base_z);
    let checker = checker_grid().translate(target_x - 22.0, target_y, base_z + 3.0);
    let fid_a = fiducial_disc("imaging_module_calibration_fiducial_a").translate(
        target_x - 46.0,
        target_y - 28.0,
        base_z + 4.0,
    );
    let fid_b = fiducial_disc("imaging_module_calibration_fiducial_b").translate(
        target_x + 46.0,
        target_y + 28.0,
        base_z + 4.0,
    );
    let fluorescence_refs =
        fluorescence_reference_wells().translate(target_x + 38.0, target_y - 8.0, base_z + 5.0);
    let gray_strip =
        neutral_density_step_strip().translate(target_x - 6.0, target_y + 34.0, base_z + 5.0);

    tile + checker + fid_a + fid_b + fluorescence_refs + gray_strip
}

fn checker_grid() -> Part {
    let mut grid = Part::empty("imaging_module_calibration_checker_grid");
    for i in 0..6 {
        grid = grid
            + centered_cube(
                format!("imaging_module_checker_vertical_{i}"),
                2.0,
                58.0,
                2.0,
            )
            .translate(-28.0 + i as f64 * 11.2, 0.0, 0.0);
        grid = grid
            + centered_cube(
                format!("imaging_module_checker_horizontal_{i}"),
                58.0,
                2.0,
                2.0,
            )
            .translate(0.0, -28.0 + i as f64 * 11.2, 0.0);
    }
    grid
}

fn fluorescence_reference_wells() -> Part {
    let mut wells = Part::empty("imaging_module_fluorescence_reference_wells");
    for (i, y) in [-22.0, 0.0, 22.0].iter().enumerate() {
        let cup = centered_cylinder(format!("imaging_module_fluor_ref_cup_{i}"), 8.0, 5.0, 32)
            .translate(0.0, *y, 0.0);
        let bore = centered_cylinder(format!("imaging_module_fluor_ref_bore_{i}"), 4.0, 6.0, 24)
            .translate(0.0, *y, 1.0);
        wells = wells + (cup - bore);
    }
    wells
}

fn neutral_density_step_strip() -> Part {
    let mut strip = Part::empty("imaging_module_neutral_density_step_strip");
    for i in 0..5 {
        strip = strip
            + centered_cube(
                format!("imaging_module_neutral_density_step_{i}"),
                14.0,
                18.0,
                1.0 + i as f64 * 0.6,
            )
            .translate(-30.0 + i as f64 * 15.0, 0.0, i as f64 * 0.3);
    }
    strip
}

fn cassette_keepout() -> Part {
    let base = centered_cube(
        "imaging_module_cassette_keepout_plate",
        CASSETTE_X,
        CASSETTE_Y,
        REVC_TOTAL_HEIGHT,
    )
    .translate(0.0, 0.0, CASSETTE_DATUM_Z + REVC_TOTAL_HEIGHT / 2.0);

    let mut chips = Part::empty("imaging_module_rev_c_chip_placeholders");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            chips = chips
                + centered_cube(
                    format!("imaging_module_chip_placeholder_{col}_{row}"),
                    REVC_CHIP_LENGTH,
                    REVC_CHIP_WIDTH,
                    REVC_TOTAL_HEIGHT + 1.0,
                )
                .translate(x, y, CASSETTE_DATUM_Z + REVC_TOTAL_HEIGHT / 2.0 + 0.5);
        }
    }

    base + chips
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    (
        -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * (REVC_CHIP_LENGTH + GUTTER),
        -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * (REVC_CHIP_WIDTH + GUTTER),
    )
}

fn fiducial_disc(name: &str) -> Part {
    let disk = centered_cylinder(format!("{name}_disk"), 9.0, 2.0, 40);
    let center = centered_cylinder(format!("{name}_center_bore"), 2.0, 3.0, 20);
    disk - center
}

fn gasket_frame_on_y(name: &str, x: f64, z: f64, rail: f64, depth: f64) -> Part {
    rect_frame_xz(name, x, z, rail, depth)
}

fn rect_frame_xz(name: &str, x: f64, z: f64, rail: f64, depth: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), x, depth, rail).translate(
        0.0,
        0.0,
        z / 2.0 - rail / 2.0,
    );
    let bottom = centered_cube(format!("{name}_bottom"), x, depth, rail).translate(
        0.0,
        0.0,
        -z / 2.0 + rail / 2.0,
    );
    let left = centered_cube(format!("{name}_left"), rail, depth, z).translate(
        -x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail, depth, z).translate(
        x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    top + bottom + left + right
}

fn rect_frame_xy(name: &str, x: f64, y: f64, rail: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        0.0,
    );
    let back =
        centered_cube(format!("{name}_back"), x, rail, z).translate(0.0, y / 2.0 - rail / 2.0, 0.0);
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
    front + back + left + right
}

fn rect_frame_yz(name: &str, y: f64, z: f64, rail: f64, depth: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), depth, y, rail).translate(
        0.0,
        0.0,
        z / 2.0 - rail / 2.0,
    );
    let bottom = centered_cube(format!("{name}_bottom"), depth, y, rail).translate(
        0.0,
        0.0,
        -z / 2.0 + rail / 2.0,
    );
    let left = centered_cube(format!("{name}_left"), depth, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), depth, rail, z).translate(
        0.0,
        y / 2.0 - rail / 2.0,
        0.0,
    );
    top + bottom + left + right
}

fn screw_pattern_xz(name: &str, x_span: f64, z_span: f64, depth: f64) -> Part {
    let mut screws = Part::empty(format!("{name}_screw_heads"));
    for (i, (x, z)) in [
        (-x_span / 2.0, -z_span / 2.0),
        (x_span / 2.0, -z_span / 2.0),
        (-x_span / 2.0, z_span / 2.0),
        (x_span / 2.0, z_span / 2.0),
        (0.0, -z_span / 2.0),
        (0.0, z_span / 2.0),
    ]
    .iter()
    .enumerate()
    {
        screws = screws
            + centered_cylinder(format!("{name}_captive_screw_{i}"), 5.0, 3.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, -depth / 2.0 - 2.0, *z);
    }
    screws
}

fn screw_pattern_yz(name: &str, y_span: f64, z_span: f64, depth: f64) -> Part {
    let mut screws = Part::empty(format!("{name}_side_screw_heads"));
    for (i, (y, z)) in [
        (-y_span / 2.0, -z_span / 2.0),
        (y_span / 2.0, -z_span / 2.0),
        (-y_span / 2.0, z_span / 2.0),
        (y_span / 2.0, z_span / 2.0),
        (0.0, -z_span / 2.0),
        (0.0, z_span / 2.0),
    ]
    .iter()
    .enumerate()
    {
        screws = screws
            + centered_cylinder(format!("{name}_side_captive_screw_{i}"), 5.0, 3.0, 24)
                .rotate(0.0, 90.0, 0.0)
                .translate(-depth / 2.0 - 2.0, *y, *z);
    }
    screws
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cassette_geometry_matches_twenty_chip_layout() {
        assert_eq!(COLS * ROWS, 20);
        assert!((ARRAY_X - 526.04).abs() < 0.01);
        assert!((ARRAY_Y - 447.40).abs() < 0.01);
    }

    #[test]
    fn gantry_travel_covers_cassette_and_calibration_target() {
        assert!(X_TRAVEL > NEST_X);
        assert!(Y_TRAVEL > CASSETTE_Y + 110.0);
        assert!(CAL_TARGET_X < X_TRAVEL - CASSETTE_X);
    }

    #[test]
    fn module_fits_inside_dark_clean_enclosure() {
        assert!(NEST_X < INNER_X - 80.0);
        assert!(NEST_Y < INNER_Y - 70.0);
        assert!(X_BEAM_LEN / 2.0 + 35.0 < INNER_X / 2.0);
        assert!(Y_RAIL_LEN / 2.0 + 20.0 < INNER_Y / 2.0);
    }

    #[test]
    fn optical_stack_has_serviceable_working_distance() {
        assert!(objective_working_gap() >= 60.0);
        assert!(OBJECTIVE_TIP_Z > CASSETTE_TOP_Z);
        assert!(OPTICS_BODY_Z < GANTRY_BEAM_Z);
    }
}
