use std::collections::BTreeSet;
use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent thaw bubble-ingress and connector-wetness validation station.
//
// Intent:
// - Present thawed reagent/media bags and vials in closed nests with no cell
//   processing, culture guidance, or acceptance thresholds.
// - Make connector wetness witness slots, bubble-ingress optical windows,
//   check-valve orientation, dripless quick-connect capture, pressure-hold
//   witness ports, custody labeling, disposition lanes, evidence capture, and
//   robot/service keepouts physically visible in one validation fixture.
// - Model engineering validation CAD only. Purchased bags, connectors, valves,
//   optics, sensors, and wetted materials remain external controlled items.

const OUTPUT_PREFIX: &str = "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_containment_deck.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_thawed_bag_vial_nest.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_connector_wetness_witness_slots.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_bubble_ingress_optical_windows.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_check_valve_orientation_rail.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_dripless_quick_connect_capture_tray.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_temperature_time_token_rail.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_pressure_hold_witness_ports.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_barcode_lot_custody_plate.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_release_hold_reject_lanes.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_reagent_thaw_bubble_ingress_connector_wetness_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "thawed_bag_vial_nest",
    "connector_wetness_witness_slots",
    "bubble_ingress_optical_windows",
    "check_valve_orientation_rail",
    "dripless_quick_connect_capture_tray",
    "temperature_time_token_rail",
    "pressure_hold_witness_ports",
    "barcode_lot_custody_plate",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "no_biological_protocol",
    "no_acceptance_thresholds",
    "not_pressure_rated_hardware",
    "purchased_wetted_components_external",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 46.0;
const SUMP_X: f64 = 1160.0;
const SUMP_Y: f64 = 680.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_TARGETS: usize = 5;

const NEST_POS: (f64, f64) = (-392.0, 142.0);
const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 276.0;
const NEST_Z: f64 = 52.0;
const BAG_BAYS: usize = 2;
const BAG_SLOT_X: f64 = 162.0;
const BAG_SLOT_Y: f64 = 206.0;
const BAG_SLOT_DEPTH: f64 = 20.0;
const VIAL_WELLS: usize = 8;
const VIAL_WELL_D: f64 = 25.0;
const VIAL_WELL_DEPTH: f64 = 22.0;
const NEST_DRAIN_SLOTS: usize = 6;
const BAG_RETENTION_FINGERS: usize = 8;

const WETNESS_POS: (f64, f64) = (-396.0, -164.0);
const WETNESS_BLOCK_X: f64 = 430.0;
const WETNESS_BLOCK_Y: f64 = 172.0;
const WETNESS_BLOCK_Z: f64 = 34.0;
const WETNESS_SLOTS: usize = 12;
const WETNESS_SLOT_X: f64 = 18.0;
const WETNESS_SLOT_Y: f64 = 118.0;
const WETNESS_SLOT_DEPTH: f64 = 16.0;
const WETNESS_WICK_WELLS: usize = WETNESS_SLOTS;

const OPTICAL_POS: (f64, f64) = (20.0, 74.0);
const OPTICAL_BASE_X: f64 = 560.0;
const OPTICAL_BASE_Y: f64 = 192.0;
const OPTICAL_BASE_Z: f64 = 28.0;
const OPTICAL_UPRIGHT_Z: f64 = 126.0;
const OPTICAL_WINDOWS: usize = 6;
const WINDOW_X: f64 = 52.0;
const WINDOW_Y: f64 = 20.0;
const WINDOW_Z: f64 = 34.0;
const CAMERA_LANDS: usize = OPTICAL_WINDOWS;
const ILLUMINATOR_LANDS: usize = OPTICAL_WINDOWS;

const VALVE_POS: (f64, f64) = (20.0, 278.0);
const VALVE_RAIL_X: f64 = 670.0;
const VALVE_RAIL_Y: f64 = 86.0;
const VALVE_RAIL_Z: f64 = 42.0;
const CHECK_VALVE_POSITIONS: usize = 6;
const VALVE_POCKET_X: f64 = 70.0;
const VALVE_POCKET_Y: f64 = 32.0;
const VALVE_POCKET_DEPTH: f64 = 18.0;
const ORIENTATION_KEYS: usize = CHECK_VALVE_POSITIONS;

const QC_POS: (f64, f64) = (414.0, -142.0);
const QC_TRAY_X: f64 = 386.0;
const QC_TRAY_Y: f64 = 226.0;
const QC_TRAY_Z: f64 = 46.0;
const QUICK_CONNECT_DOCKS: usize = 6;
const QC_DOCK_D: f64 = 30.0;
const QC_DOCK_PITCH_X: f64 = 52.0;
const QC_DRIP_WELLS: usize = QUICK_CONNECT_DOCKS;
const QC_CAPTURE_PAD_X: f64 = 300.0;
const QC_CAPTURE_PAD_Y: f64 = 76.0;

const TOKEN_POS: (f64, f64) = (-18.0, -308.0);
const TOKEN_RAIL_X: f64 = 610.0;
const TOKEN_RAIL_Y: f64 = 78.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const TOKEN_SLOTS: usize = 10;
const TOKEN_SLOT_X: f64 = 42.0;
const TOKEN_SLOT_Y: f64 = 48.0;
const TIME_INDEX_TICKS: usize = 11;

const PRESSURE_POS: (f64, f64) = (456.0, 132.0);
const PRESSURE_PANEL_X: f64 = 300.0;
const PRESSURE_PANEL_Y: f64 = 164.0;
const PRESSURE_PANEL_Z: f64 = 56.0;
const PRESSURE_WITNESS_PORTS: usize = 6;
const PRESSURE_PORT_D: f64 = 12.0;
const PRESSURE_PORT_PITCH_X: f64 = 48.0;
const REFERENCE_BLIND_PORTS: usize = 3;
const PRESSURE_SEAL_LANDS: usize = PRESSURE_WITNESS_PORTS;

