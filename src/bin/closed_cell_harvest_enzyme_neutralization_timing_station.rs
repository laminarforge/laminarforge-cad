use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-culture harvest enzyme neutralization timing station.
//
// Intent:
// - Validate no-cell enzyme exposure, quench, media recovery, wash, and residue
//   timing before harvested cells are ever used for chip seeding.
// - Make timing state, neutralization evidence, source-vessel identity, waste
//   segregation, and robot/service clearances physical and auditable.
//
// This is mechanical validation-fixture CAD only. It is not a biological
// passaging protocol, cell-use release method, or enzyme neutralization claim.

const PREFIX: &str = "closed_cell_harvest_enzyme_neutralization_timing_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_secondary_containment_deck.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_source_vessel_surrogate_dock.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_enzyme_quench_media_input_ports.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_timing_token_rail.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_temperature_probe_wells.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_wash_recovery_collection_nests.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_residue_neutralization_witness_wells.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_bubble_dead_volume_windows.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_barcode_status_lands.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_waste_segregation_manifold.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_evidence_bridge.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_robot_service_keepouts.stl",
    "output/closed_cell_harvest_enzyme_neutralization_timing_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "source_vessel_surrogate_dock",
    "enzyme_quench_media_input_ports",
    "timing_token_rail",
    "temperature_probe_wells",
    "wash_recovery_collection_nests",
    "residue_neutralization_witness_wells",
    "bubble_dead_volume_windows",
    "barcode_status_lands",
    "waste_segregation_manifold",
    "evidence_bridge",
    "robot_service_keepouts",
    "assembly_export",
];

const TIMING_STATES: [&str; 7] = [
    "pre_wash",
    "enzyme_start",
    "enzyme_limit",
    "quench_start",
    "neutralized",
    "media_recovery",
    "seed_ready_hold",
];
const INPUT_PORTS: [&str; 5] = ["wash", "enzyme", "quench", "media", "flush"];
const STATUS_LANES: [&str; 4] = ["ready", "timing", "hold", "reject"];
const WASTE_STREAMS: [&str; 4] = ["enzyme", "quench", "wash", "mixed"];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const MOUNT_HOLE_D: f64 = 6.8;

const TRAY_RECESS_X: f64 = 1110.0;
const TRAY_RECESS_Y: f64 = 660.0;
const TRAY_RECESS_Z: f64 = 9.0;
const FLOW_RIBS: usize = 6;
const LEAK_WITNESS_PADS: usize = 6;

const DOCK_CENTER: (f64, f64) = (-355.0, 135.0);
const DOCK_X: f64 = 430.0;
const DOCK_Y: f64 = 285.0;
const DOCK_Z: f64 = 74.0;
const VESSEL_SURROGATE_D: f64 = 176.0;
const DOCK_DATUM_PINS: usize = 4;
const DOCK_LATCHES: usize = 4;

const PORT_CENTER: (f64, f64) = (315.0, 218.0);
const PORT_PANEL_X: f64 = 410.0;
const PORT_PANEL_Y: f64 = 176.0;
const PORT_PANEL_Z: f64 = 58.0;
const PORT_PITCH_X: f64 = 72.0;
const PORT_BOSS_D: f64 = 36.0;
const PORT_CLEAR_D: f64 = 14.0;
const STERILE_CAP_POCKETS: usize = 5;

const TOKEN_CENTER: (f64, f64) = (42.0, 22.0);
const TOKEN_RAIL_X: f64 = 770.0;
const TOKEN_RAIL_Y: f64 = 116.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const TOKEN_SLOT_X: f64 = 82.0;
const TOKEN_SLOT_Y: f64 = 52.0;
const TOKEN_SLOT_Z: f64 = 12.0;
const TOKEN_PITCH_X: f64 = 104.0;

const PROBE_CENTER: (f64, f64) = (-355.0, -112.0);
const PROBE_BLOCK_X: f64 = 420.0;
const PROBE_BLOCK_Y: f64 = 126.0;
const PROBE_BLOCK_Z: f64 = 42.0;
const PROBE_WELLS: usize = 6;
const PROBE_WELL_D: f64 = 18.0;
const PROBE_WELL_PITCH_X: f64 = 58.0;

