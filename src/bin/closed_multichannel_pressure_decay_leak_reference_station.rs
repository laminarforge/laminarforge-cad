use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed multichannel pressure-decay leak reference station for sterile
// fluid/gas harness validation packaging before tissue-chip runs.
//
// Intent:
// - Present a 20-channel reference manifold coupon with visible inlet/outlet
//   witness ports, traceability lands, and channel separation.
// - Package calibrated leak orifice placeholders, pressure transducer docks,
//   isolation valve placeholders, reference volume blocks, vent/filter route
//   witnesses, release/hold/reject lanes, and clean/used segregation.
// - Provide evidence-camera and robot/service keepout geometry for repeatable
//   image capture and automated handling.
//
// This is mechanical validation packaging only. It is not a pressure-rated
// system design, not a leak acceptance protocol, and not a specification for
// sterile barrier or wetted internal construction.

const OUTPUTS: &[&str] = &[
    "output/closed_multichannel_pressure_decay_leak_reference_station_base_containment_tray.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_20_channel_reference_manifold_coupon.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_calibrated_leak_orifice_placeholder_nest.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_pressure_transducer_dock_panel.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_isolation_valve_placeholder_bank.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_reference_volume_blocks.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_vent_filter_routing_witness_features.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_barcode_certificate_lands.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_release_hold_reject_lanes.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_clean_used_segregation.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_evidence_camera_bridge.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_robot_service_keepout_gauges.stl",
    "output/closed_multichannel_pressure_decay_leak_reference_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_containment_tray",
    "20_channel_reference_manifold_coupon",
    "calibrated_leak_orifice_placeholder_nest",
    "pressure_transducer_dock_panel",
    "isolation_valve_placeholder_bank",
    "reference_volume_blocks",
    "vent_filter_routing_witness_features",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
    "assembly_export",
];

const LIMITATIONS: &[&str] = &[
    "mechanical_validation_packaging_only",
    "not_pressure_rated_system_design",
    "not_leak_acceptance_protocol",
    "bought_transducers_valves_filters_orifices_as_placeholders",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const BASIN_RECESS_Z: f64 = 8.0;
const DRAIN_D: f64 = 15.0;
const MOUNT_SLOT_COUNT: usize = 10;

const CHANNEL_ROWS: usize = 4;
const CHANNEL_COLS: usize = 5;
const CHANNEL_COUNT: usize = CHANNEL_ROWS * CHANNEL_COLS;
const CHANNEL_PAIR_PORTS: usize = CHANNEL_COUNT * 2;
const CHANNEL_GROUPS: usize = CHANNEL_COLS;

const MANIFOLD_X: f64 = 1080.0;
const MANIFOLD_Y: f64 = 218.0;
const MANIFOLD_Z: f64 = 30.0;
const MANIFOLD_POS: (f64, f64) = (0.0, 252.0);
const MANIFOLD_CHANNEL_PITCH_X: f64 = 198.0;
const MANIFOLD_CHANNEL_PITCH_Y: f64 = 44.0;
const CHANNEL_GROOVE_X: f64 = 126.0;
const CHANNEL_GROOVE_Y: f64 = 6.0;
const CHANNEL_GROOVE_Z: f64 = 10.0;
const PORT_D: f64 = 8.8;
const PORT_PAD_D: f64 = 19.0;

const ORIFICE_NEST_X: f64 = 360.0;
const ORIFICE_NEST_Y: f64 = 160.0;
const ORIFICE_NEST_Z: f64 = 46.0;
const ORIFICE_POS: (f64, f64) = (-450.0, -104.0);
const ORIFICE_PLACEHOLDERS: usize = CHANNEL_COUNT;
const ORIFICE_PITCH_X: f64 = 66.0;
const ORIFICE_PITCH_Y: f64 = 36.0;
const ORIFICE_BORE_D: f64 = 12.2;
const ORIFICE_WITNESS_D: f64 = 18.0;

const TRANSDUCER_PANEL_X: f64 = 470.0;
const TRANSDUCER_PANEL_Y: f64 = 168.0;
const TRANSDUCER_PANEL_Z: f64 = 52.0;
const TRANSDUCER_POS: (f64, f64) = (0.0, -104.0);
const TRANSDUCER_DOCKS: usize = CHANNEL_COUNT;
const TRANSDUCER_PITCH_X: f64 = 82.0;
const TRANSDUCER_PITCH_Y: f64 = 36.0;
const TRANSDUCER_POCKET_X: f64 = 42.0;
const TRANSDUCER_POCKET_Y: f64 = 22.0;
const TRANSDUCER_CABLE_SLOTS: usize = CHANNEL_COLS;

const VALVE_BANK_X: f64 = 1040.0;
const VALVE_BANK_Y: f64 = 118.0;
const VALVE_BANK_Z: f64 = 54.0;
const VALVE_BANK_POS: (f64, f64) = (0.0, 58.0);
const VALVE_PLACEHOLDERS: usize = CHANNEL_COUNT;
const VALVE_ROWS: usize = 2;
const VALVE_COLS: usize = CHANNEL_COUNT / VALVE_ROWS;
const VALVE_PITCH_X: f64 = 94.0;
const VALVE_PITCH_Y: f64 = 44.0;
const VALVE_BODY_D: f64 = 26.0;
const VALVE_HANDLE_X: f64 = 38.0;
const VALVE_HANDLE_Y: f64 = 12.0;

const VOLUME_BLOCKS_X: f64 = 360.0;
const VOLUME_BLOCKS_Y: f64 = 158.0;
const VOLUME_BLOCKS_Z: f64 = 66.0;
const VOLUME_POS: (f64, f64) = (448.0, -104.0);
const REFERENCE_VOLUME_BLOCKS: usize = CHANNEL_GROUPS;
const VOLUME_BLOCK_PITCH_X: f64 = 66.0;
const VOLUME_BLOCK_X: f64 = 52.0;
const VOLUME_BLOCK_Y: f64 = 118.0;
const VOLUME_BLOCK_Z: f64 = 66.0;
const VOLUME_CHANNELS_PER_BLOCK: usize = CHANNEL_ROWS;

const VENT_ROUTE_X: f64 = 270.0;
const VENT_ROUTE_Y: f64 = 126.0;
const VENT_ROUTE_Z: f64 = 42.0;
const VENT_ROUTE_POS: (f64, f64) = (-490.0, -304.0);
const VENT_ROUTE_WITNESSES: usize = CHANNEL_COUNT;
const VENT_FILTER_PLACEHOLDERS: usize = CHANNEL_GROUPS;
const VENT_TUBE_BORE_D: f64 = 5.6;
const VENT_FILTER_D: f64 = 23.0;

const TRACE_PANEL_X: f64 = 300.0;
const TRACE_PANEL_Y: f64 = 112.0;
const TRACE_PANEL_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (-216.0, -304.0);
const BARCODE_LANDS: usize = CHANNEL_COUNT;
const CERTIFICATE_LANDS: usize = CHANNEL_GROUPS;
const RUN_RECORD_LANDS: usize = 3;
const TRACE_LAND_X: f64 = 42.0;
const TRACE_LAND_Y: f64 = 18.0;

const DISPOSITION_X: f64 = 310.0;
const DISPOSITION_Y: f64 = 138.0;
const DISPOSITION_Z: f64 = 34.0;
const DISPOSITION_POS: (f64, f64) = (116.0, -304.0);
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_SLOTS_PER_LANE: usize = 8;
const STATUS_SLOT_X: f64 = 26.0;
const STATUS_SLOT_Y: f64 = 24.0;
const STATUS_LANE_PITCH_Y: f64 = 38.0;

const SEGREGATION_X: f64 = 310.0;
const SEGREGATION_Y: f64 = 126.0;
const SEGREGATION_Z: f64 = 78.0;
const SEGREGATION_POS: (f64, f64) = (468.0, -304.0);
const CLEAN_USED_LAND_OFFSET: f64 = 96.0;
const CLEAN_USED_MIN_GAP: f64 = 72.0;
const CLEAN_CAP_POSTS: usize = 10;
const USED_ORIFICE_CUPS: usize = 10;

const CAMERA_BRIDGE_X: f64 = 1050.0;
const CAMERA_BRIDGE_Y: f64 = 58.0;
const CAMERA_BRIDGE_Z: f64 = 212.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, 252.0);
const CAMERA_COUNT: usize = 3;
const CAMERA_MOUNT_PITCH_X: f64 = 320.0;
const CAMERA_WINDOW_X: f64 = 108.0;
const CAMERA_WINDOW_Y: f64 = 30.0;
const CAMERA_WINDOW_Z: f64 = 10.0;
const CAMERA_DATUMS: usize = 6;

const KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 390.0;
const REAR_MANIFOLD_SERVICE_CLEARANCE: f64 = 235.0;
const LEFT_ORIFICE_SERVICE_CLEARANCE: f64 = 170.0;
const RIGHT_TRANSDUCER_SERVICE_CLEARANCE: f64 = 190.0;
const VALVE_SERVICE_LIFT_CLEARANCE: f64 = 160.0;
const CAMERA_LIFT_CLEARANCE: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_tray();
    export(&base, OUTPUTS[0]);

    let manifold = reference_manifold_coupon_20_channel();
    export(&manifold, OUTPUTS[1]);

    let orifices = calibrated_leak_orifice_placeholder_nest();
    export(&orifices, OUTPUTS[2]);

    let transducers = pressure_transducer_dock_panel();
    export(&transducers, OUTPUTS[3]);

    let valves = isolation_valve_placeholder_bank();
    export(&valves, OUTPUTS[4]);

    let volumes = reference_volume_blocks();
    export(&volumes, OUTPUTS[5]);

    let vent_routes = vent_filter_routing_witness_features();
    export(&vent_routes, OUTPUTS[6]);

    let traceability = barcode_certificate_lands();
    export(&traceability, OUTPUTS[7]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[8]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[9]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[10]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, insert_z(MANIFOLD_Z))
        + orifices.translate(ORIFICE_POS.0, ORIFICE_POS.1, insert_z(ORIFICE_NEST_Z))
        + transducers.translate(
            TRANSDUCER_POS.0,
            TRANSDUCER_POS.1,
            insert_z(TRANSDUCER_PANEL_Z),
        )
        + valves.translate(VALVE_BANK_POS.0, VALVE_BANK_POS.1, insert_z(VALVE_BANK_Z))
        + volumes.translate(VOLUME_POS.0, VOLUME_POS.1, insert_z(VOLUME_BLOCKS_Z))
        + vent_routes.translate(VENT_ROUTE_POS.0, VENT_ROUTE_POS.1, insert_z(VENT_ROUTE_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z))
        + disposition.translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            insert_z(DISPOSITION_Z),
        )
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + camera.translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, BASE_Z / 2.0)
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed multichannel pressure-decay leak reference station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray with {RIM_Z:.0}mm rim"
    );
    println!(
        "  Reference coupon:          {CHANNEL_COUNT} channels in {CHANNEL_ROWS}x{CHANNEL_COLS} layout, {CHANNEL_PAIR_PORTS} inlet/outlet witness ports"
    );
    println!(
        "  Placeholder interfaces:    {ORIFICE_PLACEHOLDERS} calibrated leak orifice nests, {TRANSDUCER_DOCKS} transducer docks, {VALVE_PLACEHOLDERS} isolation valve envelopes"
    );
    println!(
        "  Reference/vent handling:   {REFERENCE_VOLUME_BLOCKS} reference volume blocks, {VENT_ROUTE_WITNESSES} vent route witnesses, {VENT_FILTER_PLACEHOLDERS} vent/filter placeholders"
    );
    println!(
        "  Evidence/disposition:      {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {RUN_RECORD_LANDS} run-record lands, {STATUS_LANES} release/hold/reject lanes, {CAMERA_COUNT} evidence cameras"
    );
    println!(
        "  Robot/service gauges:      {KEEP_OUT_GAUGES} keepout gauges, {FRONT_ROBOT_SWEEP_CLEARANCE:.0}mm front sweep, {REAR_MANIFOLD_SERVICE_CLEARANCE:.0}mm rear service, {CAMERA_LIFT_CLEARANCE:.0}mm camera bridge lift"
    );
    println!(
        "  Limitations:               packaging CAD only; not pressure-rated design; not a leak acceptance protocol; {} limitation markers and {} required feature groups",
        LIMITATIONS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "closed_multichannel_pressure_decay_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin_recess = centered_cube(
        "closed_multichannel_pressure_decay_sumped_containment_recess",
        STATION_X - 118.0,
        STATION_Y - 118.0,
        BASIN_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - BASIN_RECESS_Z / 2.0 + 1.0);
    let front_drain = centered_cylinder(
        "closed_multichannel_pressure_decay_front_tray_drain",
        DRAIN_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -(STATION_Y / 2.0 - 18.0), 0.0);
    let rear_drain_witness = centered_cylinder(
        "closed_multichannel_pressure_decay_rear_drain_witness",
        DRAIN_D / 2.0,
        4.0,
        32,
    )
    .translate(
        -(STATION_X / 2.0 - 88.0),
        STATION_Y / 2.0 - 70.0,
        BASE_Z / 2.0 + 2.0,
    );

    deck - basin_recess - front_drain - mounting_slots()
        + containment_rims()
        + rear_drain_witness
        + datum_fiducials()
        + module_socket_rails()
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "closed_multichannel_pressure_decay_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_multichannel_pressure_decay_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_multichannel_pressure_decay_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_multichannel_pressure_decay_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    left + right + front + rear
}

