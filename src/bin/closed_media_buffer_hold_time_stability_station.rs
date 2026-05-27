use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/buffer hold-time stability station.
//
// Scope:
// - Models physical nests, lands, labels, waste capture, and keepout gauges for
//   holding sealed media and buffer containers while timepoint samples are
//   retained for pH, osmolality, conductivity, and temperature evidence.
// - Represents release/hold/reject segregation as keyed lanes so the workcell
//   cannot silently present a rejected or pending container as released.
// - This is concept/interface CAD only. It is not a stability protocol,
//   acceptance criterion, sterile-processing claim, or release decision model.

const OUTPUTS: [&str; 11] = [
    "output/closed_media_buffer_hold_time_stability_station_base_leak_tray.stl",
    "output/closed_media_buffer_hold_time_stability_station_sealed_bag_nests.stl",
    "output/closed_media_buffer_hold_time_stability_station_sealed_bottle_nests.stl",
    "output/closed_media_buffer_hold_time_stability_station_timepoint_sample_loop.stl",
    "output/closed_media_buffer_hold_time_stability_station_temperature_logger_pocket.stl",
    "output/closed_media_buffer_hold_time_stability_station_ph_osmolality_conductivity_standard_lands.stl",
    "output/closed_media_buffer_hold_time_stability_station_release_hold_reject_lanes.stl",
    "output/closed_media_buffer_hold_time_stability_station_chain_of_custody_label_lands.stl",
    "output/closed_media_buffer_hold_time_stability_station_waste_capture.stl",
    "output/closed_media_buffer_hold_time_stability_station_robot_service_keepouts.stl",
    "output/closed_media_buffer_hold_time_stability_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "sealed_bag_nests",
    "sealed_bottle_nests",
    "timepoint_sample_loop",
    "temperature_logger_pocket",
    "ph_standard_lands",
    "osmolality_standard_lands",
    "conductivity_standard_lands",
    "release_lane",
    "hold_lane",
    "reject_lane",
    "chain_of_custody_labels",
    "waste_capture",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const SOCKET_DEPTH: f64 = 6.0;

const BAG_CENTER: (f64, f64) = (-405.0, 200.0);
const BAG_NEST_X: f64 = 430.0;
const BAG_NEST_Y: f64 = 255.0;
const BAG_NEST_Z: f64 = 54.0;
const BAG_BAY_COUNT: usize = 4;
const BAG_BAY_COLS: usize = 2;
const BAG_BAY_X: f64 = 168.0;
const BAG_BAY_Y: f64 = 86.0;
const BAG_BAY_RECESS_Z: f64 = 18.0;
const BAG_PORT_COMBS_PER_BAY: usize = 2;

const BOTTLE_CENTER: (f64, f64) = (125.0, 205.0);
const BOTTLE_RACK_X: f64 = 425.0;
const BOTTLE_RACK_Y: f64 = 255.0;
const BOTTLE_RACK_Z: f64 = 58.0;
const BOTTLE_NEST_COUNT: usize = 6;
const BOTTLE_NEST_COLS: usize = 3;
const BOTTLE_WELL_D: f64 = 64.0;
const BOTTLE_WELL_PITCH_X: f64 = 118.0;
const BOTTLE_WELL_PITCH_Y: f64 = 102.0;

const LOOP_CENTER: (f64, f64) = (-390.0, -75.0);
const LOOP_PANEL_X: f64 = 455.0;
const LOOP_PANEL_Y: f64 = 240.0;
const LOOP_PANEL_Z: f64 = 34.0;
const TIMEPOINT_COUNT: usize = 8;
const TIMEPOINT_COLS: usize = 4;
const TIMEPOINT_PITCH_X: f64 = 82.0;
const TIMEPOINT_PITCH_Y: f64 = 78.0;
const TIMEPOINT_VIAL_D: f64 = 21.0;
const SAMPLE_LOOP_CHANNELS: usize = 2;

const LOGGER_CENTER: (f64, f64) = (95.0, -80.0);
const LOGGER_BLOCK_X: f64 = 300.0;
const LOGGER_BLOCK_Y: f64 = 240.0;
const LOGGER_BLOCK_Z: f64 = 54.0;
const LOGGER_SLOT_X: f64 = 176.0;
const LOGGER_SLOT_Y: f64 = 86.0;
const LOGGER_SLOT_Z: f64 = 24.0;
const THERMOWELL_COUNT: usize = 6;
const THERMOWELL_PITCH: f64 = 37.0;

const STANDARD_CENTER: (f64, f64) = (430.0, -80.0);
const STANDARD_PANEL_X: f64 = 300.0;
const STANDARD_PANEL_Y: f64 = 240.0;
const STANDARD_PANEL_Z: f64 = 34.0;
const STANDARD_TYPES: [&str; 3] = ["ph", "osmolality", "conductivity"];
const STANDARD_REPLICATES: usize = 4;
const STANDARD_WELL_D: f64 = 20.0;

const LANE_CENTER: (f64, f64) = (-385.0, -300.0);
const LANE_PANEL_X: f64 = 450.0;
const LANE_PANEL_Y: f64 = 160.0;
const LANE_PANEL_Z: f64 = 40.0;
const LANE_NAMES: [&str; 3] = ["release", "hold", "reject"];
const LANE_POSITIONS_PER_STATUS: usize = 4;
const LANE_TOKEN_X: f64 = 54.0;
const LANE_TOKEN_Y: f64 = 36.0;

const LABEL_CENTER: (f64, f64) = (95.0, -300.0);
const LABEL_PANEL_X: f64 = 300.0;
const LABEL_PANEL_Y: f64 = 160.0;
const LABEL_PANEL_Z: f64 = 18.0;
const CHAIN_LABEL_COUNT: usize = 12;
const CHAIN_LABEL_COLS: usize = 4;
const RFID_LABEL_COUNT: usize = 4;
const TAMPER_SEAL_COUNT: usize = 8;

const WASTE_CENTER: (f64, f64) = (430.0, -300.0);
const WASTE_TRAY_X: f64 = 300.0;
const WASTE_TRAY_Y: f64 = 160.0;
const WASTE_TRAY_Z: f64 = 44.0;
const WASTE_BOTTLE_COUNT: usize = 2;
const WASTE_BOTTLE_D: f64 = 62.0;
const ABSORBENT_PAD_COUNT: usize = 6;
const DRAIN_PORT_D: f64 = 18.0;

const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_SERVICE_CLEARANCE: f64 = 220.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_LOGGER_CLEARANCE: f64 = 320.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;
const KEEP_OUT_RAIL: f64 = 8.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let bags = sealed_bag_nests();
    export(OUTPUTS[1], &bags);

    let bottles = sealed_bottle_nests();
    export(OUTPUTS[2], &bottles);

    let loop_panel = timepoint_sample_loop();
    export(OUTPUTS[3], &loop_panel);

    let logger = temperature_logger_pocket();
    export(OUTPUTS[4], &logger);

    let standards = ph_osmolality_conductivity_standard_lands();
    export(OUTPUTS[5], &standards);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[6], &lanes);

    let labels = chain_of_custody_label_lands();
    export(OUTPUTS[7], &labels);

    let waste = waste_capture();
    export(OUTPUTS[8], &waste);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + bags.translate(BAG_CENTER.0, BAG_CENTER.1, insert_z(BAG_NEST_Z))
        + bottles.translate(BOTTLE_CENTER.0, BOTTLE_CENTER.1, insert_z(BOTTLE_RACK_Z))
        + loop_panel.translate(LOOP_CENTER.0, LOOP_CENTER.1, insert_z(LOOP_PANEL_Z))
        + logger.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, insert_z(LOGGER_BLOCK_Z))
        + standards.translate(
            STANDARD_CENTER.0,
            STANDARD_CENTER.1,
            insert_z(STANDARD_PANEL_Z),
        )
        + lanes.translate(LANE_CENTER.0, LANE_CENTER.1, insert_z(LANE_PANEL_Z))
        + labels.translate(LABEL_CENTER.0, LABEL_CENTER.1, insert_z(LABEL_PANEL_Z))
        + waste.translate(WASTE_CENTER.0, WASTE_CENTER.1, insert_z(WASTE_TRAY_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_GAUGE_Z / 2.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed media/buffer hold-time stability station:");
    println!("  Station footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!(
        "  Sealed containers:          {BAG_BAY_COUNT} bag nests and {BOTTLE_NEST_COUNT} bottle nests with keyed closed-port staging"
    );
    println!(
        "  Timepoint sampling:         {TIMEPOINT_COUNT} retained timepoint vial wells on {SAMPLE_LOOP_CHANNELS} closed-loop channel rails"
    );
    println!(
        "  Stability evidence:         {THERMOWELL_COUNT} thermowell pockets plus logger cradle and pH/osmolality/conductivity standard lands"
    );
    println!(
        "  Disposition lanes:          release, hold, reject lanes with {LANE_POSITIONS_PER_STATUS} token positions per status"
    );
    println!(
        "  Chain of custody:           {CHAIN_LABEL_COUNT} barcode/QR lands, {RFID_LABEL_COUNT} RFID lands, {TAMPER_SEAL_COUNT} tamper-seal slots"
    );
    println!(
        "  Waste and access:           {WASTE_BOTTLE_COUNT} waste bottle nests, {ABSORBENT_PAD_COUNT} absorbent pad lands, front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_position(
    index: usize,
    cols: usize,
    count: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    let rows = count.div_ceil(cols);
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds closed media/buffer stability station envelope"
        );
    }
    assert!(
        BAG_BAY_X * BAG_BAY_COLS as f64 + 44.0 < BAG_NEST_X,
        "sealed bag bay grid exceeds bag nest width"
    );
    assert!(
        BOTTLE_WELL_D + 10.0 < BOTTLE_WELL_PITCH_X,
        "bottle wells do not leave enough divider material"
    );
    assert!(
        lane_token_span_x() < LANE_PANEL_X - 74.0,
        "release/hold/reject lane tokens exceed lane panel width"
    );
    assert!(
        timepoint_span_x() < LOOP_PANEL_X - 92.0,
        "timepoint sample loop exceeds loop panel width"
    );
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        ("sealed_bag_nests", BAG_CENTER, BAG_NEST_X, BAG_NEST_Y),
        (
            "sealed_bottle_nests",
            BOTTLE_CENTER,
            BOTTLE_RACK_X,
            BOTTLE_RACK_Y,
        ),
        (
            "timepoint_sample_loop",
            LOOP_CENTER,
            LOOP_PANEL_X,
            LOOP_PANEL_Y,
        ),
        (
            "temperature_logger_pocket",
            LOGGER_CENTER,
            LOGGER_BLOCK_X,
            LOGGER_BLOCK_Y,
        ),
        (
            "ph_osmolality_conductivity_standard_lands",
            STANDARD_CENTER,
            STANDARD_PANEL_X,
            STANDARD_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            LANE_CENTER,
            LANE_PANEL_X,
            LANE_PANEL_Y,
        ),
        (
            "chain_of_custody_label_lands",
            LABEL_CENTER,
            LABEL_PANEL_X,
            LABEL_PANEL_Y,
        ),
        ("waste_capture", WASTE_CENTER, WASTE_TRAY_X, WASTE_TRAY_Y),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn lane_token_span_x() -> f64 {
    LANE_TOKEN_X * LANE_NAMES.len() as f64 + 74.0 * (LANE_NAMES.len() as f64 - 1.0)
}

fn timepoint_span_x() -> f64 {
    TIMEPOINT_PITCH_X * (TIMEPOINT_COLS as f64 - 1.0) + TIMEPOINT_VIAL_D
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_media_buffer_hold_time_stability_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_media_buffer_hold_time_stability_station_washdown_recess",
        STATION_X - 116.0,
        STATION_Y - 116.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let front_drain = centered_cylinder(
        "closed_media_buffer_hold_time_stability_station_front_drain",
        DRAIN_PORT_D / 2.0,
        RIM_W + 30.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess - front_drain - insert_sockets() - mount_slots()
        + perimeter_rims()
        + row_dividers()
        + deck_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_media_buffer_hold_time_stability_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_media_buffer_hold_time_stability_station_mount_slots");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 46.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 46.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 46.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 46.0),
        (0.0, -STATION_Y / 2.0 + 46.0),
        (0.0, STATION_Y / 2.0 - 46.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_m6_hole_{i}"),
                3.3,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_mount_slot_{i}"),
                28.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_media_buffer_hold_time_stability_station_left_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_buffer_hold_time_stability_station_right_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_media_buffer_hold_time_stability_station_rear_rim",
        STATION_X - 54.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_media_buffer_hold_time_stability_station_front_low_lip",
        STATION_X - 180.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn row_dividers() -> Part {
    let upper = centered_cube(
        "closed_media_buffer_hold_time_stability_station_container_row_divider",
        STATION_X - 160.0,
        9.0,
        26.0,
    )
    .translate(0.0, 60.0, BASE_Z / 2.0 + 13.0);
    let lower = centered_cube(
        "closed_media_buffer_hold_time_stability_station_disposition_row_divider",
        STATION_X - 160.0,
        9.0,
        24.0,
    )
    .translate(0.0, -205.0, BASE_Z / 2.0 + 12.0);
    let upper_split = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bag_bottle_split",
        9.0,
        286.0,
        26.0,
    )
    .translate(-150.0, 200.0, BASE_Z / 2.0 + 13.0);
    let lower_split = centered_cube(
        "closed_media_buffer_hold_time_stability_station_loop_logger_standards_split",
        9.0,
        250.0,
        24.0,
    )
    .translate(260.0, -78.0, BASE_Z / 2.0 + 12.0);

    upper + lower + upper_split + lower_split
}

fn deck_fiducials() -> Part {
    let mut fiducials =
        Part::empty("closed_media_buffer_hold_time_stability_station_robot_fiducials");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 90.0, STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 90.0, STATION_Y / 2.0 - 86.0),
        (-STATION_X / 2.0 + 90.0, -STATION_Y / 2.0 + 92.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_media_buffer_hold_time_stability_station_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn sealed_bag_nests() -> Part {
    let body = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bag_nest_body",
        BAG_NEST_X,
        BAG_NEST_Y,
        BAG_NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bag_rear_closed_port_fence",
        BAG_NEST_X,
        14.0,
        BAG_NEST_Z + 34.0,
    )
    .translate(0.0, BAG_NEST_Y / 2.0 - 7.0, 17.0);
    let front_retainer = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bag_front_retainer_lip",
        BAG_NEST_X - 28.0,
        12.0,
        22.0,
    )
    .translate(0.0, -BAG_NEST_Y / 2.0 + 9.0, BAG_NEST_Z / 2.0 + 11.0);

    let mut recesses = Part::empty("closed_media_buffer_hold_time_stability_station_bag_recesses");
    let mut port_features =
        Part::empty("closed_media_buffer_hold_time_stability_station_bag_closed_port_features");
    for i in 0..BAG_BAY_COUNT {
        let (x, y) = grid_position(i, BAG_BAY_COLS, BAG_BAY_COUNT, 202.0, 112.0);
        recesses = recesses
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_sealed_bag_recess_{i}"),
                BAG_BAY_X,
                BAG_BAY_Y,
                BAG_BAY_RECESS_Z + 0.4,
            )
            .translate(x, y, BAG_NEST_Z / 2.0 - BAG_BAY_RECESS_Z / 2.0 + 0.2)
            + centered_cube(
                format!(
                    "closed_media_buffer_hold_time_stability_station_sealed_bag_neck_keyway_{i}"
                ),
                42.0,
                24.0,
                BAG_BAY_RECESS_Z + 4.0,
            )
            .translate(x + 58.0, y + BAG_BAY_Y / 2.0 - 5.0, BAG_NEST_Z / 2.0 - 8.0);

        for j in 0..BAG_PORT_COMBS_PER_BAY {
            let px = x - 42.0 + j as f64 * 84.0;
            port_features = port_features
                + centered_cube(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_bag_port_clip_{i}_{j}"
                    ),
                    36.0,
                    12.0,
                    20.0,
                )
                .translate(px, y + BAG_BAY_Y / 2.0 + 18.0, BAG_NEST_Z / 2.0 + 10.0)
                + centered_cylinder(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_bag_septum_guard_{i}_{j}"
                    ),
                    11.0,
                    5.0,
                    28,
                )
                .translate(px, y + BAG_BAY_Y / 2.0 + 39.0, BAG_NEST_Z / 2.0 + 2.5);
        }
    }

    body + rear_fence + front_retainer + port_features - recesses + latch_tabs("bag_nest")
}

