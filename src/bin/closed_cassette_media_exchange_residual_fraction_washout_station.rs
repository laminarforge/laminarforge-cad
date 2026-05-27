use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette media-exchange residual fraction and washout validation station.
//
// Intent:
// - Package a no-cell engineering fixture for validating closed cassette media
//   exchange mechanics: datum seating, inlet/outlet routing, residual dye
//   witness capture, staged washout fraction collection, timed exchange tokens,
//   bubble/wetness visibility, pressure/flow coupon pockets, segregated flush
//   and waste routing, traceable status lands, evidence capture, and robot plus
//   service keepouts.
// - Keep all geometry deterministic and self-describing with CSG plaques,
//   raised route arrows, witness frames, and token rails.
//
// This is mechanical validation CAD only. It is not a biological protocol,
// wetted-path specification, sterile barrier claim, or acceptance threshold.

const PREFIX: &str = "closed_cassette_media_exchange_residual_fraction_washout_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_base_containment_deck.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_sealed_cassette_datum_nest.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_inlet_outlet_route_combs.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_residual_volume_dye_witness_wells.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_washout_fraction_collection_nests.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_timed_exchange_token_rail.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_bubble_wetness_windows.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_pressure_flow_sensor_coupon_pockets.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_waste_flush_segregation_manifold.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_barcode_status_lands.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_evidence_bridge.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_robot_service_keepouts.stl",
    "output/closed_cassette_media_exchange_residual_fraction_washout_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_containment_deck",
    "sealed_cassette_datum_nest",
    "inlet_outlet_route_combs",
    "residual_volume_dye_witness_wells",
    "washout_fraction_collection_nests",
    "timed_exchange_token_rail",
    "bubble_wetness_windows",
    "pressure_flow_sensor_coupon_pockets",
    "waste_flush_segregation_manifold",
    "barcode_status_lands",
    "evidence_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 840.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.8;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_WITNESS_RAILS: usize = 7;
const DATUM_FIDUCIALS: usize = 4;

const CASSETTE_POS: (f64, f64) = (-370.0, 198.0);
const CASSETTE_X: f64 = 440.0;
const CASSETTE_Y: f64 = 250.0;
const CASSETTE_Z: f64 = 58.0;
const CASSETTE_RECESS_X: f64 = 336.0;
const CASSETTE_RECESS_Y: f64 = 164.0;
const CASSETTE_RECESS_Z: f64 = 28.0;
const CASSETTE_DATUM_PINS: usize = 6;
const CASSETTE_LATCHES: usize = 4;
const CASSETTE_PORT_DATUMS: usize = 4;

const ROUTE_POS: (f64, f64) = (215.0, 198.0);
const ROUTE_X: f64 = 560.0;
const ROUTE_Y: f64 = 250.0;
const ROUTE_Z: f64 = 46.0;
const ROUTE_CHANNELS: usize = 8;
const ROUTE_PITCH_X: f64 = 58.0;
const ROUTE_CHANNEL_W: f64 = 8.0;
const ROUTE_CHANNEL_Z: f64 = 12.0;
const ROUTE_COMB_TEETH_PER_SIDE: usize = ROUTE_CHANNELS;

const RESIDUAL_POS: (f64, f64) = (-444.0, -58.0);
const RESIDUAL_X: f64 = 374.0;
const RESIDUAL_Y: f64 = 184.0;
const RESIDUAL_Z: f64 = 36.0;
const RESIDUAL_ROWS: usize = 2;
const RESIDUAL_COLS: usize = 6;
const RESIDUAL_WELLS: usize = RESIDUAL_ROWS * RESIDUAL_COLS;
const RESIDUAL_WELL_PITCH_X: f64 = 54.0;
const RESIDUAL_WELL_PITCH_Y: f64 = 64.0;
const RESIDUAL_WELL_D: f64 = 19.0;
const RESIDUAL_DYE_REFERENCES: usize = 6;

const WASHOUT_POS: (f64, f64) = (2.0, -58.0);
const WASHOUT_X: f64 = 468.0;
const WASHOUT_Y: f64 = 184.0;
const WASHOUT_Z: f64 = 42.0;
const WASHOUT_FRACTIONS: usize = 10;
const WASHOUT_NEST_PITCH_X: f64 = 42.0;
const WASHOUT_NEST_D: f64 = 24.0;
const WASHOUT_SERIES_DIVIDERS: usize = 4;

const TOKEN_POS: (f64, f64) = (460.0, -58.0);
const TOKEN_X: f64 = 300.0;
const TOKEN_Y: f64 = 184.0;
const TOKEN_Z: f64 = 30.0;
const EXCHANGE_TOKENS: usize = 6;
const TOKEN_PITCH_X: f64 = 42.0;
const TOKEN_STOP_GATES: usize = 7;

const WINDOW_POS: (f64, f64) = (-444.0, -282.0);
const WINDOW_X: f64 = 374.0;
const WINDOW_Y: f64 = 158.0;
const WINDOW_Z: f64 = 34.0;
const BUBBLE_WINDOWS: usize = 4;
const WETNESS_PADS: usize = 8;
const WINDOW_PITCH_X: f64 = 72.0;

const SENSOR_POS: (f64, f64) = (-20.0, -282.0);
const SENSOR_X: f64 = 420.0;
const SENSOR_Y: f64 = 158.0;
const SENSOR_Z: f64 = 44.0;
const SENSOR_COUPONS: usize = 8;
const SENSOR_PITCH_X: f64 = 48.0;
const PRESSURE_POCKET_X: f64 = 30.0;
const FLOW_POCKET_X: f64 = 38.0;

const SEGREGATION_POS: (f64, f64) = (418.0, -282.0);
const SEGREGATION_X: f64 = 314.0;
const SEGREGATION_Y: f64 = 158.0;
const SEGREGATION_Z: f64 = 56.0;
const FLUSH_PORTS: usize = 6;
const WASTE_PORTS: usize = 6;
const SEGREGATION_BARRIER_Z: f64 = 86.0;

const TRACE_POS: (f64, f64) = (0.0, 342.0);
const TRACE_X: f64 = 610.0;
const TRACE_Y: f64 = 92.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const CERTIFICATE_LANDS: usize = 3;

const EVIDENCE_POS: (f64, f64) = (0.0, 48.0);
const EVIDENCE_X: f64 = 1140.0;
const EVIDENCE_Y: f64 = 60.0;
const EVIDENCE_POST_Z: f64 = 214.0;
const EVIDENCE_POST_X: f64 = 30.0;
const EVIDENCE_POST_Y: f64 = 46.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CAMERAS: usize = 4;
const EVIDENCE_LIGHT_BARS: usize = 3;