fn module_socket_rails() -> Part {
    let mut rails = Part::empty("closed_multichannel_pressure_decay_module_socket_rails");
    for (i, (pos, width, depth)) in [
        (MANIFOLD_POS, MANIFOLD_X + 28.0, MANIFOLD_Y + 24.0),
        (VALVE_BANK_POS, VALVE_BANK_X + 24.0, VALVE_BANK_Y + 22.0),
        (ORIFICE_POS, ORIFICE_NEST_X + 22.0, ORIFICE_NEST_Y + 20.0),
        (
            TRANSDUCER_POS,
            TRANSDUCER_PANEL_X + 22.0,
            TRANSDUCER_PANEL_Y + 20.0,
        ),
        (VOLUME_POS, VOLUME_BLOCKS_X + 22.0, VOLUME_BLOCKS_Y + 20.0),
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("closed_multichannel_pressure_decay_module_socket_rail_{i}_front"),
                *width,
                5.0,
                8.0,
            )
            .translate(pos.0, pos.1 - depth / 2.0, BASE_Z / 2.0 + 4.0)
            + centered_cube(
                format!("closed_multichannel_pressure_decay_module_socket_rail_{i}_rear"),
                *width,
                5.0,
                8.0,
            )
            .translate(pos.0, pos.1 + depth / 2.0, BASE_Z / 2.0 + 4.0);
    }
    rails
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_multichannel_pressure_decay_mounting_slots");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_m6_mount_round_{i}"),
                3.5,
                BASE_Z + 8.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_multichannel_pressure_decay_m6_mount_slot_{i}"),
                30.0,
                7.2,
                BASE_Z + 8.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_fiducials() -> Part {
    let mut datums = Part::empty("closed_multichannel_pressure_decay_tray_datums");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 68.0), -(STATION_Y / 2.0 - 62.0)),
        (STATION_X / 2.0 - 68.0, -(STATION_Y / 2.0 - 62.0)),
        (-(STATION_X / 2.0 - 68.0), STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 68.0, STATION_Y / 2.0 - 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (0.0, -(STATION_Y / 2.0 - 62.0)),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_datum_ring_{i}"),
                9.5,
                3.0,
                32,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("closed_multichannel_pressure_decay_datum_center_{i}"),
                2.0,
                4.0,
                18,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    datums
}

fn reference_manifold_coupon_20_channel() -> Part {
    let coupon = centered_cube(
        "closed_multichannel_pressure_decay_reference_manifold_coupon_plate",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let row_header = centered_cube(
        "closed_multichannel_pressure_decay_reference_coupon_row_header_land",
        MANIFOLD_X - 54.0,
        14.0,
        6.0,
    )
    .translate(0.0, MANIFOLD_Y / 2.0 - 18.0, MANIFOLD_Z / 2.0 + 3.0);
    let column_header = centered_cube(
        "closed_multichannel_pressure_decay_reference_coupon_column_header_land",
        14.0,
        MANIFOLD_Y - 52.0,
        6.0,
    )
    .translate(-(MANIFOLD_X / 2.0 - 28.0), 0.0, MANIFOLD_Z / 2.0 + 3.0);

    let mut pads = Part::empty("closed_multichannel_pressure_decay_reference_coupon_port_pads");
    let mut cuts = Part::empty("closed_multichannel_pressure_decay_reference_coupon_channel_cuts");
    let mut group_ribs =
        Part::empty("closed_multichannel_pressure_decay_reference_coupon_group_ribs");
    for channel in 0..CHANNEL_COUNT {
        let (row, col) = channel_row_col(channel);
        let x = lane_x(col, CHANNEL_COLS, MANIFOLD_CHANNEL_PITCH_X);
        let y = lane_x(row, CHANNEL_ROWS, MANIFOLD_CHANNEL_PITCH_Y) - 12.0;

        let groove = centered_cube(
            format!("closed_multichannel_pressure_decay_reference_coupon_channel_{channel}_groove"),
            CHANNEL_GROOVE_X,
            CHANNEL_GROOVE_Y,
            CHANNEL_GROOVE_Z,
        )
        .translate(x, y, MANIFOLD_Z / 2.0 - 2.0);
        cuts = cuts + groove;

        for (side, x_offset) in [-48.0, 48.0].iter().enumerate() {
            let port_x = x + x_offset;
            pads = pads
                + centered_cylinder(
                    format!(
                        "closed_multichannel_pressure_decay_reference_coupon_channel_{channel}_port_pad_{side}"
                    ),
                    PORT_PAD_D / 2.0,
                    4.0,
                    28,
                )
                .translate(port_x, y, MANIFOLD_Z / 2.0 + 2.0);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_multichannel_pressure_decay_reference_coupon_channel_{channel}_port_clearance_{side}"
                    ),
                    PORT_D / 2.0,
                    MANIFOLD_Z + 8.0,
                    24,
                )
                .translate(port_x, y, 0.0);
        }
    }
    for col in 0..CHANNEL_COLS {
        group_ribs = group_ribs
            + centered_cube(
                format!("closed_multichannel_pressure_decay_reference_coupon_group_{col}_spine"),
                3.5,
                MANIFOLD_Y - 42.0,
                9.0,
            )
            .translate(
                lane_x(col, CHANNEL_COLS, MANIFOLD_CHANNEL_PITCH_X),
                -10.0,
                MANIFOLD_Z / 2.0 + 4.5,
            );
    }

    coupon - cuts + pads + group_ribs + row_header + column_header
}

