use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator evaporation/media-level sensor crosscheck validation station.
//
// Design intent:
// - Package a cassette/reservoir surrogate dock, gravimetric mass reference,
//   independent optical, capacitive, pressure, RH, and dewpoint sensor holders,
//   evaporation coupon positions, condensate diversion witnesses, custody
//   lands, evidence capture, and robot/service keepouts on one contained deck.
// - Scope the generator as mechanical validation packaging only. It is not a
//   sterile-process claim, assay SOP, or biological acceptance criterion.
// - Keep all modeled features as fixture geometry, datums, pockets, witness
//   lands, removable coupon holders, and keepout gauges.

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_containment_deck.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_cassette_reservoir_surrogate_dock.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_gravimetric_mass_pad.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_optical_level_sensor_holders.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_capacitive_pressure_probe_pockets.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_rh_dewpoint_logger_nests.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_edge_center_evaporation_coupon_grid.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_condensate_diversion_witness.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_barcode_coa_custody_lands.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_release_hold_reject_gates.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_camera_evidence_bridge.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_robot_service_keepouts.stl",
    "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 23] = [
    "mechanical_validation_packaging_only",
    "no_sterile_process_claim",
    "no_assay_sop",
    "no_biological_acceptance_criterion",
    "cassette_surrogate_dock",
    "reservoir_surrogate_dock",
    "gravimetric_mass_pad",
    "optical_level_sensor_holders",
    "capacitive_level_probe_pockets",
    "pressure_level_probe_pockets",
    "rh_logger_nests",
    "dewpoint_logger_nests",
    "edge_evaporation_coupon_grid",
    "center_evaporation_coupon_grid",
    "condensate_diversion_witness",
    "barcode_custody_lands",
    "coa_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const MECHANICAL_SCOPE_NOTE: &str =
    "mechanical validation packaging only; no sterile-process claim, assay SOP, or biological acceptance criterion";

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 52.0;
const SUMP_X: f64 = 1140.0;
const SUMP_Y: f64 = 688.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const DOCK_X: f64 = 500.0;
const DOCK_Y: f64 = 238.0;
const DOCK_Z: f64 = 46.0;
const DOCK_POS: (f64, f64) = (-384.0, 248.0);
const CASSETTE_SURROGATE_SLOTS: usize = 3;
const RESERVOIR_SURROGATE_WELLS: usize = 2;
const DOCK_ROUTE_PORTS: usize = 6;
const CASSETTE_SLOT_X: f64 = 126.0;
const CASSETTE_SLOT_Y: f64 = 82.0;
const CASSETTE_SLOT_PITCH: f64 = 142.0;
const RESERVOIR_WELL_D: f64 = 70.0;
const RESERVOIR_WELL_PITCH: f64 = 92.0;

const MASS_PAD_X: f64 = 414.0;
const MASS_PAD_Y: f64 = 260.0;
const MASS_PAD_Z: f64 = 42.0;
const MASS_PAD_POS: (f64, f64) = (300.0, 254.0);
const MASS_PAN_D: f64 = 146.0;
const LOAD_CELL_POCKET_X: f64 = 190.0;
const LOAD_CELL_POCKET_Y: f64 = 86.0;
const LOAD_CELL_POCKET_Z: f64 = 18.0;
const MASS_ISOLATION_MOAT_X: f64 = 300.0;
const MASS_ISOLATION_MOAT_Y: f64 = 180.0;
const MASS_ISOLATION_MOAT_W: f64 = 11.0;
const MASS_PAD_LEVELING_FEET: usize = 4;
const MASS_DRAFT_POSTS: usize = 4;
const MASS_READOUT_WINDOW_X: f64 = 112.0;

const OPTICAL_X: f64 = 572.0;
const OPTICAL_Y: f64 = 136.0;
const OPTICAL_Z: f64 = 44.0;
const OPTICAL_POS: (f64, f64) = (-300.0, 32.0);
const OPTICAL_LEVEL_HOLDERS: usize = 6;
const OPTICAL_LEVEL_WINDOWS: usize = 6;
const OPTICAL_HOLDER_PITCH: f64 = 82.0;
const OPTICAL_WINDOW_X: f64 = 36.0;
const OPTICAL_WINDOW_Y: f64 = 13.0;

const PROBE_X: f64 = 470.0;
const PROBE_Y: f64 = 154.0;
const PROBE_Z: f64 = 46.0;
const PROBE_POS: (f64, f64) = (306.0, 34.0);
const CAPACITIVE_PROBE_POCKETS: usize = 4;
const PRESSURE_PROBE_POCKETS: usize = 4;
const PROBE_PITCH_X: f64 = 82.0;
const CAPACITIVE_POCKET_D: f64 = 18.0;
const PRESSURE_POCKET_D: f64 = 16.0;
const PRESSURE_REFERENCE_PORTS: usize = 4;

const LOGGER_X: f64 = 358.0;
const LOGGER_Y: f64 = 170.0;
const LOGGER_Z: f64 = 38.0;
const LOGGER_POS: (f64, f64) = (-454.0, -154.0);
const RH_DEWPOINT_LOGGER_NESTS: usize = 4;
const LOGGER_POCKET_X: f64 = 72.0;
const LOGGER_POCKET_Y: f64 = 66.0;
const LOGGER_PITCH_X: f64 = 82.0;
const LOGGER_CABLE_COMBS: usize = 4;

const COUPON_GRID_X: f64 = 510.0;
const COUPON_GRID_Y: f64 = 276.0;
const COUPON_GRID_Z: f64 = 36.0;
const COUPON_GRID_POS: (f64, f64) = (68.0, -214.0);
const COUPON_GRID_COLS: usize = 5;
const COUPON_GRID_ROWS: usize = 5;
const EVAPORATION_COUPONS: usize = COUPON_GRID_COLS * COUPON_GRID_ROWS;
const EDGE_EVAPORATION_COUPONS: usize = 16;
const CENTER_EVAPORATION_COUPONS: usize = 9;
const COUPON_SLOT_X: f64 = 52.0;
const COUPON_SLOT_Y: f64 = 36.0;
const COUPON_SLOT_Z: f64 = 11.0;
const COUPON_PITCH_X: f64 = 78.0;
const COUPON_PITCH_Y: f64 = 52.0;

const WITNESS_X: f64 = 430.0;
const WITNESS_Y: f64 = 144.0;
const WITNESS_Z: f64 = 40.0;
const WITNESS_POS: (f64, f64) = (420.0, -178.0);
const CONDENSATE_DIVERSION_LANES: usize = 3;
const CONDENSATE_WITNESS_CUPS: usize = 5;
const DIVERSION_WEIR_CARDS: usize = 4;

