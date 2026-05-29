use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media formulation and sterile additive mixing station for the automated
// tissue-chip culture workcell.
//
// Research basis:
// - Modern media/buffer preparation platforms use closed single-use flowpaths,
//   recipe-controlled additions, gravimetric formulation, low-shear mixing,
//   sterile filtration, and inline pH/conductivity checks.
// - Single-use mixers are normally bought/qualified assemblies; this CAD models
//   the station interfaces around them: bag docks, load-cell pads, connector
//   bulkheads, line management, sampling handoffs, status lanes, and keepouts.
// - Chilled additives and released/hold/reject segregation are modeled as
//   mechanical controls because reagent history and disposition are major
//   reproducibility variables.
//
// This file is mechanical architecture CAD only. It is not a validated media
// recipe, sterile claim, filtration validation, or process analytical method.

const OUTPUTS: [&str; 13] = [
    "output/closed_media_formulation_mixing_station_base_leak_tray.stl",
    "output/closed_media_formulation_mixing_station_basal_media_bag_dock.stl",
    "output/closed_media_formulation_mixing_station_additive_sterile_connector_bay.stl",
    "output/closed_media_formulation_mixing_station_gravimetric_load_cell_pad_matrix.stl",
    "output/closed_media_formulation_mixing_station_gentle_rocker_mixer_envelope.stl",
    "output/closed_media_formulation_mixing_station_sterile_filter_vent_manifold.stl",
    "output/closed_media_formulation_mixing_station_qc_sample_loop_handoff.stl",
    "output/closed_media_formulation_mixing_station_barcode_coa_scan_lands.stl",
    "output/closed_media_formulation_mixing_station_released_hold_reject_status_lanes.stl",
    "output/closed_media_formulation_mixing_station_chilled_additive_pocket.stl",
    "output/closed_media_formulation_mixing_station_tubing_strain_relief_and_routes.stl",
    "output/closed_media_formulation_mixing_station_robot_service_keepouts.stl",
    "output/closed_media_formulation_mixing_station_assembly.stl",
];

const DECK_X: f64 = 1460.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 24.0;
const LEAK_RIM_W: f64 = 22.0;
const LEAK_RIM_Z: f64 = 38.0;
const SUMP_X: f64 = 1300.0;
const SUMP_Y: f64 = 760.0;
const SUMP_Z: f64 = 9.0;
const DRAIN_PORT_D: f64 = 18.0;
const DATUM_BOSS_COUNT: usize = 8;

const BASAL_BAG_DOCK_X: f64 = 440.0;
const BASAL_BAG_DOCK_Y: f64 = 292.0;
const BASAL_BAG_DOCK_Z: f64 = 58.0;
const BASAL_DOCK_X: f64 = -460.0;
const BASAL_DOCK_Y: f64 = 76.0;
const BASAL_MEDIA_BAGS: usize = 2;
const BASAL_BAG_X: f64 = 174.0;
const BASAL_BAG_Y: f64 = 228.0;
const BASAL_BAG_Z: f64 = 126.0;
const BASAL_BAG_PITCH_X: f64 = 204.0;

const ADDITIVE_BAY_X: f64 = 610.0;
const ADDITIVE_BAY_Y: f64 = 164.0;
const ADDITIVE_BAY_Z: f64 = 84.0;
const ADDITIVE_BAY_CENTER_X: f64 = -330.0;
const ADDITIVE_BAY_CENTER_Y: f64 = 304.0;
const ADDITIVE_CONNECTOR_PORTS: usize = 12;
const ADDITIVE_CONNECTOR_PITCH_X: f64 = 46.0;
const ADDITIVE_CONNECTOR_D: f64 = 19.0;
const ADDITIVE_COLLAR_D: f64 = 34.0;
const ADDITIVE_VIAL_ROWS: usize = 2;
const ADDITIVE_VIAL_COLS: usize = 6;
const ADDITIVE_VIAL_COUNT: usize = ADDITIVE_VIAL_ROWS * ADDITIVE_VIAL_COLS;
const ADDITIVE_VIAL_WELL_D: f64 = 24.0;
const ADDITIVE_VIAL_PITCH_X: f64 = 46.0;
const ADDITIVE_VIAL_PITCH_Y: f64 = 48.0;

const BULKHEAD_X: f64 = 1240.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 230.0;
const BULKHEAD_CENTER_Y: f64 = DECK_Y / 2.0 - 58.0;
const BULKHEAD_CONNECTOR_PORTS: usize = 18;
const BULKHEAD_CONNECTOR_PITCH_X: f64 = 62.0;
const VENT_FILTERS: usize = 4;

const LOAD_CELL_PADS: usize = 10;
const LOAD_CELL_PAD_X: f64 = 72.0;
const LOAD_CELL_PAD_Y: f64 = 52.0;
const LOAD_CELL_PAD_Z: f64 = 12.0;

const MIXER_X: f64 = 430.0;
const MIXER_Y: f64 = 330.0;
const MIXER_BASE_Z: f64 = 82.0;
const MIXER_BAG_Z: f64 = 178.0;
const MIXER_CENTER_X: f64 = 20.0;
const MIXER_CENTER_Y: f64 = 84.0;
const ROCKER_SWING_X: f64 = 510.0;
const ROCKER_SWING_Y: f64 = 392.0;
const ROCKER_SWING_Z: f64 = 254.0;
const ROCKER_AXIS_D: f64 = 24.0;
const MIXER_NOMINAL_VOLUME_L: f64 = 10.0;

const FILTER_MANIFOLD_X: f64 = 330.0;
const FILTER_MANIFOLD_Y: f64 = 132.0;
const FILTER_MANIFOLD_Z: f64 = 78.0;
const FILTER_CENTER_X: f64 = 500.0;
const FILTER_CENTER_Y: f64 = 146.0;
const FILTER_CAPSULES: usize = 3;
const FILTER_CAPSULE_D: f64 = 42.0;
const FILTER_CAPSULE_LENGTH: f64 = 182.0;
const PRESSURE_TAPS: usize = 4;

const QC_BLOCK_X: f64 = 356.0;
const QC_BLOCK_Y: f64 = 178.0;
const QC_BLOCK_Z: f64 = 68.0;
const QC_CENTER_X: f64 = 418.0;
const QC_CENTER_Y: f64 = -172.0;
const QC_CHANNELS: usize = 3; // pH, osmolality, conductivity.
const QC_CHANNEL_PITCH_X: f64 = 86.0;
const QC_SAMPLE_BAGS: usize = 3;
const QC_HANDOFF_PORTS: usize = 6;

const STATUS_LANES: usize = 3; // released, hold, reject.
const STATUS_LANE_X: f64 = 166.0;
const STATUS_LANE_Y: f64 = 132.0;
const STATUS_LANE_Z: f64 = 34.0;
const STATUS_LANE_PITCH_X: f64 = 186.0;
const STATUS_CENTER_X: f64 = 4.0;
const STATUS_CENTER_Y: f64 = -318.0;

const CHILLED_POCKET_X: f64 = 438.0;
const CHILLED_POCKET_Y: f64 = 208.0;
const CHILLED_POCKET_Z: f64 = 96.0;
const CHILLED_CENTER_X: f64 = -462.0;
const CHILLED_CENTER_Y: f64 = -270.0;
const CHILLED_ADDITIVE_SLOTS: usize = 8;
const CHILLED_SLOT_D: f64 = 28.0;
const CHILLED_SLOT_PITCH_X: f64 = 48.0;

