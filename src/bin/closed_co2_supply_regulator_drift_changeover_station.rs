use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed CO2/O2 supply regulator drift and cylinder changeover station.
//
// This generator models validation packaging for bought, certified gas hardware
// used by a closed cell-culture cabinet. It keeps cylinder/regulator surrogates,
// dual regulator bays, pressure gauge witness pockets, zero/span reference gas
// lands, changeover controls, leak evidence, traceability, disposition lanes,
// and visible CSG label geometry mechanically repeatable. It is not pressure
// vessel, regulator, calibration protocol, or acceptance-limit design.

const OUTPUT_PREFIX: &str = "output/closed_co2_supply_regulator_drift_changeover_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_co2_supply_regulator_drift_changeover_station_base_closed_cabinet_leak_tray.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_cylinder_regulator_surrogate_restraints.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_dual_regulator_bays.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_pressure_gauge_witness_pockets.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_zero_span_gas_reference_lands.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_changeover_valve_panel.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_leak_witness_tray.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_barcode_certificate_lands.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_release_hold_reject_lanes.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_csg_label_geometry.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_robot_service_keepout_gauges.stl",
    "output/closed_co2_supply_regulator_drift_changeover_station_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 10] = [
    "cylinder_regulator_surrogate_restraints",
    "dual_regulator_bays",
    "pressure_gauge_witness_pockets",
    "zero_span_gas_reference_lands",
    "changeover_valve_panel",
    "leak_witness_tray",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "csg_label_geometry",
    "closed_cabinet_robot_service_keepouts",
];

const GAS_CHANNELS: usize = 2;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2"];
const SOURCES_PER_GAS: usize = 2;
const SOURCE_COUNT: usize = GAS_CHANNELS * SOURCES_PER_GAS;
const REGULATOR_BAY_COUNT: usize = SOURCE_COUNT;
const GAUGES_PER_REGULATOR: usize = 2;
const GAUGE_WITNESS_COUNT: usize = REGULATOR_BAY_COUNT * GAUGES_PER_REGULATOR;
const ZERO_SPAN_REFERENCE_LANDS: usize = 6;
const CHANGEOVER_SELECTORS: usize = GAS_CHANNELS;
const LEAK_WITNESS_WINDOWS: usize = SOURCE_COUNT + GAS_CHANNELS;
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 6;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;
const CSG_LABEL_COUNT: usize = 15;
const KEEP_OUT_GAUGES: usize = 5;

const STATION_X: f64 = 1480.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;

const GAS_PITCH_X: f64 = 610.0;
const SOURCE_ROW_PITCH_Y: f64 = 132.0;
const SOURCE_ROW_CENTER_Y: f64 = 178.0;

const CYLINDER_CRADLE_X: f64 = 260.0;
const CYLINDER_CRADLE_Y: f64 = 104.0;
const CYLINDER_RAIL_W: f64 = 18.0;
const CYLINDER_RAIL_Z: f64 = 38.0;
const CYLINDER_SURROGATE_D: f64 = 54.0;
const CYLINDER_FOOT_CUP_D: f64 = 86.0;
const STRAP_POST_Z: f64 = 140.0;
const REGULATOR_SURROGATE_X: f64 = 72.0;
const REGULATOR_SURROGATE_Y: f64 = 54.0;
const REGULATOR_SURROGATE_Z: f64 = 66.0;

const REG_PANEL_X: f64 = 820.0;
const REG_PANEL_Y: f64 = 44.0;
const REG_PANEL_Z: f64 = 250.0;
const REG_PANEL_POS: (f64, f64) = (0.0, 372.0);
const REG_BAY_X: f64 = 118.0;
const REG_BAY_Z: f64 = 96.0;
const REG_BAY_SOURCE_OFFSET_X: f64 = 74.0;
const REG_BAY_CENTER_Z: f64 = 158.0;

const GAUGE_PANEL_X: f64 = 760.0;
const GAUGE_PANEL_Y: f64 = 46.0;
const GAUGE_PANEL_Z: f64 = 140.0;
const GAUGE_PANEL_POS: (f64, f64) = (0.0, 318.0);
const GAUGE_D: f64 = 39.0;
const GAUGE_PAIR_OFFSET_Z: f64 = 30.0;

const REFERENCE_X: f64 = 390.0;
const REFERENCE_Y: f64 = 160.0;
const REFERENCE_Z: f64 = 42.0;
const REFERENCE_POS: (f64, f64) = (-460.0, -155.0);
const REFERENCE_PUCK_D: f64 = 38.0;
const REFERENCE_PITCH_X: f64 = 78.0;
const REFERENCE_PITCH_Y: f64 = 58.0;

const CHANGEOVER_X: f64 = 560.0;
const CHANGEOVER_Y: f64 = 150.0;
const CHANGEOVER_Z: f64 = 76.0;
const CHANGEOVER_POS: (f64, f64) = (0.0, 8.0);
const SELECTOR_X: f64 = 112.0;
const SELECTOR_Y: f64 = 48.0;
const SELECTOR_Z: f64 = 52.0;
const VALVE_HANDLE_D: f64 = 28.0;

const LEAK_TRAY_X: f64 = 560.0;
const LEAK_TRAY_Y: f64 = 130.0;
const LEAK_TRAY_Z: f64 = 34.0;
const LEAK_TRAY_POS: (f64, f64) = (0.0, -270.0);

const TRACE_X: f64 = 390.0;
const TRACE_Y: f64 = 160.0;
const TRACE_Z: f64 = 18.0;
const TRACE_POS: (f64, f64) = (460.0, -155.0);

const LANE_X: f64 = 520.0;
const LANE_Y: f64 = 94.0;
const LANE_Z: f64 = 36.0;
const LANE_POS: (f64, f64) = (0.0, -360.0);
const LANE_PITCH_X: f64 = 150.0;

const KEEP_OUT_X: f64 = 1390.0;
const KEEP_OUT_Y: f64 = 820.0;
const KEEP_OUT_Z: f64 = 8.0;
const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_CYLINDER_CLEARANCE: f64 = 330.0;
const SIDE_SERVICE_CLEARANCE: f64 = 190.0;
const TOP_REGULATOR_LIFT_CLEARANCE: f64 = 360.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }
}

#[derive(Clone, Copy, Debug)]
struct LabelSpec {
    id: &'static str,
    text: &'static str,
    center: (f64, f64),
    z: f64,
    width: f64,
    depth: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_closed_cabinet_leak_tray();
    export(OUTPUTS[0], &base);

