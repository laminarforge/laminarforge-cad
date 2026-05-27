use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed airlock transfer-cart cassette orientation error validation station.
//
// Intent:
// - Validate that a large clean-enclosure/airlock transfer cart can only dock
//   cassette trays into the intended lane and orientation.
// - Exercise backward, upside-down, and wrong-lane challenge coupons against a
//   keyed cassette nest with asymmetric datum pins, hard stops, red/green
//   witness paddles, orientation tags, and latch-force witness tabs.
// - Keep barcode readers, force gauges, labels, acceptance criteria, and any
//   sterile-barrier claims outside this mechanical CAD generator.

const OUTPUTS: [&str; 12] = [
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_base_validation_deck.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_cart_docking_rails.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_keyed_cassette_nest.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_asymmetrical_datum_pins.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_hard_stop_blocks.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_red_green_witness_paddles.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_barcode_orientation_tag_plate.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_latch_force_witness_tabs.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_removable_challenge_coupons.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_lane_identity_go_no_go_blocks.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_robot_service_keepouts.stl",
    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "keyed_cassette_nest",
    "asymmetrical_datum_pins",
    "red_green_witness_paddles",
    "barcode_orientation_tag_plate",
    "cart_docking_rails",
    "hard_stop_blocks",
    "latch_force_witness_tabs",
    "removable_challenge_coupons",
    "lane_identity_go_no_go_blocks",
    "robot_service_keepouts",
];

const LANE_COUNT: usize = 3;
const LATCH_TAB_COUNT: usize = LANE_COUNT * 2;
const DATUM_PINS_PER_LANE: usize = 3;
const TOTAL_DATUM_PIN_COUNT: usize = LANE_COUNT * DATUM_PINS_PER_LANE;
const CHALLENGE_COUPON_COUNT: usize = 6;
const WITNESS_PADDLE_PAIRS: usize = LANE_COUNT;
const BARCODE_LAND_COUNT: usize = LANE_COUNT * 2;
const ORIENTATION_TAG_COUNT: usize = LANE_COUNT;

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 34.0;
const WIPE_GUTTER_W: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CART_RAIL_X: f64 = 650.0;
const CART_RAIL_CENTER_Y: f64 = -5.0;
const CART_RAIL_LEN: f64 = 830.0;
const CART_RAIL_W: f64 = 58.0;
const CART_RAIL_Z: f64 = 42.0;
const CART_WHEEL_CHANNEL_W: f64 = 24.0;
const CART_ENTRY_FUNNEL_LEN: f64 = 118.0;
const CART_DOCK_PIN_COUNT: usize = 4;

const NEST_CENTER: (f64, f64) = (-260.0, 110.0);
const CASSETTE_X: f64 = REVC_CHIP_LENGTH + 44.0;
const CASSETTE_Y: f64 = 2.0 * REVC_CHIP_WIDTH + 72.0;
const CASSETTE_Z: f64 = REVC_TOTAL_HEIGHT + 18.0;
const LANE_PITCH_X: f64 = 210.0;
const NEST_X: f64 = 684.0;
const NEST_Y: f64 = 360.0;
const NEST_Z: f64 = 34.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 28.0;
const POCKET_DEPTH: f64 = 12.0;
const KEY_TOOTH_X: f64 = 24.0;
const KEY_TOOTH_Y: f64 = 36.0;
const KEY_TOOTH_Z: f64 = 18.0;

const DATUM_PRIMARY_D: f64 = 14.0;
const DATUM_SECONDARY_D: f64 = 9.0;
const DATUM_TERTIARY_D: f64 = 6.0;
const DATUM_PIN_Z: f64 = 32.0;
const DATUM_BOSS_Z: f64 = 10.0;

const HARD_STOP_X: f64 = 72.0;
const HARD_STOP_Y: f64 = 22.0;
const HARD_STOP_Z: f64 = 52.0;
const UPSIDE_DOWN_BRIDGE_Z: f64 = 88.0;
const UPSIDE_DOWN_BRIDGE_CLEARANCE: f64 = CASSETTE_Z + 6.0;

const WITNESS_CENTER: (f64, f64) = (-260.0, -142.0);
const WITNESS_PANEL_X: f64 = 620.0;
const WITNESS_PANEL_Y: f64 = 92.0;
const WITNESS_PANEL_Z: f64 = 12.0;
const PADDLE_W: f64 = 48.0;
const PADDLE_T: f64 = 5.0;
const PADDLE_H: f64 = 62.0;
const PADDLE_PAIR_PITCH_X: f64 = LANE_PITCH_X;

const TAG_CENTER: (f64, f64) = (-260.0, -340.0);
const TAG_PLATE_X: f64 = 620.0;
const TAG_PLATE_Y: f64 = 108.0;
const TAG_PLATE_Z: f64 = 10.0;
const TAG_LAND_X: f64 = 118.0;
const TAG_LAND_Y: f64 = 22.0;
const BARCODE_STRIP_COUNT: usize = 7;

const LATCH_CENTER: (f64, f64) = (385.0, -335.0);
const LATCH_PANEL_X: f64 = 360.0;
const LATCH_PANEL_Y: f64 = 124.0;
const LATCH_PANEL_Z: f64 = 16.0;
const LATCH_TAB_X: f64 = 34.0;
const LATCH_TAB_Y: f64 = 74.0;
const LATCH_TAB_Z: f64 = 4.0;
const LATCH_TAB_PITCH_X: f64 = 50.0;
const FORCE_STEP_COUNT: usize = 5;

const COUPON_CENTER: (f64, f64) = (380.0, -130.0);
const COUPON_TRAY_X: f64 = 360.0;
const COUPON_TRAY_Y: f64 = 190.0;
const COUPON_TRAY_Z: f64 = 14.0;
const COUPON_X: f64 = 96.0;
const COUPON_Y: f64 = 54.0;
const COUPON_Z: f64 = 10.0;
const COUPON_PITCH_X: f64 = 112.0;
const COUPON_PITCH_Y: f64 = 72.0;

const GO_NO_GO_CENTER: (f64, f64) = (330.0, 170.0);
const GO_NO_GO_PANEL_X: f64 = 330.0;
const GO_NO_GO_PANEL_Y: f64 = 260.0;
const GO_NO_GO_PANEL_Z: f64 = 16.0;
const GO_NO_GO_BLOCK_X: f64 = 84.0;
const GO_NO_GO_BLOCK_Y: f64 = 86.0;
const GO_NO_GO_BLOCK_Z: f64 = 38.0;
const GO_NO_GO_PITCH_X: f64 = 104.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 280.0;
const REAR_AIRLOCK_SERVICE_KEEP_OUT_Y: f64 = 210.0;
const CART_APPROACH_KEEP_OUT_X: f64 = 1380.0;
const CASSETTE_LIFT_CLEARANCE_Z: f64 = 260.0;
const KEEP_OUT_RAIL: f64 = 10.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Lane {
    Alpha,
    Beta,
    Gamma,
}

impl Lane {
    fn all() -> [Lane; LANE_COUNT] {
        [Lane::Alpha, Lane::Beta, Lane::Gamma]
    }

