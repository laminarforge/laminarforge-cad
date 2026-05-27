use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent/additive traceability and dispense verification station.
//
// Intent:
// - Verify additive identity, lot custody, COA presence, dispense volume/mass,
//   retain sampling, and closed handoff before media formulation.
// - Keep reagent vial/bag nests, barcode/RFID/COA evidence lands, gravimetric
//   pads, dispense verification wells, sterile connector bulkhead, status
//   lanes, flush/waste routing, evidence camera bridge, and robot/service
//   keepouts mechanically explicit.
// - Model benchtop interface CAD only. This does not define release limits,
//   formulation instructions, aseptic processing validation, or reagent claims.

const OUTPUTS: [&str; 14] = [
    "output/closed_reagent_additive_trace_dispense_verification_station_deck.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_reagent_vial_bag_nests.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_barcode_rfid_coa_lands.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_small_volume_dispense_wells.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_gravimetric_pad_placeholders.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_temperature_light_protection_cover.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_sterile_connector_bulkhead.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_released_hold_reject_lanes.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_retain_sample_pockets.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_flush_waste_route.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_closed_handoff_dock.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_evidence_camera_bridge.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_robot_service_keepouts.stl",
    "output/closed_reagent_additive_trace_dispense_verification_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 13] = [
    "reagent_vial_bag_nests",
    "barcode_rfid_coa_lands",
    "small_volume_dispense_verification_wells",
    "gravimetric_pad_placeholders",
    "temperature_light_protection_cover",
    "sterile_connector_bulkhead",
    "released_hold_reject_lanes",
    "retain_sample_pockets",
    "flush_waste_route",
    "closed_handoff_dock",
    "evidence_camera_bridge",
    "robot_keepouts",
    "service_keepouts",
];

const DECK_X: f64 = 1120.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 20.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 34.0;
const DECK_RECESS_Z: f64 = 5.0;
const WIPE_GUTTER_W: f64 = 10.0;
const DRAIN_PORT_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.6;

const NEST_CENTER: (f64, f64) = (-350.0, 190.0);
const NEST_X: f64 = 340.0;
const NEST_Y: f64 = 230.0;
const NEST_Z: f64 = 46.0;
const BAG_NESTS: usize = 2;
const BAG_NEST_X: f64 = 132.0;
const BAG_NEST_Y: f64 = 86.0;
const BAG_NEST_DEPTH: f64 = 20.0;
const VIAL_ROWS: usize = 3;
const VIAL_COLS: usize = 4;
const VIAL_WELLS: usize = VIAL_ROWS * VIAL_COLS;
const VIAL_WELL_D: f64 = 24.0;
const VIAL_WELL_DEPTH: f64 = 27.0;
const VIAL_PITCH_X: f64 = 40.0;
const VIAL_PITCH_Y: f64 = 44.0;
const LOT_CLIP_COUNT: usize = 6;

const EVIDENCE_CENTER: (f64, f64) = (40.0, 260.0);
const EVIDENCE_X: f64 = 360.0;
const EVIDENCE_Y: f64 = 120.0;
const EVIDENCE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 3;
const COA_LANDS: usize = 2;
const EVIDENCE_LANDS: usize = BARCODE_LANDS + RFID_LANDS + COA_LANDS;
const BARCODE_LAND_X: f64 = 54.0;
const BARCODE_LAND_Y: f64 = 24.0;
const RFID_ANTENNA_D: f64 = 44.0;
const COA_LAND_X: f64 = 98.0;
const COA_LAND_Y: f64 = 40.0;

const DISPENSE_CENTER: (f64, f64) = (-330.0, -70.0);
const DISPENSE_X: f64 = 360.0;
const DISPENSE_Y: f64 = 220.0;
const DISPENSE_Z: f64 = 34.0;
const WELL_ROWS: usize = 3;
const WELL_COLS: usize = 8;
const DISPENSE_WELLS: usize = WELL_ROWS * WELL_COLS;
const DISPENSE_WELL_D: f64 = 10.5;
const DISPENSE_WELL_DEPTH: f64 = 18.0;
const WELL_PITCH_X: f64 = 34.0;
const WELL_PITCH_Y: f64 = 46.0;
const MENISCUS_WINDOWS: usize = 6;

const GRAV_CENTER: (f64, f64) = (60.0, -70.0);
const GRAV_X: f64 = 300.0;
const GRAV_Y: f64 = 220.0;
const GRAV_Z: f64 = 28.0;
const GRAV_PADS: usize = 4;
const GRAV_PAD_X: f64 = 106.0;
const GRAV_PAD_Y: f64 = 70.0;
const GRAV_PAD_Z: f64 = 7.0;
const SCALE_CABLE_CHANNEL_W: f64 = 12.0;

const COVER_CENTER: (f64, f64) = (-100.0, 20.0);
const COVER_X: f64 = 860.0;
const COVER_Y: f64 = 570.0;
const COVER_Z: f64 = 214.0;
const COVER_WALL_T: f64 = 12.0;
const COVER_ROOF_Z: f64 = 14.0;
const COVER_ACCESS_SLOT_X: f64 = 172.0;
const COVER_ACCESS_SLOT_Z: f64 = 70.0;
const AMBER_LOUVER_COUNT: usize = 8;
const TEMPERATURE_PORTS: usize = 4;

const BULKHEAD_CENTER: (f64, f64) = (370.0, 210.0);
const BULKHEAD_X: f64 = 300.0;
const BULKHEAD_Y: f64 = 46.0;
const BULKHEAD_Z: f64 = 170.0;
const CONNECTOR_PORTS: usize = 8;
const CONNECTOR_PORT_D: f64 = 22.0;
const CONNECTOR_COLLAR_D: f64 = 38.0;
const CONNECTOR_PITCH_X: f64 = 34.0;
const CONNECTOR_ROW_PITCH_Z: f64 = 48.0;

const STATUS_CENTER: (f64, f64) = (380.0, -70.0);
const STATUS_X: f64 = 280.0;
const STATUS_Y: f64 = 230.0;
const STATUS_Z: f64 = 26.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_X: f64 = 78.0;
const STATUS_LANE_Y: f64 = 182.0;
const STATUS_DIVIDER_W: f64 = 9.0;
const STATUS_SLOT_DEPTH: f64 = 12.0;
const STATUS_LANE_PITCH_X: f64 = 88.0;
const STATUS_CAPACITY_PER_LANE: usize = 5;

const RETAIN_CENTER: (f64, f64) = (-260.0, -270.0);
const RETAIN_X: f64 = 300.0;
const RETAIN_Y: f64 = 110.0;
const RETAIN_Z: f64 = 30.0;
const RETAIN_POCKETS: usize = 12;
const RETAIN_COLS: usize = 6;
const RETAIN_POCKET_D: f64 = 16.0;
const RETAIN_POCKET_DEPTH: f64 = 18.0;
const RETAIN_SEAL_PADS: usize = 6;

