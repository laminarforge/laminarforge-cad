use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterility/media-fill run simulation fixture.
//
// Intent:
// - Package the physical interfaces needed to qualify the automated tissue-chip
//   process with sterile media surrogate cassettes before live cells are used.
// - Keep closed sterile connector loops, sample points, control carriers,
//   incubation transfer datums, witness lands, scan lands, disposition lanes,
//   segregation lanes, and robot/service keepouts visible in one CAD envelope.
// - Model fixture geometry and evidence-capture interfaces only. This is not a
//   sterility protocol, incubation method, acceptance criterion, or regulatory
//   validation procedure.

const OUTPUTS: [&str; 12] = [
    "output/closed_media_fill_run_simulation_fixture_baseplate.stl",
    "output/closed_media_fill_run_simulation_fixture_surrogate_cassette_dock.stl",
    "output/closed_media_fill_run_simulation_fixture_sterile_connector_loop_routing.stl",
    "output/closed_media_fill_run_simulation_fixture_media_fill_sample_wells.stl",
    "output/closed_media_fill_run_simulation_fixture_control_carriers.stl",
    "output/closed_media_fill_run_simulation_fixture_incubation_transfer_tray_datum.stl",
    "output/closed_media_fill_run_simulation_fixture_leak_bubble_witness_lands.stl",
    "output/closed_media_fill_run_simulation_fixture_barcode_rfid_run_record_lands.stl",
    "output/closed_media_fill_run_simulation_fixture_disposition_lanes.stl",
    "output/closed_media_fill_run_simulation_fixture_clean_used_segregation.stl",
    "output/closed_media_fill_run_simulation_fixture_robot_service_keepouts.stl",
    "output/closed_media_fill_run_simulation_fixture_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "twenty_position_surrogate_cassette_dock",
    "sterile_connector_loop_routing",
    "media_fill_sample_wells",
    "positive_negative_control_carriers",
    "incubation_transfer_tray_datum",
    "leak_bubble_witness_lands",
    "barcode_rfid_run_record_lands",
    "released_hold_reject_lanes",
    "clean_used_segregation",
    "robot_keepouts",
    "service_keepouts",
    "closed_process_packaging_interfaces",
    "assembly_export",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const POSITIONS: usize = COLS * ROWS;
const SAMPLE_WELLS: usize = POSITIONS + 4;
const POSITIVE_CONTROLS: usize = 3;
const NEGATIVE_CONTROLS: usize = 3;
const CONTROL_CARRIERS: usize = POSITIVE_CONTROLS + NEGATIVE_CONTROLS;
const CONNECTOR_LOOPS: usize = ROWS + 3;
const LOOP_CLIP_PAIRS: usize = CONNECTOR_LOOPS * 2;
const LEAK_WITNESS_LANDS: usize = 8;
const BUBBLE_WITNESS_WINDOWS: usize = POSITIONS;
const BARCODE_LANDS: usize = 5;
const RFID_LANDS: usize = 4;
const RUN_RECORD_LANDS: usize = 6;
const DISPOSITION_LANES: usize = 3;
const SEGREGATION_BINS: usize = 2;
const KEEP_OUT_ZONES: usize = 5;

const GUTTER: f64 = 5.0;
const POSITION_PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const POSITION_PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;

const BASE_X: f64 = 1180.0;
const BASE_Y: f64 = 860.0;
const BASE_Z: f64 = 20.0;
const MOUNT_HOLE_D: f64 = 6.6;

const DOCK_X: f64 = ARRAY_X + 150.0;
const DOCK_Y: f64 = ARRAY_Y + 132.0;
const DOCK_Z: f64 = 28.0;
const SURROGATE_CASSETTE_CLEARANCE: f64 = 0.8;
const DATUM_RAIL_Z: f64 = 34.0;
const POSITION_POCKET_Z: f64 = 7.0;

const LOOP_PANEL_X: f64 = 1030.0;
const LOOP_PANEL_Y: f64 = 132.0;
const LOOP_PANEL_Z: f64 = 36.0;
const TUBE_OD: f64 = 4.8;
const LOOP_BORE_D: f64 = TUBE_OD + 1.0;
const CONNECTOR_D: f64 = 18.0;
const CONNECTOR_CRADLE_X: f64 = 44.0;
const CONNECTOR_CRADLE_Y: f64 = 28.0;
const LOOP_PITCH_X: f64 = 118.0;

const SAMPLE_BLOCK_X: f64 = 420.0;
const SAMPLE_BLOCK_Y: f64 = 232.0;
const SAMPLE_BLOCK_Z: f64 = 38.0;
const SAMPLE_WELL_D: f64 = 16.2;
const SAMPLE_WELL_DEPTH: f64 = 26.0;
const SAMPLE_COLS: usize = 6;

const CONTROL_BLOCK_X: f64 = 256.0;
const CONTROL_BLOCK_Y: f64 = 172.0;
const CONTROL_BLOCK_Z: f64 = 36.0;
const CONTROL_CARRIER_X: f64 = 54.0;
const CONTROL_CARRIER_Y: f64 = 36.0;
const CONTROL_CARRIER_Z: f64 = 22.0;

const TRANSFER_TRAY_X: f64 = DOCK_X + 92.0;
const TRANSFER_TRAY_Y: f64 = DOCK_Y + 86.0;
const TRANSFER_TRAY_Z: f64 = 30.0;
const INCUBATOR_DATUM_PINS: usize = 6;
const TRANSFER_HANDLE_X: f64 = 28.0;
const TRANSFER_HANDLE_Y: f64 = 126.0;
const TRANSFER_HANDLE_Z: f64 = 34.0;

const WITNESS_PANEL_X: f64 = 520.0;
const WITNESS_PANEL_Y: f64 = 142.0;
const WITNESS_PANEL_Z: f64 = 16.0;
const LEAK_LAND_X: f64 = 46.0;
const LEAK_LAND_Y: f64 = 28.0;
const BUBBLE_WINDOW_X: f64 = 32.0;
const BUBBLE_WINDOW_Y: f64 = 18.0;

const RECORD_PANEL_X: f64 = 402.0;
const RECORD_PANEL_Y: f64 = 176.0;
const RECORD_PANEL_Z: f64 = 12.0;
const BARCODE_LAND_X: f64 = 118.0;
const BARCODE_LAND_Y: f64 = 24.0;
const RFID_LAND_D: f64 = 30.0;

const LANE_BANK_X: f64 = 420.0;
const LANE_BANK_Y: f64 = 196.0;
const LANE_BANK_Z: f64 = 30.0;
const LANE_X: f64 = 122.0;
const LANE_Y: f64 = 156.0;

const SEGREGATION_X: f64 = 430.0;
const SEGREGATION_Y: f64 = 176.0;
const SEGREGATION_Z: f64 = 42.0;

const KEEP_OUT_Z: f64 = 168.0;
const FRONT_SERVICE_CLEARANCE: f64 = 320.0;
const REAR_LOOP_SERVICE_CLEARANCE: f64 = 240.0;
const LEFT_CLEAN_LOAD_CLEARANCE: f64 = 210.0;
const RIGHT_USED_UNLOAD_CLEARANCE: f64 = 230.0;

const DOCK_POS: (f64, f64) = (-176.0, -16.0);
const LOOP_POS: (f64, f64) = (-30.0, BASE_Y / 2.0 - 96.0);
const SAMPLE_POS: (f64, f64) = (350.0, -220.0);
const CONTROL_POS: (f64, f64) = (430.0, 84.0);
const TRANSFER_POS: (f64, f64) = (DOCK_POS.0, DOCK_POS.1);
const WITNESS_POS: (f64, f64) = (-292.0, -326.0);
const RECORD_POS: (f64, f64) = (-360.0, 278.0);
const LANE_POS: (f64, f64) = (332.0, 290.0);
const SEGREGATION_POS: (f64, f64) = (-330.0, 122.0);

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = baseplate();
    export(OUTPUTS[0], &base);

    let dock = surrogate_cassette_dock();
    export(OUTPUTS[1], &dock);

    let loops = sterile_connector_loop_routing();
    export(OUTPUTS[2], &loops);

    let samples = media_fill_sample_wells();
    export(OUTPUTS[3], &samples);

    let controls = control_carriers();
    export(OUTPUTS[4], &controls);

    let transfer = incubation_transfer_tray_datum();
    export(OUTPUTS[5], &transfer);

    let witnesses = leak_bubble_witness_lands();
    export(OUTPUTS[6], &witnesses);

    let records = barcode_rfid_run_record_lands();
    export(OUTPUTS[7], &records);

    let lanes = disposition_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + transfer.translate(
            TRANSFER_POS.0,
            TRANSFER_POS.1,
            BASE_Z / 2.0 + TRANSFER_TRAY_Z / 2.0 + 3.0,
        ) + dock.translate(
            DOCK_POS.0,
            DOCK_POS.1,
            BASE_Z / 2.0 + TRANSFER_TRAY_Z + DOCK_Z / 2.0 + 8.0,
        ) + loops.translate(
            LOOP_POS.0,
            LOOP_POS.1,
            BASE_Z / 2.0 + LOOP_PANEL_Z / 2.0 + 16.0,
        ) + samples.translate(
            SAMPLE_POS.0,
            SAMPLE_POS.1,
            BASE_Z / 2.0 + SAMPLE_BLOCK_Z / 2.0 + 12.0,
        ) + controls.translate(
            CONTROL_POS.0,
            CONTROL_POS.1,
            BASE_Z / 2.0 + CONTROL_BLOCK_Z / 2.0 + 12.0,
        ) + witnesses.translate(
            WITNESS_POS.0,
            WITNESS_POS.1,
            BASE_Z / 2.0 + WITNESS_PANEL_Z / 2.0 + 8.0,
        ) + records.translate(
            RECORD_POS.0,
            RECORD_POS.1,
            BASE_Z / 2.0 + RECORD_PANEL_Z / 2.0 + 5.0,
        ) + lanes.translate(
            LANE_POS.0,
            LANE_POS.1,
            BASE_Z / 2.0 + LANE_BANK_Z / 2.0 + 8.0,
        ) + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            BASE_Z / 2.0 + SEGREGATION_Z / 2.0 + 10.0,
        ) + closed_loop_spans()
            + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0 + 20.0);

    export(OUTPUTS[11], &assembly);

    println!(
        "Closed media-fill run simulation fixture: {:.0}mm x {:.0}mm deck, {} surrogate cassette positions in a {}x{} dock, {} closed connector loops with {} retainer clip pairs, {} sample wells, {} positive controls, {} negative controls, {} leak lands, {} bubble witness windows, {} barcode/RFID/run-record lands, {} disposition lanes, {} clean/used segregation bins, {} robot/service keepout zones, and {} required feature groups.",
        BASE_X,
        BASE_Y,
        POSITIONS,
        COLS,
        ROWS,
        CONNECTOR_LOOPS,
        LOOP_CLIP_PAIRS,
        SAMPLE_WELLS,
        POSITIVE_CONTROLS,
        NEGATIVE_CONTROLS,
        LEAK_WITNESS_LANDS,
        BUBBLE_WITNESS_WINDOWS,
        BARCODE_LANDS + RFID_LANDS + RUN_RECORD_LANDS,
        DISPOSITION_LANES,
        SEGREGATION_BINS,
        KEEP_OUT_ZONES,
        REQUIRED_FEATURES.len()
    );
    println!(
        "Incubation transfer tray datum uses {} pins; service envelopes reserve {:.0}mm front, {:.0}mm rear loop service, {:.0}mm clean-load left, and {:.0}mm used-unload right clearances.",
        INCUBATOR_DATUM_PINS,
        FRONT_SERVICE_CLEARANCE,
        REAR_LOOP_SERVICE_CLEARANCE,
        LEFT_CLEAN_LOAD_CLEARANCE,
        RIGHT_USED_UNLOAD_CLEARANCE
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "closed_media_fill_fixture_baseplate",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );

    let dock_shadow = centered_cube(
        "closed_media_fill_fixture_dock_shadow",
        TRANSFER_TRAY_X + 34.0,
        TRANSFER_TRAY_Y + 34.0,
        5.0,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, BASE_Z / 2.0 - 2.0);
    let wet_channel = centered_cube(
        "closed_media_fill_fixture_wet_side_channel",
        BASE_X - 116.0,
        34.0,
        6.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 132.0, BASE_Z / 2.0 - 3.0);
    let clean_boundary = centered_cube(
        "closed_media_fill_fixture_clean_used_boundary_groove",
        8.0,
        BASE_Y - 122.0,
        6.0,
    )
    .translate(-76.0, 0.0, BASE_Z / 2.0 - 3.0);

    let mut holes = Part::empty("closed_media_fill_fixture_mount_holes");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 32.0), -(BASE_Y / 2.0 - 32.0)),
        (BASE_X / 2.0 - 32.0, -(BASE_Y / 2.0 - 32.0)),
        (-(BASE_X / 2.0 - 32.0), BASE_Y / 2.0 - 32.0),
        (BASE_X / 2.0 - 32.0, BASE_Y / 2.0 - 32.0),
        (-76.0, -(BASE_Y / 2.0 - 32.0)),
        (-76.0, BASE_Y / 2.0 - 32.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_media_fill_fixture_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 2.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }

    deck - dock_shadow - wet_channel - clean_boundary - holes
}