    fn index(self) -> usize {
        match self {
            Lane::Alpha => 0,
            Lane::Beta => 1,
            Lane::Gamma => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Lane::Alpha => "alpha",
            Lane::Beta => "beta",
            Lane::Gamma => "gamma",
        }
    }

    fn x(self) -> f64 {
        NEST_CENTER.0 + centered_index(self.index(), LANE_COUNT, LANE_PITCH_X)
    }

    fn key_offset(self) -> f64 {
        match self {
            Lane::Alpha => -34.0,
            Lane::Beta => 0.0,
            Lane::Gamma => 34.0,
        }
    }

    fn stop_depth_offset(self) -> f64 {
        match self {
            Lane::Alpha => -8.0,
            Lane::Beta => 0.0,
            Lane::Gamma => 8.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ChallengeCoupon {
    Nominal,
    Backward,
    UpsideDown,
    WrongLaneAlpha,
    WrongLaneBeta,
    WrongLaneGamma,
}

impl ChallengeCoupon {
    fn all() -> [ChallengeCoupon; CHALLENGE_COUPON_COUNT] {
        [
            ChallengeCoupon::Nominal,
            ChallengeCoupon::Backward,
            ChallengeCoupon::UpsideDown,
            ChallengeCoupon::WrongLaneAlpha,
            ChallengeCoupon::WrongLaneBeta,
            ChallengeCoupon::WrongLaneGamma,
        ]
    }

    fn index(self) -> usize {
        match self {
            ChallengeCoupon::Nominal => 0,
            ChallengeCoupon::Backward => 1,
            ChallengeCoupon::UpsideDown => 2,
            ChallengeCoupon::WrongLaneAlpha => 3,
            ChallengeCoupon::WrongLaneBeta => 4,
            ChallengeCoupon::WrongLaneGamma => 5,
        }
    }

    fn name(self) -> &'static str {
        match self {
            ChallengeCoupon::Nominal => "nominal_green_control",
            ChallengeCoupon::Backward => "backward_red_challenge",
            ChallengeCoupon::UpsideDown => "upside_down_red_challenge",
            ChallengeCoupon::WrongLaneAlpha => "wrong_lane_alpha_red_challenge",
            ChallengeCoupon::WrongLaneBeta => "wrong_lane_beta_red_challenge",
            ChallengeCoupon::WrongLaneGamma => "wrong_lane_gamma_red_challenge",
        }
    }

    fn key_offset(self) -> f64 {
        match self {
            ChallengeCoupon::Nominal => 0.0,
            ChallengeCoupon::Backward => 38.0,
            ChallengeCoupon::UpsideDown => -38.0,
            ChallengeCoupon::WrongLaneAlpha => -34.0,
            ChallengeCoupon::WrongLaneBeta => 0.0,
            ChallengeCoupon::WrongLaneGamma => 34.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - DECK_RIM_W - 14.0;
        let usable_y = DECK_Y / 2.0 - DECK_RIM_W - 14.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
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

    let deck = base_validation_deck();
    export(OUTPUTS[0], &deck);

    let docking_rails = cart_docking_rails();
    export(OUTPUTS[1], &docking_rails);

    let nest = keyed_cassette_nest();
    export(OUTPUTS[2], &nest);

    let datum_pins = asymmetrical_datum_pins();
    export(OUTPUTS[3], &datum_pins);

    let stops = hard_stop_blocks();
    export(OUTPUTS[4], &stops);

    let paddles = red_green_witness_paddles();
    export(OUTPUTS[5], &paddles);

    let tag_plate = barcode_orientation_tag_plate();
    export(OUTPUTS[6], &tag_plate);

    let latch_tabs = latch_force_witness_tabs();
    export(OUTPUTS[7], &latch_tabs);

    let coupons = removable_challenge_coupons();
    export(OUTPUTS[8], &coupons);

    let go_no_go = lane_identity_go_no_go_blocks();
    export(OUTPUTS[9], &go_no_go);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + docking_rails
        + nest
        + datum_pins
        + stops
        + paddles
        + tag_plate
        + latch_tabs
        + coupons
        + go_no_go
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed airlock transfer-cart cassette orientation error station:");
    println!(
        "  Deck and cart dock:          {DECK_X:.0}mm x {DECK_Y:.0}mm validation deck with two {:.0}mm docking rails.",
        CART_RAIL_LEN
    );
    println!(
        "  Cassette nest:               {LANE_COUNT} keyed lanes for {:.0}mm x {:.0}mm cassette trays, with lane-specific key offsets.",
        CASSETTE_X, CASSETTE_Y
    );
    println!(
        "  Orientation prevention:      {TOTAL_DATUM_PIN_COUNT} asymmetric datum pins, hard stops, upside-down bridge gauges, and {WITNESS_PADDLE_PAIRS} red/green witness paddle pairs."
    );
    println!(
        "  Traceability and challenge:  {BARCODE_LAND_COUNT} barcode lands, {ORIENTATION_TAG_COUNT} orientation tag lands, {LATCH_TAB_COUNT} latch-force witness tabs, {CHALLENGE_COUPON_COUNT} removable challenge coupons."
    );
    println!("  Labeled STL outputs:         {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_validation_deck() -> Part {
    let deck = cube_at(
        "closed_airlock_cart_orientation_base_validation_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
        0.0,
        0.0,
        0.0,
    );

    deck - deck_mount_holes() - deck_wipe_gutters() + deck_perimeter_lips() + station_zone_labels()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_airlock_cart_orientation_deck_mount_holes");
    for (index, (x, y)) in deck_mount_points().into_iter().enumerate() {
        holes = holes
            + cylinder_z_at(
                format!("closed_airlock_cart_orientation_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                x,
                y,
                -2.0,
                28,
            );
    }
    holes
}

fn deck_wipe_gutters() -> Part {
    let nest_front = cube_at(
        "closed_airlock_cart_orientation_nest_front_wipe_gutter",
        NEST_X + 72.0,
        WIPE_GUTTER_W,
        6.0,
        NEST_CENTER.0,
        NEST_CENTER.1 - NEST_Y / 2.0 - 24.0,
        DECK_Z - 3.0,
    );
    let tag_gutter = cube_at(
        "closed_airlock_cart_orientation_tag_plate_wipe_gutter",
        TAG_PLATE_X + 36.0,
        WIPE_GUTTER_W,
        6.0,
        TAG_CENTER.0,
        TAG_CENTER.1 + TAG_PLATE_Y / 2.0 + 18.0,
        DECK_Z - 3.0,
    );
    let right_service_sump = cube_at(
        "closed_airlock_cart_orientation_right_service_wipe_sump",
        18.0,
        DECK_Y - 210.0,
        7.0,
        DECK_X / 2.0 - 76.0,
        -18.0,
        DECK_Z - 3.5,
    );
    let front_drain = centered_cylinder(
        "closed_airlock_cart_orientation_front_drain_cross_bore",
        5.0,
        46.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(530.0, -DECK_Y / 2.0 + 20.0, DECK_Z - 7.0);

    nest_front + tag_gutter + right_service_sump + front_drain
}

fn deck_perimeter_lips() -> Part {
    let rear = cube_at(
        "closed_airlock_cart_orientation_rear_airlock_lip",
        DECK_X - 126.0,
        DECK_RIM_W,
        DECK_RIM_Z,
        0.0,
        DECK_Y / 2.0 - 32.0,
        DECK_Z,
    );
    let left = cube_at(
        "closed_airlock_cart_orientation_left_cart_guard_lip",
        DECK_RIM_W,
        DECK_Y - 148.0,
        DECK_RIM_Z,
        -DECK_X / 2.0 + 34.0,
        0.0,
        DECK_Z,
    );
    let right = cube_at(
        "closed_airlock_cart_orientation_right_service_lip",
        DECK_RIM_W,
        DECK_Y - 148.0,
        DECK_RIM_Z * 0.72,
        DECK_X / 2.0 - 34.0,
        0.0,
        DECK_Z,
    );
    let front_low = cube_at(
        "closed_airlock_cart_orientation_front_low_robot_access_lip",
        DECK_X - 320.0,
        10.0,
        16.0,
        -20.0,
        -DECK_Y / 2.0 + 34.0,
        DECK_Z,
    );

    rear + left + right + front_low
}

fn station_zone_labels() -> Part {
    let mut labels = Part::empty("closed_airlock_cart_orientation_station_zone_label_lands");
    for (index, rect) in layout_rects().into_iter().take(6).enumerate() {
        labels = labels
            + cube_at(
                format!("closed_airlock_cart_orientation_zone_{index}_label_land"),
                82.0,
                18.0,
                3.0,
                rect.center.0 - rect.x / 2.0 + 56.0,
                rect.center.1 + rect.y / 2.0 - 24.0,
                DECK_Z,
            );
    }
    labels
}

fn cart_docking_rails() -> Part {
    let mut rails = Part::empty("closed_airlock_cart_orientation_cart_docking_rails");

    for (side_index, side) in [-1.0, 1.0].into_iter().enumerate() {
        let x = side * CART_RAIL_X;
        let rail_body = cube_at(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_body"),
            CART_RAIL_W,
            CART_RAIL_LEN,
            CART_RAIL_Z,
            x,
            CART_RAIL_CENTER_Y,
            DECK_Z,
        );
        let wheel_groove = cube_at(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_wheel_channel"),
            CART_WHEEL_CHANNEL_W,
            CART_RAIL_LEN + 6.0,
            16.0,
            x,
            CART_RAIL_CENTER_Y,
            DECK_Z + CART_RAIL_Z - 14.0,
        );
        let rail = rail_body - wheel_groove;

        let inboard_fence = cube_at(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_inboard_fence"),
            8.0,
            CART_RAIL_LEN - 86.0,
            CART_RAIL_Z + 18.0,
            x - side * (CART_RAIL_W / 2.0 - 5.0),
            CART_RAIL_CENTER_Y + 16.0,
            DECK_Z,
        );
        let outboard_low_fence = cube_at(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_outboard_low_fence"),
            8.0,
            CART_RAIL_LEN - 164.0,
            CART_RAIL_Z * 0.62,
            x + side * (CART_RAIL_W / 2.0 - 5.0),
            CART_RAIL_CENTER_Y + 44.0,
            DECK_Z,
        );
        let funnel_left = centered_cube(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_entry_funnel_a"),
            CART_ENTRY_FUNNEL_LEN,
            14.0,
            28.0,
        )
        .rotate(0.0, 0.0, side * 14.0)
        .translate(
            x - side * 24.0,
            CART_RAIL_CENTER_Y - CART_RAIL_LEN / 2.0 + 34.0,
            DECK_Z + 14.0,
        );
        let funnel_right = centered_cube(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_entry_funnel_b"),
            CART_ENTRY_FUNNEL_LEN,
            14.0,
            28.0,
        )
        .rotate(0.0, 0.0, -side * 14.0)
        .translate(
            x + side * 24.0,
            CART_RAIL_CENTER_Y - CART_RAIL_LEN / 2.0 + 34.0,
            DECK_Z + 14.0,
        );
        let rear_stop = cube_at(
            format!("closed_airlock_cart_orientation_cart_rail_{side_index}_rear_positive_stop"),
            CART_RAIL_W + 22.0,
            22.0,
            CART_RAIL_Z + 28.0,
            x,
            CART_RAIL_CENTER_Y + CART_RAIL_LEN / 2.0 - 28.0,
            DECK_Z,
        );

        rails = rails
            + rail
            + inboard_fence
            + outboard_low_fence
            + funnel_left
            + funnel_right
            + rear_stop;
    }

    rails + cart_dock_pin_sockets()
}

fn cart_dock_pin_sockets() -> Part {
    let mut sockets = Part::empty("closed_airlock_cart_orientation_cart_dock_pin_sockets");
    for (index, (x, y)) in [
        (-CART_RAIL_X, CART_RAIL_CENTER_Y + 252.0),
        (CART_RAIL_X, CART_RAIL_CENTER_Y + 252.0),
        (-CART_RAIL_X, CART_RAIL_CENTER_Y - 248.0),
        (CART_RAIL_X, CART_RAIL_CENTER_Y - 248.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = cylinder_z_at(
            format!("closed_airlock_cart_orientation_docking_socket_boss_{index}"),
            18.0,
            12.0,
            x,
            y,
            DECK_Z,
            40,
        );
        let socket = cylinder_z_at(
            format!("closed_airlock_cart_orientation_docking_socket_recess_{index}"),
            8.0,
            14.0,
            x,
            y,
            DECK_Z + 2.0,
            32,
        );
        sockets = sockets + (boss - socket);
    }
    sockets
}

fn keyed_cassette_nest() -> Part {
    let body = cube_at(
        "closed_airlock_cart_orientation_keyed_three_lane_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z,
    );

    let nest_with_pockets = body - cassette_lane_pocket_cuts() - nest_drain_slots();
    nest_with_pockets + cassette_lane_rails() + lane_key_teeth() + lane_separator_posts()
}

fn cassette_lane_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_airlock_cart_orientation_cassette_lane_pocket_cuts");
    for lane in Lane::all() {
        cuts =
            cuts + cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_cassette_pocket_relief",
                    lane.name()
                ),
                CASSETTE_X,
                CASSETTE_Y,
                POCKET_DEPTH + 1.0,
                lane.x(),
                NEST_CENTER.1,
                DECK_Z + NEST_Z - POCKET_DEPTH,
            ) + cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_front_barcode_sight_notch",
                    lane.name()
                ),
                CASSETTE_X * 0.52,
                16.0,
                POCKET_DEPTH + 3.0,
                lane.x(),
                NEST_CENTER.1 - CASSETTE_Y / 2.0 + 13.0,
                DECK_Z + NEST_Z - POCKET_DEPTH,
            );
    }
    cuts
}

fn nest_drain_slots() -> Part {
    let mut slots = Part::empty("closed_airlock_cart_orientation_nest_cleanout_drain_slots");
    for lane in Lane::all() {
        slots = slots
            + cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_lane_drain_slot",
                    lane.name()
                ),
                12.0,
                CASSETTE_Y + 46.0,
                8.0,
                lane.x() + CASSETTE_X / 2.0 + 18.0,
                NEST_CENTER.1,
                DECK_Z + NEST_Z - 5.0,
            );
    }
    slots
}

fn cassette_lane_rails() -> Part {
    let mut rails = Part::empty("closed_airlock_cart_orientation_cassette_lane_datum_rails");
    for lane in Lane::all() {
        let x = lane.x();
        let rear = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_rear_y_datum_rail",
                lane.name()
            ),
            CASSETTE_X + 34.0,
            NEST_RAIL_W,
            NEST_RAIL_Z,
            x,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 + NEST_RAIL_W / 2.0 + 4.0,
            DECK_Z + NEST_Z,
        );
        let left = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_left_x_datum_rail",
                lane.name()
            ),
            NEST_RAIL_W,
            CASSETTE_Y + 26.0,
            NEST_RAIL_Z,
            x - CASSETTE_X / 2.0 - NEST_RAIL_W / 2.0 - 4.0,
            NEST_CENTER.1,
            DECK_Z + NEST_Z,
        );
        let right_soft = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_right_soft_capture_rail",
                lane.name()
            ),
            NEST_RAIL_W,
            CASSETTE_Y * 0.68,
            NEST_RAIL_Z * 0.62,
            x + CASSETTE_X / 2.0 + NEST_RAIL_W / 2.0 + 4.0,
            NEST_CENTER.1 - 18.0,
            DECK_Z + NEST_Z,
        );
        let front_low = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_front_low_load_lip",
                lane.name()
            ),
            CASSETTE_X - 54.0,
            9.0,
            14.0,
            x + 10.0,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 - 8.0,
            DECK_Z + NEST_Z,
        );
        rails = rails + rear + left + right_soft + front_low;
    }
    rails
}

