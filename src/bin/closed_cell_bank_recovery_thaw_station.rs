use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-bank recovery/thaw station for upstream lots before passaging
// and cell-suspension preparation.
//
// Research assumptions from the Exa pass:
// - Cell-therapy thaw systems such as ThawSTAR and plasmatherm emphasize
//   automated dry/water-free thawing, barrier bags, biosample sensing, and
//   chain-of-custody capture to reduce water-bath contamination and operator
//   variability.
// - Closed post-thaw wash/concentrate workflows, such as Sepax-style cell
//   processing, are used to standardize recovery and reduce debris/DMSO
//   handling after thaw.
// - This file models only mechanical envelopes, datum surfaces, custody
//   staging, segregation, and closed handoff placeholders. It is not a thaw
//   protocol, biological release criterion, sterility claim, or validated
//   manufacturing process.

const OUTPUTS: &[&str] = &[
    "output/closed_cell_bank_recovery_thaw_station_base_leak_tray.stl",
    "output/closed_cell_bank_recovery_thaw_station_cryovial_bag_receiving_nest.stl",
    "output/closed_cell_bank_recovery_thaw_station_controlled_thaw_block.stl",
    "output/closed_cell_bank_recovery_thaw_station_closed_wash_concentrate_interface.stl",
    "output/closed_cell_bank_recovery_thaw_station_count_viability_sample_handoff.stl",
    "output/closed_cell_bank_recovery_thaw_station_sterility_mycoplasma_custody_slots.stl",
    "output/closed_cell_bank_recovery_thaw_station_barcode_passage_lot_lands.stl",
    "output/closed_cell_bank_recovery_thaw_station_released_hold_reject_segregation.stl",
    "output/closed_cell_bank_recovery_thaw_station_cold_warm_zone_separation.stl",
    "output/closed_cell_bank_recovery_thaw_station_downstream_handoff_interface.stl",
    "output/closed_cell_bank_recovery_thaw_station_robot_service_keepouts.stl",
    "output/closed_cell_bank_recovery_thaw_station_assembly.stl",
];

const MODULE_X: f64 = 1420.0;
const MODULE_Y: f64 = 900.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 38.0;
const SUMP_X: f64 = 1260.0;
const SUMP_Y: f64 = 740.0;
const SUMP_DEPTH: f64 = 10.0;
const DRAIN_D: f64 = 18.0;
const DATUM_PINS: usize = 8;

const RECEIVING_X: f64 = 540.0;
const RECEIVING_Y: f64 = 352.0;
const RECEIVING_Z: f64 = 68.0;
const RECEIVING_POS_X: f64 = -440.0;
const RECEIVING_POS_Y: f64 = -206.0;
const BAG_SLOTS: usize = 2;
const BAG_SLOT_X: f64 = 188.0;
const BAG_SLOT_Y: f64 = 238.0;
const BAG_SLOT_RECESS_Z: f64 = 24.0;
const CRYOVIAL_ROWS: usize = 3;
const CRYOVIAL_COLS: usize = 6;
const CRYOVIAL_POSITIONS: usize = CRYOVIAL_ROWS * CRYOVIAL_COLS;
const CRYOVIAL_WELL_D: f64 = 18.5;
const CRYOVIAL_PITCH_X: f64 = 34.0;
const CRYOVIAL_PITCH_Y: f64 = 38.0;

const THAW_X: f64 = 560.0;
const THAW_Y: f64 = 372.0;
const THAW_Z: f64 = 86.0;
const THAW_POS_X: f64 = 190.0;
const THAW_POS_Y: f64 = 148.0;
const THAW_CHAMBERS: usize = 2;
const THAW_CHAMBER_X: f64 = 218.0;
const THAW_CHAMBER_Y: f64 = 278.0;
const THAW_RECESS_Z: f64 = 24.0;
const THAW_SENSOR_POCKETS: usize = 8;
const THERMOWELL_D: f64 = 5.2;
const BAG_BARRIER_FRAME_Z: f64 = 16.0;

const WASH_X: f64 = 460.0;
const WASH_Y: f64 = 268.0;
const WASH_Z: f64 = 148.0;
const WASH_POS_X: f64 = 220.0;
const WASH_POS_Y: f64 = 172.0;
const WASH_PORTS: usize = 8;
const WASH_PUMP_LANES: usize = 4;
const WASH_CONNECTOR_D: f64 = 24.0;
const WASH_CONCENTRATE_PLACEHOLDERS: usize = 2;

const SAMPLE_X: f64 = 430.0;
const SAMPLE_Y: f64 = 222.0;
const SAMPLE_Z: f64 = 72.0;
const SAMPLE_POS_X: f64 = 235.0;
const SAMPLE_POS_Y: f64 = -154.0;
const SAMPLE_PORTS: usize = 8;
const COUNT_VIABILITY_CARTRIDGES: usize = 4;

const CUSTODY_X: f64 = 344.0;
const CUSTODY_Y: f64 = 232.0;
const CUSTODY_Z: f64 = 48.0;
const CUSTODY_POS_X: f64 = -28.0;
const CUSTODY_POS_Y: f64 = -276.0;
const STERILITY_SLOTS: usize = 6;
const MYCOPLASMA_SLOTS: usize = 8;
const RETAIN_SLOTS: usize = 4;

const LABEL_LANDS: usize = 16;
const LABEL_X: f64 = 76.0;
const LABEL_Y: f64 = 30.0;
const LABEL_Z: f64 = 4.0;
const PASSAGE_LANDS: usize = 4;

const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_X: f64 = 132.0;
const STATUS_LANE_Y: f64 = 282.0;
const STATUS_LANE_Z: f64 = 44.0;
const STATUS_POS_X: f64 = -500.0;
const STATUS_POS_Y: f64 = 218.0;

const ZONE_WALL_X: f64 = 26.0;
const ZONE_WALL_Y: f64 = 760.0;
const ZONE_WALL_Z: f64 = 166.0;
const ZONE_WALL_POS_X: f64 = -130.0;
const ZONE_WALL_POS_Y: f64 = 24.0;
const PASS_THROUGH_PORTS: usize = 6;

