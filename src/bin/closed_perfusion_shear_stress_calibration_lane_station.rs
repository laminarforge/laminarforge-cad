use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion shear-stress calibration lane station.
//
// Intent:
// - Validate no-cell chip-lane flow/shear reproducibility before tissue-chip
//   runs using calibrated restriction ladders and paired witness lanes.
// - Put upstream/mid/downstream pressure taps next to each lane so pressure
//   drop, restriction state, and differential witness flow can be cross-checked
//   without opening the fluid path.
// - Provide bubble-safe prime/purge references with uphill trap chimneys,
//   purge headers, and optical references so bubbles are managed upstream of
//   cell-facing shear lanes.
//
// This is validation/interface CAD. It is not a biological protocol, sterile
// barrier claim, calibrated pressure device, or wetted-material release spec.

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_shear_stress_calibration_lane_station_base_containment_tray.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_chip_lane_nest.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_calibrated_restriction_ladders.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_pressure_tap_manifold.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_differential_flow_witness_lanes.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_shear_reference_channel_coupons.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_bubble_safe_prime_purge_references.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_viscosity_temperature_reference_pockets.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_tracer_recovery_waste_capture.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_barcode_certificate_status_lanes.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_robot_service_keepouts.stl",
    "output/closed_perfusion_shear_stress_calibration_lane_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "base_containment_tray",
    "chip_lane_nest",
    "calibrated_restriction_ladders",
    "pressure_tap_manifold",
    "differential_flow_witness_lanes",
    "shear_reference_channel_coupons",
    "bubble_safe_prime_purge_references",
    "viscosity_temperature_reference_pockets",
    "tracer_recovery_waste_capture",
    "barcode_certificate_status_lanes",
    "robot_service_keepouts",
    "assembly",
];

const PREFIX: &str = "closed_perfusion_shear_stress_calibration_lane_station";

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const SUMP_X: f64 = 1120.0;
const SUMP_Y: f64 = 640.0;
const SUMP_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_COUNT: usize = 4;

const LANE_COUNT: usize = 6;
const LANE_ROWS: usize = 2;
const LANE_COLS: usize = 3;
const CHIP_NEST_POS: (f64, f64) = (-330.0, 190.0);
const CHIP_NEST_X: f64 = 500.0;
const CHIP_NEST_Y: f64 = 245.0;
const CHIP_NEST_Z: f64 = 42.0;
const CHIP_SOCKET_X: f64 = REVC_CHIP_LENGTH + 18.0;
const CHIP_SOCKET_Y: f64 = REVC_CHIP_WIDTH + 16.0;
const CHIP_PITCH_X: f64 = 142.0;
const CHIP_PITCH_Y: f64 = 92.0;
const CHIP_POCKET_DEPTH: f64 = 12.0;
const GASKET_LAND_W: f64 = 8.0;

const LADDER_POS: (f64, f64) = (205.0, 190.0);
const LADDER_X: f64 = 512.0;
const LADDER_Y: f64 = 245.0;
const LADDER_Z: f64 = 46.0;
const LADDER_STEPS: usize = 5;
const LADDER_STEP_PITCH_X: f64 = 34.0;
const LADDER_LANE_PITCH_Y: f64 = 34.0;
const RESTRICTION_BORE_D: f64 = 5.6;
const RESTRICTION_TOKEN_Z: f64 = 5.0;

const PRESSURE_POS: (f64, f64) = (-365.0, -44.0);
const PRESSURE_X: f64 = 440.0;
const PRESSURE_Y: f64 = 188.0;
const PRESSURE_Z: f64 = 56.0;
const PRESSURE_TAPS_PER_LANE: usize = 3;
const PRESSURE_TAP_D: f64 = 8.0;
const PRESSURE_LANE_PITCH_X: f64 = 64.0;
const PRESSURE_STAGE_PITCH_Y: f64 = 46.0;
const PRESSURE_COMMON_BORE_D: f64 = 7.0;

const WITNESS_POS: (f64, f64) = (135.0, -50.0);
const WITNESS_X: f64 = 530.0;
const WITNESS_Y: f64 = 198.0;
const WITNESS_Z: f64 = 36.0;
const WITNESS_PAIRS: usize = LANE_COUNT;
const WITNESS_CHANNELS_PER_PAIR: usize = 2;
const WITNESS_WINDOW_X: f64 = 44.0;
const WITNESS_WINDOW_Y: f64 = 10.0;
const WITNESS_CHANNEL_W: f64 = 4.8;
const WITNESS_TICK_COUNT: usize = 7;

const SHEAR_POS: (f64, f64) = (468.0, -52.0);
const SHEAR_X: f64 = 214.0;
const SHEAR_Y: f64 = 198.0;
const SHEAR_Z: f64 = 34.0;
const SHEAR_COUPONS: usize = 4;
const SHEAR_CHANNELS_PER_COUPON: usize = 3;
const SHEAR_CHANNEL_H: f64 = 1.2;
const SHEAR_CHANNEL_W: f64 = 9.0;
const SHEAR_COUPON_PITCH_Y: f64 = 43.0;

