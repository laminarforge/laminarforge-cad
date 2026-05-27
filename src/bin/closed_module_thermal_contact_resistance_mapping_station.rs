use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module thermal-contact-resistance mapping station.
//
// Intent:
// - Map thermal contact resistance across sealed culture modules/cassettes
//   without breaching the sterile boundary.
// - Provide a controlled datum nest, interchangeable pad/gap coupons, spring
//   preload force-gauge pocket, temperature probe pockets, and a reference
//   heat-spreader block placeholder.
// - Keep release, hold, reject, traceability, imaging evidence, robot access,
//   and service keepout geometry explicit for later integration.
//
// Stable STL outputs produced by this worker:
const OUTPUTS: [&str; 12] = [
    "output/closed_module_thermal_contact_resistance_mapping_station_base_deck.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_cassette_module_datum_nest.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_thermal_pad_gap_coupon_holders.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_spring_preload_force_gauge_pocket.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_temperature_probe_pockets.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_heat_spreader_reference_block_placeholder.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_barcode_certificate_lands.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_release_hold_reject_lanes.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_evidence_camera_bridge.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_robot_service_keepouts.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_calibration_coupon_retainer_pallet.stl",
    "output/closed_module_thermal_contact_resistance_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 9] = [
    "cassette_module_datum_nest",
    "thermal_pad_gap_coupon_holders",
    "spring_preload_force_gauge_pocket",
    "temperature_probe_pockets",
    "heat_spreader_reference_block_placeholder",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.8;

const MODULE_COLS: usize = 3;
const MODULE_ROWS: usize = 2;
const MODULE_GUTTER: f64 = 8.0;
const MODULE_MARGIN_X: f64 = 72.0;
const MODULE_MARGIN_Y: f64 = 61.0;
const MODULE_X: f64 = MODULE_COLS as f64 * REVC_CHIP_LENGTH
    + (MODULE_COLS as f64 - 1.0) * MODULE_GUTTER
    + MODULE_MARGIN_X * 2.0;
const MODULE_Y: f64 = MODULE_ROWS as f64 * REVC_CHIP_WIDTH
    + (MODULE_ROWS as f64 - 1.0) * MODULE_GUTTER
    + MODULE_MARGIN_Y * 2.0;
const MODULE_KEEP_OUT_Z: f64 = 74.0;

const NEST_X: f64 = MODULE_X + 80.0;
const NEST_Y: f64 = MODULE_Y + 80.0;
const NEST_Z: f64 = 34.0;
const NEST_CENTER: (f64, f64) = (-240.0, 105.0);
const NEST_RAIL_W: f64 = 24.0;
const DATUM_PIN_D: f64 = 8.0;
const DATUM_PIN_BOSS_D: f64 = 22.0;
const LATCH_PAD_COUNT: usize = 4;

const COUPON_CENTER: (f64, f64) = (320.0, 220.0);
const COUPON_HOLDER_X: f64 = 360.0;
const COUPON_HOLDER_Y: f64 = 180.0;
const COUPON_HOLDER_Z: f64 = 28.0;
const COUPON_COLS: usize = 3;
const COUPON_ROWS: usize = 2;
const COUPON_COUNT: usize = COUPON_COLS * COUPON_ROWS;
const PAD_COUPON_X: f64 = 74.0;
const PAD_COUPON_Y: f64 = 44.0;
const PAD_COUPON_CLEARANCE_Z: f64 = 8.0;
const COUPON_PITCH_X: f64 = 104.0;
const COUPON_PITCH_Y: f64 = 72.0;
const GAP_SHIM_SLOTS_PER_COUPON: usize = 3;

const FORCE_CENTER: (f64, f64) = (330.0, 15.0);
const FORCE_BASE_X: f64 = 330.0;
const FORCE_BASE_Y: f64 = 170.0;
const FORCE_BASE_Z: f64 = 42.0;
const FORCE_GAUGE_ENV_X: f64 = 98.0;
const FORCE_GAUGE_ENV_Y: f64 = 54.0;
const FORCE_GAUGE_ENV_Z: f64 = 230.0;
const PRELOAD_PLUNGER_D: f64 = 30.0;
const PRELOAD_SPRING_POSTS: usize = 4;
const PRELOAD_FORCE_RANGE_N: f64 = 50.0;

const PROBE_CENTER: (f64, f64) = (-320.0, -245.0);
const PROBE_BLOCK_X: f64 = 410.0;
const PROBE_BLOCK_Y: f64 = 170.0;
const PROBE_BLOCK_Z: f64 = 30.0;
const PROBE_POCKETS: usize = 8;
const PROBE_PITCH_X: f64 = 44.0;
const PROBE_SOCKET_D: f64 = 5.2;
const PROBE_CABLE_GROOVE_Y: f64 = 16.0;

const HEAT_BLOCK_CENTER: (f64, f64) = (70.0, -245.0);
const HEAT_BLOCK_X: f64 = 280.0;
const HEAT_BLOCK_Y: f64 = 150.0;
const HEAT_BLOCK_Z: f64 = 44.0;
const HEAT_SPREAD_SURFACE_X: f64 = 220.0;
const HEAT_SPREAD_SURFACE_Y: f64 = 94.0;
const HEAT_SPREAD_REFERENCE_POINTS: usize = 12;
const HEAT_SENSOR_WELLS: usize = 4;

const TRACE_CENTER: (f64, f64) = (-90.0, 345.0);
const TRACE_X: f64 = 560.0;
const TRACE_Y: f64 = 62.0;
const TRACE_Z: f64 = 8.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;
const BARCODE_LAND_X: f64 = 66.0;
const BARCODE_LAND_Y: f64 = 18.0;

const STATUS_CENTER: (f64, f64) = (420.0, -245.0);
const STATUS_X: f64 = 300.0;
const STATUS_Y: f64 = 190.0;
const STATUS_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_PITCH_X: f64 = 86.0;
const STATUS_SLOT_PITCH_Y: f64 = 38.0;
const STATUS_SLOT_X: f64 = 62.0;
const STATUS_SLOT_Y: f64 = 28.0;

const CAMERA_SPAN_X: f64 = NEST_X + 92.0;
const CAMERA_SPAN_Y: f64 = NEST_Y + 72.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 36.0;
const CAMERA_UNDERSIDE_Z: f64 = 168.0;
const CAMERA_BEAM_Z: f64 = 26.0;
const CAMERA_COUNT: usize = 3;
const LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_Z: f64 = 10.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 110.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 92.0;
const LEFT_MODULE_LOAD_KEEP_OUT_X: f64 = 96.0;
const RIGHT_THERMAL_SERVICE_KEEP_OUT_X: f64 = 112.0;
const ROBOT_CLEARANCE_Z: f64 = 142.0;
const SERVICE_CLEARANCE_Z: f64 = 260.0;

const RETAIN_CENTER: (f64, f64) = (-520.0, 0.0);
const RETAIN_X: f64 = 82.0;
const RETAIN_Y: f64 = 520.0;
const RETAIN_Z: f64 = 34.0;
const RETAIN_SLOTS: usize = 10;
const RETAIN_SLOT_Y_PITCH: f64 = 46.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = station_base_deck();
    export(OUTPUTS[0], &deck);

    let nest = cassette_module_datum_nest();
    export(OUTPUTS[1], &nest);

    let coupons = thermal_pad_gap_coupon_holders();
    export(OUTPUTS[2], &coupons);

    let preload = spring_preload_force_gauge_pocket();
    export(OUTPUTS[3], &preload);

    let probes = temperature_probe_pockets();
    export(OUTPUTS[4], &probes);

    let heat_block = heat_spreader_reference_block_placeholder();
    export(OUTPUTS[5], &heat_block);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[7], &status);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let retain = calibration_coupon_retainer_pallet();
    export(OUTPUTS[10], &retain);

    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, deck_insert_z(NEST_Z))
        + coupons.translate(
            COUPON_CENTER.0,
            COUPON_CENTER.1,
            deck_insert_z(COUPON_HOLDER_Z),
        )
        + preload.translate(FORCE_CENTER.0, FORCE_CENTER.1, deck_insert_z(FORCE_BASE_Z))
        + probes.translate(PROBE_CENTER.0, PROBE_CENTER.1, deck_insert_z(PROBE_BLOCK_Z))
        + heat_block.translate(
            HEAT_BLOCK_CENTER.0,
            HEAT_BLOCK_CENTER.1,
            deck_insert_z(HEAT_BLOCK_Z),
        )
        + traceability.translate(TRACE_CENTER.0, TRACE_CENTER.1, deck_insert_z(TRACE_Z))
        + status.translate(STATUS_CENTER.0, STATUS_CENTER.1, deck_insert_z(STATUS_Z))
        + bridge.translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z / 2.0)
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_Z / 2.0)
        + retain.translate(RETAIN_CENTER.0, RETAIN_CENTER.1, deck_insert_z(RETAIN_Z));
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed-module thermal-contact-resistance mapping station:");
    println!(
        "  Deck:                         {DECK_X:.0}mm x {DECK_Y:.0}mm leak-managed mapping deck"
    );
    println!(
        "  Module datum nest:            {NEST_X:.0}mm x {NEST_Y:.0}mm for {MODULE_COLS}x{MODULE_ROWS} sealed module cassette"
    );
    println!(
        "  Pad/gap coupons:              {COUPON_COUNT} coupon pockets with {GAP_SHIM_SLOTS_PER_COUPON} gap-shim slots each"
    );
    println!(
        "  Preload force gauge pocket:   {FORCE_GAUGE_ENV_X:.0}mm x {FORCE_GAUGE_ENV_Y:.0}mm x {FORCE_GAUGE_ENV_Z:.0}mm envelope, {PRELOAD_SPRING_POSTS} spring posts, {PRELOAD_FORCE_RANGE_N:.0}N range placeholder"
    );
    println!(
        "  Temperature probe pockets:    {PROBE_POCKETS} probes plus {HEAT_SENSOR_WELLS} heat-spreader wells"
    );
    println!(
        "  Traceability and disposition: {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, release/hold/reject lanes"
    );
    println!(
        "  Evidence/keepouts:            {CAMERA_COUNT} camera pods, {LIGHT_BAR_COUNT} light bars, robot and service keepouts"
    );
    println!(
        "  Required feature groups:      {}",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(component_z: f64) -> f64 {
    DECK_Z / 2.0 + component_z / 2.0
}

fn station_base_deck() -> Part {
    let deck = centered_cube(
        "thermal_contact_resistance_mapping_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let leak_basin = centered_cube(
        "thermal_contact_resistance_mapping_leak_basin_recess",
        DECK_X - 2.0 * (RIM_W + 48.0),
        DECK_Y - 2.0 * (RIM_W + 44.0),
        8.0,
    )
    .translate(0.0, -10.0, DECK_Z / 2.0 - 3.0);
    let front_drain_channel = centered_cube(
        "thermal_contact_resistance_mapping_front_drain_channel",
        520.0,
        24.0,
        DECK_Z + 2.0,
    )
    .translate(130.0, -DECK_Y / 2.0 + 50.0, 0.0);
    let drain_port = centered_cylinder(
        "thermal_contact_resistance_mapping_deck_drain_port",
        8.0 / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 76.0, -DECK_Y / 2.0 + 18.0, -1.0);

    deck - leak_basin - front_drain_channel - drain_port
        + deck_perimeter_rims()
        + deck_mount_bosses_and_holes()
        + station_recess_labels()
}

fn deck_perimeter_rims() -> Part {
    let left = centered_cube(
        "thermal_contact_resistance_mapping_left_spill_rim",
        RIM_W,
        DECK_Y - 60.0,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - 30.0), 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "thermal_contact_resistance_mapping_right_spill_rim",
        RIM_W,
        DECK_Y - 60.0,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - 30.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let rear = centered_cube(
        "thermal_contact_resistance_mapping_rear_spill_rim",
        DECK_X - 78.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 30.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let front_low = centered_cube(
        "thermal_contact_resistance_mapping_front_low_drain_lip",
        DECK_X - 220.0,
        10.0,
        16.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 32.0, DECK_Z / 2.0 + 8.0);

    left + right + rear + front_low
}

fn deck_mount_bosses_and_holes() -> Part {
    let mut parts = Part::empty("thermal_contact_resistance_mapping_mount_features");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("thermal_contact_resistance_mapping_mount_boss_{i}"),
            14.0,
            8.0,
            32,
        )
        .translate(*x, *y, -DECK_Z / 2.0 + 5.0);
        let hole = centered_cylinder(
            format!("thermal_contact_resistance_mapping_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 16.0,
            24,
        )
        .translate(*x, *y, 0.0);
        parts = parts + (boss - hole);
    }
    parts
}

fn station_recess_labels() -> Part {
    let mut labels = Part::empty("thermal_contact_resistance_mapping_station_recess_labels");
    for (i, rect) in [
        nest_rect(),
        coupon_rect(),
        force_rect(),
        probe_rect(),
        heat_block_rect(),
        status_rect(),
    ]
    .iter()
    .enumerate()
    {
        labels = labels
            + centered_cube(
                format!("thermal_contact_resistance_mapping_recess_outline_{i}"),
                rect.w + 16.0,
                4.0,
                3.0,
            )
            .translate(rect.x, rect.y - rect.h / 2.0 - 8.0, DECK_Z / 2.0 + 2.0)
            + centered_cube(
                format!("thermal_contact_resistance_mapping_recess_outline_cross_{i}"),
                4.0,
                rect.h + 16.0,
                3.0,
            )
            .translate(rect.x - rect.w / 2.0 - 8.0, rect.y, DECK_Z / 2.0 + 2.0);
    }
    labels
}

fn cassette_module_datum_nest() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_module_nest_base",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let module_pocket = centered_cube(
        "thermal_contact_resistance_mapping_module_nest_clearance_pocket",
        MODULE_X + 1.6,
        MODULE_Y + 1.6,
        NEST_Z + 4.0,
    )
    .translate(0.0, 0.0, 8.0);
    let left_rail = centered_cube(
        "thermal_contact_resistance_mapping_left_hard_datum_rail",
        NEST_RAIL_W,
        MODULE_Y + 54.0,
        NEST_Z + 10.0,
    )
    .translate(-(MODULE_X / 2.0 + NEST_RAIL_W / 2.0 + 12.0), 0.0, 5.0);
    let rear_stop = centered_cube(
        "thermal_contact_resistance_mapping_rear_hard_stop",
        MODULE_X + 64.0,
        24.0,
        NEST_Z + 12.0,
    )
    .translate(0.0, MODULE_Y / 2.0 + 20.0, 6.0);
    let right_spring_datum = centered_cube(
        "thermal_contact_resistance_mapping_right_compliant_datum_strip",
        14.0,
        MODULE_Y + 20.0,
        NEST_Z + 4.0,
    )
    .translate(MODULE_X / 2.0 + 20.0, -8.0, 2.0);
    let front_latch_bar = centered_cube(
        "thermal_contact_resistance_mapping_front_latch_bar",
        MODULE_X + 50.0,
        18.0,
        NEST_Z + 8.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 + 18.0), 4.0);

    base - module_pocket
        + left_rail
        + rear_stop
        + right_spring_datum
        + front_latch_bar
        + module_datum_pins()
        + module_latch_pads()
        + module_contact_map_grid()
}

