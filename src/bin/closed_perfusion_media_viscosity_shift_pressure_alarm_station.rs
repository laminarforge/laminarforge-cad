use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion media viscosity-shift pressure/flow alarm station.
//
// Intent:
// - Exercise a closed media loop with high/low reference fluids so viscosity
//   shifts that bias pressure and flow alarm behavior are mechanically visible.
// - Keep reference-fluid custody, temperature equilibration, capillary
//   restrictions, pressure taps, reference flow sensors, flush capture,
//   disposition, traceability lands, camera evidence, and robot/service
//   keepouts in one bounded validation fixture.
// - Model fixture interfaces only. This file makes no biological, clinical,
//   release, diagnostic, or therapeutic claims.

const PREFIX: &str = "closed_perfusion_media_viscosity_shift_pressure_alarm_station";

const OUTPUTS: [&str; 15] = [
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_base_leak_tray_deck.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_high_low_viscosity_reference_fluid_nests.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_temperature_equilibration_block.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_capillary_restriction_coupon_rack.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_twenty_lane_pressure_tap_manifold.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_reference_flow_sensor_docks.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_bubble_wetness_windows.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_mixing_hold_time_token_rail.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_alarm_threshold_comparison_lanes.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_waste_flush_capture.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_barcode_coa_certificate_lands.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_evidence_camera_bridge.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_robot_service_keepouts.stl",
    "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 14] = [
    "base_leak_tray_deck",
    "high_low_viscosity_reference_fluid_nests",
    "temperature_equilibration_block",
    "capillary_restriction_coupon_rack",
    "twenty_lane_pressure_tap_manifold",
    "reference_flow_sensor_docks",
    "bubble_wetness_windows",
    "mixing_hold_time_token_rail",
    "alarm_threshold_comparison_lanes",
    "waste_flush_capture",
    "release_hold_reject_lanes",
    "barcode_coa_certificate_lands",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

#[cfg(test)]
const SCOPE_LIMITATIONS: [&str; 5] = [
    "fixture_geometry_only",
    "no_biological_claims",
    "no_clinical_claims",
    "no_release_acceptance_criteria",
    "no_patient_or_therapy_claims",
];

const STATION_X: f64 = 1450.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const BASIN_X: f64 = 1290.0;
const BASIN_Y: f64 = 750.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 16.0;
const DATUM_TARGETS: usize = 4;
const LEAK_RIBS: usize = 9;

const REFERENCE_POS: (f64, f64) = (-520.0, 250.0);
const REFERENCE_X: f64 = 290.0;
const REFERENCE_Y: f64 = 220.0;
const REFERENCE_Z: f64 = 48.0;
const LOW_VISCOSITY_NESTS: usize = 6;
const HIGH_VISCOSITY_NESTS: usize = 6;
const REFERENCE_FLUID_NESTS: usize = LOW_VISCOSITY_NESTS + HIGH_VISCOSITY_NESTS;
const REFERENCE_NEST_PITCH_X: f64 = 42.0;
const REFERENCE_NEST_PITCH_Y: f64 = 72.0;
const REFERENCE_WELL_D: f64 = 22.0;

const TEMP_POS: (f64, f64) = (-210.0, 250.0);
const TEMP_X: f64 = 270.0;
const TEMP_Y: f64 = 220.0;
const TEMP_Z: f64 = 58.0;
const TEMP_POCKETS: usize = REFERENCE_FLUID_NESTS;
const TEMP_POCKET_COLS: usize = 6;
const TEMP_PITCH_X: f64 = 36.0;
const TEMP_PITCH_Y: f64 = 58.0;
const TEMP_PROBE_CLIPS: usize = 4;

const CAPILLARY_POS: (f64, f64) = (140.0, 250.0);
const CAPILLARY_X: f64 = 380.0;
const CAPILLARY_Y: f64 = 220.0;
const CAPILLARY_Z: f64 = 44.0;
const CAPILLARY_COUPONS: usize = 10;
const CAPILLARY_PITCH_X: f64 = 34.0;
const CAPILLARY_SLOT_X: f64 = 24.0;
const CAPILLARY_SLOT_Y: f64 = 142.0;
const CAPILLARY_ID_LANDS: usize = CAPILLARY_COUPONS;

const FLOW_POS: (f64, f64) = (520.0, 250.0);
const FLOW_X: f64 = 280.0;
const FLOW_Y: f64 = 220.0;
const FLOW_Z: f64 = 52.0;
const REFERENCE_FLOW_SENSOR_DOCKS: usize = 4;
const FLOW_PITCH_Y: f64 = 44.0;
const FLOW_REFERENCE_TICKS: usize = 6;

const PRESSURE_POS: (f64, f64) = (-370.0, 15.0);
const PRESSURE_X: f64 = 600.0;
const PRESSURE_Y: f64 = 210.0;
const PRESSURE_Z: f64 = 58.0;
const PRESSURE_LANES: usize = 20;
const PRESSURE_TAP_COUNT: usize = PRESSURE_LANES;
const PRESSURE_PITCH_X: f64 = 28.0;
const PRESSURE_TAP_D: f64 = 6.0;
const COMMON_BORE_D: f64 = 8.0;

const WINDOW_POS: (f64, f64) = (120.0, 15.0);
const WINDOW_X: f64 = 340.0;
const WINDOW_Y: f64 = 210.0;
const WINDOW_Z: f64 = 34.0;
const BUBBLE_WINDOWS: usize = 10;
const WETNESS_WINDOWS: usize = 10;
const WINDOW_PITCH_X: f64 = 30.0;
const WINDOW_ROW_PITCH_Y: f64 = 64.0;

const TOKEN_POS: (f64, f64) = (500.0, 15.0);
const TOKEN_X: f64 = 320.0;
const TOKEN_Y: f64 = 210.0;
const TOKEN_Z: f64 = 36.0;
const MIXING_TOKEN_SLOTS: usize = 12;
const HOLD_TIME_TOKEN_SLOTS: usize = 6;
const TOKEN_PITCH_X: f64 = 24.0;

const ALARM_POS: (f64, f64) = (-400.0, -250.0);
const ALARM_X: f64 = 430.0;
const ALARM_Y: f64 = 190.0;
const ALARM_Z: f64 = 40.0;
const ALARM_THRESHOLD_LANES: usize = PRESSURE_LANES;
const ALARM_ROWS: usize = 2;
const ALARM_COLS: usize = 10;
const ALARM_LANE_PITCH_X: f64 = 38.0;
const ALARM_LANE_PITCH_Y: f64 = 58.0;
const THRESHOLD_STEPS_PER_LANE: usize = 3;

const WASTE_POS: (f64, f64) = (0.0, -250.0);
const WASTE_X: f64 = 300.0;
const WASTE_Y: f64 = 190.0;
const WASTE_Z: f64 = 52.0;
const FLUSH_PORTS: usize = PRESSURE_LANES;
const WASTE_CAPTURE_WELLS: usize = 4;
const FLUSH_PORT_PITCH_X: f64 = 26.0;

const DISPOSITION_POS: (f64, f64) = (320.0, -250.0);
const DISPOSITION_X: f64 = 300.0;
const DISPOSITION_Y: f64 = 190.0;
const DISPOSITION_Z: f64 = 38.0;
const DISPOSITION_LANES: usize = 3;
const TOKENS_PER_DISPOSITION: usize = 4;
const DISPOSITION_PITCH_X: f64 = 88.0;

const TRACE_POS: (f64, f64) = (590.0, -250.0);
const TRACE_X: f64 = 170.0;
const TRACE_Y: f64 = 190.0;
const TRACE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = PRESSURE_LANES;
const COA_LANDS: usize = 2;
const CERTIFICATE_LANDS: usize = 4;
const TRACE_ROW_PITCH_Y: f64 = 38.0;

const CAMERA_SPAN_X: f64 = 1180.0;
const CAMERA_POS_Y: f64 = -34.0;
const CAMERA_UNDERSIDE_Z: f64 = 214.0;
const CAMERA_BEAM_Z: f64 = 30.0;
const CAMERA_POST_X: f64 = 32.0;
const CAMERA_POST_Y: f64 = 46.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const EVIDENCE_LIGHT_RAILS: usize = 8;

const KEEP_OUT_X: f64 = 1330.0;
const KEEP_OUT_Y: f64 = 800.0;
const KEEP_OUT_Z: f64 = 150.0;
const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_FLUID_SERVICE_CLEARANCE: f64 = 190.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 170.0;
const TOP_CAMERA_CLEARANCE: f64 = 250.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.9;
const TUBE_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 18.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 18.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    export(OUTPUTS[0], &base_leak_tray_deck());
    export(OUTPUTS[1], &high_low_viscosity_reference_fluid_nests());
    export(OUTPUTS[2], &temperature_equilibration_block());
    export(OUTPUTS[3], &capillary_restriction_coupon_rack());
    export(OUTPUTS[4], &twenty_lane_pressure_tap_manifold());
    export(OUTPUTS[5], &reference_flow_sensor_docks());
    export(OUTPUTS[6], &bubble_wetness_windows());
    export(OUTPUTS[7], &mixing_hold_time_token_rail());
    export(OUTPUTS[8], &alarm_threshold_comparison_lanes());
    export(OUTPUTS[9], &waste_flush_capture());
    export(OUTPUTS[10], &release_hold_reject_lanes());
    export(OUTPUTS[11], &barcode_coa_certificate_lands());
    export(OUTPUTS[12], &evidence_camera_bridge());
    export(OUTPUTS[13], &robot_service_keepouts());
    export(OUTPUTS[14], &station_assembly());

    println!();
    println!("Closed perfusion media viscosity-shift pressure/flow alarm station:");
    println!(
        "  Footprint:        {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray with {DATUM_TARGETS} datum targets and {LEAK_RIBS} leak ribs"
    );
    println!(
        "  References:       {LOW_VISCOSITY_NESTS} low-viscosity nests, {HIGH_VISCOSITY_NESTS} high-viscosity nests, {TEMP_POCKETS} temperature pockets"
    );
    println!(
        "  Challenge path:   {CAPILLARY_COUPONS} capillary coupons, {PRESSURE_TAP_COUNT} pressure tap lanes, {REFERENCE_FLOW_SENSOR_DOCKS} reference flow sensor docks"
    );
    println!(
        "  Witnessing:       {BUBBLE_WINDOWS} bubble windows, {WETNESS_WINDOWS} wetness windows, {MIXING_TOKEN_SLOTS} mixing tokens, {HOLD_TIME_TOKEN_SLOTS} hold-time tokens"
    );
    println!(
        "  Alarm evidence:   {ALARM_THRESHOLD_LANES} comparison lanes, {FLUSH_PORTS} flush ports, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Service:          front {FRONT_ROBOT_CLEARANCE:.0}mm, rear {REAR_SERVICE_CLEARANCE:.0}mm, left {LEFT_FLUID_SERVICE_CLEARANCE:.0}mm, right {RIGHT_SENSOR_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray_deck()
        + high_low_viscosity_reference_fluid_nests().translate(
            REFERENCE_POS.0,
            REFERENCE_POS.1,
            insert_z(REFERENCE_Z),
        )
        + temperature_equilibration_block().translate(TEMP_POS.0, TEMP_POS.1, insert_z(TEMP_Z))
        + capillary_restriction_coupon_rack().translate(
            CAPILLARY_POS.0,
            CAPILLARY_POS.1,
            insert_z(CAPILLARY_Z),
        )
        + reference_flow_sensor_docks().translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_Z))
        + twenty_lane_pressure_tap_manifold().translate(
            PRESSURE_POS.0,
            PRESSURE_POS.1,
            insert_z(PRESSURE_Z),
        )
        + bubble_wetness_windows().translate(WINDOW_POS.0, WINDOW_POS.1, insert_z(WINDOW_Z))
        + mixing_hold_time_token_rail().translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_Z))
        + alarm_threshold_comparison_lanes().translate(ALARM_POS.0, ALARM_POS.1, insert_z(ALARM_Z))
        + waste_flush_capture().translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_Z))
        + release_hold_reject_lanes().translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            insert_z(DISPOSITION_Z),
        )
        + barcode_coa_certificate_lands().translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + evidence_camera_bridge()
        + robot_service_keepouts()
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_base_leak_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_base_recessed_washdown_basin_cut"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, -8.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_low_point_flush_drain_cut"),
        DRAIN_D / 2.0,
        62.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 112.0,
        -STATION_Y / 2.0 + 24.0,
        BASE_Z - 7.0,
    );

    deck - basin - drain - mounting_hole_cuts()
        + containment_rims()
        + module_floor_lands()
        + leak_witness_ribs()
        + datum_targets()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_leak_tray_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_leak_tray_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_leak_tray_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_leak_tray_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn mounting_hole_cuts() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_hole_cuts"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn module_floor_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_module_floor_lands"));
    for rect in layout_rects() {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_{}_floor_land", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z + 1.5);
    }
    lands
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_leak_witness_ribs"));
    for rib in 0..LEAK_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_leak_witness_rib_{rib}"),
                6.0,
                BASIN_Y - 80.0,
                5.0,
            )
            .translate(centered_index(rib, LEAK_RIBS, 135.0), -8.0, BASE_Z + 2.5);
    }
    ribs
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_robot_datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 86.0, -STATION_Y / 2.0 + 86.0),
        (STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 + 86.0),
        (-STATION_X / 2.0 + 86.0, STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 86.0, STATION_Y / 2.0 - 86.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("{PREFIX}_datum_boss_{i}"), 17.0, 6.0, 36).translate(
            *x,
            *y,
            BASE_Z + 3.0,
        );
        let bore = centered_cylinder(format!("{PREFIX}_datum_center_bore_{i}"), 3.2, 8.0, 20)
            .translate(*x, *y, BASE_Z + 3.0);
        targets = targets + (boss - bore);
    }
    targets
}

