use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed sample label print/apply/verify station for tissue-chip and
// cell-culture automation.
//
// Intent:
// - Keep sample identity in a closed, sterile path from cassette-fed label stock
//   through print, application, verifier scan, and reject/quarantine handling.
// - Model mechanical interfaces for a purchased thermal label engine, applicator,
//   barcode/RFID verification cameras, liner take-up, robot gripper datums, and
//   tube/chip carrier nests.
// - Leave software controls, label content, 21 CFR Part 11 records, scanner
//   qualification, and sterility validation outside this CAD placeholder.

const OUTPUTS: [&str; 10] = [
    "output/closed_sample_label_print_apply_verify_station_cleanable_deck.stl",
    "output/closed_sample_label_print_apply_verify_station_sterile_label_cassette_dock.stl",
    "output/closed_sample_label_print_apply_verify_station_print_apply_platen.stl",
    "output/closed_sample_label_print_apply_verify_station_sample_tube_chip_carrier_nests.stl",
    "output/closed_sample_label_print_apply_verify_station_barcode_rfid_verifier_camera_bridge.stl",
    "output/closed_sample_label_print_apply_verify_station_reject_quarantine_lane.stl",
    "output/closed_sample_label_print_apply_verify_station_label_liner_waste_spool.stl",
    "output/closed_sample_label_print_apply_verify_station_robot_gripper_datums.stl",
    "output/closed_sample_label_print_apply_verify_station_sterile_cover_service_keepouts.stl",
    "output/closed_sample_label_print_apply_verify_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_CONTROLS: [&str; 8] = [
    "sterile_label_cassette_dock",
    "thermal_print_apply_platen",
    "sample_tube_chip_carrier_nests",
    "barcode_rfid_verifier_camera_bridge",
    "reject_quarantine_lane",
    "label_liner_waste_spool",
    "robot_gripper_datums",
    "closed_sterile_cover",
];

const DECK_X: f64 = 1220.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 20.0;
const DECK_RIM_W: f64 = 16.0;
const DECK_RIM_Z: f64 = 32.0;
const DECK_RECESS_DEPTH: f64 = 5.0;
const WIPE_GUTTER_W: f64 = 12.0;
const DRAIN_PORT_D: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CASSETTE_CENTER: (f64, f64) = (-430.0, 178.0);
const CASSETTE_X: f64 = 300.0;
const CASSETTE_Y: f64 = 250.0;
const CASSETTE_Z: f64 = 64.0;
const LABEL_CASSETTE_X: f64 = 190.0;
const LABEL_CASSETTE_Y: f64 = 118.0;
const LABEL_STOCK_W: f64 = 42.0;
const LABEL_PITCH: f64 = 32.0;
const STERILE_CASSETTE_DATUM_PINS: usize = 4;

const PLATEN_CENTER: (f64, f64) = (-30.0, 150.0);
const PLATEN_X: f64 = 440.0;
const PLATEN_Y: f64 = 290.0;
const PLATEN_Z: f64 = 56.0;
const PRINT_ENGINE_X: f64 = 310.0;
const PRINT_ENGINE_Y: f64 = 118.0;
const PRINT_ENGINE_Z: f64 = 122.0;
const APPLY_PAD_X: f64 = 78.0;
const APPLY_PAD_Y: f64 = 58.0;
const APPLY_PAD_Z: f64 = 24.0;
const LABEL_APPLY_FORCE_PADS: usize = 4;
const LABEL_PRESENT_WINDOWS: usize = 3;

const CARRIER_CENTER: (f64, f64) = (-280.0, -190.0);
const CARRIER_X: f64 = 520.0;
const CARRIER_Y: f64 = 250.0;
const CARRIER_Z: f64 = 42.0;
const TUBE_WELL_COUNT: usize = 12;
const TUBE_WELL_COLS: usize = 6;
const TUBE_WELL_D: f64 = 14.2;
const TUBE_WELL_PITCH_X: f64 = 38.0;
const TUBE_WELL_PITCH_Y: f64 = 56.0;
const TUBE_WELL_ORIGIN: (f64, f64) = (-145.0, 0.0);
const CHIP_NEST_COUNT: usize = 4;
const CHIP_NEST_COLS: usize = 2;
const CHIP_POCKET_X: f64 = REVC_CHIP_LENGTH + 6.0;
const CHIP_POCKET_Y: f64 = REVC_CHIP_WIDTH + 6.0;
const CHIP_PITCH_X: f64 = 154.0;
const CHIP_PITCH_Y: f64 = 112.0;
const CHIP_NEST_ORIGIN: (f64, f64) = (90.0, 0.0);
const CARRIER_LOCK_COUNT: usize = 6;

const BRIDGE_CENTER: (f64, f64) = (120.0, -34.0);
const BRIDGE_SPAN_X: f64 = 870.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 52.0;
const BRIDGE_BEAM_Y: f64 = 78.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_UNDERSIDE_Z: f64 = 188.0;
const CAMERA_COUNT: usize = 3;
const CAMERA_PITCH_X: f64 = 180.0;
const LED_SEGMENTS: usize = 8;
const RFID_ANTENNA_X: f64 = 148.0;
const RFID_ANTENNA_Y: f64 = 88.0;

const REJECT_CENTER: (f64, f64) = (410.0, 140.0);
const REJECT_X: f64 = 315.0;
const REJECT_Y: f64 = 300.0;
const REJECT_Z: f64 = 52.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 72.0;
const STATUS_SLOT_Y: f64 = 48.0;
const STATUS_SLOT_PITCH_X: f64 = 92.0;
const STATUS_SLOT_PITCH_Y: f64 = 58.0;
const REJECT_HIGH_WALL_Z: f64 = 72.0;
const QUARANTINE_REJECT_GAP_MIN: f64 = 20.0;

const WASTE_CENTER: (f64, f64) = (405.0, -225.0);
const WASTE_X: f64 = 270.0;
const WASTE_Y: f64 = 220.0;
const WASTE_Z: f64 = 62.0;
const TAKEUP_SPOOL_D: f64 = 126.0;
const LINER_ROLLER_D: f64 = 28.0;
const LINER_GUIDE_SLOTS: usize = 5;

const COVER_CENTER: (f64, f64) = (30.0, 52.0);
const COVER_X: f64 = 980.0;
const COVER_Y: f64 = 600.0;
const COVER_Z: f64 = 238.0;
const COVER_WALL_T: f64 = 12.0;
const COVER_ROOF_Z: f64 = 16.0;
const TRANSFER_SLOT_X: f64 = 238.0;
const TRANSFER_SLOT_Z: f64 = 58.0;

