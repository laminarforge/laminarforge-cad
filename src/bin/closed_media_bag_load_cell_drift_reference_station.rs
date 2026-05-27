use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag load-cell drift/reference validation station.
//
// Intent:
// - Hold a sealed hanging media bag on a mechanically isolated load-cell
//   bridge so drift can be checked against traceable reference masses without
//   opening the fluid path.
// - Keep thermal, tubing, prime/drain, label, and calibration-custody features
//   physically keyed around the weighing zone so setup variation is visible in
//   CAD and not hidden in a procedure.
// - This is concept/interface CAD only. It is not a weighing algorithm, sterile
//   processing instruction, metrology acceptance criterion, or load-cell
//   electronics design.

const OUTPUTS: [&str; 10] = [
    "output/closed_media_bag_load_cell_drift_reference_station_base_leak_tray.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_hanging_bag_frame.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_load_cell_bridge.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_reference_mass_tray.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_thermal_shield.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_tubing_strain_relief.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_drain_prime_capture_trough.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_calibration_label_plate.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_robot_service_keepouts.stl",
    "output/closed_media_bag_load_cell_drift_reference_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 10] = [
    "hanging_bag_frame",
    "reference_mass_tray",
    "load_cell_bridge",
    "thermal_shield",
    "tubing_strain_relief",
    "drain_prime_capture_trough",
    "calibration_label_plate",
    "leak_tray",
    "reference_mass_custody",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 18.0;

const HANGING_CENTER: (f64, f64) = (-335.0, 95.0);
const FRAME_X: f64 = 390.0;
const FRAME_Y: f64 = 330.0;
const FRAME_Z: f64 = 560.0;
const FRAME_POST_W: f64 = 28.0;
const FRAME_BEAM_W: f64 = 30.0;
const HOOK_RAIL_D: f64 = 18.0;
const HANGER_COUNT: usize = 5;
const HANGER_PITCH_X: f64 = 58.0;
const BAG_CLEARANCE_X: f64 = 250.0;
const BAG_CLEARANCE_Y: f64 = 82.0;
const BAG_CLEARANCE_Z: f64 = 350.0;

const BRIDGE_CENTER: (f64, f64) = HANGING_CENTER;
const LOAD_CELL_BRIDGE_X: f64 = 430.0;
const LOAD_CELL_BRIDGE_Y: f64 = 300.0;
const LOAD_CELL_BRIDGE_Z: f64 = 92.0;
const LOAD_CELL_BASE_Z: f64 = 28.0;
const LOAD_CELL_COUNT: usize = 4;
const LOAD_CELL_PAD_X: f64 = 84.0;
const LOAD_CELL_PAD_Y: f64 = 54.0;
const LOAD_CELL_PAD_Z: f64 = 14.0;
const FLEXURE_SLOT_COUNT: usize = 6;
const BRIDGE_CENTER_SPAN_X: f64 = 250.0;
const BRIDGE_CENTER_SPAN_Y: f64 = 86.0;
const CABLE_RELIEF_SLOTS: usize = 5;

const MASS_TRAY_CENTER: (f64, f64) = (265.0, 230.0);
const MASS_TRAY_X: f64 = 380.0;
const MASS_TRAY_Y: f64 = 270.0;
const MASS_TRAY_Z: f64 = 48.0;
const REFERENCE_MASS_COUNT: usize = 8;
const REFERENCE_MASS_COLS: usize = 4;
const MASS_WELL_D: f64 = 42.0;
const MASS_WELL_DEPTH: f64 = 22.0;
const MASS_WELL_PITCH_X: f64 = 74.0;
const MASS_WELL_PITCH_Y: f64 = 72.0;
const FINE_MASS_COUNT: usize = 6;
const FINE_MASS_WELL_D: f64 = 18.0;
const CERTIFICATE_SLOT_COUNT: usize = 4;

const THERMAL_SHIELD_CENTER: (f64, f64) = HANGING_CENTER;
const THERMAL_SHIELD_X: f64 = 470.0;
const THERMAL_SHIELD_Y: f64 = 420.0;
const THERMAL_SHIELD_Z: f64 = 420.0;
const SHIELD_PANEL_T: f64 = 7.0;
const SHIELD_STANDOFF_D: f64 = 18.0;
const SHIELD_AIR_GAP: f64 = 38.0;
const THERMAL_LOGGER_POCKETS: usize = 4;
const SHIELD_ACCESS_WINDOW_X: f64 = 260.0;
const SHIELD_ACCESS_WINDOW_Z: f64 = 250.0;

const TUBING_CENTER: (f64, f64) = (265.0, -45.0);
const TUBING_RAIL_X: f64 = 510.0;
const TUBING_RAIL_Y: f64 = 120.0;
const TUBING_RAIL_Z: f64 = 36.0;
const TUBE_CHANNELS: usize = 8;
const MEDIA_TUBE_OD_MAX: f64 = 12.7;
const TUBE_CHANNEL_CLEARANCE: f64 = 2.2;
const TUBE_CHANNEL_D: f64 = MEDIA_TUBE_OD_MAX + TUBE_CHANNEL_CLEARANCE;
const TUBE_CHANNEL_PITCH_X: f64 = 56.0;
const STRAIN_LOOP_COUNT: usize = 4;
const CLAMP_FLAG_COUNT: usize = 8;

const TROUGH_CENTER: (f64, f64) = (-325.0, -270.0);
const TROUGH_X: f64 = 520.0;
const TROUGH_Y: f64 = 210.0;
const TROUGH_Z: f64 = 58.0;
const TROUGH_WALL: f64 = 16.0;
const PRIME_VIAL_COUNT: usize = 6;
const PRIME_VIAL_COLS: usize = 3;
const PRIME_VIAL_D: f64 = 24.0;
const ABSORBENT_PAD_COUNT: usize = 5;
const DRIP_RIB_COUNT: usize = 6;

const LABEL_CENTER: (f64, f64) = (265.0, -270.0);
const LABEL_PLATE_X: f64 = 380.0;
const LABEL_PLATE_Y: f64 = 210.0;
const LABEL_PLATE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const CAL_STATUS_LANES: usize = 3;
const CAL_STATUS_TOKENS_PER_LANE: usize = 3;
const LABEL_LAND_X: f64 = 72.0;
const LABEL_LAND_Y: f64 = 22.0;
const LABEL_LAND_Z: f64 = 4.0;
const STATUS_TOKEN_X: f64 = 58.0;
const STATUS_TOKEN_Y: f64 = 28.0;

const FRONT_ROBOT_CLEARANCE: f64 = 410.0;
const REAR_THERMAL_SERVICE_CLEARANCE: f64 = 250.0;
const RIGHT_MASS_TRAY_SERVICE_CLEARANCE: f64 = 210.0;
const TOP_BAG_LIFT_CLEARANCE: f64 = 220.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    export(OUTPUTS[0], &base_leak_tray());
    export(OUTPUTS[1], &hanging_bag_frame());
    export(OUTPUTS[2], &load_cell_bridge());
    export(OUTPUTS[3], &reference_mass_tray());
    export(OUTPUTS[4], &thermal_shield());
    export(OUTPUTS[5], &tubing_strain_relief());
    export(OUTPUTS[6], &drain_prime_capture_trough());
    export(OUTPUTS[7], &calibration_label_plate());
    export(OUTPUTS[8], &robot_service_keepouts());
    export(OUTPUTS[9], &station_assembly());

    println!();
    println!("Closed media-bag load-cell drift/reference validation station:");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm leak-tray deck");
    println!(
        "  Hanging weigh zone:         {FRAME_X:.0}mm x {FRAME_Y:.0}mm x {FRAME_Z:.0}mm bag frame with {HANGER_COUNT} keyed hanger points and {LOAD_CELL_COUNT} load-cell bridge pads"
    );
    println!(
        "  Reference checks:           {REFERENCE_MASS_COUNT} primary mass wells, {FINE_MASS_COUNT} fine-mass wells, and {CERTIFICATE_SLOT_COUNT} certificate slots"
    );
    println!(
        "  Drift controls:             {THERMAL_LOGGER_POCKETS} thermal logger pockets, {SHIELD_AIR_GAP:.0}mm shield air gap, and {TUBE_CHANNELS} tubing strain-relief channels"
    );
    println!(
        "  Prime/drain capture:        {PRIME_VIAL_COUNT} prime vial wells, {ABSORBENT_PAD_COUNT} absorbent pad lands, {DRIP_RIB_COUNT} drip witness ribs, and {DRAIN_PORT_D:.0}mm drain interface"
    );
    println!(
        "  Calibration traceability:   {BARCODE_LANDS} barcode lands plus release/hold/reject calibration status lanes with {} token positions",
        CAL_STATUS_LANES * CAL_STATUS_TOKENS_PER_LANE
    );
    println!(
        "  Service envelopes:          front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear shield service {REAR_THERMAL_SERVICE_CLEARANCE:.0}mm, right mass-tray service {RIGHT_MASS_TRAY_SERVICE_CLEARANCE:.0}mm, top bag lift {TOP_BAG_LIFT_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray()
        + load_cell_bridge().translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z)
        + hanging_bag_frame().translate(HANGING_CENTER.0, HANGING_CENTER.1, BASE_Z)
        + thermal_shield().translate(THERMAL_SHIELD_CENTER.0, THERMAL_SHIELD_CENTER.1, BASE_Z)
        + reference_mass_tray().translate(MASS_TRAY_CENTER.0, MASS_TRAY_CENTER.1, BASE_Z)
        + tubing_strain_relief().translate(TUBING_CENTER.0, TUBING_CENTER.1, BASE_Z)
        + drain_prime_capture_trough().translate(TROUGH_CENTER.0, TROUGH_CENTER.1, BASE_Z)
        + calibration_label_plate().translate(LABEL_CENTER.0, LABEL_CENTER.1, BASE_Z)
        + robot_service_keepouts()
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_media_bag_load_cell_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "closed_media_bag_load_cell_station_washdown_basin",
        STATION_X - 112.0,
        STATION_Y - 106.0,
        SOCKET_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SOCKET_DEPTH / 2.0);
    let drain = centered_cylinder(
        "closed_media_bag_load_cell_station_front_drain_port",
        DRAIN_PORT_D / 2.0,
        56.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -STATION_X / 2.0 + 92.0,
        -STATION_Y / 2.0 - 2.0,
        BASE_Z - 7.0,
    );

    deck - basin - drain - deck_module_recesses() - deck_mount_holes()
        + perimeter_rims()
        + leak_witness_ribs()
        + datum_targets()
}