fn high_low_viscosity_reference_fluid_nests() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_reference_fluid_nest_body"),
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    )
    .translate(0.0, 0.0, REFERENCE_Z / 2.0);
    let spill_basin = centered_cube(
        format!("{PREFIX}_reference_fluid_spill_basin_cut"),
        REFERENCE_X - 34.0,
        REFERENCE_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, REFERENCE_Z - 4.0);

    body - spill_basin - reference_well_cuts()
        + reference_well_rims()
        + viscosity_lane_label_lands()
        + reference_custody_bridge()
}

fn reference_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_reference_fluid_well_cuts"));
    for index in 0..REFERENCE_FLUID_NESTS {
        let (x, y) = reference_nest_xy(index);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_reference_fluid_well_cut_{index}"),
                REFERENCE_WELL_D / 2.0,
                REFERENCE_Z + 8.0,
                36,
            )
            .translate(x, y, REFERENCE_Z / 2.0);
    }
    cuts
}

fn reference_well_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_reference_fluid_well_rims"));
    for index in 0..REFERENCE_FLUID_NESTS {
        let (x, y) = reference_nest_xy(index);
        let outer = centered_cylinder(
            format!("{PREFIX}_reference_fluid_well_rim_{index}"),
            REFERENCE_WELL_D / 2.0 + 3.0,
            5.0,
            36,
        )
        .translate(x, y, REFERENCE_Z + 2.5);
        let inner = centered_cylinder(
            format!("{PREFIX}_reference_fluid_well_rim_open_{index}"),
            REFERENCE_WELL_D / 2.0 + 0.6,
            5.4,
            36,
        )
        .translate(x, y, REFERENCE_Z + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn viscosity_lane_label_lands() -> Part {
    let low = centered_cube(
        format!("{PREFIX}_low_viscosity_reference_label_land"),
        REFERENCE_X - 50.0,
        14.0,
        4.0,
    )
    .translate(0.0, -REFERENCE_NEST_PITCH_Y / 2.0 - 28.0, REFERENCE_Z + 2.0);
    let high = centered_cube(
        format!("{PREFIX}_high_viscosity_reference_label_land"),
        REFERENCE_X - 50.0,
        14.0,
        4.0,
    )
    .translate(0.0, REFERENCE_NEST_PITCH_Y / 2.0 + 28.0, REFERENCE_Z + 2.0);
    low + high
}

fn reference_custody_bridge() -> Part {
    centered_cube(
        format!("{PREFIX}_reference_fluid_custody_bridge"),
        REFERENCE_X - 44.0,
        10.0,
        24.0,
    )
    .translate(0.0, REFERENCE_Y / 2.0 - 20.0, REFERENCE_Z + 12.0)
}

fn temperature_equilibration_block() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_temperature_equilibration_block_body"),
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    )
    .translate(0.0, 0.0, TEMP_Z / 2.0);
    let thermal_plate = centered_cube(
        format!("{PREFIX}_temperature_plate_socket_cut"),
        TEMP_X - 34.0,
        TEMP_Y - 40.0,
        10.0,
    )
    .translate(0.0, 0.0, 6.0);

    body - thermal_plate - temperature_pocket_cuts()
        + temperature_probe_lands()
        + temperature_equalization_fins()
}

