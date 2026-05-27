use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sample fraction archive module for tissue-chip media sampling.
//
// Intent:
// - Receive small, closed media fractions from tissue-chip sampling loops.
// - Register a chilled SBS-format 12x8 tube/plate nest for archived fractions.
// - Keep sterile sample handoff, flush/waste, cap/seal staging, barcode/lot/time
//   lands, chain-of-custody slots, leak containment, and robot service keepouts
//   physically visible in the CAD envelope.
//
// Research assumptions from the Exa pass:
// - Commercial refrigerated fraction collectors commonly hold samples at 4C and
//   support microliter-to-milliliter fractions.
// - SBS 12x8 racks with 96 coded cryovials are common for automated tracking;
//   permanent 1D/2D codes and rack-level identifiers support traceability.
// - Septum and non-septum collection modes both exist, so this concept includes
//   a closed septum/loop handoff and a cap/seal staging surface.
//
// This is mechanical packaging CAD for workflow planning. It is not a validated
// sterility, custody, refrigeration, leak, or analyzer compatibility claim.

const OUTPUTS: &[&str] = &[
    "output/closed_sample_fraction_archive_module_leak_tray_base.stl",
    "output/closed_sample_fraction_archive_module_fraction_tube_nest.stl",
    "output/closed_sample_fraction_archive_module_cold_block_interface.stl",
    "output/closed_sample_fraction_archive_module_sterile_sample_loop_handoff.stl",
    "output/closed_sample_fraction_archive_module_flush_waste_manifold.stl",
    "output/closed_sample_fraction_archive_module_cap_seal_staging.stl",
    "output/closed_sample_fraction_archive_module_barcode_lot_time_lands.stl",
    "output/closed_sample_fraction_archive_module_chain_of_custody_slots.stl",
    "output/closed_sample_fraction_archive_module_robot_service_keepouts.stl",
    "output/closed_sample_fraction_archive_module_assembly.stl",
];

const MODULE_X: f64 = 720.0;
const MODULE_Y: f64 = 500.0;
const BASE_Z: f64 = 24.0;
const TRAY_RIM_W: f64 = 16.0;
const TRAY_RIM_H: f64 = 34.0;
const LEAK_SUMP_DEPTH: f64 = 10.0;

const SBS_PLATE_X: f64 = 127.76;
const SBS_PLATE_Y: f64 = 85.48;
const SBS_COLS: usize = 12;
const SBS_ROWS: usize = 8;
const FRACTION_POSITIONS: usize = SBS_COLS * SBS_ROWS;
const SBS_PITCH: f64 = 9.0;
const SBS_A1_X_FROM_LEFT: f64 = 14.38;
const SBS_A1_Y_FROM_TOP: f64 = 11.24;

const TUBE_NEST_X: f64 = 198.0;
const TUBE_NEST_Y: f64 = 148.0;
const TUBE_NEST_Z: f64 = 34.0;
const TUBE_BORE_D: f64 = 7.2;
const TUBE_RETAINING_RIM_Z: f64 = 8.0;
const TUBE_KEY_SLOT_X: f64 = 1.6;
const TUBE_KEY_SLOT_Y: f64 = 4.0;

const COLD_BLOCK_X: f64 = 252.0;
const COLD_BLOCK_Y: f64 = 188.0;
const COLD_BLOCK_Z: f64 = 48.0;
const COLD_BLOCK_POCKET_X: f64 = TUBE_NEST_X + 16.0;
const COLD_BLOCK_POCKET_Y: f64 = TUBE_NEST_Y + 16.0;
const COLD_BLOCK_POCKET_DEPTH: f64 = 8.0;
const COLD_BLOCK_STANDOFF_Z: f64 = 6.0;
const COOLANT_BORE_D: f64 = 8.0;
const THERMISTOR_BORE_D: f64 = 3.6;

const SAMPLE_LANES: usize = 8;
const LOOP_HANDOFF_X: f64 = 430.0;
const LOOP_HANDOFF_Y: f64 = 104.0;
const LOOP_HANDOFF_Z: f64 = 64.0;
const LOOP_LANE_PITCH: f64 = 48.0;
const SAMPLE_LOOP_LAND_X: f64 = 34.0;
const SAMPLE_LOOP_LAND_Y: f64 = 42.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const SEPTUM_PORT_D: f64 = 3.2;

const FLUSH_WASTE_X: f64 = 440.0;
const FLUSH_WASTE_Y: f64 = 74.0;
const FLUSH_WASTE_Z: f64 = 50.0;
const FLUSH_BUS_D: f64 = 5.5;
const WASTE_BUS_D: f64 = 9.5;
const WASTE_DROP_D: f64 = 12.0;

const CAP_STAGE_X: f64 = 228.0;
const CAP_STAGE_Y: f64 = 150.0;
const CAP_STAGE_Z: f64 = 36.0;
const CAP_POSITIONS: usize = FRACTION_POSITIONS;
const CAP_WELL_D: f64 = 6.4;
const SEAL_MAT_POCKET_X: f64 = SBS_PLATE_X + 18.0;
const SEAL_MAT_POCKET_Y: f64 = 28.0;

const CUSTODY_X: f64 = 190.0;
const CUSTODY_Y: f64 = 154.0;
const CUSTODY_Z: f64 = 44.0;
const CUSTODY_COLS: usize = 6;
const CUSTODY_ROWS: usize = 4;
const CUSTODY_SLOTS: usize = CUSTODY_COLS * CUSTODY_ROWS;

const BARCODE_LANDS: usize = 8;
const LOT_TIME_LANDS: usize = 6;

const ROBOT_KEEPOUTS: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 280.0;
const SIDE_SERVICE_CLEARANCE: f64 = 160.0;
const NEEDLE_APPROACH_CLEARANCE_Z: f64 = 220.0;
const CAPPER_CLEARANCE_Z: f64 = 180.0;