fn lane_key_teeth() -> Part {
    let mut keys = Part::empty("closed_airlock_cart_orientation_lane_specific_key_teeth");
    for lane in Lane::all() {
        let x = lane.x();
        let key_offset = lane.key_offset();
        let leading_tooth = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_leading_orientation_key_tooth",
                lane.name()
            ),
            KEY_TOOTH_X,
            KEY_TOOTH_Y,
            KEY_TOOTH_Z,
            x + key_offset,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 + 30.0,
            DECK_Z + NEST_Z,
        );
        let diagonal_witness_tooth = centered_cube(
            format!(
                "closed_airlock_cart_orientation_{}_diagonal_nonmirror_key_tooth",
                lane.name()
            ),
            KEY_TOOTH_X * 1.35,
            KEY_TOOTH_Y * 0.45,
            KEY_TOOTH_Z,
        )
        .rotate(0.0, 0.0, 18.0)
        .translate(
            x - key_offset * 0.55,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 - 40.0,
            DECK_Z + NEST_Z + KEY_TOOTH_Z / 2.0,
        );
        keys = keys + leading_tooth + diagonal_witness_tooth;
    }
    keys
}

fn lane_separator_posts() -> Part {
    let mut posts = Part::empty("closed_airlock_cart_orientation_lane_separator_posts");
    for index in 0..(LANE_COUNT - 1) {
        let x = NEST_CENTER.0 - LANE_PITCH_X / 2.0 + index as f64 * LANE_PITCH_X;
        for (post_index, y) in [
            NEST_CENTER.1 - NEST_Y / 2.0 + 46.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 46.0,
        ]
        .into_iter()
        .enumerate()
        {
            posts = posts
                + cylinder_z_at(
                    format!("closed_airlock_cart_orientation_lane_separator_{index}_{post_index}"),
                    9.0,
                    NEST_RAIL_Z,
                    x,
                    y,
                    DECK_Z + NEST_Z,
                    32,
                );
        }
    }
    posts
}

