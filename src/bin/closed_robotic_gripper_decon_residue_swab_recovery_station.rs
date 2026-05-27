use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic gripper decontamination residue swab recovery station.
//
// Intent:
// - Validate that the cassette gripper/end-effector can be decontaminated and
//   swabbed reproducibly without residue carryover between clean and dirty zones.
// - Package jaw nests, residue coupon grids, rinse capture, custody tags,
//   positive/negative witness tokens, camera evidence, and robot approach gauges
//   into one closed workcell fixture.
// - Keep this as manufacturable validation-fixture CAD only. It does not define
//   acceptance limits, residue chemistry, robot motion programs, or a validated
//   decontamination process.

const BIN_PREFIX: &str = "closed_robotic_gripper_decon_residue_swab_recovery_station";

const OUTPUTS: &[&str] = &[
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_base_tray.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_gripper_jaw_nests.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_swab_recovery_coupon_grid.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_rinse_capture_channels.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_clean_dirty_segregation_bulkhead.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_residue_witness_token_bank.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_barcode_custody_tag_lands.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_swab_tool_recovery_quiver.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_custody_disposition_lanes.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_camera_evidence_bridge.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_robot_approach_gauges.stl",
    "output/closed_robotic_gripper_decon_residue_swab_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "gripper_jaw_nests",
    "swab_recovery_coupon_grid",
    "rinse_capture_channels",
    "clean_dirty_segregation",
    "camera_evidence_bridge",
    "barcode_custody_tags",
    "positive_negative_residue_witness_tokens",
    "robot_approach_gauges",
    "assembly_export",
];

const DECK_X: f64 = 1240.0;
const DECK_Y: f64 = 780.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const MODULE_GAP: f64 = 24.0;
const DATUM_PIN_D: f64 = 6.0;

const JAW_NEST_CENTER: (f64, f64) = (-405.0, 205.0);
const JAW_NEST_X: f64 = 340.0;
const JAW_NEST_Y: f64 = 210.0;
const JAW_NEST_Z: f64 = 58.0;
const JAW_PAIR_COUNT: usize = 2;
const JAWS_PER_PAIR: usize = 2;
const JAW_NEST_COUNT: usize = JAW_PAIR_COUNT * JAWS_PER_PAIR;
const JAW_SLOT_X: f64 = 82.0;
const JAW_SLOT_Y: f64 = 38.0;
const JAW_SLOT_PITCH_X: f64 = 128.0;
const JAW_SLOT_PITCH_Y: f64 = 82.0;

const COUPON_GRID_CENTER: (f64, f64) = (0.0, 205.0);
const COUPON_GRID_X: f64 = 360.0;
const COUPON_GRID_Y: f64 = 210.0;
const COUPON_GRID_Z: f64 = 44.0;
const COUPON_COLS: usize = 6;
const COUPON_ROWS: usize = 4;
const COUPON_COUNT: usize = COUPON_COLS * COUPON_ROWS;
const COUPON_SLOT_X: f64 = 38.0;
const COUPON_SLOT_Y: f64 = 26.0;
const COUPON_PITCH_X: f64 = 50.0;
const COUPON_PITCH_Y: f64 = 38.0;

const RINSE_CENTER: (f64, f64) = (405.0, 205.0);
const RINSE_X: f64 = 340.0;
const RINSE_Y: f64 = 210.0;
const RINSE_Z: f64 = 48.0;
const RINSE_CHANNEL_COUNT: usize = 4;
const RINSE_CHANNEL_X: f64 = 52.0;
const RINSE_CHANNEL_Y: f64 = 160.0;
const RINSE_CHANNEL_PITCH_X: f64 = 66.0;
const RINSE_VIAL_COUNT: usize = RINSE_CHANNEL_COUNT;

const SEGREGATION_CENTER: (f64, f64) = (-405.0, -65.0);
const SEGREGATION_X: f64 = 340.0;
const SEGREGATION_Y: f64 = 220.0;
const SEGREGATION_Z: f64 = 54.0;
const BULKHEAD_Z: f64 = 112.0;
const CLEAN_SWAB_SLOTS: usize = 8;
const DIRTY_RETURN_SLOTS: usize = 8;

const WITNESS_CENTER: (f64, f64) = (0.0, -65.0);
const WITNESS_X: f64 = 360.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 42.0;
const POSITIVE_TOKEN_COUNT: usize = 8;
const NEGATIVE_TOKEN_COUNT: usize = 8;
const BLANK_TOKEN_COUNT: usize = 4;
const TOKEN_D: f64 = 24.0;
const TOKEN_PITCH_X: f64 = 42.0;
const TOKEN_PITCH_Y: f64 = 46.0;

const SWAB_TOOL_CENTER: (f64, f64) = (405.0, -65.0);
const SWAB_TOOL_X: f64 = 340.0;
const SWAB_TOOL_Y: f64 = 220.0;
const SWAB_TOOL_Z: f64 = 50.0;
const CLEAN_SWAB_COUNT: usize = 12;
const RECOVERY_VIAL_COUNT: usize = 12;
const SWAB_QUILL_PITCH_Y: f64 = 30.0;

const BARCODE_CENTER: (f64, f64) = (-405.0, -310.0);
const BARCODE_X: f64 = 350.0;
const BARCODE_Y: f64 = 100.0;
const BARCODE_Z: f64 = 14.0;
const BARCODE_TAG_COUNT: usize = 20;
const CUSTODY_PUNCH_COUNT: usize = 10;

const DISPOSITION_CENTER: (f64, f64) = (0.0, -310.0);
const DISPOSITION_X: f64 = 360.0;
const DISPOSITION_Y: f64 = 100.0;
const DISPOSITION_Z: f64 = 28.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 6;

const BRIDGE_SPAN_X: f64 = 1110.0;
const BRIDGE_SPAN_Y: f64 = 650.0;
const BRIDGE_POST_W: f64 = 16.0;
const BRIDGE_UNDERSIDE_Z: f64 = 188.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_WINDOW_X: f64 = 520.0;
const CAMERA_WINDOW_Y: f64 = 285.0;
const CAMERA_PLATE_X: f64 = 210.0;
const CAMERA_PLATE_Y: f64 = 92.0;
const CAMERA_LENS_D: f64 = 36.0;
const EVIDENCE_FIDUCIAL_COUNT: usize = 12;

