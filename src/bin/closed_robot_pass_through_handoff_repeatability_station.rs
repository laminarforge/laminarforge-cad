use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robot pass-through handoff repeatability station.
//
// Design intent:
// - Validate repeatable robot handoff of sealed cassettes or totes across a
//   pass-through/airlock boundary without losing datum alignment.
// - Keep custody visible with barcode/RFID lands, tamper-seal pockets, latch
//   witness slots, evidence camera geometry, and pass/fail/hold segregation.
// - Model mechanical validation fixtures only. This is not an aseptic-process
//   release protocol, a custody policy, or a sterility assurance claim.

const OUTPUTS: &[&str] = &[
    "output/closed_robot_pass_through_handoff_repeatability_station_base_airlock_deck.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_dual_side_handoff_nests.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_datum_pin_arrays.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_latch_witness_slots.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_force_tilt_logger_pockets.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_barcode_rfid_scan_lands.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_pass_fail_hold_lanes.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_tamper_seal_pockets.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_collision_keepout_gauges.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_clean_used_segregation.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_evidence_camera_bridge.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_robot_service_keepouts.stl",
    "output/closed_robot_pass_through_handoff_repeatability_station_assembly.stl",
];

const DECK_X: f64 = 1680.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const AIRLOCK_WALL_Y: f64 = 34.0;
const AIRLOCK_WALL_Z: f64 = 168.0;
const AIRLOCK_SLOT_X: f64 = 620.0;
const AIRLOCK_SLOT_Z: f64 = 112.0;

const OPERATOR_SIDE_Y: f64 = -292.0;
const ROBOT_SIDE_Y: f64 = 292.0;
const NEST_X: f64 = 455.0;
const NEST_Y: f64 = 252.0;
const NEST_Z: f64 = 54.0;
const CASSETTE_CLEAR_X: f64 = 354.0;
const CASSETTE_CLEAR_Y: f64 = 158.0;
const TOTE_CLEAR_X: f64 = 515.0;
const TOTE_CLEAR_Y: f64 = 260.0;
const NESTS_PER_SIDE: usize = 2;
const NEST_PITCH_X: f64 = 520.0;

const DATUM_PIN_ROWS: usize = 3;
const DATUM_PIN_COLS: usize = 4;
const DATUM_PIN_D: f64 = 10.0;
const DATUM_PIN_Z: f64 = 32.0;
const DATUM_ARRAY_PITCH_X: f64 = 66.0;
const DATUM_ARRAY_PITCH_Y: f64 = 54.0;

const LATCH_SLOT_COUNT: usize = 10;
const LATCH_SLOT_X: f64 = 42.0;
const LATCH_SLOT_Y: f64 = 15.0;
const LATCH_SLOT_Z: f64 = 18.0;

const LOGGER_POCKET_COUNT: usize = 6;
const LOGGER_POCKET_X: f64 = 118.0;
const LOGGER_POCKET_Y: f64 = 78.0;
const LOGGER_POCKET_Z: f64 = 34.0;
const TILT_RAMP_COUNT: usize = 5;

const BARCODE_LANDS: usize = 14;
const RFID_LANDS: usize = 8;
const CUSTODY_CARD_LANDS: usize = 6;

const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 150.0;
const STATUS_SLOT_Y: f64 = 96.0;
const STATUS_SLOT_Z: f64 = 30.0;

const TAMPER_POCKET_COUNT: usize = 12;
const SEAL_POCKET_D: f64 = 34.0;
const SEAL_POCKET_Z: f64 = 18.0;

const COLLISION_GAUGE_COUNT: usize = 8;
const KEEP_OUT_GAUGE_Z: f64 = 142.0;
const GANTRY_CLEARANCE_Z: f64 = 348.0;

const CAMERA_BRIDGE_SPAN_X: f64 = 1230.0;
const CAMERA_BRIDGE_CLEAR_Z: f64 = 315.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;