fn asymmetrical_datum_pins() -> Part {
    let mut pins = Part::empty("closed_airlock_cart_orientation_asymmetrical_datum_pins");
    for lane in Lane::all() {
        let x = lane.x();
        for (pin_index, (name, dx, dy, diameter, height)) in [
            (
                "primary_large_round",
                -CASSETTE_X / 2.0 + 30.0,
                CASSETTE_Y / 2.0 - 34.0,
                DATUM_PRIMARY_D,
                DATUM_PIN_Z,
            ),
            (
                "secondary_small_round",
                CASSETTE_X / 2.0 - 42.0,
                CASSETTE_Y / 2.0 - 58.0,
                DATUM_SECONDARY_D,
                DATUM_PIN_Z - 8.0,
            ),
            (
                "tertiary_upside_down_no_go",
                -lane.key_offset() * 0.5,
                -CASSETTE_Y / 2.0 + 48.0,
                DATUM_TERTIARY_D,
                DATUM_PIN_Z + 10.0,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let px = x + dx;
            let py = NEST_CENTER.1 + dy;
            let boss = cylinder_z_at(
                format!(
                    "closed_airlock_cart_orientation_{}_datum_{}_boss",
                    lane.name(),
                    pin_index
                ),
                diameter / 2.0 + 6.0,
                DATUM_BOSS_Z,
                px,
                py,
                DECK_Z + NEST_Z,
                36,
            );
            let pin = cylinder_z_at(
                format!(
                    "closed_airlock_cart_orientation_{}_datum_{name}_pin",
                    lane.name()
                ),
                diameter / 2.0,
                height,
                px,
                py,
                DECK_Z + NEST_Z + DATUM_BOSS_Z,
                40,
            );
            pins = pins + boss + pin;
        }
    }
    pins
}

fn hard_stop_blocks() -> Part {
    let mut stops = Part::empty("closed_airlock_cart_orientation_hard_stop_blocks");
    for lane in Lane::all() {
        let x = lane.x();
        let stop_depth = lane.stop_depth_offset();
        let rear_stop = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_rear_hard_stop",
                lane.name()
            ),
            HARD_STOP_X,
            HARD_STOP_Y,
            HARD_STOP_Z,
            x + stop_depth,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 + 35.0,
            DECK_Z + NEST_Z,
        );
        let front_backwards_stop = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_front_backward_no_go_stop",
                lane.name()
            ),
            HARD_STOP_X * 0.72,
            HARD_STOP_Y,
            HARD_STOP_Z * 0.72,
            x - stop_depth,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 - 28.0,
            DECK_Z + NEST_Z,
        );
        let upside_down_bridge = upside_down_no_go_bridge(lane);
        let datum_face_wear_strip = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_replaceable_stop_wear_strip",
                lane.name()
            ),
            HARD_STOP_X - 18.0,
            5.0,
            12.0,
            x + stop_depth,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 + 21.0,
            DECK_Z + NEST_Z + HARD_STOP_Z - 12.0,
        );
        stops =
            stops + rear_stop + front_backwards_stop + upside_down_bridge + datum_face_wear_strip;
    }
    stops
}

