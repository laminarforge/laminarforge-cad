use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator dewpoint/condensation boundary map validation station.
//
// This standalone generator models a no-cell fixture for mapping the boundary
// between dry, dewpoint onset, and visible condensate conditions inside the
// incubator module. It includes a thermal-gradient coupon rack, cold-wall
// surrogate plate, RH/temp probe mast pockets, condensate witness channels,
// droplet collection wells, anti-drip baffle coupons, traceability lands,
// disposition lanes, and raised CSG label geometry. Environmental recipes,
// probe calibration, acceptance limits, and batch release decisions remain
// external validation controls.

const OUTPUT_PREFIX: &str = "closed_incubator_dewpoint_condensation_boundary_map_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_base_boundary_map_deck.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_thermal_gradient_coupon_rack.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_cold_wall_surrogate_plate.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_rh_temp_probe_mast_pockets.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_condensate_witness_channels.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_droplet_collection_wells.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_anti_drip_baffle_coupons.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_barcode_certificate_lands.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_csg_label_geometry.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_dewpoint_condensation_boundary_map_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "thermal_gradient_coupon_rack",
    "cold_wall_surrogate_plate",
    "rh_temp_probe_mast_pockets",
    "condensate_witness_channels",
    "droplet_collection_wells",
    "anti_drip_baffle_coupons",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "csg_label_geometry",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1640.0;
const STATION_Y: f64 = 1080.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 9.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RACK_X: f64 = 620.0;
const RACK_Y: f64 = 330.0;
const RACK_Z: f64 = 50.0;
const RACK_COLS: usize = 5;
const RACK_ROWS: usize = 3;
const THERMAL_COUPON_COUNT: usize = RACK_COLS * RACK_ROWS;
const COUPON_PITCH_X: f64 = 108.0;
const COUPON_PITCH_Y: f64 = 88.0;
const COUPON_SLOT_X: f64 = 78.0;
const COUPON_SLOT_Y: f64 = 46.0;
const GRADIENT_STEP_COUNT: usize = 7;
const RACK_POS: (f64, f64) = (-450.0, 190.0);

const COLD_WALL_X: f64 = 560.0;
const COLD_WALL_Y: f64 = 320.0;
const COLD_WALL_Z: f64 = 54.0;
const COLD_WALL_UPSTAND_Z: f64 = 132.0;
const COLD_STRIPE_COUNT: usize = 8;
const COLD_THERMISTOR_POCKET_COUNT: usize = 6;
const COLD_FILM_CHANNEL_COUNT: usize = 5;
const COLD_WALL_POS: (f64, f64) = (390.0, 215.0);

const PROBE_MAST_X: f64 = 470.0;
const PROBE_MAST_Y: f64 = 205.0;
const PROBE_MAST_Z: f64 = 62.0;
const PROBE_MAST_COUNT: usize = 6;
const PROBE_PITCH_X: f64 = 132.0;
const PROBE_PITCH_Y: f64 = 76.0;
const PROBE_SOCKET_D: f64 = 22.0;
const PROBE_MAST_POS: (f64, f64) = (430.0, -75.0);

const WITNESS_X: f64 = 620.0;
const WITNESS_Y: f64 = 210.0;
const WITNESS_Z: f64 = 36.0;
const WITNESS_CHANNEL_COUNT: usize = 7;
const WITNESS_CHANNEL_PITCH_Y: f64 = 24.0;
const WITNESS_COUPON_COUNT: usize = WITNESS_CHANNEL_COUNT * 2;
const WITNESS_POS: (f64, f64) = (-450.0, -90.0);

const WELL_PLATE_X: f64 = 620.0;
const WELL_PLATE_Y: f64 = 170.0;
const WELL_PLATE_Z: f64 = 42.0;
const WELL_COLS: usize = 4;
const WELL_ROWS: usize = 3;
const DROPLET_WELL_COUNT: usize = WELL_COLS * WELL_ROWS;
const WELL_PITCH_X: f64 = 118.0;
const WELL_PITCH_Y: f64 = 50.0;
const WELL_D: f64 = 38.0;
const WELL_DEPTH: f64 = 28.0;
const WELL_POS: (f64, f64) = (-450.0, -310.0);

const BAFFLE_X: f64 = 470.0;
const BAFFLE_Y: f64 = 190.0;
const BAFFLE_Z: f64 = 38.0;
const BAFFLE_COUPON_COUNT: usize = 6;
const BAFFLE_PITCH_X: f64 = 68.0;
const BAFFLE_BLADE_Z: f64 = 74.0;
const BAFFLE_POS: (f64, f64) = (430.0, -295.0);

const TRACE_X: f64 = 360.0;
const TRACE_Y: f64 = 90.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 6;
const CERTIFICATE_LAND_COUNT: usize = 3;
const TRACE_POS: (f64, f64) = (-160.0, -455.0);

const LANE_X: f64 = 470.0;
const LANE_Y: f64 = 88.0;
const LANE_Z: f64 = 22.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const TOKENS_PER_LANE: usize = 4;
const DISPOSITION_TOKEN_COUNT: usize = DISPOSITION_LANE_COUNT * TOKENS_PER_LANE;
const LANE_POS: (f64, f64) = (430.0, -455.0);

const LABEL_X: f64 = 240.0;
const LABEL_Y: f64 = 90.0;
const LABEL_Z: f64 = 12.0;
const LABEL_PLAQUE_COUNT: usize = 6;
const LABEL_BAR_COUNT: usize = 8;
const LABEL_POS: (f64, f64) = (-620.0, -455.0);