const ROBOT_APPROACH_KEEP_OUT_Y: f64 = 430.0;
const SERVICE_DOOR_KEEP_OUT_Y: f64 = 360.0;
const SIDE_CART_KEEP_OUT_X: f64 = 260.0;
const OVERHEAD_SERVICE_Z: f64 = 520.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_airlock_deck();
    export(&base, OUTPUTS[0]);

    let nests = dual_side_handoff_nests();
    export(&nests, OUTPUTS[1]);

    let datums = datum_pin_arrays();
    export(&datums, OUTPUTS[2]);

    let latch = latch_witness_slots();
    export(&latch, OUTPUTS[3]);

    let loggers = force_tilt_logger_pockets();
    export(&loggers, OUTPUTS[4]);

    let scans = barcode_rfid_scan_lands();
    export(&scans, OUTPUTS[5]);

    let status = pass_fail_hold_lanes();
    export(&status, OUTPUTS[6]);

    let tamper = tamper_seal_pockets();
    export(&tamper, OUTPUTS[7]);

    let gauges = collision_keepout_gauges();
    export(&gauges, OUTPUTS[8]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[9]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + nests
        + datums
        + latch
        + loggers
        + scans
        + status
        + tamper
        + gauges
        + segregation
        + camera
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed robot pass-through handoff repeatability station: {DECK_X:.0} x {DECK_Y:.0} mm deck with {AIRLOCK_SLOT_X:.0} x {AIRLOCK_SLOT_Z:.0} mm pass-through datum slot."
    );
    println!(
        "Handoff fixtures: {} nests per side, {DATUM_PIN_ROWS}x{DATUM_PIN_COLS} datum pin arrays, {LATCH_SLOT_COUNT} latch witness slots.",
        NESTS_PER_SIDE
    );
    println!(
        "Custody evidence: {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {CUSTODY_CARD_LANDS} custody card lands, {TAMPER_POCKET_COUNT} tamper seal pockets."
    );
    println!(
        "Repeatability instrumentation: {LOGGER_POCKET_COUNT} force/tilt logger pockets, {TILT_RAMP_COUNT} tilt challenge ramps, {COLLISION_GAUGE_COUNT} collision gauges."
    );
    println!(
        "Keepouts: robot approach {ROBOT_APPROACH_KEEP_OUT_Y:.0} mm, service door {SERVICE_DOOR_KEEP_OUT_Y:.0} mm, side cart {SIDE_CART_KEEP_OUT_X:.0} mm, overhead {OVERHEAD_SERVICE_Z:.0} mm."
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(NESTS_PER_SIDE, 2);
    assert_eq!(DATUM_PIN_ROWS * DATUM_PIN_COLS, 12);
    assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 12);
    assert!(AIRLOCK_SLOT_X > TOTE_CLEAR_X);
    assert!(NEST_Y > TOTE_CLEAR_Y - 20.0);
    assert!(AIRLOCK_SLOT_Z > NEST_Z);
}

