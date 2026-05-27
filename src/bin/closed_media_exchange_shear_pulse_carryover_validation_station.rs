use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-exchange carryover and shear-pulse validation station.
//
// Intent:
// - Qualify no-cell media exchange at a 16-slot cassette scale before live
//   tissue-chip runs are connected.
// - Make incomplete exchange, media-to-media carryover, transient pressure and
//   flow spikes, shear limiter behavior, bubble/dead-volume bias, and per-slot
//   washout evidence physically visible on a clean-isolator deck.
// - Keep feed, flush, harvest, waste, evidence, and disposition features
//   separate enough that an automated run can be audited without opening the
//   closed validation path.
//
// This is validation fixture CAD. It is not a biological protocol, sterile
// barrier claim, acceptance criterion, calibrated sensor spec, or wetted
// material release drawing.

const PREFIX: &str = "closed_media_exchange_shear_pulse_carryover_validation_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_containment_deck.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_sixteen_slot_cassette_surrogate.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_feed_flush_harvest_route_combs.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_residual_carryover_witness_wells.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_pressure_flow_pulse_sensor_docks.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_shear_pulse_limiter_gauges.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_bubble_dead_volume_windows.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_waste_quarantine_capture.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_camera_illumination_fiducials.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_barcode_run_token_lands.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_release_hold_reject_disposition_lanes.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_isolator_service_keepouts.stl",
    "output/closed_media_exchange_shear_pulse_carryover_validation_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 13] = [
    "containment_deck",
    "sixteen_slot_cassette_surrogate",
    "feed_flush_harvest_route_combs",
    "residual_carryover_witness_wells",
    "pressure_flow_pulse_sensor_docks",
    "shear_pulse_limiter_gauges",
    "bubble_dead_volume_windows",
    "waste_quarantine_capture",
    "camera_illumination_fiducials",
    "barcode_run_token_lands",
    "release_hold_reject_disposition_lanes",
    "isolator_service_keepouts",
    "assembly",
];

const PARAMETER_SET_REV: &str = "media-exchange-shear-pulse-carryover-validation-rev-a";
const OUTPUT_MANIFEST_REV: &str = "source-only-stl-manifest-rev-a";
const USES_RANDOMNESS: bool = false;
const RANDOM_SEED: u64 = 0;
const CYLINDER_SEGMENTS: u32 = 32;
const FIDUCIAL_SEGMENTS: u32 = 36;
const FACET_TOLERANCE_MM: f64 = 0.25;

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 880.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 50.0;
const SOCKET_DEPTH: f64 = 6.0;
const MODULE_MARGIN_MM: f64 = 16.0;
const MODULE_GAP_MM: f64 = 8.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MOUNT_HOLES: usize = 8;
const LEAK_WITNESS_WELLS: usize = 8;
const DATUM_FIDUCIALS: usize = 6;

const SLOT_ROWS: usize = 4;
const SLOT_COLS: usize = 4;
const SLOT_COUNT: usize = SLOT_ROWS * SLOT_COLS;
const ROUTE_STAGES: usize = 3;
const ROUTE_LEGS: usize = SLOT_COUNT * ROUTE_STAGES;
const FEED_ROUTES: usize = SLOT_COUNT;
const FLUSH_ROUTES: usize = SLOT_COUNT;
const HARVEST_ROUTES: usize = SLOT_COUNT;
const PER_SLOT_WASHOUT_WINDOWS: usize = SLOT_COUNT;

const CASSETTE_POS: (f64, f64) = (-430.0, 214.0);
const CASSETTE_X: f64 = 444.0;
const CASSETTE_Y: f64 = 286.0;
const CASSETTE_Z: f64 = 58.0;
const SLOT_PITCH_X: f64 = 86.0;
const SLOT_PITCH_Y: f64 = 54.0;
const SLOT_SOCKET_X: f64 = 58.0;
const SLOT_SOCKET_Y: f64 = 30.0;
const SLOT_SOCKET_DEPTH: f64 = 18.0;
const SLOT_ID_TABS: usize = SLOT_COUNT;
const CASSETTE_DATUM_PINS: usize = 6;
const SURROGATE_LATCHES: usize = 4;

const ROUTE_POS: (f64, f64) = (168.0, 214.0);
const ROUTE_X: f64 = 650.0;
const ROUTE_Y: f64 = 286.0;
const ROUTE_Z: f64 = 54.0;
const ROUTE_STAGE_PITCH_Y: f64 = 76.0;
const ROUTE_CHANNEL_PITCH_X: f64 = 36.0;
const ROUTE_CHANNEL_W: f64 = 7.0;
const ROUTE_TOOTH_X: f64 = 16.0;
const ROUTE_TOOTH_Y: f64 = 40.0;
const ROUTE_BORE_D: f64 = 5.6;
const ROUTE_COMB_KEYS: usize = ROUTE_STAGES * 2;

const WITNESS_POS: (f64, f64) = (-454.0, -60.0);
const WITNESS_X: f64 = 430.0;
const WITNESS_Y: f64 = 196.0;
const WITNESS_Z: f64 = 38.0;
const RESIDUAL_WELLS: usize = SLOT_COUNT;
const CARRYOVER_WELLS: usize = SLOT_COUNT;
const CONTROL_WELLS: usize = 4;
const TOTAL_WITNESS_WELLS: usize = RESIDUAL_WELLS + CARRYOVER_WELLS + CONTROL_WELLS;
const WITNESS_COLS: usize = 9;
const WITNESS_ROWS: usize = 4;
const WITNESS_PITCH_X: f64 = 42.0;
const WITNESS_PITCH_Y: f64 = 42.0;
const WITNESS_WELL_D: f64 = 19.0;
const WITNESS_RIM_D: f64 = 25.0;
const DYE_GRADIENT_REFERENCES: usize = 6;

const SENSOR_POS: (f64, f64) = (2.0, -60.0);
const SENSOR_X: f64 = 456.0;
const SENSOR_Y: f64 = 196.0;
const SENSOR_Z: f64 = 48.0;
const PRESSURE_SENSOR_DOCKS: usize = SLOT_COUNT;
const FLOW_PULSE_SENSOR_DOCKS: usize = SLOT_COUNT;
const SENSOR_DOCK_PAIRS: usize = SLOT_COUNT;
const SENSOR_PAIR_PITCH_X: f64 = 52.0;
const PRESSURE_DOCK_X: f64 = 28.0;
const FLOW_DOCK_X: f64 = 34.0;
const SENSOR_DOCK_Y: f64 = 62.0;
const SENSOR_BORE_D: f64 = 6.0;
const PULSE_TRACE_TICKS: usize = 7;

const SHEAR_POS: (f64, f64) = (452.0, -60.0);
const SHEAR_X: f64 = 318.0;
const SHEAR_Y: f64 = 196.0;
const SHEAR_Z: f64 = 42.0;
const SHEAR_LIMITER_GAUGES: usize = 8;
const SHEAR_GAUGE_PITCH_X: f64 = 36.0;
const SHEAR_GAUGE_D: f64 = 24.0;
const SHEAR_LIMIT_STEPS: usize = 4;
const SHEAR_PULSE_BYPASS_SLOTS: usize = 4;

