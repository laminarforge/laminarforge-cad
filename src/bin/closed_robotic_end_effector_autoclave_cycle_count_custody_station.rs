use std::{f64::consts::TAU, fs};

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic end-effector autoclave cycle-count custody station.
//
// Intent:
// - Validate clean/used custody and cycle history for removable robotic end
//   effectors or sterile tooling before they re-enter a closed culture module.
// - Combine clean/used tool nests, cycle-count token witness geometry,
//   autoclave pouch/case fit gauges, post-cycle inspection coupons,
//   barcode/RFID evidence lands, dry-time witness rails, release/quarantine
//   gates, and explicit robot/service keepouts in one fixture.
// - Keep the CAD focused on validation packaging. Autoclave recipe control,
//   biological release criteria, electronic records, and final material
//   selection are intentionally outside this standalone generator.

const OUTPUT_PREFIX: &str =
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_";

const OUTPUTS: &[&str] = &[
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_base_cleanable_tray.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_clean_used_tool_nests.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_cycle_count_token_wheel.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_autoclave_pouch_case_fit_gauge.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_post_cycle_inspection_coupons.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_barcode_rfid_custody_lands.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_dry_time_witness_rail.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_quarantine_release_gates.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_robot_alignment_datums.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_evidence_camera_bridge.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_robot_service_keepouts.stl",
    "output/closed_robotic_end_effector_autoclave_cycle_count_custody_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "clean_used_tool_nests",
    "cycle_count_token_wheel",
    "autoclave_pouch_case_fit_gauge",
    "post_cycle_inspection_coupons",
    "barcode_rfid_custody_lands",
    "dry_time_witness_rail",
    "quarantine_release_gates",
    "robot_alignment_datums",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1200.0;
const DECK_Y: f64 = 800.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const DRAIN_GUTTER_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;

const TOOL_COUNT: usize = 4;
const TOOL_ROWS: usize = 2;
const CLEAN_USED_TOOL_POSITIONS: usize = TOOL_COUNT * TOOL_ROWS;
const CLEAN_ROW_Y: f64 = 170.0;
const USED_ROW_Y: f64 = 38.0;
const TOOL_PITCH_X: f64 = 140.0;
const TOOL_START_X: f64 = -500.0;
const TOOL_NEST_X: f64 = 118.0;
const TOOL_NEST_Y: f64 = 86.0;
const TOOL_NEST_Z: f64 = 42.0;
const TOOL_CAVITY_X: f64 = 82.0;
const TOOL_CAVITY_Y: f64 = 46.0;
const TOOL_CAVITY_Z: f64 = 30.0;
const CLEAN_COVER_STANDOFF_Z: f64 = 28.0;
const USED_SHADOW_TRAY_Z: f64 = 18.0;
const CLEAN_USED_SEPARATION_GAP_MIN: f64 = 44.0;

const NEST_CENTER: (f64, f64) = (-290.0, 112.0);
const NEST_PANEL_X: f64 = 580.0;
const NEST_PANEL_Y: f64 = 320.0;
const NEST_PANEL_Z: f64 = 14.0;

const WHEEL_CENTER: (f64, f64) = (360.0, 248.0);
const WHEEL_PANEL_X: f64 = 330.0;
const WHEEL_PANEL_Y: f64 = 178.0;
const WHEEL_PANEL_Z: f64 = 13.0;
const CYCLE_TOKEN_COUNT: usize = 12;
const CYCLE_WHEEL_D: f64 = 118.0;
const CYCLE_TOKEN_D: f64 = 14.0;
const CYCLE_TICK_X: f64 = 6.0;
const CYCLE_TICK_Y: f64 = 22.0;
const CYCLE_MAX_COUNT: usize = 12;
const CYCLE_INDEX_WINDOW_X: f64 = 58.0;
const CYCLE_INDEX_WINDOW_Y: f64 = 20.0;

const GAUGE_CENTER: (f64, f64) = (360.0, 24.0);
const GAUGE_PANEL_X: f64 = 350.0;
const GAUGE_PANEL_Y: f64 = 190.0;
const GAUGE_PANEL_Z: f64 = 16.0;
const POUCH_GAUGE_X: f64 = 284.0;
const POUCH_GAUGE_Y: f64 = 118.0;
const POUCH_GAUGE_WALL: f64 = 9.0;
const CASE_GAUGE_X: f64 = 232.0;
const CASE_GAUGE_Y: f64 = 82.0;
const CASE_GAUGE_WALL: f64 = 8.0;
const POUCH_MARGIN_ALLOWANCE: f64 = 18.0;
const CASE_FIT_CLEARANCE: f64 = 12.0;

const COUPON_CENTER: (f64, f64) = (-325.0, -218.0);
const COUPON_PANEL_X: f64 = 510.0;
const COUPON_PANEL_Y: f64 = 132.0;
const COUPON_PANEL_Z: f64 = 12.0;
const COUPON_COLUMNS: usize = 6;
const COUPON_ROWS: usize = 2;
const COUPON_COUNT: usize = COUPON_COLUMNS * COUPON_ROWS;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 20.0;
const COUPON_PITCH_X: f64 = 72.0;
const COUPON_PITCH_Y: f64 = 56.0;
const MIN_INSPECTION_COUPONS: usize = 12;

const CUSTODY_CENTER: (f64, f64) = (-120.0, -338.0);
const CUSTODY_PANEL_X: f64 = 540.0;
const CUSTODY_PANEL_Y: f64 = 74.0;
const CUSTODY_PANEL_Z: f64 = 10.0;
const BARCODE_LANDS_PER_TOOL: usize = 2;
const BARCODE_LANDS: usize = TOOL_COUNT * BARCODE_LANDS_PER_TOOL;
const RFID_LANDS: usize = TOOL_COUNT;
const BARCODE_LAND_X: f64 = 76.0;
const BARCODE_LAND_Y: f64 = 18.0;
const RFID_LAND_D: f64 = 22.0;

const DRY_RAIL_CENTER: (f64, f64) = (260.0, -178.0);
const DRY_RAIL_X: f64 = 300.0;
const DRY_RAIL_Y: f64 = 120.0;
const DRY_RAIL_Z: f64 = 28.0;
const DRY_WITNESS_COUNT: usize = 6;
const DRY_WITNESS_PITCH: f64 = 42.0;
const DRY_WITNESS_TAB_X: f64 = 24.0;
const DRY_WITNESS_TAB_Y: f64 = 42.0;
const DRY_AIR_GAP: f64 = 18.0;

const GATE_CENTER: (f64, f64) = (390.0, -318.0);
const GATE_PANEL_X: f64 = 360.0;
const GATE_PANEL_Y: f64 = 126.0;
const GATE_PANEL_Z: f64 = 18.0;
const GATE_COUNT: usize = 3;
const GATE_SLOT_X: f64 = 74.0;
const GATE_SLOT_Y: f64 = 40.0;
const GATE_PITCH_X: f64 = 118.0;
const GATE_ARM_Z: f64 = 46.0;
const RELEASE_HOLD_QUARANTINE_GAP_MIN: f64 = 44.0;

const DATUM_PIN_D: f64 = 8.0;
const FIDUCIAL_D: f64 = 16.0;
const DATUM_COUNT: usize = 6;
const FIDUCIAL_COUNT: usize = 8;

const BRIDGE_CENTER: (f64, f64) = (-20.0, 58.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 48.0;
const BRIDGE_BEAM_Y: f64 = 60.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const BRIDGE_UNDERSIDE_Z: f64 = 178.0;
const BRIDGE_CAMERA_COUNT: usize = 4;
const CAMERA_POD_X: f64 = 82.0;
const CAMERA_POD_Y: f64 = 52.0;
const CAMERA_POD_Z: f64 = 42.0;

const ROBOT_APPROACH_Z_CLEARANCE: f64 = 240.0;
const FRONT_SERVICE_CLEARANCE: f64 = 330.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 170.0;
const REAR_AUTOCLAVE_CART_CLEARANCE: f64 = 150.0;
const KEEP_OUT_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_cleanable_tray();
    export(OUTPUTS[0], &base);

    let nests = clean_used_tool_nests();
    export(OUTPUTS[1], &nests);

    let wheel = cycle_count_token_wheel();
    export(OUTPUTS[2], &wheel);

    let gauge = autoclave_pouch_case_fit_gauge();
    export(OUTPUTS[3], &gauge);

    let coupons = post_cycle_inspection_coupons();
    export(OUTPUTS[4], &coupons);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[5], &custody);

    let dry_rail = dry_time_witness_rail();
    export(OUTPUTS[6], &dry_rail);

    let gates = quarantine_release_gates();
    export(OUTPUTS[7], &gates);

    let datums = robot_alignment_datums();
    export(OUTPUTS[8], &datums);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + nests
        + wheel
        + gauge
        + coupons
        + custody
        + dry_rail
        + gates
        + datums
        + bridge
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed robotic end-effector autoclave cycle-count custody station:");
    println!("  Deck envelope:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Tool custody:               {TOOL_COUNT} clean nests and {TOOL_COUNT} used nests with {:.0}mm clean/used gap",
        clean_used_gap()
    );
    println!(
        "  Autoclave cycle witness:    {CYCLE_TOKEN_COUNT}-position token wheel, max validated count {CYCLE_MAX_COUNT}"
    );
    println!(
        "  Fit gauge:                  pouch frame {:.0}mm x {:.0}mm, case frame {:.0}mm x {:.0}mm",
        POUCH_GAUGE_X, POUCH_GAUGE_Y, CASE_GAUGE_X, CASE_GAUGE_Y
    );
    println!(
        "  Inspection evidence:        {COUPON_COUNT} post-cycle coupons, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands"
    );
    println!(
        "  Dry-time witness:           {DRY_WITNESS_COUNT} witness tabs with {:.0}mm airflow gaps",
        DRY_AIR_GAP
    );
    println!(
        "  Disposition gates:          release/hold/quarantine gates with {:.0}mm lane gap",
        gate_lane_gap()
    );
    println!(
        "  Robot/service controls:     {DATUM_COUNT} datum pins, {FIDUCIAL_COUNT} fiducials, {:.0}mm approach clearance, front service {:.0}mm",
        ROBOT_APPROACH_Z_CLEARANCE, FRONT_SERVICE_CLEARANCE
    );
    println!("  Labeled STL outputs:        {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_cleanable_tray() -> Part {
    let deck = centered_cube(
        "autoclave_custody_base_cleanable_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck + perimeter_rim() - drain_gutters() - mount_holes() + module_location_lands()
}

fn perimeter_rim() -> Part {
    let front = centered_cube("autoclave_custody_front_wipeable_rim", DECK_X, RIM_W, RIM_Z)
        .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube("autoclave_custody_rear_wipeable_rim", DECK_X, RIM_W, RIM_Z)
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube("autoclave_custody_left_wipeable_rim", RIM_W, DECK_Y, RIM_Z)
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube("autoclave_custody_right_wipeable_rim", RIM_W, DECK_Y, RIM_Z)
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn drain_gutters() -> Part {
    let front_gutter = centered_cube(
        "autoclave_custody_front_condensate_wipe_gutter",
        DECK_X - 120.0,
        DRAIN_GUTTER_W,
        DECK_Z + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 54.0, DECK_Z / 2.0);
    let used_lane_gutter = centered_cube(
        "autoclave_custody_used_tool_condensate_gutter",
        NEST_PANEL_X - 52.0,
        DRAIN_GUTTER_W,
        DECK_Z + 2.0,
    )
    .translate(NEST_CENTER.0, USED_ROW_Y - 58.0, DECK_Z / 2.0);
    let drain_port = centered_cylinder(
        "autoclave_custody_condensate_drain_port_clearance",
        10.0,
        DECK_Z + 8.0,
        32,
    )
    .translate(DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 + 54.0, DECK_Z / 2.0);

    front_gutter + used_lane_gutter + drain_port
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("autoclave_custody_deck_mount_holes");
    for (index, (x, y)) in mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("autoclave_custody_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 8.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn module_location_lands() -> Part {
    let mut lands = Part::empty("autoclave_custody_module_location_lands");
    for (index, rect) in layout_rects().into_iter().enumerate() {
        let land = centered_cube(
            format!("autoclave_custody_module_shadow_land_{index}_{}", rect.name),
            rect.x,
            rect.y,
            3.0,
        )
        .translate(rect.cx, rect.cy, DECK_Z + 1.5);
        lands = lands + land;
    }
    lands
}

fn clean_used_tool_nests() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_clean_used_tool_nest_panel",
            NEST_PANEL_X,
            NEST_PANEL_Y,
            NEST_PANEL_Z,
        ),
        NEST_CENTER,
        NEST_PANEL_Z,
    );
    let clean_label_rail = centered_cube(
        "autoclave_custody_clean_tool_lane_label_rail",
        NEST_PANEL_X - 34.0,
        14.0,
        22.0,
    )
    .translate(
        NEST_CENTER.0,
        CLEAN_ROW_Y + 58.0,
        DECK_Z + NEST_PANEL_Z + 11.0,
    );
    let used_label_rail = centered_cube(
        "autoclave_custody_used_tool_lane_label_rail",
        NEST_PANEL_X - 34.0,
        14.0,
        22.0,
    )
    .translate(
        NEST_CENTER.0,
        USED_ROW_Y - 58.0,
        DECK_Z + NEST_PANEL_Z + 11.0,
    );
    let segregation_rib = centered_cube(
        "autoclave_custody_clean_used_lane_segregation_rib",
        NEST_PANEL_X - 44.0,
        12.0,
        58.0,
    )
    .translate(
        NEST_CENTER.0,
        midpoint(CLEAN_ROW_Y, USED_ROW_Y),
        DECK_Z + NEST_PANEL_Z + 29.0,
    );

    panel + clean_label_rail + used_label_rail + segregation_rib + tool_nest_rows()
}

fn tool_nest_rows() -> Part {
    let mut rows = Part::empty("autoclave_custody_clean_used_tool_nest_rows");
    for tool in 0..TOOL_COUNT {
        let x = tool_x(tool);
        rows = rows + clean_tool_nest(tool, x) + used_tool_nest(tool, x);
    }
    rows
}

fn clean_tool_nest(index: usize, x: f64) -> Part {
    let body = centered_cube(
        format!("autoclave_custody_clean_tool_nest_body_{index}"),
        TOOL_NEST_X,
        TOOL_NEST_Y,
        TOOL_NEST_Z,
    )
    .translate(x, CLEAN_ROW_Y, deck_top_z() + TOOL_NEST_Z / 2.0);
    let cavity = centered_cube(
        format!("autoclave_custody_clean_tool_nest_cavity_clearance_{index}"),
        TOOL_CAVITY_X,
        TOOL_CAVITY_Y,
        TOOL_CAVITY_Z,
    )
    .translate(
        x,
        CLEAN_ROW_Y,
        deck_top_z() + TOOL_NEST_Z - TOOL_CAVITY_Z / 2.0 + 1.0,
    );
    let drain = centered_cylinder(
        format!("autoclave_custody_clean_tool_nest_drain_{index}"),
        5.0,
        TOOL_NEST_Z + 3.0,
        24,
    )
    .translate(
        x + TOOL_NEST_X / 2.0 - 18.0,
        CLEAN_ROW_Y - 24.0,
        deck_top_z() + TOOL_NEST_Z / 2.0,
    );
    let cover_standoff = centered_cube(
        format!("autoclave_custody_clean_tool_cover_standoff_{index}"),
        TOOL_NEST_X - 28.0,
        12.0,
        CLEAN_COVER_STANDOFF_Z,
    )
    .translate(
        x,
        CLEAN_ROW_Y + TOOL_NEST_Y / 2.0 + 10.0,
        deck_top_z() + CLEAN_COVER_STANDOFF_Z / 2.0,
    );
    let orientation_key = centered_cylinder(
        format!("autoclave_custody_clean_tool_orientation_key_{index}"),
        7.0,
        18.0,
        28,
    )
    .translate(
        x - TOOL_NEST_X / 2.0 + 20.0,
        CLEAN_ROW_Y - TOOL_NEST_Y / 2.0 + 18.0,
        deck_top_z() + 9.0,
    );

    (body - cavity - drain) + cover_standoff + orientation_key
}

fn used_tool_nest(index: usize, x: f64) -> Part {
    let tray = centered_cube(
        format!("autoclave_custody_used_tool_shadow_tray_{index}"),
        TOOL_NEST_X,
        TOOL_NEST_Y,
        USED_SHADOW_TRAY_Z,
    )
    .translate(x, USED_ROW_Y, deck_top_z() + USED_SHADOW_TRAY_Z / 2.0);
    let shadow = centered_cube(
        format!("autoclave_custody_used_tool_residue_shadow_relief_{index}"),
        TOOL_CAVITY_X + 8.0,
        TOOL_CAVITY_Y + 8.0,
        USED_SHADOW_TRAY_Z + 2.0,
    )
    .translate(x, USED_ROW_Y, deck_top_z() + USED_SHADOW_TRAY_Z / 2.0 + 1.0);
    let swab_land = centered_cube(
        format!("autoclave_custody_used_tool_swab_land_{index}"),
        TOOL_NEST_X - 26.0,
        14.0,
        9.0,
    )
    .translate(x, USED_ROW_Y - TOOL_NEST_Y / 2.0 - 11.0, deck_top_z() + 4.5);
    let used_flag_socket = centered_cylinder(
        format!("autoclave_custody_used_tool_flag_socket_{index}"),
        7.0,
        20.0,
        28,
    )
    .translate(
        x + TOOL_NEST_X / 2.0 - 18.0,
        USED_ROW_Y + TOOL_NEST_Y / 2.0 - 18.0,
        deck_top_z() + 10.0,
    );

    (tray - shadow) + swab_land + used_flag_socket
}

fn cycle_count_token_wheel() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_cycle_count_token_wheel_panel",
            WHEEL_PANEL_X,
            WHEEL_PANEL_Y,
            WHEEL_PANEL_Z,
        ),
        WHEEL_CENTER,
        WHEEL_PANEL_Z,
    );
    let wheel = centered_cylinder(
        "autoclave_custody_cycle_count_token_wheel_disc",
        CYCLE_WHEEL_D / 2.0,
        12.0,
        72,
    )
    .translate(
        WHEEL_CENTER.0 - 72.0,
        WHEEL_CENTER.1,
        deck_top_z() + WHEEL_PANEL_Z + 6.0,
    );
    let axle = centered_cylinder(
        "autoclave_custody_cycle_count_axle_clearance",
        8.0,
        16.0,
        32,
    )
    .translate(
        WHEEL_CENTER.0 - 72.0,
        WHEEL_CENTER.1,
        deck_top_z() + WHEEL_PANEL_Z + 6.0,
    );
    let wheel_with_tokens = (wheel - axle) + cycle_token_pockets() + cycle_tick_marks();
    let index_window = centered_cube(
        "autoclave_custody_cycle_count_current_index_window",
        CYCLE_INDEX_WINDOW_X,
        CYCLE_INDEX_WINDOW_Y,
        20.0,
    )
    .translate(
        WHEEL_CENTER.0 + 72.0,
        WHEEL_CENTER.1 + 38.0,
        deck_top_z() + WHEEL_PANEL_Z + 10.0,
    );
    let pawl = centered_cube(
        "autoclave_custody_cycle_count_increment_pawl",
        82.0,
        12.0,
        22.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(
        WHEEL_CENTER.0 + 62.0,
        WHEEL_CENTER.1 - 42.0,
        deck_top_z() + WHEEL_PANEL_Z + 11.0,
    );
    let max_count_stop = centered_cube(
        "autoclave_custody_cycle_count_maximum_stop_block",
        18.0,
        62.0,
        26.0,
    )
    .translate(
        WHEEL_CENTER.0 + WHEEL_PANEL_X / 2.0 - 36.0,
        WHEEL_CENTER.1,
        deck_top_z() + WHEEL_PANEL_Z + 13.0,
    );

    panel + wheel_with_tokens + index_window + pawl + max_count_stop
}