fn module_datum_pins() -> Part {
    let mut pins = Part::empty("thermal_contact_resistance_mapping_module_datum_pins");
    for (i, (x, y)) in module_datum_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("thermal_contact_resistance_mapping_datum_pin_boss_{i}"),
            DATUM_PIN_BOSS_D / 2.0,
            10.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        let pin = centered_cylinder(
            format!("thermal_contact_resistance_mapping_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            18.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 16.0);
        pins = pins + boss + pin;
    }
    pins
}

fn module_latch_pads() -> Part {
    let mut pads = Part::empty("thermal_contact_resistance_mapping_module_latch_pads");
    for i in 0..LATCH_PAD_COUNT {
        let x = -((LATCH_PAD_COUNT as f64 - 1.0) * 92.0) / 2.0 + i as f64 * 92.0;
        let pad = centered_cube(
            format!("thermal_contact_resistance_mapping_latch_pad_{i}"),
            54.0,
            28.0,
            12.0,
        )
        .translate(x, -(MODULE_Y / 2.0 + 54.0), NEST_Z / 2.0 + 8.0);
        let screw = centered_cylinder(
            format!("thermal_contact_resistance_mapping_latch_pad_screw_clearance_{i}"),
            3.4 / 2.0,
            16.0,
            20,
        )
        .translate(x, -(MODULE_Y / 2.0 + 54.0), NEST_Z / 2.0 + 8.0);
        pads = pads + (pad - screw);
    }
    pads
}

fn module_contact_map_grid() -> Part {
    let mut grid = Part::empty("thermal_contact_resistance_mapping_contact_map_grid");
    for col in 0..MODULE_COLS {
        for row in 0..MODULE_ROWS {
            let x = -((MODULE_COLS as f64 - 1.0) * (REVC_CHIP_LENGTH + MODULE_GUTTER)) / 2.0
                + col as f64 * (REVC_CHIP_LENGTH + MODULE_GUTTER);
            let y = -((MODULE_ROWS as f64 - 1.0) * (REVC_CHIP_WIDTH + MODULE_GUTTER)) / 2.0
                + row as f64 * (REVC_CHIP_WIDTH + MODULE_GUTTER);
            grid = grid
                + centered_cube(
                    format!("thermal_contact_resistance_mapping_cell_window_outline_{col}_{row}"),
                    REVC_CHIP_LENGTH - 20.0,
                    4.0,
                    4.0,
                )
                .translate(x, y - REVC_CHIP_WIDTH / 2.0 + 12.0, NEST_Z / 2.0 + 4.0)
                + centered_cube(
                    format!("thermal_contact_resistance_mapping_cell_window_crosshair_{col}_{row}"),
                    4.0,
                    REVC_CHIP_WIDTH - 20.0,
                    4.0,
                )
                .translate(x, y, NEST_Z / 2.0 + 4.0);
        }
    }
    grid
}

fn thermal_pad_gap_coupon_holders() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_coupon_holder_base",
        COUPON_HOLDER_X,
        COUPON_HOLDER_Y,
        COUPON_HOLDER_Z,
    );
    let mut cuts = Part::empty("thermal_contact_resistance_mapping_coupon_holder_pocket_cuts");
    let mut ledges = Part::empty("thermal_contact_resistance_mapping_coupon_holder_ledge_features");

    for index in 0..COUPON_COUNT {
        let (x, y) = coupon_position(index);
        cuts = cuts
            + centered_cube(
                format!("thermal_contact_resistance_mapping_pad_coupon_pocket_{index}"),
                PAD_COUPON_X,
                PAD_COUPON_Y,
                PAD_COUPON_CLEARANCE_Z,
            )
            .translate(
                x,
                y,
                COUPON_HOLDER_Z / 2.0 - PAD_COUPON_CLEARANCE_Z / 2.0 + 1.0,
            );

        for slot in 0..GAP_SHIM_SLOTS_PER_COUPON {
            let sx = x - 29.0 + slot as f64 * 29.0;
            cuts = cuts
                + centered_cube(
                    format!(
                        "thermal_contact_resistance_mapping_gap_coupon_shim_slot_{index}_{slot}"
                    ),
                    6.0,
                    PAD_COUPON_Y + 24.0,
                    COUPON_HOLDER_Z + 4.0,
                )
                .translate(sx, y, 3.0);
        }

        ledges = ledges
            + centered_cube(
                format!("thermal_contact_resistance_mapping_coupon_left_ledge_{index}"),
                7.0,
                PAD_COUPON_Y + 14.0,
                8.0,
            )
            .translate(x - PAD_COUPON_X / 2.0 - 8.0, y, COUPON_HOLDER_Z / 2.0 + 4.0)
            + centered_cube(
                format!("thermal_contact_resistance_mapping_coupon_right_ledge_{index}"),
                7.0,
                PAD_COUPON_Y + 14.0,
                8.0,
            )
            .translate(x + PAD_COUPON_X / 2.0 + 8.0, y, COUPON_HOLDER_Z / 2.0 + 4.0)
            + centered_cube(
                format!("thermal_contact_resistance_mapping_coupon_front_label_land_{index}"),
                PAD_COUPON_X,
                8.0,
                5.0,
            )
            .translate(
                x,
                y - PAD_COUPON_Y / 2.0 - 15.0,
                COUPON_HOLDER_Z / 2.0 + 3.0,
            );
    }

    base - cuts + ledges + coupon_holder_index_datums()
}

