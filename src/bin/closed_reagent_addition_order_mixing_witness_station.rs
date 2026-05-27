use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent addition-order and mixing-witness station for no-cell media
// preparation validation before automated tissue-chip runs.
//
// Design intent:
// - Keep additive vials, bags, sterile connectors, order tokens, and evidence
//   capture on a single contained deck so the station catches wrong sequence
//   setup before any media is released.
// - Route a closed witness side-loop past mixing coupons, bubble/wetness
//   windows, temperature/light exposure flags, recovery wells, and flush wells
//   without open handling or cells.
// - Make release/hold/reject disposition a physical lane decision with barcode,
//   COA, camera, and service keepout references visible to automation.
//
// This file is mechanical concept CAD only. It does not define chemistry,
// additive acceptance criteria, sterility validation, or a media recipe.

const OUTPUTS: [&str; 12] = [
    "output/closed_reagent_addition_order_mixing_witness_station_containment_deck.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_staged_vial_bag_nests.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_order_token_rails.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_mixing_witness_loop_coupon_bank.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_cold_light_shield_features.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_sterile_connector_bulkhead.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_bubble_wetness_windows.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_recovery_flush_wells.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_barcode_coa_custody_lands.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_release_hold_reject_lanes.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_camera_evidence_bridge_keepout_gauges.stl",
    "output/closed_reagent_addition_order_mixing_witness_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_INTENT_FEATURES: [&str; 14] = [
    "containment_deck",
    "staged_additive_vial_nests",
    "staged_additive_bag_nests",
    "order_token_rails",
    "mixing_witness_loop",
    "coupon_bank",
    "cold_light_shield",
    "sterile_connector_bulkhead",
    "bubble_windows",
    "wetness_windows",
    "recovery_wells",
    "flush_wells",
    "barcode_coa_custody_lands",
    "release_hold_reject_lanes",
];

const DECK_X: f64 = 1240.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;
const CONTAINMENT_RIM_W: f64 = 20.0;
const CONTAINMENT_RIM_Z: f64 = 44.0;
const SUMP_X: f64 = 1090.0;
const SUMP_Y: f64 = 660.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSS_COUNT: usize = 8;

const NEST_BANK_X: f64 = 430.0;
const NEST_BANK_Y: f64 = 280.0;
const NEST_BANK_Z: f64 = 42.0;
const NEST_BANK_POS: (f64, f64) = (-350.0, 215.0);
const ADDITIVE_VIAL_COUNT: usize = 12;
const VIAL_COLS: usize = 6;
const VIAL_ROWS: usize = 2;
const VIAL_WELL_D: f64 = 24.0;
const VIAL_PITCH_X: f64 = 45.0;
const VIAL_PITCH_Y: f64 = 55.0;
const BAG_NEST_COUNT: usize = 3;
const BAG_NEST_X: f64 = 98.0;
const BAG_NEST_Y: f64 = 172.0;
const BAG_NEST_RECESS_Z: f64 = 14.0;

const TOKEN_RAIL_X: f64 = 590.0;
const TOKEN_RAIL_Y: f64 = 155.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const TOKEN_RAIL_POS: (f64, f64) = (200.0, 260.0);
const ORDER_STEPS: usize = 8;
const TOKEN_SLOT_X: f64 = 44.0;
const TOKEN_SLOT_Y: f64 = 34.0;
const TOKEN_SLOT_Z: f64 = 9.0;
const TOKEN_PITCH_X: f64 = 60.0;
const TOKEN_RAIL_LANES: usize = 3;
const TOKEN_LANE_PITCH_Y: f64 = 43.0;

const MIX_LOOP_X: f64 = 725.0;
const MIX_LOOP_Y: f64 = 220.0;
const MIX_LOOP_Z: f64 = 46.0;
const MIX_LOOP_POS: (f64, f64) = (-115.0, 20.0);
const LOOP_CHANNEL_D: f64 = 7.0;
const STATIC_MIXER_BAFFLES: usize = 9;
const COUPON_COUNT: usize = 10;
const COUPON_X: f64 = 42.0;
const COUPON_Y: f64 = 24.0;
const COUPON_Z: f64 = 10.0;
const COUPON_PITCH_X: f64 = 54.0;
const MIXING_MIN_PASS_COUNT: usize = 3;

const SHIELD_X: f64 = 535.0;
const SHIELD_Y: f64 = 210.0;
const SHIELD_Z: f64 = 120.0;
const SHIELD_POS: (f64, f64) = (-385.0, -225.0);
const COLD_PACK_SLOTS: usize = 4;
const LIGHT_BAFFLES: usize = 5;
const TEMP_FLAG_POCKETS: usize = 4;

const BULKHEAD_X: f64 = 720.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 190.0;
const BULKHEAD_POS: (f64, f64) = (145.0, DECK_Y / 2.0 - 56.0);
const CONNECTOR_PORTS: usize = 9;
const CONNECTOR_PORT_D: f64 = 26.0;
const CONNECTOR_COLLAR_D: f64 = 44.0;
const CONNECTOR_PITCH_X: f64 = 70.0;
const KEYED_CAP_PARKS: usize = 9;

