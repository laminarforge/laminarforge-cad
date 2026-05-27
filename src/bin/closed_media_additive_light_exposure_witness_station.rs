use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/additive light-exposure witness station.
//
// Intent:
// - Hold light-sensitive media/additive surrogate vials in paired amber and
//   clear nests while witness coupons and shield/cover lanes capture exposure
//   evidence for a closed-system validation run.
// - Keep timed token lands, temperature logger evidence, barcode/certificate
//   traceability, release/hold/reject disposition lanes, clean/used
//   segregation, sealed transfer bulkhead, camera bridge, and keepout gauges
//   mechanically explicit.
// - Model the fixture envelope and interfaces only. This is not a photostability
//   protocol, dose model, acceptance criterion, release decision, or media
//   formulation instruction.

const OUTPUT_PREFIX: &str = "output/closed_media_additive_light_exposure_witness_station";
const OUTPUTS: [&str; 13] = [
    "output/closed_media_additive_light_exposure_witness_station_base_leak_tray.stl",
    "output/closed_media_additive_light_exposure_witness_station_amber_clear_vial_surrogate_nests.stl",
    "output/closed_media_additive_light_exposure_witness_station_light_exposure_witness_coupons.stl",
    "output/closed_media_additive_light_exposure_witness_station_shield_cover_comparison_lanes.stl",
    "output/closed_media_additive_light_exposure_witness_station_timed_token_lands.stl",
    "output/closed_media_additive_light_exposure_witness_station_temperature_logger_pocket.stl",
    "output/closed_media_additive_light_exposure_witness_station_barcode_certificate_lands.stl",
    "output/closed_media_additive_light_exposure_witness_station_release_hold_reject_lanes.stl",
    "output/closed_media_additive_light_exposure_witness_station_clean_used_segregation.stl",
    "output/closed_media_additive_light_exposure_witness_station_closed_transfer_seal_bulkhead.stl",
    "output/closed_media_additive_light_exposure_witness_station_evidence_bridge.stl",
    "output/closed_media_additive_light_exposure_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_media_additive_light_exposure_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "amber_vial_nests",
    "clear_vial_nests",
    "light_exposure_witness_coupons",
    "shield_cover_comparison_lanes",
    "timed_token_lands",
    "temperature_logger_pocket",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "closed_transfer_seal_bulkhead",
    "evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const RECESS_Z: f64 = 5.0;
const DRAIN_PORT_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;
const WIPE_GUTTER_W: f64 = 12.0;

const VIAL_CENTER: (f64, f64) = (-405.0, 190.0);
const VIAL_PANEL_X: f64 = 410.0;
const VIAL_PANEL_Y: f64 = 280.0;
const VIAL_PANEL_Z: f64 = 50.0;
const VIAL_ROWS: usize = 2;
const VIAL_COLS: usize = 5;
const VIAL_COUNT: usize = VIAL_ROWS * VIAL_COLS;
const AMBER_VIALS: usize = VIAL_COLS;
const CLEAR_VIALS: usize = VIAL_COLS;
const VIAL_WELL_D: f64 = 30.0;
const VIAL_WELL_DEPTH: f64 = 31.0;
const VIAL_PITCH_X: f64 = 64.0;
const VIAL_ROW_PITCH_Y: f64 = 82.0;
const VIAL_SAMPLE_CLIPS: usize = 10;
const AMBER_SHIELD_COUNT: usize = 5;

const COUPON_CENTER: (f64, f64) = (70.0, 190.0);
const COUPON_PANEL_X: f64 = 470.0;
const COUPON_PANEL_Y: f64 = 280.0;
const COUPON_PANEL_Z: f64 = 32.0;
const COUPON_ROWS: usize = 4;
const COUPON_COLS: usize = 6;
const COUPON_COUNT: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 30.0;
const COUPON_PITCH_X: f64 = 64.0;
const COUPON_PITCH_Y: f64 = 54.0;
const DOSE_CHIP_LANDS: usize = 8;
const COUPON_DATUMS: usize = 4;

const COMPARISON_CENTER: (f64, f64) = (-405.0, -95.0);
const COMPARISON_PANEL_X: f64 = 410.0;
const COMPARISON_PANEL_Y: f64 = 220.0;
const COMPARISON_PANEL_Z: f64 = 36.0;
const COMPARISON_LANE_NAMES: [&str; 3] = ["clear_lane", "amber_lane", "opaque_cover_lane"];
const COMPARISON_LANES: usize = COMPARISON_LANE_NAMES.len();
const COMPARISON_LANE_X: f64 = 104.0;
const COMPARISON_LANE_Y: f64 = 168.0;
const COMPARISON_LANE_PITCH_X: f64 = 126.0;
const COVER_SAMPLE_WINDOWS_PER_LANE: usize = 3;

const TOKEN_CENTER: (f64, f64) = (45.0, -95.0);
const TOKEN_PANEL_X: f64 = 350.0;
const TOKEN_PANEL_Y: f64 = 220.0;
const TOKEN_PANEL_Z: f64 = 28.0;
const TIMEPOINT_LABELS: [&str; 6] = ["t0", "t15", "t30", "t60", "t120", "t240"];
const TIMEPOINT_TOKENS: usize = TIMEPOINT_LABELS.len();
const TOKEN_LAND_D: f64 = 34.0;
const TOKEN_LAND_DEPTH: f64 = 11.0;
const TOKEN_PITCH_X: f64 = 52.0;
const CLOCK_REFERENCE_LANDS: usize = 3;

const LOGGER_CENTER: (f64, f64) = (425.0, -95.0);
const LOGGER_PANEL_X: f64 = 260.0;
const LOGGER_PANEL_Y: f64 = 220.0;
const LOGGER_PANEL_Z: f64 = 48.0;
const LOGGER_SLOT_X: f64 = 156.0;
const LOGGER_SLOT_Y: f64 = 76.0;
const LOGGER_SLOT_DEPTH: f64 = 23.0;
const THERMOWELL_COUNT: usize = 6;
const THERMOWELL_D: f64 = 10.0;
const THERMOWELL_PITCH_X: f64 = 30.0;
const LOGGER_SEAL_TABS: usize = 4;

const TRACE_CENTER: (f64, f64) = (-420.0, -305.0);
const TRACE_PANEL_X: f64 = 380.0;
const TRACE_PANEL_Y: f64 = 150.0;
const TRACE_PANEL_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;
const RFID_LANDS: usize = 3;
const EVIDENCE_LANDS: usize = BARCODE_LANDS + CERTIFICATE_LANDS + RFID_LANDS;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 24.0;
const CERTIFICATE_LAND_X: f64 = 106.0;
const CERTIFICATE_LAND_Y: f64 = 48.0;
const RFID_LAND_D: f64 = 40.0;

const STATUS_CENTER: (f64, f64) = (20.0, -305.0);
const STATUS_PANEL_X: f64 = 380.0;
const STATUS_PANEL_Y: f64 = 150.0;
const STATUS_PANEL_Z: f64 = 32.0;
const STATUS_NAMES: [&str; 3] = ["release", "hold", "reject"];
const STATUS_LANES: usize = STATUS_NAMES.len();
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 82.0;
const STATUS_SLOT_Y: f64 = 28.0;
const STATUS_LANE_PITCH_Y: f64 = 42.0;

const SEGREGATION_CENTER: (f64, f64) = (0.0, -220.0);
const SEGREGATION_WALL_X: f64 = 1120.0;
const SEGREGATION_WALL_Y: f64 = 16.0;
const SEGREGATION_WALL_Z: f64 = 78.0;
const SEGREGATION_GATE_X: f64 = 132.0;
const CLEAN_USED_TOKEN_LANDS: usize = 10;

const BULKHEAD_CENTER: (f64, f64) = (450.0, 190.0);
const BULKHEAD_X: f64 = 240.0;
const BULKHEAD_Y: f64 = 64.0;
const BULKHEAD_Z: f64 = 144.0;
const TRANSFER_PORTS: usize = 6;
const TRANSFER_PORT_COLS: usize = 3;
const TRANSFER_PORT_D: f64 = 22.0;
const TRANSFER_COLLAR_D: f64 = 38.0;
const TRANSFER_PORT_PITCH_X: f64 = 58.0;
const TRANSFER_PORT_PITCH_Z: f64 = 48.0;
const SEAL_WITNESS_LANDS: usize = 4;

const BRIDGE_CENTER: (f64, f64) = (10.0, 70.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 52.0;
const BRIDGE_POST_Z: f64 = 222.0;
const BRIDGE_BEAM_Y: f64 = 58.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_PODS: usize = 5;
const CAMERA_PITCH_X: f64 = 230.0;
const AMBER_LED_BARS: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 178.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 390.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 260.0;
const LEFT_SERVICE_KEEP_OUT_X: f64 = 230.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 260.0;
const COVER_LIFT_CLEARANCE_Z: f64 = 320.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;
const KEEP_OUT_RAIL_W: f64 = 8.0;

#[derive(Clone, Copy)]
struct ModuleEnvelope {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl ModuleEnvelope {
    fn fits_on_deck(self) -> bool {
        self.center.0 - self.x / 2.0 >= -DECK_X / 2.0 + RIM_W + 8.0
            && self.center.0 + self.x / 2.0 <= DECK_X / 2.0 - RIM_W - 8.0
            && self.center.1 - self.y / 2.0 >= -DECK_Y / 2.0 + RIM_W + 8.0
            && self.center.1 + self.y / 2.0 <= DECK_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: Self) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = base_leak_tray();
    export(OUTPUTS[0], &deck);

    let vial_nests = amber_clear_vial_surrogate_nests();
    export(OUTPUTS[1], &vial_nests);

    let coupons = light_exposure_witness_coupons();
    export(OUTPUTS[2], &coupons);

    let comparison = shield_cover_comparison_lanes();
    export(OUTPUTS[3], &comparison);

    let tokens = timed_token_lands();
    export(OUTPUTS[4], &tokens);

    let logger = temperature_logger_pocket();
    export(OUTPUTS[5], &logger);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[7], &status);

    let segregation = clean_used_segregation();
    export(OUTPUTS[8], &segregation);

    let bulkhead = closed_transfer_seal_bulkhead();
    export(OUTPUTS[9], &bulkhead);

    let bridge = evidence_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media/additive light-exposure witness station:");
    println!(
        "  Footprint:                 {DECK_X:.0}mm x {DECK_Y:.0}mm leak tray with sealed transfer evidence flow"
    );
    println!(
        "  Vial surrogate nests:      {AMBER_VIALS} amber and {CLEAR_VIALS} clear wells with {VIAL_SAMPLE_CLIPS} custody clips and {AMBER_SHIELD_COUNT} amber shield lands"
    );
    println!(
        "  Exposure witnesses:        {COUPON_ROWS} x {COUPON_COLS} coupon grid ({COUPON_COUNT} coupons), {DOSE_CHIP_LANDS} dose-chip lands, {COUPON_DATUMS} datums"
    );
    println!(
        "  Shield comparison:         {COMPARISON_LANES} lanes x {COVER_SAMPLE_WINDOWS_PER_LANE} sample windows for clear, amber, and opaque cover states"
    );
    println!(
        "  Timed evidence:            {TIMEPOINT_TOKENS} token lands, {CLOCK_REFERENCE_LANDS} clock/reference lands, {THERMOWELL_COUNT} thermowells, logger seal tabs {LOGGER_SEAL_TABS}"
    );
    println!(
        "  Traceability/disposition:  {EVIDENCE_LANDS} barcode/certificate/RFID lands, {STATUS_LANES} release/hold/reject lanes, {CLEAN_USED_TOKEN_LANDS} clean/used custody lands"
    );
    println!(
        "  Closed transfer:           {TRANSFER_PORTS} sealed bulkhead ports, {SEAL_WITNESS_LANDS} seal witness lands, {CAMERA_PODS} camera pods, {AMBER_LED_BARS} amber-safe evidence bars"
    );
    println!(
        "  Keepouts:                  front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, left service {LEFT_SERVICE_KEEP_OUT_X:.0}mm, right service {RIGHT_SERVICE_KEEP_OUT_X:.0}mm, cover lift {COVER_LIFT_CLEARANCE_Z:.0}mm"
    );
    println!("  Feature groups covered:    {}", REQUIRED_FEATURES.len());
    println!("  Output prefix:             {OUTPUT_PREFIX}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray()
        + amber_clear_vial_surrogate_nests().translate(VIAL_CENTER.0, VIAL_CENTER.1, DECK_Z)
        + light_exposure_witness_coupons().translate(COUPON_CENTER.0, COUPON_CENTER.1, DECK_Z)
        + shield_cover_comparison_lanes().translate(
            COMPARISON_CENTER.0,
            COMPARISON_CENTER.1,
            DECK_Z,
        )
        + timed_token_lands().translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z)
        + temperature_logger_pocket().translate(LOGGER_CENTER.0, LOGGER_CENTER.1, DECK_Z)
        + barcode_certificate_lands().translate(TRACE_CENTER.0, TRACE_CENTER.1, DECK_Z)
        + release_hold_reject_lanes().translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z)
        + clean_used_segregation().translate(SEGREGATION_CENTER.0, SEGREGATION_CENTER.1, DECK_Z)
        + closed_transfer_seal_bulkhead().translate(BULKHEAD_CENTER.0, BULKHEAD_CENTER.1, DECK_Z)
        + evidence_bridge().translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, DECK_Z)
        + robot_service_keepout_gauges()
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("light_exposure_station_base_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    deck - module_recesses() - mounting_holes() - wipe_gutters() - drain_port()
        + perimeter_rim()
        + row_dividers()
        + station_fiducials()
}

fn module_recesses() -> Part {
    let mut recesses = Part::empty("light_exposure_station_module_recesses");
    for module in layout_envelopes() {
        recesses = recesses
            + top_recess(
                format!("light_exposure_station_{}_socket", module.name),
                module.center,
                module.x + 14.0,
                module.y + 14.0,
                RECESS_Z,
            );
    }
    recesses
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("light_exposure_station_mounting_holes");
    for (index, (x, y)) in [
        (-DECK_X / 2.0 + 62.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 62.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 62.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 4.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 4.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 4.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 4.0, DECK_Y / 2.0 - 58.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("light_exposure_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn wipe_gutters() -> Part {
    let front = centered_cube(
        "light_exposure_station_front_wipe_gutter",
        DECK_X - 160.0,
        WIPE_GUTTER_W,
        RECESS_Z + 0.2,
    )
    .translate(0.0, -DECK_Y / 2.0 + 92.0, DECK_Z - RECESS_Z / 2.0 + 0.1);
    let rear = centered_cube(
        "light_exposure_station_rear_wipe_gutter",
        DECK_X - 160.0,
        WIPE_GUTTER_W,
        RECESS_Z + 0.2,
    )
    .translate(0.0, DECK_Y / 2.0 - 92.0, DECK_Z - RECESS_Z / 2.0 + 0.1);
    let center = centered_cube(
        "light_exposure_station_clean_used_wipe_gutter",
        DECK_X - 210.0,
        WIPE_GUTTER_W,
        RECESS_Z + 0.2,
    )
    .translate(0.0, SEGREGATION_CENTER.1, DECK_Z - RECESS_Z / 2.0 + 0.1);
    front + rear + center
}

fn drain_port() -> Part {
    centered_cylinder(
        "light_exposure_station_leak_drain_port",
        DRAIN_PORT_D / 2.0,
        RIM_W + 28.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 98.0, -DECK_Y / 2.0 + 10.0, DECK_Z - 6.0)
}

fn perimeter_rim() -> Part {
    let front = centered_cube("light_exposure_station_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("light_exposure_station_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("light_exposure_station_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("light_exposure_station_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn row_dividers() -> Part {
    let upper_lower = centered_cube(
        "light_exposure_station_upper_lower_row_divider",
        DECK_X - 160.0,
        9.0,
        24.0,
    )
    .translate(0.0, 38.0, DECK_Z + 12.0);
    let evidence_disposition = centered_cube(
        "light_exposure_station_evidence_disposition_divider",
        DECK_X - 180.0,
        9.0,
        24.0,
    )
    .translate(0.0, -238.0, DECK_Z + 12.0);
    let vial_coupon_split = centered_cube(
        "light_exposure_station_vial_coupon_split_rail",
        9.0,
        280.0,
        24.0,
    )
    .translate(-188.0, 190.0, DECK_Z + 12.0);
    let token_logger_split = centered_cube(
        "light_exposure_station_token_logger_split_rail",
        9.0,
        216.0,
        24.0,
    )
    .translate(260.0, -95.0, DECK_Z + 12.0);

    upper_lower + evidence_disposition + vial_coupon_split + token_logger_split
}

fn station_fiducials() -> Part {
    let mut fiducials = Part::empty("light_exposure_station_robot_fiducials");
    for (index, (x, y)) in [
        (-DECK_X / 2.0 + 96.0, -DECK_Y / 2.0 + 102.0),
        (DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 102.0),
        (-DECK_X / 2.0 + 96.0, DECK_Y / 2.0 - 102.0),
        (DECK_X / 2.0 - 96.0, DECK_Y / 2.0 - 102.0),
        (0.0, DECK_Y / 2.0 - 102.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(format!("light_exposure_station_fiducial_{index}")).translate(
                x,
                y,
                DECK_Z + 1.5,
            );
    }
    fiducials
}

fn amber_clear_vial_surrogate_nests() -> Part {
    let body = centered_cube(
        "light_exposure_station_vial_surrogate_nest_body",
        VIAL_PANEL_X,
        VIAL_PANEL_Y,
        VIAL_PANEL_Z,
    )
    .translate(0.0, 0.0, VIAL_PANEL_Z / 2.0);

    body - vial_wells() - vial_finger_reliefs()
        + amber_shield_lands()
        + clear_reference_lands()
        + vial_custody_clip_lands()
        + vial_nest_guard_rails()
}

fn vial_wells() -> Part {
    let mut wells = Part::empty("light_exposure_station_vial_surrogate_wells");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let index = row * VIAL_COLS + col;
            let row_name = if row == 0 { "amber" } else { "clear" };
            wells = wells
                + centered_cylinder(
                    format!("light_exposure_station_{row_name}_vial_well_{col}"),
                    VIAL_WELL_D / 2.0,
                    VIAL_WELL_DEPTH + 0.4,
                    40,
                )
                .translate(
                    centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                    vial_row_y(row),
                    VIAL_PANEL_Z - VIAL_WELL_DEPTH / 2.0 + 0.2,
                )
                + centered_cube(
                    format!("light_exposure_station_vial_lot_card_recess_{index}"),
                    46.0,
                    13.0,
                    6.0,
                )
                .translate(
                    centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                    vial_row_y(row) - 31.0,
                    VIAL_PANEL_Z - 3.0,
                );
        }
    }
    wells
}

fn vial_finger_reliefs() -> Part {
    let mut reliefs = Part::empty("light_exposure_station_vial_finger_reliefs");
    for col in 0..VIAL_COLS {
        reliefs = reliefs
            + centered_cube(
                format!("light_exposure_station_vial_finger_relief_col_{col}"),
                12.0,
                VIAL_ROW_PITCH_Y + 42.0,
                9.0,
            )
            .translate(
                centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                0.0,
                VIAL_PANEL_Z - 4.5,
            );
    }
    reliefs
}

fn amber_shield_lands() -> Part {
    let mut shields = Part::empty("light_exposure_station_amber_shield_lands");
    for col in 0..AMBER_SHIELD_COUNT {
        let x = centered_index(col, AMBER_SHIELD_COUNT, VIAL_PITCH_X);
        shields = shields
            + centered_cube(
                format!("light_exposure_station_amber_shield_saddle_{col}"),
                46.0,
                15.0,
                12.0,
            )
            .translate(x, vial_row_y(0) + 33.0, VIAL_PANEL_Z + 6.0)
            + centered_cube(
                format!("light_exposure_station_amber_shield_backstop_{col}"),
                50.0,
                6.0,
                30.0,
            )
            .translate(x, vial_row_y(0) + 50.0, VIAL_PANEL_Z + 15.0);
    }
    shields
}

fn clear_reference_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_clear_reference_lands");
    for col in 0..CLEAR_VIALS {
        let x = centered_index(col, CLEAR_VIALS, VIAL_PITCH_X);
        lands = lands
            + centered_cube(
                format!("light_exposure_station_clear_window_reference_land_{col}"),
                44.0,
                16.0,
                5.0,
            )
            .translate(x, vial_row_y(1) + 32.0, VIAL_PANEL_Z + 2.5);
    }
    lands
}

fn vial_custody_clip_lands() -> Part {
    let mut clips = Part::empty("light_exposure_station_vial_custody_clip_lands");
    for index in 0..VIAL_SAMPLE_CLIPS {
        let col = index % VIAL_COLS;
        let row = index / VIAL_COLS;
        clips = clips
            + centered_cube(
                format!("light_exposure_station_vial_custody_clip_{index}"),
                34.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                vial_row_y(row) - 50.0,
                VIAL_PANEL_Z + 3.0,
            );
    }
    clips
}

fn vial_nest_guard_rails() -> Part {
    let front = centered_cube(
        "light_exposure_station_vial_nest_front_guard_rail",
        VIAL_PANEL_X - 24.0,
        8.0,
        18.0,
    )
    .translate(0.0, -VIAL_PANEL_Y / 2.0 + 10.0, VIAL_PANEL_Z + 9.0);
    let rear = centered_cube(
        "light_exposure_station_vial_nest_rear_guard_rail",
        VIAL_PANEL_X - 24.0,
        8.0,
        18.0,
    )
    .translate(0.0, VIAL_PANEL_Y / 2.0 - 10.0, VIAL_PANEL_Z + 9.0);
    let left = centered_cube(
        "light_exposure_station_vial_nest_left_row_key",
        9.0,
        VIAL_PANEL_Y - 56.0,
        18.0,
    )
    .translate(-VIAL_PANEL_X / 2.0 + 14.0, 0.0, VIAL_PANEL_Z + 9.0);
    front + rear + left
}

fn light_exposure_witness_coupons() -> Part {
    let panel = centered_cube(
        "light_exposure_station_coupon_panel_body",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z / 2.0);

    panel - coupon_slots()
        + coupon_spring_clips()
        + dose_chip_lands()
        + coupon_datum_pins()
        + coupon_row_column_rails()
}

fn coupon_slots() -> Part {
    let mut slots = Part::empty("light_exposure_station_coupon_slots");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let (x, y) = coupon_center(row, col);
            slots = slots
                + centered_cube(
                    format!("light_exposure_station_coupon_slot_r{row}_c{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    10.0,
                )
                .translate(x, y, COUPON_PANEL_Z - 5.0);
        }
    }
    slots
}

fn coupon_spring_clips() -> Part {
    let mut clips = Part::empty("light_exposure_station_coupon_spring_clips");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let (x, y) = coupon_center(row, col);
            clips = clips
                + centered_cube(
                    format!("light_exposure_station_coupon_clip_r{row}_c{col}"),
                    COUPON_SLOT_X + 12.0,
                    4.0,
                    7.0,
                )
                .translate(x, y + COUPON_SLOT_Y / 2.0 + 5.0, COUPON_PANEL_Z + 3.5);
        }
    }
    clips
}

fn dose_chip_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_dose_chip_lands");
    for index in 0..DOSE_CHIP_LANDS {
        let x = centered_index(index % 4, 4, 92.0);
        let y = if index < 4 {
            -COUPON_PANEL_Y / 2.0 + 26.0
        } else {
            COUPON_PANEL_Y / 2.0 - 26.0
        };
        lands = lands
            + centered_cube(
                format!("light_exposure_station_dose_chip_land_{index}"),
                48.0,
                18.0,
                5.0,
            )
            .translate(x, y, COUPON_PANEL_Z + 2.5);
    }
    lands
}

fn coupon_datum_pins() -> Part {
    let mut datums = Part::empty("light_exposure_station_coupon_datum_pins");
    for (index, (x, y)) in [
        (-COUPON_PANEL_X / 2.0 + 30.0, -COUPON_PANEL_Y / 2.0 + 30.0),
        (COUPON_PANEL_X / 2.0 - 30.0, -COUPON_PANEL_Y / 2.0 + 30.0),
        (-COUPON_PANEL_X / 2.0 + 30.0, COUPON_PANEL_Y / 2.0 - 30.0),
        (COUPON_PANEL_X / 2.0 - 30.0, COUPON_PANEL_Y / 2.0 - 30.0),
    ]
    .into_iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("light_exposure_station_coupon_datum_pin_{index}"),
                5.0,
                16.0,
                24,
            )
            .translate(x, y, COUPON_PANEL_Z + 8.0);
    }
    datums
}