fn upside_down_no_go_bridge(lane: Lane) -> Part {
    let x = lane.x();
    let y = NEST_CENTER.1 - CASSETTE_Y / 2.0 + 74.0;
    let left_post = cube_at(
        format!(
            "closed_airlock_cart_orientation_{}_upside_down_bridge_left_post",
            lane.name()
        ),
        12.0,
        12.0,
        UPSIDE_DOWN_BRIDGE_Z,
        x - CASSETTE_X / 2.0 + 32.0,
        y,
        DECK_Z + NEST_Z,
    );
    let right_post = cube_at(
        format!(
            "closed_airlock_cart_orientation_{}_upside_down_bridge_right_post",
            lane.name()
        ),
        12.0,
        12.0,
        UPSIDE_DOWN_BRIDGE_Z,
        x + CASSETTE_X / 2.0 - 32.0,
        y,
        DECK_Z + NEST_Z,
    );
    let crossbar = cube_at(
        format!(
            "closed_airlock_cart_orientation_{}_upside_down_no_go_crossbar",
            lane.name()
        ),
        CASSETTE_X - 52.0,
        12.0,
        10.0,
        x,
        y,
        DECK_Z + NEST_Z + UPSIDE_DOWN_BRIDGE_CLEARANCE,
    );
    left_post + right_post + crossbar
}

fn red_green_witness_paddles() -> Part {
    let panel = cube_at(
        "closed_airlock_cart_orientation_witness_paddle_hinge_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        DECK_Z,
    );
    let mut paddles = Part::empty("closed_airlock_cart_orientation_red_green_witness_paddles");

    for lane in Lane::all() {
        let x = WITNESS_CENTER.0 + centered_index(lane.index(), LANE_COUNT, PADDLE_PAIR_PITCH_X);
        let green = witness_paddle(
            format!(
                "closed_airlock_cart_orientation_{}_green_accept_paddle",
                lane.name()
            ),
            x - 28.0,
            WITNESS_CENTER.1 - 10.0,
            true,
        );
        let red = witness_paddle(
            format!(
                "closed_airlock_cart_orientation_{}_red_reject_paddle",
                lane.name()
            ),
            x + 28.0,
            WITNESS_CENTER.1 + 18.0,
            false,
        );
        let lane_hinge = centered_cylinder(
            format!(
                "closed_airlock_cart_orientation_{}_paddle_hinge_rod",
                lane.name()
            ),
            5.0,
            78.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, WITNESS_CENTER.1 + 4.0, DECK_Z + WITNESS_PANEL_Z + 10.0);
        paddles = paddles + green + red + lane_hinge;
    }

    panel + paddles + witness_paddle_legend_lands()
}

fn witness_paddle(name: String, x: f64, y: f64, green: bool) -> Part {
    let height = if green { PADDLE_H * 0.82 } else { PADDLE_H };
    let face = cube_at(
        format!("{name}_face"),
        PADDLE_W,
        PADDLE_T,
        height,
        x,
        y,
        DECK_Z + WITNESS_PANEL_Z + 5.0,
    );
    let hinge_boss = centered_cylinder(format!("{name}_hinge_boss"), 6.0, PADDLE_W + 8.0, 28)
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, DECK_Z + WITNESS_PANEL_Z + 8.0);
    let witness_dot = cylinder_z_at(
        format!("{name}_round_witness_dot"),
        if green { 4.0 } else { 5.5 },
        3.0,
        x,
        y - 4.0,
        DECK_Z + WITNESS_PANEL_Z + height + 6.0,
        24,
    );
    face + hinge_boss + witness_dot
}

fn witness_paddle_legend_lands() -> Part {
    let mut lands = Part::empty("closed_airlock_cart_orientation_witness_paddle_legend_lands");
    for lane in Lane::all() {
        let x = WITNESS_CENTER.0 + centered_index(lane.index(), LANE_COUNT, PADDLE_PAIR_PITCH_X);
        lands = lands
            + cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_green_red_legend_land",
                    lane.name()
                ),
                112.0,
                16.0,
                3.0,
                x,
                WITNESS_CENTER.1 - WITNESS_PANEL_Y / 2.0 + 18.0,
                DECK_Z + WITNESS_PANEL_Z,
            );
    }
    lands
}

fn barcode_orientation_tag_plate() -> Part {
    let plate = cube_at(
        "closed_airlock_cart_orientation_barcode_orientation_tag_plate",
        TAG_PLATE_X,
        TAG_PLATE_Y,
        TAG_PLATE_Z,
        TAG_CENTER.0,
        TAG_CENTER.1,
        DECK_Z,
    );

    plate + barcode_lands() + orientation_tag_lands() + orientation_arrow() + tag_plate_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_airlock_cart_orientation_barcode_lands");
    for lane in Lane::all() {
        let base_x = TAG_CENTER.0 + centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X);
        for row in 0..2 {
            let y = TAG_CENTER.1 - 28.0 + row as f64 * 36.0;
            let land = cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_barcode_land_{row}",
                    lane.name()
                ),
                TAG_LAND_X,
                TAG_LAND_Y,
                3.0,
                base_x,
                y,
                DECK_Z + TAG_PLATE_Z,
            );
            let mut bars = Part::empty(format!(
                "closed_airlock_cart_orientation_{}_barcode_stripes_{row}",
                lane.name()
            ));
            for strip in 0..BARCODE_STRIP_COUNT {
                let width = if strip % 2 == 0 { 3.0 } else { 6.0 };
                bars = bars
                    + cube_at(
                        format!(
                            "closed_airlock_cart_orientation_{}_barcode_{row}_stripe_{strip}",
                            lane.name()
                        ),
                        width,
                        TAG_LAND_Y + 3.0,
                        2.5,
                        base_x - TAG_LAND_X / 2.0 + 14.0 + strip as f64 * 12.0,
                        y,
                        DECK_Z + TAG_PLATE_Z + 3.0,
                    );
            }
            lands = lands + land + bars;
        }
    }
    lands
}

fn orientation_tag_lands() -> Part {
    let mut tags = Part::empty("closed_airlock_cart_orientation_lane_orientation_tag_lands");
    for lane in Lane::all() {
        let x = TAG_CENTER.0 + centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X);
        let tag = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_orientation_tag_recess_land",
                lane.name()
            ),
            86.0,
            22.0,
            4.0,
            x,
            TAG_CENTER.1 + TAG_PLATE_Y / 2.0 - 22.0,
            DECK_Z + TAG_PLATE_Z,
        );
        let notch = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_orientation_tag_asymmetric_notch",
                lane.name()
            ),
            16.0,
            8.0,
            6.0,
            x + lane.key_offset() * 0.35,
            TAG_CENTER.1 + TAG_PLATE_Y / 2.0 - 22.0,
            DECK_Z + TAG_PLATE_Z + 3.0,
        );
        tags = tags + tag + notch;
    }
    tags
}

fn orientation_arrow() -> Part {
    let shaft = cube_at(
        "closed_airlock_cart_orientation_forward_arrow_shaft",
        168.0,
        8.0,
        4.0,
        TAG_CENTER.0 - 12.0,
        TAG_CENTER.1,
        DECK_Z + TAG_PLATE_Z,
    );
    let head_a = centered_cube(
        "closed_airlock_cart_orientation_forward_arrow_head_a",
        48.0,
        8.0,
        4.0,
    )
    .rotate(0.0, 0.0, 35.0)
    .translate(
        TAG_CENTER.0 + 92.0,
        TAG_CENTER.1 + 12.0,
        DECK_Z + TAG_PLATE_Z + 2.0,
    );
    let head_b = centered_cube(
        "closed_airlock_cart_orientation_forward_arrow_head_b",
        48.0,
        8.0,
        4.0,
    )
    .rotate(0.0, 0.0, -35.0)
    .translate(
        TAG_CENTER.0 + 92.0,
        TAG_CENTER.1 - 12.0,
        DECK_Z + TAG_PLATE_Z + 2.0,
    );
    shaft + head_a + head_b
}

