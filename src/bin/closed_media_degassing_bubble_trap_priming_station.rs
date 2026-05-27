use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media degassing, bubble-trap, and priming validation station.
//
// Intent:
// - Validate closed media preparation before perfusion without specifying
//   process acceptance limits or wetted materials.
// - Present a sterile media bag nest, vacuum/degassing chamber surrogate,
//   compliant bubble-trap cartridge bay, inline optical bubble sensor coupon
//   locations, priming loop with purge path, check-valve orientation witnesses,
//   pressure relief / occlusion challenge points, dripless quick-connect capture
//   tray, and barcode/lot/custody plate.
// - Keep the CAD deterministic and componentized for later parent integration.

const OUTPUT_PREFIX: &str = "closed_media_degassing_bubble_trap_priming_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_degassing_bubble_trap_priming_station_containment_deck.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_sterile_media_bag_nest.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_vacuum_degassing_chamber_surrogate.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_compliant_bubble_trap_cartridge_bay.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_inline_optical_bubble_sensor_coupons.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_priming_loop_purge_manifold.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_check_valve_orientation_witness_rail.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_pressure_relief_occlusion_panel.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_dripless_quick_connect_capture_tray.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_barcode_lot_custody_plate.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_robot_service_keepouts.stl",
    "output/closed_media_degassing_bubble_trap_priming_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sterile_media_bag_nest",
    "vacuum_degassing_chamber_surrogate",
    "compliant_bubble_trap_cartridge_bay",
    "inline_optical_bubble_sensor_coupon_locations",
    "priming_loop_with_purge_path",
    "check_valve_orientation_witness",
    "pressure_relief_challenge_points",
    "occlusion_challenge_points",
    "dripless_quick_connect_capture_tray",
    "barcode_lot_custody_plate",
    "secondary_containment_deck",
];

const STATION_X: f64 = 1480.0;
const STATION_Y: f64 = 920.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.9;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const DESIGN_CLEARANCE: f64 = 12.0;

const MEDIA_BAG_CENTER: (f64, f64) = (-410.0, 210.0);
const MEDIA_BAG_X: f64 = 440.0;
const MEDIA_BAG_Y: f64 = 260.0;
const MEDIA_BAG_Z: f64 = 46.0;
const MEDIA_BAG_RECESS_X: f64 = 330.0;
const MEDIA_BAG_RECESS_Y: f64 = 166.0;
const MEDIA_BAG_RECESS_DEPTH: f64 = 22.0;
const BAG_CLAMP_COUNT: usize = 6;
const BAG_PORT_COUNT: usize = 3;

const DEGASSING_CENTER: (f64, f64) = (110.0, 220.0);
const DEGASSING_X: f64 = 360.0;
const DEGASSING_Y: f64 = 250.0;
const DEGASSING_Z: f64 = 64.0;
const DEGASSING_CHAMBERS: usize = 3;
const DEGASSING_CHAMBER_D: f64 = 74.0;
const VACUUM_PORTS: usize = 3;
const VACUUM_GAUGE_D: f64 = 34.0;

const TRAP_CENTER: (f64, f64) = (500.0, 185.0);
const TRAP_X: f64 = 260.0;
const TRAP_Y: f64 = 280.0;
const TRAP_Z: f64 = 86.0;
const TRAP_COUNT: usize = 4;
const TRAP_PITCH_Y: f64 = 58.0;
const TRAP_CARTRIDGE_D: f64 = 34.0;
const TRAP_COMPLIANCE_PAD_X: f64 = 54.0;
const TRAP_COMPLIANCE_PAD_Z: f64 = 10.0;

const SENSOR_CENTER: (f64, f64) = (-410.0, -80.0);
const SENSOR_X: f64 = 440.0;
const SENSOR_Y: f64 = 170.0;
const SENSOR_Z: f64 = 54.0;
const SENSOR_COUPON_COUNT: usize = 8;
const SENSOR_COUPON_COLS: usize = 4;
const SENSOR_COUPON_PITCH_X: f64 = 88.0;
const SENSOR_COUPON_PITCH_Y: f64 = 58.0;
const SENSOR_WINDOW_X: f64 = 46.0;
const SENSOR_WINDOW_Y: f64 = 18.0;

const PRIMING_CENTER: (f64, f64) = (-90.0, -290.0);
const PRIMING_X: f64 = 650.0;
const PRIMING_Y: f64 = 175.0;
const PRIMING_Z: f64 = 48.0;
const PRIMING_LANES: usize = TRAP_COUNT;
const PRIMING_LANE_PITCH_X: f64 = 112.0;
const PURGE_BRANCHES_PER_LANE: usize = 2;
const PURGE_BRANCH_COUNT: usize = PRIMING_LANES * PURGE_BRANCHES_PER_LANE;
const PRIMING_LOOP_POST_D: f64 = 13.0;
const MAX_PRIMING_DEADLEG_MM: f64 = 10.0;

const VALVE_CENTER: (f64, f64) = (-550.0, -300.0);
const VALVE_X: f64 = 210.0;
const VALVE_Y: f64 = 160.0;
const VALVE_Z: f64 = 38.0;
const CHECK_VALVE_COUNT: usize = PRIMING_LANES;
const CHECK_VALVE_PITCH_Y: f64 = 34.0;
const ORIENTATION_ARROW_COUNT: usize = CHECK_VALVE_COUNT * 2;

const PRESSURE_CENTER: (f64, f64) = (240.0, -70.0);
const PRESSURE_X: f64 = 310.0;
const PRESSURE_Y: f64 = 190.0;
const PRESSURE_Z: f64 = 56.0;
const RELIEF_COUNT: usize = PRIMING_LANES;
const OCCLUSION_COUNT: usize = PRIMING_LANES;
const PRESSURE_POINT_PITCH_X: f64 = 58.0;
const RELIEF_LIMIT_KPA: f64 = 25.0;
const OCCLUSION_TEST_KPA: f64 = 35.0;

const QC_CENTER: (f64, f64) = (520.0, -300.0);
const QC_X: f64 = 250.0;
const QC_Y: f64 = 150.0;
const QC_Z: f64 = 42.0;
const QUICK_CONNECT_COUNT: usize = 6;
const QUICK_CONNECT_PITCH_X: f64 = 34.0;
const DRIP_CUP_D: f64 = 30.0;
const CAPTURE_TRAY_VOLUME_ML: f64 = 120.0;

const CUSTODY_CENTER: (f64, f64) = (0.0, 390.0);
const CUSTODY_X: f64 = 1150.0;
const CUSTODY_Y: f64 = 60.0;
const CUSTODY_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 8;
const LOT_TOKEN_COUNT: usize = 6;
const CUSTODY_SEAL_TABS: usize = 4;

