use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator rack slot identity, load history, and recovery-custody station.
//
// This standalone generator models a no-cell engineering validation fixture for
// reproducible cassette position tracking inside a closed incubator workflow. It
// captures mechanical identity, custody, and recovery-history features only:
// barcode/RFID lands, load/unload token lanes, logger pockets, edge/center
// witness blocks, sealed dummy cassette nests, mismatch quarantine, tamper-seal
// lands, evidence imaging, clean/used segregation, and robot/service keepouts.
// It intentionally omits biological protocol settings and acceptance thresholds.

const OUTPUT_PREFIX: &str = "closed_incubator_rack_slot_identity_load_history_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_rack_slot_identity_load_history_station_base_deck.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_six_slot_rack_surrogate.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_slot_identity_barcode_rfid_lands.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_load_unload_token_lanes.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_thermal_recovery_logger_pockets.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_edge_center_position_witness_blocks.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_sealed_cassette_dummy_nests.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_mismatch_quarantine_pocket.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_tamper_seal_lands.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_evidence_camera_bridge.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_clean_used_segregation_features.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_robot_service_keepouts.stl",
    "output/closed_incubator_rack_slot_identity_load_history_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "six_slot_rack_surrogate",
    "slot_identity_barcode_rfid_lands",
    "load_unload_token_lanes",
    "thermal_recovery_logger_pockets",
    "edge_center_position_witness_blocks",
    "sealed_cassette_dummy_nests",
    "mismatch_quarantine_pocket",
    "tamper_seal_lands",
    "evidence_camera_bridge",
    "clean_used_segregation_features",
    "robot_service_keepouts",
    "assembly",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const REAR_RIM_Z: f64 = 44.0;
const FRONT_LIP_Z: f64 = 18.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_COUNT: usize = 8;

const RACK_CENTER: (f64, f64) = (-300.0, 110.0);
const RACK_X: f64 = 620.0;
const RACK_Y: f64 = 350.0;
const RACK_Z: f64 = 42.0;
const RACK_COLS: usize = 3;
const RACK_ROWS: usize = 2;
const SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const SLOT_PITCH_X: f64 = 178.0;
const SLOT_PITCH_Y: f64 = 138.0;
const SLOT_X: f64 = 142.0;
const SLOT_Y: f64 = 104.0;
const SLOT_RELIEF_Z: f64 = 9.0;
const RACK_AIR_BYPASS_COUNT: usize = 10;

const CASSETTE_X: f64 = 128.0;
const CASSETTE_Y: f64 = 92.0;
const CASSETTE_Z: f64 = 28.0;
const CASSETTE_LID_Z: f64 = 7.0;
const CASSETTE_HANDLE_X: f64 = 52.0;

const ID_CENTER: (f64, f64) = (-300.0, -138.0);
const ID_PANEL_X: f64 = 620.0;
const ID_PANEL_Y: f64 = 112.0;
const ID_PANEL_Z: f64 = 14.0;
const BARCODE_LAND_X: f64 = 92.0;
const BARCODE_LAND_Y: f64 = 20.0;
const RFID_LAND_D: f64 = 32.0;
const CERTIFICATE_LAND_COUNT: usize = 3;

const TOKEN_CENTER: (f64, f64) = (340.0, 190.0);
const TOKEN_PANEL_X: f64 = 430.0;
const TOKEN_PANEL_Y: f64 = 166.0;
const TOKEN_PANEL_Z: f64 = 22.0;
const TOKEN_LANES: usize = 2;
const TOKENS_PER_LANE: usize = 6;
const TOKEN_COUNT: usize = TOKEN_LANES * TOKENS_PER_LANE;
const TOKEN_D: f64 = 26.0;
const TOKEN_PITCH_X: f64 = 54.0;
const TOKEN_PITCH_Y: f64 = 66.0;

const LOGGER_CENTER: (f64, f64) = (346.0, 18.0);
const LOGGER_PANEL_X: f64 = 430.0;
const LOGGER_PANEL_Y: f64 = 144.0;
const LOGGER_PANEL_Z: f64 = 26.0;
const LOGGER_POCKET_COUNT: usize = 4;
const LOGGER_POCKET_X: f64 = 76.0;
const LOGGER_POCKET_Y: f64 = 46.0;
const LOGGER_POCKET_DEPTH: f64 = 11.0;

const WITNESS_CENTER: (f64, f64) = (-300.0, 324.0);
const WITNESS_PANEL_X: f64 = 620.0;
const WITNESS_PANEL_Y: f64 = 84.0;
const WITNESS_PANEL_Z: f64 = 18.0;
const EDGE_WITNESS_COUNT: usize = 8;
const CENTER_WITNESS_COUNT: usize = 2;

const QUARANTINE_CENTER: (f64, f64) = (346.0, -208.0);
const QUARANTINE_X: f64 = 430.0;
const QUARANTINE_Y: f64 = 178.0;
const QUARANTINE_Z: f64 = 50.0;
const QUARANTINE_BIN_COUNT: usize = 2;
const QUARANTINE_POCKET_X: f64 = 150.0;
const QUARANTINE_POCKET_Y: f64 = 104.0;
const QUARANTINE_WALL_Z: f64 = 72.0;

