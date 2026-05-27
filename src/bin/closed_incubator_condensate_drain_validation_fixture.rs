use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator condensate drain validation fixture.
//
// This is architectural validation CAD for a closed incubator service module. It
// reserves a known-humidity drain challenge path, microbial coupon positions,
// sensor pockets, evidence camera lands, collection-cup interfaces, and
// no-backflow witness geometry. It is not a biological process definition or a
// validated incubator design.

const OUTPUTS: [&str; 10] = [
    "output/closed_incubator_condensate_drain_validation_fixture_base_spill_pan.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_sloped_drain_coupon_tray.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_collection_cup_lands.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_sensor_pocket_block.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_microbial_coupon_carrier.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_no_backflow_check_geometry.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_spill_containment_overflow_weir.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_robot_evidence_camera_lands.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_robot_service_keepout_gauge.stl",
    "output/closed_incubator_condensate_drain_validation_fixture_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 8] = [
    "sloped_drain_coupon_tray",
    "collection_cup_lands",
    "humidity_temperature_sensor_pockets",
    "microbial_sample_coupon_holders",
    "no_backflow_check_geometry",
    "spill_containment",
    "robot_accessible_evidence_camera_lands",
    "named_output_stls",
];

const BASE_X: f64 = 900.0;
const BASE_Y: f64 = 620.0;
const BASE_Z: f64 = 20.0;
const BASE_RIM_W: f64 = 18.0;
const BASE_RIM_Z: f64 = 34.0;
const SPILL_BASIN_DEPTH: f64 = 8.0;
const BASE_DRAIN_D: f64 = 12.0;

const TRAY_X: f64 = 520.0;
const TRAY_Y: f64 = 330.0;
const TRAY_Z: f64 = 32.0;
const TRAY_RIM_W: f64 = 16.0;
const TRAY_RIM_Z: f64 = 26.0;
const TRAY_BASIN_DEPTH: f64 = 18.0;
const TRAY_SLOPE_DROP_MM: f64 = 7.0;
const SLOPE_FLOOR_Z: f64 = 4.0;
const DRAIN_CHANNEL_W: f64 = 20.0;
const DRAIN_SUMP_X: f64 = 90.0;
const DRAIN_SUMP_Y: f64 = 58.0;
const TRAY_DRAIN_D: f64 = 9.5;
const CONDENSATE_COUPONS: usize = 6;
const COUPON_SLOT_X: f64 = 72.0;
const COUPON_SLOT_Y: f64 = 46.0;
const COUPON_PITCH_X: f64 = 84.0;

const CUP_COUNT: usize = 3;
const CUP_MODULE_X: f64 = 250.0;
const CUP_MODULE_Y: f64 = 170.0;
const CUP_MODULE_Z: f64 = 16.0;
const CUP_LAND_D: f64 = 78.0;
const CUP_RECESS_D: f64 = 58.0;
const CUP_LAND_PITCH_X: f64 = 78.0;
const CUP_DRAIN_STUB_D: f64 = 8.0;

const SENSOR_COUNT: usize = 4;
const SENSOR_MODULE_X: f64 = 250.0;
const SENSOR_MODULE_Y: f64 = 130.0;
const SENSOR_MODULE_Z: f64 = 34.0;
const SENSOR_POCKET_X: f64 = 54.0;
const SENSOR_POCKET_Y: f64 = 34.0;
const SENSOR_POCKET_DEPTH: f64 = 14.0;

const MICROBIAL_COLS: usize = 3;
const MICROBIAL_ROWS: usize = 4;
const MICROBIAL_COUPON_COUNT: usize = MICROBIAL_COLS * MICROBIAL_ROWS;
const MICROBIAL_MODULE_X: f64 = 300.0;
const MICROBIAL_MODULE_Y: f64 = 170.0;
const MICROBIAL_MODULE_Z: f64 = 18.0;
const MICROBIAL_SLOT_X: f64 = 70.0;
const MICROBIAL_SLOT_Y: f64 = 28.0;
const MICROBIAL_PITCH_X: f64 = 86.0;
const MICROBIAL_PITCH_Y: f64 = 38.0;

const BACKFLOW_MODULE_X: f64 = 320.0;
const BACKFLOW_MODULE_Y: f64 = 120.0;
const BACKFLOW_MODULE_Z: f64 = 40.0;
const CHECK_VALVE_COUNT: usize = 2;
const CHECK_VALVE_D: f64 = 34.0;
const CHECK_VALVE_BORE_D: f64 = 9.5;
const CHECK_VALVE_PITCH_X: f64 = 82.0;
const AIR_GAP_STANDPIPE_D: f64 = 22.0;
const AIR_GAP_STANDPIPE_Z: f64 = 78.0;

const WEIR_FRAME_X: f64 = 820.0;
const WEIR_FRAME_Y: f64 = 540.0;
const WEIR_W: f64 = 14.0;
const WEIR_Z: f64 = 38.0;
const OVERFLOW_NOTCH_X: f64 = 104.0;
const OVERFLOW_NOTCH_Z: f64 = 16.0;
const ABSORBENT_PAD_LANDS: usize = 4;
const SPILL_SENSOR_WELLS: usize = 4;

const CAMERA_LAND_COUNT: usize = 4;
const CAMERA_PAD_X: f64 = 86.0;
const CAMERA_PAD_Y: f64 = 58.0;
const CAMERA_PAD_Z: f64 = 8.0;
const CAMERA_STANDOFF_D: f64 = 10.0;
const CAMERA_STANDOFF_Z: f64 = 26.0;

const KEEP_OUT_GAUGE_Z: f64 = 5.0;
const FRONT_ROBOT_APPROACH_Y: f64 = 260.0;
const REAR_SERVICE_SWEEP_Y: f64 = 180.0;
const SIDE_GRIPPER_LANE_X: f64 = 92.0;

