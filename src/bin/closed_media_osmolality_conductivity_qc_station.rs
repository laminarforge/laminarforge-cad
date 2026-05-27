use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media osmolality/conductivity/pH QC station before media release to
// perfusion.
//
// Design intent:
// - Pull a sterile side-stream through a closed sample loop into sealed
//   low-volume cup cartridges without open media handling.
// - Dock purchased osmolality, conductivity, and pH analyzer envelopes while
//   keeping them mechanically separate from the sterile loop.
// - Keep calibration standards, lot evidence, flush/waste routing, temperature
//   equilibration, and release/hold/reject decisions physically visible on the
//   same station before perfusion release.
// - Reserve front robot reach, rear service access, and top analyzer service
//   keepouts so this can be integrated into the larger closed workcell.
//
// This is mechanical architecture CAD only. It does not define analytical
// acceptance criteria, calibration procedures, sterile validation, or analyzer
// vendor compatibility.

const OUTPUTS: [&str; 15] = [
    "output/closed_media_osmolality_conductivity_qc_station_base_tray.stl",
    "output/closed_media_osmolality_conductivity_qc_station_sterile_sample_loop_manifold.stl",
    "output/closed_media_osmolality_conductivity_qc_station_sealed_sample_cup_cartridge.stl",
    "output/closed_media_osmolality_conductivity_qc_station_osmolality_analyzer_dock.stl",
    "output/closed_media_osmolality_conductivity_qc_station_conductivity_analyzer_dock.stl",
    "output/closed_media_osmolality_conductivity_qc_station_ph_analyzer_dock.stl",
    "output/closed_media_osmolality_conductivity_qc_station_calibration_standard_custody_pockets.stl",
    "output/closed_media_osmolality_conductivity_qc_station_flush_waste_routing_manifold.stl",
    "output/closed_media_osmolality_conductivity_qc_station_barcode_lot_traceability_panel.stl",
    "output/closed_media_osmolality_conductivity_qc_station_temperature_equilibration_pocket.stl",
    "output/closed_media_osmolality_conductivity_qc_station_release_hold_reject_lanes.stl",
    "output/closed_media_osmolality_conductivity_qc_station_custody_interlock_bridge.stl",
    "output/closed_media_osmolality_conductivity_qc_station_robot_service_keepouts.stl",
    "output/closed_media_osmolality_conductivity_qc_station_sealed_transfer_carrier.stl",
    "output/closed_media_osmolality_conductivity_qc_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 15] = [
    "sterile_sample_loop",
    "sealed_sample_cup_cartridge",
    "osmolality_analyzer_dock",
    "conductivity_analyzer_dock",
    "ph_analyzer_dock",
    "calibration_standard_custody_pockets",
    "flush_waste_routing",
    "barcode_lot_traceability",
    "temperature_equilibration_pocket",
    "release_lane",
    "hold_lane",
    "reject_lane",
    "custody_interlock_bridge",
    "robot_keepout",
    "service_keepout",
];

const DECK_X: f64 = 1160.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;

const SAMPLE_LOOP_X: f64 = 330.0;
const SAMPLE_LOOP_Y: f64 = 132.0;
const SAMPLE_LOOP_Z: f64 = 52.0;
const SAMPLE_LOOP_POS: (f64, f64) = (-360.0, 208.0);
const LOOP_CHANNEL_D: f64 = 5.5;
const LOOP_VALVE_COUNT: usize = 5;
const LOOP_VALVE_PITCH: f64 = 54.0;
const SAMPLE_LOOP_VOLUME_ML: f64 = 2.0;

const CUP_CARTRIDGE_X: f64 = 302.0;
const CUP_CARTRIDGE_Y: f64 = 132.0;
const CUP_CARTRIDGE_Z: f64 = 42.0;
const CUP_CARTRIDGE_POS: (f64, f64) = (0.0, 208.0);
const SEALED_CUP_COUNT: usize = 12;
const CUP_COLS: usize = 6;
const CUP_ROWS: usize = 2;
const CUP_PITCH_X: f64 = 42.0;
const CUP_PITCH_Y: f64 = 52.0;
const CUP_WELL_D: f64 = 22.0;
const CUP_SEPTUM_D: f64 = 17.0;

const TEMP_EQ_X: f64 = 260.0;
const TEMP_EQ_Y: f64 = 150.0;
const TEMP_EQ_Z: f64 = 58.0;
const TEMP_EQ_POS: (f64, f64) = (350.0, 208.0);
const TEMP_POCKET_COUNT: usize = 4;
const TEMP_TARGET_C: f64 = 37.0;

const OSMO_DOCK_X: f64 = 270.0;
const OSMO_DOCK_Y: f64 = 170.0;
const OSMO_DOCK_Z: f64 = 110.0;
const OSMO_DOCK_POS: (f64, f64) = (-340.0, -10.0);

const CONDUCTIVITY_DOCK_X: f64 = 230.0;
const CONDUCTIVITY_DOCK_Y: f64 = 160.0;
const CONDUCTIVITY_DOCK_Z: f64 = 90.0;
const CONDUCTIVITY_DOCK_POS: (f64, f64) = (-45.0, -10.0);

const PH_DOCK_X: f64 = 230.0;
const PH_DOCK_Y: f64 = 160.0;
const PH_DOCK_Z: f64 = 90.0;
const PH_DOCK_POS: (f64, f64) = (225.0, -10.0);

const ANALYZER_DOCK_COUNT: usize = 3;
const ANALYZER_SAMPLE_PORTS: usize = 3;
const ANALYZER_WASTE_PORTS: usize = 3;

const CAL_BANK_X: f64 = 310.0;
const CAL_BANK_Y: f64 = 155.0;
const CAL_BANK_Z: f64 = 48.0;
const CAL_BANK_POS: (f64, f64) = (-385.0, -205.0);
const CAL_STANDARD_POCKETS: usize = 8;
const CAL_POCKET_PITCH_X: f64 = 35.0;
const CAL_CUSTODY_SEAL_PADS: usize = 8;

const FLUSH_MANIFOLD_X: f64 = 360.0;
const FLUSH_MANIFOLD_Y: f64 = 130.0;
const FLUSH_MANIFOLD_Z: f64 = 44.0;
const FLUSH_MANIFOLD_POS: (f64, f64) = (0.0, -205.0);
const FLUSH_LINE_D: f64 = 6.0;
const WASTE_LINE_D: f64 = 9.0;
const FLUSH_VALVE_COUNT: usize = 6;

const TRACE_PANEL_X: f64 = 290.0;
const TRACE_PANEL_Y: f64 = 130.0;
const TRACE_PANEL_Z: f64 = 16.0;
const TRACE_PANEL_POS: (f64, f64) = (350.0, -205.0);
const BARCODE_LANDS: usize = 10;
const LOT_CARD_SLOTS: usize = 4;
const RFID_PADS: usize = 4;

