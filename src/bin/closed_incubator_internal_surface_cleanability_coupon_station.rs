use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator internal-surface cleanability coupon validation station.
//
// Intent:
// - Represent removable wall, floor, and door internal-surface coupons in
//   repeatable nests for wipe-down challenge/recovery work.
// - Provide wipe path guidance, condensate contact witnesses, camera evidence,
//   barcode trays, clean/used segregation, and release/hold/reject routing.
// - Keep the CAD architectural: interface geometry, traceability lands, and
//   access envelopes only. This is not a validated cleaning method, microbial
//   acceptance criterion, or incubator product definition.

const OUTPUT_PREFIX: &str = "output/closed_incubator_internal_surface_cleanability_coupon_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_internal_surface_cleanability_coupon_station_base_leak_tray.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_removable_wall_coupon_panel.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_removable_floor_coupon_panel.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_removable_door_coupon_panel.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_wipe_path_guide.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_condensate_contact_witness.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_camera_evidence_bridge.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_barcode_coupon_trays.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_clean_used_segregation.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_robot_service_keepouts.stl",
    "output/closed_incubator_internal_surface_cleanability_coupon_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "removable_wall_coupons",
    "removable_floor_coupons",
    "removable_door_coupons",
    "wipe_path_guide",
    "condensate_contact_witness",
    "camera_evidence_bridge",
    "barcode_coupon_trays",
    "clean_used_segregation",
    "released_lane",
    "hold_lane",
    "reject_lane",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_PORT_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const WALL_PANEL_POS: (f64, f64) = (-430.0, 228.0);
const WALL_PANEL_X: f64 = 300.0;
const WALL_PANEL_Y: f64 = 210.0;
const WALL_PANEL_Z: f64 = 42.0;
const WALL_COUPONS: usize = 4;
const WALL_COUPON_X: f64 = 52.0;
const WALL_COUPON_Y: f64 = 7.0;
const WALL_COUPON_Z: f64 = 82.0;
const WALL_COUPON_PITCH_X: f64 = 64.0;

const FLOOR_PANEL_POS: (f64, f64) = (-80.0, 228.0);
const FLOOR_PANEL_X: f64 = 300.0;
const FLOOR_PANEL_Y: f64 = 210.0;
const FLOOR_PANEL_Z: f64 = 38.0;
const FLOOR_ROWS: usize = 2;
const FLOOR_COLS: usize = 3;
const FLOOR_COUPONS: usize = FLOOR_ROWS * FLOOR_COLS;
const FLOOR_COUPON_X: f64 = 72.0;
const FLOOR_COUPON_Y: f64 = 54.0;
const FLOOR_COUPON_PITCH_X: f64 = 88.0;
const FLOOR_COUPON_PITCH_Y: f64 = 74.0;

const DOOR_PANEL_POS: (f64, f64) = (285.0, 228.0);
const DOOR_PANEL_X: f64 = 300.0;
const DOOR_PANEL_Y: f64 = 210.0;
const DOOR_PANEL_Z: f64 = 42.0;
const DOOR_COUPONS: usize = 3;
const DOOR_COUPON_X: f64 = 62.0;
const DOOR_COUPON_Y: f64 = 8.0;
const DOOR_COUPON_Z: f64 = 96.0;
const DOOR_COUPON_PITCH_X: f64 = 76.0;

const REMOVABLE_COUPONS: usize = WALL_COUPONS + FLOOR_COUPONS + DOOR_COUPONS;

const WIPE_GUIDE_POS: (f64, f64) = (-380.0, -72.0);
const WIPE_GUIDE_X: f64 = 480.0;
const WIPE_GUIDE_Y: f64 = 250.0;
const WIPE_GUIDE_Z: f64 = 30.0;
const WIPE_PATH_SEGMENTS: usize = 7;
const WIPE_STEP_MARKERS: usize = 8;
const WIPE_RAIL_W: f64 = 8.0;
const WIPE_SWAB_WELLS: usize = 4;
const WIPE_SWAB_WELL_D: f64 = 18.0;

const CONDENSATE_POS: (f64, f64) = (175.0, -72.0);
const CONDENSATE_X: f64 = 420.0;
const CONDENSATE_Y: f64 = 250.0;
const CONDENSATE_Z: f64 = 34.0;
const CONDENSATE_COUPONS: usize = 6;
const CONDENSATE_COUPON_X: f64 = 54.0;
const CONDENSATE_COUPON_Y: f64 = 32.0;
const CONDENSATE_COUPON_PITCH_X: f64 = 62.0;
const CONDENSATE_SLOPE_DROP_MM: f64 = 5.5;
const WITNESS_DROPLET_WELLS: usize = 8;

const BARCODE_TRAY_POS: (f64, f64) = (-420.0, -310.0);
const BARCODE_TRAY_X: f64 = 420.0;
const BARCODE_TRAY_Y: f64 = 145.0;
const BARCODE_TRAY_Z: f64 = 32.0;
const BARCODE_TRAY_ROWS: usize = 2;
const BARCODE_TRAY_COLS: usize = 7;
const BARCODE_TRAYS: usize = BARCODE_TRAY_ROWS * BARCODE_TRAY_COLS;
const BARCODE_LANDS: usize = 16;
const BARCODE_LAND_X: f64 = 58.0;
const BARCODE_LAND_Y: f64 = 19.0;
const COUPON_TRAY_SLOT_X: f64 = 46.0;
const COUPON_TRAY_SLOT_Y: f64 = 32.0;

const SEGREGATION_POS: (f64, f64) = (-30.0, -305.0);
const SEGREGATION_X: f64 = 260.0;
const SEGREGATION_Y: f64 = 155.0;
const SEGREGATION_Z: f64 = 78.0;
const SEGREGATION_WALL_X: f64 = 18.0;
const SEGREGATION_GATE_COUNT: usize = 2;
const CLEAN_USED_AIR_GAP: f64 = 48.0;

const STATUS_POS: (f64, f64) = (385.0, -300.0);
const STATUS_X: f64 = 330.0;
const STATUS_Y: f64 = 205.0;
const STATUS_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 5;
const STATUS_SLOT_X: f64 = 82.0;
const STATUS_SLOT_Y: f64 = 34.0;
const STATUS_LANE_PITCH_Y: f64 = 58.0;

const CAMERA_POS: (f64, f64) = (-60.0, 258.0);
const CAMERA_BRIDGE_X: f64 = 1030.0;
const CAMERA_BRIDGE_Y: f64 = 170.0;
const CAMERA_CLEARANCE_Z: f64 = 250.0;
const CAMERA_BEAM_Z: f64 = 30.0;
const CAMERA_PODS: usize = 4;
const LED_BARS: usize = 2;
const CAMERA_CALIBRATION_TARGETS: usize = 6;

