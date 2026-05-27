use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sensor probe storage, hydration, and drift validation station.
//
// Intent:
// - Stage inline culture-run sensor probes through wet hydration, dry hold,
//   reference-standard exposure, dark-cover witnessing, elapsed-time tokening,
//   cable strain relief, traceability, and release/hold/reject disposition.
// - Keep the validation interfaces visible as named CSG geometry: probe nests,
//   standard wells, leak tray, witness cover, time tokens, cable clamps,
//   barcode/certificate lands, lane labels, and raised CSG label chips.
//
// This is validation fixture/interface CAD only. It does not define calibration
// acceptance limits, sensor chemistry, certificate authority, sterile barrier,
// or cell-culture process release criteria.

const OUTPUT_PREFIX: &str = "output/closed_sensor_probe_storage_hydration_drift_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_sensor_probe_storage_hydration_drift_station_base_leak_tray_deck.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_hydrated_probe_nest_bank.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_dry_probe_nest_bank.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_reference_standard_wells.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_dark_cover_witness.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_time_token_slots.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_cable_strain_relief.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_barcode_certificate_lands.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_release_hold_reject_lanes.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_removable_leak_tray_insert.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_csg_label_geometry.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_robot_service_keepout_gauge.stl",
    "output/closed_sensor_probe_storage_hydration_drift_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "hydrated_probe_nests",
    "dry_probe_nests",
    "reference_standard_wells",
    "dark_cover_witness",
    "time_token_slots",
    "cable_strain_relief",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "leak_tray",
    "labels_as_csg_geometry",
    "robot_service_keepouts",
    "standalone_stl_exports",
];

const LIMITATIONS: [&str; 5] = [
    "validation_fixture_only",
    "no_calibration_acceptance_limits",
    "no_sensor_chemistry_claim",
    "no_sterile_barrier_claim",
    "no_cell_culture_release_claim",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const SOCKET_DEPTH: f64 = 5.0;

const PROBE_COUNT: usize = 8;
const PROBE_ROWS: usize = 2;
const PROBE_COLS: usize = 4;
const PROBE_NAMES: [&str; PROBE_COUNT] = [
    "ph",
    "do",
    "o2",
    "temperature",
    "conductivity",
    "optical_density",
    "pressure",
    "flow",
];
const PROBE_PITCH_X: f64 = 82.0;
const PROBE_PITCH_Y: f64 = 62.0;
const PROBE_SLEEVE_D: f64 = 13.2;
const PROBE_CABLE_D: f64 = 5.8;
const PROBE_LIFT_CLEARANCE_Z: f64 = 118.0;

const HYDRATED_X: f64 = 380.0;
const HYDRATED_Y: f64 = 168.0;
const HYDRATED_Z: f64 = 54.0;
const HYDRATED_POS: (f64, f64) = (-365.0, 220.0);
const HYDRATION_WELL_D: f64 = 24.0;
const HYDRATION_WELL_DEPTH: f64 = 36.0;

const DRY_X: f64 = 380.0;
const DRY_Y: f64 = 160.0;
const DRY_Z: f64 = 44.0;
const DRY_POS: (f64, f64) = (-365.0, 20.0);
const DRY_DESICCANT_CHANNELS: usize = 6;

const STANDARD_X: f64 = 360.0;
const STANDARD_Y: f64 = 180.0;
const STANDARD_Z: f64 = 58.0;
const STANDARD_POS: (f64, f64) = (70.0, 220.0);
const STANDARD_WELL_COUNT: usize = 10;
const STANDARD_WELL_D: f64 = 31.0;
const STANDARD_WELL_DEPTH: f64 = 38.0;
const STANDARD_COLS: usize = 5;
const STANDARD_ROWS: usize = 2;
const STANDARD_PITCH_X: f64 = 62.0;
const STANDARD_PITCH_Y: f64 = 64.0;

const DARK_COVER_X: f64 = 270.0;
const DARK_COVER_Y: f64 = 178.0;
const DARK_COVER_Z: f64 = 146.0;
const DARK_COVER_POS: (f64, f64) = (425.0, 220.0);
const DARK_WITNESS_COUPONS: usize = 4;
const DARK_WITNESS_WINDOW_COUNT: usize = 3;

const TOKEN_X: f64 = 290.0;
const TOKEN_Y: f64 = 150.0;
const TOKEN_Z: f64 = 22.0;
const TOKEN_POS: (f64, f64) = (40.0, 20.0);
const TIME_TOKEN_SLOTS: usize = 8;
const TIME_TOKEN_PITCH_X: f64 = 62.0;
const TIME_TOKEN_ROWS: usize = 2;
const TIME_TOKEN_COLS: usize = 4;

const CABLE_X: f64 = 330.0;
const CABLE_Y: f64 = 150.0;
const CABLE_Z: f64 = 42.0;
const CABLE_POS: (f64, f64) = (380.0, 20.0);
const CABLE_CHANNEL_COUNT: usize = PROBE_COUNT;
const CABLE_CLAMP_COUNT: usize = 4;
const CABLE_CHANNEL_PITCH: f64 = 16.0;

const TRACE_X: f64 = 320.0;
const TRACE_Y: f64 = 128.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-390.0, -230.0);
const BARCODE_LANDS: usize = PROBE_COUNT;
const CERTIFICATE_LANDS: usize = 4;
const RUN_RECORD_LANDS: usize = 2;

const DISPOSITION_X: f64 = 360.0;
const DISPOSITION_Y: f64 = 150.0;
const DISPOSITION_Z: f64 = 36.0;
const DISPOSITION_POS: (f64, f64) = (-40.0, -230.0);
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;

