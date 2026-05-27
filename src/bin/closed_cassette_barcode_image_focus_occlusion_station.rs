use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette barcode/RFID identity, focus, and occlusion validation station.
//
// Intent:
// - Verify cassette identity, slot assignment, and evidence capture before release
//   to culture or analysis.
// - Make damaged labels, blocked fiducials, condensation on imaging windows, focus
//   drift, wrong-slot identity, and camera/illumination failures mechanically
//   visible using physical challenge lands and witness hardware.
// - Model packaging, datum, containment, and service envelopes only. This is not
//   an ML model, image classifier, or biological acceptance criterion.

const BIN_PREFIX: &str = "closed_cassette_barcode_image_focus_occlusion_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_barcode_image_focus_occlusion_station_containment_deck.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_cassette_identity_dock.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_barcode_rfid_lands.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_focus_target_ladder.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_fiducial_occlusion_masks.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_condensation_witness_window.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_illumination_reference_strip.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_camera_bridge.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_wrong_slot_challenge_tokens.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_release_hold_reject_gate.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_evidence_wipe_keepouts.stl",
    "output/closed_cassette_barcode_image_focus_occlusion_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "cassette_identity_dock",
    "barcode_rfid_lands",
    "focus_target_ladder",
    "fiducial_occlusion_masks",
    "condensation_witness_window",
    "illumination_reference_strip",
    "camera_bridge",
    "wrong_slot_challenge_tokens",
    "release_hold_reject_gate",
    "evidence_wipe_keepouts",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 38.0;
const DECK_GUTTER_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.6;
const MOUNT_HOLE_COUNT: usize = 8;

const DOCK_CENTER: (f64, f64) = (-260.0, 122.0);
const DOCK_X: f64 = 450.0;
const DOCK_Y: f64 = 330.0;
const DOCK_Z: f64 = 42.0;
const CASSETTE_POCKET_X: f64 = 310.0;
const CASSETTE_POCKET_Y: f64 = 188.0;
const CASSETTE_POCKET_DEPTH: f64 = 20.0;
const DOCK_DATUM_PINS: usize = 6;

const ID_PANEL_CENTER: (f64, f64) = (-430.0, -198.0);
const ID_PANEL_X: f64 = 330.0;
const ID_PANEL_Y: f64 = 184.0;
const ID_PANEL_Z: f64 = 18.0;
const BARCODE_LAND_X: f64 = 198.0;
const BARCODE_LAND_Y: f64 = 46.0;
const RFID_LAND_X: f64 = 118.0;
const RFID_LAND_Y: f64 = 86.0;
const DAMAGE_STRIPE_COUNT: usize = 7;

const FOCUS_CENTER: (f64, f64) = (206.0, 210.0);
const FOCUS_BASE_X: f64 = 390.0;
const FOCUS_BASE_Y: f64 = 158.0;
const FOCUS_BASE_Z: f64 = 18.0;
const FOCUS_STEP_COUNT: usize = 9;
const FOCUS_STEP_X: f64 = 31.0;
const FOCUS_STEP_Y: f64 = 104.0;
const FOCUS_STEP_PITCH_X: f64 = 38.0;
const FOCUS_STEP_START_Z: f64 = 2.0;
const FOCUS_STEP_DELTA_Z: f64 = 1.4;

const MASK_CENTER: (f64, f64) = (280.0, -18.0);
const MASK_PLATE_X: f64 = 396.0;
const MASK_PLATE_Y: f64 = 214.0;
const MASK_PLATE_Z: f64 = 16.0;
const FIDUCIAL_ROWS: usize = 3;
const FIDUCIAL_COLS: usize = 4;
const FIDUCIAL_MASK_COUNT: usize = FIDUCIAL_ROWS * FIDUCIAL_COLS;
const MASK_CELL_X: f64 = 56.0;
const MASK_CELL_Y: f64 = 42.0;
const MASK_PITCH_X: f64 = 74.0;
const MASK_PITCH_Y: f64 = 58.0;

const CONDENSATION_CENTER: (f64, f64) = (326.0, -234.0);
const WINDOW_FRAME_X: f64 = 336.0;
const WINDOW_FRAME_Y: f64 = 152.0;
const WINDOW_FRAME_Z: f64 = 18.0;
const WINDOW_COUNT: usize = 4;
const WINDOW_D: f64 = 48.0;
const FOG_DOT_COUNT: usize = 18;

const ILLUMINATION_CENTER: (f64, f64) = (-68.0, -328.0);
const ILLUMINATION_X: f64 = 646.0;
const ILLUMINATION_Y: f64 = 70.0;
const ILLUMINATION_Z: f64 = 14.0;
const ILLUMINATION_SEGMENTS: usize = 8;
const ILLUMINATION_LAND_X: f64 = 54.0;
const ILLUMINATION_LAND_Y: f64 = 38.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 48.0);
const BRIDGE_SPAN_X: f64 = 920.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 58.0;
const BRIDGE_UNDERSIDE_Z: f64 = 198.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_COUNT: usize = 3;
const CAMERA_PITCH_X: f64 = 210.0;
const CAMERA_POD_X: f64 = 86.0;
const CAMERA_POD_Y: f64 = 74.0;
const CAMERA_POD_Z: f64 = 52.0;