const LANE_BANK_X: f64 = 780.0;
const LANE_BANK_Y: f64 = 80.0;
const LANE_BANK_Z: f64 = 34.0;
const LANE_BANK_POS: (f64, f64) = (0.0, -327.0);
const STATUS_LANE_COUNT: usize = 3;
const STATUS_TOKENS_PER_LANE: usize = 8;
const LANE_CLEAR_GAP: f64 = 22.0;

const BRIDGE_X: f64 = 820.0;
const BRIDGE_Y: f64 = 36.0;
const BRIDGE_Z: f64 = 116.0;
const BRIDGE_POS: (f64, f64) = (0.0, -286.0);
const BRIDGE_SHUTTERS: usize = 3;

const TRANSFER_CARRIER_X: f64 = 300.0;
const TRANSFER_CARRIER_Y: f64 = 46.0;
const TRANSFER_CARRIER_Z: f64 = 30.0;
const TRANSFER_CARRIER_POS: (f64, f64) = (0.0, 108.0);
const TRANSFER_CARRIER_SOCKETS: usize = 6;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 220.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 170.0;
const LEFT_BARCODE_SERVICE_KEEP_OUT_X: f64 = 118.0;
const RIGHT_ANALYZER_SERVICE_KEEP_OUT_X: f64 = 136.0;
const TOP_ANALYZER_SERVICE_CLEARANCE_Z: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let sample_loop = sterile_sample_loop_manifold();
    export(OUTPUTS[1], &sample_loop);

    let cup_cartridge = sealed_sample_cup_cartridge();
    export(OUTPUTS[2], &cup_cartridge);

    let osmo = osmolality_analyzer_dock();
    export(OUTPUTS[3], &osmo);

    let conductivity = conductivity_analyzer_dock();
    export(OUTPUTS[4], &conductivity);

    let ph = ph_analyzer_dock();
    export(OUTPUTS[5], &ph);

    let calibration = calibration_standard_custody_pockets();
    export(OUTPUTS[6], &calibration);

    let flush_waste = flush_waste_routing_manifold();
    export(OUTPUTS[7], &flush_waste);

    let traceability = barcode_lot_traceability_panel();
    export(OUTPUTS[8], &traceability);

    let temperature = temperature_equilibration_pocket();
    export(OUTPUTS[9], &temperature);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[10], &lanes);

    let bridge = custody_interlock_bridge();
    export(OUTPUTS[11], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[12], &keepouts);

    let carrier = sealed_transfer_carrier();
    export(OUTPUTS[13], &carrier);

    let assembly = base
        + sample_loop.translate(SAMPLE_LOOP_POS.0, SAMPLE_LOOP_POS.1, part_z(SAMPLE_LOOP_Z))
        + cup_cartridge.translate(
            CUP_CARTRIDGE_POS.0,
            CUP_CARTRIDGE_POS.1,
            part_z(CUP_CARTRIDGE_Z),
        )
        + osmo.translate(OSMO_DOCK_POS.0, OSMO_DOCK_POS.1, part_z(OSMO_DOCK_Z))
        + conductivity.translate(
            CONDUCTIVITY_DOCK_POS.0,
            CONDUCTIVITY_DOCK_POS.1,
            part_z(CONDUCTIVITY_DOCK_Z),
        )
        + ph.translate(PH_DOCK_POS.0, PH_DOCK_POS.1, part_z(PH_DOCK_Z))
        + calibration.translate(CAL_BANK_POS.0, CAL_BANK_POS.1, part_z(CAL_BANK_Z))
        + flush_waste.translate(
            FLUSH_MANIFOLD_POS.0,
            FLUSH_MANIFOLD_POS.1,
            part_z(FLUSH_MANIFOLD_Z),
        )
        + traceability.translate(TRACE_PANEL_POS.0, TRACE_PANEL_POS.1, part_z(TRACE_PANEL_Z))
        + temperature.translate(TEMP_EQ_POS.0, TEMP_EQ_POS.1, part_z(TEMP_EQ_Z))
        + lanes.translate(LANE_BANK_POS.0, LANE_BANK_POS.1, part_z(LANE_BANK_Z))
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, part_z(BRIDGE_Z))
        + keepouts
        + carrier.translate(
            TRANSFER_CARRIER_POS.0,
            TRANSFER_CARRIER_POS.1,
            part_z(TRANSFER_CARRIER_Z),
        );
    export(OUTPUTS[14], &assembly);

    println!();
    println!("Closed media osmolality/conductivity/pH QC station:");
    println!("  Footprint:                    {DECK_X:.0}mm x {DECK_Y:.0}mm closed QC tray");
    println!(
        "  Closed sampling:              {SAMPLE_LOOP_VOLUME_ML:.1}mL sterile loop, {LOOP_VALVE_COUNT} valve seats, {SEALED_CUP_COUNT} sealed sample cups in a {CUP_ROWS}x{CUP_COLS} cartridge"
    );
    println!(
        "  Analyzer docks:               {ANALYZER_DOCK_COUNT} placeholders for osmolality, conductivity, and pH with {ANALYZER_SAMPLE_PORTS} sample and {ANALYZER_WASTE_PORTS} waste ports"
    );
    println!(
        "  Custody/QC controls:          {CAL_STANDARD_POCKETS} calibration pockets, {CAL_CUSTODY_SEAL_PADS} custody seal pads, {BARCODE_LANDS} barcode lands, {LOT_CARD_SLOTS} lot-card slots, {RFID_PADS} RFID pads, and {TEMP_POCKET_COUNT} temperature equilibration pockets at {TEMP_TARGET_C:.0}C"
    );
    println!(
        "  Release disposition:          release/hold/reject lane bank with {STATUS_TOKENS_PER_LANE} tokens per lane and {BRIDGE_SHUTTERS} custody shutters before perfusion release"
    );
    println!(
        "  Keepouts:                     front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, left scan service {LEFT_BARCODE_SERVICE_KEEP_OUT_X:.0}mm, right analyzer service {RIGHT_ANALYZER_SERVICE_KEEP_OUT_X:.0}mm, top analyzer service {TOP_ANALYZER_SERVICE_CLEARANCE_Z:.0}mm Z"
    );
    println!(
        "  Feature groups covered:       {}",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn part_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_media_qc_station_base_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let recessed_wet_basin = centered_cube(
        "closed_media_qc_station_recessed_wet_basin",
        DECK_X - 2.0 * (RIM_W + 38.0),
        302.0,
        8.0,
    )
    .translate(0.0, -42.0, DECK_Z - 4.0);
    let lane_socket = centered_cube(
        "closed_media_qc_station_release_hold_reject_lane_socket",
        LANE_BANK_X + 22.0,
        LANE_BANK_Y + 18.0,
        SOCKET_DEPTH + 1.0,
    )
    .translate(
        LANE_BANK_POS.0,
        LANE_BANK_POS.1,
        DECK_Z - SOCKET_DEPTH / 2.0,
    );
    let sample_loop_socket = centered_cube(
        "closed_media_qc_station_sample_loop_socket",
        SAMPLE_LOOP_X + 18.0,
        SAMPLE_LOOP_Y + 18.0,
        SOCKET_DEPTH + 1.0,
    )
    .translate(
        SAMPLE_LOOP_POS.0,
        SAMPLE_LOOP_POS.1,
        DECK_Z - SOCKET_DEPTH / 2.0,
    );
    let analyzer_cable_sump = centered_cube(
        "closed_media_qc_station_analyzer_cable_sump",
        780.0,
        34.0,
        DECK_Z + 2.0,
    )
    .translate(-20.0, 84.0, DECK_Z / 2.0);
    let waste_drain = centered_cylinder("closed_media_qc_station_waste_drain_port", 4.0, 42.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 + 28.0, DECK_Z - 8.0);

    deck - recessed_wet_basin
        - lane_socket
        - sample_loop_socket
        - analyzer_cable_sump
        - waste_drain
        - deck_mount_holes()
        + deck_rim()
        + zone_locator_rails()
        + deck_traceability_tick_lands()
}

