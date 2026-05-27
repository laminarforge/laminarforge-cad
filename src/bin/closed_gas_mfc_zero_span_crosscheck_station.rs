use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed gas MFC zero/span crosscheck station for CO2/O2/N2/air service panels.
//
// Intent:
// - Crosscheck purchased gas mass-flow controllers against an independent
//   reference flow meter before closed culture service-panel release.
// - Provide physically distinct zero, span, purge, release, hold, and reject
//   zones with traceability lands and evidence-camera framing.
// - Model MFCs, regulators, quick-connects, pressure-rated fittings, and meters
//   as purchased hardware envelopes/pockets only. Printed parts are packaging,
//   routing, witness, and robot/service interface geometry.
//
// Stable STL exports:
//   output/closed_gas_mfc_zero_span_crosscheck_station_base_tray.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_mfc_pocket_array.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_reference_flow_meter_dock.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_calibration_gas_quick_connect_panel.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_zero_gas_purge_manifold.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_pressure_regulator_envelope_bank.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_release_hold_reject_lanes.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_barcode_certificate_lands.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_leak_vent_witness_features.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_evidence_camera_bridge.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_robot_service_keepout_gauge.stl
//   output/closed_gas_mfc_zero_span_crosscheck_station_assembly.stl

const OUTPUTS: [&str; 12] = [
    "output/closed_gas_mfc_zero_span_crosscheck_station_base_tray.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_mfc_pocket_array.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_reference_flow_meter_dock.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_calibration_gas_quick_connect_panel.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_zero_gas_purge_manifold.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_pressure_regulator_envelope_bank.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_release_hold_reject_lanes.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_barcode_certificate_lands.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_leak_vent_witness_features.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_evidence_camera_bridge.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_robot_service_keepout_gauge.stl",
    "output/closed_gas_mfc_zero_span_crosscheck_station_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 11] = [
    "co2_o2_n2_air_mfc_pocket_placeholders",
    "independent_reference_flow_meter_dock",
    "calibration_gas_quick_connect_panel",
    "zero_gas_purge_route",
    "pressure_regulator_envelopes",
    "barcode_and_certificate_lands",
    "release_hold_reject_lanes",
    "leak_and_vent_witness_features",
    "evidence_camera_bridge",
    "robot_keepouts",
    "service_keepouts",
];

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2", "n2", "air"];
const CAL_GAS_PORTS: usize = 8;
const PURGE_LANES: usize = GAS_CHANNELS + 1;
const REGULATOR_COUNT: usize = GAS_CHANNELS;
const MFC_POCKETS: usize = GAS_CHANNELS;
const LANE_COUNT: usize = 3;
const BARCODE_LANDS: usize = 12;
const CERTIFICATE_LANDS: usize = 4;
const LEAK_WITNESS_WINDOWS: usize = 4;
const VENT_WITNESS_PORTS: usize = GAS_CHANNELS + 2;
const KEEP_OUT_ZONES: usize = 5;

const STATION_X: f64 = 1150.0;
const STATION_Y: f64 = 840.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;

const MFC_ARRAY_X: f64 = 520.0;
const MFC_ARRAY_Y: f64 = 154.0;
const MFC_ARRAY_Z: f64 = 54.0;
const MFC_POS: (f64, f64) = (-285.0, 168.0);
const MFC_PITCH_X: f64 = 122.0;
const MFC_ENVELOPE_X: f64 = 84.0;
const MFC_ENVELOPE_Y: f64 = 92.0;
const MFC_ENVELOPE_Z: f64 = 38.0;
const TUBE_BORE_D: f64 = 6.2;

const FLOW_DOCK_X: f64 = 350.0;
const FLOW_DOCK_Y: f64 = 150.0;
const FLOW_DOCK_Z: f64 = 62.0;
const FLOW_POS: (f64, f64) = (-365.0, -24.0);
const FLOW_METER_X: f64 = 230.0;
const FLOW_METER_Y: f64 = 84.0;
const FLOW_METER_Z: f64 = 44.0;

const QUICK_PANEL_X: f64 = 438.0;
const QUICK_PANEL_Y: f64 = 118.0;
const QUICK_PANEL_Z: f64 = 126.0;
const QUICK_POS: (f64, f64) = (224.0, -24.0);
const QUICK_PORT_PITCH_X: f64 = 52.0;
const QUICK_CONNECT_D: f64 = 14.2;

const PURGE_MANIFOLD_X: f64 = 760.0;
const PURGE_MANIFOLD_Y: f64 = 96.0;
const PURGE_MANIFOLD_Z: f64 = 54.0;
const PURGE_POS: (f64, f64) = (0.0, -164.0);
const PURGE_LANE_PITCH_X: f64 = 126.0;

const REG_BANK_X: f64 = 438.0;
const REG_BANK_Y: f64 = 154.0;
const REG_BANK_Z: f64 = 92.0;
const REG_POS: (f64, f64) = (285.0, 168.0);
const REG_PITCH_X: f64 = 98.0;
const REG_ENVELOPE_X: f64 = 78.0;
const REG_ENVELOPE_Y: f64 = 76.0;
const REG_ENVELOPE_Z: f64 = 76.0;