    let restraints = cylinder_regulator_surrogate_restraints();
    export(OUTPUTS[1], &restraints);

    let regulator_bays = dual_regulator_bays();
    export(OUTPUTS[2], &regulator_bays);

    let gauge_pockets = pressure_gauge_witness_pockets();
    export(OUTPUTS[3], &gauge_pockets);

    let references = zero_span_gas_reference_lands();
    export(OUTPUTS[4], &references);

    let changeover = changeover_valve_panel();
    export(OUTPUTS[5], &changeover);

    let leak = leak_witness_tray();
    export(OUTPUTS[6], &leak);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[8], &disposition);

    let labels = csg_label_geometry();
    export(OUTPUTS[9], &labels);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + restraints
        + regulator_bays
        + gauge_pockets
        + references
        + changeover
        + leak
        + traceability
        + disposition
        + labels
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed CO2/O2 supply regulator drift and changeover station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm closed-cabinet leak tray"
    );
    println!(
        "  Supply channels:           {:?} with {SOURCE_COUNT} A/B cylinder-regulator surrogate restraints and {REGULATOR_BAY_COUNT} regulator bays",
        GAS_NAMES
    );
    println!(
        "  Drift witness interfaces:  {GAUGE_WITNESS_COUNT} inlet/outlet gauge witness pockets and {ZERO_SPAN_REFERENCE_LANDS} zero/span reference gas lands"
    );
    println!(
        "  Changeover controls:       {CHANGEOVER_SELECTORS} A/B selectors with lock tabs, valve witness windows, and repeatability token pockets"
    );
    println!(
        "  Evidence controls:         {LEAK_WITNESS_WINDOWS} leak witness windows, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, and CSG labels"
    );
    println!(
        "  Disposition:               release/hold/reject lanes with {DISPOSITION_SLOTS_PER_LANE} token slots per lane"
    );
    println!(
        "  Cabinet access gauges:     {FRONT_ROBOT_CLEARANCE:.0}mm front robot clearance, {REAR_CYLINDER_CLEARANCE:.0}mm rear cylinder clearance, {SIDE_SERVICE_CLEARANCE:.0}mm side service clearance, {TOP_REGULATOR_LIFT_CLEARANCE:.0}mm lift envelope"
    );
    println!(
        "  Feature groups covered:    {} validation packaging groups; no pressure-regulator design or release criteria",
        REQUIRED_FEATURE_GROUPS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z / 2.0
}

fn place_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn gas_x(gas: usize) -> f64 {
    centered_index(gas, GAS_CHANNELS, GAS_PITCH_X)
}

fn source_y(source: usize) -> f64 {
    SOURCE_ROW_CENTER_Y + centered_index(source, SOURCES_PER_GAS, SOURCE_ROW_PITCH_Y)
}

fn source_label(source: usize) -> &'static str {
    match source {
        0 => "a",
        1 => "b",
        _ => "unknown",
    }
}

fn source_offset_x(source: usize) -> f64 {
    centered_index(source, SOURCES_PER_GAS, REG_BAY_SOURCE_OFFSET_X * 2.0)
}