const COLD_BLOCK_X_POS: f64 = -196.0;
const COLD_BLOCK_Y_POS: f64 = 52.0;
const LOOP_HANDOFF_X_POS: f64 = 164.0;
const LOOP_HANDOFF_Y_POS: f64 = 112.0;
const FLUSH_WASTE_X_POS: f64 = 162.0;
const FLUSH_WASTE_Y_POS: f64 = -12.0;
const CAP_STAGE_X_POS: f64 = -196.0;
const CAP_STAGE_Y_POS: f64 = -146.0;
const CUSTODY_X_POS: f64 = 166.0;
const CUSTODY_Y_POS: f64 = -148.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = leak_tray_base();
    export(&base, OUTPUTS[0]);

    let tube_nest = fraction_tube_nest();
    export(&tube_nest, OUTPUTS[1]);

    let cold_block = cold_block_interface();
    export(&cold_block, OUTPUTS[2]);

    let loop_handoff = sterile_sample_loop_handoff();
    export(&loop_handoff, OUTPUTS[3]);

    let flush_waste = flush_waste_manifold();
    export(&flush_waste, OUTPUTS[4]);

    let cap_stage = cap_seal_staging();
    export(&cap_stage, OUTPUTS[5]);

    let labels = barcode_lot_time_lands();
    export(&labels, OUTPUTS[6]);

    let custody = chain_of_custody_slots();
    export(&custody, OUTPUTS[7]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[8]);

    let assembly =
        base + cold_block.translate(
            COLD_BLOCK_X_POS,
            COLD_BLOCK_Y_POS,
            BASE_Z + COLD_BLOCK_Z / 2.0,
        ) + tube_nest.translate(
            COLD_BLOCK_X_POS,
            COLD_BLOCK_Y_POS,
            BASE_Z + COLD_BLOCK_Z + COLD_BLOCK_STANDOFF_Z + TUBE_NEST_Z / 2.0,
        ) + loop_handoff.translate(
            LOOP_HANDOFF_X_POS,
            LOOP_HANDOFF_Y_POS,
            BASE_Z + LOOP_HANDOFF_Z / 2.0,
        ) + flush_waste.translate(
            FLUSH_WASTE_X_POS,
            FLUSH_WASTE_Y_POS,
            BASE_Z + FLUSH_WASTE_Z / 2.0,
        ) + cap_stage.translate(CAP_STAGE_X_POS, CAP_STAGE_Y_POS, BASE_Z + CAP_STAGE_Z / 2.0)
            + labels.translate(0.0, 0.0, BASE_Z + 3.0)
            + custody.translate(CUSTODY_X_POS, CUSTODY_Y_POS, BASE_Z + CUSTODY_Z / 2.0)
            + keepouts;

    export(&assembly, OUTPUTS[9]);

    println!(
        "Closed sample fraction archive module: {:.0}mm x {:.0}mm deck, {} SBS fraction positions, {} sample-loop lanes, {} cap/seal staging wells, {} custody slots, and {} robot/service keepout cages.",
        MODULE_X,
        MODULE_Y,
        FRACTION_POSITIONS,
        SAMPLE_LANES,
        CAP_POSITIONS,
        CUSTODY_SLOTS,
        ROBOT_KEEPOUTS
    );
    println!(
        "Cold archive: {:.0}mm x {:.0}mm x {:.0}mm cold block under {:.0}mm x {:.0}mm tube nest, {:.1}mm pitch, {:.1}mm tube bores, coolant bores {:.1}mm, thermistor bore {:.1}mm.",
        COLD_BLOCK_X,
        COLD_BLOCK_Y,
        COLD_BLOCK_Z,
        TUBE_NEST_X,
        TUBE_NEST_Y,
        SBS_PITCH,
        TUBE_BORE_D,
        COOLANT_BORE_D,
        THERMISTOR_BORE_D
    );
    println!(
        "Traceability and service: {} barcode lands, {} lot/time lands, {:.0}mm front pull clearance, {:.0}mm side service clearance, {:.0}mm needle Z approach clearance.",
        BARCODE_LANDS,
        LOT_TIME_LANDS,
        FRONT_SERVICE_CLEARANCE,
        SIDE_SERVICE_CLEARANCE,
        NEEDLE_APPROACH_CLEARANCE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn leak_tray_base() -> Part {
    let pan = centered_cube(
        "closed_fraction_archive_leak_tray_pan",
        MODULE_X,
        MODULE_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sump_cut = centered_cube(
        "closed_fraction_archive_leak_tray_recessed_sump",
        MODULE_X - 96.0,
        MODULE_Y - 92.0,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z - LEAK_SUMP_DEPTH / 2.0);

    let drain = centered_cylinder(
        "closed_fraction_archive_leak_tray_front_drain_port",
        7.0 / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MODULE_X / 2.0 - 64.0, -MODULE_Y / 2.0 + 16.0, BASE_Z - 8.0);

    pan - sump_cut - drain
        + tray_rim()
        + tray_flow_ribs()
        + module_locator_bosses()
        + cold_block_slide_rails()
        + loop_handoff_rails()
        + custody_and_cap_rails()
}

fn tray_rim() -> Part {
    let left = centered_cube(
        "closed_fraction_archive_tray_left_raised_lip",
        TRAY_RIM_W,
        MODULE_Y,
        TRAY_RIM_H,
    )
    .translate(
        -(MODULE_X / 2.0 - TRAY_RIM_W / 2.0),
        0.0,
        BASE_Z + TRAY_RIM_H / 2.0,
    );
    let right = centered_cube(
        "closed_fraction_archive_tray_right_raised_lip",
        TRAY_RIM_W,
        MODULE_Y,
        TRAY_RIM_H,
    )
    .translate(
        MODULE_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        BASE_Z + TRAY_RIM_H / 2.0,
    );
    let rear = centered_cube(
        "closed_fraction_archive_tray_rear_raised_lip",
        MODULE_X,
        TRAY_RIM_W,
        TRAY_RIM_H,
    )
    .translate(
        0.0,
        MODULE_Y / 2.0 - TRAY_RIM_W / 2.0,
        BASE_Z + TRAY_RIM_H / 2.0,
    );
    let front = centered_cube(
        "closed_fraction_archive_tray_front_low_service_lip",
        MODULE_X - 142.0,
        10.0,
        18.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 - 8.0), BASE_Z + 9.0);

    left + right + rear + front
}

fn tray_flow_ribs() -> Part {
    let mut ribs = Part::empty("closed_fraction_archive_leak_tray_flow_ribs");
    for (i, x) in [-252.0, -126.0, 0.0, 126.0, 252.0].into_iter().enumerate() {
        let rib = centered_cube(
            format!("closed_fraction_archive_sump_flow_rib_{i}"),
            8.0,
            MODULE_Y - 126.0,
            6.0,
        )
        .translate(x, 12.0, BASE_Z + 3.0);
        ribs = ribs + rib;
    }

    let drain_weir = centered_cube(
        "closed_fraction_archive_drain_weir_to_front_port",
        130.0,
        8.0,
        8.0,
    )
    .translate(MODULE_X / 2.0 - 128.0, -MODULE_Y / 2.0 + 48.0, BASE_Z + 4.0);

    ribs + drain_weir
}

fn module_locator_bosses() -> Part {
    let mut bosses = Part::empty("closed_fraction_archive_module_locator_bosses");
    for (i, (x, y)) in locator_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_fraction_archive_locator_boss_{i}"),
            10.0,
            8.0,
            28,
        )
        .translate(x, y, BASE_Z + 4.0);
        let pin_clearance = centered_cylinder(
            format!("closed_fraction_archive_locator_pin_clearance_{i}"),
            3.2 / 2.0,
            10.0,
            18,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - pin_clearance);
    }
    bosses
}

fn cold_block_slide_rails() -> Part {
    let front = centered_cube(
        "closed_fraction_archive_cold_block_front_slide_rail",
        COLD_BLOCK_X + 34.0,
        10.0,
        14.0,
    )
    .translate(
        COLD_BLOCK_X_POS,
        COLD_BLOCK_Y_POS - COLD_BLOCK_Y / 2.0 - 12.0,
        BASE_Z + 7.0,
    );
    let rear = centered_cube(
        "closed_fraction_archive_cold_block_rear_slide_rail",
        COLD_BLOCK_X + 34.0,
        10.0,
        14.0,
    )
    .translate(
        COLD_BLOCK_X_POS,
        COLD_BLOCK_Y_POS + COLD_BLOCK_Y / 2.0 + 12.0,
        BASE_Z + 7.0,
    );
    let keyed_stop = centered_cube(
        "closed_fraction_archive_cold_block_rear_keyed_stop",
        COLD_BLOCK_X + 10.0,
        12.0,
        22.0,
    )
    .translate(
        COLD_BLOCK_X_POS,
        COLD_BLOCK_Y_POS + COLD_BLOCK_Y / 2.0 + 28.0,
        BASE_Z + 11.0,
    );

    front + rear + keyed_stop
}

