use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline sensor response-lag and flow-step challenge station.
//
// Intent:
// - Package no-cell validation hardware for pH, DO, conductivity, pressure,
//   and flow sensor dynamics in a closed inline fixture.
// - Keep sensor cartridge nests, step-change standard reservoirs, a
//   bypass/step-valve surrogate manifold, timed slug witness channel, flow
//   reference pocket, temperature equilibration pocket, flush/waste routing,
//   traceability lands, disposition lanes, evidence capture, and robot/service
//   keepouts visible as deterministic mechanical interfaces.
//
// This is engineering validation packaging CAD only. It does not define a
// biological protocol, sensor acceptance limits, formulation recipe, sterile
// barrier, controller algorithm, or metrology claim.

const OUTPUTS: [&str; 13] = [
    "output/closed_inline_sensor_response_lag_flow_step_station_base_containment_deck.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_sensor_cartridge_nests.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_step_change_standard_reservoirs.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_bypass_step_valve_surrogate_manifold.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_timed_slug_witness_channel.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_flow_sensor_reference_pocket.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_temperature_equilibration_pocket.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_flush_waste_routing.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_barcode_certificate_lands.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_release_hold_reject_lanes.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_evidence_bridge.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_robot_service_keepouts.stl",
    "output/closed_inline_sensor_response_lag_flow_step_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_containment_deck",
    "sensor_cartridge_nests",
    "step_change_standard_reservoirs",
    "bypass_step_valve_surrogate_manifold",
    "timed_slug_witness_channel",
    "flow_sensor_reference_pocket",
    "temperature_equilibration_pocket",
    "flush_waste_routing",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "no_cell_validation_fixture",
    "no_biological_protocol",
    "no_acceptance_thresholds",
    "no_calibration_algorithm",
    "no_sterile_barrier_claim",
    "no_metrology_claim",
];

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_inline_sensor_response_lag_flow_step_station_";

const STATION_X: f64 = 1400.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;
const TUBE_BORE_D: f64 = 6.2;

const SENSOR_POS: (f64, f64) = (-470.0, 230.0);
const SENSOR_X: f64 = 340.0;
const SENSOR_Y: f64 = 190.0;
const SENSOR_Z: f64 = 46.0;
const SENSOR_TYPES: [&str; 4] = ["ph", "do", "conductivity", "pressure"];
const SENSOR_CARTRIDGE_NESTS: usize = SENSOR_TYPES.len();
const SENSOR_POCKET_X: f64 = 58.0;
const SENSOR_POCKET_Y: f64 = 54.0;
const SENSOR_POCKET_DEPTH: f64 = 28.0;
const SENSOR_PITCH_X: f64 = 76.0;
const SENSOR_CLAMP_PAIRS: usize = SENSOR_CARTRIDGE_NESTS * 2;

const STANDARD_POS: (f64, f64) = (-80.0, 230.0);
const STANDARD_X: f64 = 360.0;
const STANDARD_Y: f64 = 190.0;
const STANDARD_Z: f64 = 58.0;
const STEP_STANDARD_PAIRS: usize = 4;
const STANDARD_RESERVOIRS: usize = STEP_STANDARD_PAIRS * 2;
const STANDARD_WELL_D: f64 = 34.0;
const STANDARD_PITCH_X: f64 = 72.0;
const STANDARD_PAIR_PITCH_Y: f64 = 62.0;
const STANDARD_RETENTION_LANDS: usize = STANDARD_RESERVOIRS;

const TEMP_POS: (f64, f64) = (285.0, 230.0);
const TEMP_X: f64 = 240.0;
const TEMP_Y: f64 = 190.0;
const TEMP_Z: f64 = 62.0;
const TEMP_EQUILIBRATION_WELLS: usize = SENSOR_CARTRIDGE_NESTS + 1;
const TEMP_SENSOR_WELLS: usize = 3;
const THERMAL_RIBS: usize = 6;

const FLOW_POS: (f64, f64) = (555.0, 230.0);
const FLOW_X: f64 = 200.0;
const FLOW_Y: f64 = 190.0;
const FLOW_Z: f64 = 54.0;
const FLOW_SENSOR_POCKETS: usize = 1;
const FLOW_REFERENCE_POCKETS: usize = 1;
const FLOW_STRAIGHTENER_RIBS: usize = 5;

const MANIFOLD_POS: (f64, f64) = (-430.0, 0.0);
const MANIFOLD_X: f64 = 410.0;
const MANIFOLD_Y: f64 = 180.0;
const MANIFOLD_Z: f64 = 56.0;
const STEP_VALVE_SURROGATES: usize = 5;
const BYPASS_CHANNELS: usize = 2;
const MANIFOLD_PORTS: usize = STEP_VALVE_SURROGATES * 2 + BYPASS_CHANNELS * 2;
const VALVE_PITCH_X: f64 = 70.0;
const VALVE_POCKET_D: f64 = 28.0;

