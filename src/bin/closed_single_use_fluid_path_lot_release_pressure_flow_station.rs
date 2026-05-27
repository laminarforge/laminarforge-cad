use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed single-use fluid path lot-release pressure/flow station.
//
// Intent:
// - Qualify disposable tubing set and manifold lots inside a clean
//   isolator/cabinet before connection to live tissue chips.
// - Make route identity, blocked filters, wrong lumen routing, low compliance,
//   pressure/flow mismatch, leak containment, custody, and disposition visible
//   in source-only CAD geometry.
// - Tie the checkout to scaled tissue-chip operation with a 4x4 route map and
//   sixteen pressure/flow witness channels instead of a single-chip fixture.
//
// Research basis:
// - BPSA single-use integrity recommendations emphasize risk-based integrity
//   assurance, visual inspection, leak testing, lifecycle custody, and supplier
//   plus end-user responsibilities.
// - Public ASTM E3336/E3051 summaries describe single-use system physical
//   integrity testing and fit-for-use verification, including pressure-based
//   methods for empty, clean, dry assemblies.
//
// This is mechanical packaging for purchased sensors, valves, filters, barcode
// readers, tubing, sterile connectors, and waste hardware. It is not a sterile
// barrier definition, validated leak limit, pressure-rated wetted design, or
// release specification.