const FRONT_ROBOT_APPROACH_Y: f64 = 430.0;
const SIDE_GRIPPER_SERVICE_X: f64 = 250.0;
const RINSE_DRAWER_SERVICE_Y: f64 = 240.0;
const ROBOT_Z_CLEARANCE: f64 = 365.0;
const APPROACH_GAUGE_COUNT: usize = 8;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(&base, OUTPUTS[0]);

    let jaw_nests = gripper_jaw_nests();
    export(&jaw_nests, OUTPUTS[1]);

    let coupons = swab_recovery_coupon_grid();
    export(&coupons, OUTPUTS[2]);

    let rinse = rinse_capture_channels();
    export(&rinse, OUTPUTS[3]);

    let segregation = clean_dirty_segregation_bulkhead();
    export(&segregation, OUTPUTS[4]);

    let witnesses = residue_witness_token_bank();
    export(&witnesses, OUTPUTS[5]);

    let barcode = barcode_custody_tag_lands();
    export(&barcode, OUTPUTS[6]);

    let swab_tools = swab_tool_recovery_quiver();
    export(&swab_tools, OUTPUTS[7]);

    let disposition = custody_disposition_lanes();
    export(&disposition, OUTPUTS[8]);

    let bridge = camera_evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let gauges = robot_approach_gauges();
    export(&gauges, OUTPUTS[10]);

    let assembly = base
        + jaw_nests.translate(JAW_NEST_CENTER.0, JAW_NEST_CENTER.1, insert_z())
        + coupons.translate(COUPON_GRID_CENTER.0, COUPON_GRID_CENTER.1, insert_z())
        + rinse.translate(RINSE_CENTER.0, RINSE_CENTER.1, insert_z())
        + segregation.translate(SEGREGATION_CENTER.0, SEGREGATION_CENTER.1, insert_z())
        + witnesses.translate(WITNESS_CENTER.0, WITNESS_CENTER.1, insert_z())
        + barcode.translate(BARCODE_CENTER.0, BARCODE_CENTER.1, insert_z())
        + swab_tools.translate(SWAB_TOOL_CENTER.0, SWAB_TOOL_CENTER.1, insert_z())
        + disposition.translate(DISPOSITION_CENTER.0, DISPOSITION_CENTER.1, insert_z())
        + bridge.translate(0.0, 0.0, BASE_Z)
        + gauges.translate(0.0, 0.0, BASE_Z);
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed robotic gripper decon residue swab recovery station:");
    println!("  Generator:               {BIN_PREFIX}");
    println!(
        "  Deck:                    {DECK_X:.0}mm x {DECK_Y:.0}mm closed tray with raised rim, sump, sockets, and datum targets"
    );
    println!(
        "  Gripper jaw nests:       {JAW_NEST_COUNT} indexed nests across {JAW_PAIR_COUNT} jaw pairs with go/no-go and spray-shadow gauges"
    );
    println!(
        "  Swab recovery:           {COUPON_COUNT} coupon pockets in a {COUPON_COLS} x {COUPON_ROWS} grid plus {RINSE_CHANNEL_COUNT} rinse capture channels and {RINSE_VIAL_COUNT} vial docks"
    );
    println!(
        "  Segregation/custody:     {CLEAN_SWAB_SLOTS} clean slots, {DIRTY_RETURN_SLOTS} dirty returns, {BARCODE_TAG_COUNT} barcode tags, {CUSTODY_PUNCH_COUNT} custody punch lands"
    );
    println!(
        "  Witness tokens:          {POSITIVE_TOKEN_COUNT} positive, {NEGATIVE_TOKEN_COUNT} negative, {BLANK_TOKEN_COUNT} blank residue tokens"
    );
    println!(
        "  Evidence/robotics:       {EVIDENCE_FIDUCIAL_COUNT} camera fiducials, {APPROACH_GAUGE_COUNT} approach gauges, {ROBOT_Z_CLEARANCE:.0}mm robot Z clearance witness"
    );
    println!("  Feature groups covered:  {}", REQUIRED_FEATURES.len());
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected output count");
    assert_eq!(COUPON_COUNT, COUPON_COLS * COUPON_ROWS);
    assert_eq!(JAW_NEST_COUNT, JAW_PAIR_COUNT * JAWS_PER_PAIR);
    assert_eq!(
        POSITIVE_TOKEN_COUNT, NEGATIVE_TOKEN_COUNT,
        "positive and negative witness token counts should balance"
    );
    assert_eq!(RINSE_CHANNEL_COUNT, RINSE_VIAL_COUNT);
    assert_eq!(STATUS_LANES, 3);
    assert!(BULKHEAD_Z > SEGREGATION_Z);
    assert!(DATUM_PIN_D >= 6.0);
    assert!(highest_fixture_feature_z() < ROBOT_Z_CLEARANCE);
    assert!(
        BRIDGE_SPAN_X + 70.0 < DECK_X - 2.0 * RIM_W && BRIDGE_SPAN_Y + 70.0 < DECK_Y - 2.0 * RIM_W,
        "camera bridge feet must land inside the containment rim"
    );

    let footprints = component_footprints();
    for footprint in footprints {
        assert!(
            fits_on_deck(footprint),
            "{} exceeds the usable closed tray",
            footprint.name
        );
    }

    for (i, left) in footprints.iter().enumerate() {
        for right in footprints.iter().skip(i + 1) {
            assert!(
                footprints_clear(*left, *right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn highest_fixture_feature_z() -> f64 {
    BASE_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z
}

fn component_footprints() -> [Footprint; 8] {
    [
        fp("gripper_jaw_nests", JAW_NEST_CENTER, JAW_NEST_X, JAW_NEST_Y),
        fp(
            "swab_recovery_coupon_grid",
            COUPON_GRID_CENTER,
            COUPON_GRID_X,
            COUPON_GRID_Y,
        ),
        fp("rinse_capture_channels", RINSE_CENTER, RINSE_X, RINSE_Y),
        fp(
            "clean_dirty_segregation_bulkhead",
            SEGREGATION_CENTER,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        fp(
            "residue_witness_token_bank",
            WITNESS_CENTER,
            WITNESS_X,
            WITNESS_Y,
        ),
        fp(
            "swab_tool_recovery_quiver",
            SWAB_TOOL_CENTER,
            SWAB_TOOL_X,
            SWAB_TOOL_Y,
        ),
        fp(
            "barcode_custody_tag_lands",
            BARCODE_CENTER,
            BARCODE_X,
            BARCODE_Y,
        ),
        fp(
            "custody_disposition_lanes",
            DISPOSITION_CENTER,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
    ]
}

fn fp(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_on_deck(footprint: Footprint) -> bool {
    let usable_x = DECK_X / 2.0 - RIM_W - 8.0;
    let usable_y = DECK_Y / 2.0 - RIM_W - 8.0;
    footprint.center.0.abs() + footprint.x / 2.0 <= usable_x
        && footprint.center.1.abs() + footprint.y / 2.0 <= usable_y
}

fn footprints_clear(left: Footprint, right: Footprint) -> bool {
    let dx = (left.center.0 - right.center.0).abs();
    let dy = (left.center.1 - right.center.1).abs();
    dx >= left.x / 2.0 + right.x / 2.0 + MODULE_GAP
        || dy >= left.y / 2.0 + right.y / 2.0 + MODULE_GAP
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "gripper_decon_swab_station_closed_base_deck",
        DECK_X,
        DECK_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let wipe_recess = centered_cube(
        "gripper_decon_swab_station_recessed_wipeable_floor",
        DECK_X - 126.0,
        DECK_Y - 118.0,
        7.0,
    )
    .translate(0.0, 0.0, BASE_Z - 3.5);
    let front_sump = centered_cube(
        "gripper_decon_swab_station_front_rinse_sump",
        DECK_X - 210.0,
        42.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 60.0, BASE_Z - 4.0);
    let drain = centered_cylinder(
        "gripper_decon_swab_station_closed_drain_capture_port",
        8.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 38.0, BASE_Z - 7.0);

    deck - wipe_recess - front_sump - drain - insert_sockets() - mount_holes()
        + perimeter_rims()
        + base_zone_dividers()
        + base_datum_targets()
        + module_transfer_lips()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("gripper_decon_swab_station_insert_sockets");
    for footprint in component_footprints() {
        sockets = sockets
            + centered_cube(
                format!("gripper_decon_swab_station_{}_socket", footprint.name),
                footprint.x + 10.0,
                footprint.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("gripper_decon_swab_station_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("gripper_decon_swab_station_m6_clearance_{i}"),
                3.4,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("gripper_decon_swab_station_mount_slot_relief_{i}"),
                26.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    centered_cube(
        "gripper_decon_swab_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0)
        + centered_cube(
            "gripper_decon_swab_station_rear_containment_rim",
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0)
        + centered_cube(
            "gripper_decon_swab_station_left_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0)
        + centered_cube(
            "gripper_decon_swab_station_right_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0)
}

fn base_zone_dividers() -> Part {
    let top_to_middle = centered_cube(
        "gripper_decon_swab_station_swab_to_witness_zone_divider",
        DECK_X - 154.0,
        10.0,
        24.0,
    )
    .translate(0.0, 72.0, BASE_Z + 12.0);
    let middle_to_custody = centered_cube(
        "gripper_decon_swab_station_witness_to_custody_zone_divider",
        DECK_X - 180.0,
        10.0,
        24.0,
    )
    .translate(0.0, -215.0, BASE_Z + 12.0);
    let clean_dirty_spine = centered_cube(
        "gripper_decon_swab_station_clean_dirty_centerline_spine",
        10.0,
        480.0,
        26.0,
    )
    .translate(-205.0, -70.0, BASE_Z + 13.0);
    top_to_middle + middle_to_custody + clean_dirty_spine
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("gripper_decon_swab_station_base_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 96.0, -DECK_Y / 2.0 + 96.0),
        (DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 96.0),
        (-DECK_X / 2.0 + 96.0, DECK_Y / 2.0 - 96.0),
        (DECK_X / 2.0 - 96.0, DECK_Y / 2.0 - 96.0),
        (-205.0, 72.0),
        (205.0, 72.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("gripper_decon_swab_station_robot_datum_boss_{i}"),
            13.0,
            7.0,
            36,
        )
        .translate(x, y, BASE_Z + 3.5);
        let center = centered_cylinder(
            format!("gripper_decon_swab_station_robot_datum_center_mark_{i}"),
            DATUM_PIN_D / 4.0,
            8.0,
            20,
        )
        .translate(x, y, BASE_Z + 3.5);
        targets = targets + (boss - center);
    }
    targets
}

fn module_transfer_lips() -> Part {
    let clean_lip = centered_cube(
        "gripper_decon_swab_station_clean_swab_transfer_lip",
        SEGREGATION_X - 46.0,
        12.0,
        16.0,
    )
    .translate(
        SEGREGATION_CENTER.0,
        SEGREGATION_CENTER.1 - SEGREGATION_Y / 2.0 - 10.0,
        BASE_Z + 8.0,
    );
    let rinse_lip = centered_cube(
        "gripper_decon_swab_station_rinse_drawer_transfer_lip",
        RINSE_X - 42.0,
        12.0,
        16.0,
    )
    .translate(
        RINSE_CENTER.0,
        RINSE_CENTER.1 - RINSE_Y / 2.0 - 10.0,
        BASE_Z + 8.0,
    );
    let custody_lip = centered_cube(
        "gripper_decon_swab_station_custody_tag_transfer_lip",
        BARCODE_X - 40.0,
        12.0,
        14.0,
    )
    .translate(
        BARCODE_CENTER.0,
        BARCODE_CENTER.1 + BARCODE_Y / 2.0 + 10.0,
        BASE_Z + 7.0,
    );
    clean_lip + rinse_lip + custody_lip
}

fn gripper_jaw_nests() -> Part {
    let body = module_panel(
        "gripper_decon_jaw_nest_body",
        JAW_NEST_X,
        JAW_NEST_Y,
        JAW_NEST_Z,
    );
    let recessed_field = centered_cube(
        "gripper_decon_jaw_nest_wipeable_recessed_field",
        JAW_NEST_X - 36.0,
        JAW_NEST_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, JAW_NEST_Z - 5.0);
    body - recessed_field - jaw_slot_cuts()
        + jaw_locator_lips()
        + jaw_opening_go_no_go_gauges()
        + spray_shadow_witness_ribs()
        + pick_fiducials("gripper_decon_jaw_nest", JAW_NEST_X, JAW_NEST_Y, JAW_NEST_Z)
}

fn jaw_slot_cuts() -> Part {
    let mut cuts = Part::empty("gripper_decon_jaw_nest_slot_cuts");
    for pair in 0..JAW_PAIR_COUNT {
        for jaw in 0..JAWS_PER_PAIR {
            let i = pair * JAWS_PER_PAIR + jaw;
            let x = index_offset(jaw, JAWS_PER_PAIR, JAW_SLOT_PITCH_X);
            let y = index_offset(pair, JAW_PAIR_COUNT, JAW_SLOT_PITCH_Y);
            cuts = cuts
                + centered_cube(
                    format!("gripper_decon_jaw_nest_jaw_slot_{i}"),
                    JAW_SLOT_X,
                    JAW_SLOT_Y,
                    34.0,
                )
                .translate(x, y, JAW_NEST_Z - 15.0)
                + centered_cube(
                    format!("gripper_decon_jaw_nest_pad_face_recess_{i}"),
                    JAW_SLOT_X - 22.0,
                    12.0,
                    18.0,
                )
                .translate(x, y + 19.0, JAW_NEST_Z - 9.0);
        }
    }
    cuts
}

fn jaw_locator_lips() -> Part {
    let mut lips = Part::empty("gripper_decon_jaw_nest_locator_lips");
    for pair in 0..JAW_PAIR_COUNT {
        for jaw in 0..JAWS_PER_PAIR {
            let i = pair * JAWS_PER_PAIR + jaw;
            let x = index_offset(jaw, JAWS_PER_PAIR, JAW_SLOT_PITCH_X);
            let y = index_offset(pair, JAW_PAIR_COUNT, JAW_SLOT_PITCH_Y);
            lips = lips
                + centered_cube(
                    format!("gripper_decon_jaw_nest_left_locator_lip_{i}"),
                    5.0,
                    JAW_SLOT_Y + 20.0,
                    18.0,
                )
                .translate(x - JAW_SLOT_X / 2.0 - 6.0, y, JAW_NEST_Z + 9.0)
                + centered_cube(
                    format!("gripper_decon_jaw_nest_right_locator_lip_{i}"),
                    5.0,
                    JAW_SLOT_Y + 20.0,
                    18.0,
                )
                .translate(x + JAW_SLOT_X / 2.0 + 6.0, y, JAW_NEST_Z + 9.0)
                + centered_cube(
                    format!("gripper_decon_jaw_nest_end_stop_lip_{i}"),
                    JAW_SLOT_X + 20.0,
                    5.0,
                    16.0,
                )
                .translate(x, y - JAW_SLOT_Y / 2.0 - 8.0, JAW_NEST_Z + 8.0);
        }
    }
    lips
}

fn jaw_opening_go_no_go_gauges() -> Part {
    let mut gauges = Part::empty("gripper_decon_jaw_nest_go_no_go_gauges");
    for (i, y) in [-78.0, 78.0].into_iter().enumerate() {
        gauges = gauges
            + centered_cube(
                format!("gripper_decon_jaw_nest_min_opening_gauge_{i}"),
                42.0,
                8.0,
                18.0,
            )
            .translate(-6.0, y, JAW_NEST_Z + 9.0)
            + centered_cube(
                format!("gripper_decon_jaw_nest_max_opening_gauge_{i}"),
                76.0,
                8.0,
                18.0,
            )
            .translate(64.0, y, JAW_NEST_Z + 9.0);
    }
    gauges
}

fn spray_shadow_witness_ribs() -> Part {
    let mut ribs = Part::empty("gripper_decon_jaw_nest_spray_shadow_ribs");
    for i in 0..6 {
        ribs = ribs
            + centered_cube(
                format!("gripper_decon_jaw_nest_decon_shadow_rib_{i}"),
                4.0,
                JAW_NEST_Y - 54.0,
                10.0,
            )
            .translate(index_offset(i, 6, 28.0), 0.0, JAW_NEST_Z + 5.0);
    }
    ribs
}

fn swab_recovery_coupon_grid() -> Part {
    let body = module_panel(
        "gripper_decon_swab_coupon_grid_body",
        COUPON_GRID_X,
        COUPON_GRID_Y,
        COUPON_GRID_Z,
    );
    let field = centered_cube(
        "gripper_decon_swab_coupon_grid_recessed_field",
        COUPON_GRID_X - 34.0,
        COUPON_GRID_Y - 32.0,
        9.0,
    )
    .translate(0.0, 0.0, COUPON_GRID_Z - 4.5);
    body - field - coupon_pocket_cuts()
        + coupon_locator_lips()
        + swab_stroke_guides()
        + coupon_grid_index_ticks()
        + pick_fiducials(
            "gripper_decon_swab_coupon_grid",
            COUPON_GRID_X,
            COUPON_GRID_Y,
            COUPON_GRID_Z,
        )
}

fn coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gripper_decon_coupon_pocket_cuts");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let i = row * COUPON_COLS + col;
            cuts = cuts
                + centered_cube(
                    format!("gripper_decon_coupon_recovery_pocket_{i}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    18.0,
                )
                .translate(
                    index_offset(col, COUPON_COLS, COUPON_PITCH_X),
                    index_offset(row, COUPON_ROWS, COUPON_PITCH_Y),
                    COUPON_GRID_Z - 8.0,
                );
        }
    }
    cuts
}

fn coupon_locator_lips() -> Part {
    let mut lips = Part::empty("gripper_decon_coupon_locator_lips");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let i = row * COUPON_COLS + col;
            let x = index_offset(col, COUPON_COLS, COUPON_PITCH_X);
            let y = index_offset(row, COUPON_ROWS, COUPON_PITCH_Y);
            lips = lips
                + centered_cube(
                    format!("gripper_decon_coupon_lower_retention_lip_{i}"),
                    COUPON_SLOT_X + 8.0,
                    4.0,
                    8.0,
                )
                .translate(x, y - COUPON_SLOT_Y / 2.0 - 4.0, COUPON_GRID_Z + 4.0)
                + centered_cube(
                    format!("gripper_decon_coupon_barcode_edge_lip_{i}"),
                    4.0,
                    COUPON_SLOT_Y + 8.0,
                    8.0,
                )
                .translate(x + COUPON_SLOT_X / 2.0 + 4.0, y, COUPON_GRID_Z + 4.0);
        }
    }
    lips
}

fn swab_stroke_guides() -> Part {
    let mut guides = Part::empty("gripper_decon_swab_stroke_guides");
    for row in 0..COUPON_ROWS {
        let y = index_offset(row, COUPON_ROWS, COUPON_PITCH_Y);
        guides = guides
            + centered_cube(
                format!("gripper_decon_coupon_row_swab_start_rail_{row}"),
                COUPON_GRID_X - 70.0,
                3.0,
                7.0,
            )
            .translate(0.0, y - 16.0, COUPON_GRID_Z + 3.5)
            + centered_cube(
                format!("gripper_decon_coupon_row_swab_end_rail_{row}"),
                COUPON_GRID_X - 70.0,
                3.0,
                7.0,
            )
            .translate(0.0, y + 16.0, COUPON_GRID_Z + 3.5);
    }
    guides
}

fn coupon_grid_index_ticks() -> Part {
    let mut ticks = Part::empty("gripper_decon_coupon_grid_index_ticks");
    for col in 0..COUPON_COLS {
        ticks = ticks
            + centered_cube(
                format!("gripper_decon_coupon_column_index_tick_{col}"),
                24.0,
                4.0,
                5.0,
            )
            .translate(
                index_offset(col, COUPON_COLS, COUPON_PITCH_X),
                COUPON_GRID_Y / 2.0 - 20.0,
                COUPON_GRID_Z + 2.5,
            );
    }
    for row in 0..COUPON_ROWS {
        ticks = ticks
            + centered_cube(
                format!("gripper_decon_coupon_row_index_tick_{row}"),
                4.0,
                20.0,
                5.0,
            )
            .translate(
                -COUPON_GRID_X / 2.0 + 22.0,
                index_offset(row, COUPON_ROWS, COUPON_PITCH_Y),
                COUPON_GRID_Z + 2.5,
            );
    }
    ticks
}

fn rinse_capture_channels() -> Part {
    let body = module_panel(
        "gripper_decon_rinse_capture_body",
        RINSE_X,
        RINSE_Y,
        RINSE_Z,
    );
    body - rinse_channel_recesses() - rinse_drain_holes()
        + rinse_channel_lips()
        + rinse_vial_collars()
        + rinse_gradient_ticks()
        + pick_fiducials("gripper_decon_rinse_capture", RINSE_X, RINSE_Y, RINSE_Z)
}

fn rinse_channel_recesses() -> Part {
    let mut recesses = Part::empty("gripper_decon_rinse_channel_recesses");
    for channel in 0..RINSE_CHANNEL_COUNT {
        let x = index_offset(channel, RINSE_CHANNEL_COUNT, RINSE_CHANNEL_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("gripper_decon_rinse_capture_channel_recess_{channel}"),
                RINSE_CHANNEL_X,
                RINSE_CHANNEL_Y,
                20.0,
            )
            .translate(x, 8.0, RINSE_Z - 9.0)
            + centered_cube(
                format!("gripper_decon_rinse_capture_funnel_recess_{channel}"),
                RINSE_CHANNEL_X + 18.0,
                30.0,
                18.0,
            )
            .translate(x, -RINSE_Y / 2.0 + 40.0, RINSE_Z - 8.0);
    }
    recesses
}

fn rinse_drain_holes() -> Part {
    let mut holes = Part::empty("gripper_decon_rinse_drain_holes");
    for channel in 0..RINSE_CHANNEL_COUNT {
        holes = holes
            + centered_cylinder(
                format!("gripper_decon_rinse_capture_drain_hole_{channel}"),
                5.0,
                RINSE_Z + 4.0,
                24,
            )
            .translate(
                index_offset(channel, RINSE_CHANNEL_COUNT, RINSE_CHANNEL_PITCH_X),
                -RINSE_Y / 2.0 + 30.0,
                RINSE_Z / 2.0,
            );
    }
    holes
}

fn rinse_channel_lips() -> Part {
    let mut lips = Part::empty("gripper_decon_rinse_channel_lips");
    for channel in 0..RINSE_CHANNEL_COUNT {
        let x = index_offset(channel, RINSE_CHANNEL_COUNT, RINSE_CHANNEL_PITCH_X);
        lips = lips
            + centered_cube(
                format!("gripper_decon_rinse_channel_left_lip_{channel}"),
                4.0,
                RINSE_CHANNEL_Y + 20.0,
                12.0,
            )
            .translate(x - RINSE_CHANNEL_X / 2.0 - 5.0, 8.0, RINSE_Z + 6.0)
            + centered_cube(
                format!("gripper_decon_rinse_channel_right_lip_{channel}"),
                4.0,
                RINSE_CHANNEL_Y + 20.0,
                12.0,
            )
            .translate(x + RINSE_CHANNEL_X / 2.0 + 5.0, 8.0, RINSE_Z + 6.0);
    }
    lips
}

fn rinse_vial_collars() -> Part {
    let mut collars = Part::empty("gripper_decon_rinse_vial_collars");
    for vial in 0..RINSE_VIAL_COUNT {
        let x = index_offset(vial, RINSE_VIAL_COUNT, RINSE_CHANNEL_PITCH_X);
        collars = collars
            + annular_collar(
                &format!("gripper_decon_rinse_capture_vial_collar_{vial}"),
                30.0,
                19.0,
                12.0,
            )
            .translate(x, -RINSE_Y / 2.0 + 30.0, RINSE_Z + 6.0);
    }
    collars
}

fn rinse_gradient_ticks() -> Part {
    let mut ticks = Part::empty("gripper_decon_rinse_gradient_ticks");
    for channel in 0..RINSE_CHANNEL_COUNT {
        let x = index_offset(channel, RINSE_CHANNEL_COUNT, RINSE_CHANNEL_PITCH_X);
        for tick in 0..5 {
            ticks = ticks
                + centered_cube(
                    format!("gripper_decon_rinse_channel_{channel}_volume_tick_{tick}"),
                    18.0,
                    2.5,
                    4.0,
                )
                .translate(
                    x,
                    -RINSE_Y / 2.0 + 60.0 + tick as f64 * 24.0,
                    RINSE_Z + 2.0,
                );
        }
    }
    ticks
}

fn clean_dirty_segregation_bulkhead() -> Part {
    let body = module_panel(
        "gripper_decon_clean_dirty_body",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let bulkhead = centered_cube(
        "gripper_decon_clean_dirty_vertical_bulkhead",
        14.0,
        SEGREGATION_Y - 30.0,
        BULKHEAD_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_Z / 2.0);
    body + bulkhead
        + clean_side_swab_slots()
        + dirty_side_return_cups()
        + one_way_residue_gate()
        + segregation_status_flags()
}

fn clean_side_swab_slots() -> Part {
    let mut slots = Part::empty("gripper_decon_clean_side_swab_slots");
    for slot in 0..CLEAN_SWAB_SLOTS {
        slots = slots
            + centered_cube(
                format!("gripper_decon_clean_side_wrapped_swab_slot_{slot}"),
                46.0,
                8.0,
                22.0,
            )
            .translate(
                -SEGREGATION_X / 4.0,
                index_offset(slot, CLEAN_SWAB_SLOTS, 22.0),
                SEGREGATION_Z + 11.0,
            );
    }
    slots
}

fn dirty_side_return_cups() -> Part {
    let mut cups = Part::empty("gripper_decon_dirty_side_return_cups");
    for slot in 0..DIRTY_RETURN_SLOTS {
        let cup = centered_cylinder(
            format!("gripper_decon_dirty_side_used_swab_cup_{slot}"),
            12.0,
            18.0,
            28,
        );
        let bore = centered_cylinder(
            format!("gripper_decon_dirty_side_used_swab_cup_bore_{slot}"),
            8.0,
            20.0,
            28,
        );
        cups = cups
            + (cup - bore).translate(
                SEGREGATION_X / 4.0,
                index_offset(slot, DIRTY_RETURN_SLOTS, 22.0),
                SEGREGATION_Z + 9.0,
            );
    }
    cups
}

fn one_way_residue_gate() -> Part {
    let chute = centered_cube(
        "gripper_decon_clean_dirty_one_way_swab_pass_chute",
        116.0,
        28.0,
        30.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 42.0, SEGREGATION_Z + 15.0);
    let window = centered_cube(
        "gripper_decon_clean_dirty_pass_chute_witness_window",
        76.0,
        32.0,
        12.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 42.0, SEGREGATION_Z + 15.0);
    let drip_lip = centered_cube(
        "gripper_decon_clean_dirty_pass_chute_drip_lip",
        138.0,
        8.0,
        8.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 22.0, SEGREGATION_Z + 4.0);
    (chute - window) + drip_lip
}

fn segregation_status_flags() -> Part {
    let clean_flag = centered_cube(
        "gripper_decon_clean_side_green_status_land",
        82.0,
        16.0,
        5.0,
    )
    .translate(
        -SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 26.0,
        SEGREGATION_Z + 2.5,
    );
    let dirty_flag = centered_cube("gripper_decon_dirty_side_red_status_land", 82.0, 16.0, 5.0)
        .translate(
            SEGREGATION_X / 4.0,
            SEGREGATION_Y / 2.0 - 26.0,
            SEGREGATION_Z + 2.5,
        );
    clean_flag + dirty_flag
}

fn residue_witness_token_bank() -> Part {
    let body = module_panel(
        "gripper_decon_residue_witness_token_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    body - witness_token_wells()
        + witness_token_collars()
        + witness_zone_separators()
        + witness_lot_tag_lands()
}

fn witness_token_wells() -> Part {
    let mut wells = Part::empty("gripper_decon_residue_witness_token_wells");
    for i in 0..POSITIVE_TOKEN_COUNT {
        wells = wells + token_well_cut("positive", i, -92.0, i);
    }
    for i in 0..NEGATIVE_TOKEN_COUNT {
        wells = wells + token_well_cut("negative", i, 92.0, i);
    }
    for i in 0..BLANK_TOKEN_COUNT {
        wells = wells + token_well_cut("blank", i, 0.0, i);
    }
    wells
}

fn token_well_cut(kind: &str, index: usize, x_center: f64, index_in_group: usize) -> Part {
    centered_cylinder(
        format!("gripper_decon_{kind}_residue_token_well_{index}"),
        TOKEN_D / 2.0,
        18.0,
        32,
    )
    .translate(
        x_center + index_offset(index_in_group % 2, 2, TOKEN_PITCH_X),
        index_offset(index_in_group / 2, 4, TOKEN_PITCH_Y),
        WITNESS_Z - 8.0,
    )
}

fn witness_token_collars() -> Part {
    let mut collars = Part::empty("gripper_decon_residue_witness_token_collars");
    for i in 0..POSITIVE_TOKEN_COUNT {
        collars = collars + token_collar("positive", i, -92.0, i);
    }
    for i in 0..NEGATIVE_TOKEN_COUNT {
        collars = collars + token_collar("negative", i, 92.0, i);
    }
    for i in 0..BLANK_TOKEN_COUNT {
        collars = collars + token_collar("blank", i, 0.0, i);
    }
    collars
}

fn token_collar(kind: &str, index: usize, x_center: f64, index_in_group: usize) -> Part {
    annular_collar(
        &format!("gripper_decon_{kind}_residue_token_retention_collar_{index}"),
        TOKEN_D + 10.0,
        TOKEN_D + 1.0,
        8.0,
    )
    .translate(
        x_center + index_offset(index_in_group % 2, 2, TOKEN_PITCH_X),
        index_offset(index_in_group / 2, 4, TOKEN_PITCH_Y),
        WITNESS_Z + 4.0,
    )
}

fn witness_zone_separators() -> Part {
    centered_cube(
        "gripper_decon_positive_to_blank_witness_separator",
        8.0,
        WITNESS_Y - 36.0,
        18.0,
    )
    .translate(-46.0, 0.0, WITNESS_Z + 9.0)
        + centered_cube(
            "gripper_decon_blank_to_negative_witness_separator",
            8.0,
            WITNESS_Y - 36.0,
            18.0,
        )
        .translate(46.0, 0.0, WITNESS_Z + 9.0)
}

fn witness_lot_tag_lands() -> Part {
    let mut lands = Part::empty("gripper_decon_witness_lot_tag_lands");
    for i in 0..6 {
        lands = lands
            + centered_cube(
                format!("gripper_decon_witness_lot_tag_land_{i}"),
                38.0,
                12.0,
                4.0,
            )
            .translate(
                index_offset(i, 6, 48.0),
                -WITNESS_Y / 2.0 + 16.0,
                WITNESS_Z + 2.0,
            );
    }
    lands
}

fn barcode_custody_tag_lands() -> Part {
    let body = module_panel(
        "gripper_decon_barcode_custody_body",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    body + barcode_tag_lands() + custody_punch_lands() + scan_alignment_ticks()
}

fn barcode_tag_lands() -> Part {
    let mut lands = Part::empty("gripper_decon_barcode_custody_tag_lands");
    for tag in 0..BARCODE_TAG_COUNT {
        let x = index_offset(tag % 5, 5, 58.0);
        let y = index_offset(tag / 5, 4, 20.0);
        lands = lands
            + centered_cube(
                format!("gripper_decon_barcode_custody_tag_land_{tag}"),
                42.0,
                13.0,
                3.0,
            )
            .translate(x, y, BARCODE_Z + 1.5)
            + barcode_stripes("gripper_decon_barcode_custody", tag, x, y, BARCODE_Z + 3.2);
    }
    lands
}

fn custody_punch_lands() -> Part {
    let mut punches = Part::empty("gripper_decon_custody_chain_punch_lands");
    for i in 0..CUSTODY_PUNCH_COUNT {
        punches = punches
            + centered_cylinder(
                format!("gripper_decon_custody_chain_punch_land_{i}"),
                4.0,
                3.0,
                22,
            )
            .translate(
                BARCODE_X / 2.0 - 28.0,
                index_offset(i, CUSTODY_PUNCH_COUNT, 8.0),
                BARCODE_Z + 1.5,
            );
    }
    punches
}

fn scan_alignment_ticks() -> Part {
    centered_cube(
        "gripper_decon_barcode_scan_left_alignment_tick",
        5.0,
        BARCODE_Y - 20.0,
        4.0,
    )
    .translate(-BARCODE_X / 2.0 + 22.0, 0.0, BARCODE_Z + 2.0)
        + centered_cube(
            "gripper_decon_barcode_scan_right_alignment_tick",
            5.0,
            BARCODE_Y - 20.0,
            4.0,
        )
        .translate(BARCODE_X / 2.0 - 22.0, 0.0, BARCODE_Z + 2.0)
}

fn swab_tool_recovery_quiver() -> Part {
    let body = module_panel(
        "gripper_decon_swab_tool_quiver_body",
        SWAB_TOOL_X,
        SWAB_TOOL_Y,
        SWAB_TOOL_Z,
    );
    body - clean_swab_quiver_cuts() - recovery_vial_wells()
        + swab_quiver_divider()
        + recovery_vial_collars()
        + swab_cap_parking_lands()
        + pick_fiducials(
            "gripper_decon_swab_tool_quiver",
            SWAB_TOOL_X,
            SWAB_TOOL_Y,
            SWAB_TOOL_Z,
        )
}

fn clean_swab_quiver_cuts() -> Part {
    let mut cuts = Part::empty("gripper_decon_clean_swab_quiver_cuts");
    for i in 0..CLEAN_SWAB_COUNT {
        cuts = cuts
            + centered_cube(
                format!("gripper_decon_clean_wrapped_swab_quiver_slot_{i}"),
                72.0,
                8.0,
                26.0,
            )
            .translate(
                -SWAB_TOOL_X / 4.0,
                index_offset(i, CLEAN_SWAB_COUNT, SWAB_QUILL_PITCH_Y / 1.4),
                SWAB_TOOL_Z - 12.0,
            );
    }
    cuts
}

fn recovery_vial_wells() -> Part {
    let mut wells = Part::empty("gripper_decon_recovery_vial_wells");
    for vial in 0..RECOVERY_VIAL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("gripper_decon_swab_recovery_vial_well_{vial}"),
                9.5,
                26.0,
                28,
            )
            .translate(
                SWAB_TOOL_X / 4.0 + index_offset(vial % 2, 2, 36.0),
                index_offset(vial / 2, 6, 31.0),
                SWAB_TOOL_Z - 12.0,
            );
    }
    wells
}

fn swab_quiver_divider() -> Part {
    centered_cube(
        "gripper_decon_swab_tool_clean_dirty_quiver_divider",
        10.0,
        SWAB_TOOL_Y - 30.0,
        44.0,
    )
    .translate(0.0, 0.0, SWAB_TOOL_Z + 22.0)
}

fn recovery_vial_collars() -> Part {
    let mut collars = Part::empty("gripper_decon_recovery_vial_collars");
    for vial in 0..RECOVERY_VIAL_COUNT {
        collars = collars
            + annular_collar(
                &format!("gripper_decon_swab_recovery_vial_collar_{vial}"),
                25.0,
                18.0,
                8.0,
            )
            .translate(
                SWAB_TOOL_X / 4.0 + index_offset(vial % 2, 2, 36.0),
                index_offset(vial / 2, 6, 31.0),
                SWAB_TOOL_Z + 4.0,
            );
    }
    collars
}

fn swab_cap_parking_lands() -> Part {
    let mut lands = Part::empty("gripper_decon_swab_cap_parking_lands");
    for i in 0..6 {
        lands = lands
            + centered_cube(
                format!("gripper_decon_recovery_vial_cap_park_land_{i}"),
                28.0,
                12.0,
                4.0,
            )
            .translate(
                index_offset(i, 6, 42.0),
                -SWAB_TOOL_Y / 2.0 + 18.0,
                SWAB_TOOL_Z + 2.0,
            );
    }
    lands
}

fn custody_disposition_lanes() -> Part {
    let body = module_panel(
        "gripper_decon_custody_disposition_body",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    body - disposition_lane_recesses() + disposition_lane_floors() + disposition_lane_gates()
}

fn disposition_lane_recesses() -> Part {
    let mut recesses = Part::empty("gripper_decon_disposition_lane_recesses");
    for lane in 0..STATUS_LANES {
        recesses = recesses
            + centered_cube(
                format!("gripper_decon_disposition_lane_recess_{lane}"),
                92.0,
                DISPOSITION_Y - 24.0,
                12.0,
            )
            .translate(
                index_offset(lane, STATUS_LANES, 108.0),
                0.0,
                DISPOSITION_Z - 5.0,
            );
    }
    recesses
}

fn disposition_lane_floors() -> Part {
    let mut floors = Part::empty("gripper_decon_disposition_lane_floors");
    for (lane, name) in ["release", "hold", "retest"].into_iter().enumerate() {
        let x = index_offset(lane, STATUS_LANES, 108.0);
        floors = floors
            + centered_cube(
                format!("gripper_decon_{name}_lane_witness_floor"),
                74.0,
                DISPOSITION_Y - 36.0,
                3.0,
            )
            .translate(x, 0.0, DISPOSITION_Z + 1.5);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            floors = floors
                + centered_cube(
                    format!("gripper_decon_{name}_lane_token_slot_{slot}"),
                    48.0,
                    5.0,
                    5.0,
                )
                .translate(
                    x,
                    index_offset(slot, STATUS_SLOTS_PER_LANE, 12.0),
                    DISPOSITION_Z + 2.5,
                );
        }
    }
    floors
}

fn disposition_lane_gates() -> Part {
    let mut gates = Part::empty("gripper_decon_disposition_lane_gates");
    for lane in 0..STATUS_LANES {
        let x = index_offset(lane, STATUS_LANES, 108.0);
        gates = gates
            + centered_cube(
                format!("gripper_decon_disposition_lane_front_gate_{lane}"),
                82.0,
                5.0,
                14.0,
            )
            .translate(x, -DISPOSITION_Y / 2.0 + 16.0, DISPOSITION_Z + 7.0)
            + centered_cube(
                format!("gripper_decon_disposition_lane_rear_gate_{lane}"),
                82.0,
                5.0,
                14.0,
            )
            .translate(x, DISPOSITION_Y / 2.0 - 16.0, DISPOSITION_Z + 7.0);
    }
    gates
}

fn camera_evidence_bridge() -> Part {
    bridge_posts()
        + bridge_top_beams()
        + bridge_camera_plate()
        + bridge_window_frame()
        + bridge_lighting_rails()
        + bridge_evidence_fiducials()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty("gripper_decon_camera_evidence_bridge_posts");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("gripper_decon_camera_bridge_corner_post_{i}"),
                BRIDGE_POST_W,
                BRIDGE_POST_W,
                BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z,
            )
            .translate(
                sx * (BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_W / 2.0),
                sy * (BRIDGE_SPAN_Y / 2.0 - BRIDGE_POST_W / 2.0),
                (BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z) / 2.0,
            );
    }
    posts
}

fn bridge_top_beams() -> Part {
    let z = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0;
    centered_cube(
        "gripper_decon_camera_bridge_front_beam",
        BRIDGE_SPAN_X,
        BRIDGE_POST_W,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, -BRIDGE_SPAN_Y / 2.0 + BRIDGE_POST_W / 2.0, z)
        + centered_cube(
            "gripper_decon_camera_bridge_rear_beam",
            BRIDGE_SPAN_X,
            BRIDGE_POST_W,
            BRIDGE_BEAM_Z,
        )
        .translate(0.0, BRIDGE_SPAN_Y / 2.0 - BRIDGE_POST_W / 2.0, z)
        + centered_cube(
            "gripper_decon_camera_bridge_left_beam",
            BRIDGE_POST_W,
            BRIDGE_SPAN_Y,
            BRIDGE_BEAM_Z,
        )
        .translate(-BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_W / 2.0, 0.0, z)
        + centered_cube(
            "gripper_decon_camera_bridge_right_beam",
            BRIDGE_POST_W,
            BRIDGE_SPAN_Y,
            BRIDGE_BEAM_Z,
        )
        .translate(BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_W / 2.0, 0.0, z)
}

fn bridge_camera_plate() -> Part {
    let plate = centered_cube(
        "gripper_decon_camera_bridge_camera_plate",
        CAMERA_PLATE_X,
        CAMERA_PLATE_Y,
        12.0,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 6.0);
    let lens = centered_cylinder(
        "gripper_decon_camera_bridge_lens_clearance",
        CAMERA_LENS_D / 2.0,
        14.0,
        40,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 6.0);
    let mount_slots = centered_cube(
        "gripper_decon_camera_bridge_mount_slot_left",
        16.0,
        62.0,
        14.0,
    )
    .translate(-64.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 6.0)
        + centered_cube(
            "gripper_decon_camera_bridge_mount_slot_right",
            16.0,
            62.0,
            14.0,
        )
        .translate(64.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 6.0);
    plate - lens - mount_slots
}

fn bridge_window_frame() -> Part {
    gasket_frame(
        "gripper_decon_camera_bridge_transparent_evidence_window_frame",
        CAMERA_WINDOW_X,
        CAMERA_WINDOW_Y,
        10.0,
        16.0,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + 10.0)
}

fn bridge_lighting_rails() -> Part {
    centered_cube(
        "gripper_decon_camera_bridge_left_low_angle_light_rail",
        CAMERA_WINDOW_X + 70.0,
        8.0,
        12.0,
    )
    .translate(
        0.0,
        -CAMERA_WINDOW_Y / 2.0 - 32.0,
        BRIDGE_UNDERSIDE_Z - 18.0,
    ) + centered_cube(
        "gripper_decon_camera_bridge_right_low_angle_light_rail",
        CAMERA_WINDOW_X + 70.0,
        8.0,
        12.0,
    )
    .translate(0.0, CAMERA_WINDOW_Y / 2.0 + 32.0, BRIDGE_UNDERSIDE_Z - 18.0)
}

fn bridge_evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("gripper_decon_camera_bridge_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIAL_COUNT {
        let row = i / 6;
        let col = i % 6;
        fiducials = fiducials
            + fiducial_disc(&format!("gripper_decon_camera_bridge_fiducial_{i}")).translate(
                index_offset(col, 6, 82.0),
                index_offset(row, 2, 215.0),
                BRIDGE_UNDERSIDE_Z + 16.0,
            );
    }
    fiducials
}

fn robot_approach_gauges() -> Part {
    front_robot_approach_bar()
        + side_service_approach_bars()
        + rinse_drawer_service_gauge()
        + robot_height_masts()
        + gripper_approach_comb()
}

fn front_robot_approach_bar() -> Part {
    centered_cube(
        "gripper_decon_robot_front_approach_gauge_bar",
        DECK_X - 190.0,
        16.0,
        10.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 + 28.0), 5.0)
        + centered_cube(
            "gripper_decon_robot_front_approach_clearance_shadow",
            DECK_X - 310.0,
            8.0,
            8.0,
        )
        .translate(0.0, -(DECK_Y / 2.0 + FRONT_ROBOT_APPROACH_Y), 4.0)
}

fn side_service_approach_bars() -> Part {
    centered_cube(
        "gripper_decon_left_jaw_service_side_clearance_gauge",
        14.0,
        JAW_NEST_Y + 80.0,
        10.0,
    )
    .translate(
        JAW_NEST_CENTER.0 - JAW_NEST_X / 2.0 - SIDE_GRIPPER_SERVICE_X,
        JAW_NEST_CENTER.1,
        5.0,
    ) + centered_cube(
        "gripper_decon_right_swab_service_side_clearance_gauge",
        14.0,
        SWAB_TOOL_Y + 80.0,
        10.0,
    )
    .translate(
        SWAB_TOOL_CENTER.0 + SWAB_TOOL_X / 2.0 + SIDE_GRIPPER_SERVICE_X,
        SWAB_TOOL_CENTER.1,
        5.0,
    )
}

fn rinse_drawer_service_gauge() -> Part {
    centered_cube(
        "gripper_decon_rinse_drawer_pull_service_gauge",
        RINSE_X + 60.0,
        12.0,
        10.0,
    )
    .translate(
        RINSE_CENTER.0,
        RINSE_CENTER.1 + RINSE_Y / 2.0 + RINSE_DRAWER_SERVICE_Y,
        5.0,
    )
}

fn robot_height_masts() -> Part {
    let mut masts = Part::empty("gripper_decon_robot_height_masts");
    for i in 0..APPROACH_GAUGE_COUNT {
        let x = index_offset(i, APPROACH_GAUGE_COUNT, 128.0);
        masts = masts
            + centered_cube(
                format!("gripper_decon_robot_z_clearance_mast_{i}"),
                10.0,
                10.0,
                120.0,
            )
            .translate(x, -DECK_Y / 2.0 + 82.0, 60.0)
            + centered_cube(
                format!("gripper_decon_robot_z_clearance_flag_{i}"),
                54.0,
                10.0,
                16.0,
            )
            .translate(x, -DECK_Y / 2.0 + 82.0, 120.0);
    }
    masts
}

fn gripper_approach_comb() -> Part {
    let mut comb = Part::empty("gripper_decon_robot_gripper_approach_comb");
    for i in 0..JAW_NEST_COUNT {
        comb = comb
            + centered_cube(
                format!("gripper_decon_robot_gripper_jaw_approach_tooth_{i}"),
                12.0,
                78.0,
                18.0,
            )
            .translate(
                JAW_NEST_CENTER.0 + index_offset(i, JAW_NEST_COUNT, 48.0),
                JAW_NEST_CENTER.1 - JAW_NEST_Y / 2.0 - 44.0,
                9.0,
            );
    }
    comb
}

fn module_panel(name: &str, x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y, z).translate(0.0, 0.0, z / 2.0)
}

fn gasket_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_opening"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 0.2,
    );
    (outer - inner).translate(0.0, 0.0, z / 2.0)
}

fn annular_collar(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(
            format!("{name}_inner_clearance"),
            inner_d / 2.0,
            z + 0.2,
            36,
        )
}

fn pick_fiducials(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    fiducial_disc(&format!("{prefix}_left_pick_fiducial")).translate(
        -x / 2.0 + 28.0,
        -y / 2.0 + 24.0,
        z + 1.5,
    ) + fiducial_disc(&format!("{prefix}_right_pick_fiducial")).translate(
        x / 2.0 - 28.0,
        -y / 2.0 + 24.0,
        z + 1.5,
    )
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 7.0, 3.0, 32);
    let center = centered_cylinder(format!("{name}_center_mark"), 2.0, 4.0, 20);
    (outer - center).translate(0.0, 0.0, 1.5)
}

fn barcode_stripes(prefix: &str, index: usize, x: f64, y: f64, z: f64) -> Part {
    let mut stripes = Part::empty(format!("{prefix}_{index}_stripes"));
    for (bar, dx) in [-14.0, -9.0, -4.0, 2.0, 8.0, 14.0].into_iter().enumerate() {
        let width = if bar % 2 == 0 { 2.2 } else { 1.1 };
        stripes =
            stripes
                + centered_cube(format!("{prefix}_{index}_stripe_{bar}"), width, 10.0, 1.2)
                    .translate(x + dx, y, z);
    }
    stripes
}

fn index_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_outputs_are_station_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_robotic_gripper_decon_residue_swab_recovery_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn layout_fits_without_module_overlap() {
        assert_layout();
    }

    #[test]
    fn residue_recovery_capacity_covers_gripper_validation() {
        assert!(JAW_NEST_COUNT >= 4);
        assert!(COUPON_COUNT >= 24);
        assert!(RINSE_CHANNEL_COUNT >= JAW_NEST_COUNT);
        assert!(RECOVERY_VIAL_COUNT >= CLEAN_SWAB_COUNT);
    }

    #[test]
    fn clean_dirty_workflow_is_physically_segregated() {
        let clean_slot_x = SEGREGATION_CENTER.0 - SEGREGATION_X / 4.0;
        let dirty_slot_x = SEGREGATION_CENTER.0 + SEGREGATION_X / 4.0;
        assert!(dirty_slot_x - clean_slot_x >= SEGREGATION_X / 2.0);
        assert!(BULKHEAD_Z >= 2.0 * SEGREGATION_Z);
        assert!(DIRTY_RETURN_SLOTS >= CLEAN_SWAB_SLOTS);
    }

    #[test]
    fn witness_tokens_include_controls_and_blanks() {
        assert_eq!(POSITIVE_TOKEN_COUNT, NEGATIVE_TOKEN_COUNT);
        assert!(BLANK_TOKEN_COUNT >= 4);
        assert!(BARCODE_TAG_COUNT >= POSITIVE_TOKEN_COUNT + NEGATIVE_TOKEN_COUNT);
    }

    #[test]
    fn evidence_bridge_and_robot_gauges_clear_the_fixture() {
        assert!(highest_fixture_feature_z() < ROBOT_Z_CLEARANCE);
        assert!(EVIDENCE_FIDUCIAL_COUNT >= 12);
        assert!(APPROACH_GAUGE_COUNT >= JAW_NEST_COUNT);
        assert!(FRONT_ROBOT_APPROACH_Y >= 400.0);
    }
}