fn module_footprints() -> [Footprint; 8] {
    [
        Footprint {
            name: "cylinder_regulator_surrogate_restraints",
            center: (0.0, SOURCE_ROW_CENTER_Y),
            x: GAS_PITCH_X + CYLINDER_CRADLE_X,
            y: SOURCE_ROW_PITCH_Y + CYLINDER_CRADLE_Y,
        },
        Footprint {
            name: "dual_regulator_bays",
            center: REG_PANEL_POS,
            x: REG_PANEL_X,
            y: REG_PANEL_Y,
        },
        Footprint {
            name: "pressure_gauge_witness_pockets",
            center: GAUGE_PANEL_POS,
            x: GAUGE_PANEL_X,
            y: GAUGE_PANEL_Y,
        },
        Footprint {
            name: "zero_span_gas_reference_lands",
            center: REFERENCE_POS,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Footprint {
            name: "changeover_valve_panel",
            center: CHANGEOVER_POS,
            x: CHANGEOVER_X,
            y: CHANGEOVER_Y,
        },
        Footprint {
            name: "leak_witness_tray",
            center: LEAK_TRAY_POS,
            x: LEAK_TRAY_X,
            y: LEAK_TRAY_Y,
        },
        Footprint {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: LANE_POS,
            x: LANE_X,
            y: LANE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURE_GROUPS.len(), 10);
    assert_eq!(GAS_NAMES.len(), GAS_CHANNELS);
    assert_eq!(SOURCE_COUNT, GAS_CHANNELS * SOURCES_PER_GAS);
    assert_eq!(REGULATOR_BAY_COUNT, SOURCE_COUNT);
    assert_eq!(
        GAUGE_WITNESS_COUNT,
        REGULATOR_BAY_COUNT * GAUGES_PER_REGULATOR
    );
    assert_eq!(CHANGEOVER_SELECTORS, GAS_CHANNELS);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(label_specs().len(), CSG_LABEL_COUNT);
    assert!(FRONT_ROBOT_CLEARANCE >= 420.0);
    assert!(REAR_CYLINDER_CLEARANCE >= 320.0);
    assert!(SIDE_SERVICE_CLEARANCE >= 180.0);
    assert!(TOP_REGULATOR_LIFT_CLEARANCE >= REG_PANEL_Z + 100.0);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for required in [
        "cylinder_regulator_surrogate_restraints",
        "dual_regulator_bays",
        "pressure_gauge_witness_pockets",
        "zero_span_gas_reference_lands",
        "changeover_valve_panel",
        "leak_witness_tray",
        "barcode_certificate_lands",
        "release_hold_reject_lanes",
        "csg_label_geometry",
        "closed_cabinet_robot_service_keepouts",
    ] {
        assert!(REQUIRED_FEATURE_GROUPS.contains(&required));
    }

    for module in module_footprints() {
        assert!(
            module.fits_inside_station(),
            "{} exceeds closed cabinet tray envelope",
            module.name
        );
    }
}

fn base_closed_cabinet_leak_tray() -> Part {
    let deck = centered_cube(
        "co2_o2_regulator_changeover_closed_cabinet_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let spill_recess = centered_cube(
        "co2_o2_regulator_changeover_low_slope_spill_recess",
        STATION_X - 144.0,
        STATION_Y - 132.0,
        7.0,
    )
    .translate(0.0, -12.0, deck_top_z() - 3.5);
    let front_drain = centered_cylinder(
        "co2_o2_regulator_changeover_front_leak_tray_drain",
        6.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - spill_recess - front_drain - insert_sockets() - mounting_slots() - datum_pin_holes()
        + perimeter_rims()
        + cabinet_locator_rails()
        + zone_divider_ribs()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("co2_o2_regulator_changeover_insert_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("co2_o2_regulator_changeover_{}_socket", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                deck_top_z() - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("co2_o2_regulator_changeover_mounting_slots");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 55.0, -STATION_Y / 2.0 + 48.0),
        (STATION_X / 2.0 - 55.0, -STATION_Y / 2.0 + 48.0),
        (-STATION_X / 2.0 + 55.0, STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 55.0, STATION_Y / 2.0 - 48.0),
        (0.0, -STATION_Y / 2.0 + 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (-STATION_X / 2.0 + 55.0, 0.0),
        (STATION_X / 2.0 - 55.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_m6_clearance_{i}"),
                3.4,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("co2_o2_regulator_changeover_service_slot_{i}"),
                26.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("co2_o2_regulator_changeover_datum_pin_holes");
    for (i, (x, y)) in [
        (-610.0, 338.0),
        (610.0, 338.0),
        (-610.0, -328.0),
        (610.0, -328.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_datum_pin_clearance_{i}"),
                3.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "co2_o2_regulator_changeover_left_spill_retention_rim",
        RIM_W,
        STATION_Y - 42.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "co2_o2_regulator_changeover_right_spill_retention_rim",
        RIM_W,
        STATION_Y - 42.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "co2_o2_regulator_changeover_rear_cabinet_bulkhead_rim",
        STATION_X - 34.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let front_lip = centered_cube(
        "co2_o2_regulator_changeover_low_front_robot_service_lip",
        STATION_X - 220.0,
        12.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, deck_top_z() + 9.0);

    left + right + rear + front_lip
}

fn cabinet_locator_rails() -> Part {
    let rear_key = centered_cube(
        "co2_o2_regulator_changeover_rear_cabinet_locator_key",
        STATION_X - 260.0,
        12.0,
        18.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 72.0, deck_top_z() + 9.0);
    let left_slide = centered_cube(
        "co2_o2_regulator_changeover_left_closed_cabinet_slide_land",
        18.0,
        STATION_Y - 220.0,
        16.0,
    )
    .translate(-STATION_X / 2.0 + 82.0, 0.0, deck_top_z() + 8.0);
    let right_slide = centered_cube(
        "co2_o2_regulator_changeover_right_closed_cabinet_slide_land",
        18.0,
        STATION_Y - 220.0,
        16.0,
    )
    .translate(STATION_X / 2.0 - 82.0, 0.0, deck_top_z() + 8.0);
    rear_key + left_slide + right_slide
}

fn zone_divider_ribs() -> Part {
    let rear_separator = centered_cube(
        "co2_o2_regulator_changeover_rear_cylinder_regulator_zone_rib",
        STATION_X - 210.0,
        9.0,
        24.0,
    )
    .translate(0.0, 276.0, deck_top_z() + 12.0);
    let center_separator = centered_cube(
        "co2_o2_regulator_changeover_changeover_reference_zone_rib",
        STATION_X - 250.0,
        9.0,
        22.0,
    )
    .translate(0.0, -84.0, deck_top_z() + 11.0);
    let front_separator = centered_cube(
        "co2_o2_regulator_changeover_traceability_disposition_zone_rib",
        STATION_X - 310.0,
        8.0,
        20.0,
    )
    .translate(0.0, -325.0, deck_top_z() + 10.0);
    rear_separator + center_separator + front_separator
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("co2_o2_regulator_changeover_robot_datum_targets");
    for (i, (x, y)) in [(-620.0, -330.0), (620.0, -330.0), (-620.0, 330.0)]
        .iter()
        .enumerate()
    {
        let outer = centered_cylinder(
            format!("co2_o2_regulator_changeover_robot_fiducial_outer_{i}"),
            14.0,
            3.0,
            36,
        )
        .translate(*x, *y, deck_top_z() + 1.5);
        let inner = centered_cylinder(
            format!("co2_o2_regulator_changeover_robot_fiducial_inner_{i}"),
            6.0,
            4.0,
            28,
        )
        .translate(*x, *y, deck_top_z() + 1.5);
        targets = targets + (outer - inner);
    }
    targets
}

fn cylinder_regulator_surrogate_restraints() -> Part {
    let mut restraints = Part::empty("co2_o2_regulator_changeover_cylinder_regulator_restraints");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            restraints = restraints
                + cylinder_restraint(gas, source).translate(gas_x(gas), source_y(source), 0.0);
        }
    }
    restraints
}

fn cylinder_restraint(gas: usize, source: usize) -> Part {
    let gas_name = GAS_NAMES[gas];
    let source_name = source_label(source);
    let prefix = format!("co2_o2_regulator_changeover_{gas_name}_{source_name}");

    let left_rail = centered_cube(
        format!("{prefix}_left_v_rail"),
        CYLINDER_CRADLE_X,
        CYLINDER_RAIL_W,
        CYLINDER_RAIL_Z,
    )
    .translate(
        0.0,
        -CYLINDER_CRADLE_Y / 2.0 + CYLINDER_RAIL_W / 2.0,
        place_z(CYLINDER_RAIL_Z),
    );
    let right_rail = centered_cube(
        format!("{prefix}_right_v_rail"),
        CYLINDER_CRADLE_X,
        CYLINDER_RAIL_W,
        CYLINDER_RAIL_Z,
    )
    .translate(
        0.0,
        CYLINDER_CRADLE_Y / 2.0 - CYLINDER_RAIL_W / 2.0,
        place_z(CYLINDER_RAIL_Z),
    );
    let surrogate_shadow = centered_cylinder(
        format!("{prefix}_cylinder_surrogate_shadow"),
        CYLINDER_SURROGATE_D / 2.0,
        CYLINDER_CRADLE_X - 64.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-18.0, 0.0, deck_top_z() + 58.0);
    let rear_foot_cup = centered_cylinder(
        format!("{prefix}_rear_foot_cup_outer"),
        CYLINDER_FOOT_CUP_D / 2.0,
        12.0,
        48,
    )
    .translate(-CYLINDER_CRADLE_X / 2.0 + 50.0, 0.0, deck_top_z() + 6.0)
        - centered_cylinder(
            format!("{prefix}_rear_foot_cup_inner"),
            CYLINDER_FOOT_CUP_D / 2.0 - 12.0,
            14.0,
            48,
        )
        .translate(-CYLINDER_CRADLE_X / 2.0 + 50.0, 0.0, deck_top_z() + 6.0);
    let neck_stop = centered_cube(
        format!("{prefix}_front_neck_repeatability_stop"),
        22.0,
        CYLINDER_CRADLE_Y + 20.0,
        54.0,
    )
    .translate(CYLINDER_CRADLE_X / 2.0 - 18.0, 0.0, place_z(54.0));
    let regulator_surrogate = centered_cube(
        format!("{prefix}_purchased_regulator_surrogate_envelope"),
        REGULATOR_SURROGATE_X,
        REGULATOR_SURROGATE_Y,
        REGULATOR_SURROGATE_Z,
    )
    .translate(CYLINDER_CRADLE_X / 2.0 - 54.0, 0.0, deck_top_z() + 98.0);

    let mut strap_posts = Part::empty(format!("{prefix}_strap_and_lockout_posts"));
    for (i, x) in [
        -CYLINDER_CRADLE_X / 2.0 + 78.0,
        CYLINDER_CRADLE_X / 2.0 - 92.0,
    ]
    .iter()
    .enumerate()
    {
        for (j, y) in [
            -CYLINDER_CRADLE_Y / 2.0 - 13.0,
            CYLINDER_CRADLE_Y / 2.0 + 13.0,
        ]
        .iter()
        .enumerate()
        {
            strap_posts = strap_posts
                + centered_cylinder(
                    format!("{prefix}_strap_post_{i}_{j}"),
                    6.0,
                    STRAP_POST_Z,
                    24,
                )
                .translate(*x, *y, deck_top_z() + STRAP_POST_Z / 2.0);
        }
    }

    let strap_bridge = centered_cube(
        format!("{prefix}_tamper_strap_bridge"),
        CYLINDER_CRADLE_X - 82.0,
        10.0,
        20.0,
    )
    .translate(0.0, 0.0, deck_top_z() + STRAP_POST_Z - 16.0);
    let regulator_clamp = centered_cube(
        format!("{prefix}_regulator_surrogate_restraint_clamp"),
        REGULATOR_SURROGATE_X + 26.0,
        10.0,
        24.0,
    )
    .translate(
        CYLINDER_CRADLE_X / 2.0 - 54.0,
        CYLINDER_CRADLE_Y / 2.0 + 12.0,
        deck_top_z() + 112.0,
    );
    let line_grommet = centered_cylinder(format!("{prefix}_line_grommet_bore"), 5.0, 28.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(CYLINDER_CRADLE_X / 2.0 - 22.0, 0.0, deck_top_z() + 78.0);

    left_rail
        + right_rail
        + surrogate_shadow
        + rear_foot_cup
        + neck_stop
        + regulator_surrogate
        + strap_posts
        + strap_bridge
        + regulator_clamp
        - line_grommet
}

fn dual_regulator_bays() -> Part {
    let backplane = centered_cube(
        "co2_o2_regulator_changeover_dual_regulator_bay_backplane",
        REG_PANEL_X,
        REG_PANEL_Y,
        REG_PANEL_Z,
    )
    .translate(REG_PANEL_POS.0, REG_PANEL_POS.1, place_z(REG_PANEL_Z));

    let mut bay_cuts = Part::empty("co2_o2_regulator_changeover_regulator_bay_recesses");
    let mut shelves = Part::empty("co2_o2_regulator_changeover_regulator_bay_shelves");
    let mut dividers = Part::empty("co2_o2_regulator_changeover_regulator_bay_dividers");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = gas_x(gas) + source_offset_x(source);
            let source_name = source_label(source);
            let gas_name = GAS_NAMES[gas];
            bay_cuts = bay_cuts
                + centered_cube(
                    format!("co2_o2_regulator_changeover_{gas_name}_{source_name}_regulator_bay_recess"),
                    REG_BAY_X,
                    REG_PANEL_Y + 8.0,
                    REG_BAY_Z,
                )
                .translate(
                    x,
                    REG_PANEL_POS.1,
                    deck_top_z() + REG_BAY_CENTER_Z,
                )
                + centered_cylinder(
                    format!("co2_o2_regulator_changeover_{gas_name}_{source_name}_regulator_thread_clearance"),
                    9.0,
                    REG_PANEL_Y + 12.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, REG_PANEL_POS.1, deck_top_z() + 94.0);

            shelves = shelves
                + centered_cube(
                    format!("co2_o2_regulator_changeover_{gas_name}_{source_name}_lower_regulator_shelf"),
                    REG_BAY_X + 34.0,
                    18.0,
                    18.0,
                )
                .translate(
                    x,
                    REG_PANEL_POS.1 - REG_PANEL_Y / 2.0 - 12.0,
                    deck_top_z() + 74.0,
                )
                + centered_cube(
                    format!("co2_o2_regulator_changeover_{gas_name}_{source_name}_upper_regulator_capture"),
                    REG_BAY_X + 24.0,
                    12.0,
                    18.0,
                )
                .translate(
                    x,
                    REG_PANEL_POS.1 - REG_PANEL_Y / 2.0 - 10.0,
                    deck_top_z() + 214.0,
                );
        }

        dividers = dividers
            + centered_cube(
                format!(
                    "co2_o2_regulator_changeover_{}_a_b_bay_center_divider",
                    GAS_NAMES[gas]
                ),
                10.0,
                REG_PANEL_Y + 20.0,
                REG_PANEL_Z - 40.0,
            )
            .translate(
                gas_x(gas),
                REG_PANEL_POS.1 - 4.0,
                deck_top_z() + REG_PANEL_Z / 2.0,
            );
    }

    let gas_center_divider = centered_cube(
        "co2_o2_regulator_changeover_co2_o2_regulator_bay_center_divider",
        12.0,
        REG_PANEL_Y + 24.0,
        REG_PANEL_Z - 22.0,
    )
    .translate(0.0, REG_PANEL_POS.1 - 4.0, place_z(REG_PANEL_Z));

    backplane - bay_cuts + shelves + dividers + gas_center_divider
}

fn pressure_gauge_witness_pockets() -> Part {
    let witness_panel = centered_cube(
        "co2_o2_regulator_changeover_pressure_gauge_witness_panel",
        GAUGE_PANEL_X,
        GAUGE_PANEL_Y,
        GAUGE_PANEL_Z,
    )
    .translate(GAUGE_PANEL_POS.0, GAUGE_PANEL_POS.1, place_z(GAUGE_PANEL_Z));

    let mut gauge_cuts = Part::empty("co2_o2_regulator_changeover_pressure_gauge_face_clearances");
    let mut witness_rings = Part::empty("co2_o2_regulator_changeover_pressure_gauge_witness_rings");
    let mut pointers =
        Part::empty("co2_o2_regulator_changeover_pressure_gauge_drift_pointer_ticks");

    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            for gauge in 0..GAUGES_PER_REGULATOR {
                let x = gas_x(gas) + source_offset_x(source);
                let z = deck_top_z()
                    + 90.0
                    + centered_index(gauge, GAUGES_PER_REGULATOR, GAUGE_PAIR_OFFSET_Z * 2.0);
                let gas_name = GAS_NAMES[gas];
                let source_name = source_label(source);
                let gauge_name = if gauge == 0 { "inlet" } else { "outlet" };

                gauge_cuts = gauge_cuts
                    + centered_cylinder(
                        format!(
                            "co2_o2_regulator_changeover_{gas_name}_{source_name}_{gauge_name}_gauge_face_clearance"
                        ),
                        GAUGE_D / 2.0,
                        GAUGE_PANEL_Y + 10.0,
                        44,
                    )
                    .rotate(90.0, 0.0, 0.0)
                    .translate(x, GAUGE_PANEL_POS.1, z);

                let ring = centered_cylinder(
                    format!(
                        "co2_o2_regulator_changeover_{gas_name}_{source_name}_{gauge_name}_gauge_witness_outer_ring"
                    ),
                    GAUGE_D / 2.0 + 6.0,
                    5.0,
                    44,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, GAUGE_PANEL_POS.1 - GAUGE_PANEL_Y / 2.0 - 4.0, z)
                    - centered_cylinder(
                        format!(
                            "co2_o2_regulator_changeover_{gas_name}_{source_name}_{gauge_name}_gauge_witness_inner_ring"
                        ),
                        GAUGE_D / 2.0 - 2.0,
                        7.0,
                        44,
                    )
                    .rotate(90.0, 0.0, 0.0)
                    .translate(x, GAUGE_PANEL_POS.1 - GAUGE_PANEL_Y / 2.0 - 4.0, z);
                witness_rings = witness_rings + ring;

                pointers = pointers
                    + centered_cube(
                        format!(
                            "co2_o2_regulator_changeover_{gas_name}_{source_name}_{gauge_name}_zero_pointer_tick"
                        ),
                        5.0,
                        4.0,
                        22.0,
                    )
                    .translate(
                        x - GAUGE_D / 2.0 - 10.0,
                        GAUGE_PANEL_POS.1 - GAUGE_PANEL_Y / 2.0 - 8.0,
                        z,
                    )
                    + centered_cube(
                        format!(
                            "co2_o2_regulator_changeover_{gas_name}_{source_name}_{gauge_name}_span_pointer_tick"
                        ),
                        5.0,
                        4.0,
                        22.0,
                    )
                    .translate(
                        x + GAUGE_D / 2.0 + 10.0,
                        GAUGE_PANEL_POS.1 - GAUGE_PANEL_Y / 2.0 - 8.0,
                        z,
                    );
            }
        }
    }

    witness_panel - gauge_cuts + witness_rings + pointers
}

fn zero_span_gas_reference_lands() -> Part {
    let plate = centered_cube(
        "co2_o2_regulator_changeover_zero_span_reference_land_plate",
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    )
    .translate(REFERENCE_POS.0, REFERENCE_POS.1, place_z(REFERENCE_Z));

    let mut puck_recesses = Part::empty("co2_o2_regulator_changeover_zero_span_reference_recesses");
    let mut rings = Part::empty("co2_o2_regulator_changeover_zero_span_reference_retention_rings");
    let mut cap_posts = Part::empty("co2_o2_regulator_changeover_zero_span_cap_tether_posts");
    for i in 0..ZERO_SPAN_REFERENCE_LANDS {
        let row = i / 3;
        let col = i % 3;
        let x = REFERENCE_POS.0 + centered_index(col, 3, REFERENCE_PITCH_X);
        let y = REFERENCE_POS.1 + centered_index(row, 2, REFERENCE_PITCH_Y);
        let land_name = reference_land_name(i);

        puck_recesses = puck_recesses
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{land_name}_reference_puck_recess"),
                REFERENCE_PUCK_D / 2.0,
                REFERENCE_Z + 4.0,
                44,
            )
            .translate(x, y, place_z(REFERENCE_Z));

        rings = rings
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{land_name}_reference_puck_retention_ring"),
                REFERENCE_PUCK_D / 2.0 + 7.0,
                5.0,
                44,
            )
            .translate(x, y, deck_top_z() + REFERENCE_Z + 2.5)
            - centered_cylinder(
                format!("co2_o2_regulator_changeover_{land_name}_reference_puck_opening"),
                REFERENCE_PUCK_D / 2.0 - 2.0,
                7.0,
                44,
            )
            .translate(x, y, deck_top_z() + REFERENCE_Z + 2.5);

        cap_posts = cap_posts
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{land_name}_cap_tether_post"),
                5.0,
                20.0,
                24,
            )
            .translate(x + 31.0, y + 20.0, deck_top_z() + REFERENCE_Z + 10.0);
    }

    let divider = centered_cube(
        "co2_o2_regulator_changeover_zero_span_reference_co2_o2_split_rib",
        8.0,
        REFERENCE_Y - 28.0,
        22.0,
    )
    .translate(
        REFERENCE_POS.0,
        REFERENCE_POS.1,
        deck_top_z() + REFERENCE_Z + 11.0,
    );
    let qc_rail = centered_cube(
        "co2_o2_regulator_changeover_reference_gas_qc_rail",
        REFERENCE_X - 72.0,
        12.0,
        18.0,
    )
    .translate(
        REFERENCE_POS.0,
        REFERENCE_POS.1 - REFERENCE_Y / 2.0 + 18.0,
        deck_top_z() + REFERENCE_Z + 9.0,
    );

    plate - puck_recesses + rings + cap_posts + divider + qc_rail
}

