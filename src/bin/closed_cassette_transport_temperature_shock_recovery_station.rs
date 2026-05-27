use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette transport temperature-shock recovery station.
//
// No-cell validation fixture for moving sealed cassettes between modules. The
// station makes short cold/hot shocks, recovery lag, condensation witness
// status, logger custody, transfer dwell timing, and release/hold/reject routing
// physically visible before a cassette can return to incubation.
//
// Mechanical concept CAD only. This file models validation fixture geometry for
// review and printing; it is not a thermal qualification protocol, biological
// acceptance rule, incubation release criterion, or sterile-barrier design.

const PART_PREFIX: &str = "closed_cassette_transport_temperature_shock_recovery_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_transport_temperature_shock_recovery_station_containment_deck.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_sealed_cassette_transport_nest.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_thermal_shock_coupon_carriers.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_recovery_logger_pockets.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_condensation_witness_wells_shields.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_dwell_timer_token_rails.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_warm_cold_comparison_blocks.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_barcode_custody_lands.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_evidence_bridge.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_transport_temperature_shock_recovery_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "sealed_cassette_transport_nest",
    "thermal_shock_coupon_carriers",
    "recovery_logger_pockets",
    "condensation_witness_wells_shields",
    "dwell_timer_token_rails",
    "warm_cold_comparison_blocks",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 48.0;
const SUMP_X: f64 = 1370.0;
const SUMP_Y: f64 = 790.0;
const SUMP_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.5;
const DRAIN_D: f64 = 18.0;
const DECK_MOUNT_BOSSES: usize = 8;

const NEST_POS: (f64, f64) = (-430.0, 215.0);
const NEST_X: f64 = 580.0;
const NEST_Y: f64 = 320.0;
const NEST_Z: f64 = 58.0;
const CASSETTE_SLOTS: usize = 2;
const CASSETTE_SLOT_X: f64 = 218.0;
const CASSETTE_SLOT_Y: f64 = 238.0;
const CASSETTE_SLOT_DEPTH: f64 = 18.0;
const CASSETTE_SLOT_PITCH_X: f64 = 270.0;
const SEAL_LATCH_LANDS: usize = 8;
const ORIENTATION_KEYS: usize = 6;

const COUPON_POS: (f64, f64) = (130.0, 270.0);
const COUPON_X: f64 = 510.0;
const COUPON_Y: f64 = 310.0;
const COUPON_Z: f64 = 44.0;
const SHOCK_COUPON_COLS: usize = 4;
const SHOCK_COUPON_ROWS: usize = 3;
const SHOCK_COUPONS: usize = SHOCK_COUPON_COLS * SHOCK_COUPON_ROWS;
const COUPON_SLOT_X: f64 = 78.0;
const COUPON_SLOT_Y: f64 = 48.0;
const COUPON_SLOT_DEPTH: f64 = 12.0;
const COUPON_PITCH_X: f64 = 108.0;
const COUPON_PITCH_Y: f64 = 72.0;
const SHOCK_RAMP_TEETH: usize = 7;

const LOGGER_POS: (f64, f64) = (-545.0, -218.0);
const LOGGER_X: f64 = 370.0;
const LOGGER_Y: f64 = 180.0;
const LOGGER_Z: f64 = 34.0;
const LOGGER_POCKETS: usize = 6;
const LOGGER_SLOT_X: f64 = 50.0;
const LOGGER_SLOT_Y: f64 = 94.0;
const LOGGER_SLOT_DEPTH: f64 = 11.0;
const LOGGER_SEAL_WELLS: usize = 8;
const LOGGER_CONTACTS_PER_POCKET: usize = 3;

const WITNESS_POS: (f64, f64) = (-95.0, -145.0);
const WITNESS_X: f64 = 430.0;
const WITNESS_Y: f64 = 190.0;
const WITNESS_Z: f64 = 34.0;
const WITNESS_WELLS: usize = 10;
const WITNESS_COLS: usize = 5;
const WITNESS_ROWS: usize = 2;
const WITNESS_WELL_D: f64 = 30.0;
const WITNESS_WELL_DEPTH: f64 = 13.0;
const WITNESS_SHIELDS: usize = 5;

const DWELL_POS: (f64, f64) = (0.0, -375.0);
const DWELL_X: f64 = 710.0;
const DWELL_Y: f64 = 104.0;
const DWELL_Z: f64 = 30.0;
const DWELL_LANES: usize = 3;
const DWELL_TOKENS_PER_LANE: usize = 5;
const DWELL_TOKEN_SLOTS: usize = DWELL_LANES * DWELL_TOKENS_PER_LANE;
const DWELL_SLOT_X: f64 = 56.0;
const DWELL_SLOT_Y: f64 = 22.0;
const DWELL_SLOT_DEPTH: f64 = 8.0;
const DWELL_TICKS: usize = DWELL_TOKENS_PER_LANE + 1;

