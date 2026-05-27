use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed iPSC differentiation media-change recipe verification station.
//
// This is a no-cell validation fixture for checking timed media-change recipe
// inputs before a fragile live iPSC differentiation workflow is released to
// the robot. The geometry packages recipe/checksum identity, staged media and
// additive consumables, day/time tokens, dose witness wells, barcode/COA lands,
// light/cold protection, sterile connector routing, waste/flush capture,
// release/hold/reject disposition, evidence imaging, and robot/service
// keepouts. It is mechanical concept CAD only; it does not define biological
// acceptance criteria or clinical/manufacturing release rules.

const OUTPUT_PREFIX: &str = "closed_ipsc_differentiation_media_change_recipe_verification_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_base_deck.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_recipe_badge_checksum_dock.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_staged_media_bag_nests.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_additive_vial_nests.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_day_time_token_lanes.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_low_volume_dose_witness_wells.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_barcode_coa_lands.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_light_cold_protection_pockets.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_sterile_connector_bulkhead.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_waste_flush_route.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_release_hold_reject_lanes.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_evidence_bridge_keepouts.stl",
    "output/closed_ipsc_differentiation_media_change_recipe_verification_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "recipe_badge_checksum_dock",
    "staged_media_bag_nests",
    "additive_vial_nests",
    "day_time_token_lanes",
    "low_volume_dose_witness_wells",
    "barcode_coa_lands",
    "light_cold_protection_pockets",
    "sterile_connector_bulkhead",
    "waste_flush_route",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const BASE_X: f64 = 2000.0;
const BASE_Y: f64 = 980.0;
const BASE_Z: f64 = 22.0;
const CURB_W: f64 = 20.0;
const CURB_Z: f64 = 44.0;
const SOCKET_Z: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;

const BADGE_POS: (f64, f64) = (-545.0, 330.0);
const BADGE_X: f64 = 356.0;
const BADGE_Y: f64 = 150.0;
const BADGE_Z: f64 = 34.0;
const CHECKSUM_BARS: usize = 10;
const BADGE_DATUM_PINS: usize = 4;

const MEDIA_POS: (f64, f64) = (-350.0, 70.0);
const MEDIA_X: f64 = 690.0;
const MEDIA_Y: f64 = 360.0;
const MEDIA_Z: f64 = 44.0;
const MEDIA_ROWS: usize = 2;
const MEDIA_COLS: usize = 3;
const MEDIA_BAGS: usize = MEDIA_ROWS * MEDIA_COLS;
const MEDIA_PITCH_X: f64 = 214.0;
const MEDIA_PITCH_Y: f64 = 160.0;
const MEDIA_NEST_X: f64 = 170.0;
const MEDIA_NEST_Y: f64 = 104.0;
const MEDIA_NEST_DEPTH: f64 = 22.0;

const VIAL_POS: (f64, f64) = (370.0, 280.0);
const VIAL_X: f64 = 390.0;
const VIAL_Y: f64 = 250.0;
const VIAL_Z: f64 = 50.0;
const VIAL_ROWS: usize = 2;
const VIAL_COLS: usize = 6;
const VIAL_COUNT: usize = VIAL_ROWS * VIAL_COLS;
const VIAL_PITCH_X: f64 = 60.0;
const VIAL_PITCH_Y: f64 = 86.0;
const VIAL_D: f64 = 28.0;
const VIAL_DEPTH: f64 = 30.0;

const TOKEN_POS: (f64, f64) = (370.0, 0.0);
const TOKEN_X: f64 = 390.0;
const TOKEN_Y: f64 = 230.0;
const TOKEN_Z: f64 = 30.0;
const DIFFERENTIATION_DAYS: usize = 7;
const TIMEPOINTS: usize = 4;
const TOKEN_PITCH_X: f64 = 52.0;
const TOKEN_PITCH_Y: f64 = 44.0;

const WITNESS_POS: (f64, f64) = (350.0, -260.0);
const WITNESS_X: f64 = 390.0;
const WITNESS_Y: f64 = 250.0;
const WITNESS_Z: f64 = 38.0;
const WITNESS_ROWS: usize = 3;
const WITNESS_COLS: usize = 8;
const WITNESS_WELLS: usize = WITNESS_ROWS * WITNESS_COLS;
const WITNESS_PITCH_X: f64 = 45.0;
const WITNESS_PITCH_Y: f64 = 58.0;
const WITNESS_D: f64 = 19.0;
const WITNESS_DEPTH: f64 = 18.0;

const BARCODE_POS: (f64, f64) = (-520.0, -350.0);
const BARCODE_X: f64 = 340.0;
const BARCODE_Y: f64 = 210.0;
const BARCODE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 6;
const COA_CLIPS: usize = 4;

const PROTECTION_POS: (f64, f64) = (-90.0, -356.0);
const PROTECTION_X: f64 = 470.0;
const PROTECTION_Y: f64 = 190.0;
const PROTECTION_Z: f64 = 64.0;
const COLD_POCKETS: usize = 4;
const LIGHT_SHIELD_SLOTS: usize = 5;

const BULKHEAD_POS: (f64, f64) = (0.0, 438.0);
const BULKHEAD_X: f64 = 1200.0;
const BULKHEAD_Y: f64 = 42.0;
const BULKHEAD_Z: f64 = 126.0;
const CONNECTOR_PORTS: usize = MEDIA_BAGS + VIAL_ROWS;
const CONNECTOR_PITCH_X: f64 = 94.0;
const CONNECTOR_D: f64 = 25.0;

const WASTE_POS: (f64, f64) = (-835.0, -95.0);
const WASTE_X: f64 = 250.0;
const WASTE_Y: f64 = 400.0;
const WASTE_Z: f64 = 48.0;
const FLUSH_LANES: usize = 5;
const FLUSH_PITCH_Y: f64 = 64.0;
const FLUSH_CHANNEL_W: f64 = 28.0;

