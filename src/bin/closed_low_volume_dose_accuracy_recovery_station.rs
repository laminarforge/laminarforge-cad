use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed low-volume dose accuracy and recovery station.
//
// Intent:
// - Validate tiny media, additive, and cell-suspension dose accuracy before
//   automated chip work.
// - Provide closed, traceable interfaces for micro-dose collection, balance
//   pad placement, capillary witness checks, dead-volume recovery, evaporation
//   control, standards, status disposition, camera evidence, and waste routing.
//
// This is packaging/interface CAD only. It does not define metrology limits,
// cell-handling SOPs, balance electronics, optical analysis, or wetted
// disposable internals.

const PREFIX: &str = "closed_low_volume_dose_accuracy_recovery_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_low_volume_dose_accuracy_recovery_station_base_tray.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_micro_dose_collection_nests.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_balance_load_cell_pad_placeholders.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_capillary_witness_channels.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_dead_volume_recovery_wells.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_evaporation_cover.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_barcode_run_record_lands.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_high_low_standard_lanes.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_flush_waste_route.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_pass_hold_reject_status_lanes.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_evidence_camera_bridge.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_robot_service_keepouts.stl",
    "output/closed_low_volume_dose_accuracy_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "micro_dose_collection_nests",
    "balance_load_cell_pad_placeholders",
    "capillary_witness_channels",
    "dead_volume_recovery_wells",
    "evaporation_cover",
    "barcode_run_record_lands",
    "high_low_standard_lanes",
    "flush_waste_route",
    "pass_hold_reject_status_lanes",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "status_custody_separation",
    "assembly_export",
];

const STATION_X: f64 = 980.0;
const STATION_Y: f64 = 680.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const FIT_MARGIN: f64 = 20.0;

const MICRO_NEST_CENTER: (f64, f64) = (-210.0, 132.0);
const MICRO_NEST_X: f64 = 340.0;
const MICRO_NEST_Y: f64 = 154.0;
const MICRO_NEST_Z: f64 = 34.0;
const MICRO_NEST_COLUMNS: usize = 6;
const MICRO_NEST_ROWS: usize = 2;
const MICRO_NEST_COUNT: usize = MICRO_NEST_COLUMNS * MICRO_NEST_ROWS;
const MICRO_NEST_PITCH_X: f64 = 48.0;
const MICRO_NEST_PITCH_Y: f64 = 56.0;
const MICRO_COLLECTION_WELL_D: f64 = 18.0;

const BALANCE_CENTER: (f64, f64) = (185.0, 132.0);
const BALANCE_RAIL_X: f64 = 330.0;
const BALANCE_RAIL_Y: f64 = 154.0;
const BALANCE_RAIL_Z: f64 = 26.0;
const BALANCE_PAD_COUNT: usize = 6;
const BALANCE_PAD_PITCH_X: f64 = 52.0;
const BALANCE_PAD_X: f64 = 42.0;
const BALANCE_PAD_Y: f64 = 78.0;
const BALANCE_PAD_Z: f64 = 8.0;

const CAPILLARY_CENTER: (f64, f64) = (-210.0, -42.0);
const CAPILLARY_PLATE_X: f64 = 340.0;
const CAPILLARY_PLATE_Y: f64 = 112.0;
const CAPILLARY_PLATE_Z: f64 = 14.0;
const CAPILLARY_CHANNEL_COUNT: usize = MICRO_NEST_COUNT;
const CAPILLARY_CHANNEL_PITCH_Y: f64 = 7.4;
const CAPILLARY_CHANNEL_X: f64 = 286.0;
const CAPILLARY_CHANNEL_W: f64 = 2.8;

const RECOVERY_CENTER: (f64, f64) = (145.0, -42.0);
const RECOVERY_BLOCK_X: f64 = 260.0;
const RECOVERY_BLOCK_Y: f64 = 112.0;
const RECOVERY_BLOCK_Z: f64 = 38.0;
const RECOVERY_WELL_COUNT: usize = 8;
const RECOVERY_WELL_PITCH_X: f64 = 28.0;
const RECOVERY_WELL_D: f64 = 15.0;

const COVER_CENTER: (f64, f64) = (0.0, 76.0);
const COVER_X: f64 = 792.0;
const COVER_Y: f64 = 292.0;
const COVER_RAIL_W: f64 = 18.0;
const COVER_Z: f64 = 10.0;
const COVER_CLEAR_Z: f64 = 92.0;
const COVER_LATCH_COUNT: usize = 4;

const TRACE_CENTER: (f64, f64) = (-235.0, -248.0);
const TRACE_PLATE_X: f64 = 300.0;
const TRACE_PLATE_Y: f64 = 86.0;
const TRACE_PLATE_Z: f64 = 8.0;
const BARCODE_LAND_COUNT: usize = 10;
const RUN_RECORD_LAND_COUNT: usize = 4;

const STANDARD_HIGH_CENTER: (f64, f64) = (45.0, -238.0);
const STANDARD_LOW_CENTER: (f64, f64) = (265.0, -238.0);
const STANDARD_LANE_X: f64 = 190.0;
const STANDARD_LANE_Y: f64 = 92.0;
const STANDARD_LANE_Z: f64 = 30.0;
const STANDARD_SLOT_COUNT: usize = 5;
const STANDARD_SLOT_PITCH_X: f64 = 30.0;
const STANDARD_LANE_CENTER_GAP: f64 = STANDARD_LOW_CENTER.0 - STANDARD_HIGH_CENTER.0;

