use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent thaw/equilibration station for the automated cell-culture workcell.
//
// Research assumptions used for the concept geometry:
// - Dry thawing devices avoid water-bath contamination, support independent thaw
//   chambers, real-time temperature capture, spill containment, and barcode tracking.
// - Controlled freeze/thaw single-use bags commonly expose thermowells, vent filters,
//   and sterile connector lines; those interfaces need datum features, not open handling.
// - Commercial aseptic connector families cover the 4-40 C fluid-transfer envelope, so
//   the station models a closed connector bulkhead and handoff manifold rather than
//   operator-open pouring.
//
// This file is mechanical concept CAD only. It is not a biological protocol, thaw-cycle
// validation, sterility claim, or cold-chain qualification.

const OUTPUTS: &[&str] = &[
    "output/closed_reagent_thaw_equilibration_station_base_leak_tray.stl",
    "output/closed_reagent_thaw_equilibration_station_frozen_receiving_nest.stl",
    "output/closed_reagent_thaw_equilibration_station_controlled_thaw_block.stl",
    "output/closed_reagent_thaw_equilibration_station_sterile_connector_bulkhead.stl",
    "output/closed_reagent_thaw_equilibration_station_inline_temperature_sensor_pockets.stl",
    "output/closed_reagent_thaw_equilibration_station_barcode_lot_scan_lands.stl",
    "output/closed_reagent_thaw_equilibration_station_handoff_manifold_positions.stl",
    "output/closed_reagent_thaw_equilibration_station_robot_service_keepouts.stl",
    "output/closed_reagent_thaw_equilibration_station_assembly.stl",
];

const DECK_X: f64 = 1240.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 18.0;
const LEAK_PAN_CURB_W: f64 = 22.0;
const LEAK_PAN_CURB_Z: f64 = 34.0;
const SUMP_X: f64 = 1090.0;
const SUMP_Y: f64 = 640.0;
const SUMP_Z: f64 = 8.0;
const DRAIN_PORT_D: f64 = 18.0;
const DATUM_PIN_D: f64 = 8.0;
const DATUM_PIN_COUNT: usize = 6;

const BAG_BAYS: usize = 2;
const BAG_BAY_X: f64 = 188.0;
const BAG_BAY_Y: f64 = 248.0;
const BAG_BAY_RECESS_Z: f64 = 18.0;
const RECEIVING_NEST_X: f64 = 470.0;
const RECEIVING_NEST_Y: f64 = 322.0;
const RECEIVING_NEST_Z: f64 = 54.0;
const RECEIVING_CENTER_X: f64 = -345.0;
const RECEIVING_CENTER_Y: f64 = -220.0;
const VIAL_ROWS: usize = 3;
const VIAL_COLS: usize = 4;
const VIAL_POSITIONS: usize = VIAL_ROWS * VIAL_COLS;
const VIAL_WELL_D: f64 = 24.0;
const VIAL_PITCH_X: f64 = 42.0;
const VIAL_PITCH_Y: f64 = 42.0;

const THAW_BLOCK_X: f64 = 520.0;
const THAW_BLOCK_Y: f64 = 334.0;
const THAW_BLOCK_Z: f64 = 74.0;
const THAW_CENTER_X: f64 = 20.0;
const THAW_CENTER_Y: f64 = 140.0;
const THAW_CHAMBER_X: f64 = 210.0;
const THAW_CHAMBER_Y: f64 = 252.0;
const THAW_CHAMBER_RECESS_Z: f64 = 20.0;
const INDEPENDENT_THAW_CHAMBERS: usize = 2;
const EQUILIBRATION_LANES: usize = 4;

const BULKHEAD_X: f64 = 760.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 222.0;
const BULKHEAD_CENTER_Y: f64 = DECK_Y / 2.0 - 56.0;
const CONNECTOR_PORTS: usize = 8;
const CONNECTOR_PITCH_X: f64 = 86.0;
const CONNECTOR_PORT_D: f64 = 27.0;
const CONNECTOR_COLLAR_D: f64 = 45.0;
const FILTER_COUNT: usize = 2;

const TEMP_SENSOR_POCKETS: usize = 10;
const SENSOR_BLOCK_X: f64 = 46.0;
const SENSOR_BLOCK_Y: f64 = 24.0;
const SENSOR_BLOCK_Z: f64 = 22.0;
const SENSOR_BORE_D: f64 = 5.2;

const BARCODE_LANDS: usize = 10;
const LABEL_LAND_X: f64 = 86.0;
const LABEL_LAND_Y: f64 = 34.0;
const LABEL_LAND_Z: f64 = 4.0;

const HANDOFFS: usize = 3;
const HANDOFF_DOCK_X: f64 = 172.0;
const HANDOFF_DOCK_Y: f64 = 76.0;
const HANDOFF_DOCK_Z: f64 = 28.0;
const HANDOFF_CENTER_X: f64 = DECK_X / 2.0 - 128.0;
const HANDOFF_PITCH_Y: f64 = 176.0;
const HANDOFF_CONNECTORS_PER_DOCK: usize = 3;