const PRIME_POS: (f64, f64) = (-345.0, -258.0);
const PRIME_X: f64 = 446.0;
const PRIME_Y: f64 = 142.0;
const PRIME_Z: f64 = 74.0;
const PRIME_REFERENCES: usize = LANE_COUNT;
const PRIME_REF_PITCH_X: f64 = 62.0;
const PRIME_TRAP_D: f64 = 28.0;
const PRIME_CHIMNEY_Z: f64 = 58.0;
const PURGE_HEADER_D: f64 = 8.0;
const PURGE_PORT_D: f64 = 10.0;

const REFERENCE_POS: (f64, f64) = (78.0, -258.0);
const REFERENCE_X: f64 = 260.0;
const REFERENCE_Y: f64 = 142.0;
const REFERENCE_Z: f64 = 34.0;
const VISCOSITY_WELLS: usize = 4;
const TEMPERATURE_POCKET_COUNT: usize = 3;
const REFERENCE_WELL_D: f64 = 32.0;

const WASTE_POS: (f64, f64) = (362.0, -258.0);
const WASTE_X: f64 = 270.0;
const WASTE_Y: f64 = 142.0;
const WASTE_Z: f64 = 58.0;
const WASTE_WELLS: usize = 6;
const WASTE_WELL_D: f64 = 34.0;
const TRACER_GRADIENT_STEPS: usize = 6;

const STATUS_POS: (f64, f64) = (448.0, 210.0);
const STATUS_X: f64 = 262.0;
const STATUS_Y: f64 = 198.0;
const STATUS_Z: f64 = 18.0;
const BARCODE_LANDS: usize = LANE_COUNT;
const CERTIFICATE_LANDS: usize = 3;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;

const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_TUBING_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_PRESSURE_SERVICE_CLEARANCE: f64 = 180.0;
const RIGHT_WASTE_SERVICE_CLEARANCE: f64 = 190.0;
const TOP_CASSETTE_LIFT_CLEARANCE: f64 = 185.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_tray(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 18.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 18.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let nest = chip_lane_nest();
    export(OUTPUTS[1], &nest);

    let ladders = calibrated_restriction_ladders();
    export(OUTPUTS[2], &ladders);

    let pressure = pressure_tap_manifold();
    export(OUTPUTS[3], &pressure);

    let witnesses = differential_flow_witness_lanes();
    export(OUTPUTS[4], &witnesses);

    let shear = shear_reference_channel_coupons();
    export(OUTPUTS[5], &shear);

    let prime = bubble_safe_prime_purge_references();
    export(OUTPUTS[6], &prime);

    let reference = viscosity_temperature_reference_pockets();
    export(OUTPUTS[7], &reference);

    let waste = tracer_recovery_waste_capture();
    export(OUTPUTS[8], &waste);

    let status = barcode_certificate_status_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed perfusion shear-stress calibration lane station:");
    println!(
        "  Footprint:       {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray with {DATUM_COUNT} datum targets"
    );
    println!(
        "  Chip lanes:      {LANE_COUNT} no-cell calibration lane sockets in a {LANE_ROWS}x{LANE_COLS} grid"
    );
    println!(
        "  Restrictions:    {LANE_COUNT} calibrated ladders with {LADDER_STEPS} restriction steps per lane"
    );
    println!(
        "  Pressure taps:   {} upstream/mid/downstream tap positions tied to a common manifold",
        LANE_COUNT * PRESSURE_TAPS_PER_LANE
    );
    println!(
        "  Flow witnesses:  {WITNESS_PAIRS} differential witness pairs, {WITNESS_TICK_COUNT} tick marks per channel, and {SHEAR_COUPONS} shear-reference coupons"
    );
    println!(
        "  Bubble controls: {PRIME_REFERENCES} bubble-safe prime/purge references with uphill trap chimneys and purge header"
    );
    println!(
        "  Evidence:        {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {STATUS_LANES} release/hold/reject lanes, and {WASTE_WELLS} tracer recovery wells"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_containment_tray()
        + chip_lane_nest().translate(CHIP_NEST_POS.0, CHIP_NEST_POS.1, insert_z(CHIP_NEST_Z))
        + calibrated_restriction_ladders().translate(LADDER_POS.0, LADDER_POS.1, insert_z(LADDER_Z))
        + pressure_tap_manifold().translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_Z))
        + differential_flow_witness_lanes().translate(
            WITNESS_POS.0,
            WITNESS_POS.1,
            insert_z(WITNESS_Z),
        )
        + shear_reference_channel_coupons().translate(SHEAR_POS.0, SHEAR_POS.1, insert_z(SHEAR_Z))
        + bubble_safe_prime_purge_references().translate(
            PRIME_POS.0,
            PRIME_POS.1,
            insert_z(PRIME_Z),
        )
        + viscosity_temperature_reference_pockets().translate(
            REFERENCE_POS.0,
            REFERENCE_POS.1,
            insert_z(REFERENCE_Z),
        )
        + tracer_recovery_waste_capture().translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_Z))
        + barcode_certificate_status_lanes().translate(
            STATUS_POS.0,
            STATUS_POS.1,
            insert_z(STATUS_Z),
        )
        + robot_service_keepouts()
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_base_containment_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        format!("{PREFIX}_base_containment_tray_recessed_sump_cut"),
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -6.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{PREFIX}_base_containment_tray_front_drain_cut"),
        DRAIN_D / 2.0,
        RIM_W + 42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 12.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain - mounting_holes()
        + containment_rims()
        + floor_footprints()
        + datum_targets()
        + leak_witness_ribs()
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

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_base_mounting_holes"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_base_mounting_hole_cut_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn floor_footprints() -> Part {
    let mut markers = Part::empty(format!("{PREFIX}_module_floor_footprints"));
    for rect in module_rects() {
        markers = markers
            + centered_cube(
                format!("{PREFIX}_{}_floor_footprint_land", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z + 1.5);
    }
    markers
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 76.0, -STATION_Y / 2.0 + 76.0),
        (STATION_X / 2.0 - 76.0, -STATION_Y / 2.0 + 76.0),
        (-STATION_X / 2.0 + 76.0, STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 76.0, STATION_Y / 2.0 - 76.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("{PREFIX}_datum_target_boss_{i}"), 17.0, 7.0, 36)
            .translate(*x, *y, BASE_Z + 3.5);
        let center = centered_cylinder(
            format!("{PREFIX}_datum_target_center_cut_{i}"),
            3.4,
            9.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        targets = targets + (boss - center);
    }
    targets
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_leak_witness_ribs"));
    for i in 0..8 {
        ribs = ribs
            + centered_cube(format!("{PREFIX}_leak_witness_rib_{i}"), 112.0, 4.0, 5.0).translate(
                -466.0 + (i % 4) as f64 * 112.0,
                -336.0 + (i / 4) as f64 * 42.0,
                BASE_Z + 2.5,
            );
    }
    ribs
}