const COMPARISON_POS: (f64, f64) = (540.0, -320.0);
const COMPARISON_X: f64 = 340.0;
const COMPARISON_Y: f64 = 220.0;
const COMPARISON_Z: f64 = 52.0;
const COMPARISON_BLOCKS: usize = 3;
const THERMISTOR_WELLS_PER_BLOCK: usize = 4;
const CONDUCTION_RIBS_PER_BLOCK: usize = 5;

const CUSTODY_POS: (f64, f64) = (560.0, 300.0);
const CUSTODY_X: f64 = 330.0;
const CUSTODY_Y: f64 = 190.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 8;
const LOGGER_CUSTODY_TAGS: usize = 6;
const CHAIN_OF_CUSTODY_CARD_SLOTS: usize = 3;

const LANE_POS: (f64, f64) = (430.0, -15.0);
const LANE_X: f64 = 520.0;
const LANE_Y: f64 = 210.0;
const LANE_Z: f64 = 36.0;
const ROUTE_LANES: usize = 3;
const ROUTE_LANE_NAMES: [&str; ROUTE_LANES] = ["release", "hold", "reject"];
const ROUTE_SLOTS_PER_LANE: usize = 4;
const ROUTE_SLOT_X: f64 = 112.0;
const ROUTE_SLOT_Y: f64 = 32.0;
const ROUTE_SLOT_DEPTH: f64 = 8.0;

const BRIDGE_POS: (f64, f64) = (0.0, 405.0);
const BRIDGE_SPAN_X: f64 = 1260.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_POST_Z: f64 = 188.0;
const BRIDGE_BEAM_Y: f64 = 42.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;

const ROBOT_SWEEP_X: f64 = 1200.0;
const ROBOT_SWEEP_Y: f64 = 790.0;
const ROBOT_SWEEP_Z: f64 = 172.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 435.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 275.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 245.0;
const TOP_BRIDGE_CLEARANCE_Z: f64 = 330.0;
const KEEP_OUT_POSTS: usize = 6;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let nest = sealed_cassette_transport_nest();
    export(&nest, OUTPUTS[1]);

    let coupons = thermal_shock_coupon_carriers();
    export(&coupons, OUTPUTS[2]);

    let loggers = recovery_logger_pockets();
    export(&loggers, OUTPUTS[3]);

    let witnesses = condensation_witness_wells_shields();
    export(&witnesses, OUTPUTS[4]);

    let dwell = dwell_timer_token_rails();
    export(&dwell, OUTPUTS[5]);

    let comparison = warm_cold_comparison_blocks();
    export(&comparison, OUTPUTS[6]);

    let custody = barcode_custody_lands();
    export(&custody, OUTPUTS[7]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[8]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[10]);

    let assembly = station_assembly();
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed cassette transport temperature-shock recovery station:");
    println!(
        "  Containment deck:      {STATION_X:.0}mm x {STATION_Y:.0}mm with sump, drain, curbs, and module socket recesses"
    );
    println!(
        "  Transport nest:        {CASSETTE_SLOTS} sealed cassette slots, {SEAL_LATCH_LANDS} latch lands, {ORIENTATION_KEYS} orientation keys"
    );
    println!(
        "  Shock coupons:         {SHOCK_COUPONS} coupon carriers, {SHOCK_RAMP_TEETH} ramp teeth for short-shock staging"
    );
    println!(
        "  Recovery loggers:      {LOGGER_POCKETS} logger pockets, {LOGGER_SEAL_WELLS} seal wells, {} contact lands",
        LOGGER_POCKETS * LOGGER_CONTACTS_PER_POCKET
    );
    println!(
        "  Condensation witness:  {WITNESS_WELLS} witness wells and {WITNESS_SHIELDS} raised splash/airflow shields"
    );
    println!(
        "  Dwell proof:           {DWELL_LANES} dwell lanes, {DWELL_TOKEN_SLOTS} token slots, {DWELL_TICKS} tick marks per lane"
    );
    println!(
        "  Thermal comparison:    {COMPARISON_BLOCKS} warm/cold/control blocks with {} thermistor wells",
        COMPARISON_BLOCKS * THERMISTOR_WELLS_PER_BLOCK
    );
    println!(
        "  Custody/routing:       {BARCODE_LANDS} barcode lands, {LOGGER_CUSTODY_TAGS} logger tags, {:?} lanes",
        ROUTE_LANE_NAMES
    );
    println!(
        "  Robot/service gauges:  front {FRONT_ROBOT_CLEARANCE_Y:.0}mm, rear {REAR_SERVICE_CLEARANCE_Y:.0}mm, side {SIDE_SERVICE_CLEARANCE_X:.0}mm, top {TOP_BRIDGE_CLEARANCE_Z:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + sealed_cassette_transport_nest().translate(NEST_POS.0, NEST_POS.1, DECK_Z)
        + thermal_shock_coupon_carriers().translate(COUPON_POS.0, COUPON_POS.1, DECK_Z)
        + recovery_logger_pockets().translate(LOGGER_POS.0, LOGGER_POS.1, DECK_Z)
        + condensation_witness_wells_shields().translate(WITNESS_POS.0, WITNESS_POS.1, DECK_Z)
        + dwell_timer_token_rails().translate(DWELL_POS.0, DWELL_POS.1, DECK_Z)
        + warm_cold_comparison_blocks().translate(COMPARISON_POS.0, COMPARISON_POS.1, DECK_Z)
        + barcode_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, DECK_Z)
        + release_hold_reject_lanes().translate(LANE_POS.0, LANE_POS.1, DECK_Z)
        + evidence_bridge().translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z)
        + robot_service_keepout_gauges()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PART_PREFIX}_containment_deck_floor"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        format!("{PART_PREFIX}_temperature_shock_condensate_sump_cut"),
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{PART_PREFIX}_front_bulkhead_drain_port"),
        DRAIN_D / 2.0,
        CURB_W + 36.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 88.0,
        -STATION_Y / 2.0 + 14.0,
        DECK_Z - 6.0,
    );

    deck - sump - drain - module_socket_recesses()
        + containment_curbs()
        + deck_mount_bosses()
        + zone_lands()
        + condensate_flow_ribs()
}