const KEEP_OUT_X: f64 = 1560.0;
const KEEP_OUT_Y: f64 = 1000.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_GAUGE_COUNT: usize = 5;
const ROBOT_FRONT_CLEARANCE: f64 = 31.0;
const SERVICE_REAR_CLEARANCE: f64 = 92.0;
const SIDE_SERVICE_CLEARANCE: f64 = 60.0;
const COLD_WALL_CLEARANCE_Z: f64 = 210.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANE_COUNT] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_boundary_map_deck();
    export(OUTPUTS[0], &base);

    let rack = thermal_gradient_coupon_rack();
    export(OUTPUTS[1], &rack);

    let cold_wall = cold_wall_surrogate_plate();
    export(OUTPUTS[2], &cold_wall);

    let probes = rh_temp_probe_mast_pockets();
    export(OUTPUTS[3], &probes);

    let witnesses = condensate_witness_channels();
    export(OUTPUTS[4], &witnesses);

    let wells = droplet_collection_wells();
    export(OUTPUTS[5], &wells);

    let baffles = anti_drip_baffle_coupons();
    export(OUTPUTS[6], &baffles);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[7], &trace);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let labels = csg_label_geometry();
    export(OUTPUTS[9], &labels);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, insert_z(RACK_Z))
        + cold_wall.translate(COLD_WALL_POS.0, COLD_WALL_POS.1, insert_z(COLD_WALL_Z))
        + probes.translate(PROBE_MAST_POS.0, PROBE_MAST_POS.1, insert_z(PROBE_MAST_Z))
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + wells.translate(WELL_POS.0, WELL_POS.1, insert_z(WELL_PLATE_Z))
        + baffles.translate(BAFFLE_POS.0, BAFFLE_POS.1, insert_z(BAFFLE_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_Z))
        + labels.translate(LABEL_POS.0, LABEL_POS.1, insert_z(LABEL_Z))
        + keepouts.translate(0.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator dewpoint/condensation boundary map station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm sealed boundary-map deck"
    );
    println!(
        "  Thermal gradient:       {THERMAL_COUPON_COUNT} coupon sockets across {GRADIENT_STEP_COUNT} indexed thermal steps"
    );
    println!(
        "  Cold-wall challenge:    {COLD_STRIPE_COUNT} temperature stripes, {COLD_FILM_CHANNEL_COUNT} film channels, {COLD_THERMISTOR_POCKET_COUNT} thermistor pockets"
    );
    println!(
        "  Measurement:            {PROBE_MAST_COUNT} RH/temp probe mast pockets with guarded cable troughs"
    );
    println!(
        "  Condensate capture:     {WITNESS_CHANNEL_COUNT} witness channels, {WITNESS_COUPON_COUNT} witness coupons, {DROPLET_WELL_COUNT} droplet wells, {:.0}mL collection capacity",
        droplet_well_capacity_ml()
    );
    println!(
        "  Controls/traceability:  {BAFFLE_COUPON_COUNT} anti-drip baffle coupons, {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, {DISPOSITION_TOKEN_COUNT} disposition tokens, {LABEL_PLAQUE_COUNT} CSG label plaques"
    );
    println!(
        "  Keepouts:               {KEEP_OUT_GAUGE_COUNT} robot/service gauges with {COLD_WALL_CLEARANCE_Z:.0}mm cold-wall service height"
    );
    println!("  Required features:      {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(THERMAL_COUPON_COUNT, RACK_COLS * RACK_ROWS);
    assert_eq!(DROPLET_WELL_COUNT, WELL_COLS * WELL_ROWS);
    assert_eq!(WITNESS_COUPON_COUNT, WITNESS_CHANNEL_COUNT * 2);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert_eq!(
        DISPOSITION_TOKEN_COUNT,
        DISPOSITION_LANE_COUNT * TOKENS_PER_LANE
    );
    assert_eq!(MOUNT_SLOT_COUNT, mount_slot_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_target_positions().len());
    assert!(COLD_WALL_UPSTAND_Z + COLD_WALL_Z < COLD_WALL_CLEARANCE_Z);
    assert!(PROBE_SOCKET_D < PROBE_MAST_Z);
    assert!(WELL_DEPTH < WELL_PLATE_Z);
    assert!(dewpoint_map_sample_count() >= 48);
    assert!(droplet_well_capacity_ml() > condensate_challenge_volume_ml());
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
    assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);

    for item in socket_rects() {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds boundary-map deck",
            item.name
        );
    }

    let rects = socket_rects();
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn socket_rects() -> [Rect; 9] {
    [
        rect("thermal_gradient_coupon_rack", RACK_POS, RACK_X, RACK_Y),
        rect(
            "cold_wall_surrogate_plate",
            COLD_WALL_POS,
            COLD_WALL_X,
            COLD_WALL_Y,
        ),
        rect(
            "rh_temp_probe_mast_pockets",
            PROBE_MAST_POS,
            PROBE_MAST_X,
            PROBE_MAST_Y,
        ),
        rect(
            "condensate_witness_channels",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "droplet_collection_wells",
            WELL_POS,
            WELL_PLATE_X,
            WELL_PLATE_Y,
        ),
        rect("anti_drip_baffle_coupons", BAFFLE_POS, BAFFLE_X, BAFFLE_Y),
        rect("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        rect("release_hold_reject_lanes", LANE_POS, LANE_X, LANE_Y),
        rect("csg_label_geometry", LABEL_POS, LABEL_X, LABEL_Y),
    ]
}

fn dewpoint_map_sample_count() -> usize {
    THERMAL_COUPON_COUNT + COLD_STRIPE_COUNT * PROBE_MAST_COUNT + DROPLET_WELL_COUNT
}

fn droplet_well_capacity_ml() -> f64 {
    let radius = WELL_D / 2.0;
    DROPLET_WELL_COUNT as f64 * std::f64::consts::PI * radius * radius * WELL_DEPTH / 1000.0
}

fn condensate_challenge_volume_ml() -> f64 {
    WITNESS_CHANNEL_COUNT as f64 * 14.0 + COLD_FILM_CHANNEL_COUNT as f64 * 18.0
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (COLD_WALL_POS.1 + COLD_WALL_Y / 2.0) + COLD_WALL_CLEARANCE_Z
}

fn side_service_clearance() -> f64 {
    STATION_X / 2.0 - (RACK_POS.0.abs() + RACK_X / 2.0)
}

fn base_boundary_map_deck() -> Part {
    let deck = centered_cube(
        "dewpoint_boundary_map_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "dewpoint_boundary_map_secondary_condensate_basin_cut",
        STATION_X - 2.0 * (RIM_W + 50.0),
        STATION_Y - 2.0 * (RIM_W + 54.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -10.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(
        "dewpoint_boundary_map_front_condensate_drain_bore",
        8.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 96.0,
        -STATION_Y / 2.0 + 34.0,
        DECK_Z - 8.0,
    );

    deck - basin - front_drain - insert_sockets() - deck_mounting_slots()
        + perimeter_rims()
        + workflow_spines()
        + robot_datum_targets()
        + boundary_map_axis_ticks()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("dewpoint_boundary_map_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("dewpoint_boundary_map_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mounting_slots() -> Part {
    let mut slots = Part::empty("dewpoint_boundary_map_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("dewpoint_boundary_map_m6_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("dewpoint_boundary_map_m6_slotted_relief_{i}"),
                30.0,
                7.4,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 60.0),
        (0.0, -STATION_Y / 2.0 + 60.0),
        (0.0, STATION_Y / 2.0 - 60.0),
        (-STATION_X / 2.0 + 60.0, 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "dewpoint_boundary_map_front_low_robot_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 14.0);
    let rear = centered_cube(
        "dewpoint_boundary_map_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "dewpoint_boundary_map_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "dewpoint_boundary_map_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_spines() -> Part {
    let upper = centered_cube(
        "dewpoint_boundary_map_gradient_to_cold_wall_zone_spine",
        STATION_X - 240.0,
        10.0,
        24.0,
    )
    .translate(0.0, 22.0, DECK_Z + 12.0);
    let lower = centered_cube(
        "dewpoint_boundary_map_witness_to_baffle_zone_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -205.0, DECK_Z + 11.0);
    let disposition = centered_cube(
        "dewpoint_boundary_map_traceability_disposition_zone_spine",
        STATION_X - 300.0,
        8.0,
        20.0,
    )
    .translate(0.0, -400.0, DECK_Z + 10.0);
    let center = centered_cube(
        "dewpoint_boundary_map_left_right_challenge_divider",
        12.0,
        STATION_Y - 210.0,
        26.0,
    )
    .translate(0.0, -38.0, DECK_Z + 13.0);

    upper + lower + disposition + center
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("dewpoint_boundary_map_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets =
            targets
                + fiducial_disc(&format!("dewpoint_boundary_map_robot_datum_target_{i}"))
                    .translate(x, y, DECK_Z + 2.5);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 104.0),
    ]
}

fn boundary_map_axis_ticks() -> Part {
    let mut ticks = Part::empty("dewpoint_boundary_map_axis_ticks");
    for i in 0..COLD_STRIPE_COUNT {
        ticks = ticks
            + centered_cube(
                format!("dewpoint_boundary_map_cold_wall_axis_tick_{i}"),
                4.0,
                STATION_Y - 230.0,
                4.0,
            )
            .translate(
                centered_index(i, COLD_STRIPE_COUNT, 66.0) + 205.0,
                -25.0,
                DECK_Z + 2.0,
            );
    }
    for i in 0..WITNESS_CHANNEL_COUNT {
        ticks = ticks
            + centered_cube(
                format!("dewpoint_boundary_map_humidity_axis_tick_{i}"),
                STATION_X - 280.0,
                3.0,
                4.0,
            )
            .translate(
                0.0,
                centered_index(i, WITNESS_CHANNEL_COUNT, 56.0) - 110.0,
                DECK_Z + 2.0,
            );
    }
    ticks
}

fn thermal_gradient_coupon_rack() -> Part {
    let tray = centered_cube(
        "dewpoint_boundary_map_thermal_gradient_coupon_rack_body",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let top_relief = centered_cube(
        "dewpoint_boundary_map_thermal_gradient_rack_top_relief",
        RACK_X - 52.0,
        RACK_Y - 44.0,
        10.0,
    )
    .translate(0.0, 0.0, RACK_Z / 2.0 - 5.0);

    tray - top_relief - thermal_coupon_slot_cuts() - thermal_air_bypass_cuts()
        + thermal_coupon_inserts()
        + gradient_step_blocks()
        + thermal_index_rails()
        + rack_gripper_handles()
}

fn thermal_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_thermal_coupon_slot_cuts");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = row * RACK_COLS + col;
            let (x, y) = thermal_coupon_xy(row, col);
            cuts = cuts
                + centered_cube(
                    format!("dewpoint_boundary_map_coupon_{index}_thermal_socket_cut"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    18.0,
                )
                .translate(x, y, RACK_Z / 2.0 - 9.0)
                + centered_cube(
                    format!("dewpoint_boundary_map_coupon_{index}_finger_lift_cut"),
                    18.0,
                    COUPON_SLOT_Y + 18.0,
                    12.0,
                )
                .translate(x + COUPON_SLOT_X / 2.0 - 8.0, y, RACK_Z / 2.0 - 5.0);
        }
    }
    cuts
}

fn thermal_air_bypass_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_thermal_air_bypass_cuts");
    for i in 0..(RACK_ROWS + 1) {
        cuts = cuts
            + centered_cube(
                format!("dewpoint_boundary_map_thermal_bypass_slot_{i}"),
                RACK_X - 96.0,
                10.0,
                RACK_Z + 2.0,
            )
            .translate(0.0, centered_index(i, RACK_ROWS + 1, 78.0), 0.0);
    }
    cuts
}

fn thermal_coupon_inserts() -> Part {
    let mut coupons = Part::empty("dewpoint_boundary_map_thermal_coupon_inserts");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = row * RACK_COLS + col;
            let (x, y) = thermal_coupon_xy(row, col);
            let coupon = centered_cube(
                format!("dewpoint_boundary_map_gradient_coupon_{index}_insert"),
                COUPON_SLOT_X - 18.0,
                COUPON_SLOT_Y - 12.0,
                5.0,
            )
            .translate(x, y, RACK_Z / 2.0 + 2.5);
            let witness_dot = centered_cylinder(
                format!("dewpoint_boundary_map_gradient_coupon_{index}_dewpoint_witness_dot"),
                7.0,
                5.6,
                28,
            )
            .translate(x - COUPON_SLOT_X / 2.0 + 24.0, y, RACK_Z / 2.0 + 2.8);
            coupons = coupons + coupon + witness_dot;
        }
    }
    coupons
}

fn thermal_coupon_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, RACK_COLS, COUPON_PITCH_X),
        centered_index(row, RACK_ROWS, COUPON_PITCH_Y),
    )
}