fn coupon_holder_index_datums() -> Part {
    let mut datums = Part::empty("thermal_contact_resistance_mapping_coupon_index_datums");
    for (i, x) in [-154.0, 154.0].iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("thermal_contact_resistance_mapping_coupon_index_pin_{i}"),
                5.0,
                12.0,
                24,
            )
            .translate(
                *x,
                COUPON_HOLDER_Y / 2.0 - 26.0,
                COUPON_HOLDER_Z / 2.0 + 6.0,
            );
    }
    datums
}

fn spring_preload_force_gauge_pocket() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_preload_force_base",
        FORCE_BASE_X,
        FORCE_BASE_Y,
        FORCE_BASE_Z,
    );
    let gauge_pocket = centered_cube(
        "thermal_contact_resistance_mapping_preload_force_gauge_pocket",
        FORCE_GAUGE_ENV_X + 8.0,
        FORCE_GAUGE_ENV_Y + 8.0,
        20.0,
    )
    .translate(-72.0, 0.0, FORCE_BASE_Z / 2.0 - 8.0);
    let gauge_envelope = centered_cube(
        "thermal_contact_resistance_mapping_preload_force_gauge_envelope",
        FORCE_GAUGE_ENV_X,
        FORCE_GAUGE_ENV_Y,
        FORCE_GAUGE_ENV_Z,
    )
    .translate(-72.0, 0.0, FORCE_BASE_Z / 2.0 + FORCE_GAUGE_ENV_Z / 2.0);
    let plunger = centered_cylinder(
        "thermal_contact_resistance_mapping_preload_plunger_contact",
        PRELOAD_PLUNGER_D / 2.0,
        74.0,
        40,
    )
    .translate(82.0, 0.0, FORCE_BASE_Z / 2.0 + 37.0);
    let plunger_foot = centered_cylinder(
        "thermal_contact_resistance_mapping_preload_contact_foot",
        44.0 / 2.0,
        8.0,
        40,
    )
    .translate(82.0, 0.0, FORCE_BASE_Z / 2.0 + 6.0);
    let crosshead = centered_cube(
        "thermal_contact_resistance_mapping_preload_crosshead",
        150.0,
        26.0,
        18.0,
    )
    .translate(82.0, 0.0, FORCE_BASE_Z / 2.0 + 94.0);

    base - gauge_pocket
        + gauge_envelope
        + plunger
        + plunger_foot
        + crosshead
        + preload_spring_posts()
        + force_scale_marks()
}