fn deck_top_z() -> f64 {
    DECK_Z
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn base_airlock_deck() -> Part {
    let deck = centered_cube("closed_handoff_base_airlock_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let wipe_basin = centered_cube(
        "closed_handoff_wipeable_recessed_basin",
        DECK_X - 160.0,
        DECK_Y - 154.0,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z - 2.5);
    let drain = centered_cylinder("closed_handoff_low_point_drain", 6.0, 48.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 106.0, -DECK_Y / 2.0 + 38.0, DECK_Z - 6.0);

    deck - wipe_basin - drain + perimeter_rim() + pass_through_airlock_wall() + mounting_holes()
}

fn perimeter_rim() -> Part {
    let front = centered_cube("closed_handoff_front_deck_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let rear = centered_cube("closed_handoff_rear_deck_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let left = centered_cube("closed_handoff_left_deck_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube("closed_handoff_right_deck_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn pass_through_airlock_wall() -> Part {
    let wall = centered_cube(
        "closed_handoff_pass_through_airlock_wall",
        DECK_X - 172.0,
        AIRLOCK_WALL_Y,
        AIRLOCK_WALL_Z,
    )
    .translate(0.0, 0.0, deck_top_z() + AIRLOCK_WALL_Z / 2.0);
    let slot = centered_cube(
        "closed_handoff_airlock_clear_transfer_slot",
        AIRLOCK_SLOT_X,
        AIRLOCK_WALL_Y + 10.0,
        AIRLOCK_SLOT_Z,
    )
    .translate(0.0, 0.0, deck_top_z() + 86.0);
    let gasket_land = rectangular_frame(
        "closed_handoff_airlock_gasket_land",
        AIRLOCK_SLOT_X + 122.0,
        10.0,
        AIRLOCK_SLOT_Z + 94.0,
        AIRLOCK_SLOT_X,
        AIRLOCK_SLOT_Z,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -AIRLOCK_WALL_Y / 2.0 - 8.0, deck_top_z() + 86.0);
    let opposing_gasket_land = rectangular_frame(
        "closed_handoff_opposing_airlock_gasket_land",
        AIRLOCK_SLOT_X + 122.0,
        10.0,
        AIRLOCK_SLOT_Z + 94.0,
        AIRLOCK_SLOT_X,
        AIRLOCK_SLOT_Z,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, AIRLOCK_WALL_Y / 2.0 + 8.0, deck_top_z() + 86.0);

    wall - slot + gasket_land + opposing_gasket_land
}

fn rectangular_frame(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, outer_z);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        inner_x,
        outer_y + 4.0,
        inner_z,
    );
    outer - inner
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("closed_handoff_mounting_holes");
    let xs = [
        -DECK_X / 2.0 + 82.0,
        -DECK_X / 4.0,
        0.0,
        DECK_X / 4.0,
        DECK_X / 2.0 - 82.0,
    ];
    let ys = [-DECK_Y / 2.0 + 70.0, DECK_Y / 2.0 - 70.0];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            holes = holes
                + centered_cylinder(format!("closed_handoff_m8_mount_{i}_{j}"), 4.4, 36.0, 28)
                    .translate(*x, *y, DECK_Z / 2.0);
        }
    }
    holes
}

fn dual_side_handoff_nests() -> Part {
    let mut nests = Part::empty("closed_handoff_dual_side_handoff_nests");
    for side in 0..2 {
        let y = if side == 0 {
            OPERATOR_SIDE_Y
        } else {
            ROBOT_SIDE_Y
        };
        let label = if side == 0 { "operator" } else { "robot" };
        for i in 0..NESTS_PER_SIDE {
            let x = centered_index(i, NESTS_PER_SIDE, NEST_PITCH_X);
            nests = nests + handoff_nest(label, i, x, y);
        }
    }
    nests
}

fn handoff_nest(side: &str, index: usize, x: f64, y: f64) -> Part {
    let base = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_datum_base"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(x, y, deck_top_z() + NEST_Z / 2.0);
    let cassette_pocket = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_cassette_recess"),
        CASSETTE_CLEAR_X,
        CASSETTE_CLEAR_Y,
        NEST_Z + 5.0,
    )
    .translate(x - 34.0, y, deck_top_z() + NEST_Z / 2.0 + 7.0);
    let tote_envelope_gauge = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_sealed_tote_envelope_gauge"),
        TOTE_CLEAR_X,
        18.0,
        96.0,
    )
    .translate(x, y + NEST_Y / 2.0 + 20.0, deck_top_z() + 48.0);
    let left_rail = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_left_lead_in_rail"),
        18.0,
        NEST_Y + 40.0,
        42.0,
    )
    .translate(x - NEST_X / 2.0 + 28.0, y, deck_top_z() + NEST_Z + 21.0);
    let right_rail = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_right_lead_in_rail"),
        18.0,
        NEST_Y + 40.0,
        42.0,
    )
    .translate(x + NEST_X / 2.0 - 28.0, y, deck_top_z() + NEST_Z + 21.0);
    let custody_stop = centered_cube(
        format!("closed_handoff_{side}_nest_{index}_custody_positive_stop"),
        NEST_X - 72.0,
        16.0,
        58.0,
    )
    .translate(x, y - NEST_Y / 2.0 + 18.0, deck_top_z() + NEST_Z + 29.0);

    base - cassette_pocket + tote_envelope_gauge + left_rail + right_rail + custody_stop
}

