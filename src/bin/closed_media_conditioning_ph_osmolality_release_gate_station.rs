use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media conditioning pH/osmolality release gate station.
//
// Research assumptions captured in geometry:
// - 21 CFR 211.160 frames laboratory controls around established specifications,
//   standards, sampling plans, and scientifically sound test methods.
// - FDA process validation guidance emphasizes maintaining a controlled process
//   lifecycle and documenting analytical methodology before release.
// - USP pH, osmolality/osmolarity, and conductivity chapters map this station to
//   traceable pH buffers, osmometer controls, conductivity standards, and
//   temperature-aware measurement handling.
//
// This is packaging/interface CAD for a closed pre-perfusion media release gate.
// It does not define acceptance limits, measurement algorithms, sterile connector
// selection, biological release criteria, or GMP validation claims.

const OUTPUT_PREFIX: &str = "closed_media_conditioning_ph_osmolality_release_gate_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_leak_tray_base.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_sample_custody_receiver.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_sensor_measurement_manifold.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_temperature_equilibration_block.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_calibration_standard_rack.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_flush_waste_routing_manifold.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_release_hold_reject_decision_gate.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_chain_of_custody_evidence_lands.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_evidence_camera_bridge.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_robot_service_keepout_gauges.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_flow_path_custody_overlay.stl",
    "output/closed_media_conditioning_ph_osmolality_release_gate_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "closed_sample_custody",
    "ph_sample_lane",
    "osmolality_sample_lane",
    "conductivity_sample_lane",
    "calibration_standards",
    "temperature_equilibration",
    "flush_waste_routing",
    "release_lane",
    "hold_lane",
    "reject_lane",
    "evidence_capture",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const SUMP_DEPTH: f64 = 8.0;
const SOCKET_DEPTH: f64 = 6.0;
const DRAIN_PORT_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 5.4;

const SAMPLE_CENTER: (f64, f64) = (-365.0, 160.0);
const SAMPLE_X: f64 = 360.0;
const SAMPLE_Y: f64 = 220.0;
const SAMPLE_Z: f64 = 54.0;
const ANALYTE_LANES: usize = 3;
const CUSTODY_REPLICATES: usize = 4;
const SAMPLE_WELLS: usize = ANALYTE_LANES * CUSTODY_REPLICATES;
const SAMPLE_COLS: usize = 4;
const SAMPLE_PITCH_X: f64 = 74.0;
const SAMPLE_PITCH_Y: f64 = 54.0;
const SAMPLE_WELL_D: f64 = 16.0;
const SAMPLE_SEAL_RIM_D: f64 = 23.0;

const MEASURE_CENTER: (f64, f64) = (70.0, 175.0);
const MEASURE_X: f64 = 430.0;
const MEASURE_Y: f64 = 220.0;
const MEASURE_Z: f64 = 58.0;
const SENSOR_COUNT: usize = 3;
const SENSOR_PITCH_X: f64 = 112.0;
const SENSOR_WELL_D: f64 = 30.0;
const SAMPLE_LOOP_PORTS: usize = 6;
const VALVE_COUNT: usize = 6;
const FLOW_BORE_D: f64 = 6.4;

const TEMP_CENTER: (f64, f64) = (-365.0, -155.0);
const TEMP_X: f64 = 340.0;
const TEMP_Y: f64 = 190.0;
const TEMP_Z: f64 = 52.0;
const TEMP_POCKETS: usize = 12;
const TEMP_COLS: usize = 4;
const TEMP_PITCH_X: f64 = 68.0;
const TEMP_PITCH_Y: f64 = 48.0;
const TEMP_VIAL_D: f64 = 18.0;
const THERMAL_CHANNELS: usize = 4;
const THERMOWELLS: usize = 3;

const CAL_CENTER: (f64, f64) = (20.0, -170.0);
const CAL_X: f64 = 330.0;
const CAL_Y: f64 = 180.0;
const CAL_Z: f64 = 50.0;
const CAL_STANDARD_FAMILIES: usize = 3;
const CAL_LEVELS: usize = 4;
const CAL_WELLS: usize = CAL_STANDARD_FAMILIES * CAL_LEVELS;
const CAL_COLS: usize = 4;
const CAL_PITCH_X: f64 = 64.0;
const CAL_PITCH_Y: f64 = 48.0;
const CAL_VIAL_D: f64 = 15.0;
const COA_CARD_SLOTS: usize = 3;

const ROUTE_CENTER: (f64, f64) = (455.0, -155.0);
const ROUTE_X: f64 = 260.0;
const ROUTE_Y: f64 = 320.0;
const ROUTE_Z: f64 = 44.0;
const FLUSH_PORTS: usize = 6;
const WASTE_CHANNELS: usize = 3;
const WASTE_BOTTLES: usize = 3;
const ROUTE_PORT_D: f64 = 10.0;
const WASTE_BOTTLE_D: f64 = 38.0;

const DECISION_CENTER: (f64, f64) = (455.0, 170.0);
const DECISION_X: f64 = 260.0;
const DECISION_Y: f64 = 220.0;
const DECISION_Z: f64 = 48.0;
const DECISION_LANES: usize = 3;
const DECISION_SLOTS_PER_LANE: usize = 4;
const DECISION_SLOTS: usize = DECISION_LANES * DECISION_SLOTS_PER_LANE;
const DECISION_SLOT_X: f64 = 58.0;
const DECISION_SLOT_Y: f64 = 34.0;
const DECISION_LANE_PITCH_X: f64 = 78.0;
const DECISION_SLOT_PITCH_Y: f64 = 42.0;