const OUTPUTS: [&str; 12] = [
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_leak_containment_deck.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_tubing_set_nest.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_keyed_route_comb.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_filter_bubble_trap_witness_holders.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_pressure_step_manifold.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_flow_split_witness_channels.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_compliance_pulse_dampening_witness_chamber.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_barcode_lot_custody_lands.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_waste_quarantine_outlet.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_camera_fiducials_optical_bridge.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_release_hold_reject_disposition_features.stl",
    "output/closed_single_use_fluid_path_lot_release_pressure_flow_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "leak_containment_deck",
    "tubing_set_nest",
    "keyed_route_comb",
    "filter_and_bubble_trap_witness_holders",
    "pressure_step_manifold",
    "flow_split_witness_channels",
    "compliance_pulse_dampening_witness_chamber",
    "barcode_lot_custody_lands",
    "waste_quarantine_outlet",
    "camera_fiducials",
    "release_hold_reject_disposition_features",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_source_only_cad",
    "not_a_sterile_barrier_definition",
    "not_a_pressure_rated_wetted_path",
    "not_a_validated_release_specification",
    "purchased_sensors_filters_and_valves_are_placeholders",
    "test_limits_require_process_validation",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const DESIGN_CLEARANCE: f64 = 10.0;
const MODULE_CLEARANCE: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 12.0;
const TUBE_OD_MAX: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 1.0;
const TUBE_BORE_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;

const ROUTE_ROWS: usize = 4;
const ROUTE_COLS: usize = 4;
const ROUTE_COUNT: usize = ROUTE_ROWS * ROUTE_COLS;
const ROUTE_PITCH_X: f64 = 88.0;
const ROUTE_PITCH_Y: f64 = 54.0;

const NEST_CENTER: (f64, f64) = (-510.0, 190.0);
const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 270.0;
const NEST_Z: f64 = 44.0;
const NEST_SOCKET_X: f64 = 62.0;
const NEST_SOCKET_Y: f64 = 38.0;
const NEST_TUBE_CHANNEL_DEPTH: f64 = 16.0;

const COMB_CENTER: (f64, f64) = (0.0, 245.0);
const COMB_X: f64 = 510.0;
const COMB_Y: f64 = 220.0;
const COMB_Z: f64 = 34.0;
const COMB_TOOTH_W: f64 = 8.0;
const COMB_TOOTH_Y: f64 = 46.0;
const COMB_TOOTH_Z: f64 = 40.0;
const KEY_PROFILES: usize = ROUTE_COUNT;

const CARTRIDGE_CENTER: (f64, f64) = (510.0, 210.0);
const CARTRIDGE_X: f64 = 410.0;
const CARTRIDGE_Y: f64 = 300.0;
const CARTRIDGE_Z: f64 = 58.0;
const FILTER_HOLDERS: usize = ROUTE_COUNT;
const BUBBLE_TRAP_HOLDERS: usize = ROUTE_COUNT;
const FILTER_BODY_D: f64 = 22.0;
const BUBBLE_TRAP_D: f64 = 24.0;

const PRESSURE_CENTER: (f64, f64) = (-510.0, -85.0);
const PRESSURE_X: f64 = 410.0;
const PRESSURE_Y: f64 = 230.0;
const PRESSURE_Z: f64 = 64.0;
const PRESSURE_STEP_LEVELS: usize = 3;
const PRESSURE_STEP_PORTS: usize = ROUTE_COUNT;
const PRESSURE_SENSOR_POCKETS: usize = ROUTE_COUNT;
const PRESSURE_STEP_PITCH_X: f64 = 24.0;
const PRESSURE_SETPOINTS_KPA: [f64; PRESSURE_STEP_LEVELS] = [10.0, 25.0, 45.0];

const FLOW_CENTER: (f64, f64) = (0.0, -65.0);
const FLOW_X: f64 = 560.0;
const FLOW_Y: f64 = 270.0;
const FLOW_Z: f64 = 38.0;
const FLOW_WITNESS_CHANNELS: usize = ROUTE_COUNT;
const FLOW_SPLIT_HEADERS: usize = ROUTE_ROWS;
const FLOW_WINDOW_X: f64 = 70.0;
const FLOW_WINDOW_Y: f64 = 14.0;

const COMPLIANCE_CENTER: (f64, f64) = (510.0, -80.0);
const COMPLIANCE_X: f64 = 410.0;
const COMPLIANCE_Y: f64 = 250.0;
const COMPLIANCE_Z: f64 = 70.0;
const COMPLIANCE_WITNESS_CHAMBERS: usize = ROUTE_ROWS;
const PULSE_DAMPER_TAPS: usize = ROUTE_COUNT;
const COMPLIANCE_CHAMBER_D: f64 = 54.0;
const LOW_COMPLIANCE_STOP_GAUGES: usize = ROUTE_ROWS;

const CUSTODY_CENTER: (f64, f64) = (-470.0, -325.0);
const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 140.0;
const CUSTODY_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_TABS: usize = 4;

const DISPOSITION_CENTER: (f64, f64) = (0.0, -330.0);
const DISPOSITION_X: f64 = 430.0;
const DISPOSITION_Y: f64 = 140.0;
const DISPOSITION_Z: f64 = 40.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_ROUTE_SLOTS: usize = ROUTE_COUNT;

const WASTE_CENTER: (f64, f64) = (510.0, -330.0);
const WASTE_X: f64 = 410.0;
const WASTE_Y: f64 = 150.0;
const WASTE_Z: f64 = 62.0;
const WASTE_QUARANTINE_CUPS: usize = 4;
const WASTE_BRANCH_INLETS: usize = ROUTE_ROWS;
const QUARANTINE_OUTLET_D: f64 = 16.0;

const CAMERA_CENTER: (f64, f64) = (0.0, 430.0);
const CAMERA_X: f64 = 900.0;
const CAMERA_Y: f64 = 48.0;
const CAMERA_Z: f64 = 86.0;
const CAMERA_FIDUCIALS: usize = 6;
const CAMERA_VIEW_WINDOWS: usize = ROUTE_COLS;

const ROBOT_KEEP_OUT_GAUGES: usize = 5;
const ISOLATOR_MAX_X: f64 = 1600.0;
const ISOLATOR_MAX_Y: f64 = 1000.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - DESIGN_CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - DESIGN_CLEARANCE;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = leak_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = tubing_set_nest();
    export(OUTPUTS[1], &nest);

    let comb = keyed_route_comb();
    export(OUTPUTS[2], &comb);

    let cartridges = filter_bubble_trap_witness_holders();
    export(OUTPUTS[3], &cartridges);

    let pressure = pressure_step_manifold();
    export(OUTPUTS[4], &pressure);

    let flow = flow_split_witness_channels();
    export(OUTPUTS[5], &flow);

    let compliance = compliance_pulse_dampening_witness_chamber();
    export(OUTPUTS[6], &compliance);

    let custody = barcode_lot_custody_lands();
    export(OUTPUTS[7], &custody);

    let waste = waste_quarantine_outlet();
    export(OUTPUTS[8], &waste);

    let camera = camera_fiducials_optical_bridge();
    export(OUTPUTS[9], &camera);

    let disposition = release_hold_reject_disposition_features();
    export(OUTPUTS[10], &disposition);

    let keepouts = robot_service_keepouts();
    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, on_deck_z(NEST_Z))
        + comb.translate(COMB_CENTER.0, COMB_CENTER.1, on_deck_z(COMB_Z))
        + cartridges.translate(
            CARTRIDGE_CENTER.0,
            CARTRIDGE_CENTER.1,
            on_deck_z(CARTRIDGE_Z),
        )
        + pressure.translate(PRESSURE_CENTER.0, PRESSURE_CENTER.1, on_deck_z(PRESSURE_Z))
        + flow.translate(FLOW_CENTER.0, FLOW_CENTER.1, on_deck_z(FLOW_Z))
        + compliance.translate(
            COMPLIANCE_CENTER.0,
            COMPLIANCE_CENTER.1,
            on_deck_z(COMPLIANCE_Z),
        )
        + custody.translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, on_deck_z(CUSTODY_Z))
        + waste.translate(WASTE_CENTER.0, WASTE_CENTER.1, on_deck_z(WASTE_Z))
        + camera.translate(CAMERA_CENTER.0, CAMERA_CENTER.1, on_deck_z(CAMERA_Z))
        + disposition.translate(
            DISPOSITION_CENTER.0,
            DISPOSITION_CENTER.1,
            on_deck_z(DISPOSITION_Z),
        )
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed single-use fluid path lot release pressure/flow station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm leak-containment deck for clean isolator placement"
    );
    println!(
        "  Route scale:            {ROUTE_COUNT} witnesses in a {ROUTE_ROWS}x{ROUTE_COLS} tissue-chip route map"
    );
    println!(
        "  Pressure checkout:      {PRESSURE_STEP_LEVELS} pressure steps ({:.0}/{:.0}/{:.0} kPa labels), {PRESSURE_STEP_PORTS} route ports, {PRESSURE_SENSOR_POCKETS} sensor pockets",
        PRESSURE_SETPOINTS_KPA[0],
        PRESSURE_SETPOINTS_KPA[1],
        PRESSURE_SETPOINTS_KPA[2]
    );
    println!(
        "  Flow checkout:          {FLOW_WITNESS_CHANNELS} split flow witnesses, {FLOW_SPLIT_HEADERS} row headers, {FILTER_HOLDERS} filter holders, {BUBBLE_TRAP_HOLDERS} bubble-trap holders"
    );
    println!(
        "  Compliance witness:     {COMPLIANCE_WITNESS_CHAMBERS} row dampening chambers with {PULSE_DAMPER_TAPS} pulse taps and {LOW_COMPLIANCE_STOP_GAUGES} low-compliance stop gauges"
    );
    println!(
        "  Custody/disposition:    {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {TAMPER_SEAL_TABS} seal tabs, release/hold/reject lanes, and waste quarantine outlet"
    );
    println!(
        "  Evidence geometry:      {CAMERA_FIDUCIALS} camera fiducials, {CAMERA_VIEW_WINDOWS} view windows, {ROBOT_KEEP_OUT_GAUGES} robot/service keepout gauges, {} required feature groups, {} explicit limitations",
        REQUIRED_FEATURES.len(),
        LIMITATIONS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn route_center(row: usize, col: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    (
        centered_index(col, ROUTE_COLS, pitch_x),
        centered_index(row, ROUTE_ROWS, pitch_y),
    )
}

fn insert_rects() -> [Rect; 10] {
    [
        Rect {
            name: "tubing_set_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "keyed_route_comb",
            center: COMB_CENTER,
            x: COMB_X,
            y: COMB_Y,
        },
        Rect {
            name: "filter_bubble_trap_witness_holders",
            center: CARTRIDGE_CENTER,
            x: CARTRIDGE_X,
            y: CARTRIDGE_Y,
        },
        Rect {
            name: "pressure_step_manifold",
            center: PRESSURE_CENTER,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "flow_split_witness_channels",
            center: FLOW_CENTER,
            x: FLOW_X,
            y: FLOW_Y,
        },
        Rect {
            name: "compliance_pulse_dampening_witness_chamber",
            center: COMPLIANCE_CENTER,
            x: COMPLIANCE_X,
            y: COMPLIANCE_Y,
        },
        Rect {
            name: "barcode_lot_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "release_hold_reject_disposition_features",
            center: DISPOSITION_CENTER,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Rect {
            name: "waste_quarantine_outlet",
            center: WASTE_CENTER,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "camera_fiducials_optical_bridge",
            center: CAMERA_CENTER,
            x: CAMERA_X,
            y: CAMERA_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(ROUTE_COUNT, ROUTE_ROWS * ROUTE_COLS);
    assert_eq!(KEY_PROFILES, ROUTE_COUNT);
    assert_eq!(FILTER_HOLDERS, ROUTE_COUNT);
    assert_eq!(BUBBLE_TRAP_HOLDERS, ROUTE_COUNT);
    assert_eq!(PRESSURE_STEP_PORTS, ROUTE_COUNT);
    assert_eq!(PRESSURE_SENSOR_POCKETS, ROUTE_COUNT);
    assert_eq!(FLOW_WITNESS_CHANNELS, ROUTE_COUNT);
    assert_eq!(PULSE_DAMPER_TAPS, ROUTE_COUNT);
    assert_eq!(COMPLIANCE_WITNESS_CHAMBERS, ROUTE_ROWS);
    assert_eq!(WASTE_BRANCH_INLETS, ROUTE_ROWS);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(DISPOSITION_ROUTE_SLOTS, ROUTE_COUNT);
    assert!(STATION_X <= ISOLATOR_MAX_X);
    assert!(STATION_Y <= ISOLATOR_MAX_Y);
    assert!(QUARANTINE_OUTLET_D > TUBE_BORE_D);

    for rect in insert_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station envelope",
            rect.name
        );
    }

    let rects = insert_rects();
    for i in 0..rects.len() {
        for j in i + 1..rects.len() {
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], MODULE_CLEARANCE),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn leak_containment_deck() -> Part {
    let deck = centered_cube(
        "closed_single_use_fluid_path_lot_release_deck_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_single_use_fluid_path_lot_release_shallow_containment_basin",
        STATION_X - 124.0,
        STATION_Y - 122.0,
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z / 2.0 - 4.0);
    let pressure_trough = centered_cube(
        "closed_single_use_fluid_path_lot_release_pressure_zone_spill_trough",
        520.0,
        52.0,
        9.0,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1 - 118.0,
        BASE_Z / 2.0 - 4.5,
    );
    let witness_trough = centered_cube(
        "closed_single_use_fluid_path_lot_release_flow_witness_spill_trough",
        1030.0,
        54.0,
        9.0,
    )
    .translate(260.0, -238.0, BASE_Z / 2.0 - 4.5);
    let drain = centered_cylinder(
        "closed_single_use_fluid_path_lot_release_deck_low_point_quarantine_drain",
        DRAIN_D / 2.0,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 54.0,
        -STATION_Y / 2.0 + 14.0,
        0.0,
    );

    deck - basin
        - pressure_trough
        - witness_trough
        - drain
        - component_sockets()
        - deck_mount_holes()
        + containment_rims()
        + leak_witness_route_grid()
        + deck_camera_datum_targets()
        + quarantine_drain_guard()
}

fn component_sockets() -> Part {
    let mut sockets = Part::empty("closed_single_use_fluid_path_lot_release_component_sockets");
    for rect in insert_rects() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_single_use_fluid_path_lot_release_{}_registration_socket",
                    rect.name
                ),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_single_use_fluid_path_lot_release_deck_mount_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(x, y, 0.0);
    }
    holes
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "closed_single_use_fluid_path_lot_release_left_containment_rim",
        RIM_W,
        STATION_Y - 68.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        -6.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_single_use_fluid_path_lot_release_right_containment_rim",
        RIM_W,
        STATION_Y - 68.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        -6.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_single_use_fluid_path_lot_release_rear_camera_rim",
        STATION_X - 80.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_single_use_fluid_path_lot_release_front_low_witness_lip",
        STATION_X - 210.0,
        13.0,
        22.0,
    )
    .translate(30.0, -STATION_Y / 2.0 + 28.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front
}

fn leak_witness_route_grid() -> Part {
    let mut grid = Part::empty("closed_single_use_fluid_path_lot_release_leak_witness_grid");
    let grid_x = FLOW_X + 26.0;
    let grid_y = FLOW_Y + 26.0;
    for row in 0..=ROUTE_ROWS {
        let y = FLOW_CENTER.1 - grid_y / 2.0 + row as f64 * grid_y / ROUTE_ROWS as f64;
        grid = grid
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_leak_row_divider_{row}"),
                grid_x,
                4.0,
                6.0,
            )
            .translate(FLOW_CENTER.0, y, BASE_Z / 2.0 + 3.0);
    }
    for col in 0..=ROUTE_COLS {
        let x = FLOW_CENTER.0 - grid_x / 2.0 + col as f64 * grid_x / ROUTE_COLS as f64;
        grid = grid
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_leak_col_divider_{col}"),
                4.0,
                grid_y,
                6.0,
            )
            .translate(x, FLOW_CENTER.1, BASE_Z / 2.0 + 3.0);
    }
    grid
}

