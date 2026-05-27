use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleaning residual TOC/conductivity release station.
//
// Intent:
// - Mechanically package a closed, reusable-module release gate that checks
//   automated CIP/decon residue before a cell-culture fluid module returns to
//   service.
// - Keep TOC sampling, conductivity measurement, rinse-loop witness coupons,
//   quarantine latch status, barcode custody, drip containment, sterile
//   boundary, and service-line isolation visible as physical CSG features.
// - Provide architecture and fit CAD only. This is not a cleaning validation
//   protocol, assay acceptance criterion, sterile connector specification, or
//   release procedure.

const BIN_NAME: &str = "closed_cleaning_residual_toc_conductivity_release_station";
const OUTPUT_PREFIX: &str = "output/closed_cleaning_residual_toc_conductivity_release_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_cleaning_residual_toc_conductivity_release_station_base_drip_containment_tray.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_sterile_boundary_module_dock.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_service_line_isolation_manifold.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_rinse_loop_sampling_manifold.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_toc_sample_custody_carousel.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_conductivity_cell_flow_block.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_rinse_loop_witness_coupon_bridge.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_quarantine_latch_gate.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_barcode_custody_panel.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_sterile_boundary_guard.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_robot_service_keepout_gauge.stl",
    "output/closed_cleaning_residual_toc_conductivity_release_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 9] = [
    "toc_sampling",
    "conductivity_cell",
    "rinse_loop_witness_coupon",
    "quarantine_latch",
    "barcode_custody",
    "drip_containment",
    "sterile_boundary",
    "service_line_isolation",
    "closed_cip_decon_rinse_loop",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_release_gate_only",
    "no_cleaning_validation_protocol",
    "no_assay_acceptance_limits",
    "no_sterile_connector_claim",
    "no_automated_release_logic",
    "no_cell_culture_performance_claim",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const MOUNT_HOLE_D: f64 = 6.6;

const DOCK_X: f64 = 430.0;
const DOCK_Y: f64 = 250.0;
const DOCK_Z: f64 = 110.0;
const MODULE_ENVELOPE_X: f64 = 330.0;
const MODULE_ENVELOPE_Y: f64 = 156.0;
const MODULE_ENVELOPE_Z: f64 = 58.0;
const DOCK_CLAMPS: usize = 6;
const STERILE_CONNECTORS: usize = 6;
const DOCK_KEY_PINS: usize = 4;

const ISOLATION_X: f64 = 430.0;
const ISOLATION_Y: f64 = 180.0;
const ISOLATION_Z: f64 = 42.0;
const SERVICE_LINES: usize = 6;
const ISOLATION_VALVES: usize = SERVICE_LINES;
const LINE_PORT_D: f64 = 7.0;

const RINSE_X: f64 = 310.0;
const RINSE_Y: f64 = 220.0;
const RINSE_Z: f64 = 54.0;
const RINSE_LOOP_PORTS: usize = 8;
const SAMPLE_TEE_COUNT: usize = 4;

const TOC_CAROUSEL_D: f64 = 286.0;
const TOC_CAROUSEL_Z: f64 = 38.0;
const TOC_VIALS: usize = 12;
const TOC_VIAL_D: f64 = 18.0;
const TOC_VIAL_RADIUS: f64 = 108.0;
const TOC_BLANKS: usize = 2;

const COND_X: f64 = 300.0;
const COND_Y: f64 = 190.0;
const COND_Z: f64 = 52.0;
const CONDUCTIVITY_CELLS: usize = 2;
const COND_ELECTRODES_PER_CELL: usize = 4;
const COND_CAL_WELLS: usize = 3;

const COUPON_X: f64 = 300.0;
const COUPON_Y: f64 = 190.0;
const COUPON_Z: f64 = 34.0;
const WITNESS_COUPONS: usize = 10;
const COUPON_SLOT_X: f64 = 36.0;
const COUPON_SLOT_Y: f64 = 72.0;

const QUARANTINE_X: f64 = 390.0;
const QUARANTINE_Y: f64 = 120.0;
const QUARANTINE_Z: f64 = 55.0;
const QUARANTINE_POSITIONS: usize = 3;
const LATCH_TEETH: usize = 5;

const BARCODE_X: f64 = 440.0;
const BARCODE_Y: f64 = 120.0;
const BARCODE_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 8;
const CUSTODY_TAGS: usize = 4;
const BARCODE_BARS_PER_LAND: usize = 7;

const GUARD_X: f64 = 720.0;
const GUARD_Y: f64 = 28.0;
const GUARD_Z: f64 = 205.0;
const BOUNDARY_POSTS: usize = 5;
const STERILE_SIDE_LABELS: usize = 4;

const KEEP_OUT_X: f64 = 1250.0;
const KEEP_OUT_Y: f64 = 790.0;
const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_APPROACH_CLEARANCE_MM: f64 = 430.0;
const FRONT_SAMPLE_CLEARANCE_MM: f64 = 360.0;
const REAR_SERVICE_LINE_CLEARANCE_MM: f64 = 270.0;
const SIDE_QUARANTINE_CLEARANCE_MM: f64 = 190.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;

