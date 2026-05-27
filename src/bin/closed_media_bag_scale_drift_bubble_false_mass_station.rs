use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/feed bag scale-drift and bubble/foam false-mass validation station.
//
// Intent:
// - Challenge gravimetric perfusion mass readings with a sealed media/feed bag
//   in both hanging and tray-supported modes.
// - Make scale-drift controls, reference mass custody, tubing strain relief,
//   bubble/foam witness geometry, thermal shielding, prime/drain capture,
//   release/hold/reject disposition, barcode custody, and robot/service
//   keepouts explicit in CAD.
// - This is validation fixture CAD only. It does not define sterile processing,
//   weighing algorithms, load-cell electronics, cell culture release criteria,
//   or pump control logic.

const BIN_NAME: &str = "closed_media_bag_scale_drift_bubble_false_mass_station";
const OUTPUT_PREFIX: &str = "output/closed_media_bag_scale_drift_bubble_false_mass_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_base_containment_deck.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_hanging_bag_mode_frame.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_tray_bag_mode_cradle.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_load_cell_reference_mass_pocket.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_tubing_strain_relief_comb.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_bubble_foam_witness_window.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_thermal_shield.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_calibration_token_rail.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_drain_prime_capture_trough.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_release_hold_reject_lanes.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_barcode_custody_panel.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_robot_service_keepouts.stl",
    "output/closed_media_bag_scale_drift_bubble_false_mass_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "hanging_bag_mode",
    "tray_bag_mode",
    "load_cell_reference_mass_pocket",
    "tubing_strain_relief_comb",
    "bubble_foam_witness_window",
    "thermal_shield",
    "calibration_token_rail",
    "drain_prime_capture_trough",
    "release_hold_reject_lanes",
    "barcode_custody_panel",
    "robot_service_keepouts",
    "gravimetric_false_mass_challenge",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "no_sterile_processing_protocol",
    "no_load_cell_electronics_design",
    "no_pump_control_algorithm",
    "no_release_acceptance_limits",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 960.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const EDGE_MARGIN: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_PORT_D: f64 = 18.0;

const WEIGHING_POS: (f64, f64) = (-445.0, 95.0);
const WEIGHING_ENVELOPE_X: f64 = 430.0;
const WEIGHING_ENVELOPE_Y: f64 = 620.0;

const HANGING_FRAME_X: f64 = 390.0;
const HANGING_FRAME_Y: f64 = 250.0;
const HANGING_FRAME_Z: f64 = 555.0;
const FRAME_POST_W: f64 = 28.0;
const FRAME_BEAM_W: f64 = 30.0;
const HANGER_COUNT: usize = 5;
const HANGER_PITCH_X: f64 = 58.0;
const HANGER_PIN_D: f64 = 11.0;
const HANGING_BAG_CLEAR_X: f64 = 260.0;
const HANGING_BAG_CLEAR_Y: f64 = 92.0;
const HANGING_BAG_CLEAR_Z: f64 = 365.0;
const ANTI_SWAY_PAD_COUNT: usize = 4;

const TRAY_MODE_OFFSET_Y: f64 = -255.0;
const TRAY_X: f64 = 410.0;
const TRAY_Y: f64 = 235.0;
const TRAY_Z: f64 = 58.0;
const TRAY_POCKET_X: f64 = 330.0;
const TRAY_POCKET_Y: f64 = 160.0;
const TRAY_POCKET_DEPTH: f64 = 20.0;
const TRAY_SADDLE_RIBS: usize = 8;
const TRAY_LOCATOR_PINS: usize = 6;
const TRAY_MODE_ID_FLAGS: usize = 4;

const LOAD_REF_POS: (f64, f64) = (35.0, 285.0);
const LOAD_REF_X: f64 = 360.0;
const LOAD_REF_Y: f64 = 210.0;
const LOAD_REF_Z: f64 = 74.0;
const LOAD_CELL_PADS: usize = 4;
const LOAD_CELL_PAD_X: f64 = 74.0;
const LOAD_CELL_PAD_Y: f64 = 48.0;
const REFERENCE_MASS_WELLS: usize = 8;
const REFERENCE_MASS_COLS: usize = 4;
const REFERENCE_MASS_D: f64 = 34.0;
const FINE_MASS_WELLS: usize = 6;
const FINE_MASS_D: f64 = 16.0;
const CERTIFICATE_SLOTS: usize = 3;

const COMB_POS: (f64, f64) = (460.0, 300.0);
const COMB_X: f64 = 420.0;
const COMB_Y: f64 = 120.0;
const COMB_Z: f64 = 42.0;
const TUBE_CHANNELS: usize = 10;
const MEDIA_TUBE_OD_MAX: f64 = 12.7;
const TUBE_CLEARANCE: f64 = 2.5;
const TUBE_CHANNEL_D: f64 = MEDIA_TUBE_OD_MAX + TUBE_CLEARANCE;
const TUBE_CHANNEL_PITCH: f64 = 38.0;
const STRAIN_LOOP_GAUGES: usize = 5;
const CLAMP_FLAG_COUNT: usize = TUBE_CHANNELS;

const BUBBLE_POS: (f64, f64) = (300.0, 50.0);
const BUBBLE_X: f64 = 500.0;
const BUBBLE_Y: f64 = 190.0;
const BUBBLE_Z: f64 = 58.0;
const WITNESS_CHANNELS: usize = 5;
const WITNESS_CHANNEL_X: f64 = 365.0;
const WITNESS_CHANNEL_Y: f64 = 20.0;
const WITNESS_CHANNEL_PITCH_Y: f64 = 30.0;
const BUBBLE_GRADUATION_TICKS: usize = 11;
const FOAM_HEADSPACE_LEVELS: usize = 4;
const FALSE_MASS_COUPONS: usize = 6;
const CAMERA_FIDUCIALS: usize = 4;

const THERMAL_SHIELD_X: f64 = 515.0;
const THERMAL_SHIELD_Y: f64 = 680.0;
const THERMAL_SHIELD_Z: f64 = 430.0;
const SHIELD_PANEL_T: f64 = 8.0;
const SHIELD_AIR_GAP: f64 = 38.0;
const THERMAL_LOGGER_POCKETS: usize = 4;
const SHIELD_WINDOW_X: f64 = 300.0;
const SHIELD_WINDOW_Z: f64 = 250.0;

const TOKEN_POS: (f64, f64) = (-440.0, -330.0);
const TOKEN_X: f64 = 360.0;
const TOKEN_Y: f64 = 150.0;
const TOKEN_Z: f64 = 22.0;
const CAL_TOKEN_LANES: usize = 3;
const CAL_TOKENS_PER_LANE: usize = 5;
const DRIFT_STEP_TOKENS: usize = 7;

const TROUGH_POS: (f64, f64) = (-40.0, -330.0);
const TROUGH_X: f64 = 360.0;
const TROUGH_Y: f64 = 150.0;
const TROUGH_Z: f64 = 58.0;
const TROUGH_WALL: f64 = 14.0;
const PRIME_VIAL_WELLS: usize = 6;
const PRIME_VIAL_COLS: usize = 3;
const PRIME_VIAL_D: f64 = 24.0;
const DRAIN_RIBS: usize = 6;
const ABSORBENT_PAD_LANDS: usize = 4;

const LANES_POS: (f64, f64) = (345.0, -330.0);
const LANES_X: f64 = 380.0;
const LANES_Y: f64 = 150.0;
const LANES_Z: f64 = 32.0;
const DISPOSITION_LANES: usize = 3;
const LANE_SLOT_COUNT: usize = 5;
const RELEASE_TOKEN_SLOTS: usize = LANE_SLOT_COUNT;
const HOLD_TOKEN_SLOTS: usize = LANE_SLOT_COUNT;
const REJECT_TOKEN_SLOTS: usize = LANE_SLOT_COUNT;