fn chip_lane_nest() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_chip_lane_nest_body"),
        CHIP_NEST_X,
        CHIP_NEST_Y,
        CHIP_NEST_Z,
    )
    .translate(0.0, 0.0, CHIP_NEST_Z / 2.0);
    body - chip_lane_socket_cuts() + chip_gasket_lands() + chip_lane_locator_posts()
}

fn chip_lane_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_chip_lane_socket_cuts"));
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_grid_xy(lane);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_chip_lane_socket_cut_{lane}"),
                CHIP_SOCKET_X,
                CHIP_SOCKET_Y,
                CHIP_POCKET_DEPTH + 2.0,
            )
            .translate(x, y, CHIP_NEST_Z - CHIP_POCKET_DEPTH / 2.0 + 1.0);
    }
    cuts
}

fn chip_gasket_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_chip_lane_gasket_lands"));
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_grid_xy(lane);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_chip_lane_{lane}_front_gasket_land"),
                CHIP_SOCKET_X + GASKET_LAND_W * 2.0,
                GASKET_LAND_W,
                5.0,
            )
            .translate(x, y - CHIP_SOCKET_Y / 2.0, CHIP_NEST_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_chip_lane_{lane}_rear_gasket_land"),
                CHIP_SOCKET_X + GASKET_LAND_W * 2.0,
                GASKET_LAND_W,
                5.0,
            )
            .translate(x, y + CHIP_SOCKET_Y / 2.0, CHIP_NEST_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_chip_lane_{lane}_left_gasket_land"),
                GASKET_LAND_W,
                CHIP_SOCKET_Y + GASKET_LAND_W * 2.0,
                5.0,
            )
            .translate(x - CHIP_SOCKET_X / 2.0, y, CHIP_NEST_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_chip_lane_{lane}_right_gasket_land"),
                GASKET_LAND_W,
                CHIP_SOCKET_Y + GASKET_LAND_W * 2.0,
                5.0,
            )
            .translate(x + CHIP_SOCKET_X / 2.0, y, CHIP_NEST_Z + 2.5);
    }
    lands
}

fn chip_lane_locator_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_chip_lane_locator_posts"));
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_grid_xy(lane);
        for (i, dx) in [-42.0, 42.0].iter().enumerate() {
            posts = posts
                + centered_cylinder(
                    format!("{PREFIX}_chip_lane_{lane}_locator_post_{i}"),
                    4.0,
                    12.0,
                    20,
                )
                .translate(
                    x + dx,
                    y + CHIP_SOCKET_Y / 2.0 + 14.0,
                    CHIP_NEST_Z + 6.0,
                );
        }
    }
    posts
}

fn calibrated_restriction_ladders() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_calibrated_restriction_ladder_block"),
        LADDER_X,
        LADDER_Y,
        LADDER_Z,
    )
    .translate(0.0, 0.0, LADDER_Z / 2.0);
    block - ladder_channel_cuts() - ladder_bore_cuts() + ladder_step_tokens() + ladder_lane_keys()
}

fn ladder_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!(
        "{PREFIX}_calibrated_restriction_ladder_channel_cuts"
    ));
    for lane in 0..LANE_COUNT {
        let y = restriction_lane_y(lane);
        for step in 0..LADDER_STEPS {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_restriction_lane_{lane}_step_{step}_cut"),
                    restriction_length(step),
                    restriction_width(step),
                    14.0,
                )
                .translate(restriction_step_x(step), y, LADDER_Z / 2.0);
        }
    }
    cuts
}

fn ladder_bore_cuts() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_calibrated_restriction_ladder_bore_cuts"));
    for lane in 0..LANE_COUNT {
        let y = restriction_lane_y(lane);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_restriction_lane_{lane}_inlet_bore_cut"),
                RESTRICTION_BORE_D / 2.0,
                LADDER_X + 10.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, LADDER_Z / 2.0);
    }
    bores
}