fn coupon_row_column_rails() -> Part {
    let vertical = centered_cube(
        "light_exposure_station_coupon_centerline_rail",
        6.0,
        COUPON_PANEL_Y - 52.0,
        10.0,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z + 5.0);
    let horizontal = centered_cube(
        "light_exposure_station_coupon_timepoint_row_rail",
        COUPON_PANEL_X - 54.0,
        6.0,
        10.0,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z + 5.0);
    vertical + horizontal
}

fn shield_cover_comparison_lanes() -> Part {
    let panel = centered_cube(
        "light_exposure_station_comparison_lane_panel",
        COMPARISON_PANEL_X,
        COMPARISON_PANEL_Y,
        COMPARISON_PANEL_Z,
    )
    .translate(0.0, 0.0, COMPARISON_PANEL_Z / 2.0);

    panel - comparison_lane_sockets() - comparison_sample_windows()
        + comparison_lane_dividers()
        + comparison_cover_stops()
        + comparison_lane_keys()
}

fn comparison_lane_sockets() -> Part {
    let mut sockets = Part::empty("light_exposure_station_comparison_lane_sockets");
    for lane in 0..COMPARISON_LANES {
        sockets = sockets
            + centered_cube(
                format!(
                    "light_exposure_station_{}_removable_cover_socket",
                    COMPARISON_LANE_NAMES[lane]
                ),
                COMPARISON_LANE_X,
                COMPARISON_LANE_Y,
                12.0,
            )
            .translate(comparison_lane_x(lane), 0.0, COMPARISON_PANEL_Z - 6.0);
    }
    sockets
}