fn containment_curbs() -> Part {
    let z = DECK_Z + CURB_Z / 2.0;
    let front = centered_cube(
        format!("{PART_PREFIX}_front_condensate_curb"),
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, z);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_incubator_return_curb"),
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, z);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_module_transfer_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_release_route_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{PART_PREFIX}_module_socket_recesses"));
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{PART_PREFIX}_{}_socket_recess", footprint.name),
                footprint.x + 8.0,
                footprint.y + 8.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.25,
            );
    }
    sockets
}

fn deck_mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PART_PREFIX}_deck_mount_bosses"));
    for index in 0..DECK_MOUNT_BOSSES {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let y_index = index / 2;
        let x = side * (STATION_X / 2.0 - 82.0);
        let y = centered_index(y_index, DECK_MOUNT_BOSSES / 2, 236.0);
        let boss = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_boss_{index}"),
            17.0,
            10.0,
            40,
        )
        .translate(x, y, DECK_Z + 5.0);
        let hole = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_hole_{index}"),
            3.4,
            14.0,
            28,
        )
        .translate(x, y, DECK_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn zone_lands() -> Part {
    let incoming = centered_cube(
        format!("{PART_PREFIX}_incoming_transport_zone_land"),
        NEST_X + 44.0,
        NEST_Y + 40.0,
        3.0,
    )
    .translate(NEST_POS.0, NEST_POS.1, DECK_Z + 1.5);
    let shock = centered_cube(
        format!("{PART_PREFIX}_temperature_shock_evidence_zone_land"),
        920.0,
        382.0,
        3.0,
    )
    .translate(170.0, 190.0, DECK_Z + 1.5);
    let decision = centered_cube(
        format!("{PART_PREFIX}_recovery_routing_decision_zone_land"),
        1120.0,
        362.0,
        3.0,
    )
    .translate(100.0, -245.0, DECK_Z + 1.5);

    incoming + shock + decision
}

fn condensate_flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_condensate_flow_ribs"));
    for index in 0..7 {
        ribs = ribs
            + centered_cube(
                format!("{PART_PREFIX}_condensate_flow_rib_{index}"),
                14.0,
                SUMP_Y - 160.0,
                5.0,
            )
            .translate(centered_index(index, 7, 160.0), -44.0, DECK_Z + 2.5);
    }
    ribs
}

fn sealed_cassette_transport_nest() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_sealed_cassette_transport_nest_body"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_sealed_cassette_nest_cuts"));
    for slot in 0..CASSETTE_SLOTS {
        let x = cassette_slot_x(slot);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_cassette_{slot}_sealed_transport_floor_recess"),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                CASSETTE_SLOT_DEPTH + 1.0,
            )
            .translate(x, 8.0, NEST_Z - CASSETTE_SLOT_DEPTH / 2.0 + 0.5);
    }

    body - cuts + cassette_guide_rails() + cassette_latch_lands() + orientation_key_posts()
}

fn cassette_guide_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_cassette_guide_rails"));
    for slot in 0..CASSETTE_SLOTS {
        let x = cassette_slot_x(slot);
        let left = centered_cube(
            format!("{PART_PREFIX}_cassette_{slot}_left_thermal_isolation_rail"),
            16.0,
            CASSETTE_SLOT_Y + 58.0,
            48.0,
        )
        .translate(x - CASSETTE_SLOT_X / 2.0 - 22.0, 8.0, NEST_Z + 24.0);
        let right = centered_cube(
            format!("{PART_PREFIX}_cassette_{slot}_right_thermal_isolation_rail"),
            16.0,
            CASSETTE_SLOT_Y + 58.0,
            48.0,
        )
        .translate(x + CASSETTE_SLOT_X / 2.0 + 22.0, 8.0, NEST_Z + 24.0);
        let rear = centered_cube(
            format!("{PART_PREFIX}_cassette_{slot}_rear_incubator_return_stop"),
            CASSETTE_SLOT_X + 64.0,
            18.0,
            48.0,
        )
        .translate(x, CASSETTE_SLOT_Y / 2.0 + 42.0, NEST_Z + 24.0);
        rails = rails + left + right + rear;
    }
    rails
}

