use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robot gripper pad wear and particle-shedding validation fixture.
//
// Intent:
// - Stage clean gripper-pad coupons, repeated-contact witness rails, cassette
//   surrogate contact faces, and force/load witnesses inside one contained
//   station so pad wear and loose particle shedding can be challenged without
//   live culture materials.
// - Keep particle collection troughs, clean/used segregation, traceability
//   lands, release/hold/reject disposition lanes, evidence capture, and robot
//   keepout gauges physically tied to the validation fixture.
// - Export modular STL groups for later parent integration. This file is only
//   fixture packaging geometry; it does not define acceptance criteria,
//   particle chemistry, validated pad materials, or robot programs.

const BIN_PREFIX: &str = "closed_robot_gripper_pad_wear_particle_shedding_station";

const OUTPUTS: &[&str] = &[
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_base_tray.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_gripper_pad_coupon_nests.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_repeated_contact_witness_rails.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_particle_collection_troughs.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_cassette_surrogate_contact_block.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_force_load_witness_pockets.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_clean_used_pad_segregation.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_barcode_certificate_lands.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_release_hold_reject_lanes.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_transparent_evidence_bridge.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_robot_service_keepout_gauges.stl",
    "output/closed_robot_gripper_pad_wear_particle_shedding_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "gripper_pad_coupon_nests",
    "repeated_contact_witness_rails",
    "particle_collection_troughs",
    "cassette_surrogate_contact_block",
    "force_load_witness_pockets",
    "clean_used_pad_segregation",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "transparent_evidence_bridge",
    "robot_service_keepout_gauges",
    "assembly_export",
];

const STATION_X: f64 = 1220.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const MODULE_GAP: f64 = 20.0;
const DATUM_PIN_D: f64 = 6.0;

const PAD_NEST_X: f64 = 330.0;
const PAD_NEST_Y: f64 = 170.0;
const PAD_NEST_Z: f64 = 52.0;
const PAD_NEST_POS: (f64, f64) = (-410.0, 225.0);
const PAD_ROWS: usize = 2;
const PAD_COLS: usize = 4;
const PAD_COUPON_COUNT: usize = PAD_ROWS * PAD_COLS;
const PAD_SLOT_X: f64 = 46.0;
const PAD_SLOT_Y: f64 = 62.0;
const PAD_PITCH_X: f64 = 68.0;
const PAD_PITCH_Y: f64 = 74.0;

const WITNESS_RAIL_X: f64 = 360.0;
const WITNESS_RAIL_Y: f64 = 170.0;
const WITNESS_RAIL_Z: f64 = 46.0;
const WITNESS_RAIL_POS: (f64, f64) = (-20.0, 225.0);
const CONTACT_RAIL_COUNT: usize = 4;
const CONTACT_TICK_COUNT: usize = 7;
const CONTACT_TICK_PITCH: f64 = 20.0;

const TROUGH_X: f64 = 330.0;
const TROUGH_Y: f64 = 170.0;
const TROUGH_Z: f64 = 48.0;
const TROUGH_POS: (f64, f64) = (380.0, 225.0);
const PARTICLE_TROUGH_COUNT: usize = 3;
const TROUGH_GRID_TICKS: usize = 6;

const CONTACT_BLOCK_X: f64 = 340.0;
const CONTACT_BLOCK_Y: f64 = 190.0;
const CONTACT_BLOCK_Z: f64 = 78.0;
const CONTACT_BLOCK_POS: (f64, f64) = (-400.0, 0.0);
const CASSETTE_CONTACT_STRIPS: usize = 5;
const CASSETTE_DATUM_BOSSES: usize = 4;

const FORCE_POCKET_X: f64 = 330.0;
const FORCE_POCKET_Y: f64 = 190.0;
const FORCE_POCKET_Z: f64 = 46.0;
const FORCE_POCKET_POS: (f64, f64) = (0.0, 0.0);
const FORCE_POCKET_COUNT: usize = 6;
const LOAD_WITNESS_POCKET_D: f64 = 26.0;
const LOAD_FILM_SLOT_COUNT: usize = 4;

const SEGREGATION_X: f64 = 340.0;
const SEGREGATION_Y: f64 = 190.0;
const SEGREGATION_Z: f64 = 70.0;
const SEGREGATION_POS: (f64, f64) = (400.0, 0.0);
const CLEAN_PAD_SLOTS: usize = 6;
const USED_PAD_SLOTS: usize = 6;
const SEGREGATION_DIVIDER_Z: f64 = 94.0;

const TRACE_X: f64 = 320.0;
const TRACE_Y: f64 = 130.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-400.0, -240.0);
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;