const ROUTE_CENTER: (f64, f64) = (430.0, 0.0);
const ROUTE_X: f64 = 72.0;
const ROUTE_Y: f64 = 544.0;
const ROUTE_Z: f64 = 34.0;
const FLUSH_PORT_COUNT: usize = 6;
const WASTE_ROUTE_COUNT: usize = 6;
const ROUTE_PORT_PITCH_Y: f64 = 72.0;

const STATUS_CENTER_X: f64 = -425.0;
const STATUS_PASS_Y: f64 = 170.0;
const STATUS_HOLD_Y: f64 = 22.0;
const STATUS_REJECT_Y: f64 = -126.0;
const STATUS_LANE_X: f64 = 72.0;
const STATUS_LANE_Y: f64 = 88.0;
const STATUS_LANE_Z: f64 = 26.0;
const STATUS_LANE_COUNT: usize = 3;
const STATUS_CUSTODY_MIN_GAP: f64 = 300.0;

const CAMERA_CENTER: (f64, f64) = (0.0, 254.0);
const CAMERA_BRIDGE_X: f64 = 832.0;
const CAMERA_BRIDGE_Y: f64 = 36.0;
const CAMERA_BRIDGE_Z: f64 = 146.0;
const CAMERA_CROSSBAR_Z: f64 = 24.0;
const CAMERA_COLUMN_X: f64 = 26.0;
const CAMERA_WINDOW_X: f64 = 112.0;
const CAMERA_WINDOW_Y: f64 = 24.0;