const TOKEN_CENTER: (f64, f64) = (-30.0, -126.0);
const TOKEN_TRAY_X: f64 = 330.0;
const TOKEN_TRAY_Y: f64 = 152.0;
const TOKEN_TRAY_Z: f64 = 20.0;
const WRONG_SLOT_TOKENS: usize = 6;
const TOKEN_D: f64 = 42.0;

const GATE_CENTER: (f64, f64) = (414.0, 214.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOT_COUNT: usize = 4;
const STATUS_LANE_X: f64 = 196.0;
const STATUS_LANE_Y: f64 = 82.0;
const STATUS_LANE_Z: f64 = 24.0;
const STATUS_LANE_PITCH_Y: f64 = 98.0;

const EVIDENCE_CENTER: (f64, f64) = (-458.0, 196.0);
const EVIDENCE_POCKET_COUNT: usize = 4;
const EVIDENCE_POCKET_X: f64 = 122.0;
const EVIDENCE_POCKET_Y: f64 = 48.0;
const EVIDENCE_BLOCK_X: f64 = 202.0;
const EVIDENCE_BLOCK_Y: f64 = 278.0;
const EVIDENCE_BLOCK_Z: f64 = 36.0;
const WIPE_COUPON_COUNT: usize = 5;
const WIPE_COUPON_X: f64 = 58.0;
const WIPE_COUPON_Y: f64 = 32.0;
const ROBOT_KEEP_OUT_X: f64 = 930.0;
const ROBOT_KEEP_OUT_Y: f64 = 104.0;
const ROBOT_KEEP_OUT_Z: f64 = 148.0;
const SERVICE_KEEP_OUT_X: f64 = 172.0;
const SERVICE_KEEP_OUT_Y: f64 = 610.0;
const SERVICE_KEEP_OUT_Z: f64 = 118.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let dock = cassette_identity_dock();
    export(OUTPUTS[1], &dock);

    let identity = barcode_rfid_lands();
    export(OUTPUTS[2], &identity);

    let focus = focus_target_ladder();
    export(OUTPUTS[3], &focus);

    let masks = fiducial_occlusion_masks();
    export(OUTPUTS[4], &masks);

    let condensation = condensation_witness_window();
    export(OUTPUTS[5], &condensation);

    let illumination = illumination_reference_strip();
    export(OUTPUTS[6], &illumination);

    let bridge = camera_bridge();
    export(OUTPUTS[7], &bridge);

    let tokens = wrong_slot_challenge_tokens();
    export(OUTPUTS[8], &tokens);

    let gates = release_hold_reject_gate();
    export(OUTPUTS[9], &gates);

    let evidence = evidence_wipe_keepouts();
    export(OUTPUTS[10], &evidence);

    let assembly = deck
        + dock
        + identity
        + focus
        + masks
        + condensation
        + illumination
        + bridge
        + tokens
        + gates
        + evidence;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette barcode/image focus/occlusion station:");
    println!("  Footprint:                 {DECK_X:.0}mm x {DECK_Y:.0}mm containment deck");
    println!(
        "  Cassette identity:         dock pocket {:.0}mm x {:.0}mm, {DOCK_DATUM_PINS} datum pins, barcode damage stripes, RFID coil land",
        CASSETTE_POCKET_X, CASSETTE_POCKET_Y
    );
    println!(
        "  Imaging challenges:        {FOCUS_STEP_COUNT} focus ladder steps over {:.1}mm, {FIDUCIAL_MASK_COUNT} occlusion cells, {WINDOW_COUNT} condensation witness windows",
        focus_ladder_span_z()
    );
    println!(
        "  Lighting and camera:       {ILLUMINATION_SEGMENTS} illumination reference lands, {CAMERA_COUNT} camera pods, bridge underside {BRIDGE_UNDERSIDE_Z:.0}mm above deck"
    );
    println!(
        "  Release workflow:          {WRONG_SLOT_TOKENS} wrong-slot challenge tokens, {STATUS_LANES} release/hold/reject lanes, {EVIDENCE_POCKET_COUNT} evidence card pockets, {WIPE_COUPON_COUNT} cleaning wipe coupons"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{BIN_PREFIX}_containment_deck"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let recessed_pan = centered_cube(
        format!("{BIN_PREFIX}_shallow_liquid_containment_recess"),
        DECK_X - 108.0,
        DECK_Y - 118.0,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z - 2.8);
    let gutter = rectangular_outline(
        format!("{BIN_PREFIX}_wipeable_perimeter_drain_gutter"),
        DECK_X - 74.0,
        DECK_Y - 82.0,
        DECK_GUTTER_W,
        4.0,
    )
    .translate(0.0, 0.0, DECK_Z + 2.0);

    deck - recessed_pan - mount_holes() + deck_rim() + gutter + station_socket_recesses()
}