const FLUSH_CENTER: (f64, f64) = (250.0, -280.0);
const FLUSH_X: f64 = 430.0;
const FLUSH_Y: f64 = 120.0;
const FLUSH_Z: f64 = 32.0;
const FLUSH_CHANNEL_W: f64 = 16.0;
const FLUSH_PORTS: usize = 4;
const WASTE_BOTTLE_NEST_D: f64 = 64.0;
const WASTE_ROUTE_SLOPE_MARKERS: usize = 5;

const HANDOFF_CENTER: (f64, f64) = (385.0, 115.0);
const HANDOFF_X: f64 = 250.0;
const HANDOFF_Y: f64 = 84.0;
const HANDOFF_Z: f64 = 36.0;
const HANDOFF_PORTS: usize = 3;
const HANDOFF_DATUMS: usize = 4;

const BRIDGE_CENTER: (f64, f64) = (0.0, 20.0);
const BRIDGE_SPAN_X: f64 = 900.0;
const BRIDGE_POST_X: f64 = 26.0;
const BRIDGE_POST_Y: f64 = 46.0;
const BRIDGE_POST_Z: f64 = 196.0;
const BRIDGE_BEAM_Y: f64 = 54.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_COUNT: usize = 4;
const CAMERA_PITCH_X: f64 = 174.0;
const LIGHT_BAR_COUNT: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 156.0;

const ROBOT_KEEP_OUT_X: f64 = 320.0;
const ROBOT_KEEP_OUT_Y: f64 = 560.0;
const ROBOT_KEEP_OUT_Z: f64 = 170.0;
const SERVICE_KEEP_OUT_X: f64 = 930.0;
const SERVICE_KEEP_OUT_Y: f64 = 150.0;
const SERVICE_KEEP_OUT_Z: f64 = 132.0;
const SCALE_DRAWER_SERVICE_X: f64 = 250.0;
const COVER_LIFT_CLEARANCE_Z: f64 = 285.0;
const KEEP_OUT_RAIL: f64 = 6.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let nests = reagent_vial_bag_nests();
    export(OUTPUTS[1], &nests);

    let evidence = barcode_rfid_coa_lands();
    export(OUTPUTS[2], &evidence);

    let wells = small_volume_dispense_wells();
    export(OUTPUTS[3], &wells);

    let grav = gravimetric_pad_placeholders();
    export(OUTPUTS[4], &grav);

    let cover = temperature_light_protection_cover();
    export(OUTPUTS[5], &cover);

    let bulkhead = sterile_connector_bulkhead();
    export(OUTPUTS[6], &bulkhead);

    let status = released_hold_reject_lanes();
    export(OUTPUTS[7], &status);

    let retain = retain_sample_pockets();
    export(OUTPUTS[8], &retain);

    let flush = flush_waste_route();
    export(OUTPUTS[9], &flush);

    let handoff = closed_handoff_dock();
    export(OUTPUTS[10], &handoff);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[11], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[12], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed reagent/additive traceability and dispense verification station:");
    println!("  Footprint:                    {DECK_X:.0}mm x {DECK_Y:.0}mm cleanable deck");
    println!(
        "  Reagent custody:              {BAG_NESTS} bag nests, {VIAL_WELLS} vial wells, {LOT_CLIP_COUNT} custody clip lands"
    );
    println!(
        "  Evidence capture:             {EVIDENCE_LANDS} total lands ({BARCODE_LANDS} barcode, {RFID_LANDS} RFID, {COA_LANDS} COA), {CAMERA_COUNT} camera pods"
    );
    println!(
        "  Dispense verification:        {DISPENSE_WELLS} small-volume wells, {MENISCUS_WINDOWS} meniscus windows, {GRAV_PADS} gravimetric pad placeholders"
    );
    println!(
        "  Protection and closure:       {TEMPERATURE_PORTS} temperature ports, {AMBER_LOUVER_COUNT} amber light-shield louvers, {CONNECTOR_PORTS} sterile bulkhead ports"
    );
    println!(
        "  Status/custody handling:      {STATUS_LANES} released/hold/reject lanes x {STATUS_CAPACITY_PER_LANE} slots, {RETAIN_POCKETS} retain pockets, {RETAIN_SEAL_PADS} seal pads"
    );
    println!(
        "  Closed handoff:               {HANDOFF_PORTS} media-formulation handoff ports, {HANDOFF_DATUMS} datum pins, {FLUSH_PORTS} flush ports, waste nest {WASTE_BOTTLE_NEST_D:.0}mm OD"
    );
    println!(
        "  Keepouts:                     robot {ROBOT_KEEP_OUT_X:.0}mm x {ROBOT_KEEP_OUT_Y:.0}mm x {ROBOT_KEEP_OUT_Z:.0}mm, cover lift {COVER_LIFT_CLEARANCE_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    station_deck()
        + reagent_vial_bag_nests().translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z)
        + barcode_rfid_coa_lands().translate(EVIDENCE_CENTER.0, EVIDENCE_CENTER.1, DECK_Z)
        + small_volume_dispense_wells().translate(DISPENSE_CENTER.0, DISPENSE_CENTER.1, DECK_Z)
        + gravimetric_pad_placeholders().translate(GRAV_CENTER.0, GRAV_CENTER.1, DECK_Z)
        + temperature_light_protection_cover().translate(COVER_CENTER.0, COVER_CENTER.1, DECK_Z)
        + sterile_connector_bulkhead().translate(BULKHEAD_CENTER.0, BULKHEAD_CENTER.1, DECK_Z)
        + released_hold_reject_lanes().translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z)
        + retain_sample_pockets().translate(RETAIN_CENTER.0, RETAIN_CENTER.1, DECK_Z)
        + flush_waste_route().translate(FLUSH_CENTER.0, FLUSH_CENTER.1, DECK_Z)
        + closed_handoff_dock().translate(HANDOFF_CENTER.0, HANDOFF_CENTER.1, DECK_Z)
        + evidence_camera_bridge().translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, DECK_Z)
        + robot_service_keepouts()
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_reagent_additive_trace_dispense_station_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - module_recesses() - deck_mount_holes() - wipe_gutters() - leak_drain_port()
        + deck_rims()
        + datum_fiducials()
}

fn module_recesses() -> Part {
    let mut recesses =
        Part::empty("closed_reagent_additive_trace_dispense_station_module_recesses");
    for (name, center, x, y) in deck_recess_specs() {
        recesses = recesses
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_{name}_deck_recess"),
                x + 16.0,
                y + 16.0,
                DECK_RECESS_Z + 0.2,
            )
            .translate(center.0, center.1, DECK_Z - DECK_RECESS_Z / 2.0 + 0.1);
    }
    recesses
}