const DISPO_POS: (f64, f64) = (650.0, -120.0);
const DISPO_X: f64 = 160.0;
const DISPO_Y: f64 = 500.0;
const DISPO_Z: f64 = 36.0;
const DISPO_LANES: usize = 3;
const DISPO_TOKEN_SLOTS: usize = 9;

const BRIDGE_X: f64 = 1360.0;
const BRIDGE_Y: f64 = 52.0;
const BRIDGE_Z: f64 = 30.0;
const BRIDGE_CLEARANCE_Z: f64 = 245.0;
const CAMERA_MOUNTS: usize = 5;
const LIGHT_MOUNTS: usize = 4;

const ROBOT_CLEARANCE_X: f64 = 1460.0;
const ROBOT_CLEARANCE_Y: f64 = 110.0;
const SERVICE_CLEARANCE_X: f64 = 260.0;
const SERVICE_CLEARANCE_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 8.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_base(self, margin: f64) -> bool {
        self.center.0 - self.x / 2.0 >= -BASE_X / 2.0 + margin
            && self.center.0 + self.x / 2.0 <= BASE_X / 2.0 - margin
            && self.center.1 - self.y / 2.0 >= -BASE_Y / 2.0 + margin
            && self.center.1 + self.y / 2.0 <= BASE_Y / 2.0 - margin
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

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let badge = recipe_badge_checksum_dock();
    export(OUTPUTS[1], &badge);

    let media = staged_media_bag_nests();
    export(OUTPUTS[2], &media);

    let vials = additive_vial_nests();
    export(OUTPUTS[3], &vials);

    let tokens = day_time_token_lanes();
    export(OUTPUTS[4], &tokens);

    let witnesses = low_volume_dose_witness_wells();
    export(OUTPUTS[5], &witnesses);

    let barcode = barcode_coa_lands();
    export(OUTPUTS[6], &barcode);

    let protection = light_cold_protection_pockets();
    export(OUTPUTS[7], &protection);

    let bulkhead = sterile_connector_bulkhead();
    export(OUTPUTS[8], &bulkhead);

    let waste = waste_flush_route();
    export(OUTPUTS[9], &waste);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[10], &disposition);

    let bridge_keepouts = evidence_bridge_keepouts();
    export(OUTPUTS[11], &bridge_keepouts);

    let assembly = base
        + badge.translate(BADGE_POS.0, BADGE_POS.1, BASE_Z / 2.0 + BADGE_Z / 2.0)
        + media.translate(MEDIA_POS.0, MEDIA_POS.1, BASE_Z / 2.0 + MEDIA_Z / 2.0)
        + vials.translate(VIAL_POS.0, VIAL_POS.1, BASE_Z / 2.0 + VIAL_Z / 2.0)
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z / 2.0 + TOKEN_Z / 2.0)
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, BASE_Z / 2.0 + WITNESS_Z / 2.0)
        + barcode.translate(BARCODE_POS.0, BARCODE_POS.1, BASE_Z / 2.0 + BARCODE_Z / 2.0)
        + protection.translate(
            PROTECTION_POS.0,
            PROTECTION_POS.1,
            BASE_Z / 2.0 + PROTECTION_Z / 2.0,
        )
        + bulkhead.translate(
            BULKHEAD_POS.0,
            BULKHEAD_POS.1,
            BASE_Z / 2.0 + BULKHEAD_Z / 2.0,
        )
        + waste.translate(WASTE_POS.0, WASTE_POS.1, BASE_Z / 2.0 + WASTE_Z / 2.0)
        + disposition.translate(DISPO_POS.0, DISPO_POS.1, BASE_Z / 2.0 + DISPO_Z / 2.0)
        + bridge_keepouts.translate(0.0, 0.0, BASE_Z);
    export(OUTPUTS[12], &assembly);

    println!(
        "{OUTPUT_PREFIX}: {:.0}mm x {:.0}mm no-cell recipe verification station; {} staged media bags, {} additive vials, {} day/time tokens, {} dose witness wells, {} connector ports, release/hold/reject lanes, evidence bridge, and robot/service keepouts.",
        BASE_X,
        BASE_Y,
        MEDIA_BAGS,
        VIAL_COUNT,
        DIFFERENTIATION_DAYS * TIMEPOINTS,
        WITNESS_WELLS,
        CONNECTOR_PORTS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "closed_ipsc_recipe_verification_base_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );
    let recessed_field = centered_cube(
        "closed_ipsc_recipe_verification_recessed_process_field",
        BASE_X - 2.0 * (CURB_W + 34.0),
        BASE_Y - 2.0 * (CURB_W + 34.0),
        SOCKET_Z,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - SOCKET_Z / 2.0);

    deck - recessed_field + base_curbs() + module_socket_shadows() + mounting_holes() + fiducials()
}

fn base_curbs() -> Part {
    let left = centered_cube(
        "closed_ipsc_recipe_verification_left_containment_curb",
        CURB_W,
        BASE_Y,
        CURB_Z,
    )
    .translate(
        -(BASE_X / 2.0 - CURB_W / 2.0),
        0.0,
        BASE_Z / 2.0 + CURB_Z / 2.0,
    );
    let right = centered_cube(
        "closed_ipsc_recipe_verification_right_containment_curb",
        CURB_W,
        BASE_Y,
        CURB_Z,
    )
    .translate(
        BASE_X / 2.0 - CURB_W / 2.0,
        0.0,
        BASE_Z / 2.0 + CURB_Z / 2.0,
    );
    let front = centered_cube(
        "closed_ipsc_recipe_verification_front_low_curb",
        BASE_X,
        CURB_W,
        CURB_Z - 14.0,
    )
    .translate(
        0.0,
        -(BASE_Y / 2.0 - CURB_W / 2.0),
        BASE_Z / 2.0 + (CURB_Z - 14.0) / 2.0,
    );
    let rear = centered_cube(
        "closed_ipsc_recipe_verification_rear_connector_curb",
        BASE_X,
        CURB_W,
        CURB_Z + 20.0,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 - CURB_W / 2.0,
        BASE_Z / 2.0 + (CURB_Z + 20.0) / 2.0,
    );

    left + right + front + rear
}