const CUSTODY_POS: (f64, f64) = (-454.0, -312.0);
const CUSTODY_PLATE_X: f64 = 318.0;
const CUSTODY_PLATE_Y: f64 = 104.0;
const CUSTODY_PLATE_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 4;
const LOT_CARD_LANDS: usize = 3;
const CHAIN_OF_CUSTODY_TOKEN_WELLS: usize = 4;

const DISPOSITION_POS: (f64, f64) = (340.0, -306.0);
const DISPOSITION_X: f64 = 426.0;
const DISPOSITION_Y: f64 = 112.0;
const DISPOSITION_Z: f64 = 30.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 46.0;
const STATUS_SLOT_Y: f64 = 28.0;

const BRIDGE_POS: (f64, f64) = (0.0, 338.0);
const BRIDGE_SPAN_X: f64 = 940.0;
const BRIDGE_POST_X: f64 = 24.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_POST_Z: f64 = 188.0;
const BRIDGE_BEAM_Y: f64 = 42.0;
const BRIDGE_BEAM_Z: f64 = 22.0;
const EVIDENCE_CAMERA_CLEARANCE_Z: f64 = 136.0;

const ROBOT_FRONT_APPROACH: f64 = 410.0;
const ROBOT_SWEEP_X: f64 = 1110.0;
const ROBOT_SWEEP_Y: f64 = 676.0;
const ROBOT_SWEEP_Z: f64 = 170.0;
const REAR_CONNECTOR_SERVICE: f64 = 240.0;
const RIGHT_PRESSURE_SERVICE: f64 = 210.0;
const LEFT_CUSTODY_SERVICE: f64 = 180.0;
const TOP_OPTICAL_SERVICE: f64 = 310.0;
const KEEP_OUT_RAIL: f64 = 7.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let nest = thawed_bag_vial_nest();
    export(&nest, OUTPUTS[1]);

    let wetness = connector_wetness_witness_slots();
    export(&wetness, OUTPUTS[2]);

    let optics = bubble_ingress_optical_windows();
    export(&optics, OUTPUTS[3]);

    let valves = check_valve_orientation_rail();
    export(&valves, OUTPUTS[4]);

    let quick_connect = dripless_quick_connect_capture_tray();
    export(&quick_connect, OUTPUTS[5]);

    let token = temperature_time_token_rail();
    export(&token, OUTPUTS[6]);

    let pressure = pressure_hold_witness_ports();
    export(&pressure, OUTPUTS[7]);

    let custody = barcode_lot_custody_plate();
    export(&custody, OUTPUTS[8]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[9]);

    let evidence = evidence_bridge_robot_service_keepouts();
    export(&evidence, OUTPUTS[10]);

    let assembly = station_assembly();
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed reagent thaw bubble-ingress connector-wetness validation station:");
    println!(
        "  Containment deck:           {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm sump, {DRAIN_D:.0}mm drain, {DATUM_TARGETS} datum targets"
    );
    println!(
        "  Thawed reagent nest:        {BAG_BAYS} closed bag bays, {VIAL_WELLS} vial wells, {NEST_DRAIN_SLOTS} drain witness slots, {BAG_RETENTION_FINGERS} retention fingers"
    );
    println!(
        "  Connector wetness witness:  {WETNESS_SLOTS} witness slots with {WETNESS_WICK_WELLS} wick wells and dry/wet comparison lands"
    );
    println!(
        "  Bubble ingress optics:      {OPTICAL_WINDOWS} optical windows, {CAMERA_LANDS} camera lands, {ILLUMINATOR_LANDS} illuminator lands"
    );
    println!(
        "  Closed connectors:          {CHECK_VALVE_POSITIONS} check-valve positions, {QUICK_CONNECT_DOCKS} dripless quick-connect docks, {PRESSURE_WITNESS_PORTS} pressure-hold witness ports"
    );
    println!(
        "  Custody/disposition:        {TOKEN_SLOTS} temperature/time token slots, {BARCODE_LANDS} barcode lands, {LOT_CARD_LANDS} lot-card lands, {STATUS_LANES} release/hold/reject lanes"
    );
    println!(
        "  Robot/service keepouts:     front robot {ROBOT_FRONT_APPROACH:.0}mm, rear connector service {REAR_CONNECTOR_SERVICE:.0}mm, right pressure service {RIGHT_PRESSURE_SERVICE:.0}mm, top optical service {TOP_OPTICAL_SERVICE:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + thawed_bag_vial_nest().translate(NEST_POS.0, NEST_POS.1, BASE_Z)
        + connector_wetness_witness_slots().translate(WETNESS_POS.0, WETNESS_POS.1, BASE_Z)
        + bubble_ingress_optical_windows().translate(OPTICAL_POS.0, OPTICAL_POS.1, BASE_Z)
        + check_valve_orientation_rail().translate(VALVE_POS.0, VALVE_POS.1, BASE_Z)
        + dripless_quick_connect_capture_tray().translate(QC_POS.0, QC_POS.1, BASE_Z)
        + temperature_time_token_rail().translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z)
        + pressure_hold_witness_ports().translate(PRESSURE_POS.0, PRESSURE_POS.1, BASE_Z)
        + barcode_lot_custody_plate().translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z)
        + release_hold_reject_lanes().translate(DISPOSITION_POS.0, DISPOSITION_POS.1, BASE_Z)
        + evidence_bridge_robot_service_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "reagent_thaw_bubble_wetness_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "reagent_thaw_bubble_wetness_recessed_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "reagent_thaw_bubble_wetness_front_drain_cut",
        DRAIN_D / 2.0,
        CURB_W + 38.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 98.0,
        -STATION_Y / 2.0 + 12.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain - mount_holes() - module_socket_recesses()
        + containment_curbs()
        + datum_targets()
        + deck_flow_ribs()
        + zone_markers()
}

