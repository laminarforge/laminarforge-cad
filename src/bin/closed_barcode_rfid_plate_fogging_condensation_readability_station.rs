use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed barcode/RFID fogging and condensation readability validation station.
//
// This standalone generator models a no-cell validation module for challenging
// cassette identity capture after transfer from a humid incubator into cooler
// robot or imaging zones. It provides physical witnesses for fogging, droplet
// formation, barcode contrast loss, RFID coupling distance, purge/dry-air dwell,
// readhead focus, disposition routing, custody transfer, and robot/service
// clearances. It does not encode acceptance limits, reader firmware, or sample
// handling logic.

const BIN_PREFIX: &str = "closed_barcode_rfid_plate_fogging_condensation_readability_station";
const OUTPUT_PREFIX: &str =
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_containment_transfer_deck.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_humid_cool_cassette_transfer_dock.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_fogging_coupon_plate.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_condensation_droplet_witness_grid.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_barcode_contrast_ladder.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_rfid_pocket_array.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_purge_dry_air_token_rail.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_camera_readhead_focus_bridge.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_reject_hold_release_gates.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_custody_transfer_lands.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_robot_service_keepout_gauges.stl",
    "output/closed_barcode_rfid_plate_fogging_condensation_readability_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "containment_transfer_deck",
    "humid_cool_cassette_transfer_dock",
    "fogging_coupon_plate",
    "condensation_droplet_witness_grid",
    "barcode_contrast_ladder",
    "rfid_pocket_array",
    "purge_dry_air_token_rail",
    "camera_readhead_focus_bridge",
    "reject_hold_release_gates",
    "custody_transfer_lands",
    "robot_service_keepout_gauges",
];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 5.5;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MOUNT_HOLE_COUNT: usize = 8;

const DOCK_POS: (f64, f64) = (-470.0, 145.0);
const DOCK_X: f64 = 410.0;
const DOCK_Y: f64 = 270.0;
const DOCK_Z: f64 = 44.0;
const CASSETTE_NEST_X: f64 = 168.0;
const CASSETTE_NEST_Y: f64 = 154.0;
const CASSETTE_NEST_DEPTH: f64 = 18.0;
const CASSETTE_NEST_COUNT: usize = 2;
const DOCK_DATUM_PIN_COUNT: usize = 8;

const FOG_POS: (f64, f64) = (-5.0, 270.0);
const FOG_PLATE_X: f64 = 430.0;
const FOG_PLATE_Y: f64 = 170.0;
const FOG_PLATE_Z: f64 = 22.0;
const FOG_COUPON_ROWS: usize = 2;
const FOG_COUPON_COLS: usize = 4;
const FOG_COUPON_COUNT: usize = FOG_COUPON_ROWS * FOG_COUPON_COLS;
const FOG_COUPON_X: f64 = 76.0;
const FOG_COUPON_Y: f64 = 42.0;
const FOG_COUPON_PITCH_X: f64 = 92.0;
const FOG_COUPON_PITCH_Y: f64 = 66.0;
const FOG_NOZZLE_COUNT: usize = 6;

const DROPLET_POS: (f64, f64) = (430.0, 190.0);
const DROPLET_GRID_X: f64 = 360.0;
const DROPLET_GRID_Y: f64 = 250.0;
const DROPLET_GRID_Z: f64 = 22.0;
const DROPLET_ROWS: usize = 4;
const DROPLET_COLS: usize = 6;
const DROPLET_WELL_COUNT: usize = DROPLET_ROWS * DROPLET_COLS;
const DROPLET_WELL_D: f64 = 20.0;
const DROPLET_WELL_DEPTH: f64 = 12.0;
const DROPLET_PITCH_X: f64 = 46.0;
const DROPLET_PITCH_Y: f64 = 44.0;

const BARCODE_POS: (f64, f64) = (-470.0, -205.0);
const BARCODE_PANEL_X: f64 = 420.0;
const BARCODE_PANEL_Y: f64 = 170.0;
const BARCODE_PANEL_Z: f64 = 18.0;
const BARCODE_STEP_COUNT: usize = 9;
const BARCODE_STEP_X: f64 = 32.0;
const BARCODE_STEP_Y: f64 = 116.0;
const BARCODE_STEP_PITCH_X: f64 = 40.0;
const CONTRAST_MIN_Z: f64 = 2.2;
const CONTRAST_DELTA_Z: f64 = 1.1;
const BARCODE_SMEAR_STRIPES: usize = 7;

const RFID_POS: (f64, f64) = (-40.0, -225.0);
const RFID_PANEL_X: f64 = 360.0;
const RFID_PANEL_Y: f64 = 180.0;
const RFID_PANEL_Z: f64 = 30.0;
const RFID_ROWS: usize = 2;
const RFID_COLS: usize = 4;
const RFID_POCKET_COUNT: usize = RFID_ROWS * RFID_COLS;
const RFID_POCKET_X: f64 = 58.0;
const RFID_POCKET_Y: f64 = 42.0;
const RFID_POCKET_DEPTH: f64 = 16.0;
const RFID_PITCH_X: f64 = 78.0;
const RFID_PITCH_Y: f64 = 70.0;

const PURGE_POS: (f64, f64) = (405.0, -250.0);
const PURGE_RAIL_X: f64 = 370.0;
const PURGE_RAIL_Y: f64 = 120.0;
const PURGE_RAIL_Z: f64 = 26.0;
const PURGE_TOKEN_COUNT: usize = 6;
const PURGE_TOKEN_D: f64 = 34.0;
const PURGE_TOKEN_DEPTH: f64 = 14.0;
const PURGE_TOKEN_PITCH_X: f64 = 48.0;
const PURGE_NOZZLE_COUNT: usize = 6;