const TAMPER_CENTER: (f64, f64) = (-300.0, -282.0);
const TAMPER_PANEL_X: f64 = 620.0;
const TAMPER_PANEL_Y: f64 = 78.0;
const TAMPER_PANEL_Z: f64 = 12.0;
const TAMPER_SEAL_COUNT: usize = SLOT_COUNT;
const TAMPER_LAND_X: f64 = 72.0;
const TAMPER_LAND_Y: f64 = 28.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 350.0);
const BRIDGE_SPAN_X: f64 = 1110.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 48.0;
const BRIDGE_POST_Z: f64 = 198.0;
const BRIDGE_BEAM_Y: f64 = 60.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const CAMERA_POD_X: f64 = 88.0;
const CAMERA_POD_Y: f64 = 52.0;
const CAMERA_POD_Z: f64 = 42.0;
const LIGHT_PIPE_COUNT: usize = 7;

const SEGREGATION_CENTER: (f64, f64) = (22.0, -34.0);
const SEGREGATION_PANEL_X: f64 = 60.0;
const SEGREGATION_PANEL_Y: f64 = 620.0;
const SEGREGATION_PANEL_Z: f64 = 20.0;
const CLEAN_ZONE_LAND_COUNT: usize = 4;
const USED_ZONE_LAND_COUNT: usize = 4;

const KEEP_OUT_Z: f64 = 5.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 96.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 48.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 80.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let rack = six_slot_rack_surrogate();
    export(OUTPUTS[1], &rack);

    let identity = slot_identity_barcode_rfid_lands();
    export(OUTPUTS[2], &identity);

    let tokens = load_unload_token_lanes();
    export(OUTPUTS[3], &tokens);

    let loggers = thermal_recovery_logger_pockets();
    export(OUTPUTS[4], &loggers);

    let witnesses = edge_center_position_witness_blocks();
    export(OUTPUTS[5], &witnesses);

    let dummies = sealed_cassette_dummy_nests();
    export(OUTPUTS[6], &dummies);

    let quarantine = mismatch_quarantine_pocket();
    export(OUTPUTS[7], &quarantine);

    let tamper = tamper_seal_lands();
    export(OUTPUTS[8], &tamper);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[9], &bridge);

    let segregation = clean_used_segregation_features();
    export(OUTPUTS[10], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator rack slot identity/load history station:");
    println!(
        "  Base deck:               {STATION_X:.0}mm x {STATION_Y:.0}mm closed validation tray"
    );
    println!(
        "  Rack surrogate:          {SLOT_COUNT} cassette slots in a {RACK_COLS}x{RACK_ROWS} layout with {RACK_AIR_BYPASS_COUNT} airflow witness bypasses"
    );
    println!(
        "  Identity/custody:        {SLOT_COUNT} barcode lands, {SLOT_COUNT} RFID lands, {TOKEN_COUNT} load/unload tokens, {TAMPER_SEAL_COUNT} tamper seal lands"
    );
    println!(
        "  Recovery history:        {LOGGER_POCKET_COUNT} thermal logger pockets, {EDGE_WITNESS_COUNT} edge witness blocks, {CENTER_WITNESS_COUNT} center witness blocks"
    );
    println!(
        "  Quarantine/evidence:     {QUARANTINE_BIN_COUNT} mismatch bins, {EVIDENCE_CAMERA_COUNT} evidence camera pods, {LIGHT_PIPE_COUNT} light-pipe witnesses"
    );
    println!(
        "  Segregation/clearance:   {} clean/used lands, {:.0}mm front robot clearance, {:.0}mm rear service clearance",
        CLEAN_ZONE_LAND_COUNT + USED_ZONE_LAND_COUNT,
        front_robot_clearance(),
        rear_service_clearance()
    );
    println!("  STL outputs:             {}", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_deck()
        + six_slot_rack_surrogate()
        + slot_identity_barcode_rfid_lands()
        + load_unload_token_lanes()
        + thermal_recovery_logger_pockets()
        + edge_center_position_witness_blocks()
        + sealed_cassette_dummy_nests()
        + mismatch_quarantine_pocket()
        + tamper_seal_lands()
        + evidence_camera_bridge()
        + clean_used_segregation_features()
        + robot_service_keepouts()
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "slot_identity_load_history_base_deck_plate",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "slot_identity_load_history_shallow_cleaning_basin_cut",
        STATION_X - 126.0,
        STATION_Y - 122.0,
        7.0,
    )
    .translate(0.0, -18.0, BASE_Z - 3.2);
    let drain = centered_cylinder(
        "slot_identity_load_history_front_right_wipe_drain_bore",
        5.8,
        52.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 7.0);

    deck - basin - drain - deck_mount_holes() - module_sockets()
        + deck_rims()
        + deck_zone_lines()
        + base_datum_targets()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("slot_identity_load_history_deck_mount_holes");
    for (index, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("slot_identity_load_history_m6_mount_hole_{index}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 56.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 56.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 56.0),
        (0.0, -(STATION_Y / 2.0 - 56.0)),
        (0.0, STATION_Y / 2.0 - 56.0),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("slot_identity_load_history_module_locator_sockets");
    for (name, center, x, y) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("slot_identity_load_history_{name}_locator_socket"),
                x + 8.0,
                y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(center.0, center.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.3);
    }
    sockets
}

fn deck_rims() -> Part {
    let rear = centered_cube(
        "slot_identity_load_history_rear_service_rim",
        STATION_X,
        RIM_W,
        REAR_RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + REAR_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "slot_identity_load_history_left_closed_workflow_rim",
        RIM_W,
        STATION_Y - 74.0,
        REAR_RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        -12.0,
        BASE_Z + REAR_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "slot_identity_load_history_right_service_rim",
        RIM_W,
        STATION_Y - 150.0,
        32.0,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, -44.0, BASE_Z + 16.0);
    let front = centered_cube(
        "slot_identity_load_history_front_low_robot_lip",
        STATION_X - 250.0,
        12.0,
        FRONT_LIP_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z + FRONT_LIP_Z / 2.0);

    rear + left + right + front
}

fn deck_zone_lines() -> Part {
    let history_line = centered_cube(
        "slot_identity_load_history_recovery_history_row_divider",
        STATION_X - 170.0,
        8.0,
        18.0,
    )
    .translate(0.0, 264.0, BASE_Z + 9.0);
    let token_line = centered_cube(
        "slot_identity_load_history_identity_token_row_divider",
        STATION_X - 170.0,
        8.0,
        18.0,
    )
    .translate(0.0, -80.0, BASE_Z + 9.0);
    let quarantine_line = centered_cube(
        "slot_identity_load_history_quarantine_row_divider",
        STATION_X - 210.0,
        8.0,
        18.0,
    )
    .translate(20.0, -246.0, BASE_Z + 9.0);
    let clean_used_spine = centered_cube(
        "slot_identity_load_history_clean_used_center_spine",
        10.0,
        640.0,
        20.0,
    )
    .translate(SEGREGATION_CENTER.0, SEGREGATION_CENTER.1, BASE_Z + 10.0);

    history_line + token_line + quarantine_line + clean_used_spine
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("slot_identity_load_history_base_datum_targets");
    for (index, (x, y)) in [
        (-548.0, 348.0),
        (548.0, 348.0),
        (-548.0, -348.0),
        (548.0, -348.0),
    ]
    .into_iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("slot_identity_load_history_robot_datum_pad_{index}"),
            13.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z + 2.5);
        let bore = centered_cylinder(
            format!("slot_identity_load_history_robot_datum_center_bore_{index}"),
            3.0,
            6.0,
            24,
        )
        .translate(x, y, BASE_Z + 3.0);
        targets = targets + (pad - bore);
    }
    targets
}