const FRONT_ROBOT_APPROACH_Y: f64 = 420.0;
const REAR_INCUBATOR_DOOR_SWING_Y: f64 = 230.0;
const LEFT_CLEAN_LOAD_ACCESS_X: f64 = 210.0;
const RIGHT_USED_UNLOAD_ACCESS_X: f64 = 240.0;
const TOP_CAMERA_SERVICE_CLEARANCE_Z: f64 = 330.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        let half_x = deck_x / 2.0 - RIM_W;
        let half_y = deck_y / 2.0 - RIM_W;
        self.center.0 - self.x / 2.0 >= -half_x
            && self.center.0 + self.x / 2.0 <= half_x
            && self.center.1 - self.y / 2.0 >= -half_y
            && self.center.1 + self.y / 2.0 <= half_y
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

    let wall = removable_wall_coupon_panel();
    export(OUTPUTS[1], &wall);

    let floor = removable_floor_coupon_panel();
    export(OUTPUTS[2], &floor);

    let door = removable_door_coupon_panel();
    export(OUTPUTS[3], &door);

    let wipe = wipe_path_guide();
    export(OUTPUTS[4], &wipe);

    let condensate = condensate_contact_witness();
    export(OUTPUTS[5], &condensate);

    let camera = camera_evidence_bridge();
    export(OUTPUTS[6], &camera);

    let barcode = barcode_coupon_trays();
    export(OUTPUTS[7], &barcode);

    let segregation = clean_used_segregation();
    export(OUTPUTS[8], &segregation);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + wall.translate(WALL_PANEL_POS.0, WALL_PANEL_POS.1, insert_z(WALL_PANEL_Z))
        + floor.translate(
            FLOOR_PANEL_POS.0,
            FLOOR_PANEL_POS.1,
            insert_z(FLOOR_PANEL_Z),
        )
        + door.translate(DOOR_PANEL_POS.0, DOOR_PANEL_POS.1, insert_z(DOOR_PANEL_Z))
        + wipe.translate(WIPE_GUIDE_POS.0, WIPE_GUIDE_POS.1, insert_z(WIPE_GUIDE_Z))
        + condensate.translate(CONDENSATE_POS.0, CONDENSATE_POS.1, insert_z(CONDENSATE_Z))
        + barcode.translate(
            BARCODE_TRAY_POS.0,
            BARCODE_TRAY_POS.1,
            insert_z(BARCODE_TRAY_Z),
        )
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, DECK_Z)
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator internal-surface cleanability coupon station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm leak-tray deck");
    println!(
        "  Removable coupons:          {WALL_COUPONS} wall, {FLOOR_COUPONS} floor, {DOOR_COUPONS} door, {REMOVABLE_COUPONS} total challenge coupons"
    );
    println!(
        "  Wipe/condensate evidence:   {WIPE_PATH_SEGMENTS} wipe guide segments, {WIPE_STEP_MARKERS} path markers, {CONDENSATE_COUPONS} condensate contact coupons, {WITNESS_DROPLET_WELLS} droplet wells"
    );
    println!(
        "  Traceability/routing:       {BARCODE_TRAYS} barcode coupon tray pockets, {BARCODE_LANDS} barcode lands, {STATUS_LANES} release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} positions each"
    );
    println!(
        "  Imaging/segregation:        {CAMERA_PODS} camera pods, {LED_BARS} LED bars, {CAMERA_CALIBRATION_TARGETS} calibration targets, {SEGREGATION_GATE_COUNT} clean/used transfer gates, {CLEAN_USED_AIR_GAP:.0}mm clean-used air gap"
    );
    println!("  Output prefix:              {OUTPUT_PREFIX}");
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn assert_design_constraints() {
    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds station deck envelope",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert!(camera_bridge_rect().fits_inside(DECK_X, DECK_Y));
    assert_eq!(
        REMOVABLE_COUPONS,
        WALL_COUPONS + FLOOR_COUPONS + DOOR_COUPONS
    );
    assert_eq!(STATUS_LANES, 3);
    assert!(BARCODE_LANDS >= REMOVABLE_COUPONS);
    assert!(BARCODE_TRAYS >= REMOVABLE_COUPONS);
    assert!(CONDENSATE_SLOPE_DROP_MM / (CONDENSATE_Y - 68.0) > 0.025);
    assert!(CAMERA_CLEARANCE_Z > highest_coupon_height() + 90.0);
    assert!(TOP_CAMERA_SERVICE_CLEARANCE_Z > CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z);
}

