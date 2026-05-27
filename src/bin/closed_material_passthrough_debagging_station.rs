use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed material pass-through receiving and de-bagging station.
//
// Intent:
// - Receive sterile totes, reagent packs, and consumable pouches into the
//   isolator support workflow without open bench transfer.
// - Separate dirty outer packaging removal from clean inner pouch staging.
// - Provide scan/photo evidence, release/hold/reject segregation, contact
//   coupons, debris capture, and a keyed pass-through tongue for robot or
//   closed-hatch handoff.
//
// Product-concept CAD only: this model is mechanical packaging and workflow
// allocation, not a sterilization validation procedure or sterility claim.

const OUTPUTS: &[&str] = &[
    "output/closed_material_passthrough_debagging_station_base_leak_debris_tray.stl",
    "output/closed_material_passthrough_debagging_station_sealed_tote_transfer_hatch_receiver.stl",
    "output/closed_material_passthrough_debagging_station_outer_bag_removal_lane.stl",
    "output/closed_material_passthrough_debagging_station_sterile_inner_pouch_staging.stl",
    "output/closed_material_passthrough_debagging_station_released_hold_reject_lanes.stl",
    "output/closed_material_passthrough_debagging_station_barcode_rfid_scan_lands.stl",
    "output/closed_material_passthrough_debagging_station_wipe_contact_coupon_pockets.stl",
    "output/closed_material_passthrough_debagging_station_particle_debris_capture_tray.stl",
    "output/closed_material_passthrough_debagging_station_clean_dirty_lane_divider.stl",
    "output/closed_material_passthrough_debagging_station_passthrough_tongue_rail_interface.stl",
    "output/closed_material_passthrough_debagging_station_evidence_photo_inspection_bridge.stl",
    "output/closed_material_passthrough_debagging_station_glove_robot_reach_keepouts.stl",
    "output/closed_material_passthrough_debagging_station_waste_offcut_trap.stl",
    "output/closed_material_passthrough_debagging_station_assembly.stl",
];

const DECK_X: f64 = 1520.0;
const DECK_Y: f64 = 960.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;

const DIRTY_SIDE_Y: f64 = -302.0;
const TRANSFER_CENTER_Y: f64 = 18.0;
const CLEAN_SIDE_Y: f64 = 252.0;
const PASS_THROUGH_Y: f64 = 398.0;

const TOTE_RECEIVER_X: f64 = 840.0;
const TOTE_RECEIVER_Y: f64 = 188.0;
const TOTE_RECEIVER_Z: f64 = 74.0;
const TOTE_CLEAR_X: f64 = 705.0;
const TOTE_CLEAR_Y: f64 = 126.0;
const TOTE_GUIDE_Z: f64 = 118.0;
const HATCH_FACE_X: f64 = 902.0;
const HATCH_FACE_Z: f64 = 238.0;

const OUTER_BAG_LANE_X: f64 = 640.0;
const OUTER_BAG_LANE_Y: f64 = 220.0;
const OUTER_BAG_LANE_Z: f64 = 38.0;
const PEEL_ROLLERS: usize = 4;
const CLAMP_STATIONS: usize = 6;

const INNER_POUCH_POSITIONS: usize = 8;
const INNER_POUCH_COLS: usize = 4;
const INNER_POUCH_ROWS: usize = 2;
const INNER_POUCH_NEST_X: f64 = 170.0;
const INNER_POUCH_NEST_Y: f64 = 128.0;
const INNER_POUCH_NEST_Z: f64 = 26.0;

const RELEASE_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_X: f64 = 178.0;
const STATUS_LANE_Y: f64 = 116.0;
const STATUS_LANE_Z: f64 = 30.0;

const BARCODE_LANDS: usize = 14;
const RFID_LANDS: usize = 8;
const CUSTODY_CARD_SLOTS: usize = 6;
const LABEL_LAND_X: f64 = 94.0;
const LABEL_LAND_Y: f64 = 30.0;
const LABEL_LAND_Z: f64 = 5.0;

const WIPE_COUPONS: usize = 12;
const CONTACT_COUPONS: usize = 8;
const COUPON_BLOCK_X: f64 = 322.0;
const COUPON_BLOCK_Y: f64 = 176.0;
const COUPON_BLOCK_Z: f64 = 24.0;

const DEBRIS_TRAY_X: f64 = 420.0;
const DEBRIS_TRAY_Y: f64 = 182.0;
const DEBRIS_TRAY_Z: f64 = 34.0;
const PARTICLE_SENSOR_WELLS: usize = 4;

const DIVIDER_X: f64 = 1330.0;
const DIVIDER_Y: f64 = 26.0;
const DIVIDER_Z: f64 = 142.0;
const PASS_SLOT_X: f64 = 570.0;
const PASS_SLOT_Z: f64 = 92.0;

const TONGUE_X: f64 = 760.0;
const TONGUE_Y: f64 = 430.0;
const TONGUE_Z: f64 = 24.0;
const TONGUE_RAIL_PITCH: f64 = 636.0;
const TONGUE_RAIL_Z: f64 = 42.0;
const TONGUE_DATUM_PINS: usize = 6;

const CAMERA_BRIDGE_SPAN_X: f64 = 1050.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 286.0;
const CAMERA_PODS: usize = 4;
const LED_BARS: usize = 4;