const CUSTODY_X: f64 = 426.0;
const CUSTODY_Y: f64 = 118.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (-420.0, -342.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 4;
const CUSTODY_SEAL_TABS: usize = 4;

const GATE_X: f64 = 392.0;
const GATE_Y: f64 = 94.0;
const GATE_Z: f64 = 36.0;
const GATE_POS: (f64, f64) = (18.0, -348.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 960.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 204.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, 0.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const BRIDGE_UNDERSIDE_CLEARANCE: f64 = 132.0;

const ROBOT_KEEPOUT_X: f64 = 1210.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 76.0;
const SERVICE_KEEPOUT_X: f64 = 112.0;
const SERVICE_KEEPOUT_Y: f64 = 700.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 320.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(cassette_reservoir_surrogate_dock(), OUTPUTS[1]);
    write_part(gravimetric_mass_pad(), OUTPUTS[2]);
    write_part(optical_level_sensor_holders(), OUTPUTS[3]);
    write_part(capacitive_pressure_probe_pockets(), OUTPUTS[4]);
    write_part(rh_dewpoint_logger_nests(), OUTPUTS[5]);
    write_part(edge_center_evaporation_coupon_grid(), OUTPUTS[6]);
    write_part(condensate_diversion_witness(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed incubator evaporation/media-level sensor crosscheck station: {:.0}mm x {:.0}mm contained deck with {} cassette slots, {} reservoir wells, and {} gravimetric mass pad.",
        DECK_X,
        DECK_Y,
        CASSETTE_SURROGATE_SLOTS,
        RESERVOIR_SURROGATE_WELLS,
        MASS_PAN_D
    );
    println!(
        "Sensor crosscheck packaging: {} optical holders, {} capacitive pockets, {} pressure pockets, {} RH/dewpoint logger nests, and {} evaporation coupons.",
        OPTICAL_LEVEL_HOLDERS,
        CAPACITIVE_PROBE_POCKETS,
        PRESSURE_PROBE_POCKETS,
        RH_DEWPOINT_LOGGER_NESTS,
        EVAPORATION_COUPONS
    );
    println!(
        "Scope: mechanical validation packaging only; no sterile-process claim, assay SOP, or biological acceptance criterion."
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "incubator_evap_level_crosscheck_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "incubator_evap_level_crosscheck_recessed_spill_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - SUMP_Z / 2.0);
    let drain = centered_cylinder(
        "incubator_evap_level_crosscheck_sump_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 34.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 10.0, 0.0);

    deck - sump - drain + containment_rims() + deck_datum_bosses() + station_landing_recesses()
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "incubator_evap_level_crosscheck_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_z());
    let right = centered_cube(
        "incubator_evap_level_crosscheck_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());
    let rear = centered_cube(
        "incubator_evap_level_crosscheck_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let front = centered_cube(
        "incubator_evap_level_crosscheck_front_low_containment_lip",
        DECK_X - 148.0,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn deck_datum_bosses() -> Part {
    let mut bosses = Part::empty("incubator_evap_level_crosscheck_deck_datums");
    for (index, (x, y)) in deck_datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("incubator_evap_level_crosscheck_datum_boss_{index}"),
            13.5,
            6.0,
            36,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("incubator_evap_level_crosscheck_datum_bore_{index}"),
            3.4,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn station_landing_recesses() -> Part {
    deck_recess(
        "cassette_reservoir_dock",
        DOCK_X + 34.0,
        DOCK_Y + 30.0,
        DOCK_POS,
    ) + deck_recess(
        "gravimetric_mass_pad",
        MASS_PAD_X + 32.0,
        MASS_PAD_Y + 30.0,
        MASS_PAD_POS,
    ) + deck_recess(
        "optical_sensor_holders",
        OPTICAL_X + 30.0,
        OPTICAL_Y + 24.0,
        OPTICAL_POS,
    ) + deck_recess(
        "capacitive_pressure_probe_pockets",
        PROBE_X + 28.0,
        PROBE_Y + 26.0,
        PROBE_POS,
    ) + deck_recess(
        "rh_dewpoint_logger_nests",
        LOGGER_X + 26.0,
        LOGGER_Y + 24.0,
        LOGGER_POS,
    ) + deck_recess(
        "edge_center_evaporation_coupon_grid",
        COUPON_GRID_X + 30.0,
        COUPON_GRID_Y + 26.0,
        COUPON_GRID_POS,
    ) + deck_recess(
        "condensate_diversion_witness",
        WITNESS_X + 26.0,
        WITNESS_Y + 24.0,
        WITNESS_POS,
    )
}

fn deck_recess(name: &str, x: f64, y: f64, center: (f64, f64)) -> Part {
    centered_cube(
        format!("incubator_evap_level_crosscheck_{name}_landing_recess"),
        x,
        y,
        4.0,
    )
    .translate(center.0, center.1, DECK_Z / 2.0 - 1.5)
}

fn cassette_reservoir_surrogate_dock() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_cassette_reservoir_surrogate_dock_base",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let route_gutter = centered_cube(
        "incubator_evap_level_crosscheck_dock_common_route_gutter",
        DOCK_X - 56.0,
        18.0,
        18.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 + 30.0, 8.0);

    base - cassette_slot_cuts() - reservoir_well_cuts() - route_gutter
        + cassette_locator_rails()
        + reservoir_rim_lands()
        + dock_route_port_collars()
        + dock_asymmetric_keys()
}

fn cassette_slot_cuts() -> Part {
    let mut cuts = Part::empty("incubator_evap_level_crosscheck_cassette_slot_cuts");
    for index in 0..CASSETTE_SURROGATE_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("incubator_evap_level_crosscheck_cassette_surrogate_slot_{index}"),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                DOCK_Z + 4.0,
            )
            .translate(cassette_slot_x(index), 36.0, 8.0);
    }
    cuts
}

fn cassette_locator_rails() -> Part {
    let mut rails = Part::empty("incubator_evap_level_crosscheck_cassette_locator_rails");
    for index in 0..CASSETTE_SURROGATE_SLOTS {
        let x = cassette_slot_x(index);
        rails = rails
            + centered_cube(
                format!("incubator_evap_level_crosscheck_cassette_left_rail_{index}"),
                8.0,
                CASSETTE_SLOT_Y + 20.0,
                18.0,
            )
            .translate(x - CASSETTE_SLOT_X / 2.0 - 8.0, 36.0, DOCK_Z / 2.0 + 9.0)
            + centered_cube(
                format!("incubator_evap_level_crosscheck_cassette_right_rail_{index}"),
                8.0,
                CASSETTE_SLOT_Y + 20.0,
                18.0,
            )
            .translate(x + CASSETTE_SLOT_X / 2.0 + 8.0, 36.0, DOCK_Z / 2.0 + 9.0);
    }
    rails
}

fn reservoir_well_cuts() -> Part {
    let mut wells = Part::empty("incubator_evap_level_crosscheck_reservoir_well_cuts");
    for index in 0..RESERVOIR_SURROGATE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_reservoir_surrogate_well_{index}"),
                RESERVOIR_WELL_D / 2.0,
                DOCK_Z + 4.0,
                48,
            )
            .translate(reservoir_well_x(index), -70.0, 8.0);
    }
    wells
}

fn reservoir_rim_lands() -> Part {
    let mut rims = Part::empty("incubator_evap_level_crosscheck_reservoir_rim_lands");
    for index in 0..RESERVOIR_SURROGATE_WELLS {
        let rim = centered_cylinder(
            format!("incubator_evap_level_crosscheck_reservoir_rim_land_{index}"),
            RESERVOIR_WELL_D / 2.0 + 9.0,
            6.0,
            48,
        )
        .translate(reservoir_well_x(index), -70.0, DOCK_Z / 2.0 + 3.0);
        let clearance = centered_cylinder(
            format!("incubator_evap_level_crosscheck_reservoir_rim_clearance_{index}"),
            RESERVOIR_WELL_D / 2.0,
            8.0,
            48,
        )
        .translate(reservoir_well_x(index), -70.0, DOCK_Z / 2.0 + 3.0);
        rims = rims + (rim - clearance);
    }
    rims
}

fn dock_route_port_collars() -> Part {
    let mut collars = Part::empty("incubator_evap_level_crosscheck_dock_route_port_collars");
    for index in 0..DOCK_ROUTE_PORTS {
        let x = -185.0 + index as f64 * 74.0;
        let collar = centered_cylinder(
            format!("incubator_evap_level_crosscheck_dock_route_port_collar_{index}"),
            15.0,
            9.0,
            32,
        )
        .translate(x, -DOCK_Y / 2.0 + 18.0, DOCK_Z / 2.0 + 4.5);
        let bore = centered_cylinder(
            format!("incubator_evap_level_crosscheck_dock_route_port_bore_{index}"),
            5.2,
            12.0,
            24,
        )
        .translate(x, -DOCK_Y / 2.0 + 18.0, DOCK_Z / 2.0 + 4.5);
        collars = collars + (collar - bore);
    }
    collars
}

fn dock_asymmetric_keys() -> Part {
    let mut keys = Part::empty("incubator_evap_level_crosscheck_dock_asymmetric_keys");
    for index in 0..CASSETTE_SURROGATE_SLOTS {
        keys = keys
            + centered_cube(
                format!("incubator_evap_level_crosscheck_cassette_asymmetric_key_{index}"),
                28.0,
                18.0,
                12.0,
            )
            .translate(
                cassette_slot_x(index) - 38.0,
                DOCK_Y / 2.0 - 22.0,
                DOCK_Z / 2.0 + 6.0,
            );
    }
    keys
}

fn gravimetric_mass_pad() -> Part {
    let pad = centered_cube(
        "incubator_evap_level_crosscheck_gravimetric_mass_pad_base",
        MASS_PAD_X,
        MASS_PAD_Y,
        MASS_PAD_Z,
    );
    let readout_window = centered_cube(
        "incubator_evap_level_crosscheck_scale_readout_window",
        MASS_READOUT_WINDOW_X,
        18.0,
        20.0,
    )
    .translate(0.0, -MASS_PAD_Y / 2.0 - 1.0, 8.0);

    pad - load_cell_cavity() - mass_isolation_moat() - readout_window
        + mass_pan_registration_ring()
        + mass_leveling_feet()
        + mass_draft_posts()
        + overload_stop_blocks()
}

fn load_cell_cavity() -> Part {
    centered_cube(
        "incubator_evap_level_crosscheck_load_cell_service_pocket",
        LOAD_CELL_POCKET_X,
        LOAD_CELL_POCKET_Y,
        LOAD_CELL_POCKET_Z + 1.0,
    )
    .translate(0.0, 0.0, MASS_PAD_Z / 2.0 - LOAD_CELL_POCKET_Z / 2.0)
}

fn mass_isolation_moat() -> Part {
    let north = centered_cube(
        "incubator_evap_level_crosscheck_mass_isolation_moat_north",
        MASS_ISOLATION_MOAT_X,
        MASS_ISOLATION_MOAT_W,
        MASS_PAD_Z + 2.0,
    )
    .translate(0.0, MASS_ISOLATION_MOAT_Y / 2.0, 0.0);
    let south = centered_cube(
        "incubator_evap_level_crosscheck_mass_isolation_moat_south",
        MASS_ISOLATION_MOAT_X,
        MASS_ISOLATION_MOAT_W,
        MASS_PAD_Z + 2.0,
    )
    .translate(0.0, -MASS_ISOLATION_MOAT_Y / 2.0, 0.0);
    let east = centered_cube(
        "incubator_evap_level_crosscheck_mass_isolation_moat_east",
        MASS_ISOLATION_MOAT_W,
        MASS_ISOLATION_MOAT_Y,
        MASS_PAD_Z + 2.0,
    )
    .translate(MASS_ISOLATION_MOAT_X / 2.0, 0.0, 0.0);
    let west = centered_cube(
        "incubator_evap_level_crosscheck_mass_isolation_moat_west",
        MASS_ISOLATION_MOAT_W,
        MASS_ISOLATION_MOAT_Y,
        MASS_PAD_Z + 2.0,
    )
    .translate(-MASS_ISOLATION_MOAT_X / 2.0, 0.0, 0.0);

    north + south + east + west
}

fn mass_pan_registration_ring() -> Part {
    let ring = centered_cylinder(
        "incubator_evap_level_crosscheck_mass_pan_registration_ring",
        MASS_PAN_D / 2.0 + 9.0,
        6.0,
        64,
    )
    .translate(0.0, 0.0, MASS_PAD_Z / 2.0 + 3.0);
    let clearance = centered_cylinder(
        "incubator_evap_level_crosscheck_mass_pan_clearance",
        MASS_PAN_D / 2.0,
        8.0,
        64,
    )
    .translate(0.0, 0.0, MASS_PAD_Z / 2.0 + 3.0);

    ring - clearance
}

fn mass_leveling_feet() -> Part {
    let mut feet = Part::empty("incubator_evap_level_crosscheck_mass_leveling_feet");
    for (index, (x, y)) in mass_foot_positions().iter().enumerate() {
        let foot = centered_cylinder(
            format!("incubator_evap_level_crosscheck_mass_leveling_foot_{index}"),
            13.0,
            8.0,
            32,
        )
        .translate(*x, *y, -MASS_PAD_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("incubator_evap_level_crosscheck_mass_leveling_bore_{index}"),
            4.2,
            10.0,
            24,
        )
        .translate(*x, *y, -MASS_PAD_Z / 2.0 + 4.0);
        feet = feet + (foot - bore);
    }
    feet
}

fn mass_draft_posts() -> Part {
    let mut posts = Part::empty("incubator_evap_level_crosscheck_mass_draft_posts");
    for (index, (x, y)) in mass_foot_positions().iter().enumerate() {
        posts = posts
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_draft_reference_post_{index}"),
                6.0,
                70.0,
                28,
            )
            .translate(*x, *y, MASS_PAD_Z / 2.0 + 35.0);
    }
    posts
}

