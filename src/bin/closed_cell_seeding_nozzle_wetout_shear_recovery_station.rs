use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-seeding nozzle wetout/shear/recovery validation station.
//
// Intent:
// - Run no-cell, closed-fluid validation before automated tissue-chip seeding.
// - Catch nozzle wetout state, transient shear spikes, droplet/film retention,
//   recovery volume, and cross-lane carryover before a live tissue-chip run.
// - Keep nozzle nests, witness coupons, restriction lanes, vial recovery nests,
//   carryover challenges, pressure/flow coupon pockets, barcode custody lands,
//   camera evidence geometry, and robot/service keepout gauges in one contained
//   station that supports automated multi-chip seeding without manual pipetting.
//
// This is architecture CAD only. It is not a sterile barrier design, validated
// wetted path, biological acceptance method, or pump/nozzle operating recipe.

const OUTPUT_PREFIX: &str = "closed_cell_seeding_nozzle_wetout_shear_recovery_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_containment_base.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_nozzle_nest_array.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_wetout_witness_coupon_grid.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_shear_recovery_restriction_lane_bank.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_droplet_film_retention_pads.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_flush_recovery_vial_nests.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_carryover_challenge_lanes.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_pressure_flow_sensor_coupon_pockets.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_barcode_custody_lands.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_camera_evidence_bridge.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_robot_service_keepout_gauges.stl",
    "output/closed_cell_seeding_nozzle_wetout_shear_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "containment_base",
    "nozzle_nest_array",
    "wetout_witness_coupon_grid",
    "shear_recovery_restriction_lane_bank",
    "droplet_film_retention_pads",
    "flush_recovery_vial_nests",
    "carryover_challenge_lanes",
    "pressure_flow_sensor_coupon_pockets",
    "barcode_custody_lands",
    "camera_evidence_bridge",
    "robot_service_keepout_gauges",
    "assembly_export",
];

const STATION_X: f64 = 1340.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_BOSS_D: f64 = 30.0;
const MOUNT_HOLE_D: f64 = 6.6;
const SOCKET_DEPTH: f64 = 5.0;

const LANES: usize = 8;
const LANE_PITCH_X: f64 = 62.0;
const NOZZLE_ROWS: usize = 2;
const NOZZLES_PER_ROW: usize = LANES;
const NOZZLE_COUNT: usize = NOZZLE_ROWS * NOZZLES_PER_ROW;

const NOZZLE_PANEL_X: f64 = 610.0;
const NOZZLE_PANEL_Y: f64 = 190.0;
const NOZZLE_PANEL_Z: f64 = 52.0;
const NOZZLE_POS: (f64, f64) = (-335.0, 225.0);
const NOZZLE_BORE_D: f64 = 9.0;
const NOZZLE_COLLAR_D: f64 = 24.0;
const NOZZLE_ROW_PITCH_Y: f64 = 78.0;
const NOZZLE_CLOCKING_KEY_X: f64 = 12.0;

const WETOUT_GRID_X: f64 = 530.0;
const WETOUT_GRID_Y: f64 = 190.0;
const WETOUT_GRID_Z: f64 = 26.0;
const WETOUT_POS: (f64, f64) = (360.0, 225.0);
const WETOUT_COUPON_COLS: usize = 8;
const WETOUT_COUPON_ROWS: usize = 3;
const WETOUT_COUPONS: usize = WETOUT_COUPON_COLS * WETOUT_COUPON_ROWS;
const WETOUT_COUPON_X: f64 = 42.0;
const WETOUT_COUPON_Y: f64 = 28.0;
const WETOUT_COUPON_PITCH_X: f64 = 56.0;
const WETOUT_COUPON_PITCH_Y: f64 = 48.0;

const RESTRICTION_BANK_X: f64 = 700.0;
const RESTRICTION_BANK_Y: f64 = 180.0;
const RESTRICTION_BANK_Z: f64 = 44.0;
const RESTRICTION_POS: (f64, f64) = (-280.0, 25.0);
const RESTRICTION_LANE_W: f64 = 11.0;
const RESTRICTION_CHANNEL_Z: f64 = 12.0;
const RECOVERY_BULB_D: f64 = 24.0;
const SPIKE_TOKEN_COUNT: usize = LANES;

const RETENTION_PANEL_X: f64 = 360.0;
const RETENTION_PANEL_Y: f64 = 170.0;
const RETENTION_PANEL_Z: f64 = 22.0;
const RETENTION_POS: (f64, f64) = (450.0, 25.0);
const RETENTION_PADS: usize = LANES;
const RETENTION_PAD_D: f64 = 28.0;
const FILM_RAKE_TEETH: usize = 9;

const VIAL_BANK_X: f64 = 610.0;
const VIAL_BANK_Y: f64 = 150.0;
const VIAL_BANK_Z: f64 = 48.0;
const VIAL_POS: (f64, f64) = (-330.0, -205.0);
const RECOVERY_VIALS: usize = LANES;
const VIAL_WELL_D: f64 = 32.0;
const VIAL_WELL_DEPTH: f64 = 34.0;
const FLUSH_PORT_D: f64 = 6.2;
const VIAL_PITCH_X: f64 = 66.0;