fn deck_camera_datum_targets() -> Part {
    let mut targets = Part::empty("closed_single_use_fluid_path_lot_release_deck_camera_datums");
    for (i, (x, y)) in [
        (CAMERA_CENTER.0 - CAMERA_X / 2.0 + 48.0, CAMERA_CENTER.1),
        (CAMERA_CENTER.0 + CAMERA_X / 2.0 - 48.0, CAMERA_CENTER.1),
        (
            NEST_CENTER.0 - NEST_X / 2.0 + 28.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 26.0,
        ),
        (
            CARTRIDGE_CENTER.0 + CARTRIDGE_X / 2.0 - 28.0,
            CARTRIDGE_CENTER.1 + CARTRIDGE_Y / 2.0 - 26.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(&format!(
                "closed_single_use_fluid_path_lot_release_deck_camera_datum_{i}"
            ))
            .translate(x, y, BASE_Z / 2.0 + 2.0);
    }
    targets
}

fn quarantine_drain_guard() -> Part {
    let guard = centered_cube(
        "closed_single_use_fluid_path_lot_release_quarantine_drain_guard",
        112.0,
        42.0,
        30.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 54.0,
        -STATION_Y / 2.0 + 48.0,
        BASE_Z / 2.0 + 15.0,
    );
    let bore = centered_cylinder(
        "closed_single_use_fluid_path_lot_release_quarantine_guard_access_bore",
        QUARANTINE_OUTLET_D / 2.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 54.0,
        -STATION_Y / 2.0 + 48.0,
        BASE_Z / 2.0 + 15.0,
    );
    guard - bore
}

fn tubing_set_nest() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_tubing_set_nest_plate",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let mut cuts = Part::empty("closed_single_use_fluid_path_lot_release_tubing_nest_cuts");
    let mut clamps = Part::empty("closed_single_use_fluid_path_lot_release_tubing_nest_clamps");
    let mut route_rims = Part::empty("closed_single_use_fluid_path_lot_release_tubing_route_rims");

    for row in 0..ROUTE_ROWS {
        for col in 0..ROUTE_COLS {
            let index = row * ROUTE_COLS + col;
            let (x, y) = route_center(row, col, ROUTE_PITCH_X, ROUTE_PITCH_Y);
            let saddle = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_route_{index}_tube_saddle"),
                TUBE_BORE_D / 2.0,
                NEST_SOCKET_X + 20.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, NEST_Z / 2.0 - NEST_TUBE_CHANNEL_DEPTH / 2.0);
            let top_access = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_route_{index}_top_access_slot"),
                NEST_SOCKET_X + 18.0,
                TUBE_BORE_D + 4.0,
                NEST_TUBE_CHANNEL_DEPTH + 8.0,
            )
            .translate(x, y, NEST_Z / 2.0 - NEST_TUBE_CHANNEL_DEPTH / 2.0 + 4.0);
            cuts = cuts + saddle + top_access;

            let left_stop = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_route_{index}_left_lumen_stop"),
                7.0,
                NEST_SOCKET_Y,
                22.0,
            )
            .translate(x - NEST_SOCKET_X / 2.0 - 9.0, y, NEST_Z / 2.0 + 11.0);
            let right_stop = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_route_{index}_right_lumen_stop"),
                7.0,
                NEST_SOCKET_Y,
                22.0,
            )
            .translate(x + NEST_SOCKET_X / 2.0 + 9.0, y, NEST_Z / 2.0 + 11.0);
            let route_socket_rim = rectangular_frame(
                &format!("closed_single_use_fluid_path_lot_release_route_{index}_nest_witness_rim"),
                NEST_SOCKET_X + 22.0,
                NEST_SOCKET_Y + 18.0,
                4.0,
                5.0,
            )
            .translate(x, y, NEST_Z / 2.0 + 2.5);
            clamps = clamps + left_stop + right_stop;
            route_rims = route_rims + route_socket_rim;
        }
    }

    plate + clamps + route_rims + nest_datum_pins() + nest_manifold_shadow() - cuts
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("closed_single_use_fluid_path_lot_release_tubing_nest_datum_pins");
    for (i, (x, y)) in [
        (-NEST_X / 2.0 + 30.0, -NEST_Y / 2.0 + 30.0),
        (NEST_X / 2.0 - 30.0, -NEST_Y / 2.0 + 30.0),
        (-NEST_X / 2.0 + 30.0, NEST_Y / 2.0 - 30.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_nest_datum_pin_{i}"),
                5.0,
                10.0,
                24,
            )
            .translate(x, y, NEST_Z / 2.0 + 5.0);
    }
    pins
}