fn gradient_step_blocks() -> Part {
    let mut steps = Part::empty("dewpoint_boundary_map_gradient_step_blocks");
    for i in 0..GRADIENT_STEP_COUNT {
        let height = 6.0 + i as f64 * 2.0;
        steps = steps
            + centered_cube(
                format!("dewpoint_boundary_map_thermal_gradient_step_{i}"),
                44.0,
                28.0,
                height,
            )
            .translate(
                centered_index(i, GRADIENT_STEP_COUNT, 62.0),
                RACK_Y / 2.0 - 34.0,
                RACK_Z / 2.0 + height / 2.0,
            );
    }
    steps
}

fn thermal_index_rails() -> Part {
    let cold_rail = centered_cube(
        "dewpoint_boundary_map_cold_side_index_rail",
        18.0,
        RACK_Y - 52.0,
        22.0,
    )
    .translate(-RACK_X / 2.0 + 36.0, 0.0, RACK_Z / 2.0 + 11.0);
    let warm_rail = centered_cube(
        "dewpoint_boundary_map_warm_side_index_rail",
        18.0,
        RACK_Y - 52.0,
        22.0,
    )
    .translate(RACK_X / 2.0 - 36.0, 0.0, RACK_Z / 2.0 + 11.0);
    let dewpoint_ridge = centered_cube(
        "dewpoint_boundary_map_nominal_dewpoint_boundary_ridge",
        RACK_X - 96.0,
        8.0,
        18.0,
    )
    .rotate(0.0, 0.0, -4.0)
    .translate(0.0, 4.0, RACK_Z / 2.0 + 9.0);

    cold_rail + warm_rail + dewpoint_ridge
}

