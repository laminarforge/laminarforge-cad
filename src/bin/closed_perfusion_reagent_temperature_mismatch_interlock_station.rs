use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system reagent/media temperature mismatch interlock validation station.
//
// This generator packages a mechanical validation fixture used before perfusion
// or media exchange reaches tissue chips. It models containment, bag routing,
// sensor agreement interfaces, thermal equilibration surrogates, custody lands,
// wetness/condensation witnesses, and disposition gates. It is packaging and
// validation hardware only, not a biological protocol or calibrated temperature
// standard.

const OUTPUTS: [&str; 13] = [
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_containment_deck.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_feed_reagent_bag_nests.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_temperature_equilibration_block_surrogate.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_upstream_downstream_probe_pockets.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_reference_logger_docks.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_route_keying_comb.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_bypass_interlock_gate.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_wetness_condensation_witness_wells.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_barcode_coa_custody_lands.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_release_hold_reject_gates.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_evidence_camera_bridge.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_robot_service_keepouts.stl",
    "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 20] = [
    "containment_deck",
    "feed_bag_nest",
    "reagent_bag_nest",
    "stratified_bag_temperature_windows",
    "temperature_equilibration_block_surrogate",
    "upstream_probe_pockets",
    "downstream_probe_pockets",
    "reference_logger_docks",
    "route_keying_comb",
    "wrong_bag_route_keying",
    "bypass_interlock_gate",
    "interlock_bypass_witness",
    "wetness_witness_wells",
    "condensation_witness_wells",
    "barcode_land",
    "coa_land",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "evidence_camera_bridge",
];

const DECK_X: f64 = 1240.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1080.0;
const SUMP_Y: f64 = 640.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const BAG_NEST_X: f64 = 312.0;
const BAG_NEST_Y: f64 = 250.0;
const BAG_NEST_Z: f64 = 42.0;
const BAG_NEST_POS_Y: f64 = 222.0;
const FEED_BAG_POS_X: f64 = -330.0;
const REAGENT_BAG_POS_X: f64 = 30.0;
const BAG_SADDLES_PER_NEST: usize = 3;
const BAG_STRATIFICATION_WINDOWS: usize = 6;
const BAG_CLAMP_RIBS: usize = 4;
const BAG_ROUTE_PORTS_PER_NEST: usize = 4;

const EQUIL_BLOCK_X: f64 = 350.0;
const EQUIL_BLOCK_Y: f64 = 176.0;
const EQUIL_BLOCK_Z: f64 = 64.0;
const EQUIL_BLOCK_POS: (f64, f64) = (335.0, 205.0);
const EQUIL_CHANNELS: usize = 8;
const EQUIL_CHANNEL_D: f64 = 9.0;
const THERMAL_MASS_SLOTS: usize = 5;
const EQUALIZATION_WINDOW_COUNT: usize = 4;

const PROBE_BAR_X: f64 = 760.0;
const PROBE_BAR_Y: f64 = 122.0;
const PROBE_BAR_Z: f64 = 44.0;
const PROBE_BAR_POS: (f64, f64) = (0.0, 32.0);
const PROBE_PAIRS: usize = 4;
const PROBE_POCKET_D: f64 = 17.0;
const PROBE_BOSS_D: f64 = 34.0;
const PROBE_SPACING_X: f64 = 78.0;
const SENSOR_DISAGREEMENT_FLAGS: usize = 5;

const LOGGER_DOCK_X: f64 = 430.0;
const LOGGER_DOCK_Y: f64 = 118.0;
const LOGGER_DOCK_Z: f64 = 38.0;
const LOGGER_DOCK_POS: (f64, f64) = (-350.0, -125.0);
const LOGGER_DOCKS: usize = 4;
const LOGGER_POCKET_X: f64 = 86.0;
const LOGGER_POCKET_Y: f64 = 64.0;
const LOGGER_CABLE_COMBS: usize = 4;

const ROUTE_COMB_X: f64 = 560.0;
const ROUTE_COMB_Y: f64 = 96.0;
const ROUTE_COMB_Z: f64 = 56.0;
const ROUTE_COMB_POS: (f64, f64) = (255.0, -125.0);
const ROUTE_LANES: usize = 8;
const ROUTE_LANE_PITCH: f64 = 60.0;
const ROUTE_CHANNEL_D: f64 = 10.0;
const WRONG_BAG_KEY_TEETH: usize = 7;

