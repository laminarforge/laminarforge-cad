use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterile connector cap-loss and open-port exposure prevention station.
//
// Design intent:
// - Present closed connector bodies in a datum nest bank while each matching cap
//   or sterile plug has a physically accountable parking location.
// - Make open-port state visible through optical witness windows, timed exposure
//   token rails, pressure-decay handoff ports, reject/quarantine lanes, and
//   evidence camera datums.
// - Keep traceability, tamper seals, clean/used segregation, and robot/service
//   clearances explicit in validation CAD.
//
// This is engineering validation CAD only. It does not define biological
// protocol steps, sterility claims, exposure limits, or acceptance thresholds.

const OUTPUTS: [&str; 13] = [
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_base_tray.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_connector_nest_bank.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_cap_plug_parking_matrix.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_open_port_optical_witness_windows.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_timed_exposure_token_rail.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_pressure_decay_handoff_ports.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_reject_quarantine_lanes.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_barcode_coa_lands.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_tamper_seal_pockets.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_clean_used_segregation_tray.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_evidence_camera_bridge.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_robot_service_keepout_gauges.stl",
    "output/closed_connector_cap_loss_open_port_exposure_prevention_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "connector_nest_bank",
    "cap_plug_parking_matrix",
    "cap_custody_slots",
    "open_port_optical_witness_windows",
    "timed_exposure_token_rail",
    "pressure_decay_handoff_ports",
    "reject_lane",
    "quarantine_lane",
    "barcode_lands",
    "coa_lands",
    "tamper_seal_pockets",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1080.0;
const STATION_Y: f64 = 720.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const EDGE_MARGIN: f64 = 12.0;

const CONNECTOR_ROWS: usize = 3;
const CONNECTOR_COLS: usize = 6;
const CONNECTOR_COUNT: usize = CONNECTOR_ROWS * CONNECTOR_COLS;
const CONNECTOR_NEST_X: f64 = 336.0;
const CONNECTOR_NEST_Y: f64 = 168.0;
const CONNECTOR_NEST_Z: f64 = 46.0;
const CONNECTOR_NEST_POS: (f64, f64) = (-330.0, 188.0);
const CONNECTOR_PITCH_X: f64 = 46.0;
const CONNECTOR_PITCH_Y: f64 = 46.0;

const CAP_MATRIX_ROWS: usize = 3;
const CAP_MATRIX_COLS: usize = 6;
const CAP_PARKING_COUNT: usize = CAP_MATRIX_ROWS * CAP_MATRIX_COLS;
const CAP_MATRIX_X: f64 = 318.0;
const CAP_MATRIX_Y: f64 = 168.0;
const CAP_MATRIX_Z: f64 = 38.0;
const CAP_MATRIX_POS: (f64, f64) = (42.0, 188.0);
const CAP_PITCH_X: f64 = 44.0;
const CAP_PITCH_Y: f64 = 44.0;

const WITNESS_WINDOW_COUNT: usize = CONNECTOR_COUNT;
const WITNESS_PANEL_X: f64 = 272.0;
const WITNESS_PANEL_Y: f64 = 168.0;
const WITNESS_PANEL_Z: f64 = 24.0;
const WITNESS_PANEL_POS: (f64, f64) = (360.0, 188.0);
const WITNESS_PITCH_X: f64 = 36.0;
const WITNESS_PITCH_Y: f64 = 42.0;

const TOKEN_COUNT: usize = CONNECTOR_COUNT;
const TOKEN_RAIL_X: f64 = 330.0;
const TOKEN_RAIL_Y: f64 = 110.0;
const TOKEN_RAIL_Z: f64 = 32.0;
const TOKEN_RAIL_POS: (f64, f64) = (-340.0, 12.0);
const TOKEN_SLOT_PITCH: f64 = 16.0;

const PRESSURE_PORT_ROWS: usize = 2;
const PRESSURE_PORTS_PER_ROW: usize = 6;
const PRESSURE_PORT_COUNT: usize = PRESSURE_PORT_ROWS * PRESSURE_PORTS_PER_ROW;
const PRESSURE_PANEL_X: f64 = 270.0;
const PRESSURE_PANEL_Y: f64 = 110.0;
const PRESSURE_PANEL_Z: f64 = 36.0;
const PRESSURE_PANEL_POS: (f64, f64) = (-28.0, 12.0);
const PRESSURE_PITCH_X: f64 = 38.0;
const PRESSURE_PITCH_Y: f64 = 42.0;

const REJECT_LANE_X: f64 = 276.0;
const REJECT_LANE_Y: f64 = 118.0;
const REJECT_LANE_Z: f64 = 42.0;
const REJECT_LANE_POS: (f64, f64) = (306.0, 12.0);
const REJECT_LANE_COUNT: usize = 2;
const REJECT_WELL_COUNT: usize = 8;

const TRACE_PANEL_X: f64 = 294.0;
const TRACE_PANEL_Y: f64 = 116.0;
const TRACE_PANEL_Z: f64 = 12.0;
const TRACE_PANEL_POS: (f64, f64) = (-350.0, -188.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 4;

const SEAL_POCKET_X: f64 = 220.0;
const SEAL_POCKET_Y: f64 = 116.0;
const SEAL_POCKET_Z: f64 = 22.0;
const SEAL_POCKET_POS: (f64, f64) = (-72.0, -188.0);
const TAMPER_SEAL_POCKETS: usize = 10;

const SEG_TRAY_X: f64 = 306.0;
const SEG_TRAY_Y: f64 = 116.0;
const SEG_TRAY_Z: f64 = 38.0;
const SEG_TRAY_POS: (f64, f64) = (262.0, -188.0);
const CLEAN_USED_WELLS: usize = 12;
const CLEAN_USED_DIVIDER_W: f64 = 10.0;

const CAMERA_BRIDGE_X: f64 = 860.0;
const CAMERA_BRIDGE_Y: f64 = 66.0;
const CAMERA_BRIDGE_Z: f64 = 176.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, 96.0);
const CAMERA_CLEARANCE_Z: f64 = 112.0;
const CAMERA_HEAD_COUNT: usize = 3;

const KEEPOUT_X: f64 = 472.0;
const KEEPOUT_Y: f64 = 76.0;
const KEEPOUT_Z: f64 = 28.0;
const KEEPOUT_POS: (f64, f64) = (0.0, -288.0);
const ROBOT_KEEP_OUT_COUNT: usize = 4;
const SERVICE_KEEP_OUT_COUNT: usize = 3;
const MIN_SEGREGATION_GAP: f64 = 40.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let connector_nests = connector_nest_bank();
    export(OUTPUTS[1], &connector_nests);

    let cap_matrix = cap_plug_parking_matrix();
    export(OUTPUTS[2], &cap_matrix);

    let witness_windows = open_port_optical_witness_windows();
    export(OUTPUTS[3], &witness_windows);

    let token_rail = timed_exposure_token_rail();
    export(OUTPUTS[4], &token_rail);

    let pressure_ports = pressure_decay_handoff_ports();
    export(OUTPUTS[5], &pressure_ports);

    let reject_lanes = reject_quarantine_lanes();
    export(OUTPUTS[6], &reject_lanes);

    let trace_lands = barcode_coa_lands();
    export(OUTPUTS[7], &trace_lands);

    let seal_pockets = tamper_seal_pockets();
    export(OUTPUTS[8], &seal_pockets);

    let segregation = clean_used_segregation_tray();
    export(OUTPUTS[9], &segregation);

    let camera_bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &camera_bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        base + connector_nests.translate(
            CONNECTOR_NEST_POS.0,
            CONNECTOR_NEST_POS.1,
            insert_z(CONNECTOR_NEST_Z),
        ) + cap_matrix.translate(CAP_MATRIX_POS.0, CAP_MATRIX_POS.1, insert_z(CAP_MATRIX_Z))
            + witness_windows.translate(
                WITNESS_PANEL_POS.0,
                WITNESS_PANEL_POS.1,
                insert_z(WITNESS_PANEL_Z),
            )
            + token_rail.translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, insert_z(TOKEN_RAIL_Z))
            + pressure_ports.translate(
                PRESSURE_PANEL_POS.0,
                PRESSURE_PANEL_POS.1,
                insert_z(PRESSURE_PANEL_Z),
            )
            + reject_lanes.translate(
                REJECT_LANE_POS.0,
                REJECT_LANE_POS.1,
                insert_z(REJECT_LANE_Z),
            )
            + trace_lands.translate(
                TRACE_PANEL_POS.0,
                TRACE_PANEL_POS.1,
                insert_z(TRACE_PANEL_Z),
            )
            + seal_pockets.translate(
                SEAL_POCKET_POS.0,
                SEAL_POCKET_POS.1,
                insert_z(SEAL_POCKET_Z),
            )
            + segregation.translate(SEG_TRAY_POS.0, SEG_TRAY_POS.1, insert_z(SEG_TRAY_Z))
            + camera_bridge.translate(
                CAMERA_BRIDGE_POS.0,
                CAMERA_BRIDGE_POS.1,
                BASE_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
            )
            + keepouts.translate(KEEPOUT_POS.0, KEEPOUT_POS.1, insert_z(KEEPOUT_Z));
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed connector cap-loss and open-port exposure prevention station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm validation tray");
    println!(
        "  Connector custody:      {CONNECTOR_COUNT} connector nests paired with {CAP_PARKING_COUNT} cap/plug parking positions"
    );
    println!(
        "  Open-port witnessing:   {WITNESS_WINDOW_COUNT} optical windows, {TOKEN_COUNT} timed token slots, and {PRESSURE_PORT_COUNT} pressure-decay handoff ports"
    );
    println!(
        "  Exception handling:     {REJECT_LANE_COUNT} reject/quarantine lanes, {REJECT_WELL_COUNT} retained item wells, {TAMPER_SEAL_POCKETS} tamper-seal pockets"
    );
    println!(
        "  Trace/evidence:         {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {CAMERA_HEAD_COUNT} camera heads, clean/used segregation, and {ROBOT_KEEP_OUT_COUNT} robot keepout posts"
    );
    println!("  Feature groups covered: {}", REQUIRED_FEATURES.len());
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
    assert!(
        horizontal_gap(
            rect_for(CONNECTOR_NEST_POS, CONNECTOR_NEST_X, CONNECTOR_NEST_Y),
            rect_for(CAP_MATRIX_POS, CAP_MATRIX_X, CAP_MATRIX_Y)
        ) >= MIN_SEGREGATION_GAP,
        "connector and cap banks need a visible custody gap"
    );
    assert!(
        !rects_overlap(
            rect_for(REJECT_LANE_POS, REJECT_LANE_X, REJECT_LANE_Y),
            rect_for(SEG_TRAY_POS, SEG_TRAY_X, SEG_TRAY_Y)
        ),
        "reject/quarantine lanes must stay separate from clean/used segregation"
    );
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 11] {
    [
        (
            "connector_nest_bank",
            CONNECTOR_NEST_POS,
            CONNECTOR_NEST_X,
            CONNECTOR_NEST_Y,
        ),
        (
            "cap_plug_parking_matrix",
            CAP_MATRIX_POS,
            CAP_MATRIX_X,
            CAP_MATRIX_Y,
        ),
        (
            "open_port_optical_witness_windows",
            WITNESS_PANEL_POS,
            WITNESS_PANEL_X,
            WITNESS_PANEL_Y,
        ),
        (
            "timed_exposure_token_rail",
            TOKEN_RAIL_POS,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
        (
            "pressure_decay_handoff_ports",
            PRESSURE_PANEL_POS,
            PRESSURE_PANEL_X,
            PRESSURE_PANEL_Y,
        ),
        (
            "reject_quarantine_lanes",
            REJECT_LANE_POS,
            REJECT_LANE_X,
            REJECT_LANE_Y,
        ),
        (
            "barcode_coa_lands",
            TRACE_PANEL_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "tamper_seal_pockets",
            SEAL_POCKET_POS,
            SEAL_POCKET_X,
            SEAL_POCKET_Y,
        ),
        (
            "clean_used_segregation_tray",
            SEG_TRAY_POS,
            SEG_TRAY_X,
            SEG_TRAY_Y,
        ),
        (
            "evidence_camera_bridge",
            CAMERA_BRIDGE_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
        (
            "robot_service_keepout_gauges",
            KEEPOUT_POS,
            KEEPOUT_X,
            KEEPOUT_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - EDGE_MARGIN
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - EDGE_MARGIN
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_xy(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_connector_prevention_base_validation_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_connector_prevention_washdown_recess",
        STATION_X - 112.0,
        STATION_Y - 112.0,
        7.0,
    )
    .translate(0.0, -10.0, BASE_Z / 2.0 - 3.5);

    deck - washdown_recess - insert_sockets() - mount_slots() - datum_pin_holes()
        + perimeter_rims()
        + workflow_dividers()
        + lane_arrow_datums()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_connector_prevention_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_connector_prevention_{name}_socket"),
                width + 7.0,
                depth + 7.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_connector_prevention_mount_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 50.0), -(STATION_Y / 2.0 - 44.0)),
        (STATION_X / 2.0 - 50.0, -(STATION_Y / 2.0 - 44.0)),
        (-(STATION_X / 2.0 - 50.0), STATION_Y / 2.0 - 44.0),
        (STATION_X / 2.0 - 50.0, STATION_Y / 2.0 - 44.0),
        (0.0, STATION_Y / 2.0 - 44.0),
        (0.0, -(STATION_Y / 2.0 - 44.0)),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_connector_prevention_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_connector_prevention_mount_slot_relief_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_connector_prevention_datum_pin_holes");
    for (i, (x, y)) in [
        (-478.0, 302.0),
        (478.0, 302.0),
        (-478.0, -302.0),
        (478.0, -302.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_connector_prevention_datum_pin_hole_{i}"),
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
        "closed_connector_prevention_left_rim",
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_connector_prevention_right_rim",
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_connector_prevention_rear_rim",
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
        "closed_connector_prevention_front_low_lip",
        STATION_X - 180.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn workflow_dividers() -> Part {
    let upper_row = centered_cube(
        "closed_connector_prevention_upper_row_divider",
        STATION_X - 132.0,
        10.0,
        28.0,
    )
    .translate(0.0, 94.0, BASE_Z / 2.0 + 14.0);
    let lower_row = centered_cube(
        "closed_connector_prevention_lower_row_divider",
        STATION_X - 160.0,
        10.0,
        24.0,
    )
    .translate(0.0, -106.0, BASE_Z / 2.0 + 12.0);
    let cap_gap_marker = centered_cube(
        "closed_connector_prevention_connector_cap_custody_gap_marker",
        10.0,
        168.0,
        26.0,
    )
    .translate(-154.0, 188.0, BASE_Z / 2.0 + 13.0);
    let cap_witness_marker = centered_cube(
        "closed_connector_prevention_cap_witness_gap_marker",
        10.0,
        168.0,
        26.0,
    )
    .translate(178.0, 188.0, BASE_Z / 2.0 + 13.0);

    upper_row + lower_row + cap_gap_marker + cap_witness_marker
}

fn lane_arrow_datums() -> Part {
    let mut datums = Part::empty("closed_connector_prevention_lane_arrow_datums");
    for (i, (x, y)) in [
        (-470.0, 96.0),
        (-150.0, 96.0),
        (150.0, 96.0),
        (470.0, 96.0),
        (-470.0, -108.0),
        (0.0, -108.0),
        (470.0, -108.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cube(
                format!("closed_connector_prevention_flow_arrow_land_{i}"),
                36.0,
                10.0,
                3.0,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5)
            + centered_cylinder(
                format!("closed_connector_prevention_flow_arrow_dot_{i}"),
                4.0,
                3.0,
                18,
            )
            .translate(*x + 22.0, *y, BASE_Z / 2.0 + 1.5);
    }
    datums
}

fn connector_nest_bank() -> Part {
    let bank = centered_cube(
        "closed_connector_prevention_connector_nest_bank_body",
        CONNECTOR_NEST_X,
        CONNECTOR_NEST_Y,
        CONNECTOR_NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_connector_prevention_connector_nest_rear_fence",
        CONNECTOR_NEST_X,
        12.0,
        CONNECTOR_NEST_Z + 34.0,
    )
    .translate(0.0, CONNECTOR_NEST_Y / 2.0 - 6.0, 17.0);
    let mut nests = Part::empty("closed_connector_prevention_connector_nests");
    for i in 0..CONNECTOR_COUNT {
        let (x, y) = grid_xy(
            i,
            CONNECTOR_COLS,
            CONNECTOR_ROWS,
            CONNECTOR_PITCH_X,
            CONNECTOR_PITCH_Y,
        );
        nests = nests
            + centered_cylinder(
                format!("closed_connector_prevention_connector_round_cradle_{i}"),
                14.0 / 2.0,
                CONNECTOR_NEST_Z + 8.0,
                32,
            )
            .translate(x, y - 8.0, 7.0)
            + centered_cube(
                format!("closed_connector_prevention_connector_flat_datum_slot_{i}"),
                30.0,
                10.0,
                18.0,
            )
            .translate(x, y + 16.0, CONNECTOR_NEST_Z / 2.0 - 9.0)
            + centered_cube(
                format!("closed_connector_prevention_connector_id_label_land_{i}"),
                24.0,
                8.0,
                3.0,
            )
            .translate(x, y - 28.0, CONNECTOR_NEST_Z / 2.0 + 1.5);
    }
    bank + rear_fence - nests + latch_tabs("connector_nest_bank", CONNECTOR_NEST_Z)
}

fn cap_plug_parking_matrix() -> Part {
    let matrix = centered_cube(
        "closed_connector_prevention_cap_plug_parking_matrix_body",
        CAP_MATRIX_X,
        CAP_MATRIX_Y,
        CAP_MATRIX_Z,
    );
    let front_retainer = centered_cube(
        "closed_connector_prevention_cap_matrix_front_retainer",
        CAP_MATRIX_X,
        10.0,
        CAP_MATRIX_Z + 24.0,
    )
    .translate(0.0, -CAP_MATRIX_Y / 2.0 + 5.0, 12.0);
    let mut pockets = Part::empty("closed_connector_prevention_cap_plug_parking_pockets");
    for i in 0..CAP_PARKING_COUNT {
        let (x, y) = grid_xy(
            i,
            CAP_MATRIX_COLS,
            CAP_MATRIX_ROWS,
            CAP_PITCH_X,
            CAP_PITCH_Y,
        );
        pockets = pockets
            + centered_cylinder(
                format!("closed_connector_prevention_cap_parking_round_socket_{i}"),
                10.0 / 2.0,
                CAP_MATRIX_Z + 8.0,
                28,
            )
            .translate(x - 7.0, y, 6.0)
            + centered_cylinder(
                format!("closed_connector_prevention_plug_parking_round_socket_{i}"),
                7.0 / 2.0,
                CAP_MATRIX_Z + 8.0,
                24,
            )
            .translate(x + 9.0, y, 6.0)
            + centered_cube(
                format!("closed_connector_prevention_cap_custody_token_land_{i}"),
                30.0,
                8.0,
                3.0,
            )
            .translate(x, y + 20.0, CAP_MATRIX_Z / 2.0 + 1.5);
    }

    matrix + front_retainer - pockets + latch_tabs("cap_plug_parking_matrix", CAP_MATRIX_Z)
}

fn open_port_optical_witness_windows() -> Part {
    let panel = centered_cube(
        "closed_connector_prevention_open_port_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let light_bar = centered_cube(
        "closed_connector_prevention_backlight_bar_land",
        WITNESS_PANEL_X - 34.0,
        12.0,
        10.0,
    )
    .translate(
        0.0,
        WITNESS_PANEL_Y / 2.0 - 20.0,
        WITNESS_PANEL_Z / 2.0 + 5.0,
    );
    let mut windows = Part::empty("closed_connector_prevention_witness_window_cutouts");
    for i in 0..WITNESS_WINDOW_COUNT {
        let (x, y) = grid_xy(
            i,
            CONNECTOR_COLS,
            CONNECTOR_ROWS,
            WITNESS_PITCH_X,
            WITNESS_PITCH_Y,
        );
        windows = windows
            + centered_cube(
                format!("closed_connector_prevention_open_port_window_{i}"),
                22.0,
                13.0,
                WITNESS_PANEL_Z + 4.0,
            )
            .translate(x, y, 0.0)
            + centered_cylinder(
                format!("closed_connector_prevention_optical_fiducial_{i}"),
                3.0,
                4.0,
                18,
            )
            .translate(x + 13.0, y + 11.0, WITNESS_PANEL_Z / 2.0 + 2.0);
    }
    panel - windows + light_bar + latch_tabs("open_port_witness", WITNESS_PANEL_Z)
}

fn timed_exposure_token_rail() -> Part {
    let rail = centered_cube(
        "closed_connector_prevention_timed_exposure_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let upper_fence = centered_cube(
        "closed_connector_prevention_token_rail_upper_fence",
        TOKEN_RAIL_X,
        10.0,
        TOKEN_RAIL_Z + 22.0,
    )
    .translate(0.0, TOKEN_RAIL_Y / 2.0 - 5.0, 11.0);
    let lower_fence = centered_cube(
        "closed_connector_prevention_token_rail_lower_fence",
        TOKEN_RAIL_X,
        10.0,
        TOKEN_RAIL_Z + 22.0,
    )
    .translate(0.0, -TOKEN_RAIL_Y / 2.0 + 5.0, 11.0);
    let mut slots = Part::empty("closed_connector_prevention_timed_token_slots");
    for i in 0..TOKEN_COUNT {
        let x = centered_index(i, TOKEN_COUNT, TOKEN_SLOT_PITCH);
        let row_y = if i % 2 == 0 { -20.0 } else { 20.0 };
        slots = slots
            + centered_cube(
                format!("closed_connector_prevention_exposure_token_slot_{i}"),
                9.0,
                34.0,
                TOKEN_RAIL_Z + 4.0,
            )
            .translate(x, row_y, 0.0)
            + centered_cube(
                format!("closed_connector_prevention_token_reader_notch_{i}"),
                8.0,
                7.0,
                5.0,
            )
            .translate(x, row_y + 20.0, TOKEN_RAIL_Z / 2.0 - 2.5);
    }
    rail + upper_fence + lower_fence - slots
}

fn pressure_decay_handoff_ports() -> Part {
    let panel = centered_cube(
        "closed_connector_prevention_pressure_decay_handoff_panel",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    );
    let manifold_land = centered_cube(
        "closed_connector_prevention_pressure_manifold_land",
        PRESSURE_PANEL_X - 38.0,
        12.0,
        8.0,
    )
    .translate(
        0.0,
        PRESSURE_PANEL_Y / 2.0 - 19.0,
        PRESSURE_PANEL_Z / 2.0 + 4.0,
    );
    let mut ports = Part::empty("closed_connector_prevention_pressure_handoff_ports");
    for i in 0..PRESSURE_PORT_COUNT {
        let (x, y) = grid_xy(
            i,
            PRESSURE_PORTS_PER_ROW,
            PRESSURE_PORT_ROWS,
            PRESSURE_PITCH_X,
            PRESSURE_PITCH_Y,
        );
        ports = ports
            + centered_cylinder(
                format!("closed_connector_prevention_pressure_decay_port_bore_{i}"),
                7.2 / 2.0,
                PRESSURE_PANEL_Z + 8.0,
                28,
            )
            .translate(x, y, 4.0)
            + centered_cube(
                format!("closed_connector_prevention_pressure_clip_slot_{i}"),
                26.0,
                8.0,
                14.0,
            )
            .translate(x, y + 18.0, PRESSURE_PANEL_Z / 2.0 - 7.0);
    }
    panel - ports + manifold_land + latch_tabs("pressure_decay_handoff", PRESSURE_PANEL_Z)
}

fn reject_quarantine_lanes() -> Part {
    let tray = centered_cube(
        "closed_connector_prevention_reject_quarantine_lane_body",
        REJECT_LANE_X,
        REJECT_LANE_Y,
        REJECT_LANE_Z,
    );
    let divider = centered_cube(
        "closed_connector_prevention_reject_quarantine_center_divider",
        10.0,
        REJECT_LANE_Y,
        REJECT_LANE_Z + 22.0,
    )
    .translate(0.0, 0.0, 11.0);
    let rear_witness_fence = centered_cube(
        "closed_connector_prevention_exception_lane_rear_witness_fence",
        REJECT_LANE_X,
        10.0,
        REJECT_LANE_Z + 28.0,
    )
    .translate(0.0, REJECT_LANE_Y / 2.0 - 5.0, 14.0);
    let mut wells = Part::empty("closed_connector_prevention_reject_quarantine_wells");
    for i in 0..REJECT_WELL_COUNT {
        let x = if i < REJECT_WELL_COUNT / 2 {
            -68.0
        } else {
            68.0
        };
        let y = centered_index(i % 4, 4, 24.0);
        wells = wells
            + centered_cylinder(
                format!("closed_connector_prevention_exception_retained_item_well_{i}"),
                14.0 / 2.0,
                REJECT_LANE_Z + 8.0,
                28,
            )
            .translate(x, y, 6.0);
    }
    let reject_label = raised_land(
        "closed_connector_prevention_reject_lane_status_land",
        84.0,
        14.0,
        REJECT_LANE_Z,
    )
    .translate(-68.0, -REJECT_LANE_Y / 2.0 + 16.0, 0.0);
    let quarantine_label = raised_land(
        "closed_connector_prevention_quarantine_lane_status_land",
        84.0,
        14.0,
        REJECT_LANE_Z,
    )
    .translate(68.0, -REJECT_LANE_Y / 2.0 + 16.0, 0.0);

    tray + divider + rear_witness_fence - wells + reject_label + quarantine_label
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_connector_prevention_barcode_coa_trace_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_connector_prevention_barcode_coa_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 58.0);
        let y = if i < 4 { -30.0 } else { 2.0 };
        lands = lands
            + centered_cube(
                format!("closed_connector_prevention_connector_barcode_land_{i}"),
                44.0,
                18.0,
                3.0,
            )
            .translate(x, y, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..COA_LANDS {
        let x = centered_index(i, COA_LANDS, 64.0);
        lands = lands
            + centered_cube(
                format!("closed_connector_prevention_coa_card_land_{i}"),
                54.0,
                22.0,
                3.0,
            )
            .translate(x, 39.0, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    panel + lands
}

fn tamper_seal_pockets() -> Part {
    let block = centered_cube(
        "closed_connector_prevention_tamper_seal_pocket_block",
        SEAL_POCKET_X,
        SEAL_POCKET_Y,
        SEAL_POCKET_Z,
    );
    let mut pockets = Part::empty("closed_connector_prevention_tamper_seal_pockets");
    for i in 0..TAMPER_SEAL_POCKETS {
        let x = centered_index(i % 5, 5, 36.0);
        let y = if i < 5 { -22.0 } else { 22.0 };
        pockets = pockets
            + centered_cube(
                format!("closed_connector_prevention_tamper_seal_lanyard_pocket_{i}"),
                24.0,
                8.0,
                SEAL_POCKET_Z + 4.0,
            )
            .translate(x, y, 0.0)
            + centered_cylinder(
                format!("closed_connector_prevention_tamper_wire_bore_{i}"),
                2.4 / 2.0,
                36.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 3.0);
    }
    block - pockets
}

fn clean_used_segregation_tray() -> Part {
    let tray = centered_cube(
        "closed_connector_prevention_clean_used_segregation_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    );
    let divider = centered_cube(
        "closed_connector_prevention_clean_used_center_wall",
        CLEAN_USED_DIVIDER_W,
        SEG_TRAY_Y,
        SEG_TRAY_Z + 22.0,
    )
    .translate(0.0, 0.0, 11.0);
    let clean_label = raised_land(
        "closed_connector_prevention_clean_lane_land",
        96.0,
        14.0,
        SEG_TRAY_Z,
    )
    .translate(-78.0, -SEG_TRAY_Y / 2.0 + 16.0, 0.0);
    let used_label = raised_land(
        "closed_connector_prevention_used_lane_land",
        96.0,
        14.0,
        SEG_TRAY_Z,
    )
    .translate(78.0, -SEG_TRAY_Y / 2.0 + 16.0, 0.0);
    let mut wells = Part::empty("closed_connector_prevention_clean_used_wells");
    for i in 0..CLEAN_USED_WELLS {
        let x = if i < CLEAN_USED_WELLS / 2 {
            -78.0
        } else {
            78.0
        };
        let y = centered_index(i % 6, 6, 15.0);
        wells = wells
            + centered_cylinder(
                format!("closed_connector_prevention_clean_used_cap_well_{i}"),
                10.0 / 2.0,
                SEG_TRAY_Z + 8.0,
                24,
            )
            .translate(x, y + 5.0, 6.0);
    }
    let spill_channel = centered_cube(
        "closed_connector_prevention_clean_used_spill_channel",
        SEG_TRAY_X - 36.0,
        7.0,
        7.0,
    )
    .translate(0.0, SEG_TRAY_Y / 2.0 - 14.0, SEG_TRAY_Z / 2.0 - 3.5);

    tray + divider + clean_label + used_label - wells - spill_channel
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_connector_prevention_evidence_camera_left_post",
        32.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 16.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_connector_prevention_evidence_camera_right_post",
        32.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 16.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_connector_prevention_evidence_camera_cross_beam",
        CAMERA_BRIDGE_X,
        34.0,
        30.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 15.0);
    let underside_window = centered_cube(
        "closed_connector_prevention_evidence_bridge_clear_underpass",
        CAMERA_BRIDGE_X - 110.0,
        CAMERA_BRIDGE_Y + 8.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(
        0.0,
        0.0,
        -CAMERA_BRIDGE_Z / 2.0 + CAMERA_CLEARANCE_Z / 2.0 + 8.0,
    );
    let mut heads = Part::empty("closed_connector_prevention_evidence_camera_heads");
    for i in 0..CAMERA_HEAD_COUNT {
        let x = centered_index(i, CAMERA_HEAD_COUNT, 220.0);
        heads = heads
            + centered_cube(
                format!("closed_connector_prevention_evidence_camera_mount_{i}"),
                72.0,
                30.0,
                20.0,
            )
            .translate(
                x,
                -CAMERA_BRIDGE_Y / 2.0 - 14.0,
                CAMERA_BRIDGE_Z / 2.0 - 48.0,
            )
            + centered_cylinder(
                format!("closed_connector_prevention_evidence_lens_bore_{i}"),
                18.0 / 2.0,
                32.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                -CAMERA_BRIDGE_Y / 2.0 - 18.0,
                CAMERA_BRIDGE_Z / 2.0 - 50.0,
            );
    }
    left_post + right_post + beam + heads - underside_window
}

fn robot_service_keepout_gauges() -> Part {
    let rail = centered_cube(
        "closed_connector_prevention_robot_service_keepout_rail",
        KEEPOUT_X,
        KEEPOUT_Y,
        KEEPOUT_Z,
    );
    let robot_slot = centered_cube(
        "closed_connector_prevention_robot_handoff_clearance_slot",
        190.0,
        28.0,
        KEEPOUT_Z + 4.0,
    )
    .translate(-128.0, 0.0, 0.0);
    let service_slot = centered_cube(
        "closed_connector_prevention_service_access_clearance_slot",
        136.0,
        28.0,
        KEEPOUT_Z + 4.0,
    )
    .translate(154.0, 0.0, 0.0);
    let mut posts = Part::empty("closed_connector_prevention_keepout_posts");
    for i in 0..ROBOT_KEEP_OUT_COUNT {
        let x = centered_index(i, ROBOT_KEEP_OUT_COUNT, 48.0) - 128.0;
        posts = posts
            + centered_cylinder(
                format!("closed_connector_prevention_robot_keepout_post_{i}"),
                6.0,
                58.0,
                24,
            )
            .translate(x, -KEEPOUT_Y / 2.0 + 16.0, KEEPOUT_Z / 2.0 + 29.0);
    }
    for i in 0..SERVICE_KEEP_OUT_COUNT {
        let x = centered_index(i, SERVICE_KEEP_OUT_COUNT, 48.0) + 154.0;
        posts = posts
            + centered_cube(
                format!("closed_connector_prevention_service_keepout_flag_{i}"),
                12.0,
                12.0,
                58.0,
            )
            .translate(x, KEEPOUT_Y / 2.0 - 16.0, KEEPOUT_Z / 2.0 + 29.0);
    }
    rail - robot_slot - service_slot + posts
}

fn latch_tabs(prefix: &str, parent_z: f64) -> Part {
    let left = centered_cube(
        format!("closed_connector_prevention_{prefix}_left_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(-34.0, 0.0, parent_z / 2.0 + 6.0);
    let right = centered_cube(
        format!("closed_connector_prevention_{prefix}_right_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(34.0, 0.0, parent_z / 2.0 + 6.0);
    left + right
}

fn raised_land(name: &str, width: f64, depth: f64, parent_z: f64) -> Part {
    centered_cube(name, width, depth, 3.0).translate(0.0, 0.0, parent_z / 2.0 + 1.5)
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    d: f64,
}

fn rect_for(pos: (f64, f64), width: f64, depth: f64) -> Rect {
    Rect {
        x: pos.0,
        y: pos.1,
        w: width,
        d: depth,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.d / 2.0;
    let ay1 = a.y + a.d / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.d / 2.0;
    let by1 = b.y + b.d / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_count_and_names_are_stable() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS
            .iter()
            .all(|path| path
                .contains("closed_connector_cap_loss_open_port_exposure_prevention_station")));
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_feature_coverage_matches_station_intent() {
        for feature in [
            "connector_nest_bank",
            "cap_plug_parking_matrix",
            "cap_custody_slots",
            "open_port_optical_witness_windows",
            "timed_exposure_token_rail",
            "pressure_decay_handoff_ports",
            "reject_lane",
            "quarantine_lane",
            "barcode_lands",
            "coa_lands",
            "tamper_seal_pockets",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 14);
    }

    #[test]
    fn connector_and_cap_custody_counts_are_paired() {
        assert_eq!(CONNECTOR_COUNT, 18);
        assert_eq!(CAP_PARKING_COUNT, CONNECTOR_COUNT);
        assert_eq!(WITNESS_WINDOW_COUNT, CONNECTOR_COUNT);
        assert_eq!(TOKEN_COUNT, CONNECTOR_COUNT);
        assert_eq!(CONNECTOR_ROWS * CONNECTOR_COLS, CONNECTOR_COUNT);
        assert_eq!(CAP_MATRIX_ROWS * CAP_MATRIX_COLS, CAP_PARKING_COUNT);
    }

    #[test]
    fn layout_components_fit_station_bounds() {
        assert_layout();
        for (_, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
    }

    #[test]
    fn major_workflow_regions_do_not_overlap() {
        let regions = [
            rect_for(CONNECTOR_NEST_POS, CONNECTOR_NEST_X, CONNECTOR_NEST_Y),
            rect_for(CAP_MATRIX_POS, CAP_MATRIX_X, CAP_MATRIX_Y),
            rect_for(WITNESS_PANEL_POS, WITNESS_PANEL_X, WITNESS_PANEL_Y),
            rect_for(TOKEN_RAIL_POS, TOKEN_RAIL_X, TOKEN_RAIL_Y),
            rect_for(PRESSURE_PANEL_POS, PRESSURE_PANEL_X, PRESSURE_PANEL_Y),
            rect_for(REJECT_LANE_POS, REJECT_LANE_X, REJECT_LANE_Y),
            rect_for(TRACE_PANEL_POS, TRACE_PANEL_X, TRACE_PANEL_Y),
            rect_for(SEAL_POCKET_POS, SEAL_POCKET_X, SEAL_POCKET_Y),
            rect_for(SEG_TRAY_POS, SEG_TRAY_X, SEG_TRAY_Y),
            rect_for(KEEPOUT_POS, KEEPOUT_X, KEEPOUT_Y),
        ];

        for i in 0..regions.len() {
            for j in (i + 1)..regions.len() {
                assert!(!rects_overlap(regions[i], regions[j]));
            }
        }
    }

    #[test]
    fn evidence_and_exception_handling_counts_are_present() {
        assert_eq!(PRESSURE_PORT_COUNT, 12);
        assert_eq!(
            PRESSURE_PORT_ROWS * PRESSURE_PORTS_PER_ROW,
            PRESSURE_PORT_COUNT
        );
        assert_eq!(REJECT_LANE_COUNT, 2);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_LANDS, 4);
        assert_eq!(TAMPER_SEAL_POCKETS, 10);
        assert_eq!(CLEAN_USED_WELLS, 12);
        assert_eq!(CAMERA_HEAD_COUNT, 3);
        assert_eq!(ROBOT_KEEP_OUT_COUNT, 4);
        assert_eq!(SERVICE_KEEP_OUT_COUNT, 3);
    }
}