fn cassette_latch_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_cassette_latch_lands"));
    for index in 0..SEAL_LATCH_LANDS {
        let slot = index / (SEAL_LATCH_LANDS / CASSETTE_SLOTS);
        let local = index % (SEAL_LATCH_LANDS / CASSETTE_SLOTS);
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_cassette_latch_tamper_land_{index}"),
                42.0,
                22.0,
                10.0,
            )
            .translate(
                cassette_slot_x(slot) + centered_index(local, 4, 56.0),
                -NEST_Y / 2.0 + 31.0,
                NEST_Z + 5.0,
            );
    }
    lands
}

fn orientation_key_posts() -> Part {
    let mut keys = Part::empty(format!("{PART_PREFIX}_orientation_key_posts"));
    for index in 0..ORIENTATION_KEYS {
        let slot = index / (ORIENTATION_KEYS / CASSETTE_SLOTS);
        let local = index % (ORIENTATION_KEYS / CASSETTE_SLOTS);
        keys = keys
            + centered_cylinder(
                format!("{PART_PREFIX}_cassette_{slot}_orientation_key_{local}"),
                6.0,
                16.0,
                24,
            )
            .translate(
                cassette_slot_x(slot) + centered_index(local, 3, 46.0),
                NEST_Y / 2.0 - 44.0,
                NEST_Z + 8.0,
            );
    }
    keys
}

fn thermal_shock_coupon_carriers() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_thermal_shock_coupon_carrier_plate"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_thermal_shock_coupon_slot_cuts"));
    for index in 0..SHOCK_COUPONS {
        let col = index % SHOCK_COUPON_COLS;
        let row = index / SHOCK_COUPON_COLS;
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_shock_coupon_{index}_recess"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_SLOT_DEPTH + 1.0,
            )
            .translate(
                centered_index(col, SHOCK_COUPON_COLS, COUPON_PITCH_X),
                centered_index(row, SHOCK_COUPON_ROWS, COUPON_PITCH_Y),
                COUPON_Z - COUPON_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    plate - cuts + coupon_end_stops() + shock_ramp_teeth()
}

fn coupon_end_stops() -> Part {
    let front = centered_cube(
        format!("{PART_PREFIX}_thermal_coupon_front_stop"),
        COUPON_X - 32.0,
        12.0,
        28.0,
    )
    .translate(0.0, -COUPON_Y / 2.0 + 24.0, COUPON_Z + 14.0);
    let rear = centered_cube(
        format!("{PART_PREFIX}_thermal_coupon_rear_stop"),
        COUPON_X - 32.0,
        12.0,
        28.0,
    )
    .translate(0.0, COUPON_Y / 2.0 - 24.0, COUPON_Z + 14.0);
    front + rear
}

fn shock_ramp_teeth() -> Part {
    let mut teeth = Part::empty(format!("{PART_PREFIX}_short_shock_ramp_teeth"));
    for index in 0..SHOCK_RAMP_TEETH {
        teeth = teeth
            + centered_cube(
                format!("{PART_PREFIX}_shock_ramp_temperature_step_tooth_{index}"),
                18.0,
                COUPON_Y - 78.0,
                8.0 + index as f64 * 1.2,
            )
            .translate(
                centered_index(index, SHOCK_RAMP_TEETH, 52.0),
                0.0,
                COUPON_Z + 4.0 + index as f64 * 0.6,
            );
    }
    teeth
}

fn recovery_logger_pockets() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_recovery_logger_pocket_panel"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_recovery_logger_pocket_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_logger_{index}_custody_recovery_slot"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH + 1.0,
            )
            .translate(
                logger_x(index),
                -8.0,
                LOGGER_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    body - cuts + logger_cable_comb() + logger_seal_wells() + logger_contact_lands()
}

fn logger_cable_comb() -> Part {
    let comb = centered_cube(
        format!("{PART_PREFIX}_logger_cable_comb"),
        LOGGER_X - 48.0,
        22.0,
        18.0,
    )
    .translate(0.0, LOGGER_Y / 2.0 + 16.0, LOGGER_Z - 8.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_logger_cable_comb_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_logger_{index}_cable_channel"),
                3.4,
                54.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(logger_x(index), LOGGER_Y / 2.0 + 16.0, LOGGER_Z - 8.0);
    }
    comb - cuts
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_logger_tamper_seal_wells"));
    for index in 0..LOGGER_SEAL_WELLS {
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_logger_tamper_seal_ring_{index}"),
            8.0,
            7.0,
            28,
        )
        .translate(
            centered_index(index, LOGGER_SEAL_WELLS, 38.0),
            -LOGGER_Y / 2.0 + 22.0,
            LOGGER_Z + 3.5,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_logger_tamper_seal_recess_{index}"),
            4.8,
            9.0,
            28,
        )
        .translate(
            centered_index(index, LOGGER_SEAL_WELLS, 38.0),
            -LOGGER_Y / 2.0 + 22.0,
            LOGGER_Z + 3.5,
        );
        wells = wells + (ring - cut);
    }
    wells
}

