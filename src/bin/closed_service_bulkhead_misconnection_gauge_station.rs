use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed service bulkhead misconnection-gauge validation station.
//
// Intent:
// - Provide a keyed bulkhead mockup for pressure, gas, and fluid service ports
//   before a real closed culture module can be connected.
// - Keep connector gauges color/shape coded with no-go blockers so the station
//   fails the wrong family mechanically instead of relying on operator memory.
// - Segregate pressure, gas, and fluid service zones, with RFID/QR evidence
//   pockets and fixed inspection fixtures that are repeatable across operators.
//
// This is a mechanical validation and training fixture. It models datums,
// gauges, blockers, labels, and keepouts, not pressure-rated hardware.

const OUTPUTS: [&str; 14] = [
    "output/closed_service_bulkhead_misconnection_gauge_station_deck.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_keyed_bulkhead_mockup.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_service_family_segregation_plate.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_pressure_round_red_gauges.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_gas_hex_amber_gauges.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_fluid_square_blue_gauges.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_no_go_blockers.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_keyed_master_plugs.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_rfid_qr_label_pocket.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_operator_independent_fixture_bridge.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_witness_reference_rack.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_misconnection_reject_tray.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_robot_service_keepouts.stl",
    "output/closed_service_bulkhead_misconnection_gauge_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "keyed_bulkhead_mockup",
    "pressure_port_zone",
    "gas_port_zone",
    "fluid_port_zone",
    "color_shape_coded_gauges",
    "no_go_blockers",
    "pressure_gas_fluid_segregation",
    "rfid_label_pocket",
    "qr_label_pocket",
    "operator_independent_fixture",
    "witness_reference_rack",
    "assembly_export",
];

const FAMILY_COUNT: usize = 3;
const PORTS_PER_FAMILY: usize = 3;
const GAUGES_PER_FAMILY: usize = 4;
const MASTER_PLUGS_PER_FAMILY: usize = 2;
const BLOCKER_PAIR_COUNT: usize = FAMILY_COUNT * (FAMILY_COUNT - 1);

const DECK_X: f64 = 980.0;
const DECK_Y: f64 = 720.0;
const DECK_Z: f64 = 18.0;
const DECK_CORNER_R: f64 = 18.0;

const BULKHEAD_PANEL_X: f64 = 760.0;
const BULKHEAD_PANEL_Y: f64 = 34.0;
const BULKHEAD_PANEL_Z: f64 = 250.0;
const BULKHEAD_CENTER_Y: f64 = 228.0;
const BULKHEAD_BASE_Z: f64 = DECK_Z / 2.0;
const FAMILY_PITCH_X: f64 = 238.0;
const PORT_PITCH_Z: f64 = 58.0;
const PORT_KEY_SLOT_W: f64 = 10.0;
const PORT_KEY_SLOT_H: f64 = 18.0;
#[cfg(test)]
const FAMILY_ZONE_MIN_GAP: f64 = 74.0;
const FAMILY_BARRIER_W: f64 = 12.0;

const SEG_PLATE_X: f64 = 830.0;
const SEG_PLATE_Y: f64 = 260.0;
const SEG_PLATE_Z: f64 = 16.0;
const SEG_PLATE_CENTER_Y: f64 = 48.0;
const SEG_ZONE_X: f64 = 160.0;
const SEG_ZONE_Y: f64 = 202.0;
const SEG_ZONE_Z: f64 = 18.0;
const SEG_WALL_Z: f64 = 62.0;

const GAUGE_BASE_X: f64 = 214.0;
const GAUGE_BASE_Y: f64 = 150.0;
const GAUGE_BASE_Z: f64 = 24.0;
const GAUGE_CENTER_Y: f64 = -196.0;
const GAUGE_PITCH_X: f64 = 43.0;
const GAUGE_BODY_H: f64 = 38.0;
const GAUGE_HANDLE_H: f64 = 16.0;
const GAUGE_RING_H: f64 = 7.0;
const GAUGE_LABEL_Z: f64 = 4.0;

const BLOCKER_BASE_X: f64 = 740.0;
const BLOCKER_BASE_Y: f64 = 124.0;
const BLOCKER_BASE_Z: f64 = 20.0;
const BLOCKER_CENTER_Y: f64 = -58.0;
const BLOCKER_WALL_X: f64 = 20.0;
const BLOCKER_WALL_Y: f64 = 34.0;
const BLOCKER_WALL_Z: f64 = 62.0;
const BLOCKER_WRONG_CLEARANCE: f64 = 2.4;

const MASTER_PLUG_RACK_X: f64 = 720.0;
const MASTER_PLUG_RACK_Y: f64 = 92.0;
const MASTER_PLUG_RACK_Z: f64 = 26.0;
const MASTER_PLUG_CENTER_Y: f64 = 120.0;
const MASTER_PLUG_H: f64 = 44.0;

const LABEL_POCKET_X: f64 = 328.0;
const LABEL_POCKET_Y: f64 = 118.0;
const LABEL_POCKET_Z: f64 = 18.0;
const LABEL_CENTER_X: f64 = 0.0;
const LABEL_CENTER_Y: f64 = -284.0;
const QR_LAND_X: f64 = 102.0;
const QR_LAND_Y: f64 = 82.0;
const RFID_LAND_X: f64 = 162.0;
const RFID_LAND_Y: f64 = 82.0;
#[cfg(test)]
const LABEL_RETAINING_CLIPS: usize = 4;
const QR_GRID: usize = 5;

const FIXTURE_BRIDGE_X: f64 = 820.0;
const FIXTURE_BRIDGE_Y: f64 = 74.0;
const FIXTURE_BRIDGE_POST_X: f64 = 24.0;
const FIXTURE_BRIDGE_POST_Z: f64 = 210.0;
const FIXTURE_BRIDGE_BEAM_Z: f64 = 22.0;
const INSPECTION_CLEARANCE_Z: f64 = 148.0;
const HARD_DATUM_PINS: usize = 4;
const CAPTURE_RAILS: usize = 2;
const WITNESS_WINDOWS: usize = 6;

const WITNESS_RACK_X: f64 = 288.0;
const WITNESS_RACK_Y: f64 = 154.0;
const WITNESS_RACK_Z: f64 = 28.0;
const WITNESS_CENTER_X: f64 = -318.0;
const WITNESS_CENTER_Y: f64 = -298.0;
const REFERENCE_COUPONS: usize = 9;