const LEAK_INSERT_X: f64 = 360.0;
const LEAK_INSERT_Y: f64 = 150.0;
const LEAK_INSERT_Z: f64 = 30.0;
const LEAK_INSERT_POS: (f64, f64) = (350.0, -230.0);
const LEAK_WITNESS_RIBS: usize = 6;
const LEAK_DRAIN_PORT_D: f64 = 11.0;

const LABEL_X: f64 = 920.0;
const LABEL_Y: f64 = 38.0;
const LABEL_Z: f64 = 7.0;
const LABEL_POS: (f64, f64) = (0.0, -335.0);
const LABEL_CHIPS: usize = 9;

const KEEP_OUT_X: f64 = 1100.0;
const KEEP_OUT_Y: f64 = 700.0;
const KEEP_OUT_Z: f64 = 5.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 36.0;
const REAR_SERVICE_CLEARANCE: f64 = 34.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: Footprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let hydrated = hydrated_probe_nest_bank();
    export(OUTPUTS[1], &hydrated);

    let dry = dry_probe_nest_bank();
    export(OUTPUTS[2], &dry);

    let standards = reference_standard_wells();
    export(OUTPUTS[3], &standards);

    let dark_cover = dark_cover_witness();
    export(OUTPUTS[4], &dark_cover);

    let tokens = time_token_slots();
    export(OUTPUTS[5], &tokens);

    let cable_relief = cable_strain_relief();
    export(OUTPUTS[6], &cable_relief);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[8], &disposition);

    let leak_insert = removable_leak_tray_insert();
    export(OUTPUTS[9], &leak_insert);

    let labels = csg_label_geometry();
    export(OUTPUTS[10], &labels);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + hydrated.translate(HYDRATED_POS.0, HYDRATED_POS.1, on_deck_z(HYDRATED_Z))
        + dry.translate(DRY_POS.0, DRY_POS.1, on_deck_z(DRY_Z))
        + standards.translate(STANDARD_POS.0, STANDARD_POS.1, on_deck_z(STANDARD_Z))
        + dark_cover.translate(DARK_COVER_POS.0, DARK_COVER_POS.1, on_deck_z(DARK_COVER_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + cable_relief.translate(CABLE_POS.0, CABLE_POS.1, on_deck_z(CABLE_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + disposition.translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            on_deck_z(DISPOSITION_Z),
        )
        + leak_insert.translate(
            LEAK_INSERT_POS.0,
            LEAK_INSERT_POS.1,
            on_deck_z(LEAK_INSERT_Z),
        )
        + labels.translate(LABEL_POS.0, LABEL_POS.1, on_deck_z(LABEL_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed sensor probe storage/hydration drift station:");
    println!(
        "  Footprint:               {STATION_X:.0}mm x {STATION_Y:.0}mm closed leak-tray deck"
    );
    println!(
        "  Probe staging:           {PROBE_COUNT} probe families ({}) with hydrated and dry nests",
        PROBE_NAMES.join(", ")
    );
    println!(
        "  Reference/drift control: {STANDARD_WELL_COUNT} standard wells, {DARK_WITNESS_COUPONS} dark-cover witness coupons, {TIME_TOKEN_SLOTS} time-token slots"
    );
    println!(
        "  Cable/traceability:      {CABLE_CHANNEL_COUNT} strain-relief channels, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {RUN_RECORD_LANDS} run-record lands"
    );
    println!(
        "  Disposition/leak:        release/hold/reject lanes with {DISPOSITION_SLOTS_PER_LANE} slots each, removable leak tray with {LEAK_WITNESS_RIBS} witness ribs"
    );
    println!(
        "  Labels/keepouts:         {LABEL_CHIPS} raised CSG label chips and {KEEP_OUT_ZONE_COUNT} robot/service keepout zones"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn module_footprints() -> [Footprint; 10] {
    [
        footprint(
            "hydrated_probe_nest_bank",
            HYDRATED_POS,
            HYDRATED_X,
            HYDRATED_Y,
        ),
        footprint("dry_probe_nest_bank", DRY_POS, DRY_X, DRY_Y),
        footprint(
            "reference_standard_wells",
            STANDARD_POS,
            STANDARD_X,
            STANDARD_Y,
        ),
        footprint(
            "dark_cover_witness",
            DARK_COVER_POS,
            DARK_COVER_X,
            DARK_COVER_Y,
        ),
        footprint("time_token_slots", TOKEN_POS, TOKEN_X, TOKEN_Y),
        footprint("cable_strain_relief", CABLE_POS, CABLE_X, CABLE_Y),
        footprint("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        footprint(
            "release_hold_reject_lanes",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        footprint(
            "removable_leak_tray_insert",
            LEAK_INSERT_POS,
            LEAK_INSERT_X,
            LEAK_INSERT_Y,
        ),
        footprint("csg_label_geometry", LABEL_POS, LABEL_X, LABEL_Y),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    for feature in [
        "hydrated_probe_nests",
        "dry_probe_nests",
        "reference_standard_wells",
        "dark_cover_witness",
        "time_token_slots",
        "cable_strain_relief",
        "barcode_certificate_lands",
        "release_hold_reject_lanes",
        "leak_tray",
        "labels_as_csg_geometry",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }
    for limitation in [
        "validation_fixture_only",
        "no_calibration_acceptance_limits",
        "no_sensor_chemistry_claim",
        "no_sterile_barrier_claim",
        "no_cell_culture_release_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(PROBE_COUNT, PROBE_ROWS * PROBE_COLS);
    assert_eq!(PROBE_NAMES.len(), PROBE_COUNT);
    assert_eq!(STANDARD_WELL_COUNT, STANDARD_ROWS * STANDARD_COLS);
    assert_eq!(TIME_TOKEN_SLOTS, TIME_TOKEN_ROWS * TIME_TOKEN_COLS);
    assert_eq!(BARCODE_LANDS, PROBE_COUNT);
    assert_eq!(CABLE_CHANNEL_COUNT, PROBE_COUNT);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(HYDRATION_WELL_DEPTH > STANDARD_WELL_DEPTH - 4.0);
    assert!(PROBE_SLEEVE_D > PROBE_CABLE_D);
    assert!(DARK_COVER_Z > PROBE_LIFT_CLEARANCE_Z);
    assert!(FRONT_ROBOT_SWEEP_CLEARANCE >= 32.0);
    assert!(REAR_SERVICE_CLEARANCE >= 30.0);
    assert!(leak_capture_volume_ml() > hydrated_probe_spill_volume_ml());

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds leak tray deck",
            module.name
        );
    }

    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn leak_capture_volume_ml() -> f64 {
    (LEAK_INSERT_X - 46.0) * (LEAK_INSERT_Y - 42.0) * 16.0 / 1000.0
}

fn hydrated_probe_spill_volume_ml() -> f64 {
    PROBE_COUNT as f64 * 4.0
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_sensor_probe_hydration_drift_base_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let recessed_pan = centered_cube(
        "closed_sensor_probe_hydration_drift_recessed_spill_pan",
        STATION_X - 112.0,
        STATION_Y - 104.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.5);
    let front_gutter = centered_cube(
        "closed_sensor_probe_hydration_drift_front_leak_gutter",
        STATION_X - 170.0,
        22.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 62.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        "closed_sensor_probe_hydration_drift_deck_drain_port",
        LEAK_DRAIN_PORT_D / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 + 48.0, 0.0);

    deck - recessed_pan - front_gutter - drain - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + deck_zone_dividers()
        + datum_targets()
        + leak_flow_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_sensor_probe_hydration_drift_insert_sockets");
    for module in module_footprints().iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_locator_socket",
                    module.name
                ),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_sensor_probe_hydration_drift_mounting_slots");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_sensor_probe_hydration_drift_m6_clearance_{i}"),
                6.8 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_m6_slot_relief_{i}"),
                26.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_sensor_probe_hydration_drift_front_low_spill_lip",
        STATION_X - 150.0,
        RIM_W,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 11.0);
    let rear = centered_cube(
        "closed_sensor_probe_hydration_drift_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_sensor_probe_hydration_drift_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_sensor_probe_hydration_drift_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_zone_dividers() -> Part {
    let wet_to_middle = centered_cube(
        "closed_sensor_probe_hydration_drift_wet_to_dry_divider",
        STATION_X - 160.0,
        10.0,
        26.0,
    )
    .translate(0.0, 118.0, BASE_Z / 2.0 + 13.0);
    let middle_to_release = centered_cube(
        "closed_sensor_probe_hydration_drift_dry_to_release_divider",
        STATION_X - 172.0,
        10.0,
        24.0,
    )
    .translate(0.0, -112.0, BASE_Z / 2.0 + 12.0);
    let wet_probe_to_standards = centered_cube(
        "closed_sensor_probe_hydration_drift_hydrated_to_standard_divider",
        10.0,
        180.0,
        26.0,
    )
    .translate(-146.0, 220.0, BASE_Z / 2.0 + 13.0);
    let standard_to_dark = centered_cube(
        "closed_sensor_probe_hydration_drift_standard_to_dark_cover_divider",
        10.0,
        180.0,
        26.0,
    )
    .translate(272.0, 220.0, BASE_Z / 2.0 + 13.0);
    let token_to_cable = centered_cube(
        "closed_sensor_probe_hydration_drift_token_to_cable_divider",
        10.0,
        150.0,
        24.0,
    )
    .translate(198.0, 20.0, BASE_Z / 2.0 + 12.0);

    wet_to_middle + middle_to_release + wet_probe_to_standards + standard_to_dark + token_to_cable
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_sensor_probe_hydration_drift_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 90.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 90.0, -STATION_Y / 2.0 + 92.0),
        (-STATION_X / 2.0 + 90.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 90.0, STATION_Y / 2.0 - 92.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_sensor_probe_hydration_drift_datum_target_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn leak_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_sensor_probe_hydration_drift_deck_leak_flow_witness_ribs");
    for i in 0..LEAK_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_deck_witness_rib_{i}"),
                STATION_X - 210.0,
                4.0,
                5.0,
            )
            .translate(
                0.0,
                centered_index(i, LEAK_WITNESS_RIBS, 78.0) - 26.0,
                BASE_Z / 2.0 + 2.5,
            );
    }
    ribs
}

fn hydrated_probe_nest_bank() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_hydrated_probe_nest_bank_body",
        HYDRATED_X,
        HYDRATED_Y,
        HYDRATED_Z,
    );
    let splash_fence = centered_cube(
        "closed_sensor_probe_hydration_drift_hydrated_probe_splash_fence",
        HYDRATED_X,
        14.0,
        HYDRATED_Z + 30.0,
    )
    .translate(0.0, HYDRATED_Y / 2.0 - 7.0, 15.0);
    let lid_land = centered_cube(
        "closed_sensor_probe_hydration_drift_hydrated_probe_evaporation_lid_land",
        HYDRATED_X - 34.0,
        HYDRATED_Y - 28.0,
        6.0,
    )
    .translate(0.0, 0.0, HYDRATED_Z / 2.0 + 3.0);

    let mut cuts = Part::empty("closed_sensor_probe_hydration_drift_hydrated_probe_cuts");
    let mut stops = Part::empty("closed_sensor_probe_hydration_drift_hydrated_probe_stops");
    let mut labels = Part::empty("closed_sensor_probe_hydration_drift_hydrated_probe_labels");

    for index in 0..PROBE_COUNT {
        let row = index / PROBE_COLS;
        let col = index % PROBE_COLS;
        let x = centered_index(col, PROBE_COLS, PROBE_PITCH_X);
        let y = centered_index(row, PROBE_ROWS, PROBE_PITCH_Y);
        let name = PROBE_NAMES[index];
        cuts = cuts
            + probe_sleeve_cut(&format!(
                "closed_sensor_probe_hydration_drift_{name}_hydrated_sleeve"
            ))
            .translate(x, y, 5.0)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_hydrated_top_access"),
                24.0,
                HYDRATED_Y - 30.0,
                24.0,
            )
            .translate(x, y, HYDRATED_Z / 2.0 - 8.0)
            + centered_cylinder(
                format!("closed_sensor_probe_hydration_drift_{name}_hydration_buffer_well"),
                HYDRATION_WELL_D / 2.0,
                HYDRATION_WELL_DEPTH,
                36,
            )
            .translate(
                x + 25.0,
                y,
                HYDRATED_Z / 2.0 - HYDRATION_WELL_DEPTH / 2.0 + 2.0,
            );

        stops = stops
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_hydrated_nose_stop"),
                34.0,
                7.0,
                18.0,
            )
            .translate(x, y + 38.0, HYDRATED_Z / 2.0 + 9.0)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_hydrated_cable_clip"),
                36.0,
                10.0,
                14.0,
            )
            .translate(x, y - 42.0, HYDRATED_Z / 2.0 + 7.0);
        labels = labels
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_hydrated_label_land"),
                54.0,
                10.0,
                3.0,
            )
            .translate(x, y - 57.0, HYDRATED_Z / 2.0 + 1.5);
    }

    let fill_header = centered_cylinder(
        "closed_sensor_probe_hydration_drift_hydrated_fill_header_bore",
        5.5 / 2.0,
        HYDRATED_X - 48.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, HYDRATED_Y / 2.0 - 28.0, 4.0);
    let drain_header = centered_cylinder(
        "closed_sensor_probe_hydration_drift_hydrated_drain_header_bore",
        5.5 / 2.0,
        HYDRATED_X - 48.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -HYDRATED_Y / 2.0 + 28.0, 4.0);

    body + splash_fence + lid_land + stops + labels - cuts - fill_header - drain_header
        + gripper_fiducials("hydrated_probe_bank", 150.0)
}

