use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator gas recovery after service-port opening validation station.
//
// Mechanical, no-cell fixture for packaging a closed-system validation event:
// a repeatable service-port/access-panel opening surrogate followed by CO2/O2/RH
// recovery mapping. The hardware carries probe masts, airflow-shadow witnesses,
// condensate-risk witnesses, alarm timing tokens, custody lands, and disposition
// gates. It does not define incubator recipes, biosafety claims, live-cell work,
// alarm limits, or operating procedures.

const OUTPUT_PREFIX: &str = "closed_incubator_gas_recovery_after_service_port_opening_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_containment_deck.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_service_port_surrogate_frame.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_cassette_load_surrogate.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_co2_o2_rh_probe_mast_pockets.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_airflow_vane_shadow_grid.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_condensate_witness_gutter.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_filtered_purge_inlet_holder.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_exhaust_return_witness_path.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_alarm_timer_token_rail.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_barcode_rfid_custody_lands.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_release_hold_reject_gates_camera_keepouts.stl",
    "output/closed_incubator_gas_recovery_after_service_port_opening_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "containment_deck",
    "service_port_surrogate_frame",
    "cassette_load_surrogate",
    "co2_o2_rh_probe_mast_pockets",
    "airflow_vane_shadow_grid",
    "condensate_witness_gutter",
    "filtered_purge_inlet_holder",
    "exhaust_return_witness_path",
    "alarm_timer_token_rail",
    "barcode_rfid_custody_lands",
    "release_hold_reject_gates",
    "evidence_camera_bridge_robot_service_keepouts",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 1080.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const REAR_RIM_Z: f64 = 64.0;
const FRONT_RIM_Z: f64 = 30.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 6;

const PORT_X: f64 = 390.0;
const PORT_Y: f64 = 340.0;
const PORT_Z: f64 = 56.0;
const PORT_POS: (f64, f64) = (-455.0, 185.0);
const PORT_OPENING_X: f64 = 255.0;
const PORT_OPENING_Y: f64 = 178.0;
const HINGE_WITNESS_COUNT: usize = 5;
const SEAL_WITNESS_COUNT: usize = 12;

const CASSETTE_X: f64 = 430.0;
const CASSETTE_Y: f64 = 290.0;
const CASSETTE_Z: f64 = 42.0;
const CASSETTE_POS: (f64, f64) = (20.0, 185.0);
const CASSETTE_BAY_ROWS: usize = 2;
const CASSETTE_BAY_COLS: usize = 3;
const CASSETTE_BAY_COUNT: usize = CASSETTE_BAY_ROWS * CASSETTE_BAY_COLS;
const CASSETTE_MASS_BLOCK_COUNT: usize = 6;

const PROBE_X: f64 = 450.0;
const PROBE_Y: f64 = 290.0;
const PROBE_Z: f64 = 58.0;
const PROBE_POS: (f64, f64) = (500.0, 185.0);
const PROBE_LEVELS: usize = 3;
const PROBE_PER_LEVEL: usize = 4;
const PROBE_SOCKET_COUNT: usize = PROBE_LEVELS * PROBE_PER_LEVEL;
const PROBE_SOCKET_D: f64 = 20.0;
const PROBE_MAST_HEIGHTS: [f64; PROBE_LEVELS] = [58.0, 118.0, 178.0];

const GRID_X: f64 = 620.0;
const GRID_Y: f64 = 212.0;
const GRID_Z: f64 = 34.0;
const GRID_POS: (f64, f64) = (-330.0, -125.0);
const VANE_COLS: usize = 9;
const VANE_ROWS: usize = 4;
const VANE_COUNT: usize = VANE_COLS * VANE_ROWS;
const SHADOW_FLAG_COUNT: usize = 9;

const GUTTER_X: f64 = 360.0;
const GUTTER_Y: f64 = 212.0;
const GUTTER_Z: f64 = 38.0;
const GUTTER_POS: (f64, f64) = (180.0, -95.0);
const GUTTER_CHANNEL_COUNT: usize = 5;
const WITNESS_COUPON_COUNT: usize = 10;

const PURGE_X: f64 = 270.0;
const PURGE_Y: f64 = 212.0;
const PURGE_Z: f64 = 44.0;
const PURGE_POS: (f64, f64) = (545.0, -95.0);
const FILTER_HOLDER_COUNT: usize = 3;
const PURGE_INLET_PORT_COUNT: usize = 3;

const RETURN_X: f64 = 520.0;
const RETURN_Y: f64 = 135.0;
const RETURN_Z: f64 = 34.0;
const RETURN_POS: (f64, f64) = (-380.0, -315.0);
const RETURN_WITNESS_SEGMENTS: usize = 8;
const EXHAUST_PLUME_FLAG_COUNT: usize = 6;

const TIMER_X: f64 = 360.0;
const TIMER_Y: f64 = 135.0;
const TIMER_Z: f64 = 28.0;
const TIMER_POS: (f64, f64) = (80.0, -315.0);
const TIMER_STAGE_COUNT: usize = 6;
const TOKENS_PER_STAGE: usize = 3;
const TIMER_TOKEN_COUNT: usize = TIMER_STAGE_COUNT * TOKENS_PER_STAGE;

const CUSTODY_X: f64 = 320.0;
const CUSTODY_Y: f64 = 135.0;
const CUSTODY_Z: f64 = 18.0;
const CUSTODY_POS: (f64, f64) = (500.0, -315.0);
const BARCODE_LAND_COUNT: usize = 5;
const RFID_LAND_COUNT: usize = 4;