fn containment_curbs() -> Part {
    let z = BASE_Z + CURB_Z / 2.0;
    let front = centered_cube(
        "reagent_thaw_bubble_wetness_front_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, z);
    let rear = centered_cube(
        "reagent_thaw_bubble_wetness_rear_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, z);
    let left = centered_cube(
        "reagent_thaw_bubble_wetness_left_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, z);
    let right = centered_cube(
        "reagent_thaw_bubble_wetness_right_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("reagent_thaw_bubble_wetness_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 72.0, -STATION_Y / 2.0 + 68.0),
        (STATION_X / 2.0 - 72.0, -STATION_Y / 2.0 + 68.0),
        (-STATION_X / 2.0 + 72.0, STATION_Y / 2.0 - 68.0),
        (STATION_X / 2.0 - 72.0, STATION_Y / 2.0 - 68.0),
        (0.0, -STATION_Y / 2.0 + 68.0),
        (0.0, STATION_Y / 2.0 - 68.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn module_socket_recesses() -> Part {
    let mut recesses = Part::empty("reagent_thaw_bubble_wetness_module_socket_recesses");
    for footprint in module_footprints() {
        recesses = recesses
            + centered_cube(
                format!(
                    "reagent_thaw_bubble_wetness_{}_socket_recess",
                    footprint.name
                ),
                footprint.x + 22.0,
                footprint.y + 18.0,
                SUMP_DEPTH + 1.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SUMP_DEPTH / 2.0 + 0.5,
            );
    }
    recesses
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("reagent_thaw_bubble_wetness_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 108.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 108.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 108.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 104.0),
        (0.0, STATION_Y / 2.0 - 104.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_datum_ring_{i}"),
            18.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z + 2.5);
        let dot = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_datum_center_cut_{i}"),
            4.0,
            6.0,
            24,
        )
        .translate(x, y, BASE_Z + 3.0);
        targets = targets + (ring - dot);
    }
    targets
}

fn deck_flow_ribs() -> Part {
    let mut ribs = Part::empty("reagent_thaw_bubble_wetness_deck_flow_ribs");
    for i in 0..9 {
        ribs = ribs
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_sump_flow_witness_rib_{i}"),
                SUMP_X - 132.0,
                5.0,
                4.0,
            )
            .translate(0.0, centered_index(i, 9, 66.0), BASE_Z + 2.0);
    }
    ribs
}

fn zone_markers() -> Part {
    let mut markers = Part::empty("reagent_thaw_bubble_wetness_zone_markers");
    for footprint in module_footprints() {
        markers = markers
            + centered_cube(
                format!(
                    "reagent_thaw_bubble_wetness_{}_floor_zone_marker",
                    footprint.name
                ),
                footprint.x + 34.0,
                footprint.y + 30.0,
                3.0,
            )
            .translate(footprint.center.0, footprint.center.1, BASE_Z + 1.5);
    }
    markers
}

fn thawed_bag_vial_nest() -> Part {
    let body = centered_cube(
        "reagent_thaw_bubble_wetness_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let mut bag_cuts = Part::empty("reagent_thaw_bubble_wetness_bag_bay_cuts");
    for bay in 0..BAG_BAYS {
        bag_cuts = bag_cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_bag_bay_{bay}_closed_recess"),
                BAG_SLOT_X,
                BAG_SLOT_Y,
                BAG_SLOT_DEPTH + 1.0,
            )
            .translate(
                centered_index(bay, BAG_BAYS, 188.0) - 28.0,
                4.0,
                NEST_Z - BAG_SLOT_DEPTH / 2.0 + 0.5,
            );
    }

    body - bag_cuts - vial_well_cuts()
        + bag_retention_fingers()
        + nest_drain_slots()
        + thaw_state_id_lands()
}

fn vial_well_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_vial_well_cuts");
    for well in 0..VIAL_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_vial_well_{well}_cut"),
                VIAL_WELL_D / 2.0,
                VIAL_WELL_DEPTH + 1.0,
                32,
            )
            .translate(
                centered_index(well % 4, 4, 42.0) + 4.0,
                -NEST_Y / 2.0 + 36.0 + (well / 4) as f64 * 44.0,
                NEST_Z - VIAL_WELL_DEPTH / 2.0 + 0.5,
            );
    }
    cuts
}

fn bag_retention_fingers() -> Part {
    let mut fingers = Part::empty("reagent_thaw_bubble_wetness_bag_retention_fingers");
    for bay in 0..BAG_BAYS {
        let x = centered_index(bay, BAG_BAYS, 188.0) - 28.0;
        for (side, y) in [
            ("front", -BAG_SLOT_Y / 2.0 - 14.0),
            ("rear", BAG_SLOT_Y / 2.0 + 14.0),
        ] {
            for i in 0..2 {
                fingers = fingers
                    + centered_cube(
                        format!(
                            "reagent_thaw_bubble_wetness_bay_{bay}_{side}_retention_finger_{i}"
                        ),
                        56.0,
                        12.0,
                        20.0,
                    )
                    .translate(
                        x + centered_index(i, 2, 82.0),
                        y,
                        NEST_Z + 10.0,
                    );
            }
        }
    }
    fingers
}

fn nest_drain_slots() -> Part {
    let mut slots = Part::empty("reagent_thaw_bubble_wetness_nest_drain_slots");
    for slot in 0..NEST_DRAIN_SLOTS {
        slots = slots
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_nest_drain_slot_{slot}"),
                72.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(slot, NEST_DRAIN_SLOTS, 58.0),
                -NEST_Y / 2.0 + 16.0,
                NEST_Z + 2.0,
            );
    }
    slots
}

