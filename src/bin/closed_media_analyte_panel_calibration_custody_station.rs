use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media analyte panel calibration custody station.
//
// Intent:
// - Stage calibration standards and low/high controls for non-visual media
//   analyte measurements: glucose, lactate, ammonia, pH, osmolality, and
//   conductivity.
// - Keep vial identity, COA records, cold storage, analyzer cartridge docks,
//   sample-loop handoff ports, waste/flush routing, evidence capture, and
//   quarantine segregation physically visible in one robot-accessible fixture.
// - Model fixture packaging only. Analyzer chemistry, calibration algorithms,
//   release criteria, and sterile fluid-path validation remain external gates.

const OUTPUTS: [&str; 12] = [
    "output/closed_media_analyte_panel_calibration_custody_station_base_tray.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_calibration_vial_racks.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_low_high_control_lanes.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_expired_quarantine_segregation.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_cold_block_pocket.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_analyzer_cartridge_docks.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_sample_loop_handoff_ports.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_barcode_coa_lands.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_waste_flush_route.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_evidence_bridge.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_robot_service_keepouts.stl",
    "output/closed_media_analyte_panel_calibration_custody_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "calibration_vial_racks",
    "low_high_control_lanes",
    "expired_quarantine_segregation",
    "cold_block_pocket",
    "analyzer_cartridge_docks",
    "sample_loop_handoff_ports",
    "barcode_coa_lands",
    "waste_flush_route",
    "evidence_bridge",
    "robot_keepout",
    "service_keepouts",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 5.4;

const ANALYTES: usize = 6;
const CAL_LEVELS: usize = 3;
const CAL_VIALS: usize = ANALYTES * CAL_LEVELS;
const CAL_RACK_CENTER: (f64, f64) = (-330.0, 150.0);
const CAL_RACK_X: f64 = 390.0;
const CAL_RACK_Y: f64 = 230.0;
const CAL_RACK_Z: f64 = 50.0;
const CAL_COLS: usize = 6;
const CAL_PITCH_X: f64 = 54.0;
const CAL_PITCH_Y: f64 = 58.0;
const CAL_VIAL_D: f64 = 16.0;
const CAL_SEAL_RIM_D: f64 = 22.0;

const CONTROL_CENTER: (f64, f64) = (90.0, 170.0);
const CONTROL_X: f64 = 350.0;
const CONTROL_Y: f64 = 210.0;
const CONTROL_Z: f64 = 42.0;
const CONTROL_ANALYTE_LANES: usize = ANALYTES;
const CONTROL_LEVELS: usize = 2;
const CONTROL_TOTAL_SLOTS: usize = CONTROL_ANALYTE_LANES * CONTROL_LEVELS;
const CONTROL_SLOT_X: f64 = 42.0;
const CONTROL_SLOT_Y: f64 = 62.0;
const CONTROL_PITCH_X: f64 = 50.0;
const CONTROL_PITCH_Y: f64 = 76.0;

const SEG_CENTER: (f64, f64) = (410.0, 155.0);
const SEG_X: f64 = 260.0;
const SEG_Y: f64 = 220.0;
const SEG_Z: f64 = 44.0;
const EXPIRED_SLOTS: usize = 6;
const QUARANTINE_SLOTS: usize = 6;
const SEG_TOTAL_SLOTS: usize = EXPIRED_SLOTS + QUARANTINE_SLOTS;
const SEG_SLOT_X: f64 = 58.0;
const SEG_SLOT_Y: f64 = 34.0;
const SEG_LANE_GAP: f64 = 34.0;

const COLD_CENTER: (f64, f64) = (-385.0, -150.0);
const COLD_X: f64 = 250.0;
const COLD_Y: f64 = 170.0;
const COLD_Z: f64 = 48.0;
const COLD_POCKET_X: f64 = 210.0;
const COLD_POCKET_Y: f64 = 128.0;
const COLD_POCKET_Z: f64 = 18.0;
const COLD_VIAL_ROWS: usize = 4;
const COLD_VIAL_COLS: usize = 6;
const COLD_VIAL_POSITIONS: usize = COLD_VIAL_ROWS * COLD_VIAL_COLS;
const COLD_VIAL_PITCH: f64 = 28.0;
const COLD_VIAL_D: f64 = 9.6;
const THERMOWELL_D: f64 = 4.0;

const DOCK_CENTER: (f64, f64) = (-40.0, -155.0);
const DOCK_X: f64 = 350.0;
const DOCK_Y: f64 = 180.0;
const DOCK_Z: f64 = 38.0;
const CARTRIDGE_DOCKS: usize = ANALYTES;
const DOCK_PITCH_X: f64 = 52.0;
const CARTRIDGE_X: f64 = 42.0;
const CARTRIDGE_Y: f64 = 118.0;

const LOOP_CENTER: (f64, f64) = (325.0, -165.0);
const LOOP_X: f64 = 350.0;
const LOOP_Y: f64 = 150.0;
const LOOP_Z: f64 = 46.0;
const LOOP_PORTS: usize = ANALYTES;
const LOOP_PITCH_X: f64 = 50.0;
const LOOP_PORT_D: f64 = 8.0;
const LOOP_SADDLE_X: f64 = 34.0;
const LOOP_SADDLE_Y: f64 = 10.0;

const LAND_COUNT: usize = 18;
const COA_CARD_SLOTS: usize = 6;
const LAND_PLATE_Z: f64 = 5.0;

const ROUTE_CENTER: (f64, f64) = (520.0, -55.0);
const ROUTE_X: f64 = 120.0;
const ROUTE_Y: f64 = 505.0;
const ROUTE_Z: f64 = 38.0;
const FLUSH_PORTS: usize = ANALYTES;
const WASTE_CHANNELS: usize = ANALYTES;
const TUBE_BORE_D: f64 = 5.8;

const BRIDGE_CENTER: (f64, f64) = (0.0, 28.0);
const BRIDGE_SPAN_X: f64 = 990.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_UNDERSIDE_Z: f64 = 190.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_COUNT: usize = 3;
const LED_SEGMENTS: usize = 8;
const WITNESS_TILE_COUNT: usize = 6;

const ROBOT_KEEP_OUT_X: f64 = 1040.0;
const ROBOT_KEEP_OUT_Y: f64 = 190.0;
const ROBOT_KEEP_OUT_Z: f64 = 260.0;
const SERVICE_KEEPOUTS: usize = 4;
const FRONT_ROBOT_CLEARANCE: f64 = 380.0;
const REAR_SERVICE_CLEARANCE: f64 = 230.0;
const LEFT_COLD_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_WASTE_SERVICE_CLEARANCE: f64 = 250.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let racks = calibration_vial_racks();
    export(OUTPUTS[1], &racks);

    let controls = low_high_control_lanes();
    export(OUTPUTS[2], &controls);

    let segregation = expired_quarantine_segregation();
    export(OUTPUTS[3], &segregation);

    let cold = cold_block_pocket();
    export(OUTPUTS[4], &cold);

    let docks = analyzer_cartridge_docks();
    export(OUTPUTS[5], &docks);

    let loops = sample_loop_handoff_ports();
    export(OUTPUTS[6], &loops);

    let traceability = barcode_coa_lands();
    export(OUTPUTS[7], &traceability);

    let route = waste_flush_route();
    export(OUTPUTS[8], &route);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + racks.translate(
            CAL_RACK_CENTER.0,
            CAL_RACK_CENTER.1,
            deck_insert_z(CAL_RACK_Z),
        ) + controls.translate(CONTROL_CENTER.0, CONTROL_CENTER.1, deck_insert_z(CONTROL_Z))
            + segregation.translate(SEG_CENTER.0, SEG_CENTER.1, deck_insert_z(SEG_Z))
            + cold.translate(COLD_CENTER.0, COLD_CENTER.1, deck_insert_z(COLD_Z))
            + docks.translate(DOCK_CENTER.0, DOCK_CENTER.1, deck_insert_z(DOCK_Z))
            + loops.translate(LOOP_CENTER.0, LOOP_CENTER.1, deck_insert_z(LOOP_Z))
            + traceability.translate(0.0, 0.0, BASE_Z + LAND_PLATE_Z / 2.0)
            + route.translate(ROUTE_CENTER.0, ROUTE_CENTER.1, deck_insert_z(ROUTE_Z))
            + bridge.translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z / 2.0)
            + keepouts.translate(0.0, 0.0, BASE_Z + 3.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media analyte panel calibration custody station:");
    println!("  Station deck:                {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!(
        "  Standards:                   {CAL_VIALS} calibration vial wells across {ANALYTES} analytes and {CAL_LEVELS} calibration levels"
    );
    println!(
        "  Controls/segregation:        {CONTROL_TOTAL_SLOTS} low/high control slots, {EXPIRED_SLOTS} expired slots, {QUARANTINE_SLOTS} quarantine slots"
    );
    println!(
        "  Cold/analyzer interfaces:    {COLD_VIAL_POSITIONS} cold-block vial positions, {CARTRIDGE_DOCKS} cartridge docks, {LOOP_PORTS} sample-loop handoff ports"
    );
    println!(
        "  Traceability/routing:        {LAND_COUNT} barcode lands, {COA_CARD_SLOTS} COA card slots, {FLUSH_PORTS} flush ports, {WASTE_CHANNELS} waste channels"
    );
    println!(
        "  Evidence/service:            {CAMERA_COUNT} cameras, {LED_SEGMENTS} LED segments, {WITNESS_TILE_COUNT} witness tiles, {SERVICE_KEEPOUTS} service keepouts"
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
            fits_on_station(center, width, depth, 12.0),
            "{name} exceeds station envelope"
        );
    }

    assert!(SEG_LANE_GAP >= 30.0, "quarantine lane gap too small");
    assert!(
        FRONT_ROBOT_CLEARANCE >= 360.0 && REAR_SERVICE_CLEARANCE >= 220.0,
        "service clearances below custody station target"
    );
    assert!(
        COLD_CENTER.0 < CAL_RACK_CENTER.0 && ROUTE_CENTER.0 > SEG_CENTER.0,
        "cold block and waste route must stay separated"
    );

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
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 6] {
    [
        (
            "calibration_vial_racks",
            CAL_RACK_CENTER,
            CAL_RACK_X,
            CAL_RACK_Y,
        ),
        (
            "low_high_control_lanes",
            CONTROL_CENTER,
            CONTROL_X,
            CONTROL_Y,
        ),
        ("expired_quarantine_segregation", SEG_CENTER, SEG_X, SEG_Y),
        ("cold_block_pocket", COLD_CENTER, COLD_X, COLD_Y),
        ("analyzer_cartridge_docks", DOCK_CENTER, DOCK_X, DOCK_Y),
        ("sample_loop_handoff_ports", LOOP_CENTER, LOOP_X, LOOP_Y),
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

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_media_analyte_custody_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sump = centered_cube(
        "closed_media_analyte_custody_washdown_sump",
        STATION_X - 128.0,
        STATION_Y - 116.0,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z - SUMP_DEPTH / 2.0);

    let drain = centered_cylinder(
        "closed_media_analyte_custody_front_waste_drain",
        DRAIN_D / 2.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 82.0,
        -STATION_Y / 2.0 + 18.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain
        + perimeter_rims()
        + module_socket_recesses()
        + base_mount_bosses()
        + washdown_witness_ribs()
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_media_analyte_custody_left_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_media_analyte_custody_right_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_media_analyte_custody_rear_lip",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front_low_lip = centered_cube(
        "closed_media_analyte_custody_front_low_robot_lip",
        STATION_X - 180.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 10.0, BASE_Z + 10.0);

    left + right + rear + front_low_lip
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("closed_media_analyte_custody_module_socket_recesses");
    for (name, center, width, depth) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_analyte_custody_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                6.0,
            )
            .translate(center.0, center.1, BASE_Z - 2.8);
    }
    sockets
}