fn loop_handoff_rails() -> Part {
    let front = centered_cube(
        "closed_fraction_archive_loop_handoff_front_mount_rail",
        LOOP_HANDOFF_X + 40.0,
        12.0,
        14.0,
    )
    .translate(
        LOOP_HANDOFF_X_POS,
        LOOP_HANDOFF_Y_POS - LOOP_HANDOFF_Y / 2.0 - 12.0,
        BASE_Z + 7.0,
    );
    let rear = centered_cube(
        "closed_fraction_archive_loop_handoff_rear_mount_rail",
        LOOP_HANDOFF_X + 40.0,
        12.0,
        14.0,
    )
    .translate(
        LOOP_HANDOFF_X_POS,
        LOOP_HANDOFF_Y_POS + LOOP_HANDOFF_Y / 2.0 + 12.0,
        BASE_Z + 7.0,
    );
    front + rear
}

fn custody_and_cap_rails() -> Part {
    let cap_front = centered_cube(
        "closed_fraction_archive_cap_stage_front_key_rail",
        CAP_STAGE_X + 24.0,
        10.0,
        12.0,
    )
    .translate(
        CAP_STAGE_X_POS,
        CAP_STAGE_Y_POS - CAP_STAGE_Y / 2.0 - 10.0,
        BASE_Z + 6.0,
    );
    let cap_rear = centered_cube(
        "closed_fraction_archive_cap_stage_rear_key_rail",
        CAP_STAGE_X + 24.0,
        10.0,
        12.0,
    )
    .translate(
        CAP_STAGE_X_POS,
        CAP_STAGE_Y_POS + CAP_STAGE_Y / 2.0 + 10.0,
        BASE_Z + 6.0,
    );
    let custody_left = centered_cube(
        "closed_fraction_archive_custody_slot_left_key_rail",
        10.0,
        CUSTODY_Y + 26.0,
        12.0,
    )
    .translate(
        CUSTODY_X_POS - CUSTODY_X / 2.0 - 10.0,
        CUSTODY_Y_POS,
        BASE_Z + 6.0,
    );
    let custody_right = centered_cube(
        "closed_fraction_archive_custody_slot_right_key_rail",
        10.0,
        CUSTODY_Y + 26.0,
        12.0,
    )
    .translate(
        CUSTODY_X_POS + CUSTODY_X / 2.0 + 10.0,
        CUSTODY_Y_POS,
        BASE_Z + 6.0,
    );

    cap_front + cap_rear + custody_left + custody_right
}

fn fraction_tube_nest() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_fraction_tube_nest_body",
        TUBE_NEST_X,
        TUBE_NEST_Y,
        TUBE_NEST_Z,
    );

    let plate_registration = centered_cube(
        "closed_fraction_archive_sbs_rack_registration_pocket",
        SBS_PLATE_X + 2.0,
        SBS_PLATE_Y + 2.0,
        TUBE_RETAINING_RIM_Z + 1.0,
    )
    .translate(0.0, 0.0, TUBE_NEST_Z / 2.0 - TUBE_RETAINING_RIM_Z / 2.0);

    let mut cuts = Part::empty("closed_fraction_archive_fraction_tube_nest_cuts");
    let mut overlays = Part::empty("closed_fraction_archive_fraction_tube_nest_overlays");
    for row in 0..SBS_ROWS {
        for col in 0..SBS_COLS {
            let index = row * SBS_COLS + col;
            let x = fraction_x(col);
            let y = fraction_y(row);

            let bore = centered_cylinder(
                format!("closed_fraction_archive_fraction_tube_bore_r{row}_c{col}"),
                TUBE_BORE_D / 2.0,
                TUBE_NEST_Z + 2.0,
                24,
            )
            .translate(x, y, 0.0);
            let key = centered_cube(
                format!("closed_fraction_archive_fraction_tube_keyway_r{row}_c{col}"),
                TUBE_KEY_SLOT_X,
                TUBE_KEY_SLOT_Y,
                TUBE_NEST_Z + 2.0,
            )
            .translate(x, y + TUBE_BORE_D / 2.0, 0.0);
            cuts = cuts + bore + key;

            let rim = centered_cylinder(
                format!("closed_fraction_archive_fraction_tube_top_retain_ring_{index}"),
                TUBE_BORE_D / 2.0 + 0.9,
                2.2,
                24,
            )
            .translate(x, y, TUBE_NEST_Z / 2.0 + 1.1);
            let rim_opening = centered_cylinder(
                format!("closed_fraction_archive_fraction_tube_top_opening_{index}"),
                TUBE_BORE_D / 2.0 + 0.15,
                2.6,
                24,
            )
            .translate(x, y, TUBE_NEST_Z / 2.0 + 1.1);
            overlays = overlays + (rim - rim_opening);
        }
    }

    let a1_marker = centered_cube(
        "closed_fraction_archive_fraction_tube_nest_a1_chamfer_proxy",
        22.0,
        22.0,
        TUBE_NEST_Z + 4.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-(TUBE_NEST_X / 2.0 - 10.0), TUBE_NEST_Y / 2.0 - 10.0, 0.0);

    body - plate_registration - cuts - a1_marker
        + overlays
        + tube_nest_side_barcode_lands()
        + tube_nest_row_column_ticks()
        + tube_nest_handle_ears()
}

fn tube_nest_side_barcode_lands() -> Part {
    let front = centered_cube(
        "closed_fraction_archive_tube_nest_front_1d_barcode_land",
        118.0,
        8.0,
        5.0,
    )
    .translate(0.0, -(TUBE_NEST_Y / 2.0 - 5.0), TUBE_NEST_Z / 2.0 + 2.5);
    let right = centered_cube(
        "closed_fraction_archive_tube_nest_right_2d_rack_code_land",
        8.0,
        80.0,
        5.0,
    )
    .translate(TUBE_NEST_X / 2.0 - 5.0, 0.0, TUBE_NEST_Z / 2.0 + 2.5);
    let lot = centered_cube(
        "closed_fraction_archive_tube_nest_lot_release_land",
        56.0,
        18.0,
        4.0,
    )
    .translate(
        -(TUBE_NEST_X / 2.0 - 38.0),
        -(TUBE_NEST_Y / 2.0 - 18.0),
        TUBE_NEST_Z / 2.0 + 2.0,
    );

    front + right + lot
}

fn tube_nest_row_column_ticks() -> Part {
    let mut ticks = Part::empty("closed_fraction_archive_tube_nest_row_column_ticks");

    for col in 0..SBS_COLS {
        let tick = centered_cube(
            format!("closed_fraction_archive_tube_nest_column_tick_{col}"),
            2.0,
            7.0,
            4.0,
        )
        .translate(
            fraction_x(col),
            TUBE_NEST_Y / 2.0 - 18.0,
            TUBE_NEST_Z / 2.0 + 2.0,
        );
        ticks = ticks + tick;
    }

    for row in 0..SBS_ROWS {
        let tick = centered_cube(
            format!("closed_fraction_archive_tube_nest_row_tick_{row}"),
            7.0,
            2.0,
            4.0,
        )
        .translate(
            -(TUBE_NEST_X / 2.0 - 18.0),
            fraction_y(row),
            TUBE_NEST_Z / 2.0 + 2.0,
        );
        ticks = ticks + tick;
    }

    ticks
}

fn tube_nest_handle_ears() -> Part {
    let left = centered_cube(
        "closed_fraction_archive_tube_nest_left_robot_handle_ear",
        18.0,
        74.0,
        22.0,
    )
    .translate(-(TUBE_NEST_X / 2.0 + 9.0), 0.0, 0.0);
    let right = centered_cube(
        "closed_fraction_archive_tube_nest_right_robot_handle_ear",
        18.0,
        74.0,
        22.0,
    )
    .translate(TUBE_NEST_X / 2.0 + 9.0, 0.0, 0.0);
    let left_grip = centered_cube(
        "closed_fraction_archive_tube_nest_left_grip_relief",
        8.0,
        52.0,
        8.0,
    )
    .translate(-(TUBE_NEST_X / 2.0 + 9.0), 0.0, 0.0);
    let right_grip = centered_cube(
        "closed_fraction_archive_tube_nest_right_grip_relief",
        8.0,
        52.0,
        8.0,
    )
    .translate(TUBE_NEST_X / 2.0 + 9.0, 0.0, 0.0);

    (left - left_grip) + (right - right_grip)
}

