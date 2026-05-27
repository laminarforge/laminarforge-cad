use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag spike/connector port swab residue station.
//
// Design intent:
// - Hold a closed media bag and its spike/connector ports in repeatable datum
//   nests so disinfectant residue, retained rinse, and exposed wet surfaces can
//   be witnessed before connection.
// - Make spike-port witness pockets, swab recovery slots, connector cap
//   parking, wet/dry comparison coupons, residue/rinse collection wells,
//   custody/barcode lands, clean/used segregation, camera evidence capture, and
//   robot/service keepout gauges explicit in manufacturable CSG geometry.
// - This is validation fixture CAD only. It does not define a cleaning
//   procedure, disinfection claim, residue acceptance threshold, or sterile
//   connection protocol.

const BIN_NAME: &str = "closed_media_bag_spike_port_swab_residue_station";
const OUTPUT_PREFIX: &str = "output/closed_media_bag_spike_port_swab_residue_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_bag_spike_port_swab_residue_station_base_leak_tray.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_media_bag_nest.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_spike_port_witness_pockets.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_swab_recovery_slots.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_connector_cap_parking.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_wet_dry_comparison_coupons.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_residue_rinse_collection_wells.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_barcode_custody_lands.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_clean_used_segregation_gate.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_evidence_camera_bridge.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_robot_service_keepout_gauges.stl",
    "output/closed_media_bag_spike_port_swab_residue_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "media_bag_nest",
    "spike_port_witness_pockets",
    "swab_recovery_slots",
    "connector_cap_parking",
    "wet_dry_comparison_coupons",
    "residue_rinse_collection_wells",
    "barcode_custody_lands",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "exposed_wet_surface_witnessing",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "no_cleaning_protocol",
    "no_disinfection_acceptance_limits",
    "no_sterile_connection_claim",
    "no_cell_culture_performance_claim",
];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const EDGE_MARGIN: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 14.0;

const BAG_NEST_POS: (f64, f64) = (-355.0, 205.0);
const BAG_NEST_X: f64 = 430.0;
const BAG_NEST_Y: f64 = 210.0;
const BAG_NEST_Z: f64 = 52.0;
const BAG_POCKET_X: f64 = 330.0;
const BAG_POCKET_Y: f64 = 132.0;
const BAG_POCKET_DEPTH: f64 = 20.0;
const BAG_SADDLE_RIBS: usize = 8;
const BAG_LOCATOR_PINS: usize = 6;
const BAG_PORT_GUIDES: usize = 4;

const SPIKE_POS: (f64, f64) = (95.0, 205.0);
const SPIKE_PANEL_X: f64 = 340.0;
const SPIKE_PANEL_Y: f64 = 210.0;
const SPIKE_PANEL_Z: f64 = 48.0;
const SPIKE_ROWS: usize = 2;
const SPIKE_COLS: usize = 4;
const SPIKE_POCKET_COUNT: usize = SPIKE_ROWS * SPIKE_COLS;
const SPIKE_PITCH_X: f64 = 72.0;
const SPIKE_PITCH_Y: f64 = 76.0;
const SPIKE_POCKET_D: f64 = 38.0;
const EXPOSED_WET_FACE_GAUGES: usize = SPIKE_POCKET_COUNT;

const CAP_POS: (f64, f64) = (440.0, 205.0);
const CAP_PANEL_X: f64 = 230.0;
const CAP_PANEL_Y: f64 = 210.0;
const CAP_PANEL_Z: f64 = 40.0;
const CAP_ROWS: usize = 2;
const CAP_COLS: usize = 4;
const CAP_PARKING_COUNT: usize = CAP_ROWS * CAP_COLS;
const CAP_PITCH_X: f64 = 46.0;
const CAP_PITCH_Y: f64 = 68.0;
const CAP_CUP_D: f64 = 28.0;
const LOST_CAP_QUARANTINE_SLOTS: usize = 3;

const SWAB_POS: (f64, f64) = (-400.0, -35.0);
const SWAB_PANEL_X: f64 = 310.0;
const SWAB_PANEL_Y: f64 = 190.0;
const SWAB_PANEL_Z: f64 = 42.0;
const SWAB_ROWS: usize = 2;
const SWAB_COLS: usize = 6;
const SWAB_SLOT_COUNT: usize = SWAB_ROWS * SWAB_COLS;
const SWAB_SLOT_X: f64 = 26.0;
const SWAB_SLOT_Y: f64 = 78.0;
const SWAB_PITCH_X: f64 = 44.0;
const SWAB_PITCH_Y: f64 = 68.0;
const SWAB_VIAL_WELLS: usize = 6;

const COUPON_POS: (f64, f64) = (-65.0, -35.0);
const COUPON_PANEL_X: f64 = 300.0;
const COUPON_PANEL_Y: f64 = 190.0;
const COUPON_PANEL_Z: f64 = 34.0;
const COUPON_LANES: usize = 2;
const COUPONS_PER_LANE: usize = 5;
const COUPON_COUNT: usize = COUPON_LANES * COUPONS_PER_LANE;
const COUPON_SLOT_X: f64 = 42.0;
const COUPON_SLOT_Y: f64 = 56.0;
const COUPON_PITCH_X: f64 = 52.0;
const COUPON_PITCH_Y: f64 = 76.0;
const COMPARISON_TICKS: usize = 9;

const WELL_POS: (f64, f64) = (285.0, -35.0);
const WELL_BLOCK_X: f64 = 320.0;
const WELL_BLOCK_Y: f64 = 190.0;
const WELL_BLOCK_Z: f64 = 56.0;
const RESIDUE_WELLS: usize = 5;
const RINSE_WELLS: usize = 5;
const COLLECTION_WELLS: usize = RESIDUE_WELLS + RINSE_WELLS;
const WELL_D: f64 = 32.0;
const WELL_PITCH_X: f64 = 52.0;
const WELL_PITCH_Y: f64 = 72.0;
const WELL_DEPTH: f64 = 42.0;
const VOLUME_TICKS_PER_WELL: usize = 4;

const CUSTODY_POS: (f64, f64) = (-405.0, -270.0);
const CUSTODY_PANEL_X: f64 = 310.0;
const CUSTODY_PANEL_Y: f64 = 150.0;
const CUSTODY_PANEL_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 8;
const CUSTODY_CARD_SLOTS: usize = 4;
const CUSTODY_TOKEN_SLOTS: usize = 6;

const SEG_POS: (f64, f64) = (-55.0, -270.0);
const SEG_PANEL_X: f64 = 330.0;
const SEG_PANEL_Y: f64 = 150.0;
const SEG_PANEL_Z: f64 = 62.0;
const CLEAN_USED_WELLS_PER_SIDE: usize = 6;
const CLEAN_USED_DIVIDER_Z: f64 = 118.0;
const CLEAN_USED_AIR_GAP: f64 = 54.0;