const INTERLOCK_X: f64 = 520.0;
const INTERLOCK_Y: f64 = 142.0;
const INTERLOCK_Z: f64 = 52.0;
const INTERLOCK_POS: (f64, f64) = (10.0, -270.0);
const BYPASS_CHANNELS: usize = 2;
const INTERLOCK_TOKENS: usize = 6;
const TAMPER_FLAG_WELLS: usize = 4;

const WITNESS_X: f64 = 300.0;
const WITNESS_Y: f64 = 156.0;
const WITNESS_Z: f64 = 35.0;
const WITNESS_POS: (f64, f64) = (-458.0, -286.0);
const WETNESS_WELLS: usize = 6;
const CONDENSATION_WELLS: usize = 6;
const WITNESS_WELL_D: f64 = 27.0;

const CUSTODY_X: f64 = 405.0;
const CUSTODY_Y: f64 = 128.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (365.0, -292.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_PADS: usize = 4;

const GATE_X: f64 = 372.0;
const GATE_Y: f64 = 122.0;
const GATE_Z: f64 = 44.0;
const GATE_POS: (f64, f64) = (-45.0, -383.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 930.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 230.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -18.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;

const ROBOT_KEEPOUT_X: f64 = 1120.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 80.0;
const SERVICE_KEEPOUT_X: f64 = 104.0;
const SERVICE_KEEPOUT_Y: f64 = 670.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 310.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(feed_reagent_bag_nests(), OUTPUTS[1]);
    write_part(temperature_equilibration_block_surrogate(), OUTPUTS[2]);
    write_part(upstream_downstream_probe_pockets(), OUTPUTS[3]);
    write_part(reference_logger_docks(), OUTPUTS[4]);
    write_part(route_keying_comb(), OUTPUTS[5]);
    write_part(bypass_interlock_gate(), OUTPUTS[6]);
    write_part(wetness_condensation_witness_wells(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(evidence_camera_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed perfusion reagent temperature mismatch interlock station: {:.0}mm x {:.0}mm contained deck, {} route lanes, {} probe pairs, {} logger docks.",
        DECK_X, DECK_Y, ROUTE_LANES, PROBE_PAIRS, LOGGER_DOCKS
    );
    println!(
        "Mismatch defenses: {} stratification windows, {} equilibration channels, {} sensor disagreement flags, {} wrong-bag key teeth, {} bypass channels.",
        BAG_STRATIFICATION_WINDOWS,
        EQUIL_CHANNELS,
        SENSOR_DISAGREEMENT_FLAGS,
        WRONG_BAG_KEY_TEETH,
        BYPASS_CHANNELS
    );
    println!(
        "Evidence and disposition: {} wetness wells, {} condensation wells, {} barcode lands, {} COA lands, release/hold/reject gates, {} camera mounts.",
        WETNESS_WELLS,
        CONDENSATION_WELLS,
        BARCODE_LANDS,
        COA_LANDS,
        CAMERA_MOUNTS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_temp_mismatch_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_temp_mismatch_station_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_temp_mismatch_station_captured_drain",
        DRAIN_D / 2.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 74.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + module_landing_recesses()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_temp_mismatch_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_center_z());
    let right = centered_cube(
        "closed_temp_mismatch_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_temp_mismatch_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_temp_mismatch_station_front_low_containment_lip",
        DECK_X - 132.0,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_temp_mismatch_station_deck_datums");
    for (i, (x, y)) in [
        (-555.0, -350.0),
        (555.0, -350.0),
        (-555.0, 350.0),
        (555.0, 350.0),
        (-285.0, -350.0),
        (285.0, -350.0),
        (-285.0, 350.0),
        (285.0, 350.0),
        (0.0, -350.0),
        (0.0, 350.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("closed_temp_mismatch_station_datum_boss_{i}"),
            13.0,
            5.0,
            40,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("closed_temp_mismatch_station_datum_bore_{i}"),
            3.2,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }
    datums
}

fn module_landing_recesses() -> Part {
    let bag_a = centered_cube(
        "closed_temp_mismatch_station_feed_bag_landing_recess",
        BAG_NEST_X + 28.0,
        BAG_NEST_Y + 28.0,
        5.0,
    )
    .translate(FEED_BAG_POS_X, BAG_NEST_POS_Y, DECK_Z / 2.0 - 1.8);
    let bag_b = centered_cube(
        "closed_temp_mismatch_station_reagent_bag_landing_recess",
        BAG_NEST_X + 28.0,
        BAG_NEST_Y + 28.0,
        5.0,
    )
    .translate(REAGENT_BAG_POS_X, BAG_NEST_POS_Y, DECK_Z / 2.0 - 1.8);
    let block = centered_cube(
        "closed_temp_mismatch_station_equilibration_block_landing_recess",
        EQUIL_BLOCK_X + 30.0,
        EQUIL_BLOCK_Y + 26.0,
        5.0,
    )
    .translate(EQUIL_BLOCK_POS.0, EQUIL_BLOCK_POS.1, DECK_Z / 2.0 - 1.8);
    let probe_bar = centered_cube(
        "closed_temp_mismatch_station_probe_bar_landing_recess",
        PROBE_BAR_X + 30.0,
        PROBE_BAR_Y + 24.0,
        5.0,
    )
    .translate(PROBE_BAR_POS.0, PROBE_BAR_POS.1, DECK_Z / 2.0 - 1.8);

    Part::empty("closed_temp_mismatch_station_module_landing_recesses")
        - bag_a
        - bag_b
        - block
        - probe_bar
}

fn feed_reagent_bag_nests() -> Part {
    bag_nest("feed", FEED_BAG_POS_X) + bag_nest("reagent", REAGENT_BAG_POS_X)
}

fn bag_nest(kind: &str, x: f64) -> Part {
    let base = centered_cube(
        format!("closed_temp_mismatch_station_{kind}_bag_nest_base"),
        BAG_NEST_X,
        BAG_NEST_Y,
        BAG_NEST_Z,
    );
    let saddle_cuts = bag_saddle_cuts(kind);
    let stratification = bag_stratification_windows(kind);
    let clamps = bag_clamp_ribs(kind);
    let ports = bag_route_port_collars(kind);
    let wrong_bag_key = centered_cube(
        format!("closed_temp_mismatch_station_{kind}_wrong_bag_asymmetric_key_land"),
        46.0,
        24.0,
        16.0,
    )
    .translate(if kind == "feed" { -112.0 } else { 112.0 }, -98.0, 28.0);

    (base - saddle_cuts - stratification + clamps + ports + wrong_bag_key).translate(
        x,
        BAG_NEST_POS_Y,
        DECK_Z / 2.0 + BAG_NEST_Z / 2.0,
    )
}

fn bag_saddle_cuts(kind: &str) -> Part {
    let mut saddles = Part::empty(format!(
        "closed_temp_mismatch_station_{kind}_bag_saddle_cuts"
    ));
    for i in 0..BAG_SADDLES_PER_NEST {
        let x = -86.0 + i as f64 * 86.0;
        let cut = centered_cylinder(
            format!("closed_temp_mismatch_station_{kind}_bag_saddle_cut_{i}"),
            39.0,
            BAG_NEST_Y + 8.0,
            42,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 24.0);
        saddles = saddles + cut;
    }
    saddles
}

fn bag_stratification_windows(kind: &str) -> Part {
    let mut windows = Part::empty(format!(
        "closed_temp_mismatch_station_{kind}_stratified_bag_temperature_windows"
    ));
    for i in 0..BAG_STRATIFICATION_WINDOWS {
        let x = -120.0 + (i % 3) as f64 * 120.0;
        let y = if i < 3 { -92.0 } else { 92.0 };
        let window = centered_cube(
            format!("closed_temp_mismatch_station_{kind}_stratification_window_{i}"),
            58.0,
            10.0,
            28.0,
        )
        .translate(x, y, 24.0);
        windows = windows + window;
    }
    windows
}

fn bag_clamp_ribs(kind: &str) -> Part {
    let mut ribs = Part::empty(format!(
        "closed_temp_mismatch_station_{kind}_bag_clamp_ribs"
    ));
    for i in 0..BAG_CLAMP_RIBS {
        let y = -88.0 + i as f64 * 58.0;
        let rib = centered_cube(
            format!("closed_temp_mismatch_station_{kind}_bag_clamp_rib_{i}"),
            BAG_NEST_X - 42.0,
            9.0,
            18.0,
        )
        .translate(0.0, y, 36.0);
        ribs = ribs + rib;
    }
    ribs
}

fn bag_route_port_collars(kind: &str) -> Part {
    let mut collars = Part::empty(format!(
        "closed_temp_mismatch_station_{kind}_bag_route_port_collars"
    ));
    for i in 0..BAG_ROUTE_PORTS_PER_NEST {
        let x = -96.0 + i as f64 * 64.0;
        let collar = centered_cylinder(
            format!("closed_temp_mismatch_station_{kind}_bag_route_port_collar_{i}"),
            15.0,
            9.0,
            32,
        )
        .translate(x, -BAG_NEST_Y / 2.0 + 18.0, 27.0);
        let bore = centered_cylinder(
            format!("closed_temp_mismatch_station_{kind}_bag_route_port_bore_{i}"),
            5.0,
            13.0,
            24,
        )
        .translate(x, -BAG_NEST_Y / 2.0 + 18.0, 27.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn temperature_equilibration_block_surrogate() -> Part {
    let block = centered_cube(
        "closed_temp_mismatch_station_temperature_equilibration_block_surrogate",
        EQUIL_BLOCK_X,
        EQUIL_BLOCK_Y,
        EQUIL_BLOCK_Z,
    );
    let thermal_slots = thermal_mass_slots();
    let flow_channels = equilibration_flow_channels();
    let windows = equilibration_read_windows();
    let datum = centered_cube(
        "closed_temp_mismatch_station_equilibration_block_orientation_key",
        44.0,
        26.0,
        20.0,
    )
    .translate(EQUIL_BLOCK_X / 2.0 - 44.0, EQUIL_BLOCK_Y / 2.0 - 26.0, 42.0);

    (block - thermal_slots - flow_channels - windows + datum).translate(
        EQUIL_BLOCK_POS.0,
        EQUIL_BLOCK_POS.1,
        DECK_Z / 2.0 + EQUIL_BLOCK_Z / 2.0,
    )
}

fn thermal_mass_slots() -> Part {
    let mut slots = Part::empty("closed_temp_mismatch_station_thermal_mass_slots");
    for i in 0..THERMAL_MASS_SLOTS {
        let x = -120.0 + i as f64 * 60.0;
        let slot = centered_cube(
            format!("closed_temp_mismatch_station_removable_thermal_mass_slot_{i}"),
            34.0,
            EQUIL_BLOCK_Y + 8.0,
            26.0,
        )
        .translate(x, 0.0, 8.0);
        slots = slots + slot;
    }
    slots
}

fn equilibration_flow_channels() -> Part {
    let mut channels = Part::empty("closed_temp_mismatch_station_equilibration_flow_channels");
    for i in 0..EQUIL_CHANNELS {
        let y = -61.0 + i as f64 * 17.5;
        let channel = centered_cylinder(
            format!("closed_temp_mismatch_station_equilibration_channel_{i}"),
            EQUIL_CHANNEL_D / 2.0,
            EQUIL_BLOCK_X + 12.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 8.0);
        channels = channels + channel;
    }
    channels
}

fn equilibration_read_windows() -> Part {
    let mut windows = Part::empty("closed_temp_mismatch_station_equilibration_read_windows");
    for i in 0..EQUALIZATION_WINDOW_COUNT {
        let x = -114.0 + i as f64 * 76.0;
        let window = centered_cube(
            format!("closed_temp_mismatch_station_equilibration_status_window_{i}"),
            42.0,
            12.0,
            32.0,
        )
        .translate(x, -EQUIL_BLOCK_Y / 2.0, 26.0);
        windows = windows + window;
    }
    windows
}

fn upstream_downstream_probe_pockets() -> Part {
    let bar = centered_cube(
        "closed_temp_mismatch_station_upstream_downstream_probe_bar",
        PROBE_BAR_X,
        PROBE_BAR_Y,
        PROBE_BAR_Z,
    );
    let bores = probe_pocket_bores();
    let bosses = probe_pocket_bosses();
    let flags = sensor_disagreement_flag_slots();

    (bar - bores - flags + bosses).translate(
        PROBE_BAR_POS.0,
        PROBE_BAR_POS.1,
        DECK_Z / 2.0 + PROBE_BAR_Z / 2.0,
    )
}

fn probe_pocket_bores() -> Part {
    let mut bores = Part::empty("closed_temp_mismatch_station_probe_pocket_bores");
    for i in 0..PROBE_PAIRS {
        let x = -probe_span_x() / 2.0 + i as f64 * PROBE_SPACING_X * 2.0;
        for (j, y) in [-34.0, 34.0].iter().enumerate() {
            let bore = centered_cylinder(
                format!("closed_temp_mismatch_station_probe_pocket_bore_{i}_{j}"),
                PROBE_POCKET_D / 2.0,
                PROBE_BAR_Z + 8.0,
                32,
            )
            .translate(x, *y, 0.0);
            bores = bores + bore;
        }
    }
    bores
}

fn probe_pocket_bosses() -> Part {
    let mut bosses = Part::empty("closed_temp_mismatch_station_probe_pocket_bosses");
    for i in 0..PROBE_PAIRS {
        let x = -probe_span_x() / 2.0 + i as f64 * PROBE_SPACING_X * 2.0;
        let upstream = probe_boss("upstream", i, x, -34.0);
        let downstream = probe_boss("downstream", i, x, 34.0);
        bosses = bosses + upstream + downstream;
    }
    bosses
}

fn probe_boss(kind: &str, index: usize, x: f64, y: f64) -> Part {
    let boss = centered_cylinder(
        format!("closed_temp_mismatch_station_{kind}_probe_boss_{index}"),
        PROBE_BOSS_D / 2.0,
        8.0,
        40,
    )
    .translate(x, y, PROBE_BAR_Z / 2.0 + 4.0);
    let label_land = centered_cube(
        format!("closed_temp_mismatch_station_{kind}_probe_label_land_{index}"),
        42.0,
        16.0,
        4.0,
    )
    .translate(x, y + if kind == "upstream" { -24.0 } else { 24.0 }, 26.0);
    boss + label_land
}

fn sensor_disagreement_flag_slots() -> Part {
    let mut flags = Part::empty("closed_temp_mismatch_station_sensor_disagreement_flag_slots");
    for i in 0..SENSOR_DISAGREEMENT_FLAGS {
        let x = -160.0 + i as f64 * 80.0;
        let flag = centered_cube(
            format!("closed_temp_mismatch_station_sensor_disagreement_flag_slot_{i}"),
            34.0,
            20.0,
            10.0,
        )
        .translate(x, 0.0, PROBE_BAR_Z / 2.0);
        flags = flags + flag;
    }
    flags
}

fn reference_logger_docks() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_reference_logger_dock_base",
        LOGGER_DOCK_X,
        LOGGER_DOCK_Y,
        LOGGER_DOCK_Z,
    );
    let pockets = logger_pockets();
    let combs = logger_cable_combs();

    (base - pockets + combs).translate(
        LOGGER_DOCK_POS.0,
        LOGGER_DOCK_POS.1,
        DECK_Z / 2.0 + LOGGER_DOCK_Z / 2.0,
    )
}

fn logger_pockets() -> Part {
    let mut pockets = Part::empty("closed_temp_mismatch_station_reference_logger_pockets");
    for i in 0..LOGGER_DOCKS {
        let x = -157.5 + i as f64 * 105.0;
        let pocket = centered_cube(
            format!("closed_temp_mismatch_station_reference_logger_pocket_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            14.0,
        )
        .translate(x, 0.0, LOGGER_DOCK_Z / 2.0 - 4.0);
        pockets = pockets + pocket;
    }
    pockets
}

fn logger_cable_combs() -> Part {
    let mut combs = Part::empty("closed_temp_mismatch_station_reference_logger_cable_combs");
    for i in 0..LOGGER_CABLE_COMBS {
        let x = -157.5 + i as f64 * 105.0;
        let comb = centered_cube(
            format!("closed_temp_mismatch_station_logger_cable_comb_{i}"),
            12.0,
            42.0,
            22.0,
        )
        .translate(x, LOGGER_DOCK_Y / 2.0 - 18.0, LOGGER_DOCK_Z / 2.0 + 7.0);
        combs = combs + comb;
    }
    combs
}

fn route_keying_comb() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_route_keying_comb_base",
        ROUTE_COMB_X,
        ROUTE_COMB_Y,
        ROUTE_COMB_Z,
    );
    let channels = route_channel_bores();
    let keys = wrong_bag_key_teeth();
    let lane_labels = route_lane_label_lands();

    (base - channels + keys + lane_labels).translate(
        ROUTE_COMB_POS.0,
        ROUTE_COMB_POS.1,
        DECK_Z / 2.0 + ROUTE_COMB_Z / 2.0,
    )
}

fn route_channel_bores() -> Part {
    let mut channels = Part::empty("closed_temp_mismatch_station_route_channel_bores");
    for i in 0..ROUTE_LANES {
        let x = -route_lane_span() / 2.0 + i as f64 * ROUTE_LANE_PITCH;
        let channel = centered_cylinder(
            format!("closed_temp_mismatch_station_keyed_route_channel_{i}"),
            ROUTE_CHANNEL_D / 2.0,
            ROUTE_COMB_Y + 10.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 4.0);
        channels = channels + channel;
    }
    channels
}

fn wrong_bag_key_teeth() -> Part {
    let mut teeth = Part::empty("closed_temp_mismatch_station_wrong_bag_route_key_teeth");
    for i in 0..WRONG_BAG_KEY_TEETH {
        let x = -route_lane_span() / 2.0 + 30.0 + i as f64 * ROUTE_LANE_PITCH;
        let tooth_h = if i % 3 == 0 { 26.0 } else { 18.0 };
        let tooth = centered_cube(
            format!("closed_temp_mismatch_station_wrong_bag_key_tooth_{i}"),
            18.0,
            20.0,
            tooth_h,
        )
        .translate(
            x,
            -ROUTE_COMB_Y / 2.0 + 20.0,
            ROUTE_COMB_Z / 2.0 + tooth_h / 2.0,
        );
        teeth = teeth + tooth;
    }
    teeth
}

fn route_lane_label_lands() -> Part {
    let mut lands = Part::empty("closed_temp_mismatch_station_route_lane_label_lands");
    for i in 0..ROUTE_LANES {
        let x = -route_lane_span() / 2.0 + i as f64 * ROUTE_LANE_PITCH;
        let land = centered_cube(
            format!("closed_temp_mismatch_station_route_lane_label_land_{i}"),
            38.0,
            14.0,
            3.0,
        )
        .translate(x, ROUTE_COMB_Y / 2.0 - 14.0, ROUTE_COMB_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn bypass_interlock_gate() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_bypass_interlock_gate_base",
        INTERLOCK_X,
        INTERLOCK_Y,
        INTERLOCK_Z,
    );
    let bypasses = bypass_channel_bores();
    let token_slots = interlock_token_slots();
    let tamper_wells = interlock_tamper_flag_wells();
    let gate_blade = centered_cube(
        "closed_temp_mismatch_station_captive_interlock_gate_blade",
        82.0,
        INTERLOCK_Y + 26.0,
        18.0,
    )
    .translate(-INTERLOCK_X / 2.0 + 118.0, 0.0, INTERLOCK_Z / 2.0 + 9.0);

    (base - bypasses - token_slots - tamper_wells + gate_blade).translate(
        INTERLOCK_POS.0,
        INTERLOCK_POS.1,
        DECK_Z / 2.0 + INTERLOCK_Z / 2.0,
    )
}

fn bypass_channel_bores() -> Part {
    let mut bores = Part::empty("closed_temp_mismatch_station_bypass_channel_bores");
    for i in 0..BYPASS_CHANNELS {
        let y = -32.0 + i as f64 * 64.0;
        let bore = centered_cylinder(
            format!("closed_temp_mismatch_station_bypass_channel_bore_{i}"),
            6.0,
            INTERLOCK_X + 8.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 4.0);
        bores = bores + bore;
    }
    bores
}

fn interlock_token_slots() -> Part {
    let mut slots = Part::empty("closed_temp_mismatch_station_interlock_token_slots");
    for i in 0..INTERLOCK_TOKENS {
        let x = -155.0 + i as f64 * 62.0;
        let slot = centered_cube(
            format!("closed_temp_mismatch_station_interlock_token_slot_{i}"),
            36.0,
            30.0,
            14.0,
        )
        .translate(x, 0.0, INTERLOCK_Z / 2.0 - 5.0);
        slots = slots + slot;
    }
    slots
}

fn interlock_tamper_flag_wells() -> Part {
    let mut wells = Part::empty("closed_temp_mismatch_station_interlock_bypass_witness_wells");
    for i in 0..TAMPER_FLAG_WELLS {
        let x = -186.0 + i as f64 * 124.0;
        let well = centered_cylinder(
            format!("closed_temp_mismatch_station_interlock_bypass_witness_well_{i}"),
            11.0,
            16.0,
            30,
        )
        .translate(x, -INTERLOCK_Y / 2.0 + 24.0, INTERLOCK_Z / 2.0);
        wells = wells + well;
    }
    wells
}

fn wetness_condensation_witness_wells() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_wetness_condensation_witness_base",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let wetness = witness_well_bank("wetness", -36.0, WETNESS_WELLS);
    let condensation = witness_well_bank("condensation", 36.0, CONDENSATION_WELLS);
    let labels = centered_cube(
        "closed_temp_mismatch_station_witness_label_land",
        WITNESS_X - 38.0,
        14.0,
        3.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 15.0, WITNESS_Z / 2.0 + 2.0);

    (base - wetness - condensation + labels).translate(
        WITNESS_POS.0,
        WITNESS_POS.1,
        DECK_Z / 2.0 + WITNESS_Z / 2.0,
    )
}

fn witness_well_bank(kind: &str, y: f64, count: usize) -> Part {
    let mut wells = Part::empty(format!(
        "closed_temp_mismatch_station_{kind}_witness_well_bank"
    ));
    for i in 0..count {
        let x = -112.5 + i as f64 * 45.0;
        let well = centered_cylinder(
            format!("closed_temp_mismatch_station_{kind}_witness_well_{i}"),
            WITNESS_WELL_D / 2.0,
            18.0,
            36,
        )
        .translate(x, y, WITNESS_Z / 2.0);
        wells = wells + well;
    }
    wells
}

fn barcode_coa_custody_lands() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_barcode_coa_custody_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let barcode = barcode_lands();
    let coa = coa_lands();
    let seals = tamper_seal_pads();

    (base + barcode + coa + seals).translate(
        CUSTODY_POS.0,
        CUSTODY_POS.1,
        DECK_Z / 2.0 + CUSTODY_Z / 2.0,
    )
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_temp_mismatch_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = -155.0 + (i % 3) as f64 * 105.0;
        let y = -30.0 + (i / 3) as f64 * 40.0;
        let land = centered_cube(
            format!("closed_temp_mismatch_station_barcode_land_{i}"),
            82.0,
            24.0,
            2.5,
        )
        .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("closed_temp_mismatch_station_coa_custody_lands");
    for i in 0..COA_LANDS {
        let x = -118.0 + i as f64 * 118.0;
        let land = centered_cube(
            format!("closed_temp_mismatch_station_coa_land_{i}"),
            94.0,
            26.0,
            2.5,
        )
        .translate(x, CUSTODY_Y / 2.0 - 22.0, CUSTODY_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn tamper_seal_pads() -> Part {
    let mut pads = Part::empty("closed_temp_mismatch_station_custody_tamper_seal_pads");
    for i in 0..TAMPER_SEAL_PADS {
        let x = if i % 2 == 0 { -180.0 } else { 180.0 };
        let y = if i < 2 { -50.0 } else { 50.0 };
        let pad = centered_cylinder(
            format!("closed_temp_mismatch_station_custody_tamper_seal_pad_{i}"),
            11.0,
            3.0,
            30,
        )
        .translate(x, y, CUSTODY_Z / 2.0 + 2.5);
        pads = pads + pad;
    }
    pads
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_temp_mismatch_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = Part::empty("closed_temp_mismatch_station_disposition_gates");
    for i in 0..DISPOSITION_GATES {
        let x = -112.0 + i as f64 * 112.0;
        gates = gates + disposition_gate(i, x);
    }
    let token_slots = gate_token_slots();

    (base - token_slots + gates).translate(GATE_POS.0, GATE_POS.1, DECK_Z / 2.0 + GATE_Z / 2.0)
}

fn disposition_gate(index: usize, x: f64) -> Part {
    let name = disposition_gate_name(index);
    let gate = centered_cube(
        format!("closed_temp_mismatch_station_{name}_gate_handle"),
        70.0,
        28.0,
        26.0,
    )
    .translate(x, -GATE_Y / 2.0 + 26.0, GATE_Z / 2.0 + 13.0);
    let flag = centered_cube(
        format!("closed_temp_mismatch_station_{name}_gate_status_flag"),
        54.0,
        10.0,
        42.0,
    )
    .translate(x, GATE_Y / 2.0 - 16.0, GATE_Z / 2.0 + 21.0);
    gate + flag
}

fn gate_token_slots() -> Part {
    let mut slots = Part::empty("closed_temp_mismatch_station_disposition_gate_token_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        let x = -147.0 + i as f64 * 58.8;
        let slot = centered_cube(
            format!("closed_temp_mismatch_station_disposition_token_slot_{i}"),
            34.0,
            24.0,
            14.0,
        )
        .translate(x, 0.0, GATE_Z / 2.0 - 5.0);
        slots = slots + slot;
    }
    slots
}

fn evidence_camera_bridge() -> Part {
    let rail = centered_cube(
        "closed_temp_mismatch_station_evidence_camera_bridge_crossrail",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        34.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z,
    );
    let left_post = camera_bridge_post(-CAMERA_BRIDGE_X / 2.0 + 34.0);
    let right_post = camera_bridge_post(CAMERA_BRIDGE_X / 2.0 - 34.0);
    let mounts = camera_mounts();
    let fiducials = evidence_fiducials();

    rail + left_post + right_post + mounts + fiducials
}

fn camera_bridge_post(x: f64) -> Part {
    centered_cube(
        format!("closed_temp_mismatch_station_camera_bridge_post_{x:.0}"),
        34.0,
        38.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 + x,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    )
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_temp_mismatch_station_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        let x = -360.0 + i as f64 * 180.0;
        let mount = centered_cylinder(
            format!("closed_temp_mismatch_station_evidence_camera_mount_{i}"),
            18.0,
            10.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            CAMERA_BRIDGE_POS.0 + x,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 26.0,
        );
        mounts = mounts + mount;
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_temp_mismatch_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -450.0 + i as f64 * 100.0;
        let fid = centered_cylinder(
            format!("closed_temp_mismatch_station_evidence_fiducial_{i}"),
            7.0,
            2.5,
            28,
        )
        .translate(x, -DECK_Y / 2.0 + 54.0, DECK_Z / 2.0 + 2.0);
        fiducials = fiducials + fid;
    }
    fiducials
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_temp_mismatch_station_front_robot_sweep_keepout",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_temp_mismatch_station_left_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "closed_temp_mismatch_station_right_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "closed_temp_mismatch_station_top_service_clearance_keepout",
        DECK_X - 140.0,
        DECK_Y - 150.0,
        28.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + left_service + right_service + top_clearance
}

fn station_assembly() -> Part {
    containment_deck()
        + feed_reagent_bag_nests()
        + temperature_equilibration_block_surrogate()
        + upstream_downstream_probe_pockets()
        + reference_logger_docks()
        + route_keying_comb()
        + bypass_interlock_gate()
        + wetness_condensation_witness_wells()
        + barcode_coa_custody_lands()
        + release_hold_reject_gates()
        + evidence_camera_bridge()
        + robot_service_keepouts()
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13, "unexpected STL output count");
    assert_eq!(DATUM_BOSSES, 10, "datum boss table changed");
    assert!(
        bag_nest_span_x() < DECK_X - 2.0 * RIM_W - 140.0,
        "bag nests exceed contained deck width"
    );
    assert!(
        route_lane_span() < ROUTE_COMB_X - 70.0,
        "route keying comb lane span exceeds comb width"
    );
    assert!(
        probe_span_x() < PROBE_BAR_X - 160.0,
        "probe pockets exceed available bar width"
    );
    assert!(
        witness_well_bank_span() < WITNESS_X - 44.0,
        "witness well bank exceeds witness block width"
    );
    assert!(
        top_bridge_clearance() >= 160.0,
        "camera bridge does not clear station modules"
    );
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn bag_nest_span_x() -> f64 {
    (REAGENT_BAG_POS_X - FEED_BAG_POS_X).abs() + BAG_NEST_X
}

fn route_lane_span() -> f64 {
    (ROUTE_LANES as f64 - 1.0) * ROUTE_LANE_PITCH
}

fn probe_span_x() -> f64 {
    (PROBE_PAIRS as f64 - 1.0) * PROBE_SPACING_X * 2.0
}

fn witness_well_bank_span() -> f64 {
    (WETNESS_WELLS as f64 - 1.0) * 45.0 + WITNESS_WELL_D
}

fn top_bridge_clearance() -> f64 {
    CAMERA_BRIDGE_Z - EQUIL_BLOCK_Z.max(BAG_NEST_Z).max(PROBE_BAR_Z)
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_expected_prefixed_stls() {
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_perfusion_reagent_temperature_mismatch_interlock_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_design_intent_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 20);
        assert!(REQUIRED_FEATURES.contains(&"bypass_interlock_gate"));
        assert!(REQUIRED_FEATURES.contains(&"wrong_bag_route_keying"));
        assert!(REQUIRED_FEATURES.contains(&"condensation_witness_wells"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
    }

    #[test]
    fn helper_dimensions_preserve_clearances() {
        assert!(bag_nest_span_x() < DECK_X - 2.0 * RIM_W - 140.0);
        assert!(route_lane_span() < ROUTE_COMB_X - 70.0);
        assert!(probe_span_x() < PROBE_BAR_X - 160.0);
        assert!(witness_well_bank_span() < WITNESS_X - 44.0);
        assert!(top_bridge_clearance() >= 160.0);
    }

    #[test]
    fn counts_match_validation_fixture_layout() {
        assert_eq!(ROUTE_LANES, 8);
        assert_eq!(PROBE_PAIRS * 2, 8);
        assert_eq!(LOGGER_DOCKS, 4);
        assert_eq!(WETNESS_WELLS + CONDENSATION_WELLS, 12);
        assert_eq!(BARCODE_LANDS + COA_LANDS, 9);
        assert_eq!(DISPOSITION_GATES, 3);
    }

    #[test]
    fn disposition_gate_names_are_stable() {
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }
}
