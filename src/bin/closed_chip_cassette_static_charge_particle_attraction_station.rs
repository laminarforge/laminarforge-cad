use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed chip cassette static-charge particle attraction validation station.
//
// Intent:
// - Compare grounded and deliberately ungrounded cassette handling lanes inside
//   a secondary-contained fixture without opening the chip cassette path.
// - Stage placebo/ionizer exposure zones, particle witness coupons, a charge
//   probe pocket, evidence window frame, barcode/custody labels, and
//   release/hold/reject disposition lanes as physical validation interfaces.
// - Make robot approach and service clearances visible as removable gauges.
//
// Product concept CAD only. This file models fixture geometry and labels, not
// acceptance limits, electrostatic discharge controls, particle chemistry,
// ionizer qualification, or release criteria.

const OUTPUTS: [&str; 11] = [
    "output/closed_chip_cassette_static_charge_particle_attraction_station_secondary_containment_deck.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_cassette_nest.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_ionizer_placebo_zone.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_particle_witness_coupon_lands.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_charge_probe_pocket.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_grounded_ungrounded_comparison_lanes.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_transparent_evidence_window_frame.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_barcode_custody_labels.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_release_hold_reject_lanes.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_robot_service_keepout_gauges.stl",
    "output/closed_chip_cassette_static_charge_particle_attraction_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "cassette_nest",
    "ionizer_placebo_zone",
    "particle_witness_coupon_lands",
    "charge_probe_pocket",
    "grounded_ungrounded_comparison_lanes",
    "transparent_evidence_window_frame",
    "barcode_custody_labels",
    "release_hold_reject_lanes",
    "secondary_containment",
    "robot_service_keepout_gauges",
];

const DECK_X: f64 = 1420.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_CHANNEL_W: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CASSETTE_NEST_CENTER: (f64, f64) = (-355.0, 108.0);
const CASSETTE_LANES: usize = 2;
const CASSETTES_PER_LANE: usize = 3;
const CASSETTE_SLOT_COUNT: usize = CASSETTE_LANES * CASSETTES_PER_LANE;
const CASSETTE_SLOT_X: f64 = REVC_CHIP_LENGTH + 16.0;
const CASSETTE_SLOT_Y: f64 = REVC_CHIP_WIDTH + 14.0;
const CASSETTE_SLOT_DEPTH: f64 = 12.0;
const CASSETTE_SLOT_PITCH_X: f64 = CASSETTE_SLOT_X + 20.0;
const CASSETTE_SLOT_PITCH_Y: f64 = CASSETTE_SLOT_Y + 20.0;
const CASSETTE_NEST_MARGIN_X: f64 = 30.0;
const CASSETTE_NEST_MARGIN_Y: f64 = 26.0;
const CASSETTE_NEST_X: f64 = (CASSETTES_PER_LANE as f64 - 1.0) * CASSETTE_SLOT_PITCH_X
    + CASSETTE_SLOT_X
    + 2.0 * CASSETTE_NEST_MARGIN_X;
const CASSETTE_NEST_Y: f64 = (CASSETTE_LANES as f64 - 1.0) * CASSETTE_SLOT_PITCH_Y
    + CASSETTE_SLOT_Y
    + 2.0 * CASSETTE_NEST_MARGIN_Y;
const CASSETTE_NEST_Z: f64 = 40.0;
const NEST_RAIL_W: f64 = 12.0;
const NEST_RAIL_Z: f64 = 26.0;
const NEST_GROUND_PIN_COUNT: usize = 6;
const NEST_INSULATOR_PAD_COUNT: usize = 6;

const IONIZER_CENTER: (f64, f64) = (270.0, 246.0);
const IONIZER_X: f64 = 430.0;
const IONIZER_Y: f64 = 190.0;
const IONIZER_Z: f64 = 58.0;
const IONIZER_TUNNELS: usize = 2;
const IONIZER_NOZZLES_PER_TUNNEL: usize = 4;
const IONIZER_TOTAL_NOZZLES: usize = IONIZER_TUNNELS * IONIZER_NOZZLES_PER_TUNNEL;
const IONIZER_TUNNEL_Y: f64 = 50.0;
const IONIZER_TUNNEL_PITCH_Y: f64 = 74.0;
const IONIZER_NOZZLE_D: f64 = 16.0;
const PLACEBO_BLANK_COUNT: usize = 4;

const WITNESS_CENTER: (f64, f64) = (-460.0, -226.0);
const WITNESS_X: f64 = 360.0;
const WITNESS_Y: f64 = 210.0;
const WITNESS_Z: f64 = 30.0;
const WITNESS_ROWS: usize = 2;
const WITNESS_COLS: usize = 5;
const WITNESS_COUPON_COUNT: usize = WITNESS_ROWS * WITNESS_COLS;
const WITNESS_COUPON_X: f64 = 48.0;
const WITNESS_COUPON_Y: f64 = 72.0;
const WITNESS_PITCH_X: f64 = 62.0;
const WITNESS_PITCH_Y: f64 = 96.0;
const WITNESS_RETENTION_CLIP_COUNT: usize = WITNESS_COUPON_COUNT * 2;

const PROBE_CENTER: (f64, f64) = (-76.0, -226.0);
const PROBE_X: f64 = 314.0;
const PROBE_Y: f64 = 210.0;
const PROBE_Z: f64 = 52.0;
const PROBE_POCKETS: usize = 4;
const PROBE_POCKET_D: f64 = 31.0;
const PROBE_POCKET_PITCH_X: f64 = 64.0;
const PROBE_CABLE_COMB_SLOTS: usize = 5;
const PROBE_REFERENCE_PLATE_X: f64 = 236.0;
const PROBE_REFERENCE_PLATE_Y: f64 = 52.0;

const COMPARISON_CENTER: (f64, f64) = (342.0, -86.0);
const COMPARISON_X: f64 = 430.0;
const COMPARISON_Y: f64 = 248.0;
const COMPARISON_Z: f64 = 42.0;
const COMPARISON_LANES: usize = 2;
const STATIONS_PER_COMPARISON_LANE: usize = 5;
const COMPARISON_STATION_COUNT: usize = COMPARISON_LANES * STATIONS_PER_COMPARISON_LANE;
const COMPARISON_STATION_X: f64 = 60.0;
const COMPARISON_STATION_Y: f64 = 58.0;
const COMPARISON_STATION_PITCH_X: f64 = 72.0;
const COMPARISON_LANE_PITCH_Y: f64 = 92.0;
const GROUND_STRAP_POSTS: usize = 5;
const FLOATING_ISOLATOR_POSTS: usize = 5;