fn reference_land_name(index: usize) -> &'static str {
    match index {
        0 => "co2_zero",
        1 => "co2_low_span",
        2 => "co2_high_span",
        3 => "o2_zero",
        4 => "o2_low_span",
        5 => "o2_high_span",
        _ => "reference",
    }
}

fn changeover_valve_panel() -> Part {
    let body = centered_cube(
        "co2_o2_regulator_changeover_valve_panel_body",
        CHANGEOVER_X,
        CHANGEOVER_Y,
        CHANGEOVER_Z,
    )
    .translate(CHANGEOVER_POS.0, CHANGEOVER_POS.1, place_z(CHANGEOVER_Z));

    let mut selectors = Part::empty("co2_o2_regulator_changeover_ab_selector_controls");
    let mut route_channels = Part::empty("co2_o2_regulator_changeover_ab_route_channel_bores");
    let mut lockouts = Part::empty("co2_o2_regulator_changeover_ab_lockout_features");

    for gas in 0..GAS_CHANNELS {
        let x = centered_index(gas, GAS_CHANNELS, 250.0);
        let gas_name = GAS_NAMES[gas];
        selectors = selectors
            + centered_cube(
                format!("co2_o2_regulator_changeover_{gas_name}_selector_valve_block"),
                SELECTOR_X,
                SELECTOR_Y,
                SELECTOR_Z,
            )
            .translate(
                CHANGEOVER_POS.0 + x,
                CHANGEOVER_POS.1,
                deck_top_z() + CHANGEOVER_Z + SELECTOR_Z / 2.0,
            )
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{gas_name}_selector_handle_disc"),
                VALVE_HANDLE_D / 2.0,
                10.0,
                36,
            )
            .translate(
                CHANGEOVER_POS.0 + x,
                CHANGEOVER_POS.1,
                deck_top_z() + CHANGEOVER_Z + SELECTOR_Z + 5.0,
            )
            + centered_cube(
                format!("co2_o2_regulator_changeover_{gas_name}_selector_pointer_bar"),
                62.0,
                7.0,
                8.0,
            )
            .translate(
                CHANGEOVER_POS.0 + x,
                CHANGEOVER_POS.1,
                deck_top_z() + CHANGEOVER_Z + SELECTOR_Z + 12.0,
            );

        route_channels = route_channels
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{gas_name}_source_a_route_bore"),
                4.2,
                CHANGEOVER_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                CHANGEOVER_POS.0 + x - 32.0,
                CHANGEOVER_POS.1,
                deck_top_z() + 36.0,
            )
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{gas_name}_source_b_route_bore"),
                4.2,
                CHANGEOVER_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                CHANGEOVER_POS.0 + x + 32.0,
                CHANGEOVER_POS.1,
                deck_top_z() + 36.0,
            );

        lockouts = lockouts
            + centered_cube(
                format!("co2_o2_regulator_changeover_{gas_name}_ab_position_witness_window"),
                82.0,
                8.0,
                20.0,
            )
            .translate(
                CHANGEOVER_POS.0 + x,
                CHANGEOVER_POS.1 - CHANGEOVER_Y / 2.0 - 6.0,
                deck_top_z() + CHANGEOVER_Z + 10.0,
            )
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_{gas_name}_lockout_pin_socket"),
                5.0,
                22.0,
                24,
            )
            .translate(
                CHANGEOVER_POS.0 + x + 58.0,
                CHANGEOVER_POS.1 + CHANGEOVER_Y / 2.0 - 26.0,
                deck_top_z() + CHANGEOVER_Z + 11.0,
            );
    }

    let common_outlet = centered_cube(
        "co2_o2_regulator_changeover_common_cabinet_outlet_manifold_land",
        CHANGEOVER_X - 90.0,
        16.0,
        18.0,
    )
    .translate(
        CHANGEOVER_POS.0,
        CHANGEOVER_POS.1 - CHANGEOVER_Y / 2.0 + 28.0,
        deck_top_z() + CHANGEOVER_Z + 9.0,
    );
    let repeatability_token_pockets = centered_cube(
        "co2_o2_regulator_changeover_repeatability_run_token_strip",
        CHANGEOVER_X - 120.0,
        16.0,
        14.0,
    )
    .translate(
        CHANGEOVER_POS.0,
        CHANGEOVER_POS.1 + CHANGEOVER_Y / 2.0 - 24.0,
        deck_top_z() + CHANGEOVER_Z + 7.0,
    );

    body - route_channels + selectors + lockouts + common_outlet + repeatability_token_pockets
}