fn thaw_state_id_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_thaw_state_id_lands");
    for bay in 0..BAG_BAYS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_bag_bay_{bay}_thaw_state_label_land"),
                104.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(bay, BAG_BAYS, 188.0) - 28.0,
                NEST_Y / 2.0 - 24.0,
                NEST_Z + 2.0,
            );
    }
    lands
}

fn connector_wetness_witness_slots() -> Part {
    let body = centered_cube(
        "reagent_thaw_bubble_wetness_witness_block",
        WETNESS_BLOCK_X,
        WETNESS_BLOCK_Y,
        WETNESS_BLOCK_Z,
    )
    .translate(0.0, 0.0, WETNESS_BLOCK_Z / 2.0);
    let mut slot_cuts = Part::empty("reagent_thaw_bubble_wetness_connector_slot_cuts");
    let mut witness = Part::empty("reagent_thaw_bubble_wetness_connector_witness_features");
    for slot in 0..WETNESS_SLOTS {
        let x = centered_index(slot, WETNESS_SLOTS, 32.0);
        slot_cuts = slot_cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_connector_{slot}_wick_slot_cut"),
                WETNESS_SLOT_X,
                WETNESS_SLOT_Y,
                WETNESS_SLOT_DEPTH + 1.0,
            )
            .translate(x, 0.0, WETNESS_BLOCK_Z - WETNESS_SLOT_DEPTH / 2.0 + 0.5)
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_connector_{slot}_round_wick_well_cut"),
                8.0,
                WETNESS_BLOCK_Z + 4.0,
                24,
            )
            .translate(x, WETNESS_BLOCK_Y / 2.0 - 26.0, WETNESS_BLOCK_Z / 2.0);

        witness = witness
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_connector_{slot}_dry_reference_land"),
                22.0,
                16.0,
                3.0,
            )
            .translate(x, -WETNESS_BLOCK_Y / 2.0 + 20.0, WETNESS_BLOCK_Z + 1.5)
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_connector_{slot}_wetness_witness_label_land"),
                22.0,
                16.0,
                3.0,
            )
            .translate(x, WETNESS_BLOCK_Y / 2.0 - 20.0, WETNESS_BLOCK_Z + 1.5);
    }
    body - slot_cuts + witness + wetness_compare_rail()
}

fn wetness_compare_rail() -> Part {
    centered_cube(
        "reagent_thaw_bubble_wetness_dry_wet_comparison_rail",
        WETNESS_BLOCK_X - 46.0,
        10.0,
        12.0,
    )
    .translate(0.0, 0.0, WETNESS_BLOCK_Z + 6.0)
}

fn bubble_ingress_optical_windows() -> Part {
    let base = centered_cube(
        "reagent_thaw_bubble_wetness_optical_window_base",
        OPTICAL_BASE_X,
        OPTICAL_BASE_Y,
        OPTICAL_BASE_Z,
    )
    .translate(0.0, 0.0, OPTICAL_BASE_Z / 2.0);
    let rear_upright = centered_cube(
        "reagent_thaw_bubble_wetness_optical_rear_upright",
        OPTICAL_BASE_X,
        18.0,
        OPTICAL_UPRIGHT_Z,
    )
    .translate(
        0.0,
        OPTICAL_BASE_Y / 2.0 - 18.0,
        OPTICAL_BASE_Z + OPTICAL_UPRIGHT_Z / 2.0,
    );
    let front_upright = centered_cube(
        "reagent_thaw_bubble_wetness_optical_front_upright",
        OPTICAL_BASE_X,
        18.0,
        OPTICAL_UPRIGHT_Z,
    )
    .translate(
        0.0,
        -OPTICAL_BASE_Y / 2.0 + 18.0,
        OPTICAL_BASE_Z + OPTICAL_UPRIGHT_Z / 2.0,
    );

    base + (rear_upright - rear_window_cuts())
        + (front_upright - front_window_cuts())
        + optical_lane_troughs()
        + camera_illuminator_lands()
}

fn rear_window_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_rear_window_cuts");
    for window in 0..OPTICAL_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_rear_bubble_window_{window}_cut"),
                WINDOW_X,
                WINDOW_Y + 6.0,
                WINDOW_Z,
            )
            .translate(
                optical_x(window),
                OPTICAL_BASE_Y / 2.0 - 18.0,
                OPTICAL_BASE_Z + 74.0,
            );
    }
    cuts
}

fn front_window_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_front_window_cuts");
    for window in 0..OPTICAL_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_front_bubble_window_{window}_cut"),
                WINDOW_X,
                WINDOW_Y + 6.0,
                WINDOW_Z,
            )
            .translate(
                optical_x(window),
                -OPTICAL_BASE_Y / 2.0 + 18.0,
                OPTICAL_BASE_Z + 74.0,
            );
    }
    cuts
}

fn optical_lane_troughs() -> Part {
    let mut troughs = Part::empty("reagent_thaw_bubble_wetness_optical_lane_troughs");
    for lane in 0..OPTICAL_WINDOWS {
        troughs = troughs
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_bubble_ingress_lane_{lane}_tube_trough"),
                5.5,
                OPTICAL_BASE_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(optical_x(lane), 0.0, OPTICAL_BASE_Z + 7.0);
    }
    troughs
}

fn camera_illuminator_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_camera_illuminator_lands");
    for lane in 0..OPTICAL_WINDOWS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_lane_{lane}_camera_land"),
                54.0,
                20.0,
                5.0,
            )
            .translate(
                optical_x(lane),
                OPTICAL_BASE_Y / 2.0 + 20.0,
                OPTICAL_BASE_Z + 96.0,
            )
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_lane_{lane}_illuminator_land"),
                54.0,
                20.0,
                5.0,
            )
            .translate(
                optical_x(lane),
                -OPTICAL_BASE_Y / 2.0 - 20.0,
                OPTICAL_BASE_Z + 96.0,
            );
    }
    lands
}

