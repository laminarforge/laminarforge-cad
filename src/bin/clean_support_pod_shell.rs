use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Clean support pod shell around a closed isolator.
//
// This is a compact support-pod architecture model, not a Grade A walk-in
// sterile room. The closed isolator remains the primary sterile boundary; the
// pod shell captures support panels, ante zones, pressure/DP stations, HEPA
// placeholders, a utility trench, RTP/VHP service wall, and service keepouts.

const POD_X: f64 = 3600.0;
const POD_Y: f64 = 2200.0;
const PANEL_H: f64 = 1850.0;
const PANEL_T: f64 = 36.0;
const MODULE_W: f64 = 600.0;

const FLOOR_T: f64 = 10.0;
const ZONE_T: f64 = 6.0;

const ISOLATOR_X: f64 = 1800.0;
const ISOLATOR_Y: f64 = 900.0;
const ISOLATOR_Z: f64 = 1320.0;

const FRONT_SERVICE_CLEARANCE: f64 = 900.0;
const REAR_SERVICE_CLEARANCE: f64 = 550.0;
const SIDE_SERVICE_CLEARANCE: f64 = 450.0;

const PERSONNEL_ANTE_X: f64 = 760.0;
const PERSONNEL_ANTE_Y: f64 = 820.0;
const MATERIAL_ANTE_X: f64 = 900.0;
const MATERIAL_ANTE_Y: f64 = 860.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let floor = floor_and_zone_tiles();
    export("output/clean_support_pod_shell_floor_and_zones.stl", &floor);

    let panels = modular_panel_shell();
    export("output/clean_support_pod_shell_modular_panels.stl", &panels);

    let ante = ante_zones();
    export("output/clean_support_pod_shell_ante_zones.stl", &ante);

    let pressure = pressure_cascade_panels();
    export(
        "output/clean_support_pod_shell_pressure_dp_stations.stl",
        &pressure,
    );

    let hepa = hepa_supply_return_placeholders();
    export(
        "output/clean_support_pod_shell_hepa_placeholders.stl",
        &hepa,
    );

    let trench = utility_trench();
    export("output/clean_support_pod_shell_utility_trench.stl", &trench);

    let isolator = isolator_footprint_zone();
    export(
        "output/clean_support_pod_shell_isolator_footprint.stl",
        &isolator,
    );

    let vhp_rtp = vhp_rtp_wall_zone();
    export("output/clean_support_pod_shell_vhp_rtp_wall.stl", &vhp_rtp);

    let clearances = service_clearance_zones();
    export(
        "output/clean_support_pod_shell_service_clearances.stl",
        &clearances,
    );

    let assembly =
        floor + panels + ante + pressure + hepa + trench + isolator + vhp_rtp + clearances;
    export("output/clean_support_pod_shell_assembly.stl", &assembly);

    println!();
    println!("Clean support pod shell dimensions:");
    println!("  Overall support-pod footprint: {POD_X:.0}mm x {POD_Y:.0}mm");
    println!("  Modular panel height:          {PANEL_H:.0}mm");
    println!(
        "  Closed isolator footprint:     {ISOLATOR_X:.0}mm x {ISOLATOR_Y:.0}mm x {ISOLATOR_Z:.0}mm envelope"
    );
    println!(
        "  Service clearances:            front {FRONT_SERVICE_CLEARANCE:.0}mm, rear {REAR_SERVICE_CLEARANCE:.0}mm, sides {SIDE_SERVICE_CLEARANCE:.0}mm"
    );
    println!("  Personnel ante zone:           {PERSONNEL_ANTE_X:.0}mm x {PERSONNEL_ANTE_Y:.0}mm");
    println!("  Material ante zone:            {MATERIAL_ANTE_X:.0}mm x {MATERIAL_ANTE_Y:.0}mm");
    println!(
        "  Boundary intent:               closed isolator plus support pod, not a Grade A walk-in room"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn floor_and_zone_tiles() -> Part {
    let floor = centered_cube("clean_pod_floor_plate", POD_X, POD_Y, FLOOR_T).translate(
        0.0,
        0.0,
        FLOOR_T / 2.0,
    );

    let seam_rail_w = 8.0;
    let mut seams = Part::empty("clean_pod_floor_panel_seams");
    for i in -2..=2 {
        let x = i as f64 * MODULE_W;
        seams = seams
            + centered_cube(
                format!("clean_pod_floor_longitudinal_seam_{i}"),
                seam_rail_w,
                POD_Y,
                2.0,
            )
            .translate(x, 0.0, FLOOR_T + 1.0);
    }
    for j in -1..=1 {
        let y = j as f64 * 550.0;
        seams = seams
            + centered_cube(
                format!("clean_pod_floor_cross_seam_{j}"),
                POD_X,
                seam_rail_w,
                2.0,
            )
            .translate(0.0, y, FLOOR_T + 1.0);
    }

    let personnel_zone = centered_cube(
        "personnel_ante_zone_floor_marker",
        PERSONNEL_ANTE_X,
        PERSONNEL_ANTE_Y,
        ZONE_T,
    )
    .translate(-1120.0, -650.0, FLOOR_T + ZONE_T / 2.0);

    let material_zone = centered_cube(
        "material_ante_zone_floor_marker",
        MATERIAL_ANTE_X,
        MATERIAL_ANTE_Y,
        ZONE_T,
    )
    .translate(1080.0, -630.0, FLOOR_T + ZONE_T / 2.0);

    let process_zone = centered_cube(
        "isolator_process_zone_floor_marker",
        ISOLATOR_X + 280.0,
        ISOLATOR_Y + 260.0,
        ZONE_T,
    )
    .translate(0.0, 80.0, FLOOR_T + ZONE_T / 2.0);

    floor + seams + personnel_zone + material_zone + process_zone
}

fn modular_panel_shell() -> Part {
    let mut shell = Part::empty("clean_support_pod_modular_panel_shell");
    let back_y = POD_Y / 2.0 - PANEL_T / 2.0;
    let side_span_y = POD_Y - PERSONNEL_ANTE_Y + 120.0;
    let side_center_y = back_y - side_span_y / 2.0 + PANEL_T / 2.0;

    for i in 0..6 {
        let x = -POD_X / 2.0 + MODULE_W / 2.0 + i as f64 * MODULE_W;
        let panel = centered_cube(
            format!("back_modular_panel_{i}"),
            MODULE_W - 42.0,
            PANEL_T,
            PANEL_H,
        )
        .translate(x, back_y, PANEL_H / 2.0);
        let vertical_seal = centered_cube(
            format!("back_panel_vertical_seal_{i}"),
            12.0,
            PANEL_T + 4.0,
            PANEL_H,
        )
        .translate(x + MODULE_W / 2.0 - 18.0, back_y, PANEL_H / 2.0);
        shell = shell + panel + vertical_seal;
    }

    for (side, x) in [
        ("left", -POD_X / 2.0 + PANEL_T / 2.0),
        ("right", POD_X / 2.0 - PANEL_T / 2.0),
    ] {
        for j in 0..3 {
            let y = side_center_y - side_span_y / 2.0 + MODULE_W / 2.0 + j as f64 * MODULE_W;
            shell = shell
                + centered_cube(
                    format!("{side}_side_modular_panel_{j}"),
                    PANEL_T,
                    MODULE_W - 48.0,
                    PANEL_H,
                )
                .translate(x, y, PANEL_H / 2.0);
        }

        let return_post_front = centered_cube(
            format!("{side}_front_return_post"),
            PANEL_T,
            PANEL_T,
            PANEL_H,
        )
        .translate(x, side_center_y - side_span_y / 2.0, PANEL_H / 2.0);
        let return_post_back = centered_cube(
            format!("{side}_back_corner_post"),
            PANEL_T,
            PANEL_T,
            PANEL_H,
        )
        .translate(x, back_y, PANEL_H / 2.0);
        shell = shell + return_post_front + return_post_back;
    }

    let back_top_rail = centered_cube("back_open_top_utility_rail", POD_X, 54.0, 72.0).translate(
        0.0,
        back_y,
        PANEL_H + 36.0,
    );
    let left_top_rail = centered_cube("left_open_top_utility_rail", 54.0, side_span_y, 72.0)
        .translate(-POD_X / 2.0 + PANEL_T / 2.0, side_center_y, PANEL_H + 36.0);
    let right_top_rail = centered_cube("right_open_top_utility_rail", 54.0, side_span_y, 72.0)
        .translate(POD_X / 2.0 - PANEL_T / 2.0, side_center_y, PANEL_H + 36.0);

    let front_low_curb = centered_cube("front_low_non_walkin_curb", POD_X - 520.0, 44.0, 220.0)
        .translate(0.0, -POD_Y / 2.0 + 22.0, 110.0);

    shell + back_top_rail + left_top_rail + right_top_rail + front_low_curb
}

fn ante_zones() -> Part {
    let personnel = ante_station(
        "personnel",
        -1120.0,
        -650.0,
        PERSONNEL_ANTE_X,
        PERSONNEL_ANTE_Y,
        1,
    );
    let material = ante_station(
        "material",
        1080.0,
        -630.0,
        MATERIAL_ANTE_X,
        MATERIAL_ANTE_Y,
        2,
    );

    let pass_through_shelf = centered_cube("shared_ante_handoff_shelf", 520.0, 120.0, 36.0)
        .translate(0.0, -POD_Y / 2.0 + 220.0, 940.0);
    let shelf_backstop = centered_cube("shared_ante_handoff_backstop", 520.0, 18.0, 180.0)
        .translate(0.0, -POD_Y / 2.0 + 286.0, 1012.0);

    personnel + material + pass_through_shelf + shelf_backstop
}

fn ante_station(name: &str, x: f64, y: f64, sx: f64, sy: f64, tier: usize) -> Part {
    let curb = rect_frame_xy(&format!("{name}_ante_floor_curb"), sx, sy, 24.0, 44.0).translate(
        x,
        y,
        FLOOR_T + 22.0,
    );

    let bench = centered_cube(
        format!("{name}_ante_service_bench"),
        sx - 120.0,
        170.0,
        70.0,
    )
    .translate(x, y - sy / 2.0 + 150.0, 210.0);

    let pass_panel = centered_cube(
        format!("{name}_ante_partial_height_panel"),
        sx - 80.0,
        30.0,
        820.0,
    )
    .translate(x, y + sy / 2.0 - 24.0, 510.0);

    let mut status_blocks = Part::empty(format!("{name}_ante_status_blocks"));
    for i in 0..tier {
        status_blocks = status_blocks
            + centered_cube(format!("{name}_ante_cascade_step_{i}"), 54.0, 18.0, 22.0).translate(
                x - sx / 2.0 + 92.0 + i as f64 * 70.0,
                y + sy / 2.0 - 42.0,
                930.0 + i as f64 * 38.0,
            );
    }

    curb + bench + pass_panel + status_blocks
}

fn pressure_cascade_panels() -> Part {
    let station_a = dp_station(
        "hall_to_personnel_ante",
        -1530.0,
        -POD_Y / 2.0 + 58.0,
        1,
        false,
    );
    let station_b = dp_station("ante_to_support_pod", -260.0, -POD_Y / 2.0 + 58.0, 2, false);
    let station_c = dp_station(
        "support_pod_to_closed_isolator_service",
        1530.0,
        -POD_Y / 2.0 + 58.0,
        3,
        false,
    );
    let rear_station = dp_station("rear_service_dp_station", 0.0, POD_Y / 2.0 - 58.0, 2, true);

    station_a + station_b + station_c + rear_station
}

fn dp_station(name: &str, x: f64, y: f64, steps: usize, face_positive_y: bool) -> Part {
    let face_offset = if face_positive_y { 16.0 } else { -16.0 };
    let gauge_y = y + face_offset;

    let body =
        centered_cube(format!("{name}_dp_panel_body"), 250.0, 28.0, 430.0).translate(x, y, 1120.0);
    let gauge_outer = centered_cylinder(format!("{name}_dp_gauge_outer"), 44.0, 14.0, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 52.0, gauge_y, 1185.0);
    let gauge_inner = centered_cylinder(format!("{name}_dp_gauge_face"), 32.0, 16.0, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 52.0, gauge_y - face_offset.signum() * 1.0, 1185.0);
    let gauge = gauge_outer - gauge_inner;

    let sensor_box = centered_cube(format!("{name}_sensor_box"), 84.0, 24.0, 68.0).translate(
        x + 64.0,
        gauge_y,
        1190.0,
    );
    let tube_riser = centered_cube(format!("{name}_dp_tube_riser"), 18.0, 14.0, 240.0).translate(
        x + 112.0,
        gauge_y,
        1070.0,
    );

    let mut cascade_steps = Part::empty(format!("{name}_cascade_steps"));
    for i in 0..steps {
        cascade_steps = cascade_steps
            + centered_cube(
                format!("{name}_pressure_step_{i}"),
                42.0 + i as f64 * 24.0,
                18.0,
                22.0,
            )
            .translate(x - 92.0 + i as f64 * 46.0, gauge_y, 940.0 + i as f64 * 34.0);
    }

    body + gauge + sensor_box + tube_riser + cascade_steps
}

fn hepa_supply_return_placeholders() -> Part {
    let supply_left = hepa_supply_box("supply_left", -780.0, 520.0);
    let supply_right = hepa_supply_box("supply_right", 780.0, 520.0);
    let return_left = return_grille_y("return_left_low", -980.0, POD_Y / 2.0 - 36.0, 360.0);
    let return_right = return_grille_y("return_right_low", 980.0, POD_Y / 2.0 - 36.0, 360.0);
    let side_return = return_grille_x("side_return_low", POD_X / 2.0 - 36.0, 140.0, 360.0);

    supply_left + supply_right + return_left + return_right + side_return
}

fn hepa_supply_box(name: &str, x: f64, y: f64) -> Part {
    let plenum = centered_cube(format!("{name}_hepa_supply_plenum"), 610.0, 305.0, 128.0)
        .translate(x, y, PANEL_H + 146.0);
    let face = centered_cube(format!("{name}_hepa_supply_face"), 560.0, 260.0, 18.0).translate(
        x,
        y,
        PANEL_H + 72.0,
    );

    let mut slats = Part::empty(format!("{name}_hepa_supply_slats"));
    for i in 0..6 {
        slats = slats
            + centered_cube(format!("{name}_supply_slat_{i}"), 520.0, 8.0, 8.0).translate(
                x,
                y - 105.0 + i as f64 * 42.0,
                PANEL_H + 60.0,
            );
    }

    plenum + face + slats
}

fn return_grille_y(name: &str, x: f64, y: f64, z: f64) -> Part {
    let frame =
        centered_cube(format!("{name}_return_frame"), 470.0, 26.0, 250.0).translate(x, y, z);
    let mut slats = Part::empty(format!("{name}_return_slats"));
    for i in 0..7 {
        slats = slats
            + centered_cube(format!("{name}_return_louver_{i}"), 420.0, 14.0, 9.0).translate(
                x,
                y - 8.0,
                z - 88.0 + i as f64 * 29.0,
            );
    }
    frame + slats
}

fn return_grille_x(name: &str, x: f64, y: f64, z: f64) -> Part {
    let frame =
        centered_cube(format!("{name}_return_frame"), 26.0, 470.0, 250.0).translate(x, y, z);
    let mut slats = Part::empty(format!("{name}_return_slats"));
    for i in 0..7 {
        slats = slats
            + centered_cube(format!("{name}_return_louver_{i}"), 14.0, 420.0, 9.0).translate(
                x - 8.0,
                y,
                z - 88.0 + i as f64 * 29.0,
            );
    }
    frame + slats
}

fn utility_trench() -> Part {
    let trench_outer = centered_cube("rear_utility_trench_outer", POD_X - 360.0, 220.0, 92.0)
        .translate(0.0, POD_Y / 2.0 - 170.0, 46.0);
    let trench_cavity = centered_cube(
        "rear_utility_trench_open_channel",
        POD_X - 520.0,
        146.0,
        84.0,
    )
    .translate(0.0, POD_Y / 2.0 - 170.0, 66.0);
    let mut covers = Part::empty("rear_utility_trench_segmented_covers");
    for i in 0..5 {
        let x = -1280.0 + i as f64 * 640.0;
        covers = covers
            + centered_cube(format!("rear_trench_cover_{i}"), 520.0, 166.0, 10.0).translate(
                x,
                POD_Y / 2.0 - 170.0,
                104.0,
            );
    }

    let gas_raceway = centered_cylinder("gas_utility_raceway_placeholder", 34.0, POD_X - 620.0, 36)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, POD_Y / 2.0 - 212.0, 76.0);
    let cable_raceway =
        centered_cylinder("cable_utility_raceway_placeholder", 24.0, POD_X - 700.0, 36)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, POD_Y / 2.0 - 128.0, 76.0);

    let sump = centered_cube("utility_trench_low_point_sump", 180.0, 92.0, 36.0).translate(
        POD_X / 2.0 - 520.0,
        POD_Y / 2.0 - 170.0,
        128.0,
    );

    trench_outer - trench_cavity + covers + gas_raceway + cable_raceway + sump
}

