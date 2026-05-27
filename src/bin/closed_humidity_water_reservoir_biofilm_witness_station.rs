use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed humidity/water-reservoir biofilm and residue witness station.
//
// This standalone CAD generator models validation packaging for the closed
// cell-culture cabinet humidifier/water-reservoir path. It provides a removable
// reservoir surrogate, coupon racks, low-point drain wells, UV/decon exposure
// witness lands, conductivity/turbidity sensor pockets, clean/dirty
// segregation, barcode/certificate lands, release/hold/reject lanes, and raised
// CSG block-letter labels. It does not define microbiology methods, cleaning
// chemistry, action limits, or release criteria.

const OUTPUTS: [&str; 12] = [
    "output/closed_humidity_water_reservoir_biofilm_witness_station_base_containment_deck.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_removable_reservoir_surrogate.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_coupon_rack_carriers.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_drain_low_point_wells.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_uv_decon_exposure_witness_lands.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_conductivity_turbidity_sensor_pockets.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_clean_dirty_segregation_bulkhead.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_barcode_certificate_lands.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_release_hold_reject_lanes.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_evidence_camera_bridge.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_humidity_water_reservoir_biofilm_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "removable_reservoir_surrogate",
    "coupon_rack_carriers",
    "drain_low_point_wells",
    "uv_decon_exposure_witness_lands",
    "conductivity_turbidity_sensor_pockets",
    "clean_dirty_segregation_bulkhead",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "csg_geometry_labels",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const CSG_LABELS: [&str; 17] = [
    "RESERVOIR",
    "BIOFILM",
    "COUPON",
    "RACK",
    "DRAIN",
    "LOW",
    "UV",
    "DECON",
    "COND",
    "TURB",
    "CLEAN",
    "DIRTY",
    "BARCODE",
    "CERT",
    "RELEASE",
    "HOLD",
    "REJECT",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 820.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RESERVOIR_X: f64 = 440.0;
const RESERVOIR_Y: f64 = 270.0;
const RESERVOIR_Z: f64 = 66.0;
const RESERVOIR_POS: (f64, f64) = (-350.0, 190.0);
const RESERVOIR_CAVITY_X: f64 = 326.0;
const RESERVOIR_CAVITY_Y: f64 = 166.0;
const RESERVOIR_CAVITY_DEPTH: f64 = 42.0;
const RESERVOIR_BAFFLE_COUNT: usize = 4;
const RESERVOIR_LIFT_HANDLE_COUNT: usize = 2;
const RESERVOIR_SAMPLE_PORTS: usize = 4;
const RESERVOIR_WORKING_VOLUME_ML: f64 = 1_720.0;

const RACK_X: f64 = 420.0;
const RACK_Y: f64 = 270.0;
const RACK_Z: f64 = 52.0;
const RACK_POS: (f64, f64) = (260.0, 190.0);
const COUPON_RACK_COUNT: usize = 3;
const COUPONS_PER_RACK: usize = 8;
const BIOFILM_COUPON_COUNT: usize = COUPON_RACK_COUNT * COUPONS_PER_RACK;
const COUPON_SLOT_X: f64 = 50.0;
const COUPON_SLOT_Y: f64 = 22.0;
const COUPON_SLOT_Z: f64 = 20.0;
const RACK_PITCH_X: f64 = 126.0;
const COUPON_PITCH_Y: f64 = 28.0;

const DRAIN_X: f64 = 300.0;
const DRAIN_Y: f64 = 180.0;
const DRAIN_Z: f64 = 42.0;
const DRAIN_POS: (f64, f64) = (-430.0, -110.0);
const LOW_POINT_WELL_COUNT: usize = 6;
const DRAIN_WELL_D: f64 = 36.0;
const DRAIN_WELL_DEPTH: f64 = 24.0;
const DRAIN_CHANNEL_COUNT: usize = LOW_POINT_WELL_COUNT - 1;
const DRAIN_STANDPIPE_COUNT: usize = 3;
const LOW_POINT_HOLDUP_ML: f64 = 42.0;

const UV_X: f64 = 360.0;
const UV_Y: f64 = 180.0;
const UV_Z: f64 = 36.0;
const UV_POS: (f64, f64) = (-30.0, -110.0);
const UV_WITNESS_LAND_COUNT: usize = 12;
const UV_SHADOW_MASK_COUNT: usize = 6;
const DECON_CARD_LANDS: usize = 4;

const SENSOR_X: f64 = 300.0;
const SENSOR_Y: f64 = 180.0;
const SENSOR_Z: f64 = 46.0;
const SENSOR_POS: (f64, f64) = (390.0, -110.0);
const SENSOR_POCKET_COUNT: usize = 4;
const SENSOR_FLOW_CELL_COUNT: usize = 2;
const SENSOR_CABLE_CLIP_COUNT: usize = 4;

const SEGREGATION_X: f64 = 1120.0;
const SEGREGATION_Y: f64 = 18.0;
const SEGREGATION_Z: f64 = 74.0;
const SEGREGATION_POS: (f64, f64) = (0.0, 32.0);
const SEGREGATION_PASS_GATE_COUNT: usize = 4;

const TRACE_X: f64 = 340.0;
const TRACE_Y: f64 = 130.0;
const TRACE_Z: f64 = 16.0;
const TRACE_POS: (f64, f64) = (-390.0, -310.0);
const BARCODE_LAND_COUNT: usize = 10;
const CERTIFICATE_LAND_COUNT: usize = 4;

const LANE_X: f64 = 460.0;
const LANE_Y: f64 = 130.0;
const LANE_Z: f64 = 34.0;
const LANE_POS: (f64, f64) = (250.0, -310.0);
const DISPOSITION_LANE_COUNT: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 5;
const DISPOSITION_TOKEN_COUNT: usize = DISPOSITION_LANE_COUNT * DISPOSITION_SLOTS_PER_LANE;
const DISPOSITION_NAMES: [&str; DISPOSITION_LANE_COUNT] = ["RELEASE", "HOLD", "REJECT"];

const CAMERA_BRIDGE_X: f64 = 1120.0;
const CAMERA_BRIDGE_Y: f64 = 50.0;
const CAMERA_BRIDGE_Z: f64 = 190.0;
const CAMERA_POS: (f64, f64) = (0.0, 350.0);
const CAMERA_POD_COUNT: usize = 5;
const EVIDENCE_LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1260.0;
const KEEP_OUT_Y: f64 = 760.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE_Y: f64 = 240.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 190.0;
const LEFT_RESERVOIR_SERVICE_X: f64 = 220.0;
const RIGHT_SENSOR_SERVICE_X: f64 = 180.0;
const TOP_RESERVOIR_LIFT_CLEARANCE_Z: f64 = 290.0;

const LABEL_Z: f64 = 2.4;

#[derive(Clone, Copy)]
struct ModuleEnvelope {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl ModuleEnvelope {
    fn fits_on_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Self) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let reservoir = removable_reservoir_surrogate();
    export(OUTPUTS[1], &reservoir);

    let racks = coupon_rack_carriers();
    export(OUTPUTS[2], &racks);

    let drains = drain_low_point_wells();
    export(OUTPUTS[3], &drains);

    let uv = uv_decon_exposure_witness_lands();
    export(OUTPUTS[4], &uv);

    let sensors = conductivity_turbidity_sensor_pockets();
    export(OUTPUTS[5], &sensors);

    let segregation = clean_dirty_segregation_bulkhead();
    export(OUTPUTS[6], &segregation);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + reservoir.translate(
            RESERVOIR_POS.0,
            RESERVOIR_POS.1,
            module_on_deck_z(RESERVOIR_Z),
        ) + racks.translate(RACK_POS.0, RACK_POS.1, module_on_deck_z(RACK_Z))
            + drains.translate(DRAIN_POS.0, DRAIN_POS.1, module_on_deck_z(DRAIN_Z))
            + uv.translate(UV_POS.0, UV_POS.1, module_on_deck_z(UV_Z))
            + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, module_on_deck_z(SENSOR_Z))
            + segregation.translate(
                SEGREGATION_POS.0,
                SEGREGATION_POS.1,
                module_on_deck_z(SEGREGATION_Z),
            )
            + traceability.translate(TRACE_POS.0, TRACE_POS.1, module_on_deck_z(TRACE_Z))
            + lanes.translate(LANE_POS.0, LANE_POS.1, module_on_deck_z(LANE_Z))
            + camera.translate(
                CAMERA_POS.0,
                CAMERA_POS.1,
                module_on_deck_z(CAMERA_BRIDGE_Z),
            )
            + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed humidity/water-reservoir biofilm witness station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Reservoir surrogate:   removable {RESERVOIR_WORKING_VOLUME_ML:.0}mL pan with {RESERVOIR_BAFFLE_COUNT} baffles, {RESERVOIR_SAMPLE_PORTS} sample ports, and drain/overflow keys"
    );
    println!(
        "  Biofilm coupons:       {BIOFILM_COUPON_COUNT} coupon slots across {COUPON_RACK_COUNT} removable racks plus indexed witness tabs"
    );
    println!(
        "  Drain controls:        {LOW_POINT_WELL_COUNT} low-point wells, {DRAIN_CHANNEL_COUNT} linked drain channels, and {DRAIN_STANDPIPE_COUNT} removable standpipe witnesses"
    );
    println!(
        "  Exposure witnesses:    {UV_WITNESS_LAND_COUNT} UV/decon witness lands, {UV_SHADOW_MASK_COUNT} shadow-mask blocks, and {DECON_CARD_LANDS} decon indicator lands"
    );
    println!(
        "  Sensor pockets:        conductivity/turbidity pockets with {SENSOR_FLOW_CELL_COUNT} flow cells and {SENSOR_CABLE_CLIP_COUNT} cable clips"
    );
    println!(
        "  Trace/disposition:     {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, {DISPOSITION_TOKEN_COUNT} release/hold/reject token recesses"
    );
    println!(
        "  Labels:                {} raised CSG block-letter labels embedded in exported geometry",
        CSG_LABELS.len()
    );
    println!(
        "  Keepouts:              robot front {ROBOT_FRONT_CLEARANCE_Y:.0}mm, rear service {SERVICE_REAR_CLEARANCE_Y:.0}mm, top reservoir lift {TOP_RESERVOIR_LIFT_CLEARANCE_Z:.0}mm Z"
    );
    println!("  Limitation:            Mechanical witness-control station CAD only; validation methods and acceptance criteria remain external.");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn module_on_deck_z(module_z: f64) -> f64 {
    DECK_Z / 2.0 + module_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(CSG_LABELS.len(), 17);
    assert_eq!(BIOFILM_COUPON_COUNT, 24);
    assert_eq!(LOW_POINT_WELL_COUNT, 6);
    assert_eq!(DRAIN_CHANNEL_COUNT, 5);
    assert_eq!(SENSOR_POCKET_COUNT, 4);
    assert_eq!(DISPOSITION_TOKEN_COUNT, 15);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert!(RESERVOIR_WORKING_VOLUME_ML > LOW_POINT_HOLDUP_ML * LOW_POINT_WELL_COUNT as f64);
    assert!(front_robot_keepout_y() >= ROBOT_FRONT_CLEARANCE_Y);
    assert!(rear_service_keepout_y() >= SERVICE_REAR_CLEARANCE_Y);
    assert!(left_reservoir_service_x() >= LEFT_RESERVOIR_SERVICE_X);
    assert!(right_sensor_service_x() >= RIGHT_SENSOR_SERVICE_X);

    let modules = layout_modules();
    for module in modules {
        assert!(module.fits_on_deck(), "{} exceeds usable deck", module.name);
    }
    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            assert!(
                !modules[i].overlaps(modules[j]),
                "{} overlaps {}",
                modules[i].name,
                modules[j].name
            );
        }
    }

    for label in [
        "RELEASE", "HOLD", "REJECT", "CLEAN", "DIRTY", "COND", "TURB",
    ] {
        assert!(CSG_LABELS.contains(&label));
    }
}

