use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette/module barrier pressure equalization station.
//
// Intent:
// - Validate closed-system pressure equalization behavior before docking and
//   undocking a tissue-chip cassette between incubator, isolator, and perfusion
//   module interfaces.
// - Keep the cassette dock, module-side surrogate dock, dual equalization
//   manifold, sterile filtered vent holder, pressure transducer bosses,
//   backflow dye witness, bypass/relief route, gasket compression witnesses,
//   custody lands, release/hold/reject gates, evidence camera bridge, and
//   robot/service keepouts on one traceable validation fixture.
// - Represent process-state markings as CSG rails, token pockets, witness lands,
//   and raised code bars so the STL exports remain self describing.
//
// Mechanical packaging/validation hardware only. This is not a pressure-rated
// vessel, biosafety containment claim, leak-test protocol, or sterile barrier
// specification.

const PREFIX: &str = "closed_cassette_module_barrier_pressure_equalization_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_module_barrier_pressure_equalization_station_containment_deck.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_cassette_dock.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_module_surrogate_dock.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_dual_equalization_manifold.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_sterile_filtered_vent_holder.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_pressure_transducer_bosses.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_backflow_dye_witness_channel.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_relief_bypass_route.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_gasket_compression_witness_lands.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_custody_and_gate_lands.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_camera_bridge_keepouts.stl",
    "output/closed_cassette_module_barrier_pressure_equalization_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "cassette_dock",
    "module_surrogate_dock",
    "dual_equalization_manifold",
    "sterile_filtered_vent_holder",
    "pressure_transducer_bosses",
    "backflow_dye_witness_channel",
    "relief_bypass_route",
    "gasket_compression_witness_lands",
    "custody_and_gate_lands",
    "camera_bridge_keepouts",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 1040.0;
const DECK_Z: f64 = 20.0;
const CURB_W: f64 = 18.0;
const CURB_Z: f64 = 46.0;
const SOCKET_Z: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DATUM_COUNT: usize = 4;
const SPILL_GUTTERS: usize = 5;

const CASSETTE_DOCK_X: f64 = 430.0;
const CASSETTE_DOCK_Y: f64 = 286.0;
const CASSETTE_DOCK_Z: f64 = 54.0;
const CASSETTE_DOCK_POS: (f64, f64) = (-335.0, 170.0);
const CASSETTE_SLOTS: usize = 2;
const CASSETTE_SLOT_X: f64 = 158.0;
const CASSETTE_SLOT_Y: f64 = 116.0;
const CASSETTE_SLOT_Z: f64 = 20.0;
const CASSETTE_SLOT_PITCH_X: f64 = 198.0;
const CASSETTE_STOP_COUNT: usize = 8;

const MODULE_DOCK_X: f64 = 430.0;
const MODULE_DOCK_Y: f64 = 286.0;
const MODULE_DOCK_Z: f64 = 58.0;
const MODULE_DOCK_POS: (f64, f64) = (335.0, 170.0);
const MODULE_PORTS: usize = 6;
const MODULE_PORT_D: f64 = 28.0;
const MODULE_PORT_PITCH_X: f64 = 60.0;
const MODULE_FACE_RAILS: usize = 4;

const MANIFOLD_X: f64 = 920.0;
const MANIFOLD_Y: f64 = 148.0;
const MANIFOLD_Z: f64 = 52.0;
const MANIFOLD_POS: (f64, f64) = (0.0, -50.0);
const EQUALIZATION_LANES: usize = 2;
const BALANCE_BRANCHES_PER_LANE: usize = 6;
const MANIFOLD_BRANCH_PITCH_X: f64 = 66.0;
const MANIFOLD_LANE_PITCH_Y: f64 = 62.0;
const MANIFOLD_CHANNEL_W: f64 = 13.0;
const MANIFOLD_CHANNEL_Z: f64 = 14.0;
const RAMP_TOKENS: usize = 7;

const VENT_HOLDER_X: f64 = 292.0;
const VENT_HOLDER_Y: f64 = 188.0;
const VENT_HOLDER_Z: f64 = 72.0;
const VENT_HOLDER_POS: (f64, f64) = (-445.0, -232.0);
const FILTER_CARTRIDGES: usize = 4;
const FILTER_D: f64 = 38.0;
const FILTER_PITCH_X: f64 = 62.0;

const PRESSURE_BOSS_X: f64 = 318.0;
const PRESSURE_BOSS_Y: f64 = 188.0;
const PRESSURE_BOSS_Z: f64 = 54.0;
const PRESSURE_BOSS_POS: (f64, f64) = (-98.0, -232.0);
const TRANSDUCER_BOSSES: usize = 8;
const TRANSDUCER_COLS: usize = 4;
const TRANSDUCER_D: f64 = 26.0;
const TRANSDUCER_PITCH_X: f64 = 66.0;
const TRANSDUCER_PITCH_Y: f64 = 70.0;

const DYE_WITNESS_X: f64 = 342.0;
const DYE_WITNESS_Y: f64 = 188.0;
const DYE_WITNESS_Z: f64 = 34.0;
const DYE_WITNESS_POS: (f64, f64) = (260.0, -232.0);
const DYE_CHANNELS: usize = 6;
const DYE_WELL_COUNT: usize = 8;

