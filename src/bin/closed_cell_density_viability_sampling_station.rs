use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed automated cell-density/viability sampling and QC station upstream of
// seeding.
//
// Intent:
// - Keep the cell suspension in a closed sample loop from upstream bag/module
//   connection through mixing, metering, cassette imaging, return, and waste
//   quarantine branches.
// - Represent gentle mixing, low-dead-volume metering, disposable
//   imaging/counting cassette handling, viability reagent isolation, barcode
//   and lot traceability, and automated pre-seeding release gates.
// - Model bought-in analyzers, valves, sensors, and reagent packs as envelopes
//   and keyed interfaces only. Biological acceptance criteria, assay
//   validation, and batch disposition remain separate quality gates.

const OUTPUTS: &[&str] = &[
    "output/closed_cell_density_viability_sampling_station_base_enclosure.stl",
    "output/closed_cell_density_viability_sampling_station_closed_sample_loop_manifold.stl",
    "output/closed_cell_density_viability_sampling_station_gentle_mixing_module.stl",
    "output/closed_cell_density_viability_sampling_station_sample_metering_bank.stl",
    "output/closed_cell_density_viability_sampling_station_disposable_imaging_counting_cassette_dock.stl",
    "output/closed_cell_density_viability_sampling_station_viability_reagent_isolation_pod.stl",
    "output/closed_cell_density_viability_sampling_station_waste_quarantine_pod.stl",
    "output/closed_cell_density_viability_sampling_station_barcode_lot_traceability_panel.stl",
    "output/closed_cell_density_viability_sampling_station_automated_release_gate_matrix.stl",
    "output/closed_cell_density_viability_sampling_station_robot_service_keepouts.stl",
    "output/closed_cell_density_viability_sampling_station_closed_sample_loop_routes.stl",
    "output/closed_cell_density_viability_sampling_station_assembly.stl",
];

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 34.0;
const LEAK_TRAY_Z: f64 = 8.0;
const MOUNT_HOLES: usize = 8;
const MOUNT_HOLE_D: f64 = 6.6;

const SAMPLE_LOOP_X: f64 = 560.0;
const SAMPLE_LOOP_Y: f64 = 160.0;
const SAMPLE_LOOP_Z: f64 = 58.0;
const SAMPLE_LOOP_POS: (f64, f64) = (-230.0, 205.0);
const LOOP_INLET_PORTS: usize = 2;
const LOOP_RETURN_PORTS: usize = 2;
const LOOP_SENSOR_TAPS: usize = 4;
const LOOP_INTERLOCK_VALVES: usize = 8;
const LOOP_BORE_D: f64 = 6.4;
const LOOP_BRANCH_D: f64 = 4.8;
const STERILE_CONNECTOR_PORTS: usize = 8;

const MIXER_X: f64 = 300.0;
const MIXER_Y: f64 = 210.0;
const MIXER_Z: f64 = 70.0;
const MIXER_POS: (f64, f64) = (-430.0, -80.0);
const MIXER_BAG_X: f64 = 226.0;
const MIXER_BAG_Y: f64 = 140.0;
const MIXER_BAG_Z: f64 = 34.0;
const MIXER_ROLLERS: usize = 2;
const MIXER_BAFFLES: usize = 5;
const MAX_MIXER_RPM: f64 = 9.0;
const ROCK_ANGLE_LIMIT_DEG: f64 = 6.0;

const METERING_X: f64 = 280.0;
const METERING_Y: f64 = 190.0;
const METERING_Z: f64 = 58.0;
const METERING_POS: (f64, f64) = (-70.0, -95.0);
const METERING_CHANNELS: usize = 4;
const METERING_LOOP_UL: f64 = 60.0;
const METERING_PITCH_X: f64 = 58.0;
const CHECK_VALVES_PER_CHANNEL: usize = 2;

const CASSETTE_DOCK_X: f64 = 400.0;
const CASSETTE_DOCK_Y: f64 = 290.0;
const CASSETTE_DOCK_Z: f64 = 64.0;
const CASSETTE_DOCK_POS: (f64, f64) = (295.0, 105.0);
const IMAGING_CASSETTE_X: f64 = 106.0;
const IMAGING_CASSETTE_Y: f64 = 58.0;
const IMAGING_CASSETTE_Z: f64 = 8.0;
const DISPOSABLE_CASSETTES: usize = 12;
const CASSETTE_DATUM_PINS: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 188.0;
const ILLUMINATION_BARS: usize = 2;

const REAGENT_POD_X: f64 = 250.0;
const REAGENT_POD_Y: f64 = 180.0;
const REAGENT_POD_Z: f64 = 72.0;
const REAGENT_POD_POS: (f64, f64) = (500.0, -150.0);
const VIABILITY_REAGENT_CARTRIDGES: usize = 2;
const REAGENT_ISOLATION_VALVES: usize = 4;
const REAGENT_SEAL_WITNESSES: usize = 2;
const REAGENT_AIR_GAP_MM: f64 = 24.0;

const WASTE_POD_X: f64 = 310.0;
const WASTE_POD_Y: f64 = 170.0;
const WASTE_POD_Z: f64 = 92.0;
const WASTE_POD_POS: (f64, f64) = (-435.0, -310.0);
const WASTE_BOTTLES: usize = 2;
const USED_CASSETTE_QUARANTINE_SLOTS: usize = DISPOSABLE_CASSETTES;
const WASTE_LOCKS: usize = 3;
const BIOBURDEN_SEGREGATION_GAP: f64 = 40.0;

const TRACE_PANEL_X: f64 = 1000.0;
const TRACE_PANEL_Y: f64 = 84.0;
const TRACE_PANEL_Z: f64 = 24.0;
const TRACE_PANEL_POS: (f64, f64) = (0.0, 346.0);
const BARCODE_LANDS: usize = 14;
const RFID_LANDS: usize = 4;
const LOT_CARD_SLOTS: usize = 6;
const BARCODE_PITCH_X: f64 = 68.0;

const RELEASE_GATE_X: f64 = 330.0;
const RELEASE_GATE_Y: f64 = 150.0;
const RELEASE_GATE_Z: f64 = 56.0;
const RELEASE_GATE_POS: (f64, f64) = (150.0, -315.0);
const RELEASE_LANES: usize = 3;
const RELEASE_GATE_SOLENOIDS: usize = 6;
const RELEASE_DECISION_INPUTS: usize = 5;
const RELEASE_LANE_PITCH_X: f64 = 92.0;

const ROUTE_Z: f64 = BASE_Z + 34.0;
const ROUTE_TUBE_D: f64 = 7.2;
const SAMPLE_BRANCH_TUBE_D: f64 = 5.0;
const CLOSED_LOOP_ROUTE_POINTS: usize = 6;
const WASTE_BRANCHES: usize = 2;