const CUSTODY_LANDS: usize = 12;
const RFID_LANDS: usize = 4;
const RESULT_CARD_SLOTS: usize = 6;
const WITNESS_TOKEN_BARS: usize = 9;

const BRIDGE_CENTER: (f64, f64) = (0.0, 20.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_CLEARANCE_Z: f64 = 210.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_COUNT: usize = 4;
const LED_SEGMENTS: usize = 8;

const FRONT_ROBOT_CLEARANCE: f64 = 380.0;
const REAR_STANDARD_SERVICE_CLEARANCE: f64 = 240.0;
const LEFT_SAMPLE_SERVICE_CLEARANCE: f64 = 200.0;
const RIGHT_WASTE_SERVICE_CLEARANCE: f64 = 260.0;
const ROBOT_KEEP_OUT_Z: f64 = 270.0;
const SERVICE_KEEPOUTS: usize = 4;
const ROBOT_DATUMS: usize = 8;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let sample = sample_custody_receiver();
    export(OUTPUTS[1], &sample);

    let measurement = sensor_measurement_manifold();
    export(OUTPUTS[2], &measurement);

    let temperature = temperature_equilibration_block();
    export(OUTPUTS[3], &temperature);

    let calibration = calibration_standard_rack();
    export(OUTPUTS[4], &calibration);

    let route = flush_waste_routing_manifold();
    export(OUTPUTS[5], &route);

    let decision = release_hold_reject_decision_gate();
    export(OUTPUTS[6], &decision);

    let custody_lands = chain_of_custody_evidence_lands();
    export(OUTPUTS[7], &custody_lands);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[9], &keepouts);

    let overlay = flow_path_custody_overlay();
    export(OUTPUTS[10], &overlay);

    let assembly = base
        + sample.translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, deck_insert_z(SAMPLE_Z))
        + measurement.translate(MEASURE_CENTER.0, MEASURE_CENTER.1, deck_insert_z(MEASURE_Z))
        + temperature.translate(TEMP_CENTER.0, TEMP_CENTER.1, deck_insert_z(TEMP_Z))
        + calibration.translate(CAL_CENTER.0, CAL_CENTER.1, deck_insert_z(CAL_Z))
        + route.translate(ROUTE_CENTER.0, ROUTE_CENTER.1, deck_insert_z(ROUTE_Z))
        + decision.translate(
            DECISION_CENTER.0,
            DECISION_CENTER.1,
            deck_insert_z(DECISION_Z),
        )
        + custody_lands.translate(0.0, 0.0, BASE_Z + 3.0)
        + bridge.translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z / 2.0)
        + keepouts.translate(0.0, 0.0, BASE_Z + 3.0)
        + overlay.translate(0.0, 0.0, BASE_Z + 5.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media conditioning pH/osmolality release gate station:");
    println!("  Output prefix:               {OUTPUT_PREFIX}");
    println!("  Deck/leak tray:              {STATION_X:.0}mm x {STATION_Y:.0}mm");
    println!(
        "  Sample custody:              {SAMPLE_WELLS} sealed wells across pH, osmolality, and conductivity lanes"
    );
    println!(
        "  Measurement manifold:        {SENSOR_COUNT} sensor/analyzer pockets, {SAMPLE_LOOP_PORTS} sample-loop ports, {VALVE_COUNT} isolate/flush valves"
    );
    println!(
        "  Temperature equilibration:   {TEMP_POCKETS} closed sample pockets, {THERMAL_CHANNELS} thermal channels, {THERMOWELLS} thermowells"
    );
    println!(
        "  Calibration custody:         {CAL_WELLS} standards for pH buffers, osmometer controls, and conductivity standards"
    );
    println!(
        "  Routing/decision lanes:      {FLUSH_PORTS} flush ports, {WASTE_CHANNELS} waste channels, {DECISION_LANES} release/hold/reject lanes with {DECISION_SLOTS} slots"
    );
    println!(
        "  Evidence/service:            {CUSTODY_LANDS} custody barcode lands, {RFID_LANDS} RFID lands, {CAMERA_COUNT} cameras, {SERVICE_KEEPOUTS} service keepouts"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    for (name, center, width, depth) in module_specs() {
        assert!(
            fits_on_station(center, width, depth, 14.0),
            "{name} exceeds station envelope"
        );
    }

    let modules = module_specs();
    for a in 0..modules.len() {
        for b in (a + 1)..modules.len() {
            assert!(
                !rects_overlap(
                    rect(modules[a].1, modules[a].2, modules[a].3),
                    rect(modules[b].1, modules[b].2, modules[b].3),
                ),
                "{} overlaps {}",
                modules[a].0,
                modules[b].0
            );
        }
    }

    assert_eq!(ANALYTE_LANES, 3);
    assert_eq!(DECISION_LANES, 3);
    assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
    assert!(RIGHT_WASTE_SERVICE_CLEARANCE >= 240.0);
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 6] {
    [
        ("sample_custody_receiver", SAMPLE_CENTER, SAMPLE_X, SAMPLE_Y),
        (
            "sensor_measurement_manifold",
            MEASURE_CENTER,
            MEASURE_X,
            MEASURE_Y,
        ),
        (
            "temperature_equilibration_block",
            TEMP_CENTER,
            TEMP_X,
            TEMP_Y,
        ),
        ("calibration_standard_rack", CAL_CENTER, CAL_X, CAL_Y),
        (
            "flush_waste_routing_manifold",
            ROUTE_CENTER,
            ROUTE_X,
            ROUTE_Y,
        ),
        (
            "release_hold_reject_decision_gate",
            DECISION_CENTER,
            DECISION_X,
            DECISION_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - margin
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - margin
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.1 && a.1 > b.0 && a.2 < b.3 && a.3 > b.2
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "closed_media_release_gate_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sump = centered_cube(
        "closed_media_release_gate_recessed_leak_sump",
        STATION_X - 124.0,
        STATION_Y - 112.0,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z - SUMP_DEPTH / 2.0);

    let drain = centered_cylinder(
        "closed_media_release_gate_front_drain_port",
        DRAIN_PORT_D / 2.0,
        58.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 88.0,
        -STATION_Y / 2.0 + 16.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain - module_socket_recesses()
        + perimeter_rims()
        + leak_witness_ribs()
        + module_mount_bosses()
        + row_boundary_rails()
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("closed_media_release_gate_module_socket_recesses");
    for (name, center, width, depth) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_release_gate_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(center.0, center.1, BASE_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_media_release_gate_left_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_media_release_gate_right_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_media_release_gate_rear_raised_lip",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_media_release_gate_front_low_robot_lip",
        STATION_X - 170.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 10.0, BASE_Z + 10.0);

    left + right + rear + front
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_release_gate_leak_witness_ribs");
    for (i, x) in [-500.0, -340.0, -180.0, -20.0, 140.0, 300.0, 460.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_media_release_gate_sump_witness_rib_{i}"),
                8.0,
                STATION_Y - 160.0,
                5.0,
            )
            .translate(x, 0.0, BASE_Z + 2.5);
    }

    let drain_gutter = centered_cube(
        "closed_media_release_gate_drain_lead_in_gutter",
        190.0,
        8.0,
        7.0,
    )
    .translate(
        STATION_X / 2.0 - 150.0,
        -STATION_Y / 2.0 + 48.0,
        BASE_Z + 3.5,
    );

    ribs + drain_gutter
}

fn module_mount_bosses() -> Part {
    let mut bosses = Part::empty("closed_media_release_gate_module_mount_bosses");
    for (i, (x, y)) in datum_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_media_release_gate_module_mount_boss_{i}"),
            10.5,
            8.0,
            28,
        )
        .translate(x, y, BASE_Z + 4.0);
        let clearance = centered_cylinder(
            format!("closed_media_release_gate_module_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - clearance);
    }
    bosses
}

fn row_boundary_rails() -> Part {
    let rear_flow_row = centered_cube(
        "closed_media_release_gate_rear_measurement_row_boundary",
        STATION_X - 170.0,
        8.0,
        24.0,
    )
    .translate(0.0, 34.0, BASE_Z + 12.0);
    let front_equilibration_row = centered_cube(
        "closed_media_release_gate_front_equilibration_row_boundary",
        STATION_X - 190.0,
        8.0,
        22.0,
    )
    .translate(0.0, -42.0, BASE_Z + 11.0);
    let clean_dirty_split = centered_cube(
        "closed_media_release_gate_clean_dirty_routing_split",
        10.0,
        STATION_Y - 138.0,
        26.0,
    )
    .translate(290.0, -12.0, BASE_Z + 13.0);

    rear_flow_row + front_equilibration_row + clean_dirty_split
}

fn sample_custody_receiver() -> Part {
    let body = centered_cube(
        "closed_media_release_sample_custody_receiver_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let gasket_groove = centered_cube(
        "closed_media_release_sample_custody_lid_gasket_groove",
        SAMPLE_X - 34.0,
        SAMPLE_Y - 32.0,
        8.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0 - 3.0);

    let mut wells = Part::empty("closed_media_release_sample_custody_well_cuts");
    let mut rims = Part::empty("closed_media_release_sample_custody_seal_rims");
    let mut analyte_tags = Part::empty("closed_media_release_sample_custody_analyte_tags");
    for index in 0..SAMPLE_WELLS {
        let x = centered_index(index % SAMPLE_COLS, SAMPLE_COLS, SAMPLE_PITCH_X);
        let y = centered_index(index / SAMPLE_COLS, ANALYTE_LANES, SAMPLE_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!("closed_media_release_sample_custody_well_{index}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);

        let rim = centered_cylinder(
            format!("closed_media_release_sample_custody_seal_rim_{index}"),
            SAMPLE_SEAL_RIM_D / 2.0,
            3.5,
            30,
        )
        .translate(x, y, SAMPLE_Z / 2.0 + 1.75);
        let opening = centered_cylinder(
            format!("closed_media_release_sample_custody_seal_opening_{index}"),
            SAMPLE_WELL_D / 2.0 + 0.7,
            4.0,
            30,
        )
        .translate(x, y, SAMPLE_Z / 2.0 + 1.75);
        rims = rims + (rim - opening);
    }

    for lane in 0..ANALYTE_LANES {
        let y = centered_index(lane, ANALYTE_LANES, SAMPLE_PITCH_Y);
        analyte_tags = analyte_tags
            + centered_cube(
                format!("closed_media_release_sample_custody_lane_tag_{lane}"),
                56.0,
                12.0,
                7.0,
            )
            .translate(-SAMPLE_X / 2.0 + 44.0, y, SAMPLE_Z / 2.0 + 3.5)
            + centered_cube(
                format!("closed_media_release_sample_custody_lane_status_rail_{lane}"),
                SAMPLE_X - 58.0,
                5.0,
                10.0,
            )
            .translate(10.0, y + 23.0, SAMPLE_Z / 2.0 + 5.0);
    }

    let sealed_inlet_saddle = centered_cube(
        "closed_media_release_sample_custody_closed_inlet_saddle",
        SAMPLE_X - 48.0,
        18.0,
        14.0,
    )
    .translate(0.0, SAMPLE_Y / 2.0 - 24.0, SAMPLE_Z / 2.0 + 7.0);
    let outlet_transfer_saddle = centered_cube(
        "closed_media_release_sample_custody_transfer_loop_saddle",
        SAMPLE_X - 72.0,
        16.0,
        12.0,
    )
    .translate(0.0, -SAMPLE_Y / 2.0 + 24.0, SAMPLE_Z / 2.0 + 6.0);

    body - gasket_groove - wells
        + rims
        + analyte_tags
        + sealed_inlet_saddle
        + outlet_transfer_saddle
}

fn sensor_measurement_manifold() -> Part {
    let body = centered_cube(
        "closed_media_release_sensor_measurement_manifold_body",
        MEASURE_X,
        MEASURE_Y,
        MEASURE_Z,
    );
    let flow_bore = centered_cylinder(
        "closed_media_release_sensor_measurement_main_flow_bore",
        FLOW_BORE_D / 2.0,
        MEASURE_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -18.0, 0.0);

    let mut sensor_cuts = Part::empty("closed_media_release_sensor_measurement_sensor_cuts");
    let mut sensor_guard_rings =
        Part::empty("closed_media_release_sensor_measurement_sensor_guard_rings");
    for i in 0..SENSOR_COUNT {
        let x = centered_index(i, SENSOR_COUNT, SENSOR_PITCH_X);
        sensor_cuts = sensor_cuts
            + centered_cylinder(
                format!("closed_media_release_sensor_measurement_sensor_well_{i}"),
                SENSOR_WELL_D / 2.0,
                MEASURE_Z + 6.0,
                36,
            )
            .translate(x, 28.0, 0.0);

        let ring = centered_cylinder(
            format!("closed_media_release_sensor_measurement_sensor_guard_ring_{i}"),
            SENSOR_WELL_D / 2.0 + 6.0,
            5.0,
            36,
        )
        .translate(x, 28.0, MEASURE_Z / 2.0 + 2.5);
        let opening = centered_cylinder(
            format!("closed_media_release_sensor_measurement_sensor_guard_opening_{i}"),
            SENSOR_WELL_D / 2.0 + 1.2,
            5.5,
            36,
        )
        .translate(x, 28.0, MEASURE_Z / 2.0 + 2.5);
        sensor_guard_rings = sensor_guard_rings + (ring - opening);
    }

    let mut sample_ports = Part::empty("closed_media_release_sensor_measurement_sample_ports");
    for i in 0..SAMPLE_LOOP_PORTS {
        let x = centered_index(i, SAMPLE_LOOP_PORTS, 56.0);
        sample_ports = sample_ports
            + centered_cylinder(
                format!("closed_media_release_sensor_measurement_sample_loop_port_{i}"),
                4.2,
                MEASURE_Y + 12.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -4.0);
    }

    let mut valves = Part::empty("closed_media_release_sensor_measurement_valve_pads");
    for i in 0..VALVE_COUNT {
        let x = centered_index(i, VALVE_COUNT, 58.0);
        valves = valves
            + centered_cube(
                format!("closed_media_release_sensor_measurement_isolate_flush_valve_pad_{i}"),
                28.0,
                22.0,
                10.0,
            )
            .translate(x, -MEASURE_Y / 2.0 + 28.0, MEASURE_Z / 2.0 + 5.0);
    }

    let osmo_cartridge_dock = centered_cube(
        "closed_media_release_sensor_measurement_osmometer_cartridge_dock",
        94.0,
        46.0,
        18.0,
    )
    .translate(0.0, MEASURE_Y / 2.0 - 36.0, MEASURE_Z / 2.0 + 9.0);

    body - flow_bore - sensor_cuts - sample_ports
        + sensor_guard_rings
        + valves
        + osmo_cartridge_dock
}

fn temperature_equilibration_block() -> Part {
    let body = centered_cube(
        "closed_media_release_temperature_equilibration_block_body",
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    );

    let mut pockets = Part::empty("closed_media_release_temperature_equilibration_pockets");
    let mut pocket_rims = Part::empty("closed_media_release_temperature_equilibration_pocket_rims");
    for index in 0..TEMP_POCKETS {
        let x = centered_index(index % TEMP_COLS, TEMP_COLS, TEMP_PITCH_X);
        let y = centered_index(index / TEMP_COLS, TEMP_POCKETS / TEMP_COLS, TEMP_PITCH_Y);
        pockets = pockets
            + centered_cylinder(
                format!("closed_media_release_temperature_sample_pocket_{index}"),
                TEMP_VIAL_D / 2.0,
                TEMP_Z + 5.0,
                30,
            )
            .translate(x, y, 0.0);
        pocket_rims = pocket_rims
            + centered_cylinder(
                format!("closed_media_release_temperature_sample_pocket_rim_{index}"),
                TEMP_VIAL_D / 2.0 + 3.5,
                3.0,
                30,
            )
            .translate(x, y, TEMP_Z / 2.0 + 1.5);
    }

    let mut channels = Part::empty("closed_media_release_temperature_thermal_channel_cuts");
    for i in 0..THERMAL_CHANNELS {
        let y = centered_index(i, THERMAL_CHANNELS, 36.0);
        channels = channels
            + centered_cylinder(
                format!("closed_media_release_temperature_thermal_channel_{i}"),
                4.8,
                TEMP_X + 18.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -6.0);
    }

    let mut thermowells = Part::empty("closed_media_release_temperature_thermowells");
    for i in 0..THERMOWELLS {
        let x = centered_index(i, THERMOWELLS, 94.0);
        thermowells = thermowells
            + centered_cylinder(
                format!("closed_media_release_temperature_reference_thermowell_{i}"),
                3.2,
                TEMP_Z + 8.0,
                20,
            )
            .translate(x, TEMP_Y / 2.0 - 30.0, 0.0);
    }

    let ready_token_lane = centered_cube(
        "closed_media_release_temperature_equilibrated_ready_token_lane",
        TEMP_X - 44.0,
        16.0,
        6.0,
    )
    .translate(0.0, -TEMP_Y / 2.0 + 22.0, TEMP_Z / 2.0 + 3.0);

    body - pockets - channels - thermowells + pocket_rims + ready_token_lane
}

fn calibration_standard_rack() -> Part {
    let body = centered_cube(
        "closed_media_release_calibration_standard_rack_body",
        CAL_X,
        CAL_Y,
        CAL_Z,
    );

    let mut wells = Part::empty("closed_media_release_calibration_standard_wells");
    let mut rims = Part::empty("closed_media_release_calibration_standard_seal_rims");
    for index in 0..CAL_WELLS {
        let x = centered_index(index % CAL_COLS, CAL_COLS, CAL_PITCH_X);
        let y = centered_index(index / CAL_COLS, CAL_STANDARD_FAMILIES, CAL_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!("closed_media_release_calibration_standard_well_{index}"),
                CAL_VIAL_D / 2.0,
                CAL_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);

        let rim = centered_cylinder(
            format!("closed_media_release_calibration_standard_rim_{index}"),
            CAL_VIAL_D / 2.0 + 4.0,
            3.5,
            30,
        )
        .translate(x, y, CAL_Z / 2.0 + 1.75);
        let opening = centered_cylinder(
            format!("closed_media_release_calibration_standard_rim_opening_{index}"),
            CAL_VIAL_D / 2.0 + 0.6,
            4.0,
            30,
        )
        .translate(x, y, CAL_Z / 2.0 + 1.75);
        rims = rims + (rim - opening);
    }

    let mut family_separators =
        Part::empty("closed_media_release_calibration_standard_family_separators");
    for i in 0..(CAL_STANDARD_FAMILIES - 1) {
        let y = (centered_index(i, CAL_STANDARD_FAMILIES, CAL_PITCH_Y)
            + centered_index(i + 1, CAL_STANDARD_FAMILIES, CAL_PITCH_Y))
            / 2.0;
        family_separators = family_separators
            + centered_cube(
                format!("closed_media_release_calibration_standard_family_separator_{i}"),
                CAL_X - 36.0,
                5.0,
                18.0,
            )
            .translate(0.0, y, CAL_Z / 2.0 + 9.0);
    }

    let mut coa_slots = Part::empty("closed_media_release_calibration_standard_coa_slots");
    for i in 0..COA_CARD_SLOTS {
        let x = centered_index(i, COA_CARD_SLOTS, 82.0);
        let slot = centered_cube(
            format!("closed_media_release_calibration_standard_coa_slot_{i}"),
            62.0,
            24.0,
            7.0,
        )
        .translate(x, -CAL_Y / 2.0 + 22.0, CAL_Z / 2.0 + 3.5);
        let relief = centered_cube(
            format!("closed_media_release_calibration_standard_coa_relief_{i}"),
            50.0,
            15.0,
            8.0,
        )
        .translate(x, -CAL_Y / 2.0 + 22.0, CAL_Z / 2.0 + 4.0);
        coa_slots = coa_slots + (slot - relief);
    }

    let expiry_quarantine_tab = centered_cube(
        "closed_media_release_calibration_expiry_quarantine_tab",
        CAL_X - 40.0,
        12.0,
        10.0,
    )
    .translate(0.0, CAL_Y / 2.0 - 18.0, CAL_Z / 2.0 + 5.0);

    body - wells + rims + family_separators + coa_slots + expiry_quarantine_tab
}

fn flush_waste_routing_manifold() -> Part {
    let body = centered_cube(
        "closed_media_release_flush_waste_routing_body",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let flush_header = centered_cylinder(
        "closed_media_release_flush_waste_flush_header_bore",
        FLOW_BORE_D / 2.0,
        ROUTE_Y + 14.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-70.0, 0.0, 2.0);
    let waste_header = centered_cylinder(
        "closed_media_release_flush_waste_waste_header_bore",
        FLOW_BORE_D / 2.0 + 1.2,
        ROUTE_Y + 14.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(58.0, 0.0, -2.0);

    let mut flush_ports = Part::empty("closed_media_release_flush_waste_flush_ports");
    for i in 0..FLUSH_PORTS {
        let y = centered_index(i, FLUSH_PORTS, 42.0);
        flush_ports = flush_ports
            + centered_cylinder(
                format!("closed_media_release_flush_waste_flush_port_{i}"),
                ROUTE_PORT_D / 2.0,
                ROUTE_Z + 5.0,
                26,
            )
            .translate(-70.0, y, 0.0);
    }

    let mut waste_channels = Part::empty("closed_media_release_flush_waste_channel_cuts");
    for i in 0..WASTE_CHANNELS {
        let y = centered_index(i, WASTE_CHANNELS, 78.0);
        waste_channels = waste_channels
            + centered_cube(
                format!("closed_media_release_flush_waste_divert_channel_{i}"),
                ROUTE_X - 58.0,
                12.0,
                10.0,
            )
            .translate(8.0, y, ROUTE_Z / 2.0 - 5.0);
    }

    let mut bottle_nests = Part::empty("closed_media_release_flush_waste_bottle_nests");
    for i in 0..WASTE_BOTTLES {
        let y = centered_index(i, WASTE_BOTTLES, 82.0);
        bottle_nests = bottle_nests
            + centered_cylinder(
                format!("closed_media_release_flush_waste_bottle_socket_{i}"),
                WASTE_BOTTLE_D / 2.0,
                ROUTE_Z + 5.0,
                36,
            )
            .translate(84.0, y, 0.0);
    }

    let mut route_arrows = Part::empty("closed_media_release_flush_waste_direction_arrows");
    for i in 0..8 {
        route_arrows = route_arrows
            + centered_cube(
                format!("closed_media_release_flush_waste_direction_bar_{i}"),
                38.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(i % 4, 4, 46.0) - 18.0,
                -130.0 + (i / 4) as f64 * 260.0,
                ROUTE_Z / 2.0 + 2.5,
            );
    }

    body - flush_header - waste_header - flush_ports - waste_channels - bottle_nests + route_arrows
}

fn release_hold_reject_decision_gate() -> Part {
    let panel = centered_cube(
        "closed_media_release_decision_gate_panel",
        DECISION_X,
        DECISION_Y,
        DECISION_Z,
    );

    let mut slots = Part::empty("closed_media_release_decision_gate_slot_cuts");
    let mut dividers = Part::empty("closed_media_release_decision_gate_lane_dividers");
    for lane in 0..DECISION_LANES {
        let x = centered_index(lane, DECISION_LANES, DECISION_LANE_PITCH_X);
        for slot in 0..DECISION_SLOTS_PER_LANE {
            let y = centered_index(slot, DECISION_SLOTS_PER_LANE, DECISION_SLOT_PITCH_Y);
            slots = slots
                + centered_cube(
                    format!("closed_media_release_decision_lane_{lane}_slot_{slot}"),
                    DECISION_SLOT_X,
                    DECISION_SLOT_Y,
                    DECISION_Z + 4.0,
                )
                .translate(x, y, 0.0);
        }

        dividers = dividers
            + centered_cube(
                format!("closed_media_release_decision_lane_{lane}_status_backstop"),
                DECISION_SLOT_X + 18.0,
                8.0,
                22.0,
            )
            .translate(x, DECISION_Y / 2.0 - 18.0, DECISION_Z / 2.0 + 11.0)
            + centered_cube(
                format!("closed_media_release_decision_lane_{lane}_front_token_land"),
                DECISION_SLOT_X + 12.0,
                12.0,
                6.0,
            )
            .translate(x, -DECISION_Y / 2.0 + 18.0, DECISION_Z / 2.0 + 3.0);
    }

    for i in 0..(DECISION_LANES - 1) {
        let x = (centered_index(i, DECISION_LANES, DECISION_LANE_PITCH_X)
            + centered_index(i + 1, DECISION_LANES, DECISION_LANE_PITCH_X))
            / 2.0;
        dividers = dividers
            + centered_cube(
                format!("closed_media_release_decision_hard_lane_separator_{i}"),
                8.0,
                DECISION_Y - 32.0,
                34.0,
            )
            .translate(x, 0.0, DECISION_Z / 2.0 + 17.0);
    }

    let gate_bridge = centered_cube(
        "closed_media_release_decision_gate_interlock_bridge",
        DECISION_X - 24.0,
        18.0,
        72.0,
    )
    .translate(0.0, DECISION_Y / 2.0 + 4.0, DECISION_Z / 2.0 + 36.0);
    let mut shutter_windows = Part::empty("closed_media_release_decision_gate_shutter_windows");
    for i in 0..DECISION_LANES {
        shutter_windows = shutter_windows
            + centered_cube(
                format!("closed_media_release_decision_gate_status_window_{i}"),
                48.0,
                20.0,
                28.0,
            )
            .translate(
                centered_index(i, DECISION_LANES, DECISION_LANE_PITCH_X),
                DECISION_Y / 2.0 + 4.0,
                DECISION_Z / 2.0 + 38.0,
            );
    }

    panel - slots + dividers + (gate_bridge - shutter_windows)
}

fn chain_of_custody_evidence_lands() -> Part {
    let mut lands = Part::empty("closed_media_release_chain_of_custody_evidence_lands");
    for i in 0..CUSTODY_LANDS {
        let x = -565.0 + (i % 6) as f64 * 62.0;
        let y = -STATION_Y / 2.0 + 72.0 + (i / 6) as f64 * 32.0;
        let land = centered_cube(
            format!("closed_media_release_custody_barcode_land_{i}"),
            48.0,
            18.0,
            4.0,
        )
        .translate(x, y, 0.0);
        let scan_relief = centered_cube(
            format!("closed_media_release_custody_barcode_scan_relief_{i}"),
            38.0,
            10.0,
            5.0,
        )
        .translate(x, y, 0.5);
        lands = lands + (land - scan_relief);
    }

    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_release_rfid_certificate_land_{i}"),
                58.0,
                28.0,
                4.0,
            )
            .translate(220.0 + i as f64 * 70.0, -STATION_Y / 2.0 + 74.0, 0.0);
    }

    for i in 0..RESULT_CARD_SLOTS {
        let x = centered_index(i, RESULT_CARD_SLOTS, 92.0);
        let slot = centered_cube(
            format!("closed_media_release_result_card_slot_{i}"),
            70.0,
            26.0,
            6.0,
        )
        .translate(x, STATION_Y / 2.0 - 74.0, 0.0);
        let relief = centered_cube(
            format!("closed_media_release_result_card_relief_{i}"),
            56.0,
            16.0,
            7.0,
        )
        .translate(x, STATION_Y / 2.0 - 74.0, 0.5);
        lands = lands + (slot - relief);
    }

    for i in 0..WITNESS_TOKEN_BARS {
        lands = lands
            + centered_cube(
                format!("closed_media_release_custody_hash_witness_bar_{i}"),
                9.0 + (i % 3) as f64 * 4.0,
                42.0,
                4.0,
            )
            .translate(-120.0 + i as f64 * 26.0, -STATION_Y / 2.0 + 118.0, 0.0);
    }

    lands
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_media_release_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "closed_media_release_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_CLEARANCE_Z / 2.0);
    let beam = centered_cube(
        "closed_media_release_evidence_bridge_cross_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut cameras = Part::empty("closed_media_release_evidence_bridge_cameras");
    for i in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("closed_media_release_evidence_camera_body_{i}"),
                42.0,
                28.0,
                24.0,
            )
            .translate(
                centered_index(i, CAMERA_COUNT, 250.0),
                -BRIDGE_POST_Y / 2.0 - 12.0,
                BRIDGE_CLEARANCE_Z - 20.0,
            )
            + centered_cylinder(
                format!("closed_media_release_evidence_camera_lens_{i}"),
                8.0,
                12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CAMERA_COUNT, 250.0),
                -BRIDGE_POST_Y / 2.0 - 28.0,
                BRIDGE_CLEARANCE_Z - 20.0,
            );
    }

    let mut lights = Part::empty("closed_media_release_evidence_bridge_led_segments");
    for i in 0..LED_SEGMENTS {
        lights = lights
            + centered_cube(
                format!("closed_media_release_evidence_led_segment_{i}"),
                82.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(i, LED_SEGMENTS, 118.0),
                BRIDGE_POST_Y / 2.0 + 5.0,
                BRIDGE_CLEARANCE_Z - 18.0,
            );
    }

    left_post + right_post + beam + cameras + lights
}

fn robot_service_keepout_gauges() -> Part {
    let robot_sweep = centered_cube(
        "closed_media_release_robot_pick_sweep_keepout",
        STATION_X - 140.0,
        FRONT_ROBOT_CLEARANCE,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_media_release_rear_standard_service_keepout",
        STATION_X - 170.0,
        REAR_STANDARD_SERVICE_CLEARANCE,
        130.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_STANDARD_SERVICE_CLEARANCE / 2.0,
        65.0,
    );
    let left_service = centered_cube(
        "closed_media_release_left_sample_custody_service_keepout",
        LEFT_SAMPLE_SERVICE_CLEARANCE,
        320.0,
        135.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_SAMPLE_SERVICE_CLEARANCE / 2.0,
        0.0,
        67.5,
    );
    let right_service = centered_cube(
        "closed_media_release_right_waste_service_keepout",
        RIGHT_WASTE_SERVICE_CLEARANCE,
        430.0,
        150.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_WASTE_SERVICE_CLEARANCE / 2.0,
        -40.0,
        75.0,
    );

    let mut datums = Part::empty("closed_media_release_robot_pick_datum_markers");
    for (i, (x, y)) in datum_points().into_iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("closed_media_release_robot_pick_datum_marker_{i}"),
                9.0,
                5.0,
                24,
            )
            .translate(x, y, 2.5);
    }

    robot_sweep + rear_service + left_service + right_service + datums
}