fn rack_gripper_handles() -> Part {
    let left = centered_cube(
        "dewpoint_boundary_map_rack_left_robot_gripper_handle",
        56.0,
        18.0,
        22.0,
    )
    .translate(
        -RACK_X / 2.0 + 52.0,
        -RACK_Y / 2.0 + 26.0,
        RACK_Z / 2.0 + 11.0,
    );
    let right = centered_cube(
        "dewpoint_boundary_map_rack_right_robot_gripper_handle",
        56.0,
        18.0,
        22.0,
    )
    .translate(
        RACK_X / 2.0 - 52.0,
        -RACK_Y / 2.0 + 26.0,
        RACK_Z / 2.0 + 11.0,
    );
    left + right
}

fn cold_wall_surrogate_plate() -> Part {
    let base = centered_cube(
        "dewpoint_boundary_map_cold_wall_surrogate_base_plate",
        COLD_WALL_X,
        COLD_WALL_Y,
        COLD_WALL_Z,
    );
    let wet_floor_relief = centered_cube(
        "dewpoint_boundary_map_cold_wall_wet_floor_relief",
        COLD_WALL_X - 70.0,
        COLD_WALL_Y - 74.0,
        8.0,
    )
    .translate(0.0, -22.0, COLD_WALL_Z / 2.0 - 4.0);
    let upstand = centered_cube(
        "dewpoint_boundary_map_vertical_cold_wall_surrogate_plate",
        COLD_WALL_X - 84.0,
        20.0,
        COLD_WALL_UPSTAND_Z,
    )
    .translate(
        0.0,
        COLD_WALL_Y / 2.0 - 34.0,
        COLD_WALL_Z / 2.0 + COLD_WALL_UPSTAND_Z / 2.0,
    );

    base - wet_floor_relief - cold_wall_film_channel_cuts() - cold_wall_sensor_cuts()
        + upstand
        + cold_wall_temperature_stripes()
        + cold_wall_edge_gutters()
        + chilled_manifold_ports()
}

fn cold_wall_film_channel_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_cold_wall_film_channel_cuts");
    for i in 0..COLD_FILM_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("dewpoint_boundary_map_cold_wall_film_channel_cut_{i}"),
                COLD_WALL_X - 130.0,
                10.0,
                12.0,
            )
            .rotate(0.0, 0.0, -6.0)
            .translate(
                0.0,
                centered_index(i, COLD_FILM_CHANNEL_COUNT, 42.0) - 34.0,
                COLD_WALL_Z / 2.0 - 6.0,
            );
    }
    cuts
}

fn cold_wall_sensor_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_cold_wall_sensor_cuts");
    for i in 0..COLD_THERMISTOR_POCKET_COUNT {
        let x = centered_index(i, COLD_THERMISTOR_POCKET_COUNT, 70.0);
        cuts = cuts
            + centered_cylinder(
                format!("dewpoint_boundary_map_cold_wall_thermistor_pocket_{i}"),
                6.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                COLD_WALL_Y / 2.0 - 34.0,
                COLD_WALL_Z / 2.0 + COLD_WALL_UPSTAND_Z * 0.45,
            );
    }
    cuts
}

fn cold_wall_temperature_stripes() -> Part {
    let mut stripes = Part::empty("dewpoint_boundary_map_cold_wall_temperature_stripes");
    for i in 0..COLD_STRIPE_COUNT {
        let height = 5.0 + (i % 4) as f64 * 1.4;
        stripes = stripes
            + centered_cube(
                format!("dewpoint_boundary_map_cold_wall_isotherm_stripe_{i}"),
                16.0,
                COLD_WALL_Y - 90.0,
                height,
            )
            .translate(
                centered_index(i, COLD_STRIPE_COUNT, 52.0),
                -18.0,
                COLD_WALL_Z / 2.0 + height / 2.0,
            );
    }
    stripes
}

fn cold_wall_edge_gutters() -> Part {
    let lower = centered_cube(
        "dewpoint_boundary_map_cold_wall_lower_condensate_gutter",
        COLD_WALL_X - 88.0,
        16.0,
        20.0,
    )
    .translate(0.0, -COLD_WALL_Y / 2.0 + 44.0, COLD_WALL_Z / 2.0 + 10.0);
    let left = centered_cube(
        "dewpoint_boundary_map_cold_wall_left_edge_gutter",
        16.0,
        COLD_WALL_Y - 94.0,
        18.0,
    )
    .translate(-COLD_WALL_X / 2.0 + 44.0, -8.0, COLD_WALL_Z / 2.0 + 9.0);
    let right = centered_cube(
        "dewpoint_boundary_map_cold_wall_right_edge_gutter",
        16.0,
        COLD_WALL_Y - 94.0,
        18.0,
    )
    .translate(COLD_WALL_X / 2.0 - 44.0, -8.0, COLD_WALL_Z / 2.0 + 9.0);
    lower + left + right
}

fn chilled_manifold_ports() -> Part {
    let mut ports = Part::empty("dewpoint_boundary_map_chilled_manifold_ports");
    for i in 0..4 {
        let x = centered_index(i, 4, 78.0);
        let block = centered_cube(
            format!("dewpoint_boundary_map_chilled_manifold_port_block_{i}"),
            48.0,
            20.0,
            34.0,
        )
        .translate(x, COLD_WALL_Y / 2.0 + 8.0, COLD_WALL_Z / 2.0 + 17.0);
        let bore = centered_cylinder(
            format!("dewpoint_boundary_map_chilled_manifold_port_bore_{i}"),
            6.0,
            24.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, COLD_WALL_Y / 2.0 + 8.0, COLD_WALL_Z / 2.0 + 17.0);
        ports = ports + (block - bore);
    }
    ports
}

fn rh_temp_probe_mast_pockets() -> Part {
    let body = centered_cube(
        "dewpoint_boundary_map_probe_mast_pocket_plate",
        PROBE_MAST_X,
        PROBE_MAST_Y,
        PROBE_MAST_Z,
    );
    let cable_trough = centered_cube(
        "dewpoint_boundary_map_probe_mast_rear_cable_trough_cut",
        PROBE_MAST_X - 54.0,
        18.0,
        16.0,
    )
    .translate(0.0, PROBE_MAST_Y / 2.0 - 24.0, PROBE_MAST_Z / 2.0 - 8.0);

    body - probe_mast_socket_cuts() - cable_trough
        + probe_mast_collar_rings()
        + probe_mast_coordinate_flags()
        + probe_wire_combs()
}