const STRAIN_RELIEF_COMBS: usize = 5;
const STRAIN_RELIEF_SLOTS_PER_COMB: usize = 8;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.9;
const TUBE_SLOT_W: f64 = TUBE_OD + TUBE_CLEARANCE;
const ROUTE_CHANNEL_Z: f64 = 14.0;

const FRONT_ROBOT_APPROACH: f64 = 460.0;
const REAR_SERVICE_CLEARANCE: f64 = 310.0;
const SIDE_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_SERVICE_CLEARANCE: f64 = 380.0;
const BAG_CHANGE_CLEARANCE: f64 = 300.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let basal_dock = basal_media_bag_dock();
    export(OUTPUTS[1], &basal_dock);

    let additive_bay = additive_sterile_connector_bay();
    export(OUTPUTS[2], &additive_bay);

    let load_cells = gravimetric_load_cell_pad_matrix();
    export(OUTPUTS[3], &load_cells);

    let mixer = gentle_rocker_mixer_envelope();
    export(OUTPUTS[4], &mixer);

    let filter_vent = sterile_filter_vent_manifold();
    export(OUTPUTS[5], &filter_vent);

    let qc = qc_sample_loop_handoff();
    export(OUTPUTS[6], &qc);

    let labels = barcode_coa_scan_lands();
    export(OUTPUTS[7], &labels);

    let status = released_hold_reject_status_lanes();
    export(OUTPUTS[8], &status);

    let chilled = chilled_additive_pocket();
    export(OUTPUTS[9], &chilled);

    let tubing = tubing_strain_relief_and_routes();
    export(OUTPUTS[10], &tubing);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + basal_dock
        + additive_bay
        + load_cells
        + mixer
        + filter_vent
        + qc
        + labels
        + status
        + chilled
        + tubing
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media formulation and sterile additive mixing station:");
    println!(
        "  Deck and containment:        {DECK_X:.0}mm x {DECK_Y:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm recessed leak tray, {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Basal media dock:            {BASAL_MEDIA_BAGS} closed basal-media bag positions with load-cell pads and connector strain relief"
    );
    println!(
        "  Additive handling:           {ADDITIVE_CONNECTOR_PORTS} sterile additive connectors, {ADDITIVE_VIAL_COUNT} vial/bag positions, {CHILLED_ADDITIVE_SLOTS} chilled additive pockets"
    );
    println!(
        "  Mixing envelope:             nominal {MIXER_NOMINAL_VOLUME_L:.0}L single-use bag interface, {MIXER_X:.0}mm x {MIXER_Y:.0}mm mixer cradle, {ROCKER_SWING_Z:.0}mm rocker swing gauge"
    );
    println!(
        "  QC and release:              {QC_CHANNELS} sample-loop lanes for pH/osmolality/conductivity handoff, {FILTER_CAPSULES} filter placeholders, {VENT_FILTERS} vent filters, {STATUS_LANES} released/hold/reject lanes"
    );
    println!(
        "  Traceability and service:    {LOAD_CELL_PADS} load-cell pads, barcode/COA lands, {STRAIN_RELIEF_COMBS} tube combs, front robot approach {FRONT_ROBOT_APPROACH:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(ADDITIVE_VIAL_ROWS * ADDITIVE_VIAL_COLS, ADDITIVE_VIAL_COUNT);
    assert!(BULKHEAD_CONNECTOR_PORTS >= ADDITIVE_CONNECTOR_PORTS + QC_HANDOFF_PORTS);
    assert!(LOAD_CELL_PADS >= BASAL_MEDIA_BAGS * 4 + 2);
    assert!(STATUS_LANES == 3);
    assert!(MIXER_BAG_Z + 44.0 < ROCKER_SWING_Z);
    assert!(component_inside_deck(
        BASAL_DOCK_X,
        BASAL_DOCK_Y,
        BASAL_BAG_DOCK_X,
        BASAL_BAG_DOCK_Y
    ));
    assert!(component_inside_deck(
        MIXER_CENTER_X,
        MIXER_CENTER_Y,
        ROCKER_SWING_X,
        ROCKER_SWING_Y
    ));
    assert!(component_inside_deck(
        FILTER_CENTER_X,
        FILTER_CENTER_Y,
        FILTER_MANIFOLD_X,
        FILTER_MANIFOLD_Y
    ));
    assert!(component_inside_deck(
        QC_CENTER_X,
        QC_CENTER_Y,
        QC_BLOCK_X,
        QC_BLOCK_Y
    ));
    assert!(component_inside_deck(
        CHILLED_CENTER_X,
        CHILLED_CENTER_Y,
        CHILLED_POCKET_X,
        CHILLED_POCKET_Y
    ));
    assert!(
        additive_connector_span() + ADDITIVE_COLLAR_D < ADDITIVE_BAY_X,
        "additive connector row exceeds bay width"
    );
    assert!(
        bulkhead_connector_span() + ADDITIVE_COLLAR_D < BULKHEAD_X,
        "bulkhead connector row exceeds rear panel width"
    );
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("closed_media_formulation_base_deck", DECK_X, DECK_Y, DECK_Z)
        .translate(0.0, 0.0, DECK_Z / 2.0);

    let sump = centered_cube(
        "closed_media_formulation_recessed_spill_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -10.0, DECK_Z - SUMP_Z / 2.0 + 0.5);

    let front_drain = centered_cylinder(
        "closed_media_formulation_leak_tray_drain_cut",
        DRAIN_PORT_D / 2.0,
        LEAK_RIM_W + 32.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 14.0, DECK_Z - 5.0);

    deck - sump - front_drain + leak_rims() + deck_datums() + floor_zone_edges()
}

fn leak_rims() -> Part {
    let rear = centered_cube(
        "closed_media_formulation_rear_leak_rim",
        DECK_X,
        LEAK_RIM_W,
        LEAK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - LEAK_RIM_W / 2.0,
        DECK_Z + LEAK_RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_media_formulation_front_low_drip_lip",
        DECK_X,
        LEAK_RIM_W,
        LEAK_RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + LEAK_RIM_W / 2.0,
        DECK_Z + LEAK_RIM_Z * 0.31,
    );
    let left = centered_cube(
        "closed_media_formulation_left_leak_rim",
        LEAK_RIM_W,
        DECK_Y,
        LEAK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEAK_RIM_W / 2.0,
        0.0,
        DECK_Z + LEAK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_formulation_right_leak_rim",
        LEAK_RIM_W,
        DECK_Y,
        LEAK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - LEAK_RIM_W / 2.0,
        0.0,
        DECK_Z + LEAK_RIM_Z / 2.0,
    );

    rear + front + left + right
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_media_formulation_deck_datums");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_media_formulation_m8_datum_boss_{i}"),
            22.0,
            10.0,
            48,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        let hole = centered_cylinder(
            format!("closed_media_formulation_m8_clearance_{i}"),
            5.0,
            12.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 6.0);
        datums = datums + (boss - hole);
    }
    datums
}