const BRIDGE_POS: (f64, f64) = (0.0, 30.0);
const BRIDGE_SPAN_X: f64 = 1320.0;
const BRIDGE_POST_X: f64 = 36.0;
const BRIDGE_POST_Y: f64 = 54.0;
const BRIDGE_UNDERSIDE_Z: f64 = 215.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_POD_COUNT: usize = 3;
const READHEAD_POD_COUNT: usize = 2;
const CAMERA_POD_X: f64 = 82.0;
const CAMERA_POD_Y: f64 = 70.0;
const CAMERA_POD_Z: f64 = 46.0;

const GATE_POS: (f64, f64) = (485.0, -30.0);
const GATE_PANEL_X: f64 = 340.0;
const GATE_PANEL_Y: f64 = 160.0;
const GATE_PANEL_Z: f64 = 24.0;
const DISPOSITION_LANES: usize = 3;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SLOT_X: f64 = 58.0;
const GATE_SLOT_Y: f64 = 24.0;
const GATE_LANE_PITCH_Y: f64 = 48.0;

const CUSTODY_POS: (f64, f64) = (0.0, 25.0);
const CUSTODY_PANEL_X: f64 = 420.0;
const CUSTODY_PANEL_Y: f64 = 100.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const CUSTODY_LAND_COUNT: usize = 4;
const CUSTODY_LAND_X: f64 = 76.0;
const CUSTODY_LAND_Y: f64 = 42.0;
const CUSTODY_TOKEN_COUNT: usize = 4;

const KEEP_OUT_X: f64 = 1400.0;
const KEEP_OUT_Y: f64 = 850.0;
const ROBOT_FRONT_CLEARANCE: f64 = 330.0;
const SERVICE_REAR_CLEARANCE: f64 = 245.0;
const SIDE_SERVICE_CLEARANCE: f64 = 230.0;
const ROBOT_Z_CLEARANCE: f64 = 315.0;
const KEEP_OUT_GAUGE_COUNT: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    #[cfg(test)]
    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Disposition {
    Release,
    Hold,
    Reject,
}

impl Disposition {
    fn all() -> [Disposition; DISPOSITION_LANES] {
        [Disposition::Release, Disposition::Hold, Disposition::Reject]
    }