const WINDOW_BANK_X: f64 = 390.0;
const WINDOW_BANK_Y: f64 = 130.0;
const WINDOW_BANK_Z: f64 = 26.0;
const WINDOW_BANK_POS: (f64, f64) = (360.0, 28.0);
const BUBBLE_WINDOWS: usize = 6;
const WETNESS_WINDOWS: usize = 4;
const WINDOW_D: f64 = 30.0;

const WELL_BANK_X: f64 = 415.0;
const WELL_BANK_Y: f64 = 155.0;
const WELL_BANK_Z: f64 = 40.0;
const WELL_BANK_POS: (f64, f64) = (360.0, -185.0);
const RECOVERY_WELLS: usize = 6;
const FLUSH_WELLS: usize = 6;
const WELL_D: f64 = 28.0;
const WELL_PITCH_X: f64 = 45.0;
const WELL_PITCH_Y: f64 = 58.0;
const LOW_VOLUME_RECOVERY_ML: f64 = 0.6;

const CUSTODY_PANEL_X: f64 = 390.0;
const CUSTODY_PANEL_Y: f64 = 122.0;
const CUSTODY_PANEL_Z: f64 = 16.0;
const CUSTODY_PANEL_POS: (f64, f64) = (-380.0, -315.0);
const BARCODE_LANDS: usize = 8;
const COA_CARD_SLOTS: usize = 4;
const SEAL_WITNESS_PADS: usize = 4;

const LANE_BANK_X: f64 = 550.0;
const LANE_BANK_Y: f64 = 96.0;
const LANE_BANK_Z: f64 = 32.0;
const LANE_BANK_POS: (f64, f64) = (235.0, -325.0);
const DISPOSITION_LANES: usize = 3;
const TOKENS_PER_DISPOSITION: usize = 6;
const RELEASE_LANE_INDEX: usize = 0;
const HOLD_LANE_INDEX: usize = 1;
const REJECT_LANE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 860.0;
const CAMERA_BRIDGE_Y: f64 = 42.0;
const CAMERA_BRIDGE_Z: f64 = 156.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (50.0, -78.0);
const CAMERA_MOUNT_COUNT: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 8;
const ROBOT_KEEP_OUT_X: f64 = 1110.0;
const ROBOT_KEEP_OUT_Y: f64 = 95.0;
const ROBOT_KEEP_OUT_Z: f64 = 70.0;
const SERVICE_KEEP_OUT_X: f64 = 95.0;
const SERVICE_KEEP_OUT_Y: f64 = 650.0;
const SERVICE_KEEP_OUT_Z: f64 = 90.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 280.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(staged_additive_vial_bag_nests(), OUTPUTS[1]);
    write_part(order_token_rails(), OUTPUTS[2]);
    write_part(mixing_witness_loop_coupon_bank(), OUTPUTS[3]);
    write_part(cold_light_shield_features(), OUTPUTS[4]);
    write_part(sterile_connector_bulkhead(), OUTPUTS[5]);
    write_part(bubble_wetness_windows(), OUTPUTS[6]);
    write_part(recovery_flush_wells(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_lanes(), OUTPUTS[9]);
    write_part(camera_evidence_bridge_keepout_gauges(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed reagent addition-order/mixing witness station: {:.0}mm x {:.0}mm contained deck, {} additive vial wells, {} bag nests, {} ordered recipe steps, {} sterile connector ports.",
        DECK_X,
        DECK_Y,
        ADDITIVE_VIAL_COUNT,
        BAG_NEST_COUNT,
        ORDER_STEPS,
        CONNECTOR_PORTS
    );
    println!(
        "Witness controls: {} static mixer baffles, {} coupons, {} bubble windows, {} wetness windows, {} recovery wells, {} flush wells, {:.1}mL low-volume recovery target.",
        STATIC_MIXER_BAFFLES,
        COUPON_COUNT,
        BUBBLE_WINDOWS,
        WETNESS_WINDOWS,
        RECOVERY_WELLS,
        FLUSH_WELLS,
        LOW_VOLUME_RECOVERY_ML
    );
    println!(
        "Evidence and disposition: {} barcode lands, {} COA card slots, {} release/hold/reject lanes, {} camera mounts, top service clearance {:.0}mm.",
        BARCODE_LANDS,
        COA_CARD_SLOTS,
        DISPOSITION_LANES,
        CAMERA_MOUNT_COUNT,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + staged_additive_vial_bag_nests()
        + order_token_rails()
        + mixing_witness_loop_coupon_bank()
        + cold_light_shield_features()
        + sterile_connector_bulkhead()
        + bubble_wetness_windows()
        + recovery_flush_wells()
        + barcode_coa_custody_lands()
        + release_hold_reject_lanes()
        + camera_evidence_bridge_keepout_gauges()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "reagent_order_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "reagent_order_station_recessed_spill_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -6.0, DECK_Z / 2.0 - SUMP_Z / 2.0);

    let drain = centered_cylinder(
        "reagent_order_station_sump_drain_cut",
        DRAIN_D / 2.0,
        CONTAINMENT_RIM_W + 26.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 105.0, -DECK_Y / 2.0 + 13.0, 0.0);

    deck - sump - drain + containment_rims() + datum_bosses() + station_zone_markers()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "reagent_order_station_front_containment_rim",
        DECK_X,
        CONTAINMENT_RIM_W,
        CONTAINMENT_RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + CONTAINMENT_RIM_W / 2.0, rim_z());
    let rear = centered_cube(
        "reagent_order_station_rear_containment_rim",
        DECK_X,
        CONTAINMENT_RIM_W,
        CONTAINMENT_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - CONTAINMENT_RIM_W / 2.0, rim_z());
    let left = centered_cube(
        "reagent_order_station_left_containment_rim",
        CONTAINMENT_RIM_W,
        DECK_Y,
        CONTAINMENT_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + CONTAINMENT_RIM_W / 2.0, 0.0, rim_z());
    let right = centered_cube(
        "reagent_order_station_right_containment_rim",
        CONTAINMENT_RIM_W,
        DECK_Y,
        CONTAINMENT_RIM_Z,
    )
    .translate(DECK_X / 2.0 - CONTAINMENT_RIM_W / 2.0, 0.0, rim_z());

    front + rear + left + right
}