fn deck_recess_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        ("reagent_nests", NEST_CENTER, NEST_X, NEST_Y),
        ("evidence_lands", EVIDENCE_CENTER, EVIDENCE_X, EVIDENCE_Y),
        ("dispense_wells", DISPENSE_CENTER, DISPENSE_X, DISPENSE_Y),
        ("gravimetric_pads", GRAV_CENTER, GRAV_X, GRAV_Y),
        (
            "connector_bulkhead",
            BULKHEAD_CENTER,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        ("status_lanes", STATUS_CENTER, STATUS_X, STATUS_Y),
        ("retain_samples", RETAIN_CENTER, RETAIN_X, RETAIN_Y),
        ("flush_waste", FLUSH_CENTER, FLUSH_X, FLUSH_Y),
        ("handoff_dock", HANDOFF_CENTER, HANDOFF_X, HANDOFF_Y),
    ]
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_reagent_additive_trace_dispense_station_mount_holes");
    for (index, (x, y)) in [
        (-510.0, -340.0),
        (-170.0, -340.0),
        (170.0, -340.0),
        (510.0, -340.0),
        (-510.0, 340.0),
        (-170.0, 340.0),
        (170.0, 340.0),
        (510.0, 340.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn wipe_gutters() -> Part {
    let front = centered_cube(
        "closed_reagent_additive_trace_dispense_station_front_wipe_gutter",
        DECK_X - 90.0,
        WIPE_GUTTER_W,
        DECK_RECESS_Z + 0.2,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 42.0,
        DECK_Z - DECK_RECESS_Z / 2.0 + 0.1,
    );
    let rear = centered_cube(
        "closed_reagent_additive_trace_dispense_station_rear_wipe_gutter",
        DECK_X - 90.0,
        WIPE_GUTTER_W,
        DECK_RECESS_Z + 0.2,
    )
    .translate(0.0, DECK_Y / 2.0 - 42.0, DECK_Z - DECK_RECESS_Z / 2.0 + 0.1);
    front + rear
}

fn leak_drain_port() -> Part {
    centered_cylinder(
        "closed_reagent_additive_trace_dispense_station_deck_leak_drain_port",
        DRAIN_PORT_D / 2.0,
        DECK_RIM_W + 26.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 10.0, DECK_Z - 5.0)
}

fn deck_rims() -> Part {
    let front = centered_cube(
        "closed_reagent_additive_trace_dispense_station_front_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_reagent_additive_trace_dispense_station_rear_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_left_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_right_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn datum_fiducials() -> Part {
    let mut fiducials =
        Part::empty("closed_reagent_additive_trace_dispense_station_datum_fiducials");
    for (index, (x, y)) in [
        (-500.0, 310.0),
        (-250.0, 310.0),
        (0.0, 310.0),
        (250.0, 310.0),
        (500.0, 310.0),
        (-500.0, -310.0),
        (-250.0, -310.0),
        (0.0, -310.0),
        (250.0, -310.0),
        (500.0, -310.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_reagent_additive_trace_dispense_station_deck_fiducial_{index}"
            ))
            .translate(*x, *y, DECK_Z + 1.5);
    }
    fiducials
}

fn reagent_vial_bag_nests() -> Part {
    let body = centered_cube(
        "closed_reagent_additive_trace_dispense_station_reagent_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);

    body - bag_recesses() - vial_wells() - vial_finger_channels()
        + custody_clip_lands()
        + nest_guard_rails()
}

fn bag_recesses() -> Part {
    let mut cuts = Part::empty("closed_reagent_additive_trace_dispense_station_bag_recesses");
    for index in 0..BAG_NESTS {
        let x = centered_index(index, BAG_NESTS, 152.0);
        cuts = cuts
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_bag_recess_{index}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_NEST_DEPTH + 0.4,
            )
            .translate(x, 54.0, NEST_Z - BAG_NEST_DEPTH / 2.0 + 0.2);
    }
    cuts
}

fn vial_wells() -> Part {
    let mut wells = Part::empty("closed_reagent_additive_trace_dispense_station_vial_wells");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let index = row * VIAL_COLS + col;
            wells = wells
                + centered_cylinder(
                    format!("closed_reagent_additive_trace_dispense_station_vial_well_{index}"),
                    VIAL_WELL_D / 2.0,
                    VIAL_WELL_DEPTH + 0.4,
                    36,
                )
                .translate(
                    centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                    -78.0 + centered_index(row, VIAL_ROWS, VIAL_PITCH_Y),
                    NEST_Z - VIAL_WELL_DEPTH / 2.0 + 0.2,
                );
        }
    }
    wells
}

fn vial_finger_channels() -> Part {
    let mut channels =
        Part::empty("closed_reagent_additive_trace_dispense_station_vial_finger_channels");
    for col in 0..VIAL_COLS {
        channels = channels
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_vial_finger_channel_{col}"),
                12.0,
                146.0,
                9.0,
            )
            .translate(
                centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                -78.0,
                NEST_Z - 4.5,
            );
    }
    channels
}

fn custody_clip_lands() -> Part {
    let mut clips =
        Part::empty("closed_reagent_additive_trace_dispense_station_lot_custody_clip_lands");
    for index in 0..LOT_CLIP_COUNT {
        clips = clips
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_lot_clip_land_{index}"),
                34.0,
                11.0,
                5.0,
            )
            .translate(
                centered_index(index, LOT_CLIP_COUNT, 46.0),
                102.0,
                NEST_Z + 2.5,
            );
    }
    clips
}

fn nest_guard_rails() -> Part {
    let front = centered_cube(
        "closed_reagent_additive_trace_dispense_station_nest_front_guard_rail",
        NEST_X - 24.0,
        8.0,
        16.0,
    )
    .translate(0.0, -NEST_Y / 2.0 + 9.0, NEST_Z + 8.0);
    let rear = centered_cube(
        "closed_reagent_additive_trace_dispense_station_nest_rear_guard_rail",
        NEST_X - 24.0,
        8.0,
        16.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 9.0, NEST_Z + 8.0);
    front + rear
}

fn barcode_rfid_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_reagent_additive_trace_dispense_station_evidence_panel",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_Z / 2.0);

    panel + barcode_lands() + rfid_lands() + coa_lands() + evidence_panel_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_reagent_additive_trace_dispense_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_barcode_land_{index}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                centered_index(index, BARCODE_LANDS, 56.0),
                32.0,
                EVIDENCE_Z + 1.5,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut antennas =
        Part::empty("closed_reagent_additive_trace_dispense_station_rfid_antenna_lands");
    for index in 0..RFID_LANDS {
        antennas = antennas
            + (centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_rfid_outer_{index}"),
                RFID_ANTENNA_D / 2.0,
                3.0,
                40,
            ) - centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_rfid_inner_{index}"),
                RFID_ANTENNA_D / 2.0 - 6.0,
                4.0,
                40,
            ))
            .translate(
                centered_index(index, RFID_LANDS, 92.0),
                -12.0,
                EVIDENCE_Z + 1.5,
            );
    }
    antennas
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("closed_reagent_additive_trace_dispense_station_coa_lands");
    for index in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_coa_land_{index}"),
                COA_LAND_X,
                COA_LAND_Y,
                3.0,
            )
            .translate(
                centered_index(index, COA_LANDS, 122.0),
                -44.0,
                EVIDENCE_Z + 1.5,
            );
    }
    lands
}