const DOCK_POS: (f64, f64) = (-390.0, 210.0);
const ISOLATION_POS: (f64, f64) = (-415.0, -95.0);
const RINSE_POS: (f64, f64) = (0.0, 210.0);
const TOC_POS: (f64, f64) = (400.0, 220.0);
const COND_POS: (f64, f64) = (415.0, -50.0);
const COUPON_POS: (f64, f64) = (40.0, -105.0);
const QUARANTINE_POS: (f64, f64) = (-410.0, -335.0);
const BARCODE_POS: (f64, f64) = (125.0, -335.0);
const GUARD_POS: (f64, f64) = (-190.0, 54.0);

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = STATION_X / 2.0 - RIM_W - 8.0;
        let half_y = STATION_Y / 2.0 - RIM_W - 8.0;

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

    let base = base_drip_containment_tray();
    export(OUTPUTS[0], &base);

    let dock = sterile_boundary_module_dock();
    export(OUTPUTS[1], &dock);

    let isolation = service_line_isolation_manifold();
    export(OUTPUTS[2], &isolation);

    let rinse = rinse_loop_sampling_manifold();
    export(OUTPUTS[3], &rinse);

    let toc = toc_sample_custody_carousel();
    export(OUTPUTS[4], &toc);

    let conductivity = conductivity_cell_flow_block();
    export(OUTPUTS[5], &conductivity);

    let coupons = rinse_loop_witness_coupon_bridge();
    export(OUTPUTS[6], &coupons);

    let quarantine = quarantine_latch_gate();
    export(OUTPUTS[7], &quarantine);

    let barcode = barcode_custody_panel();
    export(OUTPUTS[8], &barcode);

    let guard = sterile_boundary_guard();
    export(OUTPUTS[9], &guard);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + dock.translate(DOCK_POS.0, DOCK_POS.1, BASE_Z / 2.0 + DOCK_Z / 2.0 + 18.0)
        + isolation.translate(
            ISOLATION_POS.0,
            ISOLATION_POS.1,
            BASE_Z / 2.0 + ISOLATION_Z / 2.0 + 14.0,
        )
        + rinse.translate(
            RINSE_POS.0,
            RINSE_POS.1,
            BASE_Z / 2.0 + RINSE_Z / 2.0 + 18.0,
        )
        + toc.translate(
            TOC_POS.0,
            TOC_POS.1,
            BASE_Z / 2.0 + TOC_CAROUSEL_Z / 2.0 + 20.0,
        )
        + conductivity.translate(COND_POS.0, COND_POS.1, BASE_Z / 2.0 + COND_Z / 2.0 + 20.0)
        + coupons.translate(
            COUPON_POS.0,
            COUPON_POS.1,
            BASE_Z / 2.0 + COUPON_Z / 2.0 + 16.0,
        )
        + quarantine.translate(
            QUARANTINE_POS.0,
            QUARANTINE_POS.1,
            BASE_Z / 2.0 + QUARANTINE_Z / 2.0 + 14.0,
        )
        + barcode.translate(
            BARCODE_POS.0,
            BARCODE_POS.1,
            BASE_Z / 2.0 + BARCODE_Z / 2.0 + 12.0,
        )
        + guard.translate(
            GUARD_POS.0,
            GUARD_POS.1,
            BASE_Z / 2.0 + GUARD_Z / 2.0 + 48.0,
        )
        + closed_release_tube_routes()
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + ROBOT_APPROACH_CLEARANCE_MM / 2.0);
    export(OUTPUTS[11], &assembly);

    println!(
        "Closed cleaning residual TOC/conductivity release station: {:.0}mm x {:.0}mm deck with {} isolated CIP/decon/rinse service lines, {} sterile module connectors, {} keyed dock datum pins, {} TOC vial pockets plus {} blanks, {} conductivity cells, {} rinse-loop witness coupons, {} quarantine latch positions, and {} barcode/custody ID lands.",
        STATION_X,
        STATION_Y,
        SERVICE_LINES,
        STERILE_CONNECTORS,
        DOCK_KEY_PINS,
        TOC_VIALS,
        TOC_BLANKS,
        CONDUCTIVITY_CELLS,
        WITNESS_COUPONS,
        QUARANTINE_POSITIONS,
        BARCODE_LANDS + CUSTODY_TAGS
    );
    println!(
        "Modeled release cues: drip tray with low-point leak wells, sterile boundary guard, service-line isolation manifold, barcode custody panel, quarantine latch gate, {} keepout zones, and {:.0}/{:.0}/{:.0}mm robot/front/rear service clearances.",
        KEEP_OUT_ZONE_COUNT,
        ROBOT_APPROACH_CLEARANCE_MM,
        FRONT_SAMPLE_CLEARANCE_MM,
        REAR_SERVICE_LINE_CLEARANCE_MM
    );
    println!("Scope limits: {}.", LIMITATIONS.join(", "));
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert!(
        OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")),
        "all generated outputs must stay scoped to this bin"
    );

    for feature in REQUIRED_FEATURES {
        assert!(
            !feature.is_empty(),
            "required feature labels must stay explicit"
        );
    }

    for rect in module_rects() {
        assert!(rect.fits_inside_deck(), "{} is outside deck", rect.name);
    }

    let rects = module_rects();
    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_drip_containment_tray() -> Part {
    let deck = centered_cube(
        name("base_drip_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin_recess = centered_cube(
        name("base_drip_containment_shallow_basin_recess"),
        STATION_X - 116.0,
        STATION_Y - 112.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 2.5);

    deck - basin_recess - base_mount_slots() - drip_channel_cuts()
        + containment_perimeter_rim()
        + low_point_leak_wells()
        + deck_datum_rails()
        + station_zone_label_lands()
}

fn containment_perimeter_rim() -> Part {
    let front = centered_cube(
        name("drip_containment_front_low_splash_lip"),
        STATION_X - 60.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        name("drip_containment_rear_service_line_guard_lip"),
        STATION_X - 60.0,
        RIM_W,
        RIM_Z + 18.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + (RIM_Z + 18.0) / 2.0,
    );
    let left = centered_cube(
        name("drip_containment_left_quarantine_side_lip"),
        RIM_W,
        STATION_Y - 60.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        name("drip_containment_right_analytical_side_lip"),
        RIM_W,
        STATION_Y - 60.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty(name("base_mount_slots"));
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            name(&format!("m6_mount_clearance_{i}")),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 6.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            name(&format!("m6_mount_slot_relief_{i}")),
            32.0,
            MOUNT_HOLE_D + 0.8,
            BASE_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_mount_points() -> [(f64, f64); 10] {
    [
        (-(STATION_X / 2.0 - 66.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 66.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 66.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 66.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 66.0), 0.0),
        (STATION_X / 2.0 - 66.0, 0.0),
        (DOCK_POS.0, DOCK_POS.1 - DOCK_Y / 2.0 - 38.0),
        (
            COND_POS.0 + COND_X / 2.0 - 32.0,
            COND_POS.1 - COND_Y / 2.0 - 36.0,
        ),
    ]
}

fn drip_channel_cuts() -> Part {
    let main_front_channel = centered_cube(
        name("front_drip_channel_cut_to_low_point"),
        STATION_X - 190.0,
        13.0,
        8.0,
    )
    .translate(0.0, -286.0, BASE_Z / 2.0 - 2.0);
    let center_channel = centered_cube(
        name("center_drip_channel_cut_from_release_modules"),
        13.0,
        STATION_Y - 230.0,
        8.0,
    )
    .translate(-150.0, -24.0, BASE_Z / 2.0 - 2.0);
    let analytical_channel =
        centered_cube(name("analytical_side_drip_channel_cut"), 13.0, 360.0, 8.0).translate(
            300.0,
            38.0,
            BASE_Z / 2.0 - 2.0,
        );

    main_front_channel + center_channel + analytical_channel
}

fn low_point_leak_wells() -> Part {
    let mut wells = Part::empty(name("low_point_leak_sensor_wells"));
    for i in 0..4 {
        let x = centered_index(i, 4, 92.0) - 68.0;
        let outer = centered_cylinder(
            name(&format!("low_point_leak_sensor_well_{i}_rim")),
            18.0,
            7.0,
            36,
        )
        .translate(x, -322.0, BASE_Z / 2.0 + 3.5);
        let inner = centered_cylinder(
            name(&format!("low_point_leak_sensor_well_{i}_pocket_cut")),
            12.0,
            9.0,
            28,
        )
        .translate(x, -322.0, BASE_Z / 2.0 + 3.5);
        wells = wells + (outer - inner);
    }
    wells
}

fn deck_datum_rails() -> Part {
    let front_datum = centered_cube(name("front_module_loading_datum_rail"), 800.0, 10.0, 18.0)
        .translate(-150.0, -244.0, BASE_Z / 2.0 + 9.0);
    let rear_tube_datum = centered_cube(
        name("rear_closed_service_line_datum_rail"),
        930.0,
        10.0,
        18.0,
    )
    .translate(-100.0, 344.0, BASE_Z / 2.0 + 9.0);
    let sterile_split_line =
        centered_cube(name("sterile_boundary_floor_split_line"), 910.0, 7.0, 8.0).translate(
            -120.0,
            54.0,
            BASE_Z / 2.0 + 4.0,
        );

    front_datum + rear_tube_datum + sterile_split_line
}

fn station_zone_label_lands() -> Part {
    raised_label_land("zone_sterile_module_release", 132.0, 24.0, 11).translate(
        -520.0,
        360.0,
        BASE_Z / 2.0 + 2.0,
    ) + raised_label_land("zone_residue_analytics", 132.0, 24.0, 23).translate(
        382.0,
        360.0,
        BASE_Z / 2.0 + 2.0,
    ) + raised_label_land("zone_quarantine_hold", 132.0, 24.0, 37).translate(
        -484.0,
        -254.0,
        BASE_Z / 2.0 + 2.0,
    )
}

fn sterile_boundary_module_dock() -> Part {
    let body = centered_cube(
        name("sterile_boundary_module_dock_body"),
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let module_pocket = centered_cube(
        name("sterile_boundary_module_envelope_pocket_cut"),
        MODULE_ENVELOPE_X,
        MODULE_ENVELOPE_Y,
        MODULE_ENVELOPE_Z + 12.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 - MODULE_ENVELOPE_Z / 2.0 + 8.0);
    let handoff_slot = centered_cube(
        name("sealed_module_handoff_slot_cut"),
        DOCK_X + 4.0,
        38.0,
        52.0,
    )
    .translate(0.0, -(DOCK_Y / 2.0 - 46.0), 8.0);

    body - module_pocket - handoff_slot
        + sterile_gasket_land()
        + dock_connector_bulkhead()
        + dock_clamp_bosses()
        + dock_key_pin_datums()
        + dock_evidence_windows()
}

fn sterile_gasket_land() -> Part {
    let outer = centered_cube(
        name("sterile_boundary_compression_gasket_outer_land"),
        MODULE_ENVELOPE_X + 52.0,
        MODULE_ENVELOPE_Y + 50.0,
        8.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 4.0);
    let inner = centered_cube(
        name("sterile_boundary_compression_gasket_inner_relief"),
        MODULE_ENVELOPE_X + 14.0,
        MODULE_ENVELOPE_Y + 12.0,
        10.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 4.0);

    outer - inner
}

fn dock_connector_bulkhead() -> Part {
    let face = centered_cube(
        name("closed_module_sterile_connector_bulkhead_face"),
        44.0,
        DOCK_Y - 58.0,
        72.0,
    )
    .translate(-(DOCK_X / 2.0 - 36.0), 0.0, 6.0);
    let mut connectors = Part::empty(name("closed_module_sterile_connector_bores"));
    for i in 0..STERILE_CONNECTORS {
        let y = centered_index(i, STERILE_CONNECTORS, 31.0);
        let bore = centered_cylinder(
            name(&format!("sterile_connector_bore_{i}")),
            LINE_PORT_D / 2.0,
            54.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(DOCK_X / 2.0 - 36.0), y, 6.0);
        let collar = centered_cylinder(
            name(&format!("sterile_connector_clean_side_collar_{i}")),
            13.0,
            7.0,
            30,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(DOCK_X / 2.0 - 62.0), y, 6.0);
        connectors = connectors + collar + bore;
    }

    face + connectors
}

fn dock_clamp_bosses() -> Part {
    let mut bosses = Part::empty(name("module_dock_quick_clamp_bosses"));
    for i in 0..DOCK_CLAMPS {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = centered_index(i / 2, DOCK_CLAMPS / 2, 112.0);
        let boss = centered_cylinder(
            name(&format!("module_dock_toggle_clamp_boss_{i}")),
            15.0,
            18.0,
            30,
        )
        .translate(x, side * (DOCK_Y / 2.0 - 22.0), DOCK_Z / 2.0 + 9.0);
        let screw = centered_cylinder(
            name(&format!("module_dock_toggle_clamp_screw_clearance_{i}")),
            4.0,
            20.0,
            24,
        )
        .translate(x, side * (DOCK_Y / 2.0 - 22.0), DOCK_Z / 2.0 + 9.0);
        bosses = bosses + (boss - screw);
    }
    bosses
}

fn dock_key_pin_datums() -> Part {
    let mut pins = Part::empty(name("module_dock_key_pin_datums"));
    for (i, (x, y)) in [
        (
            -(MODULE_ENVELOPE_X / 2.0 + 22.0),
            -(MODULE_ENVELOPE_Y / 2.0 + 18.0),
        ),
        (
            MODULE_ENVELOPE_X / 2.0 + 22.0,
            -(MODULE_ENVELOPE_Y / 2.0 + 18.0),
        ),
        (
            -(MODULE_ENVELOPE_X / 2.0 + 22.0),
            MODULE_ENVELOPE_Y / 2.0 + 18.0,
        ),
        (
            MODULE_ENVELOPE_X / 2.0 + 22.0,
            MODULE_ENVELOPE_Y / 2.0 + 18.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                name(&format!("module_dock_key_pin_datum_{i}")),
                7.0,
                26.0,
                28,
            )
            .translate(*x, *y, DOCK_Z / 2.0 + 13.0);
    }
    pins
}

fn dock_evidence_windows() -> Part {
    let mut windows = Part::empty(name("module_dock_residue_evidence_windows"));
    for i in 0..3 {
        windows = windows
            + centered_cube(
                name(&format!("module_dock_view_window_frame_{i}")),
                62.0,
                9.0,
                32.0,
            )
            .translate(82.0 + i as f64 * 72.0, -(DOCK_Y / 2.0 + 4.0), 24.0);
    }
    windows
}

fn service_line_isolation_manifold() -> Part {
    let body = centered_cube(
        name("service_line_isolation_manifold_body"),
        ISOLATION_X,
        ISOLATION_Y,
        ISOLATION_Z,
    );
    let clean_side = centered_cube(
        name("service_line_isolation_clean_side_rail"),
        ISOLATION_X - 44.0,
        16.0,
        18.0,
    )
    .translate(0.0, ISOLATION_Y / 2.0 - 18.0, ISOLATION_Z / 2.0 + 9.0);
    let dirty_side = centered_cube(
        name("service_line_isolation_decon_side_rail"),
        ISOLATION_X - 44.0,
        16.0,
        18.0,
    )
    .translate(0.0, -(ISOLATION_Y / 2.0 - 18.0), ISOLATION_Z / 2.0 + 9.0);

    body - service_line_bores()
        + clean_side
        + dirty_side
        + isolation_valve_handles()
        + lockout_tabs()
}

fn service_line_bores() -> Part {
    let mut bores = Part::empty(name("service_line_isolation_bores"));
    for i in 0..SERVICE_LINES {
        let x = centered_index(i, SERVICE_LINES, 61.0);
        let bore = centered_cylinder(
            name(&format!("cip_decon_rinse_service_line_bore_{i}")),
            LINE_PORT_D / 2.0,
            ISOLATION_Y + 8.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 0.0);
        let cross_bore = centered_cylinder(
            name(&format!("service_line_isolation_cross_check_bore_{i}")),
            3.5,
            ISOLATION_X / SERVICE_LINES as f64,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 0.0, ISOLATION_Z / 2.0 - 12.0);
        bores = bores + bore + cross_bore;
    }
    bores
}

fn isolation_valve_handles() -> Part {
    let mut valves = Part::empty(name("service_line_isolation_valve_handles"));
    for i in 0..ISOLATION_VALVES {
        let x = centered_index(i, ISOLATION_VALVES, 61.0);
        let stem = centered_cylinder(
            name(&format!("service_line_isolation_valve_stem_{i}")),
            6.0,
            28.0,
            24,
        )
        .translate(x, 0.0, ISOLATION_Z / 2.0 + 14.0);
        let handle = centered_cube(
            name(&format!("service_line_isolation_valve_handle_{i}")),
            44.0,
            11.0,
            8.0,
        )
        .translate(x, 0.0, ISOLATION_Z / 2.0 + 31.0);
        let witness = centered_cube(
            name(&format!("service_line_isolation_witness_flag_{i}")),
            9.0,
            30.0,
            5.0,
        )
        .translate(x, -44.0, ISOLATION_Z / 2.0 + 26.0);
        valves = valves + stem + handle + witness;
    }
    valves
}

fn lockout_tabs() -> Part {
    let mut tabs = Part::empty(name("service_line_isolation_lockout_tabs"));
    for i in 0..SERVICE_LINES {
        let x = centered_index(i, SERVICE_LINES, 61.0);
        let tab = centered_cube(
            name(&format!("service_line_lockout_tab_{i}")),
            34.0,
            10.0,
            32.0,
        )
        .translate(x, -(ISOLATION_Y / 2.0 + 8.0), ISOLATION_Z / 2.0 + 16.0);
        let hole = centered_cylinder(
            name(&format!("service_line_lockout_hasp_hole_{i}")),
            4.5,
            12.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(ISOLATION_Y / 2.0 + 8.0), ISOLATION_Z / 2.0 + 18.0);
        tabs = tabs + (tab - hole);
    }
    tabs
}

fn rinse_loop_sampling_manifold() -> Part {
    let body = centered_cube(
        name("rinse_loop_sampling_manifold_body"),
        RINSE_X,
        RINSE_Y,
        RINSE_Z,
    );
    let loop_header = centered_cylinder(
        name("closed_rinse_loop_return_header_bore"),
        8.0,
        RINSE_X + 12.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 0.0);
    let purge_header = centered_cylinder(
        name("closed_rinse_loop_purge_header_bore"),
        5.0,
        RINSE_X + 12.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -46.0, 0.0);
    let body = body - loop_header - purge_header - rinse_loop_port_bores();

    body + sampling_tee_towers() + flow_arrow_ribs() + split_route_labels()
}

fn rinse_loop_port_bores() -> Part {
    let mut bores = Part::empty(name("rinse_loop_port_bores"));
    for i in 0..RINSE_LOOP_PORTS {
        let x = centered_index(i, RINSE_LOOP_PORTS, 34.0);
        let bore = centered_cylinder(
            name(&format!("rinse_loop_port_bore_{i}")),
            3.8,
            RINSE_Y + 6.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, RINSE_Z / 2.0 - 14.0);
        bores = bores + bore;
    }
    bores
}

fn sampling_tee_towers() -> Part {
    let mut towers = Part::empty(name("rinse_loop_sampling_tee_towers"));
    for i in 0..SAMPLE_TEE_COUNT {
        let x = centered_index(i, SAMPLE_TEE_COUNT, 68.0);
        let tower = centered_cylinder(
            name(&format!("toc_conductivity_sample_tee_tower_{i}")),
            16.0,
            44.0,
            32,
        )
        .translate(x, RINSE_Y / 2.0 - 38.0, RINSE_Z / 2.0 + 22.0);
        let bore = centered_cylinder(
            name(&format!("toc_conductivity_sample_tee_bore_{i}")),
            4.2,
            48.0,
            24,
        )
        .translate(x, RINSE_Y / 2.0 - 38.0, RINSE_Z / 2.0 + 22.0);
        towers = towers + (tower - bore);
    }
    towers
}

fn flow_arrow_ribs() -> Part {
    let mut ribs = Part::empty(name("rinse_loop_flow_arrow_ribs"));
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                name(&format!("rinse_loop_flow_arrow_bar_{i}")),
                34.0,
                6.0,
                5.0,
            )
            .translate(-102.0 + i as f64 * 52.0, 0.0, RINSE_Z / 2.0 + 2.5)
            + centered_cube(
                name(&format!("rinse_loop_flow_arrow_tip_{i}")),
                10.0,
                16.0,
                5.0,
            )
            .translate(-84.0 + i as f64 * 52.0, 0.0, RINSE_Z / 2.0 + 2.5);
    }
    ribs
}

fn split_route_labels() -> Part {
    raised_label_land("split_to_toc_sampling", 82.0, 18.0, 42).translate(
        -84.0,
        -(RINSE_Y / 2.0 - 24.0),
        RINSE_Z / 2.0 + 2.0,
    ) + raised_label_land("split_to_conductivity_cell", 112.0, 18.0, 55).translate(
        76.0,
        -(RINSE_Y / 2.0 - 24.0),
        RINSE_Z / 2.0 + 2.0,
    )
}

fn toc_sample_custody_carousel() -> Part {
    let disk = centered_cylinder(
        name("toc_sample_custody_carousel_disk"),
        TOC_CAROUSEL_D / 2.0,
        TOC_CAROUSEL_Z,
        96,
    );
    let center_hub = centered_cylinder(
        name("toc_sample_custody_carousel_center_hub"),
        42.0,
        28.0,
        64,
    )
    .translate(0.0, 0.0, TOC_CAROUSEL_Z / 2.0 + 14.0);
    let drive_bore = centered_cylinder(
        name("toc_sample_custody_carousel_drive_bore"),
        12.0,
        TOC_CAROUSEL_Z + 40.0,
        48,
    );
    let pockets = toc_vial_pocket_cuts();

    disk - drive_bore - pockets + center_hub + toc_vial_retainer_tabs() + toc_blank_standard_lands()
}

fn toc_vial_pocket_cuts() -> Part {
    let mut pockets = Part::empty(name("toc_vial_pocket_cuts"));
    for i in 0..TOC_VIALS {
        let angle = std::f64::consts::TAU * i as f64 / TOC_VIALS as f64;
        let x = TOC_VIAL_RADIUS * angle.cos();
        let y = TOC_VIAL_RADIUS * angle.sin();
        let pocket = centered_cylinder(
            name(&format!("toc_sample_vial_pocket_cut_{i}")),
            TOC_VIAL_D / 2.0,
            TOC_CAROUSEL_Z + 8.0,
            32,
        )
        .translate(x, y, 0.0);
        pockets = pockets + pocket;
    }
    pockets
}

fn toc_vial_retainer_tabs() -> Part {
    let mut tabs = Part::empty(name("toc_vial_retainer_tabs"));
    for i in 0..TOC_VIALS {
        let angle = std::f64::consts::TAU * i as f64 / TOC_VIALS as f64;
        let x = (TOC_VIAL_RADIUS + 20.0) * angle.cos();
        let y = (TOC_VIAL_RADIUS + 20.0) * angle.sin();
        tabs = tabs
            + centered_cube(
                name(&format!("toc_sample_vial_retainer_tab_{i}")),
                22.0,
                8.0,
                8.0,
            )
            .rotate(0.0, 0.0, angle.to_degrees())
            .translate(x, y, TOC_CAROUSEL_Z / 2.0 + 4.0);
    }
    tabs
}

fn toc_blank_standard_lands() -> Part {
    let mut lands = Part::empty(name("toc_blank_standard_lands"));
    for i in 0..TOC_BLANKS {
        lands = lands
            + raised_label_land(&format!("toc_blank_standard_land_{i}"), 54.0, 18.0, 71 + i)
                .translate(-42.0 + i as f64 * 84.0, 0.0, TOC_CAROUSEL_Z / 2.0 + 4.0);
    }
    lands
}

fn conductivity_cell_flow_block() -> Part {
    let body = centered_cube(
        name("conductivity_cell_flow_block_body"),
        COND_X,
        COND_Y,
        COND_Z,
    );
    let flow_bore = centered_cylinder(
        name("conductivity_cell_inline_flow_bore"),
        7.0,
        COND_X + 16.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 0.0);
    let bypass_bore = centered_cylinder(
        name("conductivity_cell_bypass_flow_bore"),
        4.5,
        COND_X + 16.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -48.0, 0.0);

    body - flow_bore - bypass_bore - conductivity_cell_window_cuts()
        + conductivity_electrode_bosses()
        + conductivity_calibration_wells()
        + conductivity_cable_glands()
}

fn conductivity_cell_window_cuts() -> Part {
    let mut cuts = Part::empty(name("conductivity_cell_window_cuts"));
    for i in 0..CONDUCTIVITY_CELLS {
        let x = centered_index(i, CONDUCTIVITY_CELLS, 112.0);
        cuts = cuts
            + centered_cube(
                name(&format!("conductivity_cell_view_window_cut_{i}")),
                74.0,
                COND_Y + 8.0,
                26.0,
            )
            .translate(x, 0.0, 4.0);
    }
    cuts
}

fn conductivity_electrode_bosses() -> Part {
    let mut bosses = Part::empty(name("conductivity_cell_electrode_bosses"));
    for cell in 0..CONDUCTIVITY_CELLS {
        let cell_x = centered_index(cell, CONDUCTIVITY_CELLS, 112.0);
        for electrode in 0..COND_ELECTRODES_PER_CELL {
            let x = cell_x + centered_index(electrode, COND_ELECTRODES_PER_CELL, 18.0);
            let boss = centered_cylinder(
                name(&format!(
                    "conductivity_cell_{cell}_electrode_boss_{electrode}"
                )),
                7.5,
                16.0,
                28,
            )
            .translate(x, COND_Y / 2.0 - 26.0, COND_Z / 2.0 + 8.0);
            let pin = centered_cylinder(
                name(&format!(
                    "conductivity_cell_{cell}_electrode_pin_{electrode}"
                )),
                2.5,
                19.0,
                20,
            )
            .translate(x, COND_Y / 2.0 - 26.0, COND_Z / 2.0 + 8.0);
            bosses = bosses + (boss - pin);
        }
    }
    bosses
}

fn conductivity_calibration_wells() -> Part {
    let mut wells = Part::empty(name("conductivity_calibration_standard_wells"));
    for i in 0..COND_CAL_WELLS {
        let y = centered_index(i, COND_CAL_WELLS, 42.0) - 18.0;
        let outer = centered_cylinder(
            name(&format!("conductivity_calibration_well_{i}_rim")),
            18.0,
            8.0,
            36,
        )
        .translate(COND_X / 2.0 - 38.0, y, COND_Z / 2.0 + 4.0);
        let inner = centered_cylinder(
            name(&format!("conductivity_calibration_well_{i}_pocket")),
            11.0,
            10.0,
            30,
        )
        .translate(COND_X / 2.0 - 38.0, y, COND_Z / 2.0 + 4.0);
        wells = wells + (outer - inner);
    }
    wells
}

fn conductivity_cable_glands() -> Part {
    let mut glands = Part::empty(name("conductivity_cell_cable_glands"));
    for i in 0..CONDUCTIVITY_CELLS {
        let x = centered_index(i, CONDUCTIVITY_CELLS, 112.0);
        glands = glands
            + centered_cylinder(
                name(&format!("conductivity_cell_cable_gland_{i}")),
                12.0,
                20.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(COND_Y / 2.0 + 10.0), COND_Z / 2.0 - 4.0);
    }
    glands
}

fn rinse_loop_witness_coupon_bridge() -> Part {
    let bridge = centered_cube(
        name("rinse_loop_witness_coupon_bridge_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let coupon_cuts = coupon_slot_cuts();
    bridge - coupon_cuts
        + coupon_end_clamps()
        + witness_rinse_drip_comb()
        + coupon_index_fiducials()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("rinse_loop_witness_coupon_slot_cuts"));
    for i in 0..WITNESS_COUPONS {
        let col = i % 5;
        let row = i / 5;
        let x = centered_index(col, 5, 50.0);
        let y = centered_index(row, 2, 82.0);
        cuts = cuts
            + centered_cube(
                name(&format!("rinse_loop_witness_coupon_slot_cut_{i}")),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_Z + 6.0,
            )
            .translate(x, y, 5.0);
    }
    cuts
}

fn coupon_end_clamps() -> Part {
    let mut clamps = Part::empty(name("rinse_loop_witness_coupon_end_clamps"));
    for i in 0..WITNESS_COUPONS {
        let col = i % 5;
        let row = i / 5;
        let x = centered_index(col, 5, 50.0);
        let y = centered_index(row, 2, 82.0);
        clamps = clamps
            + centered_cube(
                name(&format!("witness_coupon_front_clamp_{i}")),
                42.0,
                7.0,
                9.0,
            )
            .translate(x, y - COUPON_SLOT_Y / 2.0 - 7.0, COUPON_Z / 2.0 + 4.5)
            + centered_cube(
                name(&format!("witness_coupon_rear_clamp_{i}")),
                42.0,
                7.0,
                9.0,
            )
            .translate(x, y + COUPON_SLOT_Y / 2.0 + 7.0, COUPON_Z / 2.0 + 4.5);
    }
    clamps
}

fn witness_rinse_drip_comb() -> Part {
    let mut comb = Part::empty(name("rinse_loop_witness_coupon_drip_comb"));
    for i in 0..6 {
        comb = comb
            + centered_cube(
                name(&format!("witness_coupon_drip_comb_tooth_{i}")),
                6.0,
                162.0,
                16.0,
            )
            .translate(centered_index(i, 6, 45.0), 0.0, COUPON_Z / 2.0 + 8.0);
    }
    comb
}

fn coupon_index_fiducials() -> Part {
    let left = fiducial_target("witness_coupon_bridge_left_index").translate(
        -(COUPON_X / 2.0 - 24.0),
        COUPON_Y / 2.0 - 24.0,
        COUPON_Z / 2.0 + 2.0,
    );
    let right = fiducial_target("witness_coupon_bridge_right_index").translate(
        COUPON_X / 2.0 - 24.0,
        COUPON_Y / 2.0 - 24.0,
        COUPON_Z / 2.0 + 2.0,
    );
    left + right
}

fn quarantine_latch_gate() -> Part {
    let base = centered_cube(
        name("quarantine_latch_gate_base"),
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let gate_bar = centered_cube(
        name("quarantine_latch_swing_gate_bar"),
        QUARANTINE_X - 54.0,
        18.0,
        28.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0 + 14.0);
    let hinge = centered_cylinder(
        name("quarantine_latch_hinge_pin"),
        12.0,
        QUARANTINE_Y + 24.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(QUARANTINE_X / 2.0 - 34.0), 0.0, QUARANTINE_Z / 2.0 + 18.0);
    let hasp = centered_cube(name("quarantine_latch_padlock_hasp"), 46.0, 14.0, 48.0).translate(
        QUARANTINE_X / 2.0 - 42.0,
        0.0,
        QUARANTINE_Z / 2.0 + 24.0,
    );
    let hasp_hole = centered_cylinder(name("quarantine_latch_padlock_hasp_hole"), 6.0, 17.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(QUARANTINE_X / 2.0 - 42.0, 0.0, QUARANTINE_Z / 2.0 + 26.0);

    base + gate_bar
        + hinge
        + (hasp - hasp_hole)
        + quarantine_position_windows()
        + latch_ratchet_teeth()
}

fn quarantine_position_windows() -> Part {
    let mut windows = Part::empty(name("quarantine_latch_position_windows"));
    for i in 0..QUARANTINE_POSITIONS {
        windows = windows
            + centered_cube(
                name(&format!("quarantine_latch_position_window_{i}")),
                70.0,
                9.0,
                18.0,
            )
            .translate(
                centered_index(i, QUARANTINE_POSITIONS, 92.0),
                -(QUARANTINE_Y / 2.0 + 4.0),
                QUARANTINE_Z / 2.0 + 9.0,
            );
    }
    windows
}

fn latch_ratchet_teeth() -> Part {
    let mut teeth = Part::empty(name("quarantine_latch_ratchet_teeth"));
    for i in 0..LATCH_TEETH {
        teeth = teeth
            + centered_cube(
                name(&format!("quarantine_latch_ratchet_tooth_{i}")),
                24.0,
                10.0,
                18.0,
            )
            .translate(
                -70.0 + i as f64 * 34.0,
                QUARANTINE_Y / 2.0 - 18.0,
                QUARANTINE_Z / 2.0 + 9.0,
            );
    }
    teeth
}

fn barcode_custody_panel() -> Part {
    let panel = centered_cube(
        name("barcode_custody_panel_body"),
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    panel + barcode_id_lands() + custody_tag_slots() + custody_camera_fiducials()
}

fn barcode_id_lands() -> Part {
    let mut lands = Part::empty(name("barcode_custody_id_lands"));
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 94.0);
        let y = centered_index(i / 4, 2, 48.0);
        lands = lands
            + raised_label_land(&format!("barcode_custody_land_{i}"), 72.0, 26.0, 100 + i)
                .translate(x, y, BARCODE_Z / 2.0 + 2.0);
    }
    lands
}

fn custody_tag_slots() -> Part {
    let mut tags = Part::empty(name("barcode_custody_tag_slots"));
    for i in 0..CUSTODY_TAGS {
        let x = centered_index(i, CUSTODY_TAGS, 82.0);
        let frame = centered_cube(
            name(&format!("custody_rfid_tag_frame_{i}")),
            56.0,
            18.0,
            8.0,
        )
        .translate(x, -(BARCODE_Y / 2.0 + 13.0), BARCODE_Z / 2.0 + 4.0);
        let pocket = centered_cube(
            name(&format!("custody_rfid_tag_pocket_cut_{i}")),
            44.0,
            10.0,
            10.0,
        )
        .translate(x, -(BARCODE_Y / 2.0 + 13.0), BARCODE_Z / 2.0 + 4.0);
        tags = tags + (frame - pocket);
    }
    tags
}

fn custody_camera_fiducials() -> Part {
    fiducial_target("barcode_custody_panel_left_camera_fiducial").translate(
        -(BARCODE_X / 2.0 - 24.0),
        BARCODE_Y / 2.0 - 24.0,
        BARCODE_Z / 2.0 + 2.0,
    ) + fiducial_target("barcode_custody_panel_right_camera_fiducial").translate(
        BARCODE_X / 2.0 - 24.0,
        BARCODE_Y / 2.0 - 24.0,
        BARCODE_Z / 2.0 + 2.0,
    )
}

fn sterile_boundary_guard() -> Part {
    let lower_rail = centered_cube(
        name("sterile_boundary_guard_lower_rail"),
        GUARD_X,
        GUARD_Y,
        24.0,
    )
    .translate(0.0, 0.0, -GUARD_Z / 2.0 + 12.0);
    let upper_rail = centered_cube(
        name("sterile_boundary_guard_upper_rail"),
        GUARD_X,
        GUARD_Y,
        22.0,
    )
    .translate(0.0, 0.0, GUARD_Z / 2.0 - 11.0);
    let mut posts = Part::empty(name("sterile_boundary_guard_posts"));
    for i in 0..BOUNDARY_POSTS {
        posts = posts
            + centered_cube(
                name(&format!("sterile_boundary_guard_post_{i}")),
                18.0,
                GUARD_Y,
                GUARD_Z,
            )
            .translate(
                centered_index(i, BOUNDARY_POSTS, GUARD_X / (BOUNDARY_POSTS as f64 - 1.0)),
                0.0,
                0.0,
            );
    }
    lower_rail + upper_rail + posts + sterile_side_label_lands() + boundary_gasket_shadow()
}

fn sterile_side_label_lands() -> Part {
    let mut labels = Part::empty(name("sterile_boundary_side_label_lands"));
    for i in 0..STERILE_SIDE_LABELS {
        labels = labels
            + raised_label_land(
                &format!("sterile_boundary_side_label_{i}"),
                82.0,
                18.0,
                144 + i,
            )
            .translate(
                centered_index(i, STERILE_SIDE_LABELS, 132.0),
                -(GUARD_Y / 2.0 + 3.0),
                -GUARD_Z / 2.0 + 42.0,
            );
    }
    labels
}

fn boundary_gasket_shadow() -> Part {
    centered_cube(
        name("sterile_boundary_floor_gasket_shadow"),
        GUARD_X - 84.0,
        8.0,
        6.0,
    )
    .translate(0.0, 0.0, -GUARD_Z / 2.0 - 3.0)
}

fn robot_service_keepout_gauge() -> Part {
    let front = centered_cube(
        name("keepout_front_sample_pull_clearance_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        name("keepout_rear_service_line_clearance_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        name("keepout_left_quarantine_latch_clearance_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        name("keepout_right_analytical_service_clearance_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);

    front + rear + left + right + keepout_clearance_tags() + keepout_corner_posts()
}

fn keepout_clearance_tags() -> Part {
    raised_label_land(
        &format!("robot_{ROBOT_APPROACH_CLEARANCE_MM:.0}mm_approach_clearance"),
        166.0,
        18.0,
        201,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 32.0, KEEP_OUT_Z / 2.0 + 2.0)
        + raised_label_land(
            &format!("front_{FRONT_SAMPLE_CLEARANCE_MM:.0}mm_sample_clearance"),
            170.0,
            18.0,
            202,
        )
        .translate(310.0, -KEEP_OUT_Y / 2.0 + 32.0, KEEP_OUT_Z / 2.0 + 2.0)
        + raised_label_land(
            &format!("rear_{REAR_SERVICE_LINE_CLEARANCE_MM:.0}mm_service_line_clearance"),
            202.0,
            18.0,
            203,
        )
        .translate(0.0, KEEP_OUT_Y / 2.0 - 32.0, KEEP_OUT_Z / 2.0 + 2.0)
        + raised_label_land(
            &format!("side_{SIDE_QUARANTINE_CLEARANCE_MM:.0}mm_quarantine_latch_clearance"),
            214.0,
            18.0,
            204,
        )
        .translate(-KEEP_OUT_X / 2.0 + 32.0, -48.0, KEEP_OUT_Z / 2.0 + 2.0)
        .rotate(0.0, 0.0, 90.0)
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(name("keepout_corner_posts"));
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
            + centered_cylinder(name(&format!("keepout_corner_post_{i}")), 9.0, 34.0, 24)
                .translate(*x, *y, 17.0);
    }
    posts
}

fn closed_release_tube_routes() -> Part {
    tube_span_y("tube_route_isolation_to_dock_clean_cip", 310.0).translate(
        ISOLATION_POS.0 - 62.0,
        48.0,
        BASE_Z / 2.0 + 92.0,
    ) + tube_span_x("tube_route_dock_return_to_rinse_manifold", 300.0).translate(
        -196.0,
        212.0,
        BASE_Z / 2.0 + 106.0,
    ) + tube_span_x("tube_route_rinse_split_to_toc_carousel", 260.0).translate(
        218.0,
        252.0,
        BASE_Z / 2.0 + 86.0,
    ) + tube_span_y("tube_route_rinse_split_to_conductivity_cell", 246.0).translate(
        254.0,
        78.0,
        BASE_Z / 2.0 + 82.0,
    ) + tube_span_y("tube_route_coupon_witness_return_loop", 245.0).translate(
        22.0,
        52.0,
        BASE_Z / 2.0 + 66.0,
    ) + tube_span_x("tube_route_quarantine_drain_to_drip_tray", 280.0).translate(
        -220.0,
        -300.0,
        BASE_Z / 2.0 + 56.0,
    )
}

fn tube_span_x(label: &str, length: f64) -> Part {
    centered_cylinder(name(label), 3.8, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_span_y(label: &str, length: f64) -> Part {
    centered_cylinder(name(label), 3.8, length, 24).rotate(90.0, 0.0, 0.0)
}

fn raised_label_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(name(label), x, y, 2.0);
    land + label_code_bars(label, x, y, code)
}

fn label_code_bars(label: &str, x: f64, y: f64, code: usize) -> Part {
    let mut bars = Part::empty(name(&format!("{label}_raised_code_bars")));
    for bit in 0..BARCODE_BARS_PER_LAND {
        let bar_h = if (code + bit) % 2 == 0 {
            y - 6.0
        } else {
            (y - 8.0) / 2.0
        };
        bars = bars
            + centered_cube(
                name(&format!("{label}_raised_code_bar_{bit}")),
                2.4,
                bar_h,
                2.0,
            )
            .translate(-x / 2.0 + 9.0 + bit as f64 * 5.2, 0.0, 2.0);
    }
    bars
}

fn fiducial_target(label: &str) -> Part {
    let outer = centered_cylinder(name(&format!("{label}_outer_ring")), 13.0, 3.0, 40);
    let inner = centered_cylinder(name(&format!("{label}_inner_dot_cut")), 5.0, 4.0, 28);
    let cross_x = centered_cube(name(&format!("{label}_crosshair_x")), 32.0, 2.4, 4.0);
    let cross_y = centered_cube(name(&format!("{label}_crosshair_y")), 2.4, 32.0, 4.0);

    outer - inner + cross_x + cross_y
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(label: &str) -> String {
    format!("{BIN_NAME}_{label}")
}

fn module_rects() -> [Rect; 8] {
    [
        Rect {
            name: "sterile dock",
            center: DOCK_POS,
            x: DOCK_X,
            y: DOCK_Y,
        },
        Rect {
            name: "service-line isolation",
            center: ISOLATION_POS,
            x: ISOLATION_X,
            y: ISOLATION_Y,
        },
        Rect {
            name: "rinse-loop sampling",
            center: RINSE_POS,
            x: RINSE_X,
            y: RINSE_Y,
        },
        Rect {
            name: "toc carousel",
            center: TOC_POS,
            x: TOC_CAROUSEL_D,
            y: TOC_CAROUSEL_D,
        },
        Rect {
            name: "conductivity cell",
            center: COND_POS,
            x: COND_X,
            y: COND_Y,
        },
        Rect {
            name: "witness coupon bridge",
            center: COUPON_POS,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "quarantine latch",
            center: QUARANTINE_POS,
            x: QUARANTINE_X,
            y: QUARANTINE_Y,
        },
        Rect {
            name: "barcode custody",
            center: BARCODE_POS,
            x: BARCODE_X,
            y: BARCODE_Y,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_scoped_to_generator() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    }

    #[test]
    fn requested_features_are_declared() {
        for feature in [
            "toc_sampling",
            "conductivity_cell",
            "rinse_loop_witness_coupon",
            "quarantine_latch",
            "barcode_custody",
            "drip_containment",
            "sterile_boundary",
            "service_line_isolation",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn analytical_counts_cover_release_gate_cues() {
        assert!(TOC_VIALS >= 10);
        assert!(CONDUCTIVITY_CELLS >= 2);
        assert_eq!(ISOLATION_VALVES, SERVICE_LINES);
        assert_eq!(STERILE_CONNECTORS, SERVICE_LINES);
        assert!(WITNESS_COUPONS >= SERVICE_LINES);
        assert!(BARCODE_LANDS + CUSTODY_TAGS >= TOC_VIALS);
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
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

    #[test]
    fn keepout_and_boundary_counts_are_explicit() {
        assert_eq!(DOCK_KEY_PINS, 4);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
        assert_eq!(BOUNDARY_POSTS, 5);
        assert!(GUARD_Z > DOCK_Z);
    }
}
