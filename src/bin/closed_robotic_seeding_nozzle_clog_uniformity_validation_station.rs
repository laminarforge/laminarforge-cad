use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic seeding nozzle clog/uniformity validation station.
//
// Intent:
// - Accept the robotic seeding head inside the clean isolator before a
//   tissue-chip cassette run.
// - Run no-cell media/dye checks that reveal nozzle clogging, droplet/spray
//   misalignment, missed dispense events, wetness state, per-slot volume
//   nonuniformity, and carryover risk before cells are committed to chips.
// - Keep the validation target array at 16 positions to match the project's
//   scaled multi-chip cassette philosophy.
//
// Research basis:
// - Qian et al., Micromachines 2023, used image sensing of droplets as the
//   feedback signal for automated micro liquid dispensing control.
// - Rajabnia et al., Micromachines 2024, describes parallel droplet dispensing
//   with camera/illumination observation of droplet volume and array
//   homogeneity.
// - Montanez-Sauri et al., J Lab Autom 2011, demonstrated automated liquid
//   handler operation against arrayed microfluidic culture devices, motivating
//   cassette-scale validation before live cell seeding.
//
// This is mechanical architecture CAD only. It is not a sterile barrier
// validation, biological acceptance criterion, image-analysis algorithm, or
// pump/nozzle operating recipe.

const PREFIX: &str = "closed_robotic_seeding_nozzle_clog_uniformity_validation_station";

