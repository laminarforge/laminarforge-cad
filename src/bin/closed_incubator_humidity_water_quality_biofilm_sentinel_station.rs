use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator humidity water-quality and biofilm sentinel station.
//
// This standalone generator models mechanical validation packaging for a closed
// incubator humidity utility loop. It represents a water pan surrogate, sterile
// refill interface, conductivity/TOC/coupon stations, condensate return witness
// path, drain/overflow capture, material compatibility coupons, barcode/custody
// status surfaces, robot/service keepouts, and a closed handoff to the
// incubator utility skid. It intentionally does not encode clinical acceptance
// thresholds or biological release criteria.

const OUTPUT_PREFIX: &str = "closed_incubator_humidity_water_quality_biofilm_sentinel_station";
const PARAMETRIC_REVISION: &str = "LF-CAD-CC-HWQ-BIOFILM-SENTINEL-REV-A";
const LAYOUT_REVISION_TOKEN: u64 = 0xC105_ED1A_864F_0959;

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_base_containment_deck.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_water_reservoir_pan_surrogate.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_sterile_refill_interface.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_conductivity_toc_station.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_sample_coupon_station.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_condensate_return_witness_path.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_drain_overflow_capture.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_material_compatibility_coupon_array.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_barcode_custody_status_surfaces.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_utility_skid_handoff_bulkhead.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_evidence_camera_bridge.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "water_reservoir_pan_surrogate",
    "sterile_refill_interface",
    "conductivity_toc_station",
    "sample_coupon_station",
    "condensate_return_witness_path",
    "drain_overflow_capture",
    "material_compatibility_coupon_array",
    "barcode_custody_status_surfaces",
    "robot_service_keepout_gauges",
    "utility_skid_handoff_bulkhead",
    "evidence_camera_bridge",
    "named_stl_outputs",
    "reproducibility_controls",
];

const REPRODUCIBILITY_CONTROLS: [&str; 5] = [
    "fixed_output_manifest",
    "stable_part_names",
    "constant_layout_rectangles",
    "no_random_geometry",
    "no_external_configuration",
];

const STATION_X: f64 = 1640.0;
const STATION_Y: f64 = 1000.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 50.0;
const BASIN_DEPTH: f64 = 10.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const PAN_X: f64 = 520.0;
const PAN_Y: f64 = 310.0;
const PAN_Z: f64 = 68.0;
const PAN_POS: (f64, f64) = (-485.0, 155.0);
const PAN_INNER_X: f64 = 422.0;
const PAN_INNER_Y: f64 = 214.0;
const PAN_INNER_DEPTH: f64 = 44.0;
const PAN_WORKING_DEPTH: f64 = 30.0;
const PAN_FILL_LEVEL_MARKS: usize = 5;
const PAN_BAFFLE_COUNT: usize = 6;
const PAN_SAMPLE_PORT_COUNT: usize = 4;
const PAN_ROBOT_HANDLE_COUNT: usize = 4;

const REFILL_X: f64 = 430.0;
const REFILL_Y: f64 = 130.0;
const REFILL_Z: f64 = 72.0;
const REFILL_POS: (f64, f64) = (-485.0, 395.0);
const QUICK_CONNECT_COUNT: usize = 4;
const STERILE_FILTER_COUNT: usize = 2;
const CAP_PARK_COUNT: usize = 4;
const REFILL_DRIP_TROUGH_COUNT: usize = 2;
const SEPTUM_PORT_COUNT: usize = 2;

const ANALYZER_X: f64 = 520.0;
const ANALYZER_Y: f64 = 260.0;
const ANALYZER_Z: f64 = 58.0;
const ANALYZER_POS: (f64, f64) = (210.0, 310.0);
const CONDUCTIVITY_CELL_COUNT: usize = 3;
const TOC_VIAL_COUNT: usize = 10;
const ANALYZER_SAMPLE_WELL_COUNT: usize = 6;
const ANALYZER_CABLE_CLIP_COUNT: usize = 6;
const FLOW_CELL_D: f64 = 38.0;
const TOC_VIAL_D: f64 = 24.0;

const SAMPLE_X: f64 = 460.0;
const SAMPLE_Y: f64 = 170.0;
const SAMPLE_Z: f64 = 42.0;
const SAMPLE_POS: (f64, f64) = (205.0, 60.0);
const SAMPLE_COUPON_RACKS: usize = 4;
const SAMPLE_COUPONS_PER_RACK: usize = 4;
const SAMPLE_COUPON_COUNT: usize = SAMPLE_COUPON_RACKS * SAMPLE_COUPONS_PER_RACK;
const SAMPLE_SWAB_LANE_COUNT: usize = 6;
const SAMPLE_WELL_COUNT: usize = 8;

const RETURN_X: f64 = 520.0;
const RETURN_Y: f64 = 135.0;
const RETURN_Z: f64 = 36.0;
const RETURN_POS: (f64, f64) = (-455.0, -125.0);
const CONDENSATE_WITNESS_WINDOW_COUNT: usize = 8;
const RETURN_CHANNEL_COUNT: usize = 4;
const RETURN_SLOPE_STEP_COUNT: usize = 5;
const RETURN_CHECK_VALVE_COUNT: usize = 2;

const DRAIN_X: f64 = 520.0;
const DRAIN_Y: f64 = 170.0;
const DRAIN_Z: f64 = 54.0;
const DRAIN_POS: (f64, f64) = (-455.0, -330.0);
const DRAIN_TRAP_COUNT: usize = 3;
const OVERFLOW_CUP_COUNT: usize = 4;
const OVERFLOW_WEIR_COUNT: usize = 4;
const DRAIN_SENSOR_WELL_COUNT: usize = 6;
const DRAIN_TRAP_D: f64 = 36.0;

const MATERIAL_X: f64 = 450.0;
const MATERIAL_Y: f64 = 135.0;
const MATERIAL_Z: f64 = 38.0;
const MATERIAL_POS: (f64, f64) = (200.0, -145.0);
const MATERIAL_CASSETTE_COUNT: usize = 3;
const MATERIAL_COUPONS_PER_CASSETTE: usize = 6;
const MATERIAL_COUPON_COUNT: usize = MATERIAL_CASSETTE_COUNT * MATERIAL_COUPONS_PER_CASSETTE;
const GASKET_WITNESS_COUNT: usize = 6;
const MATERIAL_ID_LAND_COUNT: usize = 6;

const STATUS_X: f64 = 500.0;
const STATUS_Y: f64 = 140.0;
const STATUS_Z: f64 = 18.0;
const STATUS_POS: (f64, f64) = (210.0, -335.0);
const BARCODE_LAND_COUNT: usize = 12;
const CUSTODY_SEAL_COUNT: usize = 6;
const STATUS_TILE_COUNT: usize = 9;
const RUN_RECORD_LAND_COUNT: usize = 4;
const BARCODE_BARS_PER_LAND: usize = 10;

const UTILITY_X: f64 = 170.0;
const UTILITY_Y: f64 = 780.0;
const UTILITY_Z: f64 = 92.0;
const UTILITY_POS: (f64, f64) = (650.0, 30.0);
const UTILITY_FLUID_CONNECTOR_COUNT: usize = 4;
const UTILITY_GAS_CONNECTOR_COUNT: usize = 2;
const UTILITY_ELECTRICAL_CONNECTOR_COUNT: usize = 2;
const UTILITY_CONNECTOR_COUNT: usize = UTILITY_FLUID_CONNECTOR_COUNT
    + UTILITY_GAS_CONNECTOR_COUNT
    + UTILITY_ELECTRICAL_CONNECTOR_COUNT;
const HANDOFF_LATCH_COUNT: usize = 4;
const HANDOFF_GASKET_COUNT: usize = 2;

const EVIDENCE_X: f64 = 1360.0;
const EVIDENCE_Y: f64 = 50.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CLEARANCE_Z: f64 = 220.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 468.0);
const EVIDENCE_CAMERA_COUNT: usize = 5;
const EVIDENCE_LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1550.0;
const KEEP_OUT_Y: f64 = 930.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE_Y: f64 = 310.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 245.0;
const UTILITY_SERVICE_CLEARANCE_X: f64 = 210.0;
const PAN_LIFT_CLEARANCE_Z: f64 = 330.0;
const REFILL_TUBE_BEND_CLEARANCE_Z: f64 = 155.0;
const KEEP_OUT_GAUGE_COUNT: usize = 7;