const ROBOT_KEEPOUT_X: f64 = 1020.0;
const ROBOT_KEEPOUT_Y: f64 = 598.0;
const ROBOT_KEEPOUT_Z: f64 = 168.0;
const FRONT_ROBOT_CLEARANCE: f64 = 382.0;
const REAR_SERVICE_CLEARANCE: f64 = 244.0;
const LEFT_CASSETTE_SERVICE_CLEARANCE: f64 = 208.0;
const RIGHT_MANIFOLD_SERVICE_CLEARANCE: f64 = 212.0;
const CASSETTE_LIFT_CLEARANCE_Z: f64 = 288.0;
const KEEP_OUT_RAIL: f64 = 7.0;
const LABEL_BAR_COUNT: usize = 8;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let cassette = sealed_cassette_datum_nest();
    export(OUTPUTS[1], &cassette);

    let routes = inlet_outlet_route_combs();
    export(OUTPUTS[2], &routes);

    let residuals = residual_volume_dye_witness_wells();
    export(OUTPUTS[3], &residuals);

    let washout = washout_fraction_collection_nests();
    export(OUTPUTS[4], &washout);

    let tokens = timed_exchange_token_rail();
    export(OUTPUTS[5], &tokens);

    let windows = bubble_wetness_windows();
    export(OUTPUTS[6], &windows);

    let sensors = pressure_flow_sensor_coupon_pockets();
    export(OUTPUTS[7], &sensors);

    let segregation = waste_flush_segregation_manifold();
    export(OUTPUTS[8], &segregation);

    let trace = barcode_status_lands();
    export(OUTPUTS[9], &trace);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + cassette
        + routes
        + residuals
        + washout
        + tokens
        + windows
        + sensors
        + segregation
        + trace
        + evidence
        + keepouts
        + global_route_labels();
    export(OUTPUTS[12], &assembly);

    println!(
        "Closed cassette media-exchange residual/washout station: {:.0}mm x {:.0}mm contained deck, {} cassette datum pins, {} inlet/outlet route channels, {} residual dye witness wells, and {} washout fraction nests.",
        STATION_X,
        STATION_Y,
        CASSETTE_DATUM_PINS,
        ROUTE_CHANNELS,
        RESIDUAL_WELLS,
        WASHOUT_FRACTIONS
    );
    println!(
        "Validation evidence: {} timed exchange tokens, {} bubble windows, {} wetness pads, {} pressure/flow coupon pairs, {} flush/waste ports, {} barcode/status/certificate lands, {} cameras, and {} light bars.",
        EXCHANGE_TOKENS,
        BUBBLE_WINDOWS,
        WETNESS_PADS,
        SENSOR_COUPONS,
        FLUSH_PORTS + WASTE_PORTS,
        BARCODE_LANDS + STATUS_LANES * STATUS_SLOTS_PER_LANE + CERTIFICATE_LANDS,
        EVIDENCE_CAMERAS,
        EVIDENCE_LIGHT_BARS
    );
    println!(
        "Keepouts: front robot clearance {:.0}mm, rear service clearance {:.0}mm, cassette lift clearance {:.0}mm, and {} required feature groups.",
        FRONT_ROBOT_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        CASSETTE_LIFT_CLEARANCE_Z,
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(format!("{PREFIX}_base_deck"), STATION_X, STATION_Y, BASE_Z);
    let basin = centered_cube(
        format!("{PREFIX}_base_shallow_spill_basin"),
        STATION_X - 118.0,
        STATION_Y - 110.0,
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 4.0);
    let wet_exchange_recess = centered_cube(
        format!("{PREFIX}_base_cassette_route_wet_zone_recess"),
        1080.0,
        250.0,
        8.0,
    )
    .translate(-50.0, 198.0, BASE_Z / 2.0 - 4.2);
    let evidence_recess = centered_cube(
        format!("{PREFIX}_base_fraction_witness_zone_recess"),
        1130.0,
        185.0,
        8.0,
    )
    .translate(0.0, -58.0, BASE_Z / 2.0 - 4.2);
    let lower_recess = centered_cube(
        format!("{PREFIX}_base_sensor_waste_zone_recess"),
        1130.0,
        158.0,
        8.0,
    )
    .translate(0.0, -282.0, BASE_Z / 2.0 - 4.2);
    let drain = centered_cylinder(
        format!("{PREFIX}_base_front_low_point_drain"),
        DRAIN_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 90.0, -STATION_Y / 2.0 - 3.0, -1.0);

    deck - basin
        - wet_exchange_recess
        - evidence_recess
        - lower_recess
        - drain
        - insert_sockets()
        - mounting_slots()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_rails()
        + deck_flow_gutters()
        + station_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_insert_registration_sockets"));
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{name}_socket_recess"),
                width + 10.0,
                depth + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_mounting_slots"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 52.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (0.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_bore_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("{PREFIX}_m6_mount_slot_relief_{i}"),
                26.0,
                MOUNT_HOLE_D + 0.6,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z_on_base(RIM_Z));
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z_on_base(RIM_Z));
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z_on_base(RIM_Z));
    let front_low_lip = centered_cube(
        format!("{PREFIX}_front_low_waste_lip"),
        STATION_X - 174.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, z_on_base(22.0));

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let exchange_to_fraction = centered_cube(
        format!("{PREFIX}_cassette_exchange_to_fraction_zone_divider"),
        STATION_X - 174.0,
        12.0,
        30.0,
    )
    .translate(0.0, 68.0, z_on_base(30.0));
    let fraction_to_sensor = centered_cube(
        format!("{PREFIX}_fraction_collection_to_sensor_zone_divider"),
        STATION_X - 174.0,
        12.0,
        30.0,
    )
    .translate(0.0, -174.0, z_on_base(30.0));
    let cassette_to_route = centered_cube(
        format!("{PREFIX}_cassette_route_keepaway_rib"),
        10.0,
        238.0,
        28.0,
    )
    .translate(-84.0, 198.0, z_on_base(28.0));
    let residual_to_washout = centered_cube(
        format!("{PREFIX}_residual_witness_to_washout_divider"),
        10.0,
        178.0,
        28.0,
    )
    .translate(-222.0, -58.0, z_on_base(28.0));
    let washout_to_tokens = centered_cube(
        format!("{PREFIX}_washout_fraction_to_token_divider"),
        10.0,
        178.0,
        28.0,
    )
    .translate(238.0, -58.0, z_on_base(28.0));
    let sensor_to_waste = centered_cube(
        format!("{PREFIX}_sensor_coupon_to_waste_flush_divider"),
        10.0,
        154.0,
        28.0,
    )
    .translate(204.0, -282.0, z_on_base(28.0));

    exchange_to_fraction
        + fraction_to_sensor
        + cassette_to_route
        + residual_to_washout
        + washout_to_tokens
        + sensor_to_waste
}

fn leak_witness_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_base_leak_witness_rails"));
    for i in 0..LEAK_WITNESS_RAILS {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_front_leak_witness_rail_{i}"),
                112.0,
                5.0,
                7.0,
            )
            .translate(
                centered_index(i, LEAK_WITNESS_RAILS, 146.0),
                -STATION_Y / 2.0 + 62.0,
                BASE_Z / 2.0 + 3.5,
            );
    }
    rails
}

