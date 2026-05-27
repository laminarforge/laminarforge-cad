use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator CO2/O2 gas-sampling line condensate-trap validation station.
//
// This standalone CAD generator models a no-cell, closed-system validation
// fixture for checking sample-line routing, condensate capture, hydrophobic
// filter placement, pressure/flow witness points, wetness/color indicators,
// environmental logger pockets, purge/recovery tokens, custody labeling,
// release/hold/reject disposition, camera evidence, and robot/service keepouts.
// Purchased sensors, filters, tubing, camera hardware, and certificate records
// are represented as mechanical envelopes only.

const OUTPUT_PREFIX: &str = "closed_incubator_gas_sampling_line_condensate_trap_validation_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_containment_deck.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_sample_line_routing_coupons.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_condensate_trap_cartridge_envelope.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_hydrophobic_filter_holder.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_pressure_flow_witness_ports.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_wetness_color_witness_pads.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_temperature_rh_logger_pockets.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_purge_recovery_token_rail.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_barcode_coa_custody_lands.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_release_hold_reject_gates.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_camera_evidence_bridge.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sample_line_routing_coupons",
    "condensate_trap_cartridge_envelope",
    "hydrophobic_filter_holder",
    "pressure_flow_witness_ports",
    "wetness_color_witness_pads",
    "temperature_rh_logger_pockets",
    "purge_recovery_token_rail",
    "barcode_coa_custody_lands",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1520.0;
const STATION_Y: f64 = 1000.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 54.0;
const BASIN_RECESS_Z: f64 = 9.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLES: usize = 8;
const DATUM_TARGETS: usize = 4;
const LEAK_SENSOR_WELLS: usize = 6;

const ROUTE_X: f64 = 560.0;
const ROUTE_Y: f64 = 180.0;
const ROUTE_Z: f64 = 44.0;
const ROUTE_POS: (f64, f64) = (-430.0, 250.0);
const ROUTE_COUPONS: usize = 8;
const ROUTE_COUPON_COLS: usize = 4;
const ROUTE_COUPON_ROWS: usize = 2;
const ROUTE_COUPON_X: f64 = 104.0;
const ROUTE_COUPON_Y: f64 = 56.0;
const ROUTE_COUPON_Z: f64 = 12.0;
const SAMPLE_LINE_OD: f64 = 6.4;
const ROUTE_WITNESS_CLIPS_PER_COUPON: usize = 2;

const TRAP_X: f64 = 420.0;
const TRAP_Y: f64 = 220.0;
const TRAP_Z: f64 = 74.0;
const TRAP_POS: (f64, f64) = (230.0, 260.0);
const TRAP_CARTRIDGES: usize = 2;
const TRAP_CARTRIDGE_OD: f64 = 52.0;
const TRAP_CARTRIDGE_LEN: f64 = 230.0;
const TRAP_BOWL_OD: f64 = 68.0;
const TRAP_CLAMPS: usize = 4;
const TRAP_LOW_POINT_CUPS: usize = 2;

const FILTER_X: f64 = 360.0;
const FILTER_Y: f64 = 150.0;
const FILTER_Z: f64 = 66.0;
const FILTER_POS: (f64, f64) = (505.0, 55.0);
const HYDROPHOBIC_FILTERS: usize = 2;
const FILTER_OD: f64 = 36.0;
const FILTER_LEN: f64 = 168.0;
const FILTER_CLAMPS: usize = 4;
const FILTER_BULKHEADS: usize = 4;

const PORT_X: f64 = 560.0;
const PORT_Y: f64 = 150.0;
const PORT_Z: f64 = 58.0;
const PORT_POS: (f64, f64) = (-430.0, 40.0);
const PRESSURE_PORTS: usize = 4;
const FLOW_WITNESS_PORTS: usize = 4;
const SENSOR_POCKETS: usize = 4;
const PORT_ROWS: usize = 2;
const PORT_COLS: usize = 4;
const PORT_D: f64 = 12.0;

const WETNESS_X: f64 = 560.0;
const WETNESS_Y: f64 = 160.0;
const WETNESS_Z: f64 = 32.0;
const WETNESS_POS: (f64, f64) = (-430.0, -170.0);
const WETNESS_PADS: usize = 12;
const WETNESS_PAD_COLS: usize = 6;
const WETNESS_PAD_ROWS: usize = 2;
const COLOR_REFERENCE_PADS: usize = 6;
const PAD_WELL_X: f64 = 48.0;
const PAD_WELL_Y: f64 = 38.0;

const LOGGER_X: f64 = 360.0;
const LOGGER_Y: f64 = 170.0;
const LOGGER_Z: f64 = 46.0;
const LOGGER_POS: (f64, f64) = (110.0, -155.0);
const TEMP_RH_LOGGERS: usize = 6;
const LOGGER_COLS: usize = 3;
const LOGGER_ROWS: usize = 2;
const LOGGER_POCKET_X: f64 = 76.0;
const LOGGER_POCKET_Y: f64 = 52.0;

const TOKEN_X: f64 = 330.0;
const TOKEN_Y: f64 = 150.0;
const TOKEN_Z: f64 = 36.0;
const TOKEN_POS: (f64, f64) = (505.0, -165.0);
const PURGE_TOKENS: usize = 6;
const RECOVERY_TOKENS: usize = 6;
const TOKEN_NEST_D: f64 = 28.0;