fn cold_block_interface() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_cold_block_interface_body",
        COLD_BLOCK_X,
        COLD_BLOCK_Y,
        COLD_BLOCK_Z,
    );

    let nest_pocket = centered_cube(
        "closed_fraction_archive_cold_block_tube_nest_pocket",
        COLD_BLOCK_POCKET_X,
        COLD_BLOCK_POCKET_Y,
        COLD_BLOCK_POCKET_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, COLD_BLOCK_Z / 2.0 - COLD_BLOCK_POCKET_DEPTH / 2.0);

    let coolant_in = centered_cylinder(
        "closed_fraction_archive_cold_block_coolant_in_bore",
        COOLANT_BORE_D / 2.0,
        COLD_BLOCK_X + 8.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -COLD_BLOCK_Y / 2.0 + 30.0, -8.0);
    let coolant_out = centered_cylinder(
        "closed_fraction_archive_cold_block_coolant_out_bore",
        COOLANT_BORE_D / 2.0,
        COLD_BLOCK_X + 8.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, COLD_BLOCK_Y / 2.0 - 30.0, -8.0);
    let cross_channel = centered_cylinder(
        "closed_fraction_archive_cold_block_coolant_cross_channel",
        COOLANT_BORE_D / 2.0,
        COLD_BLOCK_Y - 60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, -8.0);

    let thermistor = centered_cylinder(
        "closed_fraction_archive_cold_block_thermistor_probe_bore",
        THERMISTOR_BORE_D / 2.0,
        46.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(COLD_BLOCK_Y / 2.0 - 22.0), COLD_BLOCK_Z / 2.0 - 17.0);

    let tec_left = centered_cube(
        "closed_fraction_archive_cold_block_left_peltier_recess",
        46.0,
        46.0,
        3.0,
    )
    .translate(-34.0, 0.0, -(COLD_BLOCK_Z / 2.0 - 1.0));
    let tec_right = centered_cube(
        "closed_fraction_archive_cold_block_right_peltier_recess",
        46.0,
        46.0,
        3.0,
    )
    .translate(34.0, 0.0, -(COLD_BLOCK_Z / 2.0 - 1.0));

    let condensate_drain = centered_cylinder(
        "closed_fraction_archive_cold_block_condensate_drain_bore",
        4.0 / 2.0,
        COLD_BLOCK_Y + 8.0,
        18,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(COLD_BLOCK_X / 2.0 - 28.0, 0.0, COLD_BLOCK_Z / 2.0 - 10.0);

    body - nest_pocket
        - coolant_in
        - coolant_out
        - cross_channel
        - thermistor
        - tec_left
        - tec_right
        - condensate_drain
        - cold_block_mount_holes()
        + cold_block_insulation_lip()
        + cold_block_hose_barb_bosses()
        + cold_block_latch_bosses()
}

fn cold_block_mount_holes() -> Part {
    let mut holes = Part::empty("closed_fraction_archive_cold_block_mount_holes");
    for (i, (x, y)) in [
        (-(COLD_BLOCK_X / 2.0 - 22.0), -(COLD_BLOCK_Y / 2.0 - 18.0)),
        (COLD_BLOCK_X / 2.0 - 22.0, -(COLD_BLOCK_Y / 2.0 - 18.0)),
        (-(COLD_BLOCK_X / 2.0 - 22.0), COLD_BLOCK_Y / 2.0 - 18.0),
        (COLD_BLOCK_X / 2.0 - 22.0, COLD_BLOCK_Y / 2.0 - 18.0),
    ]
    .into_iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_fraction_archive_cold_block_m4_mount_{i}"),
            4.4 / 2.0,
            COLD_BLOCK_Z + 2.0,
            22,
        )
        .translate(x, y, 0.0);
        holes = holes + hole;
    }
    holes
}

fn cold_block_insulation_lip() -> Part {
    let front = centered_cube(
        "closed_fraction_archive_cold_block_front_insulation_lip",
        COLD_BLOCK_POCKET_X + 20.0,
        8.0,
        13.0,
    )
    .translate(
        0.0,
        -(COLD_BLOCK_POCKET_Y / 2.0 + 8.0),
        COLD_BLOCK_Z / 2.0 + 6.5,
    );
    let rear = centered_cube(
        "closed_fraction_archive_cold_block_rear_insulation_lip",
        COLD_BLOCK_POCKET_X + 20.0,
        8.0,
        13.0,
    )
    .translate(
        0.0,
        COLD_BLOCK_POCKET_Y / 2.0 + 8.0,
        COLD_BLOCK_Z / 2.0 + 6.5,
    );
    let left = centered_cube(
        "closed_fraction_archive_cold_block_left_insulation_lip",
        8.0,
        COLD_BLOCK_POCKET_Y + 18.0,
        13.0,
    )
    .translate(
        -(COLD_BLOCK_POCKET_X / 2.0 + 8.0),
        0.0,
        COLD_BLOCK_Z / 2.0 + 6.5,
    );
    let right = centered_cube(
        "closed_fraction_archive_cold_block_right_insulation_lip",
        8.0,
        COLD_BLOCK_POCKET_Y + 18.0,
        13.0,
    )
    .translate(
        COLD_BLOCK_POCKET_X / 2.0 + 8.0,
        0.0,
        COLD_BLOCK_Z / 2.0 + 6.5,
    );
    front + rear + left + right
}