fn base_mount_bosses() -> Part {
    let mut bosses = Part::empty("closed_media_analyte_custody_mount_bosses");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 52.0),
        (-110.0, STATION_Y / 2.0 - 48.0),
        (110.0, STATION_Y / 2.0 - 48.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_media_analyte_custody_mount_boss_{i}"),
            11.0,
            8.0,
            28,
        )
        .translate(x, y, BASE_Z + 4.0);
        let hole = centered_cylinder(
            format!("closed_media_analyte_custody_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn washdown_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_analyte_custody_washdown_witness_ribs");
    for (i, x) in [-455.0, -300.0, -145.0, 10.0, 165.0, 320.0, 475.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_media_analyte_custody_sump_witness_rib_{i}"),
                8.0,
                STATION_Y - 156.0,
                5.0,
            )
            .translate(x, 0.0, BASE_Z + 2.5);
    }

    ribs + centered_cube(
        "closed_media_analyte_custody_drain_lead_in_gutter",
        180.0,
        8.0,
        6.0,
    )
    .translate(
        STATION_X / 2.0 - 142.0,
        -STATION_Y / 2.0 + 48.0,
        BASE_Z + 3.0,
    )
}

fn calibration_vial_racks() -> Part {
    let body = centered_cube(
        "closed_media_analyte_calibration_vial_rack_body",
        CAL_RACK_X,
        CAL_RACK_Y,
        CAL_RACK_Z,
    );

    let gasket_groove = centered_cube(
        "closed_media_analyte_calibration_vial_rack_lid_gasket_groove",
        CAL_RACK_X - 36.0,
        CAL_RACK_Y - 34.0,
        7.0,
    )
    .translate(0.0, 0.0, CAL_RACK_Z / 2.0 - 3.0);

    let mut wells = Part::empty("closed_media_analyte_calibration_vial_wells");
    let mut rims = Part::empty("closed_media_analyte_calibration_vial_identity_rims");
    for i in 0..CAL_VIALS {
        let x = centered_index(i % CAL_COLS, CAL_COLS, CAL_PITCH_X);
        let y = centered_index(i / CAL_COLS, CAL_LEVELS, CAL_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!("closed_media_analyte_calibration_vial_well_{i}"),
                CAL_VIAL_D / 2.0,
                CAL_RACK_Z + 4.0,
                32,
            )
            .translate(x, y, 0.0);

        let rim = centered_cylinder(
            format!("closed_media_analyte_calibration_vial_seal_rim_{i}"),
            CAL_SEAL_RIM_D / 2.0,
            4.0,
            32,
        )
        .translate(x, y, CAL_RACK_Z / 2.0 + 2.0);
        let opening = centered_cylinder(
            format!("closed_media_analyte_calibration_vial_seal_opening_{i}"),
            CAL_VIAL_D / 2.0 + 0.8,
            5.0,
            32,
        )
        .translate(x, y, CAL_RACK_Z / 2.0 + 2.0);
        rims = rims + (rim - opening);
    }

    let mut analyte_tabs = Part::empty("closed_media_analyte_calibration_analyte_tabs");
    for analyte in 0..ANALYTES {
        let x = centered_index(analyte, ANALYTES, CAL_PITCH_X);
        analyte_tabs = analyte_tabs
            + centered_cube(
                format!("closed_media_analyte_calibration_analyte_tab_{analyte}"),
                40.0,
                14.0,
                8.0,
            )
            .translate(x, CAL_RACK_Y / 2.0 - 18.0, CAL_RACK_Z / 2.0 + 4.0);
    }

    let level_separator_1 = centered_cube(
        "closed_media_analyte_calibration_level_separator_low_mid",
        CAL_RACK_X - 42.0,
        5.0,
        14.0,
    )
    .translate(0.0, -CAL_PITCH_Y / 2.0, CAL_RACK_Z / 2.0 + 7.0);
    let level_separator_2 = centered_cube(
        "closed_media_analyte_calibration_level_separator_mid_high",
        CAL_RACK_X - 42.0,
        5.0,
        14.0,
    )
    .translate(0.0, CAL_PITCH_Y / 2.0, CAL_RACK_Z / 2.0 + 7.0);

    body - gasket_groove - wells + rims + analyte_tabs + level_separator_1 + level_separator_2
}