fn module_socket_shadows() -> Part {
    let mut sockets = Part::empty("closed_ipsc_recipe_verification_module_socket_shadows");
    for (i, rect) in layout_rects().iter().enumerate() {
        let socket = centered_cube(
            format!("closed_ipsc_recipe_verification_socket_{}_{}", i, rect.name),
            rect.x + 18.0,
            rect.y + 18.0,
            4.0,
        )
        .translate(rect.center.0, rect.center.1, BASE_Z / 2.0 + 2.0);
        sockets = sockets + socket;
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut cutters = Part::empty("closed_ipsc_recipe_verification_mounting_hole_cutters");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 58.0), -(BASE_Y / 2.0 - 58.0)),
        (BASE_X / 2.0 - 58.0, -(BASE_Y / 2.0 - 58.0)),
        (-(BASE_X / 2.0 - 58.0), BASE_Y / 2.0 - 58.0),
        (BASE_X / 2.0 - 58.0, BASE_Y / 2.0 - 58.0),
        (0.0, -(BASE_Y / 2.0 - 58.0)),
        (0.0, BASE_Y / 2.0 - 58.0),
        (-(BASE_X / 2.0 - 58.0), 0.0),
        (BASE_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_ipsc_recipe_verification_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 8.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_ipsc_recipe_verification_m6_service_slot_{i}"),
            MOUNT_HOLE_D + 22.0,
            MOUNT_HOLE_D + 0.6,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        cutters = cutters + hole + slot;
    }
    cutters
}

fn fiducials() -> Part {
    let mut marks = Part::empty("closed_ipsc_recipe_verification_robot_fiducials");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 86.0), BASE_Y / 2.0 - 86.0),
        (BASE_X / 2.0 - 86.0, BASE_Y / 2.0 - 86.0),
        (-(BASE_X / 2.0 - 86.0), -(BASE_Y / 2.0 - 86.0)),
    ]
    .iter()
    .enumerate()
    {
        marks =
            marks
                + fiducial_target(format!("closed_ipsc_recipe_verification_fiducial_{i}"))
                    .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    marks
}

fn recipe_badge_checksum_dock() -> Part {
    let dock = centered_cube(
        "closed_ipsc_recipe_badge_checksum_dock_body",
        BADGE_X,
        BADGE_Y,
        BADGE_Z,
    );
    let badge_recess = centered_cube(
        "closed_ipsc_recipe_badge_card_recess",
        BADGE_X - 54.0,
        BADGE_Y - 44.0,
        18.0,
    )
    .translate(-24.0, 0.0, BADGE_Z / 2.0 - 7.0);
    let qr_window = centered_cube(
        "closed_ipsc_recipe_badge_checksum_qr_window",
        72.0,
        72.0,
        22.0,
    )
    .translate(BADGE_X / 2.0 - 64.0, 0.0, BADGE_Z / 2.0 - 5.0);

    dock - badge_recess - qr_window
        + checksum_bars()
        + badge_datum_pins()
        + raised_label_strip("recipe_badge")
}

fn checksum_bars() -> Part {
    let mut bars = Part::empty("closed_ipsc_recipe_checksum_bars");
    for i in 0..CHECKSUM_BARS {
        let x = -BADGE_X / 2.0 + 42.0 + i as f64 * 22.0;
        let h = if i % 3 == 0 { 48.0 } else { 32.0 };
        bars =
            bars + centered_cube(format!("closed_ipsc_recipe_checksum_bar_{i}"), 7.0, h, 5.0)
                .translate(x, -(BADGE_Y / 2.0 - 30.0), BADGE_Z / 2.0 + 2.5);
    }
    bars
}

fn badge_datum_pins() -> Part {
    let mut pins = Part::empty("closed_ipsc_recipe_badge_datum_pins");
    for (i, (x, y)) in [
        (-(BADGE_X / 2.0 - 34.0), -(BADGE_Y / 2.0 - 28.0)),
        (BADGE_X / 2.0 - 34.0, -(BADGE_Y / 2.0 - 28.0)),
        (-(BADGE_X / 2.0 - 34.0), BADGE_Y / 2.0 - 28.0),
        (BADGE_X / 2.0 - 34.0, BADGE_Y / 2.0 - 28.0),
    ]
    .iter()
    .take(BADGE_DATUM_PINS)
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_ipsc_recipe_badge_datum_pin_{i}"),
                4.0,
                8.0,
                20,
            )
            .translate(*x, *y, BADGE_Z / 2.0 + 4.0);
    }
    pins
}