fn deck_module_recesses() -> Part {
    let mut recesses = Part::empty("closed_media_bag_load_cell_station_deck_module_recesses");
    for (name, center, x, y) in module_specs() {
        recesses = recesses
            + centered_cube(
                format!("closed_media_bag_load_cell_station_{name}_socket_recess"),
                x + 18.0,
                y + 18.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(center.0, center.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.2);
    }
    recesses
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_media_bag_load_cell_station_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 56.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 56.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 56.0),
        (0.0, -STATION_Y / 2.0 + 56.0),
        (0.0, STATION_Y / 2.0 - 56.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_media_bag_load_cell_station_front_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_media_bag_load_cell_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_media_bag_load_cell_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_media_bag_load_cell_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_bag_load_cell_station_leak_witness_ribs");
    for i in 0..DRIP_RIB_COUNT {
        let y = -330.0 + i as f64 * 132.0;
        ribs = ribs
            + centered_cube(
                format!("closed_media_bag_load_cell_station_drain_slope_witness_rib_{i}"),
                STATION_X - 200.0,
                7.0,
                6.0,
            )
            .translate(0.0, y, BASE_Z + 3.0);
    }
    ribs
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_media_bag_load_cell_station_robot_datum_targets");
    for (i, (x, y)) in [
        (-540.0, -350.0),
        (540.0, -350.0),
        (-540.0, 350.0),
        (540.0, 350.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_media_bag_load_cell_station_datum_boss_{i}"),
            16.0,
            7.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 3.5);
        let target = centered_cylinder(
            format!("closed_media_bag_load_cell_station_datum_center_dot_{i}"),
            4.2,
            8.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        targets = targets + (boss - target);
    }
    targets
}

fn hanging_bag_frame() -> Part {
    let posts = frame_posts();
    let top = frame_top_beams();
    let lower = frame_lower_cross_ties();
    let hook_rail = centered_cylinder(
        "closed_media_bag_load_cell_station_hanger_hook_rail",
        HOOK_RAIL_D / 2.0,
        FRAME_X - 92.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, FRAME_Z - 66.0);
    let hangers = keyed_hanger_points();
    let bag_gauge = bag_clearance_gauge();
    let anti_sway = anti_sway_pads();

    posts + top + lower + hook_rail + hangers + bag_gauge + anti_sway
}

fn frame_posts() -> Part {
    let mut posts = Part::empty("closed_media_bag_load_cell_station_frame_posts");
    for (i, (x, y)) in frame_corner_positions().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("closed_media_bag_load_cell_station_frame_post_{i}"),
                FRAME_POST_W,
                FRAME_POST_W,
                FRAME_Z,
            )
            .translate(*x, *y, FRAME_Z / 2.0);
    }
    posts
}