fn six_slot_rack_surrogate() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_six_slot_rack_surrogate_body",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let rack = panel - rack_slot_reliefs() - rack_air_bypass_slots() - rack_finger_access_cuts()
        + rack_reference_rails()
        + rack_slot_stops()
        + rack_slot_number_tiles();
    place_on_deck(rack, RACK_CENTER, RACK_Z)
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("slot_identity_load_history_rack_slot_reliefs");
    for slot in 0..SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        reliefs = reliefs
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_locator_recess",
                    slot_label(slot)
                ),
                SLOT_X,
                SLOT_Y,
                SLOT_RELIEF_Z,
            )
            .translate(x, y, RACK_Z / 2.0 - SLOT_RELIEF_Z / 2.0 + 0.2);
    }
    reliefs
}

fn rack_air_bypass_slots() -> Part {
    let mut bypass = Part::empty("slot_identity_load_history_rack_air_bypass_slots");
    for index in 0..RACK_AIR_BYPASS_COUNT {
        let col = index % 5;
        let row = index / 5;
        bypass = bypass
            + centered_cube(
                format!("slot_identity_load_history_air_bypass_witness_slot_{index}"),
                82.0,
                8.0,
                RACK_Z + 3.0,
            )
            .translate(
                centered_index(col, 5, 110.0),
                centered_index(row, 2, 214.0),
                0.0,
            );
    }
    bypass
}

fn rack_finger_access_cuts() -> Part {
    let mut cuts = Part::empty("slot_identity_load_history_rack_finger_access_cuts");
    for slot in 0..SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        cuts = cuts
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_front_robot_finger_cut",
                    slot_label(slot)
                ),
                58.0,
                20.0,
                RACK_Z + 3.0,
            )
            .translate(x, y - SLOT_Y / 2.0 - 14.0, 0.0);
    }
    cuts
}

fn rack_reference_rails() -> Part {
    let rear = centered_cube(
        "slot_identity_load_history_rack_rear_hard_datum_rail",
        RACK_X,
        16.0,
        30.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 18.0, RACK_Z / 2.0 + 15.0);
    let left = centered_cube(
        "slot_identity_load_history_rack_left_hard_datum_rail",
        16.0,
        RACK_Y,
        30.0,
    )
    .translate(-RACK_X / 2.0 + 18.0, 0.0, RACK_Z / 2.0 + 15.0);
    let right_soft = centered_cube(
        "slot_identity_load_history_rack_right_soft_capture_rail",
        12.0,
        RACK_Y - 82.0,
        18.0,
    )
    .translate(RACK_X / 2.0 - 24.0, -18.0, RACK_Z / 2.0 + 9.0);
    let front_low = centered_cube(
        "slot_identity_load_history_rack_front_low_robot_access_rail",
        RACK_X - 150.0,
        10.0,
        14.0,
    )
    .translate(26.0, -RACK_Y / 2.0 + 18.0, RACK_Z / 2.0 + 7.0);

    rear + left + right_soft + front_low
}