fn dry_probe_nest_bank() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_dry_probe_nest_bank_body",
        DRY_X,
        DRY_Y,
        DRY_Z,
    );
    let rear_fence = centered_cube(
        "closed_sensor_probe_hydration_drift_dry_probe_rear_cable_fence",
        DRY_X,
        14.0,
        DRY_Z + 26.0,
    )
    .translate(0.0, DRY_Y / 2.0 - 7.0, 13.0);

    let mut cuts = Part::empty("closed_sensor_probe_hydration_drift_dry_probe_cuts");
    let mut rails = Part::empty("closed_sensor_probe_hydration_drift_dry_probe_rails");
    let mut labels = Part::empty("closed_sensor_probe_hydration_drift_dry_probe_labels");
    for index in 0..PROBE_COUNT {
        let row = index / PROBE_COLS;
        let col = index % PROBE_COLS;
        let x = centered_index(col, PROBE_COLS, PROBE_PITCH_X);
        let y = centered_index(row, PROBE_ROWS, PROBE_PITCH_Y);
        let name = PROBE_NAMES[index];
        cuts = cuts
            + probe_sleeve_cut(&format!(
                "closed_sensor_probe_hydration_drift_{name}_dry_sleeve"
            ))
            .translate(x, y, 1.0)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_dry_pick_access"),
                24.0,
                DRY_Y - 26.0,
                20.0,
            )
            .translate(x, y, DRY_Z / 2.0 - 7.0);
        rails = rails
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_dry_tip_stop"),
                32.0,
                6.0,
                16.0,
            )
            .translate(x, y + 35.0, DRY_Z / 2.0 + 8.0)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_dry_desiccant_gap_gauge"),
                44.0,
                4.0,
                7.0,
            )
            .translate(x, y - 34.0, DRY_Z / 2.0 + 3.5);
        labels = labels
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_dry_label_land"),
                54.0,
                9.0,
                3.0,
            )
            .translate(x, y - 52.0, DRY_Z / 2.0 + 1.5);
    }

    let mut vents = Part::empty("closed_sensor_probe_hydration_drift_dry_desiccant_vents");
    for i in 0..DRY_DESICCANT_CHANNELS {
        vents = vents
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_dry_desiccant_vent_{i}"),
                DRY_X - 72.0,
                4.0,
                8.0,
            )
            .translate(0.0, centered_index(i, DRY_DESICCANT_CHANNELS, 22.0), -8.0);
    }

    body + rear_fence + rails + labels - cuts - vents + gripper_fiducials("dry_probe_bank", 150.0)
}