fn deck_rim() -> Part {
    let front = centered_cube("closed_media_qc_station_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_media_qc_station_rear_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_media_qc_station_left_traceability_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_media_qc_station_right_analyzer_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_media_qc_station_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("closed_media_qc_station_deck_m6_clearance_{i}"),
            3.4,
            DECK_Z + 4.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("closed_media_qc_station_deck_m6_slot_{i}"),
            26.0,
            7.0,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        holes = holes + hole + slot;
    }
    holes
}

fn deck_mount_points() -> [(f64, f64); 10] {
    [
        (-(DECK_X / 2.0 - 42.0), -(DECK_Y / 2.0 - 42.0)),
        (DECK_X / 2.0 - 42.0, -(DECK_Y / 2.0 - 42.0)),
        (-(DECK_X / 2.0 - 42.0), DECK_Y / 2.0 - 42.0),
        (DECK_X / 2.0 - 42.0, DECK_Y / 2.0 - 42.0),
        (0.0, -(DECK_Y / 2.0 - 42.0)),
        (0.0, DECK_Y / 2.0 - 42.0),
        (-(DECK_X / 2.0 - 42.0), 0.0),
        (DECK_X / 2.0 - 42.0, 0.0),
        (-300.0, DECK_Y / 2.0 - 42.0),
        (300.0, DECK_Y / 2.0 - 42.0),
    ]
}

fn zone_locator_rails() -> Part {
    let mut rails = Part::empty("closed_media_qc_station_zone_locator_rails");
    for (i, (x, y, w)) in [
        (SAMPLE_LOOP_POS.0, SAMPLE_LOOP_POS.1 - 78.0, SAMPLE_LOOP_X),
        (
            CUP_CARTRIDGE_POS.0,
            CUP_CARTRIDGE_POS.1 - 78.0,
            CUP_CARTRIDGE_X,
        ),
        (TEMP_EQ_POS.0, TEMP_EQ_POS.1 - 86.0, TEMP_EQ_X),
        (
            FLUSH_MANIFOLD_POS.0,
            FLUSH_MANIFOLD_POS.1 + 78.0,
            FLUSH_MANIFOLD_X,
        ),
        (TRACE_PANEL_POS.0, TRACE_PANEL_POS.1 + 78.0, TRACE_PANEL_X),
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("closed_media_qc_station_locator_rail_{i}"),
                *w,
                8.0,
                12.0,
            )
            .translate(*x, *y, DECK_Z + 6.0);
    }
    rails
}

fn deck_traceability_tick_lands() -> Part {
    let mut ticks = Part::empty("closed_media_qc_station_deck_traceability_tick_lands");
    for i in 0..BARCODE_LANDS {
        let x = TRACE_PANEL_POS.0 - TRACE_PANEL_X / 2.0 + 24.0 + i as f64 * 26.0;
        ticks = ticks
            + centered_cube(
                format!("closed_media_qc_station_lot_tick_land_{i}"),
                16.0,
                4.0,
                4.0,
            )
            .translate(
                x,
                TRACE_PANEL_POS.1 - TRACE_PANEL_Y / 2.0 - 9.0,
                DECK_Z + 2.0,
            );
    }
    ticks
}

fn sterile_sample_loop_manifold() -> Part {
    let body = centered_cube(
        "closed_media_qc_sterile_sample_loop_manifold_body",
        SAMPLE_LOOP_X,
        SAMPLE_LOOP_Y,
        SAMPLE_LOOP_Z,
    );
    let inlet_bore = centered_cylinder(
        "closed_media_qc_sample_loop_sterile_inlet_bore",
        LOOP_CHANNEL_D / 2.0,
        SAMPLE_LOOP_X + 20.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 28.0, 0.0);
    let return_bore = centered_cylinder(
        "closed_media_qc_sample_loop_sterile_return_bore",
        LOOP_CHANNEL_D / 2.0,
        SAMPLE_LOOP_X + 20.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -28.0, 0.0);
    let u_turn = centered_cylinder(
        "closed_media_qc_sample_loop_closed_u_return_bore",
        LOOP_CHANNEL_D / 2.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SAMPLE_LOOP_X / 2.0 - 36.0, 0.0, 0.0);
    let analyzer_takeoff = centered_cylinder(
        "closed_media_qc_sample_loop_analyzer_takeoff_bore",
        LOOP_CHANNEL_D / 2.0,
        94.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-42.0, 0.0, 0.0);

    let mut valve_cuts = Part::empty("closed_media_qc_sample_loop_valve_cuts");
    let mut valve_bosses = Part::empty("closed_media_qc_sample_loop_valve_bosses");
    for valve in 0..LOOP_VALVE_COUNT {
        let x = sample_loop_valve_x(valve);
        let pocket = centered_cylinder(
            format!("closed_media_qc_sample_loop_diaphragm_valve_pocket_{valve}"),
            12.0,
            16.0,
            32,
        )
        .translate(x, 0.0, SAMPLE_LOOP_Z / 2.0 - 8.0);
        let drive_slot = centered_cube(
            format!("closed_media_qc_sample_loop_valve_drive_slot_{valve}"),
            18.0,
            SAMPLE_LOOP_Y + 4.0,
            8.0,
        )
        .translate(x, 0.0, SAMPLE_LOOP_Z / 2.0 - 8.0);
        let boss = centered_cylinder(
            format!("closed_media_qc_sample_loop_valve_boss_{valve}"),
            17.0,
            8.0,
            34,
        )
        .translate(x, 0.0, SAMPLE_LOOP_Z / 2.0 + 4.0);
        valve_cuts = valve_cuts + pocket + drive_slot;
        valve_bosses = valve_bosses + boss;
    }

    let sterile_bulkhead_in = centered_cylinder(
        "closed_media_qc_sample_loop_sterile_inlet_bulkhead",
        6.5,
        SAMPLE_LOOP_Y + 12.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(SAMPLE_LOOP_X / 2.0 - 24.0),
        SAMPLE_LOOP_Y / 2.0 - 6.0,
        0.0,
    );
    let sterile_bulkhead_out = centered_cylinder(
        "closed_media_qc_sample_loop_sterile_return_bulkhead",
        6.5,
        SAMPLE_LOOP_Y + 12.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(SAMPLE_LOOP_X / 2.0 - 24.0),
        -(SAMPLE_LOOP_Y / 2.0 - 6.0),
        0.0,
    );
    let no_touch_cover_land = centered_cube(
        "closed_media_qc_sample_loop_clear_no_touch_cover_land",
        SAMPLE_LOOP_X - 34.0,
        10.0,
        10.0,
    )
    .translate(0.0, SAMPLE_LOOP_Y / 2.0 + 5.0, SAMPLE_LOOP_Z / 2.0 - 8.0);

    body + valve_bosses + no_touch_cover_land
        - inlet_bore
        - return_bore
        - u_turn
        - analyzer_takeoff
        - valve_cuts
        - sterile_bulkhead_in
        - sterile_bulkhead_out
        + manifold_mount_bosses(
            "closed_media_qc_sample_loop",
            SAMPLE_LOOP_X,
            SAMPLE_LOOP_Y,
            SAMPLE_LOOP_Z,
        )
}