fn check_valve_orientation_rail() -> Part {
    let rail = centered_cube(
        "reagent_thaw_bubble_wetness_check_valve_orientation_rail_body",
        VALVE_RAIL_X,
        VALVE_RAIL_Y,
        VALVE_RAIL_Z,
    )
    .translate(0.0, 0.0, VALVE_RAIL_Z / 2.0);
    let mut pocket_cuts = Part::empty("reagent_thaw_bubble_wetness_check_valve_pocket_cuts");
    let mut keys = Part::empty("reagent_thaw_bubble_wetness_check_valve_orientation_keys");
    for valve in 0..CHECK_VALVE_POSITIONS {
        let x = valve_x(valve);
        pocket_cuts = pocket_cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_body_pocket_cut"),
                VALVE_POCKET_X,
                VALVE_POCKET_Y,
                VALVE_POCKET_DEPTH + 1.0,
            )
            .translate(x, 0.0, VALVE_RAIL_Z - VALVE_POCKET_DEPTH / 2.0 + 0.5)
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_inlet_tube_bore"),
                5.0,
                VALVE_RAIL_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 24.0, 0.0, VALVE_RAIL_Z / 2.0)
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_outlet_tube_bore"),
                5.0,
                VALVE_RAIL_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 24.0, 0.0, VALVE_RAIL_Z / 2.0);

        keys = keys
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_flow_arrow_stem"),
                38.0,
                6.0,
                5.0,
            )
            .translate(x, -VALVE_RAIL_Y / 2.0 + 14.0, VALVE_RAIL_Z + 2.5)
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_orientation_key"),
                12.0,
                18.0,
                8.0,
            )
            .translate(x + 31.0, -VALVE_RAIL_Y / 2.0 + 14.0, VALVE_RAIL_Z + 4.0);
    }
    rail - pocket_cuts + keys + valve_lot_lands()
}

fn valve_lot_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_check_valve_lot_lands");
    for valve in 0..CHECK_VALVE_POSITIONS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_check_valve_{valve}_lot_land"),
                68.0,
                16.0,
                3.0,
            )
            .translate(
                valve_x(valve),
                VALVE_RAIL_Y / 2.0 - 14.0,
                VALVE_RAIL_Z + 1.5,
            );
    }
    lands
}

fn dripless_quick_connect_capture_tray() -> Part {
    let body = centered_cube(
        "reagent_thaw_bubble_wetness_quick_connect_capture_tray_body",
        QC_TRAY_X,
        QC_TRAY_Y,
        QC_TRAY_Z,
    )
    .translate(0.0, 0.0, QC_TRAY_Z / 2.0);
    let capture_recess = centered_cube(
        "reagent_thaw_bubble_wetness_quick_connect_absorbent_capture_pad_recess",
        QC_CAPTURE_PAD_X,
        QC_CAPTURE_PAD_Y,
        12.0,
    )
    .translate(0.0, -QC_TRAY_Y / 2.0 + 58.0, QC_TRAY_Z - 5.5);

    body - capture_recess - quick_connect_dock_cuts()
        + quick_connect_collars()
        + drip_witness_wells()
        + tray_splash_rims()
}

fn quick_connect_dock_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_quick_connect_dock_cuts");
    for dock in 0..QUICK_CONNECT_DOCKS {
        cuts = cuts
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_quick_connect_{dock}_dock_cut"),
                QC_DOCK_D / 2.0,
                QC_TRAY_Z + 4.0,
                36,
            )
            .translate(qc_x(dock), QC_TRAY_Y / 2.0 - 58.0, QC_TRAY_Z / 2.0);
    }
    cuts
}

fn quick_connect_collars() -> Part {
    let mut collars = Part::empty("reagent_thaw_bubble_wetness_quick_connect_collars");
    for dock in 0..QUICK_CONNECT_DOCKS {
        let x = qc_x(dock);
        let collar = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_quick_connect_{dock}_dripless_collar"),
            23.0,
            8.0,
            36,
        )
        .translate(x, QC_TRAY_Y / 2.0 - 58.0, QC_TRAY_Z + 4.0);
        let bore = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_quick_connect_{dock}_collar_bore"),
            QC_DOCK_D / 2.0,
            10.0,
            36,
        )
        .translate(x, QC_TRAY_Y / 2.0 - 58.0, QC_TRAY_Z + 4.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn drip_witness_wells() -> Part {
    let mut wells = Part::empty("reagent_thaw_bubble_wetness_quick_connect_drip_witness_wells");
    for dock in 0..QC_DRIP_WELLS {
        wells = wells
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_quick_connect_{dock}_drip_witness_well"),
                11.0,
                4.0,
                28,
            )
            .translate(qc_x(dock), -QC_TRAY_Y / 2.0 + 44.0, QC_TRAY_Z + 2.0);
    }
    wells
}

fn tray_splash_rims() -> Part {
    let front = centered_cube(
        "reagent_thaw_bubble_wetness_quick_connect_front_splash_rim",
        QC_TRAY_X,
        10.0,
        26.0,
    )
    .translate(0.0, -QC_TRAY_Y / 2.0 + 5.0, QC_TRAY_Z + 13.0);
    let left = centered_cube(
        "reagent_thaw_bubble_wetness_quick_connect_left_splash_rim",
        10.0,
        QC_TRAY_Y,
        26.0,
    )
    .translate(-QC_TRAY_X / 2.0 + 5.0, 0.0, QC_TRAY_Z + 13.0);
    let right = centered_cube(
        "reagent_thaw_bubble_wetness_quick_connect_right_splash_rim",
        10.0,
        QC_TRAY_Y,
        26.0,
    )
    .translate(QC_TRAY_X / 2.0 - 5.0, 0.0, QC_TRAY_Z + 13.0);
    front + left + right
}