fn layout_modules() -> [ModuleEnvelope; 8] {
    [
        ModuleEnvelope {
            name: "removable_reservoir_surrogate",
            center: RESERVOIR_POS,
            x: RESERVOIR_X,
            y: RESERVOIR_Y,
        },
        ModuleEnvelope {
            name: "coupon_rack_carriers",
            center: RACK_POS,
            x: RACK_X,
            y: RACK_Y,
        },
        ModuleEnvelope {
            name: "drain_low_point_wells",
            center: DRAIN_POS,
            x: DRAIN_X,
            y: DRAIN_Y,
        },
        ModuleEnvelope {
            name: "uv_decon_exposure_witness_lands",
            center: UV_POS,
            x: UV_X,
            y: UV_Y,
        },
        ModuleEnvelope {
            name: "conductivity_turbidity_sensor_pockets",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        ModuleEnvelope {
            name: "clean_dirty_segregation_bulkhead",
            center: SEGREGATION_POS,
            x: SEGREGATION_X,
            y: SEGREGATION_Y,
        },
        ModuleEnvelope {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        ModuleEnvelope {
            name: "release_hold_reject_lanes",
            center: LANE_POS,
            x: LANE_X,
            y: LANE_Y,
        },
    ]
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "humidity_water_biofilm_station_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin_cut = centered_cube(
        "humidity_water_biofilm_station_shallow_basin_cut",
        STATION_X - 128.0,
        STATION_Y - 110.0,
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -2.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.3);
    let drain_slot = centered_cube(
        "humidity_water_biofilm_station_front_drain_slot_cut",
        82.0,
        12.0,
        DECK_Z + 3.0,
    )
    .translate(
        -STATION_X / 2.0 + 150.0,
        -STATION_Y / 2.0 + 11.0,
        DECK_Z / 2.0,
    );

    deck - basin_cut - drain_slot - base_locator_sockets() - base_mount_slots()
        + containment_rims()
        + deck_zone_ribs()
        + datum_targets()
        + deck_flow_grade_witness_bars()
}

fn base_locator_sockets() -> Part {
    let mut sockets = Part::empty("humidity_water_biofilm_station_module_locator_sockets");
    for module in layout_modules() {
        sockets = sockets
            + centered_cube(
                format!(
                    "humidity_water_biofilm_station_{}_locator_socket",
                    module.name
                ),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("humidity_water_biofilm_station_mount_slots");
    for (i, (x, y)) in mount_slot_positions().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("humidity_water_biofilm_station_m6_mount_clearance_{i}"),
                6.8 / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0)
            + centered_cube(
                format!("humidity_water_biofilm_station_mount_slot_relief_{i}"),
                30.0,
                8.0,
                DECK_Z + 4.0,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-220.0, STATION_Y / 2.0 - 58.0),
        (220.0, STATION_Y / 2.0 - 58.0),
        (-220.0, -(STATION_Y / 2.0 - 58.0)),
        (220.0, -(STATION_Y / 2.0 - 58.0)),
    ]
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "humidity_water_biofilm_station_left_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "humidity_water_biofilm_station_right_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "humidity_water_biofilm_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "humidity_water_biofilm_station_front_low_robot_lip",
        STATION_X - 180.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, DECK_Z / 2.0 + 11.0);

    left + right + rear + front
}