fn temperature_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_temperature_reference_pocket_cuts"));
    for index in 0..TEMP_POCKETS {
        let (x, y) = temp_pocket_xy(index);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_temperature_reference_pocket_{index}"),
                10.5,
                TEMP_Z + 8.0,
                32,
            )
            .translate(x, y, TEMP_Z / 2.0 + 4.0);
    }
    cuts
}

fn temperature_probe_lands() -> Part {
    let mut clips = Part::empty(format!("{PREFIX}_temperature_probe_clip_lands"));
    for probe in 0..TEMP_PROBE_CLIPS {
        let x = centered_index(probe, TEMP_PROBE_CLIPS, 52.0);
        clips = clips
            + centered_cube(
                format!("{PREFIX}_temperature_probe_clip_{probe}"),
                36.0,
                12.0,
                10.0,
            )
            .translate(x, -TEMP_Y / 2.0 + 22.0, TEMP_Z + 5.0)
            - centered_cylinder(
                format!("{PREFIX}_temperature_probe_bore_{probe}"),
                2.2,
                40.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TEMP_Y / 2.0 + 22.0, TEMP_Z + 5.0);
    }
    clips
}

fn temperature_equalization_fins() -> Part {
    let mut fins = Part::empty(format!("{PREFIX}_temperature_equalization_fins"));
    for fin in 0..=TEMP_POCKET_COLS {
        fins = fins
            + centered_cube(
                format!("{PREFIX}_temperature_equalization_fin_{fin}"),
                4.0,
                TEMP_Y - 48.0,
                7.0,
            )
            .translate(
                centered_index(fin, TEMP_POCKET_COLS + 1, TEMP_PITCH_X),
                0.0,
                TEMP_Z + 3.5,
            );
    }
    fins
}