fn layout_rects() -> [Rect; 8] {
    [
        rect(
            "removable_wall_coupon_panel",
            WALL_PANEL_POS,
            WALL_PANEL_X,
            WALL_PANEL_Y,
        ),
        rect(
            "removable_floor_coupon_panel",
            FLOOR_PANEL_POS,
            FLOOR_PANEL_X,
            FLOOR_PANEL_Y,
        ),
        rect(
            "removable_door_coupon_panel",
            DOOR_PANEL_POS,
            DOOR_PANEL_X,
            DOOR_PANEL_Y,
        ),
        rect(
            "wipe_path_guide",
            WIPE_GUIDE_POS,
            WIPE_GUIDE_X,
            WIPE_GUIDE_Y,
        ),
        rect(
            "condensate_contact_witness",
            CONDENSATE_POS,
            CONDENSATE_X,
            CONDENSATE_Y,
        ),
        rect(
            "barcode_coupon_trays",
            BARCODE_TRAY_POS,
            BARCODE_TRAY_X,
            BARCODE_TRAY_Y,
        ),
        rect(
            "clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        rect("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
    ]
}

fn camera_bridge_rect() -> Rect {
    rect(
        "camera_evidence_bridge",
        CAMERA_POS,
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
    )
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn highest_coupon_height() -> f64 {
    WALL_COUPON_Z.max(DOOR_COUPON_Z)
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "incubator_cleanability_base_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let recessed_pan = centered_cube(
        "incubator_cleanability_base_recessed_secondary_pan_cut",
        DECK_X - 2.0 * (RIM_W + 46.0),
        DECK_Y - 2.0 * (RIM_W + 48.0),
        8.0,
    )
    .translate(0.0, -12.0, DECK_Z - 4.0);
    let front_drain = centered_cylinder(
        "incubator_cleanability_base_low_point_drain_cut",
        DRAIN_PORT_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 7.0);

    deck - recessed_pan - front_drain - insert_sockets() - mount_holes()
        + perimeter_rim()
        + base_zone_label_lands()
        + base_datum_targets()
        + base_drain_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("incubator_cleanability_insert_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("incubator_cleanability_socket_{}", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("incubator_cleanability_base_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
        (0.0, -DECK_Y / 2.0 + 54.0),
        (0.0, DECK_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("incubator_cleanability_base_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "incubator_cleanability_front_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "incubator_cleanability_rear_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "incubator_cleanability_left_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "incubator_cleanability_right_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn base_zone_label_lands() -> Part {
    centered_cube(
        "incubator_cleanability_clean_incoming_zone_label_land",
        240.0,
        26.0,
        2.0,
    )
    .translate(-445.0, -220.0, DECK_Z + 1.0)
        + centered_cube(
            "incubator_cleanability_wipe_challenge_zone_label_land",
            245.0,
            26.0,
            2.0,
        )
        .translate(-180.0, 58.0, DECK_Z + 1.0)
        + centered_cube(
            "incubator_cleanability_used_recovered_zone_label_land",
            245.0,
            26.0,
            2.0,
        )
        .translate(360.0, -188.0, DECK_Z + 1.0)
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("incubator_cleanability_robot_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 82.0, -DECK_Y / 2.0 + 82.0),
        (DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 82.0),
        (-DECK_X / 2.0 + 82.0, DECK_Y / 2.0 - 82.0),
        (DECK_X / 2.0 - 82.0, DECK_Y / 2.0 - 82.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("incubator_cleanability_base_datum_{i}")).translate(
                *x,
                *y,
                DECK_Z + 2.0,
            );
    }
    targets
}

fn base_drain_witness_ribs() -> Part {
    let mut ribs = Part::empty("incubator_cleanability_base_drain_witness_ribs");
    for (i, y) in [-330.0, -280.0, -230.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("incubator_cleanability_base_front_flow_rib_{i}"),
                DECK_X - 180.0,
                4.0,
                5.0,
            )
            .translate(0.0, *y, DECK_Z + 2.5);
    }
    ribs
}

fn removable_wall_coupon_panel() -> Part {
    let body = centered_cube(
        "incubator_cleanability_wall_coupon_panel_body",
        WALL_PANEL_X,
        WALL_PANEL_Y,
        WALL_PANEL_Z,
    );
    let gasket_recess = centered_cube(
        "incubator_cleanability_wall_coupon_panel_gasket_recess_cut",
        WALL_PANEL_X - 38.0,
        WALL_PANEL_Y - 42.0,
        10.0,
    )
    .translate(0.0, 0.0, WALL_PANEL_Z / 2.0 - 4.0);

    body - gasket_recess - wall_coupon_recesses() - panel_keyhole_cuts(WALL_PANEL_Z)
        + wall_coupon_blades()
        + wall_coupon_retention_clips()
        + removable_panel_handles("wall", WALL_PANEL_X, WALL_PANEL_Y, WALL_PANEL_Z)
        + panel_corner_fiducials("wall", WALL_PANEL_X, WALL_PANEL_Y, WALL_PANEL_Z)
}

fn wall_coupon_recesses() -> Part {
    let mut cuts = Part::empty("incubator_cleanability_wall_coupon_recesses");
    for i in 0..WALL_COUPONS {
        let x = centered_index(i, WALL_COUPONS, WALL_COUPON_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("incubator_cleanability_wall_coupon_socket_cut_{i}"),
                WALL_COUPON_X + 10.0,
                20.0,
                16.0,
            )
            .translate(x, -20.0, WALL_PANEL_Z / 2.0 - 3.0);
    }
    cuts
}

fn wall_coupon_blades() -> Part {
    let mut coupons = Part::empty("incubator_cleanability_removable_wall_coupons");
    for i in 0..WALL_COUPONS {
        let x = centered_index(i, WALL_COUPONS, WALL_COUPON_PITCH_X);
        let coupon = centered_cube(
            format!("incubator_cleanability_vertical_wall_coupon_blade_{i}"),
            WALL_COUPON_X,
            WALL_COUPON_Y,
            WALL_COUPON_Z,
        )
        .translate(
            x,
            -WALL_PANEL_Y / 2.0 + 42.0,
            WALL_PANEL_Z / 2.0 + WALL_COUPON_Z / 2.0 - 4.0,
        );
        let witness_land = centered_cube(
            format!("incubator_cleanability_wall_coupon_wipe_witness_land_{i}"),
            WALL_COUPON_X - 12.0,
            2.5,
            18.0,
        )
        .translate(
            x,
            -WALL_PANEL_Y / 2.0 + 37.0,
            WALL_PANEL_Z / 2.0 + WALL_COUPON_Z - 22.0,
        );
        coupons = coupons + coupon + witness_land;
    }
    coupons
}

fn wall_coupon_retention_clips() -> Part {
    let mut clips = Part::empty("incubator_cleanability_wall_coupon_retention_clips");
    for i in 0..WALL_COUPONS {
        let x = centered_index(i, WALL_COUPONS, WALL_COUPON_PITCH_X);
        for side in [-1.0, 1.0] {
            clips = clips
                + centered_cube(
                    format!("incubator_cleanability_wall_coupon_clip_{i}_{side}"),
                    5.0,
                    28.0,
                    18.0,
                )
                .translate(
                    x + side * (WALL_COUPON_X / 2.0 + 5.0),
                    -WALL_PANEL_Y / 2.0 + 40.0,
                    WALL_PANEL_Z / 2.0 + 9.0,
                );
        }
    }
    clips
}

fn removable_floor_coupon_panel() -> Part {
    let body = centered_cube(
        "incubator_cleanability_floor_coupon_panel_body",
        FLOOR_PANEL_X,
        FLOOR_PANEL_Y,
        FLOOR_PANEL_Z,
    );
    let perimeter_recess = centered_cube(
        "incubator_cleanability_floor_coupon_panel_shadow_recess_cut",
        FLOOR_PANEL_X - 36.0,
        FLOOR_PANEL_Y - 36.0,
        8.0,
    )
    .translate(0.0, 0.0, FLOOR_PANEL_Z / 2.0 - 3.0);

    body - perimeter_recess - floor_coupon_recesses() - panel_keyhole_cuts(FLOOR_PANEL_Z)
        + floor_coupon_tiles()
        + floor_coupon_lift_tabs()
        + removable_panel_handles("floor", FLOOR_PANEL_X, FLOOR_PANEL_Y, FLOOR_PANEL_Z)
        + panel_corner_fiducials("floor", FLOOR_PANEL_X, FLOOR_PANEL_Y, FLOOR_PANEL_Z)
}

fn floor_coupon_recesses() -> Part {
    let mut recesses = Part::empty("incubator_cleanability_floor_coupon_recesses");
    for row in 0..FLOOR_ROWS {
        for col in 0..FLOOR_COLS {
            let (x, y) = floor_coupon_center(row, col);
            recesses = recesses
                + centered_cube(
                    format!("incubator_cleanability_floor_coupon_well_cut_{row}_{col}"),
                    FLOOR_COUPON_X + 8.0,
                    FLOOR_COUPON_Y + 8.0,
                    8.0,
                )
                .translate(x, y, FLOOR_PANEL_Z / 2.0 - 2.5);
        }
    }
    recesses
}

fn floor_coupon_tiles() -> Part {
    let mut tiles = Part::empty("incubator_cleanability_removable_floor_coupon_tiles");
    for row in 0..FLOOR_ROWS {
        for col in 0..FLOOR_COLS {
            let (x, y) = floor_coupon_center(row, col);
            let tile = centered_cube(
                format!("incubator_cleanability_floor_coupon_tile_{row}_{col}"),
                FLOOR_COUPON_X,
                FLOOR_COUPON_Y,
                5.0,
            )
            .translate(x, y, FLOOR_PANEL_Z / 2.0 + 2.5);
            let wipe_direction_bar = centered_cube(
                format!("incubator_cleanability_floor_coupon_wipe_direction_bar_{row}_{col}"),
                FLOOR_COUPON_X - 16.0,
                4.0,
                3.0,
            )
            .translate(x, y, FLOOR_PANEL_Z / 2.0 + 6.5);
            tiles = tiles + tile + wipe_direction_bar;
        }
    }
    tiles
}

fn floor_coupon_lift_tabs() -> Part {
    let mut tabs = Part::empty("incubator_cleanability_floor_coupon_lift_tabs");
    for row in 0..FLOOR_ROWS {
        for col in 0..FLOOR_COLS {
            let (x, y) = floor_coupon_center(row, col);
            tabs = tabs
                + centered_cube(
                    format!("incubator_cleanability_floor_coupon_pull_tab_{row}_{col}"),
                    18.0,
                    8.0,
                    8.0,
                )
                .translate(
                    x + FLOOR_COUPON_X / 2.0 - 10.0,
                    y,
                    FLOOR_PANEL_Z / 2.0 + 8.0,
                );
        }
    }
    tabs
}

fn removable_door_coupon_panel() -> Part {
    let body = centered_cube(
        "incubator_cleanability_door_coupon_panel_body",
        DOOR_PANEL_X,
        DOOR_PANEL_Y,
        DOOR_PANEL_Z,
    );
    let gasket_track = centered_cube(
        "incubator_cleanability_door_gasket_track_cut",
        DOOR_PANEL_X - 46.0,
        DOOR_PANEL_Y - 52.0,
        10.0,
    )
    .translate(0.0, 0.0, DOOR_PANEL_Z / 2.0 - 3.0);

    body - gasket_track - door_coupon_recesses() - panel_keyhole_cuts(DOOR_PANEL_Z)
        + door_coupon_blades()
        + door_gasket_witness_strips()
        + door_hinge_barrel_gauge()
        + removable_panel_handles("door", DOOR_PANEL_X, DOOR_PANEL_Y, DOOR_PANEL_Z)
        + panel_corner_fiducials("door", DOOR_PANEL_X, DOOR_PANEL_Y, DOOR_PANEL_Z)
}

fn door_coupon_recesses() -> Part {
    let mut recesses = Part::empty("incubator_cleanability_door_coupon_recesses");
    for i in 0..DOOR_COUPONS {
        let x = centered_index(i, DOOR_COUPONS, DOOR_COUPON_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("incubator_cleanability_door_coupon_socket_cut_{i}"),
                DOOR_COUPON_X + 10.0,
                22.0,
                16.0,
            )
            .translate(x, -18.0, DOOR_PANEL_Z / 2.0 - 3.0);
    }
    recesses
}

fn door_coupon_blades() -> Part {
    let mut coupons = Part::empty("incubator_cleanability_removable_door_coupons");
    for i in 0..DOOR_COUPONS {
        let x = centered_index(i, DOOR_COUPONS, DOOR_COUPON_PITCH_X);
        let coupon = centered_cube(
            format!("incubator_cleanability_vertical_door_coupon_blade_{i}"),
            DOOR_COUPON_X,
            DOOR_COUPON_Y,
            DOOR_COUPON_Z,
        )
        .translate(
            x,
            -DOOR_PANEL_Y / 2.0 + 44.0,
            DOOR_PANEL_Z / 2.0 + DOOR_COUPON_Z / 2.0 - 4.0,
        );
        let handle_shadow = centered_cube(
            format!("incubator_cleanability_door_coupon_handle_shadow_{i}"),
            16.0,
            8.0,
            28.0,
        )
        .translate(
            x + DOOR_COUPON_X / 2.0 - 9.0,
            -DOOR_PANEL_Y / 2.0 + 36.0,
            DOOR_PANEL_Z / 2.0 + DOOR_COUPON_Z - 24.0,
        );
        coupons = coupons + coupon + handle_shadow;
    }
    coupons
}

fn door_gasket_witness_strips() -> Part {
    let top = centered_cube(
        "incubator_cleanability_door_top_gasket_compression_witness_strip",
        DOOR_PANEL_X - 76.0,
        6.0,
        5.0,
    )
    .translate(0.0, DOOR_PANEL_Y / 2.0 - 48.0, DOOR_PANEL_Z / 2.0 + 4.0);
    let bottom = centered_cube(
        "incubator_cleanability_door_bottom_gasket_compression_witness_strip",
        DOOR_PANEL_X - 76.0,
        6.0,
        5.0,
    )
    .translate(0.0, -DOOR_PANEL_Y / 2.0 + 74.0, DOOR_PANEL_Z / 2.0 + 4.0);
    let latch_edge = centered_cube(
        "incubator_cleanability_door_latch_edge_witness_strip",
        6.0,
        DOOR_PANEL_Y - 100.0,
        5.0,
    )
    .translate(DOOR_PANEL_X / 2.0 - 44.0, 2.0, DOOR_PANEL_Z / 2.0 + 4.0);
    top + bottom + latch_edge
}

fn door_hinge_barrel_gauge() -> Part {
    let mut barrels = Part::empty("incubator_cleanability_door_hinge_barrel_gauge");
    for i in 0..3 {
        let y = centered_index(i, 3, 56.0);
        barrels = barrels
            + centered_cylinder(
                format!("incubator_cleanability_door_hinge_barrel_witness_{i}"),
                8.0,
                42.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(-DOOR_PANEL_X / 2.0 + 28.0, y, DOOR_PANEL_Z / 2.0 + 9.0);
    }
    barrels
}

fn wipe_path_guide() -> Part {
    let body = centered_cube(
        "incubator_cleanability_wipe_path_guide_body",
        WIPE_GUIDE_X,
        WIPE_GUIDE_Y,
        WIPE_GUIDE_Z,
    );
    let shallow_recess = centered_cube(
        "incubator_cleanability_wipe_path_recessed_field_cut",
        WIPE_GUIDE_X - 44.0,
        WIPE_GUIDE_Y - 42.0,
        8.0,
    )
    .translate(0.0, 0.0, WIPE_GUIDE_Z / 2.0 - 3.0);

    body - shallow_recess - wipe_swab_well_cuts()
        + wipe_path_side_rails()
        + wipe_step_markers()
        + wipe_start_end_rings()
        + wipe_motion_arrows()
}

fn wipe_path_side_rails() -> Part {
    let mut rails = Part::empty("incubator_cleanability_wipe_path_side_rails");
    for (i, (x, y, sx, sy)) in wipe_path_segments().iter().enumerate() {
        let segment = centered_cube(
            format!("incubator_cleanability_wipe_path_centerline_segment_{i}"),
            *sx,
            *sy,
            4.0,
        )
        .translate(*x, *y, WIPE_GUIDE_Z / 2.0 + 2.0);
        let left_rail = centered_cube(
            format!("incubator_cleanability_wipe_path_left_edge_rail_{i}"),
            if *sx > *sy { *sx } else { WIPE_RAIL_W },
            if *sx > *sy { WIPE_RAIL_W } else { *sy },
            8.0,
        )
        .translate(
            *x - if *sx > *sy { 0.0 } else { 14.0 },
            *y - if *sx > *sy { 14.0 } else { 0.0 },
            WIPE_GUIDE_Z / 2.0 + 6.0,
        );
        let right_rail = centered_cube(
            format!("incubator_cleanability_wipe_path_right_edge_rail_{i}"),
            if *sx > *sy { *sx } else { WIPE_RAIL_W },
            if *sx > *sy { WIPE_RAIL_W } else { *sy },
            8.0,
        )
        .translate(
            *x + if *sx > *sy { 0.0 } else { 14.0 },
            *y + if *sx > *sy { 14.0 } else { 0.0 },
            WIPE_GUIDE_Z / 2.0 + 6.0,
        );
        rails = rails + segment + left_rail + right_rail;
    }
    rails
}

fn wipe_step_markers() -> Part {
    let mut markers = Part::empty("incubator_cleanability_wipe_path_step_markers");
    for (i, (x, y)) in wipe_marker_centers().iter().enumerate() {
        markers = markers
            + centered_cylinder(
                format!("incubator_cleanability_wipe_step_marker_{i}"),
                9.0,
                3.0,
                24,
            )
            .translate(*x, *y, WIPE_GUIDE_Z / 2.0 + 7.5);
    }
    markers
}

fn wipe_start_end_rings() -> Part {
    let start_outer = centered_cylinder(
        "incubator_cleanability_wipe_start_ring_outer",
        20.0,
        5.0,
        36,
    )
    .translate(
        -WIPE_GUIDE_X / 2.0 + 60.0,
        WIPE_GUIDE_Y / 2.0 - 58.0,
        WIPE_GUIDE_Z / 2.0 + 7.0,
    );
    let start_inner = centered_cylinder(
        "incubator_cleanability_wipe_start_ring_inner_cut",
        11.0,
        7.0,
        36,
    )
    .translate(
        -WIPE_GUIDE_X / 2.0 + 60.0,
        WIPE_GUIDE_Y / 2.0 - 58.0,
        WIPE_GUIDE_Z / 2.0 + 7.0,
    );
    let end_outer = centered_cylinder("incubator_cleanability_wipe_end_ring_outer", 20.0, 5.0, 36)
        .translate(
            WIPE_GUIDE_X / 2.0 - 60.0,
            -WIPE_GUIDE_Y / 2.0 + 58.0,
            WIPE_GUIDE_Z / 2.0 + 7.0,
        );
    let end_inner = centered_cylinder(
        "incubator_cleanability_wipe_end_ring_inner_cut",
        11.0,
        7.0,
        36,
    )
    .translate(
        WIPE_GUIDE_X / 2.0 - 60.0,
        -WIPE_GUIDE_Y / 2.0 + 58.0,
        WIPE_GUIDE_Z / 2.0 + 7.0,
    );
    (start_outer - start_inner) + (end_outer - end_inner)
}

fn wipe_motion_arrows() -> Part {
    let mut arrows = Part::empty("incubator_cleanability_wipe_motion_arrow_tabs");
    for (i, (x, y)) in [(-112.0, 72.0), (-15.0, 20.0), (92.0, -40.0), (160.0, -84.0)]
        .iter()
        .enumerate()
    {
        let arrow = centered_cube(
            format!("incubator_cleanability_wipe_direction_arrow_body_{i}"),
            24.0,
            8.0,
            5.0,
        )
        .translate(*x, *y, WIPE_GUIDE_Z / 2.0 + 9.0);
        let arrow_head = centered_cube(
            format!("incubator_cleanability_wipe_direction_arrow_head_{i}"),
            12.0,
            18.0,
            5.0,
        )
        .translate(*x + 16.0, *y, WIPE_GUIDE_Z / 2.0 + 9.0);
        arrows = arrows + arrow + arrow_head;
    }
    arrows
}

fn wipe_swab_well_cuts() -> Part {
    let mut wells = Part::empty("incubator_cleanability_wipe_swab_well_cuts");
    for i in 0..WIPE_SWAB_WELLS {
        wells = wells
            + centered_cylinder(
                format!("incubator_cleanability_wipe_swab_well_cut_{i}"),
                WIPE_SWAB_WELL_D / 2.0,
                WIPE_GUIDE_Z + 3.0,
                28,
            )
            .translate(
                -WIPE_GUIDE_X / 2.0 + 48.0 + i as f64 * 28.0,
                -WIPE_GUIDE_Y / 2.0 + 36.0,
                0.0,
            );
    }
    wells
}

fn condensate_contact_witness() -> Part {
    let body = centered_cube(
        "incubator_cleanability_condensate_contact_witness_body",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    );
    let basin = centered_cube(
        "incubator_cleanability_condensate_witness_basin_cut",
        CONDENSATE_X - 48.0,
        CONDENSATE_Y - 50.0,
        12.0,
    )
    .translate(0.0, 4.0, CONDENSATE_Z / 2.0 - 4.0);
    let drain_channel = centered_cube(
        "incubator_cleanability_condensate_witness_drain_channel_cut",
        24.0,
        CONDENSATE_Y - 74.0,
        8.0,
    )
    .translate(0.0, -8.0, CONDENSATE_Z / 2.0);

    body - basin - drain_channel
        + condensate_sloped_witness_floor()
        + condensate_coupon_lands()
        + condensate_droplet_witness_wells()
        + condensate_contact_break_ribs()
        + condensate_low_point_sump()
}

fn condensate_sloped_witness_floor() -> Part {
    centered_cube(
        "incubator_cleanability_condensate_sloped_witness_floor",
        CONDENSATE_X - 86.0,
        CONDENSATE_Y - 94.0,
        4.0,
    )
    .rotate(condensate_slope_angle_deg(), 0.0, 0.0)
    .translate(0.0, -2.0, CONDENSATE_Z / 2.0 + 1.0)
}

fn condensate_coupon_lands() -> Part {
    let mut lands = Part::empty("incubator_cleanability_condensate_coupon_lands");
    for i in 0..CONDENSATE_COUPONS {
        let x = centered_index(i, CONDENSATE_COUPONS, CONDENSATE_COUPON_PITCH_X);
        let coupon = centered_cube(
            format!("incubator_cleanability_condensate_contact_coupon_{i}"),
            CONDENSATE_COUPON_X,
            CONDENSATE_COUPON_Y,
            4.0,
        )
        .translate(x, 34.0, CONDENSATE_Z / 2.0 + 6.0);
        let capillary_edge = centered_cube(
            format!("incubator_cleanability_condensate_coupon_capillary_edge_{i}"),
            CONDENSATE_COUPON_X + 8.0,
            4.0,
            6.0,
        )
        .translate(
            x,
            34.0 - CONDENSATE_COUPON_Y / 2.0 - 5.0,
            CONDENSATE_Z / 2.0 + 8.0,
        );
        lands = lands + coupon + capillary_edge;
    }
    lands
}

fn condensate_droplet_witness_wells() -> Part {
    let mut wells = Part::empty("incubator_cleanability_condensate_droplet_witness_wells");
    for i in 0..WITNESS_DROPLET_WELLS {
        let x = centered_index(i, WITNESS_DROPLET_WELLS, 40.0);
        wells = wells
            + centered_cylinder(
                format!("incubator_cleanability_condensate_droplet_well_{i}"),
                7.0,
                4.0,
                24,
            )
            .translate(x, -CONDENSATE_Y / 2.0 + 52.0, CONDENSATE_Z / 2.0 + 6.0);
    }
    wells
}

fn condensate_contact_break_ribs() -> Part {
    let mut ribs = Part::empty("incubator_cleanability_condensate_contact_break_ribs");
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("incubator_cleanability_condensate_flow_break_rib_{i}"),
                CONDENSATE_X - 132.0,
                4.0,
                5.0,
            )
            .translate(0.0, 84.0 - i as f64 * 38.0, CONDENSATE_Z / 2.0 + 7.0);
    }
    ribs
}

fn condensate_low_point_sump() -> Part {
    let sump = centered_cube(
        "incubator_cleanability_condensate_low_point_sump_witness_land",
        96.0,
        38.0,
        5.0,
    )
    .translate(0.0, -CONDENSATE_Y / 2.0 + 34.0, CONDENSATE_Z / 2.0 + 6.5);
    let drain = centered_cylinder(
        "incubator_cleanability_condensate_sump_drain_witness",
        8.0,
        5.0,
        28,
    )
    .translate(0.0, -CONDENSATE_Y / 2.0 + 34.0, CONDENSATE_Z / 2.0 + 11.0);
    sump + drain
}

fn camera_evidence_bridge() -> Part {
    let mut bridge = Part::empty("incubator_cleanability_camera_evidence_bridge");
    for (i, (x, y)) in camera_post_centers().iter().enumerate() {
        bridge = bridge
            + centered_cube(
                format!("incubator_cleanability_camera_bridge_post_{i}"),
                24.0,
                24.0,
                CAMERA_CLEARANCE_Z,
            )
            .translate(*x, *y, CAMERA_CLEARANCE_Z / 2.0);
    }

    let main_beam = centered_cube(
        "incubator_cleanability_camera_bridge_overhead_beam",
        CAMERA_BRIDGE_X,
        32.0,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0);
    let rear_beam = centered_cube(
        "incubator_cleanability_camera_bridge_rear_stiffener",
        CAMERA_BRIDGE_X,
        18.0,
        22.0,
    )
    .translate(0.0, CAMERA_BRIDGE_Y / 2.0 - 24.0, CAMERA_CLEARANCE_Z + 11.0);
    let front_beam = centered_cube(
        "incubator_cleanability_camera_bridge_front_stiffener",
        CAMERA_BRIDGE_X,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 + 24.0,
        CAMERA_CLEARANCE_Z + 11.0,
    );

    bridge
        + main_beam
        + rear_beam
        + front_beam
        + camera_pods()
        + led_bars()
        + camera_calibration_targets()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("incubator_cleanability_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = centered_index(i, CAMERA_PODS, 210.0);
        let pod = centered_cube(
            format!("incubator_cleanability_evidence_camera_pod_{i}"),
            58.0,
            44.0,
            26.0,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - 18.0);
        let lens = centered_cylinder(
            format!("incubator_cleanability_evidence_camera_lens_{i}"),
            12.0,
            8.0,
            30,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - 35.0);
        pods = pods + pod + lens;
    }
    pods
}

fn led_bars() -> Part {
    let front = centered_cube(
        "incubator_cleanability_front_evidence_led_bar",
        CAMERA_BRIDGE_X - 190.0,
        12.0,
        10.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 + 46.0,
        CAMERA_CLEARANCE_Z - 28.0,
    );
    let rear = centered_cube(
        "incubator_cleanability_rear_evidence_led_bar",
        CAMERA_BRIDGE_X - 190.0,
        12.0,
        10.0,
    )
    .translate(0.0, CAMERA_BRIDGE_Y / 2.0 - 46.0, CAMERA_CLEARANCE_Z - 28.0);
    front + rear
}

fn camera_calibration_targets() -> Part {
    let mut targets = Part::empty("incubator_cleanability_camera_calibration_targets");
    for i in 0..CAMERA_CALIBRATION_TARGETS {
        let x = centered_index(i, CAMERA_CALIBRATION_TARGETS, 74.0);
        targets = targets
            + fiducial_disc(&format!(
                "incubator_cleanability_camera_calibration_target_{i}"
            ))
            .translate(x, CAMERA_BRIDGE_Y / 2.0 - 62.0, 12.0);
    }
    targets
}

fn barcode_coupon_trays() -> Part {
    let body = centered_cube(
        "incubator_cleanability_barcode_coupon_tray_body",
        BARCODE_TRAY_X,
        BARCODE_TRAY_Y,
        BARCODE_TRAY_Z,
    );
    let tray_field = centered_cube(
        "incubator_cleanability_barcode_coupon_tray_field_cut",
        BARCODE_TRAY_X - 34.0,
        BARCODE_TRAY_Y - 30.0,
        8.0,
    )
    .translate(0.0, 0.0, BARCODE_TRAY_Z / 2.0 - 3.0);

    body - tray_field - barcode_tray_pocket_cuts()
        + barcode_tray_pocket_lips()
        + barcode_lands()
        + barcode_tray_quarantine_flags()
}

fn barcode_tray_pocket_cuts() -> Part {
    let mut cuts = Part::empty("incubator_cleanability_barcode_coupon_tray_pocket_cuts");
    for row in 0..BARCODE_TRAY_ROWS {
        for col in 0..BARCODE_TRAY_COLS {
            let (x, y) = barcode_tray_center(row, col);
            cuts = cuts
                + centered_cube(
                    format!("incubator_cleanability_barcode_coupon_pocket_cut_{row}_{col}"),
                    COUPON_TRAY_SLOT_X,
                    COUPON_TRAY_SLOT_Y,
                    10.0,
                )
                .translate(x, y, BARCODE_TRAY_Z / 2.0 - 2.0);
        }
    }
    cuts
}

fn barcode_tray_pocket_lips() -> Part {
    let mut lips = Part::empty("incubator_cleanability_barcode_coupon_tray_pocket_lips");
    for row in 0..BARCODE_TRAY_ROWS {
        for col in 0..BARCODE_TRAY_COLS {
            let (x, y) = barcode_tray_center(row, col);
            lips = lips
                + centered_cube(
                    format!("incubator_cleanability_barcode_coupon_retention_lip_{row}_{col}"),
                    COUPON_TRAY_SLOT_X + 10.0,
                    5.0,
                    5.0,
                )
                .translate(
                    x,
                    y - COUPON_TRAY_SLOT_Y / 2.0 - 5.0,
                    BARCODE_TRAY_Z / 2.0 + 5.0,
                );
        }
    }
    lips
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("incubator_cleanability_barcode_coupon_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 8;
        let col = i % 8;
        let x = centered_index(col, 8, 48.0);
        let y = BARCODE_TRAY_Y / 2.0 - 28.0 - row as f64 * 28.0;
        lands = lands
            + centered_cube(
                format!("incubator_cleanability_coupon_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, BARCODE_TRAY_Z / 2.0 + 5.0);
    }
    lands
}

fn barcode_tray_quarantine_flags() -> Part {
    let clean_flag = centered_cube(
        "incubator_cleanability_barcode_tray_clean_flag_land",
        86.0,
        18.0,
        4.0,
    )
    .translate(
        -BARCODE_TRAY_X / 2.0 + 58.0,
        -BARCODE_TRAY_Y / 2.0 + 28.0,
        BARCODE_TRAY_Z / 2.0 + 7.0,
    );
    let used_flag = centered_cube(
        "incubator_cleanability_barcode_tray_used_flag_land",
        86.0,
        18.0,
        4.0,
    )
    .translate(
        BARCODE_TRAY_X / 2.0 - 58.0,
        -BARCODE_TRAY_Y / 2.0 + 28.0,
        BARCODE_TRAY_Z / 2.0 + 7.0,
    );
    clean_flag + used_flag
}

fn clean_used_segregation() -> Part {
    let base = centered_cube(
        "incubator_cleanability_clean_used_segregation_base",
        SEGREGATION_X,
        SEGREGATION_Y,
        18.0,
    );
    let wall = centered_cube(
        "incubator_cleanability_clean_used_center_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 - 9.0);
    let clean_shelf = centered_cube(
        "incubator_cleanability_clean_coupon_incoming_shelf",
        SEGREGATION_X / 2.0 - CLEAN_USED_AIR_GAP / 2.0,
        SEGREGATION_Y - 34.0,
        12.0,
    )
    .translate(
        -SEGREGATION_X / 4.0 - CLEAN_USED_AIR_GAP / 4.0,
        0.0,
        SEGREGATION_Z / 2.0 + 2.0,
    );
    let used_shelf = centered_cube(
        "incubator_cleanability_used_coupon_recovered_shelf",
        SEGREGATION_X / 2.0 - CLEAN_USED_AIR_GAP / 2.0,
        SEGREGATION_Y - 34.0,
        12.0,
    )
    .translate(
        SEGREGATION_X / 4.0 + CLEAN_USED_AIR_GAP / 4.0,
        0.0,
        SEGREGATION_Z / 2.0 + 2.0,
    );

    base + wall + clean_shelf + used_shelf + segregation_one_way_gates() + segregation_drip_breaks()
}

fn segregation_one_way_gates() -> Part {
    let mut gates = Part::empty("incubator_cleanability_clean_used_one_way_gates");
    for i in 0..SEGREGATION_GATE_COUNT {
        let y = centered_index(i, SEGREGATION_GATE_COUNT, 58.0);
        gates = gates
            + centered_cube(
                format!("incubator_cleanability_clean_used_transfer_throat_gate_{i}"),
                62.0,
                16.0,
                28.0,
            )
            .translate(0.0, y, 24.0)
            + centered_cube(
                format!("incubator_cleanability_clean_used_gate_flag_land_{i}"),
                46.0,
                10.0,
                4.0,
            )
            .translate(0.0, y, 44.0);
    }
    gates
}

fn segregation_drip_breaks() -> Part {
    let mut breaks = Part::empty("incubator_cleanability_clean_used_drip_breaks");
    for i in 0..4 {
        let y = centered_index(i, 4, 32.0);
        breaks = breaks
            + centered_cube(
                format!("incubator_cleanability_clean_side_drip_break_{i}"),
                74.0,
                4.0,
                6.0,
            )
            .translate(-72.0, y, 17.0)
            + centered_cube(
                format!("incubator_cleanability_used_side_drip_break_{i}"),
                74.0,
                4.0,
                6.0,
            )
            .translate(72.0, y, 17.0);
    }
    breaks
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "incubator_cleanability_release_hold_reject_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let top_recess = centered_cube(
        "incubator_cleanability_status_lane_field_recess_cut",
        STATUS_X - 34.0,
        STATUS_Y - 30.0,
        8.0,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0 - 3.0);

    body - top_recess - status_slot_cuts()
        + status_lane_dividers()
        + status_lane_label_lands()
        + status_lane_stop_gates()
}

fn status_slot_cuts() -> Part {
    let mut cuts = Part::empty("incubator_cleanability_status_slot_cuts");
    for lane in 0..STATUS_LANES {
        let y = status_lane_y(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 54.0);
            cuts = cuts
                + centered_cube(
                    format!("incubator_cleanability_status_lane_{lane}_slot_{slot}_cut"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    10.0,
                )
                .translate(x, y, STATUS_Z / 2.0 - 2.0);
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("incubator_cleanability_status_lane_dividers");
    for i in 0..=STATUS_LANES {
        let y = -STATUS_Y / 2.0 + 28.0 + i as f64 * STATUS_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("incubator_cleanability_status_lane_divider_{i}"),
                STATUS_X - 46.0,
                5.0,
                11.0,
            )
            .translate(0.0, y, STATUS_Z / 2.0 + 5.5);
    }
    dividers
}

fn status_lane_label_lands() -> Part {
    let mut labels = Part::empty("incubator_cleanability_status_lane_label_lands");
    for (lane, name) in ["release", "hold", "reject"].iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("incubator_cleanability_{name}_lane_label_land"),
                82.0,
                18.0,
                4.0,
            )
            .translate(
                -STATUS_X / 2.0 + 54.0,
                status_lane_y(lane),
                STATUS_Z / 2.0 + 8.0,
            );
    }
    labels
}

fn status_lane_stop_gates() -> Part {
    let mut gates = Part::empty("incubator_cleanability_status_lane_stop_gates");
    for lane in 0..STATUS_LANES {
        gates = gates
            + centered_cube(
                format!("incubator_cleanability_status_lane_{lane}_end_stop_gate"),
                12.0,
                STATUS_SLOT_Y + 14.0,
                24.0,
            )
            .translate(
                STATUS_X / 2.0 - 32.0,
                status_lane_y(lane),
                STATUS_Z / 2.0 + 12.0,
            );
    }
    gates
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "incubator_cleanability_front_robot_approach_keepout_gauge",
        DECK_X - 120.0,
        FRONT_ROBOT_APPROACH_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_APPROACH_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        "incubator_cleanability_rear_incubator_door_swing_keepout_gauge",
        DECK_X - 180.0,
        REAR_INCUBATOR_DOOR_SWING_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_INCUBATOR_DOOR_SWING_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left = centered_cube(
        "incubator_cleanability_left_clean_load_access_keepout_gauge",
        LEFT_CLEAN_LOAD_ACCESS_X,
        DECK_Y - 160.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_CLEAN_LOAD_ACCESS_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "incubator_cleanability_right_used_unload_access_keepout_gauge",
        RIGHT_USED_UNLOAD_ACCESS_X,
        DECK_Y - 160.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_USED_UNLOAD_ACCESS_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top = centered_cube(
        "incubator_cleanability_top_camera_service_clearance_gauge",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y + 120.0,
        10.0,
    )
    .translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        DECK_Z + TOP_CAMERA_SERVICE_CLEARANCE_Z,
    );
    front + rear + left + right + top
}

fn removable_panel_handles(prefix: &str, panel_x: f64, panel_y: f64, panel_z: f64) -> Part {
    let left = centered_cube(
        format!("incubator_cleanability_{prefix}_left_robot_pull_handle"),
        34.0,
        12.0,
        18.0,
    )
    .translate(
        -panel_x / 2.0 + 44.0,
        panel_y / 2.0 - 24.0,
        panel_z / 2.0 + 9.0,
    );
    let right = centered_cube(
        format!("incubator_cleanability_{prefix}_right_robot_pull_handle"),
        34.0,
        12.0,
        18.0,
    )
    .translate(
        panel_x / 2.0 - 44.0,
        panel_y / 2.0 - 24.0,
        panel_z / 2.0 + 9.0,
    );
    left + right
}

fn panel_keyhole_cuts(panel_z: f64) -> Part {
    let mut cuts = Part::empty("incubator_cleanability_panel_keyhole_cuts");
    for (i, (x, y)) in [
        (-104.0, 72.0),
        (104.0, 72.0),
        (-104.0, -72.0),
        (104.0, -72.0),
    ]
    .iter()
    .enumerate()
    {
        let round = centered_cylinder(
            format!("incubator_cleanability_panel_keyhole_round_cut_{i}"),
            6.0,
            panel_z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("incubator_cleanability_panel_keyhole_slot_cut_{i}"),
            20.0,
            7.0,
            panel_z + 4.0,
        )
        .translate(*x + 8.0, *y, 0.0);
        cuts = cuts + round + slot;
    }
    cuts
}

fn panel_corner_fiducials(prefix: &str, panel_x: f64, panel_y: f64, panel_z: f64) -> Part {
    let mut fiducials = Part::empty(format!("incubator_cleanability_{prefix}_panel_fiducials"));
    for (i, (x, y)) in [
        (-panel_x / 2.0 + 28.0, -panel_y / 2.0 + 28.0),
        (panel_x / 2.0 - 28.0, -panel_y / 2.0 + 28.0),
        (-panel_x / 2.0 + 28.0, panel_y / 2.0 - 28.0),
        (panel_x / 2.0 - 28.0, panel_y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "incubator_cleanability_{prefix}_panel_fiducial_{i}"
            ))
            .translate(*x, *y, panel_z / 2.0 + 3.0);
    }
    fiducials
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 12.0, 3.0, 36);
    let inner = centered_cylinder(format!("{name}_center_cut"), 4.5, 4.0, 28);
    outer - inner
}

fn wipe_path_segments() -> [(f64, f64, f64, f64); WIPE_PATH_SEGMENTS] {
    [
        (-158.0, 72.0, 178.0, 8.0),
        (-70.0, 38.0, 8.0, 76.0),
        (12.0, 2.0, 164.0, 8.0),
        (94.0, -34.0, 8.0, 78.0),
        (154.0, -70.0, 128.0, 8.0),
        (12.0, -88.0, 8.0, 44.0),
        (86.0, -108.0, 168.0, 8.0),
    ]
}

fn wipe_marker_centers() -> [(f64, f64); WIPE_STEP_MARKERS] {
    [
        (-200.0, 72.0),
        (-112.0, 72.0),
        (-70.0, 22.0),
        (-10.0, 2.0),
        (94.0, -16.0),
        (154.0, -70.0),
        (12.0, -88.0),
        (168.0, -108.0),
    ]
}

fn camera_post_centers() -> [(f64, f64); 4] {
    [
        (-CAMERA_BRIDGE_X / 2.0 + 42.0, -CAMERA_BRIDGE_Y / 2.0 + 32.0),
        (CAMERA_BRIDGE_X / 2.0 - 42.0, -CAMERA_BRIDGE_Y / 2.0 + 32.0),
        (-CAMERA_BRIDGE_X / 2.0 + 42.0, CAMERA_BRIDGE_Y / 2.0 - 32.0),
        (CAMERA_BRIDGE_X / 2.0 - 42.0, CAMERA_BRIDGE_Y / 2.0 - 32.0),
    ]
}

fn floor_coupon_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, FLOOR_COLS, FLOOR_COUPON_PITCH_X),
        centered_index(row, FLOOR_ROWS, FLOOR_COUPON_PITCH_Y) - 8.0,
    )
}