const KEEP_OUT_X: f64 = 1370.0;
const KEEP_OUT_Y: f64 = 820.0;
const KEEP_OUT_Z: f64 = 155.0;
const FRONT_ROBOT_CLEARANCE: f64 = 285.0;
const REAR_BAG_SERVICE_CLEARANCE: f64 = 180.0;
const TOP_TRAP_LIFT_CLEARANCE: f64 = 235.0;
const SIDE_TUBING_SERVICE_CLEARANCE: f64 = 170.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - DESIGN_CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - DESIGN_CLEARANCE;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let bag_nest = sterile_media_bag_nest();
    export(OUTPUTS[1], &bag_nest);

    let degassing = vacuum_degassing_chamber_surrogate();
    export(OUTPUTS[2], &degassing);

    let traps = compliant_bubble_trap_cartridge_bay();
    export(OUTPUTS[3], &traps);

    let sensors = inline_optical_bubble_sensor_coupons();
    export(OUTPUTS[4], &sensors);

    let priming = priming_loop_purge_manifold();
    export(OUTPUTS[5], &priming);

    let valves = check_valve_orientation_witness_rail();
    export(OUTPUTS[6], &valves);

    let pressure = pressure_relief_occlusion_panel();
    export(OUTPUTS[7], &pressure);

    let qc_tray = dripless_quick_connect_capture_tray();
    export(OUTPUTS[8], &qc_tray);

    let custody = barcode_lot_custody_plate();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + bag_nest
        + degassing
        + traps
        + sensors
        + priming
        + valves
        + pressure
        + qc_tray
        + custody
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!(
        "Closed media degassing/bubble-trap/priming station: {:.0}mm x {:.0}mm contained deck, {} media bag ports, {} degassing chambers, {} compliant bubble traps, {} optical sensor coupon locations, {} priming lanes, {} purge branches, {} relief points, {} occlusion points, {} quick-connect drip cups, and {} custody lands.",
        STATION_X,
        STATION_Y,
        BAG_PORT_COUNT,
        DEGASSING_CHAMBERS,
        TRAP_COUNT,
        SENSOR_COUPON_COUNT,
        PRIMING_LANES,
        PURGE_BRANCH_COUNT,
        RELIEF_COUNT,
        OCCLUSION_COUNT,
        QUICK_CONNECT_COUNT,
        BARCODE_LANDS
    );
    println!(
        "Design intent coverage: {} feature groups, {:.0}kPa relief witness, {:.0}kPa occlusion challenge, {:.0}mm max priming dead-leg, {:.0}mm front robot clearance, {:.0}mm top trap lift clearance, {} STL outputs.",
        REQUIRED_FEATURES.len(),
        RELIEF_LIMIT_KPA,
        OCCLUSION_TEST_KPA,
        MAX_PRIMING_DEADLEG_MM,
        FRONT_ROBOT_CLEARANCE,
        TOP_TRAP_LIFT_CLEARANCE,
        OUTPUTS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_media_degassing_station_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "closed_media_degassing_station_shallow_washdown_basin",
        STATION_X - 2.0 * RIM_W,
        STATION_Y - 2.0 * RIM_W,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 1.0);
    let drain = centered_cylinder(
        "closed_media_degassing_station_front_basin_drain",
        12.0,
        70.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 80.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        DECK_Z - 6.0,
    );

    deck - basin - drain - deck_socket_reliefs() - deck_mount_holes()
        + containment_rims()
        + wet_path_direction_rails()
        + datum_bosses()
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "closed_media_degassing_station_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_media_degassing_station_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_media_degassing_station_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_media_degassing_station_front_low_service_lip",
        STATION_X - 150.0,
        12.0,
        24.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 6.0), DECK_Z + 12.0);

    left + right + rear + front
}

fn deck_socket_reliefs() -> Part {
    let mut sockets = Part::empty("closed_media_degassing_station_deck_socket_reliefs");
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_degassing_station_{}_socket_relief", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_media_degassing_station_mount_holes");
    for (index, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_media_degassing_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); 8] {
    [
        (-660.0, -390.0),
        (-220.0, -390.0),
        (220.0, -390.0),
        (660.0, -390.0),
        (-660.0, 350.0),
        (-220.0, 350.0),
        (220.0, 350.0),
        (660.0, 350.0),
    ]
}

fn wet_path_direction_rails() -> Part {
    let bag_to_degas = centered_cube(
        "closed_media_degassing_station_bag_to_degassing_flow_direction_rail",
        420.0,
        7.0,
        8.0,
    )
    .translate(-140.0, 78.0, DECK_Z + 4.0);
    let degas_to_trap = centered_cube(
        "closed_media_degassing_station_degassing_to_trap_flow_direction_rail",
        360.0,
        7.0,
        8.0,
    )
    .translate(315.0, 42.0, DECK_Z + 4.0);
    let trap_to_prime = centered_cube(
        "closed_media_degassing_station_trap_to_priming_flow_direction_rail",
        570.0,
        7.0,
        8.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(215.0, -116.0, DECK_Z + 4.0);

    bag_to_degas + degas_to_trap + trap_to_prime
}

fn datum_bosses() -> Part {
    let mut bosses = Part::empty("closed_media_degassing_station_datum_bosses");
    for (index, (x, y)) in [
        (-690.0, 410.0),
        (690.0, 410.0),
        (-690.0, -420.0),
        (690.0, -420.0),
    ]
    .iter()
    .enumerate()
    {
        bosses = bosses
            + datum_target(format!("closed_media_degassing_station_datum_{index}")).translate(
                *x,
                *y,
                DECK_Z + 3.0,
            );
    }
    bosses
}

fn datum_target(name: String) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 13.0, 4.0, 40);
    let dot = centered_cylinder(format!("{name}_center_dot"), 2.5, 5.0, 20);
    let cross_x = centered_cube(format!("{name}_cross_x"), 23.0, 2.5, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.5, 23.0, 5.0);
    disc + dot + cross_x + cross_y
}

fn sterile_media_bag_nest() -> Part {
    let body = centered_cube(
        "closed_media_degassing_sterile_media_bag_nest_body",
        MEDIA_BAG_X,
        MEDIA_BAG_Y,
        MEDIA_BAG_Z,
    )
    .translate(
        MEDIA_BAG_CENTER.0,
        MEDIA_BAG_CENTER.1,
        DECK_Z + MEDIA_BAG_Z / 2.0,
    );
    let bag_recess = centered_cube(
        "closed_media_degassing_sterile_bag_compliant_recess",
        MEDIA_BAG_RECESS_X,
        MEDIA_BAG_RECESS_Y,
        MEDIA_BAG_RECESS_DEPTH + 2.0,
    )
    .translate(
        MEDIA_BAG_CENTER.0 - 30.0,
        MEDIA_BAG_CENTER.1,
        DECK_Z + MEDIA_BAG_Z - MEDIA_BAG_RECESS_DEPTH / 2.0 + 1.0,
    );
    let heel_pocket = centered_cube(
        "closed_media_degassing_bag_hanger_heel_pocket",
        76.0,
        MEDIA_BAG_RECESS_Y + 18.0,
        18.0,
    )
    .translate(
        MEDIA_BAG_CENTER.0 - MEDIA_BAG_X / 2.0 + 62.0,
        MEDIA_BAG_CENTER.1,
        DECK_Z + MEDIA_BAG_Z - 8.0,
    );

    body - bag_recess - heel_pocket + bag_clamps() + bag_port_bulkhead_lands() + bag_barcode_land()
}