const LANE_TRAY_X: f64 = 430.0;
const LANE_TRAY_Y: f64 = 112.0;
const LANE_TRAY_Z: f64 = 44.0;
const LANE_POS: (f64, f64) = (-318.0, -284.0);
const LANE_PITCH_X: f64 = 132.0;

const LABEL_PANEL_X: f64 = 430.0;
const LABEL_PANEL_Y: f64 = 112.0;
const LABEL_PANEL_Z: f64 = 12.0;
const LABEL_POS: (f64, f64) = (230.0, -284.0);

const WITNESS_X: f64 = 930.0;
const WITNESS_Y: f64 = 62.0;
const WITNESS_Z: f64 = 30.0;
const WITNESS_POS: (f64, f64) = (0.0, -344.0);

const CAMERA_BRIDGE_X: f64 = 940.0;
const CAMERA_BRIDGE_Y: f64 = 54.0;
const CAMERA_BRIDGE_Z: f64 = 190.0;
const CAMERA_POS: (f64, f64) = (0.0, 286.0);

const KEEP_OUT_X: f64 = STATION_X - 112.0;
const KEEP_OUT_Y: f64 = STATION_Y - 104.0;
const KEEP_OUT_Z: f64 = 8.0;
const FRONT_ROBOT_CLEARANCE: f64 = 520.0;
const REAR_SERVICE_CLEARANCE: f64 = 330.0;
const SIDE_REGULATOR_CLEARANCE: f64 = 240.0;
const CAMERA_SERVICE_CLEARANCE: f64 = 180.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let mfc = mfc_pocket_array();
    export(OUTPUTS[1], &mfc);

    let flow = reference_flow_meter_dock();
    export(OUTPUTS[2], &flow);

    let quick = calibration_gas_quick_connect_panel();
    export(OUTPUTS[3], &quick);

    let purge = zero_gas_purge_manifold();
    export(OUTPUTS[4], &purge);

    let regulators = pressure_regulator_envelope_bank();
    export(OUTPUTS[5], &regulators);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[6], &lanes);

    let labels = barcode_certificate_lands();
    export(OUTPUTS[7], &labels);

    let witness = leak_vent_witness_features();
    export(OUTPUTS[8], &witness);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + mfc.translate(MFC_POS.0, MFC_POS.1, insert_z(MFC_ARRAY_Z))
        + flow.translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_DOCK_Z))
        + quick.translate(QUICK_POS.0, QUICK_POS.1, insert_z(QUICK_PANEL_Z))
        + purge.translate(PURGE_POS.0, PURGE_POS.1, insert_z(PURGE_MANIFOLD_Z))
        + regulators.translate(REG_POS.0, REG_POS.1, insert_z(REG_BANK_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_TRAY_Z))
        + labels.translate(LABEL_POS.0, LABEL_POS.1, insert_z(LABEL_PANEL_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, insert_z(CAMERA_BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed gas MFC zero/span crosscheck station:");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!("  Gas channels:               {:?} with {MFC_POCKETS} MFC pockets and {REGULATOR_COUNT} regulator envelopes", GAS_NAMES);
    println!(
        "  Calibration gas interface:  {CAL_GAS_PORTS} quick-connect ports, {PURGE_LANES} zero/purge lanes, reference flow-meter dock"
    );
    println!(
        "  Release controls:           release/hold/reject lanes, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Evidence and containment:    {LEAK_WITNESS_WINDOWS} leak witness windows, {VENT_WITNESS_PORTS} vent witness ports, camera bridge, and {KEEP_OUT_ZONES} robot/service keepout zones"
    );
    println!(
        "  Service clearances:          {FRONT_ROBOT_CLEARANCE:.0}mm front robot approach, {REAR_SERVICE_CLEARANCE:.0}mm rear panel service, {SIDE_REGULATOR_CLEARANCE:.0}mm side regulator service, {CAMERA_SERVICE_CLEARANCE:.0}mm camera access"
    );
    println!(
        "  Feature groups covered:      {}",
        REQUIRED_FEATURE_GROUPS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        ("mfc_pocket_array", MFC_POS, MFC_ARRAY_X, MFC_ARRAY_Y),
        (
            "reference_flow_meter_dock",
            FLOW_POS,
            FLOW_DOCK_X,
            FLOW_DOCK_Y,
        ),
        (
            "calibration_gas_quick_connect_panel",
            QUICK_POS,
            QUICK_PANEL_X,
            QUICK_PANEL_Y,
        ),
        (
            "zero_gas_purge_manifold",
            PURGE_POS,
            PURGE_MANIFOLD_X,
            PURGE_MANIFOLD_Y,
        ),
        (
            "pressure_regulator_envelope_bank",
            REG_POS,
            REG_BANK_X,
            REG_BANK_Y,
        ),
        (
            "release_hold_reject_lanes",
            LANE_POS,
            LANE_TRAY_X,
            LANE_TRAY_Y,
        ),
        (
            "barcode_certificate_lands",
            LABEL_POS,
            LABEL_PANEL_X,
            LABEL_PANEL_Y,
        ),
        (
            "leak_vent_witness_features",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        (
            "evidence_camera_bridge",
            CAMERA_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_gas_mfc_crosscheck_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let spill_recess = centered_cube(
        "closed_gas_mfc_crosscheck_station_washdown_recess",
        STATION_X - 124.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let front_drain = centered_cylinder(
        "closed_gas_mfc_crosscheck_station_front_low_point_drain",
        10.0 / 2.0,
        48.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 78.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - spill_recess - front_drain - insert_sockets() - mounting_slots() - datum_pin_holes()
        + perimeter_rims()
        + station_zone_dividers()
        + rear_service_bulkhead_tabs()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_gas_mfc_crosscheck_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_gas_mfc_crosscheck_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-360.0, 0.0),
        (360.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_station_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_station_m6_service_slot_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_gas_mfc_crosscheck_station_datum_pin_holes");
    for (i, (x, y)) in [
        (-494.0, 314.0),
        (494.0, 314.0),
        (-494.0, -314.0),
        (494.0, -314.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_station_datum_pin_clearance_{i}"),
                6.0 / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_gas_mfc_crosscheck_station_left_spill_rim",
        RIM_W,
        STATION_Y - 66.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        -4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_gas_mfc_crosscheck_station_right_spill_rim",
        RIM_W,
        STATION_Y - 66.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        -4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_gas_mfc_crosscheck_station_rear_service_rim",
        STATION_X - 38.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_gas_mfc_crosscheck_station_front_low_spill_lip",
        STATION_X - 210.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 21.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn station_zone_dividers() -> Part {
    let rear_row = centered_cube(
        "closed_gas_mfc_crosscheck_station_mfc_regulator_row_divider",
        STATION_X - 154.0,
        10.0,
        28.0,
    )
    .translate(0.0, 74.0, BASE_Z / 2.0 + 14.0);
    let middle_row = centered_cube(
        "closed_gas_mfc_crosscheck_station_meter_quickconnect_row_divider",
        STATION_X - 178.0,
        10.0,
        24.0,
    )
    .translate(0.0, -95.0, BASE_Z / 2.0 + 12.0);
    let front_row = centered_cube(
        "closed_gas_mfc_crosscheck_station_release_traceability_row_divider",
        STATION_X - 226.0,
        8.0,
        20.0,
    )
    .translate(0.0, -238.0, BASE_Z / 2.0 + 10.0);
    let left_column = centered_cube(
        "closed_gas_mfc_crosscheck_station_reference_meter_column_split",
        10.0,
        278.0,
        24.0,
    )
    .translate(-82.0, -34.0, BASE_Z / 2.0 + 12.0);
    let right_column = centered_cube(
        "closed_gas_mfc_crosscheck_station_quickconnect_regulator_column_split",
        10.0,
        278.0,
        24.0,
    )
    .translate(82.0, -34.0, BASE_Z / 2.0 + 12.0);

    rear_row + middle_row + front_row + left_column + right_column
}

fn rear_service_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("closed_gas_mfc_crosscheck_station_rear_service_bulkhead_tabs");
    for (i, x) in [-420.0, -280.0, -140.0, 0.0, 140.0, 280.0, 420.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("closed_gas_mfc_crosscheck_station_rear_bulkhead_tab_{i}"),
            58.0,
            18.0,
            28.0,
        )
        .translate(*x, STATION_Y / 2.0 - 46.0, BASE_Z / 2.0 + 14.0);
        let tube_bore = centered_cylinder(
            format!("closed_gas_mfc_crosscheck_station_rear_bulkhead_tube_bore_{i}"),
            8.0 / 2.0,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 46.0, BASE_Z / 2.0 + 14.0);
        tabs = tabs + (tab - tube_bore);
    }
    tabs
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_gas_mfc_crosscheck_station_robot_datum_targets");
    for (i, (x, y)) in [(-498.0, 308.0), (498.0, 308.0), (-498.0, -306.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_gas_mfc_crosscheck_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn mfc_pocket_array() -> Part {
    let carrier = centered_cube(
        "closed_gas_mfc_crosscheck_mfc_pocket_array_carrier",
        MFC_ARRAY_X,
        MFC_ARRAY_Y,
        MFC_ARRAY_Z,
    );
    let rear_fence = centered_cube(
        "closed_gas_mfc_crosscheck_mfc_array_rear_connector_fence",
        MFC_ARRAY_X,
        14.0,
        MFC_ARRAY_Z + 34.0,
    )
    .translate(0.0, MFC_ARRAY_Y / 2.0 - 7.0, 17.0);

    let mut cuts = Part::empty("closed_gas_mfc_crosscheck_mfc_pocket_cuts");
    let mut lands = Part::empty("closed_gas_mfc_crosscheck_mfc_label_lands");
    let mut datum_keys = Part::empty("closed_gas_mfc_crosscheck_mfc_datum_keys");

    for gas in 0..GAS_CHANNELS {
        let x = centered_index(gas, GAS_CHANNELS, MFC_PITCH_X);
        let gas_name = GAS_NAMES[gas];
        cuts = cuts
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_purchased_mfc_envelope"),
                MFC_ENVELOPE_X,
                MFC_ENVELOPE_Y,
                MFC_ENVELOPE_Z,
            )
            .translate(x, -8.0, MFC_ARRAY_Z / 2.0 - MFC_ENVELOPE_Z / 2.0 + 4.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_inlet_tube_bore"),
                TUBE_BORE_D / 2.0,
                MFC_ARRAY_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 28.0, -2.0, 5.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_outlet_tube_bore"),
                TUBE_BORE_D / 2.0,
                MFC_ARRAY_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 28.0, -2.0, 5.0)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_cable_pigtail_slot"),
                16.0,
                30.0,
                12.0,
            )
            .translate(x, MFC_ARRAY_Y / 2.0 - 20.0, MFC_ARRAY_Z / 2.0 - 6.0);

        lands = lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_serial_barcode_land"),
                72.0,
                20.0,
                3.0,
            )
            .translate(x, -MFC_ARRAY_Y / 2.0 + 18.0, MFC_ARRAY_Z / 2.0 + 1.5)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_zero_span_status_land"),
                64.0,
                18.0,
                3.0,
            )
            .translate(x, MFC_ARRAY_Y / 2.0 - 28.0, MFC_ARRAY_Z / 2.0 + 1.5);

        datum_keys = datum_keys
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_left_spring_datum"),
                8.0,
                MFC_ENVELOPE_Y + 16.0,
                18.0,
            )
            .translate(
                x - MFC_ENVELOPE_X / 2.0 - 8.0,
                -8.0,
                MFC_ARRAY_Z / 2.0 - 9.0,
            )
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_right_soft_stop"),
                8.0,
                MFC_ENVELOPE_Y + 16.0,
                18.0,
            )
            .translate(
                x + MFC_ENVELOPE_X / 2.0 + 8.0,
                -8.0,
                MFC_ARRAY_Z / 2.0 - 9.0,
            )
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_flow_direction_arrow_land"),
                44.0,
                8.0,
                4.0,
            )
            .translate(x, 0.0, MFC_ARRAY_Z / 2.0 + 2.0);
    }

    carrier + rear_fence - cuts + lands + datum_keys + gripper_fiducials("mfc_pocket_array", 176.0)
}

fn reference_flow_meter_dock() -> Part {
    let body = centered_cube(
        "closed_gas_mfc_crosscheck_reference_flow_meter_dock_body",
        FLOW_DOCK_X,
        FLOW_DOCK_Y,
        FLOW_DOCK_Z,
    );
    let rear_stop = centered_cube(
        "closed_gas_mfc_crosscheck_reference_flow_meter_rear_stop",
        FLOW_DOCK_X - 36.0,
        14.0,
        FLOW_DOCK_Z + 26.0,
    )
    .translate(0.0, FLOW_DOCK_Y / 2.0 - 7.0, 13.0);
    let meter_pocket = centered_cube(
        "closed_gas_mfc_crosscheck_reference_flow_meter_purchased_meter_envelope",
        FLOW_METER_X,
        FLOW_METER_Y,
        FLOW_METER_Z,
    )
    .translate(0.0, -2.0, FLOW_DOCK_Z / 2.0 - FLOW_METER_Z / 2.0 + 4.0);
    let display_window = centered_cube(
        "closed_gas_mfc_crosscheck_reference_flow_meter_display_window",
        144.0,
        8.0,
        30.0,
    )
    .translate(0.0, -FLOW_DOCK_Y / 2.0 + 12.0, 6.0);
    let inlet_bore = centered_cylinder(
        "closed_gas_mfc_crosscheck_reference_flow_meter_inlet_bore",
        8.0 / 2.0,
        FLOW_DOCK_X + 18.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 16.0, 6.0);
    let outlet_bore = centered_cylinder(
        "closed_gas_mfc_crosscheck_reference_flow_meter_outlet_bore",
        8.0 / 2.0,
        FLOW_DOCK_X + 18.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -30.0, 6.0);

    let mut kinematic_lands =
        Part::empty("closed_gas_mfc_crosscheck_reference_flow_meter_kinematic_lands");
    for (i, (x, y)) in [(-112.0, 46.0), (112.0, 46.0), (0.0, -54.0)]
        .iter()
        .enumerate()
    {
        kinematic_lands = kinematic_lands
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_reference_meter_datum_boss_{i}"),
                10.0,
                7.0,
                32,
            )
            .translate(*x, *y, FLOW_DOCK_Z / 2.0 + 3.5)
            - centered_cylinder(
                format!("closed_gas_mfc_crosscheck_reference_meter_datum_center_{i}"),
                2.5,
                8.0,
                20,
            )
            .translate(*x, *y, FLOW_DOCK_Z / 2.0 + 4.0);
    }

    body + rear_stop - meter_pocket - display_window - inlet_bore - outlet_bore
        + kinematic_lands
        + gripper_fiducials("reference_flow_meter_dock", 120.0)
}

fn calibration_gas_quick_connect_panel() -> Part {
    let body = centered_cube(
        "closed_gas_mfc_crosscheck_calibration_gas_quick_connect_panel_body",
        QUICK_PANEL_X,
        QUICK_PANEL_Y,
        QUICK_PANEL_Z,
    );
    let rear_backer = centered_cube(
        "closed_gas_mfc_crosscheck_calibration_gas_quick_connect_rear_backer",
        QUICK_PANEL_X,
        14.0,
        QUICK_PANEL_Z + 30.0,
    )
    .translate(0.0, QUICK_PANEL_Y / 2.0 - 7.0, 15.0);
    let gasket_land = rectangular_gasket_land(
        "closed_gas_mfc_crosscheck_calibration_panel_front_gasket_land",
        QUICK_PANEL_X - 38.0,
        QUICK_PANEL_Z - 28.0,
        5.0,
    )
    .translate(0.0, -QUICK_PANEL_Y / 2.0 - 4.0, 0.0);

    let mut port_cuts = Part::empty("closed_gas_mfc_crosscheck_calibration_quick_connect_cuts");
    let mut label_lands = Part::empty("closed_gas_mfc_crosscheck_calibration_port_label_lands");
    for port in 0..CAL_GAS_PORTS {
        let x = centered_index(port, CAL_GAS_PORTS, QUICK_PORT_PITCH_X);
        let z = if port % 2 == 0 { 20.0 } else { -34.0 };
        let name = calibration_port_name(port);
        port_cuts = port_cuts
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{name}_quick_connect_bore"),
                QUICK_CONNECT_D / 2.0,
                QUICK_PANEL_Y + 12.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -2.0, z)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_wrench_clearance_flat"),
                24.0,
                18.0,
                12.0,
            )
            .translate(x, -QUICK_PANEL_Y / 2.0 + 14.0, z);
        label_lands = label_lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_port_label_land"),
                42.0,
                4.0,
                18.0,
            )
            .translate(x, -QUICK_PANEL_Y / 2.0 - 6.0, z + 28.0);
    }

    let manifold_route_window = centered_cube(
        "closed_gas_mfc_crosscheck_calibration_panel_internal_tube_route_window",
        QUICK_PANEL_X - 72.0,
        18.0,
        22.0,
    )
    .translate(0.0, 12.0, -QUICK_PANEL_Z / 2.0 + 20.0);

    body + rear_backer - port_cuts - manifold_route_window
        + label_lands
        + gasket_land
        + gripper_fiducials("calibration_gas_quick_connect_panel", 150.0)
}