fn staged_media_bag_nests() -> Part {
    let tray = centered_cube(
        "closed_ipsc_staged_media_bag_nest_tray",
        MEDIA_X,
        MEDIA_Y,
        MEDIA_Z,
    );
    let sump = centered_cube(
        "closed_ipsc_media_nest_spill_sump",
        MEDIA_X - 42.0,
        MEDIA_Y - 42.0,
        14.0,
    )
    .translate(0.0, 0.0, MEDIA_Z / 2.0 - 5.0);
    let drain = centered_cylinder("closed_ipsc_media_nest_sump_drain", 12.0, 46.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(MEDIA_X / 2.0 - 52.0, -(MEDIA_Y / 2.0 + 8.0), -2.0);

    tray - sump - media_bag_recesses() - drain + media_bag_rails() + media_sequence_flags()
}

fn media_bag_recesses() -> Part {
    let mut recesses = Part::empty("closed_ipsc_media_bag_recesses");
    for row in 0..MEDIA_ROWS {
        for col in 0..MEDIA_COLS {
            let i = row * MEDIA_COLS + col;
            let (x, y) = media_bag_xy(row, col);
            let pocket = centered_cube(
                format!("closed_ipsc_media_day_stage_bag_recess_{i}"),
                MEDIA_NEST_X,
                MEDIA_NEST_Y,
                MEDIA_NEST_DEPTH,
            )
            .translate(x, y, MEDIA_Z / 2.0 - MEDIA_NEST_DEPTH / 2.0 + 3.0);
            let grip = centered_cube(
                format!("closed_ipsc_media_day_stage_bag_finger_clearance_{i}"),
                42.0,
                MEDIA_NEST_Y + 12.0,
                MEDIA_NEST_DEPTH + 4.0,
            )
            .translate(x + MEDIA_NEST_X / 2.0 - 18.0, y, MEDIA_Z / 2.0 - 7.0);
            recesses = recesses + pocket + grip;
        }
    }
    recesses
}

fn media_bag_rails() -> Part {
    let mut rails = Part::empty("closed_ipsc_media_bag_nest_rails");
    for row in 0..MEDIA_ROWS {
        for col in 0..MEDIA_COLS {
            let i = row * MEDIA_COLS + col;
            let (x, y) = media_bag_xy(row, col);
            let rear = centered_cube(
                format!("closed_ipsc_media_bag_{i}_rear_stop"),
                MEDIA_NEST_X + 22.0,
                7.0,
                18.0,
            )
            .translate(x, y + MEDIA_NEST_Y / 2.0 + 7.0, MEDIA_Z / 2.0 + 9.0);
            let front = centered_cube(
                format!("closed_ipsc_media_bag_{i}_front_barcode_lip"),
                MEDIA_NEST_X + 22.0,
                7.0,
                12.0,
            )
            .translate(x, y - MEDIA_NEST_Y / 2.0 - 7.0, MEDIA_Z / 2.0 + 6.0);
            rails = rails + rear + front;
        }
    }
    rails
}

fn media_sequence_flags() -> Part {
    let mut flags = Part::empty("closed_ipsc_media_sequence_flags");
    for i in 0..MEDIA_BAGS {
        let row = i / MEDIA_COLS;
        let col = i % MEDIA_COLS;
        let (x, y) = media_bag_xy(row, col);
        flags = flags
            + centered_cube(
                format!("closed_ipsc_media_stage_index_flag_{i}"),
                18.0 + i as f64 * 2.0,
                10.0,
                9.0,
            )
            .translate(
                x - MEDIA_NEST_X / 2.0 + 22.0,
                y + MEDIA_NEST_Y / 2.0 + 24.0,
                MEDIA_Z / 2.0 + 4.5,
            );
    }
    flags
}

fn additive_vial_nests() -> Part {
    let block = centered_cube(
        "closed_ipsc_additive_vial_nest_block",
        VIAL_X,
        VIAL_Y,
        VIAL_Z,
    );
    let pockets = additive_vial_pocket_cutters();
    let chilled_ring = centered_cube(
        "closed_ipsc_additive_vial_chilled_ring_plate",
        VIAL_X - 34.0,
        28.0,
        14.0,
    )
    .translate(0.0, 0.0, VIAL_Z / 2.0 + 7.0);

    block - pockets + vial_retention_clips() + chilled_ring + raised_label_strip("additive_vials")
}

fn additive_vial_pocket_cutters() -> Part {
    let mut pockets = Part::empty("closed_ipsc_additive_vial_pocket_cutters");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let i = row * VIAL_COLS + col;
            let (x, y) = vial_xy(row, col);
            let bore = centered_cylinder(
                format!("closed_ipsc_additive_vial_bore_{i}"),
                VIAL_D / 2.0,
                VIAL_DEPTH,
                32,
            )
            .translate(x, y, VIAL_Z / 2.0 - VIAL_DEPTH / 2.0 + 2.0);
            let scan_flat = centered_cube(
                format!("closed_ipsc_additive_vial_barcode_flat_{i}"),
                VIAL_D + 11.0,
                12.0,
                VIAL_DEPTH + 4.0,
            )
            .translate(x, y - VIAL_D / 2.0, VIAL_Z / 2.0 - VIAL_DEPTH / 2.0 + 2.0);
            pockets = pockets + bore + scan_flat;
        }
    }
    pockets
}

fn vial_retention_clips() -> Part {
    let mut clips = Part::empty("closed_ipsc_additive_vial_retention_clips");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let i = row * VIAL_COLS + col;
            let (x, y) = vial_xy(row, col);
            clips = clips
                + centered_cube(
                    format!("closed_ipsc_additive_vial_clip_left_{i}"),
                    5.0,
                    22.0,
                    18.0,
                )
                .translate(x - VIAL_D / 2.0 - 5.0, y, VIAL_Z / 2.0 + 9.0)
                + centered_cube(
                    format!("closed_ipsc_additive_vial_clip_right_{i}"),
                    5.0,
                    22.0,
                    18.0,
                )
                .translate(x + VIAL_D / 2.0 + 5.0, y, VIAL_Z / 2.0 + 9.0);
        }
    }
    clips
}