fn capillary_restriction_coupon_rack() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_capillary_restriction_coupon_rack_body"),
        CAPILLARY_X,
        CAPILLARY_Y,
        CAPILLARY_Z,
    )
    .translate(0.0, 0.0, CAPILLARY_Z / 2.0);
    let spill_channel = centered_cube(
        format!("{PREFIX}_capillary_rack_spill_channel_cut"),
        CAPILLARY_X - 40.0,
        18.0,
        12.0,
    )
    .translate(0.0, -CAPILLARY_Y / 2.0 + 26.0, CAPILLARY_Z - 4.0);

    body - spill_channel - capillary_slot_cuts() + capillary_clamp_bridges() + capillary_id_lands()
}

fn capillary_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_capillary_coupon_slot_cuts"));
    for coupon in 0..CAPILLARY_COUPONS {
        let x = capillary_x(coupon);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_capillary_coupon_slot_cut_{coupon}"),
                CAPILLARY_SLOT_X,
                CAPILLARY_SLOT_Y,
                CAPILLARY_Z + 8.0,
            )
            .translate(x, -4.0, CAPILLARY_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_capillary_coupon_inlet_bore_{coupon}"),
                TUBE_BORE_D / 2.0,
                36.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -CAPILLARY_Y / 2.0 + 24.0, CAPILLARY_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_capillary_coupon_outlet_bore_{coupon}"),
                TUBE_BORE_D / 2.0,
                36.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, CAPILLARY_Y / 2.0 - 24.0, CAPILLARY_Z / 2.0);
    }
    cuts
}

fn capillary_clamp_bridges() -> Part {
    let mut bridges = Part::empty(format!("{PREFIX}_capillary_coupon_clamp_bridges"));
    for coupon in 0..CAPILLARY_COUPONS {
        let x = capillary_x(coupon);
        bridges = bridges
            + centered_cube(
                format!("{PREFIX}_capillary_coupon_front_clamp_{coupon}"),
                CAPILLARY_SLOT_X + 10.0,
                10.0,
                14.0,
            )
            .translate(x, -CAPILLARY_SLOT_Y / 2.0 - 12.0, CAPILLARY_Z + 7.0)
            + centered_cube(
                format!("{PREFIX}_capillary_coupon_rear_clamp_{coupon}"),
                CAPILLARY_SLOT_X + 10.0,
                10.0,
                14.0,
            )
            .translate(x, CAPILLARY_SLOT_Y / 2.0 + 4.0, CAPILLARY_Z + 7.0);
    }
    bridges
}

fn capillary_id_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_capillary_coupon_id_lands"));
    for land in 0..CAPILLARY_ID_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_capillary_coupon_id_land_{land}"),
                26.0,
                12.0,
                4.0,
            )
            .translate(
                capillary_x(land),
                CAPILLARY_Y / 2.0 - 16.0,
                CAPILLARY_Z + 2.0,
            );
    }
    lands
}

fn twenty_lane_pressure_tap_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_twenty_lane_pressure_tap_manifold_body"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);
    let common = centered_cylinder(
        format!("{PREFIX}_pressure_manifold_common_bore_cut"),
        COMMON_BORE_D / 2.0,
        PRESSURE_X + 16.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);

    body - common - pressure_tap_cuts() + pressure_transducer_lands() + pressure_lane_number_lands()
}

fn pressure_tap_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_tap_cuts"));
    for lane in 0..PRESSURE_LANES {
        let x = pressure_lane_x(lane);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_pressure_lane_{lane}_vertical_tap_cut"),
                PRESSURE_TAP_D / 2.0,
                PRESSURE_Z + 8.0,
                24,
            )
            .translate(x, 26.0, PRESSURE_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_pressure_lane_{lane}_side_tube_bore_cut"),
                TUBE_BORE_D / 2.0,
                PRESSURE_Y + 12.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, PRESSURE_Z / 2.0);
    }
    cuts
}

fn pressure_transducer_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pressure_transducer_lands"));
    for lane in 0..PRESSURE_LANES {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_pressure_lane_{lane}_sensor_land"),
                20.0,
                18.0,
                6.0,
            )
            .translate(pressure_lane_x(lane), 52.0, PRESSURE_Z + 3.0);
    }
    lands
}

fn pressure_lane_number_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pressure_lane_number_lands"));
    for lane in 0..PRESSURE_LANES {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_pressure_lane_{lane}_number_land"),
                18.0,
                9.0,
                3.0,
            )
            .translate(
                pressure_lane_x(lane),
                -PRESSURE_Y / 2.0 + 18.0,
                PRESSURE_Z + 1.5,
            );
    }
    lands
}

fn reference_flow_sensor_docks() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_reference_flow_sensor_dock_body"),
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    )
    .translate(0.0, 0.0, FLOW_Z / 2.0);

    body - flow_sensor_socket_cuts() - flow_sensor_bores()
        + flow_sensor_latches()
        + flow_reference_tick_lands()
}

fn flow_sensor_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_reference_flow_sensor_socket_cuts"));
    for dock in 0..REFERENCE_FLOW_SENSOR_DOCKS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_reference_flow_sensor_socket_{dock}"),
                FLOW_X - 72.0,
                26.0,
                18.0,
            )
            .translate(0.0, flow_dock_y(dock), FLOW_Z - 8.0);
    }
    cuts
}