fn overload_stop_blocks() -> Part {
    let mut stops = Part::empty("incubator_evap_level_crosscheck_overload_stop_blocks");
    for (index, y) in [-70.0, 70.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("incubator_evap_level_crosscheck_overload_stop_{index}"),
                132.0,
                12.0,
                12.0,
            )
            .translate(0.0, *y, MASS_PAD_Z / 2.0 + 6.0);
    }
    stops
}

fn optical_level_sensor_holders() -> Part {
    let rail = centered_cube(
        "incubator_evap_level_crosscheck_optical_level_sensor_holder_rail",
        OPTICAL_X,
        OPTICAL_Y,
        OPTICAL_Z,
    );

    rail - optical_window_cuts() - optical_cable_slots()
        + optical_sensor_saddles()
        + optical_level_tick_lands()
        + optical_shroud_lips()
}

fn optical_window_cuts() -> Part {
    let mut cuts = Part::empty("incubator_evap_level_crosscheck_optical_window_cuts");
    for index in 0..OPTICAL_LEVEL_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("incubator_evap_level_crosscheck_optical_level_read_window_{index}"),
                OPTICAL_WINDOW_X,
                OPTICAL_WINDOW_Y,
                OPTICAL_Z + 4.0,
            )
            .translate(optical_holder_x(index), -OPTICAL_Y / 2.0 + 18.0, 4.0);
    }
    cuts
}