fn sealed_bottle_nests() -> Part {
    let body = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bottle_nest_body",
        BOTTLE_RACK_X,
        BOTTLE_RACK_Y,
        BOTTLE_RACK_Z,
    );
    let rear_fence = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bottle_rear_fence",
        BOTTLE_RACK_X,
        14.0,
        BOTTLE_RACK_Z + 40.0,
    )
    .translate(0.0, BOTTLE_RACK_Y / 2.0 - 7.0, 20.0);
    let mut wells = Part::empty("closed_media_buffer_hold_time_stability_station_bottle_wells");
    let mut collars = Part::empty("closed_media_buffer_hold_time_stability_station_bottle_collars");
    for i in 0..BOTTLE_NEST_COUNT {
        let (x, y) = grid_position(
            i,
            BOTTLE_NEST_COLS,
            BOTTLE_NEST_COUNT,
            BOTTLE_WELL_PITCH_X,
            BOTTLE_WELL_PITCH_Y,
        );
        wells = wells
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_bottle_well_{i}"),
                BOTTLE_WELL_D / 2.0,
                BOTTLE_RACK_Z + 8.0,
                48,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_bottle_flat_key_{i}"),
                BOTTLE_WELL_D * 0.72,
                12.0,
                BOTTLE_RACK_Z + 8.0,
            )
            .translate(x, y - BOTTLE_WELL_D / 2.0 + 2.0, 0.0);
        collars = collars
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_bottle_cap_land_{i}"),
                22.0,
                4.0,
                36,
            )
            .translate(x, y + 38.0, BOTTLE_RACK_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_media_buffer_hold_time_stability_station_bottle_lot_label_land_{i}"
                ),
                58.0,
                14.0,
                3.0,
            )
            .translate(x, y - 42.0, BOTTLE_RACK_Z / 2.0 + 1.5);
    }
    let tube_comb = centered_cube(
        "closed_media_buffer_hold_time_stability_station_bottle_closed_tube_comb",
        BOTTLE_RACK_X - 46.0,
        18.0,
        24.0,
    )
    .translate(0.0, -BOTTLE_RACK_Y / 2.0 + 28.0, BOTTLE_RACK_Z / 2.0 + 12.0);

    body + rear_fence + collars + tube_comb - wells + latch_tabs("bottle_nest")
}