const FRONT_ROBOT_APPROACH: f64 = 440.0;
const REAR_SERVICE_CLEARANCE: f64 = 280.0;
const SIDE_SERVICE_CLEARANCE: f64 = 240.0;
const TOP_SERVICE_CLEARANCE: f64 = 360.0;
const HANDOFF_PULL_CLEARANCE: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let receiving = frozen_receiving_nest();
    export(OUTPUTS[1], &receiving);

    let thaw = controlled_thaw_equilibration_block();
    export(OUTPUTS[2], &thaw);

    let bulkhead = sterile_connector_bulkhead();
    export(OUTPUTS[3], &bulkhead);

    let sensors = inline_temperature_sensor_pockets();
    export(OUTPUTS[4], &sensors);

    let scan = barcode_lot_scan_lands();
    export(OUTPUTS[5], &scan);

    let handoff = handoff_manifold_positions();
    export(OUTPUTS[6], &handoff);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[7], &keepouts);

    let assembly = base + receiving + thaw + bulkhead + sensors + scan + handoff + keepouts;
    export(OUTPUTS[8], &assembly);

    println!();
    println!("Closed reagent thaw/equilibration station:");
    println!(
        "  Deck and leak pan:          {DECK_X:.0}mm W x {DECK_Y:.0}mm D x {DECK_Z:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm recessed sump, {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Frozen receiving nest:      {BAG_BAYS} frozen bag bays plus {VIAL_POSITIONS} vial positions on a {RECEIVING_NEST_X:.0}mm x {RECEIVING_NEST_Y:.0}mm cold receiving plate"
    );
    println!(
        "  Dry thaw/equilibration:     {INDEPENDENT_THAW_CHAMBERS} independent dry thaw chambers, {EQUILIBRATION_LANES} outlet equilibration lanes, block {THAW_BLOCK_X:.0}mm x {THAW_BLOCK_Y:.0}mm x {THAW_BLOCK_Z:.0}mm"
    );
    println!(
        "  Closed connections:         {CONNECTOR_PORTS} sterile connector bulkhead ports, {FILTER_COUNT} vent/filter placeholders, cap and tube-comb staging"
    );
    println!(
        "  Monitoring and identity:    {TEMP_SENSOR_POCKETS} inline temperature/thermowell pockets, {BARCODE_LANDS} barcode/lot scan lands"
    );
    println!(
        "  Closed handoffs:            {HANDOFFS} docked outputs for media conditioning, seeding, and passaging modules"
    );
    println!(
        "  Robot/service keepouts:     front approach {FRONT_ROBOT_APPROACH:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, side service {SIDE_SERVICE_CLEARANCE:.0}mm, top {TOP_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert!(RECEIVING_CENTER_X - RECEIVING_NEST_X / 2.0 > -DECK_X / 2.0 + 36.0);
    assert!(receiving_right_edge() < THAW_CENTER_X - THAW_BLOCK_X / 2.0 + 138.0);
    assert!(
        RECEIVING_CENTER_Y + RECEIVING_NEST_Y / 2.0 < THAW_CENTER_Y - THAW_BLOCK_Y / 2.0 - 24.0
    );
    assert!(thaw_right_edge() < HANDOFF_CENTER_X - HANDOFF_DOCK_X / 2.0 - 80.0);
    assert!(THAW_CENTER_Y + THAW_BLOCK_Y / 2.0 < BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 6.0);
    assert!(connector_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
    assert!(handoff_y(HANDOFFS - 1).abs() + HANDOFF_DOCK_Y / 2.0 < DECK_Y / 2.0 - 72.0);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("reagent_thaw_station_base_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let sump = centered_cube(
        "reagent_thaw_station_recessed_leak_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -10.0, DECK_Z - SUMP_Z / 2.0 + 0.5);

    let drain = centered_cylinder(
        "reagent_thaw_station_leak_pan_drain_cut",
        DRAIN_PORT_D / 2.0,
        LEAK_PAN_CURB_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 14.0, DECK_Z - 5.0);

    deck - sump - drain + leak_pan_curbs() + datum_pin_bosses() + floor_zone_markers()
}

fn leak_pan_curbs() -> Part {
    let front = centered_cube(
        "reagent_thaw_station_front_leak_curb",
        DECK_X,
        LEAK_PAN_CURB_W,
        LEAK_PAN_CURB_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + LEAK_PAN_CURB_W / 2.0,
        DECK_Z + LEAK_PAN_CURB_Z / 2.0,
    );
    let rear = centered_cube(
        "reagent_thaw_station_rear_leak_curb",
        DECK_X,
        LEAK_PAN_CURB_W,
        LEAK_PAN_CURB_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - LEAK_PAN_CURB_W / 2.0,
        DECK_Z + LEAK_PAN_CURB_Z / 2.0,
    );
    let left = centered_cube(
        "reagent_thaw_station_left_leak_curb",
        LEAK_PAN_CURB_W,
        DECK_Y,
        LEAK_PAN_CURB_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEAK_PAN_CURB_W / 2.0,
        0.0,
        DECK_Z + LEAK_PAN_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "reagent_thaw_station_right_leak_curb",
        LEAK_PAN_CURB_W,
        DECK_Y,
        LEAK_PAN_CURB_Z,
    )
    .translate(
        DECK_X / 2.0 - LEAK_PAN_CURB_W / 2.0,
        0.0,
        DECK_Z + LEAK_PAN_CURB_Z / 2.0,
    );

    front + rear + left + right
}

fn datum_pin_bosses() -> Part {
    let mut bosses = Part::empty("reagent_thaw_station_datum_pin_bosses");
    for (i, (x, y)) in datum_pin_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("reagent_thaw_station_datum_pin_boss_{i}"),
            14.0,
            10.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        let hole = centered_cylinder(
            format!("reagent_thaw_station_datum_pin_clearance_{i}"),
            DATUM_PIN_D / 2.0,
            14.0,
            28,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn floor_zone_markers() -> Part {
    let receiving_marker = centered_cube(
        "reagent_thaw_station_receiving_zone_floor_marker",
        RECEIVING_NEST_X + 40.0,
        RECEIVING_NEST_Y + 36.0,
        3.0,
    )
    .translate(RECEIVING_CENTER_X, RECEIVING_CENTER_Y, DECK_Z + 1.5);
    let thaw_marker = centered_cube(
        "reagent_thaw_station_thaw_zone_floor_marker",
        THAW_BLOCK_X + 44.0,
        THAW_BLOCK_Y + 38.0,
        3.0,
    )
    .translate(THAW_CENTER_X, THAW_CENTER_Y, DECK_Z + 1.5);
    let handoff_marker = centered_cube(
        "reagent_thaw_station_handoff_zone_floor_marker",
        HANDOFF_DOCK_X + 66.0,
        HANDOFF_PITCH_Y * 2.0 + HANDOFF_DOCK_Y + 50.0,
        3.0,
    )
    .translate(HANDOFF_CENTER_X, 0.0, DECK_Z + 1.5);

    receiving_marker + thaw_marker + handoff_marker
}

fn frozen_receiving_nest() -> Part {
    let body = centered_cube(
        "reagent_thaw_station_frozen_receiving_cold_plate",
        RECEIVING_NEST_X,
        RECEIVING_NEST_Y,
        RECEIVING_NEST_Z,
    );

    let mut cuts = Part::empty("reagent_thaw_station_receiving_recess_cuts");
    for i in 0..BAG_BAYS {
        let x = bag_bay_x(i);
        cuts =
            cuts + centered_cube(
                format!("reagent_thaw_station_frozen_bag_recess_{i}"),
                BAG_BAY_X,
                BAG_BAY_Y,
                BAG_BAY_RECESS_Z + 1.0,
            )
            .translate(
                x,
                -34.0,
                RECEIVING_NEST_Z / 2.0 - BAG_BAY_RECESS_Z / 2.0 + 0.5,
            ) + centered_cube(
                format!("reagent_thaw_station_bag_pull_finger_slot_{i}"),
                48.0,
                28.0,
                BAG_BAY_RECESS_Z + 2.0,
            )
            .translate(
                x,
                -RECEIVING_NEST_Y / 2.0 + 34.0,
                RECEIVING_NEST_Z / 2.0 - 8.0,
            );
    }

    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let (x, y) = vial_position(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("reagent_thaw_station_vial_well_r{row}_c{col}"),
                    VIAL_WELL_D / 2.0,
                    34.0,
                    36,
                )
                .translate(x, y, RECEIVING_NEST_Z / 2.0 - 12.0);
        }
    }

    let cold_plate = body - cuts;
    cold_plate.translate(
        RECEIVING_CENTER_X,
        RECEIVING_CENTER_Y,
        DECK_Z + RECEIVING_NEST_Z / 2.0,
    ) + receiving_bag_retainer_lips()
        + vial_row_labels()
        + insulated_transfer_sleeve_land()
}

fn receiving_bag_retainer_lips() -> Part {
    let mut lips = Part::empty("reagent_thaw_station_bag_retainer_lips");
    for i in 0..BAG_BAYS {
        let x = RECEIVING_CENTER_X + bag_bay_x(i);
        let y = RECEIVING_CENTER_Y - 34.0;
        lips =
            lips + centered_cube(
                format!("reagent_thaw_station_bag_recess_front_lip_{i}"),
                BAG_BAY_X + 34.0,
                12.0,
                20.0,
            )
            .translate(
                x,
                y - BAG_BAY_Y / 2.0 - 6.0,
                DECK_Z + RECEIVING_NEST_Z + 10.0,
            ) + centered_cube(
                format!("reagent_thaw_station_bag_recess_rear_lip_{i}"),
                BAG_BAY_X + 34.0,
                12.0,
                20.0,
            )
            .translate(
                x,
                y + BAG_BAY_Y / 2.0 + 6.0,
                DECK_Z + RECEIVING_NEST_Z + 10.0,
            ) + centered_cube(
                format!("reagent_thaw_station_bag_recess_side_lip_{i}"),
                12.0,
                BAG_BAY_Y + 14.0,
                20.0,
            )
            .translate(
                x - BAG_BAY_X / 2.0 - 6.0,
                y,
                DECK_Z + RECEIVING_NEST_Z + 10.0,
            ) + centered_cube(
                format!("reagent_thaw_station_bag_recess_inner_lip_{i}"),
                12.0,
                BAG_BAY_Y + 14.0,
                20.0,
            )
            .translate(
                x + BAG_BAY_X / 2.0 + 6.0,
                y,
                DECK_Z + RECEIVING_NEST_Z + 10.0,
            );
    }
    lips
}

fn vial_row_labels() -> Part {
    let mut labels = Part::empty("reagent_thaw_station_vial_row_label_lands");
    for row in 0..VIAL_ROWS {
        let y = RECEIVING_CENTER_Y + vial_position(row, 0).1;
        labels = labels
            + centered_cube(
                format!("reagent_thaw_station_vial_row_label_land_{row}"),
                34.0,
                28.0,
                4.0,
            )
            .translate(
                RECEIVING_CENTER_X + RECEIVING_NEST_X / 2.0 - 40.0,
                y,
                DECK_Z + RECEIVING_NEST_Z + 2.0,
            );
    }
    labels
}

fn insulated_transfer_sleeve_land() -> Part {
    let body = centered_cube(
        "reagent_thaw_station_insulated_transfer_sleeve_land",
        RECEIVING_NEST_X - 86.0,
        36.0,
        20.0,
    )
    .translate(
        RECEIVING_CENTER_X,
        RECEIVING_CENTER_Y - RECEIVING_NEST_Y / 2.0 - 24.0,
        DECK_Z + RECEIVING_NEST_Z / 2.0,
    );
    let key = centered_cube(
        "reagent_thaw_station_insulated_transfer_sleeve_key_slot",
        82.0,
        40.0,
        8.0,
    )
    .translate(
        RECEIVING_CENTER_X,
        RECEIVING_CENTER_Y - RECEIVING_NEST_Y / 2.0 - 24.0,
        DECK_Z + RECEIVING_NEST_Z / 2.0 + 8.0,
    );

    body - key
}

fn controlled_thaw_equilibration_block() -> Part {
    let block = centered_cube(
        "reagent_thaw_station_dry_thaw_equilibration_block",
        THAW_BLOCK_X,
        THAW_BLOCK_Y,
        THAW_BLOCK_Z,
    );

    let mut chamber_cuts = Part::empty("reagent_thaw_station_thaw_chamber_cuts");
    for i in 0..INDEPENDENT_THAW_CHAMBERS {
        let x = thaw_chamber_x(i);
        chamber_cuts = chamber_cuts
            + centered_cube(
                format!("reagent_thaw_station_dry_thaw_bag_chamber_{i}"),
                THAW_CHAMBER_X,
                THAW_CHAMBER_Y,
                THAW_CHAMBER_RECESS_Z + 1.0,
            )
            .translate(
                x,
                -20.0,
                THAW_BLOCK_Z / 2.0 - THAW_CHAMBER_RECESS_Z / 2.0 + 0.5,
            )
            + centered_cylinder(
                format!("reagent_thaw_station_chamber_thermowell_relief_{i}"),
                7.0 / 2.0,
                THAW_CHAMBER_X + 22.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, THAW_CHAMBER_Y / 2.0 - 38.0, THAW_BLOCK_Z / 2.0 - 14.0);
    }

    let block =
        (block - chamber_cuts).translate(THAW_CENTER_X, THAW_CENTER_Y, DECK_Z + THAW_BLOCK_Z / 2.0);

    block + thaw_lid_clamp_frames() + peltier_water_jacket_envelopes() + equilibration_lane_comb()
}

fn thaw_lid_clamp_frames() -> Part {
    let mut frames = Part::empty("reagent_thaw_station_thaw_lid_clamp_frames");
    for i in 0..INDEPENDENT_THAW_CHAMBERS {
        let x = THAW_CENTER_X + thaw_chamber_x(i);
        let y = THAW_CENTER_Y - 20.0;
        let z = DECK_Z + THAW_BLOCK_Z + 12.0;
        frames = frames
            + centered_cube(
                format!("reagent_thaw_station_thaw_lid_front_clamp_{i}"),
                THAW_CHAMBER_X + 28.0,
                10.0,
                24.0,
            )
            .translate(x, y - THAW_CHAMBER_Y / 2.0 - 5.0, z)
            + centered_cube(
                format!("reagent_thaw_station_thaw_lid_rear_clamp_{i}"),
                THAW_CHAMBER_X + 28.0,
                10.0,
                24.0,
            )
            .translate(x, y + THAW_CHAMBER_Y / 2.0 + 5.0, z)
            + centered_cube(
                format!("reagent_thaw_station_thaw_lid_left_clamp_{i}"),
                10.0,
                THAW_CHAMBER_Y + 20.0,
                24.0,
            )
            .translate(x - THAW_CHAMBER_X / 2.0 - 5.0, y, z)
            + centered_cube(
                format!("reagent_thaw_station_thaw_lid_right_clamp_{i}"),
                10.0,
                THAW_CHAMBER_Y + 20.0,
                24.0,
            )
            .translate(x + THAW_CHAMBER_X / 2.0 + 5.0, y, z);
    }
    frames
}

fn peltier_water_jacket_envelopes() -> Part {
    let rear_manifold = centered_cube(
        "reagent_thaw_station_thermal_fluid_rear_manifold_envelope",
        THAW_BLOCK_X - 42.0,
        36.0,
        48.0,
    )
    .translate(
        THAW_CENTER_X,
        THAW_CENTER_Y + THAW_BLOCK_Y / 2.0 + 28.0,
        DECK_Z + 42.0,
    );
    let left_tec = centered_cube(
        "reagent_thaw_station_left_chamber_tec_or_heater_plate_envelope",
        176.0,
        118.0,
        18.0,
    )
    .translate(
        THAW_CENTER_X + thaw_chamber_x(0),
        THAW_CENTER_Y - 20.0,
        DECK_Z + 8.0,
    );
    let right_tec = centered_cube(
        "reagent_thaw_station_right_chamber_tec_or_heater_plate_envelope",
        176.0,
        118.0,
        18.0,
    )
    .translate(
        THAW_CENTER_X + thaw_chamber_x(1),
        THAW_CENTER_Y - 20.0,
        DECK_Z + 8.0,
    );
    let cable_gland = centered_cube(
        "reagent_thaw_station_thermal_control_cable_gland_land",
        132.0,
        24.0,
        44.0,
    )
    .translate(
        THAW_CENTER_X - THAW_BLOCK_X / 2.0 + 72.0,
        THAW_CENTER_Y + THAW_BLOCK_Y / 2.0 + 58.0,
        DECK_Z + 58.0,
    );

    rear_manifold + left_tec + right_tec + cable_gland
}

fn equilibration_lane_comb() -> Part {
    let comb_body = centered_cube(
        "reagent_thaw_station_outlet_equilibration_lane_comb",
        THAW_BLOCK_X - 88.0,
        58.0,
        26.0,
    )
    .translate(
        THAW_CENTER_X,
        THAW_CENTER_Y - THAW_BLOCK_Y / 2.0 - 42.0,
        DECK_Z + THAW_BLOCK_Z - 8.0,
    );

    let mut cuts = Part::empty("reagent_thaw_station_outlet_equilibration_lane_cuts");
    for i in 0..EQUILIBRATION_LANES {
        let x = lane_x(i, EQUILIBRATION_LANES, 92.0);
        cuts = cuts
            + centered_cylinder(
                format!("reagent_thaw_station_equilibration_tube_channel_{i}"),
                8.0 / 2.0,
                THAW_BLOCK_X - 120.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                THAW_CENTER_X + x,
                THAW_CENTER_Y - THAW_BLOCK_Y / 2.0 - 42.0,
                DECK_Z + THAW_BLOCK_Z - 8.0,
            );
    }

    comb_body - cuts
}

fn sterile_connector_bulkhead() -> Part {
    let panel = centered_cube(
        "reagent_thaw_station_sterile_connector_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, DECK_Z + BULKHEAD_Z / 2.0);

    let mut port_cuts = Part::empty("reagent_thaw_station_sterile_connector_bulkhead_port_cuts");
    for i in 0..CONNECTOR_PORTS {
        port_cuts = port_cuts
            + centered_cylinder(
                format!("reagent_thaw_station_sterile_connector_port_cut_{i}"),
                CONNECTOR_PORT_D / 2.0,
                BULKHEAD_Y + 12.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                connector_x(i),
                BULKHEAD_CENTER_Y,
                DECK_Z + 78.0 + (i % 2) as f64 * 64.0,
            );
    }

    panel - port_cuts + connector_collars() + connector_cap_staging() + sterile_filter_bank()
}

fn connector_collars() -> Part {
    let mut collars = Part::empty("reagent_thaw_station_sterile_connector_collars");
    for i in 0..CONNECTOR_PORTS {
        let x = connector_x(i);
        let z = DECK_Z + 78.0 + (i % 2) as f64 * 64.0;
        let y = BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 7.0;
        let outer = centered_cylinder(
            format!("reagent_thaw_station_connector_outer_collar_{i}"),
            CONNECTOR_COLLAR_D / 2.0,
            16.0,
            44,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z);
        let inner = centered_cylinder(
            format!("reagent_thaw_station_connector_inner_opening_{i}"),
            CONNECTOR_PORT_D / 2.0,
            18.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z);
        collars = collars + (outer - inner);
    }
    collars
}

fn connector_cap_staging() -> Part {
    let tray = centered_cube(
        "reagent_thaw_station_connector_cap_staging_tray",
        312.0,
        44.0,
        18.0,
    )
    .translate(
        BULKHEAD_X / 2.0 - 180.0,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 48.0,
        DECK_Z + 38.0,
    );

    let mut wells = Part::empty("reagent_thaw_station_connector_cap_well_cuts");
    for i in 0..CONNECTOR_PORTS {
        wells = wells
            + centered_cylinder(
                format!("reagent_thaw_station_connector_cap_well_{i}"),
                13.0,
                22.0,
                32,
            )
            .translate(
                BULKHEAD_X / 2.0 - 300.0 + i as f64 * 34.0,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 48.0,
                DECK_Z + 42.0,
            );
    }

    tray - wells
}

fn sterile_filter_bank() -> Part {
    let mut bank = Part::empty("reagent_thaw_station_vent_filter_bank");
    for i in 0..FILTER_COUNT {
        let x = -BULKHEAD_X / 2.0 + 78.0 + i as f64 * 72.0;
        let holder = centered_cylinder(
            format!("reagent_thaw_station_vent_filter_holder_{i}"),
            22.0,
            54.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 22.0,
            DECK_Z + 172.0,
        );
        let bore = centered_cylinder(
            format!("reagent_thaw_station_vent_filter_open_bore_{i}"),
            12.0,
            58.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 22.0,
            DECK_Z + 172.0,
        );
        bank = bank + (holder - bore);
    }
    bank
}

fn inline_temperature_sensor_pockets() -> Part {
    let mut pockets = Part::empty("reagent_thaw_station_inline_temperature_sensor_pockets");
    for (i, (x, y, z, rot_y)) in sensor_positions().iter().enumerate() {
        pockets = pockets + sensor_pocket(i, *x, *y, *z, *rot_y);
    }
    pockets
}

fn sensor_pocket(index: usize, x: f64, y: f64, z: f64, rot_y: bool) -> Part {
    let body = centered_cube(
        format!("reagent_thaw_station_sensor_pocket_body_{index}"),
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    );
    let bore = if rot_y {
        centered_cylinder(
            format!("reagent_thaw_station_sensor_pocket_bore_{index}"),
            SENSOR_BORE_D / 2.0,
            SENSOR_BLOCK_X + 8.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
    } else {
        centered_cylinder(
            format!("reagent_thaw_station_sensor_pocket_bore_{index}"),
            SENSOR_BORE_D / 2.0,
            SENSOR_BLOCK_Y + 8.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
    };

    (body - bore).translate(x, y, z)
}

fn barcode_lot_scan_lands() -> Part {
    let mut lands = Part::empty("reagent_thaw_station_barcode_lot_scan_lands");
    for (i, (x, y)) in barcode_land_positions().iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("reagent_thaw_station_barcode_lot_scan_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(*x, *y, DECK_Z + LABEL_LAND_Z / 2.0 + 3.0);
    }

    lands + scan_bridge()
}

fn scan_bridge() -> Part {
    let left_post = centered_cube(
        "reagent_thaw_station_scan_bridge_left_post",
        24.0,
        24.0,
        210.0,
    )
    .translate(-DECK_X / 2.0 + 86.0, -DECK_Y / 2.0 + 110.0, DECK_Z + 105.0);
    let right_post = centered_cube(
        "reagent_thaw_station_scan_bridge_right_post",
        24.0,
        24.0,
        210.0,
    )
    .translate(-DECK_X / 2.0 + 414.0, -DECK_Y / 2.0 + 110.0, DECK_Z + 105.0);
    let crossbar = centered_cube(
        "reagent_thaw_station_scan_bridge_camera_light_crossbar",
        360.0,
        34.0,
        28.0,
    )
    .translate(-DECK_X / 2.0 + 250.0, -DECK_Y / 2.0 + 110.0, DECK_Z + 208.0);
    let camera = centered_cube(
        "reagent_thaw_station_scan_bridge_camera_envelope",
        68.0,
        48.0,
        58.0,
    )
    .translate(-DECK_X / 2.0 + 250.0, -DECK_Y / 2.0 + 130.0, DECK_Z + 170.0);

    left_post + right_post + crossbar + camera
}

fn handoff_manifold_positions() -> Part {
    let mut docks = Part::empty("reagent_thaw_station_handoff_manifold_positions");
    for i in 0..HANDOFFS {
        docks = docks + handoff_dock(i);
    }
    docks + handoff_tubing_lane_guides()
}

fn handoff_dock(index: usize) -> Part {
    let y = handoff_y(index);
    let dock = centered_cube(
        format!("reagent_thaw_station_handoff_dock_{index}"),
        HANDOFF_DOCK_X,
        HANDOFF_DOCK_Y,
        HANDOFF_DOCK_Z,
    )
    .translate(HANDOFF_CENTER_X, y, DECK_Z + HANDOFF_DOCK_Z / 2.0 + 2.0);

    let key = centered_cube(
        format!("reagent_thaw_station_handoff_datum_key_slot_{index}"),
        66.0,
        HANDOFF_DOCK_Y + 4.0,
        8.0,
    )
    .translate(HANDOFF_CENTER_X, y, DECK_Z + HANDOFF_DOCK_Z + 2.0);

    let mut bosses = Part::empty(format!(
        "reagent_thaw_station_handoff_connector_bosses_{index}"
    ));
    for c in 0..HANDOFF_CONNECTORS_PER_DOCK {
        bosses = bosses
            + centered_cylinder(
                format!("reagent_thaw_station_handoff_connector_boss_{index}_{c}"),
                13.0,
                18.0,
                30,
            )
            .translate(
                HANDOFF_CENTER_X - 48.0 + c as f64 * 48.0,
                y,
                DECK_Z + HANDOFF_DOCK_Z + 13.0,
            );
    }

    let module_label_land = centered_cube(
        format!("reagent_thaw_station_handoff_module_label_land_{index}"),
        112.0,
        24.0,
        4.0,
    )
    .translate(
        HANDOFF_CENTER_X,
        y - HANDOFF_DOCK_Y / 2.0 - 18.0,
        DECK_Z + HANDOFF_DOCK_Z + 7.0,
    );

    dock - key + bosses + module_label_land
}

fn handoff_tubing_lane_guides() -> Part {
    let mut guides = Part::empty("reagent_thaw_station_handoff_tubing_lane_guides");
    for i in 0..HANDOFFS {
        let y = handoff_y(i);
        guides = guides
            + centered_cube(
                format!("reagent_thaw_station_handoff_tubing_comb_{i}"),
                250.0,
                22.0,
                22.0,
            )
            .translate(
                HANDOFF_CENTER_X - HANDOFF_DOCK_X / 2.0 - 116.0,
                y,
                DECK_Z + 42.0,
            );
    }
    guides
}

fn robot_service_keepouts() -> Part {
    let front = keepout_frame(
        "reagent_thaw_station_front_robot_approach_keepout",
        DECK_X - 120.0,
        FRONT_ROBOT_APPROACH,
        260.0,
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + 130.0,
    );
    let rear = keepout_frame(
        "reagent_thaw_station_rear_connector_service_keepout",
        BULKHEAD_X + 120.0,
        REAR_SERVICE_CLEARANCE,
        BULKHEAD_Z + 120.0,
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        DECK_Z + (BULKHEAD_Z + 120.0) / 2.0,
    );
    let side = keepout_frame(
        "reagent_thaw_station_handoff_side_pull_keepout",
        HANDOFF_PULL_CLEARANCE,
        DECK_Y - 140.0,
        210.0,
        DECK_X / 2.0 + HANDOFF_PULL_CLEARANCE / 2.0,
        0.0,
        DECK_Z + 105.0,
    );
    let top = keepout_frame(
        "reagent_thaw_station_overhead_robot_z_keepout",
        DECK_X - 80.0,
        DECK_Y - 80.0,
        TOP_SERVICE_CLEARANCE,
        0.0,
        0.0,
        DECK_Z + 220.0 + TOP_SERVICE_CLEARANCE / 2.0,
    );

    front + rear + side + top
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64, cx: f64, cy: f64, cz: f64) -> Part {
    let t = 8.0;
    let front = centered_cube(format!("{name}_front_edge"), x, t, t).translate(
        cx,
        cy - y / 2.0,
        cz - z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear_edge"), x, t, t).translate(
        cx,
        cy + y / 2.0,
        cz - z / 2.0,
    );
    let left = centered_cube(format!("{name}_left_edge"), t, y, t).translate(
        cx - x / 2.0,
        cy,
        cz - z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_edge"), t, y, t).translate(
        cx + x / 2.0,
        cy,
        cz - z / 2.0,
    );
    let v1 = centered_cube(format!("{name}_vertical_0"), t, t, z).translate(
        cx - x / 2.0,
        cy - y / 2.0,
        cz,
    );
    let v2 = centered_cube(format!("{name}_vertical_1"), t, t, z).translate(
        cx + x / 2.0,
        cy - y / 2.0,
        cz,
    );
    let v3 = centered_cube(format!("{name}_vertical_2"), t, t, z).translate(
        cx - x / 2.0,
        cy + y / 2.0,
        cz,
    );
    let v4 = centered_cube(format!("{name}_vertical_3"), t, t, z).translate(
        cx + x / 2.0,
        cy + y / 2.0,
        cz,
    );
    let top_front = centered_cube(format!("{name}_top_front_edge"), x, t, t).translate(
        cx,
        cy - y / 2.0,
        cz + z / 2.0,
    );
    let top_rear = centered_cube(format!("{name}_top_rear_edge"), x, t, t).translate(
        cx,
        cy + y / 2.0,
        cz + z / 2.0,
    );
    let top_left = centered_cube(format!("{name}_top_left_edge"), t, y, t).translate(
        cx - x / 2.0,
        cy,
        cz + z / 2.0,
    );
    let top_right = centered_cube(format!("{name}_top_right_edge"), t, y, t).translate(
        cx + x / 2.0,
        cy,
        cz + z / 2.0,
    );

    front + rear + left + right + v1 + v2 + v3 + v4 + top_front + top_rear + top_left + top_right
}

fn datum_pin_positions() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-DECK_X / 2.0 + 70.0, -DECK_Y / 2.0 + 70.0),
        (DECK_X / 2.0 - 70.0, -DECK_Y / 2.0 + 70.0),
        (-DECK_X / 2.0 + 70.0, DECK_Y / 2.0 - 70.0),
        (DECK_X / 2.0 - 70.0, DECK_Y / 2.0 - 70.0),
        (
            RECEIVING_CENTER_X,
            RECEIVING_CENTER_Y + RECEIVING_NEST_Y / 2.0 - 28.0,
        ),
        (THAW_CENTER_X, THAW_CENTER_Y - THAW_BLOCK_Y / 2.0 + 28.0),
    ]
}

fn bag_bay_x(index: usize) -> f64 {
    lane_x(index, BAG_BAYS, BAG_BAY_X + 32.0)
}

fn vial_position(row: usize, col: usize) -> (f64, f64) {
    let x = -((VIAL_COLS as f64 - 1.0) * VIAL_PITCH_X) / 2.0 + col as f64 * VIAL_PITCH_X;
    let y = RECEIVING_NEST_Y / 2.0 - 58.0 - row as f64 * VIAL_PITCH_Y;
    (x, y)
}

fn thaw_chamber_x(index: usize) -> f64 {
    lane_x(index, INDEPENDENT_THAW_CHAMBERS, THAW_CHAMBER_X + 42.0)
}

fn connector_x(index: usize) -> f64 {
    lane_x(index, CONNECTOR_PORTS, CONNECTOR_PITCH_X)
}

fn handoff_y(index: usize) -> f64 {
    lane_x(index, HANDOFFS, HANDOFF_PITCH_Y)
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn receiving_right_edge() -> f64 {
    RECEIVING_CENTER_X + RECEIVING_NEST_X / 2.0
}

fn thaw_right_edge() -> f64 {
    THAW_CENTER_X + THAW_BLOCK_X / 2.0
}

fn connector_span_x() -> f64 {
    (CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X
}

fn sensor_positions() -> [(f64, f64, f64, bool); TEMP_SENSOR_POCKETS] {
    [
        (
            RECEIVING_CENTER_X - 118.0,
            RECEIVING_CENTER_Y - 204.0,
            DECK_Z + 82.0,
            true,
        ),
        (
            RECEIVING_CENTER_X + 118.0,
            RECEIVING_CENTER_Y - 204.0,
            DECK_Z + 82.0,
            true,
        ),
        (
            RECEIVING_CENTER_X - 84.0,
            RECEIVING_CENTER_Y + 132.0,
            DECK_Z + 72.0,
            false,
        ),
        (
            RECEIVING_CENTER_X + 84.0,
            RECEIVING_CENTER_Y + 132.0,
            DECK_Z + 72.0,
            false,
        ),
        (
            THAW_CENTER_X - 126.0,
            THAW_CENTER_Y + 152.0,
            DECK_Z + 112.0,
            true,
        ),
        (
            THAW_CENTER_X + 126.0,
            THAW_CENTER_Y + 152.0,
            DECK_Z + 112.0,
            true,
        ),
        (
            THAW_CENTER_X - 126.0,
            THAW_CENTER_Y - 214.0,
            DECK_Z + 90.0,
            true,
        ),
        (
            THAW_CENTER_X + 126.0,
            THAW_CENTER_Y - 214.0,
            DECK_Z + 90.0,
            true,
        ),
        (HANDOFF_CENTER_X - 132.0, handoff_y(0), DECK_Z + 72.0, false),
        (
            HANDOFF_CENTER_X - 132.0,
            handoff_y(HANDOFFS - 1),
            DECK_Z + 72.0,
            false,
        ),
    ]
}

fn barcode_land_positions() -> [(f64, f64); BARCODE_LANDS] {
    [
        (RECEIVING_CENTER_X - 148.0, RECEIVING_CENTER_Y - 190.0),
        (RECEIVING_CENTER_X + 148.0, RECEIVING_CENTER_Y - 190.0),
        (RECEIVING_CENTER_X - 112.0, RECEIVING_CENTER_Y + 144.0),
        (RECEIVING_CENTER_X + 112.0, RECEIVING_CENTER_Y + 144.0),
        (THAW_CENTER_X - 126.0, THAW_CENTER_Y - 214.0),
        (THAW_CENTER_X + 126.0, THAW_CENTER_Y - 214.0),
        (THAW_CENTER_X - 126.0, THAW_CENTER_Y + 190.0),
        (THAW_CENTER_X + 126.0, THAW_CENTER_Y + 190.0),
        (HANDOFF_CENTER_X, handoff_y(0) - 58.0),
        (HANDOFF_CENTER_X, handoff_y(HANDOFFS - 1) + 58.0),
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
        assert_eq!(OUTPUTS.len(), 9);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_reagent_thaw_equilibration_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cold_receiving_capacity_matches_closed_reagent_inputs() {
        assert_eq!(BAG_BAYS, 2);
        assert_eq!(VIAL_ROWS * VIAL_COLS, VIAL_POSITIONS);
        assert!(RECEIVING_NEST_X < DECK_X / 2.0);
        assert!(RECEIVING_NEST_Y < DECK_Y / 2.0);
        assert!(BAG_BAY_X * BAG_BAYS as f64 + 32.0 < RECEIVING_NEST_X);
        assert!(VIAL_WELL_D + 6.0 < VIAL_PITCH_X);
        assert!(VIAL_WELL_D + 6.0 < VIAL_PITCH_Y);
    }

    #[test]
    fn thaw_block_has_independent_chambers_and_equilibration_lanes() {
        assert_eq!(INDEPENDENT_THAW_CHAMBERS, 2);
        assert_eq!(EQUILIBRATION_LANES, 4);
        assert!(THAW_CHAMBER_X * INDEPENDENT_THAW_CHAMBERS as f64 + 42.0 < THAW_BLOCK_X);
        assert!(THAW_CHAMBER_Y + 62.0 < THAW_BLOCK_Y);
        assert!(THAW_CHAMBER_RECESS_Z < THAW_BLOCK_Z / 2.0);
    }

    #[test]
    fn sterile_connector_bulkhead_matches_closed_fluid_path_counts() {
        assert_eq!(CONNECTOR_PORTS, 8);
        assert_eq!(FILTER_COUNT, 2);
        assert!(connector_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
        assert!(CONNECTOR_PORT_D < CONNECTOR_COLLAR_D);
        assert!(BULKHEAD_Z > 190.0);
    }

    #[test]
    fn temperature_identity_and_handoff_features_are_explicit() {
        assert_eq!(TEMP_SENSOR_POCKETS, sensor_positions().len());
        assert_eq!(BARCODE_LANDS, barcode_land_positions().len());
        assert_eq!(HANDOFFS, 3);
        assert_eq!(HANDOFF_CONNECTORS_PER_DOCK, 3);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 250.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 220.0);
        assert!(TOP_SERVICE_CLEARANCE >= 340.0);
    }
}