fn deck_flow_gutters() -> Part {
    let cassette_to_fraction = centered_cube(
        format!("{PREFIX}_base_cassette_to_residual_gutter"),
        8.0,
        312.0,
        6.0,
    )
    .translate(-218.0, 50.0, BASE_Z / 2.0 + 3.0);
    let washout_to_waste = centered_cube(
        format!("{PREFIX}_base_washout_to_waste_gutter"),
        8.0,
        336.0,
        6.0,
    )
    .translate(230.0, -130.0, BASE_Z / 2.0 + 3.0);
    let sensor_to_waste = centered_cube(
        format!("{PREFIX}_base_sensor_coupon_to_waste_gutter"),
        356.0,
        8.0,
        6.0,
    )
    .translate(212.0, -174.0, BASE_Z / 2.0 + 3.0);
    cassette_to_fraction + washout_to_waste + sensor_to_waste
}

fn station_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_robot_datum_fiducials"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 82.0), STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 82.0),
        (-(STATION_X / 2.0 - 82.0), -(STATION_Y / 2.0 - 82.0)),
        (STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 82.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(format!("{PREFIX}_datum_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn sealed_cassette_datum_nest() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_sealed_cassette_datum_nest_block"),
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    )
    .translate(CASSETTE_POS.0, CASSETTE_POS.1, z_on_base(CASSETTE_Z));
    let recess = centered_cube(
        format!("{PREFIX}_sealed_cassette_surrogate_recess"),
        CASSETTE_RECESS_X,
        CASSETTE_RECESS_Y,
        CASSETTE_RECESS_Z,
    )
    .translate(
        CASSETTE_POS.0,
        CASSETTE_POS.1,
        BASE_Z / 2.0 + CASSETTE_Z - CASSETTE_RECESS_Z / 2.0 + 0.4,
    );
    let port_clearance = cassette_port_clearance_bores();

    block - recess - port_clearance + cassette_datum_pin_bosses() + cassette_latches()
}

fn cassette_datum_pin_bosses() -> Part {
    let mut pins = Part::empty(format!("{PREFIX}_cassette_datum_pin_bosses"));
    for i in 0..CASSETTE_DATUM_PINS {
        let x = CASSETTE_POS.0 + centered_index(i % 3, 3, 128.0);
        let y = CASSETTE_POS.1 + centered_index(i / 3, 2, 118.0);
        let boss = centered_cylinder(format!("{PREFIX}_cassette_datum_boss_{i}"), 10.0, 9.0, 32)
            .translate(x, y, BASE_Z / 2.0 + CASSETTE_Z + 4.5);
        let bore = centered_cylinder(
            format!("{PREFIX}_cassette_datum_center_bore_{i}"),
            3.0,
            10.0,
            20,
        )
        .translate(x, y, BASE_Z / 2.0 + CASSETTE_Z + 4.5);
        pins = pins + (boss - bore);
    }
    pins
}

fn cassette_port_clearance_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_cassette_port_clearance_bores"));
    for i in 0..CASSETTE_PORT_DATUMS {
        let x = CASSETTE_POS.0 + centered_index(i, CASSETTE_PORT_DATUMS, 78.0);
        for (side, y) in [
            ("inlet", CASSETTE_POS.1 - CASSETTE_RECESS_Y / 2.0 - 18.0),
            ("outlet", CASSETTE_POS.1 + CASSETTE_RECESS_Y / 2.0 + 18.0),
        ] {
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_cassette_{side}_port_clearance_{i}"),
                    4.4,
                    44.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, BASE_Z / 2.0 + 26.0);
        }
    }
    bores
}

fn cassette_latches() -> Part {
    let mut latches = Part::empty(format!("{PREFIX}_cassette_quarter_turn_latches"));
    for i in 0..CASSETTE_LATCHES {
        let x = CASSETTE_POS.0 + centered_index(i % 2, 2, CASSETTE_RECESS_X + 34.0);
        let y = CASSETTE_POS.1 + centered_index(i / 2, 2, CASSETTE_RECESS_Y + 34.0);
        latches = latches
            + centered_cube(
                format!("{PREFIX}_cassette_latch_bridge_{i}"),
                54.0,
                12.0,
                14.0,
            )
            .rotate(0.0, 0.0, if i < 2 { 0.0 } else { 90.0 })
            .translate(x, y, BASE_Z / 2.0 + CASSETTE_Z + 7.0);
    }
    latches
}

fn inlet_outlet_route_combs() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_inlet_outlet_route_comb_plate"),
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    )
    .translate(ROUTE_POS.0, ROUTE_POS.1, z_on_base(ROUTE_Z));
    let underside = centered_cube(
        format!("{PREFIX}_route_comb_underside_lightening_pocket"),
        ROUTE_X - 56.0,
        ROUTE_Y - 48.0,
        16.0,
    )
    .translate(ROUTE_POS.0, ROUTE_POS.1, BASE_Z / 2.0 + 8.0);

    plate - underside - route_channel_recesses()
        + route_comb_teeth()
        + route_bridge_clips()
        + route_flow_arrows()
}

fn route_channel_recesses() -> Part {
    let mut channels = Part::empty(format!("{PREFIX}_route_channel_recesses"));
    for i in 0..ROUTE_CHANNELS {
        let x = ROUTE_POS.0 + centered_index(i, ROUTE_CHANNELS, ROUTE_PITCH_X);
        let inlet = centered_cube(
            format!("{PREFIX}_inlet_channel_recess_{i}"),
            ROUTE_CHANNEL_W,
            ROUTE_Y - 70.0,
            ROUTE_CHANNEL_Z,
        )
        .translate(
            x,
            ROUTE_POS.1,
            BASE_Z / 2.0 + ROUTE_Z - ROUTE_CHANNEL_Z / 2.0 + 0.3,
        );
        let outlet = centered_cube(
            format!("{PREFIX}_outlet_channel_cross_recess_{i}"),
            38.0,
            ROUTE_CHANNEL_W,
            ROUTE_CHANNEL_Z,
        )
        .translate(
            x + 18.0,
            ROUTE_POS.1 + ROUTE_Y / 2.0 - 50.0,
            BASE_Z / 2.0 + ROUTE_Z - ROUTE_CHANNEL_Z / 2.0 + 0.3,
        );
        let residual_pickoff = centered_cylinder(
            format!("{PREFIX}_route_residual_pickoff_pool_{i}"),
            10.0,
            ROUTE_CHANNEL_Z + 2.0,
            28,
        )
        .translate(
            x,
            ROUTE_POS.1 - ROUTE_Y / 2.0 + 48.0,
            BASE_Z / 2.0 + ROUTE_Z - ROUTE_CHANNEL_Z / 2.0,
        );
        channels = channels + inlet + outlet + residual_pickoff;
    }
    channels
}

