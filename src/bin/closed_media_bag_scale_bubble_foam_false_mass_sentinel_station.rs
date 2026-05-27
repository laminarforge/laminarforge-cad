use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag scale false-mass sentinel station.
//
// Mechanical validation packaging for detecting false mass from bubbles, foam,
// and media-bag position drift before feeding scaled tissue-chip cassettes. The
// geometry models fixture datums, surrogate scale/load-cell features, witness
// routes, evidence/custody lands, and robot/service keepouts only; it is not a
// sterile-process claim, dosing SOP, certified weighing device, pressure-rated
// fluid path, or biological acceptance criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str =
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_containment_deck.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_load_cell_cradle_surrogate.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_bag_hang_settle_datum.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_bubble_foam_optical_window.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_tare_checkweight_rail.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_anti_slosh_baffle_witness_coupons.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_temperature_rh_logger_pockets.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_drain_prime_route_witness.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_barcode_custody_lands.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_release_hold_reject_gates.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_media_bag_scale_bubble_foam_false_mass_sentinel_station_assembly.stl",
];

const DESIGN_SCOPE: &str = "mechanical validation packaging only; not a sterile-process claim, dosing SOP, certified weighing device, pressure-rated fluid path, or biological acceptance criterion";

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 24] = [
    "load_cell_cradle_surrogate",
    "scale_pan_false_mass_reference",
    "load_cell_isolation_moat",
    "bag_hang_datum",
    "bag_settle_datum_forks",
    "bag_position_drift_scale_ticks",
    "bubble_foam_optical_window",
    "backlight_slot_ladder",
    "tare_checkweight_rail",
    "tare_token_slots",
    "anti_slosh_baffle_witness_coupons",
    "foam_baffle_witness_ticks",
    "temperature_logger_pockets",
    "rh_logger_pockets",
    "drain_route_witness",
    "prime_route_witness",
    "barcode_custody_lands",
    "lot_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "evidence_bridge",
    "robot_keepouts",
    "service_keepouts",
];

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 1000.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1360.0;
const SUMP_Y: f64 = 820.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 20.0;
const DATUM_BOSSES: usize = 10;

const CRADLE_X: f64 = 500.0;
const CRADLE_Y: f64 = 310.0;
const CRADLE_Z: f64 = 48.0;
const CRADLE_POS: (f64, f64) = (-380.0, 140.0);
const SCALE_PAN_D: f64 = 220.0;
const LOAD_CELL_COUNT: usize = 4;
const LOAD_CELL_POCKET_X: f64 = 116.0;
const LOAD_CELL_POCKET_Y: f64 = 42.0;
const LOAD_CELL_POCKET_Z: f64 = 18.0;
const ISOLATION_MOAT_W: f64 = 14.0;
const FALSE_MASS_REFERENCE_TABS: usize = 6;
const CRADLE_FLEXURE_RIBS: usize = 8;

const HANG_X: f64 = 420.0;
const HANG_Y: f64 = 240.0;
const HANG_Z: f64 = 42.0;
const HANG_POS: (f64, f64) = (-440.0, -185.0);
const HANG_POSTS: usize = 4;
const SETTLE_FORKS: usize = 6;
const BAG_DRIFT_TICKS: usize = 9;
const HANG_PIN_SPAN_X: f64 = 300.0;
const HANG_CLEARANCE_Z: f64 = 142.0;

const WINDOW_X: f64 = 590.0;
const WINDOW_Y: f64 = 180.0;
const WINDOW_Z: f64 = 34.0;
const WINDOW_POS: (f64, f64) = (285.0, 250.0);
const OPTICAL_WINDOWS: usize = 8;
const WINDOW_SLOT_X: f64 = 48.0;
const WINDOW_SLOT_Y: f64 = 86.0;
const WINDOW_PITCH_X: f64 = 62.0;
const FOAM_HEIGHT_TICKS: usize = 7;
const BACKLIGHT_SLOTS: usize = OPTICAL_WINDOWS;

const CHECK_RAIL_X: f64 = 540.0;
const CHECK_RAIL_Y: f64 = 120.0;
const CHECK_RAIL_Z: f64 = 38.0;
const CHECK_RAIL_POS: (f64, f64) = (425.0, 60.0);
const CHECKWEIGHT_NESTS: usize = 6;
const CHECKWEIGHT_D: f64 = 34.0;
const CHECKWEIGHT_PITCH_X: f64 = 70.0;
const TARE_TOKEN_SLOTS: usize = 6;
const TARE_SLOT_X: f64 = 44.0;

const BAFFLE_X: f64 = 560.0;
const BAFFLE_Y: f64 = 180.0;
const BAFFLE_Z: f64 = 34.0;
const BAFFLE_POS: (f64, f64) = (400.0, -140.0);
const BAFFLE_COUPONS: usize = 8;
const BAFFLE_COLS: usize = 4;
const BAFFLE_ROWS: usize = 2;
const BAFFLE_COUPON_X: f64 = 74.0;
const BAFFLE_COUPON_Y: f64 = 42.0;
const BAFFLE_COUPON_Z: f64 = 12.0;
const BAFFLE_PITCH_X: f64 = 98.0;
const BAFFLE_PITCH_Y: f64 = 74.0;
const BAFFLE_TICK_MARKS: usize = 24;

const LOGGER_X: f64 = 380.0;
const LOGGER_Y: f64 = 100.0;
const LOGGER_Z: f64 = 30.0;
const LOGGER_POS: (f64, f64) = (150.0, 395.0);
const TEMP_LOGGER_POCKETS: usize = 2;
const RH_LOGGER_POCKETS: usize = 2;
const LOGGER_POCKETS: usize = TEMP_LOGGER_POCKETS + RH_LOGGER_POCKETS;
const LOGGER_POCKET_X: f64 = 70.0;
const LOGGER_POCKET_Y: f64 = 42.0;
const LOGGER_PITCH_X: f64 = 82.0;
const LOGGER_SENSOR_PORTS: usize = 4;