fn ladder_step_tokens() -> Part {
    let mut tokens = Part::empty(format!(
        "{PREFIX}_calibrated_restriction_ladder_step_tokens"
    ));
    for lane in 0..LANE_COUNT {
        let y = restriction_lane_y(lane);
        for step in 0..LADDER_STEPS {
            tokens = tokens
                + centered_cube(
                    format!("{PREFIX}_restriction_lane_{lane}_step_{step}_token_land"),
                    20.0 + step as f64 * 3.0,
                    8.0,
                    RESTRICTION_TOKEN_Z,
                )
                .translate(
                    restriction_step_x(step),
                    y + LADDER_LANE_PITCH_Y / 2.0 - 8.0,
                    LADDER_Z + RESTRICTION_TOKEN_Z / 2.0,
                );
        }
    }
    tokens
}

fn ladder_lane_keys() -> Part {
    let mut keys = Part::empty(format!("{PREFIX}_calibrated_restriction_ladder_lane_keys"));
    for lane in 0..LANE_COUNT {
        keys = keys
            + centered_cube(
                format!("{PREFIX}_restriction_lane_{lane}_keyed_front_stop"),
                22.0,
                10.0 + lane as f64,
                12.0,
            )
            .translate(
                -LADDER_X / 2.0 + 22.0,
                restriction_lane_y(lane),
                LADDER_Z + 6.0,
            );
    }
    keys
}

fn pressure_tap_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_pressure_tap_manifold_body"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);
    let common = centered_cylinder(
        format!("{PREFIX}_pressure_tap_manifold_common_bore_cut"),
        PRESSURE_COMMON_BORE_D / 2.0,
        PRESSURE_X + 10.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);
    body - common - pressure_tap_cuts() + pressure_transducer_lands() + pressure_stage_labels()
}

fn pressure_tap_cuts() -> Part {
    let mut taps = Part::empty(format!("{PREFIX}_pressure_tap_cuts"));
    for lane in 0..LANE_COUNT {
        for stage in 0..PRESSURE_TAPS_PER_LANE {
            taps = taps
                + centered_cylinder(
                    format!("{PREFIX}_pressure_lane_{lane}_stage_{stage}_tap_cut"),
                    PRESSURE_TAP_D / 2.0,
                    PRESSURE_Z + 8.0,
                    28,
                )
                .translate(
                    pressure_lane_x(lane),
                    pressure_stage_y(stage),
                    PRESSURE_Z / 2.0,
                );
        }
    }
    taps
}

fn pressure_transducer_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pressure_transducer_lands"));
    for lane in 0..LANE_COUNT {
        for stage in 0..PRESSURE_TAPS_PER_LANE {
            lands = lands
                + centered_cube(
                    format!("{PREFIX}_pressure_lane_{lane}_stage_{stage}_transducer_land"),
                    24.0,
                    18.0,
                    6.0,
                )
                .translate(
                    pressure_lane_x(lane),
                    pressure_stage_y(stage),
                    PRESSURE_Z + 3.0,
                );
        }
    }
    lands
}

fn pressure_stage_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_pressure_stage_label_lands"));
    for stage in 0..PRESSURE_TAPS_PER_LANE {
        labels = labels
            + centered_cube(
                format!("{PREFIX}_pressure_stage_{stage}_label_land"),
                76.0,
                8.0,
                4.0,
            )
            .translate(
                -PRESSURE_X / 2.0 - 12.0,
                pressure_stage_y(stage),
                PRESSURE_Z + 2.0,
            );
    }
    labels
}

fn differential_flow_witness_lanes() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_differential_flow_witness_lane_block"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);
    body - witness_channel_cuts()
        + witness_window_frames()
        + witness_tick_marks()
        + witness_pair_links()
}

fn witness_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_differential_flow_witness_channel_cuts"));
    for pair in 0..WITNESS_PAIRS {
        for channel in 0..WITNESS_CHANNELS_PER_PAIR {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_witness_pair_{pair}_channel_{channel}_cut"),
                    WITNESS_X - 70.0,
                    WITNESS_CHANNEL_W,
                    10.0,
                )
                .translate(0.0, witness_channel_y(pair, channel), WITNESS_Z / 2.0);
        }
    }
    cuts
}

fn witness_window_frames() -> Part {
    let mut frames = Part::empty(format!("{PREFIX}_differential_flow_witness_window_frames"));
    for pair in 0..WITNESS_PAIRS {
        for channel in 0..WITNESS_CHANNELS_PER_PAIR {
            for window in 0..3 {
                frames = frames
                    + centered_cube(
                        format!("{PREFIX}_witness_pair_{pair}_channel_{channel}_window_{window}"),
                        WITNESS_WINDOW_X,
                        WITNESS_WINDOW_Y,
                        6.0,
                    )
                    .translate(
                        centered_index(window, 3, 120.0),
                        witness_channel_y(pair, channel),
                        WITNESS_Z + 3.0,
                    );
            }
        }
    }
    frames
}

