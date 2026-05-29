use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed effluent neutralization and disposal QC station for automated culture modules.
//
// Intent:
// - Receive spent media and cleaning effluent through closed, segregated docks.
// - Package neutralizer addition, contact-time evidence, pH/conductivity/volume
//   QC, venting, level/overflow sensing, and disposal handoff in one station.
// - Keep biohazard waste, released neutralized effluent, hold/retest material,
//   and clean consumables physically separated and traceable.
//
// This is architecture/fit CAD only. It is not a disposal permit, neutralization
// validation, chemical compatibility certification, or biosafety approval.

const OUTPUTS: &[&str] = &[
    "output/closed_effluent_neutralization_qc_station_base_secondary_containment.stl",
    "output/closed_effluent_neutralization_qc_station_closed_waste_bag_canister_dock.stl",
    "output/closed_effluent_neutralization_qc_station_neutralizer_additive_connector_bay.stl",
    "output/closed_effluent_neutralization_qc_station_contact_time_evidence_tokens.stl",
    "output/closed_effluent_neutralization_qc_station_ph_conductivity_sample_loop_panel.stl",
    "output/closed_effluent_neutralization_qc_station_vent_filter_level_overflow_sensors.stl",
    "output/closed_effluent_neutralization_qc_station_release_hold_reject_lanes.stl",
    "output/closed_effluent_neutralization_qc_station_barcode_run_record_lands.stl",
    "output/closed_effluent_neutralization_qc_station_drain_disposal_service_interface.stl",
    "output/closed_effluent_neutralization_qc_station_clean_used_consumable_segregation.stl",
    "output/closed_effluent_neutralization_qc_station_mixing_contact_time_coil.stl",
    "output/closed_effluent_neutralization_qc_station_robot_service_keepouts.stl",
    "output/closed_effluent_neutralization_qc_station_assembly.stl",
];

const STATION_X: f64 = 1440.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 30.0;
const RIM_W: f64 = 26.0;
const RIM_H: f64 = 54.0;
const SUMP_DEPTH: f64 = 12.0;
const FRONT_SERVICE_CLEARANCE: f64 = 520.0;
const REAR_SERVICE_CLEARANCE: f64 = 300.0;
const SIDE_CANISTER_PULL_CLEARANCE: f64 = 260.0;
const ROBOT_APPROACH_CLEARANCE_Z: f64 = 360.0;

const WASTE_DOCK_X: f64 = 500.0;
const WASTE_DOCK_Y: f64 = 328.0;
const WASTE_DOCK_Z: f64 = 92.0;
const WASTE_CANISTER_POSITIONS: usize = 2;
const WASTE_CANISTER_D: f64 = 164.0;
const WASTE_BAG_SADDLES: usize = 4;
const BAG_SADDLE_X: f64 = 92.0;
const BAG_SADDLE_Y: f64 = 132.0;
const BAG_SADDLE_Z: f64 = 26.0;

const ADDITIVE_BAY_X: f64 = 386.0;
const ADDITIVE_BAY_Y: f64 = 178.0;
const ADDITIVE_BAY_Z: f64 = 72.0;
const NEUTRALIZER_PORTS: usize = 6;
const ADDITIVE_BOTTLE_POSITIONS: usize = 3;
const ADDITIVE_PORT_PITCH_X: f64 = 56.0;

const CONTACT_TOKEN_BLOCK_X: f64 = 382.0;
const CONTACT_TOKEN_BLOCK_Y: f64 = 138.0;
const CONTACT_TOKEN_BLOCK_Z: f64 = 42.0;
const CONTACT_TOKEN_SLOTS: usize = 12;
const CONTACT_TIMER_PUCKS: usize = 4;

const QC_PANEL_X: f64 = 420.0;
const QC_PANEL_Y: f64 = 164.0;
const QC_PANEL_Z: f64 = 68.0;
const PH_SENSOR_PORTS: usize = 2;
const CONDUCTIVITY_SENSOR_PORTS: usize = 2;
const SAMPLE_LOOP_PORTS: usize = 8;
const VOLUME_METER_WINDOWS: usize = 2;

const VENT_SENSOR_PANEL_X: f64 = 382.0;
const VENT_SENSOR_PANEL_Y: f64 = 112.0;
const VENT_SENSOR_PANEL_Z: f64 = 124.0;
const VENT_FILTERS: usize = 4;
const LEVEL_SENSOR_POCKETS: usize = 6;
const OVERFLOW_SENSOR_POCKETS: usize = 4;

const STATUS_LANE_X: f64 = 470.0;
const STATUS_LANE_Y: f64 = 202.0;
const STATUS_LANE_Z: f64 = 46.0;
const RELEASE_LANES: usize = 3; // released, hold/retest, reject/biohazard.
const STATUS_SLOTS_PER_LANE: usize = 4;

const BARCODE_LANDS: usize = 16;
const RUN_RECORD_CARD_SLOTS: usize = 6;

const DRAIN_INTERFACE_X: f64 = 520.0;
const DRAIN_INTERFACE_Y: f64 = 86.0;
const DRAIN_INTERFACE_Z: f64 = 70.0;
const DISPOSAL_PORTS: usize = 5;
const SERVICE_DRAIN_D: f64 = 18.0;