fn cold_block_hose_barb_bosses() -> Part {
    let inlet = centered_cylinder(
        "closed_fraction_archive_cold_block_inlet_hose_barb_boss",
        9.5,
        20.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -(COLD_BLOCK_X / 2.0 + 10.0),
        -COLD_BLOCK_Y / 2.0 + 30.0,
        -8.0,
    );
    let outlet = centered_cylinder(
        "closed_fraction_archive_cold_block_outlet_hose_barb_boss",
        9.5,
        20.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(COLD_BLOCK_X / 2.0 + 10.0, COLD_BLOCK_Y / 2.0 - 30.0, -8.0);
    let inlet_bore = centered_cylinder(
        "closed_fraction_archive_cold_block_inlet_hose_bore",
        COOLANT_BORE_D / 2.0,
        24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -(COLD_BLOCK_X / 2.0 + 10.0),
        -COLD_BLOCK_Y / 2.0 + 30.0,
        -8.0,
    );
    let outlet_bore = centered_cylinder(
        "closed_fraction_archive_cold_block_outlet_hose_bore",
        COOLANT_BORE_D / 2.0,
        24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(COLD_BLOCK_X / 2.0 + 10.0, COLD_BLOCK_Y / 2.0 - 30.0, -8.0);

    (inlet - inlet_bore) + (outlet - outlet_bore)
}

fn cold_block_latch_bosses() -> Part {
    let mut bosses = Part::empty("closed_fraction_archive_cold_block_latch_bosses");
    for (i, x) in [-76.0, 76.0].into_iter().enumerate() {
        let latch = centered_cube(
            format!("closed_fraction_archive_cold_block_pull_latch_boss_{i}"),
            42.0,
            12.0,
            16.0,
        )
        .translate(x, -(COLD_BLOCK_Y / 2.0 + 6.0), COLD_BLOCK_Z / 2.0 - 4.0);
        let latch_slot = centered_cube(
            format!("closed_fraction_archive_cold_block_pull_latch_slot_{i}"),
            20.0,
            14.0,
            6.0,
        )
        .translate(x, -(COLD_BLOCK_Y / 2.0 + 6.0), COLD_BLOCK_Z / 2.0 - 4.0);
        bosses = bosses + (latch - latch_slot);
    }
    bosses
}

fn sterile_sample_loop_handoff() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_sterile_sample_loop_handoff_body",
        LOOP_HANDOFF_X,
        LOOP_HANDOFF_Y,
        LOOP_HANDOFF_Z,
    );

    let mut cuts = Part::empty("closed_fraction_archive_sterile_loop_handoff_cuts");
    let mut features = Part::empty("closed_fraction_archive_sterile_loop_handoff_features");
    for lane in 0..SAMPLE_LANES {
        let x = sample_lane_x(lane);
        let sample_bore = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_closed_sample_path_bore"),
            FLUID_BORE_D / 2.0,
            LOOP_HANDOFF_Y + 8.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, -8.0);
        let needle_drop = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_septum_needle_drop"),
            SEPTUM_PORT_D / 2.0,
            LOOP_HANDOFF_Z + 4.0,
            20,
        )
        .translate(x, -LOOP_HANDOFF_Y / 2.0 + 24.0, 0.0);
        let loop_land_recess = centered_cube(
            format!("closed_fraction_archive_lane_{lane}_sample_loop_land_recess"),
            SAMPLE_LOOP_LAND_X,
            SAMPLE_LOOP_LAND_Y,
            8.0,
        )
        .translate(x, 4.0, LOOP_HANDOFF_Z / 2.0 - 4.0);
        cuts = cuts + sample_bore + needle_drop + loop_land_recess;

        let septum_ring = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_septum_retainer_ring"),
            8.0,
            6.0,
            30,
        )
        .translate(x, -LOOP_HANDOFF_Y / 2.0 + 24.0, LOOP_HANDOFF_Z / 2.0 + 3.0);
        let septum_opening = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_septum_center_clearance"),
            SEPTUM_PORT_D / 2.0 + 0.4,
            7.0,
            20,
        )
        .translate(x, -LOOP_HANDOFF_Y / 2.0 + 24.0, LOOP_HANDOFF_Z / 2.0 + 3.0);
        let left_clip = centered_cube(
            format!("closed_fraction_archive_lane_{lane}_sample_loop_left_clip"),
            5.0,
            SAMPLE_LOOP_LAND_Y + 10.0,
            12.0,
        )
        .translate(
            x - SAMPLE_LOOP_LAND_X / 2.0 - 5.0,
            4.0,
            LOOP_HANDOFF_Z / 2.0 + 6.0,
        );
        let right_clip = centered_cube(
            format!("closed_fraction_archive_lane_{lane}_sample_loop_right_clip"),
            5.0,
            SAMPLE_LOOP_LAND_Y + 10.0,
            12.0,
        )
        .translate(
            x + SAMPLE_LOOP_LAND_X / 2.0 + 5.0,
            4.0,
            LOOP_HANDOFF_Z / 2.0 + 6.0,
        );
        let loop_proxy = centered_cube(
            format!("closed_fraction_archive_lane_{lane}_disposable_sample_loop_proxy"),
            SAMPLE_LOOP_LAND_X - 8.0,
            5.0,
            7.0,
        )
        .translate(x, LOOP_HANDOFF_Y / 2.0 - 25.0, LOOP_HANDOFF_Z / 2.0 + 3.5);
        let lane_label = centered_cube(
            format!("closed_fraction_archive_lane_{lane}_sample_loop_barcode_land"),
            34.0,
            6.0,
            4.0,
        )
        .translate(
            x,
            -(LOOP_HANDOFF_Y / 2.0 + 3.0),
            LOOP_HANDOFF_Z / 2.0 - 10.0,
        );

        features = features
            + (septum_ring - septum_opening)
            + left_clip
            + right_clip
            + loop_proxy
            + lane_label;
    }

    let sterile_bulkhead_gasket = centered_cube(
        "closed_fraction_archive_sample_loop_sterile_bulkhead_gasket_land",
        LOOP_HANDOFF_X - 52.0,
        10.0,
        8.0,
    )
    .translate(0.0, -LOOP_HANDOFF_Y / 2.0 - 2.0, 0.0);

    body - cuts - handoff_mount_holes() + features + sterile_bulkhead_gasket
}

fn handoff_mount_holes() -> Part {
    let mut holes = Part::empty("closed_fraction_archive_sterile_handoff_mount_holes");
    for (i, (x, y)) in [
        (
            -(LOOP_HANDOFF_X / 2.0 - 24.0),
            -(LOOP_HANDOFF_Y / 2.0 - 16.0),
        ),
        (LOOP_HANDOFF_X / 2.0 - 24.0, -(LOOP_HANDOFF_Y / 2.0 - 16.0)),
        (-(LOOP_HANDOFF_X / 2.0 - 24.0), LOOP_HANDOFF_Y / 2.0 - 16.0),
        (LOOP_HANDOFF_X / 2.0 - 24.0, LOOP_HANDOFF_Y / 2.0 - 16.0),
    ]
    .into_iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_fraction_archive_sterile_handoff_m4_mount_{i}"),
            4.4 / 2.0,
            LOOP_HANDOFF_Z + 2.0,
            20,
        )
        .translate(x, y, 0.0);
        holes = holes + hole;
    }
    holes
}

fn flush_waste_manifold() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_flush_waste_manifold_body",
        FLUSH_WASTE_X,
        FLUSH_WASTE_Y,
        FLUSH_WASTE_Z,
    );

    let mut cuts = Part::empty("closed_fraction_archive_flush_waste_manifold_cuts");
    let mut features = Part::empty("closed_fraction_archive_flush_waste_manifold_features");
    for lane in 0..SAMPLE_LANES {
        let x = sample_lane_x(lane);
        let flush_drop = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_flush_drop_bore"),
            FLUSH_BUS_D / 2.0,
            FLUSH_WASTE_Z + 2.0,
            20,
        )
        .translate(x, FLUSH_WASTE_Y / 2.0 - 20.0, 0.0);
        let waste_drop = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_waste_drop_bore"),
            WASTE_DROP_D / 2.0,
            FLUSH_WASTE_Z + 2.0,
            24,
        )
        .translate(x, -(FLUSH_WASTE_Y / 2.0 - 20.0), 0.0);
        cuts = cuts + flush_drop + waste_drop;

        let check_valve_boss = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_flush_check_valve_boss"),
            8.0,
            8.0,
            24,
        )
        .translate(x, FLUSH_WASTE_Y / 2.0 - 20.0, FLUSH_WASTE_Z / 2.0 + 4.0);
        let waste_cup_rim = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_waste_capture_cup_rim"),
            9.2,
            5.0,
            24,
        )
        .translate(x, -(FLUSH_WASTE_Y / 2.0 - 20.0), FLUSH_WASTE_Z / 2.0 + 2.5);
        let waste_cup_opening = centered_cylinder(
            format!("closed_fraction_archive_lane_{lane}_waste_capture_cup_opening"),
            WASTE_DROP_D / 2.0,
            6.0,
            24,
        )
        .translate(x, -(FLUSH_WASTE_Y / 2.0 - 20.0), FLUSH_WASTE_Z / 2.0 + 2.5);
        features = features + check_valve_boss + (waste_cup_rim - waste_cup_opening);
    }

    let flush_bus = centered_cylinder(
        "closed_fraction_archive_common_flush_bus_bore",
        FLUSH_BUS_D / 2.0,
        FLUSH_WASTE_X - 54.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, FLUSH_WASTE_Y / 2.0 - 20.0, 0.0);
    let waste_bus = centered_cylinder(
        "closed_fraction_archive_common_waste_bus_bore",
        WASTE_BUS_D / 2.0,
        FLUSH_WASTE_X - 54.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(FLUSH_WASTE_Y / 2.0 - 20.0), -2.0);
    let front_waste_spout = centered_cylinder(
        "closed_fraction_archive_front_waste_spout_bore",
        WASTE_BUS_D / 2.0,
        FLUSH_WASTE_Y + 22.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FLUSH_WASTE_X / 2.0 - 36.0, 0.0, -2.0);
    cuts = cuts + flush_bus + waste_bus + front_waste_spout + flush_waste_mount_holes();

    let flush_inlet_label = centered_cube(
        "closed_fraction_archive_flush_reagent_inlet_label_land",
        94.0,
        8.0,
        4.0,
    )
    .translate(
        -(FLUSH_WASTE_X / 2.0 - 64.0),
        FLUSH_WASTE_Y / 2.0 + 3.0,
        FLUSH_WASTE_Z / 2.0 - 8.0,
    );
    let waste_label = centered_cube(
        "closed_fraction_archive_waste_path_label_land",
        104.0,
        8.0,
        4.0,
    )
    .translate(
        FLUSH_WASTE_X / 2.0 - 68.0,
        -(FLUSH_WASTE_Y / 2.0 + 3.0),
        FLUSH_WASTE_Z / 2.0 - 8.0,
    );

    body - cuts + features + flush_inlet_label + waste_label
}