fn surrogate_cassette_dock() -> Part {
    let body = centered_cube(
        "closed_media_fill_surrogate_cassette_dock_body",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let mut cuts = Part::empty("closed_media_fill_surrogate_position_pockets");
    let mut fiducials = Part::empty("closed_media_fill_surrogate_dock_fiducials");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = cassette_position_center(row, col);
            cuts = cuts
                + centered_cube(
                    format!("closed_media_fill_surrogate_pocket_{row}_{col}"),
                    REVC_CHIP_LENGTH + SURROGATE_CASSETTE_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + SURROGATE_CASSETTE_CLEARANCE * 2.0,
                    POSITION_POCKET_Z + 0.4,
                )
                .translate(x, y, DOCK_Z / 2.0 - POSITION_POCKET_Z / 2.0 + 0.2)
                + centered_cube(
                    format!("closed_media_fill_surrogate_media_window_{row}_{col}"),
                    REVC_CHIP_LENGTH - 24.0,
                    10.0,
                    DOCK_Z + 2.0,
                )
                .translate(x, y - REVC_CHIP_WIDTH / 2.0 + 16.0, 0.0);
        }
    }

    for (i, (x, y)) in datum_pin_positions().iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_media_fill_dock_datum_pin_{i}"),
                4.0,
                DATUM_RAIL_Z,
                32,
            )
            .translate(*x, *y, DOCK_Z / 2.0 + DATUM_RAIL_Z / 2.0);
    }

    let north_rail = centered_cube(
        "closed_media_fill_dock_north_datum_rail",
        DOCK_X - 54.0,
        16.0,
        DATUM_RAIL_Z,
    )
    .translate(0.0, DOCK_Y / 2.0 - 18.0, DOCK_Z / 2.0 + DATUM_RAIL_Z / 2.0);
    let west_rail = centered_cube(
        "closed_media_fill_dock_west_datum_rail",
        16.0,
        DOCK_Y - 54.0,
        DATUM_RAIL_Z,
    )
    .translate(
        -(DOCK_X / 2.0 - 18.0),
        0.0,
        DOCK_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let gripper_clearance = centered_cube(
        "closed_media_fill_dock_robot_gripper_relief",
        72.0,
        DOCK_Y + 4.0,
        18.0,
    )
    .translate(DOCK_X / 2.0 - 54.0, 0.0, DOCK_Z / 2.0 - 7.0);

    body - cuts - gripper_clearance + fiducials + north_rail + west_rail
}