const HANDOFF_DOCKS: usize = 3;
const HANDOFF_X: f64 = 184.0;
const HANDOFF_Y: f64 = 86.0;
const HANDOFF_Z: f64 = 36.0;
const HANDOFF_POS_X: f64 = MODULE_X / 2.0 - 124.0;
const HANDOFF_PITCH_Y: f64 = 176.0;
const HANDOFF_CONNECTORS_PER_DOCK: usize = 3;

const ROBOT_FRONT_CLEARANCE: f64 = 480.0;
const SERVICE_REAR_CLEARANCE: f64 = 300.0;
const SERVICE_SIDE_CLEARANCE: f64 = 260.0;
const TOP_CLEARANCE: f64 = 390.0;
const THAW_LID_SWING_CLEARANCE: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let receiving = cryovial_bag_receiving_nest();
    export(OUTPUTS[1], &receiving);

    let thaw = controlled_thaw_block();
    export(OUTPUTS[2], &thaw);

    let wash = closed_wash_concentrate_interface();
    export(OUTPUTS[3], &wash);

    let sample = count_viability_sample_handoff();
    export(OUTPUTS[4], &sample);

    let custody = sterility_mycoplasma_custody_slots();
    export(OUTPUTS[5], &custody);

    let labels = barcode_passage_lot_lands();
    export(OUTPUTS[6], &labels);

    let status = released_hold_reject_segregation();
    export(OUTPUTS[7], &status);

    let zones = cold_warm_zone_separation();
    export(OUTPUTS[8], &zones);

    let handoff = downstream_handoff_interface();
    export(OUTPUTS[9], &handoff);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + receiving
        + thaw
        + wash
        + sample
        + custody
        + labels
        + status
        + zones
        + handoff
        + keepouts
        + tubing_route_placeholders();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cell-bank recovery/thaw station:");
    println!(
        "  Deck/leak containment:      {MODULE_X:.0}mm x {MODULE_Y:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm recessed sump, {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Cell-bank receiving:        {BAG_SLOTS} cryobag slots plus {CRYOVIAL_POSITIONS} cryovial positions in a {RECEIVING_X:.0}mm x {RECEIVING_Y:.0}mm nest"
    );
    println!(
        "  Controlled thaw envelope:   {THAW_CHAMBERS} dry contact thaw chambers, {THAW_SENSOR_POCKETS} thermowell/sensing pockets, {THAW_LID_SWING_CLEARANCE:.0}mm lid swing gauge"
    );
    println!(
        "  Closed processing handoff:  {WASH_PORTS} wash/concentrate ports, {WASH_PUMP_LANES} pump lanes, {WASH_CONCENTRATE_PLACEHOLDERS} bought equipment placeholders"
    );
    println!(
        "  QC/custody staging:         {SAMPLE_PORTS} count/viability handoff ports, {STERILITY_SLOTS} sterility slots, {MYCOPLASMA_SLOTS} mycoplasma slots, {RETAIN_SLOTS} retain slots"
    );
    println!(
        "  Traceability/segregation:   {LABEL_LANDS} barcode/lot lands, {PASSAGE_LANDS} passage lands, {STATUS_LANES} released/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each"
    );
    println!(
        "  Downstream handoffs:        {HANDOFF_DOCKS} closed docks to harvest/passaging, suspension prep, and archive/waste paths"
    );
    println!(
        "  Robot/service keepouts:     front {ROBOT_FRONT_CLEARANCE:.0}mm, rear {SERVICE_REAR_CLEARANCE:.0}mm, side {SERVICE_SIDE_CLEARANCE:.0}mm, top {TOP_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(DATUM_PINS, 8);
    assert_eq!(CRYOVIAL_POSITIONS, CRYOVIAL_ROWS * CRYOVIAL_COLS);
    assert!(receiver_right_edge() < ZONE_WALL_POS_X - ZONE_WALL_X / 2.0 - 12.0);
    assert!(thaw_left_edge() > ZONE_WALL_POS_X + ZONE_WALL_X / 2.0 + 22.0);
    assert!(wash_right_edge() < HANDOFF_POS_X - HANDOFF_X / 2.0 - 42.0);
    assert!(sample_right_edge() < HANDOFF_POS_X - HANDOFF_X / 2.0 - 38.0);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= STATUS_LANES + HANDOFF_DOCKS);
    assert!(WASH_PORTS >= HANDOFF_DOCKS * HANDOFF_CONNECTORS_PER_DOCK - 1);
    assert!(LABEL_LANDS >= CRYOVIAL_ROWS + CRYOVIAL_COLS + PASSAGE_LANDS);
    assert!(TOP_CLEARANCE > WASH_Z + DECK_Z);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("cell_bank_recovery_base_deck", MODULE_X, MODULE_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let sump_cut = centered_cube(
        "cell_bank_recovery_recessed_leak_sump_cut",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -10.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        "cell_bank_recovery_front_drain_penetration",
        DRAIN_D / 2.0,
        RIM_W + 34.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MODULE_X / 2.0 - 112.0, -MODULE_Y / 2.0 + 14.0, DECK_Z - 5.0);

    deck - sump_cut - drain + tray_rims() + datum_pin_bosses() + leak_witness_strips()
}