fn temperature_time_token_rail() -> Part {
    let rail = centered_cube(
        "reagent_thaw_bubble_wetness_temperature_time_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(0.0, 0.0, TOKEN_RAIL_Z / 2.0);
    rail - token_slot_cuts() + token_time_index_ticks() + token_label_lands()
}

fn token_slot_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_temperature_time_token_slot_cuts");
    for slot in 0..TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_temperature_time_token_slot_{slot}_cut"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                16.0,
            )
            .translate(
                centered_index(slot, TOKEN_SLOTS, 55.0),
                0.0,
                TOKEN_RAIL_Z - 7.5,
            );
    }
    cuts
}

fn token_time_index_ticks() -> Part {
    let mut ticks = Part::empty("reagent_thaw_bubble_wetness_time_index_ticks");
    for tick in 0..TIME_INDEX_TICKS {
        ticks = ticks
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_time_index_tick_{tick}"),
                3.0,
                20.0,
                7.0,
            )
            .translate(
                centered_index(tick, TIME_INDEX_TICKS, 55.0),
                TOKEN_RAIL_Y / 2.0 - 11.0,
                TOKEN_RAIL_Z + 3.5,
            );
    }
    ticks
}

fn token_label_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_token_label_lands");
    for slot in 0..TOKEN_SLOTS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_temperature_time_token_{slot}_label_land"),
                42.0,
                16.0,
                3.0,
            )
            .translate(
                centered_index(slot, TOKEN_SLOTS, 55.0),
                -TOKEN_RAIL_Y / 2.0 + 12.0,
                TOKEN_RAIL_Z + 1.5,
            );
    }
    lands
}

fn pressure_hold_witness_ports() -> Part {
    let panel = centered_cube(
        "reagent_thaw_bubble_wetness_pressure_hold_witness_panel",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(0.0, 0.0, PRESSURE_PANEL_Z / 2.0);
    panel - pressure_port_cuts() + pressure_port_collars() + reference_blind_ports()
}

fn pressure_port_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_pressure_port_cuts");
    for port in 0..PRESSURE_WITNESS_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_pressure_hold_witness_port_{port}_cut"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_PANEL_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(pressure_x(port), 0.0, PRESSURE_PANEL_Z * 0.62);
    }
    cuts
}

fn pressure_port_collars() -> Part {
    let mut collars = Part::empty("reagent_thaw_bubble_wetness_pressure_port_collars");
    for port in 0..PRESSURE_SEAL_LANDS {
        let collar = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_pressure_hold_port_{port}_seal_land"),
            18.0,
            7.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            pressure_x(port),
            -PRESSURE_PANEL_Y / 2.0 - 3.5,
            PRESSURE_PANEL_Z * 0.62,
        );
        let bore = centered_cylinder(
            format!("reagent_thaw_bubble_wetness_pressure_hold_port_{port}_seal_land_bore"),
            PRESSURE_PORT_D / 2.0,
            9.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            pressure_x(port),
            -PRESSURE_PANEL_Y / 2.0 - 3.5,
            PRESSURE_PANEL_Z * 0.62,
        );
        collars = collars + (collar - bore);
    }
    collars
}

fn reference_blind_ports() -> Part {
    let mut ports = Part::empty("reagent_thaw_bubble_wetness_reference_blind_ports");
    for port in 0..REFERENCE_BLIND_PORTS {
        ports = ports
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_reference_blind_port_{port}"),
                15.0,
                8.0,
                32,
            )
            .translate(
                centered_index(port, REFERENCE_BLIND_PORTS, 62.0),
                PRESSURE_PANEL_Y / 2.0 - 34.0,
                PRESSURE_PANEL_Z + 4.0,
            )
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_reference_blind_port_{port}_label_land"),
                48.0,
                16.0,
                3.0,
            )
            .translate(
                centered_index(port, REFERENCE_BLIND_PORTS, 62.0),
                PRESSURE_PANEL_Y / 2.0 - 62.0,
                PRESSURE_PANEL_Z + 1.5,
            );
    }
    ports
}

fn barcode_lot_custody_plate() -> Part {
    let plate = centered_cube(
        "reagent_thaw_bubble_wetness_barcode_lot_custody_plate_body",
        CUSTODY_PLATE_X,
        CUSTODY_PLATE_Y,
        CUSTODY_PLATE_Z,
    )
    .translate(0.0, 0.0, CUSTODY_PLATE_Z / 2.0);
    plate + barcode_lands() + lot_card_lands() + custody_token_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_barcode_lands");
    for land in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_barcode_land_{land}"),
                62.0,
                24.0,
                3.0,
            )
            .translate(
                centered_index(land, BARCODE_LANDS, 72.0),
                CUSTODY_PLATE_Y / 2.0 - 24.0,
                CUSTODY_PLATE_Z + 1.5,
            );
    }
    lands
}

fn lot_card_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_lot_card_lands");
    for land in 0..LOT_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_lot_card_land_{land}"),
                86.0,
                24.0,
                3.0,
            )
            .translate(
                centered_index(land, LOT_CARD_LANDS, 96.0),
                -2.0,
                CUSTODY_PLATE_Z + 1.5,
            );
    }
    lands
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty("reagent_thaw_bubble_wetness_custody_token_wells");
    for well in 0..CHAIN_OF_CUSTODY_TOKEN_WELLS {
        wells = wells
            + centered_cylinder(
                format!("reagent_thaw_bubble_wetness_chain_of_custody_token_well_{well}"),
                13.0,
                5.0,
                28,
            )
            .translate(
                centered_index(well, CHAIN_OF_CUSTODY_TOKEN_WELLS, 56.0),
                -CUSTODY_PLATE_Y / 2.0 + 20.0,
                CUSTODY_PLATE_Z + 2.5,
            );
    }
    wells
}