fn probe_sleeve_cut(name: &str) -> Part {
    centered_cylinder(name, PROBE_SLEEVE_D / 2.0, 118.0, 36).rotate(90.0, 0.0, 0.0)
}

fn reference_standard_wells() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_reference_standard_well_bank_body",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    );
    let backer = centered_cube(
        "closed_sensor_probe_hydration_drift_reference_standard_capped_vial_backer",
        STANDARD_X,
        14.0,
        STANDARD_Z + 34.0,
    )
    .translate(0.0, STANDARD_Y / 2.0 - 7.0, 17.0);

    let mut wells = Part::empty("closed_sensor_probe_hydration_drift_reference_standard_wells");
    let mut clips = Part::empty("closed_sensor_probe_hydration_drift_reference_standard_clips");
    let mut labels = Part::empty("closed_sensor_probe_hydration_drift_reference_standard_labels");

    for i in 0..STANDARD_WELL_COUNT {
        let row = i / STANDARD_COLS;
        let col = i % STANDARD_COLS;
        let x = centered_index(col, STANDARD_COLS, STANDARD_PITCH_X);
        let y = centered_index(row, STANDARD_ROWS, STANDARD_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_standard_well",
                    standard_name(i)
                ),
                STANDARD_WELL_D / 2.0,
                STANDARD_WELL_DEPTH + 1.0,
                44,
            )
            .translate(x, y, STANDARD_Z / 2.0 - STANDARD_WELL_DEPTH / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_standard_key_flat",
                    standard_name(i)
                ),
                14.0,
                8.0,
                STANDARD_WELL_DEPTH + 2.0,
            )
            .translate(x + STANDARD_WELL_D / 2.0 - 4.0, y, STANDARD_Z / 2.0 - 16.0);
        clips = clips
            + centered_cube(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_standard_retention_clip",
                    standard_name(i)
                ),
                34.0,
                6.0,
                16.0,
            )
            .translate(x, y + 25.0, STANDARD_Z / 2.0 + 8.0);
        labels = labels
            + centered_cube(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_standard_label_land",
                    standard_name(i)
                ),
                44.0,
                10.0,
                3.0,
            )
            .translate(x, y - 31.0, STANDARD_Z / 2.0 + 1.5);
    }

    let row_divider = centered_cube(
        "closed_sensor_probe_hydration_drift_reference_standard_low_high_divider",
        STANDARD_X - 42.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, STANDARD_Z / 2.0 + 12.0);

    body + backer + clips + labels + row_divider - wells + gripper_fiducials("standard_bank", 140.0)
}