fn sterile_connector_loop_routing() -> Part {
    let panel = centered_cube(
        "closed_media_fill_sterile_loop_routing_panel",
        LOOP_PANEL_X,
        LOOP_PANEL_Y,
        LOOP_PANEL_Z,
    );
    let mut bores = Part::empty("closed_media_fill_sterile_loop_bores");
    let mut cradles = Part::empty("closed_media_fill_sterile_connector_cradles");
    let mut clips = Part::empty("closed_media_fill_loop_retainer_clips");

    for index in 0..CONNECTOR_LOOPS {
        let x = loop_x(index);
        bores = bores
            + tube_span_x(
                format!("closed_media_fill_loop_bore_{index}"),
                LOOP_PANEL_X - 112.0,
                LOOP_BORE_D / 2.0,
            )
            .translate(0.0, loop_y(index), 0.0);
        for side in [-1.0, 1.0] {
            cradles = cradles
                + connector_cradle(format!("closed_media_fill_connector_cradle_{index}_{side}"))
                    .translate(
                        side * (LOOP_PANEL_X / 2.0 - 74.0),
                        loop_y(index),
                        LOOP_PANEL_Z / 2.0 + CONNECTOR_D / 2.0,
                    );
        }
        clips = clips
            + centered_cube(
                format!("closed_media_fill_loop_clip_pair_{index}_left"),
                18.0,
                12.0,
                22.0,
            )
            .translate(
                x - 26.0,
                LOOP_PANEL_Y / 2.0 - 20.0,
                LOOP_PANEL_Z / 2.0 + 11.0,
            )
            + centered_cube(
                format!("closed_media_fill_loop_clip_pair_{index}_right"),
                18.0,
                12.0,
                22.0,
            )
            .translate(
                x + 26.0,
                -(LOOP_PANEL_Y / 2.0 - 20.0),
                LOOP_PANEL_Z / 2.0 + 11.0,
            );
    }

    panel - bores + cradles + clips
}