const ROUTE_X: f64 = 500.0;
const ROUTE_Y: f64 = 130.0;
const ROUTE_Z: f64 = 36.0;
const ROUTE_POS: (f64, f64) = (-440.0, 390.0);
const DRAIN_ROUTE_CHANNELS: usize = 2;
const PRIME_ROUTE_CHANNELS: usize = 2;
const ROUTE_WITNESS_PORTS: usize = 8;
const ROUTE_PORT_D: f64 = 14.0;
const ROUTE_PORT_PITCH_X: f64 = 56.0;

const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 130.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (455.0, -375.0);
const BARCODE_LANDS: usize = 8;
const LOT_CUSTODY_LANDS: usize = 4;
const TAMPER_SEAL_TABS: usize = 4;

const GATE_X: f64 = 420.0;
const GATE_Y: f64 = 126.0;
const GATE_Z: f64 = 36.0;
const GATE_POS: (f64, f64) = (-440.0, -375.0);
const DISPOSITION_GATES: usize = 3;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;
const GATE_PITCH_X: f64 = 112.0;
const GATE_TOKEN_SLOTS: usize = 6;

const BRIDGE_X: f64 = 1030.0;
const BRIDGE_Y: f64 = 46.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const BRIDGE_CLEARANCE_Z: f64 = 178.0;
const BRIDGE_POS: (f64, f64) = (20.0, 8.0);
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 64.0;
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const ROBOT_KEEPOUT_X: f64 = 1380.0;
const ROBOT_KEEPOUT_Y: f64 = 86.0;
const ROBOT_KEEPOUT_Z: f64 = 72.0;
const SERVICE_KEEPOUT_X: f64 = 102.0;
const SERVICE_KEEPOUT_Y: f64 = 770.0;
const SERVICE_KEEPOUT_Z: f64 = 92.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 318.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(load_cell_cradle_surrogate(), OUTPUTS[1]);
    write_part(bag_hang_settle_datum(), OUTPUTS[2]);
    write_part(bubble_foam_optical_window(), OUTPUTS[3]);
    write_part(tare_checkweight_rail(), OUTPUTS[4]);
    write_part(anti_slosh_baffle_witness_coupons(), OUTPUTS[5]);
    write_part(temperature_rh_logger_pockets(), OUTPUTS[6]);
    write_part(drain_prime_route_witness(), OUTPUTS[7]);
    write_part(barcode_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(evidence_bridge_robot_service_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed media-bag false-mass sentinel: {:.0}mm x {:.0}mm contained deck, {} load-cell surrogate pockets, {} bag settle forks, {} optical bubble/foam windows.",
        DECK_X, DECK_Y, LOAD_CELL_COUNT, SETTLE_FORKS, OPTICAL_WINDOWS
    );
    println!(
        "False-mass controls: {} checkweight nests, {} tare token slots, {} anti-slosh baffle coupons, {} drain/prime witness channels.",
        CHECKWEIGHT_NESTS,
        TARE_TOKEN_SLOTS,
        BAFFLE_COUPONS,
        DRAIN_ROUTE_CHANNELS + PRIME_ROUTE_CHANNELS
    );
    println!(
        "Evidence and custody: {} barcode lands, {} lot lands, {} logger pockets, {} release/hold/reject gates, top service clearance {:.0}mm.",
        BARCODE_LANDS,
        LOT_CUSTODY_LANDS,
        LOGGER_POCKETS,
        DISPOSITION_GATES,
        TOP_SERVICE_CLEARANCE_Z
    );
    println!("Scope: {DESIGN_SCOPE}.");
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + load_cell_cradle_surrogate()
        + bag_hang_settle_datum()
        + bubble_foam_optical_window()
        + tare_checkweight_rail()
        + anti_slosh_baffle_witness_coupons()
        + temperature_rh_logger_pockets()
        + drain_prime_route_witness()
        + barcode_custody_lands()
        + release_hold_reject_gates()
        + evidence_bridge_robot_service_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "media_bag_false_mass_sentinel_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "media_bag_false_mass_sentinel_shallow_sump_relief",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "media_bag_false_mass_sentinel_deck_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 110.0, -DECK_Y / 2.0 + 12.0, 0.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pads()
}

fn containment_rim() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    centered_cube(
        "media_bag_false_mass_sentinel_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            "media_bag_false_mass_sentinel_rear_containment_rim",
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            "media_bag_false_mass_sentinel_left_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            "media_bag_false_mass_sentinel_right_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("media_bag_false_mass_sentinel_deck_datums");
    for idx in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 78.0 + idx as f64 * ((DECK_X - 156.0) / 9.0);
        let y = if idx % 2 == 0 {
            DECK_Y / 2.0 - 72.0
        } else {
            -DECK_Y / 2.0 + 72.0
        };
        let boss = centered_cylinder(
            format!("media_bag_false_mass_sentinel_datum_boss_{idx}"),
            8.0,
            6.0,
            28,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("media_bag_false_mass_sentinel_datum_bore_{idx}"),
            2.2,
            8.0,
            20,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        datums = datums + boss - bore;
    }
    datums
}

