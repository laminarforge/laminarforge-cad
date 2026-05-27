use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste-container vent-filter condensate breakthrough validation station.
//
// Design intent:
// - Hold a closed waste bottle or collapsible waste bag in a secondary
//   containment nest while the vent path remains mechanically segregated from
//   clean-side service tooling.
// - Fixture a hydrophobic vent-filter cartridge, condensate challenge channel,
//   breakthrough witness windows, backpressure sensing, overflow/foam sensing,
//   retained samples, custody/barcode lands, evidence capture, and robot
//   service keepout gauges into one validation station.
// - Model bought-in filters, sensors, tubing, containers, readers, and cameras
//   as service envelopes and datum geometry. This CAD is not a pressure-rated
//   waste container, aerosol protocol, sterile barrier, or release criterion.

const OUTPUT_PREFIX: &str = "closed_waste_container_vent_filter_condensate_breakthrough_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_secondary_containment_deck.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_waste_bottle_bag_nest.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_hydrophobic_vent_filter_cartridge_dock.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_condensate_challenge_channel.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_breakthrough_witness_windows.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_backpressure_sensor_pockets.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_overflow_foam_sensor_brackets.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_sample_custody_wells.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_barcode_custody_lands.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_clean_dirty_segregation_barriers.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_evidence_bridge.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_robot_service_keepout_gauges.stl",
    "output/closed_waste_container_vent_filter_condensate_breakthrough_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "waste_bottle_bag_nest",
    "hydrophobic_vent_filter_cartridge_dock",
    "condensate_challenge_channel",
    "breakthrough_witness_windows",
    "backpressure_sensor_pockets",
    "overflow_foam_sensor_brackets",
    "sample_custody_wells",
    "barcode_custody_lands",
    "clean_dirty_segregation_barriers",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "assembly",
];

const STATION_X: f64 = 1520.0;
const STATION_Y: f64 = 940.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 58.0;
const BASIN_RECESS_Z: f64 = 10.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLES: usize = 8;
const DATUM_TARGETS: usize = 4;
const LEAK_SENSOR_WELLS: usize = 6;
const DRAIN_PORT_D: f64 = 18.0;

const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 320.0;
const NEST_Z: f64 = 96.0;
const NEST_POS: (f64, f64) = (-420.0, 170.0);
const BOTTLE_SOCKET_D: f64 = 176.0;
const BAG_TROUGH_X: f64 = 172.0;
const BAG_TROUGH_Y: f64 = 248.0;
const CONTAINER_STRAPS: usize = 3;
const BAG_CLAMP_FINGERS: usize = 5;
const NEST_LOCATOR_PINS: usize = 4;

const FILTER_DOCK_X: f64 = 420.0;
const FILTER_DOCK_Y: f64 = 150.0;
const FILTER_DOCK_Z: f64 = 70.0;
const FILTER_DOCK_POS: (f64, f64) = (260.0, 300.0);
const FILTER_OD: f64 = 50.0;
const FILTER_LENGTH: f64 = 230.0;
const FILTER_CLAMPS: usize = 4;
const FILTER_END_BULKHEADS: usize = 2;

const CHALLENGE_X: f64 = 520.0;
const CHALLENGE_Y: f64 = 170.0;
const CHALLENGE_Z: f64 = 38.0;
const CHALLENGE_POS: (f64, f64) = (-250.0, -85.0);
const CHALLENGE_LANES: usize = 4;
const CHALLENGE_WEIRS: usize = 7;
const CHALLENGE_PORTS: usize = 4;
const CHALLENGE_LANE_PITCH_Y: f64 = 34.0;

const WITNESS_X: f64 = 500.0;
const WITNESS_Y: f64 = 180.0;
const WITNESS_Z: f64 = 50.0;
const WITNESS_POS: (f64, f64) = (340.0, 65.0);
const WITNESS_WINDOWS: usize = 8;
const WITNESS_WINDOW_X: f64 = 44.0;
const WITNESS_WINDOW_Y: f64 = 72.0;
const WITNESS_PITCH_X: f64 = 56.0;
const WITNESS_COUPON_CLIPS: usize = WITNESS_WINDOWS * 2;

const PRESSURE_X: f64 = 330.0;
const PRESSURE_Y: f64 = 160.0;
const PRESSURE_Z: f64 = 54.0;
const PRESSURE_POS: (f64, f64) = (520.0, -145.0);
const BACKPRESSURE_SENSORS: usize = 4;
const SENSOR_POCKET_X: f64 = 58.0;
const SENSOR_POCKET_Y: f64 = 52.0;
const SENSOR_POCKET_Z: f64 = 22.0;
const PRESSURE_TAPS: usize = 5;

const FOAM_X: f64 = 330.0;
const FOAM_Y: f64 = 170.0;
const FOAM_Z: f64 = 136.0;
const FOAM_POS: (f64, f64) = (-500.0, -335.0);
const OVERFLOW_LEVELS: usize = 3;
const FOAM_SENSOR_FORKS: usize = 4;
const FOAM_RISER_D: f64 = 34.0;

const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 36.0;
const CUSTODY_POS: (f64, f64) = (80.0, -330.0);
const SAMPLE_WELLS: usize = 12;
const SAMPLE_WELL_D: f64 = 27.0;
const SAMPLE_WELL_PITCH_X: f64 = 58.0;
const SAMPLE_WELL_PITCH_Y: f64 = 64.0;
const CUSTODY_SEAL_POSTS: usize = 4;

const BARCODE_X: f64 = 340.0;
const BARCODE_Y: f64 = 110.0;
const BARCODE_Z: f64 = 12.0;
const BARCODE_POS: (f64, f64) = (525.0, -360.0);
const BARCODE_LANDS: usize = 16;
const BARCODE_COLS: usize = 4;
const BARCODE_ROWS: usize = 4;
const BARCODE_LAND_X: f64 = 62.0;
const BARCODE_LAND_Y: f64 = 20.0;
const TAMPER_SEAL_LANDS: usize = 6;

const BARRIER_WALL_Z: f64 = 92.0;
const BARRIER_GATE_COUNT: usize = 3;
const DIRTY_SIDE_LABEL_LANDS: usize = 5;
const CLEAN_SIDE_LABEL_LANDS: usize = 5;

const EVIDENCE_SPAN_X: f64 = 1320.0;
const EVIDENCE_POST_Z: f64 = 244.0;
const EVIDENCE_BEAM_Z: f64 = 32.0;
const EVIDENCE_POST_X: f64 = 30.0;
const EVIDENCE_POST_Y: f64 = 44.0;
const EVIDENCE_CAMERAS: usize = 5;
const EVIDENCE_LIGHT_BARS: usize = 2;
const EVIDENCE_POS: (f64, f64) = (0.0, 38.0);

const KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 380.0;
const REAR_FILTER_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_WASTE_CONTAINER_SWAP_CLEARANCE: f64 = 280.0;
const RIGHT_SENSOR_CAL_CLEARANCE: f64 = 220.0;
const TOP_FILTER_LIFT_CLEARANCE: f64 = 320.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 16.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 16.0;

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
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = waste_bottle_bag_nest();
    export(OUTPUTS[1], &nest);

    let filter = hydrophobic_vent_filter_cartridge_dock();
    export(OUTPUTS[2], &filter);

    let challenge = condensate_challenge_channel();
    export(OUTPUTS[3], &challenge);

    let windows = breakthrough_witness_windows();
    export(OUTPUTS[4], &windows);

    let pressure = backpressure_sensor_pockets();
    export(OUTPUTS[5], &pressure);

    let foam = overflow_foam_sensor_brackets();
    export(OUTPUTS[6], &foam);

    let custody = sample_custody_wells();
    export(OUTPUTS[7], &custody);

    let barcodes = barcode_custody_lands();
    export(OUTPUTS[8], &barcodes);

    let barriers = clean_dirty_segregation_barriers();
    export(OUTPUTS[9], &barriers);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + nest.translate(NEST_POS.0, NEST_POS.1, deck_insert_z())
        + filter.translate(FILTER_DOCK_POS.0, FILTER_DOCK_POS.1, deck_insert_z())
        + challenge.translate(CHALLENGE_POS.0, CHALLENGE_POS.1, deck_insert_z())
        + windows.translate(WITNESS_POS.0, WITNESS_POS.1, deck_insert_z())
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, deck_insert_z())
        + foam.translate(FOAM_POS.0, FOAM_POS.1, deck_insert_z())
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, deck_insert_z())
        + barcodes.translate(BARCODE_POS.0, BARCODE_POS.1, deck_insert_z())
        + barriers.translate(0.0, 0.0, DECK_Z)
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed waste-container vent-filter condensate breakthrough station:");
    println!(
        "  Footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm secondary containment deck with {:.0}mL modeled freeboard",
        secondary_containment_freeboard_ml()
    );
    println!(
        "  Waste nest:         {BOTTLE_SOCKET_D:.0}mm bottle socket plus {BAG_TROUGH_X:.0}mm x {BAG_TROUGH_Y:.0}mm bag trough, {CONTAINER_STRAPS} strap bridges, {BAG_CLAMP_FINGERS} bag clamp fingers"
    );
    println!(
        "  Vent challenge:     {FILTER_CLAMPS} hydrophobic filter clamps, {CHALLENGE_LANES} condensate lanes, {CHALLENGE_WEIRS} weirs, {WITNESS_WINDOWS} breakthrough witness windows"
    );
    println!(
        "  Sensing:            {BACKPRESSURE_SENSORS} backpressure pockets, {PRESSURE_TAPS} pressure taps, {OVERFLOW_LEVELS} overflow levels, {FOAM_SENSOR_FORKS} foam fork brackets"
    );
    println!(
        "  Custody/evidence:   {SAMPLE_WELLS} sample custody wells, {BARCODE_LANDS} barcode lands, {TAMPER_SEAL_LANDS} tamper lands, {EVIDENCE_CAMERAS} camera mounts, {EVIDENCE_LIGHT_BARS} light bars"
    );
    println!(
        "  Keepouts:           {KEEP_OUT_GAUGES} gauges, {FRONT_ROBOT_CLEARANCE:.0}mm front robot, {REAR_FILTER_SERVICE_CLEARANCE:.0}mm rear filter service, {LEFT_WASTE_CONTAINER_SWAP_CLEARANCE:.0}mm waste swap, {RIGHT_SENSOR_CAL_CLEARANCE:.0}mm sensor calibration, {TOP_FILTER_LIFT_CLEARANCE:.0}mm filter lift"
    );
    println!(
        "  Required features:  {} explicit feature groups; no aerosol protocol or release rule encoded.",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z() -> f64 {
    DECK_Z - SOCKET_DEPTH + 0.6
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(BARCODE_LANDS, BARCODE_COLS * BARCODE_ROWS);
    assert_eq!(WITNESS_COUPON_CLIPS, WITNESS_WINDOWS * 2);
    assert_eq!(FILTER_END_BULKHEADS, 2);
    assert_eq!(NEST_LOCATOR_PINS, 4);
    assert_eq!(CUSTODY_SEAL_POSTS, 4);
    assert_eq!(MOUNT_HOLES, mount_hole_positions().len());
    assert_eq!(DATUM_TARGETS, datum_positions().len());
    assert!(secondary_containment_freeboard_ml() > maximum_condensate_challenge_ml());
    assert!(FILTER_LENGTH > FILTER_OD * 4.0);
    assert!(SENSOR_POCKET_Z < PRESSURE_Z);
    assert!(TOP_FILTER_LIFT_CLEARANCE > EVIDENCE_POST_Z);

    let rects = socket_rects();
    for item in rects {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds station envelope",
            item.name
        );
    }

    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn socket_rects() -> [Rect; 8] {
    [
        rect("waste_bottle_bag_nest", NEST_POS, NEST_X, NEST_Y),
        rect(
            "hydrophobic_vent_filter_cartridge_dock",
            FILTER_DOCK_POS,
            FILTER_DOCK_X,
            FILTER_DOCK_Y,
        ),
        rect(
            "condensate_challenge_channel",
            CHALLENGE_POS,
            CHALLENGE_X,
            CHALLENGE_Y,
        ),
        rect(
            "breakthrough_witness_windows",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "backpressure_sensor_pockets",
            PRESSURE_POS,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        rect("overflow_foam_sensor_brackets", FOAM_POS, FOAM_X, FOAM_Y),
        rect("sample_custody_wells", CUSTODY_POS, CUSTODY_X, CUSTODY_Y),
        rect("barcode_custody_lands", BARCODE_POS, BARCODE_X, BARCODE_Y),
    ]
}

fn secondary_containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    inner_x * inner_y * (RIM_Z - BASIN_RECESS_Z) / 1000.0
}

fn maximum_condensate_challenge_ml() -> f64 {
    let challenge = CHALLENGE_LANES as f64 * 42.0;
    let witness = WITNESS_WINDOWS as f64 * 14.0;
    let samples = SAMPLE_WELLS as f64 * 8.0;
    let filter_sump = 110.0;

    challenge + witness + samples + filter_sump
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        "waste_vent_breakthrough_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin_recess = centered_cube(
        "waste_vent_breakthrough_secondary_basin_recess",
        STATION_X - 2.0 * (RIM_W + 48.0),
        STATION_Y - 2.0 * (RIM_W + 46.0),
        BASIN_RECESS_Z + 0.8,
    )
    .translate(0.0, -5.0, DECK_Z - BASIN_RECESS_Z / 2.0);
    let drain = centered_cylinder(
        "waste_vent_breakthrough_low_point_closed_drain_bore",
        DRAIN_PORT_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 96.0,
        -STATION_Y / 2.0 + 26.0,
        DECK_Z - 8.0,
    );

    deck - basin_recess - drain - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + clean_dirty_base_spines()
        + leak_sensor_recess_markers()
        + datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("waste_vent_breakthrough_insert_registration_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("waste_vent_breakthrough_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("waste_vent_breakthrough_m6_mount_holes");
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("waste_vent_breakthrough_m6_clearance_round_{i}"),
                3.4,
                DECK_Z + 6.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0)
            + centered_cube(
                format!("waste_vent_breakthrough_m6_slot_relief_{i}"),
                32.0,
                7.8,
                DECK_Z + 6.0,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (-STATION_X / 2.0 + 62.0, 0.0),
        (STATION_X / 2.0 - 62.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "waste_vent_breakthrough_front_low_robot_spill_lip",
        STATION_X,
        RIM_W,
        34.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 17.0);
    let rear = centered_cube(
        "waste_vent_breakthrough_rear_filter_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "waste_vent_breakthrough_left_dirty_waste_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "waste_vent_breakthrough_right_clean_sensor_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear_filter_notch = centered_cube(
        "waste_vent_breakthrough_rear_filter_cartridge_service_notch",
        300.0,
        RIM_W + 3.0,
        24.0,
    )
    .translate(260.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z - 12.0);

    front + rear + left + right - rear_filter_notch
}

fn clean_dirty_base_spines() -> Part {
    let center = centered_cube(
        "waste_vent_breakthrough_clean_dirty_centerline_rib",
        10.0,
        STATION_Y - 150.0,
        22.0,
    )
    .translate(0.0, 0.0, DECK_Z + 11.0);
    let challenge_row = centered_cube(
        "waste_vent_breakthrough_challenge_to_witness_flow_spine",
        STATION_X - 230.0,
        8.0,
        20.0,
    )
    .translate(0.0, -210.0, DECK_Z + 10.0);
    let filter_row = centered_cube(
        "waste_vent_breakthrough_filter_witness_service_spine",
        STATION_X - 230.0,
        8.0,
        20.0,
    )
    .translate(0.0, 198.0, DECK_Z + 10.0);
    let custody_row = centered_cube(
        "waste_vent_breakthrough_sample_custody_row_spine",
        STATION_X - 270.0,
        8.0,
        18.0,
    )
    .translate(0.0, -286.0, DECK_Z + 9.0);

    center + challenge_row + filter_row + custody_row
}

fn leak_sensor_recess_markers() -> Part {
    let mut wells = Part::empty("waste_vent_breakthrough_leak_sensor_recess_markers");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 130.0);
        let y = if i % 2 == 0 {
            -STATION_Y / 2.0 + 108.0
        } else {
            STATION_Y / 2.0 - 110.0
        };
        wells = wells
            + shallow_ring(
                &format!("waste_vent_breakthrough_leak_sensor_recess_{i}"),
                34.0,
                20.0,
                5.0,
                28,
            )
            .translate(x, y, DECK_Z + 2.5);
    }
    wells
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("waste_vent_breakthrough_robot_datum_targets");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        targets =
            targets
                + fiducial_disc(&format!("waste_vent_breakthrough_deck_datum_target_{i}"))
                    .translate(*x, *y, DECK_Z + 3.0);
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGETS] {
    [
        (-STATION_X / 2.0 + 106.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 106.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 106.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 106.0, STATION_Y / 2.0 - 104.0),
    ]
}