fn bag_clamps() -> Part {
    let mut clamps = Part::empty("closed_media_degassing_bag_clamps");
    for clamp in 0..BAG_CLAMP_COUNT {
        let x = MEDIA_BAG_CENTER.0 + centered_index(clamp, BAG_CLAMP_COUNT, 58.0) - 30.0;
        let y = if clamp % 2 == 0 {
            MEDIA_BAG_CENTER.1 - MEDIA_BAG_RECESS_Y / 2.0 - 15.0
        } else {
            MEDIA_BAG_CENTER.1 + MEDIA_BAG_RECESS_Y / 2.0 + 15.0
        };
        let jaw = centered_cube(
            format!("closed_media_degassing_bag_clamp_{clamp}_sterile_edge_jaw"),
            42.0,
            20.0,
            10.0,
        )
        .translate(x, y, DECK_Z + MEDIA_BAG_Z + 5.0);
        let screw = centered_cylinder(
            format!("closed_media_degassing_bag_clamp_{clamp}_thumb_screw_clearance"),
            3.4 / 2.0,
            12.0,
            20,
        )
        .translate(x, y, DECK_Z + MEDIA_BAG_Z + 5.0);
        clamps = clamps + (jaw - screw);
    }
    clamps
}

fn bag_port_bulkhead_lands() -> Part {
    let mut lands = Part::empty("closed_media_degassing_bag_port_bulkhead_lands");
    for port in 0..BAG_PORT_COUNT {
        let y = MEDIA_BAG_CENTER.1 + centered_index(port, BAG_PORT_COUNT, 42.0);
        let land = centered_cylinder(
            format!("closed_media_degassing_bag_port_{port}_sterile_bulkhead_land"),
            16.0,
            8.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            MEDIA_BAG_CENTER.0 + MEDIA_BAG_X / 2.0 + 4.0,
            y,
            DECK_Z + MEDIA_BAG_Z - 18.0,
        );
        let bore = centered_cylinder(
            format!("closed_media_degassing_bag_port_{port}_tube_bore"),
            FLUID_BORE_D / 2.0,
            18.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            MEDIA_BAG_CENTER.0 + MEDIA_BAG_X / 2.0 + 4.0,
            y,
            DECK_Z + MEDIA_BAG_Z - 18.0,
        );
        lands = lands + (land - bore);
    }
    lands
}

fn bag_barcode_land() -> Part {
    centered_cube(
        "closed_media_degassing_sterile_media_bag_barcode_land",
        120.0,
        22.0,
        6.0,
    )
    .translate(
        MEDIA_BAG_CENTER.0 - MEDIA_BAG_X / 2.0 + 88.0,
        MEDIA_BAG_CENTER.1 + MEDIA_BAG_Y / 2.0 - 28.0,
        DECK_Z + MEDIA_BAG_Z + 3.0,
    )
}

fn vacuum_degassing_chamber_surrogate() -> Part {
    let base = centered_cube(
        "closed_media_degassing_vacuum_chamber_surrogate_base",
        DEGASSING_X,
        DEGASSING_Y,
        DEGASSING_Z,
    )
    .translate(
        DEGASSING_CENTER.0,
        DEGASSING_CENTER.1,
        DECK_Z + DEGASSING_Z / 2.0,
    );
    let gasket_frame = rectangular_frame(
        "closed_media_degassing_vacuum_lid_gasket_witness_frame",
        DEGASSING_X - 34.0,
        DEGASSING_Y - 34.0,
        8.0,
        8.0,
    )
    .translate(
        DEGASSING_CENTER.0,
        DEGASSING_CENTER.1,
        DECK_Z + DEGASSING_Z + 4.0,
    );

    base - degassing_chamber_cavities() - vacuum_line_bores()
        + gasket_frame
        + vacuum_port_bosses()
        + degassing_lid_clamp_tabs()
}

fn degassing_chamber_cavities() -> Part {
    let mut cavities = Part::empty("closed_media_degassing_chamber_cavities");
    for chamber in 0..DEGASSING_CHAMBERS {
        let x = DEGASSING_CENTER.0 + centered_index(chamber, DEGASSING_CHAMBERS, 96.0);
        let lower = centered_cylinder(
            format!("closed_media_degassing_chamber_{chamber}_lower_bowl"),
            DEGASSING_CHAMBER_D / 2.0,
            DEGASSING_Z - 12.0,
            48,
        )
        .translate(
            x,
            DEGASSING_CENTER.1 - 14.0,
            DECK_Z + DEGASSING_Z / 2.0 + 4.0,
        );
        let view_slot = centered_cube(
            format!("closed_media_degassing_chamber_{chamber}_meniscus_view_slot"),
            15.0,
            14.0,
            DEGASSING_Z - 18.0,
        )
        .translate(
            x,
            DEGASSING_CENTER.1 - DEGASSING_Y / 2.0,
            DECK_Z + DEGASSING_Z / 2.0,
        );
        cavities = cavities + lower + view_slot;
    }
    cavities
}