fn deck_rim() -> Part {
    let left = centered_cube(
        format!("{BIN_PREFIX}_left_containment_rim"),
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{BIN_PREFIX}_right_containment_rim"),
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{BIN_PREFIX}_rear_containment_rim"),
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let front_left = centered_cube(
        format!("{BIN_PREFIX}_front_left_robot_entry_lip"),
        260.0,
        DECK_RIM_W,
        20.0,
    )
    .translate(
        -DECK_X / 2.0 + 130.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z + 10.0,
    );
    let front_right = centered_cube(
        format!("{BIN_PREFIX}_front_right_robot_entry_lip"),
        260.0,
        DECK_RIM_W,
        20.0,
    )
    .translate(
        DECK_X / 2.0 - 130.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z + 10.0,
    );

    left + right + rear + front_left + front_right
}

fn station_socket_recesses() -> Part {
    let specs = [
        (DOCK_CENTER.0, DOCK_CENTER.1, DOCK_X + 26.0, DOCK_Y + 24.0),
        (
            ID_PANEL_CENTER.0,
            ID_PANEL_CENTER.1,
            ID_PANEL_X + 22.0,
            ID_PANEL_Y + 20.0,
        ),
        (
            FOCUS_CENTER.0,
            FOCUS_CENTER.1,
            FOCUS_BASE_X + 24.0,
            FOCUS_BASE_Y + 22.0,
        ),
        (
            MASK_CENTER.0,
            MASK_CENTER.1,
            MASK_PLATE_X + 24.0,
            MASK_PLATE_Y + 24.0,
        ),
        (
            CONDENSATION_CENTER.0,
            CONDENSATION_CENTER.1,
            WINDOW_FRAME_X + 24.0,
            WINDOW_FRAME_Y + 20.0,
        ),
        (
            TOKEN_CENTER.0,
            TOKEN_CENTER.1,
            TOKEN_TRAY_X + 20.0,
            TOKEN_TRAY_Y + 18.0,
        ),
    ];

    let mut recesses = Part::empty(format!("{BIN_PREFIX}_deck_component_socket_recesses"));
    for (index, (x, y, sx, sy)) in specs.iter().enumerate() {
        recesses = recesses
            + centered_cube(
                format!("{BIN_PREFIX}_deck_component_socket_{index}"),
                *sx,
                *sy,
                5.5,
            )
            .translate(*x, *y, DECK_Z - 2.2);
    }
    recesses
}

fn cassette_identity_dock() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_cassette_dock_base"),
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    )
    .translate(DOCK_CENTER.0, DOCK_CENTER.1, DECK_Z + DOCK_Z / 2.0);
    let pocket = centered_cube(
        format!("{BIN_PREFIX}_closed_cassette_capture_pocket"),
        CASSETTE_POCKET_X,
        CASSETTE_POCKET_Y,
        CASSETTE_POCKET_DEPTH + 2.0,
    )
    .translate(
        DOCK_CENTER.0,
        DOCK_CENTER.1,
        DECK_Z + DOCK_Z - CASSETTE_POCKET_DEPTH / 2.0,
    );
    let lead_in = centered_cube(
        format!("{BIN_PREFIX}_front_wrong_slot_lead_in_window"),
        CASSETTE_POCKET_X - 54.0,
        38.0,
        16.0,
    )
    .translate(
        DOCK_CENTER.0,
        DOCK_CENTER.1 - DOCK_Y / 2.0 + 20.0,
        DECK_Z + DOCK_Z - 8.0,
    );

    base - pocket - lead_in + dock_rails() + dock_datum_pins() + dock_orientation_key()
}

fn dock_rails() -> Part {
    let rail_z = 36.0;
    let left = centered_cube(
        format!("{BIN_PREFIX}_cassette_left_datum_rail"),
        14.0,
        CASSETTE_POCKET_Y + 58.0,
        rail_z,
    )
    .translate(
        DOCK_CENTER.0 - CASSETTE_POCKET_X / 2.0 - 18.0,
        DOCK_CENTER.1,
        DECK_Z + DOCK_Z + rail_z / 2.0,
    );
    let rear = centered_cube(
        format!("{BIN_PREFIX}_cassette_rear_hard_stop_rail"),
        CASSETTE_POCKET_X + 54.0,
        14.0,
        rail_z,
    )
    .translate(
        DOCK_CENTER.0,
        DOCK_CENTER.1 + CASSETTE_POCKET_Y / 2.0 + 18.0,
        DECK_Z + DOCK_Z + rail_z / 2.0,
    );
    let right_short = centered_cube(
        format!("{BIN_PREFIX}_cassette_right_short_capture_rail"),
        14.0,
        110.0,
        rail_z,
    )
    .translate(
        DOCK_CENTER.0 + CASSETTE_POCKET_X / 2.0 + 18.0,
        DOCK_CENTER.1 + 34.0,
        DECK_Z + DOCK_Z + rail_z / 2.0,
    );
    left + rear + right_short
}