fn comparison_sample_windows() -> Part {
    let mut windows = Part::empty("light_exposure_station_comparison_lane_sample_windows");
    for lane in 0..COMPARISON_LANES {
        for window in 0..COVER_SAMPLE_WINDOWS_PER_LANE {
            windows = windows
                + centered_cube(
                    format!(
                        "light_exposure_station_{}_sample_window_{window}",
                        COMPARISON_LANE_NAMES[lane]
                    ),
                    58.0,
                    28.0,
                    8.0,
                )
                .translate(
                    comparison_lane_x(lane),
                    centered_index(window, COVER_SAMPLE_WINDOWS_PER_LANE, 48.0),
                    COMPARISON_PANEL_Z - 4.0,
                );
        }
    }
    windows
}

fn comparison_lane_dividers() -> Part {
    let mut dividers = Part::empty("light_exposure_station_comparison_lane_dividers");
    for index in 0..(COMPARISON_LANES - 1) {
        let x = (comparison_lane_x(index) + comparison_lane_x(index + 1)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("light_exposure_station_comparison_lane_divider_{index}"),
                9.0,
                COMPARISON_PANEL_Y - 28.0,
                24.0,
            )
            .translate(x, 0.0, COMPARISON_PANEL_Z + 12.0);
    }
    dividers
}