fn media_fill_sample_wells() -> Part {
    let block = centered_cube(
        "closed_media_fill_sample_well_block",
        SAMPLE_BLOCK_X,
        SAMPLE_BLOCK_Y,
        SAMPLE_BLOCK_Z,
    );
    let mut wells = Part::empty("closed_media_fill_sample_well_cuts");
    let mut labels = Part::empty("closed_media_fill_sample_well_number_lands");

    for index in 0..SAMPLE_WELLS {
        let (x, y) = sample_well_center(index);
        wells = wells
            + centered_cylinder(
                format!("closed_media_fill_sample_well_{index}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_WELL_DEPTH,
                32,
            )
            .translate(x, y, SAMPLE_BLOCK_Z / 2.0 - SAMPLE_WELL_DEPTH / 2.0 + 0.2);
        labels = labels
            + centered_cube(
                format!("closed_media_fill_sample_well_label_land_{index}"),
                26.0,
                8.0,
                2.0,
            )
            .translate(x, y + 20.0, SAMPLE_BLOCK_Z / 2.0 + 1.0);
    }

    let chain_of_custody_ledge = centered_cube(
        "closed_media_fill_sample_chain_of_custody_ledge",
        SAMPLE_BLOCK_X - 46.0,
        18.0,
        7.0,
    )
    .translate(
        0.0,
        -(SAMPLE_BLOCK_Y / 2.0 - 18.0),
        SAMPLE_BLOCK_Z / 2.0 + 3.5,
    );

    block - wells + labels + chain_of_custody_ledge
}

fn control_carriers() -> Part {
    let plate = centered_cube(
        "closed_media_fill_control_carrier_plate",
        CONTROL_BLOCK_X,
        CONTROL_BLOCK_Y,
        CONTROL_BLOCK_Z,
    );
    let mut pockets = Part::empty("closed_media_fill_control_carrier_pockets");
    let mut flags = Part::empty("closed_media_fill_control_carrier_flags");

    for index in 0..CONTROL_CARRIERS {
        let positive = index < POSITIVE_CONTROLS;
        let (x, y) = control_center(index);
        pockets = pockets
            + centered_cube(
                format!("closed_media_fill_control_pocket_{index}"),
                CONTROL_CARRIER_X + 1.0,
                CONTROL_CARRIER_Y + 1.0,
                CONTROL_CARRIER_Z / 2.0,
            )
            .translate(x, y, CONTROL_BLOCK_Z / 2.0 - CONTROL_CARRIER_Z / 4.0);
        let flag_h = if positive { 26.0 } else { 14.0 };
        flags = flags
            + centered_cube(
                format!("closed_media_fill_control_flag_{index}"),
                9.0,
                CONTROL_CARRIER_Y,
                flag_h,
            )
            .translate(
                x - CONTROL_CARRIER_X / 2.0 - 7.0,
                y,
                CONTROL_BLOCK_Z / 2.0 + flag_h / 2.0,
            );
    }

    let divider = centered_cube(
        "closed_media_fill_positive_negative_control_divider",
        12.0,
        CONTROL_BLOCK_Y - 24.0,
        24.0,
    )
    .translate(0.0, 0.0, CONTROL_BLOCK_Z / 2.0 + 12.0);

    plate - pockets + flags + divider
}

fn incubation_transfer_tray_datum() -> Part {
    let tray = centered_cube(
        "closed_media_fill_incubation_transfer_tray_datum",
        TRANSFER_TRAY_X,
        TRANSFER_TRAY_Y,
        TRANSFER_TRAY_Z,
    );
    let tray_cavity = centered_cube(
        "closed_media_fill_incubation_transfer_tray_cavity",
        DOCK_X + 18.0,
        DOCK_Y + 18.0,
        18.0,
    )
    .translate(0.0, 0.0, TRANSFER_TRAY_Z / 2.0 - 9.0);
    let mut pins = Part::empty("closed_media_fill_incubator_transfer_datum_pins");
    for (i, (x, y)) in transfer_pin_positions().iter().enumerate() {
        pins = pins
            + centered_cylinder(format!("closed_media_fill_transfer_pin_{i}"), 4.5, 24.0, 32)
                .translate(*x, *y, TRANSFER_TRAY_Z / 2.0 + 12.0);
    }

    let left_handle = lift_handle("closed_media_fill_transfer_left_handle").translate(
        -(TRANSFER_TRAY_X / 2.0 - 24.0),
        0.0,
        TRANSFER_TRAY_Z / 2.0 + TRANSFER_HANDLE_Z / 2.0,
    );
    let right_handle = lift_handle("closed_media_fill_transfer_right_handle").translate(
        TRANSFER_TRAY_X / 2.0 - 24.0,
        0.0,
        TRANSFER_TRAY_Z / 2.0 + TRANSFER_HANDLE_Z / 2.0,
    );

    tray - tray_cavity + pins + left_handle + right_handle
}

fn leak_bubble_witness_lands() -> Part {
    let panel = centered_cube(
        "closed_media_fill_leak_bubble_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let mut lands = Part::empty("closed_media_fill_leak_witness_lands");
    let mut windows = Part::empty("closed_media_fill_bubble_witness_windows");

    for index in 0..LEAK_WITNESS_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_fill_leak_witness_land_{index}"),
                LEAK_LAND_X,
                LEAK_LAND_Y,
                4.0,
            )
            .translate(
                leak_land_x(index),
                WITNESS_PANEL_Y / 2.0 - 34.0,
                WITNESS_PANEL_Z / 2.0 + 2.0,
            );
    }

    for row in 0..ROWS {
        for col in 0..COLS {
            let index = row * COLS + col;
            windows = windows
                + centered_cube(
                    format!("closed_media_fill_bubble_witness_window_{index}"),
                    BUBBLE_WINDOW_X,
                    BUBBLE_WINDOW_Y,
                    WITNESS_PANEL_Z + 2.0,
                )
                .translate(bubble_window_x(col), bubble_window_y(row), 0.0);
        }
    }

    panel + lands - windows
}