fn release_hold_reject_lanes() -> Part {
    let block = centered_cube(
        "reagent_thaw_bubble_wetness_release_hold_reject_lane_block",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(0.0, 0.0, DISPOSITION_Z / 2.0);
    block - disposition_slot_cuts() + disposition_dividers() + disposition_lane_lands()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty("reagent_thaw_bubble_wetness_disposition_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "reagent_thaw_bubble_wetness_{}_lane_slot_{slot}_cut",
                        STATUS_LANE_NAMES[lane]
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    16.0,
                )
                .translate(
                    centered_index(slot, STATUS_SLOTS_PER_LANE, 72.0),
                    disposition_lane_y(lane),
                    DISPOSITION_Z - 7.5,
                );
        }
    }
    cuts
}

fn disposition_dividers() -> Part {
    let mut dividers = Part::empty("reagent_thaw_bubble_wetness_disposition_dividers");
    for lane in 0..STATUS_LANES - 1 {
        dividers = dividers
            + centered_cube(
                format!("reagent_thaw_bubble_wetness_disposition_divider_{lane}"),
                DISPOSITION_X - 30.0,
                5.0,
                18.0,
            )
            .translate(
                0.0,
                (disposition_lane_y(lane) + disposition_lane_y(lane + 1)) / 2.0,
                DISPOSITION_Z + 9.0,
            );
    }
    dividers
}

fn disposition_lane_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_bubble_wetness_disposition_lane_lands");
    for lane in 0..STATUS_LANES {
        lands = lands
            + centered_cube(
                format!(
                    "reagent_thaw_bubble_wetness_{}_lane_label_land",
                    STATUS_LANE_NAMES[lane]
                ),
                78.0,
                18.0,
                3.0,
            )
            .translate(
                -DISPOSITION_X / 2.0 + 54.0,
                disposition_lane_y(lane),
                DISPOSITION_Z + 1.5,
            );
    }
    lands
}

fn evidence_bridge_robot_service_keepouts() -> Part {
    evidence_bridge() + robot_keepout_gauge() + service_keepout_gauges()
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "reagent_thaw_bubble_wetness_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "reagent_thaw_bubble_wetness_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "reagent_thaw_bubble_wetness_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let camera_mount = centered_cube(
        "reagent_thaw_bubble_wetness_evidence_bridge_overhead_camera_mount",
        120.0,
        34.0,
        12.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1, BASE_Z + BRIDGE_POST_Z - 18.0);
    left_post + right_post + beam + camera_mount
}

fn robot_keepout_gauge() -> Part {
    wireframe_box(
        "reagent_thaw_bubble_wetness_robot_sweep_keepout",
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        ROBOT_SWEEP_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -10.0, BASE_Z + ROBOT_SWEEP_Z / 2.0)
}

fn service_keepout_gauges() -> Part {
    let front = centered_cube(
        "reagent_thaw_bubble_wetness_front_robot_approach_keepout_gauge",
        620.0,
        KEEP_OUT_RAIL,
        38.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - ROBOT_FRONT_APPROACH / 2.0,
        BASE_Z + 19.0,
    );
    let rear = centered_cube(
        "reagent_thaw_bubble_wetness_rear_connector_service_keepout_gauge",
        620.0,
        KEEP_OUT_RAIL,
        38.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_CONNECTOR_SERVICE / 2.0,
        BASE_Z + 19.0,
    );
    let right = centered_cube(
        "reagent_thaw_bubble_wetness_right_pressure_service_keepout_gauge",
        KEEP_OUT_RAIL,
        330.0,
        38.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_PRESSURE_SERVICE / 2.0,
        0.0,
        BASE_Z + 19.0,
    );
    let left = centered_cube(
        "reagent_thaw_bubble_wetness_left_custody_service_keepout_gauge",
        KEEP_OUT_RAIL,
        330.0,
        38.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CUSTODY_SERVICE / 2.0,
        -70.0,
        BASE_Z + 19.0,
    );
    let top = centered_cube(
        "reagent_thaw_bubble_wetness_top_optical_service_keepout_gauge",
        420.0,
        8.0,
        24.0,
    )
    .translate(OPTICAL_POS.0, OPTICAL_POS.1, BASE_Z + TOP_OPTICAL_SERVICE);
    front + rear + right + left + top
}

fn wireframe_box(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut part = Part::empty(name);
    for (zi, dz) in [-1.0, 1.0].iter().copied().enumerate() {
        for (yi, dy) in [-1.0, 1.0].iter().copied().enumerate() {
            part = part
                + centered_cube(format!("{name}_x_rail_{zi}_{yi}"), x, rail, rail).translate(
                    0.0,
                    dy * y / 2.0,
                    dz * z / 2.0,
                );
        }
        for (xi, dx) in [-1.0, 1.0].iter().copied().enumerate() {
            part = part
                + centered_cube(format!("{name}_y_rail_{zi}_{xi}"), rail, y, rail).translate(
                    dx * x / 2.0,
                    0.0,
                    dz * z / 2.0,
                );
        }
    }
    part
}