fn standard_name(index: usize) -> &'static str {
    match index {
        0 => "ph4",
        1 => "ph7",
        2 => "do_zero",
        3 => "do_air",
        4 => "o2_span",
        5 => "conductivity_low",
        6 => "conductivity_high",
        7 => "temperature_37c",
        8 => "pressure_span",
        _ => "blank_dark",
    }
}

fn dark_cover_witness() -> Part {
    let roof = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_roof",
        DARK_COVER_X,
        DARK_COVER_Y,
        16.0,
    )
    .translate(0.0, 0.0, DARK_COVER_Z / 2.0 - 8.0);
    let left_wall = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_left_wall",
        16.0,
        DARK_COVER_Y,
        DARK_COVER_Z,
    )
    .translate(-DARK_COVER_X / 2.0 + 8.0, 0.0, 0.0);
    let right_wall = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_right_wall",
        16.0,
        DARK_COVER_Y,
        DARK_COVER_Z,
    )
    .translate(DARK_COVER_X / 2.0 - 8.0, 0.0, 0.0);
    let rear_wall = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_rear_wall",
        DARK_COVER_X,
        16.0,
        DARK_COVER_Z,
    )
    .translate(0.0, DARK_COVER_Y / 2.0 - 8.0, 0.0);
    let front_light_lip = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_front_light_lip",
        DARK_COVER_X - 42.0,
        14.0,
        42.0,
    )
    .translate(0.0, -DARK_COVER_Y / 2.0 + 12.0, -DARK_COVER_Z / 2.0 + 21.0);

    let mut witness_coupons =
        Part::empty("closed_sensor_probe_hydration_drift_dark_cover_witness_coupons");
    for i in 0..DARK_WITNESS_COUPONS {
        witness_coupons = witness_coupons
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_dark_witness_coupon_slot_{i}"),
                42.0,
                24.0,
                5.0,
            )
            .translate(
                centered_index(i, DARK_WITNESS_COUPONS, 54.0),
                -DARK_COVER_Y / 2.0 + 40.0,
                DARK_COVER_Z / 2.0 + 2.5,
            );
    }

    let mut shuttered_windows =
        Part::empty("closed_sensor_probe_hydration_drift_dark_cover_shuttered_witness_windows");
    for i in 0..DARK_WITNESS_WINDOW_COUNT {
        shuttered_windows = shuttered_windows
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_dark_cover_shutter_window_{i}"),
                44.0,
                12.0,
                30.0,
            )
            .translate(
                centered_index(i, DARK_WITNESS_WINDOW_COUNT, 68.0),
                -DARK_COVER_Y / 2.0 + 5.0,
                12.0,
            );
    }

    let witness_label = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_witness_label_land",
        DARK_COVER_X - 62.0,
        12.0,
        3.0,
    )
    .translate(0.0, DARK_COVER_Y / 2.0 - 34.0, DARK_COVER_Z / 2.0 + 1.5);

    roof + left_wall + right_wall + rear_wall + front_light_lip + witness_coupons + witness_label
        - shuttered_windows
}