const SEGREGATION_BARRIER_X: f64 = 26.0;
const SEGREGATION_BARRIER_Y: f64 = STATION_Y - 128.0;
const SEGREGATION_BARRIER_Z: f64 = 138.0;
const CLEAN_CONSUMABLE_POSITIONS: usize = 8;
const USED_CONSUMABLE_POSITIONS: usize = 8;

const MIXING_COIL_X: f64 = 458.0;
const MIXING_COIL_Y: f64 = 176.0;
const MIXING_COIL_Z: f64 = 58.0;
const MIXING_CHANNELS: usize = 6;
const CONTACT_DWELL_SEGMENTS: usize = 5;
const FLUID_BORE_D: f64 = 7.0;

const WASTE_DOCK_POS: (f64, f64) = (-392.0, 130.0);
const ADDITIVE_POS: (f64, f64) = (214.0, 220.0);
const CONTACT_POS: (f64, f64) = (448.0, 50.0);
const QC_POS: (f64, f64) = (120.0, -128.0);
const VENT_SENSOR_POS: (f64, f64) = (-392.0, -184.0);
const STATUS_POS: (f64, f64) = (448.0, -218.0);
const DRAIN_POS: (f64, f64) = (0.0, STATION_Y / 2.0 - 66.0);
const MIXING_POS: (f64, f64) = (-168.0, 24.0);

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_secondary_containment();
    export(&base, OUTPUTS[0]);

    let waste_dock = closed_waste_bag_canister_dock();
    export(&waste_dock, OUTPUTS[1]);

    let additive = neutralizer_additive_connector_bay();
    export(&additive, OUTPUTS[2]);

    let contact_tokens = contact_time_evidence_tokens();
    export(&contact_tokens, OUTPUTS[3]);

    let qc_panel = ph_conductivity_sample_loop_panel();
    export(&qc_panel, OUTPUTS[4]);

    let vent_sensors = vent_filter_level_overflow_sensors();
    export(&vent_sensors, OUTPUTS[5]);

    let status_lanes = release_hold_reject_lanes();
    export(&status_lanes, OUTPUTS[6]);

    let traceability = barcode_run_record_lands();
    export(&traceability, OUTPUTS[7]);

    let drain = drain_disposal_service_interface();
    export(&drain, OUTPUTS[8]);

    let segregation = clean_used_consumable_segregation();
    export(&segregation, OUTPUTS[9]);

    let mixing = mixing_contact_time_coil();
    export(&mixing, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly =
        base + waste_dock.translate(
            WASTE_DOCK_POS.0,
            WASTE_DOCK_POS.1,
            BASE_Z + WASTE_DOCK_Z / 2.0,
        ) + additive.translate(
            ADDITIVE_POS.0,
            ADDITIVE_POS.1,
            BASE_Z + ADDITIVE_BAY_Z / 2.0,
        ) + contact_tokens.translate(
            CONTACT_POS.0,
            CONTACT_POS.1,
            BASE_Z + CONTACT_TOKEN_BLOCK_Z / 2.0,
        ) + qc_panel.translate(QC_POS.0, QC_POS.1, BASE_Z + QC_PANEL_Z / 2.0)
            + vent_sensors.translate(
                VENT_SENSOR_POS.0,
                VENT_SENSOR_POS.1,
                BASE_Z + VENT_SENSOR_PANEL_Z / 2.0,
            )
            + status_lanes.translate(STATUS_POS.0, STATUS_POS.1, BASE_Z + STATUS_LANE_Z / 2.0)
            + traceability.translate(0.0, 0.0, BASE_Z + 4.0)
            + drain.translate(DRAIN_POS.0, DRAIN_POS.1, BASE_Z + DRAIN_INTERFACE_Z / 2.0)
            + segregation.translate(0.0, -16.0, BASE_Z + SEGREGATION_BARRIER_Z / 2.0)
            + mixing.translate(MIXING_POS.0, MIXING_POS.1, BASE_Z + MIXING_COIL_Z / 2.0)
            + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed effluent neutralization/QC station: {:.0}mm x {:.0}mm deck, {} canister docks, {} bag saddles, {} neutralizer ports, {} contact-token slots, {} pH/conductivity ports, {} sample-loop ports, {} vent filters, {} level/overflow sensor pockets, and {} disposal service ports.",
        STATION_X,
        STATION_Y,
        WASTE_CANISTER_POSITIONS,
        WASTE_BAG_SADDLES,
        NEUTRALIZER_PORTS,
        CONTACT_TOKEN_SLOTS,
        PH_SENSOR_PORTS + CONDUCTIVITY_SENSOR_PORTS,
        SAMPLE_LOOP_PORTS,
        VENT_FILTERS,
        LEVEL_SENSOR_POCKETS + OVERFLOW_SENSOR_POCKETS,
        DISPOSAL_PORTS
    );
    println!(
        "Segregation and service: {} clean consumable pockets, {} used/biohazard pockets, {} release/hold/reject lanes, {:.0}mm front service, {:.0}mm rear drain service, {:.0}mm side canister pull, and {:.0}mm robot Z keepout.",
        CLEAN_CONSUMABLE_POSITIONS,
        USED_CONSUMABLE_POSITIONS,
        RELEASE_LANES,
        FRONT_SERVICE_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        SIDE_CANISTER_PULL_CLEARANCE,
        ROBOT_APPROACH_CLEARANCE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_secondary_containment() -> Part {
    let pan = centered_cube(
        "effluent_neutralization_base_secondary_containment_pan",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "effluent_neutralization_recessed_leak_sump",
        STATION_X - 128.0,
        STATION_Y - 122.0,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 8.0, BASE_Z - SUMP_DEPTH / 2.0);
    let sloped_gutter = centered_cube(
        "effluent_neutralization_front_sump_gutter",
        STATION_X - 220.0,
        18.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 78.0, BASE_Z - 4.0);
    let drain_bore = centered_cylinder(
        "effluent_neutralization_secondary_containment_drain_bore",
        SERVICE_DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -STATION_Y / 2.0 + 24.0,
        BASE_Z - 7.0,
    );

    pan - sump - sloped_gutter - drain_bore
        + tray_perimeter_rim()
        + base_locator_bosses()
        + module_mount_slots()
        + spill_flow_ribs()
        + front_datum_bar()
}

fn tray_perimeter_rim() -> Part {
    let left = centered_cube(
        "effluent_neutralization_left_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_H,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, RIM_H / 2.0);
    let right = centered_cube(
        "effluent_neutralization_right_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_H,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, RIM_H / 2.0);
    let rear = centered_cube(
        "effluent_neutralization_rear_raised_lip",
        STATION_X,
        RIM_W,
        RIM_H,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, RIM_H / 2.0);
    let front = centered_cube(
        "effluent_neutralization_front_low_service_lip",
        STATION_X - 170.0,
        RIM_W,
        RIM_H - 16.0,
    )
    .translate(
        -34.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        (RIM_H - 16.0) / 2.0,
    );
    left + right + rear + front
}

fn base_locator_bosses() -> Part {
    let mut bosses = Part::empty("effluent_neutralization_base_locator_bosses");
    for (i, (x, y)) in [
        (-610.0, -330.0),
        (610.0, -330.0),
        (-610.0, 330.0),
        (610.0, 330.0),
        (-160.0, -330.0),
        (160.0, 330.0),
    ]
    .iter()
    .enumerate()
    {
        bosses = bosses
            + centered_cylinder(
                format!("effluent_neutralization_dowel_boss_{i}"),
                19.0,
                12.0,
                36,
            )
            .translate(*x, *y, BASE_Z + 6.0)
            - centered_cylinder(
                format!("effluent_neutralization_dowel_socket_{i}"),
                6.2,
                14.0,
                24,
            )
            .translate(*x, *y, BASE_Z + 6.0);
    }
    bosses
}

fn module_mount_slots() -> Part {
    let mut slots = Part::empty("effluent_neutralization_mount_slots");
    for (i, (x, y)) in mount_slot_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("effluent_neutralization_m6_clearance_hole_{i}"),
            6.8 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("effluent_neutralization_m6_slot_relief_{i}"),
            28.0,
            7.0,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
}

fn spill_flow_ribs() -> Part {
    let mut ribs = Part::empty("effluent_neutralization_spill_flow_ribs");
    for (i, y) in [-250.0, -126.0, 0.0, 126.0, 250.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("effluent_neutralization_sump_flow_rib_{i}"),
                STATION_X - 260.0,
                9.0,
                8.0,
            )
            .translate(0.0, *y, BASE_Z + 4.0);
    }
    ribs
}

fn front_datum_bar() -> Part {
    centered_cube(
        "effluent_neutralization_workcell_front_datum_bar",
        STATION_X - 260.0,
        20.0,
        34.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 48.0), BASE_Z + 17.0)
}

fn closed_waste_bag_canister_dock() -> Part {
    let deck = centered_cube(
        "effluent_waste_dock_recessed_deck",
        WASTE_DOCK_X,
        WASTE_DOCK_Y,
        WASTE_DOCK_Z,
    );
    let cavity = centered_cube(
        "effluent_waste_dock_drip_recess",
        WASTE_DOCK_X - 58.0,
        WASTE_DOCK_Y - 58.0,
        WASTE_DOCK_Z - 20.0,
    )
    .translate(0.0, 0.0, 12.0);

    let mut canisters = Part::empty("effluent_waste_canister_receivers");
    for i in 0..WASTE_CANISTER_POSITIONS {
        let x = centered_index(i, WASTE_CANISTER_POSITIONS, 196.0);
        canisters = canisters
            + centered_cylinder(
                format!("effluent_closed_canister_outer_receiver_{i}"),
                WASTE_CANISTER_D / 2.0 + 13.0,
                28.0,
                64,
            )
            .translate(x, 50.0, WASTE_DOCK_Z / 2.0 + 12.0)
            - centered_cylinder(
                format!("effluent_closed_canister_socket_clearance_{i}"),
                WASTE_CANISTER_D / 2.0,
                34.0,
                64,
            )
            .translate(x, 50.0, WASTE_DOCK_Z / 2.0 + 14.0)
            + centered_cube(
                format!("effluent_canister_anti_rotation_key_{i}"),
                34.0,
                10.0,
                22.0,
            )
            .translate(
                x,
                50.0 + WASTE_CANISTER_D / 2.0 + 9.0,
                WASTE_DOCK_Z / 2.0 + 18.0,
            );
    }

    let mut saddles = Part::empty("effluent_collapsible_bag_saddles");
    for i in 0..WASTE_BAG_SADDLES {
        let x = centered_index(i, WASTE_BAG_SADDLES, 102.0);
        let cradle = centered_cube(
            format!("effluent_waste_bag_saddle_block_{i}"),
            BAG_SADDLE_X,
            BAG_SADDLE_Y,
            BAG_SADDLE_Z,
        )
        .translate(x, -104.0, WASTE_DOCK_Z / 2.0 + BAG_SADDLE_Z / 2.0);
        let bag_arc = centered_cylinder(
            format!("effluent_waste_bag_saddle_radius_cut_{i}"),
            32.0,
            BAG_SADDLE_X + 4.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -104.0, WASTE_DOCK_Z / 2.0 + BAG_SADDLE_Z + 2.0);
        saddles = saddles + cradle - bag_arc;
    }

    let inlet_bulkhead = closed_effluent_inlet_bulkhead();
    deck - cavity + canisters + saddles + inlet_bulkhead + dock_latch_sensors()
}

fn closed_effluent_inlet_bulkhead() -> Part {
    let panel = centered_cube(
        "effluent_closed_inlet_bulkhead_panel",
        WASTE_DOCK_X - 84.0,
        32.0,
        82.0,
    )
    .translate(0.0, WASTE_DOCK_Y / 2.0 - 34.0, 36.0);
    let mut ports = Part::empty("effluent_closed_inlet_bulkhead_ports");
    for i in 0..8 {
        let x = centered_index(i, 8, 46.0);
        ports = ports
            + centered_cylinder(
                format!("effluent_closed_inlet_port_boss_{i}"),
                13.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, WASTE_DOCK_Y / 2.0 - 51.0, 42.0)
            - centered_cylinder(
                format!("effluent_closed_inlet_port_bore_{i}"),
                FLUID_BORE_D / 2.0,
                24.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, WASTE_DOCK_Y / 2.0 - 51.0, 42.0);
    }
    panel + ports
}

fn dock_latch_sensors() -> Part {
    let mut sensors = Part::empty("effluent_dock_latch_presence_sensors");
    for (i, (x, y)) in [
        (-204.0, -154.0),
        (204.0, -154.0),
        (-204.0, 154.0),
        (204.0, 154.0),
    ]
    .iter()
    .enumerate()
    {
        sensors = sensors
            + centered_cube(
                format!("effluent_waste_dock_latch_sensor_pocket_{i}"),
                34.0,
                22.0,
                10.0,
            )
            .translate(*x, *y, WASTE_DOCK_Z / 2.0 + 8.0);
    }
    sensors
}

fn neutralizer_additive_connector_bay() -> Part {
    let bay = centered_cube(
        "effluent_neutralizer_additive_bay_body",
        ADDITIVE_BAY_X,
        ADDITIVE_BAY_Y,
        ADDITIVE_BAY_Z,
    );
    let drip_cavity = centered_cube(
        "effluent_neutralizer_additive_bay_drip_cavity",
        ADDITIVE_BAY_X - 50.0,
        ADDITIVE_BAY_Y - 46.0,
        ADDITIVE_BAY_Z - 18.0,
    )
    .translate(0.0, 0.0, 12.0);

    let mut ports = Part::empty("effluent_neutralizer_additive_ports");
    for i in 0..NEUTRALIZER_PORTS {
        let x = centered_index(i, NEUTRALIZER_PORTS, ADDITIVE_PORT_PITCH_X);
        ports = ports
            + centered_cylinder(
                format!("effluent_neutralizer_additive_connector_boss_{i}"),
                12.0,
                26.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -ADDITIVE_BAY_Y / 2.0 + 20.0, 12.0)
            - centered_cylinder(
                format!("effluent_neutralizer_additive_connector_bore_{i}"),
                4.2,
                32.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -ADDITIVE_BAY_Y / 2.0 + 20.0, 12.0);
    }

    let mut bottle_nests = Part::empty("effluent_neutralizer_additive_bottle_nests");
    for i in 0..ADDITIVE_BOTTLE_POSITIONS {
        let x = centered_index(i, ADDITIVE_BOTTLE_POSITIONS, 96.0);
        bottle_nests = bottle_nests
            + centered_cylinder(
                format!("effluent_neutralizer_bottle_retainer_{i}"),
                34.0,
                18.0,
                40,
            )
            .translate(x, 48.0, ADDITIVE_BAY_Z / 2.0 + 8.0)
            - centered_cylinder(
                format!("effluent_neutralizer_bottle_socket_{i}"),
                24.0,
                20.0,
                40,
            )
            .translate(x, 48.0, ADDITIVE_BAY_Z / 2.0 + 10.0);
    }

    bay - drip_cavity + ports + bottle_nests + additive_check_valve_lands()
}

fn additive_check_valve_lands() -> Part {
    let mut lands = Part::empty("effluent_additive_check_valve_lands");
    for i in 0..NEUTRALIZER_PORTS {
        lands = lands
            + centered_cube(
                format!("effluent_additive_check_valve_land_{i}"),
                34.0,
                18.0,
                12.0,
            )
            .translate(
                centered_index(i, NEUTRALIZER_PORTS, ADDITIVE_PORT_PITCH_X),
                -18.0,
                42.0,
            );
    }
    lands
}

fn contact_time_evidence_tokens() -> Part {
    let block = centered_cube(
        "effluent_contact_time_evidence_block",
        CONTACT_TOKEN_BLOCK_X,
        CONTACT_TOKEN_BLOCK_Y,
        CONTACT_TOKEN_BLOCK_Z,
    );
    let mut token_slots = Part::empty("effluent_contact_time_token_slots");
    for i in 0..CONTACT_TOKEN_SLOTS {
        let col = i % 6;
        let row = i / 6;
        let x = centered_index(col, 6, 54.0);
        let y = centered_index(row, 2, 52.0);
        token_slots = token_slots
            + centered_cube(
                format!("effluent_contact_time_token_socket_{i}"),
                36.0,
                30.0,
                12.0,
            )
            .translate(x, y, CONTACT_TOKEN_BLOCK_Z / 2.0 + 6.0)
            - centered_cylinder(
                format!("effluent_contact_time_token_key_bore_{i}"),
                4.5,
                14.0,
                18,
            )
            .translate(x + 10.0, y, CONTACT_TOKEN_BLOCK_Z / 2.0 + 6.0);
    }

    let mut timer_pucks = Part::empty("effluent_contact_timer_puck_lands");
    for i in 0..CONTACT_TIMER_PUCKS {
        let x = centered_index(i, CONTACT_TIMER_PUCKS, 76.0);
        timer_pucks = timer_pucks
            + centered_cylinder(
                format!("effluent_contact_timer_puck_land_{i}"),
                21.0,
                9.0,
                36,
            )
            .translate(
                x,
                CONTACT_TOKEN_BLOCK_Y / 2.0 + 28.0,
                CONTACT_TOKEN_BLOCK_Z / 2.0 + 4.0,
            );
    }
    block + token_slots + timer_pucks
}

fn ph_conductivity_sample_loop_panel() -> Part {
    let panel = centered_cube(
        "effluent_qc_sensor_panel",
        QC_PANEL_X,
        QC_PANEL_Y,
        QC_PANEL_Z,
    );
    let wet_channel = centered_cube(
        "effluent_qc_sensor_panel_wet_channel_recess",
        QC_PANEL_X - 60.0,
        24.0,
        18.0,
    )
    .translate(0.0, -16.0, QC_PANEL_Z / 2.0);

    let mut ph_ports = Part::empty("effluent_qc_ph_sensor_ports");
    for i in 0..PH_SENSOR_PORTS {
        let x = -160.0 + i as f64 * 58.0;
        ph_ports =
            ph_ports
                + centered_cylinder(format!("effluent_qc_ph_probe_pocket_{i}"), 14.0, 22.0, 32)
                    .translate(x, 46.0, QC_PANEL_Z / 2.0 + 8.0)
                - centered_cylinder(format!("effluent_qc_ph_flow_bore_{i}"), 4.0, 24.0, 20)
                    .translate(x, 46.0, QC_PANEL_Z / 2.0 + 8.0);
    }

    let mut conductivity_ports = Part::empty("effluent_qc_conductivity_ports");
    for i in 0..CONDUCTIVITY_SENSOR_PORTS {
        let x = -22.0 + i as f64 * 58.0;
        conductivity_ports = conductivity_ports
            + centered_cylinder(
                format!("effluent_qc_conductivity_probe_pocket_{i}"),
                12.0,
                20.0,
                32,
            )
            .translate(x, 46.0, QC_PANEL_Z / 2.0 + 7.0)
            - centered_cylinder(
                format!("effluent_qc_conductivity_flow_bore_{i}"),
                3.6,
                22.0,
                20,
            )
            .translate(x, 46.0, QC_PANEL_Z / 2.0 + 7.0);
    }

    let mut sample_ports = Part::empty("effluent_qc_sample_loop_ports");
    for i in 0..SAMPLE_LOOP_PORTS {
        let x = centered_index(i, SAMPLE_LOOP_PORTS, 42.0);
        sample_ports = sample_ports
            + centered_cylinder(
                format!("effluent_qc_closed_sample_loop_port_boss_{i}"),
                9.0,
                18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -QC_PANEL_Y / 2.0 + 18.0, 12.0)
            - centered_cylinder(
                format!("effluent_qc_closed_sample_loop_port_bore_{i}"),
                3.4,
                22.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -QC_PANEL_Y / 2.0 + 18.0, 12.0);
    }

    panel - wet_channel + ph_ports + conductivity_ports + sample_ports + volume_meter_windows()
}

fn volume_meter_windows() -> Part {
    let mut windows = Part::empty("effluent_qc_volume_meter_windows");
    for i in 0..VOLUME_METER_WINDOWS {
        windows = windows
            + centered_cube(
                format!("effluent_qc_optical_volume_meter_window_{i}"),
                78.0,
                16.0,
                28.0,
            )
            .translate(122.0 + i as f64 * 82.0, 26.0, QC_PANEL_Z / 2.0 + 4.0);
    }
    windows
}

fn vent_filter_level_overflow_sensors() -> Part {
    let panel = centered_cube(
        "effluent_vent_filter_level_sensor_panel",
        VENT_SENSOR_PANEL_X,
        VENT_SENSOR_PANEL_Y,
        VENT_SENSOR_PANEL_Z,
    );
    let mut filters = Part::empty("effluent_vent_filter_placeholders");
    for i in 0..VENT_FILTERS {
        let x = centered_index(i, VENT_FILTERS, 76.0);
        filters = filters
            + centered_cylinder(format!("effluent_vent_hepa_filter_can_{i}"), 24.0, 82.0, 40)
                .translate(x, 0.0, VENT_SENSOR_PANEL_Z / 2.0 + 28.0)
            - centered_cylinder(format!("effluent_vent_filter_core_bore_{i}"), 9.0, 86.0, 28)
                .translate(x, 0.0, VENT_SENSOR_PANEL_Z / 2.0 + 28.0);
    }

    let mut level_sensors = Part::empty("effluent_level_sensor_pockets");
    for i in 0..LEVEL_SENSOR_POCKETS {
        level_sensors = level_sensors
            + centered_cube(
                format!("effluent_level_sensor_vertical_pocket_{i}"),
                20.0,
                18.0,
                42.0,
            )
            .translate(
                -154.0 + i as f64 * 62.0,
                -VENT_SENSOR_PANEL_Y / 2.0 - 12.0,
                4.0 + i as f64 * 9.0,
            );
    }

    let mut overflow = Part::empty("effluent_overflow_sensor_pockets");
    for i in 0..OVERFLOW_SENSOR_POCKETS {
        overflow = overflow
            + centered_cube(
                format!("effluent_high_high_overflow_sensor_pocket_{i}"),
                36.0,
                16.0,
                20.0,
            )
            .translate(
                centered_index(i, OVERFLOW_SENSOR_POCKETS, 78.0),
                VENT_SENSOR_PANEL_Y / 2.0 + 12.0,
                VENT_SENSOR_PANEL_Z / 2.0 + 34.0,
            );
    }
    panel + filters + level_sensors + overflow
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "effluent_release_hold_reject_status_lane_base",
        STATUS_LANE_X,
        STATUS_LANE_Y,
        STATUS_LANE_Z,
    );
    let mut lanes = Part::empty("effluent_release_hold_reject_status_lanes");
    for lane in 0..RELEASE_LANES {
        let y = centered_index(lane, RELEASE_LANES, 58.0);
        let rail = centered_cube(
            format!("effluent_status_lane_side_rail_{lane}"),
            STATUS_LANE_X - 44.0,
            8.0,
            18.0,
        )
        .translate(0.0, y, STATUS_LANE_Z / 2.0 + 8.0);
        lanes = lanes + rail;
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 94.0);
            lanes = lanes
                + centered_cube(
                    format!("effluent_status_lane_{lane}_evidence_tote_slot_{slot}"),
                    70.0,
                    38.0,
                    12.0,
                )
                .translate(x, y + 22.0, STATUS_LANE_Z / 2.0 + 6.0);
        }
    }
    base + lanes
}