const BYPASS_X: f64 = 278.0;
const BYPASS_Y: f64 = 188.0;
const BYPASS_Z: f64 = 46.0;
const BYPASS_POS: (f64, f64) = (570.0, -232.0);
const BYPASS_BRANCHES: usize = 3;
const RELIEF_TOKEN_COUNT: usize = 5;

const GASKET_X: f64 = 704.0;
const GASKET_Y: f64 = 116.0;
const GASKET_Z: f64 = 18.0;
const GASKET_POS: (f64, f64) = (0.0, 380.0);
const GASKET_LANDS: usize = 12;
const COMPRESSION_TICKS_PER_LAND: usize = 3;

const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 112.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (-335.0, -354.0);
const BARCODE_LANDS: usize = 4;
const RFID_LANDS: usize = 2;

const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 112.0;
const GATE_Z: f64 = 42.0;
const GATE_POS: (f64, f64) = (335.0, -430.0);
const GATE_LANES: usize = 3;
const GATE_TOKENS_PER_LANE: usize = 4;

const BRIDGE_X: f64 = 1010.0;
const BRIDGE_Y: f64 = 86.0;
const BRIDGE_Z: f64 = 120.0;
const BRIDGE_POS: (f64, f64) = (0.0, 268.0);
const KEEP_OUT_RAIL_Z: f64 = 7.0;
const CAMERA_WINDOWS: usize = 3;
const ROBOT_KEEP_OUT_X: f64 = 1110.0;
const ROBOT_KEEP_OUT_Y: f64 = 118.0;
const SERVICE_KEEP_OUT_X: f64 = 190.0;
const SERVICE_KEEP_OUT_Y: f64 = 650.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let cassette = cassette_dock();
    export(OUTPUTS[1], &cassette);

    let module = module_surrogate_dock();
    export(OUTPUTS[2], &module);

    let manifold = dual_equalization_manifold();
    export(OUTPUTS[3], &manifold);

    let vent = sterile_filtered_vent_holder();
    export(OUTPUTS[4], &vent);

    let pressure = pressure_transducer_bosses();
    export(OUTPUTS[5], &pressure);

    let dye = backflow_dye_witness_channel();
    export(OUTPUTS[6], &dye);

    let bypass = relief_bypass_route();
    export(OUTPUTS[7], &bypass);

    let gasket = gasket_compression_witness_lands();
    export(OUTPUTS[8], &gasket);

    let custody = custody_and_gate_lands();
    export(OUTPUTS[9], &custody);

    let bridge = camera_bridge_keepouts();
    export(OUTPUTS[10], &bridge);

    let assembly = containment_deck()
        + cassette.translate(
            CASSETTE_DOCK_POS.0,
            CASSETTE_DOCK_POS.1,
            insert_z(CASSETTE_DOCK_Z),
        )
        + module.translate(
            MODULE_DOCK_POS.0,
            MODULE_DOCK_POS.1,
            insert_z(MODULE_DOCK_Z),
        )
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, insert_z(MANIFOLD_Z))
        + vent.translate(
            VENT_HOLDER_POS.0,
            VENT_HOLDER_POS.1,
            insert_z(VENT_HOLDER_Z),
        )
        + pressure.translate(
            PRESSURE_BOSS_POS.0,
            PRESSURE_BOSS_POS.1,
            insert_z(PRESSURE_BOSS_Z),
        )
        + dye.translate(
            DYE_WITNESS_POS.0,
            DYE_WITNESS_POS.1,
            insert_z(DYE_WITNESS_Z),
        )
        + bypass.translate(BYPASS_POS.0, BYPASS_POS.1, insert_z(BYPASS_Z))
        + gasket.translate(GASKET_POS.0, GASKET_POS.1, insert_z(GASKET_Z))
        + custody.translate(0.0, 0.0, insert_z(GATE_Z))
        + bridge;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette/module barrier pressure equalization station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {SPILL_GUTTERS} spill/witness gutters"
    );
    println!(
        "  Docks:                  {CASSETTE_SLOTS} cassette nests, {MODULE_PORTS} module surrogate ports, and {MODULE_FACE_RAILS} face alignment rails"
    );
    println!(
        "  Equalization path:      {EQUALIZATION_LANES} manifold lanes x {BALANCE_BRANCHES_PER_LANE} branches with {RAMP_TOKENS} ramp tokens"
    );
    println!(
        "  Pressure/vent evidence: {FILTER_CARTRIDGES} filtered vent cartridges, {TRANSDUCER_BOSSES} transducer bosses, {DYE_CHANNELS} dye channels, {DYE_WELL_COUNT} witness wells"
    );
    println!(
        "  Release controls:       {GASKET_LANDS} gasket compression lands, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {GATE_LANES} release/hold/reject gates"
    );
    println!(
        "  Evidence/keepout:       {CAMERA_WINDOWS} camera sight windows plus robot and service keepout frames"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn containment_deck() -> Part {
    let floor = centered_cube(name("containment_deck_floor"), STATION_X, STATION_Y, DECK_Z)
        .translate(0.0, 0.0, DECK_Z / 2.0);

    let cassette_socket = centered_cube(
        name("containment_deck_cassette_dock_socket"),
        CASSETTE_DOCK_X + 28.0,
        CASSETTE_DOCK_Y + 24.0,
        SOCKET_Z,
    )
    .translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1,
        DECK_Z - SOCKET_Z / 2.0,
    );
    let module_socket = centered_cube(
        name("containment_deck_module_dock_socket"),
        MODULE_DOCK_X + 28.0,
        MODULE_DOCK_Y + 24.0,
        SOCKET_Z,
    )
    .translate(
        MODULE_DOCK_POS.0,
        MODULE_DOCK_POS.1,
        DECK_Z - SOCKET_Z / 2.0,
    );
    let manifold_socket = centered_cube(
        name("containment_deck_equalization_manifold_socket"),
        MANIFOLD_X + 30.0,
        MANIFOLD_Y + 22.0,
        SOCKET_Z,
    )
    .translate(MANIFOLD_POS.0, MANIFOLD_POS.1, DECK_Z - SOCKET_Z / 2.0);

    floor - cassette_socket - module_socket - manifold_socket - mounting_holes()
        + containment_curbs()
        + spill_gutters()
        + datum_targets()
        + deck_zone_label_lands()
}