fn frame_top_beams() -> Part {
    let front = centered_cube(
        "closed_media_bag_load_cell_station_frame_top_front_beam",
        FRAME_X,
        FRAME_BEAM_W,
        FRAME_BEAM_W,
    )
    .translate(
        0.0,
        -FRAME_Y / 2.0 + FRAME_BEAM_W / 2.0,
        FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let rear = centered_cube(
        "closed_media_bag_load_cell_station_frame_top_rear_beam",
        FRAME_X,
        FRAME_BEAM_W,
        FRAME_BEAM_W,
    )
    .translate(
        0.0,
        FRAME_Y / 2.0 - FRAME_BEAM_W / 2.0,
        FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let left = centered_cube(
        "closed_media_bag_load_cell_station_frame_top_left_beam",
        FRAME_BEAM_W,
        FRAME_Y,
        FRAME_BEAM_W,
    )
    .translate(
        -FRAME_X / 2.0 + FRAME_BEAM_W / 2.0,
        0.0,
        FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let right = centered_cube(
        "closed_media_bag_load_cell_station_frame_top_right_beam",
        FRAME_BEAM_W,
        FRAME_Y,
        FRAME_BEAM_W,
    )
    .translate(
        FRAME_X / 2.0 - FRAME_BEAM_W / 2.0,
        0.0,
        FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    front + rear + left + right
}

fn frame_lower_cross_ties() -> Part {
    let front = centered_cube(
        "closed_media_bag_load_cell_station_frame_lower_front_tie",
        FRAME_X - 42.0,
        18.0,
        24.0,
    )
    .translate(0.0, -FRAME_Y / 2.0 + 22.0, 150.0);
    let rear = centered_cube(
        "closed_media_bag_load_cell_station_frame_lower_rear_tie",
        FRAME_X - 42.0,
        18.0,
        24.0,
    )
    .translate(0.0, FRAME_Y / 2.0 - 22.0, 150.0);
    let left = centered_cube(
        "closed_media_bag_load_cell_station_frame_lower_left_tie",
        18.0,
        FRAME_Y - 48.0,
        24.0,
    )
    .translate(-FRAME_X / 2.0 + 22.0, 0.0, 150.0);
    let right = centered_cube(
        "closed_media_bag_load_cell_station_frame_lower_right_tie",
        18.0,
        FRAME_Y - 48.0,
        24.0,
    )
    .translate(FRAME_X / 2.0 - 22.0, 0.0, 150.0);
    front + rear + left + right
}

fn keyed_hanger_points() -> Part {
    let mut hangers = Part::empty("closed_media_bag_load_cell_station_keyed_hanger_points");
    for i in 0..HANGER_COUNT {
        let x = centered_index(i, HANGER_COUNT, HANGER_PITCH_X);
        let lug = centered_cube(
            format!("closed_media_bag_load_cell_station_hanger_lug_{i}"),
            34.0,
            24.0,
            42.0,
        )
        .translate(x, 0.0, FRAME_Z - 108.0);
        let hook_pin = centered_cylinder(
            format!("closed_media_bag_load_cell_station_hanger_pin_{i}"),
            6.0,
            46.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, FRAME_Z - 110.0);
        let id_flat = centered_cube(
            format!("closed_media_bag_load_cell_station_hanger_asymmetric_key_flat_{i}"),
            16.0,
            28.0,
            10.0,
        )
        .translate(x + 8.0, 0.0, FRAME_Z - 88.0);
        hangers = hangers + lug + hook_pin + id_flat;
    }
    hangers
}

fn bag_clearance_gauge() -> Part {
    let upper_gauge = centered_cube(
        "closed_media_bag_load_cell_station_bag_upper_swing_clearance_gauge",
        BAG_CLEARANCE_X,
        7.0,
        7.0,
    )
    .translate(0.0, -BAG_CLEARANCE_Y / 2.0, FRAME_Z - 180.0);
    let lower_gauge = centered_cube(
        "closed_media_bag_load_cell_station_bag_lower_swing_clearance_gauge",
        BAG_CLEARANCE_X * 0.78,
        7.0,
        7.0,
    )
    .translate(0.0, -BAG_CLEARANCE_Y / 2.0, FRAME_Z - BAG_CLEARANCE_Z);
    let left_gauge = centered_cube(
        "closed_media_bag_load_cell_station_bag_left_clearance_gauge",
        7.0,
        7.0,
        BAG_CLEARANCE_Z - 150.0,
    )
    .translate(
        -BAG_CLEARANCE_X / 2.0,
        -BAG_CLEARANCE_Y / 2.0,
        FRAME_Z - 265.0,
    );
    let right_gauge = centered_cube(
        "closed_media_bag_load_cell_station_bag_right_clearance_gauge",
        7.0,
        7.0,
        BAG_CLEARANCE_Z - 150.0,
    )
    .translate(
        BAG_CLEARANCE_X / 2.0,
        -BAG_CLEARANCE_Y / 2.0,
        FRAME_Z - 265.0,
    );
    let bottom_saddle_shadow = centered_cube(
        "closed_media_bag_load_cell_station_bag_bottom_load_shadow",
        BAG_CLEARANCE_X * 0.64,
        BAG_CLEARANCE_Y,
        10.0,
    )
    .translate(0.0, 0.0, 124.0);

    upper_gauge + lower_gauge + left_gauge + right_gauge + bottom_saddle_shadow
}

fn anti_sway_pads() -> Part {
    let mut pads = Part::empty("closed_media_bag_load_cell_station_anti_sway_pads");
    for (i, x) in [-122.0, 122.0].iter().enumerate() {
        pads = pads
            + centered_cube(
                format!("closed_media_bag_load_cell_station_bag_anti_sway_pad_{i}"),
                54.0,
                18.0,
                86.0,
            )
            .translate(*x, -FRAME_Y / 2.0 - 9.0, 256.0);
    }
    pads
}

fn load_cell_bridge() -> Part {
    let base = centered_cube(
        "closed_media_bag_load_cell_station_bridge_base_plate",
        LOAD_CELL_BRIDGE_X,
        LOAD_CELL_BRIDGE_Y,
        LOAD_CELL_BASE_Z,
    )
    .translate(0.0, 0.0, LOAD_CELL_BASE_Z / 2.0);
    let center_span = centered_cube(
        "closed_media_bag_load_cell_station_load_cell_bridge_span",
        BRIDGE_CENTER_SPAN_X,
        BRIDGE_CENTER_SPAN_Y,
        34.0,
    )
    .translate(0.0, 0.0, LOAD_CELL_BASE_Z + 17.0);
    let load_pan = centered_cube(
        "closed_media_bag_load_cell_station_hanging_bag_load_pan",
        230.0,
        150.0,
        18.0,
    )
    .translate(0.0, 0.0, LOAD_CELL_BRIDGE_Z - 9.0);
    let center_handoff_socket = centered_cube(
        "closed_media_bag_load_cell_station_center_handoff_socket",
        102.0,
        62.0,
        12.0,
    )
    .translate(0.0, 0.0, LOAD_CELL_BRIDGE_Z - 6.0);

    base + bridge_mount_pads() + (center_span - flexure_relief_slots()) + load_pan
        - center_handoff_socket
        + bridge_cable_strain_relief()
        + bridge_leveling_feet()
}

fn bridge_mount_pads() -> Part {
    let mut pads = Part::empty("closed_media_bag_load_cell_station_load_cell_mount_pads");
    for (i, (x, y)) in load_cell_pad_positions().iter().enumerate() {
        let pad = centered_cube(
            format!("closed_media_bag_load_cell_station_load_cell_pad_{i}"),
            LOAD_CELL_PAD_X,
            LOAD_CELL_PAD_Y,
            LOAD_CELL_PAD_Z,
        )
        .translate(*x, *y, LOAD_CELL_BASE_Z + LOAD_CELL_PAD_Z / 2.0);
        let fastener_a = centered_cylinder(
            format!("closed_media_bag_load_cell_station_load_cell_pad_{i}_fastener_a"),
            3.2,
            LOAD_CELL_PAD_Z + 4.0,
            20,
        )
        .translate(*x - 22.0, *y, LOAD_CELL_BASE_Z + LOAD_CELL_PAD_Z / 2.0);
        let fastener_b = centered_cylinder(
            format!("closed_media_bag_load_cell_station_load_cell_pad_{i}_fastener_b"),
            3.2,
            LOAD_CELL_PAD_Z + 4.0,
            20,
        )
        .translate(*x + 22.0, *y, LOAD_CELL_BASE_Z + LOAD_CELL_PAD_Z / 2.0);
        pads = pads + (pad - fastener_a - fastener_b);
    }
    pads
}

fn flexure_relief_slots() -> Part {
    let mut slots = Part::empty("closed_media_bag_load_cell_station_flexure_relief_slots");
    for i in 0..FLEXURE_SLOT_COUNT {
        let x = centered_index(i, FLEXURE_SLOT_COUNT, 37.0);
        slots = slots
            + centered_cube(
                format!("closed_media_bag_load_cell_station_bridge_flexure_slot_{i}"),
                10.0,
                BRIDGE_CENTER_SPAN_Y + 8.0,
                38.0,
            )
            .translate(x, 0.0, LOAD_CELL_BASE_Z + 18.0);
    }
    slots
}

fn bridge_cable_strain_relief() -> Part {
    let comb = centered_cube(
        "closed_media_bag_load_cell_station_bridge_cable_relief_comb",
        250.0,
        28.0,
        26.0,
    )
    .translate(
        0.0,
        LOAD_CELL_BRIDGE_Y / 2.0 + 18.0,
        LOAD_CELL_BASE_Z + 13.0,
    );
    let mut slots = Part::empty("closed_media_bag_load_cell_station_bridge_cable_slots");
    for i in 0..CABLE_RELIEF_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_media_bag_load_cell_station_bridge_cable_slot_{i}"),
                12.0,
                34.0,
                16.0,
            )
            .translate(
                centered_index(i, CABLE_RELIEF_SLOTS, 42.0),
                LOAD_CELL_BRIDGE_Y / 2.0 + 18.0,
                LOAD_CELL_BASE_Z + 13.0,
            );
    }
    comb - slots
}

fn bridge_leveling_feet() -> Part {
    let mut feet = Part::empty("closed_media_bag_load_cell_station_bridge_leveling_feet");
    for (i, (x, y)) in [
        (
            -LOAD_CELL_BRIDGE_X / 2.0 + 38.0,
            -LOAD_CELL_BRIDGE_Y / 2.0 + 36.0,
        ),
        (
            LOAD_CELL_BRIDGE_X / 2.0 - 38.0,
            -LOAD_CELL_BRIDGE_Y / 2.0 + 36.0,
        ),
        (
            -LOAD_CELL_BRIDGE_X / 2.0 + 38.0,
            LOAD_CELL_BRIDGE_Y / 2.0 - 36.0,
        ),
        (
            LOAD_CELL_BRIDGE_X / 2.0 - 38.0,
            LOAD_CELL_BRIDGE_Y / 2.0 - 36.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let foot = centered_cylinder(
            format!("closed_media_bag_load_cell_station_bridge_leveling_foot_{i}"),
            16.0,
            14.0,
            32,
        )
        .translate(*x, *y, 7.0);
        let screw = centered_cylinder(
            format!("closed_media_bag_load_cell_station_bridge_leveling_screw_clearance_{i}"),
            4.0,
            18.0,
            24,
        )
        .translate(*x, *y, 7.0);
        feet = feet + (foot - screw);
    }
    feet
}

fn reference_mass_tray() -> Part {
    let block = centered_cube(
        "closed_media_bag_load_cell_station_reference_mass_tray_block",
        MASS_TRAY_X,
        MASS_TRAY_Y,
        MASS_TRAY_Z,
    )
    .translate(0.0, 0.0, MASS_TRAY_Z / 2.0);
    let lip = tray_lip(
        "closed_media_bag_load_cell_station_reference_mass_tray_lip",
        MASS_TRAY_X,
        MASS_TRAY_Y,
        MASS_TRAY_Z,
    );
    let mass_wells = reference_mass_wells();
    let fine_wells = fine_mass_wells();
    let custody = reference_mass_custody_features();

    block - mass_wells - fine_wells + lip + custody
}

fn reference_mass_wells() -> Part {
    let mut wells = Part::empty("closed_media_bag_load_cell_station_reference_mass_wells");
    for i in 0..REFERENCE_MASS_COUNT {
        let (x, y) = grid_position(
            i,
            REFERENCE_MASS_COLS,
            REFERENCE_MASS_COUNT,
            MASS_WELL_PITCH_X,
            MASS_WELL_PITCH_Y,
        );
        wells = wells
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_reference_mass_well_{i}"),
                MASS_WELL_D / 2.0,
                MASS_WELL_DEPTH,
                48,
            )
            .translate(
                x - 18.0,
                y + 18.0,
                MASS_TRAY_Z - MASS_WELL_DEPTH / 2.0 + 0.2,
            );
    }
    wells
}

fn fine_mass_wells() -> Part {
    let mut wells = Part::empty("closed_media_bag_load_cell_station_fine_mass_wells");
    for i in 0..FINE_MASS_COUNT {
        wells = wells
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_fine_trim_mass_well_{i}"),
                FINE_MASS_WELL_D / 2.0,
                MASS_WELL_DEPTH,
                32,
            )
            .translate(
                -135.0 + i as f64 * 54.0,
                -MASS_TRAY_Y / 2.0 + 42.0,
                MASS_TRAY_Z - MASS_WELL_DEPTH / 2.0 + 0.2,
            );
    }
    wells
}

fn reference_mass_custody_features() -> Part {
    let tamper_rail = centered_cube(
        "closed_media_bag_load_cell_station_reference_mass_tamper_seal_rail",
        MASS_TRAY_X - 44.0,
        18.0,
        22.0,
    )
    .translate(0.0, MASS_TRAY_Y / 2.0 + 10.0, MASS_TRAY_Z + 11.0);
    let mut cert_slots = Part::empty("closed_media_bag_load_cell_station_mass_certificate_slots");
    for i in 0..CERTIFICATE_SLOT_COUNT {
        cert_slots = cert_slots
            + centered_cube(
                format!("closed_media_bag_load_cell_station_certificate_card_slot_{i}"),
                68.0,
                16.0,
                18.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_SLOT_COUNT, 82.0),
                -MASS_TRAY_Y / 2.0 - 12.0,
                MASS_TRAY_Z + 9.0,
            );
    }
    let scale_transfer_land = centered_cube(
        "closed_media_bag_load_cell_station_reference_mass_transfer_land",
        150.0,
        58.0,
        6.0,
    )
    .translate(MASS_TRAY_X / 2.0 - 94.0, 0.0, MASS_TRAY_Z + 3.0);

    tamper_rail + cert_slots + scale_transfer_land
}