fn optical_cable_slots() -> Part {
    let mut slots = Part::empty("incubator_evap_level_crosscheck_optical_cable_slots");
    for index in 0..OPTICAL_LEVEL_HOLDERS {
        slots = slots
            + centered_cube(
                format!("incubator_evap_level_crosscheck_optical_sensor_cable_slot_{index}"),
                10.0,
                62.0,
                16.0,
            )
            .translate(optical_holder_x(index), OPTICAL_Y / 2.0 - 34.0, 4.0);
    }
    slots
}

fn optical_sensor_saddles() -> Part {
    let mut saddles = Part::empty("incubator_evap_level_crosscheck_optical_sensor_saddles");
    for index in 0..OPTICAL_LEVEL_HOLDERS {
        let x = optical_holder_x(index);
        let left = centered_cube(
            format!("incubator_evap_level_crosscheck_optical_left_saddle_{index}"),
            8.0,
            54.0,
            24.0,
        )
        .translate(x - 25.0, 0.0, OPTICAL_Z / 2.0 + 12.0);
        let right = centered_cube(
            format!("incubator_evap_level_crosscheck_optical_right_saddle_{index}"),
            8.0,
            54.0,
            24.0,
        )
        .translate(x + 25.0, 0.0, OPTICAL_Z / 2.0 + 12.0);
        saddles = saddles + left + right;
    }
    saddles
}

fn optical_level_tick_lands() -> Part {
    let mut ticks = Part::empty("incubator_evap_level_crosscheck_optical_level_tick_lands");
    for index in 0..OPTICAL_LEVEL_HOLDERS {
        ticks = ticks
            + centered_cube(
                format!("incubator_evap_level_crosscheck_optical_level_tick_{index}"),
                38.0,
                5.0,
                3.0,
            )
            .translate(
                optical_holder_x(index),
                -OPTICAL_Y / 2.0 - 6.0,
                OPTICAL_Z / 2.0 + 1.5,
            );
    }
    ticks
}

fn optical_shroud_lips() -> Part {
    let front = centered_cube(
        "incubator_evap_level_crosscheck_optical_front_shroud_lip",
        OPTICAL_X,
        10.0,
        18.0,
    )
    .translate(0.0, -OPTICAL_Y / 2.0 - 5.0, OPTICAL_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "incubator_evap_level_crosscheck_optical_rear_cable_lip",
        OPTICAL_X,
        10.0,
        18.0,
    )
    .translate(0.0, OPTICAL_Y / 2.0 + 5.0, OPTICAL_Z / 2.0 + 9.0);

    front + rear
}

fn capacitive_pressure_probe_pockets() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_capacitive_pressure_probe_pocket_base",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    );

    base - capacitive_probe_bores() - pressure_probe_bores() - pressure_reference_channels()
        + probe_bosses("capacitive", -34.0, CAPACITIVE_PROBE_POCKETS)
        + probe_bosses("pressure", 34.0, PRESSURE_PROBE_POCKETS)
        + probe_cable_comb()
        + probe_identity_tabs()
}

fn capacitive_probe_bores() -> Part {
    let mut bores = Part::empty("incubator_evap_level_crosscheck_capacitive_probe_bores");
    for index in 0..CAPACITIVE_PROBE_POCKETS {
        bores = bores
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_capacitive_level_probe_pocket_{index}"),
                CAPACITIVE_POCKET_D / 2.0,
                PROBE_Z + 8.0,
                32,
            )
            .translate(probe_x(index), -34.0, 0.0);
    }
    bores
}

fn pressure_probe_bores() -> Part {
    let mut bores = Part::empty("incubator_evap_level_crosscheck_pressure_probe_bores");
    for index in 0..PRESSURE_PROBE_POCKETS {
        bores = bores
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_pressure_level_probe_pocket_{index}"),
                PRESSURE_POCKET_D / 2.0,
                PROBE_Z + 8.0,
                32,
            )
            .translate(probe_x(index), 34.0, 0.0);
    }
    bores
}

fn pressure_reference_channels() -> Part {
    let mut channels = Part::empty("incubator_evap_level_crosscheck_pressure_reference_channels");
    for index in 0..PRESSURE_REFERENCE_PORTS {
        channels = channels
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_pressure_reference_port_{index}"),
                4.0,
                PROBE_Y + 12.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(probe_x(index), 0.0, 2.0);
    }
    channels
}

fn probe_bosses(kind: &str, y: f64, count: usize) -> Part {
    let mut bosses = Part::empty(format!(
        "incubator_evap_level_crosscheck_{kind}_probe_bosses"
    ));
    for index in 0..count {
        let x = probe_x(index);
        let boss = centered_cylinder(
            format!("incubator_evap_level_crosscheck_{kind}_probe_boss_{index}"),
            18.0,
            7.0,
            36,
        )
        .translate(x, y, PROBE_Z / 2.0 + 3.5);
        let label = centered_cube(
            format!("incubator_evap_level_crosscheck_{kind}_probe_label_land_{index}"),
            46.0,
            14.0,
            3.0,
        )
        .translate(
            x,
            y + if y < 0.0 { -24.0 } else { 24.0 },
            PROBE_Z / 2.0 + 2.0,
        );
        bosses = bosses + boss + label;
    }
    bosses
}

fn probe_cable_comb() -> Part {
    let comb = centered_cube(
        "incubator_evap_level_crosscheck_probe_cable_comb",
        PROBE_X - 52.0,
        20.0,
        20.0,
    )
    .translate(0.0, PROBE_Y / 2.0 + 10.0, PROBE_Z / 2.0 + 10.0);
    let mut slots = Part::empty("incubator_evap_level_crosscheck_probe_cable_comb_slots");
    for index in 0..CAPACITIVE_PROBE_POCKETS {
        slots = slots
            + centered_cube(
                format!("incubator_evap_level_crosscheck_probe_cable_comb_slot_{index}"),
                12.0,
                26.0,
                22.0,
            )
            .translate(probe_x(index), PROBE_Y / 2.0 + 10.0, PROBE_Z / 2.0 + 10.0);
    }
    comb - slots
}