fn containment_curbs() -> Part {
    let front = centered_cube(name("deck_front_curb"), STATION_X, CURB_W, CURB_Z).translate(
        0.0,
        -STATION_Y / 2.0 + CURB_W / 2.0,
        DECK_Z + CURB_Z / 2.0,
    );
    let rear = centered_cube(name("deck_rear_curb"), STATION_X, CURB_W, CURB_Z).translate(
        0.0,
        STATION_Y / 2.0 - CURB_W / 2.0,
        DECK_Z + CURB_Z / 2.0,
    );
    let left = centered_cube(name("deck_left_curb"), CURB_W, STATION_Y, CURB_Z).translate(
        -STATION_X / 2.0 + CURB_W / 2.0,
        0.0,
        DECK_Z + CURB_Z / 2.0,
    );
    let right = centered_cube(name("deck_right_curb"), CURB_W, STATION_Y, CURB_Z).translate(
        STATION_X / 2.0 - CURB_W / 2.0,
        0.0,
        DECK_Z + CURB_Z / 2.0,
    );
    front + rear + left + right
}

fn spill_gutters() -> Part {
    let mut gutters = Part::empty(name("deck_spill_witness_gutters"));
    for i in 0..SPILL_GUTTERS {
        let y = centered_index(i, SPILL_GUTTERS, 96.0) - 26.0;
        gutters = gutters
            + centered_cube(
                format!("{}_deck_spill_witness_gutter_{i}", PREFIX),
                STATION_X - 160.0,
                7.0,
                4.0,
            )
            .translate(0.0, y, DECK_Z + 2.0);
    }
    gutters
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(name("deck_mounting_holes"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{}_deck_mounting_hole_{i}", PREFIX),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 8.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn datum_targets() -> Part {
    let mut datums = Part::empty(name("deck_robot_datums"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 92.0), STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
        (-(STATION_X / 2.0 - 92.0), -(STATION_Y / 2.0 - 92.0)),
        (STATION_X / 2.0 - 92.0, -(STATION_Y / 2.0 - 92.0)),
    ]
    .iter()
    .enumerate()
    {
        datums = datums + datum_target(i).translate(*x, *y, DECK_Z + 2.8);
    }
    datums
}

fn datum_target(index: usize) -> Part {
    centered_cylinder(
        format!("{}_deck_datum_target_disc_{index}", PREFIX),
        15.0,
        3.0,
        36,
    ) - centered_cylinder(
        format!("{}_deck_datum_target_center_hole_{index}", PREFIX),
        5.0,
        5.0,
        24,
    ) + centered_cube(
        format!("{}_deck_datum_target_cross_x_{index}", PREFIX),
        34.0,
        3.2,
        4.0,
    ) + centered_cube(
        format!("{}_deck_datum_target_cross_y_{index}", PREFIX),
        3.2,
        34.0,
        4.0,
    )
}

fn deck_zone_label_lands() -> Part {
    label_land("deck_cassette_zone_label", 138.0, 22.0, 1).translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1 - CASSETTE_DOCK_Y / 2.0 - 30.0,
        DECK_Z + 4.0,
    ) + label_land("deck_module_zone_label", 138.0, 22.0, 2).translate(
        MODULE_DOCK_POS.0,
        MODULE_DOCK_POS.1 - MODULE_DOCK_Y / 2.0 - 30.0,
        DECK_Z + 4.0,
    ) + label_land("deck_equalize_zone_label", 170.0, 22.0, 3).translate(
        MANIFOLD_POS.0,
        MANIFOLD_POS.1 - MANIFOLD_Y / 2.0 - 28.0,
        DECK_Z + 4.0,
    )
}

fn cassette_dock() -> Part {
    let base = centered_cube(
        name("cassette_dock_base_block"),
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let mut pockets = Part::empty(name("cassette_dock_negative_slots"));
    let mut rails = Part::empty(name("cassette_dock_positive_rails"));
    for slot in 0..CASSETTE_SLOTS {
        let x = centered_index(slot, CASSETTE_SLOTS, CASSETTE_SLOT_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!("{}_cassette_dock_slot_clearance_{slot}", PREFIX),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                CASSETTE_SLOT_Z,
            )
            .translate(x, 4.0, CASSETTE_DOCK_Z / 2.0 - CASSETTE_SLOT_Z / 2.0 + 1.0);

        rails = rails
            + centered_cube(
                format!("{}_cassette_dock_left_key_rail_{slot}", PREFIX),
                8.0,
                CASSETTE_SLOT_Y + 30.0,
                12.0,
            )
            .translate(
                x - CASSETTE_SLOT_X / 2.0 - 12.0,
                4.0,
                CASSETTE_DOCK_Z / 2.0 + 6.0,
            )
            + centered_cube(
                format!("{}_cassette_dock_right_key_rail_{slot}", PREFIX),
                8.0,
                CASSETTE_SLOT_Y + 30.0,
                12.0,
            )
            .translate(
                x + CASSETTE_SLOT_X / 2.0 + 12.0,
                4.0,
                CASSETTE_DOCK_Z / 2.0 + 6.0,
            )
            + centered_cube(
                format!("{}_cassette_dock_soft_stop_bar_{slot}", PREFIX),
                CASSETTE_SLOT_X + 36.0,
                9.0,
                18.0,
            )
            .translate(x, CASSETTE_SLOT_Y / 2.0 + 28.0, CASSETTE_DOCK_Z / 2.0 + 9.0);
    }

    base - pockets + rails + cassette_stop_tokens() + dock_latch_state_tokens("cassette_dock")
}

fn cassette_stop_tokens() -> Part {
    let mut stops = Part::empty(name("cassette_dock_pressure_shock_stop_tokens"));
    for i in 0..CASSETTE_STOP_COUNT {
        let x = centered_index(i % 4, 4, 74.0);
        let y = if i < 4 { -116.0 } else { 116.0 };
        stops = stops
            + centered_cube(
                format!("{}_cassette_dock_stop_token_{i}", PREFIX),
                36.0,
                16.0,
                8.0,
            )
            .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 4.0);
    }
    stops
}

fn module_surrogate_dock() -> Part {
    let face = centered_cube(
        name("module_surrogate_dock_face_block"),
        MODULE_DOCK_X,
        MODULE_DOCK_Y,
        MODULE_DOCK_Z,
    );
    let mut port_bores = Part::empty(name("module_surrogate_dock_port_bores"));
    let mut port_bosses = Part::empty(name("module_surrogate_dock_port_bosses"));
    for port in 0..MODULE_PORTS {
        let x = centered_index(port, MODULE_PORTS, MODULE_PORT_PITCH_X);
        port_bores = port_bores
            + centered_cylinder(
                format!("{}_module_surrogate_port_bore_{port}", PREFIX),
                8.0,
                MODULE_DOCK_Z + 8.0,
                28,
            )
            .translate(x, 18.0, 0.0);
        port_bosses = port_bosses
            + centered_cylinder(
                format!("{}_module_surrogate_port_boss_{port}", PREFIX),
                MODULE_PORT_D / 2.0,
                14.0,
                36,
            )
            .translate(x, 18.0, MODULE_DOCK_Z / 2.0 + 7.0)
            + centered_cube(
                format!("{}_module_surrogate_port_id_tab_{port}", PREFIX),
                24.0,
                6.0,
                7.0,
            )
            .translate(x, -52.0, MODULE_DOCK_Z / 2.0 + 3.5);
    }

    face - port_bores
        + port_bosses
        + module_face_alignment_rails()
        + dock_latch_state_tokens("module_dock")
}

fn module_face_alignment_rails() -> Part {
    let mut rails = Part::empty(name("module_surrogate_face_alignment_rails"));
    for i in 0..MODULE_FACE_RAILS {
        let y = centered_index(i, MODULE_FACE_RAILS, 62.0);
        rails = rails
            + centered_cube(
                format!("{}_module_surrogate_face_alignment_rail_{i}", PREFIX),
                MODULE_DOCK_X - 62.0,
                8.0,
                12.0,
            )
            .translate(0.0, y, MODULE_DOCK_Z / 2.0 + 6.0);
    }
    rails
}

fn dock_latch_state_tokens(label: &str) -> Part {
    let states = ["release", "hold", "reject"];
    let mut tokens = Part::empty(name(&format!("{label}_latch_state_tokens")));
    for (i, state) in states.iter().enumerate() {
        tokens = tokens
            + centered_cube(
                name(&format!("{label}_{state}_latch_state_token_pocket")),
                48.0,
                18.0,
                7.0,
            )
            .translate(centered_index(i, states.len(), 70.0), -122.0, 32.0)
            + code_bars(&format!("{label}_{state}_state"), 38.0, 12.0, i + 1).translate(
                centered_index(i, states.len(), 70.0),
                -122.0,
                37.0,
            );
    }
    tokens
}

fn dual_equalization_manifold() -> Part {
    let body = centered_cube(
        name("dual_equalization_manifold_body"),
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut channels = Part::empty(name("dual_equalization_manifold_channels"));
    let mut ribs = Part::empty(name("dual_equalization_manifold_flow_ribs"));
    for lane in 0..EQUALIZATION_LANES {
        let y = centered_index(lane, EQUALIZATION_LANES, MANIFOLD_LANE_PITCH_Y);
        channels = channels
            + centered_cube(
                format!("{}_equalization_lane_{lane}_main_channel", PREFIX),
                MANIFOLD_X - 112.0,
                MANIFOLD_CHANNEL_W,
                MANIFOLD_CHANNEL_Z,
            )
            .translate(0.0, y, MANIFOLD_Z / 2.0 - 8.0);
        for branch in 0..BALANCE_BRANCHES_PER_LANE {
            let x = centered_index(branch, BALANCE_BRANCHES_PER_LANE, MANIFOLD_BRANCH_PITCH_X);
            channels = channels
                + centered_cube(
                    format!("{}_equalization_lane_{lane}_branch_{branch}", PREFIX),
                    MANIFOLD_CHANNEL_W,
                    48.0,
                    MANIFOLD_CHANNEL_Z,
                )
                .translate(x, y, MANIFOLD_Z / 2.0 - 8.0);
            ribs = ribs
                + centered_cube(
                    format!("{}_equalization_lane_{lane}_metering_rib_{branch}", PREFIX),
                    18.0,
                    6.0,
                    9.0,
                )
                .translate(x, y + 24.0, MANIFOLD_Z / 2.0 + 4.5);
        }
    }
    body - channels + ribs + ramp_rate_tokens() + anti_backflow_arrows()
}

fn ramp_rate_tokens() -> Part {
    let mut tokens = Part::empty(name("equalization_ramp_rate_tokens"));
    for i in 0..RAMP_TOKENS {
        tokens = tokens
            + centered_cube(
                format!("{}_equalization_ramp_token_{i}", PREFIX),
                34.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(i, RAMP_TOKENS, 54.0),
                -MANIFOLD_Y / 2.0 + 22.0,
                MANIFOLD_Z / 2.0 + 3.0,
            );
    }
    tokens
}

fn anti_backflow_arrows() -> Part {
    let mut arrows = Part::empty(name("equalization_anti_backflow_arrows"));
    for i in 0..6 {
        let x = centered_index(i, 6, 118.0);
        arrows = arrows
            + centered_cube(
                format!("{}_equalization_arrow_stem_{i}", PREFIX),
                34.0,
                5.0,
                6.0,
            )
            .translate(x, MANIFOLD_Y / 2.0 - 30.0, MANIFOLD_Z / 2.0 + 3.0)
            + centered_cube(
                format!("{}_equalization_arrow_head_{i}", PREFIX),
                12.0,
                18.0,
                6.0,
            )
            .translate(x + 22.0, MANIFOLD_Y / 2.0 - 30.0, MANIFOLD_Z / 2.0 + 3.0);
    }
    arrows
}

fn sterile_filtered_vent_holder() -> Part {
    let base = centered_cube(
        name("sterile_filtered_vent_holder_base"),
        VENT_HOLDER_X,
        VENT_HOLDER_Y,
        VENT_HOLDER_Z,
    );
    let mut cartridge_bores = Part::empty(name("sterile_filtered_vent_holder_cartridge_bores"));
    let mut collars = Part::empty(name("sterile_filtered_vent_holder_retainer_collars"));
    for i in 0..FILTER_CARTRIDGES {
        let x = centered_index(i, FILTER_CARTRIDGES, FILTER_PITCH_X);
        cartridge_bores = cartridge_bores
            + centered_cylinder(
                format!("{}_vent_filter_cartridge_bore_{i}", PREFIX),
                FILTER_D / 2.0 - 3.0,
                VENT_HOLDER_Z + 6.0,
                40,
            )
            .translate(x, 8.0, 0.0);
        collars = collars
            + centered_cylinder(
                format!("{}_vent_filter_retainer_collar_{i}", PREFIX),
                FILTER_D / 2.0 + 4.0,
                10.0,
                40,
            )
            .translate(x, 8.0, VENT_HOLDER_Z / 2.0 + 5.0);
    }
    base - cartridge_bores
        + collars
        + label_land("vent_holder_filtered_vent_label", 210.0, 20.0, 4).translate(
            0.0,
            -VENT_HOLDER_Y / 2.0 + 26.0,
            VENT_HOLDER_Z / 2.0 + 5.0,
        )
}

fn pressure_transducer_bosses() -> Part {
    let base = centered_cube(
        name("pressure_transducer_boss_plate"),
        PRESSURE_BOSS_X,
        PRESSURE_BOSS_Y,
        PRESSURE_BOSS_Z,
    );
    let mut bores = Part::empty(name("pressure_transducer_boss_bores"));
    let mut bosses = Part::empty(name("pressure_transducer_bosses"));
    for sensor in 0..TRANSDUCER_BOSSES {
        let col = sensor % TRANSDUCER_COLS;
        let row = sensor / TRANSDUCER_COLS;
        let x = centered_index(col, TRANSDUCER_COLS, TRANSDUCER_PITCH_X);
        let y = centered_index(row, 2, TRANSDUCER_PITCH_Y);
        bores = bores
            + centered_cylinder(
                format!("{}_pressure_transducer_bore_{sensor}", PREFIX),
                6.0,
                PRESSURE_BOSS_Z + 8.0,
                24,
            )
            .translate(x, y, 0.0);
        bosses = bosses
            + centered_cylinder(
                format!("{}_pressure_transducer_boss_{sensor}", PREFIX),
                TRANSDUCER_D / 2.0,
                14.0,
                36,
            )
            .translate(x, y, PRESSURE_BOSS_Z / 2.0 + 7.0)
            + centered_cube(
                format!("{}_pressure_transducer_cable_strain_land_{sensor}", PREFIX),
                38.0,
                7.0,
                6.0,
            )
            .translate(x, y - 25.0, PRESSURE_BOSS_Z / 2.0 + 3.0);
    }
    base - bores + bosses
}

fn backflow_dye_witness_channel() -> Part {
    let body = centered_cube(
        name("backflow_dye_witness_channel_body"),
        DYE_WITNESS_X,
        DYE_WITNESS_Y,
        DYE_WITNESS_Z,
    );
    let mut channels = Part::empty(name("backflow_dye_witness_channel_negative_channels"));
    for i in 0..DYE_CHANNELS {
        channels = channels
            + centered_cube(
                format!("{}_backflow_dye_witness_channel_{i}", PREFIX),
                DYE_WITNESS_X - 66.0,
                7.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, DYE_CHANNELS, 22.0),
                DYE_WITNESS_Z / 2.0 - 5.0,
            );
    }
    body - channels + dye_witness_wells() + dye_direction_baffles()
}

fn dye_witness_wells() -> Part {
    let mut wells = Part::empty(name("backflow_dye_witness_wells"));
    for i in 0..DYE_WELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("{}_backflow_dye_witness_well_{i}", PREFIX),
                13.0,
                8.0,
                32,
            )
            .translate(
                centered_index(i % 4, 4, 70.0),
                centered_index(i / 4, 2, 76.0),
                DYE_WITNESS_Z / 2.0 + 4.0,
            );
    }
    wells
}