fn sample_loop_valve_x(valve: usize) -> f64 {
    -((LOOP_VALVE_COUNT as f64 - 1.0) * LOOP_VALVE_PITCH) / 2.0 + valve as f64 * LOOP_VALVE_PITCH
}

fn sealed_sample_cup_cartridge() -> Part {
    let body = centered_cube(
        "closed_media_qc_sealed_sample_cup_cartridge_body",
        CUP_CARTRIDGE_X,
        CUP_CARTRIDGE_Y,
        CUP_CARTRIDGE_Z,
    );

    let mut cup_wells = Part::empty("closed_media_qc_sealed_sample_cup_wells");
    let mut septum_rings = Part::empty("closed_media_qc_sealed_sample_cup_septum_rings");
    for cup in 0..SEALED_CUP_COUNT {
        let (x, y) = cup_position(cup);
        let well = centered_cylinder(
            format!("closed_media_qc_sealed_sample_cup_well_{cup}"),
            CUP_WELL_D / 2.0,
            CUP_CARTRIDGE_Z + 4.0,
            36,
        )
        .translate(x, y, 0.0);
        let septum = centered_cylinder(
            format!("closed_media_qc_sealed_sample_cup_septum_ring_{cup}"),
            CUP_SEPTUM_D / 2.0,
            4.0,
            36,
        )
        .translate(x, y, CUP_CARTRIDGE_Z / 2.0 + 2.0);
        let puncture_guard = centered_cube(
            format!("closed_media_qc_sealed_sample_cup_puncture_guard_{cup}"),
            20.0,
            4.0,
            8.0,
        )
        .translate(x, y + CUP_WELL_D / 2.0 + 5.0, CUP_CARTRIDGE_Z / 2.0 + 4.0);
        cup_wells = cup_wells + well;
        septum_rings = septum_rings + septum + puncture_guard;
    }

    let cartridge_key = centered_cube(
        "closed_media_qc_sealed_sample_cup_cartridge_asymmetric_key",
        34.0,
        18.0,
        CUP_CARTRIDGE_Z + 6.0,
    )
    .translate(
        CUP_CARTRIDGE_X / 2.0 - 32.0,
        -CUP_CARTRIDGE_Y / 2.0 + 22.0,
        0.0,
    );
    let pull_handle = centered_cube(
        "closed_media_qc_sealed_sample_cup_cartridge_robot_pull_handle",
        92.0,
        16.0,
        22.0,
    )
    .translate(0.0, -CUP_CARTRIDGE_Y / 2.0 - 8.0, 4.0);
    let tamper_strip = centered_cube(
        "closed_media_qc_sealed_sample_cup_cartridge_tamper_witness_strip",
        CUP_CARTRIDGE_X - 40.0,
        5.0,
        6.0,
    )
    .translate(
        0.0,
        CUP_CARTRIDGE_Y / 2.0 + 2.5,
        CUP_CARTRIDGE_Z / 2.0 - 8.0,
    );

    body + septum_rings + pull_handle + tamper_strip - cup_wells - cartridge_key
}

fn cup_position(cup: usize) -> (f64, f64) {
    let col = cup % CUP_COLS;
    let row = cup / CUP_COLS;
    let x = -((CUP_COLS as f64 - 1.0) * CUP_PITCH_X) / 2.0 + col as f64 * CUP_PITCH_X;
    let y = -((CUP_ROWS as f64 - 1.0) * CUP_PITCH_Y) / 2.0 + row as f64 * CUP_PITCH_Y;
    (x, y)
}

fn osmolality_analyzer_dock() -> Part {
    analyzer_dock(
        "osmolality",
        OSMO_DOCK_X,
        OSMO_DOCK_Y,
        OSMO_DOCK_Z,
        172.0,
        104.0,
        82.0,
    )
}

fn conductivity_analyzer_dock() -> Part {
    analyzer_dock(
        "conductivity",
        CONDUCTIVITY_DOCK_X,
        CONDUCTIVITY_DOCK_Y,
        CONDUCTIVITY_DOCK_Z,
        146.0,
        92.0,
        64.0,
    )
}

fn ph_analyzer_dock() -> Part {
    analyzer_dock("ph", PH_DOCK_X, PH_DOCK_Y, PH_DOCK_Z, 146.0, 92.0, 64.0)
}

fn analyzer_dock(
    prefix: &str,
    dock_x: f64,
    dock_y: f64,
    dock_z: f64,
    envelope_x: f64,
    envelope_y: f64,
    envelope_z: f64,
) -> Part {
    let base = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_dock_base"),
        dock_x,
        dock_y,
        22.0,
    )
    .translate(0.0, 0.0, -(dock_z / 2.0 - 11.0));
    let rear_wall = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_dock_rear_wall"),
        dock_x,
        16.0,
        dock_z,
    )
    .translate(0.0, dock_y / 2.0 - 8.0, 0.0);
    let left_rail = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_dock_left_rail"),
        16.0,
        dock_y,
        36.0,
    )
    .translate(-(dock_x / 2.0 - 8.0), 0.0, -(dock_z / 2.0 - 40.0));
    let right_rail = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_dock_right_rail"),
        16.0,
        dock_y,
        36.0,
    )
    .translate(dock_x / 2.0 - 8.0, 0.0, -(dock_z / 2.0 - 40.0));

    let analyzer_envelope = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_placeholder_envelope"),
        envelope_x,
        envelope_y,
        envelope_z,
    )
    .translate(0.0, -8.0, -(dock_z / 2.0 - 22.0) + envelope_z / 2.0);
    let sample_port = centered_cylinder(
        format!("closed_media_qc_{prefix}_analyzer_sample_port"),
        4.0,
        dock_y + 18.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-36.0, 0.0, -(dock_z / 2.0 - 28.0));
    let waste_port = centered_cylinder(
        format!("closed_media_qc_{prefix}_analyzer_waste_port"),
        4.5,
        dock_y + 18.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(36.0, 0.0, -(dock_z / 2.0 - 28.0));
    let cable_gutter = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_cable_gutter"),
        envelope_x - 18.0,
        12.0,
        14.0,
    )
    .translate(0.0, dock_y / 2.0 - 20.0, dock_z / 2.0 - 24.0);

    let datum_pins = analyzer_datum_pins(prefix, dock_x, dock_y, dock_z);
    let drip_lip = centered_cube(
        format!("closed_media_qc_{prefix}_analyzer_front_drip_lip"),
        dock_x - 36.0,
        8.0,
        14.0,
    )
    .translate(0.0, -(dock_y / 2.0 + 4.0), -(dock_z / 2.0 - 20.0));

    base + rear_wall + left_rail + right_rail + datum_pins + drip_lip
        - analyzer_envelope
        - sample_port
        - waste_port
        - cable_gutter
}