fn leak_witness_tray() -> Part {
    let tray = centered_cube(
        "co2_o2_regulator_changeover_leak_witness_tray_body",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    )
    .translate(LEAK_TRAY_POS.0, LEAK_TRAY_POS.1, place_z(LEAK_TRAY_Z));
    let recess = centered_cube(
        "co2_o2_regulator_changeover_leak_witness_absorbent_recess",
        LEAK_TRAY_X - 74.0,
        LEAK_TRAY_Y - 42.0,
        16.0,
    )
    .translate(
        LEAK_TRAY_POS.0,
        LEAK_TRAY_POS.1,
        deck_top_z() + LEAK_TRAY_Z - 8.0,
    );

    let mut windows = Part::empty("co2_o2_regulator_changeover_leak_witness_windows");
    for i in 0..LEAK_WITNESS_WINDOWS {
        let x = LEAK_TRAY_POS.0 + centered_index(i, LEAK_WITNESS_WINDOWS, 74.0);
        windows = windows
            + centered_cube(
                format!("co2_o2_regulator_changeover_leak_witness_window_{i}"),
                46.0,
                10.0,
                18.0,
            )
            .translate(
                x,
                LEAK_TRAY_POS.1 - LEAK_TRAY_Y / 2.0 + 22.0,
                deck_top_z() + LEAK_TRAY_Z + 9.0,
            )
            + centered_cylinder(
                format!("co2_o2_regulator_changeover_leak_witness_drip_port_{i}"),
                4.5,
                20.0,
                22,
            )
            .translate(
                x,
                LEAK_TRAY_POS.1 + LEAK_TRAY_Y / 2.0 - 24.0,
                deck_top_z() + LEAK_TRAY_Z + 10.0,
            );
    }

    let left_sump = centered_cube(
        "co2_o2_regulator_changeover_left_leak_sump_barrier",
        14.0,
        LEAK_TRAY_Y - 24.0,
        28.0,
    )
    .translate(
        LEAK_TRAY_POS.0 - LEAK_TRAY_X / 2.0 + 30.0,
        LEAK_TRAY_POS.1,
        deck_top_z() + LEAK_TRAY_Z + 14.0,
    );
    let right_sump = centered_cube(
        "co2_o2_regulator_changeover_right_leak_sump_barrier",
        14.0,
        LEAK_TRAY_Y - 24.0,
        28.0,
    )
    .translate(
        LEAK_TRAY_POS.0 + LEAK_TRAY_X / 2.0 - 30.0,
        LEAK_TRAY_POS.1,
        deck_top_z() + LEAK_TRAY_Z + 14.0,
    );

    tray - recess + windows + left_sump + right_sump
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "co2_o2_regulator_changeover_traceability_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, place_z(TRACE_Z));

    let mut barcode_recesses = Part::empty("co2_o2_regulator_changeover_barcode_recesses");
    for i in 0..BARCODE_LANDS {
        let row = i / 5;
        let col = i % 5;
        barcode_recesses = barcode_recesses
            + centered_cube(
                format!("co2_o2_regulator_changeover_barcode_land_{i}"),
                58.0,
                18.0,
                TRACE_Z + 4.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(col, 5, 68.0),
                TRACE_POS.1 + 42.0 - row as f64 * 40.0,
                place_z(TRACE_Z),
            );
    }

    let mut certificate_slots = Part::empty("co2_o2_regulator_changeover_certificate_slots");
    for i in 0..CERTIFICATE_LANDS {
        let col = i % 3;
        let row = i / 3;
        certificate_slots = certificate_slots
            + centered_cube(
                format!("co2_o2_regulator_changeover_certificate_card_slot_{i}"),
                72.0,
                5.0,
                TRACE_Z + 10.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(col, 3, 94.0),
                TRACE_POS.1 - 36.0 + row as f64 * 20.0,
                place_z(TRACE_Z),
            );
    }

    let custody_witness_strip = centered_cube(
        "co2_o2_regulator_changeover_certificate_expiry_witness_strip",
        TRACE_X - 70.0,
        12.0,
        14.0,
    )
    .translate(
        TRACE_POS.0,
        TRACE_POS.1 - TRACE_Y / 2.0 + 20.0,
        deck_top_z() + TRACE_Z + 7.0,
    );
    let quarantine_card_pocket = centered_cube(
        "co2_o2_regulator_changeover_quarantine_certificate_pocket",
        120.0,
        22.0,
        14.0,
    )
    .translate(
        TRACE_POS.0 + TRACE_X / 2.0 - 82.0,
        TRACE_POS.1 + TRACE_Y / 2.0 - 22.0,
        deck_top_z() + TRACE_Z + 7.0,
    );

    plate - barcode_recesses - certificate_slots + custody_witness_strip + quarantine_card_pocket
}