const CUSTODY_POS: (f64, f64) = (590.0, -120.0);
const CUSTODY_X: f64 = 240.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 6;
const BAG_ID_CARD_SLOTS: usize = 3;
const SCALE_RUN_CARD_SLOTS: usize = 3;

const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 265.0;
const LEFT_BAG_LOAD_CLEARANCE: f64 = 250.0;
const RIGHT_SCALE_SERVICE_CLEARANCE: f64 = 235.0;
const TOP_BAG_LIFT_CLEARANCE: f64 = 320.0;
const TOP_SHIELD_LIFT_CLEARANCE: f64 = 510.0;
const KEEP_OUT_RAIL: f64 = 8.0;
const KEEP_OUT_GROUPS: usize = 6;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - EDGE_MARGIN;
        let usable_y = STATION_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn gate_height(self) -> f64 {
        match self {
            DispositionLane::Release => 20.0,
            DispositionLane::Hold => 34.0,
            DispositionLane::Reject => 48.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    export(OUTPUTS[0], &base_containment_deck());
    export(OUTPUTS[1], &hanging_bag_mode_frame());
    export(OUTPUTS[2], &tray_bag_mode_cradle());
    export(OUTPUTS[3], &load_cell_reference_mass_pocket());
    export(OUTPUTS[4], &tubing_strain_relief_comb());
    export(OUTPUTS[5], &bubble_foam_witness_window());
    export(OUTPUTS[6], &thermal_shield());
    export(OUTPUTS[7], &calibration_token_rail());
    export(OUTPUTS[8], &drain_prime_capture_trough());
    export(OUTPUTS[9], &release_hold_reject_lanes());
    export(OUTPUTS[10], &barcode_custody_panel());
    export(OUTPUTS[11], &robot_service_keepouts());
    export(OUTPUTS[12], &station_assembly());

    println!();
    println!("Closed media/feed bag scale-drift bubble/foam false-mass station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Bag modes:              hanging frame with {HANGER_COUNT} keyed hanger points plus tray cradle with {TRAY_SADDLE_RIBS} saddle ribs and {TRAY_LOCATOR_PINS} locator pins"
    );
    println!(
        "  Scale challenge:        {LOAD_CELL_PADS} load-cell pads, {REFERENCE_MASS_WELLS} reference mass wells, {FINE_MASS_WELLS} fine-mass wells, {CERTIFICATE_SLOTS} certificate slots"
    );
    println!(
        "  Tubing and false mass:  {TUBE_CHANNELS} strain-relief comb channels, {WITNESS_CHANNELS} bubble/foam witness channels, {FALSE_MASS_COUPONS} false-mass coupons"
    );
    println!(
        "  Controls:               {THERMAL_LOGGER_POCKETS} thermal logger pockets, {CAL_TOKEN_LANES} calibration token lanes, {PRIME_VIAL_WELLS} prime vial wells, release/hold/reject lanes"
    );
    println!(
        "  Custody and keepouts:   {BARCODE_LANDS} barcode lands, {BAG_ID_CARD_SLOTS} bag ID card slots, {KEEP_OUT_GROUPS} robot/service keepout groups"
    );
    println!("  Output prefix:          {OUTPUT_PREFIX}");
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
    let col = index % cols;
    let row = index / cols;

    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn station_assembly() -> Part {
    base_containment_deck()
        + hanging_bag_mode_frame().translate(WEIGHING_POS.0, WEIGHING_POS.1, BASE_Z)
        + tray_bag_mode_cradle().translate(
            WEIGHING_POS.0,
            WEIGHING_POS.1 + TRAY_MODE_OFFSET_Y,
            on_deck_z(TRAY_Z),
        )
        + load_cell_reference_mass_pocket().translate(
            LOAD_REF_POS.0,
            LOAD_REF_POS.1,
            on_deck_z(LOAD_REF_Z),
        )
        + tubing_strain_relief_comb().translate(COMB_POS.0, COMB_POS.1, on_deck_z(COMB_Z))
        + bubble_foam_witness_window().translate(BUBBLE_POS.0, BUBBLE_POS.1, on_deck_z(BUBBLE_Z))
        + thermal_shield().translate(WEIGHING_POS.0, WEIGHING_POS.1, BASE_Z)
        + calibration_token_rail().translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + drain_prime_capture_trough().translate(TROUGH_POS.0, TROUGH_POS.1, on_deck_z(TROUGH_Z))
        + release_hold_reject_lanes().translate(LANES_POS.0, LANES_POS.1, on_deck_z(LANES_Z))
        + barcode_custody_panel().translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + robot_service_keepouts()
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        name("base_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let broad_sump = centered_cube(
        name("base_false_mass_washdown_sump_cut"),
        STATION_X - 2.0 * (RIM_W + 50.0),
        STATION_Y - 2.0 * (RIM_W + 52.0),
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z - 4.0);
    let weighing_sump = centered_cube(
        name("base_weighing_zone_drip_shadow_cut"),
        WEIGHING_ENVELOPE_X + 80.0,
        WEIGHING_ENVELOPE_Y - 60.0,
        10.0,
    )
    .translate(WEIGHING_POS.0, WEIGHING_POS.1 - 25.0, BASE_Z - 5.0);
    let front_drain = centered_cylinder(
        name("base_front_prime_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        62.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -STATION_X / 2.0 + 92.0,
        -STATION_Y / 2.0 + 42.0,
        BASE_Z - 7.0,
    );

    deck - broad_sump - weighing_sump - front_drain - module_sockets() - mounting_holes()
        + perimeter_rims()
        + workflow_dividers()
        + base_datum_targets()
        + drip_flow_ribs()
        + mode_label_lands()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty(name("base_module_socket_cuts"));
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("base_socket_{}", module.name)),
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

fn mounting_holes() -> Part {
    let mut holes = Part::empty(name("base_mounting_hole_cuts"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 56.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 56.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 56.0),
        (0.0, -STATION_Y / 2.0 + 56.0),
        (0.0, STATION_Y / 2.0 - 56.0),
        (-STATION_X / 2.0 + 56.0, 0.0),
        (STATION_X / 2.0 - 56.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                name(&format!("base_m6_mount_clearance_{i}")),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(name("base_front_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("base_rear_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(name("base_left_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(name("base_right_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn workflow_dividers() -> Part {
    let upper_row = centered_cube(
        name("base_weighing_to_witness_row_divider"),
        STATION_X - 190.0,
        10.0,
        28.0,
    )
    .translate(0.0, 155.0, BASE_Z + 14.0);
    let lower_row = centered_cube(
        name("base_witness_to_disposition_row_divider"),
        STATION_X - 220.0,
        10.0,
        28.0,
    )
    .translate(0.0, -185.0, BASE_Z + 14.0);
    let bag_mode_split = centered_cube(
        name("base_hanging_mode_to_tray_mode_centerline"),
        WEIGHING_ENVELOPE_X - 40.0,
        8.0,
        24.0,
    )
    .translate(WEIGHING_POS.0, WEIGHING_POS.1 - 95.0, BASE_Z + 12.0);
    let custody_split = centered_cube(name("base_custody_from_witness_boundary"), 8.0, 180.0, 24.0)
        .translate(470.0, 68.0, BASE_Z + 12.0);

    upper_row + lower_row + bag_mode_split + custody_split
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty(name("base_robot_datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 90.0, -STATION_Y / 2.0 + 90.0),
        (STATION_X / 2.0 - 90.0, -STATION_Y / 2.0 + 90.0),
        (-STATION_X / 2.0 + 90.0, STATION_Y / 2.0 - 90.0),
        (STATION_X / 2.0 - 90.0, STATION_Y / 2.0 - 90.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("base_datum_target_{i}")).translate(*x, *y, BASE_Z + 3.0);
    }
    targets
}

fn drip_flow_ribs() -> Part {
    let mut ribs = Part::empty(name("base_drain_flow_witness_ribs"));
    for i in 0..DRAIN_RIBS {
        ribs = ribs
            + centered_cube(
                name(&format!("base_drain_flow_rib_{i}")),
                STATION_X - 260.0,
                7.0,
                6.0,
            )
            .translate(0.0, -360.0 + i as f64 * 116.0, BASE_Z + 3.0);
    }
    ribs
}

fn mode_label_lands() -> Part {
    label_land("base_hanging_mode_land", 180.0, 24.0, 4).translate(
        WEIGHING_POS.0,
        WEIGHING_POS.1 + 205.0,
        BASE_Z + 3.0,
    ) + label_land("base_tray_mode_land", 180.0, 24.0, 4).translate(
        WEIGHING_POS.0,
        WEIGHING_POS.1 + TRAY_MODE_OFFSET_Y - 104.0,
        BASE_Z + 3.0,
    ) + label_land("base_false_mass_witness_land", 205.0, 24.0, 5).translate(
        BUBBLE_POS.0,
        BUBBLE_POS.1 + BUBBLE_Y / 2.0 - 20.0,
        BASE_Z + 3.0,
    )
}

fn hanging_bag_mode_frame() -> Part {
    frame_posts()
        + frame_top_beams()
        + frame_lower_cross_ties()
        + keyed_hanger_points()
        + hanging_bag_clearance_gauge()
        + anti_sway_pads()
        + hanging_mode_scale_linkage()
}

fn frame_posts() -> Part {
    let mut posts = Part::empty(name("hanging_frame_posts"));
    for (i, (x, y)) in frame_corner_positions().iter().enumerate() {
        posts = posts
            + centered_cube(
                name(&format!("hanging_frame_post_{i}")),
                FRAME_POST_W,
                FRAME_POST_W,
                HANGING_FRAME_Z,
            )
            .translate(*x, *y, HANGING_FRAME_Z / 2.0);
    }
    posts
}

fn frame_top_beams() -> Part {
    let front = centered_cube(
        name("hanging_frame_top_front_beam"),
        HANGING_FRAME_X,
        FRAME_BEAM_W,
        FRAME_BEAM_W,
    )
    .translate(
        0.0,
        -HANGING_FRAME_Y / 2.0 + FRAME_BEAM_W / 2.0,
        HANGING_FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let rear = centered_cube(
        name("hanging_frame_top_rear_beam"),
        HANGING_FRAME_X,
        FRAME_BEAM_W,
        FRAME_BEAM_W,
    )
    .translate(
        0.0,
        HANGING_FRAME_Y / 2.0 - FRAME_BEAM_W / 2.0,
        HANGING_FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let left = centered_cube(
        name("hanging_frame_top_left_beam"),
        FRAME_BEAM_W,
        HANGING_FRAME_Y,
        FRAME_BEAM_W,
    )
    .translate(
        -HANGING_FRAME_X / 2.0 + FRAME_BEAM_W / 2.0,
        0.0,
        HANGING_FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let right = centered_cube(
        name("hanging_frame_top_right_beam"),
        FRAME_BEAM_W,
        HANGING_FRAME_Y,
        FRAME_BEAM_W,
    )
    .translate(
        HANGING_FRAME_X / 2.0 - FRAME_BEAM_W / 2.0,
        0.0,
        HANGING_FRAME_Z - FRAME_BEAM_W / 2.0,
    );
    let hook_rail = centered_cylinder(
        name("hanging_frame_load_hook_rail"),
        HANGER_PIN_D / 2.0,
        HANGING_FRAME_X - 92.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, HANGING_FRAME_Z - 72.0);

    front + rear + left + right + hook_rail
}

fn frame_lower_cross_ties() -> Part {
    let front = centered_cube(
        name("hanging_frame_lower_front_tie"),
        HANGING_FRAME_X - 48.0,
        18.0,
        24.0,
    )
    .translate(0.0, -HANGING_FRAME_Y / 2.0 + 26.0, 132.0);
    let rear = centered_cube(
        name("hanging_frame_lower_rear_tie"),
        HANGING_FRAME_X - 48.0,
        18.0,
        24.0,
    )
    .translate(0.0, HANGING_FRAME_Y / 2.0 - 26.0, 132.0);
    let left = centered_cube(
        name("hanging_frame_lower_left_tie"),
        18.0,
        HANGING_FRAME_Y - 48.0,
        24.0,
    )
    .translate(-HANGING_FRAME_X / 2.0 + 26.0, 0.0, 132.0);
    let right = centered_cube(
        name("hanging_frame_lower_right_tie"),
        18.0,
        HANGING_FRAME_Y - 48.0,
        24.0,
    )
    .translate(HANGING_FRAME_X / 2.0 - 26.0, 0.0, 132.0);

    front + rear + left + right
}

fn keyed_hanger_points() -> Part {
    let mut hangers = Part::empty(name("hanging_frame_keyed_hanger_points"));
    for i in 0..HANGER_COUNT {
        let x = centered_index(i, HANGER_COUNT, HANGER_PITCH_X);
        let lug = centered_cube(
            name(&format!("hanging_frame_bag_hanger_lug_{i}")),
            34.0,
            24.0,
            42.0,
        )
        .translate(x, 0.0, HANGING_FRAME_Z - 112.0);
        let pin = centered_cylinder(
            name(&format!("hanging_frame_bag_hanger_pin_{i}")),
            HANGER_PIN_D / 2.0,
            46.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, HANGING_FRAME_Z - 112.0);
        let asymmetric_key = centered_cube(
            name(&format!("hanging_frame_asymmetric_key_flag_{i}")),
            16.0,
            28.0,
            10.0,
        )
        .translate(x + 9.0, 0.0, HANGING_FRAME_Z - 90.0);
        hangers = hangers + lug + pin + asymmetric_key;
    }
    hangers
}

fn hanging_bag_clearance_gauge() -> Part {
    let upper = centered_cube(
        name("hanging_bag_upper_swing_clearance_gauge"),
        HANGING_BAG_CLEAR_X,
        7.0,
        7.0,
    )
    .translate(0.0, -HANGING_BAG_CLEAR_Y / 2.0, HANGING_FRAME_Z - 185.0);
    let lower = centered_cube(
        name("hanging_bag_lower_swing_clearance_gauge"),
        HANGING_BAG_CLEAR_X * 0.76,
        7.0,
        7.0,
    )
    .translate(
        0.0,
        -HANGING_BAG_CLEAR_Y / 2.0,
        HANGING_FRAME_Z - HANGING_BAG_CLEAR_Z,
    );
    let left = centered_cube(
        name("hanging_bag_left_clearance_gauge"),
        7.0,
        7.0,
        HANGING_BAG_CLEAR_Z - 150.0,
    )
    .translate(
        -HANGING_BAG_CLEAR_X / 2.0,
        -HANGING_BAG_CLEAR_Y / 2.0,
        HANGING_FRAME_Z - 265.0,
    );
    let right = centered_cube(
        name("hanging_bag_right_clearance_gauge"),
        7.0,
        7.0,
        HANGING_BAG_CLEAR_Z - 150.0,
    )
    .translate(
        HANGING_BAG_CLEAR_X / 2.0,
        -HANGING_BAG_CLEAR_Y / 2.0,
        HANGING_FRAME_Z - 265.0,
    );
    let load_shadow = centered_cube(
        name("hanging_bag_center_load_shadow_gauge"),
        HANGING_BAG_CLEAR_X * 0.66,
        HANGING_BAG_CLEAR_Y,
        10.0,
    )
    .translate(0.0, 0.0, 118.0);

    upper + lower + left + right + load_shadow
}

fn anti_sway_pads() -> Part {
    let mut pads = Part::empty(name("hanging_bag_anti_sway_pads"));
    for i in 0..ANTI_SWAY_PAD_COUNT {
        let x = if i % 2 == 0 { -118.0 } else { 118.0 };
        let y = if i < 2 {
            -HANGING_FRAME_Y / 2.0 - 9.0
        } else {
            HANGING_FRAME_Y / 2.0 + 9.0
        };
        pads = pads
            + centered_cube(
                name(&format!("hanging_bag_anti_sway_pad_{i}")),
                54.0,
                18.0,
                82.0,
            )
            .translate(x, y, 250.0);
    }
    pads
}

fn hanging_mode_scale_linkage() -> Part {
    let downrod = centered_cylinder(name("hanging_mode_scale_linkage_downrod"), 6.0, 100.0, 28)
        .translate(0.0, 0.0, 74.0);
    let load_clevis = centered_cube(
        name("hanging_mode_scale_linkage_load_clevis"),
        68.0,
        24.0,
        28.0,
    )
    .translate(0.0, 0.0, 24.0);
    let isolated_gap_gauge = centered_cube(
        name("hanging_mode_scale_isolation_gap_gauge"),
        150.0,
        10.0,
        10.0,
    )
    .translate(0.0, HANGING_FRAME_Y / 2.0 - 36.0, 88.0);

    downrod + load_clevis + isolated_gap_gauge
}

fn tray_bag_mode_cradle() -> Part {
    let body = centered_cube(name("tray_bag_mode_cradle_body"), TRAY_X, TRAY_Y, TRAY_Z);
    let pocket = centered_cube(
        name("tray_bag_mode_soft_bag_pocket_cut"),
        TRAY_POCKET_X,
        TRAY_POCKET_Y,
        TRAY_POCKET_DEPTH,
    )
    .translate(0.0, 0.0, TRAY_Z / 2.0 - TRAY_POCKET_DEPTH / 2.0 + 0.2);
    let drain_notch = centered_cube(
        name("tray_bag_mode_drain_corner_notch_cut"),
        70.0,
        22.0,
        TRAY_Z + 4.0,
    )
    .translate(TRAY_X / 2.0 - 45.0, -TRAY_Y / 2.0 + 18.0, 0.0);

    body - pocket - drain_notch + tray_saddle_ribs() + tray_locator_pins() + tray_mode_id_flags()
}

fn tray_saddle_ribs() -> Part {
    let mut ribs = Part::empty(name("tray_bag_mode_saddle_ribs"));
    for i in 0..TRAY_SADDLE_RIBS {
        ribs = ribs
            + centered_cube(
                name(&format!("tray_bag_mode_saddle_rib_{i}")),
                10.0,
                TRAY_POCKET_Y + 12.0,
                12.0,
            )
            .translate(
                centered_index(i, TRAY_SADDLE_RIBS, 38.0),
                0.0,
                TRAY_Z / 2.0 + 6.0,
            );
    }
    ribs
}

fn tray_locator_pins() -> Part {
    let mut pins = Part::empty(name("tray_bag_mode_locator_pins"));
    for i in 0..TRAY_LOCATOR_PINS {
        pins = pins
            + centered_cylinder(
                name(&format!("tray_bag_mode_locator_pin_{i}")),
                5.0,
                20.0,
                24,
            )
            .translate(
                centered_index(i % 3, 3, 122.0),
                if i < 3 { -86.0 } else { 86.0 },
                TRAY_Z / 2.0 + 10.0,
            );
    }
    pins
}

fn tray_mode_id_flags() -> Part {
    let mut flags = Part::empty(name("tray_bag_mode_id_flags"));
    for i in 0..TRAY_MODE_ID_FLAGS {
        flags = flags
            + centered_cube(
                name(&format!("tray_bag_mode_id_flag_{i}")),
                42.0,
                10.0,
                18.0 + i as f64 * 4.0,
            )
            .translate(
                centered_index(i, TRAY_MODE_ID_FLAGS, 54.0),
                TRAY_Y / 2.0 - 18.0,
                TRAY_Z / 2.0 + 9.0 + i as f64 * 2.0,
            );
    }
    flags
}

fn load_cell_reference_mass_pocket() -> Part {
    let body = centered_cube(
        name("load_cell_reference_mass_pocket_body"),
        LOAD_REF_X,
        LOAD_REF_Y,
        LOAD_REF_Z,
    );
    let bridge_socket = centered_cube(
        name("load_cell_reference_mass_bridge_socket_cut"),
        150.0,
        70.0,
        18.0,
    )
    .translate(-72.0, 0.0, LOAD_REF_Z / 2.0 - 9.0);
    let cable_exit = centered_cube(
        name("load_cell_reference_mass_cable_exit_cut"),
        34.0,
        LOAD_REF_Y + 8.0,
        18.0,
    )
    .translate(-150.0, 0.0, -4.0);

    body - bridge_socket - cable_exit - reference_mass_well_cuts() - fine_mass_well_cuts()
        + load_cell_mount_pads()
        + certificate_slots()
        + reference_mass_custody_ribs()
}

fn load_cell_mount_pads() -> Part {
    let mut pads = Part::empty(name("load_cell_mount_pads"));
    for (i, (x, y)) in [
        (-116.0, -62.0),
        (-28.0, -62.0),
        (-116.0, 62.0),
        (-28.0, 62.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cube(
            name(&format!("load_cell_mount_pad_{i}")),
            LOAD_CELL_PAD_X,
            LOAD_CELL_PAD_Y,
            12.0,
        )
        .translate(*x, *y, LOAD_REF_Z / 2.0 + 6.0);
        let fastener_a = centered_cylinder(
            name(&format!("load_cell_mount_pad_{i}_fastener_a")),
            3.2,
            16.0,
            20,
        )
        .translate(*x - 22.0, *y, LOAD_REF_Z / 2.0 + 6.0);
        let fastener_b = centered_cylinder(
            name(&format!("load_cell_mount_pad_{i}_fastener_b")),
            3.2,
            16.0,
            20,
        )
        .translate(*x + 22.0, *y, LOAD_REF_Z / 2.0 + 6.0);
        pads = pads + (pad - fastener_a - fastener_b);
    }
    pads
}

fn reference_mass_well_cuts() -> Part {
    let rows = REFERENCE_MASS_WELLS.div_ceil(REFERENCE_MASS_COLS);
    let mut cuts = Part::empty(name("reference_mass_well_cuts"));
    for i in 0..REFERENCE_MASS_WELLS {
        let (x, y) = grid_xy(i, REFERENCE_MASS_COLS, rows, 48.0, 52.0);
        cuts = cuts
            + centered_cylinder(
                name(&format!("reference_mass_primary_well_cut_{i}")),
                REFERENCE_MASS_D / 2.0,
                LOAD_REF_Z + 6.0,
                32,
            )
            .translate(x + 94.0, y, LOAD_REF_Z / 2.0);
    }
    cuts
}

fn fine_mass_well_cuts() -> Part {
    let mut cuts = Part::empty(name("fine_reference_mass_well_cuts"));
    for i in 0..FINE_MASS_WELLS {
        cuts = cuts
            + centered_cylinder(
                name(&format!("fine_reference_mass_well_cut_{i}")),
                FINE_MASS_D / 2.0,
                LOAD_REF_Z + 6.0,
                24,
            )
            .translate(
                145.0,
                centered_index(i, FINE_MASS_WELLS, 25.0),
                LOAD_REF_Z / 2.0,
            );
    }
    cuts
}

fn certificate_slots() -> Part {
    let mut slots = Part::empty(name("reference_mass_certificate_slots"));
    for i in 0..CERTIFICATE_SLOTS {
        slots = slots
            + centered_cube(
                name(&format!("reference_mass_certificate_slot_{i}")),
                62.0,
                8.0,
                12.0,
            )
            .translate(
                18.0,
                centered_index(i, CERTIFICATE_SLOTS, 48.0),
                LOAD_REF_Z / 2.0 + 8.0,
            );
    }
    slots
}

fn reference_mass_custody_ribs() -> Part {
    let left_rib = centered_cube(
        name("reference_mass_custody_left_boundary_rib"),
        8.0,
        LOAD_REF_Y - 28.0,
        18.0,
    )
    .translate(36.0, 0.0, LOAD_REF_Z / 2.0 + 9.0);
    let right_rib = centered_cube(
        name("reference_mass_custody_right_boundary_rib"),
        8.0,
        LOAD_REF_Y - 28.0,
        18.0,
    )
    .translate(164.0, 0.0, LOAD_REF_Z / 2.0 + 9.0);
    let status_flag = centered_cube(
        name("reference_mass_traceability_status_flag"),
        92.0,
        14.0,
        20.0,
    )
    .translate(98.0, -LOAD_REF_Y / 2.0 + 24.0, LOAD_REF_Z / 2.0 + 10.0);

    left_rib + right_rib + status_flag
}

fn tubing_strain_relief_comb() -> Part {
    let body = centered_cube(
        name("tubing_strain_relief_comb_body"),
        COMB_X,
        COMB_Y,
        COMB_Z,
    );
    let mut cuts = Part::empty(name("tubing_strain_relief_comb_channel_cuts"));
    for i in 0..TUBE_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                name(&format!("tubing_strain_relief_channel_cut_{i}")),
                TUBE_CHANNEL_D / 2.0,
                COMB_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH),
                0.0,
                8.0,
            );
    }

    body - cuts + strain_loop_gauges() + tubing_clamp_flags() + comb_direction_arrows()
}

fn strain_loop_gauges() -> Part {
    let mut gauges = Part::empty(name("strain_relief_loop_radius_gauges"));
    for i in 0..STRAIN_LOOP_GAUGES {
        gauges = gauges
            + centered_cylinder(
                name(&format!("strain_relief_loop_radius_gauge_{i}")),
                14.0 + i as f64 * 4.0,
                5.0,
                40,
            )
            .translate(
                centered_index(i, STRAIN_LOOP_GAUGES, 68.0),
                -42.0,
                COMB_Z / 2.0 + 2.5,
            );
    }
    gauges
}

fn tubing_clamp_flags() -> Part {
    let mut flags = Part::empty(name("tubing_strain_relief_clamp_flags"));
    for i in 0..CLAMP_FLAG_COUNT {
        flags = flags
            + centered_cube(
                name(&format!("tubing_strain_relief_clamp_flag_{i}")),
                18.0,
                8.0,
                18.0,
            )
            .translate(
                centered_index(i, CLAMP_FLAG_COUNT, TUBE_CHANNEL_PITCH),
                COMB_Y / 2.0 - 10.0,
                COMB_Z / 2.0 + 9.0,
            );
    }
    flags
}

fn comb_direction_arrows() -> Part {
    let inlet = centered_cube(
        name("tubing_comb_inlet_direction_bar"),
        COMB_X - 58.0,
        6.0,
        8.0,
    )
    .translate(0.0, -COMB_Y / 2.0 + 18.0, COMB_Z / 2.0 + 4.0);
    let outlet = centered_cube(
        name("tubing_comb_outlet_direction_bar"),
        COMB_X - 58.0,
        6.0,
        8.0,
    )
    .translate(0.0, COMB_Y / 2.0 - 18.0, COMB_Z / 2.0 + 4.0);
    inlet + outlet
}

fn bubble_foam_witness_window() -> Part {
    let body = centered_cube(
        name("bubble_foam_witness_window_body"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );
    let transparent_window_cut = centered_cube(
        name("bubble_foam_witness_transparent_panel_window_cut"),
        WITNESS_CHANNEL_X + 40.0,
        BUBBLE_Y - 58.0,
        18.0,
    )
    .translate(-26.0, 0.0, BUBBLE_Z / 2.0 - 9.0);

    body - transparent_window_cut - witness_channel_cuts()
        + bubble_graduation_ticks()
        + foam_headspace_ladders()
        + false_mass_coupon_nests()
        + camera_fiducial_posts()
}

fn witness_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("bubble_foam_witness_channel_cuts"));
    for i in 0..WITNESS_CHANNELS {
        cuts = cuts
            + centered_cube(
                name(&format!("bubble_foam_witness_channel_cut_{i}")),
                WITNESS_CHANNEL_X,
                WITNESS_CHANNEL_Y,
                BUBBLE_Z + 6.0,
            )
            .translate(
                -28.0,
                centered_index(i, WITNESS_CHANNELS, WITNESS_CHANNEL_PITCH_Y),
                0.0,
            );
    }
    cuts
}

fn bubble_graduation_ticks() -> Part {
    let mut ticks = Part::empty(name("bubble_foam_witness_graduation_ticks"));
    for lane in 0..WITNESS_CHANNELS {
        let y = centered_index(lane, WITNESS_CHANNELS, WITNESS_CHANNEL_PITCH_Y);
        for tick in 0..BUBBLE_GRADUATION_TICKS {
            let major = tick % 5 == 0;
            ticks = ticks
                + centered_cube(
                    name(&format!(
                        "bubble_witness_lane_{lane}_graduation_tick_{tick}"
                    )),
                    if major { 22.0 } else { 12.0 },
                    4.0,
                    6.0,
                )
                .translate(
                    -190.0 + tick as f64 * 36.0,
                    y + 15.0,
                    BUBBLE_Z / 2.0 + 3.0,
                );
        }
    }
    ticks
}

fn foam_headspace_ladders() -> Part {
    let mut ladders = Part::empty(name("foam_headspace_level_ladders"));
    for i in 0..FOAM_HEADSPACE_LEVELS {
        ladders = ladders
            + centered_cube(
                name(&format!("foam_headspace_level_ladder_{i}")),
                42.0,
                6.0,
                8.0,
            )
            .translate(
                198.0,
                centered_index(i, FOAM_HEADSPACE_LEVELS, 32.0),
                BUBBLE_Z / 2.0 + 4.0,
            );
    }
    ladders
}

fn false_mass_coupon_nests() -> Part {
    let mut nests = Part::empty(name("false_mass_coupon_nests"));
    for i in 0..FALSE_MASS_COUPONS {
        let x = 168.0 + (i % 2) as f64 * 45.0;
        let y = centered_index(i / 2, FALSE_MASS_COUPONS / 2, 42.0);
        let land = centered_cube(
            name(&format!("false_mass_coupon_land_{i}")),
            34.0,
            28.0,
            10.0,
        )
        .translate(x, y, BUBBLE_Z / 2.0 + 5.0);
        let center_cut = centered_cylinder(
            name(&format!("false_mass_coupon_index_cut_{i}")),
            5.0,
            12.0,
            24,
        )
        .translate(x, y, BUBBLE_Z / 2.0 + 5.0);
        nests = nests + (land - center_cut);
    }
    nests
}

fn camera_fiducial_posts() -> Part {
    let mut posts = Part::empty(name("bubble_foam_camera_fiducial_posts"));
    for (i, (x, y)) in [
        (-BUBBLE_X / 2.0 + 34.0, -BUBBLE_Y / 2.0 + 28.0),
        (BUBBLE_X / 2.0 - 34.0, -BUBBLE_Y / 2.0 + 28.0),
        (-BUBBLE_X / 2.0 + 34.0, BUBBLE_Y / 2.0 - 28.0),
        (BUBBLE_X / 2.0 - 34.0, BUBBLE_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + fiducial_disc(&format!("bubble_foam_camera_fiducial_{i}")).translate(
                *x,
                *y,
                BUBBLE_Z / 2.0 + 4.0,
            );
    }
    posts
}

fn thermal_shield() -> Part {
    let rear = centered_cube(
        name("thermal_shield_rear_panel"),
        THERMAL_SHIELD_X,
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Z,
    )
    .translate(0.0, THERMAL_SHIELD_Y / 2.0, THERMAL_SHIELD_Z / 2.0);
    let left = centered_cube(
        name("thermal_shield_left_panel"),
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Y,
        THERMAL_SHIELD_Z,
    )
    .translate(-THERMAL_SHIELD_X / 2.0, 0.0, THERMAL_SHIELD_Z / 2.0);
    let right = centered_cube(
        name("thermal_shield_right_panel"),
        SHIELD_PANEL_T,
        THERMAL_SHIELD_Y,
        THERMAL_SHIELD_Z,
    )
    .translate(THERMAL_SHIELD_X / 2.0, 0.0, THERMAL_SHIELD_Z / 2.0);
    let top_lip = centered_cube(
        name("thermal_shield_top_bag_lift_lip"),
        THERMAL_SHIELD_X,
        THERMAL_SHIELD_Y,
        SHIELD_PANEL_T,
    )
    .translate(0.0, 0.0, THERMAL_SHIELD_Z + SHIELD_PANEL_T / 2.0);

    rear + left + right + top_lip - thermal_observation_window_cuts()
        + thermal_logger_pockets()
        + thermal_air_gap_gauges()
}

fn thermal_observation_window_cuts() -> Part {
    let rear_window = centered_cube(
        name("thermal_shield_rear_bag_observation_window_cut"),
        SHIELD_WINDOW_X,
        SHIELD_PANEL_T + 4.0,
        SHIELD_WINDOW_Z,
    )
    .translate(0.0, THERMAL_SHIELD_Y / 2.0, 250.0);
    let left_window = centered_cube(
        name("thermal_shield_left_scale_observation_window_cut"),
        SHIELD_PANEL_T + 4.0,
        220.0,
        SHIELD_WINDOW_Z * 0.76,
    )
    .translate(-THERMAL_SHIELD_X / 2.0, -92.0, 220.0);
    let right_window = centered_cube(
        name("thermal_shield_right_service_observation_window_cut"),
        SHIELD_PANEL_T + 4.0,
        220.0,
        SHIELD_WINDOW_Z * 0.76,
    )
    .translate(THERMAL_SHIELD_X / 2.0, -92.0, 220.0);

    rear_window + left_window + right_window
}

fn thermal_logger_pockets() -> Part {
    let mut pockets = Part::empty(name("thermal_shield_logger_pockets"));
    for i in 0..THERMAL_LOGGER_POCKETS {
        pockets = pockets
            + centered_cube(
                name(&format!("thermal_logger_pocket_{i}")),
                52.0,
                10.0,
                40.0,
            )
            .translate(
                centered_index(i % 2, 2, 150.0),
                THERMAL_SHIELD_Y / 2.0 - 9.0,
                95.0 + (i / 2) as f64 * 245.0,
            );
    }
    pockets
}

fn thermal_air_gap_gauges() -> Part {
    let left_gap = centered_cube(name("thermal_shield_left_air_gap_gauge"), 8.0, 210.0, 8.0)
        .translate(-HANGING_FRAME_X / 2.0 - SHIELD_AIR_GAP, 0.0, 112.0);
    let right_gap = centered_cube(name("thermal_shield_right_air_gap_gauge"), 8.0, 210.0, 8.0)
        .translate(HANGING_FRAME_X / 2.0 + SHIELD_AIR_GAP, 0.0, 112.0);
    let rear_gap = centered_cube(name("thermal_shield_rear_air_gap_gauge"), 220.0, 8.0, 8.0)
        .translate(0.0, HANGING_FRAME_Y / 2.0 + SHIELD_AIR_GAP, 112.0);

    left_gap + right_gap + rear_gap
}

fn calibration_token_rail() -> Part {
    let body = centered_cube(
        name("calibration_token_rail_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    body + calibration_token_lanes() + drift_step_token_lands() + calibration_rail_end_stops()
}

fn calibration_token_lanes() -> Part {
    let mut lanes = Part::empty(name("calibration_token_lanes"));
    for lane in 0..CAL_TOKEN_LANES {
        let y = centered_index(lane, CAL_TOKEN_LANES, 42.0);
        lanes = lanes
            + centered_cube(
                name(&format!("calibration_token_lane_{lane}_rail")),
                TOKEN_X - 40.0,
                7.0,
                9.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 4.5);
        for slot in 0..CAL_TOKENS_PER_LANE {
            lanes = lanes
                + centered_cylinder(
                    name(&format!("calibration_token_lane_{lane}_slot_{slot}")),
                    10.0,
                    7.0,
                    28,
                )
                .translate(
                    centered_index(slot, CAL_TOKENS_PER_LANE, 58.0),
                    y,
                    TOKEN_Z / 2.0 + 8.0,
                );
        }
    }
    lanes
}

fn drift_step_token_lands() -> Part {
    let mut lands = Part::empty(name("drift_step_token_lands"));
    for i in 0..DRIFT_STEP_TOKENS {
        lands = lands
            + centered_cube(name(&format!("drift_step_token_land_{i}")), 30.0, 18.0, 6.0)
                .translate(
                    centered_index(i, DRIFT_STEP_TOKENS, 42.0),
                    -TOKEN_Y / 2.0 + 18.0,
                    TOKEN_Z / 2.0 + 3.0,
                );
    }
    lands
}

fn calibration_rail_end_stops() -> Part {
    let left = centered_cube(
        name("calibration_token_rail_left_end_stop"),
        12.0,
        TOKEN_Y,
        24.0,
    )
    .translate(-TOKEN_X / 2.0 + 6.0, 0.0, TOKEN_Z / 2.0 + 12.0);
    let right = centered_cube(
        name("calibration_token_rail_right_end_stop"),
        12.0,
        TOKEN_Y,
        24.0,
    )
    .translate(TOKEN_X / 2.0 - 6.0, 0.0, TOKEN_Z / 2.0 + 12.0);
    left + right
}

fn drain_prime_capture_trough() -> Part {
    let body = centered_cube(
        name("drain_prime_capture_trough_body"),
        TROUGH_X,
        TROUGH_Y,
        TROUGH_Z,
    );
    let basin = centered_cube(
        name("drain_prime_capture_trough_basin_cut"),
        TROUGH_X - 2.0 * TROUGH_WALL,
        TROUGH_Y - 2.0 * TROUGH_WALL,
        TROUGH_Z - 12.0,
    )
    .translate(0.0, 0.0, 8.0);
    let drain_cut = centered_cylinder(
        name("drain_prime_capture_trough_drain_interface_cut"),
        10.0,
        TROUGH_X + 10.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -TROUGH_Y / 2.0 + 20.0, -8.0);

    body - basin - drain_cut + prime_vial_wells() + absorbent_pad_lands() + drain_witness_ribs()
}

fn prime_vial_wells() -> Part {
    let rows = PRIME_VIAL_WELLS.div_ceil(PRIME_VIAL_COLS);
    let mut wells = Part::empty(name("prime_vial_well_lands"));
    for i in 0..PRIME_VIAL_WELLS {
        let (x, y) = grid_xy(i, PRIME_VIAL_COLS, rows, 48.0, 42.0);
        let land = centered_cylinder(
            name(&format!("prime_vial_well_land_{i}")),
            PRIME_VIAL_D / 2.0 + 6.0,
            8.0,
            32,
        )
        .translate(x - 72.0, y, TROUGH_Z / 2.0 + 4.0);
        let cup = centered_cylinder(
            name(&format!("prime_vial_well_center_cut_{i}")),
            PRIME_VIAL_D / 2.0,
            10.0,
            28,
        )
        .translate(x - 72.0, y, TROUGH_Z / 2.0 + 4.0);
        wells = wells + (land - cup);
    }
    wells
}

fn absorbent_pad_lands() -> Part {
    let mut pads = Part::empty(name("prime_drain_absorbent_pad_lands"));
    for i in 0..ABSORBENT_PAD_LANDS {
        pads = pads
            + centered_cube(name(&format!("absorbent_pad_land_{i}")), 46.0, 26.0, 6.0).translate(
                88.0,
                centered_index(i, ABSORBENT_PAD_LANDS, 30.0),
                TROUGH_Z / 2.0 + 3.0,
            );
    }
    pads
}

fn drain_witness_ribs() -> Part {
    let mut ribs = Part::empty(name("drain_prime_capture_witness_ribs"));
    for i in 0..DRAIN_RIBS {
        ribs = ribs
            + centered_cube(
                name(&format!("drain_prime_capture_rib_{i}")),
                220.0,
                5.0,
                6.0,
            )
            .translate(
                0.0,
                -TROUGH_Y / 2.0 + 28.0 + i as f64 * 17.0,
                TROUGH_Z / 2.0 + 3.0,
            );
    }
    ribs
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        name("release_hold_reject_lanes_body"),
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    body + disposition_lanes() + disposition_gate_flags()
}

fn disposition_lanes() -> Part {
    let mut lanes = Part::empty(name("release_hold_reject_lane_slots"));
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), DISPOSITION_LANES, 122.0);
        let rail = centered_cube(
            name(&format!("{}_lane_center_rail", lane.label())),
            96.0,
            LANES_Y - 26.0,
            8.0,
        )
        .translate(x, 0.0, LANES_Z / 2.0 + 4.0);
        lanes = lanes + rail;
        for slot in 0..LANE_SLOT_COUNT {
            lanes = lanes
                + centered_cube(
                    name(&format!("{}_lane_token_slot_{slot}", lane.label())),
                    34.0,
                    16.0,
                    10.0,
                )
                .translate(
                    x,
                    centered_index(slot, LANE_SLOT_COUNT, 24.0),
                    LANES_Z / 2.0 + 5.0,
                );
        }
    }
    lanes
}

fn disposition_gate_flags() -> Part {
    let mut flags = Part::empty(name("release_hold_reject_gate_flags"));
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), DISPOSITION_LANES, 122.0);
        flags = flags
            + centered_cube(
                name(&format!("{}_lane_gate_flag", lane.label())),
                76.0,
                10.0,
                lane.gate_height(),
            )
            .translate(
                x,
                LANES_Y / 2.0 - 16.0,
                LANES_Z / 2.0 + lane.gate_height() / 2.0,
            );
    }
    flags
}

fn barcode_custody_panel() -> Part {
    let body = centered_cube(
        name("barcode_custody_panel_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    body + barcode_lands() + custody_card_slots() + custody_boundary_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(name("barcode_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + label_land(&format!("barcode_land_{i}"), 74.0, 18.0, 4).translate(
                centered_index(i % 2, 2, 88.0),
                centered_index(i / 2, BARCODE_LANDS / 2, 28.0),
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty(name("bag_scale_run_custody_card_slots"));
    for i in 0..BAG_ID_CARD_SLOTS {
        slots = slots
            + centered_cube(name(&format!("bag_id_card_slot_{i}")), 84.0, 6.0, 10.0).translate(
                -54.0,
                -CUSTODY_Y / 2.0 + 18.0 + i as f64 * 16.0,
                CUSTODY_Z / 2.0 + 5.0,
            );
    }
    for i in 0..SCALE_RUN_CARD_SLOTS {
        slots = slots
            + centered_cube(name(&format!("scale_run_card_slot_{i}")), 84.0, 6.0, 10.0).translate(
                54.0,
                -CUSTODY_Y / 2.0 + 18.0 + i as f64 * 16.0,
                CUSTODY_Z / 2.0 + 5.0,
            );
    }
    slots
}

fn custody_boundary_tabs() -> Part {
    let top_tab = centered_cube(
        name("barcode_custody_chain_top_boundary_tab"),
        CUSTODY_X - 20.0,
        6.0,
        18.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 - 6.0, CUSTODY_Z / 2.0 + 9.0);
    let bottom_tab = centered_cube(
        name("barcode_custody_chain_bottom_boundary_tab"),
        CUSTODY_X - 20.0,
        6.0,
        18.0,
    )
    .translate(0.0, -CUSTODY_Y / 2.0 + 6.0, CUSTODY_Z / 2.0 + 9.0);
    top_tab + bottom_tab
}

fn robot_service_keepouts() -> Part {
    keepout_rect(
        "front_robot_sweep_keepout",
        (0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0),
        STATION_X - 160.0,
        FRONT_ROBOT_CLEARANCE,
        60.0,
    ) + keepout_rect(
        "rear_service_keepout",
        (0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0),
        STATION_X - 160.0,
        REAR_SERVICE_CLEARANCE,
        60.0,
    ) + keepout_rect(
        "left_bag_load_keepout",
        (-STATION_X / 2.0 - LEFT_BAG_LOAD_CLEARANCE / 2.0, 0.0),
        LEFT_BAG_LOAD_CLEARANCE,
        STATION_Y - 100.0,
        60.0,
    ) + keepout_rect(
        "right_scale_service_keepout",
        (STATION_X / 2.0 + RIGHT_SCALE_SERVICE_CLEARANCE / 2.0, 0.0),
        RIGHT_SCALE_SERVICE_CLEARANCE,
        STATION_Y - 100.0,
        60.0,
    ) + keepout_rect(
        "top_bag_lift_keepout",
        WEIGHING_POS,
        WEIGHING_ENVELOPE_X + 130.0,
        WEIGHING_ENVELOPE_Y + 80.0,
        TOP_BAG_LIFT_CLEARANCE,
    ) + keepout_rect(
        "top_thermal_shield_lift_keepout",
        WEIGHING_POS,
        THERMAL_SHIELD_X + 90.0,
        THERMAL_SHIELD_Y + 90.0,
        TOP_SHIELD_LIFT_CLEARANCE,
    ) + vertical_keepout_posts()
}

fn keepout_rect(name_suffix: &str, center: (f64, f64), x: f64, y: f64, z: f64) -> Part {
    let left = centered_cube(
        name(&format!("{name_suffix}_left_rail")),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(center.0 - x / 2.0, center.1, z);
    let right = centered_cube(
        name(&format!("{name_suffix}_right_rail")),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(center.0 + x / 2.0, center.1, z);
    let front = centered_cube(
        name(&format!("{name_suffix}_front_rail")),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(center.0, center.1 - y / 2.0, z);
    let rear = centered_cube(
        name(&format!("{name_suffix}_rear_rail")),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(center.0, center.1 + y / 2.0, z);

    left + right + front + rear
}

fn vertical_keepout_posts() -> Part {
    let mut posts = Part::empty(name("vertical_keepout_posts"));
    let x = WEIGHING_ENVELOPE_X + 130.0;
    let y = WEIGHING_ENVELOPE_Y + 80.0;
    for (i, (dx, dy)) in [
        (-x / 2.0, -y / 2.0),
        (x / 2.0, -y / 2.0),
        (-x / 2.0, y / 2.0),
        (x / 2.0, y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                name(&format!("bag_lift_keepout_vertical_post_{i}")),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                TOP_BAG_LIFT_CLEARANCE,
            )
            .translate(
                WEIGHING_POS.0 + *dx,
                WEIGHING_POS.1 + *dy,
                TOP_BAG_LIFT_CLEARANCE / 2.0,
            );
    }
    posts
}

fn label_land(suffix: &str, x: f64, y: f64, ticks: usize) -> Part {
    let plate = centered_cube(name(suffix), x, y, 5.0);
    let mut tick_marks = Part::empty(name(&format!("{suffix}_tick_marks")));
    for i in 0..ticks {
        tick_marks = tick_marks
            + centered_cube(name(&format!("{suffix}_tick_{i}")), 4.0, y - 6.0, 6.0).translate(
                centered_index(i, ticks, x / (ticks as f64 + 1.0)),
                0.0,
                0.5,
            );
    }
    plate + tick_marks
}

fn fiducial_disc(suffix: &str) -> Part {
    centered_cylinder(name(&format!("{suffix}_outer_ring")), 15.0, 4.0, 32)
        - centered_cylinder(name(&format!("{suffix}_inner_dot")), 5.5, 6.0, 24)
}

fn frame_corner_positions() -> [(f64, f64); 4] {
    [
        (
            -HANGING_FRAME_X / 2.0 + FRAME_POST_W / 2.0,
            -HANGING_FRAME_Y / 2.0 + FRAME_POST_W / 2.0,
        ),
        (
            HANGING_FRAME_X / 2.0 - FRAME_POST_W / 2.0,
            -HANGING_FRAME_Y / 2.0 + FRAME_POST_W / 2.0,
        ),
        (
            -HANGING_FRAME_X / 2.0 + FRAME_POST_W / 2.0,
            HANGING_FRAME_Y / 2.0 - FRAME_POST_W / 2.0,
        ),
        (
            HANGING_FRAME_X / 2.0 - FRAME_POST_W / 2.0,
            HANGING_FRAME_Y / 2.0 - FRAME_POST_W / 2.0,
        ),
    ]
}

fn module_rects() -> [Rect; 8] {
    [
        Rect {
            name: "hanging_and_tray_bag_modes",
            center: WEIGHING_POS,
            x: WEIGHING_ENVELOPE_X,
            y: WEIGHING_ENVELOPE_Y,
        },
        Rect {
            name: "load_cell_reference_mass_pocket",
            center: LOAD_REF_POS,
            x: LOAD_REF_X,
            y: LOAD_REF_Y,
        },
        Rect {
            name: "tubing_strain_relief_comb",
            center: COMB_POS,
            x: COMB_X,
            y: COMB_Y,
        },
        Rect {
            name: "bubble_foam_witness_window",
            center: BUBBLE_POS,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Rect {
            name: "calibration_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Rect {
            name: "drain_prime_capture_trough",
            center: TROUGH_POS,
            x: TROUGH_X,
            y: TROUGH_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: LANES_POS,
            x: LANES_X,
            y: LANES_Y,
        },
        Rect {
            name: "barcode_custody_panel",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for feature in [
        "hanging_bag_mode",
        "tray_bag_mode",
        "load_cell_reference_mass_pocket",
        "tubing_strain_relief_comb",
        "bubble_foam_witness_window",
        "thermal_shield",
        "calibration_token_rail",
        "drain_prime_capture_trough",
        "release_hold_reject_lanes",
        "barcode_custody_panel",
        "robot_service_keepouts",
        "gravimetric_false_mass_challenge",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "mechanical_validation_fixture_only",
        "no_sterile_processing_protocol",
        "no_load_cell_electronics_design",
        "no_pump_control_algorithm",
        "no_release_acceptance_limits",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(LOAD_CELL_PADS, 4);
    assert_eq!(TUBE_CHANNEL_D, MEDIA_TUBE_OD_MAX + TUBE_CLEARANCE);
    assert_eq!(CLAMP_FLAG_COUNT, TUBE_CHANNELS);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANES);
    assert_eq!(
        RELEASE_TOKEN_SLOTS + HOLD_TOKEN_SLOTS + REJECT_TOKEN_SLOTS,
        15
    );
    assert!(REFERENCE_MASS_WELLS > CERTIFICATE_SLOTS);
    assert!(FINE_MASS_WELLS >= CERTIFICATE_SLOTS * 2);
    assert!(FALSE_MASS_COUPONS >= FOAM_HEADSPACE_LEVELS);
    assert_eq!(CAMERA_FIDUCIALS, 4);
    assert!(SHIELD_AIR_GAP >= 35.0);
    assert!(THERMAL_SHIELD_X > HANGING_FRAME_X + 2.0 * SHIELD_AIR_GAP);
    assert!(THERMAL_SHIELD_Y > WEIGHING_ENVELOPE_Y + 40.0);
    assert!(TOP_SHIELD_LIFT_CLEARANCE > TOP_BAG_LIFT_CLEARANCE);

    let modules = module_rects();
    for module in modules {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station envelope",
            module.name
        );
    }

    for (i, a) in modules.iter().enumerate() {
        for b in modules.iter().skip(i + 1) {
            assert!(
                !a.overlaps_with_clearance(*b, 12.0),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_unique_and_station_scoped() {
        assert_design_constraints();

        let mut sorted = OUTPUTS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), OUTPUTS.len());
    }

    #[test]
    fn requested_feature_contract_is_explicit() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature))
                    || feature == "gravimetric_false_mass_challenge",
                "missing output cue for {feature}"
            );
        }
    }

    #[test]
    fn layout_keeps_modules_inside_containment_without_collisions() {
        for module in module_rects() {
            assert!(
                module.fits_inside_station(),
                "{} outside station",
                module.name
            );
        }
        assert_design_constraints();
    }

    #[test]
    fn weighing_modes_have_distinct_mechanical_cues() {
        assert!(HANGER_COUNT >= 5);
        assert!(TRAY_SADDLE_RIBS >= 8);
        assert!(TRAY_MODE_ID_FLAGS >= 4);
        assert!(HANGING_BAG_CLEAR_Z > TRAY_Z * 5.0);
    }

    #[test]
    fn false_mass_challenge_has_bubble_foam_and_reference_controls() {
        assert!(WITNESS_CHANNELS >= 5);
        assert!(BUBBLE_GRADUATION_TICKS >= 10);
        assert!(FOAM_HEADSPACE_LEVELS >= 4);
        assert!(FALSE_MASS_COUPONS >= 6);
        assert!(REFERENCE_MASS_WELLS >= 8);
        assert!(FINE_MASS_WELLS >= 6);
    }

    #[test]
    fn custody_and_disposition_are_parametric() {
        assert_eq!(
            DispositionLane::all(),
            [
                DispositionLane::Release,
                DispositionLane::Hold,
                DispositionLane::Reject,
            ]
        );
        assert_eq!(BARCODE_LANDS, 6);
        assert_eq!(BAG_ID_CARD_SLOTS, SCALE_RUN_CARD_SLOTS);
        assert_eq!(CAL_TOKEN_LANES * CAL_TOKENS_PER_LANE, 15);
    }
}
