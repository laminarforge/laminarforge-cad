use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag connector wetness/capillary holdover validation station.
//
// This generator packages a mechanical validation fixture for connector
// handling, visible capillary wetness holdover witnesses, custody labels,
// evidence capture, and robot/service keepout gauges around closed media-bag
// connector samples. It is mechanical validation packaging only; it is not a
// sterile-process claim, connector SOP, pressure-rated device, or biological
// acceptance criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_media_bag_connector_wetness_capillary_holdover_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_deck_drip_condensate_capture_moat.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_media_bag_connector_nests.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_capillary_wetness_witness_channels.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_timed_holdover_token_rail.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_dry_wet_comparison_coupon_lands.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_connector_cap_parks.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_pressure_decay_witness_ports.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_barcode_coa_custody_lands.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_release_hold_reject_gates.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_camera_evidence_bridge.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_robot_service_keepouts.stl",
    "output/closed_media_bag_connector_wetness_capillary_holdover_station_assembly.stl",
];

const DESIGN_SCOPE: &str = "mechanical validation packaging only; not a sterile-process claim, connector SOP, pressure-rated device, or biological acceptance criterion";

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 24] = [
    "media_bag_connector_nests",
    "media_bag_connector_datum_saddles",
    "connector_orientation_keys",
    "capillary_wetness_witness_channels",
    "capillary_meniscus_tick_marks",
    "timed_holdover_token_rail",
    "holdover_token_slots",
    "dry_comparison_coupon_lands",
    "wet_comparison_coupon_lands",
    "connector_cap_parks",
    "cap_custody_ticks",
    "pressure_decay_witness_ports",
    "drip_condensate_capture_moat",
    "drip_witness_pads",
    "barcode_custody_lands",
    "coa_custody_lands",
    "tamper_seal_tabs",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepouts",
    "service_keepouts",
];

const DECK_X: f64 = 1260.0;
const DECK_Y: f64 = 850.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const MOAT_INSET_X: f64 = 1086.0;
const MOAT_INSET_Y: f64 = 646.0;
const MOAT_DEPTH: f64 = 6.0;
const MOAT_RAIL_W: f64 = 20.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const NEST_X: f64 = 448.0;
const NEST_Y: f64 = 238.0;
const NEST_Z: f64 = 44.0;
const NEST_POS: (f64, f64) = (-372.0, 236.0);
const NEST_COLS: usize = 3;
const NEST_ROWS: usize = 2;
const CONNECTOR_NESTS: usize = NEST_COLS * NEST_ROWS;
const NEST_PITCH_X: f64 = 112.0;
const NEST_PITCH_Y: f64 = 92.0;
const CONNECTOR_POCKET_D: f64 = 36.0;
const CONNECTOR_KEY_W: f64 = 10.0;
const BAG_DATUM_SADDLES: usize = 4;

const CAPILLARY_X: f64 = 560.0;
const CAPILLARY_Y: f64 = 156.0;
const CAPILLARY_Z: f64 = 34.0;
const CAPILLARY_POS: (f64, f64) = (58.0, 224.0);
const CAPILLARY_CHANNELS: usize = 8;
const CAPILLARY_PITCH_Y: f64 = 17.0;
const CAPILLARY_CHANNEL_W: f64 = 6.0;
const MENISCUS_TICKS_PER_CHANNEL: usize = 5;

const TOKEN_X: f64 = 510.0;
const TOKEN_Y: f64 = 92.0;
const TOKEN_Z: f64 = 34.0;
const TOKEN_POS: (f64, f64) = (340.0, 70.0);
const HOLDOVER_TOKENS: usize = 8;
const TOKEN_PITCH_X: f64 = 58.0;
const TOKEN_D: f64 = 27.0;

const COUPON_X: f64 = 430.0;
const COUPON_Y: f64 = 214.0;
const COUPON_Z: f64 = 30.0;
const COUPON_POS: (f64, f64) = (-380.0, -56.0);
const COUPONS_PER_LANE: usize = 4;
const COMPARISON_LANES: usize = 2;
const DRY_LANE_INDEX: usize = 0;
const WET_LANE_INDEX: usize = 1;
const COUPON_PITCH_X: f64 = 78.0;
const COUPON_LANE_PITCH_Y: f64 = 76.0;
const COUPON_LAND_X: f64 = 58.0;
const COUPON_LAND_Y: f64 = 34.0;
const DRY_COUPON_LANDS: usize = COUPONS_PER_LANE;
const WET_COUPON_LANDS: usize = COUPONS_PER_LANE;

