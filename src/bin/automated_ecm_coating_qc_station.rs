use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Automated ECM/coating QC station for the 20-chip cassette workflow.
//
// Intent:
// - Provide a robot-serviceable station for coating cartridge docking,
//   controlled dispense/recirculation by cassette row, timed dwell/wetness
//   witness pockets, fluorescent coating witness slide/coupon handling, and
//   traceability lands.
// - Keep cassette datum geometry compatible with the 4x5 Rev C culture cassette
//   used by the surrounding LaminarForge automation modules.
// - Model mechanical envelopes and interfaces only. ECM chemistry, dwell timing,
//   fluorescence acceptance thresholds, and sterilization validation remain
//   process controls outside this CAD placeholder.

const OUTPUTS: [&str; 11] = [
    "output/automated_ecm_coating_qc_station_baseplate.stl",
    "output/automated_ecm_coating_qc_station_cassette_datum_nest.stl",
    "output/automated_ecm_coating_qc_station_coating_cartridge_interface.stl",
    "output/automated_ecm_coating_qc_station_dispense_recirculation_lanes.stl",
    "output/automated_ecm_coating_qc_station_timed_wetness_witness_pockets.stl",
    "output/automated_ecm_coating_qc_station_fluorescent_witness_slide_carrier.stl",
    "output/automated_ecm_coating_qc_station_row_valve_prime_ports.stl",
    "output/automated_ecm_coating_qc_station_bubble_degas_waste_path.stl",
    "output/automated_ecm_coating_qc_station_barcode_lot_lands.stl",
    "output/automated_ecm_coating_qc_station_robot_service_keepouts.stl",
    "output/automated_ecm_coating_qc_station_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const POSITION_COUNT: usize = COLS * ROWS;

const COATING_CARTRIDGES: usize = 4; // ECM A, ECM B, blocker/rinse, fluorescent tracer.
const ROW_DISPENSE_LANES: usize = ROWS;
const ROW_RECIRCULATION_LANES: usize = ROWS;
const PRIME_LANES: usize = ROWS;
const VALVE_PORTS_PER_ROW: usize = 3; // dispense, recirculation return, prime/waste.
const ROW_VALVE_PORTS: usize = ROWS * VALVE_PORTS_PER_ROW;
const WETNESS_WITNESS_POCKETS: usize = POSITION_COUNT;
const FLUORESCENT_WITNESS_SLOTS: usize = ROWS + 1; // one row coupon plus one blank/reference slide.
const BARCODE_LOT_LANDS: usize = COATING_CARTRIDGES + FLUORESCENT_WITNESS_SLOTS + 2;

const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;
const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const MODULE_X: f64 = 1180.0;
const MODULE_Y: f64 = 760.0;
const BASE_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 22.0;
const SOCKET_DEPTH: f64 = 5.0;

const NEST_X: f64 = CASSETTE_X + 120.0;
const NEST_Y: f64 = CASSETTE_Y + 116.0;
const NEST_Z: f64 = 16.0;
const CASSETTE_CENTER_X: f64 = 215.0;
const CASSETTE_CENTER_Y: f64 = 0.0;
const CASSETTE_DATUM_RAIL_Z: f64 = 28.0;
const ROW_TRUNK_D: f64 = 6.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.6;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const RECIRC_BORE_D: f64 = 6.8;
const PRIME_BORE_D: f64 = 4.2;
const WASTE_BORE_D: f64 = 7.0;

const CARTRIDGE_BAY_X: f64 = 348.0;
const CARTRIDGE_BAY_Y: f64 = 176.0;
const CARTRIDGE_BAY_Z: f64 = 34.0;
const CARTRIDGE_CENTER_X: f64 = -398.0;
const CARTRIDGE_CENTER_Y: f64 = 250.0;
const CARTRIDGE_SOCKET_X: f64 = 58.0;
const CARTRIDGE_SOCKET_Y: f64 = 112.0;
const CARTRIDGE_SOCKET_Z: f64 = 28.0;
const CARTRIDGE_PITCH_X: f64 = 76.0;

const MANIFOLD_X: f64 = 390.0;
const MANIFOLD_Y: f64 = 336.0;
const MANIFOLD_Z: f64 = 42.0;
const MANIFOLD_CENTER_X: f64 = -374.0;
const MANIFOLD_CENTER_Y: f64 = 18.0;
const LANE_STEP_Z: f64 = 10.0;

const VALVE_BANK_X: f64 = 112.0;
const VALVE_BANK_Y: f64 = CASSETTE_Y + 46.0;
const VALVE_BANK_Z: f64 = 56.0;
const VALVE_BANK_CENTER_X: f64 = -205.0;
const VALVE_BANK_CENTER_Y: f64 = CASSETTE_CENTER_Y;
const VALVE_PORT_PITCH_X: f64 = 30.0;

const WITNESS_PLATE_X: f64 = CASSETTE_X + 36.0;
const WITNESS_PLATE_Y: f64 = CASSETTE_Y + 34.0;
const WITNESS_PLATE_Z: f64 = 10.0;
const WITNESS_POCKET_D: f64 = 14.0;
const WITNESS_POCKET_DEPTH: f64 = 7.0;

const SLIDE_CARRIER_X: f64 = 504.0;
const SLIDE_CARRIER_Y: f64 = 72.0;
const SLIDE_CARRIER_Z: f64 = 28.0;
const SLIDE_CENTER_X: f64 = 205.0;
const SLIDE_CENTER_Y: f64 = -335.0;
const SLIDE_SLOT_X: f64 = 70.0;
const SLIDE_SLOT_Y: f64 = 28.0;
const SLIDE_SLOT_DEPTH: f64 = 20.0;
const SLIDE_PITCH_X: f64 = 76.0;

const DEGAS_WASTE_X: f64 = 342.0;
const DEGAS_WASTE_Y: f64 = 166.0;
const DEGAS_WASTE_Z: f64 = 34.0;
const DEGAS_CENTER_X: f64 = -414.0;
const DEGAS_CENTER_Y: f64 = -250.0;
const BUBBLE_TOWER_D: f64 = 46.0;
const BUBBLE_TOWER_Z: f64 = 92.0;

const KEEP_OUT_Z: f64 = 148.0;
const CASSETTE_ROBOT_CLEARANCE_X: f64 = NEST_X + 112.0;
const CASSETTE_ROBOT_CLEARANCE_Y: f64 = NEST_Y + 96.0;
const FLUID_SERVICE_CLEARANCE_Z: f64 = 170.0;
const FRONT_SERVICE_CLEARANCE_Y: f64 = 96.0;
const WRIST_SWEEP_CLEARANCE_X: f64 = 760.0;
const WRIST_SWEEP_CLEARANCE_Y: f64 = 128.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let baseplate = baseplate();
    export(OUTPUTS[0], &baseplate);

    let cassette = cassette_datum_nest();
    export(OUTPUTS[1], &cassette);

    let cartridges = coating_cartridge_interface();
    export(OUTPUTS[2], &cartridges);

    let lanes = dispense_recirculation_lanes();
    export(OUTPUTS[3], &lanes);

    let wetness = timed_wetness_witness_pockets();
    export(OUTPUTS[4], &wetness);

    let slides = fluorescent_witness_slide_carrier();
    export(OUTPUTS[5], &slides);

    let valves = row_valve_prime_ports();
    export(OUTPUTS[6], &valves);

    let degas = bubble_degas_waste_path();
    export(OUTPUTS[7], &degas);

    let labels = barcode_lot_lands();
    export(OUTPUTS[8], &labels);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = baseplate
        + cassette.translate(
            CASSETTE_CENTER_X,
            CASSETTE_CENTER_Y,
            BASE_Z / 2.0 + NEST_Z / 2.0,
        )
        + cartridges.translate(
            CARTRIDGE_CENTER_X,
            CARTRIDGE_CENTER_Y,
            BASE_Z / 2.0 + CARTRIDGE_BAY_Z / 2.0,
        )
        + lanes.translate(
            MANIFOLD_CENTER_X,
            MANIFOLD_CENTER_Y,
            BASE_Z / 2.0 + MANIFOLD_Z / 2.0,
        )
        + wetness.translate(
            CASSETTE_CENTER_X,
            CASSETTE_CENTER_Y,
            BASE_Z / 2.0 + NEST_Z + WITNESS_PLATE_Z / 2.0 + 1.5,
        )
        + slides.translate(
            SLIDE_CENTER_X,
            SLIDE_CENTER_Y,
            BASE_Z / 2.0 + SLIDE_CARRIER_Z / 2.0,
        )
        + valves.translate(
            VALVE_BANK_CENTER_X,
            VALVE_BANK_CENTER_Y,
            BASE_Z / 2.0 + VALVE_BANK_Z / 2.0,
        )
        + degas.translate(
            DEGAS_CENTER_X,
            DEGAS_CENTER_Y,
            BASE_Z / 2.0 + DEGAS_WASTE_Z / 2.0,
        )
        + labels
        + keepouts;

    export(OUTPUTS[10], &assembly);

    println!();
    println!("Automated ECM/coating QC station:");
    println!(
        "  Deck footprint:             {MODULE_X:.0}mm x {MODULE_Y:.0}mm x {BASE_Z:.0}mm base"
    );
    println!(
        "  Cassette datum:             {COLS}x{ROWS} Rev C cassette, {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm envelope, {PITCH_X:.1}mm x {PITCH_Y:.1}mm pitch"
    );
    println!(
        "  Cartridge interface:        {COATING_CARTRIDGES} coating/flush/tracer sockets with sealed bulkhead lands"
    );
    println!(
        "  Fluid routing:              {ROW_DISPENSE_LANES} row dispense lanes, {ROW_RECIRCULATION_LANES} row recirculation lanes, {PRIME_LANES} prime/waste lanes, {ROW_VALVE_PORTS} row valve/prime ports"
    );
    println!(
        "  Witness QC:                 {WETNESS_WITNESS_POCKETS} wetness dwell pockets and {FLUORESCENT_WITNESS_SLOTS} fluorescent slide/coupon positions"
    );
    println!(
        "  Traceability and service:   {BARCODE_LOT_LANDS} barcode/lot lands, {KEEP_OUT_Z:.0}mm cassette robot clearance, {FLUID_SERVICE_CLEARANCE_Z:.0}mm cartridge pull clearance"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube("ecm_qc_baseplate_deck", MODULE_X, MODULE_Y, BASE_Z).translate(
        0.0,
        0.0,
        BASE_Z / 2.0,
    );

    let cartridge_sump = centered_cube(
        "ecm_qc_baseplate_cartridge_spill_sump",
        CARTRIDGE_BAY_X + 36.0,
        CARTRIDGE_BAY_Y + 28.0,
        8.0,
    )
    .translate(CARTRIDGE_CENTER_X, CARTRIDGE_CENTER_Y, BASE_Z - 3.0);

    let manifold_drip_basin = centered_cube(
        "ecm_qc_baseplate_manifold_drip_basin",
        MANIFOLD_X + 32.0,
        MANIFOLD_Y + 22.0,
        8.0,
    )
    .translate(MANIFOLD_CENTER_X, MANIFOLD_CENTER_Y, BASE_Z - 3.0);

    let cassette_recess = centered_cube(
        "ecm_qc_baseplate_cassette_nest_socket",
        NEST_X + 18.0,
        NEST_Y + 18.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let slide_socket = centered_cube(
        "ecm_qc_baseplate_witness_slide_socket",
        SLIDE_CARRIER_X + 10.0,
        SLIDE_CARRIER_Y + 12.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        SLIDE_CENTER_X,
        SLIDE_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let front_service_relief = centered_cube(
        "ecm_qc_baseplate_front_robot_service_relief",
        WRIST_SWEEP_CLEARANCE_X,
        FRONT_SERVICE_CLEARANCE_Y,
        5.0,
    )
    .translate(
        150.0,
        -MODULE_Y / 2.0 + FRONT_SERVICE_CLEARANCE_Y / 2.0,
        BASE_Z - 2.5,
    );

    let waste_gutter = centered_cube(
        "ecm_qc_baseplate_waste_gutter",
        DEGAS_WASTE_X + 90.0,
        24.0,
        9.0,
    )
    .translate(DEGAS_CENTER_X + 28.0, DEGAS_CENTER_Y - 66.0, BASE_Z - 4.0);

    let deck_drain = centered_cylinder("ecm_qc_baseplate_waste_drain", 8.0 / 2.0, 38.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            DEGAS_CENTER_X + DEGAS_WASTE_X / 2.0 - 38.0,
            -MODULE_Y / 2.0 + 16.0,
            BASE_Z / 2.0,
        );

    deck - cartridge_sump
        - manifold_drip_basin
        - cassette_recess
        - slide_socket
        - front_service_relief
        - waste_gutter
        - deck_drain
        - deck_mount_slots()
        - deck_route_trenches()
        + deck_perimeter_rim()
        + station_locator_targets()
        + module_zone_lands()
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("ecm_qc_baseplate_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("ecm_qc_baseplate_m6_clearance_{i}"),
            6.6 / 2.0,
            BASE_Z + 2.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("ecm_qc_baseplate_m6_slot_relief_{i}"),
            26.0,
            6.8,
            BASE_Z + 2.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
}

fn deck_route_trenches() -> Part {
    let mut trenches = Part::empty("ecm_qc_baseplate_route_trenches");

    for row in 0..ROWS {
        let y = lane_y(row);
        let bridge = centered_cube(
            format!("ecm_qc_baseplate_row_{row}_dispense_route_trench"),
            cassette_left_edge() - (MANIFOLD_CENTER_X + MANIFOLD_X / 2.0) + 72.0,
            9.0,
            7.0,
        )
        .translate(
            (cassette_left_edge() + MANIFOLD_CENTER_X + MANIFOLD_X / 2.0) / 2.0 - 22.0,
            y,
            BASE_Z - 3.2,
        );
        trenches = trenches + bridge;
    }

    let cartridge_to_manifold = centered_cube(
        "ecm_qc_baseplate_cartridge_to_manifold_trench",
        70.0,
        CARTRIDGE_CENTER_Y - (MANIFOLD_CENTER_Y + MANIFOLD_Y / 2.0) + 42.0,
        7.0,
    )
    .translate(
        MANIFOLD_CENTER_X - 108.0,
        (CARTRIDGE_CENTER_Y + MANIFOLD_CENTER_Y + MANIFOLD_Y / 2.0) / 2.0 - 20.0,
        BASE_Z - 3.2,
    );

    let waste_route = centered_cube(
        "ecm_qc_baseplate_waste_route_trench",
        78.0,
        (MANIFOLD_CENTER_Y - MANIFOLD_Y / 2.0) - (DEGAS_CENTER_Y + DEGAS_WASTE_Y / 2.0) + 56.0,
        7.0,
    )
    .translate(
        MANIFOLD_CENTER_X - 100.0,
        (MANIFOLD_CENTER_Y - MANIFOLD_Y / 2.0 + DEGAS_CENTER_Y + DEGAS_WASTE_Y / 2.0) / 2.0,
        BASE_Z - 3.2,
    );

    trenches + cartridge_to_manifold + waste_route
}

fn deck_perimeter_rim() -> Part {
    let rear = centered_cube(
        "ecm_qc_baseplate_rear_rim",
        MODULE_X - 2.0 * RIM_W,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, MODULE_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "ecm_qc_baseplate_left_rim",
        RIM_W,
        MODULE_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-MODULE_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "ecm_qc_baseplate_right_rim",
        RIM_W,
        MODULE_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(MODULE_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let front_left = centered_cube("ecm_qc_baseplate_front_left_low_lip", 328.0, 12.0, 14.0)
        .translate(
            -MODULE_X / 2.0 + 190.0,
            -MODULE_Y / 2.0 + 22.0,
            BASE_Z + 7.0,
        );
    let front_right = centered_cube("ecm_qc_baseplate_front_right_low_lip", 328.0, 12.0, 14.0)
        .translate(MODULE_X / 2.0 - 190.0, -MODULE_Y / 2.0 + 22.0, BASE_Z + 7.0);

    rear + left + right + front_left + front_right
}

fn station_locator_targets() -> Part {
    let mut targets = Part::empty("ecm_qc_station_locator_targets");
    for (i, (x, y)) in [
        (cassette_left_edge() + 34.0, cassette_top_edge() - 34.0),
        (cassette_right_edge() - 34.0, cassette_top_edge() - 34.0),
        (cassette_right_edge() - 34.0, cassette_bottom_edge() + 34.0),
        (cassette_left_edge() + 34.0, cassette_bottom_edge() + 34.0),
        (
            CARTRIDGE_CENTER_X - CARTRIDGE_BAY_X / 2.0 + 32.0,
            CARTRIDGE_CENTER_Y + CARTRIDGE_BAY_Y / 2.0 - 28.0,
        ),
        (
            CARTRIDGE_CENTER_X + CARTRIDGE_BAY_X / 2.0 - 32.0,
            CARTRIDGE_CENTER_Y + CARTRIDGE_BAY_Y / 2.0 - 28.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(format!("ecm_qc_station_datum_target_{i}"), 8.0, 2.4, 40)
            .translate(*x, *y, BASE_Z + 1.2);
        let center = centered_cylinder(format!("ecm_qc_station_datum_center_{i}"), 1.7, 3.0, 20)
            .translate(*x, *y, BASE_Z + 1.2);
        targets = targets + (target - center);
    }
    targets
}

fn module_zone_lands() -> Part {
    let cassette_zone = centered_cube(
        "ecm_qc_baseplate_cassette_zone_land",
        NEST_X + 34.0,
        12.0,
        3.0,
    )
    .translate(CASSETTE_CENTER_X, cassette_top_edge() + 15.0, BASE_Z + 1.5);
    let fluid_zone = centered_cube(
        "ecm_qc_baseplate_fluid_zone_land",
        CARTRIDGE_BAY_X,
        12.0,
        3.0,
    )
    .translate(
        CARTRIDGE_CENTER_X,
        CARTRIDGE_CENTER_Y - CARTRIDGE_BAY_Y / 2.0 - 15.0,
        BASE_Z + 1.5,
    );
    let witness_zone = centered_cube(
        "ecm_qc_baseplate_witness_zone_land",
        SLIDE_CARRIER_X,
        10.0,
        3.0,
    )
    .translate(
        SLIDE_CENTER_X,
        SLIDE_CENTER_Y + SLIDE_CARRIER_Y / 2.0 + 13.0,
        BASE_Z + 1.5,
    );

    cassette_zone + fluid_zone + witness_zone
}

fn cassette_datum_nest() -> Part {
    let tray = centered_cube("ecm_qc_cassette_nest_tray", NEST_X, NEST_Y, NEST_Z);
    let cassette_recess = centered_cube(
        "ecm_qc_cassette_nest_recess",
        CASSETTE_X + 12.0,
        CASSETTE_Y + 12.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, 0.0, 3.0);
    let side_tube_trench = centered_cube(
        "ecm_qc_cassette_nest_side_tube_trench",
        52.0,
        CASSETTE_Y + 72.0,
        NEST_Z + 2.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 45.0), 0.0, 2.4);
    let rear_port_clearance = centered_cube(
        "ecm_qc_cassette_nest_rear_manifold_port_clearance",
        CASSETTE_X + 92.0,
        44.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 42.0, 2.4);
    let front_witness_drain = centered_cube(
        "ecm_qc_cassette_nest_front_witness_drain",
        CASSETTE_X - 70.0,
        28.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 34.0), 2.4);
    let waste_port = centered_cylinder("ecm_qc_cassette_nest_witness_drain_port", 5.5, 30.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(CASSETTE_X / 2.0 - 58.0, -(CASSETTE_Y / 2.0 + 54.0), 0.0);

    tray - cassette_recess
        - side_tube_trench
        - rear_port_clearance
        - front_witness_drain
        - waste_port
        + cassette_datum_rails()
        + cassette_latch_posts()
        + cassette_row_port_comb()
        + cassette_fiducials()
}

fn cassette_datum_rails() -> Part {
    let z = NEST_Z / 2.0 + CASSETTE_DATUM_RAIL_Z / 2.0;
    let rear_y_datum = centered_cube(
        "ecm_qc_cassette_rear_y_datum_rail",
        CASSETTE_X + 42.0,
        16.0,
        CASSETTE_DATUM_RAIL_Z,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 20.0, z);
    let left_x_datum = centered_cube(
        "ecm_qc_cassette_left_x_datum_rail",
        16.0,
        CASSETTE_Y + 40.0,
        CASSETTE_DATUM_RAIL_Z,
    )
    .translate(-(CASSETTE_X / 2.0 + 20.0), 0.0, z);
    let right_soft_guide = centered_cube(
        "ecm_qc_cassette_right_soft_guide_rail",
        14.0,
        CASSETTE_Y + 36.0,
        18.0,
    )
    .translate(CASSETTE_X / 2.0 + 18.0, 0.0, z - 5.0);
    let front_low_lip = centered_cube(
        "ecm_qc_cassette_front_low_load_lip",
        CASSETTE_X - 92.0,
        10.0,
        14.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 11.0), z - 7.0);

    rear_y_datum + left_x_datum + right_soft_guide + front_low_lip
}

fn cassette_latch_posts() -> Part {
    let mut posts = Part::empty("ecm_qc_cassette_latch_posts");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 40.0), -(CASSETTE_Y / 2.0 - 36.0)),
        (CASSETTE_X / 2.0 - 40.0, -(CASSETTE_Y / 2.0 - 36.0)),
        (-(CASSETTE_X / 2.0 - 40.0), CASSETTE_Y / 2.0 - 36.0),
        (CASSETTE_X / 2.0 - 40.0, CASSETTE_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cylinder(format!("ecm_qc_cassette_latch_post_{i}"), 8.0, 22.0, 32)
            .translate(*x, *y, NEST_Z / 2.0 + 11.0);
        let screw = centered_cylinder(
            format!("ecm_qc_cassette_latch_m3_clearance_{i}"),
            3.4 / 2.0,
            24.0,
            20,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 11.0);
        posts = posts + (post - screw);
    }
    posts
}

fn cassette_row_port_comb() -> Part {
    let mut comb = Part::empty("ecm_qc_cassette_row_port_comb");
    for row in 0..ROWS {
        let y = row_y(row);
        let port_landing = centered_cube(
            format!("ecm_qc_cassette_row_{row}_port_landing_bar"),
            72.0,
            26.0,
            12.0,
        )
        .translate(-(CASSETTE_X / 2.0 + 28.0), y, NEST_Z / 2.0 + 6.0);
        let tube_relief = centered_cylinder(
            format!("ecm_qc_cassette_row_{row}_tube_relief"),
            ROW_TRUNK_D / 2.0,
            86.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(CASSETTE_X / 2.0 + 28.0), y, NEST_Z / 2.0 + 8.0);
        let row_register = centered_cube(
            format!("ecm_qc_cassette_row_{row}_dwell_register_bar"),
            CASSETTE_X + 36.0,
            5.0,
            7.0,
        )
        .translate(0.0, y, NEST_Z / 2.0 + 3.5);
        comb = comb + (port_landing - tube_relief) + row_register;
    }

    for col in 0..COLS {
        comb = comb
            + centered_cube(
                format!("ecm_qc_cassette_col_{col}_witness_alignment_bar"),
                5.0,
                CASSETTE_Y + 36.0,
                5.0,
            )
            .translate(chip_x(col), 0.0, NEST_Z / 2.0 + 2.5);
    }

    comb
}

fn cassette_fiducials() -> Part {
    let mut fiducials = Part::empty("ecm_qc_cassette_robot_fiducials");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 30.0), CASSETTE_Y / 2.0 - 24.0),
        (CASSETTE_X / 2.0 - 30.0, CASSETTE_Y / 2.0 - 24.0),
        (CASSETTE_X / 2.0 - 30.0, -(CASSETTE_Y / 2.0 - 24.0)),
    ]
    .iter()
    .enumerate()
    {
        let target =
            centered_cylinder(format!("ecm_qc_cassette_fiducial_target_{i}"), 7.5, 2.2, 40)
                .translate(*x, *y, NEST_Z / 2.0 + 1.1);
        let center =
            centered_cylinder(format!("ecm_qc_cassette_fiducial_center_{i}"), 1.6, 3.0, 20)
                .translate(*x, *y, NEST_Z / 2.0 + 1.1);
        fiducials = fiducials + (target - center);
    }
    fiducials
}