const CUSTODY_X: f64 = 460.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (-430.0, -385.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_POSTS: usize = 4;

const GATE_X: f64 = 500.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 28.0;
const GATE_POS: (f64, f64) = (200.0, -385.0);
const DISPOSITION_GATES: usize = 3;
const DISPOSITION_LANES: [&str; DISPOSITION_GATES] = ["release", "hold", "reject"];
const TOKENS_PER_GATE: usize = 4;

const CAMERA_SPAN_X: f64 = 1320.0;
const CAMERA_Y: f64 = 86.0;
const CAMERA_POST_Z: f64 = 238.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const CAMERA_POST_X: f64 = 32.0;
const CAMERA_POST_Y: f64 = 46.0;
const CAMERA_POS: (f64, f64) = (0.0, -10.0);
const CAMERA_MOUNTS: usize = 4;
const LIGHT_BARS: usize = 3;
const VIEW_FIELD_RIBS: usize = 6;

const KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 52.0;
const REAR_TRAP_SERVICE_CLEARANCE: f64 = 120.0;
const RIGHT_FILTER_SERVICE_CLEARANCE: f64 = 70.0;
const LEFT_LINE_SERVICE_CLEARANCE: f64 = 48.0;
const TOP_TRAP_LIFT_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 16.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 16.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let routes = sample_line_routing_coupons();
    export(OUTPUTS[1], &routes);

    let trap = condensate_trap_cartridge_envelope();
    export(OUTPUTS[2], &trap);

    let filter = hydrophobic_filter_holder();
    export(OUTPUTS[3], &filter);

    let ports = pressure_flow_witness_ports();
    export(OUTPUTS[4], &ports);

    let pads = wetness_color_witness_pads();
    export(OUTPUTS[5], &pads);

    let loggers = temperature_rh_logger_pockets();
    export(OUTPUTS[6], &loggers);

    let token_rail = purge_recovery_token_rail();
    export(OUTPUTS[7], &token_rail);

    let custody = barcode_coa_custody_lands();
    export(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[9], &gates);

    let camera = camera_evidence_bridge();
    export(OUTPUTS[10], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + routes.translate(ROUTE_POS.0, ROUTE_POS.1, deck_insert_z())
        + trap.translate(TRAP_POS.0, TRAP_POS.1, deck_insert_z())
        + filter.translate(FILTER_POS.0, FILTER_POS.1, deck_insert_z())
        + ports.translate(PORT_POS.0, PORT_POS.1, deck_insert_z())
        + pads.translate(WETNESS_POS.0, WETNESS_POS.1, deck_insert_z())
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, deck_insert_z())
        + token_rail.translate(TOKEN_POS.0, TOKEN_POS.1, deck_insert_z())
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, deck_insert_z())
        + gates.translate(GATE_POS.0, GATE_POS.1, deck_insert_z())
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator gas-sampling line condensate-trap validation station:");
    println!(
        "  Footprint:       {STATION_X:.0}mm x {STATION_Y:.0}mm closed secondary-containment deck with {:.0}mL modeled freeboard",
        containment_freeboard_ml()
    );
    println!(
        "  Sample routing:  {ROUTE_COUPONS} removable routing coupons for {SAMPLE_LINE_OD:.1}mm OD sample line, {TRAP_CARTRIDGES} condensate trap envelopes, {HYDROPHOBIC_FILTERS} hydrophobic filter holders"
    );
    println!(
        "  Witnessing:      {PRESSURE_PORTS} pressure witness ports, {FLOW_WITNESS_PORTS} flow witness ports, {WETNESS_PADS} wetness pads, {COLOR_REFERENCE_PADS} color references, {TEMP_RH_LOGGERS} temp/RH logger pockets"
    );
    println!(
        "  Custody/gating:  {PURGE_TOKENS} purge tokens, {RECOVERY_TOKENS} recovery tokens, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {DISPOSITION_GATES} release/hold/reject gates"
    );
    println!(
        "  Evidence:        {CAMERA_MOUNTS} camera mounts, {LIGHT_BARS} light bars, {VIEW_FIELD_RIBS} field ribs, {KEEP_OUT_GAUGES} robot/service keepout gauges"
    );
    println!(
        "  Scope:           no live-cell process, gas acceptance criterion, sensor calibration rule, or pressure-rated containment is encoded."
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z() -> f64 {
    DECK_Z - SOCKET_DEPTH + 0.6
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(ROUTE_COUPONS, ROUTE_COUPON_COLS * ROUTE_COUPON_ROWS);
    assert_eq!(
        ROUTE_COUPONS * ROUTE_WITNESS_CLIPS_PER_COUPON,
        sample_route_clip_count()
    );
    assert_eq!(PRESSURE_PORTS + FLOW_WITNESS_PORTS, PORT_ROWS * PORT_COLS);
    assert_eq!(WETNESS_PADS, WETNESS_PAD_COLS * WETNESS_PAD_ROWS);
    assert_eq!(TEMP_RH_LOGGERS, LOGGER_COLS * LOGGER_ROWS);
    assert_eq!(FILTER_BULKHEADS, HYDROPHOBIC_FILTERS * 2);
    assert_eq!(DISPOSITION_GATES, DISPOSITION_LANES.len());
    assert_eq!(MOUNT_HOLES, mount_hole_positions().len());
    assert_eq!(DATUM_TARGETS, datum_positions().len());
    assert_eq!(TAMPER_SEAL_POSTS, 4);
    assert!(TRAP_CARTRIDGE_LEN > TRAP_CARTRIDGE_OD * 3.5);
    assert!(FILTER_LEN > FILTER_OD * 4.0);
    assert!(SAMPLE_LINE_OD < PORT_D);
    assert!(containment_freeboard_ml() > maximum_condensate_challenge_ml());
    assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE);
    assert!(rear_trap_service_clearance() >= REAR_TRAP_SERVICE_CLEARANCE);
    assert!(right_filter_service_clearance() >= RIGHT_FILTER_SERVICE_CLEARANCE);
    assert!(left_line_service_clearance() >= LEFT_LINE_SERVICE_CLEARANCE);
    assert!(TOP_TRAP_LIFT_CLEARANCE > CAMERA_POST_Z);
    assert!(CAMERA_SPAN_X < STATION_X - 120.0);

    let rects = socket_rects();
    for item in rects {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds condensate-trap validation deck",
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

fn socket_rects() -> [Rect; 9] {
    [
        rect("sample_line_routing_coupons", ROUTE_POS, ROUTE_X, ROUTE_Y),
        rect(
            "condensate_trap_cartridge_envelope",
            TRAP_POS,
            TRAP_X,
            TRAP_Y,
        ),
        rect("hydrophobic_filter_holder", FILTER_POS, FILTER_X, FILTER_Y),
        rect("pressure_flow_witness_ports", PORT_POS, PORT_X, PORT_Y),
        rect(
            "wetness_color_witness_pads",
            WETNESS_POS,
            WETNESS_X,
            WETNESS_Y,
        ),
        rect(
            "temperature_rh_logger_pockets",
            LOGGER_POS,
            LOGGER_X,
            LOGGER_Y,
        ),
        rect("purge_recovery_token_rail", TOKEN_POS, TOKEN_X, TOKEN_Y),
        rect(
            "barcode_coa_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect("release_hold_reject_gates", GATE_POS, GATE_X, GATE_Y),
    ]
}

fn sample_route_clip_count() -> usize {
    ROUTE_COUPONS * ROUTE_WITNESS_CLIPS_PER_COUPON
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    inner_x * inner_y * (RIM_Z - BASIN_RECESS_Z) / 1000.0
}

fn maximum_condensate_challenge_ml() -> f64 {
    let sample_line_hold_up = ROUTE_COUPONS as f64 * 7.5;
    let trap_hold_up = TRAP_LOW_POINT_CUPS as f64 * 75.0;
    let witness_hold_up = WETNESS_PADS as f64 * 4.0;
    let port_hold_up = (PRESSURE_PORTS + FLOW_WITNESS_PORTS) as f64 * 5.0;

    sample_line_hold_up + trap_hold_up + witness_hold_up + port_hold_up
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (GATE_POS.1.abs() + GATE_Y / 2.0)
}

fn rear_trap_service_clearance() -> f64 {
    STATION_Y / 2.0 - (TRAP_POS.1 + TRAP_Y / 2.0)
}

fn right_filter_service_clearance() -> f64 {
    STATION_X / 2.0 - (FILTER_POS.0 + FILTER_X / 2.0)
}

fn left_line_service_clearance() -> f64 {
    STATION_X / 2.0 - (ROUTE_POS.0.abs() + ROUTE_X / 2.0)
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "gas_sampling_condensate_trap_validation_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "gas_sampling_condensate_trap_validation_secondary_basin_recess",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        BASIN_RECESS_Z + 0.8,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_RECESS_Z / 2.0);
    let drain = centered_cylinder(
        "gas_sampling_condensate_trap_validation_low_point_drain_bore",
        9.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 30.0,
        DECK_Z - 8.0,
    );

    deck - basin - drain - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + workflow_spines()
        + leak_sensor_wells()
        + datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("gas_sampling_condensate_trap_validation_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!(
                    "gas_sampling_condensate_trap_validation_{}_socket",
                    item.name
                ),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("gas_sampling_condensate_trap_validation_mount_holes");
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("gas_sampling_condensate_trap_validation_m6_clearance_{i}"),
                3.4,
                DECK_Z + 5.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0)
            + centered_cube(
                format!("gas_sampling_condensate_trap_validation_m6_slot_relief_{i}"),
                30.0,
                7.4,
                DECK_Z + 5.0,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
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
        "gas_sampling_condensate_trap_validation_front_low_robot_rim",
        STATION_X,
        RIM_W,
        32.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 16.0);
    let rear = centered_cube(
        "gas_sampling_condensate_trap_validation_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "gas_sampling_condensate_trap_validation_left_sample_line_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "gas_sampling_condensate_trap_validation_right_filter_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_spines() -> Part {
    let route_to_trap = centered_cube(
        "gas_sampling_condensate_trap_validation_route_to_trap_flow_spine",
        STATION_X - 240.0,
        8.0,
        22.0,
    )
    .translate(0.0, 150.0, DECK_Z + 11.0);
    let pressure_to_witness = centered_cube(
        "gas_sampling_condensate_trap_validation_pressure_to_wetness_flow_spine",
        STATION_X - 260.0,
        8.0,
        20.0,
    )
    .translate(0.0, -65.0, DECK_Z + 10.0);
    let custody_divider = centered_cube(
        "gas_sampling_condensate_trap_validation_custody_disposition_spine",
        STATION_X - 300.0,
        8.0,
        20.0,
    )
    .translate(0.0, -295.0, DECK_Z + 10.0);
    let clean_dirty_centerline = centered_cube(
        "gas_sampling_condensate_trap_validation_clean_dirty_centerline_spine",
        10.0,
        STATION_Y - 205.0,
        24.0,
    )
    .translate(0.0, -32.0, DECK_Z + 12.0);

    route_to_trap + pressure_to_witness + custody_divider + clean_dirty_centerline
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("gas_sampling_condensate_trap_validation_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 126.0);
        let y = if i % 2 == 0 {
            -STATION_Y / 2.0 + 110.0
        } else {
            STATION_Y / 2.0 - 110.0
        };
        wells = wells
            + shallow_ring(
                &format!("gas_sampling_condensate_trap_validation_leak_sensor_well_{i}"),
                34.0,
                20.0,
                5.0,
                28,
            )
            .translate(x, y, DECK_Z + 2.5);
    }
    wells
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("gas_sampling_condensate_trap_validation_datum_targets");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "gas_sampling_condensate_trap_validation_robot_datum_target_{i}"
            ))
            .translate(*x, *y, DECK_Z + 3.0);
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGETS] {
    [
        (-STATION_X / 2.0 + 108.0, -STATION_Y / 2.0 + 106.0),
        (STATION_X / 2.0 - 108.0, -STATION_Y / 2.0 + 106.0),
        (-STATION_X / 2.0 + 108.0, STATION_Y / 2.0 - 106.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 106.0),
    ]
}

fn sample_line_routing_coupons() -> Part {
    let rail = centered_cube(
        "gas_sampling_condensate_trap_sample_line_routing_coupon_rail",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    )
    .translate(0.0, 0.0, ROUTE_Z / 2.0);
    let relief = centered_cube(
        "gas_sampling_condensate_trap_sample_line_routing_coupon_relief",
        ROUTE_X - 42.0,
        ROUTE_Y - 38.0,
        10.0,
    )
    .translate(0.0, 0.0, ROUTE_Z - 5.0);

    let mut coupons = Part::empty("gas_sampling_condensate_trap_route_coupon_inserts");
    for i in 0..ROUTE_COUPONS {
        let col = i % ROUTE_COUPON_COLS;
        let row = i / ROUTE_COUPON_COLS;
        coupons = coupons
            + routing_coupon(i).translate(
                centered_index(col, ROUTE_COUPON_COLS, 124.0),
                centered_index(row, ROUTE_COUPON_ROWS, 76.0),
                ROUTE_Z,
            );
    }

    rail - relief + coupons + route_end_bulkheads() + sample_line_axis_rulers()
}

fn routing_coupon(index: usize) -> Part {
    let coupon = centered_cube(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_body"),
        ROUTE_COUPON_X,
        ROUTE_COUPON_Y,
        ROUTE_COUPON_Z,
    )
    .translate(0.0, 0.0, ROUTE_COUPON_Z / 2.0);
    let straight_groove = centered_cylinder(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_straight_groove"),
        SAMPLE_LINE_OD / 2.0 + 1.2,
        ROUTE_COUPON_X - 22.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -8.0, ROUTE_COUPON_Z);
    let offset_groove = centered_cylinder(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_offset_groove"),
        SAMPLE_LINE_OD / 2.0 + 1.2,
        ROUTE_COUPON_X - 42.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(8.0, 14.0, ROUTE_COUPON_Z);
    let cross_groove = centered_cylinder(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_vertical_cross_groove"),
        SAMPLE_LINE_OD / 2.0 + 1.0,
        ROUTE_COUPON_Y - 18.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-28.0 + (index % 3) as f64 * 18.0, 0.0, ROUTE_COUPON_Z);
    let raised_centerline = centered_cylinder(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_raised_sample_line_axis"),
        2.0,
        ROUTE_COUPON_X - 18.0,
        16,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -8.0, ROUTE_COUPON_Z + 5.0);
    let clips = route_coupon_clips(index);
    let witness_window = centered_cube(
        format!("gas_sampling_condensate_trap_route_coupon_{index}_transparent_witness_window"),
        22.0,
        16.0,
        4.0,
    )
    .translate(32.0, 14.0, ROUTE_COUPON_Z + 2.0);

    coupon - straight_groove - offset_groove - cross_groove
        + raised_centerline
        + clips
        + witness_window
}

fn route_coupon_clips(index: usize) -> Part {
    let mut clips = Part::empty(format!(
        "gas_sampling_condensate_trap_route_coupon_{index}_wet_line_clips"
    ));
    for side in 0..ROUTE_WITNESS_CLIPS_PER_COUPON {
        let x = centered_index(side, ROUTE_WITNESS_CLIPS_PER_COUPON, 74.0);
        clips = clips
            + centered_cube(
                format!("gas_sampling_condensate_trap_route_coupon_{index}_clip_{side}"),
                18.0,
                10.0,
                12.0,
            )
            .translate(x, -ROUTE_COUPON_Y / 2.0 - 3.0, ROUTE_COUPON_Z + 6.0);
    }
    clips
}

fn route_end_bulkheads() -> Part {
    let inlet = tube_bulkhead(
        "gas_sampling_condensate_trap_sample_line_inlet_bulkhead",
        -ROUTE_X / 2.0 + 30.0,
        0.0,
        ROUTE_Z + 28.0,
        18.0,
    );
    let outlet = tube_bulkhead(
        "gas_sampling_condensate_trap_sample_line_outlet_bulkhead",
        ROUTE_X / 2.0 - 30.0,
        0.0,
        ROUTE_Z + 28.0,
        18.0,
    );
    inlet + outlet
}

fn sample_line_axis_rulers() -> Part {
    let mut ticks = Part::empty("gas_sampling_condensate_trap_sample_line_axis_rulers");
    for i in 0..11 {
        ticks = ticks
            + centered_cube(
                format!("gas_sampling_condensate_trap_route_axis_tick_{i}"),
                4.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(i, 11, 46.0),
                ROUTE_Y / 2.0 - 20.0,
                ROUTE_Z + 3.0,
            );
    }
    ticks
}

fn condensate_trap_cartridge_envelope() -> Part {
    let base = centered_cube(
        "gas_sampling_condensate_trap_cartridge_envelope_base",
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    )
    .translate(0.0, 0.0, TRAP_Z / 2.0);
    let sump = centered_cube(
        "gas_sampling_condensate_trap_cartridge_sump_relief",
        TRAP_X - 70.0,
        TRAP_Y - 72.0,
        18.0,
    )
    .translate(0.0, -8.0, TRAP_Z - 9.0);
    let mut cartridges = Part::empty("gas_sampling_condensate_trap_cartridge_service_envelopes");
    for i in 0..TRAP_CARTRIDGES {
        let y = centered_index(i, TRAP_CARTRIDGES, 76.0);
        cartridges = cartridges
            + trap_cartridge(i).translate(0.0, y, TRAP_Z + TRAP_CARTRIDGE_OD / 2.0 + 6.0);
    }

    base - sump + cartridges + trap_clamps() + trap_low_point_cups() + trap_flow_keys()
}

fn trap_cartridge(index: usize) -> Part {
    let body = centered_cylinder(
        format!("gas_sampling_condensate_trap_cartridge_{index}_clear_bowl_envelope"),
        TRAP_CARTRIDGE_OD / 2.0,
        TRAP_CARTRIDGE_LEN,
        48,
    )
    .rotate(0.0, 90.0, 0.0);
    let bowl = centered_cylinder(
        format!("gas_sampling_condensate_trap_cartridge_{index}_condensate_bowl_envelope"),
        TRAP_BOWL_OD / 2.0,
        52.0,
        48,
    )
    .translate(-36.0, 0.0, -TRAP_CARTRIDGE_OD / 2.0);
    let inlet_axis = centered_cylinder(
        format!("gas_sampling_condensate_trap_cartridge_{index}_inlet_tube_axis"),
        4.0,
        48.0,
        18,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(TRAP_CARTRIDGE_LEN / 2.0 + 24.0), 0.0, 0.0);
    let outlet_axis = centered_cylinder(
        format!("gas_sampling_condensate_trap_cartridge_{index}_outlet_tube_axis"),
        4.0,
        48.0,
        18,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(TRAP_CARTRIDGE_LEN / 2.0 + 24.0, 0.0, 0.0);

    body + bowl + inlet_axis + outlet_axis
}

fn trap_clamps() -> Part {
    let mut clamps = Part::empty("gas_sampling_condensate_trap_cartridge_swing_clamps");
    for i in 0..TRAP_CLAMPS {
        let cartridge = i / 2;
        let side = i % 2;
        let x = if side == 0 { -112.0 } else { 112.0 };
        let y = centered_index(cartridge, TRAP_CARTRIDGES, 76.0);
        clamps = clamps
            + centered_cube(
                format!("gas_sampling_condensate_trap_cartridge_clamp_{i}"),
                48.0,
                18.0,
                24.0,
            )
            .translate(x, y - 32.0, TRAP_Z + 28.0);
    }
    clamps
}

fn trap_low_point_cups() -> Part {
    let mut cups = Part::empty("gas_sampling_condensate_trap_low_point_cups");
    for i in 0..TRAP_LOW_POINT_CUPS {
        cups = cups
            + shallow_ring(
                &format!("gas_sampling_condensate_trap_low_point_cup_{i}"),
                58.0,
                34.0,
                18.0,
                40,
            )
            .translate(
                -42.0,
                centered_index(i, TRAP_LOW_POINT_CUPS, 76.0),
                TRAP_Z + 9.0,
            );
    }
    cups
}

fn trap_flow_keys() -> Part {
    let inlet_arrow = centered_cube(
        "gas_sampling_condensate_trap_cartridge_inlet_direction_key",
        106.0,
        8.0,
        8.0,
    )
    .translate(-118.0, TRAP_Y / 2.0 - 28.0, TRAP_Z + 4.0);
    let outlet_arrow = centered_cube(
        "gas_sampling_condensate_trap_cartridge_outlet_direction_key",
        106.0,
        8.0,
        8.0,
    )
    .translate(118.0, TRAP_Y / 2.0 - 28.0, TRAP_Z + 4.0);
    let orientation_stop = centered_cube(
        "gas_sampling_condensate_trap_keyed_single_direction_stop",
        18.0,
        TRAP_Y - 64.0,
        28.0,
    )
    .translate(TRAP_X / 2.0 - 38.0, 0.0, TRAP_Z + 14.0);

    inlet_arrow + outlet_arrow + orientation_stop
}

fn hydrophobic_filter_holder() -> Part {
    let block = centered_cube(
        "gas_sampling_condensate_trap_hydrophobic_filter_holder_block",
        FILTER_X,
        FILTER_Y,
        FILTER_Z,
    )
    .translate(0.0, 0.0, FILTER_Z / 2.0);
    let drain_moat = centered_cube(
        "gas_sampling_condensate_trap_hydrophobic_filter_holder_drain_moat",
        FILTER_X - 56.0,
        FILTER_Y - 54.0,
        10.0,
    )
    .translate(0.0, 0.0, FILTER_Z - 5.0);
    let mut filters = Part::empty("gas_sampling_condensate_trap_hydrophobic_filter_envelopes");
    for i in 0..HYDROPHOBIC_FILTERS {
        filters = filters
            + hydrophobic_filter_envelope(i).translate(
                0.0,
                centered_index(i, HYDROPHOBIC_FILTERS, 62.0),
                FILTER_Z + FILTER_OD / 2.0 + 6.0,
            );
    }

    block - drain_moat + filters + filter_clamps() + filter_bulkheads() + filter_orientation_key()
}

fn hydrophobic_filter_envelope(index: usize) -> Part {
    let filter = centered_cylinder(
        format!("gas_sampling_condensate_trap_hydrophobic_filter_{index}_envelope"),
        FILTER_OD / 2.0,
        FILTER_LEN,
        40,
    )
    .rotate(0.0, 90.0, 0.0);
    let upstream = centered_cylinder(
        format!("gas_sampling_condensate_trap_hydrophobic_filter_{index}_upstream_axis"),
        3.2,
        38.0,
        18,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(FILTER_LEN / 2.0 + 19.0), 0.0, 0.0);
    let downstream = centered_cylinder(
        format!("gas_sampling_condensate_trap_hydrophobic_filter_{index}_downstream_axis"),
        3.2,
        38.0,
        18,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(FILTER_LEN / 2.0 + 19.0, 0.0, 0.0);

    filter + upstream + downstream
}

fn filter_clamps() -> Part {
    let mut clamps = Part::empty("gas_sampling_condensate_trap_hydrophobic_filter_clamps");
    for i in 0..FILTER_CLAMPS {
        let row = i / 2;
        let side = i % 2;
        clamps = clamps
            + centered_cube(
                format!("gas_sampling_condensate_trap_hydrophobic_filter_clamp_{i}"),
                42.0,
                14.0,
                22.0,
            )
            .translate(
                if side == 0 { -74.0 } else { 74.0 },
                centered_index(row, HYDROPHOBIC_FILTERS, 62.0) - 24.0,
                FILTER_Z + 20.0,
            );
    }
    clamps
}

fn filter_bulkheads() -> Part {
    let mut bulkheads = Part::empty("gas_sampling_condensate_trap_filter_holder_bulkheads");
    for i in 0..FILTER_BULKHEADS {
        let row = i / 2;
        let side = i % 2;
        bulkheads = bulkheads
            + tube_bulkhead(
                &format!("gas_sampling_condensate_trap_filter_bulkhead_{i}"),
                if side == 0 {
                    -FILTER_X / 2.0 + 34.0
                } else {
                    FILTER_X / 2.0 - 34.0
                },
                centered_index(row, HYDROPHOBIC_FILTERS, 62.0),
                FILTER_Z + 36.0,
                16.0,
            );
    }
    bulkheads
}

fn filter_orientation_key() -> Part {
    let keyed_stop = centered_cube(
        "gas_sampling_condensate_trap_hydrophobic_filter_single_direction_key",
        16.0,
        FILTER_Y - 34.0,
        28.0,
    )
    .translate(FILTER_X / 2.0 - 36.0, 0.0, FILTER_Z + 14.0);
    let drip_line = centered_cube(
        "gas_sampling_condensate_trap_hydrophobic_filter_downstream_dry_side_line",
        FILTER_X - 82.0,
        4.0,
        7.0,
    )
    .translate(0.0, -FILTER_Y / 2.0 + 26.0, FILTER_Z + 3.5);

    keyed_stop + drip_line
}

fn pressure_flow_witness_ports() -> Part {
    let manifold = centered_cube(
        "gas_sampling_condensate_trap_pressure_flow_witness_manifold",
        PORT_X,
        PORT_Y,
        PORT_Z,
    )
    .translate(0.0, 0.0, PORT_Z / 2.0);
    let top_relief = centered_cube(
        "gas_sampling_condensate_trap_pressure_flow_manifold_top_relief",
        PORT_X - 42.0,
        PORT_Y - 38.0,
        12.0,
    )
    .translate(0.0, 0.0, PORT_Z - 6.0);

    manifold - top_relief
        + pressure_witness_ports()
        + flow_witness_ports()
        + sensor_pocket_bank()
        + capillary_flow_rulers()
}

fn pressure_witness_ports() -> Part {
    let mut ports = Part::empty("gas_sampling_condensate_trap_pressure_witness_ports");
    for i in 0..PRESSURE_PORTS {
        ports = ports
            + port_tower(
                &format!("gas_sampling_condensate_trap_pressure_witness_port_{i}"),
                centered_index(i, PRESSURE_PORTS, 86.0) - 42.0,
                30.0,
                PORT_Z,
            );
    }
    ports
}

fn flow_witness_ports() -> Part {
    let mut ports = Part::empty("gas_sampling_condensate_trap_flow_witness_ports");
    for i in 0..FLOW_WITNESS_PORTS {
        let x = centered_index(i, FLOW_WITNESS_PORTS, 86.0) - 42.0;
        ports = ports
            + port_tower(
                &format!("gas_sampling_condensate_trap_flow_witness_port_{i}"),
                x,
                -32.0,
                PORT_Z,
            )
            + centered_cube(
                format!("gas_sampling_condensate_trap_flow_witness_sight_slot_{i}"),
                58.0,
                10.0,
                18.0,
            )
            .translate(x, -54.0, PORT_Z + 11.0);
    }
    ports
}

fn port_tower(name: &str, x: f64, y: f64, z: f64) -> Part {
    vertical_ring(&format!("{name}_luer_face_ring"), 28.0, PORT_D, 18.0, 32).translate(
        x,
        y,
        z + 9.0,
    ) + centered_cube(format!("{name}_wrench_flat"), 36.0, 8.0, 10.0).translate(
        x,
        y - 18.0,
        z + 14.0,
    )
}

fn sensor_pocket_bank() -> Part {
    let mut pockets = Part::empty("gas_sampling_condensate_trap_pressure_flow_sensor_pockets");
    for i in 0..SENSOR_POCKETS {
        pockets = pockets
            + centered_cube(
                format!("gas_sampling_condensate_trap_sensor_pocket_{i}"),
                58.0,
                32.0,
                16.0,
            )
            .translate(
                centered_index(i, SENSOR_POCKETS, 86.0) - 42.0,
                PORT_Y / 2.0 - 26.0,
                PORT_Z + 8.0,
            );
    }
    pockets
}

fn capillary_flow_rulers() -> Part {
    let mut ticks = Part::empty("gas_sampling_condensate_trap_capillary_flow_ruler_ticks");
    for i in 0..13 {
        ticks = ticks
            + centered_cube(
                format!("gas_sampling_condensate_trap_flow_ruler_tick_{i}"),
                3.0,
                18.0,
                6.0,
            )
            .translate(centered_index(i, 13, 34.0), 0.0, PORT_Z + 3.0);
    }
    ticks
}

fn wetness_color_witness_pads() -> Part {
    let plate = centered_cube(
        "gas_sampling_condensate_trap_wetness_color_witness_pad_plate",
        WETNESS_X,
        WETNESS_Y,
        WETNESS_Z,
    )
    .translate(0.0, 0.0, WETNESS_Z / 2.0);
    let pad_recess = centered_cube(
        "gas_sampling_condensate_trap_wetness_pad_bank_recess",
        WETNESS_X - 50.0,
        WETNESS_Y - 38.0,
        9.0,
    )
    .translate(0.0, 0.0, WETNESS_Z - 4.5);

    plate - pad_recess + wetness_pad_wells() + color_reference_strip() + pad_clip_tabs()
}

fn wetness_pad_wells() -> Part {
    let mut pads = Part::empty("gas_sampling_condensate_trap_wetness_pad_wells");
    for i in 0..WETNESS_PADS {
        let col = i % WETNESS_PAD_COLS;
        let row = i / WETNESS_PAD_COLS;
        pads = pads
            + shallow_ring_rect(
                &format!("gas_sampling_condensate_trap_wetness_color_pad_well_{i}"),
                PAD_WELL_X,
                PAD_WELL_Y,
                PAD_WELL_X - 12.0,
                PAD_WELL_Y - 12.0,
                8.0,
            )
            .translate(
                centered_index(col, WETNESS_PAD_COLS, 70.0),
                centered_index(row, WETNESS_PAD_ROWS, 54.0),
                WETNESS_Z + 4.0,
            );
    }
    pads
}

fn color_reference_strip() -> Part {
    let mut strip = Part::empty("gas_sampling_condensate_trap_color_reference_strip");
    for i in 0..COLOR_REFERENCE_PADS {
        strip = strip
            + centered_cube(
                format!("gas_sampling_condensate_trap_color_reference_pad_{i}"),
                30.0,
                18.0,
                5.0,
            )
            .translate(
                WETNESS_X / 2.0 - 52.0,
                centered_index(i, COLOR_REFERENCE_PADS, 22.0),
                WETNESS_Z + 2.5,
            );
    }
    strip
}

fn pad_clip_tabs() -> Part {
    let mut tabs = Part::empty("gas_sampling_condensate_trap_wetness_pad_retainer_tabs");
    for i in 0..WETNESS_PAD_ROWS {
        tabs = tabs
            + centered_cube(
                format!("gas_sampling_condensate_trap_wetness_pad_row_retainer_tab_{i}"),
                WETNESS_X - 130.0,
                8.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, WETNESS_PAD_ROWS, 54.0) + 23.0,
                WETNESS_Z + 5.0,
            );
    }
    tabs
}

fn temperature_rh_logger_pockets() -> Part {
    let block = centered_cube(
        "gas_sampling_condensate_trap_temperature_rh_logger_pocket_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);
    let cable_trough = centered_cube(
        "gas_sampling_condensate_trap_temperature_rh_logger_cable_trough",
        LOGGER_X - 42.0,
        14.0,
        16.0,
    )
    .translate(0.0, 0.0, LOGGER_Z - 8.0);

    block - cable_trough + logger_pocket_lands() + rh_probe_wells() + logger_cable_combs()
}

fn logger_pocket_lands() -> Part {
    let mut pockets = Part::empty("gas_sampling_condensate_trap_temperature_rh_logger_lands");
    for i in 0..TEMP_RH_LOGGERS {
        let col = i % LOGGER_COLS;
        let row = i / LOGGER_COLS;
        pockets = pockets
            + shallow_ring_rect(
                &format!("gas_sampling_condensate_trap_temp_rh_logger_pocket_{i}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_X - 14.0,
                LOGGER_POCKET_Y - 14.0,
                9.0,
            )
            .translate(
                centered_index(col, LOGGER_COLS, 98.0),
                centered_index(row, LOGGER_ROWS, 68.0),
                LOGGER_Z + 4.5,
            );
    }
    pockets
}

fn rh_probe_wells() -> Part {
    let mut wells = Part::empty("gas_sampling_condensate_trap_rh_probe_wells");
    for i in 0..TEMP_RH_LOGGERS {
        let col = i % LOGGER_COLS;
        let row = i / LOGGER_COLS;
        wells = wells
            + shallow_ring(
                &format!("gas_sampling_condensate_trap_rh_probe_guard_ring_{i}"),
                22.0,
                10.0,
                7.0,
                28,
            )
            .translate(
                centered_index(col, LOGGER_COLS, 98.0) + 34.0,
                centered_index(row, LOGGER_ROWS, 68.0),
                LOGGER_Z + 3.5,
            );
    }
    wells
}

fn logger_cable_combs() -> Part {
    let mut combs = Part::empty("gas_sampling_condensate_trap_logger_cable_comb_slots");
    for i in 0..7 {
        combs = combs
            + centered_cube(
                format!("gas_sampling_condensate_trap_logger_cable_comb_tooth_{i}"),
                6.0,
                LOGGER_Y - 38.0,
                14.0,
            )
            .translate(centered_index(i, 7, 42.0), 0.0, LOGGER_Z + 7.0);
    }
    combs
}

fn purge_recovery_token_rail() -> Part {
    let rail = centered_cube(
        "gas_sampling_condensate_trap_purge_recovery_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0);
    let channel = centered_cube(
        "gas_sampling_condensate_trap_purge_recovery_token_slide_channel",
        TOKEN_X - 42.0,
        34.0,
        11.0,
    )
    .translate(0.0, 0.0, TOKEN_Z - 5.5);

    rail - channel + token_nests() + purge_recovery_stop_gates()
}

fn token_nests() -> Part {
    let mut nests = Part::empty("gas_sampling_condensate_trap_purge_recovery_token_nests");
    for i in 0..PURGE_TOKENS {
        nests = nests
            + shallow_ring(
                &format!("gas_sampling_condensate_trap_purge_token_nest_{i}"),
                TOKEN_NEST_D + 10.0,
                TOKEN_NEST_D,
                7.0,
                30,
            )
            .translate(centered_index(i, PURGE_TOKENS, 42.0), 32.0, TOKEN_Z + 3.5);
    }
    for i in 0..RECOVERY_TOKENS {
        nests = nests
            + shallow_ring(
                &format!("gas_sampling_condensate_trap_recovery_token_nest_{i}"),
                TOKEN_NEST_D + 10.0,
                TOKEN_NEST_D,
                7.0,
                30,
            )
            .translate(
                centered_index(i, RECOVERY_TOKENS, 42.0),
                -32.0,
                TOKEN_Z + 3.5,
            );
    }
    nests
}

fn purge_recovery_stop_gates() -> Part {
    let left = centered_cube(
        "gas_sampling_condensate_trap_purge_recovery_left_stop_gate",
        12.0,
        TOKEN_Y - 34.0,
        34.0,
    )
    .translate(-TOKEN_X / 2.0 + 28.0, 0.0, TOKEN_Z + 17.0);
    let right = centered_cube(
        "gas_sampling_condensate_trap_purge_recovery_right_stop_gate",
        12.0,
        TOKEN_Y - 34.0,
        34.0,
    )
    .translate(TOKEN_X / 2.0 - 28.0, 0.0, TOKEN_Z + 17.0);
    let center = centered_cube(
        "gas_sampling_condensate_trap_purge_recovery_center_sequence_divider",
        TOKEN_X - 88.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, TOKEN_Z + 9.0);

    left + right + center
}

fn barcode_coa_custody_lands() -> Part {
    let plate = centered_cube(
        "gas_sampling_condensate_trap_barcode_coa_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);

    plate + barcode_lands() + coa_lands() + tamper_seal_posts() + custody_chain_ruler()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("gas_sampling_condensate_trap_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("gas_sampling_condensate_trap_barcode_land_{i}"),
                70.0,
                20.0,
                5.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 48.0),
                28.0,
                CUSTODY_Z + 2.5,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("gas_sampling_condensate_trap_coa_certificate_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + shallow_ring_rect(
                &format!("gas_sampling_condensate_trap_coa_card_pocket_{i}"),
                112.0,
                34.0,
                96.0,
                22.0,
                6.0,
            )
            .translate(centered_index(i, COA_LANDS, 132.0), -30.0, CUSTODY_Z + 3.0);
    }
    lands
}

fn tamper_seal_posts() -> Part {
    let mut posts = Part::empty("gas_sampling_condensate_trap_tamper_seal_posts");
    for (i, (x, y)) in [
        (-CUSTODY_X / 2.0 + 26.0, -CUSTODY_Y / 2.0 + 24.0),
        (CUSTODY_X / 2.0 - 26.0, -CUSTODY_Y / 2.0 + 24.0),
        (-CUSTODY_X / 2.0 + 26.0, CUSTODY_Y / 2.0 - 24.0),
        (CUSTODY_X / 2.0 - 26.0, CUSTODY_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("gas_sampling_condensate_trap_tamper_seal_post_{i}"),
                5.0,
                18.0,
                20,
            )
            .translate(*x, *y, CUSTODY_Z + 9.0);
    }
    posts
}

fn custody_chain_ruler() -> Part {
    let mut ticks = Part::empty("gas_sampling_condensate_trap_custody_chain_ruler_ticks");
    for i in 0..9 {
        ticks = ticks
            + centered_cube(
                format!("gas_sampling_condensate_trap_custody_chain_tick_{i}"),
                4.0,
                12.0,
                5.0,
            )
            .translate(centered_index(i, 9, 48.0), 0.0, CUSTODY_Z + 2.5);
    }
    ticks
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "gas_sampling_condensate_trap_release_hold_reject_gate_plate",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(0.0, 0.0, GATE_Z / 2.0);

    base + disposition_gate_lanes() + disposition_token_pockets() + gate_position_flags()
}

fn disposition_gate_lanes() -> Part {
    let mut lanes = Part::empty("gas_sampling_condensate_trap_release_hold_reject_lanes");
    for (i, lane) in DISPOSITION_LANES.iter().enumerate() {
        let x = centered_index(i, DISPOSITION_GATES, 152.0);
        lanes = lanes
            + shallow_ring_rect(
                &format!("gas_sampling_condensate_trap_{lane}_lane_guard"),
                132.0,
                92.0,
                106.0,
                66.0,
                18.0,
            )
            .translate(x, 0.0, GATE_Z + 9.0)
            + centered_cube(
                format!("gas_sampling_condensate_trap_{lane}_gate_stop_bar"),
                116.0,
                10.0,
                32.0,
            )
            .translate(x, -GATE_Y / 2.0 + 24.0, GATE_Z + 16.0);
    }
    lanes
}

fn disposition_token_pockets() -> Part {
    let mut pockets = Part::empty("gas_sampling_condensate_trap_disposition_token_pockets");
    for gate in 0..DISPOSITION_GATES {
        for token in 0..TOKENS_PER_GATE {
            pockets = pockets
                + shallow_ring(
                    &format!("gas_sampling_condensate_trap_gate_{gate}_token_pocket_{token}"),
                    22.0,
                    12.0,
                    6.0,
                    24,
                )
                .translate(
                    centered_index(gate, DISPOSITION_GATES, 152.0)
                        + centered_index(token, TOKENS_PER_GATE, 24.0),
                    24.0,
                    GATE_Z + 3.0,
                );
        }
    }
    pockets
}

fn gate_position_flags() -> Part {
    let mut flags = Part::empty("gas_sampling_condensate_trap_gate_position_flags");
    for (i, lane) in DISPOSITION_LANES.iter().enumerate() {
        flags = flags
            + centered_cube(
                format!("gas_sampling_condensate_trap_{lane}_raised_status_flag"),
                72.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(i, DISPOSITION_GATES, 152.0),
                GATE_Y / 2.0 - 20.0,
                GATE_Z + 5.0,
            );
    }
    flags
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "gas_sampling_condensate_trap_camera_evidence_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_POST_Z,
    )
    .translate(-CAMERA_SPAN_X / 2.0 + 28.0, 0.0, CAMERA_POST_Z / 2.0);
    let right_post = centered_cube(
        "gas_sampling_condensate_trap_camera_evidence_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_POST_Z,
    )
    .translate(CAMERA_SPAN_X / 2.0 - 28.0, 0.0, CAMERA_POST_Z / 2.0);
    let beam = centered_cube(
        "gas_sampling_condensate_trap_camera_evidence_cross_beam",
        CAMERA_SPAN_X,
        24.0,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_POST_Z + CAMERA_BEAM_Z / 2.0);

    left_post + right_post + beam + camera_mounts() + camera_light_bars() + view_field_ribs()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("gas_sampling_condensate_trap_camera_evidence_mounts");
    for i in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("gas_sampling_condensate_trap_camera_mount_land_{i}"),
                78.0,
                42.0,
                16.0,
            )
            .translate(
                centered_index(i, CAMERA_MOUNTS, 310.0),
                -18.0,
                CAMERA_POST_Z - 32.0,
            );
    }
    mounts
}

fn camera_light_bars() -> Part {
    let mut bars = Part::empty("gas_sampling_condensate_trap_camera_evidence_light_bars");
    for i in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("gas_sampling_condensate_trap_light_bar_{i}"),
                180.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(i, LIGHT_BARS, 360.0),
                22.0,
                CAMERA_POST_Z - 54.0,
            );
    }
    bars
}

fn view_field_ribs() -> Part {
    let mut ribs = Part::empty("gas_sampling_condensate_trap_camera_view_field_ribs");
    for i in 0..VIEW_FIELD_RIBS {
        ribs = ribs
            + centered_cube(
                format!("gas_sampling_condensate_trap_camera_view_field_rib_{i}"),
                4.0,
                CAMERA_Y,
                8.0,
            )
            .translate(
                centered_index(i, VIEW_FIELD_RIBS, 180.0),
                0.0,
                CAMERA_POST_Z - 78.0,
            );
    }
    ribs
}

fn robot_service_keepout_gauges() -> Part {
    let front = keepout_frame(
        "gas_sampling_condensate_trap_front_robot_keepout_gauge",
        STATION_X - 160.0,
        88.0,
        30.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 74.0, 15.0);
    let rear = keepout_frame(
        "gas_sampling_condensate_trap_rear_trap_service_keepout_gauge",
        STATION_X - 200.0,
        82.0,
        32.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 86.0, 16.0);
    let left = keepout_frame(
        "gas_sampling_condensate_trap_left_sample_line_service_keepout_gauge",
        88.0,
        STATION_Y - 220.0,
        36.0,
    )
    .translate(-STATION_X / 2.0 + 88.0, -15.0, 18.0);
    let right = keepout_frame(
        "gas_sampling_condensate_trap_right_filter_service_keepout_gauge",
        88.0,
        STATION_Y - 230.0,
        36.0,
    )
    .translate(STATION_X / 2.0 - 92.0, -10.0, 18.0);
    let top_trap = centered_cube(
        "gas_sampling_condensate_trap_top_trap_lift_keepout_height_gauge",
        450.0,
        24.0,
        10.0,
    )
    .translate(TRAP_POS.0, TRAP_POS.1, TOP_TRAP_LIFT_CLEARANCE);
    let camera_service = centered_cube(
        "gas_sampling_condensate_trap_camera_bridge_service_height_gauge",
        CAMERA_SPAN_X - 150.0,
        20.0,
        10.0,
    )
    .translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        CAMERA_POST_Z + CAMERA_BEAM_Z + 30.0,
    );

    front + rear + left + right + top_trap + camera_service + keepout_height_posts()
}