const ROBOT_KEEP_OUT_Z: f64 = 260.0;
const FRONT_SERVICE_CLEARANCE: f64 = 310.0;
const REAR_TUBE_SERVICE_CLEARANCE: f64 = 170.0;
const CASSETTE_PICK_CLEARANCE_Z: f64 = 150.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let base = base_enclosure();
    export(&base, OUTPUTS[0]);

    let loop_manifold = closed_sample_loop_manifold();
    export(&loop_manifold, OUTPUTS[1]);

    let mixer = gentle_mixing_module();
    export(&mixer, OUTPUTS[2]);

    let metering = sample_metering_bank();
    export(&metering, OUTPUTS[3]);

    let cassette = disposable_imaging_counting_cassette_dock();
    export(&cassette, OUTPUTS[4]);

    let reagent = viability_reagent_isolation_pod();
    export(&reagent, OUTPUTS[5]);

    let waste = waste_quarantine_pod();
    export(&waste, OUTPUTS[6]);

    let trace = barcode_lot_traceability_panel();
    export(&trace, OUTPUTS[7]);

    let gates = automated_release_gate_matrix();
    export(&gates, OUTPUTS[8]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[9]);

    let routes = closed_sample_loop_routes();
    export(&routes, OUTPUTS[10]);

    let assembly =
        base + loop_manifold.translate(
            SAMPLE_LOOP_POS.0,
            SAMPLE_LOOP_POS.1,
            BASE_Z / 2.0 + SAMPLE_LOOP_Z / 2.0,
        ) + mixer.translate(MIXER_POS.0, MIXER_POS.1, BASE_Z / 2.0 + MIXER_Z / 2.0)
            + metering.translate(
                METERING_POS.0,
                METERING_POS.1,
                BASE_Z / 2.0 + METERING_Z / 2.0,
            )
            + cassette.translate(
                CASSETTE_DOCK_POS.0,
                CASSETTE_DOCK_POS.1,
                BASE_Z / 2.0 + CASSETTE_DOCK_Z / 2.0,
            )
            + reagent.translate(
                REAGENT_POD_POS.0,
                REAGENT_POD_POS.1,
                BASE_Z / 2.0 + REAGENT_POD_Z / 2.0,
            )
            + waste.translate(
                WASTE_POD_POS.0,
                WASTE_POD_POS.1,
                BASE_Z / 2.0 + WASTE_POD_Z / 2.0,
            )
            + trace.translate(
                TRACE_PANEL_POS.0,
                TRACE_PANEL_POS.1,
                BASE_Z / 2.0 + TRACE_PANEL_Z / 2.0,
            )
            + gates.translate(
                RELEASE_GATE_POS.0,
                RELEASE_GATE_POS.1,
                BASE_Z / 2.0 + RELEASE_GATE_Z / 2.0,
            )
            + routes
            + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed cell-density/viability sampling station: {:.0}mm x {:.0}mm deck with closed {}-point recirculating sample loop, {} low-shear mixer rollers at <= {:.0} rpm, {} metering channels ({:.0} uL loops), {} disposable imaging/counting cassettes, {} isolated viability reagent cartridges, {} quarantine waste bottles, {} barcode/RFID traceability lands, and {} automated release lanes upstream of seeding.",
        STATION_X,
        STATION_Y,
        CLOSED_LOOP_ROUTE_POINTS,
        MIXER_ROLLERS,
        MAX_MIXER_RPM,
        METERING_CHANNELS,
        METERING_LOOP_UL,
        DISPOSABLE_CASSETTES,
        VIABILITY_REAGENT_CARTRIDGES,
        WASTE_BOTTLES,
        BARCODE_LANDS + RFID_LANDS,
        RELEASE_LANES
    );
    println!(
        "Isolation features modeled: {} loop interlock valves, {} reagent isolation valves, {:.0}mm reagent air-gap land, {} sealed waste locks, {} used-cassette quarantine slots, and {:.0}mm front service clearance.",
        LOOP_INTERLOCK_VALVES,
        REAGENT_ISOLATION_VALVES,
        REAGENT_AIR_GAP_MM,
        WASTE_LOCKS,
        USED_CASSETTE_QUARANTINE_SLOTS,
        FRONT_SERVICE_CLEARANCE
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout_constraints() {
    let rects = [
        sample_loop_rect(),
        mixer_rect(),
        metering_rect(),
        cassette_rect(),
        reagent_rect(),
        waste_rect(),
        trace_rect(),
        release_gate_rect(),
    ];

    for rect in rects {
        assert!(
            rect.fits_inside(STATION_X, STATION_Y),
            "{} exceeds station deck footprint",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in i + 1..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert!(clearance_between(reagent_rect(), waste_rect()) >= BIOBURDEN_SEGREGATION_GAP);
    assert!(DISPOSABLE_CASSETTES >= METERING_CHANNELS * 2);
    assert!(BARCODE_LANDS >= DISPOSABLE_CASSETTES);
    assert!(RELEASE_LANES == 3);
    assert!(CLOSED_LOOP_ROUTE_POINTS == closed_loop_nodes().len());
    assert_eq!(LOOP_INLET_PORTS, LOOP_RETURN_PORTS);
    assert!(ROCK_ANGLE_LIMIT_DEG <= 7.0);
    assert_eq!(CHECK_VALVES_PER_CHANNEL, 2);
    assert_eq!(REAGENT_SEAL_WITNESSES, VIABILITY_REAGENT_CARTRIDGES);
    assert_eq!(WASTE_BRANCHES, WASTE_BOTTLES);
}

fn base_enclosure() -> Part {
    let deck = centered_cube(
        "density_viability_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let shallow_leak_basin = centered_cube(
        "density_viability_station_continuous_leak_basin_relief",
        STATION_X - 118.0,
        STATION_Y - 112.0,
        LEAK_TRAY_Z,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - LEAK_TRAY_Z / 2.0 + 1.0);
    let low_point_drain = centered_cylinder(
        "density_viability_station_closed_drain_clearance",
        14.0 / 2.0,
        BASE_Z + 6.0,
        32,
    )
    .translate(-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 46.0, 0.0);

    let rear_rim = centered_cube(
        "density_viability_station_rear_gasket_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_rim = centered_cube(
        "density_viability_station_front_gasket_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left_rim = centered_cube(
        "density_viability_station_left_gasket_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right_rim = centered_cube(
        "density_viability_station_right_gasket_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    deck - shallow_leak_basin - low_point_drain
        + rear_rim
        + front_rim
        + left_rim
        + right_rim
        + station_mount_bosses()
        + module_datum_rails()
}

fn station_mount_bosses() -> Part {
    let mut bosses = Part::empty("density_viability_station_mount_bosses");
    for (i, (x, y)) in station_mount_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("density_viability_station_mount_boss_{i}"),
            17.0 / 2.0,
            9.0,
            28,
        )
        .translate(x, y, BASE_Z / 2.0 + 9.0 / 2.0);
        let bore = centered_cylinder(
            format!("density_viability_station_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            13.0,
            24,
        )
        .translate(x, y, BASE_Z / 2.0 + 9.0 / 2.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn module_datum_rails() -> Part {
    let mut rails = Part::empty("density_viability_station_module_datum_rails");
    for (i, rect) in [
        sample_loop_rect(),
        mixer_rect(),
        metering_rect(),
        cassette_rect(),
        reagent_rect(),
        waste_rect(),
        release_gate_rect(),
    ]
    .into_iter()
    .enumerate()
    {
        let rear = centered_cube(
            format!("density_viability_station_module_{i}_rear_datum_rail"),
            rect.w + 18.0,
            8.0,
            12.0,
        )
        .translate(rect.x, rect.y + rect.h / 2.0 + 8.0, BASE_Z / 2.0 + 6.0);
        let left = centered_cube(
            format!("density_viability_station_module_{i}_left_datum_rail"),
            8.0,
            rect.h + 18.0,
            12.0,
        )
        .translate(rect.x - rect.w / 2.0 - 8.0, rect.y, BASE_Z / 2.0 + 6.0);
        rails = rails + rear + left;
    }
    rails
}

fn closed_sample_loop_manifold() -> Part {
    let body = centered_cube(
        "density_viability_closed_sample_loop_manifold_body",
        SAMPLE_LOOP_X,
        SAMPLE_LOOP_Y,
        SAMPLE_LOOP_Z,
    );
    let cover_land = centered_cube(
        "density_viability_closed_sample_loop_lid_land",
        SAMPLE_LOOP_X - 36.0,
        SAMPLE_LOOP_Y - 28.0,
        7.0,
    )
    .translate(0.0, 0.0, SAMPLE_LOOP_Z / 2.0 + 3.5);

    let supply_bore = tube_cut_x(
        "density_viability_closed_loop_supply_bore",
        -SAMPLE_LOOP_X / 2.0 + 36.0,
        SAMPLE_LOOP_X / 2.0 - 36.0,
        30.0,
        4.0,
        LOOP_BORE_D,
    );
    let return_bore = tube_cut_x(
        "density_viability_closed_loop_return_bore",
        -SAMPLE_LOOP_X / 2.0 + 36.0,
        SAMPLE_LOOP_X / 2.0 - 36.0,
        -30.0,
        4.0,
        LOOP_BORE_D,
    );
    let recirc_bridge = tube_cut_y(
        "density_viability_closed_loop_recirc_bridge_bore",
        -SAMPLE_LOOP_X / 2.0 + 72.0,
        -30.0,
        30.0,
        4.0,
        LOOP_BRANCH_D,
    );
    let analyzer_branch = tube_cut_y(
        "density_viability_closed_loop_analyzer_branch_bore",
        SAMPLE_LOOP_X / 2.0 - 118.0,
        -30.0,
        30.0,
        4.0,
        LOOP_BRANCH_D,
    );

    let mut connector_ports = Part::empty("density_viability_closed_loop_sterile_ports");
    for i in 0..STERILE_CONNECTOR_PORTS {
        let x = connector_port_x(i);
        let y = if i < STERILE_CONNECTOR_PORTS / 2 {
            58.0
        } else {
            -58.0
        };
        let port = centered_cylinder(
            format!("density_viability_closed_loop_sterile_port_{i}"),
            10.0 / 2.0,
            24.0,
            28,
        )
        .translate(x, y, SAMPLE_LOOP_Z / 2.0 - 8.0);
        let label_land = centered_cube(
            format!("density_viability_closed_loop_port_{i}_id_land"),
            36.0,
            16.0,
            4.0,
        )
        .translate(x, y.signum() * 76.0, SAMPLE_LOOP_Z / 2.0 + 2.0);
        connector_ports = connector_ports + label_land - port;
    }

    let mut valves = Part::empty("density_viability_closed_loop_interlock_valves");
    for i in 0..LOOP_INTERLOCK_VALVES {
        let x = valve_x(i);
        let y = if i % 2 == 0 { 30.0 } else { -30.0 };
        let saddle = centered_cube(
            format!("density_viability_closed_loop_valve_saddle_{i}"),
            34.0,
            24.0,
            18.0,
        )
        .translate(x, y, SAMPLE_LOOP_Z / 2.0 + 9.0);
        let pinch_window = centered_cube(
            format!("density_viability_closed_loop_valve_pinch_window_{i}"),
            16.0,
            12.0,
            12.0,
        )
        .translate(x, y, SAMPLE_LOOP_Z / 2.0 + 10.0);
        valves = valves + (saddle - pinch_window);
    }

    let mut sensors = Part::empty("density_viability_closed_loop_sensor_taps");
    for i in 0..LOOP_SENSOR_TAPS {
        let x = -180.0 + i as f64 * 120.0;
        let pad = centered_cube(
            format!("density_viability_closed_loop_sensor_pad_{i}"),
            54.0,
            26.0,
            8.0,
        )
        .translate(x, 0.0, SAMPLE_LOOP_Z / 2.0 + 4.0);
        let tap = centered_cylinder(
            format!("density_viability_closed_loop_sensor_tap_bore_{i}"),
            4.0 / 2.0,
            20.0,
            20,
        )
        .translate(x, 0.0, SAMPLE_LOOP_Z / 2.0 + 4.0);
        sensors = sensors + (pad - tap);
    }

    body + cover_land + valves + sensors + connector_ports
        - supply_bore
        - return_bore
        - recirc_bridge
        - analyzer_branch
        + bubble_witness_dome()
}

fn bubble_witness_dome() -> Part {
    let base = centered_cylinder(
        "density_viability_closed_loop_bubble_witness_dome",
        38.0 / 2.0,
        18.0,
        36,
    )
    .translate(SAMPLE_LOOP_X / 2.0 - 64.0, 0.0, SAMPLE_LOOP_Z / 2.0 + 9.0);
    let sight = centered_cylinder(
        "density_viability_closed_loop_bubble_witness_sight_clearance",
        22.0 / 2.0,
        20.0,
        32,
    )
    .translate(SAMPLE_LOOP_X / 2.0 - 64.0, 0.0, SAMPLE_LOOP_Z / 2.0 + 9.0);
    base - sight
}

fn gentle_mixing_module() -> Part {
    let tray = centered_cube(
        "density_viability_gentle_mixer_secondary_tray",
        MIXER_X,
        MIXER_Y,
        28.0,
    );
    let bag_recess = centered_cube(
        "density_viability_gentle_mixer_single_use_bag_recess",
        MIXER_BAG_X,
        MIXER_BAG_Y,
        MIXER_BAG_Z / 2.0,
    )
    .translate(0.0, 10.0, 8.0);
    let rear_stop = centered_cube(
        "density_viability_gentle_mixer_rear_bag_stop",
        MIXER_BAG_X + 36.0,
        10.0,
        34.0,
    )
    .translate(0.0, MIXER_Y / 2.0 - 20.0, 31.0);
    let front_soft_lip = centered_cube(
        "density_viability_gentle_mixer_front_soft_lip",
        MIXER_BAG_X + 10.0,
        8.0,
        22.0,
    )
    .translate(0.0, -MIXER_Y / 2.0 + 28.0, 25.0);

    let mut rollers = Part::empty("density_viability_gentle_mixer_slow_roller_pair");
    for i in 0..MIXER_ROLLERS {
        let y = -36.0 + i as f64 * 72.0;
        let roller = centered_cylinder(
            format!("density_viability_gentle_mixer_roller_{i}"),
            15.0 / 2.0,
            MIXER_BAG_X + 18.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 52.0);
        let shaft_keepout = centered_cylinder(
            format!("density_viability_gentle_mixer_roller_{i}_shaft_clearance"),
            5.0 / 2.0,
            MIXER_BAG_X + 32.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 52.0);
        rollers = rollers + (roller - shaft_keepout);
    }

    let mut baffles = Part::empty("density_viability_gentle_mixer_bag_baffle_reliefs");
    for i in 0..MIXER_BAFFLES {
        let x = -88.0 + i as f64 * 44.0;
        let rib = centered_cube(
            format!("density_viability_gentle_mixer_low_shear_baffle_{i}"),
            8.0,
            MIXER_BAG_Y - 36.0,
            7.0,
        )
        .translate(x, 10.0, 32.0);
        baffles = baffles + rib;
    }

    let inlet_gland = centered_cylinder(
        "density_viability_gentle_mixer_closed_inlet_gland",
        18.0 / 2.0,
        24.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-MIXER_X / 2.0 + 34.0, MIXER_Y / 2.0 - 46.0, 42.0);
    let outlet_gland = centered_cylinder(
        "density_viability_gentle_mixer_closed_outlet_gland",
        18.0 / 2.0,
        24.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MIXER_X / 2.0 - 34.0, -MIXER_Y / 2.0 + 46.0, 42.0);
    let tach_pad = centered_cube(
        "density_viability_gentle_mixer_rocker_tachometer_pad",
        68.0,
        30.0,
        8.0,
    )
    .translate(-MIXER_X / 2.0 + 58.0, -MIXER_Y / 2.0 + 42.0, 32.0);

    tray - bag_recess + rear_stop + front_soft_lip + rollers + baffles + tach_pad
        - inlet_gland
        - outlet_gland
}

fn sample_metering_bank() -> Part {
    let base = centered_cube(
        "density_viability_sample_metering_bank_base",
        METERING_X,
        METERING_Y,
        METERING_Z,
    );
    let service_lid = centered_cube(
        "density_viability_sample_metering_bank_lid_land",
        METERING_X - 24.0,
        METERING_Y - 24.0,
        7.0,
    )
    .translate(0.0, 0.0, METERING_Z / 2.0 + 3.5);

    let mut channels = Part::empty("density_viability_sample_metering_channels");
    for i in 0..METERING_CHANNELS {
        let x = metering_channel_x(i);
        let pump = centered_cylinder(
            format!("density_viability_sample_metering_pump_head_{i}"),
            25.0 / 2.0,
            18.0,
            36,
        )
        .translate(x, 28.0, METERING_Z / 2.0 + 9.0);
        let loop_pocket = centered_cube(
            format!("density_viability_sample_metering_{i}_sixty_ul_loop_pocket"),
            40.0,
            58.0,
            12.0,
        )
        .translate(x, -34.0, METERING_Z / 2.0 + 5.0);
        let loop_bore = centered_cylinder(
            format!("density_viability_sample_metering_{i}_loop_bore"),
            6.0 / 2.0,
            44.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -34.0, METERING_Z / 2.0 + 5.0);
        let check_a = check_valve_land(i, 0).translate(x - 15.0, 72.0, METERING_Z / 2.0 + 9.0);
        let check_b = check_valve_land(i, 1).translate(x + 15.0, -78.0, METERING_Z / 2.0 + 9.0);
        channels = channels + pump + check_a + check_b + (loop_pocket - loop_bore);
    }

    let inlet_header = tube_cut_x(
        "density_viability_sample_metering_inlet_header_bore",
        -METERING_X / 2.0 + 26.0,
        METERING_X / 2.0 - 26.0,
        74.0,
        0.0,
        LOOP_BRANCH_D,
    );
    let outlet_header = tube_cut_x(
        "density_viability_sample_metering_outlet_header_bore",
        -METERING_X / 2.0 + 26.0,
        METERING_X / 2.0 - 26.0,
        -78.0,
        0.0,
        LOOP_BRANCH_D,
    );
    let drip_lip = centered_cube(
        "density_viability_sample_metering_drip_lip_to_waste",
        METERING_X - 42.0,
        8.0,
        10.0,
    )
    .translate(0.0, -METERING_Y / 2.0 + 12.0, METERING_Z / 2.0 + 5.0);

    base + service_lid + channels + drip_lip - inlet_header - outlet_header
}

fn check_valve_land(channel: usize, index: usize) -> Part {
    let body = centered_cube(
        format!("density_viability_sample_metering_channel_{channel}_check_valve_{index}"),
        26.0,
        18.0,
        14.0,
    );
    let bore = centered_cylinder(
        format!("density_viability_sample_metering_channel_{channel}_check_valve_{index}_bore"),
        4.2 / 2.0,
        30.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0);
    body - bore
}

fn disposable_imaging_counting_cassette_dock() -> Part {
    let dock = centered_cube(
        "density_viability_disposable_cassette_dock_plate",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let cassette_slot = centered_cube(
        "density_viability_disposable_cassette_active_counting_slot",
        IMAGING_CASSETTE_X + 18.0,
        IMAGING_CASSETTE_Y + 16.0,
        IMAGING_CASSETTE_Z + 10.0,
    )
    .translate(-72.0, 12.0, CASSETTE_DOCK_Z / 2.0 - 6.0);
    let imaging_window = centered_cube(
        "density_viability_disposable_cassette_optical_window",
        58.0,
        34.0,
        CASSETTE_DOCK_Z + 4.0,
    )
    .translate(-72.0, 12.0, 0.0);

    let mut datum_pins = Part::empty("density_viability_disposable_cassette_datum_pins");
    for (i, (x, y)) in cassette_datum_points().into_iter().enumerate() {
        let pin = centered_cylinder(
            format!("density_viability_disposable_cassette_datum_pin_{i}"),
            5.0 / 2.0,
            12.0,
            24,
        )
        .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 6.0);
        let screw = centered_cylinder(
            format!("density_viability_disposable_cassette_datum_pin_{i}_fastener"),
            2.8 / 2.0,
            14.0,
            18,
        )
        .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 6.0);
        datum_pins = datum_pins + (pin - screw);
    }

    let camera_bridge = centered_cube(
        "density_viability_disposable_cassette_camera_bridge",
        148.0,
        26.0,
        24.0,
    )
    .translate(-72.0, 94.0, CAMERA_CLEARANCE_Z);
    let left_post = centered_cube(
        "density_viability_disposable_cassette_camera_left_post",
        18.0,
        24.0,
        CAMERA_CLEARANCE_Z - CASSETTE_DOCK_Z / 2.0,
    )
    .translate(
        -148.0,
        94.0,
        (CAMERA_CLEARANCE_Z + CASSETTE_DOCK_Z / 2.0) / 2.0,
    );
    let right_post = centered_cube(
        "density_viability_disposable_cassette_camera_right_post",
        18.0,
        24.0,
        CAMERA_CLEARANCE_Z - CASSETTE_DOCK_Z / 2.0,
    )
    .translate(
        4.0,
        94.0,
        (CAMERA_CLEARANCE_Z + CASSETTE_DOCK_Z / 2.0) / 2.0,
    );

    let mut illumination = Part::empty("density_viability_disposable_cassette_illumination");
    for i in 0..ILLUMINATION_BARS {
        let y = if i == 0 { -38.0 } else { 62.0 };
        let bar = centered_cube(
            format!("density_viability_disposable_cassette_illumination_bar_{i}"),
            132.0,
            10.0,
            14.0,
        )
        .translate(-72.0, y, CASSETTE_DOCK_Z / 2.0 + 12.0);
        illumination = illumination + bar;
    }

    let magazine = cassette_magazine();
    let eject_chute = centered_cube(
        "density_viability_disposable_cassette_used_chute_to_quarantine",
        34.0,
        212.0,
        22.0,
    )
    .translate(
        CASSETTE_DOCK_X / 2.0 - 36.0,
        -18.0,
        CASSETTE_DOCK_Z / 2.0 + 8.0,
    );

    dock + datum_pins
        + camera_bridge
        + left_post
        + right_post
        + illumination
        + magazine
        + eject_chute
        - cassette_slot
        - imaging_window
}

fn cassette_magazine() -> Part {
    let magazine = centered_cube(
        "density_viability_disposable_cassette_clean_magazine",
        112.0,
        226.0,
        38.0,
    )
    .translate(104.0, 8.0, CASSETTE_DOCK_Z / 2.0 + 19.0);
    let mut slots = Part::empty("density_viability_disposable_cassette_clean_magazine_slots");
    for i in 0..DISPOSABLE_CASSETTES {
        let row = i / 2;
        let col = i % 2;
        let x = 78.0 + col as f64 * 52.0;
        let y = -92.0 + row as f64 * 34.0;
        let slot = centered_cube(
            format!("density_viability_disposable_cassette_magazine_slot_{i}"),
            IMAGING_CASSETTE_X / 2.0,
            IMAGING_CASSETTE_Y / 2.4,
            42.0,
        )
        .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 19.0);
        let thumb = centered_cube(
            format!("density_viability_disposable_cassette_magazine_slot_{i}_pick_relief"),
            18.0,
            10.0,
            44.0,
        )
        .translate(x + 20.0, y, CASSETTE_DOCK_Z / 2.0 + 19.0);
        slots = slots + slot + thumb;
    }
    magazine - slots
}

fn viability_reagent_isolation_pod() -> Part {
    let tray = centered_cube(
        "density_viability_reagent_isolation_secondary_tray",
        REAGENT_POD_X,
        REAGENT_POD_Y,
        REAGENT_POD_Z,
    );
    let secondary_basin = centered_cube(
        "density_viability_reagent_isolation_secondary_basin",
        REAGENT_POD_X - 34.0,
        REAGENT_POD_Y - 30.0,
        18.0,
    )
    .translate(0.0, 0.0, REAGENT_POD_Z / 2.0 - 9.0);
    let air_gap_land = centered_cube(
        "density_viability_reagent_isolation_air_gap_land",
        REAGENT_AIR_GAP_MM,
        REAGENT_POD_Y - 48.0,
        34.0,
    )
    .translate(18.0, 0.0, REAGENT_POD_Z / 2.0 + 17.0);

    let mut cartridges = Part::empty("density_viability_reagent_isolation_cartridges");
    for i in 0..VIABILITY_REAGENT_CARTRIDGES {
        let y = -38.0 + i as f64 * 76.0;
        let well = centered_cube(
            format!("density_viability_reagent_cartridge_well_{i}"),
            72.0,
            46.0,
            28.0,
        )
        .translate(-68.0, y, REAGENT_POD_Z / 2.0 - 5.0);
        let keyed_cap = centered_cube(
            format!("density_viability_reagent_cartridge_{i}_keyed_cap_land"),
            86.0,
            58.0,
            10.0,
        )
        .translate(-68.0, y, REAGENT_POD_Z / 2.0 + 5.0);
        let seal_witness = centered_cylinder(
            format!("density_viability_reagent_cartridge_{i}_seal_witness"),
            16.0 / 2.0,
            8.0,
            26,
        )
        .translate(-118.0, y, REAGENT_POD_Z / 2.0 + 8.0);
        cartridges = cartridges + keyed_cap + seal_witness - well;
    }

    let mut valves = Part::empty("density_viability_reagent_isolation_valve_bank");
    for i in 0..REAGENT_ISOLATION_VALVES {
        let y = -60.0 + i as f64 * 40.0;
        let block = centered_cube(
            format!("density_viability_reagent_isolation_valve_{i}"),
            34.0,
            24.0,
            20.0,
        )
        .translate(72.0, y, REAGENT_POD_Z / 2.0 + 10.0);
        let pinch = centered_cube(
            format!("density_viability_reagent_isolation_valve_{i}_pinch_window"),
            15.0,
            10.0,
            13.0,
        )
        .translate(72.0, y, REAGENT_POD_Z / 2.0 + 10.0);
        valves = valves + (block - pinch);
    }

    let guarded_junction = centered_cylinder(
        "density_viability_reagent_isolation_guarded_mix_junction",
        30.0 / 2.0,
        24.0,
        32,
    )
    .translate(110.0, 0.0, REAGENT_POD_Z / 2.0 + 12.0);
    let junction_bore = centered_cylinder(
        "density_viability_reagent_isolation_guarded_mix_junction_bore",
        6.0 / 2.0,
        26.0,
        24,
    )
    .translate(110.0, 0.0, REAGENT_POD_Z / 2.0 + 12.0);
    let barcode_land = centered_cube(
        "density_viability_reagent_isolation_reagent_lot_barcode_land",
        92.0,
        24.0,
        6.0,
    )
    .translate(0.0, -REAGENT_POD_Y / 2.0 + 22.0, REAGENT_POD_Z / 2.0 + 3.0);

    tray + air_gap_land + cartridges + valves + barcode_land + (guarded_junction - junction_bore)
        - secondary_basin
}

fn waste_quarantine_pod() -> Part {
    let vault = centered_cube(
        "density_viability_waste_quarantine_locking_vault",
        WASTE_POD_X,
        WASTE_POD_Y,
        WASTE_POD_Z,
    );
    let sump = centered_cube(
        "density_viability_waste_quarantine_secondary_sump",
        WASTE_POD_X - 34.0,
        WASTE_POD_Y - 26.0,
        22.0,
    )
    .translate(0.0, 0.0, WASTE_POD_Z / 2.0 - 11.0);

    let mut bottles = Part::empty("density_viability_waste_quarantine_bottle_sockets");
    for i in 0..WASTE_BOTTLES {
        let x = -62.0 + i as f64 * 124.0;
        let socket = centered_cylinder(
            format!("density_viability_waste_quarantine_bottle_socket_{i}"),
            46.0 / 2.0,
            42.0,
            40,
        )
        .translate(x, 18.0, WASTE_POD_Z / 2.0 - 6.0);
        let lock_ring = centered_cylinder(
            format!("density_viability_waste_quarantine_bottle_lock_ring_{i}"),
            62.0 / 2.0,
            10.0,
            40,
        )
        .translate(x, 18.0, WASTE_POD_Z / 2.0 + 5.0);
        let ring_opening = centered_cylinder(
            format!("density_viability_waste_quarantine_bottle_lock_ring_{i}_opening"),
            42.0 / 2.0,
            12.0,
            36,
        )
        .translate(x, 18.0, WASTE_POD_Z / 2.0 + 5.0);
        bottles = bottles + (lock_ring - ring_opening) - socket;
    }

    let cassette_bin = centered_cube(
        "density_viability_waste_quarantine_used_cassette_bin",
        252.0,
        48.0,
        40.0,
    )
    .translate(0.0, -48.0, WASTE_POD_Z / 2.0 + 4.0);
    let mut cassette_slots = Part::empty("density_viability_waste_quarantine_used_cassette_slots");
    for i in 0..USED_CASSETTE_QUARANTINE_SLOTS {
        let x = -115.0 + i as f64 * 20.8;
        let slot = centered_cube(
            format!("density_viability_waste_quarantine_used_cassette_slot_{i}"),
            12.0,
            38.0,
            42.0,
        )
        .translate(x, -48.0, WASTE_POD_Z / 2.0 + 4.0);
        cassette_slots = cassette_slots + slot;
    }

    let mut locks = Part::empty("density_viability_waste_quarantine_lock_witnesses");
    for i in 0..WASTE_LOCKS {
        let x = -76.0 + i as f64 * 76.0;
        let lock = centered_cube(
            format!("density_viability_waste_quarantine_lock_witness_{i}"),
            44.0,
            18.0,
            12.0,
        )
        .translate(x, WASTE_POD_Y / 2.0 - 18.0, WASTE_POD_Z / 2.0 + 6.0);
        locks = locks + lock;
    }

    let vent_filter = centered_cylinder(
        "density_viability_waste_quarantine_hydrophobic_vent_filter",
        24.0 / 2.0,
        12.0,
        32,
    )
    .translate(
        WASTE_POD_X / 2.0 - 34.0,
        WASTE_POD_Y / 2.0 - 32.0,
        WASTE_POD_Z / 2.0 + 6.0,
    );
    let waste_barcode = centered_cube(
        "density_viability_waste_quarantine_barcode_land",
        92.0,
        24.0,
        6.0,
    )
    .translate(
        -WASTE_POD_X / 2.0 + 62.0,
        WASTE_POD_Y / 2.0 - 26.0,
        WASTE_POD_Z / 2.0 + 3.0,
    );

    vault + (cassette_bin - cassette_slots) + locks + vent_filter + waste_barcode - sump + bottles
}

fn barcode_lot_traceability_panel() -> Part {
    let panel = centered_cube(
        "density_viability_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let scanner_window = centered_cube(
        "density_viability_traceability_scanner_window_relief",
        170.0,
        34.0,
        TRACE_PANEL_Z + 4.0,
    )
    .translate(TRACE_PANEL_X / 2.0 - 110.0, 0.0, 0.0);

    let mut barcodes = Part::empty("density_viability_traceability_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 7;
        let col = i % 7;
        let x = -330.0 + col as f64 * BARCODE_PITCH_X;
        let y = -20.0 + row as f64 * 40.0;
        let land = centered_cube(
            format!("density_viability_traceability_barcode_land_{i}"),
            54.0,
            20.0,
            5.0,
        )
        .translate(x, y, TRACE_PANEL_Z / 2.0 + 2.5);
        let corner = centered_cylinder(
            format!("density_viability_traceability_barcode_land_{i}_fiducial"),
            4.0 / 2.0,
            6.0,
            18,
        )
        .translate(x - 20.0, y + 6.0, TRACE_PANEL_Z / 2.0 + 3.0);
        barcodes = barcodes + land + corner;
    }

    let mut rfid = Part::empty("density_viability_traceability_rfid_lands");
    for i in 0..RFID_LANDS {
        let x = 186.0 + i as f64 * 64.0;
        let pad = centered_cube(
            format!("density_viability_traceability_rfid_land_{i}"),
            44.0,
            30.0,
            5.0,
        )
        .translate(x, -18.0, TRACE_PANEL_Z / 2.0 + 2.5);
        rfid = rfid + pad;
    }

    let mut lot_cards = Part::empty("density_viability_traceability_lot_card_slots");
    for i in 0..LOT_CARD_SLOTS {
        let x = -330.0 + i as f64 * 110.0;
        let slot = centered_cube(
            format!("density_viability_traceability_lot_card_slot_{i}"),
            86.0,
            8.0,
            12.0,
        )
        .translate(x, TRACE_PANEL_Y / 2.0 - 16.0, TRACE_PANEL_Z / 2.0);
        lot_cards = lot_cards + slot;
    }

    let run_record_spine = centered_cube(
        "density_viability_traceability_electronic_batch_record_spine",
        TRACE_PANEL_X - 80.0,
        6.0,
        10.0,
    )
    .translate(0.0, -TRACE_PANEL_Y / 2.0 + 12.0, TRACE_PANEL_Z / 2.0 + 5.0);

    panel + barcodes + rfid + run_record_spine - scanner_window - lot_cards
}

fn automated_release_gate_matrix() -> Part {
    let base = centered_cube(
        "density_viability_release_gate_matrix_base",
        RELEASE_GATE_X,
        RELEASE_GATE_Y,
        RELEASE_GATE_Z,
    );
    let mut lanes = Part::empty("density_viability_release_gate_lanes");
    for i in 0..RELEASE_LANES {
        let x = release_lane_x(i);
        let lane = centered_cube(
            format!("density_viability_release_gate_lane_{i}"),
            68.0,
            RELEASE_GATE_Y - 28.0,
            12.0,
        )
        .translate(x, 0.0, RELEASE_GATE_Z / 2.0 + 6.0);
        let slot = centered_cube(
            format!("density_viability_release_gate_lane_{i}_status_token_slot"),
            42.0,
            92.0,
            14.0,
        )
        .translate(x, 0.0, RELEASE_GATE_Z / 2.0 + 7.0);
        lanes = lanes + (lane - slot);
    }

    let mut solenoids = Part::empty("density_viability_release_gate_solenoid_interlocks");
    for i in 0..RELEASE_GATE_SOLENOIDS {
        let row = i / 3;
        let col = i % 3;
        let x = -92.0 + col as f64 * 92.0;
        let y = -48.0 + row as f64 * 96.0;
        let body = centered_cube(
            format!("density_viability_release_gate_solenoid_{i}"),
            38.0,
            20.0,
            20.0,
        )
        .translate(x, y, RELEASE_GATE_Z / 2.0 + 10.0);
        let plunger = centered_cylinder(
            format!("density_viability_release_gate_solenoid_{i}_plunger_clearance"),
            5.0 / 2.0,
            32.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, RELEASE_GATE_Z / 2.0 + 10.0);
        solenoids = solenoids + (body - plunger);
    }

    let mut inputs = Part::empty("density_viability_release_gate_decision_inputs");
    for i in 0..RELEASE_DECISION_INPUTS {
        let x = -132.0 + i as f64 * 66.0;
        let input = centered_cube(
            format!("density_viability_release_gate_decision_input_{i}"),
            44.0,
            18.0,
            8.0,
        )
        .translate(x, RELEASE_GATE_Y / 2.0 - 18.0, RELEASE_GATE_Z / 2.0 + 4.0);
        inputs = inputs + input;
    }

    let diverter = centered_cylinder(
        "density_viability_release_gate_closed_loop_diverter_axis",
        30.0 / 2.0,
        36.0,
        32,
    )
    .translate(
        0.0,
        -RELEASE_GATE_Y / 2.0 + 26.0,
        RELEASE_GATE_Z / 2.0 + 18.0,
    );
    let diverter_bore = centered_cylinder(
        "density_viability_release_gate_diverter_axis_bore",
        12.0 / 2.0,
        40.0,
        28,
    )
    .translate(
        0.0,
        -RELEASE_GATE_Y / 2.0 + 26.0,
        RELEASE_GATE_Z / 2.0 + 18.0,
    );

    base + lanes + solenoids + inputs + (diverter - diverter_bore)
}

fn robot_service_keepouts() -> Part {
    let cassette_pick = centered_cube(
        "density_viability_keepout_robot_cassette_pick_volume",
        CASSETTE_DOCK_X + 70.0,
        CASSETTE_DOCK_Y + 46.0,
        CASSETTE_PICK_CLEARANCE_Z,
    )
    .translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1,
        BASE_Z + CASSETTE_PICK_CLEARANCE_Z / 2.0 + 52.0,
    );
    let front_service = centered_cube(
        "density_viability_keepout_front_service_sweep",
        STATION_X - 120.0,
        FRONT_SERVICE_CLEARANCE,
        20.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 10.0,
    );
    let rear_tube = centered_cube(
        "density_viability_keepout_rear_closed_tube_service",
        STATION_X - 180.0,
        REAR_TUBE_SERVICE_CLEARANCE,
        20.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_TUBE_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 10.0,
    );
    let vertical_lift = centered_cube(
        "density_viability_keepout_station_vertical_lift_volume",
        STATION_X - 170.0,
        STATION_Y - 150.0,
        16.0,
    )
    .translate(0.0, 0.0, BASE_Z + ROBOT_KEEP_OUT_Z);

    cassette_pick + front_service + rear_tube + vertical_lift
}

fn closed_sample_loop_routes() -> Part {
    let nodes = closed_loop_nodes();
    let mut routes = Part::empty("density_viability_closed_sample_loop_route_placeholders");
    for i in 0..nodes.len() - 1 {
        let (name, start) = nodes[i];
        let (_, end) = nodes[i + 1];
        routes = routes + tube_between_xy(name, start, end, ROUTE_Z, ROUTE_TUBE_D);
    }

    let reagent_branch = tube_between_xy(
        "density_viability_reagent_branch_to_guarded_mix_junction",
        (REAGENT_POD_POS.0 - 20.0, REAGENT_POD_POS.1 + 78.0),
        (CASSETTE_DOCK_POS.0 + 18.0, CASSETTE_DOCK_POS.1 - 88.0),
        ROUTE_Z + 10.0,
        SAMPLE_BRANCH_TUBE_D,
    );
    let waste_branch = tube_between_xy(
        "density_viability_waste_branch_to_quarantine",
        (CASSETTE_DOCK_POS.0 + 155.0, CASSETTE_DOCK_POS.1 - 58.0),
        (WASTE_POD_POS.0 + 110.0, WASTE_POD_POS.1 + 58.0),
        ROUTE_Z - 6.0,
        SAMPLE_BRANCH_TUBE_D,
    );
    let release_signal_harness = tube_between_xy(
        "density_viability_release_gate_signal_harness",
        (CASSETTE_DOCK_POS.0 - 20.0, CASSETTE_DOCK_POS.1 - 138.0),
        (RELEASE_GATE_POS.0, RELEASE_GATE_POS.1 + 64.0),
        ROUTE_Z + 22.0,
        4.0,
    );

    routes + reagent_branch + waste_branch + release_signal_harness
}

fn tube_between_xy(name: &str, start: (f64, f64), end: (f64, f64), z: f64, diameter: f64) -> Part {
    let elbow = (end.0, start.1);
    tube_run_x(&format!("{name}_x"), start.0, elbow.0, start.1, z, diameter)
        + tube_run_y(&format!("{name}_y"), elbow.0, start.1, end.1, z, diameter)
}

fn tube_run_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    let len = (x_b - x_a).abs().max(0.1);
    centered_cylinder(format!("{name}_tube"), diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x_a + x_b) / 2.0, y, z)
}

fn tube_run_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    let len = (y_b - y_a).abs().max(0.1);
    centered_cylinder(format!("{name}_tube"), diameter / 2.0, len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y_a + y_b) / 2.0, z)
}