const REJECT_TRAY_X: f64 = 292.0;
const REJECT_TRAY_Y: f64 = 160.0;
const REJECT_TRAY_Z: f64 = 30.0;
const REJECT_TRAY_CENTER_X: f64 = 320.0;
const REJECT_TRAY_CENTER_Y: f64 = -296.0;
const REJECT_TRAY_WALL: f64 = 9.0;

const ROBOT_KEEP_OUT_X: f64 = 220.0;
const ROBOT_KEEP_OUT_Y: f64 = 560.0;
const ROBOT_KEEP_OUT_Z: f64 = 170.0;
const SERVICE_KEEP_OUT_X: f64 = 870.0;
const SERVICE_KEEP_OUT_Y: f64 = 128.0;
const SERVICE_KEEP_OUT_Z: f64 = 130.0;
const KEEP_OUT_RAIL: f64 = 6.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ServiceFamily {
    Pressure,
    Gas,
    Fluid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GaugeShape {
    Round,
    Hex,
    Square,
}

const FAMILIES: [ServiceFamily; FAMILY_COUNT] = [
    ServiceFamily::Pressure,
    ServiceFamily::Gas,
    ServiceFamily::Fluid,
];

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let bulkhead = keyed_bulkhead_mockup();
    export(OUTPUTS[1], &bulkhead);

    let segregation = service_family_segregation_plate();
    export(OUTPUTS[2], &segregation);

    let pressure_gauges = connector_gauge_set(ServiceFamily::Pressure);
    export(OUTPUTS[3], &pressure_gauges);

    let gas_gauges = connector_gauge_set(ServiceFamily::Gas);
    export(OUTPUTS[4], &gas_gauges);

    let fluid_gauges = connector_gauge_set(ServiceFamily::Fluid);
    export(OUTPUTS[5], &fluid_gauges);

    let blockers = no_go_blockers();
    export(OUTPUTS[6], &blockers);

    let master_plugs = keyed_master_plugs();
    export(OUTPUTS[7], &master_plugs);

    let label_pocket = rfid_qr_label_pocket();
    export(OUTPUTS[8], &label_pocket);

    let bridge = operator_independent_fixture_bridge();
    export(OUTPUTS[9], &bridge);

    let witness = witness_reference_rack();
    export(OUTPUTS[10], &witness);

    let reject = misconnection_reject_tray();
    export(OUTPUTS[11], &reject);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[12], &keepouts);

    let assembly =
        deck + bulkhead.translate(
            0.0,
            BULKHEAD_CENTER_Y,
            BULKHEAD_BASE_Z + BULKHEAD_PANEL_Z / 2.0,
        ) + segregation.translate(0.0, SEG_PLATE_CENTER_Y, DECK_Z / 2.0 + SEG_PLATE_Z / 2.0)
            + pressure_gauges.translate(
                ServiceFamily::Pressure.center_x(),
                GAUGE_CENTER_Y,
                DECK_Z / 2.0 + GAUGE_BASE_Z / 2.0,
            )
            + gas_gauges.translate(
                ServiceFamily::Gas.center_x(),
                GAUGE_CENTER_Y,
                DECK_Z / 2.0 + GAUGE_BASE_Z / 2.0,
            )
            + fluid_gauges.translate(
                ServiceFamily::Fluid.center_x(),
                GAUGE_CENTER_Y,
                DECK_Z / 2.0 + GAUGE_BASE_Z / 2.0,
            )
            + blockers.translate(0.0, BLOCKER_CENTER_Y, DECK_Z / 2.0 + BLOCKER_BASE_Z / 2.0)
            + master_plugs.translate(
                0.0,
                MASTER_PLUG_CENTER_Y,
                DECK_Z / 2.0 + MASTER_PLUG_RACK_Z / 2.0,
            )
            + label_pocket.translate(
                LABEL_CENTER_X,
                LABEL_CENTER_Y,
                DECK_Z / 2.0 + LABEL_POCKET_Z / 2.0,
            )
            + bridge.translate(
                0.0,
                BULKHEAD_CENTER_Y,
                DECK_Z / 2.0 + FIXTURE_BRIDGE_POST_Z / 2.0,
            )
            + witness.translate(
                WITNESS_CENTER_X,
                WITNESS_CENTER_Y,
                DECK_Z / 2.0 + WITNESS_RACK_Z / 2.0,
            )
            + reject.translate(
                REJECT_TRAY_CENTER_X,
                REJECT_TRAY_CENTER_Y,
                DECK_Z / 2.0 + REJECT_TRAY_Z / 2.0,
            )
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);

    export(OUTPUTS[13], &assembly);

    println!(
        "Closed service bulkhead misconnection gauge station: {:.0}mm x {:.0}mm deck, {} keyed service families, {} bulkhead mockup ports, {} color/shape coded gauges, {} cross-family no-go blockers, pressure/gas/fluid segregation barriers, RFID/QR evidence pocket, {} hard datum pins, {} capture rails, {} witness windows, and robot/service keepouts.",
        DECK_X,
        DECK_Y,
        FAMILY_COUNT,
        FAMILY_COUNT * PORTS_PER_FAMILY,
        FAMILY_COUNT * GAUGES_PER_FAMILY,
        BLOCKER_PAIR_COUNT,
        HARD_DATUM_PINS,
        CAPTURE_RAILS,
        WITNESS_WINDOWS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_service_bulkhead_misconnection_gauge_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let corner_reliefs = corner_cylinders(
        "closed_service_bulkhead_misconnection_deck_corner_relief",
        DECK_X - 2.0 * DECK_CORNER_R,
        DECK_Y - 2.0 * DECK_CORNER_R,
        DECK_CORNER_R / 2.0,
        DECK_Z + 4.0,
    );
    let bulkhead_socket = centered_cube(
        "closed_service_bulkhead_misconnection_bulkhead_panel_socket",
        BULKHEAD_PANEL_X + 34.0,
        BULKHEAD_PANEL_Y + 18.0,
        7.0,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, DECK_Z / 2.0 - 2.5);
    let label_socket = centered_cube(
        "closed_service_bulkhead_misconnection_label_pocket_socket",
        LABEL_POCKET_X + 24.0,
        LABEL_POCKET_Y + 18.0,
        5.0,
    )
    .translate(LABEL_CENTER_X, LABEL_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let reject_socket = centered_cube(
        "closed_service_bulkhead_misconnection_reject_tray_socket",
        REJECT_TRAY_X + 20.0,
        REJECT_TRAY_Y + 18.0,
        6.0,
    )
    .translate(
        REJECT_TRAY_CENTER_X,
        REJECT_TRAY_CENTER_Y,
        DECK_Z / 2.0 - 2.5,
    );

    deck - corner_reliefs - bulkhead_socket - label_socket - reject_socket + deck_datum_pads()
}

fn deck_datum_pads() -> Part {
    let mut pads = Part::empty("closed_service_bulkhead_deck_datum_pads");
    for (i, (x, y)) in deck_datum_points().iter().enumerate() {
        pads = pads
            + centered_cylinder(
                format!("closed_service_bulkhead_deck_hardened_datum_pad_{i}"),
                18.0,
                5.0,
                32,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 2.5);
    }
    pads
}

fn keyed_bulkhead_mockup() -> Part {
    let panel = centered_cube(
        "closed_service_bulkhead_keyed_mockup_panel",
        BULKHEAD_PANEL_X,
        BULKHEAD_PANEL_Y,
        BULKHEAD_PANEL_Z,
    );
    let ports = service_port_cutouts();
    let keyways = service_keyway_cutouts();
    let labels = bulkhead_family_label_lands();
    let shields = bulkhead_cross_family_shields();
    let datums = bulkhead_hard_datums();
    let foot = centered_cube(
        "closed_service_bulkhead_mockup_wide_heel",
        BULKHEAD_PANEL_X + 40.0,
        BULKHEAD_PANEL_Y + 26.0,
        26.0,
    )
    .translate(0.0, 0.0, -BULKHEAD_PANEL_Z / 2.0 + 13.0);

    panel - ports - keyways + labels + shields + datums + foot
}

fn service_port_cutouts() -> Part {
    let mut ports = Part::empty("closed_service_bulkhead_mockup_service_port_cutouts");
    for family in FAMILIES {
        for port in 0..PORTS_PER_FAMILY {
            let (x, z) = bulkhead_port_center(family, port);
            ports = ports + port_profile_cutout(family, port).translate(x, 0.0, z);
        }
    }
    ports
}

fn port_profile_cutout(family: ServiceFamily, port: usize) -> Part {
    let name = format!(
        "closed_service_bulkhead_{}_port_{}_{}_cutout",
        family.name(),
        port,
        family.shape().name()
    );
    match family.shape() {
        GaugeShape::Round => centered_cylinder(
            name,
            family.port_diameter() / 2.0,
            BULKHEAD_PANEL_Y + 8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0),
        GaugeShape::Hex => centered_cylinder(
            name,
            family.port_diameter() / 2.0,
            BULKHEAD_PANEL_Y + 8.0,
            6,
        )
        .rotate(90.0, 0.0, 0.0),
        GaugeShape::Square => centered_cube(
            name,
            family.port_diameter(),
            BULKHEAD_PANEL_Y + 8.0,
            family.port_diameter(),
        ),
    }
}

fn service_keyway_cutouts() -> Part {
    let mut keyways = Part::empty("closed_service_bulkhead_mockup_asymmetric_keyway_cutouts");
    for family in FAMILIES {
        for port in 0..PORTS_PER_FAMILY {
            let (x, z) = bulkhead_port_center(family, port);
            let key = centered_cube(
                format!(
                    "closed_service_bulkhead_{}_port_{}_keyway_cut",
                    family.name(),
                    port
                ),
                PORT_KEY_SLOT_W,
                BULKHEAD_PANEL_Y + 10.0,
                PORT_KEY_SLOT_H,
            )
            .translate(
                x + family.key_offset(),
                0.0,
                z + family.port_diameter() / 2.0 + 3.0,
            );
            let no_go_flat = centered_cube(
                format!(
                    "closed_service_bulkhead_{}_port_{}_wrong_family_flat_stop_cut",
                    family.name(),
                    port
                ),
                PORT_KEY_SLOT_W + 5.0,
                BULKHEAD_PANEL_Y + 10.0,
                PORT_KEY_SLOT_H / 2.0,
            )
            .translate(
                x - family.key_offset() * 0.7,
                0.0,
                z - family.port_diameter() / 2.0 - 4.0,
            );
            keyways = keyways + key + no_go_flat;
        }
    }
    keyways
}

fn bulkhead_family_label_lands() -> Part {
    let mut lands = Part::empty("closed_service_bulkhead_mockup_family_label_lands");
    for family in FAMILIES {
        let x = family.center_x();
        let land = centered_cube(
            format!(
                "closed_service_bulkhead_{}_{}_label_land",
                family.name(),
                family.color()
            ),
            150.0,
            8.0,
            22.0,
        )
        .translate(
            x,
            -BULKHEAD_PANEL_Y / 2.0 - 4.0,
            BULKHEAD_PANEL_Z / 2.0 - 34.0,
        );
        let shape = shape_code_marker(family, 28.0, 7.0).translate(
            x,
            -BULKHEAD_PANEL_Y / 2.0 - 9.0,
            BULKHEAD_PANEL_Z / 2.0 - 70.0,
        );
        lands = lands + land + shape;
    }
    lands
}

fn bulkhead_cross_family_shields() -> Part {
    let mut shields = Part::empty("closed_service_bulkhead_cross_family_segregation_shields");
    for i in 0..FAMILY_COUNT - 1 {
        let x = (FAMILIES[i].center_x() + FAMILIES[i + 1].center_x()) / 2.0;
        shields = shields
            + centered_cube(
                format!("closed_service_bulkhead_family_divider_shield_{i}"),
                FAMILY_BARRIER_W,
                BULKHEAD_PANEL_Y + 18.0,
                BULKHEAD_PANEL_Z - 34.0,
            )
            .translate(x, 0.0, 6.0);
    }
    shields
}

fn bulkhead_hard_datums() -> Part {
    let mut datums = Part::empty("closed_service_bulkhead_mockup_inspection_datums");
    for (i, (x, z)) in [
        (
            -BULKHEAD_PANEL_X / 2.0 + 34.0,
            -BULKHEAD_PANEL_Z / 2.0 + 46.0,
        ),
        (
            BULKHEAD_PANEL_X / 2.0 - 34.0,
            -BULKHEAD_PANEL_Z / 2.0 + 46.0,
        ),
        (
            -BULKHEAD_PANEL_X / 2.0 + 34.0,
            BULKHEAD_PANEL_Z / 2.0 - 42.0,
        ),
        (BULKHEAD_PANEL_X / 2.0 - 34.0, BULKHEAD_PANEL_Z / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_service_bulkhead_mockup_datum_button_{i}"),
                10.0,
                7.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -BULKHEAD_PANEL_Y / 2.0 - 3.5, *z);
    }
    datums
}

fn service_family_segregation_plate() -> Part {
    let base = centered_cube(
        "closed_service_bulkhead_family_segregation_plate",
        SEG_PLATE_X,
        SEG_PLATE_Y,
        SEG_PLATE_Z,
    );
    base + segregation_family_bays() + segregation_barriers() + segregation_service_features()
}

fn segregation_family_bays() -> Part {
    let mut bays = Part::empty("closed_service_bulkhead_family_segregation_bays");
    for family in FAMILIES {
        let bay = centered_cube(
            format!(
                "closed_service_bulkhead_{}_{}_segregated_bay",
                family.name(),
                family.color()
            ),
            SEG_ZONE_X,
            SEG_ZONE_Y,
            SEG_ZONE_Z,
        )
        .translate(family.center_x(), 0.0, SEG_PLATE_Z / 2.0 + SEG_ZONE_Z / 2.0);
        let rear_lip = centered_cube(
            format!("closed_service_bulkhead_{}_bay_rear_lip", family.name()),
            SEG_ZONE_X,
            10.0,
            34.0,
        )
        .translate(
            family.center_x(),
            SEG_ZONE_Y / 2.0 - 5.0,
            SEG_PLATE_Z / 2.0 + 17.0,
        );
        bays = bays + bay + rear_lip;
    }
    bays
}

fn segregation_barriers() -> Part {
    let mut barriers = Part::empty("closed_service_bulkhead_pressure_gas_fluid_barriers");
    for i in 0..FAMILY_COUNT - 1 {
        let x = (FAMILIES[i].center_x() + FAMILIES[i + 1].center_x()) / 2.0;
        barriers = barriers
            + centered_cube(
                format!("closed_service_bulkhead_segregation_wall_{i}"),
                FAMILY_BARRIER_W,
                SEG_PLATE_Y - 34.0,
                SEG_WALL_Z,
            )
            .translate(x, 0.0, SEG_PLATE_Z / 2.0 + SEG_WALL_Z / 2.0);
    }
    barriers
}

fn segregation_service_features() -> Part {
    let pressure_relief = centered_cube(
        "closed_service_bulkhead_pressure_zone_relief_groove_marker",
        SEG_ZONE_X - 40.0,
        14.0,
        7.0,
    )
    .translate(
        ServiceFamily::Pressure.center_x(),
        -SEG_ZONE_Y / 2.0 + 28.0,
        SEG_PLATE_Z / 2.0 + SEG_ZONE_Z + 3.5,
    );
    let gas_vent = centered_cube(
        "closed_service_bulkhead_gas_zone_vent_clearance_marker",
        SEG_ZONE_X - 54.0,
        18.0,
        7.0,
    )
    .translate(
        ServiceFamily::Gas.center_x(),
        -SEG_ZONE_Y / 2.0 + 28.0,
        SEG_PLATE_Z / 2.0 + SEG_ZONE_Z + 3.5,
    );
    let fluid_drip = centered_cube(
        "closed_service_bulkhead_fluid_zone_drip_trough_marker",
        SEG_ZONE_X - 32.0,
        22.0,
        8.0,
    )
    .translate(
        ServiceFamily::Fluid.center_x(),
        -SEG_ZONE_Y / 2.0 + 30.0,
        SEG_PLATE_Z / 2.0 + SEG_ZONE_Z + 4.0,
    );
    pressure_relief + gas_vent + fluid_drip
}

fn connector_gauge_set(family: ServiceFamily) -> Part {
    let base = centered_cube(
        format!(
            "closed_service_bulkhead_{}_{}_gauge_carrier",
            family.name(),
            family.color()
        ),
        GAUGE_BASE_X,
        GAUGE_BASE_Y,
        GAUGE_BASE_Z,
    );
    let mut gauges = Part::empty(format!(
        "closed_service_bulkhead_{}_shape_coded_gauges",
        family.name()
    ));
    for i in 0..GAUGES_PER_FAMILY {
        let x = centered_index(i, GAUGES_PER_FAMILY, GAUGE_PITCH_X);
        gauges = gauges + gauge_plug(family, i).translate(x, -12.0, GAUGE_BASE_Z / 2.0);
    }
    let label = centered_cube(
        format!(
            "closed_service_bulkhead_{}_{}_gauge_color_band",
            family.name(),
            family.color()
        ),
        GAUGE_BASE_X - 28.0,
        16.0,
        GAUGE_LABEL_Z,
    )
    .translate(0.0, GAUGE_BASE_Y / 2.0 - 20.0, GAUGE_BASE_Z / 2.0 + 2.0);
    let shape = shape_code_marker(family, 30.0, 7.0).translate(
        0.0,
        GAUGE_BASE_Y / 2.0 - 48.0,
        GAUGE_BASE_Z / 2.0 + 3.5,
    );
    let wrong_family_fence = centered_cube(
        format!(
            "closed_service_bulkhead_{}_wrong_family_gauge_fence",
            family.name()
        ),
        GAUGE_BASE_X - 26.0,
        8.0,
        28.0,
    )
    .translate(0.0, -GAUGE_BASE_Y / 2.0 + 10.0, GAUGE_BASE_Z / 2.0 + 14.0);

    base + gauges + label + shape + wrong_family_fence
}

fn gauge_plug(family: ServiceFamily, index: usize) -> Part {
    let diameter = family.gauge_diameter(index);
    let body = shape_prism(
        format!(
            "closed_service_bulkhead_{}_{}_gauge_body_{index}",
            family.name(),
            family.shape().name()
        ),
        family.shape(),
        diameter,
        GAUGE_BODY_H,
    )
    .translate(0.0, 0.0, GAUGE_BODY_H / 2.0);
    let key_fin = centered_cube(
        format!(
            "closed_service_bulkhead_{}_gauge_key_fin_{index}",
            family.name()
        ),
        8.0,
        7.0,
        GAUGE_BODY_H,
    )
    .translate(
        family.key_offset().signum() * (diameter / 2.0 + 4.0),
        0.0,
        GAUGE_BODY_H / 2.0,
    );
    let handle = centered_cube(
        format!(
            "closed_service_bulkhead_{}_{}_gauge_handle_{index}",
            family.name(),
            family.color()
        ),
        24.0,
        36.0,
        GAUGE_HANDLE_H,
    )
    .translate(0.0, 0.0, GAUGE_BODY_H + GAUGE_HANDLE_H / 2.0);
    let ring = shape_prism(
        format!(
            "closed_service_bulkhead_{}_gauge_size_witness_ring_{index}",
            family.name()
        ),
        family.shape(),
        diameter + 7.0 + index as f64 * 0.7,
        GAUGE_RING_H,
    )
    .translate(0.0, 0.0, GAUGE_BODY_H + GAUGE_HANDLE_H + GAUGE_RING_H / 2.0);
    body + key_fin + handle + ring
}

fn no_go_blockers() -> Part {
    let base = centered_cube(
        "closed_service_bulkhead_no_go_blocker_base",
        BLOCKER_BASE_X,
        BLOCKER_BASE_Y,
        BLOCKER_BASE_Z,
    );
    let mut blockers = Part::empty("closed_service_bulkhead_cross_family_no_go_blockers");
    for target in FAMILIES {
        let mut wrong_lane = 0;
        for wrong in FAMILIES {
            if target == wrong {
                continue;
            }
            let x = target.center_x() + centered_index(wrong_lane, FAMILY_COUNT - 1, 42.0);
            let y = centered_index(wrong.index(), FAMILY_COUNT, 30.0);
            blockers = blockers
                + single_no_go_blocker(target, wrong, wrong_lane).translate(
                    x,
                    y,
                    BLOCKER_BASE_Z / 2.0,
                );
            wrong_lane += 1;
        }
    }
    let witness_bar = centered_cube(
        "closed_service_bulkhead_no_go_blocker_witness_bar",
        BLOCKER_BASE_X - 46.0,
        8.0,
        18.0,
    )
    .translate(0.0, BLOCKER_BASE_Y / 2.0 - 12.0, BLOCKER_BASE_Z / 2.0 + 9.0);

    base + blockers + witness_bar
}

fn single_no_go_blocker(target: ServiceFamily, wrong: ServiceFamily, lane: usize) -> Part {
    let wall = centered_cube(
        format!(
            "closed_service_bulkhead_{}_blocks_{}_no_go_wall_{lane}",
            target.name(),
            wrong.name()
        ),
        BLOCKER_WALL_X,
        BLOCKER_WALL_Y,
        BLOCKER_WALL_Z,
    )
    .translate(0.0, 0.0, BLOCKER_WALL_Z / 2.0);
    let wrong_profile = shape_prism(
        format!(
            "closed_service_bulkhead_{}_wrong_{}_witness_profile_{lane}",
            target.name(),
            wrong.name()
        ),
        wrong.shape(),
        wrong.port_diameter() + BLOCKER_WRONG_CLEARANCE,
        12.0,
    )
    .translate(0.0, 0.0, BLOCKER_WALL_Z + 6.0);
    let offset_key_stop = centered_cube(
        format!(
            "closed_service_bulkhead_{}_blocks_{}_offset_key_stop_{lane}",
            target.name(),
            wrong.name()
        ),
        9.0,
        BLOCKER_WALL_Y + 10.0,
        18.0,
    )
    .translate(
        wrong.key_offset() - target.key_offset(),
        0.0,
        BLOCKER_WALL_Z / 2.0 + 9.0,
    );
    wall + wrong_profile + offset_key_stop
}

fn keyed_master_plugs() -> Part {
    let rack = centered_cube(
        "closed_service_bulkhead_keyed_master_plug_rack",
        MASTER_PLUG_RACK_X,
        MASTER_PLUG_RACK_Y,
        MASTER_PLUG_RACK_Z,
    );
    let mut plugs = Part::empty("closed_service_bulkhead_keyed_master_plugs");
    for family in FAMILIES {
        for i in 0..MASTER_PLUGS_PER_FAMILY {
            let x = family.center_x() + centered_index(i, MASTER_PLUGS_PER_FAMILY, GAUGE_PITCH_X);
            let plug = shape_prism(
                format!(
                    "closed_service_bulkhead_{}_master_plug_{}",
                    family.name(),
                    i
                ),
                family.shape(),
                family.port_diameter() - 0.9 + i as f64 * 0.6,
                MASTER_PLUG_H,
            )
            .translate(x, -8.0, MASTER_PLUG_RACK_Z / 2.0 + MASTER_PLUG_H / 2.0);
            let handle = centered_cube(
                format!(
                    "closed_service_bulkhead_{}_master_plug_{}_handle",
                    family.name(),
                    i
                ),
                34.0,
                24.0,
                14.0,
            )
            .translate(x, 28.0, MASTER_PLUG_RACK_Z / 2.0 + MASTER_PLUG_H + 7.0);
            plugs = plugs + plug + handle;
        }
    }
    rack - master_plug_pockets() + plugs
}

fn master_plug_pockets() -> Part {
    let mut pockets = Part::empty("closed_service_bulkhead_keyed_master_plug_pockets");
    for family in FAMILIES {
        for i in 0..MASTER_PLUGS_PER_FAMILY {
            let x = family.center_x() + centered_index(i, MASTER_PLUGS_PER_FAMILY, GAUGE_PITCH_X);
            pockets = pockets
                + shape_prism(
                    format!(
                        "closed_service_bulkhead_{}_master_plug_pocket_{}",
                        family.name(),
                        i
                    ),
                    family.shape(),
                    family.port_diameter() + 4.0,
                    14.0,
                )
                .translate(x, -8.0, MASTER_PLUG_RACK_Z / 2.0 - 5.0);
        }
    }
    pockets
}

fn rfid_qr_label_pocket() -> Part {
    let base = centered_cube(
        "closed_service_bulkhead_rfid_qr_label_pocket_base",
        LABEL_POCKET_X,
        LABEL_POCKET_Y,
        LABEL_POCKET_Z,
    );
    let qr_recess = centered_cube(
        "closed_service_bulkhead_qr_label_recess",
        QR_LAND_X,
        QR_LAND_Y,
        LABEL_POCKET_Z - 5.0,
    )
    .translate(
        -LABEL_POCKET_X / 2.0 + QR_LAND_X / 2.0 + 20.0,
        0.0,
        LABEL_POCKET_Z / 2.0 - 5.0,
    );
    let rfid_recess = centered_cube(
        "closed_service_bulkhead_rfid_label_recess",
        RFID_LAND_X,
        RFID_LAND_Y,
        LABEL_POCKET_Z - 5.0,
    )
    .translate(
        LABEL_POCKET_X / 2.0 - RFID_LAND_X / 2.0 - 20.0,
        0.0,
        LABEL_POCKET_Z / 2.0 - 5.0,
    );
    base - qr_recess - rfid_recess
        + qr_grid_markers()
        + rfid_coil_markers()
        + label_retaining_clips()
}

fn qr_grid_markers() -> Part {
    let mut markers = Part::empty("closed_service_bulkhead_qr_grid_markers");
    for row in 0..QR_GRID {
        for col in 0..QR_GRID {
            if (row + col) % 2 == 0 || row == 0 || col == 0 {
                markers = markers
                    + centered_cube(
                        format!("closed_service_bulkhead_qr_grid_marker_{row}_{col}"),
                        9.0,
                        9.0,
                        3.0,
                    )
                    .translate(
                        -LABEL_POCKET_X / 2.0
                            + 20.0
                            + centered_index(col, QR_GRID, 15.0)
                            + QR_LAND_X / 2.0,
                        centered_index(row, QR_GRID, 15.0),
                        LABEL_POCKET_Z / 2.0 + 1.5,
                    );
            }
        }
    }
    markers
}

fn rfid_coil_markers() -> Part {
    let center_x = LABEL_POCKET_X / 2.0 - RFID_LAND_X / 2.0 - 20.0;
    let outer = centered_cube(
        "closed_service_bulkhead_rfid_outer_coil_land",
        RFID_LAND_X - 28.0,
        RFID_LAND_Y - 16.0,
        3.0,
    )
    .translate(center_x, 0.0, LABEL_POCKET_Z / 2.0 + 1.5);
    let inner_cut = centered_cube(
        "closed_service_bulkhead_rfid_inner_clear_land",
        RFID_LAND_X - 58.0,
        RFID_LAND_Y - 42.0,
        5.0,
    )
    .translate(center_x, 0.0, LABEL_POCKET_Z / 2.0 + 1.5);
    let antenna_tail = centered_cube(
        "closed_service_bulkhead_rfid_antenna_tail_marker",
        12.0,
        RFID_LAND_Y - 20.0,
        4.0,
    )
    .translate(
        center_x + RFID_LAND_X / 2.0 - 30.0,
        0.0,
        LABEL_POCKET_Z / 2.0 + 2.0,
    );
    outer - inner_cut + antenna_tail
}

fn label_retaining_clips() -> Part {
    let mut clips = Part::empty("closed_service_bulkhead_label_retaining_clips");
    for (i, (x, y)) in [
        (-LABEL_POCKET_X / 2.0 + 18.0, -LABEL_POCKET_Y / 2.0 + 16.0),
        (-LABEL_POCKET_X / 2.0 + 18.0, LABEL_POCKET_Y / 2.0 - 16.0),
        (LABEL_POCKET_X / 2.0 - 18.0, -LABEL_POCKET_Y / 2.0 + 16.0),
        (LABEL_POCKET_X / 2.0 - 18.0, LABEL_POCKET_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        clips = clips
            + centered_cube(
                format!("closed_service_bulkhead_label_retaining_clip_{i}"),
                18.0,
                8.0,
                8.0,
            )
            .translate(*x, *y, LABEL_POCKET_Z / 2.0 + 4.0);
    }
    clips
}

fn operator_independent_fixture_bridge() -> Part {
    let left_post = centered_cube(
        "closed_service_bulkhead_fixture_bridge_left_post",
        FIXTURE_BRIDGE_POST_X,
        FIXTURE_BRIDGE_Y,
        FIXTURE_BRIDGE_POST_Z,
    )
    .translate(-FIXTURE_BRIDGE_X / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_service_bulkhead_fixture_bridge_right_post",
        FIXTURE_BRIDGE_POST_X,
        FIXTURE_BRIDGE_Y,
        FIXTURE_BRIDGE_POST_Z,
    )
    .translate(FIXTURE_BRIDGE_X / 2.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_service_bulkhead_fixture_bridge_reference_beam",
        FIXTURE_BRIDGE_X + FIXTURE_BRIDGE_POST_X,
        FIXTURE_BRIDGE_Y,
        FIXTURE_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        FIXTURE_BRIDGE_POST_Z / 2.0 + FIXTURE_BRIDGE_BEAM_Z / 2.0,
    );
    let datum_pins = fixture_datum_pins();
    let rails = fixture_capture_rails();
    let windows = fixture_witness_windows();
    let scanner_mount = centered_cube(
        "closed_service_bulkhead_fixture_bridge_scanner_mount_land",
        230.0,
        34.0,
        9.0,
    )
    .translate(0.0, -FIXTURE_BRIDGE_Y / 2.0 - 8.0, INSPECTION_CLEARANCE_Z);

    left_post + right_post + beam + datum_pins + rails + windows + scanner_mount
}

fn fixture_datum_pins() -> Part {
    let mut pins = Part::empty("closed_service_bulkhead_fixture_hard_datum_pins");
    for (i, x) in [
        -BULKHEAD_PANEL_X / 2.0 + 52.0,
        -BULKHEAD_PANEL_X / 2.0 + 118.0,
        BULKHEAD_PANEL_X / 2.0 - 118.0,
        BULKHEAD_PANEL_X / 2.0 - 52.0,
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_service_bulkhead_fixture_datum_pin_{i}"),
                6.0,
                48.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -FIXTURE_BRIDGE_Y / 2.0 - 24.0, -44.0);
    }
    pins
}

fn fixture_capture_rails() -> Part {
    let upper = centered_cube(
        "closed_service_bulkhead_fixture_upper_capture_rail",
        BULKHEAD_PANEL_X - 60.0,
        10.0,
        16.0,
    )
    .translate(0.0, -FIXTURE_BRIDGE_Y / 2.0 - 18.0, 52.0);
    let lower = centered_cube(
        "closed_service_bulkhead_fixture_lower_capture_rail",
        BULKHEAD_PANEL_X - 60.0,
        10.0,
        16.0,
    )
    .translate(0.0, -FIXTURE_BRIDGE_Y / 2.0 - 18.0, -78.0);
    upper + lower
}

fn fixture_witness_windows() -> Part {
    let mut windows = Part::empty("closed_service_bulkhead_fixture_witness_windows");
    for i in 0..WITNESS_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_service_bulkhead_fixture_witness_window_frame_{i}"),
                54.0,
                8.0,
                30.0,
            )
            .translate(
                centered_index(i, WITNESS_WINDOWS, 78.0),
                -FIXTURE_BRIDGE_Y / 2.0 - 26.0,
                8.0,
            );
    }
    windows
}