const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 95.0;
const GATE_Z: f64 = 26.0;
const GATE_POS: (f64, f64) = (-405.0, -450.0);
const DISPOSITION_GATE_COUNT: usize = 3;
const CAMERA_BRIDGE_X: f64 = 900.0;
const CAMERA_BRIDGE_Y: f64 = 74.0;
const CAMERA_BRIDGE_Z: f64 = 146.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (310.0, -432.0);
const CAMERA_MOUNT_COUNT: usize = 4;
const KEEP_OUT_GAUGE_COUNT: usize = 7;

const ROBOT_FRONT_CLEARANCE: f64 = 34.0;
const SERVICE_REAR_CLEARANCE: f64 = 58.0;
const SIDE_SERVICE_CLEARANCE: f64 = 42.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 10.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 10.0
    }

    fn overlaps(self, other: Rect) -> bool {
        let x_overlap = (self.center.0 - other.center.0).abs() < (self.x + other.x) / 2.0 + 10.0;
        let y_overlap = (self.center.1 - other.center.1).abs() < (self.y + other.y) / 2.0 + 10.0;
        x_overlap && y_overlap
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Gate {
    Release,
    Hold,
    Reject,
}

impl Gate {
    fn all() -> [Gate; DISPOSITION_GATE_COUNT] {
        [Gate::Release, Gate::Hold, Gate::Reject]
    }

    fn index(self) -> usize {
        match self {
            Gate::Release => 0,
            Gate::Hold => 1,
            Gate::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Gate::Release => "release",
            Gate::Hold => "hold",
            Gate::Reject => "reject",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let port = service_port_surrogate_frame();
    export(OUTPUTS[1], &port);

    let cassette = cassette_load_surrogate();
    export(OUTPUTS[2], &cassette);

    let probes = co2_o2_rh_probe_mast_pockets();
    export(OUTPUTS[3], &probes);

    let grid = airflow_vane_shadow_grid();
    export(OUTPUTS[4], &grid);

    let gutter = condensate_witness_gutter();
    export(OUTPUTS[5], &gutter);

    let purge = filtered_purge_inlet_holder();
    export(OUTPUTS[6], &purge);

    let return_path = exhaust_return_witness_path();
    export(OUTPUTS[7], &return_path);

    let timer = alarm_timer_token_rail();
    export(OUTPUTS[8], &timer);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[9], &custody);

    let gates_bridge = release_hold_reject_gates_camera_keepouts();
    export(OUTPUTS[10], &gates_bridge);

    let assembly = deck
        + port.translate(PORT_POS.0, PORT_POS.1, insert_z(PORT_Z))
        + cassette.translate(CASSETTE_POS.0, CASSETTE_POS.1, insert_z(CASSETTE_Z))
        + probes.translate(PROBE_POS.0, PROBE_POS.1, insert_z(PROBE_Z))
        + grid.translate(GRID_POS.0, GRID_POS.1, insert_z(GRID_Z))
        + gutter.translate(GUTTER_POS.0, GUTTER_POS.1, insert_z(GUTTER_Z))
        + purge.translate(PURGE_POS.0, PURGE_POS.1, insert_z(PURGE_Z))
        + return_path.translate(RETURN_POS.0, RETURN_POS.1, insert_z(RETURN_Z))
        + timer.translate(TIMER_POS.0, TIMER_POS.1, insert_z(TIMER_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + gates_bridge.translate(0.0, 0.0, insert_z(GATE_Z));
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator gas recovery after service-port opening station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Opening surrogate:     {PORT_OPENING_X:.0}mm x {PORT_OPENING_Y:.0}mm service-port aperture, {HINGE_WITNESS_COUNT} hinge ticks, {SEAL_WITNESS_COUNT} seal witness tabs"
    );
    println!(
        "  Recovery measurement:  {PROBE_SOCKET_COUNT} CO2/O2/RH probe sockets across {PROBE_LEVELS} mast heights; recovery sample points {}",
        recovery_sample_points()
    );
    println!(
        "  Flow/condensate:       {VANE_COUNT} vane cells, {SHADOW_FLAG_COUNT} shadow flags, {GUTTER_CHANNEL_COUNT} gutter channels, {WITNESS_COUPON_COUNT} witness coupons"
    );
    println!(
        "  Purge/return:          {FILTER_HOLDER_COUNT} filtered purge holders, {PURGE_INLET_PORT_COUNT} inlet ports, {RETURN_WITNESS_SEGMENTS} return path witness segments"
    );
    println!(
        "  Decisions/custody:     {TIMER_TOKEN_COUNT} alarm timer tokens, {BARCODE_LAND_COUNT} barcode lands, {RFID_LAND_COUNT} RFID lands, {DISPOSITION_GATE_COUNT} disposition gates"
    );
    println!(
        "  Camera/keepouts:       {CAMERA_MOUNT_COUNT} camera mounts, {KEEP_OUT_GAUGE_COUNT} robot/service keepout gauges"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(CASSETTE_BAY_COUNT, CASSETTE_BAY_ROWS * CASSETTE_BAY_COLS);
    assert_eq!(PROBE_SOCKET_COUNT, PROBE_LEVELS * PROBE_PER_LEVEL);
    assert_eq!(VANE_COUNT, VANE_COLS * VANE_ROWS);
    assert_eq!(TIMER_TOKEN_COUNT, TIMER_STAGE_COUNT * TOKENS_PER_STAGE);
    assert_eq!(Gate::all().len(), DISPOSITION_GATE_COUNT);
    assert_eq!(mount_slot_positions().len(), MOUNT_SLOT_COUNT);
    assert_eq!(datum_target_positions().len(), DATUM_TARGET_COUNT);
    assert!(PROBE_SOCKET_D < PROBE_Z);
    assert!(PORT_OPENING_X < PORT_X - 70.0);
    assert!(PORT_OPENING_Y < PORT_Y - 70.0);
    assert!(condensate_capacity_ml() > expected_condensate_challenge_ml());
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
    assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);

    let rects = station_rects();
    for item in rects {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds containment deck",
            item.name
        );
    }
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn station_rects() -> [Rect; 10] {
    [
        rect("service_port_surrogate_frame", PORT_POS, PORT_X, PORT_Y),
        rect(
            "cassette_load_surrogate",
            CASSETTE_POS,
            CASSETTE_X,
            CASSETTE_Y,
        ),
        rect("co2_o2_rh_probe_mast_pockets", PROBE_POS, PROBE_X, PROBE_Y),
        rect("airflow_vane_shadow_grid", GRID_POS, GRID_X, GRID_Y),
        rect("condensate_witness_gutter", GUTTER_POS, GUTTER_X, GUTTER_Y),
        rect("filtered_purge_inlet_holder", PURGE_POS, PURGE_X, PURGE_Y),
        rect(
            "exhaust_return_witness_path",
            RETURN_POS,
            RETURN_X,
            RETURN_Y,
        ),
        rect("alarm_timer_token_rail", TIMER_POS, TIMER_X, TIMER_Y),
        rect(
            "barcode_rfid_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect("release_hold_reject_gates", GATE_POS, GATE_X, GATE_Y),
    ]
}

fn recovery_sample_points() -> usize {
    PROBE_SOCKET_COUNT + VANE_COUNT + RETURN_WITNESS_SEGMENTS + GUTTER_CHANNEL_COUNT
}

fn condensate_capacity_ml() -> f64 {
    GUTTER_CHANNEL_COUNT as f64 * (GUTTER_X - 84.0) * 14.0 * 12.0 / 1000.0
}

fn expected_condensate_challenge_ml() -> f64 {
    WITNESS_COUPON_COUNT as f64 * 8.0 + FILTER_HOLDER_COUNT as f64 * 14.0
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (GATE_POS.1.abs() + GATE_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (PORT_POS.1 + PORT_Y / 2.0)
}

fn side_service_clearance() -> f64 {
    STATION_X / 2.0 - (PURGE_POS.0 + PURGE_X / 2.0)
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "gas_recovery_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let shallow_sump = centered_cube(
        "gas_recovery_containment_low_condensate_sump_cut",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        8.0,
    )
    .translate(0.0, -22.0, DECK_Z - 4.0);
    let drain_bore = centered_cylinder("gas_recovery_condensate_deck_drain_bore", 7.0, 52.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            STATION_X / 2.0 - 86.0,
            -STATION_Y / 2.0 + 30.0,
            DECK_Z - 8.0,
        );

    deck - shallow_sump - drain_bore - deck_insert_sockets() - deck_mount_slots()
        + perimeter_rims()
        + deck_flow_axis_spines()
        + datum_targets()
        + service_opening_angle_ticks()
}

fn deck_insert_sockets() -> Part {
    let mut sockets = Part::empty("gas_recovery_deck_insert_sockets");
    for item in station_rects() {
        sockets = sockets
            + centered_cube(
                format!("gas_recovery_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("gas_recovery_deck_mount_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("gas_recovery_m6_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("gas_recovery_m6_slot_relief_{i}"),
                30.0,
                7.4,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (-STATION_X / 2.0 + 62.0, 0.0),
        (STATION_X / 2.0 - 62.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "gas_recovery_low_front_robot_access_rim",
        STATION_X,
        RIM_W,
        FRONT_RIM_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + FRONT_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "gas_recovery_tall_rear_service_opening_rim",
        STATION_X,
        RIM_W,
        REAR_RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + REAR_RIM_Z / 2.0,
    );
    let left = centered_cube("gas_recovery_left_spill_rim", RIM_W, STATION_Y, REAR_RIM_Z)
        .translate(
            -STATION_X / 2.0 + RIM_W / 2.0,
            0.0,
            DECK_Z + REAR_RIM_Z / 2.0,
        );
    let right = centered_cube("gas_recovery_right_spill_rim", RIM_W, STATION_Y, REAR_RIM_Z)
        .translate(
            STATION_X / 2.0 - RIM_W / 2.0,
            0.0,
            DECK_Z + REAR_RIM_Z / 2.0,
        );
    front + rear + left + right
}

fn deck_flow_axis_spines() -> Part {
    let service_to_grid = centered_cube(
        "gas_recovery_service_port_to_vane_flow_axis",
        16.0,
        575.0,
        20.0,
    )
    .rotate(0.0, 0.0, -8.0)
    .translate(-380.0, 38.0, DECK_Z + 10.0);
    let cassette_to_probe = centered_cube(
        "gas_recovery_cassette_to_probe_recovery_axis",
        610.0,
        12.0,
        20.0,
    )
    .translate(260.0, 38.0, DECK_Z + 10.0);
    let purge_to_return = centered_cube(
        "gas_recovery_purge_to_return_witness_axis",
        1160.0,
        10.0,
        18.0,
    )
    .translate(20.0, -215.0, DECK_Z + 9.0);
    let disposition_spine =
        centered_cube("gas_recovery_alarm_to_disposition_spine", 1040.0, 8.0, 16.0).translate(
            40.0,
            -382.0,
            DECK_Z + 8.0,
        );
    service_to_grid + cassette_to_probe + purge_to_return + disposition_spine
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("gas_recovery_robot_camera_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("gas_recovery_datum_target_{i}")).translate(
                x,
                y,
                DECK_Z + 2.5,
            );
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 105.0, -STATION_Y / 2.0 + 105.0),
        (STATION_X / 2.0 - 105.0, -STATION_Y / 2.0 + 105.0),
        (-STATION_X / 2.0 + 105.0, STATION_Y / 2.0 - 105.0),
        (STATION_X / 2.0 - 105.0, STATION_Y / 2.0 - 105.0),
        (-88.0, 18.0),
        (590.0, 20.0),
    ]
}

fn service_opening_angle_ticks() -> Part {
    let mut ticks = Part::empty("gas_recovery_service_opening_angle_ticks");
    for i in 0..7 {
        let angle = -38.0 + i as f64 * 12.0;
        ticks = ticks
            + centered_cube(
                format!("gas_recovery_access_panel_angle_tick_{i}"),
                54.0,
                6.0,
                9.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(
                -645.0 + i as f64 * 17.0,
                22.0 + i as f64 * 26.0,
                DECK_Z + 4.5,
            );
    }
    ticks
}

fn service_port_surrogate_frame() -> Part {
    let body = centered_cube(
        "gas_recovery_service_port_surrogate_frame_body",
        PORT_X,
        PORT_Y,
        PORT_Z,
    );
    let aperture = centered_cube(
        "gas_recovery_service_port_opening_aperture_cut",
        PORT_OPENING_X,
        PORT_OPENING_Y,
        PORT_Z + 2.0,
    );
    let rear_door_plane = centered_cube(
        "gas_recovery_access_panel_open_position_plane",
        PORT_OPENING_X + 38.0,
        14.0,
        124.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(-22.0, PORT_Y / 2.0 - 34.0, PORT_Z / 2.0 + 62.0);

    body - aperture - frame_probe_edge_reliefs()
        + rear_door_plane
        + hinge_witness_ticks()
        + seal_witness_tabs()
        + opening_event_token_pockets()
}

fn frame_probe_edge_reliefs() -> Part {
    let mut reliefs = Part::empty("gas_recovery_service_port_probe_edge_reliefs");
    for i in 0..4 {
        reliefs = reliefs
            + centered_cylinder(
                format!("gas_recovery_service_port_edge_probe_relief_{i}"),
                7.0,
                PORT_Z + 2.0,
                24,
            )
            .translate(centered_index(i, 4, 58.0), -PORT_Y / 2.0 + 28.0, 0.0);
    }
    reliefs
}

fn hinge_witness_ticks() -> Part {
    let mut ticks = Part::empty("gas_recovery_service_port_hinge_witness_ticks");
    for i in 0..HINGE_WITNESS_COUNT {
        ticks = ticks
            + centered_cube(
                format!("gas_recovery_hinge_angle_witness_tick_{i}"),
                12.0,
                44.0,
                16.0,
            )
            .rotate(0.0, 0.0, -20.0 + i as f64 * 10.0)
            .translate(
                -PORT_X / 2.0 + 34.0,
                centered_index(i, HINGE_WITNESS_COUNT, 44.0),
                PORT_Z / 2.0 + 8.0,
            );
    }
    ticks
}

fn seal_witness_tabs() -> Part {
    let mut tabs = Part::empty("gas_recovery_service_port_seal_witness_tabs");
    for i in 0..SEAL_WITNESS_COUNT {
        let top = i < SEAL_WITNESS_COUNT / 2;
        let index = if top { i } else { i - SEAL_WITNESS_COUNT / 2 };
        let y = if top {
            PORT_Y / 2.0 - 22.0
        } else {
            -PORT_Y / 2.0 + 22.0
        };
        tabs = tabs
            + centered_cube(
                format!("gas_recovery_seal_witness_tab_{i}"),
                28.0,
                10.0,
                18.0,
            )
            .translate(
                centered_index(index, SEAL_WITNESS_COUNT / 2, 48.0),
                y,
                PORT_Z / 2.0 + 9.0,
            );
    }
    tabs
}

fn opening_event_token_pockets() -> Part {
    let mut tokens = Part::empty("gas_recovery_opening_event_token_pockets");
    for i in 0..4 {
        tokens = tokens
            + centered_cube(
                format!("gas_recovery_access_open_duration_token_{i}"),
                38.0,
                24.0,
                10.0,
            )
            .translate(
                PORT_X / 2.0 - 52.0,
                centered_index(i, 4, 44.0),
                PORT_Z / 2.0 + 5.0,
            );
    }
    tokens
}

fn cassette_load_surrogate() -> Part {
    let tray = centered_cube(
        "gas_recovery_cassette_load_surrogate_tray",
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    tray - cassette_bay_cuts()
        + cassette_mass_blocks()
        + cassette_shadow_shims()
        + cassette_robot_grip_lands()
}

fn cassette_bay_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_cassette_bay_cuts");
    for row in 0..CASSETTE_BAY_ROWS {
        for col in 0..CASSETTE_BAY_COLS {
            let index = row * CASSETTE_BAY_COLS + col;
            cuts = cuts
                + centered_cube(
                    format!("gas_recovery_cassette_bay_{index}_clearance_cut"),
                    92.0,
                    82.0,
                    16.0,
                )
                .translate(
                    centered_index(col, CASSETTE_BAY_COLS, 116.0),
                    centered_index(row, CASSETTE_BAY_ROWS, 112.0),
                    CASSETTE_Z / 2.0 - 8.0,
                );
        }
    }
    cuts
}

fn cassette_mass_blocks() -> Part {
    let mut blocks = Part::empty("gas_recovery_cassette_load_mass_blocks");
    for i in 0..CASSETTE_MASS_BLOCK_COUNT {
        blocks = blocks
            + centered_cube(
                format!("gas_recovery_cassette_mass_block_{i}"),
                70.0,
                46.0,
                24.0,
            )
            .translate(
                centered_index(i % 3, 3, 118.0),
                centered_index(i / 3, 2, 110.0),
                CASSETTE_Z / 2.0 + 12.0,
            );
    }
    blocks
}

fn cassette_shadow_shims() -> Part {
    let mut shims = Part::empty("gas_recovery_cassette_airflow_shadow_shims");
    for i in 0..5 {
        shims = shims
            + centered_cube(
                format!("gas_recovery_cassette_shadow_step_shim_{i}"),
                18.0,
                220.0,
                10.0 + i as f64 * 3.0,
            )
            .translate(
                centered_index(i, 5, 54.0),
                0.0,
                CASSETTE_Z / 2.0 + 5.0 + i as f64 * 1.5,
            );
    }
    shims
}

fn cassette_robot_grip_lands() -> Part {
    let front = centered_cube(
        "gas_recovery_cassette_front_robot_grip_land",
        120.0,
        18.0,
        20.0,
    )
    .translate(0.0, -CASSETTE_Y / 2.0 + 24.0, CASSETTE_Z / 2.0 + 10.0);
    let rear = centered_cube(
        "gas_recovery_cassette_rear_robot_grip_land",
        120.0,
        18.0,
        20.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 - 24.0, CASSETTE_Z / 2.0 + 10.0);
    front + rear
}

fn co2_o2_rh_probe_mast_pockets() -> Part {
    let base = centered_cube(
        "gas_recovery_probe_mast_pocket_base",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    );
    base - probe_socket_cuts() - cable_trough_cuts() + probe_masts() + gas_layer_reference_steps()
}

fn probe_socket_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_probe_socket_cuts");
    for level in 0..PROBE_LEVELS {
        for sensor in 0..PROBE_PER_LEVEL {
            let index = level * PROBE_PER_LEVEL + sensor;
            cuts = cuts
                + centered_cylinder(
                    format!("gas_recovery_co2_o2_rh_probe_socket_cut_{index}"),
                    PROBE_SOCKET_D / 2.0,
                    PROBE_Z + 2.0,
                    30,
                )
                .translate(
                    centered_index(sensor, PROBE_PER_LEVEL, 76.0),
                    centered_index(level, PROBE_LEVELS, 78.0),
                    0.0,
                );
        }
    }
    cuts
}

fn cable_trough_cuts() -> Part {
    let mut troughs = Part::empty("gas_recovery_probe_cable_trough_cuts");
    for level in 0..PROBE_LEVELS {
        troughs = troughs
            + centered_cube(
                format!("gas_recovery_probe_level_{level}_cable_trough_cut"),
                PROBE_X - 72.0,
                9.0,
                12.0,
            )
            .translate(
                0.0,
                centered_index(level, PROBE_LEVELS, 78.0),
                PROBE_Z / 2.0 - 6.0,
            );
    }
    troughs
}

fn probe_masts() -> Part {
    let mut masts = Part::empty("gas_recovery_probe_mast_height_posts");
    for level in 0..PROBE_LEVELS {
        for sensor in 0..PROBE_PER_LEVEL {
            let index = level * PROBE_PER_LEVEL + sensor;
            let height = PROBE_MAST_HEIGHTS[level];
            masts = masts
                + centered_cylinder(
                    format!("gas_recovery_probe_mast_post_{index}"),
                    7.0,
                    height,
                    24,
                )
                .translate(
                    centered_index(sensor, PROBE_PER_LEVEL, 76.0),
                    centered_index(level, PROBE_LEVELS, 78.0),
                    PROBE_Z / 2.0 + height / 2.0,
                );
        }
    }
    masts
}

fn gas_layer_reference_steps() -> Part {
    let mut steps = Part::empty("gas_recovery_gas_stratification_reference_steps");
    for i in 0..PROBE_LEVELS {
        steps = steps
            + centered_cube(
                format!("gas_recovery_layer_height_reference_step_{i}"),
                58.0,
                30.0,
                10.0 + i as f64 * 8.0,
            )
            .translate(
                -PROBE_X / 2.0 + 50.0,
                centered_index(i, PROBE_LEVELS, 76.0),
                PROBE_Z / 2.0 + 5.0 + i as f64 * 4.0,
            );
    }
    steps
}

fn airflow_vane_shadow_grid() -> Part {
    let frame = centered_cube(
        "gas_recovery_airflow_vane_grid_frame",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    frame - vane_window_cuts() + vane_blades() + shadow_flags() + stagnation_witness_dots()
}

fn vane_window_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_vane_grid_window_cuts");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let index = row * VANE_COLS + col;
            cuts = cuts
                + centered_cube(
                    format!("gas_recovery_vane_window_cut_{index}"),
                    42.0,
                    32.0,
                    GRID_Z + 2.0,
                )
                .translate(
                    centered_index(col, VANE_COLS, 58.0),
                    centered_index(row, VANE_ROWS, 43.0),
                    0.0,
                );
        }
    }
    cuts
}

fn vane_blades() -> Part {
    let mut blades = Part::empty("gas_recovery_airflow_vane_blades");
    for col in 0..VANE_COLS {
        let angle = -18.0 + (col % 5) as f64 * 9.0;
        blades = blades
            + centered_cube(
                format!("gas_recovery_directional_vane_blade_{col}"),
                8.0,
                GRID_Y - 36.0,
                42.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(
                centered_index(col, VANE_COLS, 58.0),
                0.0,
                GRID_Z / 2.0 + 21.0,
            );
    }
    blades
}

fn shadow_flags() -> Part {
    let mut flags = Part::empty("gas_recovery_airflow_shadow_flags");
    for i in 0..SHADOW_FLAG_COUNT {
        flags = flags
            + centered_cube(format!("gas_recovery_shadow_flag_{i}"), 30.0, 8.0, 58.0).translate(
                centered_index(i, SHADOW_FLAG_COUNT, 60.0),
                GRID_Y / 2.0 - 22.0,
                GRID_Z / 2.0 + 29.0,
            );
    }
    flags
}

fn stagnation_witness_dots() -> Part {
    let mut dots = Part::empty("gas_recovery_stagnation_witness_dots");
    for i in 0..8 {
        dots = dots
            + centered_cylinder(format!("gas_recovery_stagnation_dot_{i}"), 6.0, 5.0, 24)
                .translate(
                    centered_index(i, 8, 64.0),
                    -GRID_Y / 2.0 + 30.0,
                    GRID_Z / 2.0 + 2.5,
                );
    }
    dots
}

fn condensate_witness_gutter() -> Part {
    let body = centered_cube(
        "gas_recovery_condensate_witness_gutter_body",
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    );
    body - gutter_channel_cuts() + witness_coupon_lands() + dewpoint_risk_lips()
}

fn gutter_channel_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_condensate_gutter_channel_cuts");
    for i in 0..GUTTER_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("gas_recovery_condensate_channel_cut_{i}"),
                GUTTER_X - 84.0,
                14.0,
                13.0,
            )
            .rotate(0.0, 0.0, -3.0)
            .translate(
                0.0,
                centered_index(i, GUTTER_CHANNEL_COUNT, 30.0),
                GUTTER_Z / 2.0 - 6.5,
            );
    }
    cuts
}

fn witness_coupon_lands() -> Part {
    let mut lands = Part::empty("gas_recovery_condensate_witness_coupon_lands");
    for i in 0..WITNESS_COUPON_COUNT {
        lands = lands
            + centered_cube(
                format!("gas_recovery_condensate_witness_coupon_{i}"),
                38.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(i % 5, 5, 60.0),
                centered_index(i / 5, 2, 126.0),
                GUTTER_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn dewpoint_risk_lips() -> Part {
    let cold_lip = centered_cube(
        "gas_recovery_cold_edge_condensate_risk_lip",
        GUTTER_X - 42.0,
        12.0,
        26.0,
    )
    .translate(0.0, GUTTER_Y / 2.0 - 22.0, GUTTER_Z / 2.0 + 13.0);
    let return_lip = centered_cube(
        "gas_recovery_return_edge_condensate_risk_lip",
        GUTTER_X - 42.0,
        12.0,
        18.0,
    )
    .translate(0.0, -GUTTER_Y / 2.0 + 22.0, GUTTER_Z / 2.0 + 9.0);
    cold_lip + return_lip
}

fn filtered_purge_inlet_holder() -> Part {
    let base = centered_cube(
        "gas_recovery_filtered_purge_inlet_holder_base",
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    );
    base - purge_port_cuts() + filter_cradles() + inlet_flow_arrows()
}

fn purge_port_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_purge_inlet_port_cuts");
    for i in 0..PURGE_INLET_PORT_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("gas_recovery_filtered_purge_inlet_port_cut_{i}"),
                12.0,
                PURGE_Z + 2.0,
                32,
            )
            .translate(
                centered_index(i, PURGE_INLET_PORT_COUNT, 68.0),
                -PURGE_Y / 2.0 + 58.0,
                0.0,
            );
    }
    cuts
}

fn filter_cradles() -> Part {
    let mut cradles = Part::empty("gas_recovery_purge_filter_cradles");
    for i in 0..FILTER_HOLDER_COUNT {
        cradles = cradles
            + centered_cylinder(
                format!("gas_recovery_filter_capsule_cradle_{i}"),
                18.0,
                76.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, FILTER_HOLDER_COUNT, 68.0),
                16.0,
                PURGE_Z / 2.0 + 18.0,
            )
            + centered_cube(
                format!("gas_recovery_filter_retainer_strap_{i}"),
                46.0,
                8.0,
                34.0,
            )
            .translate(
                centered_index(i, FILTER_HOLDER_COUNT, 68.0),
                16.0,
                PURGE_Z / 2.0 + 17.0,
            );
    }
    cradles
}

fn inlet_flow_arrows() -> Part {
    let mut arrows = Part::empty("gas_recovery_purge_inlet_flow_arrow_blocks");
    for i in 0..PURGE_INLET_PORT_COUNT {
        arrows = arrows
            + centered_cube(
                format!("gas_recovery_purge_flow_arrow_stem_{i}"),
                8.0,
                62.0,
                8.0,
            )
            .translate(
                centered_index(i, PURGE_INLET_PORT_COUNT, 68.0),
                -16.0,
                PURGE_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("gas_recovery_purge_flow_arrow_head_{i}"),
                24.0,
                18.0,
                8.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(
                centered_index(i, PURGE_INLET_PORT_COUNT, 68.0),
                20.0,
                PURGE_Z / 2.0 + 4.0,
            );
    }
    arrows
}

fn exhaust_return_witness_path() -> Part {
    let base = centered_cube(
        "gas_recovery_exhaust_return_witness_path_base",
        RETURN_X,
        RETURN_Y,
        RETURN_Z,
    );
    base - return_channel_cut() + return_witness_segments() + exhaust_plume_flags()
}

fn return_channel_cut() -> Part {
    centered_cube(
        "gas_recovery_exhaust_return_serpentine_channel_cut",
        RETURN_X - 64.0,
        28.0,
        12.0,
    )
    .translate(0.0, 0.0, RETURN_Z / 2.0 - 6.0)
}

fn return_witness_segments() -> Part {
    let mut segments = Part::empty("gas_recovery_return_witness_segments");
    for i in 0..RETURN_WITNESS_SEGMENTS {
        segments = segments
            + centered_cube(
                format!("gas_recovery_return_path_witness_segment_{i}"),
                36.0,
                22.0,
                8.0,
            )
            .translate(
                centered_index(i, RETURN_WITNESS_SEGMENTS, 52.0),
                -RETURN_Y / 2.0 + 34.0 + (i % 2) as f64 * 66.0,
                RETURN_Z / 2.0 + 4.0,
            );
    }
    segments
}

fn exhaust_plume_flags() -> Part {
    let mut flags = Part::empty("gas_recovery_exhaust_plume_flags");
    for i in 0..EXHAUST_PLUME_FLAG_COUNT {
        flags = flags
            + centered_cube(
                format!("gas_recovery_exhaust_plume_flag_{i}"),
                8.0,
                38.0,
                48.0,
            )
            .rotate(0.0, 0.0, -12.0 + i as f64 * 5.0)
            .translate(
                centered_index(i, EXHAUST_PLUME_FLAG_COUNT, 70.0),
                RETURN_Y / 2.0 - 26.0,
                RETURN_Z / 2.0 + 24.0,
            );
    }
    flags
}

fn alarm_timer_token_rail() -> Part {
    let base = centered_cube(
        "gas_recovery_alarm_timer_token_rail_base",
        TIMER_X,
        TIMER_Y,
        TIMER_Z,
    );
    base - timer_stage_pocket_cuts() + timer_tokens() + alarm_threshold_flags()
}

fn timer_stage_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gas_recovery_alarm_timer_stage_pocket_cuts");
    for i in 0..TIMER_STAGE_COUNT {
        cuts = cuts
            + centered_cube(
                format!("gas_recovery_timer_stage_pocket_cut_{i}"),
                42.0,
                86.0,
                10.0,
            )
            .translate(
                centered_index(i, TIMER_STAGE_COUNT, 52.0),
                0.0,
                TIMER_Z / 2.0 - 5.0,
            );
    }
    cuts
}

fn timer_tokens() -> Part {
    let mut tokens = Part::empty("gas_recovery_alarm_timer_tokens");
    for stage in 0..TIMER_STAGE_COUNT {
        for token in 0..TOKENS_PER_STAGE {
            let index = stage * TOKENS_PER_STAGE + token;
            tokens = tokens
                + centered_cylinder(
                    format!("gas_recovery_alarm_timer_token_{index}"),
                    8.0,
                    6.0,
                    24,
                )
                .translate(
                    centered_index(stage, TIMER_STAGE_COUNT, 52.0),
                    centered_index(token, TOKENS_PER_STAGE, 24.0),
                    TIMER_Z / 2.0 + 3.0,
                );
        }
    }
    tokens
}

fn alarm_threshold_flags() -> Part {
    let early = centered_cube(
        "gas_recovery_alarm_early_threshold_flag",
        8.0,
        TIMER_Y - 28.0,
        34.0,
    )
    .translate(
        centered_index(1, TIMER_STAGE_COUNT, 52.0),
        0.0,
        TIMER_Z / 2.0 + 17.0,
    );
    let target = centered_cube(
        "gas_recovery_alarm_target_recovery_threshold_flag",
        8.0,
        TIMER_Y - 28.0,
        48.0,
    )
    .translate(
        centered_index(3, TIMER_STAGE_COUNT, 52.0),
        0.0,
        TIMER_Z / 2.0 + 24.0,
    );
    let late = centered_cube(
        "gas_recovery_alarm_late_recovery_threshold_flag",
        8.0,
        TIMER_Y - 28.0,
        62.0,
    )
    .translate(
        centered_index(5, TIMER_STAGE_COUNT, 52.0),
        0.0,
        TIMER_Z / 2.0 + 31.0,
    );
    early + target + late
}

fn barcode_rfid_custody_lands() -> Part {
    let base = centered_cube(
        "gas_recovery_barcode_rfid_custody_land_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    base + barcode_lands() + rfid_lands() + custody_chain_token_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("gas_recovery_barcode_custody_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(format!("gas_recovery_barcode_land_{i}"), 48.0, 26.0, 5.0).translate(
                centered_index(i, BARCODE_LAND_COUNT, 54.0),
                -CUSTODY_Y / 2.0 + 32.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("gas_recovery_rfid_custody_lands");
    for i in 0..RFID_LAND_COUNT {
        lands = lands
            + centered_cube(format!("gas_recovery_rfid_land_{i}"), 44.0, 38.0, 5.0).translate(
                centered_index(i, RFID_LAND_COUNT, 62.0),
                CUSTODY_Y / 2.0 - 34.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn custody_chain_token_slots() -> Part {
    let mut slots = Part::empty("gas_recovery_custody_chain_token_slots");
    for i in 0..6 {
        slots = slots
            + centered_cylinder(format!("gas_recovery_custody_token_land_{i}"), 9.0, 5.0, 24)
                .translate(centered_index(i, 6, 44.0), 0.0, CUSTODY_Z / 2.0 + 2.5);
    }
    slots
}

fn release_hold_reject_gates_camera_keepouts() -> Part {
    let gates = disposition_gates().translate(GATE_POS.0, GATE_POS.1, 0.0);
    let bridge = evidence_camera_bridge().translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        CAMERA_BRIDGE_Z / 2.0 - GATE_Z / 2.0,
    );
    let keepouts = robot_service_keepout_gauges();
    gates + bridge + keepouts
}

fn disposition_gates() -> Part {
    let base = centered_cube(
        "gas_recovery_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = Part::empty("gas_recovery_release_hold_reject_gate_posts");
    for gate in Gate::all() {
        let index = gate.index();
        gates = gates
            + centered_cube(
                format!("gas_recovery_{}_gate_vertical_paddle", gate.name()),
                70.0,
                14.0,
                58.0 + index as f64 * 12.0,
            )
            .translate(
                centered_index(index, DISPOSITION_GATE_COUNT, 126.0),
                0.0,
                GATE_Z / 2.0 + 29.0 + index as f64 * 6.0,
            )
            + centered_cube(
                format!("gas_recovery_{}_gate_token_shelf", gate.name()),
                86.0,
                36.0,
                8.0,
            )
            .translate(
                centered_index(index, DISPOSITION_GATE_COUNT, 126.0),
                -28.0,
                GATE_Z / 2.0 + 4.0,
            );
    }
    base + gates
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "gas_recovery_camera_bridge_left_post",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 36.0, 0.0, 0.0);
    let right_post = centered_cube(
        "gas_recovery_camera_bridge_right_post",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 36.0, 0.0, 0.0);
    let beam = centered_cube(
        "gas_recovery_camera_bridge_overhead_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 14.0);
    left_post + right_post + beam + camera_mounts()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("gas_recovery_evidence_camera_mounts");
    for i in 0..CAMERA_MOUNT_COUNT {
        mounts = mounts
            + centered_cube(
                format!("gas_recovery_evidence_camera_mount_{i}"),
                58.0,
                42.0,
                18.0,
            )
            .translate(
                centered_index(i, CAMERA_MOUNT_COUNT, 210.0),
                0.0,
                CAMERA_BRIDGE_Z / 2.0 + 9.0,
            )
            + centered_cylinder(
                format!("gas_recovery_evidence_camera_lens_bore_{i}"),
                8.0,
                20.0,
                24,
            )
            .translate(
                centered_index(i, CAMERA_MOUNT_COUNT, 210.0),
                0.0,
                CAMERA_BRIDGE_Z / 2.0 + 20.0,
            );
    }
    mounts
}

fn robot_service_keepout_gauges() -> Part {
    let mut gauges = Part::empty("gas_recovery_robot_service_keepout_gauges");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        let x = centered_index(i, KEEP_OUT_GAUGE_COUNT, 132.0) + 300.0;
        gauges = gauges
            + centered_cube(
                format!("gas_recovery_robot_service_keepout_gauge_{i}"),
                16.0,
                70.0,
                46.0,
            )
            .translate(x, CAMERA_BRIDGE_POS.1 - 8.0, 23.0);
    }
    let robot_front = centered_cube(
        "gas_recovery_front_robot_service_keepout_bar",
        STATION_X - 180.0,
        8.0,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 46.0, 14.0);
    let rear_service = centered_cube(
        "gas_recovery_rear_access_panel_service_keepout_bar",
        STATION_X - 180.0,
        8.0,
        36.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 54.0, 18.0);
    gauges + robot_front + rear_service
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer"), 14.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_cut"), 4.0, 4.0, 24)
        + centered_cube(format!("{name}_cross_x"), 22.0, 3.0, 3.2)
        + centered_cube(format!("{name}_cross_y"), 3.0, 22.0, 3.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_are_prefixed_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn feature_counts_cover_design_intent() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert_eq!(PROBE_SOCKET_COUNT, 12);
        assert_eq!(VANE_COUNT, 36);
        assert_eq!(TIMER_TOKEN_COUNT, 18);
        assert_eq!(Gate::all().len(), 3);
    }

    #[test]
    fn inserts_fit_without_overlap() {
        let rects = station_rects();
        assert!(rects.iter().all(|rect| rect.fits_inside_deck()));
        for a in 0..rects.len() {
            for b in (a + 1)..rects.len() {
                assert!(
                    !rects[a].overlaps(rects[b]),
                    "{} overlaps {}",
                    rects[a].name,
                    rects[b].name
                );
            }
        }
    }

    #[test]
    fn condensate_and_recovery_metadata_are_conservative() {
        assert!(recovery_sample_points() >= 60);
        assert!(condensate_capacity_ml() > expected_condensate_challenge_ml());
        assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
        assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
        assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);
    }

    #[test]
    fn design_constraints_hold() {
        assert_design_constraints();
    }
}