fn calibrated_leak_orifice_placeholder_nest() -> Part {
    let body = centered_cube(
        "closed_multichannel_pressure_decay_orifice_placeholder_nest_body",
        ORIFICE_NEST_X,
        ORIFICE_NEST_Y,
        ORIFICE_NEST_Z,
    );
    let bevel_witness = centered_cube(
        "closed_multichannel_pressure_decay_orifice_nest_beveled_clean_lip_witness",
        ORIFICE_NEST_X - 24.0,
        ORIFICE_NEST_Y - 24.0,
        8.0,
    )
    .translate(0.0, 0.0, ORIFICE_NEST_Z / 2.0 + 4.0);

    let mut bores = Part::empty("closed_multichannel_pressure_decay_orifice_placeholder_bores");
    let mut witness_rings =
        Part::empty("closed_multichannel_pressure_decay_orifice_placeholder_witness_rings");
    for i in 0..ORIFICE_PLACEHOLDERS {
        let (row, col) = channel_row_col(i);
        let x = lane_x(col, CHANNEL_COLS, ORIFICE_PITCH_X);
        let y = lane_x(row, CHANNEL_ROWS, ORIFICE_PITCH_Y);
        bores = bores
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_calibrated_leak_orifice_placeholder_bore_{i}"),
                ORIFICE_BORE_D / 2.0,
                ORIFICE_NEST_Z + 10.0,
                28,
            )
            .translate(x, y, 0.0);
        witness_rings = witness_rings
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_calibrated_leak_orifice_placeholder_ring_{i}"),
                ORIFICE_WITNESS_D / 2.0,
                3.0,
                28,
            )
            .translate(x, y, ORIFICE_NEST_Z / 2.0 + 2.0);
    }

    let quarantine_notch = centered_cube(
        "closed_multichannel_pressure_decay_orifice_nest_used_orifice_notch",
        92.0,
        18.0,
        ORIFICE_NEST_Z + 6.0,
    )
    .translate(
        ORIFICE_NEST_X / 2.0 - 54.0,
        -(ORIFICE_NEST_Y / 2.0 - 16.0),
        0.0,
    );
    let seal_land = centered_cube(
        "closed_multichannel_pressure_decay_orifice_nest_certificate_seal_land",
        118.0,
        18.0,
        5.0,
    )
    .translate(
        -(ORIFICE_NEST_X / 2.0 - 74.0),
        ORIFICE_NEST_Y / 2.0 - 16.0,
        ORIFICE_NEST_Z / 2.0 + 2.5,
    );

    body - bores - quarantine_notch + bevel_witness + witness_rings + seal_land
}

fn pressure_transducer_dock_panel() -> Part {
    let panel = centered_cube(
        "closed_multichannel_pressure_decay_pressure_transducer_dock_panel_body",
        TRANSDUCER_PANEL_X,
        TRANSDUCER_PANEL_Y,
        TRANSDUCER_PANEL_Z,
    );
    let top_land = centered_cube(
        "closed_multichannel_pressure_decay_pressure_transducer_top_id_land",
        TRANSDUCER_PANEL_X - 28.0,
        16.0,
        5.0,
    )
    .translate(
        0.0,
        TRANSDUCER_PANEL_Y / 2.0 - 18.0,
        TRANSDUCER_PANEL_Z / 2.0 + 2.5,
    );

    let mut pockets = Part::empty("closed_multichannel_pressure_decay_transducer_dock_pockets");
    let mut cable_slots =
        Part::empty("closed_multichannel_pressure_decay_transducer_cable_witness_slots");
    let mut datum_tabs = Part::empty("closed_multichannel_pressure_decay_transducer_datum_tabs");
    for i in 0..TRANSDUCER_DOCKS {
        let (row, col) = channel_row_col(i);
        let x = lane_x(col, CHANNEL_COLS, TRANSDUCER_PITCH_X);
        let y = lane_x(row, CHANNEL_ROWS, TRANSDUCER_PITCH_Y) - 8.0;
        pockets = pockets
            + centered_cube(
                format!("closed_multichannel_pressure_decay_pressure_transducer_pocket_{i}"),
                TRANSDUCER_POCKET_X,
                TRANSDUCER_POCKET_Y,
                TRANSDUCER_PANEL_Z + 8.0,
            )
            .translate(x, y, TRANSDUCER_PANEL_Z / 2.0 - 8.0);
        datum_tabs = datum_tabs
            + centered_cube(
                format!("closed_multichannel_pressure_decay_pressure_transducer_datum_tab_{i}"),
                22.0,
                4.0,
                5.0,
            )
            .translate(
                x,
                y + TRANSDUCER_POCKET_Y / 2.0 + 5.0,
                TRANSDUCER_PANEL_Z / 2.0 + 2.5,
            );
    }
    for col in 0..TRANSDUCER_CABLE_SLOTS {
        cable_slots = cable_slots
            + centered_cube(
                format!(
                    "closed_multichannel_pressure_decay_transducer_group_{col}_cable_exit_slot"
                ),
                16.0,
                28.0,
                TRANSDUCER_PANEL_Z + 8.0,
            )
            .translate(
                lane_x(col, CHANNEL_COLS, TRANSDUCER_PITCH_X),
                -(TRANSDUCER_PANEL_Y / 2.0 - 12.0),
                0.0,
            );
    }

    panel - pockets - cable_slots + top_land + datum_tabs
}