fn rack_slot_stops() -> Part {
    let mut stops = Part::empty("slot_identity_load_history_rack_slot_hard_stops");
    for slot in 0..SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        stops = stops
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_rear_hard_stop",
                    slot_label(slot)
                ),
                SLOT_X * 0.52,
                7.0,
                14.0,
            )
            .translate(x, y + SLOT_Y / 2.0 + 12.0, RACK_Z / 2.0 + 7.0)
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_left_hard_stop",
                    slot_label(slot)
                ),
                7.0,
                SLOT_Y * 0.46,
                14.0,
            )
            .translate(x - SLOT_X / 2.0 - 12.0, y, RACK_Z / 2.0 + 7.0);
    }
    stops
}

fn rack_slot_number_tiles() -> Part {
    let mut tiles = Part::empty("slot_identity_load_history_rack_slot_number_tiles");
    for slot in 0..SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        tiles = tiles
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_raised_number_tile",
                    slot_label(slot)
                ),
                48.0,
                18.0,
                4.0,
            )
            .translate(x, y + SLOT_Y / 2.0 + 32.0, RACK_Z / 2.0 + 2.0);
    }
    tiles
}

fn slot_identity_barcode_rfid_lands() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_identity_panel",
        ID_PANEL_X,
        ID_PANEL_Y,
        ID_PANEL_Z,
    );
    let lands = barcode_lands() + rfid_lands() + identity_certificate_lands();
    place_on_deck(panel + lands, ID_CENTER, ID_PANEL_Z)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("slot_identity_load_history_slot_barcode_lands");
    for slot in 0..SLOT_COUNT {
        let x = centered_index(slot % RACK_COLS, RACK_COLS, SLOT_PITCH_X);
        let y = centered_index(slot / RACK_COLS, RACK_ROWS, 48.0);
        lands = lands
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_barcode_land",
                    slot_label(slot)
                ),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x - 24.0, y, ID_PANEL_Z / 2.0 + 2.0);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("slot_identity_load_history_slot_rfid_lands");
    for slot in 0..SLOT_COUNT {
        let x = centered_index(slot % RACK_COLS, RACK_COLS, SLOT_PITCH_X);
        let y = centered_index(slot / RACK_COLS, RACK_ROWS, 48.0);
        let tag = centered_cylinder(
            format!(
                "slot_identity_load_history_slot_{}_rfid_tag_land",
                slot_label(slot)
            ),
            RFID_LAND_D / 2.0,
            4.0,
            36,
        )
        .translate(x + 54.0, y, ID_PANEL_Z / 2.0 + 2.0);
        let center_bore = centered_cylinder(
            format!(
                "slot_identity_load_history_slot_{}_rfid_center_witness",
                slot_label(slot)
            ),
            3.0,
            5.0,
            20,
        )
        .translate(x + 54.0, y, ID_PANEL_Z / 2.0 + 2.5);
        lands = lands + (tag - center_bore);
    }
    lands
}

fn identity_certificate_lands() -> Part {
    let mut lands = Part::empty("slot_identity_load_history_certificate_lands");
    for index in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("slot_identity_load_history_identity_certificate_land_{index}"),
                128.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(index, CERTIFICATE_LAND_COUNT, 152.0),
                -ID_PANEL_Y / 2.0 + 17.0,
                ID_PANEL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn load_unload_token_lanes() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_load_unload_token_lane_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    );
    let lane_divider = centered_cube(
        "slot_identity_load_history_load_unload_center_lane_divider",
        TOKEN_PANEL_X - 46.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, TOKEN_PANEL_Z / 2.0 + 9.0);
    let tokens = token_recesses() + token_retainer_posts() + token_lane_labels();

    place_on_deck(
        panel - token_recesses() + lane_divider + tokens,
        TOKEN_CENTER,
        TOKEN_PANEL_Z,
    )
}

fn token_recesses() -> Part {
    let mut recesses = Part::empty("slot_identity_load_history_token_recesses");
    for index in 0..TOKEN_COUNT {
        let lane = index / TOKENS_PER_LANE;
        let token = index % TOKENS_PER_LANE;
        recesses = recesses
            + centered_cylinder(
                format!("slot_identity_load_history_token_{index:02}_round_recess"),
                TOKEN_D / 2.0,
                7.0,
                30,
            )
            .translate(
                centered_index(token, TOKENS_PER_LANE, TOKEN_PITCH_X),
                centered_index(lane, TOKEN_LANES, TOKEN_PITCH_Y),
                TOKEN_PANEL_Z / 2.0 - 3.0,
            );
    }
    recesses
}