    fn index(self) -> usize {
        match self {
            Disposition::Release => 0,
            Disposition::Hold => 1,
            Disposition::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Disposition::Release => "release",
            Disposition::Hold => "hold",
            Disposition::Reject => "reject",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_transfer_deck();
    export(OUTPUTS[0], &deck);

    let dock = humid_cool_cassette_transfer_dock();
    export(OUTPUTS[1], &dock);

    let fogging = fogging_coupon_plate();
    export(OUTPUTS[2], &fogging);

    let droplets = condensation_droplet_witness_grid();
    export(OUTPUTS[3], &droplets);

    let barcode = barcode_contrast_ladder();
    export(OUTPUTS[4], &barcode);

    let rfid = rfid_pocket_array();
    export(OUTPUTS[5], &rfid);

    let purge = purge_dry_air_token_rail();
    export(OUTPUTS[6], &purge);

    let bridge = camera_readhead_focus_bridge();
    export(OUTPUTS[7], &bridge);

    let gates = reject_hold_release_gates();
    export(OUTPUTS[8], &gates);

    let custody = custody_transfer_lands();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + dock
        + fogging
        + droplets
        + barcode
        + rfid
        + purge
        + bridge
        + gates
        + custody
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed barcode/RFID fogging-condensation readability station:");
    println!(
        "  Footprint:                 {DECK_X:.0}mm x {DECK_Y:.0}mm containment transfer deck"
    );
    println!(
        "  Humid/cool transfer:       {CASSETTE_NEST_COUNT} cassette nests, {DOCK_DATUM_PIN_COUNT} datum pins, fog-drip containment basin"
    );
    println!(
        "  Readability challenges:    {FOG_COUPON_COUNT} fogging coupons, {DROPLET_WELL_COUNT} droplet witness wells, {BARCODE_STEP_COUNT} barcode contrast steps, {RFID_POCKET_COUNT} RFID pockets"
    );
    println!(
        "  Recovery workflow:         {PURGE_TOKEN_COUNT} purge/dry-air tokens, {DISPOSITION_LANES} release/hold/reject gates, {CUSTODY_LAND_COUNT} custody transfer lands"
    );
    println!(
        "  Readhead bridge:           {CAMERA_POD_COUNT} camera pods, {READHEAD_POD_COUNT} RFID readheads, {:.1}mm clearance over barcode ladder",
        bridge_clearance_over_barcode()
    );
    println!("  Output prefix:             {OUTPUT_PREFIX}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_transfer_deck() -> Part {
    let deck = centered_cube(format!("{BIN_PREFIX}_base_deck"), DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let basin = centered_cube(
        format!("{BIN_PREFIX}_shallow_condensate_containment_basin"),
        DECK_X - 122.0,
        DECK_Y - 126.0,
        BASIN_DEPTH,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 1.0);
    let gutter = rectangular_outline(
        format!("{BIN_PREFIX}_wipeable_perimeter_condensate_gutter"),
        DECK_X - 84.0,
        DECK_Y - 88.0,
        14.0,
        5.0,
    )
    .translate(0.0, 0.0, DECK_Z + 2.5);

    deck - basin - mount_holes() - component_socket_recesses() + deck_rim() + gutter
}

fn deck_rim() -> Part {
    let left = centered_cube(
        format!("{BIN_PREFIX}_left_liquid_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{BIN_PREFIX}_right_liquid_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{BIN_PREFIX}_rear_liquid_containment_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front_left = centered_cube(
        format!("{BIN_PREFIX}_front_left_robot_loading_lip"),
        380.0,
        RIM_W,
        22.0,
    )
    .translate(
        -DECK_X / 2.0 + 190.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 11.0,
    );
    let front_right = centered_cube(
        format!("{BIN_PREFIX}_front_right_robot_loading_lip"),
        380.0,
        RIM_W,
        22.0,
    )
    .translate(
        DECK_X / 2.0 - 190.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 11.0,
    );

    left + right + rear + front_left + front_right
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{BIN_PREFIX}_deck_mount_holes"));
    for index in 0..MOUNT_HOLE_COUNT {
        let x = if index % 2 == 0 {
            -DECK_X / 2.0 + 72.0
        } else {
            DECK_X / 2.0 - 72.0
        };
        let y = lane_offset(index / 2, MOUNT_HOLE_COUNT / 2, 220.0);
        holes = holes
            + centered_cylinder(
                format!("{BIN_PREFIX}_mount_hole_{index}"),
                MOUNT_HOLE_D,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn component_socket_recesses() -> Part {
    let sockets = surface_rects();
    let mut cuts = Part::empty(format!("{BIN_PREFIX}_component_socket_recesses"));
    for (index, rect) in sockets.iter().enumerate() {
        cuts = cuts
            + centered_cube(
                format!("{BIN_PREFIX}_component_socket_{index}_{}", rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                SOCKET_DEPTH,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.8,
            );
    }
    cuts
}

fn humid_cool_cassette_transfer_dock() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_humid_cool_transfer_dock_base"),
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, insert_z(DOCK_Z));
    let mut pocket_cuts = Part::empty(format!("{BIN_PREFIX}_humid_cool_cassette_pocket_cuts"));
    let mut labels = Part::empty(format!("{BIN_PREFIX}_humid_cool_transfer_labels"));
    for index in 0..CASSETTE_NEST_COUNT {
        let x = DOCK_POS.0 + lane_offset(index, CASSETTE_NEST_COUNT, 190.0);
        pocket_cuts = pocket_cuts
            + centered_cube(
                format!("{BIN_PREFIX}_cassette_transfer_nest_cut_{index}"),
                CASSETTE_NEST_X,
                CASSETTE_NEST_Y,
                CASSETTE_NEST_DEPTH + 2.0,
            )
            .translate(
                x,
                DOCK_POS.1 + 16.0,
                DECK_Z + DOCK_Z - CASSETTE_NEST_DEPTH / 2.0 + 1.0,
            );
        labels = labels
            + centered_cube(
                format!("{BIN_PREFIX}_transfer_state_label_land_{index}"),
                124.0,
                28.0,
                3.5,
            )
            .translate(x, DOCK_POS.1 - 110.0, DECK_Z + DOCK_Z + 1.75);
    }

    base - pocket_cuts + cassette_dock_rails() + cassette_datum_pins() + labels
}

fn cassette_dock_rails() -> Part {
    let mut rails = Part::empty(format!("{BIN_PREFIX}_cassette_transfer_guide_rails"));
    for index in 0..CASSETTE_NEST_COUNT {
        let x = DOCK_POS.0 + lane_offset(index, CASSETTE_NEST_COUNT, 190.0);
        let left = centered_cube(
            format!("{BIN_PREFIX}_cassette_nest_{index}_left_wipeable_rail"),
            12.0,
            CASSETTE_NEST_Y + 46.0,
            28.0,
        )
        .translate(
            x - CASSETTE_NEST_X / 2.0 - 15.0,
            DOCK_POS.1 + 16.0,
            DECK_Z + DOCK_Z + 14.0,
        );
        let right = centered_cube(
            format!("{BIN_PREFIX}_cassette_nest_{index}_right_wipeable_rail"),
            12.0,
            CASSETTE_NEST_Y + 46.0,
            28.0,
        )
        .translate(
            x + CASSETTE_NEST_X / 2.0 + 15.0,
            DOCK_POS.1 + 16.0,
            DECK_Z + DOCK_Z + 14.0,
        );
        let rear = centered_cube(
            format!("{BIN_PREFIX}_cassette_nest_{index}_rear_hard_stop"),
            CASSETTE_NEST_X + 52.0,
            12.0,
            28.0,
        )
        .translate(
            x,
            DOCK_POS.1 + 16.0 + CASSETTE_NEST_Y / 2.0 + 15.0,
            DECK_Z + DOCK_Z + 14.0,
        );
        rails = rails + left + right + rear;
    }
    rails
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{BIN_PREFIX}_cassette_transfer_datum_pins"));
    for index in 0..DOCK_DATUM_PIN_COUNT {
        let nest = index / 4;
        let pin = index % 4;
        let x = DOCK_POS.0
            + lane_offset(nest, CASSETTE_NEST_COUNT, 190.0)
            + if pin % 2 == 0 { -54.0 } else { 54.0 };
        let y = DOCK_POS.1 + 16.0 + if pin < 2 { -56.0 } else { 56.0 };
        pins = pins
            + centered_cylinder(
                format!("{BIN_PREFIX}_dock_datum_pin_{index}"),
                5.2,
                17.0,
                28,
            )
            .translate(x, y, DECK_Z + DOCK_Z + 8.5);
    }
    pins
}

fn fogging_coupon_plate() -> Part {
    let plate = centered_cube(
        format!("{BIN_PREFIX}_fogging_coupon_plate_base"),
        FOG_PLATE_X,
        FOG_PLATE_Y,
        FOG_PLATE_Z,
    )
    .translate(FOG_POS.0, FOG_POS.1, insert_z(FOG_PLATE_Z));
    let mut coupon_lands = Part::empty(format!("{BIN_PREFIX}_replaceable_fogging_coupon_lands"));
    for row in 0..FOG_COUPON_ROWS {
        for col in 0..FOG_COUPON_COLS {
            let index = row * FOG_COUPON_COLS + col;
            let x = FOG_POS.0 + lane_offset(col, FOG_COUPON_COLS, FOG_COUPON_PITCH_X);
            let y = FOG_POS.1 + lane_offset(row, FOG_COUPON_ROWS, FOG_COUPON_PITCH_Y);
            coupon_lands = coupon_lands
                + centered_cube(
                    format!("{BIN_PREFIX}_fogging_coupon_land_{index}"),
                    FOG_COUPON_X,
                    FOG_COUPON_Y,
                    5.0,
                )
                .translate(x, y, DECK_Z + FOG_PLATE_Z + 2.5)
                + rectangular_outline(
                    format!("{BIN_PREFIX}_fogging_coupon_retainer_frame_{index}"),
                    FOG_COUPON_X + 12.0,
                    FOG_COUPON_Y + 10.0,
                    3.0,
                    5.0,
                )
                .translate(x, y, DECK_Z + FOG_PLATE_Z + 5.0);
        }
    }

    plate + coupon_lands + fog_nozzle_bar() + humidity_film_lip()
}

fn fog_nozzle_bar() -> Part {
    let header = centered_cube(
        format!("{BIN_PREFIX}_fog_mist_header_bar"),
        FOG_PLATE_X - 34.0,
        18.0,
        18.0,
    )
    .translate(
        FOG_POS.0,
        FOG_POS.1 + FOG_PLATE_Y / 2.0 - 22.0,
        DECK_Z + FOG_PLATE_Z + 9.0,
    );
    let mut nozzles = Part::empty(format!("{BIN_PREFIX}_fog_nozzle_witness_ports"));
    for index in 0..FOG_NOZZLE_COUNT {
        let x = FOG_POS.0 + lane_offset(index, FOG_NOZZLE_COUNT, 58.0);
        nozzles = nozzles
            + centered_cylinder(
                format!("{BIN_PREFIX}_fog_nozzle_port_{index}"),
                7.0,
                12.0,
                24,
            )
            .translate(
                x,
                FOG_POS.1 + FOG_PLATE_Y / 2.0 - 22.0,
                DECK_Z + FOG_PLATE_Z + 21.0,
            );
    }
    header + nozzles
}

fn humidity_film_lip() -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_humidity_film_reference_lip"),
        FOG_PLATE_X - 42.0,
        10.0,
        12.0,
    )
    .translate(
        FOG_POS.0,
        FOG_POS.1 - FOG_PLATE_Y / 2.0 + 22.0,
        DECK_Z + FOG_PLATE_Z + 6.0,
    )
}

fn condensation_droplet_witness_grid() -> Part {
    let plate = centered_cube(
        format!("{BIN_PREFIX}_condensation_droplet_witness_grid_plate"),
        DROPLET_GRID_X,
        DROPLET_GRID_Y,
        DROPLET_GRID_Z,
    )
    .translate(DROPLET_POS.0, DROPLET_POS.1, insert_z(DROPLET_GRID_Z));
    let mut wells = Part::empty(format!("{BIN_PREFIX}_droplet_witness_well_cuts"));
    let mut dot_markers = Part::empty(format!("{BIN_PREFIX}_droplet_size_reference_dots"));
    for row in 0..DROPLET_ROWS {
        for col in 0..DROPLET_COLS {
            let index = row * DROPLET_COLS + col;
            let x = DROPLET_POS.0 + lane_offset(col, DROPLET_COLS, DROPLET_PITCH_X);
            let y = DROPLET_POS.1 + lane_offset(row, DROPLET_ROWS, DROPLET_PITCH_Y);
            wells = wells
                + centered_cylinder(
                    format!("{BIN_PREFIX}_droplet_witness_well_cut_{index}"),
                    DROPLET_WELL_D,
                    DROPLET_WELL_DEPTH + 2.0,
                    32,
                )
                .translate(
                    x,
                    y,
                    DECK_Z + DROPLET_GRID_Z - DROPLET_WELL_DEPTH / 2.0 + 1.0,
                );
            dot_markers = dot_markers
                + centered_cylinder(
                    format!("{BIN_PREFIX}_droplet_size_boss_{index}"),
                    4.0 + (index % 4) as f64 * 1.4,
                    2.6,
                    20,
                )
                .translate(x + 14.0, y - 13.0, DECK_Z + DROPLET_GRID_Z + 1.3);
        }
    }

    plate - wells + grid_rulers() + dot_markers
}

fn grid_rulers() -> Part {
    let horizontal = rectangular_outline(
        format!("{BIN_PREFIX}_droplet_grid_index_frame"),
        DROPLET_GRID_X - 26.0,
        DROPLET_GRID_Y - 24.0,
        3.0,
        4.0,
    )
    .translate(DROPLET_POS.0, DROPLET_POS.1, DECK_Z + DROPLET_GRID_Z + 2.0);
    let mut tick_marks = Part::empty(format!("{BIN_PREFIX}_droplet_grid_tick_marks"));
    for col in 0..DROPLET_COLS {
        let x = DROPLET_POS.0 + lane_offset(col, DROPLET_COLS, DROPLET_PITCH_X);
        tick_marks = tick_marks
            + centered_cube(
                format!("{BIN_PREFIX}_droplet_col_tick_{col}"),
                4.0,
                14.0,
                4.0,
            )
            .translate(
                x,
                DROPLET_POS.1 - DROPLET_GRID_Y / 2.0 + 18.0,
                DECK_Z + DROPLET_GRID_Z + 2.0,
            );
    }
    horizontal + tick_marks
}

fn barcode_contrast_ladder() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_barcode_contrast_ladder_panel"),
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    )
    .translate(BARCODE_POS.0, BARCODE_POS.1, insert_z(BARCODE_PANEL_Z));
    let mut steps = Part::empty(format!("{BIN_PREFIX}_barcode_contrast_step_lands"));
    for index in 0..BARCODE_STEP_COUNT {
        let h = barcode_step_height(index);
        let x = BARCODE_POS.0 + lane_offset(index, BARCODE_STEP_COUNT, BARCODE_STEP_PITCH_X);
        steps = steps
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_contrast_step_{index}"),
                BARCODE_STEP_X,
                BARCODE_STEP_Y,
                h,
            )
            .translate(x, BARCODE_POS.1, DECK_Z + BARCODE_PANEL_Z + h / 2.0)
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_edge_finder_notch_{index}"),
                BARCODE_STEP_X - 10.0,
                6.0,
                2.0,
            )
            .translate(
                x,
                BARCODE_POS.1 - BARCODE_STEP_Y / 2.0 + 12.0,
                DECK_Z + BARCODE_PANEL_Z + h + 1.0,
            );
    }

    panel + steps + barcode_smear_stripes() + barcode_guard_fences()
}

fn barcode_smear_stripes() -> Part {
    let mut stripes = Part::empty(format!("{BIN_PREFIX}_barcode_condensate_smear_stripes"));
    for index in 0..BARCODE_SMEAR_STRIPES {
        let x = BARCODE_POS.0 + lane_offset(index, BARCODE_SMEAR_STRIPES, 50.0);
        let y = BARCODE_POS.1 + if index % 2 == 0 { 62.0 } else { -62.0 };
        stripes = stripes
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_smear_occlusion_stripe_{index}"),
                10.0,
                34.0,
                5.0,
            )
            .translate(x, y, DECK_Z + BARCODE_PANEL_Z + 2.5);
    }
    stripes
}