fn floor_zone_edges() -> Part {
    let basal = zone_frame(
        "closed_media_formulation_basal_zone_outline",
        BASAL_DOCK_X,
        BASAL_DOCK_Y,
        BASAL_BAG_DOCK_X + 30.0,
        BASAL_BAG_DOCK_Y + 34.0,
    );
    let mixer = zone_frame(
        "closed_media_formulation_mixer_zone_outline",
        MIXER_CENTER_X,
        MIXER_CENTER_Y,
        ROCKER_SWING_X + 24.0,
        ROCKER_SWING_Y + 24.0,
    );
    let qc = zone_frame(
        "closed_media_formulation_qc_release_zone_outline",
        QC_CENTER_X,
        QC_CENTER_Y,
        QC_BLOCK_X + 42.0,
        QC_BLOCK_Y + 42.0,
    );
    basal + mixer + qc
}

fn zone_frame(name: &str, cx: f64, cy: f64, sx: f64, sy: f64) -> Part {
    let rail_z = 5.0;
    let rail_w = 6.0;
    let front = centered_cube(format!("{name}_front"), sx, rail_w, rail_z).translate(
        cx,
        cy - sy / 2.0,
        DECK_Z + rail_z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear"), sx, rail_w, rail_z).translate(
        cx,
        cy + sy / 2.0,
        DECK_Z + rail_z / 2.0,
    );
    let left = centered_cube(format!("{name}_left"), rail_w, sy, rail_z).translate(
        cx - sx / 2.0,
        cy,
        DECK_Z + rail_z / 2.0,
    );
    let right = centered_cube(format!("{name}_right"), rail_w, sy, rail_z).translate(
        cx + sx / 2.0,
        cy,
        DECK_Z + rail_z / 2.0,
    );
    front + rear + left + right
}

fn basal_media_bag_dock() -> Part {
    let dock = centered_cube(
        "closed_media_formulation_basal_bag_dock_body",
        BASAL_BAG_DOCK_X,
        BASAL_BAG_DOCK_Y,
        BASAL_BAG_DOCK_Z,
    )
    .translate(BASAL_DOCK_X, BASAL_DOCK_Y, DECK_Z + BASAL_BAG_DOCK_Z / 2.0);

    let mut recesses = Part::empty("closed_media_formulation_basal_bag_recesses");
    let mut bags = Part::empty("closed_media_formulation_basal_bag_envelopes");
    let mut clamps = Part::empty("closed_media_formulation_basal_bag_clamps");
    for i in 0..BASAL_MEDIA_BAGS {
        let x = BASAL_DOCK_X + centered_index(i, BASAL_MEDIA_BAGS, BASAL_BAG_PITCH_X);
        let recess = centered_cube(
            format!("closed_media_formulation_basal_bag_recess_{i}"),
            BASAL_BAG_X + 20.0,
            BASAL_BAG_Y + 18.0,
            22.0,
        )
        .translate(x, BASAL_DOCK_Y, DECK_Z + BASAL_BAG_DOCK_Z - 9.0);
        recesses = recesses + recess;

        let bag = centered_cube(
            format!("closed_media_formulation_basal_media_bag_envelope_{i}"),
            BASAL_BAG_X,
            BASAL_BAG_Y,
            BASAL_BAG_Z,
        )
        .translate(
            x,
            BASAL_DOCK_Y,
            DECK_Z + BASAL_BAG_DOCK_Z + BASAL_BAG_Z / 2.0 - 8.0,
        );
        bags = bags + bag;

        let inlet_clamp = centered_cube(
            format!("closed_media_formulation_basal_bag_inlet_clamp_{i}"),
            42.0,
            22.0,
            34.0,
        )
        .translate(
            x - BASAL_BAG_X / 2.0 + 24.0,
            BASAL_DOCK_Y + BASAL_BAG_Y / 2.0 + 18.0,
            DECK_Z + BASAL_BAG_DOCK_Z + 17.0,
        );
        let outlet_clamp = centered_cube(
            format!("closed_media_formulation_basal_bag_outlet_clamp_{i}"),
            42.0,
            22.0,
            34.0,
        )
        .translate(
            x + BASAL_BAG_X / 2.0 - 24.0,
            BASAL_DOCK_Y - BASAL_BAG_Y / 2.0 - 18.0,
            DECK_Z + BASAL_BAG_DOCK_Z + 17.0,
        );
        clamps = clamps + inlet_clamp + outlet_clamp;
    }

    let front_scale_guard = centered_cube(
        "closed_media_formulation_basal_front_scale_guard",
        BASAL_BAG_DOCK_X + 38.0,
        18.0,
        54.0,
    )
    .translate(
        BASAL_DOCK_X,
        BASAL_DOCK_Y - BASAL_BAG_DOCK_Y / 2.0 - 9.0,
        DECK_Z + 27.0,
    );
    let rear_line_guard = centered_cube(
        "closed_media_formulation_basal_rear_line_guard",
        BASAL_BAG_DOCK_X + 38.0,
        18.0,
        54.0,
    )
    .translate(
        BASAL_DOCK_X,
        BASAL_DOCK_Y + BASAL_BAG_DOCK_Y / 2.0 + 9.0,
        DECK_Z + 27.0,
    );

    dock - recesses + bags + clamps + front_scale_guard + rear_line_guard
}

fn additive_sterile_connector_bay() -> Part {
    let tray = centered_cube(
        "closed_media_formulation_additive_connector_tray",
        ADDITIVE_BAY_X,
        ADDITIVE_BAY_Y,
        ADDITIVE_BAY_Z,
    )
    .translate(
        ADDITIVE_BAY_CENTER_X,
        ADDITIVE_BAY_CENTER_Y,
        DECK_Z + ADDITIVE_BAY_Z / 2.0,
    );

    let vial_recesses = additive_vial_recesses();
    let connector_row = additive_connector_row();
    let cap_parks = additive_cap_parks();
    let rear_bulkhead = rear_sterile_bulkhead_panel();
    let line_gate = centered_cube(
        "closed_media_formulation_additive_line_gate",
        ADDITIVE_BAY_X - 80.0,
        20.0,
        54.0,
    )
    .translate(
        ADDITIVE_BAY_CENTER_X,
        ADDITIVE_BAY_CENTER_Y - ADDITIVE_BAY_Y / 2.0 - 10.0,
        DECK_Z + 42.0,
    );

    tray - vial_recesses + connector_row + cap_parks + rear_bulkhead + line_gate
}

fn additive_vial_recesses() -> Part {
    let mut wells = Part::empty("closed_media_formulation_additive_vial_well_cuts");
    for row in 0..ADDITIVE_VIAL_ROWS {
        for col in 0..ADDITIVE_VIAL_COLS {
            let x = ADDITIVE_BAY_CENTER_X
                + centered_index(col, ADDITIVE_VIAL_COLS, ADDITIVE_VIAL_PITCH_X);
            let y = ADDITIVE_BAY_CENTER_Y - 24.0
                + centered_index(row, ADDITIVE_VIAL_ROWS, ADDITIVE_VIAL_PITCH_Y);
            let well = centered_cylinder(
                format!("closed_media_formulation_additive_vial_well_cut_{row}_{col}"),
                ADDITIVE_VIAL_WELL_D / 2.0,
                ADDITIVE_BAY_Z + 2.0,
                36,
            )
            .translate(x, y, DECK_Z + ADDITIVE_BAY_Z / 2.0 + 1.0);
            wells = wells + well;
        }
    }
    wells
}