const WINDOW_POS: (f64, f64) = (-456.0, -282.0);
const WINDOW_X: f64 = 420.0;
const WINDOW_Y: f64 = 150.0;
const WINDOW_Z: f64 = 36.0;
const BUBBLE_WINDOWS: usize = 8;
const DEAD_VOLUME_WINDOWS: usize = 8;
const WINDOW_PITCH_X: f64 = 48.0;
const WINDOW_PANE_X: f64 = 34.0;
const WINDOW_PANE_Y: f64 = 76.0;
const DEAD_VOLUME_TICKS_PER_WINDOW: usize = 5;
const BUBBLE_REFERENCE_BEADS: usize = 16;

const WASTE_POS: (f64, f64) = (4.0, -282.0);
const WASTE_X: f64 = 468.0;
const WASTE_Y: f64 = 150.0;
const WASTE_Z: f64 = 60.0;
const WASTE_QUARANTINE_BOTTLES: usize = 6;
const WASTE_BOTTLE_D: f64 = 34.0;
const WASTE_BOTTLE_CLEARANCE_D: f64 = 38.0;
const WASTE_STREAM_PORTS: usize = ROUTE_STAGES * 2;
const WASTE_SEGREGATION_RIBS: usize = 5;
const WASTE_CAPTURE_VOLUME_ML: f64 = 960.0;

const CAMERA_POS: (f64, f64) = (432.0, -282.0);
const CAMERA_X: f64 = 302.0;
const CAMERA_Y: f64 = 150.0;
const CAMERA_Z: f64 = 186.0;
const CAMERA_FIDUCIALS: usize = 4;
const ILLUMINATION_BARS: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 158.0;
const FIDUCIAL_TARGET_D: f64 = 18.0;

const TRACE_POS: (f64, f64) = (-340.0, 384.0);
const TRACE_X: f64 = 520.0;
const TRACE_Y: f64 = 34.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = SLOT_COUNT;
const RUN_TOKEN_LANDS: usize = 8;
const TRACE_CARD_LANDS: usize = 4;
const TRACE_LAND_PITCH_X: f64 = 30.0;

const DISPOSITION_POS: (f64, f64) = (332.0, 384.0);
const DISPOSITION_X: f64 = 520.0;
const DISPOSITION_Y: f64 = 34.0;
const DISPOSITION_Z: f64 = 34.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 6;
const DISPOSITION_SLOT_X: f64 = 60.0;
const DISPOSITION_SLOT_Y: f64 = 8.0;
const RELEASE_LANE_INDEX: usize = 0;
const HOLD_LANE_INDEX: usize = 1;
const REJECT_LANE_INDEX: usize = 2;

const ROBOT_KEEP_OUT_X: f64 = 1160.0;
const ROBOT_KEEP_OUT_Y: f64 = 640.0;
const ROBOT_KEEP_OUT_Z: f64 = 196.0;
const FRONT_ROBOT_CLEARANCE: f64 = 405.0;
const REAR_ISOLATOR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_CASSETTE_SERVICE_CLEARANCE: f64 = 220.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 230.0;
const TOP_CASSETTE_LIFT_CLEARANCE: f64 = 285.0;
const KEEP_OUT_RAIL_W: f64 = 8.0;
const SERVICE_GAUGES: usize = 5;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - MODULE_MARGIN_MM
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - MODULE_MARGIN_MM
    }

    fn overlaps(self, other: Footprint, margin: f64) -> bool {
        let a = rect(self.center, self.x, self.y);
        let b = rect(other.center, other.x, other.y);
        a.0 < b.1 + margin && a.1 + margin > b.0 && a.2 < b.3 + margin && a.3 + margin > b.2
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let cassette = sixteen_slot_cassette_surrogate();
    export(OUTPUTS[1], &cassette);

    let routes = feed_flush_harvest_route_combs();
    export(OUTPUTS[2], &routes);

    let witnesses = residual_carryover_witness_wells();
    export(OUTPUTS[3], &witnesses);

    let sensors = pressure_flow_pulse_sensor_docks();
    export(OUTPUTS[4], &sensors);

    let shear = shear_pulse_limiter_gauges();
    export(OUTPUTS[5], &shear);

    let windows = bubble_dead_volume_windows();
    export(OUTPUTS[6], &windows);

    let waste = waste_quarantine_capture();
    export(OUTPUTS[7], &waste);

    let cameras = camera_illumination_fiducials();
    export(OUTPUTS[8], &cameras);

    let trace = barcode_run_token_lands();
    export(OUTPUTS[9], &trace);

    let disposition = release_hold_reject_disposition_lanes();
    export(OUTPUTS[10], &disposition);

    let keepouts = isolator_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media-exchange shear-pulse/carryover validation station:");
    println!(
        "  Footprint:      {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck for clean-isolator qualification"
    );
    println!(
        "  Cassette scale: {SLOT_COUNT} no-cell cassette surrogate slots in a {SLOT_ROWS}x{SLOT_COLS} grid with {CASSETTE_DATUM_PINS} datum pins"
    );
    println!(
        "  Routes:         {FEED_ROUTES} feed, {FLUSH_ROUTES} flush, and {HARVEST_ROUTES} harvest route legs ({ROUTE_LEGS} total)"
    );
    println!(
        "  Witnesses:      {RESIDUAL_WELLS} residual wells, {CARRYOVER_WELLS} carryover wells, {CONTROL_WELLS} controls, {PER_SLOT_WASHOUT_WINDOWS} per-slot washout windows"
    );
    println!(
        "  Dynamics:       {PRESSURE_SENSOR_DOCKS} pressure docks, {FLOW_PULSE_SENSOR_DOCKS} flow-pulse docks, {SHEAR_LIMITER_GAUGES} shear-pulse limiter gauges"
    );
    println!(
        "  Evidence:       {BUBBLE_WINDOWS} bubble windows, {DEAD_VOLUME_WINDOWS} dead-volume windows, {WASTE_QUARANTINE_BOTTLES} waste quarantine captures, {CAMERA_FIDUCIALS} camera fiducials, {ILLUMINATION_BARS} illumination bars"
    );
    println!(
        "  Traceability:   {BARCODE_LANDS} barcode lands, {RUN_TOKEN_LANDS} run-token lands, and release/hold/reject disposition lanes"
    );
    println!(
        "  Source metadata: {PARAMETER_SET_REV}, {OUTPUT_MANIFEST_REV}, randomness={USES_RANDOMNESS}, seed={RANDOM_SEED}, facet tolerance {FACET_TOLERANCE_MM:.2}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn station_assembly() -> Part {
    containment_deck()
        + sixteen_slot_cassette_surrogate().translate(
            CASSETTE_POS.0,
            CASSETTE_POS.1,
            on_deck_z(CASSETTE_Z),
        )
        + feed_flush_harvest_route_combs().translate(ROUTE_POS.0, ROUTE_POS.1, on_deck_z(ROUTE_Z))
        + residual_carryover_witness_wells().translate(
            WITNESS_POS.0,
            WITNESS_POS.1,
            on_deck_z(WITNESS_Z),
        )
        + pressure_flow_pulse_sensor_docks().translate(
            SENSOR_POS.0,
            SENSOR_POS.1,
            on_deck_z(SENSOR_Z),
        )
        + shear_pulse_limiter_gauges().translate(SHEAR_POS.0, SHEAR_POS.1, on_deck_z(SHEAR_Z))
        + bubble_dead_volume_windows().translate(WINDOW_POS.0, WINDOW_POS.1, on_deck_z(WINDOW_Z))
        + waste_quarantine_capture().translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_Z))
        + camera_illumination_fiducials().translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
        + barcode_run_token_lands().translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + release_hold_reject_disposition_lanes().translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            on_deck_z(DISPOSITION_Z),
        )
        + route_evidence_tubes()
        + isolator_service_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        format!("{PREFIX}_recessed_wet_validation_sump"),
        STATION_X - 126.0,
        STATION_Y - 124.0,
        8.0,
    )
    .translate(0.0, -18.0, BASE_Z - 4.0);
    let upper_wet_zone = centered_cube(
        format!("{PREFIX}_cassette_route_wet_zone_recess"),
        1155.0,
        286.0,
        8.0,
    )
    .translate(-118.0, 214.0, BASE_Z - 4.2);
    let witness_zone = centered_cube(
        format!("{PREFIX}_witness_sensor_zone_recess"),
        1220.0,
        196.0,
        8.0,
    )
    .translate(-10.0, -60.0, BASE_Z - 4.2);
    let lower_wet_zone = centered_cube(
        format!("{PREFIX}_bubble_waste_camera_zone_recess"),
        1220.0,
        150.0,
        8.0,
    )
    .translate(-10.0, -282.0, BASE_Z - 4.2);
    let drain = centered_cylinder(
        format!("{PREFIX}_quarantine_low_point_drain"),
        DRAIN_D / 2.0,
        RIM_W + 42.0,
        CYLINDER_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -STATION_Y / 2.0 + 11.0,
        BASE_Z - 6.0,
    );

    deck - sump
        - upper_wet_zone
        - witness_zone
        - lower_wet_zone
        - drain
        - insert_sockets()
        - mount_holes()
        + containment_rims()
        + deck_zone_dividers()
        + leak_witness_well_rail()
        + clean_isolator_datum_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_registration_sockets"));
    for spec in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket", spec.name),
                spec.x + 10.0,
                spec.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                spec.center.0,
                spec.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_deck_mount_holes"));
    let points = [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
    ];
    for (idx, (x, y)) in points.iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_deck_mount_hole_{idx}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn containment_rims() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn deck_zone_dividers() -> Part {
    let wet_to_witness = centered_cube(
        format!("{PREFIX}_raised_divider_cassette_to_witness_zone"),
        1240.0,
        8.0,
        18.0,
    )
    .translate(-12.0, 77.0, BASE_Z + 9.0);
    let witness_to_lower = centered_cube(
        format!("{PREFIX}_raised_divider_witness_to_waste_zone"),
        1240.0,
        8.0,
        18.0,
    )
    .translate(-12.0, -176.0, BASE_Z + 9.0);
    let trace_barrier = centered_cube(
        format!("{PREFIX}_raised_trace_disposition_barrier"),
        8.0,
        86.0,
        18.0,
    )
    .translate(0.0, 360.0, BASE_Z + 9.0);
    wet_to_witness + witness_to_lower + trace_barrier
}