fn calibration_port_name(index: usize) -> &'static str {
    match index {
        0 => "co2_zero_n2",
        1 => "co2_span_mix",
        2 => "o2_zero_n2",
        3 => "o2_span_air",
        4 => "n2_span_reference",
        5 => "air_span_reference",
        6 => "reference_meter_bypass",
        _ => "vent_exhaust_return",
    }
}

fn zero_gas_purge_manifold() -> Part {
    let body = centered_cube(
        "closed_gas_mfc_crosscheck_zero_gas_purge_manifold_body",
        PURGE_MANIFOLD_X,
        PURGE_MANIFOLD_Y,
        PURGE_MANIFOLD_Z,
    );
    let front_sight_strip = centered_cube(
        "closed_gas_mfc_crosscheck_zero_gas_purge_front_sight_strip",
        PURGE_MANIFOLD_X - 52.0,
        8.0,
        18.0,
    )
    .translate(0.0, -PURGE_MANIFOLD_Y / 2.0 - 5.0, 4.0);

    let mut bores = Part::empty("closed_gas_mfc_crosscheck_zero_gas_purge_lane_bores");
    let mut valve_lands = Part::empty("closed_gas_mfc_crosscheck_zero_gas_purge_valve_lands");
    for lane in 0..PURGE_LANES {
        let x = centered_index(lane, PURGE_LANES, PURGE_LANE_PITCH_X);
        let label = purge_lane_name(lane);
        bores = bores
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{label}_purge_lane_longitudinal_bore"),
                TUBE_BORE_D / 2.0,
                PURGE_MANIFOLD_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{label}_zero_gas_drop_bore"),
                TUBE_BORE_D / 2.0,
                PURGE_MANIFOLD_Z + 10.0,
                24,
            )
            .translate(x, 18.0, 0.0);
        valve_lands = valve_lands
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{label}_purge_valve_stem_land"),
                12.0,
                7.0,
                32,
            )
            .translate(x, 18.0, PURGE_MANIFOLD_Z / 2.0 + 3.5)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{label}_purge_route_label_tick"),
                44.0,
                8.0,
                4.0,
            )
            .translate(
                x,
                -PURGE_MANIFOLD_Y / 2.0 + 18.0,
                PURGE_MANIFOLD_Z / 2.0 + 2.0,
            );
    }

    let common_zero_header = centered_cylinder(
        "closed_gas_mfc_crosscheck_common_zero_gas_header_bore",
        7.0 / 2.0,
        PURGE_MANIFOLD_X - 72.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, 0.0);
    let vent_return_header = centered_cylinder(
        "closed_gas_mfc_crosscheck_common_vent_return_header_bore",
        7.0 / 2.0,
        PURGE_MANIFOLD_X - 72.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -24.0, -10.0);

    body - bores - common_zero_header - vent_return_header
        + valve_lands
        + front_sight_strip
        + gripper_fiducials("zero_gas_purge_manifold", 220.0)
}

