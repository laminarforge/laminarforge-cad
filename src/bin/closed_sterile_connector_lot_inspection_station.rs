use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterile connector lot incoming inspection and retention station.
//
// Intent:
// - Receive purchased sterile connector lots without mixing clean incoming
//   pieces, inspection samples, retained samples, and used inspection waste.
// - Present connector trays, caps/plugs, gauge masters, pressure-decay handoff
//   ports, barcode/RFID/COA evidence lands, and release-status lanes in one
//   repeatable benchtop station.
// - Keep leak witness containment, clean/used segregation, robot approach
//   space, and service keepouts visible for cell-culture workcell planning.
//
// This models mechanical packaging/interface CAD for purchased sterile
// connector lots. It is not a sterile barrier design, inspection protocol, or
// release decision system.

const OUTPUTS: [&str; 13] = [
    "output/closed_sterile_connector_lot_inspection_station_deck.stl",
    "output/closed_sterile_connector_lot_inspection_station_incoming_lot_trays.stl",
    "output/closed_sterile_connector_lot_inspection_station_cap_plug_parks.stl",
    "output/closed_sterile_connector_lot_inspection_station_gonogo_gauge_pockets.stl",
    "output/closed_sterile_connector_lot_inspection_station_pressure_decay_handoff_ports.stl",
    "output/closed_sterile_connector_lot_inspection_station_barcode_rfid_coa_lands.stl",
    "output/closed_sterile_connector_lot_inspection_station_retain_sample_pockets.stl",
    "output/closed_sterile_connector_lot_inspection_station_status_lanes.stl",
    "output/closed_sterile_connector_lot_inspection_station_leak_tray.stl",
    "output/closed_sterile_connector_lot_inspection_station_clean_used_segregation.stl",
    "output/closed_sterile_connector_lot_inspection_station_robot_service_keepouts.stl",
    "output/closed_sterile_connector_lot_inspection_station_evidence_bridge.stl",
    "output/closed_sterile_connector_lot_inspection_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "connector_lot_trays",
    "cap_plug_parks",
    "go_nogo_gauge_pockets",
    "pressure_decay_handoff_ports",
    "barcode_rfid_coa_lands",
    "retain_sample_pockets",
    "released_hold_reject_lanes",
    "leak_tray",
    "clean_used_segregation",
    "robot_keepouts",
    "service_keepouts",
    "assembly_export",
];

const DECK_X: f64 = 940.0;
const DECK_Y: f64 = 720.0;
const DECK_Z: f64 = 18.0;
const DECK_CORNER_R: f64 = 16.0;

const LOT_TRAY_COUNT: usize = 6;
const LOT_TRAY_X: f64 = 134.0;
const LOT_TRAY_Y: f64 = 96.0;
const LOT_TRAY_Z: f64 = 28.0;
const LOT_TRAY_COLS: usize = 3;
const LOT_TRAY_ROWS: usize = 2;
const LOT_TRAY_PITCH_X: f64 = 164.0;
const LOT_TRAY_PITCH_Y: f64 = 124.0;
const LOT_TRAY_CENTER_X: f64 = -230.0;
const LOT_TRAY_CENTER_Y: f64 = 170.0;
const CONNECTOR_NESTS_PER_TRAY: usize = 8;
const CONNECTOR_NEST_D: f64 = 15.0;
const CONNECTOR_NEST_DEPTH: f64 = 12.0;

const CAP_PARK_WELLS: usize = 24;
const PLUG_PARK_WELLS: usize = 24;
const CAP_PLUG_PARK_X: f64 = 300.0;
const CAP_PLUG_PARK_Y: f64 = 170.0;
const CAP_PLUG_PARK_Z: f64 = 26.0;
const CAP_PLUG_CENTER_X: f64 = 230.0;
const CAP_PLUG_CENTER_Y: f64 = 165.0;
const CAP_WELL_D: f64 = 9.0;
const PLUG_WELL_D: f64 = 7.0;
const CAP_PLUG_WELL_DEPTH: f64 = 13.0;