fn leak_witness_well_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_leak_witness_rail_body"),
        36.0,
        STATION_Y - 160.0,
        12.0,
    )
    .translate(-STATION_X / 2.0 + 96.0, 0.0, BASE_Z + 6.0);
    let mut wells = Part::empty(format!("{PREFIX}_leak_witness_well_cuts"));
    for idx in 0..LEAK_WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_leak_witness_well_cut_{idx}"),
                8.0,
                14.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                -STATION_X / 2.0 + 96.0,
                grid_center(idx, LEAK_WITNESS_WELLS, 72.0),
                BASE_Z + 7.0,
            );
    }
    rail - wells
}

fn clean_isolator_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_clean_isolator_datum_fiducials"));
    let points = [
        (-610.0, -346.0),
        (610.0, -346.0),
        (-610.0, 346.0),
        (610.0, 346.0),
        (-60.0, 346.0),
        (60.0, 346.0),
    ];
    for (idx, (x, y)) in points.iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(format!("{PREFIX}_deck_optical_datum_{idx}"), 22.0, 4.0).translate(
                *x,
                *y,
                BASE_Z + 2.0,
            );
    }
    fiducials
}

fn sixteen_slot_cassette_surrogate() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_cassette_surrogate_body"),
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    let tray_recess = centered_cube(
        format!("{PREFIX}_cassette_surrogate_clear_lid_recess"),
        CASSETTE_X - 48.0,
        CASSETTE_Y - 44.0,
        16.0,
    )
    .translate(0.0, 0.0, CASSETTE_Z / 2.0 - 8.0);

    body - tray_recess - slot_socket_cuts() - cassette_port_bores() - cassette_datum_holes()
        + slot_id_tabs()
        + cassette_gasket_land_grid()
        + surrogate_latches()
}

fn slot_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_cassette_slot_socket_cuts"));
    for idx in 0..SLOT_COUNT {
        let (x, y) = slot_center(idx);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_no_cell_chip_socket_cut"),
                SLOT_SOCKET_X,
                SLOT_SOCKET_Y,
                SLOT_SOCKET_DEPTH + 0.4,
            )
            .translate(x, y, CASSETTE_Z / 2.0 - SLOT_SOCKET_DEPTH / 2.0 + 0.2)
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_clear_optical_window_cut"),
                SLOT_SOCKET_X - 16.0,
                SLOT_SOCKET_Y - 10.0,
                CASSETTE_Z + 2.0,
            )
            .translate(x, y, 0.0);
    }
    cuts
}

fn cassette_port_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_cassette_feed_flush_harvest_port_bores"));
    for idx in 0..SLOT_COUNT {
        let (x, y) = slot_center(idx);
        for (stage, dy) in [(-12.0, "feed"), (0.0, "flush"), (12.0, "harvest")] {
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_slot_{idx:02}_{dy}_port_bore"),
                    ROUTE_BORE_D / 2.0,
                    CASSETTE_X + 18.0,
                    CYLINDER_SEGMENTS,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(x, y + stage, CASSETTE_Z / 2.0 - 12.0);
        }
    }
    bores
}

fn cassette_datum_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_cassette_datum_holes"));
    let points = [
        (-CASSETTE_X / 2.0 + 32.0, -CASSETTE_Y / 2.0 + 32.0),
        (CASSETTE_X / 2.0 - 32.0, -CASSETTE_Y / 2.0 + 32.0),
        (-CASSETTE_X / 2.0 + 32.0, CASSETTE_Y / 2.0 - 32.0),
        (CASSETTE_X / 2.0 - 32.0, CASSETTE_Y / 2.0 - 32.0),
        (0.0, -CASSETTE_Y / 2.0 + 32.0),
        (0.0, CASSETTE_Y / 2.0 - 32.0),
    ];
    for (idx, (x, y)) in points.iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_cassette_datum_pin_hole_{idx}"),
                4.0,
                CASSETTE_Z + 3.0,
                CYLINDER_SEGMENTS,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn slot_id_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_cassette_slot_id_tabs"));
    for idx in 0..SLOT_COUNT {
        let (x, y) = slot_center(idx);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_raised_id_tab"),
                34.0,
                8.0,
                4.0,
            )
            .translate(x, y + SLOT_SOCKET_Y / 2.0 + 10.0, CASSETTE_Z / 2.0 + 2.0);
    }
    tabs
}