fn thermal_shield() -> Part {
    let rear_panel = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_rear_panel",
        THERMAL_SHIELD_X,
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Z,
    )
    .translate(
        0.0,
        THERMAL_SHIELD_Y / 2.0 - SHIELD_PANEL_T / 2.0,
        THERMAL_SHIELD_Z / 2.0,
    );
    let left_panel = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_left_panel",
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Y,
        THERMAL_SHIELD_Z,
    )
    .translate(
        -THERMAL_SHIELD_X / 2.0 + SHIELD_PANEL_T / 2.0,
        0.0,
        THERMAL_SHIELD_Z / 2.0,
    );
    let right_panel = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_right_panel",
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Y,
        THERMAL_SHIELD_Z,
    )
    .translate(
        THERMAL_SHIELD_X / 2.0 - SHIELD_PANEL_T / 2.0,
        0.0,
        THERMAL_SHIELD_Z / 2.0,
    );
    let roof = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_roof",
        THERMAL_SHIELD_X,
        THERMAL_SHIELD_Y,
        SHIELD_PANEL_T,
    )
    .translate(0.0, 0.0, THERMAL_SHIELD_Z - SHIELD_PANEL_T / 2.0);
    let front_window_header = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_front_window_header",
        THERMAL_SHIELD_X,
        18.0,
        32.0,
    )
    .translate(0.0, -THERMAL_SHIELD_Y / 2.0 + 9.0, THERMAL_SHIELD_Z - 58.0);
    let front_sill = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_front_sill",
        THERMAL_SHIELD_X,
        18.0,
        28.0,
    )
    .translate(0.0, -THERMAL_SHIELD_Y / 2.0 + 9.0, 72.0);
    let access_gauge = centered_cube(
        "closed_media_bag_load_cell_station_thermal_shield_front_access_window_gauge",
        SHIELD_ACCESS_WINDOW_X,
        6.0,
        SHIELD_ACCESS_WINDOW_Z,
    )
    .translate(0.0, -THERMAL_SHIELD_Y / 2.0 - 6.0, 230.0);

    rear_panel
        + left_panel
        + right_panel
        + roof
        + front_window_header
        + front_sill
        + access_gauge
        + shield_standoffs()
        + thermal_logger_pockets()
        + shield_air_gap_gauges()
}

