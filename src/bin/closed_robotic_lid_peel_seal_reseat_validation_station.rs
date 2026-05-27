use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic lid peel/seal/reseat validation station.
//
// Standalone validation CAD for challenging robotic peel and reseat operations
// on sealed cassette or consumable lids. The fixture keeps the cassette/lid
// nest, peel path witness rails, seal witness compression coupons, force gauge
// pockets, particle/shedding capture inserts, open-time tokens, quarantine
// lanes, evidence capture, and robot/service keepout gauges physically tied to
// one contained station. It is packaging and metrology support geometry only:
// it does not define sterile barrier acceptance criteria, robot recipes, seal
// materials, particle limits, or release decisions.

const OUTPUT_PREFIX: &str = "closed_robotic_lid_peel_seal_reseat_validation_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_base_containment_tray.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_lid_reseat_cassette_nest.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_peel_path_witness_rails.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_seal_compression_witness_bank.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_force_gauge_panel.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_particle_shedding_capture_cassette.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_open_time_token_clock_rail.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_quarantine_disposition_lanes.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_barcode_custody_certificate_lands.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_evidence_camera_bridge.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_robot_service_keepout_gauges.stl",
    "output/closed_robotic_lid_peel_seal_reseat_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "cassette_lid_reseat_nest",
    "robotic_peel_path_witness_rails",
    "seal_witness_compression_bank",
    "peel_force_gauge_pocket",
    "reseat_force_gauge_pocket",
    "particle_shedding_capture",
    "open_time_token_rail",
    "quarantine_disposition_lanes",
    "barcode_custody_certificate_lands",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1460.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_RECESS_Z: f64 = 8.0;
const MODULE_GAP: f64 = 14.0;
const MOUNT_HOLE_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const CASSETTE_COLS: usize = 1;
const CASSETTE_ROWS: usize = 1;
const CHIP_GAP_X: f64 = 6.5;
const CHIP_GAP_Y: f64 = 5.5;
const CASSETTE_MARGIN_X: f64 = 52.0;
const CASSETTE_MARGIN_Y: f64 = 48.0;
const CASSETTE_X: f64 = CASSETTE_COLS as f64 * REVC_CHIP_LENGTH
    + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP_X
    + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH
    + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP_Y
    + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = REVC_TOTAL_HEIGHT + 26.0;
const LID_FRAME_X: f64 = CASSETTE_X + 80.0;
const LID_FRAME_Y: f64 = CASSETTE_Y + 72.0;
const LID_FRAME_Z: f64 = 18.0;
const LID_NEST_X: f64 = 630.0;
const LID_NEST_Y: f64 = 370.0;
const LID_NEST_Z: f64 = 46.0;
const LID_NEST_CENTER: (f64, f64) = (-300.0, 145.0);
const LID_RECESS_DEPTH: f64 = 12.0;
const DATUM_PIN_COUNT: usize = 4;
const VACUUM_PAD_COUNT: usize = 6;
const LID_KEEPER_COUNT: usize = 8;

const PEEL_PANEL_X: f64 = 430.0;
const PEEL_PANEL_Y: f64 = 230.0;
const PEEL_PANEL_Z: f64 = 42.0;
const PEEL_CENTER: (f64, f64) = (425.0, 245.0);
const PEEL_LANE_COUNT: usize = 3;
const PEEL_TICK_COUNT: usize = 7;
const PEEL_ROLLER_COUNT: usize = 4;
const PEEL_ANGLE_TICK_PITCH: f64 = 22.0;

const SEAL_BANK_X: f64 = 320.0;
const SEAL_BANK_Y: f64 = 240.0;
const SEAL_BANK_Z: f64 = 42.0;
const SEAL_CENTER: (f64, f64) = (-535.0, -205.0);
const SEAL_WITNESS_COUNT: usize = 8;
const COMPRESSION_STEP_COUNT: usize = 6;
const SEAL_COUPON_X: f64 = 56.0;
const SEAL_COUPON_Y: f64 = 38.0;
const SEAL_COUPON_PITCH_X: f64 = 74.0;
const SEAL_COUPON_PITCH_Y: f64 = 64.0;
const COMPRESSION_FILM_SLOTS: usize = 4;

const FORCE_PANEL_X: f64 = 410.0;
const FORCE_PANEL_Y: f64 = 240.0;
const FORCE_PANEL_Z: f64 = 54.0;
const FORCE_CENTER: (f64, f64) = (-120.0, -205.0);
const PEEL_FORCE_GAUGE_X: f64 = 152.0;
const PEEL_FORCE_GAUGE_Y: f64 = 58.0;
const RESEAT_FORCE_GAUGE_X: f64 = 152.0;
const RESEAT_FORCE_GAUGE_Y: f64 = 68.0;
const FORCE_PLUNGER_COUNT: usize = 4;
const CAL_WEIGHT_COUNT: usize = 4;
const LOAD_CELL_CABLE_CHANNELS: usize = 2;

const PARTICLE_BANK_X: f64 = 430.0;
const PARTICLE_BANK_Y: f64 = 240.0;
const PARTICLE_BANK_Z: f64 = 40.0;
const PARTICLE_CENTER: (f64, f64) = (375.0, -205.0);
const PARTICLE_TROUGH_COUNT: usize = 4;
const FILTER_PAD_COUNT: usize = 8;
const WITNESS_SLIDE_COUNT: usize = 6;
const PARTICLE_PORT_COUNT: usize = 4;

const TOKEN_RAIL_X: f64 = 430.0;
const TOKEN_RAIL_Y: f64 = 92.0;
const TOKEN_RAIL_Z: f64 = 28.0;
const TOKEN_CENTER: (f64, f64) = (375.0, -386.0);
const OPEN_TIME_TOKEN_COUNT: usize = 12;
const TOKEN_PITCH_X: f64 = 31.0;
const TOKEN_D: f64 = 21.0;
const TIMER_WINDOW_COUNT: usize = 3;

const QUARANTINE_X: f64 = 410.0;
const QUARANTINE_Y: f64 = 92.0;
const QUARANTINE_Z: f64 = 34.0;
const QUARANTINE_CENTER: (f64, f64) = (-120.0, -386.0);
const DISPOSITION_LANE_COUNT: usize = 4;
const DISPOSITION_SLOT_COUNT: usize = 12;
const LANE_WALL_W: f64 = 7.0;