fn evidence_panel_tabs() -> Part {
    let left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_evidence_left_latch_tab",
        28.0,
        14.0,
        9.0,
    )
    .translate(
        -EVIDENCE_X / 2.0 + 26.0,
        EVIDENCE_Y / 2.0 - 14.0,
        EVIDENCE_Z + 4.5,
    );
    let right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_evidence_right_latch_tab",
        28.0,
        14.0,
        9.0,
    )
    .translate(
        EVIDENCE_X / 2.0 - 26.0,
        EVIDENCE_Y / 2.0 - 14.0,
        EVIDENCE_Z + 4.5,
    );
    left + right
}

fn small_volume_dispense_wells() -> Part {
    let block = centered_cube(
        "closed_reagent_additive_trace_dispense_station_dispense_well_block",
        DISPENSE_X,
        DISPENSE_Y,
        DISPENSE_Z,
    )
    .translate(0.0, 0.0, DISPENSE_Z / 2.0);

    block - dispense_well_cuts() - meniscus_window_cuts()
        + row_identifier_tabs()
        + pipette_datum_rails()
}

fn dispense_well_cuts() -> Part {
    let mut wells =
        Part::empty("closed_reagent_additive_trace_dispense_station_dispense_well_cuts");
    for row in 0..WELL_ROWS {
        for col in 0..WELL_COLS {
            let index = row * WELL_COLS + col;
            wells = wells
                + centered_cylinder(
                    format!(
                        "closed_reagent_additive_trace_dispense_station_small_volume_well_{index}"
                    ),
                    DISPENSE_WELL_D / 2.0,
                    DISPENSE_WELL_DEPTH + 0.4,
                    28,
                )
                .translate(
                    centered_index(col, WELL_COLS, WELL_PITCH_X),
                    centered_index(row, WELL_ROWS, WELL_PITCH_Y),
                    DISPENSE_Z - DISPENSE_WELL_DEPTH / 2.0 + 0.2,
                );
        }
    }
    wells
}

fn meniscus_window_cuts() -> Part {
    let mut windows =
        Part::empty("closed_reagent_additive_trace_dispense_station_meniscus_window_cuts");
    for index in 0..MENISCUS_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_meniscus_window_{index}"),
                28.0,
                7.0,
                9.0,
            )
            .translate(
                centered_index(index, MENISCUS_WINDOWS, 48.0),
                -DISPENSE_Y / 2.0 + 18.0,
                DISPENSE_Z - 4.5,
            );
    }
    windows
}

fn row_identifier_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_reagent_additive_trace_dispense_station_dispense_row_identifier_tabs");
    for row in 0..WELL_ROWS {
        tabs = tabs
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_dispense_row_id_tab_{row}"),
                18.0,
                26.0,
                5.0,
            )
            .translate(
                -DISPENSE_X / 2.0 + 24.0,
                centered_index(row, WELL_ROWS, WELL_PITCH_Y),
                DISPENSE_Z + 2.5,
            );
    }
    tabs
}

fn pipette_datum_rails() -> Part {
    let left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_dispense_left_datum_rail",
        9.0,
        DISPENSE_Y - 34.0,
        12.0,
    )
    .translate(-DISPENSE_X / 2.0 + 13.0, 0.0, DISPENSE_Z + 6.0);
    let right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_dispense_right_datum_rail",
        9.0,
        DISPENSE_Y - 34.0,
        12.0,
    )
    .translate(DISPENSE_X / 2.0 - 13.0, 0.0, DISPENSE_Z + 6.0);
    left + right
}

fn gravimetric_pad_placeholders() -> Part {
    let base = centered_cube(
        "closed_reagent_additive_trace_dispense_station_gravimetric_pad_base",
        GRAV_X,
        GRAV_Y,
        GRAV_Z,
    )
    .translate(0.0, 0.0, GRAV_Z / 2.0);

    base - scale_cable_channels() + scale_pads() + load_cell_datum_bosses()
}

fn scale_pads() -> Part {
    let mut pads =
        Part::empty("closed_reagent_additive_trace_dispense_station_gravimetric_pad_placeholders");
    for index in 0..GRAV_PADS {
        let col = index % 2;
        let row = index / 2;
        let x = centered_index(col, 2, 132.0);
        let y = centered_index(row, 2, 94.0);
        pads = pads
            + centered_cube(
                format!(
                    "closed_reagent_additive_trace_dispense_station_scale_pan_placeholder_{index}"
                ),
                GRAV_PAD_X,
                GRAV_PAD_Y,
                GRAV_PAD_Z,
            )
            .translate(x, y, GRAV_Z + GRAV_PAD_Z / 2.0);
        pads = pads
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_scale_guard_ring_{index}"),
                GRAV_PAD_X + 18.0,
                GRAV_PAD_Y + 18.0,
                3.0,
            )
            .translate(x, y, GRAV_Z + GRAV_PAD_Z + 1.5);
    }
    pads
}

fn scale_cable_channels() -> Part {
    let horizontal = centered_cube(
        "closed_reagent_additive_trace_dispense_station_scale_horizontal_cable_channel",
        GRAV_X - 30.0,
        SCALE_CABLE_CHANNEL_W,
        9.0,
    )
    .translate(0.0, 0.0, GRAV_Z - 4.5);
    let vertical = centered_cube(
        "closed_reagent_additive_trace_dispense_station_scale_vertical_cable_channel",
        SCALE_CABLE_CHANNEL_W,
        GRAV_Y - 30.0,
        9.0,
    )
    .translate(0.0, 0.0, GRAV_Z - 4.5);
    horizontal + vertical
}

fn load_cell_datum_bosses() -> Part {
    let mut bosses =
        Part::empty("closed_reagent_additive_trace_dispense_station_load_cell_datum_bosses");
    for index in 0..GRAV_PADS {
        let col = index % 2;
        let row = index / 2;
        bosses = bosses
            + centered_cylinder(
                format!(
                    "closed_reagent_additive_trace_dispense_station_load_cell_datum_boss_{index}"
                ),
                5.0,
                8.0,
                24,
            )
            .translate(
                centered_index(col, 2, 132.0),
                centered_index(row, 2, 94.0),
                GRAV_Z + 4.0,
            );
    }
    bosses
}