fn tube_cut_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    tube_run_x(name, x_a, x_b, y, z, diameter)
}

fn tube_cut_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    tube_run_y(name, x, y_a, y_b, z, diameter)
}

fn station_mount_points() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-STATION_X / 2.0 + 54.0, 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
}

fn connector_port_x(index: usize) -> f64 {
    let col = index % (STERILE_CONNECTOR_PORTS / 2);
    -210.0 + col as f64 * 140.0
}

fn valve_x(index: usize) -> f64 {
    -226.0 + index as f64 * 64.0
}

fn metering_channel_x(index: usize) -> f64 {
    -((METERING_CHANNELS as f64 - 1.0) * METERING_PITCH_X) / 2.0 + index as f64 * METERING_PITCH_X
}

fn cassette_datum_points() -> [(f64, f64); CASSETTE_DATUM_PINS] {
    [
        (-132.0, -92.0),
        (-12.0, -92.0),
        (-132.0, 116.0),
        (-12.0, 116.0),
    ]
}

fn release_lane_x(index: usize) -> f64 {
    -((RELEASE_LANES as f64 - 1.0) * RELEASE_LANE_PITCH_X) / 2.0
        + index as f64 * RELEASE_LANE_PITCH_X
}

fn closed_loop_nodes() -> [(&'static str, (f64, f64)); CLOSED_LOOP_ROUTE_POINTS] {
    [
        (
            "density_viability_loop_upstream_supply_to_mixer",
            (SAMPLE_LOOP_POS.0 - 260.0, SAMPLE_LOOP_POS.1 - 30.0),
        ),
        (
            "density_viability_loop_mixer_to_metering",
            (MIXER_POS.0 + 122.0, MIXER_POS.1 + 24.0),
        ),
        (
            "density_viability_loop_metering_to_cassette",
            (METERING_POS.0 + 112.0, METERING_POS.1 + 44.0),
        ),
        (
            "density_viability_loop_cassette_to_return_sensor",
            (CASSETTE_DOCK_POS.0 - 92.0, CASSETTE_DOCK_POS.1 + 64.0),
        ),
        (
            "density_viability_loop_return_sensor_to_manifold",
            (SAMPLE_LOOP_POS.0 + 238.0, SAMPLE_LOOP_POS.1 - 26.0),
        ),
        (
            "density_viability_loop_manifold_return_to_upstream",
            (SAMPLE_LOOP_POS.0 - 260.0, SAMPLE_LOOP_POS.1 - 30.0),
        ),
    ]
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        self.x - self.w / 2.0 >= -deck_x / 2.0
            && self.x + self.w / 2.0 <= deck_x / 2.0
            && self.y - self.h / 2.0 >= -deck_y / 2.0
            && self.y + self.h / 2.0 <= deck_y / 2.0
    }

    fn overlaps(self, other: Rect) -> bool {
        let ax0 = self.x - self.w / 2.0;
        let ax1 = self.x + self.w / 2.0;
        let ay0 = self.y - self.h / 2.0;
        let ay1 = self.y + self.h / 2.0;
        let bx0 = other.x - other.w / 2.0;
        let bx1 = other.x + other.w / 2.0;
        let by0 = other.y - other.h / 2.0;
        let by1 = other.y + other.h / 2.0;

        ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
    }
}