fn preload_spring_posts() -> Part {
    let mut posts = Part::empty("thermal_contact_resistance_mapping_preload_spring_posts");
    for (i, (x, y)) in [(22.0, -54.0), (142.0, -54.0), (22.0, 54.0), (142.0, 54.0)]
        .iter()
        .enumerate()
    {
        let post = centered_cylinder(
            format!("thermal_contact_resistance_mapping_preload_spring_post_{i}"),
            10.0,
            92.0,
            32,
        )
        .translate(*x, *y, FORCE_BASE_Z / 2.0 + 46.0);
        let bore = centered_cylinder(
            format!("thermal_contact_resistance_mapping_preload_spring_post_bore_{i}"),
            3.6 / 2.0,
            96.0,
            20,
        )
        .translate(*x, *y, FORCE_BASE_Z / 2.0 + 46.0);
        posts = posts + (post - bore);
    }
    posts
}

fn force_scale_marks() -> Part {
    let mut marks = Part::empty("thermal_contact_resistance_mapping_preload_force_scale_marks");
    for i in 0..6 {
        marks = marks
            + centered_cube(
                format!("thermal_contact_resistance_mapping_preload_force_tick_{i}"),
                36.0,
                3.0,
                4.0,
            )
            .translate(-148.0, -54.0 + i as f64 * 21.6, FORCE_BASE_Z / 2.0 + 4.0);
    }
    marks
}