fn coating_cartridge_interface() -> Part {
    let tray = centered_cube(
        "ecm_qc_coating_cartridge_interface_tray",
        CARTRIDGE_BAY_X,
        CARTRIDGE_BAY_Y,
        CARTRIDGE_BAY_Z,
    );

    let mut socket_cuts = Part::empty("ecm_qc_coating_cartridge_socket_cuts");
    let mut cartridges = Part::empty("ecm_qc_coating_cartridge_envelopes");
    let mut bulkheads = Part::empty("ecm_qc_coating_cartridge_bulkheads");
    for i in 0..COATING_CARTRIDGES {
        let x = cartridge_x(i);
        let pocket = centered_cube(
            format!("ecm_qc_coating_cartridge_socket_{i}"),
            CARTRIDGE_SOCKET_X,
            CARTRIDGE_SOCKET_Y,
            CARTRIDGE_SOCKET_Z + 2.0,
        )
        .translate(x, 8.0, 5.0);
        socket_cuts = socket_cuts + pocket;

        let cartridge = centered_cube(
            format!("ecm_qc_coating_cartridge_placeholder_{i}"),
            CARTRIDGE_SOCKET_X - 8.0,
            CARTRIDGE_SOCKET_Y - 16.0,
            54.0,
        )
        .translate(x, 8.0, CARTRIDGE_BAY_Z / 2.0 + 27.0);
        let cap = centered_cube(
            format!("ecm_qc_coating_cartridge_cap_lip_{i}"),
            CARTRIDGE_SOCKET_X + 8.0,
            20.0,
            10.0,
        )
        .translate(
            x,
            8.0 + CARTRIDGE_SOCKET_Y / 2.0 - 7.0,
            CARTRIDGE_BAY_Z / 2.0 + 9.0,
        );
        cartridges = cartridges + cartridge + cap;

        let feed_port = cartridge_bulkhead_port(
            format!("ecm_qc_cartridge_{i}_dispense_bulkhead"),
            x - 13.0,
            -CARTRIDGE_BAY_Y / 2.0 + 15.0,
            7.0,
        );
        let return_port = cartridge_bulkhead_port(
            format!("ecm_qc_cartridge_{i}_return_bulkhead"),
            x + 13.0,
            -CARTRIDGE_BAY_Y / 2.0 + 15.0,
            7.0,
        );
        bulkheads = bulkheads + feed_port + return_port;
    }

    let common_header = centered_cube(
        "ecm_qc_coating_cartridge_common_header_land",
        CARTRIDGE_BAY_X - 30.0,
        18.0,
        16.0,
    )
    .translate(
        0.0,
        -CARTRIDGE_BAY_Y / 2.0 + 20.0,
        CARTRIDGE_BAY_Z / 2.0 + 8.0,
    );

    let seal_gasket_land = centered_cube(
        "ecm_qc_coating_cartridge_gasket_compression_land",
        CARTRIDGE_BAY_X - 44.0,
        12.0,
        4.0,
    )
    .translate(
        0.0,
        CARTRIDGE_BAY_Y / 2.0 - 18.0,
        CARTRIDGE_BAY_Z / 2.0 + 2.0,
    );

    tray - socket_cuts + cartridges + bulkheads + common_header + seal_gasket_land
}