fn probe_identity_tabs() -> Part {
    let mut tabs = Part::empty("incubator_evap_level_crosscheck_probe_identity_tabs");
    for index in 0..CAPACITIVE_PROBE_POCKETS {
        tabs = tabs
            + centered_cube(
                format!("incubator_evap_level_crosscheck_probe_pair_identity_tab_{index}"),
                42.0,
                7.0,
                5.0,
            )
            .translate(probe_x(index), -PROBE_Y / 2.0 - 6.0, PROBE_Z / 2.0 + 2.5);
    }
    tabs
}

fn rh_dewpoint_logger_nests() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_rh_dewpoint_logger_nest_base",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );

    base - logger_pocket_cuts() + logger_cable_combs() + logger_status_lands() + logger_clip_tabs()
}

fn logger_pocket_cuts() -> Part {
    let mut pockets = Part::empty("incubator_evap_level_crosscheck_logger_pocket_cuts");
    for index in 0..RH_DEWPOINT_LOGGER_NESTS {
        pockets = pockets
            + centered_cube(
                format!("incubator_evap_level_crosscheck_rh_dewpoint_logger_pocket_{index}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_Z + 4.0,
            )
            .translate(logger_x(index), -16.0, 7.0);
    }
    pockets
}

fn logger_cable_combs() -> Part {
    let mut combs = Part::empty("incubator_evap_level_crosscheck_logger_cable_combs");
    for index in 0..LOGGER_CABLE_COMBS {
        let comb = centered_cube(
            format!("incubator_evap_level_crosscheck_logger_cable_comb_{index}"),
            12.0,
            50.0,
            21.0,
        )
        .translate(
            logger_x(index),
            LOGGER_Y / 2.0 - 24.0,
            LOGGER_Z / 2.0 + 10.5,
        );
        let slot = centered_cube(
            format!("incubator_evap_level_crosscheck_logger_cable_slot_{index}"),
            5.0,
            54.0,
            23.0,
        )
        .translate(
            logger_x(index),
            LOGGER_Y / 2.0 - 24.0,
            LOGGER_Z / 2.0 + 10.5,
        );
        combs = combs + (comb - slot);
    }
    combs
}

fn logger_status_lands() -> Part {
    let mut lands = Part::empty("incubator_evap_level_crosscheck_logger_status_lands");
    for index in 0..RH_DEWPOINT_LOGGER_NESTS {
        lands = lands
            + centered_cube(
                format!("incubator_evap_level_crosscheck_rh_logger_status_land_{index}"),
                32.0,
                10.0,
                3.0,
            )
            .translate(
                logger_x(index) - 18.0,
                -LOGGER_Y / 2.0 + 22.0,
                LOGGER_Z / 2.0 + 1.5,
            )
            + centered_cube(
                format!("incubator_evap_level_crosscheck_dewpoint_logger_status_land_{index}"),
                32.0,
                10.0,
                3.0,
            )
            .translate(
                logger_x(index) + 18.0,
                -LOGGER_Y / 2.0 + 22.0,
                LOGGER_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn logger_clip_tabs() -> Part {
    let mut tabs = Part::empty("incubator_evap_level_crosscheck_logger_clip_tabs");
    for index in 0..RH_DEWPOINT_LOGGER_NESTS {
        tabs =
            tabs + centered_cube(
                format!("incubator_evap_level_crosscheck_logger_left_clip_{index}"),
                7.0,
                LOGGER_POCKET_Y,
                18.0,
            )
            .translate(
                logger_x(index) - LOGGER_POCKET_X / 2.0 - 6.0,
                -16.0,
                LOGGER_Z / 2.0 + 9.0,
            ) + centered_cube(
                format!("incubator_evap_level_crosscheck_logger_right_clip_{index}"),
                7.0,
                LOGGER_POCKET_Y,
                18.0,
            )
            .translate(
                logger_x(index) + LOGGER_POCKET_X / 2.0 + 6.0,
                -16.0,
                LOGGER_Z / 2.0 + 9.0,
            );
    }
    tabs
}

fn edge_center_evaporation_coupon_grid() -> Part {
    let tray = centered_cube(
        "incubator_evap_level_crosscheck_edge_center_evaporation_coupon_grid_tray",
        COUPON_GRID_X,
        COUPON_GRID_Y,
        COUPON_GRID_Z,
    );
    let humidity_baffle = centered_cube(
        "incubator_evap_level_crosscheck_coupon_grid_humidity_baffle_land",
        COUPON_GRID_X - 48.0,
        12.0,
        16.0,
    )
    .translate(0.0, COUPON_GRID_Y / 2.0 + 6.0, COUPON_GRID_Z / 2.0 + 8.0);

    tray - coupon_socket_cuts()
        + evaporation_coupons()
        + coupon_edge_center_flags()
        + humidity_baffle
}

fn coupon_socket_cuts() -> Part {
    let mut cuts = Part::empty("incubator_evap_level_crosscheck_coupon_socket_cuts");
    for index in 0..EVAPORATION_COUPONS {
        let (x, y) = coupon_center(index);
        cuts = cuts
            + centered_cube(
                format!("incubator_evap_level_crosscheck_evaporation_coupon_socket_{index}"),
                COUPON_SLOT_X + 6.0,
                COUPON_SLOT_Y + 6.0,
                COUPON_SLOT_Z + 3.0,
            )
            .translate(x, y, COUPON_GRID_Z / 2.0 - COUPON_SLOT_Z / 2.0);
    }
    cuts
}

fn evaporation_coupons() -> Part {
    let mut coupons = Part::empty("incubator_evap_level_crosscheck_evaporation_coupons");
    for index in 0..EVAPORATION_COUPONS {
        let (x, y) = coupon_center(index);
        let coupon_z = if coupon_is_edge(index) { 9.0 } else { 12.0 };
        coupons = coupons
            + centered_cube(
                format!("incubator_evap_level_crosscheck_evaporation_coupon_{index}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                coupon_z,
            )
            .translate(x, y, COUPON_GRID_Z / 2.0 + coupon_z / 2.0);
    }
    coupons
}

fn coupon_edge_center_flags() -> Part {
    let mut flags = Part::empty("incubator_evap_level_crosscheck_coupon_edge_center_flags");
    for index in 0..EVAPORATION_COUPONS {
        let (x, y) = coupon_center(index);
        let flag_y = if coupon_is_edge(index) { -18.0 } else { 18.0 };
        flags = flags
            + centered_cube(
                format!("incubator_evap_level_crosscheck_coupon_edge_center_flag_{index}"),
                24.0,
                5.0,
                3.0,
            )
            .translate(x, y + flag_y, COUPON_GRID_Z / 2.0 + 1.5);
    }
    flags
}

fn condensate_diversion_witness() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_condensate_diversion_witness_base",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let trough = centered_cube(
        "incubator_evap_level_crosscheck_condensate_diversion_common_trough",
        WITNESS_X - 44.0,
        18.0,
        18.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 26.0, 8.0);

    base - diversion_lane_cuts() - witness_cup_cuts() - trough
        + diversion_weir_cards()
        + witness_cup_rims()
        + diversion_arrow_lands()
}

fn diversion_lane_cuts() -> Part {
    let mut lanes = Part::empty("incubator_evap_level_crosscheck_condensate_diversion_lanes");
    for index in 0..CONDENSATE_DIVERSION_LANES {
        lanes = lanes
            + centered_cube(
                format!("incubator_evap_level_crosscheck_condensate_diversion_lane_{index}"),
                WITNESS_X - 72.0,
                13.0,
                WITNESS_Z + 4.0,
            )
            .translate(0.0, -22.0 + index as f64 * 34.0, 8.0);
    }
    lanes
}

fn witness_cup_cuts() -> Part {
    let mut cups = Part::empty("incubator_evap_level_crosscheck_condensate_witness_cup_cuts");
    for index in 0..CONDENSATE_WITNESS_CUPS {
        cups = cups
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_condensate_witness_cup_{index}"),
                15.0,
                18.0,
                36,
            )
            .translate(
                -160.0 + index as f64 * 80.0,
                -WITNESS_Y / 2.0 + 28.0,
                WITNESS_Z / 2.0,
            );
    }
    cups
}

fn diversion_weir_cards() -> Part {
    let mut cards = Part::empty("incubator_evap_level_crosscheck_diversion_weir_cards");
    for index in 0..DIVERSION_WEIR_CARDS {
        cards = cards
            + centered_cube(
                format!("incubator_evap_level_crosscheck_condensate_weir_card_{index}"),
                10.0,
                74.0,
                22.0 + index as f64 * 4.0,
            )
            .translate(-150.0 + index as f64 * 100.0, 18.0, WITNESS_Z / 2.0 + 11.0);
    }
    cards
}

fn witness_cup_rims() -> Part {
    let mut rims = Part::empty("incubator_evap_level_crosscheck_witness_cup_rims");
    for index in 0..CONDENSATE_WITNESS_CUPS {
        rims = rims
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_condensate_witness_cup_rim_{index}"),
                19.0,
                5.0,
                36,
            )
            .translate(
                -160.0 + index as f64 * 80.0,
                -WITNESS_Y / 2.0 + 28.0,
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    rims
}

fn diversion_arrow_lands() -> Part {
    let mut arrows = Part::empty("incubator_evap_level_crosscheck_diversion_arrow_lands");
    for index in 0..CONDENSATE_DIVERSION_LANES {
        arrows = arrows
            + centered_cube(
                format!("incubator_evap_level_crosscheck_diversion_flow_arrow_land_{index}"),
                48.0,
                7.0,
                3.0,
            )
            .translate(
                WITNESS_X / 2.0 - 72.0,
                -22.0 + index as f64 * 34.0,
                WITNESS_Z / 2.0 + 1.5,
            );
    }
    arrows
}

fn barcode_coa_custody_lands() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_barcode_coa_custody_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    base + barcode_lands() + coa_lands() + custody_seal_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("incubator_evap_level_crosscheck_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let x = -150.0 + (index % 4) as f64 * 100.0;
        let y = -30.0 + (index / 4) as f64 * 38.0;
        lands = lands
            + centered_cube(
                format!("incubator_evap_level_crosscheck_barcode_custody_land_{index}"),
                78.0,
                22.0,
                2.5,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("incubator_evap_level_crosscheck_coa_custody_lands");
    for index in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("incubator_evap_level_crosscheck_coa_land_{index}"),
                86.0,
                24.0,
                2.5,
            )
            .translate(
                -150.0 + index as f64 * 100.0,
                CUSTODY_Y / 2.0 - 20.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_seal_tabs() -> Part {
    let mut tabs = Part::empty("incubator_evap_level_crosscheck_custody_seal_tabs");
    for index in 0..CUSTODY_SEAL_TABS {
        let x = if index % 2 == 0 { -196.0 } else { 196.0 };
        let y = if index < 2 { -48.0 } else { 48.0 };
        tabs = tabs
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_custody_seal_tab_{index}"),
                10.0,
                3.0,
                28,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.5);
    }
    tabs
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "incubator_evap_level_crosscheck_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = Part::empty("incubator_evap_level_crosscheck_disposition_gates");
    for index in 0..DISPOSITION_GATES {
        gates = gates + disposition_gate(index);
    }

    base - gate_token_slot_cuts() + gates
}

fn disposition_gate(index: usize) -> Part {
    let name = disposition_gate_name(index);
    let x = gate_x(index);
    let handle = centered_cube(
        format!("incubator_evap_level_crosscheck_{name}_gate_handle"),
        72.0,
        28.0,
        24.0,
    )
    .translate(x, -GATE_Y / 2.0 + 26.0, GATE_Z / 2.0 + 12.0);
    let flag = centered_cube(
        format!("incubator_evap_level_crosscheck_{name}_gate_status_flag"),
        52.0,
        10.0,
        40.0,
    )
    .translate(x, GATE_Y / 2.0 - 14.0, GATE_Z / 2.0 + 20.0);

    handle + flag
}

fn gate_token_slot_cuts() -> Part {
    let mut slots = Part::empty("incubator_evap_level_crosscheck_gate_token_slot_cuts");
    for index in 0..GATE_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("incubator_evap_level_crosscheck_disposition_token_slot_{index}"),
                34.0,
                24.0,
                14.0,
            )
            .translate(-145.0 + index as f64 * 58.0, 0.0, GATE_Z / 2.0 - 5.0);
    }
    slots
}

fn camera_evidence_bridge() -> Part {
    let rail = centered_cube(
        "incubator_evap_level_crosscheck_camera_evidence_bridge_crossrail",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        34.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z);
    let left_post = camera_bridge_post(-CAMERA_BRIDGE_X / 2.0 + 34.0);
    let right_post = camera_bridge_post(CAMERA_BRIDGE_X / 2.0 - 34.0);

    rail + left_post + right_post + camera_mounts() + evidence_fiducials()
}

fn camera_bridge_post(x: f64) -> Part {
    let post = centered_cube(
        format!("incubator_evap_level_crosscheck_camera_bridge_post_{x:.0}"),
        34.0,
        38.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let foot = centered_cube(
        format!("incubator_evap_level_crosscheck_camera_bridge_foot_{x:.0}"),
        72.0,
        60.0,
        12.0,
    )
    .translate(x, 0.0, 6.0);

    post + foot
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("incubator_evap_level_crosscheck_camera_mounts");
    for index in 0..CAMERA_MOUNTS {
        let x = -320.0 + index as f64 * 160.0;
        let mount = centered_cube(
            format!("incubator_evap_level_crosscheck_camera_mount_plate_{index}"),
            72.0,
            16.0,
            12.0,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 14.0, CAMERA_BRIDGE_Z + 26.0);
        let bore = centered_cylinder(
            format!("incubator_evap_level_crosscheck_camera_mount_bore_{index}"),
            4.0,
            18.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 14.0, CAMERA_BRIDGE_Z + 26.0);
        mounts = mounts + (mount - bore);
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("incubator_evap_level_crosscheck_evidence_fiducials");
    for index in 0..EVIDENCE_FIDUCIALS {
        let x = -450.0 + (index % 5) as f64 * 225.0;
        let y = if index < 5 { -60.0 } else { 60.0 };
        fiducials = fiducials
            + centered_cylinder(
                format!("incubator_evap_level_crosscheck_evidence_fiducial_{index}"),
                8.0,
                4.0,
                28,
            )
            .translate(x, y, 2.0);
    }
    fiducials
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "incubator_evap_level_crosscheck_front_robot_approach_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W + ROBOT_KEEPOUT_Y / 2.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let rear_robot = centered_cube(
        "incubator_evap_level_crosscheck_rear_robot_approach_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - RIM_W - ROBOT_KEEPOUT_Y / 2.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "incubator_evap_level_crosscheck_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + RIM_W + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "incubator_evap_level_crosscheck_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 - RIM_W - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "incubator_evap_level_crosscheck_top_service_clearance_plane",
        DECK_X - 2.0 * RIM_W,
        DECK_Y - 2.0 * RIM_W,
        4.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + rear_robot + left_service + right_service + top_clearance
}

fn station_assembly() -> Part {
    containment_deck()
        + cassette_reservoir_surrogate_dock().translate(DOCK_POS.0, DOCK_POS.1, top_z(DOCK_Z))
        + gravimetric_mass_pad().translate(MASS_PAD_POS.0, MASS_PAD_POS.1, top_z(MASS_PAD_Z))
        + optical_level_sensor_holders().translate(OPTICAL_POS.0, OPTICAL_POS.1, top_z(OPTICAL_Z))
        + capacitive_pressure_probe_pockets().translate(PROBE_POS.0, PROBE_POS.1, top_z(PROBE_Z))
        + rh_dewpoint_logger_nests().translate(LOGGER_POS.0, LOGGER_POS.1, top_z(LOGGER_Z))
        + edge_center_evaporation_coupon_grid().translate(
            COUPON_GRID_POS.0,
            COUPON_GRID_POS.1,
            top_z(COUPON_GRID_Z),
        )
        + condensate_diversion_witness().translate(WITNESS_POS.0, WITNESS_POS.1, top_z(WITNESS_Z))
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge().translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, DECK_Z / 2.0)
        + robot_service_keepouts()
}

fn deck_datum_positions() -> [(f64, f64); DATUM_BOSSES] {
    [
        (-600.0, -382.0),
        (600.0, -382.0),
        (-600.0, 382.0),
        (600.0, 382.0),
        (-320.0, -382.0),
        (320.0, -382.0),
        (-320.0, 382.0),
        (320.0, 382.0),
        (0.0, -382.0),
        (0.0, 382.0),
    ]
}

fn cassette_slot_x(index: usize) -> f64 {
    -((CASSETTE_SURROGATE_SLOTS as f64 - 1.0) * CASSETTE_SLOT_PITCH) / 2.0
        + index as f64 * CASSETTE_SLOT_PITCH
}

fn reservoir_well_x(index: usize) -> f64 {
    142.0 + (index as f64 - 0.5) * RESERVOIR_WELL_PITCH
}

fn mass_foot_positions() -> [(f64, f64); MASS_PAD_LEVELING_FEET] {
    [
        (-MASS_PAD_X / 2.0 + 32.0, -MASS_PAD_Y / 2.0 + 32.0),
        (MASS_PAD_X / 2.0 - 32.0, -MASS_PAD_Y / 2.0 + 32.0),
        (-MASS_PAD_X / 2.0 + 32.0, MASS_PAD_Y / 2.0 - 32.0),
        (MASS_PAD_X / 2.0 - 32.0, MASS_PAD_Y / 2.0 - 32.0),
    ]
}

fn optical_holder_x(index: usize) -> f64 {
    -((OPTICAL_LEVEL_HOLDERS as f64 - 1.0) * OPTICAL_HOLDER_PITCH) / 2.0
        + index as f64 * OPTICAL_HOLDER_PITCH
}

fn probe_x(index: usize) -> f64 {
    -((CAPACITIVE_PROBE_POCKETS as f64 - 1.0) * PROBE_PITCH_X) / 2.0 + index as f64 * PROBE_PITCH_X
}

fn logger_x(index: usize) -> f64 {
    -((RH_DEWPOINT_LOGGER_NESTS as f64 - 1.0) * LOGGER_PITCH_X) / 2.0
        + index as f64 * LOGGER_PITCH_X
}

fn coupon_center(index: usize) -> (f64, f64) {
    let row = index / COUPON_GRID_COLS;
    let col = index % COUPON_GRID_COLS;
    (
        -((COUPON_GRID_COLS as f64 - 1.0) * COUPON_PITCH_X) / 2.0 + col as f64 * COUPON_PITCH_X,
        -((COUPON_GRID_ROWS as f64 - 1.0) * COUPON_PITCH_Y) / 2.0 + row as f64 * COUPON_PITCH_Y,
    )
}

fn coupon_is_edge(index: usize) -> bool {
    let row = index / COUPON_GRID_COLS;
    let col = index % COUPON_GRID_COLS;
    row == 0 || row == COUPON_GRID_ROWS - 1 || col == 0 || col == COUPON_GRID_COLS - 1
}

fn gate_x(index: usize) -> f64 {
    -112.0 + index as f64 * 112.0
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index {index}"),
    }
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn assert_layout() {
    assert_eq!(DATUM_BOSSES, 10);
    assert_eq!(CASSETTE_SURROGATE_SLOTS, 3);
    assert_eq!(RESERVOIR_SURROGATE_WELLS, 2);
    assert_eq!(MASS_DRAFT_POSTS, MASS_PAD_LEVELING_FEET);
    assert_eq!(OPTICAL_LEVEL_HOLDERS, OPTICAL_LEVEL_WINDOWS);
    assert_eq!(CAPACITIVE_PROBE_POCKETS, PRESSURE_PROBE_POCKETS);
    assert_eq!(PRESSURE_REFERENCE_PORTS, PRESSURE_PROBE_POCKETS);
    assert_eq!(LOGGER_CABLE_COMBS, RH_DEWPOINT_LOGGER_NESTS);
    assert_eq!(EVAPORATION_COUPONS, COUPON_GRID_COLS * COUPON_GRID_ROWS);
    assert_eq!(
        EDGE_EVAPORATION_COUPONS + CENTER_EVAPORATION_COUPONS,
        EVAPORATION_COUPONS
    );
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX, 0);
    assert_eq!(HOLD_GATE_INDEX, 1);
    assert_eq!(REJECT_GATE_INDEX, 2);
    assert!(MASS_PAD_X > MASS_ISOLATION_MOAT_X + 2.0 * MASS_ISOLATION_MOAT_W);
    assert!(MASS_PAD_Y > MASS_ISOLATION_MOAT_Y + 2.0 * MASS_ISOLATION_MOAT_W);
    assert!(LOAD_CELL_POCKET_X > MASS_PAN_D);
    assert!(cassette_slot_span() + CASSETTE_SLOT_X < DOCK_X - 42.0);
    assert!(optical_holder_span() + OPTICAL_WINDOW_X < OPTICAL_X - 44.0);
    assert!(probe_span() + CAPACITIVE_POCKET_D < PROBE_X - 120.0);
    assert!(coupon_grid_span_x() + COUPON_SLOT_X < COUPON_GRID_X - 80.0);
    assert!(coupon_grid_span_y() + COUPON_SLOT_Y < COUPON_GRID_Y - 20.0);
    assert!(BRIDGE_UNDERSIDE_CLEARANCE > RIM_Z + DECK_Z);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
}

fn cassette_slot_span() -> f64 {
    (CASSETTE_SURROGATE_SLOTS - 1) as f64 * CASSETTE_SLOT_PITCH
}

fn optical_holder_span() -> f64 {
    (OPTICAL_LEVEL_HOLDERS - 1) as f64 * OPTICAL_HOLDER_PITCH
}

fn probe_span() -> f64 {
    (CAPACITIVE_PROBE_POCKETS - 1) as f64 * PROBE_PITCH_X
}

fn coupon_grid_span_x() -> f64 {
    (COUPON_GRID_COLS - 1) as f64 * COUPON_PITCH_X
}

fn coupon_grid_span_y() -> f64 {
    (COUPON_GRID_ROWS - 1) as f64 * COUPON_PITCH_Y
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Rect {
    center: (f64, f64),
    x: f64,
    y: f64,
}

#[cfg(test)]
fn rect_fits_deck(rect: Rect, margin: f64) -> bool {
    rect.center.0.abs() + rect.x / 2.0 <= DECK_X / 2.0 - margin
        && rect.center.1.abs() + rect.y / 2.0 <= DECK_Y / 2.0 - margin
}

#[cfg(test)]
fn major_module_rects() -> Vec<Rect> {
    vec![
        Rect {
            center: DOCK_POS,
            x: DOCK_X,
            y: DOCK_Y,
        },
        Rect {
            center: MASS_PAD_POS,
            x: MASS_PAD_X,
            y: MASS_PAD_Y,
        },
        Rect {
            center: OPTICAL_POS,
            x: OPTICAL_X,
            y: OPTICAL_Y,
        },
        Rect {
            center: PROBE_POS,
            x: PROBE_X,
            y: PROBE_Y,
        },
        Rect {
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            center: COUPON_GRID_POS,
            x: COUPON_GRID_X,
            y: COUPON_GRID_Y,
        },
        Rect {
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
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
            assert!(path.starts_with(
                "output/closed_incubator_evaporation_media_level_sensor_crosscheck_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn requested_feature_scope_is_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 23);
        for feature in [
            "mechanical_validation_packaging_only",
            "no_sterile_process_claim",
            "no_assay_sop",
            "no_biological_acceptance_criterion",
            "cassette_surrogate_dock",
            "reservoir_surrogate_dock",
            "gravimetric_mass_pad",
            "optical_level_sensor_holders",
            "capacitive_level_probe_pockets",
            "pressure_level_probe_pockets",
            "rh_logger_nests",
            "dewpoint_logger_nests",
            "edge_evaporation_coupon_grid",
            "center_evaporation_coupon_grid",
            "condensate_diversion_witness",
            "barcode_custody_lands",
            "coa_custody_lands",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "camera_evidence_bridge",
            "robot_keepouts",
            "service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn scope_note_excludes_process_and_biology_claims() {
        assert!(MECHANICAL_SCOPE_NOTE.contains("mechanical validation packaging only"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("no sterile-process claim"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("assay SOP"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("biological acceptance criterion"));
    }

    #[test]
    fn repeated_feature_counts_cover_crosscheck_station() {
        assert_eq!(CASSETTE_SURROGATE_SLOTS, 3);
        assert_eq!(RESERVOIR_SURROGATE_WELLS, 2);
        assert_eq!(DOCK_ROUTE_PORTS, 6);
        assert_eq!(OPTICAL_LEVEL_HOLDERS, 6);
        assert_eq!(CAPACITIVE_PROBE_POCKETS, 4);
        assert_eq!(PRESSURE_PROBE_POCKETS, 4);
        assert_eq!(RH_DEWPOINT_LOGGER_NESTS, 4);
        assert_eq!(CONDENSATE_DIVERSION_LANES, 3);
        assert_eq!(CONDENSATE_WITNESS_CUPS, 5);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_LANDS, 4);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_MOUNTS, 5);
        assert_eq!(EVIDENCE_FIDUCIALS, 10);
    }

    #[test]
    fn major_feature_envelopes_stay_inside_contained_deck() {
        for rect in major_module_rects() {
            assert!(rect_fits_deck(rect, RIM_W));
        }
        assert!(SUMP_X < DECK_X - 2.0 * RIM_W);
        assert!(SUMP_Y < DECK_Y - 2.0 * RIM_W);
        assert_layout();
    }

    #[test]
    fn evaporation_coupon_grid_has_edge_and_center_positions() {
        let mut edge_count = 0;
        let mut center_count = 0;
        for index in 0..EVAPORATION_COUPONS {
            let (x, y) = coupon_center(index);
            assert!(x.abs() + COUPON_SLOT_X / 2.0 < COUPON_GRID_X / 2.0);
            assert!(y.abs() + COUPON_SLOT_Y / 2.0 < COUPON_GRID_Y / 2.0);
            if coupon_is_edge(index) {
                edge_count += 1;
            } else {
                center_count += 1;
            }
        }
        assert_eq!(edge_count, EDGE_EVAPORATION_COUPONS);
        assert_eq!(center_count, CENTER_EVAPORATION_COUPONS);
    }

    #[test]
    fn sensor_mass_and_keepout_geometry_have_expected_clearance() {
        assert!(LOAD_CELL_POCKET_X > MASS_PAN_D);
        assert!(MASS_ISOLATION_MOAT_X > LOAD_CELL_POCKET_X + 80.0);
        assert!(MASS_ISOLATION_MOAT_Y > LOAD_CELL_POCKET_Y + 70.0);
        assert!(MASS_READOUT_WINDOW_X < MASS_PAD_X / 3.0);
        assert!(CAPACITIVE_POCKET_D > PRESSURE_POCKET_D);
        assert!(OPTICAL_WINDOW_X > CAPACITIVE_POCKET_D);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(BRIDGE_UNDERSIDE_CLEARANCE > RIM_Z + DECK_Z);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
    }

    #[test]
    fn helper_geometry_is_centered_and_disposition_indices_are_explicit() {
        assert!((cassette_slot_x(0) + cassette_slot_x(CASSETTE_SURROGATE_SLOTS - 1)).abs() < 0.001);
        assert!((optical_holder_x(0) + optical_holder_x(OPTICAL_LEVEL_HOLDERS - 1)).abs() < 0.001);
        assert!((probe_x(0) + probe_x(CAPACITIVE_PROBE_POCKETS - 1)).abs() < 0.001);
        assert!((logger_x(0) + logger_x(RH_DEWPOINT_LOGGER_NESTS - 1)).abs() < 0.001);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert_eq!(GATE_TOKEN_SLOTS % DISPOSITION_GATES, 0);
    }
}