const CARRYOVER_X: f64 = 380.0;
const CARRYOVER_Y: f64 = 150.0;
const CARRYOVER_Z: f64 = 34.0;
const CARRYOVER_POS: (f64, f64) = (185.0, -205.0);
const CARRYOVER_LANES: usize = 4;
const CARRYOVER_SLOTS_PER_LANE: usize = 5;
const CARRYOVER_SLOT_X: f64 = 46.0;
const CARRYOVER_SLOT_Y: f64 = 18.0;
const CARRYOVER_LANE_PITCH_Y: f64 = 34.0;

const SENSOR_PANEL_X: f64 = 260.0;
const SENSOR_PANEL_Y: f64 = 150.0;
const SENSOR_PANEL_Z: f64 = 40.0;
const SENSOR_POS: (f64, f64) = (505.0, -205.0);
const SENSOR_COUPONS: usize = 6;
const SENSOR_POCKET_X: f64 = 32.0;
const SENSOR_POCKET_Y: f64 = 24.0;
const SENSOR_POCKET_Z: f64 = 18.0;
const PRESSURE_TAP_D: f64 = 4.4;
const FLOW_WINDOW_X: f64 = 28.0;

const CUSTODY_PANEL_X: f64 = 350.0;
const CUSTODY_PANEL_Y: f64 = 102.0;
const CUSTODY_PANEL_Z: f64 = 10.0;
const CUSTODY_POS: (f64, f64) = (-430.0, -350.0);
const BARCODE_LANDS: usize = LANES;
const CUSTODY_LAND_X: f64 = 54.0;
const CUSTODY_LAND_Y: f64 = 16.0;
const CERTIFICATE_LANDS: usize = 3;

const BRIDGE_X: f64 = 1160.0;
const BRIDGE_Y: f64 = 66.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_CLEARANCE_Z: f64 = 210.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_PODS: usize = 4;
const LIGHT_BARS: usize = 3;
const BRIDGE_POS: (f64, f64) = (0.0, 55.0);

const KEEP_OUT_WINDOWS: usize = 4;
const ROBOT_FRONT_CLEARANCE: f64 = 390.0;
const SERVICE_REAR_CLEARANCE: f64 = 250.0;
const NOZZLE_SERVICE_CLEARANCE: f64 = 210.0;
const SENSOR_SERVICE_CLEARANCE: f64 = 180.0;
const KEEP_OUT_Z: f64 = 6.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 10.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 10.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
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

    let base = containment_base();
    export(OUTPUTS[0], &base);

    let nozzle_nests = nozzle_nest_array();
    export(OUTPUTS[1], &nozzle_nests);

    let wetout = wetout_witness_coupon_grid();
    export(OUTPUTS[2], &wetout);

    let shear = shear_recovery_restriction_lane_bank();
    export(OUTPUTS[3], &shear);

    let retention = droplet_film_retention_pads();
    export(OUTPUTS[4], &retention);

    let vials = flush_recovery_vial_nests();
    export(OUTPUTS[5], &vials);

    let carryover = carryover_challenge_lanes();
    export(OUTPUTS[6], &carryover);

    let sensors = pressure_flow_sensor_coupon_pockets();
    export(OUTPUTS[7], &sensors);

    let custody = barcode_custody_lands();
    export(OUTPUTS[8], &custody);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + nozzle_nests
        + wetout
        + shear
        + retention
        + vials
        + carryover
        + sensors
        + custody
        + bridge
        + keepouts
        + closed_fluid_route_witnesses();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cell-seeding nozzle wetout/shear/recovery validation station:");
    println!(
        "  Containment:          {STATION_X:.0}mm x {STATION_Y:.0}mm closed leak tray, {RIM_Z:.0}mm raised rim, {DRAIN_D:.0}mm drain witness"
    );
    println!(
        "  Nozzle validation:    {NOZZLE_COUNT} no-cell nozzle nests in {NOZZLE_ROWS} rows, {WETOUT_COUPONS} wetout witness coupons, {RETENTION_PADS} droplet/film retention pads"
    );
    println!(
        "  Shear/recovery:       {LANES} restriction lanes with recovery bulbs, {RECOVERY_VIALS} recovery vial nests, and {SPIKE_TOKEN_COUNT} transient spike tokens"
    );
    println!(
        "  Carryover challenge:  {CARRYOVER_LANES} segregated challenge lanes x {CARRYOVER_SLOTS_PER_LANE} standard positions"
    );
    println!(
        "  Instrument evidence:  {SENSOR_COUPONS} pressure/flow sensor coupon pockets, {CAMERA_PODS} camera pods, {LIGHT_BARS} light bars, {BARCODE_LANDS} barcode lands"
    );
    println!(
        "  Automation clearance: front robot {ROBOT_FRONT_CLEARANCE:.0}mm, rear service {SERVICE_REAR_CLEARANCE:.0}mm, nozzle service {NOZZLE_SERVICE_CLEARANCE:.0}mm, sensor service {SENSOR_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(NOZZLE_COUNT, NOZZLE_ROWS * NOZZLES_PER_ROW);
    assert_eq!(NOZZLES_PER_ROW, LANES);
    assert!(WETOUT_COUPONS >= NOZZLE_COUNT);
    assert!(RECOVERY_VIALS >= LANES);
    assert!(RETENTION_PADS >= LANES);
    assert!(CARRYOVER_LANE_PITCH_Y > CARRYOVER_SLOT_Y);
    assert!(BRIDGE_CLEARANCE_Z > BASE_Z + NOZZLE_PANEL_Z + 80.0);
    assert!(ROBOT_FRONT_CLEARANCE > 300.0);
    assert!(SERVICE_REAR_CLEARANCE > 200.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds tray envelope",
            rect.name
        );
    }

    for (left_index, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(left_index + 1) {
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }

    for output in OUTPUTS {
        assert!(output.starts_with("output/"));
        assert!(output.ends_with(".stl"));
        assert!(output.contains(OUTPUT_PREFIX));
    }
    assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
}