fn isolator_footprint_zone() -> Part {
    let footprint_plate = centered_cube(
        "closed_isolator_footprint_plate",
        ISOLATOR_X,
        ISOLATOR_Y,
        12.0,
    )
    .translate(0.0, 80.0, FLOOR_T + 18.0);
    let footprint_frame = rect_frame_xy(
        "closed_isolator_footprint_frame",
        ISOLATOR_X + 90.0,
        ISOLATOR_Y + 90.0,
        32.0,
        42.0,
    )
    .translate(0.0, 80.0, FLOOR_T + 54.0);

    let base_plinth = centered_cube("closed_isolator_base_plinth", ISOLATOR_X, ISOLATOR_Y, 150.0)
        .translate(0.0, 80.0, 120.0);
    let isolator_envelope = open_box_frame(
        "closed_isolator_envelope_frame",
        ISOLATOR_X,
        ISOLATOR_Y,
        ISOLATOR_Z,
        34.0,
    )
    .translate(0.0, 80.0, 150.0 + ISOLATOR_Z / 2.0);

    let front_glove_band = centered_cube(
        "closed_isolator_front_glove_band_placeholder",
        1180.0,
        28.0,
        250.0,
    )
    .translate(0.0, 80.0 - ISOLATOR_Y / 2.0 - 24.0, 780.0);
    let mut glove_ports = Part::empty("closed_isolator_glove_port_placeholders");
    for (i, x) in [-420.0, -140.0, 140.0, 420.0].iter().enumerate() {
        let outer = centered_cylinder(
            format!("closed_isolator_glove_port_outer_{i}"),
            88.0,
            18.0,
            56,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 80.0 - ISOLATOR_Y / 2.0 - 42.0, 780.0);
        let inner = centered_cylinder(
            format!("closed_isolator_glove_port_inner_{i}"),
            64.0,
            20.0,
            56,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 80.0 - ISOLATOR_Y / 2.0 - 43.0, 780.0);
        glove_ports = glove_ports + (outer - inner);
    }

    footprint_plate
        + footprint_frame
        + base_plinth
        + isolator_envelope
        + front_glove_band
        + glove_ports
}