const GAUGE_POCKET_COUNT: usize = 12;
const GAUGE_BLOCK_X: f64 = 380.0;
const GAUGE_BLOCK_Y: f64 = 86.0;
const GAUGE_BLOCK_Z: f64 = 30.0;
const GAUGE_CENTER_X: f64 = -205.0;
const GAUGE_CENTER_Y: f64 = -36.0;
const GAUGE_POCKET_X: f64 = 38.0;
const GAUGE_POCKET_Y: f64 = 26.0;
const GAUGE_POCKET_Z: f64 = 15.0;

const PRESSURE_PORTS: usize = 8;
const PRESSURE_BAR_X: f64 = 360.0;
const PRESSURE_BAR_Y: f64 = 74.0;
const PRESSURE_BAR_Z: f64 = 38.0;
const PRESSURE_CENTER_X: f64 = 210.0;
const PRESSURE_CENTER_Y: f64 = -40.0;
const PRESSURE_PORT_D: f64 = 8.0;
const PRESSURE_TUBE_OD: f64 = 4.8;

const EVIDENCE_LANDS: usize = 7;
const EVIDENCE_PANEL_X: f64 = 760.0;
const EVIDENCE_PANEL_Y: f64 = 96.0;
const EVIDENCE_PANEL_Z: f64 = 10.0;
const EVIDENCE_CENTER_X: f64 = 0.0;
const EVIDENCE_CENTER_Y: f64 = -290.0;
#[cfg(test)]
const BARCODE_LANDS: usize = 3;
#[cfg(test)]
const RFID_LANDS: usize = 2;
#[cfg(test)]
const COA_LANDS: usize = 2;

const RETAIN_POCKETS: usize = 18;
const RETAIN_BLOCK_X: f64 = 300.0;
const RETAIN_BLOCK_Y: f64 = 190.0;
const RETAIN_BLOCK_Z: f64 = 34.0;
const RETAIN_CENTER_X: f64 = -300.0;
const RETAIN_CENTER_Y: f64 = -185.0;
const RETAIN_POCKET_D: f64 = 17.0;
const RETAIN_POCKET_DEPTH: f64 = 18.0;

const STATUS_LANES: usize = 3;
const STATUS_LANE_X: f64 = 160.0;
const STATUS_LANE_Y: f64 = 300.0;
const STATUS_LANE_Z: f64 = 22.0;
const STATUS_LANE_PITCH_X: f64 = 184.0;
const STATUS_CENTER_X: f64 = 235.0;
const STATUS_CENTER_Y: f64 = -190.0;
const STATUS_DIVIDER_W: f64 = 12.0;

const LEAK_TRAY_X: f64 = 820.0;
const LEAK_TRAY_Y: f64 = 560.0;
const LEAK_TRAY_Z: f64 = 22.0;
const LEAK_TRAY_WALL: f64 = 10.0;
const LEAK_DRAIN_D: f64 = 10.0;

const CLEAN_USED_BARRIER_X: f64 = 28.0;
const CLEAN_USED_BARRIER_Y: f64 = 540.0;
const CLEAN_USED_BARRIER_Z: f64 = 54.0;
#[cfg(test)]
const CLEAN_USED_GAP_MIN: f64 = 70.0;

const ROBOT_KEEP_OUT_X: f64 = 260.0;
const ROBOT_KEEP_OUT_Y: f64 = 500.0;
const ROBOT_KEEP_OUT_Z: f64 = 160.0;
const SERVICE_KEEP_OUT_X: f64 = 820.0;
const SERVICE_KEEP_OUT_Y: f64 = 160.0;
const SERVICE_KEEP_OUT_Z: f64 = 120.0;
const KEEP_OUT_RAIL: f64 = 6.0;