fn flow_sensor_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_reference_flow_sensor_bores"));
    for dock in 0..REFERENCE_FLOW_SENSOR_DOCKS {
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_reference_flow_sensor_dock_{dock}_bore"),
                TUBE_BORE_D / 2.0,
                FLOW_X + 12.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, flow_dock_y(dock), FLOW_Z / 2.0);
    }
    bores
}

fn flow_sensor_latches() -> Part {
    let mut latches = Part::empty(format!("{PREFIX}_reference_flow_sensor_latches"));
    for dock in 0..REFERENCE_FLOW_SENSOR_DOCKS {
        latches = latches
            + centered_cube(
                format!("{PREFIX}_reference_flow_sensor_dock_{dock}_front_latch"),
                42.0,
                8.0,
                16.0,
            )
            .translate(-82.0, flow_dock_y(dock) - 18.0, FLOW_Z + 8.0)
            + centered_cube(
                format!("{PREFIX}_reference_flow_sensor_dock_{dock}_rear_latch"),
                42.0,
                8.0,
                16.0,
            )
            .translate(82.0, flow_dock_y(dock) + 18.0, FLOW_Z + 8.0);
    }
    latches
}

fn flow_reference_tick_lands() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_flow_reference_tick_lands"));
    for tick in 0..FLOW_REFERENCE_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_flow_reference_tick_{tick}"),
                4.0,
                FLOW_Y - 48.0,
                4.0,
            )
            .translate(
                centered_index(tick, FLOW_REFERENCE_TICKS, 32.0),
                0.0,
                FLOW_Z + 2.0,
            );
    }
    ticks
}

fn bubble_wetness_windows() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_wetness_window_panel"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(0.0, 0.0, WINDOW_Z / 2.0);

    body - optical_window_cuts() + optical_window_frames() + wetness_probe_lands()
}

fn optical_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_wetness_window_cuts"));
    for window in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_bubble_window_cut_{window}"),
                22.0,
                44.0,
                WINDOW_Z + 4.0,
            )
            .translate(window_x(window), -WINDOW_ROW_PITCH_Y / 2.0, WINDOW_Z / 2.0);
    }
    for window in 0..WETNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_wetness_window_cut_{window}"),
                22.0,
                44.0,
                WINDOW_Z + 4.0,
            )
            .translate(window_x(window), WINDOW_ROW_PITCH_Y / 2.0, WINDOW_Z / 2.0);
    }
    cuts
}

fn optical_window_frames() -> Part {
    let mut frames = Part::empty(format!("{PREFIX}_bubble_wetness_window_frames"));
    for window in 0..BUBBLE_WINDOWS {
        frames = frames
            + centered_cube(
                format!("{PREFIX}_bubble_window_frame_{window}"),
                30.0,
                52.0,
                5.0,
            )
            .translate(window_x(window), -WINDOW_ROW_PITCH_Y / 2.0, WINDOW_Z + 2.5);
    }
    for window in 0..WETNESS_WINDOWS {
        frames = frames
            + centered_cube(
                format!("{PREFIX}_wetness_window_frame_{window}"),
                30.0,
                52.0,
                5.0,
            )
            .translate(window_x(window), WINDOW_ROW_PITCH_Y / 2.0, WINDOW_Z + 2.5);
    }
    frames
}

fn wetness_probe_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_wetness_probe_lands"));
    for window in 0..WETNESS_WINDOWS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_wetness_probe_land_{window}"),
                20.0,
                8.0,
                5.0,
            )
            .translate(
                window_x(window),
                WINDOW_ROW_PITCH_Y / 2.0 + 34.0,
                WINDOW_Z + 2.5,
            );
    }
    lands
}

fn mixing_hold_time_token_rail() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_mixing_hold_time_token_rail_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0);

    body - token_slot_cuts() + token_lane_ribs() + hold_time_flag_posts()
}

fn token_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_mixing_hold_time_token_slot_cuts"));
    for slot in 0..MIXING_TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_mixing_token_slot_{slot}"),
                18.0,
                44.0,
                TOKEN_Z + 4.0,
            )
            .translate(
                centered_index(slot, MIXING_TOKEN_SLOTS, TOKEN_PITCH_X),
                -44.0,
                TOKEN_Z / 2.0,
            );
    }
    for slot in 0..HOLD_TIME_TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_hold_time_token_slot_{slot}"),
                28.0,
                44.0,
                TOKEN_Z + 4.0,
            )
            .translate(
                centered_index(slot, HOLD_TIME_TOKEN_SLOTS, 42.0),
                42.0,
                TOKEN_Z / 2.0,
            );
    }
    cuts
}

fn token_lane_ribs() -> Part {
    let mixing = centered_cube(
        format!("{PREFIX}_mixing_token_lane_label_land"),
        TOKEN_X - 40.0,
        10.0,
        4.0,
    )
    .translate(0.0, -86.0, TOKEN_Z + 2.0);
    let hold = centered_cube(
        format!("{PREFIX}_hold_time_token_lane_label_land"),
        TOKEN_X - 72.0,
        10.0,
        4.0,
    )
    .translate(0.0, 86.0, TOKEN_Z + 2.0);
    mixing + hold
}

fn hold_time_flag_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_hold_time_flag_posts"));
    for post in 0..HOLD_TIME_TOKEN_SLOTS {
        posts = posts
            + centered_cylinder(
                format!("{PREFIX}_hold_time_flag_post_{post}"),
                4.0,
                18.0,
                20,
            )
            .translate(
                centered_index(post, HOLD_TIME_TOKEN_SLOTS, 42.0),
                80.0,
                TOKEN_Z + 9.0,
            );
    }
    posts
}

fn alarm_threshold_comparison_lanes() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_alarm_threshold_comparison_lane_panel"),
        ALARM_X,
        ALARM_Y,
        ALARM_Z,
    )
    .translate(0.0, 0.0, ALARM_Z / 2.0);

    body - alarm_lane_slot_cuts() + threshold_step_lands() + alarm_state_flag_lands()
}