fn temperature_probe_pockets() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_temperature_probe_block",
        PROBE_BLOCK_X,
        PROBE_BLOCK_Y,
        PROBE_BLOCK_Z,
    );
    let mut cuts = Part::empty("thermal_contact_resistance_mapping_temperature_probe_pocket_cuts");
    let mut collars = Part::empty("thermal_contact_resistance_mapping_temperature_probe_collars");

    for i in 0..PROBE_POCKETS {
        let x = probe_x(i);
        let y = if i % 2 == 0 { -28.0 } else { 28.0 };
        cuts = cuts
            + centered_cylinder(
                format!("thermal_contact_resistance_mapping_probe_socket_{i}"),
                PROBE_SOCKET_D / 2.0,
                PROBE_BLOCK_Z + 6.0,
                24,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("thermal_contact_resistance_mapping_probe_cable_groove_{i}"),
                8.0,
                PROBE_BLOCK_Y / 2.0 + 18.0,
                PROBE_CABLE_GROOVE_Y,
            )
            .translate(x, y.signum() * 44.0, PROBE_BLOCK_Z / 2.0 - 4.0);
        collars = collars
            + centered_cylinder(
                format!("thermal_contact_resistance_mapping_probe_stop_collar_{i}"),
                12.0 / 2.0,
                5.0,
                24,
            )
            .translate(x, y, PROBE_BLOCK_Z / 2.0 + 2.5);
    }

    base - cuts + collars + probe_wire_strain_relief()
}

fn probe_wire_strain_relief() -> Part {
    let rear_bar = centered_cube(
        "thermal_contact_resistance_mapping_probe_rear_strain_relief_bar",
        PROBE_BLOCK_X - 48.0,
        12.0,
        22.0,
    )
    .translate(0.0, PROBE_BLOCK_Y / 2.0 + 18.0, PROBE_BLOCK_Z / 2.0 + 11.0);
    let front_bar = centered_cube(
        "thermal_contact_resistance_mapping_probe_front_strain_relief_bar",
        PROBE_BLOCK_X - 48.0,
        12.0,
        22.0,
    )
    .translate(
        0.0,
        -(PROBE_BLOCK_Y / 2.0 + 18.0),
        PROBE_BLOCK_Z / 2.0 + 11.0,
    );

    rear_bar + front_bar
}

fn heat_spreader_reference_block_placeholder() -> Part {
    let block = centered_cube(
        "thermal_contact_resistance_mapping_heat_spreader_reference_block",
        HEAT_BLOCK_X,
        HEAT_BLOCK_Y,
        HEAT_BLOCK_Z,
    );
    let reference_surface = centered_cube(
        "thermal_contact_resistance_mapping_heat_spreader_lapped_contact_surface",
        HEAT_SPREAD_SURFACE_X,
        HEAT_SPREAD_SURFACE_Y,
        6.0,
    )
    .translate(0.0, 0.0, HEAT_BLOCK_Z / 2.0 + 3.0);
    let heater_cartridge_bore = centered_cylinder(
        "thermal_contact_resistance_mapping_heat_spreader_heater_bore",
        6.2 / 2.0,
        HEAT_BLOCK_X + 10.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -42.0, -8.0);

    block - heater_cartridge_bore
        + reference_surface
        + heat_spreader_reference_points()
        + heat_sensor_wells()
        + heat_block_mount_ears()
}

fn heat_spreader_reference_points() -> Part {
    let mut points = Part::empty("thermal_contact_resistance_mapping_heat_reference_points");
    for i in 0..HEAT_SPREAD_REFERENCE_POINTS {
        let col = i % 4;
        let row = i / 4;
        let x = -75.0 + col as f64 * 50.0;
        let y = -34.0 + row as f64 * 34.0;
        points = points
            + centered_cylinder(
                format!("thermal_contact_resistance_mapping_heat_reference_disc_{i}"),
                6.0,
                2.0,
                24,
            )
            .translate(x, y, HEAT_BLOCK_Z / 2.0 + 8.0)
            - centered_cylinder(
                format!("thermal_contact_resistance_mapping_heat_reference_center_dot_{i}"),
                1.2,
                3.0,
                16,
            )
            .translate(x, y, HEAT_BLOCK_Z / 2.0 + 8.0);
    }
    points
}

fn heat_sensor_wells() -> Part {
    let mut wells = Part::empty("thermal_contact_resistance_mapping_heat_sensor_wells");
    for (i, x) in [-90.0, -30.0, 30.0, 90.0].iter().enumerate() {
        let collar = centered_cylinder(
            format!("thermal_contact_resistance_mapping_heat_sensor_well_collar_{i}"),
            12.0 / 2.0,
            5.0,
            24,
        )
        .translate(*x, HEAT_BLOCK_Y / 2.0 - 20.0, HEAT_BLOCK_Z / 2.0 + 3.0);
        let pocket = centered_cylinder(
            format!("thermal_contact_resistance_mapping_heat_sensor_well_cut_{i}"),
            3.4 / 2.0,
            14.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, HEAT_BLOCK_Y / 2.0 - 12.0, HEAT_BLOCK_Z / 2.0 - 6.0);
        wells = wells + (collar - pocket);
    }
    wells
}