fn route_comb_teeth() -> Part {
    let mut teeth = Part::empty(format!("{PREFIX}_route_comb_teeth"));
    for i in 0..ROUTE_COMB_TEETH_PER_SIDE {
        let x = ROUTE_POS.0 + centered_index(i, ROUTE_COMB_TEETH_PER_SIDE, ROUTE_PITCH_X);
        teeth = teeth
            + centered_cube(format!("{PREFIX}_inlet_comb_tooth_{i}"), 18.0, 10.0, 14.0).translate(
                x,
                ROUTE_POS.1 - ROUTE_Y / 2.0 + 22.0,
                BASE_Z / 2.0 + ROUTE_Z + 7.0,
            )
            + centered_cube(format!("{PREFIX}_outlet_comb_tooth_{i}"), 18.0, 10.0, 14.0).translate(
                x,
                ROUTE_POS.1 + ROUTE_Y / 2.0 - 22.0,
                BASE_Z / 2.0 + ROUTE_Z + 7.0,
            );
    }
    teeth
}

fn route_bridge_clips() -> Part {
    let mut clips = Part::empty(format!("{PREFIX}_route_bridge_clips"));
    for i in 0..4 {
        clips = clips
            + centered_cube(
                format!("{PREFIX}_route_bridge_retainer_clip_{i}"),
                72.0,
                9.0,
                12.0,
            )
            .translate(
                ROUTE_POS.0 + centered_index(i, 4, 122.0),
                ROUTE_POS.1,
                BASE_Z / 2.0 + ROUTE_Z + 6.0,
            );
    }
    clips
}

fn route_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_route_flow_arrows"));
    for i in 0..ROUTE_CHANNELS {
        arrows = arrows
            + flow_arrow(format!("{PREFIX}_route_flow_arrow_{i}"), 30.0, 16.0, 5.0)
                .rotate(0.0, 0.0, 90.0)
                .translate(
                    ROUTE_POS.0 + centered_index(i, ROUTE_CHANNELS, ROUTE_PITCH_X),
                    ROUTE_POS.1,
                    BASE_Z / 2.0 + ROUTE_Z + 2.5,
                );
    }
    arrows
}

fn residual_volume_dye_witness_wells() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_residual_dye_witness_well_bank"),
        RESIDUAL_X,
        RESIDUAL_Y,
        RESIDUAL_Z,
    )
    .translate(RESIDUAL_POS.0, RESIDUAL_POS.1, z_on_base(RESIDUAL_Z));
    let mut cutters = Part::empty(format!("{PREFIX}_residual_dye_well_cutters"));
    let mut features = Part::empty(format!("{PREFIX}_residual_dye_well_features"));

    for row in 0..RESIDUAL_ROWS {
        for col in 0..RESIDUAL_COLS {
            let idx = row * RESIDUAL_COLS + col;
            let x = RESIDUAL_POS.0 + centered_index(col, RESIDUAL_COLS, RESIDUAL_WELL_PITCH_X);
            let y = RESIDUAL_POS.1 + centered_index(row, RESIDUAL_ROWS, RESIDUAL_WELL_PITCH_Y);
            let well_d = RESIDUAL_WELL_D + idx as f64 * 0.9;
            let well = centered_cylinder(
                format!("{PREFIX}_residual_volume_dye_witness_well_{idx}"),
                well_d / 2.0,
                RESIDUAL_Z + 6.0,
                32,
            )
            .translate(x, y, z_on_base(RESIDUAL_Z));
            let rim = centered_cylinder(
                format!("{PREFIX}_residual_volume_dye_witness_rim_{idx}"),
                well_d / 2.0 + 4.0,
                4.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0 + RESIDUAL_Z + 2.0);
            let rim_cut = centered_cylinder(
                format!("{PREFIX}_residual_volume_dye_witness_rim_open_{idx}"),
                well_d / 2.0,
                5.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0 + RESIDUAL_Z + 2.0);
            let dye_flag = centered_cube(
                format!("{PREFIX}_residual_dye_intensity_flag_{idx}"),
                7.0 + idx as f64,
                8.0,
                5.0,
            )
            .translate(
                x,
                y + RESIDUAL_WELL_PITCH_Y / 2.0 - 11.0,
                BASE_Z / 2.0 + RESIDUAL_Z + 2.5,
            );
            cutters = cutters + well;
            features = features + (rim - rim_cut) + dye_flag;
        }
    }

    block - cutters + features + residual_reference_swatches()
}

fn residual_reference_swatches() -> Part {
    let mut swatches = Part::empty(format!("{PREFIX}_residual_dye_reference_swatches"));
    for i in 0..RESIDUAL_DYE_REFERENCES {
        swatches = swatches
            + centered_cube(
                format!("{PREFIX}_residual_dye_reference_swatch_{i}"),
                30.0,
                12.0,
                4.0,
            )
            .translate(
                RESIDUAL_POS.0 + centered_index(i, RESIDUAL_DYE_REFERENCES, 46.0),
                RESIDUAL_POS.1 - RESIDUAL_Y / 2.0 + 16.0,
                BASE_Z / 2.0 + RESIDUAL_Z + 2.0,
            );
    }
    swatches
}

fn washout_fraction_collection_nests() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_washout_fraction_collection_nest_bank"),
        WASHOUT_X,
        WASHOUT_Y,
        WASHOUT_Z,
    )
    .translate(WASHOUT_POS.0, WASHOUT_POS.1, z_on_base(WASHOUT_Z));
    let mut cutters = Part::empty(format!("{PREFIX}_washout_fraction_nest_cutters"));
    let mut features = Part::empty(format!("{PREFIX}_washout_fraction_nest_features"));

    for i in 0..WASHOUT_FRACTIONS {
        let x = WASHOUT_POS.0 + centered_index(i, WASHOUT_FRACTIONS, WASHOUT_NEST_PITCH_X);
        let nest = centered_cylinder(
            format!("{PREFIX}_washout_fraction_collection_nest_{i}"),
            WASHOUT_NEST_D / 2.0,
            WASHOUT_Z + 8.0,
            36,
        )
        .translate(x, WASHOUT_POS.1, z_on_base(WASHOUT_Z));
        let collar = centered_cylinder(
            format!("{PREFIX}_washout_fraction_collection_collar_{i}"),
            WASHOUT_NEST_D / 2.0 + 5.0,
            5.0,
            36,
        )
        .translate(x, WASHOUT_POS.1, BASE_Z / 2.0 + WASHOUT_Z + 2.5);
        let collar_cut = centered_cylinder(
            format!("{PREFIX}_washout_fraction_collection_collar_open_{i}"),
            WASHOUT_NEST_D / 2.0,
            6.0,
            36,
        )
        .translate(x, WASHOUT_POS.1, BASE_Z / 2.0 + WASHOUT_Z + 2.5);
        let sequence_tab = centered_cube(
            format!("{PREFIX}_washout_fraction_sequence_tab_{i}"),
            18.0,
            8.0,
            7.0,
        )
        .translate(
            x,
            WASHOUT_POS.1 + WASHOUT_Y / 2.0 - 18.0,
            BASE_Z / 2.0 + WASHOUT_Z + 3.5,
        );
        cutters = cutters + nest;
        features = features + (collar - collar_cut) + sequence_tab;
    }

    block - cutters + features + washout_series_dividers() + washout_flow_header()
}

