use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste quarantine / sample retain split verification station.
//
// Design intent:
// - Package a closed-system validation fixture for contaminated, suspect, or
//   audit-retained waste/sample paths in tissue-on-chip automation.
// - Make split routing, sample retention, quarantine docking, custody labels,
//   release/hold/reject state, leak/backflow evidence, filtered venting,
//   camera evidence, robot/service envelopes, and wipe witness coupons visible
//   in the mechanical layout without opening the sterile boundary.
// - This is product-concept packaging and verification CAD only. It is not a
//   biological acceptance criterion, waste-disposal SOP, or contamination
//   release decision workflow.

const OUTPUTS: [&str; 13] = [
    "output/closed_waste_quarantine_sample_retain_split_verification_station_secondary_containment_deck.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_sealed_waste_inlet_bulkhead.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_sample_retain_vial_nest.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_split_manifold_surrogate.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_capped_retain_carousel.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_quarantine_bag_bottle_dock.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_leak_moat_backflow_witness_channel.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_filtered_vent_holder.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_barcode_rfid_custody_status_gates.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_evidence_camera_bridge.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_robot_service_keepouts.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_cleaning_wipe_witness_coupons.stl",
    "output/closed_waste_quarantine_sample_retain_split_verification_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "secondary_containment_deck",
    "sealed_waste_inlet_bulkhead",
    "sample_retain_vial_nest",
    "split_manifold_surrogate",
    "capped_retain_carousel",
    "quarantine_bag_bottle_dock",
    "leak_moat_backflow_witness_channel",
    "filtered_vent_holder",
    "barcode_rfid_custody_status_gates",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "cleaning_wipe_witness_coupons",
];

const STATUS_NAMES: [&str; 3] = ["release", "hold", "reject"];
const CUSTODY_ZONE_NAMES: [&str; 4] = ["incoming", "split", "retain", "quarantine"];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const MOAT_W: f64 = 18.0;
const MOAT_DEPTH: f64 = 9.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 10.0;

const BULKHEAD_POS: (f64, f64) = (-450.0, 260.0);
const BULKHEAD_X: f64 = 410.0;
const BULKHEAD_Y: f64 = 110.0;
const BULKHEAD_Z: f64 = 96.0;
const WASTE_INLET_PORTS: usize = 6;
const INLET_PORT_D: f64 = 12.0;
const INLET_PITCH: f64 = 58.0;
const CAP_PARKS: usize = 6;

const VIAL_NEST_POS: (f64, f64) = (-408.0, -70.0);
const VIAL_NEST_X: f64 = 350.0;
const VIAL_NEST_Y: f64 = 250.0;
const VIAL_NEST_Z: f64 = 54.0;
const RETAIN_VIAL_ROWS: usize = 4;
const RETAIN_VIAL_COLS: usize = 6;
const RETAIN_VIALS: usize = RETAIN_VIAL_ROWS * RETAIN_VIAL_COLS;
const VIAL_PITCH_X: f64 = 46.0;
const VIAL_PITCH_Y: f64 = 44.0;
const VIAL_BORE_D: f64 = 17.8;

const MANIFOLD_POS: (f64, f64) = (-35.0, 185.0);
const MANIFOLD_X: f64 = 390.0;
const MANIFOLD_Y: f64 = 170.0;
const MANIFOLD_Z: f64 = 66.0;
const SPLIT_LANES: usize = 8;
const SPLIT_BRANCHES_PER_LANE: usize = 3;
const SPLIT_OUTPUT_PORTS: usize = SPLIT_LANES * SPLIT_BRANCHES_PER_LANE;
const SPLIT_LANE_PITCH: f64 = 40.0;
const SPLIT_PORT_D: f64 = 7.0;

const CAROUSEL_POS: (f64, f64) = (305.0, -175.0);
const CAROUSEL_D: f64 = 310.0;
const CAROUSEL_Z: f64 = 44.0;
const CAROUSEL_RETAIN_POSITIONS: usize = 18;
const CAROUSEL_PITCH_RADIUS: f64 = 118.0;
const CAROUSEL_CAP_CLIPS: usize = 18;

const QUARANTINE_POS: (f64, f64) = (415.0, 170.0);
const QUARANTINE_X: f64 = 360.0;
const QUARANTINE_Y: f64 = 260.0;
const QUARANTINE_Z: f64 = 58.0;
const QUARANTINE_BOTTLES: usize = 3;
const QUARANTINE_BAGS: usize = 2;
const BOTTLE_POCKET_D: f64 = 78.0;
const BAG_CLAMP_COUNT: usize = 4;