const CUSTODY_X: f64 = 320.0;
const CUSTODY_Y: f64 = 86.0;
const CUSTODY_Z: f64 = 12.0;
const CUSTODY_CENTER: (f64, f64) = (-535.0, -386.0);
const BARCODE_LAND_COUNT: usize = 10;
const CERTIFICATE_LAND_COUNT: usize = 4;
const CUSTODY_TOKEN_COUNT: usize = 8;

const CAMERA_BRIDGE_X: f64 = 1210.0;
const CAMERA_BRIDGE_Y: f64 = 64.0;
const CAMERA_BRIDGE_Z: f64 = 26.0;
const CAMERA_BRIDGE_CENTER: (f64, f64) = (0.0, 392.0);
const CAMERA_POST_Z: f64 = 198.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 2;
const EVIDENCE_FIDUCIAL_COUNT: usize = 10;

const KEEP_OUT_X: f64 = 1370.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;
const ROBOT_PEEL_CLEARANCE_Z: f64 = 285.0;
const ROBOT_RESEAT_CLEARANCE_Z: f64 = 250.0;
const FRONT_SERVICE_CLEARANCE: f64 = 130.0;
const REAR_CAMERA_CLEARANCE: f64 = 210.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_tray(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn clears(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx >= (self.x + other.x) / 2.0 + clearance || dy >= (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let nest = lid_reseat_cassette_nest();
    export(OUTPUTS[1], &nest);

    let peel = peel_path_witness_rails();
    export(OUTPUTS[2], &peel);

    let seal = seal_compression_witness_bank();
    export(OUTPUTS[3], &seal);

    let force = force_gauge_panel();
    export(OUTPUTS[4], &force);

    let particles = particle_shedding_capture_cassette();
    export(OUTPUTS[5], &particles);

    let tokens = open_time_token_clock_rail();
    export(OUTPUTS[6], &tokens);

    let quarantine = quarantine_disposition_lanes();
    export(OUTPUTS[7], &quarantine);

    let custody = barcode_custody_certificate_lands();
    export(OUTPUTS[8], &custody);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + nest.translate(LID_NEST_CENTER.0, LID_NEST_CENTER.1, module_z_offset())
        + peel.translate(PEEL_CENTER.0, PEEL_CENTER.1, module_z_offset())
        + seal.translate(SEAL_CENTER.0, SEAL_CENTER.1, module_z_offset())
        + force.translate(FORCE_CENTER.0, FORCE_CENTER.1, module_z_offset())
        + particles.translate(PARTICLE_CENTER.0, PARTICLE_CENTER.1, module_z_offset())
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, module_z_offset())
        + quarantine.translate(QUARANTINE_CENTER.0, QUARANTINE_CENTER.1, module_z_offset())
        + custody.translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, module_z_offset())
        + camera.translate(CAMERA_BRIDGE_CENTER.0, CAMERA_BRIDGE_CENTER.1, BASE_Z)
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_PAD_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed robotic lid peel/seal/reseat validation station:");
    println!("  Generator:              {OUTPUT_PREFIX}");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck with {RIM_Z:.0}mm rim"
    );
    println!(
        "  Cassette/lid nest:      {:.0}mm x {:.0}mm cassette envelope, {:.0}mm x {:.0}mm lid frame, {VACUUM_PAD_COUNT} vacuum witness pads",
        CASSETTE_X, CASSETTE_Y, LID_FRAME_X, LID_FRAME_Y
    );
    println!(
        "  Peel/reseat evidence:   {PEEL_LANE_COUNT} peel lanes, {PEEL_TICK_COUNT} angle ticks per lane, {FORCE_PLUNGER_COUNT} force plungers, {CAL_WEIGHT_COUNT} calibration pucks"
    );
    println!(
        "  Seal compression:       {SEAL_WITNESS_COUNT} witness coupons, {COMPRESSION_STEP_COUNT} height steps, {COMPRESSION_FILM_SLOTS} force-film slots"
    );
    println!(
        "  Particle capture:       {PARTICLE_TROUGH_COUNT} trough liners, {FILTER_PAD_COUNT} filter pads, {WITNESS_SLIDE_COUNT} witness slides, {PARTICLE_PORT_COUNT} sample ports"
    );
    println!(
        "  Open-time/quarantine:   {OPEN_TIME_TOKEN_COUNT} open-time tokens, {TIMER_WINDOW_COUNT} timer windows, {DISPOSITION_LANE_COUNT} disposition lanes with quarantine capacity"
    );
    println!(
        "  Evidence/keepouts:      {CAMERA_COUNT} camera pads, {LIGHT_BAR_COUNT} light bars, {KEEP_OUT_GAUGE_COUNT} keepout gauges, {ROBOT_PEEL_CLEARANCE_Z:.0}mm peel Z clearance"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn module_z_offset() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "stable STL output count changed");
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(DATUM_PIN_COUNT, 4);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert_eq!(MOUNT_HOLE_COUNT, 10);
    assert_eq!(DISPOSITION_LANE_COUNT, 4);
    assert_eq!(SEAL_WITNESS_COUNT, 8);
    assert_eq!(OPEN_TIME_TOKEN_COUNT, 12);
    assert!(CASSETTE_X > CASSETTE_COLS as f64 * REVC_CHIP_LENGTH);
    assert!(CASSETTE_Y > CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH);
    assert!(CASSETTE_Z > REVC_TOTAL_HEIGHT);
    assert!(LID_FRAME_X > CASSETTE_X + 70.0);
    assert!(LID_FRAME_Y > CASSETTE_Y + 60.0);
    assert!(ROBOT_PEEL_CLEARANCE_Z > CAMERA_POST_Z);
    assert!(ROBOT_RESEAT_CLEARANCE_Z > LID_NEST_Z + LID_FRAME_Z);
    assert!(FRONT_SERVICE_CLEARANCE >= 120.0);
    assert!(REAR_CAMERA_CLEARANCE >= 200.0);

    let rects = component_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_tray(),
            "{} does not fit inside the contained tray",
            rect.name
        );
    }
    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                left.clears(*right, MODULE_GAP),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn component_rects() -> [Rect; 8] {
    [
        rect(
            "lid_reseat_cassette_nest",
            LID_NEST_CENTER,
            LID_NEST_X,
            LID_NEST_Y,
        ),
        rect(
            "peel_path_witness_rails",
            PEEL_CENTER,
            PEEL_PANEL_X,
            PEEL_PANEL_Y,
        ),
        rect(
            "seal_compression_witness_bank",
            SEAL_CENTER,
            SEAL_BANK_X,
            SEAL_BANK_Y,
        ),
        rect(
            "force_gauge_panel",
            FORCE_CENTER,
            FORCE_PANEL_X,
            FORCE_PANEL_Y,
        ),
        rect(
            "particle_shedding_capture_cassette",
            PARTICLE_CENTER,
            PARTICLE_BANK_X,
            PARTICLE_BANK_Y,
        ),
        rect(
            "open_time_token_clock_rail",
            TOKEN_CENTER,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
        rect(
            "quarantine_disposition_lanes",
            QUARANTINE_CENTER,
            QUARANTINE_X,
            QUARANTINE_Y,
        ),
        rect(
            "barcode_custody_certificate_lands",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "lid_peel_reseat_station_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "lid_peel_reseat_station_wipeable_basin_recess",
        STATION_X - 132.0,
        STATION_Y - 122.0,
        BASIN_RECESS_Z + 0.4,
    )
    .translate(0.0, -8.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.2);
    let front_sump = centered_cube(
        "lid_peel_reseat_station_front_particle_sump",
        STATION_X - 240.0,
        38.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z - 4.0);
    let drain = centered_cylinder(
        "lid_peel_reseat_station_closed_drain_witness_port",
        5.0,
        54.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 78.0,
        -STATION_Y / 2.0 + 36.0,
        BASE_Z - 8.0,
    );

    deck - basin - front_sump - drain - insert_sockets() - mount_holes()
        + perimeter_rims()
        + zone_dividers()
        + base_datum_targets()
        + transfer_lips()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("lid_peel_reseat_station_insert_sockets");
    for footprint in component_rects() {
        sockets = sockets
            + centered_cube(
                format!("lid_peel_reseat_station_{}_socket", footprint.name),
                footprint.x + 10.0,
                footprint.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("lid_peel_reseat_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
        (-STATION_X / 2.0 + 58.0, -210.0),
        (STATION_X / 2.0 - 58.0, -210.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("lid_peel_reseat_station_m6_clearance_hole_{i}"),
                3.4,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("lid_peel_reseat_station_mount_slot_relief_{i}"),
                26.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "lid_peel_reseat_station_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "lid_peel_reseat_station_rear_camera_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "lid_peel_reseat_station_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "lid_peel_reseat_station_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn zone_dividers() -> Part {
    let top_lane = centered_cube(
        "lid_peel_reseat_station_lid_peel_to_reseat_zone_divider",
        STATION_X - 170.0,
        10.0,
        26.0,
    )
    .translate(0.0, 42.0, BASE_Z + 13.0);
    let lower_lane = centered_cube(
        "lid_peel_reseat_station_metrology_to_quarantine_zone_divider",
        STATION_X - 170.0,
        10.0,
        26.0,
    )
    .translate(0.0, -326.0, BASE_Z + 13.0);
    let center_split = centered_cube(
        "lid_peel_reseat_station_force_particle_zone_splitter",
        10.0,
        236.0,
        24.0,
    )
    .translate(126.0, -205.0, BASE_Z + 12.0);
    top_lane + lower_lane + center_split
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("lid_peel_reseat_station_base_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 96.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 96.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 96.0, STATION_Y / 2.0 - 96.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("lid_peel_reseat_station_robot_datum_boss_{i}"),
                13.0,
                7.0,
                36,
            )
            .translate(x, y, BASE_Z + 3.5)
            - centered_cylinder(
                format!("lid_peel_reseat_station_robot_datum_crosshair_{i}"),
                1.8,
                8.0,
                18,
            )
            .translate(x, y, BASE_Z + 4.0);
    }
    targets
}

fn transfer_lips() -> Part {
    let front = centered_cube(
        "lid_peel_reseat_station_front_robot_transfer_lip",
        STATION_X - 260.0,
        12.0,
        10.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 86.0, BASE_Z + 5.0);
    let right = centered_cube(
        "lid_peel_reseat_station_side_lid_tool_transfer_lip",
        12.0,
        STATION_Y - 260.0,
        10.0,
    )
    .translate(STATION_X / 2.0 - 90.0, 0.0, BASE_Z + 5.0);
    front + right
}

fn lid_reseat_cassette_nest() -> Part {
    let plate = centered_cube(
        "lid_peel_reseat_station_lid_reseat_nest_plate",
        LID_NEST_X,
        LID_NEST_Y,
        LID_NEST_Z,
    )
    .translate(0.0, 0.0, LID_NEST_Z / 2.0);
    let cassette_recess = centered_cube(
        "lid_peel_reseat_station_cassette_body_recess",
        CASSETTE_X + 8.0,
        CASSETTE_Y + 8.0,
        LID_RECESS_DEPTH + 0.4,
    )
    .translate(0.0, 0.0, LID_NEST_Z - LID_RECESS_DEPTH / 2.0 + 0.2);
    let lid_relief = centered_cube(
        "lid_peel_reseat_station_lid_frame_relief",
        LID_FRAME_X,
        LID_FRAME_Y,
        5.0,
    )
    .translate(0.0, 0.0, LID_NEST_Z - 2.5);

    plate - cassette_recess - lid_relief
        + cassette_locator_rails()
        + lid_keeper_tabs()
        + nest_datum_pins()
        + vacuum_witness_pads()
        + peel_start_lip()
        + reseat_hard_stops()
}

fn cassette_locator_rails() -> Part {
    let rail_z = 20.0;
    let rail_w = 14.0;
    let top = centered_cube(
        "lid_peel_reseat_station_cassette_rear_locator_rail",
        LID_FRAME_X + 18.0,
        rail_w,
        rail_z,
    )
    .translate(
        0.0,
        LID_FRAME_Y / 2.0 + rail_w / 2.0,
        LID_NEST_Z + rail_z / 2.0,
    );
    let bottom = centered_cube(
        "lid_peel_reseat_station_cassette_front_locator_rail",
        LID_FRAME_X + 18.0,
        rail_w,
        rail_z,
    )
    .translate(
        0.0,
        -LID_FRAME_Y / 2.0 - rail_w / 2.0,
        LID_NEST_Z + rail_z / 2.0,
    );
    let left = centered_cube(
        "lid_peel_reseat_station_cassette_left_locator_rail",
        rail_w,
        LID_FRAME_Y + 18.0,
        rail_z,
    )
    .translate(
        -LID_FRAME_X / 2.0 - rail_w / 2.0,
        0.0,
        LID_NEST_Z + rail_z / 2.0,
    );
    let right = centered_cube(
        "lid_peel_reseat_station_cassette_right_locator_rail",
        rail_w,
        LID_FRAME_Y + 18.0,
        rail_z,
    )
    .translate(
        LID_FRAME_X / 2.0 + rail_w / 2.0,
        0.0,
        LID_NEST_Z + rail_z / 2.0,
    );
    top + bottom + left + right
}

fn lid_keeper_tabs() -> Part {
    let mut tabs = Part::empty("lid_peel_reseat_station_lid_keeper_tabs");
    for i in 0..LID_KEEPER_COUNT {
        let side = if i < LID_KEEPER_COUNT / 2 { -1.0 } else { 1.0 };
        let local = i % (LID_KEEPER_COUNT / 2);
        let y = centered_index(local, LID_KEEPER_COUNT / 2, 64.0);
        tabs = tabs
            + centered_cube(
                format!("lid_peel_reseat_station_lid_keeper_tab_{i}"),
                38.0,
                16.0,
                18.0,
            )
            .translate(side * (LID_FRAME_X / 2.0 + 42.0), y, LID_NEST_Z + 24.0);
    }
    tabs
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("lid_peel_reseat_station_nest_datum_pins");
    for (i, (x, y)) in [
        (-LID_FRAME_X / 2.0 + 28.0, -LID_FRAME_Y / 2.0 + 26.0),
        (LID_FRAME_X / 2.0 - 28.0, -LID_FRAME_Y / 2.0 + 26.0),
        (-LID_FRAME_X / 2.0 + 28.0, LID_FRAME_Y / 2.0 - 26.0),
        (LID_FRAME_X / 2.0 - 28.0, LID_FRAME_Y / 2.0 - 26.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("lid_peel_reseat_station_lid_nest_datum_pin_boss_{i}"),
                11.0,
                9.0,
                32,
            )
            .translate(x, y, LID_NEST_Z + 4.5)
            - centered_cylinder(
                format!("lid_peel_reseat_station_lid_nest_pin_hole_{i}"),
                3.1,
                10.0,
                24,
            )
            .translate(x, y, LID_NEST_Z + 5.0);
    }
    pins
}

fn vacuum_witness_pads() -> Part {
    let mut pads = Part::empty("lid_peel_reseat_station_vacuum_witness_pads");
    for i in 0..VACUUM_PAD_COUNT {
        let x = centered_index(i % 3, 3, 112.0);
        let y = centered_index(i / 3, 2, 126.0);
        pads = pads
            + centered_cylinder(
                format!("lid_peel_reseat_station_vacuum_pad_witness_ring_{i}"),
                18.0,
                4.0,
                36,
            )
            .translate(x, y, LID_NEST_Z + 2.0)
            - centered_cylinder(
                format!("lid_peel_reseat_station_vacuum_pad_center_relief_{i}"),
                8.0,
                5.0,
                28,
            )
            .translate(x, y, LID_NEST_Z + 2.5);
    }
    pads
}

fn peel_start_lip() -> Part {
    centered_cube(
        "lid_peel_reseat_station_peel_start_lip_anvil",
        LID_FRAME_X - 60.0,
        18.0,
        22.0,
    )
    .translate(0.0, -LID_FRAME_Y / 2.0 - 48.0, LID_NEST_Z + 11.0)
}

fn reseat_hard_stops() -> Part {
    let mut stops = Part::empty("lid_peel_reseat_station_reseat_hard_stops");
    for i in 0..4 {
        let x = centered_index(i, 4, 96.0);
        stops = stops
            + centered_cube(
                format!("lid_peel_reseat_station_reseat_height_hard_stop_{i}"),
                34.0,
                18.0,
                14.0,
            )
            .translate(x, LID_FRAME_Y / 2.0 + 42.0, LID_NEST_Z + 7.0);
    }
    stops
}

fn peel_path_witness_rails() -> Part {
    let panel = centered_cube(
        "lid_peel_reseat_station_peel_path_panel",
        PEEL_PANEL_X,
        PEEL_PANEL_Y,
        PEEL_PANEL_Z,
    )
    .translate(0.0, 0.0, PEEL_PANEL_Z / 2.0);
    let lane_cuts = peel_lane_reliefs();

    panel - lane_cuts + peel_lanes() + peel_angle_ticks() + peel_rollers() + peel_tail_clamps()
}

fn peel_lane_reliefs() -> Part {
    let mut cuts = Part::empty("lid_peel_reseat_station_peel_lane_reliefs");
    for i in 0..PEEL_LANE_COUNT {
        let y = centered_index(i, PEEL_LANE_COUNT, 58.0);
        cuts = cuts
            + centered_cube(
                format!("lid_peel_reseat_station_peel_lane_recess_{i}"),
                PEEL_PANEL_X - 82.0,
                24.0,
                8.0,
            )
            .translate(0.0, y, PEEL_PANEL_Z - 4.0);
    }
    cuts
}

fn peel_lanes() -> Part {
    let mut lanes = Part::empty("lid_peel_reseat_station_peel_witness_lanes");
    for i in 0..PEEL_LANE_COUNT {
        let y = centered_index(i, PEEL_LANE_COUNT, 58.0);
        lanes = lanes
            + centered_cube(
                format!("lid_peel_reseat_station_peel_lane_left_rib_{i}"),
                PEEL_PANEL_X - 72.0,
                6.0,
                10.0,
            )
            .translate(0.0, y - 18.0, PEEL_PANEL_Z + 5.0)
            + centered_cube(
                format!("lid_peel_reseat_station_peel_lane_right_rib_{i}"),
                PEEL_PANEL_X - 72.0,
                6.0,
                10.0,
            )
            .translate(0.0, y + 18.0, PEEL_PANEL_Z + 5.0);
    }
    lanes
}

fn peel_angle_ticks() -> Part {
    let mut ticks = Part::empty("lid_peel_reseat_station_peel_angle_ticks");
    for lane in 0..PEEL_LANE_COUNT {
        let y = centered_index(lane, PEEL_LANE_COUNT, 58.0);
        for tick in 0..PEEL_TICK_COUNT {
            let x = centered_index(tick, PEEL_TICK_COUNT, PEEL_ANGLE_TICK_PITCH);
            let h = 4.0 + tick as f64 * 1.4;
            ticks = ticks
                + centered_cube(
                    format!("lid_peel_reseat_station_peel_angle_tick_lane_{lane}_{tick}"),
                    4.0,
                    22.0,
                    h,
                )
                .translate(x + 76.0, y, PEEL_PANEL_Z + h / 2.0);
        }
    }
    ticks
}

fn peel_rollers() -> Part {
    let mut rollers = Part::empty("lid_peel_reseat_station_peel_rollers");
    for i in 0..PEEL_ROLLER_COUNT {
        let x = centered_index(i, PEEL_ROLLER_COUNT, 72.0) - 108.0;
        rollers = rollers
            + centered_cylinder(
                format!("lid_peel_reseat_station_peel_low_friction_roller_{i}"),
                11.0,
                PEEL_PANEL_Y - 58.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, PEEL_PANEL_Z + 20.0);
    }
    rollers
}

fn peel_tail_clamps() -> Part {
    let left = centered_cube(
        "lid_peel_reseat_station_spent_lid_tail_left_clamp",
        34.0,
        PEEL_PANEL_Y - 62.0,
        28.0,
    )
    .translate(-PEEL_PANEL_X / 2.0 + 42.0, 0.0, PEEL_PANEL_Z + 14.0);
    let right = centered_cube(
        "lid_peel_reseat_station_spent_lid_tail_right_clamp",
        34.0,
        PEEL_PANEL_Y - 62.0,
        28.0,
    )
    .translate(PEEL_PANEL_X / 2.0 - 42.0, 0.0, PEEL_PANEL_Z + 14.0);
    left + right
}

fn seal_compression_witness_bank() -> Part {
    let bank = centered_cube(
        "lid_peel_reseat_station_seal_compression_bank_plate",
        SEAL_BANK_X,
        SEAL_BANK_Y,
        SEAL_BANK_Z,
    )
    .translate(0.0, 0.0, SEAL_BANK_Z / 2.0);

    bank - compression_film_slots()
        + seal_witness_frames()
        + compression_step_gauges()
        + seal_witness_datum_strip()
}

fn seal_witness_frames() -> Part {
    let mut frames = Part::empty("lid_peel_reseat_station_seal_witness_frames");
    for i in 0..SEAL_WITNESS_COUNT {
        let x = centered_index(i % 4, 4, SEAL_COUPON_PITCH_X);
        let y = centered_index(i / 4, 2, SEAL_COUPON_PITCH_Y) + 30.0;
        let outer = centered_cube(
            format!("lid_peel_reseat_station_seal_witness_coupon_outer_{i}"),
            SEAL_COUPON_X,
            SEAL_COUPON_Y,
            10.0,
        )
        .translate(x, y, SEAL_BANK_Z + 5.0);
        let inner = centered_cube(
            format!("lid_peel_reseat_station_seal_witness_coupon_window_{i}"),
            SEAL_COUPON_X - 18.0,
            SEAL_COUPON_Y - 14.0,
            11.0,
        )
        .translate(x, y, SEAL_BANK_Z + 5.5);
        frames = frames + (outer - inner);
    }
    frames
}

fn compression_step_gauges() -> Part {
    let mut steps = Part::empty("lid_peel_reseat_station_compression_step_gauges");
    for i in 0..COMPRESSION_STEP_COUNT {
        let height = 2.0 + i as f64 * 0.55;
        steps = steps
            + centered_cube(
                format!("lid_peel_reseat_station_compression_height_step_{i}"),
                26.0,
                54.0,
                height,
            )
            .translate(
                centered_index(i, COMPRESSION_STEP_COUNT, 38.0),
                -82.0,
                SEAL_BANK_Z + height / 2.0,
            );
    }
    steps
}

fn compression_film_slots() -> Part {
    let mut slots = Part::empty("lid_peel_reseat_station_compression_film_slots");
    for i in 0..COMPRESSION_FILM_SLOTS {
        slots = slots
            + centered_cube(
                format!("lid_peel_reseat_station_pressure_film_pull_slot_{i}"),
                46.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, COMPRESSION_FILM_SLOTS, 66.0),
                -33.0,
                SEAL_BANK_Z - 4.0,
            );
    }
    slots
}

fn seal_witness_datum_strip() -> Part {
    centered_cube(
        "lid_peel_reseat_station_seal_witness_datum_strip",
        SEAL_BANK_X - 44.0,
        10.0,
        14.0,
    )
    .translate(0.0, SEAL_BANK_Y / 2.0 - 28.0, SEAL_BANK_Z + 7.0)
}

fn force_gauge_panel() -> Part {
    let panel = centered_cube(
        "lid_peel_reseat_station_force_gauge_panel_plate",
        FORCE_PANEL_X,
        FORCE_PANEL_Y,
        FORCE_PANEL_Z,
    )
    .translate(0.0, 0.0, FORCE_PANEL_Z / 2.0);

    panel - force_gauge_pocket_cuts() - calibration_weight_wells()
        + force_gauge_saddles()
        + force_plungers_and_anvils()
        + load_cell_cable_channels()
}

fn force_gauge_pocket_cuts() -> Part {
    let peel = centered_cube(
        "lid_peel_reseat_station_peel_force_gauge_recess",
        PEEL_FORCE_GAUGE_X,
        PEEL_FORCE_GAUGE_Y,
        18.0,
    )
    .translate(-86.0, 50.0, FORCE_PANEL_Z - 9.0);
    let reseat = centered_cube(
        "lid_peel_reseat_station_reseat_force_gauge_recess",
        RESEAT_FORCE_GAUGE_X,
        RESEAT_FORCE_GAUGE_Y,
        18.0,
    )
    .translate(92.0, 50.0, FORCE_PANEL_Z - 9.0);
    peel + reseat
}

fn calibration_weight_wells() -> Part {
    let mut wells = Part::empty("lid_peel_reseat_station_calibration_weight_wells");
    for i in 0..CAL_WEIGHT_COUNT {
        wells = wells
            + centered_cylinder(
                format!("lid_peel_reseat_station_calibration_weight_well_{i}"),
                15.0,
                14.0,
                32,
            )
            .translate(
                centered_index(i, CAL_WEIGHT_COUNT, 44.0),
                -67.0,
                FORCE_PANEL_Z - 7.0,
            );
    }
    wells
}

fn force_gauge_saddles() -> Part {
    let peel = centered_cube(
        "lid_peel_reseat_station_peel_force_gauge_saddle",
        PEEL_FORCE_GAUGE_X + 32.0,
        PEEL_FORCE_GAUGE_Y + 24.0,
        12.0,
    )
    .translate(-86.0, 50.0, FORCE_PANEL_Z + 6.0);
    let reseat = centered_cube(
        "lid_peel_reseat_station_reseat_force_gauge_saddle",
        RESEAT_FORCE_GAUGE_X + 32.0,
        RESEAT_FORCE_GAUGE_Y + 24.0,
        12.0,
    )
    .translate(92.0, 50.0, FORCE_PANEL_Z + 6.0);
    peel + reseat
}

fn force_plungers_and_anvils() -> Part {
    let mut plungers = Part::empty("lid_peel_reseat_station_force_plungers_and_anvils");
    for i in 0..FORCE_PLUNGER_COUNT {
        let x = centered_index(i, FORCE_PLUNGER_COUNT, 72.0);
        plungers = plungers
            + centered_cylinder(
                format!("lid_peel_reseat_station_force_plunger_boss_{i}"),
                9.0,
                26.0,
                28,
            )
            .translate(x, -12.0, FORCE_PANEL_Z + 13.0)
            + centered_cylinder(
                format!("lid_peel_reseat_station_force_anvil_face_{i}"),
                18.0,
                5.0,
                36,
            )
            .translate(x, -12.0, FORCE_PANEL_Z + 28.5);
    }
    plungers
}

fn load_cell_cable_channels() -> Part {
    let mut channels = Part::empty("lid_peel_reseat_station_load_cell_cable_channels");
    for i in 0..LOAD_CELL_CABLE_CHANNELS {
        let y = 100.0 - i as f64 * 42.0;
        channels = channels
            + centered_cube(
                format!("lid_peel_reseat_station_load_cell_cable_channel_rib_{i}"),
                FORCE_PANEL_X - 70.0,
                7.0,
                8.0,
            )
            .translate(0.0, y, FORCE_PANEL_Z + 4.0);
    }
    channels
}

fn particle_shedding_capture_cassette() -> Part {
    let plate = centered_cube(
        "lid_peel_reseat_station_particle_capture_plate",
        PARTICLE_BANK_X,
        PARTICLE_BANK_Y,
        PARTICLE_BANK_Z,
    )
    .translate(0.0, 0.0, PARTICLE_BANK_Z / 2.0);

    plate - particle_trough_reliefs() - filter_pad_recesses()
        + particle_trough_liners()
        + witness_slide_rack()
        + particle_sample_ports()
}

fn particle_trough_reliefs() -> Part {
    let mut cuts = Part::empty("lid_peel_reseat_station_particle_trough_reliefs");
    for i in 0..PARTICLE_TROUGH_COUNT {
        cuts = cuts
            + centered_cube(
                format!("lid_peel_reseat_station_particle_trough_recess_{i}"),
                72.0,
                118.0,
                16.0,
            )
            .translate(
                centered_index(i, PARTICLE_TROUGH_COUNT, 88.0),
                38.0,
                PARTICLE_BANK_Z - 8.0,
            );
    }
    cuts
}

fn filter_pad_recesses() -> Part {
    let mut cuts = Part::empty("lid_peel_reseat_station_filter_pad_recesses");
    for i in 0..FILTER_PAD_COUNT {
        let x = centered_index(i % 4, 4, 82.0);
        let y = centered_index(i / 4, 2, 42.0) - 70.0;
        cuts = cuts
            + centered_cube(
                format!("lid_peel_reseat_station_filter_pad_recess_{i}"),
                50.0,
                22.0,
                8.0,
            )
            .translate(x, y, PARTICLE_BANK_Z - 4.0);
    }
    cuts
}

fn particle_trough_liners() -> Part {
    let mut liners = Part::empty("lid_peel_reseat_station_particle_trough_liners");
    for i in 0..PARTICLE_TROUGH_COUNT {
        let x = centered_index(i, PARTICLE_TROUGH_COUNT, 88.0);
        let frame = centered_cube(
            format!("lid_peel_reseat_station_particle_trough_liner_frame_{i}"),
            76.0,
            122.0,
            8.0,
        )
        .translate(x, 38.0, PARTICLE_BANK_Z + 4.0);
        let well = centered_cube(
            format!("lid_peel_reseat_station_particle_trough_liner_opening_{i}"),
            58.0,
            94.0,
            9.0,
        )
        .translate(x, 38.0, PARTICLE_BANK_Z + 4.5);
        liners = liners + (frame - well);
    }
    liners
}

fn witness_slide_rack() -> Part {
    let mut rack = Part::empty("lid_peel_reseat_station_witness_slide_rack");
    for i in 0..WITNESS_SLIDE_COUNT {
        rack = rack
            + centered_cube(
                format!("lid_peel_reseat_station_particle_witness_slide_slot_{i}"),
                46.0,
                8.0,
                14.0,
            )
            .translate(
                centered_index(i, WITNESS_SLIDE_COUNT, 58.0),
                -106.0,
                PARTICLE_BANK_Z + 7.0,
            );
    }
    rack
}

fn particle_sample_ports() -> Part {
    let mut ports = Part::empty("lid_peel_reseat_station_particle_sample_ports");
    for i in 0..PARTICLE_PORT_COUNT {
        let x = centered_index(i, PARTICLE_PORT_COUNT, 98.0);
        ports = ports
            + centered_cylinder(
                format!("lid_peel_reseat_station_particle_sample_port_boss_{i}"),
                15.0,
                10.0,
                32,
            )
            .translate(x, PARTICLE_BANK_Y / 2.0 - 26.0, PARTICLE_BANK_Z + 5.0)
            - centered_cylinder(
                format!("lid_peel_reseat_station_particle_sample_port_bore_{i}"),
                5.0,
                11.0,
                24,
            )
            .translate(x, PARTICLE_BANK_Y / 2.0 - 26.0, PARTICLE_BANK_Z + 5.5);
    }
    ports
}

fn open_time_token_clock_rail() -> Part {
    let rail = centered_cube(
        "lid_peel_reseat_station_open_time_token_clock_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(0.0, 0.0, TOKEN_RAIL_Z / 2.0);

    rail - token_wells() + token_retainer_lips() + timer_windows()
}

fn token_wells() -> Part {
    let mut wells = Part::empty("lid_peel_reseat_station_open_time_token_wells");
    for i in 0..OPEN_TIME_TOKEN_COUNT {
        wells = wells
            + centered_cylinder(
                format!("lid_peel_reseat_station_open_time_token_well_{i}"),
                TOKEN_D / 2.0,
                12.0,
                28,
            )
            .translate(
                centered_index(i, OPEN_TIME_TOKEN_COUNT, TOKEN_PITCH_X),
                18.0,
                TOKEN_RAIL_Z - 6.0,
            );
    }
    wells
}

fn token_retainer_lips() -> Part {
    let front = centered_cube(
        "lid_peel_reseat_station_open_time_token_front_retainer",
        TOKEN_RAIL_X - 36.0,
        7.0,
        9.0,
    )
    .translate(0.0, 38.0, TOKEN_RAIL_Z + 4.5);
    let rear = centered_cube(
        "lid_peel_reseat_station_open_time_token_rear_retainer",
        TOKEN_RAIL_X - 36.0,
        7.0,
        9.0,
    )
    .translate(0.0, -4.0, TOKEN_RAIL_Z + 4.5);
    front + rear
}

fn timer_windows() -> Part {
    let mut windows = Part::empty("lid_peel_reseat_station_timer_windows");
    for i in 0..TIMER_WINDOW_COUNT {
        let frame = centered_cube(
            format!("lid_peel_reseat_station_timer_window_frame_{i}"),
            88.0,
            26.0,
            9.0,
        )
        .translate(
            centered_index(i, TIMER_WINDOW_COUNT, 122.0),
            -30.0,
            TOKEN_RAIL_Z + 4.5,
        );
        let opening = centered_cube(
            format!("lid_peel_reseat_station_timer_window_opening_{i}"),
            64.0,
            12.0,
            10.0,
        )
        .translate(
            centered_index(i, TIMER_WINDOW_COUNT, 122.0),
            -30.0,
            TOKEN_RAIL_Z + 5.0,
        );
        windows = windows + (frame - opening);
    }
    windows
}

fn quarantine_disposition_lanes() -> Part {
    let plate = centered_cube(
        "lid_peel_reseat_station_quarantine_disposition_lane_plate",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0);

    plate - disposition_slot_recesses() + disposition_lane_walls() + quarantine_lock_bars()
}

fn disposition_slot_recesses() -> Part {
    let mut recesses = Part::empty("lid_peel_reseat_station_disposition_slot_recesses");
    for i in 0..DISPOSITION_SLOT_COUNT {
        let lane = i % DISPOSITION_LANE_COUNT;
        let row = i / DISPOSITION_LANE_COUNT;
        recesses = recesses
            + centered_cube(
                format!("lid_peel_reseat_station_disposition_token_recess_{i}"),
                46.0,
                16.0,
                10.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANE_COUNT, 86.0),
                centered_index(row, DISPOSITION_SLOT_COUNT / DISPOSITION_LANE_COUNT, 26.0),
                QUARANTINE_Z - 5.0,
            );
    }
    recesses
}

fn disposition_lane_walls() -> Part {
    let mut walls = Part::empty("lid_peel_reseat_station_disposition_lane_walls");
    for i in 0..=DISPOSITION_LANE_COUNT {
        let x = -((DISPOSITION_LANE_COUNT as f64) * 86.0) / 2.0 + i as f64 * 86.0;
        walls = walls
            + centered_cube(
                format!("lid_peel_reseat_station_disposition_lane_wall_{i}"),
                LANE_WALL_W,
                QUARANTINE_Y - 20.0,
                26.0,
            )
            .translate(x, 0.0, QUARANTINE_Z + 13.0);
    }
    walls
}

fn quarantine_lock_bars() -> Part {
    let front = centered_cube(
        "lid_peel_reseat_station_quarantine_front_lock_bar",
        QUARANTINE_X - 48.0,
        8.0,
        18.0,
    )
    .translate(0.0, -QUARANTINE_Y / 2.0 + 13.0, QUARANTINE_Z + 9.0);
    let rear = centered_cube(
        "lid_peel_reseat_station_quarantine_rear_lock_bar",
        QUARANTINE_X - 48.0,
        8.0,
        18.0,
    )
    .translate(0.0, QUARANTINE_Y / 2.0 - 13.0, QUARANTINE_Z + 9.0);
    front + rear
}

fn barcode_custody_certificate_lands() -> Part {
    let plate = centered_cube(
        "lid_peel_reseat_station_barcode_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);

    plate + barcode_lands() + certificate_lands() + custody_token_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("lid_peel_reseat_station_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("lid_peel_reseat_station_barcode_land_{i}"),
                46.0,
                13.0,
                3.0,
            )
            .translate(
                centered_index(i % 5, 5, 56.0),
                23.0 - (i / 5) as f64 * 22.0,
                CUSTODY_Z + 1.5,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("lid_peel_reseat_station_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("lid_peel_reseat_station_certificate_land_{i}"),
                54.0,
                17.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 68.0),
                -33.0,
                CUSTODY_Z + 1.5,
            );
    }
    lands
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty("lid_peel_reseat_station_custody_token_wells");
    for i in 0..CUSTODY_TOKEN_COUNT {
        wells = wells
            + centered_cylinder(
                format!("lid_peel_reseat_station_custody_token_boss_{i}"),
                7.5,
                4.0,
                22,
            )
            .translate(
                centered_index(i, CUSTODY_TOKEN_COUNT, 34.0),
                0.0,
                CUSTODY_Z + 2.0,
            );
    }
    wells
}

fn evidence_camera_bridge() -> Part {
    let mut bridge = Part::empty("lid_peel_reseat_station_evidence_camera_bridge");
    for (i, x) in [
        -CAMERA_BRIDGE_X / 2.0 + 42.0,
        CAMERA_BRIDGE_X / 2.0 - 42.0,
        -CAMERA_BRIDGE_X / 6.0,
        CAMERA_BRIDGE_X / 6.0,
    ]
    .into_iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("lid_peel_reseat_station_camera_bridge_post_{i}"),
                24.0,
                24.0,
                CAMERA_POST_Z,
            )
            .translate(x, 0.0, CAMERA_POST_Z / 2.0);
    }
    bridge + camera_beam() + camera_carriages() + light_bars() + evidence_fiducials()
}

fn camera_beam() -> Part {
    centered_cube(
        "lid_peel_reseat_station_camera_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(0.0, 0.0, CAMERA_POST_Z + CAMERA_BRIDGE_Z / 2.0)
}

fn camera_carriages() -> Part {
    let mut carriages = Part::empty("lid_peel_reseat_station_camera_carriages");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 250.0);
        carriages = carriages
            + centered_cube(
                format!("lid_peel_reseat_station_camera_carriage_{i}"),
                88.0,
                42.0,
                16.0,
            )
            .translate(x, 0.0, CAMERA_POST_Z + CAMERA_BRIDGE_Z + 8.0)
            + centered_cylinder(
                format!("lid_peel_reseat_station_camera_lens_clearance_{i}"),
                13.0,
                8.0,
                28,
            )
            .translate(x, 0.0, CAMERA_POST_Z + CAMERA_BRIDGE_Z - 4.0);
    }
    carriages
}

fn light_bars() -> Part {
    let mut bars = Part::empty("lid_peel_reseat_station_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        let y = if i == 0 { -25.0 } else { 25.0 };
        bars = bars
            + centered_cube(
                format!("lid_peel_reseat_station_evidence_light_bar_{i}"),
                CAMERA_BRIDGE_X - 180.0,
                8.0,
                8.0,
            )
            .translate(0.0, y, CAMERA_POST_Z + CAMERA_BRIDGE_Z + 6.0);
    }
    bars
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("lid_peel_reseat_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIAL_COUNT {
        fiducials = fiducials
            + centered_cylinder(
                format!("lid_peel_reseat_station_evidence_fiducial_{i}"),
                4.0,
                3.0,
                20,
            )
            .translate(
                centered_index(i, EVIDENCE_FIDUCIAL_COUNT, 98.0),
                -CAMERA_BRIDGE_Y / 2.0 - 20.0,
                CAMERA_POST_Z + 1.5,
            );
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "lid_peel_reseat_station_front_robot_keepout_strip",
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        "lid_peel_reseat_station_rear_camera_keepout_strip",
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        "lid_peel_reseat_station_left_service_keepout_strip",
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "lid_peel_reseat_station_right_lid_tool_keepout_strip",
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);

    front + rear + left + right + keepout_gauge_posts()
}

fn keepout_gauge_posts() -> Part {
    let mut posts = Part::empty("lid_peel_reseat_station_keepout_gauge_posts");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        let angle_index = i as f64 / KEEP_OUT_GAUGE_COUNT as f64;
        let x = -KEEP_OUT_X / 2.0 + 115.0 + angle_index * (KEEP_OUT_X - 230.0);
        let y = if i % 2 == 0 {
            -KEEP_OUT_Y / 2.0 + 32.0
        } else {
            KEEP_OUT_Y / 2.0 - 32.0
        };
        let height = 48.0 + i as f64 * 8.0;
        posts = posts
            + centered_cylinder(
                format!("lid_peel_reseat_station_robot_keepout_height_gauge_{i}"),
                8.0,
                height,
                24,
            )
            .translate(x, y, height / 2.0);
    }
    posts
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        assert_design_constraints();

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);

        let scoped_prefix = format!("output/{OUTPUT_PREFIX}_");
        for path in OUTPUTS {
            assert!(
                path.starts_with(scoped_prefix.as_str()),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_lid_peel_reseat_scope() {
        for feature in [
            "cassette_lid_reseat_nest",
            "robotic_peel_path_witness_rails",
            "seal_witness_compression_bank",
            "peel_force_gauge_pocket",
            "reseat_force_gauge_pocket",
            "particle_shedding_capture",
            "open_time_token_rail",
            "quarantine_disposition_lanes",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn cassette_and_lid_envelope_clear_revc_module() {
        assert_eq!(CASSETTE_COLS * CASSETTE_ROWS, 1);
        assert!(CASSETTE_X > CASSETTE_COLS as f64 * REVC_CHIP_LENGTH);
        assert!(CASSETTE_Y > CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH);
        assert!(CASSETTE_Z > REVC_TOTAL_HEIGHT);
        assert!(LID_FRAME_X > CASSETTE_X);
        assert!(LID_FRAME_Y > CASSETTE_Y);
        assert!(LID_NEST_X > LID_FRAME_X + 110.0);
        assert!(LID_NEST_Y > LID_FRAME_Y + 100.0);
    }

    #[test]
    fn seal_compression_and_force_evidence_are_explicit() {
        assert_eq!(SEAL_WITNESS_COUNT, 8);
        assert_eq!(COMPRESSION_STEP_COUNT, 6);
        assert_eq!(COMPRESSION_FILM_SLOTS, 4);
        assert_eq!(FORCE_PLUNGER_COUNT, 4);
        assert_eq!(CAL_WEIGHT_COUNT, 4);
        assert!(PEEL_FORCE_GAUGE_X < FORCE_PANEL_X / 2.0);
        assert!(RESEAT_FORCE_GAUGE_Y < FORCE_PANEL_Y / 2.0);
    }

    #[test]
    fn particle_capture_and_open_time_controls_are_present() {
        assert_eq!(PARTICLE_TROUGH_COUNT, 4);
        assert_eq!(FILTER_PAD_COUNT, 8);
        assert_eq!(WITNESS_SLIDE_COUNT, 6);
        assert_eq!(OPEN_TIME_TOKEN_COUNT, 12);
        assert_eq!(TIMER_WINDOW_COUNT, 3);
        assert!(TOKEN_RAIL_X > OPEN_TIME_TOKEN_COUNT as f64 * TOKEN_D);
    }

    #[test]
    fn quarantine_flow_has_release_hold_rework_and_reject_capacity() {
        assert_eq!(DISPOSITION_LANE_COUNT, 4);
        assert_eq!(DISPOSITION_SLOT_COUNT, 12);
        assert!(DISPOSITION_SLOT_COUNT >= DISPOSITION_LANE_COUNT * 3);
        assert!(QUARANTINE_Z > TOKEN_RAIL_Z);
    }

    #[test]
    fn station_regions_fit_and_do_not_collide() {
        let rects = component_rects();
        for rect in rects {
            assert!(rect.fits_inside_tray(), "{} does not fit", rect.name);
        }

        for (i, left) in rects.iter().enumerate() {
            for right in rects.iter().skip(i + 1) {
                assert!(
                    left.clears(*right, MODULE_GAP),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn robot_and_evidence_clearances_are_large_enough() {
        assert!(ROBOT_PEEL_CLEARANCE_Z > CAMERA_POST_Z);
        assert!(ROBOT_RESEAT_CLEARANCE_Z > LID_NEST_Z + LID_FRAME_Z);
        assert!(CAMERA_BRIDGE_X < STATION_X);
        assert!(CAMERA_BRIDGE_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 < STATION_Y / 2.0 - RIM_W);
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 6);
    }
}