fn datum_bosses() -> Part {
    let mut bosses = Part::empty("reagent_order_station_datum_bosses");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("reagent_order_station_datum_boss_{i}"),
            14.0,
            10.0,
            32,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        let hole = centered_cylinder(
            format!("reagent_order_station_datum_bore_{i}"),
            4.2,
            12.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn station_zone_markers() -> Part {
    let mut markers = Part::empty("reagent_order_station_zone_markers");
    for (i, (x, y, sx, sy)) in [
        (
            NEST_BANK_POS.0,
            NEST_BANK_POS.1,
            NEST_BANK_X + 22.0,
            NEST_BANK_Y + 22.0,
        ),
        (
            TOKEN_RAIL_POS.0,
            TOKEN_RAIL_POS.1,
            TOKEN_RAIL_X + 20.0,
            TOKEN_RAIL_Y + 20.0,
        ),
        (
            MIX_LOOP_POS.0,
            MIX_LOOP_POS.1,
            MIX_LOOP_X + 26.0,
            MIX_LOOP_Y + 24.0,
        ),
        (SHIELD_POS.0, SHIELD_POS.1, SHIELD_X + 24.0, SHIELD_Y + 22.0),
        (
            WINDOW_BANK_POS.0,
            WINDOW_BANK_POS.1,
            WINDOW_BANK_X + 18.0,
            WINDOW_BANK_Y + 18.0,
        ),
        (
            WELL_BANK_POS.0,
            WELL_BANK_POS.1,
            WELL_BANK_X + 18.0,
            WELL_BANK_Y + 18.0,
        ),
        (
            CUSTODY_PANEL_POS.0,
            CUSTODY_PANEL_POS.1,
            CUSTODY_PANEL_X + 18.0,
            CUSTODY_PANEL_Y + 16.0,
        ),
        (
            LANE_BANK_POS.0,
            LANE_BANK_POS.1,
            LANE_BANK_X + 18.0,
            LANE_BANK_Y + 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + centered_cube(
                format!("reagent_order_station_recess_zone_marker_{i}"),
                *sx,
                *sy,
                2.4,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 1.2);
    }
    markers
}

fn staged_additive_vial_bag_nests() -> Part {
    let base = centered_cube(
        "reagent_order_station_staged_additive_nest_bank",
        NEST_BANK_X,
        NEST_BANK_Y,
        NEST_BANK_Z,
    )
    .translate(NEST_BANK_POS.0, NEST_BANK_POS.1, top_z(NEST_BANK_Z));

    let mut cuts = Part::empty("reagent_order_station_additive_nest_cuts");
    for idx in 0..ADDITIVE_VIAL_COUNT {
        let (x, y) = vial_position(idx);
        cuts = cuts
            + centered_cylinder(
                format!("reagent_order_station_additive_vial_well_{idx}"),
                VIAL_WELL_D / 2.0,
                NEST_BANK_Z + 2.0,
                40,
            )
            .translate(NEST_BANK_POS.0 + x, NEST_BANK_POS.1 + y, top_z(NEST_BANK_Z));
    }
    for idx in 0..BAG_NEST_COUNT {
        let x = -118.0 + idx as f64 * 118.0;
        cuts = cuts
            + centered_cube(
                format!("reagent_order_station_additive_bag_recess_{idx}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_NEST_RECESS_Z + 1.0,
            )
            .translate(
                NEST_BANK_POS.0 + x,
                NEST_BANK_POS.1 - 58.0,
                top_z(NEST_BANK_Z) + NEST_BANK_Z / 2.0 - BAG_NEST_RECESS_Z / 2.0,
            );
    }

    let mut clips = Part::empty("reagent_order_station_bag_edge_clips");
    for idx in 0..BAG_NEST_COUNT {
        let x = NEST_BANK_POS.0 - 118.0 + idx as f64 * 118.0;
        clips = clips
            + centered_cube(
                format!("reagent_order_station_bag_clip_front_{idx}"),
                BAG_NEST_X,
                10.0,
                16.0,
            )
            .translate(x, NEST_BANK_POS.1 - 150.0, DECK_Z / 2.0 + NEST_BANK_Z + 8.0)
            + centered_cube(
                format!("reagent_order_station_bag_clip_rear_{idx}"),
                BAG_NEST_X,
                10.0,
                16.0,
            )
            .translate(x, NEST_BANK_POS.1 + 34.0, DECK_Z / 2.0 + NEST_BANK_Z + 8.0);
    }

    base - cuts + clips + additive_stage_numbers()
}

fn additive_stage_numbers() -> Part {
    let mut ticks = Part::empty("reagent_order_station_additive_stage_order_ticks");
    for idx in 0..ADDITIVE_VIAL_COUNT {
        let (x, y) = vial_position(idx);
        ticks = ticks
            + centered_cube(
                format!("reagent_order_station_additive_order_tick_{idx}"),
                18.0,
                3.5,
                3.0,
            )
            .translate(
                NEST_BANK_POS.0 + x,
                NEST_BANK_POS.1 + y + 21.0,
                DECK_Z / 2.0 + NEST_BANK_Z + 1.5,
            );
    }
    ticks
}

fn order_token_rails() -> Part {
    let rail = centered_cube(
        "reagent_order_station_order_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, top_z(TOKEN_RAIL_Z));

    let mut slots = Part::empty("reagent_order_station_order_token_slot_cuts");
    let start_x = -(ORDER_STEPS as f64 - 1.0) * TOKEN_PITCH_X / 2.0;
    let start_y = -(TOKEN_RAIL_LANES as f64 - 1.0) * TOKEN_LANE_PITCH_Y / 2.0;
    for lane in 0..TOKEN_RAIL_LANES {
        for step in 0..ORDER_STEPS {
            slots = slots
                + centered_cube(
                    format!("reagent_order_station_order_token_slot_lane_{lane}_step_{step}"),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_SLOT_Z + 1.0,
                )
                .translate(
                    TOKEN_RAIL_POS.0 + start_x + step as f64 * TOKEN_PITCH_X,
                    TOKEN_RAIL_POS.1 + start_y + lane as f64 * TOKEN_LANE_PITCH_Y,
                    top_z(TOKEN_RAIL_Z) + TOKEN_RAIL_Z / 2.0 - TOKEN_SLOT_Z / 2.0,
                );
        }
    }

    let mut gates = Part::empty("reagent_order_station_order_interlock_gates");
    for step in 0..=ORDER_STEPS {
        let x = TOKEN_RAIL_POS.0 + start_x - TOKEN_PITCH_X / 2.0 + step as f64 * TOKEN_PITCH_X;
        gates = gates
            + centered_cube(
                format!("reagent_order_station_order_interlock_gate_{step}"),
                5.0,
                TOKEN_RAIL_Y + 16.0,
                22.0,
            )
            .translate(x, TOKEN_RAIL_POS.1, DECK_Z / 2.0 + TOKEN_RAIL_Z + 11.0);
    }

    rail - slots + gates
}

fn mixing_witness_loop_coupon_bank() -> Part {
    let plate = centered_cube(
        "reagent_order_station_mixing_witness_loop_plate",
        MIX_LOOP_X,
        MIX_LOOP_Y,
        MIX_LOOP_Z,
    )
    .translate(MIX_LOOP_POS.0, MIX_LOOP_POS.1, top_z(MIX_LOOP_Z));

    let loop_cuts = witness_loop_channels();
    let baffles = static_mixer_baffles();
    let coupons = mixing_coupon_bank();
    let pass_counter = centered_cube(
        "reagent_order_station_three_pass_counter_window",
        104.0,
        36.0,
        12.0,
    )
    .translate(
        MIX_LOOP_POS.0 + MIX_LOOP_X / 2.0 - 78.0,
        MIX_LOOP_POS.1 - MIX_LOOP_Y / 2.0 + 40.0,
        DECK_Z / 2.0 + MIX_LOOP_Z + 6.0,
    );

    plate - loop_cuts + baffles + coupons + pass_counter
}

fn witness_loop_channels() -> Part {
    let mut cuts = Part::empty("reagent_order_station_closed_witness_loop_channel_cuts");
    for (i, (x, y, sx, sy)) in [
        (-250.0, 78.0, 372.0, LOOP_CHANNEL_D),
        (-64.0, 28.0, LOOP_CHANNEL_D, 104.0),
        (52.0, -24.0, 232.0, LOOP_CHANNEL_D),
        (168.0, 28.0, LOOP_CHANNEL_D, 104.0),
        (264.0, 78.0, 192.0, LOOP_CHANNEL_D),
        (-250.0, -84.0, 372.0, LOOP_CHANNEL_D),
        (264.0, -84.0, 192.0, LOOP_CHANNEL_D),
    ]
    .iter()
    .enumerate()
    {
        cuts = cuts
            + centered_cube(
                format!("reagent_order_station_witness_loop_channel_{i}"),
                *sx,
                *sy,
                MIX_LOOP_Z + 2.0,
            )
            .translate(MIX_LOOP_POS.0 + x, MIX_LOOP_POS.1 + y, top_z(MIX_LOOP_Z));
    }
    for (i, (x, y)) in [(-64.0, 78.0), (-64.0, -24.0), (168.0, -24.0), (168.0, 78.0)]
        .iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                format!("reagent_order_station_witness_loop_turn_{i}"),
                18.0,
                MIX_LOOP_Z + 2.0,
                40,
            )
            .translate(MIX_LOOP_POS.0 + x, MIX_LOOP_POS.1 + y, top_z(MIX_LOOP_Z));
    }
    cuts
}

fn static_mixer_baffles() -> Part {
    let mut baffles = Part::empty("reagent_order_station_static_mixer_baffles");
    for i in 0..STATIC_MIXER_BAFFLES {
        let x = MIX_LOOP_POS.0 - 260.0 + i as f64 * 42.0;
        let angle = if i % 2 == 0 { 32.0 } else { -32.0 };
        baffles = baffles
            + centered_cube(
                format!("reagent_order_station_static_mixer_baffle_{i}"),
                8.0,
                48.0,
                18.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(x, MIX_LOOP_POS.1 + 78.0, DECK_Z / 2.0 + MIX_LOOP_Z + 9.0);
    }
    baffles
}

fn mixing_coupon_bank() -> Part {
    let mut bank = Part::empty("reagent_order_station_mixing_coupon_bank");
    let start_x = -(COUPON_COUNT as f64 - 1.0) * COUPON_PITCH_X / 2.0;
    for i in 0..COUPON_COUNT {
        let coupon = centered_cube(
            format!("reagent_order_station_mixing_coupon_{i}"),
            COUPON_X,
            COUPON_Y,
            COUPON_Z,
        )
        .translate(
            MIX_LOOP_POS.0 + start_x + i as f64 * COUPON_PITCH_X,
            MIX_LOOP_POS.1 - 84.0,
            DECK_Z / 2.0 + MIX_LOOP_Z + COUPON_Z / 2.0,
        );
        let sample_dot = centered_cylinder(
            format!("reagent_order_station_mixing_coupon_scan_dot_{i}"),
            4.5,
            COUPON_Z + 2.0,
            20,
        )
        .translate(
            MIX_LOOP_POS.0 + start_x + i as f64 * COUPON_PITCH_X,
            MIX_LOOP_POS.1 - 84.0,
            DECK_Z / 2.0 + MIX_LOOP_Z + COUPON_Z / 2.0,
        );
        bank = bank + (coupon - sample_dot);
    }
    bank
}

fn cold_light_shield_features() -> Part {
    let rear_wall = centered_cube(
        "reagent_order_station_rear_light_shield_wall",
        SHIELD_X,
        18.0,
        SHIELD_Z,
    )
    .translate(
        SHIELD_POS.0,
        SHIELD_POS.1 + SHIELD_Y / 2.0 - 9.0,
        DECK_Z / 2.0 + SHIELD_Z / 2.0,
    );
    let left_wall = centered_cube(
        "reagent_order_station_left_light_shield_wall",
        18.0,
        SHIELD_Y,
        SHIELD_Z,
    )
    .translate(
        SHIELD_POS.0 - SHIELD_X / 2.0 + 9.0,
        SHIELD_POS.1,
        DECK_Z / 2.0 + SHIELD_Z / 2.0,
    );
    let right_wall = centered_cube(
        "reagent_order_station_right_light_shield_wall",
        18.0,
        SHIELD_Y,
        SHIELD_Z,
    )
    .translate(
        SHIELD_POS.0 + SHIELD_X / 2.0 - 9.0,
        SHIELD_POS.1,
        DECK_Z / 2.0 + SHIELD_Z / 2.0,
    );
    let roof = centered_cube(
        "reagent_order_station_amber_light_shield_roof",
        SHIELD_X,
        SHIELD_Y,
        12.0,
    )
    .translate(SHIELD_POS.0, SHIELD_POS.1, DECK_Z / 2.0 + SHIELD_Z + 6.0);

    rear_wall + left_wall + right_wall + roof + cold_pack_slots() + light_baffles() + temp_flags()
}

fn cold_pack_slots() -> Part {
    let mut slots = Part::empty("reagent_order_station_cold_pack_slots");
    for i in 0..COLD_PACK_SLOTS {
        slots = slots
            + centered_cube(
                format!("reagent_order_station_cold_pack_slot_{i}"),
                76.0,
                124.0,
                16.0,
            )
            .translate(
                SHIELD_POS.0 - 170.0 + i as f64 * 112.0,
                SHIELD_POS.1 - 18.0,
                DECK_Z / 2.0 + 8.0,
            );
    }
    slots
}

fn light_baffles() -> Part {
    let mut baffles = Part::empty("reagent_order_station_light_exposure_baffles");
    for i in 0..LIGHT_BAFFLES {
        baffles = baffles
            + centered_cube(
                format!("reagent_order_station_light_baffle_{i}"),
                10.0,
                SHIELD_Y - 30.0,
                SHIELD_Z - 30.0,
            )
            .translate(
                SHIELD_POS.0 - 190.0 + i as f64 * 95.0,
                SHIELD_POS.1,
                DECK_Z / 2.0 + SHIELD_Z / 2.0,
            );
    }
    baffles
}

fn temp_flags() -> Part {
    let mut flags = Part::empty("reagent_order_station_temperature_exposure_flag_pockets");
    for i in 0..TEMP_FLAG_POCKETS {
        flags = flags
            + centered_cube(
                format!("reagent_order_station_temperature_flag_pocket_{i}"),
                58.0,
                18.0,
                10.0,
            )
            .translate(
                SHIELD_POS.0 - 165.0 + i as f64 * 110.0,
                SHIELD_POS.1 - SHIELD_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + 18.0,
            );
    }
    flags
}

fn sterile_connector_bulkhead() -> Part {
    let wall = centered_cube(
        "reagent_order_station_sterile_connector_bulkhead_wall",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(
        BULKHEAD_POS.0,
        BULKHEAD_POS.1,
        DECK_Z / 2.0 + BULKHEAD_Z / 2.0,
    );

    let mut cuts = Part::empty("reagent_order_station_bulkhead_connector_cuts");
    let mut collars = Part::empty("reagent_order_station_bulkhead_connector_collars");
    let start_x = -(CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X / 2.0;
    for i in 0..CONNECTOR_PORTS {
        let x = BULKHEAD_POS.0 + start_x + i as f64 * CONNECTOR_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("reagent_order_station_connector_port_cut_{i}"),
                CONNECTOR_PORT_D / 2.0,
                BULKHEAD_Y + 4.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_POS.1, DECK_Z / 2.0 + 98.0);
        collars = collars
            + centered_cylinder(
                format!("reagent_order_station_connector_keyed_collar_{i}"),
                CONNECTOR_COLLAR_D / 2.0,
                10.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                BULKHEAD_POS.1 - BULKHEAD_Y / 2.0 - 5.0,
                DECK_Z / 2.0 + 98.0,
            );
    }

    wall - cuts + collars + keyed_cap_parks()
}

fn keyed_cap_parks() -> Part {
    let mut parks = Part::empty("reagent_order_station_keyed_cap_parks");
    let start_x = -(KEYED_CAP_PARKS as f64 - 1.0) * CONNECTOR_PITCH_X / 2.0;
    for i in 0..KEYED_CAP_PARKS {
        parks = parks
            + centered_cylinder(
                format!("reagent_order_station_keyed_cap_park_{i}"),
                13.0,
                14.0,
                28,
            )
            .translate(
                BULKHEAD_POS.0 + start_x + i as f64 * CONNECTOR_PITCH_X,
                BULKHEAD_POS.1 - 60.0,
                DECK_Z / 2.0 + 20.0,
            );
    }
    parks
}

fn bubble_wetness_windows() -> Part {
    let bank = centered_cube(
        "reagent_order_station_bubble_wetness_window_bank",
        WINDOW_BANK_X,
        WINDOW_BANK_Y,
        WINDOW_BANK_Z,
    )
    .translate(WINDOW_BANK_POS.0, WINDOW_BANK_POS.1, top_z(WINDOW_BANK_Z));

    let mut cuts = Part::empty("reagent_order_station_bubble_wetness_window_cuts");
    for i in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cylinder(
                format!("reagent_order_station_bubble_window_{i}"),
                WINDOW_D / 2.0,
                WINDOW_BANK_Z + 2.0,
                36,
            )
            .translate(
                WINDOW_BANK_POS.0 - 142.0 + i as f64 * 57.0,
                WINDOW_BANK_POS.1 + 24.0,
                top_z(WINDOW_BANK_Z),
            );
    }
    for i in 0..WETNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("reagent_order_station_wetness_window_{i}"),
                48.0,
                18.0,
                WINDOW_BANK_Z + 2.0,
            )
            .translate(
                WINDOW_BANK_POS.0 - 95.0 + i as f64 * 64.0,
                WINDOW_BANK_POS.1 - 36.0,
                top_z(WINDOW_BANK_Z),
            );
    }

    bank - cuts + window_backlight_ledges()
}

fn window_backlight_ledges() -> Part {
    let mut ledges = Part::empty("reagent_order_station_window_backlight_ledges");
    for i in 0..BUBBLE_WINDOWS {
        ledges = ledges
            + centered_cube(
                format!("reagent_order_station_bubble_window_backlight_ledge_{i}"),
                38.0,
                8.0,
                8.0,
            )
            .translate(
                WINDOW_BANK_POS.0 - 142.0 + i as f64 * 57.0,
                WINDOW_BANK_POS.1 + 55.0,
                DECK_Z / 2.0 + WINDOW_BANK_Z + 4.0,
            );
    }
    ledges
}

fn recovery_flush_wells() -> Part {
    let bank = centered_cube(
        "reagent_order_station_recovery_flush_well_bank",
        WELL_BANK_X,
        WELL_BANK_Y,
        WELL_BANK_Z,
    )
    .translate(WELL_BANK_POS.0, WELL_BANK_POS.1, top_z(WELL_BANK_Z));

    let mut cuts = Part::empty("reagent_order_station_recovery_flush_well_cuts");
    for row in 0..2 {
        for col in 0..RECOVERY_WELLS {
            let y = if row == 0 {
                WELL_PITCH_Y / 2.0
            } else {
                -WELL_PITCH_Y / 2.0
            };
            cuts = cuts
                + centered_cylinder(
                    format!("reagent_order_station_recovery_flush_well_row_{row}_col_{col}"),
                    WELL_D / 2.0,
                    WELL_BANK_Z + 2.0,
                    36,
                )
                .translate(
                    WELL_BANK_POS.0 - (RECOVERY_WELLS as f64 - 1.0) * WELL_PITCH_X / 2.0
                        + col as f64 * WELL_PITCH_X,
                    WELL_BANK_POS.1 + y,
                    top_z(WELL_BANK_Z),
                );
        }
    }

    let low_volume_weir = centered_cube(
        "reagent_order_station_low_volume_recovery_weir_gauge",
        WELL_BANK_X - 40.0,
        6.0,
        16.0,
    )
    .translate(
        WELL_BANK_POS.0,
        WELL_BANK_POS.1,
        DECK_Z / 2.0 + WELL_BANK_Z + 8.0,
    );

    bank - cuts + low_volume_weir
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "reagent_order_station_barcode_coa_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_PANEL_POS.0,
        CUSTODY_PANEL_POS.1,
        top_z(CUSTODY_PANEL_Z),
    );

    let mut lands = Part::empty("reagent_order_station_barcode_coa_custody_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("reagent_order_station_barcode_land_{i}"),
                72.0,
                22.0,
                3.0,
            )
            .translate(
                CUSTODY_PANEL_POS.0 - 147.0 + (i % 4) as f64 * 98.0,
                CUSTODY_PANEL_POS.1 + 34.0 - (i / 4) as f64 * 38.0,
                DECK_Z / 2.0 + CUSTODY_PANEL_Z + 1.5,
            );
    }
    for i in 0..COA_CARD_SLOTS {
        lands = lands
            + centered_cube(
                format!("reagent_order_station_coa_card_slot_{i}"),
                70.0,
                4.0,
                12.0,
            )
            .translate(
                CUSTODY_PANEL_POS.0 - 145.0 + i as f64 * 96.0,
                CUSTODY_PANEL_POS.1 - 44.0,
                DECK_Z / 2.0 + CUSTODY_PANEL_Z + 6.0,
            );
    }
    for i in 0..SEAL_WITNESS_PADS {
        lands = lands
            + centered_cylinder(
                format!("reagent_order_station_custody_seal_witness_pad_{i}"),
                9.0,
                4.0,
                24,
            )
            .translate(
                CUSTODY_PANEL_POS.0 - 144.0 + i as f64 * 96.0,
                CUSTODY_PANEL_POS.1 - 12.0,
                DECK_Z / 2.0 + CUSTODY_PANEL_Z + 2.0,
            );
    }

    panel + lands
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "reagent_order_station_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(LANE_BANK_POS.0, LANE_BANK_POS.1, top_z(LANE_BANK_Z));

    let mut cuts = Part::empty("reagent_order_station_disposition_lane_token_cuts");
    let lane_pitch_y = LANE_BANK_Y / DISPOSITION_LANES as f64;
    for lane in 0..DISPOSITION_LANES {
        for token in 0..TOKENS_PER_DISPOSITION {
            cuts = cuts
                + centered_cube(
                    format!("reagent_order_station_disposition_lane_{lane}_token_{token}"),
                    54.0,
                    18.0,
                    10.0,
                )
                .translate(
                    LANE_BANK_POS.0 - 188.0 + token as f64 * 75.0,
                    LANE_BANK_POS.1 - lane_pitch_y + lane as f64 * lane_pitch_y,
                    top_z(LANE_BANK_Z) + LANE_BANK_Z / 2.0 - 5.0,
                );
        }
    }

    let release_gate = disposition_lane_marker(RELEASE_LANE_INDEX, "release");
    let hold_gate = disposition_lane_marker(HOLD_LANE_INDEX, "hold");
    let reject_gate = disposition_lane_marker(REJECT_LANE_INDEX, "reject");

    bank - cuts + release_gate + hold_gate + reject_gate
}