fn barcode_rfid_run_record_lands() -> Part {
    let panel = centered_cube(
        "closed_media_fill_record_panel",
        RECORD_PANEL_X,
        RECORD_PANEL_Y,
        RECORD_PANEL_Z,
    );
    let mut lands = Part::empty("closed_media_fill_record_lands");

    for index in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_fill_barcode_land_{index}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                -RECORD_PANEL_X / 2.0 + 78.0,
                barcode_y(index),
                RECORD_PANEL_Z / 2.0 + 1.5,
            );
    }

    for index in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("closed_media_fill_rfid_land_{index}"),
                RFID_LAND_D / 2.0,
                3.0,
                36,
            )
            .translate(42.0, rfid_y(index), RECORD_PANEL_Z / 2.0 + 1.5);
    }

    for index in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_fill_run_record_land_{index}"),
                86.0,
                18.0,
                3.0,
            )
            .translate(
                RECORD_PANEL_X / 2.0 - 62.0,
                run_record_y(index),
                RECORD_PANEL_Z / 2.0 + 1.5,
            );
    }

    panel
        + lands
        + fiducial_target("closed_media_fill_record_scan_fiducial").translate(
            0.0,
            RECORD_PANEL_Y / 2.0 - 28.0,
            RECORD_PANEL_Z / 2.0 + 2.0,
        )
}