fn layout_rects() -> [Rect; 8] {
    [
        rect(
            "nozzle_nest_array",
            NOZZLE_POS,
            NOZZLE_PANEL_X,
            NOZZLE_PANEL_Y,
        ),
        rect(
            "wetout_witness_coupon_grid",
            WETOUT_POS,
            WETOUT_GRID_X,
            WETOUT_GRID_Y,
        ),
        rect(
            "shear_recovery_restriction_lane_bank",
            RESTRICTION_POS,
            RESTRICTION_BANK_X,
            RESTRICTION_BANK_Y,
        ),
        rect(
            "droplet_film_retention_pads",
            RETENTION_POS,
            RETENTION_PANEL_X,
            RETENTION_PANEL_Y,
        ),
        rect(
            "flush_recovery_vial_nests",
            VIAL_POS,
            VIAL_BANK_X,
            VIAL_BANK_Y,
        ),
        rect(
            "carryover_challenge_lanes",
            CARRYOVER_POS,
            CARRYOVER_X,
            CARRYOVER_Y,
        ),
        rect(
            "pressure_flow_sensor_coupon_pockets",
            SENSOR_POS,
            SENSOR_PANEL_X,
            SENSOR_PANEL_Y,
        ),
        rect(
            "barcode_custody_lands",
            CUSTODY_POS,
            CUSTODY_PANEL_X,
            CUSTODY_PANEL_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn containment_base() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_no_cell_leak_basin_recess"),
        STATION_X - 120.0,
        STATION_Y - 120.0,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_recovery_drain_port"),
        DRAIN_D / 2.0,
        72.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 118.0,
        -STATION_Y / 2.0 + 14.0,
        BASE_Z - 4.0,
    );

    deck - basin - drain - component_sockets()
        + containment_rims()
        + mount_bosses()
        + wet_dry_zone_dividers()
        + leak_sensor_pucks()
        + robot_datum_fiducials()
}

fn component_sockets() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_component_socket_recesses"));
    for rect in layout_rects().iter().take(7) {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_socket_recess", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn containment_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_containment_lip"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z_on_base(RIM_Z));
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z_on_base(RIM_Z));
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z_on_base(RIM_Z));
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z_on_base(RIM_Z));

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_bosses"));
    for (index, (x, y)) in mount_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_mount_boss_{index}"),
            MOUNT_BOSS_D / 2.0,
            9.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            11.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn wet_dry_zone_dividers() -> Part {
    let wet_to_dry = centered_cube(
        format!("{OUTPUT_PREFIX}_wet_validation_to_dry_custody_divider"),
        12.0,
        150.0,
        24.0,
    )
    .translate(-245.0, 350.0, z_on_base(24.0));
    let nozzle_to_witness = centered_cube(
        format!("{OUTPUT_PREFIX}_nozzle_to_witness_keepaway_rib"),
        10.0,
        205.0,
        26.0,
    )
    .translate(10.0, 225.0, z_on_base(26.0));
    let upper_lower = centered_cube(
        format!("{OUTPUT_PREFIX}_upper_lower_validation_zone_divider"),
        STATION_X - 190.0,
        8.0,
        20.0,
    )
    .translate(0.0, 128.0, z_on_base(20.0));
    let challenge_sensor = centered_cube(
        format!("{OUTPUT_PREFIX}_carryover_sensor_keepaway_rib"),
        8.0,
        154.0,
        20.0,
    )
    .translate(345.0, -205.0, z_on_base(20.0));

    wet_to_dry + nozzle_to_witness + upper_lower + challenge_sensor
}