fn station_landing_pads() -> Part {
    landing_pad("load_cell_cradle_land", CRADLE_X, CRADLE_Y, CRADLE_POS)
        + landing_pad("bag_hang_settle_land", HANG_X, HANG_Y, HANG_POS)
        + landing_pad("bubble_foam_window_land", WINDOW_X, WINDOW_Y, WINDOW_POS)
        + landing_pad(
            "tare_checkweight_rail_land",
            CHECK_RAIL_X,
            CHECK_RAIL_Y,
            CHECK_RAIL_POS,
        )
        + landing_pad("anti_slosh_coupon_land", BAFFLE_X, BAFFLE_Y, BAFFLE_POS)
        + landing_pad("temperature_rh_logger_land", LOGGER_X, LOGGER_Y, LOGGER_POS)
        + landing_pad("drain_prime_route_land", ROUTE_X, ROUTE_Y, ROUTE_POS)
        + landing_pad("barcode_custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pad("release_hold_reject_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pad(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("media_bag_false_mass_sentinel_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn load_cell_cradle_surrogate() -> Part {
    let base = centered_cube(
        "media_bag_false_mass_sentinel_load_cell_cradle_surrogate_base",
        CRADLE_X,
        CRADLE_Y,
        CRADLE_Z,
    )
    .translate(CRADLE_POS.0, CRADLE_POS.1, top_z(CRADLE_Z));
    let pan_recess = centered_cylinder(
        "media_bag_false_mass_sentinel_scale_pan_false_mass_reference_recess",
        SCALE_PAN_D / 2.0,
        CRADLE_Z + 2.0,
        80,
    )
    .translate(CRADLE_POS.0, CRADLE_POS.1, top_z(CRADLE_Z));
    let isolation_x = centered_cube(
        "media_bag_false_mass_sentinel_load_cell_isolation_moat_x",
        CRADLE_X - 76.0,
        ISOLATION_MOAT_W,
        CRADLE_Z + 2.0,
    )
    .translate(CRADLE_POS.0, CRADLE_POS.1, top_z(CRADLE_Z));
    let isolation_y = centered_cube(
        "media_bag_false_mass_sentinel_load_cell_isolation_moat_y",
        ISOLATION_MOAT_W,
        CRADLE_Y - 68.0,
        CRADLE_Z + 2.0,
    )
    .translate(CRADLE_POS.0, CRADLE_POS.1, top_z(CRADLE_Z));

    let mut pockets = Part::empty("media_bag_false_mass_sentinel_load_cell_pocket_cuts");
    for idx in 0..LOAD_CELL_COUNT {
        let (x, y) = load_cell_center(idx);
        pockets = pockets
            + centered_cube(
                format!("media_bag_false_mass_sentinel_load_cell_pocket_{idx}"),
                LOAD_CELL_POCKET_X,
                LOAD_CELL_POCKET_Y,
                LOAD_CELL_POCKET_Z,
            )
            .translate(CRADLE_POS.0 + x, CRADLE_POS.1 + y, top_z(CRADLE_Z));
    }

    base - pan_recess - isolation_x - isolation_y - pockets
        + false_mass_reference_tabs()
        + cradle_flexure_ribs()
        + scale_cable_witness_trough()
}

fn false_mass_reference_tabs() -> Part {
    let mut tabs = Part::empty("media_bag_false_mass_sentinel_false_mass_reference_tabs");
    for idx in 0..FALSE_MASS_REFERENCE_TABS {
        let angle_step = std::f64::consts::TAU / FALSE_MASS_REFERENCE_TABS as f64;
        let theta = idx as f64 * angle_step;
        let x = CRADLE_POS.0 + theta.cos() * (SCALE_PAN_D / 2.0 + 32.0);
        let y = CRADLE_POS.1 + theta.sin() * (SCALE_PAN_D / 2.0 + 32.0);
        tabs = tabs
            + centered_cube(
                format!("media_bag_false_mass_sentinel_scale_pan_reference_tab_{idx}"),
                44.0,
                12.0,
                6.0,
            )
            .rotate(0.0, 0.0, theta.to_degrees())
            .translate(x, y, DECK_Z / 2.0 + CRADLE_Z + 3.0);
    }
    tabs
}

fn cradle_flexure_ribs() -> Part {
    let mut ribs = Part::empty("media_bag_false_mass_sentinel_cradle_flexure_ribs");
    for idx in 0..CRADLE_FLEXURE_RIBS {
        let x = CRADLE_POS.0 - 210.0 + idx as f64 * 60.0;
        let y = if idx % 2 == 0 {
            CRADLE_POS.1 - CRADLE_Y / 2.0 + 32.0
        } else {
            CRADLE_POS.1 + CRADLE_Y / 2.0 - 32.0
        };
        ribs = ribs
            + centered_cube(
                format!("media_bag_false_mass_sentinel_load_cell_flexure_rib_{idx}"),
                34.0,
                8.0,
                12.0,
            )
            .translate(x, y, DECK_Z / 2.0 + CRADLE_Z + 6.0);
    }
    ribs
}

fn scale_cable_witness_trough() -> Part {
    centered_cube(
        "media_bag_false_mass_sentinel_scale_cable_witness_trough",
        180.0,
        18.0,
        8.0,
    )
    .translate(
        CRADLE_POS.0 + CRADLE_X / 2.0 - 88.0,
        CRADLE_POS.1 - CRADLE_Y / 2.0 + 24.0,
        DECK_Z / 2.0 + CRADLE_Z + 4.0,
    )
}

fn bag_hang_settle_datum() -> Part {
    let base = centered_cube(
        "media_bag_false_mass_sentinel_bag_hang_settle_datum_plate",
        HANG_X,
        HANG_Y,
        HANG_Z,
    )
    .translate(HANG_POS.0, HANG_POS.1, top_z(HANG_Z));
    let settle_slot = centered_cube(
        "media_bag_false_mass_sentinel_bag_settle_shadow_slot",
        HANG_X - 92.0,
        42.0,
        HANG_Z + 2.0,
    )
    .translate(HANG_POS.0, HANG_POS.1 + 22.0, top_z(HANG_Z));

    base - settle_slot + hang_posts_and_crossbar() + settle_datum_forks() + bag_drift_scale_ticks()
}

fn hang_posts_and_crossbar() -> Part {
    let mut posts = Part::empty("media_bag_false_mass_sentinel_bag_hang_datum_posts");
    for idx in 0..HANG_POSTS {
        let x = if idx % 2 == 0 {
            HANG_POS.0 - HANG_PIN_SPAN_X / 2.0
        } else {
            HANG_POS.0 + HANG_PIN_SPAN_X / 2.0
        };
        let y = if idx < 2 {
            HANG_POS.1 - 70.0
        } else {
            HANG_POS.1 + 70.0
        };
        posts = posts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_bag_hang_post_{idx}"),
                28.0,
                28.0,
                HANG_CLEARANCE_Z,
            )
            .translate(x, y, DECK_Z / 2.0 + HANG_CLEARANCE_Z / 2.0);
    }
    let front_bar = centered_cube(
        "media_bag_false_mass_sentinel_bag_hang_front_crossbar",
        HANG_PIN_SPAN_X + 56.0,
        18.0,
        18.0,
    )
    .translate(
        HANG_POS.0,
        HANG_POS.1 - 70.0,
        DECK_Z / 2.0 + HANG_CLEARANCE_Z - 8.0,
    );
    let rear_bar = centered_cube(
        "media_bag_false_mass_sentinel_bag_hang_rear_crossbar",
        HANG_PIN_SPAN_X + 56.0,
        18.0,
        18.0,
    )
    .translate(
        HANG_POS.0,
        HANG_POS.1 + 70.0,
        DECK_Z / 2.0 + HANG_CLEARANCE_Z - 8.0,
    );

    posts + front_bar + rear_bar
}

fn settle_datum_forks() -> Part {
    let mut forks = Part::empty("media_bag_false_mass_sentinel_bag_settle_datum_forks");
    for idx in 0..SETTLE_FORKS {
        let x = HANG_POS.0 + settle_fork_x(idx);
        forks = forks
            + centered_cube(
                format!("media_bag_false_mass_sentinel_bag_settle_fork_left_{idx}"),
                8.0,
                56.0,
                28.0,
            )
            .translate(x - 15.0, HANG_POS.1 + 12.0, DECK_Z / 2.0 + HANG_Z + 14.0)
            + centered_cube(
                format!("media_bag_false_mass_sentinel_bag_settle_fork_right_{idx}"),
                8.0,
                56.0,
                28.0,
            )
            .translate(x + 15.0, HANG_POS.1 + 12.0, DECK_Z / 2.0 + HANG_Z + 14.0);
    }
    forks
}

fn bag_drift_scale_ticks() -> Part {
    let mut ticks = Part::empty("media_bag_false_mass_sentinel_bag_position_drift_scale_ticks");
    for idx in 0..BAG_DRIFT_TICKS {
        let x = HANG_POS.0 - 160.0 + idx as f64 * 40.0;
        let z = 5.0 + (idx % 3) as f64 * 4.0;
        ticks = ticks
            + centered_cube(
                format!("media_bag_false_mass_sentinel_bag_position_drift_tick_{idx}"),
                4.0,
                30.0,
                z,
            )
            .translate(
                x,
                HANG_POS.1 - HANG_Y / 2.0 + 28.0,
                DECK_Z / 2.0 + HANG_Z + z / 2.0,
            );
    }
    ticks
}

fn bubble_foam_optical_window() -> Part {
    let frame = centered_cube(
        "media_bag_false_mass_sentinel_bubble_foam_optical_window_frame",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(WINDOW_POS.0, WINDOW_POS.1, top_z(WINDOW_Z));

    let mut window_cuts = Part::empty("media_bag_false_mass_sentinel_optical_window_cuts");
    let mut witness = Part::empty("media_bag_false_mass_sentinel_bubble_foam_window_witnesses");
    for idx in 0..OPTICAL_WINDOWS {
        let x = WINDOW_POS.0 + optical_window_x(idx);
        window_cuts = window_cuts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_bubble_foam_window_slot_{idx}"),
                WINDOW_SLOT_X,
                WINDOW_SLOT_Y,
                WINDOW_Z + 2.0,
            )
            .translate(x, WINDOW_POS.1, top_z(WINDOW_Z));
        witness = witness
            + centered_cube(
                format!("media_bag_false_mass_sentinel_backlight_slot_ladder_{idx}"),
                WINDOW_SLOT_X - 10.0,
                8.0,
                5.0,
            )
            .translate(
                x,
                WINDOW_POS.1 - WINDOW_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + WINDOW_Z + 2.5,
            );
    }
    for idx in 0..FOAM_HEIGHT_TICKS {
        witness = witness
            + centered_cube(
                format!("media_bag_false_mass_sentinel_foam_height_tick_{idx}"),
                18.0 + idx as f64 * 4.0,
                4.0,
                6.0,
            )
            .translate(
                WINDOW_POS.0 + WINDOW_X / 2.0 - 46.0,
                WINDOW_POS.1 - 60.0 + idx as f64 * 20.0,
                DECK_Z / 2.0 + WINDOW_Z + 3.0,
            );
    }

    frame - window_cuts + witness + optical_window_side_rails()
}

fn optical_window_side_rails() -> Part {
    let left = centered_cube(
        "media_bag_false_mass_sentinel_optical_window_left_tube_reference_rail",
        16.0,
        WINDOW_Y + 22.0,
        18.0,
    )
    .translate(
        WINDOW_POS.0 - WINDOW_X / 2.0 + 26.0,
        WINDOW_POS.1,
        DECK_Z / 2.0 + WINDOW_Z + 9.0,
    );
    let right = centered_cube(
        "media_bag_false_mass_sentinel_optical_window_right_tube_reference_rail",
        16.0,
        WINDOW_Y + 22.0,
        18.0,
    )
    .translate(
        WINDOW_POS.0 + WINDOW_X / 2.0 - 26.0,
        WINDOW_POS.1,
        DECK_Z / 2.0 + WINDOW_Z + 9.0,
    );
    left + right
}

fn tare_checkweight_rail() -> Part {
    let rail = centered_cube(
        "media_bag_false_mass_sentinel_tare_checkweight_rail",
        CHECK_RAIL_X,
        CHECK_RAIL_Y,
        CHECK_RAIL_Z,
    )
    .translate(CHECK_RAIL_POS.0, CHECK_RAIL_POS.1, top_z(CHECK_RAIL_Z));

    let mut cuts = Part::empty("media_bag_false_mass_sentinel_checkweight_and_tare_cuts");
    let mut collars = Part::empty("media_bag_false_mass_sentinel_checkweight_reference_collars");
    for idx in 0..CHECKWEIGHT_NESTS {
        let x = CHECK_RAIL_POS.0 + checkweight_x(idx);
        cuts = cuts
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_checkweight_nest_{idx}"),
                CHECKWEIGHT_D / 2.0,
                CHECK_RAIL_Z + 2.0,
                40,
            )
            .translate(x, CHECK_RAIL_POS.1 + 22.0, top_z(CHECK_RAIL_Z));
        collars = collars
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_checkweight_reference_collar_{idx}"),
                CHECKWEIGHT_D / 2.0 + 6.0,
                5.0,
                40,
            )
            .translate(
                x,
                CHECK_RAIL_POS.1 + 22.0,
                DECK_Z / 2.0 + CHECK_RAIL_Z + 2.5,
            );
        cuts = cuts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_tare_token_slot_{idx}"),
                TARE_SLOT_X,
                16.0,
                CHECK_RAIL_Z + 2.0,
            )
            .translate(x, CHECK_RAIL_POS.1 - 32.0, top_z(CHECK_RAIL_Z));
    }

    rail - cuts + collars + checkweight_end_stops()
}