fn nest_manifold_shadow() -> Part {
    let outline = rectangular_frame(
        "closed_single_use_fluid_path_lot_release_4x4_manifold_shadow_outline",
        ROUTE_PITCH_X * 3.0 + 118.0,
        ROUTE_PITCH_Y * 3.0 + 84.0,
        5.0,
        5.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 + 2.5);
    let inlet_flag = centered_cube(
        "closed_single_use_fluid_path_lot_release_nest_inlet_lumen_flag",
        52.0,
        10.0,
        14.0,
    )
    .translate(
        -NEST_X / 2.0 + 58.0,
        NEST_Y / 2.0 - 44.0,
        NEST_Z / 2.0 + 7.0,
    );
    let outlet_flag = centered_cube(
        "closed_single_use_fluid_path_lot_release_nest_outlet_lumen_flag",
        52.0,
        10.0,
        14.0,
    )
    .translate(
        NEST_X / 2.0 - 58.0,
        -NEST_Y / 2.0 + 44.0,
        NEST_Z / 2.0 + 7.0,
    );
    outline + inlet_flag + outlet_flag
}

fn keyed_route_comb() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_keyed_route_comb_plate",
        COMB_X,
        COMB_Y,
        COMB_Z,
    );
    let mut channel_cuts =
        Part::empty("closed_single_use_fluid_path_lot_release_comb_channel_cuts");
    let mut teeth = Part::empty("closed_single_use_fluid_path_lot_release_comb_teeth");
    let mut key_posts = Part::empty("closed_single_use_fluid_path_lot_release_comb_key_posts");

    for row in 0..ROUTE_ROWS {
        for col in 0..ROUTE_COLS {
            let route = row * ROUTE_COLS + col;
            let (x, y) = route_center(row, col, 104.0, 42.0);
            let channel = centered_cylinder(
                format!(
                    "closed_single_use_fluid_path_lot_release_comb_route_{route}_tube_clearance"
                ),
                (TUBE_BORE_D + 0.8) / 2.0,
                82.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, COMB_Z / 2.0 + 10.0);
            channel_cuts = channel_cuts + channel;

            for tooth in 0..3 {
                let tx = x - 18.0 + tooth as f64 * 18.0 + row as f64 * 1.4;
                teeth = teeth
                    + centered_cube(
                        format!("closed_single_use_fluid_path_lot_release_route_{route}_key_tooth_{tooth}"),
                        COMB_TOOTH_W,
                        COMB_TOOTH_Y,
                        COMB_TOOTH_Z,
                    )
                    .translate(tx, y, COMB_Z / 2.0 + COMB_TOOTH_Z / 2.0);
            }

            let key_offset = -18.0 + col as f64 * 12.0;
            key_posts = key_posts
                + centered_cube(
                    format!("closed_single_use_fluid_path_lot_release_route_{route}_asymmetric_lumen_key"),
                    10.0,
                    14.0 + row as f64 * 2.5,
                    28.0,
                )
                .translate(x + key_offset, y + 29.0, COMB_Z / 2.0 + 14.0);
        }
    }

    plate + teeth + key_posts + comb_row_headers() - channel_cuts
}

fn comb_row_headers() -> Part {
    let mut headers = Part::empty("closed_single_use_fluid_path_lot_release_comb_row_headers");
    for row in 0..ROUTE_ROWS {
        let y = centered_index(row, ROUTE_ROWS, 42.0);
        headers = headers
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_comb_row_{row}_header_bar"),
                COMB_X - 62.0,
                6.0,
                9.0,
            )
            .translate(0.0, y - 29.0, COMB_Z / 2.0 + 4.5);
    }
    headers
}