fn comparison_cover_stops() -> Part {
    let mut stops = Part::empty("light_exposure_station_comparison_cover_stops");
    for lane in 0..COMPARISON_LANES {
        let x = comparison_lane_x(lane);
        let height = 16.0 + lane as f64 * 10.0;
        stops = stops
            + centered_cube(
                format!(
                    "light_exposure_station_{}_rear_cover_stop",
                    COMPARISON_LANE_NAMES[lane]
                ),
                COMPARISON_LANE_X,
                9.0,
                height,
            )
            .translate(
                x,
                COMPARISON_LANE_Y / 2.0 - 10.0,
                COMPARISON_PANEL_Z + height / 2.0,
            )
            + centered_cube(
                format!(
                    "light_exposure_station_{}_front_cover_stop",
                    COMPARISON_LANE_NAMES[lane]
                ),
                COMPARISON_LANE_X,
                9.0,
                height,
            )
            .translate(
                x,
                -COMPARISON_LANE_Y / 2.0 + 10.0,
                COMPARISON_PANEL_Z + height / 2.0,
            );
    }
    stops
}

fn comparison_lane_keys() -> Part {
    let mut keys = Part::empty("light_exposure_station_comparison_lane_keys");
    for lane in 0..COMPARISON_LANES {
        keys = keys
            + centered_cube(
                format!(
                    "light_exposure_station_{}_keyed_label_land",
                    COMPARISON_LANE_NAMES[lane]
                ),
                76.0,
                18.0,
                5.0,
            )
            .translate(
                comparison_lane_x(lane),
                -COMPARISON_PANEL_Y / 2.0 + 22.0,
                COMPARISON_PANEL_Z + 2.5,
            );
    }
    keys
}