fn washout_series_dividers() -> Part {
    let mut dividers = Part::empty(format!("{PREFIX}_washout_fraction_series_dividers"));
    for i in 0..WASHOUT_SERIES_DIVIDERS {
        dividers = dividers
            + centered_cube(
                format!("{PREFIX}_washout_series_divider_{i}"),
                7.0,
                WASHOUT_Y - 28.0,
                16.0,
            )
            .translate(
                WASHOUT_POS.0 + centered_index(i, WASHOUT_SERIES_DIVIDERS, 104.0),
                WASHOUT_POS.1,
                BASE_Z / 2.0 + WASHOUT_Z + 8.0,
            );
    }
    dividers
}

fn washout_flow_header() -> Part {
    let header = centered_cylinder(
        format!("{PREFIX}_washout_fraction_closed_route_header"),
        5.0,
        WASHOUT_X - 64.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        WASHOUT_POS.0,
        WASHOUT_POS.1 - WASHOUT_Y / 2.0 + 34.0,
        BASE_Z / 2.0 + WASHOUT_Z + 10.0,
    );
    let drain_stub = centered_cylinder(
        format!("{PREFIX}_washout_fraction_to_waste_stub"),
        7.0,
        70.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        WASHOUT_POS.0 + WASHOUT_X / 2.0 + 28.0,
        WASHOUT_POS.1 - WASHOUT_Y / 2.0 + 34.0,
        BASE_Z / 2.0 + WASHOUT_Z + 10.0,
    );
    header + drain_stub
}

fn timed_exchange_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_timed_exchange_token_rail_base"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, z_on_base(TOKEN_Z));
    let track_cut = centered_cube(
        format!("{PREFIX}_timed_exchange_token_sliding_track_recess"),
        TOKEN_X - 44.0,
        36.0,
        12.0,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z / 2.0 + TOKEN_Z - 6.0 + 0.3);

    rail - track_cut + exchange_tokens() + token_stop_gates()
}

fn exchange_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_timed_exchange_tokens"));
    for i in 0..EXCHANGE_TOKENS {
        let token = if i % 2 == 0 {
            centered_cylinder(
                format!("{PREFIX}_round_timed_exchange_token_{i}"),
                15.0,
                8.0,
                32,
            )
        } else {
            centered_cube(
                format!("{PREFIX}_rect_timed_exchange_token_{i}"),
                28.0,
                22.0,
                8.0,
            )
        };
        tokens = tokens
            + token.translate(
                TOKEN_POS.0 + centered_index(i, EXCHANGE_TOKENS, TOKEN_PITCH_X),
                TOKEN_POS.1,
                BASE_Z / 2.0 + TOKEN_Z + 4.0,
            )
            + centered_cube(
                format!("{PREFIX}_timed_exchange_token_tick_{i}"),
                4.0,
                TOKEN_Y - 30.0,
                5.0,
            )
            .translate(
                TOKEN_POS.0 + centered_index(i, EXCHANGE_TOKENS, TOKEN_PITCH_X),
                TOKEN_POS.1,
                BASE_Z / 2.0 + TOKEN_Z + 2.5,
            );
    }
    tokens
}

fn token_stop_gates() -> Part {
    let mut gates = Part::empty(format!("{PREFIX}_timed_exchange_token_stop_gates"));
    for i in 0..TOKEN_STOP_GATES {
        gates = gates
            + centered_cube(format!("{PREFIX}_token_stop_gate_{i}"), 6.0, 56.0, 16.0).translate(
                TOKEN_POS.0 + centered_index(i, TOKEN_STOP_GATES, TOKEN_PITCH_X),
                TOKEN_POS.1,
                BASE_Z / 2.0 + TOKEN_Z + 8.0,
            );
    }
    gates
}

fn bubble_wetness_windows() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_bubble_wetness_window_panel"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(WINDOW_POS.0, WINDOW_POS.1, z_on_base(WINDOW_Z));
    let mut cutouts = Part::empty(format!("{PREFIX}_bubble_window_cutouts"));
    let mut features = Part::empty(format!("{PREFIX}_bubble_wetness_window_features"));

    for i in 0..BUBBLE_WINDOWS {
        let x = WINDOW_POS.0 + centered_index(i, BUBBLE_WINDOWS, WINDOW_PITCH_X);
        let frame = centered_cube(
            format!("{PREFIX}_bubble_window_frame_{i}"),
            46.0,
            32.0,
            18.0,
        )
        .translate(x, WINDOW_POS.1 + 22.0, BASE_Z / 2.0 + WINDOW_Z + 9.0);
        let opening = centered_cube(
            format!("{PREFIX}_bubble_window_transparent_insert_opening_{i}"),
            34.0,
            20.0,
            20.0,
        )
        .translate(x, WINDOW_POS.1 + 22.0, BASE_Z / 2.0 + WINDOW_Z + 9.0);
        let frame_opening = centered_cube(
            format!("{PREFIX}_bubble_window_frame_opening_{i}"),
            34.0,
            20.0,
            20.0,
        )
        .translate(x, WINDOW_POS.1 + 22.0, BASE_Z / 2.0 + WINDOW_Z + 9.0);
        let bubble_dome = centered_cylinder(
            format!("{PREFIX}_bubble_window_dome_outline_{i}"),
            12.0,
            5.0,
            32,
        )
        .translate(x, WINDOW_POS.1 - 28.0, BASE_Z / 2.0 + WINDOW_Z + 2.5);
        cutouts = cutouts + opening;
        features = features + (frame - frame_opening) + bubble_dome;
    }

    for i in 0..WETNESS_PADS {
        features = features
            + centered_cube(
                format!("{PREFIX}_wetness_indicator_pad_land_{i}"),
                24.0,
                14.0,
                4.0,
            )
            .translate(
                WINDOW_POS.0 + centered_index(i, WETNESS_PADS, 42.0),
                WINDOW_POS.1 - WINDOW_Y / 2.0 + 18.0,
                BASE_Z / 2.0 + WINDOW_Z + 2.0,
            );
    }

    panel - cutouts + features
}