const COLLECTION_CENTER: (f64, f64) = (235.0, -134.0);
const COLLECTION_BLOCK_X: f64 = 470.0;
const COLLECTION_BLOCK_Y: f64 = 164.0;
const COLLECTION_BLOCK_Z: f64 = 44.0;
const COLLECTION_NESTS: usize = 8;
const COLLECTION_NEST_D: f64 = 30.0;
const COLLECTION_PITCH_X: f64 = 52.0;

const WITNESS_CENTER: (f64, f64) = (-355.0, -292.0);
const WITNESS_BLOCK_X: f64 = 420.0;
const WITNESS_BLOCK_Y: f64 = 118.0;
const WITNESS_BLOCK_Z: f64 = 34.0;
const RESIDUE_WELLS: usize = 8;
const RESIDUE_WELL_D: f64 = 22.0;
const RESIDUE_PITCH_X: f64 = 48.0;
const PH_TILE_COUNT: usize = 6;

const WINDOW_CENTER: (f64, f64) = (235.0, -292.0);
const WINDOW_PLATE_X: f64 = 470.0;
const WINDOW_PLATE_Y: f64 = 118.0;
const WINDOW_PLATE_Z: f64 = 22.0;
const OBSERVATION_WINDOWS: usize = 6;
const WINDOW_SLOT_X: f64 = 52.0;
const WINDOW_SLOT_Y: f64 = 76.0;
const WINDOW_PITCH_X: f64 = 68.0;

const TRACE_CENTER: (f64, f64) = (-20.0, 332.0);
const TRACE_PANEL_X: f64 = 720.0;
const TRACE_PANEL_Y: f64 = 94.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 12;
const STATUS_TOKENS_PER_LANE: usize = 3;

const WASTE_CENTER: (f64, f64) = (530.0, -60.0);
const WASTE_PANEL_X: f64 = 142.0;
const WASTE_PANEL_Y: f64 = 520.0;
const WASTE_PANEL_Z: f64 = 50.0;
const WASTE_PORT_D: f64 = 28.0;
const WASTE_BAG_NESTS: usize = 4;