fn cycle_token_pockets() -> Part {
    let mut tokens = Part::empty("autoclave_custody_cycle_count_token_pockets");
    let center_x = WHEEL_CENTER.0 - 72.0;
    let center_y = WHEEL_CENTER.1;
    let radius = CYCLE_WHEEL_D / 2.0 - 18.0;
    for index in 0..CYCLE_TOKEN_COUNT {
        let angle = token_angle(index);
        let token = centered_cylinder(
            format!("autoclave_custody_cycle_token_land_{index:02}"),
            CYCLE_TOKEN_D / 2.0,
            6.0,
            24,
        )
        .translate(
            center_x + radius * angle.cos(),
            center_y + radius * angle.sin(),
            deck_top_z() + WHEEL_PANEL_Z + 15.0,
        );
        tokens = tokens + token;
    }
    tokens
}

fn cycle_tick_marks() -> Part {
    let mut ticks = Part::empty("autoclave_custody_cycle_count_tick_marks");
    let center_x = WHEEL_CENTER.0 - 72.0;
    let center_y = WHEEL_CENTER.1;
    let radius = CYCLE_WHEEL_D / 2.0 + 8.0;
    for index in 0..CYCLE_TOKEN_COUNT {
        let angle = token_angle(index);
        let tick = centered_cube(
            format!("autoclave_custody_cycle_tick_mark_{index:02}"),
            CYCLE_TICK_X,
            CYCLE_TICK_Y,
            8.0,
        )
        .rotate(0.0, 0.0, angle.to_degrees())
        .translate(
            center_x + radius * angle.cos(),
            center_y + radius * angle.sin(),
            deck_top_z() + WHEEL_PANEL_Z + 4.0,
        );
        ticks = ticks + tick;
    }
    ticks
}