fn clearance_between(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    let x_gap = if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    };
    let y_gap = if ay1 < by0 {
        by0 - ay1
    } else if by1 < ay0 {
        ay0 - by1
    } else {
        0.0
    };

    if x_gap > 0.0 && y_gap > 0.0 {
        x_gap.hypot(y_gap)
    } else {
        x_gap.max(y_gap)
    }
}

fn sample_loop_rect() -> Rect {
    Rect {
        name: "closed_sample_loop_manifold",
        x: SAMPLE_LOOP_POS.0,
        y: SAMPLE_LOOP_POS.1,
        w: SAMPLE_LOOP_X,
        h: SAMPLE_LOOP_Y,
    }
}

fn mixer_rect() -> Rect {
    Rect {
        name: "gentle_mixing_module",
        x: MIXER_POS.0,
        y: MIXER_POS.1,
        w: MIXER_X,
        h: MIXER_Y,
    }
}

fn metering_rect() -> Rect {
    Rect {
        name: "sample_metering_bank",
        x: METERING_POS.0,
        y: METERING_POS.1,
        w: METERING_X,
        h: METERING_Y,
    }
}

fn cassette_rect() -> Rect {
    Rect {
        name: "disposable_imaging_counting_cassette_dock",
        x: CASSETTE_DOCK_POS.0,
        y: CASSETTE_DOCK_POS.1,
        w: CASSETTE_DOCK_X,
        h: CASSETTE_DOCK_Y,
    }
}