const SLUG_POS: (f64, f64) = (20.0, 0.0);
const SLUG_X: f64 = 430.0;
const SLUG_Y: f64 = 180.0;
const SLUG_Z: f64 = 40.0;
const TIMED_SLUG_LANES: usize = 4;
const SLUG_TICK_COUNT: usize = 9;
const SLUG_LANE_PITCH_Y: f64 = 34.0;
const SLUG_WINDOW_X: f64 = 330.0;
const SLUG_CAPTURE_CUPS: usize = TIMED_SLUG_LANES * 2;

const ROUTING_POS: (f64, f64) = (465.0, 0.0);
const ROUTING_X: f64 = 330.0;
const ROUTING_Y: f64 = 180.0;
const ROUTING_Z: f64 = 44.0;
const FLUSH_PORT_COUNT: usize = 6;
const WASTE_CHANNEL_COUNT: usize = 6;
const WASTE_TRAP_CUPS: usize = 3;
const DRAIN_PORT_D: f64 = 13.0;

const TRACE_POS: (f64, f64) = (-465.0, -250.0);
const TRACE_X: f64 = 300.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 5;
const RUN_RECORD_LANDS: usize = 2;

const LANES_POS: (f64, f64) = (-80.0, -250.0);
const LANES_X: f64 = 390.0;
const LANES_Y: f64 = 150.0;
const LANES_Z: f64 = 38.0;
const DISPOSITION_LANES: usize = 3;
const LANE_TOKEN_SLOTS: usize = SENSOR_CARTRIDGE_NESTS + 1;
const LANE_SLOT_X: f64 = 58.0;
const LANE_SLOT_Y: f64 = 32.0;

const EVIDENCE_POS: (f64, f64) = (315.0, -250.0);
const EVIDENCE_FOOTPRINT_X: f64 = 360.0;
const EVIDENCE_FOOTPRINT_Y: f64 = 150.0;
const EVIDENCE_SPAN_X: f64 = 1010.0;
const EVIDENCE_POST_X: f64 = 28.0;
const EVIDENCE_POST_Y: f64 = 42.0;
const EVIDENCE_UNDERSIDE_Z: f64 = 210.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const CAMERA_BRACKETS: usize = 4;
const LIGHT_RAIL_SEGMENTS: usize = 8;