fn leak_sensor_pucks() -> Part {
    let mut pucks = Part::empty(format!("{OUTPUT_PREFIX}_leak_sensor_pucks"));
    for index in 0..5 {
        let x = centered_index(index, 5, 190.0);
        let puck = centered_cylinder(
            format!("{OUTPUT_PREFIX}_leak_sensor_puck_{index}"),
            14.0,
            5.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z + 2.5);
        let well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_leak_sensor_wetness_well_{index}"),
            6.0,
            6.0,
            24,
        )
        .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z + 2.5);
        pucks = pucks + (puck - well);
    }
    pucks
}

fn robot_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{OUTPUT_PREFIX}_robot_datum_fiducials"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 76.0, -STATION_Y / 2.0 + 76.0),
        (STATION_X / 2.0 - 76.0, -STATION_Y / 2.0 + 76.0),
        (-STATION_X / 2.0 + 76.0, STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 76.0, STATION_Y / 2.0 - 76.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("{OUTPUT_PREFIX}_robot_datum_disc_{index}"),
            12.0,
            4.0,
            32,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        let center = centered_cylinder(
            format!("{OUTPUT_PREFIX}_robot_datum_center_bore_{index}"),
            3.0,
            5.0,
            20,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn nozzle_nest_array() -> Part {
    let plate = centered_cube(
        format!("{OUTPUT_PREFIX}_nozzle_nest_array_plate"),
        NOZZLE_PANEL_X,
        NOZZLE_PANEL_Y,
        NOZZLE_PANEL_Z,
    )
    .translate(NOZZLE_POS.0, NOZZLE_POS.1, z_on_base(NOZZLE_PANEL_Z));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_nozzle_nest_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_nozzle_nest_features"));

    for row in 0..NOZZLE_ROWS {
        let y = NOZZLE_POS.1 + centered_index(row, NOZZLE_ROWS, NOZZLE_ROW_PITCH_Y);
        for col in 0..NOZZLES_PER_ROW {
            let lane = row * NOZZLES_PER_ROW + col;
            let x = NOZZLE_POS.0 + centered_index(col, NOZZLES_PER_ROW, LANE_PITCH_X);
            let bore = centered_cylinder(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_vertical_wetout_bore"),
                NOZZLE_BORE_D / 2.0,
                NOZZLE_PANEL_Z + 8.0,
                32,
            )
            .translate(x, y, z_on_base(NOZZLE_PANEL_Z));
            let cone = centered_cylinder(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_flared_tip_witness_clearance"),
                NOZZLE_COLLAR_D / 2.0,
                11.0,
                36,
            )
            .translate(x, y, BASE_Z + NOZZLE_PANEL_Z - 4.0);
            let clocking = centered_cube(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_clocking_key_cut"),
                NOZZLE_CLOCKING_KEY_X,
                38.0,
                NOZZLE_PANEL_Z + 6.0,
            )
            .translate(
                x + NOZZLE_COLLAR_D / 2.0 - 3.0,
                y,
                z_on_base(NOZZLE_PANEL_Z),
            );
            let collar = centered_cylinder(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_raised_nozzle_collaring_land"),
                NOZZLE_COLLAR_D / 2.0 + 5.0,
                5.0,
                36,
            )
            .translate(x, y, BASE_Z + NOZZLE_PANEL_Z + 2.5);
            let collar_void = centered_cylinder(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_collar_center_void"),
                NOZZLE_COLLAR_D / 2.0,
                6.0,
                36,
            )
            .translate(x, y, BASE_Z + NOZZLE_PANEL_Z + 2.5);
            let latch = centered_cube(
                format!("{OUTPUT_PREFIX}_nozzle_{lane}_spring_clip_latch_land"),
                28.0,
                6.0,
                9.0,
            )
            .translate(x, y - 28.0, BASE_Z + NOZZLE_PANEL_Z + 4.5);

            cuts = cuts + bore + cone + clocking;
            features = features + (collar - collar_void) + latch;
        }
    }

    plate - cuts + features + nozzle_lane_index_ticks()
}

fn nozzle_lane_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_nozzle_lane_index_ticks"));
    for lane in 0..=NOZZLES_PER_ROW {
        let x =
            NOZZLE_POS.0 - (NOZZLES_PER_ROW as f64 - 1.0) * LANE_PITCH_X / 2.0 - LANE_PITCH_X / 2.0
                + lane as f64 * LANE_PITCH_X;
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_nozzle_lane_tick_{lane}"),
                3.0,
                NOZZLE_PANEL_Y - 22.0,
                7.0,
            )
            .translate(x, NOZZLE_POS.1, BASE_Z + NOZZLE_PANEL_Z + 3.5);
    }
    ticks
}