fn witness_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_differential_flow_witness_tick_marks"));
    for pair in 0..WITNESS_PAIRS {
        for channel in 0..WITNESS_CHANNELS_PER_PAIR {
            for tick in 0..WITNESS_TICK_COUNT {
                ticks = ticks
                    + centered_cube(
                        format!("{PREFIX}_witness_pair_{pair}_channel_{channel}_tick_{tick}"),
                        3.0,
                        10.0,
                        4.0,
                    )
                    .translate(
                        centered_index(tick, WITNESS_TICK_COUNT, 48.0),
                        witness_channel_y(pair, channel) + 10.0,
                        WITNESS_Z + 2.0,
                    );
            }
        }
    }
    ticks
}

fn witness_pair_links() -> Part {
    let mut links = Part::empty(format!("{PREFIX}_differential_flow_witness_pair_links"));
    for pair in 0..WITNESS_PAIRS {
        links = links
            + centered_cube(
                format!("{PREFIX}_witness_pair_{pair}_delta_bridge_land"),
                28.0,
                22.0,
                7.0,
            )
            .translate(
                WITNESS_X / 2.0 - 34.0,
                witness_pair_y(pair),
                WITNESS_Z + 3.5,
            );
    }
    links
}

fn shear_reference_channel_coupons() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_shear_reference_channel_coupon_rack"),
        SHEAR_X,
        SHEAR_Y,
        SHEAR_Z,
    )
    .translate(0.0, 0.0, SHEAR_Z / 2.0);
    rack - shear_coupon_slot_cuts() + shear_coupon_bodies() + shear_channel_height_tokens()
}

fn shear_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_shear_reference_coupon_slot_cuts"));
    for coupon in 0..SHEAR_COUPONS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_shear_reference_coupon_slot_cut_{coupon}"),
                172.0,
                28.0,
                SHEAR_Z + 2.0,
            )
            .translate(0.0, shear_coupon_y(coupon), SHEAR_Z / 2.0);
    }
    cuts
}

fn shear_coupon_bodies() -> Part {
    let mut coupons = Part::empty(format!("{PREFIX}_shear_reference_coupon_bodies"));
    for coupon in 0..SHEAR_COUPONS {
        let body = centered_cube(
            format!("{PREFIX}_shear_reference_coupon_{coupon}_body"),
            160.0,
            24.0,
            8.0,
        );
        coupons = coupons
            + (body - shear_channel_cuts(coupon)).translate(
                0.0,
                shear_coupon_y(coupon),
                SHEAR_Z + 4.0,
            );
    }
    coupons
}

fn shear_channel_cuts(coupon: usize) -> Part {
    let mut channels = Part::empty(format!(
        "{PREFIX}_shear_reference_coupon_{coupon}_channel_cuts"
    ));
    for channel in 0..SHEAR_CHANNELS_PER_COUPON {
        channels = channels
            + centered_cube(
                format!("{PREFIX}_shear_reference_coupon_{coupon}_channel_{channel}_cut"),
                146.0,
                SHEAR_CHANNEL_W - channel as f64 * 1.2,
                SHEAR_CHANNEL_H + channel as f64 * 0.4,
            )
            .translate(
                0.0,
                centered_index(channel, SHEAR_CHANNELS_PER_COUPON, 7.0),
                1.0,
            );
    }
    channels
}

fn shear_channel_height_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_shear_reference_channel_height_tokens"));
    for coupon in 0..SHEAR_COUPONS {
        for channel in 0..SHEAR_CHANNELS_PER_COUPON {
            tokens = tokens
                + centered_cube(
                    format!("{PREFIX}_shear_reference_coupon_{coupon}_height_token_{channel}"),
                    12.0,
                    4.0 + channel as f64 * 2.0,
                    4.0,
                )
                .translate(
                    -SHEAR_X / 2.0 + 18.0,
                    shear_coupon_y(coupon)
                        + centered_index(channel, SHEAR_CHANNELS_PER_COUPON, 7.0),
                    SHEAR_Z + 10.0,
                );
        }
    }
    tokens
}

fn bubble_safe_prime_purge_references() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_safe_prime_purge_reference_block"),
        PRIME_X,
        PRIME_Y,
        PRIME_Z,
    )
    .translate(0.0, 0.0, PRIME_Z / 2.0);
    body - prime_purge_bores()
        + prime_trap_chimneys()
        + purge_port_collars()
        + prime_direction_lands()
}

fn prime_purge_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_bubble_safe_prime_purge_bores"));
    let header = centered_cylinder(
        format!("{PREFIX}_bubble_safe_purge_common_header_cut"),
        PURGE_HEADER_D / 2.0,
        PRIME_X + 8.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, PRIME_Y / 2.0 - 34.0, PRIME_Z / 2.0);
    bores = bores + header;
    for reference in 0..PRIME_REFERENCES {
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_prime_reference_{reference}_vertical_trap_core_cut"),
                PRIME_TRAP_D / 2.0 - 5.0,
                PRIME_Z + 8.0,
                32,
            )
            .translate(prime_reference_x(reference), -14.0, PRIME_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_purge_reference_{reference}_port_cut"),
                PURGE_PORT_D / 2.0,
                PRIME_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                prime_reference_x(reference),
                PRIME_Y / 2.0 - 16.0,
                PRIME_Z / 2.0,
            );
    }
    bores
}