fn heat_block_mount_ears() -> Part {
    let mut ears = Part::empty("thermal_contact_resistance_mapping_heat_block_mount_ears");
    for (i, x) in [-120.0, 120.0].iter().enumerate() {
        let ear = centered_cube(
            format!("thermal_contact_resistance_mapping_heat_block_mount_ear_{i}"),
            40.0,
            30.0,
            16.0,
        )
        .translate(*x, -(HEAT_BLOCK_Y / 2.0 + 18.0), -HEAT_BLOCK_Z / 2.0 + 8.0);
        let hole = centered_cylinder(
            format!("thermal_contact_resistance_mapping_heat_block_mount_hole_{i}"),
            4.2 / 2.0,
            20.0,
            24,
        )
        .translate(*x, -(HEAT_BLOCK_Y / 2.0 + 18.0), -HEAT_BLOCK_Z / 2.0 + 8.0);
        ears = ears + (ear - hole);
    }
    ears
}

fn barcode_certificate_lands() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_barcode_certificate_base",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("thermal_contact_resistance_mapping_barcode_certificate_lands");
    for i in 0..BARCODE_LANDS {
        let x = -((BARCODE_LANDS as f64 - 1.0) * 58.0) / 2.0 + i as f64 * 58.0;
        lands = lands
            + centered_cube(
                format!("thermal_contact_resistance_mapping_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, -14.0, TRACE_Z / 2.0 + 2.0);
    }

    for i in 0..CERTIFICATE_LANDS {
        let x = -160.0 + i as f64 * 160.0;
        lands = lands
            + centered_cube(
                format!("thermal_contact_resistance_mapping_certificate_clip_land_{i}"),
                120.0,
                18.0,
                5.0,
            )
            .translate(x, 18.0, TRACE_Z / 2.0 + 2.5)
            + centered_cube(
                format!("thermal_contact_resistance_mapping_certificate_clip_stop_{i}"),
                120.0,
                4.0,
                12.0,
            )
            .translate(x, 31.0, TRACE_Z / 2.0 + 6.0);
    }

    base + lands
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "thermal_contact_resistance_mapping_release_hold_reject_base",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut cuts = Part::empty("thermal_contact_resistance_mapping_status_slot_cuts");
    let mut walls = Part::empty("thermal_contact_resistance_mapping_status_lane_walls");

    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        walls = walls
            + centered_cube(
                format!("thermal_contact_resistance_mapping_status_lane_header_{lane}"),
                STATUS_SLOT_X + 20.0,
                10.0,
                10.0,
            )
            .translate(x, STATUS_Y / 2.0 - 24.0, STATUS_Z / 2.0 + 5.0);

        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = -54.0 + slot as f64 * STATUS_SLOT_PITCH_Y;
            cuts = cuts
                + centered_cube(
                    format!("thermal_contact_resistance_mapping_status_slot_{lane}_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    12.0,
                )
                .translate(x, y, STATUS_Z / 2.0 - 4.0);
        }
    }

    for x in [
        -STATUS_LANE_PITCH_X / 2.0,
        STATUS_LANE_PITCH_X / 2.0,
        STATUS_LANE_PITCH_X * 1.5,
    ] {
        walls = walls
            + centered_cube(
                "thermal_contact_resistance_mapping_status_lane_segregation_wall",
                8.0,
                STATUS_Y - 28.0,
                34.0,
            )
            .translate(x, -4.0, STATUS_Z / 2.0 + 17.0);
    }

    base - cuts + walls
}