fn wetout_witness_coupon_grid() -> Part {
    let plate = centered_cube(
        format!("{OUTPUT_PREFIX}_wetout_witness_coupon_grid_plate"),
        WETOUT_GRID_X,
        WETOUT_GRID_Y,
        WETOUT_GRID_Z,
    )
    .translate(WETOUT_POS.0, WETOUT_POS.1, z_on_base(WETOUT_GRID_Z));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_wetout_coupon_pocket_cutters"));
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_wetout_coupon_rails"));

    for row in 0..WETOUT_COUPON_ROWS {
        let y = WETOUT_POS.1 + centered_index(row, WETOUT_COUPON_ROWS, WETOUT_COUPON_PITCH_Y);
        for col in 0..WETOUT_COUPON_COLS {
            let index = row * WETOUT_COUPON_COLS + col;
            let x = WETOUT_POS.0 + centered_index(col, WETOUT_COUPON_COLS, WETOUT_COUPON_PITCH_X);
            let pocket = centered_cube(
                format!("{OUTPUT_PREFIX}_wetout_coupon_{index}_recess"),
                WETOUT_COUPON_X,
                WETOUT_COUPON_Y,
                8.0,
            )
            .translate(x, y, BASE_Z + WETOUT_GRID_Z - 3.5);
            let witness_dot = centered_cylinder(
                format!("{OUTPUT_PREFIX}_wetout_coupon_{index}_drop_center_target"),
                5.5,
                3.0,
                24,
            )
            .translate(x, y, BASE_Z + WETOUT_GRID_Z + 1.5);
            let meniscus_bar = centered_cube(
                format!("{OUTPUT_PREFIX}_wetout_coupon_{index}_meniscus_edge_bar"),
                WETOUT_COUPON_X - 8.0,
                3.0,
                4.0,
            )
            .translate(
                x,
                y + WETOUT_COUPON_Y / 2.0 + 4.0,
                BASE_Z + WETOUT_GRID_Z + 2.0,
            );

            cuts = cuts + pocket;
            rails = rails + witness_dot + meniscus_bar;
        }
    }

    plate - cuts + rails
}