fn filter_bubble_trap_witness_holders() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_filter_bubble_trap_holder_plate",
        CARTRIDGE_X,
        CARTRIDGE_Y,
        CARTRIDGE_Z,
    );
    let mut cuts = Part::empty("closed_single_use_fluid_path_lot_release_cartridge_holder_cuts");
    let mut cradles = Part::empty("closed_single_use_fluid_path_lot_release_cartridge_cradles");
    let mut trap_columns =
        Part::empty("closed_single_use_fluid_path_lot_release_bubble_trap_columns");
    let mut witness_windows =
        Part::empty("closed_single_use_fluid_path_lot_release_cartridge_witness_windows");

    for row in 0..ROUTE_ROWS {
        for col in 0..ROUTE_COLS {
            let index = row * ROUTE_COLS + col;
            let (x, y) = route_center(row, col, 82.0, 58.0);
            let filter_x = x - 18.0;
            let trap_x = x + 24.0;
            let filter_cradle = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_filter_{index}_cradle"),
                50.0,
                30.0,
                22.0,
            )
            .translate(filter_x, y, CARTRIDGE_Z / 2.0 + 11.0);
            let filter_cut = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_filter_{index}_body_clearance"),
                FILTER_BODY_D / 2.0,
                58.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(filter_x, y, CARTRIDGE_Z / 2.0 + 13.0);
            let trap_ring = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_bubble_trap_{index}_outer_ring"),
                BUBBLE_TRAP_D / 2.0 + 5.0,
                12.0,
                32,
            )
            .translate(trap_x, y, CARTRIDGE_Z / 2.0 + 6.0);
            let trap_socket = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_bubble_trap_{index}_socket"),
                BUBBLE_TRAP_D / 2.0,
                CARTRIDGE_Z + 10.0,
                32,
            )
            .translate(trap_x, y, CARTRIDGE_Z / 2.0);
            let window = rectangular_frame(
                &format!(
                    "closed_single_use_fluid_path_lot_release_filter_trap_{index}_witness_window"
                ),
                74.0,
                14.0,
                3.0,
                5.0,
            )
            .translate(x + 2.0, y - 26.0, CARTRIDGE_Z / 2.0 + 22.0);
            cradles = cradles + filter_cradle;
            trap_columns = trap_columns + trap_ring;
            cuts = cuts + filter_cut + trap_socket;
            witness_windows = witness_windows + window;
        }
    }

    plate + cradles + trap_columns + witness_windows + cartridge_lot_latch_rail() - cuts
}

fn cartridge_lot_latch_rail() -> Part {
    let rail = centered_cube(
        "closed_single_use_fluid_path_lot_release_cartridge_lot_latch_rail",
        CARTRIDGE_X - 36.0,
        12.0,
        26.0,
    )
    .translate(0.0, -CARTRIDGE_Y / 2.0 + 32.0, CARTRIDGE_Z / 2.0 + 13.0);
    let mut tabs = Part::empty("closed_single_use_fluid_path_lot_release_cartridge_latch_tabs");
    for col in 0..ROUTE_COLS {
        tabs = tabs
            + centered_cube(
                format!(
                    "closed_single_use_fluid_path_lot_release_cartridge_column_{col}_lot_lock_tab"
                ),
                42.0,
                10.0,
                22.0,
            )
            .translate(
                centered_index(col, ROUTE_COLS, 82.0),
                -CARTRIDGE_Y / 2.0 + 18.0,
                CARTRIDGE_Z / 2.0 + 11.0,
            );
    }
    rail + tabs
}

fn pressure_step_manifold() -> Part {
    let body = centered_cube(
        "closed_single_use_fluid_path_lot_release_pressure_step_manifold_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let mut cuts = Part::empty("closed_single_use_fluid_path_lot_release_pressure_step_cuts");
    let mut blocks = Part::empty("closed_single_use_fluid_path_lot_release_pressure_step_blocks");
    let mut sensors =
        Part::empty("closed_single_use_fluid_path_lot_release_pressure_sensor_pockets");

    for step in 0..PRESSURE_STEP_LEVELS {
        let y = -68.0 + step as f64 * 68.0;
        let header = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_pressure_step_{step}_header_bore"),
            (TUBE_BORE_D + 1.8) / 2.0,
            PRESSURE_X - 42.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, PRESSURE_Z / 2.0 - 5.0);
        cuts = cuts + header;
        blocks = blocks
            + centered_cube(
                format!(
                    "closed_single_use_fluid_path_lot_release_pressure_step_{step}_setpoint_land"
                ),
                PRESSURE_X - 58.0,
                12.0,
                7.0,
            )
            .translate(0.0, y + 18.0, PRESSURE_Z / 2.0 + 3.5);
    }

    for route in 0..ROUTE_COUNT {
        let x = centered_index(route, ROUTE_COUNT, PRESSURE_STEP_PITCH_X);
        let y = -98.0 + (route % ROUTE_ROWS) as f64 * 24.0;
        let port = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_pressure_route_{route}_step_port"),
            4.8,
            PRESSURE_Z + 10.0,
            24,
        )
        .translate(x, y, 0.0);
        let sensor_pocket = centered_cube(
            format!(
                "closed_single_use_fluid_path_lot_release_pressure_route_{route}_sensor_pocket"
            ),
            20.0,
            17.0,
            10.0,
        )
        .translate(x, 82.0, PRESSURE_Z / 2.0 + 5.0);
        let valve_block = centered_cube(
            format!(
                "closed_single_use_fluid_path_lot_release_pressure_route_{route}_pinch_valve_block"
            ),
            18.0,
            20.0,
            18.0,
        )
        .translate(x, 38.0, PRESSURE_Z / 2.0 + 9.0);
        cuts = cuts + port;
        sensors = sensors + sensor_pocket;
        blocks = blocks + valve_block;
    }

    body + pressure_manifold_relief_bypass() + blocks + pressure_step_evidence_tabs()
        - cuts
        - sensors
}

fn pressure_manifold_relief_bypass() -> Part {
    let relief = centered_cube(
        "closed_single_use_fluid_path_lot_release_pressure_relief_bypass_rail",
        PRESSURE_X - 64.0,
        16.0,
        18.0,
    )
    .translate(0.0, -PRESSURE_Y / 2.0 + 26.0, PRESSURE_Z / 2.0 + 9.0);
    let outlet = centered_cylinder(
        "closed_single_use_fluid_path_lot_release_pressure_relief_to_quarantine_port",
        QUARANTINE_OUTLET_D / 2.0,
        34.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        PRESSURE_X / 2.0 - 28.0,
        -PRESSURE_Y / 2.0 + 26.0,
        PRESSURE_Z / 2.0 + 9.0,
    );
    relief - outlet
}