fn checkweight_end_stops() -> Part {
    centered_cube(
        "media_bag_false_mass_sentinel_checkweight_left_end_stop",
        14.0,
        CHECK_RAIL_Y,
        32.0,
    )
    .translate(
        CHECK_RAIL_POS.0 - CHECK_RAIL_X / 2.0 + 16.0,
        CHECK_RAIL_POS.1,
        DECK_Z / 2.0 + CHECK_RAIL_Z + 16.0,
    ) + centered_cube(
        "media_bag_false_mass_sentinel_checkweight_right_end_stop",
        14.0,
        CHECK_RAIL_Y,
        32.0,
    )
    .translate(
        CHECK_RAIL_POS.0 + CHECK_RAIL_X / 2.0 - 16.0,
        CHECK_RAIL_POS.1,
        DECK_Z / 2.0 + CHECK_RAIL_Z + 16.0,
    )
}

fn anti_slosh_baffle_witness_coupons() -> Part {
    let plate = centered_cube(
        "media_bag_false_mass_sentinel_anti_slosh_baffle_witness_coupon_plate",
        BAFFLE_X,
        BAFFLE_Y,
        BAFFLE_Z,
    )
    .translate(BAFFLE_POS.0, BAFFLE_POS.1, top_z(BAFFLE_Z));

    let mut pockets = Part::empty("media_bag_false_mass_sentinel_baffle_coupon_recesses");
    let mut coupons = Part::empty("media_bag_false_mass_sentinel_baffle_witness_coupons");
    for idx in 0..BAFFLE_COUPONS {
        let (x, y) = baffle_coupon_center(idx);
        let world_x = BAFFLE_POS.0 + x;
        let world_y = BAFFLE_POS.1 + y;
        pockets = pockets
            + centered_cube(
                format!("media_bag_false_mass_sentinel_baffle_coupon_recess_{idx}"),
                BAFFLE_COUPON_X + 8.0,
                BAFFLE_COUPON_Y + 8.0,
                BAFFLE_Z + 2.0,
            )
            .translate(world_x, world_y, top_z(BAFFLE_Z));
        coupons = coupons
            + centered_cube(
                format!("media_bag_false_mass_sentinel_anti_slosh_baffle_coupon_{idx}"),
                BAFFLE_COUPON_X,
                BAFFLE_COUPON_Y,
                BAFFLE_COUPON_Z,
            )
            .translate(
                world_x,
                world_y,
                DECK_Z / 2.0 + BAFFLE_Z + BAFFLE_COUPON_Z / 2.0,
            )
            + centered_cube(
                format!("media_bag_false_mass_sentinel_foam_baffle_witness_fin_{idx}"),
                8.0,
                BAFFLE_COUPON_Y,
                34.0,
            )
            .translate(world_x, world_y, DECK_Z / 2.0 + BAFFLE_Z + 17.0);
    }

    plate - pockets + coupons + baffle_tick_marks()
}