const KEEP_OUT_X: f64 = 1320.0;
const KEEP_OUT_Y: f64 = 790.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 405.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const LEFT_SENSOR_SERVICE_CLEARANCE: f64 = 175.0;
const RIGHT_FLUIDIC_SERVICE_CLEARANCE: f64 = 185.0;
const CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 155.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps_with_clearance(self, other: Footprint, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn divider_height(self) -> f64 {
        match self {
            DispositionLane::Release => 20.0,
            DispositionLane::Hold => 32.0,
            DispositionLane::Reject => 46.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let sensors = sensor_cartridge_nests();
    export(OUTPUTS[1], &sensors);

    let standards = step_change_standard_reservoirs();
    export(OUTPUTS[2], &standards);

    let manifold = bypass_step_valve_surrogate_manifold();
    export(OUTPUTS[3], &manifold);

    let slug = timed_slug_witness_channel();
    export(OUTPUTS[4], &slug);

    let flow = flow_sensor_reference_pocket();
    export(OUTPUTS[5], &flow);

    let temp = temperature_equilibration_pocket();
    export(OUTPUTS[6], &temp);

    let routing = flush_waste_routing();
    export(OUTPUTS[7], &routing);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[8], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, insert_z(SENSOR_Z))
        + standards.translate(STANDARD_POS.0, STANDARD_POS.1, insert_z(STANDARD_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, insert_z(MANIFOLD_Z))
        + slug.translate(SLUG_POS.0, SLUG_POS.1, insert_z(SLUG_Z))
        + flow.translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_Z))
        + temp.translate(TEMP_POS.0, TEMP_POS.1, insert_z(TEMP_Z))
        + routing.translate(ROUTING_POS.0, ROUTING_POS.1, insert_z(ROUTING_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + lanes.translate(LANES_POS.0, LANES_POS.1, insert_z(LANES_Z))
        + evidence
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed inline sensor response-lag and flow-step challenge station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained no-cell validation deck"
    );
    println!(
        "  Sensor cartridges:         {SENSOR_CARTRIDGE_NESTS} cartridge nests ({}) plus {FLOW_SENSOR_POCKETS} inline flow sensor pocket and {FLOW_REFERENCE_POCKETS} flow reference pocket",
        SENSOR_TYPES.join(", ")
    );
    println!(
        "  Step challenge hardware:   {STANDARD_RESERVOIRS} standard reservoirs in {STEP_STANDARD_PAIRS} paired step sets, {STEP_VALVE_SURROGATES} valve surrogates, {BYPASS_CHANNELS} bypass channels"
    );
    println!(
        "  Dynamic witnesses:         {TIMED_SLUG_LANES} timed slug lanes, {SLUG_TICK_COUNT} tick stations, {SLUG_CAPTURE_CUPS} slug capture cups, {FLOW_STRAIGHTENER_RIBS} flow straightener ribs"
    );
    println!(
        "  Temperature / routing:     {TEMP_EQUILIBRATION_WELLS} equilibration wells, {TEMP_SENSOR_WELLS} temperature wells, {FLUSH_PORT_COUNT} flush ports, {WASTE_CHANNEL_COUNT} waste channels"
    );
    println!(
        "  Evidence and disposition:  {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, release/hold/reject lanes, {CAMERA_BRACKETS} camera brackets"
    );
    println!(
        "  Keepouts and limitations:  {KEEP_OUT_ZONE_COUNT} keepout gauges, {} required feature groups, {} explicit non-scope notes",
        REQUIRED_FEATURES.len(),
        LIMITATIONS.len()
    );
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

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "sensor_cartridge_nests",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Footprint {
            name: "step_change_standard_reservoirs",
            center: STANDARD_POS,
            x: STANDARD_X,
            y: STANDARD_Y,
        },
        Footprint {
            name: "temperature_equilibration_pocket",
            center: TEMP_POS,
            x: TEMP_X,
            y: TEMP_Y,
        },
        Footprint {
            name: "flow_sensor_reference_pocket",
            center: FLOW_POS,
            x: FLOW_X,
            y: FLOW_Y,
        },
        Footprint {
            name: "bypass_step_valve_surrogate_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Footprint {
            name: "timed_slug_witness_channel",
            center: SLUG_POS,
            x: SLUG_X,
            y: SLUG_Y,
        },
        Footprint {
            name: "flush_waste_routing",
            center: ROUTING_POS,
            x: ROUTING_X,
            y: ROUTING_Y,
        },
        Footprint {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: LANES_POS,
            x: LANES_X,
            y: LANES_Y,
        },
        Footprint {
            name: "evidence_bridge_footprint",
            center: EVIDENCE_POS,
            x: EVIDENCE_FOOTPRINT_X,
            y: EVIDENCE_FOOTPRINT_Y,
        },
    ]
}

fn assert_design_constraints() {
    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds station envelope",
            footprint.name
        );
    }
    for i in 0..footprints.len() {
        for j in (i + 1)..footprints.len() {
            assert!(
                !footprints[i].overlaps_with_clearance(footprints[j], 12.0),
                "{} overlaps {}",
                footprints[i].name,
                footprints[j].name
            );
        }
    }

    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert_eq!(SENSOR_CARTRIDGE_NESTS, SENSOR_TYPES.len());
    assert_eq!(SENSOR_CLAMP_PAIRS, SENSOR_CARTRIDGE_NESTS * 2);
    assert_eq!(STANDARD_RESERVOIRS, STEP_STANDARD_PAIRS * 2);
    assert_eq!(STANDARD_RETENTION_LANDS, STANDARD_RESERVOIRS);
    assert_eq!(
        MANIFOLD_PORTS,
        STEP_VALVE_SURROGATES * 2 + BYPASS_CHANNELS * 2
    );
    assert_eq!(SLUG_CAPTURE_CUPS, TIMED_SLUG_LANES * 2);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANES);
    assert!(TEMP_EQUILIBRATION_WELLS >= SENSOR_CARTRIDGE_NESTS);
    assert!(FLOW_STRAIGHTENER_RIBS >= 3);
    assert!(FRONT_ROBOT_SWEEP_CLEARANCE > 350.0);
    assert!(CARTRIDGE_LIFT_CLEARANCE_Z > SENSOR_Z + TEMP_Z);
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "closed_inline_sensor_lag_step_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_inline_sensor_lag_step_station_washdown_recess",
        STATION_X - 116.0,
        STATION_Y - 114.0,
        7.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.5);
    let wet_path_sump = centered_cube(
        "closed_inline_sensor_lag_step_station_wet_path_sump",
        1110.0,
        300.0,
        8.0,
    )
    .translate(30.0, 116.0, BASE_Z / 2.0 - 4.0);
    let front_disposition_sump = centered_cube(
        "closed_inline_sensor_lag_step_station_front_disposition_sump",
        STATION_X - 210.0,
        104.0,
        8.0,
    )
    .translate(0.0, -252.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        "closed_inline_sensor_lag_step_station_front_drain",
        DRAIN_PORT_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 76.0, -(STATION_Y / 2.0 - 4.0), -1.0);

    deck - washdown_recess
        - wet_path_sump
        - front_disposition_sump
        - drain
        - insert_sockets()
        - mounting_slots()
        + perimeter_rims()
        + zone_dividers()
        + flow_direction_ribs()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_inline_sensor_lag_step_station_insert_sockets");
    for footprint in station_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_inline_sensor_lag_step_station_{}_socket",
                    footprint.name
                ),
                footprint.x + 8.0,
                footprint.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_inline_sensor_lag_step_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_mount_slot_relief_{i}"),
                28.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_inline_sensor_lag_step_station_left_containment_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_inline_sensor_lag_step_station_right_containment_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_inline_sensor_lag_step_station_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_inline_sensor_lag_step_station_front_low_service_lip",
        STATION_X - 190.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let upper_to_middle = centered_cube(
        "closed_inline_sensor_lag_step_station_upper_dynamic_row_divider",
        STATION_X - 160.0,
        10.0,
        28.0,
    )
    .translate(0.0, 118.0, BASE_Z / 2.0 + 14.0);
    let middle_to_disposition = centered_cube(
        "closed_inline_sensor_lag_step_station_middle_disposition_row_divider",
        STATION_X - 210.0,
        10.0,
        26.0,
    )
    .translate(0.0, -132.0, BASE_Z / 2.0 + 13.0);
    let traceability_divider = centered_cube(
        "closed_inline_sensor_lag_step_station_traceability_to_lanes_divider",
        10.0,
        162.0,
        26.0,
    )
    .translate(-282.0, -250.0, BASE_Z / 2.0 + 13.0);
    let evidence_divider = centered_cube(
        "closed_inline_sensor_lag_step_station_lanes_to_evidence_divider",
        10.0,
        162.0,
        26.0,
    )
    .translate(132.0, -250.0, BASE_Z / 2.0 + 13.0);

    upper_to_middle + middle_to_disposition + traceability_divider + evidence_divider
}