fn deck_zone_ribs() -> Part {
    let clean_dirty = centered_cube(
        "humidity_water_biofilm_station_clean_dirty_zone_rib",
        STATION_X - 180.0,
        6.0,
        18.0,
    )
    .translate(0.0, 4.0, DECK_Z / 2.0 + 9.0);
    let trace_row = centered_cube(
        "humidity_water_biofilm_station_traceability_row_rib",
        STATION_X - 220.0,
        6.0,
        16.0,
    )
    .translate(0.0, -228.0, DECK_Z / 2.0 + 8.0);

    clean_dirty + trace_row
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("humidity_water_biofilm_station_datum_targets");
    for (i, (x, y)) in [
        (-560.0, 342.0),
        (560.0, 342.0),
        (-560.0, -340.0),
        (560.0, -340.0),
    ]
    .iter()
    .enumerate()
    {
        targets =
            targets
                + fiducial_disc(format!("humidity_water_biofilm_station_datum_target_{i}"))
                    .translate(*x, *y, DECK_Z + 2.0);
    }
    targets
}

fn deck_flow_grade_witness_bars() -> Part {
    let mut bars = Part::empty("humidity_water_biofilm_station_flow_grade_witness_bars");
    for (i, y) in [-178.0, -118.0, -58.0, 72.0, 138.0, 204.0, 270.0]
        .iter()
        .enumerate()
    {
        bars = bars
            + centered_cube(
                format!("humidity_water_biofilm_station_deck_slope_witness_bar_{i}"),
                STATION_X - 240.0,
                3.0,
                4.0,
            )
            .translate(0.0, *y, DECK_Z + 2.0);
    }
    bars
}