fn temperature_light_protection_cover() -> Part {
    cover_walls() + cover_roof() + amber_louvers() + temperature_probe_ports() + cover_lift_tabs()
}

fn cover_walls() -> Part {
    let left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_left_wall",
        COVER_WALL_T,
        COVER_Y,
        COVER_Z,
    )
    .translate(-COVER_X / 2.0 + COVER_WALL_T / 2.0, 0.0, COVER_Z / 2.0);
    let right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_right_wall",
        COVER_WALL_T,
        COVER_Y,
        COVER_Z,
    )
    .translate(COVER_X / 2.0 - COVER_WALL_T / 2.0, 0.0, COVER_Z / 2.0);
    let rear = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_rear_wall",
        COVER_X,
        COVER_WALL_T,
        COVER_Z,
    )
    .translate(0.0, COVER_Y / 2.0 - COVER_WALL_T / 2.0, COVER_Z / 2.0);
    let front_left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_front_left_wall",
        (COVER_X - COVER_ACCESS_SLOT_X) / 2.0,
        COVER_WALL_T,
        COVER_Z,
    )
    .translate(
        -(COVER_X + COVER_ACCESS_SLOT_X) / 4.0,
        -COVER_Y / 2.0 + COVER_WALL_T / 2.0,
        COVER_Z / 2.0,
    );
    let front_right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_front_right_wall",
        (COVER_X - COVER_ACCESS_SLOT_X) / 2.0,
        COVER_WALL_T,
        COVER_Z,
    )
    .translate(
        (COVER_X + COVER_ACCESS_SLOT_X) / 4.0,
        -COVER_Y / 2.0 + COVER_WALL_T / 2.0,
        COVER_Z / 2.0,
    );
    let front_header = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_front_access_header",
        COVER_ACCESS_SLOT_X,
        COVER_WALL_T,
        COVER_Z - COVER_ACCESS_SLOT_Z,
    )
    .translate(
        0.0,
        -COVER_Y / 2.0 + COVER_WALL_T / 2.0,
        COVER_ACCESS_SLOT_Z + (COVER_Z - COVER_ACCESS_SLOT_Z) / 2.0,
    );

    left + right + rear + front_left + front_right + front_header
}

fn cover_roof() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_light_protective_roof",
        COVER_X,
        COVER_Y,
        COVER_ROOF_Z,
    )
    .translate(0.0, 0.0, COVER_Z + COVER_ROOF_Z / 2.0)
}

fn amber_louvers() -> Part {
    let mut louvers =
        Part::empty("closed_reagent_additive_trace_dispense_station_amber_light_louvers");
    for index in 0..AMBER_LOUVER_COUNT {
        louvers = louvers
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_amber_louver_{index}"),
                60.0,
                7.0,
                18.0,
            )
            .rotate(18.0, 0.0, 0.0)
            .translate(
                centered_index(index, AMBER_LOUVER_COUNT, 74.0),
                -COVER_Y / 2.0 + 9.0,
                COVER_Z - 44.0,
            );
    }
    louvers
}

fn temperature_probe_ports() -> Part {
    let mut ports =
        Part::empty("closed_reagent_additive_trace_dispense_station_temperature_probe_ports");
    for index in 0..TEMPERATURE_PORTS {
        ports = ports
            + (centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_temperature_probe_grommet_outer_{index}"),
                13.0,
                5.0,
                28,
            ) - centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_temperature_probe_grommet_inner_{index}"),
                5.0,
                6.0,
                28,
            ))
            .translate(centered_index(index, TEMPERATURE_PORTS, 92.0), COVER_Y / 2.0 - 14.0, COVER_Z - 60.0);
    }
    ports
}

fn cover_lift_tabs() -> Part {
    let left = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_left_lift_tab",
        58.0,
        18.0,
        18.0,
    )
    .translate(-COVER_X / 2.0 + 72.0, 0.0, COVER_Z + COVER_ROOF_Z + 9.0);
    let right = centered_cube(
        "closed_reagent_additive_trace_dispense_station_cover_right_lift_tab",
        58.0,
        18.0,
        18.0,
    )
    .translate(COVER_X / 2.0 - 72.0, 0.0, COVER_Z + COVER_ROOF_Z + 9.0);
    left + right
}

fn sterile_connector_bulkhead() -> Part {
    let panel = centered_cube(
        "closed_reagent_additive_trace_dispense_station_sterile_connector_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_Z / 2.0);

    panel - connector_port_cuts() + connector_collars() + bulkhead_label_shelf()
}

fn connector_port_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_reagent_additive_trace_dispense_station_connector_port_cuts");
    for row in 0..2 {
        for col in 0..4 {
            let index = row * 4 + col;
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_reagent_additive_trace_dispense_station_connector_port_cut_{index}"
                    ),
                    CONNECTOR_PORT_D / 2.0,
                    BULKHEAD_Y + 4.0,
                    36,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(col, 4, CONNECTOR_PITCH_X),
                    0.0,
                    58.0 + row as f64 * CONNECTOR_ROW_PITCH_Z,
                );
        }
    }
    cuts
}

fn connector_collars() -> Part {
    let mut collars =
        Part::empty("closed_reagent_additive_trace_dispense_station_connector_port_collars");
    for row in 0..2 {
        for col in 0..4 {
            let index = row * 4 + col;
            collars = collars
                + (centered_cylinder(
                    format!("closed_reagent_additive_trace_dispense_station_connector_collar_outer_{index}"),
                    CONNECTOR_COLLAR_D / 2.0,
                    8.0,
                    36,
                ) - centered_cylinder(
                    format!("closed_reagent_additive_trace_dispense_station_connector_collar_inner_{index}"),
                    CONNECTOR_PORT_D / 2.0,
                    9.0,
                    36,
                ))
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(col, 4, CONNECTOR_PITCH_X),
                    -BULKHEAD_Y / 2.0 - 4.0,
                    58.0 + row as f64 * CONNECTOR_ROW_PITCH_Z,
                );
        }
    }
    collars
}

fn bulkhead_label_shelf() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_bulkhead_lot_label_shelf",
        BULKHEAD_X - 40.0,
        18.0,
        10.0,
    )
    .translate(0.0, -BULKHEAD_Y / 2.0 - 9.0, 20.0)
}

fn released_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_reagent_additive_trace_dispense_station_status_lane_base",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);

    base - status_slot_cuts() + status_dividers() + status_gate_posts()
}