fn cassette_gasket_land_grid() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_cassette_gasket_land_grid"));
    for idx in 0..SLOT_COUNT {
        let (x, y) = slot_center(idx);
        let frame = centered_cube(
            format!("{PREFIX}_slot_{idx:02}_gasket_land_outer"),
            SLOT_SOCKET_X + 12.0,
            SLOT_SOCKET_Y + 12.0,
            3.0,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 1.5);
        let opening = centered_cube(
            format!("{PREFIX}_slot_{idx:02}_gasket_land_inner_cut"),
            SLOT_SOCKET_X + 2.0,
            SLOT_SOCKET_Y + 2.0,
            4.0,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 1.5);
        lands = lands + (frame - opening);
    }
    lands
}

fn surrogate_latches() -> Part {
    let mut latches = Part::empty(format!("{PREFIX}_surrogate_latches"));
    let points = [
        (-CASSETTE_X / 2.0 - 10.0, -72.0),
        (-CASSETTE_X / 2.0 - 10.0, 72.0),
        (CASSETTE_X / 2.0 + 10.0, -72.0),
        (CASSETTE_X / 2.0 + 10.0, 72.0),
    ];
    for (idx, (x, y)) in points.iter().enumerate() {
        latches = latches
            + centered_cube(
                format!("{PREFIX}_surrogate_side_latch_{idx}"),
                20.0,
                54.0,
                20.0,
            )
            .translate(*x, *y, 4.0)
            - centered_cube(
                format!("{PREFIX}_surrogate_side_latch_grip_cut_{idx}"),
                8.0,
                34.0,
                10.0,
            )
            .translate(*x, *y, 6.0);
    }
    latches
}

fn feed_flush_harvest_route_combs() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_feed_flush_harvest_comb_body"),
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    body - route_channel_cuts() - route_bore_cuts()
        + route_comb_teeth()
        + stage_header_ridges()
        + route_key_tabs()
        + route_stage_labels()
}

fn route_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_route_channel_cuts"));
    for stage in 0..ROUTE_STAGES {
        for slot in 0..SLOT_COUNT {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_route_stage_{stage}_slot_{slot:02}_open_channel_cut"),
                    ROUTE_CHANNEL_W,
                    70.0,
                    ROUTE_CHANNEL_W + 2.0,
                )
                .translate(
                    route_slot_x(slot),
                    route_stage_y(stage),
                    ROUTE_Z / 2.0 - ROUTE_CHANNEL_W / 2.0,
                );
        }
    }
    cuts
}

fn route_bore_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_route_bore_cuts"));
    for stage in 0..ROUTE_STAGES {
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_route_stage_{stage}_common_header_bore"),
                ROUTE_BORE_D / 2.0,
                ROUTE_X - 64.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, route_stage_y(stage), 0.0);
        for slot in 0..SLOT_COUNT {
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_route_stage_{stage}_slot_{slot:02}_drop_bore"),
                    ROUTE_BORE_D / 2.0,
                    ROUTE_Z + 4.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(route_slot_x(slot), route_stage_y(stage), 0.0);
        }
    }
    cuts
}

fn route_comb_teeth() -> Part {
    let mut teeth = Part::empty(format!("{PREFIX}_route_comb_teeth"));
    for stage in 0..ROUTE_STAGES {
        for slot in 0..SLOT_COUNT {
            teeth = teeth
                + centered_cube(
                    format!("{PREFIX}_route_stage_{stage}_slot_{slot:02}_comb_tooth"),
                    ROUTE_TOOTH_X,
                    ROUTE_TOOTH_Y,
                    11.0,
                )
                .translate(
                    route_slot_x(slot),
                    route_stage_y(stage),
                    ROUTE_Z / 2.0 + 5.5,
                );
        }
    }
    teeth
}

fn stage_header_ridges() -> Part {
    let mut ridges = Part::empty(format!("{PREFIX}_stage_header_ridges"));
    for (stage, name) in ["feed", "flush", "harvest"].iter().enumerate() {
        ridges = ridges
            + centered_cube(
                format!("{PREFIX}_{name}_header_raised_ridge"),
                ROUTE_X - 48.0,
                10.0,
                7.0,
            )
            .translate(0.0, route_stage_y(stage), ROUTE_Z / 2.0 + 3.5);
    }
    ridges
}

fn route_key_tabs() -> Part {
    let mut keys = Part::empty(format!("{PREFIX}_route_comb_key_tabs"));
    for idx in 0..ROUTE_COMB_KEYS {
        let x = if idx % 2 == 0 {
            -ROUTE_X / 2.0 - 8.0
        } else {
            ROUTE_X / 2.0 + 8.0
        };
        let y = route_stage_y(idx / 2);
        keys =
            keys + centered_cube(format!("{PREFIX}_route_key_tab_{idx}"), 16.0, 42.0, 18.0)
                .translate(x, y, ROUTE_Z / 2.0);
    }
    keys
}

fn route_stage_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_route_stage_label_blocks"));
    for (stage, width) in [54.0, 64.0, 76.0].iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("{PREFIX}_route_stage_{stage}_label_length_code"),
                *width,
                8.0,
                5.0,
            )
            .translate(
                -ROUTE_X / 2.0 + 54.0,
                route_stage_y(stage) - 27.0,
                ROUTE_Z / 2.0 + 2.5,
            );
    }
    labels
}

fn residual_carryover_witness_wells() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_residual_carryover_witness_block"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    block - witness_well_cuts()
        + witness_well_rims()
        + dye_gradient_reference_rail()
        + slot_washout_bias_ticks()
}

fn witness_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_witness_well_cuts"));
    for idx in 0..TOTAL_WITNESS_WELLS {
        let (x, y) = witness_center(idx);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_witness_well_cut_{idx:02}"),
                WITNESS_WELL_D / 2.0,
                WITNESS_Z + 2.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, 0.0);
    }
    cuts
}

fn witness_well_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_witness_well_rims"));
    for idx in 0..TOTAL_WITNESS_WELLS {
        let (x, y) = witness_center(idx);
        let outer = centered_cylinder(
            format!("{PREFIX}_witness_well_rim_outer_{idx:02}"),
            WITNESS_RIM_D / 2.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, WITNESS_Z / 2.0 + 2.5);
        let inner = centered_cylinder(
            format!("{PREFIX}_witness_well_rim_inner_cut_{idx:02}"),
            WITNESS_WELL_D / 2.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, WITNESS_Z / 2.0 + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn dye_gradient_reference_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_dye_gradient_reference_rail"),
        WITNESS_X - 42.0,
        16.0,
        8.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 18.0, WITNESS_Z / 2.0 + 4.0);
    let mut chips = Part::empty(format!("{PREFIX}_dye_gradient_reference_chips"));
    for idx in 0..DYE_GRADIENT_REFERENCES {
        chips = chips
            + centered_cube(
                format!("{PREFIX}_dye_gradient_reference_chip_{idx}"),
                36.0 + idx as f64 * 6.0,
                8.0,
                4.0,
            )
            .translate(
                grid_center(idx, DYE_GRADIENT_REFERENCES, 58.0),
                -WITNESS_Y / 2.0 + 18.0,
                WITNESS_Z / 2.0 + 10.0,
            );
    }
    rail + chips
}