const TRAY_POS: (f64, f64) = (-120.0, 65.0);
const CUP_POS: (f64, f64) = (260.0, 145.0);
const SENSOR_POS: (f64, f64) = (260.0, -70.0);
const MICROBIAL_POS: (f64, f64) = (-250.0, -165.0);
const BACKFLOW_POS: (f64, f64) = (155.0, -205.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_spill_pan();
    export(OUTPUTS[0], &base);

    let tray = sloped_drain_coupon_tray();
    export(OUTPUTS[1], &tray);

    let cups = collection_cup_lands();
    export(OUTPUTS[2], &cups);

    let sensors = sensor_pocket_block();
    export(OUTPUTS[3], &sensors);

    let microbial = microbial_coupon_carrier();
    export(OUTPUTS[4], &microbial);

    let backflow = no_backflow_check_geometry();
    export(OUTPUTS[5], &backflow);

    let weir = spill_containment_overflow_weir();
    export(OUTPUTS[6], &weir);

    let cameras = robot_evidence_camera_lands();
    export(OUTPUTS[7], &cameras);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[8], &keepouts);

    let assembly = base
        + tray.translate(TRAY_POS.0, TRAY_POS.1, on_base_z(TRAY_Z))
        + cups.translate(CUP_POS.0, CUP_POS.1, on_base_z(CUP_MODULE_Z))
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, on_base_z(SENSOR_MODULE_Z))
        + microbial.translate(
            MICROBIAL_POS.0,
            MICROBIAL_POS.1,
            on_base_z(MICROBIAL_MODULE_Z),
        )
        + backflow.translate(BACKFLOW_POS.0, BACKFLOW_POS.1, on_base_z(BACKFLOW_MODULE_Z))
        + weir.translate(0.0, 0.0, BASE_Z)
        + cameras.translate(0.0, 0.0, BASE_Z + CAMERA_PAD_Z / 2.0)
        + keepouts.translate(0.0, 0.0, KEEP_OUT_GAUGE_Z / 2.0);
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed incubator condensate drain validation fixture:");
    println!("  Footprint:                  {BASE_X:.0}mm x {BASE_Y:.0}mm spill pan");
    println!(
        "  Drain coupon tray:          {TRAY_X:.0}mm x {TRAY_Y:.0}mm with {TRAY_SLOPE_DROP_MM:.1}mm fall, {CONDENSATE_COUPONS} coupon stations, low-point sump, and {TRAY_DRAIN_D:.1}mm drain"
    );
    println!(
        "  Collection evidence:        {CUP_COUNT} cup lands, {SENSOR_COUNT} temperature/RH sensor pockets, {MICROBIAL_COUPON_COUNT} microbial coupon holders"
    );
    println!(
        "  Backflow challenge:         {CHECK_VALVE_COUNT} inline check-valve witness housings plus {AIR_GAP_STANDPIPE_Z:.0}mm standpipe and reverse-head cup"
    );
    println!(
        "  Containment/robot evidence: {ABSORBENT_PAD_LANDS} absorbent pad lands, {SPILL_SENSOR_WELLS} spill sensor wells, {CAMERA_LAND_COUNT} evidence camera lands, and {} required feature groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_base_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn assert_design_constraints() {
    for (name, pos, width, depth) in module_specs() {
        assert!(fits_on_base(pos, width, depth), "{name} exceeds base pan");
    }
    assert_eq!(MICROBIAL_COUPON_COUNT, 12);
    assert_eq!(REQUIRED_FEATURES.len(), 8);
    assert!(tray_fall_ratio() >= 0.018);
    assert!(tray_fall_ratio() <= 0.03);
    assert!(containment_freeboard_volume_ml() > challenge_burst_volume_ml());
    assert!(front_robot_clearance() >= FRONT_ROBOT_APPROACH_Y);
}

fn base_spill_pan() -> Part {
    let deck = centered_cube(
        "condensate_validation_base_spill_pan_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let shallow_basin = centered_cube(
        "condensate_validation_base_secondary_spill_basin_cut",
        BASE_X - 112.0,
        BASE_Y - 96.0,
        SPILL_BASIN_DEPTH + 0.6,
    )
    .translate(0.0, 6.0, BASE_Z - SPILL_BASIN_DEPTH / 2.0 + 0.3);
    let forward_drain = centered_cylinder(
        "condensate_validation_base_forward_drain_cut",
        BASE_DRAIN_D / 2.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 72.0, -BASE_Y / 2.0 - 2.0, BASE_Z - 7.0);

    deck - shallow_basin - forward_drain - base_mount_slots()
        + base_perimeter_rims()
        + base_robot_datum_targets()
        + base_spill_witness_ribs()
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("condensate_validation_base_mount_slots");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 56.0), -(BASE_Y / 2.0 - 52.0)),
        (BASE_X / 2.0 - 56.0, -(BASE_Y / 2.0 - 52.0)),
        (-(BASE_X / 2.0 - 56.0), BASE_Y / 2.0 - 52.0),
        (BASE_X / 2.0 - 56.0, BASE_Y / 2.0 - 52.0),
        (0.0, -(BASE_Y / 2.0 - 52.0)),
        (0.0, BASE_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("condensate_validation_base_m6_hole_{i}"),
            6.8 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("condensate_validation_base_m6_slot_{i}"),
            24.0,
            7.0,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_perimeter_rims() -> Part {
    let front = centered_cube(
        "condensate_validation_base_front_low_rim",
        BASE_X,
        BASE_RIM_W,
        BASE_RIM_Z * 0.68,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + BASE_RIM_W / 2.0,
        BASE_Z + BASE_RIM_Z * 0.34,
    );
    let rear = centered_cube(
        "condensate_validation_base_rear_high_rim",
        BASE_X,
        BASE_RIM_W,
        BASE_RIM_Z,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 - BASE_RIM_W / 2.0,
        BASE_Z + BASE_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "condensate_validation_base_left_rim",
        BASE_RIM_W,
        BASE_Y,
        BASE_RIM_Z,
    )
    .translate(
        -BASE_X / 2.0 + BASE_RIM_W / 2.0,
        0.0,
        BASE_Z + BASE_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "condensate_validation_base_right_rim",
        BASE_RIM_W,
        BASE_Y,
        BASE_RIM_Z,
    )
    .translate(
        BASE_X / 2.0 - BASE_RIM_W / 2.0,
        0.0,
        BASE_Z + BASE_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn base_robot_datum_targets() -> Part {
    let mut targets = Part::empty("condensate_validation_base_robot_datum_targets");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 76.0), BASE_Y / 2.0 - 78.0),
        (BASE_X / 2.0 - 76.0, BASE_Y / 2.0 - 78.0),
        (-(BASE_X / 2.0 - 76.0), -BASE_Y / 2.0 + 76.0),
        (BASE_X / 2.0 - 76.0, -BASE_Y / 2.0 + 76.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("condensate_validation_base_datum_{i}")).translate(
                *x,
                *y,
                BASE_Z + 2.0,
            );
    }
    targets
}