const BRIDGE_SPAN_X: f64 = 700.0;
const BRIDGE_POST_X: f64 = 22.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_POST_Z: f64 = 170.0;
const BRIDGE_BEAM_Y: f64 = 42.0;
const BRIDGE_BEAM_Z: f64 = 20.0;
const SCANNER_CLEARANCE_Z: f64 = 118.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let lot_trays = incoming_lot_trays();
    export(OUTPUTS[1], &lot_trays);

    let cap_plugs = cap_plug_parks();
    export(OUTPUTS[2], &cap_plugs);

    let gauges = gonogo_gauge_pockets();
    export(OUTPUTS[3], &gauges);

    let pressure_ports = pressure_decay_handoff_ports();
    export(OUTPUTS[4], &pressure_ports);

    let evidence = barcode_rfid_coa_lands();
    export(OUTPUTS[5], &evidence);

    let retain = retain_sample_pockets();
    export(OUTPUTS[6], &retain);

    let status = status_lanes();
    export(OUTPUTS[7], &status);

    let leak = leak_tray();
    export(OUTPUTS[8], &leak);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let bridge = evidence_bridge();
    export(OUTPUTS[11], &bridge);

    let assembly = deck
        + leak.translate(0.0, 0.0, DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0 + 2.0)
        + lot_trays.translate(
            LOT_TRAY_CENTER_X,
            LOT_TRAY_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + LOT_TRAY_Z / 2.0 + 8.0,
        )
        + cap_plugs.translate(
            CAP_PLUG_CENTER_X,
            CAP_PLUG_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + CAP_PLUG_PARK_Z / 2.0 + 8.0,
        )
        + gauges.translate(
            GAUGE_CENTER_X,
            GAUGE_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + GAUGE_BLOCK_Z / 2.0 + 8.0,
        )
        + pressure_ports.translate(
            PRESSURE_CENTER_X,
            PRESSURE_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + PRESSURE_BAR_Z / 2.0 + 8.0,
        )
        + evidence.translate(
            EVIDENCE_CENTER_X,
            EVIDENCE_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + EVIDENCE_PANEL_Z / 2.0 + 8.0,
        )
        + retain.translate(
            RETAIN_CENTER_X,
            RETAIN_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + RETAIN_BLOCK_Z / 2.0 + 8.0,
        )
        + status.translate(
            STATUS_CENTER_X,
            STATUS_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + STATUS_LANE_Z / 2.0 + 8.0,
        )
        + segregation.translate(0.0, -20.0, DECK_Z / 2.0 + LEAK_TRAY_Z + 35.0)
        + bridge.translate(0.0, -250.0, DECK_Z / 2.0 + LEAK_TRAY_Z + 16.0)
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);

    export(OUTPUTS[12], &assembly);

    println!(
        "Closed sterile connector lot inspection station: {:.0}mm x {:.0}mm deck, {} lot trays, {} connector nests, {} cap wells, {} plug wells, {} gauge pockets, {} pressure-decay handoff ports, {} evidence lands, {} retain pockets, released/hold/reject lanes, leak tray, clean/used barrier, and robot/service keepouts.",
        DECK_X,
        DECK_Y,
        LOT_TRAY_COUNT,
        LOT_TRAY_COUNT * CONNECTOR_NESTS_PER_TRAY,
        CAP_PARK_WELLS,
        PLUG_PARK_WELLS,
        GAUGE_POCKET_COUNT,
        PRESSURE_PORTS,
        EVIDENCE_LANDS,
        RETAIN_POCKETS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_sterile_connector_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let corner_reliefs = corner_cylinders(
        "closed_sterile_connector_station_corner_relief",
        DECK_X - 2.0 * DECK_CORNER_R,
        DECK_Y - 2.0 * DECK_CORNER_R,
        DECK_CORNER_R / 2.0,
        DECK_Z + 4.0,
    );
    let leak_tray_socket = centered_cube(
        "closed_sterile_connector_station_leak_tray_socket",
        LEAK_TRAY_X + 24.0,
        LEAK_TRAY_Y + 24.0,
        5.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let evidence_socket = centered_cube(
        "closed_sterile_connector_station_evidence_panel_socket",
        EVIDENCE_PANEL_X + 24.0,
        EVIDENCE_PANEL_Y + 18.0,
        6.0,
    )
    .translate(EVIDENCE_CENTER_X, EVIDENCE_CENTER_Y, DECK_Z / 2.0 - 2.5);

    deck - leak_tray_socket - evidence_socket - corner_reliefs + deck_datum_pads()
}

fn deck_datum_pads() -> Part {
    let mut pads = Part::empty("closed_sterile_connector_station_deck_datum_pads");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        pads = pads
            + centered_cylinder(
                format!("closed_sterile_connector_station_datum_pad_{i}"),
                18.0,
                5.0,
                32,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 2.5);
    }
    pads
}