const LABEL_Z: f64 = 2.2;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 16.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 16.0;
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

    let pan = water_reservoir_pan_surrogate();
    export(OUTPUTS[1], &pan);

    let refill = sterile_refill_interface();
    export(OUTPUTS[2], &refill);

    let analyzer = conductivity_toc_station();
    export(OUTPUTS[3], &analyzer);

    let sample = sample_coupon_station();
    export(OUTPUTS[4], &sample);

    let condensate = condensate_return_witness_path();
    export(OUTPUTS[5], &condensate);

    let drain = drain_overflow_capture();
    export(OUTPUTS[6], &drain);

    let material = material_compatibility_coupon_array();
    export(OUTPUTS[7], &material);

    let status = barcode_custody_status_surfaces();
    export(OUTPUTS[8], &status);

    let utility = utility_skid_handoff_bulkhead();
    export(OUTPUTS[9], &utility);

    let evidence = evidence_camera_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + pan.translate(PAN_POS.0, PAN_POS.1, insert_z(PAN_Z))
        + refill.translate(REFILL_POS.0, REFILL_POS.1, insert_z(REFILL_Z))
        + analyzer.translate(ANALYZER_POS.0, ANALYZER_POS.1, insert_z(ANALYZER_Z))
        + sample.translate(SAMPLE_POS.0, SAMPLE_POS.1, insert_z(SAMPLE_Z))
        + condensate.translate(RETURN_POS.0, RETURN_POS.1, insert_z(RETURN_Z))
        + drain.translate(DRAIN_POS.0, DRAIN_POS.1, insert_z(DRAIN_Z))
        + material.translate(MATERIAL_POS.0, MATERIAL_POS.1, insert_z(MATERIAL_Z))
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + utility.translate(UTILITY_POS.0, UTILITY_POS.1, insert_z(UTILITY_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator humidity water-quality biofilm sentinel station:");
    println!(
        "  Revision:          {PARAMETRIC_REVISION} / layout token {LAYOUT_REVISION_TOKEN:#018x}"
    );
    println!(
        "  Footprint:         {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {:.0}mL fixture freeboard",
        containment_freeboard_ml()
    );
    println!(
        "  Water path:        pan surrogate with {PAN_FILL_LEVEL_MARKS} fill witness marks, {PAN_BAFFLE_COUNT} baffles, {PAN_SAMPLE_PORT_COUNT} sample ports, and {QUICK_CONNECT_COUNT} sterile refill connectors"
    );
    println!(
        "  Quality station:   {CONDUCTIVITY_CELL_COUNT} conductivity cells, {TOC_VIAL_COUNT} TOC vial positions, {SAMPLE_WELL_COUNT} sample wells, and {SAMPLE_COUPON_COUNT} sample coupon nests"
    );
    println!(
        "  Biofilm controls:  {CONDENSATE_WITNESS_WINDOW_COUNT} condensate witness windows, {DRAIN_TRAP_COUNT} drain traps, {OVERFLOW_CUP_COUNT} overflow cups, and {MATERIAL_COUPON_COUNT} material coupons"
    );
    println!(
        "  Trace/handoff:     {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_SEAL_COUNT} custody seals, {STATUS_TILE_COUNT} status tiles, {UTILITY_CONNECTOR_COUNT} utility-skid connectors"
    );
    println!(
        "  Keepouts:          {KEEP_OUT_GAUGE_COUNT} robot/service gauges with pan lift {:.0}mm Z and utility service {:.0}mm X clearances",
        PAN_LIFT_CLEARANCE_Z, UTILITY_SERVICE_CLEARANCE_X
    );
    println!("  Required features: {}", REQUIRED_FEATURES.len());
    println!(
        "  Scope limit:       validation fixture geometry only; no clinical acceptance thresholds or release criteria are encoded."
    );
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

fn name(label: &str) -> String {
    format!("{OUTPUT_PREFIX}_{label}")
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS.iter().all(|path| path
        .starts_with("output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_")
        && path.ends_with(".stl")));
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 5);
    assert_eq!(PARAMETRIC_REVISION, "LF-CAD-CC-HWQ-BIOFILM-SENTINEL-REV-A");
    assert_ne!(LAYOUT_REVISION_TOKEN, 0);
    assert_eq!(PAN_ROBOT_HANDLE_COUNT, 4);
    assert_eq!(QUICK_CONNECT_COUNT, CAP_PARK_COUNT);
    assert_eq!(UTILITY_CONNECTOR_COUNT, 8);
    assert_eq!(SAMPLE_COUPON_COUNT, 16);
    assert_eq!(MATERIAL_COUPON_COUNT, 18);
    assert_eq!(
        BARCODE_LAND_COUNT,
        ANALYZER_SAMPLE_WELL_COUNT + CUSTODY_SEAL_COUNT
    );
    assert_eq!(MOUNT_HOLE_COUNT, mount_hole_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_positions().len());
    assert!(PAN_WORKING_DEPTH < PAN_INNER_DEPTH);
    assert!(water_pan_working_volume_ml() > internal_witness_liquid_volume_ml());
    assert!(containment_freeboard_ml() > maximum_fixture_hold_up_ml());
    assert!(EVIDENCE_CLEARANCE_Z > PAN_Z + REFILL_Z);
    assert!(PAN_LIFT_CLEARANCE_Z > EVIDENCE_CLEARANCE_Z);
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE_Y);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE_Y);
    assert!(utility_service_clearance() >= UTILITY_SERVICE_CLEARANCE_X);

    for item in socket_rects() {
        assert!(item.fits_inside_deck(), "{} exceeds usable deck", item.name);
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
        rect("water_reservoir_pan_surrogate", PAN_POS, PAN_X, PAN_Y),
        rect("sterile_refill_interface", REFILL_POS, REFILL_X, REFILL_Y),
        rect(
            "conductivity_toc_station",
            ANALYZER_POS,
            ANALYZER_X,
            ANALYZER_Y,
        ),
        rect("sample_coupon_station", SAMPLE_POS, SAMPLE_X, SAMPLE_Y),
        rect(
            "condensate_return_witness_path",
            RETURN_POS,
            RETURN_X,
            RETURN_Y,
        ),
        rect("drain_overflow_capture", DRAIN_POS, DRAIN_X, DRAIN_Y),
        rect(
            "material_compatibility_coupon_array",
            MATERIAL_POS,
            MATERIAL_X,
            MATERIAL_Y,
        ),
        rect(
            "barcode_custody_status_surfaces",
            STATUS_POS,
            STATUS_X,
            STATUS_Y,
        ),
        rect(
            "utility_skid_handoff_bulkhead",
            UTILITY_POS,
            UTILITY_X,
            UTILITY_Y,
        ),
    ]
}

fn front_robot_clearance() -> f64 {
    ROBOT_FRONT_CLEARANCE_Y
}

fn rear_service_clearance() -> f64 {
    SERVICE_REAR_CLEARANCE_Y
}

fn utility_service_clearance() -> f64 {
    STATION_X / 2.0 - (UTILITY_POS.0 + UTILITY_X / 2.0) + UTILITY_SERVICE_CLEARANCE_X
}

fn water_pan_working_volume_ml() -> f64 {
    PAN_INNER_X * PAN_INNER_Y * PAN_WORKING_DEPTH / 1000.0
}

fn internal_witness_liquid_volume_ml() -> f64 {
    ANALYZER_SAMPLE_WELL_COUNT as f64 * 12.0
        + SAMPLE_WELL_COUNT as f64 * 10.0
        + CONDENSATE_WITNESS_WINDOW_COUNT as f64 * 8.0
        + DRAIN_TRAP_COUNT as f64 * 54.0
}