fn barcode_run_record_lands() -> Part {
    let mut lands = Part::empty("effluent_barcode_run_record_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 8;
        let col = i % 8;
        lands = lands
            + centered_cube(format!("effluent_barcode_rfid_land_{i}"), 54.0, 28.0, 6.0).translate(
                centered_index(col, 8, 72.0),
                -382.0 + row as f64 * 42.0,
                0.0,
            );
    }
    for i in 0..RUN_RECORD_CARD_SLOTS {
        lands = lands
            + centered_cube(
                format!("effluent_run_record_card_slot_{i}"),
                72.0,
                8.0,
                42.0,
            )
            .translate(-618.0 + i as f64 * 74.0, -338.0, 20.0);
    }
    lands
}

fn drain_disposal_service_interface() -> Part {
    let block = centered_cube(
        "effluent_disposal_service_interface_block",
        DRAIN_INTERFACE_X,
        DRAIN_INTERFACE_Y,
        DRAIN_INTERFACE_Z,
    );
    let mut ports = Part::empty("effluent_disposal_service_ports");
    for i in 0..DISPOSAL_PORTS {
        let x = centered_index(i, DISPOSAL_PORTS, 82.0);
        let d = if i == DISPOSAL_PORTS - 1 {
            SERVICE_DRAIN_D
        } else {
            11.0
        };
        ports = ports
            + centered_cylinder(
                format!("effluent_disposal_service_port_boss_{i}"),
                d / 2.0 + 7.0,
                28.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, DRAIN_INTERFACE_Y / 2.0 - 14.0, 6.0)
            - centered_cylinder(
                format!("effluent_disposal_service_port_bore_{i}"),
                d / 2.0,
                34.0,
                26,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, DRAIN_INTERFACE_Y / 2.0 - 14.0, 6.0);
    }
    block + ports + disposal_release_valve_lands()
}