fn dye_direction_baffles() -> Part {
    let mut baffles = Part::empty(name("backflow_dye_direction_baffles"));
    for i in 0..5 {
        baffles = baffles
            + centered_cube(
                format!("{}_backflow_dye_direction_baffle_{i}", PREFIX),
                9.0,
                DYE_WITNESS_Y - 48.0,
                8.0,
            )
            .translate(centered_index(i, 5, 58.0), 0.0, DYE_WITNESS_Z / 2.0 + 4.0);
    }
    baffles
}

fn relief_bypass_route() -> Part {
    let body = centered_cube(
        name("relief_bypass_route_body"),
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    let mut bypass_channels = Part::empty(name("relief_bypass_route_negative_channels"));
    let mut route_marks = Part::empty(name("relief_bypass_route_marks"));
    for branch in 0..BYPASS_BRANCHES {
        let y = centered_index(branch, BYPASS_BRANCHES, 48.0);
        bypass_channels = bypass_channels
            + centered_cube(
                format!("{}_relief_bypass_route_channel_{branch}", PREFIX),
                BYPASS_X - 72.0,
                10.0,
                12.0,
            )
            .translate(0.0, y, BYPASS_Z / 2.0 - 6.0);
        route_marks = route_marks
            + centered_cylinder(
                format!("{}_relief_bypass_route_check_orifice_{branch}", PREFIX),
                10.0,
                7.0,
                28,
            )
            .translate(
                -BYPASS_X / 2.0 + 54.0 + branch as f64 * 26.0,
                y,
                BYPASS_Z / 2.0 + 3.5,
            );
    }
    body - bypass_channels + route_marks + relief_tokens()
}

fn relief_tokens() -> Part {
    let mut tokens = Part::empty(name("relief_bypass_route_tokens"));
    for i in 0..RELIEF_TOKEN_COUNT {
        tokens = tokens
            + centered_cube(
                format!("{}_relief_bypass_token_{i}", PREFIX),
                32.0,
                18.0,
                7.0,
            )
            .translate(
                centered_index(i, RELIEF_TOKEN_COUNT, 42.0),
                -BYPASS_Y / 2.0 + 24.0,
                BYPASS_Z / 2.0 + 3.5,
            );
    }
    tokens
}

fn gasket_compression_witness_lands() -> Part {
    let base = centered_cube(
        name("gasket_compression_witness_land_bar"),
        GASKET_X,
        GASKET_Y,
        GASKET_Z,
    );
    let mut lands = Part::empty(name("gasket_compression_witness_lands"));
    for land in 0..GASKET_LANDS {
        let x = centered_index(land, GASKET_LANDS, 54.0);
        lands = lands
            + centered_cube(
                format!("{}_gasket_compression_witness_land_{land}", PREFIX),
                36.0,
                40.0,
                6.0,
            )
            .translate(x, 0.0, GASKET_Z / 2.0 + 3.0);
        for tick in 0..COMPRESSION_TICKS_PER_LAND {
            lands = lands
                + centered_cube(
                    format!("{}_gasket_compression_land_{land}_tick_{tick}", PREFIX),
                    5.0,
                    8.0 + tick as f64 * 6.0,
                    4.0,
                )
                .translate(
                    x - 11.0 + tick as f64 * 11.0,
                    35.0,
                    GASKET_Z / 2.0 + 2.0,
                );
        }
    }
    base + lands
}

fn custody_and_gate_lands() -> Part {
    barcode_rfid_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, 0.0)
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, 0.0)
}