fn pressure_flow_sensor_coupon_pockets() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_pressure_flow_sensor_coupon_block"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, z_on_base(SENSOR_Z));
    let mut cutters = Part::empty(format!("{PREFIX}_pressure_flow_coupon_pocket_cutters"));
    let mut features = Part::empty(format!("{PREFIX}_pressure_flow_coupon_features"));

    for i in 0..SENSOR_COUPONS {
        let x = SENSOR_POS.0 + centered_index(i, SENSOR_COUPONS, SENSOR_PITCH_X);
        let pressure = centered_cube(
            format!("{PREFIX}_pressure_sensor_coupon_pocket_{i}"),
            PRESSURE_POCKET_X,
            28.0,
            18.0,
        )
        .translate(x, SENSOR_POS.1 + 28.0, BASE_Z / 2.0 + SENSOR_Z - 9.0 + 0.3);
        let flow = centered_cube(
            format!("{PREFIX}_flow_sensor_coupon_pocket_{i}"),
            FLOW_POCKET_X,
            24.0,
            18.0,
        )
        .translate(x, SENSOR_POS.1 - 28.0, BASE_Z / 2.0 + SENSOR_Z - 9.0 + 0.3);
        let route_bore = centered_cylinder(
            format!("{PREFIX}_sensor_coupon_route_bore_{i}"),
            3.4,
            72.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SENSOR_POS.1, BASE_Z / 2.0 + SENSOR_Z - 14.0);
        let retaining_clip = centered_cube(
            format!("{PREFIX}_sensor_coupon_retaining_clip_{i}"),
            38.0,
            7.0,
            10.0,
        )
        .translate(x, SENSOR_POS.1, BASE_Z / 2.0 + SENSOR_Z + 5.0);
        cutters = cutters + pressure + flow + route_bore;
        features = features + retaining_clip;
    }

    block - cutters + features + sensor_cable_strain_reliefs()
}

fn sensor_cable_strain_reliefs() -> Part {
    let mut reliefs = Part::empty(format!("{PREFIX}_sensor_cable_strain_reliefs"));
    for i in 0..4 {
        reliefs = reliefs
            + centered_cube(format!("{PREFIX}_sensor_cable_clip_{i}"), 52.0, 8.0, 10.0).translate(
                SENSOR_POS.0 + centered_index(i, 4, 82.0),
                SENSOR_POS.1 + SENSOR_Y / 2.0 - 16.0,
                BASE_Z / 2.0 + SENSOR_Z + 5.0,
            );
    }
    reliefs
}

fn waste_flush_segregation_manifold() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_waste_flush_segregation_manifold_block"),
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1,
        z_on_base(SEGREGATION_Z),
    );
    let mut cutters = Part::empty(format!("{PREFIX}_waste_flush_port_cutters"));
    let mut features = Part::empty(format!("{PREFIX}_waste_flush_features"));

    for i in 0..FLUSH_PORTS {
        let x = SEGREGATION_POS.0 + centered_index(i, FLUSH_PORTS, 42.0);
        cutters = cutters
            + centered_cylinder(format!("{PREFIX}_flush_inlet_port_bore_{i}"), 4.4, 64.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x,
                    SEGREGATION_POS.1 + 34.0,
                    BASE_Z / 2.0 + SEGREGATION_Z - 18.0,
                )
            + centered_cylinder(
                format!("{PREFIX}_waste_outlet_port_bore_{i}"),
                6.2,
                64.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                SEGREGATION_POS.1 - 34.0,
                BASE_Z / 2.0 + SEGREGATION_Z - 18.0,
            );
        features = features
            + centered_cylinder(
                format!("{PREFIX}_flush_quickconnect_boss_{i}"),
                8.0,
                7.0,
                28,
            )
            .translate(
                x,
                SEGREGATION_POS.1 + 34.0,
                BASE_Z / 2.0 + SEGREGATION_Z + 3.5,
            )
            + centered_cylinder(
                format!("{PREFIX}_waste_quickconnect_boss_{i}"),
                11.0,
                7.0,
                28,
            )
            .translate(
                x,
                SEGREGATION_POS.1 - 34.0,
                BASE_Z / 2.0 + SEGREGATION_Z + 3.5,
            );
    }

    let barrier = centered_cube(
        format!("{PREFIX}_flush_waste_centerline_segregation_barrier"),
        SEGREGATION_X - 42.0,
        10.0,
        SEGREGATION_BARRIER_Z,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1,
        BASE_Z / 2.0 + SEGREGATION_BARRIER_Z / 2.0,
    );
    let trough = centered_cube(
        format!("{PREFIX}_segregated_waste_trough"),
        SEGREGATION_X - 54.0,
        28.0,
        20.0,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1 - SEGREGATION_Y / 2.0 + 24.0,
        BASE_Z / 2.0 + SEGREGATION_Z + 10.0,
    );
    let trough_opening = centered_cube(
        format!("{PREFIX}_segregated_waste_trough_open_recess"),
        SEGREGATION_X - 78.0,
        16.0,
        22.0,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1 - SEGREGATION_Y / 2.0 + 24.0,
        BASE_Z / 2.0 + SEGREGATION_Z + 11.0,
    );

    block - cutters + features + barrier + (trough - trough_opening)
}

fn barcode_status_lands() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_status_land_plate"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, z_on_base(TRACE_Z));
    plate + barcode_lands() + status_lanes() + certificate_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_cassette_fraction_barcode_land_{i}"),
                48.0,
                15.0,
                4.0,
                i,
            )
            .translate(
                TRACE_POS.0 - TRACE_X / 2.0 + 44.0 + i as f64 * 56.0,
                TRACE_POS.1 + TRACE_Y / 2.0 - 22.0,
                BASE_Z / 2.0 + TRACE_Z + 2.0,
            );
    }
    lands
}

fn status_lanes() -> Part {
    let mut lanes = Part::empty(format!("{PREFIX}_release_hold_reject_status_lanes"));
    for lane in 0..STATUS_LANES {
        let y = TRACE_POS.1 - TRACE_Y / 2.0 + 24.0 + lane as f64 * 22.0;
        let lane_bar = centered_cube(format!("{PREFIX}_status_lane_bar_{lane}"), 188.0, 14.0, 5.0)
            .translate(TRACE_POS.0 + 126.0, y, BASE_Z / 2.0 + TRACE_Z + 2.5);
        lanes = lanes + lane_bar;
        for slot in 0..STATUS_SLOTS_PER_LANE {
            lanes = lanes
                + centered_cube(
                    format!("{PREFIX}_status_lane_{lane}_cassette_slot_{slot}"),
                    32.0,
                    10.0,
                    7.0,
                )
                .translate(
                    TRACE_POS.0 + 60.0 + slot as f64 * 42.0,
                    y,
                    BASE_Z / 2.0 + TRACE_Z + 3.5,
                );
        }
    }
    lanes
}