fn barcode_guard_fences() -> Part {
    let left = centered_cube(
        format!("{BIN_PREFIX}_barcode_ladder_left_light_shield"),
        8.0,
        BARCODE_PANEL_Y - 24.0,
        22.0,
    )
    .translate(
        BARCODE_POS.0 - BARCODE_PANEL_X / 2.0 + 18.0,
        BARCODE_POS.1,
        DECK_Z + BARCODE_PANEL_Z + 11.0,
    );
    let right = centered_cube(
        format!("{BIN_PREFIX}_barcode_ladder_right_light_shield"),
        8.0,
        BARCODE_PANEL_Y - 24.0,
        22.0,
    )
    .translate(
        BARCODE_POS.0 + BARCODE_PANEL_X / 2.0 - 18.0,
        BARCODE_POS.1,
        DECK_Z + BARCODE_PANEL_Z + 11.0,
    );
    left + right
}

fn rfid_pocket_array() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_rfid_pocket_array_panel"),
        RFID_PANEL_X,
        RFID_PANEL_Y,
        RFID_PANEL_Z,
    )
    .translate(RFID_POS.0, RFID_POS.1, insert_z(RFID_PANEL_Z));
    let mut pocket_cuts = Part::empty(format!("{BIN_PREFIX}_rfid_tag_pocket_cuts"));
    let mut coils = Part::empty(format!("{BIN_PREFIX}_rfid_readability_coil_outlines"));
    for row in 0..RFID_ROWS {
        for col in 0..RFID_COLS {
            let index = row * RFID_COLS + col;
            let x = RFID_POS.0 + lane_offset(col, RFID_COLS, RFID_PITCH_X);
            let y = RFID_POS.1 + lane_offset(row, RFID_ROWS, RFID_PITCH_Y);
            pocket_cuts = pocket_cuts
                + centered_cube(
                    format!("{BIN_PREFIX}_rfid_pocket_cut_{index}"),
                    RFID_POCKET_X,
                    RFID_POCKET_Y,
                    RFID_POCKET_DEPTH + 2.0,
                )
                .translate(
                    x,
                    y,
                    DECK_Z + RFID_PANEL_Z - RFID_POCKET_DEPTH / 2.0 + 1.0,
                );
            coils = coils
                + rectangular_outline(
                    format!("{BIN_PREFIX}_rfid_pocket_coil_outline_{index}"),
                    RFID_POCKET_X + 16.0,
                    RFID_POCKET_Y + 14.0,
                    2.5,
                    3.0,
                )
                .translate(x, y, DECK_Z + RFID_PANEL_Z + 1.5)
                + centered_cube(
                    format!("{BIN_PREFIX}_rfid_read_distance_shim_{index}"),
                    9.0,
                    RFID_POCKET_Y + 20.0,
                    5.0 + index as f64 * 0.6,
                )
                .translate(
                    x + RFID_POCKET_X / 2.0 + 18.0,
                    y,
                    DECK_Z + RFID_PANEL_Z + (5.0 + index as f64 * 0.6) / 2.0,
                );
        }
    }

    panel - pocket_cuts + coils
}