fn flow_direction_ribs() -> Part {
    let mut ribs = Part::empty("closed_inline_sensor_lag_step_station_flow_direction_ribs");
    for i in 0..8 {
        let x = centered_index(i, 8, 146.0);
        ribs = ribs
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_flow_arrow_stem_{i}"),
                82.0,
                5.0,
                5.0,
            )
            .translate(x, 104.0, BASE_Z / 2.0 + 4.0)
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_flow_arrow_head_{i}"),
                16.0,
                16.0,
                5.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 48.0, 104.0, BASE_Z / 2.0 + 4.0);
    }
    ribs
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_inline_sensor_lag_step_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 86.0), STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 86.0, STATION_Y / 2.0 - 86.0),
        (-(STATION_X / 2.0 - 86.0), -(STATION_Y / 2.0 - 86.0)),
        (STATION_X / 2.0 - 86.0, -(STATION_Y / 2.0 - 86.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_datum_ring_{i}"),
                15.0,
                4.0,
                36,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_datum_center_bore_{i}"),
                3.0,
                6.0,
                20,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn sensor_cartridge_nests() -> Part {
    let tray = centered_cube(
        "closed_inline_sensor_lag_step_station_sensor_cartridge_nest_tray",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let mut features = Part::empty("closed_inline_sensor_lag_step_station_sensor_nest_features");
    for (i, sensor) in SENSOR_TYPES.iter().enumerate() {
        let x = centered_index(i, SENSOR_CARTRIDGE_NESTS, SENSOR_PITCH_X);
        features = features
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_{sensor}_cartridge_pocket"),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                SENSOR_POCKET_DEPTH,
            )
            .translate(x, 10.0, SENSOR_Z / 2.0 - SENSOR_POCKET_DEPTH / 2.0 + 1.0)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_{sensor}_inline_tube_bore"),
                TUBE_BORE_D / 2.0,
                SENSOR_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 10.0, 6.0)
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_{sensor}_label_land"),
                54.0,
                20.0,
                4.0,
            )
            .translate(x, -58.0, SENSOR_Z / 2.0 + 2.0);
        for side in [-1.0, 1.0] {
            features = features
                + centered_cube(
                    format!(
                        "closed_inline_sensor_lag_step_station_{sensor}_spring_clamp_land_{}",
                        if side < 0.0 { "left" } else { "right" }
                    ),
                    12.0,
                    66.0,
                    12.0,
                )
                .translate(
                    x + side * (SENSOR_POCKET_X / 2.0 + 14.0),
                    10.0,
                    SENSOR_Z / 2.0 + 6.0,
                );
        }
    }
    tray - features + sensor_nest_index_ticks()
}