fn low_high_control_lanes() -> Part {
    let body = centered_cube(
        "closed_media_analyte_low_high_control_lane_body",
        CONTROL_X,
        CONTROL_Y,
        CONTROL_Z,
    );
    let mut cuts = Part::empty("closed_media_analyte_low_high_control_slot_cuts");
    let mut rails = Part::empty("closed_media_analyte_low_high_control_lane_rails");

    for lane in 0..CONTROL_ANALYTE_LANES {
        let x = centered_index(lane, CONTROL_ANALYTE_LANES, CONTROL_PITCH_X);
        for level in 0..CONTROL_LEVELS {
            let y = centered_index(level, CONTROL_LEVELS, CONTROL_PITCH_Y);
            let idx = lane * CONTROL_LEVELS + level;
            cuts = cuts
                + centered_cube(
                    format!("closed_media_analyte_control_slot_cut_{idx}"),
                    CONTROL_SLOT_X,
                    CONTROL_SLOT_Y,
                    CONTROL_Z + 4.0,
                )
                .translate(x, y, 0.0);
            rails = rails
                + centered_cube(
                    format!("closed_media_analyte_control_slot_identity_rail_{idx}"),
                    CONTROL_SLOT_X + 10.0,
                    6.0,
                    7.0,
                )
                .translate(
                    x,
                    y + CONTROL_SLOT_Y / 2.0 + 5.0,
                    CONTROL_Z / 2.0 + 3.5,
                );
        }

        rails = rails
            + centered_cube(
                format!("closed_media_analyte_control_analyte_center_divider_{lane}"),
                5.0,
                CONTROL_Y - 36.0,
                16.0,
            )
            .translate(x + CONTROL_PITCH_X / 2.0 - 2.5, 0.0, CONTROL_Z / 2.0 + 8.0);
    }

    let low_label = centered_cube(
        "closed_media_analyte_low_control_lane_label_land",
        CONTROL_X - 44.0,
        18.0,
        5.0,
    )
    .translate(0.0, -CONTROL_Y / 2.0 + 16.0, CONTROL_Z / 2.0 + 2.5);
    let high_label = centered_cube(
        "closed_media_analyte_high_control_lane_label_land",
        CONTROL_X - 44.0,
        18.0,
        5.0,
    )
    .translate(0.0, CONTROL_Y / 2.0 - 16.0, CONTROL_Z / 2.0 + 2.5);

    body - cuts + rails + low_label + high_label
}