fn analyzer_datum_pins(prefix: &str, dock_x: f64, dock_y: f64, dock_z: f64) -> Part {
    let mut pins = Part::empty(format!("closed_media_qc_{prefix}_analyzer_datum_pins"));
    for (i, (x, y)) in [
        (-(dock_x / 2.0 - 34.0), -(dock_y / 2.0 - 30.0)),
        (dock_x / 2.0 - 34.0, -(dock_y / 2.0 - 30.0)),
        (-(dock_x / 2.0 - 34.0), dock_y / 2.0 - 34.0),
        (dock_x / 2.0 - 34.0, dock_y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_media_qc_{prefix}_analyzer_datum_socket_{i}"),
                3.0,
                20.0,
                20,
            )
            .translate(*x, *y, -(dock_z / 2.0 - 20.0));
    }
    pins
}

fn calibration_standard_custody_pockets() -> Part {
    let body = centered_cube(
        "closed_media_qc_calibration_standard_custody_block",
        CAL_BANK_X,
        CAL_BANK_Y,
        CAL_BANK_Z,
    );
    let custody_lid_ledge = centered_cube(
        "closed_media_qc_calibration_standard_locking_lid_ledge",
        CAL_BANK_X - 26.0,
        10.0,
        8.0,
    )
    .translate(0.0, CAL_BANK_Y / 2.0 + 5.0, CAL_BANK_Z / 2.0 - 8.0);

    let mut cuts = Part::empty("closed_media_qc_calibration_standard_custody_cuts");
    let mut pads = Part::empty("closed_media_qc_calibration_standard_custody_pads");
    for pocket in 0..CAL_STANDARD_POCKETS {
        let x = calibration_pocket_x(pocket);
        let y = if pocket % 2 == 0 { 26.0 } else { -26.0 };
        let pocket_cut = centered_cylinder(
            format!("closed_media_qc_calibration_standard_pocket_{pocket}"),
            11.0,
            CAL_BANK_Z + 4.0,
            32,
        )
        .translate(x, y, 0.0);
        let custody_seal = centered_cube(
            format!("closed_media_qc_calibration_standard_tamper_seal_pad_{pocket}"),
            24.0,
            8.0,
            5.0,
        )
        .translate(x, y + 20.0, CAL_BANK_Z / 2.0 + 2.5);
        cuts = cuts + pocket_cut;
        pads = pads + custody_seal;
    }

    let certificate_slot = centered_cube(
        "closed_media_qc_calibration_standard_certificate_card_slot",
        96.0,
        8.0,
        22.0,
    )
    .translate(CAL_BANK_X / 2.0 - 64.0, -(CAL_BANK_Y / 2.0 - 12.0), 6.0);
    let quarantine_lip = centered_cube(
        "closed_media_qc_calibration_standard_quarantine_return_lip",
        88.0,
        12.0,
        18.0,
    )
    .translate(-(CAL_BANK_X / 2.0 - 62.0), -(CAL_BANK_Y / 2.0 + 6.0), -4.0);

    body + custody_lid_ledge + pads + quarantine_lip - cuts - certificate_slot
}

fn calibration_pocket_x(pocket: usize) -> f64 {
    -((CAL_STANDARD_POCKETS as f64 - 1.0) * CAL_POCKET_PITCH_X) / 2.0
        + pocket as f64 * CAL_POCKET_PITCH_X
}

fn flush_waste_routing_manifold() -> Part {
    let body = centered_cube(
        "closed_media_qc_flush_waste_routing_manifold_body",
        FLUSH_MANIFOLD_X,
        FLUSH_MANIFOLD_Y,
        FLUSH_MANIFOLD_Z,
    );
    let flush_bus = centered_cylinder(
        "closed_media_qc_flush_waste_clean_flush_bus",
        FLUSH_LINE_D / 2.0,
        FLUSH_MANIFOLD_X + 18.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 24.0, 6.0);
    let waste_bus = centered_cylinder(
        "closed_media_qc_flush_waste_waste_bus",
        WASTE_LINE_D / 2.0,
        FLUSH_MANIFOLD_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -24.0, -6.0);

    let mut valve_cuts = Part::empty("closed_media_qc_flush_waste_valve_cuts");
    let mut valve_labels = Part::empty("closed_media_qc_flush_waste_valve_labels");
    for valve in 0..FLUSH_VALVE_COUNT {
        let x = flush_valve_x(valve);
        let selector = centered_cylinder(
            format!("closed_media_qc_flush_waste_selector_valve_{valve}"),
            10.0,
            14.0,
            28,
        )
        .translate(x, 0.0, FLUSH_MANIFOLD_Z / 2.0 - 7.0);
        let sample_drop = centered_cylinder(
            format!("closed_media_qc_flush_waste_sample_drop_{valve}"),
            LOOP_CHANNEL_D / 2.0,
            FLUSH_MANIFOLD_Z + 6.0,
            24,
        )
        .translate(x, 0.0, 0.0);
        let label = centered_cube(
            format!("closed_media_qc_flush_waste_valve_label_land_{valve}"),
            32.0,
            5.0,
            6.0,
        )
        .translate(
            x,
            -(FLUSH_MANIFOLD_Y / 2.0 + 2.5),
            FLUSH_MANIFOLD_Z / 2.0 - 9.0,
        );
        valve_cuts = valve_cuts + selector + sample_drop;
        valve_labels = valve_labels + label;
    }

    let flush_inlet = centered_cylinder(
        "closed_media_qc_flush_waste_flush_inlet_bulkhead",
        5.0,
        FLUSH_MANIFOLD_Y + 12.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(FLUSH_MANIFOLD_X / 2.0 - 28.0), 0.0, 6.0);
    let waste_outlet = centered_cylinder(
        "closed_media_qc_flush_waste_sealed_waste_outlet_bulkhead",
        7.0,
        FLUSH_MANIFOLD_Y + 12.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FLUSH_MANIFOLD_X / 2.0 - 28.0, 0.0, -6.0);

    body + valve_labels - flush_bus - waste_bus - valve_cuts - flush_inlet - waste_outlet
        + manifold_mount_bosses(
            "closed_media_qc_flush_waste",
            FLUSH_MANIFOLD_X,
            FLUSH_MANIFOLD_Y,
            FLUSH_MANIFOLD_Z,
        )
}