const ROBOT_GRIPPER_DATUMS: usize = 12;
const FIDUCIAL_COUNT: usize = 10;
const DATUM_PIN_COUNT: usize = 6;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 112.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 84.0;
const RIGHT_SPOOL_SERVICE_CLEARANCE_X: f64 = 126.0;
const CAMERA_SERVICE_CLEARANCE_Z: f64 = 270.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_deck();
    export(OUTPUTS[0], &deck);

    let cassette = sterile_label_cassette_dock();
    export(OUTPUTS[1], &cassette);

    let platen = print_apply_platen();
    export(OUTPUTS[2], &platen);

    let carriers = sample_tube_chip_carrier_nests();
    export(OUTPUTS[3], &carriers);

    let bridge = barcode_rfid_verifier_camera_bridge();
    export(OUTPUTS[4], &bridge);

    let reject = reject_quarantine_lane();
    export(OUTPUTS[5], &reject);

    let waste = label_liner_waste_spool();
    export(OUTPUTS[6], &waste);

    let datums = robot_gripper_datums();
    export(OUTPUTS[7], &datums);

    let cover = sterile_cover_service_keepouts();
    export(OUTPUTS[8], &cover);

    let assembly = station_assembly();
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed sample label print/apply/verify station:");
    println!("  Cleanable deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Label path:                  {LABEL_STOCK_W:.0}mm sterile cassette stock at {LABEL_PITCH:.0}mm pitch, {} present windows, {} cassette datum pins, liner take-up spool {TAKEUP_SPOOL_D:.0}mm OD",
        LABEL_PRESENT_WINDOWS, STERILE_CASSETTE_DATUM_PINS
    );
    println!(
        "  Sample nests:                {TUBE_WELL_COUNT} tube wells plus {CHIP_NEST_COUNT} Rev C chip carrier pockets ({:.1}mm x {:.1}mm x {:.1}mm chip envelope)",
        REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT
    );
    println!(
        "  Print/apply controls:        {LABEL_APPLY_FORCE_PADS} force pads under the label tamp and closed label presentation platen"
    );
    println!(
        "  Verification:                {CAMERA_COUNT} camera pods, {LED_SEGMENTS} LED segments, RFID antenna {:.0}mm x {:.0}mm, bridge underside {:.0}mm above deck",
        RFID_ANTENNA_X, RFID_ANTENNA_Y, bridge_clearance_above_deck()
    );
    println!(
        "  Status handling:             {STATUS_LANES} lanes x {STATUS_SLOTS_PER_LANE} slots, dedicated reject high-wall lane and quarantine gap {:.0}mm",
        quarantine_reject_gap()
    );
    println!(
        "  Automation datums:           {ROBOT_GRIPPER_DATUMS} gripper pads, {DATUM_PIN_COUNT} datum pins, {FIDUCIAL_COUNT} fiducial lands"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    cleanable_deck()
        + sterile_label_cassette_dock()
        + print_apply_platen()
        + sample_tube_chip_carrier_nests()
        + barcode_rfid_verifier_camera_bridge()
        + reject_quarantine_lane()
        + label_liner_waste_spool()
        + robot_gripper_datums()
        + sterile_cover_service_keepouts()
}

fn cleanable_deck() -> Part {
    let deck = centered_cube(
        "closed_sample_label_station_cleanable_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let mut recesses = Part::empty("closed_sample_label_station_module_recesses");
    for (name, center, x, y) in module_specs() {
        recesses = recesses + top_recess(name, center, x + 22.0, y + 22.0, DECK_RECESS_DEPTH);
    }

    deck - recesses - deck_mount_holes() - label_web_trough() - wipe_gutters()
        + deck_perimeter_lips()
        + rear_cable_datum_rail()
        + deck_leak_witness_ribs()
}

fn top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(
        format!("closed_sample_label_station_{name}_deck_recess"),
        x,
        y,
        depth + 0.2,
    )
    .translate(center.0, center.1, DECK_Z - depth / 2.0 + 0.1)
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_sample_label_station_deck_mount_holes");
    for (index, (x, y)) in deck_mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_sample_label_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn label_web_trough() -> Part {
    let cassette_to_print = centered_cube(
        "closed_sample_label_station_label_web_trough_cassette_to_print",
        520.0,
        LABEL_STOCK_W + 12.0,
        6.0,
    )
    .translate(-210.0, 135.0, DECK_Z - 2.4);
    let print_to_waste = centered_cube(
        "closed_sample_label_station_liner_web_trough_print_to_waste",
        510.0,
        LABEL_STOCK_W + 10.0,
        6.0,
    )
    .translate(225.0, -36.0, DECK_Z - 2.4);
    let peel_drain = centered_cylinder(
        "closed_sample_label_station_label_path_drain",
        DRAIN_PORT_D / 2.0,
        52.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(22.0, -DECK_Y / 2.0 + 16.0, DECK_Z - 7.0);

    cassette_to_print + print_to_waste + peel_drain
}

fn wipe_gutters() -> Part {
    let left = centered_cube(
        "closed_sample_label_station_left_wipe_gutter",
        WIPE_GUTTER_W,
        DECK_Y - 108.0,
        6.0,
    )
    .translate(-DECK_X / 2.0 + 56.0, 0.0, DECK_Z - 2.5);
    let center = centered_cube(
        "closed_sample_label_station_cross_wipe_gutter",
        DECK_X - 178.0,
        WIPE_GUTTER_W,
        6.0,
    )
    .translate(24.0, -82.0, DECK_Z - 2.5);
    let front_sump = centered_cube(
        "closed_sample_label_station_front_sump",
        DECK_X - 230.0,
        18.0,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 48.0, DECK_Z - 2.6);

    left + center + front_sump
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "closed_sample_label_station_rear_raised_cleanable_lip",
        DECK_X - 84.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 28.0, DECK_Z + DECK_RIM_Z / 2.0);
    let left = centered_cube(
        "closed_sample_label_station_left_raised_cleanable_lip",
        DECK_RIM_W,
        DECK_Y - 116.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 28.0, 0.0, DECK_Z + DECK_RIM_Z / 2.0);
    let right = centered_cube(
        "closed_sample_label_station_right_short_spool_lip",
        DECK_RIM_W,
        DECK_Y - 202.0,
        22.0,
    )
    .translate(DECK_X / 2.0 - 28.0, -22.0, DECK_Z + 11.0);
    let front = centered_cube(
        "closed_sample_label_station_front_low_robot_lip",
        DECK_X - 330.0,
        10.0,
        14.0,
    )
    .translate(-80.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 7.0);

    rear + left + right + front
}

fn rear_cable_datum_rail() -> Part {
    let rail = centered_cube(
        "closed_sample_label_station_rear_scanner_cable_rail",
        DECK_X - 230.0,
        16.0,
        24.0,
    )
    .translate(36.0, DECK_Y / 2.0 - 70.0, DECK_Z + 12.0);

    let mut glands = Part::empty("closed_sample_label_station_rear_cable_glands");
    for index in 0..6 {
        glands = glands
            + centered_cylinder(
                format!("closed_sample_label_station_rear_cable_gland_{index}"),
                7.0,
                22.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -265.0 + index as f64 * 106.0,
                DECK_Y / 2.0 - 70.0,
                DECK_Z + 12.0,
            );
    }

    rail - glands
}

fn deck_leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_sample_label_station_leak_witness_ribs");
    for (index, x) in [-420.0, -280.0, -140.0, 0.0, 140.0, 280.0, 420.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_sample_label_station_leak_witness_rib_{index}"),
                7.0,
                DECK_Y - 170.0,
                5.0,
            )
            .translate(x, 0.0, DECK_Z + 2.5);
    }
    ribs
}