fn evidence_camera_bridge() -> Part {
    let mut bridge = Part::empty("thermal_contact_resistance_mapping_evidence_camera_bridge");

    for (i, (x, y)) in [
        (-(CAMERA_SPAN_X / 2.0), -(CAMERA_SPAN_Y / 2.0)),
        (CAMERA_SPAN_X / 2.0, -(CAMERA_SPAN_Y / 2.0)),
        (-(CAMERA_SPAN_X / 2.0), CAMERA_SPAN_Y / 2.0),
        (CAMERA_SPAN_X / 2.0, CAMERA_SPAN_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("thermal_contact_resistance_mapping_camera_bridge_post_{i}"),
                CAMERA_POST_X,
                CAMERA_POST_Y,
                CAMERA_UNDERSIDE_Z,
            )
            .translate(*x, *y, CAMERA_UNDERSIDE_Z / 2.0);
    }

    let beam_front = centered_cube(
        "thermal_contact_resistance_mapping_camera_bridge_front_beam",
        CAMERA_SPAN_X + CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        -(CAMERA_SPAN_Y / 2.0),
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let beam_rear = centered_cube(
        "thermal_contact_resistance_mapping_camera_bridge_rear_beam",
        CAMERA_SPAN_X + CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        CAMERA_SPAN_Y / 2.0,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let beam_cross = centered_cube(
        "thermal_contact_resistance_mapping_camera_bridge_center_crossbeam",
        44.0,
        CAMERA_SPAN_Y,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0);

    bridge = bridge + beam_front + beam_rear + beam_cross;

    for i in 0..CAMERA_COUNT {
        let x = -160.0 + i as f64 * 160.0;
        bridge = bridge
            + centered_cube(
                format!("thermal_contact_resistance_mapping_evidence_camera_pod_{i}"),
                54.0,
                44.0,
                34.0,
            )
            .translate(x, 0.0, CAMERA_UNDERSIDE_Z - 20.0)
            - centered_cylinder(
                format!("thermal_contact_resistance_mapping_evidence_camera_lens_relief_{i}"),
                14.0 / 2.0,
                40.0,
                28,
            )
            .translate(x, 0.0, CAMERA_UNDERSIDE_Z - 28.0);
    }

    for i in 0..LIGHT_BAR_COUNT {
        let y = if i == 0 { -136.0 } else { 136.0 };
        bridge = bridge
            + centered_cube(
                format!("thermal_contact_resistance_mapping_evidence_light_bar_{i}"),
                CAMERA_SPAN_X - 140.0,
                18.0,
                16.0,
            )
            .translate(0.0, y, CAMERA_UNDERSIDE_Z - 36.0);
    }

    bridge
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "thermal_contact_resistance_mapping_front_robot_pick_keepout",
        DECK_X - 130.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0, 0.0);
    let rear_service = centered_cube(
        "thermal_contact_resistance_mapping_rear_service_access_keepout",
        DECK_X - 160.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0, 0.0);
    let left_load = centered_cube(
        "thermal_contact_resistance_mapping_left_module_load_keepout",
        LEFT_MODULE_LOAD_KEEP_OUT_X,
        DECK_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_MODULE_LOAD_KEEP_OUT_X / 2.0,
        -16.0,
        0.0,
    );
    let right_service = centered_cube(
        "thermal_contact_resistance_mapping_right_thermal_service_keepout",
        RIGHT_THERMAL_SERVICE_KEEP_OUT_X,
        DECK_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_THERMAL_SERVICE_KEEP_OUT_X / 2.0,
        -12.0,
        0.0,
    );
    let robot_clearance_gauge = centered_cube(
        "thermal_contact_resistance_mapping_robot_clearance_height_gauge",
        140.0,
        24.0,
        ROBOT_CLEARANCE_Z,
    )
    .translate(-480.0, -DECK_Y / 2.0 + 76.0, ROBOT_CLEARANCE_Z / 2.0);
    let service_clearance_gauge = centered_cube(
        "thermal_contact_resistance_mapping_service_clearance_height_gauge",
        28.0,
        90.0,
        SERVICE_CLEARANCE_Z,
    )
    .translate(DECK_X / 2.0 - 86.0, 150.0, SERVICE_CLEARANCE_Z / 2.0);

    front_robot
        + rear_service
        + left_load
        + right_service
        + robot_clearance_gauge
        + service_clearance_gauge
}

fn calibration_coupon_retainer_pallet() -> Part {
    let body = centered_cube(
        "thermal_contact_resistance_mapping_calibration_coupon_retain_pallet",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    );
    let mut cuts = Part::empty("thermal_contact_resistance_mapping_retain_pallet_slot_cuts");
    let mut lands = Part::empty("thermal_contact_resistance_mapping_retain_pallet_barcode_lands");

    for i in 0..RETAIN_SLOTS {
        let y = -((RETAIN_SLOTS as f64 - 1.0) * RETAIN_SLOT_Y_PITCH) / 2.0
            + i as f64 * RETAIN_SLOT_Y_PITCH;
        cuts = cuts
            + centered_cube(
                format!("thermal_contact_resistance_mapping_retained_coupon_slot_{i}"),
                52.0,
                24.0,
                12.0,
            )
            .translate(0.0, y, RETAIN_Z / 2.0 - 4.0);
        lands = lands
            + centered_cube(
                format!("thermal_contact_resistance_mapping_retained_coupon_barcode_land_{i}"),
                58.0,
                7.0,
                4.0,
            )
            .translate(0.0, y + 17.0, RETAIN_Z / 2.0 + 2.0);
    }

    body - cuts + lands
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 42.0), -(DECK_Y / 2.0 - 42.0)),
        (DECK_X / 2.0 - 42.0, -(DECK_Y / 2.0 - 42.0)),
        (-(DECK_X / 2.0 - 42.0), DECK_Y / 2.0 - 42.0),
        (DECK_X / 2.0 - 42.0, DECK_Y / 2.0 - 42.0),
        (0.0, -(DECK_Y / 2.0 - 42.0)),
        (0.0, DECK_Y / 2.0 - 42.0),
        (-(DECK_X / 2.0 - 42.0), 0.0),
        (DECK_X / 2.0 - 42.0, 0.0),
    ]
}

fn module_datum_points() -> [(f64, f64); 4] {
    [
        (-(MODULE_X / 2.0 - 34.0), -(MODULE_Y / 2.0 - 28.0)),
        (MODULE_X / 2.0 - 34.0, -(MODULE_Y / 2.0 - 28.0)),
        (-(MODULE_X / 2.0 - 34.0), MODULE_Y / 2.0 - 28.0),
        (MODULE_X / 2.0 - 34.0, MODULE_Y / 2.0 - 28.0),
    ]
}

fn coupon_position(index: usize) -> (f64, f64) {
    let col = index % COUPON_COLS;
    let row = index / COUPON_COLS;
    (
        -((COUPON_COLS as f64 - 1.0) * COUPON_PITCH_X) / 2.0 + col as f64 * COUPON_PITCH_X,
        -((COUPON_ROWS as f64 - 1.0) * COUPON_PITCH_Y) / 2.0 + row as f64 * COUPON_PITCH_Y,
    )
}

fn probe_x(index: usize) -> f64 {
    -((PROBE_POCKETS as f64 - 1.0) * PROBE_PITCH_X) / 2.0 + index as f64 * PROBE_PITCH_X
}

fn status_lane_x(lane: usize) -> f64 {
    (lane as f64 - 1.0) * STATUS_LANE_PITCH_X
}