fn vhp_rtp_wall_zone() -> Part {
    let wall_x = POD_X / 2.0 - PANEL_T / 2.0;
    let wall = centered_cube("vhp_rtp_wall_zone_panel", PANEL_T, 820.0, 960.0)
        .translate(wall_x, 320.0, 840.0);

    let rtp_outer = centered_cylinder("rtp_outer_flange_placeholder", 155.0, 28.0, 72)
        .rotate(0.0, 90.0, 0.0)
        .translate(wall_x - 24.0, 120.0, 920.0);
    let rtp_inner = centered_cylinder("rtp_inner_clearance_placeholder", 108.0, 32.0, 72)
        .rotate(0.0, 90.0, 0.0)
        .translate(wall_x - 25.0, 120.0, 920.0);
    let rtp_ring = rtp_outer - rtp_inner;

    let hatch_frame_outer = centered_cube("vhp_hatch_frame_outer", 30.0, 310.0, 420.0).translate(
        wall_x - 26.0,
        520.0,
        980.0,
    );
    let hatch_frame_inner = centered_cube("vhp_hatch_frame_inner", 34.0, 222.0, 318.0).translate(
        wall_x - 27.0,
        520.0,
        980.0,
    );
    let hatch_frame = hatch_frame_outer - hatch_frame_inner;

    let vapor_port_high = centered_cylinder("vhp_vapor_inlet_placeholder", 32.0, 34.0, 36)
        .rotate(0.0, 90.0, 0.0)
        .translate(wall_x - 28.0, 735.0, 1170.0);
    let vapor_port_low = centered_cylinder("vhp_return_port_placeholder", 32.0, 34.0, 36)
        .rotate(0.0, 90.0, 0.0)
        .translate(wall_x - 28.0, 735.0, 790.0);
    let interlock_box = centered_cube("vhp_rtp_interlock_box_placeholder", 24.0, 170.0, 90.0)
        .translate(wall_x - 32.0, -190.0, 1120.0);

    wall + rtp_ring + hatch_frame + vapor_port_high + vapor_port_low + interlock_box
}