const CAP_PARK_X: f64 = 330.0;
const CAP_PARK_Y: f64 = 198.0;
const CAP_PARK_Z: f64 = 30.0;
const CAP_PARK_POS: (f64, f64) = (402.0, 248.0);
const CAP_PARK_COLS: usize = 4;
const CAP_PARK_ROWS: usize = 3;
const CONNECTOR_CAP_PARKS: usize = CAP_PARK_COLS * CAP_PARK_ROWS;
const CAP_PARK_PITCH_X: f64 = 72.0;
const CAP_PARK_PITCH_Y: f64 = 54.0;
const CAP_PARK_D: f64 = 23.0;

const PRESSURE_X: f64 = 500.0;
const PRESSURE_Y: f64 = 92.0;
const PRESSURE_Z: f64 = 40.0;
const PRESSURE_POS: (f64, f64) = (72.0, -116.0);
const PRESSURE_DECAY_PORTS: usize = 6;
const PRESSURE_PORT_PITCH_X: f64 = 66.0;
const PRESSURE_PORT_D: f64 = 13.0;

const CUSTODY_X: f64 = 390.0;
const CUSTODY_Y: f64 = 118.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (360.0, -304.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_TABS: usize = 4;

const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 110.0;
const GATE_Z: f64 = 38.0;
const GATE_POS: (f64, f64) = (-112.0, -334.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;
const GATE_PITCH_X: f64 = 112.0;

const CAMERA_BRIDGE_X: f64 = 956.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_CLEARANCE_Z: f64 = 154.0;
const CAMERA_POST_X: f64 = 30.0;
const CAMERA_POST_Y: f64 = 60.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -18.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;

const ROBOT_KEEPOUT_X: f64 = 1140.0;
const ROBOT_KEEPOUT_Y: f64 = 88.0;
const ROBOT_KEEPOUT_Z: f64 = 72.0;
const SERVICE_KEEPOUT_X: f64 = 96.0;
const SERVICE_KEEPOUT_Y: f64 = 668.0;
const SERVICE_KEEPOUT_Z: f64 = 92.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 312.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(deck_drip_condensate_capture_moat(), OUTPUTS[0]);
    write_part(media_bag_connector_nests(), OUTPUTS[1]);
    write_part(capillary_wetness_witness_channels(), OUTPUTS[2]);
    write_part(timed_holdover_token_rail(), OUTPUTS[3]);
    write_part(dry_wet_comparison_coupon_lands(), OUTPUTS[4]);
    write_part(connector_cap_parks(), OUTPUTS[5]);
    write_part(pressure_decay_witness_ports(), OUTPUTS[6]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[7]);
    write_part(release_hold_reject_gates(), OUTPUTS[8]);
    write_part(camera_evidence_bridge(), OUTPUTS[9]);
    write_part(robot_service_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed media-bag connector wetness/capillary holdover station: {:.0}mm x {:.0}mm deck, {} connector nests, {} capillary witness channels, {} holdover tokens.",
        DECK_X, DECK_Y, CONNECTOR_NESTS, CAPILLARY_CHANNELS, HOLDOVER_TOKENS
    );
    println!(
        "Comparison and custody packaging: {} dry coupon lands, {} wet coupon lands, {} cap parks, {} pressure-decay witness ports, {} barcode lands, {} COA lands.",
        DRY_COUPON_LANDS,
        WET_COUPON_LANDS,
        CONNECTOR_CAP_PARKS,
        PRESSURE_DECAY_PORTS,
        BARCODE_LANDS,
        COA_LANDS
    );
    println!("Scope: {DESIGN_SCOPE}.");
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    deck_drip_condensate_capture_moat()
        + media_bag_connector_nests()
        + capillary_wetness_witness_channels()
        + timed_holdover_token_rail()
        + dry_wet_comparison_coupon_lands()
        + connector_cap_parks()
        + pressure_decay_witness_ports()
        + barcode_coa_custody_lands()
        + release_hold_reject_gates()
        + camera_evidence_bridge()
        + robot_service_keepouts()
}

fn deck_drip_condensate_capture_moat() -> Part {
    let deck = centered_cube(
        "closed_media_bag_holdover_station_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_media_bag_holdover_station_shallow_condensate_sump",
        MOAT_INSET_X,
        MOAT_INSET_Y,
        MOAT_DEPTH + 1.0,
    )
    .translate(0.0, -6.0, DECK_Z / 2.0 - MOAT_DEPTH / 2.0);
    let drain = centered_cylinder(
        "closed_media_bag_holdover_station_moat_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 10.0, 0.0);

    deck - sump - drain
        + containment_rim()
        + condensate_capture_moat_rails()
        + drip_witness_pads()
        + deck_datums()
        + station_landing_pads()
}

fn containment_rim() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    centered_cube(
        "closed_media_bag_holdover_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            "closed_media_bag_holdover_station_rear_containment_rim",
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            "closed_media_bag_holdover_station_left_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            "closed_media_bag_holdover_station_right_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn condensate_capture_moat_rails() -> Part {
    let z = DECK_Z / 2.0 + 8.0;
    let outer_x = COUPON_X + CAPILLARY_X + 170.0;
    let outer_y = 365.0;
    let center = (-106.0, 34.0);

    centered_cube(
        "closed_media_bag_holdover_station_drip_condensate_moat_front_rail",
        outer_x,
        MOAT_RAIL_W,
        16.0,
    )
    .translate(center.0, center.1 - outer_y / 2.0, z)
        + centered_cube(
            "closed_media_bag_holdover_station_drip_condensate_moat_rear_rail",
            outer_x,
            MOAT_RAIL_W,
            16.0,
        )
        .translate(center.0, center.1 + outer_y / 2.0, z)
        + centered_cube(
            "closed_media_bag_holdover_station_drip_condensate_moat_left_rail",
            MOAT_RAIL_W,
            outer_y,
            16.0,
        )
        .translate(center.0 - outer_x / 2.0, center.1, z)
        + centered_cube(
            "closed_media_bag_holdover_station_drip_condensate_moat_right_rail",
            MOAT_RAIL_W,
            outer_y,
            16.0,
        )
        .translate(center.0 + outer_x / 2.0, center.1, z)
}

fn drip_witness_pads() -> Part {
    let mut pads = Part::empty("closed_media_bag_holdover_station_drip_witness_pads");
    for idx in 0..CONNECTOR_NESTS {
        let x = -382.0 + idx as f64 * 74.0;
        pads = pads
            + centered_cube(
                format!("closed_media_bag_holdover_station_drip_condensate_witness_pad_{idx}"),
                46.0,
                18.0,
                3.0,
            )
            .translate(x, -220.0, DECK_Z / 2.0 + 1.5);
    }
    pads
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_media_bag_holdover_station_deck_datums");
    for idx in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 72.0 + idx as f64 * ((DECK_X - 144.0) / 9.0);
        let y = if idx % 2 == 0 {
            DECK_Y / 2.0 - 66.0
        } else {
            -DECK_Y / 2.0 + 66.0
        };
        let boss = centered_cylinder(
            format!("closed_media_bag_holdover_station_datum_boss_{idx}"),
            8.0,
            6.0,
            28,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("closed_media_bag_holdover_station_datum_bore_{idx}"),
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
    landing_pad("connector_nest_land", NEST_X, NEST_Y, NEST_POS)
        + landing_pad(
            "capillary_witness_land",
            CAPILLARY_X,
            CAPILLARY_Y,
            CAPILLARY_POS,
        )
        + landing_pad("holdover_token_land", TOKEN_X, TOKEN_Y, TOKEN_POS)
        + landing_pad("dry_wet_coupon_land", COUPON_X, COUPON_Y, COUPON_POS)
        + landing_pad("cap_park_land", CAP_PARK_X, CAP_PARK_Y, CAP_PARK_POS)
        + landing_pad(
            "pressure_decay_witness_land",
            PRESSURE_X,
            PRESSURE_Y,
            PRESSURE_POS,
        )
        + landing_pad("custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pad("disposition_gate_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pad(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("closed_media_bag_holdover_station_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn media_bag_connector_nests() -> Part {
    let base = centered_cube(
        "closed_media_bag_holdover_station_media_bag_connector_nest_plate",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(NEST_POS.0, NEST_POS.1, top_z(NEST_Z));

    let mut cuts = Part::empty("closed_media_bag_holdover_station_connector_nest_cuts");
    let mut fixtures = Part::empty("closed_media_bag_holdover_station_connector_nest_fixtures");
    for idx in 0..CONNECTOR_NESTS {
        let (x, y) = connector_nest_center(idx);
        let world_x = NEST_POS.0 + x;
        let world_y = NEST_POS.1 + y;
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_connector_socket_{idx}"),
                CONNECTOR_POCKET_D / 2.0,
                NEST_Z + 2.0,
                48,
            )
            .translate(world_x, world_y, top_z(NEST_Z))
            + centered_cube(
                format!("closed_media_bag_holdover_station_connector_orientation_key_{idx}"),
                CONNECTOR_KEY_W,
                32.0,
                NEST_Z + 3.0,
            )
            .translate(world_x + CONNECTOR_POCKET_D / 2.0, world_y, top_z(NEST_Z));
        fixtures = fixtures
            + centered_cube(
                format!("closed_media_bag_holdover_station_connector_capture_flag_{idx}"),
                34.0,
                7.0,
                12.0,
            )
            .translate(world_x, world_y + 34.0, DECK_Z / 2.0 + NEST_Z + 6.0)
            + centered_cube(
                format!("closed_media_bag_holdover_station_connector_tail_relief_marker_{idx}"),
                18.0,
                36.0,
                6.0,
            )
            .translate(world_x - 30.0, world_y - 34.0, DECK_Z / 2.0 + NEST_Z + 3.0);
    }

    base - cuts + fixtures + media_bag_datum_saddles()
}

fn media_bag_datum_saddles() -> Part {
    let mut saddles = Part::empty("closed_media_bag_holdover_station_media_bag_datum_saddles");
    for idx in 0..BAG_DATUM_SADDLES {
        let x = NEST_POS.0 - 158.0 + idx as f64 * 104.0;
        let saddle = centered_cylinder(
            format!("closed_media_bag_holdover_station_media_bag_datum_saddle_{idx}"),
            11.0,
            NEST_Y - 64.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, NEST_POS.1, DECK_Z / 2.0 + NEST_Z + 13.0);
        saddles = saddles + saddle;
    }
    saddles
}

fn capillary_wetness_witness_channels() -> Part {
    let base = centered_cube(
        "closed_media_bag_holdover_station_capillary_wetness_witness_block",
        CAPILLARY_X,
        CAPILLARY_Y,
        CAPILLARY_Z,
    )
    .translate(CAPILLARY_POS.0, CAPILLARY_POS.1, top_z(CAPILLARY_Z));

    let mut channels = Part::empty("closed_media_bag_holdover_station_capillary_channel_cuts");
    let mut witness_marks =
        Part::empty("closed_media_bag_holdover_station_capillary_meniscus_tick_marks");
    for idx in 0..CAPILLARY_CHANNELS {
        let y = CAPILLARY_POS.1 + capillary_channel_y(idx);
        channels = channels
            + centered_cube(
                format!("closed_media_bag_holdover_station_capillary_wetness_channel_{idx}"),
                CAPILLARY_X - 72.0,
                CAPILLARY_CHANNEL_W,
                CAPILLARY_Z + 2.0,
            )
            .translate(CAPILLARY_POS.0, y, top_z(CAPILLARY_Z));

        for tick in 0..MENISCUS_TICKS_PER_CHANNEL {
            let x = CAPILLARY_POS.0 - 198.0 + tick as f64 * 99.0;
            witness_marks = witness_marks
                + centered_cube(
                    format!("closed_media_bag_holdover_station_channel_{idx}_meniscus_tick_{tick}"),
                    4.0,
                    CAPILLARY_CHANNEL_W + 13.0,
                    5.0,
                )
                .translate(x, y, DECK_Z / 2.0 + CAPILLARY_Z + 2.5);
        }
    }

    base - channels + capillary_entry_cups() + witness_marks
}

fn capillary_entry_cups() -> Part {
    let mut cups = Part::empty("closed_media_bag_holdover_station_capillary_entry_cups");
    for idx in 0..CAPILLARY_CHANNELS {
        let y = CAPILLARY_POS.1 + capillary_channel_y(idx);
        let wet_side = centered_cylinder(
            format!("closed_media_bag_holdover_station_capillary_wet_entry_cup_{idx}"),
            10.0,
            6.0,
            28,
        )
        .translate(
            CAPILLARY_POS.0 - CAPILLARY_X / 2.0 + 30.0,
            y,
            DECK_Z / 2.0 + CAPILLARY_Z + 3.0,
        );
        let dry_side = centered_cylinder(
            format!("closed_media_bag_holdover_station_capillary_dry_reference_cup_{idx}"),
            8.0,
            5.0,
            28,
        )
        .translate(
            CAPILLARY_POS.0 + CAPILLARY_X / 2.0 - 30.0,
            y,
            DECK_Z / 2.0 + CAPILLARY_Z + 2.5,
        );
        cups = cups + wet_side + dry_side;
    }
    cups
}

fn timed_holdover_token_rail() -> Part {
    let rail = centered_cube(
        "closed_media_bag_holdover_station_timed_holdover_token_rail",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, top_z(TOKEN_Z));
    let sight_slot = centered_cube(
        "closed_media_bag_holdover_station_holdover_token_sight_slot",
        TOKEN_X - 54.0,
        16.0,
        TOKEN_Z + 2.0,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1 - 18.0, top_z(TOKEN_Z));

    let mut token_slots = Part::empty("closed_media_bag_holdover_station_holdover_token_slots");
    let mut token_steps = Part::empty("closed_media_bag_holdover_station_holdover_token_steps");
    for idx in 0..HOLDOVER_TOKENS {
        let x = TOKEN_POS.0 + token_x(idx);
        token_slots = token_slots
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_holdover_token_socket_{idx}"),
                TOKEN_D / 2.0,
                TOKEN_Z + 3.0,
                36,
            )
            .translate(x, TOKEN_POS.1 + 18.0, top_z(TOKEN_Z));
        token_steps = token_steps
            + centered_cube(
                format!("closed_media_bag_holdover_station_holdover_elapsed_step_{idx}"),
                14.0,
                34.0,
                5.0 + idx as f64 * 1.4,
            )
            .translate(
                x,
                TOKEN_POS.1 - TOKEN_Y / 2.0 + 19.0,
                DECK_Z / 2.0 + TOKEN_Z + 2.5 + idx as f64 * 0.7,
            )
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_holdover_token_gauge_{idx}"),
                TOKEN_D / 2.0 - 4.0,
                6.0,
                36,
            )
            .translate(x, TOKEN_POS.1 + 18.0, DECK_Z / 2.0 + TOKEN_Z + 3.0);
    }

    rail - sight_slot - token_slots + token_steps
}

fn dry_wet_comparison_coupon_lands() -> Part {
    let base = centered_cube(
        "closed_media_bag_holdover_station_dry_wet_coupon_comparison_plate",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(COUPON_POS.0, COUPON_POS.1, top_z(COUPON_Z));

    let mut land_cuts =
        Part::empty("closed_media_bag_holdover_station_dry_wet_coupon_land_recesses");
    let mut stops = Part::empty("closed_media_bag_holdover_station_dry_wet_coupon_stops");
    for lane in 0..COMPARISON_LANES {
        let lane_y = COUPON_POS.1 + coupon_lane_y(lane);
        for idx in 0..COUPONS_PER_LANE {
            let x = COUPON_POS.0 + coupon_x(idx);
            land_cuts = land_cuts
                + centered_cube(
                    format!(
                        "closed_media_bag_holdover_station_{}_coupon_land_recess_{idx}",
                        lane_name(lane)
                    ),
                    COUPON_LAND_X,
                    COUPON_LAND_Y,
                    COUPON_Z + 2.0,
                )
                .translate(x, lane_y, top_z(COUPON_Z));
            stops = stops
                + centered_cube(
                    format!(
                        "closed_media_bag_holdover_station_{}_coupon_front_stop_{idx}",
                        lane_name(lane)
                    ),
                    COUPON_LAND_X,
                    5.0,
                    8.0,
                )
                .translate(
                    x,
                    lane_y - COUPON_LAND_Y / 2.0 - 5.0,
                    DECK_Z / 2.0 + COUPON_Z + 4.0,
                )
                + centered_cube(
                    format!(
                        "closed_media_bag_holdover_station_{}_coupon_rear_stop_{idx}",
                        lane_name(lane)
                    ),
                    COUPON_LAND_X,
                    5.0,
                    8.0,
                )
                .translate(
                    x,
                    lane_y + COUPON_LAND_Y / 2.0 + 5.0,
                    DECK_Z / 2.0 + COUPON_Z + 4.0,
                );
        }
    }

    base - land_cuts + stops + coupon_lane_divider()
}

fn coupon_lane_divider() -> Part {
    centered_cube(
        "closed_media_bag_holdover_station_dry_wet_comparison_lane_divider",
        COUPON_X - 42.0,
        8.0,
        16.0,
    )
    .translate(COUPON_POS.0, COUPON_POS.1, DECK_Z / 2.0 + COUPON_Z + 8.0)
}

fn connector_cap_parks() -> Part {
    let base = centered_cube(
        "closed_media_bag_holdover_station_connector_cap_park_plate",
        CAP_PARK_X,
        CAP_PARK_Y,
        CAP_PARK_Z,
    )
    .translate(CAP_PARK_POS.0, CAP_PARK_POS.1, top_z(CAP_PARK_Z));

    let mut pockets = Part::empty("closed_media_bag_holdover_station_connector_cap_park_pockets");
    let mut ticks = Part::empty("closed_media_bag_holdover_station_connector_cap_custody_ticks");
    for idx in 0..CONNECTOR_CAP_PARKS {
        let (x, y) = cap_park_center(idx);
        let world_x = CAP_PARK_POS.0 + x;
        let world_y = CAP_PARK_POS.1 + y;
        pockets = pockets
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_connector_cap_park_{idx}"),
                CAP_PARK_D / 2.0,
                CAP_PARK_Z + 3.0,
                32,
            )
            .translate(world_x, world_y, top_z(CAP_PARK_Z));
        ticks = ticks
            + centered_cube(
                format!("closed_media_bag_holdover_station_connector_cap_custody_tick_{idx}"),
                24.0,
                3.5,
                4.0,
            )
            .translate(world_x, world_y - 23.0, DECK_Z / 2.0 + CAP_PARK_Z + 2.0);
    }

    base - pockets + ticks
}

fn pressure_decay_witness_ports() -> Part {
    let bar = centered_cube(
        "closed_media_bag_holdover_station_pressure_decay_witness_port_bar",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(PRESSURE_POS.0, PRESSURE_POS.1, top_z(PRESSURE_Z));

    let mut bores = Part::empty("closed_media_bag_holdover_station_pressure_decay_witness_bores");
    let mut collars =
        Part::empty("closed_media_bag_holdover_station_pressure_decay_witness_port_collars");
    for idx in 0..PRESSURE_DECAY_PORTS {
        let x = PRESSURE_POS.0 + pressure_port_x(idx);
        bores = bores
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_pressure_decay_witness_port_{idx}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_Z + 3.0,
                32,
            )
            .translate(x, PRESSURE_POS.1, top_z(PRESSURE_Z));
        collars = collars
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_pressure_decay_witness_collar_{idx}"),
                17.0,
                8.0,
                32,
            )
            .translate(x, PRESSURE_POS.1, DECK_Z / 2.0 + PRESSURE_Z + 4.0)
            + centered_cube(
                format!(
                    "closed_media_bag_holdover_station_pressure_decay_witness_reference_tab_{idx}"
                ),
                28.0,
                8.0,
                5.0,
            )
            .translate(x, PRESSURE_POS.1 + 32.0, DECK_Z / 2.0 + PRESSURE_Z + 2.5);
    }

    bar - bores + collars
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_media_bag_holdover_station_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z));

    let mut lands = Part::empty("closed_media_bag_holdover_station_barcode_coa_custody_lands");
    for idx in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_bag_holdover_station_barcode_custody_land_{idx}"),
                48.0,
                22.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 145.0 + idx as f64 * 58.0,
                CUSTODY_POS.1 + 28.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    for idx in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_bag_holdover_station_coa_custody_land_{idx}"),
                76.0,
                26.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 86.0 + idx as f64 * 86.0,
                CUSTODY_POS.1 - 30.0,
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    for idx in 0..TAMPER_SEAL_TABS {
        lands = lands
            + centered_cube(
                format!("closed_media_bag_holdover_station_tamper_seal_tab_{idx}"),
                18.0,
                32.0,
                6.0,
            )
            .translate(
                CUSTODY_POS.0 - CUSTODY_X / 2.0 + 36.0 + idx as f64 * 106.0,
                CUSTODY_POS.1,
                DECK_Z / 2.0 + CUSTODY_Z + 3.0,
            );
    }

    panel + lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_media_bag_holdover_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z));

    let mut gate_cuts = Part::empty("closed_media_bag_holdover_station_disposition_gate_cuts");
    let mut paddles = Part::empty("closed_media_bag_holdover_station_disposition_gate_paddles");
    for idx in 0..DISPOSITION_GATES {
        let x = GATE_POS.0 + gate_x(idx);
        gate_cuts = gate_cuts
            + centered_cube(
                format!(
                    "closed_media_bag_holdover_station_{}_gate_trough",
                    disposition_gate_name(idx)
                ),
                82.0,
                58.0,
                GATE_Z + 2.0,
            )
            .translate(x, GATE_POS.1, top_z(GATE_Z));
        paddles = paddles
            + centered_cube(
                format!(
                    "closed_media_bag_holdover_station_{}_gate_paddle",
                    disposition_gate_name(idx)
                ),
                10.0,
                88.0,
                42.0,
            )
            .translate(x + 42.0, GATE_POS.1, DECK_Z / 2.0 + GATE_Z + 21.0);
    }
    for idx in 0..GATE_TOKEN_SLOTS {
        let x = GATE_POS.0 - 165.0 + idx as f64 * 66.0;
        gate_cuts = gate_cuts
            + centered_cube(
                format!("closed_media_bag_holdover_station_disposition_token_slot_{idx}"),
                38.0,
                14.0,
                GATE_Z + 2.0,
            )
            .translate(x, GATE_POS.1 - GATE_Y / 2.0 + 18.0, top_z(GATE_Z));
    }

    base - gate_cuts + paddles
}