fn probe_mast_socket_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_probe_mast_socket_cuts");
    for i in 0..PROBE_MAST_COUNT {
        let (x, y) = probe_mast_xy(i);
        cuts = cuts
            + centered_cylinder(
                format!("dewpoint_boundary_map_probe_mast_socket_cut_{i}"),
                PROBE_SOCKET_D / 2.0,
                PROBE_MAST_Z + 4.0,
                36,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("dewpoint_boundary_map_probe_mast_keyway_cut_{i}"),
                9.0,
                24.0,
                PROBE_MAST_Z + 5.0,
            )
            .translate(x + PROBE_SOCKET_D / 2.0, y, 0.0);
    }
    cuts
}

fn probe_mast_xy(index: usize) -> (f64, f64) {
    let row = index / 3;
    let col = index % 3;
    (
        centered_index(col, 3, PROBE_PITCH_X),
        centered_index(row, 2, PROBE_PITCH_Y) - 10.0,
    )
}

fn probe_mast_collar_rings() -> Part {
    let mut rings = Part::empty("dewpoint_boundary_map_probe_mast_collar_rings");
    for i in 0..PROBE_MAST_COUNT {
        let (x, y) = probe_mast_xy(i);
        let outer = centered_cylinder(
            format!("dewpoint_boundary_map_probe_mast_collar_outer_{i}"),
            PROBE_SOCKET_D / 2.0 + 8.0,
            7.0,
            40,
        );
        let inner = centered_cylinder(
            format!("dewpoint_boundary_map_probe_mast_collar_inner_cut_{i}"),
            PROBE_SOCKET_D / 2.0 + 1.6,
            8.0,
            40,
        );
        rings = rings + (outer - inner).translate(x, y, PROBE_MAST_Z / 2.0 + 3.5);
    }
    rings
}

fn probe_mast_coordinate_flags() -> Part {
    let mut flags = Part::empty("dewpoint_boundary_map_probe_mast_coordinate_flags");
    for i in 0..PROBE_MAST_COUNT {
        let (x, y) = probe_mast_xy(i);
        flags = flags
            + centered_cube(
                format!("dewpoint_boundary_map_probe_mast_coordinate_flag_{i}"),
                36.0,
                12.0,
                4.0,
            )
            .translate(x, y - 32.0, PROBE_MAST_Z / 2.0 + 2.0);
    }
    flags
}

fn probe_wire_combs() -> Part {
    let rear = centered_cube(
        "dewpoint_boundary_map_probe_wire_comb_rear_rail",
        PROBE_MAST_X - 72.0,
        10.0,
        22.0,
    )
    .translate(0.0, PROBE_MAST_Y / 2.0 - 16.0, PROBE_MAST_Z / 2.0 + 11.0);
    let mut teeth = Part::empty("dewpoint_boundary_map_probe_wire_comb_teeth");
    for i in 0..(PROBE_MAST_COUNT + 1) {
        teeth = teeth
            + centered_cube(
                format!("dewpoint_boundary_map_probe_wire_comb_tooth_{i}"),
                8.0,
                22.0,
                24.0,
            )
            .translate(
                centered_index(i, PROBE_MAST_COUNT + 1, 58.0),
                PROBE_MAST_Y / 2.0 - 16.0,
                PROBE_MAST_Z / 2.0 + 12.0,
            );
    }
    rear + teeth
}

fn condensate_witness_channels() -> Part {
    let body = centered_cube(
        "dewpoint_boundary_map_condensate_witness_channel_plate",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let sump_relief = centered_cube(
        "dewpoint_boundary_map_condensate_witness_sump_relief",
        WITNESS_X - 74.0,
        WITNESS_Y - 56.0,
        8.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0 - 4.0);

    body - sump_relief - witness_channel_cuts()
        + witness_channel_lips()
        + witness_coupon_tabs()
        + witness_flow_arrows()
}

fn witness_channel_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_witness_channel_cuts");
    for i in 0..WITNESS_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("dewpoint_boundary_map_condensate_witness_channel_cut_{i}"),
                WITNESS_X - 100.0,
                11.0 + (i % 3) as f64 * 2.0,
                14.0,
            )
            .translate(
                0.0,
                centered_index(i, WITNESS_CHANNEL_COUNT, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 - 7.0,
            );
    }
    cuts
}

fn witness_channel_lips() -> Part {
    let mut lips = Part::empty("dewpoint_boundary_map_witness_channel_lips");
    for i in 0..WITNESS_CHANNEL_COUNT {
        let y = centered_index(i, WITNESS_CHANNEL_COUNT, WITNESS_CHANNEL_PITCH_Y);
        lips = lips
            + centered_cube(
                format!("dewpoint_boundary_map_witness_channel_{i}_left_lip"),
                WITNESS_X - 112.0,
                4.0,
                8.0,
            )
            .translate(0.0, y - 10.0, WITNESS_Z / 2.0 + 4.0)
            + centered_cube(
                format!("dewpoint_boundary_map_witness_channel_{i}_right_lip"),
                WITNESS_X - 112.0,
                4.0,
                8.0,
            )
            .translate(0.0, y + 10.0, WITNESS_Z / 2.0 + 4.0);
    }
    lips
}

fn witness_coupon_tabs() -> Part {
    let mut tabs = Part::empty("dewpoint_boundary_map_condensate_witness_coupon_tabs");
    for i in 0..WITNESS_COUPON_COUNT {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let channel = i / 2;
        tabs = tabs
            + centered_cube(
                format!("dewpoint_boundary_map_channel_{channel}_witness_coupon_tab_{i}"),
                44.0,
                18.0,
                5.0,
            )
            .translate(
                side * (WITNESS_X / 2.0 - 58.0),
                centered_index(channel, WITNESS_CHANNEL_COUNT, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn witness_flow_arrows() -> Part {
    let mut arrows = Part::empty("dewpoint_boundary_map_witness_flow_arrows");
    for i in 0..WITNESS_CHANNEL_COUNT {
        let y = centered_index(i, WITNESS_CHANNEL_COUNT, WITNESS_CHANNEL_PITCH_Y);
        let shaft = centered_cube(
            format!("dewpoint_boundary_map_witness_flow_arrow_shaft_{i}"),
            62.0,
            4.0,
            4.0,
        )
        .translate(-74.0, y, WITNESS_Z / 2.0 + 2.0);
        let head = centered_cube(
            format!("dewpoint_boundary_map_witness_flow_arrow_head_{i}"),
            14.0,
            14.0,
            4.0,
        )
        .rotate(0.0, 0.0, 45.0)
        .translate(-36.0, y, WITNESS_Z / 2.0 + 2.0);
        arrows = arrows + shaft + head;
    }
    arrows
}

fn droplet_collection_wells() -> Part {
    let body = centered_cube(
        "dewpoint_boundary_map_droplet_collection_well_plate",
        WELL_PLATE_X,
        WELL_PLATE_Y,
        WELL_PLATE_Z,
    );
    let spill_gutter = centered_cube(
        "dewpoint_boundary_map_droplet_collection_spill_gutter_cut",
        WELL_PLATE_X - 84.0,
        14.0,
        14.0,
    )
    .translate(0.0, WELL_PLATE_Y / 2.0 - 28.0, WELL_PLATE_Z / 2.0 - 7.0);

    body - droplet_well_cuts() - spill_gutter
        + droplet_well_rims()
        + overflow_bridge_channels()
        + volume_step_pins()
}

fn droplet_well_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_droplet_well_cuts");
    for row in 0..WELL_ROWS {
        for col in 0..WELL_COLS {
            let index = row * WELL_COLS + col;
            let (x, y) = droplet_well_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("dewpoint_boundary_map_droplet_collection_well_cut_{index}"),
                    WELL_D / 2.0,
                    WELL_DEPTH + 1.0,
                    48,
                )
                .translate(x, y, WELL_PLATE_Z / 2.0 - WELL_DEPTH / 2.0 + 0.5);
        }
    }
    cuts
}

fn droplet_well_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, WELL_COLS, WELL_PITCH_X),
        centered_index(row, WELL_ROWS, WELL_PITCH_Y) - 6.0,
    )
}