const EVIDENCE_CENTER: (f64, f64) = (-22.0, 380.0);
const EVIDENCE_X: f64 = 1110.0;
const EVIDENCE_Y: f64 = 70.0;
const EVIDENCE_Z: f64 = 34.0;
const EVIDENCE_RAIL_W: f64 = 12.0;
const EVIDENCE_CLEAR_X: f64 = 928.0;
const EVIDENCE_CLEAR_Y: f64 = 34.0;
const EVIDENCE_CAMERA_PODS: usize = 4;
const EVIDENCE_LED_SEGMENTS: usize = 10;
const EVIDENCE_HINGE_KNUCKLES: usize = 6;
const EVIDENCE_DATUM_PINS: usize = 4;

const LABEL_CENTER: (f64, f64) = (510.0, 95.0);
const LABEL_X: f64 = 318.0;
const LABEL_Y: f64 = 112.0;
const LABEL_Z: f64 = 12.0;
const BARCODE_LABELS: usize = 8;
const CUSTODY_LABELS: usize = 4;
const LOT_CARD_LANDS: usize = 3;

const STATUS_CENTER: (f64, f64) = (456.0, -314.0);
const STATUS_X: f64 = 388.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 36.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 74.0;
const STATUS_SLOT_Y: f64 = 30.0;
const STATUS_SLOT_PITCH_X: f64 = 86.0;
const STATUS_LANE_PITCH_Y: f64 = 44.0;

const KEEP_OUT_X: f64 = 1320.0;
const KEEP_OUT_Y: f64 = 790.0;
const KEEP_OUT_Z: f64 = 108.0;
const ROBOT_FRONT_CLEARANCE_MM: f64 = 370.0;
const SERVICE_REAR_CLEARANCE_MM: f64 = 250.0;
const IONIZER_SERVICE_CLEARANCE_MM: f64 = 190.0;
const CHARGE_PROBE_PULL_CLEARANCE_MM: f64 = 210.0;
const ROBOT_PICK_CLEARANCE_Z_MM: f64 = 150.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 14.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 14.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = cassette_nest();
    export(OUTPUTS[1], &nest);

    let ionizer = ionizer_placebo_zone();
    export(OUTPUTS[2], &ionizer);

    let witnesses = particle_witness_coupon_lands();
    export(OUTPUTS[3], &witnesses);

    let probe = charge_probe_pocket();
    export(OUTPUTS[4], &probe);

    let comparison = grounded_ungrounded_comparison_lanes();
    export(OUTPUTS[5], &comparison);

    let evidence = transparent_evidence_window_frame();
    export(OUTPUTS[6], &evidence);

    let labels = barcode_custody_labels();
    export(OUTPUTS[7], &labels);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[8], &status);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[9], &keepouts);

    let assembly = deck
        + nest
        + ionizer
        + witnesses
        + probe
        + comparison
        + evidence
        + labels
        + status
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed chip cassette static-charge particle attraction station:");
    println!(
        "  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm secondary-contained deck with drain channel and sealed rim"
    );
    println!(
        "  Cassette nest:              {CASSETTE_LANES} lanes x {CASSETTES_PER_LANE} cassette slots ({CASSETTE_SLOT_COUNT} total), {:.1}mm x {:.1}mm slot envelope",
        CASSETTE_SLOT_X, CASSETTE_SLOT_Y
    );
    println!(
        "  Charge challenge:           {IONIZER_TUNNELS} placebo/ionizer tunnels, {IONIZER_TOTAL_NOZZLES} nozzle bosses, {PLACEBO_BLANK_COUNT} placebo blank caps"
    );
    println!(
        "  Particle evidence:          {WITNESS_COUPON_COUNT} witness coupon lands with {WITNESS_RETENTION_CLIP_COUNT} retention clips and {PROBE_POCKETS} charge probe pockets"
    );
    println!(
        "  Comparison controls:        {COMPARISON_LANES} grounded/ungrounded lanes, {COMPARISON_STATION_COUNT} indexed stations, {GROUND_STRAP_POSTS} ground strap posts, {FLOATING_ISOLATOR_POSTS} floating isolator posts"
    );
    println!(
        "  Traceability:               transparent evidence window with {EVIDENCE_CAMERA_PODS} camera pods, {BARCODE_LABELS} barcode labels, {CUSTODY_LABELS} custody labels, and {LOT_CARD_LANDS} lot-card lands"
    );
    println!(
        "  Disposition/clearance:      release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each; robot/service gauges cover {:.0}mm front, {:.0}mm rear, {:.0}mm ionizer side, {:.0}mm probe pull, {:.0}mm Z",
        ROBOT_FRONT_CLEARANCE_MM,
        SERVICE_REAR_CLEARANCE_MM,
        IONIZER_SERVICE_CLEARANCE_MM,
        CHARGE_PROBE_PULL_CLEARANCE_MM,
        ROBOT_PICK_CLEARANCE_Z_MM
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    for rect in layout_rects() {
        assert!(rect.fits_inside(), "{} exceeds station envelope", rect.name);
    }

    for (a, b) in non_overlap_pairs() {
        assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
    }

    assert_eq!(CASSETTE_SLOT_COUNT, CASSETTE_LANES * CASSETTES_PER_LANE);
    assert_eq!(
        IONIZER_TOTAL_NOZZLES,
        IONIZER_TUNNELS * IONIZER_NOZZLES_PER_TUNNEL
    );
    assert_eq!(WITNESS_COUPON_COUNT, WITNESS_ROWS * WITNESS_COLS);
    assert_eq!(STATUS_LANES, 3);
    assert_eq!(COMPARISON_LANES, 2);
    assert_eq!(
        COMPARISON_STATION_COUNT,
        COMPARISON_LANES * STATIONS_PER_COMPARISON_LANE
    );
    assert_eq!(EVIDENCE_DATUM_PINS, 4);
    assert!(ROBOT_PICK_CLEARANCE_Z_MM > CASSETTE_NEST_Z + DECK_Z + 80.0);
    assert!(EVIDENCE_CLEAR_X < EVIDENCE_X - 2.0 * EVIDENCE_RAIL_W);
    assert!(EVIDENCE_CLEAR_Y < EVIDENCE_Y - 2.0 * EVIDENCE_RAIL_W);
    assert!(CASSETTE_SLOT_X > REVC_CHIP_LENGTH);
    assert!(CASSETTE_SLOT_Y > REVC_CHIP_WIDTH);
}