fn autoclave_pouch_case_fit_gauge() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_pouch_case_fit_gauge_panel",
            GAUGE_PANEL_X,
            GAUGE_PANEL_Y,
            GAUGE_PANEL_Z,
        ),
        GAUGE_CENTER,
        GAUGE_PANEL_Z,
    );
    let pouch_frame = rectangular_frame(
        "autoclave_custody_autoclave_pouch_fit_frame",
        POUCH_GAUGE_X,
        POUCH_GAUGE_Y,
        18.0,
        POUCH_GAUGE_WALL,
    )
    .translate(
        GAUGE_CENTER.0 - 24.0,
        GAUGE_CENTER.1 + 16.0,
        deck_top_z() + GAUGE_PANEL_Z + 9.0,
    );
    let case_frame = rectangular_frame(
        "autoclave_custody_rigid_case_fit_frame",
        CASE_GAUGE_X,
        CASE_GAUGE_Y,
        16.0,
        CASE_GAUGE_WALL,
    )
    .translate(
        GAUGE_CENTER.0 - 24.0,
        GAUGE_CENTER.1 - 10.0,
        deck_top_z() + GAUGE_PANEL_Z + 24.0,
    );
    let pass_slot = centered_cube(
        "autoclave_custody_pouch_case_pass_width_slot",
        78.0,
        16.0,
        24.0,
    )
    .translate(
        GAUGE_CENTER.0 + GAUGE_PANEL_X / 2.0 - 56.0,
        GAUGE_CENTER.1 + 48.0,
        deck_top_z() + GAUGE_PANEL_Z + 12.0,
    );
    let fail_step = centered_cube(
        "autoclave_custody_pouch_case_fail_overpack_step",
        46.0,
        58.0,
        32.0,
    )
    .translate(
        GAUGE_CENTER.0 + GAUGE_PANEL_X / 2.0 - 58.0,
        GAUGE_CENTER.1 - 38.0,
        deck_top_z() + GAUGE_PANEL_Z + 16.0,
    );
    let seam_relief_gauge = centered_cube(
        "autoclave_custody_pouch_seam_relief_allowance_bar",
        POUCH_GAUGE_X - 42.0,
        10.0,
        10.0,
    )
    .translate(
        GAUGE_CENTER.0 - 24.0,
        GAUGE_CENTER.1 + POUCH_GAUGE_Y / 2.0 - 10.0,
        deck_top_z() + GAUGE_PANEL_Z + 27.0,
    );

    panel + pouch_frame + case_frame + pass_slot + fail_step + seam_relief_gauge
}