const BRIDGE_CENTER: (f64, f64) = (0.0, -12.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 40.0;
const BRIDGE_UNDERSIDE_Z: f64 = 235.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_WINDOWS: usize = 5;

const KEEP_OUT_RAIL_W: f64 = 12.0;
const KEEP_OUT_Z: f64 = 88.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 390.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 235.0;
const LEFT_VESSEL_SERVICE_X: f64 = 230.0;
const RIGHT_WASTE_SERVICE_X: f64 = 205.0;
const OVERHEAD_EVIDENCE_CLEARANCE_Z: f64 = 320.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(&deck, OUTPUTS[0]);

    let dock = source_vessel_surrogate_dock();
    export(&dock, OUTPUTS[1]);

    let ports = enzyme_quench_media_input_ports();
    export(&ports, OUTPUTS[2]);

    let tokens = timing_token_rail();
    export(&tokens, OUTPUTS[3]);

    let probes = temperature_probe_wells();
    export(&probes, OUTPUTS[4]);

    let collection = wash_recovery_collection_nests();
    export(&collection, OUTPUTS[5]);

    let witnesses = residue_neutralization_witness_wells();
    export(&witnesses, OUTPUTS[6]);

    let windows = bubble_dead_volume_windows();
    export(&windows, OUTPUTS[7]);

    let trace = barcode_status_lands();
    export(&trace, OUTPUTS[8]);

    let waste = waste_segregation_manifold();
    export(&waste, OUTPUTS[9]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = deck
        + dock.translate(DOCK_CENTER.0, DOCK_CENTER.1, deck_insert_z(DOCK_Z))
        + ports.translate(PORT_CENTER.0, PORT_CENTER.1, deck_insert_z(PORT_PANEL_Z))
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_insert_z(TOKEN_RAIL_Z))
        + probes.translate(PROBE_CENTER.0, PROBE_CENTER.1, deck_insert_z(PROBE_BLOCK_Z))
        + collection.translate(
            COLLECTION_CENTER.0,
            COLLECTION_CENTER.1,
            deck_insert_z(COLLECTION_BLOCK_Z),
        )
        + witnesses.translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            deck_insert_z(WITNESS_BLOCK_Z),
        )
        + windows.translate(
            WINDOW_CENTER.0,
            WINDOW_CENTER.1,
            deck_insert_z(WINDOW_PLATE_Z),
        )
        + trace.translate(TRACE_CENTER.0, TRACE_CENTER.1, deck_insert_z(TRACE_PANEL_Z))
        + waste.translate(WASTE_CENTER.0, WASTE_CENTER.1, deck_insert_z(WASTE_PANEL_Z))
        + bridge.translate(
            BRIDGE_CENTER.0,
            BRIDGE_CENTER.1,
            deck_insert_z(BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z),
        )
        + keepouts.translate(0.0, 0.0, deck_insert_z(KEEP_OUT_Z));
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed cell-culture harvest enzyme neutralization timing station:");
    println!(
        "  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm deck with secondary containment, {FLOW_RIBS} flow ribs, and {LEAK_WITNESS_PADS} leak witness pads"
    );
    println!(
        "  Source surrogate:            {DOCK_X:.0}mm x {DOCK_Y:.0}mm dock, {VESSEL_SURROGATE_D:.0}mm vessel saddle, {DOCK_LATCHES} latch lands, {DOCK_DATUM_PINS} datum pins"
    );
    println!(
        "  Closed fluid inputs:         {} keyed ports for wash/enzyme/quench/media/flush with {STERILE_CAP_POCKETS} cap pockets",
        INPUT_PORTS.len()
    );
    println!(
        "  Timing evidence:             {} token states, {PROBE_WELLS} temperature probe wells, {RESIDUE_WELLS} residue/neutralization wells, {PH_TILE_COUNT} pH reference tiles",
        TIMING_STATES.len()
    );
    println!(
        "  Recovery and observation:    {COLLECTION_NESTS} wash/recovery nests, {OBSERVATION_WINDOWS} bubble/dead-volume windows, {WASTE_BAG_NESTS} segregated waste stream nests"
    );
    println!(
        "  Traceability:                {BARCODE_LANDS} barcode lands, {} status token pockets, evidence bridge with {CAMERA_WINDOWS} camera windows",
        STATUS_LANES.len() * STATUS_TOKENS_PER_LANE
    );
    println!(
        "  Robot/service envelopes:     front robot {FRONT_ROBOT_CLEARANCE_Y:.0}mm, rear service {REAR_SERVICE_CLEARANCE_Y:.0}mm, vessel side {LEFT_VESSEL_SERVICE_X:.0}mm, waste side {RIGHT_WASTE_SERVICE_X:.0}mm, overhead {OVERHEAD_EVIDENCE_CLEARANCE_Z:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(INPUT_PORTS.len(), STERILE_CAP_POCKETS);
    assert_eq!(WASTE_STREAMS.len(), WASTE_BAG_NESTS);
    assert!(TIMING_STATES.len() >= 6);
    assert!(TOKEN_RAIL_X > TIMING_STATES.len() as f64 * TOKEN_SLOT_X);
    assert!(COLLECTION_NESTS >= RESIDUE_WELLS);
    assert!(BARCODE_LANDS >= INPUT_PORTS.len() + STATUS_LANES.len());
    assert!(left_edge(DOCK_CENTER.0, DOCK_X) > -STATION_X / 2.0 + 36.0);
    assert!(right_edge(WASTE_CENTER.0, WASTE_PANEL_X) < STATION_X / 2.0 - 18.0);
    assert!(top_edge(TRACE_CENTER.1, TRACE_PANEL_Y) < STATION_Y / 2.0 - 22.0);
    assert!(bottom_edge(WITNESS_CENTER.1, WITNESS_BLOCK_Y) > -STATION_Y / 2.0 + 24.0);
    assert!(right_edge(TRACE_CENTER.0, TRACE_PANEL_X) < STATION_X / 2.0 - 50.0);
    assert!(BRIDGE_UNDERSIDE_Z > DOCK_Z + 100.0);
    assert!(OVERHEAD_EVIDENCE_CLEARANCE_Z > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z + 8.0 + height / 2.0
}

fn left_edge(center: f64, width: f64) -> f64 {
    center - width / 2.0
}

fn right_edge(center: f64, width: f64) -> f64 {
    center + width / 2.0
}

fn bottom_edge(center: f64, depth: f64) -> f64 {
    center - depth / 2.0
}

fn top_edge(center: f64, depth: f64) -> f64 {
    center + depth / 2.0
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recess = centered_cube(
        format!("{PREFIX}_tray_recess"),
        TRAY_RECESS_X,
        TRAY_RECESS_Y,
        TRAY_RECESS_Z,
    )
    .translate(0.0, -8.0, BASE_Z - TRAY_RECESS_Z / 2.0);
    let drain = centered_cylinder(format!("{PREFIX}_front_drain_interface"), 10.0, 70.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -STATION_Y / 2.0 + 26.0, BASE_Z - 6.0);

    deck - recess - drain - mounting_holes()
        + containment_rim()
        + flow_ribs()
        + leak_witness_pads()
        + zone_datum_lands()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_holes"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 50.0, -STATION_Y / 2.0 + 50.0),
        (STATION_X / 2.0 - 50.0, -STATION_Y / 2.0 + 50.0),
        (-STATION_X / 2.0 + 50.0, STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 50.0, STATION_Y / 2.0 - 50.0),
        (0.0, -STATION_Y / 2.0 + 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_tray_flow_ribs"));
    for i in 0..FLOW_RIBS {
        let y = -270.0 + i as f64 * 108.0;
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_enzymatic_residue_flow_rib_{i}"),
                TRAY_RECESS_X - 140.0,
                8.0,
                7.0,
            )
            .translate(0.0, y, BASE_Z + 3.5);
    }
    ribs
}

fn leak_witness_pads() -> Part {
    let mut pads = Part::empty(format!("{PREFIX}_leak_witness_pads"));
    for i in 0..LEAK_WITNESS_PADS {
        let x = -420.0 + (i % 3) as f64 * 420.0;
        let y = -315.0 + (i / 3) as f64 * 630.0;
        pads =
            pads + centered_cube(format!("{PREFIX}_leak_witness_pad_{i}"), 62.0, 34.0, 6.0)
                .translate(x, y, BASE_Z + 3.0)
                + centered_cube(format!("{PREFIX}_leak_trace_groove_{i}"), 92.0, 6.0, 5.0)
                    .translate(x, y + 28.0, BASE_Z + 2.5);
    }
    pads
}

fn zone_datum_lands() -> Part {
    centered_cube(
        format!("{PREFIX}_dock_datum_land"),
        DOCK_X + 52.0,
        DOCK_Y + 44.0,
        6.0,
    )
    .translate(DOCK_CENTER.0, DOCK_CENTER.1, BASE_Z + 3.0)
        + centered_cube(
            format!("{PREFIX}_port_panel_datum_land"),
            PORT_PANEL_X + 42.0,
            PORT_PANEL_Y + 36.0,
            6.0,
        )
        .translate(PORT_CENTER.0, PORT_CENTER.1, BASE_Z + 3.0)
        + centered_cube(
            format!("{PREFIX}_collection_datum_land"),
            COLLECTION_BLOCK_X + 36.0,
            COLLECTION_BLOCK_Y + 34.0,
            6.0,
        )
        .translate(COLLECTION_CENTER.0, COLLECTION_CENTER.1, BASE_Z + 3.0)
        + centered_cube(
            format!("{PREFIX}_waste_datum_land"),
            WASTE_PANEL_X + 28.0,
            WASTE_PANEL_Y + 32.0,
            6.0,
        )
        .translate(WASTE_CENTER.0, WASTE_CENTER.1, BASE_Z + 3.0)
}

fn source_vessel_surrogate_dock() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_source_vessel_surrogate_dock_base"),
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let saddle = centered_cylinder(
        format!("{PREFIX}_source_vessel_saddle_cut"),
        VESSEL_SURROGATE_D / 2.0,
        DOCK_Y + 24.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 38.0);
    base - saddle - front_service_window()
        + dock_side_rails()
        + dock_latches()
        + dock_datum_pins()
        + vessel_identity_lands()
}

