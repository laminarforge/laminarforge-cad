use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Sterile tubing weld/seal coupon leak-and-pull-test station.
//
// Research assumptions for placeholder geometry:
// - Commercial sterile tube welders/sealers produce short coupon samples that
//   are commonly challenged by pressure-decay leak checks and tensile pull
//   checks before releasing a welding/sealing setup for production use.
// - This station packages purchased weld/seal coupons, pressure-decay fittings,
//   a bought digital pull-test gauge, camera/barcode evidence capture, and
//   release/hold/reject routing. It models interface CAD only, not a validated
//   test method, sterile barrier, or force calibration standard.
//
// Exports:
//   output/sterile_tubing_weld_seal_coupon_test_station_deck.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_coupon_rack.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_sample_clamps.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_pressure_decay_ports.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_pull_gauge_envelope.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_cap_plug_parks.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_evidence_scanner_camera.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_status_lanes.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_leak_tray_segregation.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_robot_service_keepouts.stl
//   output/sterile_tubing_weld_seal_coupon_test_station_assembly.stl

const OUTPUT_PREFIX: &str = "output/sterile_tubing_weld_seal_coupon_test_station";
const OUTPUTS: [&str; 11] = [
    "output/sterile_tubing_weld_seal_coupon_test_station_deck.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_coupon_rack.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_sample_clamps.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_pressure_decay_ports.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_pull_gauge_envelope.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_cap_plug_parks.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_evidence_scanner_camera.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_status_lanes.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_leak_tray_segregation.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_robot_service_keepouts.stl",
    "output/sterile_tubing_weld_seal_coupon_test_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "coupon_rack",
    "weld_seal_sample_clamps",
    "pressure_decay_ports",
    "pull_test_gauge_envelope",
    "cap_plug_parks",
    "barcode_lands",
    "rfid_lands",
    "scanner_camera_evidence",
    "released_lane",
    "hold_lane",
    "reject_lane",
    "leak_tray",
    "clean_used_segregation",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;

const COUPON_RACK_X: f64 = 430.0;
const COUPON_RACK_Y: f64 = 245.0;
const COUPON_RACK_Z: f64 = 42.0;
const COUPON_RACK_CENTER_X: f64 = -335.0;
const COUPON_RACK_CENTER_Y: f64 = 185.0;
const COUPON_COLS: usize = 4;
const COUPON_ROWS: usize = 3;
const COUPON_SLOTS: usize = COUPON_COLS * COUPON_ROWS;
const COUPON_SLOT_X: f64 = 72.0;
const COUPON_SLOT_Y: f64 = 34.0;
const COUPON_SLOT_Z: f64 = 30.0;
const COUPON_PITCH_X: f64 = 92.0;
const COUPON_PITCH_Y: f64 = 58.0;

const CLAMP_RAIL_X: f64 = 720.0;
const CLAMP_RAIL_Y: f64 = 190.0;
const CLAMP_RAIL_Z: f64 = 34.0;
const CLAMP_CENTER_X: f64 = 35.0;
const CLAMP_CENTER_Y: f64 = 58.0;
const TEST_LANES: usize = 4;
const LANE_PITCH_Y: f64 = 42.0;
const TUBE_OD_MIN: f64 = 6.35;
const TUBE_OD_MAX: f64 = 25.4;
const TUBE_CLEARANCE: f64 = 2.0;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const WELD_WINDOW_X: f64 = -190.0;
const SEAL_WINDOW_X: f64 = 0.0;
const PULL_GRIP_WINDOW_X: f64 = 205.0;

const PRESSURE_PORT_BAR_X: f64 = 545.0;
const PRESSURE_PORT_BAR_Y: f64 = 100.0;
const PRESSURE_PORT_BAR_Z: f64 = 30.0;
const PRESSURE_PORT_CENTER_X: f64 = -25.0;
const PRESSURE_PORT_CENTER_Y: f64 = 252.0;
const PRESSURE_DECAY_PORTS: usize = TEST_LANES * 2;
const PRESSURE_PORT_D: f64 = 8.5;
const PRESSURE_PORT_PITCH_X: f64 = 60.0;
const SENSOR_TAP_D: f64 = 3.2;

const PULL_GAUGE_X: f64 = 250.0;
const PULL_GAUGE_Y: f64 = 175.0;
const PULL_GAUGE_Z: f64 = 135.0;
const PULL_GAUGE_CENTER_X: f64 = 405.0;
const PULL_GAUGE_CENTER_Y: f64 = 50.0;
const PULL_TRAVEL_X: f64 = 315.0;
const PULL_LOAD_AXIS_Z: f64 = 88.0;
const PULL_FORCE_LABEL_LAND_X: f64 = 92.0;

const CAP_PARK_X: f64 = 240.0;
const CAP_PARK_Y: f64 = 176.0;
const CAP_PARK_Z: f64 = 24.0;
const CAP_PARK_CENTER_X: f64 = 462.0;
const CAP_PARK_CENTER_Y: f64 = -212.0;
const CAP_PARK_COLS: usize = 5;
const CAP_PARK_ROWS: usize = 3;
const CAP_PLUG_WELLS: usize = CAP_PARK_COLS * CAP_PARK_ROWS;
const CAP_WELL_D: f64 = 18.0;
const CAP_WELL_PITCH_X: f64 = 36.0;
const CAP_WELL_PITCH_Y: f64 = 34.0;

const EVIDENCE_PANEL_X: f64 = 335.0;
const EVIDENCE_PANEL_Y: f64 = 132.0;
const EVIDENCE_PANEL_Z: f64 = 150.0;
const EVIDENCE_CENTER_X: f64 = -410.0;
const EVIDENCE_CENTER_Y: f64 = -226.0;
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;

const STATUS_LANE_X: f64 = 372.0;
const STATUS_LANE_Y: f64 = 286.0;
const STATUS_LANE_Z: f64 = 30.0;
const STATUS_CENTER_X: f64 = 56.0;
const STATUS_CENTER_Y: f64 = -232.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_PITCH_Y: f64 = 82.0;

const LEAK_TRAY_X: f64 = 820.0;
const LEAK_TRAY_Y: f64 = 292.0;
const LEAK_TRAY_Z: f64 = 26.0;
const LEAK_TRAY_CENTER_X: f64 = -20.0;
const LEAK_TRAY_CENTER_Y: f64 = 64.0;
const DRAIN_D: f64 = 10.0;
const SEGREGATION_WALL_X: f64 = 32.0;
const SEGREGATION_WALL_Y: f64 = 654.0;
const SEGREGATION_WALL_Z: f64 = 76.0;
const SEGREGATION_WALL_CENTER_X: f64 = 304.0;
const CLEAN_USED_AIR_GAP: f64 = 58.0;

const ROBOT_KEEP_OUTS: usize = 4;
const ROBOT_PICK_CLEARANCE_Z: f64 = 154.0;
const SERVICE_KEEP_OUT_Z: f64 = 118.0;
const MOUNT_HOLE_D: f64 = 6.6;

fn main() {
    fs::create_dir_all("output").unwrap();

    write_part(station_deck(), OUTPUTS[0]);
    write_part(coupon_rack(), OUTPUTS[1]);
    write_part(weld_seal_sample_clamps(), OUTPUTS[2]);
    write_part(pressure_decay_port_bar(), OUTPUTS[3]);
    write_part(pull_test_gauge_envelope(), OUTPUTS[4]);
    write_part(cap_plug_parks(), OUTPUTS[5]);
    write_part(scanner_camera_evidence_panel(), OUTPUTS[6]);
    write_part(release_status_lanes(), OUTPUTS[7]);
    write_part(leak_tray_and_clean_used_segregation(), OUTPUTS[8]);
    write_part(robot_service_keepouts(), OUTPUTS[9]);
    write_part(station_assembly(), OUTPUTS[10]);

    println!(
        "Sterile tubing weld/seal coupon test station: {:.0}mm x {:.0}mm deck, {} coupon slots, {} weld/seal clamp lanes, {} pressure-decay ports, {} cap/plug wells, {} barcode lands, {} RFID lands, {} status lanes, {} required interface groups, and {} keepout envelopes.",
        DECK_X,
        DECK_Y,
        COUPON_SLOTS,
        TEST_LANES,
        PRESSURE_DECAY_PORTS,
        CAP_PLUG_WELLS,
        BARCODE_LANDS,
        RFID_LANDS,
        STATUS_LANES,
        REQUIRED_FEATURES.len(),
        ROBOT_KEEP_OUTS
    );
    println!(
        "Purchased pull-test gauge envelope: {:.0}mm x {:.0}mm x {:.0}mm with {:.0}mm pull travel axis; tubing placeholders cover {:.2}-{:.1}mm OD coupons; outputs use prefix {OUTPUT_PREFIX}.",
        PULL_GAUGE_X,
        PULL_GAUGE_Y,
        PULL_GAUGE_Z,
        PULL_TRAVEL_X,
        TUBE_OD_MIN,
        TUBE_OD_MAX
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    station_deck()
        + coupon_rack()
        + leak_tray_and_clean_used_segregation()
        + weld_seal_sample_clamps()
        + pressure_decay_port_bar()
        + pull_test_gauge_envelope()
        + cap_plug_parks()
        + scanner_camera_evidence_panel()
        + release_status_lanes()
        + robot_service_keepouts()
}

fn station_deck() -> Part {
    let deck = centered_cube("sterile_tubing_coupon_station_deck", DECK_X, DECK_Y, DECK_Z);

    let mut recesses = Part::empty("sterile_tubing_coupon_station_component_recesses");
    for (i, (x, y, sx, sy)) in [
        (
            COUPON_RACK_CENTER_X,
            COUPON_RACK_CENTER_Y,
            COUPON_RACK_X + 28.0,
            COUPON_RACK_Y + 24.0,
        ),
        (
            CLAMP_CENTER_X,
            CLAMP_CENTER_Y,
            CLAMP_RAIL_X + 34.0,
            CLAMP_RAIL_Y + 30.0,
        ),
        (
            PRESSURE_PORT_CENTER_X,
            PRESSURE_PORT_CENTER_Y,
            PRESSURE_PORT_BAR_X + 22.0,
            PRESSURE_PORT_BAR_Y + 22.0,
        ),
        (
            PULL_GAUGE_CENTER_X,
            PULL_GAUGE_CENTER_Y,
            PULL_GAUGE_X + 38.0,
            PULL_GAUGE_Y + 34.0,
        ),
        (
            CAP_PARK_CENTER_X,
            CAP_PARK_CENTER_Y,
            CAP_PARK_X + 24.0,
            CAP_PARK_Y + 24.0,
        ),
        (
            EVIDENCE_CENTER_X,
            EVIDENCE_CENTER_Y,
            EVIDENCE_PANEL_X + 28.0,
            EVIDENCE_PANEL_Y + 30.0,
        ),
        (
            STATUS_CENTER_X,
            STATUS_CENTER_Y,
            STATUS_LANE_X + 26.0,
            STATUS_LANE_Y + 22.0,
        ),
        (
            LEAK_TRAY_CENTER_X,
            LEAK_TRAY_CENTER_Y,
            LEAK_TRAY_X + 30.0,
            LEAK_TRAY_Y + 26.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        recesses = recesses
            + centered_cube(
                format!("sterile_tubing_coupon_station_recess_{i}"),
                *sx,
                *sy,
                6.0,
            )
            .translate(*x, *y, DECK_Z / 2.0 - 2.5);
    }

    deck - recesses - deck_mount_holes()
        + deck_wipe_lips()
        + robot_fiducials()
        + deck_zone_label_lands()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("sterile_tubing_coupon_station_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("sterile_tubing_coupon_station_m6_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 58.0), -(DECK_Y / 2.0 - 54.0)),
        (DECK_X / 2.0 - 58.0, -(DECK_Y / 2.0 - 54.0)),
        (-(DECK_X / 2.0 - 58.0), DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 54.0),
        (0.0, -(DECK_Y / 2.0 - 54.0)),
        (0.0, DECK_Y / 2.0 - 54.0),
        (-(DECK_X / 2.0 - 58.0), 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
}

fn deck_wipe_lips() -> Part {
    let rear = centered_cube(
        "sterile_tubing_coupon_station_rear_wipe_lip",
        DECK_X - 86.0,
        14.0,
        26.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 28.0, DECK_Z / 2.0 + 13.0);
    let left = centered_cube(
        "sterile_tubing_coupon_station_clean_side_wipe_lip",
        14.0,
        DECK_Y - 100.0,
        24.0,
    )
    .translate(-DECK_X / 2.0 + 28.0, 0.0, DECK_Z / 2.0 + 12.0);
    let front_low = centered_cube(
        "sterile_tubing_coupon_station_front_robot_low_lip",
        DECK_X - 320.0,
        10.0,
        12.0,
    )
    .translate(-96.0, -DECK_Y / 2.0 + 24.0, DECK_Z / 2.0 + 6.0);

    rear + left + front_low
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("sterile_tubing_coupon_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 76.0), DECK_Y / 2.0 - 76.0),
        (DECK_X / 2.0 - 76.0, DECK_Y / 2.0 - 76.0),
        (-(DECK_X / 2.0 - 76.0), -(DECK_Y / 2.0 - 76.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials =
            fiducials
                + fiducial_target(&format!("sterile_tubing_coupon_station_fiducial_{i}"))
                    .translate(*x, *y, DECK_Z / 2.0 + 2.0);
    }
    fiducials
}

fn deck_zone_label_lands() -> Part {
    centered_cube(
        "sterile_tubing_coupon_station_clean_zone_label_land",
        165.0,
        36.0,
        4.0,
    )
    .translate(-454.0, 318.0, DECK_Z / 2.0 + 2.0)
        + centered_cube(
            "sterile_tubing_coupon_station_used_zone_label_land",
            165.0,
            36.0,
            4.0,
        )
        .translate(410.0, 318.0, DECK_Z / 2.0 + 2.0)
}

fn coupon_rack() -> Part {
    let rack = centered_cube(
        "sterile_tubing_coupon_rack_body",
        COUPON_RACK_X,
        COUPON_RACK_Y,
        COUPON_RACK_Z,
    );
    let mut pockets = Part::empty("sterile_tubing_coupon_rack_pockets");
    let mut retainer_tabs = Part::empty("sterile_tubing_coupon_rack_retainer_tabs");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let idx = row * COUPON_COLS + col;
            let (x, y) = coupon_slot_center(row, col);
            pockets = pockets
                + centered_cube(
                    format!("sterile_tubing_coupon_slot_{idx}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_Z,
                )
                .translate(x, y, COUPON_RACK_Z / 2.0 - COUPON_SLOT_Z / 2.0 + 5.0);
            retainer_tabs = retainer_tabs
                + centered_cube(
                    format!("sterile_tubing_coupon_slot_{idx}_spring_retainer"),
                    COUPON_SLOT_X - 18.0,
                    4.0,
                    8.0,
                )
                .translate(
                    x,
                    y + COUPON_SLOT_Y / 2.0 + 5.0,
                    COUPON_RACK_Z / 2.0 + 4.0,
                );
        }
    }

    let clean_feed_comb = centered_cube(
        "sterile_tubing_coupon_rack_clean_feed_comb",
        COUPON_RACK_X - 48.0,
        18.0,
        22.0,
    )
    .translate(0.0, COUPON_RACK_Y / 2.0 + 12.0, 1.0);
    let lot_land = centered_cube(
        "sterile_tubing_coupon_rack_lot_barcode_land",
        COUPON_RACK_X - 92.0,
        24.0,
        4.0,
    )
    .translate(
        0.0,
        -(COUPON_RACK_Y / 2.0 - 20.0),
        COUPON_RACK_Z / 2.0 + 2.0,
    );

    (rack - pockets + retainer_tabs + clean_feed_comb + lot_land).translate(
        COUPON_RACK_CENTER_X,
        COUPON_RACK_CENTER_Y,
        DECK_Z / 2.0 + COUPON_RACK_Z / 2.0,
    )
}

fn weld_seal_sample_clamps() -> Part {
    let rail = centered_cube(
        "sterile_tubing_coupon_clamp_lane_rail",
        CLAMP_RAIL_X,
        CLAMP_RAIL_Y,
        CLAMP_RAIL_Z,
    );

    let mut lane_cuts = Part::empty("sterile_tubing_coupon_clamp_lane_cuts");
    let mut clamp_bridges = Part::empty("sterile_tubing_coupon_clamp_bridges");
    let mut witness_lands = Part::empty("sterile_tubing_coupon_weld_seal_witness_lands");
    for lane in 0..TEST_LANES {
        let y = lane_y(lane);
        let channel = centered_cylinder(
            format!("sterile_tubing_coupon_lane_{lane}_max_od_channel"),
            TUBE_CHANNEL_D / 2.0,
            CLAMP_RAIL_X + 8.0,
            40,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 1.0);
        let top_slot = centered_cube(
            format!("sterile_tubing_coupon_lane_{lane}_open_top_slot"),
            CLAMP_RAIL_X + 10.0,
            TUBE_CHANNEL_D + 3.0,
            CLAMP_RAIL_Z,
        )
        .translate(0.0, y, CLAMP_RAIL_Z / 2.0 - 5.0);
        lane_cuts = lane_cuts + channel + top_slot;

        for (side, x) in [
            ("weld_left", WELD_WINDOW_X - 68.0),
            ("weld_right", WELD_WINDOW_X + 68.0),
            ("seal_left", SEAL_WINDOW_X - 58.0),
            ("seal_right", SEAL_WINDOW_X + 58.0),
            ("pull_grip_left", PULL_GRIP_WINDOW_X - 72.0),
            ("pull_grip_right", PULL_GRIP_WINDOW_X + 72.0),
        ] {
            clamp_bridges = clamp_bridges
                + centered_cube(
                    format!("sterile_tubing_coupon_lane_{lane}_{side}_clamp_bridge"),
                    30.0,
                    TUBE_CHANNEL_D + 19.0,
                    20.0,
                )
                .translate(x, y, CLAMP_RAIL_Z / 2.0 + 10.0);
        }

        witness_lands = witness_lands
            + centered_cube(
                format!("sterile_tubing_coupon_lane_{lane}_camera_weld_shadow_land"),
                92.0,
                16.0,
                4.0,
            )
            .translate(
                WELD_WINDOW_X,
                y - TUBE_CHANNEL_D / 2.0 - 13.0,
                CLAMP_RAIL_Z / 2.0 + 2.0,
            )
            + centered_cube(
                format!("sterile_tubing_coupon_lane_{lane}_camera_seal_shadow_land"),
                86.0,
                16.0,
                4.0,
            )
            .translate(
                SEAL_WINDOW_X,
                y - TUBE_CHANNEL_D / 2.0 - 13.0,
                CLAMP_RAIL_Z / 2.0 + 2.0,
            );
    }

    let weld_window = process_window("weld_coupon", WELD_WINDOW_X, 52.0);
    let seal_window = process_window("seal_coupon", SEAL_WINDOW_X, 48.0);
    let pull_window = process_window("pull_grip", PULL_GRIP_WINDOW_X, 62.0);
    let end_stops = centered_cube(
        "sterile_tubing_coupon_clean_end_hard_stop",
        18.0,
        CLAMP_RAIL_Y - 24.0,
        CLAMP_RAIL_Z + 18.0,
    )
    .translate(-CLAMP_RAIL_X / 2.0 + 28.0, 0.0, 9.0)
        + centered_cube(
            "sterile_tubing_coupon_used_end_hard_stop",
            18.0,
            CLAMP_RAIL_Y - 24.0,
            CLAMP_RAIL_Z + 18.0,
        )
        .translate(CLAMP_RAIL_X / 2.0 - 28.0, 0.0, 9.0);

    (rail - lane_cuts - weld_window - seal_window - pull_window
        + clamp_bridges
        + witness_lands
        + end_stops)
        .translate(
            CLAMP_CENTER_X,
            CLAMP_CENTER_Y,
            DECK_Z / 2.0 + CLAMP_RAIL_Z / 2.0,
        )
}

fn process_window(name: &str, x: f64, width: f64) -> Part {
    centered_cube(
        format!("sterile_tubing_coupon_{name}_service_window"),
        width,
        CLAMP_RAIL_Y + 8.0,
        CLAMP_RAIL_Z + 4.0,
    )
    .translate(x, 0.0, 0.0)
}

fn pressure_decay_port_bar() -> Part {
    let bar = centered_cube(
        "sterile_tubing_coupon_pressure_decay_port_bar",
        PRESSURE_PORT_BAR_X,
        PRESSURE_PORT_BAR_Y,
        PRESSURE_PORT_BAR_Z,
    );
    let mut ports = Part::empty("sterile_tubing_coupon_pressure_decay_port_cuts");
    let mut bosses = Part::empty("sterile_tubing_coupon_pressure_decay_port_bosses");
    let mut taps = Part::empty("sterile_tubing_coupon_pressure_sensor_taps");
    for port in 0..PRESSURE_DECAY_PORTS {
        let x = pressure_port_x(port);
        let y = if port % 2 == 0 { -21.0 } else { 21.0 };
        bosses = bosses
            + centered_cylinder(
                format!("sterile_tubing_coupon_pressure_port_boss_{port}"),
                23.0 / 2.0,
                10.0,
                32,
            )
            .translate(x, y, PRESSURE_PORT_BAR_Z / 2.0 + 5.0);
        ports = ports
            + centered_cylinder(
                format!("sterile_tubing_coupon_pressure_port_clearance_{port}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_PORT_BAR_Z + 14.0,
                28,
            )
            .translate(x, y, 2.0);
        taps = taps
            + centered_cylinder(
                format!("sterile_tubing_coupon_pressure_sensor_tap_{port}"),
                SENSOR_TAP_D / 2.0,
                42.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                PRESSURE_PORT_BAR_Y / 2.0 + 7.0,
                PRESSURE_PORT_BAR_Z / 2.0 + 9.0,
            );
    }

    let pressure_header = centered_cube(
        "sterile_tubing_coupon_pressure_decay_manifold_header",
        PRESSURE_PORT_BAR_X - 48.0,
        12.0,
        16.0,
    )
    .translate(
        0.0,
        PRESSURE_PORT_BAR_Y / 2.0 + 6.0,
        PRESSURE_PORT_BAR_Z / 2.0 + 8.0,
    );
    let pass_fail_land = centered_cube(
        "sterile_tubing_coupon_pressure_decay_pass_fail_label_land",
        156.0,
        24.0,
        4.0,
    )
    .translate(
        -(PRESSURE_PORT_BAR_X / 2.0 - 92.0),
        -(PRESSURE_PORT_BAR_Y / 2.0 - 18.0),
        PRESSURE_PORT_BAR_Z / 2.0 + 2.0,
    );

    (bar + bosses + taps + pressure_header + pass_fail_land - ports).translate(
        PRESSURE_PORT_CENTER_X,
        PRESSURE_PORT_CENTER_Y,
        DECK_Z / 2.0 + PRESSURE_PORT_BAR_Z / 2.0,
    )
}

fn pull_test_gauge_envelope() -> Part {
    let gauge = centered_cube(
        "sterile_tubing_coupon_pull_test_gauge_purchased_envelope",
        PULL_GAUGE_X,
        PULL_GAUGE_Y,
        PULL_GAUGE_Z,
    );
    let display = centered_cube(
        "sterile_tubing_coupon_pull_gauge_display_window",
        104.0,
        8.0,
        44.0,
    )
    .translate(32.0, -(PULL_GAUGE_Y / 2.0 + 4.0), PULL_GAUGE_Z / 2.0 - 42.0);
    let load_cell_axis = centered_cylinder(
        "sterile_tubing_coupon_pull_test_load_axis_clearance",
        13.0 / 2.0,
        PULL_TRAVEL_X,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -PULL_GAUGE_X / 2.0 - PULL_TRAVEL_X / 2.0 + 10.0,
        0.0,
        PULL_LOAD_AXIS_Z,
    );
    let fixed_grip = centered_cube(
        "sterile_tubing_coupon_pull_fixed_grip_envelope",
        54.0,
        82.0,
        76.0,
    )
    .translate(-PULL_GAUGE_X / 2.0 - 48.0, 0.0, PULL_LOAD_AXIS_Z);
    let moving_grip = centered_cube(
        "sterile_tubing_coupon_pull_moving_grip_envelope",
        54.0,
        82.0,
        76.0,
    )
    .translate(PULL_GAUGE_X / 2.0 - 30.0, 0.0, PULL_LOAD_AXIS_Z);
    let travel_guard = centered_cube(
        "sterile_tubing_coupon_pull_travel_guard_keepclear",
        PULL_TRAVEL_X,
        24.0,
        34.0,
    )
    .translate(
        -PULL_GAUGE_X / 2.0 - PULL_TRAVEL_X / 2.0 + 10.0,
        0.0,
        PULL_LOAD_AXIS_Z,
    );
    let force_label = centered_cube(
        "sterile_tubing_coupon_pull_force_result_barcode_land",
        PULL_FORCE_LABEL_LAND_X,
        28.0,
        4.0,
    )
    .translate(42.0, -(PULL_GAUGE_Y / 2.0 + 18.0), 18.0);

    (gauge + display + fixed_grip + moving_grip + travel_guard + force_label - load_cell_axis)
        .translate(
            PULL_GAUGE_CENTER_X,
            PULL_GAUGE_CENTER_Y,
            DECK_Z / 2.0 + PULL_GAUGE_Z / 2.0,
        )
}

fn cap_plug_parks() -> Part {
    let tray = centered_cube(
        "sterile_tubing_coupon_cap_plug_park_tray",
        CAP_PARK_X,
        CAP_PARK_Y,
        CAP_PARK_Z,
    );
    let divider = centered_cube(
        "sterile_tubing_coupon_cap_plug_clean_used_divider",
        CAP_PARK_X - 28.0,
        8.0,
        CAP_PARK_Z + 14.0,
    )
    .translate(0.0, 0.0, 7.0);
    let mut wells = Part::empty("sterile_tubing_coupon_cap_plug_park_wells");
    let mut tick_marks = Part::empty("sterile_tubing_coupon_cap_plug_orientation_marks");
    for row in 0..CAP_PARK_ROWS {
        for col in 0..CAP_PARK_COLS {
            let idx = row * CAP_PARK_COLS + col;
            let (x, y) = cap_well_center(row, col);
            wells = wells
                + centered_cylinder(
                    format!("sterile_tubing_coupon_cap_plug_well_{idx}"),
                    CAP_WELL_D / 2.0,
                    CAP_PARK_Z + 3.0,
                    32,
                )
                .translate(x, y, 1.5);
            tick_marks = tick_marks
                + centered_cube(
                    format!("sterile_tubing_coupon_cap_plug_orientation_tick_{idx}"),
                    4.0,
                    11.0,
                    3.0,
                )
                .translate(x + CAP_WELL_D / 2.0 + 4.0, y, CAP_PARK_Z / 2.0 + 1.5);
        }
    }
    let empty_full_lands = centered_cube(
        "sterile_tubing_coupon_cap_plug_clean_lot_land",
        CAP_PARK_X / 2.0 - 28.0,
        22.0,
        4.0,
    )
    .translate(
        -CAP_PARK_X / 4.0,
        -(CAP_PARK_Y / 2.0 - 18.0),
        CAP_PARK_Z / 2.0 + 2.0,
    ) + centered_cube(
        "sterile_tubing_coupon_cap_plug_used_lot_land",
        CAP_PARK_X / 2.0 - 28.0,
        22.0,
        4.0,
    )
    .translate(
        CAP_PARK_X / 4.0,
        -(CAP_PARK_Y / 2.0 - 18.0),
        CAP_PARK_Z / 2.0 + 2.0,
    );

    (tray - wells + divider + tick_marks + empty_full_lands).translate(
        CAP_PARK_CENTER_X,
        CAP_PARK_CENTER_Y,
        DECK_Z / 2.0 + CAP_PARK_Z / 2.0,
    )
}

fn scanner_camera_evidence_panel() -> Part {
    let mast = centered_cube(
        "sterile_tubing_coupon_evidence_scanner_camera_mast",
        EVIDENCE_PANEL_X,
        EVIDENCE_PANEL_Y,
        EVIDENCE_PANEL_Z,
    );
    let camera_window = centered_cube(
        "sterile_tubing_coupon_camera_evidence_window",
        136.0,
        10.0,
        88.0,
    )
    .translate(-66.0, -(EVIDENCE_PANEL_Y / 2.0 + 5.0), 18.0);
    let scanner_slot = centered_cube(
        "sterile_tubing_coupon_barcode_scanner_slot",
        108.0,
        10.0,
        36.0,
    )
    .translate(92.0, -(EVIDENCE_PANEL_Y / 2.0 + 5.0), -38.0);
    let ring_light = centered_cylinder(
        "sterile_tubing_coupon_camera_ring_light_boss",
        56.0,
        8.0,
        56,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-66.0, -(EVIDENCE_PANEL_Y / 2.0 + 9.0), 20.0);
    let lens_cut = centered_cylinder(
        "sterile_tubing_coupon_camera_lens_clearance",
        26.0,
        12.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-66.0, -(EVIDENCE_PANEL_Y / 2.0 + 10.0), 20.0);
    let mut lands = Part::empty("sterile_tubing_coupon_evidence_barcode_rfid_lands");
    for land in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("sterile_tubing_coupon_barcode_land_{land}"),
                78.0,
                22.0,
                4.0,
            )
            .translate(
                -128.0 + land as f64 * 52.0,
                EVIDENCE_PANEL_Y / 2.0 + 16.0,
                -58.0,
            );
    }
    for land in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("sterile_tubing_coupon_rfid_land_{land}"),
                44.0,
                34.0,
                4.0,
            )
            .translate(
                -112.0 + land as f64 * 74.0,
                EVIDENCE_PANEL_Y / 2.0 + 16.0,
                58.0,
            );
    }

    (mast + ring_light + lands - camera_window - scanner_slot - lens_cut).translate(
        EVIDENCE_CENTER_X,
        EVIDENCE_CENTER_Y,
        DECK_Z / 2.0 + EVIDENCE_PANEL_Z / 2.0,
    )
}

fn release_status_lanes() -> Part {
    let body = centered_cube(
        "sterile_tubing_coupon_release_hold_reject_lane_body",
        STATUS_LANE_X,
        STATUS_LANE_Y,
        STATUS_LANE_Z,
    );
    let mut trays = Part::empty("sterile_tubing_coupon_release_hold_reject_lane_recesses");
    let mut labels = Part::empty("sterile_tubing_coupon_release_hold_reject_labels");
    for lane in 0..STATUS_LANES {
        let y = status_lane_y(lane);
        let name = match lane {
            0 => "released",
            1 => "hold",
            _ => "reject",
        };
        trays = trays
            + centered_cube(
                format!("sterile_tubing_coupon_{name}_lane_recess"),
                STATUS_LANE_X - 52.0,
                48.0,
                16.0,
            )
            .translate(0.0, y, STATUS_LANE_Z / 2.0 - 6.0);
        labels = labels
            + centered_cube(
                format!("sterile_tubing_coupon_{name}_lane_result_label_land"),
                118.0,
                22.0,
                4.0,
            )
            .translate(-(STATUS_LANE_X / 2.0 - 76.0), y, STATUS_LANE_Z / 2.0 + 2.0)
            + centered_cube(
                format!("sterile_tubing_coupon_{name}_lane_rfid_land"),
                42.0,
                28.0,
                4.0,
            )
            .translate(STATUS_LANE_X / 2.0 - 54.0, y, STATUS_LANE_Z / 2.0 + 2.0);
    }

    (body - trays + labels + status_lane_dividers()).translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y,
        DECK_Z / 2.0 + STATUS_LANE_Z / 2.0,
    )
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("sterile_tubing_coupon_status_lane_dividers");
    for i in 0..2 {
        let y = -STATUS_LANE_PITCH_Y / 2.0 + i as f64 * STATUS_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("sterile_tubing_coupon_status_lane_divider_{i}"),
                STATUS_LANE_X - 36.0,
                6.0,
                STATUS_LANE_Z + 14.0,
            )
            .translate(0.0, y, 7.0);
    }
    dividers
}