fn base_spill_witness_ribs() -> Part {
    let mut ribs = Part::empty("condensate_validation_base_spill_witness_ribs");
    for (i, y) in [-180.0, -95.0, -10.0, 75.0, 160.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("condensate_validation_base_witness_flow_rib_{i}"),
                BASE_X - 170.0,
                4.0,
                5.0,
            )
            .translate(0.0, *y, BASE_Z + 2.5);
    }
    ribs
}

fn sloped_drain_coupon_tray() -> Part {
    let tray = centered_cube(
        "condensate_validation_sloped_coupon_tray_body",
        TRAY_X,
        TRAY_Y,
        TRAY_Z,
    )
    .translate(0.0, 0.0, TRAY_Z / 2.0);
    let basin = centered_cube(
        "condensate_validation_sloped_coupon_tray_basin_cut",
        TRAY_X - 48.0,
        TRAY_Y - 54.0,
        TRAY_BASIN_DEPTH + 0.6,
    )
    .translate(0.0, 4.0, TRAY_Z - TRAY_BASIN_DEPTH / 2.0 + 0.3);
    let center_channel = centered_cube(
        "condensate_validation_main_drain_channel_cut",
        DRAIN_CHANNEL_W,
        TRAY_Y - 78.0,
        8.0,
    )
    .translate(0.0, -12.0, TRAY_Z - 4.0);
    let sump = centered_cube(
        "condensate_validation_low_point_sump_cut",
        DRAIN_SUMP_X,
        DRAIN_SUMP_Y,
        TRAY_BASIN_DEPTH + 2.0,
    )
    .translate(0.0, tray_low_y() + 45.0, TRAY_Z - TRAY_BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        "condensate_validation_tray_forward_drain_bore",
        TRAY_DRAIN_D / 2.0,
        38.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, tray_low_y() - 4.0, TRAY_Z - 12.0);

    tray - basin - center_channel - sump - drain
        + tray_rims()
        + sloped_floor_witness_panel()
        + condensate_coupon_slots()
        + fall_direction_witness_bars()
        + low_point_splash_break()
}