fn keepout_height_posts() -> Part {
    let mut posts = Part::empty("gas_sampling_condensate_trap_keepout_height_posts");
    for (i, (x, y, height)) in [
        (ROUTE_POS.0, ROUTE_POS.1, 128.0),
        (TRAP_POS.0, TRAP_POS.1, TOP_TRAP_LIFT_CLEARANCE),
        (FILTER_POS.0, FILTER_POS.1, 190.0),
        (PORT_POS.0, PORT_POS.1, 145.0),
        (LOGGER_POS.0, LOGGER_POS.1, 132.0),
        (GATE_POS.0, GATE_POS.1, 110.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("gas_sampling_condensate_trap_keepout_height_post_{i}"),
                5.0,
                *height,
                18,
            )
            .translate(*x, *y, height / 2.0);
    }
    posts
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    shallow_ring_rect(name, x, y, x - 24.0, y - 24.0, z)
}

fn fiducial_disc(name: &str) -> Part {
    vertical_ring(name, 32.0, 10.0, 6.0, 36)
        + centered_cylinder(format!("{name}_center_dot"), 3.0, 8.0, 18).translate(0.0, 0.0, 1.0)
}

fn tube_bulkhead(name: &str, x: f64, y: f64, z: f64, od: f64) -> Part {
    let boss = vertical_ring(&format!("{name}_face_ring"), od + 16.0, od, 14.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, z);
    let flat = centered_cube(format!("{name}_wrench_flat"), od + 22.0, 7.0, 10.0).translate(
        x,
        y - 9.0,
        z + od / 2.0,
    );

    boss + flat
}