fn timed_token_lands() -> Part {
    let panel = centered_cube(
        "light_exposure_station_timed_token_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    )
    .translate(0.0, 0.0, TOKEN_PANEL_Z / 2.0);

    panel - token_recesses()
        + token_retainer_clips()
        + clock_reference_lands()
        + token_route_arrows()
}

fn token_recesses() -> Part {
    let mut recesses = Part::empty("light_exposure_station_timed_token_recesses");
    for (index, label) in TIMEPOINT_LABELS.into_iter().enumerate() {
        recesses = recesses
            + centered_cylinder(
                format!("light_exposure_station_{label}_token_recess"),
                TOKEN_LAND_D / 2.0,
                TOKEN_LAND_DEPTH + 0.4,
                36,
            )
            .translate(
                centered_index(index, TIMEPOINT_TOKENS, TOKEN_PITCH_X),
                30.0,
                TOKEN_PANEL_Z - TOKEN_LAND_DEPTH / 2.0 + 0.2,
            );
    }
    recesses
}

fn token_retainer_clips() -> Part {
    let mut clips = Part::empty("light_exposure_station_timed_token_retainer_clips");
    for (index, label) in TIMEPOINT_LABELS.into_iter().enumerate() {
        clips = clips
            + centered_cube(
                format!("light_exposure_station_{label}_token_retainer_clip"),
                34.0,
                7.0,
                7.0,
            )
            .translate(
                centered_index(index, TIMEPOINT_TOKENS, TOKEN_PITCH_X),
                -3.0,
                TOKEN_PANEL_Z + 3.5,
            );
    }
    clips
}

fn clock_reference_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_clock_reference_lands");
    for index in 0..CLOCK_REFERENCE_LANDS {
        lands = lands
            + centered_cube(
                format!("light_exposure_station_clock_reference_land_{index}"),
                76.0,
                28.0,
                5.0,
            )
            .translate(
                centered_index(index, CLOCK_REFERENCE_LANDS, 100.0),
                -TOKEN_PANEL_Y / 2.0 + 38.0,
                TOKEN_PANEL_Z + 2.5,
            );
    }
    lands
}

fn token_route_arrows() -> Part {
    let start = centered_cube(
        "light_exposure_station_token_route_arrow_start",
        TOKEN_PANEL_X - 70.0,
        5.0,
        5.0,
    )
    .translate(0.0, 78.0, TOKEN_PANEL_Z + 2.5);
    let end = centered_cube(
        "light_exposure_station_token_route_arrow_head",
        18.0,
        18.0,
        5.0,
    )
    .translate(TOKEN_PANEL_X / 2.0 - 48.0, 78.0, TOKEN_PANEL_Z + 2.5);
    start + end
}

fn temperature_logger_pocket() -> Part {
    let block = centered_cube(
        "light_exposure_station_temperature_logger_block",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);

    block - logger_slot() - thermowell_pockets() - logger_cable_channel()
        + logger_seal_tab_lands()
        + logger_probe_guard_rails()
}

fn logger_slot() -> Part {
    centered_cube(
        "light_exposure_station_temperature_logger_slot",
        LOGGER_SLOT_X,
        LOGGER_SLOT_Y,
        LOGGER_SLOT_DEPTH + 0.4,
    )
    .translate(0.0, 32.0, LOGGER_PANEL_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.2)
}

fn thermowell_pockets() -> Part {
    let mut wells = Part::empty("light_exposure_station_thermowell_pockets");
    for index in 0..THERMOWELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("light_exposure_station_thermowell_pocket_{index}"),
                THERMOWELL_D / 2.0,
                25.0,
                24,
            )
            .translate(
                centered_index(index, THERMOWELL_COUNT, THERMOWELL_PITCH_X),
                -58.0,
                LOGGER_PANEL_Z - 12.5,
            );
    }
    wells
}

fn logger_cable_channel() -> Part {
    centered_cube(
        "light_exposure_station_logger_cable_channel",
        22.0,
        LOGGER_PANEL_Y,
        11.0,
    )
    .translate(LOGGER_PANEL_X / 2.0 - 40.0, 0.0, LOGGER_PANEL_Z - 5.5)
}

fn logger_seal_tab_lands() -> Part {
    let mut tabs = Part::empty("light_exposure_station_logger_seal_tab_lands");
    for (index, (x, y)) in [
        (-LOGGER_SLOT_X / 2.0 - 24.0, 32.0),
        (LOGGER_SLOT_X / 2.0 + 24.0, 32.0),
        (-LOGGER_SLOT_X / 2.0 - 24.0, -26.0),
        (LOGGER_SLOT_X / 2.0 + 24.0, -26.0),
    ]
    .into_iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cube(
                format!("light_exposure_station_logger_seal_tab_land_{index}"),
                32.0,
                13.0,
                6.0,
            )
            .translate(x, y, LOGGER_PANEL_Z + 3.0);
    }
    tabs
}