fn flush_valve_x(valve: usize) -> f64 {
    -((FLUSH_VALVE_COUNT as f64 - 1.0) * 52.0) / 2.0 + valve as f64 * 52.0
}

fn barcode_lot_traceability_panel() -> Part {
    let panel = centered_cube(
        "closed_media_qc_barcode_lot_traceability_panel_body",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut cuts = Part::empty("closed_media_qc_barcode_lot_traceability_panel_cuts");
    let mut raised_lands = Part::empty("closed_media_qc_barcode_lot_traceability_panel_lands");

    for land in 0..BARCODE_LANDS {
        let x = -TRACE_PANEL_X / 2.0 + 24.0 + (land % 5) as f64 * 54.0;
        let y = 28.0 - (land / 5) as f64 * 46.0;
        raised_lands = raised_lands
            + centered_cube(
                format!("closed_media_qc_barcode_lot_barcode_land_{land}"),
                42.0,
                18.0,
                5.0,
            )
            .translate(x, y, TRACE_PANEL_Z / 2.0 + 2.5);
    }

    for slot in 0..LOT_CARD_SLOTS {
        let x = -90.0 + slot as f64 * 60.0;
        let card_slot = centered_cube(
            format!("closed_media_qc_barcode_lot_card_slot_{slot}"),
            48.0,
            7.0,
            TRACE_PANEL_Z + 4.0,
        )
        .translate(x, -(TRACE_PANEL_Y / 2.0 - 14.0), 0.0);
        cuts = cuts + card_slot;
    }

    for pad in 0..RFID_PADS {
        let x = -90.0 + pad as f64 * 60.0;
        raised_lands = raised_lands
            + centered_cube(
                format!("closed_media_qc_barcode_lot_rfid_pad_{pad}"),
                34.0,
                22.0,
                4.0,
            )
            .translate(x, TRACE_PANEL_Y / 2.0 - 18.0, TRACE_PANEL_Z / 2.0 + 2.0);
    }

    panel + raised_lands - cuts + fiducial_pair("barcode_lot_traceability", TRACE_PANEL_X)
}

fn temperature_equilibration_pocket() -> Part {
    let body = centered_cube(
        "closed_media_qc_temperature_equilibration_block",
        TEMP_EQ_X,
        TEMP_EQ_Y,
        TEMP_EQ_Z,
    );

    let mut sample_pockets =
        Part::empty("closed_media_qc_temperature_equilibration_sample_pockets");
    let mut pocket_rims = Part::empty("closed_media_qc_temperature_equilibration_pocket_rims");
    for pocket in 0..TEMP_POCKET_COUNT {
        let x = -((TEMP_POCKET_COUNT as f64 - 1.0) * 48.0) / 2.0 + pocket as f64 * 48.0;
        let cup = centered_cylinder(
            format!("closed_media_qc_temperature_equilibration_sealed_cup_pocket_{pocket}"),
            13.0,
            TEMP_EQ_Z + 4.0,
            32,
        )
        .translate(x, -18.0, 0.0);
        let rim = centered_cylinder(
            format!("closed_media_qc_temperature_equilibration_cup_stop_rim_{pocket}"),
            17.0,
            5.0,
            32,
        )
        .translate(x, -18.0, TEMP_EQ_Z / 2.0 + 2.5);
        sample_pockets = sample_pockets + cup;
        pocket_rims = pocket_rims + rim;
    }

    let water_jacket_in = centered_cylinder(
        "closed_media_qc_temperature_equilibration_water_jacket_in",
        4.0,
        TEMP_EQ_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 34.0, -8.0);
    let water_jacket_out = centered_cylinder(
        "closed_media_qc_temperature_equilibration_water_jacket_out",
        4.0,
        TEMP_EQ_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 54.0, 8.0);
    let temp_sensor_socket = centered_cylinder(
        "closed_media_qc_temperature_equilibration_temperature_probe_socket",
        3.0,
        TEMP_EQ_Y + 10.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TEMP_EQ_X / 2.0 - 34.0, 0.0, 12.0);

    let condensation_gutter = centered_cube(
        "closed_media_qc_temperature_equilibration_condensation_gutter",
        TEMP_EQ_X - 28.0,
        10.0,
        8.0,
    )
    .translate(0.0, -(TEMP_EQ_Y / 2.0 + 5.0), -10.0);

    body + pocket_rims + condensation_gutter
        - sample_pockets
        - water_jacket_in
        - water_jacket_out
        - temp_sensor_socket
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_media_qc_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    );
    let mut lane_cuts = Part::empty("closed_media_qc_release_hold_reject_lane_cuts");
    let mut lane_labels = Part::empty("closed_media_qc_release_hold_reject_lane_labels");

    for lane in 0..STATUS_LANE_COUNT {
        let x = status_lane_x(lane);
        let pocket = centered_cube(
            format!(
                "closed_media_qc_{}_lane_token_trough",
                status_lane_name(lane)
            ),
            lane_width(),
            LANE_BANK_Y - 22.0,
            16.0,
        )
        .translate(x, 0.0, LANE_BANK_Z / 2.0 - 8.0);
        lane_cuts = lane_cuts + pocket;

        for token in 0..STATUS_TOKENS_PER_LANE {
            let token_x = x - lane_width() / 2.0 + 26.0 + token as f64 * 26.0;
            let token_stop = centered_cube(
                format!(
                    "closed_media_qc_{}_lane_token_stop_{}",
                    status_lane_name(lane),
                    token
                ),
                4.0,
                LANE_BANK_Y - 28.0,
                10.0,
            )
            .translate(token_x, 0.0, LANE_BANK_Z / 2.0 + 5.0);
            lane_labels = lane_labels + token_stop;
        }
    }

    let left_separator = lane_separator(-lane_width() / 2.0 - LANE_CLEAR_GAP / 2.0);
    let right_separator = lane_separator(lane_width() / 2.0 + LANE_CLEAR_GAP / 2.0);

    base + lane_labels + left_separator + right_separator - lane_cuts
}

fn lane_width() -> f64 {
    (LANE_BANK_X - 4.0 * LANE_CLEAR_GAP) / 3.0
}

fn status_lane_x(lane: usize) -> f64 {
    (lane as f64 - 1.0) * (lane_width() + LANE_CLEAR_GAP)
}

fn status_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn lane_separator(x: f64) -> Part {
    centered_cube(
        format!("closed_media_qc_release_hold_reject_lane_separator_{x:.0}"),
        10.0,
        LANE_BANK_Y,
        LANE_BANK_Z + 18.0,
    )
    .translate(x, 0.0, 9.0)
}