fn purge_lane_name(index: usize) -> &'static str {
    match index {
        0 => "co2",
        1 => "o2",
        2 => "n2",
        3 => "air",
        _ => "reference_meter",
    }
}

fn pressure_regulator_envelope_bank() -> Part {
    let carrier = centered_cube(
        "closed_gas_mfc_crosscheck_pressure_regulator_bank_carrier",
        REG_BANK_X,
        REG_BANK_Y,
        REG_BANK_Z,
    );
    let rear_service_fence = centered_cube(
        "closed_gas_mfc_crosscheck_pressure_regulator_bank_rear_service_fence",
        REG_BANK_X,
        14.0,
        REG_BANK_Z + 38.0,
    )
    .translate(0.0, REG_BANK_Y / 2.0 - 7.0, 19.0);

    let mut envelope_cuts =
        Part::empty("closed_gas_mfc_crosscheck_pressure_regulator_envelope_cuts");
    let mut gauge_faces = Part::empty("closed_gas_mfc_crosscheck_pressure_regulator_gauge_faces");
    let mut trim_lands = Part::empty("closed_gas_mfc_crosscheck_pressure_regulator_trim_lands");

    for gas in 0..GAS_CHANNELS {
        let x = centered_index(gas, GAS_CHANNELS, REG_PITCH_X);
        let gas_name = GAS_NAMES[gas];
        envelope_cuts = envelope_cuts
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_purchased_regulator_envelope"),
                REG_ENVELOPE_X,
                REG_ENVELOPE_Y,
                REG_ENVELOPE_Z,
            )
            .translate(x, -8.0, REG_BANK_Z / 2.0 - REG_ENVELOPE_Z / 2.0 + 5.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_regulator_inlet_bore"),
                8.0 / 2.0,
                REG_BANK_Y + 14.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 22.0, 0.0, 0.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_regulator_outlet_bore"),
                8.0 / 2.0,
                REG_BANK_Y + 14.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 22.0, 0.0, 0.0);

        gauge_faces = gauge_faces
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_regulator_gauge_face_land"),
                21.0,
                6.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -REG_BANK_Y / 2.0 - 3.0, 18.0)
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_{gas_name}_regulator_gauge_center_mark"),
                5.0,
                7.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -REG_BANK_Y / 2.0 - 4.0, 18.0);

        trim_lands = trim_lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{gas_name}_pressure_setpoint_card_land"),
                70.0,
                20.0,
                4.0,
            )
            .translate(x, REG_BANK_Y / 2.0 - 24.0, REG_BANK_Z / 2.0 + 2.0);
    }

    carrier + rear_service_fence - envelope_cuts
        + gauge_faces
        + trim_lands
        + gripper_fiducials("pressure_regulator_envelope_bank", 142.0)
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_gas_mfc_crosscheck_release_hold_reject_lane_tray_body",
        LANE_TRAY_X,
        LANE_TRAY_Y,
        LANE_TRAY_Z,
    );
    let mut lane_cuts = Part::empty("closed_gas_mfc_crosscheck_status_lane_recesses");
    let mut lane_rails = Part::empty("closed_gas_mfc_crosscheck_status_lane_rails");

    for lane in 0..LANE_COUNT {
        let x = centered_index(lane, LANE_COUNT, LANE_PITCH_X);
        let name = status_lane_name(lane);
        lane_cuts = lane_cuts
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_card_recess"),
                108.0,
                74.0,
                LANE_TRAY_Z - 8.0,
            )
            .translate(x, -6.0, LANE_TRAY_Z / 2.0 - (LANE_TRAY_Z - 8.0) / 2.0 + 4.0);
        lane_rails = lane_rails
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_lane_left_guide"),
                8.0,
                86.0,
                20.0,
            )
            .translate(x - 58.0, -4.0, LANE_TRAY_Z / 2.0 + 10.0)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_lane_right_guide"),
                8.0,
                86.0,
                20.0,
            )
            .translate(x + 58.0, -4.0, LANE_TRAY_Z / 2.0 + 10.0)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_{name}_status_label_land"),
                84.0,
                20.0,
                3.0,
            )
            .translate(x, LANE_TRAY_Y / 2.0 - 18.0, LANE_TRAY_Z / 2.0 + 1.5);
    }

    let interlock_bar = centered_cube(
        "closed_gas_mfc_crosscheck_release_hold_reject_interlock_bar",
        LANE_TRAY_X - 36.0,
        10.0,
        16.0,
    )
    .translate(0.0, -LANE_TRAY_Y / 2.0 + 16.0, LANE_TRAY_Z / 2.0 + 8.0);

    body - lane_cuts + lane_rails + interlock_bar + gripper_fiducials("status_lane_tray", 128.0)
}