fn waste_bottle_bag_nest() -> Part {
    let tray = centered_cube(
        "waste_vent_breakthrough_waste_bottle_bag_nest_tray",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let bottle_socket = centered_cylinder(
        "waste_vent_breakthrough_round_waste_bottle_socket_cut",
        BOTTLE_SOCKET_D / 2.0,
        NEST_Z + 8.0,
        72,
    )
    .translate(-92.0, 24.0, NEST_Z / 2.0);
    let bag_trough = centered_cube(
        "waste_vent_breakthrough_collapsible_bag_trough_cut",
        BAG_TROUGH_X,
        BAG_TROUGH_Y,
        44.0,
    )
    .translate(112.0, 0.0, NEST_Z - 21.0);
    let drain_scallop = centered_cylinder(
        "waste_vent_breakthrough_nest_low_point_drain_scallop",
        15.0,
        NEST_X + 20.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -NEST_Y / 2.0 + 34.0, 25.0);

    tray - bottle_socket - bag_trough - drain_scallop
        + bottle_bag_side_rails()
        + container_strap_bridges()
        + bag_clamp_finger_bank()
        + nest_locator_pins()
        + container_shadow_envelopes()
}

fn bottle_bag_side_rails() -> Part {
    let left_rail = centered_cube(
        "waste_vent_breakthrough_bottle_hard_datum_side_rail",
        18.0,
        NEST_Y - 70.0,
        78.0,
    )
    .translate(-205.0, 4.0, NEST_Z + 39.0);
    let bottle_rear_stop = centered_cube(
        "waste_vent_breakthrough_bottle_rear_neck_stop",
        210.0,
        18.0,
        88.0,
    )
    .translate(-92.0, 142.0, NEST_Z + 44.0);
    let bag_left = centered_cube(
        "waste_vent_breakthrough_bag_left_pressure_edge_rail",
        18.0,
        BAG_TROUGH_Y + 10.0,
        54.0,
    )
    .translate(18.0, 0.0, NEST_Z + 27.0);
    let bag_right = centered_cube(
        "waste_vent_breakthrough_bag_right_pressure_edge_rail",
        18.0,
        BAG_TROUGH_Y + 10.0,
        54.0,
    )
    .translate(204.0, 0.0, NEST_Z + 27.0);
    let bag_front_stop = centered_cube(
        "waste_vent_breakthrough_bag_front_low_stop",
        BAG_TROUGH_X + 70.0,
        16.0,
        42.0,
    )
    .translate(112.0, -142.0, NEST_Z + 21.0);

    left_rail + bottle_rear_stop + bag_left + bag_right + bag_front_stop
}

fn container_strap_bridges() -> Part {
    let mut straps = Part::empty("waste_vent_breakthrough_container_strap_bridges");
    for i in 0..CONTAINER_STRAPS {
        let y = centered_index(i, CONTAINER_STRAPS, 88.0) - 2.0;
        let bridge = centered_cube(
            format!("waste_vent_breakthrough_bottle_bag_strap_bridge_{i}"),
            NEST_X - 72.0,
            10.0,
            16.0,
        )
        .translate(0.0, y, NEST_Z + 66.0);
        let latch = centered_cube(
            format!("waste_vent_breakthrough_strap_latch_boss_{i}"),
            26.0,
            24.0,
            24.0,
        )
        .translate(NEST_X / 2.0 - 42.0, y, NEST_Z + 64.0);
        straps = straps + bridge + latch;
    }
    straps
}

fn bag_clamp_finger_bank() -> Part {
    let mut fingers = Part::empty("waste_vent_breakthrough_bag_clamp_fingers");
    for i in 0..BAG_CLAMP_FINGERS {
        let y = centered_index(i, BAG_CLAMP_FINGERS, 43.0);
        fingers = fingers
            + centered_cube(
                format!("waste_vent_breakthrough_bag_clamp_finger_{i}"),
                78.0,
                8.0,
                20.0,
            )
            .translate(112.0, y, NEST_Z + 58.0)
            + centered_cylinder(
                format!("waste_vent_breakthrough_bag_clamp_hinge_pin_{i}"),
                4.0,
                102.0,
                18,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(112.0, y, NEST_Z + 45.0);
    }
    fingers
}

fn nest_locator_pins() -> Part {
    let mut pins = Part::empty("waste_vent_breakthrough_nest_locator_pins");
    for (i, (x, y)) in [
        (-175.0, -126.0),
        (-175.0, 126.0),
        (175.0, -126.0),
        (175.0, 126.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("waste_vent_breakthrough_nest_locator_pin_{i}"),
                8.0,
                18.0,
                24,
            )
            .translate(*x, *y, NEST_Z + 9.0);
    }
    pins
}

fn container_shadow_envelopes() -> Part {
    let bottle = shallow_ring(
        "waste_vent_breakthrough_closed_bottle_shadow_envelope",
        BOTTLE_SOCKET_D + 32.0,
        BOTTLE_SOCKET_D + 8.0,
        12.0,
        80,
    )
    .translate(-92.0, 24.0, NEST_Z + 122.0);
    let bag = centered_cube(
        "waste_vent_breakthrough_collapsible_bag_shadow_envelope",
        BAG_TROUGH_X + 34.0,
        BAG_TROUGH_Y + 22.0,
        10.0,
    )
    .translate(112.0, 0.0, NEST_Z + 104.0);
    let neck = vertical_ring(
        "waste_vent_breakthrough_bottle_neck_capture_ring",
        76.0,
        48.0,
        22.0,
        48,
    )
    .translate(-92.0, 112.0, NEST_Z + 98.0);

    bottle + bag + neck
}

fn hydrophobic_vent_filter_cartridge_dock() -> Part {
    let base = centered_cube(
        "waste_vent_breakthrough_filter_cartridge_dock_base",
        FILTER_DOCK_X,
        FILTER_DOCK_Y,
        FILTER_DOCK_Z,
    )
    .translate(0.0, 0.0, FILTER_DOCK_Z / 2.0);
    let saddle_cut = centered_cylinder(
        "waste_vent_breakthrough_filter_cylindrical_saddle_cut",
        FILTER_OD / 2.0 + 3.0,
        FILTER_LENGTH + 44.0,
        64,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, FILTER_DOCK_Z - 13.0);
    let condensate_sump = centered_cube(
        "waste_vent_breakthrough_filter_dock_condensate_sump_cut",
        FILTER_LENGTH + 86.0,
        34.0,
        16.0,
    )
    .translate(0.0, -FILTER_DOCK_Y / 2.0 + 34.0, FILTER_DOCK_Z - 8.0);

    base - saddle_cut - condensate_sump
        + filter_clamp_bridges()
        + filter_end_bulkheads()
        + filter_condensate_drip_lands()
        + filter_orientation_keys()
}

fn filter_clamp_bridges() -> Part {
    let mut clamps = Part::empty("waste_vent_breakthrough_filter_clamp_bridges");
    for i in 0..FILTER_CLAMPS {
        let x = centered_index(
            i,
            FILTER_CLAMPS,
            FILTER_LENGTH / (FILTER_CLAMPS as f64 - 1.0),
        );
        let strap = centered_cube(
            format!("waste_vent_breakthrough_filter_overstrap_bridge_{i}"),
            22.0,
            FILTER_OD + 34.0,
            16.0,
        )
        .translate(x, 0.0, FILTER_DOCK_Z + 36.0);
        let thumb_screw = centered_cylinder(
            format!("waste_vent_breakthrough_filter_thumb_screw_boss_{i}"),
            8.0,
            12.0,
            24,
        )
        .translate(x, FILTER_OD / 2.0 + 30.0, FILTER_DOCK_Z + 52.0);
        clamps = clamps + strap + thumb_screw;
    }
    clamps
}

fn filter_end_bulkheads() -> Part {
    let inlet = tube_bulkhead(
        "waste_vent_breakthrough_filter_dirty_inlet_bulkhead",
        -FILTER_LENGTH / 2.0 - 36.0,
        0.0,
        FILTER_DOCK_Z - 2.0,
        26.0,
    );
    let outlet = tube_bulkhead(
        "waste_vent_breakthrough_filter_clean_outlet_bulkhead",
        FILTER_LENGTH / 2.0 + 36.0,
        0.0,
        FILTER_DOCK_Z - 2.0,
        26.0,
    );
    let inlet_key = centered_cube(
        "waste_vent_breakthrough_filter_dirty_side_triangle_key_surrogate",
        28.0,
        10.0,
        22.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-FILTER_LENGTH / 2.0 - 70.0, -42.0, FILTER_DOCK_Z + 16.0);
    let outlet_key = centered_cube(
        "waste_vent_breakthrough_filter_clean_side_square_key_surrogate",
        26.0,
        26.0,
        18.0,
    )
    .translate(FILTER_LENGTH / 2.0 + 70.0, 42.0, FILTER_DOCK_Z + 14.0);

    inlet + outlet + inlet_key + outlet_key
}

fn filter_condensate_drip_lands() -> Part {
    let mut lands = Part::empty("waste_vent_breakthrough_filter_condensate_drip_lands");
    for i in 0..CHALLENGE_PORTS {
        let x = centered_index(i, CHALLENGE_PORTS, 62.0);
        lands = lands
            + vertical_ring(
                &format!("waste_vent_breakthrough_filter_drip_cup_land_{i}"),
                36.0,
                19.0,
                8.0,
                32,
            )
            .translate(x, -FILTER_DOCK_Y / 2.0 + 28.0, FILTER_DOCK_Z + 4.0);
    }
    lands
}

fn filter_orientation_keys() -> Part {
    let arrow_stem = centered_cube(
        "waste_vent_breakthrough_filter_flow_arrow_stem",
        FILTER_LENGTH - 38.0,
        8.0,
        8.0,
    )
    .translate(0.0, FILTER_DOCK_Y / 2.0 - 28.0, FILTER_DOCK_Z + 8.0);
    let arrow_head = centered_cube(
        "waste_vent_breakthrough_filter_flow_arrow_head",
        34.0,
        34.0,
        8.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        FILTER_LENGTH / 2.0 - 18.0,
        FILTER_DOCK_Y / 2.0 - 28.0,
        FILTER_DOCK_Z + 8.0,
    );
    let cartridge_shadow = centered_cylinder(
        "waste_vent_breakthrough_filter_cartridge_service_envelope",
        FILTER_OD / 2.0,
        FILTER_LENGTH,
        72,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, FILTER_DOCK_Z + FILTER_OD / 2.0 + 22.0);

    arrow_stem + arrow_head + cartridge_shadow
}

fn condensate_challenge_channel() -> Part {
    let plate = centered_cube(
        "waste_vent_breakthrough_condensate_challenge_channel_plate",
        CHALLENGE_X,
        CHALLENGE_Y,
        CHALLENGE_Z,
    )
    .translate(0.0, 0.0, CHALLENGE_Z / 2.0);

    plate - challenge_lane_cuts() - challenge_port_cuts()
        + challenge_weirs()
        + channel_metering_ticks()
        + channel_end_collection_sumps()
}

fn challenge_lane_cuts() -> Part {
    let mut lanes = Part::empty("waste_vent_breakthrough_condensate_lane_cuts");
    for i in 0..CHALLENGE_LANES {
        let y = centered_index(i, CHALLENGE_LANES, CHALLENGE_LANE_PITCH_Y);
        lanes = lanes
            + centered_cube(
                format!("waste_vent_breakthrough_condensate_lane_cut_{i}"),
                CHALLENGE_X - 76.0,
                16.0,
                13.0,
            )
            .translate(8.0, y, CHALLENGE_Z - 6.0);
    }
    lanes
}

fn challenge_port_cuts() -> Part {
    let mut ports = Part::empty("waste_vent_breakthrough_condensate_challenge_port_cuts");
    for i in 0..CHALLENGE_PORTS {
        let y = centered_index(i, CHALLENGE_PORTS, CHALLENGE_LANE_PITCH_Y);
        ports = ports
            + centered_cylinder(
                format!("waste_vent_breakthrough_condensate_inlet_port_cut_{i}"),
                9.0,
                CHALLENGE_Z + 4.0,
                24,
            )
            .translate(-CHALLENGE_X / 2.0 + 40.0, y, CHALLENGE_Z / 2.0);
    }
    ports
}

fn challenge_weirs() -> Part {
    let mut weirs = Part::empty("waste_vent_breakthrough_condensate_challenge_weirs");
    for i in 0..CHALLENGE_WEIRS {
        let x = centered_index(i, CHALLENGE_WEIRS, 56.0) + 28.0;
        let height = 9.0 + (i % 3) as f64 * 3.0;
        weirs = weirs
            + centered_cube(
                format!("waste_vent_breakthrough_condensate_metering_weir_{i}"),
                8.0,
                CHALLENGE_Y - 54.0,
                height,
            )
            .translate(x, 0.0, CHALLENGE_Z + height / 2.0);
    }
    weirs
}

fn channel_metering_ticks() -> Part {
    let mut ticks = Part::empty("waste_vent_breakthrough_condensate_metering_ticks");
    for i in 0..12 {
        let x = centered_index(i, 12, 34.0);
        ticks = ticks
            + centered_cube(
                format!("waste_vent_breakthrough_condensate_volume_tick_{i}"),
                3.0,
                24.0,
                5.0,
            )
            .translate(x, CHALLENGE_Y / 2.0 - 22.0, CHALLENGE_Z + 2.5);
    }
    ticks
}

fn channel_end_collection_sumps() -> Part {
    let dirty_start = vertical_ring(
        "waste_vent_breakthrough_condensate_dirty_start_cup",
        46.0,
        30.0,
        12.0,
        36,
    )
    .translate(
        -CHALLENGE_X / 2.0 + 42.0,
        -CHALLENGE_Y / 2.0 + 26.0,
        CHALLENGE_Z + 6.0,
    );
    let clean_end = vertical_ring(
        "waste_vent_breakthrough_condensate_clean_end_cup",
        52.0,
        32.0,
        12.0,
        36,
    )
    .translate(
        CHALLENGE_X / 2.0 - 42.0,
        CHALLENGE_Y / 2.0 - 26.0,
        CHALLENGE_Z + 6.0,
    );

    dirty_start + clean_end
}

fn breakthrough_witness_windows() -> Part {
    let panel = centered_cube(
        "waste_vent_breakthrough_witness_window_panel",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);

    panel - witness_window_cuts()
        + witness_window_frames()
        + witness_coupon_clip_bank()
        + witness_reference_scale()
}

fn witness_window_cuts() -> Part {
    let mut cuts = Part::empty("waste_vent_breakthrough_witness_window_cuts");
    for i in 0..WITNESS_WINDOWS {
        let x = centered_index(i, WITNESS_WINDOWS, WITNESS_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("waste_vent_breakthrough_witness_window_opening_{i}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(x, 4.0, WITNESS_Z / 2.0);
    }
    cuts
}

fn witness_window_frames() -> Part {
    let mut frames = Part::empty("waste_vent_breakthrough_witness_window_frames");
    for i in 0..WITNESS_WINDOWS {
        let x = centered_index(i, WITNESS_WINDOWS, WITNESS_PITCH_X);
        frames = frames
            + shallow_ring_rect(
                &format!("waste_vent_breakthrough_witness_window_frame_{i}"),
                WITNESS_WINDOW_X + 18.0,
                WITNESS_WINDOW_Y + 18.0,
                WITNESS_WINDOW_X + 4.0,
                WITNESS_WINDOW_Y + 4.0,
                6.0,
            )
            .translate(x, 4.0, WITNESS_Z + 3.0);
    }
    frames
}

fn witness_coupon_clip_bank() -> Part {
    let mut clips = Part::empty("waste_vent_breakthrough_witness_coupon_clip_bank");
    for i in 0..WITNESS_WINDOWS {
        let x = centered_index(i, WITNESS_WINDOWS, WITNESS_PITCH_X);
        clips = clips
            + centered_cube(
                format!("waste_vent_breakthrough_witness_coupon_lower_clip_{i}"),
                38.0,
                8.0,
                12.0,
            )
            .translate(x, -WITNESS_Y / 2.0 + 24.0, WITNESS_Z + 6.0)
            + centered_cube(
                format!("waste_vent_breakthrough_witness_coupon_upper_clip_{i}"),
                38.0,
                8.0,
                12.0,
            )
            .translate(x, WITNESS_Y / 2.0 - 24.0, WITNESS_Z + 6.0);
    }
    clips
}

fn witness_reference_scale() -> Part {
    let baseline = centered_cube(
        "waste_vent_breakthrough_witness_window_breakthrough_threshold_baseline",
        WITNESS_X - 70.0,
        5.0,
        5.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 54.0, WITNESS_Z + 2.5);
    let mut ticks = Part::empty("waste_vent_breakthrough_witness_window_reference_ticks");
    for i in 0..9 {
        let x = centered_index(i, 9, 50.0);
        ticks = ticks
            + centered_cube(
                format!("waste_vent_breakthrough_witness_reference_tick_{i}"),
                4.0,
                18.0,
                7.0,
            )
            .translate(x, -WITNESS_Y / 2.0 + 54.0, WITNESS_Z + 3.5);
    }
    baseline + ticks
}

fn backpressure_sensor_pockets() -> Part {
    let plate = centered_cube(
        "waste_vent_breakthrough_backpressure_sensor_pocket_plate",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);

    plate - pressure_sensor_recesses() - pressure_tap_cuts()
        + sensor_retainer_lips()
        + pressure_manifold_route_ribs()
        + overpressure_relief_guard()
}

fn pressure_sensor_recesses() -> Part {
    let mut pockets = Part::empty("waste_vent_breakthrough_pressure_sensor_recesses");
    for i in 0..BACKPRESSURE_SENSORS {
        let x = centered_index(i, BACKPRESSURE_SENSORS, 72.0);
        pockets = pockets
            + centered_cube(
                format!("waste_vent_breakthrough_backpressure_sensor_pocket_cut_{i}"),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                SENSOR_POCKET_Z + 2.0,
            )
            .translate(x, -22.0, PRESSURE_Z - SENSOR_POCKET_Z / 2.0);
    }
    pockets
}

fn pressure_tap_cuts() -> Part {
    let mut taps = Part::empty("waste_vent_breakthrough_pressure_tap_cuts");
    for i in 0..PRESSURE_TAPS {
        let x = centered_index(i, PRESSURE_TAPS, 56.0);
        taps = taps
            + centered_cylinder(
                format!("waste_vent_breakthrough_pressure_tap_cut_{i}"),
                5.0,
                PRESSURE_Y + 10.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, PRESSURE_Z - 16.0);
    }
    taps
}

fn sensor_retainer_lips() -> Part {
    let mut lips = Part::empty("waste_vent_breakthrough_sensor_retainer_lips");
    for i in 0..BACKPRESSURE_SENSORS {
        let x = centered_index(i, BACKPRESSURE_SENSORS, 72.0);
        lips = lips
            + shallow_ring_rect(
                &format!("waste_vent_breakthrough_sensor_retainer_lip_{i}"),
                SENSOR_POCKET_X + 14.0,
                SENSOR_POCKET_Y + 14.0,
                SENSOR_POCKET_X + 3.0,
                SENSOR_POCKET_Y + 3.0,
                6.0,
            )
            .translate(x, -22.0, PRESSURE_Z + 3.0);
    }
    lips
}

fn pressure_manifold_route_ribs() -> Part {
    let dirty_header = centered_cube(
        "waste_vent_breakthrough_dirty_side_pressure_header_rib",
        PRESSURE_X - 58.0,
        7.0,
        8.0,
    )
    .translate(0.0, PRESSURE_Y / 2.0 - 34.0, PRESSURE_Z + 4.0);
    let clean_header = centered_cube(
        "waste_vent_breakthrough_clean_side_pressure_header_rib",
        PRESSURE_X - 58.0,
        7.0,
        8.0,
    )
    .translate(0.0, PRESSURE_Y / 2.0 - 54.0, PRESSURE_Z + 4.0);
    let mut branches = Part::empty("waste_vent_breakthrough_pressure_branch_ribs");
    for i in 0..BACKPRESSURE_SENSORS {
        let x = centered_index(i, BACKPRESSURE_SENSORS, 72.0);
        branches = branches
            + centered_cube(
                format!("waste_vent_breakthrough_pressure_branch_rib_{i}"),
                7.0,
                62.0,
                7.0,
            )
            .translate(x, 24.0, PRESSURE_Z + 3.5);
    }
    dirty_header + clean_header + branches
}

fn overpressure_relief_guard() -> Part {
    let guard = vertical_ring(
        "waste_vent_breakthrough_overpressure_relief_guard_ring",
        74.0,
        48.0,
        12.0,
        48,
    )
    .translate(
        -PRESSURE_X / 2.0 + 48.0,
        -PRESSURE_Y / 2.0 + 42.0,
        PRESSURE_Z + 6.0,
    );
    let flag = centered_cube(
        "waste_vent_breakthrough_overpressure_relief_flag_land",
        74.0,
        16.0,
        8.0,
    )
    .translate(
        -PRESSURE_X / 2.0 + 48.0,
        -PRESSURE_Y / 2.0 + 88.0,
        PRESSURE_Z + 4.0,
    );

    guard + flag
}

fn overflow_foam_sensor_brackets() -> Part {
    let base = centered_cube(
        "waste_vent_breakthrough_overflow_foam_sensor_base_plate",
        FOAM_X,
        FOAM_Y,
        28.0,
    )
    .translate(0.0, 0.0, 14.0);
    let back_wall = centered_cube(
        "waste_vent_breakthrough_overflow_foam_sensor_back_wall",
        FOAM_X,
        18.0,
        FOAM_Z,
    )
    .translate(0.0, FOAM_Y / 2.0 - 9.0, FOAM_Z / 2.0);
    let riser = vertical_ring(
        "waste_vent_breakthrough_overflow_standpipe_witness_riser",
        FOAM_RISER_D + 22.0,
        FOAM_RISER_D,
        FOAM_Z - 16.0,
        44,
    )
    .translate(-FOAM_X / 2.0 + 58.0, 0.0, 28.0 + (FOAM_Z - 16.0) / 2.0);

    base + back_wall + riser + overflow_level_brackets() + foam_sensor_forks() + cable_tie_lands()
}

fn overflow_level_brackets() -> Part {
    let mut brackets = Part::empty("waste_vent_breakthrough_overflow_level_brackets");
    for i in 0..OVERFLOW_LEVELS {
        let z = 48.0 + i as f64 * 34.0;
        brackets = brackets
            + centered_cube(
                format!("waste_vent_breakthrough_overflow_level_bracket_{i}"),
                FOAM_X - 74.0,
                14.0,
                10.0,
            )
            .translate(22.0, FOAM_Y / 2.0 - 28.0, z)
            + centered_cube(
                format!("waste_vent_breakthrough_overflow_level_label_land_{i}"),
                54.0,
                9.0,
                18.0,
            )
            .translate(FOAM_X / 2.0 - 46.0, FOAM_Y / 2.0 - 42.0, z);
    }
    brackets
}

fn foam_sensor_forks() -> Part {
    let mut forks = Part::empty("waste_vent_breakthrough_foam_sensor_forks");
    for i in 0..FOAM_SENSOR_FORKS {
        let x = centered_index(i, FOAM_SENSOR_FORKS, 56.0) + 34.0;
        forks = forks
            + centered_cube(
                format!("waste_vent_breakthrough_foam_sensor_left_fork_{i}"),
                8.0,
                66.0,
                16.0,
            )
            .translate(x - 10.0, -24.0, 96.0)
            + centered_cube(
                format!("waste_vent_breakthrough_foam_sensor_right_fork_{i}"),
                8.0,
                66.0,
                16.0,
            )
            .translate(x + 10.0, -24.0, 96.0)
            + centered_cube(
                format!("waste_vent_breakthrough_foam_sensor_cable_stand_{i}"),
                30.0,
                10.0,
                44.0,
            )
            .translate(x, -FOAM_Y / 2.0 + 28.0, 70.0);
    }
    forks
}

fn cable_tie_lands() -> Part {
    let mut lands = Part::empty("waste_vent_breakthrough_foam_sensor_cable_tie_lands");
    for i in 0..4 {
        let x = centered_index(i, 4, 62.0);
        lands = lands
            + shallow_ring_rect(
                &format!("waste_vent_breakthrough_foam_sensor_cable_tie_land_{i}"),
                36.0,
                18.0,
                18.0,
                8.0,
                7.0,
            )
            .translate(x, -FOAM_Y / 2.0 + 28.0, 31.5);
    }
    lands
}

fn sample_custody_wells() -> Part {
    let tray = centered_cube(
        "waste_vent_breakthrough_sample_custody_well_tray",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);

    tray - sample_well_cuts()
        + sample_well_rims()
        + custody_seal_posts()
        + sample_disposition_lanes()
}

fn sample_well_cuts() -> Part {
    let mut wells = Part::empty("waste_vent_breakthrough_sample_well_cuts");
    for row in 0..2 {
        for col in 0..6 {
            let i = row * 6 + col;
            let x = centered_index(col, 6, SAMPLE_WELL_PITCH_X);
            let y = centered_index(row, 2, SAMPLE_WELL_PITCH_Y) + 4.0;
            wells = wells
                + centered_cylinder(
                    format!("waste_vent_breakthrough_sample_custody_well_cut_{i}"),
                    SAMPLE_WELL_D / 2.0,
                    CUSTODY_Z + 5.0,
                    32,
                )
                .translate(x, y, CUSTODY_Z / 2.0);
        }
    }
    wells
}

fn sample_well_rims() -> Part {
    let mut rims = Part::empty("waste_vent_breakthrough_sample_well_rims");
    for row in 0..2 {
        for col in 0..6 {
            let i = row * 6 + col;
            let x = centered_index(col, 6, SAMPLE_WELL_PITCH_X);
            let y = centered_index(row, 2, SAMPLE_WELL_PITCH_Y) + 4.0;
            rims = rims
                + vertical_ring(
                    &format!("waste_vent_breakthrough_sample_custody_well_rim_{i}"),
                    SAMPLE_WELL_D + 14.0,
                    SAMPLE_WELL_D + 2.0,
                    7.0,
                    32,
                )
                .translate(x, y, CUSTODY_Z + 3.5);
        }
    }
    rims
}

fn custody_seal_posts() -> Part {
    let mut posts = Part::empty("waste_vent_breakthrough_sample_custody_seal_posts");
    for (i, (x, y)) in [
        (-CUSTODY_X / 2.0 + 32.0, -CUSTODY_Y / 2.0 + 28.0),
        (CUSTODY_X / 2.0 - 32.0, -CUSTODY_Y / 2.0 + 28.0),
        (-CUSTODY_X / 2.0 + 32.0, CUSTODY_Y / 2.0 - 28.0),
        (CUSTODY_X / 2.0 - 32.0, CUSTODY_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("waste_vent_breakthrough_custody_wire_seal_post_{i}"),
                8.0,
                25.0,
                24,
            )
            .translate(*x, *y, CUSTODY_Z + 12.5);
    }
    posts
}

fn sample_disposition_lanes() -> Part {
    let retain = centered_cube(
        "waste_vent_breakthrough_retain_sample_lane_rib",
        CUSTODY_X - 58.0,
        6.0,
        8.0,
    )
    .translate(0.0, -CUSTODY_Y / 2.0 + 25.0, CUSTODY_Z + 4.0);
    let split = centered_cube(
        "waste_vent_breakthrough_split_sample_lane_rib",
        CUSTODY_X - 58.0,
        6.0,
        8.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z + 4.0);
    let quarantine = centered_cube(
        "waste_vent_breakthrough_quarantine_sample_lane_rib",
        CUSTODY_X - 58.0,
        6.0,
        8.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 - 25.0, CUSTODY_Z + 4.0);

    retain + split + quarantine
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "waste_vent_breakthrough_barcode_custody_panel",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    )
    .translate(0.0, 0.0, BARCODE_Z / 2.0);

    panel + barcode_land_grid() + tamper_seal_lands() + custody_card_slots()
}

fn barcode_land_grid() -> Part {
    let mut lands = Part::empty("waste_vent_breakthrough_barcode_land_grid");
    for row in 0..BARCODE_ROWS {
        for col in 0..BARCODE_COLS {
            let i = row * BARCODE_COLS + col;
            let x = centered_index(col, BARCODE_COLS, 78.0);
            let y = centered_index(row, BARCODE_ROWS, 24.0);
            lands = lands
                + centered_cube(
                    format!("waste_vent_breakthrough_barcode_custody_land_{i}"),
                    BARCODE_LAND_X,
                    BARCODE_LAND_Y,
                    4.0,
                )
                .translate(x, y, BARCODE_Z + 2.0);
        }
    }
    lands
}

fn tamper_seal_lands() -> Part {
    let mut lands = Part::empty("waste_vent_breakthrough_tamper_seal_lands");
    for i in 0..TAMPER_SEAL_LANDS {
        let x = centered_index(i, TAMPER_SEAL_LANDS, 48.0);
        lands = lands
            + vertical_ring(
                &format!("waste_vent_breakthrough_tamper_seal_land_{i}"),
                23.0,
                11.0,
                5.0,
                24,
            )
            .translate(x, BARCODE_Y / 2.0 - 18.0, BARCODE_Z + 2.5);
    }
    lands
}

fn custody_card_slots() -> Part {
    let front = centered_cube(
        "waste_vent_breakthrough_custody_card_front_capture_rail",
        BARCODE_X - 52.0,
        5.0,
        11.0,
    )
    .translate(0.0, -BARCODE_Y / 2.0 + 12.0, BARCODE_Z + 5.5);
    let rear = centered_cube(
        "waste_vent_breakthrough_custody_card_rear_capture_rail",
        BARCODE_X - 52.0,
        5.0,
        11.0,
    )
    .translate(0.0, -BARCODE_Y / 2.0 + 34.0, BARCODE_Z + 5.5);
    front + rear
}

fn clean_dirty_segregation_barriers() -> Part {
    let center_wall = centered_cube(
        "waste_vent_breakthrough_clean_dirty_main_segregation_wall",
        14.0,
        STATION_Y - 174.0,
        BARRIER_WALL_Z,
    )
    .translate(0.0, 0.0, BARRIER_WALL_Z / 2.0);
    let lateral_wall = centered_cube(
        "waste_vent_breakthrough_challenge_waste_lateral_segregation_wall",
        STATION_X - 220.0,
        12.0,
        54.0,
    )
    .translate(0.0, -210.0, 27.0);
    let rear_wall = centered_cube(
        "waste_vent_breakthrough_filter_service_clean_side_barrier",
        STATION_X - 220.0,
        12.0,
        64.0,
    )
    .translate(0.0, 205.0, 32.0);

    center_wall + lateral_wall + rear_wall + segregation_gate_frames() + clean_dirty_label_lands()
}

fn segregation_gate_frames() -> Part {
    let mut gates = Part::empty("waste_vent_breakthrough_clean_dirty_gate_frames");
    for i in 0..BARRIER_GATE_COUNT {
        let y = centered_index(i, BARRIER_GATE_COUNT, 170.0) - 20.0;
        gates = gates
            + shallow_ring_rect(
                &format!("waste_vent_breakthrough_clean_dirty_pass_through_gate_frame_{i}"),
                88.0,
                38.0,
                64.0,
                20.0,
                10.0,
            )
            .rotate(0.0, 0.0, 90.0)
            .translate(0.0, y, 58.0)
            + centered_cube(
                format!("waste_vent_breakthrough_gate_drip_lip_{i}"),
                74.0,
                8.0,
                10.0,
            )
            .translate(0.0, y - 28.0, 40.0);
    }
    gates
}

fn clean_dirty_label_lands() -> Part {
    let mut lands = Part::empty("waste_vent_breakthrough_clean_dirty_label_lands");
    for i in 0..DIRTY_SIDE_LABEL_LANDS {
        let y = centered_index(i, DIRTY_SIDE_LABEL_LANDS, 58.0);
        lands = lands
            + centered_cube(
                format!("waste_vent_breakthrough_dirty_side_label_land_{i}"),
                74.0,
                18.0,
                5.0,
            )
            .translate(-58.0, y, BARRIER_WALL_Z + 2.5);
    }
    for i in 0..CLEAN_SIDE_LABEL_LANDS {
        let y = centered_index(i, CLEAN_SIDE_LABEL_LANDS, 58.0);
        lands = lands
            + centered_cube(
                format!("waste_vent_breakthrough_clean_side_label_land_{i}"),
                74.0,
                18.0,
                5.0,
            )
            .translate(58.0, y, BARRIER_WALL_Z + 2.5);
    }
    lands
}

fn evidence_bridge() -> Part {
    let left_front = bridge_post(-EVIDENCE_SPAN_X / 2.0 + 48.0, -250.0, "left_front");
    let left_rear = bridge_post(-EVIDENCE_SPAN_X / 2.0 + 48.0, 250.0, "left_rear");
    let right_front = bridge_post(EVIDENCE_SPAN_X / 2.0 - 48.0, -250.0, "right_front");
    let right_rear = bridge_post(EVIDENCE_SPAN_X / 2.0 - 48.0, 250.0, "right_rear");
    let front_beam = centered_cube(
        "waste_vent_breakthrough_evidence_bridge_front_beam",
        EVIDENCE_SPAN_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, -250.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0);
    let rear_beam = centered_cube(
        "waste_vent_breakthrough_evidence_bridge_rear_beam",
        EVIDENCE_SPAN_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 250.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0);
    let camera_rail = centered_cube(
        "waste_vent_breakthrough_evidence_bridge_camera_rail",
        EVIDENCE_SPAN_X - 170.0,
        18.0,
        24.0,
    )
    .translate(0.0, 0.0, EVIDENCE_POST_Z - 24.0);

    left_front
        + left_rear
        + right_front
        + right_rear
        + front_beam
        + rear_beam
        + camera_rail
        + evidence_camera_mounts()
        + evidence_light_bars()
        + evidence_scale_rulers()
}

fn bridge_post(x: f64, y: f64, name: &str) -> Part {
    centered_cube(
        format!("waste_vent_breakthrough_evidence_bridge_{name}_post"),
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(x, y, EVIDENCE_POST_Z / 2.0)
}

fn evidence_camera_mounts() -> Part {
    let mut mounts = Part::empty("waste_vent_breakthrough_evidence_camera_mounts");
    for i in 0..EVIDENCE_CAMERAS {
        let x = centered_index(i, EVIDENCE_CAMERAS, 230.0);
        mounts = mounts
            + centered_cube(
                format!("waste_vent_breakthrough_evidence_camera_mount_{i}"),
                72.0,
                52.0,
                18.0,
            )
            .translate(x, 0.0, EVIDENCE_POST_Z - 50.0)
            + centered_cylinder(
                format!("waste_vent_breakthrough_evidence_camera_lens_envelope_{i}"),
                17.0,
                24.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -34.0, EVIDENCE_POST_Z - 50.0);
    }
    mounts
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("waste_vent_breakthrough_evidence_light_bars");
    for (i, y) in [-110.0, 110.0].iter().enumerate() {
        bars = bars
            + centered_cube(
                format!("waste_vent_breakthrough_evidence_light_bar_{i}"),
                EVIDENCE_SPAN_X - 260.0,
                16.0,
                12.0,
            )
            .translate(0.0, *y, EVIDENCE_POST_Z - 78.0);
    }
    bars
}

fn evidence_scale_rulers() -> Part {
    let mut rulers = Part::empty("waste_vent_breakthrough_evidence_scale_rulers");
    for (i, y) in [-212.0, 212.0].iter().enumerate() {
        rulers = rulers
            + centered_cube(
                format!("waste_vent_breakthrough_evidence_scale_ruler_{i}"),
                EVIDENCE_SPAN_X - 250.0,
                6.0,
                8.0,
            )
            .translate(0.0, *y, EVIDENCE_POST_Z - 104.0);
        for tick in 0..12 {
            let x = centered_index(tick, 12, 86.0);
            rulers = rulers
                + centered_cube(
                    format!("waste_vent_breakthrough_evidence_ruler_{i}_tick_{tick}"),
                    4.0,
                    20.0,
                    8.0,
                )
                .translate(x, *y, EVIDENCE_POST_Z - 104.0);
        }
    }
    rulers
}

fn robot_service_keepout_gauges() -> Part {
    let front = keepout_frame(
        "waste_vent_breakthrough_front_robot_service_keepout_gauge",
        STATION_X - 160.0,
        88.0,
        32.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 74.0, 16.0);
    let rear = keepout_frame(
        "waste_vent_breakthrough_rear_filter_service_keepout_gauge",
        STATION_X - 200.0,
        80.0,
        34.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 86.0, 17.0);
    let left = keepout_frame(
        "waste_vent_breakthrough_left_waste_container_swap_keepout_gauge",
        90.0,
        STATION_Y - 210.0,
        38.0,
    )
    .translate(-STATION_X / 2.0 + 88.0, 0.0, 19.0);
    let right = keepout_frame(
        "waste_vent_breakthrough_right_sensor_calibration_keepout_gauge",
        86.0,
        STATION_Y - 240.0,
        38.0,
    )
    .translate(STATION_X / 2.0 - 92.0, -20.0, 19.0);
    let top_filter = centered_cube(
        "waste_vent_breakthrough_top_filter_lift_keepout_height_gauge",
        380.0,
        28.0,
        10.0,
    )
    .translate(
        FILTER_DOCK_POS.0,
        FILTER_DOCK_POS.1,
        TOP_FILTER_LIFT_CLEARANCE,
    );
    let evidence_height = centered_cube(
        "waste_vent_breakthrough_evidence_bridge_overhead_keepout_gauge",
        EVIDENCE_SPAN_X - 160.0,
        20.0,
        10.0,
    )
    .translate(
        0.0,
        EVIDENCE_POS.1,
        EVIDENCE_POST_Z + EVIDENCE_BEAM_Z + 34.0,
    );

    front + rear + left + right + top_filter + evidence_height + keepout_height_posts()
}

fn keepout_height_posts() -> Part {
    let mut posts = Part::empty("waste_vent_breakthrough_keepout_height_posts");
    for (i, (x, y, z)) in [
        (NEST_POS.0, NEST_POS.1, TOP_FILTER_LIFT_CLEARANCE - 52.0),
        (
            FILTER_DOCK_POS.0,
            FILTER_DOCK_POS.1,
            TOP_FILTER_LIFT_CLEARANCE,
        ),
        (PRESSURE_POS.0, PRESSURE_POS.1, 190.0),
        (FOAM_POS.0, FOAM_POS.1, 210.0),
        (WITNESS_POS.0, WITNESS_POS.1, 175.0),
        (CUSTODY_POS.0, CUSTODY_POS.1, 128.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("waste_vent_breakthrough_keepout_height_post_{i}"),
                5.0,
                *z,
                18,
            )
            .translate(*x, *y, z / 2.0);
    }
    posts
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    shallow_ring_rect(name, x, y, x - 24.0, y - 24.0, z)
}

fn fiducial_disc(name: &str) -> Part {
    vertical_ring(name, 32.0, 10.0, 6.0, 36)
        + centered_cylinder(format!("{name}_center_dot"), 3.0, 8.0, 18).translate(0.0, 0.0, 1.0)
}

fn tube_bulkhead(name: &str, x: f64, y: f64, z: f64, od: f64) -> Part {
    let boss = vertical_ring(&format!("{name}_face_ring"), od + 18.0, od, 14.0, 36)
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, z);
    let wrench_flat = centered_cube(format!("{name}_wrench_flat"), od + 24.0, 7.0, 10.0).translate(
        x,
        y - 9.0,
        z + od / 2.0,
    );

    boss + wrench_flat
}

fn vertical_ring(name: &str, outer_d: f64, inner_d: f64, height: f64, segments: u32) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, height, segments)
        - centered_cylinder(
            format!("{name}_inner"),
            inner_d / 2.0,
            height + 2.0,
            segments,
        )
}

fn shallow_ring(name: &str, outer_d: f64, inner_d: f64, height: f64, segments: u32) -> Part {
    vertical_ring(name, outer_d, inner_d, height, segments)
}

fn shallow_ring_rect(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    inner_x: f64,
    inner_y: f64,
    height: f64,
) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
        - centered_cube(format!("{name}_inner"), inner_x, inner_y, height + 2.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_waste_container_vent_filter_condensate_breakthrough_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "waste_bottle_bag_nest",
            "hydrophobic_vent_filter_cartridge_dock",
            "condensate_challenge_channel",
            "breakthrough_witness_windows",
            "backpressure_sensor_pockets",
            "overflow_foam_sensor_brackets",
            "sample_custody_wells",
            "barcode_custody_lands",
            "clean_dirty_segregation_barriers",
            "evidence_bridge",
            "robot_service_keepout_gauges",
            "assembly",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "{feature} missing from output manifest"
            );
        }
    }

    #[test]
    fn modules_fit_and_do_not_overlap_on_deck() {
        assert_design_constraints();
        for item in socket_rects() {
            assert!(item.fits_inside_deck(), "{} does not fit", item.name);
        }
    }

    #[test]
    fn containment_exceeds_challenge_volume() {
        assert!(secondary_containment_freeboard_ml() > maximum_condensate_challenge_ml());
        assert!(maximum_condensate_challenge_ml() < 500.0);
    }

    #[test]
    fn vent_filter_condensate_and_witness_counts_match_design_intent() {
        assert_eq!(FILTER_CLAMPS, 4);
        assert_eq!(FILTER_END_BULKHEADS, 2);
        assert_eq!(CHALLENGE_LANES, 4);
        assert_eq!(CHALLENGE_PORTS, CHALLENGE_LANES);
        assert_eq!(WITNESS_WINDOWS, 8);
        assert_eq!(WITNESS_COUPON_CLIPS, WITNESS_WINDOWS * 2);
    }

    #[test]
    fn sensing_custody_and_keepout_capacity_is_explicit() {
        assert_eq!(BACKPRESSURE_SENSORS, 4);
        assert_eq!(OVERFLOW_LEVELS, 3);
        assert_eq!(FOAM_SENSOR_FORKS, 4);
        assert_eq!(SAMPLE_WELLS, 12);
        assert_eq!(BARCODE_LANDS, 16);
        assert_eq!(BARRIER_GATE_COUNT, 3);
        assert_eq!(KEEP_OUT_GAUGES, 6);
    }
}