fn disposition_lane_marker(lane: usize, label: &str) -> Part {
    let lane_pitch_y = LANE_BANK_Y / DISPOSITION_LANES as f64;
    centered_cube(
        format!("reagent_order_station_{label}_lane_positive_stop"),
        18.0,
        lane_pitch_y - 8.0,
        24.0,
    )
    .translate(
        LANE_BANK_POS.0 + LANE_BANK_X / 2.0 - 28.0,
        LANE_BANK_POS.1 - lane_pitch_y + lane as f64 * lane_pitch_y,
        DECK_Z / 2.0 + LANE_BANK_Z + 12.0,
    )
}

fn camera_evidence_bridge_keepout_gauges() -> Part {
    let bridge = centered_cube(
        "reagent_order_station_camera_evidence_bridge",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );

    let mut camera_mounts = Part::empty("reagent_order_station_camera_mounts");
    for i in 0..CAMERA_MOUNT_COUNT {
        camera_mounts = camera_mounts
            + centered_cube(
                format!("reagent_order_station_camera_mount_plate_{i}"),
                64.0,
                12.0,
                38.0,
            )
            .translate(
                CAMERA_BRIDGE_POS.0 - 315.0 + i as f64 * 210.0,
                CAMERA_BRIDGE_POS.1 - CAMERA_BRIDGE_Y / 2.0 - 6.0,
                DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 28.0,
            );
    }

    bridge + camera_mounts + evidence_fiducials() + robot_service_keepout_gauges()
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("reagent_order_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = if i % 2 == 0 { -1.0 } else { 1.0 } * (DECK_X / 2.0 - 70.0);
        let y = -300.0 + (i / 2) as f64 * 180.0;
        let puck = centered_cylinder(
            format!("reagent_order_station_evidence_fiducial_puck_{i}"),
            10.0,
            5.0,
            32,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("reagent_order_station_evidence_fiducial_center_{i}"),
            3.5,
            6.0,
            20,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        fiducials = fiducials + (puck - bore);
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "reagent_order_station_front_robot_approach_keepout_gauge",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 80.0,
        DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "reagent_order_station_left_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 78.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "reagent_order_station_right_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - 78.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "reagent_order_station_top_service_clearance_gauge",
        220.0,
        160.0,
        12.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 + TOP_SERVICE_CLEARANCE_Z);

    front_robot + left_service + right_service + top_clearance
}

fn datum_positions() -> [(f64, f64); DATUM_BOSS_COUNT] {
    [
        (-540.0, -345.0),
        (-180.0, -345.0),
        (180.0, -345.0),
        (540.0, -345.0),
        (-540.0, 345.0),
        (-180.0, 345.0),
        (180.0, 345.0),
        (540.0, 345.0),
    ]
}

fn vial_position(idx: usize) -> (f64, f64) {
    let col = idx % VIAL_COLS;
    let row = idx / VIAL_COLS;
    (
        -((VIAL_COLS - 1) as f64) * VIAL_PITCH_X / 2.0 + col as f64 * VIAL_PITCH_X,
        70.0 - ((VIAL_ROWS - 1) as f64) * VIAL_PITCH_Y / 2.0 + row as f64 * VIAL_PITCH_Y,
    )
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + CONTAINMENT_RIM_Z / 2.0
}

fn assert_layout() {
    assert_eq!(ADDITIVE_VIAL_COUNT, VIAL_COLS * VIAL_ROWS);
    assert!(NEST_BANK_POS.0 - NEST_BANK_X / 2.0 > -DECK_X / 2.0 + CONTAINMENT_RIM_W + 20.0);
    assert!(TOKEN_RAIL_POS.0 + TOKEN_RAIL_X / 2.0 < DECK_X / 2.0 - CONTAINMENT_RIM_W - 20.0);
    assert!(BULKHEAD_POS.1 + BULKHEAD_Y / 2.0 < DECK_Y / 2.0 - CONTAINMENT_RIM_W);
    assert!(connector_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
    assert!(CUSTODY_PANEL_POS.1 - CUSTODY_PANEL_Y / 2.0 > -DECK_Y / 2.0 + CONTAINMENT_RIM_W + 4.0);
    assert!(LANE_BANK_POS.1 - LANE_BANK_Y / 2.0 > -DECK_Y / 2.0 + CONTAINMENT_RIM_W + 4.0);
    assert!(WINDOW_BANK_POS.0 + WINDOW_BANK_X / 2.0 < DECK_X / 2.0 - CONTAINMENT_RIM_W);
    assert!(MIXING_MIN_PASS_COUNT >= 3);
}

fn connector_span_x() -> f64 {
    (CONNECTOR_PORTS - 1) as f64 * CONNECTOR_PITCH_X
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_reagent_addition_order_mixing_witness_station_")
            );
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_validation_intent_is_explicit() {
        assert!(REQUIRED_INTENT_FEATURES.contains(&"order_token_rails"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"mixing_witness_loop"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"cold_light_shield"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"bubble_windows"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"wetness_windows"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"barcode_coa_custody_lands"));
    }

    #[test]
    fn additive_order_and_connector_capacity_match_recipe_witnessing() {
        assert_eq!(ORDER_STEPS, 8);
        assert_eq!(TOKEN_RAIL_LANES, DISPOSITION_LANES);
        assert!(ADDITIVE_VIAL_COUNT + BAG_NEST_COUNT >= ORDER_STEPS);
        assert!(CONNECTOR_PORTS >= BAG_NEST_COUNT + 2 * MIXING_MIN_PASS_COUNT);
    }

    #[test]
    fn mixing_and_recovery_checks_have_redundant_evidence() {
        assert!(STATIC_MIXER_BAFFLES >= ORDER_STEPS);
        assert!(COUPON_COUNT >= BUBBLE_WINDOWS + WETNESS_WINDOWS);
        assert_eq!(RECOVERY_WELLS, FLUSH_WELLS);
        assert!(LOW_VOLUME_RECOVERY_ML <= 1.0);
    }

    #[test]
    fn layout_guards_hold() {
        assert_layout();
    }
}