fn release_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "co2_o2_regulator_changeover_release_hold_reject_lane_tray",
        LANE_X,
        LANE_Y,
        LANE_Z,
    )
    .translate(LANE_POS.0, LANE_POS.1, place_z(LANE_Z));

    let mut lane_slots = Part::empty("co2_o2_regulator_changeover_disposition_token_slots");
    let mut lane_labels = Part::empty("co2_o2_regulator_changeover_disposition_lane_label_lands");
    for lane in 0..DISPOSITION_LANES {
        let x = LANE_POS.0 + centered_index(lane, DISPOSITION_LANES, LANE_PITCH_X);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            lane_slots = lane_slots
                + centered_cube(
                    format!(
                        "co2_o2_regulator_changeover_{}_token_slot_{slot}",
                        disposition_name(lane)
                    ),
                    52.0,
                    15.0,
                    LANE_Z + 6.0,
                )
                .translate(
                    x,
                    LANE_POS.1 + centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 18.0),
                    place_z(LANE_Z),
                );
        }
        lane_labels = lane_labels
            + centered_cube(
                format!(
                    "co2_o2_regulator_changeover_{}_raised_lane_label_land",
                    disposition_name(lane)
                ),
                92.0,
                12.0,
                10.0,
            )
            .translate(
                x,
                LANE_POS.1 + LANE_Y / 2.0 - 12.0,
                deck_top_z() + LANE_Z + 5.0,
            );
    }

    let lane_dividers = centered_cube(
        "co2_o2_regulator_changeover_release_hold_divider",
        8.0,
        LANE_Y - 22.0,
        22.0,
    )
    .translate(
        LANE_POS.0 - LANE_PITCH_X / 2.0,
        LANE_POS.1,
        deck_top_z() + LANE_Z + 11.0,
    ) + centered_cube(
        "co2_o2_regulator_changeover_hold_reject_divider",
        8.0,
        LANE_Y - 22.0,
        22.0,
    )
    .translate(
        LANE_POS.0 + LANE_PITCH_X / 2.0,
        LANE_POS.1,
        deck_top_z() + LANE_Z + 11.0,
    );

    tray - lane_slots + lane_labels + lane_dividers
}