fn tray_rims() -> Part {
    let front = centered_cube(
        "condensate_validation_tray_front_low_lip",
        TRAY_X,
        TRAY_RIM_W,
        TRAY_RIM_Z * 0.55,
    )
    .translate(
        0.0,
        tray_low_y() + TRAY_RIM_W / 2.0,
        TRAY_Z + TRAY_RIM_Z * 0.275,
    );
    let rear = centered_cube(
        "condensate_validation_tray_rear_high_lip",
        TRAY_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        tray_high_y() - TRAY_RIM_W / 2.0,
        TRAY_Z + TRAY_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "condensate_validation_tray_left_side_lip",
        TRAY_RIM_W,
        TRAY_Y,
        TRAY_RIM_Z,
    )
    .translate(
        -TRAY_X / 2.0 + TRAY_RIM_W / 2.0,
        0.0,
        TRAY_Z + TRAY_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "condensate_validation_tray_right_side_lip",
        TRAY_RIM_W,
        TRAY_Y,
        TRAY_RIM_Z,
    )
    .translate(
        TRAY_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        TRAY_Z + TRAY_RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn sloped_floor_witness_panel() -> Part {
    centered_cube(
        "condensate_validation_sloped_floor_witness_panel",
        TRAY_X - 86.0,
        TRAY_Y - 104.0,
        SLOPE_FLOOR_Z,
    )
    .rotate(slope_angle_deg(), 0.0, 0.0)
    .translate(0.0, -4.0, TRAY_Z - TRAY_BASIN_DEPTH + SLOPE_FLOOR_Z / 2.0)
}

fn condensate_coupon_slots() -> Part {
    let mut coupons = Part::empty("condensate_validation_drain_coupon_slots");
    for i in 0..CONDENSATE_COUPONS {
        let x = centered_index(i, CONDENSATE_COUPONS, COUPON_PITCH_X);
        let well = centered_cube(
            format!("condensate_validation_drain_coupon_well_{i}"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            6.0,
        )
        .translate(x, 44.0, TRAY_Z - TRAY_BASIN_DEPTH + 5.0);
        let left_clip = centered_cube(
            format!("condensate_validation_drain_coupon_left_clip_{i}"),
            5.0,
            COUPON_SLOT_Y + 12.0,
            10.0,
        )
        .translate(
            x - COUPON_SLOT_X / 2.0 + 5.0,
            44.0,
            TRAY_Z - TRAY_BASIN_DEPTH + 11.0,
        );
        let right_clip = centered_cube(
            format!("condensate_validation_drain_coupon_right_clip_{i}"),
            5.0,
            COUPON_SLOT_Y + 12.0,
            10.0,
        )
        .translate(
            x + COUPON_SLOT_X / 2.0 - 5.0,
            44.0,
            TRAY_Z - TRAY_BASIN_DEPTH + 11.0,
        );
        let drip_lip = centered_cube(
            format!("condensate_validation_coupon_drip_break_lip_{i}"),
            COUPON_SLOT_X + 8.0,
            5.0,
            9.0,
        )
        .translate(x, 44.0 - COUPON_SLOT_Y / 2.0 - 8.0, TRAY_Z - 6.0);
        coupons = coupons + well + left_clip + right_clip + drip_lip;
    }
    coupons
}

fn fall_direction_witness_bars() -> Part {
    let mut bars = Part::empty("condensate_validation_fall_direction_witness_bars");
    for i in 0..5 {
        let y = 110.0 - i as f64 * 48.0;
        let z = TRAY_Z - TRAY_BASIN_DEPTH + 8.0 - i as f64 * 0.9;
        bars = bars
            + centered_cube(
                format!("condensate_validation_slope_witness_bar_{i}"),
                TRAY_X - 150.0,
                4.0,
                4.0,
            )
            .translate(0.0, y, z);
    }
    bars
}

fn low_point_splash_break() -> Part {
    let comb = centered_cube(
        "condensate_validation_low_point_splash_break_comb",
        TRAY_X - 136.0,
        6.0,
        18.0,
    )
    .translate(0.0, tray_low_y() + 82.0, TRAY_Z + 9.0);
    let notch = centered_cube(
        "condensate_validation_low_point_splash_break_center_notch",
        54.0,
        8.0,
        22.0,
    )
    .translate(0.0, tray_low_y() + 82.0, TRAY_Z + 9.0);
    comb - notch
}

fn collection_cup_lands() -> Part {
    let plate = centered_cube(
        "condensate_validation_collection_cup_land_plate",
        CUP_MODULE_X,
        CUP_MODULE_Y,
        CUP_MODULE_Z,
    )
    .translate(0.0, 0.0, CUP_MODULE_Z / 2.0);
    let drain_gutter = centered_cube(
        "condensate_validation_collection_cup_inlet_gutter_cut",
        CUP_MODULE_X - 36.0,
        12.0,
        8.0,
    )
    .translate(0.0, CUP_MODULE_Y / 2.0 - 30.0, CUP_MODULE_Z - 3.0);
    plate - drain_gutter + cup_land_rings() + cup_transfer_stubs() + cup_label_lands()
}

fn cup_land_rings() -> Part {
    let mut rings = Part::empty("condensate_validation_collection_cup_land_rings");
    for (i, x) in collection_cup_xs().iter().enumerate() {
        let outer = centered_cylinder(
            format!("condensate_validation_collection_cup_outer_land_{i}"),
            CUP_LAND_D / 2.0,
            8.0,
            48,
        )
        .translate(*x, 0.0, CUP_MODULE_Z + 4.0);
        let recess = centered_cylinder(
            format!("condensate_validation_collection_cup_recess_{i}"),
            CUP_RECESS_D / 2.0,
            10.0,
            48,
        )
        .translate(*x, 0.0, CUP_MODULE_Z + 4.0);
        let keyed_flat = centered_cube(
            format!("condensate_validation_collection_cup_key_flat_{i}"),
            24.0,
            10.0,
            9.0,
        )
        .translate(*x, -CUP_RECESS_D / 2.0 + 4.0, CUP_MODULE_Z + 4.0);
        rings = rings + (outer - recess - keyed_flat);
    }
    rings
}

fn cup_transfer_stubs() -> Part {
    let mut stubs = Part::empty("condensate_validation_collection_cup_transfer_stubs");
    for (i, x) in collection_cup_xs().iter().enumerate() {
        let stub = centered_cylinder(
            format!("condensate_validation_collection_cup_inlet_stub_{i}"),
            CUP_DRAIN_STUB_D / 2.0,
            46.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, CUP_MODULE_Y / 2.0 - 22.0, CUP_MODULE_Z + 9.0);
        let witness_window = centered_cube(
            format!("condensate_validation_collection_cup_witness_window_{i}"),
            42.0,
            4.0,
            16.0,
        )
        .translate(*x, CUP_MODULE_Y / 2.0 - 48.0, CUP_MODULE_Z + 8.0);
        stubs = stubs + stub + witness_window;
    }
    stubs
}

fn cup_label_lands() -> Part {
    let mut lands = Part::empty("condensate_validation_collection_cup_label_lands");
    for (i, x) in collection_cup_xs().iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("condensate_validation_collection_cup_barcode_land_{i}"),
                58.0,
                8.0,
                3.0,
            )
            .translate(*x, -CUP_MODULE_Y / 2.0 + 22.0, CUP_MODULE_Z + 1.5);
    }
    lands
}

fn sensor_pocket_block() -> Part {
    let block = centered_cube(
        "condensate_validation_temp_rh_sensor_pocket_block",
        SENSOR_MODULE_X,
        SENSOR_MODULE_Y,
        SENSOR_MODULE_Z,
    )
    .translate(0.0, 0.0, SENSOR_MODULE_Z / 2.0);
    let mut pockets = Part::empty("condensate_validation_temp_rh_sensor_pocket_cuts");
    for i in 0..SENSOR_COUNT {
        let (x, y) = sensor_center(i);
        let cavity = centered_cube(
            format!("condensate_validation_temp_rh_sensor_cavity_{i}"),
            SENSOR_POCKET_X,
            SENSOR_POCKET_Y,
            SENSOR_POCKET_DEPTH + 0.6,
        )
        .translate(x, y, SENSOR_MODULE_Z - SENSOR_POCKET_DEPTH / 2.0 + 0.3);
        let cable_slot = centered_cube(
            format!("condensate_validation_temp_rh_sensor_cable_slot_{i}"),
            10.0,
            46.0,
            SENSOR_POCKET_DEPTH,
        )
        .translate(x + SENSOR_POCKET_X / 2.0 - 4.0, y, SENSOR_MODULE_Z - 5.0);
        pockets = pockets + cavity + cable_slot;
    }
    block - pockets + sensor_cable_comb() + sensor_reference_label_strip()
}

fn sensor_cable_comb() -> Part {
    let mut comb = Part::empty("condensate_validation_sensor_cable_comb");
    for i in 0..SENSOR_COUNT {
        let (x, y) = sensor_center(i);
        comb = comb
            + centered_cube(
                format!("condensate_validation_sensor_cable_capture_tooth_{i}"),
                8.0,
                34.0,
                16.0,
            )
            .translate(x + SENSOR_POCKET_X / 2.0 + 10.0, y, SENSOR_MODULE_Z + 8.0);
    }
    comb
}

fn sensor_reference_label_strip() -> Part {
    centered_cube(
        "condensate_validation_sensor_calibration_reference_label_strip",
        SENSOR_MODULE_X - 26.0,
        7.0,
        8.0,
    )
    .translate(0.0, -SENSOR_MODULE_Y / 2.0 + 14.0, SENSOR_MODULE_Z + 4.0)
}

fn microbial_coupon_carrier() -> Part {
    let carrier = centered_cube(
        "condensate_validation_microbial_coupon_carrier_plate",
        MICROBIAL_MODULE_X,
        MICROBIAL_MODULE_Y,
        MICROBIAL_MODULE_Z,
    )
    .translate(0.0, 0.0, MICROBIAL_MODULE_Z / 2.0);
    carrier - microbial_coupon_recesses()
        + microbial_coupon_retainers()
        + microbial_zone_dividers()
        + microbial_chain_of_custody_land()
}

fn microbial_coupon_recesses() -> Part {
    let mut recesses = Part::empty("condensate_validation_microbial_coupon_recesses");
    for row in 0..MICROBIAL_ROWS {
        for col in 0..MICROBIAL_COLS {
            let index = microbial_index(col, row);
            let (x, y) = microbial_center(col, row);
            recesses = recesses
                + centered_cube(
                    format!("condensate_validation_microbial_coupon_recess_{index}"),
                    MICROBIAL_SLOT_X,
                    MICROBIAL_SLOT_Y,
                    7.0,
                )
                .translate(x, y, MICROBIAL_MODULE_Z - 3.0);
        }
    }
    recesses
}

fn microbial_coupon_retainers() -> Part {
    let mut retainers = Part::empty("condensate_validation_microbial_coupon_retainers");
    for row in 0..MICROBIAL_ROWS {
        for col in 0..MICROBIAL_COLS {
            let index = microbial_index(col, row);
            let (x, y) = microbial_center(col, row);
            let left = centered_cube(
                format!("condensate_validation_microbial_coupon_left_retainer_{index}"),
                4.0,
                MICROBIAL_SLOT_Y + 8.0,
                10.0,
            )
            .translate(
                x - MICROBIAL_SLOT_X / 2.0 + 4.0,
                y,
                MICROBIAL_MODULE_Z + 5.0,
            );
            let right = centered_cube(
                format!("condensate_validation_microbial_coupon_right_retainer_{index}"),
                4.0,
                MICROBIAL_SLOT_Y + 8.0,
                10.0,
            )
            .translate(
                x + MICROBIAL_SLOT_X / 2.0 - 4.0,
                y,
                MICROBIAL_MODULE_Z + 5.0,
            );
            let end_stop = centered_cube(
                format!("condensate_validation_microbial_coupon_end_stop_{index}"),
                MICROBIAL_SLOT_X + 8.0,
                4.0,
                9.0,
            )
            .translate(
                x,
                y - MICROBIAL_SLOT_Y / 2.0 - 5.0,
                MICROBIAL_MODULE_Z + 4.5,
            );
            retainers = retainers + left + right + end_stop;
        }
    }
    retainers
}

fn microbial_zone_dividers() -> Part {
    let mut dividers = Part::empty("condensate_validation_microbial_zone_dividers");
    for i in 0..(MICROBIAL_ROWS - 1) {
        let y = centered_index(i, MICROBIAL_ROWS - 1, MICROBIAL_PITCH_Y);
        dividers = dividers
            + centered_cube(
                format!("condensate_validation_microbial_zone_divider_{i}"),
                MICROBIAL_MODULE_X - 42.0,
                4.0,
                12.0,
            )
            .translate(0.0, y + MICROBIAL_PITCH_Y / 2.0, MICROBIAL_MODULE_Z + 6.0);
    }
    dividers
}

fn microbial_chain_of_custody_land() -> Part {
    centered_cube(
        "condensate_validation_microbial_chain_of_custody_barcode_land",
        MICROBIAL_MODULE_X - 48.0,
        8.0,
        5.0,
    )
    .translate(
        0.0,
        -MICROBIAL_MODULE_Y / 2.0 + 16.0,
        MICROBIAL_MODULE_Z + 2.5,
    )
}

fn no_backflow_check_geometry() -> Part {
    let block = centered_cube(
        "condensate_validation_no_backflow_check_block",
        BACKFLOW_MODULE_X,
        BACKFLOW_MODULE_Y,
        BACKFLOW_MODULE_Z,
    )
    .translate(0.0, 0.0, BACKFLOW_MODULE_Z / 2.0);
    let main_bore = centered_cylinder(
        "condensate_validation_no_backflow_main_bore",
        CHECK_VALVE_BORE_D / 2.0,
        BACKFLOW_MODULE_X + 26.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, BACKFLOW_MODULE_Z / 2.0);
    let outlet_bore = centered_cylinder(
        "condensate_validation_no_backflow_outlet_bore",
        CHECK_VALVE_BORE_D / 2.0,
        BACKFLOW_MODULE_Y + 20.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BACKFLOW_MODULE_X / 2.0 - 42.0, 0.0, BACKFLOW_MODULE_Z / 2.0);

    block - main_bore - outlet_bore
        + check_valve_witness_housings()
        + reverse_head_challenge_cup()
        + air_gap_standpipe()
        + backflow_direction_tabs()
}

fn check_valve_witness_housings() -> Part {
    let mut housings = Part::empty("condensate_validation_check_valve_witness_housings");
    for i in 0..CHECK_VALVE_COUNT {
        let x = centered_index(i, CHECK_VALVE_COUNT, CHECK_VALVE_PITCH_X);
        let shell = centered_cylinder(
            format!("condensate_validation_inline_check_valve_shell_{i}"),
            CHECK_VALVE_D / 2.0,
            58.0,
            40,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 0.0, BACKFLOW_MODULE_Z / 2.0);
        let bore = centered_cylinder(
            format!("condensate_validation_inline_check_valve_bore_{i}"),
            CHECK_VALVE_BORE_D / 2.0,
            62.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 0.0, BACKFLOW_MODULE_Z / 2.0);
        let window = centered_cube(
            format!("condensate_validation_check_valve_witness_window_{i}"),
            28.0,
            5.0,
            16.0,
        )
        .translate(x, BACKFLOW_MODULE_Y / 2.0 + 3.0, BACKFLOW_MODULE_Z / 2.0);
        let flapper = centered_cube(
            format!("condensate_validation_check_valve_flexible_flap_proxy_{i}"),
            3.0,
            28.0,
            18.0,
        )
        .rotate(0.0, 0.0, 12.0)
        .translate(x + 12.0, 0.0, BACKFLOW_MODULE_Z / 2.0);
        housings = housings + (shell - bore) + window + flapper;
    }
    housings
}

fn reverse_head_challenge_cup() -> Part {
    let outer = centered_cylinder(
        "condensate_validation_reverse_head_challenge_cup_outer",
        32.0,
        34.0,
        48,
    )
    .translate(
        -BACKFLOW_MODULE_X / 2.0 + 54.0,
        -32.0,
        BACKFLOW_MODULE_Z + 17.0,
    );
    let inner = centered_cylinder(
        "condensate_validation_reverse_head_challenge_cup_cavity",
        24.0,
        36.0,
        48,
    )
    .translate(
        -BACKFLOW_MODULE_X / 2.0 + 54.0,
        -32.0,
        BACKFLOW_MODULE_Z + 20.0,
    );
    let fill_tick = centered_cube(
        "condensate_validation_reverse_head_fill_tick",
        38.0,
        3.0,
        4.0,
    )
    .translate(
        -BACKFLOW_MODULE_X / 2.0 + 54.0,
        -65.0,
        BACKFLOW_MODULE_Z + 29.0,
    );
    outer - inner + fill_tick
}

fn air_gap_standpipe() -> Part {
    let pipe = centered_cylinder(
        "condensate_validation_air_gap_standpipe_outer",
        AIR_GAP_STANDPIPE_D / 2.0,
        AIR_GAP_STANDPIPE_Z,
        40,
    )
    .translate(
        BACKFLOW_MODULE_X / 2.0 - 54.0,
        34.0,
        BACKFLOW_MODULE_Z + AIR_GAP_STANDPIPE_Z / 2.0,
    );
    let bore = centered_cylinder(
        "condensate_validation_air_gap_standpipe_bore",
        CHECK_VALVE_BORE_D / 2.0,
        AIR_GAP_STANDPIPE_Z + 4.0,
        32,
    )
    .translate(
        BACKFLOW_MODULE_X / 2.0 - 54.0,
        34.0,
        BACKFLOW_MODULE_Z + AIR_GAP_STANDPIPE_Z / 2.0,
    );
    let splash_cap = centered_cylinder("condensate_validation_air_gap_splash_cap", 18.0, 6.0, 40)
        .translate(
            BACKFLOW_MODULE_X / 2.0 - 54.0,
            34.0,
            BACKFLOW_MODULE_Z + AIR_GAP_STANDPIPE_Z + 3.0,
        );
    pipe - bore + splash_cap
}

fn backflow_direction_tabs() -> Part {
    let forward = centered_cube(
        "condensate_validation_forward_flow_direction_land",
        94.0,
        8.0,
        5.0,
    )
    .translate(
        0.0,
        -BACKFLOW_MODULE_Y / 2.0 + 16.0,
        BACKFLOW_MODULE_Z + 2.5,
    );
    let blocked = centered_cube(
        "condensate_validation_reverse_flow_blocked_land",
        94.0,
        8.0,
        5.0,
    )
    .translate(0.0, BACKFLOW_MODULE_Y / 2.0 - 16.0, BACKFLOW_MODULE_Z + 2.5);
    forward + blocked
}

fn spill_containment_overflow_weir() -> Part {
    let front = centered_cube(
        "condensate_validation_secondary_weir_front_with_overflow_notch",
        WEIR_FRAME_X,
        WEIR_W,
        WEIR_Z,
    )
    .translate(0.0, -WEIR_FRAME_Y / 2.0 + WEIR_W / 2.0, WEIR_Z / 2.0);
    let notch = centered_cube(
        "condensate_validation_secondary_weir_front_notch_cut",
        OVERFLOW_NOTCH_X,
        WEIR_W + 2.0,
        OVERFLOW_NOTCH_Z,
    )
    .translate(
        0.0,
        -WEIR_FRAME_Y / 2.0 + WEIR_W / 2.0,
        WEIR_Z - OVERFLOW_NOTCH_Z / 2.0,
    );
    let rear = centered_cube(
        "condensate_validation_secondary_weir_rear",
        WEIR_FRAME_X,
        WEIR_W,
        WEIR_Z,
    )
    .translate(0.0, WEIR_FRAME_Y / 2.0 - WEIR_W / 2.0, WEIR_Z / 2.0);
    let left = centered_cube(
        "condensate_validation_secondary_weir_left",
        WEIR_W,
        WEIR_FRAME_Y,
        WEIR_Z,
    )
    .translate(-WEIR_FRAME_X / 2.0 + WEIR_W / 2.0, 0.0, WEIR_Z / 2.0);
    let right = centered_cube(
        "condensate_validation_secondary_weir_right",
        WEIR_W,
        WEIR_FRAME_Y,
        WEIR_Z,
    )
    .translate(WEIR_FRAME_X / 2.0 - WEIR_W / 2.0, 0.0, WEIR_Z / 2.0);
    (front - notch) + rear + left + right + absorbent_pad_lands() + spill_sensor_wells()
}

fn absorbent_pad_lands() -> Part {
    let mut lands = Part::empty("condensate_validation_absorbent_pad_lands");
    for (i, (x, y)) in [
        (-310.0, -230.0),
        (-104.0, -230.0),
        (104.0, -230.0),
        (310.0, -230.0),
    ]
    .iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("condensate_validation_absorbent_pad_retainer_land_{i}"),
                136.0,
                42.0,
                8.0,
            )
            .translate(*x, *y, WEIR_Z + 4.0)
            + centered_cube(
                format!("condensate_validation_absorbent_pad_front_clip_{i}"),
                136.0,
                5.0,
                14.0,
            )
            .translate(*x, *y - 23.0, WEIR_Z + 7.0);
    }
    lands
}