fn removable_reservoir_surrogate() -> Part {
    let shell = centered_cube(
        "humidity_water_biofilm_reservoir_surrogate_shell",
        RESERVOIR_X,
        RESERVOIR_Y,
        RESERVOIR_Z,
    );
    let cavity = centered_cube(
        "humidity_water_biofilm_reservoir_surrogate_water_cavity_cut",
        RESERVOIR_CAVITY_X,
        RESERVOIR_CAVITY_Y,
        RESERVOIR_CAVITY_DEPTH + 0.8,
    )
    .translate(
        0.0,
        14.0,
        RESERVOIR_Z / 2.0 - RESERVOIR_CAVITY_DEPTH / 2.0 + 0.3,
    );
    let overflow_slot = centered_cube(
        "humidity_water_biofilm_reservoir_overflow_slot_cut",
        74.0,
        10.0,
        24.0,
    )
    .translate(
        RESERVOIR_X / 2.0 - 32.0,
        -RESERVOIR_Y / 2.0 + 48.0,
        RESERVOIR_Z / 2.0 - 16.0,
    );
    let drain_bore = centered_cylinder(
        "humidity_water_biofilm_reservoir_drain_barb_bore",
        12.0 / 2.0,
        60.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -RESERVOIR_X / 2.0 + 22.0,
        -RESERVOIR_Y / 2.0 + 42.0,
        -RESERVOIR_Z / 2.0 + 18.0,
    );

    shell - cavity - overflow_slot - drain_bore
        + reservoir_baffles()
        + reservoir_lift_handles()
        + reservoir_sample_port_lands()
        + reservoir_biofilm_witness_terraces()
        + raised_label("reservoir_surrogate_reservoir_label", "RESERVOIR", 3.0).translate(
            -72.0,
            -RESERVOIR_Y / 2.0 + 24.0,
            RESERVOIR_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("reservoir_surrogate_biofilm_label", "BIOFILM", 2.8).translate(
            92.0,
            RESERVOIR_Y / 2.0 - 30.0,
            RESERVOIR_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn reservoir_baffles() -> Part {
    let mut baffles = Part::empty("humidity_water_biofilm_reservoir_baffles");
    for i in 0..RESERVOIR_BAFFLE_COUNT {
        let x = centered_index(i, RESERVOIR_BAFFLE_COUNT, 68.0);
        baffles = baffles
            + centered_cube(
                format!("humidity_water_biofilm_reservoir_removable_baffle_{i}"),
                7.0,
                RESERVOIR_CAVITY_Y - 24.0,
                28.0,
            )
            .translate(x, 14.0, RESERVOIR_Z / 2.0 - RESERVOIR_CAVITY_DEPTH + 18.0)
            + centered_cube(
                format!("humidity_water_biofilm_reservoir_baffle_notch_gauge_{i}"),
                24.0,
                8.0,
                7.0,
            )
            .translate(x, -RESERVOIR_Y / 2.0 + 50.0, RESERVOIR_Z / 2.0 + 3.5);
    }
    baffles
}

fn reservoir_lift_handles() -> Part {
    let mut handles = Part::empty("humidity_water_biofilm_reservoir_lift_handles");
    for i in 0..RESERVOIR_LIFT_HANDLE_COUNT {
        let x = centered_index(i, RESERVOIR_LIFT_HANDLE_COUNT, RESERVOIR_X - 74.0);
        handles = handles
            + centered_cube(
                format!("humidity_water_biofilm_reservoir_lift_handle_{i}_bridge"),
                58.0,
                16.0,
                20.0,
            )
            .translate(x, -RESERVOIR_Y / 2.0 + 28.0, RESERVOIR_Z / 2.0 + 10.0)
            - centered_cube(
                format!("humidity_water_biofilm_reservoir_lift_handle_{i}_finger_gap"),
                38.0,
                18.0,
                12.0,
            )
            .translate(x, -RESERVOIR_Y / 2.0 + 28.0, RESERVOIR_Z / 2.0 + 10.0);
    }
    handles
}

fn reservoir_sample_port_lands() -> Part {
    let mut ports = Part::empty("humidity_water_biofilm_reservoir_sample_port_lands");
    for i in 0..RESERVOIR_SAMPLE_PORTS {
        let x = centered_index(i, RESERVOIR_SAMPLE_PORTS, 72.0);
        ports = ports
            + centered_cylinder(
                format!("humidity_water_biofilm_reservoir_sample_port_land_{i}"),
                18.0,
                5.0,
                36,
            )
            .translate(x, RESERVOIR_Y / 2.0 - 36.0, RESERVOIR_Z / 2.0 + 2.5)
            - centered_cylinder(
                format!("humidity_water_biofilm_reservoir_sample_port_bore_{i}"),
                6.5,
                7.0,
                24,
            )
            .translate(x, RESERVOIR_Y / 2.0 - 36.0, RESERVOIR_Z / 2.0 + 2.5);
    }
    ports
}

fn reservoir_biofilm_witness_terraces() -> Part {
    let mut terraces = Part::empty("humidity_water_biofilm_reservoir_witness_terraces");
    for i in 0..5 {
        terraces = terraces
            + centered_cube(
                format!("humidity_water_biofilm_reservoir_residue_step_{i}"),
                52.0,
                10.0,
                4.0 + i as f64,
            )
            .translate(
                centered_index(i, 5, 58.0),
                -RESERVOIR_Y / 2.0 + 78.0,
                RESERVOIR_Z / 2.0 - 32.0 + i as f64 * 4.0,
            );
    }
    terraces
}

fn coupon_rack_carriers() -> Part {
    let base = centered_cube(
        "humidity_water_biofilm_coupon_rack_base",
        RACK_X,
        RACK_Y,
        16.0,
    )
    .translate(0.0, 0.0, -RACK_Z / 2.0 + 8.0);
    let rear_rail = centered_cube(
        "humidity_water_biofilm_coupon_rack_rear_stop_rail",
        RACK_X - 26.0,
        12.0,
        26.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 18.0, -RACK_Z / 2.0 + 29.0);
    let front_rail = centered_cube(
        "humidity_water_biofilm_coupon_rack_front_stop_rail",
        RACK_X - 26.0,
        12.0,
        24.0,
    )
    .translate(0.0, -RACK_Y / 2.0 + 18.0, -RACK_Z / 2.0 + 28.0);

    let mut racks = Part::empty("humidity_water_biofilm_coupon_rack_carriers");
    let mut slot_cuts = Part::empty("humidity_water_biofilm_coupon_slot_cuts");
    let mut witness_tabs = Part::empty("humidity_water_biofilm_coupon_serial_witness_tabs");
    for rack in 0..COUPON_RACK_COUNT {
        let x = centered_index(rack, COUPON_RACK_COUNT, RACK_PITCH_X);
        racks = racks
            + centered_cube(
                format!("humidity_water_biofilm_coupon_rack_body_{rack}"),
                88.0,
                226.0,
                34.0,
            )
            .translate(x, 0.0, -RACK_Z / 2.0 + 16.0 + 17.0)
            + centered_cube(
                format!("humidity_water_biofilm_coupon_rack_robot_grip_{rack}"),
                74.0,
                10.0,
                12.0,
            )
            .translate(x, RACK_Y / 2.0 - 38.0, RACK_Z / 2.0 - 6.0);

        for coupon in 0..COUPONS_PER_RACK {
            let y = centered_index(coupon, COUPONS_PER_RACK, COUPON_PITCH_Y);
            let index = rack * COUPONS_PER_RACK + coupon;
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("humidity_water_biofilm_coupon_slot_cut_{index}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_Z,
                )
                .translate(x, y, RACK_Z / 2.0 - COUPON_SLOT_Z / 2.0 + 0.5);
            witness_tabs = witness_tabs
                + centered_cube(
                    format!("humidity_water_biofilm_coupon_trace_tab_{index}"),
                    42.0,
                    6.0,
                    3.0,
                )
                .translate(x, y + 12.0, RACK_Z / 2.0 + 1.5);
        }
    }

    base + rear_rail + front_rail + racks - slot_cuts
        + witness_tabs
        + rack_fiducials()
        + raised_label("coupon_rack_coupon_label", "COUPON", 3.0).translate(
            -88.0,
            -RACK_Y / 2.0 + 24.0,
            RACK_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("coupon_rack_rack_label", "RACK", 3.0).translate(
            112.0,
            -RACK_Y / 2.0 + 24.0,
            RACK_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn rack_fiducials() -> Part {
    fiducial_disc("humidity_water_biofilm_coupon_rack_left_fiducial").translate(
        -RACK_X / 2.0 + 26.0,
        RACK_Y / 2.0 - 30.0,
        RACK_Z / 2.0 + 2.0,
    ) + fiducial_disc("humidity_water_biofilm_coupon_rack_right_fiducial").translate(
        RACK_X / 2.0 - 26.0,
        RACK_Y / 2.0 - 30.0,
        RACK_Z / 2.0 + 2.0,
    )
}

fn drain_low_point_wells() -> Part {
    let plate = centered_cube(
        "humidity_water_biofilm_drain_low_point_plate",
        DRAIN_X,
        DRAIN_Y,
        DRAIN_Z,
    );
    let mut cuts = Part::empty("humidity_water_biofilm_low_point_well_cuts");
    let mut rings = Part::empty("humidity_water_biofilm_low_point_well_witness_rings");
    for i in 0..LOW_POINT_WELL_COUNT {
        let col = i % 3;
        let row = i / 3;
        let x = centered_index(col, 3, 82.0);
        let y = centered_index(row, 2, 68.0) + 10.0;
        cuts = cuts
            + centered_cylinder(
                format!("humidity_water_biofilm_low_point_well_cut_{i}"),
                DRAIN_WELL_D / 2.0,
                DRAIN_WELL_DEPTH + 0.8,
                44,
            )
            .translate(x, y, DRAIN_Z / 2.0 - DRAIN_WELL_DEPTH / 2.0 + 0.3);
        rings = rings
            + centered_cylinder(
                format!("humidity_water_biofilm_low_point_well_ring_outer_{i}"),
                DRAIN_WELL_D / 2.0 + 5.0,
                4.0,
                44,
            )
            .translate(x, y, DRAIN_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("humidity_water_biofilm_low_point_well_ring_inner_{i}"),
                DRAIN_WELL_D / 2.0 + 0.8,
                5.0,
                44,
            )
            .translate(x, y, DRAIN_Z / 2.0 + 2.0);
    }

    plate - cuts - drain_channel_cuts()
        + rings
        + drain_standpipe_witnesses()
        + raised_label("drain_low_point_drain_label", "DRAIN", 3.0).translate(
            -70.0,
            -DRAIN_Y / 2.0 + 24.0,
            DRAIN_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("drain_low_point_low_label", "LOW", 3.0).translate(
            82.0,
            -DRAIN_Y / 2.0 + 24.0,
            DRAIN_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn drain_channel_cuts() -> Part {
    let mut channels = Part::empty("humidity_water_biofilm_low_point_drain_channel_cuts");
    for i in 0..DRAIN_CHANNEL_COUNT {
        let x = centered_index(i, DRAIN_CHANNEL_COUNT, 48.0);
        channels = channels
            + centered_cube(
                format!("humidity_water_biofilm_low_point_drain_channel_{i}"),
                38.0,
                12.0,
                11.0,
            )
            .translate(x, 10.0, DRAIN_Z / 2.0 - 8.0);
    }
    channels
}

fn drain_standpipe_witnesses() -> Part {
    let mut posts = Part::empty("humidity_water_biofilm_drain_standpipe_witnesses");
    for i in 0..DRAIN_STANDPIPE_COUNT {
        posts = posts
            + centered_cylinder(
                format!("humidity_water_biofilm_removable_standpipe_witness_{i}"),
                8.0,
                34.0,
                28,
            )
            .translate(
                centered_index(i, DRAIN_STANDPIPE_COUNT, 72.0),
                DRAIN_Y / 2.0 - 30.0,
                DRAIN_Z / 2.0 + 17.0,
            );
    }
    posts
}

fn uv_decon_exposure_witness_lands() -> Part {
    let plate = centered_cube(
        "humidity_water_biofilm_uv_decon_witness_plate",
        UV_X,
        UV_Y,
        UV_Z,
    );
    let mut witness_lands = Part::empty("humidity_water_biofilm_uv_decon_witness_lands");
    for i in 0..UV_WITNESS_LAND_COUNT {
        let row = i / 4;
        let col = i % 4;
        let x = centered_index(col, 4, 72.0);
        let y = 34.0 - row as f64 * 44.0;
        witness_lands = witness_lands
            + centered_cube(
                format!("humidity_water_biofilm_uv_decon_coupon_land_{i}"),
                50.0,
                28.0,
                4.0,
            )
            .translate(x, y, UV_Z / 2.0 + 2.0)
            + centered_cube(
                format!("humidity_water_biofilm_uv_decon_index_tick_{i}"),
                6.0,
                20.0,
                5.0,
            )
            .translate(x - 32.0, y, UV_Z / 2.0 + 2.5);
    }

    plate
        + witness_lands
        + uv_shadow_mask_blocks()
        + decon_indicator_card_lands()
        + raised_label("uv_decon_uv_label", "UV", 3.4).translate(
            -120.0,
            -UV_Y / 2.0 + 24.0,
            UV_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("uv_decon_decon_label", "DECON", 2.8).translate(
            92.0,
            -UV_Y / 2.0 + 24.0,
            UV_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn uv_shadow_mask_blocks() -> Part {
    let mut blocks = Part::empty("humidity_water_biofilm_uv_shadow_mask_blocks");
    for i in 0..UV_SHADOW_MASK_COUNT {
        let x = centered_index(i, UV_SHADOW_MASK_COUNT, 44.0);
        blocks = blocks
            + centered_cube(
                format!("humidity_water_biofilm_uv_shadow_mask_step_{i}"),
                26.0,
                14.0,
                8.0 + i as f64 * 2.5,
            )
            .translate(x, UV_Y / 2.0 - 24.0, UV_Z / 2.0 + 4.0 + i as f64 * 1.25);
    }
    blocks
}

fn decon_indicator_card_lands() -> Part {
    let mut lands = Part::empty("humidity_water_biofilm_decon_indicator_card_lands");
    for i in 0..DECON_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("humidity_water_biofilm_decon_indicator_card_land_{i}"),
                52.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(i, DECON_CARD_LANDS, 68.0),
                0.0,
                UV_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn conductivity_turbidity_sensor_pockets() -> Part {
    let plate = centered_cube(
        "humidity_water_biofilm_sensor_pocket_plate",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let conductivity_pocket = sensor_pocket_cut(
        "humidity_water_biofilm_conductivity_probe_pocket_cut",
        (-70.0, 26.0),
        86.0,
        38.0,
        24.0,
    );
    let turbidity_pocket = sensor_pocket_cut(
        "humidity_water_biofilm_turbidity_flow_cell_pocket_cut",
        (72.0, 26.0),
        86.0,
        38.0,
        24.0,
    );

    plate - conductivity_pocket - turbidity_pocket - sensor_flow_cell_cuts()
        + sensor_cable_clips()
        + sensor_reference_bubbles()
        + raised_label("sensor_pockets_cond_label", "COND", 3.0).translate(
            -76.0,
            -SENSOR_Y / 2.0 + 24.0,
            SENSOR_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("sensor_pockets_turb_label", "TURB", 3.0).translate(
            76.0,
            -SENSOR_Y / 2.0 + 24.0,
            SENSOR_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn sensor_pocket_cut(
    name: impl Into<String>,
    center: (f64, f64),
    x: f64,
    y: f64,
    depth: f64,
) -> Part {
    centered_cube(name, x, y, depth + 0.6).translate(
        center.0,
        center.1,
        SENSOR_Z / 2.0 - depth / 2.0 + 0.3,
    )
}

fn sensor_flow_cell_cuts() -> Part {
    let mut cuts = Part::empty("humidity_water_biofilm_sensor_flow_cell_cuts");
    for i in 0..SENSOR_FLOW_CELL_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("humidity_water_biofilm_sensor_flow_cell_bore_{i}"),
                6.0,
                220.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 26.0 - i as f64 * 52.0, SENSOR_Z / 2.0 - 18.0);
    }
    cuts
}

fn sensor_cable_clips() -> Part {
    let mut clips = Part::empty("humidity_water_biofilm_sensor_cable_clips");
    for i in 0..SENSOR_CABLE_CLIP_COUNT {
        clips = clips
            + centered_cube(
                format!("humidity_water_biofilm_sensor_cable_clip_{i}"),
                40.0,
                8.0,
                12.0,
            )
            .translate(
                centered_index(i, SENSOR_CABLE_CLIP_COUNT, 64.0),
                -34.0,
                SENSOR_Z / 2.0 + 6.0,
            )
            - centered_cube(
                format!("humidity_water_biofilm_sensor_cable_clip_gap_{i}"),
                24.0,
                10.0,
                6.0,
            )
            .translate(
                centered_index(i, SENSOR_CABLE_CLIP_COUNT, 64.0),
                -34.0,
                SENSOR_Z / 2.0 + 8.0,
            );
    }
    clips
}

fn sensor_reference_bubbles() -> Part {
    let conductivity = centered_cylinder(
        "humidity_water_biofilm_conductivity_reference_land",
        17.0,
        4.0,
        36,
    )
    .translate(-70.0, SENSOR_Y / 2.0 - 30.0, SENSOR_Z / 2.0 + 2.0);
    let turbidity = centered_cylinder(
        "humidity_water_biofilm_turbidity_reference_land",
        17.0,
        4.0,
        36,
    )
    .translate(72.0, SENSOR_Y / 2.0 - 30.0, SENSOR_Z / 2.0 + 2.0);
    conductivity + turbidity
}

fn clean_dirty_segregation_bulkhead() -> Part {
    let wall = centered_cube(
        "humidity_water_biofilm_clean_dirty_bulkhead_wall",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let drip_skirt = centered_cube(
        "humidity_water_biofilm_dirty_side_drip_skirt",
        SEGREGATION_X - 90.0,
        16.0,
        28.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 - 9.0, -SEGREGATION_Z / 2.0 + 18.0);
    let clean_positive_stop = centered_cube(
        "humidity_water_biofilm_clean_side_positive_stop",
        SEGREGATION_X - 120.0,
        10.0,
        22.0,
    )
    .translate(0.0, SEGREGATION_Y / 2.0 + 8.0, -SEGREGATION_Z / 2.0 + 34.0);

    wall - segregation_pass_gates()
        + drip_skirt
        + clean_positive_stop
        + raised_label("clean_dirty_bulkhead_clean_label", "CLEAN", 3.0).translate(
            -380.0,
            SEGREGATION_Y / 2.0 + 12.0,
            SEGREGATION_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("clean_dirty_bulkhead_dirty_label", "DIRTY", 3.0).translate(
            380.0,
            -SEGREGATION_Y / 2.0 - 12.0,
            SEGREGATION_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn segregation_pass_gates() -> Part {
    let mut gates = Part::empty("humidity_water_biofilm_clean_dirty_pass_gate_cuts");
    for i in 0..SEGREGATION_PASS_GATE_COUNT {
        gates = gates
            + centered_cube(
                format!("humidity_water_biofilm_clean_dirty_controlled_pass_gate_{i}"),
                92.0,
                SEGREGATION_Y + 4.0,
                34.0,
            )
            .translate(
                centered_index(i, SEGREGATION_PASS_GATE_COUNT, 230.0),
                0.0,
                -SEGREGATION_Z / 2.0 + 34.0,
            );
    }
    gates
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "humidity_water_biofilm_traceability_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("humidity_water_biofilm_barcode_certificate_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 5;
        let col = i % 5;
        let x = centered_index(col, 5, 58.0);
        let y = 26.0 - row as f64 * 32.0;
        lands = lands
            + centered_cube(
                format!("humidity_water_biofilm_barcode_land_recess_floor_{i}"),
                46.0,
                18.0,
                2.5,
            )
            .translate(x, y, TRACE_Z / 2.0 + 1.25)
            + barcode_stripes(i, x, y);
    }
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_water_biofilm_certificate_land_{i}"),
                70.0,
                28.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 78.0),
                -42.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }

    panel
        + lands
        + raised_label("traceability_barcode_label", "BARCODE", 2.2).translate(
            -90.0,
            TRACE_Y / 2.0 - 16.0,
            TRACE_Z / 2.0 + LABEL_Z / 2.0,
        )
        + raised_label("traceability_cert_label", "CERT", 2.4).translate(
            116.0,
            TRACE_Y / 2.0 - 16.0,
            TRACE_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn barcode_stripes(index: usize, x: f64, y: f64) -> Part {
    let mut stripes = Part::empty(format!("humidity_water_biofilm_barcode_stripes_{index}"));
    for stripe in 0..6 {
        let stripe_w = if (stripe + index) % 2 == 0 { 2.0 } else { 4.0 };
        stripes = stripes
            + centered_cube(
                format!("humidity_water_biofilm_barcode_{index}_stripe_{stripe}"),
                stripe_w,
                14.0,
                2.0,
            )
            .translate(x - 16.0 + stripe as f64 * 6.0, y, TRACE_Z / 2.0 + 2.5);
    }
    stripes
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "humidity_water_biofilm_release_hold_reject_panel",
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    let mut cuts = Part::empty("humidity_water_biofilm_disposition_lane_token_cuts");
    let mut dividers = Part::empty("humidity_water_biofilm_disposition_lane_dividers");
    let mut labels = Part::empty("humidity_water_biofilm_disposition_csg_labels");

    for (lane, label) in DISPOSITION_NAMES.iter().enumerate() {
        let y = centered_index(lane, DISPOSITION_LANE_COUNT, 38.0);
        dividers = dividers
            + centered_cube(
                format!(
                    "humidity_water_biofilm_{}_lane_back_stop",
                    label.to_lowercase()
                ),
                LANE_X - 32.0,
                5.0,
                12.0,
            )
            .translate(0.0, y + 18.0, LANE_Z / 2.0 + 6.0);
        labels = labels
            + raised_label(
                format!(
                    "humidity_water_biofilm_{}_raised_label",
                    label.to_lowercase()
                ),
                label,
                if *label == "RELEASE" { 2.15 } else { 2.55 },
            )
            .translate(-LANE_X / 2.0 + 68.0, y - 3.0, LANE_Z / 2.0 + LABEL_Z / 2.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "humidity_water_biofilm_{}_token_recess_{slot}",
                        label.to_lowercase()
                    ),
                    48.0,
                    22.0,
                    10.0,
                )
                .translate(-78.0 + slot as f64 * 58.0, y, LANE_Z / 2.0 - 5.0 + 0.3);
        }
    }

    panel - cuts + dividers + labels
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "humidity_water_biofilm_evidence_bridge_left_post",
        32.0,
        32.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + 32.0,
        -CAMERA_BRIDGE_Y / 2.0 + 10.0,
        0.0,
    );
    let right_post = centered_cube(
        "humidity_water_biofilm_evidence_bridge_right_post",
        32.0,
        32.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0 - 32.0,
        -CAMERA_BRIDGE_Y / 2.0 + 10.0,
        0.0,
    );
    let rear_left_post = centered_cube(
        "humidity_water_biofilm_evidence_bridge_rear_left_post",
        32.0,
        32.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + 32.0,
        CAMERA_BRIDGE_Y / 2.0 - 10.0,
        0.0,
    );
    let rear_right_post = centered_cube(
        "humidity_water_biofilm_evidence_bridge_rear_right_post",
        32.0,
        32.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0 - 32.0,
        CAMERA_BRIDGE_Y / 2.0 - 10.0,
        0.0,
    );
    let beam = centered_cube(
        "humidity_water_biofilm_evidence_bridge_camera_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        26.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 13.0);

    left_post + right_post + rear_left_post + rear_right_post + beam + camera_pods() + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("humidity_water_biofilm_evidence_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let x = centered_index(i, CAMERA_POD_COUNT, 220.0);
        pods = pods
            + centered_cube(
                format!("humidity_water_biofilm_evidence_camera_mount_pod_{i}"),
                74.0,
                36.0,
                10.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 5.0)
            - centered_cylinder(
                format!("humidity_water_biofilm_evidence_camera_mount_bore_{i}"),
                5.0 / 2.0,
                12.0,
                24,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 5.0);
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty("humidity_water_biofilm_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BAR_COUNT {
        let y = centered_index(i, EVIDENCE_LIGHT_BAR_COUNT, 28.0);
        bars = bars
            + centered_cube(
                format!("humidity_water_biofilm_evidence_led_bar_{i}"),
                CAMERA_BRIDGE_X - 180.0,
                8.0,
                10.0,
            )
            .translate(0.0, y, CAMERA_BRIDGE_Z / 2.0 - 42.0);
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = keepout_frame(
        "humidity_water_biofilm_front_robot_reach_keepout",
        KEEP_OUT_X,
        ROBOT_FRONT_CLEARANCE_Y,
        58.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - ROBOT_FRONT_CLEARANCE_Y / 2.0,
        DECK_Z + 29.0,
    );
    let rear_service = keepout_frame(
        "humidity_water_biofilm_rear_service_sweep_keepout",
        KEEP_OUT_X,
        SERVICE_REAR_CLEARANCE_Y,
        52.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + SERVICE_REAR_CLEARANCE_Y / 2.0,
        DECK_Z + 26.0,
    );
    let left_service = keepout_frame(
        "humidity_water_biofilm_left_reservoir_service_keepout",
        LEFT_RESERVOIR_SERVICE_X,
        KEEP_OUT_Y,
        52.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_RESERVOIR_SERVICE_X / 2.0,
        0.0,
        DECK_Z + 26.0,
    );
    let right_service = keepout_frame(
        "humidity_water_biofilm_right_sensor_service_keepout",
        RIGHT_SENSOR_SERVICE_X,
        KEEP_OUT_Y,
        52.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SENSOR_SERVICE_X / 2.0,
        0.0,
        DECK_Z + 26.0,
    );
    let top_lift = centered_cube(
        "humidity_water_biofilm_reservoir_top_lift_clearance_gauge",
        RESERVOIR_X + 80.0,
        RESERVOIR_Y + 70.0,
        KEEP_OUT_Z,
    )
    .translate(
        RESERVOIR_POS.0,
        RESERVOIR_POS.1,
        DECK_Z + TOP_RESERVOIR_LIFT_CLEARANCE_Z,
    );

    front_robot + rear_service + left_service + right_service + top_lift
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let shell = centered_cube(format!("{name}_shell"), x, y, z);
    let hollow = centered_cube(format!("{name}_hollow"), x - 12.0, y - 12.0, z - 16.0);
    let label_land = centered_cube(format!("{name}_label_land"), x * 0.36, 12.0, KEEP_OUT_Z)
        .translate(0.0, -y / 2.0 + 12.0, z / 2.0 + KEEP_OUT_Z / 2.0);
    shell - hollow + label_land
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    let pad = centered_cylinder(format!("{name}_pad"), 14.0, 4.0, 48);
    let cross_x = centered_cube(format!("{name}_cross_x_cut"), 22.0, 3.0, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y_cut"), 3.0, 22.0, 5.0);
    pad - cross_x - cross_y
}

fn raised_label(name: impl Into<String>, text: &str, scale: f64) -> Part {
    let name = name.into();
    let mut label = Part::empty(format!("{name}_csg_text"));
    let char_count = text.chars().count();
    for (char_index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            continue;
        }
        let char_center_x = (char_index as f64 - (char_count as f64 - 1.0) / 2.0) * scale * 6.0;
        for (row, pattern) in glyph_rows(ch).iter().enumerate() {
            for (col, byte) in pattern.as_bytes().iter().enumerate() {
                if *byte != b' ' {
                    label = label
                        + centered_cube(
                            format!("{name}_glyph_{char_index}_{row}_{col}"),
                            scale,
                            scale,
                            LABEL_Z,
                        )
                        .translate(
                            char_center_x + (col as f64 - 2.0) * scale,
                            (3.0 - row as f64) * scale,
                            0.0,
                        );
                }
            }
        }
    }
    label
}

fn glyph_rows(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => [
            " ### ", "#   #", "#   #", "#####", "#   #", "#   #", "#   #",
        ],
        'B' => [
            "#### ", "#   #", "#   #", "#### ", "#   #", "#   #", "#### ",
        ],
        'C' => [
            " ####", "#    ", "#    ", "#    ", "#    ", "#    ", " ####",
        ],
        'D' => [
            "#### ", "#   #", "#   #", "#   #", "#   #", "#   #", "#### ",
        ],
        'E' => [
            "#####", "#    ", "#    ", "#### ", "#    ", "#    ", "#####",
        ],
        'F' => [
            "#####", "#    ", "#    ", "#### ", "#    ", "#    ", "#    ",
        ],
        'G' => [
            " ####", "#    ", "#    ", "#  ##", "#   #", "#   #", " ####",
        ],
        'H' => [
            "#   #", "#   #", "#   #", "#####", "#   #", "#   #", "#   #",
        ],
        'I' => [
            "#####", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ", "#####",
        ],
        'J' => [
            "#####", "   # ", "   # ", "   # ", "   # ", "#  # ", " ##  ",
        ],
        'K' => [
            "#   #", "#  # ", "# #  ", "##   ", "# #  ", "#  # ", "#   #",
        ],
        'L' => [
            "#    ", "#    ", "#    ", "#    ", "#    ", "#    ", "#####",
        ],
        'M' => [
            "#   #", "## ##", "# # #", "#   #", "#   #", "#   #", "#   #",
        ],
        'N' => [
            "#   #", "##  #", "# # #", "#  ##", "#   #", "#   #", "#   #",
        ],
        'O' => [
            " ### ", "#   #", "#   #", "#   #", "#   #", "#   #", " ### ",
        ],
        'P' => [
            "#### ", "#   #", "#   #", "#### ", "#    ", "#    ", "#    ",
        ],
        'Q' => [
            " ### ", "#   #", "#   #", "#   #", "# # #", "#  # ", " ## #",
        ],
        'R' => [
            "#### ", "#   #", "#   #", "#### ", "# #  ", "#  # ", "#   #",
        ],
        'S' => [
            " ####", "#    ", "#    ", " ### ", "    #", "    #", "#### ",
        ],
        'T' => [
            "#####", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ", "  #  ",
        ],
        'U' => [
            "#   #", "#   #", "#   #", "#   #", "#   #", "#   #", " ### ",
        ],
        'V' => [
            "#   #", "#   #", "#   #", "#   #", "#   #", " # # ", "  #  ",
        ],
        'W' => [
            "#   #", "#   #", "#   #", "# # #", "# # #", "## ##", "#   #",
        ],
        'X' => [
            "#   #", "#   #", " # # ", "  #  ", " # # ", "#   #", "#   #",
        ],
        'Y' => [
            "#   #", "#   #", " # # ", "  #  ", "  #  ", "  #  ", "  #  ",
        ],
        'Z' => [
            "#####", "    #", "   # ", "  #  ", " #   ", "#    ", "#####",
        ],
        _ => [
            "     ", "     ", "     ", "     ", "     ", "     ", "     ",
        ],
    }
}

fn front_robot_keepout_y() -> f64 {
    ROBOT_FRONT_CLEARANCE_Y
}

fn rear_service_keepout_y() -> f64 {
    SERVICE_REAR_CLEARANCE_Y
}

fn left_reservoir_service_x() -> f64 {
    LEFT_RESERVOIR_SERVICE_X
}

fn right_sensor_service_x() -> f64 {
    RIGHT_SENSOR_SERVICE_X
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_modules_fit_without_overlap() {
        let modules = layout_modules();
        for module in modules {
            assert!(module.fits_on_deck());
        }
        for i in 0..modules.len() {
            for j in (i + 1)..modules.len() {
                assert!(!modules[i].overlaps(modules[j]));
            }
        }
    }

    #[test]
    fn requested_feature_groups_are_present() {
        for feature in [
            "removable_reservoir_surrogate",
            "coupon_rack_carriers",
            "drain_low_point_wells",
            "uv_decon_exposure_witness_lands",
            "conductivity_turbidity_sensor_pockets",
            "clean_dirty_segregation_bulkhead",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "csg_geometry_labels",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn witness_counts_match_lane_and_rack_design() {
        assert_eq!(BIOFILM_COUPON_COUNT, 24);
        assert_eq!(UV_WITNESS_LAND_COUNT, 12);
        assert_eq!(DISPOSITION_TOKEN_COUNT, 15);
        assert_eq!(SENSOR_POCKET_COUNT, SENSOR_FLOW_CELL_COUNT + 2);
    }

    #[test]
    fn csg_labels_include_operational_words() {
        for label in [
            "CLEAN", "DIRTY", "RELEASE", "HOLD", "REJECT", "BARCODE", "CERT",
        ] {
            assert!(CSG_LABELS.contains(&label));
        }
    }
}