fn time_token_slots() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_time_token_slot_panel_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut slots = Part::empty("closed_sensor_probe_hydration_drift_time_token_slot_cuts");
    let mut stops = Part::empty("closed_sensor_probe_hydration_drift_time_token_positive_stops");
    let mut labels = Part::empty("closed_sensor_probe_hydration_drift_time_token_label_lands");

    for i in 0..TIME_TOKEN_SLOTS {
        let row = i / TIME_TOKEN_COLS;
        let col = i % TIME_TOKEN_COLS;
        let x = centered_index(col, TIME_TOKEN_COLS, TIME_TOKEN_PITCH_X);
        let y = centered_index(row, TIME_TOKEN_ROWS, 52.0);
        slots = slots
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_time_token_slot_{i}"),
                42.0,
                30.0,
                18.0,
            )
            .translate(x, y, TOKEN_Z / 2.0 - 8.0);
        stops = stops
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_time_token_slot_{i}_stop"),
                44.0,
                5.0,
                9.0,
            )
            .translate(x, y + 20.0, TOKEN_Z / 2.0 + 4.5);
        labels = labels
            + coded_label(
                &format!("closed_sensor_probe_hydration_drift_time_token_{i}_csg_label"),
                token_code(i),
                42.0,
            )
            .translate(x, y - 24.0, TOKEN_Z / 2.0 + 2.0);
    }

    let elapsed_gate = centered_cube(
        "closed_sensor_probe_hydration_drift_elapsed_time_gate_label_land",
        TOKEN_X - 36.0,
        11.0,
        3.0,
    )
    .translate(0.0, TOKEN_Y / 2.0 - 18.0, TOKEN_Z / 2.0 + 1.5);

    body + stops + labels + elapsed_gate - slots + gripper_fiducials("time_token_panel", 110.0)
}

fn token_code(index: usize) -> u16 {
    match index {
        0 => 0b000001111,
        1 => 0b000011011,
        2 => 0b000110101,
        3 => 0b001101001,
        4 => 0b011010001,
        5 => 0b101100001,
        6 => 0b110100010,
        _ => 0b111000100,
    }
}

fn cable_strain_relief() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_cable_strain_relief_comb_body",
        CABLE_X,
        CABLE_Y,
        CABLE_Z,
    );
    let rear_bulkhead = centered_cube(
        "closed_sensor_probe_hydration_drift_cable_rear_bulkhead",
        CABLE_X,
        16.0,
        CABLE_Z + 34.0,
    )
    .translate(0.0, CABLE_Y / 2.0 - 8.0, 17.0);
    let clamp_bar = centered_cube(
        "closed_sensor_probe_hydration_drift_cable_compression_clamp_bar",
        CABLE_X - 38.0,
        18.0,
        18.0,
    )
    .translate(0.0, -CABLE_Y / 2.0 + 34.0, CABLE_Z / 2.0 + 9.0);

    let mut channels = Part::empty("closed_sensor_probe_hydration_drift_cable_channel_cuts");
    let mut saddles = Part::empty("closed_sensor_probe_hydration_drift_cable_saddles");
    for i in 0..CABLE_CHANNEL_COUNT {
        let y = centered_index(i, CABLE_CHANNEL_COUNT, CABLE_CHANNEL_PITCH);
        channels = channels
            + centered_cylinder(
                format!("closed_sensor_probe_hydration_drift_cable_channel_bore_{i}"),
                (PROBE_CABLE_D + 1.2) / 2.0,
                CABLE_X + 8.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -5.0);
        saddles = saddles
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_cable_channel_{i}_serial_tab"),
                42.0,
                7.0,
                5.0,
            )
            .translate(-CABLE_X / 2.0 + 58.0, y, CABLE_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_cable_channel_{i}_probe_tab"),
                42.0,
                7.0,
                5.0,
            )
            .translate(CABLE_X / 2.0 - 58.0, y, CABLE_Z / 2.0 + 2.5);
    }

    let mut tie_slots = Part::empty("closed_sensor_probe_hydration_drift_cable_tie_slots");
    for i in 0..CABLE_CLAMP_COUNT {
        let x = centered_index(i, CABLE_CLAMP_COUNT, 72.0);
        tie_slots = tie_slots
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_cable_tie_slot_{i}"),
                10.0,
                CABLE_Y - 42.0,
                12.0,
            )
            .translate(x, 0.0, CABLE_Z / 2.0 - 6.0);
    }

    body + rear_bulkhead + clamp_bar + saddles - channels - tie_slots
        + gripper_fiducials("cable_relief", 126.0)
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_sensor_probe_hydration_drift_barcode_certificate_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut barcode_lands = Part::empty("closed_sensor_probe_hydration_drift_barcode_serial_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        barcode_lands = barcode_lands
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_probe_barcode_land_{i}"),
                58.0,
                20.0,
                3.0,
            )
            .translate(
                centered_index(col, 4, 72.0),
                centered_index(row, 2, 34.0) + 20.0,
                TRACE_Z / 2.0 + 1.5,
            )
            + barcode_tick_pattern(
                &format!("closed_sensor_probe_hydration_drift_probe_barcode_ticks_{i}"),
                i,
            )
            .translate(
                centered_index(col, 4, 72.0),
                centered_index(row, 2, 34.0) + 20.0,
                TRACE_Z / 2.0 + 4.0,
            );
    }

    let mut certificate_lands =
        Part::empty("closed_sensor_probe_hydration_drift_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        certificate_lands = certificate_lands
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_certificate_card_land_{i}"),
                62.0,
                28.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LANDS, 76.0),
                -36.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }

    let mut run_record_lands = Part::empty("closed_sensor_probe_hydration_drift_run_record_lands");
    for i in 0..RUN_RECORD_LANDS {
        run_record_lands = run_record_lands
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_run_record_land_{i}"),
                118.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(i, RUN_RECORD_LANDS, 138.0),
                -TRACE_Y / 2.0 + 14.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }

    panel + barcode_lands + certificate_lands + run_record_lands
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_sensor_probe_hydration_drift_release_hold_reject_lane_body",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut lane_cuts =
        Part::empty("closed_sensor_probe_hydration_drift_release_hold_reject_slot_cuts");
    let mut lane_labels =
        Part::empty("closed_sensor_probe_hydration_drift_release_hold_reject_label_lands");
    let mut lane_end_stops =
        Part::empty("closed_sensor_probe_hydration_drift_release_hold_reject_end_stops");

    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 44.0);
        lane_labels = lane_labels
            + centered_cube(
                format!(
                    "closed_sensor_probe_hydration_drift_{}_lane_label_land",
                    disposition_name(lane)
                ),
                84.0,
                11.0,
                3.0,
            )
            .translate(-DISPOSITION_X / 2.0 + 54.0, y, DISPOSITION_Z / 2.0 + 1.5)
            + coded_label(
                &format!(
                    "closed_sensor_probe_hydration_drift_{}_lane_csg_label",
                    disposition_name(lane)
                ),
                disposition_code(lane),
                48.0,
            )
            .translate(DISPOSITION_X / 2.0 - 56.0, y, DISPOSITION_Z / 2.0 + 2.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            let x = centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 58.0);
            lane_cuts = lane_cuts
                + centered_cube(
                    format!(
                        "closed_sensor_probe_hydration_drift_{}_lane_probe_slot_{slot}",
                        disposition_name(lane)
                    ),
                    44.0,
                    24.0,
                    24.0,
                )
                .translate(x, y, DISPOSITION_Z / 2.0 - 10.0);
            lane_end_stops = lane_end_stops
                + centered_cube(
                    format!(
                        "closed_sensor_probe_hydration_drift_{}_lane_slot_{slot}_end_stop",
                        disposition_name(lane)
                    ),
                    46.0,
                    5.0,
                    11.0,
                )
                .translate(x, y + 17.0, DISPOSITION_Z / 2.0 + 5.5);
        }
    }

    let reject_witness_cup = centered_cylinder(
        "closed_sensor_probe_hydration_drift_reject_lane_leak_witness_cup",
        16.0,
        16.0,
        36,
    )
    .translate(
        DISPOSITION_X / 2.0 - 32.0,
        -DISPOSITION_Y / 2.0 + 24.0,
        DISPOSITION_Z / 2.0 + 8.0,
    );

    body + lane_labels + lane_end_stops + reject_witness_cup - lane_cuts
        + gripper_fiducials("disposition_lanes", 132.0)
}