fn vertical_ring(name: &str, outer_d: f64, inner_d: f64, height: f64, segments: u32) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, height, segments)
        - centered_cylinder(
            format!("{name}_inner"),
            inner_d / 2.0,
            height + 2.0,
            segments,
        )
}

fn shallow_ring(name: &str, outer_d: f64, inner_d: f64, height: f64, segments: u32) -> Part {
    vertical_ring(name, outer_d, inner_d, height, segments)
}

fn shallow_ring_rect(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    inner_x: f64,
    inner_y: f64,
    height: f64,
) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
        - centered_cube(format!("{name}_inner"), inner_x, inner_y, height + 2.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_incubator_gas_sampling_line_condensate_trap_validation_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit_and_exported() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "{feature} missing from output manifest"
            );
        }
    }

    #[test]
    fn modules_fit_inside_deck_and_do_not_overlap() {
        assert_design_constraints();
        for item in socket_rects() {
            assert!(item.fits_inside_deck(), "{} does not fit", item.name);
        }
    }

    #[test]
    fn closed_condensate_containment_exceeds_modeled_challenge() {
        assert!(containment_freeboard_ml() > maximum_condensate_challenge_ml());
        assert!(maximum_condensate_challenge_ml() < 350.0);
    }

    #[test]
    fn sampling_trap_filter_and_witness_counts_match_scope() {
        assert_eq!(ROUTE_COUPONS, 8);
        assert_eq!(TRAP_CARTRIDGES, 2);
        assert_eq!(TRAP_CLAMPS, 4);
        assert_eq!(HYDROPHOBIC_FILTERS, 2);
        assert_eq!(FILTER_CLAMPS, 4);
        assert_eq!(PRESSURE_PORTS, 4);
        assert_eq!(FLOW_WITNESS_PORTS, 4);
        assert_eq!(SENSOR_POCKETS, 4);
    }

    #[test]
    fn witness_logger_token_and_custody_capacity_is_explicit() {
        assert_eq!(WETNESS_PADS, 12);
        assert_eq!(COLOR_REFERENCE_PADS, 6);
        assert_eq!(TEMP_RH_LOGGERS, 6);
        assert_eq!(PURGE_TOKENS, 6);
        assert_eq!(RECOVERY_TOKENS, 6);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_LANDS, 3);
        assert_eq!(TAMPER_SEAL_POSTS, 4);
    }

    #[test]
    fn disposition_evidence_and_keepout_features_are_present() {
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(TOKENS_PER_GATE, 4);
        assert_eq!(CAMERA_MOUNTS, 4);
        assert_eq!(LIGHT_BARS, 3);
        assert_eq!(VIEW_FIELD_RIBS, 6);
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(TOP_TRAP_LIFT_CLEARANCE > CAMERA_POST_Z);
    }
}