fn logger_contact_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_logger_contact_lands"));
    for pocket in 0..LOGGER_POCKETS {
        for contact in 0..LOGGER_CONTACTS_PER_POCKET {
            lands = lands
                + centered_cylinder(
                    format!("{PART_PREFIX}_logger_{pocket}_contact_land_{contact}"),
                    2.4,
                    3.0,
                    18,
                )
                .translate(
                    logger_x(pocket) + centered_index(contact, LOGGER_CONTACTS_PER_POCKET, 10.0),
                    48.0,
                    LOGGER_Z + 1.5,
                );
        }
    }
    lands
}

fn condensation_witness_wells_shields() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_condensation_witness_well_panel"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);
    body - witness_well_cuts()
        + witness_well_rings()
        + witness_splash_shields()
        + dry_control_tabs()
}

fn witness_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_condensation_witness_well_cuts"));
    for index in 0..WITNESS_WELLS {
        let col = index % WITNESS_COLS;
        let row = index / WITNESS_COLS;
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_condensation_witness_well_cut_{index}"),
                WITNESS_WELL_D / 2.0,
                WITNESS_WELL_DEPTH + 1.0,
                36,
            )
            .translate(
                centered_index(col, WITNESS_COLS, 72.0),
                centered_index(row, WITNESS_ROWS, 58.0),
                WITNESS_Z - WITNESS_WELL_DEPTH / 2.0 + 0.5,
            );
    }
    cuts
}

fn witness_well_rings() -> Part {
    let mut rings = Part::empty(format!("{PART_PREFIX}_condensation_witness_well_rings"));
    for index in 0..WITNESS_WELLS {
        let col = index % WITNESS_COLS;
        let row = index / WITNESS_COLS;
        rings = rings
            + centered_cylinder(
                format!("{PART_PREFIX}_condensation_witness_well_rim_{index}"),
                WITNESS_WELL_D / 2.0 + 4.0,
                5.0,
                36,
            )
            .translate(
                centered_index(col, WITNESS_COLS, 72.0),
                centered_index(row, WITNESS_ROWS, 58.0),
                WITNESS_Z + 2.5,
            );
    }
    rings - witness_well_cuts().translate(0.0, 0.0, WITNESS_WELL_DEPTH)
}

fn witness_splash_shields() -> Part {
    let mut shields = Part::empty(format!("{PART_PREFIX}_condensation_witness_splash_shields"));
    for index in 0..WITNESS_SHIELDS {
        shields = shields
            + centered_cube(
                format!("{PART_PREFIX}_condensation_witness_airflow_shield_{index}"),
                12.0,
                WITNESS_Y - 26.0,
                54.0,
            )
            .translate(
                centered_index(index, WITNESS_SHIELDS, 78.0),
                0.0,
                WITNESS_Z + 27.0,
            );
    }
    shields
}

fn dry_control_tabs() -> Part {
    let cold = centered_cube(
        format!("{PART_PREFIX}_cold_shock_dry_control_tab"),
        82.0,
        24.0,
        6.0,
    )
    .translate(
        -WITNESS_X / 2.0 + 58.0,
        WITNESS_Y / 2.0 + 20.0,
        WITNESS_Z + 3.0,
    );
    let warm = centered_cube(
        format!("{PART_PREFIX}_warm_recovery_dry_control_tab"),
        82.0,
        24.0,
        6.0,
    )
    .translate(
        WITNESS_X / 2.0 - 58.0,
        WITNESS_Y / 2.0 + 20.0,
        WITNESS_Z + 3.0,
    );
    cold + warm
}

fn dwell_timer_token_rails() -> Part {
    let rail = centered_cube(
        format!("{PART_PREFIX}_dwell_timer_token_rail_body"),
        DWELL_X,
        DWELL_Y,
        DWELL_Z,
    )
    .translate(0.0, 0.0, DWELL_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_dwell_timer_token_slot_cuts"));
    for lane in 0..DWELL_LANES {
        for slot in 0..DWELL_TOKENS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{PART_PREFIX}_dwell_lane_{lane}_token_slot_{slot}"),
                    DWELL_SLOT_X,
                    DWELL_SLOT_Y,
                    DWELL_SLOT_DEPTH + 1.0,
                )
                .translate(
                    dwell_slot_x(slot),
                    dwell_lane_y(lane),
                    DWELL_Z - DWELL_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }
    rail - cuts + dwell_tick_marks() + dwell_lane_guards()
}

fn dwell_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_dwell_timer_tick_marks"));
    for lane in 0..DWELL_LANES {
        for tick in 0..DWELL_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{PART_PREFIX}_dwell_lane_{lane}_tick_{tick}"),
                    4.0,
                    18.0,
                    8.0,
                )
                .translate(
                    centered_index(tick, DWELL_TICKS, 112.0),
                    dwell_lane_y(lane) + 18.0,
                    DWELL_Z + 4.0,
                );
        }
    }
    ticks
}