fn purge_dry_air_token_rail() -> Part {
    let rail = centered_cube(
        format!("{BIN_PREFIX}_purge_dry_air_token_rail_base"),
        PURGE_RAIL_X,
        PURGE_RAIL_Y,
        PURGE_RAIL_Z,
    )
    .translate(PURGE_POS.0, PURGE_POS.1, insert_z(PURGE_RAIL_Z));
    let mut token_cuts = Part::empty(format!("{BIN_PREFIX}_purge_token_pocket_cuts"));
    let mut token_bosses = Part::empty(format!("{BIN_PREFIX}_purge_dwell_token_bosses"));
    for index in 0..PURGE_TOKEN_COUNT {
        let x = PURGE_POS.0 + lane_offset(index, PURGE_TOKEN_COUNT, PURGE_TOKEN_PITCH_X);
        token_cuts = token_cuts
            + centered_cylinder(
                format!("{BIN_PREFIX}_purge_token_socket_cut_{index}"),
                PURGE_TOKEN_D,
                PURGE_TOKEN_DEPTH + 2.0,
                32,
            )
            .translate(
                x,
                PURGE_POS.1 - 18.0,
                DECK_Z + PURGE_RAIL_Z - PURGE_TOKEN_DEPTH / 2.0 + 1.0,
            );
        token_bosses = token_bosses
            + centered_cylinder(
                format!("{BIN_PREFIX}_purge_token_id_boss_{index}"),
                8.0,
                3.0,
                24,
            )
            .translate(x, PURGE_POS.1 - 18.0, DECK_Z + PURGE_RAIL_Z + 1.5);
    }

    rail - token_cuts + dry_air_nozzle_row() + token_bosses
}