fn post_cycle_inspection_coupons() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_post_cycle_coupon_panel",
            COUPON_PANEL_X,
            COUPON_PANEL_Y,
            COUPON_PANEL_Z,
        ),
        COUPON_CENTER,
        COUPON_PANEL_Z,
    );
    let mut pockets = Part::empty("autoclave_custody_post_cycle_coupon_pockets");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLUMNS {
            let index = row * COUPON_COLUMNS + col;
            let x = COUPON_CENTER.0 + lane_offset(col, COUPON_COLUMNS, COUPON_PITCH_X);
            let y = COUPON_CENTER.1 + lane_offset(row, COUPON_ROWS, COUPON_PITCH_Y);
            let pocket = rectangular_frame(
                format!("autoclave_custody_post_cycle_inspection_coupon_pocket_{index:02}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                10.0,
                4.0,
            )
            .translate(x, y, deck_top_z() + COUPON_PANEL_Z + 5.0);
            let swab_dot = centered_cylinder(
                format!("autoclave_custody_post_cycle_swab_witness_dot_{index:02}"),
                4.0,
                7.0,
                20,
            )
            .translate(
                x + COUPON_SLOT_X / 2.0 + 10.0,
                y,
                deck_top_z() + COUPON_PANEL_Z + 3.5,
            );
            pockets = pockets + pocket + swab_dot;
        }
    }
    let indicator_strip = centered_cube(
        "autoclave_custody_chemical_indicator_strip_lane",
        COUPON_PANEL_X - 54.0,
        12.0,
        9.0,
    )
    .translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1 - COUPON_PANEL_Y / 2.0 + 18.0,
        deck_top_z() + COUPON_PANEL_Z + 4.5,
    );

    panel + pockets + indicator_strip
}