fn front_service_window() -> Part {
    centered_cube(
        format!("{PREFIX}_source_dock_front_service_window"),
        DOCK_X - 88.0,
        34.0,
        DOCK_Z - 22.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 + 17.0, 2.0)
}

fn dock_side_rails() -> Part {
    centered_cube(
        format!("{PREFIX}_source_dock_left_retention_rail"),
        24.0,
        DOCK_Y + 34.0,
        48.0,
    )
    .translate(-DOCK_X / 2.0 + 26.0, 0.0, DOCK_Z / 2.0 + 12.0)
        + centered_cube(
            format!("{PREFIX}_source_dock_right_retention_rail"),
            24.0,
            DOCK_Y + 34.0,
            48.0,
        )
        .translate(DOCK_X / 2.0 - 26.0, 0.0, DOCK_Z / 2.0 + 12.0)
        + centered_cube(
            format!("{PREFIX}_source_dock_rear_stop"),
            DOCK_X - 80.0,
            22.0,
            58.0,
        )
        .translate(0.0, DOCK_Y / 2.0 - 12.0, DOCK_Z / 2.0 + 15.0)
}

fn dock_latches() -> Part {
    let mut latches = Part::empty(format!("{PREFIX}_source_vessel_latches"));
    for i in 0..DOCK_LATCHES {
        let x = if i % 2 == 0 { -172.0 } else { 172.0 };
        let y = if i < 2 { -112.0 } else { 112.0 };
        latches = latches
            + centered_cube(
                format!("{PREFIX}_source_vessel_latch_land_{i}"),
                50.0,
                24.0,
                12.0,
            )
            .translate(x, y, DOCK_Z / 2.0 + 12.0)
            + centered_cylinder(
                format!("{PREFIX}_source_vessel_latch_pin_{i}"),
                6.0,
                18.0,
                24,
            )
            .translate(x, y, DOCK_Z / 2.0 + 24.0);
    }
    latches
}