fn status_lane_name(index: usize) -> &'static str {
    match index {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_gas_mfc_crosscheck_barcode_certificate_land_panel",
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    );
    let mut barcode_lands = Part::empty("closed_gas_mfc_crosscheck_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 6;
        let col = i % 6;
        let x = centered_index(col, 6, 62.0);
        let y = if row == 0 { 30.0 } else { 5.0 };
        barcode_lands = barcode_lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_barcode_label_land_{i}"),
                52.0,
                18.0,
                2.5,
            )
            .translate(x, y, LABEL_PANEL_Z / 2.0 + 1.25);
    }

    let mut certificate_lands = Part::empty("closed_gas_mfc_crosscheck_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 94.0);
        certificate_lands = certificate_lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_traceable_gas_certificate_land_{i}"),
                82.0,
                30.0,
                2.5,
            )
            .translate(x, -30.0, LABEL_PANEL_Z / 2.0 + 1.25);
    }

    let witness_scale = centered_cube(
        "closed_gas_mfc_crosscheck_evidence_photo_scale_land",
        LABEL_PANEL_X - 58.0,
        4.0,
        3.0,
    )
    .translate(0.0, -LABEL_PANEL_Y / 2.0 + 12.0, LABEL_PANEL_Z / 2.0 + 1.5);

    let mut clip_holes = Part::empty("closed_gas_mfc_crosscheck_label_panel_clip_holes");
    for (i, x) in [
        -(LABEL_PANEL_X / 2.0 - 18.0),
        0.0,
        LABEL_PANEL_X / 2.0 - 18.0,
    ]
    .iter()
    .enumerate()
    {
        clip_holes = clip_holes
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_traceability_clip_hole_{i}"),
                3.0 / 2.0,
                LABEL_PANEL_Z + 4.0,
                18,
            )
            .translate(*x, 0.0, 0.0);
    }

    panel - clip_holes + barcode_lands + certificate_lands + witness_scale
}