fn isolation_valve_placeholder_bank() -> Part {
    let bank = centered_cube(
        "closed_multichannel_pressure_decay_isolation_valve_placeholder_bank_body",
        VALVE_BANK_X,
        VALVE_BANK_Y,
        VALVE_BANK_Z,
    );
    let common_tube_witness_upper = centered_cube(
        "closed_multichannel_pressure_decay_isolation_valve_upper_common_tube_witness",
        VALVE_BANK_X - 72.0,
        5.0,
        7.0,
    )
    .translate(0.0, VALVE_PITCH_Y / 2.0, VALVE_BANK_Z / 2.0 + 3.5);
    let common_tube_witness_lower = centered_cube(
        "closed_multichannel_pressure_decay_isolation_valve_lower_common_tube_witness",
        VALVE_BANK_X - 72.0,
        5.0,
        7.0,
    )
    .translate(0.0, -VALVE_PITCH_Y / 2.0, VALVE_BANK_Z / 2.0 + 3.5);

    let mut valve_cuts = Part::empty("closed_multichannel_pressure_decay_isolation_valve_cuts");
    let mut valve_lands = Part::empty("closed_multichannel_pressure_decay_isolation_valve_lands");
    for valve in 0..VALVE_PLACEHOLDERS {
        let row = valve / VALVE_COLS;
        let col = valve % VALVE_COLS;
        let x = lane_x(col, VALVE_COLS, VALVE_PITCH_X);
        let y = lane_x(row, VALVE_ROWS, VALVE_PITCH_Y);
        valve_cuts = valve_cuts
            + centered_cylinder(
                format!(
                    "closed_multichannel_pressure_decay_isolation_valve_body_clearance_{valve}"
                ),
                VALVE_BODY_D / 2.0,
                VALVE_BANK_Z + 8.0,
                28,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!(
                    "closed_multichannel_pressure_decay_isolation_valve_handle_clearance_{valve}"
                ),
                VALVE_HANDLE_X,
                VALVE_HANDLE_Y,
                VALVE_BANK_Z + 8.0,
            )
            .translate(x, y, VALVE_BANK_Z / 2.0 - 10.0);
        valve_lands = valve_lands
            + centered_cube(
                format!("closed_multichannel_pressure_decay_isolation_valve_index_land_{valve}"),
                30.0,
                8.0,
                5.0,
            )
            .translate(x, y + 23.0, VALVE_BANK_Z / 2.0 + 2.5);
    }

    bank - valve_cuts + valve_lands + common_tube_witness_upper + common_tube_witness_lower
}