fn flow_path_custody_overlay() -> Part {
    let mut overlay = Part::empty("closed_media_release_flow_path_custody_overlay");
    for (i, (x1, y1, x2, y2)) in flow_segments().into_iter().enumerate() {
        let width = (x2 - x1).abs().max(8.0);
        let height = (y2 - y1).abs().max(8.0);
        let cx = (x1 + x2) / 2.0;
        let cy = (y1 + y2) / 2.0;
        overlay = overlay
            + centered_cube(
                format!("closed_media_release_flow_path_overlay_segment_{i}"),
                width,
                height,
                5.0,
            )
            .translate(cx, cy, 0.0);
    }

    for lane in 0..DECISION_LANES {
        overlay = overlay
            + centered_cube(
                format!("closed_media_release_decision_lane_arrow_token_{lane}"),
                42.0,
                10.0,
                5.0,
            )
            .translate(
                DECISION_CENTER.0 + centered_index(lane, DECISION_LANES, DECISION_LANE_PITCH_X),
                DECISION_CENTER.1 - DECISION_Y / 2.0 - 22.0,
                0.0,
            );
    }

    overlay
}

fn datum_points() -> [(f64, f64); ROBOT_DATUMS] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 52.0),
        (-220.0, STATION_Y / 2.0 - 50.0),
        (220.0, STATION_Y / 2.0 - 50.0),
        (-220.0, -STATION_Y / 2.0 + 50.0),
        (220.0, -STATION_Y / 2.0 + 50.0),
    ]
}