fn timepoint_sample_loop() -> Part {
    let panel = centered_cube(
        "closed_media_buffer_hold_time_stability_station_timepoint_loop_panel",
        LOOP_PANEL_X,
        LOOP_PANEL_Y,
        LOOP_PANEL_Z,
    );
    let mut wells = Part::empty("closed_media_buffer_hold_time_stability_station_timepoint_wells");
    let mut raised_features =
        Part::empty("closed_media_buffer_hold_time_stability_station_timepoint_loop_features");
    for i in 0..TIMEPOINT_COUNT {
        let (x, y) = grid_position(
            i,
            TIMEPOINT_COLS,
            TIMEPOINT_COUNT,
            TIMEPOINT_PITCH_X,
            TIMEPOINT_PITCH_Y,
        );
        wells = wells
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_timepoint_vial_{i}"),
                TIMEPOINT_VIAL_D / 2.0,
                LOOP_PANEL_Z + 8.0,
                32,
            )
            .translate(x, y, 0.0);
        raised_features = raised_features
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_timepoint_label_{i}"),
                46.0,
                12.0,
                3.0,
            )
            .translate(x, y - 28.0, LOOP_PANEL_Z / 2.0 + 1.5)
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_timepoint_septum_{i}"),
                9.0,
                5.0,
                28,
            )
            .translate(x, y + 30.0, LOOP_PANEL_Z / 2.0 + 2.5);
    }
    let upper_loop = centered_cube(
        "closed_media_buffer_hold_time_stability_station_upper_sample_loop_rail",
        LOOP_PANEL_X - 60.0,
        10.0,
        14.0,
    )
    .translate(0.0, 71.0, LOOP_PANEL_Z / 2.0 + 7.0);
    let lower_loop = centered_cube(
        "closed_media_buffer_hold_time_stability_station_lower_sample_loop_rail",
        LOOP_PANEL_X - 60.0,
        10.0,
        14.0,
    )
    .translate(0.0, -71.0, LOOP_PANEL_Z / 2.0 + 7.0);
    let left_loop = centered_cube(
        "closed_media_buffer_hold_time_stability_station_left_sample_loop_return",
        10.0,
        152.0,
        14.0,
    )
    .translate(-LOOP_PANEL_X / 2.0 + 38.0, 0.0, LOOP_PANEL_Z / 2.0 + 7.0);
    let right_loop = centered_cube(
        "closed_media_buffer_hold_time_stability_station_right_sample_loop_return",
        10.0,
        152.0,
        14.0,
    )
    .translate(LOOP_PANEL_X / 2.0 - 38.0, 0.0, LOOP_PANEL_Z / 2.0 + 7.0);
    let inlet_outlet = centered_cube(
        "closed_media_buffer_hold_time_stability_station_sample_loop_inlet_outlet_bulkhead",
        70.0,
        28.0,
        32.0,
    )
    .translate(-LOOP_PANEL_X / 2.0 + 50.0, 0.0, LOOP_PANEL_Z / 2.0 + 16.0);

    panel + raised_features + upper_loop + lower_loop + left_loop + right_loop + inlet_outlet
        - wells
        + latch_tabs("timepoint_loop")
}