fn dock_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{BIN_PREFIX}_cassette_dock_datum_pins"));
    for index in 0..DOCK_DATUM_PINS {
        let x = DOCK_CENTER.0 + lane_x(index % 3, 3, 108.0);
        let y = DOCK_CENTER.1 + if index < 3 { -92.0 } else { 92.0 };
        pins = pins
            + centered_cylinder(
                format!("{BIN_PREFIX}_dock_datum_pin_{index}"),
                5.0,
                18.0,
                28,
            )
            .translate(x, y, DECK_Z + DOCK_Z + 9.0);
    }
    pins
}

fn dock_orientation_key() -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_asymmetric_slot_orientation_key"),
        72.0,
        26.0,
        24.0,
    )
    .translate(
        DOCK_CENTER.0 + CASSETTE_POCKET_X / 2.0 - 44.0,
        DOCK_CENTER.1 - CASSETTE_POCKET_Y / 2.0 - 18.0,
        DECK_Z + DOCK_Z + 12.0,
    )
}

fn barcode_rfid_lands() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_barcode_rfid_damage_challenge_panel"),
        ID_PANEL_X,
        ID_PANEL_Y,
        ID_PANEL_Z,
    )
    .translate(
        ID_PANEL_CENTER.0,
        ID_PANEL_CENTER.1,
        DECK_Z + ID_PANEL_Z / 2.0,
    );
    let barcode = centered_cube(
        format!("{BIN_PREFIX}_barcode_label_flat_scan_land"),
        BARCODE_LAND_X,
        BARCODE_LAND_Y,
        4.0,
    )
    .translate(
        ID_PANEL_CENTER.0 - 46.0,
        ID_PANEL_CENTER.1 - 38.0,
        DECK_Z + ID_PANEL_Z + 2.0,
    );
    let rfid = centered_cube(
        format!("{BIN_PREFIX}_rfid_antenna_coupling_land"),
        RFID_LAND_X,
        RFID_LAND_Y,
        4.0,
    )
    .translate(
        ID_PANEL_CENTER.0 + 88.0,
        ID_PANEL_CENTER.1 + 38.0,
        DECK_Z + ID_PANEL_Z + 2.0,
    );
    let barcode_damage = label_damage_stripes();
    let rfid_coil = rfid_coil_markers();
    let fences = side_fence_pair(
        "barcode_label_damage_land",
        BARCODE_LAND_X + 22.0,
        BARCODE_LAND_Y + 18.0,
        7.0,
        19.0,
    )
    .translate(
        ID_PANEL_CENTER.0 - 46.0,
        ID_PANEL_CENTER.1 - 38.0,
        DECK_Z + ID_PANEL_Z + 9.5,
    );

    panel + barcode + rfid + barcode_damage + rfid_coil + fences
}

fn label_damage_stripes() -> Part {
    let mut stripes = Part::empty(format!("{BIN_PREFIX}_barcode_label_damage_stripes"));
    for index in 0..DAMAGE_STRIPE_COUNT {
        let x = ID_PANEL_CENTER.0 - 46.0 + lane_x(index, DAMAGE_STRIPE_COUNT, 24.0);
        let h = if index % 2 == 0 { 18.0 } else { 31.0 };
        stripes = stripes
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_tear_or_smudge_stripe_{index}"),
                8.0,
                h,
                5.0,
            )
            .translate(x, ID_PANEL_CENTER.1 - 38.0, DECK_Z + ID_PANEL_Z + 6.5);
    }
    stripes
}

fn rfid_coil_markers() -> Part {
    let mut coil = Part::empty(format!("{BIN_PREFIX}_rfid_nested_coil_marker"));
    for index in 0..4 {
        coil = coil
            + rectangular_outline(
                format!("{BIN_PREFIX}_rfid_coil_outline_{index}"),
                RFID_LAND_X - 18.0 - index as f64 * 18.0,
                RFID_LAND_Y - 16.0 - index as f64 * 13.0,
                2.0,
                2.0,
            )
            .translate(
                ID_PANEL_CENTER.0 + 88.0,
                ID_PANEL_CENTER.1 + 38.0,
                DECK_Z + ID_PANEL_Z + 5.0,
            );
    }
    coil
}

fn focus_target_ladder() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_focus_target_ladder_base"),
        FOCUS_BASE_X,
        FOCUS_BASE_Y,
        FOCUS_BASE_Z,
    )
    .translate(FOCUS_CENTER.0, FOCUS_CENTER.1, DECK_Z + FOCUS_BASE_Z / 2.0);
    let mut steps = Part::empty(format!("{BIN_PREFIX}_focus_target_height_steps"));
    for index in 0..FOCUS_STEP_COUNT {
        let step_z = focus_step_height(index);
        let x = FOCUS_CENTER.0 + lane_x(index, FOCUS_STEP_COUNT, FOCUS_STEP_PITCH_X);
        steps = steps
            + centered_cube(
                format!("{BIN_PREFIX}_focus_step_drift_reference_{index}"),
                FOCUS_STEP_X,
                FOCUS_STEP_Y,
                step_z,
            )
            .translate(x, FOCUS_CENTER.1, DECK_Z + FOCUS_BASE_Z + step_z / 2.0)
            + centered_cylinder(
                format!("{BIN_PREFIX}_specular_focus_dot_{index}"),
                4.8,
                2.0,
                24,
            )
            .translate(
                x,
                FOCUS_CENTER.1 - 34.0,
                DECK_Z + FOCUS_BASE_Z + step_z + 1.0,
            );
    }

    base + steps
        + centered_cube(
            format!("{BIN_PREFIX}_zero_focus_reference_bar"),
            FOCUS_BASE_X - 58.0,
            10.0,
            5.0,
        )
        .translate(
            FOCUS_CENTER.0,
            FOCUS_CENTER.1 + FOCUS_BASE_Y / 2.0 - 22.0,
            DECK_Z + FOCUS_BASE_Z + 2.5,
        )
}