fn shield_standoffs() -> Part {
    let mut posts = Part::empty("closed_media_bag_load_cell_station_thermal_shield_standoffs");
    for (i, (x, y)) in [
        (
            -THERMAL_SHIELD_X / 2.0 + 34.0,
            -THERMAL_SHIELD_Y / 2.0 + 36.0,
        ),
        (
            THERMAL_SHIELD_X / 2.0 - 34.0,
            -THERMAL_SHIELD_Y / 2.0 + 36.0,
        ),
        (
            -THERMAL_SHIELD_X / 2.0 + 34.0,
            THERMAL_SHIELD_Y / 2.0 - 36.0,
        ),
        (THERMAL_SHIELD_X / 2.0 - 34.0, THERMAL_SHIELD_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_shield_standoff_{i}"),
                SHIELD_STANDOFF_D / 2.0,
                THERMAL_SHIELD_Z,
                28,
            )
            .translate(*x, *y, THERMAL_SHIELD_Z / 2.0);
    }
    posts
}

fn thermal_logger_pockets() -> Part {
    let mut pockets = Part::empty("closed_media_bag_load_cell_station_thermal_logger_pockets");
    for i in 0..THERMAL_LOGGER_POCKETS {
        pockets = pockets
            + centered_cube(
                format!("closed_media_bag_load_cell_station_thermal_logger_pocket_{i}"),
                72.0,
                18.0,
                38.0,
            )
            .translate(
                centered_index(i, THERMAL_LOGGER_POCKETS, 90.0),
                THERMAL_SHIELD_Y / 2.0 + 9.0,
                110.0,
            );
    }
    pockets
}