fn incoming_lot_trays() -> Part {
    let mut trays = Part::empty("closed_sterile_connector_incoming_lot_trays");
    for row in 0..LOT_TRAY_ROWS {
        for col in 0..LOT_TRAY_COLS {
            let index = row * LOT_TRAY_COLS + col;
            let (x, y) = lot_tray_xy(row, col);
            let tray = single_lot_tray(index).translate(x, y, 0.0);
            trays = trays + tray;
        }
    }
    trays
}

fn single_lot_tray(index: usize) -> Part {
    let tray = centered_cube(
        format!("closed_sterile_connector_lot_tray_{index}"),
        LOT_TRAY_X,
        LOT_TRAY_Y,
        LOT_TRAY_Z,
    );
    let pocket = centered_cube(
        format!("closed_sterile_connector_lot_tray_label_recess_{index}"),
        LOT_TRAY_X - 28.0,
        18.0,
        6.0,
    )
    .translate(0.0, LOT_TRAY_Y / 2.0 - 18.0, LOT_TRAY_Z / 2.0 - 2.5);

    tray - pocket - connector_tray_nests(index) + tray_lip(index)
}

fn connector_tray_nests(index: usize) -> Part {
    let mut nests = Part::empty(format!("closed_sterile_connector_tray_{index}_nests"));
    for i in 0..CONNECTOR_NESTS_PER_TRAY {
        let col = i % 4;
        let row = i / 4;
        let x = lane_position(col, 4, 27.0);
        let y = lane_position(row, 2, 35.0) - 8.0;
        nests = nests
            + centered_cylinder(
                format!("closed_sterile_connector_tray_{index}_nest_{i}"),
                CONNECTOR_NEST_D / 2.0,
                CONNECTOR_NEST_DEPTH,
                28,
            )
            .translate(x, y, LOT_TRAY_Z / 2.0 - CONNECTOR_NEST_DEPTH / 2.0 + 1.0);
    }
    nests
}

fn tray_lip(index: usize) -> Part {
    let front = centered_cube(
        format!("closed_sterile_connector_lot_tray_{index}_front_lip"),
        LOT_TRAY_X,
        7.0,
        12.0,
    )
    .translate(0.0, -LOT_TRAY_Y / 2.0 + 3.5, LOT_TRAY_Z / 2.0 + 6.0);
    let rear = centered_cube(
        format!("closed_sterile_connector_lot_tray_{index}_rear_lip"),
        LOT_TRAY_X,
        7.0,
        12.0,
    )
    .translate(0.0, LOT_TRAY_Y / 2.0 - 3.5, LOT_TRAY_Z / 2.0 + 6.0);
    let left = centered_cube(
        format!("closed_sterile_connector_lot_tray_{index}_left_lip"),
        7.0,
        LOT_TRAY_Y,
        12.0,
    )
    .translate(-LOT_TRAY_X / 2.0 + 3.5, 0.0, LOT_TRAY_Z / 2.0 + 6.0);
    let right = centered_cube(
        format!("closed_sterile_connector_lot_tray_{index}_right_lip"),
        7.0,
        LOT_TRAY_Y,
        12.0,
    )
    .translate(LOT_TRAY_X / 2.0 - 3.5, 0.0, LOT_TRAY_Z / 2.0 + 6.0);
    front + rear + left + right
}

fn cap_plug_parks() -> Part {
    let base = centered_cube(
        "closed_sterile_connector_cap_plug_park_base",
        CAP_PLUG_PARK_X,
        CAP_PLUG_PARK_Y,
        CAP_PLUG_PARK_Z,
    );
    let clean_label = centered_cube(
        "closed_sterile_connector_clean_cap_plug_label_land",
        CAP_PLUG_PARK_X - 40.0,
        18.0,
        5.0,
    )
    .translate(
        0.0,
        CAP_PLUG_PARK_Y / 2.0 - 18.0,
        CAP_PLUG_PARK_Z / 2.0 + 2.5,
    );

    base - cap_wells() - plug_wells() + clean_label + park_center_divider()
}

fn cap_wells() -> Part {
    let mut wells = Part::empty("closed_sterile_connector_cap_wells");
    for i in 0..CAP_PARK_WELLS {
        let col = i % 6;
        let row = i / 6;
        wells = wells
            + centered_cylinder(
                format!("closed_sterile_connector_cap_well_{i}"),
                CAP_WELL_D / 2.0,
                CAP_PLUG_WELL_DEPTH,
                24,
            )
            .translate(
                lane_position(col, 6, 24.0),
                28.0 + lane_position(row, 4, 24.0),
                CAP_PLUG_PARK_Z / 2.0 - CAP_PLUG_WELL_DEPTH / 2.0 + 1.0,
            );
    }
    wells
}