fn dock_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{PREFIX}_source_vessel_datum_pins"));
    for i in 0..DOCK_DATUM_PINS {
        let x = if i % 2 == 0 { -160.0 } else { 160.0 };
        let y = if i < 2 { -96.0 } else { 96.0 };
        pins = pins
            + centered_cylinder(
                format!("{PREFIX}_source_vessel_datum_pin_{i}"),
                7.0,
                20.0,
                28,
            )
            .translate(x, y, DOCK_Z / 2.0 + 10.0);
    }
    pins
}

fn vessel_identity_lands() -> Part {
    centered_cube(
        format!("{PREFIX}_source_lot_barcode_land"),
        116.0,
        28.0,
        5.0,
    )
    .translate(-128.0, -DOCK_Y / 2.0 - 18.0, 6.0)
        + centered_cube(format!("{PREFIX}_source_rfid_status_land"), 82.0, 36.0, 5.0).translate(
            96.0,
            -DOCK_Y / 2.0 - 18.0,
            6.0,
        )
}

fn enzyme_quench_media_input_ports() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_input_port_panel"),
        PORT_PANEL_X,
        PORT_PANEL_Y,
        PORT_PANEL_Z,
    );
    let mut bosses = Part::empty(format!("{PREFIX}_input_port_bosses"));
    let mut clears = Part::empty(format!("{PREFIX}_input_port_clearances"));
    let mut caps = Part::empty(format!("{PREFIX}_sterile_cap_pockets"));
    let start_x = -((INPUT_PORTS.len() - 1) as f64) * PORT_PITCH_X / 2.0;
    for (i, name) in INPUT_PORTS.iter().enumerate() {
        let x = start_x + i as f64 * PORT_PITCH_X;
        bosses = bosses
            + centered_cylinder(
                format!("{PREFIX}_{name}_keyed_port_boss"),
                PORT_BOSS_D / 2.0,
                18.0,
                36,
            )
            .translate(x, 18.0, PORT_PANEL_Z / 2.0 + 9.0)
            + centered_cube(format!("{PREFIX}_{name}_key_flat"), 18.0, 5.0, 8.0).translate(
                x,
                42.0,
                PORT_PANEL_Z / 2.0 + 18.0,
            );
        clears = clears
            + centered_cylinder(
                format!("{PREFIX}_{name}_port_clearance"),
                PORT_CLEAR_D / 2.0,
                PORT_PANEL_Z + 24.0,
                28,
            )
            .translate(x, 18.0, 0.0);
        caps = caps
            + centered_cylinder(format!("{PREFIX}_{name}_sterile_cap_pocket"), 15.0, 9.0, 28)
                .translate(x, -52.0, PORT_PANEL_Z / 2.0 + 4.5);
    }
    panel - clears + bosses + caps + port_route_grooves()
}