fn sensor_nest_index_ticks() -> Part {
    let mut ticks = Part::empty("closed_inline_sensor_lag_step_station_sensor_nest_index_ticks");
    for i in 0..=SENSOR_CARTRIDGE_NESTS {
        let x =
            -((SENSOR_CARTRIDGE_NESTS as f64) * SENSOR_PITCH_X) / 2.0 + i as f64 * SENSOR_PITCH_X;
        ticks = ticks
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_sensor_nest_tick_{i}"),
                4.0,
                SENSOR_Y - 28.0,
                6.0,
            )
            .translate(x, 0.0, SENSOR_Z / 2.0 + 3.0);
    }
    ticks
}

fn step_change_standard_reservoirs() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_lag_step_station_step_standard_block",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    );
    let mut features = Part::empty("closed_inline_sensor_lag_step_station_step_standard_features");
    for pair in 0..STEP_STANDARD_PAIRS {
        let x = centered_index(pair, STEP_STANDARD_PAIRS, STANDARD_PITCH_X);
        for (phase, y) in [
            ("baseline", -STANDARD_PAIR_PITCH_Y / 2.0),
            ("step", STANDARD_PAIR_PITCH_Y / 2.0),
        ] {
            features = features
                + centered_cylinder(
                    format!(
                        "closed_inline_sensor_lag_step_station_{phase}_standard_reservoir_{pair}"
                    ),
                    STANDARD_WELL_D / 2.0,
                    STANDARD_Z - 10.0,
                    40,
                )
                .translate(x, y, 8.0)
                + centered_cube(
                    format!("closed_inline_sensor_lag_step_station_{phase}_certificate_tab_{pair}"),
                    48.0,
                    14.0,
                    5.0,
                )
                .translate(x, y + 28.0, STANDARD_Z / 2.0 + 2.5);
        }
        features = features
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_step_pair_retention_land_{pair}"),
                54.0,
                STANDARD_Y - 32.0,
                5.0,
            )
            .translate(x, 0.0, STANDARD_Z / 2.0 + 2.5);
    }
    block - features + standard_pair_isolation_ribs()
}

fn standard_pair_isolation_ribs() -> Part {
    let mut ribs = Part::empty("closed_inline_sensor_lag_step_station_standard_pair_isolation");
    for i in 0..=STEP_STANDARD_PAIRS {
        let x =
            -((STEP_STANDARD_PAIRS as f64) * STANDARD_PITCH_X) / 2.0 + i as f64 * STANDARD_PITCH_X;
        ribs = ribs
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_standard_pair_divider_{i}"),
                5.0,
                STANDARD_Y - 24.0,
                12.0,
            )
            .translate(x, 0.0, STANDARD_Z / 2.0 + 6.0);
    }
    ribs
}

fn bypass_step_valve_surrogate_manifold() -> Part {
    let body = centered_cube(
        "closed_inline_sensor_lag_step_station_bypass_step_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut cuts = Part::empty("closed_inline_sensor_lag_step_station_bypass_step_manifold_cuts");
    let mut lands = Part::empty("closed_inline_sensor_lag_step_station_bypass_step_manifold_lands");
    for valve in 0..STEP_VALVE_SURROGATES {
        let x = centered_index(valve, STEP_VALVE_SURROGATES, VALVE_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_step_valve_pocket_{valve}"),
                VALVE_POCKET_D / 2.0,
                MANIFOLD_Z + 8.0,
                36,
            )
            .translate(x, 0.0, 0.0)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_step_valve_inlet_bore_{valve}"),
                TUBE_BORE_D / 2.0,
                MANIFOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -18.0, 4.0);
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_step_valve_state_land_{valve}"),
                48.0,
                18.0,
                5.0,
            )
            .translate(x, 58.0, MANIFOLD_Z / 2.0 + 2.5);
    }
    for bypass in 0..BYPASS_CHANNELS {
        let y = if bypass == 0 { -58.0 } else { 58.0 };
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_bypass_channel_bore_{bypass}"),
                TUBE_BORE_D / 2.0,
                MANIFOLD_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 2.0);
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_bypass_label_land_{bypass}"),
                MANIFOLD_X - 42.0,
                10.0,
                4.0,
            )
            .translate(0.0, y, MANIFOLD_Z / 2.0 + 2.0);
    }
    body - cuts + lands + manifold_port_markers()
}

fn manifold_port_markers() -> Part {
    let mut markers = Part::empty("closed_inline_sensor_lag_step_station_manifold_port_markers");
    for i in 0..MANIFOLD_PORTS {
        let x = centered_index(i % 7, 7, 52.0);
        let y = if i < 7 {
            -MANIFOLD_Y / 2.0 + 12.0
        } else {
            MANIFOLD_Y / 2.0 - 12.0
        };
        markers = markers
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_manifold_port_marker_{i}"),
                5.0,
                4.0,
                18,
            )
            .translate(x, y, MANIFOLD_Z / 2.0 + 2.0);
    }
    markers
}