fn temperature_logger_pocket() -> Part {
    let body = centered_cube(
        "closed_media_buffer_hold_time_stability_station_temperature_logger_body",
        LOGGER_BLOCK_X,
        LOGGER_BLOCK_Y,
        LOGGER_BLOCK_Z,
    );
    let logger_slot = centered_cube(
        "closed_media_buffer_hold_time_stability_station_temperature_logger_slot",
        LOGGER_SLOT_X,
        LOGGER_SLOT_Y,
        LOGGER_SLOT_Z + 0.4,
    )
    .translate(0.0, 14.0, LOGGER_BLOCK_Z / 2.0 - LOGGER_SLOT_Z / 2.0 + 0.2);
    let cable_exit = centered_cube(
        "closed_media_buffer_hold_time_stability_station_temperature_logger_cable_exit",
        32.0,
        LOGGER_BLOCK_Y + 8.0,
        16.0,
    )
    .translate(LOGGER_SLOT_X / 2.0 - 16.0, 0.0, 6.0);

    let mut thermowells =
        Part::empty("closed_media_buffer_hold_time_stability_station_thermowell_pockets");
    for i in 0..THERMOWELL_COUNT {
        let x = centered_index(i, THERMOWELL_COUNT, THERMOWELL_PITCH);
        thermowells = thermowells
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_thermowell_{i}"),
                4.0,
                LOGGER_BLOCK_Y + 8.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -8.0, 4.0)
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_thermowell_label_{i}"),
                28.0,
                10.0,
                3.0,
            )
            .translate(x, -LOGGER_BLOCK_Y / 2.0 + 24.0, LOGGER_BLOCK_Z / 2.0 + 1.5);
    }
    let latch_bridge = centered_cube(
        "closed_media_buffer_hold_time_stability_station_logger_retention_bridge",
        LOGGER_SLOT_X + 34.0,
        12.0,
        16.0,
    )
    .translate(0.0, LOGGER_SLOT_Y / 2.0 + 34.0, LOGGER_BLOCK_Z / 2.0 + 8.0);

    body + thermowells + latch_bridge - logger_slot - cable_exit + latch_tabs("temperature_logger")
}