fn plug_wells() -> Part {
    let mut wells = Part::empty("closed_sterile_connector_plug_wells");
    for i in 0..PLUG_PARK_WELLS {
        let col = i % 6;
        let row = i / 6;
        wells = wells
            + centered_cylinder(
                format!("closed_sterile_connector_plug_well_{i}"),
                PLUG_WELL_D / 2.0,
                CAP_PLUG_WELL_DEPTH,
                24,
            )
            .translate(
                lane_position(col, 6, 24.0),
                -56.0 + lane_position(row, 4, 18.0),
                CAP_PLUG_PARK_Z / 2.0 - CAP_PLUG_WELL_DEPTH / 2.0 + 1.0,
            );
    }
    wells
}

fn park_center_divider() -> Part {
    centered_cube(
        "closed_sterile_connector_cap_plug_center_divider",
        CAP_PLUG_PARK_X - 30.0,
        8.0,
        18.0,
    )
    .translate(0.0, -12.0, CAP_PLUG_PARK_Z / 2.0 + 9.0)
}

fn gonogo_gauge_pockets() -> Part {
    let base = centered_cube(
        "closed_sterile_connector_gonogo_gauge_block",
        GAUGE_BLOCK_X,
        GAUGE_BLOCK_Y,
        GAUGE_BLOCK_Z,
    );
    let mut pockets = Part::empty("closed_sterile_connector_gonogo_gauge_pockets");
    for i in 0..GAUGE_POCKET_COUNT {
        pockets = pockets
            + centered_cube(
                format!("closed_sterile_connector_gonogo_pocket_{i}"),
                GAUGE_POCKET_X,
                GAUGE_POCKET_Y,
                GAUGE_POCKET_Z,
            )
            .translate(
                lane_position(i, GAUGE_POCKET_COUNT, 28.0),
                0.0,
                GAUGE_BLOCK_Z / 2.0 - GAUGE_POCKET_Z / 2.0 + 1.0,
            );
    }
    let master_land = centered_cube(
        "closed_sterile_connector_gauge_master_label_land",
        GAUGE_BLOCK_X - 40.0,
        16.0,
        5.0,
    )
    .translate(0.0, GAUGE_BLOCK_Y / 2.0 - 16.0, GAUGE_BLOCK_Z / 2.0 + 2.5);
    base - pockets + master_land
}