fn timed_slug_witness_channel() -> Part {
    let body = centered_cube(
        "closed_inline_sensor_lag_step_station_timed_slug_witness_body",
        SLUG_X,
        SLUG_Y,
        SLUG_Z,
    );
    let mut cuts = Part::empty("closed_inline_sensor_lag_step_station_timed_slug_cuts");
    let mut features = Part::empty("closed_inline_sensor_lag_step_station_timed_slug_features");
    for lane in 0..TIMED_SLUG_LANES {
        let y = centered_index(lane, TIMED_SLUG_LANES, SLUG_LANE_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_slug_window_recess_{lane}"),
                SLUG_WINDOW_X,
                16.0,
                10.0,
            )
            .translate(0.0, y, SLUG_Z / 2.0 - 4.0)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_slug_lane_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                SLUG_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 0.0);
        features = features
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_slug_inlet_capture_cup_{lane}"),
                12.0,
                9.0,
                28,
            )
            .translate(-SLUG_X / 2.0 + 34.0, y, SLUG_Z / 2.0 + 4.5)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_slug_outlet_capture_cup_{lane}"),
                12.0,
                9.0,
                28,
            )
            .translate(SLUG_X / 2.0 - 34.0, y, SLUG_Z / 2.0 + 4.5);
    }
    body - cuts + features + slug_timing_ticks()
}

fn slug_timing_ticks() -> Part {
    let mut ticks = Part::empty("closed_inline_sensor_lag_step_station_slug_timing_ticks");
    for i in 0..SLUG_TICK_COUNT {
        let x = centered_index(
            i,
            SLUG_TICK_COUNT,
            SLUG_WINDOW_X / (SLUG_TICK_COUNT as f64 - 1.0),
        );
        ticks = ticks
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_slug_time_tick_{i}"),
                4.0,
                SLUG_Y - 34.0,
                6.0,
            )
            .translate(x, 0.0, SLUG_Z / 2.0 + 3.0);
    }
    ticks
}

fn flow_sensor_reference_pocket() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_lag_step_station_flow_reference_block",
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    );
    let sensor_pocket = centered_cube(
        "closed_inline_sensor_lag_step_station_inline_flow_sensor_pocket",
        82.0,
        54.0,
        30.0,
    )
    .translate(-42.0, 28.0, FLOW_Z / 2.0 - 14.0);
    let reference_pocket = centered_cube(
        "closed_inline_sensor_lag_step_station_flow_reference_meter_pocket",
        92.0,
        66.0,
        32.0,
    )
    .translate(42.0, -38.0, FLOW_Z / 2.0 - 15.0);
    let inline_bore = centered_cylinder(
        "closed_inline_sensor_lag_step_station_flow_inline_bore",
        TUBE_BORE_D / 2.0,
        FLOW_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 28.0, 0.0);
    let reference_bore = centered_cylinder(
        "closed_inline_sensor_lag_step_station_flow_reference_bore",
        TUBE_BORE_D / 2.0,
        FLOW_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -38.0, 0.0);

    block - sensor_pocket - reference_pocket - inline_bore - reference_bore
        + flow_straightener_ribs()
        + flow_pressure_taps()
}

fn flow_straightener_ribs() -> Part {
    let mut ribs = Part::empty("closed_inline_sensor_lag_step_station_flow_straightener_ribs");
    for i in 0..FLOW_STRAIGHTENER_RIBS {
        let x = centered_index(i, FLOW_STRAIGHTENER_RIBS, 28.0);
        ribs = ribs
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_flow_straightener_rib_{i}"),
                4.0,
                130.0,
                11.0,
            )
            .translate(x, -5.0, FLOW_Z / 2.0 + 5.5);
    }
    ribs
}

fn flow_pressure_taps() -> Part {
    let mut taps = Part::empty("closed_inline_sensor_lag_step_station_flow_pressure_taps");
    for (i, x) in [-68.0, 68.0].iter().enumerate() {
        taps = taps
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_flow_reference_tap_{i}"),
                7.0,
                9.0,
                24,
            )
            .translate(*x, 72.0, FLOW_Z / 2.0 + 4.5)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_flow_sensor_tap_{i}"),
                7.0,
                9.0,
                24,
            )
            .translate(*x, -82.0, FLOW_Z / 2.0 + 4.5);
    }
    taps
}