fn witness_reference_rack() -> Part {
    let rack = centered_cube(
        "closed_service_bulkhead_witness_reference_rack",
        WITNESS_RACK_X,
        WITNESS_RACK_Y,
        WITNESS_RACK_Z,
    );
    let mut pockets = Part::empty("closed_service_bulkhead_witness_reference_pockets");
    let mut coupons = Part::empty("closed_service_bulkhead_witness_reference_coupons");
    for i in 0..REFERENCE_COUPONS {
        let col = i % FAMILY_COUNT;
        let row = i / FAMILY_COUNT;
        let family = FAMILIES[col];
        let x = centered_index(col, FAMILY_COUNT, 82.0);
        let y = centered_index(row, REFERENCE_COUPONS / FAMILY_COUNT, 42.0);
        pockets = pockets
            + shape_prism(
                format!(
                    "closed_service_bulkhead_witness_{}_coupon_pocket_{row}",
                    family.name()
                ),
                family.shape(),
                family.port_diameter() + 5.0,
                14.0,
            )
            .translate(x, y, WITNESS_RACK_Z / 2.0 - 5.0);
        coupons = coupons
            + centered_cube(
                format!(
                    "closed_service_bulkhead_witness_{}_reference_coupon_{row}",
                    family.name()
                ),
                50.0,
                14.0,
                6.0,
            )
            .translate(x, y + 18.0, WITNESS_RACK_Z / 2.0 + 3.0);
    }
    rack - pockets + coupons
}

