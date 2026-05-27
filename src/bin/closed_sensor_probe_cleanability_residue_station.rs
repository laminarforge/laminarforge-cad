use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sensor probe cleanability residue station.
//
// Intent:
// - Package reusable inline pH, DO, O2, temperature, pressure, and flow probes
//   in repeatable nests for cleaning residue and cleanability challenge work in
//   a closed cell-culture cabinet support workflow.
// - Keep rinse/soak wells, swab/sample slots, residue witness coupons, drying
//   rack, clean/used segregation, leak-safe tray drainage, and traceability
//   labels visible as mechanical CSG interfaces.
// - Model validation station architecture only. This is not a cleaning
//   procedure, microbial limit, assay validation, reusable-probe IFU, or
//   product release criterion.

const OUTPUT_PREFIX: &str = "output/closed_sensor_probe_cleanability_residue_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_sensor_probe_cleanability_residue_station_base_leak_safe_tray.stl",
    "output/closed_sensor_probe_cleanability_residue_station_probe_nest_array.stl",
    "output/closed_sensor_probe_cleanability_residue_station_rinse_soak_well_block.stl",
    "output/closed_sensor_probe_cleanability_residue_station_swab_slot_sample_strip.stl",
    "output/closed_sensor_probe_cleanability_residue_station_residue_witness_coupon_carrier.stl",
    "output/closed_sensor_probe_cleanability_residue_station_drying_rack_drip_comb.stl",
    "output/closed_sensor_probe_cleanability_residue_station_clean_used_segregation_gate.stl",
    "output/closed_sensor_probe_cleanability_residue_station_barcode_residue_label_lands.stl",
    "output/closed_sensor_probe_cleanability_residue_station_evidence_camera_bridge.stl",
    "output/closed_sensor_probe_cleanability_residue_station_waste_drain_leak_monitoring.stl",
    "output/closed_sensor_probe_cleanability_residue_station_robot_service_keepout_gauge.stl",
    "output/closed_sensor_probe_cleanability_residue_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "probe_nests",
    "rinse_soak_wells",
    "swab_slots",
    "residue_witness_coupons",
    "drying_rack",
    "leak_safe_tray",
    "clean_used_segregation",
    "labels_as_csg_geometry",
    "waste_drain_leak_monitoring",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_station_only",
    "no_cleaning_protocol",
    "no_assay_acceptance_limits",
    "no_sterile_barrier_claim",
    "no_probe_ifu_claim",
    "no_cell_culture_performance_claim",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 13.0;

const SENSOR_FAMILIES: usize = 6;
const SENSOR_NAMES: [&str; SENSOR_FAMILIES] = ["ph", "do", "o2", "temp", "pressure", "flow"];
const CLEAN_USED_LANES: usize = 2;
const PROBE_NEST_COUNT: usize = SENSOR_FAMILIES * CLEAN_USED_LANES;
const PROBE_NEST_X: f64 = 360.0;
const PROBE_NEST_Y: f64 = 180.0;
const PROBE_NEST_Z: f64 = 46.0;
const PROBE_NEST_POS: (f64, f64) = (-410.0, 230.0);
const PROBE_PITCH_X: f64 = 52.0;
const PROBE_LANE_PITCH_Y: f64 = 72.0;
const PROBE_SLOT_Y: f64 = 116.0;
const PROBE_SLOT_D: f64 = 17.5;
const PROBE_TIP_WELL_D: f64 = 24.0;
const PROBE_CABLE_NOTCH_COUNT: usize = PROBE_NEST_COUNT;

const WELL_BLOCK_X: f64 = 410.0;
const WELL_BLOCK_Y: f64 = 180.0;
const WELL_BLOCK_Z: f64 = 56.0;
const WELL_POS: (f64, f64) = (0.0, 230.0);
const RINSE_WELL_COUNT: usize = SENSOR_FAMILIES;
const SOAK_WELL_COUNT: usize = SENSOR_FAMILIES;
const TOTAL_WELLS: usize = RINSE_WELL_COUNT + SOAK_WELL_COUNT;
const WELL_PITCH_X: f64 = 58.0;
const WELL_ROW_PITCH_Y: f64 = 70.0;
const RINSE_WELL_D: f64 = 30.0;
const SOAK_WELL_D: f64 = 36.0;
const WELL_DEPTH: f64 = 38.0;
const WELL_VOLUME_TICK_COUNT: usize = 4;

const COUPON_CARRIER_X: f64 = 290.0;
const COUPON_CARRIER_Y: f64 = 180.0;
const COUPON_CARRIER_Z: f64 = 34.0;
const COUPON_POS: (f64, f64) = (430.0, 230.0);
const COUPON_COUNT: usize = 8;
const COUPON_PITCH_X: f64 = 58.0;
const COUPON_SLOT_X: f64 = 40.0;
const COUPON_SLOT_Y: f64 = 74.0;
const COUPON_PAD_Z: f64 = 4.0;
const WITNESS_DROP_WELLS: usize = 8;

const SWAB_STRIP_X: f64 = 330.0;
const SWAB_STRIP_Y: f64 = 190.0;
const SWAB_STRIP_Z: f64 = 40.0;
const SWAB_POS: (f64, f64) = (-420.0, 0.0);
const SWAB_SLOT_COUNT: usize = 12;
const SWAB_COLS: usize = 6;
const SWAB_ROWS: usize = 2;
const SWAB_PITCH_X: f64 = 48.0;
const SWAB_PITCH_Y: f64 = 62.0;
const SWAB_SLOT_X: f64 = 28.0;
const SWAB_SLOT_Y: f64 = 78.0;
const SAMPLE_VIAL_WELLS: usize = 6;
const SWAB_RETAINER_TEETH: usize = SWAB_COLS + 1;

const DRY_RACK_X: f64 = 380.0;
const DRY_RACK_Y: f64 = 190.0;
const DRY_RACK_Z: f64 = 78.0;
const DRY_POS: (f64, f64) = (0.0, 0.0);
const DRY_SLOT_COUNT: usize = PROBE_NEST_COUNT;
const DRY_SLOT_PITCH_X: f64 = 28.0;
const DRY_POST_COUNT: usize = 8;
const DRY_AIR_GAP_MM: f64 = 22.0;
const DRIP_CUP_COUNT: usize = 6;

const SEGREGATION_X: f64 = 300.0;
const SEGREGATION_Y: f64 = 190.0;
const SEGREGATION_Z: f64 = 70.0;
const SEG_POS: (f64, f64) = (420.0, 0.0);
const SEG_WELLS_PER_SIDE: usize = SENSOR_FAMILIES;
const CLEAN_USED_DIVIDER_Z: f64 = 118.0;
const SEGREGATION_AIR_GAP_MM: f64 = 54.0;

const LABEL_PANEL_X: f64 = 330.0;
const LABEL_PANEL_Y: f64 = 180.0;
const LABEL_PANEL_Z: f64 = 12.0;
const LABEL_POS: (f64, f64) = (-420.0, -245.0);
const BARCODE_LANDS: usize = 12;
const RESIDUE_LABEL_TOKENS: usize = 9;
const CSG_LABEL_BARS: usize = 54;

const WASTE_BLOCK_X: f64 = 410.0;
const WASTE_BLOCK_Y: f64 = 180.0;
const WASTE_BLOCK_Z: f64 = 48.0;
const WASTE_POS: (f64, f64) = (0.0, -245.0);
const DRAIN_CHANNELS: usize = 6;
const LEAK_SENSOR_WELLS: usize = 4;
const WASTE_TRAP_CUPS: usize = 4;

const CAMERA_BRIDGE_X: f64 = 300.0;
const CAMERA_BRIDGE_Y: f64 = 180.0;
const CAMERA_BRIDGE_Z: f64 = 188.0;
const CAMERA_POS: (f64, f64) = (420.0, -245.0);
const CAMERA_PORTS: usize = 3;
const LIGHT_BAR_COUNT: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 156.0;
const FOCUS_TARGET_COUNT: usize = 5;