fn service_clearance_zones() -> Part {
    let keepout_x = ISOLATOR_X + SIDE_SERVICE_CLEARANCE * 2.0;
    let keepout_y = ISOLATOR_Y + FRONT_SERVICE_CLEARANCE + REAR_SERVICE_CLEARANCE;
    let keepout_center_y = 80.0 + (REAR_SERVICE_CLEARANCE - FRONT_SERVICE_CLEARANCE) / 2.0;

    let floor_keepout = rect_frame_xy(
        "closed_isolator_service_keepout_floor",
        keepout_x,
        keepout_y,
        22.0,
        28.0,
    )
    .translate(0.0, keepout_center_y, FLOOR_T + 24.0);

    let rear_service_line = centered_cube(
        "rear_filter_service_clearance_bar",
        ISOLATOR_X + 260.0,
        36.0,
        80.0,
    )
    .translate(0.0, 80.0 + ISOLATOR_Y / 2.0 + REAR_SERVICE_CLEARANCE, 120.0);

    let front_operator_line = centered_cube(
        "front_operator_clearance_bar",
        ISOLATOR_X + 120.0,
        36.0,
        80.0,
    )
    .translate(
        0.0,
        80.0 - ISOLATOR_Y / 2.0 - FRONT_SERVICE_CLEARANCE,
        120.0,
    );

    let left_cart_line = centered_cube(
        "left_side_service_cart_clearance_bar",
        36.0,
        ISOLATOR_Y,
        80.0,
    )
    .translate(-ISOLATOR_X / 2.0 - SIDE_SERVICE_CLEARANCE, 80.0, 120.0);
    let right_cart_line = centered_cube(
        "right_side_service_cart_clearance_bar",
        36.0,
        ISOLATOR_Y,
        80.0,
    )
    .translate(ISOLATOR_X / 2.0 + SIDE_SERVICE_CLEARANCE, 80.0, 120.0);

    floor_keepout + rear_service_line + front_operator_line + left_cart_line + right_cart_line
}