fn fiducial_occlusion_masks() -> Part {
    let plate = centered_cube(
        format!("{BIN_PREFIX}_fiducial_occlusion_mask_plate"),
        MASK_PLATE_X,
        MASK_PLATE_Y,
        MASK_PLATE_Z,
    )
    .translate(MASK_CENTER.0, MASK_CENTER.1, DECK_Z + MASK_PLATE_Z / 2.0);
    let mut mask_cuts = Part::empty(format!("{BIN_PREFIX}_fiducial_visible_window_cuts"));
    let mut shutters = Part::empty(format!("{BIN_PREFIX}_fiducial_occlusion_shutter_tabs"));
    let mut fiducials = Part::empty(format!("{BIN_PREFIX}_raised_unblocked_fiducial_targets"));
    for row in 0..FIDUCIAL_ROWS {
        for col in 0..FIDUCIAL_COLS {
            let index = row * FIDUCIAL_COLS + col;
            let x = MASK_CENTER.0 + lane_x(col, FIDUCIAL_COLS, MASK_PITCH_X);
            let y = MASK_CENTER.1 + lane_x(row, FIDUCIAL_ROWS, MASK_PITCH_Y);
            mask_cuts = mask_cuts
                + centered_cube(
                    format!("{BIN_PREFIX}_fiducial_window_cut_{index}"),
                    MASK_CELL_X,
                    MASK_CELL_Y,
                    MASK_PLATE_Z + 6.0,
                )
                .translate(x, y, DECK_Z + MASK_PLATE_Z / 2.0);
            let shutter_x = if (row + col) % 2 == 0 { 25.0 } else { 12.0 };
            shutters = shutters
                + centered_cube(
                    format!("{BIN_PREFIX}_partial_occlusion_shutter_{index}"),
                    shutter_x,
                    MASK_CELL_Y + 16.0,
                    7.0,
                )
                .translate(
                    x - MASK_CELL_X / 2.0 + shutter_x / 2.0,
                    y,
                    DECK_Z + MASK_PLATE_Z + 3.5,
                );
            fiducials = fiducials
                + centered_cylinder(format!("{BIN_PREFIX}_fiducial_dot_{index}"), 5.0, 4.0, 24)
                    .translate(x + 14.0, y, DECK_Z + MASK_PLATE_Z + 2.0);
        }
    }
    plate - mask_cuts + shutters + fiducials
}

fn condensation_witness_window() -> Part {
    let frame = centered_cube(
        format!("{BIN_PREFIX}_condensation_witness_window_frame"),
        WINDOW_FRAME_X,
        WINDOW_FRAME_Y,
        WINDOW_FRAME_Z,
    )
    .translate(
        CONDENSATION_CENTER.0,
        CONDENSATION_CENTER.1,
        DECK_Z + WINDOW_FRAME_Z / 2.0,
    );
    let mut windows = Part::empty(format!("{BIN_PREFIX}_clear_window_openings"));
    let mut rings = Part::empty(format!("{BIN_PREFIX}_window_gasket_rings"));
    for index in 0..WINDOW_COUNT {
        let x = CONDENSATION_CENTER.0 + lane_x(index, WINDOW_COUNT, 72.0);
        windows = windows
            + centered_cylinder(
                format!("{BIN_PREFIX}_condensation_window_cut_{index}"),
                WINDOW_D / 2.0,
                WINDOW_FRAME_Z + 6.0,
                48,
            )
            .translate(x, CONDENSATION_CENTER.1, DECK_Z + WINDOW_FRAME_Z / 2.0);
        rings = rings
            + (centered_cylinder(
                format!("{BIN_PREFIX}_condensation_window_outer_ring_{index}"),
                WINDOW_D / 2.0 + 8.0,
                4.0,
                48,
            ) - centered_cylinder(
                format!("{BIN_PREFIX}_condensation_window_inner_ring_{index}"),
                WINDOW_D / 2.0 + 1.0,
                5.0,
                48,
            ))
            .translate(x, CONDENSATION_CENTER.1, DECK_Z + WINDOW_FRAME_Z + 2.0);
    }

    frame - windows + rings + fog_dot_witnesses()
}