fn leak_vent_witness_features() -> Part {
    let body = centered_cube(
        "closed_gas_mfc_crosscheck_leak_vent_witness_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let vent_gutter = centered_cube(
        "closed_gas_mfc_crosscheck_leak_vent_common_gutter",
        WITNESS_X - 56.0,
        14.0,
        12.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 16.0, WITNESS_Z / 2.0 - 6.0);

    let mut witness_cuts = Part::empty("closed_gas_mfc_crosscheck_witness_cuts");
    let mut witness_lands = Part::empty("closed_gas_mfc_crosscheck_witness_lands");
    for i in 0..LEAK_WITNESS_WINDOWS {
        let x = centered_index(i, LEAK_WITNESS_WINDOWS, 192.0);
        witness_cuts = witness_cuts
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_leak_witness_card_slot_{i}"),
                138.0,
                10.0,
                WITNESS_Z + 4.0,
            )
            .translate(x, 7.0, 0.0)
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_leak_witness_dye_sump_{i}"),
                104.0,
                18.0,
                12.0,
            )
            .translate(x, -18.0, WITNESS_Z / 2.0 - 6.0);
        witness_lands = witness_lands
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_leak_witness_time_label_land_{i}"),
                128.0,
                12.0,
                3.0,
            )
            .translate(x, WITNESS_Y / 2.0 - 10.0, WITNESS_Z / 2.0 + 1.5);
    }

    let mut vent_ports = Part::empty("closed_gas_mfc_crosscheck_vent_witness_ports");
    for i in 0..VENT_WITNESS_PORTS {
        let x = centered_index(i, VENT_WITNESS_PORTS, 116.0);
        vent_ports = vent_ports
            + centered_cylinder(
                format!("closed_gas_mfc_crosscheck_vent_bubbler_witness_port_{i}"),
                8.0 / 2.0,
                WITNESS_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -4.0);
    }

    body - vent_gutter - witness_cuts - vent_ports + witness_lands
}