fn ph_osmolality_conductivity_standard_lands() -> Part {
    let panel = centered_cube(
        "closed_media_buffer_hold_time_stability_station_standard_lands_panel",
        STANDARD_PANEL_X,
        STANDARD_PANEL_Y,
        STANDARD_PANEL_Z,
    );
    let mut wells = Part::empty("closed_media_buffer_hold_time_stability_station_standard_wells");
    let mut lands =
        Part::empty("closed_media_buffer_hold_time_stability_station_standard_label_lands");
    for (standard_index, standard) in STANDARD_TYPES.iter().enumerate() {
        let x = centered_index(standard_index, STANDARD_TYPES.len(), 94.0);
        let section_backer = centered_cube(
            format!("closed_media_buffer_hold_time_stability_station_{standard}_standard_section"),
            78.0,
            STANDARD_PANEL_Y - 30.0,
            5.0,
        )
        .translate(x, 0.0, STANDARD_PANEL_Z / 2.0 + 2.5);
        lands = lands + section_backer;
        for i in 0..STANDARD_REPLICATES {
            let y = centered_index(i, STANDARD_REPLICATES, 42.0);
            wells = wells
                + centered_cylinder(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_{standard}_standard_well_{i}"
                    ),
                    STANDARD_WELL_D / 2.0,
                    STANDARD_PANEL_Z + 8.0,
                    32,
                )
                .translate(x, y, 0.0);
            lands = lands
                + centered_cube(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_{standard}_lot_land_{i}"
                    ),
                    50.0,
                    11.0,
                    3.0,
                )
                .translate(x, y + 20.0, STANDARD_PANEL_Z / 2.0 + 1.5);
        }
    }
    let cert_land = centered_cube(
        "closed_media_buffer_hold_time_stability_station_standard_certificate_land",
        STANDARD_PANEL_X - 44.0,
        22.0,
        3.0,
    )
    .translate(
        0.0,
        STANDARD_PANEL_Y / 2.0 - 22.0,
        STANDARD_PANEL_Z / 2.0 + 1.5,
    );

    panel + lands + cert_land - wells + latch_tabs("standard_lands")
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_media_buffer_hold_time_stability_station_release_hold_reject_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut lane_features =
        Part::empty("closed_media_buffer_hold_time_stability_station_disposition_lane_features");
    let mut pockets =
        Part::empty("closed_media_buffer_hold_time_stability_station_disposition_token_pockets");
    for (lane_index, lane_name) in LANE_NAMES.iter().enumerate() {
        let lane_x = centered_index(lane_index, LANE_NAMES.len(), 128.0);
        lane_features = lane_features
            + centered_cube(
                format!(
                    "closed_media_buffer_hold_time_stability_station_{lane_name}_lane_raised_key"
                ),
                102.0,
                20.0,
                10.0,
            )
            .translate(lane_x, LANE_PANEL_Y / 2.0 - 22.0, LANE_PANEL_Z / 2.0 + 5.0);
        for i in 0..LANE_POSITIONS_PER_STATUS {
            let y = centered_index(i, LANE_POSITIONS_PER_STATUS, 30.0) - 14.0;
            pockets = pockets
                + centered_cube(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_{lane_name}_token_pocket_{i}"
                    ),
                    LANE_TOKEN_X,
                    LANE_TOKEN_Y,
                    13.0,
                )
                .translate(lane_x, y, LANE_PANEL_Z / 2.0 - 6.5);
            lane_features = lane_features
                + centered_cube(
                    format!(
                        "closed_media_buffer_hold_time_stability_station_{lane_name}_barcode_land_{i}"
                    ),
                    46.0,
                    9.0,
                    3.0,
                )
                .translate(lane_x, y - 24.0, LANE_PANEL_Z / 2.0 + 1.5);
        }
    }
    let center_dividers = lane_divider_set();

    panel + lane_features + center_dividers - pockets + latch_tabs("disposition_lanes")
}