fn status_slot_cuts() -> Part {
    let mut slots =
        Part::empty("closed_reagent_additive_trace_dispense_station_status_lane_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_CAPACITY_PER_LANE {
            slots = slots
                + centered_cube(
                    format!(
                        "closed_reagent_additive_trace_dispense_station_{}_slot_{slot}",
                        status_lane_name(lane)
                    ),
                    STATUS_LANE_X - 18.0,
                    24.0,
                    STATUS_SLOT_DEPTH + 0.2,
                )
                .translate(
                    status_lane_x(lane),
                    centered_index(slot, STATUS_CAPACITY_PER_LANE, 32.0),
                    STATUS_Z - STATUS_SLOT_DEPTH / 2.0 + 0.1,
                );
        }
    }
    slots
}

fn status_dividers() -> Part {
    let mut dividers =
        Part::empty("closed_reagent_additive_trace_dispense_station_status_lane_dividers");
    for index in 0..(STATUS_LANES - 1) {
        let x = (status_lane_x(index) + status_lane_x(index + 1)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_status_divider_{index}"),
                STATUS_DIVIDER_W,
                STATUS_Y - 24.0,
                26.0,
            )
            .translate(x, 0.0, STATUS_Z + 13.0);
    }
    dividers
}

fn status_gate_posts() -> Part {
    let mut posts =
        Part::empty("closed_reagent_additive_trace_dispense_station_status_lane_gate_posts");
    for lane in 0..STATUS_LANES {
        posts = posts
            + centered_cube(
                format!(
                    "closed_reagent_additive_trace_dispense_station_{}_release_gate_post",
                    status_lane_name(lane)
                ),
                18.0,
                18.0,
                54.0,
            )
            .translate(status_lane_x(lane), STATUS_Y / 2.0 - 22.0, STATUS_Z + 27.0);
    }
    posts
}

fn retain_sample_pockets() -> Part {
    let body = centered_cube(
        "closed_reagent_additive_trace_dispense_station_retain_sample_block",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    )
    .translate(0.0, 0.0, RETAIN_Z / 2.0);

    body - retain_pocket_cuts() + retain_seal_pads() + retain_custody_rail()
}

fn retain_pocket_cuts() -> Part {
    let mut pockets =
        Part::empty("closed_reagent_additive_trace_dispense_station_retain_sample_pocket_cuts");
    for index in 0..RETAIN_POCKETS {
        let col = index % RETAIN_COLS;
        let row = index / RETAIN_COLS;
        pockets = pockets
            + centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_retain_pocket_{index}"),
                RETAIN_POCKET_D / 2.0,
                RETAIN_POCKET_DEPTH + 0.4,
                28,
            )
            .translate(
                centered_index(col, RETAIN_COLS, 38.0),
                centered_index(row, 2, 42.0),
                RETAIN_Z - RETAIN_POCKET_DEPTH / 2.0 + 0.2,
            );
    }
    pockets
}

fn retain_seal_pads() -> Part {
    let mut pads =
        Part::empty("closed_reagent_additive_trace_dispense_station_retain_tamper_seal_pads");
    for index in 0..RETAIN_SEAL_PADS {
        pads = pads
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_retain_seal_pad_{index}"),
                30.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(index, RETAIN_SEAL_PADS, 42.0),
                -RETAIN_Y / 2.0 + 16.0,
                RETAIN_Z + 2.0,
            );
    }
    pads
}

fn retain_custody_rail() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_retain_custody_lock_rail",
        RETAIN_X - 24.0,
        8.0,
        16.0,
    )
    .translate(0.0, RETAIN_Y / 2.0 - 12.0, RETAIN_Z + 8.0)
}

fn flush_waste_route() -> Part {
    let body = centered_cube(
        "closed_reagent_additive_trace_dispense_station_flush_waste_body",
        FLUSH_X,
        FLUSH_Y,
        FLUSH_Z,
    )
    .translate(0.0, 0.0, FLUSH_Z / 2.0);

    body - flush_channel_cuts() - waste_bottle_nest_cut() - flush_port_cuts()
        + waste_route_slope_markers()
        + waste_bag_guard()
}

fn flush_channel_cuts() -> Part {
    let main = centered_cube(
        "closed_reagent_additive_trace_dispense_station_flush_main_channel",
        FLUSH_X - 92.0,
        FLUSH_CHANNEL_W,
        12.0,
    )
    .translate(-26.0, 0.0, FLUSH_Z - 6.0);
    let branch = centered_cube(
        "closed_reagent_additive_trace_dispense_station_flush_branch_channel",
        FLUSH_CHANNEL_W,
        FLUSH_Y - 36.0,
        12.0,
    )
    .translate(88.0, 0.0, FLUSH_Z - 6.0);
    main + branch
}

fn waste_bottle_nest_cut() -> Part {
    centered_cylinder(
        "closed_reagent_additive_trace_dispense_station_waste_bottle_nest_cut",
        WASTE_BOTTLE_NEST_D / 2.0,
        FLUSH_Z + 0.4,
        40,
    )
    .translate(FLUSH_X / 2.0 - 70.0, 0.0, FLUSH_Z / 2.0)
}

fn flush_port_cuts() -> Part {
    let mut ports = Part::empty("closed_reagent_additive_trace_dispense_station_flush_port_cuts");
    for index in 0..FLUSH_PORTS {
        ports = ports
            + centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_flush_port_cut_{index}"),
                5.0,
                FLUSH_Z + 0.4,
                24,
            )
            .translate(
                -FLUSH_X / 2.0 + 38.0 + index as f64 * 42.0,
                0.0,
                FLUSH_Z / 2.0,
            );
    }
    ports
}

fn waste_route_slope_markers() -> Part {
    let mut markers =
        Part::empty("closed_reagent_additive_trace_dispense_station_waste_route_slope_markers");
    for index in 0..WASTE_ROUTE_SLOPE_MARKERS {
        markers = markers
            + centered_cube(
                format!(
                    "closed_reagent_additive_trace_dispense_station_waste_slope_marker_{index}"
                ),
                28.0,
                4.0,
                4.0,
            )
            .translate(
                centered_index(index, WASTE_ROUTE_SLOPE_MARKERS, 48.0),
                -18.0,
                FLUSH_Z + 2.0,
            );
    }
    markers
}

fn waste_bag_guard() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_waste_bag_guard_rail",
        96.0,
        8.0,
        30.0,
    )
    .translate(FLUSH_X / 2.0 - 70.0, -FLUSH_Y / 2.0 + 13.0, FLUSH_Z + 15.0)
}

fn closed_handoff_dock() -> Part {
    let body = centered_cube(
        "closed_reagent_additive_trace_dispense_station_handoff_dock_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    )
    .translate(0.0, 0.0, HANDOFF_Z / 2.0);

    body - handoff_connector_cuts() - handoff_carrier_socket()
        + handoff_datum_pins()
        + handoff_clamp_bridge()
}

fn handoff_connector_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_reagent_additive_trace_dispense_station_handoff_connector_cuts");
    for index in 0..HANDOFF_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_handoff_port_cut_{index}"),
                8.0,
                HANDOFF_Y + 0.4,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, HANDOFF_PORTS, 46.0),
                0.0,
                HANDOFF_Z / 2.0,
            );
    }
    cuts
}

