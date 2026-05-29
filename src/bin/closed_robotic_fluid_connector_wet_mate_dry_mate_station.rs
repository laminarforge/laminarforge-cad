use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system validation fixture for robotic wet-mate and dry-mate fluid
// connector operations before tissue-chip connection.
//
// Intent:
// - Exercise robotic connector approach, capture, mate, release, and reject
//   handling on a contained no-chip validation deck.
// - Compare wet-mate and dry-mate lanes while observing misalignment, residual
//   drip, trapped bubbles, cap/plug custody, wetness state, force trace, and
//   pressure-decay handoff readiness.
// - Keep evidence capture, custody IDs, and robot/service keepouts physically
//   referenced to the same fixture.
//
// This is mechanical CAD packaging/validation hardware only. It does not define
// a sterile connector design, biological acceptance criterion, or live-chip
// release specification.

const OUTPUTS: [&str; 12] = [
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_containment_deck.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_connector_nest_arrays.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_wet_dry_comparison_lanes.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_cap_plug_parking_matrix.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_drip_capture_moat.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_wetness_bubble_witness_bank.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_force_torque_sensor_pocket.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_pressure_decay_handoff_ports.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_barcode_rfid_custody_lands.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_release_hold_reject_gates.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_evidence_camera_bridge_keepouts.stl",
    "output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 15] = [
    "containment_deck",
    "connector_nest_arrays",
    "wet_mate_lane",
    "dry_mate_lane",
    "cap_plug_parking_matrix",
    "drip_capture_moat",
    "wetness_witness_strips",
    "bubble_sight_windows",
    "force_torque_sensor_pocket",
    "pressure_decay_handoff_ports",
    "barcode_rfid_custody_lands",
    "release_hold_reject_gates",
    "evidence_camera_bridge",
    "robot_keepout",
    "service_keepout",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 790.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const SUMP_X: f64 = 1030.0;
const SUMP_Y: f64 = 635.0;
const SUMP_Z: f64 = 5.5;
const DATUM_COUNT: usize = 8;

const NEST_ARRAY_X: f64 = 570.0;
const NEST_ARRAY_Y: f64 = 255.0;
const NEST_ARRAY_Z: f64 = 38.0;
const NEST_ARRAY_POS: (f64, f64) = (-250.0, 175.0);
const NEST_COLS: usize = 4;
const NEST_ROWS: usize = 2;
const CONNECTOR_NESTS: usize = NEST_COLS * NEST_ROWS;
const NEST_PITCH_X: f64 = 118.0;
const NEST_PITCH_Y: f64 = 92.0;
const CONNECTOR_POCKET_D: f64 = 34.0;
#[cfg(test)]
const MISALIGNMENT_GAUGE_COUNT: usize = CONNECTOR_NESTS;

const LANE_BANK_X: f64 = 760.0;
const LANE_BANK_Y: f64 = 185.0;
const LANE_BANK_Z: f64 = 30.0;
const LANE_BANK_POS: (f64, f64) = (55.0, -20.0);
const COMPARISON_LANES: usize = 2;
const LANE_PITCH_Y: f64 = 78.0;
const WET_LANE_INDEX: usize = 0;
const DRY_LANE_INDEX: usize = 1;
const APPROACH_STATIONS_PER_LANE: usize = 5;
const APPROACH_PITCH_X: f64 = 118.0;

const PARKING_X: f64 = 350.0;
const PARKING_Y: f64 = 225.0;
const PARKING_Z: f64 = 28.0;
const PARKING_POS: (f64, f64) = (405.0, 210.0);
const PARK_ROWS: usize = 3;
const PARK_COLS: usize = 4;
const CAP_PLUG_PARKS: usize = PARK_ROWS * PARK_COLS;
const PARK_PITCH_X: f64 = 72.0;
const PARK_PITCH_Y: f64 = 58.0;
const PARKING_POCKET_D: f64 = 23.0;

const MOAT_X: f64 = 865.0;
const MOAT_Y: f64 = 310.0;
const MOAT_Z: f64 = 16.0;
const MOAT_POS: (f64, f64) = (20.0, -15.0);
const MOAT_W: f64 = 22.0;
const DRIP_WITNESS_PADS: usize = 10;

const WITNESS_X: f64 = 455.0;
const WITNESS_Y: f64 = 138.0;
const WITNESS_Z: f64 = 24.0;
const WITNESS_POS: (f64, f64) = (-355.0, -245.0);
const WETNESS_STRIPS: usize = 6;
const BUBBLE_WINDOWS: usize = 6;
const WITNESS_PITCH_X: f64 = 58.0;

const SENSOR_X: f64 = 235.0;
const SENSOR_Y: f64 = 168.0;
const SENSOR_Z: f64 = 42.0;
const SENSOR_POS: (f64, f64) = (380.0, -95.0);
#[cfg(test)]
const FORCE_TRACE_FIDUCIALS: usize = 4;
const SENSOR_POCKET_DEPTH: f64 = 18.0;

const PRESSURE_BAR_X: f64 = 515.0;
const PRESSURE_BAR_Y: f64 = 88.0;
const PRESSURE_BAR_Z: f64 = 32.0;
const PRESSURE_BAR_POS: (f64, f64) = (115.0, -300.0);
const PRESSURE_PORTS: usize = 8;
const PRESSURE_PORT_PITCH_X: f64 = 56.0;
const PRESSURE_PORT_D: f64 = 12.0;

const CUSTODY_X: f64 = 390.0;
const CUSTODY_Y: f64 = 112.0;
const CUSTODY_Z: f64 = 14.0;
const CUSTODY_POS: (f64, f64) = (-365.0, 325.0);
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;

const GATE_X: f64 = 355.0;
const GATE_Y: f64 = 118.0;
const GATE_Z: f64 = 34.0;
const GATE_POS: (f64, f64) = (385.0, -300.0);
const DISPOSITION_GATES: usize = 3;
const GATE_PITCH_X: f64 = 102.0;

const CAMERA_SPAN_X: f64 = 860.0;
const CAMERA_BEAM_Y: f64 = 40.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_CLEARANCE_Z: f64 = 142.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 58.0;
const CAMERA_MOUNTS: usize = 4;
#[cfg(test)]
const EVIDENCE_FIDUCIALS: usize = 8;
const ROBOT_KEEPOUT_X: f64 = 1085.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 64.0;
const SERVICE_KEEPOUT_X: f64 = 92.0;
const SERVICE_KEEPOUT_Y: f64 = 625.0;
const SERVICE_KEEPOUT_Z: f64 = 82.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(connector_nest_arrays(), OUTPUTS[1]);
    write_part(wet_dry_comparison_lanes(), OUTPUTS[2]);
    write_part(cap_plug_parking_matrix(), OUTPUTS[3]);
    write_part(drip_capture_moat(), OUTPUTS[4]);
    write_part(wetness_bubble_witness_bank(), OUTPUTS[5]);
    write_part(force_torque_sensor_pocket(), OUTPUTS[6]);
    write_part(pressure_decay_handoff_ports(), OUTPUTS[7]);
    write_part(barcode_rfid_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(evidence_camera_bridge_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed robotic fluid connector wet/dry mate station: {:.0}mm x {:.0}mm contained deck, {} connector nests, {} wet/dry lanes, {} approach stations per lane.",
        DECK_X, DECK_Y, CONNECTOR_NESTS, COMPARISON_LANES, APPROACH_STATIONS_PER_LANE
    );
    println!(
        "Validation witnesses: {} cap/plug custody parks, {} drip pads, {} wetness strips, {} bubble windows, {} pressure-decay ports, {} disposition gates.",
        CAP_PLUG_PARKS,
        DRIP_WITNESS_PADS,
        WETNESS_STRIPS,
        BUBBLE_WINDOWS,
        PRESSURE_PORTS,
        DISPOSITION_GATES
    );
    println!(
        "Evidence packaging: force/torque pocket, {} barcode lands, {} RFID lands, {} camera mounts, robot/service keepout gauges.",
        BARCODE_LANDS, RFID_LANDS, CAMERA_MOUNTS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + connector_nest_arrays()
        + wet_dry_comparison_lanes()
        + cap_plug_parking_matrix()
        + drip_capture_moat()
        + wetness_bubble_witness_bank()
        + force_torque_sensor_pocket()
        + pressure_decay_handoff_ports()
        + barcode_rfid_custody_lands()
        + release_hold_reject_gates()
        + evidence_camera_bridge_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_robotic_connector_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_robotic_connector_station_sump_recess",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -12.0, DECK_Z / 2.0 - SUMP_Z / 2.0);
    let drain = centered_cylinder(
        "closed_robotic_connector_station_front_drain_cut",
        9.0,
        RIM_W + 30.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 12.0, 0.0);

    deck - sump - drain + containment_rims() + datum_bosses() + zone_engraving_pads()
}

fn containment_rims() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    centered_cube(
        "closed_robotic_connector_station_front_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            "closed_robotic_connector_station_rear_rim",
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            "closed_robotic_connector_station_left_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            "closed_robotic_connector_station_right_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn datum_bosses() -> Part {
    let mut bosses = Part::empty("closed_robotic_connector_station_datum_bosses");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_robotic_connector_station_datum_boss_{i}"),
            13.0,
            10.0,
            32,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        let bore = centered_cylinder(
            format!("closed_robotic_connector_station_datum_bore_{i}"),
            4.1,
            12.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn zone_engraving_pads() -> Part {
    let zones = [
        (
            NEST_ARRAY_POS.0,
            NEST_ARRAY_POS.1,
            NEST_ARRAY_X + 22.0,
            NEST_ARRAY_Y + 20.0,
        ),
        (
            LANE_BANK_POS.0,
            LANE_BANK_POS.1,
            LANE_BANK_X + 28.0,
            LANE_BANK_Y + 26.0,
        ),
        (
            PARKING_POS.0,
            PARKING_POS.1,
            PARKING_X + 20.0,
            PARKING_Y + 20.0,
        ),
        (
            WITNESS_POS.0,
            WITNESS_POS.1,
            WITNESS_X + 18.0,
            WITNESS_Y + 18.0,
        ),
        (SENSOR_POS.0, SENSOR_POS.1, SENSOR_X + 20.0, SENSOR_Y + 20.0),
        (
            PRESSURE_BAR_POS.0,
            PRESSURE_BAR_POS.1,
            PRESSURE_BAR_X + 18.0,
            PRESSURE_BAR_Y + 18.0,
        ),
        (
            CUSTODY_POS.0,
            CUSTODY_POS.1,
            CUSTODY_X + 16.0,
            CUSTODY_Y + 16.0,
        ),
        (GATE_POS.0, GATE_POS.1, GATE_X + 18.0, GATE_Y + 18.0),
    ];

    let mut pads = Part::empty("closed_robotic_connector_station_zone_reference_pads");
    for (i, (x, y, sx, sy)) in zones.iter().enumerate() {
        pads = pads
            + centered_cube(
                format!("closed_robotic_connector_station_zone_pad_{i}"),
                *sx,
                *sy,
                2.2,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 1.1);
    }
    pads
}

fn connector_nest_arrays() -> Part {
    let base = centered_cube(
        "closed_robotic_connector_station_connector_nest_array_plate",
        NEST_ARRAY_X,
        NEST_ARRAY_Y,
        NEST_ARRAY_Z,
    )
    .translate(NEST_ARRAY_POS.0, NEST_ARRAY_POS.1, top_z(NEST_ARRAY_Z));

    let mut cuts = Part::empty("closed_robotic_connector_station_connector_nest_cuts");
    let mut keys = Part::empty("closed_robotic_connector_station_misalignment_key_gauges");
    for idx in 0..CONNECTOR_NESTS {
        let (x, y) = connector_nest_center(idx);
        cuts =
            cuts + centered_cylinder(
                format!("closed_robotic_connector_station_connector_socket_{idx}"),
                CONNECTOR_POCKET_D / 2.0,
                NEST_ARRAY_Z + 2.0,
                48,
            )
            .translate(
                NEST_ARRAY_POS.0 + x,
                NEST_ARRAY_POS.1 + y,
                top_z(NEST_ARRAY_Z),
            ) + centered_cube(
                format!("closed_robotic_connector_station_keyway_relief_{idx}"),
                10.0,
                31.0,
                NEST_ARRAY_Z + 3.0,
            )
            .translate(
                NEST_ARRAY_POS.0 + x + CONNECTOR_POCKET_D / 2.0,
                NEST_ARRAY_POS.1 + y,
                top_z(NEST_ARRAY_Z),
            );

        keys = keys
            + centered_cube(
                format!("closed_robotic_connector_station_misalignment_flag_{idx}"),
                32.0,
                5.0,
                12.0,
            )
            .translate(
                NEST_ARRAY_POS.0 + x,
                NEST_ARRAY_POS.1 + y + 32.0,
                DECK_Z / 2.0 + NEST_ARRAY_Z + 6.0,
            );
    }

    base - cuts
        + keys
        + lane_label_tabs(
            "closed_robotic_connector_station_connector_nest_lane",
            NEST_ARRAY_POS,
            CONNECTOR_NESTS,
        )
}

fn wet_dry_comparison_lanes() -> Part {
    let bank = centered_cube(
        "closed_robotic_connector_station_wet_dry_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(LANE_BANK_POS.0, LANE_BANK_POS.1, top_z(LANE_BANK_Z));

    let mut channels = Part::empty("closed_robotic_connector_station_lane_channel_cuts");
    let mut stations = Part::empty("closed_robotic_connector_station_approach_station_blocks");
    for lane in 0..COMPARISON_LANES {
        let lane_y = lane_center_y(lane);
        channels = channels
            + centered_cube(
                format!("closed_robotic_connector_station_lane_channel_{lane}"),
                LANE_BANK_X - 60.0,
                27.0,
                LANE_BANK_Z + 2.0,
            )
            .translate(
                LANE_BANK_POS.0,
                LANE_BANK_POS.1 + lane_y,
                top_z(LANE_BANK_Z),
            );
        for station in 0..APPROACH_STATIONS_PER_LANE {
            let x = station_center_x(station);
            stations = stations
                + centered_cube(
                    format!(
                        "closed_robotic_connector_station_lane_{lane}_approach_datum_{station}"
                    ),
                    36.0,
                    42.0,
                    18.0,
                )
                .translate(
                    LANE_BANK_POS.0 + x,
                    LANE_BANK_POS.1 + lane_y,
                    DECK_Z / 2.0 + LANE_BANK_Z + 9.0,
                );
        }
    }

    let wet_rail = centered_cube(
        "closed_robotic_connector_station_wet_mate_lane_residual_fluid_rail",
        LANE_BANK_X - 75.0,
        10.0,
        20.0,
    )
    .translate(
        LANE_BANK_POS.0,
        LANE_BANK_POS.1 + lane_center_y(WET_LANE_INDEX) - 25.0,
        DECK_Z / 2.0 + LANE_BANK_Z + 10.0,
    );
    let dry_rail = centered_cube(
        "closed_robotic_connector_station_dry_mate_lane_dryness_datum_rail",
        LANE_BANK_X - 75.0,
        10.0,
        20.0,
    )
    .translate(
        LANE_BANK_POS.0,
        LANE_BANK_POS.1 + lane_center_y(DRY_LANE_INDEX) + 25.0,
        DECK_Z / 2.0 + LANE_BANK_Z + 10.0,
    );

    bank - channels + stations + wet_rail + dry_rail
}

fn cap_plug_parking_matrix() -> Part {
    let plate = centered_cube(
        "closed_robotic_connector_station_cap_plug_parking_matrix_plate",
        PARKING_X,
        PARKING_Y,
        PARKING_Z,
    )
    .translate(PARKING_POS.0, PARKING_POS.1, top_z(PARKING_Z));

    let mut pockets = Part::empty("closed_robotic_connector_station_cap_plug_parking_pockets");
    let mut custody_ticks = Part::empty("closed_robotic_connector_station_cap_plug_custody_ticks");
    for idx in 0..CAP_PLUG_PARKS {
        let (x, y) = parking_center(idx);
        pockets = pockets
            + centered_cylinder(
                format!("closed_robotic_connector_station_cap_plug_pocket_{idx}"),
                PARKING_POCKET_D / 2.0,
                PARKING_Z + 2.0,
                32,
            )
            .translate(PARKING_POS.0 + x, PARKING_POS.1 + y, top_z(PARKING_Z));
        custody_ticks = custody_ticks
            + centered_cube(
                format!("closed_robotic_connector_station_cap_plug_custody_tick_{idx}"),
                24.0,
                3.5,
                4.0,
            )
            .translate(
                PARKING_POS.0 + x,
                PARKING_POS.1 + y - 24.0,
                DECK_Z / 2.0 + PARKING_Z + 2.0,
            );
    }

    plate - pockets + custody_ticks
}

fn drip_capture_moat() -> Part {
    let outer = centered_cube(
        "closed_robotic_connector_station_drip_capture_moat_outer_rim",
        MOAT_X,
        MOAT_Y,
        MOAT_Z,
    )
    .translate(MOAT_POS.0, MOAT_POS.1, top_z(MOAT_Z));
    let inner_cut = centered_cube(
        "closed_robotic_connector_station_drip_capture_moat_void",
        MOAT_X - MOAT_W * 2.0,
        MOAT_Y - MOAT_W * 2.0,
        MOAT_Z + 2.0,
    )
    .translate(MOAT_POS.0, MOAT_POS.1, top_z(MOAT_Z));
    let drain_groove = centered_cube(
        "closed_robotic_connector_station_drip_capture_moat_front_drain_groove",
        84.0,
        MOAT_W + 6.0,
        MOAT_Z + 2.0,
    )
    .translate(
        MOAT_POS.0 + MOAT_X / 2.0 - 118.0,
        MOAT_POS.1 - MOAT_Y / 2.0 + MOAT_W / 2.0,
        top_z(MOAT_Z),
    );

    outer - inner_cut - drain_groove + drip_witness_pads()
}

fn drip_witness_pads() -> Part {
    let mut pads = Part::empty("closed_robotic_connector_station_drip_capture_witness_pads");
    let start_x = -(DRIP_WITNESS_PADS as f64 - 1.0) * 70.0 / 2.0;
    for idx in 0..DRIP_WITNESS_PADS {
        pads = pads
            + centered_cube(
                format!("closed_robotic_connector_station_residual_drip_pad_{idx}"),
                42.0,
                16.0,
                3.0,
            )
            .translate(
                MOAT_POS.0 + start_x + idx as f64 * 70.0,
                MOAT_POS.1 - MOAT_Y / 2.0 + 54.0,
                DECK_Z / 2.0 + MOAT_Z + 1.5,
            );
    }
    pads
}

fn wetness_bubble_witness_bank() -> Part {
    let plate = centered_cube(
        "closed_robotic_connector_station_wetness_bubble_witness_bank",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(WITNESS_POS.0, WITNESS_POS.1, top_z(WITNESS_Z));

    let mut cuts = Part::empty("closed_robotic_connector_station_bubble_window_cuts");
    let mut strips = Part::empty("closed_robotic_connector_station_wetness_witness_strips");
    let start_x = -(BUBBLE_WINDOWS as f64 - 1.0) * WITNESS_PITCH_X / 2.0;
    for idx in 0..BUBBLE_WINDOWS {
        let x = WITNESS_POS.0 + start_x + idx as f64 * WITNESS_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("closed_robotic_connector_station_bubble_sight_window_{idx}"),
                17.0,
                WITNESS_Z + 2.0,
                40,
            )
            .translate(x, WITNESS_POS.1 + 28.0, top_z(WITNESS_Z));
    }
    for idx in 0..WETNESS_STRIPS {
        let x = WITNESS_POS.0 + start_x + idx as f64 * WITNESS_PITCH_X;
        strips = strips
            + centered_cube(
                format!("closed_robotic_connector_station_wetness_witness_strip_{idx}"),
                42.0,
                12.0,
                4.0,
            )
            .translate(x, WITNESS_POS.1 - 34.0, DECK_Z / 2.0 + WITNESS_Z + 2.0);
    }

    plate - cuts + strips
}

fn force_torque_sensor_pocket() -> Part {
    let body = centered_cube(
        "closed_robotic_connector_station_force_torque_sensor_mount_block",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, top_z(SENSOR_Z));
    let pocket = centered_cube(
        "closed_robotic_connector_station_force_torque_sensor_pocket",
        SENSOR_X - 62.0,
        SENSOR_Y - 58.0,
        SENSOR_POCKET_DEPTH + 1.0,
    )
    .translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        top_z(SENSOR_Z) + SENSOR_Z / 2.0 - SENSOR_POCKET_DEPTH / 2.0,
    );
    let wire_exit = centered_cube(
        "closed_robotic_connector_station_force_trace_wire_exit",
        74.0,
        18.0,
        20.0,
    )
    .translate(
        SENSOR_POS.0 + SENSOR_X / 2.0 - 28.0,
        SENSOR_POS.1 - SENSOR_Y / 2.0,
        DECK_Z / 2.0 + SENSOR_Z - 4.0,
    );

    body - pocket - wire_exit + force_trace_fiducials()
}

fn force_trace_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_robotic_connector_station_force_trace_fiducials");
    for (i, (x, y)) in [(-80.0, -54.0), (80.0, -54.0), (-80.0, 54.0), (80.0, 54.0)]
        .iter()
        .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_robotic_connector_station_force_trace_fiducial_{i}"),
                6.0,
                4.0,
                24,
            )
            .translate(
                SENSOR_POS.0 + *x,
                SENSOR_POS.1 + *y,
                DECK_Z / 2.0 + SENSOR_Z + 2.0,
            );
    }
    fiducials
}

fn pressure_decay_handoff_ports() -> Part {
    let bar = centered_cube(
        "closed_robotic_connector_station_pressure_decay_handoff_bar",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    )
    .translate(
        PRESSURE_BAR_POS.0,
        PRESSURE_BAR_POS.1,
        top_z(PRESSURE_BAR_Z),
    );

    let mut ports = Part::empty("closed_robotic_connector_station_pressure_decay_port_cuts");
    let mut collars = Part::empty("closed_robotic_connector_station_pressure_decay_port_collars");
    let start_x = -(PRESSURE_PORTS as f64 - 1.0) * PRESSURE_PORT_PITCH_X / 2.0;
    for idx in 0..PRESSURE_PORTS {
        let x = PRESSURE_BAR_POS.0 + start_x + idx as f64 * PRESSURE_PORT_PITCH_X;
        ports = ports
            + centered_cylinder(
                format!("closed_robotic_connector_station_pressure_decay_handoff_port_{idx}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_BAR_Z + 2.0,
                32,
            )
            .translate(x, PRESSURE_BAR_POS.1, top_z(PRESSURE_BAR_Z));
        collars = collars
            + centered_cylinder(
                format!("closed_robotic_connector_station_pressure_decay_port_collar_{idx}"),
                16.0,
                8.0,
                32,
            )
            .translate(x, PRESSURE_BAR_POS.1, DECK_Z / 2.0 + PRESSURE_BAR_Z + 4.0);
    }

    bar - ports + collars
}

fn barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_robotic_connector_station_barcode_rfid_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z));

    let mut lands = Part::empty("closed_robotic_connector_station_custody_lands");
    for idx in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_robotic_connector_station_barcode_land_{idx}"),
                48.0,
                22.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 145.0 + idx as f64 * 58.0,
                CUSTODY_POS.1 + 26.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    for idx in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_robotic_connector_station_rfid_land_{idx}"),
                62.0,
                26.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 117.0 + idx as f64 * 78.0,
                CUSTODY_POS.1 - 30.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }

    panel + lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_robotic_connector_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z));

    let mut troughs = Part::empty("closed_robotic_connector_station_disposition_gate_troughs");
    let mut paddles = Part::empty("closed_robotic_connector_station_disposition_gate_paddles");
    let start_x = -(DISPOSITION_GATES as f64 - 1.0) * GATE_PITCH_X / 2.0;
    for idx in 0..DISPOSITION_GATES {
        let x = GATE_POS.0 + start_x + idx as f64 * GATE_PITCH_X;
        troughs = troughs
            + centered_cube(
                format!("closed_robotic_connector_station_disposition_trough_{idx}"),
                72.0,
                58.0,
                GATE_Z + 2.0,
            )
            .translate(x, GATE_POS.1, top_z(GATE_Z));
        paddles = paddles
            + centered_cube(
                format!("closed_robotic_connector_station_disposition_gate_paddle_{idx}"),
                9.0,
                86.0,
                40.0,
            )
            .translate(x + 38.0, GATE_POS.1, DECK_Z / 2.0 + GATE_Z + 20.0);
    }

    base - troughs + paddles
}