fn reference_volume_blocks() -> Part {
    let backing = centered_cube(
        "closed_multichannel_pressure_decay_reference_volume_block_backing_plate",
        VOLUME_BLOCKS_X,
        VOLUME_BLOCKS_Y,
        14.0,
    )
    .translate(0.0, 0.0, -VOLUME_BLOCKS_Z / 2.0 + 7.0);
    let mut blocks = Part::empty("closed_multichannel_pressure_decay_reference_volume_blocks");
    for block in 0..REFERENCE_VOLUME_BLOCKS {
        let x = lane_x(block, REFERENCE_VOLUME_BLOCKS, VOLUME_BLOCK_PITCH_X);
        let block_body = centered_cube(
            format!("closed_multichannel_pressure_decay_reference_volume_group_{block}_block"),
            VOLUME_BLOCK_X,
            VOLUME_BLOCK_Y,
            VOLUME_BLOCK_Z,
        )
        .translate(x, 0.0, 0.0);
        let mut volume_cuts = Part::empty(format!(
            "closed_multichannel_pressure_decay_reference_volume_group_{block}_pockets"
        ));
        for row in 0..VOLUME_CHANNELS_PER_BLOCK {
            volume_cuts = volume_cuts
                + centered_cylinder(
                    format!("closed_multichannel_pressure_decay_reference_volume_group_{block}_channel_{row}_pocket"),
                    8.0,
                    VOLUME_BLOCK_X + 10.0,
                    28,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(x, lane_x(row, VOLUME_CHANNELS_PER_BLOCK, 24.0), 8.0);
        }
        let cap_land = centered_cube(
            format!("closed_multichannel_pressure_decay_reference_volume_group_{block}_certificate_cap_land"),
            VOLUME_BLOCK_X - 10.0,
            14.0,
            5.0,
        )
        .translate(x, VOLUME_BLOCK_Y / 2.0 - 12.0, VOLUME_BLOCK_Z / 2.0 + 2.5);
        blocks = blocks + block_body - volume_cuts + cap_land;
    }
    backing + blocks
}

fn vent_filter_routing_witness_features() -> Part {
    let plate = centered_cube(
        "closed_multichannel_pressure_decay_vent_filter_route_witness_plate",
        VENT_ROUTE_X,
        VENT_ROUTE_Y,
        VENT_ROUTE_Z,
    );
    let mut tube_witnesses =
        Part::empty("closed_multichannel_pressure_decay_vent_filter_tube_witnesses");
    for i in 0..VENT_ROUTE_WITNESSES {
        let row = i / CHANNEL_COLS;
        let col = i % CHANNEL_COLS;
        tube_witnesses = tube_witnesses
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_vent_route_channel_{i}_tube_witness"),
                VENT_TUBE_BORE_D / 2.0,
                86.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                lane_x(col, CHANNEL_COLS, 48.0),
                lane_x(row, CHANNEL_ROWS, 23.0),
                VENT_ROUTE_Z / 2.0 - 6.0,
            );
    }

    let mut filter_lands =
        Part::empty("closed_multichannel_pressure_decay_vent_filter_placeholder_lands");
    let mut filter_cuts =
        Part::empty("closed_multichannel_pressure_decay_vent_filter_placeholder_cuts");
    for group in 0..VENT_FILTER_PLACEHOLDERS {
        let x = lane_x(group, VENT_FILTER_PLACEHOLDERS, 48.0);
        filter_lands = filter_lands
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_vent_filter_group_{group}_land"),
                VENT_FILTER_D / 2.0,
                5.0,
                28,
            )
            .translate(x, VENT_ROUTE_Y / 2.0 - 20.0, VENT_ROUTE_Z / 2.0 + 2.5);
        filter_cuts = filter_cuts
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_vent_filter_group_{group}_placeholder"),
                (VENT_FILTER_D - 6.0) / 2.0,
                VENT_ROUTE_Z + 8.0,
                28,
            )
            .translate(x, VENT_ROUTE_Y / 2.0 - 20.0, 0.0);
    }
    let exhaust_arrow_witness = centered_cube(
        "closed_multichannel_pressure_decay_vent_filter_exhaust_direction_witness",
        VENT_ROUTE_X - 38.0,
        5.0,
        7.0,
    )
    .translate(0.0, -(VENT_ROUTE_Y / 2.0 - 18.0), VENT_ROUTE_Z / 2.0 + 3.5);

    plate - tube_witnesses - filter_cuts + filter_lands + exhaust_arrow_witness
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_multichannel_pressure_decay_barcode_certificate_land_plate",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_multichannel_pressure_decay_traceability_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / CHANNEL_COLS;
        let col = i % CHANNEL_COLS;
        lands = lands
            + centered_cube(
                format!("closed_multichannel_pressure_decay_channel_{i}_barcode_land"),
                TRACE_LAND_X,
                TRACE_LAND_Y,
                4.0,
            )
            .translate(
                lane_x(col, CHANNEL_COLS, 52.0),
                lane_x(row, CHANNEL_ROWS, 22.0) + 10.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_multichannel_pressure_decay_certificate_group_{i}_land"),
                38.0,
                16.0,
                5.0,
            )
            .translate(
                lane_x(i, CERTIFICATE_LANDS, 52.0),
                TRACE_PANEL_Y / 2.0 - 16.0,
                TRACE_PANEL_Z / 2.0 + 2.5,
            );
    }
    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_multichannel_pressure_decay_run_record_land_{i}"),
                72.0,
                16.0,
                5.0,
            )
            .translate(
                lane_x(i, RUN_RECORD_LANDS, 90.0),
                -(TRACE_PANEL_Y / 2.0 - 16.0),
                TRACE_PANEL_Z / 2.0 + 2.5,
            );
    }
    plate + lands
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_multichannel_pressure_decay_release_hold_reject_lane_panel",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut slot_cuts = Part::empty("closed_multichannel_pressure_decay_disposition_slot_cuts");
    let mut lane_rails = Part::empty("closed_multichannel_pressure_decay_disposition_lane_rails");
    for (lane, name) in STATUS_LANE_NAMES.iter().enumerate() {
        let y = lane_x(lane, STATUS_LANES, STATUS_LANE_PITCH_Y);
        lane_rails = lane_rails
            + centered_cube(
                format!("closed_multichannel_pressure_decay_{name}_lane_front_rail"),
                DISPOSITION_X - 28.0,
                4.0,
                18.0,
            )
            .translate(
                0.0,
                y - STATUS_SLOT_Y / 2.0 - 6.0,
                DISPOSITION_Z / 2.0 + 9.0,
            )
            + centered_cube(
                format!("closed_multichannel_pressure_decay_{name}_lane_rear_rail"),
                DISPOSITION_X - 28.0,
                4.0,
                18.0,
            )
            .translate(
                0.0,
                y + STATUS_SLOT_Y / 2.0 + 6.0,
                DISPOSITION_Z / 2.0 + 9.0,
            );
        for slot in 0..STATUS_SLOTS_PER_LANE {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_multichannel_pressure_decay_{name}_lane_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    DISPOSITION_Z + 8.0,
                )
                .translate(
                    lane_x(slot, STATUS_SLOTS_PER_LANE, 34.0),
                    y,
                    DISPOSITION_Z / 2.0 - 7.0,
                );
        }
    }
    let decision_gate = centered_cube(
        "closed_multichannel_pressure_decay_release_hold_reject_decision_gate",
        14.0,
        DISPOSITION_Y - 20.0,
        58.0,
    )
    .translate(
        -(DISPOSITION_X / 2.0 - 38.0),
        0.0,
        DISPOSITION_Z / 2.0 + 29.0,
    );

    panel - slot_cuts + lane_rails + decision_gate
}