const OUTPUTS: [&str; 14] = [
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_isolator_deck.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_sealed_waste_containment_trough.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_seeding_head_receiver_nest.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_nozzle_alignment_datum_bank.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_sixteen_position_witness_target_array.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_bubble_clog_witness_wells.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_volume_uniformity_sample_rack.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_missed_dispense_wetness_lanes.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_carryover_flush_challenge_lanes.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_camera_illumination_fiducial_bridge.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_sensor_sampling_manifold.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_barcode_status_custody_lands.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_robot_service_keepouts.stl",
    "output/closed_robotic_seeding_nozzle_clog_uniformity_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "isolator_deck",
    "sealed_waste_containment_trough",
    "seeding_head_receiver_nest",
    "nozzle_alignment_datum_bank",
    "sixteen_position_witness_target_array",
    "bubble_clog_witness_wells",
    "volume_uniformity_sample_rack",
    "missed_dispense_wetness_lanes",
    "carryover_flush_challenge_lanes",
    "camera_illumination_fiducial_bridge",
    "sensor_sampling_manifold",
    "barcode_status_custody_lands",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 840.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_RECESS_Z: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.5;
const FIT_MARGIN: f64 = 16.0;
const MOUNT_BOSS_D: f64 = 30.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_SENSOR_COUNT: usize = 6;

const SLOT_ROWS: usize = 4;
const SLOT_COLS: usize = 4;
const SLOT_COUNT: usize = SLOT_ROWS * SLOT_COLS;
const TARGET_PITCH_X: f64 = 72.0;
const TARGET_PITCH_Y: f64 = 54.0;

const HEAD_POS: (f64, f64) = (-410.0, 235.0);
const HEAD_X: f64 = 360.0;
const HEAD_Y: f64 = 170.0;
const HEAD_Z: f64 = 56.0;
const HEAD_SOCKET_X: f64 = 282.0;
const HEAD_SOCKET_Y: f64 = 96.0;
const HEAD_SOCKET_DEPTH: f64 = 18.0;
const HEAD_DATUM_PINS: usize = 4;
const HEAD_LATCH_LANDS: usize = 4;

const DATUM_POS: (f64, f64) = (0.0, 235.0);
const DATUM_X: f64 = 350.0;
const DATUM_Y: f64 = 170.0;
const DATUM_Z: f64 = 38.0;
const DATUM_BORE_D: f64 = 7.2;
const DATUM_TARGET_D: f64 = 20.0;
const DATUM_CROSSHAIR_X: f64 = 36.0;
const DATUM_CROSSHAIR_Y: f64 = 4.0;
const ALIGNMENT_DATUM_COUNT: usize = SLOT_COUNT;

const TARGET_POS: (f64, f64) = (405.0, 235.0);
const TARGET_X: f64 = 350.0;
const TARGET_Y: f64 = 250.0;
const TARGET_Z: f64 = 30.0;
const TARGET_DISH_D: f64 = 28.0;
const TARGET_RING_D: f64 = 40.0;
const TARGET_DISH_DEPTH: f64 = 10.0;
const TARGET_BAR_X: f64 = 48.0;
const TARGET_BAR_Y: f64 = 5.0;

const BUBBLE_POS: (f64, f64) = (-410.0, 15.0);
const BUBBLE_X: f64 = 370.0;
const BUBBLE_Y: f64 = 170.0;
const BUBBLE_Z: f64 = 40.0;
const BUBBLE_WELL_D: f64 = 22.0;
const BUBBLE_WINDOW_X: f64 = 34.0;
const BUBBLE_WINDOW_Y: f64 = 12.0;
const CLOG_RESTRICTOR_D: f64 = 8.0;
const BUBBLE_WITNESS_WELLS: usize = SLOT_COUNT;
const CLOG_WITNESS_RESTRICTORS: usize = SLOT_COUNT;

const VOLUME_POS: (f64, f64) = (0.0, 15.0);
const VOLUME_X: f64 = 380.0;
const VOLUME_Y: f64 = 170.0;
const VOLUME_Z: f64 = 42.0;
const SAMPLE_CUP_D: f64 = 24.0;
const SAMPLE_CUP_DEPTH: f64 = 18.0;
const LOAD_CELL_PAD_X: f64 = 44.0;
const LOAD_CELL_PAD_Y: f64 = 28.0;
const CAPILLARY_STRIP_X: f64 = 46.0;
const CAPILLARY_STRIP_Y: f64 = 5.0;
const VOLUME_SAMPLE_CUPS: usize = SLOT_COUNT;
const LOAD_CELL_PLACEHOLDERS: usize = SLOT_COUNT;

const WETNESS_POS: (f64, f64) = (410.0, 15.0);
const WETNESS_X: f64 = 340.0;
const WETNESS_Y: f64 = 170.0;
const WETNESS_Z: f64 = 24.0;
const WETNESS_ROWS: usize = 2;
const WETNESS_COLS: usize = 8;
const WETNESS_STRIPS: usize = WETNESS_ROWS * WETNESS_COLS;
const WETNESS_STRIP_X: f64 = 28.0;
const WETNESS_STRIP_Y: f64 = 58.0;
const DRY_MISSED_DOT_D: f64 = 12.0;

const CARRYOVER_POS: (f64, f64) = (-410.0, -205.0);
const CARRYOVER_X: f64 = 370.0;
const CARRYOVER_Y: f64 = 160.0;
const CARRYOVER_Z: f64 = 34.0;
const CARRYOVER_LANES: usize = 4;
const CARRYOVER_POSITIONS_PER_LANE: usize = 4;
const CARRYOVER_CHALLENGE_POSITIONS: usize = CARRYOVER_LANES * CARRYOVER_POSITIONS_PER_LANE;
const CARRYOVER_SLOT_X: f64 = 50.0;
const CARRYOVER_SLOT_Y: f64 = 22.0;
const FLUSH_PORT_D: f64 = 7.0;
const WASTE_PORT_D: f64 = 9.0;

const SENSOR_POS: (f64, f64) = (0.0, -205.0);
const SENSOR_X: f64 = 380.0;
const SENSOR_Y: f64 = 160.0;
const SENSOR_Z: f64 = 40.0;
const SENSOR_SAMPLE_PORTS: usize = SLOT_COUNT;
const PRESSURE_TAPS: usize = 8;
const OPTICAL_WINDOWS: usize = 4;
const WETNESS_SENSOR_PUCKS: usize = 6;
const SENSOR_PORT_D: f64 = 8.0;
const PRESSURE_TAP_D: f64 = 4.2;

const STATUS_POS: (f64, f64) = (410.0, -205.0);
const STATUS_X: f64 = 340.0;
const STATUS_Y: f64 = 160.0;
const STATUS_Z: f64 = 16.0;
const STATUS_LANES: usize = 3;
const BARCODE_LANDS: usize = 8;
const RUN_RECORD_LANDS: usize = 4;

const TROUGH_POS: (f64, f64) = (0.0, -344.0);
const TROUGH_X: f64 = 940.0;
const TROUGH_Y: f64 = 58.0;
const TROUGH_Z: f64 = 54.0;
const TROUGH_DEPTH: f64 = 34.0;
const TROUGH_SEPTA: usize = 6;
const TROUGH_DRAIN_D: f64 = 14.0;

const BRIDGE_X: f64 = 1120.0;
const BRIDGE_Y: f64 = 58.0;
const BRIDGE_POST_X: f64 = 28.0;
const BRIDGE_POST_Y: f64 = 34.0;
const BRIDGE_BEAM_Z: f64 = 26.0;
const BRIDGE_CLEARANCE_Z: f64 = 186.0;
const CAMERA_PODS: usize = 4;
const LIGHT_BARS: usize = 4;
const FIDUCIAL_MARKERS: usize = 8;

const KEEP_OUT_ZONES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 350.0;
const REAR_SERVICE_CLEARANCE: f64 = 224.0;
const LEFT_HEAD_SERVICE_CLEARANCE: f64 = 190.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 180.0;
const KEEP_OUT_Z: f64 = 7.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - FIT_MARGIN;
        let usable_y = STATION_Y / 2.0 - RIM_W - FIT_MARGIN;

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
    assert_layout();

    let deck = isolator_deck();
    export(&deck, OUTPUTS[0]);

    let trough = sealed_waste_containment_trough();
    export(&trough, OUTPUTS[1]);

    let receiver = seeding_head_receiver_nest();
    export(&receiver, OUTPUTS[2]);

    let datums = nozzle_alignment_datum_bank();
    export(&datums, OUTPUTS[3]);

    let targets = sixteen_position_witness_target_array();
    export(&targets, OUTPUTS[4]);

    let bubbles = bubble_clog_witness_wells();
    export(&bubbles, OUTPUTS[5]);

    let volume = volume_uniformity_sample_rack();
    export(&volume, OUTPUTS[6]);

    let wetness = missed_dispense_wetness_lanes();
    export(&wetness, OUTPUTS[7]);

    let carryover = carryover_flush_challenge_lanes();
    export(&carryover, OUTPUTS[8]);

    let bridge = camera_illumination_fiducial_bridge();
    export(&bridge, OUTPUTS[9]);

    let sensors = sensor_sampling_manifold();
    export(&sensors, OUTPUTS[10]);

    let status = barcode_status_custody_lands();
    export(&status, OUTPUTS[11]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[12]);

    let assembly = deck
        + trough
        + receiver
        + datums
        + targets
        + bubbles
        + volume
        + wetness
        + carryover
        + bridge
        + sensors
        + status
        + keepouts
        + sealed_route_placeholders();
    export(&assembly, OUTPUTS[13]);

    println!();
    println!("Closed robotic seeding nozzle clog/uniformity validation station:");
    println!(
        "  Deck: {:.0}mm x {:.0}mm isolator tray with sealed {:.0}mm waste trough and {} leak sensor wells",
        STATION_X, STATION_Y, TROUGH_X, LEAK_SENSOR_COUNT
    );
    println!(
        "  Nozzle evidence: {} alignment datums, {} witness targets, {} bubble wells, {} clog restrictor witnesses",
        ALIGNMENT_DATUM_COUNT, SLOT_COUNT, BUBBLE_WITNESS_WELLS, CLOG_WITNESS_RESTRICTORS
    );
    println!(
        "  Uniformity checks: {} sample cups, {} load-cell placeholders, {} wetness/missed-dispense strips",
        VOLUME_SAMPLE_CUPS, LOAD_CELL_PLACEHOLDERS, WETNESS_STRIPS
    );
    println!(
        "  Carryover and sensors: {} carryover challenge positions, {} sample ports, {} pressure taps, {} optical windows, {} wetness pucks",
        CARRYOVER_CHALLENGE_POSITIONS,
        SENSOR_SAMPLE_PORTS,
        PRESSURE_TAPS,
        OPTICAL_WINDOWS,
        WETNESS_SENSOR_PUCKS
    );
    println!(
        "  Imaging: {} camera pods, {} light bars, {} fiducial markers, {:.0}mm bridge clearance over the closed deck",
        CAMERA_PODS, LIGHT_BARS, FIDUCIAL_MARKERS, BRIDGE_CLEARANCE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(SLOT_COUNT, 16, "witness map must stay 4 x 4");
    assert_eq!(ALIGNMENT_DATUM_COUNT, SLOT_COUNT);
    assert_eq!(BUBBLE_WITNESS_WELLS, SLOT_COUNT);
    assert_eq!(CLOG_WITNESS_RESTRICTORS, SLOT_COUNT);
    assert_eq!(VOLUME_SAMPLE_CUPS, SLOT_COUNT);
    assert_eq!(LOAD_CELL_PLACEHOLDERS, SLOT_COUNT);
    assert_eq!(WETNESS_STRIPS, SLOT_COUNT);
    assert_eq!(CARRYOVER_CHALLENGE_POSITIONS, SLOT_COUNT);
    assert_eq!(SENSOR_SAMPLE_PORTS, SLOT_COUNT);
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(OUTPUTS.iter().all(|path| path.contains(PREFIX)));
    assert_eq!(
        OUTPUTS
            .iter()
            .filter(|path| path.ends_with("_assembly.stl"))
            .count(),
        1
    );
    assert!(BRIDGE_CLEARANCE_Z > BASE_Z + HEAD_Z + 90.0);
    assert!(FRONT_ROBOT_CLEARANCE > REAR_SERVICE_CLEARANCE);
    assert!(LEFT_HEAD_SERVICE_CLEARANCE >= 180.0);
    assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 170.0);
    assert!(TROUGH_DEPTH < TROUGH_Z);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds isolator deck envelope",
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
}