fn day_time_token_lanes() -> Part {
    let block = centered_cube(
        "closed_ipsc_day_time_token_lane_block",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let lane_recess = centered_cube(
        "closed_ipsc_day_time_token_recess_field",
        TOKEN_X - 34.0,
        TOKEN_Y - 34.0,
        12.0,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0 - 5.0);
    block - lane_recess - token_slot_cutters() + token_lane_rails() + timepoint_tick_marks()
}

fn token_slot_cutters() -> Part {
    let mut slots = Part::empty("closed_ipsc_day_time_token_slot_cutters");
    for day in 0..DIFFERENTIATION_DAYS {
        for tp in 0..TIMEPOINTS {
            let i = day * TIMEPOINTS + tp;
            let x = -(DIFFERENTIATION_DAYS as f64 - 1.0) * TOKEN_PITCH_X / 2.0
                + day as f64 * TOKEN_PITCH_X;
            let y = -(TIMEPOINTS as f64 - 1.0) * TOKEN_PITCH_Y / 2.0 + tp as f64 * TOKEN_PITCH_Y;
            slots = slots
                + centered_cube(
                    format!("closed_ipsc_day_{day}_timepoint_{tp}_token_slot_{i}"),
                    34.0,
                    26.0,
                    15.0,
                )
                .translate(x, y, TOKEN_Z / 2.0 - 5.0);
        }
    }
    slots
}

fn token_lane_rails() -> Part {
    let mut rails = Part::empty("closed_ipsc_day_time_token_lane_rails");
    for tp in 0..=TIMEPOINTS {
        let y = -(TIMEPOINTS as f64) * TOKEN_PITCH_Y / 2.0 + tp as f64 * TOKEN_PITCH_Y;
        rails = rails
            + centered_cube(
                format!("closed_ipsc_timepoint_lane_rail_{tp}"),
                TOKEN_X - 54.0,
                4.0,
                12.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 6.0);
    }
    rails
}

fn timepoint_tick_marks() -> Part {
    let mut ticks = Part::empty("closed_ipsc_day_time_token_tick_marks");
    for day in 0..DIFFERENTIATION_DAYS {
        let x =
            -(DIFFERENTIATION_DAYS as f64 - 1.0) * TOKEN_PITCH_X / 2.0 + day as f64 * TOKEN_PITCH_X;
        ticks = ticks
            + centered_cube(
                format!("closed_ipsc_day_token_tick_{day}"),
                5.0,
                TOKEN_Y - 52.0,
                5.0,
            )
            .translate(x, 0.0, TOKEN_Z / 2.0 + 2.5);
    }
    ticks
}

fn low_volume_dose_witness_wells() -> Part {
    let plate = centered_cube(
        "closed_ipsc_low_volume_dose_witness_plate",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    plate - witness_well_cutters() + witness_well_rims() + dose_ladder_reference()
}

fn witness_well_cutters() -> Part {
    let mut wells = Part::empty("closed_ipsc_low_volume_dose_witness_well_cutters");
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let i = row * WITNESS_COLS + col;
            let (x, y) = witness_xy(row, col);
            wells = wells
                + centered_cylinder(
                    format!("closed_ipsc_dose_witness_well_{i}"),
                    WITNESS_D / 2.0,
                    WITNESS_DEPTH,
                    28,
                )
                .translate(x, y, WITNESS_Z / 2.0 - WITNESS_DEPTH / 2.0 + 3.0);
        }
    }
    wells
}

fn witness_well_rims() -> Part {
    let mut rims = Part::empty("closed_ipsc_low_volume_dose_witness_well_rims");
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let i = row * WITNESS_COLS + col;
            let (x, y) = witness_xy(row, col);
            rims = rims
                + centered_cylinder(
                    format!("closed_ipsc_dose_witness_well_rim_{i}"),
                    WITNESS_D / 2.0 + 4.5,
                    5.0,
                    28,
                )
                .translate(x, y, WITNESS_Z / 2.0 + 2.5);
        }
    }
    rims
}

fn dose_ladder_reference() -> Part {
    let mut ladder = Part::empty("closed_ipsc_dose_witness_volume_ladder");
    for i in 0..5 {
        ladder = ladder
            + centered_cube(
                format!("closed_ipsc_dose_ladder_step_{i}"),
                18.0 + i as f64 * 14.0,
                8.0,
                7.0,
            )
            .translate(
                -(WITNESS_X / 2.0 - 64.0),
                WITNESS_Y / 2.0 - 28.0 - i as f64 * 18.0,
                WITNESS_Z / 2.0 + 3.5,
            );
    }
    ladder
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_ipsc_barcode_coa_land_panel",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    let paper_recess = centered_cube("closed_ipsc_coa_paper_recess", BARCODE_X - 58.0, 78.0, 8.0)
        .translate(0.0, BARCODE_Y / 2.0 - 58.0, BARCODE_Z / 2.0 - 2.0);
    panel - paper_recess + barcode_land_strips() + coa_corner_clips()
}