fn barcode_rfid_custody_lands() -> Part {
    let base = centered_cube(
        name("barcode_rfid_custody_land_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty(name("barcode_rfid_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{}_barcode_custody_land_{i}", PREFIX),
                78.0,
                26.0,
                5.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 92.0),
                20.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(format!("{}_rfid_custody_land_{i}", PREFIX), 86.0, 34.0, 5.0)
                .translate(
                    centered_index(i, RFID_LANDS, 154.0),
                    -30.0,
                    CUSTODY_Z / 2.0 + 2.5,
                );
    }
    base + lands
        + code_bars("custody_code_bar_matrix", CUSTODY_X - 70.0, 12.0, 13).translate(
            0.0,
            -CUSTODY_Y / 2.0 + 13.0,
            CUSTODY_Z / 2.0 + 4.0,
        )
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        name("release_hold_reject_gate_panel"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let names = ["release", "hold", "reject"];
    let mut gates = Part::empty(name("release_hold_reject_gate_lanes"));
    for lane in 0..GATE_LANES {
        let x = centered_index(lane, GATE_LANES, 128.0);
        gates = gates
            + centered_cube(
                format!("{}_{}_gate_lane_socket", PREFIX, names[lane]),
                96.0,
                52.0,
                12.0,
            )
            .translate(x, 0.0, GATE_Z / 2.0 + 6.0);
        for token in 0..GATE_TOKENS_PER_LANE {
            gates = gates
                + centered_cube(
                    format!("{}_{}_gate_token_{}", PREFIX, names[lane], token),
                    16.0,
                    16.0,
                    8.0,
                )
                .translate(
                    x + centered_index(token, GATE_TOKENS_PER_LANE, 21.0),
                    -31.0,
                    GATE_Z / 2.0 + 4.0,
                );
        }
    }
    base + gates
}

fn camera_bridge_keepouts() -> Part {
    let left_post = centered_cube(
        name("evidence_camera_bridge_left_post"),
        26.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        name("evidence_camera_bridge_right_post"),
        26.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        name("evidence_camera_bridge_cross_beam"),
        BRIDGE_X + 26.0,
        28.0,
        24.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z + BRIDGE_Z + 12.0);

    left_post + right_post + beam + camera_sight_windows() + keepout_frames()
}

fn camera_sight_windows() -> Part {
    let mut windows = Part::empty(name("evidence_camera_sight_windows"));
    for i in 0..CAMERA_WINDOWS {
        windows = windows
            + centered_cube(
                format!("{}_evidence_camera_sight_window_{i}", PREFIX),
                116.0,
                9.0,
                6.0,
            )
            .translate(
                centered_index(i, CAMERA_WINDOWS, 270.0),
                BRIDGE_POS.1 - BRIDGE_Y / 2.0 - 16.0,
                DECK_Z + BRIDGE_Z + 28.0,
            );
    }
    windows
}

fn keepout_frames() -> Part {
    keepout_frame(
        "front_robot_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        0.0,
        -STATION_Y / 2.0 + ROBOT_KEEP_OUT_Y / 2.0 + 32.0,
    ) + keepout_frame(
        "right_service_access_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        STATION_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 34.0,
        0.0,
    )
}

fn keepout_frame(label: &str, x: f64, y: f64, cx: f64, cy: f64) -> Part {
    let front = centered_cube(
        name(&format!("{label}_front_rail")),
        x,
        6.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(cx, cy - y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let rear = centered_cube(name(&format!("{label}_rear_rail")), x, 6.0, KEEP_OUT_RAIL_Z)
        .translate(cx, cy + y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let left = centered_cube(name(&format!("{label}_left_rail")), 6.0, y, KEEP_OUT_RAIL_Z)
        .translate(cx - x / 2.0, cy, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let right = centered_cube(
        name(&format!("{label}_right_rail")),
        6.0,
        y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(cx + x / 2.0, cy, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    front + rear + left + right
}

fn label_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    centered_cube(name(&format!("{label}_land")), x, y, 3.0)
        + code_bars(label, x - 14.0, y - 8.0, code).translate(0.0, 0.0, 3.0)
}

fn code_bars(label: &str, x: f64, y: f64, code: usize) -> Part {
    let mut bars = Part::empty(name(&format!("{label}_raised_code_bars")));
    for bit in 0..8 {
        if ((code + 1) & (1 << bit)) != 0 {
            bars = bars
                + centered_cube(
                    name(&format!("{label}_raised_code_bar_{bit}")),
                    x / 10.0,
                    y,
                    3.0,
                )
                .translate(-x / 2.0 + (bit as f64 + 1.0) * x / 9.0, 0.0, 0.0);
        }
    }
    bars
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(suffix: &str) -> String {
    format!("{PREFIX}_{suffix}")
}

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn module_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "cassette_dock",
            center: CASSETTE_DOCK_POS,
            x: CASSETTE_DOCK_X,
            y: CASSETTE_DOCK_Y,
        },
        Footprint {
            name: "module_surrogate_dock",
            center: MODULE_DOCK_POS,
            x: MODULE_DOCK_X,
            y: MODULE_DOCK_Y,
        },
        Footprint {
            name: "dual_equalization_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Footprint {
            name: "sterile_filtered_vent_holder",
            center: VENT_HOLDER_POS,
            x: VENT_HOLDER_X,
            y: VENT_HOLDER_Y,
        },
        Footprint {
            name: "pressure_transducer_bosses",
            center: PRESSURE_BOSS_POS,
            x: PRESSURE_BOSS_X,
            y: PRESSURE_BOSS_Y,
        },
        Footprint {
            name: "backflow_dye_witness_channel",
            center: DYE_WITNESS_POS,
            x: DYE_WITNESS_X,
            y: DYE_WITNESS_Y,
        },
        Footprint {
            name: "relief_bypass_route",
            center: BYPASS_POS,
            x: BYPASS_X,
            y: BYPASS_Y,
        },
        Footprint {
            name: "gasket_compression_witness_lands",
            center: GASKET_POS,
            x: GASKET_X,
            y: GASKET_Y,
        },
        Footprint {
            name: "gate_lands",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn overlaps(left: Footprint, right: Footprint) -> bool {
    let dx = (left.center.0 - right.center.0).abs();
    let dy = (left.center.1 - right.center.1).abs();
    dx < (left.x + right.x) / 2.0 && dy < (left.y + right.y) / 2.0
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(EQUALIZATION_LANES, 2);
    assert_eq!(CASSETTE_SLOTS, 2);
    assert_eq!(GATE_LANES, 3);
    assert_eq!(DATUM_COUNT, 4);
    assert!(TRANSDUCER_BOSSES >= MODULE_PORTS);
    assert!(GASKET_LANDS >= MODULE_PORTS * 2);
    assert!(BRIDGE_Z > CURB_Z);

    let usable_x = STATION_X / 2.0 - CURB_W - 12.0;
    let usable_y = STATION_Y / 2.0 - CURB_W - 12.0;
    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            footprint.center.0.abs() + footprint.x / 2.0 <= usable_x,
            "{} exceeds usable deck x",
            footprint.name
        );
        assert!(
            footprint.center.1.abs() + footprint.y / 2.0 <= usable_y,
            "{} exceeds usable deck y",
            footprint.name
        );
    }
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            assert!(
                !overlaps(footprints[left], footprints[right]),
                "{} overlaps {}",
                footprints[left].name,
                footprints[right].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_paths_are_unique_and_prefixed() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_cassette_module_barrier_pressure_equalization_station_")));
        assert_eq!(
            OUTPUTS.last().copied(),
            Some(
                "output/closed_cassette_module_barrier_pressure_equalization_station_assembly.stl"
            )
        );
    }

    #[test]
    fn required_feature_groups_have_export_paths() {
        let joined = OUTPUTS.join("\n");
        for feature in REQUIRED_FEATURES {
            assert!(
                joined.contains(feature),
                "missing required feature output for {feature}"
            );
        }
    }

    #[test]
    fn geometry_counts_cover_pressure_equalization_risks() {
        assert_eq!(EQUALIZATION_LANES * BALANCE_BRANCHES_PER_LANE, 12);
        assert_eq!(FILTER_CARTRIDGES, 4);
        assert_eq!(TRANSDUCER_BOSSES, 8);
        assert_eq!(DYE_CHANNELS, 6);
        assert_eq!(DYE_WELL_COUNT, 8);
        assert_eq!(GASKET_LANDS * COMPRESSION_TICKS_PER_LAND, 36);
        assert_eq!(GATE_LANES * GATE_TOKENS_PER_LANE, 12);
        assert!(RAMP_TOKENS > RELIEF_TOKEN_COUNT);
    }

    #[test]
    fn major_modules_fit_without_overlapping() {
        assert_layout();
    }

    #[test]
    fn centered_index_is_symmetric() {
        assert_eq!(centered_index(0, 3, 10.0), -10.0);
        assert_eq!(centered_index(1, 3, 10.0), 0.0);
        assert_eq!(centered_index(2, 3, 10.0), 10.0);
        assert_eq!(centered_index(0, 2, 20.0), -10.0);
        assert_eq!(centered_index(1, 2, 20.0), 10.0);
    }
}