fn flow_segments() -> [(f64, f64, f64, f64); 8] {
    [
        (
            SAMPLE_CENTER.0 + SAMPLE_X / 2.0,
            SAMPLE_CENTER.1,
            MEASURE_CENTER.0 - MEASURE_X / 2.0,
            SAMPLE_CENTER.1,
        ),
        (
            SAMPLE_CENTER.0,
            SAMPLE_CENTER.1 - SAMPLE_Y / 2.0,
            TEMP_CENTER.0,
            TEMP_CENTER.1 + TEMP_Y / 2.0,
        ),
        (
            TEMP_CENTER.0 + TEMP_X / 2.0,
            TEMP_CENTER.1,
            CAL_CENTER.0 - CAL_X / 2.0,
            TEMP_CENTER.1,
        ),
        (
            CAL_CENTER.0 + CAL_X / 2.0,
            CAL_CENTER.1,
            ROUTE_CENTER.0 - ROUTE_X / 2.0,
            ROUTE_CENTER.1,
        ),
        (
            MEASURE_CENTER.0 + MEASURE_X / 2.0,
            MEASURE_CENTER.1,
            DECISION_CENTER.0 - DECISION_X / 2.0,
            DECISION_CENTER.1,
        ),
        (
            DECISION_CENTER.0,
            DECISION_CENTER.1 - DECISION_Y / 2.0,
            ROUTE_CENTER.0,
            ROUTE_CENTER.1 + ROUTE_Y / 2.0,
        ),
        (
            ROUTE_CENTER.0 - ROUTE_X / 2.0,
            ROUTE_CENTER.1 + 60.0,
            MEASURE_CENTER.0,
            MEASURE_CENTER.1 - MEASURE_Y / 2.0,
        ),
        (
            DECISION_CENTER.0 + DECISION_X / 2.0 - 26.0,
            DECISION_CENTER.1,
            STATION_X / 2.0 - 62.0,
            DECISION_CENTER.1,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_paths_are_scoped_and_unique() {
        assert_eq!(OUTPUTS.len(), 12);
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_media_conditioning_ph_osmolality_release_gate_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_release_gate_features_are_represented() {
        for feature in [
            "closed_sample_custody",
            "ph_sample_lane",
            "osmolality_sample_lane",
            "conductivity_sample_lane",
            "calibration_standards",
            "temperature_equilibration",
            "flush_waste_routing",
            "release_lane",
            "hold_lane",
            "reject_lane",
            "evidence_capture",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn analyte_and_decision_counts_match_scope() {
        assert_eq!(ANALYTE_LANES, 3);
        assert_eq!(SAMPLE_WELLS, 12);
        assert_eq!(SENSOR_COUNT, 3);
        assert_eq!(CAL_STANDARD_FAMILIES, 3);
        assert_eq!(CAL_WELLS, 12);
        assert_eq!(DECISION_LANES, 3);
        assert_eq!(DECISION_SLOTS, 12);
        assert_eq!(FLUSH_PORTS, 6);
        assert_eq!(WASTE_CHANNELS, 3);
    }

    #[test]
    fn layout_fits_without_insert_collisions() {
        assert_layout();
        assert!(module_specs()
            .iter()
            .all(|(_, center, width, depth)| fits_on_station(*center, *width, *depth, 14.0)));
    }

    #[test]
    fn release_hold_reject_lanes_are_physically_separated() {
        assert!(DECISION_LANE_PITCH_X > DECISION_SLOT_X + 16.0);
        assert_eq!(DECISION_SLOTS_PER_LANE, 4);
        assert_eq!(WASTE_BOTTLES, DECISION_LANES);
    }

    #[test]
    fn service_and_evidence_envelopes_are_visible() {
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(LED_SEGMENTS, 8);
        assert_eq!(ROBOT_DATUMS, 8);
        assert_eq!(SERVICE_KEEPOUTS, 4);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
        assert!(REAR_STANDARD_SERVICE_CLEARANCE >= 220.0);
        assert!(RIGHT_WASTE_SERVICE_CLEARANCE >= 240.0);
    }
}