fn barcode_land_strips() -> Part {
    let mut lands = Part::empty("closed_ipsc_barcode_scan_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_ipsc_barcode_scan_land_{i}"),
                BARCODE_X - 68.0,
                18.0,
                5.0,
            )
            .translate(
                0.0,
                -(BARCODE_Y / 2.0 - 30.0) + i as f64 * 22.0,
                BARCODE_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn coa_corner_clips() -> Part {
    let mut clips = Part::empty("closed_ipsc_coa_corner_clips");
    for (i, (x, y)) in [
        (-(BARCODE_X / 2.0 - 44.0), BARCODE_Y / 2.0 - 22.0),
        (BARCODE_X / 2.0 - 44.0, BARCODE_Y / 2.0 - 22.0),
        (-(BARCODE_X / 2.0 - 44.0), BARCODE_Y / 2.0 - 94.0),
        (BARCODE_X / 2.0 - 44.0, BARCODE_Y / 2.0 - 94.0),
    ]
    .iter()
    .take(COA_CLIPS)
    .enumerate()
    {
        clips = clips
            + centered_cube(format!("closed_ipsc_coa_corner_clip_{i}"), 28.0, 10.0, 10.0)
                .translate(*x, *y, BARCODE_Z / 2.0 + 5.0);
    }
    clips
}

fn light_cold_protection_pockets() -> Part {
    let body = centered_cube(
        "closed_ipsc_light_cold_protection_body",
        PROTECTION_X,
        PROTECTION_Y,
        PROTECTION_Z,
    );
    body - cold_pack_pocket_cutters() - light_shield_slot_cutters()
        + amber_lid_parking_rails()
        + thermal_dot_lands()
}

fn cold_pack_pocket_cutters() -> Part {
    let mut pockets = Part::empty("closed_ipsc_cold_pack_pocket_cutters");
    for i in 0..COLD_POCKETS {
        let x = -(COLD_POCKETS as f64 - 1.0) * 92.0 / 2.0 + i as f64 * 92.0;
        pockets = pockets
            + centered_cube(
                format!("closed_ipsc_cold_pack_pocket_{i}"),
                72.0,
                102.0,
                28.0,
            )
            .translate(x, -20.0, PROTECTION_Z / 2.0 - 10.0);
    }
    pockets
}

fn light_shield_slot_cutters() -> Part {
    let mut slots = Part::empty("closed_ipsc_light_shield_slot_cutters");
    for i in 0..LIGHT_SHIELD_SLOTS {
        let x = -(LIGHT_SHIELD_SLOTS as f64 - 1.0) * 78.0 / 2.0 + i as f64 * 78.0;
        slots = slots
            + centered_cube(
                format!("closed_ipsc_amber_light_shield_slot_{i}"),
                10.0,
                PROTECTION_Y + 6.0,
                44.0,
            )
            .translate(x, 0.0, PROTECTION_Z / 2.0 + 2.0);
    }
    slots
}

fn amber_lid_parking_rails() -> Part {
    let rear = centered_cube(
        "closed_ipsc_amber_lid_rear_parking_rail",
        PROTECTION_X - 44.0,
        10.0,
        18.0,
    )
    .translate(0.0, PROTECTION_Y / 2.0 - 22.0, PROTECTION_Z / 2.0 + 9.0);
    let front = centered_cube(
        "closed_ipsc_amber_lid_front_parking_rail",
        PROTECTION_X - 44.0,
        10.0,
        18.0,
    )
    .translate(0.0, -(PROTECTION_Y / 2.0 - 22.0), PROTECTION_Z / 2.0 + 9.0);
    rear + front
}

fn thermal_dot_lands() -> Part {
    let mut dots = Part::empty("closed_ipsc_thermal_dot_lands");
    for i in 0..8 {
        dots = dots
            + centered_cylinder(format!("closed_ipsc_thermal_dot_land_{i}"), 8.0, 4.0, 24)
                .translate(
                    -(PROTECTION_X / 2.0 - 32.0) + i as f64 * 52.0,
                    PROTECTION_Y / 2.0 - 44.0,
                    PROTECTION_Z / 2.0 + 2.0,
                );
    }
    dots
}

fn sterile_connector_bulkhead() -> Part {
    let wall = centered_cube(
        "closed_ipsc_sterile_connector_bulkhead_wall",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let foot = centered_cube(
        "closed_ipsc_sterile_connector_bulkhead_foot",
        BULKHEAD_X + 60.0,
        64.0,
        18.0,
    )
    .translate(0.0, -18.0, -(BULKHEAD_Z / 2.0 - 9.0));
    wall + foot - connector_port_cutters() + connector_key_blocks() + tube_strain_relief_combs()
}

fn connector_port_cutters() -> Part {
    let mut ports = Part::empty("closed_ipsc_sterile_connector_bulkhead_port_cutters");
    for i in 0..CONNECTOR_PORTS {
        let x = -(CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X / 2.0
            + i as f64 * CONNECTOR_PITCH_X;
        ports = ports
            + centered_cylinder(
                format!("closed_ipsc_sterile_connector_port_{i}"),
                CONNECTOR_D / 2.0,
                BULKHEAD_Y + 10.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 20.0);
    }
    ports
}

fn connector_key_blocks() -> Part {
    let mut keys = Part::empty("closed_ipsc_sterile_connector_key_blocks");
    for i in 0..CONNECTOR_PORTS {
        let x = -(CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X / 2.0
            + i as f64 * CONNECTOR_PITCH_X;
        keys = keys
            + centered_cube(
                format!("closed_ipsc_sterile_connector_orientation_key_{i}"),
                34.0,
                10.0,
                12.0,
            )
            .translate(x, -(BULKHEAD_Y / 2.0 + 6.0), 48.0);
    }
    keys
}

fn tube_strain_relief_combs() -> Part {
    let mut combs = Part::empty("closed_ipsc_sterile_connector_tube_strain_relief_combs");
    for i in 0..CONNECTOR_PORTS {
        let x = -(CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X / 2.0
            + i as f64 * CONNECTOR_PITCH_X;
        combs = combs
            + centered_cube(
                format!("closed_ipsc_sterile_connector_tube_comb_{i}"),
                38.0,
                26.0,
                8.0,
            )
            .translate(x, -(BULKHEAD_Y / 2.0 + 24.0), -26.0);
    }
    combs
}

fn waste_flush_route() -> Part {
    let tray = centered_cube(
        "closed_ipsc_waste_flush_route_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let gutter = centered_cube(
        "closed_ipsc_waste_flush_route_main_gutter",
        FLUSH_CHANNEL_W,
        WASTE_Y - 50.0,
        22.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 8.0);
    let drain = centered_cylinder(
        "closed_ipsc_waste_flush_route_drain_bulkhead",
        18.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(WASTE_Y / 2.0 + 10.0), -3.0);

    tray - gutter - flush_lane_cutters() - drain + flush_lane_ribs() + waste_bottle_socket()
}

fn flush_lane_cutters() -> Part {
    let mut lanes = Part::empty("closed_ipsc_waste_flush_lane_cutters");
    for i in 0..FLUSH_LANES {
        let y = -(FLUSH_LANES as f64 - 1.0) * FLUSH_PITCH_Y / 2.0 + i as f64 * FLUSH_PITCH_Y;
        lanes = lanes
            + centered_cube(
                format!("closed_ipsc_flush_lane_{i}"),
                WASTE_X - 74.0,
                18.0,
                18.0,
            )
            .translate(8.0, y, WASTE_Z / 2.0 - 7.0);
    }
    lanes
}

fn flush_lane_ribs() -> Part {
    let mut ribs = Part::empty("closed_ipsc_waste_flush_lane_ribs");
    for i in 0..=FLUSH_LANES {
        let y = -(FLUSH_LANES as f64) * FLUSH_PITCH_Y / 2.0 + i as f64 * FLUSH_PITCH_Y;
        ribs = ribs
            + centered_cube(
                format!("closed_ipsc_flush_lane_rib_{i}"),
                WASTE_X - 42.0,
                5.0,
                12.0,
            )
            .translate(0.0, y, WASTE_Z / 2.0 + 6.0);
    }
    ribs
}

fn waste_bottle_socket() -> Part {
    let socket = centered_cube("closed_ipsc_waste_flush_bottle_socket", 96.0, 60.0, 18.0)
        .translate(0.0, -(WASTE_Y / 2.0 - 44.0), WASTE_Z / 2.0 + 9.0);
    let cutter = centered_cylinder(
        "closed_ipsc_waste_flush_bottle_neck_clearance",
        18.0,
        24.0,
        32,
    )
    .translate(0.0, -(WASTE_Y / 2.0 - 44.0), WASTE_Z / 2.0 + 9.0);
    socket - cutter
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_ipsc_release_hold_reject_lane_body",
        DISPO_X,
        DISPO_Y,
        DISPO_Z,
    );
    body - disposition_lane_recesses() + disposition_lane_walls() + disposition_token_slots()
}

fn disposition_lane_recesses() -> Part {
    let mut lanes = Part::empty("closed_ipsc_disposition_lane_recesses");
    for i in 0..DISPO_LANES {
        let y = -(DISPO_LANES as f64 - 1.0) * 150.0 / 2.0 + i as f64 * 150.0;
        lanes = lanes
            + centered_cube(
                format!("closed_ipsc_disposition_lane_recess_{i}"),
                DISPO_X - 54.0,
                104.0,
                14.0,
            )
            .translate(0.0, y, DISPO_Z / 2.0 - 4.0);
    }
    lanes
}

fn disposition_lane_walls() -> Part {
    let mut walls = Part::empty("closed_ipsc_disposition_lane_walls");
    for i in 0..=DISPO_LANES {
        let y = -(DISPO_LANES as f64) * 150.0 / 2.0 + i as f64 * 150.0;
        walls = walls
            + centered_cube(
                format!("closed_ipsc_disposition_lane_wall_{i}"),
                DISPO_X - 28.0,
                8.0,
                22.0,
            )
            .translate(0.0, y, DISPO_Z / 2.0 + 11.0);
    }
    walls
}

fn disposition_token_slots() -> Part {
    let mut slots = Part::empty("closed_ipsc_disposition_token_slots_release_hold_reject");
    for i in 0..DISPO_TOKEN_SLOTS {
        let lane = i / 3;
        let col = i % 3;
        let y = -(DISPO_LANES as f64 - 1.0) * 150.0 / 2.0 + lane as f64 * 150.0;
        let x = -54.0 + col as f64 * 54.0;
        slots = slots
            + centered_cube(
                format!("closed_ipsc_disposition_token_parking_slot_{i}"),
                32.0,
                24.0,
                8.0,
            )
            .translate(x, y, DISPO_Z / 2.0 + 4.0);
    }
    slots
}

fn evidence_bridge_keepouts() -> Part {
    evidence_bridge() + robot_service_keepouts()
}

fn evidence_bridge() -> Part {
    let beam = centered_cube(
        "closed_ipsc_evidence_camera_bridge_crossbeam",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(0.0, 0.0, BRIDGE_CLEARANCE_Z);
    let left_post = centered_cube(
        "closed_ipsc_evidence_bridge_left_post",
        34.0,
        44.0,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(-(BRIDGE_X / 2.0 - 46.0), 0.0, BRIDGE_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "closed_ipsc_evidence_bridge_right_post",
        34.0,
        44.0,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 46.0, 0.0, BRIDGE_CLEARANCE_Z / 2.0);
    beam + left_post + right_post + camera_mounts() + evidence_light_mounts()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_ipsc_evidence_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        let x = -(CAMERA_MOUNTS as f64 - 1.0) * 230.0 / 2.0 + i as f64 * 230.0;
        let mount = centered_cube(
            format!("closed_ipsc_evidence_camera_mount_plate_{i}"),
            76.0,
            34.0,
            12.0,
        )
        .translate(x, -BRIDGE_Y / 2.0 - 10.0, BRIDGE_CLEARANCE_Z - 24.0);
        let lens = centered_cylinder(
            format!("closed_ipsc_evidence_camera_lens_clearance_{i}"),
            12.0,
            16.0,
            30,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BRIDGE_Y / 2.0 - 12.0, BRIDGE_CLEARANCE_Z - 24.0);
        mounts = mounts + (mount - lens);
    }
    mounts
}

fn evidence_light_mounts() -> Part {
    let mut lights = Part::empty("closed_ipsc_evidence_light_mounts");
    for i in 0..LIGHT_MOUNTS {
        let x = -(LIGHT_MOUNTS as f64 - 1.0) * 300.0 / 2.0 + i as f64 * 300.0;
        lights = lights
            + centered_cube(
                format!("closed_ipsc_evidence_diffuse_light_mount_{i}"),
                128.0,
                12.0,
                12.0,
            )
            .translate(x, BRIDGE_Y / 2.0 + 10.0, BRIDGE_CLEARANCE_Z - 18.0);
    }
    lights
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_ipsc_robot_front_access_keepout_gauge",
        ROBOT_CLEARANCE_X,
        ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -(BASE_Y / 2.0 + ROBOT_CLEARANCE_Y / 2.0 - 20.0),
        KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_ipsc_rear_connector_service_keepout_gauge",
        ROBOT_CLEARANCE_X,
        ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 + ROBOT_CLEARANCE_Y / 2.0 - 20.0,
        KEEP_OUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_ipsc_left_waste_service_keepout_gauge",
        SERVICE_CLEARANCE_X,
        SERVICE_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -(BASE_X / 2.0 + SERVICE_CLEARANCE_X / 2.0 - 20.0),
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let right_robot = centered_cube(
        "closed_ipsc_right_disposition_service_keepout_gauge",
        SERVICE_CLEARANCE_X,
        SERVICE_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        BASE_X / 2.0 + SERVICE_CLEARANCE_X / 2.0 - 20.0,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    front_robot + rear_service + left_service + right_robot
}

fn raised_label_strip(name: &str) -> Part {
    let mut strip = Part::empty(format!("closed_ipsc_{name}_raised_csg_label_strip"));
    for i in 0..6 {
        strip = strip
            + centered_cube(
                format!("closed_ipsc_{name}_label_code_bar_{i}"),
                10.0 + (i % 3) as f64 * 7.0,
                5.0,
                5.0,
            )
            .translate(-54.0 + i as f64 * 22.0, 0.0, 2.5);
    }
    strip
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let outer = centered_cylinder(format!("{name}_outer_ring"), 18.0, 4.0, 36);
    let inner = centered_cylinder(format!("{name}_center_relief"), 10.0, 6.0, 36);
    let cross_x = centered_cube(format!("{name}_cross_x"), 34.0, 4.0, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 4.0, 34.0, 5.0);
    outer - inner + cross_x + cross_y
}

fn media_bag_xy(row: usize, col: usize) -> (f64, f64) {
    (
        -(MEDIA_COLS as f64 - 1.0) * MEDIA_PITCH_X / 2.0 + col as f64 * MEDIA_PITCH_X,
        -(MEDIA_ROWS as f64 - 1.0) * MEDIA_PITCH_Y / 2.0 + row as f64 * MEDIA_PITCH_Y,
    )
}

fn vial_xy(row: usize, col: usize) -> (f64, f64) {
    (
        -(VIAL_COLS as f64 - 1.0) * VIAL_PITCH_X / 2.0 + col as f64 * VIAL_PITCH_X,
        -(VIAL_ROWS as f64 - 1.0) * VIAL_PITCH_Y / 2.0 + row as f64 * VIAL_PITCH_Y,
    )
}

fn witness_xy(row: usize, col: usize) -> (f64, f64) {
    (
        -(WITNESS_COLS as f64 - 1.0) * WITNESS_PITCH_X / 2.0 + col as f64 * WITNESS_PITCH_X,
        -(WITNESS_ROWS as f64 - 1.0) * WITNESS_PITCH_Y / 2.0 + row as f64 * WITNESS_PITCH_Y,
    )
}

fn layout_rects() -> [Rect; 10] {
    [
        Rect {
            name: "badge",
            center: BADGE_POS,
            x: BADGE_X,
            y: BADGE_Y,
        },
        Rect {
            name: "media",
            center: MEDIA_POS,
            x: MEDIA_X,
            y: MEDIA_Y,
        },
        Rect {
            name: "vials",
            center: VIAL_POS,
            x: VIAL_X,
            y: VIAL_Y,
        },
        Rect {
            name: "tokens",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Rect {
            name: "witness",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "barcode",
            center: BARCODE_POS,
            x: BARCODE_X,
            y: BARCODE_Y,
        },
        Rect {
            name: "protection",
            center: PROTECTION_POS,
            x: PROTECTION_X,
            y: PROTECTION_Y,
        },
        Rect {
            name: "bulkhead",
            center: BULKHEAD_POS,
            x: BULKHEAD_X,
            y: BULKHEAD_Y,
        },
        Rect {
            name: "waste",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "disposition",
            center: DISPO_POS,
            x: DISPO_X,
            y: DISPO_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(MEDIA_BAGS, 6);
    assert_eq!(VIAL_COUNT, 12);
    assert_eq!(WITNESS_WELLS, 24);
    assert_eq!(DIFFERENTIATION_DAYS * TIMEPOINTS, 28);
    assert_eq!(CONNECTOR_PORTS, 8);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_base(28.0),
            "{} is outside the base envelope",
            rect.name
        );
    }
    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_deterministic_and_prefixed() {
        for output in OUTPUTS {
            assert!(output.starts_with(
                "output/closed_ipsc_differentiation_media_change_recipe_verification_station_"
            ));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn station_contains_requested_feature_groups() {
        for feature in REQUIRED_FEATURES {
            assert!(!feature.is_empty());
        }
        assert!(REQUIRED_FEATURES.contains(&"recipe_badge_checksum_dock"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn layout_is_non_overlapping_inside_base() {
        assert_design_constraints();
    }
}
