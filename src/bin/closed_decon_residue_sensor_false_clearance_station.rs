use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed decontamination residue sensor false-clearance station.
//
// Design intent:
// - Package a closed-system validation fixture that makes residue-sensor false
//   "safe/clear" states visible before tissue-chip modules return to culture
//   service.
// - Represent the mechanical datums for sensor cartridge nests, removable
//   material coupons, wipe/swab recovery wells, aeration timing tokens,
//   airflow witnesses, reference residue lands, custody marks, segregation,
//   camera evidence capture, robot/service keepouts, and a filtered exhaust
//   placeholder.
// - This is CAD packaging for validation hardware only. It does not specify a
//   decontamination process, sterilization claim, residue chemistry, or
//   residue acceptance threshold.

const BIN_NAME: &str = "closed_decon_residue_sensor_false_clearance_station";
const OUTPUT_PREFIX: &str = "output/closed_decon_residue_sensor_false_clearance_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_decon_residue_sensor_false_clearance_station_secondary_containment_deck.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_material_coupon_bank.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_residue_sensor_cartridge_nests.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_wipe_swab_recovery_wells.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_aeration_timer_token_rail.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_airflow_aeration_witness_channel.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_high_low_residue_reference_lands.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_barcode_coa_custody_lands.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_clean_dirty_reject_segregation_gate.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_evidence_camera_bridge.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_robot_service_keepout_exhaust_placeholder.stl",
    "output/closed_decon_residue_sensor_false_clearance_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "secondary_containment_deck",
    "removable_material_coupon_bank",
    "residue_sensor_cartridge_nests",
    "wipe_swab_recovery_wells",
    "aeration_timer_token_rail",
    "airflow_aeration_witness_channel",
    "high_low_residue_reference_lands",
    "barcode_coa_custody_lands",
    "clean_dirty_reject_segregation",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "filtered_exhaust_placeholder",
    "false_clearance_visibility",
];