const WITNESS_POS: (f64, f64) = (-40.0, -235.0);
const WITNESS_X: f64 = 520.0;
const WITNESS_Y: f64 = 128.0;
const WITNESS_Z: f64 = 34.0;
const BACKFLOW_CHANNELS: usize = 8;
const WITNESS_WINDOWS: usize = 6;

const VENT_POS: (f64, f64) = (502.0, -120.0);
const VENT_X: f64 = 190.0;
const VENT_Y: f64 = 128.0;
const VENT_Z: f64 = 72.0;
const FILTERED_VENTS: usize = 4;
const VENT_CARTRIDGE_D: f64 = 28.0;

const CUSTODY_POS: (f64, f64) = (-50.0, 360.0);
const CUSTODY_X: f64 = 690.0;
const CUSTODY_Y: f64 = 96.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 8;
const STATUS_GATE_COUNT: usize = 3;
const STATUS_TOKEN_POCKETS: usize = STATUS_GATE_COUNT * 4;

const BRIDGE_POS: (f64, f64) = (10.0, 20.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 44.0;
const BRIDGE_UNDERSIDE_Z: f64 = 230.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_COUNT: usize = 4;
const LED_STRIP_COUNT: usize = 8;

const KEEP_OUT_Z: f64 = 92.0;
const FRONT_ROBOT_CLEARANCE: f64 = 350.0;
const REAR_SERVICE_CLEARANCE: f64 = 230.0;
const LEFT_BULKHEAD_SERVICE_CLEARANCE: f64 = 190.0;
const RIGHT_QUARANTINE_SERVICE_CLEARANCE: f64 = 245.0;
const KEEP_OUT_ZONES: usize = 4;

const COUPON_POS: (f64, f64) = (-380.0, -335.0);
const COUPON_PANEL_X: f64 = 370.0;
const COUPON_PANEL_Y: f64 = 92.0;
const COUPON_PANEL_Z: f64 = 18.0;
const WIPE_COUPONS: usize = 10;
const COUPON_PITCH: f64 = 34.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let bulkhead = sealed_waste_inlet_bulkhead();
    export(OUTPUTS[1], &bulkhead);

    let vial_nest = sample_retain_vial_nest();
    export(OUTPUTS[2], &vial_nest);

    let manifold = split_manifold_surrogate();
    export(OUTPUTS[3], &manifold);

    let carousel = capped_retain_carousel();
    export(OUTPUTS[4], &carousel);

    let quarantine = quarantine_bag_bottle_dock();
    export(OUTPUTS[5], &quarantine);

    let witness = leak_moat_backflow_witness_channel();
    export(OUTPUTS[6], &witness);

    let vent = filtered_vent_holder();
    export(OUTPUTS[7], &vent);

    let custody = barcode_rfid_custody_status_gates();
    export(OUTPUTS[8], &custody);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let coupons = cleaning_wipe_witness_coupons();
    export(OUTPUTS[11], &coupons);

    let assembly = deck
        + bulkhead.translate(BULKHEAD_POS.0, BULKHEAD_POS.1, deck_z(BULKHEAD_Z))
        + vial_nest.translate(VIAL_NEST_POS.0, VIAL_NEST_POS.1, deck_z(VIAL_NEST_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, deck_z(MANIFOLD_Z))
        + carousel.translate(CAROUSEL_POS.0, CAROUSEL_POS.1, deck_z(CAROUSEL_Z))
        + quarantine.translate(QUARANTINE_POS.0, QUARANTINE_POS.1, deck_z(QUARANTINE_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, deck_z(WITNESS_Z))
        + vent.translate(VENT_POS.0, VENT_POS.1, deck_z(VENT_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, deck_z(CUSTODY_Z))
        + bridge.translate(
            BRIDGE_POS.0,
            BRIDGE_POS.1,
            deck_z(BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z),
        )
        + keepouts.translate(0.0, 0.0, deck_z(KEEP_OUT_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, deck_z(COUPON_PANEL_Z));
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed waste quarantine / sample retain split verification station:");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm secondary containment deck");
    println!(
        "  Closed waste inlet bulkhead: {WASTE_INLET_PORTS} sealed inlet ports with {CAP_PARKS} cap parks"
    );
    println!(
        "  Split/retain:                {SPLIT_LANES} split lanes, {SPLIT_OUTPUT_PORTS} output witness ports, {RETAIN_VIALS} retain vial nest pockets, {CAROUSEL_RETAIN_POSITIONS} capped carousel positions"
    );
    println!(
        "  Quarantine dock:             {QUARANTINE_BOTTLES} bottle pockets, {QUARANTINE_BAGS} bag lanes, {BAG_CLAMP_COUNT} clamp bridges"
    );
    println!(
        "  Evidence/audit:              {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {STATUS_TOKEN_POCKETS} status token pockets, {CAMERA_COUNT} evidence cameras"
    );
    println!(
        "  Leak/backflow/wipe witness:  {BACKFLOW_CHANNELS} backflow channels, {WITNESS_WINDOWS} witness windows, {WIPE_COUPONS} removable wipe coupons, {FILTERED_VENTS} filtered vents"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_z(part_height: f64) -> f64 {
    BASE_Z / 2.0 + part_height / 2.0 + 4.0
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        "closed_waste_quarantine_station_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let inner_basin = centered_cube(
        "closed_waste_quarantine_station_secondary_containment_basin",
        STATION_X - 96.0,
        STATION_Y - 104.0,
        MOAT_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - MOAT_DEPTH / 2.0 + 1.0);
    let front_moat = centered_cube(
        "closed_waste_quarantine_station_front_leak_moat",
        STATION_X - 120.0,
        MOAT_W,
        MOAT_DEPTH + 4.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 78.0, BASE_Z / 2.0 - 4.0);
    let rear_moat = centered_cube(
        "closed_waste_quarantine_station_rear_leak_moat",
        STATION_X - 120.0,
        MOAT_W,
        MOAT_DEPTH + 4.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 78.0, BASE_Z / 2.0 - 4.0);
    let left_moat = centered_cube(
        "closed_waste_quarantine_station_left_leak_moat",
        MOAT_W,
        STATION_Y - 120.0,
        MOAT_DEPTH + 4.0,
    )
    .translate(-STATION_X / 2.0 + 78.0, 0.0, BASE_Z / 2.0 - 4.0);
    let right_moat = centered_cube(
        "closed_waste_quarantine_station_right_leak_moat",
        MOAT_W,
        STATION_Y - 120.0,
        MOAT_DEPTH + 4.0,
    )
    .translate(STATION_X / 2.0 - 78.0, 0.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        "closed_waste_quarantine_station_secondary_containment_drain_port",
        DRAIN_PORT_D / 2.0,
        54.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 68.0, -STATION_Y / 2.0 + 54.0, 0.0);

    deck - inner_basin - front_moat - rear_moat - left_moat - right_moat - drain
        + containment_rims()
        + mounting_slots()
        + station_fiducials()
        + module_registration_lands()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "closed_waste_quarantine_station_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_waste_quarantine_station_rear_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_waste_quarantine_station_left_spill_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_waste_quarantine_station_right_spill_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_waste_quarantine_station_mounting_slots");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("closed_waste_quarantine_station_m6_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 8.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_waste_quarantine_station_m6_mount_slot_{i}"),
            30.0,
            MOUNT_HOLE_D + 0.8,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn mount_points() -> [(f64, f64); 10] {
    [
        (-(STATION_X / 2.0 - 64.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 64.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 64.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 64.0), 0.0),
        (STATION_X / 2.0 - 64.0, 0.0),
        (MANIFOLD_POS.0 - MANIFOLD_X / 2.0 + 52.0, MANIFOLD_POS.1),
        (
            QUARANTINE_POS.0 + QUARANTINE_X / 2.0 - 54.0,
            QUARANTINE_POS.1,
        ),
    ]
}

fn station_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_waste_quarantine_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 82.0), STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 82.0),
        (-(STATION_X / 2.0 - 82.0), -(STATION_Y / 2.0 - 82.0)),
        (STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 82.0)),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("closed_waste_quarantine_station_fiducial_outer_{i}"),
            15.0,
            4.0,
            36,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("closed_waste_quarantine_station_fiducial_center_{i}"),
                5.0,
                6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
        fiducials = fiducials + target;
    }
    fiducials
}