fn alarm_lane_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_alarm_threshold_lane_slot_cuts"));
    for lane in 0..ALARM_THRESHOLD_LANES {
        let (x, y) = alarm_lane_xy(lane);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_alarm_threshold_lane_{lane}_slot_cut"),
                26.0,
                38.0,
                ALARM_Z + 4.0,
            )
            .translate(x, y, ALARM_Z / 2.0);
    }
    cuts
}

fn threshold_step_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_threshold_step_lands"));
    for lane in 0..ALARM_THRESHOLD_LANES {
        let (x, y) = alarm_lane_xy(lane);
        for step in 0..THRESHOLD_STEPS_PER_LANE {
            lands = lands
                + centered_cube(
                    format!("{PREFIX}_alarm_lane_{lane}_threshold_step_{step}"),
                    6.0 + step as f64 * 4.0,
                    5.0,
                    4.0,
                )
                .translate(x, y - 26.0 + step as f64 * 8.0, ALARM_Z + 2.0);
        }
    }
    lands
}

fn alarm_state_flag_lands() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_alarm_state_flag_lands"));
    for row in 0..ALARM_ROWS {
        flags = flags
            + centered_cube(
                format!("{PREFIX}_alarm_row_{row}_state_flag_land"),
                ALARM_X - 52.0,
                8.0,
                4.0,
            )
            .translate(
                0.0,
                centered_index(row, ALARM_ROWS, ALARM_LANE_PITCH_Y) + 30.0,
                ALARM_Z + 2.0,
            );
    }
    flags
}

fn waste_flush_capture() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_waste_flush_capture_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0);
    let sump = centered_cube(
        format!("{PREFIX}_waste_flush_capture_sump_cut"),
        WASTE_X - 46.0,
        WASTE_Y - 54.0,
        16.0,
    )
    .translate(0.0, 0.0, WASTE_Z - 8.0);

    body - sump - flush_port_cuts() - waste_capture_well_cuts()
        + flush_route_comb()
        + waste_bag_retainer_lands()
}

fn flush_port_cuts() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_flush_port_cuts"));
    for port in 0..FLUSH_PORTS {
        ports = ports
            + centered_cylinder(
                format!("{PREFIX}_flush_port_{port}_cut"),
                TUBE_BORE_D / 2.0,
                34.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(port, FLUSH_PORTS, FLUSH_PORT_PITCH_X),
                -WASTE_Y / 2.0 + 18.0,
                WASTE_Z / 2.0,
            );
    }
    ports
}

fn waste_capture_well_cuts() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_waste_capture_well_cuts"));
    for well in 0..WASTE_CAPTURE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_waste_capture_well_{well}_cut"),
                24.0,
                WASTE_Z + 6.0,
                32,
            )
            .translate(
                centered_index(well, WASTE_CAPTURE_WELLS, 62.0),
                36.0,
                WASTE_Z / 2.0,
            );
    }
    wells
}

fn flush_route_comb() -> Part {
    let mut comb = Part::empty(format!("{PREFIX}_flush_route_comb"));
    for port in 0..FLUSH_PORTS {
        comb = comb
            + centered_cube(
                format!("{PREFIX}_flush_route_clip_{port}"),
                18.0,
                16.0,
                12.0,
            )
            .translate(
                centered_index(port, FLUSH_PORTS, FLUSH_PORT_PITCH_X),
                -WASTE_Y / 2.0 - 12.0,
                WASTE_Z + 6.0,
            );
    }
    comb
}

fn waste_bag_retainer_lands() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_waste_bag_left_retainer_land"),
        96.0,
        18.0,
        10.0,
    )
    .translate(-70.0, WASTE_Y / 2.0 - 24.0, WASTE_Z + 5.0);
    let right = centered_cube(
        format!("{PREFIX}_flush_bag_right_retainer_land"),
        96.0,
        18.0,
        10.0,
    )
    .translate(70.0, WASTE_Y / 2.0 - 24.0, WASTE_Z + 5.0);
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_panel"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(0.0, 0.0, DISPOSITION_Z / 2.0);

    body - disposition_token_cuts() + disposition_lane_ribs() + disposition_gate_posts()
}

fn disposition_token_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_release_hold_reject_token_cuts"));
    for lane in 0..DISPOSITION_LANES {
        for token in 0..TOKENS_PER_DISPOSITION {
            cuts = cuts
                + centered_cube(
                    format!(
                        "{PREFIX}_{}_token_slot_{token}",
                        disposition_lane_name(lane)
                    ),
                    42.0,
                    24.0,
                    DISPOSITION_Z + 4.0,
                )
                .translate(
                    disposition_lane_x(lane),
                    centered_index(token, TOKENS_PER_DISPOSITION, 36.0),
                    DISPOSITION_Z / 2.0,
                );
        }
    }
    cuts
}

fn disposition_lane_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_disposition_lane_ribs"));
    for lane in 0..DISPOSITION_LANES {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_{}_lane_label_land", disposition_lane_name(lane)),
                68.0,
                12.0,
                4.0,
            )
            .translate(
                disposition_lane_x(lane),
                -DISPOSITION_Y / 2.0 + 18.0,
                DISPOSITION_Z + 2.0,
            );
    }
    ribs
}

fn disposition_gate_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_disposition_gate_posts"));
    for lane in 0..DISPOSITION_LANES {
        posts = posts
            + centered_cylinder(
                format!("{PREFIX}_{}_gate_post", disposition_lane_name(lane)),
                5.0,
                22.0,
                20,
            )
            .translate(
                disposition_lane_x(lane),
                DISPOSITION_Y / 2.0 - 22.0,
                DISPOSITION_Z + 11.0,
            );
    }
    posts
}