fn fog_dot_witnesses() -> Part {
    let mut dots = Part::empty(format!("{BIN_PREFIX}_condensation_fog_dot_witnesses"));
    for index in 0..FOG_DOT_COUNT {
        let col = index % 6;
        let row = index / 6;
        let x = CONDENSATION_CENTER.0 - 134.0 + col as f64 * 18.0;
        let y = CONDENSATION_CENTER.1 + 54.0 - row as f64 * 18.0;
        dots =
            dots + centered_cylinder(format!("{BIN_PREFIX}_fog_dot_{index}"), 2.5, 2.2, 16)
                .translate(x, y, DECK_Z + WINDOW_FRAME_Z + 1.1);
    }
    dots
}

fn illumination_reference_strip() -> Part {
    let strip = centered_cube(
        format!("{BIN_PREFIX}_illumination_reference_strip_base"),
        ILLUMINATION_X,
        ILLUMINATION_Y,
        ILLUMINATION_Z,
    )
    .translate(
        ILLUMINATION_CENTER.0,
        ILLUMINATION_CENTER.1,
        DECK_Z + ILLUMINATION_Z / 2.0,
    );
    let mut lands = Part::empty(format!("{BIN_PREFIX}_illumination_brightness_lands"));
    for index in 0..ILLUMINATION_SEGMENTS {
        let x = ILLUMINATION_CENTER.0 + lane_x(index, ILLUMINATION_SEGMENTS, 74.0);
        let z = 3.0 + index as f64 * 0.55;
        lands = lands
            + centered_cube(
                format!("{BIN_PREFIX}_illumination_reference_patch_{index}"),
                ILLUMINATION_LAND_X,
                ILLUMINATION_LAND_Y,
                z,
            )
            .translate(x, ILLUMINATION_CENTER.1, DECK_Z + ILLUMINATION_Z + z / 2.0);
    }
    strip
        + lands
        + rectangular_outline(
            format!("{BIN_PREFIX}_illumination_strip_protective_fence"),
            ILLUMINATION_X - 28.0,
            ILLUMINATION_Y - 16.0,
            4.0,
            18.0,
        )
        .translate(
            ILLUMINATION_CENTER.0,
            ILLUMINATION_CENTER.1,
            DECK_Z + ILLUMINATION_Z + 9.0,
        )
}