fn reagent_rect() -> Rect {
    Rect {
        name: "viability_reagent_isolation_pod",
        x: REAGENT_POD_POS.0,
        y: REAGENT_POD_POS.1,
        w: REAGENT_POD_X,
        h: REAGENT_POD_Y,
    }
}

fn waste_rect() -> Rect {
    Rect {
        name: "waste_quarantine_pod",
        x: WASTE_POD_POS.0,
        y: WASTE_POD_POS.1,
        w: WASTE_POD_X,
        h: WASTE_POD_Y,
    }
}

fn trace_rect() -> Rect {
    Rect {
        name: "barcode_lot_traceability_panel",
        x: TRACE_PANEL_POS.0,
        y: TRACE_PANEL_POS.1,
        w: TRACE_PANEL_X,
        h: TRACE_PANEL_Y,
    }
}

fn release_gate_rect() -> Rect {
    Rect {
        name: "automated_release_gate_matrix",
        x: RELEASE_GATE_POS.0,
        y: RELEASE_GATE_POS.1,
        w: RELEASE_GATE_X,
        h: RELEASE_GATE_Y,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_cell_density_viability_sampling_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn major_modules_fit_on_closed_station_without_primary_overlap() {
        assert_layout_constraints();
        assert!(STATION_X >= 1200.0);
        assert!(STATION_Y >= 800.0);
        assert!(trace_rect().y + trace_rect().h / 2.0 <= STATION_Y / 2.0);
        assert!(waste_rect().y - waste_rect().h / 2.0 >= -STATION_Y / 2.0);
    }

    #[test]
    fn closed_loop_has_return_path_and_interlocked_ports() {
        let nodes = closed_loop_nodes();
        assert_eq!(nodes.len(), CLOSED_LOOP_ROUTE_POINTS);
        assert_eq!(nodes[0].1, nodes[nodes.len() - 1].1);
        assert_eq!(LOOP_INLET_PORTS, LOOP_RETURN_PORTS);
        assert!(STERILE_CONNECTOR_PORTS >= LOOP_INLET_PORTS + LOOP_RETURN_PORTS + LOOP_SENSOR_TAPS);
        assert!(LOOP_INTERLOCK_VALVES >= STERILE_CONNECTOR_PORTS);
        assert!(LOOP_BORE_D > SAMPLE_BRANCH_TUBE_D);
    }

    #[test]
    fn gentle_mixing_and_metering_encode_low_shear_sampling() {
        assert_eq!(MIXER_ROLLERS, 2);
        assert!(MAX_MIXER_RPM <= 10.0);
        assert!(ROCK_ANGLE_LIMIT_DEG <= 7.0);
        assert_eq!(METERING_CHANNELS, 4);
        assert!(METERING_LOOP_UL <= 100.0);
        assert_eq!(
            METERING_CHANNELS * CHECK_VALVES_PER_CHANNEL,
            METERING_CHANNELS * 2
        );
        assert!(metering_channel_x(0).abs() < METERING_X / 2.0 - 24.0);
        assert!(metering_channel_x(METERING_CHANNELS - 1).abs() < METERING_X / 2.0 - 24.0);
    }

    #[test]
    fn disposable_cassette_inventory_and_traceability_cover_sampling_run() {
        assert!(DISPOSABLE_CASSETTES >= METERING_CHANNELS * 2);
        assert_eq!(CASSETTE_DATUM_PINS, cassette_datum_points().len());
        assert!(CAMERA_CLEARANCE_Z > CASSETTE_DOCK_Z + 100.0);
        assert_eq!(ILLUMINATION_BARS, 2);
        assert!(BARCODE_LANDS >= DISPOSABLE_CASSETTES);
        assert!(RFID_LANDS >= VIABILITY_REAGENT_CARTRIDGES + WASTE_BOTTLES);
        assert!(LOT_CARD_SLOTS >= RELEASE_DECISION_INPUTS);
    }

    #[test]
    fn reagent_and_waste_are_isolated_before_release_decision() {
        assert_eq!(VIABILITY_REAGENT_CARTRIDGES, REAGENT_SEAL_WITNESSES);
        assert!(REAGENT_ISOLATION_VALVES >= VIABILITY_REAGENT_CARTRIDGES * 2);
        assert!(REAGENT_AIR_GAP_MM >= 20.0);
        assert!(clearance_between(reagent_rect(), waste_rect()) >= BIOBURDEN_SEGREGATION_GAP);
        assert_eq!(WASTE_BRANCHES, WASTE_BOTTLES);
        assert!(USED_CASSETTE_QUARANTINE_SLOTS >= DISPOSABLE_CASSETTES);
        assert!(WASTE_LOCKS >= RELEASE_LANES);
    }

    #[test]
    fn automated_release_gates_are_pre_seeding_and_fail_closed() {
        assert_eq!(RELEASE_LANES, 3);
        assert_eq!(RELEASE_GATE_SOLENOIDS, RELEASE_LANES * 2);
        assert!(RELEASE_DECISION_INPUTS >= 5);
        assert!(release_lane_x(0) < release_lane_x(1));
        assert!(release_lane_x(1) < release_lane_x(2));
        assert!(release_gate_rect().y + release_gate_rect().h / 2.0 < cassette_rect().y);
        assert!(CASSETTE_PICK_CLEARANCE_Z > CASSETTE_DOCK_Z + 70.0);
    }
}