fn shield_air_gap_gauges() -> Part {
    let left_gap = centered_cube(
        "closed_media_bag_load_cell_station_left_thermal_air_gap_gauge",
        10.0,
        SHIELD_AIR_GAP,
        80.0,
    )
    .translate(-BAG_CLEARANCE_X / 2.0 - SHIELD_AIR_GAP / 2.0, 0.0, 250.0);
    let right_gap = centered_cube(
        "closed_media_bag_load_cell_station_right_thermal_air_gap_gauge",
        10.0,
        SHIELD_AIR_GAP,
        80.0,
    )
    .translate(BAG_CLEARANCE_X / 2.0 + SHIELD_AIR_GAP / 2.0, 0.0, 250.0);
    let rear_gap = centered_cube(
        "closed_media_bag_load_cell_station_rear_thermal_air_gap_gauge",
        BAG_CLEARANCE_X,
        10.0,
        80.0,
    )
    .translate(0.0, BAG_CLEARANCE_Y / 2.0 + SHIELD_AIR_GAP, 250.0);

    left_gap + right_gap + rear_gap
}

fn tubing_strain_relief() -> Part {
    let rail = centered_cube(
        "closed_media_bag_load_cell_station_tubing_strain_relief_rail",
        TUBING_RAIL_X,
        TUBING_RAIL_Y,
        TUBING_RAIL_Z,
    )
    .translate(0.0, 0.0, TUBING_RAIL_Z / 2.0);
    let channel_cuts = tubing_channel_cuts();
    let bridge_slots = tube_bridge_slots();
    let flags = tube_id_flags();
    let loops = strain_loop_gauges();
    let drain_clamps = drain_prime_tube_clamps();

    rail - channel_cuts - bridge_slots + flags + loops + drain_clamps
}

fn tubing_channel_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bag_load_cell_station_tubing_channel_cuts");
    for i in 0..TUBE_CHANNELS {
        let x = centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH_X);
        let bore = centered_cylinder(
            format!("closed_media_bag_load_cell_station_tube_channel_bore_{i}"),
            TUBE_CHANNEL_D / 2.0,
            TUBING_RAIL_Y + 12.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, TUBING_RAIL_Z / 2.0 + 2.0);
        let top_slot = centered_cube(
            format!("closed_media_bag_load_cell_station_tube_channel_top_slot_{i}"),
            TUBE_CHANNEL_D + 2.0,
            TUBING_RAIL_Y + 14.0,
            TUBING_RAIL_Z,
        )
        .translate(x, 0.0, TUBING_RAIL_Z - 6.0);
        cuts = cuts + bore + top_slot;
    }
    cuts
}

fn tube_bridge_slots() -> Part {
    let mut slots = Part::empty("closed_media_bag_load_cell_station_tube_bridge_slots");
    for i in 0..STRAIN_LOOP_COUNT {
        slots = slots
            + centered_cube(
                format!("closed_media_bag_load_cell_station_strain_loop_slot_{i}"),
                46.0,
                30.0,
                20.0,
            )
            .translate(
                centered_index(i, STRAIN_LOOP_COUNT, 104.0),
                -TUBING_RAIL_Y / 2.0 + 28.0,
                TUBING_RAIL_Z / 2.0,
            );
    }
    slots
}

fn tube_id_flags() -> Part {
    let mut flags = Part::empty("closed_media_bag_load_cell_station_tube_id_flags");
    for i in 0..CLAMP_FLAG_COUNT {
        flags = flags
            + centered_cube(
                format!("closed_media_bag_load_cell_station_tube_id_flag_{i}"),
                38.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(i, CLAMP_FLAG_COUNT, TUBE_CHANNEL_PITCH_X),
                TUBING_RAIL_Y / 2.0 + 12.0,
                TUBING_RAIL_Z + 2.0,
            );
    }
    flags
}

fn strain_loop_gauges() -> Part {
    let mut loops = Part::empty("closed_media_bag_load_cell_station_strain_loop_gauges");
    for i in 0..STRAIN_LOOP_COUNT {
        let x = centered_index(i, STRAIN_LOOP_COUNT, 104.0);
        loops = loops
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_strain_loop_radius_gauge_{i}"),
                34.0,
                8.0,
                48,
            )
            .translate(x, -TUBING_RAIL_Y / 2.0 - 22.0, TUBING_RAIL_Z + 4.0);
    }
    loops
}

fn drain_prime_tube_clamps() -> Part {
    let mut clamps = Part::empty("closed_media_bag_load_cell_station_drain_prime_tube_clamps");
    for i in 0..3 {
        clamps = clamps
            + centered_cube(
                format!("closed_media_bag_load_cell_station_drain_prime_tube_clamp_{i}"),
                70.0,
                24.0,
                28.0,
            )
            .translate(-150.0 + i as f64 * 150.0, -TUBING_RAIL_Y / 2.0 - 46.0, 18.0);
    }
    clamps
}