fn slot_washout_bias_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_slot_washout_bias_ticks"));
    for slot in 0..SLOT_COUNT {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_slot_{slot:02}_washout_bias_tick"),
                4.0,
                14.0,
                5.0,
            )
            .translate(
                grid_center(slot % SLOT_COUNT, SLOT_COUNT, 22.0),
                WITNESS_Y / 2.0 - 12.0,
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    ticks
}

fn pressure_flow_pulse_sensor_docks() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_pressure_flow_pulse_sensor_dock_body"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    body - sensor_dock_cuts() - sensor_bore_cuts()
        + pulse_trace_ticks()
        + sensor_pair_numbers()
        + sensor_cable_strain_reliefs()
}

fn sensor_dock_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_sensor_dock_cuts"));
    for idx in 0..SENSOR_DOCK_PAIRS {
        let x = sensor_pair_x(idx);
        let y = sensor_pair_y(idx);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_pressure_sensor_dock_cut"),
                PRESSURE_DOCK_X,
                SENSOR_DOCK_Y,
                22.0,
            )
            .translate(x - 12.0, y, SENSOR_Z / 2.0 - 11.0)
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_flow_pulse_sensor_dock_cut"),
                FLOW_DOCK_X,
                SENSOR_DOCK_Y,
                22.0,
            )
            .translate(x + 18.0, y, SENSOR_Z / 2.0 - 11.0);
    }
    cuts
}

fn sensor_bore_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_sensor_bore_cuts"));
    for idx in 0..SENSOR_DOCK_PAIRS {
        let x = sensor_pair_x(idx);
        let y = sensor_pair_y(idx);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_slot_{idx:02}_pressure_pulse_bore"),
                SENSOR_BORE_D / 2.0,
                SENSOR_X + 10.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y - 14.0, 0.0)
            + centered_cylinder(
                format!("{PREFIX}_slot_{idx:02}_flow_pulse_bore"),
                SENSOR_BORE_D / 2.0,
                SENSOR_X + 10.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y + 14.0, 0.0);
    }
    cuts
}

fn pulse_trace_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_pulse_trace_ticks"));
    for idx in 0..SENSOR_DOCK_PAIRS {
        let x = sensor_pair_x(idx);
        let y = sensor_pair_y(idx);
        for tick in 0..PULSE_TRACE_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{PREFIX}_slot_{idx:02}_pulse_trace_tick_{tick}"),
                    3.0,
                    11.0,
                    4.0,
                )
                .translate(x - 25.0 + tick as f64 * 8.0, y, SENSOR_Z / 2.0 + 2.0);
        }
    }
    ticks
}

fn sensor_pair_numbers() -> Part {
    let mut markers = Part::empty(format!("{PREFIX}_sensor_pair_number_markers"));
    for idx in 0..SENSOR_DOCK_PAIRS {
        markers = markers
            + centered_cube(
                format!("{PREFIX}_sensor_slot_{idx:02}_number_marker"),
                14.0 + (idx % 4) as f64 * 3.0,
                6.0,
                4.0,
            )
            .translate(
                sensor_pair_x(idx),
                sensor_pair_y(idx) + 42.0,
                SENSOR_Z / 2.0 + 2.0,
            );
    }
    markers
}

fn sensor_cable_strain_reliefs() -> Part {
    let mut reliefs = Part::empty(format!("{PREFIX}_sensor_cable_strain_reliefs"));
    for row in 0..2 {
        reliefs = reliefs
            + centered_cube(
                format!("{PREFIX}_sensor_cable_strain_relief_row_{row}"),
                SENSOR_X - 38.0,
                10.0,
                10.0,
            )
            .translate(
                0.0,
                if row == 0 { -83.0 } else { 83.0 },
                SENSOR_Z / 2.0 + 5.0,
            );
    }
    reliefs
}

fn shear_pulse_limiter_gauges() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_shear_pulse_limiter_gauge_body"),
        SHEAR_X,
        SHEAR_Y,
        SHEAR_Z,
    );
    body - shear_gauge_bores() - shear_bypass_slot_cuts()
        + shear_gauge_rims()
        + shear_limit_step_ladders()
}

fn shear_gauge_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_shear_gauge_bores"));
    for idx in 0..SHEAR_LIMITER_GAUGES {
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_shear_limiter_gauge_bore_{idx}"),
                SHEAR_GAUGE_D / 2.0,
                SHEAR_Z + 2.0,
                CYLINDER_SEGMENTS,
            )
            .translate(shear_gauge_x(idx), shear_gauge_y(idx), 0.0);
    }
    bores
}

fn shear_gauge_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_shear_gauge_rims"));
    for idx in 0..SHEAR_LIMITER_GAUGES {
        let outer = centered_cylinder(
            format!("{PREFIX}_shear_limiter_gauge_rim_outer_{idx}"),
            SHEAR_GAUGE_D / 2.0 + 4.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(shear_gauge_x(idx), shear_gauge_y(idx), SHEAR_Z / 2.0 + 2.5);
        let inner = centered_cylinder(
            format!("{PREFIX}_shear_limiter_gauge_rim_inner_cut_{idx}"),
            SHEAR_GAUGE_D / 2.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(shear_gauge_x(idx), shear_gauge_y(idx), SHEAR_Z / 2.0 + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn shear_limit_step_ladders() -> Part {
    let mut ladders = Part::empty(format!("{PREFIX}_shear_limit_step_ladders"));
    for idx in 0..SHEAR_LIMITER_GAUGES {
        for step in 0..SHEAR_LIMIT_STEPS {
            ladders = ladders
                + centered_cube(
                    format!("{PREFIX}_shear_limiter_{idx}_step_{step}"),
                    7.0 + step as f64 * 4.0,
                    5.0,
                    4.0,
                )
                .translate(
                    shear_gauge_x(idx),
                    shear_gauge_y(idx) + 26.0 + step as f64 * 7.0,
                    SHEAR_Z / 2.0 + 2.0,
                );
        }
    }
    ladders
}

fn shear_bypass_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_shear_pulse_bypass_slot_cuts"));
    for idx in 0..SHEAR_PULSE_BYPASS_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_shear_pulse_bypass_slot_cut_{idx}"),
                56.0,
                10.0,
                18.0,
            )
            .translate(
                grid_center(idx, SHEAR_PULSE_BYPASS_SLOTS, 66.0),
                -SHEAR_Y / 2.0 + 32.0,
                SHEAR_Z / 2.0 - 9.0,
            );
    }
    cuts
}

fn bubble_dead_volume_windows() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_dead_volume_window_body"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    body - bubble_window_cuts() - dead_volume_window_cuts()
        + bubble_reference_beads()
        + dead_volume_tick_ladders()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_window_cuts"));
    for idx in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_bubble_window_{idx}_clear_panel_cut"),
                WINDOW_PANE_X,
                WINDOW_PANE_Y,
                WINDOW_Z + 2.0,
            )
            .translate(grid_center(idx, BUBBLE_WINDOWS, WINDOW_PITCH_X), 28.0, 0.0);
    }
    cuts
}