const FRONT_GLOVE_KEEP_OUT: f64 = 390.0;
const REAR_HATCH_KEEP_OUT: f64 = 330.0;
const SIDE_TOTE_KEEP_OUT: f64 = 260.0;
const TOP_CAMERA_SERVICE_Z: f64 = 380.0;

const WASTE_TRAP_X: f64 = 292.0;
const WASTE_TRAP_Y: f64 = 210.0;
const WASTE_TRAP_Z: f64 = 92.0;
const OFFCUT_SLOTS: usize = 5;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_leak_debris_tray();
    export(&base, OUTPUTS[0]);

    let receiver = sealed_tote_transfer_hatch_receiver();
    export(&receiver, OUTPUTS[1]);

    let debag_lane = outer_bag_removal_lane();
    export(&debag_lane, OUTPUTS[2]);

    let inner_stage = sterile_inner_pouch_staging();
    export(&inner_stage, OUTPUTS[3]);

    let status = released_hold_reject_lanes();
    export(&status, OUTPUTS[4]);

    let traceability = barcode_rfid_scan_lands();
    export(&traceability, OUTPUTS[5]);

    let coupons = wipe_contact_coupon_pockets();
    export(&coupons, OUTPUTS[6]);

    let debris = particle_debris_capture_tray();
    export(&debris, OUTPUTS[7]);

    let divider = clean_dirty_lane_divider();
    export(&divider, OUTPUTS[8]);

    let tongue = passthrough_tongue_rail_interface();
    export(&tongue, OUTPUTS[9]);

    let inspection = evidence_photo_inspection_bridge();
    export(&inspection, OUTPUTS[10]);

    let keepouts = glove_robot_reach_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let waste = waste_offcut_trap();
    export(&waste, OUTPUTS[12]);

    let assembly = base
        + receiver
        + debag_lane
        + inner_stage
        + status
        + traceability
        + coupons
        + debris
        + divider
        + tongue
        + inspection
        + keepouts
        + waste;
    export(&assembly, OUTPUTS[13]);

    println!(
        "Closed material pass-through/de-bagging station: {:.0} x {:.0} mm deck; {:.0} x {:.0} mm tote receiver clear opening; {:.0} x {:.0} mm pass-through tongue.",
        DECK_X, DECK_Y, TOTE_CLEAR_X, TOTE_CLEAR_Y, TONGUE_X, TONGUE_Y
    );
    println!(
        "Workflow allocation: {} pouch nests, {} release/hold/reject slots, {} barcode lands, {} RFID lands, {} custody card slots.",
        INNER_POUCH_POSITIONS,
        RELEASE_LANES * STATUS_SLOTS_PER_LANE,
        BARCODE_LANDS,
        RFID_LANDS,
        CUSTODY_CARD_SLOTS
    );
    println!(
        "Contamination controls: {:.0} mm clean/dirty divider, {} wipe coupons, {} contact coupons, {} particle wells, {} offcut slots.",
        DIVIDER_Z, WIPE_COUPONS, CONTACT_COUPONS, PARTICLE_SENSOR_WELLS, OFFCUT_SLOTS
    );
    println!(
        "Access keepouts: front glove/robot {:.0} mm, rear hatch {:.0} mm, side tote swing {:.0} mm, top camera service {:.0} mm Z.",
        FRONT_GLOVE_KEEP_OUT, REAR_HATCH_KEEP_OUT, SIDE_TOTE_KEEP_OUT, TOP_CAMERA_SERVICE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_debris_tray() -> Part {
    let deck = centered_cube("material_passthrough_deck_pan", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let shallow_basin = centered_cube(
        "material_passthrough_recessed_wipeable_basin",
        DECK_X - 138.0,
        DECK_Y - 146.0,
        9.0,
    )
    .translate(0.0, 6.0, DECK_Z - 4.5);
    let low_point_sump = centered_cube(
        "material_passthrough_low_point_sump",
        108.0,
        58.0,
        DECK_Z + 4.0,
    )
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 68.0, DECK_Z / 2.0);
    let drain_port = centered_cylinder("material_passthrough_drain_port", 8.0, 46.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 26.0, DECK_Z - 8.0);

    deck - shallow_basin - low_point_sump - drain_port - deck_mount_holes()
        + perimeter_rim()
        + zone_floor_lands()
}

fn perimeter_rim() -> Part {
    let front = centered_cube("material_passthrough_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("material_passthrough_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("material_passthrough_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("material_passthrough_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn zone_floor_lands() -> Part {
    let dirty = centered_cube(
        "dirty_outer_packaging_zone_floor_land",
        DECK_X - 220.0,
        250.0,
        4.0,
    )
    .translate(0.0, DIRTY_SIDE_Y, DECK_Z + 2.0);
    let transfer = centered_cube(
        "debagging_transfer_zone_floor_land",
        DECK_X - 260.0,
        206.0,
        4.0,
    )
    .translate(0.0, TRANSFER_CENTER_Y, DECK_Z + 2.0);
    let clean = centered_cube(
        "clean_inner_pouch_zone_floor_land",
        DECK_X - 240.0,
        238.0,
        4.0,
    )
    .translate(0.0, CLEAN_SIDE_Y, DECK_Z + 2.0);

    dirty + transfer + clean
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("material_passthrough_mount_holes");
    let xs = [
        -DECK_X / 2.0 + 74.0,
        -DECK_X / 6.0,
        DECK_X / 6.0,
        DECK_X / 2.0 - 74.0,
    ];
    let ys = [-DECK_Y / 2.0 + 62.0, DECK_Y / 2.0 - 62.0];

    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            holes = holes
                + centered_cylinder(
                    format!("material_passthrough_m8_mount_{i}_{j}"),
                    4.4,
                    36.0,
                    28,
                )
                .translate(*x, *y, DECK_Z / 2.0);
        }
    }

    holes
}

fn sealed_tote_transfer_hatch_receiver() -> Part {
    let receiver_plate = centered_cube(
        "sealed_tote_receiver_floor_datum",
        TOTE_RECEIVER_X,
        TOTE_RECEIVER_Y,
        TOTE_RECEIVER_Z,
    )
    .translate(0.0, DIRTY_SIDE_Y, DECK_Z + TOTE_RECEIVER_Z / 2.0);
    let tote_clearance = centered_cube(
        "sealed_tote_receiver_clearance_cut",
        TOTE_CLEAR_X,
        TOTE_CLEAR_Y,
        TOTE_RECEIVER_Z + 6.0,
    )
    .translate(0.0, DIRTY_SIDE_Y, DECK_Z + TOTE_RECEIVER_Z / 2.0 + 10.0);

    let hatch_face = rectangular_frame(
        "sealed_transfer_hatch_gasket_frame",
        HATCH_FACE_X,
        16.0,
        HATCH_FACE_Z,
        TOTE_CLEAR_X,
        TOTE_CLEAR_Y,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        0.0,
        DIRTY_SIDE_Y - TOTE_RECEIVER_Y / 2.0 - 16.0,
        DECK_Z + 125.0,
    );

    receiver_plate - tote_clearance
        + hatch_face
        + tote_side_guides()
        + tote_clamp_bosses()
        + interlock_sensor_land_bar()
}

fn tote_side_guides() -> Part {
    let left = centered_cube(
        "left_keyed_tote_guide_rail",
        32.0,
        TOTE_RECEIVER_Y + 72.0,
        TOTE_GUIDE_Z,
    )
    .translate(
        -TOTE_CLEAR_X / 2.0 - 42.0,
        DIRTY_SIDE_Y - 8.0,
        DECK_Z + TOTE_GUIDE_Z / 2.0,
    );
    let right = centered_cube(
        "right_keyed_tote_guide_rail",
        32.0,
        TOTE_RECEIVER_Y + 72.0,
        TOTE_GUIDE_Z,
    )
    .translate(
        TOTE_CLEAR_X / 2.0 + 42.0,
        DIRTY_SIDE_Y - 8.0,
        DECK_Z + TOTE_GUIDE_Z / 2.0,
    );
    let rear_stop = centered_cube(
        "sealed_tote_positive_rear_stop",
        TOTE_CLEAR_X + 120.0,
        34.0,
        96.0,
    )
    .translate(
        0.0,
        DIRTY_SIDE_Y + TOTE_RECEIVER_Y / 2.0 + 30.0,
        DECK_Z + 48.0,
    );

    left + right + rear_stop
}

fn tote_clamp_bosses() -> Part {
    let mut bosses = Part::empty("sealed_tote_receiver_clamp_bosses");
    for (i, x) in [-370.0, -220.0, 220.0, 370.0].iter().enumerate() {
        let boss = centered_cylinder(format!("tote_clamp_boss_{i}"), 22.0, 32.0, 32).translate(
            *x,
            DIRTY_SIDE_Y - TOTE_RECEIVER_Y / 2.0 - 38.0,
            DECK_Z + 76.0,
        );
        let hole = centered_cylinder(format!("tote_clamp_m8_clearance_{i}"), 4.5, 36.0, 24)
            .translate(
                *x,
                DIRTY_SIDE_Y - TOTE_RECEIVER_Y / 2.0 - 38.0,
                DECK_Z + 76.0,
            );
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn interlock_sensor_land_bar() -> Part {
    let bar = centered_cube("transfer_hatch_interlock_sensor_bar", 760.0, 28.0, 28.0).translate(
        0.0,
        DIRTY_SIDE_Y + TOTE_RECEIVER_Y / 2.0 + 58.0,
        DECK_Z + 96.0,
    );

    let mut lands = Part::empty("transfer_hatch_tote_present_lands");
    for (i, x) in [-300.0, -100.0, 100.0, 300.0].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("tote_present_latch_sensor_land_{i}"),
                58.0,
                8.0,
                20.0,
            )
            .translate(
                *x,
                DIRTY_SIDE_Y + TOTE_RECEIVER_Y / 2.0 + 75.0,
                DECK_Z + 96.0,
            );
    }

    bar + lands
}

fn outer_bag_removal_lane() -> Part {
    let lane = centered_cube(
        "outer_bag_removal_lane_recessed_table",
        OUTER_BAG_LANE_X,
        OUTER_BAG_LANE_Y,
        OUTER_BAG_LANE_Z,
    )
    .translate(
        -220.0,
        TRANSFER_CENTER_Y - 36.0,
        DECK_Z + OUTER_BAG_LANE_Z / 2.0,
    );
    let groove = centered_cube(
        "outer_bag_opening_groove",
        OUTER_BAG_LANE_X - 86.0,
        24.0,
        OUTER_BAG_LANE_Z + 4.0,
    )
    .translate(
        -220.0,
        TRANSFER_CENTER_Y - 36.0,
        DECK_Z + OUTER_BAG_LANE_Z / 2.0 + 2.0,
    );
    let shield = centered_cube(
        "guarded_outer_bag_cut_line_shield",
        OUTER_BAG_LANE_X - 120.0,
        20.0,
        72.0,
    )
    .translate(
        -220.0,
        TRANSFER_CENTER_Y - 36.0,
        DECK_Z + OUTER_BAG_LANE_Z + 36.0,
    );

    lane - groove
        + shield
        + outer_bag_clamp_stations()
        + peel_roller_placeholders()
        + vacuum_edge_slot_bar()
}

fn outer_bag_clamp_stations() -> Part {
    let mut clamps = Part::empty("outer_bag_clamp_stations");
    for i in 0..CLAMP_STATIONS {
        let x = -220.0 - OUTER_BAG_LANE_X / 2.0 + 74.0 + i as f64 * 100.0;
        let clamp = centered_cube(format!("outer_bag_low_profile_clamp_{i}"), 58.0, 28.0, 24.0)
            .translate(
                x,
                TRANSFER_CENTER_Y - OUTER_BAG_LANE_Y / 2.0 - 28.0,
                DECK_Z + 58.0,
            );
        let latch = centered_cube(format!("outer_bag_clamp_latch_land_{i}"), 34.0, 14.0, 12.0)
            .translate(
                x,
                TRANSFER_CENTER_Y + OUTER_BAG_LANE_Y / 2.0 - 44.0,
                DECK_Z + 52.0,
            );
        clamps = clamps + clamp + latch;
    }
    clamps
}

fn peel_roller_placeholders() -> Part {
    let mut rollers = Part::empty("outer_bag_peel_roller_placeholders");
    for i in 0..PEEL_ROLLERS {
        let x = -220.0 - 225.0 + i as f64 * 150.0;
        let roller = centered_cylinder(format!("outer_bag_peel_roller_{i}"), 15.0, 104.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x,
                TRANSFER_CENTER_Y + OUTER_BAG_LANE_Y / 2.0 + 18.0,
                DECK_Z + 78.0,
            );
        let shaft = centered_cylinder(format!("outer_bag_peel_roller_shaft_{i}"), 4.0, 118.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x,
                TRANSFER_CENTER_Y + OUTER_BAG_LANE_Y / 2.0 + 18.0,
                DECK_Z + 78.0,
            );
        rollers = rollers + (roller - shaft);
    }
    rollers
}

fn vacuum_edge_slot_bar() -> Part {
    let bar = centered_cube(
        "outer_bag_static_capture_vacuum_slot_bar",
        OUTER_BAG_LANE_X - 62.0,
        34.0,
        24.0,
    )
    .translate(
        -220.0,
        TRANSFER_CENTER_Y - OUTER_BAG_LANE_Y / 2.0 + 30.0,
        DECK_Z + 58.0,
    );
    let mut cuts = Part::empty("outer_bag_vacuum_slot_cuts");
    for i in 0..12 {
        let x = -220.0 - 250.0 + i as f64 * 45.0;
        cuts = cuts
            + centered_cube(format!("outer_bag_vacuum_slot_{i}"), 24.0, 38.0, 8.0).translate(
                x,
                TRANSFER_CENTER_Y - OUTER_BAG_LANE_Y / 2.0 + 30.0,
                DECK_Z + 58.0,
            );
    }

    bar - cuts
}

fn sterile_inner_pouch_staging() -> Part {
    let mut nests = Part::empty("sterile_inner_pouch_staging_nests");
    for row in 0..INNER_POUCH_ROWS {
        for col in 0..INNER_POUCH_COLS {
            let i = row * INNER_POUCH_COLS + col;
            let x = -360.0 + col as f64 * 210.0;
            let y = CLEAN_SIDE_Y - 72.0 + row as f64 * 150.0;
            let nest = pouch_nest(i, x, y);
            nests = nests + nest;
        }
    }
    nests + inner_pouch_datum_pin_row()
}

fn pouch_nest(index: usize, x: f64, y: f64) -> Part {
    let tray = centered_cube(
        format!("inner_pouch_staging_nest_{index}"),
        INNER_POUCH_NEST_X,
        INNER_POUCH_NEST_Y,
        INNER_POUCH_NEST_Z,
    )
    .translate(x, y, DECK_Z + INNER_POUCH_NEST_Z / 2.0);
    let pocket = centered_cube(
        format!("inner_pouch_shallow_recess_{index}"),
        INNER_POUCH_NEST_X - 32.0,
        INNER_POUCH_NEST_Y - 26.0,
        INNER_POUCH_NEST_Z + 3.0,
    )
    .translate(x, y, DECK_Z + INNER_POUCH_NEST_Z / 2.0 + 6.0);
    let left_rail = centered_cube(
        format!("inner_pouch_left_retain_rail_{index}"),
        10.0,
        INNER_POUCH_NEST_Y,
        22.0,
    )
    .translate(
        x - INNER_POUCH_NEST_X / 2.0 + 11.0,
        y,
        DECK_Z + INNER_POUCH_NEST_Z + 11.0,
    );
    let right_rail = centered_cube(
        format!("inner_pouch_right_retain_rail_{index}"),
        10.0,
        INNER_POUCH_NEST_Y,
        22.0,
    )
    .translate(
        x + INNER_POUCH_NEST_X / 2.0 - 11.0,
        y,
        DECK_Z + INNER_POUCH_NEST_Z + 11.0,
    );
    let vacuum_land = centered_cube(
        format!("inner_pouch_pick_vacuum_land_{index}"),
        52.0,
        30.0,
        5.0,
    )
    .translate(
        x,
        y - INNER_POUCH_NEST_Y / 2.0 + 24.0,
        DECK_Z + INNER_POUCH_NEST_Z + 2.5,
    );

    tray - pocket + left_rail + right_rail + vacuum_land
}

fn inner_pouch_datum_pin_row() -> Part {
    let mut pins = Part::empty("inner_pouch_staging_datum_pins");
    for (i, x) in [-456.0, -248.0, -40.0, 168.0, 376.0].iter().enumerate() {
        pins = pins
            + centered_cylinder(format!("inner_pouch_robot_datum_pin_{i}"), 8.0, 22.0, 28)
                .translate(*x, CLEAN_SIDE_Y - 168.0, DECK_Z + 33.0);
    }
    pins
}

fn released_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("released_hold_reject_lanes");
    for lane in 0..RELEASE_LANES {
        let x = 470.0;
        let y = CLEAN_SIDE_Y - 170.0 + lane as f64 * 138.0;
        let lane_frame = centered_cube(
            format!("material_status_lane_frame_{lane}"),
            STATUS_LANE_X + 36.0,
            STATUS_LANE_Y + 24.0,
            STATUS_LANE_Z,
        )
        .translate(x, y, DECK_Z + STATUS_LANE_Z / 2.0);
        let mut slot_cuts = Part::empty(format!("material_status_lane_slot_cuts_{lane}"));
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let sy = y - 42.0 + slot as f64 * 28.0;
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("material_status_slot_cut_{lane}_{slot}"),
                    STATUS_LANE_X - 24.0,
                    15.0,
                    STATUS_LANE_Z + 4.0,
                )
                .translate(x, sy, DECK_Z + STATUS_LANE_Z / 2.0 + 4.0);
        }
        lanes = lanes + (lane_frame - slot_cuts);
    }

    let released_header = centered_cube("released_material_lane_header", 142.0, 18.0, 16.0)
        .translate(470.0, CLEAN_SIDE_Y - 240.0, DECK_Z + 56.0);
    let hold_header = centered_cube("hold_material_lane_header", 142.0, 18.0, 16.0).translate(
        470.0,
        CLEAN_SIDE_Y - 102.0,
        DECK_Z + 56.0,
    );
    let reject_header = centered_cube("reject_material_lane_header", 142.0, 18.0, 16.0).translate(
        470.0,
        CLEAN_SIDE_Y + 36.0,
        DECK_Z + 56.0,
    );

    lanes + released_header + hold_header + reject_header
}