fn drain_prime_capture_trough() -> Part {
    let outer = centered_cube(
        "closed_media_bag_load_cell_station_drain_prime_capture_trough_outer",
        TROUGH_X,
        TROUGH_Y,
        TROUGH_Z,
    )
    .translate(0.0, 0.0, TROUGH_Z / 2.0);
    let cavity = centered_cube(
        "closed_media_bag_load_cell_station_drain_prime_capture_trough_cavity",
        TROUGH_X - 2.0 * TROUGH_WALL,
        TROUGH_Y - 2.0 * TROUGH_WALL,
        TROUGH_Z - 18.0,
    )
    .translate(0.0, 0.0, TROUGH_Z / 2.0 + 9.0);
    let drain = centered_cylinder(
        "closed_media_bag_load_cell_station_trough_barb_drain",
        8.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TROUGH_X / 2.0 - 48.0, -TROUGH_Y / 2.0 - 6.0, 20.0);

    outer - cavity - drain + prime_vial_pockets() + absorbent_pad_lands() + trough_flow_ribs()
}

fn prime_vial_pockets() -> Part {
    let mut pockets = Part::empty("closed_media_bag_load_cell_station_prime_vial_pockets");
    for i in 0..PRIME_VIAL_COUNT {
        let (x, y) = grid_position(i, PRIME_VIAL_COLS, PRIME_VIAL_COUNT, 66.0, 60.0);
        pockets = pockets
            + centered_cylinder(
                format!("closed_media_bag_load_cell_station_prime_vial_well_{i}"),
                PRIME_VIAL_D / 2.0,
                24.0,
                32,
            )
            .translate(x - 135.0, y, TROUGH_Z + 12.0);
    }
    pockets
}

fn absorbent_pad_lands() -> Part {
    let mut pads = Part::empty("closed_media_bag_load_cell_station_absorbent_pad_lands");
    for i in 0..ABSORBENT_PAD_COUNT {
        pads = pads
            + centered_cube(
                format!("closed_media_bag_load_cell_station_absorbent_pad_land_{i}"),
                54.0,
                28.0,
                5.0,
            )
            .translate(-34.0 + i as f64 * 64.0, 0.0, TROUGH_Z + 2.5);
    }
    pads
}

fn trough_flow_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_bag_load_cell_station_trough_flow_ribs");
    for i in 0..DRIP_RIB_COUNT {
        ribs = ribs
            + centered_cube(
                format!("closed_media_bag_load_cell_station_trough_flow_rib_{i}"),
                7.0,
                TROUGH_Y - 46.0,
                8.0,
            )
            .translate(-205.0 + i as f64 * 82.0, 0.0, TROUGH_Z + 4.0);
    }
    ribs
}

fn calibration_label_plate() -> Part {
    let plate = centered_cube(
        "closed_media_bag_load_cell_station_calibration_label_plate",
        LABEL_PLATE_X,
        LABEL_PLATE_Y,
        LABEL_PLATE_Z,
    )
    .translate(0.0, 0.0, LABEL_PLATE_Z / 2.0);
    let bevel_gauge = centered_cube(
        "closed_media_bag_load_cell_station_label_plate_wipe_bevel_gauge",
        LABEL_PLATE_X - 28.0,
        LABEL_PLATE_Y - 28.0,
        4.0,
    )
    .translate(0.0, 0.0, LABEL_PLATE_Z + 2.0);

    plate + bevel_gauge + barcode_lands() + calibration_status_lanes() + evidence_card_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_media_bag_load_cell_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let (x, y) = grid_position(i, 5, BARCODE_LANDS, 70.0, 42.0);
        lands = lands
            + centered_cube(
                format!("closed_media_bag_load_cell_station_calibration_barcode_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(x, y + 42.0, LABEL_PLATE_Z + LABEL_LAND_Z / 2.0);
    }
    lands
}

fn calibration_status_lanes() -> Part {
    let mut lanes = Part::empty("closed_media_bag_load_cell_station_calibration_status_lanes");
    for lane in 0..CAL_STATUS_LANES {
        let x = centered_index(lane, CAL_STATUS_LANES, 108.0);
        let rail = centered_cube(
            format!("closed_media_bag_load_cell_station_status_lane_{lane}_rail"),
            88.0,
            96.0,
            12.0,
        )
        .translate(x, -44.0, LABEL_PLATE_Z + 6.0);
        let mut tokens = Part::empty(format!(
            "closed_media_bag_load_cell_station_status_lane_{lane}_token_slots"
        ));
        for slot in 0..CAL_STATUS_TOKENS_PER_LANE {
            tokens = tokens
                + centered_cube(
                    format!("closed_media_bag_load_cell_station_status_lane_{lane}_token_{slot}"),
                    STATUS_TOKEN_X,
                    STATUS_TOKEN_Y,
                    8.0,
                )
                .translate(x, -82.0 + slot as f64 * 38.0, LABEL_PLATE_Z + 10.0);
        }
        lanes = lanes + rail + tokens;
    }
    lanes
}

fn evidence_card_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_bag_load_cell_station_evidence_card_tabs");
    for i in 0..CERTIFICATE_SLOT_COUNT {
        tabs = tabs
            + centered_cube(
                format!("closed_media_bag_load_cell_station_label_evidence_card_tab_{i}"),
                72.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_SLOT_COUNT, 82.0),
                LABEL_PLATE_Y / 2.0 + 10.0,
                LABEL_PLATE_Z + 5.0,
            );
    }
    tabs
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_media_bag_load_cell_station_front_robot_keepout_gauge",
        STATION_X - 160.0,
        FRONT_ROBOT_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_media_bag_load_cell_station_rear_thermal_service_keepout_gauge",
        THERMAL_SHIELD_X + 130.0,
        REAR_THERMAL_SERVICE_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        HANGING_CENTER.0,
        STATION_Y / 2.0 + REAR_THERMAL_SERVICE_CLEARANCE / 2.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_bag_load_cell_station_right_mass_tray_service_keepout_gauge",
        RIGHT_MASS_TRAY_SERVICE_CLEARANCE,
        330.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_MASS_TRAY_SERVICE_CLEARANCE / 2.0,
        MASS_TRAY_CENTER.1,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top_lift = centered_cube(
        "closed_media_bag_load_cell_station_top_bag_lift_keepout_gauge",
        FRAME_X - 70.0,
        FRAME_Y - 70.0,
        TOP_BAG_LIFT_CLEARANCE,
    )
    .translate(
        HANGING_CENTER.0,
        HANGING_CENTER.1,
        BASE_Z + FRAME_Z + TOP_BAG_LIFT_CLEARANCE / 2.0,
    );

    front + rear + right + top_lift
}