fn dead_volume_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_dead_volume_window_cuts"));
    for idx in 0..DEAD_VOLUME_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_dead_volume_window_{idx}_clear_panel_cut"),
                WINDOW_PANE_X,
                WINDOW_PANE_Y - 12.0,
                WINDOW_Z + 2.0,
            )
            .translate(
                grid_center(idx, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X),
                -38.0,
                0.0,
            );
    }
    cuts
}

fn bubble_reference_beads() -> Part {
    let mut beads = Part::empty(format!("{PREFIX}_bubble_reference_beads"));
    for idx in 0..BUBBLE_REFERENCE_BEADS {
        beads = beads
            + centered_cylinder(
                format!("{PREFIX}_bubble_reference_bead_{idx}"),
                3.5,
                4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                grid_center(idx % 8, 8, WINDOW_PITCH_X),
                if idx < 8 { 76.0 } else { -76.0 },
                WINDOW_Z / 2.0 + 2.0,
            );
    }
    beads
}

fn dead_volume_tick_ladders() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_dead_volume_tick_ladders"));
    for idx in 0..DEAD_VOLUME_WINDOWS {
        for tick in 0..DEAD_VOLUME_TICKS_PER_WINDOW {
            ticks = ticks
                + centered_cube(
                    format!("{PREFIX}_dead_volume_window_{idx}_tick_{tick}"),
                    10.0 + tick as f64 * 3.0,
                    3.0,
                    4.0,
                )
                .translate(
                    grid_center(idx, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X),
                    -66.0 + tick as f64 * 7.0,
                    WINDOW_Z / 2.0 + 2.0,
                );
        }
    }
    ticks
}

fn waste_quarantine_capture() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_waste_quarantine_capture_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    body - waste_bottle_cuts() - waste_stream_port_cuts()
        + waste_bottle_rims()
        + waste_quarantine_ribs()
        + waste_chain_of_custody_tabs()
}

fn waste_bottle_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_waste_bottle_cuts"));
    for idx in 0..WASTE_QUARANTINE_BOTTLES {
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_waste_quarantine_bottle_cut_{idx}"),
                WASTE_BOTTLE_CLEARANCE_D / 2.0,
                WASTE_Z + 2.0,
                CYLINDER_SEGMENTS,
            )
            .translate(grid_center(idx, WASTE_QUARANTINE_BOTTLES, 62.0), -16.0, 0.0);
    }
    cuts
}

fn waste_bottle_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_waste_bottle_rims"));
    for idx in 0..WASTE_QUARANTINE_BOTTLES {
        let x = grid_center(idx, WASTE_QUARANTINE_BOTTLES, 62.0);
        let outer = centered_cylinder(
            format!("{PREFIX}_waste_bottle_rim_outer_{idx}"),
            WASTE_BOTTLE_CLEARANCE_D / 2.0 + 4.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, -16.0, WASTE_Z / 2.0 + 3.0);
        let inner = centered_cylinder(
            format!("{PREFIX}_waste_bottle_rim_inner_cut_{idx}"),
            WASTE_BOTTLE_D / 2.0,
            7.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, -16.0, WASTE_Z / 2.0 + 3.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn waste_stream_port_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_waste_stream_port_cuts"));
    for idx in 0..WASTE_STREAM_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_waste_stream_port_cut_{idx}"),
                5.0,
                WASTE_X + 8.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 48.0 + idx as f64 * 8.0, 0.0);
    }
    cuts
}

fn waste_quarantine_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_waste_quarantine_ribs"));
    for idx in 0..WASTE_SEGREGATION_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_waste_stream_segregation_rib_{idx}"),
                8.0,
                WASTE_Y - 24.0,
                18.0,
            )
            .translate(
                grid_center(idx, WASTE_SEGREGATION_RIBS, 72.0),
                0.0,
                WASTE_Z / 2.0 + 9.0,
            );
    }
    ribs
}

fn waste_chain_of_custody_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_waste_chain_of_custody_tabs"));
    for idx in 0..WASTE_QUARANTINE_BOTTLES {
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_waste_quarantine_custody_tab_{idx}"),
                46.0,
                12.0,
                5.0,
            )
            .translate(
                grid_center(idx, WASTE_QUARANTINE_BOTTLES, 62.0),
                -64.0,
                WASTE_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn camera_illumination_fiducials() -> Part {
    let bridge = centered_cube(
        format!("{PREFIX}_camera_illumination_bridge"),
        CAMERA_X,
        22.0,
        22.0,
    )
    .translate(0.0, 0.0, CAMERA_Z / 2.0 - 11.0);
    let left_post = centered_cube(
        format!("{PREFIX}_camera_bridge_left_post"),
        24.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(-CAMERA_X / 2.0 + 12.0, 0.0, 0.0);
    let right_post = centered_cube(
        format!("{PREFIX}_camera_bridge_right_post"),
        24.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(CAMERA_X / 2.0 - 12.0, 0.0, 0.0);
    bridge + left_post + right_post + camera_fiducial_targets() + illumination_bars()
}

fn camera_fiducial_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_camera_fiducial_targets"));
    let points = [
        (-CAMERA_X / 2.0 + 44.0, -CAMERA_Y / 2.0 + 32.0),
        (CAMERA_X / 2.0 - 44.0, -CAMERA_Y / 2.0 + 32.0),
        (-CAMERA_X / 2.0 + 44.0, CAMERA_Y / 2.0 - 32.0),
        (CAMERA_X / 2.0 - 44.0, CAMERA_Y / 2.0 - 32.0),
    ];
    for (idx, (x, y)) in points.iter().enumerate() {
        targets = targets
            + fiducial_disc(
                format!("{PREFIX}_camera_optical_fiducial_{idx}"),
                FIDUCIAL_TARGET_D,
                5.0,
            )
            .translate(*x, *y, CAMERA_Z / 2.0 + 2.5);
    }
    targets
}

fn illumination_bars() -> Part {
    let mut bars = Part::empty(format!("{PREFIX}_illumination_bars"));
    for idx in 0..ILLUMINATION_BARS {
        bars = bars
            + centered_cube(
                format!("{PREFIX}_illumination_bar_{idx}"),
                CAMERA_X - 88.0,
                8.0,
                8.0,
            )
            .translate(
                0.0,
                grid_center(idx, ILLUMINATION_BARS, 28.0),
                CAMERA_Z / 2.0 - 30.0,
            );
    }
    bars
}

fn barcode_run_token_lands() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_barcode_run_token_land_panel"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    body + barcode_lands() + run_token_lands() + trace_card_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for idx in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_slot_{idx:02}_barcode_land"),
                26.0,
                14.0,
                4.0,
            )
            .translate(
                grid_center(idx, BARCODE_LANDS, TRACE_LAND_PITCH_X),
                8.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn run_token_lands() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_run_token_lands"));
    for idx in 0..RUN_TOKEN_LANDS {
        tokens = tokens
            + centered_cylinder(
                format!("{PREFIX}_run_token_land_{idx}"),
                12.0,
                4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                grid_center(idx, RUN_TOKEN_LANDS, 44.0),
                -6.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    tokens
}

fn trace_card_lands() -> Part {
    let mut cards = Part::empty(format!("{PREFIX}_trace_card_lands"));
    for idx in 0..TRACE_CARD_LANDS {
        cards = cards
            + centered_cube(
                format!("{PREFIX}_run_record_card_land_{idx}"),
                72.0,
                20.0,
                4.0,
            )
            .translate(
                grid_center(idx, TRACE_CARD_LANDS, 96.0),
                -TRACE_Y / 2.0 + 5.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    cards
}

fn release_hold_reject_disposition_lanes() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_release_hold_reject_disposition_body"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    body - disposition_slot_cuts() + disposition_lane_labels() + disposition_gate_flags()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_disposition_slot_cuts"));
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{lane}_slot_{slot}_cut"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    14.0,
                )
                .translate(
                    grid_center(slot, DISPOSITION_SLOTS_PER_LANE, 72.0),
                    disposition_lane_y(lane),
                    DISPOSITION_Z / 2.0 - 7.0,
                );
        }
    }
    cuts
}