fn dry_air_nozzle_row() -> Part {
    let header = centered_cube(
        format!("{BIN_PREFIX}_dry_air_purge_header"),
        PURGE_RAIL_X - 42.0,
        16.0,
        18.0,
    )
    .translate(
        PURGE_POS.0,
        PURGE_POS.1 + PURGE_RAIL_Y / 2.0 - 24.0,
        DECK_Z + PURGE_RAIL_Z + 9.0,
    );
    let mut nozzles = Part::empty(format!("{BIN_PREFIX}_dry_air_nozzle_checks"));
    for index in 0..PURGE_NOZZLE_COUNT {
        let x = PURGE_POS.0 + lane_offset(index, PURGE_NOZZLE_COUNT, 50.0);
        nozzles = nozzles
            + centered_cylinder(
                format!("{BIN_PREFIX}_dry_air_nozzle_{index}"),
                5.0,
                14.0,
                20,
            )
            .translate(
                x,
                PURGE_POS.1 + PURGE_RAIL_Y / 2.0 - 24.0,
                DECK_Z + PURGE_RAIL_Z + 23.0,
            );
    }
    header + nozzles
}

fn camera_readhead_focus_bridge() -> Part {
    let left_front = bridge_post("left_front", -BRIDGE_SPAN_X / 2.0, -72.0);
    let left_rear = bridge_post("left_rear", -BRIDGE_SPAN_X / 2.0, 72.0);
    let right_front = bridge_post("right_front", BRIDGE_SPAN_X / 2.0, -72.0);
    let right_rear = bridge_post("right_rear", BRIDGE_SPAN_X / 2.0, 72.0);
    let beam = centered_cube(
        format!("{BIN_PREFIX}_camera_readhead_bridge_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let camera_pods = camera_focus_pods();
    let readheads = rfid_readhead_pods();
    let focus_rule = centered_cube(
        format!("{BIN_PREFIX}_bridge_focus_height_rule"),
        520.0,
        9.0,
        20.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1 + 46.0, BRIDGE_UNDERSIDE_Z - 10.0);

    left_front + left_rear + right_front + right_rear + beam + camera_pods + readheads + focus_rule
}

fn bridge_post(name: &str, local_x: f64, local_y: f64) -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_bridge_{name}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z - DECK_Z,
    )
    .translate(
        BRIDGE_POS.0 + local_x,
        BRIDGE_POS.1 + local_y,
        DECK_Z + (BRIDGE_UNDERSIDE_Z - DECK_Z) / 2.0,
    )
}

fn camera_focus_pods() -> Part {
    let mut pods = Part::empty(format!("{BIN_PREFIX}_camera_focus_pods"));
    for index in 0..CAMERA_POD_COUNT {
        let x = BRIDGE_POS.0 + lane_offset(index, CAMERA_POD_COUNT, 230.0);
        pods =
            pods + centered_cube(
                format!("{BIN_PREFIX}_camera_focus_pod_{index}"),
                CAMERA_POD_X,
                CAMERA_POD_Y,
                CAMERA_POD_Z,
            )
            .translate(
                x,
                BRIDGE_POS.1 - 18.0,
                BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
            ) + centered_cylinder(
                format!("{BIN_PREFIX}_camera_focus_aperture_ring_{index}"),
                34.0,
                5.0,
                36,
            )
            .translate(
                x,
                BRIDGE_POS.1 - 18.0,
                BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z - 2.5,
            );
    }
    pods
}