fn datum_pin_arrays() -> Part {
    let mut pins = Part::empty("closed_handoff_datum_pin_arrays");
    for side in 0..2 {
        let y_center = if side == 0 {
            OPERATOR_SIDE_Y
        } else {
            ROBOT_SIDE_Y
        };
        let side_name = if side == 0 { "operator" } else { "robot" };
        for nest in 0..NESTS_PER_SIDE {
            let x_center = centered_index(nest, NESTS_PER_SIDE, NEST_PITCH_X);
            for row in 0..DATUM_PIN_ROWS {
                for col in 0..DATUM_PIN_COLS {
                    let x = x_center + centered_index(col, DATUM_PIN_COLS, DATUM_ARRAY_PITCH_X);
                    let y = y_center + centered_index(row, DATUM_PIN_ROWS, DATUM_ARRAY_PITCH_Y);
                    let pin = centered_cylinder(
                        format!("closed_handoff_{side_name}_nest_{nest}_datum_pin_{row}_{col}"),
                        DATUM_PIN_D / 2.0,
                        DATUM_PIN_Z,
                        32,
                    )
                    .translate(x, y, deck_top_z() + NEST_Z + DATUM_PIN_Z / 2.0);
                    let witness_ring = centered_cylinder(
                        format!(
                            "closed_handoff_{side_name}_nest_{nest}_datum_witness_ring_{row}_{col}"
                        ),
                        DATUM_PIN_D,
                        3.0,
                        32,
                    )
                    .translate(x, y, deck_top_z() + NEST_Z + 1.5);
                    pins = pins + pin + witness_ring;
                }
            }
        }
    }
    pins
}

fn latch_witness_slots() -> Part {
    let bar = centered_cube("closed_handoff_latch_witness_slot_bar", 980.0, 74.0, 30.0).translate(
        0.0,
        0.0,
        deck_top_z() + AIRLOCK_WALL_Z + 30.0,
    );
    let mut slots = Part::empty("closed_handoff_latch_witness_slot_cuts");
    let mut flags = Part::empty("closed_handoff_latch_witness_flags");
    for i in 0..LATCH_SLOT_COUNT {
        let x = centered_index(i, LATCH_SLOT_COUNT, 84.0);
        slots = slots
            + centered_cube(
                format!("closed_handoff_latch_witness_slot_{i}"),
                LATCH_SLOT_X,
                LATCH_SLOT_Y,
                LATCH_SLOT_Z,
            )
            .translate(x, 0.0, deck_top_z() + AIRLOCK_WALL_Z + 34.0);
        flags = flags
            + centered_cube(
                format!("closed_handoff_latch_witness_red_green_flag_land_{i}"),
                28.0,
                10.0,
                10.0,
            )
            .translate(x, -47.0, deck_top_z() + AIRLOCK_WALL_Z + 48.0);
    }
    bar - slots + flags
}