fn camera_bridge() -> Part {
    let left_post = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_crossbeam"),
        BRIDGE_SPAN_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    left_post + right_post + beam + camera_pods() + cable_gland_lands()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{BIN_PREFIX}_camera_and_lighting_failure_pods"));
    for index in 0..CAMERA_COUNT {
        let x = BRIDGE_CENTER.0 + lane_x(index, CAMERA_COUNT, CAMERA_PITCH_X);
        let body = centered_cube(
            format!("{BIN_PREFIX}_camera_pod_body_{index}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0 - 10.0,
        );
        let lens = centered_cylinder(
            format!("{BIN_PREFIX}_camera_lens_aperture_{index}"),
            14.0,
            8.0,
            36,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z - 16.0,
        );
        let ring = centered_cylinder(
            format!("{BIN_PREFIX}_camera_ring_light_reference_{index}"),
            24.0,
            3.0,
            42,
        ) - centered_cylinder(
            format!("{BIN_PREFIX}_camera_ring_light_opening_{index}"),
            15.5,
            4.0,
            42,
        );
        pods = pods
            + body
            + lens
            + ring.translate(
                x,
                BRIDGE_CENTER.1,
                DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z - 22.0,
            );
    }
    pods
}

fn cable_gland_lands() -> Part {
    let mut glands = Part::empty(format!("{BIN_PREFIX}_camera_cable_gland_lands"));
    for index in 0..CAMERA_COUNT {
        glands = glands
            + centered_cube(
                format!("{BIN_PREFIX}_camera_service_cable_gland_land_{index}"),
                42.0,
                14.0,
                8.0,
            )
            .translate(
                BRIDGE_CENTER.0 + lane_x(index, CAMERA_COUNT, CAMERA_PITCH_X),
                BRIDGE_CENTER.1 + BRIDGE_POST_Y / 2.0 + 9.0,
                DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 4.0,
            );
    }
    glands
}

fn wrong_slot_challenge_tokens() -> Part {
    let tray = centered_cube(
        format!("{BIN_PREFIX}_wrong_slot_challenge_token_tray"),
        TOKEN_TRAY_X,
        TOKEN_TRAY_Y,
        TOKEN_TRAY_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z + TOKEN_TRAY_Z / 2.0);
    let mut tokens = Part::empty(format!("{BIN_PREFIX}_wrong_slot_identity_tokens"));
    let mut pockets = Part::empty(format!("{BIN_PREFIX}_wrong_slot_token_pockets"));
    for index in 0..WRONG_SLOT_TOKENS {
        let x = TOKEN_CENTER.0 + lane_x(index % 3, 3, 82.0);
        let y = TOKEN_CENTER.1 + if index < 3 { -34.0 } else { 34.0 };
        pockets = pockets
            + centered_cylinder(
                format!("{BIN_PREFIX}_wrong_slot_token_pocket_{index}"),
                TOKEN_D / 2.0 + 3.0,
                9.0,
                36,
            )
            .translate(x, y, DECK_Z + TOKEN_TRAY_Z - 4.0);
        tokens = tokens
            + centered_cylinder(
                format!("{BIN_PREFIX}_slot_mismatch_token_{index}"),
                TOKEN_D / 2.0,
                9.0,
                36,
            )
            .translate(x, y, DECK_Z + TOKEN_TRAY_Z + 4.5)
            + centered_cube(
                format!("{BIN_PREFIX}_token_barcode_land_{index}"),
                28.0,
                7.0,
                2.0,
            )
            .translate(x, y, DECK_Z + TOKEN_TRAY_Z + 10.0);
    }
    tray - pockets + tokens
}

fn release_hold_reject_gate() -> Part {
    let mut gates = Part::empty(format!("{BIN_PREFIX}_release_hold_reject_gate"));
    for lane in 0..STATUS_LANES {
        let y = GATE_CENTER.1 + lane_x(lane, STATUS_LANES, STATUS_LANE_PITCH_Y);
        let lane_base = centered_cube(
            format!("{BIN_PREFIX}_status_lane_base_{lane}"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(GATE_CENTER.0, y, DECK_Z + STATUS_LANE_Z / 2.0);
        let gate_bar = centered_cube(
            format!("{BIN_PREFIX}_status_lane_sliding_gate_bar_{lane}"),
            STATUS_LANE_X + 24.0,
            9.0,
            42.0,
        )
        .translate(
            GATE_CENTER.0,
            y + STATUS_LANE_Y / 2.0 + 8.0,
            DECK_Z + STATUS_LANE_Z + 21.0,
        );
        let mut slots = Part::empty(format!("{BIN_PREFIX}_status_lane_slots_{lane}"));
        for slot in 0..STATUS_SLOT_COUNT {
            slots = slots
                + centered_cube(
                    format!("{BIN_PREFIX}_status_lane_{lane}_card_slot_{slot}"),
                    34.0,
                    42.0,
                    10.0,
                )
                .translate(
                    GATE_CENTER.0 + lane_x(slot, STATUS_SLOT_COUNT, 43.0),
                    y,
                    DECK_Z + STATUS_LANE_Z - 5.0,
                );
        }
        gates = gates + lane_base - slots + gate_bar;
    }
    gates
}

fn evidence_wipe_keepouts() -> Part {
    evidence_card_pockets() + cleaning_wipe_coupons() + robot_service_keepouts()
}

fn evidence_card_pockets() -> Part {
    let block = centered_cube(
        format!("{BIN_PREFIX}_evidence_card_pocket_block"),
        EVIDENCE_BLOCK_X,
        EVIDENCE_BLOCK_Y,
        EVIDENCE_BLOCK_Z,
    )
    .translate(
        EVIDENCE_CENTER.0,
        EVIDENCE_CENTER.1,
        DECK_Z + EVIDENCE_BLOCK_Z / 2.0,
    );
    let mut pockets = Part::empty(format!("{BIN_PREFIX}_evidence_card_recesses"));
    for index in 0..EVIDENCE_POCKET_COUNT {
        pockets = pockets
            + centered_cube(
                format!("{BIN_PREFIX}_evidence_card_pocket_{index}"),
                EVIDENCE_POCKET_X,
                EVIDENCE_POCKET_Y,
                18.0,
            )
            .translate(
                EVIDENCE_CENTER.0,
                EVIDENCE_CENTER.1 + lane_x(index, EVIDENCE_POCKET_COUNT, 58.0),
                DECK_Z + EVIDENCE_BLOCK_Z - 9.0,
            );
    }
    block - pockets
}

fn cleaning_wipe_coupons() -> Part {
    let mut coupons = Part::empty(format!("{BIN_PREFIX}_cleaning_wipe_coupons"));
    for index in 0..WIPE_COUPON_COUNT {
        coupons = coupons
            + centered_cube(
                format!("{BIN_PREFIX}_wipe_coupon_surface_{index}"),
                WIPE_COUPON_X,
                WIPE_COUPON_Y,
                5.0,
            )
            .translate(
                -238.0 + lane_x(index, WIPE_COUPON_COUNT, 68.0),
                342.0,
                DECK_Z + 2.5,
            )
            + centered_cube(
                format!("{BIN_PREFIX}_wipe_coupon_pull_tab_{index}"),
                12.0,
                18.0,
                9.0,
            )
            .translate(
                -238.0 + lane_x(index, WIPE_COUPON_COUNT, 68.0),
                342.0 + WIPE_COUPON_Y / 2.0 + 10.0,
                DECK_Z + 4.5,
            );
    }
    coupons
}

fn robot_service_keepouts() -> Part {
    let robot = wireframe_box(
        format!("{BIN_PREFIX}_front_robot_sweep_keepout"),
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
        5.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 72.0, DECK_Z + ROBOT_KEEP_OUT_Z / 2.0);
    let service = wireframe_box(
        format!("{BIN_PREFIX}_right_scanner_service_keepout"),
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        5.0,
    )
    .translate(
        DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 30.0,
        0.0,
        DECK_Z + SERVICE_KEEP_OUT_Z / 2.0,
    );
    robot + service
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{BIN_PREFIX}_m6_mount_holes"));
    for (index, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{BIN_PREFIX}_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn rectangular_outline(name: impl Into<String>, sx: f64, sy: f64, rail: f64, z: f64) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_front"), sx, rail, z).translate(0.0, -sy / 2.0 + rail / 2.0, 0.0)
        + centered_cube(format!("{name}_rear"), sx, rail, z).translate(
            0.0,
            sy / 2.0 - rail / 2.0,
            0.0,
        )
        + centered_cube(format!("{name}_left"), rail, sy, z).translate(
            -sx / 2.0 + rail / 2.0,
            0.0,
            0.0,
        )
        + centered_cube(format!("{name}_right"), rail, sy, z).translate(
            sx / 2.0 - rail / 2.0,
            0.0,
            0.0,
        )
}

fn side_fence_pair(name: &str, sx: f64, sy: f64, fence_w: f64, fence_z: f64) -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_{name}_left_fence"),
        fence_w,
        sy,
        fence_z,
    )
    .translate(-sx / 2.0 + fence_w / 2.0, 0.0, 0.0)
        + centered_cube(
            format!("{BIN_PREFIX}_{name}_right_fence"),
            fence_w,
            sy,
            fence_z,
        )
        .translate(sx / 2.0 - fence_w / 2.0, 0.0, 0.0)
}

fn wireframe_box(name: impl Into<String>, sx: f64, sy: f64, sz: f64, rail: f64) -> Part {
    let name = name.into();
    let bottom = rectangular_outline(format!("{name}_bottom"), sx, sy, rail, rail).translate(
        0.0,
        0.0,
        -sz / 2.0 + rail / 2.0,
    );
    let top = rectangular_outline(format!("{name}_top"), sx, sy, rail, rail).translate(
        0.0,
        0.0,
        sz / 2.0 - rail / 2.0,
    );
    let mut posts = Part::empty(format!("{name}_posts"));
    for (index, (x, y)) in [
        (-sx / 2.0 + rail / 2.0, -sy / 2.0 + rail / 2.0),
        (sx / 2.0 - rail / 2.0, -sy / 2.0 + rail / 2.0),
        (-sx / 2.0 + rail / 2.0, sy / 2.0 - rail / 2.0),
        (sx / 2.0 - rail / 2.0, sy / 2.0 - rail / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{index}"), rail, rail, sz)
                .translate(*x, *y, 0.0);
    }
    bottom + top + posts
}

fn mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 250.0, 0.0),
        (DECK_X / 2.0 - 250.0, 0.0),
        (-80.0, DECK_Y / 2.0 - 58.0),
        (80.0, DECK_Y / 2.0 - 58.0),
    ]
}