fn tray_rims() -> Part {
    let front = centered_cube("cell_bank_recovery_front_rim", MODULE_X, RIM_W, RIM_Z).translate(
        0.0,
        -MODULE_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("cell_bank_recovery_rear_rim", MODULE_X, RIM_W, RIM_Z).translate(
        0.0,
        MODULE_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("cell_bank_recovery_left_rim", RIM_W, MODULE_Y, RIM_Z).translate(
        -MODULE_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("cell_bank_recovery_right_rim", RIM_W, MODULE_Y, RIM_Z).translate(
        MODULE_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn datum_pin_bosses() -> Part {
    let mut bosses = Part::empty("cell_bank_recovery_datum_pin_bosses");
    let points = [
        (-610.0, -370.0),
        (-250.0, -370.0),
        (250.0, -370.0),
        (610.0, -370.0),
        (-610.0, 370.0),
        (-250.0, 370.0),
        (250.0, 370.0),
        (610.0, 370.0),
    ];
    for (i, (x, y)) in points.into_iter().enumerate() {
        let boss = centered_cylinder(format!("cell_bank_recovery_datum_boss_{i}"), 13.0, 8.0, 36)
            .translate(x, y, DECK_Z + 4.0);
        let bore = centered_cylinder(
            format!("cell_bank_recovery_datum_pin_bore_{i}"),
            4.0,
            10.0,
            28,
        )
        .translate(x, y, DECK_Z + 5.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn leak_witness_strips() -> Part {
    let mut strips = Part::empty("cell_bank_recovery_leak_witness_strips");
    for (i, y) in [-308.0, 0.0, 308.0].into_iter().enumerate() {
        strips = strips
            + centered_cube(
                format!("cell_bank_recovery_front_leak_witness_strip_{i}"),
                92.0,
                9.0,
                3.0,
            )
            .translate(MODULE_X / 2.0 - 182.0, y, DECK_Z + 1.5);
    }
    strips
}

fn cryovial_bag_receiving_nest() -> Part {
    let z = DECK_Z + RECEIVING_Z / 2.0;
    let body = centered_cube(
        "cell_bank_recovery_receiving_nest_body",
        RECEIVING_X,
        RECEIVING_Y,
        RECEIVING_Z,
    )
    .translate(RECEIVING_POS_X, RECEIVING_POS_Y, z);

    let mut cuts = Part::empty("cell_bank_recovery_receiving_nest_recess_cuts");
    let bag_y = RECEIVING_POS_Y + 44.0;
    for i in 0..BAG_SLOTS {
        let x = RECEIVING_POS_X - 118.0 + i as f64 * 236.0;
        cuts = cuts
            + centered_cube(
                format!("cell_bank_recovery_cryobag_recess_cut_{i}"),
                BAG_SLOT_X,
                BAG_SLOT_Y,
                BAG_SLOT_RECESS_Z + 2.0,
            )
            .translate(
                x,
                bag_y,
                DECK_Z + RECEIVING_Z - BAG_SLOT_RECESS_Z / 2.0 + 1.0,
            );
    }

    let vial_origin_x = RECEIVING_POS_X - (CRYOVIAL_COLS as f64 - 1.0) * CRYOVIAL_PITCH_X / 2.0;
    let vial_origin_y = RECEIVING_POS_Y - 144.0;
    for row in 0..CRYOVIAL_ROWS {
        for col in 0..CRYOVIAL_COLS {
            let x = vial_origin_x + col as f64 * CRYOVIAL_PITCH_X;
            let y = vial_origin_y + row as f64 * CRYOVIAL_PITCH_Y;
            cuts = cuts
                + centered_cylinder(
                    format!("cell_bank_recovery_cryovial_well_cut_{row}_{col}"),
                    CRYOVIAL_WELL_D / 2.0,
                    42.0,
                    32,
                )
                .translate(x, y, DECK_Z + RECEIVING_Z - 20.0);
        }
    }

    body - cuts + cryobag_retainers() + receiving_lot_tabs() + insulated_transfer_sleeves()
}

fn cryobag_retainers() -> Part {
    let mut rails = Part::empty("cell_bank_recovery_cryobag_retainers");
    let bag_y = RECEIVING_POS_Y + 44.0;
    for i in 0..BAG_SLOTS {
        let x = RECEIVING_POS_X - 118.0 + i as f64 * 236.0;
        let left = centered_cube(
            format!("cell_bank_recovery_cryobag_left_retainer_{i}"),
            10.0,
            BAG_SLOT_Y + 28.0,
            24.0,
        )
        .translate(
            x - BAG_SLOT_X / 2.0 - 10.0,
            bag_y,
            DECK_Z + RECEIVING_Z + 12.0,
        );
        let right = centered_cube(
            format!("cell_bank_recovery_cryobag_right_retainer_{i}"),
            10.0,
            BAG_SLOT_Y + 28.0,
            24.0,
        )
        .translate(
            x + BAG_SLOT_X / 2.0 + 10.0,
            bag_y,
            DECK_Z + RECEIVING_Z + 12.0,
        );
        let back = centered_cube(
            format!("cell_bank_recovery_cryobag_back_stop_{i}"),
            BAG_SLOT_X + 42.0,
            10.0,
            24.0,
        )
        .translate(
            x,
            bag_y + BAG_SLOT_Y / 2.0 + 10.0,
            DECK_Z + RECEIVING_Z + 12.0,
        );
        rails = rails + left + right + back;
    }
    rails
}

fn receiving_lot_tabs() -> Part {
    let mut tabs = Part::empty("cell_bank_recovery_receiving_lot_tabs");
    for i in 0..BAG_SLOTS {
        tabs = tabs
            + centered_cube(
                format!("cell_bank_recovery_cryobag_lot_tab_{i}"),
                92.0,
                28.0,
                8.0,
            )
            .translate(
                RECEIVING_POS_X - 118.0 + i as f64 * 236.0,
                RECEIVING_POS_Y + RECEIVING_Y / 2.0 - 28.0,
                DECK_Z + RECEIVING_Z + 4.0,
            );
    }
    tabs
}

fn insulated_transfer_sleeves() -> Part {
    let sleeve = centered_cube(
        "cell_bank_recovery_insulated_transfer_sleeve_land",
        RECEIVING_X - 70.0,
        36.0,
        18.0,
    )
    .translate(
        RECEIVING_POS_X,
        RECEIVING_POS_Y - RECEIVING_Y / 2.0 + 38.0,
        DECK_Z + RECEIVING_Z + 9.0,
    );
    let throat = centered_cube(
        "cell_bank_recovery_transfer_sleeve_throat",
        130.0,
        46.0,
        26.0,
    )
    .translate(
        RECEIVING_POS_X + RECEIVING_X / 2.0 - 86.0,
        RECEIVING_POS_Y - RECEIVING_Y / 2.0 + 40.0,
        DECK_Z + RECEIVING_Z + 13.0,
    );
    sleeve + throat
}

fn controlled_thaw_block() -> Part {
    let z = DECK_Z + THAW_Z / 2.0;
    let block = centered_cube(
        "cell_bank_recovery_controlled_thaw_block_body",
        THAW_X,
        THAW_Y,
        THAW_Z,
    )
    .translate(THAW_POS_X, THAW_POS_Y, z);

    let mut cuts = Part::empty("cell_bank_recovery_thaw_block_recesses");
    for i in 0..THAW_CHAMBERS {
        let x = THAW_POS_X - 126.0 + i as f64 * 252.0;
        cuts = cuts
            + centered_cube(
                format!("cell_bank_recovery_thaw_chamber_recess_{i}"),
                THAW_CHAMBER_X,
                THAW_CHAMBER_Y,
                THAW_RECESS_Z + 2.0,
            )
            .translate(x, THAW_POS_Y, DECK_Z + THAW_Z - THAW_RECESS_Z / 2.0 + 1.0);
    }

    for i in 0..THAW_SENSOR_POCKETS {
        let x = THAW_POS_X - 220.0 + i as f64 * (440.0 / (THAW_SENSOR_POCKETS as f64 - 1.0));
        let y = if i % 2 == 0 {
            THAW_POS_Y - THAW_Y / 2.0 + 26.0
        } else {
            THAW_POS_Y + THAW_Y / 2.0 - 26.0
        };
        cuts = cuts
            + centered_cylinder(
                format!("cell_bank_recovery_thaw_thermowell_bore_{i}"),
                THERMOWELL_D / 2.0,
                90.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, DECK_Z + THAW_Z - 24.0);
    }

    block - cuts + thaw_chamber_lid_frames() + dry_contact_plate_envelopes() + lid_swing_gauge()
}

fn thaw_chamber_lid_frames() -> Part {
    let mut frames = Part::empty("cell_bank_recovery_thaw_chamber_lid_frames");
    for i in 0..THAW_CHAMBERS {
        let x = THAW_POS_X - 126.0 + i as f64 * 252.0;
        let front = centered_cube(
            format!("cell_bank_recovery_thaw_barrier_frame_front_{i}"),
            THAW_CHAMBER_X + 32.0,
            9.0,
            BAG_BARRIER_FRAME_Z,
        )
        .translate(
            x,
            THAW_POS_Y - THAW_CHAMBER_Y / 2.0 - 8.0,
            DECK_Z + THAW_Z + BAG_BARRIER_FRAME_Z / 2.0,
        );
        let rear = centered_cube(
            format!("cell_bank_recovery_thaw_barrier_frame_rear_{i}"),
            THAW_CHAMBER_X + 32.0,
            9.0,
            BAG_BARRIER_FRAME_Z,
        )
        .translate(
            x,
            THAW_POS_Y + THAW_CHAMBER_Y / 2.0 + 8.0,
            DECK_Z + THAW_Z + BAG_BARRIER_FRAME_Z / 2.0,
        );
        let left = centered_cube(
            format!("cell_bank_recovery_thaw_barrier_frame_left_{i}"),
            9.0,
            THAW_CHAMBER_Y + 34.0,
            BAG_BARRIER_FRAME_Z,
        )
        .translate(
            x - THAW_CHAMBER_X / 2.0 - 8.0,
            THAW_POS_Y,
            DECK_Z + THAW_Z + BAG_BARRIER_FRAME_Z / 2.0,
        );
        let right = centered_cube(
            format!("cell_bank_recovery_thaw_barrier_frame_right_{i}"),
            9.0,
            THAW_CHAMBER_Y + 34.0,
            BAG_BARRIER_FRAME_Z,
        )
        .translate(
            x + THAW_CHAMBER_X / 2.0 + 8.0,
            THAW_POS_Y,
            DECK_Z + THAW_Z + BAG_BARRIER_FRAME_Z / 2.0,
        );
        frames = frames + front + rear + left + right;
    }
    frames
}

fn dry_contact_plate_envelopes() -> Part {
    let mut plates = Part::empty("cell_bank_recovery_dry_contact_plate_envelopes");
    for i in 0..THAW_CHAMBERS {
        let x = THAW_POS_X - 126.0 + i as f64 * 252.0;
        plates = plates
            + centered_cube(
                format!("cell_bank_recovery_lower_dry_contact_plate_{i}"),
                THAW_CHAMBER_X - 18.0,
                THAW_CHAMBER_Y - 18.0,
                6.0,
            )
            .translate(x, THAW_POS_Y, DECK_Z + THAW_Z - 31.0)
            + centered_cube(
                format!("cell_bank_recovery_upper_dry_contact_plate_stowed_{i}"),
                THAW_CHAMBER_X - 22.0,
                18.0,
                10.0,
            )
            .translate(
                x,
                THAW_POS_Y + THAW_CHAMBER_Y / 2.0 + 32.0,
                DECK_Z + THAW_Z + 26.0,
            );
    }
    plates
}

fn lid_swing_gauge() -> Part {
    keepout_frame(
        "cell_bank_recovery_thaw_lid_swing_gauge",
        THAW_X + 70.0,
        THAW_Y + 78.0,
        THAW_LID_SWING_CLEARANCE,
        THAW_POS_X,
        THAW_POS_Y,
        DECK_Z + THAW_LID_SWING_CLEARANCE / 2.0,
    )
}

fn closed_wash_concentrate_interface() -> Part {
    let z = DECK_Z + WASH_Z / 2.0;
    let base = centered_cube(
        "cell_bank_recovery_wash_concentrate_dock_plate",
        WASH_X,
        WASH_Y,
        40.0,
    )
    .translate(WASH_POS_X, WASH_POS_Y, DECK_Z + 20.0);

    let envelope = centered_cube(
        "cell_bank_recovery_bought_wash_concentrate_equipment_envelope",
        WASH_X - 72.0,
        WASH_Y - 74.0,
        WASH_Z,
    )
    .translate(WASH_POS_X, WASH_POS_Y + 8.0, z + 18.0);

    base + envelope + wash_port_bulkhead() + pump_lane_placeholders() + concentrate_bag_saddles()
}

fn wash_port_bulkhead() -> Part {
    let panel_y = WASH_POS_Y - WASH_Y / 2.0 - 24.0;
    let panel = centered_cube(
        "cell_bank_recovery_wash_port_bulkhead_panel",
        WASH_X - 48.0,
        28.0,
        96.0,
    )
    .translate(WASH_POS_X, panel_y, DECK_Z + 78.0);

    let mut collars = Part::empty("cell_bank_recovery_wash_port_bulkhead_collars");
    let origin_x = WASH_POS_X - (WASH_PORTS as f64 - 1.0) * 42.0 / 2.0;
    for i in 0..WASH_PORTS {
        let x = origin_x + i as f64 * 42.0;
        let outer = centered_cylinder(
            format!("cell_bank_recovery_wash_connector_collar_{i}"),
            WASH_CONNECTOR_D / 2.0,
            12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, panel_y - 16.0, DECK_Z + 80.0);
        let inner = centered_cylinder(
            format!("cell_bank_recovery_wash_connector_opening_{i}"),
            7.0,
            14.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, panel_y - 16.0, DECK_Z + 80.0);
        collars = collars + (outer - inner);
    }
    panel + collars
}

fn pump_lane_placeholders() -> Part {
    let mut lanes = Part::empty("cell_bank_recovery_wash_pump_lane_placeholders");
    let origin_x = WASH_POS_X - (WASH_PUMP_LANES as f64 - 1.0) * 82.0 / 2.0;
    for i in 0..WASH_PUMP_LANES {
        let x = origin_x + i as f64 * 82.0;
        lanes = lanes
            + centered_cube(
                format!("cell_bank_recovery_closed_pump_lane_{i}"),
                52.0,
                126.0,
                22.0,
            )
            .translate(x, WASH_POS_Y + 18.0, DECK_Z + 58.0)
            + centered_cylinder(
                format!("cell_bank_recovery_pump_lane_rotor_access_{i}"),
                22.0,
                10.0,
                40,
            )
            .translate(x, WASH_POS_Y + 18.0, DECK_Z + 74.0);
    }
    lanes
}

fn concentrate_bag_saddles() -> Part {
    let mut saddles = Part::empty("cell_bank_recovery_concentrate_bag_saddles");
    for i in 0..WASH_CONCENTRATE_PLACEHOLDERS {
        saddles = saddles
            + centered_cube(
                format!("cell_bank_recovery_concentrate_bag_saddle_{i}"),
                132.0,
                66.0,
                18.0,
            )
            .translate(
                WASH_POS_X - 86.0 + i as f64 * 172.0,
                WASH_POS_Y + WASH_Y / 2.0 - 38.0,
                DECK_Z + WASH_Z + 48.0,
            );
    }
    saddles
}

fn count_viability_sample_handoff() -> Part {
    let z = DECK_Z + SAMPLE_Z / 2.0;
    let dock = centered_cube(
        "cell_bank_recovery_count_viability_dock",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_POS_X, SAMPLE_POS_Y, z);

    let analyzer_placeholder = centered_cube(
        "cell_bank_recovery_count_viability_analyzer_envelope",
        260.0,
        128.0,
        128.0,
    )
    .translate(
        SAMPLE_POS_X + 42.0,
        SAMPLE_POS_Y + 20.0,
        DECK_Z + SAMPLE_Z + 64.0,
    );

    dock - sample_port_cuts() + analyzer_placeholder + sample_loop_lands() + cartridge_staging()
}

fn sample_port_cuts() -> Part {
    let mut cuts = Part::empty("cell_bank_recovery_sample_port_cuts");
    let origin_x = SAMPLE_POS_X - (SAMPLE_PORTS as f64 - 1.0) * 38.0 / 2.0;
    for i in 0..SAMPLE_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("cell_bank_recovery_count_viability_sample_port_cut_{i}"),
                5.0,
                78.0,
                24,
            )
            .translate(
                origin_x + i as f64 * 38.0,
                SAMPLE_POS_Y - 72.0,
                DECK_Z + 48.0,
            );
    }
    cuts
}

fn sample_loop_lands() -> Part {
    let mut lands = Part::empty("cell_bank_recovery_sample_loop_lands");
    let origin_x = SAMPLE_POS_X - (SAMPLE_PORTS as f64 - 1.0) * 38.0 / 2.0;
    for i in 0..SAMPLE_PORTS {
        lands = lands
            + centered_cube(
                format!("cell_bank_recovery_sample_loop_land_{i}"),
                28.0,
                46.0,
                8.0,
            )
            .translate(
                origin_x + i as f64 * 38.0,
                SAMPLE_POS_Y - 72.0,
                DECK_Z + SAMPLE_Z + 4.0,
            );
    }
    lands
}

fn cartridge_staging() -> Part {
    let mut holders = Part::empty("cell_bank_recovery_count_viability_cartridge_staging");
    for i in 0..COUNT_VIABILITY_CARTRIDGES {
        holders = holders
            + centered_cube(
                format!("cell_bank_recovery_viability_cartridge_slot_{i}"),
                44.0,
                72.0,
                18.0,
            )
            .translate(
                SAMPLE_POS_X - 78.0 + i as f64 * 52.0,
                SAMPLE_POS_Y + SAMPLE_Y / 2.0 - 42.0,
                DECK_Z + SAMPLE_Z + 9.0,
            );
    }
    holders
}

fn sterility_mycoplasma_custody_slots() -> Part {
    let base = centered_cube(
        "cell_bank_recovery_sterility_mycoplasma_custody_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS_X, CUSTODY_POS_Y, DECK_Z + CUSTODY_Z / 2.0);

    base - custody_slot_cuts()
        + custody_slot_labels()
        + retain_sample_cold_finger()
        + tamper_evidence_card_lands()
}

fn custody_slot_cuts() -> Part {
    let mut cuts = Part::empty("cell_bank_recovery_custody_slot_cuts");
    for i in 0..STERILITY_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("cell_bank_recovery_sterility_custody_slot_cut_{i}"),
                34.0,
                52.0,
                28.0,
            )
            .translate(
                CUSTODY_POS_X - 126.0 + i as f64 * 42.0,
                CUSTODY_POS_Y + 58.0,
                DECK_Z + CUSTODY_Z - 13.0,
            );
    }
    for i in 0..MYCOPLASMA_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("cell_bank_recovery_mycoplasma_custody_slot_cut_{i}"),
                30.0,
                46.0,
                26.0,
            )
            .translate(
                CUSTODY_POS_X - 122.0 + i as f64 * 35.0,
                CUSTODY_POS_Y - 8.0,
                DECK_Z + CUSTODY_Z - 12.0,
            );
    }
    for i in 0..RETAIN_SLOTS {
        cuts = cuts
            + centered_cylinder(
                format!("cell_bank_recovery_retain_sample_well_cut_{i}"),
                10.0,
                32.0,
                28,
            )
            .translate(
                CUSTODY_POS_X - 66.0 + i as f64 * 44.0,
                CUSTODY_POS_Y - 82.0,
                DECK_Z + CUSTODY_Z - 15.0,
            );
    }
    cuts
}

fn custody_slot_labels() -> Part {
    let sterility = centered_cube(
        "cell_bank_recovery_sterility_row_label_land",
        116.0,
        18.0,
        4.0,
    )
    .translate(
        CUSTODY_POS_X - 96.0,
        CUSTODY_POS_Y + 106.0,
        DECK_Z + CUSTODY_Z + 2.0,
    );
    let myco = centered_cube(
        "cell_bank_recovery_mycoplasma_row_label_land",
        142.0,
        18.0,
        4.0,
    )
    .translate(
        CUSTODY_POS_X + 72.0,
        CUSTODY_POS_Y + 106.0,
        DECK_Z + CUSTODY_Z + 2.0,
    );
    sterility + myco
}

fn retain_sample_cold_finger() -> Part {
    centered_cube(
        "cell_bank_recovery_retain_sample_cold_finger_envelope",
        222.0,
        28.0,
        32.0,
    )
    .translate(
        CUSTODY_POS_X,
        CUSTODY_POS_Y - 110.0,
        DECK_Z + CUSTODY_Z + 16.0,
    )
}

fn tamper_evidence_card_lands() -> Part {
    let mut lands = Part::empty("cell_bank_recovery_tamper_evidence_card_lands");
    for i in 0..3 {
        lands = lands
            + centered_cube(
                format!("cell_bank_recovery_tamper_evidence_card_land_{i}"),
                68.0,
                28.0,
                4.0,
            )
            .translate(
                CUSTODY_POS_X - 82.0 + i as f64 * 82.0,
                CUSTODY_POS_Y + CUSTODY_Y / 2.0 - 20.0,
                DECK_Z + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn barcode_passage_lot_lands() -> Part {
    let mut lands = Part::empty("cell_bank_recovery_barcode_passage_lot_lands");
    for i in 0..LABEL_LANDS {
        let x = -610.0 + (i % 8) as f64 * 174.0;
        let y = if i < 8 {
            -MODULE_Y / 2.0 + 64.0
        } else {
            MODULE_Y / 2.0 - 64.0
        };
        lands = lands
            + centered_cube(
                format!("cell_bank_recovery_barcode_lot_land_{i}"),
                LABEL_X,
                LABEL_Y,
                LABEL_Z,
            )
            .translate(x, y, DECK_Z + LABEL_Z / 2.0 + 2.0);
    }

    for i in 0..PASSAGE_LANDS {
        lands = lands
            + centered_cube(
                format!("cell_bank_recovery_passage_number_land_{i}"),
                92.0,
                34.0,
                4.0,
            )
            .translate(
                -210.0 + i as f64 * 140.0,
                -MODULE_Y / 2.0 + 112.0,
                DECK_Z + 4.0,
            );
    }

    lands + scan_bridge()
}

fn scan_bridge() -> Part {
    let left = centered_cube(
        "cell_bank_recovery_scan_bridge_left_post",
        24.0,
        32.0,
        156.0,
    )
    .translate(-660.0, -MODULE_Y / 2.0 + 128.0, DECK_Z + 78.0);
    let right = centered_cube(
        "cell_bank_recovery_scan_bridge_right_post",
        24.0,
        32.0,
        156.0,
    )
    .translate(660.0, -MODULE_Y / 2.0 + 128.0, DECK_Z + 78.0);
    let bar = centered_cube(
        "cell_bank_recovery_barcode_rfid_camera_bridge",
        1344.0,
        30.0,
        26.0,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 128.0, DECK_Z + 156.0);
    let camera = centered_cube(
        "cell_bank_recovery_barcode_rfid_camera_envelope",
        86.0,
        54.0,
        42.0,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 134.0, DECK_Z + 116.0);
    left + right + bar + camera
}

fn released_hold_reject_segregation() -> Part {
    let mut lanes = Part::empty("cell_bank_recovery_status_lanes");
    for lane in 0..STATUS_LANES {
        let x = STATUS_POS_X + lane as f64 * (STATUS_LANE_X + 28.0);
        let lane_body = centered_cube(
            format!("cell_bank_recovery_status_lane_body_{lane}"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(x, STATUS_POS_Y, DECK_Z + STATUS_LANE_Z / 2.0);

        let mut cuts = Part::empty(format!("cell_bank_recovery_status_lane_cuts_{lane}"));
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("cell_bank_recovery_status_lane_{lane}_slot_cut_{slot}"),
                    STATUS_LANE_X - 38.0,
                    44.0,
                    26.0,
                )
                .translate(
                    x,
                    STATUS_POS_Y - 92.0 + slot as f64 * 62.0,
                    DECK_Z + STATUS_LANE_Z - 12.0,
                );
        }

        lanes = lanes + (lane_body - cuts) + status_lane_label(lane, x);
    }
    lanes + status_lane_barrier()
}

fn status_lane_label(lane: usize, x: f64) -> Part {
    centered_cube(
        format!("cell_bank_recovery_status_lane_label_{lane}"),
        STATUS_LANE_X - 26.0,
        20.0,
        5.0,
    )
    .translate(
        x,
        STATUS_POS_Y + STATUS_LANE_Y / 2.0 - 18.0,
        DECK_Z + STATUS_LANE_Z + 2.5,
    )
}

fn status_lane_barrier() -> Part {
    centered_cube(
        "cell_bank_recovery_released_hold_reject_flow_barrier",
        STATUS_LANES as f64 * STATUS_LANE_X + 88.0,
        14.0,
        82.0,
    )
    .translate(STATUS_POS_X + 160.0, STATUS_POS_Y - 156.0, DECK_Z + 41.0)
}

fn cold_warm_zone_separation() -> Part {
    let wall = centered_cube(
        "cell_bank_recovery_cold_warm_zone_separation_wall",
        ZONE_WALL_X,
        ZONE_WALL_Y,
        ZONE_WALL_Z,
    )
    .translate(ZONE_WALL_POS_X, ZONE_WALL_POS_Y, DECK_Z + ZONE_WALL_Z / 2.0);

    let mut ports = Part::empty("cell_bank_recovery_zone_pass_through_ports");
    let origin_y = ZONE_WALL_POS_Y - (PASS_THROUGH_PORTS as f64 - 1.0) * 82.0 / 2.0;
    for i in 0..PASS_THROUGH_PORTS {
        ports = ports
            + centered_cylinder(
                format!("cell_bank_recovery_cold_warm_pass_through_collar_{i}"),
                18.0,
                18.0,
                36,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(ZONE_WALL_POS_X, origin_y + i as f64 * 82.0, DECK_Z + 92.0);
    }

    wall + ports + condensate_gutter() + zone_label_lands()
}

fn condensate_gutter() -> Part {
    centered_cube(
        "cell_bank_recovery_cold_zone_condensate_gutter",
        42.0,
        ZONE_WALL_Y - 52.0,
        18.0,
    )
    .translate(ZONE_WALL_POS_X - 44.0, ZONE_WALL_POS_Y, DECK_Z + 9.0)
}

fn zone_label_lands() -> Part {
    let cold = centered_cube("cell_bank_recovery_cold_zone_label_land", 112.0, 30.0, 5.0)
        .translate(
            ZONE_WALL_POS_X - 84.0,
            ZONE_WALL_POS_Y + ZONE_WALL_Y / 2.0 - 42.0,
            DECK_Z + 8.0,
        );
    let warm = centered_cube("cell_bank_recovery_warm_zone_label_land", 112.0, 30.0, 5.0)
        .translate(
            ZONE_WALL_POS_X + 84.0,
            ZONE_WALL_POS_Y + ZONE_WALL_Y / 2.0 - 42.0,
            DECK_Z + 8.0,
        );
    cold + warm
}

fn downstream_handoff_interface() -> Part {
    let mut handoffs = Part::empty("cell_bank_recovery_downstream_handoff_interface");
    for i in 0..HANDOFF_DOCKS {
        let y = handoff_y(i);
        let dock = centered_cube(
            format!("cell_bank_recovery_downstream_handoff_dock_{i}"),
            HANDOFF_X,
            HANDOFF_Y,
            HANDOFF_Z,
        )
        .translate(HANDOFF_POS_X, y, DECK_Z + HANDOFF_Z / 2.0);

        let key = centered_cube(
            format!("cell_bank_recovery_downstream_handoff_keyway_{i}"),
            HANDOFF_X - 36.0,
            12.0,
            10.0,
        )
        .translate(HANDOFF_POS_X, y, DECK_Z + HANDOFF_Z + 5.0);

        let mut connectors = Part::empty(format!(
            "cell_bank_recovery_downstream_handoff_connectors_{i}"
        ));
        for c in 0..HANDOFF_CONNECTORS_PER_DOCK {
            connectors = connectors
                + centered_cylinder(
                    format!("cell_bank_recovery_handoff_{i}_connector_{c}"),
                    10.0,
                    18.0,
                    32,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(
                    HANDOFF_POS_X - HANDOFF_X / 2.0 - 10.0,
                    y - 24.0 + c as f64 * 24.0,
                    DECK_Z + HANDOFF_Z + 18.0,
                );
        }
        handoffs = handoffs + dock + key + connectors;
    }
    handoffs + handoff_tubing_comb()
}

fn handoff_tubing_comb() -> Part {
    let mut comb = Part::empty("cell_bank_recovery_handoff_tubing_comb");
    for i in 0..(HANDOFF_DOCKS * HANDOFF_CONNECTORS_PER_DOCK) {
        comb = comb
            + centered_cube(
                format!("cell_bank_recovery_handoff_tube_comb_slot_{i}"),
                72.0,
                8.0,
                18.0,
            )
            .translate(
                HANDOFF_POS_X - 132.0,
                -236.0 + i as f64 * 58.0,
                DECK_Z + 34.0,
            );
    }
    comb
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_frame(
        "cell_bank_recovery_front_robot_approach_keepout",
        MODULE_X - 80.0,
        ROBOT_FRONT_CLEARANCE,
        210.0,
        0.0,
        -MODULE_Y / 2.0 - ROBOT_FRONT_CLEARANCE / 2.0 + 22.0,
        105.0,
    );
    let rear_service = keepout_frame(
        "cell_bank_recovery_rear_service_keepout",
        MODULE_X - 160.0,
        SERVICE_REAR_CLEARANCE,
        220.0,
        0.0,
        MODULE_Y / 2.0 + SERVICE_REAR_CLEARANCE / 2.0 - 12.0,
        110.0,
    );
    let side_service = keepout_frame(
        "cell_bank_recovery_right_module_handoff_service_keepout",
        SERVICE_SIDE_CLEARANCE,
        MODULE_Y - 120.0,
        240.0,
        MODULE_X / 2.0 + SERVICE_SIDE_CLEARANCE / 2.0 - 20.0,
        0.0,
        120.0,
    );
    let top = keepout_frame(
        "cell_bank_recovery_top_access_keepout",
        MODULE_X - 120.0,
        MODULE_Y - 120.0,
        TOP_CLEARANCE,
        0.0,
        0.0,
        DECK_Z + TOP_CLEARANCE / 2.0,
    );
    front_robot + rear_service + side_service + top
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64, cx: f64, cy: f64, cz: f64) -> Part {
    let t = 6.0;
    let front = centered_cube(format!("{name}_front_edge"), x, t, t).translate(
        cx,
        cy - y / 2.0,
        cz - z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear_edge"), x, t, t).translate(
        cx,
        cy + y / 2.0,
        cz - z / 2.0,
    );
    let left = centered_cube(format!("{name}_left_edge"), t, y, t).translate(
        cx - x / 2.0,
        cy,
        cz - z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_edge"), t, y, t).translate(
        cx + x / 2.0,
        cy,
        cz - z / 2.0,
    );

    let v1 = centered_cube(format!("{name}_vertical_0"), t, t, z).translate(
        cx - x / 2.0,
        cy - y / 2.0,
        cz,
    );
    let v2 = centered_cube(format!("{name}_vertical_1"), t, t, z).translate(
        cx + x / 2.0,
        cy - y / 2.0,
        cz,
    );
    let v3 = centered_cube(format!("{name}_vertical_2"), t, t, z).translate(
        cx - x / 2.0,
        cy + y / 2.0,
        cz,
    );
    let v4 = centered_cube(format!("{name}_vertical_3"), t, t, z).translate(
        cx + x / 2.0,
        cy + y / 2.0,
        cz,
    );

    let top_front = centered_cube(format!("{name}_top_front_edge"), x, t, t).translate(
        cx,
        cy - y / 2.0,
        cz + z / 2.0,
    );
    let top_rear = centered_cube(format!("{name}_top_rear_edge"), x, t, t).translate(
        cx,
        cy + y / 2.0,
        cz + z / 2.0,
    );
    let top_left = centered_cube(format!("{name}_top_left_edge"), t, y, t).translate(
        cx - x / 2.0,
        cy,
        cz + z / 2.0,
    );
    let top_right = centered_cube(format!("{name}_top_right_edge"), t, y, t).translate(
        cx + x / 2.0,
        cy,
        cz + z / 2.0,
    );

    front + rear + left + right + v1 + v2 + v3 + v4 + top_front + top_rear + top_left + top_right
}

fn tubing_route_placeholders() -> Part {
    let cold_to_thaw = tube_segment(
        "cell_bank_recovery_receiving_to_thaw_closed_tube_route",
        440.0,
        6.0,
        RECEIVING_POS_X + 250.0,
        RECEIVING_POS_Y + 58.0,
        DECK_Z + 98.0,
        0.0,
    );
    let thaw_to_wash = tube_segment(
        "cell_bank_recovery_thaw_to_wash_closed_tube_route",
        520.0,
        6.0,
        THAW_POS_X + 320.0,
        THAW_POS_Y + 54.0,
        DECK_Z + 124.0,
        0.0,
    );
    let wash_to_sample = tube_segment(
        "cell_bank_recovery_wash_to_sample_closed_sample_route",
        290.0,
        5.0,
        WASH_POS_X + 10.0,
        WASH_POS_Y - 212.0,
        DECK_Z + 112.0,
        90.0,
    );
    let wash_to_handoff = tube_segment(
        "cell_bank_recovery_wash_to_downstream_handoff_route",
        360.0,
        6.0,
        WASH_POS_X + 248.0,
        WASH_POS_Y + 18.0,
        DECK_Z + 112.0,
        0.0,
    );
    cold_to_thaw + thaw_to_wash + wash_to_sample + wash_to_handoff
}

fn tube_segment(name: &str, len: f64, diameter: f64, x: f64, y: f64, z: f64, rot_z: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, rot_z)
        .translate(x, y, z)
}

fn receiver_right_edge() -> f64 {
    RECEIVING_POS_X + RECEIVING_X / 2.0
}

fn thaw_left_edge() -> f64 {
    THAW_POS_X - THAW_X / 2.0
}

fn wash_right_edge() -> f64 {
    WASH_POS_X + WASH_X / 2.0
}

fn sample_right_edge() -> f64 {
    SAMPLE_POS_X + SAMPLE_X / 2.0
}

fn handoff_y(index: usize) -> f64 {
    -HANDOFF_PITCH_Y + index as f64 * HANDOFF_PITCH_Y
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_cell_bank_recovery_thaw_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_footprint_and_core_dimensions_match_intent() {
        assert!(MODULE_X >= 1300.0);
        assert!(MODULE_Y >= 850.0);
        assert!(SUMP_X < MODULE_X - 80.0);
        assert!(SUMP_Y < MODULE_Y - 80.0);
        assert_eq!(DATUM_PINS, 8);
        assert!(RECEIVING_X + THAW_X + WASH_X < MODULE_X * 1.2);
    }

    #[test]
    fn receiving_thaw_and_wash_capacity_are_sized_for_closed_recovery() {
        assert_eq!(BAG_SLOTS, 2);
        assert_eq!(CRYOVIAL_POSITIONS, 18);
        assert_eq!(THAW_CHAMBERS, 2);
        assert!(THAW_SENSOR_POCKETS >= THAW_CHAMBERS * 4);
        assert!(WASH_PORTS >= 8);
        assert!(WASH_PUMP_LANES >= WASH_CONCENTRATE_PLACEHOLDERS * 2);
    }

    #[test]
    fn cold_and_warm_zones_are_physically_separated() {
        assert!(receiver_right_edge() < ZONE_WALL_POS_X - ZONE_WALL_X / 2.0);
        assert!(thaw_left_edge() > ZONE_WALL_POS_X + ZONE_WALL_X / 2.0);
        assert!(PASS_THROUGH_PORTS >= HANDOFF_DOCKS + WASH_CONCENTRATE_PLACEHOLDERS);
        assert!(ZONE_WALL_Z > RECEIVING_Z + 40.0);
    }

    #[test]
    fn qc_custody_traceability_and_status_controls_are_visible() {
        assert_eq!(SAMPLE_PORTS, 8);
        assert_eq!(STERILITY_SLOTS + MYCOPLASMA_SLOTS + RETAIN_SLOTS, 18);
        assert!(LABEL_LANDS >= 16);
        assert!(PASSAGE_LANDS >= 4);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 12);
    }

    #[test]
    fn downstream_handoffs_and_service_clearances_are_explicit() {
        assert_eq!(HANDOFF_DOCKS, 3);
        assert_eq!(HANDOFF_CONNECTORS_PER_DOCK, 3);
        assert!(wash_right_edge() < HANDOFF_POS_X - HANDOFF_X / 2.0);
        assert!(sample_right_edge() < HANDOFF_POS_X - HANDOFF_X / 2.0);
        assert!(ROBOT_FRONT_CLEARANCE >= 400.0);
        assert!(SERVICE_REAR_CLEARANCE >= 250.0);
        assert!(TOP_CLEARANCE >= 350.0);
    }
}