fn lane_divider_set() -> Part {
    let mut dividers = Part::empty("closed_media_buffer_hold_time_stability_station_lane_dividers");
    for (i, x) in [-64.0, 64.0].iter().enumerate() {
        dividers = dividers
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_lane_divider_{i}"),
                8.0,
                LANE_PANEL_Y - 24.0,
                LANE_PANEL_Z + 24.0,
            )
            .translate(*x, 0.0, 12.0);
    }
    dividers
}

fn chain_of_custody_label_lands() -> Part {
    let panel = centered_cube(
        "closed_media_buffer_hold_time_stability_station_chain_label_panel",
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    );
    let mut lands =
        Part::empty("closed_media_buffer_hold_time_stability_station_chain_label_lands");
    for i in 0..CHAIN_LABEL_COUNT {
        let (x, y) = grid_position(i, CHAIN_LABEL_COLS, CHAIN_LABEL_COUNT, 66.0, 42.0);
        lands = lands
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_chain_barcode_land_{i}"),
                50.0,
                18.0,
                3.0,
            )
            .translate(x, y, LABEL_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..RFID_LABEL_COUNT {
        let x = centered_index(i, RFID_LABEL_COUNT, 62.0);
        lands = lands
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_rfid_label_land_{i}"),
                42.0,
                24.0,
                3.0,
            )
            .translate(x, LABEL_PANEL_Y / 2.0 - 24.0, LABEL_PANEL_Z / 2.0 + 1.5);
    }
    let mut seal_slots =
        Part::empty("closed_media_buffer_hold_time_stability_station_tamper_seal_slots");
    for i in 0..TAMPER_SEAL_COUNT {
        let x = centered_index(i % 4, 4, 62.0);
        let y = if i < 4 {
            -LABEL_PANEL_Y / 2.0 + 22.0
        } else {
            LABEL_PANEL_Y / 2.0 - 58.0
        };
        seal_slots = seal_slots
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_tamper_slot_{i}"),
                34.0,
                7.0,
                LABEL_PANEL_Z + 4.0,
            )
            .translate(x, y, 0.0)
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_tamper_wire_bore_{i}"),
                1.8,
                42.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 2.0);
    }

    panel + lands - seal_slots + latch_tabs("chain_labels")
}