fn vacuum_line_bores() -> Part {
    let feed = centered_cylinder(
        "closed_media_degassing_vacuum_feed_gallery_bore",
        FLUID_BORE_D / 2.0,
        DEGASSING_X + 24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(DEGASSING_CENTER.0, DEGASSING_CENTER.1 + 78.0, DECK_Z + 31.0);
    let outlet = centered_cylinder(
        "closed_media_degassing_media_outlet_gallery_bore",
        FLUID_BORE_D / 2.0,
        DEGASSING_X + 24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(DEGASSING_CENTER.0, DEGASSING_CENTER.1 - 78.0, DECK_Z + 31.0);
    feed + outlet
}

fn vacuum_port_bosses() -> Part {
    let mut bosses = Part::empty("closed_media_degassing_vacuum_port_bosses");
    for port in 0..VACUUM_PORTS {
        let x = DEGASSING_CENTER.0 + centered_index(port, VACUUM_PORTS, 96.0);
        bosses = bosses
            + centered_cylinder(
                format!("closed_media_degassing_vacuum_port_{port}_boss"),
                13.0,
                10.0,
                32,
            )
            .translate(x, DEGASSING_CENTER.1 + 78.0, DECK_Z + DEGASSING_Z + 5.0);
    }
    bosses
}

fn degassing_lid_clamp_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_degassing_lid_clamp_tabs");
    for (i, (x, y)) in [
        (
            DEGASSING_CENTER.0 - DEGASSING_X / 2.0 + 34.0,
            DEGASSING_CENTER.1 - DEGASSING_Y / 2.0 + 28.0,
        ),
        (
            DEGASSING_CENTER.0 + DEGASSING_X / 2.0 - 34.0,
            DEGASSING_CENTER.1 - DEGASSING_Y / 2.0 + 28.0,
        ),
        (
            DEGASSING_CENTER.0 - DEGASSING_X / 2.0 + 34.0,
            DEGASSING_CENTER.1 + DEGASSING_Y / 2.0 - 28.0,
        ),
        (
            DEGASSING_CENTER.0 + DEGASSING_X / 2.0 - 34.0,
            DEGASSING_CENTER.1 + DEGASSING_Y / 2.0 - 28.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let tab = centered_cube(
            format!("closed_media_degassing_lid_clamp_tab_{i}"),
            42.0,
            24.0,
            12.0,
        )
        .translate(*x, *y, DECK_Z + DEGASSING_Z + 6.0);
        let screw = centered_cylinder(
            format!("closed_media_degassing_lid_clamp_tab_{i}_screw"),
            3.8 / 2.0,
            14.0,
            22,
        )
        .translate(*x, *y, DECK_Z + DEGASSING_Z + 6.0);
        tabs = tabs + (tab - screw);
    }
    tabs
}

fn compliant_bubble_trap_cartridge_bay() -> Part {
    let bay = centered_cube(
        "closed_media_degassing_compliant_bubble_trap_cartridge_bay_body",
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    )
    .translate(TRAP_CENTER.0, TRAP_CENTER.1, DECK_Z + TRAP_Z / 2.0);
    let backplane = centered_cube(
        "closed_media_degassing_bubble_trap_sterile_bulkhead_backplane",
        22.0,
        TRAP_Y - 28.0,
        72.0,
    )
    .translate(
        TRAP_CENTER.0 - TRAP_X / 2.0 + 18.0,
        TRAP_CENTER.1,
        DECK_Z + 52.0,
    );

    bay - bubble_trap_cartridge_cuts() - bubble_trap_tube_bores()
        + backplane
        + bubble_trap_compliance_pads()
        + bubble_trap_up_witnesses()
}

fn bubble_trap_cartridge_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_degassing_bubble_trap_cartridge_cuts");
    for trap in 0..TRAP_COUNT {
        let y = trap_y(trap);
        let cavity = centered_cylinder(
            format!("closed_media_degassing_bubble_trap_{trap}_cartridge_socket"),
            TRAP_CARTRIDGE_D / 2.0,
            TRAP_X - 52.0,
            40,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(TRAP_CENTER.0 + 8.0, y, DECK_Z + 48.0);
        let view_window = centered_cube(
            format!("closed_media_degassing_bubble_trap_{trap}_air_line_view_window"),
            74.0,
            12.0,
            34.0,
        )
        .translate(TRAP_CENTER.0 + 28.0, y, DECK_Z + 72.0);
        cuts = cuts + cavity + view_window;
    }
    cuts
}

fn bubble_trap_tube_bores() -> Part {
    let mut bores = Part::empty("closed_media_degassing_bubble_trap_tube_bores");
    for trap in 0..TRAP_COUNT {
        let y = trap_y(trap);
        bores = bores
            + centered_cylinder(
                format!("closed_media_degassing_bubble_trap_{trap}_low_inlet_bore"),
                FLUID_BORE_D / 2.0,
                TRAP_X + 16.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(TRAP_CENTER.0, y - 10.0, DECK_Z + 38.0)
            + centered_cylinder(
                format!("closed_media_degassing_bubble_trap_{trap}_high_outlet_bore"),
                FLUID_BORE_D / 2.0,
                TRAP_X + 16.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(TRAP_CENTER.0, y + 10.0, DECK_Z + 64.0);
    }
    bores
}

fn bubble_trap_compliance_pads() -> Part {
    let mut pads = Part::empty("closed_media_degassing_bubble_trap_compliance_pads");
    for trap in 0..TRAP_COUNT {
        let y = trap_y(trap);
        pads = pads
            + centered_cube(
                format!("closed_media_degassing_bubble_trap_{trap}_front_compliance_pad"),
                TRAP_COMPLIANCE_PAD_X,
                8.0,
                TRAP_COMPLIANCE_PAD_Z,
            )
            .translate(TRAP_CENTER.0 + 12.0, y - 24.0, DECK_Z + TRAP_Z + 5.0)
            + centered_cube(
                format!("closed_media_degassing_bubble_trap_{trap}_rear_compliance_pad"),
                TRAP_COMPLIANCE_PAD_X,
                8.0,
                TRAP_COMPLIANCE_PAD_Z,
            )
            .translate(TRAP_CENTER.0 + 12.0, y + 24.0, DECK_Z + TRAP_Z + 5.0);
    }
    pads
}

fn bubble_trap_up_witnesses() -> Part {
    let mut witnesses = Part::empty("closed_media_degassing_bubble_trap_up_witnesses");
    for trap in 0..TRAP_COUNT {
        let y = trap_y(trap);
        witnesses = witnesses
            + centered_cube(
                format!("closed_media_degassing_bubble_trap_{trap}_up_stem"),
                6.0,
                6.0,
                44.0,
            )
            .translate(TRAP_CENTER.0 + TRAP_X / 2.0 - 34.0, y, DECK_Z + 54.0)
            + centered_cube(
                format!("closed_media_degassing_bubble_trap_{trap}_up_arrow_cap"),
                22.0,
                8.0,
                8.0,
            )
            .translate(TRAP_CENTER.0 + TRAP_X / 2.0 - 34.0, y, DECK_Z + 80.0);
    }
    witnesses
}

fn inline_optical_bubble_sensor_coupons() -> Part {
    let plate = centered_cube(
        "closed_media_degassing_inline_optical_sensor_coupon_plate",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(SENSOR_CENTER.0, SENSOR_CENTER.1, DECK_Z + SENSOR_Z / 2.0);

    plate - sensor_coupon_recesses() + optical_sensor_forks() + sensor_cable_tie_lands()
}

fn sensor_coupon_recesses() -> Part {
    let mut recesses = Part::empty("closed_media_degassing_optical_sensor_coupon_recesses");
    for coupon in 0..SENSOR_COUPON_COUNT {
        let (x, y) = sensor_coupon_xy(coupon);
        let slot = centered_cube(
            format!("closed_media_degassing_sensor_coupon_{coupon}_transparent_tube_window"),
            SENSOR_WINDOW_X,
            SENSOR_WINDOW_Y,
            18.0,
        )
        .translate(x, y, DECK_Z + SENSOR_Z - 8.0);
        let bore = centered_cylinder(
            format!("closed_media_degassing_sensor_coupon_{coupon}_tube_clearance"),
            FLUID_BORE_D / 2.0,
            SENSOR_WINDOW_X + 20.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, DECK_Z + SENSOR_Z - 14.0);
        recesses = recesses + slot + bore;
    }
    recesses
}

fn optical_sensor_forks() -> Part {
    let mut forks = Part::empty("closed_media_degassing_optical_sensor_forks");
    for coupon in 0..SENSOR_COUPON_COUNT {
        let (x, y) = sensor_coupon_xy(coupon);
        forks = forks
            + centered_cube(
                format!("closed_media_degassing_sensor_coupon_{coupon}_led_arm"),
                12.0,
                10.0,
                30.0,
            )
            .translate(x - 18.0, y, DECK_Z + SENSOR_Z + 15.0)
            + centered_cube(
                format!("closed_media_degassing_sensor_coupon_{coupon}_detector_arm"),
                12.0,
                10.0,
                30.0,
            )
            .translate(x + 18.0, y, DECK_Z + SENSOR_Z + 15.0);
    }
    forks
}

fn sensor_cable_tie_lands() -> Part {
    let mut lands = Part::empty("closed_media_degassing_sensor_cable_tie_lands");
    for land in 0..4 {
        let x = SENSOR_CENTER.0 + centered_index(land, 4, 92.0);
        let pad = centered_cube(
            format!("closed_media_degassing_sensor_cable_tie_land_{land}"),
            52.0,
            14.0,
            8.0,
        )
        .translate(
            x,
            SENSOR_CENTER.1 - SENSOR_Y / 2.0 + 20.0,
            DECK_Z + SENSOR_Z + 4.0,
        );
        let slot = centered_cube(
            format!("closed_media_degassing_sensor_cable_tie_slot_{land}"),
            34.0,
            4.0,
            10.0,
        )
        .translate(
            x,
            SENSOR_CENTER.1 - SENSOR_Y / 2.0 + 20.0,
            DECK_Z + SENSOR_Z + 4.0,
        );
        lands = lands + (pad - slot);
    }
    lands
}

fn priming_loop_purge_manifold() -> Part {
    let body = centered_cube(
        "closed_media_degassing_priming_loop_purge_manifold_body",
        PRIMING_X,
        PRIMING_Y,
        PRIMING_Z,
    )
    .translate(PRIMING_CENTER.0, PRIMING_CENTER.1, DECK_Z + PRIMING_Z / 2.0);

    body - priming_lane_bores()
        + priming_loop_posts()
        + purge_branch_lands()
        + deadleg_limit_flags()
}

fn priming_lane_bores() -> Part {
    let mut bores = Part::empty("closed_media_degassing_priming_lane_bores");
    for lane in 0..PRIMING_LANES {
        let x = priming_lane_x(lane);
        bores = bores
            + centered_cylinder(
                format!("closed_media_degassing_priming_lane_{lane}_main_loop_bore"),
                FLUID_BORE_D / 2.0,
                PRIMING_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PRIMING_CENTER.1, DECK_Z + 27.0)
            + centered_cylinder(
                format!("closed_media_degassing_priming_lane_{lane}_purge_branch_bore"),
                FLUID_BORE_D / 2.0,
                92.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x + 28.0, PRIMING_CENTER.1 - 45.0, DECK_Z + 30.0);
    }
    bores
}

fn priming_loop_posts() -> Part {
    let mut posts = Part::empty("closed_media_degassing_priming_loop_posts");
    for lane in 0..PRIMING_LANES {
        let x = priming_lane_x(lane);
        for (side, y) in [
            ("inlet", PRIMING_CENTER.1 + PRIMING_Y / 2.0 - 24.0),
            ("return", PRIMING_CENTER.1 - PRIMING_Y / 2.0 + 24.0),
        ] {
            posts = posts
                + centered_cylinder(
                    format!("closed_media_degassing_priming_lane_{lane}_{side}_loop_post"),
                    PRIMING_LOOP_POST_D / 2.0,
                    24.0,
                    28,
                )
                .translate(x, y, DECK_Z + PRIMING_Z + 12.0);
        }
    }
    posts
}

fn purge_branch_lands() -> Part {
    let mut lands = Part::empty("closed_media_degassing_purge_branch_lands");
    for lane in 0..PRIMING_LANES {
        let x = priming_lane_x(lane);
        for branch in 0..PURGE_BRANCHES_PER_LANE {
            let branch_index = lane * PURGE_BRANCHES_PER_LANE + branch;
            lands = lands
                + centered_cube(
                    format!("closed_media_degassing_purge_branch_{branch_index}_pinch_land"),
                    30.0,
                    18.0,
                    8.0,
                )
                .translate(
                    x + 44.0,
                    PRIMING_CENTER.1 + centered_index(branch, PURGE_BRANCHES_PER_LANE, 58.0),
                    DECK_Z + PRIMING_Z + 4.0,
                );
        }
    }
    lands
}

fn deadleg_limit_flags() -> Part {
    let mut flags = Part::empty("closed_media_degassing_deadleg_limit_flags");
    for lane in 0..PRIMING_LANES {
        let x = priming_lane_x(lane);
        flags = flags
            + centered_cube(
                format!(
                    "closed_media_degassing_priming_lane_{lane}_{:.0}mm_max_deadleg_flag",
                    MAX_PRIMING_DEADLEG_MM
                ),
                42.0,
                7.0,
                12.0,
            )
            .translate(x, PRIMING_CENTER.1, DECK_Z + PRIMING_Z + 6.0);
    }
    flags
}

fn check_valve_orientation_witness_rail() -> Part {
    let rail = centered_cube(
        "closed_media_degassing_check_valve_orientation_witness_rail",
        VALVE_X,
        VALVE_Y,
        VALVE_Z,
    )
    .translate(VALVE_CENTER.0, VALVE_CENTER.1, DECK_Z + VALVE_Z / 2.0);

    rail - check_valve_socket_cuts() + check_valve_arrows() + check_valve_stop_tabs()
}

fn check_valve_socket_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_degassing_check_valve_socket_cuts");
    for valve in 0..CHECK_VALVE_COUNT {
        let y = check_valve_y(valve);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_degassing_check_valve_{valve}_body_socket"),
                12.0,
                68.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(VALVE_CENTER.0, y, DECK_Z + VALVE_Z - 12.0)
            + centered_cylinder(
                format!("closed_media_degassing_check_valve_{valve}_flow_bore"),
                FLUID_BORE_D / 2.0,
                VALVE_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(VALVE_CENTER.0, y, DECK_Z + VALVE_Z - 12.0);
    }
    cuts
}

fn check_valve_arrows() -> Part {
    let mut arrows = Part::empty("closed_media_degassing_check_valve_orientation_arrows");
    for valve in 0..CHECK_VALVE_COUNT {
        let y = check_valve_y(valve);
        for arrow in 0..2 {
            let x = VALVE_CENTER.0 + centered_index(arrow, 2, 42.0);
            arrows = arrows
                + centered_cube(
                    format!("closed_media_degassing_check_valve_{valve}_orientation_arrow_{arrow}"),
                    30.0,
                    6.0,
                    7.0,
                )
                .translate(x, y + 14.0, DECK_Z + VALVE_Z + 3.5)
                + centered_cube(
                    format!("closed_media_degassing_check_valve_{valve}_arrow_head_{arrow}"),
                    8.0,
                    18.0,
                    7.0,
                )
                .translate(x + 17.0, y + 14.0, DECK_Z + VALVE_Z + 3.5);
        }
    }
    arrows
}

fn check_valve_stop_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_degassing_check_valve_stop_tabs");
    for valve in 0..CHECK_VALVE_COUNT {
        let y = check_valve_y(valve);
        tabs = tabs
            + centered_cube(
                format!("closed_media_degassing_check_valve_{valve}_inlet_stop_tab"),
                8.0,
                26.0,
                18.0,
            )
            .translate(
                VALVE_CENTER.0 - VALVE_X / 2.0 + 38.0,
                y,
                DECK_Z + VALVE_Z + 9.0,
            );
    }
    tabs
}

fn pressure_relief_occlusion_panel() -> Part {
    let panel = centered_cube(
        "closed_media_degassing_pressure_relief_occlusion_panel_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1,
        DECK_Z + PRESSURE_Z / 2.0,
    );

    panel - pressure_bores()
        + relief_valve_lands()
        + occlusion_anvil_posts()
        + pressure_witness_flags()
}

fn pressure_bores() -> Part {
    let mut bores = Part::empty("closed_media_degassing_pressure_panel_bores");
    for lane in 0..PRIMING_LANES {
        let x = pressure_point_x(lane);
        bores = bores
            + centered_cylinder(
                format!("closed_media_degassing_pressure_lane_{lane}_inline_bore"),
                FLUID_BORE_D / 2.0,
                PRESSURE_Y + 14.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PRESSURE_CENTER.1, DECK_Z + 28.0)
            + centered_cylinder(
                format!("closed_media_degassing_pressure_lane_{lane}_gauge_socket"),
                VACUUM_GAUGE_D / 2.0,
                16.0,
                36,
            )
            .translate(x, PRESSURE_CENTER.1 + 46.0, DECK_Z + PRESSURE_Z);
    }
    bores
}

fn relief_valve_lands() -> Part {
    let mut lands = Part::empty("closed_media_degassing_pressure_relief_valve_lands");
    for relief in 0..RELIEF_COUNT {
        let x = pressure_point_x(relief);
        lands = lands
            + centered_cylinder(
                format!(
                    "closed_media_degassing_relief_{relief}_{:.0}kpa_land",
                    RELIEF_LIMIT_KPA
                ),
                15.0,
                9.0,
                32,
            )
            .translate(x, PRESSURE_CENTER.1 - 44.0, DECK_Z + PRESSURE_Z + 4.5);
    }
    lands
}

fn occlusion_anvil_posts() -> Part {
    let mut posts = Part::empty("closed_media_degassing_occlusion_anvil_posts");
    for occlusion in 0..OCCLUSION_COUNT {
        let x = pressure_point_x(occlusion);
        posts = posts
            + centered_cube(
                format!("closed_media_degassing_occlusion_{occlusion}_fixed_anvil"),
                34.0,
                8.0,
                26.0,
            )
            .translate(
                x - 12.0,
                PRESSURE_CENTER.1 + 8.0,
                DECK_Z + PRESSURE_Z + 13.0,
            )
            + centered_cube(
                format!("closed_media_degassing_occlusion_{occlusion}_movable_anvil_reference"),
                34.0,
                8.0,
                18.0,
            )
            .translate(
                x + 12.0,
                PRESSURE_CENTER.1 + 22.0,
                DECK_Z + PRESSURE_Z + 9.0,
            );
    }
    posts
}

fn pressure_witness_flags() -> Part {
    let relief_flag = centered_cube(
        "closed_media_degassing_pressure_relief_setpoint_witness_flag",
        86.0,
        8.0,
        16.0,
    )
    .translate(
        PRESSURE_CENTER.0 - PRESSURE_X / 2.0 + 58.0,
        PRESSURE_CENTER.1 - PRESSURE_Y / 2.0 + 20.0,
        DECK_Z + PRESSURE_Z + 8.0,
    );
    let occlusion_flag = centered_cube(
        "closed_media_degassing_occlusion_challenge_witness_flag",
        96.0,
        8.0,
        16.0,
    )
    .translate(
        PRESSURE_CENTER.0 + PRESSURE_X / 2.0 - 64.0,
        PRESSURE_CENTER.1 - PRESSURE_Y / 2.0 + 20.0,
        DECK_Z + PRESSURE_Z + 8.0,
    );
    relief_flag + occlusion_flag
}

fn dripless_quick_connect_capture_tray() -> Part {
    let tray = centered_cube(
        "closed_media_degassing_dripless_quick_connect_capture_tray_body",
        QC_X,
        QC_Y,
        QC_Z,
    )
    .translate(QC_CENTER.0, QC_CENTER.1, DECK_Z + QC_Z / 2.0);
    let sump = centered_cube(
        "closed_media_degassing_quick_connect_capture_sump",
        QC_X - 42.0,
        QC_Y - 44.0,
        QC_Z - 16.0,
    )
    .translate(QC_CENTER.0, QC_CENTER.1, DECK_Z + QC_Z / 2.0 + 6.0);
    let front_pour_off = centered_cylinder(
        "closed_media_degassing_quick_connect_capture_drain_port",
        7.0,
        QC_Y + 12.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(QC_CENTER.0, QC_CENTER.1 - QC_Y / 2.0, DECK_Z + 16.0);

    tray - sump - front_pour_off + quick_connect_cups() + drip_shields()
}

fn quick_connect_cups() -> Part {
    let mut cups = Part::empty("closed_media_degassing_quick_connect_cups");
    for connector in 0..QUICK_CONNECT_COUNT {
        let x = QC_CENTER.0 + centered_index(connector, QUICK_CONNECT_COUNT, QUICK_CONNECT_PITCH_X);
        let y = QC_CENTER.1 + 20.0;
        let cup = centered_cylinder(
            format!("closed_media_degassing_quick_connect_{connector}_drip_cup"),
            DRIP_CUP_D / 2.0,
            16.0,
            32,
        )
        .translate(x, y, DECK_Z + QC_Z + 8.0);
        let socket = centered_cylinder(
            format!("closed_media_degassing_quick_connect_{connector}_socket_relief"),
            18.0 / 2.0,
            18.0,
            28,
        )
        .translate(x, y, DECK_Z + QC_Z + 8.0);
        cups = cups + (cup - socket);
    }
    cups
}

fn drip_shields() -> Part {
    let rear = centered_cube(
        "closed_media_degassing_quick_connect_rear_splash_shield",
        QC_X - 30.0,
        8.0,
        34.0,
    )
    .translate(
        QC_CENTER.0,
        QC_CENTER.1 + QC_Y / 2.0 - 8.0,
        DECK_Z + QC_Z + 17.0,
    );
    let left = centered_cube(
        "closed_media_degassing_quick_connect_left_splash_shield",
        8.0,
        QC_Y - 22.0,
        24.0,
    )
    .translate(
        QC_CENTER.0 - QC_X / 2.0 + 8.0,
        QC_CENTER.1,
        DECK_Z + QC_Z + 12.0,
    );
    let right = centered_cube(
        "closed_media_degassing_quick_connect_right_splash_shield",
        8.0,
        QC_Y - 22.0,
        24.0,
    )
    .translate(
        QC_CENTER.0 + QC_X / 2.0 - 8.0,
        QC_CENTER.1,
        DECK_Z + QC_Z + 12.0,
    );
    rear + left + right
}

fn barcode_lot_custody_plate() -> Part {
    let plate = centered_cube(
        "closed_media_degassing_barcode_lot_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, DECK_Z + CUSTODY_Z / 2.0);

    plate + barcode_lands() + lot_tokens() + custody_seal_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_media_degassing_barcode_lands");
    for land in 0..BARCODE_LANDS {
        let x = CUSTODY_CENTER.0 + centered_index(land, BARCODE_LANDS, 128.0);
        lands = lands
            + centered_cube(
                format!("closed_media_degassing_barcode_land_{land}"),
                96.0,
                18.0,
                5.0,
            )
            .translate(x, CUSTODY_CENTER.1 - 14.0, DECK_Z + CUSTODY_Z + 2.5);
    }
    lands
}

fn lot_tokens() -> Part {
    let mut tokens = Part::empty("closed_media_degassing_lot_tokens");
    for token in 0..LOT_TOKEN_COUNT {
        let x = CUSTODY_CENTER.0 + centered_index(token, LOT_TOKEN_COUNT, 64.0);
        tokens = tokens
            + centered_cylinder(
                format!("closed_media_degassing_lot_token_{token}"),
                10.0,
                5.0,
                28,
            )
            .translate(x, CUSTODY_CENTER.1 + 16.0, DECK_Z + CUSTODY_Z + 2.5);
    }
    tokens
}

fn custody_seal_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_degassing_custody_seal_tabs");
    for tab in 0..CUSTODY_SEAL_TABS {
        let x = CUSTODY_CENTER.0 + centered_index(tab, CUSTODY_SEAL_TABS, 330.0);
        let ear = centered_cube(
            format!("closed_media_degassing_custody_seal_tab_{tab}"),
            34.0,
            24.0,
            8.0,
        )
        .translate(x, CUSTODY_CENTER.1 + CUSTODY_Y / 2.0 + 10.0, DECK_Z + 4.0);
        let seal_hole = centered_cylinder(
            format!("closed_media_degassing_custody_seal_tab_{tab}_tamper_hole"),
            4.0,
            10.0,
            20,
        )
        .translate(x, CUSTODY_CENTER.1 + CUSTODY_Y / 2.0 + 10.0, DECK_Z + 4.0);
        tabs = tabs + (ear - seal_hole);
    }
    tabs
}

fn robot_service_keepouts() -> Part {
    let outline = keepout_frame(
        "closed_media_degassing_station_robot_service_outline",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        8.0,
    )
    .translate(0.0, 0.0, DECK_Z + 4.0);
    let trap_lift = keepout_frame(
        "closed_media_degassing_bubble_trap_vertical_lift_keepout",
        TRAP_X + 80.0,
        TRAP_Y + 80.0,
        8.0,
    )
    .translate(
        TRAP_CENTER.0,
        TRAP_CENTER.1,
        DECK_Z + TOP_TRAP_LIFT_CLEARANCE,
    );
    let bag_service = centered_cube(
        "closed_media_degassing_rear_bag_service_clearance_gauge",
        MEDIA_BAG_X + 90.0,
        8.0,
        8.0,
    )
    .translate(
        MEDIA_BAG_CENTER.0,
        MEDIA_BAG_CENTER.1 + MEDIA_BAG_Y / 2.0 + REAR_BAG_SERVICE_CLEARANCE,
        DECK_Z + 4.0,
    );
    let front_robot = centered_cube(
        "closed_media_degassing_front_robot_sweep_clearance_gauge",
        STATION_X - 180.0,
        8.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, DECK_Z + 4.0);
    let side_service = centered_cube(
        "closed_media_degassing_side_tubing_service_clearance_gauge",
        8.0,
        STATION_Y - 240.0,
        8.0,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_TUBING_SERVICE_CLEARANCE,
        -20.0,
        DECK_Z + 4.0,
    );
    let deck_height_limit = keepout_frame(
        "closed_media_degassing_station_component_height_limit_keepout",
        STATION_X - 260.0,
        STATION_Y - 260.0,
        8.0,
    )
    .translate(0.0, -20.0, DECK_Z + KEEP_OUT_Z);

    outline + trap_lift + bag_service + front_robot + side_service + deck_height_limit
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn rectangular_frame(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, wall, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear"), x, wall, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), wall, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), wall, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "sterile_media_bag_nest",
            MEDIA_BAG_CENTER,
            MEDIA_BAG_X,
            MEDIA_BAG_Y,
        ),
        rect(
            "vacuum_degassing_chamber_surrogate",
            DEGASSING_CENTER,
            DEGASSING_X,
            DEGASSING_Y,
        ),
        rect(
            "compliant_bubble_trap_cartridge_bay",
            TRAP_CENTER,
            TRAP_X,
            TRAP_Y,
        ),
        rect(
            "inline_optical_bubble_sensor_coupons",
            SENSOR_CENTER,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect(
            "priming_loop_purge_manifold",
            PRIMING_CENTER,
            PRIMING_X,
            PRIMING_Y,
        ),
        rect(
            "check_valve_orientation_witness_rail",
            VALVE_CENTER,
            VALVE_X,
            VALVE_Y,
        ),
        rect(
            "pressure_relief_occlusion_panel",
            PRESSURE_CENTER,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        rect("dripless_quick_connect_capture_tray", QC_CENTER, QC_X, QC_Y),
        rect(
            "barcode_lot_custody_plate",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn trap_y(trap: usize) -> f64 {
    TRAP_CENTER.1 + centered_index(trap, TRAP_COUNT, TRAP_PITCH_Y)
}

fn sensor_coupon_xy(coupon: usize) -> (f64, f64) {
    let col = coupon % SENSOR_COUPON_COLS;
    let row = coupon / SENSOR_COUPON_COLS;
    let rows = SENSOR_COUPON_COUNT.div_ceil(SENSOR_COUPON_COLS);
    (
        SENSOR_CENTER.0 + centered_index(col, SENSOR_COUPON_COLS, SENSOR_COUPON_PITCH_X),
        SENSOR_CENTER.1 + centered_index(row, rows, SENSOR_COUPON_PITCH_Y),
    )
}

fn priming_lane_x(lane: usize) -> f64 {
    PRIMING_CENTER.0 + centered_index(lane, PRIMING_LANES, PRIMING_LANE_PITCH_X)
}

fn check_valve_y(valve: usize) -> f64 {
    VALVE_CENTER.1 + centered_index(valve, CHECK_VALVE_COUNT, CHECK_VALVE_PITCH_Y)
}

fn pressure_point_x(point: usize) -> f64 {
    PRESSURE_CENTER.0 + centered_index(point, RELIEF_COUNT, PRESSURE_POINT_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn capture_tray_nominal_volume_ml() -> f64 {
    (QC_X - 42.0) * (QC_Y - 44.0) * (QC_Z - 16.0) / 1000.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(TRAP_COUNT, PRIMING_LANES);
    assert_eq!(CHECK_VALVE_COUNT, PRIMING_LANES);
    assert_eq!(RELIEF_COUNT, PRIMING_LANES);
    assert_eq!(OCCLUSION_COUNT, PRIMING_LANES);
    assert_eq!(PURGE_BRANCH_COUNT, PRIMING_LANES * PURGE_BRANCHES_PER_LANE);
    assert_eq!(ORIENTATION_ARROW_COUNT, CHECK_VALVE_COUNT * 2);
    assert!(MEDIA_BAG_RECESS_DEPTH < MEDIA_BAG_Z);
    assert!(DEGASSING_Z > DEGASSING_CHAMBER_D / 2.0);
    assert!(TRAP_Z > TRAP_CARTRIDGE_D * 2.0);
    assert!(MAX_PRIMING_DEADLEG_MM <= 10.0);
    assert!(RELIEF_LIMIT_KPA < OCCLUSION_TEST_KPA);
    assert!(capture_tray_nominal_volume_ml() > CAPTURE_TRAY_VOLUME_ML);
    assert!(TOP_TRAP_LIFT_CLEARANCE > DECK_Z + TRAP_Z);

    let rects = module_rects();
    for rect in rects {
        assert!(rect.fits_inside_station(), "{} exceeds deck", rect.name);
    }
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps_with_clearance(rects[b], DESIGN_CLEARANCE),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_media_degassing_bubble_trap_priming_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_design_intent_is_represented() {
        assert!(REQUIRED_FEATURES.contains(&"sterile_media_bag_nest"));
        assert!(REQUIRED_FEATURES.contains(&"vacuum_degassing_chamber_surrogate"));
        assert!(REQUIRED_FEATURES.contains(&"compliant_bubble_trap_cartridge_bay"));
        assert!(REQUIRED_FEATURES.contains(&"inline_optical_bubble_sensor_coupon_locations"));
        assert!(REQUIRED_FEATURES.contains(&"priming_loop_with_purge_path"));
        assert!(REQUIRED_FEATURES.contains(&"check_valve_orientation_witness"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_relief_challenge_points"));
        assert!(REQUIRED_FEATURES.contains(&"occlusion_challenge_points"));
        assert!(REQUIRED_FEATURES.contains(&"dripless_quick_connect_capture_tray"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_lot_custody_plate"));
    }

    #[test]
    fn closed_fluid_path_counts_are_lane_consistent() {
        assert_eq!(TRAP_COUNT, 4);
        assert_eq!(PRIMING_LANES, TRAP_COUNT);
        assert_eq!(CHECK_VALVE_COUNT, PRIMING_LANES);
        assert_eq!(RELIEF_COUNT, PRIMING_LANES);
        assert_eq!(OCCLUSION_COUNT, PRIMING_LANES);
        assert_eq!(PURGE_BRANCH_COUNT, 8);
        assert_eq!(ORIENTATION_ARROW_COUNT, 8);
    }

    #[test]
    fn dimensions_and_capacity_have_clear_validation_margins() {
        assert!(MEDIA_BAG_RECESS_X < MEDIA_BAG_X);
        assert!(MEDIA_BAG_RECESS_Y < MEDIA_BAG_Y);
        assert!(MEDIA_BAG_RECESS_DEPTH < MEDIA_BAG_Z);
        assert!(TRAP_Z > TRAP_CARTRIDGE_D * 2.0);
        assert!(DEGASSING_CHAMBER_D < DEGASSING_Y / 2.0);
        assert!(capture_tray_nominal_volume_ml() > CAPTURE_TRAY_VOLUME_ML);
        assert!(RELIEF_LIMIT_KPA < OCCLUSION_TEST_KPA);
    }

    #[test]
    fn modules_fit_and_maintain_clearance() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_station());
        }
    }

    #[test]
    fn critical_workflow_order_is_left_to_right_then_forward_to_priming() {
        assert!(MEDIA_BAG_CENTER.0 < DEGASSING_CENTER.0);
        assert!(DEGASSING_CENTER.0 < TRAP_CENTER.0);
        assert!(TRAP_CENTER.1 > PRIMING_CENTER.1);
        assert!(SENSOR_CENTER.0 < PRESSURE_CENTER.0);
        assert!(QC_CENTER.1 < PRESSURE_CENTER.1);
        assert!(CUSTODY_CENTER.1 > MEDIA_BAG_CENTER.1);
    }

    #[test]
    fn coordinate_helpers_preserve_symmetry() {
        assert_eq!(
            priming_lane_x(0) - PRIMING_CENTER.0,
            -(priming_lane_x(PRIMING_LANES - 1) - PRIMING_CENTER.0)
        );
        assert_eq!(
            trap_y(0) - TRAP_CENTER.1,
            -(trap_y(TRAP_COUNT - 1) - TRAP_CENTER.1)
        );
        let first_coupon = sensor_coupon_xy(0);
        let last_coupon = sensor_coupon_xy(SENSOR_COUPON_COUNT - 1);
        assert_eq!(
            first_coupon.0 - SENSOR_CENTER.0,
            -(last_coupon.0 - SENSOR_CENTER.0)
        );
    }
}