fn prime_trap_chimneys() -> Part {
    let mut traps = Part::empty(format!("{PREFIX}_bubble_safe_prime_trap_chimneys"));
    for reference in 0..PRIME_REFERENCES {
        let x = prime_reference_x(reference);
        let outer = centered_cylinder(
            format!("{PREFIX}_prime_reference_{reference}_uphill_trap_chimney_outer"),
            PRIME_TRAP_D / 2.0,
            PRIME_CHIMNEY_Z,
            36,
        )
        .translate(x, -14.0, PRIME_Z + PRIME_CHIMNEY_Z / 2.0);
        let inner = centered_cylinder(
            format!("{PREFIX}_prime_reference_{reference}_uphill_trap_chimney_inner_cut"),
            PRIME_TRAP_D / 2.0 - 5.0,
            PRIME_CHIMNEY_Z + 2.0,
            32,
        )
        .translate(x, -14.0, PRIME_Z + PRIME_CHIMNEY_Z / 2.0);
        let uphill_marker = centered_cube(
            format!("{PREFIX}_prime_reference_{reference}_uphill_flow_marker"),
            32.0,
            8.0,
            5.0,
        )
        .translate(x, -PRIME_Y / 2.0 + 18.0, PRIME_Z + 2.5);
        traps = traps + (outer - inner) + uphill_marker;
    }
    traps
}

fn purge_port_collars() -> Part {
    let mut collars = Part::empty(format!("{PREFIX}_bubble_safe_purge_port_collars"));
    for reference in 0..PRIME_REFERENCES {
        let x = prime_reference_x(reference);
        let outer = centered_cylinder(
            format!("{PREFIX}_purge_reference_{reference}_port_collar_outer"),
            14.0,
            7.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PRIME_Y / 2.0 + 3.5, PRIME_Z / 2.0);
        let inner = centered_cylinder(
            format!("{PREFIX}_purge_reference_{reference}_port_collar_inner_clearance"),
            PURGE_PORT_D / 2.0,
            9.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PRIME_Y / 2.0 + 3.5, PRIME_Z / 2.0);
        collars = collars + (outer - inner);
    }
    collars
}

fn prime_direction_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_bubble_safe_prime_direction_lands"));
    for reference in 0..PRIME_REFERENCES {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_prime_reference_{reference}_purge_direction_land"),
                28.0,
                8.0,
                5.0,
            )
            .translate(
                prime_reference_x(reference),
                PRIME_Y / 2.0 - 18.0,
                PRIME_Z + 2.5,
            );
    }
    lands
}

fn viscosity_temperature_reference_pockets() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_viscosity_temperature_reference_pocket_block"),
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    )
    .translate(0.0, 0.0, REFERENCE_Z / 2.0);
    body - viscosity_well_cuts() - temperature_probe_pocket_cuts() + reference_certificate_tabs()
}

fn viscosity_well_cuts() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_viscosity_reference_well_cuts"));
    for well in 0..VISCOSITY_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_viscosity_reference_well_cut_{well}"),
                REFERENCE_WELL_D / 2.0,
                REFERENCE_Z + 4.0,
                32,
            )
            .translate(
                centered_index(well, VISCOSITY_WELLS, 48.0),
                26.0,
                REFERENCE_Z / 2.0,
            );
    }
    wells
}

fn temperature_probe_pocket_cuts() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_temperature_probe_pocket_cuts"));
    for pocket in 0..TEMPERATURE_POCKET_COUNT {
        pockets = pockets
            + centered_cube(
                format!("{PREFIX}_temperature_probe_pocket_cut_{pocket}"),
                38.0,
                18.0,
                16.0,
            )
            .translate(
                centered_index(pocket, TEMPERATURE_POCKET_COUNT, 58.0),
                -34.0,
                REFERENCE_Z - 7.0,
            );
    }
    pockets
}

fn reference_certificate_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_viscosity_temperature_certificate_tabs"));
    for tab in 0..3 {
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_reference_certificate_tab_{tab}"),
                48.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(tab, 3, 66.0),
                -REFERENCE_Y / 2.0 - 5.0,
                REFERENCE_Z + 2.5,
            );
    }
    tabs
}

fn tracer_recovery_waste_capture() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_tracer_recovery_waste_capture_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0);
    body - waste_well_cuts() + tracer_gradient_lands() + waste_lid_locator_tabs()
}

fn waste_well_cuts() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_tracer_recovery_waste_well_cuts"));
    for well in 0..WASTE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_tracer_recovery_waste_well_cut_{well}"),
                WASTE_WELL_D / 2.0,
                WASTE_Z + 4.0,
                32,
            )
            .translate(centered_index(well, WASTE_WELLS, 42.0), 18.0, WASTE_Z / 2.0);
    }
    wells
}

fn tracer_gradient_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_tracer_gradient_lands"));
    for step in 0..TRACER_GRADIENT_STEPS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_tracer_gradient_step_{step}_land"),
                16.0 + step as f64 * 6.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(step, TRACER_GRADIENT_STEPS, 36.0),
                -WASTE_Y / 2.0 + 20.0,
                WASTE_Z + 2.5,
            );
    }
    lands
}

