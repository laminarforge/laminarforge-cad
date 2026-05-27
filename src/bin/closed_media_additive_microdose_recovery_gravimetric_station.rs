use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media additive microdose recovery gravimetric validation station.
//
// Design intent:
// - Package additive vial and bag nests, collection wells, balance/load-cell
//   references, flush/recovery routing, low-dead-volume adapter coupons, custody
//   lands, evidence capture, and disposition gates on one contained deck.
// - Provide mechanical validation packaging for checking recovered microdose
//   mass and station evidence without defining a dosing SOP, biological
//   acceptance criterion, or sterile-process claim.
// - Keep all modeled features as fixture geometry, keepouts, envelopes, witness
//   lands, and removable coupons for downstream mechanical review.

const OUTPUTS: [&str; 14] = [
    "output/closed_media_additive_microdose_recovery_gravimetric_station_containment_deck.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_additive_vial_bag_nests.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_microdose_collection_wells.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_gravimetric_balance_load_cell_pad.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_flush_recovery_route.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_low_dead_volume_adapter_coupons.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_evaporation_cover.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_high_low_standard_lanes.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_barcode_coa_custody_lands.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_waste_capture.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_release_hold_reject_gates.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_camera_evidence_bridge.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_robot_service_keepouts.stl",
    "output/closed_media_additive_microdose_recovery_gravimetric_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 19] = [
    "mechanical_validation_packaging_only",
    "additive_vial_nests",
    "additive_bag_nests",
    "microdose_collection_wells",
    "gravimetric_balance_pad",
    "load_cell_isolation_moat",
    "flush_recovery_route",
    "low_dead_volume_adapter_coupons",
    "evaporation_cover",
    "high_standard_lane",
    "low_standard_lane",
    "barcode_land",
    "coa_land",
    "waste_capture",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 840.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 46.0;
const SUMP_X: f64 = 1110.0;
const SUMP_Y: f64 = 665.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 8;

const NEST_BANK_X: f64 = 430.0;
const NEST_BANK_Y: f64 = 240.0;
const NEST_BANK_Z: f64 = 42.0;
const NEST_BANK_POS: (f64, f64) = (-390.0, 250.0);
const ADDITIVE_VIAL_NESTS: usize = 12;
const VIAL_COLS: usize = 6;
const VIAL_ROWS: usize = 2;
const VIAL_WELL_D: f64 = 24.0;
const VIAL_PITCH_X: f64 = 47.0;
const VIAL_PITCH_Y: f64 = 58.0;
const ADDITIVE_BAG_NESTS: usize = 4;
const BAG_NEST_X: f64 = 84.0;
const BAG_NEST_Y: f64 = 150.0;
const BAG_RECESS_Z: f64 = 14.0;
const ADDITIVE_RETAINING_CLIPS: usize = 8;

const COLLECTION_BANK_X: f64 = 470.0;
const COLLECTION_BANK_Y: f64 = 220.0;
const COLLECTION_BANK_Z: f64 = 44.0;
const COLLECTION_BANK_POS: (f64, f64) = (280.0, 255.0);
const MICRODOSE_WELLS: usize = 16;
const MICRODOSE_COLS: usize = 8;
const MICRODOSE_ROWS: usize = 2;
const MICRODOSE_WELL_D: f64 = 21.0;
const MICRODOSE_PITCH_X: f64 = 49.0;
const MICRODOSE_PITCH_Y: f64 = 66.0;
const WELL_RIM_D: f64 = 28.0;
const WELL_VOLUME_UL: f64 = 850.0;
const EVAPORATION_FLAG_PADS: usize = 4;

const BALANCE_PAD_X: f64 = 500.0;
const BALANCE_PAD_Y: f64 = 260.0;
const BALANCE_PAD_Z: f64 = 36.0;
const BALANCE_PAD_POS: (f64, f64) = (-60.0, 20.0);
const BALANCE_PAN_D: f64 = 138.0;
const LOAD_CELL_POCKET_X: f64 = 188.0;
const LOAD_CELL_POCKET_Y: f64 = 82.0;
const LOAD_CELL_POCKET_Z: f64 = 18.0;
const ISOLATION_MOAT_X: f64 = 315.0;
const ISOLATION_MOAT_Y: f64 = 174.0;
const ISOLATION_MOAT_W: f64 = 11.0;
const LEVELING_FEET: usize = 4;
const DRAFT_POSTS: usize = 4;
const BALANCE_CABLE_TROUGH_X: f64 = 96.0;
const BALANCE_RESOLUTION_MG: f64 = 1.0;

const ROUTE_PLATE_X: f64 = 570.0;
const ROUTE_PLATE_Y: f64 = 160.0;
const ROUTE_PLATE_Z: f64 = 38.0;
const ROUTE_PLATE_POS: (f64, f64) = (-315.0, -175.0);
const ROUTE_CHANNEL_D: f64 = 6.0;
const RECOVERY_CHANNELS: usize = 2;
const FLUSH_CHANNELS: usize = 2;
const ROUTE_PORTS: usize = 8;
const ROUTE_PORT_D: f64 = 13.0;
const ROUTE_VALVE_SEATS: usize = 6;
const RECOVERY_TRAP_CUPS: usize = 4;

const COUPON_BANK_X: f64 = 430.0;
const COUPON_BANK_Y: f64 = 180.0;
const COUPON_BANK_Z: f64 = 34.0;
const COUPON_BANK_POS: (f64, f64) = (300.0, -70.0);
const ADAPTER_COUPONS: usize = 12;
const COUPON_COLS: usize = 6;
const COUPON_ROWS: usize = 2;
const COUPON_X: f64 = 46.0;
const COUPON_Y: f64 = 34.0;
const COUPON_Z: f64 = 10.0;
const COUPON_PITCH_X: f64 = 57.0;
const COUPON_PITCH_Y: f64 = 62.0;
const LDV_BORE_D: f64 = 3.2;
const COUPON_DEAD_VOLUME_TARGET_UL: f64 = 25.0;

const COVER_X: f64 = 785.0;
const COVER_Y: f64 = 360.0;
const COVER_Z: f64 = 84.0;
const COVER_POS: (f64, f64) = (170.0, 145.0);
const COVER_WALL: f64 = 12.0;
const COVER_WINDOW_X: f64 = 160.0;
const COVER_WINDOW_Y: f64 = 112.0;
const COVER_WINDOWS: usize = 3;
const COVER_HANDLE_X: f64 = 230.0;
const COVER_HANDLE_Y: f64 = 24.0;
const COVER_LATCHES: usize = 4;

const STANDARD_LANE_X: f64 = 450.0;
const STANDARD_LANE_Y: f64 = 120.0;
const STANDARD_LANE_Z: f64 = 34.0;
const STANDARD_LANE_POS: (f64, f64) = (-385.0, -325.0);
const STANDARD_LANES: usize = 2;
const HIGH_STANDARD_INDEX: usize = 0;
const LOW_STANDARD_INDEX: usize = 1;
const STANDARD_SLOTS_PER_LANE: usize = 8;
const STANDARD_SLOT_X: f64 = 36.0;
const STANDARD_SLOT_Y: f64 = 30.0;
const STANDARD_PITCH_X: f64 = 48.0;
const STANDARD_LANE_PITCH_Y: f64 = 50.0;

const WASTE_X: f64 = 280.0;
const WASTE_Y: f64 = 134.0;
const WASTE_Z: f64 = 48.0;
const WASTE_POS: (f64, f64) = (15.0, -330.0);
const WASTE_CELLS: usize = 4;
const WASTE_CELL_X: f64 = 54.0;
const WASTE_CELL_Y: f64 = 54.0;
const WASTE_OVERFLOW_WEIRS: usize = 3;

const CUSTODY_X: f64 = 410.0;
const CUSTODY_Y: f64 = 118.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (360.0, -330.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 4;
const SEAL_WITNESS_PADS: usize = 4;

const GATE_BANK_X: f64 = 220.0;
const GATE_BANK_Y: f64 = 190.0;
const GATE_BANK_Z: f64 = 34.0;
const GATE_BANK_POS: (f64, f64) = (500.0, 60.0);
const DISPOSITION_GATES: usize = 3;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;
const GATE_TOKEN_SLOTS: usize = 9;
const GATE_PITCH_Y: f64 = 52.0;

const CAMERA_BRIDGE_X: f64 = 910.0;
const CAMERA_BRIDGE_Y: f64 = 42.0;
const CAMERA_BRIDGE_Z: f64 = 178.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (8.0, 34.0);
const CAMERA_MOUNTS: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 10;
const SCALE_READOUT_WINDOW_X: f64 = 132.0;
const SCALE_READOUT_WINDOW_Y: f64 = 34.0;

const ROBOT_KEEPOUT_X: f64 = 1160.0;
const ROBOT_KEEPOUT_Y: f64 = 82.0;
const ROBOT_KEEPOUT_Z: f64 = 72.0;
const SERVICE_KEEPOUT_X: f64 = 96.0;
const SERVICE_KEEPOUT_Y: f64 = 660.0;
const SERVICE_KEEPOUT_Z: f64 = 92.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 290.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(additive_vial_bag_nests(), OUTPUTS[1]);
    write_part(microdose_collection_wells(), OUTPUTS[2]);
    write_part(gravimetric_balance_load_cell_pad(), OUTPUTS[3]);
    write_part(flush_recovery_route(), OUTPUTS[4]);
    write_part(low_dead_volume_adapter_coupons(), OUTPUTS[5]);
    write_part(evaporation_cover(), OUTPUTS[6]);
    write_part(high_low_standard_lanes(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(waste_capture(), OUTPUTS[9]);
    write_part(release_hold_reject_gates(), OUTPUTS[10]);
    write_part(camera_evidence_bridge(), OUTPUTS[11]);
    write_part(robot_service_keepouts(), OUTPUTS[12]);
    write_part(station_assembly(), OUTPUTS[13]);

    println!(
        "Closed additive microdose recovery gravimetric station: {:.0}mm x {:.0}mm contained deck, {} additive vial nests, {} bag nests, {} microdose collection wells at {:.0}uL envelope.",
        DECK_X, DECK_Y, ADDITIVE_VIAL_NESTS, ADDITIVE_BAG_NESTS, MICRODOSE_WELLS, WELL_VOLUME_UL
    );
    println!(
        "Mechanical validation packaging only: balance pad {:.0}mm pan envelope at {:.1}mg reference resolution, {} flush/recovery channels, {} low-dead-volume adapter coupons below {:.0}uL target.",
        BALANCE_PAN_D,
        BALANCE_RESOLUTION_MG,
        FLUSH_CHANNELS + RECOVERY_CHANNELS,
        ADAPTER_COUPONS,
        COUPON_DEAD_VOLUME_TARGET_UL
    );
    println!(
        "Evidence and custody: {} high/low standard slots, {} barcode lands, {} COA lands, {} release/hold/reject gates, {} evidence fiducials, top service clearance {:.0}mm.",
        STANDARD_LANES * STANDARD_SLOTS_PER_LANE,
        BARCODE_LANDS,
        COA_LANDS,
        DISPOSITION_GATES,
        EVIDENCE_FIDUCIALS,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + additive_vial_bag_nests()
        + microdose_collection_wells()
        + gravimetric_balance_load_cell_pad()
        + flush_recovery_route()
        + low_dead_volume_adapter_coupons()
        + evaporation_cover()
        + high_low_standard_lanes()
        + barcode_coa_custody_lands()
        + waste_capture()
        + release_hold_reject_gates()
        + camera_evidence_bridge()
        + robot_service_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "microdose_recovery_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let spill_sump = centered_cube(
        "microdose_recovery_station_recessed_spill_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - SUMP_Z / 2.0);
    let drain = centered_cylinder(
        "microdose_recovery_station_sump_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 30.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 98.0, -DECK_Y / 2.0 + 12.0, 0.0);

    deck - spill_sump - drain + containment_rims() + deck_datum_bosses() + station_zone_markers()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "microdose_recovery_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, rim_z());
    let rear = centered_cube(
        "microdose_recovery_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let left = centered_cube(
        "microdose_recovery_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_z());
    let right = centered_cube(
        "microdose_recovery_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());

    front + rear + left + right
}

fn deck_datum_bosses() -> Part {
    let mut bosses = Part::empty("microdose_recovery_station_deck_datum_bosses");
    for (i, (x, y)) in deck_datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("microdose_recovery_station_datum_boss_{i}"),
            14.0,
            8.0,
            36,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("microdose_recovery_station_datum_bore_{i}"),
            4.2,
            10.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn station_zone_markers() -> Part {
    let mut markers = Part::empty("microdose_recovery_station_zone_markers");
    for (i, (x, y, sx, sy)) in [
        (
            NEST_BANK_POS.0,
            NEST_BANK_POS.1,
            NEST_BANK_X + 22.0,
            NEST_BANK_Y + 20.0,
        ),
        (
            COLLECTION_BANK_POS.0,
            COLLECTION_BANK_POS.1,
            COLLECTION_BANK_X + 22.0,
            COLLECTION_BANK_Y + 20.0,
        ),
        (
            BALANCE_PAD_POS.0,
            BALANCE_PAD_POS.1,
            BALANCE_PAD_X + 26.0,
            BALANCE_PAD_Y + 24.0,
        ),
        (
            ROUTE_PLATE_POS.0,
            ROUTE_PLATE_POS.1,
            ROUTE_PLATE_X + 20.0,
            ROUTE_PLATE_Y + 18.0,
        ),
        (
            COUPON_BANK_POS.0,
            COUPON_BANK_POS.1,
            COUPON_BANK_X + 20.0,
            COUPON_BANK_Y + 20.0,
        ),
        (
            STANDARD_LANE_POS.0,
            STANDARD_LANE_POS.1,
            STANDARD_LANE_X + 18.0,
            STANDARD_LANE_Y + 16.0,
        ),
        (WASTE_POS.0, WASTE_POS.1, WASTE_X + 18.0, WASTE_Y + 16.0),
        (
            CUSTODY_POS.0,
            CUSTODY_POS.1,
            CUSTODY_X + 18.0,
            CUSTODY_Y + 16.0,
        ),
        (
            GATE_BANK_POS.0,
            GATE_BANK_POS.1,
            GATE_BANK_X + 18.0,
            GATE_BANK_Y + 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + centered_cube(
                format!("microdose_recovery_station_zone_marker_{i}"),
                *sx,
                *sy,
                2.4,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 1.2);
    }
    markers
}

fn additive_vial_bag_nests() -> Part {
    let base = centered_cube(
        "microdose_recovery_station_additive_vial_bag_nest_bank",
        NEST_BANK_X,
        NEST_BANK_Y,
        NEST_BANK_Z,
    )
    .translate(NEST_BANK_POS.0, NEST_BANK_POS.1, top_z(NEST_BANK_Z));

    base - vial_well_cuts() - bag_recess_cuts() + bag_edge_clips() + vial_scan_lands()
}

fn vial_well_cuts() -> Part {
    let mut cuts = Part::empty("microdose_recovery_station_additive_vial_well_cuts");
    for index in 0..ADDITIVE_VIAL_NESTS {
        let (x, y) = vial_center(index);
        cuts = cuts
            + centered_cylinder(
                format!("microdose_recovery_station_additive_vial_well_{index}"),
                VIAL_WELL_D / 2.0,
                NEST_BANK_Z + 2.0,
                40,
            )
            .translate(NEST_BANK_POS.0 + x, NEST_BANK_POS.1 + y, top_z(NEST_BANK_Z));
    }
    cuts
}

fn bag_recess_cuts() -> Part {
    let mut cuts = Part::empty("microdose_recovery_station_additive_bag_recess_cuts");
    let start_x = -((ADDITIVE_BAG_NESTS as f64 - 1.0) * 96.0) / 2.0;
    for index in 0..ADDITIVE_BAG_NESTS {
        cuts = cuts
            + centered_cube(
                format!("microdose_recovery_station_additive_bag_nest_recess_{index}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_RECESS_Z + 1.0,
            )
            .translate(
                NEST_BANK_POS.0 + start_x + index as f64 * 96.0,
                NEST_BANK_POS.1 - 47.0,
                top_z(NEST_BANK_Z) + NEST_BANK_Z / 2.0 - BAG_RECESS_Z / 2.0,
            );
    }
    cuts
}

fn bag_edge_clips() -> Part {
    let mut clips = Part::empty("microdose_recovery_station_additive_bag_edge_clips");
    let start_x = -((ADDITIVE_BAG_NESTS as f64 - 1.0) * 96.0) / 2.0;
    for index in 0..ADDITIVE_BAG_NESTS {
        let x = NEST_BANK_POS.0 + start_x + index as f64 * 96.0;
        clips = clips
            + centered_cube(
                format!("microdose_recovery_station_bag_front_clip_{index}"),
                BAG_NEST_X,
                10.0,
                16.0,
            )
            .translate(x, NEST_BANK_POS.1 - 128.0, DECK_Z / 2.0 + NEST_BANK_Z + 8.0)
            + centered_cube(
                format!("microdose_recovery_station_bag_rear_clip_{index}"),
                BAG_NEST_X,
                10.0,
                16.0,
            )
            .translate(x, NEST_BANK_POS.1 + 34.0, DECK_Z / 2.0 + NEST_BANK_Z + 8.0);
    }
    clips
}

fn vial_scan_lands() -> Part {
    let mut lands = Part::empty("microdose_recovery_station_additive_vial_scan_lands");
    for index in 0..ADDITIVE_VIAL_NESTS {
        let (x, y) = vial_center(index);
        lands = lands
            + centered_cube(
                format!("microdose_recovery_station_additive_vial_scan_land_{index}"),
                22.0,
                6.0,
                3.0,
            )
            .translate(
                NEST_BANK_POS.0 + x,
                NEST_BANK_POS.1 + y + 23.0,
                DECK_Z / 2.0 + NEST_BANK_Z + 1.5,
            );
    }
    lands
}

fn microdose_collection_wells() -> Part {
    let bank = centered_cube(
        "microdose_recovery_station_microdose_collection_well_bank",
        COLLECTION_BANK_X,
        COLLECTION_BANK_Y,
        COLLECTION_BANK_Z,
    )
    .translate(
        COLLECTION_BANK_POS.0,
        COLLECTION_BANK_POS.1,
        top_z(COLLECTION_BANK_Z),
    );

    bank - collection_well_cuts()
        + collection_well_rims()
        + well_identity_ticks()
        + evaporation_flag_pads()
}

fn collection_well_cuts() -> Part {
    let mut cuts = Part::empty("microdose_recovery_station_collection_well_cuts");
    for index in 0..MICRODOSE_WELLS {
        let (x, y) = microdose_well_center(index);
        cuts = cuts
            + centered_cylinder(
                format!("microdose_recovery_station_microdose_collection_well_{index}"),
                MICRODOSE_WELL_D / 2.0,
                COLLECTION_BANK_Z + 2.0,
                40,
            )
            .translate(
                COLLECTION_BANK_POS.0 + x,
                COLLECTION_BANK_POS.1 + y,
                top_z(COLLECTION_BANK_Z),
            );
    }
    cuts
}

fn collection_well_rims() -> Part {
    let mut rims = Part::empty("microdose_recovery_station_collection_well_rims");
    for index in 0..MICRODOSE_WELLS {
        let (x, y) = microdose_well_center(index);
        rims = rims
            + centered_cylinder(
                format!("microdose_recovery_station_collection_well_rim_{index}"),
                WELL_RIM_D / 2.0,
                5.0,
                40,
            )
            .translate(
                COLLECTION_BANK_POS.0 + x,
                COLLECTION_BANK_POS.1 + y,
                DECK_Z / 2.0 + COLLECTION_BANK_Z + 2.5,
            );
    }
    rims
}

fn well_identity_ticks() -> Part {
    let mut ticks = Part::empty("microdose_recovery_station_collection_well_identity_ticks");
    for index in 0..MICRODOSE_WELLS {
        let (x, y) = microdose_well_center(index);
        let length = if index < MICRODOSE_COLS { 20.0 } else { 12.0 };
        ticks = ticks
            + centered_cube(
                format!("microdose_recovery_station_collection_well_tick_{index}"),
                length,
                4.0,
                3.0,
            )
            .translate(
                COLLECTION_BANK_POS.0 + x,
                COLLECTION_BANK_POS.1 + y - 24.0,
                DECK_Z / 2.0 + COLLECTION_BANK_Z + 1.5,
            );
    }
    ticks
}

fn evaporation_flag_pads() -> Part {
    let mut pads = Part::empty("microdose_recovery_station_evaporation_flag_pads");
    for index in 0..EVAPORATION_FLAG_PADS {
        pads = pads
            + centered_cube(
                format!("microdose_recovery_station_evaporation_flag_pad_{index}"),
                44.0,
                18.0,
                4.0,
            )
            .translate(
                COLLECTION_BANK_POS.0 - 176.0 + index as f64 * 62.0,
                COLLECTION_BANK_POS.1 - COLLECTION_BANK_Y / 2.0 + 24.0,
                DECK_Z / 2.0 + COLLECTION_BANK_Z + 2.0,
            );
    }
    pads
}

fn gravimetric_balance_load_cell_pad() -> Part {
    let pad = centered_cube(
        "microdose_recovery_station_gravimetric_balance_pad",
        BALANCE_PAD_X,
        BALANCE_PAD_Y,
        BALANCE_PAD_Z,
    )
    .translate(BALANCE_PAD_POS.0, BALANCE_PAD_POS.1, top_z(BALANCE_PAD_Z));

    pad - load_cell_cavity() - isolation_moat() - balance_cable_trough()
        + pan_registration_ring()
        + load_cell_stop_blocks()
        + balance_leveling_feet()
        + draft_shield_posts()
}

fn load_cell_cavity() -> Part {
    centered_cube(
        "microdose_recovery_station_load_cell_service_pocket",
        LOAD_CELL_POCKET_X,
        LOAD_CELL_POCKET_Y,
        LOAD_CELL_POCKET_Z + 1.0,
    )
    .translate(
        BALANCE_PAD_POS.0,
        BALANCE_PAD_POS.1,
        top_z(BALANCE_PAD_Z) + BALANCE_PAD_Z / 2.0 - LOAD_CELL_POCKET_Z / 2.0,
    )
}

fn isolation_moat() -> Part {
    let north = centered_cube(
        "microdose_recovery_station_load_cell_isolation_moat_north",
        ISOLATION_MOAT_X,
        ISOLATION_MOAT_W,
        BALANCE_PAD_Z + 2.0,
    )
    .translate(
        BALANCE_PAD_POS.0,
        BALANCE_PAD_POS.1 + ISOLATION_MOAT_Y / 2.0,
        top_z(BALANCE_PAD_Z),
    );
    let south = centered_cube(
        "microdose_recovery_station_load_cell_isolation_moat_south",
        ISOLATION_MOAT_X,
        ISOLATION_MOAT_W,
        BALANCE_PAD_Z + 2.0,
    )
    .translate(
        BALANCE_PAD_POS.0,
        BALANCE_PAD_POS.1 - ISOLATION_MOAT_Y / 2.0,
        top_z(BALANCE_PAD_Z),
    );
    let east = centered_cube(
        "microdose_recovery_station_load_cell_isolation_moat_east",
        ISOLATION_MOAT_W,
        ISOLATION_MOAT_Y,
        BALANCE_PAD_Z + 2.0,
    )
    .translate(
        BALANCE_PAD_POS.0 + ISOLATION_MOAT_X / 2.0,
        BALANCE_PAD_POS.1,
        top_z(BALANCE_PAD_Z),
    );
    let west = centered_cube(
        "microdose_recovery_station_load_cell_isolation_moat_west",
        ISOLATION_MOAT_W,
        ISOLATION_MOAT_Y,
        BALANCE_PAD_Z + 2.0,
    )
    .translate(
        BALANCE_PAD_POS.0 - ISOLATION_MOAT_X / 2.0,
        BALANCE_PAD_POS.1,
        top_z(BALANCE_PAD_Z),
    );

    north + south + east + west
}

fn balance_cable_trough() -> Part {
    centered_cube(
        "microdose_recovery_station_balance_load_cell_cable_trough",
        BALANCE_CABLE_TROUGH_X,
        22.0,
        BALANCE_PAD_Z + 2.0,
    )
    .translate(
        BALANCE_PAD_POS.0 - BALANCE_PAD_X / 2.0 + BALANCE_CABLE_TROUGH_X / 2.0,
        BALANCE_PAD_POS.1,
        top_z(BALANCE_PAD_Z),
    )
}

fn pan_registration_ring() -> Part {
    let ring = centered_cylinder(
        "microdose_recovery_station_balance_pan_registration_ring",
        BALANCE_PAN_D / 2.0 + 9.0,
        6.0,
        64,
    )
    .translate(
        BALANCE_PAD_POS.0,
        BALANCE_PAD_POS.1,
        DECK_Z / 2.0 + BALANCE_PAD_Z + 3.0,
    );
    let pan_clearance = centered_cylinder(
        "microdose_recovery_station_balance_pan_clearance",
        BALANCE_PAN_D / 2.0,
        8.0,
        64,
    )
    .translate(
        BALANCE_PAD_POS.0,
        BALANCE_PAD_POS.1,
        DECK_Z / 2.0 + BALANCE_PAD_Z + 3.0,
    );

    ring - pan_clearance
}

fn load_cell_stop_blocks() -> Part {
    let mut stops = Part::empty("microdose_recovery_station_load_cell_stop_blocks");
    for (i, y) in [-72.0, 72.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("microdose_recovery_station_overload_stop_block_{i}"),
                128.0,
                12.0,
                12.0,
            )
            .translate(
                BALANCE_PAD_POS.0,
                BALANCE_PAD_POS.1 + y,
                DECK_Z / 2.0 + BALANCE_PAD_Z + 6.0,
            );
    }
    stops
}

fn balance_leveling_feet() -> Part {
    let mut feet = Part::empty("microdose_recovery_station_balance_leveling_feet");
    for (i, (x, y)) in balance_foot_positions().iter().enumerate() {
        let foot = centered_cylinder(
            format!("microdose_recovery_station_balance_leveling_foot_{i}"),
            14.0,
            8.0,
            36,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("microdose_recovery_station_balance_leveling_bore_{i}"),
            4.3,
            10.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        feet = feet + (foot - bore);
    }
    feet
}

fn draft_shield_posts() -> Part {
    let mut posts = Part::empty("microdose_recovery_station_evaporation_draft_posts");
    for (i, (x, y)) in balance_foot_positions().iter().enumerate() {
        posts = posts
            + centered_cylinder(
                format!("microdose_recovery_station_draft_shield_post_{i}"),
                7.0,
                72.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0 + BALANCE_PAD_Z + 36.0);
    }
    posts
}

fn flush_recovery_route() -> Part {
    let plate = centered_cube(
        "microdose_recovery_station_flush_recovery_route_plate",
        ROUTE_PLATE_X,
        ROUTE_PLATE_Y,
        ROUTE_PLATE_Z,
    )
    .translate(ROUTE_PLATE_POS.0, ROUTE_PLATE_POS.1, top_z(ROUTE_PLATE_Z));

    plate - route_channels() - route_port_bores() - recovery_trap_recesses()
        + route_port_collars()
        + valve_seat_lands()
        + recovery_trap_lips()
}

fn route_channels() -> Part {
    let mut channels = Part::empty("microdose_recovery_station_flush_recovery_channel_cuts");
    for index in 0..RECOVERY_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("microdose_recovery_station_recovery_route_channel_{index}"),
                ROUTE_CHANNEL_D / 2.0,
                ROUTE_PLATE_X - 62.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                ROUTE_PLATE_POS.0,
                ROUTE_PLATE_POS.1 - 32.0 + index as f64 * 64.0,
                top_z(ROUTE_PLATE_Z) + 2.0,
            );
    }
    for index in 0..FLUSH_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("microdose_recovery_station_flush_route_channel_{index}"),
                ROUTE_CHANNEL_D / 2.0,
                ROUTE_PLATE_Y - 44.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                ROUTE_PLATE_POS.0 - 150.0 + index as f64 * 300.0,
                ROUTE_PLATE_POS.1,
                top_z(ROUTE_PLATE_Z) + 2.0,
            );
    }
    channels
}

fn route_port_bores() -> Part {
    let mut bores = Part::empty("microdose_recovery_station_route_port_bores");
    let start_x = -((ROUTE_PORTS as f64 - 1.0) * 62.0) / 2.0;
    for index in 0..ROUTE_PORTS {
        bores = bores
            + centered_cylinder(
                format!("microdose_recovery_station_flush_recovery_port_bore_{index}"),
                ROUTE_PORT_D / 2.0,
                ROUTE_PLATE_Z + 8.0,
                28,
            )
            .translate(
                ROUTE_PLATE_POS.0 + start_x + index as f64 * 62.0,
                ROUTE_PLATE_POS.1 + if index % 2 == 0 { -54.0 } else { 54.0 },
                top_z(ROUTE_PLATE_Z),
            );
    }
    bores
}

fn route_port_collars() -> Part {
    let mut collars = Part::empty("microdose_recovery_station_route_port_collars");
    let start_x = -((ROUTE_PORTS as f64 - 1.0) * 62.0) / 2.0;
    for index in 0..ROUTE_PORTS {
        collars = collars
            + centered_cylinder(
                format!("microdose_recovery_station_flush_recovery_port_collar_{index}"),
                17.0,
                7.0,
                32,
            )
            .translate(
                ROUTE_PLATE_POS.0 + start_x + index as f64 * 62.0,
                ROUTE_PLATE_POS.1 + if index % 2 == 0 { -54.0 } else { 54.0 },
                DECK_Z / 2.0 + ROUTE_PLATE_Z + 3.5,
            );
    }
    collars
}

fn valve_seat_lands() -> Part {
    let mut seats = Part::empty("microdose_recovery_station_route_valve_seat_lands");
    let start_x = -((ROUTE_VALVE_SEATS as f64 - 1.0) * 74.0) / 2.0;
    for index in 0..ROUTE_VALVE_SEATS {
        seats = seats
            + centered_cube(
                format!("microdose_recovery_station_flush_recovery_valve_seat_{index}"),
                42.0,
                38.0,
                9.0,
            )
            .translate(
                ROUTE_PLATE_POS.0 + start_x + index as f64 * 74.0,
                ROUTE_PLATE_POS.1,
                DECK_Z / 2.0 + ROUTE_PLATE_Z + 4.5,
            );
    }
    seats
}

fn recovery_trap_recesses() -> Part {
    let mut traps = Part::empty("microdose_recovery_station_recovery_trap_recesses");
    for index in 0..RECOVERY_TRAP_CUPS {
        traps = traps
            + centered_cylinder(
                format!("microdose_recovery_station_recovery_trap_recess_{index}"),
                19.0,
                18.0,
                36,
            )
            .translate(
                ROUTE_PLATE_POS.0 - 210.0 + index as f64 * 140.0,
                ROUTE_PLATE_POS.1 + 2.0,
                top_z(ROUTE_PLATE_Z) + ROUTE_PLATE_Z / 2.0 - 7.0,
            );
    }
    traps
}

fn recovery_trap_lips() -> Part {
    let mut lips = Part::empty("microdose_recovery_station_recovery_trap_lips");
    for index in 0..RECOVERY_TRAP_CUPS {
        lips = lips
            + centered_cylinder(
                format!("microdose_recovery_station_recovery_trap_lip_{index}"),
                23.0,
                5.0,
                36,
            )
            .translate(
                ROUTE_PLATE_POS.0 - 210.0 + index as f64 * 140.0,
                ROUTE_PLATE_POS.1 + 2.0,
                DECK_Z / 2.0 + ROUTE_PLATE_Z + 2.5,
            );
    }
    lips
}

fn low_dead_volume_adapter_coupons() -> Part {
    let tray = centered_cube(
        "microdose_recovery_station_low_dead_volume_adapter_coupon_tray",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(COUPON_BANK_POS.0, COUPON_BANK_POS.1, top_z(COUPON_BANK_Z));

    tray - coupon_socket_cuts() + adapter_coupons() + coupon_pull_tabs()
}

fn coupon_socket_cuts() -> Part {
    let mut cuts = Part::empty("microdose_recovery_station_adapter_coupon_socket_cuts");
    for index in 0..ADAPTER_COUPONS {
        let (x, y) = coupon_center(index);
        cuts = cuts
            + centered_cube(
                format!("microdose_recovery_station_adapter_coupon_socket_{index}"),
                COUPON_X + 6.0,
                COUPON_Y + 6.0,
                COUPON_Z + 4.0,
            )
            .translate(
                COUPON_BANK_POS.0 + x,
                COUPON_BANK_POS.1 + y,
                top_z(COUPON_BANK_Z) + COUPON_BANK_Z / 2.0 - (COUPON_Z + 4.0) / 2.0,
            );
    }
    cuts
}

fn adapter_coupons() -> Part {
    let mut coupons = Part::empty("microdose_recovery_station_low_dead_volume_adapter_coupons");
    for index in 0..ADAPTER_COUPONS {
        let (x, y) = coupon_center(index);
        let coupon = centered_cube(
            format!("microdose_recovery_station_low_dead_volume_adapter_coupon_{index}"),
            COUPON_X,
            COUPON_Y,
            COUPON_Z,
        )
        .translate(
            COUPON_BANK_POS.0 + x,
            COUPON_BANK_POS.1 + y,
            DECK_Z / 2.0 + COUPON_BANK_Z + COUPON_Z / 2.0,
        );
        let bore = centered_cylinder(
            format!("microdose_recovery_station_coupon_low_dead_volume_bore_{index}"),
            LDV_BORE_D / 2.0,
            COUPON_X + 4.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            COUPON_BANK_POS.0 + x,
            COUPON_BANK_POS.1 + y,
            DECK_Z / 2.0 + COUPON_BANK_Z + COUPON_Z / 2.0,
        );
        coupons = coupons + (coupon - bore);
    }
    coupons
}

fn coupon_pull_tabs() -> Part {
    let mut tabs = Part::empty("microdose_recovery_station_adapter_coupon_pull_tabs");
    for index in 0..ADAPTER_COUPONS {
        let (x, y) = coupon_center(index);
        tabs = tabs
            + centered_cube(
                format!("microdose_recovery_station_adapter_coupon_pull_tab_{index}"),
                24.0,
                5.0,
                5.0,
            )
            .translate(
                COUPON_BANK_POS.0 + x,
                COUPON_BANK_POS.1 + y - COUPON_Y / 2.0 - 5.0,
                DECK_Z / 2.0 + COUPON_BANK_Z + 2.5,
            );
    }
    tabs
}

fn evaporation_cover() -> Part {
    let shell = centered_cube(
        "microdose_recovery_station_evaporation_cover_outer_shell",
        COVER_X,
        COVER_Y,
        COVER_Z,
    )
    .translate(COVER_POS.0, COVER_POS.1, DECK_Z / 2.0 + COVER_Z / 2.0);
    let hollow = centered_cube(
        "microdose_recovery_station_evaporation_cover_inner_clearance",
        COVER_X - COVER_WALL * 2.0,
        COVER_Y - COVER_WALL * 2.0,
        COVER_Z - COVER_WALL,
    )
    .translate(
        COVER_POS.0,
        COVER_POS.1,
        DECK_Z / 2.0 + COVER_WALL / 2.0 + (COVER_Z - COVER_WALL) / 2.0,
    );

    shell - hollow - cover_windows() + cover_latches() + cover_handle() + cover_gasket_land()
}

fn cover_windows() -> Part {
    let mut windows = Part::empty("microdose_recovery_station_evaporation_cover_windows");
    for index in 0..COVER_WINDOWS {
        windows = windows
            + centered_cube(
                format!("microdose_recovery_station_evaporation_cover_view_window_{index}"),
                COVER_WINDOW_X,
                COVER_WINDOW_Y,
                COVER_WALL + 4.0,
            )
            .translate(
                COVER_POS.0 - 220.0 + index as f64 * 220.0,
                COVER_POS.1,
                DECK_Z / 2.0 + COVER_Z - COVER_WALL / 2.0,
            );
    }
    windows
}

fn cover_latches() -> Part {
    let mut latches = Part::empty("microdose_recovery_station_evaporation_cover_latches");
    let start_x = -((COVER_LATCHES as f64 - 1.0) * 220.0) / 2.0;
    for index in 0..COVER_LATCHES {
        latches = latches
            + centered_cube(
                format!("microdose_recovery_station_evaporation_cover_latch_{index}"),
                44.0,
                16.0,
                18.0,
            )
            .translate(
                COVER_POS.0 + start_x + index as f64 * 220.0,
                COVER_POS.1 - COVER_Y / 2.0 - 8.0,
                DECK_Z / 2.0 + 28.0,
            );
    }
    latches
}

fn cover_handle() -> Part {
    let handle_bar = centered_cube(
        "microdose_recovery_station_evaporation_cover_handle_bar",
        COVER_HANDLE_X,
        COVER_HANDLE_Y,
        18.0,
    )
    .translate(COVER_POS.0, COVER_POS.1, DECK_Z / 2.0 + COVER_Z + 9.0);
    let left = centered_cube(
        "microdose_recovery_station_evaporation_cover_handle_left_standoff",
        18.0,
        18.0,
        30.0,
    )
    .translate(
        COVER_POS.0 - COVER_HANDLE_X / 2.0 + 24.0,
        COVER_POS.1,
        DECK_Z / 2.0 + COVER_Z + 15.0,
    );
    let right = centered_cube(
        "microdose_recovery_station_evaporation_cover_handle_right_standoff",
        18.0,
        18.0,
        30.0,
    )
    .translate(
        COVER_POS.0 + COVER_HANDLE_X / 2.0 - 24.0,
        COVER_POS.1,
        DECK_Z / 2.0 + COVER_Z + 15.0,
    );

    handle_bar + left + right
}

fn cover_gasket_land() -> Part {
    let front = centered_cube(
        "microdose_recovery_station_evaporation_cover_front_gasket_land",
        COVER_X - 40.0,
        7.0,
        6.0,
    )
    .translate(
        COVER_POS.0,
        COVER_POS.1 - COVER_Y / 2.0 + 24.0,
        DECK_Z / 2.0 + 3.0,
    );
    let rear = centered_cube(
        "microdose_recovery_station_evaporation_cover_rear_gasket_land",
        COVER_X - 40.0,
        7.0,
        6.0,
    )
    .translate(
        COVER_POS.0,
        COVER_POS.1 + COVER_Y / 2.0 - 24.0,
        DECK_Z / 2.0 + 3.0,
    );
    let left = centered_cube(
        "microdose_recovery_station_evaporation_cover_left_gasket_land",
        7.0,
        COVER_Y - 40.0,
        6.0,
    )
    .translate(
        COVER_POS.0 - COVER_X / 2.0 + 24.0,
        COVER_POS.1,
        DECK_Z / 2.0 + 3.0,
    );
    let right = centered_cube(
        "microdose_recovery_station_evaporation_cover_right_gasket_land",
        7.0,
        COVER_Y - 40.0,
        6.0,
    )
    .translate(
        COVER_POS.0 + COVER_X / 2.0 - 24.0,
        COVER_POS.1,
        DECK_Z / 2.0 + 3.0,
    );

    front + rear + left + right
}

fn high_low_standard_lanes() -> Part {
    let plate = centered_cube(
        "microdose_recovery_station_high_low_standard_lane_plate",
        STANDARD_LANE_X,
        STANDARD_LANE_Y,
        STANDARD_LANE_Z,
    )
    .translate(
        STANDARD_LANE_POS.0,
        STANDARD_LANE_POS.1,
        top_z(STANDARD_LANE_Z),
    );

    plate - standard_slot_cuts() + standard_lane_divider() + standard_lane_scan_ticks()
}

fn standard_slot_cuts() -> Part {
    let mut slots = Part::empty("microdose_recovery_station_high_low_standard_slot_cuts");
    for lane in 0..STANDARD_LANES {
        for slot in 0..STANDARD_SLOTS_PER_LANE {
            let (x, y) = standard_slot_center(lane, slot);
            slots = slots
                + centered_cube(
                    format!("microdose_recovery_station_standard_lane_{lane}_slot_{slot}_recess"),
                    STANDARD_SLOT_X,
                    STANDARD_SLOT_Y,
                    12.0,
                )
                .translate(
                    STANDARD_LANE_POS.0 + x,
                    STANDARD_LANE_POS.1 + y,
                    top_z(STANDARD_LANE_Z) + STANDARD_LANE_Z / 2.0 - 6.0,
                );
        }
    }
    slots
}

fn standard_lane_divider() -> Part {
    centered_cube(
        "microdose_recovery_station_high_low_standard_lane_divider",
        STANDARD_LANE_X - 34.0,
        6.0,
        18.0,
    )
    .translate(
        STANDARD_LANE_POS.0,
        STANDARD_LANE_POS.1,
        DECK_Z / 2.0 + STANDARD_LANE_Z + 9.0,
    )
}

fn standard_lane_scan_ticks() -> Part {
    let mut ticks = Part::empty("microdose_recovery_station_standard_lane_scan_ticks");
    for lane in 0..STANDARD_LANES {
        for slot in 0..STANDARD_SLOTS_PER_LANE {
            let (x, y) = standard_slot_center(lane, slot);
            let tick_y = if lane == HIGH_STANDARD_INDEX {
                y + 22.0
            } else {
                y - 22.0
            };
            ticks = ticks
                + centered_cube(
                    format!("microdose_recovery_station_standard_lane_{lane}_scan_tick_{slot}"),
                    18.0,
                    3.5,
                    3.0,
                )
                .translate(
                    STANDARD_LANE_POS.0 + x,
                    STANDARD_LANE_POS.1 + tick_y,
                    DECK_Z / 2.0 + STANDARD_LANE_Z + 1.5,
                );
        }
    }
    ticks
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "microdose_recovery_station_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z));

    panel + barcode_lands() + coa_lands() + seal_witness_pads()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("microdose_recovery_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("microdose_recovery_station_barcode_land_{index}"),
                76.0,
                18.0,
                3.0,
            )
            .translate(
                CUSTODY_POS.0 - 144.0 + (index % 4) as f64 * 96.0,
                CUSTODY_POS.1 + if index < 4 { 28.0 } else { 0.0 },
                DECK_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("microdose_recovery_station_coa_lands");
    for index in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("microdose_recovery_station_coa_card_land_{index}"),
                72.0,
                20.0,
                4.0,
            )
            .translate(
                CUSTODY_POS.0 - 144.0 + index as f64 * 96.0,
                CUSTODY_POS.1 - 33.0,
                DECK_Z / 2.0 + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn seal_witness_pads() -> Part {
    let mut pads = Part::empty("microdose_recovery_station_custody_seal_witness_pads");
    for index in 0..SEAL_WITNESS_PADS {
        pads = pads
            + centered_cylinder(
                format!("microdose_recovery_station_custody_seal_witness_pad_{index}"),
                10.0,
                4.0,
                28,
            )
            .translate(
                CUSTODY_POS.0 - 172.0 + index as f64 * 114.0,
                CUSTODY_POS.1 + CUSTODY_Y / 2.0 - 16.0,
                DECK_Z / 2.0 + CUSTODY_Z + 2.0,
            );
    }
    pads
}

fn waste_capture() -> Part {
    let block = centered_cube(
        "microdose_recovery_station_waste_capture_block",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(WASTE_POS.0, WASTE_POS.1, top_z(WASTE_Z));

    block - waste_cell_cuts() + waste_cell_lips() + overflow_weirs()
}

fn waste_cell_cuts() -> Part {
    let mut cells = Part::empty("microdose_recovery_station_waste_cell_cuts");
    let start_x = -((WASTE_CELLS as f64 - 1.0) * 64.0) / 2.0;
    for index in 0..WASTE_CELLS {
        cells = cells
            + centered_cube(
                format!("microdose_recovery_station_waste_capture_cell_{index}"),
                WASTE_CELL_X,
                WASTE_CELL_Y,
                26.0,
            )
            .translate(
                WASTE_POS.0 + start_x + index as f64 * 64.0,
                WASTE_POS.1,
                top_z(WASTE_Z) + WASTE_Z / 2.0 - 13.0,
            );
    }
    cells
}

fn waste_cell_lips() -> Part {
    let mut lips = Part::empty("microdose_recovery_station_waste_cell_lips");
    let start_x = -((WASTE_CELLS as f64 - 1.0) * 64.0) / 2.0;
    for index in 0..WASTE_CELLS {
        lips = lips
            + centered_cube(
                format!("microdose_recovery_station_waste_capture_lip_{index}"),
                WASTE_CELL_X + 10.0,
                WASTE_CELL_Y + 10.0,
                5.0,
            )
            .translate(
                WASTE_POS.0 + start_x + index as f64 * 64.0,
                WASTE_POS.1,
                DECK_Z / 2.0 + WASTE_Z + 2.5,
            );
    }
    lips
}

fn overflow_weirs() -> Part {
    let mut weirs = Part::empty("microdose_recovery_station_waste_overflow_weirs");
    for index in 0..WASTE_OVERFLOW_WEIRS {
        weirs = weirs
            + centered_cube(
                format!("microdose_recovery_station_waste_overflow_weir_{index}"),
                8.0,
                WASTE_CELL_Y + 12.0,
                16.0,
            )
            .translate(
                WASTE_POS.0 - 64.0 + index as f64 * 64.0,
                WASTE_POS.1,
                DECK_Z / 2.0 + WASTE_Z + 8.0,
            );
    }
    weirs
}

fn release_hold_reject_gates() -> Part {
    let plate = centered_cube(
        "microdose_recovery_station_release_hold_reject_gate_plate",
        GATE_BANK_X,
        GATE_BANK_Y,
        GATE_BANK_Z,
    )
    .translate(GATE_BANK_POS.0, GATE_BANK_POS.1, top_z(GATE_BANK_Z));

    plate - gate_token_slot_cuts() + gate_rails() + gate_state_pockets()
}

fn gate_token_slot_cuts() -> Part {
    let mut slots = Part::empty("microdose_recovery_station_disposition_gate_token_slot_cuts");
    for gate in 0..DISPOSITION_GATES {
        for slot in 0..(GATE_TOKEN_SLOTS / DISPOSITION_GATES) {
            slots = slots
                + centered_cube(
                    format!("microdose_recovery_station_gate_{gate}_token_slot_{slot}"),
                    42.0,
                    26.0,
                    10.0,
                )
                .translate(
                    GATE_BANK_POS.0 - 48.0 + slot as f64 * 48.0,
                    GATE_BANK_POS.1 + gate_y(gate),
                    top_z(GATE_BANK_Z) + GATE_BANK_Z / 2.0 - 5.0,
                );
        }
    }
    slots
}

fn gate_rails() -> Part {
    let mut rails = Part::empty("microdose_recovery_station_disposition_gate_rails");
    for gate in 0..DISPOSITION_GATES {
        rails = rails
            + centered_cube(
                format!("microdose_recovery_station_disposition_gate_rail_{gate}"),
                GATE_BANK_X - 24.0,
                5.0,
                20.0,
            )
            .translate(
                GATE_BANK_POS.0,
                GATE_BANK_POS.1 + gate_y(gate) + 20.0,
                DECK_Z / 2.0 + GATE_BANK_Z + 10.0,
            );
    }
    rails
}

fn gate_state_pockets() -> Part {
    let mut pockets = Part::empty("microdose_recovery_station_release_hold_reject_state_pockets");
    for gate in 0..DISPOSITION_GATES {
        pockets = pockets
            + centered_cylinder(
                format!("microdose_recovery_station_disposition_gate_state_pocket_{gate}"),
                13.0,
                6.0,
                28,
            )
            .translate(
                GATE_BANK_POS.0 + GATE_BANK_X / 2.0 - 30.0,
                GATE_BANK_POS.1 + gate_y(gate),
                DECK_Z / 2.0 + GATE_BANK_Z + 3.0,
            );
    }
    pockets
}

fn camera_evidence_bridge() -> Part {
    let beam = centered_cube(
        "microdose_recovery_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        28.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z,
    );
    let left = centered_cube(
        "microdose_recovery_station_camera_bridge_left_upright",
        38.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 - CAMERA_BRIDGE_X / 2.0 + 32.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let right = centered_cube(
        "microdose_recovery_station_camera_bridge_right_upright",
        38.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 + CAMERA_BRIDGE_X / 2.0 - 32.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );

    beam + left + right + camera_mounts() + evidence_fiducials() + scale_readout_frame()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("microdose_recovery_station_camera_mounts");
    for index in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("microdose_recovery_station_camera_mount_plate_{index}"),
                58.0,
                32.0,
                9.0,
            )
            .translate(
                CAMERA_BRIDGE_POS.0 - 300.0 + index as f64 * 200.0,
                CAMERA_BRIDGE_POS.1 - 28.0,
                DECK_Z / 2.0 + CAMERA_BRIDGE_Z + 22.0,
            );
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("microdose_recovery_station_evidence_fiducials");
    for index in 0..EVIDENCE_FIDUCIALS {
        let x = -450.0 + (index % 5) as f64 * 225.0;
        let y = if index < 5 { -350.0 } else { 350.0 };
        fiducials = fiducials
            + centered_cylinder(
                format!("microdose_recovery_station_evidence_fiducial_{index}"),
                8.0,
                3.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0 + 1.5);
    }
    fiducials
}

fn scale_readout_frame() -> Part {
    let frame = centered_cube(
        "microdose_recovery_station_scale_readout_evidence_frame",
        SCALE_READOUT_WINDOW_X + 20.0,
        SCALE_READOUT_WINDOW_Y + 18.0,
        8.0,
    )
    .translate(
        BALANCE_PAD_POS.0 + 150.0,
        BALANCE_PAD_POS.1 - BALANCE_PAD_Y / 2.0 + 30.0,
        DECK_Z / 2.0 + BALANCE_PAD_Z + 4.0,
    );
    let window = centered_cube(
        "microdose_recovery_station_scale_readout_window_cut",
        SCALE_READOUT_WINDOW_X,
        SCALE_READOUT_WINDOW_Y,
        10.0,
    )
    .translate(
        BALANCE_PAD_POS.0 + 150.0,
        BALANCE_PAD_POS.1 - BALANCE_PAD_Y / 2.0 + 30.0,
        DECK_Z / 2.0 + BALANCE_PAD_Z + 4.0,
    );

    frame - window
}

fn robot_service_keepouts() -> Part {
    let robot_sweep = centered_cube(
        "microdose_recovery_station_robot_sweep_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0);
    let left_service = centered_cube(
        "microdose_recovery_station_left_service_keepout_gauge",
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
        "microdose_recovery_station_right_service_keepout_gauge",
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
        "microdose_recovery_station_top_service_clearance_gauge",
        DECK_X - 180.0,
        DECK_Y - 150.0,
        10.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 + TOP_SERVICE_CLEARANCE_Z);

    robot_sweep + left_service + right_service + top_clearance
}

fn deck_datum_positions() -> [(f64, f64); DATUM_BOSSES] {
    [
        (-570.0, -370.0),
        (-285.0, -370.0),
        (0.0, -370.0),
        (285.0, -370.0),
        (570.0, -370.0),
        (-570.0, 370.0),
        (0.0, 370.0),
        (570.0, 370.0),
    ]
}

fn balance_foot_positions() -> [(f64, f64); LEVELING_FEET] {
    [
        (
            BALANCE_PAD_POS.0 - BALANCE_PAD_X / 2.0 + 46.0,
            BALANCE_PAD_POS.1 - BALANCE_PAD_Y / 2.0 + 42.0,
        ),
        (
            BALANCE_PAD_POS.0 + BALANCE_PAD_X / 2.0 - 46.0,
            BALANCE_PAD_POS.1 - BALANCE_PAD_Y / 2.0 + 42.0,
        ),
        (
            BALANCE_PAD_POS.0 - BALANCE_PAD_X / 2.0 + 46.0,
            BALANCE_PAD_POS.1 + BALANCE_PAD_Y / 2.0 - 42.0,
        ),
        (
            BALANCE_PAD_POS.0 + BALANCE_PAD_X / 2.0 - 46.0,
            BALANCE_PAD_POS.1 + BALANCE_PAD_Y / 2.0 - 42.0,
        ),
    ]
}

fn grid_center(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        (col as f64 - (cols as f64 - 1.0) / 2.0) * pitch_x,
        (row as f64 - (rows as f64 - 1.0) / 2.0) * pitch_y,
    )
}

fn vial_center(index: usize) -> (f64, f64) {
    let (x, y) = grid_center(index, VIAL_COLS, VIAL_ROWS, VIAL_PITCH_X, VIAL_PITCH_Y);
    (x, y + 56.0)
}

fn microdose_well_center(index: usize) -> (f64, f64) {
    grid_center(
        index,
        MICRODOSE_COLS,
        MICRODOSE_ROWS,
        MICRODOSE_PITCH_X,
        MICRODOSE_PITCH_Y,
    )
}

fn coupon_center(index: usize) -> (f64, f64) {
    grid_center(
        index,
        COUPON_COLS,
        COUPON_ROWS,
        COUPON_PITCH_X,
        COUPON_PITCH_Y,
    )
}

fn standard_slot_center(lane: usize, slot: usize) -> (f64, f64) {
    let start_x = -((STANDARD_SLOTS_PER_LANE as f64 - 1.0) * STANDARD_PITCH_X) / 2.0;
    let lane_y = match lane {
        HIGH_STANDARD_INDEX => STANDARD_LANE_PITCH_Y / 2.0,
        LOW_STANDARD_INDEX => -STANDARD_LANE_PITCH_Y / 2.0,
        _ => panic!("unknown standard lane index {lane}"),
    };
    (start_x + slot as f64 * STANDARD_PITCH_X, lane_y)
}

fn gate_y(gate: usize) -> f64 {
    (gate as f64 - (DISPOSITION_GATES as f64 - 1.0) / 2.0) * GATE_PITCH_Y
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn assert_layout() {
    assert_eq!(ADDITIVE_VIAL_NESTS, VIAL_COLS * VIAL_ROWS);
    assert_eq!(MICRODOSE_WELLS, MICRODOSE_COLS * MICRODOSE_ROWS);
    assert_eq!(ADAPTER_COUPONS, COUPON_COLS * COUPON_ROWS);
    assert_eq!(STANDARD_LANES, 2);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX, 0);
    assert_eq!(HOLD_GATE_INDEX, 1);
    assert_eq!(REJECT_GATE_INDEX, 2);
    assert_eq!(ADDITIVE_RETAINING_CLIPS, ADDITIVE_BAG_NESTS * 2);
    assert_eq!(DRAFT_POSTS, LEVELING_FEET);
    assert_eq!(COVER_LATCHES, 4);
    assert!(BALANCE_PAD_X > ISOLATION_MOAT_X + 2.0 * ISOLATION_MOAT_W);
    assert!(BALANCE_PAD_Y > ISOLATION_MOAT_Y + 2.0 * ISOLATION_MOAT_W);
    assert!(COVER_X > COLLECTION_BANK_X + BALANCE_PAN_D);
    assert!(COVER_Y > COLLECTION_BANK_Y);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
}

#[cfg(test)]
fn rect_fits_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_media_additive_microdose_recovery_gravimetric_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn feature_metadata_covers_requested_station_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 19);
        assert!(REQUIRED_FEATURES.contains(&"mechanical_validation_packaging_only"));
        assert!(REQUIRED_FEATURES.contains(&"additive_vial_nests"));
        assert!(REQUIRED_FEATURES.contains(&"additive_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"microdose_collection_wells"));
        assert!(REQUIRED_FEATURES.contains(&"gravimetric_balance_pad"));
        assert!(REQUIRED_FEATURES.contains(&"load_cell_isolation_moat"));
        assert!(REQUIRED_FEATURES.contains(&"flush_recovery_route"));
        assert!(REQUIRED_FEATURES.contains(&"low_dead_volume_adapter_coupons"));
        assert!(REQUIRED_FEATURES.contains(&"evaporation_cover"));
        assert!(REQUIRED_FEATURES.contains(&"high_standard_lane"));
        assert!(REQUIRED_FEATURES.contains(&"low_standard_lane"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_land"));
        assert!(REQUIRED_FEATURES.contains(&"coa_land"));
        assert!(REQUIRED_FEATURES.contains(&"waste_capture"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn repeated_feature_counts_match_validation_packaging() {
        assert_eq!(ADDITIVE_VIAL_NESTS, 12);
        assert_eq!(ADDITIVE_BAG_NESTS, 4);
        assert_eq!(MICRODOSE_WELLS, 16);
        assert_eq!(ADAPTER_COUPONS, 12);
        assert_eq!(STANDARD_LANES * STANDARD_SLOTS_PER_LANE, 16);
        assert_eq!(WASTE_CELLS, 4);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_LANDS, 4);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_MOUNTS, 4);
        assert_eq!(EVIDENCE_FIDUCIALS, 10);
    }

    #[test]
    fn major_modules_fit_inside_contained_deck() {
        assert!(rect_fits_deck(
            NEST_BANK_POS,
            NEST_BANK_X,
            NEST_BANK_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            COLLECTION_BANK_POS,
            COLLECTION_BANK_X,
            COLLECTION_BANK_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            BALANCE_PAD_POS,
            BALANCE_PAD_X,
            BALANCE_PAD_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            ROUTE_PLATE_POS,
            ROUTE_PLATE_X,
            ROUTE_PLATE_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            COUPON_BANK_POS,
            COUPON_BANK_X,
            COUPON_BANK_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            STANDARD_LANE_POS,
            STANDARD_LANE_X,
            STANDARD_LANE_Y,
            18.0
        ));
        assert!(rect_fits_deck(WASTE_POS, WASTE_X, WASTE_Y, 18.0));
        assert!(rect_fits_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y, 18.0));
        assert!(rect_fits_deck(
            GATE_BANK_POS,
            GATE_BANK_X,
            GATE_BANK_Y,
            18.0
        ));
        assert!(rect_fits_deck(COVER_POS, COVER_X, COVER_Y, 18.0));
    }

    #[test]
    fn arrays_stay_inside_their_fixture_banks() {
        let first_vial = vial_center(0);
        let last_vial = vial_center(ADDITIVE_VIAL_NESTS - 1);
        assert!(first_vial.0.abs() < NEST_BANK_X / 2.0 - VIAL_WELL_D);
        assert!(last_vial.0.abs() < NEST_BANK_X / 2.0 - VIAL_WELL_D);
        assert!(first_vial.1.abs() < NEST_BANK_Y / 2.0 - VIAL_WELL_D);
        assert!(last_vial.1.abs() < NEST_BANK_Y / 2.0 - VIAL_WELL_D);

        let first_well = microdose_well_center(0);
        let last_well = microdose_well_center(MICRODOSE_WELLS - 1);
        assert!(first_well.0.abs() < COLLECTION_BANK_X / 2.0 - WELL_RIM_D);
        assert!(last_well.0.abs() < COLLECTION_BANK_X / 2.0 - WELL_RIM_D);
        assert!(first_well.1.abs() < COLLECTION_BANK_Y / 2.0 - WELL_RIM_D);
        assert!(last_well.1.abs() < COLLECTION_BANK_Y / 2.0 - WELL_RIM_D);

        let first_coupon = coupon_center(0);
        let last_coupon = coupon_center(ADAPTER_COUPONS - 1);
        assert!(first_coupon.0.abs() < COUPON_BANK_X / 2.0 - COUPON_X);
        assert!(last_coupon.0.abs() < COUPON_BANK_X / 2.0 - COUPON_X);
        assert!(first_coupon.1.abs() < COUPON_BANK_Y / 2.0 - COUPON_Y);
        assert!(last_coupon.1.abs() < COUPON_BANK_Y / 2.0 - COUPON_Y);
    }

    #[test]
    fn balance_route_and_keepout_geometry_have_expected_clearance() {
        assert!(LOAD_CELL_POCKET_X > BALANCE_PAN_D);
        assert!(ISOLATION_MOAT_X > LOAD_CELL_POCKET_X + 80.0);
        assert!(ISOLATION_MOAT_Y > LOAD_CELL_POCKET_Y + 70.0);
        assert!(BALANCE_CABLE_TROUGH_X < BALANCE_PAD_X / 3.0);
        assert!(ROUTE_PLATE_X > (ROUTE_PORTS as f64 - 1.0) * 62.0 + 70.0);
        assert!(ROUTE_CHANNEL_D > LDV_BORE_D);
        assert!(COUPON_DEAD_VOLUME_TARGET_UL < WELL_VOLUME_UL / 20.0);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(TOP_SERVICE_CLEARANCE_Z > COVER_Z + DECK_Z);
    }

    #[test]
    fn high_low_lanes_and_disposition_indices_are_explicit() {
        assert_ne!(HIGH_STANDARD_INDEX, LOW_STANDARD_INDEX);
        assert_eq!(RELEASE_GATE_INDEX, 0);
        assert_eq!(HOLD_GATE_INDEX, 1);
        assert_eq!(REJECT_GATE_INDEX, 2);
        assert_eq!(GATE_TOKEN_SLOTS % DISPOSITION_GATES, 0);
        let high = standard_slot_center(HIGH_STANDARD_INDEX, 0);
        let low = standard_slot_center(LOW_STANDARD_INDEX, 0);
        assert!(high.1 > 0.0);
        assert!(low.1 < 0.0);
        assert!((high.1 - low.1).abs() >= STANDARD_LANE_PITCH_Y);
    }
}