fn barcode_rfid_custody_lands() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_barcode_rfid_custody_land_panel",
            CUSTODY_PANEL_X,
            CUSTODY_PANEL_Y,
            CUSTODY_PANEL_Z,
        ),
        CUSTODY_CENTER,
        CUSTODY_PANEL_Z,
    );
    let mut lands = Part::empty("autoclave_custody_barcode_rfid_lands");
    for tool in 0..TOOL_COUNT {
        let x = CUSTODY_CENTER.0 + lane_offset(tool, TOOL_COUNT, 116.0);
        let clean_barcode = centered_cube(
            format!("autoclave_custody_clean_barcode_land_tool_{tool}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            6.0,
        )
        .translate(
            x,
            CUSTODY_CENTER.1 + 18.0,
            deck_top_z() + CUSTODY_PANEL_Z + 3.0,
        );
        let used_barcode = centered_cube(
            format!("autoclave_custody_used_barcode_land_tool_{tool}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            6.0,
        )
        .translate(
            x,
            CUSTODY_CENTER.1 - 18.0,
            deck_top_z() + CUSTODY_PANEL_Z + 3.0,
        );
        let rfid = centered_cylinder(
            format!("autoclave_custody_rfid_land_tool_{tool}"),
            RFID_LAND_D / 2.0,
            8.0,
            32,
        )
        .translate(
            x + 48.0,
            CUSTODY_CENTER.1,
            deck_top_z() + CUSTODY_PANEL_Z + 4.0,
        );
        lands = lands + clean_barcode + used_barcode + rfid;
    }
    let chain_card_rail = centered_cube(
        "autoclave_custody_chain_of_custody_card_rail",
        CUSTODY_PANEL_X - 28.0,
        8.0,
        14.0,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1 + CUSTODY_PANEL_Y / 2.0 + 8.0,
        deck_top_z() + 7.0,
    );

    panel + lands + chain_card_rail
}

fn dry_time_witness_rail() -> Part {
    let rail = place_on_deck(
        centered_cube(
            "autoclave_custody_dry_time_witness_rail_body",
            DRY_RAIL_X,
            DRY_RAIL_Y,
            DRY_RAIL_Z,
        ),
        DRY_RAIL_CENTER,
        DRY_RAIL_Z,
    );
    let mut witnesses = Part::empty("autoclave_custody_dry_time_witness_tabs");
    for index in 0..DRY_WITNESS_COUNT {
        let x = DRY_RAIL_CENTER.0 + lane_offset(index, DRY_WITNESS_COUNT, DRY_WITNESS_PITCH);
        let tab = centered_cube(
            format!("autoclave_custody_dry_time_witness_tab_{index:02}"),
            DRY_WITNESS_TAB_X,
            DRY_WITNESS_TAB_Y,
            22.0,
        )
        .translate(x, DRY_RAIL_CENTER.1 + 8.0, deck_top_z() + DRY_RAIL_Z + 11.0);
        let gap = centered_cube(
            format!("autoclave_custody_dry_time_air_gap_clearance_{index:02}"),
            DRY_AIR_GAP,
            DRY_RAIL_Y + 4.0,
            DRY_RAIL_Z + 4.0,
        )
        .translate(
            x + DRY_WITNESS_TAB_X / 2.0 + DRY_AIR_GAP / 2.0,
            DRY_RAIL_CENTER.1,
            deck_top_z() + DRY_RAIL_Z / 2.0,
        );
        witnesses = witnesses + tab - gap;
    }
    let time_token_tray = centered_cube(
        "autoclave_custody_dry_time_release_token_tray",
        DRY_RAIL_X - 42.0,
        18.0,
        10.0,
    )
    .translate(
        DRY_RAIL_CENTER.0,
        DRY_RAIL_CENTER.1 - DRY_RAIL_Y / 2.0 + 16.0,
        deck_top_z() + DRY_RAIL_Z + 5.0,
    );

    rail + witnesses + time_token_tray
}

fn quarantine_release_gates() -> Part {
    let panel = place_on_deck(
        centered_cube(
            "autoclave_custody_release_hold_quarantine_gate_panel",
            GATE_PANEL_X,
            GATE_PANEL_Y,
            GATE_PANEL_Z,
        ),
        GATE_CENTER,
        GATE_PANEL_Z,
    );
    let statuses = ["release", "hold", "quarantine"];
    let mut gates = Part::empty("autoclave_custody_release_hold_quarantine_gates");
    for (index, status) in statuses.iter().enumerate() {
        let x = GATE_CENTER.0 + lane_offset(index, GATE_COUNT, GATE_PITCH_X);
        let slot = rectangular_frame(
            format!("autoclave_custody_{status}_status_token_gate"),
            GATE_SLOT_X,
            GATE_SLOT_Y,
            24.0,
            7.0,
        )
        .translate(x, GATE_CENTER.1, deck_top_z() + GATE_PANEL_Z + 12.0);
        let arm = centered_cube(
            format!("autoclave_custody_{status}_gate_arm"),
            GATE_SLOT_X + 20.0,
            10.0,
            GATE_ARM_Z,
        )
        .translate(
            x,
            GATE_CENTER.1 + GATE_SLOT_Y / 2.0 + 14.0,
            deck_top_z() + GATE_ARM_Z / 2.0,
        );
        let lock_pin = centered_cylinder(
            format!("autoclave_custody_{status}_gate_lock_pin_bore"),
            5.0,
            30.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            GATE_CENTER.1 - GATE_SLOT_Y / 2.0 - 14.0,
            deck_top_z() + GATE_PANEL_Z + 15.0,
        );
        gates = gates + slot + arm - lock_pin;
    }
    let quarantine_tall_wall = centered_cube(
        "autoclave_custody_quarantine_gate_tall_redline_wall",
        82.0,
        12.0,
        76.0,
    )
    .translate(
        GATE_CENTER.0 + lane_offset(2, GATE_COUNT, GATE_PITCH_X),
        GATE_CENTER.1 - GATE_PANEL_Y / 2.0 + 16.0,
        deck_top_z() + 38.0,
    );

    panel + gates + quarantine_tall_wall
}

fn robot_alignment_datums() -> Part {
    let mut datums = Part::empty("autoclave_custody_robot_alignment_datums");
    for (index, (x, y)) in datum_points().into_iter().enumerate() {
        let pad = centered_cylinder(
            format!("autoclave_custody_robot_datum_pad_{index}"),
            18.0,
            10.0,
            36,
        )
        .translate(x, y, deck_top_z() + 5.0);
        let pin = centered_cylinder(
            format!("autoclave_custody_robot_datum_pin_clearance_{index}"),
            DATUM_PIN_D / 2.0,
            14.0,
            24,
        )
        .translate(x, y, deck_top_z() + 5.0);
        datums = datums + (pad - pin);
    }
    datums + robot_fiducials()
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("autoclave_custody_robot_vision_fiducials");
    for (index, (x, y)) in fiducial_points().into_iter().enumerate() {
        let outer = centered_cylinder(
            format!("autoclave_custody_robot_fiducial_outer_{index}"),
            FIDUCIAL_D / 2.0,
            4.0,
            30,
        )
        .translate(x, y, deck_top_z() + 2.0);
        let center = centered_cylinder(
            format!("autoclave_custody_robot_fiducial_center_relief_{index}"),
            2.8,
            5.0,
            20,
        )
        .translate(x, y, deck_top_z() + 2.0);
        fiducials = fiducials + (outer - center);
    }
    fiducials
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "autoclave_custody_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "autoclave_custody_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "autoclave_custody_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let mut pods = Part::empty("autoclave_custody_evidence_bridge_camera_pods");
    for index in 0..BRIDGE_CAMERA_COUNT {
        let x = BRIDGE_CENTER.0 + lane_offset(index, BRIDGE_CAMERA_COUNT, 230.0);
        let pod = centered_cube(
            format!("autoclave_custody_evidence_camera_pod_{index}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            deck_top_z() + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("autoclave_custody_evidence_camera_lens_clearance_{index}"),
            9.0,
            CAMERA_POD_Z + 4.0,
            28,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            deck_top_z() + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
        );
        pods = pods + (pod - lens);
    }

    left_post + right_post + beam + pods
}

fn robot_service_keepouts() -> Part {
    let robot_approach = centered_cube(
        "autoclave_custody_robot_wrist_approach_keepout",
        DECK_X - 180.0,
        DECK_Y - 160.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, ROBOT_APPROACH_Z_CLEARANCE);
    let front_service = centered_cube(
        "autoclave_custody_front_tool_exchange_service_keepout",
        DECK_X - 130.0,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_SERVICE_CLEARANCE,
        deck_top_z() + 11.0,
    );
    let right_service = centered_cube(
        "autoclave_custody_right_autoclave_case_service_keepout",
        18.0,
        DECK_Y - 120.0,
        22.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_SERVICE_CLEARANCE,
        0.0,
        deck_top_z() + 11.0,
    );
    let rear_cart = centered_cube(
        "autoclave_custody_rear_autoclave_cart_keepout",
        DECK_X - 180.0,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_AUTOCLAVE_CART_CLEARANCE,
        deck_top_z() + 11.0,
    );

    robot_approach + front_service + right_service + rear_cart
}

fn place_on_deck(part: Part, center: (f64, f64), height: f64) -> Part {
    part.translate(center.0, center.1, deck_top_z() + height / 2.0)
}

fn rectangular_frame(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    wall: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

fn lane_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn midpoint(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn token_angle(index: usize) -> f64 {
    index as f64 * TAU / CYCLE_TOKEN_COUNT as f64
}

fn tool_x(index: usize) -> f64 {
    TOOL_START_X + index as f64 * TOOL_PITCH_X
}

fn deck_top_z() -> f64 {
    DECK_Z
}

fn clean_used_gap() -> f64 {
    (CLEAN_ROW_Y - USED_ROW_Y).abs() - TOOL_NEST_Y
}

fn gate_lane_gap() -> f64 {
    GATE_PITCH_X - GATE_SLOT_X
}

fn gauge_fits_pouch_and_case_allowances() -> bool {
    POUCH_GAUGE_X > CASE_GAUGE_X + 2.0 * POUCH_MARGIN_ALLOWANCE
        && POUCH_GAUGE_Y > CASE_GAUGE_Y + CASE_FIT_CLEARANCE
}

fn station_fits_closed_workcell() -> bool {
    const WORKCELL_DECK_X: f64 = 1320.0;
    const WORKCELL_DECK_Y: f64 = 900.0;

    DECK_X <= WORKCELL_DECK_X && DECK_Y <= WORKCELL_DECK_Y
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 44.0, -DECK_Y / 2.0 + 44.0),
        (DECK_X / 2.0 - 44.0, -DECK_Y / 2.0 + 44.0),
        (-DECK_X / 2.0 + 44.0, DECK_Y / 2.0 - 44.0),
        (DECK_X / 2.0 - 44.0, DECK_Y / 2.0 - 44.0),
        (NEST_CENTER.0 - NEST_PANEL_X / 2.0 + 36.0, NEST_CENTER.1),
        (NEST_CENTER.0 + NEST_PANEL_X / 2.0 - 36.0, NEST_CENTER.1),
        (GATE_CENTER.0, GATE_CENTER.1),
        (GAUGE_CENTER.0, GAUGE_CENTER.1 + GAUGE_PANEL_Y / 2.0 - 26.0),
    ]
}

fn datum_points() -> [(f64, f64); DATUM_COUNT] {
    [
        (TOOL_START_X - 34.0, CLEAN_ROW_Y + 72.0),
        (
            TOOL_START_X + (TOOL_COUNT as f64 - 1.0) * TOOL_PITCH_X + 34.0,
            CLEAN_ROW_Y + 72.0,
        ),
        (TOOL_START_X - 34.0, USED_ROW_Y - 72.0),
        (
            TOOL_START_X + (TOOL_COUNT as f64 - 1.0) * TOOL_PITCH_X + 34.0,
            USED_ROW_Y - 72.0,
        ),
        (
            GAUGE_CENTER.0 + GAUGE_PANEL_X / 2.0 - 34.0,
            GAUGE_CENTER.1 + GAUGE_PANEL_Y / 2.0 - 34.0,
        ),
        (
            GATE_CENTER.0 + GATE_PANEL_X / 2.0 - 34.0,
            GATE_CENTER.1 - GATE_PANEL_Y / 2.0 + 34.0,
        ),
    ]
}

fn fiducial_points() -> [(f64, f64); FIDUCIAL_COUNT] {
    [
        (
            NEST_CENTER.0 - NEST_PANEL_X / 2.0 + 30.0,
            NEST_CENTER.1 - NEST_PANEL_Y / 2.0 + 30.0,
        ),
        (
            NEST_CENTER.0 + NEST_PANEL_X / 2.0 - 30.0,
            NEST_CENTER.1 + NEST_PANEL_Y / 2.0 - 30.0,
        ),
        (
            WHEEL_CENTER.0 - WHEEL_PANEL_X / 2.0 + 28.0,
            WHEEL_CENTER.1 + WHEEL_PANEL_Y / 2.0 - 28.0,
        ),
        (
            WHEEL_CENTER.0 + WHEEL_PANEL_X / 2.0 - 28.0,
            WHEEL_CENTER.1 - WHEEL_PANEL_Y / 2.0 + 28.0,
        ),
        (
            GAUGE_CENTER.0 - GAUGE_PANEL_X / 2.0 + 30.0,
            GAUGE_CENTER.1 - GAUGE_PANEL_Y / 2.0 + 30.0,
        ),
        (
            COUPON_CENTER.0 - COUPON_PANEL_X / 2.0 + 30.0,
            COUPON_CENTER.1 + COUPON_PANEL_Y / 2.0 - 26.0,
        ),
        (
            DRY_RAIL_CENTER.0 + DRY_RAIL_X / 2.0 - 26.0,
            DRY_RAIL_CENTER.1 - DRY_RAIL_Y / 2.0 + 26.0,
        ),
        (
            GATE_CENTER.0 + GATE_PANEL_X / 2.0 - 26.0,
            GATE_CENTER.1 + GATE_PANEL_Y / 2.0 - 26.0,
        ),
    ]
}

fn layout_rects() -> [Rect; 7] {
    [
        nest_rect(),
        wheel_rect(),
        gauge_rect(),
        coupon_rect(),
        custody_rect(),
        dry_rail_rect(),
        gate_rect(),
    ]
}

fn nest_rect() -> Rect {
    Rect::new(
        "clean_used_tool_nests",
        NEST_CENTER.0,
        NEST_CENTER.1,
        NEST_PANEL_X,
        NEST_PANEL_Y,
    )
}

fn wheel_rect() -> Rect {
    Rect::new(
        "cycle_count_token_wheel",
        WHEEL_CENTER.0,
        WHEEL_CENTER.1,
        WHEEL_PANEL_X,
        WHEEL_PANEL_Y,
    )
}

fn gauge_rect() -> Rect {
    Rect::new(
        "autoclave_pouch_case_fit_gauge",
        GAUGE_CENTER.0,
        GAUGE_CENTER.1,
        GAUGE_PANEL_X,
        GAUGE_PANEL_Y,
    )
}

fn coupon_rect() -> Rect {
    Rect::new(
        "post_cycle_inspection_coupons",
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
    )
}

fn custody_rect() -> Rect {
    Rect::new(
        "barcode_rfid_custody_lands",
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
    )
}

fn dry_rail_rect() -> Rect {
    Rect::new(
        "dry_time_witness_rail",
        DRY_RAIL_CENTER.0,
        DRY_RAIL_CENTER.1,
        DRY_RAIL_X,
        DRY_RAIL_Y,
    )
}

fn gate_rect() -> Rect {
    Rect::new(
        "quarantine_release_gates",
        GATE_CENTER.0,
        GATE_CENTER.1,
        GATE_PANEL_X,
        GATE_PANEL_Y,
    )
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 9);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert!(
        station_fits_closed_workcell(),
        "station exceeds closed workcell deck allowance"
    );
    assert_eq!(
        CLEAN_USED_TOOL_POSITIONS,
        TOOL_COUNT * TOOL_ROWS,
        "clean/used custody positions must cover every tool row"
    );
    assert!(
        clean_used_gap() >= CLEAN_USED_SEPARATION_GAP_MIN,
        "clean and used tool nests are too close"
    );
    assert_eq!(
        CYCLE_TOKEN_COUNT, CYCLE_MAX_COUNT,
        "token wheel must expose the validated max cycle count"
    );
    assert!(
        gauge_fits_pouch_and_case_allowances(),
        "autoclave pouch/case fit gauge lacks clearance allowance"
    );
    assert!(COUPON_COUNT >= MIN_INSPECTION_COUPONS);
    assert_eq!(BARCODE_LANDS, TOOL_COUNT * BARCODE_LANDS_PER_TOOL);
    assert_eq!(RFID_LANDS, TOOL_COUNT);
    assert!(gate_lane_gap() >= RELEASE_HOLD_QUARANTINE_GAP_MIN);
    assert!(BRIDGE_UNDERSIDE_Z > TOOL_NEST_Z + CLEAN_COVER_STANDOFF_Z + DECK_Z + 64.0);
    assert!(ROBOT_APPROACH_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 20.0);

    for rect in layout_rects() {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds deck footprint",
            rect.name
        );
    }

    assert!(!nest_rect().overlaps(wheel_rect()));
    assert!(!nest_rect().overlaps(coupon_rect()));
    assert!(!wheel_rect().overlaps(gauge_rect()));
    assert!(!gauge_rect().overlaps(dry_rail_rect()));
    assert!(!coupon_rect().overlaps(custody_rect()));
    assert!(!dry_rail_rect().overlaps(gate_rect()));
    assert!(!custody_rect().overlaps(gate_rect()));
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    cx: f64,
    cy: f64,
    x: f64,
    y: f64,
}