fn waste_lid_locator_tabs() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_waste_capture_front_lid_locator_tab"),
        WASTE_X - 36.0,
        8.0,
        10.0,
    )
    .translate(0.0, -WASTE_Y / 2.0 + 10.0, WASTE_Z + 5.0);
    let rear = centered_cube(
        format!("{PREFIX}_waste_capture_rear_lid_locator_tab"),
        WASTE_X - 36.0,
        8.0,
        10.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 10.0, WASTE_Z + 5.0);
    front + rear
}

fn barcode_certificate_status_lanes() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_certificate_status_lane_plate"),
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    plate + barcode_lands() + certificate_lands() + release_hold_reject_lanes()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for lane in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_barcode_land"),
                26.0,
                56.0,
                4.0,
            )
            .translate(
                centered_index(lane, BARCODE_LANDS, 36.0),
                50.0,
                STATUS_Z + 2.0,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_certificate_lands"));
    for cert in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_certificate_land_{cert}"), 58.0, 20.0, 5.0)
                .translate(
                    centered_index(cert, CERTIFICATE_LANDS, 72.0),
                    5.0,
                    STATUS_Z + 2.5,
                );
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty(format!("{PREFIX}_release_hold_reject_lanes"));
    for lane in 0..STATUS_LANES {
        let y = -STATUS_Y / 2.0 + 30.0 + lane as f64 * 24.0;
        lanes = lanes
            + centered_cube(
                format!("{PREFIX}_status_lane_{lane}_header_land"),
                54.0,
                14.0,
                5.0,
            )
            .translate(-STATUS_X / 2.0 + 36.0, y, STATUS_Z + 2.5);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            lanes = lanes
                + centered_cube(
                    format!("{PREFIX}_status_lane_{lane}_slot_{slot}"),
                    28.0,
                    14.0,
                    5.0,
                )
                .translate(-54.0 + slot as f64 * 38.0, y, STATUS_Z + 2.5);
        }
    }
    lanes
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_access_keepout_gauge"),
        STATION_X - 180.0,
        FRONT_ROBOT_CLEARANCE,
        64.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0),
        BASE_Z + 32.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_tubing_service_keepout_gauge"),
        STATION_X - 260.0,
        REAR_TUBING_SERVICE_CLEARANCE,
        78.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_TUBING_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 39.0,
    );
    let left = centered_cube(
        format!("{PREFIX}_left_pressure_service_keepout_gauge"),
        LEFT_PRESSURE_SERVICE_CLEARANCE,
        STATION_Y - 168.0,
        70.0,
    )
    .translate(
        -(STATION_X / 2.0 + LEFT_PRESSURE_SERVICE_CLEARANCE / 2.0),
        0.0,
        BASE_Z + 35.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_waste_service_keepout_gauge"),
        RIGHT_WASTE_SERVICE_CLEARANCE,
        STATION_Y - 168.0,
        70.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_WASTE_SERVICE_CLEARANCE / 2.0,
        0.0,
        BASE_Z + 35.0,
    );
    let top = centered_cube(
        format!("{PREFIX}_top_calibration_lane_lift_keepout_gauge"),
        CHIP_NEST_X + LADDER_X + 72.0,
        16.0,
        8.0,
    )
    .translate(
        -58.0,
        CHIP_NEST_POS.1 + CHIP_NEST_Y / 2.0 + 22.0,
        BASE_Z + TOP_CASSETTE_LIFT_CLEARANCE,
    );
    front + rear + left + right + top
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "chip_lane_nest",
            center: CHIP_NEST_POS,
            x: CHIP_NEST_X,
            y: CHIP_NEST_Y,
        },
        Rect {
            name: "calibrated_restriction_ladders",
            center: LADDER_POS,
            x: LADDER_X,
            y: LADDER_Y,
        },
        Rect {
            name: "pressure_tap_manifold",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "differential_flow_witness_lanes",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "shear_reference_channel_coupons",
            center: SHEAR_POS,
            x: SHEAR_X,
            y: SHEAR_Y,
        },
        Rect {
            name: "bubble_safe_prime_purge_references",
            center: PRIME_POS,
            x: PRIME_X,
            y: PRIME_Y,
        },
        Rect {
            name: "viscosity_temperature_reference_pockets",
            center: REFERENCE_POS,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Rect {
            name: "tracer_recovery_waste_capture",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "barcode_certificate_status_lanes",
            center: STATUS_POS,
            x: STATUS_X,
            y: STATUS_Y,
        },
    ]
}

fn mount_points() -> [(f64, f64); 6] {
    [
        (-STATION_X / 2.0 + 68.0, -STATION_Y / 2.0 + 64.0),
        (STATION_X / 2.0 - 68.0, -STATION_Y / 2.0 + 64.0),
        (-STATION_X / 2.0 + 68.0, STATION_Y / 2.0 - 64.0),
        (STATION_X / 2.0 - 68.0, STATION_Y / 2.0 - 64.0),
        (0.0, -STATION_Y / 2.0 + 64.0),
        (0.0, STATION_Y / 2.0 - 64.0),
    ]
}

fn lane_grid_xy(lane: usize) -> (f64, f64) {
    let col = lane % LANE_COLS;
    let row = lane / LANE_COLS;
    (
        centered_index(col, LANE_COLS, CHIP_PITCH_X),
        centered_index(row, LANE_ROWS, CHIP_PITCH_Y),
    )
}

fn restriction_lane_y(lane: usize) -> f64 {
    centered_index(lane, LANE_COUNT, LADDER_LANE_PITCH_Y)
}