fn module_registration_lands() -> Part {
    let mut lands = Part::empty("closed_waste_quarantine_station_module_registration_lands");
    for (i, (x, y, sx, sy)) in [
        (
            BULKHEAD_POS.0,
            BULKHEAD_POS.1,
            BULKHEAD_X + 34.0,
            BULKHEAD_Y + 30.0,
        ),
        (
            VIAL_NEST_POS.0,
            VIAL_NEST_POS.1,
            VIAL_NEST_X + 32.0,
            VIAL_NEST_Y + 32.0,
        ),
        (
            MANIFOLD_POS.0,
            MANIFOLD_POS.1,
            MANIFOLD_X + 34.0,
            MANIFOLD_Y + 30.0,
        ),
        (
            QUARANTINE_POS.0,
            QUARANTINE_POS.1,
            QUARANTINE_X + 34.0,
            QUARANTINE_Y + 34.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("closed_waste_quarantine_station_registration_land_{i}"),
                *sx,
                *sy,
                6.0,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    lands
}

fn sealed_waste_inlet_bulkhead() -> Part {
    let body = centered_cube(
        "closed_waste_quarantine_station_sealed_waste_inlet_bulkhead_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let gasket_land = centered_cube(
        "closed_waste_quarantine_station_bulkhead_compression_gasket_land",
        BULKHEAD_X - 34.0,
        14.0,
        BULKHEAD_Z + 8.0,
    )
    .translate(0.0, -BULKHEAD_Y / 2.0 + 18.0, 0.0);
    let mut ports = Part::empty("closed_waste_quarantine_station_sealed_inlet_port_cutouts");
    for i in 0..WASTE_INLET_PORTS {
        let x = centered_index(i, WASTE_INLET_PORTS, INLET_PITCH);
        ports = ports
            + centered_cylinder(
                format!("closed_waste_quarantine_station_waste_inlet_port_{i}"),
                INLET_PORT_D / 2.0,
                BULKHEAD_Y + 10.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 12.0);
    }

    body - ports
        + gasket_land
        + cap_park_posts()
        + bulkhead_latch_tabs()
        + centered_cube(
            "closed_waste_quarantine_station_bulkhead_no_open_boundary_flag_land",
            BULKHEAD_X - 58.0,
            10.0,
            12.0,
        )
        .translate(0.0, BULKHEAD_Y / 2.0 - 16.0, BULKHEAD_Z / 2.0 + 6.0)
}

fn cap_park_posts() -> Part {
    let mut posts = Part::empty("closed_waste_quarantine_station_cap_park_posts");
    for i in 0..CAP_PARKS {
        let x = centered_index(i, CAP_PARKS, INLET_PITCH);
        let post = centered_cylinder(
            format!("closed_waste_quarantine_station_cap_park_post_{i}"),
            10.0,
            28.0,
            28,
        )
        .translate(x, BULKHEAD_Y / 2.0 + 18.0, 8.0);
        let tether_slot = centered_cube(
            format!("closed_waste_quarantine_station_cap_tether_slot_{i}"),
            4.0,
            20.0,
            8.0,
        )
        .translate(x, BULKHEAD_Y / 2.0 + 18.0, 8.0);
        posts = posts + (post - tether_slot);
    }
    posts
}

fn bulkhead_latch_tabs() -> Part {
    let left = centered_cube(
        "closed_waste_quarantine_station_bulkhead_left_latch_tab",
        34.0,
        28.0,
        18.0,
    )
    .translate(-BULKHEAD_X / 2.0 + 28.0, 0.0, BULKHEAD_Z / 2.0 + 9.0);
    let right = centered_cube(
        "closed_waste_quarantine_station_bulkhead_right_latch_tab",
        34.0,
        28.0,
        18.0,
    )
    .translate(BULKHEAD_X / 2.0 - 28.0, 0.0, BULKHEAD_Z / 2.0 + 9.0);
    left + right
}

fn sample_retain_vial_nest() -> Part {
    let nest = centered_cube(
        "closed_waste_quarantine_station_sample_retain_vial_nest_block",
        VIAL_NEST_X,
        VIAL_NEST_Y,
        VIAL_NEST_Z,
    );
    let mut bores = Part::empty("closed_waste_quarantine_station_retain_vial_bores");
    for row in 0..RETAIN_VIAL_ROWS {
        for col in 0..RETAIN_VIAL_COLS {
            let i = row * RETAIN_VIAL_COLS + col;
            let x = centered_index(col, RETAIN_VIAL_COLS, VIAL_PITCH_X);
            let y = centered_index(row, RETAIN_VIAL_ROWS, VIAL_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("closed_waste_quarantine_station_retain_vial_bore_{i}"),
                    VIAL_BORE_D / 2.0,
                    VIAL_NEST_Z + 8.0,
                    30,
                )
                .translate(x, y, 0.0);
        }
    }
    nest - bores + vial_nest_label_lands() + vial_nest_lift_handles()
}

fn vial_nest_label_lands() -> Part {
    let mut lands = Part::empty("closed_waste_quarantine_station_vial_nest_label_lands");
    for row in 0..RETAIN_VIAL_ROWS {
        let y = centered_index(row, RETAIN_VIAL_ROWS, VIAL_PITCH_Y);
        lands = lands
            + centered_cube(
                format!("closed_waste_quarantine_station_vial_row_label_land_{row}"),
                42.0,
                16.0,
                5.0,
            )
            .translate(-VIAL_NEST_X / 2.0 + 28.0, y, VIAL_NEST_Z / 2.0 + 2.5);
    }
    lands
}

fn vial_nest_lift_handles() -> Part {
    let front_handle = centered_cube(
        "closed_waste_quarantine_station_vial_nest_front_lift_handle",
        60.0,
        14.0,
        18.0,
    )
    .translate(0.0, -VIAL_NEST_Y / 2.0 - 10.0, 4.0);
    let rear_handle = centered_cube(
        "closed_waste_quarantine_station_vial_nest_rear_lift_handle",
        60.0,
        14.0,
        18.0,
    )
    .translate(0.0, VIAL_NEST_Y / 2.0 + 10.0, 4.0);
    front_handle + rear_handle
}

fn split_manifold_surrogate() -> Part {
    let body = centered_cube(
        "closed_waste_quarantine_station_split_manifold_surrogate_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let source_trunk = centered_cylinder(
        "closed_waste_quarantine_station_split_manifold_source_trunk",
        8.0,
        MANIFOLD_X + 24.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -44.0, 8.0);
    let mut branch_ports = Part::empty("closed_waste_quarantine_station_split_branch_ports");
    let mut valve_pads = Part::empty("closed_waste_quarantine_station_split_valve_pads");
    for lane in 0..SPLIT_LANES {
        let x = centered_index(lane, SPLIT_LANES, SPLIT_LANE_PITCH);
        let lane_bore = centered_cylinder(
            format!("closed_waste_quarantine_station_split_lane_bore_{lane}"),
            SPLIT_PORT_D / 2.0,
            MANIFOLD_Y + 16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 8.0);
        branch_ports = branch_ports + lane_bore;

        for branch in 0..SPLIT_BRANCHES_PER_LANE {
            let y = -28.0 + branch as f64 * 32.0;
            branch_ports = branch_ports
                + centered_cylinder(
                    format!("closed_waste_quarantine_station_split_output_lane_{lane}_{branch}"),
                    SPLIT_PORT_D / 2.0,
                    MANIFOLD_Z + 12.0,
                    24,
                )
                .translate(x, y, 0.0);
        }
        valve_pads = valve_pads
            + centered_cube(
                format!("closed_waste_quarantine_station_split_pinchoff_valve_pad_{lane}"),
                26.0,
                22.0,
                8.0,
            )
            .translate(x, MANIFOLD_Y / 2.0 - 24.0, MANIFOLD_Z / 2.0 + 4.0);
    }

    body - source_trunk - branch_ports
        + valve_pads
        + centered_cube(
            "closed_waste_quarantine_station_split_manifold_flow_arrow_land",
            MANIFOLD_X - 46.0,
            10.0,
            6.0,
        )
        .translate(0.0, -MANIFOLD_Y / 2.0 + 18.0, MANIFOLD_Z / 2.0 + 3.0)
}

fn capped_retain_carousel() -> Part {
    let disk = centered_cylinder(
        "closed_waste_quarantine_station_capped_retain_carousel_disk",
        CAROUSEL_D / 2.0,
        CAROUSEL_Z,
        96,
    );
    let hub = centered_cylinder(
        "closed_waste_quarantine_station_capped_retain_carousel_keyed_hub",
        34.0,
        CAROUSEL_Z + 14.0,
        48,
    );
    let mut wells = Part::empty("closed_waste_quarantine_station_carousel_retain_wells");
    let mut cap_clips = Part::empty("closed_waste_quarantine_station_carousel_cap_clips");
    for i in 0..CAROUSEL_RETAIN_POSITIONS {
        let (x, y) = polar_xy(i, CAROUSEL_RETAIN_POSITIONS, CAROUSEL_PITCH_RADIUS);
        wells = wells
            + centered_cylinder(
                format!("closed_waste_quarantine_station_carousel_retain_well_{i}"),
                10.2,
                CAROUSEL_Z + 8.0,
                28,
            )
            .translate(x, y, 0.0);
    }
    for i in 0..CAROUSEL_CAP_CLIPS {
        let (x, y) = polar_xy(i, CAROUSEL_CAP_CLIPS, CAROUSEL_PITCH_RADIUS);
        cap_clips = cap_clips
            + centered_cube(
                format!("closed_waste_quarantine_station_carousel_cap_clip_{i}"),
                16.0,
                5.0,
                10.0,
            )
            .translate(x * 0.86, y * 0.86, CAROUSEL_Z / 2.0 + 5.0);
    }
    disk - wells + hub + cap_clips
}

fn quarantine_bag_bottle_dock() -> Part {
    let dock = centered_cube(
        "closed_waste_quarantine_station_quarantine_bag_bottle_dock_body",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let mut pockets = Part::empty("closed_waste_quarantine_station_quarantine_bottle_pockets");
    for i in 0..QUARANTINE_BOTTLES {
        let x = centered_index(i, QUARANTINE_BOTTLES, 92.0);
        pockets = pockets
            + centered_cylinder(
                format!("closed_waste_quarantine_station_quarantine_bottle_pocket_{i}"),
                BOTTLE_POCKET_D / 2.0,
                QUARANTINE_Z + 8.0,
                48,
            )
            .translate(x, -42.0, 0.0);
    }
    let bag_lane_left = centered_cube(
        "closed_waste_quarantine_station_quarantine_bag_lane_left",
        120.0,
        62.0,
        20.0,
    )
    .translate(-76.0, 72.0, QUARANTINE_Z / 2.0 - 7.0);
    let bag_lane_right = centered_cube(
        "closed_waste_quarantine_station_quarantine_bag_lane_right",
        120.0,
        62.0,
        20.0,
    )
    .translate(76.0, 72.0, QUARANTINE_Z / 2.0 - 7.0);

    dock - pockets - bag_lane_left - bag_lane_right + quarantine_clamp_bridges()
}

fn quarantine_clamp_bridges() -> Part {
    let mut clamps = Part::empty("closed_waste_quarantine_station_quarantine_bag_clamps");
    for i in 0..BAG_CLAMP_COUNT {
        let x = centered_index(i, BAG_CLAMP_COUNT, 74.0);
        clamps = clamps
            + centered_cube(
                format!("closed_waste_quarantine_station_quarantine_bag_clamp_bridge_{i}"),
                46.0,
                12.0,
                24.0,
            )
            .translate(x, QUARANTINE_Y / 2.0 - 28.0, QUARANTINE_Z / 2.0 + 12.0);
    }
    clamps
}

fn leak_moat_backflow_witness_channel() -> Part {
    let rail = centered_cube(
        "closed_waste_quarantine_station_leak_moat_backflow_witness_rail",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut channels = Part::empty("closed_waste_quarantine_station_backflow_witness_channels");
    for i in 0..BACKFLOW_CHANNELS {
        let x = centered_index(i, BACKFLOW_CHANNELS, 58.0);
        channels = channels
            + centered_cube(
                format!("closed_waste_quarantine_station_backflow_witness_channel_{i}"),
                34.0,
                WITNESS_Y + 8.0,
                14.0,
            )
            .translate(x, 0.0, WITNESS_Z / 2.0 - 5.0);
    }
    rail - channels + witness_window_tabs()
}

fn witness_window_tabs() -> Part {
    let mut tabs = Part::empty("closed_waste_quarantine_station_clear_witness_window_tabs");
    for i in 0..WITNESS_WINDOWS {
        let x = centered_index(i, WITNESS_WINDOWS, 72.0);
        tabs = tabs
            + centered_cube(
                format!("closed_waste_quarantine_station_clear_witness_window_tab_{i}"),
                46.0,
                12.0,
                8.0,
            )
            .translate(x, -WITNESS_Y / 2.0 - 8.0, WITNESS_Z / 2.0 + 4.0);
    }
    tabs
}

fn filtered_vent_holder() -> Part {
    let holder = centered_cube(
        "closed_waste_quarantine_station_filtered_vent_holder_body",
        VENT_X,
        VENT_Y,
        VENT_Z,
    );
    let mut bores = Part::empty("closed_waste_quarantine_station_filtered_vent_bores");
    let mut collars = Part::empty("closed_waste_quarantine_station_filtered_vent_collars");
    for i in 0..FILTERED_VENTS {
        let x = centered_index(i, FILTERED_VENTS, 42.0);
        bores = bores
            + centered_cylinder(
                format!("closed_waste_quarantine_station_filtered_vent_bore_{i}"),
                VENT_CARTRIDGE_D / 2.0,
                VENT_Z + 10.0,
                36,
            )
            .translate(x, 0.0, 0.0);
        collars = collars
            + centered_cylinder(
                format!("closed_waste_quarantine_station_filtered_vent_retainer_collar_{i}"),
                VENT_CARTRIDGE_D / 2.0 + 6.0,
                10.0,
                36,
            )
            .translate(x, 0.0, VENT_Z / 2.0 + 5.0);
    }
    holder - bores + collars
}

fn barcode_rfid_custody_status_gates() -> Part {
    let panel = centered_cube(
        "closed_waste_quarantine_station_barcode_rfid_custody_status_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel + barcode_lands() + rfid_lands() + status_gate_tokens() + custody_zone_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_waste_quarantine_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i, BARCODE_LANDS, 48.0);
        lands = lands
            + centered_cube(
                format!("closed_waste_quarantine_station_barcode_land_{i}"),
                34.0,
                18.0,
                4.0,
            )
            .translate(x, -22.0, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("closed_waste_quarantine_station_rfid_lands");
    for i in 0..RFID_LANDS {
        let x = centered_index(i, RFID_LANDS, 58.0);
        lands = lands
            + centered_cube(
                format!("closed_waste_quarantine_station_rfid_land_{i}"),
                42.0,
                22.0,
                4.0,
            )
            .translate(x, 20.0, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn status_gate_tokens() -> Part {
    let mut gates = Part::empty("closed_waste_quarantine_station_release_hold_reject_gates");
    for (i, name) in STATUS_NAMES.iter().enumerate() {
        let x = -CUSTODY_X / 2.0 + 70.0 + i as f64 * 104.0;
        let gate = centered_cube(
            format!("closed_waste_quarantine_station_{name}_status_gate"),
            72.0,
            70.0,
            20.0,
        )
        .translate(x, 0.0, CUSTODY_Z / 2.0 + 10.0);
        let slot = centered_cube(
            format!("closed_waste_quarantine_station_{name}_status_token_slot"),
            48.0,
            12.0,
            24.0,
        )
        .translate(x, 0.0, CUSTODY_Z / 2.0 + 10.0);
        gates = gates + (gate - slot);
    }
    gates
}

fn custody_zone_tabs() -> Part {
    let mut tabs = Part::empty("closed_waste_quarantine_station_custody_zone_tabs");
    for (i, name) in CUSTODY_ZONE_NAMES.iter().enumerate() {
        let x = CUSTODY_X / 2.0 - 250.0 + i as f64 * 80.0;
        tabs = tabs
            + centered_cube(
                format!("closed_waste_quarantine_station_{name}_custody_zone_tab"),
                62.0,
                16.0,
                10.0,
            )
            .translate(x, CUSTODY_Y / 2.0 + 10.0, CUSTODY_Z / 2.0 + 5.0);
    }
    tabs
}

fn evidence_camera_bridge() -> Part {
    let post_z = BRIDGE_UNDERSIDE_Z;
    let left_post = centered_cube(
        "closed_waste_quarantine_station_camera_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let right_post = centered_cube(
        "closed_waste_quarantine_station_camera_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let beam = centered_cube(
        "closed_waste_quarantine_station_camera_bridge_crossbeam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, post_z / 2.0);

    left_post + right_post + beam + camera_mounts() + led_strip_lands()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_waste_quarantine_station_evidence_camera_mounts");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 240.0);
        let plate = centered_cube(
            format!("closed_waste_quarantine_station_evidence_camera_mount_plate_{i}"),
            70.0,
            42.0,
            10.0,
        )
        .translate(
            x,
            -BRIDGE_POST_Y / 2.0 - 8.0,
            BRIDGE_UNDERSIDE_Z / 2.0 - 8.0,
        );
        let lens_clearance = centered_cylinder(
            format!("closed_waste_quarantine_station_evidence_camera_lens_clearance_{i}"),
            11.0,
            14.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            -BRIDGE_POST_Y / 2.0 - 8.0,
            BRIDGE_UNDERSIDE_Z / 2.0 - 8.0,
        );
        mounts = mounts + (plate - lens_clearance);
    }
    mounts
}

fn led_strip_lands() -> Part {
    let mut lands = Part::empty("closed_waste_quarantine_station_camera_bridge_led_lands");
    for i in 0..LED_STRIP_COUNT {
        let x = centered_index(i, LED_STRIP_COUNT, 132.0);
        lands = lands
            + centered_cube(
                format!("closed_waste_quarantine_station_led_strip_land_{i}"),
                78.0,
                8.0,
                5.0,
            )
            .translate(x, BRIDGE_POST_Y / 2.0 + 6.0, BRIDGE_UNDERSIDE_Z / 2.0 - 8.0);
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_waste_quarantine_station_front_robot_keepout_envelope",
        STATION_X - 180.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, 0.0);
    let rear = centered_cube(
        "closed_waste_quarantine_station_rear_service_keepout_envelope",
        STATION_X - 160.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE, 0.0);
    let left = centered_cube(
        "closed_waste_quarantine_station_left_bulkhead_service_keepout_envelope",
        18.0,
        STATION_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 + LEFT_BULKHEAD_SERVICE_CLEARANCE, 0.0, 0.0);
    let right = centered_cube(
        "closed_waste_quarantine_station_right_quarantine_service_keepout_envelope",
        18.0,
        STATION_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_QUARANTINE_SERVICE_CLEARANCE,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn cleaning_wipe_witness_coupons() -> Part {
    let panel = centered_cube(
        "closed_waste_quarantine_station_cleaning_wipe_coupon_panel",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );
    let mut coupons = Part::empty("closed_waste_quarantine_station_removable_wipe_coupons");
    for i in 0..WIPE_COUPONS {
        let x = centered_index(i, WIPE_COUPONS, COUPON_PITCH);
        let coupon = centered_cube(
            format!("closed_waste_quarantine_station_wipe_witness_coupon_{i}"),
            24.0,
            58.0,
            8.0,
        )
        .translate(x, 0.0, COUPON_PANEL_Z / 2.0 + 4.0);
        let lift_notch = centered_cylinder(
            format!("closed_waste_quarantine_station_wipe_coupon_lift_notch_{i}"),
            4.0,
            28.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -30.0, COUPON_PANEL_Z / 2.0 + 4.0);
        coupons = coupons + (coupon - lift_notch);
    }
    panel + coupons
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn polar_xy(index: usize, count: usize, radius: f64) -> (f64, f64) {
    let angle = index as f64 * std::f64::consts::TAU / count as f64;
    (radius * angle.cos(), radius * angle.sin())
}

fn assert_layout() {
    assert!(STATION_X > BULKHEAD_X + MANIFOLD_X + QUARANTINE_X);
    assert!(STATION_Y > VIAL_NEST_Y + WITNESS_Y + CUSTODY_Y);
    assert_eq!(RETAIN_VIALS, 24);
    assert_eq!(SPLIT_OUTPUT_PORTS, 24);
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(KEEP_OUT_ZONES, 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_are_deterministic_and_prefixed() {
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_waste_quarantine_sample_retain_split_verification_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_manifest_covers_design_intent() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"sealed_waste_inlet_bulkhead"));
        assert!(REQUIRED_FEATURES.contains(&"sample_retain_vial_nest"));
        assert!(REQUIRED_FEATURES.contains(&"leak_moat_backflow_witness_channel"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn retain_and_split_counts_are_balanced_for_audit() {
        assert_eq!(RETAIN_VIAL_ROWS * RETAIN_VIAL_COLS, RETAIN_VIALS);
        assert_eq!(SPLIT_LANES * SPLIT_BRANCHES_PER_LANE, SPLIT_OUTPUT_PORTS);
        assert_eq!(RETAIN_VIALS, SPLIT_OUTPUT_PORTS);
        assert!(CAROUSEL_RETAIN_POSITIONS < RETAIN_VIALS);
    }

    #[test]
    fn centered_index_spacing_is_symmetric() {
        let first = centered_index(0, WASTE_INLET_PORTS, INLET_PITCH);
        let last = centered_index(WASTE_INLET_PORTS - 1, WASTE_INLET_PORTS, INLET_PITCH);
        assert!((first + last).abs() < 1e-9);
        assert_eq!(
            centered_index(3, WASTE_INLET_PORTS, INLET_PITCH)
                - centered_index(2, WASTE_INLET_PORTS, INLET_PITCH),
            INLET_PITCH
        );
    }

    #[test]
    fn polar_layout_places_carousel_wells_on_pitch_radius() {
        for i in 0..CAROUSEL_RETAIN_POSITIONS {
            let (x, y) = polar_xy(i, CAROUSEL_RETAIN_POSITIONS, CAROUSEL_PITCH_RADIUS);
            let radius = (x * x + y * y).sqrt();
            assert!((radius - CAROUSEL_PITCH_RADIUS).abs() < 1e-9);
        }
    }

    #[test]
    fn clearance_and_status_metadata_are_explicit() {
        assert_eq!(STATUS_NAMES, ["release", "hold", "reject"]);
        assert_eq!(STATUS_GATE_COUNT, STATUS_NAMES.len());
        assert_eq!(KEEP_OUT_ZONES, 4);
        assert!(FRONT_ROBOT_CLEARANCE > REAR_SERVICE_CLEARANCE);
        assert!(LEFT_BULKHEAD_SERVICE_CLEARANCE < RIGHT_QUARANTINE_SERVICE_CLEARANCE);
    }
}