fn token_retainer_posts() -> Part {
    let mut posts = Part::empty("slot_identity_load_history_token_retainer_posts");
    for index in 0..TOKEN_COUNT {
        let lane = index / TOKENS_PER_LANE;
        let token = index % TOKENS_PER_LANE;
        let x = centered_index(token, TOKENS_PER_LANE, TOKEN_PITCH_X);
        let y = centered_index(lane, TOKEN_LANES, TOKEN_PITCH_Y);
        posts = posts
            + centered_cylinder(
                format!("slot_identity_load_history_token_{index:02}_left_retainer"),
                3.0,
                8.0,
                18,
            )
            .translate(x - TOKEN_D / 2.0 - 8.0, y, TOKEN_PANEL_Z / 2.0 + 4.0)
            + centered_cylinder(
                format!("slot_identity_load_history_token_{index:02}_right_retainer"),
                3.0,
                8.0,
                18,
            )
            .translate(x + TOKEN_D / 2.0 + 8.0, y, TOKEN_PANEL_Z / 2.0 + 4.0);
    }
    posts
}

fn token_lane_labels() -> Part {
    let load = centered_cube(
        "slot_identity_load_history_load_token_lane_label_land",
        78.0,
        24.0,
        4.0,
    )
    .translate(
        -TOKEN_PANEL_X / 2.0 + 54.0,
        TOKEN_PITCH_Y / 2.0,
        TOKEN_PANEL_Z / 2.0 + 2.0,
    );
    let unload = centered_cube(
        "slot_identity_load_history_unload_token_lane_label_land",
        78.0,
        24.0,
        4.0,
    )
    .translate(
        -TOKEN_PANEL_X / 2.0 + 54.0,
        -TOKEN_PITCH_Y / 2.0,
        TOKEN_PANEL_Z / 2.0 + 2.0,
    );
    load + unload
}

fn thermal_recovery_logger_pockets() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_thermal_recovery_logger_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let pockets = logger_pocket_cuts();
    let datums = logger_datum_bosses() + logger_cable_clip_lands();

    place_on_deck(panel - pockets + datums, LOGGER_CENTER, LOGGER_PANEL_Z)
}

fn logger_pocket_cuts() -> Part {
    let mut pockets = Part::empty("slot_identity_load_history_logger_pocket_cuts");
    for index in 0..LOGGER_POCKET_COUNT {
        pockets = pockets
            + centered_cube(
                format!("slot_identity_load_history_thermal_logger_pocket_{index}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH,
            )
            .translate(
                centered_index(index, LOGGER_POCKET_COUNT, 96.0),
                0.0,
                LOGGER_PANEL_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0 + 0.3,
            );
    }
    pockets
}

fn logger_datum_bosses() -> Part {
    let mut bosses = Part::empty("slot_identity_load_history_logger_datum_bosses");
    for index in 0..LOGGER_POCKET_COUNT {
        let x = centered_index(index, LOGGER_POCKET_COUNT, 96.0);
        bosses = bosses
            + centered_cylinder(
                format!("slot_identity_load_history_logger_{index}_datum_boss"),
                5.5,
                7.0,
                24,
            )
            .translate(
                x - LOGGER_POCKET_X / 2.0 + 10.0,
                LOGGER_POCKET_Y / 2.0 + 14.0,
                LOGGER_PANEL_Z / 2.0 + 3.5,
            );
    }
    bosses
}

fn logger_cable_clip_lands() -> Part {
    let mut clips = Part::empty("slot_identity_load_history_logger_cable_clip_lands");
    for index in 0..LOGGER_POCKET_COUNT {
        clips = clips
            + centered_cube(
                format!("slot_identity_load_history_logger_{index}_cable_clip_land"),
                42.0,
                10.0,
                8.0,
            )
            .translate(
                centered_index(index, LOGGER_POCKET_COUNT, 96.0),
                -LOGGER_POCKET_Y / 2.0 - 16.0,
                LOGGER_PANEL_Z / 2.0 + 4.0,
            );
    }
    clips
}

fn edge_center_position_witness_blocks() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_edge_center_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let edge = edge_witness_blocks();
    let center = center_witness_blocks();

    place_on_deck(panel + edge + center, WITNESS_CENTER, WITNESS_PANEL_Z)
}

fn edge_witness_blocks() -> Part {
    let mut blocks = Part::empty("slot_identity_load_history_edge_position_witness_blocks");
    for index in 0..EDGE_WITNESS_COUNT {
        let side = index / 4;
        let pos = index % 4;
        blocks = blocks
            + centered_cube(
                format!("slot_identity_load_history_edge_witness_block_{index}"),
                46.0,
                24.0,
                22.0,
            )
            .translate(
                centered_index(pos, 4, 126.0),
                centered_index(side, 2, 46.0),
                WITNESS_PANEL_Z / 2.0 + 11.0,
            );
    }
    blocks
}

fn center_witness_blocks() -> Part {
    let mut blocks = Part::empty("slot_identity_load_history_center_position_witness_blocks");
    for index in 0..CENTER_WITNESS_COUNT {
        blocks = blocks
            + centered_cylinder(
                format!("slot_identity_load_history_center_witness_tower_{index}"),
                18.0,
                26.0,
                36,
            )
            .translate(
                centered_index(index, CENTER_WITNESS_COUNT, 72.0),
                0.0,
                WITNESS_PANEL_Z / 2.0 + 13.0,
            );
    }
    blocks
}