fn restriction_step_x(step: usize) -> f64 {
    centered_index(step, LADDER_STEPS, LADDER_STEP_PITCH_X)
}

fn restriction_width(step: usize) -> f64 {
    10.0 - step as f64 * 1.3
}

fn restriction_length(step: usize) -> f64 {
    28.0 + step as f64 * 12.0
}

fn pressure_lane_x(lane: usize) -> f64 {
    centered_index(lane, LANE_COUNT, PRESSURE_LANE_PITCH_X)
}

fn pressure_stage_y(stage: usize) -> f64 {
    centered_index(stage, PRESSURE_TAPS_PER_LANE, PRESSURE_STAGE_PITCH_Y)
}

fn witness_pair_y(pair: usize) -> f64 {
    centered_index(pair, WITNESS_PAIRS, 27.0)
}

fn witness_channel_y(pair: usize, channel: usize) -> f64 {
    witness_pair_y(pair) + centered_index(channel, WITNESS_CHANNELS_PER_PAIR, 9.0)
}

fn shear_coupon_y(coupon: usize) -> f64 {
    centered_index(coupon, SHEAR_COUPONS, SHEAR_COUPON_PITCH_Y)
}

fn prime_reference_x(reference: usize) -> f64 {
    centered_index(reference, PRIME_REFERENCES, PRIME_REF_PITCH_X)
}

fn insert_z(module_z: f64) -> f64 {
    BASE_Z + module_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(LANE_COUNT, LANE_ROWS * LANE_COLS);
    assert_eq!(WITNESS_PAIRS, LANE_COUNT);
    assert_eq!(PRIME_REFERENCES, LANE_COUNT);
    assert!(SUMP_X < STATION_X - RIM_W * 2.0);
    assert!(SUMP_Y < STATION_Y - RIM_W * 2.0);
    assert!(restriction_width(LADDER_STEPS - 1) > 3.0);
    assert!(PRESSURE_TAPS_PER_LANE >= 3);
    assert!(SHEAR_CHANNEL_H > 0.8);
    assert!(PRIME_CHIMNEY_Z > PRIME_Z / 2.0);
    assert!(TOP_CASSETTE_LIFT_CLEARANCE > PRIME_Z + PRIME_CHIMNEY_Z);
    for rect in module_rects() {
        assert!(
            rect.fits_inside_tray(),
            "{} outside containment tray",
            rect.name
        );
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
        for output in OUTPUTS {
            assert!(output
                .starts_with("output/closed_perfusion_shear_stress_calibration_lane_station_"));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_are_visible_as_outputs() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|output| output.contains(feature)),
                "missing output for {feature}"
            );
        }
    }

    #[test]
    fn lane_reproducibility_geometry_has_matched_counts() {
        assert_eq!(LANE_COUNT, 6);
        assert_eq!(LANE_COUNT, LANE_ROWS * LANE_COLS);
        assert_eq!(WITNESS_PAIRS, LANE_COUNT);
        assert_eq!(PRIME_REFERENCES, LANE_COUNT);
        assert_eq!(PRESSURE_TAPS_PER_LANE, 3);
        assert_eq!(LANE_COUNT * PRESSURE_TAPS_PER_LANE, 18);
    }

    #[test]
    fn restriction_ladder_steps_are_monotonic_and_printable() {
        for step in 1..LADDER_STEPS {
            assert!(restriction_length(step) > restriction_length(step - 1));
            assert!(restriction_width(step) < restriction_width(step - 1));
        }
        assert!(restriction_width(LADDER_STEPS - 1) >= 4.8);
        assert!(restriction_length(LADDER_STEPS - 1) <= 80.0);
    }

    #[test]
    fn pressure_witness_and_shear_arrays_fit_station() {
        assert_layout();
        for rect in module_rects() {
            assert!(
                rect.fits_inside_tray(),
                "{} outside containment tray",
                rect.name
            );
        }
        assert!(pressure_lane_x(0).abs() < PRESSURE_X / 2.0 - 42.0);
        assert!(pressure_lane_x(LANE_COUNT - 1).abs() < PRESSURE_X / 2.0 - 42.0);
        assert!(witness_channel_y(0, 0).abs() < WITNESS_Y / 2.0 - 18.0);
        assert!(
            witness_channel_y(WITNESS_PAIRS - 1, WITNESS_CHANNELS_PER_PAIR - 1).abs()
                < WITNESS_Y / 2.0 - 18.0
        );
    }

    #[test]
    fn bubble_safe_prime_purge_references_clear_robot_lift() {
        assert!(PRIME_TRAP_D >= 28.0);
        assert!(PRIME_CHIMNEY_Z >= 58.0);
        assert!(PURGE_HEADER_D >= RESTRICTION_BORE_D);
        assert!(TOP_CASSETTE_LIFT_CLEARANCE > PRIME_Z + PRIME_CHIMNEY_Z);
        assert!(FRONT_ROBOT_CLEARANCE >= 340.0);
        assert!(REAR_TUBING_SERVICE_CLEARANCE >= 240.0);
        assert!(LEFT_PRESSURE_SERVICE_CLEARANCE >= 170.0);
        assert!(RIGHT_WASTE_SERVICE_CLEARANCE >= 180.0);
    }
}