fn temperature_equilibration_pocket() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_lag_step_station_temperature_equilibration_block",
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    );
    let mut cuts = Part::empty("closed_inline_sensor_lag_step_station_temperature_cuts");
    let mut features = Part::empty("closed_inline_sensor_lag_step_station_temperature_features");
    for i in 0..TEMP_EQUILIBRATION_WELLS {
        let x = centered_index(i, TEMP_EQUILIBRATION_WELLS, 38.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_equilibration_well_{i}"),
                15.0,
                TEMP_Z - 10.0,
                32,
            )
            .translate(x, 18.0, 8.0);
    }
    for i in 0..TEMP_SENSOR_WELLS {
        let x = centered_index(i, TEMP_SENSOR_WELLS, 54.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_temperature_probe_well_{i}"),
                5.0,
                TEMP_Z - 8.0,
                20,
            )
            .translate(x, -54.0, 8.0);
    }
    for i in 0..THERMAL_RIBS {
        let x = centered_index(i, THERMAL_RIBS, 34.0);
        features = features
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_thermal_mass_rib_{i}"),
                8.0,
                TEMP_Y - 26.0,
                12.0,
            )
            .translate(x, 0.0, TEMP_Z / 2.0 + 6.0);
    }
    block - cuts + features
}

fn flush_waste_routing() -> Part {
    let body = centered_cube(
        "closed_inline_sensor_lag_step_station_flush_waste_body",
        ROUTING_X,
        ROUTING_Y,
        ROUTING_Z,
    );
    let mut cuts = Part::empty("closed_inline_sensor_lag_step_station_flush_waste_cuts");
    let mut features = Part::empty("closed_inline_sensor_lag_step_station_flush_waste_features");
    for port in 0..FLUSH_PORT_COUNT {
        let x = centered_index(port, FLUSH_PORT_COUNT, 46.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_flush_port_{port}"),
                8.0,
                ROUTING_Z + 8.0,
                24,
            )
            .translate(x, 48.0, 0.0)
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_waste_channel_bore_{port}"),
                TUBE_BORE_D / 2.0,
                ROUTING_Y + 8.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_flush_label_land_{port}"),
                32.0,
                14.0,
                4.0,
            )
            .translate(x, 78.0, ROUTING_Z / 2.0 + 2.0);
    }
    for cup in 0..WASTE_TRAP_CUPS {
        let x = centered_index(cup, WASTE_TRAP_CUPS, 86.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_sensor_lag_step_station_waste_trap_cup_{cup}"),
                22.0,
                ROUTING_Z - 8.0,
                36,
            )
            .translate(x, -54.0, 6.0);
    }
    body - cuts + features
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_inline_sensor_lag_step_station_barcode_certificate_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("closed_inline_sensor_lag_step_station_traceability_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 64.0);
        let y = 42.0 - (i / 4) as f64 * 38.0;
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_barcode_land_{i}"),
                52.0,
                22.0,
                4.0,
            )
            .translate(x, y, TRACE_Z / 2.0 + 2.0);
    }
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 48.0);
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_certificate_land_{i}"),
                36.0,
                24.0,
                4.0,
            )
            .translate(x, -38.0, TRACE_Z / 2.0 + 2.0);
    }
    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_run_record_land_{i}"),
                116.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, RUN_RECORD_LANDS, 128.0),
                -64.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    plate + lands
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_inline_sensor_lag_step_station_disposition_lane_base",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    let mut cuts = Part::empty("closed_inline_sensor_lag_step_station_disposition_cuts");
    let mut ribs = Part::empty("closed_inline_sensor_lag_step_station_disposition_ribs");
    for lane in DispositionLane::all() {
        let y = centered_index(lane.index(), DISPOSITION_LANES, 44.0);
        ribs = ribs
            + centered_cube(
                format!(
                    "closed_inline_sensor_lag_step_station_{}_lane_status_rib",
                    lane.label()
                ),
                LANES_X - 22.0,
                5.0,
                lane.divider_height(),
            )
            .translate(0.0, y + 18.0, LANES_Z / 2.0 + lane.divider_height() / 2.0);
        for slot in 0..LANE_TOKEN_SLOTS {
            let x = centered_index(slot, LANE_TOKEN_SLOTS, 66.0);
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_inline_sensor_lag_step_station_{}_token_slot_{slot}",
                        lane.label()
                    ),
                    LANE_SLOT_X,
                    LANE_SLOT_Y,
                    LANES_Z - 8.0,
                )
                .translate(x, y - 8.0, 6.0);
        }
    }
    base - cuts + ribs
}