fn baffle_tick_marks() -> Part {
    let mut ticks = Part::empty("media_bag_false_mass_sentinel_foam_baffle_witness_ticks");
    for idx in 0..BAFFLE_TICK_MARKS {
        let col = idx % 12;
        let row = idx / 12;
        ticks = ticks
            + centered_cube(
                format!("media_bag_false_mass_sentinel_baffle_witness_tick_{idx}"),
                4.0,
                14.0,
                5.0,
            )
            .translate(
                BAFFLE_POS.0 - 245.0 + col as f64 * 44.0,
                BAFFLE_POS.1 - 76.0 + row as f64 * 152.0,
                DECK_Z / 2.0 + BAFFLE_Z + 2.5,
            );
    }
    ticks
}

fn temperature_rh_logger_pockets() -> Part {
    let plate = centered_cube(
        "media_bag_false_mass_sentinel_temperature_rh_logger_pocket_plate",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(LOGGER_POS.0, LOGGER_POS.1, top_z(LOGGER_Z));

    let mut pockets = Part::empty("media_bag_false_mass_sentinel_logger_pocket_cuts");
    let mut labels = Part::empty("media_bag_false_mass_sentinel_logger_pocket_labels");
    for idx in 0..LOGGER_POCKETS {
        let x = LOGGER_POS.0 + logger_x(idx);
        pockets = pockets
            + centered_cube(
                format!(
                    "media_bag_false_mass_sentinel_{}_logger_pocket_{idx}",
                    logger_kind(idx)
                ),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_Z + 2.0,
            )
            .translate(x, LOGGER_POS.1, top_z(LOGGER_Z));
        labels = labels
            + centered_cube(
                format!(
                    "media_bag_false_mass_sentinel_{}_logger_label_land_{idx}",
                    logger_kind(idx)
                ),
                LOGGER_POCKET_X - 12.0,
                7.0,
                4.0,
            )
            .translate(x, LOGGER_POS.1 - 34.0, DECK_Z / 2.0 + LOGGER_Z + 2.0);
    }

    plate - pockets + labels + logger_sensor_ports()
}

fn logger_sensor_ports() -> Part {
    let mut ports = Part::empty("media_bag_false_mass_sentinel_logger_sensor_ports");
    for idx in 0..LOGGER_SENSOR_PORTS {
        let x = LOGGER_POS.0 + logger_x(idx);
        ports = ports
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_logger_sensor_port_{idx}"),
                7.0,
                6.0,
                28,
            )
            .translate(x, LOGGER_POS.1 + 36.0, DECK_Z / 2.0 + LOGGER_Z + 3.0);
    }
    ports
}