fn droplet_well_rims() -> Part {
    let mut rims = Part::empty("dewpoint_boundary_map_droplet_well_rims");
    for row in 0..WELL_ROWS {
        for col in 0..WELL_COLS {
            let index = row * WELL_COLS + col;
            let (x, y) = droplet_well_xy(row, col);
            let outer = centered_cylinder(
                format!("dewpoint_boundary_map_droplet_well_rim_outer_{index}"),
                WELL_D / 2.0 + 5.5,
                5.0,
                48,
            );
            let inner = centered_cylinder(
                format!("dewpoint_boundary_map_droplet_well_rim_inner_{index}"),
                WELL_D / 2.0 + 1.0,
                6.0,
                48,
            );
            rims = rims + (outer - inner).translate(x, y, WELL_PLATE_Z / 2.0 + 2.5);
        }
    }
    rims
}

fn overflow_bridge_channels() -> Part {
    let mut bridges = Part::empty("dewpoint_boundary_map_droplet_overflow_bridge_channels");
    for row in 0..WELL_ROWS {
        for col in 0..(WELL_COLS - 1) {
            let index = row * (WELL_COLS - 1) + col;
            let (x0, y) = droplet_well_xy(row, col);
            bridges = bridges
                + centered_cube(
                    format!("dewpoint_boundary_map_droplet_overflow_bridge_{index}"),
                    WELL_PITCH_X - WELL_D,
                    7.0,
                    5.0,
                )
                .translate(x0 + WELL_PITCH_X / 2.0, y, WELL_PLATE_Z / 2.0 + 2.5);
        }
    }
    bridges
}

fn volume_step_pins() -> Part {
    let mut pins = Part::empty("dewpoint_boundary_map_droplet_volume_step_pins");
    for i in 0..4 {
        let height = 8.0 + i as f64 * 3.0;
        pins = pins
            + centered_cylinder(
                format!("dewpoint_boundary_map_droplet_volume_step_pin_{i}"),
                5.0,
                height,
                24,
            )
            .translate(
                WELL_PLATE_X / 2.0 - 42.0,
                centered_index(i, 4, 28.0),
                WELL_PLATE_Z / 2.0 + height / 2.0,
            );
    }
    pins
}

fn anti_drip_baffle_coupons() -> Part {
    let base = centered_cube(
        "dewpoint_boundary_map_anti_drip_baffle_coupon_base",
        BAFFLE_X,
        BAFFLE_Y,
        BAFFLE_Z,
    );
    let tray_relief = centered_cube(
        "dewpoint_boundary_map_anti_drip_baffle_tray_relief",
        BAFFLE_X - 52.0,
        BAFFLE_Y - 50.0,
        8.0,
    )
    .translate(0.0, 0.0, BAFFLE_Z / 2.0 - 4.0);

    base - tray_relief - baffle_coupon_slot_cuts()
        + baffle_coupon_blades()
        + baffle_drip_lips()
        + baffle_angle_gauges()
}

fn baffle_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_baffle_coupon_slot_cuts");
    for i in 0..BAFFLE_COUPON_COUNT {
        cuts = cuts
            + centered_cube(
                format!("dewpoint_boundary_map_baffle_coupon_slot_cut_{i}"),
                48.0,
                120.0,
                14.0,
            )
            .rotate(0.0, 0.0, baffle_angle(i))
            .translate(
                centered_index(i, BAFFLE_COUPON_COUNT, BAFFLE_PITCH_X),
                0.0,
                BAFFLE_Z / 2.0 - 7.0,
            );
    }
    cuts
}

fn baffle_coupon_blades() -> Part {
    let mut blades = Part::empty("dewpoint_boundary_map_baffle_coupon_blades");
    for i in 0..BAFFLE_COUPON_COUNT {
        blades = blades
            + centered_cube(
                format!("dewpoint_boundary_map_anti_drip_baffle_coupon_blade_{i}"),
                44.0,
                8.0,
                BAFFLE_BLADE_Z,
            )
            .rotate(0.0, 0.0, baffle_angle(i))
            .translate(
                centered_index(i, BAFFLE_COUPON_COUNT, BAFFLE_PITCH_X),
                0.0,
                BAFFLE_Z / 2.0 + BAFFLE_BLADE_Z / 2.0,
            );
    }
    blades
}

fn baffle_angle(index: usize) -> f64 {
    [-18.0, -10.0, -4.0, 4.0, 10.0, 18.0][index]
}

fn baffle_drip_lips() -> Part {
    let mut lips = Part::empty("dewpoint_boundary_map_baffle_drip_lips");
    for i in 0..BAFFLE_COUPON_COUNT {
        lips = lips
            + centered_cube(
                format!("dewpoint_boundary_map_baffle_coupon_drip_lip_{i}"),
                54.0,
                10.0,
                8.0,
            )
            .rotate(0.0, 0.0, baffle_angle(i))
            .translate(
                centered_index(i, BAFFLE_COUPON_COUNT, BAFFLE_PITCH_X),
                -58.0,
                BAFFLE_Z / 2.0 + 4.0,
            );
    }
    lips
}

fn baffle_angle_gauges() -> Part {
    let mut gauges = Part::empty("dewpoint_boundary_map_baffle_angle_gauges");
    for i in 0..BAFFLE_COUPON_COUNT {
        gauges = gauges
            + centered_cube(
                format!("dewpoint_boundary_map_baffle_angle_gauge_tick_{i}"),
                6.0,
                30.0,
                4.0,
            )
            .rotate(0.0, 0.0, baffle_angle(i))
            .translate(
                centered_index(i, BAFFLE_COUPON_COUNT, BAFFLE_PITCH_X),
                BAFFLE_Y / 2.0 - 28.0,
                BAFFLE_Z / 2.0 + 2.0,
            );
    }
    gauges
}