fn misconnection_reject_tray() -> Part {
    let tray = centered_cube(
        "closed_service_bulkhead_misconnection_reject_tray",
        REJECT_TRAY_X,
        REJECT_TRAY_Y,
        REJECT_TRAY_Z,
    );
    let basin = centered_cube(
        "closed_service_bulkhead_misconnection_reject_basin",
        REJECT_TRAY_X - 2.0 * REJECT_TRAY_WALL,
        REJECT_TRAY_Y - 2.0 * REJECT_TRAY_WALL,
        REJECT_TRAY_Z - 8.0,
    )
    .translate(0.0, 0.0, REJECT_TRAY_Z / 2.0 - 6.0);
    let mut dividers = Part::empty("closed_service_bulkhead_misconnection_reject_dividers");
    for i in 1..FAMILY_COUNT {
        dividers = dividers
            + centered_cube(
                format!("closed_service_bulkhead_misconnection_reject_divider_{i}"),
                8.0,
                REJECT_TRAY_Y - 18.0,
                24.0,
            )
            .translate(
                centered_index(i, FAMILY_COUNT + 1, REJECT_TRAY_X / 3.0),
                0.0,
                REJECT_TRAY_Z / 2.0 + 12.0,
            );
    }
    let status_land = centered_cube(
        "closed_service_bulkhead_misconnection_reject_status_land",
        REJECT_TRAY_X - 40.0,
        18.0,
        5.0,
    )
    .translate(0.0, -REJECT_TRAY_Y / 2.0 + 22.0, REJECT_TRAY_Z / 2.0 + 2.5);
    tray - basin + dividers + status_land
}