fn drain_prime_route_witness() -> Part {
    let plate = centered_cube(
        "media_bag_false_mass_sentinel_drain_prime_route_witness_plate",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    )
    .translate(ROUTE_POS.0, ROUTE_POS.1, top_z(ROUTE_Z));

    let mut cuts = Part::empty("media_bag_false_mass_sentinel_drain_prime_route_cuts");
    let mut collars = Part::empty("media_bag_false_mass_sentinel_route_witness_port_collars");
    for idx in 0..ROUTE_WITNESS_PORTS {
        let x = ROUTE_POS.0 + route_port_x(idx);
        cuts = cuts
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_route_witness_port_{idx}"),
                ROUTE_PORT_D / 2.0,
                ROUTE_Z + 2.0,
                32,
            )
            .translate(x, ROUTE_POS.1, top_z(ROUTE_Z));
        collars = collars
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_route_witness_port_collar_{idx}"),
                ROUTE_PORT_D / 2.0 + 5.0,
                5.0,
                32,
            )
            .translate(x, ROUTE_POS.1, DECK_Z / 2.0 + ROUTE_Z + 2.5);
    }
    for lane in 0..DRAIN_ROUTE_CHANNELS {
        cuts = cuts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_drain_route_witness_channel_{lane}"),
                ROUTE_X - 82.0,
                8.0,
                ROUTE_Z + 2.0,
            )
            .translate(
                ROUTE_POS.0,
                ROUTE_POS.1 - 42.0 + lane as f64 * 18.0,
                top_z(ROUTE_Z),
            );
    }
    for lane in 0..PRIME_ROUTE_CHANNELS {
        cuts = cuts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_prime_route_witness_channel_{lane}"),
                ROUTE_X - 82.0,
                8.0,
                ROUTE_Z + 2.0,
            )
            .translate(
                ROUTE_POS.0,
                ROUTE_POS.1 + 24.0 + lane as f64 * 18.0,
                top_z(ROUTE_Z),
            );
    }

    plate - cuts + collars + drain_prime_direction_arrows()
}