fn shear_recovery_restriction_lane_bank() -> Part {
    let bank = centered_cube(
        format!("{OUTPUT_PREFIX}_shear_recovery_restriction_lane_bank_body"),
        RESTRICTION_BANK_X,
        RESTRICTION_BANK_Y,
        RESTRICTION_BANK_Z,
    )
    .translate(
        RESTRICTION_POS.0,
        RESTRICTION_POS.1,
        z_on_base(RESTRICTION_BANK_Z),
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_restriction_lane_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_restriction_lane_features"));

    for lane in 0..LANES {
        let x = RESTRICTION_POS.0 + centered_index(lane, LANES, LANE_PITCH_X);
        let restriction = centered_cube(
            format!("{OUTPUT_PREFIX}_lane_{lane}_narrow_high_shear_restriction"),
            RESTRICTION_LANE_W,
            RESTRICTION_BANK_Y + 8.0,
            RESTRICTION_CHANNEL_Z,
        )
        .translate(x, RESTRICTION_POS.1, BASE_Z + RESTRICTION_BANK_Z - 9.0);
        let recovery_bulb = centered_cylinder(
            format!("{OUTPUT_PREFIX}_lane_{lane}_post_shear_recovery_volume_bulb"),
            RECOVERY_BULB_D / 2.0,
            RESTRICTION_CHANNEL_Z + 2.0,
            34,
        )
        .translate(
            x,
            RESTRICTION_POS.1 - 48.0,
            BASE_Z + RESTRICTION_BANK_Z - 8.0,
        );
        let inlet_header = centered_cylinder(
            format!("{OUTPUT_PREFIX}_lane_{lane}_upstream_pressure_tap_bore"),
            PRESSURE_TAP_D / 2.0,
            RESTRICTION_BANK_Z + 5.0,
            18,
        )
        .translate(x, RESTRICTION_POS.1 + 58.0, z_on_base(RESTRICTION_BANK_Z));
        let spike_token = centered_cube(
            format!("{OUTPUT_PREFIX}_lane_{lane}_transient_shear_spike_token"),
            24.0,
            8.0,
            8.0,
        )
        .translate(
            x,
            RESTRICTION_POS.1 + 80.0,
            BASE_Z + RESTRICTION_BANK_Z + 4.0,
        );
        let recovery_tick = centered_cube(
            format!("{OUTPUT_PREFIX}_lane_{lane}_recovery_decay_tick"),
            4.0,
            52.0,
            6.0,
        )
        .translate(
            x + 19.0,
            RESTRICTION_POS.1 - 42.0,
            BASE_Z + RESTRICTION_BANK_Z + 3.0,
        );

        cuts = cuts + restriction + recovery_bulb + inlet_header;
        features = features + spike_token + recovery_tick;
    }

    let upstream_header = centered_cylinder(
        format!("{OUTPUT_PREFIX}_common_upstream_closed_header_placeholder"),
        5.0,
        RESTRICTION_BANK_X - 60.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(RESTRICTION_POS.0, RESTRICTION_POS.1 + 70.0, BASE_Z + 22.0);
    let downstream_header = centered_cylinder(
        format!("{OUTPUT_PREFIX}_common_downstream_recovery_header_placeholder"),
        7.0,
        RESTRICTION_BANK_X - 60.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(RESTRICTION_POS.0, RESTRICTION_POS.1 - 70.0, BASE_Z + 20.0);

    bank - cuts + features + upstream_header + downstream_header
}

fn droplet_film_retention_pads() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_droplet_film_retention_pad_panel"),
        RETENTION_PANEL_X,
        RETENTION_PANEL_Y,
        RETENTION_PANEL_Z,
    )
    .translate(
        RETENTION_POS.0,
        RETENTION_POS.1,
        z_on_base(RETENTION_PANEL_Z),
    );
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_droplet_retention_pad_features"));

    for index in 0..RETENTION_PADS {
        let x = RETENTION_POS.0 + centered_index(index, RETENTION_PADS, 39.0);
        let y = RETENTION_POS.1 + if index % 2 == 0 { -26.0 } else { 26.0 };
        let pad = centered_cylinder(
            format!("{OUTPUT_PREFIX}_droplet_retention_absorbent_pad_{index}"),
            RETENTION_PAD_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z + RETENTION_PANEL_Z + 2.5);
        let pad_void = centered_cylinder(
            format!("{OUTPUT_PREFIX}_droplet_retention_pad_center_witness_void_{index}"),
            RETENTION_PAD_D / 2.0 - 6.0,
            6.0,
            36,
        )
        .translate(x, y, BASE_Z + RETENTION_PANEL_Z + 2.5);
        let film_track = centered_cube(
            format!("{OUTPUT_PREFIX}_droplet_film_creep_track_{index}"),
            30.0,
            5.0,
            4.0,
        )
        .translate(x, y + 28.0, BASE_Z + RETENTION_PANEL_Z + 2.0);
        pads = pads + (pad - pad_void) + film_track;
    }

    panel + pads + film_rake()
}

fn film_rake() -> Part {
    let mut rake = Part::empty(format!("{OUTPUT_PREFIX}_film_rake_evaporation_teeth"));
    for tooth in 0..FILM_RAKE_TEETH {
        rake = rake
            + centered_cube(
                format!("{OUTPUT_PREFIX}_film_retention_rake_tooth_{tooth}"),
                8.0,
                54.0,
                6.0,
            )
            .translate(
                RETENTION_POS.0 + centered_index(tooth, FILM_RAKE_TEETH, 30.0),
                RETENTION_POS.1,
                BASE_Z + RETENTION_PANEL_Z + 3.0,
            );
    }
    rake
}

fn flush_recovery_vial_nests() -> Part {
    let bank = centered_cube(
        format!("{OUTPUT_PREFIX}_flush_recovery_vial_nest_bank"),
        VIAL_BANK_X,
        VIAL_BANK_Y,
        VIAL_BANK_Z,
    )
    .translate(VIAL_POS.0, VIAL_POS.1, z_on_base(VIAL_BANK_Z));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_flush_recovery_vial_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_flush_recovery_vial_features"));

    for lane in 0..RECOVERY_VIALS {
        let x = VIAL_POS.0 + centered_index(lane, RECOVERY_VIALS, VIAL_PITCH_X);
        let vial = centered_cylinder(
            format!("{OUTPUT_PREFIX}_recovery_vial_{lane}_graduated_well"),
            VIAL_WELL_D / 2.0,
            VIAL_WELL_DEPTH + 2.0,
            44,
        )
        .translate(
            x,
            VIAL_POS.1 - 22.0,
            BASE_Z + VIAL_BANK_Z - VIAL_WELL_DEPTH / 2.0,
        );
        let flush_port = centered_cylinder(
            format!("{OUTPUT_PREFIX}_flush_port_{lane}_closed_line_bore"),
            FLUSH_PORT_D / 2.0,
            VIAL_BANK_Z + 5.0,
            20,
        )
        .translate(x, VIAL_POS.1 + 42.0, z_on_base(VIAL_BANK_Z));
        let vial_collar = centered_cylinder(
            format!("{OUTPUT_PREFIX}_recovery_vial_{lane}_retainer_collar"),
            VIAL_WELL_D / 2.0 + 6.0,
            5.0,
            44,
        )
        .translate(x, VIAL_POS.1 - 22.0, BASE_Z + VIAL_BANK_Z + 2.5);
        let collar_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_recovery_vial_{lane}_retainer_opening"),
            VIAL_WELL_D / 2.0,
            6.0,
            44,
        )
        .translate(x, VIAL_POS.1 - 22.0, BASE_Z + VIAL_BANK_Z + 2.5);
        let graduation_bar = centered_cube(
            format!("{OUTPUT_PREFIX}_recovery_vial_{lane}_volume_graduation_bar"),
            4.0,
            42.0,
            6.0,
        )
        .translate(x + 24.0, VIAL_POS.1 - 22.0, BASE_Z + VIAL_BANK_Z + 3.0);

        cuts = cuts + vial + flush_port;
        features = features + (vial_collar - collar_cut) + graduation_bar;
    }

    bank - cuts + features
}