fn disposition_lane_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_disposition_lane_label_blocks"));
    for lane in 0..DISPOSITION_LANES {
        labels = labels
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_label_code"),
                match lane {
                    RELEASE_LANE_INDEX => 42.0,
                    HOLD_LANE_INDEX => 58.0,
                    REJECT_LANE_INDEX => 74.0,
                    _ => 30.0,
                },
                7.0,
                5.0,
            )
            .translate(
                -DISPOSITION_X / 2.0 + 48.0,
                disposition_lane_y(lane),
                DISPOSITION_Z / 2.0 + 2.5,
            );
    }
    labels
}

fn disposition_gate_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_disposition_gate_flags"));
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            flags = flags
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{lane}_slot_{slot}_gate_flag"),
                    8.0,
                    8.0,
                    12.0,
                )
                .translate(
                    grid_center(slot, DISPOSITION_SLOTS_PER_LANE, 72.0) + 32.0,
                    disposition_lane_y(lane),
                    DISPOSITION_Z / 2.0 + 6.0,
                );
        }
    }
    flags
}

fn route_evidence_tubes() -> Part {
    let mut tubes = Part::empty(format!("{PREFIX}_closed_route_evidence_tubes"));
    for stage in 0..ROUTE_STAGES {
        tubes = tubes
            + centered_cube(
                format!("{PREFIX}_cassette_to_comb_stage_{stage}_route_placeholder"),
                150.0,
                8.0,
                8.0,
            )
            .translate(
                -132.0,
                CASSETTE_POS.1 + route_stage_y(stage) * 0.45,
                BASE_Z + 42.0,
            );
    }
    for idx in 0..4 {
        tubes = tubes
            + centered_cube(
                format!("{PREFIX}_sensor_to_waste_quarantine_route_placeholder_{idx}"),
                230.0,
                7.0,
                7.0,
            )
            .translate(230.0, -190.0 - idx as f64 * 26.0, BASE_Z + 34.0);
    }
    tubes
}

fn isolator_service_keepouts() -> Part {
    let robot_front = centered_cube(
        format!("{PREFIX}_front_robot_sweep_clearance_gauge"),
        ROBOT_KEEP_OUT_X,
        KEEP_OUT_RAIL_W,
        14.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, BASE_Z + 7.0);
    let rear_service = centered_cube(
        format!("{PREFIX}_rear_isolator_service_clearance_gauge"),
        ROBOT_KEEP_OUT_X,
        KEEP_OUT_RAIL_W,
        14.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_ISOLATOR_SERVICE_CLEARANCE,
        BASE_Z + 7.0,
    );
    let left_service = centered_cube(
        format!("{PREFIX}_left_cassette_service_clearance_gauge"),
        KEEP_OUT_RAIL_W,
        ROBOT_KEEP_OUT_Y,
        14.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_CASSETTE_SERVICE_CLEARANCE,
        -10.0,
        BASE_Z + 7.0,
    );
    let right_service = centered_cube(
        format!("{PREFIX}_right_sensor_service_clearance_gauge"),
        KEEP_OUT_RAIL_W,
        ROBOT_KEEP_OUT_Y,
        14.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SENSOR_SERVICE_CLEARANCE,
        -10.0,
        BASE_Z + 7.0,
    );
    let lift_gauge = centered_cube(
        format!("{PREFIX}_top_cassette_lift_clearance_gauge"),
        CASSETTE_X + 84.0,
        CASSETTE_Y + 84.0,
        10.0,
    )
    .translate(
        CASSETTE_POS.0,
        CASSETTE_POS.1,
        BASE_Z + TOP_CASSETTE_LIFT_CLEARANCE,
    );
    let height_post = centered_cube(
        format!("{PREFIX}_robot_keepout_height_post"),
        18.0,
        18.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SENSOR_SERVICE_CLEARANCE,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        BASE_Z + ROBOT_KEEP_OUT_Z / 2.0,
    );

    robot_front
        + rear_service
        + left_service
        + right_service
        + lift_gauge
        + height_post
        + service_gauge_tokens()
}

fn service_gauge_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_service_gauge_tokens"));
    for idx in 0..SERVICE_GAUGES {
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_service_gauge_token_{idx}"),
                18.0 + idx as f64 * 10.0,
                10.0,
                5.0,
            )
            .translate(
                -STATION_X / 2.0 + 160.0 + idx as f64 * 54.0,
                -STATION_Y / 2.0 + 104.0,
                BASE_Z + 2.5,
            );
    }
    tokens
}