fn barcode_coa_certificate_lands() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_barcode_coa_certificate_trace_panel"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);

    body + barcode_lands() + coa_lands() + certificate_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for land in 0..BARCODE_LANDS {
        let col = land % 5;
        let row = land / 5;
        lands = lands
            + centered_cube(format!("{PREFIX}_barcode_land_{land}"), 26.0, 10.0, 3.0).translate(
                centered_index(col, 5, 31.0),
                54.0 - row as f64 * 22.0,
                TRACE_Z + 1.5,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_coa_lands"));
    for land in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_coa_land_{land}"),
                TRACE_X - 30.0,
                14.0,
                3.0,
            )
            .translate(0.0, -42.0 - land as f64 * TRACE_ROW_PITCH_Y, TRACE_Z + 1.5);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_certificate_lands"));
    for land in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_certificate_land_{land}"), 62.0, 16.0, 3.0)
                .translate(
                    centered_index(land, CERTIFICATE_LANDS, 36.0),
                    TRACE_Y / 2.0 - 22.0,
                    TRACE_Z + 1.5,
                );
    }
    lands
}

fn evidence_camera_bridge() -> Part {
    let post_z = BASE_Z + CAMERA_UNDERSIDE_Z / 2.0;
    let beam_z = BASE_Z + CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0;
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_camera_bridge_left_post"),
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_UNDERSIDE_Z,
    )
    .translate(-CAMERA_SPAN_X / 2.0, CAMERA_POS_Y, post_z);
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_camera_bridge_right_post"),
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_UNDERSIDE_Z,
    )
    .translate(CAMERA_SPAN_X / 2.0, CAMERA_POS_Y, post_z);
    let beam = centered_cube(
        format!("{PREFIX}_evidence_camera_bridge_beam"),
        CAMERA_SPAN_X + CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, CAMERA_POS_Y, beam_z);
    left_post + right_post + beam + evidence_camera_mounts() + evidence_light_rails()
}

fn evidence_camera_mounts() -> Part {
    let mut mounts = Part::empty(format!("{PREFIX}_evidence_camera_mounts"));
    for camera in 0..EVIDENCE_CAMERA_COUNT {
        mounts = mounts
            + centered_cube(
                format!("{PREFIX}_evidence_camera_mount_{camera}"),
                58.0,
                18.0,
                24.0,
            )
            .translate(
                centered_index(camera, EVIDENCE_CAMERA_COUNT, 310.0),
                CAMERA_POS_Y - 34.0,
                BASE_Z + CAMERA_UNDERSIDE_Z - 12.0,
            );
    }
    mounts
}

fn evidence_light_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_evidence_light_rails"));
    for rail in 0..EVIDENCE_LIGHT_RAILS {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_evidence_light_rail_{rail}"),
                92.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(rail, EVIDENCE_LIGHT_RAILS, 135.0),
                CAMERA_POS_Y + 34.0,
                BASE_Z + CAMERA_UNDERSIDE_Z - 32.0,
            );
    }
    rails
}

fn robot_service_keepouts() -> Part {
    let footprint = centered_cube(
        format!("{PREFIX}_robot_sweep_outer_footprint_gauge"),
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z + KEEP_OUT_Z / 2.0);
    let front = centered_cube(
        format!("{PREFIX}_front_robot_access_keepout"),
        KEEP_OUT_X - 120.0,
        10.0,
        32.0,
    )
    .translate(0.0, -FRONT_ROBOT_CLEARANCE, BASE_Z + 16.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_service_access_keepout"),
        KEEP_OUT_X - 160.0,
        10.0,
        30.0,
    )
    .translate(0.0, REAR_SERVICE_CLEARANCE, BASE_Z + 15.0);
    let left = centered_cube(
        format!("{PREFIX}_left_fluidic_service_keepout"),
        10.0,
        KEEP_OUT_Y - 140.0,
        30.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_FLUID_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + 15.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_sensor_service_keepout"),
        10.0,
        KEEP_OUT_Y - 140.0,
        30.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SENSOR_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + 15.0,
    );
    let top = centered_cube(
        format!("{PREFIX}_camera_bridge_top_clearance_gauge"),
        CAMERA_SPAN_X - 100.0,
        12.0,
        18.0,
    )
    .translate(0.0, CAMERA_POS_Y, BASE_Z + TOP_CAMERA_CLEARANCE);
    footprint + front + rear + left + right + top
}