fn port_route_grooves() -> Part {
    let mut grooves = Part::empty(format!("{PREFIX}_input_port_route_grooves"));
    for i in 0..INPUT_PORTS.len() {
        let x = -((INPUT_PORTS.len() - 1) as f64) * PORT_PITCH_X / 2.0 + i as f64 * PORT_PITCH_X;
        grooves = grooves
            + centered_cube(
                format!("{PREFIX}_input_port_route_groove_{i}"),
                10.0,
                128.0,
                6.0,
            )
            .translate(x, 0.0, PORT_PANEL_Z / 2.0 + 3.0);
    }
    grooves
}

fn timing_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_timing_token_rail_body"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_timing_token_slot_cuts"));
    let mut labels = Part::empty(format!("{PREFIX}_timing_token_label_lands"));
    let start_x = -((TIMING_STATES.len() - 1) as f64) * TOKEN_PITCH_X / 2.0;
    for (i, state) in TIMING_STATES.iter().enumerate() {
        let x = start_x + i as f64 * TOKEN_PITCH_X;
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_{state}_token_slot"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_SLOT_Z,
            )
            .translate(x, 0.0, TOKEN_RAIL_Z / 2.0 - TOKEN_SLOT_Z / 2.0 + 1.0);
        labels = labels
            + centered_cube(
                format!("{PREFIX}_{state}_label_land"),
                TOKEN_SLOT_X - 14.0,
                14.0,
                4.0,
            )
            .translate(x, -TOKEN_RAIL_Y / 2.0 - 12.0, TOKEN_RAIL_Z / 2.0 + 2.0);
    }
    rail - cuts + labels + timer_stop_bosses()
}

fn timer_stop_bosses() -> Part {
    centered_cylinder(
        format!("{PREFIX}_timer_rail_start_stop_boss"),
        12.0,
        10.0,
        28,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 28.0, 0.0, TOKEN_RAIL_Z / 2.0 + 5.0)
        + centered_cylinder(format!("{PREFIX}_timer_rail_end_stop_boss"), 12.0, 10.0, 28).translate(
            TOKEN_RAIL_X / 2.0 - 28.0,
            0.0,
            TOKEN_RAIL_Z / 2.0 + 5.0,
        )
}

fn temperature_probe_wells() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_temperature_probe_block"),
        PROBE_BLOCK_X,
        PROBE_BLOCK_Y,
        PROBE_BLOCK_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_temperature_probe_well_cuts"));
    let mut retainers = Part::empty(format!("{PREFIX}_temperature_probe_retainer_bosses"));
    let start_x = -((PROBE_WELLS - 1) as f64) * PROBE_WELL_PITCH_X / 2.0;
    for i in 0..PROBE_WELLS {
        let x = start_x + i as f64 * PROBE_WELL_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_temperature_probe_well_{i}"),
                PROBE_WELL_D / 2.0,
                PROBE_BLOCK_Z + 10.0,
                32,
            )
            .translate(x, 0.0, 0.0);
        retainers = retainers
            + centered_cube(
                format!("{PREFIX}_temperature_probe_clip_land_{i}"),
                34.0,
                16.0,
                5.0,
            )
            .translate(x, 42.0, PROBE_BLOCK_Z / 2.0 + 2.5);
    }
    block - cuts + retainers
}

fn wash_recovery_collection_nests() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_wash_recovery_collection_block"),
        COLLECTION_BLOCK_X,
        COLLECTION_BLOCK_Y,
        COLLECTION_BLOCK_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_wash_recovery_collection_nest_cuts"));
    let mut lips = Part::empty(format!("{PREFIX}_wash_recovery_collection_lips"));
    let start_x = -((COLLECTION_NESTS - 1) as f64) * COLLECTION_PITCH_X / 2.0;
    for i in 0..COLLECTION_NESTS {
        let x = start_x + i as f64 * COLLECTION_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_wash_recovery_collection_nest_{i}"),
                COLLECTION_NEST_D / 2.0,
                COLLECTION_BLOCK_Z + 10.0,
                40,
            )
            .translate(x, 20.0, 0.0);
        lips = lips
            + centered_cylinder(
                format!("{PREFIX}_wash_recovery_collection_lip_{i}"),
                COLLECTION_NEST_D / 2.0 + 5.0,
                6.0,
                40,
            )
            .translate(x, 20.0, COLLECTION_BLOCK_Z / 2.0 + 3.0)
            + centered_cube(
                format!("{PREFIX}_collection_chain_of_custody_land_{i}"),
                36.0,
                18.0,
                4.0,
            )
            .translate(x, -50.0, COLLECTION_BLOCK_Z / 2.0 + 2.0);
    }
    block - cuts + lips
}