fn leak_tray_and_clean_used_segregation() -> Part {
    leak_tray() + clean_used_segregation_wall()
}

fn leak_tray() -> Part {
    let tray = centered_cube(
        "sterile_tubing_coupon_leak_witness_outer_tray",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let basin = centered_cube(
        "sterile_tubing_coupon_leak_witness_basin",
        LEAK_TRAY_X - 44.0,
        LEAK_TRAY_Y - 42.0,
        13.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0 - 5.5);
    let drain = centered_cylinder(
        "sterile_tubing_coupon_leak_tray_low_point_drain",
        DRAIN_D / 2.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_TRAY_X / 2.0 - 44.0, -(LEAK_TRAY_Y / 2.0 + 12.0), 0.0);
    let mut ribs = Part::empty("sterile_tubing_coupon_leak_tray_lane_ribs");
    for lane in 0..TEST_LANES {
        ribs = ribs
            + centered_cube(
                format!("sterile_tubing_coupon_leak_tray_lane_{lane}_flow_rib"),
                LEAK_TRAY_X - 96.0,
                4.0,
                6.0,
            )
            .translate(0.0, lane_y(lane), LEAK_TRAY_Z / 2.0 - 2.0);
    }
    let witness_sensor_land = centered_cube(
        "sterile_tubing_coupon_leak_tray_wetness_sensor_land",
        82.0,
        38.0,
        6.0,
    )
    .translate(
        LEAK_TRAY_X / 2.0 - 76.0,
        -(LEAK_TRAY_Y / 2.0 - 38.0),
        LEAK_TRAY_Z / 2.0 + 3.0,
    );

    (tray - basin - drain + ribs + witness_sensor_land).translate(
        LEAK_TRAY_CENTER_X,
        LEAK_TRAY_CENTER_Y,
        DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0,
    )
}

fn clean_used_segregation_wall() -> Part {
    let wall = centered_cube(
        "sterile_tubing_coupon_clean_used_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X,
        0.0,
        DECK_Z / 2.0 + SEGREGATION_WALL_Z / 2.0,
    );
    let air_gap_gauge = centered_cube(
        "sterile_tubing_coupon_clean_used_air_gap_gauge",
        CLEAN_USED_AIR_GAP,
        SEGREGATION_WALL_Y - 86.0,
        10.0,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X - SEGREGATION_WALL_X / 2.0 - CLEAN_USED_AIR_GAP / 2.0,
        0.0,
        DECK_Z / 2.0 + 5.0,
    );
    let pass_through_gate = centered_cube(
        "sterile_tubing_coupon_status_pass_through_gate_clearance",
        SEGREGATION_WALL_X + 6.0,
        138.0,
        44.0,
    )
    .translate(SEGREGATION_WALL_CENTER_X, -246.0, DECK_Z / 2.0 + 28.0);

    wall + air_gap_gauge - pass_through_gate
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        "sterile_tubing_coupon_front_robot_pick_keepout",
        DECK_X - 270.0,
        92.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 64.0,
        DECK_Z / 2.0 + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let coupon_pick = keepout_box(
        "sterile_tubing_coupon_rack_robot_pick_keepout",
        COUPON_RACK_X + 120.0,
        COUPON_RACK_Y + 92.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        COUPON_RACK_CENTER_X,
        COUPON_RACK_CENTER_Y,
        DECK_Z / 2.0 + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let pull_service = keepout_box(
        "sterile_tubing_coupon_pull_gauge_service_keepout",
        PULL_GAUGE_X + 110.0,
        PULL_GAUGE_Y + 138.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        PULL_GAUGE_CENTER_X,
        PULL_GAUGE_CENTER_Y,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let rear_pressure = keepout_box(
        "sterile_tubing_coupon_rear_pressure_service_keepout",
        PRESSURE_PORT_BAR_X + 130.0,
        120.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        PRESSURE_PORT_CENTER_X,
        DECK_Y / 2.0 - 88.0,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );

    front_robot + coupon_pick + pull_service + rear_pressure
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let shell = centered_cube(format!("{name}_envelope"), x, y, z);
    let inner = centered_cube(format!("{name}_hollow"), x - 16.0, y - 16.0, z - 12.0);
    let label = centered_cube(format!("{name}_label_land"), x * 0.42, 18.0, 3.0).translate(
        0.0,
        -(y / 2.0 - 12.0),
        z / 2.0 + 1.5,
    );
    shell - inner + label
}

fn fiducial_target(name: &str) -> Part {
    let pad = centered_cylinder(format!("{name}_pad"), 15.0, 3.0, 48);
    let cross_x = centered_cube(format!("{name}_cross_x"), 25.0, 3.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 25.0, 4.0);
    pad - cross_x - cross_y
}

fn coupon_slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X,
        ((COUPON_ROWS as f64 - 1.0) / 2.0 - row as f64) * COUPON_PITCH_Y,
    )
}

fn lane_y(lane: usize) -> f64 {
    (lane as f64 - (TEST_LANES as f64 - 1.0) / 2.0) * LANE_PITCH_Y
}

fn pressure_port_x(port: usize) -> f64 {
    (port as f64 - (PRESSURE_DECAY_PORTS as f64 - 1.0) / 2.0) * PRESSURE_PORT_PITCH_X
}

fn cap_well_center(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (CAP_PARK_COLS as f64 - 1.0) / 2.0) * CAP_WELL_PITCH_X,
        ((CAP_PARK_ROWS as f64 - 1.0) / 2.0 - row as f64) * CAP_WELL_PITCH_Y,
    )
}