fn pressure_step_evidence_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_single_use_fluid_path_lot_release_pressure_step_evidence_tabs");
    for step in 0..PRESSURE_STEP_LEVELS {
        tabs = tabs
            + centered_cube(
                format!(
                "closed_single_use_fluid_path_lot_release_pressure_step_{step}_camera_evidence_tab"
            ),
                48.0,
                20.0,
                8.0,
            )
            .translate(
                -PRESSURE_X / 2.0 + 42.0,
                -68.0 + step as f64 * 68.0,
                PRESSURE_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn flow_split_witness_channels() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_flow_split_witness_plate",
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    );
    let mut channel_cuts =
        Part::empty("closed_single_use_fluid_path_lot_release_flow_channel_cuts");
    let mut windows = Part::empty("closed_single_use_fluid_path_lot_release_flow_witness_windows");
    let mut ports = Part::empty("closed_single_use_fluid_path_lot_release_flow_split_ports");

    for row in 0..ROUTE_ROWS {
        let row_y = centered_index(row, ROUTE_ROWS, 54.0);
        let row_header = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_flow_row_{row}_split_header"),
            (TUBE_BORE_D + 1.4) / 2.0,
            FLOW_X - 76.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, row_y - 22.0, FLOW_Z / 2.0 - 6.0);
        channel_cuts = channel_cuts + row_header;
    }

    for row in 0..ROUTE_ROWS {
        for col in 0..ROUTE_COLS {
            let route = row * ROUTE_COLS + col;
            let (x, y) = route_center(row, col, 120.0, 54.0);
            let lane = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_flow_route_{route}_witness_channel_cut"),
                FLOW_WINDOW_X,
                FLOW_WINDOW_Y,
                14.0,
            )
            .translate(x, y, FLOW_Z / 2.0 - 5.0);
            let branch_port = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_flow_route_{route}_split_port"),
                5.5,
                16.0,
                24,
            )
            .translate(x - 46.0, y, FLOW_Z / 2.0 + 8.0);
            let witness = rectangular_frame(
                &format!("closed_single_use_fluid_path_lot_release_flow_route_{route}_view_frame"),
                FLOW_WINDOW_X + 10.0,
                FLOW_WINDOW_Y + 10.0,
                3.0,
                5.0,
            )
            .translate(x, y, FLOW_Z / 2.0 + 2.5);
            channel_cuts = channel_cuts + lane;
            ports = ports + branch_port;
            windows = windows + witness;
        }
    }

    plate + ports + windows + flow_inlet_outlet_headers() - channel_cuts
}

fn flow_inlet_outlet_headers() -> Part {
    let inlet = centered_cube(
        "closed_single_use_fluid_path_lot_release_flow_split_inlet_header_land",
        34.0,
        FLOW_Y - 42.0,
        22.0,
    )
    .translate(-FLOW_X / 2.0 + 34.0, 0.0, FLOW_Z / 2.0 + 11.0);
    let outlet = centered_cube(
        "closed_single_use_fluid_path_lot_release_flow_split_outlet_header_land",
        34.0,
        FLOW_Y - 42.0,
        22.0,
    )
    .translate(FLOW_X / 2.0 - 34.0, 0.0, FLOW_Z / 2.0 + 11.0);
    let mismatch_bar = centered_cube(
        "closed_single_use_fluid_path_lot_release_flow_pressure_mismatch_witness_bar",
        FLOW_X - 112.0,
        7.0,
        8.0,
    )
    .translate(0.0, -FLOW_Y / 2.0 + 26.0, FLOW_Z / 2.0 + 4.0);
    inlet + outlet + mismatch_bar
}

fn compliance_pulse_dampening_witness_chamber() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_compliance_witness_plate",
        COMPLIANCE_X,
        COMPLIANCE_Y,
        COMPLIANCE_Z,
    );
    let mut chamber_cuts =
        Part::empty("closed_single_use_fluid_path_lot_release_compliance_chamber_cuts");
    let mut rings =
        Part::empty("closed_single_use_fluid_path_lot_release_compliance_chamber_rings");
    let mut pulse_taps =
        Part::empty("closed_single_use_fluid_path_lot_release_compliance_pulse_taps");
    let mut stop_gauges =
        Part::empty("closed_single_use_fluid_path_lot_release_low_compliance_stop_gauges");

    for row in 0..ROUTE_ROWS {
        let y = centered_index(row, ROUTE_ROWS, 48.0);
        let chamber_x = -COMPLIANCE_X / 2.0 + 86.0 + row as f64 * 76.0;
        let pocket = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_row_{row}_compliance_chamber_pocket"),
            COMPLIANCE_CHAMBER_D / 2.0,
            COMPLIANCE_Z + 10.0,
            42,
        )
        .translate(chamber_x, y, COMPLIANCE_Z / 2.0);
        let ring = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_row_{row}_compliance_chamber_retainer_ring"),
            COMPLIANCE_CHAMBER_D / 2.0 + 8.0,
            10.0,
            42,
        )
        .translate(chamber_x, y, COMPLIANCE_Z / 2.0 + 5.0)
            - centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_row_{row}_compliance_ring_opening"),
                COMPLIANCE_CHAMBER_D / 2.0 - 2.0,
                12.0,
                42,
            )
            .translate(chamber_x, y, COMPLIANCE_Z / 2.0 + 5.0);
        let stop = centered_cube(
            format!("closed_single_use_fluid_path_lot_release_row_{row}_low_compliance_stop_gauge"),
            58.0,
            8.0,
            28.0,
        )
        .translate(chamber_x + 74.0, y, COMPLIANCE_Z / 2.0 + 14.0);
        chamber_cuts = chamber_cuts + pocket;
        rings = rings + ring;
        stop_gauges = stop_gauges + stop;
    }

    for row in 0..ROUTE_ROWS {
        for col in 0..ROUTE_COLS {
            let route = row * ROUTE_COLS + col;
            let x = centered_index(col, ROUTE_COLS, 74.0) + 94.0;
            let y = centered_index(row, ROUTE_ROWS, 48.0);
            let tap = centered_cylinder(
                format!("closed_single_use_fluid_path_lot_release_route_{route}_pulse_damper_tap"),
                5.0,
                16.0,
                24,
            )
            .translate(x, y, COMPLIANCE_Z / 2.0 + 8.0);
            let witness_pin = centered_cube(
                format!("closed_single_use_fluid_path_lot_release_route_{route}_pulse_decay_witness_pin"),
                12.0,
                8.0,
                22.0,
            )
            .translate(x, y + 18.0, COMPLIANCE_Z / 2.0 + 11.0);
            pulse_taps = pulse_taps + tap + witness_pin;
        }
    }

    plate + rings + pulse_taps + stop_gauges + compliance_relief_manifold() - chamber_cuts
}

fn compliance_relief_manifold() -> Part {
    let rail = centered_cube(
        "closed_single_use_fluid_path_lot_release_compliance_relief_manifold",
        COMPLIANCE_X - 54.0,
        14.0,
        18.0,
    )
    .translate(0.0, -COMPLIANCE_Y / 2.0 + 28.0, COMPLIANCE_Z / 2.0 + 9.0);
    let outlet = centered_cylinder(
        "closed_single_use_fluid_path_lot_release_compliance_relief_to_waste_bore",
        QUARANTINE_OUTLET_D / 2.0,
        36.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        COMPLIANCE_X / 2.0 - 36.0,
        -COMPLIANCE_Y / 2.0 + 28.0,
        COMPLIANCE_Z / 2.0 + 9.0,
    );
    rail - outlet
}