fn rect_frame_xy(name: &str, x: f64, y: f64, rail: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let back = centered_cube(format!("{name}_back"), x, rail, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(x / 2.0, 0.0, 0.0);

    front + back + left + right
}

fn open_box_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut frame = Part::empty(format!("{name}_open_box_frame"));

    for (i, sx) in [-1.0_f64, 1.0].iter().enumerate() {
        for (j, sy) in [-1.0_f64, 1.0].iter().enumerate() {
            frame = frame
                + centered_cube(format!("{name}_post_{i}_{j}"), rail, rail, z).translate(
                    sx * (x / 2.0 - rail / 2.0),
                    sy * (y / 2.0 - rail / 2.0),
                    0.0,
                );
        }
    }

    for (level, zc) in [(-z / 2.0 + rail / 2.0), (z / 2.0 - rail / 2.0)]
        .iter()
        .enumerate()
    {
        frame = frame
            + centered_cube(format!("{name}_front_xrail_{level}"), x, rail, rail).translate(
                0.0,
                -y / 2.0 + rail / 2.0,
                *zc,
            )
            + centered_cube(format!("{name}_back_xrail_{level}"), x, rail, rail).translate(
                0.0,
                y / 2.0 - rail / 2.0,
                *zc,
            )
            + centered_cube(format!("{name}_left_yrail_{level}"), rail, y, rail).translate(
                -x / 2.0 + rail / 2.0,
                0.0,
                *zc,
            )
            + centered_cube(format!("{name}_right_yrail_{level}"), rail, y, rail).translate(
                x / 2.0 - rail / 2.0,
                0.0,
                *zc,
            );
    }

    frame
}