fn pressure_decay_handoff_ports() -> Part {
    let bar = centered_cube(
        "closed_sterile_connector_pressure_decay_handoff_bar",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    );
    let mut ports = Part::empty("closed_sterile_connector_pressure_decay_ports");
    let mut tube_guides = Part::empty("closed_sterile_connector_pressure_decay_tube_guides");
    for i in 0..PRESSURE_PORTS {
        let x = lane_position(i, PRESSURE_PORTS, 40.0);
        ports = ports
            + centered_cylinder(
                format!("closed_sterile_connector_pressure_decay_port_{i}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_BAR_Y + 4.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, PRESSURE_BAR_Z / 2.0);
        tube_guides = tube_guides
            + centered_cylinder(
                format!("closed_sterile_connector_pressure_decay_tube_guide_{i}"),
                (PRESSURE_TUBE_OD + 1.2) / 2.0,
                26.0,
                24,
            )
            .translate(x, -PRESSURE_BAR_Y / 2.0 - 13.0, PRESSURE_BAR_Z / 2.0);
    }
    bar - ports + tube_guides
}

fn barcode_rfid_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_sterile_connector_barcode_rfid_coa_panel",
        EVIDENCE_PANEL_X,
        EVIDENCE_PANEL_Y,
        EVIDENCE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_sterile_connector_evidence_lands");
    for i in 0..EVIDENCE_LANDS {
        let width = match i {
            0..=2 => 118.0,
            3..=4 => 92.0,
            _ => 134.0,
        };
        lands = lands
            + centered_cube(
                format!("closed_sterile_connector_evidence_land_{i}"),
                width,
                42.0,
                4.0,
            )
            .translate(
                lane_position(i, EVIDENCE_LANDS, 104.0),
                0.0,
                EVIDENCE_PANEL_Z / 2.0 + 2.0,
            );
    }
    panel + lands
}

fn retain_sample_pockets() -> Part {
    let block = centered_cube(
        "closed_sterile_connector_retain_sample_block",
        RETAIN_BLOCK_X,
        RETAIN_BLOCK_Y,
        RETAIN_BLOCK_Z,
    );
    let mut pockets = Part::empty("closed_sterile_connector_retain_sample_pockets");
    for i in 0..RETAIN_POCKETS {
        let col = i % 6;
        let row = i / 6;
        pockets = pockets
            + centered_cylinder(
                format!("closed_sterile_connector_retain_sample_pocket_{i}"),
                RETAIN_POCKET_D / 2.0,
                RETAIN_POCKET_DEPTH,
                28,
            )
            .translate(
                lane_position(col, 6, 38.0),
                lane_position(row, 3, 44.0),
                RETAIN_BLOCK_Z / 2.0 - RETAIN_POCKET_DEPTH / 2.0 + 1.0,
            );
    }
    let seal_land = centered_cube(
        "closed_sterile_connector_retain_chain_of_custody_land",
        RETAIN_BLOCK_X - 44.0,
        22.0,
        5.0,
    )
    .translate(
        0.0,
        -RETAIN_BLOCK_Y / 2.0 + 24.0,
        RETAIN_BLOCK_Z / 2.0 + 2.5,
    );
    block - pockets + seal_land
}

fn status_lanes() -> Part {
    let mut lanes = Part::empty("closed_sterile_connector_release_status_lanes");
    for i in 0..STATUS_LANES {
        let name = match i {
            0 => "released",
            1 => "hold",
            _ => "reject",
        };
        let lane = centered_cube(
            format!("closed_sterile_connector_status_lane_{name}"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(
            lane_position(i, STATUS_LANES, STATUS_LANE_PITCH_X),
            0.0,
            0.0,
        );
        let recess = centered_cube(
            format!("closed_sterile_connector_status_lane_{name}_recess"),
            STATUS_LANE_X - 28.0,
            STATUS_LANE_Y - 44.0,
            7.0,
        )
        .translate(
            lane_position(i, STATUS_LANES, STATUS_LANE_PITCH_X),
            0.0,
            STATUS_LANE_Z / 2.0 - 3.0,
        );
        let label = centered_cube(
            format!("closed_sterile_connector_status_lane_{name}_label_land"),
            STATUS_LANE_X - 38.0,
            24.0,
            5.0,
        )
        .translate(
            lane_position(i, STATUS_LANES, STATUS_LANE_PITCH_X),
            STATUS_LANE_Y / 2.0 - 28.0,
            STATUS_LANE_Z / 2.0 + 2.5,
        );
        lanes = lanes + (lane - recess + label);
    }

    lanes + status_dividers()
}

fn status_dividers() -> Part {
    let left = centered_cube(
        "closed_sterile_connector_status_lane_left_divider",
        STATUS_DIVIDER_W,
        STATUS_LANE_Y,
        36.0,
    )
    .translate(-STATUS_LANE_PITCH_X / 2.0, 0.0, STATUS_LANE_Z / 2.0 + 18.0);
    let right = centered_cube(
        "closed_sterile_connector_status_lane_right_divider",
        STATUS_DIVIDER_W,
        STATUS_LANE_Y,
        36.0,
    )
    .translate(STATUS_LANE_PITCH_X / 2.0, 0.0, STATUS_LANE_Z / 2.0 + 18.0);
    left + right
}

fn leak_tray() -> Part {
    let tray = centered_cube(
        "closed_sterile_connector_leak_witness_tray",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let basin = centered_cube(
        "closed_sterile_connector_leak_witness_basin",
        LEAK_TRAY_X - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Y - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Z - 7.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0 - 5.0);
    let drain = centered_cylinder(
        "closed_sterile_connector_leak_tray_drain",
        LEAK_DRAIN_D / 2.0,
        36.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_TRAY_X / 2.0 - 54.0, -LEAK_TRAY_Y / 2.0 + 10.0, 0.0);

    tray - basin - drain + leak_tray_rails()
}

fn leak_tray_rails() -> Part {
    let front = centered_cube(
        "closed_sterile_connector_leak_tray_front_rail",
        LEAK_TRAY_X,
        LEAK_TRAY_WALL,
        18.0,
    )
    .translate(
        0.0,
        -LEAK_TRAY_Y / 2.0 + LEAK_TRAY_WALL / 2.0,
        LEAK_TRAY_Z / 2.0 + 9.0,
    );
    let rear = centered_cube(
        "closed_sterile_connector_leak_tray_rear_rail",
        LEAK_TRAY_X,
        LEAK_TRAY_WALL,
        18.0,
    )
    .translate(
        0.0,
        LEAK_TRAY_Y / 2.0 - LEAK_TRAY_WALL / 2.0,
        LEAK_TRAY_Z / 2.0 + 9.0,
    );
    let left = centered_cube(
        "closed_sterile_connector_leak_tray_left_rail",
        LEAK_TRAY_WALL,
        LEAK_TRAY_Y,
        18.0,
    )
    .translate(
        -LEAK_TRAY_X / 2.0 + LEAK_TRAY_WALL / 2.0,
        0.0,
        LEAK_TRAY_Z / 2.0 + 9.0,
    );
    let right = centered_cube(
        "closed_sterile_connector_leak_tray_right_rail",
        LEAK_TRAY_WALL,
        LEAK_TRAY_Y,
        18.0,
    )
    .translate(
        LEAK_TRAY_X / 2.0 - LEAK_TRAY_WALL / 2.0,
        0.0,
        LEAK_TRAY_Z / 2.0 + 9.0,
    );
    front + rear + left + right
}

fn clean_used_segregation() -> Part {
    let barrier = centered_cube(
        "closed_sterile_connector_clean_used_center_barrier",
        CLEAN_USED_BARRIER_X,
        CLEAN_USED_BARRIER_Y,
        CLEAN_USED_BARRIER_Z,
    );
    let clean_marker = centered_cube(
        "closed_sterile_connector_clean_side_floor_marker",
        320.0,
        20.0,
        5.0,
    )
    .translate(
        -190.0,
        CLEAN_USED_BARRIER_Y / 2.0 - 34.0,
        -CLEAN_USED_BARRIER_Z / 2.0 + 2.5,
    );
    let used_marker = centered_cube(
        "closed_sterile_connector_used_side_floor_marker",
        320.0,
        20.0,
        5.0,
    )
    .translate(
        190.0,
        -CLEAN_USED_BARRIER_Y / 2.0 + 34.0,
        -CLEAN_USED_BARRIER_Z / 2.0 + 2.5,
    );
    barrier + clean_marker + used_marker
}

fn robot_service_keepouts() -> Part {
    let robot = clearance_box(
        "closed_sterile_connector_robot_pick_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(-DECK_X / 2.0 - ROBOT_KEEP_OUT_X / 2.0 + 40.0, 0.0, 0.0);
    let service_front = clearance_box(
        "closed_sterile_connector_front_service_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -DECK_Y / 2.0 - SERVICE_KEEP_OUT_Y / 2.0 + 35.0, -20.0);
    let service_rear = clearance_box(
        "closed_sterile_connector_rear_service_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, DECK_Y / 2.0 + SERVICE_KEEP_OUT_Y / 2.0 - 35.0, -20.0);
    robot + service_front + service_rear
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_sterile_connector_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let right_post = centered_cube(
        "closed_sterile_connector_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let beam = centered_cube(
        "closed_sterile_connector_evidence_bridge_scanner_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);
    let scanner_land = centered_cube(
        "closed_sterile_connector_evidence_bridge_scanner_mount_land",
        210.0,
        34.0,
        8.0,
    )
    .translate(0.0, 0.0, SCANNER_CLEARANCE_Z);
    left_post + right_post + beam + scanner_land
}

fn corner_cylinders(name: &str, x_span: f64, y_span: f64, radius: f64, height: f64) -> Part {
    let mut part = Part::empty(format!("{name}_set"));
    for (i, dx) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cylinder(format!("{name}_{i}_{j}"), radius, height, 28).translate(
                    dx * x_span / 2.0,
                    dy * y_span / 2.0,
                    0.0,
                );
        }
    }
    part
}

fn clearance_box(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut part = Part::empty(format!("{name}_rails"));
    for (i, dx) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_post_{i}_{j}"), rail, rail, z).translate(
                    dx * x / 2.0,
                    dy * y / 2.0,
                    0.0,
                );
        }
    }
    for (i, dz) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_x_rail_{i}_{j}"), x, rail, rail).translate(
                    0.0,
                    dy * y / 2.0,
                    dz * z / 2.0,
                );
        }
        for (j, dx) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_y_rail_{i}_{j}"), rail, y, rail).translate(
                    dx * x / 2.0,
                    0.0,
                    dz * z / 2.0,
                );
        }
    }
    part
}