fn dwell_lane_guards() -> Part {
    let mut guards = Part::empty(format!("{PART_PREFIX}_dwell_lane_guards"));
    for lane in 0..DWELL_LANES {
        guards = guards
            + centered_cube(
                format!("{PART_PREFIX}_dwell_lane_{lane}_raised_guard"),
                DWELL_X - 42.0,
                5.0,
                10.0,
            )
            .translate(0.0, dwell_lane_y(lane) - 16.0, DWELL_Z + 5.0);
    }
    guards
}

fn warm_cold_comparison_blocks() -> Part {
    let mut blocks = Part::empty(format!("{PART_PREFIX}_warm_cold_comparison_blocks"));
    for block in 0..COMPARISON_BLOCKS {
        let x = centered_index(block, COMPARISON_BLOCKS, 108.0);
        let body = centered_cube(
            format!(
                "{PART_PREFIX}_{}_comparison_block_body",
                comparison_name(block)
            ),
            88.0,
            COMPARISON_Y,
            COMPARISON_Z,
        )
        .translate(x, 0.0, COMPARISON_Z / 2.0);
        blocks = blocks + body + comparison_thermistor_wells(block, x) + conduction_ribs(block, x);
    }
    blocks
}

fn comparison_thermistor_wells(block: usize, x: f64) -> Part {
    let mut wells = Part::empty(format!(
        "{PART_PREFIX}_{}_thermistor_wells",
        comparison_name(block)
    ));
    for index in 0..THERMISTOR_WELLS_PER_BLOCK {
        let ring = centered_cylinder(
            format!(
                "{PART_PREFIX}_{}_thermistor_ring_{index}",
                comparison_name(block)
            ),
            9.0,
            7.0,
            28,
        )
        .translate(
            x,
            centered_index(index, THERMISTOR_WELLS_PER_BLOCK, 42.0),
            COMPARISON_Z + 3.5,
        );
        let cut = centered_cylinder(
            format!(
                "{PART_PREFIX}_{}_thermistor_recess_{index}",
                comparison_name(block)
            ),
            5.0,
            9.0,
            28,
        )
        .translate(
            x,
            centered_index(index, THERMISTOR_WELLS_PER_BLOCK, 42.0),
            COMPARISON_Z + 3.5,
        );
        wells = wells + (ring - cut);
    }
    wells
}

fn conduction_ribs(block: usize, x: f64) -> Part {
    let mut ribs = Part::empty(format!(
        "{PART_PREFIX}_{}_conduction_ribs",
        comparison_name(block)
    ));
    for index in 0..CONDUCTION_RIBS_PER_BLOCK {
        ribs = ribs
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_conduction_rib_{index}",
                    comparison_name(block)
                ),
                58.0,
                8.0,
                8.0,
            )
            .translate(
                x,
                centered_index(index, CONDUCTION_RIBS_PER_BLOCK, 34.0),
                COMPARISON_Z + 4.0,
            );
    }
    ribs
}

fn barcode_custody_lands() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_barcode_custody_land_plate"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);
    plate - custody_card_slot_cuts()
        + barcode_lands()
        + logger_custody_tags()
        + custody_token_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_barcode_lands"));
    for index in 0..BARCODE_LANDS {
        let row = index / 4;
        let col = index % 4;
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_barcode_land_{index}"),
                62.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 76.0),
                -CUSTODY_Y / 2.0 + 30.0 + row as f64 * 34.0,
                CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn logger_custody_tags() -> Part {
    let mut tags = Part::empty(format!("{PART_PREFIX}_logger_custody_tag_lands"));
    for index in 0..LOGGER_CUSTODY_TAGS {
        tags = tags
            + centered_cube(
                format!("{PART_PREFIX}_logger_custody_tag_land_{index}"),
                34.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(index, LOGGER_CUSTODY_TAGS, 44.0),
                CUSTODY_Y / 2.0 - 32.0,
                CUSTODY_Z + 2.5,
            );
    }
    tags
}