fn tag_plate_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_airlock_cart_orientation_tag_plate_fiducials");
    for (index, (x, y)) in [
        (
            TAG_CENTER.0 - TAG_PLATE_X / 2.0 + 28.0,
            TAG_CENTER.1 - TAG_PLATE_Y / 2.0 + 22.0,
        ),
        (
            TAG_CENTER.0 + TAG_PLATE_X / 2.0 - 28.0,
            TAG_CENTER.1 - TAG_PLATE_Y / 2.0 + 22.0,
        ),
        (
            TAG_CENTER.0 - TAG_PLATE_X / 2.0 + 28.0,
            TAG_CENTER.1 + TAG_PLATE_Y / 2.0 - 22.0,
        ),
        (
            TAG_CENTER.0 + TAG_PLATE_X / 2.0 - 28.0,
            TAG_CENTER.1 + TAG_PLATE_Y / 2.0 - 22.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let disc = cylinder_z_at(
            format!("closed_airlock_cart_orientation_tag_plate_fiducial_disc_{index}"),
            7.0,
            3.0,
            x,
            y,
            DECK_Z + TAG_PLATE_Z,
            32,
        );
        let hole = cylinder_z_at(
            format!("closed_airlock_cart_orientation_tag_plate_fiducial_center_{index}"),
            1.6,
            4.0,
            x,
            y,
            DECK_Z + TAG_PLATE_Z,
            20,
        );
        fiducials = fiducials + (disc - hole);
    }
    fiducials
}

fn latch_force_witness_tabs() -> Part {
    let panel = cube_at(
        "closed_airlock_cart_orientation_latch_force_witness_tab_panel",
        LATCH_PANEL_X,
        LATCH_PANEL_Y,
        LATCH_PANEL_Z,
        LATCH_CENTER.0,
        LATCH_CENTER.1,
        DECK_Z,
    );

    let mut tabs = Part::empty("closed_airlock_cart_orientation_latch_force_witness_tabs");
    for tab_index in 0..LATCH_TAB_COUNT {
        let x = LATCH_CENTER.0 + centered_index(tab_index, LATCH_TAB_COUNT, LATCH_TAB_PITCH_X);
        let side_name = if tab_index % 2 == 0 { "left" } else { "right" };
        let lane = Lane::all()[tab_index / 2];
        let tab = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_{}_latch_force_leaf_tab",
                lane.name(),
                side_name
            ),
            LATCH_TAB_X,
            LATCH_TAB_Y,
            LATCH_TAB_Z,
            x,
            LATCH_CENTER.1 + 4.0,
            DECK_Z + LATCH_PANEL_Z,
        );
        let root = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_{}_latch_tab_fixed_root",
                lane.name(),
                side_name
            ),
            LATCH_TAB_X + 10.0,
            12.0,
            16.0,
            x,
            LATCH_CENTER.1 - LATCH_PANEL_Y / 2.0 + 22.0,
            DECK_Z + LATCH_PANEL_Z,
        );
        let witness_tip = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_{}_latch_witness_tip",
                lane.name(),
                side_name
            ),
            LATCH_TAB_X * 0.65,
            8.0,
            12.0,
            x,
            LATCH_CENTER.1 + LATCH_PANEL_Y / 2.0 - 22.0,
            DECK_Z + LATCH_PANEL_Z,
        );
        tabs = tabs + tab + root + witness_tip;
    }

    panel + tabs + latch_force_step_gauge()
}

fn latch_force_step_gauge() -> Part {
    let mut steps = Part::empty("closed_airlock_cart_orientation_latch_force_step_gauge");
    for step in 0..FORCE_STEP_COUNT {
        let height = 4.0 + step as f64 * 2.0;
        steps = steps
            + cube_at(
                format!("closed_airlock_cart_orientation_latch_force_step_{step}"),
                26.0,
                12.0,
                height,
                LATCH_CENTER.0 - LATCH_PANEL_X / 2.0 + 30.0 + step as f64 * 32.0,
                LATCH_CENTER.1 - LATCH_PANEL_Y / 2.0 + 20.0,
                DECK_Z + LATCH_PANEL_Z,
            );
    }
    steps
}

fn removable_challenge_coupons() -> Part {
    let tray = cube_at(
        "closed_airlock_cart_orientation_removable_challenge_coupon_tray",
        COUPON_TRAY_X,
        COUPON_TRAY_Y,
        COUPON_TRAY_Z,
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        DECK_Z,
    );
    let mut recesses = Part::empty("closed_airlock_cart_orientation_challenge_coupon_recesses");
    let mut coupons = Part::empty("closed_airlock_cart_orientation_challenge_coupons");

    for coupon in ChallengeCoupon::all() {
        let (x, y) = coupon_xy(coupon);
        recesses = recesses
            + cube_at(
                format!(
                    "closed_airlock_cart_orientation_{}_coupon_shadow_recess",
                    coupon.name()
                ),
                COUPON_X + 8.0,
                COUPON_Y + 8.0,
                5.0,
                x,
                y,
                DECK_Z + COUPON_TRAY_Z - 4.5,
            );
        coupons = coupons + challenge_coupon(coupon, x, y);
    }

    (tray - recesses) + coupons + coupon_retainer_rail()
}

fn challenge_coupon(coupon: ChallengeCoupon, x: f64, y: f64) -> Part {
    let body = cube_at(
        format!(
            "closed_airlock_cart_orientation_{}_coupon_body",
            coupon.name()
        ),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
        x,
        y,
        DECK_Z + COUPON_TRAY_Z,
    );
    let grip = cylinder_z_at(
        format!(
            "closed_airlock_cart_orientation_{}_coupon_finger_pull",
            coupon.name()
        ),
        9.0,
        COUPON_Z + 2.0,
        x - COUPON_X / 2.0 + 18.0,
        y,
        DECK_Z + COUPON_TRAY_Z - 1.0,
        28,
    );
    let key_tooth = cube_at(
        format!(
            "closed_airlock_cart_orientation_{}_coupon_key_tooth",
            coupon.name()
        ),
        16.0,
        20.0,
        12.0,
        x + coupon.key_offset() * 0.42,
        y + COUPON_Y / 2.0 - 12.0,
        DECK_Z + COUPON_TRAY_Z + COUPON_Z,
    );
    let orientation_boss = cylinder_z_at(
        format!(
            "closed_airlock_cart_orientation_{}_coupon_orientation_boss",
            coupon.name()
        ),
        if matches!(coupon, ChallengeCoupon::UpsideDown) {
            6.5
        } else {
            4.5
        },
        8.0,
        x + COUPON_X / 2.0 - 22.0,
        y - COUPON_Y / 2.0 + 16.0,
        DECK_Z + COUPON_TRAY_Z + COUPON_Z,
        28,
    );

    (body - grip) + key_tooth + orientation_boss
}