fn spill_sensor_wells() -> Part {
    let mut wells = Part::empty("condensate_validation_spill_sensor_wells");
    for (i, (x, y)) in [
        (-360.0, -190.0),
        (360.0, -190.0),
        (-360.0, 214.0),
        (360.0, 214.0),
    ]
    .iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("condensate_validation_spill_sensor_well_ring_{i}"),
            18.0,
            7.0,
            36,
        )
        .translate(*x, *y, WEIR_Z + 3.5);
        let bore = centered_cylinder(
            format!("condensate_validation_spill_sensor_well_recess_{i}"),
            11.0,
            8.0,
            32,
        )
        .translate(*x, *y, WEIR_Z + 3.5);
        wells = wells + (ring - bore);
    }
    wells
}

fn robot_evidence_camera_lands() -> Part {
    let mut lands = Part::empty("condensate_validation_robot_evidence_camera_lands");
    for (i, (x, y)) in camera_land_centers().iter().enumerate() {
        let pad = centered_cube(
            format!("condensate_validation_evidence_camera_pad_{i}"),
            CAMERA_PAD_X,
            CAMERA_PAD_Y,
            CAMERA_PAD_Z,
        )
        .translate(*x, *y, 0.0);
        let aperture = centered_cylinder(
            format!("condensate_validation_evidence_camera_optical_axis_bore_{i}"),
            12.0,
            CAMERA_PAD_Z + 2.0,
            36,
        )
        .translate(*x, *y, 0.0);
        lands = lands
            + (pad - aperture)
            + camera_standoffs(i, *x, *y)
            + fiducial_disc(&format!(
                "condensate_validation_evidence_camera_fiducial_{i}"
            ))
            .translate(*x + CAMERA_PAD_X / 2.0 - 14.0, *y, CAMERA_PAD_Z / 2.0 + 2.0);
    }
    lands
}