fn residue_neutralization_witness_wells() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_residue_neutralization_witness_block"),
        WITNESS_BLOCK_X,
        WITNESS_BLOCK_Y,
        WITNESS_BLOCK_Z,
    );
    let mut wells = Part::empty(format!("{PREFIX}_residue_neutralization_witness_cuts"));
    let mut tiles = Part::empty(format!("{PREFIX}_neutralization_reference_tiles"));
    let start_x = -((RESIDUE_WELLS - 1) as f64) * RESIDUE_PITCH_X / 2.0;
    for i in 0..RESIDUE_WELLS {
        let x = start_x + i as f64 * RESIDUE_PITCH_X;
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_residue_neutralization_witness_well_{i}"),
                RESIDUE_WELL_D / 2.0,
                WITNESS_BLOCK_Z + 10.0,
                36,
            )
            .translate(x, 20.0, 0.0);
    }
    for i in 0..PH_TILE_COUNT {
        let x = -150.0 + i as f64 * 60.0;
        tiles = tiles
            + centered_cube(
                format!("{PREFIX}_ph_color_reference_tile_{i}"),
                42.0,
                20.0,
                4.0,
            )
            .translate(x, -42.0, WITNESS_BLOCK_Z / 2.0 + 2.0);
    }
    block - wells + tiles
}

fn bubble_dead_volume_windows() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_bubble_dead_volume_window_plate"),
        WINDOW_PLATE_X,
        WINDOW_PLATE_Y,
        WINDOW_PLATE_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_dead_volume_window_cuts"));
    let mut route_tabs = Part::empty(format!("{PREFIX}_bubble_dead_volume_route_tabs"));
    let start_x = -((OBSERVATION_WINDOWS - 1) as f64) * WINDOW_PITCH_X / 2.0;
    for i in 0..OBSERVATION_WINDOWS {
        let x = start_x + i as f64 * WINDOW_PITCH_X;
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_bubble_dead_volume_clear_window_{i}"),
                WINDOW_SLOT_X,
                WINDOW_SLOT_Y,
                WINDOW_PLATE_Z + 8.0,
            )
            .translate(x, 0.0, 0.0);
        route_tabs = route_tabs
            + centered_cube(
                format!("{PREFIX}_bubble_dead_volume_route_tab_{i}"),
                WINDOW_SLOT_X + 14.0,
                10.0,
                5.0,
            )
            .translate(x, -WINDOW_SLOT_Y / 2.0 - 10.0, WINDOW_PLATE_Z / 2.0 + 2.5);
    }
    plate - cuts + route_tabs
}

fn barcode_status_lands() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_barcode_status_panel"),
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty(format!("{PREFIX}_barcode_status_lands"));
    for i in 0..BARCODE_LANDS {
        let x = -315.0 + (i % 6) as f64 * 126.0;
        let y = -22.0 + (i / 6) as f64 * 44.0;
        lands = lands
            + centered_cube(format!("{PREFIX}_barcode_land_{i}"), 92.0, 24.0, 4.0).translate(
                x,
                y,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for (lane_i, lane) in STATUS_LANES.iter().enumerate() {
        let x = -270.0 + lane_i as f64 * 180.0;
        for j in 0..STATUS_TOKENS_PER_LANE {
            lands = lands
                + centered_cube(
                    format!("{PREFIX}_{lane}_status_token_pocket_{j}"),
                    42.0,
                    22.0,
                    5.0,
                )
                .translate(
                    x + j as f64 * 48.0,
                    TRACE_PANEL_Y / 2.0 + 20.0,
                    TRACE_PANEL_Z / 2.0 + 2.5,
                );
        }
    }
    panel + lands
}

fn waste_segregation_manifold() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_waste_segregation_manifold_panel"),
        WASTE_PANEL_X,
        WASTE_PANEL_Y,
        WASTE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_waste_stream_port_cuts"));
    let mut nests = Part::empty(format!("{PREFIX}_waste_stream_bag_nests"));
    let pitch_y = WASTE_PANEL_Y / (WASTE_STREAMS.len() as f64 + 1.0);
    for (i, stream) in WASTE_STREAMS.iter().enumerate() {
        let y = -WASTE_PANEL_Y / 2.0 + pitch_y * (i as f64 + 1.0);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_{stream}_waste_port_cut"),
                WASTE_PORT_D / 2.0,
                WASTE_PANEL_Z + 10.0,
                32,
            )
            .translate(-26.0, y, 0.0);
        nests = nests
            + centered_cube(
                format!("{PREFIX}_{stream}_waste_bag_nest"),
                78.0,
                58.0,
                10.0,
            )
            .translate(20.0, y, WASTE_PANEL_Z / 2.0 + 5.0)
            + centered_cube(
                format!("{PREFIX}_{stream}_waste_barcode_land"),
                56.0,
                18.0,
                4.0,
            )
            .translate(20.0, y + 43.0, WASTE_PANEL_Z / 2.0 + 2.0);
    }
    panel - cuts + nests + waste_cross_contamination_dividers()
}