fn coupon_retainer_rail() -> Part {
    let front = cube_at(
        "closed_airlock_cart_orientation_challenge_coupon_front_retainer_rail",
        COUPON_TRAY_X - 38.0,
        8.0,
        18.0,
        COUPON_CENTER.0,
        COUPON_CENTER.1 - COUPON_TRAY_Y / 2.0 + 12.0,
        DECK_Z + COUPON_TRAY_Z,
    );
    let rear = cube_at(
        "closed_airlock_cart_orientation_challenge_coupon_rear_retainer_rail",
        COUPON_TRAY_X - 38.0,
        8.0,
        18.0,
        COUPON_CENTER.0,
        COUPON_CENTER.1 + COUPON_TRAY_Y / 2.0 - 12.0,
        DECK_Z + COUPON_TRAY_Z,
    );
    front + rear
}

fn lane_identity_go_no_go_blocks() -> Part {
    let panel = cube_at(
        "closed_airlock_cart_orientation_lane_identity_go_no_go_panel",
        GO_NO_GO_PANEL_X,
        GO_NO_GO_PANEL_Y,
        GO_NO_GO_PANEL_Z,
        GO_NO_GO_CENTER.0,
        GO_NO_GO_CENTER.1,
        DECK_Z,
    );

    let mut blocks = Part::empty("closed_airlock_cart_orientation_lane_identity_go_no_go_blocks");
    for lane in Lane::all() {
        let x = GO_NO_GO_CENTER.0 + centered_index(lane.index(), LANE_COUNT, GO_NO_GO_PITCH_X);
        let y = GO_NO_GO_CENTER.1 + 32.0;
        let block_body = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_go_no_go_block_body",
                lane.name()
            ),
            GO_NO_GO_BLOCK_X,
            GO_NO_GO_BLOCK_Y,
            GO_NO_GO_BLOCK_Z,
            x,
            y,
            DECK_Z + GO_NO_GO_PANEL_Z,
        );
        let go_slot = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_green_go_slot",
                lane.name()
            ),
            18.0,
            GO_NO_GO_BLOCK_Y + 2.0,
            18.0,
            x + lane.key_offset() * 0.35,
            y,
            DECK_Z + GO_NO_GO_PANEL_Z + GO_NO_GO_BLOCK_Z - 16.0,
        );
        let no_go_pin = cylinder_z_at(
            format!(
                "closed_airlock_cart_orientation_{}_red_no_go_pin",
                lane.name()
            ),
            7.0,
            26.0,
            x - lane.key_offset() * 0.45,
            y - GO_NO_GO_BLOCK_Y / 2.0 + 20.0,
            DECK_Z + GO_NO_GO_PANEL_Z + GO_NO_GO_BLOCK_Z,
            32,
        );
        let label_land = cube_at(
            format!(
                "closed_airlock_cart_orientation_{}_go_no_go_label_land",
                lane.name()
            ),
            72.0,
            16.0,
            4.0,
            x,
            GO_NO_GO_CENTER.1 - GO_NO_GO_PANEL_Y / 2.0 + 22.0,
            DECK_Z + GO_NO_GO_PANEL_Z,
        );
        blocks = blocks + (block_body - go_slot) + no_go_pin + label_land;
    }

    panel + blocks + go_no_go_cross_lane_reject_bar()
}

fn go_no_go_cross_lane_reject_bar() -> Part {
    cube_at(
        "closed_airlock_cart_orientation_cross_lane_red_reject_bar",
        GO_NO_GO_PANEL_X - 48.0,
        12.0,
        18.0,
        GO_NO_GO_CENTER.0,
        GO_NO_GO_CENTER.1 - GO_NO_GO_PANEL_Y / 2.0 + 54.0,
        DECK_Z + GO_NO_GO_PANEL_Z,
    )
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        "closed_airlock_cart_orientation_front_robot_cart_approach_keepout",
        CART_APPROACH_KEEP_OUT_X,
        FRONT_ROBOT_KEEP_OUT_Y,
        CASSETTE_LIFT_CLEARANCE_Z,
        (0.0, -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 22.0),
    );
    let rear_service = keepout_box(
        "closed_airlock_cart_orientation_rear_airlock_service_keepout",
        CART_APPROACH_KEEP_OUT_X - 160.0,
        REAR_AIRLOCK_SERVICE_KEEP_OUT_Y,
        CASSETTE_LIFT_CLEARANCE_Z * 0.72,
        (
            0.0,
            DECK_Y / 2.0 - REAR_AIRLOCK_SERVICE_KEEP_OUT_Y / 2.0 - 26.0,
        ),
    );
    let nest_lift = keepout_box(
        "closed_airlock_cart_orientation_cassette_vertical_lift_keepout",
        NEST_X + 84.0,
        NEST_Y + 72.0,
        CASSETTE_LIFT_CLEARANCE_Z,
        NEST_CENTER,
    );

    front_robot + rear_service + nest_lift
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64, center: (f64, f64)) -> Part {
    let bottom = rectangular_frame_at(
        format!("{name}_bottom_frame"),
        x,
        y,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
        center,
        DECK_Z,
    );
    let top = rectangular_frame_at(
        format!("{name}_top_frame"),
        x,
        y,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
        center,
        DECK_Z + z - KEEP_OUT_RAIL,
    );
    let mut posts = Part::empty(format!("{name}_posts"));
    for (index, (xsign, ysign)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        posts = posts
            + cube_at(
                format!("{name}_vertical_post_{index}"),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                z,
                center.0 + xsign * (x / 2.0 - KEEP_OUT_RAIL / 2.0),
                center.1 + ysign * (y / 2.0 - KEEP_OUT_RAIL / 2.0),
                DECK_Z,
            );
    }
    bottom + top + posts
}

fn rectangular_frame_at(
    name: String,
    x: f64,
    y: f64,
    rail_w: f64,
    rail_z: f64,
    center: (f64, f64),
    bottom_z: f64,
) -> Part {
    let rear = cube_at(
        format!("{name}_rear_rail"),
        x,
        rail_w,
        rail_z,
        center.0,
        center.1 + y / 2.0 - rail_w / 2.0,
        bottom_z,
    );
    let front = cube_at(
        format!("{name}_front_rail"),
        x,
        rail_w,
        rail_z,
        center.0,
        center.1 - y / 2.0 + rail_w / 2.0,
        bottom_z,
    );
    let left = cube_at(
        format!("{name}_left_rail"),
        rail_w,
        y,
        rail_z,
        center.0 - x / 2.0 + rail_w / 2.0,
        center.1,
        bottom_z,
    );
    let right = cube_at(
        format!("{name}_right_rail"),
        rail_w,
        y,
        rail_z,
        center.0 + x / 2.0 - rail_w / 2.0,
        center.1,
        bottom_z,
    );
    rear + front + left + right
}

fn cube_at<N: Into<String>>(
    name: N,
    x: f64,
    y: f64,
    z: f64,
    cx: f64,
    cy: f64,
    bottom_z: f64,
) -> Part {
    centered_cube(name, x, y, z).translate(cx, cy, bottom_z + z / 2.0)
}