fn custody_interlock_bridge() -> Part {
    let left_post = centered_cube(
        "closed_media_qc_custody_interlock_bridge_left_post",
        22.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(-(BRIDGE_X / 2.0 - 16.0), 0.0, 0.0);
    let right_post = centered_cube(
        "closed_media_qc_custody_interlock_bridge_right_post",
        22.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 16.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_media_qc_custody_interlock_bridge_status_beam",
        BRIDGE_X,
        BRIDGE_Y,
        24.0,
    )
    .translate(0.0, 0.0, BRIDGE_Z / 2.0 - 12.0);

    let mut shutter_windows = Part::empty("closed_media_qc_custody_interlock_bridge_shutters");
    for shutter in 0..BRIDGE_SHUTTERS {
        let x = status_lane_x(shutter);
        shutter_windows = shutter_windows
            + centered_cube(
                format!("closed_media_qc_custody_interlock_shutter_window_{shutter}"),
                lane_width() - 30.0,
                BRIDGE_Y + 4.0,
                34.0,
            )
            .translate(x, 0.0, BRIDGE_Z / 2.0 - 30.0);
    }

    let badge_rail = centered_cube(
        "closed_media_qc_custody_interlock_perfusion_release_badge_rail",
        BRIDGE_X - 92.0,
        8.0,
        12.0,
    )
    .translate(0.0, -(BRIDGE_Y / 2.0 + 4.0), BRIDGE_Z / 2.0 - 52.0);

    left_post + right_post + beam + badge_rail - shutter_windows
}

fn sealed_transfer_carrier() -> Part {
    let body = centered_cube(
        "closed_media_qc_sealed_transfer_carrier_body",
        TRANSFER_CARRIER_X,
        TRANSFER_CARRIER_Y,
        TRANSFER_CARRIER_Z,
    );
    let mut sockets = Part::empty("closed_media_qc_sealed_transfer_carrier_socket_cuts");
    for socket in 0..TRANSFER_CARRIER_SOCKETS {
        let x = -((TRANSFER_CARRIER_SOCKETS as f64 - 1.0) * 42.0) / 2.0 + socket as f64 * 42.0;
        sockets = sockets
            + centered_cylinder(
                format!("closed_media_qc_sealed_transfer_carrier_cup_socket_{socket}"),
                9.0,
                TRANSFER_CARRIER_Z + 4.0,
                28,
            )
            .translate(x, 0.0, 0.0);
    }
    let robot_tab = centered_cube(
        "closed_media_qc_sealed_transfer_carrier_robot_grip_tab",
        72.0,
        12.0,
        20.0,
    )
    .translate(0.0, -TRANSFER_CARRIER_Y / 2.0 - 6.0, 0.0);
    let sterile_lid_land = centered_cube(
        "closed_media_qc_sealed_transfer_carrier_sterile_lid_land",
        TRANSFER_CARRIER_X - 32.0,
        5.0,
        5.0,
    )
    .translate(
        0.0,
        TRANSFER_CARRIER_Y / 2.0 + 2.5,
        TRANSFER_CARRIER_Z / 2.0 - 6.0,
    );

    body + robot_tab + sterile_lid_land - sockets
}

fn robot_service_keepouts() -> Part {
    let front_robot_bar = centered_cube(
        "closed_media_qc_front_robot_keepout_bar",
        DECK_X - 90.0,
        10.0,
        18.0,
    )
    .translate(0.0, -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y, DECK_Z + 9.0);
    let front_robot_posts = keepout_posts(
        "front_robot",
        DECK_X - 120.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        108.0,
    );

    let rear_service_bar = centered_cube(
        "closed_media_qc_rear_service_keepout_bar",
        DECK_X - 90.0,
        10.0,
        18.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y, DECK_Z + 9.0);
    let rear_service_posts = keepout_posts(
        "rear_service",
        DECK_X - 120.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0,
        REAR_SERVICE_KEEP_OUT_Y,
        122.0,
    );

    let left_scan_clearance = centered_cube(
        "closed_media_qc_left_barcode_service_keepout_gauge",
        10.0,
        DECK_Y - 120.0,
        76.0,
    )
    .translate(
        -DECK_X / 2.0 - LEFT_BARCODE_SERVICE_KEEP_OUT_X,
        -10.0,
        DECK_Z + 38.0,
    );
    let right_analyzer_clearance = centered_cube(
        "closed_media_qc_right_analyzer_service_keepout_gauge",
        10.0,
        DECK_Y - 120.0,
        96.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_ANALYZER_SERVICE_KEEP_OUT_X,
        -10.0,
        DECK_Z + 48.0,
    );
    let top_service_gauge = centered_cube(
        "closed_media_qc_top_analyzer_service_clearance_gauge",
        520.0,
        36.0,
        12.0,
    )
    .translate(-40.0, 96.0, DECK_Z + TOP_ANALYZER_SERVICE_CLEARANCE_Z);

    front_robot_bar
        + front_robot_posts
        + rear_service_bar
        + rear_service_posts
        + left_scan_clearance
        + right_analyzer_clearance
        + top_service_gauge
}

fn keepout_posts(prefix: &str, width: f64, center_y: f64, depth: f64, height: f64) -> Part {
    let mut posts = Part::empty(format!("closed_media_qc_{prefix}_keepout_posts"));
    for (i, x) in [-(width / 2.0), width / 2.0].iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("closed_media_qc_{prefix}_keepout_post_{i}"),
                12.0,
                12.0,
                height,
            )
            .translate(*x, center_y - depth / 2.0, DECK_Z + height / 2.0)
            + centered_cube(
                format!("closed_media_qc_{prefix}_keepout_rear_post_{i}"),
                12.0,
                12.0,
                height,
            )
            .translate(*x, center_y + depth / 2.0, DECK_Z + height / 2.0);
    }
    posts
}

fn manifold_mount_bosses(prefix: &str, span_x: f64, span_y: f64, span_z: f64) -> Part {
    let mut bosses = Part::empty(format!("{prefix}_mount_bosses"));
    for (i, (x, y)) in [
        (-(span_x / 2.0 - 22.0), -(span_y / 2.0 - 18.0)),
        (span_x / 2.0 - 22.0, -(span_y / 2.0 - 18.0)),
        (-(span_x / 2.0 - 22.0), span_y / 2.0 - 18.0),
        (span_x / 2.0 - 22.0, span_y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("{prefix}_mount_boss_{i}"), 8.0, 6.0, 24).translate(
            *x,
            *y,
            span_z / 2.0 + 3.0,
        );
        let screw = centered_cylinder(format!("{prefix}_mount_screw_clearance_{i}"), 2.2, 9.0, 20)
            .translate(*x, *y, span_z / 2.0 + 3.0);
        bosses = bosses + boss - screw;
    }
    bosses
}