fn disposition_name(index: usize) -> &'static str {
    match index {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn disposition_code(index: usize) -> u16 {
    match index {
        0 => 0b101010101,
        1 => 0b111000111,
        _ => 0b100111001,
    }
}

fn removable_leak_tray_insert() -> Part {
    let tray = centered_cube(
        "closed_sensor_probe_hydration_drift_removable_leak_tray_insert_body",
        LEAK_INSERT_X,
        LEAK_INSERT_Y,
        LEAK_INSERT_Z,
    );
    let basin = centered_cube(
        "closed_sensor_probe_hydration_drift_removable_leak_tray_basin_cut",
        LEAK_INSERT_X - 42.0,
        LEAK_INSERT_Y - 38.0,
        18.0,
    )
    .translate(0.0, 0.0, LEAK_INSERT_Z / 2.0 - 9.0);
    let drain = centered_cylinder(
        "closed_sensor_probe_hydration_drift_removable_leak_tray_drain_bore",
        LEAK_DRAIN_PORT_D / 2.0,
        38.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_INSERT_X / 2.0 - 36.0, -LEAK_INSERT_Y / 2.0 + 18.0, 0.0);

    let mut ribs = Part::empty("closed_sensor_probe_hydration_drift_leak_tray_witness_ribs");
    for i in 0..LEAK_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_leak_tray_witness_rib_{i}"),
                LEAK_INSERT_X - 66.0,
                4.0,
                5.0,
            )
            .translate(
                0.0,
                centered_index(i, LEAK_WITNESS_RIBS, 18.0),
                LEAK_INSERT_Z / 2.0 + 2.5,
            );
    }

    let witness_card_land = centered_cube(
        "closed_sensor_probe_hydration_drift_leak_tray_witness_card_land",
        118.0,
        28.0,
        4.0,
    )
    .translate(
        -LEAK_INSERT_X / 2.0 + 88.0,
        -LEAK_INSERT_Y / 2.0 + 28.0,
        LEAK_INSERT_Z / 2.0 + 2.0,
    );

    tray + ribs + witness_card_land - basin - drain + gripper_fiducials("leak_tray_insert", 130.0)
}