fn flush_waste_mount_holes() -> Part {
    let mut holes = Part::empty("closed_fraction_archive_flush_waste_mount_holes");
    for (i, x) in [
        -(FLUSH_WASTE_X / 2.0 - 24.0),
        FLUSH_WASTE_X / 2.0 - 24.0,
        -72.0,
        72.0,
    ]
    .into_iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_fraction_archive_flush_waste_m4_mount_{i}"),
            4.4 / 2.0,
            FLUSH_WASTE_Z + 2.0,
            20,
        )
        .translate(x, 0.0, 0.0);
        holes = holes + hole;
    }
    holes
}

fn cap_seal_staging() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_cap_seal_staging_body",
        CAP_STAGE_X,
        CAP_STAGE_Y,
        CAP_STAGE_Z,
    );

    let mut cuts = Part::empty("closed_fraction_archive_cap_seal_staging_cuts");
    let mut features = Part::empty("closed_fraction_archive_cap_seal_staging_features");
    for row in 0..SBS_ROWS {
        for col in 0..SBS_COLS {
            let index = row * SBS_COLS + col;
            let x = fraction_x(col);
            let y = fraction_y(row) + 16.0;
            let cap_well = centered_cylinder(
                format!("closed_fraction_archive_cap_seal_well_r{row}_c{col}"),
                CAP_WELL_D / 2.0,
                12.0,
                20,
            )
            .translate(x, y, CAP_STAGE_Z / 2.0 - 5.0);
            let cap_key = centered_cube(
                format!("closed_fraction_archive_cap_seal_key_slot_r{row}_c{col}"),
                1.4,
                3.0,
                12.0,
            )
            .translate(x, y + CAP_WELL_D / 2.0, CAP_STAGE_Z / 2.0 - 5.0);
            cuts = cuts + cap_well + cap_key;

            if index % 12 == 0 {
                let row_marker = centered_cube(
                    format!("closed_fraction_archive_cap_stage_row_marker_{row}"),
                    8.0,
                    2.0,
                    3.0,
                )
                .translate(-(CAP_STAGE_X / 2.0 - 22.0), y, CAP_STAGE_Z / 2.0 + 1.5);
                features = features + row_marker;
            }
        }
    }

    let seal_mat_pocket = centered_cube(
        "closed_fraction_archive_septum_seal_mat_pocket",
        SEAL_MAT_POCKET_X,
        SEAL_MAT_POCKET_Y,
        8.0,
    )
    .translate(0.0, -(CAP_STAGE_Y / 2.0 - 20.0), CAP_STAGE_Z / 2.0 - 4.0);
    let spent_cap_trough = centered_cube(
        "closed_fraction_archive_spent_cap_return_trough",
        CAP_STAGE_X - 42.0,
        12.0,
        12.0,
    )
    .translate(0.0, CAP_STAGE_Y / 2.0 - 12.0, CAP_STAGE_Z / 2.0 - 6.0);

    let seal_pressure_frame_front = centered_cube(
        "closed_fraction_archive_seal_pressure_frame_front",
        SEAL_MAT_POCKET_X + 18.0,
        6.0,
        12.0,
    )
    .translate(0.0, -(CAP_STAGE_Y / 2.0 - 38.0), CAP_STAGE_Z / 2.0 + 6.0);
    let seal_pressure_frame_rear = centered_cube(
        "closed_fraction_archive_seal_pressure_frame_rear",
        SEAL_MAT_POCKET_X + 18.0,
        6.0,
        12.0,
    )
    .translate(0.0, -(CAP_STAGE_Y / 2.0 - 4.0), CAP_STAGE_Z / 2.0 + 6.0);

    body - cuts - seal_mat_pocket - spent_cap_trough
        + features
        + seal_pressure_frame_front
        + seal_pressure_frame_rear
        + cap_stage_latches()
}

fn cap_stage_latches() -> Part {
    let left = centered_cube(
        "closed_fraction_archive_cap_stage_left_seal_cassette_latch",
        12.0,
        CAP_STAGE_Y - 34.0,
        16.0,
    )
    .translate(-(CAP_STAGE_X / 2.0 - 10.0), 0.0, CAP_STAGE_Z / 2.0 - 2.0);
    let right = centered_cube(
        "closed_fraction_archive_cap_stage_right_seal_cassette_latch",
        12.0,
        CAP_STAGE_Y - 34.0,
        16.0,
    )
    .translate(CAP_STAGE_X / 2.0 - 10.0, 0.0, CAP_STAGE_Z / 2.0 - 2.0);
    left + right
}

fn barcode_lot_time_lands() -> Part {
    let mut lands = Part::empty("closed_fraction_archive_barcode_lot_time_lands");
    for (i, (x, y, land_x, land_y)) in barcode_land_specs().into_iter().enumerate() {
        let land = centered_cube(
            format!("closed_fraction_archive_barcode_land_{i}"),
            land_x,
            land_y,
            5.0,
        )
        .translate(x, y, 0.0);
        lands = lands
            + land
            + fiducial_pair(
                format!("closed_fraction_archive_barcode_land_{i}"),
                x,
                y,
                land_x,
            );
    }

    for (i, (x, y, land_x)) in lot_time_land_specs().into_iter().enumerate() {
        let land = centered_cube(
            format!("closed_fraction_archive_lot_time_land_{i}"),
            land_x,
            16.0,
            4.0,
        )
        .translate(x, y, 0.5);
        lands = lands + land;
    }

    lands
}

fn fiducial_pair(name: String, x: f64, y: f64, span_x: f64) -> Part {
    let left = centered_cylinder(format!("{name}_left_scan_fiducial"), 2.0, 3.0, 18).translate(
        x - span_x / 2.0 + 8.0,
        y,
        2.5,
    );
    let right = centered_cylinder(format!("{name}_right_scan_fiducial"), 2.0, 3.0, 18).translate(
        x + span_x / 2.0 - 8.0,
        y,
        2.5,
    );
    left + right
}