fn clean_used_segregation() -> Part {
    let plate = centered_cube(
        "closed_multichannel_pressure_decay_clean_used_segregation_plate",
        SEGREGATION_X,
        SEGREGATION_Y,
        16.0,
    );
    let divider = centered_cube(
        "closed_multichannel_pressure_decay_clean_used_high_divider",
        18.0,
        SEGREGATION_Y - 16.0,
        SEGREGATION_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0);
    let clean_land = centered_cube(
        "closed_multichannel_pressure_decay_clean_reference_parts_land",
        104.0,
        86.0,
        8.0,
    )
    .translate(-CLEAN_USED_LAND_OFFSET, 0.0, 12.0);
    let used_land = centered_cube(
        "closed_multichannel_pressure_decay_used_quarantine_parts_land",
        104.0,
        86.0,
        8.0,
    )
    .translate(CLEAN_USED_LAND_OFFSET, 0.0, 12.0);

    let mut clean_posts = Part::empty("closed_multichannel_pressure_decay_clean_cap_posts");
    for i in 0..CLEAN_CAP_POSTS {
        clean_posts = clean_posts
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_clean_cap_post_{i}"),
                3.6,
                22.0,
                18,
            )
            .translate(
                -CLEAN_USED_LAND_OFFSET + lane_x(i % 5, 5, 18.0),
                lane_x(i / 5, 2, 28.0),
                27.0,
            );
    }

    let mut used_cups = Part::empty("closed_multichannel_pressure_decay_used_orifice_cups");
    for i in 0..USED_ORIFICE_CUPS {
        used_cups = used_cups
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_used_orifice_cup_{i}"),
                7.0,
                18.0,
                24,
            )
            .translate(
                CLEAN_USED_LAND_OFFSET + lane_x(i % 5, 5, 18.0),
                lane_x(i / 5, 2, 28.0),
                22.0,
            );
    }
    let one_way_gate = centered_cube(
        "closed_multichannel_pressure_decay_clean_used_one_way_gate_witness",
        46.0,
        18.0,
        34.0,
    )
    .translate(0.0, -(SEGREGATION_Y / 2.0 - 24.0), 33.0);

    plate + divider + clean_land + used_land + clean_posts + used_cups - one_way_gate
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_multichannel_pressure_decay_evidence_camera_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-(CAMERA_BRIDGE_X / 2.0 - 38.0), 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_multichannel_pressure_decay_evidence_camera_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 38.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "closed_multichannel_pressure_decay_evidence_camera_bridge_overhead_beam",
        CAMERA_BRIDGE_X,
        34.0,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);
    let front_light_bar = centered_cube(
        "closed_multichannel_pressure_decay_evidence_camera_bridge_front_light_bar",
        CAMERA_BRIDGE_X - 140.0,
        10.0,
        12.0,
    )
    .translate(0.0, -(CAMERA_BRIDGE_Y / 2.0 + 10.0), CAMERA_BRIDGE_Z - 42.0);

    let mut camera_mounts =
        Part::empty("closed_multichannel_pressure_decay_evidence_camera_mounts");
    let mut camera_windows =
        Part::empty("closed_multichannel_pressure_decay_evidence_camera_windows");
    for camera in 0..CAMERA_COUNT {
        let x = lane_x(camera, CAMERA_COUNT, CAMERA_MOUNT_PITCH_X);
        camera_mounts = camera_mounts
            + centered_cube(
                format!("closed_multichannel_pressure_decay_evidence_camera_{camera}_mount_land"),
                88.0,
                34.0,
                10.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z - 34.0);
        camera_windows = camera_windows
            + centered_cube(
                format!("closed_multichannel_pressure_decay_evidence_camera_{camera}_view_window"),
                CAMERA_WINDOW_X,
                CAMERA_WINDOW_Y,
                CAMERA_WINDOW_Z,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z - 14.0);
    }

    let mut datums =
        Part::empty("closed_multichannel_pressure_decay_evidence_camera_bridge_datums");
    for i in 0..CAMERA_DATUMS {
        datums = datums
            + centered_cylinder(
                format!("closed_multichannel_pressure_decay_evidence_camera_bridge_datum_{i}"),
                5.0,
                3.0,
                24,
            )
            .translate(
                lane_x(i % 3, 3, 320.0),
                if i < 3 { -28.0 } else { 28.0 },
                CAMERA_BRIDGE_Z - 58.0,
            );
    }

    left_post + right_post + beam + front_light_bar + camera_mounts + datums - camera_windows
}

fn robot_service_keepout_gauges() -> Part {
    let mut gauges = Part::empty("closed_multichannel_pressure_decay_robot_service_keepout_gauges");
    for (i, (name, x, y, width, depth, height)) in [
        (
            "front_robot_sweep",
            0.0,
            -(STATION_Y / 2.0 + FRONT_ROBOT_SWEEP_CLEARANCE / 2.0),
            STATION_X - 160.0,
            FRONT_ROBOT_SWEEP_CLEARANCE,
            6.0,
        ),
        (
            "rear_manifold_service",
            0.0,
            STATION_Y / 2.0 + REAR_MANIFOLD_SERVICE_CLEARANCE / 2.0,
            STATION_X - 200.0,
            REAR_MANIFOLD_SERVICE_CLEARANCE,
            6.0,
        ),
        (
            "left_orifice_service",
            -(STATION_X / 2.0 + LEFT_ORIFICE_SERVICE_CLEARANCE / 2.0),
            -130.0,
            LEFT_ORIFICE_SERVICE_CLEARANCE,
            360.0,
            6.0,
        ),
        (
            "right_transducer_service",
            STATION_X / 2.0 + RIGHT_TRANSDUCER_SERVICE_CLEARANCE / 2.0,
            -92.0,
            RIGHT_TRANSDUCER_SERVICE_CLEARANCE,
            384.0,
            6.0,
        ),
        (
            "valve_service_lift",
            VALVE_BANK_POS.0,
            VALVE_BANK_POS.1,
            VALVE_BANK_X + 52.0,
            VALVE_BANK_Y + 58.0,
            VALVE_SERVICE_LIFT_CLEARANCE,
        ),
        (
            "camera_bridge_lift",
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            CAMERA_BRIDGE_X + 70.0,
            CAMERA_BRIDGE_Y + 90.0,
            CAMERA_LIFT_CLEARANCE,
        ),
    ]
    .iter()
    .enumerate()
    {
        gauges = gauges
            + centered_cube(
                format!("closed_multichannel_pressure_decay_keepout_gauge_{i}_{name}"),
                *width,
                *depth,
                *height,
            )
            .translate(*x, *y, BASE_Z / 2.0 + height / 2.0);
    }
    let center_cross = centered_cube(
        "closed_multichannel_pressure_decay_robot_keepout_station_centerline_x",
        STATION_X - 180.0,
        4.0,
        6.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + 3.0)
        + centered_cube(
            "closed_multichannel_pressure_decay_robot_keepout_station_centerline_y",
            4.0,
            STATION_Y - 160.0,
            6.0,
        )
        .translate(0.0, 0.0, BASE_Z / 2.0 + 3.0);
    gauges + center_cross
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn channel_row_col(index: usize) -> (usize, usize) {
    (index / CHANNEL_COLS, index % CHANNEL_COLS)
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 74.0)),
        (STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 74.0)),
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 74.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 74.0),
        (-396.0, -(STATION_Y / 2.0 - 74.0)),
        (0.0, -(STATION_Y / 2.0 - 74.0)),
        (396.0, -(STATION_Y / 2.0 - 74.0)),
        (-396.0, STATION_Y / 2.0 - 74.0),
        (0.0, STATION_Y / 2.0 - 74.0),
        (396.0, STATION_Y / 2.0 - 74.0),
    ]
}