fn handoff_carrier_socket() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_handoff_media_formulation_socket",
        HANDOFF_X - 54.0,
        HANDOFF_Y - 32.0,
        12.0,
    )
    .translate(0.0, 0.0, HANDOFF_Z - 6.0)
}

fn handoff_datum_pins() -> Part {
    let mut datums =
        Part::empty("closed_reagent_additive_trace_dispense_station_handoff_datum_pins");
    for (index, (x, y)) in [
        (-HANDOFF_X / 2.0 + 30.0, -HANDOFF_Y / 2.0 + 18.0),
        (HANDOFF_X / 2.0 - 30.0, -HANDOFF_Y / 2.0 + 18.0),
        (-HANDOFF_X / 2.0 + 30.0, HANDOFF_Y / 2.0 - 18.0),
        (HANDOFF_X / 2.0 - 30.0, HANDOFF_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_reagent_additive_trace_dispense_station_handoff_datum_pin_{index}"),
                5.0,
                10.0,
                24,
            )
            .translate(*x, *y, HANDOFF_Z + 5.0);
    }
    datums
}

fn handoff_clamp_bridge() -> Part {
    centered_cube(
        "closed_reagent_additive_trace_dispense_station_handoff_closed_clamp_bridge",
        HANDOFF_X - 70.0,
        10.0,
        42.0,
    )
    .translate(0.0, HANDOFF_Y / 2.0 - 10.0, HANDOFF_Z + 21.0)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_reagent_additive_trace_dispense_station_camera_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let right_post = centered_cube(
        "closed_reagent_additive_trace_dispense_station_camera_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let beam = centered_cube(
        "closed_reagent_additive_trace_dispense_station_camera_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);

    left_post + right_post + beam + camera_pods() + bridge_light_bars()
}

fn camera_pods() -> Part {
    let mut pods =
        Part::empty("closed_reagent_additive_trace_dispense_station_evidence_camera_pods");
    for index in 0..CAMERA_COUNT {
        pods = pods
            + centered_cube(
                format!(
                    "closed_reagent_additive_trace_dispense_station_evidence_camera_pod_{index}"
                ),
                42.0,
                34.0,
                24.0,
            )
            .translate(
                centered_index(index, CAMERA_COUNT, CAMERA_PITCH_X),
                -BRIDGE_BEAM_Y / 2.0 - 18.0,
                CAMERA_CLEARANCE_Z,
            );
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty("closed_reagent_additive_trace_dispense_station_bridge_light_bars");
    for index in 0..LIGHT_BAR_COUNT {
        let y = if index == 0 {
            -BRIDGE_BEAM_Y / 2.0 - 5.0
        } else {
            BRIDGE_BEAM_Y / 2.0 + 5.0
        };
        bars = bars
            + centered_cube(
                format!("closed_reagent_additive_trace_dispense_station_bridge_light_bar_{index}"),
                BRIDGE_SPAN_X - 130.0,
                8.0,
                8.0,
            )
            .translate(0.0, y, BRIDGE_POST_Z - 20.0);
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let robot = clearance_box(
        "closed_reagent_additive_trace_dispense_station_robot_front_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(
        -DECK_X / 2.0 - ROBOT_KEEP_OUT_X / 2.0 + 42.0,
        0.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );

    let rear_service = clearance_box(
        "closed_reagent_additive_trace_dispense_station_rear_service_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + SERVICE_KEEP_OUT_Y / 2.0 - 36.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );

    let scale_service = clearance_box(
        "closed_reagent_additive_trace_dispense_station_scale_drawer_service_keepout",
        SCALE_DRAWER_SERVICE_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
        KEEP_OUT_RAIL,
    )
    .translate(
        GRAV_CENTER.0,
        -DECK_Y / 2.0 - SERVICE_KEEP_OUT_Y / 2.0 + 38.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );

    let cover_lift = clearance_box(
        "closed_reagent_additive_trace_dispense_station_cover_lift_keepout",
        COVER_X,
        COVER_Y,
        COVER_LIFT_CLEARANCE_Z,
        KEEP_OUT_RAIL,
    )
    .translate(COVER_CENTER.0, COVER_CENTER.1, COVER_LIFT_CLEARANCE_Z / 2.0);

    robot + rear_service + scale_service + cover_lift
}

fn clearance_box(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut part = Part::empty(format!("{name}_rails"));
    for (i, dx) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_post_{i}_{j}"), rail, rail, z).translate(
                    dx * x / 2.0,
                    dy * y / 2.0,
                    0.0,
                );
        }
    }
    for (i, dz) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_x_rail_{i}_{j}"), x, rail, rail).translate(
                    0.0,
                    dy * y / 2.0,
                    dz * z / 2.0,
                );
        }
        for (j, dx) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_y_rail_{i}_{j}"), rail, y, rail).translate(
                    dx * x / 2.0,
                    0.0,
                    dz * z / 2.0,
                );
        }
    }
    part
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 10.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center"), 3.2, 4.0, 24)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn status_lane_name(index: usize) -> &'static str {
    match index {
        0 => "released_lane",
        1 => "hold_lane",
        2 => "reject_lane",
        _ => "unknown_lane",
    }
}

fn status_lane_x(index: usize) -> f64 {
    centered_index(index, STATUS_LANES, STATUS_LANE_PITCH_X)
}

fn custody_status_gap_x() -> f64 {
    let retain = rect(RETAIN_CENTER, RETAIN_X, RETAIN_Y);
    let status = rect(STATUS_CENTER, STATUS_X, STATUS_Y);
    status.x0 - retain.x1
}