fn sterile_label_cassette_dock() -> Part {
    let body = centered_cube(
        "closed_sample_label_station_sterile_label_cassette_dock_body",
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    let cassette_pocket = centered_cube(
        "closed_sample_label_station_sterile_label_cassette_pocket",
        LABEL_CASSETTE_X,
        LABEL_CASSETTE_Y,
        22.0,
    )
    .translate(0.0, 28.0, CASSETTE_Z / 2.0 - 9.0);
    let supply_spool_recess = centered_cylinder(
        "closed_sample_label_station_sterile_supply_spool_recess",
        56.0,
        24.0,
        64,
    )
    .translate(-54.0, 38.0, CASSETTE_Z / 2.0 - 10.0);
    let cassette_key = centered_cube(
        "closed_sample_label_station_asymmetric_cassette_key_notch",
        28.0,
        18.0,
        CASSETTE_Z + 2.0,
    )
    .translate(-(LABEL_CASSETTE_X / 2.0 - 24.0), -34.0, 0.0);
    let peel_slot = centered_cube(
        "closed_sample_label_station_sterile_label_peel_slot",
        LABEL_STOCK_W + 12.0,
        36.0,
        18.0,
    )
    .translate(76.0, -CASSETTE_Y / 2.0 + 12.0, 8.0);
    let rfid_window = centered_cube(
        "closed_sample_label_station_cassette_rfid_read_window",
        92.0,
        52.0,
        8.0,
    )
    .translate(82.0, 70.0, CASSETTE_Z / 2.0 - 3.0);

    let gasket = cassette_gasket_rim();
    let pins = cassette_datum_pins();
    let rollers = cassette_label_exit_rollers();

    let part = body - cassette_pocket - supply_spool_recess - cassette_key - peel_slot
        + gasket
        + pins
        + rollers
        + rfid_window;

    place_on_deck(part, CASSETTE_CENTER, CASSETTE_Z)
}

fn cassette_gasket_rim() -> Part {
    let outer = centered_cube(
        "closed_sample_label_station_cassette_gasket_outer_land",
        LABEL_CASSETTE_X + 28.0,
        LABEL_CASSETTE_Y + 30.0,
        5.0,
    )
    .translate(0.0, 28.0, CASSETTE_Z / 2.0 + 2.5);
    let inner = centered_cube(
        "closed_sample_label_station_cassette_gasket_inner_relief",
        LABEL_CASSETTE_X + 8.0,
        LABEL_CASSETTE_Y + 10.0,
        6.0,
    )
    .translate(0.0, 28.0, CASSETTE_Z / 2.0 + 3.0);
    let front_lip = centered_cube(
        "closed_sample_label_station_cassette_sterile_transfer_lip",
        CASSETTE_X - 44.0,
        12.0,
        24.0,
    )
    .translate(0.0, -CASSETTE_Y / 2.0 + 20.0, CASSETTE_Z / 2.0 + 12.0);

    outer - inner + front_lip
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("closed_sample_label_station_cassette_datum_pins");
    for (index, (x, y)) in [
        (-112.0, -88.0),
        (112.0, -88.0),
        (-112.0, 102.0),
        (112.0, 102.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_sample_label_station_cassette_pin_boss_{index}"),
            10.0,
            9.0,
            32,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 4.5);
        let clearance = centered_cylinder(
            format!("closed_sample_label_station_cassette_pin_clearance_{index}"),
            2.6,
            10.0,
            24,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 4.5);
        pins = pins + (boss - clearance);
    }
    pins
}

fn cassette_label_exit_rollers() -> Part {
    let drive = centered_cylinder(
        "closed_sample_label_station_cassette_exit_drive_roller",
        10.0,
        LABEL_STOCK_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(76.0, -CASSETTE_Y / 2.0 - 4.0, 16.0);
    let idler = centered_cylinder(
        "closed_sample_label_station_cassette_exit_idler_roller",
        8.0,
        LABEL_STOCK_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(112.0, -CASSETTE_Y / 2.0 - 4.0, 26.0);
    let sterile_brush = centered_cube(
        "closed_sample_label_station_cassette_exit_sterile_brush_placeholder",
        LABEL_STOCK_W + 42.0,
        8.0,
        26.0,
    )
    .translate(94.0, -CASSETTE_Y / 2.0 - 18.0, 16.0);

    drive + idler + sterile_brush
}

fn print_apply_platen() -> Part {
    let platen = centered_cube(
        "closed_sample_label_station_print_apply_platen_body",
        PLATEN_X,
        PLATEN_Y,
        PLATEN_Z,
    );
    let vacuum_bed = centered_cube(
        "closed_sample_label_station_label_vacuum_bed_cut",
        188.0,
        LABEL_STOCK_W + 18.0,
        9.0,
    )
    .translate(-42.0, -18.0, PLATEN_Z / 2.0 - 3.8);
    let apply_window = centered_cube(
        "closed_sample_label_station_sample_apply_window_cut",
        112.0,
        78.0,
        PLATEN_Z + 4.0,
    )
    .translate(118.0, -60.0, 0.0);
    let print_head_clearance = centered_cube(
        "closed_sample_label_station_thermal_print_head_clearance",
        PRINT_ENGINE_X - 70.0,
        34.0,
        22.0,
    )
    .translate(-52.0, 58.0, PLATEN_Z / 2.0 - 4.0);

    let thermal_engine = centered_cube(
        "closed_sample_label_station_thermal_printer_engine_envelope",
        PRINT_ENGINE_X,
        PRINT_ENGINE_Y,
        PRINT_ENGINE_Z,
    )
    .translate(-58.0, 76.0, PLATEN_Z / 2.0 + PRINT_ENGINE_Z / 2.0 + 5.0);
    let peel_bar = centered_cylinder(
        "closed_sample_label_station_sharp_peel_plate_radius_placeholder",
        8.0,
        LABEL_STOCK_W + 76.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(82.0, -20.0, PLATEN_Z / 2.0 + 11.0);
    let tamp_pad = centered_cube(
        "closed_sample_label_station_vertical_apply_tamp_pad",
        APPLY_PAD_X,
        APPLY_PAD_Y,
        APPLY_PAD_Z,
    )
    .translate(118.0, -60.0, PLATEN_Z / 2.0 + 44.0);
    let sample_anvil = centered_cube(
        "closed_sample_label_station_under_sample_apply_anvil",
        APPLY_PAD_X + 18.0,
        APPLY_PAD_Y + 16.0,
        16.0,
    )
    .translate(118.0, -60.0, PLATEN_Z / 2.0 + 8.0);

    let present_windows = label_present_windows();
    let force_pads = label_apply_force_pads();
    let web_guides = platen_web_guides();

    let part = platen - vacuum_bed - apply_window - print_head_clearance
        + thermal_engine
        + peel_bar
        + tamp_pad
        + sample_anvil
        + present_windows
        + force_pads
        + web_guides;

    place_on_deck(part, PLATEN_CENTER, PLATEN_Z)
}

fn label_present_windows() -> Part {
    let mut windows = Part::empty("closed_sample_label_station_label_present_windows");
    for index in 0..LABEL_PRESENT_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_sample_label_station_label_present_window_{index}"),
                70.0,
                18.0,
                5.0,
            )
            .translate(
                -124.0 + index as f64 * 58.0,
                -PLATEN_Y / 2.0 + 38.0,
                PLATEN_Z / 2.0 + 2.5,
            );
    }
    windows
}

fn label_apply_force_pads() -> Part {
    let mut pads = Part::empty("closed_sample_label_station_apply_force_sensor_pads");
    for (index, (x, y)) in [
        (72.0, -106.0),
        (164.0, -106.0),
        (72.0, -14.0),
        (164.0, -14.0),
    ]
    .into_iter()
    .enumerate()
    {
        pads = pads
            + centered_cube(
                format!("closed_sample_label_station_apply_force_sensor_pad_{index}"),
                24.0,
                18.0,
                4.0,
            )
            .translate(x, y, PLATEN_Z / 2.0 + 2.0);
    }
    pads
}

fn platen_web_guides() -> Part {
    let left_rail = centered_cube(
        "closed_sample_label_station_web_left_guide_rail",
        300.0,
        4.0,
        12.0,
    )
    .translate(-72.0, -(LABEL_STOCK_W / 2.0 + 7.0), PLATEN_Z / 2.0 + 6.0);
    let right_rail = centered_cube(
        "closed_sample_label_station_web_right_guide_rail",
        300.0,
        4.0,
        12.0,
    )
    .translate(-72.0, LABEL_STOCK_W / 2.0 + 7.0, PLATEN_Z / 2.0 + 6.0);
    let datum_stop = centered_cube(
        "closed_sample_label_station_print_apply_hard_stop",
        12.0,
        LABEL_STOCK_W + 26.0,
        22.0,
    )
    .translate(176.0, -18.0, PLATEN_Z / 2.0 + 11.0);

    left_rail + right_rail + datum_stop
}

fn sample_tube_chip_carrier_nests() -> Part {
    let tray = centered_cube(
        "closed_sample_label_station_sample_carrier_nest_tray",
        CARRIER_X,
        CARRIER_Y,
        CARRIER_Z,
    );
    let basin = centered_cube(
        "closed_sample_label_station_sample_carrier_washdown_basin",
        CARRIER_X - 36.0,
        CARRIER_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, CARRIER_Z / 2.0 - 3.5);

    let part = tray - basin - tube_well_cuts() - chip_nest_cuts()
        + tube_well_rims()
        + chip_nest_datums()
        + sample_label_target_lands()
        + carrier_lock_rails();

    place_on_deck(part, CARRIER_CENTER, CARRIER_Z)
}

fn tube_well_cuts() -> Part {
    let mut wells = Part::empty("closed_sample_label_station_tube_well_cuts");
    for index in 0..TUBE_WELL_COUNT {
        let (x, y) = tube_well_position(index);
        wells = wells
            + centered_cylinder(
                format!("closed_sample_label_station_sample_tube_well_cut_{index}"),
                TUBE_WELL_D / 2.0,
                CARRIER_Z + 3.0,
                32,
            )
            .translate(x, y, 1.0);
    }
    wells
}

fn tube_well_rims() -> Part {
    let mut rims = Part::empty("closed_sample_label_station_tube_well_rims");
    for index in 0..TUBE_WELL_COUNT {
        let (x, y) = tube_well_position(index);
        let outer = centered_cylinder(
            format!("closed_sample_label_station_sample_tube_well_rim_{index}"),
            TUBE_WELL_D / 2.0 + 2.6,
            3.0,
            32,
        )
        .translate(x, y, CARRIER_Z / 2.0 + 1.5);
        let inner = centered_cylinder(
            format!("closed_sample_label_station_sample_tube_well_rim_open_{index}"),
            TUBE_WELL_D / 2.0 + 0.4,
            3.4,
            32,
        )
        .translate(x, y, CARRIER_Z / 2.0 + 1.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn chip_nest_cuts() -> Part {
    let mut pockets = Part::empty("closed_sample_label_station_chip_carrier_pocket_cuts");
    for index in 0..CHIP_NEST_COUNT {
        let (x, y) = chip_nest_position(index);
        pockets = pockets
            + centered_cube(
                format!("closed_sample_label_station_rev_c_chip_carrier_pocket_{index}"),
                CHIP_POCKET_X,
                CHIP_POCKET_Y,
                12.0,
            )
            .translate(x, y, CARRIER_Z / 2.0 - 5.0);
    }
    pockets
}

fn chip_nest_datums() -> Part {
    let mut datums = Part::empty("closed_sample_label_station_chip_carrier_datums");
    for index in 0..CHIP_NEST_COUNT {
        let (cx, cy) = chip_nest_position(index);
        for (corner, (dx, dy)) in [
            (-(CHIP_POCKET_X / 2.0 - 13.0), -(CHIP_POCKET_Y / 2.0 - 13.0)),
            (CHIP_POCKET_X / 2.0 - 13.0, -(CHIP_POCKET_Y / 2.0 - 13.0)),
            (-(CHIP_POCKET_X / 2.0 - 13.0), CHIP_POCKET_Y / 2.0 - 13.0),
            (CHIP_POCKET_X / 2.0 - 13.0, CHIP_POCKET_Y / 2.0 - 13.0),
        ]
        .into_iter()
        .enumerate()
        {
            let boss = centered_cylinder(
                format!("closed_sample_label_station_chip_{index}_datum_boss_{corner}"),
                5.0,
                5.0,
                24,
            )
            .translate(cx + dx, cy + dy, CARRIER_Z / 2.0 + 2.5);
            let pin_clearance = centered_cylinder(
                format!("closed_sample_label_station_chip_{index}_datum_pin_clearance_{corner}"),
                1.6,
                5.4,
                20,
            )
            .translate(cx + dx, cy + dy, CARRIER_Z / 2.0 + 2.5);
            datums = datums + (boss - pin_clearance);
        }
    }
    datums
}

fn sample_label_target_lands() -> Part {
    let mut lands = Part::empty("closed_sample_label_station_sample_label_target_lands");
    for index in 0..TUBE_WELL_COUNT {
        let (x, y) = tube_well_position(index);
        lands = lands
            + centered_cube(
                format!("closed_sample_label_station_tube_label_land_{index}"),
                34.0,
                10.0,
                3.0,
            )
            .translate(x, y - 22.0, CARRIER_Z / 2.0 + 1.5);
    }
    for index in 0..CHIP_NEST_COUNT {
        let (x, y) = chip_nest_position(index);
        lands = lands
            + centered_cube(
                format!("closed_sample_label_station_chip_label_land_{index}"),
                REVC_CHIP_LENGTH * 0.46,
                12.0,
                3.0,
            )
            .translate(x, y + REVC_CHIP_WIDTH / 2.0 + 15.0, CARRIER_Z / 2.0 + 1.5);
    }
    lands
}

fn carrier_lock_rails() -> Part {
    let rear = centered_cube(
        "closed_sample_label_station_sample_carrier_rear_lock_rail",
        CARRIER_X - 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, CARRIER_Y / 2.0 - 20.0, CARRIER_Z / 2.0 + 9.0);
    let front = centered_cube(
        "closed_sample_label_station_sample_carrier_front_lock_rail",
        CARRIER_X - 116.0,
        10.0,
        16.0,
    )
    .translate(-32.0, -CARRIER_Y / 2.0 + 20.0, CARRIER_Z / 2.0 + 8.0);
    let mut clamp_pads = Part::empty("closed_sample_label_station_sample_carrier_clamp_pads");
    for index in 0..CARRIER_LOCK_COUNT {
        clamp_pads = clamp_pads
            + centered_cube(
                format!("closed_sample_label_station_carrier_lock_pad_{index}"),
                42.0,
                18.0,
                8.0,
            )
            .translate(
                centered_index(index, CARRIER_LOCK_COUNT, 78.0),
                -CARRIER_Y / 2.0 + 52.0,
                CARRIER_Z / 2.0 + 4.0,
            );
    }

    rear + front + clamp_pads
}

fn barcode_rfid_verifier_camera_bridge() -> Part {
    let post_height = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
    let post_z = DECK_Z + post_height / 2.0;
    let left_x = BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0;
    let right_x = BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0;

    let mut posts = Part::empty("closed_sample_label_station_verifier_bridge_posts");
    for (index, (x, y)) in [
        (left_x, BRIDGE_CENTER.1 - BRIDGE_BEAM_Y / 2.0),
        (left_x, BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0),
        (right_x, BRIDGE_CENTER.1 - BRIDGE_BEAM_Y / 2.0),
        (right_x, BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_sample_label_station_verifier_bridge_post_{index}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                post_height,
            )
            .translate(x, y, post_z);
    }

    let beam = centered_cube(
        "closed_sample_label_station_verifier_camera_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    posts + beam + verifier_camera_pods() + verifier_led_segments() + rfid_antenna_panel()
}

fn verifier_camera_pods() -> Part {
    let mut pods = Part::empty("closed_sample_label_station_verifier_camera_pods");
    for index in 0..CAMERA_COUNT {
        let x = BRIDGE_CENTER.0 + centered_index(index, CAMERA_COUNT, CAMERA_PITCH_X);
        let camera_body = centered_cube(
            format!("closed_sample_label_station_barcode_verifier_camera_body_{index}"),
            52.0,
            44.0,
            36.0,
        )
        .translate(x, BRIDGE_CENTER.1 - 2.0, DECK_Z + BRIDGE_UNDERSIDE_Z - 18.0);
        let lens = centered_cylinder(
            format!("closed_sample_label_station_barcode_verifier_lens_{index}"),
            11.0,
            14.0,
            32,
        )
        .translate(x, BRIDGE_CENTER.1 - 2.0, DECK_Z + BRIDGE_UNDERSIDE_Z - 42.0);
        let focus_gauge = centered_cube(
            format!("closed_sample_label_station_camera_{index}_focus_gauge"),
            62.0,
            4.0,
            126.0,
        )
        .translate(x, BRIDGE_CENTER.1 - 52.0, DECK_Z + 92.0);
        pods = pods + camera_body + lens + focus_gauge;
    }
    pods
}

fn verifier_led_segments() -> Part {
    let mut leds = Part::empty("closed_sample_label_station_verifier_led_segments");
    for index in 0..LED_SEGMENTS {
        let x = BRIDGE_CENTER.0 + centered_index(index, LED_SEGMENTS, 94.0);
        leds = leds
            + centered_cube(
                format!("closed_sample_label_station_verifier_led_segment_{index}"),
                54.0,
                8.0,
                9.0,
            )
            .translate(
                x,
                BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0 + 5.0,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 8.0,
            );
    }
    leds
}

fn rfid_antenna_panel() -> Part {
    let panel = centered_cube(
        "closed_sample_label_station_rfid_verifier_antenna_panel",
        RFID_ANTENNA_X,
        RFID_ANTENNA_Y,
        8.0,
    )
    .translate(322.0, -34.0, DECK_Z + 58.0);
    let shield = centered_cube(
        "closed_sample_label_station_rfid_field_shield_backer",
        RFID_ANTENNA_X + 24.0,
        8.0,
        72.0,
    )
    .translate(322.0, 16.0, DECK_Z + 82.0);
    let cable_land = centered_cube(
        "closed_sample_label_station_rfid_cable_strain_land",
        44.0,
        22.0,
        18.0,
    )
    .translate(410.0, 20.0, DECK_Z + 92.0);

    panel + shield + cable_land
}

fn reject_quarantine_lane() -> Part {
    let panel = centered_cube(
        "closed_sample_label_station_reject_quarantine_lane_panel",
        REJECT_X,
        REJECT_Y,
        REJECT_Z,
    );

    let mut slot_cuts = Part::empty("closed_sample_label_station_status_slot_cuts");
    let mut separators = Part::empty("closed_sample_label_station_status_lane_separators");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_sample_label_station_status_lane_{lane}_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    REJECT_Z + 4.0,
                )
                .translate(status_lane_x(lane), status_slot_y(slot), 0.0);
        }
        separators = separators
            + centered_cube(
                format!("closed_sample_label_station_status_lane_{lane}_backstop"),
                STATUS_SLOT_X + 18.0,
                8.0,
                24.0,
            )
            .translate(
                status_lane_x(lane),
                REJECT_Y / 2.0 - 20.0,
                REJECT_Z / 2.0 + 12.0,
            );
    }
    for lane in 0..STATUS_LANES - 1 {
        let x = (status_lane_x(lane) + status_lane_x(lane + 1)) / 2.0;
        separators = separators
            + centered_cube(
                format!("closed_sample_label_station_quarantine_reject_separator_{lane}"),
                9.0,
                REJECT_Y - 34.0,
                42.0,
            )
            .translate(x, 0.0, REJECT_Z / 2.0 + 21.0);
    }

    let reject_high_wall = centered_cube(
        "closed_sample_label_station_reject_lane_high_wall",
        STATUS_SLOT_X + 30.0,
        REJECT_Y - 30.0,
        REJECT_HIGH_WALL_Z,
    )
    .translate(
        status_lane_x(STATUS_LANES - 1),
        0.0,
        REJECT_Z / 2.0 + REJECT_HIGH_WALL_Z / 2.0,
    );
    let reject_view_cut = centered_cube(
        "closed_sample_label_station_reject_lane_view_cut",
        STATUS_SLOT_X + 12.0,
        REJECT_Y - 74.0,
        REJECT_HIGH_WALL_Z - 18.0,
    )
    .translate(
        status_lane_x(STATUS_LANES - 1),
        -10.0,
        REJECT_Z / 2.0 + REJECT_HIGH_WALL_Z / 2.0 + 8.0,
    );
    let barcode_status_lands = status_barcode_lands();

    let part = panel - slot_cuts
        + separators
        + (reject_high_wall - reject_view_cut)
        + barcode_status_lands;
    place_on_deck(part, REJECT_CENTER, REJECT_Z)
}

fn status_barcode_lands() -> Part {
    let mut lands = Part::empty("closed_sample_label_station_status_barcode_lands");
    for lane in 0..STATUS_LANES {
        lands = lands
            + centered_cube(
                format!("closed_sample_label_station_status_lane_{lane}_barcode_land"),
                62.0,
                16.0,
                4.0,
            )
            .translate(
                status_lane_x(lane),
                -REJECT_Y / 2.0 + 24.0,
                REJECT_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn label_liner_waste_spool() -> Part {
    let tray = centered_cube(
        "closed_sample_label_station_label_liner_waste_spool_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let takeup_well = centered_cylinder(
        "closed_sample_label_station_liner_takeup_spool_well",
        TAKEUP_SPOOL_D / 2.0,
        WASTE_Z + 3.0,
        72,
    )
    .translate(-30.0, 26.0, 6.0);
    let liner_debris_sump = centered_cube(
        "closed_sample_label_station_liner_debris_sump",
        WASTE_X - 46.0,
        38.0,
        12.0,
    )
    .translate(0.0, -WASTE_Y / 2.0 + 36.0, WASTE_Z / 2.0 - 5.0);
    let hub = centered_cylinder(
        "closed_sample_label_station_liner_takeup_hub",
        19.0,
        WASTE_Z + 28.0,
        48,
    )
    .translate(-30.0, 26.0, 14.0);
    let roller_pair = liner_roller_pair();
    let guide_comb = liner_guide_comb();
    let full_spool_gauge = centered_cylinder(
        "closed_sample_label_station_full_liner_spool_gauge",
        TAKEUP_SPOOL_D / 2.0 + 12.0,
        8.0,
        72,
    )
    .translate(-30.0, 26.0, WASTE_Z / 2.0 + 4.0);

    let part =
        tray - takeup_well - liner_debris_sump + hub + roller_pair + guide_comb + full_spool_gauge;
    place_on_deck(part, WASTE_CENTER, WASTE_Z)
}

fn liner_roller_pair() -> Part {
    let drive = centered_cylinder(
        "closed_sample_label_station_liner_drive_roller",
        LINER_ROLLER_D / 2.0,
        LABEL_STOCK_W + 46.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(88.0, -42.0, WASTE_Z / 2.0 + 18.0);
    let idler = centered_cylinder(
        "closed_sample_label_station_liner_idler_roller",
        LINER_ROLLER_D / 2.0,
        LABEL_STOCK_W + 46.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(88.0, 2.0, WASTE_Z / 2.0 + 18.0);
    let dancer_arm = centered_cube(
        "closed_sample_label_station_liner_tension_dancer_arm",
        118.0,
        12.0,
        14.0,
    )
    .rotate(0.0, 0.0, -14.0)
    .translate(52.0, -18.0, WASTE_Z / 2.0 + 36.0);

    drive + idler + dancer_arm
}

fn liner_guide_comb() -> Part {
    let body = centered_cube(
        "closed_sample_label_station_liner_guide_comb_body",
        108.0,
        28.0,
        24.0,
    )
    .translate(76.0, -WASTE_Y / 2.0 + 18.0, WASTE_Z / 2.0 + 12.0);
    let mut slots = Part::empty("closed_sample_label_station_liner_guide_comb_slots");
    for index in 0..LINER_GUIDE_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_sample_label_station_liner_guide_slot_{index}"),
                8.0,
                30.0,
                25.0,
            )
            .translate(
                76.0 + centered_index(index, LINER_GUIDE_SLOTS, 18.0),
                -WASTE_Y / 2.0 + 18.0,
                WASTE_Z / 2.0 + 12.0,
            );
    }
    body - slots
}

fn robot_gripper_datums() -> Part {
    let mut pads = Part::empty("closed_sample_label_station_robot_gripper_datum_pads");
    for (index, (x, y)) in robot_gripper_pad_positions().into_iter().enumerate() {
        let pad = centered_cube(
            format!("closed_sample_label_station_robot_gripper_pad_{index}"),
            44.0,
            28.0,
            6.0,
        )
        .translate(x, y, DECK_Z + 3.0);
        let cross_x = centered_cube(
            format!("closed_sample_label_station_robot_gripper_pad_{index}_cross_x"),
            34.0,
            2.0,
            7.0,
        )
        .translate(x, y, DECK_Z + 3.5);
        let cross_y = centered_cube(
            format!("closed_sample_label_station_robot_gripper_pad_{index}_cross_y"),
            2.0,
            20.0,
            7.0,
        )
        .translate(x, y, DECK_Z + 3.5);
        pads = pads + pad + cross_x + cross_y;
    }

    pads + datum_pins() + fiducial_lands()
}

fn datum_pins() -> Part {
    let mut pins = Part::empty("closed_sample_label_station_robot_datum_pins");
    for (index, (x, y)) in datum_pin_positions().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_sample_label_station_robot_datum_pin_boss_{index}"),
            8.0,
            10.0,
            28,
        )
        .translate(x, y, DECK_Z + 5.0);
        let pin = centered_cylinder(
            format!("closed_sample_label_station_robot_datum_pin_clearance_{index}"),
            2.0,
            11.0,
            24,
        )
        .translate(x, y, DECK_Z + 5.0);
        pins = pins + (boss - pin);
    }
    pins
}

fn fiducial_lands() -> Part {
    let mut lands = Part::empty("closed_sample_label_station_optical_fiducial_lands");
    for index in 0..FIDUCIAL_COUNT {
        let (x, y) = fiducial_position(index);
        let disk = centered_cylinder(
            format!("closed_sample_label_station_optical_fiducial_disk_{index}"),
            8.0,
            2.0,
            32,
        )
        .translate(x, y, DECK_Z + 1.0);
        let dot = centered_cylinder(
            format!("closed_sample_label_station_optical_fiducial_dot_{index}"),
            2.4,
            2.4,
            24,
        )
        .translate(x, y, DECK_Z + 1.2);
        lands = lands + (disk - dot);
    }
    lands
}

fn sterile_cover_service_keepouts() -> Part {
    let cover = closed_sterile_cover();
    let keepouts = service_keepout_gauges();
    cover + keepouts
}

fn closed_sterile_cover() -> Part {
    let left = centered_cube(
        "closed_sample_label_station_sterile_cover_left_wall",
        COVER_WALL_T,
        COVER_Y,
        COVER_Z,
    )
    .translate(
        COVER_CENTER.0 - COVER_X / 2.0 + COVER_WALL_T / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z / 2.0,
    );
    let right = centered_cube(
        "closed_sample_label_station_sterile_cover_right_wall",
        COVER_WALL_T,
        COVER_Y,
        COVER_Z,
    )
    .translate(
        COVER_CENTER.0 + COVER_X / 2.0 - COVER_WALL_T / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_sample_label_station_sterile_cover_rear_wall",
        COVER_X,
        COVER_WALL_T,
        COVER_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 + COVER_Y / 2.0 - COVER_WALL_T / 2.0,
        DECK_Z + COVER_Z / 2.0,
    );
    let front = centered_cube(
        "closed_sample_label_station_sterile_cover_front_transfer_wall",
        COVER_X,
        COVER_WALL_T,
        COVER_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 - COVER_Y / 2.0 + COVER_WALL_T / 2.0,
        DECK_Z + COVER_Z / 2.0,
    );
    let roof = centered_cube(
        "closed_sample_label_station_sterile_cover_low_shear_roof",
        COVER_X,
        COVER_Y,
        COVER_ROOF_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z + COVER_ROOF_Z / 2.0,
    );

    let transfer_slot = centered_cube(
        "closed_sample_label_station_sterile_cover_front_robot_transfer_slot",
        TRANSFER_SLOT_X,
        COVER_WALL_T + 4.0,
        TRANSFER_SLOT_Z,
    )
    .translate(
        CARRIER_CENTER.0,
        COVER_CENTER.1 - COVER_Y / 2.0 + COVER_WALL_T / 2.0,
        DECK_Z + 84.0,
    );
    let camera_service_window = centered_cube(
        "closed_sample_label_station_sterile_cover_camera_service_window",
        210.0,
        COVER_WALL_T + 4.0,
        72.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        COVER_CENTER.1 + COVER_Y / 2.0 - COVER_WALL_T / 2.0,
        DECK_Z + 172.0,
    );

    left + right + rear + roof + (front - transfer_slot) - camera_service_window
}

fn service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "closed_sample_label_station_front_robot_approach_keepout",
        DECK_X - 260.0,
        FRONT_ROBOT_CLEARANCE_Y,
        8.0,
    )
    .translate(
        -36.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y / 2.0 + 8.0,
        DECK_Z + 4.0,
    );
    let rear_service = centered_cube(
        "closed_sample_label_station_rear_scanner_service_keepout",
        DECK_X - 250.0,
        REAR_SERVICE_CLEARANCE_Y,
        8.0,
    )
    .translate(
        40.0,
        DECK_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y / 2.0 - 8.0,
        DECK_Z + 4.0,
    );
    let right_spool_service = centered_cube(
        "closed_sample_label_station_right_spool_service_keepout",
        RIGHT_SPOOL_SERVICE_CLEARANCE_X,
        470.0,
        8.0,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SPOOL_SERVICE_CLEARANCE_X / 2.0 - 14.0,
        -82.0,
        DECK_Z + 4.0,
    );
    let camera_z = centered_cube(
        "closed_sample_label_station_camera_z_service_envelope",
        BRIDGE_SPAN_X - 120.0,
        34.0,
        12.0,
    )
    .translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, CAMERA_SERVICE_CLEARANCE_Z);

    front_robot + rear_service + right_spool_service + camera_z
}

fn place_on_deck(part: Part, center: (f64, f64), height: f64) -> Part {
    part.translate(center.0, center.1, DECK_Z + height / 2.0)
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 5] {
    [
        (
            "sterile_label_cassette_dock",
            CASSETTE_CENTER,
            CASSETTE_X,
            CASSETTE_Y,
        ),
        ("print_apply_platen", PLATEN_CENTER, PLATEN_X, PLATEN_Y),
        (
            "sample_tube_chip_carrier_nests",
            CARRIER_CENTER,
            CARRIER_X,
            CARRIER_Y,
        ),
        ("reject_quarantine_lane", REJECT_CENTER, REJECT_X, REJECT_Y),
        ("label_liner_waste_spool", WASTE_CENTER, WASTE_X, WASTE_Y),
    ]
}

fn deck_mount_points() -> [(f64, f64); 10] {
    [
        (-560.0, -350.0),
        (-280.0, -350.0),
        (0.0, -350.0),
        (280.0, -350.0),
        (560.0, -350.0),
        (-560.0, 350.0),
        (-280.0, 350.0),
        (0.0, 350.0),
        (280.0, 350.0),
        (560.0, 350.0),
    ]
}

fn tube_well_position(index: usize) -> (f64, f64) {
    let col = index % TUBE_WELL_COLS;
    let row = index / TUBE_WELL_COLS;
    (
        TUBE_WELL_ORIGIN.0 + centered_index(col, TUBE_WELL_COLS, TUBE_WELL_PITCH_X),
        TUBE_WELL_ORIGIN.1
            + centered_index(row, TUBE_WELL_COUNT / TUBE_WELL_COLS, TUBE_WELL_PITCH_Y),
    )
}

fn chip_nest_position(index: usize) -> (f64, f64) {
    let col = index % CHIP_NEST_COLS;
    let row = index / CHIP_NEST_COLS;
    (
        CHIP_NEST_ORIGIN.0 + centered_index(col, CHIP_NEST_COLS, CHIP_PITCH_X),
        CHIP_NEST_ORIGIN.1 + centered_index(row, CHIP_NEST_COUNT / CHIP_NEST_COLS, CHIP_PITCH_Y),
    )
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, STATUS_SLOT_PITCH_X)
}