fn module_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "thawed_bag_vial_nest",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Footprint {
            name: "connector_wetness_witness",
            center: WETNESS_POS,
            x: WETNESS_BLOCK_X,
            y: WETNESS_BLOCK_Y,
        },
        Footprint {
            name: "bubble_ingress_optics",
            center: OPTICAL_POS,
            x: OPTICAL_BASE_X,
            y: OPTICAL_BASE_Y,
        },
        Footprint {
            name: "check_valve_orientation_rail",
            center: VALVE_POS,
            x: VALVE_RAIL_X,
            y: VALVE_RAIL_Y,
        },
        Footprint {
            name: "quick_connect_capture",
            center: QC_POS,
            x: QC_TRAY_X,
            y: QC_TRAY_Y,
        },
        Footprint {
            name: "temperature_time_tokens",
            center: TOKEN_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Footprint {
            name: "pressure_hold_ports",
            center: PRESSURE_POS,
            x: PRESSURE_PANEL_X,
            y: PRESSURE_PANEL_Y,
        },
        Footprint {
            name: "barcode_lot_custody",
            center: CUSTODY_POS,
            x: CUSTODY_PLATE_X,
            y: CUSTODY_PLATE_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn fits_inside_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - CURB_W - 24.0;
    let usable_y = STATION_Y / 2.0 - CURB_W - 24.0;
    footprint.center.0 - footprint.x / 2.0 >= -usable_x
        && footprint.center.0 + footprint.x / 2.0 <= usable_x
        && footprint.center.1 - footprint.y / 2.0 >= -usable_y
        && footprint.center.1 + footprint.y / 2.0 <= usable_y
}

fn optical_x(index: usize) -> f64 {
    centered_index(index, OPTICAL_WINDOWS, 78.0)
}

fn valve_x(index: usize) -> f64 {
    centered_index(index, CHECK_VALVE_POSITIONS, 96.0)
}

fn qc_x(index: usize) -> f64 {
    centered_index(index, QUICK_CONNECT_DOCKS, QC_DOCK_PITCH_X)
}

fn pressure_x(index: usize) -> f64 {
    centered_index(index, PRESSURE_WITNESS_PORTS, PRESSURE_PORT_PITCH_X)
}

fn disposition_lane_y(index: usize) -> f64 {
    centered_index(index, STATUS_LANES, 34.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(DATUM_TARGETS, 5);
    assert_eq!(WETNESS_WICK_WELLS, WETNESS_SLOTS);
    assert_eq!(CAMERA_LANDS, OPTICAL_WINDOWS);
    assert_eq!(ILLUMINATOR_LANDS, OPTICAL_WINDOWS);
    assert_eq!(ORIENTATION_KEYS, CHECK_VALVE_POSITIONS);
    assert_eq!(QC_DRIP_WELLS, QUICK_CONNECT_DOCKS);
    assert_eq!(PRESSURE_SEAL_LANDS, PRESSURE_WITNESS_PORTS);
    assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= WETNESS_SLOTS);
    assert!(BRIDGE_POST_Z > EVIDENCE_CAMERA_CLEARANCE_Z);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.ends_with(".stl"), "{path}");
    }

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            fits_inside_station(footprint),
            "{} exceeds deck envelope",
            footprint.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        assert_layout();
        assert!(OUTPUTS[0].contains("containment_deck"));
        assert!(OUTPUTS[1].contains("thawed_bag_vial_nest"));
        assert!(OUTPUTS[2].contains("connector_wetness_witness_slots"));
        assert!(OUTPUTS[3].contains("bubble_ingress_optical_windows"));
        assert!(OUTPUTS[10].contains("evidence_bridge_robot_service_keepouts"));
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for feature in [
            "thawed_bag_vial_nest",
            "connector_wetness_witness_slots",
            "bubble_ingress_optical_windows",
            "check_valve_orientation_rail",
            "dripless_quick_connect_capture_tray",
            "temperature_time_token_rail",
            "pressure_hold_witness_ports",
            "barcode_lot_custody_plate",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn station_modules_fit_without_footprint_overlap() {
        for footprint in module_footprints() {
            assert!(fits_inside_station(footprint));
        }
        assert!(VALVE_POS.1 > OPTICAL_POS.1);
        assert!(CUSTODY_POS.0 < TOKEN_POS.0);
        assert!(DISPOSITION_POS.0 > TOKEN_POS.0);
        assert!(QC_POS.0 > OPTICAL_POS.0);
    }

    #[test]
    fn witness_and_optical_lane_counts_are_explicit() {
        assert_eq!(BAG_BAYS, 2);
        assert_eq!(VIAL_WELLS, 8);
        assert_eq!(WETNESS_SLOTS, 12);
        assert_eq!(WETNESS_WICK_WELLS, WETNESS_SLOTS);
        assert_eq!(OPTICAL_WINDOWS, 6);
        assert_eq!(CAMERA_LANDS, OPTICAL_WINDOWS);
        assert_eq!(ILLUMINATOR_LANDS, OPTICAL_WINDOWS);
    }

    #[test]
    fn closed_connector_pressure_and_disposition_capacity_are_sane() {
        assert_eq!(CHECK_VALVE_POSITIONS, QUICK_CONNECT_DOCKS);
        assert_eq!(ORIENTATION_KEYS, CHECK_VALVE_POSITIONS);
        assert_eq!(PRESSURE_WITNESS_PORTS, QUICK_CONNECT_DOCKS);
        assert_eq!(PRESSURE_SEAL_LANDS, PRESSURE_WITNESS_PORTS);
        assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= WETNESS_SLOTS);
    }

    #[test]
    fn custody_token_and_evidence_geometry_are_declared() {
        assert_eq!(TOKEN_SLOTS, 10);
        assert_eq!(TIME_INDEX_TICKS, TOKEN_SLOTS + 1);
        assert_eq!(
            BARCODE_LANDS + LOT_CARD_LANDS + CHAIN_OF_CUSTODY_TOKEN_WELLS,
            11
        );
        assert!(BRIDGE_SPAN_X > OPTICAL_BASE_X);
        assert!(BRIDGE_POST_Z > EVIDENCE_CAMERA_CLEARANCE_Z);
        assert!(ROBOT_SWEEP_X < STATION_X);
        assert!(ROBOT_SWEEP_Y < STATION_Y);
    }

    #[test]
    fn limitation_markers_prevent_protocol_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"no_biological_protocol"));
        assert!(LIMITATIONS.contains(&"no_acceptance_thresholds"));
        assert!(LIMITATIONS.contains(&"not_pressure_rated_hardware"));
        assert!(LIMITATIONS.contains(&"purchased_wetted_components_external"));
    }
}