fn barcode_rfid_scan_lands() -> Part {
    let mut lands = Part::empty("barcode_rfid_scan_lands");
    for i in 0..BARCODE_LANDS {
        let x = -660.0 + (i % 7) as f64 * 104.0;
        let y = if i < 7 { -424.0 } else { 422.0 };
        lands = lands
            + centered_cube(
                format!("barcode_lot_scan_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(x, y, DECK_Z + LABEL_LAND_Z / 2.0 + 2.0);
    }

    for i in 0..RFID_LANDS {
        let x = 218.0 + (i % 4) as f64 * 112.0;
        let y = -206.0 + (i / 4) as f64 * 96.0;
        lands = lands
            + centered_cube(
                format!("rfid_inlay_scan_land_{i}"),
                62.0,
                42.0,
                LABEL_LAND_Z,
            )
            .translate(x, y, DECK_Z + LABEL_LAND_Z / 2.0 + 2.0);
    }

    for i in 0..CUSTODY_CARD_SLOTS {
        lands = lands
            + centered_cube(
                format!("material_chain_of_custody_card_slot_{i}"),
                82.0,
                10.0,
                34.0,
            )
            .translate(322.0 + i as f64 * 62.0, DIRTY_SIDE_Y + 116.0, DECK_Z + 34.0);
    }

    lands
}

fn wipe_contact_coupon_pockets() -> Part {
    let base = centered_cube(
        "wipe_contact_coupon_base_block",
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
        COUPON_BLOCK_Z,
    )
    .translate(-594.0, CLEAN_SIDE_Y + 34.0, DECK_Z + COUPON_BLOCK_Z / 2.0);
    let mut cuts = Part::empty("wipe_contact_coupon_recess_cuts");

    for i in 0..WIPE_COUPONS {
        let x = -716.0 + (i % 4) as f64 * 82.0;
        let y = CLEAN_SIDE_Y - 12.0 + (i / 4) as f64 * 42.0;
        cuts = cuts
            + centered_cube(
                format!("wipe_coupon_recess_{i}"),
                54.0,
                24.0,
                COUPON_BLOCK_Z + 5.0,
            )
            .translate(x, y, DECK_Z + COUPON_BLOCK_Z / 2.0 + 4.0);
    }

    for i in 0..CONTACT_COUPONS {
        let x = -724.0 + (i % 4) as f64 * 76.0;
        let y = CLEAN_SIDE_Y + 108.0 + (i / 4) as f64 * 38.0;
        cuts = cuts
            + centered_cylinder(
                format!("contact_plate_coupon_recess_{i}"),
                17.0,
                COUPON_BLOCK_Z + 5.0,
                28,
            )
            .translate(x, y, DECK_Z + COUPON_BLOCK_Z / 2.0 + 4.0);
    }

    base - cuts
}

fn particle_debris_capture_tray() -> Part {
    let tray = centered_cube(
        "particle_debris_capture_removable_tray",
        DEBRIS_TRAY_X,
        DEBRIS_TRAY_Y,
        DEBRIS_TRAY_Z,
    )
    .translate(-566.0, DIRTY_SIDE_Y + 122.0, DECK_Z + DEBRIS_TRAY_Z / 2.0);
    let basin_cut = centered_cube(
        "particle_debris_capture_basin_cut",
        DEBRIS_TRAY_X - 44.0,
        DEBRIS_TRAY_Y - 44.0,
        DEBRIS_TRAY_Z + 4.0,
    )
    .translate(
        -566.0,
        DIRTY_SIDE_Y + 122.0,
        DECK_Z + DEBRIS_TRAY_Z / 2.0 + 7.0,
    );
    let baffle = centered_cube(
        "particle_debris_capture_baffle",
        DEBRIS_TRAY_X - 54.0,
        12.0,
        44.0,
    )
    .translate(-566.0, DIRTY_SIDE_Y + 122.0, DECK_Z + DEBRIS_TRAY_Z + 18.0);

    tray - basin_cut + baffle + particle_sensor_wells()
}

fn particle_sensor_wells() -> Part {
    let mut wells = Part::empty("particle_debris_sensor_wells");
    for i in 0..PARTICLE_SENSOR_WELLS {
        wells = wells
            + centered_cylinder(format!("particle_sensor_well_{i}"), 18.0, 18.0, 28).translate(
                -704.0 + i as f64 * 92.0,
                DIRTY_SIDE_Y + 46.0,
                DECK_Z + 46.0,
            );
    }
    wells
}

fn clean_dirty_lane_divider() -> Part {
    let wall = centered_cube(
        "clean_dirty_physical_divider_wall",
        DIVIDER_X,
        DIVIDER_Y,
        DIVIDER_Z,
    )
    .translate(0.0, 118.0, DECK_Z + DIVIDER_Z / 2.0);
    let pass_slot = centered_cube(
        "clean_dirty_divider_pass_slot",
        PASS_SLOT_X,
        DIVIDER_Y + 6.0,
        PASS_SLOT_Z,
    )
    .translate(-125.0, 118.0, DECK_Z + PASS_SLOT_Z / 2.0 + 30.0);
    let gasket = rectangular_frame(
        "divider_pass_slot_gasket_land",
        PASS_SLOT_X + 70.0,
        8.0,
        PASS_SLOT_Z + 54.0,
        PASS_SLOT_X,
        PASS_SLOT_Z,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-125.0, 102.0, DECK_Z + 88.0);

    wall - pass_slot + gasket + divider_air_knife_placeholder() + divider_status_lights()
}

fn divider_air_knife_placeholder() -> Part {
    let upper = centered_cube(
        "divider_clean_sweep_airknife_placeholder",
        PASS_SLOT_X + 96.0,
        20.0,
        18.0,
    )
    .translate(-125.0, 134.0, DECK_Z + 142.0);
    let lower = centered_cube(
        "divider_low_return_slot_placeholder",
        PASS_SLOT_X + 50.0,
        16.0,
        14.0,
    )
    .translate(-125.0, 88.0, DECK_Z + 40.0);

    upper + lower
}

fn divider_status_lights() -> Part {
    let mut lights = Part::empty("divider_interlock_status_lights");
    for (i, x) in [430.0, 500.0, 570.0].iter().enumerate() {
        lights = lights
            + centered_cylinder(format!("divider_status_light_{i}"), 14.0, 8.0, 28)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 102.0, DECK_Z + 118.0);
    }
    lights
}

fn passthrough_tongue_rail_interface() -> Part {
    let tongue = centered_cube(
        "passthrough_transfer_tongue_plate",
        TONGUE_X,
        TONGUE_Y,
        TONGUE_Z,
    )
    .translate(-66.0, PASS_THROUGH_Y, DECK_Z + TONGUE_Z / 2.0);
    let leading_chamfer_land = centered_cube(
        "passthrough_tongue_leading_chamfer_land",
        TONGUE_X - 110.0,
        36.0,
        18.0,
    )
    .translate(
        -66.0,
        PASS_THROUGH_Y + TONGUE_Y / 2.0 - 20.0,
        DECK_Z + TONGUE_Z + 9.0,
    );
    let left_rail = centered_cube(
        "passthrough_left_keyed_rail",
        24.0,
        TONGUE_Y + 70.0,
        TONGUE_RAIL_Z,
    )
    .translate(
        -66.0 - TONGUE_RAIL_PITCH / 2.0,
        PASS_THROUGH_Y,
        DECK_Z + TONGUE_Z + TONGUE_RAIL_Z / 2.0,
    );
    let right_rail = centered_cube(
        "passthrough_right_keyed_rail",
        24.0,
        TONGUE_Y + 70.0,
        TONGUE_RAIL_Z,
    )
    .translate(
        -66.0 + TONGUE_RAIL_PITCH / 2.0,
        PASS_THROUGH_Y,
        DECK_Z + TONGUE_Z + TONGUE_RAIL_Z / 2.0,
    );
    let crossbar = centered_cube(
        "passthrough_module_stop_crossbar",
        TONGUE_X - 46.0,
        24.0,
        58.0,
    )
    .translate(-66.0, PASS_THROUGH_Y - TONGUE_Y / 2.0 - 38.0, DECK_Z + 64.0);

    tongue + leading_chamfer_land + left_rail + right_rail + crossbar + tongue_datum_pins()
}

fn tongue_datum_pins() -> Part {
    let mut pins = Part::empty("passthrough_tongue_datum_pins");
    for i in 0..TONGUE_DATUM_PINS {
        let x = -66.0 - 270.0 + (i % 3) as f64 * 270.0;
        let y = PASS_THROUGH_Y - 126.0 + (i / 3) as f64 * 186.0;
        pins =
            pins + centered_cylinder(format!("passthrough_datum_pin_{i}"), 7.0, 26.0, 28)
                .translate(x, y, DECK_Z + TONGUE_Z + 13.0);
    }
    pins
}

fn evidence_photo_inspection_bridge() -> Part {
    let left_post = centered_cube(
        "inspection_bridge_left_post",
        36.0,
        70.0,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_SPAN_X / 2.0,
        TRANSFER_CENTER_Y + 12.0,
        DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "inspection_bridge_right_post",
        36.0,
        70.0,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        CAMERA_BRIDGE_SPAN_X / 2.0,
        TRANSFER_CENTER_Y + 12.0,
        DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "inspection_bridge_camera_beam",
        CAMERA_BRIDGE_SPAN_X + 72.0,
        220.0,
        32.0,
    )
    .translate(
        0.0,
        TRANSFER_CENTER_Y + 12.0,
        DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 16.0,
    );
    let neutral_backdrop = centered_cube(
        "photo_evidence_neutral_backdrop_plate",
        CAMERA_BRIDGE_SPAN_X - 110.0,
        12.0,
        118.0,
    )
    .translate(0.0, TRANSFER_CENTER_Y + 116.0, DECK_Z + 128.0);

    left_post
        + right_post
        + beam
        + neutral_backdrop
        + camera_pods()
        + led_light_bars()
        + evidence_bridge_focus_targets()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("inspection_bridge_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = -390.0 + i as f64 * 260.0;
        let pod = centered_cube(format!("evidence_photo_camera_pod_{i}"), 74.0, 58.0, 42.0)
            .translate(
                x,
                TRANSFER_CENTER_Y - 24.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 54.0,
            );
        let lens = centered_cylinder(format!("evidence_photo_lens_clearance_{i}"), 16.0, 10.0, 28)
            .translate(
                x,
                TRANSFER_CENTER_Y - 24.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 28.0,
            );
        let focus_ring = centered_cylinder(format!("evidence_photo_focus_ring_{i}"), 22.0, 5.0, 32)
            .translate(
                x,
                TRANSFER_CENTER_Y - 24.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 16.0,
            );
        pods = pods + pod + lens + focus_ring;
    }
    pods
}

fn led_light_bars() -> Part {
    let mut bars = Part::empty("inspection_bridge_led_bars");
    for i in 0..LED_BARS {
        let x = -390.0 + i as f64 * 260.0;
        bars = bars
            + centered_cube(
                format!("evidence_photo_diffuse_led_bar_{i}"),
                170.0,
                12.0,
                16.0,
            )
            .translate(
                x,
                TRANSFER_CENTER_Y + 74.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 18.0,
            );
    }
    bars
}

fn evidence_bridge_focus_targets() -> Part {
    let mut targets = Part::empty("inspection_bridge_focus_targets");
    for (i, x) in [-480.0, -160.0, 160.0, 480.0].iter().enumerate() {
        targets = targets
            + centered_cylinder(
                format!("inspection_bridge_focus_target_disc_{i}"),
                18.0,
                12.0,
                32,
            )
            .translate(
                *x,
                TRANSFER_CENTER_Y + 102.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 44.0,
            )
            + centered_cube(
                format!("inspection_bridge_focus_target_crosshair_{i}"),
                42.0,
                6.0,
                6.0,
            )
            .translate(
                *x,
                TRANSFER_CENTER_Y + 102.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 54.0,
            )
            + centered_cube(
                format!("inspection_bridge_focus_target_crosshair_perp_{i}"),
                6.0,
                42.0,
                6.0,
            )
            .translate(
                *x,
                TRANSFER_CENTER_Y + 102.0,
                DECK_Z + CAMERA_BRIDGE_UNDERSIDE_Z + 54.0,
            );
    }
    targets
}

fn glove_robot_reach_keepouts() -> Part {
    let front = centered_cube(
        "front_glove_robot_reach_keepout",
        DECK_X - 220.0,
        FRONT_GLOVE_KEEP_OUT,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_GLOVE_KEEP_OUT / 2.0,
        DECK_Z + 4.0,
    );
    let rear = centered_cube(
        "rear_hatch_service_keepout",
        DECK_X - 320.0,
        REAR_HATCH_KEEP_OUT,
        8.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_HATCH_KEEP_OUT / 2.0, DECK_Z + 4.0);
    let left = centered_cube(
        "left_tote_swing_keepout",
        SIDE_TOTE_KEEP_OUT,
        DECK_Y - 140.0,
        8.0,
    )
    .translate(-DECK_X / 2.0 - SIDE_TOTE_KEEP_OUT / 2.0, 0.0, DECK_Z + 4.0);
    let right = centered_cube(
        "right_tote_swing_keepout",
        SIDE_TOTE_KEEP_OUT,
        DECK_Y - 140.0,
        8.0,
    )
    .translate(DECK_X / 2.0 + SIDE_TOTE_KEEP_OUT / 2.0, 0.0, DECK_Z + 4.0);
    let top = centered_cube(
        "top_camera_service_keepout_gauge",
        CAMERA_BRIDGE_SPAN_X,
        60.0,
        8.0,
    )
    .translate(0.0, TRANSFER_CENTER_Y + 12.0, TOP_CAMERA_SERVICE_Z);

    front + rear + left + right + top
}

fn waste_offcut_trap() -> Part {
    let trap = centered_cube(
        "outer_bag_waste_offcut_trap_box",
        WASTE_TRAP_X,
        WASTE_TRAP_Y,
        WASTE_TRAP_Z,
    )
    .translate(566.0, DIRTY_SIDE_Y + 54.0, DECK_Z + WASTE_TRAP_Z / 2.0);
    let cavity = centered_cube(
        "outer_bag_waste_offcut_cavity",
        WASTE_TRAP_X - 46.0,
        WASTE_TRAP_Y - 50.0,
        WASTE_TRAP_Z + 4.0,
    )
    .translate(
        566.0,
        DIRTY_SIDE_Y + 54.0,
        DECK_Z + WASTE_TRAP_Z / 2.0 + 14.0,
    );
    let chute = centered_cube(
        "outer_bag_waste_offcut_chute",
        WASTE_TRAP_X - 82.0,
        46.0,
        62.0,
    )
    .translate(
        566.0,
        DIRTY_SIDE_Y - WASTE_TRAP_Y / 2.0 - 18.0,
        DECK_Z + 66.0,
    );

    trap - cavity + chute + offcut_slot_comb() + reject_bag_ring()
}

fn offcut_slot_comb() -> Part {
    let mut comb = Part::empty("outer_bag_offcut_slot_comb");
    for i in 0..OFFCUT_SLOTS {
        let x = 566.0 - 92.0 + i as f64 * 46.0;
        comb =
            comb + centered_cube(format!("offcut_film_retainer_slot_{i}"), 12.0, 74.0, 42.0)
                .translate(x, DIRTY_SIDE_Y + 50.0, DECK_Z + 92.0);
    }
    comb
}

fn reject_bag_ring() -> Part {
    rectangular_frame(
        "reject_outer_bag_ring_lip",
        WASTE_TRAP_X - 34.0,
        10.0,
        118.0,
        WASTE_TRAP_X - 92.0,
        62.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        566.0,
        DIRTY_SIDE_Y + WASTE_TRAP_Y / 2.0 + 10.0,
        DECK_Z + 84.0,
    )
}

fn rectangular_frame(
    name: impl Into<String>,
    outer_x: f64,
    y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let top = centered_cube(format!("{name}_top"), outer_x, y, (outer_z - inner_z) / 2.0)
        .translate(0.0, 0.0, inner_z / 2.0 + (outer_z - inner_z) / 4.0);
    let bottom = centered_cube(
        format!("{name}_bottom"),
        outer_x,
        y,
        (outer_z - inner_z) / 2.0,
    )
    .translate(0.0, 0.0, -inner_z / 2.0 - (outer_z - inner_z) / 4.0);
    let left = centered_cube(
        format!("{name}_left"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(-inner_x / 2.0 - (outer_x - inner_x) / 4.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{name}_right"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(inner_x / 2.0 + (outer_x - inner_x) / 4.0, 0.0, 0.0);

    top + bottom + left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_complete_station_package() {
        assert_eq!(OUTPUTS.len(), 14);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/closed_material_passthrough_debagging_station_")));
    }

    #[test]
    fn footprint_is_bench_module_sized() {
        assert!(DECK_X <= 1600.0);
        assert!(DECK_Y <= 1000.0);
        assert!(DECK_X > TOTE_RECEIVER_X);
        assert!(DECK_Y > TONGUE_Y);
    }

    #[test]
    fn receiver_and_tongue_accept_closed_transfer_payloads() {
        assert!(TOTE_CLEAR_X >= 700.0);
        assert!(TOTE_CLEAR_Y >= 120.0);
        assert!(TONGUE_X >= TOTE_CLEAR_X);
        assert!(TONGUE_RAIL_PITCH < TONGUE_X);
    }

    #[test]
    fn clean_dirty_controls_are_explicit() {
        assert!(DIVIDER_Z >= 120.0);
        assert!(PASS_SLOT_X >= 500.0);
        assert!(PARTICLE_SENSOR_WELLS >= 4);
        assert!(WIPE_COUPONS >= 10);
        assert!(CONTACT_COUPONS >= 8);
    }

    #[test]
    fn material_status_and_traceability_capacity_is_plausible() {
        assert_eq!(RELEASE_LANES * STATUS_SLOTS_PER_LANE, 12);
        assert!(INNER_POUCH_POSITIONS >= 8);
        assert!(BARCODE_LANDS >= INNER_POUCH_POSITIONS);
        assert!(RFID_LANDS >= 8);
        assert!(CUSTODY_CARD_SLOTS >= 6);
    }

    #[test]
    fn evidence_and_access_clearances_are_robot_friendly() {
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z >= 250.0);
        assert!(FRONT_GLOVE_KEEP_OUT >= 360.0);
        assert!(REAR_HATCH_KEEP_OUT >= 300.0);
        assert!(SIDE_TOTE_KEEP_OUT >= 240.0);
        assert!(TOP_CAMERA_SERVICE_Z > CAMERA_BRIDGE_UNDERSIDE_Z);
    }
}
