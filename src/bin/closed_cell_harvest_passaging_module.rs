use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed upstream cell harvest/passaging module for feeding the LaminarForge
// cell-suspension prep/QC and seeding workflow.
//
// Research basis:
// - Closed adherent-cell harvest systems use wash, dissociation enzyme,
//   inhibition/quench, mechanical agitation, repeated harvest cycles, and pump
//   calibration rather than open manual pipetting.
// - Closed wash/concentrate hardware, sterile connectors, sensors, enzyme
//   compatibility, and process acceptance rules should be bought/qualified.
//
// This file models mechanical layout and validation fixtures only. It is not a
// validated biological passaging process.

const OUTPUTS: [&str; 11] = [
    "output/closed_cell_harvest_passaging_module_base_deck.stl",
    "output/closed_cell_harvest_passaging_module_vessel_docking_bay.stl",
    "output/closed_cell_harvest_passaging_module_reagent_bag_carousel.stl",
    "output/closed_cell_harvest_passaging_module_pump_valve_bank.stl",
    "output/closed_cell_harvest_passaging_module_dissociation_incubation_rocker.stl",
    "output/closed_cell_harvest_passaging_module_harvest_collection_qc_loop.stl",
    "output/closed_cell_harvest_passaging_module_closed_wash_concentrate_interface.stl",
    "output/closed_cell_harvest_passaging_module_waste_decon_and_leak_tray.stl",
    "output/closed_cell_harvest_passaging_module_barcode_lot_lands.stl",
    "output/closed_cell_harvest_passaging_module_robot_service_keepouts.stl",
    "output/closed_cell_harvest_passaging_module_assembly.stl",
];

const MODULE_X: f64 = 1320.0;
const MODULE_Y: f64 = 820.0;
const DECK_Z: f64 = 28.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 36.0;

const SOURCE_VESSEL_X: f64 = 420.0;
const SOURCE_VESSEL_Y: f64 = 310.0;
const SOURCE_VESSEL_Z: f64 = 120.0;
const SOURCE_VESSEL_CENTER_X: f64 = -390.0;
const SOURCE_VESSEL_CENTER_Y: f64 = 160.0;

const REAGENT_BAG_COUNT: usize = 6;
const REAGENT_BAG_X: f64 = 116.0;
const REAGENT_BAG_Y: f64 = 74.0;
const REAGENT_BAG_Z: f64 = 150.0;
const REAGENT_PITCH_X: f64 = 142.0;

const PUMP_CHANNELS: usize = 7;
const VALVE_POSITIONS: usize = 14;
const PUMP_BANK_X: f64 = 620.0;
const PUMP_BANK_Y: f64 = 170.0;
const PUMP_BANK_Z: f64 = 92.0;
const PUMP_PITCH: f64 = 82.0;

const ROCKER_X: f64 = 510.0;
const ROCKER_Y: f64 = 350.0;
const ROCKER_SWING_CLEARANCE_Z: f64 = 170.0;

const COLLECTION_X: f64 = 455.0;
const COLLECTION_Y: f64 = 230.0;
const COLLECTION_Z: f64 = 92.0;
const QC_PORTS: usize = 5;

const WASH_INTERFACE_X: f64 = 380.0;
const WASH_INTERFACE_Y: f64 = 255.0;
const WASH_INTERFACE_Z: f64 = 150.0;

const WASTE_TRAY_X: f64 = 600.0;
const WASTE_TRAY_Y: f64 = 190.0;
const WASTE_TRAY_Z: f64 = 54.0;

const LABEL_LAND_COUNT: usize = 18;
const LABEL_LAND_X: f64 = 52.0;
const LABEL_LAND_Y: f64 = 24.0;
const LABEL_LAND_Z: f64 = 4.0;

const ROBOT_SERVICE_Z: f64 = 240.0;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 220.0;