fn status_lane_y(lane: usize) -> f64 {
    ((STATUS_LANES as f64 - 1.0) / 2.0 - lane as f64) * STATUS_LANE_PITCH_Y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_count_matches_exported_components() {
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn includes_required_station_interfaces() {
        for feature in [
            "coupon_rack",
            "weld_seal_sample_clamps",
            "pressure_decay_ports",
            "pull_test_gauge_envelope",
            "cap_plug_parks",
            "scanner_camera_evidence",
            "released_lane",
            "hold_lane",
            "reject_lane",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 14);
    }

    #[test]
    fn coupon_and_test_lane_counts_are_consistent() {
        assert_eq!(COUPON_SLOTS, COUPON_COLS * COUPON_ROWS);
        assert_eq!(PRESSURE_DECAY_PORTS, TEST_LANES * 2);
        assert_eq!(CAP_PLUG_WELLS, CAP_PARK_COLS * CAP_PARK_ROWS);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(ROBOT_KEEP_OUTS, 4);
    }

    #[test]
    fn tubing_and_pull_gauge_geometry_has_clearance() {
        assert!(TUBE_OD_MIN < TUBE_OD_MAX);
        assert!(TUBE_CHANNEL_D > TUBE_OD_MAX);
        assert!(PULL_TRAVEL_X > PULL_GAUGE_X);
        assert!(PULL_LOAD_AXIS_Z < PULL_GAUGE_Z);
        assert!(PRESSURE_PORT_D < TUBE_CHANNEL_D);
    }

    #[test]
    fn deck_contains_all_major_modules() {
        for (x, y, module_x, module_y) in [
            (
                COUPON_RACK_CENTER_X,
                COUPON_RACK_CENTER_Y,
                COUPON_RACK_X,
                COUPON_RACK_Y,
            ),
            (CLAMP_CENTER_X, CLAMP_CENTER_Y, CLAMP_RAIL_X, CLAMP_RAIL_Y),
            (
                PRESSURE_PORT_CENTER_X,
                PRESSURE_PORT_CENTER_Y,
                PRESSURE_PORT_BAR_X,
                PRESSURE_PORT_BAR_Y,
            ),
            (
                PULL_GAUGE_CENTER_X,
                PULL_GAUGE_CENTER_Y,
                PULL_GAUGE_X,
                PULL_GAUGE_Y,
            ),
            (CAP_PARK_CENTER_X, CAP_PARK_CENTER_Y, CAP_PARK_X, CAP_PARK_Y),
            (
                STATUS_CENTER_X,
                STATUS_CENTER_Y,
                STATUS_LANE_X,
                STATUS_LANE_Y,
            ),
        ] {
            assert!(x.abs() + module_x / 2.0 < DECK_X / 2.0);
            assert!(y.abs() + module_y / 2.0 < DECK_Y / 2.0);
        }
    }

    #[test]
    fn generated_parts_are_non_empty() {
        let parts = [
            station_deck(),
            coupon_rack(),
            weld_seal_sample_clamps(),
            pressure_decay_port_bar(),
            pull_test_gauge_envelope(),
            cap_plug_parks(),
            scanner_camera_evidence_panel(),
            release_status_lanes(),
            leak_tray_and_clean_used_segregation(),
            robot_service_keepouts(),
            station_assembly(),
        ];
        assert_eq!(parts.len(), OUTPUTS.len());
    }
}