fn custody_card_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_chain_of_custody_card_slot_cuts"));
    for index in 0..CHAIN_OF_CUSTODY_CARD_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_custody_card_slot_cut_{index}"),
                74.0,
                38.0,
                6.0,
            )
            .translate(
                centered_index(index, CHAIN_OF_CUSTODY_CARD_SLOTS, 88.0),
                20.0,
                CUSTODY_Z - 3.0,
            );
    }
    cuts
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_custody_mismatch_token_wells"));
    for index in 0..6 {
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_custody_mismatch_token_ring_{index}"),
            8.0,
            6.0,
            28,
        )
        .translate(
            centered_index(index, 6, 42.0),
            -CUSTODY_Y / 2.0 - 16.0,
            CUSTODY_Z + 3.0,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_custody_mismatch_token_recess_{index}"),
            4.6,
            8.0,
            28,
        )
        .translate(
            centered_index(index, 6, 42.0),
            -CUSTODY_Y / 2.0 - 16.0,
            CUSTODY_Z + 3.0,
        );
        wells = wells + (ring - cut);
    }
    wells
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_release_hold_reject_lane_panel"),
        LANE_X,
        LANE_Y,
        LANE_Z,
    )
    .translate(0.0, 0.0, LANE_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_release_hold_reject_lane_slot_cuts"));
    for lane in 0..ROUTE_LANES {
        for slot in 0..ROUTE_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "{PART_PREFIX}_{}_cassette_route_slot_{slot}",
                        ROUTE_LANE_NAMES[lane]
                    ),
                    ROUTE_SLOT_X,
                    ROUTE_SLOT_Y,
                    ROUTE_SLOT_DEPTH + 1.0,
                )
                .translate(
                    route_lane_x(lane),
                    centered_index(slot, ROUTE_SLOTS_PER_LANE, 46.0),
                    LANE_Z - ROUTE_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }
    panel - cuts + route_lane_dividers() + route_gate_lands()
}

fn route_lane_dividers() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_release_hold_lane_divider"),
        12.0,
        LANE_Y - 24.0,
        32.0,
    )
    .translate(-104.0, 0.0, LANE_Z + 16.0);
    let right = centered_cube(
        format!("{PART_PREFIX}_hold_reject_lane_divider"),
        12.0,
        LANE_Y - 24.0,
        32.0,
    )
    .translate(104.0, 0.0, LANE_Z + 16.0);
    let stop = centered_cube(
        format!("{PART_PREFIX}_locked_route_front_stop_bar"),
        LANE_X - 36.0,
        12.0,
        26.0,
    )
    .translate(0.0, -LANE_Y / 2.0 + 22.0, LANE_Z + 13.0);
    left + right + stop
}

fn route_gate_lands() -> Part {
    let mut gates = Part::empty(format!("{PART_PREFIX}_route_gate_lands"));
    for lane in 0..ROUTE_LANES {
        gates = gates
            + centered_cube(
                format!("{PART_PREFIX}_{}_route_gate_land", ROUTE_LANE_NAMES[lane]),
                118.0,
                22.0,
                18.0,
            )
            .translate(route_lane_x(lane), LANE_Y / 2.0 + 17.0, LANE_Z + 9.0);
    }
    gates
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let right_post = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let beam = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_camera_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);
    left_post + right_post + beam + camera_pods() + bridge_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_evidence_camera_pods"));
    for index in 0..CAMERA_PODS {
        let pod = centered_cube(
            format!("{PART_PREFIX}_evidence_camera_pod_{index}"),
            54.0,
            34.0,
            28.0,
        )
        .translate(
            centered_index(index, CAMERA_PODS, 240.0),
            -BRIDGE_BEAM_Y / 2.0 - 20.0,
            BRIDGE_POST_Z - 14.0,
        );
        let lens = centered_cylinder(
            format!("{PART_PREFIX}_evidence_camera_lens_clearance_{index}"),
            8.0,
            12.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, CAMERA_PODS, 240.0),
            -BRIDGE_BEAM_Y / 2.0 - 26.0,
            BRIDGE_POST_Z - 14.0,
        );
        pods = pods + (pod - lens);
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{PART_PREFIX}_evidence_bridge_light_bars"));
    for index in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{PART_PREFIX}_evidence_light_bar_{index}"),
                180.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(index, LIGHT_BARS, 270.0),
                BRIDGE_BEAM_Y / 2.0 + 8.0,
                BRIDGE_POST_Z - 18.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let sweep = centered_cube(
        format!("{PART_PREFIX}_robot_sweep_keepout_gauge"),
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        6.0,
    )
    .translate(0.0, -20.0, DECK_Z + ROBOT_SWEEP_Z);
    let front = centered_cube(
        format!("{PART_PREFIX}_front_robot_approach_clearance_gauge"),
        ROBOT_SWEEP_X,
        10.0,
        44.0,
    )
    .translate(0.0, -FRONT_ROBOT_CLEARANCE_Y, DECK_Z + 22.0);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_service_clearance_gauge"),
        ROBOT_SWEEP_X,
        10.0,
        44.0,
    )
    .translate(0.0, REAR_SERVICE_CLEARANCE_Y, DECK_Z + 22.0);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_service_clearance_gauge"),
        10.0,
        ROBOT_SWEEP_Y,
        44.0,
    )
    .translate(-SIDE_SERVICE_CLEARANCE_X, 0.0, DECK_Z + 22.0);
    let top = centered_cube(
        format!("{PART_PREFIX}_top_bridge_service_clearance_gauge"),
        BRIDGE_SPAN_X,
        18.0,
        10.0,
    )
    .translate(0.0, BRIDGE_POS.1, TOP_BRIDGE_CLEARANCE_Z);
    sweep + front + rear + left + top + keepout_posts()
}