fn disposal_release_valve_lands() -> Part {
    let mut valves = Part::empty("effluent_disposal_release_valve_lands");
    for i in 0..DISPOSAL_PORTS {
        valves = valves
            + centered_cube(
                format!("effluent_disposal_release_valve_land_{i}"),
                54.0,
                24.0,
                18.0,
            )
            .translate(centered_index(i, DISPOSAL_PORTS, 82.0), -18.0, 38.0);
    }
    valves
}

fn clean_used_consumable_segregation() -> Part {
    let barrier = centered_cube(
        "effluent_clean_used_consumable_center_barrier",
        SEGREGATION_BARRIER_X,
        SEGREGATION_BARRIER_Y,
        SEGREGATION_BARRIER_Z,
    );
    let clean_header = centered_cube("effluent_clean_consumable_lane_header", 360.0, 24.0, 42.0)
        .translate(-270.0, -SEGREGATION_BARRIER_Y / 2.0 + 54.0, 24.0);
    let used_header = centered_cube(
        "effluent_used_biohazard_consumable_lane_header",
        360.0,
        24.0,
        42.0,
    )
    .translate(270.0, -SEGREGATION_BARRIER_Y / 2.0 + 54.0, 24.0);

    let mut pockets = Part::empty("effluent_clean_used_consumable_pockets");
    for i in 0..CLEAN_CONSUMABLE_POSITIONS {
        let x = -432.0 + (i % 4) as f64 * 76.0;
        let y = -240.0 + (i / 4) as f64 * 68.0;
        pockets = pockets
            + centered_cube(
                format!("effluent_clean_consumable_cap_or_probe_pocket_{i}"),
                50.0,
                42.0,
                18.0,
            )
            .translate(x, y, 18.0);
    }
    for i in 0..USED_CONSUMABLE_POSITIONS {
        let x = 204.0 + (i % 4) as f64 * 76.0;
        let y = -240.0 + (i / 4) as f64 * 68.0;
        pockets = pockets
            + centered_cube(
                format!("effluent_used_biohazard_consumable_pocket_{i}"),
                50.0,
                42.0,
                18.0,
            )
            .translate(x, y, 18.0);
    }
    barrier + clean_header + used_header + pockets
}