const KEEP_OUT_ZONE_COUNT: usize = 5;
const ROBOT_KEEP_OUT_X: f64 = 870.0;
const ROBOT_KEEP_OUT_Y: f64 = 144.0;
const ROBOT_KEEP_OUT_Z: f64 = 92.0;
const FRONT_SERVICE_CLEARANCE: f64 = 420.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 180.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(&base, OUTPUTS[0]);

    let nests = micro_dose_collection_nests();
    export(&nests, OUTPUTS[1]);

    let balance = balance_load_cell_pad_placeholders();
    export(&balance, OUTPUTS[2]);

    let capillary = capillary_witness_channels();
    export(&capillary, OUTPUTS[3]);

    let recovery = dead_volume_recovery_wells();
    export(&recovery, OUTPUTS[4]);

    let cover = evaporation_cover();
    export(&cover, OUTPUTS[5]);

    let trace = barcode_run_record_lands();
    export(&trace, OUTPUTS[6]);

    let standards = high_low_standard_lanes();
    export(&standards, OUTPUTS[7]);

    let route = flush_waste_route();
    export(&route, OUTPUTS[8]);

    let status = pass_hold_reject_status_lanes();
    export(&status, OUTPUTS[9]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + nests
        + balance
        + capillary
        + recovery
        + cover
        + trace
        + standards
        + route
        + status
        + camera
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed low-volume dose accuracy and recovery station: {:.0}mm x {:.0}mm closed tray, {} micro-dose collection nests, {} balance/load-cell pads, {} capillary witness channels, {} dead-volume recovery wells, and high/low standards lanes.",
        STATION_X,
        STATION_Y,
        MICRO_NEST_COUNT,
        BALANCE_PAD_COUNT,
        CAPILLARY_CHANNEL_COUNT,
        RECOVERY_WELL_COUNT
    );
    println!(
        "Traceability and disposition: {} barcode/run-record lands, {} flush ports, {} waste routes, {} pass/hold/reject status lanes, evidence camera bridge, {} robot/service keepouts, {:.0}mm standard-lane gap, {:.0}mm front service clearance, {:.0}mm right service clearance, and {} required feature groups.",
        BARCODE_LAND_COUNT + RUN_RECORD_LAND_COUNT,
        FLUSH_PORT_COUNT,
        WASTE_ROUTE_COUNT,
        STATUS_LANE_COUNT,
        KEEP_OUT_ZONE_COUNT,
        STANDARD_LANE_CENTER_GAP,
        FRONT_SERVICE_CLEARANCE,
        RIGHT_SERVICE_CLEARANCE,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_tray() -> Part {
    let deck = centered_cube(name("base_deck"), STATION_X, STATION_Y, BASE_Z);
    let basin = centered_cube(
        name("low_volume_closed_leak_capture_recess"),
        STATION_X - 104.0,
        STATION_Y - 92.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(name("front_flush_waste_drain_bore"), 5.0, 60.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(ROUTE_CENTER.0, -(STATION_Y / 2.0 - 18.0), 0.0);

    deck - basin - drain + tray_rims() + mounting_and_datum_features() + zone_separator_ribs()
}

fn tray_rims() -> Part {
    let left = centered_cube(name("left_closed_tray_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(name("right_closed_tray_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("rear_closed_tray_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        name("front_low_access_lip_with_evaporation_cover_seat"),
        STATION_X - 126.0,
        12.0,
        24.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 18.0), BASE_Z / 2.0 + 12.0);

    left + right + rear + front
}

fn mounting_and_datum_features() -> Part {
    let mut features = Part::empty(name("mounting_and_robot_datum_features"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 64.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 64.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 64.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        let bore = centered_cylinder(name(&format!("m6_mount_bore_{i}")), 3.4, BASE_Z + 4.0, 24)
            .translate(*x, *y, 0.0);
        let fiducial = centered_cylinder(name(&format!("robot_datum_fiducial_{i}")), 12.0, 3.0, 32)
            .translate(*x, *y, BASE_Z / 2.0 + 1.5)
            - centered_cylinder(name(&format!("robot_datum_center_mark_{i}")), 2.2, 5.0, 20)
                .translate(*x, *y, BASE_Z / 2.0 + 1.5);
        features = features - bore + fiducial;
    }
    features
}

fn zone_separator_ribs() -> Part {
    let status_wall = centered_cube(
        name("status_to_custody_physical_separation_wall"),
        10.0,
        STATION_Y - 128.0,
        44.0,
    )
    .translate(-390.0, 0.0, BASE_Z / 2.0 + 22.0);
    let waste_wall = centered_cube(
        name("waste_route_to_measurement_zone_splash_wall"),
        10.0,
        STATION_Y - 138.0,
        40.0,
    )
    .translate(382.0, 0.0, BASE_Z / 2.0 + 20.0);
    let custody_wall = centered_cube(
        name("high_low_standard_custody_lane_divider"),
        14.0,
        STANDARD_LANE_Y + 22.0,
        46.0,
    )
    .translate(
        (STANDARD_HIGH_CENTER.0 + STANDARD_LOW_CENTER.0) / 2.0,
        STANDARD_HIGH_CENTER.1,
        BASE_Z / 2.0 + 23.0,
    );

    status_wall + waste_wall + custody_wall
}

fn micro_dose_collection_nests() -> Part {
    let body = centered_cube(
        name("micro_dose_collection_nest_block"),
        MICRO_NEST_X,
        MICRO_NEST_Y,
        MICRO_NEST_Z,
    )
    .translate(
        MICRO_NEST_CENTER.0,
        MICRO_NEST_CENTER.1,
        stage_z(MICRO_NEST_Z),
    );
    let mut cuts = Part::empty(name("micro_dose_collection_nest_well_cuts"));
    let mut rims = Part::empty(name("micro_dose_collection_nest_rims_and_approach_marks"));

    for row in 0..MICRO_NEST_ROWS {
        for col in 0..MICRO_NEST_COLUMNS {
            let index = row * MICRO_NEST_COLUMNS + col;
            let x = MICRO_NEST_CENTER.0 + lane_offset(col, MICRO_NEST_COLUMNS, MICRO_NEST_PITCH_X);
            let y = MICRO_NEST_CENTER.1 + lane_offset(row, MICRO_NEST_ROWS, MICRO_NEST_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("micro_dose_collection_well_cut_{index}")),
                    MICRO_COLLECTION_WELL_D / 2.0,
                    MICRO_NEST_Z + 8.0,
                    32,
                )
                .translate(x, y, stage_z(MICRO_NEST_Z))
                + centered_cube(
                    name(&format!("pipette_tip_approach_relief_{index}")),
                    9.0,
                    34.0,
                    MICRO_NEST_Z + 8.0,
                )
                .translate(x, y - 18.0, stage_z(MICRO_NEST_Z));
            rims = rims
                + centered_cylinder(
                    name(&format!("raised_micro_collection_rim_{index}")),
                    MICRO_COLLECTION_WELL_D / 2.0 + 4.0,
                    4.0,
                    32,
                )
                .translate(x, y, BASE_Z / 2.0 + MICRO_NEST_Z + 2.0)
                - centered_cylinder(
                    name(&format!("raised_micro_collection_rim_opening_{index}")),
                    MICRO_COLLECTION_WELL_D / 2.0,
                    5.0,
                    32,
                )
                .translate(x, y, BASE_Z / 2.0 + MICRO_NEST_Z + 2.0);
        }
    }

    body - cuts + rims + nest_row_index_tabs()
}

fn nest_row_index_tabs() -> Part {
    let mut tabs = Part::empty(name("micro_dose_row_index_tabs"));
    for row in 0..MICRO_NEST_ROWS {
        let y = MICRO_NEST_CENTER.1 + lane_offset(row, MICRO_NEST_ROWS, MICRO_NEST_PITCH_Y);
        tabs = tabs
            + centered_cube(
                name(&format!("micro_dose_row_index_land_{row}")),
                22.0,
                34.0,
                5.0,
            )
            .translate(
                MICRO_NEST_CENTER.0 - MICRO_NEST_X / 2.0 + 20.0,
                y,
                BASE_Z / 2.0 + MICRO_NEST_Z + 2.5,
            );
    }
    tabs
}

fn balance_load_cell_pad_placeholders() -> Part {
    let rail = centered_cube(
        name("balance_load_cell_placeholder_rail"),
        BALANCE_RAIL_X,
        BALANCE_RAIL_Y,
        BALANCE_RAIL_Z,
    )
    .translate(BALANCE_CENTER.0, BALANCE_CENTER.1, stage_z(BALANCE_RAIL_Z));
    let mut additions = Part::empty(name("balance_pad_lands"));
    let mut cuts = Part::empty(name("balance_load_cell_placeholder_cuts"));

    for lane in 0..BALANCE_PAD_COUNT {
        let x = BALANCE_CENTER.0 + lane_offset(lane, BALANCE_PAD_COUNT, BALANCE_PAD_PITCH_X);
        let y = BALANCE_CENTER.1;
        additions = additions
            + centered_cube(
                name(&format!("removable_balance_pan_land_{lane}")),
                BALANCE_PAD_X,
                BALANCE_PAD_Y,
                BALANCE_PAD_Z,
            )
            .translate(x, y, BASE_Z / 2.0 + BALANCE_RAIL_Z + BALANCE_PAD_Z / 2.0)
            + centered_cube(
                name(&format!("balance_pad_status_tick_land_{lane}")),
                BALANCE_PAD_X - 8.0,
                5.0,
                5.0,
            )
            .translate(
                x,
                y + BALANCE_PAD_Y / 2.0 + 8.0,
                BASE_Z / 2.0 + BALANCE_RAIL_Z + 2.5,
            );
        cuts = cuts
            + centered_cube(
                name(&format!("load_cell_body_placeholder_pocket_{lane}")),
                BALANCE_PAD_X - 10.0,
                BALANCE_PAD_Y - 22.0,
                12.0,
            )
            .translate(x, y, BASE_Z / 2.0 + BALANCE_RAIL_Z - 3.0)
            + centered_cube(
                name(&format!("load_cell_cable_relief_channel_{lane}")),
                8.0,
                BALANCE_PAD_Y + 34.0,
                8.0,
            )
            .translate(x, y - 18.0, BASE_Z / 2.0 + BALANCE_RAIL_Z - 2.0);
    }

    rail + additions - cuts
}

fn capillary_witness_channels() -> Part {
    let plate = centered_cube(
        name("capillary_witness_channel_plate"),
        CAPILLARY_PLATE_X,
        CAPILLARY_PLATE_Y,
        CAPILLARY_PLATE_Z,
    )
    .translate(
        CAPILLARY_CENTER.0,
        CAPILLARY_CENTER.1,
        stage_z(CAPILLARY_PLATE_Z),
    );
    let mut channels = Part::empty(name("capillary_witness_channel_grooves"));
    let mut ticks = Part::empty(name("capillary_witness_read_ticks"));

    for channel in 0..CAPILLARY_CHANNEL_COUNT {
        let y = CAPILLARY_CENTER.1
            + lane_offset(channel, CAPILLARY_CHANNEL_COUNT, CAPILLARY_CHANNEL_PITCH_Y);
        channels = channels
            + centered_cube(
                name(&format!("capillary_witness_channel_groove_{channel}")),
                CAPILLARY_CHANNEL_X,
                CAPILLARY_CHANNEL_W,
                CAPILLARY_PLATE_Z + 4.0,
            )
            .translate(CAPILLARY_CENTER.0, y, stage_z(CAPILLARY_PLATE_Z));
        for tick in 0..4 {
            let x = CAPILLARY_CENTER.0 - 120.0 + tick as f64 * 80.0;
            ticks = ticks
                + centered_cube(
                    name(&format!("capillary_witness_tick_{channel}_{tick}")),
                    3.0,
                    CAPILLARY_CHANNEL_W + 4.0,
                    4.0,
                )
                .translate(x, y, BASE_Z / 2.0 + CAPILLARY_PLATE_Z + 2.0);
        }
    }

    let inlet_land = centered_cube(
        name("capillary_witness_common_inlet_land"),
        26.0,
        CAPILLARY_PLATE_Y - 22.0,
        5.0,
    )
    .translate(
        CAPILLARY_CENTER.0 - CAPILLARY_PLATE_X / 2.0 + 26.0,
        CAPILLARY_CENTER.1,
        BASE_Z / 2.0 + CAPILLARY_PLATE_Z + 2.5,
    );
    let outlet_land = centered_cube(
        name("capillary_witness_common_outlet_land"),
        26.0,
        CAPILLARY_PLATE_Y - 22.0,
        5.0,
    )
    .translate(
        CAPILLARY_CENTER.0 + CAPILLARY_PLATE_X / 2.0 - 26.0,
        CAPILLARY_CENTER.1,
        BASE_Z / 2.0 + CAPILLARY_PLATE_Z + 2.5,
    );

    plate - channels + ticks + inlet_land + outlet_land
}

fn dead_volume_recovery_wells() -> Part {
    let block = centered_cube(
        name("dead_volume_recovery_well_block"),
        RECOVERY_BLOCK_X,
        RECOVERY_BLOCK_Y,
        RECOVERY_BLOCK_Z,
    )
    .translate(
        RECOVERY_CENTER.0,
        RECOVERY_CENTER.1,
        stage_z(RECOVERY_BLOCK_Z),
    );
    let mut cuts = Part::empty(name("dead_volume_recovery_well_cuts"));
    let mut ledges = Part::empty(name("dead_volume_recovery_transfer_ledges"));

    for well in 0..RECOVERY_WELL_COUNT {
        let x = RECOVERY_CENTER.0 + lane_offset(well, RECOVERY_WELL_COUNT, RECOVERY_WELL_PITCH_X);
        let y = RECOVERY_CENTER.1 + if well % 2 == 0 { -20.0 } else { 20.0 };
        cuts = cuts
            + centered_cylinder(
                name(&format!("dead_volume_recovery_well_cut_{well}")),
                RECOVERY_WELL_D / 2.0,
                RECOVERY_BLOCK_Z + 8.0,
                32,
            )
            .translate(x, y, stage_z(RECOVERY_BLOCK_Z))
            + centered_cube(
                name(&format!("dead_volume_recovery_return_notch_{well}")),
                8.0,
                36.0,
                RECOVERY_BLOCK_Z + 8.0,
            )
            .translate(x, y - 18.0, stage_z(RECOVERY_BLOCK_Z));
        ledges = ledges
            + centered_cube(
                name(&format!("dead_volume_recovery_label_land_{well}")),
                22.0,
                9.0,
                5.0,
            )
            .translate(x, y + 24.0, BASE_Z / 2.0 + RECOVERY_BLOCK_Z + 2.5);
    }

    let sump = centered_cube(
        name("dead_volume_recovery_sump_gutter"),
        RECOVERY_BLOCK_X - 32.0,
        8.0,
        7.0,
    )
    .translate(
        RECOVERY_CENTER.0,
        RECOVERY_CENTER.1 - RECOVERY_BLOCK_Y / 2.0 + 14.0,
        BASE_Z / 2.0 + RECOVERY_BLOCK_Z - 2.0,
    );

    block - cuts - sump + ledges
}

fn evaporation_cover() -> Part {
    let frame = rect_frame(
        "evaporation_cover_gasket_frame",
        COVER_X,
        COVER_Y,
        COVER_Z,
        COVER_RAIL_W,
    )
    .translate(COVER_CENTER.0, COVER_CENTER.1, COVER_CLEAR_Z);
    let hinge = centered_cylinder(
        name("evaporation_cover_rear_hinge_barrel"),
        7.0,
        COVER_X,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 + COVER_Y / 2.0 + 10.0,
        COVER_CLEAR_Z,
    );
    let mut latches = Part::empty(name("evaporation_cover_front_latches"));
    for latch in 0..COVER_LATCH_COUNT {
        let x = COVER_CENTER.0 + lane_offset(latch, COVER_LATCH_COUNT, 180.0);
        latches = latches
            + centered_cube(
                name(&format!("evaporation_cover_quick_latch_{latch}")),
                44.0,
                14.0,
                16.0,
            )
            .translate(
                x,
                COVER_CENTER.1 - COVER_Y / 2.0 - 9.0,
                COVER_CLEAR_Z - COVER_Z / 2.0,
            );
    }
    let gasket_witness = centered_cube(
        name("evaporation_cover_compression_witness_strip"),
        COVER_X - 86.0,
        6.0,
        5.0,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 - COVER_Y / 2.0 + 28.0,
        COVER_CLEAR_Z - COVER_Z / 2.0 - 4.0,
    );

    frame + hinge + latches + gasket_witness
}

fn barcode_run_record_lands() -> Part {
    let plate = centered_cube(
        name("barcode_run_record_land_plate"),
        TRACE_PLATE_X,
        TRACE_PLATE_Y,
        TRACE_PLATE_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, stage_z(TRACE_PLATE_Z));
    let mut lands = Part::empty(name("barcode_and_run_record_lands"));
    for land in 0..BARCODE_LAND_COUNT {
        let x = TRACE_CENTER.0 + lane_offset(land, BARCODE_LAND_COUNT, 26.0);
        lands = lands
            + centered_cube(name(&format!("barcode_land_{land}")), 20.0, 30.0, 4.0).translate(
                x,
                TRACE_CENTER.1 + 18.0,
                BASE_Z / 2.0 + TRACE_PLATE_Z + 2.0,
            );
    }
    for record in 0..RUN_RECORD_LAND_COUNT {
        let x = TRACE_CENTER.0 + lane_offset(record, RUN_RECORD_LAND_COUNT, 66.0);
        lands = lands
            + centered_cube(
                name(&format!("run_record_custody_land_{record}")),
                54.0,
                22.0,
                4.0,
            )
            .translate(x, TRACE_CENTER.1 - 22.0, BASE_Z / 2.0 + TRACE_PLATE_Z + 2.0);
    }
    let chain_of_custody_tab = centered_cube(
        name("run_record_chain_of_custody_tab"),
        TRACE_PLATE_X - 34.0,
        8.0,
        6.0,
    )
    .translate(
        TRACE_CENTER.0,
        TRACE_CENTER.1 - TRACE_PLATE_Y / 2.0 + 8.0,
        BASE_Z / 2.0 + TRACE_PLATE_Z + 3.0,
    );

    plate + lands + chain_of_custody_tab
}

fn high_low_standard_lanes() -> Part {
    standard_lane("high_standard_lane", STANDARD_HIGH_CENTER, 0)
        + standard_lane("low_standard_lane", STANDARD_LOW_CENTER, 1)
        + standard_lane_divider_bridge()
}

fn standard_lane(label: &str, center: (f64, f64), index_offset: usize) -> Part {
    let lane = centered_cube(
        name(label),
        STANDARD_LANE_X,
        STANDARD_LANE_Y,
        STANDARD_LANE_Z,
    )
    .translate(center.0, center.1, stage_z(STANDARD_LANE_Z));
    let mut cuts = Part::empty(name(&format!("{label}_standard_tube_pockets")));
    let mut tags = Part::empty(name(&format!("{label}_index_tags")));

    for slot in 0..STANDARD_SLOT_COUNT {
        let x = center.0 + lane_offset(slot, STANDARD_SLOT_COUNT, STANDARD_SLOT_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                name(&format!("{label}_standard_tube_pocket_{slot}")),
                8.5,
                STANDARD_LANE_Z + 8.0,
                28,
            )
            .translate(x, center.1, stage_z(STANDARD_LANE_Z));
        tags = tags
            + centered_cube(
                name(&format!(
                    "{label}_custody_index_tag_{}",
                    slot + index_offset * 10
                )),
                22.0,
                8.0,
                5.0,
            )
            .translate(
                x,
                center.1 + STANDARD_LANE_Y / 2.0 - 12.0,
                BASE_Z / 2.0 + STANDARD_LANE_Z + 2.5,
            );
    }

    lane - cuts + tags + standard_lane_handle(label, center)
}

fn standard_lane_handle(label: &str, center: (f64, f64)) -> Part {
    centered_cube(
        name(&format!("{label}_sealed_caddy_pull_handle")),
        78.0,
        10.0,
        16.0,
    )
    .translate(
        center.0,
        center.1 - STANDARD_LANE_Y / 2.0 - 8.0,
        BASE_Z / 2.0 + STANDARD_LANE_Z + 8.0,
    )
}

fn standard_lane_divider_bridge() -> Part {
    centered_cube(
        name("high_low_standard_lane_custody_gap_bridge"),
        24.0,
        STANDARD_LANE_Y + 30.0,
        34.0,
    )
    .translate(
        (STANDARD_HIGH_CENTER.0 + STANDARD_LOW_CENTER.0) / 2.0,
        STANDARD_HIGH_CENTER.1,
        BASE_Z / 2.0 + 17.0,
    )
}

fn flush_waste_route() -> Part {
    let rail = centered_cube(
        name("flush_waste_route_manifold_rail"),
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    )
    .translate(ROUTE_CENTER.0, ROUTE_CENTER.1, stage_z(ROUTE_Z));
    let mut cuts = Part::empty(name("flush_waste_route_channel_cuts"));
    let mut ports = Part::empty(name("flush_waste_route_ports_and_labels"));

    for lane in 0..FLUSH_PORT_COUNT {
        let y = ROUTE_CENTER.1 + lane_offset(lane, FLUSH_PORT_COUNT, ROUTE_PORT_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                name(&format!("flush_inlet_port_bore_{lane}")),
                4.0,
                ROUTE_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(ROUTE_CENTER.0, y, stage_z(ROUTE_Z))
            + centered_cube(
                name(&format!("waste_route_groove_{lane}")),
                10.0,
                ROUTE_PORT_PITCH_Y - 10.0,
                8.0,
            )
            .translate(ROUTE_CENTER.0, y, BASE_Z / 2.0 + ROUTE_Z - 4.0);
        ports = ports
            + centered_cylinder(name(&format!("flush_port_boss_{lane}")), 8.0, 8.0, 24)
                .rotate(0.0, 90.0, 0.0)
                .translate(
                    ROUTE_CENTER.0 - ROUTE_X / 2.0 - 4.0,
                    y,
                    BASE_Z / 2.0 + ROUTE_Z - 10.0,
                );
    }

    let waste_cup =
        centered_cylinder(name("closed_waste_recovery_cup_socket"), 24.0, 28.0, 40).translate(
            ROUTE_CENTER.0,
            ROUTE_CENTER.1 - ROUTE_Y / 2.0 + 42.0,
            BASE_Z / 2.0 + ROUTE_Z + 14.0,
        ) - centered_cylinder(name("closed_waste_recovery_cup_opening"), 18.0, 30.0, 40).translate(
            ROUTE_CENTER.0,
            ROUTE_CENTER.1 - ROUTE_Y / 2.0 + 42.0,
            BASE_Z / 2.0 + ROUTE_Z + 14.0,
        );

    rail - cuts + ports + waste_cup
}

fn pass_hold_reject_status_lanes() -> Part {
    disposition_lane("pass", STATUS_PASS_Y, 0)
        + disposition_lane("hold", STATUS_HOLD_Y, 1)
        + disposition_lane("reject", STATUS_REJECT_Y, 2)
        + centered_cube(
            name("status_lane_to_custody_gap_gauge"),
            12.0,
            STATUS_PASS_Y - STATUS_REJECT_Y + STATUS_LANE_Y,
            54.0,
        )
        .translate(
            STATUS_CENTER_X + STATUS_LANE_X / 2.0 + 18.0,
            (STATUS_PASS_Y + STATUS_REJECT_Y) / 2.0,
            BASE_Z / 2.0 + 27.0,
        )
}

fn disposition_lane(label: &str, center_y: f64, index: usize) -> Part {
    let lane = centered_cube(
        name(&format!("{label}_status_lane_caddy")),
        STATUS_LANE_X,
        STATUS_LANE_Y,
        STATUS_LANE_Z,
    )
    .translate(STATUS_CENTER_X, center_y, stage_z(STATUS_LANE_Z));
    let pocket = centered_cube(
        name(&format!("{label}_status_lane_micro_carrier_pocket")),
        STATUS_LANE_X - 20.0,
        STATUS_LANE_Y - 28.0,
        STATUS_LANE_Z + 6.0,
    )
    .translate(STATUS_CENTER_X, center_y, stage_z(STATUS_LANE_Z));
    let shutter = centered_cube(
        name(&format!("{label}_status_lane_shutter_land")),
        STATUS_LANE_X - 12.0,
        10.0,
        10.0,
    )
    .translate(
        STATUS_CENTER_X,
        center_y + STATUS_LANE_Y / 2.0 - 10.0,
        BASE_Z / 2.0 + STATUS_LANE_Z + 5.0,
    );
    let index_pin = centered_cylinder(
        name(&format!("{label}_status_lane_index_pin_{index}")),
        4.0,
        18.0,
        20,
    )
    .translate(
        STATUS_CENTER_X - STATUS_LANE_X / 2.0 + 12.0,
        center_y - STATUS_LANE_Y / 2.0 + 12.0,
        BASE_Z / 2.0 + STATUS_LANE_Z + 9.0,
    );

    lane - pocket + shutter + index_pin
}

fn evidence_camera_bridge() -> Part {
    let left_column = centered_cube(
        name("evidence_camera_bridge_left_column"),
        CAMERA_COLUMN_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_CENTER.0 - CAMERA_BRIDGE_X / 2.0 + CAMERA_COLUMN_X / 2.0,
        CAMERA_CENTER.1,
        BASE_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_column = centered_cube(
        name("evidence_camera_bridge_right_column"),
        CAMERA_COLUMN_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_CENTER.0 + CAMERA_BRIDGE_X / 2.0 - CAMERA_COLUMN_X / 2.0,
        CAMERA_CENTER.1,
        BASE_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let crossbar = centered_cube(
        name("evidence_camera_bridge_crossbar"),
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_CROSSBAR_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1,
        BASE_Z / 2.0 + CAMERA_BRIDGE_Z - CAMERA_CROSSBAR_Z / 2.0,
    );
    let camera_window = centered_cube(
        name("evidence_camera_bridge_camera_window_cut"),
        CAMERA_WINDOW_X,
        CAMERA_WINDOW_Y,
        CAMERA_CROSSBAR_Z + 6.0,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1,
        BASE_Z / 2.0 + CAMERA_BRIDGE_Z - CAMERA_CROSSBAR_Z / 2.0,
    );
    let light_bar = centered_cube(
        name("evidence_camera_bridge_oblique_light_bar"),
        CAMERA_BRIDGE_X - 160.0,
        8.0,
        10.0,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1 - CAMERA_BRIDGE_Y / 2.0 - 8.0,
        BASE_Z / 2.0 + CAMERA_BRIDGE_Z - 42.0,
    );
    let focus_coupon = centered_cube(
        name("evidence_camera_bridge_focus_coupon_land"),
        92.0,
        22.0,
        5.0,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 + 14.0,
        BASE_Z / 2.0 + 34.0,
    );

    left_column + right_column + crossbar - camera_window + light_bar + focus_coupon
}

fn robot_service_keepouts() -> Part {
    let rear_robot_bar = centered_cube(
        name("robot_approach_keepout_bar"),
        ROBOT_KEEP_OUT_X,
        12.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 78.0,
        BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0,
    );
    let front_service_bar = centered_cube(
        name("front_service_clearance_keepout_bar"),
        ROBOT_KEEP_OUT_X,
        10.0,
        64.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 76.0), BASE_Z / 2.0 + 32.0);
    let right_service_bar = centered_cube(
        name("right_balance_route_service_keepout_bar"),
        10.0,
        ROBOT_KEEP_OUT_Y + 220.0,
        64.0,
    )
    .translate(STATION_X / 2.0 - 96.0, 0.0, BASE_Z / 2.0 + 32.0);

    let mut posts = Part::empty(name("robot_service_keepout_gauge_posts"));
    for (i, (x, y)) in [
        (-360.0, 266.0),
        (0.0, 266.0),
        (360.0, 266.0),
        (-360.0, -266.0),
        (360.0, -266.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                name(&format!("robot_service_keepout_post_{i}")),
                6.0,
                62.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 31.0);
    }

    rear_robot_bar + front_service_bar + right_service_bar + posts
}

fn rect_frame(name_suffix: &str, outer_x: f64, outer_y: f64, z: f64, rail_w: f64) -> Part {
    let left = centered_cube(
        name(&format!("{name_suffix}_left_rail")),
        rail_w,
        outer_y,
        z,
    )
    .translate(-(outer_x / 2.0 - rail_w / 2.0), 0.0, 0.0);
    let right = centered_cube(
        name(&format!("{name_suffix}_right_rail")),
        rail_w,
        outer_y,
        z,
    )
    .translate(outer_x / 2.0 - rail_w / 2.0, 0.0, 0.0);
    let rear = centered_cube(
        name(&format!("{name_suffix}_rear_rail")),
        outer_x,
        rail_w,
        z,
    )
    .translate(0.0, outer_y / 2.0 - rail_w / 2.0, 0.0);
    let front = centered_cube(
        name(&format!("{name_suffix}_front_rail")),
        outer_x,
        rail_w,
        z,
    )
    .translate(0.0, -(outer_y / 2.0 - rail_w / 2.0), 0.0);

    left + right + rear + front
}

fn stage_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn lane_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(suffix: &str) -> String {
    format!("{PREFIX}_{suffix}")
}

#[derive(Clone, Copy)]
struct LayoutSpec {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

fn layout_specs() -> [LayoutSpec; 10] {
    [
        LayoutSpec {
            name: "micro_dose_collection_nests",
            center: MICRO_NEST_CENTER,
            width: MICRO_NEST_X,
            depth: MICRO_NEST_Y,
        },
        LayoutSpec {
            name: "balance_load_cell_pad_placeholders",
            center: BALANCE_CENTER,
            width: BALANCE_RAIL_X,
            depth: BALANCE_RAIL_Y,
        },
        LayoutSpec {
            name: "capillary_witness_channels",
            center: CAPILLARY_CENTER,
            width: CAPILLARY_PLATE_X,
            depth: CAPILLARY_PLATE_Y,
        },
        LayoutSpec {
            name: "dead_volume_recovery_wells",
            center: RECOVERY_CENTER,
            width: RECOVERY_BLOCK_X,
            depth: RECOVERY_BLOCK_Y,
        },
        LayoutSpec {
            name: "barcode_run_record_lands",
            center: TRACE_CENTER,
            width: TRACE_PLATE_X,
            depth: TRACE_PLATE_Y,
        },
        LayoutSpec {
            name: "high_standard_lane",
            center: STANDARD_HIGH_CENTER,
            width: STANDARD_LANE_X,
            depth: STANDARD_LANE_Y,
        },
        LayoutSpec {
            name: "low_standard_lane",
            center: STANDARD_LOW_CENTER,
            width: STANDARD_LANE_X,
            depth: STANDARD_LANE_Y,
        },
        LayoutSpec {
            name: "flush_waste_route",
            center: ROUTE_CENTER,
            width: ROUTE_X,
            depth: ROUTE_Y,
        },
        LayoutSpec {
            name: "status_lanes",
            center: (STATUS_CENTER_X, STATUS_HOLD_Y),
            width: STATUS_LANE_X,
            depth: STATUS_PASS_Y - STATUS_REJECT_Y + STATUS_LANE_Y,
        },
        LayoutSpec {
            name: "evidence_camera_bridge",
            center: CAMERA_CENTER,
            width: CAMERA_BRIDGE_X,
            depth: CAMERA_BRIDGE_Y,
        },
    ]
}

fn assert_layout() {
    for spec in layout_specs() {
        assert!(
            fits_on_station(spec.center, spec.width, spec.depth),
            "{} does not fit the station envelope",
            spec.name
        );
    }
    assert!(status_to_custody_gap() >= STATUS_CUSTODY_MIN_GAP);
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    let half_x = STATION_X / 2.0 - FIT_MARGIN;
    let half_y = STATION_Y / 2.0 - FIT_MARGIN;
    center.0 - width / 2.0 >= -half_x
        && center.0 + width / 2.0 <= half_x
        && center.1 - depth / 2.0 >= -half_y
        && center.1 + depth / 2.0 <= half_y
}

fn status_to_custody_gap() -> f64 {
    let status_right_edge = STATUS_CENTER_X + STATUS_LANE_X / 2.0;
    let custody_left_edge = STANDARD_HIGH_CENTER.0 - STANDARD_LANE_X / 2.0;
    custody_left_edge - status_right_edge
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn required_features_cover_station_intent() {
        for feature in [
            "micro_dose_collection_nests",
            "balance_load_cell_pad_placeholders",
            "capillary_witness_channels",
            "dead_volume_recovery_wells",
            "evaporation_cover",
            "barcode_run_record_lands",
            "high_low_standard_lanes",
            "flush_waste_route",
            "pass_hold_reject_status_lanes",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 13);
    }

    #[test]
    fn outputs_are_unique_prefixed_and_include_assembly() {
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(&format!("output/{PREFIX}_"))));
        assert!(OUTPUTS
            .iter()
            .any(|path| path
                .ends_with("closed_low_volume_dose_accuracy_recovery_station_assembly.stl")));
        assert_eq!(OUTPUTS.len(), 13);
    }

    #[test]
    fn basic_dimensions_and_layout_fit_station_envelope() {
        assert_layout();
        assert!(STATION_X <= 1000.0);
        assert!(STATION_Y <= 700.0);
        assert!(COVER_X < STATION_X - 120.0);
        assert!(CAMERA_BRIDGE_X < STATION_X - 100.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 400.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 160.0);
    }

    #[test]
    fn dose_accuracy_and_recovery_capacity_are_explicit() {
        assert_eq!(MICRO_NEST_COUNT, 12);
        assert_eq!(BALANCE_PAD_COUNT, 6);
        assert_eq!(CAPILLARY_CHANNEL_COUNT, MICRO_NEST_COUNT);
        assert!(RECOVERY_WELL_COUNT >= 8);
        assert!(RECOVERY_WELL_D < MICRO_COLLECTION_WELL_D);
    }

    #[test]
    fn status_and_custody_are_physically_separated() {
        assert_eq!(STATUS_LANE_COUNT, 3);
        assert_eq!(STANDARD_SLOT_COUNT * 2, 10);
        assert!(STANDARD_LANE_CENTER_GAP > STANDARD_LANE_X);
        assert!(status_to_custody_gap() >= STATUS_CUSTODY_MIN_GAP);
        assert!(STATUS_REJECT_Y > TRACE_CENTER.1 + TRACE_PLATE_Y / 2.0);
    }
}