fn rfid_readhead_pods() -> Part {
    let mut pods = Part::empty(format!("{BIN_PREFIX}_rfid_readhead_pods"));
    for index in 0..READHEAD_POD_COUNT {
        let x = BRIDGE_POS.0 + lane_offset(index, READHEAD_POD_COUNT, 710.0);
        pods = pods
            + centered_cube(
                format!("{BIN_PREFIX}_rfid_readhead_pod_{index}"),
                118.0,
                54.0,
                36.0,
            )
            .translate(x, BRIDGE_POS.1 + 30.0, BRIDGE_UNDERSIDE_Z - 18.0)
            + rectangular_outline(
                format!("{BIN_PREFIX}_rfid_readhead_antenna_outline_{index}"),
                96.0,
                38.0,
                3.0,
                3.0,
            )
            .translate(x, BRIDGE_POS.1 + 30.0, BRIDGE_UNDERSIDE_Z - 37.5);
    }
    pods
}

fn reject_hold_release_gates() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_reject_hold_release_gate_panel"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_PANEL_Z));
    let mut slot_cuts = Part::empty(format!("{BIN_PREFIX}_disposition_gate_slot_cuts"));
    let mut paddles = Part::empty(format!("{BIN_PREFIX}_disposition_gate_paddles"));
    for lane in Disposition::all() {
        let lane_y = GATE_POS.1 + lane_offset(lane.index(), DISPOSITION_LANES, GATE_LANE_PITCH_Y);
        let name = lane.name();
        paddles = paddles
            + centered_cube(
                format!("{BIN_PREFIX}_{name}_lane_status_paddle"),
                GATE_PANEL_X - 42.0,
                9.0,
                18.0,
            )
            .translate(
                GATE_POS.0,
                lane_y + 18.0,
                DECK_Z + GATE_PANEL_Z + 9.0 + lane.index() as f64 * 2.0,
            );
        for slot in 0..GATE_SLOTS_PER_LANE {
            let x = GATE_POS.0 + lane_offset(slot, GATE_SLOTS_PER_LANE, 72.0);
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("{BIN_PREFIX}_{name}_gate_token_slot_{slot}"),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    12.0,
                )
                .translate(x, lane_y - 12.0, DECK_Z + GATE_PANEL_Z - 5.0);
        }
    }

    panel - slot_cuts + paddles
}

fn custody_transfer_lands() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_custody_transfer_lands_panel"),
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_PANEL_Z));
    let mut lands = Part::empty(format!("{BIN_PREFIX}_custody_transfer_lands"));
    for index in 0..CUSTODY_LAND_COUNT {
        let x = CUSTODY_POS.0 + lane_offset(index, CUSTODY_LAND_COUNT, 94.0);
        lands = lands
            + centered_cube(
                format!("{BIN_PREFIX}_custody_land_{index}_inbound_outbound_hold_release"),
                CUSTODY_LAND_X,
                CUSTODY_LAND_Y,
                4.0,
            )
            .translate(x, CUSTODY_POS.1 + 18.0, DECK_Z + CUSTODY_PANEL_Z + 2.0)
            + centered_cube(
                format!("{BIN_PREFIX}_custody_chain_barcode_land_{index}"),
                CUSTODY_LAND_X - 18.0,
                14.0,
                3.0,
            )
            .translate(x, CUSTODY_POS.1 - 34.0, DECK_Z + CUSTODY_PANEL_Z + 1.5);
    }

    panel + lands + custody_transfer_tokens()
}

fn custody_transfer_tokens() -> Part {
    let mut tokens = Part::empty(format!("{BIN_PREFIX}_custody_transfer_keyed_tokens"));
    for index in 0..CUSTODY_TOKEN_COUNT {
        let x = CUSTODY_POS.0 + lane_offset(index, CUSTODY_TOKEN_COUNT, 94.0);
        tokens = tokens
            + centered_cylinder(
                format!("{BIN_PREFIX}_custody_transfer_token_{index}"),
                20.0,
                6.0,
                28,
            )
            .translate(
                x + 22.0,
                CUSTODY_POS.1 + 18.0,
                DECK_Z + CUSTODY_PANEL_Z + 7.0,
            );
    }
    tokens
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        format!("{BIN_PREFIX}_front_robot_approach_clearance_gauge"),
        KEEP_OUT_X,
        10.0,
        18.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE, DECK_Z + 9.0);
    let rear_service = centered_cube(
        format!("{BIN_PREFIX}_rear_service_clearance_gauge"),
        KEEP_OUT_X,
        10.0,
        18.0,
    )
    .translate(0.0, DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE, DECK_Z + 9.0);
    let left_service = centered_cube(
        format!("{BIN_PREFIX}_left_service_clearance_gauge"),
        10.0,
        KEEP_OUT_Y,
        18.0,
    )
    .translate(-DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE, 0.0, DECK_Z + 9.0);
    let right_service = centered_cube(
        format!("{BIN_PREFIX}_right_service_clearance_gauge"),
        10.0,
        KEEP_OUT_Y,
        18.0,
    )
    .translate(DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE, 0.0, DECK_Z + 9.0);
    let mut height_posts = Part::empty(format!("{BIN_PREFIX}_robot_z_clearance_posts"));
    for index in 0..KEEP_OUT_GAUGE_COUNT {
        let x = lane_offset(index, KEEP_OUT_GAUGE_COUNT, 260.0);
        height_posts = height_posts
            + centered_cylinder(
                format!("{BIN_PREFIX}_robot_z_clearance_post_{index}"),
                12.0,
                ROBOT_Z_CLEARANCE,
                28,
            )
            .translate(x, -DECK_Y / 2.0 + 60.0, DECK_Z + ROBOT_Z_CLEARANCE / 2.0);
    }

    front_robot + rear_service + left_service + right_service + height_posts
}