fn evidence_camera_bridge_keepouts() -> Part {
    let post_z = CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z;
    let left_post = centered_cube(
        "closed_robotic_connector_station_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(-CAMERA_SPAN_X / 2.0, -42.0, DECK_Z / 2.0 + post_z / 2.0);
    let right_post = centered_cube(
        "closed_robotic_connector_station_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(CAMERA_SPAN_X / 2.0, -42.0, DECK_Z / 2.0 + post_z / 2.0);
    let beam = centered_cube(
        "closed_robotic_connector_station_evidence_camera_bridge_beam",
        CAMERA_SPAN_X + CAMERA_POST_X,
        CAMERA_BEAM_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        -42.0,
        DECK_Z / 2.0 + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_mount_tabs() + evidence_fiducials() + keepout_gauges()
}

fn camera_mount_tabs() -> Part {
    let mut tabs = Part::empty("closed_robotic_connector_station_camera_mount_tabs");
    let start_x = -(CAMERA_MOUNTS as f64 - 1.0) * 170.0 / 2.0;
    for idx in 0..CAMERA_MOUNTS {
        tabs = tabs
            + centered_cube(
                format!("closed_robotic_connector_station_camera_mount_tab_{idx}"),
                62.0,
                28.0,
                8.0,
            )
            .translate(
                start_x + idx as f64 * 170.0,
                -42.0,
                DECK_Z / 2.0 + CAMERA_CLEARANCE_Z - 4.0,
            );
    }
    tabs
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_robotic_connector_station_evidence_fiducials");
    let positions = [
        (-500.0, -340.0),
        (-360.0, -340.0),
        (-500.0, 340.0),
        (-360.0, 340.0),
        (360.0, -340.0),
        (500.0, -340.0),
        (360.0, 340.0),
        (500.0, 340.0),
    ];
    for (idx, (x, y)) in positions.iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_robotic_connector_station_evidence_fiducial_{idx}"),
                7.0,
                4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 2.0);
    }
    fiducials
}

fn keepout_gauges() -> Part {
    let robot = centered_cube(
        "closed_robotic_connector_station_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 92.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let service = centered_cube(
        "closed_robotic_connector_station_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 82.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );

    robot + service
}

fn lane_label_tabs(prefix: &str, origin: (f64, f64), count: usize) -> Part {
    let mut tabs = Part::empty(format!("{prefix}_label_tabs"));
    for idx in 0..count {
        let (x, y) = connector_nest_center(idx);
        tabs = tabs
            + centered_cube(format!("{prefix}_tab_{idx}"), 34.0, 10.0, 4.0).translate(
                origin.0 + x,
                origin.1 + y - 34.0,
                DECK_Z / 2.0 + NEST_ARRAY_Z + 2.0,
            );
    }
    tabs
}

fn datum_positions() -> [(f64, f64); DATUM_COUNT] {
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

fn connector_nest_center(index: usize) -> (f64, f64) {
    let col = index % NEST_COLS;
    let row = index / NEST_COLS;
    grid_center(col, row, NEST_COLS, NEST_ROWS, NEST_PITCH_X, NEST_PITCH_Y)
}

fn parking_center(index: usize) -> (f64, f64) {
    let col = index % PARK_COLS;
    let row = index / PARK_COLS;
    grid_center(col, row, PARK_COLS, PARK_ROWS, PARK_PITCH_X, PARK_PITCH_Y)
}

fn grid_center(
    col: usize,
    row: usize,
    cols: usize,
    rows: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    (
        (col as f64 - (cols as f64 - 1.0) / 2.0) * pitch_x,
        (row as f64 - (rows as f64 - 1.0) / 2.0) * pitch_y,
    )
}

fn lane_center_y(lane: usize) -> f64 {
    (lane as f64 - (COMPARISON_LANES as f64 - 1.0) / 2.0) * LANE_PITCH_Y
}

fn station_center_x(station: usize) -> f64 {
    (station as f64 - (APPROACH_STATIONS_PER_LANE as f64 - 1.0) / 2.0) * APPROACH_PITCH_X
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert_eq!(CONNECTOR_NESTS, NEST_COLS * NEST_ROWS);
    assert_eq!(CAP_PLUG_PARKS, PARK_COLS * PARK_ROWS);
    assert!(COMPARISON_LANES == 2);
    assert!(PRESSURE_PORTS >= CONNECTOR_NESTS);
    assert!(CAMERA_CLEARANCE_Z > SENSOR_Z + DECK_Z);
    assert!(MOAT_X > LANE_BANK_X);
    assert!(MOAT_Y > LANE_BANK_Y);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_deterministic_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_robotic_fluid_connector_wet_mate_dry_mate_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn feature_metadata_covers_requested_validation_modes() {
        for feature in REQUIRED_FEATURES {
            assert!(!feature.is_empty());
        }
        assert_eq!(REQUIRED_FEATURES.len(), 15);
        assert!(REQUIRED_FEATURES.contains(&"wetness_witness_strips"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_sight_windows"));
        assert!(REQUIRED_FEATURES.contains(&"force_torque_sensor_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_handoff_ports"));
    }

    #[test]
    fn connector_and_custody_capacity_match_validation_flow() {
        assert_eq!(CONNECTOR_NESTS, 8);
        assert_eq!(MISALIGNMENT_GAUGE_COUNT, CONNECTOR_NESTS);
        assert!(PRESSURE_PORTS >= CONNECTOR_NESTS);
        assert!(CAP_PLUG_PARKS > CONNECTOR_NESTS);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(FORCE_TRACE_FIDUCIALS, 4);
        assert_eq!(EVIDENCE_FIDUCIALS, 8);
    }

    #[test]
    fn grid_helpers_keep_pockets_inside_their_plates() {
        let first_nest = connector_nest_center(0);
        let last_nest = connector_nest_center(CONNECTOR_NESTS - 1);
        assert!(first_nest.0.abs() < NEST_ARRAY_X / 2.0 - CONNECTOR_POCKET_D);
        assert!(last_nest.0.abs() < NEST_ARRAY_X / 2.0 - CONNECTOR_POCKET_D);
        assert!(first_nest.1.abs() < NEST_ARRAY_Y / 2.0 - CONNECTOR_POCKET_D);
        assert!(last_nest.1.abs() < NEST_ARRAY_Y / 2.0 - CONNECTOR_POCKET_D);

        let last_park = parking_center(CAP_PLUG_PARKS - 1);
        assert!(last_park.0.abs() < PARKING_X / 2.0 - PARKING_POCKET_D);
        assert!(last_park.1.abs() < PARKING_Y / 2.0 - PARKING_POCKET_D);
    }

    #[test]
    fn wet_and_dry_lanes_are_separate_and_inside_drip_moat() {
        let wet_y = lane_center_y(WET_LANE_INDEX);
        let dry_y = lane_center_y(DRY_LANE_INDEX);
        assert!((dry_y - wet_y).abs() >= LANE_PITCH_Y);
        assert!(station_center_x(0).abs() < LANE_BANK_X / 2.0 - 80.0);
        assert!(station_center_x(APPROACH_STATIONS_PER_LANE - 1).abs() < LANE_BANK_X / 2.0 - 80.0);
        assert!(LANE_BANK_X + 2.0 * MOAT_W < MOAT_X);
        assert!(LANE_BANK_Y + 2.0 * MOAT_W < MOAT_Y);
    }
}