fn waste_capture() -> Part {
    let tray = centered_cube(
        "closed_media_buffer_hold_time_stability_station_waste_tray_body",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let sump = centered_cube(
        "closed_media_buffer_hold_time_stability_station_waste_sump",
        WASTE_TRAY_X - 38.0,
        WASTE_TRAY_Y - 38.0,
        12.0,
    )
    .translate(0.0, 0.0, WASTE_TRAY_Z / 2.0 - 6.0);
    let drain = centered_cylinder(
        "closed_media_buffer_hold_time_stability_station_waste_drain",
        DRAIN_PORT_D / 2.0,
        WASTE_TRAY_Y + 8.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_TRAY_X / 2.0 - 46.0, 0.0, WASTE_TRAY_Z / 2.0 - 10.0);

    let mut waste_wells =
        Part::empty("closed_media_buffer_hold_time_stability_station_waste_bottle_wells");
    let mut pads =
        Part::empty("closed_media_buffer_hold_time_stability_station_absorbent_pad_lands");
    for i in 0..WASTE_BOTTLE_COUNT {
        let x = centered_index(i, WASTE_BOTTLE_COUNT, 84.0) - 60.0;
        waste_wells = waste_wells
            + centered_cylinder(
                format!("closed_media_buffer_hold_time_stability_station_waste_bottle_well_{i}"),
                WASTE_BOTTLE_D / 2.0,
                WASTE_TRAY_Z + 8.0,
                42,
            )
            .translate(x, -18.0, 0.0);
    }
    for i in 0..ABSORBENT_PAD_COUNT {
        let (x, y) = grid_position(i, 3, ABSORBENT_PAD_COUNT, 54.0, 40.0);
        pads = pads
            + centered_cube(
                format!("closed_media_buffer_hold_time_stability_station_absorbent_pad_{i}"),
                42.0,
                26.0,
                3.0,
            )
            .translate(x + 72.0, y, WASTE_TRAY_Z / 2.0 + 1.5);
    }
    let rejected_tip_gate = centered_cube(
        "closed_media_buffer_hold_time_stability_station_rejected_sample_tip_gate",
        82.0,
        12.0,
        24.0,
    )
    .translate(
        WASTE_TRAY_X / 2.0 - 58.0,
        -WASTE_TRAY_Y / 2.0 + 18.0,
        WASTE_TRAY_Z / 2.0 + 12.0,
    );

    tray + pads + rejected_tip_gate - sump - drain - waste_wells + latch_tabs("waste_capture")
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_frame(
        "closed_media_buffer_hold_time_stability_station_front_robot_keepout",
        STATION_X - 170.0,
        FRONT_ROBOT_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0, 0.0);
    let rear_service = keepout_frame(
        "closed_media_buffer_hold_time_stability_station_rear_service_keepout",
        STATION_X - 210.0,
        REAR_SERVICE_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 0.0);
    let left_service = keepout_frame(
        "closed_media_buffer_hold_time_stability_station_left_service_keepout",
        LEFT_SERVICE_CLEARANCE,
        STATION_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_RAIL,
    )
    .translate(-STATION_X / 2.0 - LEFT_SERVICE_CLEARANCE / 2.0, 0.0, 0.0);
    let right_service = keepout_frame(
        "closed_media_buffer_hold_time_stability_station_right_service_keepout",
        RIGHT_SERVICE_CLEARANCE,
        STATION_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_RAIL,
    )
    .translate(STATION_X / 2.0 + RIGHT_SERVICE_CLEARANCE / 2.0, 0.0, 0.0);
    let top_logger_swing = centered_cube(
        "closed_media_buffer_hold_time_stability_station_top_logger_service_clearance",
        LOGGER_BLOCK_X + 120.0,
        LOGGER_BLOCK_Y + 80.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(LOGGER_CENTER.0, LOGGER_CENTER.1, TOP_LOGGER_CLEARANCE);

    front_robot + rear_service + left_service + right_service + top_logger_swing
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let front = centered_cube(format!("{name}_front_rail"), x, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail, z).translate(
        0.0,
        y / 2.0 - rail / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left_rail"), rail, y, z).translate(
        -x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, y, z).translate(
        x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );

    front + rear + left + right
}

fn latch_tabs(prefix: &str) -> Part {
    let left = centered_cube(
        format!("closed_media_buffer_hold_time_stability_station_{prefix}_left_latch_tab"),
        42.0,
        14.0,
        10.0,
    )
    .translate(-34.0, 0.0, 5.0);
    let right = centered_cube(
        format!("closed_media_buffer_hold_time_stability_station_{prefix}_right_latch_tab"),
        42.0,
        14.0,
        10.0,
    )
    .translate(34.0, 0.0, 5.0);

    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 12.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_cut"), 3.0, 4.0, 24)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_media_buffer_hold_time_stability_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "sealed_bag_nests",
            "sealed_bottle_nests",
            "timepoint_sample_loop",
            "temperature_logger_pocket",
            "ph_standard_lands",
            "osmolality_standard_lands",
            "conductivity_standard_lands",
            "release_lane",
            "hold_lane",
            "reject_lane",
            "chain_of_custody_labels",
            "waste_capture",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn insert_layout_fits_station_envelope() {
        assert_layout();
        assert!(insert_specs()
            .iter()
            .all(|(_, pos, width, depth)| fits_on_station(*pos, *width, *depth)));
    }

    #[test]
    fn sealed_container_capacity_supports_media_and_buffer_holds() {
        assert_eq!(BAG_BAY_COUNT, 4);
        assert_eq!(BOTTLE_NEST_COUNT, 6);
        assert_eq!(BAG_PORT_COMBS_PER_BAY, 2);
        assert!(BAG_BAY_X * BAG_BAY_COLS as f64 + 44.0 < BAG_NEST_X);
        assert!(BOTTLE_WELL_D + 10.0 < BOTTLE_WELL_PITCH_X);
        assert!(BOTTLE_WELL_D + 10.0 < BOTTLE_WELL_PITCH_Y);
    }

    #[test]
    fn timepoint_standards_and_custody_counts_are_balanced() {
        assert_eq!(TIMEPOINT_COUNT, 8);
        assert_eq!(STANDARD_TYPES, ["ph", "osmolality", "conductivity"]);
        assert_eq!(STANDARD_TYPES.len() * STANDARD_REPLICATES, 12);
        assert_eq!(CHAIN_LABEL_COUNT, 12);
        assert!(CHAIN_LABEL_COUNT >= TIMEPOINT_COUNT);
        assert_eq!(LANE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(LANE_POSITIONS_PER_STATUS * LANE_NAMES.len(), 12);
    }

    #[test]
    fn logger_waste_and_keepouts_are_serviceable() {
        assert_eq!(THERMOWELL_COUNT, 6);
        assert_eq!(WASTE_BOTTLE_COUNT, 2);
        assert_eq!(ABSORBENT_PAD_COUNT, 6);
        assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 250.0);
        assert!(LEFT_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 250.0);
        assert!(TOP_LOGGER_CLEARANCE >= 300.0);
    }
}