fn csg_label_geometry() -> Part {
    let rail = centered_cube(
        "closed_sensor_probe_hydration_drift_csg_label_front_rail",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    );
    let mut chips = Part::empty("closed_sensor_probe_hydration_drift_raised_csg_label_chips");
    for (i, (name, code)) in [
        ("hydrated", 0b101011001),
        ("dry", 0b110010101),
        ("standard", 0b111100010),
        ("dark", 0b100101111),
        ("time", 0b101100110),
        ("cable", 0b111010100),
        ("release", 0b101010101),
        ("hold", 0b111000111),
        ("reject", 0b100111001),
    ]
    .iter()
    .enumerate()
    {
        chips = chips
            + centered_cube(
                format!("closed_sensor_probe_hydration_drift_{name}_label_chip_land"),
                82.0,
                24.0,
                3.0,
            )
            .translate(
                centered_index(i, LABEL_CHIPS, 98.0),
                0.0,
                LABEL_Z / 2.0 + 1.5,
            )
            + coded_label(
                &format!("closed_sensor_probe_hydration_drift_{name}_raised_csg_label_code"),
                *code,
                62.0,
            )
            .translate(
                centered_index(i, LABEL_CHIPS, 98.0),
                0.0,
                LABEL_Z / 2.0 + 4.0,
            );
    }
    rail + chips
}

fn robot_service_keepout_gauge() -> Part {
    let deck = centered_cube(
        "closed_sensor_probe_hydration_drift_robot_service_keepout_gauge_plate",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let front_sweep = centered_cube(
        "closed_sensor_probe_hydration_drift_front_robot_sweep_keepout",
        KEEP_OUT_X - 120.0,
        36.0,
        4.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 52.0, KEEP_OUT_Z / 2.0 + 2.0);
    let rear_service = centered_cube(
        "closed_sensor_probe_hydration_drift_rear_service_cable_sweep_keepout",
        KEEP_OUT_X - 150.0,
        34.0,
        4.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 52.0, KEEP_OUT_Z / 2.0 + 2.0);
    let left_probe_lift = centered_cube(
        "closed_sensor_probe_hydration_drift_left_probe_lift_keepout",
        42.0,
        KEEP_OUT_Y - 150.0,
        4.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 76.0, 0.0, KEEP_OUT_Z / 2.0 + 2.0);
    let right_certificate_access = centered_cube(
        "closed_sensor_probe_hydration_drift_right_certificate_access_keepout",
        42.0,
        KEEP_OUT_Y - 150.0,
        4.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 76.0, 0.0, KEEP_OUT_Z / 2.0 + 2.0);
    let overhead_dark_cover_lift = centered_cube(
        "closed_sensor_probe_hydration_drift_dark_cover_vertical_lift_keepout",
        DARK_COVER_X + 72.0,
        DARK_COVER_Y + 72.0,
        4.0,
    )
    .translate(DARK_COVER_POS.0, DARK_COVER_POS.1, KEEP_OUT_Z / 2.0 + 2.0);

    deck + front_sweep
        + rear_service
        + left_probe_lift
        + right_certificate_access
        + overhead_dark_cover_lift
}

fn coded_label(name: &str, code: u16, width: f64) -> Part {
    let base = centered_cube(format!("{name}_base"), width, 8.0, 1.6);
    let mut bars = Part::empty(format!("{name}_bars"));
    for bit in 0..9 {
        if (code >> bit) & 1 == 1 {
            bars = bars
                + centered_cube(format!("{name}_bar_{bit}"), 3.0, 8.0, 3.0).translate(
                    centered_index(bit, 9, width / 10.0),
                    0.0,
                    2.3,
                );
        }
    }
    base + bars
}

fn barcode_tick_pattern(name: &str, index: usize) -> Part {
    let mut ticks = Part::empty(name);
    for bit in 0..8 {
        let tall = ((index + bit) % 3) == 0;
        ticks = ticks
            + centered_cube(
                format!("{name}_tick_{bit}"),
                2.0,
                if tall { 16.0 } else { 10.0 },
                2.0,
            )
            .translate(centered_index(bit, 8, 6.0), 0.0, 0.0);
    }
    ticks
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 10.0, 3.0, 40);
    let center = centered_cylinder(format!("{name}_center_dot"), 3.0, 4.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 24.0, 2.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.0, 24.0, 4.0);
    disc + center + cross_x + cross_y
}

fn gripper_fiducials(prefix: &str, span_x: f64) -> Part {
    fiducial_disc(&format!(
        "closed_sensor_probe_hydration_drift_{prefix}_left_fiducial"
    ))
    .translate(-span_x / 2.0, 0.0, 2.0)
        + fiducial_disc(&format!(
            "closed_sensor_probe_hydration_drift_{prefix}_right_fiducial"
        ))
        .translate(span_x / 2.0, 0.0, 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_manifest_covers_requested_scope() {
        for feature in [
            "hydrated_probe_nests",
            "dry_probe_nests",
            "reference_standard_wells",
            "dark_cover_witness",
            "time_token_slots",
            "cable_strain_relief",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "leak_tray",
            "labels_as_csg_geometry",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn modules_are_inside_deck_and_non_overlapping() {
        let footprints = module_footprints();
        for module in footprints {
            assert!(module.fits_inside_deck(), "{} out of bounds", module.name);
        }
        for (index, a) in footprints.iter().enumerate() {
            for b in footprints.iter().skip(index + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
    }

    #[test]
    fn counts_match_validation_lanes() {
        assert_eq!(PROBE_COUNT, PROBE_ROWS * PROBE_COLS);
        assert_eq!(TIME_TOKEN_SLOTS, PROBE_COUNT);
        assert_eq!(BARCODE_LANDS, PROBE_COUNT);
        assert_eq!(CABLE_CHANNEL_COUNT, PROBE_COUNT);
        assert_eq!(DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE, 12);
    }
}