fn robot_service_keepouts() -> Part {
    let robot = clearance_box(
        "closed_service_bulkhead_left_robot_approach_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(-DECK_X / 2.0 - ROBOT_KEEP_OUT_X / 2.0 + 38.0, 10.0, 0.0);
    let front_service = clearance_box(
        "closed_service_bulkhead_front_operator_service_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -DECK_Y / 2.0 - SERVICE_KEEP_OUT_Y / 2.0 + 34.0, -18.0);
    let rear_service = clearance_box(
        "closed_service_bulkhead_rear_bulkhead_service_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, DECK_Y / 2.0 + SERVICE_KEEP_OUT_Y / 2.0 - 34.0, -18.0);
    robot + front_service + rear_service
}

fn shape_code_marker(family: ServiceFamily, size: f64, height: f64) -> Part {
    shape_prism(
        format!(
            "closed_service_bulkhead_{}_{}_shape_code_marker",
            family.name(),
            family.shape().name()
        ),
        family.shape(),
        size,
        height,
    )
}

fn shape_prism(name: impl Into<String>, shape: GaugeShape, size: f64, height: f64) -> Part {
    match shape {
        GaugeShape::Round => centered_cylinder(name, size / 2.0, height, 36),
        GaugeShape::Hex => centered_cylinder(name, size / 2.0, height, 6),
        GaugeShape::Square => centered_cube(name, size, size, height),
    }
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

fn bulkhead_port_center(family: ServiceFamily, port: usize) -> (f64, f64) {
    (
        family.center_x(),
        centered_index(port, PORTS_PER_FAMILY, PORT_PITCH_Z),
    )
}

fn deck_datum_points() -> [(f64, f64); HARD_DATUM_PINS] {
    [
        (-DECK_X / 2.0 + 52.0, -DECK_Y / 2.0 + 52.0),
        (DECK_X / 2.0 - 52.0, -DECK_Y / 2.0 + 52.0),
        (-DECK_X / 2.0 + 52.0, DECK_Y / 2.0 - 52.0),
        (DECK_X / 2.0 - 52.0, DECK_Y / 2.0 - 52.0),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

impl ServiceFamily {
    fn index(self) -> usize {
        match self {
            ServiceFamily::Pressure => 0,
            ServiceFamily::Gas => 1,
            ServiceFamily::Fluid => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            ServiceFamily::Pressure => "pressure",
            ServiceFamily::Gas => "gas",
            ServiceFamily::Fluid => "fluid",
        }
    }

    fn color(self) -> &'static str {
        match self {
            ServiceFamily::Pressure => "red",
            ServiceFamily::Gas => "amber",
            ServiceFamily::Fluid => "blue",
        }
    }

    fn shape(self) -> GaugeShape {
        match self {
            ServiceFamily::Pressure => GaugeShape::Round,
            ServiceFamily::Gas => GaugeShape::Hex,
            ServiceFamily::Fluid => GaugeShape::Square,
        }
    }

    fn center_x(self) -> f64 {
        centered_index(self.index(), FAMILY_COUNT, FAMILY_PITCH_X)
    }

    fn port_diameter(self) -> f64 {
        match self {
            ServiceFamily::Pressure => 25.0,
            ServiceFamily::Gas => 29.0,
            ServiceFamily::Fluid => 33.0,
        }
    }

    fn gauge_diameter(self, index: usize) -> f64 {
        let delta = match index {
            0 => -1.0,
            1 => 0.0,
            2 => 1.1,
            _ => 2.2,
        };
        self.port_diameter() + delta
    }

    fn key_offset(self) -> f64 {
        match self {
            ServiceFamily::Pressure => -13.0,
            ServiceFamily::Gas => 0.0,
            ServiceFamily::Fluid => 13.0,
        }
    }
}

impl GaugeShape {
    fn name(self) -> &'static str {
        match self {
            GaugeShape::Round => "round",
            GaugeShape::Hex => "hex",
            GaugeShape::Square => "square",
        }
    }
}

#[cfg(test)]
fn family_zone_gap() -> f64 {
    FAMILY_PITCH_X - SEG_ZONE_X
}

#[cfg(test)]
fn blocker_count() -> usize {
    let mut count = 0;
    for target in FAMILIES {
        for wrong in FAMILIES {
            if target != wrong {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
fn shape_codes() -> [&'static str; FAMILY_COUNT] {
    [
        ServiceFamily::Pressure.shape().name(),
        ServiceFamily::Gas.shape().name(),
        ServiceFamily::Fluid.shape().name(),
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
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_service_bulkhead_misconnection_gauge_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_set_covers_ticket_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"keyed_bulkhead_mockup"));
        assert!(REQUIRED_FEATURES.contains(&"color_shape_coded_gauges"));
        assert!(REQUIRED_FEATURES.contains(&"no_go_blockers"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_gas_fluid_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"rfid_label_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"qr_label_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"operator_independent_fixture"));
    }

    #[test]
    fn pressure_gas_and_fluid_ports_are_segregated() {
        assert_eq!(FAMILY_COUNT, 3);
        assert_eq!(PORTS_PER_FAMILY, 3);
        assert_eq!(FAMILY_COUNT * PORTS_PER_FAMILY, 9);
        assert!(family_zone_gap() >= FAMILY_ZONE_MIN_GAP);
        assert!(FAMILY_BARRIER_W >= 10.0);
        assert!(ServiceFamily::Pressure.center_x() < ServiceFamily::Gas.center_x());
        assert!(ServiceFamily::Gas.center_x() < ServiceFamily::Fluid.center_x());

        for family in FAMILIES {
            let x = family.center_x();
            assert!(x.abs() + SEG_ZONE_X / 2.0 < SEG_PLATE_X / 2.0 - 30.0);
            assert!(x.abs() + family.port_diameter() / 2.0 < BULKHEAD_PANEL_X / 2.0 - 42.0);
        }
    }

    #[test]
    fn gauge_shapes_and_colors_are_unique_by_service_family() {
        let shapes: BTreeSet<&str> = shape_codes().iter().copied().collect();
        let colors: BTreeSet<&str> = FAMILIES.iter().map(|family| family.color()).collect();
        assert_eq!(shapes.len(), FAMILY_COUNT);
        assert_eq!(colors.len(), FAMILY_COUNT);
        assert_eq!(ServiceFamily::Pressure.shape(), GaugeShape::Round);
        assert_eq!(ServiceFamily::Gas.shape(), GaugeShape::Hex);
        assert_eq!(ServiceFamily::Fluid.shape(), GaugeShape::Square);
        assert_eq!(GAUGES_PER_FAMILY, 4);
    }

    #[test]
    fn no_go_blockers_cover_every_wrong_family_pair() {
        assert_eq!(blocker_count(), BLOCKER_PAIR_COUNT);
        assert_eq!(BLOCKER_PAIR_COUNT, 6);
        assert!(BLOCKER_WRONG_CLEARANCE < 3.0);
        for target in FAMILIES {
            for wrong in FAMILIES {
                if target != wrong {
                    assert_ne!(target.shape().name(), wrong.shape().name());
                }
            }
        }
    }

    #[test]
    fn rfid_qr_label_pocket_has_redundant_evidence_lands() {
        assert!(LABEL_POCKET_X > QR_LAND_X + RFID_LAND_X + 44.0);
        assert!(LABEL_POCKET_Y > QR_LAND_Y + 24.0);
        assert_eq!(LABEL_RETAINING_CLIPS, 4);
        assert_eq!(QR_GRID * QR_GRID, 25);
        assert!(RFID_LAND_X > QR_LAND_X);
    }

    #[test]
    fn operator_independent_fixtures_define_hard_datums() {
        assert_eq!(HARD_DATUM_PINS, 4);
        assert_eq!(CAPTURE_RAILS, 2);
        assert_eq!(WITNESS_WINDOWS, 6);
        assert!(INSPECTION_CLEARANCE_Z > BULKHEAD_PANEL_Z / 2.0);
        assert!(FIXTURE_BRIDGE_X > BULKHEAD_PANEL_X);
        assert!(FIXTURE_BRIDGE_POST_Z > INSPECTION_CLEARANCE_Z + 40.0);
    }

    #[test]
    fn assembly_footprint_fits_on_station_deck() {
        assert!(BULKHEAD_PANEL_X < DECK_X - 120.0);
        assert!(SEG_PLATE_X < DECK_X - 100.0);
        assert!(SERVICE_KEEP_OUT_X < DECK_X);
        assert!(LABEL_CENTER_Y - LABEL_POCKET_Y / 2.0 > -DECK_Y / 2.0 + 8.0);
        assert!(REJECT_TRAY_CENTER_X + REJECT_TRAY_X / 2.0 < DECK_X / 2.0 - 20.0);
        assert!(WITNESS_CENTER_X - WITNESS_RACK_X / 2.0 > -DECK_X / 2.0 + 20.0);
    }
}