fn drain_prime_direction_arrows() -> Part {
    let drain_arrow = centered_cube(
        "media_bag_false_mass_sentinel_drain_route_direction_arrow",
        58.0,
        12.0,
        5.0,
    )
    .translate(
        ROUTE_POS.0 - ROUTE_X / 2.0 + 70.0,
        ROUTE_POS.1 - 52.0,
        DECK_Z / 2.0 + ROUTE_Z + 2.5,
    );
    let prime_arrow = centered_cube(
        "media_bag_false_mass_sentinel_prime_route_direction_arrow",
        58.0,
        12.0,
        5.0,
    )
    .translate(
        ROUTE_POS.0 + ROUTE_X / 2.0 - 70.0,
        ROUTE_POS.1 + 52.0,
        DECK_Z / 2.0 + ROUTE_Z + 2.5,
    );
    drain_arrow + prime_arrow
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "media_bag_false_mass_sentinel_barcode_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z));

    let mut lands = Part::empty("media_bag_false_mass_sentinel_custody_lands");
    for idx in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("media_bag_false_mass_sentinel_barcode_custody_land_{idx}"),
                46.0,
                22.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 164.0 + idx as f64 * 47.0,
                CUSTODY_POS.1 + 34.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    for idx in 0..LOT_CUSTODY_LANDS {
        lands = lands
            + centered_cube(
                format!("media_bag_false_mass_sentinel_lot_custody_land_{idx}"),
                78.0,
                24.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 126.0 + idx as f64 * 84.0,
                CUSTODY_POS.1 - 28.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    for idx in 0..TAMPER_SEAL_TABS {
        lands = lands
            + centered_cube(
                format!("media_bag_false_mass_sentinel_tamper_seal_tab_{idx}"),
                18.0,
                32.0,
                6.0,
            )
            .translate(
                CUSTODY_POS.0 - CUSTODY_X / 2.0 + 36.0 + idx as f64 * 118.0,
                CUSTODY_POS.1,
                DECK_Z / 2.0 + CUSTODY_Z + 3.0,
            );
    }

    panel + lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "media_bag_false_mass_sentinel_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z));

    let mut gate_cuts = Part::empty("media_bag_false_mass_sentinel_disposition_gate_cuts");
    let mut paddles = Part::empty("media_bag_false_mass_sentinel_disposition_gate_paddles");
    for idx in 0..DISPOSITION_GATES {
        let x = GATE_POS.0 + gate_x(idx);
        gate_cuts = gate_cuts
            + centered_cube(
                format!(
                    "media_bag_false_mass_sentinel_{}_gate_trough",
                    disposition_gate_name(idx)
                ),
                82.0,
                62.0,
                GATE_Z + 2.0,
            )
            .translate(x, GATE_POS.1, top_z(GATE_Z));
        paddles = paddles
            + centered_cube(
                format!(
                    "media_bag_false_mass_sentinel_{}_gate_paddle",
                    disposition_gate_name(idx)
                ),
                10.0,
                92.0,
                44.0,
            )
            .translate(x + 42.0, GATE_POS.1, DECK_Z / 2.0 + GATE_Z + 22.0);
    }
    for idx in 0..GATE_TOKEN_SLOTS {
        let x = GATE_POS.0 - 165.0 + idx as f64 * 66.0;
        gate_cuts = gate_cuts
            + centered_cube(
                format!("media_bag_false_mass_sentinel_disposition_token_slot_{idx}"),
                38.0,
                14.0,
                GATE_Z + 2.0,
            )
            .translate(x, GATE_POS.1 - GATE_Y / 2.0 + 18.0, top_z(GATE_Z));
    }

    base - gate_cuts + paddles
}

fn evidence_bridge_robot_service_keepouts() -> Part {
    evidence_bridge() + robot_service_keepouts()
}

fn evidence_bridge() -> Part {
    let post_z = BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z;
    let left_post = centered_cube(
        "media_bag_false_mass_sentinel_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z / 2.0 + post_z / 2.0,
    );
    let right_post = centered_cube(
        "media_bag_false_mass_sentinel_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z / 2.0 + post_z / 2.0,
    );
    let beam = centered_cube(
        "media_bag_false_mass_sentinel_evidence_bridge_beam",
        BRIDGE_X + BRIDGE_POST_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        DECK_Z / 2.0 + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_mount_tabs() + evidence_fiducials()
}

fn camera_mount_tabs() -> Part {
    let mut tabs = Part::empty("media_bag_false_mass_sentinel_camera_mount_tabs");
    for idx in 0..CAMERA_MOUNTS {
        tabs = tabs
            + centered_cube(
                format!("media_bag_false_mass_sentinel_camera_mount_tab_{idx}"),
                64.0,
                28.0,
                8.0,
            )
            .translate(
                BRIDGE_POS.0 - 380.0 + idx as f64 * 190.0,
                BRIDGE_POS.1,
                DECK_Z / 2.0 + BRIDGE_CLEARANCE_Z - 4.0,
            );
    }
    tabs
}

fn evidence_fiducials() -> Part {
    let positions = [
        (-680.0, -438.0),
        (-505.0, -438.0),
        (-680.0, 438.0),
        (-505.0, 438.0),
        (-78.0, -438.0),
        (78.0, 438.0),
        (505.0, -438.0),
        (680.0, -438.0),
        (505.0, 438.0),
        (680.0, 438.0),
    ];
    assert_eq!(positions.len(), EVIDENCE_FIDUCIALS);
    let mut fiducials = Part::empty("media_bag_false_mass_sentinel_evidence_fiducials");
    for (idx, (x, y)) in positions.iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("media_bag_false_mass_sentinel_evidence_fiducial_{idx}"),
                7.0,
                4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 2.0);
    }
    fiducials
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "media_bag_false_mass_sentinel_front_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 100.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let rear_robot = centered_cube(
        "media_bag_false_mass_sentinel_rear_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 100.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "media_bag_false_mass_sentinel_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 88.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "media_bag_false_mass_sentinel_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 - 88.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "media_bag_false_mass_sentinel_top_service_clearance_gauge",
        DECK_X - 160.0,
        DECK_Y - 170.0,
        28.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + rear_robot + left_service + right_service + top_clearance
}

fn load_cell_center(index: usize) -> (f64, f64) {
    let x = if index % 2 == 0 { -142.0 } else { 142.0 };
    let y = if index < 2 { -86.0 } else { 86.0 };
    (x, y)
}

fn settle_fork_x(index: usize) -> f64 {
    (index as f64 - (SETTLE_FORKS as f64 - 1.0) / 2.0) * 54.0
}

fn optical_window_x(index: usize) -> f64 {
    (index as f64 - (OPTICAL_WINDOWS as f64 - 1.0) / 2.0) * WINDOW_PITCH_X
}

fn checkweight_x(index: usize) -> f64 {
    (index as f64 - (CHECKWEIGHT_NESTS as f64 - 1.0) / 2.0) * CHECKWEIGHT_PITCH_X
}

fn baffle_coupon_center(index: usize) -> (f64, f64) {
    let col = index % BAFFLE_COLS;
    let row = index / BAFFLE_COLS;
    grid_center(
        col,
        row,
        BAFFLE_COLS,
        BAFFLE_ROWS,
        BAFFLE_PITCH_X,
        BAFFLE_PITCH_Y,
    )
}

fn logger_x(index: usize) -> f64 {
    (index as f64 - (LOGGER_POCKETS as f64 - 1.0) / 2.0) * LOGGER_PITCH_X
}

fn logger_kind(index: usize) -> &'static str {
    if index < TEMP_LOGGER_POCKETS {
        "temperature"
    } else {
        "rh"
    }
}

fn route_port_x(index: usize) -> f64 {
    (index as f64 - (ROUTE_WITNESS_PORTS as f64 - 1.0) / 2.0) * ROUTE_PORT_PITCH_X
}

fn gate_x(index: usize) -> f64 {
    (index as f64 - (DISPOSITION_GATES as f64 - 1.0) / 2.0) * GATE_PITCH_X
}