fn barcode_lot_custody_lands() -> Part {
    let plate = centered_cube(
        "closed_single_use_fluid_path_lot_release_barcode_lot_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty("closed_single_use_fluid_path_lot_release_barcode_lands");
    let mut windows = Part::empty("closed_single_use_fluid_path_lot_release_barcode_scan_windows");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        let x = centered_index(col, 4, 92.0);
        let y = 30.0 - row as f64 * 54.0;
        lands = lands
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_barcode_lot_land_{i}"),
                76.0,
                34.0,
                4.0,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
        windows = windows
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_barcode_scan_slot_{i}"),
                54.0,
                9.0,
                CUSTODY_Z + 6.0,
            )
            .translate(x, y, 0.0);
    }

    let mut coa = Part::empty("closed_single_use_fluid_path_lot_release_coa_lands");
    for i in 0..COA_LANDS {
        coa = coa
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_coa_custody_land_{i}"),
                92.0,
                24.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 62.0 + i as f64 * 104.0,
                -CUSTODY_Y / 2.0 + 18.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    plate + lands + coa + custody_seal_tabs() - windows
}

fn custody_seal_tabs() -> Part {
    let mut tabs = Part::empty("closed_single_use_fluid_path_lot_release_custody_seal_tabs");
    for i in 0..TAMPER_SEAL_TABS {
        let x = -CUSTODY_X / 2.0 + 40.0 + i as f64 * 116.0;
        tabs = tabs
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_tamper_seal_tab_{i}"),
                44.0,
                10.0,
                18.0,
            )
            .translate(x, CUSTODY_Y / 2.0 - 18.0, CUSTODY_Z / 2.0 + 9.0);
    }
    tabs
}

fn waste_quarantine_outlet() -> Part {
    let body = centered_cube(
        "closed_single_use_fluid_path_lot_release_waste_quarantine_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let mut cuts = Part::empty("closed_single_use_fluid_path_lot_release_waste_quarantine_cuts");
    let mut cups = Part::empty("closed_single_use_fluid_path_lot_release_waste_quarantine_cups");
    for i in 0..WASTE_QUARANTINE_CUPS {
        let x = centered_index(i, WASTE_QUARANTINE_CUPS, 72.0);
        let cup = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_waste_quarantine_cup_{i}"),
            24.0,
            18.0,
            36,
        )
        .translate(x, 20.0, WASTE_Z / 2.0 + 9.0);
        let recess = centered_cylinder(
            format!("closed_single_use_fluid_path_lot_release_waste_quarantine_cup_{i}_recess"),
            17.0,
            22.0,
            36,
        )
        .translate(x, 20.0, WASTE_Z / 2.0 + 10.0);
        cups = cups + cup;
        cuts = cuts + recess;
    }

    for row in 0..WASTE_BRANCH_INLETS {
        let y = -WASTE_Y / 2.0 + 26.0 + row as f64 * 28.0;
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_single_use_fluid_path_lot_release_waste_row_{row}_quarantine_inlet"
                ),
                TUBE_BORE_D / 2.0,
                56.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-WASTE_X / 2.0 + 30.0, y, WASTE_Z / 2.0 - 4.0);
    }

    let outlet = centered_cylinder(
        "closed_single_use_fluid_path_lot_release_waste_quarantine_master_outlet",
        QUARANTINE_OUTLET_D / 2.0,
        68.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WASTE_X / 2.0 - 56.0,
        -WASTE_Y / 2.0 + 4.0,
        WASTE_Z / 2.0 - 6.0,
    );
    cuts = cuts + outlet;

    body + cups + waste_hold_down_gate() + waste_quarantine_witness_flag() - cuts
}

fn waste_hold_down_gate() -> Part {
    centered_cube(
        "closed_single_use_fluid_path_lot_release_waste_one_way_hold_down_gate",
        WASTE_X - 64.0,
        10.0,
        28.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 28.0, WASTE_Z / 2.0 + 14.0)
}

fn waste_quarantine_witness_flag() -> Part {
    centered_cube(
        "closed_single_use_fluid_path_lot_release_waste_quarantine_red_tag_land",
        78.0,
        28.0,
        6.0,
    )
    .translate(
        WASTE_X / 2.0 - 62.0,
        WASTE_Y / 2.0 - 54.0,
        WASTE_Z / 2.0 + 3.0,
    )
}

fn camera_fiducials_optical_bridge() -> Part {
    let rail = centered_cube(
        "closed_single_use_fluid_path_lot_release_camera_bridge_rear_rail",
        CAMERA_X,
        CAMERA_Y,
        18.0,
    )
    .translate(0.0, 0.0, -CAMERA_Z / 2.0 + 9.0);
    let left_post = centered_cube(
        "closed_single_use_fluid_path_lot_release_camera_bridge_left_post",
        24.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(-CAMERA_X / 2.0 + 20.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_single_use_fluid_path_lot_release_camera_bridge_right_post",
        24.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(CAMERA_X / 2.0 - 20.0, 0.0, 0.0);
    let top_beam = centered_cube(
        "closed_single_use_fluid_path_lot_release_camera_bridge_top_beam",
        CAMERA_X - 36.0,
        18.0,
        22.0,
    )
    .translate(0.0, 0.0, CAMERA_Z / 2.0 - 11.0);

    rail + left_post + right_post + top_beam + camera_fiducial_targets() + camera_view_windows()
}

fn camera_fiducial_targets() -> Part {
    let mut targets = Part::empty("closed_single_use_fluid_path_lot_release_camera_fiducials");
    for i in 0..CAMERA_FIDUCIALS {
        let x = centered_index(i, CAMERA_FIDUCIALS, 148.0);
        targets = targets
            + fiducial_target(&format!(
                "closed_single_use_fluid_path_lot_release_camera_fiducial_{i}"
            ))
            .translate(x, -CAMERA_Y / 2.0 - 2.0, CAMERA_Z / 2.0 - 12.0);
    }
    targets
}

fn camera_view_windows() -> Part {
    let mut windows = Part::empty("closed_single_use_fluid_path_lot_release_camera_view_windows");
    for col in 0..CAMERA_VIEW_WINDOWS {
        windows = windows
            + rectangular_frame(
                &format!("closed_single_use_fluid_path_lot_release_camera_view_window_{col}"),
                116.0,
                18.0,
                3.0,
                5.0,
            )
            .translate(
                centered_index(col, CAMERA_VIEW_WINDOWS, 162.0),
                0.0,
                CAMERA_Z / 2.0 - 36.0,
            );
    }
    windows
}

fn release_hold_reject_disposition_features() -> Part {
    let base = centered_cube(
        "closed_single_use_fluid_path_lot_release_disposition_base",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut lane_ribs =
        Part::empty("closed_single_use_fluid_path_lot_release_disposition_lane_ribs");
    let mut slot_cuts =
        Part::empty("closed_single_use_fluid_path_lot_release_disposition_route_slot_cuts");
    let mut gates = Part::empty("closed_single_use_fluid_path_lot_release_disposition_gates");
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 42.0);
        let rib = centered_cube(
            format!("closed_single_use_fluid_path_lot_release_disposition_lane_{lane}_rib"),
            DISPOSITION_X - 36.0,
            5.0,
            10.0,
        )
        .translate(0.0, y - 18.0, DISPOSITION_Z / 2.0 + 5.0);
        let gate = centered_cube(
            format!("closed_single_use_fluid_path_lot_release_disposition_lane_{lane}_status_gate"),
            38.0,
            26.0,
            28.0,
        )
        .translate(-DISPOSITION_X / 2.0 + 40.0, y, DISPOSITION_Z / 2.0 + 14.0);
        lane_ribs = lane_ribs + rib;
        gates = gates + gate;
    }

    for route in 0..ROUTE_COUNT {
        let x = centered_index(route, ROUTE_COUNT, 22.0) + 34.0;
        for lane in 0..DISPOSITION_LANES {
            let y = centered_index(lane, DISPOSITION_LANES, 42.0);
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_single_use_fluid_path_lot_release_disposition_lane_{lane}_route_{route}_evidence_slot"),
                    12.0,
                    18.0,
                    DISPOSITION_Z + 6.0,
                )
                .translate(x, y, 0.0);
        }
    }

    base + lane_ribs + gates + disposition_status_lands() - slot_cuts
}