fn fiducial_pair(prefix: &str, span_x: f64) -> Part {
    let left = fiducial_disc(&format!("closed_media_qc_{prefix}_left_fiducial")).translate(
        -(span_x / 2.0 - 22.0),
        0.0,
        TRACE_PANEL_Z / 2.0 + 3.0,
    );
    let right = fiducial_disc(&format!("closed_media_qc_{prefix}_right_fiducial")).translate(
        span_x / 2.0 - 22.0,
        0.0,
        TRACE_PANEL_Z / 2.0 + 3.0,
    );
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 8.0, 3.0, 28)
        - centered_cylinder(format!("{name}_center"), 2.2, 4.0, 20)
}

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn primary_footprints() -> [Footprint; 11] {
    [
        Footprint {
            name: "sterile_sample_loop_manifold",
            center: SAMPLE_LOOP_POS,
            x: SAMPLE_LOOP_X,
            y: SAMPLE_LOOP_Y,
        },
        Footprint {
            name: "sealed_sample_cup_cartridge",
            center: CUP_CARTRIDGE_POS,
            x: CUP_CARTRIDGE_X,
            y: CUP_CARTRIDGE_Y,
        },
        Footprint {
            name: "temperature_equilibration_pocket",
            center: TEMP_EQ_POS,
            x: TEMP_EQ_X,
            y: TEMP_EQ_Y,
        },
        Footprint {
            name: "osmolality_analyzer_dock",
            center: OSMO_DOCK_POS,
            x: OSMO_DOCK_X,
            y: OSMO_DOCK_Y,
        },
        Footprint {
            name: "conductivity_analyzer_dock",
            center: CONDUCTIVITY_DOCK_POS,
            x: CONDUCTIVITY_DOCK_X,
            y: CONDUCTIVITY_DOCK_Y,
        },
        Footprint {
            name: "ph_analyzer_dock",
            center: PH_DOCK_POS,
            x: PH_DOCK_X,
            y: PH_DOCK_Y,
        },
        Footprint {
            name: "calibration_standard_custody_pockets",
            center: CAL_BANK_POS,
            x: CAL_BANK_X,
            y: CAL_BANK_Y,
        },
        Footprint {
            name: "flush_waste_routing_manifold",
            center: FLUSH_MANIFOLD_POS,
            x: FLUSH_MANIFOLD_X,
            y: FLUSH_MANIFOLD_Y,
        },
        Footprint {
            name: "barcode_lot_traceability_panel",
            center: TRACE_PANEL_POS,
            x: TRACE_PANEL_X,
            y: TRACE_PANEL_Y,
        },
        Footprint {
            name: "sealed_transfer_carrier",
            center: TRANSFER_CARRIER_POS,
            x: TRANSFER_CARRIER_X,
            y: TRANSFER_CARRIER_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: LANE_BANK_POS,
            x: LANE_BANK_X,
            y: LANE_BANK_Y,
        },
    ]
}

fn assert_layout() {
    let footprints = primary_footprints();
    for footprint in footprints {
        assert!(
            fits_on_deck(footprint.center, footprint.x, footprint.y, 12.0),
            "{} must fit on the base tray",
            footprint.name
        );
    }

    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            let a = footprints[left];
            let b = footprints[right];
            assert!(
                !rects_overlap(a, b),
                "{} must not overlap {}",
                a.name,
                b.name
            );
        }
    }
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

fn rects_overlap(a: Footprint, b: Footprint) -> bool {
    let ax0 = a.center.0 - a.x / 2.0;
    let ax1 = a.center.0 + a.x / 2.0;
    let ay0 = a.center.1 - a.y / 2.0;
    let ay1 = a.center.1 + a.y / 2.0;
    let bx0 = b.center.0 - b.x / 2.0;
    let bx1 = b.center.0 + b.x / 2.0;
    let by0 = b.center.1 - b.y / 2.0;
    let by1 = b.center.1 + b.y / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_contract_is_scoped_and_unique() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 15);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(
            |path| path.starts_with("output/closed_media_osmolality_conductivity_qc_station_")
        ));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_closed_media_qc_intent() {
        for feature in [
            "sterile_sample_loop",
            "sealed_sample_cup_cartridge",
            "osmolality_analyzer_dock",
            "conductivity_analyzer_dock",
            "ph_analyzer_dock",
            "calibration_standard_custody_pockets",
            "flush_waste_routing",
            "barcode_lot_traceability",
            "temperature_equilibration_pocket",
            "release_lane",
            "hold_lane",
            "reject_lane",
            "robot_keepout",
            "service_keepout",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn sample_loop_and_cup_cartridge_support_batch_qc() {
        assert_eq!(LOOP_VALVE_COUNT, 5);
        assert!(SAMPLE_LOOP_VOLUME_ML <= 2.0);
        assert_eq!(SEALED_CUP_COUNT, CUP_COLS * CUP_ROWS);
        assert!(SEALED_CUP_COUNT >= ANALYZER_DOCK_COUNT * 4);
        assert!(CUP_PITCH_X > CUP_WELL_D + 16.0);
        assert!(CUP_PITCH_Y > CUP_WELL_D + 24.0);
    }

    #[test]
    fn analyzer_and_calibration_custody_counts_are_complete() {
        assert_eq!(ANALYZER_DOCK_COUNT, 3);
        assert_eq!(ANALYZER_SAMPLE_PORTS, 3);
        assert_eq!(ANALYZER_WASTE_PORTS, 3);
        assert_eq!(CAL_STANDARD_POCKETS, 8);
        assert_eq!(CAL_CUSTODY_SEAL_PADS, CAL_STANDARD_POCKETS);
        assert!(BARCODE_LANDS >= CAL_STANDARD_POCKETS);
        assert!(LOT_CARD_SLOTS >= ANALYZER_DOCK_COUNT);
        assert!(RFID_PADS >= ANALYZER_DOCK_COUNT);
    }

    #[test]
    fn release_hold_reject_lanes_are_physical_and_separated() {
        assert_eq!(STATUS_LANE_COUNT, 3);
        assert_eq!(BRIDGE_SHUTTERS, 3);
        assert!(lane_width() > STATUS_TOKENS_PER_LANE as f64 * 24.0);
        assert!(LANE_CLEAR_GAP >= 20.0);
        assert!(status_lane_x(0) < status_lane_x(1));
        assert!(status_lane_x(1) < status_lane_x(2));
        assert!(LANE_BANK_POS.1 < CAL_BANK_POS.1);
    }

    #[test]
    fn primary_station_layout_fits_without_overlap() {
        assert_layout();
        assert!(primary_footprints()
            .iter()
            .any(|footprint| footprint.name == "release_hold_reject_lanes"));
    }

    #[test]
    fn robot_and_service_keepouts_reserve_access() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 200.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 150.0);
        assert!(LEFT_BARCODE_SERVICE_KEEP_OUT_X >= 100.0);
        assert!(RIGHT_ANALYZER_SERVICE_KEEP_OUT_X >= 120.0);
        assert!(TOP_ANALYZER_SERVICE_CLEARANCE_Z > OSMO_DOCK_Z + DECK_Z + 110.0);
    }

    #[test]
    fn temperature_equilibration_is_defined_before_analysis() {
        assert_eq!(TEMP_POCKET_COUNT, 4);
        assert!((TEMP_TARGET_C - 37.0).abs() < 0.01);
        assert!(TEMP_EQ_POS.1 > OSMO_DOCK_POS.1);
        assert!(TRANSFER_CARRIER_POS.1 < CUP_CARTRIDGE_POS.1);
        assert!(TRANSFER_CARRIER_POS.1 > OSMO_DOCK_POS.1);
    }
}