fn cylinder_z_at<N: Into<String>>(
    name: N,
    radius: f64,
    height: f64,
    cx: f64,
    cy: f64,
    bottom_z: f64,
    segments: u32,
) -> Part {
    centered_cylinder(name, radius, height, segments).translate(cx, cy, bottom_z + height / 2.0)
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
        (-CART_RAIL_X, CART_RAIL_CENTER_Y - 156.0),
        (CART_RAIL_X, CART_RAIL_CENTER_Y - 156.0),
        (-CART_RAIL_X, CART_RAIL_CENTER_Y + 156.0),
        (CART_RAIL_X, CART_RAIL_CENTER_Y + 156.0),
    ]
}

fn coupon_xy(coupon: ChallengeCoupon) -> (f64, f64) {
    let index = coupon.index();
    let col = index % 3;
    let row = index / 3;
    (
        COUPON_CENTER.0 + centered_index(col, 3, COUPON_PITCH_X),
        COUPON_CENTER.1 + centered_index(row, 2, COUPON_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn layout_rects() -> [Rect; 8] {
    [
        Rect {
            name: "keyed_cassette_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "witness_paddles",
            center: WITNESS_CENTER,
            x: WITNESS_PANEL_X,
            y: WITNESS_PANEL_Y,
        },
        Rect {
            name: "barcode_orientation_tag_plate",
            center: TAG_CENTER,
            x: TAG_PLATE_X,
            y: TAG_PLATE_Y,
        },
        Rect {
            name: "lane_identity_go_no_go_blocks",
            center: GO_NO_GO_CENTER,
            x: GO_NO_GO_PANEL_X,
            y: GO_NO_GO_PANEL_Y,
        },
        Rect {
            name: "removable_challenge_coupons",
            center: COUPON_CENTER,
            x: COUPON_TRAY_X,
            y: COUPON_TRAY_Y,
        },
        Rect {
            name: "latch_force_witness_tabs",
            center: LATCH_CENTER,
            x: LATCH_PANEL_X,
            y: LATCH_PANEL_Y,
        },
        Rect {
            name: "left_cart_docking_rail",
            center: (-CART_RAIL_X, CART_RAIL_CENTER_Y),
            x: CART_RAIL_W + 34.0,
            y: CART_RAIL_LEN,
        },
        Rect {
            name: "right_cart_docking_rail",
            center: (CART_RAIL_X, CART_RAIL_CENTER_Y),
            x: CART_RAIL_W + 34.0,
            y: CART_RAIL_LEN,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "output manifest changed");
    assert_eq!(REQUIRED_FEATURES.len(), 10, "required feature list changed");
    assert_eq!(Lane::all().len(), LANE_COUNT);
    assert_eq!(ChallengeCoupon::all().len(), CHALLENGE_COUPON_COUNT);
    assert_eq!(LATCH_TAB_COUNT, LANE_COUNT * 2);
    assert_eq!(TOTAL_DATUM_PIN_COUNT, LANE_COUNT * DATUM_PINS_PER_LANE);
    assert_eq!(CART_DOCK_PIN_COUNT, 4);

    assert!(CASSETTE_X > REVC_CHIP_LENGTH + 40.0);
    assert!(CASSETTE_Y > 2.0 * REVC_CHIP_WIDTH + 68.0);
    assert!(CASSETTE_Z > REVC_TOTAL_HEIGHT);
    assert!(NEST_X > LANE_PITCH_X * (LANE_COUNT as f64 - 1.0) + CASSETTE_X + 80.0);
    assert!(NEST_Y > CASSETTE_Y + 100.0);
    assert!(UPSIDE_DOWN_BRIDGE_CLEARANCE > CASSETTE_Z);
    assert!(DATUM_PRIMARY_D > DATUM_SECONDARY_D);
    assert!(DATUM_SECONDARY_D > DATUM_TERTIARY_D);

    for rect in layout_rects() {
        assert!(
            rect.fits_inside_deck(),
            "{} exceeds usable deck area",
            rect.name
        );
    }

    let rects = layout_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert!(FRONT_ROBOT_KEEP_OUT_Y >= 260.0);
    assert!(REAR_AIRLOCK_SERVICE_KEEP_OUT_Y >= 200.0);
    assert!(CASSETTE_LIFT_CLEARANCE_Z > UPSIDE_DOWN_BRIDGE_CLEARANCE);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_airlock_transfer_cart_cassette_orientation_error_station_"
                ),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "keyed_cassette_nest",
            "asymmetrical_datum_pins",
            "red_green_witness_paddles",
            "barcode_orientation_tag_plate",
            "cart_docking_rails",
            "hard_stop_blocks",
            "latch_force_witness_tabs",
            "removable_challenge_coupons",
            "lane_identity_go_no_go_blocks",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn lane_geometry_is_asymmetric_and_keyed() {
        assert_eq!(LANE_COUNT, 3);
        assert_eq!(Lane::Alpha.key_offset(), -34.0);
        assert_eq!(Lane::Beta.key_offset(), 0.0);
        assert_eq!(Lane::Gamma.key_offset(), 34.0);
        assert_ne!(
            Lane::Alpha.stop_depth_offset(),
            Lane::Gamma.stop_depth_offset()
        );
        assert!(DATUM_PRIMARY_D > DATUM_SECONDARY_D);
        assert!(DATUM_SECONDARY_D > DATUM_TERTIARY_D);
    }

    #[test]
    fn challenge_coupon_set_covers_nominal_and_error_modes() {
        assert_eq!(CHALLENGE_COUPON_COUNT, 6);
        let names: BTreeSet<&str> = ChallengeCoupon::all()
            .into_iter()
            .map(ChallengeCoupon::name)
            .collect();
        for expected in [
            "nominal_green_control",
            "backward_red_challenge",
            "upside_down_red_challenge",
            "wrong_lane_alpha_red_challenge",
            "wrong_lane_beta_red_challenge",
            "wrong_lane_gamma_red_challenge",
        ] {
            assert!(names.contains(expected));
        }
    }

    #[test]
    fn station_regions_are_bounded_and_non_overlapping() {
        assert_design_constraints();
    }

    #[test]
    fn witness_traceability_and_latch_counts_match_lanes() {
        assert_eq!(WITNESS_PADDLE_PAIRS, LANE_COUNT);
        assert_eq!(BARCODE_LAND_COUNT, LANE_COUNT * 2);
        assert_eq!(ORIENTATION_TAG_COUNT, LANE_COUNT);
        assert_eq!(LATCH_TAB_COUNT, LANE_COUNT * 2);
        assert_eq!(FORCE_STEP_COUNT, 5);
    }

    #[test]
    fn cart_and_robot_clearances_are_kept_explicit() {
        assert_eq!(CART_DOCK_PIN_COUNT, 4);
        assert!(CART_RAIL_X + CART_RAIL_W / 2.0 < DECK_X / 2.0 - DECK_RIM_W);
        assert!(CART_RAIL_LEN < DECK_Y - 100.0);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 260.0);
        assert!(CASSETTE_LIFT_CLEARANCE_Z > UPSIDE_DOWN_BRIDGE_CLEARANCE);
    }
}