fn tray_lip(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, 10.0, 22.0).translate(
        0.0,
        -y / 2.0 + 5.0,
        z + 11.0,
    );
    let rear = centered_cube(format!("{name}_rear"), x, 10.0, 22.0).translate(
        0.0,
        y / 2.0 - 5.0,
        z + 11.0,
    );
    let left = centered_cube(format!("{name}_left"), 10.0, y, 22.0).translate(
        -x / 2.0 + 5.0,
        0.0,
        z + 11.0,
    );
    let right = centered_cube(format!("{name}_right"), 10.0, y, 22.0).translate(
        x / 2.0 - 5.0,
        0.0,
        z + 11.0,
    );
    front + rear + left + right
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 6] {
    [
        (
            "hanging_frame_and_load_cell_bridge",
            HANGING_CENTER,
            THERMAL_SHIELD_X,
            THERMAL_SHIELD_Y,
        ),
        (
            "reference_mass_tray",
            MASS_TRAY_CENTER,
            MASS_TRAY_X,
            MASS_TRAY_Y,
        ),
        (
            "tubing_strain_relief",
            TUBING_CENTER,
            TUBING_RAIL_X,
            TUBING_RAIL_Y + 58.0,
        ),
        (
            "drain_prime_capture_trough",
            TROUGH_CENTER,
            TROUGH_X,
            TROUGH_Y,
        ),
        (
            "calibration_label_plate",
            LABEL_CENTER,
            LABEL_PLATE_X,
            LABEL_PLATE_Y,
        ),
        ("front_robot_access", (0.0, -350.0), 820.0, 54.0),
    ]
}

fn frame_corner_positions() -> [(f64, f64); 4] {
    [
        (
            -FRAME_X / 2.0 + FRAME_POST_W / 2.0,
            -FRAME_Y / 2.0 + FRAME_POST_W / 2.0,
        ),
        (
            FRAME_X / 2.0 - FRAME_POST_W / 2.0,
            -FRAME_Y / 2.0 + FRAME_POST_W / 2.0,
        ),
        (
            -FRAME_X / 2.0 + FRAME_POST_W / 2.0,
            FRAME_Y / 2.0 - FRAME_POST_W / 2.0,
        ),
        (
            FRAME_X / 2.0 - FRAME_POST_W / 2.0,
            FRAME_Y / 2.0 - FRAME_POST_W / 2.0,
        ),
    ]
}

fn load_cell_pad_positions() -> [(f64, f64); LOAD_CELL_COUNT] {
    [
        (-150.0, -100.0),
        (150.0, -100.0),
        (-150.0, 100.0),
        (150.0, 100.0),
    ]
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
    for (name, center, x, y) in module_specs() {
        assert!(
            fits_on_station(center, x, y),
            "{name} exceeds closed media-bag load-cell station footprint"
        );
    }
    assert_eq!(LOAD_CELL_COUNT, load_cell_pad_positions().len());
    assert!(
        HANGER_COUNT >= 3,
        "bag frame needs at least center and side hanger positions"
    );
    assert!(
        TUBE_CHANNEL_D > MEDIA_TUBE_OD_MAX,
        "tube channel must clear maximum media tubing OD"
    );
    assert!(
        MASS_WELL_D + 20.0 < MASS_WELL_PITCH_X,
        "reference mass wells do not leave enough tray material"
    );
    assert!(
        thermal_shield_air_gap_ok(),
        "thermal shield violates bag-frame air-gap assumptions"
    );
    assert!(
        TROUGH_X > BAG_CLEARANCE_X + 160.0,
        "capture trough must overhang hanging bag drip shadow"
    );
}

fn fits_on_station(center: (f64, f64), x: f64, y: f64) -> bool {
    let edge_margin = RIM_W + 32.0;
    center.0 - x / 2.0 > -STATION_X / 2.0 + edge_margin
        && center.0 + x / 2.0 < STATION_X / 2.0 - edge_margin
        && center.1 - y / 2.0 > -STATION_Y / 2.0 + edge_margin
        && center.1 + y / 2.0 < STATION_Y / 2.0 - edge_margin
}

fn thermal_shield_air_gap_ok() -> bool {
    THERMAL_SHIELD_X > FRAME_X + 2.0 * SHIELD_AIR_GAP
        && THERMAL_SHIELD_Y > FRAME_Y + 2.0 * SHIELD_AIR_GAP
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_feature_contract_is_explicit() {
        for feature in [
            "hanging_bag_frame",
            "reference_mass_tray",
            "load_cell_bridge",
            "thermal_shield",
            "tubing_strain_relief",
            "drain_prime_capture_trough",
            "calibration_label_plate",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn module_layout_fits_deck() {
        assert_layout();
    }

    #[test]
    fn output_paths_are_unique_and_station_scoped() {
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_media_bag_load_cell_drift_reference_station_"));
        }

        let mut sorted = OUTPUTS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), OUTPUTS.len());
    }

    #[test]
    fn reference_mass_inventory_has_coarse_and_fine_positions() {
        assert_eq!(REFERENCE_MASS_COUNT, 8);
        assert_eq!(FINE_MASS_COUNT, 6);
        assert!(REFERENCE_MASS_COUNT > CERTIFICATE_SLOT_COUNT);
        assert!(MASS_WELL_D + 20.0 < MASS_WELL_PITCH_X);
    }

    #[test]
    fn tubing_and_thermal_clearances_are_parametric() {
        assert!(TUBE_CHANNEL_D >= MEDIA_TUBE_OD_MAX + TUBE_CHANNEL_CLEARANCE);
        assert!(thermal_shield_air_gap_ok());
        assert!(TOP_BAG_LIFT_CLEARANCE > 0.35 * BAG_CLEARANCE_Z);
    }
}