fn evidence_camera_bridge() -> Part {
    let left_post = camera_bridge_post("left").translate(
        -CAMERA_BRIDGE_X / 2.0 + 32.0,
        0.0,
        -CAMERA_BRIDGE_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = camera_bridge_post("right").translate(
        CAMERA_BRIDGE_X / 2.0 - 32.0,
        0.0,
        -CAMERA_BRIDGE_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "closed_gas_mfc_crosscheck_evidence_camera_bridge_crossbeam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        26.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 18.0);
    let camera_sled = centered_cube(
        "closed_gas_mfc_crosscheck_evidence_camera_mount_sled",
        168.0,
        CAMERA_BRIDGE_Y + 18.0,
        34.0,
    )
    .translate(0.0, -2.0, CAMERA_BRIDGE_Z / 2.0 - 52.0);
    let lens_bore = centered_cylinder(
        "closed_gas_mfc_crosscheck_evidence_camera_lens_clearance",
        30.0 / 2.0,
        42.0,
        40,
    )
    .translate(0.0, -2.0, CAMERA_BRIDGE_Z / 2.0 - 52.0);
    let light_bar_front = centered_cube(
        "closed_gas_mfc_crosscheck_evidence_front_light_bar",
        CAMERA_BRIDGE_X - 132.0,
        10.0,
        12.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 - 10.0,
        CAMERA_BRIDGE_Z / 2.0 - 42.0,
    );
    let light_bar_rear = centered_cube(
        "closed_gas_mfc_crosscheck_evidence_rear_light_bar",
        CAMERA_BRIDGE_X - 132.0,
        10.0,
        12.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_Y / 2.0 + 10.0,
        CAMERA_BRIDGE_Z / 2.0 - 42.0,
    );

    left_post
        + right_post
        + beam
        + (camera_sled - lens_bore)
        + light_bar_front
        + light_bar_rear
        + evidence_scale_ticks()
}

fn camera_bridge_post(side: &str) -> Part {
    let post = centered_cube(
        format!("closed_gas_mfc_crosscheck_evidence_camera_bridge_{side}_post"),
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    );
    let cable_slot = centered_cube(
        format!("closed_gas_mfc_crosscheck_evidence_camera_bridge_{side}_cable_slot"),
        12.0,
        CAMERA_BRIDGE_Y + 6.0,
        96.0,
    )
    .translate(0.0, 0.0, 20.0);
    let tie_bore = centered_cylinder(
        format!("closed_gas_mfc_crosscheck_evidence_camera_bridge_{side}_tie_bore"),
        4.2 / 2.0,
        CAMERA_BRIDGE_Y + 8.0,
        18,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 58.0);
    post - cable_slot - tie_bore
}

fn evidence_scale_ticks() -> Part {
    let mut ticks = Part::empty("closed_gas_mfc_crosscheck_evidence_camera_scale_ticks");
    for i in 0..11 {
        let x = centered_index(i, 11, 72.0);
        let height = if i % 5 == 0 { 22.0 } else { 12.0 };
        ticks = ticks
            + centered_cube(
                format!("closed_gas_mfc_crosscheck_evidence_scale_tick_{i}"),
                3.0,
                8.0,
                height,
            )
            .translate(
                x,
                -CAMERA_BRIDGE_Y / 2.0 - 18.0,
                CAMERA_BRIDGE_Z / 2.0 - 60.0,
            );
    }
    ticks
}

fn robot_service_keepout_gauge() -> Part {
    let robot_sweep = rectangular_frame(
        "closed_gas_mfc_crosscheck_front_robot_sweep_keepout",
        KEEP_OUT_X,
        168.0,
        KEEP_OUT_Z,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0, 0.0);
    let rear_service = rectangular_frame(
        "closed_gas_mfc_crosscheck_rear_service_keepout",
        KEEP_OUT_X - 56.0,
        REAR_SERVICE_CLEARANCE,
        KEEP_OUT_Z,
        8.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE / 2.0, 0.0);
    let left_regulator_service = rectangular_frame(
        "closed_gas_mfc_crosscheck_left_side_regulator_service_keepout",
        SIDE_REGULATOR_CLEARANCE,
        KEEP_OUT_Y - 120.0,
        KEEP_OUT_Z,
        8.0,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_REGULATOR_CLEARANCE / 2.0,
        -20.0,
        0.0,
    );
    let right_regulator_service = rectangular_frame(
        "closed_gas_mfc_crosscheck_right_side_regulator_service_keepout",
        SIDE_REGULATOR_CLEARANCE,
        KEEP_OUT_Y - 120.0,
        KEEP_OUT_Z,
        8.0,
    )
    .translate(STATION_X / 2.0 - SIDE_REGULATOR_CLEARANCE / 2.0, -20.0, 0.0);
    let camera_access = rectangular_frame(
        "closed_gas_mfc_crosscheck_camera_service_keepout",
        CAMERA_BRIDGE_X - 92.0,
        CAMERA_SERVICE_CLEARANCE,
        KEEP_OUT_Z,
        8.0,
    )
    .translate(0.0, CAMERA_POS.1 - CAMERA_SERVICE_CLEARANCE / 2.0, 0.0);

    robot_sweep + rear_service + left_regulator_service + right_regulator_service + camera_access
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 13.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_bore"), 2.6, 7.0, 20);
    disc - center
}

fn gripper_fiducials(name: &str, span_x: f64) -> Part {
    let left = fiducial_disc(&format!(
        "closed_gas_mfc_crosscheck_{name}_left_grip_fiducial"
    ))
    .translate(-span_x / 2.0, 0.0, 4.0);
    let right = fiducial_disc(&format!(
        "closed_gas_mfc_crosscheck_{name}_right_grip_fiducial"
    ))
    .translate(span_x / 2.0, 0.0, 4.0);
    left + right
}

fn rectangular_gasket_land(name: &str, x: f64, z: f64, rail: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), x, rail, rail).translate(0.0, 0.0, z / 2.0);
    let bottom =
        centered_cube(format!("{name}_bottom"), x, rail, rail).translate(0.0, 0.0, -z / 2.0);
    let left = centered_cube(format!("{name}_left"), rail, rail, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, rail, z).translate(x / 2.0, 0.0, 0.0);
    top + bottom + left + right
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear"), x, rail, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}