fn barcode_certificate_lands() -> Part {
    let base = centered_cube(
        "dewpoint_boundary_map_barcode_certificate_land_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let bevel_relief = centered_cube(
        "dewpoint_boundary_map_barcode_certificate_bevel_relief",
        TRACE_X - 30.0,
        TRACE_Y - 24.0,
        5.0,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0 - 2.0);

    base - bevel_relief + barcode_lands() + certificate_lands() + qr_grid_surrogates()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("dewpoint_boundary_map_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + csg_barcode_plaque(
                format!("dewpoint_boundary_map_barcode_land_{i}"),
                46.0,
                22.0,
                4.0,
                i,
            )
            .translate(
                centered_index(i, BARCODE_LAND_COUNT, 52.0),
                TRACE_Y / 2.0 - 26.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("dewpoint_boundary_map_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("dewpoint_boundary_map_certificate_land_{i}"),
                86.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 102.0),
                -TRACE_Y / 2.0 + 24.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn qr_grid_surrogates() -> Part {
    let mut dots = Part::empty("dewpoint_boundary_map_qr_grid_surrogate_dots");
    for i in 0..12 {
        dots = dots
            + centered_cube(
                format!("dewpoint_boundary_map_qr_grid_dot_{i}"),
                5.0 + (i % 3) as f64,
                5.0 + (i % 2) as f64,
                4.5,
            )
            .translate(
                -TRACE_X / 2.0 + 32.0 + (i % 4) as f64 * 10.0,
                -TRACE_Y / 2.0 + 18.0 + (i / 4) as f64 * 10.0,
                TRACE_Z / 2.0 + 2.25,
            );
    }
    dots
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "dewpoint_boundary_map_release_hold_reject_lane_plate",
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    let lane_tray = centered_cube(
        "dewpoint_boundary_map_release_hold_reject_lane_tray_cut",
        LANE_X - 38.0,
        LANE_Y - 26.0,
        8.0,
    )
    .translate(0.0, 0.0, LANE_Z / 2.0 - 4.0);

    plate - lane_tray - disposition_token_pocket_cuts()
        + disposition_lane_dividers()
        + disposition_lane_label_lands()
        + disposition_status_tokens()
}

fn disposition_token_pocket_cuts() -> Part {
    let mut cuts = Part::empty("dewpoint_boundary_map_disposition_token_pocket_cuts");
    for lane in DispositionLane::all() {
        for token in 0..TOKENS_PER_LANE {
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "dewpoint_boundary_map_{}_lane_token_pocket_cut_{token}",
                        lane.name()
                    ),
                    11.0,
                    10.0,
                    28,
                )
                .translate(
                    lane_token_x(lane, token),
                    lane_y(lane),
                    LANE_Z / 2.0 - 5.0,
                );
        }
    }
    cuts
}

fn lane_y(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), DISPOSITION_LANE_COUNT, 26.0)
}

fn lane_token_x(lane: DispositionLane, token: usize) -> f64 {
    centered_index(token, TOKENS_PER_LANE, 50.0) + (lane.index() as f64 - 1.0) * 8.0
}

fn disposition_lane_dividers() -> Part {
    let upper = centered_cube(
        "dewpoint_boundary_map_release_hold_lane_divider",
        LANE_X - 54.0,
        4.0,
        12.0,
    )
    .translate(0.0, 13.0, LANE_Z / 2.0 + 6.0);
    let lower = centered_cube(
        "dewpoint_boundary_map_hold_reject_lane_divider",
        LANE_X - 54.0,
        4.0,
        12.0,
    )
    .translate(0.0, -13.0, LANE_Z / 2.0 + 6.0);
    upper + lower
}

fn disposition_lane_label_lands() -> Part {
    let mut labels = Part::empty("dewpoint_boundary_map_disposition_lane_labels");
    for lane in DispositionLane::all() {
        labels = labels
            + centered_cube(
                format!(
                    "dewpoint_boundary_map_{}_lane_raised_label_land",
                    lane.name()
                ),
                88.0,
                16.0,
                4.0,
            )
            .translate(-LANE_X / 2.0 + 70.0, lane_y(lane), LANE_Z / 2.0 + 2.0);
    }
    labels
}

fn disposition_status_tokens() -> Part {
    let mut tokens = Part::empty("dewpoint_boundary_map_disposition_status_tokens");
    for lane in DispositionLane::all() {
        let token = centered_cylinder(
            format!("dewpoint_boundary_map_{}_lane_status_token", lane.name()),
            13.0,
            5.0,
            32,
        )
        .translate(LANE_X / 2.0 - 48.0, lane_y(lane), LANE_Z / 2.0 + 2.5);
        let notch = centered_cube(
            format!(
                "dewpoint_boundary_map_{}_lane_status_token_notch",
                lane.name()
            ),
            6.0,
            18.0,
            6.0,
        )
        .rotate(0.0, 0.0, lane.index() as f64 * 45.0)
        .translate(LANE_X / 2.0 - 48.0, lane_y(lane), LANE_Z / 2.0 + 2.5);
        tokens = tokens + (token - notch);
    }
    tokens
}

fn csg_label_geometry() -> Part {
    let panel = centered_cube(
        "dewpoint_boundary_map_csg_label_panel",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    );
    let recessed_field = centered_cube(
        "dewpoint_boundary_map_csg_label_panel_recess",
        LABEL_X - 24.0,
        LABEL_Y - 22.0,
        4.0,
    )
    .translate(0.0, 0.0, LABEL_Z / 2.0 - 2.0);

    panel - recessed_field + label_plaques() + label_droplet_icons() + label_boundary_arrows()
}

fn label_plaques() -> Part {
    let mut plaques = Part::empty("dewpoint_boundary_map_csg_label_plaques");
    for i in 0..LABEL_PLAQUE_COUNT {
        plaques = plaques
            + csg_barcode_plaque(
                format!("dewpoint_boundary_map_csg_label_plaque_{i}"),
                64.0,
                20.0,
                4.0,
                i + 11,
            )
            .translate(
                centered_index(i % 3, 3, 74.0),
                centered_index(i / 3, 2, 34.0) + 8.0,
                LABEL_Z / 2.0 + 2.0,
            );
    }
    plaques
}