const STERILE_CONNECTOR_COUNT: usize = 16;
const CLOSED_PROCESS_STEPS: usize = 6; // wash, enzyme, incubate, quench, harvest, concentrate.

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let vessel = vessel_docking_bay();
    export(OUTPUTS[1], &vessel);

    let reagents = reagent_bag_carousel();
    export(OUTPUTS[2], &reagents);

    let pump_valves = pump_valve_bank();
    export(OUTPUTS[3], &pump_valves);

    let rocker = dissociation_incubation_rocker();
    export(OUTPUTS[4], &rocker);

    let collection = harvest_collection_qc_loop();
    export(OUTPUTS[5], &collection);

    let wash = closed_wash_concentrate_interface();
    export(OUTPUTS[6], &wash);

    let waste = waste_decon_and_leak_tray();
    export(OUTPUTS[7], &waste);

    let labels = barcode_lot_lands();
    export(OUTPUTS[8], &labels);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + vessel
        + reagents
        + pump_valves
        + rocker
        + collection
        + wash
        + waste
        + labels
        + keepouts
        + tube_route_placeholders();
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cell harvest/passaging module:");
    println!(
        "  Deck footprint:              {MODULE_X:.0}mm x {MODULE_Y:.0}mm x {DECK_Z:.0}mm base"
    );
    println!(
        "  Source vessel bay:           {SOURCE_VESSEL_X:.0}mm x {SOURCE_VESSEL_Y:.0}mm cradle, {SOURCE_VESSEL_Z:.0}mm vessel envelope"
    );
    println!(
        "  Reagent bags:                {REAGENT_BAG_COUNT} closed inputs for wash/enzyme/quench/media/flush/collection"
    );
    println!(
        "  Pump/valve capacity:         {PUMP_CHANNELS} pump lanes, {VALVE_POSITIONS} valve placeholders, {STERILE_CONNECTOR_COUNT} sterile connector positions"
    );
    println!(
        "  Process controls:            {CLOSED_PROCESS_STEPS} closed steps with rocker, QC loop, wash/concentrate interface, waste/decon path"
    );
    println!(
        "  Robot/service clearances:    {ROBOT_SERVICE_Z:.0}mm Z keepout, {FRONT_SERVICE_CLEARANCE:.0}mm front, {REAR_SERVICE_CLEARANCE:.0}mm rear"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert!(PUMP_CHANNELS >= REAGENT_BAG_COUNT);
    assert!(VALVE_POSITIONS >= PUMP_CHANNELS * 2);
    assert!(STERILE_CONNECTOR_COUNT >= REAGENT_BAG_COUNT + QC_PORTS + 4);
    assert!(ROCKER_SWING_CLEARANCE_Z > SOURCE_VESSEL_Z + 40.0);
    assert!(FRONT_SERVICE_CLEARANCE + MODULE_Y > MODULE_Y);
    assert!(LABEL_LAND_COUNT >= STERILE_CONNECTOR_COUNT);
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "closed_cell_harvest_passaging_base_deck",
        MODULE_X,
        MODULE_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let rear_rim = centered_cube(
        "closed_cell_harvest_passaging_rear_rim",
        MODULE_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, MODULE_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front_rim = centered_cube(
        "closed_cell_harvest_passaging_front_drip_lip",
        MODULE_X,
        RIM_W,
        RIM_Z * 0.6,
    )
    .translate(0.0, -MODULE_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z * 0.3);
    let left_rim = centered_cube(
        "closed_cell_harvest_passaging_left_rim",
        RIM_W,
        MODULE_Y,
        RIM_Z,
    )
    .translate(-MODULE_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right_rim = centered_cube(
        "closed_cell_harvest_passaging_right_rim",
        RIM_W,
        MODULE_Y,
        RIM_Z,
    )
    .translate(MODULE_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    let datum_grid = deck_mounting_datums();
    deck + rear_rim + front_rim + left_rim + right_rim + datum_grid
}

fn deck_mounting_datums() -> Part {
    let mut datums = Part::empty("closed_cell_harvest_passaging_deck_mounting_datums");
    for x in [-560.0, -280.0, 0.0, 280.0, 560.0] {
        for y in [-330.0, 0.0, 330.0] {
            let boss = centered_cylinder(
                format!("closed_cell_harvest_passaging_m6_datum_boss_{x}_{y}"),
                22.0,
                7.0,
                48,
            )
            .translate(x, y, DECK_Z + 3.5);
            let hole = centered_cylinder(
                format!("closed_cell_harvest_passaging_m6_clearance_{x}_{y}"),
                7.0,
                8.0,
                32,
            )
            .translate(x, y, DECK_Z + 4.0);
            datums = datums + (boss - hole);
        }
    }
    datums
}

fn vessel_docking_bay() -> Part {
    let base = centered_cube(
        "closed_cell_harvest_vessel_cradle_base",
        SOURCE_VESSEL_X,
        SOURCE_VESSEL_Y,
        44.0,
    )
    .translate(
        SOURCE_VESSEL_CENTER_X,
        SOURCE_VESSEL_CENTER_Y,
        DECK_Z + 22.0,
    );
    let vessel_envelope = centered_cube(
        "closed_cell_harvest_single_use_vessel_envelope",
        SOURCE_VESSEL_X - 72.0,
        SOURCE_VESSEL_Y - 70.0,
        SOURCE_VESSEL_Z,
    )
    .translate(
        SOURCE_VESSEL_CENTER_X,
        SOURCE_VESSEL_CENTER_Y,
        DECK_Z + 44.0 + SOURCE_VESSEL_Z / 2.0,
    );
    let cavity = centered_cube(
        "closed_cell_harvest_vessel_socket_relief",
        SOURCE_VESSEL_X - 105.0,
        SOURCE_VESSEL_Y - 100.0,
        48.0,
    )
    .translate(
        SOURCE_VESSEL_CENTER_X,
        SOURCE_VESSEL_CENTER_Y,
        DECK_Z + 30.0,
    );
    let left_clamp = centered_cube("closed_cell_harvest_vessel_left_clamp", 28.0, 260.0, 88.0)
        .translate(
            SOURCE_VESSEL_CENTER_X - SOURCE_VESSEL_X / 2.0 + 38.0,
            SOURCE_VESSEL_CENTER_Y,
            DECK_Z + 88.0,
        );
    let right_clamp = centered_cube("closed_cell_harvest_vessel_right_clamp", 28.0, 260.0, 88.0)
        .translate(
            SOURCE_VESSEL_CENTER_X + SOURCE_VESSEL_X / 2.0 - 38.0,
            SOURCE_VESSEL_CENTER_Y,
            DECK_Z + 88.0,
        );
    let sterile_port_panel = connector_row(
        "closed_cell_harvest_vessel_port_panel",
        6,
        46.0,
        SOURCE_VESSEL_CENTER_X,
        SOURCE_VESSEL_CENTER_Y - SOURCE_VESSEL_Y / 2.0 - 18.0,
        DECK_Z + 96.0,
    );
    base - cavity + vessel_envelope + left_clamp + right_clamp + sterile_port_panel
}

fn reagent_bag_carousel() -> Part {
    let rack = centered_cube(
        "closed_cell_harvest_reagent_bag_rack_spine",
        REAGENT_BAG_COUNT as f64 * REAGENT_PITCH_X + 64.0,
        48.0,
        132.0,
    )
    .translate(130.0, MODULE_Y / 2.0 - 88.0, DECK_Z + 66.0);

    let mut bags = Part::empty("closed_cell_harvest_reagent_bag_positions");
    for i in 0..REAGENT_BAG_COUNT {
        let x = 130.0 + centered_index(i, REAGENT_BAG_COUNT, REAGENT_PITCH_X);
        let bag = centered_cube(
            format!("closed_cell_harvest_reagent_bag_envelope_{i}"),
            REAGENT_BAG_X,
            REAGENT_BAG_Y,
            REAGENT_BAG_Z,
        )
        .translate(x, MODULE_Y / 2.0 - 166.0, DECK_Z + REAGENT_BAG_Z / 2.0);
        let hanger = centered_cube(
            format!("closed_cell_harvest_reagent_bag_hanger_{i}"),
            REAGENT_BAG_X + 18.0,
            18.0,
            20.0,
        )
        .translate(x, MODULE_Y / 2.0 - 102.0, DECK_Z + REAGENT_BAG_Z + 12.0);
        let neck_receiver = centered_cylinder(
            format!("closed_cell_harvest_reagent_neck_receiver_{i}"),
            20.0,
            20.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, MODULE_Y / 2.0 - 212.0, DECK_Z + 62.0);
        bags = bags + bag + hanger + neck_receiver;
    }

    rack + bags
}

fn pump_valve_bank() -> Part {
    let base = centered_cube(
        "closed_cell_harvest_pump_valve_bank_base",
        PUMP_BANK_X,
        PUMP_BANK_Y,
        32.0,
    )
    .translate(145.0, 94.0, DECK_Z + 16.0);
    let mut pumps = Part::empty("closed_cell_harvest_peristaltic_pump_envelopes");
    for i in 0..PUMP_CHANNELS {
        let x = 145.0 + centered_index(i, PUMP_CHANNELS, PUMP_PITCH);
        let pump = centered_cube(
            format!("closed_cell_harvest_peristaltic_pump_{i}"),
            62.0,
            78.0,
            PUMP_BANK_Z,
        )
        .translate(x, 70.0, DECK_Z + 32.0 + PUMP_BANK_Z / 2.0);
        let tube_window = centered_cube(
            format!("closed_cell_harvest_pump_tube_window_{i}"),
            42.0,
            10.0,
            34.0,
        )
        .translate(x, 22.0, DECK_Z + 78.0);
        pumps = pumps + pump + tube_window;
    }

    let mut valves = Part::empty("closed_cell_harvest_valve_placeholders");
    for i in 0..VALVE_POSITIONS {
        let col = i % 7;
        let row = i / 7;
        let x = 145.0 + centered_index(col, 7, PUMP_PITCH);
        let y = -42.0 - row as f64 * 46.0;
        let valve = centered_cube(
            format!("closed_cell_harvest_pinch_valve_placeholder_{i}"),
            50.0,
            28.0,
            48.0,
        )
        .translate(x, y, DECK_Z + 56.0);
        valves = valves + valve;
    }

    let connector_land = connector_row(
        "closed_cell_harvest_pump_sterile_connector_row",
        10,
        42.0,
        145.0,
        -128.0,
        DECK_Z + 76.0,
    );
    base + pumps + valves + connector_land
}

fn dissociation_incubation_rocker() -> Part {
    let rocker_base = centered_cube(
        "closed_cell_harvest_dissociation_rocker_base",
        ROCKER_X,
        ROCKER_Y,
        42.0,
    )
    .translate(-388.0, -184.0, DECK_Z + 21.0);
    let swing_envelope = centered_cube(
        "closed_cell_harvest_rocker_swing_keepout",
        ROCKER_X - 44.0,
        ROCKER_Y - 36.0,
        ROCKER_SWING_CLEARANCE_Z,
    )
    .translate(
        -388.0,
        -184.0,
        DECK_Z + 42.0 + ROCKER_SWING_CLEARANCE_Z / 2.0,
    );
    let pivot_left = centered_cylinder("closed_cell_harvest_rocker_left_pivot", 34.0, 52.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(-388.0 - ROCKER_X / 2.0 + 44.0, -184.0, DECK_Z + 94.0);
    let pivot_right = centered_cylinder("closed_cell_harvest_rocker_right_pivot", 34.0, 52.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(-388.0 + ROCKER_X / 2.0 - 44.0, -184.0, DECK_Z + 94.0);
    let enzyme_timer_lands = centered_cube(
        "closed_cell_harvest_dissociation_timer_sensor_lands",
        360.0,
        24.0,
        28.0,
    )
    .translate(-388.0, -184.0 - ROCKER_Y / 2.0 - 26.0, DECK_Z + 54.0);
    rocker_base + swing_envelope + pivot_left + pivot_right + enzyme_timer_lands
}

fn harvest_collection_qc_loop() -> Part {
    let block = centered_cube(
        "closed_cell_harvest_collection_qc_loop_block",
        COLLECTION_X,
        COLLECTION_Y,
        COLLECTION_Z,
    )
    .translate(410.0, -204.0, DECK_Z + COLLECTION_Z / 2.0);
    let collection_bag = centered_cube(
        "closed_cell_harvest_collection_bag_envelope",
        190.0,
        132.0,
        120.0,
    )
    .translate(276.0, -214.0, DECK_Z + COLLECTION_Z + 60.0);
    let sample_loop = serpentine_lanes(
        "closed_cell_harvest_qc_sample_loop_lanes",
        QC_PORTS,
        58.0,
        162.0,
        10.0,
    )
    .translate(485.0, -204.0, DECK_Z + COLLECTION_Z + 12.0);
    let qc_ports = connector_row(
        "closed_cell_harvest_qc_sensor_ports",
        QC_PORTS,
        42.0,
        498.0,
        -332.0,
        DECK_Z + 96.0,
    );
    let cell_counter_dock = centered_cube(
        "closed_cell_harvest_at_line_counter_dock_placeholder",
        124.0,
        92.0,
        72.0,
    )
    .translate(610.0, -158.0, DECK_Z + 104.0);
    block + collection_bag + sample_loop + qc_ports + cell_counter_dock
}

fn closed_wash_concentrate_interface() -> Part {
    let skid = centered_cube(
        "closed_cell_harvest_wash_concentrate_bought_processor_envelope",
        WASH_INTERFACE_X,
        WASH_INTERFACE_Y,
        WASH_INTERFACE_Z,
    )
    .translate(500.0, 210.0, DECK_Z + WASH_INTERFACE_Z / 2.0);
    let docking_face = centered_cube(
        "closed_cell_harvest_wash_concentrate_docking_face",
        WASH_INTERFACE_X,
        32.0,
        132.0,
    )
    .translate(500.0, 210.0 - WASH_INTERFACE_Y / 2.0 - 18.0, DECK_Z + 88.0);
    let ports = connector_row(
        "closed_cell_harvest_wash_concentrate_closed_ports",
        6,
        46.0,
        500.0,
        210.0 - WASH_INTERFACE_Y / 2.0 - 38.0,
        DECK_Z + 96.0,
    );
    let scale_pads = load_cell_pad_array(
        "closed_cell_harvest_wash_processor_load_pads",
        500.0,
        210.0,
        WASH_INTERFACE_X - 68.0,
        WASH_INTERFACE_Y - 58.0,
    );
    skid + docking_face + ports + scale_pads
}

fn waste_decon_and_leak_tray() -> Part {
    let tray = centered_cube(
        "closed_cell_harvest_waste_decon_secondary_tray",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 112.0, DECK_Z + WASTE_TRAY_Z / 2.0);
    let cavity = centered_cube(
        "closed_cell_harvest_waste_decon_tray_cavity",
        WASTE_TRAY_X - 42.0,
        WASTE_TRAY_Y - 42.0,
        WASTE_TRAY_Z,
    )
    .translate(
        0.0,
        -MODULE_Y / 2.0 + 112.0,
        DECK_Z + WASTE_TRAY_Z / 2.0 + 14.0,
    );
    let leak_strip = centered_cube(
        "closed_cell_harvest_leak_sensor_strip",
        WASTE_TRAY_X - 74.0,
        12.0,
        8.0,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 34.0, DECK_Z + WASTE_TRAY_Z + 6.0);
    let waste_ports = connector_row(
        "closed_cell_harvest_waste_decon_ports",
        4,
        54.0,
        220.0,
        -MODULE_Y / 2.0 + 112.0,
        DECK_Z + WASTE_TRAY_Z + 28.0,
    );
    tray - cavity + leak_strip + waste_ports
}

fn barcode_lot_lands() -> Part {
    let mut lands = Part::empty("closed_cell_harvest_barcode_lot_lands");
    for i in 0..LABEL_LAND_COUNT {
        let col = i % 9;
        let row = i / 9;
        let x = -260.0 + col as f64 * 64.0;
        let y = MODULE_Y / 2.0 - 34.0 - row as f64 * 34.0;
        let land = centered_cube(
            format!("closed_cell_harvest_barcode_lot_land_{i}"),
            LABEL_LAND_X,
            LABEL_LAND_Y,
            LABEL_LAND_Z,
        )
        .translate(x, y, DECK_Z + RIM_Z + LABEL_LAND_Z / 2.0 + 6.0);
        lands = lands + land;
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_cell_harvest_front_robot_service_keepout",
        MODULE_X,
        FRONT_SERVICE_CLEARANCE,
        ROBOT_SERVICE_Z,
    )
    .translate(
        0.0,
        -MODULE_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        DECK_Z + ROBOT_SERVICE_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_cell_harvest_rear_tubing_service_keepout",
        MODULE_X,
        REAR_SERVICE_CLEARANCE,
        180.0,
    )
    .translate(
        0.0,
        MODULE_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        DECK_Z + 90.0,
    );
    let top = centered_cube(
        "closed_cell_harvest_overhead_vessel_removal_keepout",
        SOURCE_VESSEL_X + 120.0,
        SOURCE_VESSEL_Y + 120.0,
        220.0,
    )
    .translate(
        SOURCE_VESSEL_CENTER_X,
        SOURCE_VESSEL_CENTER_Y,
        DECK_Z + SOURCE_VESSEL_Z + 160.0,
    );
    front + rear + top
}

fn tube_route_placeholders() -> Part {
    let mut routes = Part::empty("closed_cell_harvest_tube_route_placeholders");
    let route_specs = [
        (-235.0, 116.0, 505.0, 12.0),
        (-85.0, 52.0, 585.0, 12.0),
        (145.0, -16.0, 600.0, 12.0),
        (250.0, -126.0, 420.0, 12.0),
    ];
    for (i, (x, y, len, height)) in route_specs.iter().enumerate() {
        let route = centered_cube(
            format!("closed_cell_harvest_tube_route_channel_{i}"),
            *len,
            12.0,
            *height,
        )
        .translate(*x, *y, DECK_Z + 72.0 + i as f64 * 8.0);
        routes = routes + route;
    }
    routes
}

fn connector_row(label: &str, count: usize, pitch: f64, x: f64, y: f64, z: f64) -> Part {
    let mut row = Part::empty(label.to_string());
    for i in 0..count {
        let port = centered_cylinder(format!("{label}_port_{i}"), 22.0, 18.0, 48)
            .rotate(90.0, 0.0, 0.0)
            .translate(x + centered_index(i, count, pitch), y, z);
        let label_land = centered_cube(format!("{label}_label_land_{i}"), 32.0, 5.0, 16.0)
            .translate(x + centered_index(i, count, pitch), y - 13.0, z + 24.0);
        row = row + port + label_land;
    }
    row
}

fn serpentine_lanes(label: &str, lanes: usize, pitch: f64, length: f64, height: f64) -> Part {
    let mut part = Part::empty(label.to_string());
    for i in 0..lanes {
        let y = centered_index(i, lanes, pitch);
        let lane = centered_cube(format!("{label}_straight_lane_{i}"), length, 8.0, height)
            .translate(0.0, y, 0.0);
        let u_bend = centered_cylinder(format!("{label}_u_bend_{i}"), 18.0, height, 48).translate(
            length / 2.0,
            y,
            0.0,
        );
        part = part + lane + u_bend;
    }
    part
}

fn load_cell_pad_array(label: &str, x: f64, y: f64, span_x: f64, span_y: f64) -> Part {
    let mut pads = Part::empty(label.to_string());
    for (i, (dx, dy)) in [
        (-span_x / 2.0, -span_y / 2.0),
        (span_x / 2.0, -span_y / 2.0),
        (-span_x / 2.0, span_y / 2.0),
        (span_x / 2.0, span_y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cube(format!("{label}_{i}"), 52.0, 38.0, 10.0).translate(
            x + dx,
            y + dy,
            DECK_Z + 8.0,
        );
        pads = pads + pad;
    }
    pads
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pump_and_connector_capacity_cover_closed_harvest_steps() {
        assert!(PUMP_CHANNELS >= REAGENT_BAG_COUNT);
        assert!(VALVE_POSITIONS >= PUMP_CHANNELS * 2);
        assert!(STERILE_CONNECTOR_COUNT >= REAGENT_BAG_COUNT + QC_PORTS + 4);
        assert_eq!(CLOSED_PROCESS_STEPS, 6);
    }

    #[test]
    fn source_vessel_and_rocker_fit_on_deck() {
        assert!(SOURCE_VESSEL_CENTER_X - SOURCE_VESSEL_X / 2.0 > -MODULE_X / 2.0 + RIM_W);
        assert!(SOURCE_VESSEL_CENTER_Y + SOURCE_VESSEL_Y / 2.0 < MODULE_Y / 2.0 - RIM_W);
        assert!(ROCKER_X + SOURCE_VESSEL_X < MODULE_X);
        assert!(ROCKER_SWING_CLEARANCE_Z > SOURCE_VESSEL_Z);
    }

    #[test]
    fn service_keepouts_are_explicit_for_robot_and_tubing_access() {
        assert!(ROBOT_SERVICE_Z >= 200.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_SERVICE_CLEARANCE >= 180.0);
    }

    #[test]
    fn output_names_are_unique_and_scoped() {
        let mut names = OUTPUTS.to_vec();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), OUTPUTS.len());
        assert!(OUTPUTS
            .iter()
            .all(|name| name.contains("closed_cell_harvest_passaging_module")));
    }
}