fn waste_cross_contamination_dividers() -> Part {
    let mut dividers = Part::empty(format!("{PREFIX}_waste_cross_contamination_dividers"));
    for i in 0..(WASTE_STREAMS.len() - 1) {
        let y = -WASTE_PANEL_Y / 2.0
            + (i as f64 + 1.5) * (WASTE_PANEL_Y / (WASTE_STREAMS.len() as f64 + 1.0));
        dividers = dividers
            + centered_cube(
                format!("{PREFIX}_waste_stream_divider_{i}"),
                WASTE_PANEL_X + 16.0,
                8.0,
                22.0,
            )
            .translate(0.0, y, WASTE_PANEL_Z / 2.0 + 11.0);
    }
    dividers
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        0.0,
        -BRIDGE_BEAM_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        -BRIDGE_BEAM_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_evidence_bridge_beam"),
        BRIDGE_SPAN_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    );
    let mut cameras = Part::empty(format!("{PREFIX}_evidence_camera_windows"));
    for i in 0..CAMERA_WINDOWS {
        let x = -420.0 + i as f64 * 210.0;
        cameras =
            cameras
                + centered_cube(format!("{PREFIX}_camera_window_land_{i}"), 92.0, 18.0, 5.0)
                    .translate(x, -BRIDGE_POST_Y / 2.0 - 12.0, BRIDGE_BEAM_Z / 2.0 + 2.5)
                + centered_cylinder(format!("{PREFIX}_camera_mount_boss_{i}"), 8.0, 8.0, 24)
                    .translate(x, 0.0, BRIDGE_BEAM_Z / 2.0 + 4.0);
    }
    left_post + right_post + beam + cameras
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_keepout_rail"),
        STATION_X - 110.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE_Y / 2.0, 0.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_service_keepout_rail"),
        STATION_X - 110.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE_Y / 2.0, 0.0);
    let left = centered_cube(
        format!("{PREFIX}_left_vessel_service_keepout_rail"),
        KEEP_OUT_RAIL_W,
        STATION_Y - 70.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 - LEFT_VESSEL_SERVICE_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{PREFIX}_right_waste_service_keepout_rail"),
        KEEP_OUT_RAIL_W,
        STATION_Y - 70.0,
        KEEP_OUT_Z,
    )
    .translate(STATION_X / 2.0 + RIGHT_WASTE_SERVICE_X / 2.0, 0.0, 0.0);
    let overhead = centered_cube(
        format!("{PREFIX}_overhead_evidence_keepout_gauge"),
        STATION_X - 180.0,
        14.0,
        10.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 86.0,
        OVERHEAD_EVIDENCE_CLEARANCE_Z - KEEP_OUT_Z / 2.0,
    );
    front + rear + left + right + overhead
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_features_are_represented() {
        assert_eq!(REQUIRED_FEATURES.len(), OUTPUTS.len() - 1);
        assert!(REQUIRED_FEATURES.contains(&"source_vessel_surrogate_dock"));
        assert!(REQUIRED_FEATURES.contains(&"residue_neutralization_witness_wells"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn layout_contract_keeps_fixture_inside_deck() {
        assert_layout();
    }
}