fn label_droplet_icons() -> Part {
    let mut icons = Part::empty("dewpoint_boundary_map_csg_label_droplet_icons");
    for i in 0..3 {
        let drop = centered_cylinder(
            format!("dewpoint_boundary_map_csg_label_droplet_icon_round_{i}"),
            7.0,
            4.5,
            32,
        )
        .translate(
            -LABEL_X / 2.0 + 26.0 + i as f64 * 22.0,
            -LABEL_Y / 2.0 + 20.0,
            LABEL_Z / 2.0 + 2.25,
        );
        let tail = centered_cube(
            format!("dewpoint_boundary_map_csg_label_droplet_icon_tail_{i}"),
            9.0,
            9.0,
            4.5,
        )
        .rotate(0.0, 0.0, 45.0)
        .translate(
            -LABEL_X / 2.0 + 26.0 + i as f64 * 22.0,
            -LABEL_Y / 2.0 + 29.0,
            LABEL_Z / 2.0 + 2.25,
        );
        icons = icons + drop + tail;
    }
    icons
}

fn label_boundary_arrows() -> Part {
    let mut arrows = Part::empty("dewpoint_boundary_map_csg_label_boundary_arrows");
    for i in 0..2 {
        let y = -LABEL_Y / 2.0 + 18.0 + i as f64 * 18.0;
        arrows = arrows
            + centered_cube(
                format!("dewpoint_boundary_map_csg_label_boundary_arrow_shaft_{i}"),
                58.0,
                4.0,
                4.0,
            )
            .translate(56.0, y, LABEL_Z / 2.0 + 2.0)
            + centered_cube(
                format!("dewpoint_boundary_map_csg_label_boundary_arrow_head_{i}"),
                13.0,
                13.0,
                4.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(90.0, y, LABEL_Z / 2.0 + 2.0);
    }
    arrows
}

fn csg_barcode_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_csg_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 2.0 + ((seed + i) % 4) as f64 * 1.6;
        let height = y - 7.0 - (i % 2) as f64 * 3.0;
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 1.8).translate(
                -x / 2.0 + 8.0 + i as f64 * (x - 18.0) / LABEL_BAR_COUNT as f64,
                0.0,
                z / 2.0 + 0.9,
            );
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 12.0, 4.0, z + 1.8)
        .translate(x / 2.0 - 10.0, y / 2.0 - 5.0, z / 2.0 + 0.9);
    base + bars + orientation_tab
}

fn robot_service_keepout_gauges() -> Part {
    let perimeter = rectangular_frame_xy(
        "dewpoint_boundary_map_robot_service_keepout_perimeter",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        8.0,
        KEEP_OUT_Z,
    );
    perimeter
        + front_robot_goalposts()
        + rear_cold_wall_service_arch()
        + side_service_sweep_gauges()
        + keepout_zone_tokens()
}

fn front_robot_goalposts() -> Part {
    let left = centered_cube(
        "dewpoint_boundary_map_front_robot_left_goalpost",
        18.0,
        18.0,
        88.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 90.0, -KEEP_OUT_Y / 2.0 + 34.0, 44.0);
    let right = centered_cube(
        "dewpoint_boundary_map_front_robot_right_goalpost",
        18.0,
        18.0,
        88.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 90.0, -KEEP_OUT_Y / 2.0 + 34.0, 44.0);
    let bridge = centered_cube(
        "dewpoint_boundary_map_front_robot_approach_clearance_bridge",
        KEEP_OUT_X - 180.0,
        10.0,
        10.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 34.0, 88.0);
    left + right + bridge
}

fn rear_cold_wall_service_arch() -> Part {
    let left = centered_cube(
        "dewpoint_boundary_map_cold_wall_service_arch_left_post",
        18.0,
        20.0,
        COLD_WALL_CLEARANCE_Z,
    )
    .translate(-260.0, KEEP_OUT_Y / 2.0 - 42.0, COLD_WALL_CLEARANCE_Z / 2.0);
    let right = centered_cube(
        "dewpoint_boundary_map_cold_wall_service_arch_right_post",
        18.0,
        20.0,
        COLD_WALL_CLEARANCE_Z,
    )
    .translate(260.0, KEEP_OUT_Y / 2.0 - 42.0, COLD_WALL_CLEARANCE_Z / 2.0);
    let top = centered_cube(
        "dewpoint_boundary_map_cold_wall_service_arch_top_bar",
        538.0,
        14.0,
        12.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 42.0, COLD_WALL_CLEARANCE_Z + 6.0);
    left + right + top
}

fn side_service_sweep_gauges() -> Part {
    let left = centered_cube(
        "dewpoint_boundary_map_left_side_service_sweep_gauge",
        12.0,
        KEEP_OUT_Y - 170.0,
        46.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 54.0, 0.0, 23.0);
    let right = centered_cube(
        "dewpoint_boundary_map_right_side_service_sweep_gauge",
        12.0,
        KEEP_OUT_Y - 170.0,
        46.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 54.0, 0.0, 23.0);
    left + right
}

fn keepout_zone_tokens() -> Part {
    let mut tokens = Part::empty("dewpoint_boundary_map_keepout_zone_tokens");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        let token = centered_cylinder(
            format!("dewpoint_boundary_map_keepout_zone_token_{i}"),
            12.0,
            5.0,
            32,
        )
        .translate(
            centered_index(i, KEEP_OUT_GAUGE_COUNT, 58.0),
            -KEEP_OUT_Y / 2.0 + 76.0,
            KEEP_OUT_Z / 2.0 + 2.5,
        );
        let notch = centered_cube(
            format!("dewpoint_boundary_map_keepout_zone_token_notch_{i}"),
            5.0,
            16.0,
            6.0,
        )
        .rotate(0.0, 0.0, i as f64 * 36.0)
        .translate(
            centered_index(i, KEEP_OUT_GAUGE_COUNT, 58.0),
            -KEEP_OUT_Y / 2.0 + 76.0,
            KEEP_OUT_Z / 2.0 + 2.5,
        );
        tokens = tokens + (token - notch);
    }
    tokens
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    let disc = centered_cylinder(format!("{name}_disc"), 13.0, 5.0, 40);
    let x_mark = centered_cube(format!("{name}_x_mark"), 22.0, 3.0, 5.8);
    let y_mark = centered_cube(format!("{name}_y_mark"), 3.0, 22.0, 5.8);
    disc + x_mark + y_mark
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    wall: f64,
    z: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_contract_is_stable() {
        assert_design_constraints();
        assert_eq!(OUTPUTS.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"csg_label_geometry"));
    }

    #[test]
    fn condensate_capture_exceeds_challenge_volume() {
        assert!(droplet_well_capacity_ml() > condensate_challenge_volume_ml());
    }

    #[test]
    fn disposition_lanes_are_release_hold_reject() {
        let names: Vec<_> = DispositionLane::all()
            .into_iter()
            .map(DispositionLane::name)
            .collect();
        assert_eq!(names, ["release", "hold", "reject"]);
    }
}