fn certificate_lands() -> Part {
    let mut certs = Part::empty(format!("{PREFIX}_certificate_lands"));
    for i in 0..CERTIFICATE_LANDS {
        certs = certs
            + certificate_plaque(
                format!("{PREFIX}_washout_certificate_land_{i}"),
                68.0,
                36.0,
                4.0,
                20 + i,
            )
            .translate(
                TRACE_POS.0 + TRACE_X / 2.0 - 42.0 - i as f64 * 78.0,
                TRACE_POS.1,
                BASE_Z / 2.0 + TRACE_Z + 2.0,
            );
    }
    certs
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_left_post"),
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_right_post"),
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_evidence_bridge_camera_beam"),
        EVIDENCE_X + EVIDENCE_POST_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z / 2.0,
    );
    left_post + right_post + beam + evidence_camera_mounts() + evidence_light_bars()
}

fn evidence_camera_mounts() -> Part {
    let mut mounts = Part::empty(format!("{PREFIX}_evidence_camera_mounts"));
    for i in 0..EVIDENCE_CAMERAS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_CAMERAS, EVIDENCE_X / 5.0);
        let mount = centered_cube(
            format!("{PREFIX}_evidence_camera_mount_{i}"),
            48.0,
            28.0,
            14.0,
        )
        .translate(
            x,
            EVIDENCE_POS.1 - EVIDENCE_Y / 2.0 - 11.0,
            BASE_Z / 2.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z - 7.0,
        );
        let lens = centered_cylinder(
            format!("{PREFIX}_evidence_camera_lens_axis_{i}"),
            8.0,
            24.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            EVIDENCE_POS.1 - EVIDENCE_Y / 2.0 - 24.0,
            BASE_Z / 2.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z - 7.0,
        );
        mounts = mounts + mount + lens;
    }
    mounts
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty(format!("{PREFIX}_evidence_light_bars"));
    for i in 0..EVIDENCE_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{PREFIX}_evidence_light_bar_{i}"),
                250.0,
                10.0,
                10.0,
            )
            .translate(
                EVIDENCE_POS.0 + centered_index(i, EVIDENCE_LIGHT_BARS, 310.0),
                EVIDENCE_POS.1 + EVIDENCE_Y / 2.0 + 8.0,
                BASE_Z / 2.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z - 8.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let robot_window = keepout_box(
        format!("{PREFIX}_robot_motion_keepout_window"),
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
        (0.0, -22.0, BASE_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0),
    );
    let front = centered_cube(
        format!("{PREFIX}_front_robot_clearance_gauge"),
        STATION_X - 190.0,
        KEEP_OUT_RAIL,
        30.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        z_on_base(30.0),
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_service_clearance_gauge"),
        STATION_X - 190.0,
        KEEP_OUT_RAIL,
        30.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE,
        z_on_base(30.0),
    );
    let left = centered_cube(
        format!("{PREFIX}_left_cassette_service_clearance_gauge"),
        KEEP_OUT_RAIL,
        STATION_Y - 160.0,
        30.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_CASSETTE_SERVICE_CLEARANCE,
        0.0,
        z_on_base(30.0),
    );
    let right = centered_cube(
        format!("{PREFIX}_right_manifold_service_clearance_gauge"),
        KEEP_OUT_RAIL,
        STATION_Y - 160.0,
        30.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_MANIFOLD_SERVICE_CLEARANCE,
        0.0,
        z_on_base(30.0),
    );
    let cassette_lift = keepout_box(
        format!("{PREFIX}_cassette_vertical_lift_keepout"),
        CASSETTE_X + 80.0,
        CASSETTE_Y + 80.0,
        CASSETTE_LIFT_CLEARANCE_Z,
        (
            CASSETTE_POS.0,
            CASSETTE_POS.1,
            BASE_Z / 2.0 + CASSETTE_LIFT_CLEARANCE_Z / 2.0,
        ),
    );

    robot_window + front + rear + left + right + cassette_lift
}

fn keepout_box(name: String, x: f64, y: f64, z: f64, center: (f64, f64, f64)) -> Part {
    let front = centered_cube(
        format!("{name}_front_rail"),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(center.0, center.1 - y / 2.0, center.2 + z / 2.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, KEEP_OUT_RAIL, KEEP_OUT_RAIL)
        .translate(center.0, center.1 + y / 2.0, center.2 + z / 2.0);
    let left = centered_cube(format!("{name}_left_rail"), KEEP_OUT_RAIL, y, KEEP_OUT_RAIL)
        .translate(center.0 - x / 2.0, center.1, center.2 + z / 2.0);
    let right = centered_cube(
        format!("{name}_right_rail"),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(center.0 + x / 2.0, center.1, center.2 + z / 2.0);
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (px, py)) in [
        (center.0 - x / 2.0, center.1 - y / 2.0),
        (center.0 + x / 2.0, center.1 - y / 2.0),
        (center.0 - x / 2.0, center.1 + y / 2.0),
        (center.0 + x / 2.0, center.1 + y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{name}_corner_post_{i}"),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                z,
            )
            .translate(*px, *py, center.2);
    }
    front + rear + left + right + posts
}

fn global_route_labels() -> Part {
    let cassette_label = csg_label_plaque(
        format!("{PREFIX}_global_no_cell_cassette_datum_label"),
        156.0,
        22.0,
        4.0,
        100,
    )
    .translate(
        CASSETTE_POS.0,
        CASSETTE_POS.1 + CASSETTE_Y / 2.0 - 18.0,
        BASE_Z / 2.0 + 4.0,
    );
    let route_label = csg_label_plaque(
        format!("{PREFIX}_global_inlet_outlet_route_label"),
        156.0,
        22.0,
        4.0,
        101,
    )
    .translate(
        ROUTE_POS.0,
        ROUTE_POS.1 + ROUTE_Y / 2.0 - 18.0,
        BASE_Z / 2.0 + 4.0,
    );
    let residual_label = csg_label_plaque(
        format!("{PREFIX}_global_residual_fraction_label"),
        150.0,
        22.0,
        4.0,
        102,
    )
    .translate(
        RESIDUAL_POS.0,
        RESIDUAL_POS.1 + RESIDUAL_Y / 2.0 - 16.0,
        BASE_Z / 2.0 + 4.0,
    );
    let washout_label = csg_label_plaque(
        format!("{PREFIX}_global_washout_fraction_label"),
        154.0,
        22.0,
        4.0,
        103,
    )
    .translate(
        WASHOUT_POS.0,
        WASHOUT_POS.1 + WASHOUT_Y / 2.0 - 16.0,
        BASE_Z / 2.0 + 4.0,
    );
    let waste_label = csg_label_plaque(
        format!("{PREFIX}_global_flush_waste_seg_label"),
        158.0,
        22.0,
        4.0,
        104,
    )
    .translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1 + SEGREGATION_Y / 2.0 - 16.0,
        BASE_Z / 2.0 + 4.0,
    );
    let cassette_to_route = flow_arrow(
        format!("{PREFIX}_global_cassette_to_route_arrow"),
        48.0,
        22.0,
        5.0,
    )
    .translate(-82.0, 198.0, BASE_Z / 2.0 + 4.0);
    let route_to_washout = flow_arrow(
        format!("{PREFIX}_global_route_to_washout_arrow"),
        48.0,
        22.0,
        5.0,
    )
    .rotate(0.0, 0.0, -90.0)
    .translate(215.0, 70.0, BASE_Z / 2.0 + 4.0);
    let washout_to_waste = flow_arrow(
        format!("{PREFIX}_global_washout_to_waste_arrow"),
        48.0,
        22.0,
        5.0,
    )
    .rotate(0.0, 0.0, -90.0)
    .translate(230.0, -174.0, BASE_Z / 2.0 + 4.0);

    cassette_label
        + route_label
        + residual_label
        + washout_label
        + waste_label
        + cassette_to_route
        + route_to_washout
        + washout_to_waste
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    let disc = centered_cylinder(format!("{name}_disc"), 13.0, 4.0, 40);
    let cross_x = centered_cube(format!("{name}_cross_x"), 22.0, 3.0, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 22.0, 5.0);
    disc + cross_x + cross_y
}