impl Rect {
    const fn new(name: &'static str, cx: f64, cy: f64, x: f64, y: f64) -> Self {
        Self { name, cx, cy, x, y }
    }

    fn left(self) -> f64 {
        self.cx - self.x / 2.0
    }

    fn right(self) -> f64 {
        self.cx + self.x / 2.0
    }

    fn bottom(self) -> f64 {
        self.cy - self.y / 2.0
    }

    fn top(self) -> f64 {
        self.cy + self.y / 2.0
    }

    fn fits_inside(self, max_x: f64, max_y: f64) -> bool {
        self.left() >= -max_x / 2.0 + RIM_W
            && self.right() <= max_x / 2.0 - RIM_W
            && self.bottom() >= -max_y / 2.0 + RIM_W
            && self.top() <= max_y / 2.0 - RIM_W
    }

    fn overlaps(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_plan_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_workflow_features_are_declared() {
        assert!(REQUIRED_FEATURES.contains(&"clean_used_tool_nests"));
        assert!(REQUIRED_FEATURES.contains(&"cycle_count_token_wheel"));
        assert!(REQUIRED_FEATURES.contains(&"autoclave_pouch_case_fit_gauge"));
        assert!(REQUIRED_FEATURES.contains(&"post_cycle_inspection_coupons"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"dry_time_witness_rail"));
        assert!(REQUIRED_FEATURES.contains(&"quarantine_release_gates"));
        assert!(REQUIRED_FEATURES.contains(&"robot_alignment_datums"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn dimensions_and_layout_fit_closed_workcell() {
        assert_layout();
        assert!(station_fits_closed_workcell());
        for rect in layout_rects() {
            assert!(rect.fits_inside(DECK_X, DECK_Y));
        }
    }

    #[test]
    fn custody_evidence_covers_every_tool() {
        assert_eq!(CLEAN_USED_TOOL_POSITIONS, 8);
        assert_eq!(BARCODE_LANDS, TOOL_COUNT * BARCODE_LANDS_PER_TOOL);
        assert_eq!(RFID_LANDS, TOOL_COUNT);
        assert!(COUPON_COUNT >= MIN_INSPECTION_COUPONS);
        assert_eq!(DATUM_COUNT, datum_points().len());
        assert_eq!(FIDUCIAL_COUNT, fiducial_points().len());
    }

    #[test]
    fn autoclave_and_dry_time_controls_have_clearance() {
        assert!(gauge_fits_pouch_and_case_allowances());
        assert_eq!(CYCLE_TOKEN_COUNT, CYCLE_MAX_COUNT);
        assert!(clean_used_gap() >= CLEAN_USED_SEPARATION_GAP_MIN);
        assert_eq!(DRY_WITNESS_COUNT, 6);
        assert!(gate_lane_gap() >= RELEASE_HOLD_QUARANTINE_GAP_MIN);
        assert!(ROBOT_APPROACH_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
    }
}