const STATUS_X: f64 = 350.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 44.0;
const STATUS_POS: (f64, f64) = (0.0, -240.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;

const BRIDGE_SPAN_X: f64 = 1080.0;
const BRIDGE_SPAN_Y: f64 = 610.0;
const BRIDGE_UNDERSIDE_Z: f64 = 176.0;
const BRIDGE_BEAM_Z: f64 = 26.0;
const BRIDGE_WINDOW_X: f64 = 610.0;
const BRIDGE_WINDOW_Y: f64 = 290.0;
const EVIDENCE_FIDUCIAL_COUNT: usize = 10;

const ROBOT_SWEEP_CLEARANCE_Y: f64 = 430.0;
const SERVICE_REAR_CLEARANCE: f64 = 260.0;
const SIDE_PAD_LOAD_CLEARANCE: f64 = 210.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;

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
    export(OUTPUTS[0], &base);

    let pad_nests = gripper_pad_coupon_nests();
    export(OUTPUTS[1], &pad_nests);

    let witness_rails = repeated_contact_witness_rails();
    export(OUTPUTS[2], &witness_rails);

    let troughs = particle_collection_troughs();
    export(OUTPUTS[3], &troughs);

    let contact_block = cassette_surrogate_contact_block();
    export(OUTPUTS[4], &contact_block);

    let force_pockets = force_load_witness_pockets();
    export(OUTPUTS[5], &force_pockets);

    let segregation = clean_used_pad_segregation();
    export(OUTPUTS[6], &segregation);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[8], &disposition);

    let bridge = transparent_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + pad_nests.translate(PAD_NEST_POS.0, PAD_NEST_POS.1, insert_z())
        + witness_rails.translate(WITNESS_RAIL_POS.0, WITNESS_RAIL_POS.1, insert_z())
        + troughs.translate(TROUGH_POS.0, TROUGH_POS.1, insert_z())
        + contact_block.translate(CONTACT_BLOCK_POS.0, CONTACT_BLOCK_POS.1, insert_z())
        + force_pockets.translate(FORCE_POCKET_POS.0, FORCE_POCKET_POS.1, insert_z())
        + segregation.translate(SEGREGATION_POS.0, SEGREGATION_POS.1, insert_z())
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z())
        + disposition.translate(STATUS_POS.0, STATUS_POS.1, insert_z())
        + bridge.translate(0.0, 0.0, BASE_Z)
        + keepouts.translate(0.0, 0.0, BASE_Z);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed robot gripper pad wear / particle-shedding station:");
    println!("  Generator:                 {BIN_PREFIX}");
    println!("  Fixture footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Pad coupons:                {PAD_COUPON_COUNT} gripper-pad nests with clean orientation keys and witness lips"
    );
    println!(
        "  Wear challenge:             {CONTACT_RAIL_COUNT} repeated-contact rails, {CONTACT_TICK_COUNT} contact ticks per rail, cassette surrogate block with {CASSETTE_CONTACT_STRIPS} contact strips"
    );
    println!(
        "  Particle capture:           {PARTICLE_TROUGH_COUNT} removable trough liners with grid ticks and closed drain witness ports"
    );
    println!(
        "  Load evidence:              {FORCE_POCKET_COUNT} force/load witness pockets and {LOAD_FILM_SLOT_COUNT} force-film slots"
    );
    println!(
        "  Traceability/disposition:   {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {STATUS_LANES} release/hold/reject lanes"
    );
    println!(
        "  Segregation/keepouts:       {CLEAN_PAD_SLOTS} clean pad slots, {USED_PAD_SLOTS} used pad slots, {KEEP_OUT_GAUGE_COUNT} robot/service gauges, {ROBOT_Z_CLEARANCE:.0}mm Z clearance witness"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    assert_eq!(PAD_COUPON_COUNT, PAD_ROWS * PAD_COLS);
    assert_eq!(STATUS_LANES, 3);
    assert_eq!(CASSETTE_DATUM_BOSSES, 4);
    assert!(SEGREGATION_DIVIDER_Z > SEGREGATION_Z);
    assert!(DATUM_PIN_D > 5.0);
    assert!(ROBOT_SWEEP_CLEARANCE_Y > STATUS_Y + TROUGH_Y);
    assert!(highest_fixture_feature_z() < ROBOT_Z_CLEARANCE);

    let footprints = component_footprints();
    for footprint in footprints {
        assert!(
            fits_on_station(footprint),
            "{} exceeds usable contained deck",
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

    assert!(
        BRIDGE_SPAN_X + 70.0 < STATION_X - 2.0 * RIM_W
            && BRIDGE_SPAN_Y + 70.0 < STATION_Y - 2.0 * RIM_W,
        "evidence bridge feet must land inside the containment rim"
    );
}

fn highest_fixture_feature_z() -> f64 {
    BASE_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z
}

fn component_footprints() -> [Footprint; 8] {
    [
        fp(
            "gripper_pad_coupon_nests",
            PAD_NEST_POS,
            PAD_NEST_X,
            PAD_NEST_Y,
        ),
        fp(
            "repeated_contact_witness_rails",
            WITNESS_RAIL_POS,
            WITNESS_RAIL_X,
            WITNESS_RAIL_Y,
        ),
        fp(
            "particle_collection_troughs",
            TROUGH_POS,
            TROUGH_X,
            TROUGH_Y,
        ),
        fp(
            "cassette_surrogate_contact_block",
            CONTACT_BLOCK_POS,
            CONTACT_BLOCK_X,
            CONTACT_BLOCK_Y,
        ),
        fp(
            "force_load_witness_pockets",
            FORCE_POCKET_POS,
            FORCE_POCKET_X,
            FORCE_POCKET_Y,
        ),
        fp(
            "clean_used_pad_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        fp("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        fp("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
    ]
}

fn fp(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_on_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - RIM_W - 8.0;
    let usable_y = STATION_Y / 2.0 - RIM_W - 8.0;
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
        "gripper_pad_shedding_station_closed_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let wipe_recess = centered_cube(
        "gripper_pad_shedding_station_wipeable_recessed_floor",
        STATION_X - 122.0,
        STATION_Y - 116.0,
        7.0,
    )
    .translate(0.0, -5.0, BASE_Z - 3.5);
    let particle_sump = centered_cube(
        "gripper_pad_shedding_station_front_particle_sump",
        STATION_X - 210.0,
        38.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z - 4.0);
    let drain_port = centered_cylinder(
        "gripper_pad_shedding_station_closed_drain_witness_port",
        5.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 78.0,
        -STATION_Y / 2.0 + 36.0,
        BASE_Z - 8.0,
    );

    deck - wipe_recess - particle_sump - drain_port - insert_sockets() - mount_holes()
        + perimeter_rims()
        + zone_dividers()
        + base_datum_targets()
        + transfer_lips()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("gripper_pad_shedding_station_insert_sockets");
    for footprint in component_footprints() {
        sockets = sockets
            + centered_cube(
                format!("gripper_pad_shedding_station_{}_socket", footprint.name),
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
    let mut holes = Part::empty("gripper_pad_shedding_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("gripper_pad_shedding_station_m6_clearance_{i}"),
                3.4,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("gripper_pad_shedding_station_mount_slot_relief_{i}"),
                26.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "gripper_pad_shedding_station_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "gripper_pad_shedding_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "gripper_pad_shedding_station_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "gripper_pad_shedding_station_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn zone_dividers() -> Part {
    let upper = centered_cube(
        "gripper_pad_shedding_station_clean_to_wear_zone_divider",
        STATION_X - 150.0,
        10.0,
        26.0,
    )
    .translate(0.0, 112.0, BASE_Z + 13.0);
    let lower = centered_cube(
        "gripper_pad_shedding_station_wear_to_disposition_zone_divider",
        STATION_X - 164.0,
        10.0,
        26.0,
    )
    .translate(0.0, -132.0, BASE_Z + 13.0);
    let clean_dirty = centered_cube(
        "gripper_pad_shedding_station_clean_used_vertical_divider",
        10.0,
        220.0,
        28.0,
    )
    .translate(210.0, 0.0, BASE_Z + 14.0);
    let trace_divider = centered_cube(
        "gripper_pad_shedding_station_traceability_lane_divider",
        10.0,
        172.0,
        24.0,
    )
    .translate(-205.0, -240.0, BASE_Z + 12.0);
    upper + lower + clean_dirty + trace_divider
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("gripper_pad_shedding_station_base_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 96.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 96.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 96.0, STATION_Y / 2.0 - 96.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("gripper_pad_shedding_station_robot_datum_boss_{i}"),
            13.0,
            7.0,
            36,
        )
        .translate(x, y, BASE_Z + 3.5);
        let center = centered_cylinder(
            format!("gripper_pad_shedding_station_robot_datum_center_mark_{i}"),
            DATUM_PIN_D / 4.0,
            8.0,
            20,
        )
        .translate(x, y, BASE_Z + 3.5);
        targets = targets + (boss - center);
    }
    targets
}

fn transfer_lips() -> Part {
    let coupon_lip = centered_cube(
        "gripper_pad_shedding_station_clean_pad_transfer_lip",
        PAD_NEST_X - 42.0,
        12.0,
        16.0,
    )
    .translate(
        PAD_NEST_POS.0,
        PAD_NEST_POS.1 - PAD_NEST_Y / 2.0 - 10.0,
        BASE_Z + 8.0,
    );
    let trough_lip = centered_cube(
        "gripper_pad_shedding_station_particle_trough_transfer_lip",
        TROUGH_X - 44.0,
        12.0,
        18.0,
    )
    .translate(
        TROUGH_POS.0,
        TROUGH_POS.1 - TROUGH_Y / 2.0 - 10.0,
        BASE_Z + 9.0,
    );
    let used_lip = centered_cube(
        "gripper_pad_shedding_station_used_pad_return_lip",
        SEGREGATION_X - 50.0,
        12.0,
        18.0,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1 - SEGREGATION_Y / 2.0 - 10.0,
        BASE_Z + 9.0,
    );
    coupon_lip + trough_lip + used_lip
}

fn gripper_pad_coupon_nests() -> Part {
    let body = module_panel(
        "gripper_pad_coupon_nest_body",
        PAD_NEST_X,
        PAD_NEST_Y,
        PAD_NEST_Z,
    );
    let recessed_field = centered_cube(
        "gripper_pad_coupon_nest_wipeable_recessed_field",
        PAD_NEST_X - 38.0,
        PAD_NEST_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, PAD_NEST_Z - 5.0);
    body - recessed_field - pad_coupon_slot_cuts()
        + pad_coupon_locator_lips()
        + pad_coupon_orientation_keys()
        + pad_coupon_id_lands()
        + gripper_pick_fiducials("pad_coupon_nests", PAD_NEST_X, PAD_NEST_Y, PAD_NEST_Z)
}

fn pad_coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("gripper_pad_coupon_nest_slot_cuts");
    for row in 0..PAD_ROWS {
        for col in 0..PAD_COLS {
            let i = row * PAD_COLS + col;
            let x = centered_index(col, PAD_COLS, PAD_PITCH_X);
            let y = centered_index(row, PAD_ROWS, PAD_PITCH_Y) + 2.0;
            let slot = centered_cube(
                format!("gripper_pad_coupon_nest_pad_coupon_slot_{i}"),
                PAD_SLOT_X,
                PAD_SLOT_Y,
                28.0,
            )
            .translate(x, y, PAD_NEST_Z - 12.0);
            let thumb_relief = centered_cylinder(
                format!("gripper_pad_coupon_nest_thumb_relief_{i}"),
                8.0,
                18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - PAD_SLOT_Y / 2.0 + 2.0, PAD_NEST_Z - 10.0);
            slots = slots + slot + thumb_relief;
        }
    }
    slots
}

fn pad_coupon_locator_lips() -> Part {
    let mut lips = Part::empty("gripper_pad_coupon_nest_locator_lips");
    for row in 0..PAD_ROWS {
        for col in 0..PAD_COLS {
            let i = row * PAD_COLS + col;
            let x = centered_index(col, PAD_COLS, PAD_PITCH_X);
            let y = centered_index(row, PAD_ROWS, PAD_PITCH_Y) + 2.0;
            let left = centered_cube(
                format!("gripper_pad_coupon_nest_left_locator_lip_{i}"),
                5.0,
                PAD_SLOT_Y + 12.0,
                14.0,
            )
            .translate(x - PAD_SLOT_X / 2.0 - 5.0, y, PAD_NEST_Z + 7.0);
            let right = centered_cube(
                format!("gripper_pad_coupon_nest_right_locator_lip_{i}"),
                5.0,
                PAD_SLOT_Y + 12.0,
                14.0,
            )
            .translate(x + PAD_SLOT_X / 2.0 + 5.0, y, PAD_NEST_Z + 7.0);
            let rear_stop = centered_cube(
                format!("gripper_pad_coupon_nest_rear_pad_stop_{i}"),
                PAD_SLOT_X + 18.0,
                5.0,
                16.0,
            )
            .translate(x, y + PAD_SLOT_Y / 2.0 + 7.0, PAD_NEST_Z + 8.0);
            lips = lips + left + right + rear_stop;
        }
    }
    lips
}

fn pad_coupon_orientation_keys() -> Part {
    let mut keys = Part::empty("gripper_pad_coupon_nest_orientation_keys");
    for row in 0..PAD_ROWS {
        for col in 0..PAD_COLS {
            let i = row * PAD_COLS + col;
            let x = centered_index(col, PAD_COLS, PAD_PITCH_X);
            let y = centered_index(row, PAD_ROWS, PAD_PITCH_Y) + 2.0;
            let key = centered_cube(
                format!("gripper_pad_coupon_nest_asymmetric_key_land_{i}"),
                17.0,
                7.0,
                5.0,
            )
            .translate(x - 12.0, y - PAD_SLOT_Y / 2.0 - 10.0, PAD_NEST_Z + 2.5);
            let witness_pin = centered_cylinder(
                format!("gripper_pad_coupon_nest_preload_witness_pin_{i}"),
                3.0,
                7.0,
                18,
            )
            .translate(x + 15.0, y - PAD_SLOT_Y / 2.0 - 10.0, PAD_NEST_Z + 3.5);
            keys = keys + key + witness_pin;
        }
    }
    keys
}

fn pad_coupon_id_lands() -> Part {
    let mut lands = Part::empty("gripper_pad_coupon_nest_id_lands");
    for col in 0..PAD_COLS {
        let x = centered_index(col, PAD_COLS, PAD_PITCH_X);
        lands = lands
            + centered_cube(
                format!("gripper_pad_coupon_nest_column_barcode_land_{col}"),
                48.0,
                13.0,
                3.0,
            )
            .translate(x, -PAD_NEST_Y / 2.0 + 17.0, PAD_NEST_Z + 1.5);
    }
    lands
}

fn repeated_contact_witness_rails() -> Part {
    let body = module_panel(
        "gripper_pad_repeated_contact_witness_rail_panel",
        WITNESS_RAIL_X,
        WITNESS_RAIL_Y,
        WITNESS_RAIL_Z,
    );
    let debris_recess = centered_cube(
        "gripper_pad_repeated_contact_witness_debris_recess",
        WITNESS_RAIL_X - 42.0,
        WITNESS_RAIL_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, WITNESS_RAIL_Z - 4.0);

    body - debris_recess
        + contact_rails()
        + contact_tick_lands()
        + rail_end_stops()
        + gripper_pick_fiducials(
            "repeated_contact_witness_rails",
            WITNESS_RAIL_X,
            WITNESS_RAIL_Y,
            WITNESS_RAIL_Z,
        )
}

fn contact_rails() -> Part {
    let mut rails = Part::empty("gripper_pad_repeated_contact_rails");
    for i in 0..CONTACT_RAIL_COUNT {
        let x = centered_index(i, CONTACT_RAIL_COUNT, 78.0);
        let base = centered_cube(
            format!("gripper_pad_repeated_contact_flat_rail_base_{i}"),
            38.0,
            WITNESS_RAIL_Y - 44.0,
            12.0,
        )
        .translate(x, 0.0, WITNESS_RAIL_Z + 6.0);
        let crown = centered_cylinder(
            format!("gripper_pad_repeated_contact_rounded_witness_crown_{i}"),
            7.0 + i as f64,
            WITNESS_RAIL_Y - 54.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, WITNESS_RAIL_Z + 15.0);
        let collection_shadow = centered_cube(
            format!("gripper_pad_repeated_contact_side_shed_shadow_{i}"),
            58.0,
            8.0,
            6.0,
        )
        .translate(x, -WITNESS_RAIL_Y / 2.0 + 29.0, WITNESS_RAIL_Z + 3.0);
        rails = rails + base + crown + collection_shadow;
    }
    rails
}

fn contact_tick_lands() -> Part {
    let mut ticks = Part::empty("gripper_pad_repeated_contact_tick_lands");
    for rail in 0..CONTACT_RAIL_COUNT {
        let x = centered_index(rail, CONTACT_RAIL_COUNT, 78.0);
        for tick in 0..CONTACT_TICK_COUNT {
            let y = centered_index(tick, CONTACT_TICK_COUNT, CONTACT_TICK_PITCH);
            ticks = ticks
                + centered_cube(
                    format!("gripper_pad_repeated_contact_rail_{rail}_tick_{tick}"),
                    47.0,
                    2.5,
                    4.0,
                )
                .translate(x, y, WITNESS_RAIL_Z + 25.0);
        }
    }
    ticks
}

fn rail_end_stops() -> Part {
    let front = centered_cube(
        "gripper_pad_repeated_contact_front_stroke_stop",
        WITNESS_RAIL_X - 44.0,
        10.0,
        28.0,
    )
    .translate(0.0, -WITNESS_RAIL_Y / 2.0 + 18.0, WITNESS_RAIL_Z + 14.0);
    let rear = centered_cube(
        "gripper_pad_repeated_contact_rear_stroke_stop",
        WITNESS_RAIL_X - 44.0,
        10.0,
        34.0,
    )
    .translate(0.0, WITNESS_RAIL_Y / 2.0 - 18.0, WITNESS_RAIL_Z + 17.0);
    let stroke_counter_land = centered_cube(
        "gripper_pad_repeated_contact_stroke_counter_land",
        92.0,
        20.0,
        8.0,
    )
    .translate(WITNESS_RAIL_X / 2.0 - 72.0, 0.0, WITNESS_RAIL_Z + 4.0);
    front + rear + stroke_counter_land
}

fn particle_collection_troughs() -> Part {
    let body = module_panel(
        "gripper_pad_particle_collection_trough_panel",
        TROUGH_X,
        TROUGH_Y,
        TROUGH_Z,
    );
    let gasket = gasket_frame(
        "gripper_pad_particle_collection_trough_lid_gasket",
        TROUGH_X - 30.0,
        TROUGH_Y - 28.0,
        7.0,
        8.0,
    )
    .translate(0.0, 0.0, TROUGH_Z + 2.0);

    body - trough_recess_cuts()
        + trough_liner_lips()
        + trough_grid_ticks()
        + trough_drain_witness_ports()
        + gasket
        + gripper_pick_fiducials("particle_collection_troughs", TROUGH_X, TROUGH_Y, TROUGH_Z)
}

fn trough_recess_cuts() -> Part {
    let mut cuts = Part::empty("gripper_pad_particle_collection_trough_recess_cuts");
    for i in 0..PARTICLE_TROUGH_COUNT {
        let x = centered_index(i, PARTICLE_TROUGH_COUNT, 94.0);
        let cut = centered_cube(
            format!("gripper_pad_particle_collection_trough_recess_{i}"),
            70.0,
            TROUGH_Y - 54.0,
            30.0,
        )
        .translate(x, 0.0, TROUGH_Z - 13.0);
        let front_spout = centered_cylinder(
            format!("gripper_pad_particle_collection_trough_spout_cut_{i}"),
            5.0,
            24.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -TROUGH_Y / 2.0 + 25.0, TROUGH_Z - 12.0);
        cuts = cuts + cut + front_spout;
    }
    cuts
}

fn trough_liner_lips() -> Part {
    let mut lips = Part::empty("gripper_pad_particle_collection_trough_liner_lips");
    for i in 0..PARTICLE_TROUGH_COUNT {
        let x = centered_index(i, PARTICLE_TROUGH_COUNT, 94.0);
        let left = centered_cube(
            format!("gripper_pad_particle_collection_left_liner_lip_{i}"),
            5.0,
            TROUGH_Y - 44.0,
            14.0,
        )
        .translate(x - 41.0, 0.0, TROUGH_Z + 7.0);
        let right = centered_cube(
            format!("gripper_pad_particle_collection_right_liner_lip_{i}"),
            5.0,
            TROUGH_Y - 44.0,
            14.0,
        )
        .translate(x + 41.0, 0.0, TROUGH_Z + 7.0);
        let rear = centered_cube(
            format!("gripper_pad_particle_collection_rear_liner_stop_{i}"),
            82.0,
            5.0,
            16.0,
        )
        .translate(x, TROUGH_Y / 2.0 - 27.0, TROUGH_Z + 8.0);
        lips = lips + left + right + rear;
    }
    lips
}

fn trough_grid_ticks() -> Part {
    let mut ticks = Part::empty("gripper_pad_particle_collection_count_grid_ticks");
    for trough in 0..PARTICLE_TROUGH_COUNT {
        let x = centered_index(trough, PARTICLE_TROUGH_COUNT, 94.0);
        for tick in 0..TROUGH_GRID_TICKS {
            let y = centered_index(tick, TROUGH_GRID_TICKS, 19.0);
            ticks = ticks
                + centered_cube(
                    format!("gripper_pad_particle_collection_trough_{trough}_grid_tick_{tick}"),
                    54.0,
                    2.0,
                    3.0,
                )
                .translate(x, y, TROUGH_Z + 1.5);
        }
    }
    ticks
}

fn trough_drain_witness_ports() -> Part {
    let mut ports = Part::empty("gripper_pad_particle_collection_trough_drain_witness_ports");
    for i in 0..PARTICLE_TROUGH_COUNT {
        let x = centered_index(i, PARTICLE_TROUGH_COUNT, 94.0);
        let collar = centered_cylinder(
            format!("gripper_pad_particle_collection_drain_port_collar_{i}"),
            11.0,
            5.0,
            28,
        )
        .translate(x, -TROUGH_Y / 2.0 + 20.0, TROUGH_Z + 2.5);
        let bore = centered_cylinder(
            format!("gripper_pad_particle_collection_drain_port_bore_{i}"),
            4.0,
            7.0,
            22,
        )
        .translate(x, -TROUGH_Y / 2.0 + 20.0, TROUGH_Z + 2.5);
        ports = ports + (collar - bore);
    }
    ports
}

fn cassette_surrogate_contact_block() -> Part {
    let plate_z = 28.0;
    let plate = module_panel(
        "gripper_pad_cassette_surrogate_contact_base_plate",
        CONTACT_BLOCK_X,
        CONTACT_BLOCK_Y,
        plate_z,
    );
    let surrogate = centered_cube(
        "gripper_pad_cassette_surrogate_contact_block_body",
        CONTACT_BLOCK_X - 56.0,
        CONTACT_BLOCK_Y - 58.0,
        CONTACT_BLOCK_Z - plate_z,
    )
    .translate(0.0, 0.0, plate_z + (CONTACT_BLOCK_Z - plate_z) / 2.0);
    let recess = centered_cube(
        "gripper_pad_cassette_surrogate_wear_face_relief",
        CONTACT_BLOCK_X - 96.0,
        CONTACT_BLOCK_Y - 98.0,
        14.0,
    )
    .translate(0.0, 0.0, CONTACT_BLOCK_Z - 6.0);

    plate
        + (surrogate - recess)
        + cassette_contact_strips()
        + cassette_datum_bosses()
        + cassette_wear_witness_flags()
        + gripper_pick_fiducials(
            "cassette_surrogate_contact_block",
            CONTACT_BLOCK_X,
            CONTACT_BLOCK_Y,
            CONTACT_BLOCK_Z,
        )
}

fn cassette_contact_strips() -> Part {
    let mut strips = Part::empty("gripper_pad_cassette_surrogate_contact_strips");
    for i in 0..CASSETTE_CONTACT_STRIPS {
        let y = centered_index(i, CASSETTE_CONTACT_STRIPS, 24.0);
        let strip = centered_cube(
            format!("gripper_pad_cassette_surrogate_replaceable_contact_strip_{i}"),
            CONTACT_BLOCK_X - 92.0,
            9.0,
            10.0,
        )
        .translate(0.0, y, CONTACT_BLOCK_Z + 5.0);
        let witness_crown = centered_cylinder(
            format!("gripper_pad_cassette_surrogate_strip_witness_crown_{i}"),
            5.0,
            CONTACT_BLOCK_X - 104.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, CONTACT_BLOCK_Z + 12.0);
        strips = strips + strip + witness_crown;
    }
    strips
}

fn cassette_datum_bosses() -> Part {
    let mut bosses = Part::empty("gripper_pad_cassette_surrogate_datum_bosses");
    for (i, (x, y)) in [
        (-CONTACT_BLOCK_X / 2.0 + 58.0, -CONTACT_BLOCK_Y / 2.0 + 38.0),
        (CONTACT_BLOCK_X / 2.0 - 58.0, -CONTACT_BLOCK_Y / 2.0 + 38.0),
        (-CONTACT_BLOCK_X / 2.0 + 58.0, CONTACT_BLOCK_Y / 2.0 - 38.0),
        (CONTACT_BLOCK_X / 2.0 - 58.0, CONTACT_BLOCK_Y / 2.0 - 38.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("gripper_pad_cassette_surrogate_datum_receiver_boss_{i}"),
            12.0,
            8.0,
            32,
        )
        .translate(x, y, 32.0);
        let bore = centered_cylinder(
            format!("gripper_pad_cassette_surrogate_datum_receiver_bore_{i}"),
            DATUM_PIN_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, 32.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn cassette_wear_witness_flags() -> Part {
    let mut flags = Part::empty("gripper_pad_cassette_surrogate_wear_witness_flags");
    for i in 0..6 {
        let x = centered_index(i, 6, 42.0);
        flags = flags
            + centered_cube(
                format!("gripper_pad_cassette_surrogate_edge_wear_flag_{i}"),
                7.0,
                28.0,
                34.0,
            )
            .translate(x, -CONTACT_BLOCK_Y / 2.0 + 18.0, 45.0);
    }
    flags
}

fn force_load_witness_pockets() -> Part {
    let body = module_panel(
        "gripper_pad_force_load_witness_pocket_panel",
        FORCE_POCKET_X,
        FORCE_POCKET_Y,
        FORCE_POCKET_Z,
    );
    body - force_pocket_cuts()
        + force_pocket_rims()
        + force_film_slots()
        + load_scale_lands()
        + gripper_pick_fiducials(
            "force_load_witness_pockets",
            FORCE_POCKET_X,
            FORCE_POCKET_Y,
            FORCE_POCKET_Z,
        )
}

fn force_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gripper_pad_force_load_witness_pocket_cuts");
    for i in 0..FORCE_POCKET_COUNT {
        let x = centered_index(i % 3, 3, 86.0);
        let y = centered_index(i / 3, 2, 70.0);
        cuts = cuts
            + centered_cylinder(
                format!("gripper_pad_force_load_witness_round_pocket_cut_{i}"),
                LOAD_WITNESS_POCKET_D / 2.0 + i as f64 * 0.6,
                24.0,
                42,
            )
            .translate(x, y, FORCE_POCKET_Z - 10.0);
    }
    cuts
}

fn force_pocket_rims() -> Part {
    let mut rims = Part::empty("gripper_pad_force_load_witness_pocket_rims");
    for i in 0..FORCE_POCKET_COUNT {
        let x = centered_index(i % 3, 3, 86.0);
        let y = centered_index(i / 3, 2, 70.0);
        let radius = LOAD_WITNESS_POCKET_D / 2.0 + i as f64 * 0.6;
        let outer = centered_cylinder(
            format!("gripper_pad_force_load_witness_pocket_outer_rim_{i}"),
            radius + 5.0,
            5.0,
            42,
        )
        .translate(x, y, FORCE_POCKET_Z + 2.5);
        let inner = centered_cylinder(
            format!("gripper_pad_force_load_witness_pocket_inner_relief_{i}"),
            radius - 1.0,
            7.0,
            42,
        )
        .translate(x, y, FORCE_POCKET_Z + 2.5);
        let clip = centered_cube(
            format!("gripper_pad_force_load_witness_retention_clip_{i}"),
            radius * 1.8,
            7.0,
            9.0,
        )
        .translate(x, y - 28.0, FORCE_POCKET_Z + 4.5);
        rims = rims + (outer - inner) + clip;
    }
    rims
}

fn force_film_slots() -> Part {
    let mut slots = Part::empty("gripper_pad_force_film_witness_slots");
    for i in 0..LOAD_FILM_SLOT_COUNT {
        let y = centered_index(i, LOAD_FILM_SLOT_COUNT, 28.0);
        let slot = centered_cube(
            format!("gripper_pad_force_film_witness_slot_land_{i}"),
            84.0,
            12.0,
            4.0,
        )
        .translate(FORCE_POCKET_X / 2.0 - 62.0, y, FORCE_POCKET_Z + 2.0);
        let stop = centered_cube(
            format!("gripper_pad_force_film_witness_slot_end_stop_{i}"),
            8.0,
            18.0,
            12.0,
        )
        .translate(FORCE_POCKET_X / 2.0 - 106.0, y, FORCE_POCKET_Z + 6.0);
        slots = slots + slot + stop;
    }
    slots
}

fn load_scale_lands() -> Part {
    let mut lands = Part::empty("gripper_pad_force_load_scale_lands");
    for i in 0..7 {
        lands = lands
            + centered_cube(
                format!("gripper_pad_force_load_scale_tick_{i}"),
                4.0,
                24.0,
                3.0,
            )
            .translate(
                -FORCE_POCKET_X / 2.0 + 42.0 + i as f64 * 18.0,
                -FORCE_POCKET_Y / 2.0 + 23.0,
                FORCE_POCKET_Z + 1.5,
            );
    }
    lands
}

fn clean_used_pad_segregation() -> Part {
    let body = module_panel(
        "gripper_pad_clean_used_segregation_tray_body",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let clean_recess = centered_cube(
        "gripper_pad_clean_used_segregation_clean_side_recess",
        SEGREGATION_X / 2.0 - 34.0,
        SEGREGATION_Y - 44.0,
        24.0,
    )
    .translate(-SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z - 10.0);
    let used_recess = centered_cube(
        "gripper_pad_clean_used_segregation_used_side_recess",
        SEGREGATION_X / 2.0 - 34.0,
        SEGREGATION_Y - 44.0,
        30.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z - 13.0);
    body - clean_recess - used_recess
        + segregation_divider()
        + clean_pad_slot_rack()
        + used_pad_slot_bin()
        + one_way_used_pad_chute()
        + gripper_pick_fiducials(
            "clean_used_pad_segregation",
            SEGREGATION_X,
            SEGREGATION_Y,
            SEGREGATION_Z,
        )
}

fn segregation_divider() -> Part {
    centered_cube(
        "gripper_pad_clean_used_segregation_high_center_divider",
        12.0,
        SEGREGATION_Y - 26.0,
        SEGREGATION_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_DIVIDER_Z / 2.0)
}

fn clean_pad_slot_rack() -> Part {
    let mut rack = Part::empty("gripper_pad_clean_side_slot_rack");
    for i in 0..CLEAN_PAD_SLOTS {
        let y = centered_index(i % 3, 3, 42.0);
        let x = -SEGREGATION_X / 4.0 + centered_index(i / 3, 2, 46.0);
        rack = rack
            + centered_cube(
                format!("gripper_pad_clean_side_pad_slot_lip_{i}"),
                32.0,
                5.0,
                24.0,
            )
            .translate(x, y - 20.0, SEGREGATION_Z + 12.0)
            + centered_cube(
                format!("gripper_pad_clean_side_pad_slot_rear_stop_{i}"),
                32.0,
                5.0,
                24.0,
            )
            .translate(x, y + 20.0, SEGREGATION_Z + 12.0);
    }
    rack
}

fn used_pad_slot_bin() -> Part {
    let mut bin = Part::empty("gripper_pad_used_side_slot_bin");
    for i in 0..USED_PAD_SLOTS {
        let y = centered_index(i % 3, 3, 42.0);
        let x = SEGREGATION_X / 4.0 + centered_index(i / 3, 2, 46.0);
        let cup = centered_cylinder(
            format!("gripper_pad_used_side_particle_pad_cup_{i}"),
            15.0,
            18.0,
            32,
        )
        .translate(x, y, SEGREGATION_Z + 9.0);
        let inner = centered_cylinder(
            format!("gripper_pad_used_side_particle_pad_cup_inner_{i}"),
            10.0,
            20.0,
            32,
        )
        .translate(x, y, SEGREGATION_Z + 10.0);
        bin = bin + (cup - inner);
    }
    bin
}

fn one_way_used_pad_chute() -> Part {
    let chute = centered_cube(
        "gripper_pad_used_side_one_way_return_chute",
        104.0,
        24.0,
        28.0,
    )
    .translate(
        SEGREGATION_X / 4.0,
        -SEGREGATION_Y / 2.0 + 26.0,
        SEGREGATION_Z + 14.0,
    );
    let witness_window = centered_cube(
        "gripper_pad_used_side_chute_witness_window",
        68.0,
        28.0,
        12.0,
    )
    .translate(
        SEGREGATION_X / 4.0,
        -SEGREGATION_Y / 2.0 + 26.0,
        SEGREGATION_Z + 14.0,
    );
    let reject_flag = centered_cube(
        "gripper_pad_used_side_red_tag_quarantine_flag_land",
        64.0,
        12.0,
        5.0,
    )
    .translate(
        SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 22.0,
        SEGREGATION_Z + 2.5,
    );
    (chute - witness_window) + reject_flag
}

fn barcode_certificate_lands() -> Part {
    let panel = module_panel(
        "gripper_pad_barcode_certificate_land_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("gripper_pad_barcode_certificate_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 58.0);
        let y = centered_index(i / 4, 2, 38.0) + 16.0;
        let land = centered_cube(
            format!("gripper_pad_barcode_scan_land_{i}"),
            42.0,
            22.0,
            3.0,
        )
        .translate(x, y, TRACE_Z + 1.5);
        lands = lands + land + barcode_stripes("gripper_pad_barcode", i, x, y, TRACE_Z + 3.2);
    }

    let mut certs = Part::empty("gripper_pad_certificate_laminate_lands");
    for i in 0..CERTIFICATE_LANDS {
        certs = certs
            + centered_cube(
                format!("gripper_pad_certificate_laminate_land_{i}"),
                82.0,
                24.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LANDS, 94.0),
                -TRACE_Y / 2.0 + 24.0,
                TRACE_Z + 1.5,
            );
    }

    panel + lands + certs + certificate_punch_bank()
}

fn certificate_punch_bank() -> Part {
    let mut bank = Part::empty("gripper_pad_certificate_witness_punch_bank");
    for i in 0..6 {
        bank = bank
            + centered_cylinder(
                format!("gripper_pad_certificate_witness_punch_{i}"),
                4.0,
                3.0,
                22,
            )
            .translate(
                TRACE_X / 2.0 - 42.0,
                centered_index(i, 6, 18.0),
                TRACE_Z + 1.5,
            );
    }
    bank
}

fn release_hold_reject_lanes() -> Part {
    let panel = module_panel(
        "gripper_pad_release_hold_reject_lane_panel",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    panel - status_lane_recesses() + status_lane_floors() + status_lane_gates() + status_dividers()
}

fn status_lane_recesses() -> Part {
    let mut recesses = Part::empty("gripper_pad_release_hold_reject_lane_recesses");
    for (i, _) in ["release", "hold", "reject"].into_iter().enumerate() {
        let x = centered_index(i, STATUS_LANES, 98.0);
        recesses = recesses
            + centered_cube(
                format!("gripper_pad_status_lane_recess_{i}"),
                74.0,
                STATUS_Y - 32.0,
                14.0,
            )
            .translate(x, 0.0, STATUS_Z - 6.0);
    }
    recesses
}

fn status_lane_floors() -> Part {
    let mut floors = Part::empty("gripper_pad_release_hold_reject_lane_floors");
    for (lane, name) in ["release", "hold", "reject"].into_iter().enumerate() {
        let x = centered_index(lane, STATUS_LANES, 98.0);
        floors = floors
            + centered_cube(
                format!("gripper_pad_{name}_lane_witness_floor"),
                62.0,
                STATUS_Y - 44.0,
                3.0,
            )
            .translate(x, 0.0, STATUS_Z + 1.5);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            floors = floors
                + centered_cube(
                    format!("gripper_pad_{name}_lane_pad_status_slot_{slot}"),
                    50.0,
                    8.0,
                    5.0,
                )
                .translate(
                    x,
                    centered_index(slot, STATUS_SLOTS_PER_LANE, 24.0),
                    STATUS_Z + 4.0,
                );
        }
    }
    floors
}

fn status_lane_gates() -> Part {
    let mut gates = Part::empty("gripper_pad_release_hold_reject_lane_gates");
    for (i, name) in ["release", "hold", "reject"].into_iter().enumerate() {
        let x = centered_index(i, STATUS_LANES, 98.0);
        let gate_z = 22.0 + i as f64 * 10.0;
        gates = gates
            + centered_cube(
                format!("gripper_pad_{name}_lane_rear_disposition_gate"),
                62.0,
                8.0,
                gate_z,
            )
            .translate(x, STATUS_Y / 2.0 - 20.0, STATUS_Z + gate_z / 2.0)
            + centered_cube(
                format!("gripper_pad_{name}_lane_front_lip"),
                66.0,
                8.0,
                16.0,
            )
            .translate(x, -STATUS_Y / 2.0 + 16.0, STATUS_Z + 8.0);
    }
    gates
}

fn status_dividers() -> Part {
    centered_cube(
        "gripper_pad_status_left_lane_divider",
        7.0,
        STATUS_Y - 22.0,
        34.0,
    )
    .translate(-49.0, 0.0, STATUS_Z + 17.0)
        + centered_cube(
            "gripper_pad_status_right_lane_divider",
            7.0,
            STATUS_Y - 22.0,
            34.0,
        )
        .translate(49.0, 0.0, STATUS_Z + 17.0)
}

fn transparent_evidence_bridge() -> Part {
    let mut posts = Part::empty("gripper_pad_transparent_evidence_bridge_posts");
    for (i, (x, y)) in [
        (-BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (-BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("gripper_pad_transparent_evidence_bridge_post_{i}"),
            26.0,
            26.0,
            BRIDGE_UNDERSIDE_Z,
        )
        .translate(x, y, BRIDGE_UNDERSIDE_Z / 2.0);
        let foot = centered_cube(
            format!("gripper_pad_transparent_evidence_bridge_foot_{i}"),
            58.0,
            44.0,
            10.0,
        )
        .translate(x, y, 5.0);
        posts = posts + post + foot;
    }

    let front = centered_cube(
        "gripper_pad_transparent_evidence_bridge_front_beam",
        BRIDGE_SPAN_X + 36.0,
        24.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "gripper_pad_transparent_evidence_bridge_rear_beam",
        BRIDGE_SPAN_X + 36.0,
        24.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        "gripper_pad_transparent_evidence_bridge_left_beam",
        24.0,
        BRIDGE_SPAN_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        "gripper_pad_transparent_evidence_bridge_right_beam",
        24.0,
        BRIDGE_SPAN_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let transparent_window = evidence_window_frame();
    let camera_land = centered_cube(
        "gripper_pad_transparent_evidence_bridge_camera_land",
        124.0,
        74.0,
        18.0,
    )
    .translate(0.0, -BRIDGE_SPAN_Y / 2.0 + 94.0, BRIDGE_UNDERSIDE_Z - 9.0);
    let lens_clearance = centered_cylinder(
        "gripper_pad_transparent_evidence_bridge_lens_clearance",
        18.0,
        22.0,
        42,
    )
    .translate(0.0, -BRIDGE_SPAN_Y / 2.0 + 94.0, BRIDGE_UNDERSIDE_Z - 9.0);

    posts
        + front
        + rear
        + left
        + right
        + transparent_window
        + (camera_land - lens_clearance)
        + evidence_fiducials()
}

fn evidence_window_frame() -> Part {
    let outer = centered_cube(
        "gripper_pad_transparent_evidence_bridge_clear_panel_outer_frame",
        BRIDGE_WINDOW_X,
        BRIDGE_WINDOW_Y,
        8.0,
    );
    let inner = centered_cube(
        "gripper_pad_transparent_evidence_bridge_clear_panel_view_window",
        BRIDGE_WINDOW_X - 72.0,
        BRIDGE_WINDOW_Y - 54.0,
        10.0,
    );
    let cross_x = centered_cube(
        "gripper_pad_transparent_evidence_bridge_clear_panel_cross_x",
        BRIDGE_WINDOW_X - 92.0,
        8.0,
        8.0,
    );
    let cross_y = centered_cube(
        "gripper_pad_transparent_evidence_bridge_clear_panel_cross_y",
        8.0,
        BRIDGE_WINDOW_Y - 74.0,
        8.0,
    );
    ((outer - inner) + cross_x + cross_y).translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z - 22.0)
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("gripper_pad_transparent_evidence_bridge_fiducials");
    for i in 0..EVIDENCE_FIDUCIAL_COUNT {
        let x = centered_index(i, EVIDENCE_FIDUCIAL_COUNT, 76.0);
        let y = if i % 2 == 0 { -26.0 } else { 26.0 };
        fiducials = fiducials
            + fiducial_disc(&format!("gripper_pad_evidence_bridge_fiducial_{i}")).translate(
                x,
                y,
                BRIDGE_UNDERSIDE_Z - 17.0,
            );
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let front_sweep = centered_cube(
        "gripper_pad_robot_keepout_front_sweep_gauge",
        STATION_X - 144.0,
        12.0,
        36.0,
    )
    .translate(0.0, -ROBOT_SWEEP_CLEARANCE_Y / 2.0, 18.0);
    let rear_service = centered_cube(
        "gripper_pad_robot_keepout_rear_service_depth_gauge",
        STATION_X - 180.0,
        12.0,
        42.0,
    )
    .translate(0.0, STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE, 21.0);
    let left_side = centered_cube(
        "gripper_pad_robot_keepout_left_pad_load_gauge",
        12.0,
        STATION_Y - 164.0,
        38.0,
    )
    .translate(-STATION_X / 2.0 + SIDE_PAD_LOAD_CLEARANCE, 0.0, 19.0);
    let right_side = centered_cube(
        "gripper_pad_robot_keepout_right_pad_load_gauge",
        12.0,
        STATION_Y - 164.0,
        38.0,
    )
    .translate(STATION_X / 2.0 - SIDE_PAD_LOAD_CLEARANCE, 0.0, 19.0);
    let overhead = overhead_clearance_gauges();
    front_sweep + rear_service + left_side + right_side + overhead + keepout_label_ticks()
}

fn overhead_clearance_gauges() -> Part {
    let mut gauges = Part::empty("gripper_pad_robot_keepout_overhead_gauges");
    for (i, (x, y)) in [
        (-470.0, -280.0),
        (470.0, -280.0),
        (-470.0, 280.0),
        (470.0, 280.0),
        (-120.0, -280.0),
        (120.0, -280.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mast = centered_cube(
            format!("gripper_pad_robot_keepout_z_clearance_mast_{i}"),
            18.0,
            18.0,
            118.0,
        )
        .translate(x, y, 59.0);
        let flag = centered_cube(
            format!("gripper_pad_robot_keepout_z_clearance_flag_{i}"),
            56.0,
            10.0,
            16.0,
        )
        .translate(x, y, 118.0);
        gauges = gauges + mast + flag;
    }
    gauges
}

fn keepout_label_ticks() -> Part {
    let mut ticks = Part::empty("gripper_pad_robot_service_keepout_label_ticks");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        ticks = ticks
            + centered_cube(
                format!("gripper_pad_robot_service_keepout_tick_{i}"),
                48.0,
                5.0,
                4.0,
            )
            .translate(
                centered_index(i, KEEP_OUT_GAUGE_COUNT, 128.0),
                -STATION_Y / 2.0 + 76.0,
                2.0,
            );
    }
    ticks
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

fn gripper_pick_fiducials(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    fiducial_disc(&format!("{prefix}_left_gripper_fiducial")).translate(
        -x / 2.0 + 28.0,
        -y / 2.0 + 26.0,
        z + 1.5,
    ) + fiducial_disc(&format!("{prefix}_right_gripper_fiducial")).translate(
        x / 2.0 - 28.0,
        -y / 2.0 + 26.0,
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
    for (bar, dx) in [-14.0, -9.0, -3.0, 4.0, 11.0, 15.0].into_iter().enumerate() {
        let width = if bar % 2 == 0 { 2.2 } else { 1.1 };
        stripes =
            stripes
                + centered_cube(format!("{prefix}_{index}_stripe_{bar}"), width, 18.0, 1.2)
                    .translate(x + dx, y, z);
    }
    stripes
}