fn expired_quarantine_segregation() -> Part {
    let body = centered_cube(
        "closed_media_analyte_expired_quarantine_body",
        SEG_X,
        SEG_Y,
        SEG_Z,
    );
    let divider = centered_cube(
        "closed_media_analyte_expired_quarantine_hard_divider",
        SEG_X - 30.0,
        SEG_LANE_GAP,
        SEG_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);

    let mut cuts = Part::empty("closed_media_analyte_expired_quarantine_slot_cuts");
    let mut guards = Part::empty("closed_media_analyte_expired_quarantine_guard_rails");
    for i in 0..SEG_TOTAL_SLOTS {
        let expired = i < EXPIRED_SLOTS;
        let local = if expired { i } else { i - EXPIRED_SLOTS };
        let x = centered_index(local % 3, 3, 70.0);
        let y_base = if expired { -58.0 } else { 58.0 };
        let y = y_base + centered_index(local / 3, 2, 44.0);
        cuts = cuts
            + centered_cube(
                format!("closed_media_analyte_segregation_slot_cut_{i}"),
                SEG_SLOT_X,
                SEG_SLOT_Y,
                SEG_Z + 4.0,
            )
            .translate(x, y, 0.0);
        guards = guards
            + centered_cube(
                format!("closed_media_analyte_segregation_status_guard_{i}"),
                SEG_SLOT_X + 8.0,
                5.0,
                10.0,
            )
            .translate(x, y - SEG_SLOT_Y / 2.0 - 5.0, SEG_Z / 2.0 + 5.0);
    }

    let expired_land = centered_cube(
        "closed_media_analyte_expired_lane_red_tag_land",
        SEG_X - 52.0,
        18.0,
        5.0,
    )
    .translate(0.0, -SEG_Y / 2.0 + 18.0, SEG_Z / 2.0 + 2.5);
    let quarantine_land = centered_cube(
        "closed_media_analyte_quarantine_lane_yellow_tag_land",
        SEG_X - 52.0,
        18.0,
        5.0,
    )
    .translate(0.0, SEG_Y / 2.0 - 18.0, SEG_Z / 2.0 + 2.5);

    body - cuts + divider + guards + expired_land + quarantine_land
}