fn force_tilt_logger_pockets() -> Part {
    let mut pockets = Part::empty("closed_handoff_force_tilt_logger_pockets");
    for i in 0..LOGGER_POCKET_COUNT {
        let side_y = if i < LOGGER_POCKET_COUNT / 2 {
            OPERATOR_SIDE_Y - 200.0
        } else {
            ROBOT_SIDE_Y + 200.0
        };
        let local_i = i % (LOGGER_POCKET_COUNT / 2);
        let x = centered_index(local_i, LOGGER_POCKET_COUNT / 2, 210.0);
        let pocket = centered_cube(
            format!("closed_handoff_force_tilt_logger_pocket_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_Z,
        )
        .translate(x, side_y, deck_top_z() + LOGGER_POCKET_Z / 2.0);
        let recess = centered_cube(
            format!("closed_handoff_force_tilt_logger_recess_{i}"),
            LOGGER_POCKET_X - 22.0,
            LOGGER_POCKET_Y - 18.0,
            LOGGER_POCKET_Z + 4.0,
        )
        .translate(x, side_y, deck_top_z() + LOGGER_POCKET_Z / 2.0 + 9.0);
        let cable_slot = centered_cube(
            format!("closed_handoff_logger_cable_exit_{i}"),
            18.0,
            LOGGER_POCKET_Y + 42.0,
            12.0,
        )
        .translate(
            x + LOGGER_POCKET_X / 2.0 - 10.0,
            side_y,
            deck_top_z() + 19.0,
        );
        pockets = pockets + (pocket - recess - cable_slot);
    }

    pockets + tilt_challenge_ramps()
}

fn tilt_challenge_ramps() -> Part {
    let mut ramps = Part::empty("closed_handoff_tilt_challenge_ramps");
    for i in 0..TILT_RAMP_COUNT {
        let x = centered_index(i, TILT_RAMP_COUNT, 92.0);
        let height = 6.0 + i as f64 * 3.0;
        ramps = ramps
            + centered_cube(
                format!("closed_handoff_tilt_step_ramp_{i}"),
                70.0,
                92.0,
                height,
            )
            .translate(x, 0.0, deck_top_z() + height / 2.0);
    }
    ramps
}

fn barcode_rfid_scan_lands() -> Part {
    let mut lands = Part::empty("closed_handoff_barcode_rfid_scan_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 7, 7, 116.0);
        let y = if i < 7 { -446.0 } else { 446.0 };
        lands = lands
            + barcode_land(
                format!("closed_handoff_barcode_scan_land_{i}"),
                x,
                y,
                88.0,
                32.0,
            );
    }
    for i in 0..RFID_LANDS {
        let x = if i < 4 { -690.0 } else { 690.0 };
        let y = centered_index(i % 4, 4, 88.0);
        lands = lands
            + centered_cube(
                format!("closed_handoff_rfid_coil_scan_land_{i}"),
                70.0,
                52.0,
                6.0,
            )
            .translate(x, y, deck_top_z() + 3.0);
    }
    for i in 0..CUSTODY_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_handoff_custody_card_clip_land_{i}"),
                96.0,
                38.0,
                8.0,
            )
            .translate(
                centered_index(i, CUSTODY_CARD_LANDS, 122.0),
                118.0,
                deck_top_z() + 4.0,
            );
    }
    lands
}

fn barcode_land(name: String, x: f64, y: f64, sx: f64, sy: f64) -> Part {
    let plaque =
        centered_cube(format!("{name}_plaque"), sx, sy, 5.0).translate(x, y, deck_top_z() + 2.5);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..5 {
        bars =
            bars + centered_cube(format!("{name}_bar_{i}"), 4.0 + i as f64, sy - 8.0, 4.0)
                .translate(x - 25.0 + i as f64 * 12.0, y, deck_top_z() + 7.0);
    }
    plaque + bars
}

fn pass_fail_hold_lanes() -> Part {
    let mut lanes = Part::empty("closed_handoff_pass_fail_hold_lanes");
    for lane in 0..STATUS_LANES {
        let x = 486.0 + centered_index(lane, STATUS_LANES, 174.0);
        let lane_name = match lane {
            0 => "pass",
            1 => "fail",
            _ => "hold",
        };
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = -166.0 + slot as f64 * 96.0;
            let tray = centered_cube(
                format!("closed_handoff_{lane_name}_lane_slot_{slot}"),
                STATUS_SLOT_X,
                STATUS_SLOT_Y,
                STATUS_SLOT_Z,
            )
            .translate(x, y, deck_top_z() + STATUS_SLOT_Z / 2.0);
            let recess = centered_cube(
                format!("closed_handoff_{lane_name}_lane_recess_{slot}"),
                STATUS_SLOT_X - 22.0,
                STATUS_SLOT_Y - 20.0,
                STATUS_SLOT_Z + 3.0,
            )
            .translate(x, y, deck_top_z() + STATUS_SLOT_Z / 2.0 + 5.0);
            lanes = lanes + (tray - recess);
        }
    }
    lanes
}