fn camera_evidence_bridge() -> Part {
    let post_z = CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z;
    let left_post = centered_cube(
        "closed_media_bag_holdover_station_camera_evidence_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 - CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + post_z / 2.0,
    );
    let right_post = centered_cube(
        "closed_media_bag_holdover_station_camera_evidence_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 + CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + post_z / 2.0,
    );
    let beam = centered_cube(
        "closed_media_bag_holdover_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X + CAMERA_POST_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_mount_tabs() + evidence_fiducials()
}

fn camera_mount_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_bag_holdover_station_camera_mount_tabs");
    for idx in 0..CAMERA_MOUNTS {
        let x = CAMERA_BRIDGE_POS.0 - 340.0 + idx as f64 * 170.0;
        tabs = tabs
            + centered_cube(
                format!("closed_media_bag_holdover_station_camera_mount_tab_{idx}"),
                64.0,
                28.0,
                8.0,
            )
            .translate(
                x,
                CAMERA_BRIDGE_POS.1,
                DECK_Z / 2.0 + CAMERA_CLEARANCE_Z - 4.0,
            );
    }
    tabs
}

fn evidence_fiducials() -> Part {
    let positions = [
        (-542.0, -360.0),
        (-392.0, -360.0),
        (-542.0, 360.0),
        (-392.0, 360.0),
        (-72.0, -360.0),
        (72.0, 360.0),
        (392.0, -360.0),
        (542.0, -360.0),
        (392.0, 360.0),
        (542.0, 360.0),
    ];
    assert_eq!(positions.len(), EVIDENCE_FIDUCIALS);
    let mut fiducials = Part::empty("closed_media_bag_holdover_station_evidence_fiducials");
    for (idx, (x, y)) in positions.iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_media_bag_holdover_station_evidence_fiducial_{idx}"),
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
        "closed_media_bag_holdover_station_front_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 94.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let rear_robot = centered_cube(
        "closed_media_bag_holdover_station_rear_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 94.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_media_bag_holdover_station_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 84.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "closed_media_bag_holdover_station_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 - 84.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "closed_media_bag_holdover_station_top_service_clearance_gauge",
        DECK_X - 150.0,
        DECK_Y - 160.0,
        28.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + rear_robot + left_service + right_service + top_clearance
}