fn mixing_contact_time_coil() -> Part {
    let base = centered_cube(
        "effluent_mixing_contact_time_coil_base",
        MIXING_COIL_X,
        MIXING_COIL_Y,
        MIXING_COIL_Z,
    );
    let mut channels = Part::empty("effluent_static_mixing_channel_placeholders");
    for i in 0..MIXING_CHANNELS {
        let y = centered_index(i, MIXING_CHANNELS, 26.0);
        channels = channels
            + centered_cylinder(
                format!("effluent_static_mixer_channel_{i}"),
                FLUID_BORE_D / 2.0,
                MIXING_COIL_X - 62.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 4.0);
    }
    let mut dwell_blocks = Part::empty("effluent_contact_dwell_segment_markers");
    for i in 0..CONTACT_DWELL_SEGMENTS {
        dwell_blocks = dwell_blocks
            + centered_cube(
                format!("effluent_contact_dwell_segment_marker_{i}"),
                42.0,
                MIXING_COIL_Y - 28.0,
                14.0,
            )
            .translate(centered_index(i, CONTACT_DWELL_SEGMENTS, 82.0), 0.0, 35.0);
    }
    base - channels + dwell_blocks + mixing_inlet_outlet_ports()
}

fn mixing_inlet_outlet_ports() -> Part {
    let inlet = centered_cylinder(
        "effluent_mixing_coil_inlet_closed_connector",
        13.0,
        30.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -(MIXING_COIL_X / 2.0 + 10.0),
        -MIXING_COIL_Y / 2.0 + 34.0,
        4.0,
    );
    let outlet = centered_cylinder(
        "effluent_mixing_coil_outlet_closed_connector",
        13.0,
        30.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(MIXING_COIL_X / 2.0 + 10.0, MIXING_COIL_Y / 2.0 - 34.0, 4.0);
    inlet + outlet
}

fn robot_service_keepouts() -> Part {
    let front_pull = centered_cube(
        "effluent_front_service_pullout_keepout",
        STATION_X - 140.0,
        FRONT_SERVICE_CLEARANCE,
        18.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        40.0,
    );
    let rear_drain = centered_cube(
        "effluent_rear_disposal_hose_service_keepout",
        DRAIN_INTERFACE_X + 120.0,
        REAR_SERVICE_CLEARANCE,
        20.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 72.0);
    let side_pull = centered_cube(
        "effluent_left_canister_pull_service_keepout",
        SIDE_CANISTER_PULL_CLEARANCE,
        WASTE_DOCK_Y + 160.0,
        20.0,
    )
    .translate(
        -(STATION_X / 2.0 + SIDE_CANISTER_PULL_CLEARANCE / 2.0),
        WASTE_DOCK_POS.1,
        72.0,
    );
    let robot_z = centered_cube(
        "effluent_robot_vertical_approach_keepout",
        STATION_X - 220.0,
        STATION_Y - 180.0,
        ROBOT_APPROACH_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -18.0,
        BASE_Z + ROBOT_APPROACH_CLEARANCE_Z / 2.0 + 118.0,
    );
    front_pull + rear_drain + side_pull + robot_z + robot_pick_datums()
}

fn robot_pick_datums() -> Part {
    let mut datums = Part::empty("effluent_robot_pick_and_vision_datums");
    for (i, (x, y)) in [
        (-610.0, -342.0),
        (610.0, -342.0),
        (-610.0, 342.0),
        (610.0, 342.0),
        (0.0, -342.0),
        (0.0, 342.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("effluent_robot_vision_datum_target_{i}"),
                18.0,
                6.0,
                36,
            )
            .translate(*x, *y, BASE_Z + 9.0)
            - centered_cylinder(
                format!("effluent_robot_vision_datum_cross_bore_{i}"),
                3.0,
                8.0,
                18,
            )
            .translate(*x, *y, BASE_Z + 9.0);
    }
    datums
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn mount_slot_points() -> [(f64, f64); 10] {
    [
        (-648.0, -358.0),
        (648.0, -358.0),
        (-648.0, 358.0),
        (648.0, 358.0),
        (-360.0, -358.0),
        (360.0, -358.0),
        (-360.0, 358.0),
        (360.0, 358.0),
        (0.0, -358.0),
        (0.0, 358.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_effluent_neutralization_qc_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_has_required_effluent_workflow_features() {
        assert_eq!(WASTE_CANISTER_POSITIONS, 2);
        assert_eq!(WASTE_BAG_SADDLES, 4);
        assert_eq!(NEUTRALIZER_PORTS, 6);
        assert_eq!(CONTACT_TOKEN_SLOTS, 12);
        assert_eq!(SAMPLE_LOOP_PORTS, 8);
        assert_eq!(DISPOSAL_PORTS, 5);
    }

    #[test]
    fn major_modules_fit_inside_station_deck() {
        assert!(WASTE_DOCK_POS.0 - WASTE_DOCK_X / 2.0 > -STATION_X / 2.0 + RIM_W);
        assert!(ADDITIVE_POS.0 + ADDITIVE_BAY_X / 2.0 < STATION_X / 2.0 - RIM_W);
        assert!(CONTACT_POS.0 + CONTACT_TOKEN_BLOCK_X / 2.0 < STATION_X / 2.0 - RIM_W);
        assert!(QC_POS.1 - QC_PANEL_Y / 2.0 > -STATION_Y / 2.0 + RIM_W);
        assert!(DRAIN_POS.1 + DRAIN_INTERFACE_Y / 2.0 < STATION_Y / 2.0 + 1.0);
        assert!(VENT_SENSOR_POS.0 - VENT_SENSOR_PANEL_X / 2.0 > -STATION_X / 2.0 + RIM_W);
    }

    #[test]
    fn clean_used_segregation_is_balanced_and_visible() {
        assert_eq!(CLEAN_CONSUMABLE_POSITIONS, USED_CONSUMABLE_POSITIONS);
        assert_eq!(CLEAN_CONSUMABLE_POSITIONS + USED_CONSUMABLE_POSITIONS, 16);
        assert!(SEGREGATION_BARRIER_Z > STATUS_LANE_Z);
        assert!(SEGREGATION_BARRIER_Y < STATION_Y);
    }

    #[test]
    fn qc_sensor_and_release_evidence_counts_are_sufficient() {
        assert_eq!(PH_SENSOR_PORTS + CONDUCTIVITY_SENSOR_PORTS, 4);
        assert_eq!(VOLUME_METER_WINDOWS, 2);
        assert_eq!(RELEASE_LANES * STATUS_SLOTS_PER_LANE, 12);
        assert_eq!(LEVEL_SENSOR_POCKETS + OVERFLOW_SENSOR_POCKETS, 10);
        assert!(CONTACT_TIMER_PUCKS >= 4);
    }

    #[test]
    fn service_keepouts_match_closed_waste_handling() {
        assert!(FRONT_SERVICE_CLEARANCE >= 500.0);
        assert!(REAR_SERVICE_CLEARANCE >= 300.0);
        assert!(SIDE_CANISTER_PULL_CLEARANCE >= WASTE_CANISTER_D + 80.0);
        assert!(ROBOT_APPROACH_CLEARANCE_Z >= 320.0);
    }

    #[test]
    fn mixing_and_disposal_paths_have_plausible_geometry() {
        assert!(FLUID_BORE_D > 4.8);
        assert_eq!(MIXING_CHANNELS, 6);
        assert_eq!(CONTACT_DWELL_SEGMENTS, 5);
        assert!(SERVICE_DRAIN_D > FLUID_BORE_D * 2.0);
        assert!(centered_index(0, DISPOSAL_PORTS, 82.0).abs() < DRAIN_INTERFACE_X / 2.0 - 48.0);
    }
}