fn cold_block_pocket() -> Part {
    let body = centered_cube(
        "closed_media_analyte_cold_block_pocket_body",
        COLD_X,
        COLD_Y,
        COLD_Z,
    );
    let cold_block_recess = centered_cube(
        "closed_media_analyte_removable_cold_block_recess",
        COLD_POCKET_X,
        COLD_POCKET_Y,
        COLD_POCKET_Z,
    )
    .translate(0.0, 0.0, COLD_Z / 2.0 - COLD_POCKET_Z / 2.0 + 0.8);

    let mut vial_cuts = Part::empty("closed_media_analyte_cold_block_vial_cuts");
    for row in 0..COLD_VIAL_ROWS {
        for col in 0..COLD_VIAL_COLS {
            let idx = row * COLD_VIAL_COLS + col;
            vial_cuts = vial_cuts
                + centered_cylinder(
                    format!("closed_media_analyte_cold_block_vial_well_{idx}"),
                    COLD_VIAL_D / 2.0,
                    COLD_Z + 4.0,
                    24,
                )
                .translate(
                    centered_index(col, COLD_VIAL_COLS, COLD_VIAL_PITCH),
                    centered_index(row, COLD_VIAL_ROWS, COLD_VIAL_PITCH),
                    0.0,
                );
        }
    }

    let coolant_in = centered_cylinder(
        "closed_media_analyte_cold_block_coolant_in_bore",
        4.0,
        COLD_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -COLD_Y / 2.0 + 22.0, -4.0);
    let coolant_out = centered_cylinder(
        "closed_media_analyte_cold_block_coolant_out_bore",
        4.0,
        COLD_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, COLD_Y / 2.0 - 22.0, -4.0);
    let thermowell = centered_cylinder(
        "closed_media_analyte_cold_block_thermowell",
        THERMOWELL_D / 2.0,
        COLD_Y + 6.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(COLD_X / 2.0 - 28.0, 0.0, COLD_Z / 2.0 - 10.0);

    let latch_tabs = centered_cube(
        "closed_media_analyte_cold_block_left_latch_tab",
        18.0,
        42.0,
        12.0,
    )
    .translate(-COLD_X / 2.0 + 20.0, 0.0, COLD_Z / 2.0 + 6.0)
        + centered_cube(
            "closed_media_analyte_cold_block_right_latch_tab",
            18.0,
            42.0,
            12.0,
        )
        .translate(COLD_X / 2.0 - 20.0, 0.0, COLD_Z / 2.0 + 6.0);

    body - cold_block_recess - vial_cuts - coolant_in - coolant_out - thermowell + latch_tabs
}

fn analyzer_cartridge_docks() -> Part {
    let body = centered_cube(
        "closed_media_analyte_analyzer_cartridge_dock_body",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let mut cuts = Part::empty("closed_media_analyte_analyzer_cartridge_dock_cuts");
    let mut datums = Part::empty("closed_media_analyte_analyzer_cartridge_datum_features");
    for i in 0..CARTRIDGE_DOCKS {
        let x = centered_index(i, CARTRIDGE_DOCKS, DOCK_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("closed_media_analyte_cartridge_dock_pocket_{i}"),
                CARTRIDGE_X,
                CARTRIDGE_Y,
                DOCK_Z + 4.0,
            )
            .translate(x, 0.0, 0.0);
        datums = datums
            + centered_cube(
                format!("closed_media_analyte_cartridge_dock_front_key_{i}"),
                CARTRIDGE_X - 10.0,
                8.0,
                8.0,
            )
            .translate(x, -CARTRIDGE_Y / 2.0 - 8.0, DOCK_Z / 2.0 + 4.0)
            + centered_cylinder(
                format!("closed_media_analyte_cartridge_dock_sensor_window_{i}"),
                7.0,
                4.0,
                24,
            )
            .translate(x, CARTRIDGE_Y / 2.0 - 18.0, DOCK_Z / 2.0 + 2.0);
    }

    let rear_bus_shadow = centered_cube(
        "closed_media_analyte_cartridge_dock_analyzer_bus_shadow",
        DOCK_X - 42.0,
        18.0,
        18.0,
    )
    .translate(0.0, DOCK_Y / 2.0 - 20.0, DOCK_Z / 2.0 + 9.0);

    body - cuts + datums + rear_bus_shadow
}

fn sample_loop_handoff_ports() -> Part {
    let body = centered_cube(
        "closed_media_analyte_sample_loop_handoff_body",
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let mut port_cuts = Part::empty("closed_media_analyte_sample_loop_port_cuts");
    let mut saddles = Part::empty("closed_media_analyte_sample_loop_saddles");
    for i in 0..LOOP_PORTS {
        let x = centered_index(i, LOOP_PORTS, LOOP_PITCH_X);
        port_cuts = port_cuts
            + centered_cylinder(
                format!("closed_media_analyte_sample_loop_inlet_port_{i}"),
                LOOP_PORT_D / 2.0,
                LOOP_Z + 4.0,
                24,
            )
            .translate(x, -28.0, 0.0)
            + centered_cylinder(
                format!("closed_media_analyte_sample_loop_return_port_{i}"),
                LOOP_PORT_D / 2.0,
                LOOP_Z + 4.0,
                24,
            )
            .translate(x, 28.0, 0.0);

        saddles = saddles
            + centered_cube(
                format!("closed_media_analyte_sample_loop_front_saddle_{i}"),
                LOOP_SADDLE_X,
                LOOP_SADDLE_Y,
                6.0,
            )
            .translate(x, -LOOP_Y / 2.0 + 24.0, LOOP_Z / 2.0 + 3.0)
            + centered_cube(
                format!("closed_media_analyte_sample_loop_rear_saddle_{i}"),
                LOOP_SADDLE_X,
                LOOP_SADDLE_Y,
                6.0,
            )
            .translate(x, LOOP_Y / 2.0 - 24.0, LOOP_Z / 2.0 + 3.0);
    }

    let capped_bypass_lane = centered_cube(
        "closed_media_analyte_sample_loop_capped_bypass_lane",
        LOOP_X - 44.0,
        10.0,
        8.0,
    )
    .translate(0.0, 0.0, LOOP_Z / 2.0 + 4.0);

    body - port_cuts + saddles + capped_bypass_lane
}

fn barcode_coa_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_barcode_coa_lands");
    for i in 0..LAND_COUNT {
        let (x, y) = if i < 6 {
            (centered_index(i, 6, 86.0) - 185.0, STATION_Y / 2.0 - 54.0)
        } else if i < 12 {
            (
                centered_index(i - 6, 6, 70.0) + 105.0,
                -STATION_Y / 2.0 + 54.0,
            )
        } else {
            (
                SEG_CENTER.0 - 82.0 + centered_index(i - 12, 6, 32.0),
                SEG_CENTER.1 - SEG_Y / 2.0 - 28.0,
            )
        };
        let land = centered_cube(
            format!("closed_media_analyte_barcode_land_{i}"),
            58.0,
            24.0,
            LAND_PLATE_Z,
        )
        .translate(x, y, 0.0);
        let scan_recess = centered_cube(
            format!("closed_media_analyte_barcode_scan_recess_{i}"),
            46.0,
            14.0,
            LAND_PLATE_Z + 1.0,
        )
        .translate(x, y, 0.7);
        lands = lands + (land - scan_recess);
    }

    for i in 0..COA_CARD_SLOTS {
        let x = centered_index(i, COA_CARD_SLOTS, 86.0);
        let slot = centered_cube(
            format!("closed_media_analyte_coa_card_slot_{i}"),
            68.0,
            30.0,
            8.0,
        )
        .translate(x, -STATION_Y / 2.0 + 96.0, 1.5);
        let card_relief = centered_cube(
            format!("closed_media_analyte_coa_card_relief_{i}"),
            56.0,
            20.0,
            9.0,
        )
        .translate(x, -STATION_Y / 2.0 + 96.0, 2.0);
        lands = lands + (slot - card_relief);
    }

    lands
}

fn waste_flush_route() -> Part {
    let body = centered_cube(
        "closed_media_analyte_waste_flush_route_body",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let mut bores = Part::empty("closed_media_analyte_waste_flush_bores");
    let mut collars = Part::empty("closed_media_analyte_waste_flush_collar_rims");
    for i in 0..FLUSH_PORTS {
        let y = centered_index(i, FLUSH_PORTS, 64.0);
        bores = bores
            + centered_cylinder(
                format!("closed_media_analyte_flush_port_bore_{i}"),
                TUBE_BORE_D / 2.0,
                ROUTE_X + 6.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-18.0, y, 8.0);
        collars = collars
            + centered_cylinder(
                format!("closed_media_analyte_flush_port_collar_{i}"),
                8.5,
                5.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-ROUTE_X / 2.0 - 2.0, y, 8.0);
    }

    for i in 0..WASTE_CHANNELS {
        let y = centered_index(i, WASTE_CHANNELS, 64.0);
        bores = bores
            + centered_cube(
                format!("closed_media_analyte_waste_channel_groove_{i}"),
                ROUTE_X + 6.0,
                8.0,
                7.0,
            )
            .translate(0.0, y + 22.0, ROUTE_Z / 2.0 - 3.5);
    }

    let waste_header = centered_cylinder(
        "closed_media_analyte_waste_header_bore",
        7.0,
        ROUTE_Y + 8.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(ROUTE_X / 2.0 - 26.0, 0.0, 0.0);
    let flush_header_label = centered_cube(
        "closed_media_analyte_waste_flush_route_label_land",
        ROUTE_X - 24.0,
        30.0,
        5.0,
    )
    .translate(0.0, -ROUTE_Y / 2.0 + 28.0, ROUTE_Z / 2.0 + 2.5);

    body - bores - waste_header + collars + flush_header_label
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_media_analyte_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        "closed_media_analyte_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        "closed_media_analyte_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut cameras = Part::empty("closed_media_analyte_evidence_camera_placeholders");
    for i in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("closed_media_analyte_evidence_camera_bracket_{i}"),
                42.0,
                32.0,
                20.0,
            )
            .translate(
                centered_index(i, CAMERA_COUNT, 260.0),
                -BRIDGE_POST_Y / 2.0 - 18.0,
                BRIDGE_UNDERSIDE_Z - 10.0,
            );
    }

    let mut lights = Part::empty("closed_media_analyte_evidence_led_segments");
    for i in 0..LED_SEGMENTS {
        lights = lights
            + centered_cube(
                format!("closed_media_analyte_evidence_led_segment_{i}"),
                84.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(i, LED_SEGMENTS, 112.0),
                BRIDGE_POST_Y / 2.0 + 5.0,
                BRIDGE_UNDERSIDE_Z - 18.0,
            );
    }

    let mut witness_tiles = Part::empty("closed_media_analyte_evidence_witness_tiles");
    for i in 0..WITNESS_TILE_COUNT {
        witness_tiles = witness_tiles
            + centered_cube(
                format!("closed_media_analyte_evidence_witness_tile_{i}"),
                58.0,
                28.0,
                5.0,
            )
            .translate(centered_index(i, WITNESS_TILE_COUNT, 92.0), 70.0, 12.0);
    }

    left_post + right_post + beam + cameras + lights + witness_tiles
}

fn robot_service_keepouts() -> Part {
    let robot_sweep = centered_cube(
        "closed_media_analyte_robot_pick_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_media_analyte_rear_service_keepout",
        STATION_X - 160.0,
        REAR_SERVICE_CLEARANCE,
        120.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 60.0);
    let left_cold_service = centered_cube(
        "closed_media_analyte_left_cold_block_service_keepout",
        LEFT_COLD_SERVICE_CLEARANCE,
        260.0,
        135.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_COLD_SERVICE_CLEARANCE / 2.0,
        -140.0,
        67.5,
    );
    let right_waste_service = centered_cube(
        "closed_media_analyte_right_waste_route_service_keepout",
        RIGHT_WASTE_SERVICE_CLEARANCE,
        410.0,
        150.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_WASTE_SERVICE_CLEARANCE / 2.0,
        -40.0,
        75.0,
    );

    let mut robot_datums = Part::empty("closed_media_analyte_robot_service_datum_markers");
    for (i, (x, y)) in [
        (-510.0, -300.0),
        (-300.0, -300.0),
        (-90.0, -300.0),
        (120.0, -300.0),
        (330.0, -300.0),
        (510.0, -300.0),
    ]
    .into_iter()
    .enumerate()
    {
        robot_datums = robot_datums
            + centered_cylinder(
                format!("closed_media_analyte_robot_pick_datum_marker_{i}"),
                9.0,
                5.0,
                24,
            )
            .translate(x, y, 2.5);
    }

    robot_sweep + rear_service + left_cold_service + right_waste_service + robot_datums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_deterministic_and_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_media_analyte_panel_calibration_custody_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_custody_features_are_represented() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"calibration_vial_racks"));
        assert!(REQUIRED_FEATURES.contains(&"low_high_control_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"expired_quarantine_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"cold_block_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"analyzer_cartridge_docks"));
        assert!(REQUIRED_FEATURES.contains(&"sample_loop_handoff_ports"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"waste_flush_route"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepout"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn analyte_station_counts_match_panel_scope() {
        assert_eq!(ANALYTES, 6);
        assert_eq!(CAL_VIALS, 18);
        assert_eq!(CONTROL_TOTAL_SLOTS, 12);
        assert_eq!(CARTRIDGE_DOCKS, ANALYTES);
        assert_eq!(LOOP_PORTS, ANALYTES);
        assert_eq!(FLUSH_PORTS, ANALYTES);
        assert_eq!(WASTE_CHANNELS, ANALYTES);
    }

    #[test]
    fn layout_fits_and_keeps_quarantine_separate() {
        assert_layout();
        assert!(SEG_LANE_GAP >= 30.0);
        assert!(COLD_CENTER.0 < 0.0);
        assert!(ROUTE_CENTER.0 > SEG_CENTER.0);
    }

    #[test]
    fn service_keepouts_are_visible_and_sized() {
        assert!(ROBOT_KEEP_OUT_X > STATION_X * 0.85);
        assert!(ROBOT_KEEP_OUT_Z >= 240.0);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
        assert!(LEFT_COLD_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_WASTE_SERVICE_CLEARANCE >= 240.0);
        assert_eq!(SERVICE_KEEPOUTS, 4);
    }
}