fn carryover_challenge_lanes() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_carryover_challenge_lane_body"),
        CARRYOVER_X,
        CARRYOVER_Y,
        CARRYOVER_Z,
    )
    .translate(CARRYOVER_POS.0, CARRYOVER_POS.1, z_on_base(CARRYOVER_Z));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_carryover_challenge_cuts"));
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_carryover_challenge_rails"));

    for lane in 0..CARRYOVER_LANES {
        let y = CARRYOVER_POS.1 + centered_index(lane, CARRYOVER_LANES, CARRYOVER_LANE_PITCH_Y);
        let separator = centered_cube(
            format!("{OUTPUT_PREFIX}_carryover_lane_{lane}_splash_separator"),
            CARRYOVER_X - 30.0,
            4.0,
            9.0,
        )
        .translate(
            CARRYOVER_POS.0,
            y + CARRYOVER_LANE_PITCH_Y / 2.0 - 4.0,
            BASE_Z + CARRYOVER_Z + 4.5,
        );
        rails = rails + separator;

        for slot in 0..CARRYOVER_SLOTS_PER_LANE {
            let x = CARRYOVER_POS.0 + centered_index(slot, CARRYOVER_SLOTS_PER_LANE, 62.0);
            let pocket = centered_cube(
                format!("{OUTPUT_PREFIX}_carryover_lane_{lane}_standard_slot_{slot}"),
                CARRYOVER_SLOT_X,
                CARRYOVER_SLOT_Y,
                9.0,
            )
            .translate(x, y, BASE_Z + CARRYOVER_Z - 4.0);
            let witness_well = centered_cylinder(
                format!("{OUTPUT_PREFIX}_carryover_lane_{lane}_slot_{slot}_trace_well"),
                5.5 + lane as f64,
                CARRYOVER_Z + 4.0,
                24,
            )
            .translate(x, y, z_on_base(CARRYOVER_Z));
            cuts = cuts + pocket + witness_well;
        }
    }

    body - cuts + rails
}

fn pressure_flow_sensor_coupon_pockets() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_flow_sensor_coupon_panel"),
        SENSOR_PANEL_X,
        SENSOR_PANEL_Y,
        SENSOR_PANEL_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, z_on_base(SENSOR_PANEL_Z));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_pressure_flow_sensor_coupon_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_pressure_flow_sensor_features"));

    for index in 0..SENSOR_COUPONS {
        let x = SENSOR_POS.0 + centered_index(index % 3, 3, 76.0);
        let y = SENSOR_POS.1 + centered_index(index / 3, 2, 58.0);
        let pocket = centered_cube(
            format!("{OUTPUT_PREFIX}_sensor_coupon_{index}_rectangular_pocket"),
            SENSOR_POCKET_X,
            SENSOR_POCKET_Y,
            SENSOR_POCKET_Z,
        )
        .translate(x, y, BASE_Z + SENSOR_PANEL_Z - SENSOR_POCKET_Z / 2.0 + 1.0);
        let pressure = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sensor_coupon_{index}_pressure_tap"),
            PRESSURE_TAP_D / 2.0,
            SENSOR_PANEL_Z + 4.0,
            18,
        )
        .translate(x - 16.0, y, z_on_base(SENSOR_PANEL_Z));
        let flow = centered_cube(
            format!("{OUTPUT_PREFIX}_sensor_coupon_{index}_flow_window"),
            FLOW_WINDOW_X,
            6.0,
            8.0,
        )
        .translate(x + 16.0, y, BASE_Z + SENSOR_PANEL_Z + 4.0);
        let cable_clip = centered_cube(
            format!("{OUTPUT_PREFIX}_sensor_coupon_{index}_cable_strain_relief_clip"),
            34.0,
            5.0,
            8.0,
        )
        .translate(x, y + 23.0, BASE_Z + SENSOR_PANEL_Z + 4.0);

        cuts = cuts + pocket + pressure;
        features = features + flow + cable_clip;
    }

    panel - cuts + features
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_panel"),
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, z_on_base(CUSTODY_PANEL_Z));
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_custody_lands"));

    for index in 0..BARCODE_LANDS {
        let x = CUSTODY_POS.0 + centered_index(index % 4, 4, 76.0);
        let y = CUSTODY_POS.1 + centered_index(index / 4, 2, 36.0);
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_lane_{index}_barcode_land"),
                CUSTODY_LAND_X,
                CUSTODY_LAND_Y,
                4.0,
            )
            .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + 2.0)
            + barcode_bars(index, x, y);
    }

    for index in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_certificate_custody_land_{index}"),
                86.0,
                18.0,
                4.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(index, CERTIFICATE_LANDS, 96.0),
                CUSTODY_POS.1 - 40.0,
                BASE_Z + CUSTODY_PANEL_Z + 2.0,
            );
    }

    panel + lands
}