fn tamper_seal_pockets() -> Part {
    let block = centered_cube(
        "closed_handoff_tamper_seal_storage_block",
        520.0,
        180.0,
        30.0,
    )
    .translate(-520.0, 0.0, deck_top_z() + 15.0);
    let mut cuts = Part::empty("closed_handoff_tamper_seal_pocket_cuts");
    let mut label_tabs = Part::empty("closed_handoff_tamper_seal_number_tabs");
    for i in 0..TAMPER_POCKET_COUNT {
        let row = i / 6;
        let col = i % 6;
        let x = -520.0 + centered_index(col, 6, 72.0);
        let y = centered_index(row, 2, 72.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_handoff_tamper_seal_round_pocket_{i}"),
                SEAL_POCKET_D / 2.0,
                SEAL_POCKET_Z,
                32,
            )
            .translate(x, y, deck_top_z() + 24.0);
        label_tabs = label_tabs
            + centered_cube(
                format!("closed_handoff_tamper_seal_serial_tab_{i}"),
                38.0,
                10.0,
                6.0,
            )
            .translate(x, y + 33.0, deck_top_z() + 35.0);
    }
    block - cuts + label_tabs
}

fn collision_keepout_gauges() -> Part {
    let mut gauges = Part::empty("closed_handoff_collision_keepout_gauges");
    for i in 0..COLLISION_GAUGE_COUNT {
        let x = centered_index(i % 4, 4, 326.0);
        let y = if i < 4 { -64.0 } else { 64.0 };
        let post = centered_cube(
            format!("closed_handoff_collision_keepout_post_{i}"),
            22.0,
            22.0,
            KEEP_OUT_GAUGE_Z,
        )
        .translate(x, y, deck_top_z() + KEEP_OUT_GAUGE_Z / 2.0);
        let whisker = centered_cylinder(
            format!("closed_handoff_collision_sweep_whisker_{i}"),
            5.0,
            146.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, deck_top_z() + KEEP_OUT_GAUGE_Z - 8.0);
        gauges = gauges + post + whisker;
    }
    let gantry = centered_cube(
        "closed_handoff_overhead_gantry_clearance_gauge",
        940.0,
        20.0,
        24.0,
    )
    .translate(0.0, 0.0, deck_top_z() + GANTRY_CLEARANCE_Z);
    gauges + gantry
}