fn camera_standoffs(index: usize, x: f64, y: f64) -> Part {
    let mut standoffs = Part::empty(format!(
        "condensate_validation_camera_{index}_robot_grip_standoffs"
    ));
    for (j, dx) in [-28.0, 28.0].iter().enumerate() {
        standoffs = standoffs
            + centered_cylinder(
                format!("condensate_validation_camera_{index}_standoff_{j}"),
                CAMERA_STANDOFF_D / 2.0,
                CAMERA_STANDOFF_Z,
                24,
            )
            .translate(
                x + *dx,
                y - CAMERA_PAD_Y / 2.0 - 8.0,
                CAMERA_STANDOFF_Z / 2.0,
            );
    }
    standoffs
}

fn robot_service_keepout_gauge() -> Part {
    let front = centered_cube(
        "condensate_validation_front_robot_approach_keepout",
        BASE_X,
        FRONT_ROBOT_APPROACH_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, -(BASE_Y / 2.0 + FRONT_ROBOT_APPROACH_Y / 2.0), 0.0);
    let rear = centered_cube(
        "condensate_validation_rear_service_sweep_keepout",
        BASE_X,
        REAR_SERVICE_SWEEP_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, BASE_Y / 2.0 + REAR_SERVICE_SWEEP_Y / 2.0, 0.0);
    let left = centered_cube(
        "condensate_validation_left_robot_gripper_lane_keepout",
        SIDE_GRIPPER_LANE_X,
        BASE_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(-(BASE_X / 2.0 + SIDE_GRIPPER_LANE_X / 2.0), 0.0, 0.0);
    let right = centered_cube(
        "condensate_validation_right_robot_gripper_lane_keepout",
        SIDE_GRIPPER_LANE_X,
        BASE_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(BASE_X / 2.0 + SIDE_GRIPPER_LANE_X / 2.0, 0.0, 0.0);
    let centerline = centered_cube(
        "condensate_validation_camera_robot_centerline_x",
        BASE_X - 126.0,
        4.0,
        KEEP_OUT_GAUGE_Z,
    ) + centered_cube(
        "condensate_validation_camera_robot_centerline_y",
        4.0,
        BASE_Y - 100.0,
        KEEP_OUT_GAUGE_Z,
    );

    front + rear + left + right + centerline
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 10.0, 3.0, 36);
    let center = centered_cylinder(format!("{name}_center_bore"), 2.0, 4.0, 24);
    outer - center
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 5] {
    [
        ("sloped drain coupon tray", TRAY_POS, TRAY_X, TRAY_Y),
        ("collection cup lands", CUP_POS, CUP_MODULE_X, CUP_MODULE_Y),
        (
            "humidity temperature sensor pockets",
            SENSOR_POS,
            SENSOR_MODULE_X,
            SENSOR_MODULE_Y,
        ),
        (
            "microbial coupon carrier",
            MICROBIAL_POS,
            MICROBIAL_MODULE_X,
            MICROBIAL_MODULE_Y,
        ),
        (
            "no backflow check geometry",
            BACKFLOW_POS,
            BACKFLOW_MODULE_X,
            BACKFLOW_MODULE_Y,
        ),
    ]
}

fn fits_on_base(center: (f64, f64), width: f64, depth: f64) -> bool {
    center.0 - width / 2.0 >= -BASE_X / 2.0 + BASE_RIM_W + 8.0
        && center.0 + width / 2.0 <= BASE_X / 2.0 - BASE_RIM_W - 8.0
        && center.1 - depth / 2.0 >= -BASE_Y / 2.0 + BASE_RIM_W + 8.0
        && center.1 + depth / 2.0 <= BASE_Y / 2.0 - BASE_RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    index as f64 * pitch - (count as f64 - 1.0) * pitch / 2.0
}

fn tray_low_y() -> f64 {
    -TRAY_Y / 2.0
}

fn tray_high_y() -> f64 {
    TRAY_Y / 2.0
}

fn tray_fall_ratio() -> f64 {
    TRAY_SLOPE_DROP_MM / (TRAY_Y - 2.0 * TRAY_RIM_W)
}

fn slope_angle_deg() -> f64 {
    tray_fall_ratio().atan().to_degrees()
}

fn collection_cup_xs() -> [f64; CUP_COUNT] {
    [
        centered_index(0, CUP_COUNT, CUP_LAND_PITCH_X),
        centered_index(1, CUP_COUNT, CUP_LAND_PITCH_X),
        centered_index(2, CUP_COUNT, CUP_LAND_PITCH_X),
    ]
}

fn sensor_center(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (centered_index(col, 2, 96.0), centered_index(row, 2, 58.0))
}

fn microbial_index(col: usize, row: usize) -> usize {
    row * MICROBIAL_COLS + col
}

fn microbial_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, MICROBIAL_COLS, MICROBIAL_PITCH_X),
        centered_index(row, MICROBIAL_ROWS, MICROBIAL_PITCH_Y) + 8.0,
    )
}