fn chain_of_custody_slots() -> Part {
    let body = centered_cube(
        "closed_fraction_archive_chain_of_custody_slot_bank_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    let mut cuts = Part::empty("closed_fraction_archive_chain_of_custody_slot_bank_cuts");
    let mut features = Part::empty("closed_fraction_archive_chain_of_custody_slot_bank_features");
    for row in 0..CUSTODY_ROWS {
        for col in 0..CUSTODY_COLS {
            let index = row * CUSTODY_COLS + col;
            let x = custody_slot_x(col);
            let y = custody_slot_y(row);
            let slot = centered_cube(
                format!("closed_fraction_archive_chain_custody_slot_{index}"),
                18.0,
                4.2,
                CUSTODY_Z + 2.0,
            )
            .rotate(0.0, 0.0, 8.0)
            .translate(x, y, 0.0);
            let lead_in = centered_cube(
                format!("closed_fraction_archive_chain_custody_slot_lead_in_{index}"),
                20.0,
                7.5,
                8.0,
            )
            .rotate(0.0, 0.0, 8.0)
            .translate(x, y, CUSTODY_Z / 2.0 - 4.0);
            cuts = cuts + slot + lead_in;

            let status_land = centered_cube(
                format!("closed_fraction_archive_chain_custody_status_land_{index}"),
                16.0,
                6.0,
                3.0,
            )
            .translate(x, y + 10.0, CUSTODY_Z / 2.0 + 1.5);
            features = features + status_land;
        }
    }

    let tamper_seal_trench = centered_cube(
        "closed_fraction_archive_chain_custody_tamper_seal_trench",
        CUSTODY_X - 30.0,
        10.0,
        10.0,
    )
    .translate(0.0, -(CUSTODY_Y / 2.0 - 18.0), CUSTODY_Z / 2.0 - 5.0);

    let audit_token_wells = custody_audit_token_wells();

    body - cuts - tamper_seal_trench - audit_token_wells + features + custody_slot_side_lands()
}

fn custody_audit_token_wells() -> Part {
    let mut wells = Part::empty("closed_fraction_archive_chain_custody_audit_token_wells");
    for (i, x) in [-54.0, -18.0, 18.0, 54.0].into_iter().enumerate() {
        let well = centered_cylinder(
            format!("closed_fraction_archive_chain_custody_audit_token_well_{i}"),
            7.0,
            10.0,
            24,
        )
        .translate(x, CUSTODY_Y / 2.0 - 18.0, CUSTODY_Z / 2.0 - 5.0);
        wells = wells + well;
    }
    wells
}

fn custody_slot_side_lands() -> Part {
    let scan_land = centered_cube(
        "closed_fraction_archive_chain_custody_rack_scan_land",
        CUSTODY_X - 42.0,
        12.0,
        4.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 + 4.0, CUSTODY_Z / 2.0 - 6.0);
    let time_land = centered_cube(
        "closed_fraction_archive_chain_custody_time_release_land",
        82.0,
        12.0,
        4.0,
    )
    .translate(0.0, -(CUSTODY_Y / 2.0 + 4.0), CUSTODY_Z / 2.0 - 6.0);
    scan_land + time_land
}

fn robot_service_keepouts() -> Part {
    let needle_bottom =
        BASE_Z + COLD_BLOCK_Z + COLD_BLOCK_STANDOFF_Z + TUBE_NEST_Z + TUBE_RETAINING_RIM_Z;
    let needle = keepout_cage(
        "closed_fraction_archive_needle_probe_keepout",
        COLD_BLOCK_X_POS,
        COLD_BLOCK_Y_POS,
        COLD_BLOCK_X + 52.0,
        COLD_BLOCK_Y + 44.0,
        NEEDLE_APPROACH_CLEARANCE_Z,
        needle_bottom,
    );

    let capper = keepout_cage(
        "closed_fraction_archive_capper_decapper_keepout",
        CAP_STAGE_X_POS,
        CAP_STAGE_Y_POS,
        CAP_STAGE_X + 48.0,
        CAP_STAGE_Y + 42.0,
        CAPPER_CLEARANCE_Z,
        BASE_Z + CAP_STAGE_Z,
    );

    let loop_service = keepout_cage(
        "closed_fraction_archive_sample_loop_robot_handoff_keepout",
        LOOP_HANDOFF_X_POS,
        LOOP_HANDOFF_Y_POS,
        LOOP_HANDOFF_X + 54.0,
        LOOP_HANDOFF_Y + 56.0,
        152.0,
        BASE_Z + LOOP_HANDOFF_Z,
    );

    let front_pull = keepout_cage(
        "closed_fraction_archive_front_service_pull_keepout",
        0.0,
        -(MODULE_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0 - 12.0),
        MODULE_X - 92.0,
        FRONT_SERVICE_CLEARANCE,
        92.0,
        BASE_Z,
    );

    needle + capper + loop_service + front_pull
}

fn keepout_cage(
    name: &str,
    center_x: f64,
    center_y: f64,
    width: f64,
    depth: f64,
    height: f64,
    bottom_z: f64,
) -> Part {
    let t = 4.0;
    let bottom_mid_z = bottom_z + t / 2.0;
    let top_mid_z = bottom_z + height - t / 2.0;
    let post_mid_z = bottom_z + height / 2.0;

    let mut cage = Part::empty(format!("{name}_cage"));
    for (level, z) in [("bottom", bottom_mid_z), ("top", top_mid_z)] {
        let front = centered_cube(format!("{name}_{level}_front_rail"), width, t, t).translate(
            center_x,
            center_y - depth / 2.0,
            z,
        );
        let rear = centered_cube(format!("{name}_{level}_rear_rail"), width, t, t).translate(
            center_x,
            center_y + depth / 2.0,
            z,
        );
        let left = centered_cube(format!("{name}_{level}_left_rail"), t, depth, t).translate(
            center_x - width / 2.0,
            center_y,
            z,
        );
        let right = centered_cube(format!("{name}_{level}_right_rail"), t, depth, t).translate(
            center_x + width / 2.0,
            center_y,
            z,
        );
        cage = cage + front + rear + left + right;
    }

    for (i, (x, y)) in [
        (center_x - width / 2.0, center_y - depth / 2.0),
        (center_x + width / 2.0, center_y - depth / 2.0),
        (center_x - width / 2.0, center_y + depth / 2.0),
        (center_x + width / 2.0, center_y + depth / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let post = centered_cube(format!("{name}_vertical_post_{i}"), t, t, height)
            .translate(x, y, post_mid_z);
        cage = cage + post;
    }

    let land = centered_cube(format!("{name}_keepout_label_land"), 72.0, 12.0, 3.0).translate(
        center_x,
        center_y - depth / 2.0 - 8.0,
        bottom_z + 18.0,
    );
    cage + land
}

fn locator_points() -> [(f64, f64); 10] {
    [
        (
            COLD_BLOCK_X_POS - COLD_BLOCK_X / 2.0 + 28.0,
            COLD_BLOCK_Y_POS - COLD_BLOCK_Y / 2.0 + 28.0,
        ),
        (
            COLD_BLOCK_X_POS + COLD_BLOCK_X / 2.0 - 28.0,
            COLD_BLOCK_Y_POS - COLD_BLOCK_Y / 2.0 + 28.0,
        ),
        (
            COLD_BLOCK_X_POS - COLD_BLOCK_X / 2.0 + 28.0,
            COLD_BLOCK_Y_POS + COLD_BLOCK_Y / 2.0 - 28.0,
        ),
        (
            COLD_BLOCK_X_POS + COLD_BLOCK_X / 2.0 - 28.0,
            COLD_BLOCK_Y_POS + COLD_BLOCK_Y / 2.0 - 28.0,
        ),
        (
            LOOP_HANDOFF_X_POS - LOOP_HANDOFF_X / 2.0 + 30.0,
            LOOP_HANDOFF_Y_POS,
        ),
        (
            LOOP_HANDOFF_X_POS + LOOP_HANDOFF_X / 2.0 - 30.0,
            LOOP_HANDOFF_Y_POS,
        ),
        (CAP_STAGE_X_POS - CAP_STAGE_X / 2.0 + 24.0, CAP_STAGE_Y_POS),
        (CAP_STAGE_X_POS + CAP_STAGE_X / 2.0 - 24.0, CAP_STAGE_Y_POS),
        (CUSTODY_X_POS - CUSTODY_X / 2.0 + 24.0, CUSTODY_Y_POS),
        (CUSTODY_X_POS + CUSTODY_X / 2.0 - 24.0, CUSTODY_Y_POS),
    ]
}

fn barcode_land_specs() -> [(f64, f64, f64, f64); BARCODE_LANDS] {
    [
        (COLD_BLOCK_X_POS, COLD_BLOCK_Y_POS - 128.0, 132.0, 18.0),
        (COLD_BLOCK_X_POS, COLD_BLOCK_Y_POS + 128.0, 132.0, 18.0),
        (CAP_STAGE_X_POS, CAP_STAGE_Y_POS - 98.0, 116.0, 18.0),
        (CAP_STAGE_X_POS, CAP_STAGE_Y_POS + 98.0, 116.0, 18.0),
        (CUSTODY_X_POS, CUSTODY_Y_POS - 96.0, 118.0, 18.0),
        (CUSTODY_X_POS, CUSTODY_Y_POS + 96.0, 118.0, 18.0),
        (
            LOOP_HANDOFF_X_POS - 96.0,
            LOOP_HANDOFF_Y_POS - 76.0,
            104.0,
            18.0,
        ),
        (
            LOOP_HANDOFF_X_POS + 96.0,
            LOOP_HANDOFF_Y_POS - 76.0,
            104.0,
            18.0,
        ),
    ]
}

fn lot_time_land_specs() -> [(f64, f64, f64); LOT_TIME_LANDS] {
    [
        (-(MODULE_X / 2.0 - 88.0), -(MODULE_Y / 2.0 - 44.0), 116.0),
        (0.0, -(MODULE_Y / 2.0 - 44.0), 146.0),
        (MODULE_X / 2.0 - 88.0, -(MODULE_Y / 2.0 - 44.0), 116.0),
        (-(MODULE_X / 2.0 - 88.0), MODULE_Y / 2.0 - 44.0, 116.0),
        (0.0, MODULE_Y / 2.0 - 44.0, 146.0),
        (MODULE_X / 2.0 - 88.0, MODULE_Y / 2.0 - 44.0, 116.0),
    ]
}

fn fraction_x(col: usize) -> f64 {
    SBS_A1_X_FROM_LEFT - SBS_PLATE_X / 2.0 + col as f64 * SBS_PITCH
}

fn fraction_y(row: usize) -> f64 {
    SBS_PLATE_Y / 2.0 - SBS_A1_Y_FROM_TOP - row as f64 * SBS_PITCH
}

fn sample_lane_x(lane: usize) -> f64 {
    (lane as f64 - (SAMPLE_LANES as f64 - 1.0) / 2.0) * LOOP_LANE_PITCH
}

fn custody_slot_x(col: usize) -> f64 {
    (col as f64 - (CUSTODY_COLS as f64 - 1.0) / 2.0) * 26.0
}

fn custody_slot_y(row: usize) -> f64 {
    (row as f64 - (CUSTODY_ROWS as f64 - 1.0) / 2.0) * 27.0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sample_fraction_archive_module_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn sbs_fraction_archive_has_standard_count_and_pitch() {
        assert_eq!(FRACTION_POSITIONS, 96);
        assert_eq!(SBS_COLS * SBS_ROWS, FRACTION_POSITIONS);
        assert!((fraction_x(0) + fraction_x(SBS_COLS - 1)).abs() < 1.0e-9);
        assert!((fraction_y(0) + fraction_y(SBS_ROWS - 1)).abs() < 1.0e-9);
        assert!(TUBE_BORE_D + TUBE_KEY_SLOT_X < SBS_PITCH);
        assert!(fraction_x(0) > -SBS_PLATE_X / 2.0);
        assert!(fraction_x(SBS_COLS - 1) < SBS_PLATE_X / 2.0);
        assert!(fraction_y(0) < SBS_PLATE_Y / 2.0);
        assert!(fraction_y(SBS_ROWS - 1) > -SBS_PLATE_Y / 2.0);
    }

    #[test]
    fn cold_block_and_tube_nest_fit_inside_deck() {
        assert!(COLD_BLOCK_X_POS - COLD_BLOCK_X / 2.0 > -MODULE_X / 2.0 + TRAY_RIM_W);
        assert!(COLD_BLOCK_X_POS + COLD_BLOCK_X / 2.0 < MODULE_X / 2.0 - TRAY_RIM_W);
        assert!(COLD_BLOCK_Y_POS - COLD_BLOCK_Y / 2.0 > -MODULE_Y / 2.0 + TRAY_RIM_W);
        assert!(COLD_BLOCK_Y_POS + COLD_BLOCK_Y / 2.0 < MODULE_Y / 2.0 - TRAY_RIM_W);
        assert!(COLD_BLOCK_POCKET_X > TUBE_NEST_X);
        assert!(COLD_BLOCK_POCKET_Y > TUBE_NEST_Y);
        assert!(COLD_BLOCK_STANDOFF_Z > 0.0);
    }

    #[test]
    fn loop_handoff_and_flush_waste_cover_every_sampling_lane() {
        assert_eq!(SAMPLE_LANES, 8);
        assert!((sample_lane_x(0) + sample_lane_x(SAMPLE_LANES - 1)).abs() < 1.0e-9);
        assert!(sample_lane_x(0).abs() < LOOP_HANDOFF_X / 2.0 - 40.0);
        assert!(sample_lane_x(SAMPLE_LANES - 1).abs() < FLUSH_WASTE_X / 2.0 - 40.0);
        assert!(LOOP_LANE_PITCH > SAMPLE_LOOP_LAND_X);
        assert!(WASTE_BUS_D > FLUSH_BUS_D);
        assert!(SEPTUM_PORT_D < FLUID_BORE_D);
    }

    #[test]
    fn cap_staging_and_custody_counts_match_archive_positions() {
        assert_eq!(CAP_POSITIONS, FRACTION_POSITIONS);
        assert_eq!(CUSTODY_SLOTS, 24);
        assert_eq!(CUSTODY_COLS * CUSTODY_ROWS, CUSTODY_SLOTS);
        assert!(SEAL_MAT_POCKET_X > SBS_PLATE_X);
        assert!(CAP_WELL_D < SBS_PITCH);
        assert!(custody_slot_x(0).abs() < CUSTODY_X / 2.0 - 24.0);
        assert!(custody_slot_y(0).abs() < CUSTODY_Y / 2.0 - 24.0);
    }

    #[test]
    fn traceability_lands_and_service_keepouts_are_explicit() {
        assert_eq!(barcode_land_specs().len(), BARCODE_LANDS);
        assert_eq!(lot_time_land_specs().len(), LOT_TIME_LANDS);
        assert_eq!(ROBOT_KEEPOUTS, 4);
        assert!(FRONT_SERVICE_CLEARANCE >= 260.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 150.0);
        assert!(NEEDLE_APPROACH_CLEARANCE_Z >= 200.0);
        assert!(CAPPER_CLEARANCE_Z >= 160.0);
    }
}