fn clean_used_segregation() -> Part {
    let clean_floor = centered_cube("closed_handoff_clean_side_floor_land", 640.0, 206.0, 5.0)
        .translate(-372.0, ROBOT_SIDE_Y + 172.0, deck_top_z() + 2.5);
    let used_floor = centered_cube("closed_handoff_used_side_floor_land", 640.0, 206.0, 5.0)
        .translate(-372.0, OPERATOR_SIDE_Y - 172.0, deck_top_z() + 2.5);
    let divider = centered_cube(
        "closed_handoff_clean_used_custody_divider",
        760.0,
        24.0,
        92.0,
    )
    .translate(-372.0, 0.0, deck_top_z() + 46.0);
    let return_chute = centered_cube("closed_handoff_used_return_chute", 170.0, 248.0, 66.0)
        .translate(-730.0, OPERATOR_SIDE_Y - 106.0, deck_top_z() + 33.0);
    let return_chute_cut = centered_cube(
        "closed_handoff_used_return_chute_clearance",
        126.0,
        200.0,
        70.0,
    )
    .translate(-730.0, OPERATOR_SIDE_Y - 106.0, deck_top_z() + 42.0);

    clean_floor + used_floor + divider + (return_chute - return_chute_cut)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_handoff_evidence_camera_bridge_left_post",
        36.0,
        42.0,
        CAMERA_BRIDGE_CLEAR_Z,
    )
    .translate(
        -CAMERA_BRIDGE_SPAN_X / 2.0,
        0.0,
        deck_top_z() + CAMERA_BRIDGE_CLEAR_Z / 2.0,
    );
    let right_post = centered_cube(
        "closed_handoff_evidence_camera_bridge_right_post",
        36.0,
        42.0,
        CAMERA_BRIDGE_CLEAR_Z,
    )
    .translate(
        CAMERA_BRIDGE_SPAN_X / 2.0,
        0.0,
        deck_top_z() + CAMERA_BRIDGE_CLEAR_Z / 2.0,
    );
    let crossbar = centered_cube(
        "closed_handoff_evidence_camera_bridge_crossbar",
        CAMERA_BRIDGE_SPAN_X + 96.0,
        42.0,
        34.0,
    )
    .translate(0.0, 0.0, deck_top_z() + CAMERA_BRIDGE_CLEAR_Z + 17.0);

    left_post + right_post + crossbar + camera_pods() + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("closed_handoff_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = centered_index(i, CAMERA_PODS, 235.0);
        let pod = centered_cube(format!("closed_handoff_camera_pod_{i}"), 68.0, 58.0, 36.0)
            .translate(x, -35.0, deck_top_z() + CAMERA_BRIDGE_CLEAR_Z + 50.0);
        let lens = centered_cylinder(
            format!("closed_handoff_camera_lens_gauge_{i}"),
            14.0,
            18.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -72.0, deck_top_z() + CAMERA_BRIDGE_CLEAR_Z + 50.0);
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty("closed_handoff_evidence_light_bars");
    for i in 0..LIGHT_BARS {
        let x = centered_index(i, LIGHT_BARS, 275.0);
        bars = bars
            + centered_cube(
                format!("closed_handoff_evidence_light_bar_{i}"),
                170.0,
                16.0,
                18.0,
            )
            .translate(x, 35.0, deck_top_z() + CAMERA_BRIDGE_CLEAR_Z + 41.0);
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let robot = keepout_box(
        "closed_handoff_robot_approach_keepout",
        1030.0,
        ROBOT_APPROACH_KEEP_OUT_Y,
        118.0,
        0.0,
        DECK_Y / 2.0 - ROBOT_APPROACH_KEEP_OUT_Y / 2.0 - 34.0,
    );
    let service = keepout_box(
        "closed_handoff_service_door_swing_keepout",
        960.0,
        SERVICE_DOOR_KEEP_OUT_Y,
        94.0,
        0.0,
        -DECK_Y / 2.0 + SERVICE_DOOR_KEEP_OUT_Y / 2.0 + 34.0,
    );
    let left_cart = keepout_box(
        "closed_handoff_left_side_cart_keepout",
        SIDE_CART_KEEP_OUT_X,
        760.0,
        82.0,
        -DECK_X / 2.0 + SIDE_CART_KEEP_OUT_X / 2.0 + 26.0,
        0.0,
    );
    let right_cart = keepout_box(
        "closed_handoff_right_side_cart_keepout",
        SIDE_CART_KEEP_OUT_X,
        760.0,
        82.0,
        DECK_X / 2.0 - SIDE_CART_KEEP_OUT_X / 2.0 - 26.0,
        0.0,
    );
    let overhead = centered_cube(
        "closed_handoff_overhead_service_clearance_marker",
        880.0,
        24.0,
        24.0,
    )
    .translate(0.0, 0.0, deck_top_z() + OVERHEAD_SERVICE_Z);

    robot + service + left_cart + right_cart + overhead
}

fn keepout_box(name: &str, sx: f64, sy: f64, sz: f64, x: f64, y: f64) -> Part {
    let front = centered_cube(format!("{name}_front_edge"), sx, 12.0, sz).translate(
        x,
        y - sy / 2.0,
        deck_top_z() + sz / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear_edge"), sx, 12.0, sz).translate(
        x,
        y + sy / 2.0,
        deck_top_z() + sz / 2.0,
    );
    let left = centered_cube(format!("{name}_left_edge"), 12.0, sy, sz).translate(
        x - sx / 2.0,
        y,
        deck_top_z() + sz / 2.0,
    );
    let right = centered_cube(format!("{name}_right_edge"), 12.0, sy, sz).translate(
        x + sx / 2.0,
        y,
        deck_top_z() + sz / 2.0,
    );
    front + rear + left + right
}