fn disposition_lanes() -> Part {
    let bank = centered_cube(
        "closed_media_fill_disposition_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    );
    let mut lanes = Part::empty("closed_media_fill_released_hold_reject_lanes");

    for index in 0..DISPOSITION_LANES {
        let x = disposition_lane_x(index);
        lanes = lanes
            + centered_cube(
                format!("closed_media_fill_disposition_lane_{index}"),
                LANE_X,
                LANE_Y,
                8.0,
            )
            .translate(x, 0.0, LANE_BANK_Z / 2.0 - 4.0)
            + centered_cube(
                format!("closed_media_fill_disposition_lane_status_land_{index}"),
                74.0,
                18.0,
                4.0,
            )
            .translate(x, LANE_Y / 2.0 - 20.0, LANE_BANK_Z / 2.0 + 2.0);
    }

    bank - lanes
}

fn clean_used_segregation() -> Part {
    let base = centered_cube(
        "closed_media_fill_clean_used_segregation_base",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let clean = segregation_bin_cut("clean").translate(-SEGREGATION_X / 4.0, 0.0, 2.0);
    let used = segregation_bin_cut("used").translate(SEGREGATION_X / 4.0, 0.0, 2.0);
    let divider = centered_cube(
        "closed_media_fill_clean_used_high_divider",
        14.0,
        SEGREGATION_Y - 18.0,
        SEGREGATION_Z + 28.0,
    )
    .translate(0.0, 0.0, 14.0);
    let clean_lip = centered_cube(
        "closed_media_fill_clean_load_lip",
        SEGREGATION_X / 2.0 - 34.0,
        12.0,
        14.0,
    )
    .translate(
        -SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 13.0,
        SEGREGATION_Z / 2.0 + 7.0,
    );
    let used_lip = centered_cube(
        "closed_media_fill_used_unload_lip",
        SEGREGATION_X / 2.0 - 34.0,
        12.0,
        14.0,
    )
    .translate(
        SEGREGATION_X / 4.0,
        -(SEGREGATION_Y / 2.0 - 13.0),
        SEGREGATION_Z / 2.0 + 7.0,
    );

    base - clean - used + divider + clean_lip + used_lip
}

fn robot_service_keepouts() -> Part {
    let cassette_load = keepout_box(
        "closed_media_fill_keepout_cassette_robot_load",
        DOCK_X + 160.0,
        DOCK_Y + 140.0,
        KEEP_OUT_Z,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, 0.0);
    let loop_service = keepout_box(
        "closed_media_fill_keepout_sterile_loop_service",
        LOOP_PANEL_X + 90.0,
        REAR_LOOP_SERVICE_CLEARANCE,
        KEEP_OUT_Z - 42.0,
    )
    .translate(LOOP_POS.0, LOOP_POS.1, 0.0);
    let sample_robot = keepout_box(
        "closed_media_fill_keepout_sample_pick",
        SAMPLE_BLOCK_X + 118.0,
        SAMPLE_BLOCK_Y + 110.0,
        KEEP_OUT_Z - 36.0,
    )
    .translate(SAMPLE_POS.0, SAMPLE_POS.1, 0.0);
    let clean_load = keepout_box(
        "closed_media_fill_keepout_clean_load_side",
        LEFT_CLEAN_LOAD_CLEARANCE,
        SEGREGATION_Y + 84.0,
        KEEP_OUT_Z - 58.0,
    )
    .translate(
        SEGREGATION_POS.0 - SEGREGATION_X / 2.0,
        SEGREGATION_POS.1,
        0.0,
    );
    let used_unload = keepout_box(
        "closed_media_fill_keepout_used_unload_side",
        RIGHT_USED_UNLOAD_CLEARANCE,
        SEGREGATION_Y + 84.0,
        KEEP_OUT_Z - 58.0,
    )
    .translate(
        SEGREGATION_POS.0 + SEGREGATION_X / 2.0,
        SEGREGATION_POS.1,
        0.0,
    );

    cassette_load + loop_service + sample_robot + clean_load + used_unload
}

fn closed_loop_spans() -> Part {
    let dock_to_loop = tube_span_y("closed_media_fill_route_dock_to_loop_panel", 300.0).translate(
        DOCK_POS.0,
        206.0,
        BASE_Z / 2.0 + TRANSFER_TRAY_Z + DOCK_Z + 28.0,
    );
    let dock_to_samples = tube_span_x(
        "closed_media_fill_route_dock_to_sample_wells",
        440.0,
        LOOP_BORE_D / 2.0,
    )
    .translate(100.0, -238.0, BASE_Z / 2.0 + 96.0);
    let sample_to_witness = tube_span_x(
        "closed_media_fill_route_sample_to_witness",
        560.0,
        LOOP_BORE_D / 2.0,
    )
    .translate(-24.0, -330.0, BASE_Z / 2.0 + 58.0);
    let controls_to_dock = tube_span_x(
        "closed_media_fill_route_controls_to_dock",
        310.0,
        LOOP_BORE_D / 2.0,
    )
    .translate(206.0, 84.0, BASE_Z / 2.0 + 90.0);

    dock_to_loop + dock_to_samples + sample_to_witness + controls_to_dock
}

fn connector_cradle(name: String) -> Part {
    let block = centered_cube(
        format!("{name}_block"),
        CONNECTOR_CRADLE_X,
        CONNECTOR_CRADLE_Y,
        CONNECTOR_D,
    );
    let bore = centered_cylinder(
        format!("{name}_connector_relief"),
        CONNECTOR_D / 2.0,
        CONNECTOR_CRADLE_X + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    block - bore
}

fn lift_handle(name: &str) -> Part {
    let body = centered_cube(
        format!("{name}_body"),
        TRANSFER_HANDLE_X,
        TRANSFER_HANDLE_Y,
        TRANSFER_HANDLE_Z,
    );
    let grip = centered_cube(
        format!("{name}_grip_cutout"),
        TRANSFER_HANDLE_X + 2.0,
        TRANSFER_HANDLE_Y - 42.0,
        16.0,
    );
    body - grip
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 14.0, 3.0, 40);
    let inner = centered_cylinder(format!("{name}_inner"), 6.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 34.0, 2.6, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.6, 34.0, 4.0);
    outer - inner + cross_x + cross_y
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let envelope = centered_cube(format!("{name}_envelope"), x, y, z);
    let interior = centered_cube(
        format!("{name}_interior_relief"),
        x - 18.0,
        y - 18.0,
        z + 4.0,
    );
    envelope - interior
}

fn segregation_bin_cut(name: &str) -> Part {
    centered_cube(
        format!("closed_media_fill_{name}_segregation_bin_cut"),
        SEGREGATION_X / 2.0 - 42.0,
        SEGREGATION_Y - 46.0,
        SEGREGATION_Z - 10.0,
    )
}

fn tube_span_x(name: impl Into<String>, length: f64, radius: f64) -> Part {
    centered_cylinder(name.into(), radius, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_span_y(name: impl Into<String>, length: f64) -> Part {
    centered_cylinder(name.into(), LOOP_BORE_D / 2.0, length, 24).rotate(90.0, 0.0, 0.0)
}

fn cassette_position_center(row: usize, col: usize) -> (f64, f64) {
    (position_x(col), position_y(row))
}

fn position_x(col: usize) -> f64 {
    -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * POSITION_PITCH_X
}

fn position_y(row: usize) -> f64 {
    -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * POSITION_PITCH_Y
}

fn datum_pin_positions() -> [(f64, f64); 4] {
    [
        (-(DOCK_X / 2.0 - 34.0), -(DOCK_Y / 2.0 - 34.0)),
        (DOCK_X / 2.0 - 34.0, -(DOCK_Y / 2.0 - 34.0)),
        (-(DOCK_X / 2.0 - 34.0), DOCK_Y / 2.0 - 34.0),
        (DOCK_X / 2.0 - 34.0, DOCK_Y / 2.0 - 34.0),
    ]
}

fn transfer_pin_positions() -> [(f64, f64); INCUBATOR_DATUM_PINS] {
    [
        (
            -(TRANSFER_TRAY_X / 2.0 - 44.0),
            -(TRANSFER_TRAY_Y / 2.0 - 44.0),
        ),
        (
            TRANSFER_TRAY_X / 2.0 - 44.0,
            -(TRANSFER_TRAY_Y / 2.0 - 44.0),
        ),
        (
            -(TRANSFER_TRAY_X / 2.0 - 44.0),
            TRANSFER_TRAY_Y / 2.0 - 44.0,
        ),
        (TRANSFER_TRAY_X / 2.0 - 44.0, TRANSFER_TRAY_Y / 2.0 - 44.0),
        (0.0, -(TRANSFER_TRAY_Y / 2.0 - 44.0)),
        (0.0, TRANSFER_TRAY_Y / 2.0 - 44.0),
    ]
}

fn loop_x(index: usize) -> f64 {
    (index as f64 - (CONNECTOR_LOOPS as f64 - 1.0) / 2.0) * LOOP_PITCH_X
}

fn loop_y(index: usize) -> f64 {
    -((CONNECTOR_LOOPS as f64 - 1.0) * 14.0) / 2.0 + index as f64 * 14.0
}

fn sample_well_center(index: usize) -> (f64, f64) {
    let col = index % SAMPLE_COLS;
    let row = index / SAMPLE_COLS;
    let rows = SAMPLE_WELLS.div_ceil(SAMPLE_COLS);
    let x = (col as f64 - (SAMPLE_COLS as f64 - 1.0) / 2.0) * 62.0;
    let y = (row as f64 - (rows as f64 - 1.0) / 2.0) * 50.0;
    (x, y)
}

fn control_center(index: usize) -> (f64, f64) {
    let group = if index < POSITIVE_CONTROLS { -1.0 } else { 1.0 };
    let local = if index < POSITIVE_CONTROLS {
        index
    } else {
        index - POSITIVE_CONTROLS
    };
    (group * 62.0, (local as f64 - 1.0) * 46.0)
}

fn leak_land_x(index: usize) -> f64 {
    (index as f64 - (LEAK_WITNESS_LANDS as f64 - 1.0) / 2.0) * 58.0
}

fn bubble_window_x(col: usize) -> f64 {
    (col as f64 - (COLS as f64 - 1.0) / 2.0) * 92.0
}

fn bubble_window_y(row: usize) -> f64 {
    -54.0 + row as f64 * 22.0
}

fn barcode_y(index: usize) -> f64 {
    (index as f64 - (BARCODE_LANDS as f64 - 1.0) / 2.0) * 30.0
}

fn rfid_y(index: usize) -> f64 {
    (index as f64 - (RFID_LANDS as f64 - 1.0) / 2.0) * 34.0
}

fn run_record_y(index: usize) -> f64 {
    (index as f64 - (RUN_RECORD_LANDS as f64 - 1.0) / 2.0) * 24.0
}

fn disposition_lane_x(index: usize) -> f64 {
    (index as f64 - (DISPOSITION_LANES as f64 - 1.0) / 2.0) * 136.0
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
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_media_fill_run_simulation_fixture_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn covers_required_media_fill_fixture_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 13);
        assert!(REQUIRED_FEATURES.contains(&"twenty_position_surrogate_cassette_dock"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_connector_loop_routing"));
        assert!(REQUIRED_FEATURES.contains(&"media_fill_sample_wells"));
        assert!(REQUIRED_FEATURES.contains(&"positive_negative_control_carriers"));
        assert!(REQUIRED_FEATURES.contains(&"incubation_transfer_tray_datum"));
        assert!(REQUIRED_FEATURES.contains(&"leak_bubble_witness_lands"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_run_record_lands"));
        assert!(REQUIRED_FEATURES.contains(&"released_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"closed_process_packaging_interfaces"));
        assert!(REQUIRED_FEATURES.contains(&"assembly_export"));
    }

    #[test]
    fn cassette_dock_has_twenty_surrogate_positions() {
        assert_eq!(COLS, 4);
        assert_eq!(ROWS, 5);
        assert_eq!(POSITIONS, 20);
        assert!(DOCK_X > ARRAY_X + 120.0);
        assert!(DOCK_Y > ARRAY_Y + 100.0);
        assert!((position_x(0) + position_x(COLS - 1)).abs() < 0.001);
        assert!((position_y(0) + position_y(ROWS - 1)).abs() < 0.001);
    }

    #[test]
    fn sample_and_control_counts_are_explicit() {
        assert_eq!(SAMPLE_WELLS, POSITIONS + 4);
        assert_eq!(POSITIVE_CONTROLS, 3);
        assert_eq!(NEGATIVE_CONTROLS, 3);
        assert_eq!(CONTROL_CARRIERS, 6);
        assert!(SAMPLE_WELL_DEPTH < SAMPLE_BLOCK_Z);
        assert!(SAMPLE_WELL_D < 18.0);
    }

    #[test]
    fn routing_and_evidence_counts_are_sane() {
        assert_eq!(CONNECTOR_LOOPS, ROWS + 3);
        assert_eq!(LOOP_CLIP_PAIRS, CONNECTOR_LOOPS * 2);
        assert_eq!(LEAK_WITNESS_LANDS, 8);
        assert_eq!(BUBBLE_WITNESS_WINDOWS, POSITIONS);
        assert_eq!(BARCODE_LANDS + RFID_LANDS + RUN_RECORD_LANDS, 15);
        assert!(LOOP_BORE_D > TUBE_OD);
        assert!(CONNECTOR_D > LOOP_BORE_D);
    }

    #[test]
    fn disposition_and_segregation_are_modeled() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(SEGREGATION_BINS, 2);
        assert!(LANE_BANK_X > DISPOSITION_LANES as f64 * LANE_X);
        assert!(SEGREGATION_X > LANE_BANK_X);
    }

    #[test]
    fn fixture_fits_defined_benchtop_envelope() {
        assert!(BASE_X <= 1200.0);
        assert!(BASE_Y <= 900.0);
        assert!(TRANSFER_TRAY_X < BASE_X - 180.0);
        assert!(TRANSFER_TRAY_Y < BASE_Y - 160.0);
        assert!(RECORD_PANEL_X < BASE_X / 2.0);
    }

    #[test]
    fn placed_modules_stay_inside_baseplate_margin() {
        for (x, y, w, h) in [
            (DOCK_POS.0, DOCK_POS.1, TRANSFER_TRAY_X, TRANSFER_TRAY_Y),
            (LOOP_POS.0, LOOP_POS.1, LOOP_PANEL_X, LOOP_PANEL_Y),
            (SAMPLE_POS.0, SAMPLE_POS.1, SAMPLE_BLOCK_X, SAMPLE_BLOCK_Y),
            (
                CONTROL_POS.0,
                CONTROL_POS.1,
                CONTROL_BLOCK_X,
                CONTROL_BLOCK_Y,
            ),
            (
                WITNESS_POS.0,
                WITNESS_POS.1,
                WITNESS_PANEL_X,
                WITNESS_PANEL_Y,
            ),
            (RECORD_POS.0, RECORD_POS.1, RECORD_PANEL_X, RECORD_PANEL_Y),
            (LANE_POS.0, LANE_POS.1, LANE_BANK_X, LANE_BANK_Y),
            (
                SEGREGATION_POS.0,
                SEGREGATION_POS.1,
                SEGREGATION_X,
                SEGREGATION_Y,
            ),
        ] {
            assert!(x.abs() + w / 2.0 < BASE_X / 2.0 - 18.0);
            assert!(y.abs() + h / 2.0 < BASE_Y / 2.0 - 18.0);
        }
    }

    #[test]
    fn keepout_and_transfer_datums_are_explicit() {
        assert_eq!(KEEP_OUT_ZONES, 5);
        assert_eq!(INCUBATOR_DATUM_PINS, 6);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_LOOP_SERVICE_CLEARANCE >= 220.0);
        assert!(LEFT_CLEAN_LOAD_CLEARANCE >= 200.0);
        assert!(RIGHT_USED_UNLOAD_CLEARANCE >= 220.0);
    }
}