fn assert_layout() {
    assert_eq!(CHANNEL_COUNT, 20);
    assert_eq!(CHANNEL_PAIR_PORTS, CHANNEL_COUNT * 2);
    assert_eq!(ORIFICE_PLACEHOLDERS, CHANNEL_COUNT);
    assert_eq!(TRANSDUCER_DOCKS, CHANNEL_COUNT);
    assert_eq!(VALVE_PLACEHOLDERS, CHANNEL_COUNT);
    assert_eq!(VENT_ROUTE_WITNESSES, CHANNEL_COUNT);
    assert_eq!(
        REFERENCE_VOLUME_BLOCKS * VOLUME_CHANNELS_PER_BLOCK,
        CHANNEL_COUNT
    );
    assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= CHANNEL_COUNT);

    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_inside_rims(pos, width, depth),
            "{name} exceeds tray rims"
        );
    }
    assert!(clean_used_gap() >= CLEAN_USED_MIN_GAP);
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        (
            "20_channel_reference_manifold_coupon",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        (
            "calibrated_leak_orifice_placeholder_nest",
            ORIFICE_POS,
            ORIFICE_NEST_X,
            ORIFICE_NEST_Y,
        ),
        (
            "pressure_transducer_dock_panel",
            TRANSDUCER_POS,
            TRANSDUCER_PANEL_X,
            TRANSDUCER_PANEL_Y,
        ),
        (
            "isolation_valve_placeholder_bank",
            VALVE_BANK_POS,
            VALVE_BANK_X,
            VALVE_BANK_Y,
        ),
        (
            "reference_volume_blocks",
            VOLUME_POS,
            VOLUME_BLOCKS_X,
            VOLUME_BLOCKS_Y,
        ),
        (
            "vent_filter_routing_witness_features",
            VENT_ROUTE_POS,
            VENT_ROUTE_X,
            VENT_ROUTE_Y,
        ),
        (
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        (
            "clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        (
            "evidence_camera_bridge_footprint",
            CAMERA_BRIDGE_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_inside_rims(pos: (f64, f64), width: f64, depth: f64) -> bool {
    let usable_half_x = STATION_X / 2.0 - RIM_W - 12.0;
    let usable_half_y = STATION_Y / 2.0 - RIM_W - 12.0;
    pos.0.abs() + width / 2.0 <= usable_half_x && pos.1.abs() + depth / 2.0 <= usable_half_y
}

fn clean_used_gap() -> f64 {
    let clean_right_edge = SEGREGATION_POS.0 - CLEAN_USED_LAND_OFFSET + 104.0 / 2.0;
    let used_left_edge = SEGREGATION_POS.0 + CLEAN_USED_LAND_OFFSET - 104.0 / 2.0;
    used_left_edge - clean_right_edge
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_stable_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_multichannel_pressure_decay_leak_reference_station_"
                ),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_groups_cover_worker_scope() {
        for feature in [
            "base_containment_tray",
            "20_channel_reference_manifold_coupon",
            "calibrated_leak_orifice_placeholder_nest",
            "pressure_transducer_dock_panel",
            "isolation_valve_placeholder_bank",
            "reference_volume_blocks",
            "vent_filter_routing_witness_features",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 13);
    }

    #[test]
    fn limitation_markers_prevent_protocol_or_pressure_rating_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
        assert!(LIMITATIONS.contains(&"not_pressure_rated_system_design"));
        assert!(LIMITATIONS.contains(&"not_leak_acceptance_protocol"));
        assert!(LIMITATIONS.contains(&"bought_transducers_valves_filters_orifices_as_placeholders"));
    }

    #[test]
    fn twenty_channel_reference_topology_is_consistent() {
        assert_eq!(CHANNEL_ROWS, 4);
        assert_eq!(CHANNEL_COLS, 5);
        assert_eq!(CHANNEL_COUNT, 20);
        assert_eq!(CHANNEL_PAIR_PORTS, 40);
        assert_eq!(channel_row_col(0), (0, 0));
        assert_eq!(channel_row_col(19), (3, 4));
        assert!(MANIFOLD_CHANNEL_PITCH_X > CHANNEL_GROOVE_X);
        assert!(PORT_PAD_D > PORT_D);
    }

    #[test]
    fn placeholder_counts_match_channel_count() {
        assert_eq!(ORIFICE_PLACEHOLDERS, CHANNEL_COUNT);
        assert_eq!(TRANSDUCER_DOCKS, CHANNEL_COUNT);
        assert_eq!(VALVE_PLACEHOLDERS, CHANNEL_COUNT);
        assert_eq!(VALVE_ROWS * VALVE_COLS, CHANNEL_COUNT);
        assert_eq!(VENT_ROUTE_WITNESSES, CHANNEL_COUNT);
        assert_eq!(
            REFERENCE_VOLUME_BLOCKS * VOLUME_CHANNELS_PER_BLOCK,
            CHANNEL_COUNT
        );
    }

    #[test]
    fn modules_fit_inside_containment_tray() {
        assert_layout();
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_inside_rims(pos, width, depth));
        }
        assert!(MANIFOLD_X < STATION_X - 2.0 * RIM_W);
        assert!(VALVE_BANK_X < STATION_X - 2.0 * RIM_W);
    }

    #[test]
    fn traceability_and_disposition_capacity_cover_each_channel() {
        assert_eq!(BARCODE_LANDS, CHANNEL_COUNT);
        assert_eq!(CERTIFICATE_LANDS, CHANNEL_GROUPS);
        assert!(RUN_RECORD_LANDS >= STATUS_LANES);
        assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= CHANNEL_COUNT);
    }

    #[test]
    fn clean_used_segregation_and_evidence_bridge_are_visible() {
        assert!(clean_used_gap() >= CLEAN_USED_MIN_GAP);
        assert_eq!(CLEAN_CAP_POSTS, 10);
        assert_eq!(USED_ORIFICE_CUPS, 10);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(CAMERA_DATUMS, 6);
        assert!(CAMERA_BRIDGE_Z >= 200.0);
        assert!(CAMERA_WINDOW_X > 100.0);
    }

    #[test]
    fn robot_and_service_keepout_gauges_are_declared() {
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(FRONT_ROBOT_SWEEP_CLEARANCE >= 380.0);
        assert!(REAR_MANIFOLD_SERVICE_CLEARANCE >= 220.0);
        assert!(LEFT_ORIFICE_SERVICE_CLEARANCE >= 160.0);
        assert!(RIGHT_TRANSDUCER_SERVICE_CLEARANCE >= 180.0);
        assert!(VALVE_SERVICE_LIFT_CLEARANCE >= 150.0);
        assert!(CAMERA_LIFT_CLEARANCE >= 250.0);
    }
}