const KEEP_OUT_X: f64 = 1200.0;
const KEEP_OUT_Y: f64 = 740.0;
const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_APPROACH_CLEARANCE_MM: f64 = 420.0;
const REAR_SERVICE_CLEARANCE_MM: f64 = 260.0;
const SIDE_PROBE_LOAD_CLEARANCE_MM: f64 = 190.0;
const DRY_RACK_LIFT_CLEARANCE_Z: f64 = 150.0;
const ROBOT_KEEP_OUT_ZONE_COUNT: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = STATION_X / 2.0 - RIM_W - 10.0;
        let half_y = STATION_Y / 2.0 - RIM_W - 10.0;

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

    let base = base_leak_safe_tray();
    export(OUTPUTS[0], &base);

    let nests = probe_nest_array();
    export(OUTPUTS[1], &nests);

    let wells = rinse_soak_well_block();
    export(OUTPUTS[2], &wells);

    let swabs = swab_slot_sample_strip();
    export(OUTPUTS[3], &swabs);

    let coupons = residue_witness_coupon_carrier();
    export(OUTPUTS[4], &coupons);

    let drying = drying_rack_drip_comb();
    export(OUTPUTS[5], &drying);

    let segregation = clean_used_segregation_gate();
    export(OUTPUTS[6], &segregation);

    let labels = barcode_residue_label_lands();
    export(OUTPUTS[7], &labels);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[8], &camera);

    let waste = waste_drain_leak_monitoring();
    export(OUTPUTS[9], &waste);

    let keepout = robot_service_keepout_gauge();
    export(OUTPUTS[10], &keepout);

    let assembly = base
        + nests.translate(PROBE_NEST_POS.0, PROBE_NEST_POS.1, on_deck_z(PROBE_NEST_Z))
        + wells.translate(WELL_POS.0, WELL_POS.1, on_deck_z(WELL_BLOCK_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_CARRIER_Z))
        + swabs.translate(SWAB_POS.0, SWAB_POS.1, on_deck_z(SWAB_STRIP_Z))
        + drying.translate(DRY_POS.0, DRY_POS.1, on_deck_z(DRY_RACK_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, on_deck_z(SEGREGATION_Z))
        + labels.translate(LABEL_POS.0, LABEL_POS.1, on_deck_z(LABEL_PANEL_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_BLOCK_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_BRIDGE_Z))
        + keepout.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0 + 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed sensor probe cleanability residue station:");
    println!("  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm leak-safe tray");
    println!(
        "  Probe coverage:            {SENSOR_FAMILIES} inline sensor families ({}) with {PROBE_NEST_COUNT} clean/used nest positions",
        SENSOR_NAMES.join(", ")
    );
    println!(
        "  Wet processing:            {RINSE_WELL_COUNT} rinse wells, {SOAK_WELL_COUNT} soak wells, {DRAIN_CHANNELS} isolated drain channels, {WASTE_TRAP_CUPS} waste trap cups"
    );
    println!(
        "  Residue evidence:          {SWAB_SLOT_COUNT} swab slots, {SAMPLE_VIAL_WELLS} sample vial wells, {COUPON_COUNT} residue witness coupons, {WITNESS_DROP_WELLS} droplet wells"
    );
    println!(
        "  Dry/segregate/trace:       {DRY_SLOT_COUNT} drying rack positions, {SEG_WELLS_PER_SIDE} clean and {SEG_WELLS_PER_SIDE} used wells, {BARCODE_LANDS} barcode lands, {CSG_LABEL_BARS} raised CSG label bars"
    );
    println!(
        "  Evidence/access:           {CAMERA_PORTS} camera ports, {LIGHT_BAR_COUNT} light bars, {LEAK_SENSOR_WELLS} leak sensor wells, {ROBOT_KEEP_OUT_ZONE_COUNT} keepout zones"
    );
    println!(
        "  Limitations:               mechanical validation station only; no cleaning protocol, assay limits, sterile barrier claim, or probe IFU claim"
    );
    println!("  Output prefix:             {OUTPUT_PREFIX}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn name(suffix: &str) -> String {
    format!("closed_sensor_probe_cleanability_residue_station_{suffix}")
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "probe_nest_array",
            PROBE_NEST_POS,
            PROBE_NEST_X,
            PROBE_NEST_Y,
        ),
        rect(
            "rinse_soak_well_block",
            WELL_POS,
            WELL_BLOCK_X,
            WELL_BLOCK_Y,
        ),
        rect(
            "residue_witness_coupon_carrier",
            COUPON_POS,
            COUPON_CARRIER_X,
            COUPON_CARRIER_Y,
        ),
        rect(
            "swab_slot_sample_strip",
            SWAB_POS,
            SWAB_STRIP_X,
            SWAB_STRIP_Y,
        ),
        rect("drying_rack_drip_comb", DRY_POS, DRY_RACK_X, DRY_RACK_Y),
        rect(
            "clean_used_segregation_gate",
            SEG_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        rect(
            "barcode_residue_label_lands",
            LABEL_POS,
            LABEL_PANEL_X,
            LABEL_PANEL_Y,
        ),
        rect(
            "waste_drain_leak_monitoring",
            WASTE_POS,
            WASTE_BLOCK_X,
            WASTE_BLOCK_Y,
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
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for feature in [
        "probe_nests",
        "rinse_soak_wells",
        "swab_slots",
        "residue_witness_coupons",
        "drying_rack",
        "leak_safe_tray",
        "labels_as_csg_geometry",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "mechanical_validation_station_only",
        "no_cleaning_protocol",
        "no_assay_acceptance_limits",
        "no_sterile_barrier_claim",
        "no_probe_ifu_claim",
        "no_cell_culture_performance_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(PROBE_NEST_COUNT, SENSOR_FAMILIES * CLEAN_USED_LANES);
    assert_eq!(TOTAL_WELLS, RINSE_WELL_COUNT + SOAK_WELL_COUNT);
    assert_eq!(SWAB_SLOT_COUNT, SWAB_ROWS * SWAB_COLS);
    assert_eq!(DRY_SLOT_COUNT, PROBE_NEST_COUNT);
    assert_eq!(PROBE_CABLE_NOTCH_COUNT, PROBE_NEST_COUNT);
    assert_eq!(SEG_WELLS_PER_SIDE, SENSOR_FAMILIES);
    assert!(WELL_DEPTH < WELL_BLOCK_Z);
    assert!(DRY_AIR_GAP_MM >= 20.0);
    assert!(SEGREGATION_AIR_GAP_MM >= 50.0);
    assert!(CLEAN_USED_DIVIDER_Z > SEGREGATION_Z);
    assert!(CAMERA_CLEARANCE_Z > DRY_RACK_LIFT_CLEARANCE_Z);
    assert!(ROBOT_APPROACH_CLEARANCE_MM >= 400.0);
    assert!(REAR_SERVICE_CLEARANCE_MM >= 250.0);
    assert!(SIDE_PROBE_LOAD_CLEARANCE_MM >= 180.0);

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

fn base_leak_safe_tray() -> Part {
    let deck = centered_cube(
        name("base_leak_safe_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_pan = centered_cube(
        name("base_secondary_containment_pan_cut"),
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z - 4.0);
    let front_drain_gutter = centered_cube(
        name("base_front_drain_gutter_cut"),
        STATION_X - 190.0,
        24.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 64.0, BASE_Z - 4.5);
    let drain = centered_cylinder(
        name("base_low_point_drain_bore_cut"),
        DRAIN_PORT_D / 2.0,
        56.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 42.0,
        BASE_Z - 8.0,
    );

    deck - recessed_pan - front_drain_gutter - drain - insert_sockets() - mount_holes()
        + perimeter_rim()
        + base_flow_witness_ribs()
        + base_datum_targets()
        + base_zone_label_lands()
        + leak_sensor_index_pockets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(name("base_module_insert_sockets"));
    for module in module_rects().iter().take(8) {
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

fn base_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty(name("base_leak_flow_witness_ribs"));
    for i in 0..9 {
        let x = centered_index(i, 9, 118.0);
        ribs = ribs
            + centered_cube(name(&format!("base_flow_witness_rib_{i}")), 6.0, 500.0, 4.0)
                .translate(x, -34.0, BASE_Z + 2.0);
    }
    for i in 0..5 {
        let x = centered_index(i, 5, 180.0);
        ribs = ribs
            + centered_cube(
                name(&format!("base_front_gutter_witness_tick_{i}")),
                18.0,
                4.0,
                6.0,
            )
            .translate(x, -STATION_Y / 2.0 + 80.0, BASE_Z + 3.0);
    }
    ribs
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

fn base_zone_label_lands() -> Part {
    raised_label_land("base_clean_probe_zone_label", 190.0, 24.0, 0).translate(
        -420.0,
        342.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_rinse_soak_zone_label", 190.0, 24.0, 1).translate(
        0.0,
        342.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_residue_coupon_zone_label", 190.0, 24.0, 2).translate(
        420.0,
        342.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_used_swab_zone_label", 190.0, 24.0, 3).translate(
        -420.0,
        -142.0,
        BASE_Z + 2.0,
    ) + raised_label_land("base_drain_waste_zone_label", 190.0, 24.0, 4).translate(
        0.0,
        -142.0,
        BASE_Z + 2.0,
    )
}

fn leak_sensor_index_pockets() -> Part {
    let mut pockets = Part::empty(name("base_leak_sensor_index_pocket_lands"));
    for (i, x) in [-510.0, -170.0, 170.0, 510.0].iter().enumerate() {
        let boss = centered_cube(name(&format!("base_leak_sensor_land_{i}")), 46.0, 30.0, 6.0)
            .translate(*x, -STATION_Y / 2.0 + 106.0, BASE_Z + 3.0);
        let cup = centered_cylinder(name(&format!("base_leak_sensor_cup_cut_{i}")), 8.0, 8.0, 28)
            .translate(*x, -STATION_Y / 2.0 + 106.0, BASE_Z + 4.0);
        pockets = pockets + (boss - cup);
    }
    pockets
}

fn probe_nest_array() -> Part {
    let body = centered_cube(
        name("probe_nest_array_body"),
        PROBE_NEST_X,
        PROBE_NEST_Y,
        PROBE_NEST_Z,
    );
    let recessed_field = centered_cube(
        name("probe_nest_array_cleaning_residue_recess_cut"),
        PROBE_NEST_X - 34.0,
        PROBE_NEST_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, PROBE_NEST_Z / 2.0 - 4.0);

    body - recessed_field - probe_body_channel_cuts() - probe_tip_well_cuts() - cable_notch_cuts()
        + probe_side_rails()
        + probe_end_stops()
        + probe_lane_label_lands()
        + probe_presence_fiducials()
}

fn probe_body_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("probe_body_channel_cuts"));
    for lane in 0..CLEAN_USED_LANES {
        let y = centered_index(lane, CLEAN_USED_LANES, PROBE_LANE_PITCH_Y);
        for sensor in 0..SENSOR_FAMILIES {
            let x = centered_index(sensor, SENSOR_FAMILIES, PROBE_PITCH_X);
            cuts = cuts
                + centered_cylinder(
                    name(&format!(
                        "probe_{}_lane_{lane}_body_cradle_cut",
                        SENSOR_NAMES[sensor]
                    )),
                    PROBE_SLOT_D / 2.0,
                    PROBE_SLOT_Y,
                    32,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, PROBE_NEST_Z / 2.0 + 1.5)
                + centered_cube(
                    name(&format!(
                        "probe_{}_lane_{lane}_flat_inspection_window_cut",
                        SENSOR_NAMES[sensor]
                    )),
                    22.0,
                    PROBE_SLOT_Y - 28.0,
                    16.0,
                )
                .translate(x, y, PROBE_NEST_Z / 2.0 + 4.0);
        }
    }
    cuts
}

fn probe_tip_well_cuts() -> Part {
    let mut cuts = Part::empty(name("probe_tip_residue_capture_well_cuts"));
    for lane in 0..CLEAN_USED_LANES {
        let y = centered_index(lane, CLEAN_USED_LANES, PROBE_LANE_PITCH_Y) + 40.0;
        for sensor in 0..SENSOR_FAMILIES {
            let x = centered_index(sensor, SENSOR_FAMILIES, PROBE_PITCH_X);
            cuts = cuts
                + centered_cylinder(
                    name(&format!(
                        "probe_{}_lane_{lane}_tip_residue_well_cut",
                        SENSOR_NAMES[sensor]
                    )),
                    PROBE_TIP_WELL_D / 2.0,
                    PROBE_NEST_Z + 6.0,
                    30,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn cable_notch_cuts() -> Part {
    let mut notches = Part::empty(name("probe_cable_exit_notch_cuts"));
    for lane in 0..CLEAN_USED_LANES {
        let y = centered_index(lane, CLEAN_USED_LANES, PROBE_LANE_PITCH_Y) - 56.0;
        for sensor in 0..SENSOR_FAMILIES {
            let x = centered_index(sensor, SENSOR_FAMILIES, PROBE_PITCH_X);
            notches = notches
                + centered_cube(
                    name(&format!(
                        "probe_{}_lane_{lane}_cable_exit_notch",
                        SENSOR_NAMES[sensor]
                    )),
                    18.0,
                    28.0,
                    PROBE_NEST_Z + 8.0,
                )
                .translate(x, y, 6.0);
        }
    }
    notches
}

fn probe_side_rails() -> Part {
    let mut rails = Part::empty(name("probe_nest_side_retainer_rails"));
    for lane in 0..CLEAN_USED_LANES {
        let y = centered_index(lane, CLEAN_USED_LANES, PROBE_LANE_PITCH_Y);
        rails = rails
            + centered_cube(
                name(&format!("probe_lane_{lane}_left_retainer_rail")),
                PROBE_NEST_X - 52.0,
                5.0,
                15.0,
            )
            .translate(0.0, y - 24.0, PROBE_NEST_Z / 2.0 + 7.5)
            + centered_cube(
                name(&format!("probe_lane_{lane}_right_retainer_rail")),
                PROBE_NEST_X - 52.0,
                5.0,
                15.0,
            )
            .translate(0.0, y + 24.0, PROBE_NEST_Z / 2.0 + 7.5);
    }
    rails
}

fn probe_end_stops() -> Part {
    let mut stops = Part::empty(name("probe_nest_indexed_end_stops"));
    for sensor in 0..SENSOR_FAMILIES {
        let x = centered_index(sensor, SENSOR_FAMILIES, PROBE_PITCH_X);
        stops = stops
            + centered_cube(
                name(&format!("probe_{}_front_tip_stop", SENSOR_NAMES[sensor])),
                28.0,
                5.0,
                20.0,
            )
            .translate(x, PROBE_NEST_Y / 2.0 - 24.0, PROBE_NEST_Z / 2.0 + 10.0)
            + centered_cube(
                name(&format!("probe_{}_rear_cable_stop", SENSOR_NAMES[sensor])),
                28.0,
                5.0,
                20.0,
            )
            .translate(x, -PROBE_NEST_Y / 2.0 + 24.0, PROBE_NEST_Z / 2.0 + 10.0);
    }
    stops
}

fn probe_lane_label_lands() -> Part {
    let mut labels = Part::empty(name("probe_nest_sensor_label_lands"));
    for sensor in 0..SENSOR_FAMILIES {
        let x = centered_index(sensor, SENSOR_FAMILIES, PROBE_PITCH_X);
        labels = labels
            + raised_label_land(
                &format!("probe_{}_family_label_land", SENSOR_NAMES[sensor]),
                38.0,
                16.0,
                sensor,
            )
            .translate(x, -PROBE_NEST_Y / 2.0 + 12.0, PROBE_NEST_Z / 2.0 + 3.0);
    }
    for (lane, y) in [(-PROBE_NEST_Y / 2.0 + 34.0), (PROBE_NEST_Y / 2.0 - 34.0)]
        .iter()
        .enumerate()
    {
        labels = labels
            + raised_label_land(
                &format!("probe_lane_{lane}_clean_used_label"),
                82.0,
                14.0,
                lane + 6,
            )
            .translate(-PROBE_NEST_X / 2.0 + 54.0, *y, PROBE_NEST_Z / 2.0 + 3.0);
    }
    labels
}

fn probe_presence_fiducials() -> Part {
    let mut fiducials = Part::empty(name("probe_nest_presence_fiducials"));
    for (i, (x, y)) in [
        (-PROBE_NEST_X / 2.0 + 24.0, -PROBE_NEST_Y / 2.0 + 24.0),
        (PROBE_NEST_X / 2.0 - 24.0, -PROBE_NEST_Y / 2.0 + 24.0),
        (-PROBE_NEST_X / 2.0 + 24.0, PROBE_NEST_Y / 2.0 - 24.0),
        (PROBE_NEST_X / 2.0 - 24.0, PROBE_NEST_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + datum_disc(&format!("probe_nest_fiducial_{i}")).translate(
                *x,
                *y,
                PROBE_NEST_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn rinse_soak_well_block() -> Part {
    let block = centered_cube(
        name("rinse_soak_well_block_body"),
        WELL_BLOCK_X,
        WELL_BLOCK_Y,
        WELL_BLOCK_Z,
    );
    let sump = centered_cube(
        name("rinse_soak_well_block_common_spill_sump_cut"),
        WELL_BLOCK_X - 34.0,
        WELL_BLOCK_Y - 32.0,
        12.0,
    )
    .translate(0.0, 0.0, WELL_BLOCK_Z / 2.0 - 6.0);

    block - sump - rinse_well_cuts() - soak_well_cuts() - overflow_channel_cuts()
        + well_rims_and_ticks()
        + well_row_label_lands()
        + pipette_guard_rails()
}

fn rinse_well_cuts() -> Part {
    let mut cuts = Part::empty(name("rinse_well_cuts"));
    for i in 0..RINSE_WELL_COUNT {
        let x = centered_index(i, RINSE_WELL_COUNT, WELL_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                name(&format!("rinse_well_{i}_cup_cut")),
                RINSE_WELL_D / 2.0,
                WELL_DEPTH,
                40,
            )
            .translate(
                x,
                WELL_ROW_PITCH_Y / 2.0,
                WELL_BLOCK_Z / 2.0 - WELL_DEPTH / 2.0 + 2.0,
            );
    }
    cuts
}

fn soak_well_cuts() -> Part {
    let mut cuts = Part::empty(name("soak_well_cuts"));
    for i in 0..SOAK_WELL_COUNT {
        let x = centered_index(i, SOAK_WELL_COUNT, WELL_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                name(&format!("soak_well_{i}_cup_cut")),
                SOAK_WELL_D / 2.0,
                WELL_DEPTH + 6.0,
                44,
            )
            .translate(
                x,
                -WELL_ROW_PITCH_Y / 2.0,
                WELL_BLOCK_Z / 2.0 - (WELL_DEPTH + 6.0) / 2.0 + 2.0,
            );
    }
    cuts
}

fn overflow_channel_cuts() -> Part {
    let rinse_header = centered_cube(
        name("rinse_well_overflow_header_cut"),
        WELL_BLOCK_X - 74.0,
        10.0,
        14.0,
    )
    .translate(0.0, WELL_ROW_PITCH_Y / 2.0 - 24.0, WELL_BLOCK_Z / 2.0 - 3.0);
    let soak_header = centered_cube(
        name("soak_well_overflow_header_cut"),
        WELL_BLOCK_X - 74.0,
        10.0,
        14.0,
    )
    .translate(
        0.0,
        -WELL_ROW_PITCH_Y / 2.0 - 24.0,
        WELL_BLOCK_Z / 2.0 - 3.0,
    );
    let drain_slot = centered_cube(
        name("rinse_soak_well_block_front_overflow_to_waste_cut"),
        22.0,
        48.0,
        16.0,
    )
    .translate(
        WELL_BLOCK_X / 2.0 - 34.0,
        -WELL_BLOCK_Y / 2.0 + 22.0,
        WELL_BLOCK_Z / 2.0 - 4.0,
    );
    rinse_header + soak_header + drain_slot
}

fn well_rims_and_ticks() -> Part {
    let mut details = Part::empty(name("rinse_soak_well_rims_and_volume_ticks"));
    for (row, y, diameter, count) in [
        (
            "rinse",
            WELL_ROW_PITCH_Y / 2.0,
            RINSE_WELL_D,
            RINSE_WELL_COUNT,
        ),
        (
            "soak",
            -WELL_ROW_PITCH_Y / 2.0,
            SOAK_WELL_D,
            SOAK_WELL_COUNT,
        ),
    ] {
        for i in 0..count {
            let x = centered_index(i, count, WELL_PITCH_X);
            let outer = centered_cylinder(
                name(&format!("{row}_well_{i}_raised_rim_outer")),
                diameter / 2.0 + 6.0,
                6.0,
                44,
            )
            .translate(x, y, WELL_BLOCK_Z / 2.0 + 3.0);
            let inner = centered_cylinder(
                name(&format!("{row}_well_{i}_raised_rim_inner_cut")),
                diameter / 2.0 + 1.5,
                8.0,
                44,
            )
            .translate(x, y, WELL_BLOCK_Z / 2.0 + 3.0);
            details = details + (outer - inner);
            for tick in 0..WELL_VOLUME_TICK_COUNT {
                details = details
                    + centered_cube(
                        name(&format!("{row}_well_{i}_volume_tick_{tick}")),
                        9.0 + tick as f64 * 2.0,
                        2.0,
                        5.0,
                    )
                    .translate(
                        x + diameter / 2.0 + 10.0,
                        y - 11.0 + tick as f64 * 7.0,
                        WELL_BLOCK_Z / 2.0 + 4.0,
                    );
            }
        }
    }
    details
}

fn well_row_label_lands() -> Part {
    raised_label_land("rinse_row_csg_label", 116.0, 18.0, 0).translate(
        -WELL_BLOCK_X / 2.0 + 70.0,
        WELL_ROW_PITCH_Y / 2.0,
        WELL_BLOCK_Z / 2.0 + 5.0,
    ) + raised_label_land("soak_row_csg_label", 116.0, 18.0, 1).translate(
        -WELL_BLOCK_X / 2.0 + 70.0,
        -WELL_ROW_PITCH_Y / 2.0,
        WELL_BLOCK_Z / 2.0 + 5.0,
    )
}

fn pipette_guard_rails() -> Part {
    let front = centered_cube(
        name("rinse_soak_front_pipette_guard_rail"),
        WELL_BLOCK_X - 36.0,
        5.0,
        20.0,
    )
    .translate(0.0, -WELL_BLOCK_Y / 2.0 + 18.0, WELL_BLOCK_Z / 2.0 + 10.0);
    let rear = centered_cube(
        name("rinse_soak_rear_pipette_guard_rail"),
        WELL_BLOCK_X - 36.0,
        5.0,
        20.0,
    )
    .translate(0.0, WELL_BLOCK_Y / 2.0 - 18.0, WELL_BLOCK_Z / 2.0 + 10.0);
    front + rear
}

fn swab_slot_sample_strip() -> Part {
    let tray = centered_cube(
        name("swab_slot_sample_strip_body"),
        SWAB_STRIP_X,
        SWAB_STRIP_Y,
        SWAB_STRIP_Z,
    );
    let sump = centered_cube(
        name("swab_slot_sample_strip_sump_cut"),
        SWAB_STRIP_X - 32.0,
        SWAB_STRIP_Y - 30.0,
        12.0,
    )
    .translate(0.0, 0.0, SWAB_STRIP_Z / 2.0 - 5.0);

    tray - sump - swab_slot_cuts() - sample_vial_well_cuts()
        + swab_retainer_comb()
        + swab_row_label_lands()
        + sample_vial_rims()
        + swab_handle_parking_rails()
}

fn swab_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("swab_slot_cut_grid"));
    for row in 0..SWAB_ROWS {
        let y = centered_index(row, SWAB_ROWS, SWAB_PITCH_Y) + 10.0;
        for col in 0..SWAB_COLS {
            let x = centered_index(col, SWAB_COLS, SWAB_PITCH_X);
            cuts = cuts
                + centered_cube(
                    name(&format!("swab_slot_row_{row}_col_{col}_cut")),
                    SWAB_SLOT_X,
                    SWAB_SLOT_Y,
                    SWAB_STRIP_Z + 6.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn sample_vial_well_cuts() -> Part {
    let mut cuts = Part::empty(name("swab_sample_vial_well_cuts"));
    for i in 0..SAMPLE_VIAL_WELLS {
        let x = centered_index(i, SAMPLE_VIAL_WELLS, 46.0);
        cuts = cuts
            + centered_cylinder(
                name(&format!("swab_sample_vial_well_{i}_cut")),
                11.0,
                SWAB_STRIP_Z + 6.0,
                30,
            )
            .translate(x, -SWAB_STRIP_Y / 2.0 + 28.0, 0.0);
    }
    cuts
}

fn swab_retainer_comb() -> Part {
    let mut comb = Part::empty(name("swab_retainer_comb_teeth"));
    for tooth in 0..SWAB_RETAINER_TEETH {
        let x = -((SWAB_COLS as f64) * SWAB_PITCH_X) / 2.0 + tooth as f64 * SWAB_PITCH_X;
        comb = comb
            + centered_cube(
                name(&format!("swab_retainer_comb_tooth_{tooth}")),
                4.0,
                154.0,
                18.0,
            )
            .translate(x, 10.0, SWAB_STRIP_Z / 2.0 + 9.0);
    }
    comb
}

fn swab_row_label_lands() -> Part {
    let mut labels = Part::empty(name("swab_slot_csg_label_lands"));
    for row in 0..SWAB_ROWS {
        let y = centered_index(row, SWAB_ROWS, SWAB_PITCH_Y) + 10.0;
        labels =
            labels
                + raised_label_land(&format!("swab_row_{row}_csg_label"), 76.0, 14.0, row)
                    .translate(-SWAB_STRIP_X / 2.0 + 44.0, y, SWAB_STRIP_Z / 2.0 + 3.0);
    }
    labels
}

fn sample_vial_rims() -> Part {
    let mut rims = Part::empty(name("swab_sample_vial_rims"));
    for i in 0..SAMPLE_VIAL_WELLS {
        let x = centered_index(i, SAMPLE_VIAL_WELLS, 46.0);
        let outer = centered_cylinder(
            name(&format!("swab_sample_vial_{i}_rim_outer")),
            15.5,
            5.0,
            30,
        )
        .translate(x, -SWAB_STRIP_Y / 2.0 + 28.0, SWAB_STRIP_Z / 2.0 + 2.5);
        let inner = centered_cylinder(
            name(&format!("swab_sample_vial_{i}_rim_inner_cut")),
            11.5,
            7.0,
            30,
        )
        .translate(x, -SWAB_STRIP_Y / 2.0 + 28.0, SWAB_STRIP_Z / 2.0 + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn swab_handle_parking_rails() -> Part {
    centered_cube(
        name("swab_handle_front_parking_rail"),
        SWAB_STRIP_X - 48.0,
        6.0,
        14.0,
    )
    .translate(0.0, -SWAB_STRIP_Y / 2.0 + 54.0, SWAB_STRIP_Z / 2.0 + 7.0)
        + centered_cube(
            name("swab_handle_rear_parking_rail"),
            SWAB_STRIP_X - 48.0,
            6.0,
            14.0,
        )
        .translate(0.0, SWAB_STRIP_Y / 2.0 - 18.0, SWAB_STRIP_Z / 2.0 + 7.0)
}

fn residue_witness_coupon_carrier() -> Part {
    let plate = centered_cube(
        name("residue_witness_coupon_carrier_plate"),
        COUPON_CARRIER_X,
        COUPON_CARRIER_Y,
        COUPON_CARRIER_Z,
    );
    let recessed_field = centered_cube(
        name("residue_witness_coupon_wet_recess_cut"),
        COUPON_CARRIER_X - 28.0,
        COUPON_CARRIER_Y - 30.0,
        9.0,
    )
    .translate(0.0, 0.0, COUPON_CARRIER_Z / 2.0 - 4.0);

    plate - recessed_field - coupon_slot_cuts() - witness_drop_well_cuts()
        + removable_coupon_surfaces()
        + coupon_retainer_clips()
        + coupon_material_label_lands()
        + witness_reference_tick_bars()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("residue_witness_coupon_slot_cuts"));
    for i in 0..COUPON_COUNT {
        let x = centered_index(i, COUPON_COUNT, COUPON_PITCH_X);
        cuts = cuts
            + centered_cube(
                name(&format!("residue_witness_coupon_slot_{i}_cut")),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_CARRIER_Z + 6.0,
            )
            .translate(x, 24.0, 0.0);
    }
    cuts
}

fn witness_drop_well_cuts() -> Part {
    let mut cuts = Part::empty(name("residue_witness_drop_well_cuts"));
    for i in 0..WITNESS_DROP_WELLS {
        let x = centered_index(i, WITNESS_DROP_WELLS, 32.0);
        cuts = cuts
            + centered_cylinder(
                name(&format!("residue_witness_drop_well_{i}_cut")),
                8.0,
                COUPON_CARRIER_Z + 6.0,
                26,
            )
            .translate(x, -COUPON_CARRIER_Y / 2.0 + 32.0, 0.0);
    }
    cuts
}

fn removable_coupon_surfaces() -> Part {
    let mut coupons = Part::empty(name("removable_residue_witness_coupon_surfaces"));
    for i in 0..COUPON_COUNT {
        let x = centered_index(i, COUPON_COUNT, COUPON_PITCH_X);
        coupons = coupons
            + centered_cube(
                name(&format!("removable_residue_witness_coupon_{i}")),
                COUPON_SLOT_X - 8.0,
                COUPON_SLOT_Y - 14.0,
                COUPON_PAD_Z,
            )
            .translate(x, 24.0, COUPON_CARRIER_Z / 2.0 + COUPON_PAD_Z / 2.0);
    }
    coupons
}

fn coupon_retainer_clips() -> Part {
    let mut clips = Part::empty(name("residue_coupon_retainer_clip_pairs"));
    for i in 0..COUPON_COUNT {
        let x = centered_index(i, COUPON_COUNT, COUPON_PITCH_X);
        clips = clips
            + centered_cube(
                name(&format!("coupon_{i}_left_clip")),
                5.0,
                COUPON_SLOT_Y + 10.0,
                12.0,
            )
            .translate(
                x - COUPON_SLOT_X / 2.0 - 5.0,
                24.0,
                COUPON_CARRIER_Z / 2.0 + 6.0,
            )
            + centered_cube(
                name(&format!("coupon_{i}_right_clip")),
                5.0,
                COUPON_SLOT_Y + 10.0,
                12.0,
            )
            .translate(
                x + COUPON_SLOT_X / 2.0 + 5.0,
                24.0,
                COUPON_CARRIER_Z / 2.0 + 6.0,
            );
    }
    clips
}

fn coupon_material_label_lands() -> Part {
    let mut labels = Part::empty(name("residue_coupon_material_csg_label_lands"));
    for i in 0..COUPON_COUNT {
        let x = centered_index(i, COUPON_COUNT, COUPON_PITCH_X);
        labels = labels
            + raised_label_land(&format!("residue_coupon_material_{i}_label"), 36.0, 13.0, i)
                .translate(
                    x,
                    COUPON_CARRIER_Y / 2.0 - 14.0,
                    COUPON_CARRIER_Z / 2.0 + 3.0,
                );
    }
    labels
}

fn witness_reference_tick_bars() -> Part {
    let mut bars = Part::empty(name("residue_witness_reference_tick_bars"));
    for i in 0..WITNESS_DROP_WELLS {
        let x = centered_index(i, WITNESS_DROP_WELLS, 32.0);
        bars = bars
            + centered_cube(
                name(&format!("residue_witness_drop_well_{i}_reference_bar")),
                18.0,
                3.0,
                5.0,
            )
            .translate(
                x,
                -COUPON_CARRIER_Y / 2.0 + 55.0,
                COUPON_CARRIER_Z / 2.0 + 2.5,
            );
    }
    bars
}

fn drying_rack_drip_comb() -> Part {
    let base = centered_cube(
        name("drying_rack_drip_comb_base"),
        DRY_RACK_X,
        DRY_RACK_Y,
        24.0,
    );
    let drip_trough = centered_cube(
        name("drying_rack_drip_trough_cut"),
        DRY_RACK_X - 44.0,
        54.0,
        12.0,
    )
    .translate(0.0, -DRY_RACK_Y / 2.0 + 38.0, 8.0);
    base - drip_trough - drying_slot_relief_cuts()
        + drying_posts()
        + drying_probe_saddle_rails()
        + drip_cup_rims()
        + drying_air_gap_gauge_bars()
        + drying_rack_label_lands()
}

fn drying_slot_relief_cuts() -> Part {
    let mut cuts = Part::empty(name("drying_rack_probe_slot_relief_cuts"));
    for i in 0..DRY_SLOT_COUNT {
        let x = centered_index(i, DRY_SLOT_COUNT, DRY_SLOT_PITCH_X);
        cuts = cuts
            + centered_cube(
                name(&format!("drying_probe_slot_{i}_drip_relief_cut")),
                16.0,
                88.0,
                28.0,
            )
            .translate(x, 18.0, 6.0);
    }
    cuts
}

fn drying_posts() -> Part {
    let mut posts = Part::empty(name("drying_rack_upright_posts"));
    for i in 0..DRY_POST_COUNT {
        let x = centered_index(i, DRY_POST_COUNT, 48.0);
        posts = posts
            + centered_cube(
                name(&format!("drying_rack_rear_upright_post_{i}")),
                12.0,
                12.0,
                DRY_RACK_Z,
            )
            .translate(x, DRY_RACK_Y / 2.0 - 30.0, DRY_RACK_Z / 2.0)
            + centered_cube(
                name(&format!("drying_rack_front_short_post_{i}")),
                12.0,
                12.0,
                DRY_RACK_Z - 28.0,
            )
            .translate(x, 8.0, (DRY_RACK_Z - 28.0) / 2.0);
    }
    posts
}

fn drying_probe_saddle_rails() -> Part {
    let rear = centered_cube(
        name("drying_rack_rear_probe_saddle_rail"),
        DRY_RACK_X - 48.0,
        10.0,
        14.0,
    )
    .translate(0.0, DRY_RACK_Y / 2.0 - 30.0, DRY_RACK_Z - 10.0);
    let front = centered_cube(
        name("drying_rack_front_probe_saddle_rail"),
        DRY_RACK_X - 48.0,
        10.0,
        14.0,
    )
    .translate(0.0, 8.0, DRY_RACK_Z - 38.0);
    let retainer = centered_cube(
        name("drying_rack_probe_tip_retainer_bar"),
        DRY_RACK_X - 74.0,
        6.0,
        16.0,
    )
    .translate(0.0, -DRY_RACK_Y / 2.0 + 74.0, 32.0);
    rear + front + retainer
}

fn drip_cup_rims() -> Part {
    let mut cups = Part::empty(name("drying_rack_drip_cup_rims"));
    for i in 0..DRIP_CUP_COUNT {
        let x = centered_index(i, DRIP_CUP_COUNT, 52.0);
        let outer = centered_cylinder(
            name(&format!("drying_drip_cup_{i}_rim_outer")),
            15.0,
            5.0,
            30,
        )
        .translate(x, -DRY_RACK_Y / 2.0 + 38.0, 24.0 / 2.0 + 2.5);
        let inner = centered_cylinder(
            name(&format!("drying_drip_cup_{i}_rim_inner_cut")),
            10.0,
            7.0,
            30,
        )
        .translate(x, -DRY_RACK_Y / 2.0 + 38.0, 24.0 / 2.0 + 2.5);
        cups = cups + (outer - inner);
    }
    cups
}

fn drying_air_gap_gauge_bars() -> Part {
    let mut bars = Part::empty(name("drying_rack_air_gap_gauge_bars"));
    for i in 0..4 {
        let x = -DRY_RACK_X / 2.0 + 36.0 + i as f64 * 18.0;
        bars = bars
            + centered_cube(
                name(&format!(
                    "drying_rack_{DRY_AIR_GAP_MM:.0}mm_air_gap_bar_{i}"
                )),
                12.0,
                58.0,
                5.0,
            )
            .translate(x, 38.0, 24.0 / 2.0 + 2.5);
    }
    bars
}

fn drying_rack_label_lands() -> Part {
    raised_label_land("drying_rack_csg_label", 132.0, 18.0, 4).translate(
        DRY_RACK_X / 2.0 - 82.0,
        -DRY_RACK_Y / 2.0 + 18.0,
        24.0 / 2.0 + 4.0,
    )
}

fn clean_used_segregation_gate() -> Part {
    let body = centered_cube(
        name("clean_used_segregation_gate_body"),
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let clean_basin = centered_cube(
        name("clean_probe_basin_cut"),
        SEGREGATION_X / 2.0 - 34.0,
        SEGREGATION_Y - 42.0,
        SEGREGATION_Z - 14.0,
    )
    .translate(-SEGREGATION_X / 4.0, 0.0, 6.0);
    let used_basin = centered_cube(
        name("used_probe_basin_cut"),
        SEGREGATION_X / 2.0 - 34.0,
        SEGREGATION_Y - 42.0,
        SEGREGATION_Z - 14.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, 6.0);
    let divider = centered_cube(
        name("clean_used_tall_splash_divider"),
        16.0,
        SEGREGATION_Y - 20.0,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(0.0, 0.0, CLEAN_USED_DIVIDER_Z / 2.0);
    let one_way_bridge = centered_cube(
        name("clean_to_used_one_way_transfer_bridge"),
        58.0,
        34.0,
        18.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 45.0, SEGREGATION_Z / 2.0 + 9.0);
    let bridge_slot = centered_cube(
        name("clean_to_used_one_way_transfer_slot_cut"),
        34.0,
        38.0,
        22.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 45.0, SEGREGATION_Z / 2.0 + 9.0);

    body - clean_basin - used_basin - segregation_well_cuts()
        + divider
        + (one_way_bridge - bridge_slot)
        + segregation_label_lands()
        + segregation_gate_tabs()
}

fn segregation_well_cuts() -> Part {
    let mut cuts = Part::empty(name("clean_used_probe_parking_well_cuts"));
    for side in 0..2 {
        let center_x = if side == 0 {
            -SEGREGATION_X / 4.0
        } else {
            SEGREGATION_X / 4.0
        };
        for i in 0..SEG_WELLS_PER_SIDE {
            let y = centered_index(i, SEG_WELLS_PER_SIDE, 24.0);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("segregation_side_{side}_probe_well_{i}_cut")),
                    8.0,
                    SEGREGATION_Z + 6.0,
                    24,
                )
                .translate(center_x, y, 0.0);
        }
    }
    cuts
}

fn segregation_label_lands() -> Part {
    raised_label_land("segregation_clean_label", 82.0, 18.0, 0).translate(
        -SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 16.0,
        SEGREGATION_Z / 2.0 + 4.0,
    ) + raised_label_land("segregation_used_label", 82.0, 18.0, 1).translate(
        SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 16.0,
        SEGREGATION_Z / 2.0 + 4.0,
    )
}

fn segregation_gate_tabs() -> Part {
    let mut tabs = Part::empty(name("clean_used_segregation_air_gap_tabs"));
    for i in 0..4 {
        tabs = tabs
            + centered_cube(
                name(&format!(
                    "segregation_{SEGREGATION_AIR_GAP_MM:.0}mm_air_gap_tab_{i}"
                )),
                10.0,
                32.0,
                6.0,
            )
            .translate(
                -15.0 + i as f64 * 10.0,
                -SEGREGATION_Y / 2.0 + 20.0,
                SEGREGATION_Z / 2.0 + 3.0,
            );
    }
    tabs
}

fn barcode_residue_label_lands() -> Part {
    let panel = centered_cube(
        name("barcode_residue_label_panel"),
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    );
    let recess = centered_cube(
        name("barcode_residue_label_panel_recess_cut"),
        LABEL_PANEL_X - 28.0,
        LABEL_PANEL_Y - 28.0,
        6.0,
    )
    .translate(0.0, 0.0, LABEL_PANEL_Z / 2.0 - 2.0);

    panel - recess
        + barcode_label_lands()
        + residue_process_label_tokens()
        + raised_csg_label_bar_matrix()
        + traceability_fiducials()
}

fn barcode_label_lands() -> Part {
    let mut lands = Part::empty(name("barcode_scan_label_lands"));
    for row in 0..3 {
        let y = centered_index(row, 3, 42.0) + 20.0;
        for col in 0..4 {
            let x = centered_index(col, 4, 72.0);
            let index = row * 4 + col;
            lands = lands
                + centered_cube(name(&format!("barcode_scan_land_{index}")), 54.0, 20.0, 4.0)
                    .translate(x, y, LABEL_PANEL_Z / 2.0 + 2.0);
        }
    }
    lands
}

fn residue_process_label_tokens() -> Part {
    let mut tokens = Part::empty(name("residue_process_label_tokens"));
    for i in 0..RESIDUE_LABEL_TOKENS {
        let x = centered_index(i % 3, 3, 86.0);
        let y = -LABEL_PANEL_Y / 2.0 + 28.0 + (i / 3) as f64 * 22.0;
        tokens =
            tokens
                + raised_label_land(&format!("residue_process_token_{i}"), 58.0, 14.0, i)
                    .translate(x, y, LABEL_PANEL_Z / 2.0 + 3.0);
    }
    tokens
}

fn raised_csg_label_bar_matrix() -> Part {
    let mut bars = Part::empty(name("raised_csg_label_bar_matrix"));
    let mut count = 0;
    for row in 0..6 {
        let y = LABEL_PANEL_Y / 2.0 - 22.0 - row as f64 * 12.0;
        for col in 0..9 {
            let x = -LABEL_PANEL_X / 2.0 + 34.0 + col as f64 * 9.0;
            let height = if (row + col) % 3 == 0 { 9.0 } else { 5.0 };
            bars = bars
                + centered_cube(
                    name(&format!("raised_csg_label_bar_{row}_{col}")),
                    3.0,
                    height,
                    3.0,
                )
                .translate(x, y, LABEL_PANEL_Z / 2.0 + 1.5);
            count += 1;
        }
    }
    assert_eq!(count, CSG_LABEL_BARS);
    bars
}

fn traceability_fiducials() -> Part {
    datum_disc("label_panel_left_fiducial").translate(
        -LABEL_PANEL_X / 2.0 + 24.0,
        -LABEL_PANEL_Y / 2.0 + 24.0,
        LABEL_PANEL_Z / 2.0 + 2.0,
    ) + datum_disc("label_panel_right_fiducial").translate(
        LABEL_PANEL_X / 2.0 - 24.0,
        -LABEL_PANEL_Y / 2.0 + 24.0,
        LABEL_PANEL_Z / 2.0 + 2.0,
    )
}

fn evidence_camera_bridge() -> Part {
    let left_column = centered_cube(
        name("evidence_camera_bridge_left_column"),
        24.0,
        CAMERA_BRIDGE_Y - 20.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 24.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_column = centered_cube(
        name("evidence_camera_bridge_right_column"),
        24.0,
        CAMERA_BRIDGE_Y - 20.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 24.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let crossbar = centered_cube(
        name("evidence_camera_bridge_crossbar"),
        CAMERA_BRIDGE_X - 30.0,
        34.0,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);
    let windows = camera_port_cuts();

    left_column
        + right_column
        + (crossbar - windows)
        + light_bar_mounts()
        + camera_focus_targets()
        + camera_bridge_label_lands()
}

fn camera_port_cuts() -> Part {
    let mut cuts = Part::empty(name("evidence_camera_port_cuts"));
    for i in 0..CAMERA_PORTS {
        let x = centered_index(i, CAMERA_PORTS, 74.0);
        cuts = cuts
            + centered_cube(
                name(&format!("evidence_camera_port_{i}_window_cut")),
                42.0,
                42.0,
                36.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z - 14.0);
    }
    cuts
}

fn light_bar_mounts() -> Part {
    let mut bars = Part::empty(name("evidence_camera_light_bar_mounts"));
    for i in 0..LIGHT_BAR_COUNT {
        let y = if i == 0 { -50.0 } else { 50.0 };
        bars = bars
            + centered_cube(
                name(&format!("evidence_camera_light_bar_{i}_mount")),
                CAMERA_BRIDGE_X - 80.0,
                8.0,
                10.0,
            )
            .translate(0.0, y, CAMERA_BRIDGE_Z - 48.0);
    }
    bars
}

fn camera_focus_targets() -> Part {
    let mut targets = Part::empty(name("evidence_camera_focus_target_lands"));
    for i in 0..FOCUS_TARGET_COUNT {
        let x = centered_index(i, FOCUS_TARGET_COUNT, 46.0);
        targets = targets
            + datum_disc(&format!("evidence_focus_target_{i}")).translate(
                x,
                -CAMERA_BRIDGE_Y / 2.0 + 28.0,
                8.0,
            );
    }
    targets
}

fn camera_bridge_label_lands() -> Part {
    raised_label_land("evidence_camera_bridge_csg_label", 126.0, 18.0, 7).translate(
        0.0,
        CAMERA_BRIDGE_Y / 2.0 - 20.0,
        CAMERA_BRIDGE_Z - 48.0,
    )
}

fn waste_drain_leak_monitoring() -> Part {
    let block = centered_cube(
        name("waste_drain_leak_monitoring_block"),
        WASTE_BLOCK_X,
        WASTE_BLOCK_Y,
        WASTE_BLOCK_Z,
    );
    let sump = centered_cube(
        name("waste_drain_common_secondary_sump_cut"),
        WASTE_BLOCK_X - 34.0,
        WASTE_BLOCK_Y - 34.0,
        14.0,
    )
    .translate(0.0, 0.0, WASTE_BLOCK_Z / 2.0 - 6.0);
    let main_drain = centered_cylinder(
        name("waste_drain_main_deck_port_cut"),
        DRAIN_PORT_D / 2.0,
        WASTE_BLOCK_Z + 8.0,
        32,
    )
    .translate(WASTE_BLOCK_X / 2.0 - 42.0, -WASTE_BLOCK_Y / 2.0 + 42.0, 0.0);

    block
        - sump
        - main_drain
        - waste_channel_cuts()
        - waste_trap_cup_cuts()
        - leak_monitor_well_cuts()
        + waste_channel_labels()
        + leak_sensor_rims()
        + waste_service_connector_lands()
}

fn waste_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("waste_isolated_drain_channel_cuts"));
    for i in 0..DRAIN_CHANNELS {
        let y = centered_index(i, DRAIN_CHANNELS, 22.0);
        cuts = cuts
            + centered_cube(
                name(&format!("waste_drain_channel_{i}_cut")),
                WASTE_BLOCK_X - 92.0,
                7.0,
                14.0,
            )
            .translate(-24.0, y, WASTE_BLOCK_Z / 2.0 - 4.0);
    }
    cuts
}

fn waste_trap_cup_cuts() -> Part {
    let mut cuts = Part::empty(name("waste_trap_cup_cuts"));
    for i in 0..WASTE_TRAP_CUPS {
        let x = centered_index(i, WASTE_TRAP_CUPS, 58.0) - 72.0;
        cuts = cuts
            + centered_cylinder(
                name(&format!("waste_trap_cup_{i}_cut")),
                17.0,
                WASTE_BLOCK_Z + 8.0,
                36,
            )
            .translate(x, -WASTE_BLOCK_Y / 2.0 + 34.0, 0.0);
    }
    cuts
}

fn leak_monitor_well_cuts() -> Part {
    let mut cuts = Part::empty(name("waste_leak_monitor_well_cuts"));
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 58.0) + 70.0;
        cuts = cuts
            + centered_cylinder(
                name(&format!("waste_leak_monitor_well_{i}_cut")),
                9.0,
                WASTE_BLOCK_Z + 8.0,
                28,
            )
            .translate(x, WASTE_BLOCK_Y / 2.0 - 36.0, 0.0);
    }
    cuts
}

fn waste_channel_labels() -> Part {
    let mut labels = Part::empty(name("waste_channel_label_lands"));
    for i in 0..DRAIN_CHANNELS {
        let y = centered_index(i, DRAIN_CHANNELS, 22.0);
        labels = labels
            + centered_cube(
                name(&format!("waste_channel_{i}_label_land")),
                42.0,
                7.0,
                4.0,
            )
            .translate(-WASTE_BLOCK_X / 2.0 + 34.0, y, WASTE_BLOCK_Z / 2.0 + 2.0);
    }
    labels
}

fn leak_sensor_rims() -> Part {
    let mut rims = Part::empty(name("waste_leak_sensor_rims"));
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 58.0) + 70.0;
        let outer = centered_cylinder(
            name(&format!("waste_leak_sensor_{i}_rim_outer")),
            13.5,
            4.0,
            28,
        )
        .translate(x, WASTE_BLOCK_Y / 2.0 - 36.0, WASTE_BLOCK_Z / 2.0 + 2.0);
        let inner = centered_cylinder(
            name(&format!("waste_leak_sensor_{i}_rim_inner_cut")),
            9.5,
            6.0,
            28,
        )
        .translate(x, WASTE_BLOCK_Y / 2.0 - 36.0, WASTE_BLOCK_Z / 2.0 + 2.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn waste_service_connector_lands() -> Part {
    let mut lands = Part::empty(name("waste_service_connector_lands"));
    for i in 0..3 {
        lands = lands
            + centered_cube(
                name(&format!("waste_service_connector_land_{i}")),
                52.0,
                18.0,
                8.0,
            )
            .translate(
                WASTE_BLOCK_X / 2.0 - 48.0,
                centered_index(i, 3, 42.0),
                WASTE_BLOCK_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn robot_service_keepout_gauge() -> Part {
    let outline_front = centered_cube(
        name("robot_keepout_front_probe_load_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let outline_rear = centered_cube(
        name("robot_keepout_rear_service_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let outline_left = centered_cube(
        name("robot_keepout_left_probe_load_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let outline_right = centered_cube(
        name("robot_keepout_right_probe_load_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    outline_front
        + outline_rear
        + outline_left
        + outline_right
        + keepout_clearance_lands()
        + keepout_corner_posts()
}

fn keepout_clearance_lands() -> Part {
    centered_cube(
        name(&format!(
            "robot_front_{ROBOT_APPROACH_CLEARANCE_MM:.0}mm_approach_clearance_land"
        )),
        260.0,
        18.0,
        4.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 34.0, KEEP_OUT_Z / 2.0 + 2.0)
        + centered_cube(
            name(&format!(
                "rear_{REAR_SERVICE_CLEARANCE_MM:.0}mm_service_clearance_land"
            )),
            260.0,
            18.0,
            4.0,
        )
        .translate(0.0, KEEP_OUT_Y / 2.0 - 34.0, KEEP_OUT_Z / 2.0 + 2.0)
        + centered_cube(
            name(&format!(
                "side_{SIDE_PROBE_LOAD_CLEARANCE_MM:.0}mm_probe_load_clearance_land"
            )),
            18.0,
            180.0,
            4.0,
        )
        .translate(KEEP_OUT_X / 2.0 - 34.0, 0.0, KEEP_OUT_Z / 2.0 + 2.0)
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(name("robot_keepout_corner_posts"));
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                name(&format!("robot_keepout_corner_post_{i}")),
                10.0,
                34.0,
                28,
            )
            .translate(*x, *y, 17.0);
    }
    posts
}

fn raised_label_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(name(label), x, y, 2.0);
    land + label_code_bars(label, x, y, code)
}

fn label_code_bars(label: &str, x: f64, y: f64, code: usize) -> Part {
    let mut bars = Part::empty(name(&format!("{label}_raised_code_bars")));
    for bit in 0..6 {
        let bar_h = if (code + bit) % 2 == 0 {
            y - 6.0
        } else {
            (y - 8.0) / 2.0
        };
        bars = bars
            + centered_cube(
                name(&format!("{label}_raised_code_bar_{bit}")),
                2.2,
                bar_h,
                2.0,
            )
            .translate(-x / 2.0 + 10.0 + bit as f64 * 5.2, 0.0, 2.0);
    }
    bars
}

fn datum_disc(label: &str) -> Part {
    let outer = centered_cylinder(name(&format!("{label}_outer_disc")), 13.0, 4.0, 36);
    let inner = centered_cylinder(name(&format!("{label}_inner_dot_cut")), 4.0, 5.0, 24);
    let xhair = centered_cube(name(&format!("{label}_crosshair_x")), 24.0, 2.0, 5.0);
    let yhair = centered_cube(name(&format!("{label}_crosshair_y")), 2.0, 24.0, 5.0);
    outer - inner + xhair + yhair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_scoped_to_generator() {
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
        assert_eq!(OUTPUTS.len(), 12);
    }

    #[test]
    fn requested_feature_groups_are_declared() {
        for feature in [
            "probe_nests",
            "rinse_soak_wells",
            "swab_slots",
            "residue_witness_coupons",
            "drying_rack",
            "leak_safe_tray",
            "labels_as_csg_geometry",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn wet_cleaning_counts_match_probe_families() {
        assert_eq!(PROBE_NEST_COUNT, SENSOR_FAMILIES * CLEAN_USED_LANES);
        assert_eq!(TOTAL_WELLS, RINSE_WELL_COUNT + SOAK_WELL_COUNT);
        assert_eq!(RINSE_WELL_COUNT, SENSOR_FAMILIES);
        assert_eq!(SOAK_WELL_COUNT, SENSOR_FAMILIES);
        assert_eq!(SEG_WELLS_PER_SIDE, SENSOR_FAMILIES);
    }

    #[test]
    fn layout_modules_fit_without_overlap() {
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit", rect.name);
        }
        for (i, a) in rects.iter().enumerate() {
            for b in rects.iter().skip(i + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
    }
}