fn grid_center(
    col: usize,
    row: usize,
    cols: usize,
    rows: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    (
        (col as f64 - (cols as f64 - 1.0) / 2.0) * pitch_x,
        (row as f64 - (rows as f64 - 1.0) / 2.0) * pitch_y,
    )
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn module_fits(pos: (f64, f64), x: f64, y: f64) -> bool {
    let inner_x = DECK_X / 2.0 - RIM_W - 8.0;
    let inner_y = DECK_Y / 2.0 - RIM_W - 8.0;
    pos.0.abs() + x / 2.0 <= inner_x && pos.1.abs() + y / 2.0 <= inner_y
}

fn all_modules_fit_inner_deck() -> bool {
    module_fits(CRADLE_POS, CRADLE_X, CRADLE_Y)
        && module_fits(HANG_POS, HANG_X, HANG_Y)
        && module_fits(WINDOW_POS, WINDOW_X, WINDOW_Y)
        && module_fits(CHECK_RAIL_POS, CHECK_RAIL_X, CHECK_RAIL_Y)
        && module_fits(BAFFLE_POS, BAFFLE_X, BAFFLE_Y)
        && module_fits(LOGGER_POS, LOGGER_X, LOGGER_Y)
        && module_fits(ROUTE_POS, ROUTE_X, ROUTE_Y)
        && module_fits(CUSTODY_POS, CUSTODY_X, CUSTODY_Y)
        && module_fits(GATE_POS, GATE_X, GATE_Y)
}

fn optical_window_span_x() -> f64 {
    (OPTICAL_WINDOWS as f64 - 1.0) * WINDOW_PITCH_X + WINDOW_SLOT_X
}

fn checkweight_span_x() -> f64 {
    (CHECKWEIGHT_NESTS as f64 - 1.0) * CHECKWEIGHT_PITCH_X + CHECKWEIGHT_D
}

fn baffle_coupon_span_x() -> f64 {
    (BAFFLE_COLS as f64 - 1.0) * BAFFLE_PITCH_X + BAFFLE_COUPON_X
}

fn baffle_coupon_span_y() -> f64 {
    (BAFFLE_ROWS as f64 - 1.0) * BAFFLE_PITCH_Y + BAFFLE_COUPON_Y
}

fn route_port_span_x() -> f64 {
    (ROUTE_WITNESS_PORTS as f64 - 1.0) * ROUTE_PORT_PITCH_X + ROUTE_PORT_D
}

fn logger_span_x() -> f64 {
    (LOGGER_POCKETS as f64 - 1.0) * LOGGER_PITCH_X + LOGGER_POCKET_X
}

fn bridge_clearance_over_modules() -> f64 {
    BRIDGE_CLEARANCE_Z
        - CRADLE_Z
            .max(HANG_Z)
            .max(WINDOW_Z)
            .max(CHECK_RAIL_Z)
            .max(BAFFLE_Z)
            .max(ROUTE_Z)
            .max(GATE_Z)
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected STL output count");
    assert_eq!(LOAD_CELL_COUNT, 4);
    assert_eq!(BACKLIGHT_SLOTS, OPTICAL_WINDOWS);
    assert_eq!(BAFFLE_COUPONS, BAFFLE_COLS * BAFFLE_ROWS);
    assert_eq!(LOGGER_POCKETS, TEMP_LOGGER_POCKETS + RH_LOGGER_POCKETS);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert!(SCALE_PAN_D > 2.0 * CHECKWEIGHT_D);
    assert!(optical_window_span_x() < WINDOW_X - 70.0);
    assert!(checkweight_span_x() < CHECK_RAIL_X - 92.0);
    assert!(baffle_coupon_span_x() < BAFFLE_X - 100.0);
    assert!(baffle_coupon_span_y() < BAFFLE_Y - 40.0);
    assert!(route_port_span_x() < ROUTE_X - 74.0);
    assert!(logger_span_x() < LOGGER_X - 48.0);
    assert!(bridge_clearance_over_modules() >= 120.0);
    assert!(all_modules_fit_inner_deck());
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn requested_feature_coverage_is_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 24);
        for feature in [
            "load_cell_cradle_surrogate",
            "bag_hang_datum",
            "bag_position_drift_scale_ticks",
            "bubble_foam_optical_window",
            "tare_checkweight_rail",
            "anti_slosh_baffle_witness_coupons",
            "temperature_logger_pockets",
            "rh_logger_pockets",
            "drain_route_witness",
            "prime_route_witness",
            "barcode_custody_lands",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "evidence_bridge",
            "robot_keepouts",
            "service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn scope_excludes_process_weighing_pressure_and_biology_claims() {
        assert!(DESIGN_SCOPE.contains("mechanical validation packaging only"));
        assert!(DESIGN_SCOPE.contains("not a sterile-process claim"));
        assert!(DESIGN_SCOPE.contains("dosing SOP"));
        assert!(DESIGN_SCOPE.contains("certified weighing device"));
        assert!(DESIGN_SCOPE.contains("pressure-rated fluid path"));
        assert!(DESIGN_SCOPE.contains("biological acceptance criterion"));
    }

    #[test]
    fn feature_counts_match_false_mass_sentinel_packaging() {
        assert_eq!(LOAD_CELL_COUNT, 4);
        assert_eq!(FALSE_MASS_REFERENCE_TABS, 6);
        assert_eq!(SETTLE_FORKS, 6);
        assert_eq!(OPTICAL_WINDOWS, 8);
        assert_eq!(BACKLIGHT_SLOTS, OPTICAL_WINDOWS);
        assert_eq!(CHECKWEIGHT_NESTS, 6);
        assert_eq!(TARE_TOKEN_SLOTS, CHECKWEIGHT_NESTS);
        assert_eq!(BAFFLE_COUPONS, 8);
        assert_eq!(BAFFLE_TICK_MARKS, 24);
        assert_eq!(LOGGER_POCKETS, 4);
        assert_eq!(ROUTE_WITNESS_PORTS, 8);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(LOT_CUSTODY_LANDS, 4);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(EVIDENCE_FIDUCIALS, 10);
    }

    #[test]
    fn repeated_features_stay_inside_their_hardware_envelopes() {
        for idx in 0..LOAD_CELL_COUNT {
            let (x, y) = load_cell_center(idx);
            assert!(x.abs() + LOAD_CELL_POCKET_X / 2.0 < CRADLE_X / 2.0 - 34.0);
            assert!(y.abs() + LOAD_CELL_POCKET_Y / 2.0 < CRADLE_Y / 2.0 - 24.0);
        }

        assert!(optical_window_span_x() < WINDOW_X - 70.0);
        assert!(checkweight_span_x() < CHECK_RAIL_X - 92.0);
        assert!(baffle_coupon_span_x() < BAFFLE_X - 100.0);
        assert!(baffle_coupon_span_y() < BAFFLE_Y - 40.0);
        assert!(route_port_span_x() < ROUTE_X - 74.0);
        assert!(logger_span_x() < LOGGER_X - 48.0);
    }

    #[test]
    fn disposition_names_and_clearances_are_stable() {
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert!(bridge_clearance_over_modules() >= 120.0);
        assert!(all_modules_fit_inner_deck());
        assert_layout();
    }
}