fn disposition_status_lands() -> Part {
    let release = centered_cube(
        "closed_single_use_fluid_path_lot_release_release_status_land",
        70.0,
        22.0,
        5.0,
    )
    .translate(
        DISPOSITION_X / 2.0 - 54.0,
        centered_index(0, DISPOSITION_LANES, 42.0),
        DISPOSITION_Z / 2.0 + 2.5,
    );
    let hold = centered_cube(
        "closed_single_use_fluid_path_lot_release_hold_status_land",
        70.0,
        22.0,
        5.0,
    )
    .translate(
        DISPOSITION_X / 2.0 - 54.0,
        centered_index(1, DISPOSITION_LANES, 42.0),
        DISPOSITION_Z / 2.0 + 2.5,
    );
    let reject = centered_cube(
        "closed_single_use_fluid_path_lot_release_reject_status_land",
        70.0,
        22.0,
        5.0,
    )
    .translate(
        DISPOSITION_X / 2.0 - 54.0,
        centered_index(2, DISPOSITION_LANES, 42.0),
        DISPOSITION_Z / 2.0 + 2.5,
    );
    release + hold + reject
}

fn robot_service_keepouts() -> Part {
    let mut gauges =
        Part::empty("closed_single_use_fluid_path_lot_release_robot_service_keepout_gauges");
    for (i, (x, y, sx, sy, sz)) in [
        (0.0, CAMERA_CENTER.1 - 62.0, CAMERA_X - 180.0, 10.0, 130.0),
        (NEST_CENTER.0, NEST_CENTER.1, NEST_X + 42.0, 8.0, 110.0),
        (
            CARTRIDGE_CENTER.0,
            CARTRIDGE_CENTER.1,
            CARTRIDGE_X + 42.0,
            8.0,
            160.0,
        ),
        (
            PRESSURE_CENTER.0,
            PRESSURE_CENTER.1,
            PRESSURE_X + 40.0,
            8.0,
            150.0,
        ),
        (
            WASTE_CENTER.0,
            WASTE_CENTER.1 - 92.0,
            WASTE_X + 40.0,
            8.0,
            120.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        gauges = gauges
            + centered_cube(
                format!("closed_single_use_fluid_path_lot_release_robot_service_keepout_{i}"),
                sx,
                sy,
                sz,
            )
            .translate(x, y, BASE_Z / 2.0 + sz / 2.0);
    }
    gauges
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_disc"), 12.0, 4.0, 36);
    let inner = centered_cylinder(format!("{name}_center_bore"), 3.2, 6.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 20.0, 2.8, 4.8);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.8, 20.0, 4.8);
    outer + cross_x + cross_y - inner
}

fn rectangular_frame(name: &str, x: f64, y: f64, rail: f64, z: f64) -> Part {
    let left = centered_cube(format!("{name}_left_rail"), rail, y, z).translate(
        -x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, y, z).translate(
        x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    let top = centered_cube(format!("{name}_top_rail"), x, rail, z).translate(
        0.0,
        y / 2.0 - rail / 2.0,
        0.0,
    );
    let bottom = centered_cube(format!("{name}_bottom_rail"), x, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        0.0,
    );
    left + right + top + bottom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_map_is_scaled_to_sixteen_witnesses() {
        assert_eq!(ROUTE_ROWS, 4);
        assert_eq!(ROUTE_COLS, 4);
        assert_eq!(ROUTE_COUNT, 16);
        assert_eq!(FLOW_WITNESS_CHANNELS, ROUTE_COUNT);
        assert_eq!(PRESSURE_STEP_PORTS, ROUTE_COUNT);
        assert_eq!(FILTER_HOLDERS, ROUTE_COUNT);
        assert_eq!(BUBBLE_TRAP_HOLDERS, ROUTE_COUNT);
        assert_eq!(PULSE_DAMPER_TAPS, ROUTE_COUNT);
    }

    #[test]
    fn layout_components_fit_inside_clean_cabinet_envelope() {
        assert!(STATION_X <= ISOLATOR_MAX_X);
        assert!(STATION_Y <= ISOLATOR_MAX_Y);
        for rect in insert_rects() {
            assert!(rect.fits_inside_station(), "{} out of bounds", rect.name);
        }
    }

    #[test]
    fn layout_components_have_clearance() {
        let rects = insert_rects();
        for i in 0..rects.len() {
            for j in i + 1..rects.len() {
                assert!(
                    !rects[i].overlaps_with_clearance(rects[j], MODULE_CLEARANCE),
                    "{} overlaps {}",
                    rects[i].name,
                    rects[j].name
                );
            }
        }
    }

    #[test]
    fn custody_disposition_and_quarantine_are_complete() {
        assert!(BARCODE_LANDS >= 8);
        assert!(COA_LANDS >= 3);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_ROUTE_SLOTS, ROUTE_COUNT);
        assert!(QUARANTINE_OUTLET_D > TUBE_BORE_D);
        assert_eq!(WASTE_BRANCH_INLETS, ROUTE_ROWS);
    }

    #[test]
    fn output_manifest_matches_source_only_modules() {
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert_eq!(LIMITATIONS.len(), 6);
    }

    #[test]
    fn main_assertions_cover_the_same_design_constraints() {
        assert_design_constraints();
    }
}