fn lot_tray_xy(row: usize, col: usize) -> (f64, f64) {
    (
        lane_position(col, LOT_TRAY_COLS, LOT_TRAY_PITCH_X),
        lane_position(row, LOT_TRAY_ROWS, LOT_TRAY_PITCH_Y),
    )
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_counted() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sterile_connector_lot_inspection_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_station_features_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"connector_lot_trays"));
        assert!(REQUIRED_FEATURES.contains(&"cap_plug_parks"));
        assert!(REQUIRED_FEATURES.contains(&"go_nogo_gauge_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_handoff_ports"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"retain_sample_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"released_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"assembly_export"));
    }

    #[test]
    fn geometry_constants_fit_benchtop_envelope() {
        assert!(DECK_X <= 1000.0);
        assert!(DECK_Y <= 760.0);
        assert!(LEAK_TRAY_X < DECK_X - 80.0);
        assert!(LEAK_TRAY_Y < DECK_Y - 120.0);
        assert!(BRIDGE_POST_Z > SCANNER_CLEARANCE_Z);
        assert!(ROBOT_KEEP_OUT_Z >= 150.0);
        assert!(SERVICE_KEEP_OUT_X < DECK_X);
    }

    #[test]
    fn lot_tray_and_retention_counts_match_sampling_plan() {
        assert_eq!(LOT_TRAY_COUNT, LOT_TRAY_COLS * LOT_TRAY_ROWS);
        assert_eq!(LOT_TRAY_COUNT * CONNECTOR_NESTS_PER_TRAY, 48);
        assert_eq!(RETAIN_POCKETS, 18);
        assert!(RETAIN_POCKETS <= LOT_TRAY_COUNT * CONNECTOR_NESTS_PER_TRAY);
        assert_eq!(CAP_PARK_WELLS, PLUG_PARK_WELLS);
        assert!(CAP_PARK_WELLS >= 2 * GAUGE_POCKET_COUNT);
    }

    #[test]
    fn evidence_lands_cover_barcode_rfid_and_coa() {
        assert_eq!(BARCODE_LANDS + RFID_LANDS + COA_LANDS, EVIDENCE_LANDS);
        assert!(BARCODE_LANDS >= 3);
        assert!(RFID_LANDS >= 2);
        assert!(COA_LANDS >= 2);
        assert!(EVIDENCE_PANEL_X > EVIDENCE_LANDS as f64 * 90.0);
    }

    #[test]
    fn status_lanes_and_segregation_are_sane() {
        assert_eq!(STATUS_LANES, 3);
        assert!(STATUS_LANE_PITCH_X > STATUS_LANE_X + STATUS_DIVIDER_W);
        assert!(CLEAN_USED_BARRIER_X >= 24.0);
        assert!(CLEAN_USED_GAP_MIN >= 64.0);
        assert!(STATUS_CENTER_X > CLEAN_USED_BARRIER_X);
        assert!(RETAIN_CENTER_X < -CLEAN_USED_BARRIER_X);
    }

    #[test]
    fn placement_helpers_keep_trays_inside_carrier() {
        for row in 0..LOT_TRAY_ROWS {
            for col in 0..LOT_TRAY_COLS {
                let (x, y) = lot_tray_xy(row, col);
                assert!(x.abs() + LOT_TRAY_X / 2.0 < 270.0);
                assert!(y.abs() + LOT_TRAY_Y / 2.0 < 170.0);
            }
        }
        assert!((lane_position(0, 3, STATUS_LANE_PITCH_X) + STATUS_LANE_PITCH_X).abs() < 0.001);
        assert!(lane_position(1, 3, STATUS_LANE_PITCH_X).abs() < 0.001);
    }
}