fn barcode_tray_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, BARCODE_TRAY_COLS, 52.0),
        centered_index(row, BARCODE_TRAY_ROWS, 46.0) - 14.0,
    )
}

fn status_lane_y(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn condensate_slope_angle_deg() -> f64 {
    (CONDENSATE_SLOPE_DROP_MM / (CONDENSATE_Y - 68.0))
        .atan()
        .to_degrees()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "removable_wall_coupons",
            "removable_floor_coupons",
            "removable_door_coupons",
            "wipe_path_guide",
            "condensate_contact_witness",
            "camera_evidence_bridge",
            "barcode_coupon_trays",
            "clean_used_segregation",
            "released_lane",
            "hold_lane",
            "reject_lane",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 12);
    }

    #[test]
    fn module_layout_fits_the_incubator_coupon_deck() {
        assert_design_constraints();
        for rect in layout_rects() {
            assert!(rect.fits_inside(DECK_X, DECK_Y));
        }
        assert!(camera_bridge_rect().fits_inside(DECK_X, DECK_Y));
    }

    #[test]
    fn removable_coupon_inventory_matches_traceability_capacity() {
        assert_eq!(FLOOR_COUPONS, FLOOR_ROWS * FLOOR_COLS);
        assert_eq!(REMOVABLE_COUPONS, 13);
        assert_eq!(BARCODE_TRAYS, BARCODE_TRAY_ROWS * BARCODE_TRAY_COLS);
        assert!(BARCODE_TRAYS >= REMOVABLE_COUPONS);
        assert!(BARCODE_LANDS >= REMOVABLE_COUPONS);
    }

    #[test]
    fn wipe_path_and_condensate_witness_have_controlled_evidence_geometry() {
        assert_eq!(wipe_path_segments().len(), WIPE_PATH_SEGMENTS);
        assert_eq!(wipe_marker_centers().len(), WIPE_STEP_MARKERS);
        assert_eq!(CONDENSATE_COUPONS, 6);
        assert!(condensate_slope_angle_deg() > 1.5);
        assert!(CONDENSATE_COUPON_X > WITNESS_DROPLET_WELLS as f64);
    }

    #[test]
    fn clean_used_and_status_lanes_are_not_under_specified() {
        assert_eq!(SEGREGATION_GATE_COUNT, 2);
        assert!(CLEAN_USED_AIR_GAP >= 40.0);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE * STATUS_LANES, 15);
        assert!(STATUS_SLOT_X > COUPON_TRAY_SLOT_X);
    }

    #[test]
    fn camera_bridge_clears_coupon_and_service_envelopes() {
        assert_eq!(CAMERA_PODS, 4);
        assert_eq!(LED_BARS, 2);
        assert_eq!(CAMERA_CALIBRATION_TARGETS, 6);
        assert!(CAMERA_CLEARANCE_Z > highest_coupon_height() + 90.0);
        assert!(TOP_CAMERA_SERVICE_CLEARANCE_Z > CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z);
    }

    #[test]
    fn generated_parts_cover_all_exported_components() {
        let parts = [
            base_leak_tray(),
            removable_wall_coupon_panel(),
            removable_floor_coupon_panel(),
            removable_door_coupon_panel(),
            wipe_path_guide(),
            condensate_contact_witness(),
            camera_evidence_bridge(),
            barcode_coupon_trays(),
            clean_used_segregation(),
            release_hold_reject_lanes(),
            robot_service_keepouts(),
            base_leak_tray()
                + removable_wall_coupon_panel().translate(
                    WALL_PANEL_POS.0,
                    WALL_PANEL_POS.1,
                    insert_z(WALL_PANEL_Z),
                ),
        ];
        assert_eq!(parts.len(), OUTPUTS.len());
    }
}