fn disposition_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn csg_label_geometry() -> Part {
    let mut labels = Part::empty("co2_o2_regulator_changeover_all_csg_labels");
    for spec in label_specs() {
        labels = labels
            + csg_label_tag(spec.id, spec.text, spec.width, spec.depth).translate(
                spec.center.0,
                spec.center.1,
                spec.z,
            );
    }
    labels
}

fn label_specs() -> [LabelSpec; CSG_LABEL_COUNT] {
    [
        LabelSpec {
            id: "co2_a_cylinder_label",
            text: "CO2-A",
            center: (gas_x(0), source_y(0) - 58.0),
            z: deck_top_z() + 6.0,
            width: 68.0,
            depth: 24.0,
        },
        LabelSpec {
            id: "co2_b_cylinder_label",
            text: "CO2-B",
            center: (gas_x(0), source_y(1) - 58.0),
            z: deck_top_z() + 6.0,
            width: 68.0,
            depth: 24.0,
        },
        LabelSpec {
            id: "o2_a_cylinder_label",
            text: "O2-A",
            center: (gas_x(1), source_y(0) - 58.0),
            z: deck_top_z() + 6.0,
            width: 68.0,
            depth: 24.0,
        },
        LabelSpec {
            id: "o2_b_cylinder_label",
            text: "O2-B",
            center: (gas_x(1), source_y(1) - 58.0),
            z: deck_top_z() + 6.0,
            width: 68.0,
            depth: 24.0,
        },
        LabelSpec {
            id: "co2_zero_label",
            text: "CO2 ZERO",
            center: (REFERENCE_POS.0 - 92.0, REFERENCE_POS.1 + 76.0),
            z: deck_top_z() + REFERENCE_Z + 4.0,
            width: 98.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "co2_span_label",
            text: "CO2 SPAN",
            center: (REFERENCE_POS.0 + 30.0, REFERENCE_POS.1 + 76.0),
            z: deck_top_z() + REFERENCE_Z + 4.0,
            width: 104.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "o2_zero_label",
            text: "O2 ZERO",
            center: (REFERENCE_POS.0 - 92.0, REFERENCE_POS.1 - 76.0),
            z: deck_top_z() + REFERENCE_Z + 4.0,
            width: 96.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "o2_span_label",
            text: "O2 SPAN",
            center: (REFERENCE_POS.0 + 30.0, REFERENCE_POS.1 - 76.0),
            z: deck_top_z() + REFERENCE_Z + 4.0,
            width: 100.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "co2_changeover_label",
            text: "CO2 A/B",
            center: (CHANGEOVER_POS.0 - 125.0, CHANGEOVER_POS.1 - 82.0),
            z: deck_top_z() + CHANGEOVER_Z + 4.0,
            width: 92.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "o2_changeover_label",
            text: "O2 A/B",
            center: (CHANGEOVER_POS.0 + 125.0, CHANGEOVER_POS.1 - 82.0),
            z: deck_top_z() + CHANGEOVER_Z + 4.0,
            width: 86.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "leak_witness_label",
            text: "LEAK",
            center: (LEAK_TRAY_POS.0 - 226.0, LEAK_TRAY_POS.1 + 64.0),
            z: deck_top_z() + LEAK_TRAY_Z + 4.0,
            width: 70.0,
            depth: 20.0,
        },
        LabelSpec {
            id: "release_lane_label",
            text: "RELEASE",
            center: (LANE_POS.0 - LANE_PITCH_X, LANE_POS.1 + 54.0),
            z: deck_top_z() + LANE_Z + 4.0,
            width: 88.0,
            depth: 18.0,
        },
        LabelSpec {
            id: "hold_lane_label",
            text: "HOLD",
            center: (LANE_POS.0, LANE_POS.1 + 54.0),
            z: deck_top_z() + LANE_Z + 4.0,
            width: 68.0,
            depth: 18.0,
        },
        LabelSpec {
            id: "reject_lane_label",
            text: "REJECT",
            center: (LANE_POS.0 + LANE_PITCH_X, LANE_POS.1 + 54.0),
            z: deck_top_z() + LANE_Z + 4.0,
            width: 82.0,
            depth: 18.0,
        },
        LabelSpec {
            id: "traceability_label",
            text: "CERTS",
            center: (TRACE_POS.0 - 122.0, TRACE_POS.1 - 74.0),
            z: deck_top_z() + TRACE_Z + 4.0,
            width: 82.0,
            depth: 20.0,
        },
    ]
}