fn assert_layout() {
    for rect in [
        nest_rect(),
        coupon_rect(),
        force_rect(),
        probe_rect(),
        heat_block_rect(),
        trace_rect(),
        status_rect(),
        retain_rect(),
    ] {
        assert!(
            rect_inside(rect, deck_rect(), 12.0),
            "{} is outside the station deck footprint",
            rect.name
        );
    }

    assert!(!rects_overlap(nest_rect(), coupon_rect()));
    assert!(!rects_overlap(nest_rect(), force_rect()));
    assert!(!rects_overlap(nest_rect(), probe_rect()));
    assert!(!rects_overlap(coupon_rect(), force_rect()));
    assert!(!rects_overlap(coupon_rect(), trace_rect()));
    assert!(!rects_overlap(force_rect(), status_rect()));
    assert!(!rects_overlap(probe_rect(), heat_block_rect()));
    assert!(!rects_overlap(heat_block_rect(), status_rect()));
    assert!(!rects_overlap(status_rect(), retain_rect()));
    assert!(!rects_overlap(trace_rect(), nest_rect()));

    assert!(CAMERA_UNDERSIDE_Z > MODULE_KEEP_OUT_Z + 70.0);
    assert!(ROBOT_CLEARANCE_Z > MODULE_KEEP_OUT_Z + 48.0);
    assert!(SERVICE_CLEARANCE_Z > FORCE_GAUGE_ENV_Z + 20.0);
}

fn deck_rect() -> Rect {
    Rect::new("deck", 0.0, 0.0, DECK_X, DECK_Y)
}

fn nest_rect() -> Rect {
    Rect::new(
        "cassette/module datum nest",
        NEST_CENTER.0,
        NEST_CENTER.1,
        NEST_X,
        NEST_Y,
    )
}

fn coupon_rect() -> Rect {
    Rect::new(
        "thermal pad/gap coupon holders",
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        COUPON_HOLDER_X,
        COUPON_HOLDER_Y,
    )
}

fn force_rect() -> Rect {
    Rect::new(
        "spring preload force gauge pocket",
        FORCE_CENTER.0,
        FORCE_CENTER.1,
        FORCE_BASE_X,
        FORCE_BASE_Y,
    )
}

fn probe_rect() -> Rect {
    Rect::new(
        "temperature probe pockets",
        PROBE_CENTER.0,
        PROBE_CENTER.1,
        PROBE_BLOCK_X,
        PROBE_BLOCK_Y,
    )
}

fn heat_block_rect() -> Rect {
    Rect::new(
        "heat spreader reference block placeholder",
        HEAT_BLOCK_CENTER.0,
        HEAT_BLOCK_CENTER.1,
        HEAT_BLOCK_X,
        HEAT_BLOCK_Y,
    )
}

fn trace_rect() -> Rect {
    Rect::new(
        "barcode/certificate lands",
        TRACE_CENTER.0,
        TRACE_CENTER.1,
        TRACE_X,
        TRACE_Y,
    )
}

fn status_rect() -> Rect {
    Rect::new(
        "release/hold/reject lanes",
        STATUS_CENTER.0,
        STATUS_CENTER.1,
        STATUS_X,
        STATUS_Y,
    )
}

fn retain_rect() -> Rect {
    Rect::new(
        "calibration coupon retainer pallet",
        RETAIN_CENTER.0,
        RETAIN_CENTER.1,
        RETAIN_X,
        RETAIN_Y,
    )
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    const fn new(name: &'static str, x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { name, x, y, w, h }
    }

    fn left(self) -> f64 {
        self.x - self.w / 2.0
    }

    fn right(self) -> f64 {
        self.x + self.w / 2.0
    }

    fn bottom(self) -> f64 {
        self.y - self.h / 2.0
    }

    fn top(self) -> f64 {
        self.y + self.h / 2.0
    }
}

fn rect_inside(inner: Rect, outer: Rect, margin: f64) -> bool {
    inner.left() >= outer.left() + margin
        && inner.right() <= outer.right() - margin
        && inner.bottom() >= outer.bottom() + margin
        && inner.top() <= outer.top() - margin
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.left() < b.right() && a.right() > b.left() && a.bottom() < b.top() && a.top() > b.bottom()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(
                output.starts_with(
                    "output/closed_module_thermal_contact_resistance_mapping_station_"
                ),
                "{output} should be station-scoped"
            );
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_groups_cover_user_request() {
        assert_eq!(REQUIRED_FEATURES.len(), 9);
        assert!(REQUIRED_FEATURES.contains(&"cassette_module_datum_nest"));
        assert!(REQUIRED_FEATURES.contains(&"thermal_pad_gap_coupon_holders"));
        assert!(REQUIRED_FEATURES.contains(&"spring_preload_force_gauge_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"temperature_probe_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"heat_spreader_reference_block_placeholder"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn layout_rectangles_fit_and_do_not_collide() {
        assert_layout();
        assert!(NEST_X > MODULE_X + 70.0);
        assert!(NEST_Y > MODULE_Y + 70.0);
        assert!(rect_inside(trace_rect(), deck_rect(), 12.0));
    }

    #[test]
    fn thermal_mapping_capacity_is_explicit() {
        assert_eq!(COUPON_COUNT, COUPON_COLS * COUPON_ROWS);
        assert_eq!(COUPON_COUNT, 6);
        assert_eq!(GAP_SHIM_SLOTS_PER_COUPON, 3);
        assert_eq!(PROBE_POCKETS, 8);
        assert_eq!(HEAT_SPREAD_REFERENCE_POINTS, 12);
        assert_eq!(HEAT_SENSOR_WELLS, 4);
        assert!(PAD_COUPON_X > 60.0);
        assert!(PAD_COUPON_Y > 35.0);
    }

    #[test]
    fn preload_and_instrument_envelopes_are_practical() {
        assert_eq!(PRELOAD_SPRING_POSTS, 4);
        assert!(FORCE_GAUGE_ENV_Z > FORCE_BASE_Z * 4.0);
        assert!(PRELOAD_FORCE_RANGE_N >= 50.0);
        assert!(PRELOAD_PLUNGER_D >= 25.0);
        assert!(SERVICE_CLEARANCE_Z > FORCE_GAUGE_ENV_Z);
    }

    #[test]
    fn traceability_disposition_and_evidence_are_segregated() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert!(BARCODE_LANDS >= COUPON_COUNT);
        assert!(CERTIFICATE_LANDS >= 3);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LIGHT_BAR_COUNT, 2);
        assert!(CAMERA_UNDERSIDE_Z > MODULE_KEEP_OUT_Z + 70.0);
    }
}