fn sealed_cassette_dummy_nests() -> Part {
    let mut dummies = Part::empty("slot_identity_load_history_sealed_cassette_dummy_nests");
    for slot in 0..SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        let cassette = centered_cube(
            format!(
                "slot_identity_load_history_slot_{}_sealed_dummy_cassette",
                slot_label(slot)
            ),
            CASSETTE_X,
            CASSETTE_Y,
            CASSETTE_Z,
        );
        let lid_land = centered_cube(
            format!(
                "slot_identity_load_history_slot_{}_sealed_lid_witness",
                slot_label(slot)
            ),
            CASSETTE_X - 22.0,
            CASSETTE_Y - 18.0,
            CASSETTE_LID_Z,
        )
        .translate(0.0, 0.0, CASSETTE_Z / 2.0 + CASSETTE_LID_Z / 2.0);
        let handle = centered_cube(
            format!(
                "slot_identity_load_history_slot_{}_dummy_gripper_handle",
                slot_label(slot)
            ),
            CASSETTE_HANDLE_X,
            12.0,
            16.0,
        )
        .translate(0.0, -CASSETTE_Y / 2.0 - 10.0, CASSETTE_Z / 2.0 + 8.0);
        let seal_dot = centered_cylinder(
            format!(
                "slot_identity_load_history_slot_{}_sealed_dummy_orientation_dot",
                slot_label(slot)
            ),
            5.0,
            5.0,
            24,
        )
        .translate(
            -CASSETTE_X / 2.0 + 16.0,
            CASSETTE_Y / 2.0 - 16.0,
            CASSETTE_Z / 2.0 + CASSETTE_LID_Z + 2.5,
        );
        dummies = dummies
            + (cassette + lid_land + handle + seal_dot).translate(
                RACK_CENTER.0 + x,
                RACK_CENTER.1 + y,
                BASE_Z + RACK_Z + CASSETTE_Z / 2.0 + 1.0,
            );
    }
    dummies
}