const CAMERA_POS: (f64, f64) = (350.0, -260.0);
const CAMERA_BRIDGE_X: f64 = 360.0;
const CAMERA_BRIDGE_Y: f64 = 170.0;
const CAMERA_BRIDGE_Z: f64 = 188.0;
const CAMERA_POST_W: f64 = 24.0;
const CAMERA_BEAM_Z: f64 = 34.0;
const CAMERA_PORTS: usize = 3;
const LIGHT_BAR_COUNT: usize = 2;
const FOCUS_TARGETS: usize = 5;
const CAMERA_CLEARANCE_Z: f64 = 150.0;

const KEEP_OUT_X: f64 = 1160.0;
const KEEP_OUT_Y: f64 = 760.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;
const ROBOT_APPROACH_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 250.0;
const BAG_LOAD_CLEARANCE: f64 = 230.0;
const SWAB_PICK_CLEARANCE: f64 = 180.0;
const CAMERA_OVERHEAD_CLEARANCE: f64 = 215.0;
const KEEP_OUT_ZONE_COUNT: usize = 6;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = STATION_X / 2.0 - RIM_W - EDGE_MARGIN;
        let half_y = STATION_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let bag_nest = media_bag_nest();
    export(OUTPUTS[1], &bag_nest);

    let spike_pockets = spike_port_witness_pockets();
    export(OUTPUTS[2], &spike_pockets);

    let swabs = swab_recovery_slots();
    export(OUTPUTS[3], &swabs);

    let caps = connector_cap_parking();
    export(OUTPUTS[4], &caps);

    let coupons = wet_dry_comparison_coupons();
    export(OUTPUTS[5], &coupons);

    let wells = residue_rinse_collection_wells();
    export(OUTPUTS[6], &wells);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let segregation = clean_used_segregation_gate();
    export(OUTPUTS[8], &segregation);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + bag_nest.translate(BAG_NEST_POS.0, BAG_NEST_POS.1, on_deck_z(BAG_NEST_Z))
        + spike_pockets.translate(SPIKE_POS.0, SPIKE_POS.1, on_deck_z(SPIKE_PANEL_Z))
        + caps.translate(CAP_POS.0, CAP_POS.1, on_deck_z(CAP_PANEL_Z))
        + swabs.translate(SWAB_POS.0, SWAB_POS.1, on_deck_z(SWAB_PANEL_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_PANEL_Z))
        + wells.translate(WELL_POS.0, WELL_POS.1, on_deck_z(WELL_BLOCK_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_PANEL_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, on_deck_z(SEG_PANEL_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0 + 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media-bag spike-port swab residue station:");
    println!("  Footprint:            {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!(
        "  Bag/port custody:     media bag nest with {BAG_SADDLE_RIBS} saddle ribs, {BAG_LOCATOR_PINS} locator pins, {BAG_PORT_GUIDES} port guides, {SPIKE_POCKET_COUNT} spike-port witness pockets"
    );
    println!(
        "  Residue recovery:     {SWAB_SLOT_COUNT} swab recovery slots, {SWAB_VIAL_WELLS} vial wells, {COUPON_COUNT} wet/dry coupons, {COLLECTION_WELLS} residue/rinse wells"
    );
    println!(
        "  Cap/segregation:      {CAP_PARKING_COUNT} cap parking cups, {LOST_CAP_QUARANTINE_SLOTS} lost-cap quarantine slots, {CLEAN_USED_WELLS_PER_SIDE} clean and {CLEAN_USED_WELLS_PER_SIDE} used segregation wells"
    );
    println!(
        "  Trace/evidence:       {BARCODE_LANDS} barcode lands, {CUSTODY_CARD_SLOTS} custody card slots, {CAMERA_PORTS} camera ports, {LIGHT_BAR_COUNT} light bars, {FOCUS_TARGETS} focus targets"
    );
    println!(
        "  Keepouts:             front robot {ROBOT_APPROACH_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, bag load {BAG_LOAD_CLEARANCE:.0}mm, swab pick {SWAB_PICK_CLEARANCE:.0}mm, camera overhead {CAMERA_OVERHEAD_CLEARANCE:.0}mm"
    );
    println!(
        "  Limitations:          mechanical validation fixture only; no protocol, acceptance limit, sterile connection claim, or cell-culture performance claim"
    );
    println!("  Output prefix:        {OUTPUT_PREFIX}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn name(suffix: &str) -> String {
    format!("{BIN_NAME}_{suffix}")
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_xy(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;

    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 9] {
    [
        rect("media_bag_nest", BAG_NEST_POS, BAG_NEST_X, BAG_NEST_Y),
        rect(
            "spike_port_witness_pockets",
            SPIKE_POS,
            SPIKE_PANEL_X,
            SPIKE_PANEL_Y,
        ),
        rect("connector_cap_parking", CAP_POS, CAP_PANEL_X, CAP_PANEL_Y),
        rect("swab_recovery_slots", SWAB_POS, SWAB_PANEL_X, SWAB_PANEL_Y),
        rect(
            "wet_dry_comparison_coupons",
            COUPON_POS,
            COUPON_PANEL_X,
            COUPON_PANEL_Y,
        ),
        rect(
            "residue_rinse_collection_wells",
            WELL_POS,
            WELL_BLOCK_X,
            WELL_BLOCK_Y,
        ),
        rect(
            "barcode_custody_lands",
            CUSTODY_POS,
            CUSTODY_PANEL_X,
            CUSTODY_PANEL_Y,
        ),
        rect(
            "clean_used_segregation_gate",
            SEG_POS,
            SEG_PANEL_X,
            SEG_PANEL_Y,
        ),
        rect(
            "evidence_camera_bridge",
            CAMERA_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for feature in [
        "media_bag_nest",
        "spike_port_witness_pockets",
        "swab_recovery_slots",
        "connector_cap_parking",
        "wet_dry_comparison_coupons",
        "residue_rinse_collection_wells",
        "barcode_custody_lands",
        "clean_used_segregation",
        "evidence_camera_bridge",
        "robot_service_keepouts",
        "exposed_wet_surface_witnessing",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "mechanical_validation_fixture_only",
        "no_cleaning_protocol",
        "no_disinfection_acceptance_limits",
        "no_sterile_connection_claim",
        "no_cell_culture_performance_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(SPIKE_POCKET_COUNT, SPIKE_ROWS * SPIKE_COLS);
    assert_eq!(EXPOSED_WET_FACE_GAUGES, SPIKE_POCKET_COUNT);
    assert_eq!(CAP_PARKING_COUNT, CAP_ROWS * CAP_COLS);
    assert_eq!(SWAB_SLOT_COUNT, SWAB_ROWS * SWAB_COLS);
    assert_eq!(COUPON_COUNT, COUPON_LANES * COUPONS_PER_LANE);
    assert_eq!(COLLECTION_WELLS, RESIDUE_WELLS + RINSE_WELLS);
    assert!(BAG_POCKET_DEPTH < BAG_NEST_Z);
    assert!(WELL_DEPTH < WELL_BLOCK_Z);
    assert!(CLEAN_USED_AIR_GAP >= 50.0);
    assert!(CLEAN_USED_DIVIDER_Z > SEG_PANEL_Z);
    assert!(CAMERA_CLEARANCE_Z > CLEAN_USED_DIVIDER_Z);
    assert!(ROBOT_APPROACH_CLEARANCE >= 400.0);
    assert!(REAR_SERVICE_CLEARANCE >= 240.0);
    assert!(BAG_LOAD_CLEARANCE >= 220.0);
    assert!(SWAB_PICK_CLEARANCE >= 170.0);

    let rects = module_rects();
    for module in rects {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds tray usable envelope",
            module.name
        );
    }

    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(name("base_leak_tray_deck"), STATION_X, STATION_Y, BASE_Z).translate(
        0.0,
        0.0,
        BASE_Z / 2.0,
    );
    let containment_pan = centered_cube(
        name("base_secondary_containment_pan_cut"),
        STATION_X - 2.0 * (RIM_W + 46.0),
        STATION_Y - 2.0 * (RIM_W + 48.0),
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z - 4.0);
    let front_gutter = centered_cube(
        name("base_front_residue_rinse_gutter_cut"),
        STATION_X - 180.0,
        24.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 66.0, BASE_Z - 4.5);
    let drain = centered_cylinder(
        name("base_low_point_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -STATION_Y / 2.0 + 42.0,
        BASE_Z - 8.0,
    );

    deck - containment_pan - front_gutter - drain - module_sockets() - mount_holes()
        + perimeter_rim()
        + workflow_dividers()
        + base_zone_label_lands()
        + base_datum_targets()
        + leak_sensor_witness_lands()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty(name("base_module_socket_cuts"));
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("base_socket_{}", module.name)),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(name("base_mount_hole_cuts"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                name(&format!("base_m6_clearance_hole_{i}")),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(name("base_front_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("base_rear_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(name("base_left_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(name("base_right_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn workflow_dividers() -> Part {
    let upper_row_divider = centered_cube(
        name("base_bag_port_row_to_recovery_row_divider"),
        STATION_X - 190.0,
        10.0,
        24.0,
    )
    .translate(0.0, 83.0, BASE_Z + 12.0);
    let lower_row_divider = centered_cube(
        name("base_recovery_row_to_custody_row_divider"),
        STATION_X - 220.0,
        10.0,
        24.0,
    )
    .translate(0.0, -160.0, BASE_Z + 12.0);
    let clean_used_vertical = centered_cube(
        name("base_clean_used_lane_boundary_marker"),
        10.0,
        150.0,
        28.0,
    )
    .translate(-230.0, -270.0, BASE_Z + 14.0);
    let evidence_boundary = centered_cube(
        name("base_custody_to_camera_boundary_marker"),
        10.0,
        150.0,
        28.0,
    )
    .translate(140.0, -270.0, BASE_Z + 14.0);

    upper_row_divider + lower_row_divider + clean_used_vertical + evidence_boundary
}

fn base_zone_label_lands() -> Part {
    raised_label_land("base_media_bag_zone_land", 178.0, 24.0, 5).translate(
        BAG_NEST_POS.0,
        BAG_NEST_POS.1 + BAG_NEST_Y / 2.0 - 20.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_spike_witness_zone_land", 178.0, 24.0, 6).translate(
        SPIKE_POS.0,
        SPIKE_POS.1 + SPIKE_PANEL_Y / 2.0 - 20.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_swab_recovery_zone_land", 178.0, 24.0, 5).translate(
        SWAB_POS.0,
        SWAB_POS.1 - SWAB_PANEL_Y / 2.0 + 20.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_rinse_residue_zone_land", 178.0, 24.0, 6).translate(
        WELL_POS.0,
        WELL_POS.1 - WELL_BLOCK_Y / 2.0 + 20.0,
        BASE_Z + 2.0,
    )
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty(name("base_robot_datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 82.0, -STATION_Y / 2.0 + 82.0),
        (STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 + 82.0),
        (-STATION_X / 2.0 + 82.0, STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 82.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + datum_disc(&format!("base_robot_datum_target_{i}")).translate(*x, *y, BASE_Z + 2.0);
    }
    targets
}

fn leak_sensor_witness_lands() -> Part {
    let mut lands = Part::empty(name("base_leak_sensor_witness_lands"));
    for (i, x) in centered_row(6, 172.0).iter().enumerate() {
        let land = centered_cube(name(&format!("base_leak_sensor_land_{i}")), 44.0, 30.0, 6.0)
            .translate(*x, -STATION_Y / 2.0 + 104.0, BASE_Z + 3.0);
        let cup = centered_cylinder(name(&format!("base_leak_sensor_cup_cut_{i}")), 7.5, 8.0, 28)
            .translate(*x, -STATION_Y / 2.0 + 104.0, BASE_Z + 4.0);
        lands = lands + (land - cup);
    }
    lands
}

fn media_bag_nest() -> Part {
    let body = centered_cube(
        name("media_bag_nest_body"),
        BAG_NEST_X,
        BAG_NEST_Y,
        BAG_NEST_Z,
    );
    let bag_pocket = centered_cube(
        name("media_bag_nest_soft_bag_pocket_cut"),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH,
    )
    .translate(0.0, 8.0, BAG_NEST_Z / 2.0 - BAG_POCKET_DEPTH / 2.0 + 0.5);
    let port_throat = centered_cube(
        name("media_bag_nest_spike_port_throat_clearance_cut"),
        96.0,
        54.0,
        BAG_NEST_Z + 6.0,
    )
    .translate(BAG_POCKET_X / 2.0 - 12.0, -BAG_POCKET_Y / 2.0 + 6.0, 0.0);
    let hang_tab_relief = centered_cube(
        name("media_bag_nest_hanger_tab_relief_cut"),
        92.0,
        28.0,
        18.0,
    )
    .translate(
        -BAG_POCKET_X / 2.0 + 34.0,
        BAG_POCKET_Y / 2.0 - 8.0,
        BAG_NEST_Z / 2.0 - 8.0,
    );

    body - bag_pocket - port_throat - hang_tab_relief
        + bag_saddle_ribs()
        + bag_locator_pins()
        + bag_port_guides()
        + bag_edge_witness_lips()
}

fn bag_saddle_ribs() -> Part {
    let mut ribs = Part::empty(name("media_bag_nest_saddle_ribs"));
    for i in 0..BAG_SADDLE_RIBS {
        let x = centered_index(i, BAG_SADDLE_RIBS, 42.0);
        ribs = ribs
            + centered_cube(
                name(&format!("media_bag_nest_low_residue_saddle_rib_{i}")),
                6.0,
                BAG_POCKET_Y - 16.0,
                8.0,
            )
            .translate(x, 8.0, BAG_NEST_Z / 2.0 + 4.0);
    }
    ribs
}

fn bag_locator_pins() -> Part {
    let mut pins = Part::empty(name("media_bag_nest_locator_pins"));
    for (i, (x, y)) in [
        (-170.0, 78.0),
        (0.0, 82.0),
        (170.0, 78.0),
        (-170.0, -58.0),
        (0.0, -64.0),
        (170.0, -58.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                name(&format!("media_bag_nest_soft_locator_pin_{i}")),
                6.0,
                12.0,
                28,
            )
            .translate(*x, *y, BAG_NEST_Z / 2.0 + 6.0);
    }
    pins
}

fn bag_port_guides() -> Part {
    let mut guides = Part::empty(name("media_bag_nest_spike_port_guides"));
    for i in 0..BAG_PORT_GUIDES {
        let y = centered_index(i, BAG_PORT_GUIDES, 28.0) - 28.0;
        guides = guides
            + centered_cube(
                name(&format!("media_bag_nest_port_clocking_guide_{i}")),
                58.0,
                8.0,
                16.0,
            )
            .translate(BAG_POCKET_X / 2.0 + 14.0, y, BAG_NEST_Z / 2.0 + 8.0)
            + centered_cylinder(
                name(&format!("media_bag_nest_port_tip_no_contact_gauge_{i}")),
                5.0,
                52.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(BAG_POCKET_X / 2.0 + 14.0, y + 10.0, BAG_NEST_Z / 2.0 + 18.0);
    }
    guides
}

fn bag_edge_witness_lips() -> Part {
    let rear_lip = centered_cube(
        name("media_bag_nest_dry_outer_edge_witness_lip"),
        BAG_POCKET_X + 34.0,
        8.0,
        10.0,
    )
    .translate(0.0, BAG_POCKET_Y / 2.0 + 18.0, BAG_NEST_Z / 2.0 + 5.0);
    let front_lip = centered_cube(
        name("media_bag_nest_wet_port_edge_witness_lip"),
        BAG_POCKET_X + 34.0,
        8.0,
        10.0,
    )
    .translate(0.0, -BAG_POCKET_Y / 2.0 - 4.0, BAG_NEST_Z / 2.0 + 5.0);
    let handle_land = raised_label_land("media_bag_nest_bag_lot_label_land", 96.0, 24.0, 4)
        .translate(
            -BAG_NEST_X / 2.0 + 82.0,
            -BAG_NEST_Y / 2.0 + 28.0,
            BAG_NEST_Z / 2.0 + 2.0,
        );

    rear_lip + front_lip + handle_land
}

fn spike_port_witness_pockets() -> Part {
    let body = centered_cube(
        name("spike_port_witness_pocket_panel_body"),
        SPIKE_PANEL_X,
        SPIKE_PANEL_Y,
        SPIKE_PANEL_Z,
    );
    let recessed_field = centered_cube(
        name("spike_port_witness_recessed_wet_field_cut"),
        SPIKE_PANEL_X - 34.0,
        SPIKE_PANEL_Y - 32.0,
        8.0,
    )
    .translate(0.0, 0.0, SPIKE_PANEL_Z / 2.0 - 4.0);

    body - recessed_field - spike_socket_cuts() - wet_face_slot_cuts()
        + spike_annular_witness_rings()
        + exposed_wet_face_gauges()
        + spike_residue_direction_arrows()
}

fn spike_socket_cuts() -> Part {
    let mut cuts = Part::empty(name("spike_port_witness_socket_cuts"));
    for i in 0..SPIKE_POCKET_COUNT {
        let (x, y) = grid_xy(i, SPIKE_COLS, SPIKE_ROWS, SPIKE_PITCH_X, SPIKE_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                name(&format!("spike_port_witness_connector_socket_cut_{i}")),
                SPIKE_POCKET_D / 2.0,
                SPIKE_PANEL_Z + 8.0,
                36,
            )
            .translate(x, y, 8.0)
            + centered_cube(
                name(&format!("spike_port_witness_swab_access_flat_cut_{i}")),
                44.0,
                12.0,
                18.0,
            )
            .translate(x, y - 22.0, SPIKE_PANEL_Z / 2.0 - 8.0);
    }
    cuts
}

fn wet_face_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("spike_port_witness_exposed_wet_face_slot_cuts"));
    for i in 0..SPIKE_POCKET_COUNT {
        let (x, y) = grid_xy(i, SPIKE_COLS, SPIKE_ROWS, SPIKE_PITCH_X, SPIKE_PITCH_Y);
        cuts = cuts
            + centered_cube(
                name(&format!(
                    "spike_port_witness_residual_wet_face_shadow_cut_{i}"
                )),
                12.0,
                38.0,
                12.0,
            )
            .translate(x + 28.0, y, SPIKE_PANEL_Z / 2.0 - 5.0);
    }
    cuts
}

fn spike_annular_witness_rings() -> Part {
    let mut rings = Part::empty(name("spike_port_witness_annular_witness_rings"));
    for i in 0..SPIKE_POCKET_COUNT {
        let (x, y) = grid_xy(i, SPIKE_COLS, SPIKE_ROWS, SPIKE_PITCH_X, SPIKE_PITCH_Y);
        let ring = centered_cylinder(
            name(&format!("spike_port_witness_annular_residue_ring_{i}")),
            25.0,
            5.0,
            40,
        ) - centered_cylinder(
            name(&format!("spike_port_witness_annular_clear_center_{i}")),
            SPIKE_POCKET_D / 2.0 + 1.0,
            7.0,
            40,
        );
        rings = rings + ring.translate(x, y, SPIKE_PANEL_Z / 2.0 + 2.5);
    }
    rings
}

fn exposed_wet_face_gauges() -> Part {
    let mut gauges = Part::empty(name("spike_port_witness_exposed_wet_face_gauges"));
    for i in 0..EXPOSED_WET_FACE_GAUGES {
        let (x, y) = grid_xy(i, SPIKE_COLS, SPIKE_ROWS, SPIKE_PITCH_X, SPIKE_PITCH_Y);
        gauges = gauges
            + centered_cube(
                name(&format!("spike_port_witness_wet_face_gauge_fin_{i}")),
                4.0,
                44.0,
                28.0,
            )
            .translate(x + 38.0, y, SPIKE_PANEL_Z / 2.0 + 14.0)
            + centered_cube(
                name(&format!("spike_port_witness_dry_backstop_gauge_{i}")),
                38.0,
                4.0,
                16.0,
            )
            .translate(x, y + 31.0, SPIKE_PANEL_Z / 2.0 + 8.0);
    }
    gauges
}

fn spike_residue_direction_arrows() -> Part {
    let mut arrows = Part::empty(name("spike_port_witness_residue_direction_arrows"));
    for i in 0..SPIKE_COLS {
        let x = centered_index(i, SPIKE_COLS, SPIKE_PITCH_X);
        arrows = arrows
            + centered_cube(
                name(&format!("spike_port_witness_flow_arrow_bar_{i}")),
                36.0,
                5.0,
                4.0,
            )
            .translate(x, -SPIKE_PANEL_Y / 2.0 + 20.0, SPIKE_PANEL_Z / 2.0 + 2.0)
            + centered_cylinder(
                name(&format!("spike_port_witness_flow_arrow_dot_{i}")),
                4.0,
                4.0,
                18,
            )
            .translate(
                x + 22.0,
                -SPIKE_PANEL_Y / 2.0 + 20.0,
                SPIKE_PANEL_Z / 2.0 + 2.0,
            );
    }
    arrows
}

fn swab_recovery_slots() -> Part {
    let body = centered_cube(
        name("swab_recovery_slot_panel_body"),
        SWAB_PANEL_X,
        SWAB_PANEL_Y,
        SWAB_PANEL_Z,
    );
    let field = centered_cube(
        name("swab_recovery_slot_field_recess_cut"),
        SWAB_PANEL_X - 24.0,
        SWAB_PANEL_Y - 28.0,
        7.0,
    )
    .translate(0.0, 0.0, SWAB_PANEL_Z / 2.0 - 3.5);

    body - field - swab_slot_cuts() - swab_vial_well_cuts()
        + swab_retainer_teeth()
        + swab_lane_label_lands()
}

fn swab_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("swab_recovery_slot_cuts"));
    for i in 0..SWAB_SLOT_COUNT {
        let (x, y) = grid_xy(i, SWAB_COLS, SWAB_ROWS, SWAB_PITCH_X, SWAB_PITCH_Y);
        let angle = if i % 2 == 0 { 7.0 } else { -7.0 };
        cuts = cuts
            + centered_cube(
                name(&format!("swab_recovery_absorbent_tip_slot_cut_{i}")),
                SWAB_SLOT_X,
                SWAB_SLOT_Y,
                22.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(x, y, SWAB_PANEL_Z / 2.0 - 8.0);
    }
    cuts
}

fn swab_vial_well_cuts() -> Part {
    let mut cuts = Part::empty(name("swab_recovery_sample_vial_well_cuts"));
    for i in 0..SWAB_VIAL_WELLS {
        let x = centered_index(i, SWAB_VIAL_WELLS, 44.0);
        cuts = cuts
            + centered_cylinder(
                name(&format!("swab_recovery_vial_well_cut_{i}")),
                8.5,
                28.0,
                30,
            )
            .translate(x, -SWAB_PANEL_Y / 2.0 + 24.0, SWAB_PANEL_Z / 2.0 - 10.0);
    }
    cuts
}

fn swab_retainer_teeth() -> Part {
    let mut teeth = Part::empty(name("swab_recovery_retainer_teeth"));
    for i in 0..=SWAB_COLS {
        let x = centered_index(i, SWAB_COLS + 1, SWAB_PITCH_X);
        teeth = teeth
            + centered_cube(
                name(&format!("swab_recovery_retainer_tooth_{i}")),
                4.0,
                SWAB_PANEL_Y - 54.0,
                18.0,
            )
            .translate(x, 16.0, SWAB_PANEL_Z / 2.0 + 9.0);
    }
    teeth
}

fn swab_lane_label_lands() -> Part {
    raised_label_land("swab_recovery_pre_swab_label_land", 116.0, 22.0, 5).translate(
        -SWAB_PANEL_X / 2.0 + 74.0,
        SWAB_PANEL_Y / 2.0 - 18.0,
        SWAB_PANEL_Z / 2.0 + 2.0,
    ) + raised_label_land("swab_recovery_post_swab_label_land", 116.0, 22.0, 5).translate(
        SWAB_PANEL_X / 2.0 - 74.0,
        SWAB_PANEL_Y / 2.0 - 18.0,
        SWAB_PANEL_Z / 2.0 + 2.0,
    )
}

fn connector_cap_parking() -> Part {
    let body = centered_cube(
        name("connector_cap_parking_panel_body"),
        CAP_PANEL_X,
        CAP_PANEL_Y,
        CAP_PANEL_Z,
    );
    let tray_recess = centered_cube(
        name("connector_cap_parking_recess_cut"),
        CAP_PANEL_X - 24.0,
        CAP_PANEL_Y - 28.0,
        8.0,
    )
    .translate(0.0, 0.0, CAP_PANEL_Z / 2.0 - 4.0);

    body - tray_recess - cap_cup_cuts() - cap_tether_slot_cuts()
        + cap_custody_pin_lands()
        + lost_cap_quarantine_slots()
}

fn cap_cup_cuts() -> Part {
    let mut cuts = Part::empty(name("connector_cap_parking_cup_cuts"));
    for i in 0..CAP_PARKING_COUNT {
        let (x, y) = grid_xy(i, CAP_COLS, CAP_ROWS, CAP_PITCH_X, CAP_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                name(&format!("connector_cap_parking_cup_cut_{i}")),
                CAP_CUP_D / 2.0,
                26.0,
                30,
            )
            .translate(x, y, CAP_PANEL_Z / 2.0 - 10.0)
            + centered_cube(
                name(&format!("connector_cap_parking_key_flat_cut_{i}")),
                10.0,
                CAP_CUP_D,
                14.0,
            )
            .translate(x + CAP_CUP_D / 2.0, y, CAP_PANEL_Z / 2.0 - 8.0);
    }
    cuts
}

fn cap_tether_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("connector_cap_parking_tether_slot_cuts"));
    for i in 0..CAP_ROWS {
        let y = centered_index(i, CAP_ROWS, CAP_PITCH_Y);
        cuts = cuts
            + centered_cube(
                name(&format!("connector_cap_parking_tether_slot_cut_{i}")),
                CAP_PANEL_X - 44.0,
                7.0,
                12.0,
            )
            .translate(0.0, y + 26.0, CAP_PANEL_Z / 2.0 - 6.0);
    }
    cuts
}

fn cap_custody_pin_lands() -> Part {
    let mut pins = Part::empty(name("connector_cap_parking_custody_pin_lands"));
    for i in 0..CAP_PARKING_COUNT {
        let (x, y) = grid_xy(i, CAP_COLS, CAP_ROWS, CAP_PITCH_X, CAP_PITCH_Y);
        pins = pins
            + centered_cylinder(
                name(&format!("connector_cap_parking_presence_pin_{i}")),
                3.5,
                8.0,
                18,
            )
            .translate(x - 18.0, y - 20.0, CAP_PANEL_Z / 2.0 + 4.0);
    }
    pins
}

fn lost_cap_quarantine_slots() -> Part {
    let mut slots = Part::empty(name("connector_cap_parking_lost_cap_quarantine_slots"));
    for i in 0..LOST_CAP_QUARANTINE_SLOTS {
        let x = centered_index(i, LOST_CAP_QUARANTINE_SLOTS, 52.0);
        let bin = centered_cube(
            name(&format!("connector_cap_parking_quarantine_slot_{i}")),
            38.0,
            24.0,
            10.0,
        )
        .translate(x, -CAP_PANEL_Y / 2.0 + 22.0, CAP_PANEL_Z / 2.0 + 5.0);
        let cut = centered_cube(
            name(&format!("connector_cap_parking_quarantine_slot_cut_{i}")),
            28.0,
            14.0,
            11.0,
        )
        .translate(x, -CAP_PANEL_Y / 2.0 + 22.0, CAP_PANEL_Z / 2.0 + 5.5);
        slots = slots + (bin - cut);
    }
    slots
}

fn wet_dry_comparison_coupons() -> Part {
    let body = centered_cube(
        name("wet_dry_comparison_coupon_panel_body"),
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );
    let lane_recess = centered_cube(
        name("wet_dry_comparison_coupon_recess_cut"),
        COUPON_PANEL_X - 26.0,
        COUPON_PANEL_Y - 28.0,
        7.0,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z / 2.0 - 3.5);

    body - lane_recess - coupon_slot_cuts()
        + coupon_lane_divider()
        + coupon_retainer_tabs()
        + coupon_comparison_scale()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("wet_dry_comparison_coupon_slot_cuts"));
    for lane in 0..COUPON_LANES {
        let y = centered_index(lane, COUPON_LANES, COUPON_PITCH_Y);
        for coupon in 0..COUPONS_PER_LANE {
            let x = centered_index(coupon, COUPONS_PER_LANE, COUPON_PITCH_X);
            cuts = cuts
                + centered_cube(
                    name(&format!(
                        "wet_dry_comparison_coupon_lane_{lane}_slot_{coupon}_cut"
                    )),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    12.0,
                )
                .translate(x, y, COUPON_PANEL_Z / 2.0 - 5.0);
        }
    }
    cuts
}

fn coupon_lane_divider() -> Part {
    centered_cube(
        name("wet_dry_comparison_coupon_clean_wet_dry_lane_divider"),
        COUPON_PANEL_X - 28.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z / 2.0 + 12.0)
}

fn coupon_retainer_tabs() -> Part {
    let mut tabs = Part::empty(name("wet_dry_comparison_coupon_retainer_tabs"));
    for lane in 0..COUPON_LANES {
        let y = centered_index(lane, COUPON_LANES, COUPON_PITCH_Y);
        for coupon in 0..COUPONS_PER_LANE {
            let x = centered_index(coupon, COUPONS_PER_LANE, COUPON_PITCH_X);
            tabs = tabs
                + centered_cube(
                    name(&format!(
                        "wet_dry_comparison_coupon_lane_{lane}_retainer_{coupon}"
                    )),
                    COUPON_SLOT_X + 8.0,
                    5.0,
                    6.0,
                )
                .translate(
                    x,
                    y - COUPON_SLOT_Y / 2.0 - 6.0,
                    COUPON_PANEL_Z / 2.0 + 3.0,
                );
        }
    }
    tabs
}

fn coupon_comparison_scale() -> Part {
    let mut scale = Part::empty(name("wet_dry_comparison_coupon_residue_scale"));
    for i in 0..COMPARISON_TICKS {
        let x = centered_index(i, COMPARISON_TICKS, 28.0);
        let height = 5.0 + i as f64 * 1.5;
        scale = scale
            + centered_cube(
                name(&format!("wet_dry_comparison_coupon_residue_scale_tick_{i}")),
                4.0,
                16.0,
                height,
            )
            .translate(
                x,
                COUPON_PANEL_Y / 2.0 - 18.0,
                COUPON_PANEL_Z / 2.0 + height / 2.0,
            );
    }
    scale
}

fn residue_rinse_collection_wells() -> Part {
    let body = centered_cube(
        name("residue_rinse_collection_well_block_body"),
        WELL_BLOCK_X,
        WELL_BLOCK_Y,
        WELL_BLOCK_Z,
    );
    let spill_slope = centered_cube(
        name("residue_rinse_collection_spill_slope_recess_cut"),
        WELL_BLOCK_X - 30.0,
        WELL_BLOCK_Y - 26.0,
        8.0,
    )
    .translate(0.0, 0.0, WELL_BLOCK_Z / 2.0 - 4.0);

    body - spill_slope - collection_well_cuts() - collection_drain_channel_cuts()
        + collection_volume_ticks()
        + collection_cup_id_lands()
}

fn collection_well_cuts() -> Part {
    let mut cuts = Part::empty(name("residue_rinse_collection_well_cuts"));
    for row in 0..2 {
        let y = centered_index(row, 2, WELL_PITCH_Y);
        for well in 0..RESIDUE_WELLS {
            let x = centered_index(well, RESIDUE_WELLS, WELL_PITCH_X);
            cuts = cuts
                + centered_cylinder(
                    name(&format!(
                        "residue_rinse_collection_row_{row}_well_{well}_cut"
                    )),
                    WELL_D / 2.0,
                    WELL_DEPTH,
                    36,
                )
                .translate(x, y, WELL_BLOCK_Z / 2.0 - WELL_DEPTH / 2.0 + 1.0);
        }
    }
    cuts
}

fn collection_drain_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("residue_rinse_collection_drain_channel_cuts"));
    for row in 0..2 {
        let y = centered_index(row, 2, WELL_PITCH_Y);
        cuts = cuts
            + centered_cube(
                name(&format!(
                    "residue_rinse_collection_row_{row}_drain_channel_cut"
                )),
                WELL_BLOCK_X - 66.0,
                7.0,
                12.0,
            )
            .translate(0.0, y - 27.0, WELL_BLOCK_Z / 2.0 - 6.0);
    }
    cuts
}

fn collection_volume_ticks() -> Part {
    let mut ticks = Part::empty(name("residue_rinse_collection_volume_ticks"));
    for row in 0..2 {
        let y = centered_index(row, 2, WELL_PITCH_Y);
        for well in 0..RESIDUE_WELLS {
            let x = centered_index(well, RESIDUE_WELLS, WELL_PITCH_X);
            for tick in 0..VOLUME_TICKS_PER_WELL {
                ticks = ticks
                    + centered_cube(
                        name(&format!(
                            "residue_rinse_collection_row_{row}_well_{well}_volume_tick_{tick}"
                        )),
                        9.0,
                        2.5,
                        2.5,
                    )
                    .translate(
                        x + WELL_D / 2.0 + 6.0,
                        y - 12.0 + tick as f64 * 8.0,
                        WELL_BLOCK_Z / 2.0 + 1.25,
                    );
            }
        }
    }
    ticks
}

fn collection_cup_id_lands() -> Part {
    raised_label_land("residue_collection_wells_label_land", 104.0, 20.0, 5).translate(
        -WELL_BLOCK_X / 2.0 + 72.0,
        WELL_BLOCK_Y / 2.0 - 18.0,
        WELL_BLOCK_Z / 2.0 + 2.0,
    ) + raised_label_land("rinse_collection_wells_label_land", 104.0, 20.0, 5).translate(
        WELL_BLOCK_X / 2.0 - 72.0,
        WELL_BLOCK_Y / 2.0 - 18.0,
        WELL_BLOCK_Z / 2.0 + 2.0,
    )
}

fn barcode_custody_lands() -> Part {
    let body = centered_cube(
        name("barcode_custody_land_panel_body"),
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    body + barcode_land_array() + custody_card_slot_lands() + custody_token_rail()
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty(name("barcode_custody_barcode_land_array"));
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 68.0);
        let y = centered_index(i / 4, 2, 48.0) + 22.0;
        lands = lands
            + barcode_land(&format!("barcode_custody_scan_land_{i}"), 54.0, 24.0, 7).translate(
                x,
                y,
                CUSTODY_PANEL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_card_slot_lands() -> Part {
    let mut lands = Part::empty(name("barcode_custody_card_slot_lands"));
    for i in 0..CUSTODY_CARD_SLOTS {
        let x = centered_index(i, CUSTODY_CARD_SLOTS, 72.0);
        let land = centered_cube(
            name(&format!("barcode_custody_card_slot_land_{i}")),
            58.0,
            22.0,
            8.0,
        )
        .translate(
            x,
            -CUSTODY_PANEL_Y / 2.0 + 26.0,
            CUSTODY_PANEL_Z / 2.0 + 4.0,
        );
        let slot = centered_cube(
            name(&format!("barcode_custody_card_slot_recess_{i}")),
            48.0,
            6.0,
            9.0,
        )
        .translate(
            x,
            -CUSTODY_PANEL_Y / 2.0 + 26.0,
            CUSTODY_PANEL_Z / 2.0 + 4.5,
        );
        lands = lands + (land - slot);
    }
    lands
}

fn custody_token_rail() -> Part {
    let rail = centered_cube(
        name("barcode_custody_chain_of_custody_token_rail"),
        CUSTODY_PANEL_X - 36.0,
        10.0,
        10.0,
    )
    .translate(0.0, -14.0, CUSTODY_PANEL_Z / 2.0 + 5.0);
    let mut tokens = Part::empty(name("barcode_custody_token_slots"));
    for i in 0..CUSTODY_TOKEN_SLOTS {
        let x = centered_index(i, CUSTODY_TOKEN_SLOTS, 42.0);
        tokens = tokens
            + centered_cube(
                name(&format!("barcode_custody_token_slot_{i}")),
                26.0,
                18.0,
                6.0,
            )
            .translate(x, -14.0, CUSTODY_PANEL_Z / 2.0 + 8.0);
    }
    rail + tokens
}

fn clean_used_segregation_gate() -> Part {
    let body = centered_cube(
        name("clean_used_segregation_gate_body"),
        SEG_PANEL_X,
        SEG_PANEL_Y,
        SEG_PANEL_Z,
    );
    let divider = centered_cube(
        name("clean_used_segregation_tall_center_divider"),
        12.0,
        SEG_PANEL_Y - 18.0,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(0.0, 0.0, CLEAN_USED_DIVIDER_Z / 2.0 - SEG_PANEL_Z / 2.0);

    body - clean_used_well_cuts() + divider + clean_used_one_way_gate_teeth() + clean_used_labels()
}

fn clean_used_well_cuts() -> Part {
    let mut cuts = Part::empty(name("clean_used_segregation_well_cuts"));
    for side in 0..2 {
        let x_base = if side == 0 { -82.0 } else { 82.0 };
        for i in 0..CLEAN_USED_WELLS_PER_SIDE {
            let col = i % 3;
            let row = i / 3;
            let x = x_base + centered_index(col, 3, 42.0);
            let y = centered_index(row, 2, 48.0);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("clean_used_segregation_side_{side}_well_{i}_cut")),
                    13.0,
                    38.0,
                    30,
                )
                .translate(x, y, SEG_PANEL_Z / 2.0 - 15.0);
        }
    }
    cuts
}

fn clean_used_one_way_gate_teeth() -> Part {
    let mut teeth = Part::empty(name("clean_used_segregation_one_way_gate_teeth"));
    for i in 0..7 {
        let y = centered_index(i, 7, 18.0);
        teeth = teeth
            + centered_cube(
                name(&format!("clean_used_segregation_one_way_gate_tooth_{i}")),
                20.0,
                4.0,
                16.0,
            )
            .translate(18.0, y, SEG_PANEL_Z / 2.0 + 8.0);
    }
    teeth
}

fn clean_used_labels() -> Part {
    raised_label_land("clean_used_segregation_clean_label_land", 96.0, 22.0, 5).translate(
        -92.0,
        SEG_PANEL_Y / 2.0 - 18.0,
        SEG_PANEL_Z / 2.0 + 2.0,
    ) + raised_label_land("clean_used_segregation_used_label_land", 96.0, 22.0, 5).translate(
        92.0,
        SEG_PANEL_Y / 2.0 - 18.0,
        SEG_PANEL_Z / 2.0 + 2.0,
    )
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        name("evidence_camera_bridge_left_post"),
        CAMERA_POST_W,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + CAMERA_POST_W / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        name("evidence_camera_bridge_right_post"),
        CAMERA_POST_W,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - CAMERA_POST_W / 2.0, 0.0, 0.0);
    let rear_crossbeam = centered_cube(
        name("evidence_camera_bridge_rear_crossbeam"),
        CAMERA_BRIDGE_X,
        24.0,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_Y / 2.0 - 12.0,
        CAMERA_BRIDGE_Z / 2.0 - CAMERA_BEAM_Z / 2.0,
    );
    let front_light_beam = centered_cube(
        name("evidence_camera_bridge_front_light_beam"),
        CAMERA_BRIDGE_X,
        18.0,
        26.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 + 14.0,
        CAMERA_BRIDGE_Z / 2.0 - 42.0,
    );
    let throat_clearance = centered_cube(
        name("evidence_camera_bridge_center_access_throat_cut"),
        CAMERA_BRIDGE_X - 86.0,
        CAMERA_BRIDGE_Y - 66.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -10.0,
        -CAMERA_BRIDGE_Z / 2.0 + CAMERA_CLEARANCE_Z / 2.0,
    );

    left_post + right_post + rear_crossbeam + front_light_beam - throat_clearance
        + camera_port_rings()
        + camera_light_bars()
        + camera_focus_targets()
}

fn camera_port_rings() -> Part {
    let mut ports = Part::empty(name("evidence_camera_bridge_camera_port_rings"));
    for i in 0..CAMERA_PORTS {
        let x = centered_index(i, CAMERA_PORTS, 82.0);
        let ring = centered_cylinder(
            name(&format!("evidence_camera_bridge_camera_port_ring_{i}")),
            18.0,
            6.0,
            34,
        ) - centered_cylinder(
            name(&format!("evidence_camera_bridge_camera_port_clearance_{i}")),
            11.0,
            8.0,
            34,
        );
        ports = ports
            + ring.rotate(90.0, 0.0, 0.0).translate(
                x,
                CAMERA_BRIDGE_Y / 2.0 - 27.0,
                CAMERA_BRIDGE_Z / 2.0 - CAMERA_BEAM_Z / 2.0,
            );
    }
    ports
}

fn camera_light_bars() -> Part {
    let mut bars = Part::empty(name("evidence_camera_bridge_light_bars"));
    for i in 0..LIGHT_BAR_COUNT {
        let y = if i == 0 {
            -CAMERA_BRIDGE_Y / 2.0 + 28.0
        } else {
            CAMERA_BRIDGE_Y / 2.0 - 44.0
        };
        bars = bars
            + centered_cube(
                name(&format!("evidence_camera_bridge_linear_light_bar_{i}")),
                CAMERA_BRIDGE_X - 84.0,
                6.0,
                8.0,
            )
            .translate(0.0, y, CAMERA_BRIDGE_Z / 2.0 - 66.0);
    }
    bars
}

fn camera_focus_targets() -> Part {
    let mut targets = Part::empty(name("evidence_camera_bridge_focus_targets"));
    for i in 0..FOCUS_TARGETS {
        let x = centered_index(i, FOCUS_TARGETS, 54.0);
        targets = targets
            + datum_disc(&format!("evidence_camera_bridge_focus_target_{i}")).translate(
                x,
                -CAMERA_BRIDGE_Y / 2.0 + 38.0,
                -CAMERA_BRIDGE_Z / 2.0 + 4.0,
            );
    }
    targets
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        name("robot_service_keepout_front_robot_approach_gauge"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        name("robot_service_keepout_rear_service_gauge"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        name("robot_service_keepout_bag_load_side_gauge"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        name("robot_service_keepout_swab_pick_side_gauge"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);

    front + rear + left + right + keepout_vertical_posts() + keepout_zone_tags()
}

fn keepout_vertical_posts() -> Part {
    let mut posts = Part::empty(name("robot_service_keepout_vertical_posts"));
    for (i, (x, y, height)) in [
        (
            -KEEP_OUT_X / 2.0 + 72.0,
            -KEEP_OUT_Y / 2.0 + 58.0,
            ROBOT_APPROACH_CLEARANCE,
        ),
        (
            KEEP_OUT_X / 2.0 - 72.0,
            -KEEP_OUT_Y / 2.0 + 58.0,
            SWAB_PICK_CLEARANCE,
        ),
        (
            -KEEP_OUT_X / 2.0 + 72.0,
            KEEP_OUT_Y / 2.0 - 58.0,
            BAG_LOAD_CLEARANCE,
        ),
        (
            KEEP_OUT_X / 2.0 - 72.0,
            KEEP_OUT_Y / 2.0 - 58.0,
            REAR_SERVICE_CLEARANCE,
        ),
        (0.0, CAMERA_POS.1, CAMERA_OVERHEAD_CLEARANCE),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                name(&format!("robot_service_keepout_height_post_{i}")),
                7.0,
                *height,
                24,
            )
            .translate(*x, *y, height / 2.0);
    }
    posts
}

fn keepout_zone_tags() -> Part {
    let mut tags = Part::empty(name("robot_service_keepout_zone_tags"));
    for i in 0..KEEP_OUT_ZONE_COUNT {
        let x = centered_index(i, KEEP_OUT_ZONE_COUNT, 96.0);
        tags = tags
            + centered_cube(
                name(&format!("robot_service_keepout_zone_tag_{i}")),
                72.0,
                22.0,
                6.0,
            )
            .translate(x, -KEEP_OUT_Y / 2.0 + 26.0, KEEP_OUT_RAIL_Z / 2.0 + 3.0);
    }
    tags
}

fn raised_label_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut part = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        part = part
            + centered_cube(name(&format!("{id}_raised_bar_{i}")), 3.0, depth - 8.0, 3.0)
                .translate(x, 0.0, 3.5);
    }
    part
}

fn barcode_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut part = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        let bar_w = if i % 2 == 0 { 2.0 } else { 4.0 };
        part = part
            + centered_cube(
                name(&format!("{id}_barcode_bar_{i}")),
                bar_w,
                depth - 7.0,
                3.0,
            )
            .translate(x, 0.0, 3.5);
    }
    part
}

fn datum_disc(id: &str) -> Part {
    let disc = centered_cylinder(name(id), 13.0, 4.0, 30);
    let x_bar =
        centered_cube(name(&format!("{id}_x_bar")), 21.0, 3.0, 2.0).translate(0.0, 0.0, 3.0);
    let y_bar =
        centered_cube(name(&format!("{id}_y_bar")), 3.0, 21.0, 2.0).translate(0.0, 0.0, 3.0);
    disc + x_bar + y_bar
}

fn centered_row(count: usize, pitch: f64) -> Vec<f64> {
    (0..count)
        .map(|index| centered_index(index, count, pitch))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_constraints_hold() {
        assert_design_constraints();
    }

    #[test]
    fn output_manifest_names_are_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_groups_cover_validation_intent() {
        for feature in [
            "media_bag_nest",
            "spike_port_witness_pockets",
            "swab_recovery_slots",
            "connector_cap_parking",
            "wet_dry_comparison_coupons",
            "residue_rinse_collection_wells",
            "barcode_custody_lands",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepouts",
            "exposed_wet_surface_witnessing",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }
}