fn keepout_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_keepout_corner_posts"));
    for index in 0..KEEP_OUT_POSTS {
        let x = centered_index(index % 3, 3, ROBOT_SWEEP_X / 2.0);
        let y = if index < 3 {
            -ROBOT_SWEEP_Y / 2.0
        } else {
            ROBOT_SWEEP_Y / 2.0
        };
        posts = posts
            + centered_cylinder(
                format!("{PART_PREFIX}_robot_service_keepout_post_{index}"),
                7.0,
                ROBOT_SWEEP_Z,
                24,
            )
            .translate(x, y, DECK_Z + ROBOT_SWEEP_Z / 2.0);
    }
    posts
}

fn module_footprints() -> [Footprint; 8] {
    [
        Footprint {
            name: "transport_nest",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Footprint {
            name: "shock_coupons",
            center: COUPON_POS,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Footprint {
            name: "logger_pockets",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Footprint {
            name: "witness_wells",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "dwell_timer",
            center: DWELL_POS,
            x: DWELL_X,
            y: DWELL_Y,
        },
        Footprint {
            name: "comparison_blocks",
            center: COMPARISON_POS,
            x: COMPARISON_X,
            y: COMPARISON_Y,
        },
        Footprint {
            name: "custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "route_lanes",
            center: LANE_POS,
            x: LANE_X,
            y: LANE_Y,
        },
    ]
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(CASSETTE_SLOTS, 2);
    assert_eq!(SHOCK_COUPONS, 12);
    assert_eq!(DWELL_TOKEN_SLOTS, 15);
    assert_eq!(ROUTE_LANES, 3);
    assert!(FRONT_ROBOT_CLEARANCE_Y > CASSETTE_SLOT_Y);
    assert!(TOP_BRIDGE_CLEARANCE_Z > BRIDGE_POST_Z + BRIDGE_BEAM_Z);

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

fn overlaps(left: Footprint, right: Footprint) -> bool {
    let dx = (left.center.0 - right.center.0).abs();
    let dy = (left.center.1 - right.center.1).abs();
    dx < (left.x + right.x) / 2.0 && dy < (left.y + right.y) / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn cassette_slot_x(slot: usize) -> f64 {
    centered_index(slot, CASSETTE_SLOTS, CASSETTE_SLOT_PITCH_X)
}

fn logger_x(index: usize) -> f64 {
    centered_index(index, LOGGER_POCKETS, 56.0)
}

fn dwell_slot_x(slot: usize) -> f64 {
    centered_index(slot, DWELL_TOKENS_PER_LANE, 116.0)
}

fn dwell_lane_y(lane: usize) -> f64 {
    centered_index(lane, DWELL_LANES, 30.0)
}

fn comparison_name(block: usize) -> &'static str {
    match block {
        0 => "cold_shock",
        1 => "ambient_control",
        2 => "warm_recovery",
        _ => unreachable!("comparison block index out of range"),
    }
}

fn route_lane_x(lane: usize) -> f64 {
    centered_index(lane, ROUTE_LANES, 156.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_count_and_paths_are_unique() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_cassette_transport_temperature_shock_recovery_station_")));
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_validation_features_are_represented() {
        let joined = OUTPUTS.join("\n");
        for feature in REQUIRED_FEATURES {
            assert!(
                joined.contains(feature),
                "missing required feature output for {feature}"
            );
        }
    }

    #[test]
    fn geometry_intent_covers_transport_shock_recovery_and_routing() {
        assert_eq!(CASSETTE_SLOTS, 2);
        assert_eq!(SHOCK_COUPONS, 12);
        assert_eq!(LOGGER_POCKETS, 6);
        assert_eq!(WITNESS_WELLS, 10);
        assert_eq!(DWELL_LANES, 3);
        assert_eq!(DWELL_TOKEN_SLOTS, 15);
        assert_eq!(COMPARISON_BLOCKS, 3);
        assert_eq!(ROUTE_LANE_NAMES, ["release", "hold", "reject"]);
        assert!(FRONT_ROBOT_CLEARANCE_Y > CASSETTE_SLOT_Y);
        assert!(REAR_SERVICE_CLEARANCE_Y > ROUTE_SLOT_Y);
    }

    #[test]
    fn module_footprints_fit_and_do_not_overlap() {
        assert_layout();
    }

    #[test]
    fn assembly_export_is_last_output() {
        assert_eq!(
            OUTPUTS.last().copied(),
            Some(
                "output/closed_cassette_transport_temperature_shock_recovery_station_assembly.stl"
            )
        );
    }
}