fn layout_rects() -> [Rect; 8] {
    [
        rect(
            "cassette_nest",
            CASSETTE_NEST_CENTER,
            CASSETTE_NEST_X,
            CASSETTE_NEST_Y,
        ),
        rect("ionizer_placebo_zone", IONIZER_CENTER, IONIZER_X, IONIZER_Y),
        rect(
            "particle_witness_coupon_lands",
            WITNESS_CENTER,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect("charge_probe_pocket", PROBE_CENTER, PROBE_X, PROBE_Y),
        rect(
            "grounded_ungrounded_comparison_lanes",
            COMPARISON_CENTER,
            COMPARISON_X,
            COMPARISON_Y,
        ),
        rect(
            "transparent_evidence_window_frame",
            EVIDENCE_CENTER,
            EVIDENCE_X,
            EVIDENCE_Y,
        ),
        rect("barcode_custody_labels", LABEL_CENTER, LABEL_X, LABEL_Y),
        rect(
            "release_hold_reject_lanes",
            STATUS_CENTER,
            STATUS_X,
            STATUS_Y,
        ),
    ]
}

fn non_overlap_pairs() -> [(Rect, Rect); 14] {
    let rects = layout_rects();
    [
        (rects[0], rects[1]),
        (rects[0], rects[2]),
        (rects[0], rects[3]),
        (rects[0], rects[4]),
        (rects[0], rects[5]),
        (rects[1], rects[5]),
        (rects[1], rects[6]),
        (rects[1], rects[7]),
        (rects[2], rects[3]),
        (rects[2], rects[4]),
        (rects[3], rects[4]),
        (rects[4], rects[6]),
        (rects[4], rects[7]),
        (rects[6], rects[7]),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn place_on_deck(part: Part, center: (f64, f64), height: f64) -> Part {
    part.translate(
        center.0,
        center.1,
        DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0,
    )
}

fn deck_top_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        "static_charge_particle_station_secondary_containment_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - insert_sockets() - drain_channels() - deck_mount_holes()
        + perimeter_rim()
        + internal_zone_spines()
        + drain_label_tabs()
        + deck_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("static_charge_particle_station_insert_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("static_charge_particle_station_{}_socket", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn drain_channels() -> Part {
    let front = centered_cube(
        "static_charge_particle_station_front_secondary_containment_drain_channel",
        DECK_X - 160.0,
        DRAIN_CHANNEL_W,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 64.0, DECK_Z - 3.0);
    let left_witness = centered_cube(
        "static_charge_particle_station_witness_coupon_runoff_channel",
        DRAIN_CHANNEL_W,
        WITNESS_Y + 56.0,
        6.0,
    )
    .translate(
        WITNESS_CENTER.0 + WITNESS_X / 2.0 + 26.0,
        WITNESS_CENTER.1,
        DECK_Z - 3.0,
    );
    let probe = centered_cube(
        "static_charge_particle_station_probe_pocket_runoff_channel",
        PROBE_X + 42.0,
        12.0,
        6.0,
    )
    .translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1 - PROBE_Y / 2.0 - 24.0,
        DECK_Z - 3.0,
    );
    let drain_port = centered_cylinder(
        "static_charge_particle_station_closed_drain_port",
        7.5,
        46.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 32.0, DECK_Z - 7.0);

    front + left_witness + probe + drain_port
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("static_charge_particle_station_deck_mount_holes");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("static_charge_particle_station_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                32,
            )
            .translate(
                sx * (DECK_X / 2.0 - 52.0),
                sy * (DECK_Y / 2.0 - 52.0),
                DECK_Z / 2.0,
            );
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "static_charge_particle_station_front_secondary_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "static_charge_particle_station_rear_service_secondary_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "static_charge_particle_station_left_secondary_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "static_charge_particle_station_right_secondary_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn internal_zone_spines() -> Part {
    let clean_to_charge_spine = centered_cube(
        "static_charge_particle_station_clean_charge_zone_spine",
        DECK_X - 170.0,
        10.0,
        26.0,
    )
    .translate(0.0, -104.0, DECK_Z + 13.0);
    let nest_to_evidence_spine = centered_cube(
        "static_charge_particle_station_nest_to_evidence_spine",
        12.0,
        360.0,
        24.0,
    )
    .translate(102.0, 168.0, DECK_Z + 12.0);
    let comparison_to_status_spine = centered_cube(
        "static_charge_particle_station_comparison_status_spine",
        430.0,
        10.0,
        22.0,
    )
    .translate(342.0, -214.0, DECK_Z + 11.0);

    clean_to_charge_spine + nest_to_evidence_spine + comparison_to_status_spine
}

fn drain_label_tabs() -> Part {
    let mut tabs = Part::empty("static_charge_particle_station_secondary_containment_label_tabs");
    for i in 0..5 {
        tabs = tabs
            + centered_cube(
                format!("static_charge_particle_station_containment_drain_label_tab_{i}"),
                86.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, 5, 150.0),
                -DECK_Y / 2.0 + 94.0,
                DECK_Z + 2.0,
            );
    }
    tabs
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("static_charge_particle_station_robot_base_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 92.0, -DECK_Y / 2.0 + 92.0),
        (DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 92.0),
        (-DECK_X / 2.0 + 92.0, DECK_Y / 2.0 - 92.0),
        (DECK_X / 2.0 - 92.0, DECK_Y / 2.0 - 92.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets =
            targets
                + fiducial_disc(&format!("static_charge_particle_station_base_datum_{i}"))
                    .translate(x, y, DECK_Z + 2.5);
    }
    targets
}

fn cassette_nest() -> Part {
    let body = centered_cube(
        "static_charge_particle_cassette_nest_body",
        CASSETTE_NEST_X,
        CASSETTE_NEST_Y,
        CASSETTE_NEST_Z,
    );
    let pockets = cassette_slot_pockets();
    let rails = cassette_nest_rails();
    let ground_pins = cassette_ground_reference_pins();
    let insulators = cassette_insulator_pads();
    let witness_index_lands = cassette_slot_index_lands();
    let lane_divider = centered_cube(
        "static_charge_particle_cassette_nest_grounded_ungrounded_center_divider",
        CASSETTE_NEST_X - 34.0,
        10.0,
        28.0,
    )
    .translate(0.0, 0.0, CASSETTE_NEST_Z / 2.0 + 14.0);

    place_on_deck(
        body - pockets + rails + ground_pins + insulators + witness_index_lands + lane_divider,
        CASSETTE_NEST_CENTER,
        CASSETTE_NEST_Z,
    )
}

fn cassette_slot_pockets() -> Part {
    let mut pockets = Part::empty("static_charge_particle_cassette_slot_pockets");
    for index in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = cassette_slot_center(index);
        let slot = centered_cube(
            format!("static_charge_particle_cassette_slot_{index:02}_closed_path_pocket"),
            CASSETTE_SLOT_X,
            CASSETTE_SLOT_Y,
            CASSETTE_SLOT_DEPTH + 1.0,
        )
        .translate(
            x,
            y,
            CASSETTE_NEST_Z / 2.0 - CASSETTE_SLOT_DEPTH / 2.0 + 0.6,
        );
        let barcode_sightline = centered_cube(
            format!("static_charge_particle_cassette_slot_{index:02}_barcode_sightline_notch"),
            CASSETTE_SLOT_X * 0.42,
            9.0,
            CASSETTE_SLOT_DEPTH + 2.0,
        )
        .translate(
            x,
            y - CASSETTE_SLOT_Y / 2.0 + 8.0,
            CASSETTE_NEST_Z / 2.0 - CASSETTE_SLOT_DEPTH / 2.0 + 1.0,
        );
        pockets = pockets + slot + barcode_sightline;
    }
    pockets
}

fn cassette_nest_rails() -> Part {
    let rear = centered_cube(
        "static_charge_particle_cassette_nest_rear_datum_rail",
        CASSETTE_NEST_X,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        CASSETTE_NEST_Y / 2.0 - NEST_RAIL_W / 2.0,
        CASSETTE_NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let left = centered_cube(
        "static_charge_particle_cassette_nest_left_ground_datum_rail",
        NEST_RAIL_W,
        CASSETTE_NEST_Y,
        NEST_RAIL_Z,
    )
    .translate(
        -CASSETTE_NEST_X / 2.0 + NEST_RAIL_W / 2.0,
        0.0,
        CASSETTE_NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let right = centered_cube(
        "static_charge_particle_cassette_nest_right_floating_soft_rail",
        NEST_RAIL_W,
        CASSETTE_NEST_Y - 34.0,
        NEST_RAIL_Z * 0.72,
    )
    .translate(
        CASSETTE_NEST_X / 2.0 - NEST_RAIL_W / 2.0,
        -10.0,
        CASSETTE_NEST_Z / 2.0 + NEST_RAIL_Z * 0.36,
    );
    let front_low = centered_cube(
        "static_charge_particle_cassette_nest_front_low_robot_access_rail",
        CASSETTE_NEST_X - 90.0,
        9.0,
        14.0,
    )
    .translate(
        8.0,
        -CASSETTE_NEST_Y / 2.0 + 14.0,
        CASSETTE_NEST_Z / 2.0 + 7.0,
    );

    rear + left + right + front_low
}

fn cassette_ground_reference_pins() -> Part {
    let mut pins = Part::empty("static_charge_particle_cassette_ground_reference_pins");
    for i in 0..NEST_GROUND_PIN_COUNT {
        let (x, y) = cassette_slot_center(i);
        pins = pins
            + centered_cylinder(
                format!("static_charge_particle_grounded_lane_contact_pin_{i}"),
                4.0,
                8.0,
                28,
            )
            .translate(
                x - CASSETTE_SLOT_X / 2.0 + 22.0,
                y + CASSETTE_SLOT_Y / 2.0 - 18.0,
                CASSETTE_NEST_Z / 2.0 + 4.0,
            );
    }
    pins
}

fn cassette_insulator_pads() -> Part {
    let mut pads = Part::empty("static_charge_particle_cassette_insulator_pads");
    for i in 0..NEST_INSULATOR_PAD_COUNT {
        let (x, y) = cassette_slot_center(i);
        pads = pads
            + centered_cube(
                format!("static_charge_particle_floating_lane_insulator_pad_{i}"),
                42.0,
                10.0,
                5.0,
            )
            .translate(
                x + CASSETTE_SLOT_X / 2.0 - 34.0,
                y - CASSETTE_SLOT_Y / 2.0 + 16.0,
                CASSETTE_NEST_Z / 2.0 + 2.5,
            );
    }
    pads
}

fn cassette_slot_index_lands() -> Part {
    let mut lands = Part::empty("static_charge_particle_cassette_slot_index_lands");
    for index in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = cassette_slot_center(index);
        lands = lands
            + centered_cube(
                format!("static_charge_particle_cassette_slot_{index:02}_index_land"),
                78.0,
                16.0,
                4.0,
            )
            .translate(
                x,
                y + CASSETTE_SLOT_Y / 2.0 + 13.0,
                CASSETTE_NEST_Z / 2.0 + 2.0,
            )
            + fiducial_disc(&format!(
                "static_charge_particle_cassette_slot_{index:02}_orientation_dot"
            ))
            .translate(
                x - CASSETTE_SLOT_X / 2.0 + 18.0,
                y - CASSETTE_SLOT_Y / 2.0 + 18.0,
                CASSETTE_NEST_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn cassette_slot_center(index: usize) -> (f64, f64) {
    let lane = index / CASSETTES_PER_LANE;
    let column = index % CASSETTES_PER_LANE;
    (
        centered_index(column, CASSETTES_PER_LANE, CASSETTE_SLOT_PITCH_X),
        centered_index(lane, CASSETTE_LANES, CASSETTE_SLOT_PITCH_Y),
    )
}

fn ionizer_placebo_zone() -> Part {
    let body = centered_cube(
        "static_charge_particle_ionizer_placebo_zone_body",
        IONIZER_X,
        IONIZER_Y,
        IONIZER_Z,
    );
    let tunnel_cuts = ionizer_tunnel_cuts();
    let nozzle_manifold = ionizer_nozzle_manifold();
    let placebo_caps = placebo_blank_caps();
    let flow_arrows = ionizer_airflow_index_arrows();
    let cable_bulkhead = centered_cube(
        "static_charge_particle_ionizer_cable_bulkhead_land",
        108.0,
        24.0,
        26.0,
    )
    .translate(0.0, IONIZER_Y / 2.0 + 10.0, IONIZER_Z / 2.0 + 13.0);

    place_on_deck(
        body - tunnel_cuts + nozzle_manifold + placebo_caps + flow_arrows + cable_bulkhead,
        IONIZER_CENTER,
        IONIZER_Z,
    )
}

fn ionizer_tunnel_cuts() -> Part {
    let mut cuts = Part::empty("static_charge_particle_ionizer_placebo_tunnel_cuts");
    for i in 0..IONIZER_TUNNELS {
        let y = centered_index(i, IONIZER_TUNNELS, IONIZER_TUNNEL_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("static_charge_particle_ionizer_placebo_tunnel_{i}"),
                IONIZER_X - 54.0,
                IONIZER_TUNNEL_Y,
                IONIZER_Z + 2.0,
            )
            .translate(0.0, y, 0.0);
    }
    cuts
}

fn ionizer_nozzle_manifold() -> Part {
    let manifold = centered_cube(
        "static_charge_particle_ionizer_nozzle_manifold_bar",
        IONIZER_X - 76.0,
        18.0,
        28.0,
    )
    .translate(0.0, IONIZER_TUNNEL_PITCH_Y / 2.0, IONIZER_Z / 2.0 + 14.0);
    let mut nozzles = Part::empty("static_charge_particle_ionizer_nozzle_bosses");
    for i in 0..IONIZER_NOZZLES_PER_TUNNEL {
        let x = centered_index(i, IONIZER_NOZZLES_PER_TUNNEL, 82.0);
        nozzles = nozzles
            + centered_cylinder(
                format!("static_charge_particle_active_ionizer_nozzle_boss_{i}"),
                IONIZER_NOZZLE_D / 2.0,
                18.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, IONIZER_TUNNEL_PITCH_Y / 2.0, IONIZER_Z / 2.0 + 14.0);
    }
    manifold + nozzles
}

fn placebo_blank_caps() -> Part {
    let mut caps = Part::empty("static_charge_particle_placebo_blank_caps");
    for i in 0..PLACEBO_BLANK_COUNT {
        let x = centered_index(i, PLACEBO_BLANK_COUNT, 82.0);
        caps = caps
            + centered_cylinder(
                format!("static_charge_particle_placebo_blank_cap_{i}"),
                IONIZER_NOZZLE_D / 2.0,
                10.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -IONIZER_TUNNEL_PITCH_Y / 2.0, IONIZER_Z / 2.0 + 10.0);
    }
    caps
}

fn ionizer_airflow_index_arrows() -> Part {
    let mut arrows = Part::empty("static_charge_particle_ionizer_airflow_index_arrows");
    for i in 0..IONIZER_TUNNELS {
        let y = centered_index(i, IONIZER_TUNNELS, IONIZER_TUNNEL_PITCH_Y);
        arrows = arrows
            + centered_cube(
                format!("static_charge_particle_ionizer_airflow_arrow_shaft_{i}"),
                88.0,
                6.0,
                4.0,
            )
            .translate(-IONIZER_X / 2.0 + 72.0, y, IONIZER_Z / 2.0 + 2.0)
            + centered_cube(
                format!("static_charge_particle_ionizer_airflow_arrow_head_{i}"),
                18.0,
                18.0,
                4.0,
            )
            .translate(-IONIZER_X / 2.0 + 122.0, y, IONIZER_Z / 2.0 + 2.0);
    }
    arrows
}

fn particle_witness_coupon_lands() -> Part {
    let body = centered_cube(
        "static_charge_particle_witness_coupon_land_bank_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let coupon_recesses = witness_coupon_recesses();
    let witness_frames = witness_coupon_frames();
    let clips = witness_retention_clips();
    let clean_used_divider = centered_cube(
        "static_charge_particle_witness_coupon_clean_used_divider",
        12.0,
        WITNESS_Y - 28.0,
        38.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0 + 19.0);
    let transfer_lip = centered_cube(
        "static_charge_particle_witness_coupon_closed_transfer_lip",
        WITNESS_X - 52.0,
        10.0,
        18.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 12.0, WITNESS_Z / 2.0 + 9.0);

    place_on_deck(
        body - coupon_recesses + witness_frames + clips + clean_used_divider + transfer_lip,
        WITNESS_CENTER,
        WITNESS_Z,
    )
}

fn witness_coupon_recesses() -> Part {
    let mut recesses = Part::empty("static_charge_particle_witness_coupon_recesses");
    for i in 0..WITNESS_COUPON_COUNT {
        let (x, y) = witness_coupon_center(i);
        recesses = recesses
            + centered_cube(
                format!("static_charge_particle_witness_coupon_{i:02}_glass_land_recess"),
                WITNESS_COUPON_X,
                WITNESS_COUPON_Y,
                8.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 - 3.6);
    }
    recesses
}

fn witness_coupon_frames() -> Part {
    let mut frames = Part::empty("static_charge_particle_witness_coupon_frames");
    for i in 0..WITNESS_COUPON_COUNT {
        let (x, y) = witness_coupon_center(i);
        frames = frames
            + rectangular_frame(
                &format!("static_charge_particle_witness_coupon_{i:02}_raised_frame"),
                WITNESS_COUPON_X + 12.0,
                WITNESS_COUPON_Y + 12.0,
                5.0,
                5.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 2.5);
    }
    frames
}

fn witness_retention_clips() -> Part {
    let mut clips = Part::empty("static_charge_particle_witness_coupon_retention_clips");
    for i in 0..WITNESS_COUPON_COUNT {
        let (x, y) = witness_coupon_center(i);
        for side in [-1.0, 1.0] {
            clips = clips
                + centered_cube(
                    format!(
                        "static_charge_particle_witness_coupon_{i:02}_retention_clip_{}",
                        if side < 0.0 { "left" } else { "right" }
                    ),
                    7.0,
                    28.0,
                    10.0,
                )
                .translate(
                    x + side * (WITNESS_COUPON_X / 2.0 + 5.0),
                    y,
                    WITNESS_Z / 2.0 + 5.0,
                );
        }
    }
    clips
}

fn witness_coupon_center(index: usize) -> (f64, f64) {
    let row = index / WITNESS_COLS;
    let col = index % WITNESS_COLS;
    (
        centered_index(col, WITNESS_COLS, WITNESS_PITCH_X),
        centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y),
    )
}

fn charge_probe_pocket() -> Part {
    let body = centered_cube(
        "static_charge_particle_charge_probe_pocket_body",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    );
    let pockets = charge_probe_recesses();
    let reference_plate = centered_cube(
        "static_charge_particle_charge_probe_reference_plate_land",
        PROBE_REFERENCE_PLATE_X,
        PROBE_REFERENCE_PLATE_Y,
        8.0,
    )
    .translate(0.0, PROBE_Y / 2.0 - 52.0, PROBE_Z / 2.0 + 4.0);
    let faraday_ring = rectangular_frame(
        "static_charge_particle_charge_probe_faraday_reference_guard",
        PROBE_REFERENCE_PLATE_X + 36.0,
        PROBE_REFERENCE_PLATE_Y + 32.0,
        8.0,
        7.0,
    )
    .translate(0.0, PROBE_Y / 2.0 - 52.0, PROBE_Z / 2.0 + 7.5);
    let cable_comb = charge_probe_cable_comb();
    let datum_pins = charge_probe_datum_pins();

    place_on_deck(
        body - pockets + reference_plate + faraday_ring + cable_comb + datum_pins,
        PROBE_CENTER,
        PROBE_Z,
    )
}

fn charge_probe_recesses() -> Part {
    let mut pockets = Part::empty("static_charge_particle_charge_probe_recesses");
    for i in 0..PROBE_POCKETS {
        let x = centered_index(i, PROBE_POCKETS, PROBE_POCKET_PITCH_X);
        let cylinder = centered_cylinder(
            format!("static_charge_particle_charge_probe_pocket_{i}"),
            PROBE_POCKET_D / 2.0,
            18.0,
            36,
        )
        .translate(x, -PROBE_Y / 2.0 + 62.0, PROBE_Z / 2.0 - 7.0);
        let handle_slot = centered_cube(
            format!("static_charge_particle_charge_probe_handle_relief_{i}"),
            18.0,
            64.0,
            20.0,
        )
        .translate(x, -PROBE_Y / 2.0 + 82.0, PROBE_Z / 2.0 - 4.0);
        pockets = pockets + cylinder + handle_slot;
    }
    pockets
}

fn charge_probe_cable_comb() -> Part {
    let comb = centered_cube(
        "static_charge_particle_charge_probe_cable_comb_body",
        PROBE_X - 56.0,
        18.0,
        18.0,
    )
    .translate(0.0, 0.0, PROBE_Z / 2.0 + 9.0);
    let mut cuts = Part::empty("static_charge_particle_charge_probe_cable_comb_slots");
    for i in 0..PROBE_CABLE_COMB_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("static_charge_particle_charge_probe_cable_comb_slot_{i}"),
                12.0,
                22.0,
                20.0,
            )
            .translate(
                centered_index(i, PROBE_CABLE_COMB_SLOTS, 44.0),
                0.0,
                PROBE_Z / 2.0 + 9.0,
            );
    }
    comb - cuts
}

fn charge_probe_datum_pins() -> Part {
    let mut pins = Part::empty("static_charge_particle_charge_probe_datum_pins");
    for (i, (x, y)) in [
        (-PROBE_X / 2.0 + 28.0, -PROBE_Y / 2.0 + 28.0),
        (PROBE_X / 2.0 - 28.0, -PROBE_Y / 2.0 + 28.0),
        (-PROBE_X / 2.0 + 28.0, PROBE_Y / 2.0 - 28.0),
        (PROBE_X / 2.0 - 28.0, PROBE_Y / 2.0 - 28.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("static_charge_particle_charge_probe_datum_pin_{i}"),
                5.0,
                8.0,
                28,
            )
            .translate(x, y, PROBE_Z / 2.0 + 4.0);
    }
    pins
}

fn grounded_ungrounded_comparison_lanes() -> Part {
    let body = centered_cube(
        "static_charge_particle_grounded_ungrounded_comparison_lane_body",
        COMPARISON_X,
        COMPARISON_Y,
        COMPARISON_Z,
    );
    let lane_recesses = comparison_station_recesses();
    let lane_rails = comparison_lane_rails();
    let ground_posts = grounded_lane_posts();
    let floating_posts = floating_lane_isolators();
    let readout_lands = comparison_readout_lands();

    place_on_deck(
        body - lane_recesses + lane_rails + ground_posts + floating_posts + readout_lands,
        COMPARISON_CENTER,
        COMPARISON_Z,
    )
}

fn comparison_station_recesses() -> Part {
    let mut recesses = Part::empty("static_charge_particle_comparison_station_recesses");
    for i in 0..COMPARISON_STATION_COUNT {
        let (x, y) = comparison_station_center(i);
        recesses = recesses
            + centered_cube(
                format!("static_charge_particle_comparison_station_{i:02}_coupon_recess"),
                COMPARISON_STATION_X,
                COMPARISON_STATION_Y,
                10.0,
            )
            .translate(x, y, COMPARISON_Z / 2.0 - 4.5);
    }
    recesses
}

fn comparison_lane_rails() -> Part {
    let mut rails = Part::empty("static_charge_particle_comparison_lane_rails");
    for lane in 0..COMPARISON_LANES {
        let y = centered_index(lane, COMPARISON_LANES, COMPARISON_LANE_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("static_charge_particle_comparison_lane_{lane}_rear_index_rail"),
                COMPARISON_X - 46.0,
                8.0,
                20.0,
            )
            .translate(
                0.0,
                y + COMPARISON_STATION_Y / 2.0 + 14.0,
                COMPARISON_Z / 2.0 + 10.0,
            )
            + centered_cube(
                format!("static_charge_particle_comparison_lane_{lane}_front_index_rail"),
                COMPARISON_X - 46.0,
                8.0,
                16.0,
            )
            .translate(
                0.0,
                y - COMPARISON_STATION_Y / 2.0 - 14.0,
                COMPARISON_Z / 2.0 + 8.0,
            );
    }
    let divider = centered_cube(
        "static_charge_particle_comparison_ground_float_separator",
        COMPARISON_X - 36.0,
        10.0,
        26.0,
    )
    .translate(0.0, 0.0, COMPARISON_Z / 2.0 + 13.0);
    rails + divider
}

fn grounded_lane_posts() -> Part {
    let mut posts = Part::empty("static_charge_particle_grounded_lane_strap_posts");
    for i in 0..GROUND_STRAP_POSTS {
        let x = centered_index(i, STATIONS_PER_COMPARISON_LANE, COMPARISON_STATION_PITCH_X);
        posts = posts
            + centered_cylinder(
                format!("static_charge_particle_grounded_lane_strap_post_{i}"),
                5.0,
                16.0,
                28,
            )
            .translate(
                x,
                COMPARISON_LANE_PITCH_Y / 2.0 + COMPARISON_STATION_Y / 2.0 + 32.0,
                COMPARISON_Z / 2.0 + 8.0,
            );
    }
    posts
}

fn floating_lane_isolators() -> Part {
    let mut posts = Part::empty("static_charge_particle_ungrounded_lane_isolator_posts");
    for i in 0..FLOATING_ISOLATOR_POSTS {
        let x = centered_index(i, STATIONS_PER_COMPARISON_LANE, COMPARISON_STATION_PITCH_X);
        posts = posts
            + centered_cube(
                format!("static_charge_particle_ungrounded_lane_isolator_post_{i}"),
                18.0,
                18.0,
                10.0,
            )
            .translate(
                x,
                -COMPARISON_LANE_PITCH_Y / 2.0 - COMPARISON_STATION_Y / 2.0 - 32.0,
                COMPARISON_Z / 2.0 + 5.0,
            );
    }
    posts
}

fn comparison_readout_lands() -> Part {
    let mut lands = Part::empty("static_charge_particle_comparison_readout_lands");
    for i in 0..COMPARISON_STATION_COUNT {
        let (x, y) = comparison_station_center(i);
        lands = lands
            + centered_cube(
                format!("static_charge_particle_comparison_station_{i:02}_readout_land"),
                46.0,
                14.0,
                4.0,
            )
            .translate(
                x,
                y + COMPARISON_STATION_Y / 2.0 + 12.0,
                COMPARISON_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn comparison_station_center(index: usize) -> (f64, f64) {
    let lane = index / STATIONS_PER_COMPARISON_LANE;
    let station = index % STATIONS_PER_COMPARISON_LANE;
    (
        centered_index(
            station,
            STATIONS_PER_COMPARISON_LANE,
            COMPARISON_STATION_PITCH_X,
        ),
        centered_index(lane, COMPARISON_LANES, COMPARISON_LANE_PITCH_Y),
    )
}

fn transparent_evidence_window_frame() -> Part {
    let frame = rectangular_frame(
        "static_charge_particle_transparent_evidence_window_frame",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_RAIL_W,
        EVIDENCE_Z,
    );
    let glass_land = rectangular_frame(
        "static_charge_particle_replaceable_transparent_window_glass_land",
        EVIDENCE_CLEAR_X + 48.0,
        EVIDENCE_CLEAR_Y + 28.0,
        6.0,
        6.0,
    )
    .translate(0.0, 0.0, EVIDENCE_Z / 2.0 + 3.0);
    let camera_pods = evidence_camera_pods();
    let led_segments = evidence_led_segments();
    let hinges = evidence_hinge_knuckles();
    let datum_pins = evidence_window_datum_pins();

    place_on_deck(
        frame + glass_land + camera_pods + led_segments + hinges + datum_pins,
        EVIDENCE_CENTER,
        EVIDENCE_Z,
    )
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("static_charge_particle_evidence_window_camera_pods");
    for i in 0..EVIDENCE_CAMERA_PODS {
        let x = centered_index(i, EVIDENCE_CAMERA_PODS, 228.0);
        let pod = centered_cube(
            format!("static_charge_particle_evidence_camera_pod_{i}"),
            82.0,
            46.0,
            36.0,
        )
        .translate(x, 0.0, EVIDENCE_Z / 2.0 + 18.0);
        let lens = centered_cylinder(
            format!("static_charge_particle_evidence_camera_lens_relief_{i}"),
            12.0,
            38.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, EVIDENCE_Z / 2.0 + 18.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_led_segments() -> Part {
    let mut segments = Part::empty("static_charge_particle_evidence_window_led_segments");
    for i in 0..EVIDENCE_LED_SEGMENTS {
        segments = segments
            + centered_cube(
                format!("static_charge_particle_evidence_window_led_segment_{i}"),
                64.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LED_SEGMENTS, 92.0),
                -EVIDENCE_Y / 2.0 + 12.0,
                EVIDENCE_Z / 2.0 + 2.5,
            );
    }
    segments
}

fn evidence_hinge_knuckles() -> Part {
    let mut hinges = Part::empty("static_charge_particle_evidence_window_hinge_knuckles");
    for i in 0..EVIDENCE_HINGE_KNUCKLES {
        hinges = hinges
            + centered_cylinder(
                format!("static_charge_particle_evidence_window_hinge_knuckle_{i}"),
                8.0,
                62.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                centered_index(i, EVIDENCE_HINGE_KNUCKLES, 166.0),
                EVIDENCE_Y / 2.0 + 7.0,
                EVIDENCE_Z / 2.0 + 8.0,
            );
    }
    hinges
}

fn evidence_window_datum_pins() -> Part {
    let mut pins = Part::empty("static_charge_particle_evidence_window_datum_pins");
    for (i, (x, y)) in [
        (-EVIDENCE_X / 2.0 + 46.0, -EVIDENCE_Y / 2.0 + 30.0),
        (EVIDENCE_X / 2.0 - 46.0, -EVIDENCE_Y / 2.0 + 30.0),
        (-EVIDENCE_X / 2.0 + 46.0, EVIDENCE_Y / 2.0 - 30.0),
        (EVIDENCE_X / 2.0 - 46.0, EVIDENCE_Y / 2.0 - 30.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("static_charge_particle_evidence_window_datum_pin_{i}"),
                4.5,
                8.0,
                28,
            )
            .translate(x, y, EVIDENCE_Z / 2.0 + 4.0);
    }
    pins
}

fn barcode_custody_labels() -> Part {
    let panel = centered_cube(
        "static_charge_particle_barcode_custody_label_panel",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    );
    let barcode_lands = barcode_label_lands();
    let custody_lands = custody_label_lands();
    let lot_cards = lot_card_lands();
    let evidence_seal_posts = custody_seal_posts();

    place_on_deck(
        panel + barcode_lands + custody_lands + lot_cards + evidence_seal_posts,
        LABEL_CENTER,
        LABEL_Z,
    )
}

fn barcode_label_lands() -> Part {
    let mut lands = Part::empty("static_charge_particle_barcode_label_lands");
    for i in 0..BARCODE_LABELS {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(
                format!("static_charge_particle_barcode_label_land_{i}"),
                58.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 68.0),
                LABEL_Y / 2.0 - 24.0 - row as f64 * 22.0,
                LABEL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_label_lands() -> Part {
    let mut lands = Part::empty("static_charge_particle_custody_label_lands");
    for i in 0..CUSTODY_LABELS {
        lands = lands
            + centered_cube(
                format!("static_charge_particle_custody_label_land_{i}"),
                62.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(i, CUSTODY_LABELS, 72.0),
                -18.0,
                LABEL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn lot_card_lands() -> Part {
    let mut cards = Part::empty("static_charge_particle_lot_card_lands");
    for i in 0..LOT_CARD_LANDS {
        cards = cards
            + centered_cube(
                format!("static_charge_particle_particle_lot_certificate_card_land_{i}"),
                82.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, LOT_CARD_LANDS, 96.0),
                -43.0,
                LABEL_Z / 2.0 + 2.5,
            );
    }
    cards
}

fn custody_seal_posts() -> Part {
    let mut posts = Part::empty("static_charge_particle_custody_seal_posts");
    for i in 0..6 {
        posts = posts
            + centered_cylinder(
                format!("static_charge_particle_custody_wire_seal_post_{i}"),
                4.0,
                14.0,
                24,
            )
            .translate(centered_index(i, 6, 48.0), 0.0, LABEL_Z / 2.0 + 7.0);
    }
    posts
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "static_charge_particle_release_hold_reject_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let slot_recesses = status_lane_recesses();
    let lane_dividers = status_lane_dividers();
    let lane_labels = status_lane_label_lands();
    let quarantine_wall = centered_cube(
        "static_charge_particle_reject_lane_high_quarantine_wall",
        STATUS_X - 36.0,
        8.0,
        58.0,
    )
    .translate(0.0, -STATUS_LANE_PITCH_Y, STATUS_Z / 2.0 + 29.0);

    place_on_deck(
        body - slot_recesses + lane_dividers + lane_labels + quarantine_wall,
        STATUS_CENTER,
        STATUS_Z,
    )
}

fn status_lane_recesses() -> Part {
    let mut recesses = Part::empty("static_charge_particle_release_hold_reject_slot_recesses");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            recesses = recesses
                + centered_cube(
                    format!("static_charge_particle_status_lane_{lane}_slot_{slot}_recess"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    10.0,
                )
                .translate(
                    centered_index(slot, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_X),
                    centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y),
                    STATUS_Z / 2.0 - 4.5,
                );
        }
    }
    recesses
}

fn status_lane_dividers() -> Part {
    let upper = centered_cube(
        "static_charge_particle_release_hold_lane_divider",
        STATUS_X - 28.0,
        6.0,
        22.0,
    )
    .translate(0.0, STATUS_LANE_PITCH_Y / 2.0, STATUS_Z / 2.0 + 11.0);
    let lower = centered_cube(
        "static_charge_particle_hold_reject_lane_divider",
        STATUS_X - 28.0,
        6.0,
        24.0,
    )
    .translate(0.0, -STATUS_LANE_PITCH_Y / 2.0, STATUS_Z / 2.0 + 12.0);

    upper + lower
}

fn status_lane_label_lands() -> Part {
    let mut labels = Part::empty("static_charge_particle_status_lane_label_lands");
    for lane in 0..STATUS_LANES {
        labels = labels
            + centered_cube(
                format!("static_charge_particle_status_lane_{lane}_label_land"),
                96.0,
                14.0,
                4.0,
            )
            .translate(
                -STATUS_X / 2.0 + 64.0,
                centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y),
                STATUS_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "static_charge_particle_front_robot_approach_keepout_gauge",
        DECK_X - 210.0,
        26.0,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE_MM,
        deck_top_z(KEEP_OUT_Z) + 4.0,
    );
    let rear_service = centered_cube(
        "static_charge_particle_rear_service_access_keepout_gauge",
        DECK_X - 240.0,
        22.0,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE_MM,
        deck_top_z(KEEP_OUT_Z) + 4.0,
    );
    let ionizer_side = centered_cube(
        "static_charge_particle_ionizer_side_service_keepout_gauge",
        22.0,
        IONIZER_SERVICE_CLEARANCE_MM,
        8.0,
    )
    .translate(
        DECK_X / 2.0 - 138.0,
        IONIZER_CENTER.1,
        deck_top_z(KEEP_OUT_Z) + 4.0,
    );
    let probe_pull = centered_cube(
        "static_charge_particle_charge_probe_pull_keepout_gauge",
        CHARGE_PROBE_PULL_CLEARANCE_MM,
        22.0,
        8.0,
    )
    .translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1 - 142.0,
        deck_top_z(KEEP_OUT_Z) + 4.0,
    );
    let z_bridge = rectangular_frame(
        "static_charge_particle_robot_z_clearance_bridge_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        10.0,
        8.0,
    )
    .translate(0.0, 0.0, DECK_Z + ROBOT_PICK_CLEARANCE_Z_MM);

    front_robot + rear_service + ionizer_side + probe_pull + z_bridge
}

fn rectangular_frame(name: &str, x: f64, y: f64, rail: f64, z: f64) -> Part {
    centered_cube(format!("{name}_outer"), x, y, z)
        - centered_cube(
            format!("{name}_clear_opening"),
            x - 2.0 * rail,
            y - 2.0 * rail,
            z + 2.0,
        )
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 8.0, 4.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 2.2, 5.0, 24)
}