fn connector_nest_center(index: usize) -> (f64, f64) {
    let col = index % NEST_COLS;
    let row = index / NEST_COLS;
    grid_center(col, row, NEST_COLS, NEST_ROWS, NEST_PITCH_X, NEST_PITCH_Y)
}

fn cap_park_center(index: usize) -> (f64, f64) {
    let col = index % CAP_PARK_COLS;
    let row = index / CAP_PARK_COLS;
    grid_center(
        col,
        row,
        CAP_PARK_COLS,
        CAP_PARK_ROWS,
        CAP_PARK_PITCH_X,
        CAP_PARK_PITCH_Y,
    )
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

fn capillary_channel_y(index: usize) -> f64 {
    (index as f64 - (CAPILLARY_CHANNELS as f64 - 1.0) / 2.0) * CAPILLARY_PITCH_Y
}

fn token_x(index: usize) -> f64 {
    (index as f64 - (HOLDOVER_TOKENS as f64 - 1.0) / 2.0) * TOKEN_PITCH_X
}

fn coupon_x(index: usize) -> f64 {
    (index as f64 - (COUPONS_PER_LANE as f64 - 1.0) / 2.0) * COUPON_PITCH_X
}

fn coupon_lane_y(lane: usize) -> f64 {
    (lane as f64 - (COMPARISON_LANES as f64 - 1.0) / 2.0) * COUPON_LANE_PITCH_Y
}

fn pressure_port_x(index: usize) -> f64 {
    (index as f64 - (PRESSURE_DECAY_PORTS as f64 - 1.0) / 2.0) * PRESSURE_PORT_PITCH_X
}

fn gate_x(index: usize) -> f64 {
    (index as f64 - (DISPOSITION_GATES as f64 - 1.0) / 2.0) * GATE_PITCH_X
}

fn lane_name(lane: usize) -> &'static str {
    match lane {
        DRY_LANE_INDEX => "dry_comparison",
        WET_LANE_INDEX => "wet_comparison",
        _ => panic!("unknown coupon lane index"),
    }
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

fn capillary_channel_span_y() -> f64 {
    (CAPILLARY_CHANNELS as f64 - 1.0) * CAPILLARY_PITCH_Y + CAPILLARY_CHANNEL_W
}

fn holdover_token_span_x() -> f64 {
    (HOLDOVER_TOKENS as f64 - 1.0) * TOKEN_PITCH_X + TOKEN_D
}

fn comparison_coupon_span_x() -> f64 {
    (COUPONS_PER_LANE as f64 - 1.0) * COUPON_PITCH_X + COUPON_LAND_X
}

fn pressure_port_span_x() -> f64 {
    (PRESSURE_DECAY_PORTS as f64 - 1.0) * PRESSURE_PORT_PITCH_X + PRESSURE_PORT_D
}

fn camera_bridge_clearance() -> f64 {
    CAMERA_CLEARANCE_Z - NEST_Z.max(PRESSURE_Z).max(GATE_Z).max(CAPILLARY_Z)
}

fn module_extents_fit_inner_deck() -> bool {
    module_fits(NEST_POS, NEST_X, NEST_Y)
        && module_fits(CAPILLARY_POS, CAPILLARY_X, CAPILLARY_Y)
        && module_fits(TOKEN_POS, TOKEN_X, TOKEN_Y)
        && module_fits(COUPON_POS, COUPON_X, COUPON_Y)
        && module_fits(CAP_PARK_POS, CAP_PARK_X, CAP_PARK_Y)
        && module_fits(PRESSURE_POS, PRESSURE_X, PRESSURE_Y)
        && module_fits(CUSTODY_POS, CUSTODY_X, CUSTODY_Y)
        && module_fits(GATE_POS, GATE_X, GATE_Y)
}

fn module_fits(pos: (f64, f64), x: f64, y: f64) -> bool {
    let inner_x = DECK_X / 2.0 - RIM_W - 8.0;
    let inner_y = DECK_Y / 2.0 - RIM_W - 8.0;
    pos.0.abs() + x / 2.0 <= inner_x && pos.1.abs() + y / 2.0 <= inner_y
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected STL output count");
    assert_eq!(CONNECTOR_NESTS, NEST_COLS * NEST_ROWS);
    assert_eq!(CONNECTOR_CAP_PARKS, CAP_PARK_COLS * CAP_PARK_ROWS);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert!(PRESSURE_DECAY_PORTS >= CONNECTOR_NESTS);
    assert!(CONNECTOR_CAP_PARKS >= CONNECTOR_NESTS * 2);
    assert!(capillary_channel_span_y() < CAPILLARY_Y - 18.0);
    assert!(holdover_token_span_x() < TOKEN_X - 36.0);
    assert!(comparison_coupon_span_x() < COUPON_X - 74.0);
    assert!(pressure_port_span_x() < PRESSURE_X - 90.0);
    assert!(camera_bridge_clearance() >= 100.0);
    assert!(module_extents_fit_inner_deck());
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
            "media_bag_connector_nests",
            "capillary_wetness_witness_channels",
            "timed_holdover_token_rail",
            "dry_comparison_coupon_lands",
            "wet_comparison_coupon_lands",
            "connector_cap_parks",
            "pressure_decay_witness_ports",
            "drip_condensate_capture_moat",
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
    fn scope_excludes_process_pressure_and_biology_claims() {
        assert!(DESIGN_SCOPE.contains("mechanical validation packaging only"));
        assert!(DESIGN_SCOPE.contains("not a sterile-process claim"));
        assert!(DESIGN_SCOPE.contains("connector SOP"));
        assert!(DESIGN_SCOPE.contains("pressure-rated device"));
        assert!(DESIGN_SCOPE.contains("biological acceptance criterion"));
    }

    #[test]
    fn feature_counts_match_holdover_validation_packaging() {
        assert_eq!(CONNECTOR_NESTS, 6);
        assert_eq!(CAPILLARY_CHANNELS, 8);
        assert_eq!(HOLDOVER_TOKENS, 8);
        assert_eq!(DRY_COUPON_LANDS, 4);
        assert_eq!(WET_COUPON_LANDS, 4);
        assert_eq!(CONNECTOR_CAP_PARKS, 12);
        assert_eq!(PRESSURE_DECAY_PORTS, CONNECTOR_NESTS);
        assert_eq!(BARCODE_LANDS, CONNECTOR_NESTS);
        assert_eq!(COA_LANDS, DISPOSITION_GATES);
        assert_eq!(EVIDENCE_FIDUCIALS, 10);
    }

    #[test]
    fn repeated_features_stay_inside_their_hardware() {
        let first_nest = connector_nest_center(0);
        let last_nest = connector_nest_center(CONNECTOR_NESTS - 1);
        assert!(first_nest.0.abs() + CONNECTOR_POCKET_D / 2.0 < NEST_X / 2.0 - 22.0);
        assert!(last_nest.0.abs() + CONNECTOR_POCKET_D / 2.0 < NEST_X / 2.0 - 22.0);
        assert!(first_nest.1.abs() + CONNECTOR_POCKET_D / 2.0 < NEST_Y / 2.0 - 22.0);
        assert!(last_nest.1.abs() + CONNECTOR_POCKET_D / 2.0 < NEST_Y / 2.0 - 22.0);

        let last_cap = cap_park_center(CONNECTOR_CAP_PARKS - 1);
        assert!(last_cap.0.abs() + CAP_PARK_D / 2.0 < CAP_PARK_X / 2.0 - 16.0);
        assert!(last_cap.1.abs() + CAP_PARK_D / 2.0 < CAP_PARK_Y / 2.0 - 16.0);

        assert!(capillary_channel_span_y() < CAPILLARY_Y - 18.0);
        assert!(holdover_token_span_x() < TOKEN_X - 36.0);
        assert!(comparison_coupon_span_x() < COUPON_X - 74.0);
        assert!(pressure_port_span_x() < PRESSURE_X - 90.0);
    }

    #[test]
    fn disposition_names_and_layout_clearances_are_stable() {
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert_eq!(lane_name(DRY_LANE_INDEX), "dry_comparison");
        assert_eq!(lane_name(WET_LANE_INDEX), "wet_comparison");
        assert!(coupon_lane_y(WET_LANE_INDEX) > coupon_lane_y(DRY_LANE_INDEX));
        assert_layout();
    }
}