fn evidence_bridge() -> Part {
    let z_post = BASE_Z / 2.0 + EVIDENCE_UNDERSIDE_Z / 2.0;
    let z_beam = BASE_Z / 2.0 + EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z / 2.0;
    let left_post = centered_cube(
        "closed_inline_sensor_lag_step_station_evidence_bridge_left_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(-EVIDENCE_SPAN_X / 2.0, EVIDENCE_POS.1, z_post);
    let right_post = centered_cube(
        "closed_inline_sensor_lag_step_station_evidence_bridge_right_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(EVIDENCE_SPAN_X / 2.0, EVIDENCE_POS.1, z_post);
    let beam = centered_cube(
        "closed_inline_sensor_lag_step_station_evidence_bridge_camera_beam",
        EVIDENCE_SPAN_X + EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, EVIDENCE_POS.1, z_beam);
    let mut brackets = Part::empty("closed_inline_sensor_lag_step_station_evidence_brackets");
    for camera in 0..CAMERA_BRACKETS {
        let x = centered_index(camera, CAMERA_BRACKETS, 250.0);
        brackets = brackets
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_camera_bracket_{camera}"),
                54.0,
                18.0,
                26.0,
            )
            .translate(
                x,
                EVIDENCE_POS.1 - 34.0,
                z_beam - EVIDENCE_BEAM_Z / 2.0 - 13.0,
            );
    }
    for segment in 0..LIGHT_RAIL_SEGMENTS {
        let x = centered_index(segment, LIGHT_RAIL_SEGMENTS, 110.0);
        brackets = brackets
            + centered_cube(
                format!("closed_inline_sensor_lag_step_station_light_rail_segment_{segment}"),
                82.0,
                8.0,
                8.0,
            )
            .translate(x, EVIDENCE_POS.1 + 32.0, z_beam - 30.0);
    }
    left_post + right_post + beam + brackets
}

fn robot_service_keepouts() -> Part {
    let base_gauge = centered_cube(
        "closed_inline_sensor_lag_step_station_keepout_outer_footprint_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let front_sweep = centered_cube(
        "closed_inline_sensor_lag_step_station_front_robot_sweep_keepout",
        KEEP_OUT_X - 120.0,
        10.0,
        34.0,
    )
    .translate(0.0, -FRONT_ROBOT_SWEEP_CLEARANCE / 2.0, BASE_Z / 2.0 + 17.0);
    let rear_service = centered_cube(
        "closed_inline_sensor_lag_step_station_rear_service_clearance_gauge",
        KEEP_OUT_X - 150.0,
        10.0,
        30.0,
    )
    .translate(0.0, REAR_SERVICE_CLEARANCE, BASE_Z / 2.0 + 15.0);
    let left_service = centered_cube(
        "closed_inline_sensor_lag_step_station_left_sensor_service_keepout",
        10.0,
        KEEP_OUT_Y - 150.0,
        28.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_SENSOR_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + 14.0,
    );
    let right_service = centered_cube(
        "closed_inline_sensor_lag_step_station_right_fluidic_service_keepout",
        10.0,
        KEEP_OUT_Y - 150.0,
        28.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_FLUIDIC_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + 14.0,
    );
    let lift_clearance = centered_cube(
        "closed_inline_sensor_lag_step_station_cartridge_lift_clearance_gauge",
        SENSOR_X + TEMP_X + 110.0,
        14.0,
        18.0,
    )
    .translate(
        -110.0,
        SENSOR_POS.1 + SENSOR_Y / 2.0 + 26.0,
        BASE_Z / 2.0 + CARTRIDGE_LIFT_CLEARANCE_Z,
    );

    base_gauge + front_sweep + rear_service + left_service + right_service + lift_clearance
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn station_footprints_fit_and_do_not_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn output_names_are_complete_unique_and_prefixed() {
        assert_eq!(OUTPUTS.len(), 13);
        let unique = OUTPUTS.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_design_scope() {
        let required = REQUIRED_FEATURES.iter().copied().collect::<HashSet<_>>();
        for feature in [
            "sensor_cartridge_nests",
            "step_change_standard_reservoirs",
            "bypass_step_valve_surrogate_manifold",
            "timed_slug_witness_channel",
            "flow_sensor_reference_pocket",
            "temperature_equilibration_pocket",
            "flush_waste_routing",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(required.contains(feature), "missing feature {feature}");
        }
        assert!(LIMITATIONS.contains(&"no_biological_protocol"));
        assert!(LIMITATIONS.contains(&"no_acceptance_thresholds"));
    }

    #[test]
    fn dynamic_station_counts_are_consistent() {
        assert_eq!(SENSOR_CARTRIDGE_NESTS, 4);
        assert_eq!(STANDARD_RESERVOIRS, 8);
        assert_eq!(STEP_VALVE_SURROGATES, 5);
        assert_eq!(TIMED_SLUG_LANES, 4);
        assert_eq!(FLOW_SENSOR_POCKETS + FLOW_REFERENCE_POCKETS, 2);
        assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
        assert_eq!(DispositionLane::all().len(), 3);
    }
}