fn layout_rects() -> [Rect; 10] {
    [
        rect("seeding_head_receiver_nest", HEAD_POS, HEAD_X, HEAD_Y),
        rect("nozzle_alignment_datum_bank", DATUM_POS, DATUM_X, DATUM_Y),
        rect(
            "sixteen_position_witness_target_array",
            TARGET_POS,
            TARGET_X,
            TARGET_Y,
        ),
        rect("bubble_clog_witness_wells", BUBBLE_POS, BUBBLE_X, BUBBLE_Y),
        rect(
            "volume_uniformity_sample_rack",
            VOLUME_POS,
            VOLUME_X,
            VOLUME_Y,
        ),
        rect(
            "missed_dispense_wetness_lanes",
            WETNESS_POS,
            WETNESS_X,
            WETNESS_Y,
        ),
        rect(
            "carryover_flush_challenge_lanes",
            CARRYOVER_POS,
            CARRYOVER_X,
            CARRYOVER_Y,
        ),
        rect("sensor_sampling_manifold", SENSOR_POS, SENSOR_X, SENSOR_Y),
        rect(
            "barcode_status_custody_lands",
            STATUS_POS,
            STATUS_X,
            STATUS_Y,
        ),
        rect(
            "sealed_waste_containment_trough",
            TROUGH_POS,
            TROUGH_X,
            TROUGH_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn isolator_deck() -> Part {
    let deck = centered_cube(
        name("isolator_closed_validation_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        name("shallow_leak_capture_basin_recess"),
        STATION_X - 112.0,
        STATION_Y - 110.0,
        BASIN_RECESS_Z + 0.5,
    )
    .translate(0.0, 2.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.3);

    deck - basin - component_socket_recesses()
        + containment_rims()
        + deck_mount_bosses()
        + deck_zone_dividers()
        + isolator_location_fiducials()
        + leak_sensor_wells()
}

fn component_socket_recesses() -> Part {
    let mut sockets = Part::empty(name("component_socket_recesses"));
    for rect in layout_rects().iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket_recess", rect.name),
                rect.x + 8.0,
                rect.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn containment_rims() -> Part {
    let front = centered_cube(name("front_containment_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        z_on_base(RIM_Z),
    );
    let rear = centered_cube(name("rear_containment_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        z_on_base(RIM_Z),
    );
    let left = centered_cube(name("left_containment_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        z_on_base(RIM_Z),
    );
    let right = centered_cube(name("right_containment_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        z_on_base(RIM_Z),
    );

    front + rear + left + right
}

fn deck_mount_bosses() -> Part {
    let mut bosses = Part::empty(name("deck_mount_bosses"));
    for (index, (x, y)) in mount_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{PREFIX}_mount_boss_{index}"),
            MOUNT_BOSS_D / 2.0,
            9.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        let hole = centered_cylinder(
            format!("{PREFIX}_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            11.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn deck_zone_dividers() -> Part {
    let head_to_witness = centered_cube(
        name("head_receiver_to_witness_zone_divider"),
        10.0,
        206.0,
        24.0,
    )
    .translate(-205.0, 235.0, z_on_base(24.0));
    let target_to_wet = centered_cube(
        name("target_array_to_wetness_zone_divider"),
        10.0,
        380.0,
        24.0,
    )
    .translate(212.0, 118.0, z_on_base(24.0));
    let row_separator = centered_cube(
        name("upper_lower_validation_row_separator"),
        STATION_X - 150.0,
        8.0,
        22.0,
    )
    .translate(0.0, 112.0, z_on_base(22.0));
    let trough_lip = centered_cube(
        name("waste_trough_keepaway_lip"),
        TROUGH_X + 80.0,
        8.0,
        26.0,
    )
    .translate(0.0, -305.0, z_on_base(26.0));

    head_to_witness + target_to_wet + row_separator + trough_lip
}

fn isolator_location_fiducials() -> Part {
    let mut fiducials = Part::empty(name("isolator_location_fiducials"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 74.0, -STATION_Y / 2.0 + 74.0),
        (STATION_X / 2.0 - 74.0, -STATION_Y / 2.0 + 74.0),
        (-STATION_X / 2.0 + 74.0, STATION_Y / 2.0 - 74.0),
        (STATION_X / 2.0 - 74.0, STATION_Y / 2.0 - 74.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("{PREFIX}_isolator_fiducial_disc_{index}"),
            13.0,
            4.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        let bore = centered_cylinder(
            format!("{PREFIX}_isolator_fiducial_center_bore_{index}"),
            3.0,
            5.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        fiducials = fiducials + (disc - bore);
    }
    fiducials
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty(name("deck_leak_sensor_wells"));
    for index in 0..LEAK_SENSOR_COUNT {
        let x = centered_index(index, LEAK_SENSOR_COUNT, 146.0);
        let puck = centered_cylinder(format!("{PREFIX}_leak_sensor_puck_{index}"), 13.0, 5.0, 32)
            .translate(x, -STATION_Y / 2.0 + 70.0, BASE_Z + 2.5);
        let wet_well = centered_cylinder(
            format!("{PREFIX}_leak_sensor_wetness_well_{index}"),
            5.4,
            6.0,
            24,
        )
        .translate(x, -STATION_Y / 2.0 + 70.0, BASE_Z + 2.5);
        wells = wells + (puck - wet_well);
    }
    wells
}

fn sealed_waste_containment_trough() -> Part {
    let body = centered_cube(
        name("sealed_waste_containment_trough_body"),
        TROUGH_X,
        TROUGH_Y,
        TROUGH_Z,
    )
    .translate(TROUGH_POS.0, TROUGH_POS.1, z_on_base(TROUGH_Z));
    let cavity = centered_cube(
        name("sealed_waste_containment_trough_cavity"),
        TROUGH_X - 74.0,
        TROUGH_Y - 20.0,
        TROUGH_DEPTH + 1.0,
    )
    .translate(
        TROUGH_POS.0,
        TROUGH_POS.1,
        BASE_Z + TROUGH_Z - TROUGH_DEPTH / 2.0 + 0.5,
    );
    let drain = centered_cylinder(
        name("sealed_waste_trough_front_drain_bore"),
        TROUGH_DRAIN_D / 2.0,
        TROUGH_Y + 10.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        TROUGH_POS.0 + TROUGH_X / 2.0 - 80.0,
        TROUGH_POS.1,
        BASE_Z + 18.0,
    );
    let latch_lip = centered_cube(
        name("sealed_waste_trough_removable_lid_lip"),
        TROUGH_X - 42.0,
        8.0,
        10.0,
    )
    .translate(
        TROUGH_POS.0,
        TROUGH_POS.1 + TROUGH_Y / 2.0 - 8.0,
        BASE_Z + TROUGH_Z + 5.0,
    );

    body - cavity - drain + latch_lip + trough_sampling_septa()
}

fn trough_sampling_septa() -> Part {
    let mut septa = Part::empty(name("sealed_waste_trough_sampling_septa"));
    for index in 0..TROUGH_SEPTA {
        let x = TROUGH_POS.0 + centered_index(index, TROUGH_SEPTA, 130.0);
        let pad = centered_cylinder(
            format!("{PREFIX}_waste_trough_sampling_septum_pad_{index}"),
            14.0,
            6.0,
            32,
        )
        .translate(x, TROUGH_POS.1 - 4.0, BASE_Z + TROUGH_Z + 3.0);
        let bore = centered_cylinder(
            format!("{PREFIX}_waste_trough_sampling_septum_bore_{index}"),
            4.0,
            7.0,
            24,
        )
        .translate(x, TROUGH_POS.1 - 4.0, BASE_Z + TROUGH_Z + 3.0);
        septa = septa + (pad - bore);
    }
    septa
}

fn seeding_head_receiver_nest() -> Part {
    let base = plate_at(
        "seeding_head_receiver_nest_base",
        HEAD_POS,
        HEAD_X,
        HEAD_Y,
        HEAD_Z,
    );
    let socket = centered_cube(
        name("seeding_head_receiver_socket_clearance"),
        HEAD_SOCKET_X,
        HEAD_SOCKET_Y,
        HEAD_SOCKET_DEPTH + 0.6,
    )
    .translate(
        HEAD_POS.0,
        HEAD_POS.1,
        BASE_Z + HEAD_Z - HEAD_SOCKET_DEPTH / 2.0 + 0.4,
    );
    let nozzle_window = centered_cube(
        name("receiver_front_nozzle_approach_window"),
        HEAD_SOCKET_X - 34.0,
        20.0,
        HEAD_Z + 3.0,
    )
    .translate(
        HEAD_POS.0,
        HEAD_POS.1 - HEAD_Y / 2.0 + 12.0,
        z_on_base(HEAD_Z),
    );

    base - socket - nozzle_window + receiver_rails() + receiver_datum_pins() + receiver_latches()
}

fn receiver_rails() -> Part {
    let left = centered_cube(name("receiver_left_hard_rail"), 14.0, HEAD_Y - 24.0, 34.0).translate(
        HEAD_POS.0 - HEAD_X / 2.0 + 28.0,
        HEAD_POS.1,
        z_on_base(34.0),
    );
    let rear = centered_cube(
        name("receiver_rear_hard_stop_rail"),
        HEAD_X - 52.0,
        14.0,
        34.0,
    )
    .translate(
        HEAD_POS.0,
        HEAD_POS.1 + HEAD_Y / 2.0 - 28.0,
        z_on_base(34.0),
    );
    let right_soft = centered_cube(
        name("receiver_right_soft_capture_rail"),
        12.0,
        HEAD_Y - 70.0,
        22.0,
    )
    .translate(
        HEAD_POS.0 + HEAD_X / 2.0 - 28.0,
        HEAD_POS.1 - 10.0,
        z_on_base(22.0),
    );
    let front_soft = centered_cube(
        name("receiver_front_soft_capture_rail"),
        HEAD_X - 120.0,
        12.0,
        20.0,
    )
    .translate(
        HEAD_POS.0 + 20.0,
        HEAD_POS.1 - HEAD_Y / 2.0 + 28.0,
        z_on_base(20.0),
    );

    left + rear + right_soft + front_soft
}

fn receiver_datum_pins() -> Part {
    let mut pins = Part::empty(name("receiver_datum_pins"));
    for (index, (x, y)) in [
        (-126.0, 54.0),
        (126.0, 54.0),
        (-126.0, -42.0),
        (126.0, -42.0),
    ]
    .iter()
    .enumerate()
    {
        let pin = centered_cylinder(
            format!("{PREFIX}_receiver_head_datum_pin_{index}"),
            5.0,
            14.0,
            28,
        )
        .translate(HEAD_POS.0 + *x, HEAD_POS.1 + *y, BASE_Z + HEAD_Z + 7.0);
        let base = centered_cylinder(
            format!("{PREFIX}_receiver_head_datum_pin_boss_{index}"),
            11.0,
            4.0,
            32,
        )
        .translate(HEAD_POS.0 + *x, HEAD_POS.1 + *y, BASE_Z + HEAD_Z + 2.0);
        pins = pins + base + pin;
    }
    pins
}

fn receiver_latches() -> Part {
    let mut latches = Part::empty(name("receiver_latch_and_clocking_lands"));
    for index in 0..HEAD_LATCH_LANDS {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let y = HEAD_POS.1 + if index < 2 { -54.0 } else { 54.0 };
        let x = HEAD_POS.0 + side * (HEAD_X / 2.0 - 48.0);
        latches = latches
            + centered_cube(
                format!("{PREFIX}_receiver_latch_land_{index}"),
                44.0,
                12.0,
                10.0,
            )
            .translate(x, y, BASE_Z + HEAD_Z + 5.0)
            + centered_cube(
                format!("{PREFIX}_receiver_latch_flag_{index}"),
                9.0,
                34.0,
                24.0,
            )
            .translate(x, y, BASE_Z + HEAD_Z + 12.0);
    }
    assert_eq!(HEAD_DATUM_PINS, 4);
    latches
}

fn nozzle_alignment_datum_bank() -> Part {
    let plate = plate_at(
        "nozzle_alignment_datum_bank_plate",
        DATUM_POS,
        DATUM_X,
        DATUM_Y,
        DATUM_Z,
    );
    let mut cuts = Part::empty(name("nozzle_alignment_datum_bore_cutters"));
    let mut features = Part::empty(name("nozzle_alignment_datum_crosshair_features"));

    for row in 0..SLOT_ROWS {
        for col in 0..SLOT_COLS {
            let slot = row * SLOT_COLS + col;
            let (x, y) = grid_xy(
                DATUM_POS,
                row,
                SLOT_ROWS,
                col,
                SLOT_COLS,
                TARGET_PITCH_X,
                38.0,
            );
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_alignment_nozzle_bore_{slot:02}"),
                    DATUM_BORE_D / 2.0,
                    DATUM_Z + 5.0,
                    28,
                )
                .translate(x, y, z_on_base(DATUM_Z));
            let ring = centered_cylinder(
                format!("{PREFIX}_alignment_target_ring_{slot:02}"),
                DATUM_TARGET_D / 2.0,
                4.0,
                36,
            )
            .translate(x, y, BASE_Z + DATUM_Z + 2.0);
            let ring_void = centered_cylinder(
                format!("{PREFIX}_alignment_target_ring_void_{slot:02}"),
                DATUM_BORE_D / 2.0 + 1.0,
                5.0,
                28,
            )
            .translate(x, y, BASE_Z + DATUM_Z + 2.0);
            let xhair = centered_cube(
                format!("{PREFIX}_alignment_slot_{slot:02}_x_crosshair"),
                DATUM_CROSSHAIR_X,
                DATUM_CROSSHAIR_Y,
                3.0,
            )
            .translate(x, y, BASE_Z + DATUM_Z + 1.5);
            let yhair = centered_cube(
                format!("{PREFIX}_alignment_slot_{slot:02}_y_crosshair"),
                DATUM_CROSSHAIR_Y,
                DATUM_CROSSHAIR_X,
                3.0,
            )
            .translate(x, y, BASE_Z + DATUM_Z + 1.5);
            features = features + (ring - ring_void) + xhair + yhair;
        }
    }

    plate - cuts + features + datum_bank_hard_stops()
}

fn datum_bank_hard_stops() -> Part {
    let left = centered_cube(
        name("alignment_bank_left_hard_stop"),
        12.0,
        DATUM_Y - 24.0,
        24.0,
    )
    .translate(
        DATUM_POS.0 - DATUM_X / 2.0 + 20.0,
        DATUM_POS.1,
        z_on_base(24.0),
    );
    let rear = centered_cube(
        name("alignment_bank_rear_hard_stop"),
        DATUM_X - 42.0,
        10.0,
        24.0,
    )
    .translate(
        DATUM_POS.0,
        DATUM_POS.1 + DATUM_Y / 2.0 - 20.0,
        z_on_base(24.0),
    );
    let clocking_notch = centered_cube(
        name("alignment_bank_clocking_notch_gauge"),
        36.0,
        14.0,
        20.0,
    )
    .translate(
        DATUM_POS.0 + DATUM_X / 2.0 - 50.0,
        DATUM_POS.1 - 58.0,
        z_on_base(20.0),
    );

    left + rear + clocking_notch
}

fn sixteen_position_witness_target_array() -> Part {
    let plate = plate_at(
        "sixteen_position_witness_target_array_plate",
        TARGET_POS,
        TARGET_X,
        TARGET_Y,
        TARGET_Z,
    );
    let mut dishes = Part::empty(name("witness_target_dish_cutters"));
    let mut targets = Part::empty(name("witness_target_rings_and_crosshairs"));

    for row in 0..SLOT_ROWS {
        for col in 0..SLOT_COLS {
            let slot = row * SLOT_COLS + col;
            let (x, y) = grid_xy(
                TARGET_POS,
                row,
                SLOT_ROWS,
                col,
                SLOT_COLS,
                TARGET_PITCH_X,
                TARGET_PITCH_Y,
            );
            dishes = dishes
                + centered_cylinder(
                    format!("{PREFIX}_witness_target_dish_{slot:02}"),
                    TARGET_DISH_D / 2.0,
                    TARGET_DISH_DEPTH + 0.6,
                    40,
                )
                .translate(x, y, BASE_Z + TARGET_Z - TARGET_DISH_DEPTH / 2.0 + 0.3);
            let ring = centered_cylinder(
                format!("{PREFIX}_witness_target_outer_ring_{slot:02}"),
                TARGET_RING_D / 2.0,
                4.0,
                48,
            )
            .translate(x, y, BASE_Z + TARGET_Z + 2.0);
            let ring_void = centered_cylinder(
                format!("{PREFIX}_witness_target_inner_void_{slot:02}"),
                TARGET_DISH_D / 2.0 + 1.6,
                5.0,
                40,
            )
            .translate(x, y, BASE_Z + TARGET_Z + 2.0);
            let xbar = centered_cube(
                format!("{PREFIX}_witness_target_{slot:02}_x_alignment_bar"),
                TARGET_BAR_X,
                TARGET_BAR_Y,
                3.0,
            )
            .translate(x, y, BASE_Z + TARGET_Z + 1.5);
            let ybar = centered_cube(
                format!("{PREFIX}_witness_target_{slot:02}_y_alignment_bar"),
                TARGET_BAR_Y,
                TARGET_BAR_X,
                3.0,
            )
            .translate(x, y, BASE_Z + TARGET_Z + 1.5);
            let runner = centered_cube(
                format!("{PREFIX}_witness_target_{slot:02}_sealed_waste_runner"),
                6.0,
                24.0,
                4.0,
            )
            .translate(x, y - TARGET_DISH_D / 2.0 - 12.0, BASE_Z + TARGET_Z - 2.0);
            targets = targets + (ring - ring_void) + xbar + ybar + runner;
        }
    }

    plate - dishes + targets + target_array_edge_fiducials()
}

fn target_array_edge_fiducials() -> Part {
    let mut fiducials = Part::empty(name("target_array_edge_fiducials"));
    for (index, (dx, dy)) in [
        (-TARGET_X / 2.0 + 28.0, -TARGET_Y / 2.0 + 24.0),
        (TARGET_X / 2.0 - 28.0, -TARGET_Y / 2.0 + 24.0),
        (-TARGET_X / 2.0 + 28.0, TARGET_Y / 2.0 - 24.0),
        (TARGET_X / 2.0 - 28.0, TARGET_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("{PREFIX}_target_array_camera_fiducial_{index}"),
            10.0,
            4.0,
            32,
        )
        .translate(
            TARGET_POS.0 + *dx,
            TARGET_POS.1 + *dy,
            BASE_Z + TARGET_Z + 2.0,
        );
        let center = centered_cylinder(
            format!("{PREFIX}_target_array_fiducial_center_{index}"),
            2.8,
            5.0,
            20,
        )
        .translate(
            TARGET_POS.0 + *dx,
            TARGET_POS.1 + *dy,
            BASE_Z + TARGET_Z + 2.0,
        );
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn bubble_clog_witness_wells() -> Part {
    let plate = plate_at(
        "bubble_clog_witness_wells_plate",
        BUBBLE_POS,
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );
    let mut cuts = Part::empty(name("bubble_clog_witness_well_cutters"));
    let mut features = Part::empty(name("bubble_clog_witness_well_features"));

    for row in 0..SLOT_ROWS {
        for col in 0..SLOT_COLS {
            let slot = row * SLOT_COLS + col;
            let (x, y) = grid_xy(BUBBLE_POS, row, SLOT_ROWS, col, SLOT_COLS, 72.0, 36.0);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_bubble_witness_well_{slot:02}"),
                    BUBBLE_WELL_D / 2.0,
                    16.0,
                    36,
                )
                .translate(x - 14.0, y, BASE_Z + BUBBLE_Z - 7.5)
                + centered_cylinder(
                    format!("{PREFIX}_clog_restrictor_reference_bore_{slot:02}"),
                    CLOG_RESTRICTOR_D / 2.0,
                    BUBBLE_Z + 5.0,
                    24,
                )
                .translate(x + 17.0, y, z_on_base(BUBBLE_Z));
            let window = centered_cube(
                format!("{PREFIX}_bubble_window_flat_{slot:02}"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y,
                4.0,
            )
            .translate(x - 14.0, y + 19.0, BASE_Z + BUBBLE_Z + 2.0);
            let pressure_flag = centered_cube(
                format!("{PREFIX}_clog_backpressure_flag_land_{slot:02}"),
                22.0,
                10.0,
                6.0,
            )
            .translate(x + 17.0, y - 18.0, BASE_Z + BUBBLE_Z + 3.0);
            features = features + window + pressure_flag;
        }
    }

    plate - cuts + features + bubble_clog_reference_rail()
}

fn bubble_clog_reference_rail() -> Part {
    let rail = centered_cube(
        name("bubble_clog_clear_blocked_reference_rail"),
        BUBBLE_X - 50.0,
        12.0,
        16.0,
    )
    .translate(
        BUBBLE_POS.0,
        BUBBLE_POS.1 - BUBBLE_Y / 2.0 + 22.0,
        z_on_base(16.0),
    );
    let blocked = centered_cube(
        name("bubble_clog_blocked_nozzle_comparator_land"),
        92.0,
        18.0,
        5.0,
    )
    .translate(
        BUBBLE_POS.0 - 96.0,
        BUBBLE_POS.1 - BUBBLE_Y / 2.0 + 42.0,
        BASE_Z + BUBBLE_Z + 2.5,
    );
    let clear = centered_cube(
        name("bubble_clog_clear_nozzle_comparator_land"),
        92.0,
        18.0,
        5.0,
    )
    .translate(
        BUBBLE_POS.0 + 96.0,
        BUBBLE_POS.1 - BUBBLE_Y / 2.0 + 42.0,
        BASE_Z + BUBBLE_Z + 2.5,
    );

    rail + blocked + clear
}

fn volume_uniformity_sample_rack() -> Part {
    let plate = plate_at(
        "volume_uniformity_sample_rack_body",
        VOLUME_POS,
        VOLUME_X,
        VOLUME_Y,
        VOLUME_Z,
    );
    let mut cup_cuts = Part::empty(name("volume_uniformity_sample_cup_cutters"));
    let mut features = Part::empty(name("volume_uniformity_sample_features"));

    for row in 0..SLOT_ROWS {
        for col in 0..SLOT_COLS {
            let slot = row * SLOT_COLS + col;
            let (x, y) = grid_xy(VOLUME_POS, row, SLOT_ROWS, col, SLOT_COLS, 72.0, 36.0);
            cup_cuts = cup_cuts
                + centered_cylinder(
                    format!("{PREFIX}_volume_sample_cup_{slot:02}"),
                    SAMPLE_CUP_D / 2.0,
                    SAMPLE_CUP_DEPTH + 0.5,
                    36,
                )
                .translate(
                    x - 12.0,
                    y,
                    BASE_Z + VOLUME_Z - SAMPLE_CUP_DEPTH / 2.0 + 0.2,
                );
            let load_pad = centered_cube(
                format!("{PREFIX}_load_cell_pad_placeholder_{slot:02}"),
                LOAD_CELL_PAD_X,
                LOAD_CELL_PAD_Y,
                5.0,
            )
            .translate(x + 16.0, y, BASE_Z + VOLUME_Z + 2.5);
            let strip = centered_cube(
                format!("{PREFIX}_capillary_uniformity_strip_{slot:02}"),
                CAPILLARY_STRIP_X,
                CAPILLARY_STRIP_Y,
                3.0,
            )
            .translate(x, y - 20.0, BASE_Z + VOLUME_Z + 1.5);
            features = features + load_pad + strip;
        }
    }

    plate - cup_cuts + features + volume_standard_slots()
}

fn volume_standard_slots() -> Part {
    let high = centered_cube(name("volume_high_standard_lane"), 118.0, 18.0, 6.0).translate(
        VOLUME_POS.0 - 82.0,
        VOLUME_POS.1 - VOLUME_Y / 2.0 + 18.0,
        BASE_Z + VOLUME_Z + 3.0,
    );
    let low = centered_cube(name("volume_low_standard_lane"), 118.0, 18.0, 6.0).translate(
        VOLUME_POS.0 + 82.0,
        VOLUME_POS.1 - VOLUME_Y / 2.0 + 18.0,
        BASE_Z + VOLUME_Z + 3.0,
    );
    let reference = centered_cube(name("volume_blank_reference_lane"), 72.0, 14.0, 5.0).translate(
        VOLUME_POS.0,
        VOLUME_POS.1 + VOLUME_Y / 2.0 - 20.0,
        BASE_Z + VOLUME_Z + 2.5,
    );

    high + low + reference
}

fn missed_dispense_wetness_lanes() -> Part {
    let plate = plate_at(
        "missed_dispense_wetness_lane_plate",
        WETNESS_POS,
        WETNESS_X,
        WETNESS_Y,
        WETNESS_Z,
    );
    let mut features = Part::empty(name("missed_dispense_wetness_lane_features"));
    let mut dry_dots = Part::empty(name("missed_dispense_dry_dot_recesses"));

    for row in 0..WETNESS_ROWS {
        for col in 0..WETNESS_COLS {
            let slot = row * WETNESS_COLS + col;
            let (x, y) = grid_xy(
                WETNESS_POS,
                row,
                WETNESS_ROWS,
                col,
                WETNESS_COLS,
                36.0,
                70.0,
            );
            let strip = centered_cube(
                format!("{PREFIX}_wetness_strip_lane_{slot:02}"),
                WETNESS_STRIP_X,
                WETNESS_STRIP_Y,
                4.0,
            )
            .translate(x, y, BASE_Z + WETNESS_Z + 2.0);
            let missed_flag = centered_cube(
                format!("{PREFIX}_missed_dispense_flag_land_{slot:02}"),
                WETNESS_STRIP_X,
                8.0,
                5.0,
            )
            .translate(x, y - 34.0, BASE_Z + WETNESS_Z + 2.5);
            dry_dots = dry_dots
                + centered_cylinder(
                    format!("{PREFIX}_dry_control_dot_recess_{slot:02}"),
                    DRY_MISSED_DOT_D / 2.0,
                    5.0,
                    24,
                )
                .translate(x, y + 34.0, BASE_Z + WETNESS_Z - 1.5);
            features = features + strip + missed_flag;
        }
    }

    plate - dry_dots + features + wetness_common_electrode_lands()
}

fn wetness_common_electrode_lands() -> Part {
    let upper = centered_cube(
        name("wetness_common_electrode_upper_bus_land"),
        WETNESS_X - 54.0,
        8.0,
        4.0,
    )
    .translate(
        WETNESS_POS.0,
        WETNESS_POS.1 + WETNESS_Y / 2.0 - 18.0,
        BASE_Z + WETNESS_Z + 2.0,
    );
    let lower = centered_cube(
        name("wetness_common_electrode_lower_bus_land"),
        WETNESS_X - 54.0,
        8.0,
        4.0,
    )
    .translate(
        WETNESS_POS.0,
        WETNESS_POS.1 - WETNESS_Y / 2.0 + 18.0,
        BASE_Z + WETNESS_Z + 2.0,
    );

    upper + lower
}

fn carryover_flush_challenge_lanes() -> Part {
    let plate = plate_at(
        "carryover_flush_challenge_lane_body",
        CARRYOVER_POS,
        CARRYOVER_X,
        CARRYOVER_Y,
        CARRYOVER_Z,
    );
    let mut cuts = Part::empty(name("carryover_flush_port_cutters"));
    let mut features = Part::empty(name("carryover_flush_challenge_features"));

    for lane in 0..CARRYOVER_LANES {
        let y = CARRYOVER_POS.1 + centered_index(lane, CARRYOVER_LANES, 34.0);
        let lane_rail = centered_cube(
            format!("{PREFIX}_carryover_lane_{lane}_segregation_rail"),
            CARRYOVER_X - 44.0,
            6.0,
            8.0,
        )
        .translate(CARRYOVER_POS.0, y + 16.0, BASE_Z + CARRYOVER_Z + 4.0);
        features = features + lane_rail;

        for position in 0..CARRYOVER_POSITIONS_PER_LANE {
            let slot = lane * CARRYOVER_POSITIONS_PER_LANE + position;
            let x = CARRYOVER_POS.0 + centered_index(position, CARRYOVER_POSITIONS_PER_LANE, 76.0);
            let high_low_tag = if (lane + position) % 2 == 0 {
                "high"
            } else {
                "low"
            };
            let pocket = centered_cube(
                format!("{PREFIX}_carryover_{high_low_tag}_challenge_slot_{slot:02}"),
                CARRYOVER_SLOT_X,
                CARRYOVER_SLOT_Y,
                8.0,
            )
            .translate(x, y, BASE_Z + CARRYOVER_Z - 3.5);
            let witness_land = centered_cube(
                format!("{PREFIX}_carryover_blank_witness_land_{slot:02}"),
                36.0,
                12.0,
                4.0,
            )
            .translate(x, y - 16.0, BASE_Z + CARRYOVER_Z + 2.0);
            cuts = cuts + pocket;
            features = features + witness_land;
        }

        let flush_x = CARRYOVER_POS.0 - CARRYOVER_X / 2.0 + 28.0;
        let waste_x = CARRYOVER_POS.0 + CARRYOVER_X / 2.0 - 28.0;
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_carryover_lane_{lane}_flush_port"),
                FLUSH_PORT_D / 2.0,
                CARRYOVER_Z + 4.0,
                24,
            )
            .translate(flush_x, y, z_on_base(CARRYOVER_Z))
            + centered_cylinder(
                format!("{PREFIX}_carryover_lane_{lane}_waste_port"),
                WASTE_PORT_D / 2.0,
                CARRYOVER_Z + 4.0,
                24,
            )
            .translate(waste_x, y, z_on_base(CARRYOVER_Z));
    }

    plate - cuts + features
}

fn camera_illumination_fiducial_bridge() -> Part {
    let z_beam = BASE_Z + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0;
    let rear_beam = centered_cube(
        name("camera_bridge_rear_beam"),
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 176.0, z_beam);
    let front_beam = centered_cube(
        name("camera_bridge_front_beam"),
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, -38.0, z_beam);
    let left_post = bridge_post(-BRIDGE_X / 2.0 + 48.0, 176.0, 0);
    let right_post = bridge_post(BRIDGE_X / 2.0 - 48.0, 176.0, 1);
    let left_front_post = bridge_post(-BRIDGE_X / 2.0 + 48.0, -38.0, 2);
    let right_front_post = bridge_post(BRIDGE_X / 2.0 - 48.0, -38.0, 3);

    rear_beam
        + front_beam
        + left_post
        + right_post
        + left_front_post
        + right_front_post
        + camera_pods()
        + illumination_bars()
        + bridge_reference_fiducials()
}

fn bridge_post(x: f64, y: f64, index: usize) -> Part {
    centered_cube(
        format!("{PREFIX}_camera_bridge_post_{index}"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(x, y, BASE_Z + BRIDGE_CLEARANCE_Z / 2.0)
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(name("camera_pods_over_witness_zones"));
    for index in 0..CAMERA_PODS {
        let x = centered_index(index, CAMERA_PODS, 250.0);
        let pod = centered_cube(
            format!("{PREFIX}_evidence_camera_pod_{index}"),
            58.0,
            40.0,
            28.0,
        )
        .translate(x, 70.0, BASE_Z + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 14.0);
        let lens = centered_cylinder(
            format!("{PREFIX}_evidence_camera_lens_bore_{index}"),
            9.0,
            30.0,
            32,
        )
        .translate(x, 70.0, BASE_Z + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 1.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn illumination_bars() -> Part {
    let mut bars = Part::empty(name("illumination_reference_bars"));
    for index in 0..LIGHT_BARS {
        let x = centered_index(index, LIGHT_BARS, 260.0);
        bars = bars
            + centered_cube(
                format!("{PREFIX}_diffuse_led_bar_{index}"),
                170.0,
                12.0,
                12.0,
            )
            .translate(x, 132.0, BASE_Z + BRIDGE_CLEARANCE_Z + 10.0)
            + centered_cube(
                format!("{PREFIX}_illumination_gray_reference_land_{index}"),
                48.0,
                12.0,
                4.0,
            )
            .translate(x, 112.0, BASE_Z + 4.0);
    }
    bars
}

fn bridge_reference_fiducials() -> Part {
    let mut fiducials = Part::empty(name("camera_illumination_reference_fiducials"));
    for index in 0..FIDUCIAL_MARKERS {
        let x = centered_index(index % 4, 4, 250.0);
        let y = if index < 4 { 318.0 } else { -70.0 };
        let disc = centered_cylinder(
            format!("{PREFIX}_camera_reference_fiducial_disc_{index}"),
            12.0,
            4.0,
            32,
        )
        .translate(x, y, BASE_Z + 2.0);
        let center = centered_cylinder(
            format!("{PREFIX}_camera_reference_fiducial_center_{index}"),
            3.0,
            5.0,
            20,
        )
        .translate(x, y, BASE_Z + 2.0);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn sensor_sampling_manifold() -> Part {
    let body = plate_at(
        "sensor_sampling_manifold_body",
        SENSOR_POS,
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let mut cuts = Part::empty(name("sensor_sampling_manifold_cutters"));
    let mut features = Part::empty(name("sensor_sampling_manifold_features"));

    for row in 0..2 {
        for col in 0..8 {
            let port = row * 8 + col;
            let (x, y) = grid_xy(SENSOR_POS, row, 2, col, 8, 39.0, 54.0);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_sensor_sample_port_{port:02}"),
                    SENSOR_PORT_D / 2.0,
                    SENSOR_Z + 4.0,
                    24,
                )
                .translate(x, y, z_on_base(SENSOR_Z));
            features = features
                + centered_cube(
                    format!("{PREFIX}_sensor_sample_port_label_land_{port:02}"),
                    28.0,
                    8.0,
                    3.0,
                )
                .translate(x, y - 16.0, BASE_Z + SENSOR_Z + 1.5);
        }
    }

    for tap in 0..PRESSURE_TAPS {
        let x = SENSOR_POS.0 + centered_index(tap, PRESSURE_TAPS, 42.0);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_pressure_tap_bore_{tap}"),
                PRESSURE_TAP_D / 2.0,
                SENSOR_Z + 5.0,
                20,
            )
            .translate(x, SENSOR_POS.1 + SENSOR_Y / 2.0 - 25.0, z_on_base(SENSOR_Z));
        features = features
            + centered_cube(
                format!("{PREFIX}_pressure_tap_transducer_pad_{tap}"),
                28.0,
                14.0,
                4.0,
            )
            .translate(
                x,
                SENSOR_POS.1 + SENSOR_Y / 2.0 - 42.0,
                BASE_Z + SENSOR_Z + 2.0,
            );
    }

    body - cuts + features + optical_windows() + wetness_sensor_pucks()
}

fn optical_windows() -> Part {
    let mut windows = Part::empty(name("sensor_optical_window_features"));
    for index in 0..OPTICAL_WINDOWS {
        let x = SENSOR_POS.0 + centered_index(index, OPTICAL_WINDOWS, 76.0);
        windows = windows
            + centered_cube(
                format!("{PREFIX}_inline_optical_window_clearance_{index}"),
                54.0,
                20.0,
                4.0,
            )
            .translate(
                x,
                SENSOR_POS.1 - SENSOR_Y / 2.0 + 22.0,
                BASE_Z + SENSOR_Z + 2.0,
            );
    }
    windows
}

fn wetness_sensor_pucks() -> Part {
    let mut pucks = Part::empty(name("sensor_manifold_wetness_pucks"));
    for index in 0..WETNESS_SENSOR_PUCKS {
        let x = SENSOR_POS.0 + centered_index(index, WETNESS_SENSOR_PUCKS, 54.0);
        let puck = centered_cylinder(
            format!("{PREFIX}_wetness_sensor_puck_{index}"),
            10.0,
            5.0,
            28,
        )
        .translate(x, SENSOR_POS.1, BASE_Z + SENSOR_Z + 2.5);
        let dot = centered_cylinder(
            format!("{PREFIX}_wetness_sensor_probe_dot_{index}"),
            3.0,
            6.0,
            20,
        )
        .translate(x, SENSOR_POS.1, BASE_Z + SENSOR_Z + 2.5);
        pucks = pucks + (puck - dot);
    }
    pucks
}

fn barcode_status_custody_lands() -> Part {
    let board = plate_at(
        "barcode_status_custody_land_board",
        STATUS_POS,
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut features = Part::empty(name("barcode_status_custody_features"));

    for lane in 0..STATUS_LANES {
        let x = STATUS_POS.0 + centered_index(lane, STATUS_LANES, 98.0);
        let label = match lane {
            0 => "pass",
            1 => "hold",
            _ => "reject",
        };
        features = features
            + centered_cube(
                format!("{PREFIX}_{label}_status_lane_recess_visual_{lane}"),
                72.0,
                76.0,
                4.0,
            )
            .translate(x, STATUS_POS.1 + 20.0, BASE_Z + STATUS_Z + 2.0)
            + centered_cube(
                format!("{PREFIX}_{label}_status_lane_result_tab_{lane}"),
                68.0,
                16.0,
                5.0,
            )
            .translate(x, STATUS_POS.1 + 64.0, BASE_Z + STATUS_Z + 2.5);
    }

    for index in 0..BARCODE_LANDS {
        let x = STATUS_POS.0 + centered_index(index % 4, 4, 70.0);
        let y = STATUS_POS.1 - 48.0 + (index / 4) as f64 * 22.0;
        features =
            features
                + centered_cube(format!("{PREFIX}_barcode_land_{index}"), 52.0, 13.0, 3.0)
                    .translate(x, y, BASE_Z + STATUS_Z + 1.5);
    }

    for index in 0..RUN_RECORD_LANDS {
        let x = STATUS_POS.0 + centered_index(index, RUN_RECORD_LANDS, 58.0);
        features = features
            + centered_cube(
                format!("{PREFIX}_run_record_certificate_land_{index}"),
                44.0,
                18.0,
                3.0,
            )
            .translate(
                x,
                STATUS_POS.1 - STATUS_Y / 2.0 + 18.0,
                BASE_Z + STATUS_Z + 1.5,
            );
    }

    board + features
}

fn robot_service_keepouts() -> Part {
    let mut gauges = Part::empty(name("robot_service_keepout_gauges"));
    let specs = [
        (
            "front_robot_approach",
            0.0,
            -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0,
            STATION_X - 160.0,
            12.0,
            64.0,
        ),
        (
            "rear_service_access",
            0.0,
            STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE / 2.0,
            STATION_X - 170.0,
            12.0,
            56.0,
        ),
        (
            "left_head_service",
            -STATION_X / 2.0 + LEFT_HEAD_SERVICE_CLEARANCE / 2.0,
            0.0,
            12.0,
            STATION_Y - 170.0,
            60.0,
        ),
        (
            "right_sensor_service",
            STATION_X / 2.0 - RIGHT_SENSOR_SERVICE_CLEARANCE / 2.0,
            0.0,
            12.0,
            STATION_Y - 170.0,
            60.0,
        ),
        (
            "seeding_head_lift_clearance",
            HEAD_POS.0,
            HEAD_POS.1,
            92.0,
            16.0,
            112.0,
        ),
    ];

    assert_eq!(specs.len(), KEEP_OUT_ZONES);
    for (index, (label, x, y, sx, sy, height)) in specs.iter().enumerate() {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_{label}_floor_keepout_gauge_{index}"),
                *sx,
                *sy,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + KEEP_OUT_Z / 2.0)
            + centered_cube(
                format!("{PREFIX}_{label}_height_flag_{index}"),
                28.0,
                8.0,
                *height,
            )
            .translate(*x, *y, BASE_Z + *height / 2.0);
    }
    gauges
}

fn sealed_route_placeholders() -> Part {
    let mut routes = Part::empty(name("sealed_route_placeholders"));
    for row in 0..SLOT_ROWS {
        for col in 0..SLOT_COLS {
            let slot = row * SLOT_COLS + col;
            let (tx, ty) = grid_xy(
                TARGET_POS,
                row,
                SLOT_ROWS,
                col,
                SLOT_COLS,
                TARGET_PITCH_X,
                TARGET_PITCH_Y,
            );
            let down_runner = centered_cube(
                format!("{PREFIX}_target_{slot:02}_to_waste_sealed_runner"),
                4.0,
                116.0,
                4.0,
            )
            .translate(tx, ty - 74.0, BASE_Z + 16.0);
            routes = routes + down_runner;
        }
    }

    for lane in 0..CARRYOVER_LANES {
        let y = CARRYOVER_POS.1 + centered_index(lane, CARRYOVER_LANES, 34.0);
        routes = routes
            + centered_cube(
                format!("{PREFIX}_carryover_lane_{lane}_to_trough_runner"),
                5.0,
                132.0,
                4.0,
            )
            .translate(
                CARRYOVER_POS.0 + CARRYOVER_X / 2.0 - 28.0,
                y - 74.0,
                BASE_Z + 16.0,
            );
    }
    routes
}

fn plate_at(label: &str, center: (f64, f64), x: f64, y: f64, z: f64) -> Part {
    centered_cube(format!("{PREFIX}_{label}"), x, y, z).translate(center.0, center.1, z_on_base(z))
}

fn grid_xy(
    center: (f64, f64),
    row: usize,
    rows: usize,
    col: usize,
    cols: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    (
        center.0 + centered_index(col, cols, pitch_x),
        center.1 + centered_index(row, rows, pitch_y),
    )
}

fn mount_positions() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 52.0),
        (-220.0, -STATION_Y / 2.0 + 52.0),
        (220.0, -STATION_Y / 2.0 + 52.0),
        (-220.0, STATION_Y / 2.0 - 52.0),
        (220.0, STATION_Y / 2.0 - 52.0),
    ]
}

fn z_on_base(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(suffix: &str) -> String {
    format!("{PREFIX}_{suffix}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn output_paths_are_unique_and_scoped() {
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(PREFIX)));
    }

    #[test]
    fn sixteen_slot_validation_counts_are_stable() {
        assert_eq!(SLOT_ROWS, 4);
        assert_eq!(SLOT_COLS, 4);
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(ALIGNMENT_DATUM_COUNT, SLOT_COUNT);
        assert_eq!(BUBBLE_WITNESS_WELLS, SLOT_COUNT);
        assert_eq!(VOLUME_SAMPLE_CUPS, SLOT_COUNT);
        assert_eq!(WETNESS_STRIPS, SLOT_COUNT);
        assert_eq!(CARRYOVER_CHALLENGE_POSITIONS, SLOT_COUNT);
        assert_eq!(SENSOR_SAMPLE_PORTS, SLOT_COUNT);
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
        assert_layout();
    }

    #[test]
    fn validation_modes_are_explicitly_represented() {
        for feature in [
            "nozzle_alignment_datum_bank",
            "sixteen_position_witness_target_array",
            "bubble_clog_witness_wells",
            "volume_uniformity_sample_rack",
            "missed_dispense_wetness_lanes",
            "carryover_flush_challenge_lanes",
            "sensor_sampling_manifold",
            "camera_illumination_fiducial_bridge",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn bridge_and_service_clearances_exceed_fixture_stack() {
        assert!(BRIDGE_CLEARANCE_Z > BASE_Z + HEAD_Z + 90.0);
        assert!(FRONT_ROBOT_CLEARANCE >= 350.0);
        assert!(REAR_SERVICE_CLEARANCE >= 220.0);
        assert!(LEFT_HEAD_SERVICE_CLEARANCE >= 180.0);
        assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 170.0);
    }
}