fn csg_label_tag(id: &str, text: &str, width: f64, depth: f64) -> Part {
    let plate = centered_cube(
        format!("co2_o2_regulator_changeover_{id}_plate"),
        width,
        depth,
        2.0,
    );
    let underline = centered_cube(
        format!("co2_o2_regulator_changeover_{id}_underline"),
        width - 10.0,
        2.0,
        1.4,
    )
    .translate(0.0, -depth / 2.0 + 4.0, 1.7);

    let mut glyphs = Part::empty(format!(
        "co2_o2_regulator_changeover_{id}_raised_csg_glyph_bars"
    ));
    let max_columns = ((width - 14.0) / 5.0).floor() as usize;
    for (column, byte) in text.bytes().take(max_columns).enumerate() {
        for row in 0..5 {
            if ((byte >> (row % 6)) & 1) == 1 {
                glyphs = glyphs
                    + centered_cube(
                        format!("co2_o2_regulator_changeover_{id}_glyph_{column}_{row}"),
                        3.0,
                        2.0,
                        1.2,
                    )
                    .translate(
                        -width / 2.0 + 8.0 + column as f64 * 5.0,
                        -depth / 2.0 + 8.0 + row as f64 * 2.7,
                        1.8,
                    );
            }
        }
    }

    plate + underline + glyphs
}

fn robot_service_keepout_gauges() -> Part {
    let outer = centered_cube(
        "co2_o2_regulator_changeover_keepout_outer_closed_cabinet_envelope",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let inner = centered_cube(
        "co2_o2_regulator_changeover_keepout_inner_working_envelope_void",
        KEEP_OUT_X - 44.0,
        KEEP_OUT_Y - 44.0,
        KEEP_OUT_Z + 2.0,
    )
    .translate(0.0, 0.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let frame = outer - inner;

    let front_robot_bar = centered_cube(
        "co2_o2_regulator_changeover_front_robot_approach_clearance_gauge",
        640.0,
        12.0,
        28.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        deck_top_z() + 14.0,
    );
    let rear_cylinder_bar = centered_cube(
        "co2_o2_regulator_changeover_rear_cylinder_change_clearance_gauge",
        760.0,
        12.0,
        28.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_CYLINDER_CLEARANCE,
        deck_top_z() + 14.0,
    );
    let left_service_bar = centered_cube(
        "co2_o2_regulator_changeover_left_service_cart_clearance_gauge",
        12.0,
        460.0,
        28.0,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE,
        0.0,
        deck_top_z() + 14.0,
    );
    let right_service_bar = centered_cube(
        "co2_o2_regulator_changeover_right_service_cart_clearance_gauge",
        12.0,
        460.0,
        28.0,
    )
    .translate(
        STATION_X / 2.0 - SIDE_SERVICE_CLEARANCE,
        0.0,
        deck_top_z() + 14.0,
    );
    let lift_gauge = centered_cube(
        "co2_o2_regulator_changeover_top_regulator_lift_clearance_gauge",
        120.0,
        120.0,
        6.0,
    )
    .translate(
        0.0,
        REG_PANEL_POS.1 - 20.0,
        deck_top_z() + TOP_REGULATOR_LIFT_CLEARANCE,
    );

    assert_eq!(KEEP_OUT_GAUGES, 5);
    frame + front_robot_bar + rear_cylinder_bar + left_service_bar + right_service_bar + lift_gauge
}