fn additive_connector_row() -> Part {
    let mut row = Part::empty("closed_media_formulation_additive_sterile_connector_row");
    for i in 0..ADDITIVE_CONNECTOR_PORTS {
        let x = ADDITIVE_BAY_CENTER_X
            + centered_index(i, ADDITIVE_CONNECTOR_PORTS, ADDITIVE_CONNECTOR_PITCH_X);
        let y = ADDITIVE_BAY_CENTER_Y + ADDITIVE_BAY_Y / 2.0 - 24.0;
        let collar = centered_cylinder(
            format!("closed_media_formulation_additive_connector_collar_{i}"),
            ADDITIVE_COLLAR_D / 2.0,
            22.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, DECK_Z + ADDITIVE_BAY_Z + 28.0);
        let bore = centered_cylinder(
            format!("closed_media_formulation_additive_connector_bore_gauge_{i}"),
            ADDITIVE_CONNECTOR_D / 2.0,
            28.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y - 2.0, DECK_Z + ADDITIVE_BAY_Z + 28.0);
        let flag = centered_cube(
            format!("closed_media_formulation_additive_connector_id_flag_{i}"),
            26.0,
            4.0,
            18.0,
        )
        .translate(x, y - 18.0, DECK_Z + ADDITIVE_BAY_Z + 58.0);
        row = row + (collar - bore) + flag;
    }
    row
}

fn additive_cap_parks() -> Part {
    let mut parks = Part::empty("closed_media_formulation_additive_cap_parks");
    for i in 0..ADDITIVE_CONNECTOR_PORTS {
        let x = ADDITIVE_BAY_CENTER_X
            + centered_index(i, ADDITIVE_CONNECTOR_PORTS, ADDITIVE_CONNECTOR_PITCH_X);
        let park = centered_cylinder(
            format!("closed_media_formulation_additive_cap_park_{i}"),
            11.0,
            12.0,
            32,
        )
        .translate(
            x,
            ADDITIVE_BAY_CENTER_Y - ADDITIVE_BAY_Y / 2.0 + 24.0,
            DECK_Z + ADDITIVE_BAY_Z + 6.0,
        );
        parks = parks + park;
    }
    parks
}