fn logger_probe_guard_rails() -> Part {
    let front = centered_cube(
        "light_exposure_station_logger_probe_front_guard",
        LOGGER_PANEL_X - 36.0,
        8.0,
        16.0,
    )
    .translate(0.0, -LOGGER_PANEL_Y / 2.0 + 12.0, LOGGER_PANEL_Z + 8.0);
    let rear = centered_cube(
        "light_exposure_station_logger_probe_rear_guard",
        LOGGER_PANEL_X - 36.0,
        8.0,
        16.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 - 12.0, LOGGER_PANEL_Z + 8.0);
    front + rear
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "light_exposure_station_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(0.0, 0.0, TRACE_PANEL_Z / 2.0);

    panel + barcode_lands() + certificate_lands() + rfid_lands() + custody_seal_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let col = index % 4;
        let row = index / 4;
        lands = lands
            + centered_cube(
                format!("light_exposure_station_barcode_land_{index}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(
                centered_index(col, 4, 86.0),
                38.0 - row as f64 * 36.0,
                TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_certificate_lands");
    for index in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("light_exposure_station_certificate_land_{index}"),
                CERTIFICATE_LAND_X,
                CERTIFICATE_LAND_Y,
                4.0,
            )
            .translate(
                centered_index(index, CERTIFICATE_LANDS, 118.0),
                -TRACE_PANEL_Y / 2.0 + 30.0,
                TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_rfid_lands");
    for index in 0..RFID_LANDS {
        lands = lands
            + (centered_cylinder(
                format!("light_exposure_station_rfid_outer_land_{index}"),
                RFID_LAND_D / 2.0,
                4.0,
                48,
            ) - centered_cylinder(
                format!("light_exposure_station_rfid_inner_relief_{index}"),
                RFID_LAND_D / 2.0 - 6.0,
                5.0,
                48,
            ))
            .translate(
                -TRACE_PANEL_X / 2.0 + 38.0 + index as f64 * 42.0,
                TRACE_PANEL_Y / 2.0 - 34.0,
                TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn custody_seal_slots() -> Part {
    let mut slots = Part::empty("light_exposure_station_traceability_custody_seal_slots");
    for index in 0..6 {
        slots = slots
            + centered_cube(
                format!("light_exposure_station_traceability_custody_seal_slot_{index}"),
                28.0,
                6.0,
                7.0,
            )
            .translate(
                TRACE_PANEL_X / 2.0 - 44.0,
                centered_index(index, 6, 20.0),
                TRACE_PANEL_Z + 3.5,
            );
    }
    slots
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "light_exposure_station_release_hold_reject_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(0.0, 0.0, STATUS_PANEL_Z / 2.0);

    panel - status_slot_recesses() + status_lane_dividers() + status_header_lands()
}

fn status_slot_recesses() -> Part {
    let mut recesses = Part::empty("light_exposure_station_status_slot_recesses");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            recesses = recesses
                + centered_cube(
                    format!(
                        "light_exposure_station_{}_status_slot_{slot}",
                        STATUS_NAMES[lane]
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    12.0,
                )
                .translate(
                    centered_index(slot, STATUS_SLOTS_PER_LANE, 88.0),
                    status_lane_y(lane),
                    STATUS_PANEL_Z - 6.0,
                );
        }
    }
    recesses
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("light_exposure_station_status_lane_dividers");
    for index in 0..(STATUS_LANES - 1) {
        dividers = dividers
            + centered_cube(
                format!("light_exposure_station_status_lane_divider_{index}"),
                STATUS_PANEL_X - 36.0,
                7.0,
                18.0,
            )
            .translate(
                0.0,
                (status_lane_y(index) + status_lane_y(index + 1)) / 2.0,
                STATUS_PANEL_Z + 9.0,
            );
    }
    dividers
}

fn status_header_lands() -> Part {
    let mut headers = Part::empty("light_exposure_station_status_header_lands");
    for lane in 0..STATUS_LANES {
        headers = headers
            + centered_cube(
                format!("light_exposure_station_{}_header_land", STATUS_NAMES[lane]),
                68.0,
                16.0,
                5.0,
            )
            .translate(
                -STATUS_PANEL_X / 2.0 + 42.0,
                status_lane_y(lane),
                STATUS_PANEL_Z + 2.5,
            );
    }
    headers
}

fn clean_used_segregation() -> Part {
    let left_wall = centered_cube(
        "light_exposure_station_clean_used_left_wall_segment",
        (SEGREGATION_WALL_X - SEGREGATION_GATE_X) / 2.0,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(
        -SEGREGATION_WALL_X / 4.0 - SEGREGATION_GATE_X / 4.0,
        0.0,
        SEGREGATION_WALL_Z / 2.0,
    );
    let right_wall = centered_cube(
        "light_exposure_station_clean_used_right_wall_segment",
        (SEGREGATION_WALL_X - SEGREGATION_GATE_X) / 2.0,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(
        SEGREGATION_WALL_X / 4.0 + SEGREGATION_GATE_X / 4.0,
        0.0,
        SEGREGATION_WALL_Z / 2.0,
    );
    let pass_gate = centered_cube(
        "light_exposure_station_clean_used_one_way_gate_land",
        SEGREGATION_GATE_X,
        SEGREGATION_WALL_Y + 26.0,
        12.0,
    )
    .translate(0.0, 0.0, 6.0);

    left_wall + right_wall + pass_gate + clean_used_token_lands() + segregation_label_lands()
}

fn clean_used_token_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_clean_used_token_lands");
    for index in 0..CLEAN_USED_TOKEN_LANDS {
        let side_y = if index < CLEAN_USED_TOKEN_LANDS / 2 {
            34.0
        } else {
            -34.0
        };
        lands = lands
            + centered_cube(
                format!("light_exposure_station_clean_used_token_land_{index}"),
                46.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(index % 5, 5, 66.0),
                side_y,
                SEGREGATION_WALL_Z + 2.5,
            );
    }
    lands
}

fn segregation_label_lands() -> Part {
    let clean = centered_cube(
        "light_exposure_station_clean_side_label_land",
        140.0,
        22.0,
        5.0,
    )
    .translate(
        -SEGREGATION_WALL_X / 2.0 + 128.0,
        34.0,
        SEGREGATION_WALL_Z + 2.5,
    );
    let used = centered_cube(
        "light_exposure_station_used_side_label_land",
        140.0,
        22.0,
        5.0,
    )
    .translate(
        SEGREGATION_WALL_X / 2.0 - 128.0,
        -34.0,
        SEGREGATION_WALL_Z + 2.5,
    );
    clean + used
}

fn closed_transfer_seal_bulkhead() -> Part {
    let wall = centered_cube(
        "light_exposure_station_closed_transfer_bulkhead_wall",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_Z / 2.0);

    wall - transfer_port_bores()
        + transfer_port_collars()
        + seal_witness_lands()
        + bulkhead_foot_lugs()
}

fn transfer_port_bores() -> Part {
    let mut bores = Part::empty("light_exposure_station_transfer_port_bores");
    for index in 0..TRANSFER_PORTS {
        let (x, z) = transfer_port_center(index);
        bores = bores
            + centered_cylinder(
                format!("light_exposure_station_transfer_port_bore_{index}"),
                TRANSFER_PORT_D / 2.0,
                BULKHEAD_Y + 6.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, z);
    }
    bores
}

fn transfer_port_collars() -> Part {
    let mut collars = Part::empty("light_exposure_station_transfer_port_collars");
    for index in 0..TRANSFER_PORTS {
        let (x, z) = transfer_port_center(index);
        let ring = centered_cylinder(
            format!("light_exposure_station_transfer_port_front_collar_{index}"),
            TRANSFER_COLLAR_D / 2.0,
            8.0,
            48,
        ) - centered_cylinder(
            format!("light_exposure_station_transfer_port_front_relief_{index}"),
            TRANSFER_PORT_D / 2.0,
            9.0,
            40,
        );
        collars = collars
            + ring
                .rotate(90.0, 0.0, 0.0)
                .translate(x, -BULKHEAD_Y / 2.0 - 4.0, z)
            + centered_cylinder(
                format!("light_exposure_station_transfer_port_rear_collar_{index}"),
                TRANSFER_COLLAR_D / 2.0,
                8.0,
                48,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_Y / 2.0 + 4.0, z);
    }
    collars
}

fn seal_witness_lands() -> Part {
    let mut lands = Part::empty("light_exposure_station_bulkhead_seal_witness_lands");
    for index in 0..SEAL_WITNESS_LANDS {
        lands = lands
            + centered_cube(
                format!("light_exposure_station_bulkhead_seal_witness_land_{index}"),
                44.0,
                7.0,
                16.0,
            )
            .translate(
                centered_index(index, SEAL_WITNESS_LANDS, 54.0),
                -BULKHEAD_Y / 2.0 - 5.0,
                BULKHEAD_Z - 16.0,
            );
    }
    lands
}

fn bulkhead_foot_lugs() -> Part {
    let left = centered_cube(
        "light_exposure_station_bulkhead_left_foot_lug",
        62.0,
        84.0,
        16.0,
    )
    .translate(-BULKHEAD_X / 2.0 + 40.0, 0.0, 8.0);
    let right = centered_cube(
        "light_exposure_station_bulkhead_right_foot_lug",
        62.0,
        84.0,
        16.0,
    )
    .translate(BULKHEAD_X / 2.0 - 40.0, 0.0, 8.0);
    left + right
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "light_exposure_station_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let right_post = centered_cube(
        "light_exposure_station_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let beam = centered_cube(
        "light_exposure_station_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);

    left_post + right_post + beam + camera_pods() + amber_safe_led_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("light_exposure_station_evidence_camera_pods");
    for index in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("light_exposure_station_camera_pod_{index}"),
                56.0,
                42.0,
                28.0,
            )
            .translate(
                centered_index(index, CAMERA_PODS, CAMERA_PITCH_X),
                -BRIDGE_BEAM_Y / 2.0 - 15.0,
                CAMERA_CLEARANCE_Z,
            ) + centered_cylinder(
                format!("light_exposure_station_camera_lens_guard_{index}"),
                12.0,
                8.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, CAMERA_PODS, CAMERA_PITCH_X),
                -BRIDGE_BEAM_Y / 2.0 - 40.0,
                CAMERA_CLEARANCE_Z,
            );
    }
    pods
}

fn amber_safe_led_bars() -> Part {
    let mut bars = Part::empty("light_exposure_station_amber_safe_led_bars");
    for index in 0..AMBER_LED_BARS {
        bars = bars
            + centered_cube(
                format!("light_exposure_station_amber_safe_led_bar_{index}"),
                BRIDGE_SPAN_X * 0.42,
                12.0,
                10.0,
            )
            .translate(
                centered_index(index, AMBER_LED_BARS, BRIDGE_SPAN_X * 0.48),
                BRIDGE_BEAM_Y / 2.0 + 12.0,
                BRIDGE_POST_Z - 30.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = keepout_frame(
        "light_exposure_station_front_robot_keepout",
        DECK_X - 160.0,
        FRONT_ROBOT_KEEP_OUT_Y,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let rear_service = keepout_frame(
        "light_exposure_station_rear_service_keepout",
        DECK_X - 160.0,
        REAR_SERVICE_KEEP_OUT_Y,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let left_service = keepout_frame(
        "light_exposure_station_left_service_keepout",
        LEFT_SERVICE_KEEP_OUT_X,
        DECK_Y - 170.0,
    )
    .translate(
        -DECK_X / 2.0 - LEFT_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let right_service = keepout_frame(
        "light_exposure_station_right_service_keepout",
        RIGHT_SERVICE_KEEP_OUT_X,
        DECK_Y - 170.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let cover_lift = centered_cube(
        "light_exposure_station_cover_lift_clearance_gauge",
        COUPON_PANEL_X + VIAL_PANEL_X + 220.0,
        34.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, 352.0, COVER_LIFT_CLEARANCE_Z);

    front_robot + rear_service + left_service + right_service + cover_lift
}

fn keepout_frame(name: &str, x: f64, y: f64) -> Part {
    let front = centered_cube(
        format!("{name}_front_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -y / 2.0 + KEEP_OUT_RAIL_W / 2.0, 0.0);
    let rear = centered_cube(
        format!("{name}_rear_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, y / 2.0 - KEEP_OUT_RAIL_W / 2.0, 0.0);
    let left = centered_cube(
        format!("{name}_left_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-x / 2.0 + KEEP_OUT_RAIL_W / 2.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{name}_right_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(x / 2.0 - KEEP_OUT_RAIL_W / 2.0, 0.0, 0.0);
    let label = centered_cube(
        format!("{name}_label_land"),
        x.min(220.0),
        18.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -y / 2.0 + 20.0, KEEP_OUT_RAIL_Z);
    front + rear + left + right + label
}

fn top_recess(name: impl Into<String>, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(center.0, center.1, DECK_Z - depth / 2.0 + 0.1)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let pad = centered_cylinder(format!("{name}_pad"), 14.0, 3.0, 48);
    let cross_x = centered_cube(format!("{name}_cross_x_cut"), 23.0, 3.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y_cut"), 3.0, 23.0, 4.0);
    pad - cross_x - cross_y
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn vial_row_y(row: usize) -> f64 {
    if row == 0 {
        VIAL_ROW_PITCH_Y / 2.0
    } else {
        -VIAL_ROW_PITCH_Y / 2.0
    }
}

fn coupon_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COUPON_COLS, COUPON_PITCH_X),
        -centered_index(row, COUPON_ROWS, COUPON_PITCH_Y),
    )
}

fn comparison_lane_x(lane: usize) -> f64 {
    centered_index(lane, COMPARISON_LANES, COMPARISON_LANE_PITCH_X)
}

fn status_lane_y(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y)
}

fn transfer_port_center(index: usize) -> (f64, f64) {
    let col = index % TRANSFER_PORT_COLS;
    let row = index / TRANSFER_PORT_COLS;
    (
        centered_index(col, TRANSFER_PORT_COLS, TRANSFER_PORT_PITCH_X),
        BULKHEAD_Z / 2.0
            + centered_index(
                row,
                TRANSFER_PORTS / TRANSFER_PORT_COLS,
                TRANSFER_PORT_PITCH_Z,
            ),
    )
}

fn layout_envelopes() -> [ModuleEnvelope; 8] {
    [
        ModuleEnvelope {
            name: "amber_clear_vial_surrogate_nests",
            center: VIAL_CENTER,
            x: VIAL_PANEL_X,
            y: VIAL_PANEL_Y,
        },
        ModuleEnvelope {
            name: "light_exposure_witness_coupons",
            center: COUPON_CENTER,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        ModuleEnvelope {
            name: "shield_cover_comparison_lanes",
            center: COMPARISON_CENTER,
            x: COMPARISON_PANEL_X,
            y: COMPARISON_PANEL_Y,
        },
        ModuleEnvelope {
            name: "timed_token_lands",
            center: TOKEN_CENTER,
            x: TOKEN_PANEL_X,
            y: TOKEN_PANEL_Y,
        },
        ModuleEnvelope {
            name: "temperature_logger_pocket",
            center: LOGGER_CENTER,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        ModuleEnvelope {
            name: "barcode_certificate_lands",
            center: TRACE_CENTER,
            x: TRACE_PANEL_X,
            y: TRACE_PANEL_Y,
        },
        ModuleEnvelope {
            name: "release_hold_reject_lanes",
            center: STATUS_CENTER,
            x: STATUS_PANEL_X,
            y: STATUS_PANEL_Y,
        },
        ModuleEnvelope {
            name: "closed_transfer_seal_bulkhead",
            center: BULKHEAD_CENTER,
            x: BULKHEAD_X,
            y: BULKHEAD_Y,
        },
    ]
}

fn assert_layout() {
    let envelopes = layout_envelopes();
    for envelope in envelopes {
        assert!(
            envelope.fits_on_deck(),
            "{} exceeds light exposure station envelope",
            envelope.name
        );
    }
    for i in 0..envelopes.len() {
        for j in i + 1..envelopes.len() {
            assert!(
                !envelopes[i].overlaps(envelopes[j]),
                "{} overlaps {}",
                envelopes[i].name,
                envelopes[j].name
            );
        }
    }
    assert_eq!(VIAL_COUNT, AMBER_VIALS + CLEAR_VIALS);
    assert_eq!(COUPON_COUNT, COUPON_ROWS * COUPON_COLS);
    assert!(COUPON_SLOT_X < COUPON_PITCH_X - 10.0);
    assert!(COUPON_SLOT_Y < COUPON_PITCH_Y - 10.0);
    assert!(comparison_lane_x(0) < comparison_lane_x(1));
    assert!(comparison_lane_x(1) < comparison_lane_x(2));
    assert!(TOKEN_PITCH_X * (TIMEPOINT_TOKENS as f64 - 1.0) + TOKEN_LAND_D < TOKEN_PANEL_X - 35.0);
    assert!(THERMOWELL_D + 8.0 < THERMOWELL_PITCH_X);
    assert!(status_lane_y(0) < status_lane_y(1));
    assert!(status_lane_y(1) < status_lane_y(2));
    assert!(SEGREGATION_CENTER.1 < COMPARISON_CENTER.1 - COMPARISON_PANEL_Y / 2.0);
    assert!(SEGREGATION_CENTER.1 > TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0);
    assert!(TRANSFER_PORTS % TRANSFER_PORT_COLS == 0);
    assert!(
        TRANSFER_PORT_PITCH_X * (TRANSFER_PORT_COLS as f64 - 1.0) + TRANSFER_COLLAR_D
            < BULKHEAD_X - 36.0
    );
    assert!(CAMERA_CLEARANCE_Z > VIAL_PANEL_Z + 70.0);
    assert!(BRIDGE_SPAN_X + BRIDGE_POST_X < DECK_X);
    assert!(COVER_LIFT_CLEARANCE_Z > BRIDGE_POST_Z + BRIDGE_BEAM_Z);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "amber_vial_nests",
            "clear_vial_nests",
            "light_exposure_witness_coupons",
            "shield_cover_comparison_lanes",
            "timed_token_lands",
            "temperature_logger_pocket",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "closed_transfer_seal_bulkhead",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 12);
    }

    #[test]
    fn layout_constraints_hold_without_primary_collisions() {
        assert_layout();
        let envelopes = layout_envelopes();
        assert_eq!(envelopes.len(), 8);
        assert!(envelopes.iter().all(|envelope| envelope.fits_on_deck()));
    }

    #[test]
    fn vial_coupon_and_light_witness_capacity_match_scope() {
        assert_eq!(VIAL_ROWS, 2);
        assert_eq!(VIAL_COLS, 5);
        assert_eq!(VIAL_COUNT, 10);
        assert_eq!(AMBER_VIALS, 5);
        assert_eq!(CLEAR_VIALS, 5);
        assert_eq!(COUPON_COUNT, 24);
        assert!(DOSE_CHIP_LANDS >= COMPARISON_LANES * 2);
        assert_eq!(COUPON_DATUMS, 4);
    }

    #[test]
    fn comparison_time_and_logger_evidence_are_traceable() {
        assert_eq!(
            COMPARISON_LANE_NAMES,
            ["clear_lane", "amber_lane", "opaque_cover_lane"]
        );
        assert_eq!(COMPARISON_LANES, 3);
        assert_eq!(TIMEPOINT_TOKENS, 6);
        assert_eq!(CLOCK_REFERENCE_LANDS, 3);
        assert_eq!(THERMOWELL_COUNT, 6);
        assert!(LOGGER_SLOT_X > 2.0 * TOKEN_LAND_D);
        assert_eq!(LOGGER_SEAL_TABS, 4);
    }

    #[test]
    fn traceability_disposition_and_segregation_are_separated() {
        assert_eq!(
            EVIDENCE_LANDS,
            BARCODE_LANDS + CERTIFICATE_LANDS + RFID_LANDS
        );
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE * STATUS_LANES, 12);
        assert_eq!(CLEAN_USED_TOKEN_LANDS, 10);
        assert!(TRACE_CENTER.0 + TRACE_PANEL_X / 2.0 < STATUS_CENTER.0 - STATUS_PANEL_X / 2.0);
        assert!(SEGREGATION_CENTER.1 < COMPARISON_CENTER.1 - COMPARISON_PANEL_Y / 2.0);
        assert!(SEGREGATION_CENTER.1 > TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0);
    }

    #[test]
    fn closed_transfer_bridge_and_keepouts_have_clearance() {
        assert_eq!(TRANSFER_PORTS, 6);
        assert_eq!(SEAL_WITNESS_LANDS, 4);
        assert_eq!(CAMERA_PODS, 5);
        assert_eq!(AMBER_LED_BARS, 2);
        assert!(CAMERA_CLEARANCE_Z > BULKHEAD_Z);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 360.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 250.0);
        assert!(LEFT_SERVICE_KEEP_OUT_X >= 220.0);
        assert!(RIGHT_SERVICE_KEEP_OUT_X >= 250.0);
        assert!(COVER_LIFT_CLEARANCE_Z > CAMERA_CLEARANCE_Z);
    }

    #[test]
    fn all_part_groups_construct_for_export() {
        let groups = [
            base_leak_tray(),
            amber_clear_vial_surrogate_nests(),
            light_exposure_witness_coupons(),
            shield_cover_comparison_lanes(),
            timed_token_lands(),
            temperature_logger_pocket(),
            barcode_certificate_lands(),
            release_hold_reject_lanes(),
            clean_used_segregation(),
            closed_transfer_seal_bulkhead(),
            evidence_bridge(),
            robot_service_keepout_gauges(),
        ];
        assert_eq!(groups.len(), OUTPUTS.len() - 1);
    }
}