fn cartridge_bulkhead_port(name: String, x: f64, y: f64, radius: f64) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), radius, 14.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, CARTRIDGE_BAY_Z / 2.0 + 8.0);
    let bore = centered_cylinder(format!("{name}_bore"), FLUID_BORE_D / 2.0, 18.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, CARTRIDGE_BAY_Z / 2.0 + 8.0);
    boss - bore
}

fn dispense_recirculation_lanes() -> Part {
    let body = centered_cube(
        "ecm_qc_dispense_recirculation_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let lane_bores = dispense_lane_bores() + recirculation_lane_bores() + prime_lane_bores();

    let mut valve_saddles = Part::empty("ecm_qc_dispense_recirculation_valve_saddles");
    for row in 0..ROWS {
        let y = manifold_row_y(row);
        let saddle = centered_cube(
            format!("ecm_qc_manifold_row_{row}_metering_valve_saddle"),
            42.0,
            24.0,
            10.0,
        )
        .translate(MANIFOLD_X / 2.0 - 70.0, y, MANIFOLD_Z / 2.0 + 5.0);
        let actuator_clearance = centered_cylinder(
            format!("ecm_qc_manifold_row_{row}_valve_actuator_clearance"),
            8.5,
            12.0,
            24,
        )
        .translate(MANIFOLD_X / 2.0 - 70.0, y, MANIFOLD_Z / 2.0 + 5.0);
        valve_saddles = valve_saddles + (saddle - actuator_clearance);
    }

    let recirc_pump_land = centered_cube("ecm_qc_recirculation_pump_mount_land", 112.0, 74.0, 14.0)
        .translate(
            -MANIFOLD_X / 2.0 + 76.0,
            -MANIFOLD_Y / 2.0 + 62.0,
            MANIFOLD_Z / 2.0 + 7.0,
        );

    let flow_sensor_land = centered_cube(
        "ecm_qc_dispense_flow_sensor_strip",
        82.0,
        MANIFOLD_Y - 56.0,
        12.0,
    )
    .translate(6.0, 0.0, MANIFOLD_Z / 2.0 + 6.0);

    let route_labels = manifold_lane_label_ticks();

    body - lane_bores + valve_saddles + recirc_pump_land + flow_sensor_land + route_labels
}

fn dispense_lane_bores() -> Part {
    let mut bores = Part::empty("ecm_qc_dispense_lane_bores");
    for row in 0..ROWS {
        let y = manifold_row_y(row);
        let z = lane_z(row, 0);
        let bore = centered_cylinder(
            format!("ecm_qc_row_{row}_dispense_lane_bore"),
            FLUID_BORE_D / 2.0,
            MANIFOLD_X + 18.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, z);
        let branch = centered_cylinder(
            format!("ecm_qc_row_{row}_dispense_to_valve_branch"),
            FLUID_BORE_D / 2.0,
            54.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(MANIFOLD_X / 2.0 - 46.0, y + 20.0, z);
        bores = bores + bore + branch;
    }
    bores
}

fn recirculation_lane_bores() -> Part {
    let mut bores = Part::empty("ecm_qc_recirculation_lane_bores");
    for row in 0..ROWS {
        let y = manifold_row_y(row);
        let z = lane_z(row, 1);
        let bore = centered_cylinder(
            format!("ecm_qc_row_{row}_recirculation_return_bore"),
            RECIRC_BORE_D / 2.0,
            MANIFOLD_X + 18.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y + 10.0, z);
        let cross_return = centered_cylinder(
            format!("ecm_qc_row_{row}_recirc_cross_return"),
            RECIRC_BORE_D / 2.0,
            46.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-MANIFOLD_X / 2.0 + 56.0, y - 10.0, z);
        bores = bores + bore + cross_return;
    }
    let common_return = centered_cylinder(
        "ecm_qc_common_recirc_return_header",
        RECIRC_BORE_D / 2.0,
        MANIFOLD_Y - 34.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-MANIFOLD_X / 2.0 + 56.0, 0.0, MANIFOLD_Z / 2.0 - 8.0);

    bores + common_return
}

fn prime_lane_bores() -> Part {
    let mut bores = Part::empty("ecm_qc_prime_lane_bores");
    for row in 0..ROWS {
        let y = manifold_row_y(row);
        let z = lane_z(row, 2);
        let prime = centered_cylinder(
            format!("ecm_qc_row_{row}_prime_lane_bore"),
            PRIME_BORE_D / 2.0,
            MANIFOLD_X - 54.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-12.0, y - 10.0, z);
        bores = bores + prime;
    }
    bores
}

fn manifold_lane_label_ticks() -> Part {
    let mut ticks = Part::empty("ecm_qc_manifold_lane_label_ticks");
    for row in 0..ROWS {
        let y = manifold_row_y(row);
        ticks = ticks
            + centered_cube(
                format!("ecm_qc_manifold_row_{row}_dispense_label_tick"),
                42.0,
                3.0,
                3.0,
            )
            .translate(MANIFOLD_X / 2.0 - 44.0, y, MANIFOLD_Z / 2.0 + 1.5)
            + centered_cube(
                format!("ecm_qc_manifold_row_{row}_recirc_label_tick"),
                42.0,
                3.0,
                3.0,
            )
            .translate(-MANIFOLD_X / 2.0 + 44.0, y, MANIFOLD_Z / 2.0 + 1.5);
    }
    ticks
}

fn timed_wetness_witness_pockets() -> Part {
    let plate = centered_cube(
        "ecm_qc_timed_wetness_witness_plate",
        WITNESS_PLATE_X,
        WITNESS_PLATE_Y,
        WITNESS_PLATE_Z,
    );

    let mut pocket_cuts = Part::empty("ecm_qc_timed_wetness_witness_pocket_cuts");
    let mut rim_marks = Part::empty("ecm_qc_timed_wetness_witness_pocket_rim_marks");
    for row in 0..ROWS {
        for col in 0..COLS {
            let index = row * COLS + col;
            let x = chip_x(col);
            let y = row_y(row);
            let pocket = centered_cylinder(
                format!("ecm_qc_wetness_witness_pocket_{index:02}"),
                WITNESS_POCKET_D / 2.0,
                WITNESS_POCKET_DEPTH + 1.0,
                32,
            )
            .translate(
                x,
                y,
                WITNESS_PLATE_Z / 2.0 - WITNESS_POCKET_DEPTH / 2.0 + 0.5,
            );
            let rim = centered_cylinder(
                format!("ecm_qc_wetness_witness_pocket_{index:02}_rim"),
                WITNESS_POCKET_D / 2.0 + 2.0,
                2.0,
                32,
            )
            .translate(x, y, WITNESS_PLATE_Z / 2.0 + 1.0);
            let rim_hole = centered_cylinder(
                format!("ecm_qc_wetness_witness_pocket_{index:02}_rim_hole"),
                WITNESS_POCKET_D / 2.0,
                2.4,
                32,
            )
            .translate(x, y, WITNESS_PLATE_Z / 2.0 + 1.0);
            pocket_cuts = pocket_cuts + pocket;
            rim_marks = rim_marks + (rim - rim_hole);
        }
    }

    let mut dwell_ticks = Part::empty("ecm_qc_dwell_time_tick_lands");
    for row in 0..ROWS {
        let y = row_y(row);
        for tick in 0..3 {
            dwell_ticks = dwell_ticks
                + centered_cube(
                    format!("ecm_qc_row_{row}_dwell_tick_{tick}"),
                    18.0,
                    3.0,
                    2.0,
                )
                .translate(
                    -(CASSETTE_X / 2.0 - 22.0) + tick as f64 * 24.0,
                    y,
                    WITNESS_PLATE_Z / 2.0 + 1.0,
                );
        }
    }

    let drain_channel = centered_cube(
        "ecm_qc_wetness_witness_front_drain_channel",
        WITNESS_PLATE_X - 58.0,
        14.0,
        7.0,
    )
    .translate(
        0.0,
        -(WITNESS_PLATE_Y / 2.0 - 18.0),
        WITNESS_PLATE_Z / 2.0 - 2.0,
    );

    plate - pocket_cuts - drain_channel + rim_marks + dwell_ticks
}

fn fluorescent_witness_slide_carrier() -> Part {
    let carrier = centered_cube(
        "ecm_qc_fluorescent_witness_slide_carrier",
        SLIDE_CARRIER_X,
        SLIDE_CARRIER_Y,
        SLIDE_CARRIER_Z,
    );

    let mut slot_cuts = Part::empty("ecm_qc_fluorescent_slide_slot_cuts");
    let mut coupon_stops = Part::empty("ecm_qc_fluorescent_slide_coupon_stops");
    for i in 0..FLUORESCENT_WITNESS_SLOTS {
        let x = slide_slot_x(i);
        let slot = centered_cube(
            format!("ecm_qc_fluorescent_slide_slot_{i}"),
            SLIDE_SLOT_X,
            SLIDE_SLOT_Y,
            SLIDE_SLOT_DEPTH + 1.0,
        )
        .translate(x, 4.0, SLIDE_CARRIER_Z / 2.0 - SLIDE_SLOT_DEPTH / 2.0 + 0.5);
        let aperture = centered_cylinder(
            format!("ecm_qc_fluorescent_slide_slot_{i}_reader_aperture"),
            5.0,
            SLIDE_CARRIER_Y + 4.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, SLIDE_CARRIER_Z / 2.0 - 7.0);
        slot_cuts = slot_cuts + slot + aperture;

        let rear_stop = centered_cube(
            format!("ecm_qc_fluorescent_slide_slot_{i}_rear_stop"),
            SLIDE_SLOT_X,
            4.0,
            10.0,
        )
        .translate(x, SLIDE_CARRIER_Y / 2.0 - 7.0, SLIDE_CARRIER_Z / 2.0 + 5.0);
        let front_thumb_cut = centered_cube(
            format!("ecm_qc_fluorescent_slide_slot_{i}_front_thumb_relief_land"),
            28.0,
            4.0,
            8.0,
        )
        .translate(
            x,
            -(SLIDE_CARRIER_Y / 2.0 - 7.0),
            SLIDE_CARRIER_Z / 2.0 + 4.0,
        );
        coupon_stops = coupon_stops + rear_stop + front_thumb_cut;
    }

    let excitation_bar = centered_cube(
        "ecm_qc_fluorescent_witness_blue_excitation_bar_placeholder",
        SLIDE_CARRIER_X - 44.0,
        10.0,
        12.0,
    )
    .translate(
        0.0,
        -(SLIDE_CARRIER_Y / 2.0 + 10.0),
        SLIDE_CARRIER_Z / 2.0 + 6.0,
    );

    let dark_reference_pocket = centered_cube(
        "ecm_qc_fluorescent_witness_dark_reference_pocket",
        46.0,
        24.0,
        10.0,
    )
    .translate(
        slide_slot_x(FLUORESCENT_WITNESS_SLOTS - 1),
        -18.0,
        SLIDE_CARRIER_Z / 2.0 - 2.0,
    );

    carrier - slot_cuts - dark_reference_pocket + coupon_stops + excitation_bar
}

fn row_valve_prime_ports() -> Part {
    let spine = centered_cube(
        "ecm_qc_row_valve_prime_port_spine",
        VALVE_BANK_X,
        VALVE_BANK_Y,
        VALVE_BANK_Z,
    );
    let mut blocks = Part::empty("ecm_qc_row_valve_prime_blocks");
    let mut bores = Part::empty("ecm_qc_row_valve_prime_port_bores");

    for row in 0..ROWS {
        let y = row_y(row);
        let row_block = centered_cube(
            format!("ecm_qc_row_{row}_valve_block"),
            VALVE_BANK_X + 30.0,
            48.0,
            20.0,
        )
        .translate(0.0, y, VALVE_BANK_Z / 2.0 + 10.0);
        blocks = blocks + row_block;

        for port in 0..VALVE_PORTS_PER_ROW {
            let x = valve_port_x(port);
            let port_bore = centered_cylinder(
                format!("ecm_qc_row_{row}_valve_prime_port_{port}"),
                match port {
                    0 => FLUID_BORE_D / 2.0,
                    1 => RECIRC_BORE_D / 2.0,
                    _ => PRIME_BORE_D / 2.0,
                },
                VALVE_BANK_Z + 28.0,
                24,
            )
            .translate(x, y, VALVE_BANK_Z / 2.0);
            let front_bulkhead = centered_cylinder(
                format!("ecm_qc_row_{row}_front_bulkhead_boss_{port}"),
                8.0,
                12.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - 18.0, VALVE_BANK_Z / 2.0 + 6.0);
            let front_bore = centered_cylinder(
                format!("ecm_qc_row_{row}_front_bulkhead_bore_{port}"),
                FLUID_BORE_D / 2.0,
                16.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - 18.0, VALVE_BANK_Z / 2.0 + 6.0);
            bores = bores + port_bore;
            blocks = blocks + (front_bulkhead - front_bore);
        }

        let pinch_actuator_keepout = centered_cube(
            format!("ecm_qc_row_{row}_pinch_valve_actuator_keepout_placeholder"),
            96.0,
            20.0,
            16.0,
        )
        .translate(0.0, y + 20.0, VALVE_BANK_Z + 8.0);
        blocks = blocks + pinch_actuator_keepout;
    }

    let common_prime_header = centered_cylinder(
        "ecm_qc_common_prime_header_bore",
        PRIME_BORE_D / 2.0,
        VALVE_BANK_Y + 22.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(valve_port_x(2), 0.0, VALVE_BANK_Z / 2.0 - 12.0);

    spine + blocks - bores - common_prime_header
}

fn bubble_degas_waste_path() -> Part {
    let tray = centered_cube(
        "ecm_qc_bubble_degas_waste_tray",
        DEGAS_WASTE_X,
        DEGAS_WASTE_Y,
        DEGAS_WASTE_Z,
    );

    let recirc_sump = centered_cube(
        "ecm_qc_bubble_degas_recirc_sump",
        118.0,
        82.0,
        DEGAS_WASTE_Z + 2.0,
    )
    .translate(-DEGAS_WASTE_X / 2.0 + 86.0, 20.0, 4.0);
    let waste_sump = centered_cube(
        "ecm_qc_bubble_degas_waste_sump",
        136.0,
        78.0,
        DEGAS_WASTE_Z + 2.0,
    )
    .translate(DEGAS_WASTE_X / 2.0 - 92.0, -22.0, 4.0);
    let drain = centered_cylinder(
        "ecm_qc_bubble_degas_waste_drain_bore",
        WASTE_BORE_D / 2.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DEGAS_WASTE_X / 2.0 - 54.0, -DEGAS_WASTE_Y / 2.0 + 16.0, 0.0);

    let degas_placeholder =
        centered_cube("ecm_qc_membrane_degas_placeholder_bay", 108.0, 48.0, 24.0).translate(
            -34.0,
            DEGAS_WASTE_Y / 2.0 - 38.0,
            DEGAS_WASTE_Z / 2.0 + 12.0,
        );
    let degas_window = centered_cube("ecm_qc_membrane_degas_visual_window", 82.0, 6.0, 12.0)
        .translate(
            -34.0,
            DEGAS_WASTE_Y / 2.0 - 64.0,
            DEGAS_WASTE_Z / 2.0 + 12.0,
        );

    let bubble_tower = centered_cylinder(
        "ecm_qc_bubble_trap_tower_placeholder",
        BUBBLE_TOWER_D / 2.0,
        BUBBLE_TOWER_Z,
        48,
    )
    .translate(
        -DEGAS_WASTE_X / 2.0 + 82.0,
        -DEGAS_WASTE_Y / 2.0 + 52.0,
        DEGAS_WASTE_Z / 2.0 + BUBBLE_TOWER_Z / 2.0,
    );
    let tower_core = centered_cylinder(
        "ecm_qc_bubble_trap_tower_core",
        (BUBBLE_TOWER_D - 12.0) / 2.0,
        BUBBLE_TOWER_Z + 2.0,
        48,
    )
    .translate(
        -DEGAS_WASTE_X / 2.0 + 82.0,
        -DEGAS_WASTE_Y / 2.0 + 52.0,
        DEGAS_WASTE_Z / 2.0 + BUBBLE_TOWER_Z / 2.0,
    );
    let optical_bubble_fork = centered_cube(
        "ecm_qc_bubble_detector_optical_fork_placeholder",
        52.0,
        24.0,
        28.0,
    )
    .translate(
        -DEGAS_WASTE_X / 2.0 + 82.0,
        -DEGAS_WASTE_Y / 2.0 + 100.0,
        DEGAS_WASTE_Z / 2.0 + 14.0,
    );

    let waste_bulkhead = centered_cylinder("ecm_qc_waste_path_bulkhead_boss", 10.0, 16.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            DEGAS_WASTE_X / 2.0 - 50.0,
            -DEGAS_WASTE_Y / 2.0 + 18.0,
            DEGAS_WASTE_Z / 2.0,
        );
    let waste_bulkhead_bore = centered_cylinder(
        "ecm_qc_waste_path_bulkhead_bore",
        WASTE_BORE_D / 2.0,
        20.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        DEGAS_WASTE_X / 2.0 - 50.0,
        -DEGAS_WASTE_Y / 2.0 + 18.0,
        DEGAS_WASTE_Z / 2.0,
    );

    tray - recirc_sump - waste_sump - drain
        + (degas_placeholder - degas_window)
        + (bubble_tower - tower_core)
        + optical_bubble_fork
        + (waste_bulkhead - waste_bulkhead_bore)
}

fn barcode_lot_lands() -> Part {
    let mut lands = Part::empty("ecm_qc_barcode_lot_lands");

    for i in 0..COATING_CARTRIDGES {
        lands = lands
            + barcode_land(
                format!("ecm_qc_barcode_coating_cartridge_{i}"),
                CARTRIDGE_CENTER_X + cartridge_x(i),
                CARTRIDGE_CENTER_Y + CARTRIDGE_BAY_Y / 2.0 + 24.0,
                54.0,
                20.0,
            );
    }

    for i in 0..FLUORESCENT_WITNESS_SLOTS {
        lands = lands
            + barcode_land(
                format!("ecm_qc_barcode_fluorescent_witness_{i}"),
                SLIDE_CENTER_X + slide_slot_x(i),
                SLIDE_CENTER_Y - SLIDE_CARRIER_Y / 2.0 - 18.0,
                58.0,
                18.0,
            );
    }

    let cassette_lot = barcode_land(
        "ecm_qc_barcode_cassette_lot_land".to_string(),
        CASSETTE_CENTER_X + CASSETTE_X / 2.0 - 64.0,
        CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 + 34.0,
        84.0,
        22.0,
    );
    let process_record = barcode_land(
        "ecm_qc_barcode_process_record_land".to_string(),
        CASSETTE_CENTER_X + CASSETTE_X / 2.0 - 64.0,
        CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 - 34.0,
        84.0,
        22.0,
    );

    lands + cassette_lot + process_record
}

fn barcode_land(name: String, x: f64, y: f64, sx: f64, sy: f64) -> Part {
    let land = centered_cube(format!("{name}_land"), sx, sy, 3.0).translate(x, y, BASE_Z + 1.5);
    let notch = centered_cube(format!("{name}_orientation_notch"), 8.0, 3.0, 3.4).translate(
        x - sx / 2.0 + 10.0,
        y + sy / 2.0 - 3.0,
        BASE_Z + 1.5,
    );
    land - notch
}

fn robot_service_keepouts() -> Part {
    let cassette_frame = keepout_frame(
        "ecm_qc_robot_cassette_overhead_keepout",
        CASSETTE_ROBOT_CLEARANCE_X,
        CASSETTE_ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        BASE_Z + NEST_Z + KEEP_OUT_Z / 2.0,
    );

    let cartridge_pull_frame = keepout_frame(
        "ecm_qc_robot_cartridge_pull_keepout",
        CARTRIDGE_BAY_X + 86.0,
        CARTRIDGE_BAY_Y + 92.0,
        FLUID_SERVICE_CLEARANCE_Z,
    )
    .translate(
        CARTRIDGE_CENTER_X,
        CARTRIDGE_CENTER_Y,
        BASE_Z + CARTRIDGE_BAY_Z + FLUID_SERVICE_CLEARANCE_Z / 2.0,
    );

    let slide_access_frame = keepout_frame(
        "ecm_qc_robot_fluorescent_slide_service_keepout",
        SLIDE_CARRIER_X + 76.0,
        SLIDE_CARRIER_Y + 72.0,
        112.0,
    )
    .translate(
        SLIDE_CENTER_X,
        SLIDE_CENTER_Y,
        BASE_Z + SLIDE_CARRIER_Z + 56.0,
    );

    let wrist_sweep = centered_cube(
        "ecm_qc_front_robot_wrist_sweep_keepout_floor_marker",
        WRIST_SWEEP_CLEARANCE_X,
        WRIST_SWEEP_CLEARANCE_Y,
        4.0,
    )
    .translate(
        150.0,
        -MODULE_Y / 2.0 + WRIST_SWEEP_CLEARANCE_Y / 2.0,
        BASE_Z + 2.0,
    );

    cassette_frame + cartridge_pull_frame + slide_access_frame + wrist_sweep
}

fn keepout_frame(name: &str, sx: f64, sy: f64, sz: f64) -> Part {
    let rail = 7.0;
    let z_mid = sz / 2.0;
    let bottom_front = centered_cube(format!("{name}_bottom_front"), sx, rail, rail).translate(
        0.0,
        -sy / 2.0,
        rail / 2.0,
    );
    let bottom_rear = centered_cube(format!("{name}_bottom_rear"), sx, rail, rail).translate(
        0.0,
        sy / 2.0,
        rail / 2.0,
    );
    let bottom_left = centered_cube(format!("{name}_bottom_left"), rail, sy, rail).translate(
        -sx / 2.0,
        0.0,
        rail / 2.0,
    );
    let bottom_right = centered_cube(format!("{name}_bottom_right"), rail, sy, rail).translate(
        sx / 2.0,
        0.0,
        rail / 2.0,
    );
    let top_front = centered_cube(format!("{name}_top_front"), sx, rail, rail).translate(
        0.0,
        -sy / 2.0,
        sz - rail / 2.0,
    );
    let top_rear = centered_cube(format!("{name}_top_rear"), sx, rail, rail).translate(
        0.0,
        sy / 2.0,
        sz - rail / 2.0,
    );
    let top_left = centered_cube(format!("{name}_top_left"), rail, sy, rail).translate(
        -sx / 2.0,
        0.0,
        sz - rail / 2.0,
    );
    let top_right = centered_cube(format!("{name}_top_right"), rail, sy, rail).translate(
        sx / 2.0,
        0.0,
        sz - rail / 2.0,
    );

    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (x, y)) in [
        (-sx / 2.0, -sy / 2.0),
        (sx / 2.0, -sy / 2.0),
        (-sx / 2.0, sy / 2.0),
        (sx / 2.0, sy / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_post_{i}"), rail, rail, sz).translate(*x, *y, z_mid);
    }

    bottom_front
        + bottom_rear
        + bottom_left
        + bottom_right
        + top_front
        + top_rear
        + top_left
        + top_right
        + posts
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(MODULE_X / 2.0 - 44.0), -(MODULE_Y / 2.0 - 44.0)),
        (MODULE_X / 2.0 - 44.0, -(MODULE_Y / 2.0 - 44.0)),
        (-(MODULE_X / 2.0 - 44.0), MODULE_Y / 2.0 - 44.0),
        (MODULE_X / 2.0 - 44.0, MODULE_Y / 2.0 - 44.0),
        (-170.0, -(MODULE_Y / 2.0 - 44.0)),
        (170.0, -(MODULE_Y / 2.0 - 44.0)),
        (-170.0, MODULE_Y / 2.0 - 44.0),
        (170.0, MODULE_Y / 2.0 - 44.0),
    ]
}

fn cartridge_x(index: usize) -> f64 {
    let centered = index as f64 - (COATING_CARTRIDGES as f64 - 1.0) / 2.0;
    centered * CARTRIDGE_PITCH_X
}

fn slide_slot_x(index: usize) -> f64 {
    let centered = index as f64 - (FLUORESCENT_WITNESS_SLOTS as f64 - 1.0) / 2.0;
    centered * SLIDE_PITCH_X
}

fn chip_x(col: usize) -> f64 {
    -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * PITCH_X
}

fn row_y(row: usize) -> f64 {
    -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * PITCH_Y
}

fn lane_y(row: usize) -> f64 {
    CASSETTE_CENTER_Y + row_y(row)
}

fn manifold_row_y(row: usize) -> f64 {
    row_y(row) * 0.62
}

fn lane_z(row: usize, lane_group: usize) -> f64 {
    -MANIFOLD_Z / 2.0 + 9.0 + lane_group as f64 * LANE_STEP_Z + (row % 2) as f64 * 1.2
}

fn valve_port_x(port: usize) -> f64 {
    (port as f64 - (VALVE_PORTS_PER_ROW as f64 - 1.0) / 2.0) * VALVE_PORT_PITCH_X
}

fn cassette_left_edge() -> f64 {
    CASSETTE_CENTER_X - NEST_X / 2.0
}

fn cassette_right_edge() -> f64 {
    CASSETTE_CENTER_X + NEST_X / 2.0
}

fn cassette_top_edge() -> f64 {
    CASSETTE_CENTER_Y + NEST_Y / 2.0
}

fn cassette_bottom_edge() -> f64 {
    CASSETTE_CENTER_Y - NEST_Y / 2.0
}

fn cartridge_left_edge() -> f64 {
    CARTRIDGE_CENTER_X - CARTRIDGE_BAY_X / 2.0
}

fn cartridge_right_edge() -> f64 {
    CARTRIDGE_CENTER_X + CARTRIDGE_BAY_X / 2.0
}

fn slide_bottom_edge() -> f64 {
    SLIDE_CENTER_Y - SLIDE_CARRIER_Y / 2.0
}

fn assert_layout() {
    assert_eq!(POSITION_COUNT, 20);
    assert_eq!(WETNESS_WITNESS_POCKETS, POSITION_COUNT);
    assert_eq!(ROW_DISPENSE_LANES, ROWS);
    assert_eq!(ROW_RECIRCULATION_LANES, ROWS);
    assert_eq!(PRIME_LANES, ROWS);
    assert_eq!(ROW_VALVE_PORTS, ROWS * VALVE_PORTS_PER_ROW);
    assert_eq!(FLUORESCENT_WITNESS_SLOTS, ROWS + 1);
    assert_eq!(
        BARCODE_LOT_LANDS,
        COATING_CARTRIDGES + FLUORESCENT_WITNESS_SLOTS + 2
    );

    assert!(cassette_right_edge() < MODULE_X / 2.0 - 16.0);
    assert!(cassette_left_edge() > VALVE_BANK_CENTER_X + VALVE_BANK_X / 2.0 - 10.0);
    assert!(cartridge_left_edge() > -MODULE_X / 2.0 + RIM_W);
    assert!(cartridge_right_edge() < MODULE_X / 2.0 - RIM_W);
    assert!(cassette_top_edge() < MODULE_Y / 2.0 - 28.0);
    assert!(slide_bottom_edge() > -MODULE_Y / 2.0 + 6.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn cassette_geometry_matches_twenty_chip_workflow() {
        assert_eq!(COLS * ROWS, 20);
        assert_eq!(PITCH_X, REVC_CHIP_LENGTH + GUTTER);
        assert_eq!(PITCH_Y, REVC_CHIP_WIDTH + GUTTER);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);
        assert!(NEST_X > CASSETTE_X + 100.0);
        assert!(NEST_Y > CASSETTE_Y + 100.0);
    }

    #[test]
    fn fluid_lane_and_port_counts_are_per_row() {
        assert_eq!(ROW_DISPENSE_LANES, ROWS);
        assert_eq!(ROW_RECIRCULATION_LANES, ROWS);
        assert_eq!(PRIME_LANES, ROWS);
        assert_eq!(VALVE_PORTS_PER_ROW, 3);
        assert_eq!(ROW_VALVE_PORTS, 15);
        assert!(valve_port_x(0) < valve_port_x(1));
        assert!(valve_port_x(1) < valve_port_x(2));
        assert!(manifold_row_y(0) < manifold_row_y(ROWS - 1));
    }

    #[test]
    fn witness_and_traceability_counts_cover_all_rows() {
        assert_eq!(WETNESS_WITNESS_POCKETS, POSITION_COUNT);
        assert_eq!(FLUORESCENT_WITNESS_SLOTS, ROWS + 1);
        assert_eq!(
            BARCODE_LOT_LANDS,
            COATING_CARTRIDGES + FLUORESCENT_WITNESS_SLOTS + 2
        );

        let mut slide_positions = HashSet::new();
        for i in 0..FLUORESCENT_WITNESS_SLOTS {
            slide_positions.insert((slide_slot_x(i) * 10.0).round() as i64);
        }
        assert_eq!(slide_positions.len(), FLUORESCENT_WITNESS_SLOTS);
    }

    #[test]
    fn major_modules_fit_on_station_without_crossing_edges() {
        assert_layout();
        assert!(MODULE_X >= 1100.0);
        assert!(MODULE_Y >= 720.0);
        assert!(CASSETTE_ROBOT_CLEARANCE_X > NEST_X + 100.0);
        assert!(CASSETTE_ROBOT_CLEARANCE_Y > NEST_Y + 90.0);
        assert!(KEEP_OUT_Z >= 140.0);
        assert!(FLUID_SERVICE_CLEARANCE_Z >= 160.0);
    }

    #[test]
    fn cartridge_and_slide_arrays_have_expected_spacing() {
        assert_eq!(COATING_CARTRIDGES, 4);
        assert!(cartridge_x(0) < cartridge_x(COATING_CARTRIDGES - 1));
        assert!(
            cartridge_x(COATING_CARTRIDGES - 1) - cartridge_x(0)
                >= CARTRIDGE_PITCH_X * (COATING_CARTRIDGES as f64 - 1.0)
        );
        assert!(slide_slot_x(0) < slide_slot_x(FLUORESCENT_WITNESS_SLOTS - 1));
        assert!(SLIDE_SLOT_X < SLIDE_PITCH_X);
    }
}