fn rectangular_outline(name: String, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let left = centered_cube(format!("{name}_left"), wall, y, z).translate(
        -x / 2.0 + wall / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), wall, y, z).translate(
        x / 2.0 - wall / 2.0,
        0.0,
        0.0,
    );
    let front = centered_cube(format!("{name}_front"), x, wall, z).translate(
        0.0,
        -y / 2.0 + wall / 2.0,
        0.0,
    );
    let rear =
        centered_cube(format!("{name}_rear"), x, wall, z).translate(0.0, y / 2.0 - wall / 2.0, 0.0);
    left + right + front + rear
}

fn surface_rects() -> [Rect; 8] {
    [
        Rect {
            name: "dock",
            center: DOCK_POS,
            x: DOCK_X,
            y: DOCK_Y,
        },
        Rect {
            name: "fogging",
            center: FOG_POS,
            x: FOG_PLATE_X,
            y: FOG_PLATE_Y,
        },
        Rect {
            name: "droplet_grid",
            center: DROPLET_POS,
            x: DROPLET_GRID_X,
            y: DROPLET_GRID_Y,
        },
        Rect {
            name: "barcode",
            center: BARCODE_POS,
            x: BARCODE_PANEL_X,
            y: BARCODE_PANEL_Y,
        },
        Rect {
            name: "rfid",
            center: RFID_POS,
            x: RFID_PANEL_X,
            y: RFID_PANEL_Y,
        },
        Rect {
            name: "purge",
            center: PURGE_POS,
            x: PURGE_RAIL_X,
            y: PURGE_RAIL_Y,
        },
        Rect {
            name: "gates",
            center: GATE_POS,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
        Rect {
            name: "custody",
            center: CUSTODY_POS,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
    ]
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn barcode_step_height(index: usize) -> f64 {
    CONTRAST_MIN_Z + index as f64 * CONTRAST_DELTA_Z
}

fn barcode_ladder_max_z() -> f64 {
    DECK_Z + BARCODE_PANEL_Z + barcode_step_height(BARCODE_STEP_COUNT - 1)
}

fn bridge_clearance_over_barcode() -> f64 {
    BRIDGE_UNDERSIDE_Z - barcode_ladder_max_z()
}

fn lane_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_design_constraints() {
    assert!(
        bridge_clearance_over_barcode() > 160.0,
        "camera/readhead bridge does not clear the tallest barcode contrast step"
    );
    assert_eq!(
        DISPOSITION_LANES, 3,
        "release/hold/reject gates must provide exactly three disposition lanes"
    );
    assert!(
        FOG_COUPON_COUNT >= 8,
        "fogging coupon plate needs at least eight coupon lands"
    );
    assert!(
        DROPLET_WELL_COUNT >= 24,
        "condensation witness grid needs at least twenty-four droplet wells"
    );
    for rect in surface_rects() {
        assert!(
            rect.fits_inside_deck(),
            "{} fixture exceeds deck envelope",
            rect.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_manifest_is_scoped_unique_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(
            OUTPUTS.last().copied(),
            Some("output/closed_barcode_rfid_plate_fogging_condensation_readability_station_assembly.stl")
        );
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_outputs_cover_requested_station() {
        let joined = OUTPUTS.join("\n");
        for feature in REQUIRED_FEATURES {
            assert!(
                joined.contains(feature),
                "missing required feature output for {feature}"
            );
        }
        assert_eq!(REQUIRED_FEATURES.len(), OUTPUTS.len() - 1);
    }

    #[test]
    fn challenge_counts_are_mechanically_visible() {
        assert_eq!(FOG_COUPON_COUNT, 8);
        assert_eq!(DROPLET_WELL_COUNT, 24);
        assert_eq!(BARCODE_STEP_COUNT, 9);
        assert_eq!(RFID_POCKET_COUNT, 8);
        assert_eq!(PURGE_TOKEN_COUNT, 6);
        assert_eq!(CUSTODY_LAND_COUNT, 4);
        assert_eq!(DISPOSITION_LANES, 3);
    }

    #[test]
    fn barcode_contrast_ladder_is_monotonic() {
        for index in 1..BARCODE_STEP_COUNT {
            assert!(barcode_step_height(index) > barcode_step_height(index - 1));
        }
        assert!(barcode_step_height(BARCODE_STEP_COUNT - 1) - barcode_step_height(0) >= 8.0);
    }

    #[test]
    fn surface_modules_fit_without_2d_collision() {
        let rects = surface_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit deck", rect.name);
        }
        for left in 0..rects.len() {
            for right in left + 1..rects.len() {
                assert!(
                    !rects[left].overlaps(rects[right]),
                    "{} overlaps {}",
                    rects[left].name,
                    rects[right].name
                );
            }
        }
    }

    #[test]
    fn bridge_and_keepout_dimensions_clear_readability_surfaces() {
        assert!(bridge_clearance_over_barcode() > 160.0);
        assert!(BRIDGE_SPAN_X < DECK_X - 100.0);
        assert!(ROBOT_FRONT_CLEARANCE > PURGE_RAIL_Y);
        assert!(SERVICE_REAR_CLEARANCE > FOG_PLATE_Y);
        assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z);
    }
}