fn barcode_bars(index: usize, x: f64, y: f64) -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_barcode_{index}_raised_bars"));
    for bar in 0..5 {
        let width = if (bar + index) % 2 == 0 { 2.0 } else { 4.0 };
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_{index}_bar_{bar}"),
                width,
                12.0,
                2.0,
            )
            .translate(
                x - CUSTODY_LAND_X / 2.0 + 10.0 + bar as f64 * 8.0,
                y,
                BASE_Z + CUSTODY_PANEL_Z + 5.0,
            );
    }
    bars
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0 + BRIDGE_POST_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_CLEARANCE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0 - BRIDGE_POST_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_CLEARANCE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_cross_beam"),
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_pods() + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_camera_pods"));
    for index in 0..CAMERA_PODS {
        let x = BRIDGE_POS.0 + centered_index(index, CAMERA_PODS, 245.0);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_camera_pod_{index}_mount"),
            54.0,
            32.0,
            18.0,
        )
        .translate(
            x,
            BRIDGE_POS.1,
            BASE_Z + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 9.0,
        );
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_camera_pod_{index}_lens_axis"),
            8.0,
            20.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, BRIDGE_POS.1 - 22.0, BASE_Z + BRIDGE_CLEARANCE_Z + 7.0);
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for index in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{index}"),
                210.0,
                10.0,
                8.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(index, LIGHT_BARS, 300.0),
                BRIDGE_POS.1 + 28.0,
                BASE_Z + BRIDGE_CLEARANCE_Z + 18.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_robot_service_keepout_gauges"));
    let specs = [
        (
            "front_robot_approach",
            0.0,
            -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE / 2.0,
            STATION_X - 160.0,
            12.0,
        ),
        (
            "rear_service_access",
            0.0,
            STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE / 2.0,
            STATION_X - 190.0,
            12.0,
        ),
        (
            "left_nozzle_service",
            -STATION_X / 2.0 + NOZZLE_SERVICE_CLEARANCE / 2.0,
            0.0,
            12.0,
            STATION_Y - 170.0,
        ),
        (
            "right_sensor_service",
            STATION_X / 2.0 - SENSOR_SERVICE_CLEARANCE / 2.0,
            0.0,
            12.0,
            STATION_Y - 170.0,
        ),
    ];

    for (index, (name, x, y, width, depth)) in specs.iter().enumerate() {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{name}_keepout_gauge_{index}"),
                *width,
                *depth,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + KEEP_OUT_Z / 2.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{name}_keepout_height_flag_{index}"),
                32.0,
                8.0,
                70.0,
            )
            .translate(*x, *y, BASE_Z + 35.0);
    }

    for index in 0..KEEP_OUT_WINDOWS {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_robot_vision_clearance_window_{index}"),
                82.0,
                10.0,
                60.0,
            )
            .translate(
                centered_index(index, KEEP_OUT_WINDOWS, 240.0),
                -STATION_Y / 2.0 + 92.0,
                BASE_Z + 30.0,
            );
    }

    gauges
}

fn closed_fluid_route_witnesses() -> Part {
    let mut routes = Part::empty(format!("{OUTPUT_PREFIX}_closed_fluid_route_witnesses"));
    for lane in 0..LANES {
        let x = RESTRICTION_POS.0 + centered_index(lane, LANES, LANE_PITCH_X);
        let feed = centered_cylinder(
            format!("{OUTPUT_PREFIX}_lane_{lane}_nozzle_to_shear_route_placeholder"),
            3.0,
            180.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 122.0, BASE_Z + 18.0);
        let recovery = centered_cylinder(
            format!("{OUTPUT_PREFIX}_lane_{lane}_shear_to_recovery_route_placeholder"),
            3.5,
            150.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -118.0, BASE_Z + 16.0);
        routes = routes + feed + recovery;
    }
    routes
}

fn mount_positions() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 50.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 50.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (-220.0, -STATION_Y / 2.0 + 50.0),
        (220.0, -STATION_Y / 2.0 + 50.0),
        (-220.0, STATION_Y / 2.0 - 50.0),
        (220.0, STATION_Y / 2.0 - 50.0),
    ]
}

fn z_on_base(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn output_paths_are_unique_and_scoped() {
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert_eq!(
            OUTPUTS
                .iter()
                .filter(|path| path.ends_with("_assembly.stl"))
                .count(),
            1
        );
    }

    #[test]
    fn key_geometry_intent_is_encoded() {
        assert_eq!(LANES, 8);
        assert_eq!(NOZZLE_COUNT, 16);
        assert!(WETOUT_COUPONS > NOZZLE_COUNT);
        assert!(RECOVERY_VIALS >= LANES);
        assert!(SENSOR_COUPONS >= CARRYOVER_LANES);
        assert!(BRIDGE_CLEARANCE_Z > BASE_Z + NOZZLE_PANEL_Z + 80.0);
        assert!(ROBOT_FRONT_CLEARANCE > SERVICE_REAR_CLEARANCE);
    }

    #[test]
    fn layout_fixtures_fit_without_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn required_feature_names_cover_validation_workflow() {
        for feature in [
            "nozzle_nest_array",
            "wetout_witness_coupon_grid",
            "shear_recovery_restriction_lane_bank",
            "droplet_film_retention_pads",
            "flush_recovery_vial_nests",
            "carryover_challenge_lanes",
            "pressure_flow_sensor_coupon_pockets",
            "barcode_custody_lands",
            "camera_evidence_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }
}