fn mismatch_quarantine_pocket() -> Part {
    let tray = centered_cube(
        "slot_identity_load_history_mismatch_quarantine_pocket_body",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let pockets = quarantine_pocket_cuts();
    let walls = quarantine_high_walls() + quarantine_lockout_tab_lands();

    place_on_deck(tray - pockets + walls, QUARANTINE_CENTER, QUARANTINE_Z)
}

fn quarantine_pocket_cuts() -> Part {
    let mut pockets = Part::empty("slot_identity_load_history_quarantine_pocket_cuts");
    for index in 0..QUARANTINE_BIN_COUNT {
        pockets = pockets
            + centered_cube(
                format!("slot_identity_load_history_quarantine_bin_{index}_deep_pocket"),
                QUARANTINE_POCKET_X,
                QUARANTINE_POCKET_Y,
                28.0,
            )
            .translate(
                centered_index(index, QUARANTINE_BIN_COUNT, 184.0),
                0.0,
                QUARANTINE_Z / 2.0 - 14.0 + 0.3,
            );
    }
    pockets
}

fn quarantine_high_walls() -> Part {
    let rear = centered_cube(
        "slot_identity_load_history_quarantine_rear_high_wall",
        QUARANTINE_X,
        12.0,
        QUARANTINE_WALL_Z,
    )
    .translate(
        0.0,
        QUARANTINE_Y / 2.0 - 6.0,
        QUARANTINE_Z / 2.0 + QUARANTINE_WALL_Z / 2.0,
    );
    let center = centered_cube(
        "slot_identity_load_history_quarantine_center_divider_wall",
        12.0,
        QUARANTINE_Y - 26.0,
        QUARANTINE_WALL_Z - 14.0,
    )
    .translate(
        0.0,
        0.0,
        QUARANTINE_Z / 2.0 + (QUARANTINE_WALL_Z - 14.0) / 2.0,
    );
    let left = centered_cube(
        "slot_identity_load_history_quarantine_left_retention_wall",
        12.0,
        QUARANTINE_Y,
        QUARANTINE_WALL_Z - 6.0,
    )
    .translate(
        -QUARANTINE_X / 2.0 + 6.0,
        0.0,
        QUARANTINE_Z / 2.0 + (QUARANTINE_WALL_Z - 6.0) / 2.0,
    );
    let right = centered_cube(
        "slot_identity_load_history_quarantine_right_retention_wall",
        12.0,
        QUARANTINE_Y,
        QUARANTINE_WALL_Z - 6.0,
    )
    .translate(
        QUARANTINE_X / 2.0 - 6.0,
        0.0,
        QUARANTINE_Z / 2.0 + (QUARANTINE_WALL_Z - 6.0) / 2.0,
    );

    rear + center + left + right
}

fn quarantine_lockout_tab_lands() -> Part {
    let mut lands = Part::empty("slot_identity_load_history_quarantine_lockout_tab_lands");
    for index in 0..QUARANTINE_BIN_COUNT {
        lands = lands
            + centered_cube(
                format!("slot_identity_load_history_quarantine_bin_{index}_lockout_tag_land"),
                116.0,
                22.0,
                6.0,
            )
            .translate(
                centered_index(index, QUARANTINE_BIN_COUNT, 184.0),
                -QUARANTINE_Y / 2.0 + 20.0,
                QUARANTINE_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn tamper_seal_lands() -> Part {
    let panel = centered_cube(
        "slot_identity_load_history_tamper_seal_panel",
        TAMPER_PANEL_X,
        TAMPER_PANEL_Y,
        TAMPER_PANEL_Z,
    );
    let mut lands = Part::empty("slot_identity_load_history_tamper_seal_lands");
    for index in 0..TAMPER_SEAL_COUNT {
        lands = lands
            + centered_cube(
                format!(
                    "slot_identity_load_history_slot_{}_tamper_seal_land",
                    slot_label(index)
                ),
                TAMPER_LAND_X,
                TAMPER_LAND_Y,
                5.0,
            )
            .translate(
                centered_index(index, TAMPER_SEAL_COUNT, 92.0),
                0.0,
                TAMPER_PANEL_Z / 2.0 + 2.5,
            )
            + centered_cylinder(
                format!(
                    "slot_identity_load_history_slot_{}_seal_wire_bore",
                    slot_label(index)
                ),
                2.2,
                TAMPER_LAND_Y + 8.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, TAMPER_SEAL_COUNT, 92.0),
                0.0,
                TAMPER_PANEL_Z / 2.0 + 3.0,
            );
    }

    place_on_deck(panel + lands, TAMPER_CENTER, TAMPER_PANEL_Z)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "slot_identity_load_history_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "slot_identity_load_history_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "slot_identity_load_history_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);
    let cameras = evidence_camera_pods();
    let lights = evidence_light_pipes();

    (left_post + right_post + beam + cameras + lights).translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        BASE_Z,
    )
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("slot_identity_load_history_evidence_camera_pods");
    for index in 0..EVIDENCE_CAMERA_COUNT {
        let pod = centered_cube(
            format!("slot_identity_load_history_evidence_camera_pod_{index}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            centered_index(index, EVIDENCE_CAMERA_COUNT, 248.0),
            0.0,
            BRIDGE_POST_Z - CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("slot_identity_load_history_evidence_camera_lens_clearance_{index}"),
            12.0,
            10.0,
            32,
        )
        .translate(
            centered_index(index, EVIDENCE_CAMERA_COUNT, 248.0),
            0.0,
            BRIDGE_POST_Z - CAMERA_POD_Z - 3.0,
        );
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_light_pipes() -> Part {
    let mut pipes = Part::empty("slot_identity_load_history_evidence_light_pipes");
    for index in 0..LIGHT_PIPE_COUNT {
        pipes = pipes
            + centered_cylinder(
                format!("slot_identity_load_history_evidence_light_pipe_{index}"),
                5.0,
                18.0,
                20,
            )
            .translate(
                centered_index(index, LIGHT_PIPE_COUNT, 142.0),
                BRIDGE_BEAM_Y / 2.0 + 6.0,
                BRIDGE_POST_Z - 12.0,
            );
    }
    pipes
}

fn clean_used_segregation_features() -> Part {
    let spine = centered_cube(
        "slot_identity_load_history_clean_used_segregation_raised_spine",
        SEGREGATION_PANEL_X,
        SEGREGATION_PANEL_Y,
        SEGREGATION_PANEL_Z,
    );
    let clean = zone_lands("clean", -104.0, CLEAN_ZONE_LAND_COUNT);
    let used = zone_lands("used", 104.0, USED_ZONE_LAND_COUNT);
    let arrow = centered_cube(
        "slot_identity_load_history_one_way_clean_to_used_arrow_land",
        34.0,
        118.0,
        6.0,
    )
    .translate(0.0, -36.0, SEGREGATION_PANEL_Z / 2.0 + 3.0);

    place_on_deck(
        spine + clean + used + arrow,
        SEGREGATION_CENTER,
        SEGREGATION_PANEL_Z,
    )
}

fn zone_lands(zone: &str, x: f64, count: usize) -> Part {
    let mut lands = Part::empty(format!("slot_identity_load_history_{zone}_zone_lands"));
    for index in 0..count {
        lands = lands
            + centered_cube(
                format!("slot_identity_load_history_{zone}_zone_custody_land_{index}"),
                72.0,
                34.0,
                5.0,
            )
            .translate(
                x,
                centered_index(index, count, 126.0),
                SEGREGATION_PANEL_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "slot_identity_load_history_front_robot_approach_keepout_gauge",
        STATION_X - 220.0,
        FRONT_ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y / 2.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "slot_identity_load_history_rear_service_sweep_keepout_gauge",
        STATION_X - 180.0,
        REAR_SERVICE_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y / 2.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let left = centered_cube(
        "slot_identity_load_history_left_robot_gripper_keepout_gauge",
        SIDE_SERVICE_CLEARANCE_X,
        STATION_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE_X / 2.0,
        -12.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "slot_identity_load_history_right_service_door_keepout_gauge",
        SIDE_SERVICE_CLEARANCE_X,
        STATION_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - SIDE_SERVICE_CLEARANCE_X / 2.0,
        -12.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );

    front + rear + left + right
}

fn rack_slot_center(slot: usize) -> (f64, f64) {
    let col = slot % RACK_COLS;
    let row = slot / RACK_COLS;
    (
        centered_index(col, RACK_COLS, SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, SLOT_PITCH_Y),
    )
}

fn slot_label(slot: usize) -> String {
    format!("r{}c{}", slot / RACK_COLS + 1, slot % RACK_COLS + 1)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn place_on_deck(part: Part, center: (f64, f64), part_z: f64) -> Part {
    part.translate(center.0, center.1, BASE_Z + part_z / 2.0)
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        ("rack_surrogate", RACK_CENTER, RACK_X, RACK_Y),
        ("identity_lands", ID_CENTER, ID_PANEL_X, ID_PANEL_Y),
        ("token_lanes", TOKEN_CENTER, TOKEN_PANEL_X, TOKEN_PANEL_Y),
        (
            "logger_pockets",
            LOGGER_CENTER,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        ),
        (
            "witness_blocks",
            WITNESS_CENTER,
            WITNESS_PANEL_X,
            WITNESS_PANEL_Y,
        ),
        (
            "quarantine_pocket",
            QUARANTINE_CENTER,
            QUARANTINE_X,
            QUARANTINE_Y,
        ),
        (
            "tamper_seal_lands",
            TAMPER_CENTER,
            TAMPER_PANEL_X,
            TAMPER_PANEL_Y,
        ),
        (
            "clean_used_segregation",
            SEGREGATION_CENTER,
            SEGREGATION_PANEL_X,
            SEGREGATION_PANEL_Y,
        ),
    ]
}

fn fits_inside_rim(center: (f64, f64), x: f64, y: f64) -> bool {
    center.0.abs() + x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && center.1.abs() + y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (TAMPER_CENTER.1.abs() + TAMPER_PANEL_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0)
}

fn clean_used_land_count() -> usize {
    CLEAN_ZONE_LAND_COUNT + USED_ZONE_LAND_COUNT
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(SLOT_COUNT, 6);
    assert_eq!(TOKEN_COUNT, 12);
    assert_eq!(TAMPER_SEAL_COUNT, SLOT_COUNT);
    assert_eq!(LOGGER_POCKET_COUNT, 4);
    assert_eq!(EDGE_WITNESS_COUNT + CENTER_WITNESS_COUNT, 10);
    assert_eq!(QUARANTINE_BIN_COUNT, 2);
    assert_eq!(clean_used_land_count(), 8);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE_Y);
    assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE_Y);
    assert!(SIDE_SERVICE_CLEARANCE_X >= 70.0);
    assert!(BRIDGE_SPAN_X <= STATION_X - 2.0 * SIDE_SERVICE_CLEARANCE_X);

    for (name, center, x, y) in module_specs() {
        assert!(fits_inside_rim(center, x, y), "{name} exceeds usable deck");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_complete_and_deterministic() {
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(
            OUTPUTS[12],
            "output/closed_incubator_rack_slot_identity_load_history_station_assembly.stl"
        );
        for output in OUTPUTS {
            assert!(output.starts_with("output/"));
            assert!(output.ends_with(".stl"));
            assert!(output.contains(OUTPUT_PREFIX));
        }

        let mut sorted = OUTPUTS.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), OUTPUTS.len());
    }

    #[test]
    fn required_features_have_named_outputs() {
        let joined = OUTPUTS.join("\n");
        for feature in REQUIRED_FEATURES {
            assert!(
                joined.contains(feature),
                "missing required feature output for {feature}"
            );
        }
    }

    #[test]
    fn modules_fit_inside_base_rim() {
        for (name, center, x, y) in module_specs() {
            assert!(
                fits_inside_rim(center, x, y),
                "{name} exceeds station bounds"
            );
        }
        assert!(BRIDGE_CENTER.0.abs() + BRIDGE_SPAN_X / 2.0 <= STATION_X / 2.0 - 42.0);
        assert!(BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0 <= STATION_Y / 2.0 - 30.0);
    }

    #[test]
    fn fixture_covers_identity_history_and_custody_features() {
        assert_eq!(SLOT_COUNT, 6);
        assert_eq!(TAMPER_SEAL_COUNT, SLOT_COUNT);
        assert_eq!(TOKEN_LANES, 2);
        assert_eq!(TOKENS_PER_LANE, SLOT_COUNT);
        assert_eq!(LOGGER_POCKET_COUNT, 4);
        assert!(EDGE_WITNESS_COUNT >= SLOT_COUNT);
        assert!(CENTER_WITNESS_COUNT >= 2);
        assert_eq!(QUARANTINE_BIN_COUNT, 2);
        assert_eq!(clean_used_land_count(), 8);
        assert!(EVIDENCE_CAMERA_COUNT >= 4);
        assert!(LIGHT_PIPE_COUNT >= SLOT_COUNT);
    }

    #[test]
    fn clearances_remain_robot_and_service_accessible() {
        assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE_Y);
        assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE_Y);
        assert!(SIDE_SERVICE_CLEARANCE_X >= 70.0);
    }
}