fn assert_layout() {
    for (name, center, x, y) in module_specs() {
        assert!(
            fits_on_deck(center, x, y, 24.0),
            "{name} exceeds station deck envelope"
        );
    }

    let primary = [
        ("nests", rect(NEST_CENTER, NEST_X, NEST_Y)),
        ("evidence", rect(EVIDENCE_CENTER, EVIDENCE_X, EVIDENCE_Y)),
        ("dispense", rect(DISPENSE_CENTER, DISPENSE_X, DISPENSE_Y)),
        ("gravimetric", rect(GRAV_CENTER, GRAV_X, GRAV_Y)),
        ("bulkhead", rect(BULKHEAD_CENTER, BULKHEAD_X, BULKHEAD_Y)),
        ("status", rect(STATUS_CENTER, STATUS_X, STATUS_Y)),
        ("retain", rect(RETAIN_CENTER, RETAIN_X, RETAIN_Y)),
        ("flush", rect(FLUSH_CENTER, FLUSH_X, FLUSH_Y)),
        ("handoff", rect(HANDOFF_CENTER, HANDOFF_X, HANDOFF_Y)),
    ];

    for i in 0..primary.len() {
        for j in (i + 1)..primary.len() {
            assert!(
                !rects_overlap(primary[i].1, primary[j].1),
                "{} overlaps {}",
                primary[i].0,
                primary[j].0
            );
        }
    }

    assert!(COVER_Z > CAMERA_CLEARANCE_Z);
    assert!(CONNECTOR_PORTS == 8);
    assert!(STATUS_LANE_Y < STATUS_Y);
    assert!(status_lane_x(0) < status_lane_x(1));
    assert!(status_lane_x(1) < status_lane_x(2));
    assert!(custody_status_gap_x() >= 60.0);
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        ("reagent_vial_bag_nests", NEST_CENTER, NEST_X, NEST_Y),
        (
            "barcode_rfid_coa_lands",
            EVIDENCE_CENTER,
            EVIDENCE_X,
            EVIDENCE_Y,
        ),
        ("dispense_wells", DISPENSE_CENTER, DISPENSE_X, DISPENSE_Y),
        ("gravimetric_pads", GRAV_CENTER, GRAV_X, GRAV_Y),
        ("temperature_light_cover", COVER_CENTER, COVER_X, COVER_Y),
        (
            "sterile_connector_bulkhead",
            BULKHEAD_CENTER,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        ("status_lanes", STATUS_CENTER, STATUS_X, STATUS_Y),
        ("retain_samples", RETAIN_CENTER, RETAIN_X, RETAIN_Y),
        ("flush_waste_route", FLUSH_CENTER, FLUSH_X, FLUSH_Y),
        ("closed_handoff_dock", HANDOFF_CENTER, HANDOFF_X, HANDOFF_Y),
    ]
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0 - x / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - y / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + y / 2.0 <= DECK_Y / 2.0 - margin
}

#[derive(Clone, Copy)]
struct Rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

fn rect(center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect {
        x0: center.0 - x / 2.0,
        x1: center.0 + x / 2.0,
        y0: center.1 - y / 2.0,
        y1: center.1 + y / 2.0,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x0 < b.x1 && a.x1 > b.x0 && a.y0 < b.y1 && a.y1 > b.y0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_include_assembly() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_reagent_additive_trace_dispense_verification_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_are_explicit_for_traceability_and_dispense() {
        assert_eq!(REQUIRED_FEATURES.len(), 13);
        for feature in [
            "reagent_vial_bag_nests",
            "barcode_rfid_coa_lands",
            "small_volume_dispense_verification_wells",
            "gravimetric_pad_placeholders",
            "temperature_light_protection_cover",
            "sterile_connector_bulkhead",
            "released_hold_reject_lanes",
            "retain_sample_pockets",
            "flush_waste_route",
            "closed_handoff_dock",
            "evidence_camera_bridge",
            "robot_keepouts",
            "service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn geometry_fits_station_envelope_without_primary_collisions() {
        assert_layout();
        assert!(DECK_X <= 1120.0);
        assert!(DECK_Y <= 760.0);
        assert!(COVER_X < DECK_X);
        assert!(COVER_Y < DECK_Y);
        assert!(BRIDGE_SPAN_X + BRIDGE_POST_X < DECK_X);
        assert!(BULKHEAD_Z < COVER_Z);
    }

    #[test]
    fn reagent_and_evidence_capacity_matches_custody_intent() {
        assert_eq!(VIAL_WELLS, VIAL_ROWS * VIAL_COLS);
        assert_eq!(VIAL_WELLS, 12);
        assert_eq!(BAG_NESTS, 2);
        assert_eq!(EVIDENCE_LANDS, BARCODE_LANDS + RFID_LANDS + COA_LANDS);
        assert!(BARCODE_LANDS >= BAG_NESTS + RFID_LANDS);
        assert!(COA_LANDS >= BAG_NESTS);
        assert!(LOT_CLIP_COUNT >= BARCODE_LANDS);
    }

    #[test]
    fn dispense_mass_and_flush_controls_have_sufficient_channels() {
        assert_eq!(DISPENSE_WELLS, WELL_ROWS * WELL_COLS);
        assert_eq!(DISPENSE_WELLS, 24);
        assert_eq!(GRAV_PADS, 4);
        assert!(DISPENSE_WELL_DEPTH > DISPENSE_WELL_D);
        assert!(GRAV_PAD_X > 2.0 * WASTE_BOTTLE_NEST_D / 3.0);
        assert_eq!(FLUSH_PORTS, 4);
        assert!(WASTE_ROUTE_SLOPE_MARKERS >= 5);
    }

    #[test]
    fn status_and_custody_paths_are_physically_separated() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(status_lane_name(0), "released_lane");
        assert_eq!(status_lane_name(1), "hold_lane");
        assert_eq!(status_lane_name(2), "reject_lane");
        assert!(STATUS_LANE_PITCH_X > STATUS_LANE_X + STATUS_DIVIDER_W);
        assert!(custody_status_gap_x() >= 60.0);
        assert!(RETAIN_CENTER.0 < DISPENSE_CENTER.0 + DISPENSE_X / 2.0);
        assert!(STATUS_CENTER.0 > GRAV_CENTER.0 + GRAV_X / 2.0);
        assert_eq!(RETAIN_POCKETS, 12);
        assert_eq!(RETAIN_SEAL_PADS, 6);
    }

    #[test]
    fn closed_handoff_and_bulkhead_are_sized_for_media_formulation_transfer() {
        assert_eq!(CONNECTOR_PORTS, 8);
        assert!(connector_row_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
        assert_eq!(HANDOFF_PORTS, 3);
        assert_eq!(HANDOFF_DATUMS, 4);
        assert!(HANDOFF_CENTER.1 + HANDOFF_Y / 2.0 < BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0);
        assert!(HANDOFF_CENTER.0 - HANDOFF_X / 2.0 > STATUS_CENTER.0 - STATUS_X / 2.0);
    }

    #[test]
    fn protection_bridge_and_keepouts_are_declared() {
        assert_eq!(TEMPERATURE_PORTS, 4);
        assert_eq!(AMBER_LOUVER_COUNT, 8);
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(LIGHT_BAR_COUNT, 2);
        assert!(CAMERA_CLEARANCE_Z > DISPENSE_Z + DECK_Z);
        assert!(ROBOT_KEEP_OUT_Z >= 160.0);
        assert!(SERVICE_KEEP_OUT_X > DECK_X * 0.8);
        assert!(SCALE_DRAWER_SERVICE_X >= GRAV_PAD_X * 2.0);
        assert!(COVER_LIFT_CLEARANCE_Z > COVER_Z);
    }

    fn connector_row_span_x() -> f64 {
        CONNECTOR_PITCH_X * 3.0
    }
}