fn maximum_fixture_hold_up_ml() -> f64 {
    water_pan_working_volume_ml() * 0.48
        + OVERFLOW_CUP_COUNT as f64 * 95.0
        + DRAIN_SENSOR_WELL_COUNT as f64 * 14.0
        + internal_witness_liquid_volume_ml()
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        name("base_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        name("base_secondary_containment_basin_cut"),
        STATION_X - 2.0 * (RIM_W + 54.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(name("base_front_closed_drain_placeholder"), 9.0, 58.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -STATION_X / 2.0 + 136.0,
            -STATION_Y / 2.0 + 34.0,
            DECK_Z - 8.0,
        );

    deck - basin - front_drain - module_locator_sockets() - deck_mount_holes()
        + containment_rims()
        + deck_workflow_spines()
        + deck_datum_targets()
        + evidence_anchor_lands()
        + base_flow_witness_ribs()
}

fn module_locator_sockets() -> Part {
    let mut sockets = Part::empty(name("module_locator_sockets"));
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("{}_locator_socket", item.name)),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(name("deck_mount_holes"));
    for (i, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                name(&format!("m6_mount_clearance_{i}")),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                name(&format!("m6_mount_slot_relief_{i}")),
                32.0,
                8.0,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (-STATION_X / 2.0 + 62.0, 0.0),
        (STATION_X / 2.0 - 62.0, 0.0),
        (UTILITY_POS.0, UTILITY_POS.1 - UTILITY_Y / 2.0 - 32.0),
        (REFILL_POS.0, REFILL_POS.1 + REFILL_Y / 2.0 + 22.0),
    ]
}

fn containment_rims() -> Part {
    let front = centered_cube(name("front_low_robot_splash_lip"), STATION_X, RIM_W, 30.0)
        .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 15.0);
    let rear = centered_cube(
        name("rear_service_high_splash_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(name("left_spill_retention_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(name("right_utility_service_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn deck_workflow_spines() -> Part {
    let clean_refill_lane = centered_cube(
        name("clean_refill_to_pan_workflow_spine"),
        10.0,
        840.0,
        22.0,
    )
    .translate(-190.0, 80.0, DECK_Z + 11.0);
    let analyzer_lane = centered_cube(name("sample_to_analyzer_workflow_spine"), 780.0, 8.0, 20.0)
        .translate(0.0, 162.0, DECK_Z + 10.0);
    let return_lane = centered_cube(
        name("condensate_return_to_drain_workflow_spine"),
        710.0,
        8.0,
        20.0,
    )
    .translate(-455.0, -222.0, DECK_Z + 10.0);
    let utility_lane = centered_cube(
        name("closed_utility_skid_handoff_workflow_spine"),
        8.0,
        790.0,
        24.0,
    )
    .translate(548.0, 30.0, DECK_Z + 12.0);

    clean_refill_lane + analyzer_lane + return_lane + utility_lane
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty(name("deck_robot_datum_targets"));
    for (i, (x, y)) in datum_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_target(&format!("deck_datum_target_{i}")).translate(x, y, DECK_Z + 2.5);
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 100.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 100.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 100.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 100.0),
    ]
}

fn evidence_anchor_lands() -> Part {
    let left = centered_cube(name("evidence_bridge_left_anchor_land"), 104.0, 42.0, 8.0).translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0 + 70.0,
        EVIDENCE_POS.1,
        DECK_Z + 4.0,
    );
    let right = centered_cube(name("evidence_bridge_right_anchor_land"), 104.0, 42.0, 8.0)
        .translate(
            EVIDENCE_POS.0 + EVIDENCE_X / 2.0 - 70.0,
            EVIDENCE_POS.1,
            DECK_Z + 4.0,
        );
    left + right
}

fn base_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty(name("base_flow_grade_witness_ribs"));
    for (i, y) in [-420.0, -350.0, -235.0, -90.0, 80.0, 245.0, 380.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                name(&format!("base_flow_grade_witness_rib_{i}")),
                STATION_X - 260.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn water_reservoir_pan_surrogate() -> Part {
    let shell = centered_cube(name("water_pan_surrogate_outer_shell"), PAN_X, PAN_Y, PAN_Z);
    let basin = centered_cube(
        name("water_pan_surrogate_internal_basin_cut"),
        PAN_INNER_X,
        PAN_INNER_Y,
        PAN_INNER_DEPTH + 0.8,
    )
    .translate(0.0, 8.0, PAN_Z / 2.0 - PAN_INNER_DEPTH / 2.0 + 0.3);
    let overflow_notch = centered_cube(
        name("water_pan_surrogate_overflow_notch_cut"),
        110.0,
        12.0,
        28.0,
    )
    .translate(PAN_X / 2.0 - 38.0, -PAN_Y / 2.0 + 42.0, PAN_Z / 2.0 - 18.0);
    let drain_bore = centered_cylinder(name("water_pan_surrogate_drain_barb_bore"), 7.5, 64.0, 28)
        .rotate(0.0, 90.0, 0.0)
        .translate(
            -PAN_X / 2.0 + 28.0,
            -PAN_Y / 2.0 + 45.0,
            -PAN_Z / 2.0 + 22.0,
        );

    shell - basin - overflow_notch - drain_bore
        + pan_baffles()
        + pan_fill_level_marks()
        + pan_sample_ports()
        + pan_robot_handles()
        + pan_locator_feet()
}

fn pan_baffles() -> Part {
    let mut baffles = Part::empty(name("water_pan_surrogate_splash_baffles"));
    for i in 0..PAN_BAFFLE_COUNT {
        let x = centered_index(i, PAN_BAFFLE_COUNT, 62.0);
        let baffle = centered_cube(name(&format!("water_pan_baffle_{i}")), 8.0, 184.0, 36.0)
            .translate(x, 8.0, 8.0);
        let drain_slot = centered_cube(
            name(&format!("water_pan_baffle_low_slot_cut_{i}")),
            10.0,
            28.0,
            14.0,
        )
        .translate(x, -PAN_INNER_Y / 2.0 + 44.0, -10.0);
        baffles = baffles + (baffle - drain_slot);
    }
    baffles
}

fn pan_fill_level_marks() -> Part {
    let mut marks = Part::empty(name("water_pan_fill_level_witness_marks"));
    for i in 0..PAN_FILL_LEVEL_MARKS {
        marks = marks
            + centered_cube(
                name(&format!("water_pan_fill_level_mark_{i}")),
                52.0,
                3.2,
                4.0,
            )
            .translate(
                -PAN_X / 2.0 + 44.0,
                -PAN_Y / 2.0 + 78.0 + i as f64 * 32.0,
                PAN_Z / 2.0 + 2.0,
            );
    }
    marks
}

fn pan_sample_ports() -> Part {
    let mut ports = Part::empty(name("water_pan_sample_port_witnesses"));
    for i in 0..PAN_SAMPLE_PORT_COUNT {
        let x = centered_index(i, PAN_SAMPLE_PORT_COUNT, 74.0);
        let boss = centered_cylinder(
            name(&format!("water_pan_sample_port_boss_{i}")),
            17.0,
            10.0,
            32,
        )
        .translate(x, PAN_Y / 2.0 - 32.0, PAN_Z / 2.0 + 5.0);
        let bore = centered_cylinder(
            name(&format!("water_pan_sample_port_bore_{i}")),
            5.0,
            12.0,
            24,
        )
        .translate(x, PAN_Y / 2.0 - 32.0, PAN_Z / 2.0 + 5.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn pan_robot_handles() -> Part {
    let mut handles = Part::empty(name("water_pan_robot_lift_handles"));
    for (i, (x, y)) in [
        (-PAN_X / 2.0 + 58.0, -PAN_Y / 2.0 + 54.0),
        (PAN_X / 2.0 - 58.0, -PAN_Y / 2.0 + 54.0),
        (-PAN_X / 2.0 + 58.0, PAN_Y / 2.0 - 54.0),
        (PAN_X / 2.0 - 58.0, PAN_Y / 2.0 - 54.0),
    ]
    .into_iter()
    .enumerate()
    {
        let lug = centered_cube(
            name(&format!("water_pan_robot_handle_lug_{i}")),
            58.0,
            20.0,
            16.0,
        )
        .translate(x, y, PAN_Z / 2.0 + 8.0);
        let slot = centered_cube(
            name(&format!("water_pan_robot_handle_slot_cut_{i}")),
            34.0,
            8.0,
            18.0,
        )
        .translate(x, y, PAN_Z / 2.0 + 8.0);
        handles = handles + (lug - slot);
    }
    handles
}

fn pan_locator_feet() -> Part {
    let mut feet = Part::empty(name("water_pan_socket_locator_feet"));
    for (i, (x, y)) in [
        (-PAN_X / 2.0 + 60.0, -PAN_Y / 2.0 + 46.0),
        (PAN_X / 2.0 - 60.0, -PAN_Y / 2.0 + 46.0),
        (-PAN_X / 2.0 + 60.0, PAN_Y / 2.0 - 46.0),
        (PAN_X / 2.0 - 60.0, PAN_Y / 2.0 - 46.0),
    ]
    .into_iter()
    .enumerate()
    {
        feet = feet
            + centered_cylinder(name(&format!("water_pan_locator_foot_{i}")), 12.0, 10.0, 28)
                .translate(x, y, -PAN_Z / 2.0 - 5.0);
    }
    feet
}

fn sterile_refill_interface() -> Part {
    let body = centered_cube(
        name("sterile_refill_interface_body"),
        REFILL_X,
        REFILL_Y,
        REFILL_Z,
    );
    let inlet_face = centered_cube(
        name("sterile_refill_interface_keyed_connector_face"),
        REFILL_X - 70.0,
        18.0,
        64.0,
    )
    .translate(0.0, -REFILL_Y / 2.0 + 16.0, 2.0);
    let trough_cut = centered_cube(
        name("sterile_refill_interface_drip_trough_cut"),
        REFILL_X - 92.0,
        34.0,
        16.0,
    )
    .translate(0.0, -REFILL_Y / 2.0 + 44.0, -REFILL_Z / 2.0 + 10.0);

    body - trough_cut
        + inlet_face
        + quick_connects()
        + refill_filter_bosses()
        + refill_cap_parks()
        + septum_ports()
        + refill_status_lands()
}

fn quick_connects() -> Part {
    let mut connects = Part::empty(name("sterile_refill_quick_connects"));
    for i in 0..QUICK_CONNECT_COUNT {
        let x = centered_index(i, QUICK_CONNECT_COUNT, 70.0);
        let collar = centered_cylinder(
            name(&format!("sterile_refill_qc_keyed_collar_{i}")),
            18.0,
            18.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -REFILL_Y / 2.0 + 6.0, 8.0);
        let bore = centered_cylinder(name(&format!("sterile_refill_qc_bore_{i}")), 6.0, 22.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -REFILL_Y / 2.0 + 6.0, 8.0);
        let key = centered_cube(
            name(&format!("sterile_refill_qc_key_flat_{i}")),
            10.0,
            5.0,
            28.0,
        )
        .translate(x + 18.0, -REFILL_Y / 2.0 + 4.0, 8.0);
        connects = connects + (collar - bore) + key;
    }
    connects
}

fn refill_filter_bosses() -> Part {
    let mut filters = Part::empty(name("sterile_refill_inline_filter_placeholders"));
    for i in 0..STERILE_FILTER_COUNT {
        let x = centered_index(i, STERILE_FILTER_COUNT, 140.0);
        filters = filters
            + centered_cylinder(
                name(&format!("sterile_refill_filter_capsule_{i}")),
                18.0,
                98.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, 10.0, REFILL_Z / 2.0 + 16.0)
            + centered_cube(
                name(&format!("sterile_refill_filter_saddle_{i}")),
                104.0,
                18.0,
                12.0,
            )
            .translate(x, 10.0, REFILL_Z / 2.0 + 6.0);
    }
    filters
}

fn refill_cap_parks() -> Part {
    let mut parks = Part::empty(name("sterile_refill_cap_parks"));
    for i in 0..CAP_PARK_COUNT {
        let x = centered_index(i, CAP_PARK_COUNT, 70.0);
        let park = centered_cylinder(name(&format!("sterile_refill_cap_park_{i}")), 14.0, 7.0, 30)
            .translate(x, REFILL_Y / 2.0 - 26.0, REFILL_Z / 2.0 + 3.5);
        let tether = centered_cube(
            name(&format!("sterile_refill_cap_tether_slot_{i}")),
            28.0,
            4.0,
            8.0,
        )
        .translate(x, REFILL_Y / 2.0 - 48.0, REFILL_Z / 2.0 + 4.0);
        parks = parks + park + tether;
    }
    parks
}

fn septum_ports() -> Part {
    let mut ports = Part::empty(name("sterile_refill_sample_septum_ports"));
    for i in 0..SEPTUM_PORT_COUNT {
        let x = centered_index(i, SEPTUM_PORT_COUNT, 78.0) + 126.0;
        let boss = centered_cylinder(
            name(&format!("sterile_refill_septum_boss_{i}")),
            16.0,
            10.0,
            30,
        )
        .translate(x, 8.0, REFILL_Z / 2.0 + 5.0);
        let target = fiducial_target(&format!("sterile_refill_septum_target_{i}")).translate(
            x,
            8.0,
            REFILL_Z / 2.0 + 11.0,
        );
        ports = ports + boss + target;
    }
    ports
}

fn refill_status_lands() -> Part {
    let mut lands = Part::empty(name("sterile_refill_status_lands"));
    for i in 0..REFILL_DRIP_TROUGH_COUNT {
        lands = lands
            + barcode_land(
                &format!("sterile_refill_drip_trough_status_land_{i}"),
                94.0,
                20.0,
                i + 11,
            )
            .translate(
                centered_index(i, REFILL_DRIP_TROUGH_COUNT, 118.0) - 116.0,
                44.0,
                REFILL_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn conductivity_toc_station() -> Part {
    let plate = centered_cube(
        name("conductivity_toc_station_sensor_plate"),
        ANALYZER_X,
        ANALYZER_Y,
        ANALYZER_Z,
    );
    let shallow_tray = centered_cube(
        name("conductivity_toc_station_shallow_spill_recess_cut"),
        ANALYZER_X - 52.0,
        ANALYZER_Y - 44.0,
        14.0,
    )
    .translate(0.0, 0.0, ANALYZER_Z / 2.0 - 7.0);

    plate - shallow_tray - analyzer_well_cuts()
        + conductivity_flow_cells()
        + toc_vial_carousel()
        + analyzer_sample_wells()
        + analyzer_cable_clips()
        + analyzer_barcode_lands()
}

fn conductivity_flow_cells() -> Part {
    let mut cells = Part::empty(name("conductivity_flow_cells"));
    for i in 0..CONDUCTIVITY_CELL_COUNT {
        let x = -ANALYZER_X / 2.0 + 82.0 + i as f64 * 76.0;
        let block = centered_cube(
            name(&format!("conductivity_flow_cell_block_{i}")),
            56.0,
            84.0,
            32.0,
        )
        .translate(x, -50.0, ANALYZER_Z / 2.0 + 16.0);
        let bore = centered_cylinder(
            name(&format!("conductivity_flow_cell_bore_{i}")),
            8.0,
            90.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -50.0, ANALYZER_Z / 2.0 + 16.0);
        let probe = centered_cylinder(
            name(&format!("conductivity_probe_pocket_{i}")),
            6.0,
            38.0,
            24,
        )
        .translate(x, -50.0, ANALYZER_Z / 2.0 + 30.0);
        cells = cells + (block - bore - probe);
    }
    cells
}

fn toc_vial_carousel() -> Part {
    let mut carousel = Part::empty(name("toc_vial_carousel"));
    let hub = centered_cylinder(name("toc_vial_carousel_center_hub"), 34.0, 18.0, 40).translate(
        96.0,
        30.0,
        ANALYZER_Z / 2.0 + 9.0,
    );
    carousel = carousel + hub;
    for i in 0..TOC_VIAL_COUNT {
        let angle = i as f64 * 360.0 / TOC_VIAL_COUNT as f64;
        let x = 96.0 + angle.to_radians().cos() * 82.0;
        let y = 30.0 + angle.to_radians().sin() * 64.0;
        let nest = centered_cylinder(
            name(&format!("toc_vial_nest_{i}")),
            TOC_VIAL_D / 2.0 + 4.0,
            16.0,
            32,
        )
        .translate(x, y, ANALYZER_Z / 2.0 + 8.0);
        let cut = centered_cylinder(
            name(&format!("toc_vial_pocket_cut_{i}")),
            TOC_VIAL_D / 2.0,
            18.0,
            28,
        )
        .translate(x, y, ANALYZER_Z / 2.0 + 8.0);
        carousel = carousel + (nest - cut);
    }
    carousel
}

fn analyzer_sample_wells() -> Part {
    let mut wells = Part::empty(name("analyzer_sample_well_lands"));
    for i in 0..ANALYZER_SAMPLE_WELL_COUNT {
        let x = centered_index(i, ANALYZER_SAMPLE_WELL_COUNT, 52.0) + 12.0;
        let rim = centered_cylinder(
            name(&format!("analyzer_sample_well_rim_{i}")),
            FLOW_CELL_D / 2.0,
            12.0,
            32,
        )
        .translate(x, ANALYZER_Y / 2.0 - 42.0, ANALYZER_Z / 2.0 + 6.0);
        let cut = centered_cylinder(
            name(&format!("analyzer_sample_well_pocket_cut_{i}")),
            FLOW_CELL_D / 2.0 - 6.0,
            14.0,
            28,
        )
        .translate(x, ANALYZER_Y / 2.0 - 42.0, ANALYZER_Z / 2.0 + 6.0);
        wells = wells + (rim - cut);
    }
    wells
}

fn analyzer_cable_clips() -> Part {
    let mut clips = Part::empty(name("analyzer_cable_retention_clips"));
    for i in 0..ANALYZER_CABLE_CLIP_COUNT {
        let x = centered_index(i, ANALYZER_CABLE_CLIP_COUNT, 54.0);
        let clip = centered_cube(name(&format!("analyzer_cable_clip_{i}")), 34.0, 10.0, 16.0)
            .translate(x, -ANALYZER_Y / 2.0 + 22.0, ANALYZER_Z / 2.0 + 8.0);
        let relief = centered_cube(
            name(&format!("analyzer_cable_clip_relief_{i}")),
            20.0,
            12.0,
            8.0,
        )
        .translate(x, -ANALYZER_Y / 2.0 + 22.0, ANALYZER_Z / 2.0 + 8.0);
        clips = clips + (clip - relief);
    }
    clips
}

fn analyzer_barcode_lands() -> Part {
    let mut lands = Part::empty(name("analyzer_local_barcode_lands"));
    for i in 0..4 {
        lands = lands
            + barcode_land(&format!("analyzer_barcode_land_{i}"), 76.0, 20.0, i + 23).translate(
                -ANALYZER_X / 2.0 + 72.0 + i as f64 * 90.0,
                90.0,
                ANALYZER_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn analyzer_well_cuts() -> Part {
    let mut cuts = Part::empty(name("analyzer_floor_drainage_relief_cuts"));
    for i in 0..4 {
        cuts = cuts
            + centered_cube(
                name(&format!("analyzer_floor_drainage_relief_cut_{i}")),
                86.0,
                5.0,
                12.0,
            )
            .translate(centered_index(i, 4, 110.0), -96.0, ANALYZER_Z / 2.0 - 4.0);
    }
    cuts
}

fn sample_coupon_station() -> Part {
    let plate = centered_cube(
        name("sample_coupon_station_base_plate"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let tray_cut = centered_cube(
        name("sample_coupon_station_sample_recess_cut"),
        SAMPLE_X - 50.0,
        SAMPLE_Y - 36.0,
        12.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0 - 6.0);

    plate - tray_cut - sample_coupon_slot_cuts()
        + sample_coupon_rack_rails()
        + sample_swab_lanes()
        + sample_well_rims()
        + coupon_chain_index_tabs()
}

fn sample_coupon_rack_rails() -> Part {
    let mut rails = Part::empty(name("sample_coupon_rack_rails"));
    for rack in 0..SAMPLE_COUPON_RACKS {
        let x = centered_index(rack, SAMPLE_COUPON_RACKS, 92.0);
        rails = rails
            + centered_cube(
                name(&format!("sample_coupon_rack_{rack}_left_rail")),
                8.0,
                116.0,
                18.0,
            )
            .translate(x - 28.0, -8.0, SAMPLE_Z / 2.0 + 9.0)
            + centered_cube(
                name(&format!("sample_coupon_rack_{rack}_right_rail")),
                8.0,
                116.0,
                18.0,
            )
            .translate(x + 28.0, -8.0, SAMPLE_Z / 2.0 + 9.0);
    }
    rails
}

fn sample_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("sample_coupon_slot_cuts"));
    for rack in 0..SAMPLE_COUPON_RACKS {
        for slot in 0..SAMPLE_COUPONS_PER_RACK {
            let x = centered_index(rack, SAMPLE_COUPON_RACKS, 92.0);
            let y = centered_index(slot, SAMPLE_COUPONS_PER_RACK, 24.0) - 8.0;
            cuts = cuts
                + centered_cube(
                    name(&format!("sample_coupon_rack_{rack}_slot_{slot}_cut")),
                    42.0,
                    16.0,
                    20.0,
                )
                .translate(x, y, SAMPLE_Z / 2.0 + 4.0);
        }
    }
    cuts
}

fn sample_swab_lanes() -> Part {
    let mut lanes = Part::empty(name("sample_swab_lane_witnesses"));
    for i in 0..SAMPLE_SWAB_LANE_COUNT {
        lanes = lanes
            + centered_cube(name(&format!("sample_swab_lane_{i}")), 54.0, 10.0, 6.0).translate(
                -SAMPLE_X / 2.0 + 54.0 + i as f64 * 66.0,
                SAMPLE_Y / 2.0 - 28.0,
                SAMPLE_Z / 2.0 + 3.0,
            );
    }
    lanes
}

fn sample_well_rims() -> Part {
    let mut wells = Part::empty(name("sample_coupon_station_well_rims"));
    for i in 0..SAMPLE_WELL_COUNT {
        let x = -SAMPLE_X / 2.0 + 36.0 + i as f64 * 54.0;
        let rim = centered_cylinder(
            name(&format!("sample_coupon_station_well_rim_{i}")),
            16.0,
            9.0,
            30,
        )
        .translate(x, -SAMPLE_Y / 2.0 + 30.0, SAMPLE_Z / 2.0 + 4.5);
        let cut = centered_cylinder(
            name(&format!("sample_coupon_station_well_cut_{i}")),
            10.5,
            10.0,
            24,
        )
        .translate(x, -SAMPLE_Y / 2.0 + 30.0, SAMPLE_Z / 2.0 + 4.5);
        wells = wells + (rim - cut);
    }
    wells
}

fn coupon_chain_index_tabs() -> Part {
    let mut tabs = Part::empty(name("sample_coupon_chain_index_tabs"));
    for i in 0..SAMPLE_COUPON_RACKS {
        tabs = tabs
            + barcode_land(
                &format!("sample_coupon_rack_index_tab_{i}"),
                54.0,
                18.0,
                i + 41,
            )
            .translate(
                centered_index(i, SAMPLE_COUPON_RACKS, 92.0),
                -SAMPLE_Y / 2.0 + 70.0,
                SAMPLE_Z / 2.0 + LABEL_Z,
            );
    }
    tabs
}

fn condensate_return_witness_path() -> Part {
    let plate = centered_cube(
        name("condensate_return_witness_base_plate"),
        RETURN_X,
        RETURN_Y,
        RETURN_Z,
    );
    let channel_cut = centered_cube(
        name("condensate_return_primary_channel_cut"),
        RETURN_X - 70.0,
        24.0,
        16.0,
    )
    .translate(0.0, 0.0, RETURN_Z / 2.0 - 8.0);
    let bypass_cut = centered_cube(
        name("condensate_return_bypass_channel_cut"),
        RETURN_X - 150.0,
        18.0,
        14.0,
    )
    .translate(0.0, 42.0, RETURN_Z / 2.0 - 8.0);

    plate - channel_cut - bypass_cut
        + condensate_witness_windows()
        + condensate_return_tubes()
        + return_slope_steps()
        + return_check_valves()
}

fn condensate_witness_windows() -> Part {
    let mut windows = Part::empty(name("condensate_return_witness_windows"));
    for i in 0..CONDENSATE_WITNESS_WINDOW_COUNT {
        windows = windows
            + centered_cube(
                name(&format!("condensate_return_witness_window_land_{i}")),
                42.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, CONDENSATE_WITNESS_WINDOW_COUNT, 52.0),
                -34.0,
                RETURN_Z / 2.0 + 2.5,
            )
            + barcode_land(
                &format!("condensate_window_index_code_{i}"),
                32.0,
                12.0,
                i + 53,
            )
            .translate(
                centered_index(i, CONDENSATE_WITNESS_WINDOW_COUNT, 52.0),
                -34.0,
                RETURN_Z / 2.0 + 8.0,
            );
    }
    windows
}

fn condensate_return_tubes() -> Part {
    let mut tubes = Part::empty(name("condensate_return_tube_spans"));
    for i in 0..RETURN_CHANNEL_COUNT {
        let y = centered_index(i, RETURN_CHANNEL_COUNT, 28.0);
        tubes =
            tubes
                + tube_span_x(&format!("condensate_return_tube_span_{i}"), RETURN_X - 88.0)
                    .translate(0.0, y, RETURN_Z / 2.0 + 18.0);
    }
    tubes
}

fn return_slope_steps() -> Part {
    let mut steps = Part::empty(name("condensate_return_slope_step_witnesses"));
    for i in 0..RETURN_SLOPE_STEP_COUNT {
        steps = steps
            + centered_cube(
                name(&format!("condensate_return_slope_step_{i}")),
                76.0,
                10.0,
                5.0 + i as f64,
            )
            .translate(
                -RETURN_X / 2.0 + 80.0 + i as f64 * 90.0,
                RETURN_Y / 2.0 - 22.0,
                RETURN_Z / 2.0 + 3.0 + i as f64 * 0.5,
            );
    }
    steps
}

fn return_check_valves() -> Part {
    let mut valves = Part::empty(name("condensate_return_check_valve_placeholders"));
    for i in 0..RETURN_CHECK_VALVE_COUNT {
        valves = valves
            + centered_cylinder(
                name(&format!("condensate_return_check_valve_body_{i}")),
                16.0,
                58.0,
                30,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                centered_index(i, RETURN_CHECK_VALVE_COUNT, 280.0),
                4.0,
                RETURN_Z / 2.0 + 26.0,
            );
    }
    valves
}

fn drain_overflow_capture() -> Part {
    let tray = centered_cube(
        name("drain_overflow_capture_tray"),
        DRAIN_X,
        DRAIN_Y,
        DRAIN_Z,
    );
    let gutter_cut = centered_cube(
        name("drain_overflow_capture_long_gutter_cut"),
        DRAIN_X - 64.0,
        38.0,
        22.0,
    )
    .translate(0.0, 20.0, DRAIN_Z / 2.0 - 11.0);
    let low_channel_cut = centered_cube(
        name("drain_overflow_capture_low_channel_cut"),
        DRAIN_X - 118.0,
        22.0,
        18.0,
    )
    .translate(0.0, -52.0, DRAIN_Z / 2.0 - 9.0);

    tray - gutter_cut - low_channel_cut
        + drain_traps()
        + overflow_capture_cups()
        + overflow_weirs()
        + drain_sensor_wells()
}

fn drain_traps() -> Part {
    let mut traps = Part::empty(name("drain_trap_witnesses"));
    for i in 0..DRAIN_TRAP_COUNT {
        let x = centered_index(i, DRAIN_TRAP_COUNT, 118.0);
        let trap = centered_cylinder(
            name(&format!("drain_trap_body_{i}")),
            DRAIN_TRAP_D / 2.0,
            34.0,
            32,
        )
        .translate(x, -48.0, DRAIN_Z / 2.0 + 17.0);
        let well = centered_cylinder(
            name(&format!("drain_trap_center_well_cut_{i}")),
            10.0,
            36.0,
            24,
        )
        .translate(x, -48.0, DRAIN_Z / 2.0 + 17.0);
        let standpipe = centered_cylinder(
            name(&format!("drain_trap_standpipe_witness_{i}")),
            5.0,
            44.0,
            22,
        )
        .translate(x + 24.0, -48.0, DRAIN_Z / 2.0 + 22.0);
        traps = traps + (trap - well) + standpipe;
    }
    traps
}

fn overflow_capture_cups() -> Part {
    let mut cups = Part::empty(name("overflow_capture_cups"));
    for i in 0..OVERFLOW_CUP_COUNT {
        let x = centered_index(i, OVERFLOW_CUP_COUNT, 82.0);
        let outer = centered_cylinder(name(&format!("overflow_capture_cup_{i}")), 18.0, 24.0, 32)
            .translate(x, DRAIN_Y / 2.0 - 34.0, DRAIN_Z / 2.0 + 12.0);
        let inner = centered_cylinder(
            name(&format!("overflow_capture_cup_pocket_cut_{i}")),
            12.0,
            26.0,
            28,
        )
        .translate(x, DRAIN_Y / 2.0 - 34.0, DRAIN_Z / 2.0 + 12.0);
        cups = cups + (outer - inner);
    }
    cups
}

fn overflow_weirs() -> Part {
    let mut weirs = Part::empty(name("overflow_weir_witness_tabs"));
    for i in 0..OVERFLOW_WEIR_COUNT {
        weirs = weirs
            + centered_cube(
                name(&format!("overflow_weir_height_witness_{i}")),
                56.0,
                8.0,
                20.0 + i as f64 * 4.0,
            )
            .translate(
                centered_index(i, OVERFLOW_WEIR_COUNT, 86.0),
                14.0,
                DRAIN_Z / 2.0 + 10.0 + i as f64 * 2.0,
            );
    }
    weirs
}

fn drain_sensor_wells() -> Part {
    let mut wells = Part::empty(name("drain_overflow_sensor_wells"));
    for i in 0..DRAIN_SENSOR_WELL_COUNT {
        let x = centered_index(i, DRAIN_SENSOR_WELL_COUNT, 58.0);
        let well = centered_cylinder(name(&format!("drain_sensor_well_rim_{i}")), 13.0, 8.0, 30)
            .translate(x, -DRAIN_Y / 2.0 + 28.0, DRAIN_Z / 2.0 + 4.0);
        let cut = centered_cylinder(
            name(&format!("drain_sensor_well_pocket_cut_{i}")),
            8.0,
            9.0,
            24,
        )
        .translate(x, -DRAIN_Y / 2.0 + 28.0, DRAIN_Z / 2.0 + 4.0);
        wells = wells + (well - cut);
    }
    wells
}

fn material_compatibility_coupon_array() -> Part {
    let plate = centered_cube(
        name("material_compatibility_coupon_array_plate"),
        MATERIAL_X,
        MATERIAL_Y,
        MATERIAL_Z,
    );
    let wash_channel = centered_cube(
        name("material_compatibility_condensate_exposure_channel_cut"),
        MATERIAL_X - 72.0,
        24.0,
        14.0,
    )
    .translate(0.0, 2.0, MATERIAL_Z / 2.0 - 7.0);

    plate - wash_channel - material_coupon_slot_cuts()
        + material_coupon_cassette_rails()
        + gasket_witness_pockets()
        + material_id_lands()
}

fn material_coupon_cassette_rails() -> Part {
    let mut rails = Part::empty(name("material_coupon_cassette_rails"));
    for cassette in 0..MATERIAL_CASSETTE_COUNT {
        let x = centered_index(cassette, MATERIAL_CASSETTE_COUNT, 126.0);
        rails = rails
            + centered_cube(
                name(&format!("material_coupon_cassette_{cassette}_front_rail")),
                96.0,
                8.0,
                16.0,
            )
            .translate(x, -42.0, MATERIAL_Z / 2.0 + 8.0)
            + centered_cube(
                name(&format!("material_coupon_cassette_{cassette}_rear_rail")),
                96.0,
                8.0,
                16.0,
            )
            .translate(x, 42.0, MATERIAL_Z / 2.0 + 8.0);
    }
    rails
}

fn material_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("material_coupon_slot_cuts"));
    for cassette in 0..MATERIAL_CASSETTE_COUNT {
        for slot in 0..MATERIAL_COUPONS_PER_CASSETTE {
            let x = centered_index(cassette, MATERIAL_CASSETTE_COUNT, 126.0)
                + centered_index(slot % 3, 3, 30.0);
            let y = if slot < 3 { -22.0 } else { 22.0 };
            cuts = cuts
                + centered_cube(
                    name(&format!(
                        "material_coupon_cassette_{cassette}_slot_{slot}_cut"
                    )),
                    24.0,
                    18.0,
                    18.0,
                )
                .translate(x, y, MATERIAL_Z / 2.0 + 4.0);
        }
    }
    cuts
}

fn gasket_witness_pockets() -> Part {
    let mut pockets = Part::empty(name("gasket_witness_pockets"));
    for i in 0..GASKET_WITNESS_COUNT {
        let x = centered_index(i, GASKET_WITNESS_COUNT, 56.0);
        let rim = centered_cylinder(
            name(&format!("gasket_witness_pocket_rim_{i}")),
            14.0,
            8.0,
            30,
        )
        .translate(x, -MATERIAL_Y / 2.0 + 22.0, MATERIAL_Z / 2.0 + 4.0);
        let cut = centered_cylinder(
            name(&format!("gasket_witness_pocket_cut_{i}")),
            8.0,
            10.0,
            24,
        )
        .translate(x, -MATERIAL_Y / 2.0 + 22.0, MATERIAL_Z / 2.0 + 4.0);
        pockets = pockets + (rim - cut);
    }
    pockets
}

fn material_id_lands() -> Part {
    let mut lands = Part::empty(name("material_id_lands"));
    for i in 0..MATERIAL_ID_LAND_COUNT {
        lands = lands
            + barcode_land(&format!("material_id_land_{i}"), 52.0, 15.0, i + 67).translate(
                centered_index(i, MATERIAL_ID_LAND_COUNT, 58.0),
                MATERIAL_Y / 2.0 - 20.0,
                MATERIAL_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn barcode_custody_status_surfaces() -> Part {
    let plate = centered_cube(
        name("barcode_custody_status_plate"),
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    plate + barcode_custody_lands() + custody_seal_lands() + status_tiles() + run_record_lands()
}

fn barcode_custody_lands() -> Part {
    let mut lands = Part::empty(name("barcode_custody_lands"));
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 6;
        let col = i % 6;
        lands = lands
            + barcode_land(&format!("custody_barcode_land_{i}"), 62.0, 18.0, i + 79).translate(
                -STATUS_X / 2.0 + 54.0 + col as f64 * 72.0,
                -38.0 + row as f64 * 28.0,
                STATUS_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty(name("custody_seal_lands"));
    for i in 0..CUSTODY_SEAL_COUNT {
        let x = centered_index(i, CUSTODY_SEAL_COUNT, 66.0);
        let seal = centered_cube(
            name(&format!("custody_tamper_seal_land_{i}")),
            46.0,
            18.0,
            4.0,
        )
        .translate(x, STATUS_Y / 2.0 - 26.0, STATUS_Z / 2.0 + 2.0);
        let notch = centered_cube(
            name(&format!("custody_tamper_seal_notch_{i}")),
            12.0,
            20.0,
            5.0,
        )
        .translate(x, STATUS_Y / 2.0 - 26.0, STATUS_Z / 2.0 + 2.0);
        seals = seals + (seal - notch);
    }
    seals
}

fn status_tiles() -> Part {
    let mut tiles = Part::empty(name("status_tile_surfaces"));
    for i in 0..STATUS_TILE_COUNT {
        let row = i / 3;
        let col = i % 3;
        tiles = tiles
            + centered_cube(
                name(&format!("status_tile_{i}_ready_hold_review_surface")),
                58.0,
                24.0,
                5.0,
            )
            .translate(
                112.0 + col as f64 * 72.0,
                -44.0 + row as f64 * 36.0,
                STATUS_Z / 2.0 + 2.5,
            );
    }
    tiles
}

fn run_record_lands() -> Part {
    let mut lands = Part::empty(name("run_record_lands"));
    for i in 0..RUN_RECORD_LAND_COUNT {
        lands = lands
            + centered_cube(name(&format!("run_record_land_{i}")), 84.0, 18.0, 3.0).translate(
                -STATUS_X / 2.0 + 56.0 + i as f64 * 104.0,
                STATUS_Y / 2.0 - 58.0,
                STATUS_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn utility_skid_handoff_bulkhead() -> Part {
    let bulkhead = centered_cube(
        name("utility_skid_handoff_bulkhead_body"),
        UTILITY_X,
        UTILITY_Y,
        UTILITY_Z,
    );
    let gasket_relief = centered_cube(
        name("utility_skid_handoff_gasket_window_relief"),
        UTILITY_X + 4.0,
        UTILITY_Y - 92.0,
        34.0,
    )
    .translate(-UTILITY_X / 2.0 + 24.0, 0.0, 4.0);

    bulkhead - gasket_relief
        + utility_connector_banks()
        + handoff_gasket_lands()
        + handoff_latches()
        + utility_tube_bend_witnesses()
        + utility_status_lands()
}

fn utility_connector_banks() -> Part {
    let mut connectors = Part::empty(name("utility_skid_connector_banks"));
    for i in 0..UTILITY_FLUID_CONNECTOR_COUNT {
        let y = 222.0 - i as f64 * 70.0;
        connectors = connectors
            + face_connector(&format!("utility_fluid_connector_{i}"), 17.0).translate(
                -UTILITY_X / 2.0 - 6.0,
                y,
                8.0,
            );
    }
    for i in 0..UTILITY_GAS_CONNECTOR_COUNT {
        let y = -84.0 - i as f64 * 70.0;
        connectors = connectors
            + face_connector(&format!("utility_gas_connector_{i}"), 14.0).translate(
                -UTILITY_X / 2.0 - 6.0,
                y,
                8.0,
            );
    }
    for i in 0..UTILITY_ELECTRICAL_CONNECTOR_COUNT {
        let y = -244.0 - i as f64 * 54.0;
        connectors = connectors
            + centered_cube(
                name(&format!("utility_electrical_connector_keyed_face_{i}")),
                28.0,
                32.0,
                24.0,
            )
            .translate(-UTILITY_X / 2.0 - 8.0, y, 8.0);
    }
    connectors
}

fn face_connector(label: &str, radius: f64) -> Part {
    let collar = centered_cylinder(name(&format!("{label}_collar")), radius, 18.0, 32)
        .rotate(0.0, 90.0, 0.0);
    let bore = centered_cylinder(name(&format!("{label}_bore")), radius * 0.42, 22.0, 24)
        .rotate(0.0, 90.0, 0.0);
    collar - bore
}

fn handoff_gasket_lands() -> Part {
    let mut lands = Part::empty(name("utility_handoff_gasket_lands"));
    for i in 0..HANDOFF_GASKET_COUNT {
        lands = lands
            + centered_cube(
                name(&format!("utility_handoff_compression_gasket_land_{i}")),
                14.0,
                UTILITY_Y - 130.0 - i as f64 * 72.0,
                8.0,
            )
            .translate(
                UTILITY_X / 2.0 - 20.0 - i as f64 * 22.0,
                0.0,
                UTILITY_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn handoff_latches() -> Part {
    let mut latches = Part::empty(name("utility_handoff_latches"));
    for i in 0..HANDOFF_LATCH_COUNT {
        let y = centered_index(i, HANDOFF_LATCH_COUNT, 170.0);
        latches = latches
            + centered_cube(
                name(&format!("utility_handoff_toggle_latch_{i}")),
                42.0,
                24.0,
                22.0,
            )
            .translate(UTILITY_X / 2.0 - 22.0, y, UTILITY_Z / 2.0 + 11.0)
            + centered_cylinder(
                name(&format!("utility_handoff_latch_pin_{i}")),
                5.0,
                48.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(UTILITY_X / 2.0 - 22.0, y, UTILITY_Z / 2.0 + 24.0);
    }
    latches
}

fn utility_tube_bend_witnesses() -> Part {
    let mut tubes = Part::empty(name("utility_tube_bend_radius_witnesses"));
    for i in 0..UTILITY_FLUID_CONNECTOR_COUNT {
        tubes = tubes
            + tube_span_y(&format!("utility_closed_tube_bend_witness_{i}"), 86.0).translate(
                0.0,
                222.0 - i as f64 * 70.0 - 22.0,
                UTILITY_Z / 2.0 + 22.0,
            );
    }
    tubes
}

fn utility_status_lands() -> Part {
    let mut lands = Part::empty(name("utility_skid_handoff_status_lands"));
    for i in 0..UTILITY_CONNECTOR_COUNT {
        lands = lands
            + barcode_land(
                &format!("utility_connector_status_land_{i}"),
                48.0,
                15.0,
                i + 97,
            )
            .translate(
                UTILITY_X / 2.0 - 40.0,
                296.0 - i as f64 * 74.0,
                UTILITY_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn evidence_camera_bridge() -> Part {
    let beam = centered_cube(
        name("evidence_camera_bridge_cross_beam"),
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0);
    let posts = evidence_bridge_posts();
    let cameras = evidence_camera_pods();
    let lights = evidence_light_bars();
    beam + posts + cameras + lights
}

fn evidence_bridge_posts() -> Part {
    let mut posts = Part::empty(name("evidence_bridge_posts"));
    for (i, x) in [
        -EVIDENCE_X / 2.0 + 66.0,
        -EVIDENCE_X / 4.0,
        EVIDENCE_X / 4.0,
        EVIDENCE_X / 2.0 - 66.0,
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                name(&format!("evidence_bridge_post_{i}")),
                20.0,
                28.0,
                EVIDENCE_CLEARANCE_Z,
            )
            .translate(x, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    }
    posts
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty(name("evidence_camera_pods"));
    for i in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(i, EVIDENCE_CAMERA_COUNT, 250.0);
        let pod = centered_cube(name(&format!("evidence_camera_pod_{i}")), 52.0, 38.0, 26.0)
            .translate(
                x,
                -EVIDENCE_Y / 2.0 - 22.0,
                EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 13.0,
            );
        let lens = centered_cylinder(name(&format!("evidence_camera_lens_{i}")), 8.0, 18.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                -EVIDENCE_Y / 2.0 - 44.0,
                EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 13.0,
            );
        pods = pods + pod + lens;
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty(name("evidence_light_bars"));
    for i in 0..EVIDENCE_LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                name(&format!("evidence_light_bar_{i}")),
                EVIDENCE_X - 220.0,
                8.0,
                8.0,
            )
            .translate(
                0.0,
                if i == 0 {
                    EVIDENCE_Y / 2.0 + 12.0
                } else {
                    -EVIDENCE_Y / 2.0 - 12.0
                },
                EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z - 8.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    keepout_floor_outline()
        + front_robot_keepout_gauge()
        + rear_service_keepout_gauge()
        + utility_service_gauge()
        + pan_lift_clearance_towers()
        + refill_tube_bend_clearance_towers()
        + evidence_shadow_gauge()
        + skid_handoff_swing_gauge()
}

fn keepout_floor_outline() -> Part {
    let front = centered_cube(
        name("keepout_floor_front_outline"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        name("keepout_floor_rear_outline"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        name("keepout_floor_left_outline"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        name("keepout_floor_right_outline"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    front + rear + left + right
}

fn front_robot_keepout_gauge() -> Part {
    centered_cube(
        name("front_robot_approach_keepout_gauge"),
        KEEP_OUT_X - 180.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn rear_service_keepout_gauge() -> Part {
    centered_cube(
        name("rear_service_access_keepout_gauge"),
        KEEP_OUT_X - 260.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn utility_service_gauge() -> Part {
    centered_cube(
        name("utility_skid_service_side_keepout_gauge"),
        18.0,
        KEEP_OUT_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - UTILITY_SERVICE_CLEARANCE_X,
        0.0,
        KEEP_OUT_Z / 2.0,
    )
}

fn pan_lift_clearance_towers() -> Part {
    let mut towers = Part::empty(name("pan_lift_clearance_towers"));
    for (i, (x, y)) in [
        (
            PAN_POS.0 - PAN_X / 2.0 + 44.0,
            PAN_POS.1 - PAN_Y / 2.0 + 42.0,
        ),
        (
            PAN_POS.0 + PAN_X / 2.0 - 44.0,
            PAN_POS.1 - PAN_Y / 2.0 + 42.0,
        ),
        (
            PAN_POS.0 - PAN_X / 2.0 + 44.0,
            PAN_POS.1 + PAN_Y / 2.0 - 42.0,
        ),
        (
            PAN_POS.0 + PAN_X / 2.0 - 44.0,
            PAN_POS.1 + PAN_Y / 2.0 - 42.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        towers = towers
            + centered_cylinder(
                name(&format!("pan_lift_z_clearance_tower_{i}")),
                7.0,
                PAN_LIFT_CLEARANCE_Z,
                20,
            )
            .translate(x, y, PAN_LIFT_CLEARANCE_Z / 2.0);
    }
    towers
}

fn refill_tube_bend_clearance_towers() -> Part {
    let mut towers = Part::empty(name("refill_tube_bend_clearance_towers"));
    for i in 0..QUICK_CONNECT_COUNT {
        towers = towers
            + centered_cylinder(
                name(&format!("refill_tube_bend_clearance_tower_{i}")),
                5.0,
                REFILL_TUBE_BEND_CLEARANCE_Z,
                18,
            )
            .translate(
                REFILL_POS.0 + centered_index(i, QUICK_CONNECT_COUNT, 70.0),
                REFILL_POS.1 - REFILL_Y / 2.0 + 6.0,
                REFILL_TUBE_BEND_CLEARANCE_Z / 2.0,
            );
    }
    towers
}

fn evidence_shadow_gauge() -> Part {
    centered_cube(
        name("evidence_bridge_shadow_keepout_gauge"),
        EVIDENCE_X - 240.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(EVIDENCE_POS.0, EVIDENCE_POS.1 - 58.0, KEEP_OUT_Z / 2.0)
}

fn skid_handoff_swing_gauge() -> Part {
    centered_cube(
        name("utility_handoff_door_swing_keepout_gauge"),
        104.0,
        UTILITY_Y - 140.0,
        KEEP_OUT_Z,
    )
    .translate(
        UTILITY_POS.0 - UTILITY_X / 2.0 - 68.0,
        UTILITY_POS.1,
        KEEP_OUT_Z / 2.0,
    )
}

fn barcode_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(name(label), x, y, LABEL_Z);
    land + barcode_bars(label, x, y, code)
}

fn barcode_bars(label: &str, x: f64, y: f64, code: usize) -> Part {
    let mut bars = Part::empty(name(&format!("{label}_bars")));
    for bit in 0..BARCODE_BARS_PER_LAND {
        if bit == 0 || bit == BARCODE_BARS_PER_LAND - 1 || ((code >> (bit % 6)) & 1 == 1) {
            let bar_h = if (code + bit) % 3 == 0 {
                y - 5.0
            } else {
                (y - 7.0) * 0.62
            };
            bars = bars
                + centered_cube(
                    name(&format!("{label}_bar_{bit}")),
                    2.5 + (bit % 2) as f64,
                    bar_h,
                    LABEL_Z,
                )
                .translate(
                    -x / 2.0 + 7.0 + bit as f64 * (x - 14.0) / 10.0,
                    0.0,
                    LABEL_Z,
                );
        }
    }
    bars
}

fn fiducial_target(label: &str) -> Part {
    let outer = centered_cylinder(name(&format!("{label}_outer_disc")), 11.0, 5.0, 36);
    let center = centered_cylinder(name(&format!("{label}_center_dot_cut")), 2.2, 6.0, 20);
    let cross_x = centered_cube(name(&format!("{label}_cross_x_cut")), 18.0, 2.2, 6.0);
    let cross_y = centered_cube(name(&format!("{label}_cross_y_cut")), 2.2, 18.0, 6.0);
    outer - center - cross_x - cross_y
}

fn tube_span_x(label: &str, length: f64) -> Part {
    centered_cylinder(name(label), 4.0, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_span_y(label: &str, length: f64) -> Part {
    centered_cylinder(name(label), 4.0, length, 24).rotate(90.0, 0.0, 0.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(OUTPUTS[0], "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_base_containment_deck.stl");
        assert_eq!(
            OUTPUTS[12],
            "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_assembly.stl"
        );
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_incubator_humidity_water_quality_biofilm_sentinel_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "water_reservoir_pan_surrogate",
            "sterile_refill_interface",
            "conductivity_toc_station",
            "sample_coupon_station",
            "condensate_return_witness_path",
            "drain_overflow_capture",
            "material_compatibility_coupon_array",
            "barcode_custody_status_surfaces",
            "robot_service_keepout_gauges",
            "utility_skid_handoff_bulkhead",
            "reproducibility_controls",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
        assert_design_constraints();
        let rects = socket_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit", rect.name);
        }
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

    #[test]
    fn feature_counts_cover_humidity_water_quality_and_biofilm_controls() {
        assert_eq!(PAN_FILL_LEVEL_MARKS, 5);
        assert_eq!(PAN_BAFFLE_COUNT, 6);
        assert_eq!(QUICK_CONNECT_COUNT, 4);
        assert_eq!(CONDUCTIVITY_CELL_COUNT, 3);
        assert_eq!(TOC_VIAL_COUNT, 10);
        assert_eq!(SAMPLE_COUPON_COUNT, 16);
        assert_eq!(CONDENSATE_WITNESS_WINDOW_COUNT, 8);
        assert_eq!(DRAIN_TRAP_COUNT, 3);
        assert_eq!(OVERFLOW_CUP_COUNT, 4);
        assert_eq!(MATERIAL_COUPON_COUNT, 18);
        assert_eq!(UTILITY_CONNECTOR_COUNT, 8);
    }

    #[test]
    fn containment_and_keepout_budgets_are_explicit_fixture_controls() {
        assert!(water_pan_working_volume_ml() > 2500.0);
        assert!(internal_witness_liquid_volume_ml() > 300.0);
        assert!(maximum_fixture_hold_up_ml() > 1700.0);
        assert!(containment_freeboard_ml() > maximum_fixture_hold_up_ml());
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 7);
        assert!(PAN_LIFT_CLEARANCE_Z > EVIDENCE_CLEARANCE_Z);
        assert!(utility_service_clearance() >= UTILITY_SERVICE_CLEARANCE_X);
    }

    #[test]
    fn traceability_status_and_reproducibility_controls_are_stable() {
        assert_eq!(BARCODE_LAND_COUNT, 12);
        assert_eq!(CUSTODY_SEAL_COUNT, 6);
        assert_eq!(STATUS_TILE_COUNT, 9);
        assert_eq!(RUN_RECORD_LAND_COUNT, 4);
        assert_eq!(PARAMETRIC_REVISION, "LF-CAD-CC-HWQ-BIOFILM-SENTINEL-REV-A");
        assert_eq!(LAYOUT_REVISION_TOKEN, 0xC105_ED1A_864F_0959);
        for control in [
            "fixed_output_manifest",
            "stable_part_names",
            "constant_layout_rectangles",
            "no_random_geometry",
            "no_external_configuration",
        ] {
            assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
        }
    }
}