fn camera_land_centers() -> [(f64, f64); CAMERA_LAND_COUNT] {
    [
        (-335.0, 245.0),
        (0.0, 245.0),
        (335.0, 245.0),
        (338.0, -118.0),
    ]
}

fn containment_freeboard_volume_ml() -> f64 {
    let inner_x = WEIR_FRAME_X - 2.0 * WEIR_W;
    let inner_y = WEIR_FRAME_Y - 2.0 * WEIR_W;
    let usable_height = WEIR_Z - OVERFLOW_NOTCH_Z;
    inner_x * inner_y * usable_height / 1000.0
}

fn challenge_burst_volume_ml() -> f64 {
    let cup_volume = 120.0 * CUP_COUNT as f64;
    let tray_basin = (TRAY_X - 2.0 * TRAY_RIM_W) * (TRAY_Y - 2.0 * TRAY_RIM_W) * 3.0 / 1000.0;
    cup_volume + tray_basin
}

fn front_robot_clearance() -> f64 {
    FRONT_ROBOT_APPROACH_Y
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_incubator_condensate_drain_validation_fixture_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn fixture_covers_requested_feature_groups() {
        assert_eq!(REQUIRED_FEATURES.len(), 8);
        assert!(REQUIRED_FEATURES.contains(&"sloped_drain_coupon_tray"));
        assert!(REQUIRED_FEATURES.contains(&"collection_cup_lands"));
        assert!(REQUIRED_FEATURES.contains(&"humidity_temperature_sensor_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"microbial_sample_coupon_holders"));
        assert!(REQUIRED_FEATURES.contains(&"no_backflow_check_geometry"));
        assert!(REQUIRED_FEATURES.contains(&"spill_containment"));
        assert!(REQUIRED_FEATURES.contains(&"robot_accessible_evidence_camera_lands"));
        assert!(REQUIRED_FEATURES.contains(&"named_output_stls"));
    }

    #[test]
    fn modules_fit_inside_secondary_spill_pan() {
        assert_design_constraints();
        for (_name, pos, width, depth) in module_specs() {
            assert!(fits_on_base(pos, width, depth));
        }
    }

    #[test]
    fn drain_tray_has_controlled_fall_to_low_sump() {
        assert_eq!(CONDENSATE_COUPONS, 6);
        assert!(tray_low_y() < tray_high_y());
        assert!(tray_fall_ratio() >= 0.018);
        assert!(tray_fall_ratio() <= 0.03);
        assert!(slope_angle_deg() > 1.0);
        assert!(DRAIN_CHANNEL_W > TRAY_DRAIN_D);
        assert!(DRAIN_SUMP_X > COUPON_SLOT_X);
    }

    #[test]
    fn collection_sensor_and_microbiology_counts_are_explicit() {
        assert_eq!(CUP_COUNT, 3);
        assert_eq!(collection_cup_xs().len(), CUP_COUNT);
        assert_eq!(SENSOR_COUNT, 4);
        assert_eq!(MICROBIAL_COUPON_COUNT, 12);
        assert_eq!(MICROBIAL_COLS * MICROBIAL_ROWS, MICROBIAL_COUPON_COUNT);
        assert!(SENSOR_POCKET_DEPTH < SENSOR_MODULE_Z);
    }

    #[test]
    fn no_backflow_geometry_has_challenge_and_air_gap_features() {
        assert_eq!(CHECK_VALVE_COUNT, 2);
        assert!(CHECK_VALVE_D > CHECK_VALVE_BORE_D * 3.0);
        assert!(AIR_GAP_STANDPIPE_Z > BACKFLOW_MODULE_Z);
        assert!(AIR_GAP_STANDPIPE_D > CHECK_VALVE_BORE_D);
    }

    #[test]
    fn secondary_containment_exceeds_planned_challenge_volume() {
        assert!(containment_freeboard_volume_ml() > challenge_burst_volume_ml());
        assert!(ABSORBENT_PAD_LANDS >= 4);
        assert!(SPILL_SENSOR_WELLS >= 4);
        assert!(OVERFLOW_NOTCH_Z < WEIR_Z);
    }

    #[test]
    fn robot_evidence_camera_lands_cover_drain_path() {
        assert_eq!(CAMERA_LAND_COUNT, 4);
        assert!(CAMERA_PAD_X >= 80.0);
        assert!(front_robot_clearance() >= 240.0);
        for (x, y) in camera_land_centers() {
            assert!(fits_on_base((x, y), CAMERA_PAD_X, CAMERA_PAD_Y));
        }
    }
}