fn module_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "sixteen_slot_cassette_surrogate",
            center: CASSETTE_POS,
            x: CASSETTE_X,
            y: CASSETTE_Y,
        },
        Footprint {
            name: "feed_flush_harvest_route_combs",
            center: ROUTE_POS,
            x: ROUTE_X,
            y: ROUTE_Y,
        },
        Footprint {
            name: "residual_carryover_witness_wells",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "pressure_flow_pulse_sensor_docks",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Footprint {
            name: "shear_pulse_limiter_gauges",
            center: SHEAR_POS,
            x: SHEAR_X,
            y: SHEAR_Y,
        },
        Footprint {
            name: "bubble_dead_volume_windows",
            center: WINDOW_POS,
            x: WINDOW_X,
            y: WINDOW_Y,
        },
        Footprint {
            name: "waste_quarantine_capture",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Footprint {
            name: "camera_illumination_fiducials",
            center: CAMERA_POS,
            x: CAMERA_X,
            y: CAMERA_Y,
        },
        Footprint {
            name: "barcode_run_token_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Footprint {
            name: "release_hold_reject_disposition_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn assert_layout() {
    assert_eq!(SLOT_COUNT, 16);
    assert_eq!(SLOT_ROWS * SLOT_COLS, SLOT_COUNT);
    assert_eq!(SLOT_ID_TABS, SLOT_COUNT);
    assert_eq!(ROUTE_STAGES, 3);
    assert_eq!(ROUTE_LEGS, SLOT_COUNT * ROUTE_STAGES);
    assert_eq!(FEED_ROUTES, SLOT_COUNT);
    assert_eq!(FLUSH_ROUTES, SLOT_COUNT);
    assert_eq!(HARVEST_ROUTES, SLOT_COUNT);
    assert_eq!(PRESSURE_SENSOR_DOCKS, SLOT_COUNT);
    assert_eq!(FLOW_PULSE_SENSOR_DOCKS, SLOT_COUNT);
    assert_eq!(SENSOR_DOCK_PAIRS, SLOT_COUNT);
    assert_eq!(SURROGATE_LATCHES, 4);
    assert_eq!(RESIDUAL_WELLS, SLOT_COUNT);
    assert_eq!(CARRYOVER_WELLS, SLOT_COUNT);
    assert_eq!(TOTAL_WITNESS_WELLS, SLOT_COUNT * 2 + CONTROL_WELLS);
    assert_eq!(PER_SLOT_WASHOUT_WINDOWS, SLOT_COUNT);
    assert_eq!(BARCODE_LANDS, SLOT_COUNT);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(RELEASE_LANE_INDEX + HOLD_LANE_INDEX + REJECT_LANE_INDEX, 3);
    assert_eq!(MOUNT_HOLES, 8);
    assert_eq!(DATUM_FIDUCIALS, 6);
    assert!(WASTE_BOTTLE_CLEARANCE_D > WASTE_BOTTLE_D);
    assert!(WASTE_CAPTURE_VOLUME_ML >= SLOT_COUNT as f64 * ROUTE_STAGES as f64 * 18.0);
    assert!(CAMERA_CLEARANCE_Z > CASSETTE_Z + ROUTE_Z);
    assert_eq!(CYLINDER_SEGMENTS, 32);
    assert_eq!(FIDUCIAL_SEGMENTS, 36);

    let footprints = module_footprints();
    for fp in footprints {
        assert!(
            fp.fits_inside_station(),
            "{} exceeds station footprint",
            fp.name
        );
    }
    for i in 0..footprints.len() {
        for j in (i + 1)..footprints.len() {
            assert!(
                !footprints[i].overlaps(footprints[j], MODULE_GAP_MM),
                "{} overlaps {}",
                footprints[i].name,
                footprints[j].name
            );
        }
    }
}

fn rect(center: (f64, f64), x: f64, y: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - x / 2.0,
        center.0 + x / 2.0,
        center.1 - y / 2.0,
        center.1 + y / 2.0,
    )
}

fn grid_center(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn slot_center(idx: usize) -> (f64, f64) {
    let col = idx % SLOT_COLS;
    let row = idx / SLOT_COLS;
    (
        grid_center(col, SLOT_COLS, SLOT_PITCH_X),
        grid_center(row, SLOT_ROWS, SLOT_PITCH_Y),
    )
}

fn route_slot_x(slot: usize) -> f64 {
    grid_center(slot, SLOT_COUNT, ROUTE_CHANNEL_PITCH_X)
}

fn route_stage_y(stage: usize) -> f64 {
    grid_center(stage, ROUTE_STAGES, ROUTE_STAGE_PITCH_Y)
}

fn witness_center(idx: usize) -> (f64, f64) {
    let col = idx % WITNESS_COLS;
    let row = idx / WITNESS_COLS;
    (
        grid_center(col, WITNESS_COLS, WITNESS_PITCH_X),
        grid_center(row, WITNESS_ROWS, WITNESS_PITCH_Y) + 10.0,
    )
}

fn sensor_pair_x(idx: usize) -> f64 {
    grid_center(idx % 8, 8, SENSOR_PAIR_PITCH_X)
}

fn sensor_pair_y(idx: usize) -> f64 {
    if idx < 8 {
        -42.0
    } else {
        42.0
    }
}

fn shear_gauge_x(idx: usize) -> f64 {
    grid_center(idx % 4, 4, SHEAR_GAUGE_PITCH_X * 1.6)
}

fn shear_gauge_y(idx: usize) -> f64 {
    if idx < 4 {
        -40.0
    } else {
        40.0
    }
}

fn disposition_lane_y(lane: usize) -> f64 {
    match lane {
        RELEASE_LANE_INDEX => 10.0,
        HOLD_LANE_INDEX => 0.0,
        REJECT_LANE_INDEX => -10.0,
        _ => 0.0,
    }
}

fn fiducial_disc(name: String, diameter: f64, height: f64) -> Part {
    let outer = centered_cylinder(
        format!("{name}_outer"),
        diameter / 2.0,
        height,
        FIDUCIAL_SEGMENTS,
    );
    let inner = centered_cylinder(
        format!("{name}_center_cut"),
        diameter / 4.0,
        height + 1.0,
        FIDUCIAL_SEGMENTS,
    );
    let cross_x = centered_cube(
        format!("{name}_cross_x_cut"),
        diameter + 2.0,
        diameter / 6.0,
        height + 1.0,
    );
    let cross_y = centered_cube(
        format!("{name}_cross_y_cut"),
        diameter / 6.0,
        diameter + 2.0,
        height + 1.0,
    );
    outer - inner - cross_x - cross_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covers_exactly_sixteen_cassette_slots() {
        assert_eq!(SLOT_ROWS, 4);
        assert_eq!(SLOT_COLS, 4);
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(SLOT_ID_TABS, SLOT_COUNT);
        assert_eq!(PER_SLOT_WASHOUT_WINDOWS, SLOT_COUNT);
        assert_eq!(BARCODE_LANDS, SLOT_COUNT);
    }

    #[test]
    fn feed_flush_harvest_routes_cover_every_slot() {
        assert_eq!(ROUTE_STAGES, 3);
        assert_eq!(FEED_ROUTES + FLUSH_ROUTES + HARVEST_ROUTES, ROUTE_LEGS);
        assert_eq!(ROUTE_LEGS, SLOT_COUNT * ROUTE_STAGES);
        assert_eq!(PRESSURE_SENSOR_DOCKS, SLOT_COUNT);
        assert_eq!(FLOW_PULSE_SENSOR_DOCKS, SLOT_COUNT);
    }

    #[test]
    fn witness_count_covers_residual_carryover_and_controls() {
        assert_eq!(RESIDUAL_WELLS, SLOT_COUNT);
        assert_eq!(CARRYOVER_WELLS, SLOT_COUNT);
        assert_eq!(CONTROL_WELLS, 4);
        assert_eq!(TOTAL_WITNESS_WELLS, 36);
        assert!(WITNESS_ROWS * WITNESS_COLS >= TOTAL_WITNESS_WELLS);
    }

    #[test]
    fn disposition_and_quarantine_are_explicit() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE * DISPOSITION_LANES, 18);
        assert!(WASTE_QUARANTINE_BOTTLES >= ROUTE_STAGES * 2);
        assert!(WASTE_CAPTURE_VOLUME_ML >= 864.0);
    }

    #[test]
    fn all_modules_fit_without_overlap() {
        assert_layout();
    }

    #[test]
    fn output_manifest_is_source_only_and_complete() {
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_media_exchange_shear_pulse_carryover_validation_station_"
        )));
        assert!(!USES_RANDOMNESS);
        assert_eq!(RANDOM_SEED, 0);
        assert_eq!(
            PARAMETER_SET_REV,
            "media-exchange-shear-pulse-carryover-validation-rev-a"
        );
        assert_eq!(OUTPUT_MANIFEST_REV, "source-only-stl-manifest-rev-a");
    }
}