const LIMITATIONS: [&str; 4] = [
    "mechanical_validation_fixture_only",
    "no_decontamination_sop",
    "no_sterilization_claim",
    "no_residue_acceptance_threshold",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const EDGE_MARGIN: f64 = 14.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;

const COUPON_POS: (f64, f64) = (-420.0, 240.0);
const COUPON_X: f64 = 350.0;
const COUPON_Y: f64 = 210.0;
const COUPON_Z: f64 = 42.0;
const COUPON_ROWS: usize = 3;
const COUPON_COLS: usize = 6;
const COUPON_COUNT: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_PITCH_X: f64 = 48.0;
const COUPON_PITCH_Y: f64 = 54.0;
const COUPON_SLOT_X: f64 = 36.0;
const COUPON_SLOT_Y: f64 = 42.0;

const SENSOR_POS: (f64, f64) = (0.0, 240.0);
const SENSOR_X: f64 = 390.0;
const SENSOR_Y: f64 = 210.0;
const SENSOR_Z: f64 = 54.0;
const SENSOR_ROWS: usize = 2;
const SENSOR_COLS: usize = 4;
const SENSOR_COUNT: usize = SENSOR_ROWS * SENSOR_COLS;
const SENSOR_PITCH_X: f64 = 78.0;
const SENSOR_PITCH_Y: f64 = 76.0;
const SENSOR_SOCKET_X: f64 = 54.0;
const SENSOR_SOCKET_Y: f64 = 34.0;

const SWAB_POS: (f64, f64) = (425.0, 240.0);
const SWAB_X: f64 = 350.0;
const SWAB_Y: f64 = 210.0;
const SWAB_Z: f64 = 52.0;
const RECOVERY_WELLS: usize = 12;
const WIPE_PARKING_SLOTS: usize = 8;
const WELL_D: f64 = 28.0;
const WELL_PITCH_X: f64 = 50.0;

const TIMER_POS: (f64, f64) = (-420.0, 0.0);
const TIMER_X: f64 = 350.0;
const TIMER_Y: f64 = 170.0;
const TIMER_Z: f64 = 36.0;
const TIMER_TOKENS: usize = 7;
const TIMER_TOKEN_PITCH: f64 = 43.0;
const TIMER_TOKEN_D: f64 = 24.0;

const AIRFLOW_POS: (f64, f64) = (0.0, 0.0);
const AIRFLOW_X: f64 = 390.0;
const AIRFLOW_Y: f64 = 170.0;
const AIRFLOW_Z: f64 = 44.0;
const WITNESS_CHANNELS: usize = 3;
const WITNESS_TICKS_PER_CHANNEL: usize = 6;

const REFERENCE_POS: (f64, f64) = (425.0, 0.0);
const REFERENCE_X: f64 = 350.0;
const REFERENCE_Y: f64 = 170.0;
const REFERENCE_Z: f64 = 32.0;
const HIGH_REF_LANDS: usize = 6;
const LOW_REF_LANDS: usize = 6;
const BLANK_REF_LANDS: usize = 3;

const CUSTODY_POS: (f64, f64) = (-420.0, -245.0);
const CUSTODY_X: f64 = 350.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 8;
const COA_CARD_SLOTS: usize = 4;
const CUSTODY_PUNCHES: usize = 10;

const SEG_POS: (f64, f64) = (0.0, -245.0);
const SEG_X: f64 = 390.0;
const SEG_Y: f64 = 170.0;
const SEG_Z: f64 = 56.0;
const SEG_LANES: usize = 3;
const LANE_SLOTS: usize = 6;
const SEG_BULKHEAD_Z: f64 = 118.0;

const CAMERA_POS: (f64, f64) = (425.0, -245.0);
const CAMERA_X: f64 = 350.0;
const CAMERA_Y: f64 = 170.0;
const CAMERA_Z: f64 = 184.0;
const CAMERA_POST_W: f64 = 22.0;
const CAMERA_BEAM_Z: f64 = 32.0;
const CAMERA_PORTS: usize = 3;
const EVIDENCE_FIDUCIALS: usize = 8;

const KEEP_OUT_X: f64 = 1200.0;
const KEEP_OUT_Y: f64 = 800.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;
const EXHAUST_POS_X: f64 = 544.0;
const EXHAUST_POS_Y: f64 = -360.0;
const FILTER_FRAME_X: f64 = 142.0;
const FILTER_FRAME_Y: f64 = 78.0;
const FILTER_FRAME_Z: f64 = 58.0;
const FILTER_SLOTS: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = DECK_X / 2.0 - RIM_W - EDGE_MARGIN;
        let half_y = DECK_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = secondary_containment_deck();
    export(OUTPUTS[0], &base);

    let coupons = material_coupon_bank();
    export(OUTPUTS[1], &coupons);

    let sensors = residue_sensor_cartridge_nests();
    export(OUTPUTS[2], &sensors);

    let swabs = wipe_swab_recovery_wells();
    export(OUTPUTS[3], &swabs);

    let timer = aeration_timer_token_rail();
    export(OUTPUTS[4], &timer);

    let airflow = airflow_aeration_witness_channel();
    export(OUTPUTS[5], &airflow);

    let references = high_low_residue_reference_lands();
    export(OUTPUTS[6], &references);

    let custody = barcode_coa_custody_lands();
    export(OUTPUTS[7], &custody);

    let segregation = clean_dirty_reject_segregation_gate();
    export(OUTPUTS[8], &segregation);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_exhaust_placeholder();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_Z))
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, on_deck_z(SENSOR_Z))
        + swabs.translate(SWAB_POS.0, SWAB_POS.1, on_deck_z(SWAB_Z))
        + timer.translate(TIMER_POS.0, TIMER_POS.1, on_deck_z(TIMER_Z))
        + airflow.translate(AIRFLOW_POS.0, AIRFLOW_POS.1, on_deck_z(AIRFLOW_Z))
        + references.translate(REFERENCE_POS.0, REFERENCE_POS.1, on_deck_z(REFERENCE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, on_deck_z(SEG_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0 + 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed decon residue sensor false-clearance station:");
    println!("  Footprint:       {DECK_X:.0}mm x {DECK_Y:.0}mm secondary containment deck");
    println!(
        "  Residue checks:  {SENSOR_COUNT} sensor cartridge nests, {COUPON_COUNT} coupon slots, {RECOVERY_WELLS} recovery wells, {HIGH_REF_LANDS} high and {LOW_REF_LANDS} low reference lands"
    );
    println!(
        "  Aeration record: {TIMER_TOKENS} timer tokens, {WITNESS_CHANNELS} witness channels, {FILTER_SLOTS} filtered exhaust placeholder slots"
    );
    println!(
        "  Custody/control: {BARCODE_LANDS} barcode lands, {COA_CARD_SLOTS} COA slots, {SEG_LANES} segregation lanes with {LANE_SLOTS} slots each, {CAMERA_PORTS} camera ports"
    );
    println!(
        "  Limitations:     mechanical validation fixture only; no decontamination SOP, sterilization claim, or residue acceptance threshold"
    );
    println!("  Output prefix:   {OUTPUT_PREFIX}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn name(suffix: &str) -> String {
    format!("{BIN_NAME}_{suffix}")
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_xy(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    (
        centered_index(index % cols, cols, pitch_x),
        centered_index(index / cols, rows, pitch_y),
    )
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 9] {
    [
        rect("material_coupon_bank", COUPON_POS, COUPON_X, COUPON_Y),
        rect(
            "residue_sensor_cartridge_nests",
            SENSOR_POS,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect("wipe_swab_recovery_wells", SWAB_POS, SWAB_X, SWAB_Y),
        rect("aeration_timer_token_rail", TIMER_POS, TIMER_X, TIMER_Y),
        rect(
            "airflow_aeration_witness_channel",
            AIRFLOW_POS,
            AIRFLOW_X,
            AIRFLOW_Y,
        ),
        rect(
            "high_low_residue_reference_lands",
            REFERENCE_POS,
            REFERENCE_X,
            REFERENCE_Y,
        ),
        rect(
            "barcode_coa_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect("clean_dirty_reject_segregation_gate", SEG_POS, SEG_X, SEG_Y),
        rect("evidence_camera_bridge", CAMERA_POS, CAMERA_X, CAMERA_Y),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for feature in [
        "secondary_containment_deck",
        "removable_material_coupon_bank",
        "residue_sensor_cartridge_nests",
        "wipe_swab_recovery_wells",
        "aeration_timer_token_rail",
        "airflow_aeration_witness_channel",
        "high_low_residue_reference_lands",
        "barcode_coa_custody_lands",
        "clean_dirty_reject_segregation",
        "evidence_camera_bridge",
        "robot_service_keepouts",
        "filtered_exhaust_placeholder",
        "false_clearance_visibility",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "mechanical_validation_fixture_only",
        "no_decontamination_sop",
        "no_sterilization_claim",
        "no_residue_acceptance_threshold",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(COUPON_COUNT, COUPON_ROWS * COUPON_COLS);
    assert_eq!(SENSOR_COUNT, SENSOR_ROWS * SENSOR_COLS);
    assert_eq!(HIGH_REF_LANDS + LOW_REF_LANDS + BLANK_REF_LANDS, 15);
    assert_eq!(SEG_LANES * LANE_SLOTS, 18);
    assert!(SEG_BULKHEAD_Z > SEG_Z);
    assert!(CAMERA_Z > SEG_BULKHEAD_Z);
    assert!(FILTER_FRAME_Z > BASE_Z);
    assert!(KEEP_OUT_X < DECK_X && KEEP_OUT_Y < DECK_Y);

    let rects = module_rects();
    for module in rects {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds deck usable envelope",
            module.name
        );
    }

    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        name("secondary_containment_deck_base"),
        DECK_X,
        DECK_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin_cut = centered_cube(
        name("secondary_containment_deck_shallow_basin_cut"),
        DECK_X - 2.0 * (RIM_W + 46.0),
        DECK_Y - 2.0 * (RIM_W + 50.0),
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z - 4.0);
    let false_clearance_gutter = centered_cube(
        name("secondary_containment_false_clearance_gutter_cut"),
        DECK_X - 240.0,
        24.0,
        10.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 70.0, BASE_Z - 4.0);
    let drain = centered_cylinder(
        name("secondary_containment_low_point_drain_placeholder_cut"),
        8.0,
        54.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 42.0, BASE_Z - 8.0);

    deck - basin_cut - false_clearance_gutter - drain - module_sockets() - mount_holes()
        + perimeter_rim()
        + zone_dividers()
        + base_datum_targets()
        + false_clearance_witness_arrow_lands()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty(name("secondary_containment_module_socket_cuts"));
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("secondary_containment_socket_{}", module.name)),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(name("secondary_containment_mount_hole_cuts"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                name(&format!("secondary_containment_m6_mount_hole_{i}")),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    centered_cube(
        name("secondary_containment_front_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0)
        + centered_cube(name("secondary_containment_rear_rim"), DECK_X, RIM_W, RIM_Z).translate(
            0.0,
            DECK_Y / 2.0 - RIM_W / 2.0,
            BASE_Z + RIM_Z / 2.0,
        )
        + centered_cube(name("secondary_containment_left_rim"), RIM_W, DECK_Y, RIM_Z).translate(
            -DECK_X / 2.0 + RIM_W / 2.0,
            0.0,
            BASE_Z + RIM_Z / 2.0,
        )
        + centered_cube(
            name("secondary_containment_right_rim"),
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0)
}

fn zone_dividers() -> Part {
    centered_cube(
        name("secondary_containment_top_to_aeration_row_divider"),
        DECK_X - 220.0,
        10.0,
        26.0,
    )
    .translate(0.0, 124.0, BASE_Z + 13.0)
        + centered_cube(
            name("secondary_containment_aeration_to_disposition_row_divider"),
            DECK_X - 220.0,
            10.0,
            26.0,
        )
        .translate(0.0, -126.0, BASE_Z + 13.0)
        + centered_cube(
            name("secondary_containment_clean_dirty_vertical_boundary_left"),
            10.0,
            DECK_Y - 210.0,
            30.0,
        )
        .translate(-215.0, 0.0, BASE_Z + 15.0)
        + centered_cube(
            name("secondary_containment_dirty_reject_vertical_boundary_right"),
            10.0,
            DECK_Y - 210.0,
            30.0,
        )
        .translate(215.0, 0.0, BASE_Z + 15.0)
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty(name("secondary_containment_robot_datum_targets"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 88.0, -DECK_Y / 2.0 + 88.0),
        (DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 88.0),
        (-DECK_X / 2.0 + 88.0, DECK_Y / 2.0 - 88.0),
        (DECK_X / 2.0 - 88.0, DECK_Y / 2.0 - 88.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + datum_disc(&format!("secondary_containment_datum_{i}")).translate(
                *x,
                *y,
                BASE_Z + 2.0,
            );
    }
    targets
}

fn false_clearance_witness_arrow_lands() -> Part {
    let mut lands = Part::empty(name("secondary_containment_false_clearance_witness_arrows"));
    for (i, x) in centered_row(7, 76.0).iter().enumerate() {
        lands = lands
            + centered_cube(
                name(&format!("secondary_containment_clear_state_arrow_land_{i}")),
                50.0,
                16.0,
                5.0,
            )
            .translate(*x, -DECK_Y / 2.0 + 102.0, BASE_Z + 2.5)
            + centered_cube(
                name(&format!("secondary_containment_clear_state_arrow_tip_{i}")),
                15.0,
                15.0,
                5.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(*x + 28.0, -DECK_Y / 2.0 + 102.0, BASE_Z + 2.5);
    }
    lands
}

fn material_coupon_bank() -> Part {
    let body = centered_cube(
        name("material_coupon_bank_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let field_cut = centered_cube(
        name("material_coupon_bank_recessed_removable_coupon_field_cut"),
        COUPON_X - 34.0,
        COUPON_Y - 36.0,
        8.0,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0 - 4.0);

    body - field_cut - coupon_slot_cuts() + coupon_pull_tabs() + coupon_material_lands()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("material_coupon_bank_slot_cuts"));
    for i in 0..COUPON_COUNT {
        let (x, y) = grid_xy(i, COUPON_COLS, COUPON_ROWS, COUPON_PITCH_X, COUPON_PITCH_Y);
        cuts = cuts
            + centered_cube(
                name(&format!(
                    "material_coupon_bank_removable_coupon_pocket_cut_{i}"
                )),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                16.0,
            )
            .translate(x, y, COUPON_Z / 2.0 - 8.0)
            + centered_cylinder(
                name(&format!("material_coupon_bank_finger_lift_cut_{i}")),
                6.0,
                10.0,
                24,
            )
            .translate(x + COUPON_SLOT_X / 2.0 - 2.0, y, COUPON_Z / 2.0 - 3.0);
    }
    cuts
}

fn coupon_pull_tabs() -> Part {
    let mut tabs = Part::empty(name("material_coupon_bank_pull_tabs"));
    for i in 0..COUPON_COUNT {
        let (x, y) = grid_xy(i, COUPON_COLS, COUPON_ROWS, COUPON_PITCH_X, COUPON_PITCH_Y);
        tabs = tabs
            + centered_cube(
                name(&format!("material_coupon_bank_tab_stop_{i}")),
                26.0,
                5.0,
                7.0,
            )
            .translate(x, y - COUPON_SLOT_Y / 2.0 - 5.0, COUPON_Z / 2.0 + 3.5);
    }
    tabs
}

fn coupon_material_lands() -> Part {
    let polymer = raised_label_land("material_coupon_bank_polymer_land", 80.0, 22.0, 4).translate(
        -112.0,
        COUPON_Y / 2.0 - 22.0,
        COUPON_Z / 2.0 + 2.0,
    );
    let metal = raised_label_land("material_coupon_bank_metal_land", 80.0, 22.0, 4).translate(
        0.0,
        COUPON_Y / 2.0 - 22.0,
        COUPON_Z / 2.0 + 2.0,
    );
    let elastomer = raised_label_land("material_coupon_bank_elastomer_land", 88.0, 22.0, 4)
        .translate(112.0, COUPON_Y / 2.0 - 22.0, COUPON_Z / 2.0 + 2.0);
    polymer + metal + elastomer
}

fn residue_sensor_cartridge_nests() -> Part {
    let body = centered_cube(
        name("residue_sensor_cartridge_nests_body"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let recess = centered_cube(
        name("residue_sensor_cartridge_common_recess_cut"),
        SENSOR_X - 36.0,
        SENSOR_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0 - 5.0);

    body - recess - sensor_socket_cuts() + sensor_latch_stops() + sensor_trace_channel_lands()
}

fn sensor_socket_cuts() -> Part {
    let mut cuts = Part::empty(name("residue_sensor_cartridge_socket_cuts"));
    for i in 0..SENSOR_COUNT {
        let (x, y) = grid_xy(i, SENSOR_COLS, SENSOR_ROWS, SENSOR_PITCH_X, SENSOR_PITCH_Y);
        cuts = cuts
            + centered_cube(
                name(&format!("residue_sensor_cartridge_rect_socket_cut_{i}")),
                SENSOR_SOCKET_X,
                SENSOR_SOCKET_Y,
                24.0,
            )
            .translate(x, y, SENSOR_Z / 2.0 - 12.0)
            + centered_cube(
                name(&format!("residue_sensor_cartridge_optical_window_cut_{i}")),
                28.0,
                8.0,
                12.0,
            )
            .translate(x, y - SENSOR_SOCKET_Y / 2.0 + 2.0, SENSOR_Z / 2.0 - 5.0);
    }
    cuts
}

fn sensor_latch_stops() -> Part {
    let mut stops = Part::empty(name("residue_sensor_cartridge_latch_stops"));
    for i in 0..SENSOR_COUNT {
        let (x, y) = grid_xy(i, SENSOR_COLS, SENSOR_ROWS, SENSOR_PITCH_X, SENSOR_PITCH_Y);
        stops = stops
            + centered_cube(
                name(&format!(
                    "residue_sensor_cartridge_clear_state_latch_stop_{i}"
                )),
                10.0,
                SENSOR_SOCKET_Y + 18.0,
                14.0,
            )
            .translate(x - SENSOR_SOCKET_X / 2.0 - 8.0, y, SENSOR_Z / 2.0 + 7.0)
            + centered_cube(
                name(&format!(
                    "residue_sensor_cartridge_alarm_state_latch_stop_{i}"
                )),
                10.0,
                SENSOR_SOCKET_Y + 18.0,
                14.0,
            )
            .translate(x + SENSOR_SOCKET_X / 2.0 + 8.0, y, SENSOR_Z / 2.0 + 7.0);
    }
    stops
}

fn sensor_trace_channel_lands() -> Part {
    let mut lands = Part::empty(name("residue_sensor_trace_channel_lands"));
    for i in 0..SENSOR_ROWS {
        let y = centered_index(i, SENSOR_ROWS, SENSOR_PITCH_Y);
        lands = lands
            + centered_cube(
                name(&format!("residue_sensor_false_clearance_trace_channel_{i}")),
                SENSOR_X - 62.0,
                6.0,
                6.0,
            )
            .translate(0.0, y + 30.0, SENSOR_Z / 2.0 + 3.0);
    }
    lands
}

fn wipe_swab_recovery_wells() -> Part {
    let body = centered_cube(
        name("wipe_swab_recovery_wells_body"),
        SWAB_X,
        SWAB_Y,
        SWAB_Z,
    );

    body - recovery_well_cuts() - wipe_slot_cuts()
        + swab_vial_index_rings()
        + wipe_used_flag_lands()
}

fn recovery_well_cuts() -> Part {
    let mut cuts = Part::empty(name("wipe_swab_recovery_well_cuts"));
    for i in 0..RECOVERY_WELLS {
        let x = centered_index(i % 6, 6, WELL_PITCH_X);
        let y = centered_index(i / 6, 2, 66.0) + 28.0;
        cuts = cuts
            + centered_cylinder(
                name(&format!("wipe_swab_recovery_round_well_cut_{i}")),
                WELL_D / 2.0,
                SWAB_Z + 6.0,
                36,
            )
            .translate(x, y, 5.0);
    }
    cuts
}

fn wipe_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("wipe_swab_recovery_wipe_slot_cuts"));
    for i in 0..WIPE_PARKING_SLOTS {
        let x = centered_index(i, WIPE_PARKING_SLOTS, 38.0);
        cuts = cuts
            + centered_cube(
                name(&format!("wipe_swab_recovery_wipe_coupon_slot_cut_{i}")),
                24.0,
                58.0,
                16.0,
            )
            .translate(x, -SWAB_Y / 2.0 + 48.0, SWAB_Z / 2.0 - 8.0);
    }
    cuts
}

fn swab_vial_index_rings() -> Part {
    let mut rings = Part::empty(name("wipe_swab_recovery_index_rings"));
    for i in 0..RECOVERY_WELLS {
        let x = centered_index(i % 6, 6, WELL_PITCH_X);
        let y = centered_index(i / 6, 2, 66.0) + 28.0;
        let ring = centered_cylinder(
            name(&format!("wipe_swab_recovery_vial_index_ring_{i}")),
            WELL_D / 2.0 + 4.0,
            5.0,
            36,
        ) - centered_cylinder(
            name(&format!("wipe_swab_recovery_vial_index_ring_clear_{i}")),
            WELL_D / 2.0 + 0.8,
            7.0,
            36,
        );
        rings = rings + ring.translate(x, y, SWAB_Z / 2.0 + 2.5);
    }
    rings
}

fn wipe_used_flag_lands() -> Part {
    let clean = raised_label_land("wipe_swab_recovery_clean_wipe_land", 82.0, 22.0, 4).translate(
        -85.0,
        -SWAB_Y / 2.0 + 18.0,
        SWAB_Z / 2.0 + 2.0,
    );
    let used = raised_label_land("wipe_swab_recovery_used_swab_land", 82.0, 22.0, 4).translate(
        85.0,
        -SWAB_Y / 2.0 + 18.0,
        SWAB_Z / 2.0 + 2.0,
    );
    clean + used
}

fn aeration_timer_token_rail() -> Part {
    let body = centered_cube(
        name("aeration_timer_token_rail_body"),
        TIMER_X,
        TIMER_Y,
        TIMER_Z,
    );
    let rail_cut = centered_cube(
        name("aeration_timer_token_rail_center_track_cut"),
        TIMER_X - 54.0,
        34.0,
        14.0,
    )
    .translate(0.0, 0.0, TIMER_Z / 2.0 - 7.0);

    body - rail_cut - timer_token_socket_cuts() + timer_stop_teeth() + timer_elapsed_witness_ticks()
}

fn timer_token_socket_cuts() -> Part {
    let mut cuts = Part::empty(name("aeration_timer_token_socket_cuts"));
    for i in 0..TIMER_TOKENS {
        let x = centered_index(i, TIMER_TOKENS, TIMER_TOKEN_PITCH);
        cuts = cuts
            + centered_cylinder(
                name(&format!("aeration_timer_token_round_socket_cut_{i}")),
                TIMER_TOKEN_D / 2.0,
                TIMER_Z + 4.0,
                30,
            )
            .translate(x, 0.0, TIMER_Z / 2.0 - 8.0);
    }
    cuts
}

fn timer_stop_teeth() -> Part {
    let mut teeth = Part::empty(name("aeration_timer_token_stop_teeth"));
    for i in 0..(TIMER_TOKENS + 1) {
        let x = centered_index(i, TIMER_TOKENS + 1, TIMER_TOKEN_PITCH);
        teeth = teeth
            + centered_cube(
                name(&format!("aeration_timer_increment_stop_tooth_{i}")),
                6.0,
                46.0,
                12.0,
            )
            .translate(x - TIMER_TOKEN_PITCH / 2.0, 0.0, TIMER_Z / 2.0 + 6.0);
    }
    teeth
}

fn timer_elapsed_witness_ticks() -> Part {
    let mut ticks = Part::empty(name("aeration_timer_elapsed_witness_ticks"));
    for i in 0..TIMER_TOKENS {
        let x = centered_index(i, TIMER_TOKENS, TIMER_TOKEN_PITCH);
        ticks = ticks
            + centered_cube(
                name(&format!("aeration_timer_elapsed_tick_land_{i}")),
                16.0,
                5.0,
                5.0,
            )
            .translate(x, TIMER_Y / 2.0 - 26.0, TIMER_Z / 2.0 + 2.5);
    }
    ticks
}

fn airflow_aeration_witness_channel() -> Part {
    let body = centered_cube(
        name("airflow_aeration_witness_channel_body"),
        AIRFLOW_X,
        AIRFLOW_Y,
        AIRFLOW_Z,
    );

    body - airflow_channel_cuts() + witness_tick_lands() + flow_direction_vanes()
}

fn airflow_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("airflow_aeration_channel_cuts"));
    for i in 0..WITNESS_CHANNELS {
        let y = centered_index(i, WITNESS_CHANNELS, 48.0);
        cuts = cuts
            + centered_cube(
                name(&format!("airflow_aeration_witness_channel_cut_{i}")),
                AIRFLOW_X - 64.0,
                18.0,
                18.0,
            )
            .translate(0.0, y, AIRFLOW_Z / 2.0 - 9.0);
    }
    cuts
}

fn witness_tick_lands() -> Part {
    let mut ticks = Part::empty(name("airflow_aeration_witness_tick_lands"));
    for channel in 0..WITNESS_CHANNELS {
        let y = centered_index(channel, WITNESS_CHANNELS, 48.0);
        for i in 0..WITNESS_TICKS_PER_CHANNEL {
            let x = centered_index(i, WITNESS_TICKS_PER_CHANNEL, 46.0);
            ticks = ticks
                + centered_cube(
                    name(&format!(
                        "airflow_aeration_channel_{channel}_witness_tick_{i}"
                    )),
                    5.0,
                    28.0,
                    6.0,
                )
                .translate(x, y, AIRFLOW_Z / 2.0 + 3.0);
        }
    }
    ticks
}

fn flow_direction_vanes() -> Part {
    let inlet = centered_cube(
        name("airflow_aeration_witness_inlet_vane"),
        20.0,
        AIRFLOW_Y - 40.0,
        12.0,
    )
    .translate(-AIRFLOW_X / 2.0 + 42.0, 0.0, AIRFLOW_Z / 2.0 + 6.0);
    let outlet = centered_cube(
        name("airflow_aeration_witness_outlet_vane"),
        20.0,
        AIRFLOW_Y - 40.0,
        12.0,
    )
    .translate(AIRFLOW_X / 2.0 - 42.0, 0.0, AIRFLOW_Z / 2.0 + 6.0);
    inlet + outlet
}

fn high_low_residue_reference_lands() -> Part {
    let body = centered_cube(
        name("high_low_residue_reference_lands_body"),
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    );

    body + residue_reference_lands() + reference_guard_rails()
}

fn residue_reference_lands() -> Part {
    let mut lands = Part::empty(name("high_low_residue_reference_lands_all"));
    for i in 0..HIGH_REF_LANDS {
        let x = centered_index(i, HIGH_REF_LANDS, 42.0);
        lands = lands
            + raised_label_land(&format!("high_residue_reference_land_{i}"), 32.0, 28.0, 3)
                .translate(x, 46.0, REFERENCE_Z / 2.0 + 2.0);
    }
    for i in 0..LOW_REF_LANDS {
        let x = centered_index(i, LOW_REF_LANDS, 42.0);
        lands = lands
            + raised_label_land(&format!("low_residue_reference_land_{i}"), 32.0, 28.0, 2)
                .translate(x, 0.0, REFERENCE_Z / 2.0 + 2.0);
    }
    for i in 0..BLANK_REF_LANDS {
        let x = centered_index(i, BLANK_REF_LANDS, 52.0);
        lands = lands
            + raised_label_land(&format!("blank_reference_land_{i}"), 38.0, 28.0, 1).translate(
                x,
                -48.0,
                REFERENCE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn reference_guard_rails() -> Part {
    centered_cube(
        name("high_low_residue_reference_high_guard_rail"),
        REFERENCE_X - 42.0,
        6.0,
        10.0,
    )
    .translate(0.0, 68.0, REFERENCE_Z / 2.0 + 5.0)
        + centered_cube(
            name("high_low_residue_reference_low_guard_rail"),
            REFERENCE_X - 42.0,
            6.0,
            10.0,
        )
        .translate(0.0, -68.0, REFERENCE_Z / 2.0 + 5.0)
}

fn barcode_coa_custody_lands() -> Part {
    let body = centered_cube(
        name("barcode_coa_custody_lands_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    body + barcode_lands() + coa_card_slots() + custody_punch_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(name("barcode_coa_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 76.0);
        let y = centered_index(i / 4, 2, 42.0) + 30.0;
        lands =
            lands
                + raised_label_land(&format!("barcode_coa_barcode_land_{i}"), 62.0, 24.0, 5)
                    .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn coa_card_slots() -> Part {
    let mut slots = Part::empty(name("barcode_coa_card_slot_rails"));
    for i in 0..COA_CARD_SLOTS {
        let x = centered_index(i, COA_CARD_SLOTS, 74.0);
        slots = slots
            + centered_cube(
                name(&format!("barcode_coa_certificate_card_backstop_{i}")),
                56.0,
                5.0,
                12.0,
            )
            .translate(x, -42.0, CUSTODY_Z / 2.0 + 6.0);
    }
    slots
}

fn custody_punch_lands() -> Part {
    let mut punches = Part::empty(name("barcode_coa_custody_punch_lands"));
    for i in 0..CUSTODY_PUNCHES {
        let x = centered_index(i, CUSTODY_PUNCHES, 28.0);
        punches = punches
            + centered_cylinder(
                name(&format!("barcode_coa_custody_punch_land_{i}")),
                7.0,
                4.0,
                22,
            )
            .translate(x, -CUSTODY_Y / 2.0 + 18.0, CUSTODY_Z / 2.0 + 2.0);
    }
    punches
}

fn clean_dirty_reject_segregation_gate() -> Part {
    let body = centered_cube(
        name("clean_dirty_reject_segregation_gate_body"),
        SEG_X,
        SEG_Y,
        SEG_Z,
    );
    let bulkheads = centered_cube(
        name("clean_dirty_reject_clean_dirty_bulkhead"),
        10.0,
        SEG_Y - 22.0,
        SEG_BULKHEAD_Z,
    )
    .translate(-SEG_X / 6.0, 0.0, SEG_BULKHEAD_Z / 2.0)
        + centered_cube(
            name("clean_dirty_reject_dirty_reject_bulkhead"),
            10.0,
            SEG_Y - 22.0,
            SEG_BULKHEAD_Z,
        )
        .translate(SEG_X / 6.0, 0.0, SEG_BULKHEAD_Z / 2.0);

    body - segregation_slot_cuts() + bulkheads + segregation_lane_labels()
}

fn segregation_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("clean_dirty_reject_slot_cuts"));
    for lane in 0..SEG_LANES {
        let x = centered_index(lane, SEG_LANES, 124.0);
        for i in 0..LANE_SLOTS {
            let y = centered_index(i, LANE_SLOTS, 24.0);
            cuts = cuts
                + centered_cube(
                    name(&format!("clean_dirty_reject_lane_{lane}_slot_cut_{i}")),
                    76.0,
                    14.0,
                    18.0,
                )
                .translate(x, y, SEG_Z / 2.0 - 9.0);
        }
    }
    cuts
}

fn segregation_lane_labels() -> Part {
    raised_label_land("clean_dirty_reject_clean_lane_land", 84.0, 20.0, 4).translate(
        -124.0,
        SEG_Y / 2.0 - 20.0,
        SEG_Z / 2.0 + 2.0,
    ) + raised_label_land("clean_dirty_reject_dirty_lane_land", 84.0, 20.0, 4).translate(
        0.0,
        SEG_Y / 2.0 - 20.0,
        SEG_Z / 2.0 + 2.0,
    ) + raised_label_land("clean_dirty_reject_reject_lane_land", 84.0, 20.0, 4).translate(
        124.0,
        SEG_Y / 2.0 - 20.0,
        SEG_Z / 2.0 + 2.0,
    )
}

fn evidence_camera_bridge() -> Part {
    let post_z = CAMERA_Z - CAMERA_BEAM_Z / 2.0;
    let mut bridge = Part::empty(name("evidence_camera_bridge"));
    for (i, (x, y)) in [
        (-CAMERA_X / 2.0 + 28.0, -CAMERA_Y / 2.0 + 28.0),
        (CAMERA_X / 2.0 - 28.0, -CAMERA_Y / 2.0 + 28.0),
        (-CAMERA_X / 2.0 + 28.0, CAMERA_Y / 2.0 - 28.0),
        (CAMERA_X / 2.0 - 28.0, CAMERA_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                name(&format!("evidence_camera_bridge_post_{i}")),
                CAMERA_POST_W,
                CAMERA_POST_W,
                post_z,
            )
            .translate(*x, *y, post_z / 2.0);
    }

    bridge + camera_beams() + camera_mount_plate() + evidence_fiducials()
}

fn camera_beams() -> Part {
    centered_cube(
        name("evidence_camera_bridge_front_beam"),
        CAMERA_X,
        CAMERA_POST_W,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, -CAMERA_Y / 2.0 + CAMERA_POST_W / 2.0, CAMERA_Z)
        + centered_cube(
            name("evidence_camera_bridge_rear_beam"),
            CAMERA_X,
            CAMERA_POST_W,
            CAMERA_BEAM_Z,
        )
        .translate(0.0, CAMERA_Y / 2.0 - CAMERA_POST_W / 2.0, CAMERA_Z)
        + centered_cube(
            name("evidence_camera_bridge_left_beam"),
            CAMERA_POST_W,
            CAMERA_Y,
            CAMERA_BEAM_Z,
        )
        .translate(-CAMERA_X / 2.0 + CAMERA_POST_W / 2.0, 0.0, CAMERA_Z)
        + centered_cube(
            name("evidence_camera_bridge_right_beam"),
            CAMERA_POST_W,
            CAMERA_Y,
            CAMERA_BEAM_Z,
        )
        .translate(CAMERA_X / 2.0 - CAMERA_POST_W / 2.0, 0.0, CAMERA_Z)
}

fn camera_mount_plate() -> Part {
    let plate = centered_cube(
        name("evidence_camera_bridge_camera_mount_plate"),
        168.0,
        72.0,
        10.0,
    )
    .translate(0.0, 0.0, CAMERA_Z + CAMERA_BEAM_Z / 2.0 + 5.0);
    let mut holes = Part::empty(name("evidence_camera_bridge_camera_port_cuts"));
    for i in 0..CAMERA_PORTS {
        let x = centered_index(i, CAMERA_PORTS, 48.0);
        holes = holes
            + centered_cylinder(
                name(&format!("evidence_camera_bridge_lens_port_cut_{i}")),
                10.0,
                14.0,
                30,
            )
            .translate(x, 0.0, CAMERA_Z + CAMERA_BEAM_Z / 2.0 + 5.0);
    }
    plate - holes
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty(name("evidence_camera_bridge_fiducials"));
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = centered_index(i % 4, 4, 70.0);
        let y = centered_index(i / 4, 2, 92.0);
        fiducials = fiducials
            + datum_disc(&format!("evidence_camera_bridge_fiducial_{i}")).translate(
                x,
                y,
                CAMERA_Z - CAMERA_BEAM_Z / 2.0 - 4.0,
            );
    }
    fiducials
}

fn robot_service_keepout_exhaust_placeholder() -> Part {
    keepout_rails() + filtered_exhaust_placeholder()
}

fn keepout_rails() -> Part {
    centered_cube(
        name("robot_service_keepout_front_robot_approach_rail"),
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0)
        + centered_cube(
            name("robot_service_keepout_rear_service_rail"),
            KEEP_OUT_X,
            10.0,
            KEEP_OUT_RAIL_Z,
        )
        .translate(0.0, KEEP_OUT_Y / 2.0, 0.0)
        + centered_cube(
            name("robot_service_keepout_left_robot_sweep_rail"),
            10.0,
            KEEP_OUT_Y,
            KEEP_OUT_RAIL_Z,
        )
        .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0)
        + centered_cube(
            name("robot_service_keepout_right_exhaust_service_rail"),
            10.0,
            KEEP_OUT_Y,
            KEEP_OUT_RAIL_Z,
        )
        .translate(KEEP_OUT_X / 2.0, 0.0, 0.0)
}

fn filtered_exhaust_placeholder() -> Part {
    let frame = centered_cube(
        name("filtered_exhaust_placeholder_frame"),
        FILTER_FRAME_X,
        FILTER_FRAME_Y,
        FILTER_FRAME_Z,
    )
    .translate(EXHAUST_POS_X, EXHAUST_POS_Y, FILTER_FRAME_Z / 2.0);
    let open_center = centered_cube(
        name("filtered_exhaust_placeholder_media_pack_cut"),
        FILTER_FRAME_X - 26.0,
        FILTER_FRAME_Y - 24.0,
        FILTER_FRAME_Z + 4.0,
    )
    .translate(EXHAUST_POS_X, EXHAUST_POS_Y, FILTER_FRAME_Z / 2.0);

    frame - open_center + filter_slot_bars()
}

fn filter_slot_bars() -> Part {
    let mut bars = Part::empty(name("filtered_exhaust_placeholder_slot_bars"));
    for i in 0..FILTER_SLOTS {
        let x = centered_index(i, FILTER_SLOTS, 22.0);
        bars = bars
            + centered_cube(
                name(&format!("filtered_exhaust_placeholder_louver_bar_{i}")),
                6.0,
                FILTER_FRAME_Y - 18.0,
                FILTER_FRAME_Z,
            )
            .translate(EXHAUST_POS_X + x, EXHAUST_POS_Y, FILTER_FRAME_Z / 2.0);
    }
    bars
}

fn raised_label_land(name_suffix: &str, x: f64, y: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(name_suffix), x, y, 4.0);
    for i in 0..bars {
        let bar_x = centered_index(i, bars, x / (bars as f64 + 1.0));
        land = land
            + centered_cube(
                name(&format!("{name_suffix}_barcode_bar_{i}")),
                3.0,
                y - 7.0,
                2.5,
            )
            .translate(bar_x, 0.0, 3.25);
    }
    land
}

fn datum_disc(name_suffix: &str) -> Part {
    centered_cylinder(name(name_suffix), 11.0, 4.0, 34)
        - centered_cylinder(name(&format!("{name_suffix}_center_cut")), 3.0, 6.0, 24)
}

fn centered_row(count: usize, pitch: f64) -> Vec<f64> {
    (0..count)
        .map(|index| centered_index(index, count, pitch))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_deterministic_and_prefixed() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    }

    #[test]
    fn validation_features_and_limitations_are_explicit() {
        assert!(REQUIRED_FEATURES.contains(&"false_clearance_visibility"));
        assert!(REQUIRED_FEATURES.contains(&"filtered_exhaust_placeholder"));
        assert!(LIMITATIONS.contains(&"no_decontamination_sop"));
        assert!(LIMITATIONS.contains(&"no_sterilization_claim"));
        assert!(LIMITATIONS.contains(&"no_residue_acceptance_threshold"));
    }

    #[test]
    fn geometry_counts_match_layout_intent() {
        assert_eq!(COUPON_COUNT, 18);
        assert_eq!(SENSOR_COUNT, 8);
        assert_eq!(RECOVERY_WELLS, 12);
        assert_eq!(HIGH_REF_LANDS + LOW_REF_LANDS + BLANK_REF_LANDS, 15);
        assert_eq!(SEG_LANES * LANE_SLOTS, 18);
    }

    #[test]
    fn modules_fit_and_do_not_overlap() {
        let rects = module_rects();
        assert!(rects.iter().all(|rect| rect.fits_inside_deck()));

        for (i, a) in rects.iter().enumerate() {
            for b in rects.iter().skip(i + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
    }

    #[test]
    fn centered_grid_places_extents_symmetrically() {
        assert_eq!(centered_index(0, 3, 54.0), -54.0);
        assert_eq!(centered_index(1, 3, 54.0), 0.0);
        assert_eq!(centered_index(2, 3, 54.0), 54.0);
        assert_eq!(grid_xy(7, 4, 2, 78.0, 76.0), (117.0, 38.0));
    }
}