fn rear_sterile_bulkhead_panel() -> Part {
    let panel = centered_cube(
        "closed_media_formulation_rear_sterile_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, DECK_Z + BULKHEAD_Z / 2.0);

    let mut connector_features = Part::empty("closed_media_formulation_rear_bulkhead_features");
    for i in 0..BULKHEAD_CONNECTOR_PORTS {
        let x = centered_index(i, BULKHEAD_CONNECTOR_PORTS, BULKHEAD_CONNECTOR_PITCH_X);
        let collar = centered_cylinder(
            format!("closed_media_formulation_rear_bulkhead_connector_collar_{i}"),
            18.0,
            18.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 6.0,
            DECK_Z + 114.0,
        );
        let bore = centered_cylinder(
            format!("closed_media_formulation_rear_bulkhead_connector_bore_{i}"),
            8.5,
            24.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 8.0,
            DECK_Z + 114.0,
        );
        let label = centered_cube(
            format!("closed_media_formulation_rear_bulkhead_label_land_{i}"),
            34.0,
            4.0,
            14.0,
        )
        .translate(x, BULKHEAD_CENTER_Y - BULKHEAD_Y - 8.0, DECK_Z + 152.0);
        connector_features = connector_features + (collar - bore) + label;
    }

    for i in 0..VENT_FILTERS {
        let x = 430.0 + centered_index(i, VENT_FILTERS, 72.0);
        let vent = centered_cylinder(
            format!("closed_media_formulation_rear_vent_filter_placeholder_{i}"),
            16.0,
            46.0,
            40,
        )
        .translate(x, BULKHEAD_CENTER_Y - 28.0, DECK_Z + BULKHEAD_Z + 23.0);
        let collar = centered_cylinder(
            format!("closed_media_formulation_rear_vent_filter_collar_{i}"),
            24.0,
            8.0,
            40,
        )
        .translate(x, BULKHEAD_CENTER_Y - 28.0, DECK_Z + BULKHEAD_Z + 4.0);
        connector_features = connector_features + vent + collar;
    }

    let cable_gland_bank = centered_cube(
        "closed_media_formulation_rear_sensor_cable_gland_bank",
        240.0,
        20.0,
        50.0,
    )
    .translate(-500.0, BULKHEAD_CENTER_Y - 30.0, DECK_Z + 184.0);

    panel + connector_features + cable_gland_bank
}

fn gravimetric_load_cell_pad_matrix() -> Part {
    let mut pads = Part::empty("closed_media_formulation_load_cell_pad_matrix");
    for (i, (x, y, label)) in load_cell_positions().iter().enumerate() {
        let pad = centered_cube(
            format!("closed_media_formulation_load_cell_pad_{i}_{label}"),
            LOAD_CELL_PAD_X,
            LOAD_CELL_PAD_Y,
            LOAD_CELL_PAD_Z,
        )
        .translate(*x, *y, DECK_Z + LOAD_CELL_PAD_Z / 2.0 + 2.0);
        let boss = centered_cylinder(
            format!("closed_media_formulation_load_cell_cable_relief_{i}_{label}"),
            7.0,
            10.0,
            28,
        )
        .translate(*x + 28.0, *y - 18.0, DECK_Z + LOAD_CELL_PAD_Z + 7.0);
        pads = pads + pad + boss;
    }
    pads
}

fn gentle_rocker_mixer_envelope() -> Part {
    let cradle = centered_cube(
        "closed_media_formulation_mixer_cradle_base",
        MIXER_X,
        MIXER_Y,
        MIXER_BASE_Z,
    )
    .translate(MIXER_CENTER_X, MIXER_CENTER_Y, DECK_Z + MIXER_BASE_Z / 2.0);

    let bag_recess = centered_cube(
        "closed_media_formulation_mixer_bag_recess_cut",
        MIXER_X - 70.0,
        MIXER_Y - 68.0,
        30.0,
    )
    .translate(MIXER_CENTER_X, MIXER_CENTER_Y, DECK_Z + MIXER_BASE_Z - 12.0);

    let bag = centered_cube(
        "closed_media_formulation_single_use_mixer_bag_envelope",
        MIXER_X - 94.0,
        MIXER_Y - 92.0,
        MIXER_BAG_Z,
    )
    .translate(
        MIXER_CENTER_X,
        MIXER_CENTER_Y,
        DECK_Z + MIXER_BASE_Z + MIXER_BAG_Z / 2.0 - 8.0,
    );

    let rocker_axis_left = centered_cylinder(
        "closed_media_formulation_rocker_axis_left",
        ROCKER_AXIS_D / 2.0,
        58.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        MIXER_CENTER_X - MIXER_X / 2.0 - 22.0,
        MIXER_CENTER_Y,
        DECK_Z + MIXER_BASE_Z + 34.0,
    );
    let rocker_axis_right = centered_cylinder(
        "closed_media_formulation_rocker_axis_right",
        ROCKER_AXIS_D / 2.0,
        58.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        MIXER_CENTER_X + MIXER_X / 2.0 + 22.0,
        MIXER_CENTER_Y,
        DECK_Z + MIXER_BASE_Z + 34.0,
    );

    let swing = rocker_swing_gauge();
    let ports = mixer_port_lands();
    let slope_rails = mixer_slope_and_drain_rails();

    cradle - bag_recess + bag + rocker_axis_left + rocker_axis_right + swing + ports + slope_rails
}

fn rocker_swing_gauge() -> Part {
    let front = centered_cube(
        "closed_media_formulation_rocker_swing_front_gauge",
        ROCKER_SWING_X,
        10.0,
        12.0,
    )
    .translate(
        MIXER_CENTER_X,
        MIXER_CENTER_Y - ROCKER_SWING_Y / 2.0,
        DECK_Z + ROCKER_SWING_Z,
    );
    let rear = centered_cube(
        "closed_media_formulation_rocker_swing_rear_gauge",
        ROCKER_SWING_X,
        10.0,
        12.0,
    )
    .translate(
        MIXER_CENTER_X,
        MIXER_CENTER_Y + ROCKER_SWING_Y / 2.0,
        DECK_Z + ROCKER_SWING_Z,
    );
    let left = centered_cube(
        "closed_media_formulation_rocker_swing_left_gauge",
        10.0,
        ROCKER_SWING_Y,
        12.0,
    )
    .translate(
        MIXER_CENTER_X - ROCKER_SWING_X / 2.0,
        MIXER_CENTER_Y,
        DECK_Z + ROCKER_SWING_Z,
    );
    let right = centered_cube(
        "closed_media_formulation_rocker_swing_right_gauge",
        10.0,
        ROCKER_SWING_Y,
        12.0,
    )
    .translate(
        MIXER_CENTER_X + ROCKER_SWING_X / 2.0,
        MIXER_CENTER_Y,
        DECK_Z + ROCKER_SWING_Z,
    );
    front + rear + left + right
}

fn mixer_port_lands() -> Part {
    let mut ports = Part::empty("closed_media_formulation_mixer_port_lands");
    for i in 0..6 {
        let x = MIXER_CENTER_X + centered_index(i, 6, 56.0);
        let port = centered_cylinder(
            format!("closed_media_formulation_mixer_bag_port_land_{i}"),
            13.0,
            16.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            MIXER_CENTER_Y + MIXER_Y / 2.0 + 16.0,
            DECK_Z + MIXER_BASE_Z + 72.0,
        );
        ports = ports + port;
    }
    ports
}

fn mixer_slope_and_drain_rails() -> Part {
    let left = centered_cube(
        "closed_media_formulation_mixer_sloped_bottom_left_rail",
        MIXER_X - 88.0,
        18.0,
        22.0,
    )
    .translate(
        MIXER_CENTER_X,
        MIXER_CENTER_Y - 92.0,
        DECK_Z + MIXER_BASE_Z + 10.0,
    );
    let right = centered_cube(
        "closed_media_formulation_mixer_sloped_bottom_right_rail",
        MIXER_X - 88.0,
        18.0,
        34.0,
    )
    .translate(
        MIXER_CENTER_X,
        MIXER_CENTER_Y + 96.0,
        DECK_Z + MIXER_BASE_Z + 16.0,
    );
    let drain_gate = centered_cube(
        "closed_media_formulation_mixer_low_point_drain_gate",
        72.0,
        16.0,
        38.0,
    )
    .translate(
        MIXER_CENTER_X + MIXER_X / 2.0 - 52.0,
        MIXER_CENTER_Y - MIXER_Y / 2.0 - 8.0,
        DECK_Z + MIXER_BASE_Z + 22.0,
    );
    left + right + drain_gate
}

fn sterile_filter_vent_manifold() -> Part {
    let block = centered_cube(
        "closed_media_formulation_filter_vent_manifold_block",
        FILTER_MANIFOLD_X,
        FILTER_MANIFOLD_Y,
        FILTER_MANIFOLD_Z,
    )
    .translate(
        FILTER_CENTER_X,
        FILTER_CENTER_Y,
        DECK_Z + FILTER_MANIFOLD_Z / 2.0,
    );

    let filter_capsules = filter_capsule_placeholders();
    let pressure_taps = pressure_tap_pockets();
    let vent_bank = vent_filter_bank();
    let drip_bridge = centered_cube(
        "closed_media_formulation_filter_drip_bridge",
        FILTER_MANIFOLD_X + 50.0,
        42.0,
        18.0,
    )
    .translate(
        FILTER_CENTER_X,
        FILTER_CENTER_Y - FILTER_MANIFOLD_Y / 2.0 - 26.0,
        DECK_Z + 9.0,
    );

    block + filter_capsules + pressure_taps + vent_bank + drip_bridge
}

fn filter_capsule_placeholders() -> Part {
    let mut bank = Part::empty("closed_media_formulation_filter_capsule_bank");
    for i in 0..FILTER_CAPSULES {
        let y = FILTER_CENTER_Y + centered_index(i, FILTER_CAPSULES, 42.0);
        let capsule = centered_cylinder(
            format!("closed_media_formulation_sterile_filter_capsule_{i}"),
            FILTER_CAPSULE_D / 2.0,
            FILTER_CAPSULE_LENGTH,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(FILTER_CENTER_X, y, DECK_Z + FILTER_MANIFOLD_Z + 26.0);
        let saddle = centered_cube(
            format!("closed_media_formulation_filter_capsule_saddle_{i}"),
            FILTER_CAPSULE_LENGTH + 32.0,
            18.0,
            20.0,
        )
        .translate(FILTER_CENTER_X, y, DECK_Z + FILTER_MANIFOLD_Z + 4.0);
        bank = bank + capsule + saddle;
    }
    bank
}

fn pressure_tap_pockets() -> Part {
    let mut taps = Part::empty("closed_media_formulation_pressure_tap_pockets");
    for i in 0..PRESSURE_TAPS {
        let x = FILTER_CENTER_X + centered_index(i, PRESSURE_TAPS, 64.0);
        let tap = centered_cylinder(
            format!("closed_media_formulation_pressure_tap_{i}"),
            10.0,
            20.0,
            32,
        )
        .translate(
            x,
            FILTER_CENTER_Y - FILTER_MANIFOLD_Y / 2.0 + 22.0,
            DECK_Z + FILTER_MANIFOLD_Z + 10.0,
        );
        taps = taps + tap;
    }
    taps
}

fn vent_filter_bank() -> Part {
    let mut vents = Part::empty("closed_media_formulation_local_vent_filter_bank");
    for i in 0..VENT_FILTERS {
        let x = FILTER_CENTER_X + centered_index(i, VENT_FILTERS, 56.0);
        let vent = centered_cylinder(
            format!("closed_media_formulation_local_vent_filter_{i}"),
            13.0,
            52.0,
            32,
        )
        .translate(
            x,
            FILTER_CENTER_Y + FILTER_MANIFOLD_Y / 2.0 - 20.0,
            DECK_Z + FILTER_MANIFOLD_Z + 26.0,
        );
        vents = vents + vent;
    }
    vents
}

fn qc_sample_loop_handoff() -> Part {
    let block = centered_cube(
        "closed_media_formulation_qc_sample_loop_block",
        QC_BLOCK_X,
        QC_BLOCK_Y,
        QC_BLOCK_Z,
    )
    .translate(QC_CENTER_X, QC_CENTER_Y, DECK_Z + QC_BLOCK_Z / 2.0);

    let mut loops = Part::empty("closed_media_formulation_qc_sample_loop_lanes");
    for i in 0..QC_CHANNELS {
        let x = QC_CENTER_X + centered_index(i, QC_CHANNELS, QC_CHANNEL_PITCH_X);
        let lane = centered_cube(
            format!("closed_media_formulation_qc_lane_{i}"),
            54.0,
            QC_BLOCK_Y - 42.0,
            12.0,
        )
        .translate(x, QC_CENTER_Y, DECK_Z + QC_BLOCK_Z + 6.0);
        let loop_bend = centered_cylinder(
            format!("closed_media_formulation_qc_sample_loop_bend_{i}"),
            24.0,
            12.0,
            42,
        )
        .translate(x, QC_CENTER_Y + 52.0, DECK_Z + QC_BLOCK_Z + 6.0);
        let sensor_boss = centered_cube(
            format!("closed_media_formulation_qc_sensor_boss_{i}"),
            62.0,
            32.0,
            28.0,
        )
        .translate(x, QC_CENTER_Y - 48.0, DECK_Z + QC_BLOCK_Z + 14.0);
        loops = loops + lane + loop_bend + sensor_boss;
    }

    let handoff = qc_handoff_ports();
    let sample_bag_lands = qc_sample_bag_lands();
    block + loops + handoff + sample_bag_lands
}

fn qc_handoff_ports() -> Part {
    let mut ports = Part::empty("closed_media_formulation_qc_handoff_ports");
    for i in 0..QC_HANDOFF_PORTS {
        let x = QC_CENTER_X + centered_index(i, QC_HANDOFF_PORTS, 46.0);
        let port = centered_cylinder(
            format!("closed_media_formulation_qc_handoff_connector_{i}"),
            12.0,
            18.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            QC_CENTER_Y - QC_BLOCK_Y / 2.0 - 18.0,
            DECK_Z + QC_BLOCK_Z + 32.0,
        );
        ports = ports + port;
    }
    ports
}

fn qc_sample_bag_lands() -> Part {
    let mut lands = Part::empty("closed_media_formulation_qc_sample_bag_lands");
    for i in 0..QC_SAMPLE_BAGS {
        let x = QC_CENTER_X + centered_index(i, QC_SAMPLE_BAGS, 92.0);
        let land = centered_cube(
            format!("closed_media_formulation_qc_sample_bag_land_{i}"),
            78.0,
            44.0,
            10.0,
        )
        .translate(
            x,
            QC_CENTER_Y + QC_BLOCK_Y / 2.0 - 26.0,
            DECK_Z + QC_BLOCK_Z + 5.0,
        );
        lands = lands + land;
    }
    lands
}

fn barcode_coa_scan_lands() -> Part {
    let mut lands = Part::empty("closed_media_formulation_barcode_coa_scan_lands");
    for (i, (x, y, sx, sy)) in label_land_positions().iter().enumerate() {
        let land = centered_cube(
            format!("closed_media_formulation_barcode_coa_land_{i}"),
            *sx,
            *sy,
            4.0,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        let fiducial = centered_cylinder(
            format!("closed_media_formulation_scan_fiducial_{i}"),
            4.0,
            4.0,
            20,
        )
        .translate(*x + *sx / 2.0 - 9.0, *y + *sy / 2.0 - 9.0, DECK_Z + 8.0);
        lands = lands + land + fiducial;
    }

    let coa_document_shelf = centered_cube(
        "closed_media_formulation_coa_document_scan_shelf",
        280.0,
        118.0,
        8.0,
    )
    .translate(-42.0, -420.0, DECK_Z + 8.0);
    let coa_backstop = centered_cube(
        "closed_media_formulation_coa_document_backstop",
        280.0,
        12.0,
        38.0,
    )
    .translate(-42.0, -355.0, DECK_Z + 27.0);

    lands + coa_document_shelf + coa_backstop
}

fn released_hold_reject_status_lanes() -> Part {
    let mut lanes = Part::empty("closed_media_formulation_released_hold_reject_lanes");
    for i in 0..STATUS_LANES {
        let x = STATUS_CENTER_X + centered_index(i, STATUS_LANES, STATUS_LANE_PITCH_X);
        let lane = centered_cube(
            format!("closed_media_formulation_status_lane_body_{i}"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(x, STATUS_CENTER_Y, DECK_Z + STATUS_LANE_Z / 2.0);
        let recess = centered_cube(
            format!("closed_media_formulation_status_lane_recess_{i}"),
            STATUS_LANE_X - 30.0,
            STATUS_LANE_Y - 34.0,
            12.0,
        )
        .translate(x, STATUS_CENTER_Y, DECK_Z + STATUS_LANE_Z - 4.0);
        let flag = centered_cube(
            format!("closed_media_formulation_status_lane_flag_{i}"),
            54.0,
            8.0,
            42.0,
        )
        .translate(
            x,
            STATUS_CENTER_Y + STATUS_LANE_Y / 2.0 + 8.0,
            DECK_Z + 58.0,
        );
        lanes = lanes + (lane - recess) + flag;
    }

    let hard_divider_left = centered_cube(
        "closed_media_formulation_status_lane_hard_divider_left",
        10.0,
        STATUS_LANE_Y + 38.0,
        48.0,
    )
    .translate(
        STATUS_CENTER_X - STATUS_LANE_PITCH_X / 2.0,
        STATUS_CENTER_Y,
        DECK_Z + 24.0,
    );
    let hard_divider_right = centered_cube(
        "closed_media_formulation_status_lane_hard_divider_right",
        10.0,
        STATUS_LANE_Y + 38.0,
        48.0,
    )
    .translate(
        STATUS_CENTER_X + STATUS_LANE_PITCH_X / 2.0,
        STATUS_CENTER_Y,
        DECK_Z + 24.0,
    );

    lanes + hard_divider_left + hard_divider_right
}

fn chilled_additive_pocket() -> Part {
    let cold_block = centered_cube(
        "closed_media_formulation_chilled_additive_pocket_body",
        CHILLED_POCKET_X,
        CHILLED_POCKET_Y,
        CHILLED_POCKET_Z,
    )
    .translate(
        CHILLED_CENTER_X,
        CHILLED_CENTER_Y,
        DECK_Z + CHILLED_POCKET_Z / 2.0,
    );

    let mut slots = Part::empty("closed_media_formulation_chilled_additive_slot_cuts");
    for i in 0..CHILLED_ADDITIVE_SLOTS {
        let x = CHILLED_CENTER_X + centered_index(i, CHILLED_ADDITIVE_SLOTS, CHILLED_SLOT_PITCH_X);
        let slot = centered_cylinder(
            format!("closed_media_formulation_chilled_additive_slot_cut_{i}"),
            CHILLED_SLOT_D / 2.0,
            CHILLED_POCKET_Z + 4.0,
            36,
        )
        .translate(
            x,
            CHILLED_CENTER_Y + 18.0,
            DECK_Z + CHILLED_POCKET_Z / 2.0 + 2.0,
        );
        slots = slots + slot;
    }

    let insulation_lid = centered_cube(
        "closed_media_formulation_chilled_additive_insulation_lid_gauge",
        CHILLED_POCKET_X + 38.0,
        CHILLED_POCKET_Y + 36.0,
        22.0,
    )
    .translate(
        CHILLED_CENTER_X,
        CHILLED_CENTER_Y,
        DECK_Z + CHILLED_POCKET_Z + 30.0,
    );
    let temp_sensor = centered_cylinder(
        "closed_media_formulation_chilled_additive_temperature_probe_pocket",
        7.0,
        54.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        CHILLED_CENTER_X + CHILLED_POCKET_X / 2.0 - 34.0,
        CHILLED_CENTER_Y - CHILLED_POCKET_Y / 2.0 - 10.0,
        DECK_Z + CHILLED_POCKET_Z / 2.0,
    );
    let condensate_channel = centered_cube(
        "closed_media_formulation_chilled_additive_condensate_channel",
        CHILLED_POCKET_X - 46.0,
        12.0,
        10.0,
    )
    .translate(
        CHILLED_CENTER_X,
        CHILLED_CENTER_Y - CHILLED_POCKET_Y / 2.0 + 18.0,
        DECK_Z + 8.0,
    );

    cold_block - slots + insulation_lid + temp_sensor + condensate_channel
}

fn tubing_strain_relief_and_routes() -> Part {
    let mut tubing = Part::empty("closed_media_formulation_tubing_strain_relief_routes");
    for i in 0..STRAIN_RELIEF_COMBS {
        let (x, y, rot) = comb_positions()[i];
        let comb = strain_relief_comb(i, x, y, rot);
        tubing = tubing + comb;
    }

    for (i, (x, y, sx, sy, z)) in route_channel_specs().iter().enumerate() {
        let route = centered_cube(
            format!("closed_media_formulation_tube_route_channel_{i}"),
            *sx,
            *sy,
            ROUTE_CHANNEL_Z,
        )
        .translate(*x, *y, DECK_Z + *z);
        tubing = tubing + route;
    }

    tubing
}

fn strain_relief_comb(index: usize, x: f64, y: f64, rotate_z: bool) -> Part {
    let body_x = if rotate_z { 42.0 } else { 118.0 };
    let body_y = if rotate_z { 118.0 } else { 42.0 };
    let body = centered_cube(
        format!("closed_media_formulation_strain_relief_comb_body_{index}"),
        body_x,
        body_y,
        28.0,
    )
    .translate(x, y, DECK_Z + 14.0);

    let mut slots = Part::empty(format!(
        "closed_media_formulation_strain_relief_comb_slots_{index}"
    ));
    for i in 0..STRAIN_RELIEF_SLOTS_PER_COMB {
        let offset = centered_index(i, STRAIN_RELIEF_SLOTS_PER_COMB, 12.0);
        let slot = if rotate_z {
            centered_cube(
                format!("closed_media_formulation_strain_relief_slot_{index}_{i}"),
                TUBE_SLOT_W,
                34.0,
                30.0,
            )
            .translate(x + offset, y, DECK_Z + 15.0)
        } else {
            centered_cube(
                format!("closed_media_formulation_strain_relief_slot_{index}_{i}"),
                34.0,
                TUBE_SLOT_W,
                30.0,
            )
            .translate(x, y + offset, DECK_Z + 15.0)
        };
        slots = slots + slot;
    }

    body - slots
}

fn robot_service_keepouts() -> Part {
    let front = keepout_frame(
        "closed_media_formulation_front_robot_approach_keepout",
        DECK_X,
        FRONT_ROBOT_APPROACH,
        24.0,
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + 150.0,
    );
    let rear = keepout_frame(
        "closed_media_formulation_rear_service_keepout",
        DECK_X,
        REAR_SERVICE_CLEARANCE,
        24.0,
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        DECK_Z + 150.0,
    );
    let left = keepout_frame(
        "closed_media_formulation_left_bag_change_keepout",
        SIDE_SERVICE_CLEARANCE,
        DECK_Y,
        24.0,
        -DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE / 2.0,
        0.0,
        DECK_Z + 150.0,
    );
    let top = keepout_frame(
        "closed_media_formulation_top_rocker_clearance_keepout",
        ROCKER_SWING_X + 80.0,
        ROCKER_SWING_Y + 80.0,
        18.0,
        MIXER_CENTER_X,
        MIXER_CENTER_Y,
        DECK_Z + TOP_SERVICE_CLEARANCE,
    );
    let bag_change = keepout_frame(
        "closed_media_formulation_basal_bag_change_pull_keepout",
        BAG_CHANGE_CLEARANCE,
        BASAL_BAG_DOCK_Y + 80.0,
        18.0,
        BASAL_DOCK_X - BASAL_BAG_DOCK_X / 2.0 - BAG_CHANGE_CLEARANCE / 2.0,
        BASAL_DOCK_Y,
        DECK_Z + 112.0,
    );
    front + rear + left + top + bag_change
}

fn keepout_frame(name: &str, sx: f64, sy: f64, rail: f64, cx: f64, cy: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front"), sx, rail, rail).translate(cx, cy - sy / 2.0, z);
    let rear =
        centered_cube(format!("{name}_rear"), sx, rail, rail).translate(cx, cy + sy / 2.0, z);
    let left =
        centered_cube(format!("{name}_left"), rail, sy, rail).translate(cx - sx / 2.0, cy, z);
    let right =
        centered_cube(format!("{name}_right"), rail, sy, rail).translate(cx + sx / 2.0, cy, z);
    front + rear + left + right
}

fn component_inside_deck(cx: f64, cy: f64, sx: f64, sy: f64) -> bool {
    cx - sx / 2.0 > -DECK_X / 2.0 + LEAK_RIM_W
        && cx + sx / 2.0 < DECK_X / 2.0 - LEAK_RIM_W
        && cy - sy / 2.0 > -DECK_Y / 2.0 + LEAK_RIM_W
        && cy + sy / 2.0 < DECK_Y / 2.0 - LEAK_RIM_W
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn additive_connector_span() -> f64 {
    (ADDITIVE_CONNECTOR_PORTS as f64 - 1.0) * ADDITIVE_CONNECTOR_PITCH_X
}

fn bulkhead_connector_span() -> f64 {
    (BULKHEAD_CONNECTOR_PORTS as f64 - 1.0) * BULKHEAD_CONNECTOR_PITCH_X
}

fn datum_positions() -> [(f64, f64); DATUM_BOSS_COUNT] {
    [
        (-640.0, -390.0),
        (-220.0, -390.0),
        (220.0, -390.0),
        (640.0, -390.0),
        (-640.0, 362.0),
        (-220.0, 362.0),
        (220.0, 362.0),
        (640.0, 362.0),
    ]
}

fn load_cell_positions() -> [(f64, f64, &'static str); LOAD_CELL_PADS] {
    [
        (BASAL_DOCK_X - 104.0, BASAL_DOCK_Y - 96.0, "basal_a_front"),
        (BASAL_DOCK_X - 104.0, BASAL_DOCK_Y + 96.0, "basal_a_rear"),
        (BASAL_DOCK_X + 104.0, BASAL_DOCK_Y - 96.0, "basal_b_front"),
        (BASAL_DOCK_X + 104.0, BASAL_DOCK_Y + 96.0, "basal_b_rear"),
        (
            MIXER_CENTER_X - 150.0,
            MIXER_CENTER_Y - 116.0,
            "mixer_front_left",
        ),
        (
            MIXER_CENTER_X + 150.0,
            MIXER_CENTER_Y - 116.0,
            "mixer_front_right",
        ),
        (
            MIXER_CENTER_X - 150.0,
            MIXER_CENTER_Y + 116.0,
            "mixer_rear_left",
        ),
        (
            MIXER_CENTER_X + 150.0,
            MIXER_CENTER_Y + 116.0,
            "mixer_rear_right",
        ),
        (
            CHILLED_CENTER_X - 144.0,
            CHILLED_CENTER_Y - 82.0,
            "chilled_left",
        ),
        (
            CHILLED_CENTER_X + 144.0,
            CHILLED_CENTER_Y - 82.0,
            "chilled_right",
        ),
    ]
}

fn label_land_positions() -> [(f64, f64, f64, f64); 18] {
    [
        (BASAL_DOCK_X - 104.0, BASAL_DOCK_Y - 176.0, 86.0, 26.0),
        (BASAL_DOCK_X + 104.0, BASAL_DOCK_Y - 176.0, 86.0, 26.0),
        (BASAL_DOCK_X - 104.0, BASAL_DOCK_Y + 174.0, 86.0, 26.0),
        (BASAL_DOCK_X + 104.0, BASAL_DOCK_Y + 174.0, 86.0, 26.0),
        (
            ADDITIVE_BAY_CENTER_X - 220.0,
            ADDITIVE_BAY_CENTER_Y - 104.0,
            78.0,
            24.0,
        ),
        (
            ADDITIVE_BAY_CENTER_X - 110.0,
            ADDITIVE_BAY_CENTER_Y - 104.0,
            78.0,
            24.0,
        ),
        (
            ADDITIVE_BAY_CENTER_X,
            ADDITIVE_BAY_CENTER_Y - 104.0,
            78.0,
            24.0,
        ),
        (
            ADDITIVE_BAY_CENTER_X + 110.0,
            ADDITIVE_BAY_CENTER_Y - 104.0,
            78.0,
            24.0,
        ),
        (
            ADDITIVE_BAY_CENTER_X + 220.0,
            ADDITIVE_BAY_CENTER_Y - 104.0,
            78.0,
            24.0,
        ),
        (MIXER_CENTER_X - 148.0, MIXER_CENTER_Y - 204.0, 96.0, 28.0),
        (MIXER_CENTER_X + 148.0, MIXER_CENTER_Y - 204.0, 96.0, 28.0),
        (FILTER_CENTER_X - 104.0, FILTER_CENTER_Y - 118.0, 86.0, 24.0),
        (FILTER_CENTER_X + 104.0, FILTER_CENTER_Y - 118.0, 86.0, 24.0),
        (QC_CENTER_X - 112.0, QC_CENTER_Y + 128.0, 88.0, 26.0),
        (QC_CENTER_X, QC_CENTER_Y + 128.0, 88.0, 26.0),
        (QC_CENTER_X + 112.0, QC_CENTER_Y + 128.0, 88.0, 26.0),
        (
            CHILLED_CENTER_X - 120.0,
            CHILLED_CENTER_Y + 132.0,
            92.0,
            26.0,
        ),
        (
            CHILLED_CENTER_X + 120.0,
            CHILLED_CENTER_Y + 132.0,
            92.0,
            26.0,
        ),
    ]
}

fn comb_positions() -> [(f64, f64, bool); STRAIN_RELIEF_COMBS] {
    [
        (BASAL_DOCK_X + 262.0, BASAL_DOCK_Y + 132.0, false),
        (
            ADDITIVE_BAY_CENTER_X + 348.0,
            ADDITIVE_BAY_CENTER_Y - 28.0,
            true,
        ),
        (MIXER_CENTER_X + 292.0, MIXER_CENTER_Y + 112.0, true),
        (FILTER_CENTER_X - 210.0, FILTER_CENTER_Y - 118.0, false),
        (QC_CENTER_X - 220.0, QC_CENTER_Y - 84.0, false),
    ]
}

fn route_channel_specs() -> [(f64, f64, f64, f64, f64); 7] {
    [
        (-230.0, 186.0, 430.0, 12.0, 46.0),
        (92.0, 208.0, 360.0, 12.0, 58.0),
        (320.0, 26.0, 12.0, 300.0, 54.0),
        (456.0, -18.0, 12.0, 260.0, 62.0),
        (228.0, -196.0, 314.0, 12.0, 50.0),
        (-300.0, -108.0, 12.0, 330.0, 42.0),
        (-252.0, -266.0, 404.0, 12.0, 42.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_media_formulation_mixing_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn closed_formulation_interfaces_cover_media_additives_and_qc() {
        assert_eq!(BASAL_MEDIA_BAGS, 2);
        assert_eq!(ADDITIVE_VIAL_COUNT, 12);
        assert_eq!(ADDITIVE_CONNECTOR_PORTS, 12);
        assert!(BULKHEAD_CONNECTOR_PORTS >= ADDITIVE_CONNECTOR_PORTS + QC_HANDOFF_PORTS);
        assert_eq!(QC_CHANNELS, 3);
        assert_eq!(QC_SAMPLE_BAGS, 3);
    }

    #[test]
    fn gravimetric_and_disposition_controls_are_explicit() {
        assert_eq!(LOAD_CELL_PADS, load_cell_positions().len());
        assert!(LOAD_CELL_PADS >= BASAL_MEDIA_BAGS * 4 + 2);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(CHILLED_ADDITIVE_SLOTS, 8);
        assert_eq!(label_land_positions().len(), 18);
    }

    #[test]
    fn mixer_and_filters_fit_with_service_clearance() {
        assert!(component_inside_deck(
            MIXER_CENTER_X,
            MIXER_CENTER_Y,
            ROCKER_SWING_X,
            ROCKER_SWING_Y
        ));
        assert!(MIXER_BAG_Z + 44.0 < ROCKER_SWING_Z);
        assert_eq!(FILTER_CAPSULES, 3);
        assert_eq!(VENT_FILTERS, 4);
        assert!(FRONT_ROBOT_APPROACH >= 420.0);
        assert!(REAR_SERVICE_CLEARANCE >= 280.0);
        assert!(TOP_SERVICE_CLEARANCE >= 360.0);
    }

    #[test]
    fn connector_rows_stay_inside_hardware_envelopes() {
        assert!(additive_connector_span() + ADDITIVE_COLLAR_D < ADDITIVE_BAY_X);
        assert!(bulkhead_connector_span() + ADDITIVE_COLLAR_D < BULKHEAD_X);
        assert!(TUBE_SLOT_W > TUBE_OD);
        assert_eq!(STRAIN_RELIEF_COMBS, comb_positions().len());
        assert_eq!(route_channel_specs().len(), 7);
    }
}