fn status_slot_y(slot: usize) -> f64 {
    centered_index(slot, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y)
}

fn robot_gripper_pad_positions() -> [(f64, f64); ROBOT_GRIPPER_DATUMS] {
    [
        (-540.0, 312.0),
        (-420.0, 312.0),
        (-300.0, 312.0),
        (-180.0, 312.0),
        (-540.0, -330.0),
        (-360.0, -330.0),
        (-180.0, -330.0),
        (115.0, -320.0),
        (315.0, -330.0),
        (500.0, -330.0),
        (540.0, 292.0),
        (340.0, 292.0),
    ]
}

fn datum_pin_positions() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-548.0, 252.0),
        (-282.0, 252.0),
        (-528.0, -258.0),
        (-32.0, -258.0),
        (265.0, 262.0),
        (548.0, -248.0),
    ]
}

fn fiducial_position(index: usize) -> (f64, f64) {
    let positions = [
        (-560.0, 346.0),
        (-284.0, 346.0),
        (22.0, 346.0),
        (302.0, 346.0),
        (548.0, 346.0),
        (-560.0, -346.0),
        (-252.0, -346.0),
        (34.0, -346.0),
        (314.0, -346.0),
        (548.0, -346.0),
    ];
    positions[index]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

fn quarantine_reject_gap() -> f64 {
    status_lane_x(STATUS_LANES - 1) - status_lane_x(STATUS_LANES - 2) - STATUS_SLOT_X
}

fn label_stock_path_is_monotonic() -> bool {
    CASSETTE_CENTER.0 < PLATEN_CENTER.0 && PLATEN_CENTER.0 < WASTE_CENTER.0
}

fn assert_layout() {
    for (name, center, x, y) in module_specs() {
        assert!(
            fits_on_deck(center, x, y, 18.0),
            "{name} exceeds cleanable deck envelope"
        );
    }

    let cassette = rect(CASSETTE_CENTER, CASSETTE_X, CASSETTE_Y);
    let platen = rect(PLATEN_CENTER, PLATEN_X, PLATEN_Y);
    let carriers = rect(CARRIER_CENTER, CARRIER_X, CARRIER_Y);
    let reject = rect(REJECT_CENTER, REJECT_X, REJECT_Y);
    let waste = rect(WASTE_CENTER, WASTE_X, WASTE_Y);

    assert!(!rects_overlap(cassette, platen));
    assert!(!rects_overlap(cassette, carriers));
    assert!(!rects_overlap(platen, carriers));
    assert!(!rects_overlap(platen, reject));
    assert!(!rects_overlap(carriers, waste));
    assert!(!rects_overlap(reject, waste));
    assert!(label_stock_path_is_monotonic());
    assert!(bridge_clearance_above_deck() >= 180.0);
    assert!(quarantine_reject_gap() >= QUARANTINE_REJECT_GAP_MIN);
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0 - x / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - y / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + y / 2.0 <= DECK_Y / 2.0 - margin
}

#[derive(Clone, Copy)]
struct Rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

fn rect(center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect {
        x0: center.0 - x / 2.0,
        x1: center.0 + x / 2.0,
        y0: center.1 - y / 2.0,
        y1: center.1 + y / 2.0,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x0 < b.x1 && a.x1 > b.x0 && a.y0 < b.y1 && a.y1 > b.y0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped_to_station() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sample_label_print_apply_verify_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_represents_required_closed_label_controls() {
        assert_eq!(REQUIRED_CONTROLS.len(), 8);
        assert!(REQUIRED_CONTROLS.contains(&"sterile_label_cassette_dock"));
        assert!(REQUIRED_CONTROLS.contains(&"thermal_print_apply_platen"));
        assert!(REQUIRED_CONTROLS.contains(&"sample_tube_chip_carrier_nests"));
        assert!(REQUIRED_CONTROLS.contains(&"barcode_rfid_verifier_camera_bridge"));
        assert!(REQUIRED_CONTROLS.contains(&"reject_quarantine_lane"));
        assert!(REQUIRED_CONTROLS.contains(&"label_liner_waste_spool"));
        assert!(REQUIRED_CONTROLS.contains(&"robot_gripper_datums"));
        assert!(REQUIRED_CONTROLS.contains(&"closed_sterile_cover"));
    }

    #[test]
    fn major_modules_fit_without_deck_collisions() {
        assert_layout();
    }

    #[test]
    fn carrier_nests_cover_tube_and_tissue_chip_samples() {
        assert_eq!(TUBE_WELL_COUNT, 12);
        assert_eq!(CHIP_NEST_COUNT, 4);
        assert!(CHIP_POCKET_X > REVC_CHIP_LENGTH);
        assert!(CHIP_POCKET_Y > REVC_CHIP_WIDTH);
        assert!(REVC_TOTAL_HEIGHT < CARRIER_Z);

        for index in 0..TUBE_WELL_COUNT {
            let (x, y) = tube_well_position(index);
            assert!(x.abs() + TUBE_WELL_D / 2.0 + 12.0 < CARRIER_X / 2.0);
            assert!(y.abs() + TUBE_WELL_D / 2.0 + 12.0 < CARRIER_Y / 2.0);
        }
        for index in 0..CHIP_NEST_COUNT {
            let (x, y) = chip_nest_position(index);
            assert!(x.abs() + CHIP_POCKET_X / 2.0 + 12.0 < CARRIER_X / 2.0);
            assert!(y.abs() + CHIP_POCKET_Y / 2.0 + 12.0 < CARRIER_Y / 2.0);
        }
    }

    #[test]
    fn print_apply_and_liner_path_are_mechanically_ordered() {
        assert!(label_stock_path_is_monotonic());
        assert!(LABEL_STOCK_W >= 40.0);
        assert_eq!(LABEL_PRESENT_WINDOWS, 3);
        assert_eq!(LABEL_APPLY_FORCE_PADS, 4);
        assert!(TAKEUP_SPOOL_D > LABEL_STOCK_W * 2.0);
        assert!(LINER_GUIDE_SLOTS >= 5);
    }

    #[test]
    fn verifier_bridge_and_status_paths_are_controlled() {
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LED_SEGMENTS, 8);
        assert!(bridge_clearance_above_deck() >= 180.0);
        assert!(RFID_ANTENNA_X >= 140.0);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert!(quarantine_reject_gap() >= QUARANTINE_REJECT_GAP_MIN);
        assert!(REJECT_HIGH_WALL_Z >= 70.0);
    }

    #[test]
    fn robot_datums_and_service_keepouts_are_explicit() {
        assert_eq!(robot_gripper_pad_positions().len(), ROBOT_GRIPPER_DATUMS);
        assert_eq!(datum_pin_positions().len(), DATUM_PIN_COUNT);
        assert_eq!(FIDUCIAL_COUNT, 10);
        assert!(FRONT_ROBOT_CLEARANCE_Y >= 100.0);
        assert!(REAR_SERVICE_CLEARANCE_Y >= 80.0);
        assert!(RIGHT_SPOOL_SERVICE_CLEARANCE_X >= 120.0);
        assert!(CAMERA_SERVICE_CLEARANCE_Z > DECK_Z + BRIDGE_UNDERSIDE_Z);
        assert!(COVER_Z > BRIDGE_UNDERSIDE_Z);
    }

    #[test]
    fn cassette_dock_has_sterile_datum_controls() {
        assert_eq!(STERILE_CASSETTE_DATUM_PINS, 4);
        assert!(LABEL_CASSETTE_X < CASSETTE_X - 80.0);
        assert!(LABEL_CASSETTE_Y < CASSETTE_Y - 80.0);
        assert!(CASSETTE_CENTER.0 + CASSETTE_X / 2.0 < PLATEN_CENTER.0 - PLATEN_X / 2.0);
    }
}