fn focus_step_height(index: usize) -> f64 {
    FOCUS_STEP_START_Z + index as f64 * FOCUS_STEP_DELTA_Z
}

fn focus_ladder_span_z() -> f64 {
    focus_step_height(FOCUS_STEP_COUNT - 1) - focus_step_height(0)
}

fn bridge_clearance_over_focus() -> f64 {
    BRIDGE_UNDERSIDE_Z - (FOCUS_BASE_Z + focus_step_height(FOCUS_STEP_COUNT - 1))
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert!(
        DOCK_X < DECK_X - 420.0,
        "cassette dock leaves insufficient deck margin"
    );
    assert!(
        bridge_clearance_over_focus() > 150.0,
        "camera bridge collides with focus target ladder"
    );
    assert!(
        STATUS_LANES == 3,
        "release/hold/reject gate must model exactly three lanes"
    );
    assert!(
        FIDUCIAL_MASK_COUNT >= 12,
        "occlusion mask does not include enough fiducial challenge cells"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_are_prefixed_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_cassette_barcode_image_focus_occlusion_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_requested_station_intent() {
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_lands"));
        assert!(REQUIRED_FEATURES.contains(&"focus_target_ladder"));
        assert!(REQUIRED_FEATURES.contains(&"fiducial_occlusion_masks"));
        assert!(REQUIRED_FEATURES.contains(&"condensation_witness_window"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_gate"));
        assert_eq!(REQUIRED_FEATURES.len(), OUTPUTS.len() - 1);
    }

    #[test]
    fn focus_ladder_has_monotonic_drift_challenge() {
        for index in 1..FOCUS_STEP_COUNT {
            assert!(focus_step_height(index) > focus_step_height(index - 1));
        }
        assert!(focus_ladder_span_z() >= 10.0);
    }

    #[test]
    fn challenge_counts_are_mechanically_visible() {
        assert_eq!(FIDUCIAL_MASK_COUNT, 12);
        assert_eq!(WINDOW_COUNT, 4);
        assert_eq!(ILLUMINATION_SEGMENTS, 8);
        assert_eq!(WRONG_SLOT_TOKENS, 6);
        assert_eq!(STATUS_LANES, 3);
    }

    #[test]
    fn bridge_and_service_clearances_are_positive() {
        assert!(bridge_clearance_over_focus() > 150.0);
        assert!(ROBOT_KEEP_OUT_X > DOCK_X);
        assert!(SERVICE_KEEP_OUT_Y > DOCK_Y);
    }
}