fn flow_arrow(name: impl Into<String>, length: f64, width: f64, height: f64) -> Part {
    let name = name.into();
    let shaft = centered_cube(format!("{name}_shaft"), length * 0.58, width * 0.28, height)
        .translate(-length * 0.13, 0.0, 0.0);
    let head = centered_cube(format!("{name}_head"), width * 0.58, width * 0.58, height)
        .rotate(0.0, 0.0, 45.0)
        .translate(length * 0.28, 0.0, 0.0);
    shaft + head
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 2.0 + ((seed + i) % 4) as f64 * 1.5;
        let height = (y - 7.0 - (i % 3) as f64 * 2.0).max(3.0);
        let x_offset = -x / 2.0 + 9.0 + i as f64 * ((x - 20.0) / LABEL_BAR_COUNT as f64);
        bars =
            bars + centered_cube(format!("{name}_raised_bar_{i}"), width, height, z + 1.6)
                .translate(x_offset, 0.0, z / 2.0 + 0.8);
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 12.0, 4.0, z + 1.8)
        .translate(x / 2.0 - 11.0, y / 2.0 - 5.0, z / 2.0 + 0.9);
    base + bars + orientation_tab
}

fn certificate_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let barcode = csg_label_plaque(format!("{name}_barcode"), x - 12.0, 10.0, z + 0.8, seed)
        .translate(0.0, y / 2.0 - 10.0, z / 2.0 + 0.4);
    let signature_line = centered_cube(format!("{name}_signature_line"), x - 14.0, 2.5, z + 1.0)
        .translate(0.0, -y / 2.0 + 9.0, z / 2.0 + 0.5);
    let seal = centered_cylinder(
        format!("{name}_raised_circular_certificate_seal"),
        7.0,
        z + 1.2,
        30,
    )
    .translate(x / 2.0 - 12.0, -y / 2.0 + 11.0, z / 2.0 + 0.6);
    sheet + barcode + signature_line + seal
}

fn z_on_base(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "sealed_cassette_datum_nest",
            CASSETTE_POS,
            CASSETTE_X,
            CASSETTE_Y,
        ),
        ("inlet_outlet_route_combs", ROUTE_POS, ROUTE_X, ROUTE_Y),
        (
            "residual_volume_dye_witness_wells",
            RESIDUAL_POS,
            RESIDUAL_X,
            RESIDUAL_Y,
        ),
        (
            "washout_fraction_collection_nests",
            WASHOUT_POS,
            WASHOUT_X,
            WASHOUT_Y,
        ),
        ("timed_exchange_token_rail", TOKEN_POS, TOKEN_X, TOKEN_Y),
        ("bubble_wetness_windows", WINDOW_POS, WINDOW_X, WINDOW_Y),
        (
            "pressure_flow_sensor_coupon_pockets",
            SENSOR_POS,
            SENSOR_X,
            SENSOR_Y,
        ),
        (
            "waste_flush_segregation_manifold",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        ("barcode_status_lands", TRACE_POS, TRACE_X, TRACE_Y),
    ]
}

fn assert_layout() {
    assert_eq!(RESIDUAL_WELLS, RESIDUAL_ROWS * RESIDUAL_COLS);
    assert_eq!(ROUTE_COMB_TEETH_PER_SIDE, ROUTE_CHANNELS);
    assert_eq!(FLUSH_PORTS, WASTE_PORTS);
    assert_eq!(DATUM_FIDUCIALS, 4);
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert!(WASHOUT_FRACTIONS >= RESIDUAL_ROWS + RESIDUAL_COLS);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= WASHOUT_FRACTIONS);
    assert!(CASSETTE_LIFT_CLEARANCE_Z > CASSETTE_Z);
    assert!(FRONT_ROBOT_CLEARANCE > 340.0);
    assert!(REAR_SERVICE_CLEARANCE > 220.0);

    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        assert_eq!(OUTPUTS.len(), 13);
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cassette_media_exchange_residual_fraction_washout_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[0].ends_with("_base_containment_deck.stl"));
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_design_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for feature in [
            "sealed_cassette_datum_nest",
            "inlet_outlet_route_combs",
            "residual_volume_dye_witness_wells",
            "washout_fraction_collection_nests",
            "timed_exchange_token_rail",
            "bubble_wetness_windows",
            "pressure_flow_sensor_coupon_pockets",
            "waste_flush_segregation_manifold",
            "barcode_status_lands",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn fixtures_fit_inside_station_bounds() {
        assert_layout();
        assert!(STATION_X >= 1200.0);
        assert!(STATION_Y >= 800.0);
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
    }

    #[test]
    fn validation_capacity_is_explicit_without_biological_thresholds() {
        assert_eq!(CASSETTE_DATUM_PINS, 6);
        assert_eq!(CASSETTE_PORT_DATUMS, 4);
        assert_eq!(ROUTE_CHANNELS, 8);
        assert_eq!(RESIDUAL_WELLS, 12);
        assert_eq!(WASHOUT_FRACTIONS, 10);
        assert_eq!(EXCHANGE_TOKENS, 6);
        assert_eq!(BUBBLE_WINDOWS, 4);
        assert_eq!(SENSOR_COUPONS, 8);
    }

    #[test]
    fn traceability_evidence_and_keepouts_are_sized_for_the_station() {
        assert!(BARCODE_LANDS >= ROUTE_CHANNELS);
        assert!(STATUS_LANES == 3);
        assert!(CERTIFICATE_LANDS >= 3);
        assert!(EVIDENCE_CAMERAS >= 4);
        assert!(EVIDENCE_X < STATION_X);
        assert!(ROBOT_KEEPOUT_X < STATION_X);
        assert!(ROBOT_KEEPOUT_Y < STATION_Y);
    }
}