fn layout_rects() -> [Rect; 12] {
    [
        Rect {
            name: "high_low_viscosity_reference_fluid_nests",
            center: REFERENCE_POS,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Rect {
            name: "temperature_equilibration_block",
            center: TEMP_POS,
            x: TEMP_X,
            y: TEMP_Y,
        },
        Rect {
            name: "capillary_restriction_coupon_rack",
            center: CAPILLARY_POS,
            x: CAPILLARY_X,
            y: CAPILLARY_Y,
        },
        Rect {
            name: "reference_flow_sensor_docks",
            center: FLOW_POS,
            x: FLOW_X,
            y: FLOW_Y,
        },
        Rect {
            name: "twenty_lane_pressure_tap_manifold",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "bubble_wetness_windows",
            center: WINDOW_POS,
            x: WINDOW_X,
            y: WINDOW_Y,
        },
        Rect {
            name: "mixing_hold_time_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Rect {
            name: "alarm_threshold_comparison_lanes",
            center: ALARM_POS,
            x: ALARM_X,
            y: ALARM_Y,
        },
        Rect {
            name: "waste_flush_capture",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Rect {
            name: "barcode_coa_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "evidence_camera_bridge",
            center: (0.0, CAMERA_POS_Y),
            x: CAMERA_SPAN_X + CAMERA_POST_X,
            y: CAMERA_POST_Y,
        },
    ]
}

fn mount_points() -> [(f64, f64); 6] {
    [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 68.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 68.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 68.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 68.0),
        (0.0, -STATION_Y / 2.0 + 68.0),
        (0.0, STATION_Y / 2.0 - 68.0),
    ]
}

fn reference_nest_xy(index: usize) -> (f64, f64) {
    let lane = index / LOW_VISCOSITY_NESTS;
    let col = index % LOW_VISCOSITY_NESTS;
    (
        centered_index(col, LOW_VISCOSITY_NESTS, REFERENCE_NEST_PITCH_X),
        centered_index(lane, 2, REFERENCE_NEST_PITCH_Y),
    )
}

fn temp_pocket_xy(index: usize) -> (f64, f64) {
    let col = index % TEMP_POCKET_COLS;
    let row = index / TEMP_POCKET_COLS;
    (
        centered_index(col, TEMP_POCKET_COLS, TEMP_PITCH_X),
        centered_index(row, 2, TEMP_PITCH_Y),
    )
}

fn capillary_x(index: usize) -> f64 {
    centered_index(index, CAPILLARY_COUPONS, CAPILLARY_PITCH_X)
}

fn pressure_lane_x(lane: usize) -> f64 {
    centered_index(lane, PRESSURE_LANES, PRESSURE_PITCH_X)
}

fn flow_dock_y(dock: usize) -> f64 {
    centered_index(dock, REFERENCE_FLOW_SENSOR_DOCKS, FLOW_PITCH_Y)
}

fn window_x(window: usize) -> f64 {
    centered_index(window, BUBBLE_WINDOWS, WINDOW_PITCH_X)
}

fn alarm_lane_xy(lane: usize) -> (f64, f64) {
    let col = lane % ALARM_COLS;
    let row = lane / ALARM_COLS;
    (
        centered_index(col, ALARM_COLS, ALARM_LANE_PITCH_X),
        centered_index(row, ALARM_ROWS, ALARM_LANE_PITCH_Y),
    )
}

fn disposition_lane_x(lane: usize) -> f64 {
    centered_index(lane, DISPOSITION_LANES, DISPOSITION_PITCH_X)
}

fn disposition_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn insert_z(module_z: f64) -> f64 {
    BASE_Z + module_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(REFERENCE_FLUID_NESTS, 12);
    assert_eq!(TEMP_POCKETS, REFERENCE_FLUID_NESTS);
    assert_eq!(PRESSURE_LANES, 20);
    assert_eq!(PRESSURE_TAP_COUNT, PRESSURE_LANES);
    assert_eq!(ALARM_THRESHOLD_LANES, PRESSURE_LANES);
    assert_eq!(ALARM_THRESHOLD_LANES, ALARM_ROWS * ALARM_COLS);
    assert_eq!(FLUSH_PORTS, PRESSURE_LANES);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(BASIN_X < STATION_X - RIM_W * 2.0);
    assert!(BASIN_Y < STATION_Y - RIM_W * 2.0);
    assert!(TUBE_BORE_D > TUBE_OD);
    assert!(CAMERA_UNDERSIDE_Z > FLOW_Z + BASE_Z + 110.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} outside usable station deck",
            rect.name
        );
    }
    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            if left.name == "evidence_camera_bridge" || right.name == "evidence_camera_bridge" {
                continue;
            }
            assert!(
                !left.overlaps_with(*right, 8.0),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn reference_fluid_capillary_and_sensor_counts_are_explicit() {
        assert_eq!(LOW_VISCOSITY_NESTS, 6);
        assert_eq!(HIGH_VISCOSITY_NESTS, 6);
        assert_eq!(REFERENCE_FLUID_NESTS, 12);
        assert_eq!(TEMP_POCKETS, REFERENCE_FLUID_NESTS);
        assert_eq!(CAPILLARY_COUPONS, 10);
        assert_eq!(CAPILLARY_ID_LANDS, CAPILLARY_COUPONS);
        assert_eq!(PRESSURE_LANES, 20);
        assert_eq!(PRESSURE_TAP_COUNT, 20);
        assert_eq!(REFERENCE_FLOW_SENSOR_DOCKS, 4);
        assert_eq!(ALARM_THRESHOLD_LANES, PRESSURE_TAP_COUNT);
        assert_eq!(FLUSH_PORTS, PRESSURE_TAP_COUNT);
    }

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 15);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_perfusion_media_viscosity_shift_pressure_alarm_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[0].ends_with("_base_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn station_modules_fit_bounds_without_collisions() {
        assert_layout();
        for rect in layout_rects() {
            assert!(
                rect.fits_inside_station(),
                "{} outside usable station deck",
                rect.name
            );
        }
        assert!(KEEP_OUT_X < STATION_X);
        assert!(KEEP_OUT_Y < STATION_Y);
        assert!(CAMERA_SPAN_X + CAMERA_POST_X < STATION_X);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
        assert!(REAR_SERVICE_CLEARANCE >= 250.0);
        assert!(LEFT_FLUID_SERVICE_CLEARANCE >= 180.0);
        assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 160.0);
    }

    #[test]
    fn required_feature_coverage_matches_requested_station_scope() {
        let features: BTreeSet<&str> = REQUIRED_FEATURES.iter().copied().collect();
        assert_eq!(features.len(), REQUIRED_FEATURES.len());
        for feature in [
            "base_leak_tray_deck",
            "high_low_viscosity_reference_fluid_nests",
            "temperature_equilibration_block",
            "capillary_restriction_coupon_rack",
            "twenty_lane_pressure_tap_manifold",
            "reference_flow_sensor_docks",
            "bubble_wetness_windows",
            "mixing_hold_time_token_rail",
            "alarm_threshold_comparison_lanes",
            "waste_flush_capture",
            "release_hold_reject_lanes",
            "barcode_coa_certificate_lands",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(features.contains(feature), "missing feature {feature}");
            assert!(
                OUTPUTS.iter().any(|output| output.contains(feature)),
                "missing STL output for {feature}"
            );
        }
    }

    #[test]
    fn no_biological_or_clinical_claims_are_encoded() {
        assert!(SCOPE_LIMITATIONS.contains(&"fixture_geometry_only"));
        assert!(SCOPE_LIMITATIONS.contains(&"no_biological_claims"));
        assert!(SCOPE_LIMITATIONS.contains(&"no_clinical_claims"));
        assert!(SCOPE_LIMITATIONS.contains(&"no_release_acceptance_criteria"));
        assert!(SCOPE_LIMITATIONS.contains(&"no_patient_or_therapy_claims"));

        let prohibited_claim_terms = [
            "diagnostic",
            "therapeutic",
            "treatment",
            "patient",
            "clinical_release",
            "cure",
        ];
        for output in OUTPUTS {
            for term in prohibited_claim_terms {
                assert!(
                    !output.contains(term),
                    "output path contains prohibited claim term {term}"
                );
            }
        }
    }
}
