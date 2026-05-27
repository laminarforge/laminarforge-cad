use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed ECM/coating reagent thaw, dwell, and uniformity validation station.
//
// Intent:
// - Validate ECM/coating reagent transition from chilled storage through thaw,
//   dwell timing, temperature hold, and homogeneity checks before automated
//   coating starts.
// - Keep chilled and warm vial nests, dwell token rails, probe wells, mixing
//   witness coupons, viscosity/osmolality pockets, light protection, sterile
//   connector bulkhead, barcode/status lands, waste segregation, evidence
//   bridge, and robot/service keepouts mechanically explicit.
// - This is interface and fixture CAD only. It is not a coating protocol,
//   acceptance criterion, stability claim, or sterility validation.

const OUTPUTS: [&str; 13] = [
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_base_leak_tray_deck.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_chilled_vial_nest.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_warm_hold_vial_nest.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_dwell_token_rails.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_temperature_probe_wells.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_mixing_witness_coupon_pockets.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_viscosity_osmolality_sample_pockets.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_light_protection_cover_envelope.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_sterile_connector_bulkhead.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_barcode_status_lands.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_waste_segregation.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_assembly.stl",
];

const FEATURE_NAMES: [&str; 12] = [
    "base_leak_tray_deck",
    "chilled_vial_nest",
    "warm_hold_vial_nest",
    "dwell_token_rails",
    "temperature_probe_wells",
    "mixing_witness_coupon_pockets",
    "viscosity_osmolality_sample_pockets",
    "light_protection_cover_envelope",
    "sterile_connector_bulkhead",
    "barcode_status_lands",
    "waste_segregation",
    "evidence_bridge_robot_service_keepouts",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_D: f64 = 16.0;
const DATUM_TARGETS: usize = 6;

const CHILLED_CENTER: (f64, f64) = (-430.0, 225.0);
const CHILLED_X: f64 = 360.0;
const CHILLED_Y: f64 = 250.0;
const CHILLED_Z: f64 = 58.0;
const CHILLED_ROWS: usize = 3;
const CHILLED_COLS: usize = 4;
const CHILLED_VIALS: usize = CHILLED_ROWS * CHILLED_COLS;
const CHILLED_WELL_D: f64 = 24.0;
const CHILLED_PITCH_X: f64 = 62.0;
const CHILLED_PITCH_Y: f64 = 58.0;
const ICE_PACK_SLOTS: usize = 4;

const WARM_CENTER: (f64, f64) = (-30.0, 225.0);
const WARM_X: f64 = 360.0;
const WARM_Y: f64 = 250.0;
const WARM_Z: f64 = 64.0;
const WARM_ROWS: usize = 2;
const WARM_COLS: usize = 4;
const WARM_VIALS: usize = WARM_ROWS * WARM_COLS;
const WARM_WELL_D: f64 = 26.0;
const WARM_PITCH_X: f64 = 72.0;
const WARM_PITCH_Y: f64 = 72.0;
const HEAT_TRACE_CHANNELS: usize = 4;

const TOKEN_CENTER: (f64, f64) = (385.0, 235.0);
const TOKEN_X: f64 = 340.0;
const TOKEN_Y: f64 = 230.0;
const TOKEN_Z: f64 = 28.0;
const DWELL_RAILS: usize = 4;
const TOKENS_PER_RAIL: usize = 6;
const TOKEN_SLOTS: usize = DWELL_RAILS * TOKENS_PER_RAIL;
const TOKEN_SLOT_X: f64 = 38.0;
const TOKEN_SLOT_Y: f64 = 20.0;
const TOKEN_RAIL_PITCH_Y: f64 = 46.0;

const PROBE_CENTER: (f64, f64) = (-450.0, -55.0);
const PROBE_X: f64 = 315.0;
const PROBE_Y: f64 = 170.0;
const PROBE_Z: f64 = 40.0;
const PROBE_WELLS: usize = 8;
const PROBE_WELL_D: f64 = 8.4;
const PROBE_PITCH_X: f64 = 36.0;
const PROBE_CABLE_CLIPS: usize = 5;

const COUPON_CENTER: (f64, f64) = (-95.0, -60.0);
const COUPON_X: f64 = 330.0;
const COUPON_Y: f64 = 180.0;
const COUPON_Z: f64 = 28.0;
const COUPON_ROWS: usize = 2;
const COUPON_COLS: usize = 5;
const MIXING_COUPONS: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 26.0;
const COUPON_PITCH_X: f64 = 58.0;
const COUPON_PITCH_Y: f64 = 58.0;

const SAMPLE_CENTER: (f64, f64) = (275.0, -60.0);
const SAMPLE_X: f64 = 300.0;
const SAMPLE_Y: f64 = 180.0;
const SAMPLE_Z: f64 = 36.0;
const VISCOSITY_POCKETS: usize = 4;
const OSMOLALITY_POCKETS: usize = 4;
const SAMPLE_POCKET_D: f64 = 18.0;
const SAMPLE_PITCH_X: f64 = 58.0;

const COVER_CENTER: (f64, f64) = (-230.0, 100.0);
const COVER_X: f64 = 825.0;
const COVER_Y: f64 = 430.0;
const COVER_Z: f64 = 145.0;
const COVER_POST_X: f64 = 22.0;
const COVER_POST_Y: f64 = 22.0;
const COVER_BEAM_Z: f64 = 18.0;
const COVER_INSPECTION_WINDOWS: usize = 4;

const BULKHEAD_CENTER: (f64, f64) = (505.0, -50.0);
const BULKHEAD_X: f64 = 54.0;
const BULKHEAD_Y: f64 = 360.0;
const BULKHEAD_Z: f64 = 190.0;
const CONNECTOR_PORTS: usize = 6;
const CONNECTOR_PORT_D: f64 = 25.0;
const CONNECTOR_COLLAR_D: f64 = 43.0;
const BULKHEAD_PORT_PITCH_Y: f64 = 48.0;

const STATUS_CENTER: (f64, f64) = (420.0, -285.0);
const STATUS_X: f64 = 355.0;
const STATUS_Y: f64 = 112.0;
const STATUS_Z: f64 = 8.0;
const BARCODE_LANDS: usize = 10;
const STATUS_LANES: usize = 4;

const WASTE_CENTER: (f64, f64) = (-300.0, -285.0);
const WASTE_X: f64 = 455.0;
const WASTE_Y: f64 = 130.0;
const WASTE_Z: f64 = 46.0;
const WASTE_BINS: usize = 3;
const WASTE_PORTS: usize = 6;

const EVIDENCE_CENTER: (f64, f64) = (0.0, -10.0);
const EVIDENCE_BRIDGE_X: f64 = 1160.0;
const EVIDENCE_BRIDGE_Y: f64 = 54.0;
const EVIDENCE_BRIDGE_Z: f64 = 225.0;
const EVIDENCE_CAMERAS: usize = 4;
const SERVICE_LIGHTS: usize = 3;

const ROBOT_KEEP_OUT_X: f64 = 980.0;
const ROBOT_KEEP_OUT_Y: f64 = 610.0;
const ROBOT_KEEP_OUT_Z: f64 = 155.0;
const FRONT_ROBOT_APPROACH: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 285.0;
const SIDE_SERVICE_CLEARANCE: f64 = 240.0;
const COVER_LIFT_CLEARANCE: f64 = 310.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let chilled = chilled_vial_nest();
    export(OUTPUTS[1], &chilled);

    let warm = warm_hold_vial_nest();
    export(OUTPUTS[2], &warm);

    let tokens = dwell_token_rails();
    export(OUTPUTS[3], &tokens);

    let probes = temperature_probe_wells();
    export(OUTPUTS[4], &probes);

    let coupons = mixing_witness_coupon_pockets();
    export(OUTPUTS[5], &coupons);

    let samples = viscosity_osmolality_sample_pockets();
    export(OUTPUTS[6], &samples);

    let cover = light_protection_cover_envelope();
    export(OUTPUTS[7], &cover);

    let bulkhead = sterile_connector_bulkhead();
    export(OUTPUTS[8], &bulkhead);

    let status = barcode_status_lands();
    export(OUTPUTS[9], &status);

    let waste = waste_segregation();
    export(OUTPUTS[10], &waste);

    let evidence = evidence_bridge_robot_service_keepouts();
    export(OUTPUTS[11], &evidence);

    let assembly = base
        + chilled
        + warm
        + tokens
        + probes
        + coupons
        + samples
        + cover
        + bulkhead
        + status
        + waste
        + evidence;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed ECM/coating reagent thaw dwell uniformity station:");
    println!("  Deck:                       {DECK_X:.0}mm x {DECK_Y:.0}mm leak tray with {DATUM_TARGETS} datum targets and {DRAIN_D:.0}mm drain");
    println!("  Thermal staging:            {CHILLED_VIALS} chilled vial wells, {ICE_PACK_SLOTS} cold pack slots, {WARM_VIALS} warm hold wells, {HEAT_TRACE_CHANNELS} heat trace channels");
    println!("  Dwell controls:             {DWELL_RAILS} token rails with {TOKEN_SLOTS} discrete time-token slots");
    println!("  Measurement witnesses:      {PROBE_WELLS} temperature probe wells, {MIXING_COUPONS} mixing coupon pockets, {VISCOSITY_POCKETS} viscosity and {OSMOLALITY_POCKETS} osmolality sample pockets");
    println!("  Closed transfer:            {CONNECTOR_PORTS} sterile connector bulkhead ports under a light-protection cover envelope");
    println!("  Evidence and service:       {BARCODE_LANDS} barcode/status lands, {WASTE_BINS} segregated waste bins, {EVIDENCE_CAMERAS} evidence camera lands, front robot approach {FRONT_ROBOT_APPROACH:.0}mm");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
    assert!(CHILLED_CENTER.0 - CHILLED_X / 2.0 > -DECK_X / 2.0 + 48.0);
    assert!(BULKHEAD_CENTER.0 + BULKHEAD_X / 2.0 < DECK_X / 2.0 - 82.0);
    assert!(TOKEN_CENTER.0 - TOKEN_X / 2.0 > WARM_CENTER.0 + WARM_X / 2.0 - 84.0);
    assert!(WASTE_CENTER.1 - WASTE_Y / 2.0 > -DECK_Y / 2.0 + 44.0);
    assert!(STATUS_CENTER.1 - STATUS_Y / 2.0 > -DECK_Y / 2.0 + 44.0);
    assert!(connector_span_y() + CONNECTOR_COLLAR_D < BULKHEAD_Y);
    assert!(COVER_X < DECK_X - 2.0 * RIM_W);
    assert!(COVER_Y < DECK_Y - 2.0 * RIM_W);
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube("ecm_thaw_dwell_base_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let chilled_socket = deck_socket(
        "ecm_thaw_dwell_chilled_nest_socket",
        CHILLED_CENTER,
        CHILLED_X + 22.0,
        CHILLED_Y + 22.0,
    );
    let warm_socket = deck_socket(
        "ecm_thaw_dwell_warm_nest_socket",
        WARM_CENTER,
        WARM_X + 22.0,
        WARM_Y + 22.0,
    );
    let coupon_socket = deck_socket(
        "ecm_thaw_dwell_coupon_socket",
        COUPON_CENTER,
        COUPON_X + 20.0,
        COUPON_Y + 18.0,
    );
    let sample_socket = deck_socket(
        "ecm_thaw_dwell_sample_socket",
        SAMPLE_CENTER,
        SAMPLE_X + 18.0,
        SAMPLE_Y + 18.0,
    );
    let waste_gutter = centered_cube("ecm_thaw_dwell_waste_gutter_cut", WASTE_X + 80.0, 20.0, 9.0)
        .translate(
            WASTE_CENTER.0,
            WASTE_CENTER.1 - WASTE_Y / 2.0 - 18.0,
            DECK_Z - 4.0,
        );
    let drain = centered_cylinder("ecm_thaw_dwell_tray_drain", DRAIN_D / 2.0, 42.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 17.0, DECK_Z / 2.0);

    deck - chilled_socket
        - warm_socket
        - coupon_socket
        - sample_socket
        - waste_gutter
        - drain
        - mounting_holes()
        + perimeter_rim()
        + datum_targets()
        + zone_lands()
}

fn deck_socket(name: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(name, x, y, SOCKET_DEPTH + 0.4).translate(
        center.0,
        center.1,
        DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("ecm_thaw_dwell_mounting_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 64.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 64.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 64.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 64.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("ecm_thaw_dwell_m6_clearance_{i}"),
                3.4,
                DECK_Z + 2.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let rear = centered_cube("ecm_thaw_dwell_rear_leak_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("ecm_thaw_dwell_left_leak_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("ecm_thaw_dwell_right_leak_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let front_left = centered_cube("ecm_thaw_dwell_front_left_low_load_lip", 390.0, RIM_W, 16.0)
        .translate(
            -DECK_X / 2.0 + 205.0,
            -DECK_Y / 2.0 + RIM_W / 2.0,
            DECK_Z + 8.0,
        );
    let front_right = centered_cube(
        "ecm_thaw_dwell_front_right_low_load_lip",
        390.0,
        RIM_W,
        16.0,
    )
    .translate(
        DECK_X / 2.0 - 205.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 8.0,
    );
    rear + left + right + front_left + front_right
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("ecm_thaw_dwell_datum_targets");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        let puck = centered_cylinder(format!("ecm_thaw_dwell_datum_puck_{i}"), 12.0, 4.0, 40)
            .translate(*x, *y, DECK_Z + 2.0);
        let center =
            centered_cylinder(format!("ecm_thaw_dwell_datum_cross_bore_{i}"), 2.2, 5.0, 20)
                .translate(*x, *y, DECK_Z + 2.0);
        targets = targets + (puck - center);
    }
    targets
}

fn zone_lands() -> Part {
    let thermal = centered_cube("ecm_thaw_dwell_thermal_zone_land", 820.0, 10.0, 3.0).translate(
        -210.0,
        CHILLED_CENTER.1 - CHILLED_Y / 2.0 - 18.0,
        DECK_Z + 1.5,
    );
    let measurement = centered_cube("ecm_thaw_dwell_measurement_zone_land", 1010.0, 10.0, 3.0)
        .translate(-72.0, COUPON_CENTER.1 + COUPON_Y / 2.0 + 18.0, DECK_Z + 1.5);
    let quarantine = centered_cube("ecm_thaw_dwell_quarantine_zone_land", WASTE_X, 10.0, 3.0)
        .translate(
            WASTE_CENTER.0,
            WASTE_CENTER.1 + WASTE_Y / 2.0 + 17.0,
            DECK_Z + 1.5,
        );

    thermal + measurement + quarantine
}

fn chilled_vial_nest() -> Part {
    let body = centered_cube(
        "ecm_thaw_dwell_chilled_vial_cold_plate",
        CHILLED_X,
        CHILLED_Y,
        CHILLED_Z,
    );
    let mut cuts = Part::empty("ecm_thaw_dwell_chilled_vial_well_cuts");

    for row in 0..CHILLED_ROWS {
        for col in 0..CHILLED_COLS {
            let (x, y) = chilled_vial_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("ecm_thaw_dwell_chilled_vial_well_r{row}_c{col}"),
                    CHILLED_WELL_D / 2.0,
                    34.0,
                    36,
                )
                .translate(x, y, CHILLED_Z / 2.0 - 12.0);
        }
    }

    for i in 0..ICE_PACK_SLOTS {
        let y = if i < 2 {
            -CHILLED_Y / 2.0 + 28.0
        } else {
            CHILLED_Y / 2.0 - 28.0
        };
        let x = if i % 2 == 0 { -92.0 } else { 92.0 };
        cuts = cuts
            + centered_cube(
                format!("ecm_thaw_dwell_ice_pack_recess_{i}"),
                138.0,
                24.0,
                24.0,
            )
            .translate(x, y, CHILLED_Z / 2.0 - 10.0);
    }

    (body - cuts).translate(CHILLED_CENTER.0, CHILLED_CENTER.1, DECK_Z + CHILLED_Z / 2.0)
        + chilled_retainer_fingers()
        + chilled_barrier_label_lands()
}

fn chilled_retainer_fingers() -> Part {
    let mut fingers = Part::empty("ecm_thaw_dwell_chilled_vial_retainer_fingers");
    for row in 0..CHILLED_ROWS {
        for col in 0..CHILLED_COLS {
            let (x, y) = chilled_vial_xy(row, col);
            for side in [-1.0, 1.0] {
                fingers = fingers
                    + centered_cube(
                        format!("ecm_thaw_dwell_chilled_retainer_r{row}_c{col}_{side}"),
                        8.0,
                        4.0,
                        12.0,
                    )
                    .translate(
                        CHILLED_CENTER.0 + x + side * (CHILLED_WELL_D / 2.0 + 5.0),
                        CHILLED_CENTER.1 + y,
                        DECK_Z + CHILLED_Z + 6.0,
                    );
            }
        }
    }
    fingers
}

fn chilled_barrier_label_lands() -> Part {
    let front = centered_cube(
        "ecm_thaw_dwell_chilled_chain_of_custody_label_land",
        CHILLED_X - 56.0,
        24.0,
        4.0,
    )
    .translate(
        CHILLED_CENTER.0,
        CHILLED_CENTER.1 - CHILLED_Y / 2.0 - 22.0,
        DECK_Z + 3.0,
    );
    let frost_witness = centered_cube(
        "ecm_thaw_dwell_chilled_frost_witness_land",
        112.0,
        28.0,
        4.0,
    )
    .translate(
        CHILLED_CENTER.0 + CHILLED_X / 2.0 - 72.0,
        CHILLED_CENTER.1 + CHILLED_Y / 2.0 + 24.0,
        DECK_Z + 3.0,
    );
    front + frost_witness
}

fn warm_hold_vial_nest() -> Part {
    let body = centered_cube("ecm_thaw_dwell_warm_hold_block", WARM_X, WARM_Y, WARM_Z);
    let mut cuts = Part::empty("ecm_thaw_dwell_warm_hold_cuts");

    for row in 0..WARM_ROWS {
        for col in 0..WARM_COLS {
            let (x, y) = warm_vial_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("ecm_thaw_dwell_warm_vial_well_r{row}_c{col}"),
                    WARM_WELL_D / 2.0,
                    38.0,
                    40,
                )
                .translate(x, y, WARM_Z / 2.0 - 13.0);
        }
    }

    for i in 0..HEAT_TRACE_CHANNELS {
        let y = -WARM_Y / 2.0 + 42.0 + i as f64 * 52.0;
        cuts = cuts
            + centered_cylinder(
                format!("ecm_thaw_dwell_heat_trace_channel_{i}"),
                4.2,
                WARM_X + 6.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 4.0);
    }

    (body - cuts).translate(WARM_CENTER.0, WARM_CENTER.1, DECK_Z + WARM_Z / 2.0)
        + warm_lid_clamp_tabs()
        + dwell_start_gate()
}

fn warm_lid_clamp_tabs() -> Part {
    let mut tabs = Part::empty("ecm_thaw_dwell_warm_lid_clamp_tabs");
    for i in 0..4 {
        let x = WARM_CENTER.0
            + if i % 2 == 0 {
                -WARM_X / 2.0 - 8.0
            } else {
                WARM_X / 2.0 + 8.0
            };
        let y = WARM_CENTER.1 + if i < 2 { -86.0 } else { 86.0 };
        tabs = tabs
            + centered_cube(
                format!("ecm_thaw_dwell_warm_lid_clamp_tab_{i}"),
                16.0,
                52.0,
                20.0,
            )
            .translate(x, y, DECK_Z + WARM_Z + 10.0);
    }
    tabs
}

fn dwell_start_gate() -> Part {
    centered_cube(
        "ecm_thaw_dwell_warm_hold_dwell_start_gate",
        WARM_X - 40.0,
        14.0,
        34.0,
    )
    .translate(
        WARM_CENTER.0,
        WARM_CENTER.1 - WARM_Y / 2.0 - 12.0,
        DECK_Z + WARM_Z + 17.0,
    )
}

fn dwell_token_rails() -> Part {
    let base = centered_cube("ecm_thaw_dwell_token_rail_base", TOKEN_X, TOKEN_Y, TOKEN_Z);
    let mut slots = Part::empty("ecm_thaw_dwell_token_slot_cuts");
    for rail in 0..DWELL_RAILS {
        let y = token_rail_y(rail);
        for token in 0..TOKENS_PER_RAIL {
            let x = -TOKEN_X / 2.0 + 48.0 + token as f64 * 48.0;
            slots = slots
                + centered_cube(
                    format!("ecm_thaw_dwell_token_slot_r{rail}_t{token}"),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_Z + 2.0,
                )
                .translate(x, y, 3.0);
        }
    }
    let rails = (base - slots).translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z + TOKEN_Z / 2.0);
    rails + token_time_index_tabs() + overdue_quarantine_gate()
}

fn token_time_index_tabs() -> Part {
    let mut tabs = Part::empty("ecm_thaw_dwell_token_time_index_tabs");
    for rail in 0..DWELL_RAILS {
        let y = TOKEN_CENTER.1 + token_rail_y(rail);
        tabs = tabs
            + centered_cube(
                format!("ecm_thaw_dwell_token_rail_{rail}_time_label_land"),
                52.0,
                28.0,
                4.0,
            )
            .translate(
                TOKEN_CENTER.0 + TOKEN_X / 2.0 - 32.0,
                y,
                DECK_Z + TOKEN_Z + 2.0,
            );
    }
    tabs
}

fn overdue_quarantine_gate() -> Part {
    centered_cube(
        "ecm_thaw_dwell_overdue_token_quarantine_gate",
        TOKEN_X - 58.0,
        12.0,
        38.0,
    )
    .translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1 - TOKEN_Y / 2.0 - 18.0,
        DECK_Z + TOKEN_Z + 19.0,
    )
}

fn temperature_probe_wells() -> Part {
    let body = centered_cube("ecm_thaw_dwell_probe_well_block", PROBE_X, PROBE_Y, PROBE_Z);
    let mut cuts = Part::empty("ecm_thaw_dwell_probe_well_cuts");
    for i in 0..PROBE_WELLS {
        let x = -PROBE_PITCH_X * (PROBE_WELLS as f64 - 1.0) / 2.0 + i as f64 * PROBE_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("ecm_thaw_dwell_temperature_probe_well_{i}"),
                PROBE_WELL_D / 2.0,
                PROBE_Z + 3.0,
                24,
            )
            .translate(x, 28.0, 4.0);
    }
    let block = (body - cuts).translate(PROBE_CENTER.0, PROBE_CENTER.1, DECK_Z + PROBE_Z / 2.0);
    block + probe_cable_clip_comb() + calibration_reference_sockets()
}

fn probe_cable_clip_comb() -> Part {
    let mut clips = Part::empty("ecm_thaw_dwell_probe_cable_clip_comb");
    for i in 0..PROBE_CABLE_CLIPS {
        let x = PROBE_CENTER.0 - 100.0 + i as f64 * 50.0;
        clips = clips
            + centered_cube(
                format!("ecm_thaw_dwell_probe_cable_clip_{i}"),
                22.0,
                12.0,
                18.0,
            )
            .translate(x, PROBE_CENTER.1 - PROBE_Y / 2.0 - 12.0, DECK_Z + 18.0);
    }
    clips
}

fn calibration_reference_sockets() -> Part {
    let ice = centered_cube(
        "ecm_thaw_dwell_probe_ice_point_reference_socket",
        70.0,
        40.0,
        18.0,
    )
    .translate(
        PROBE_CENTER.0 - 76.0,
        PROBE_CENTER.1 - 36.0,
        DECK_Z + PROBE_Z + 9.0,
    );
    let warm = centered_cube(
        "ecm_thaw_dwell_probe_warm_reference_socket",
        70.0,
        40.0,
        18.0,
    )
    .translate(
        PROBE_CENTER.0 + 76.0,
        PROBE_CENTER.1 - 36.0,
        DECK_Z + PROBE_Z + 9.0,
    );
    ice + warm
}

fn mixing_witness_coupon_pockets() -> Part {
    let plate = centered_cube(
        "ecm_thaw_dwell_mixing_coupon_plate",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let mut cuts = Part::empty("ecm_thaw_dwell_mixing_coupon_pocket_cuts");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let x =
                -COUPON_PITCH_X * (COUPON_COLS as f64 - 1.0) / 2.0 + col as f64 * COUPON_PITCH_X;
            let y = -COUPON_PITCH_Y / 2.0 + row as f64 * COUPON_PITCH_Y;
            cuts = cuts
                + centered_cube(
                    format!("ecm_thaw_dwell_mixing_coupon_pocket_r{row}_c{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    18.0,
                )
                .translate(x, y, COUPON_Z / 2.0 - 8.0);
        }
    }
    (plate - cuts).translate(COUPON_CENTER.0, COUPON_CENTER.1, DECK_Z + COUPON_Z / 2.0)
        + coupon_blank_reference_lane()
        + coating_uniformity_gradient_scale()
}

fn coupon_blank_reference_lane() -> Part {
    centered_cube(
        "ecm_thaw_dwell_coupon_blank_reference_lane",
        COUPON_X - 52.0,
        18.0,
        8.0,
    )
    .translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1 + COUPON_Y / 2.0 + 14.0,
        DECK_Z + COUPON_Z + 4.0,
    )
}

fn coating_uniformity_gradient_scale() -> Part {
    let mut marks = Part::empty("ecm_thaw_dwell_coating_uniformity_gradient_scale");
    for i in 0..7 {
        marks = marks
            + centered_cube(
                format!("ecm_thaw_dwell_gradient_scale_mark_{i}"),
                6.0,
                18.0 + i as f64 * 4.0,
                4.0,
            )
            .translate(
                COUPON_CENTER.0 - COUPON_X / 2.0 - 14.0,
                COUPON_CENTER.1 - 54.0 + i as f64 * 18.0,
                DECK_Z + 4.0,
            );
    }
    marks
}

fn viscosity_osmolality_sample_pockets() -> Part {
    let body = centered_cube(
        "ecm_thaw_dwell_sample_pocket_block",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let mut cuts = Part::empty("ecm_thaw_dwell_sample_pocket_cuts");
    for i in 0..VISCOSITY_POCKETS {
        let x = -SAMPLE_PITCH_X * 1.5 + i as f64 * SAMPLE_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("ecm_thaw_dwell_viscosity_sample_pocket_{i}"),
                SAMPLE_POCKET_D / 2.0,
                SAMPLE_Z + 2.0,
                32,
            )
            .translate(x, 42.0, 3.0);
    }
    for i in 0..OSMOLALITY_POCKETS {
        let x = -SAMPLE_PITCH_X * 1.5 + i as f64 * SAMPLE_PITCH_X;
        cuts = cuts
            + centered_cylinder(
                format!("ecm_thaw_dwell_osmolality_sample_pocket_{i}"),
                SAMPLE_POCKET_D / 2.0,
                SAMPLE_Z + 2.0,
                32,
            )
            .translate(x, -42.0, 3.0);
    }
    (body - cuts).translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, DECK_Z + SAMPLE_Z / 2.0)
        + sample_cap_parking_comb()
        + retained_sample_chain_lands()
}

fn sample_cap_parking_comb() -> Part {
    let mut comb = Part::empty("ecm_thaw_dwell_sample_cap_parking_comb");
    for i in 0..(VISCOSITY_POCKETS + OSMOLALITY_POCKETS) {
        let x = SAMPLE_CENTER.0 - 126.0 + i as f64 * 36.0;
        comb = comb
            + centered_cylinder(
                format!("ecm_thaw_dwell_sample_cap_parking_post_{i}"),
                6.0,
                12.0,
                24,
            )
            .translate(x, SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 - 20.0, DECK_Z + 6.0);
    }
    comb
}

fn retained_sample_chain_lands() -> Part {
    centered_cube(
        "ecm_thaw_dwell_retained_sample_chain_of_custody_land",
        SAMPLE_X - 44.0,
        24.0,
        4.0,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1 + SAMPLE_Y / 2.0 + 20.0,
        DECK_Z + 3.0,
    )
}

fn light_protection_cover_envelope() -> Part {
    let mut cover = Part::empty("ecm_thaw_dwell_light_protection_cover_envelope");
    for (i, (x, y)) in cover_post_positions().iter().enumerate() {
        cover = cover
            + centered_cube(
                format!("ecm_thaw_dwell_amber_cover_post_{i}"),
                COVER_POST_X,
                COVER_POST_Y,
                COVER_Z,
            )
            .translate(*x, *y, DECK_Z + COVER_Z / 2.0);
    }

    let front = centered_cube(
        "ecm_thaw_dwell_amber_cover_front_beam",
        COVER_X,
        COVER_POST_Y,
        COVER_BEAM_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 - COVER_Y / 2.0,
        DECK_Z + COVER_Z - COVER_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "ecm_thaw_dwell_amber_cover_rear_beam",
        COVER_X,
        COVER_POST_Y,
        COVER_BEAM_Z,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1 + COVER_Y / 2.0,
        DECK_Z + COVER_Z - COVER_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        "ecm_thaw_dwell_amber_cover_left_beam",
        COVER_POST_X,
        COVER_Y,
        COVER_BEAM_Z,
    )
    .translate(
        COVER_CENTER.0 - COVER_X / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z - COVER_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        "ecm_thaw_dwell_amber_cover_right_beam",
        COVER_POST_X,
        COVER_Y,
        COVER_BEAM_Z,
    )
    .translate(
        COVER_CENTER.0 + COVER_X / 2.0,
        COVER_CENTER.1,
        DECK_Z + COVER_Z - COVER_BEAM_Z / 2.0,
    );
    let roof = centered_cube(
        "ecm_thaw_dwell_amber_cover_lift_clearance_roof",
        COVER_X - 62.0,
        COVER_Y - 62.0,
        8.0,
    )
    .translate(COVER_CENTER.0, COVER_CENTER.1, DECK_Z + COVER_Z + 4.0);

    cover + front + rear + left + right + roof + cover_window_lands() + cover_latch_pockets()
}

fn cover_window_lands() -> Part {
    let mut lands = Part::empty("ecm_thaw_dwell_cover_inspection_window_lands");
    for i in 0..COVER_INSPECTION_WINDOWS {
        let x = COVER_CENTER.0 - 270.0 + i as f64 * 180.0;
        lands = lands
            + centered_cube(
                format!("ecm_thaw_dwell_cover_inspection_window_land_{i}"),
                92.0,
                8.0,
                42.0,
            )
            .translate(x, COVER_CENTER.1 - COVER_Y / 2.0 - 5.0, DECK_Z + 90.0);
    }
    lands
}

fn cover_latch_pockets() -> Part {
    let mut pockets = Part::empty("ecm_thaw_dwell_cover_latch_pockets");
    for i in 0..4 {
        let x = COVER_CENTER.0
            + if i % 2 == 0 {
                -COVER_X / 2.0 - 18.0
            } else {
                COVER_X / 2.0 + 18.0
            };
        let y = COVER_CENTER.1
            + if i < 2 {
                -COVER_Y / 2.0 + 72.0
            } else {
                COVER_Y / 2.0 - 72.0
            };
        pockets = pockets
            + centered_cube(
                format!("ecm_thaw_dwell_cover_latch_pocket_{i}"),
                20.0,
                54.0,
                16.0,
            )
            .translate(x, y, DECK_Z + 18.0);
    }
    pockets
}

fn sterile_connector_bulkhead() -> Part {
    let plate = centered_cube(
        "ecm_thaw_dwell_sterile_connector_bulkhead_plate",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let mut holes = Part::empty("ecm_thaw_dwell_sterile_connector_bulkhead_holes");
    for i in 0..CONNECTOR_PORTS {
        let y = connector_y(i);
        holes = holes
            + centered_cylinder(
                format!("ecm_thaw_dwell_sterile_connector_port_{i}"),
                CONNECTOR_PORT_D / 2.0,
                BULKHEAD_X + 4.0,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 34.0);
    }
    let bulkhead = (plate - holes).translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1,
        DECK_Z + BULKHEAD_Z / 2.0,
    );
    bulkhead + connector_collars() + tube_bend_relief_rack() + cap_quarantine_clip_bar()
}

fn connector_collars() -> Part {
    let mut collars = Part::empty("ecm_thaw_dwell_sterile_connector_collars");
    for i in 0..CONNECTOR_PORTS {
        let y = BULKHEAD_CENTER.1 + connector_y(i);
        let collar = centered_cylinder(
            format!("ecm_thaw_dwell_sterile_connector_collar_{i}"),
            CONNECTOR_COLLAR_D / 2.0,
            12.0,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(BULKHEAD_CENTER.0 - BULKHEAD_X / 2.0 - 6.0, y, DECK_Z + 34.0);
        let bore = centered_cylinder(
            format!("ecm_thaw_dwell_sterile_connector_collar_bore_{i}"),
            CONNECTOR_PORT_D / 2.0,
            14.0,
            40,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(BULKHEAD_CENTER.0 - BULKHEAD_X / 2.0 - 6.0, y, DECK_Z + 34.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn tube_bend_relief_rack() -> Part {
    let mut rack = Part::empty("ecm_thaw_dwell_tube_bend_relief_rack");
    for i in 0..CONNECTOR_PORTS {
        rack = rack
            + centered_cube(
                format!("ecm_thaw_dwell_connector_tube_bend_relief_{i}"),
                88.0,
                10.0,
                18.0,
            )
            .translate(
                BULKHEAD_CENTER.0 - BULKHEAD_X / 2.0 - 58.0,
                BULKHEAD_CENTER.1 + connector_y(i),
                DECK_Z + 70.0,
            );
    }
    rack
}

fn cap_quarantine_clip_bar() -> Part {
    centered_cube(
        "ecm_thaw_dwell_connector_cap_quarantine_clip_bar",
        34.0,
        BULKHEAD_Y - 42.0,
        18.0,
    )
    .translate(
        BULKHEAD_CENTER.0 + BULKHEAD_X / 2.0 + 22.0,
        BULKHEAD_CENTER.1,
        DECK_Z + 32.0,
    )
}

fn barcode_status_lands() -> Part {
    let plate = centered_cube(
        "ecm_thaw_dwell_barcode_status_plate",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z + STATUS_Z / 2.0);
    let mut lands = Part::empty("ecm_thaw_dwell_barcode_status_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 5;
        let row = i / 5;
        lands = lands
            + centered_cube(format!("ecm_thaw_dwell_barcode_land_{i}"), 54.0, 24.0, 4.0).translate(
                STATUS_CENTER.0 - 120.0 + col as f64 * 60.0,
                STATUS_CENTER.1 - 28.0 + row as f64 * 56.0,
                DECK_Z + STATUS_Z + 2.0,
            );
    }
    for i in 0..STATUS_LANES {
        lands = lands
            + centered_cube(format!("ecm_thaw_dwell_status_lane_{i}"), 66.0, 18.0, 10.0).translate(
                STATUS_CENTER.0 - 114.0 + i as f64 * 76.0,
                STATUS_CENTER.1 + STATUS_Y / 2.0 + 22.0,
                DECK_Z + 8.0,
            );
    }
    plate + lands
}

fn waste_segregation() -> Part {
    let body = centered_cube(
        "ecm_thaw_dwell_waste_segregation_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let mut cuts = Part::empty("ecm_thaw_dwell_waste_bin_cuts");
    for i in 0..WASTE_BINS {
        let x = -WASTE_X / 2.0 + 82.0 + i as f64 * 145.0;
        cuts = cuts
            + centered_cube(
                format!("ecm_thaw_dwell_waste_bin_recess_{i}"),
                108.0,
                86.0,
                34.0,
            )
            .translate(x, 0.0, WASTE_Z / 2.0 - 13.0);
    }
    for i in 0..WASTE_PORTS {
        let x = -WASTE_X / 2.0 + 52.0 + i as f64 * 70.0;
        cuts = cuts
            + centered_cylinder(
                format!("ecm_thaw_dwell_waste_drain_port_{i}"),
                5.0,
                WASTE_Z + 2.0,
                24,
            )
            .translate(x, -WASTE_Y / 2.0 + 18.0, 2.0);
    }
    (body - cuts).translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z / 2.0)
        + waste_divider_signals()
        + contaminated_tip_drop_chute()
}

fn waste_divider_signals() -> Part {
    let mut dividers = Part::empty("ecm_thaw_dwell_waste_divider_signals");
    for i in 1..WASTE_BINS {
        dividers = dividers
            + centered_cube(
                format!("ecm_thaw_dwell_waste_stream_divider_{i}"),
                8.0,
                WASTE_Y,
                30.0,
            )
            .translate(
                WASTE_CENTER.0 - WASTE_X / 2.0 + i as f64 * (WASTE_X / WASTE_BINS as f64),
                WASTE_CENTER.1,
                DECK_Z + WASTE_Z + 15.0,
            );
    }
    dividers
}

fn contaminated_tip_drop_chute() -> Part {
    centered_cube(
        "ecm_thaw_dwell_contaminated_tip_drop_chute",
        74.0,
        WASTE_Y + 26.0,
        60.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 + 48.0,
        WASTE_CENTER.1,
        DECK_Z + 30.0,
    )
}

fn evidence_bridge_robot_service_keepouts() -> Part {
    let mut bridge = Part::empty("ecm_thaw_dwell_evidence_bridge_robot_service_keepouts");
    let left_post = centered_cube(
        "ecm_thaw_dwell_evidence_bridge_left_post",
        28.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(
        -EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        DECK_Z + EVIDENCE_BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "ecm_thaw_dwell_evidence_bridge_right_post",
        28.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(
        EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        DECK_Z + EVIDENCE_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "ecm_thaw_dwell_evidence_bridge_cross_beam",
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        30.0,
    )
    .translate(
        EVIDENCE_CENTER.0,
        EVIDENCE_CENTER.1,
        DECK_Z + EVIDENCE_BRIDGE_Z - 15.0,
    );
    bridge = bridge + left_post + right_post + beam;

    for i in 0..EVIDENCE_CAMERAS {
        bridge = bridge
            + centered_cube(
                format!("ecm_thaw_dwell_evidence_camera_mount_{i}"),
                52.0,
                32.0,
                18.0,
            )
            .translate(
                -390.0 + i as f64 * 260.0,
                EVIDENCE_CENTER.1 - 42.0,
                DECK_Z + EVIDENCE_BRIDGE_Z - 48.0,
            );
    }
    for i in 0..SERVICE_LIGHTS {
        bridge = bridge
            + centered_cube(
                format!("ecm_thaw_dwell_evidence_light_bar_{i}"),
                150.0,
                18.0,
                14.0,
            )
            .translate(
                -220.0 + i as f64 * 220.0,
                EVIDENCE_CENTER.1 + 42.0,
                DECK_Z + EVIDENCE_BRIDGE_Z - 54.0,
            );
    }

    bridge + keepout_gauges()
}

fn keepout_gauges() -> Part {
    let robot = centered_cube(
        "ecm_thaw_dwell_robot_wrist_sweep_keepout_gauge",
        ROBOT_KEEP_OUT_X,
        8.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        50.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_APPROACH,
        DECK_Z + ROBOT_KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "ecm_thaw_dwell_rear_service_keepout_gauge",
        DECK_X - 170.0,
        8.0,
        92.0,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_SERVICE_CLEARANCE, DECK_Z + 46.0);
    let side = centered_cube(
        "ecm_thaw_dwell_connector_side_service_keepout_gauge",
        8.0,
        ROBOT_KEEP_OUT_Y,
        96.0,
    )
    .translate(DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE, -20.0, DECK_Z + 48.0);
    let cover_lift = centered_cube(
        "ecm_thaw_dwell_light_cover_lift_keepout_gauge",
        COVER_X,
        COVER_Y,
        8.0,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1,
        DECK_Z + COVER_LIFT_CLEARANCE,
    );
    robot + rear + side + cover_lift
}

fn datum_positions() -> [(f64, f64); DATUM_TARGETS] {
    [
        (-DECK_X / 2.0 + 72.0, -DECK_Y / 2.0 + 72.0),
        (DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 + 72.0),
        (-DECK_X / 2.0 + 72.0, DECK_Y / 2.0 - 72.0),
        (DECK_X / 2.0 - 72.0, DECK_Y / 2.0 - 72.0),
        (CHILLED_CENTER.0, CHILLED_CENTER.1 + CHILLED_Y / 2.0 + 32.0),
        (
            BULKHEAD_CENTER.0,
            BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 + 34.0,
        ),
    ]
}

fn chilled_vial_xy(row: usize, col: usize) -> (f64, f64) {
    (
        -CHILLED_PITCH_X * (CHILLED_COLS as f64 - 1.0) / 2.0 + col as f64 * CHILLED_PITCH_X,
        -CHILLED_PITCH_Y * (CHILLED_ROWS as f64 - 1.0) / 2.0 + row as f64 * CHILLED_PITCH_Y,
    )
}

fn warm_vial_xy(row: usize, col: usize) -> (f64, f64) {
    (
        -WARM_PITCH_X * (WARM_COLS as f64 - 1.0) / 2.0 + col as f64 * WARM_PITCH_X,
        -WARM_PITCH_Y * (WARM_ROWS as f64 - 1.0) / 2.0 + row as f64 * WARM_PITCH_Y,
    )
}

fn token_rail_y(rail: usize) -> f64 {
    -TOKEN_RAIL_PITCH_Y * (DWELL_RAILS as f64 - 1.0) / 2.0 + rail as f64 * TOKEN_RAIL_PITCH_Y
}

fn connector_y(index: usize) -> f64 {
    -BULKHEAD_PORT_PITCH_Y * (CONNECTOR_PORTS as f64 - 1.0) / 2.0
        + index as f64 * BULKHEAD_PORT_PITCH_Y
}

fn connector_span_y() -> f64 {
    BULKHEAD_PORT_PITCH_Y * (CONNECTOR_PORTS as f64 - 1.0)
}

fn cover_post_positions() -> [(f64, f64); 4] {
    [
        (
            COVER_CENTER.0 - COVER_X / 2.0,
            COVER_CENTER.1 - COVER_Y / 2.0,
        ),
        (
            COVER_CENTER.0 + COVER_X / 2.0,
            COVER_CENTER.1 - COVER_Y / 2.0,
        ),
        (
            COVER_CENTER.0 - COVER_X / 2.0,
            COVER_CENTER.1 + COVER_Y / 2.0,
        ),
        (
            COVER_CENTER.0 + COVER_X / 2.0,
            COVER_CENTER.1 + COVER_Y / 2.0,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_ecm_coating_reagent_thaw_dwell_uniformity_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn thermal_staging_counts_are_explicit() {
        assert_eq!(CHILLED_ROWS * CHILLED_COLS, CHILLED_VIALS);
        assert_eq!(WARM_ROWS * WARM_COLS, WARM_VIALS);
        assert_eq!(ICE_PACK_SLOTS, 4);
        assert_eq!(HEAT_TRACE_CHANNELS, 4);
        assert!(CHILLED_WELL_D + 12.0 < CHILLED_PITCH_X);
        assert!(WARM_WELL_D + 16.0 < WARM_PITCH_X);
    }

    #[test]
    fn dwell_measurement_and_sample_features_have_capacity() {
        assert_eq!(DWELL_RAILS * TOKENS_PER_RAIL, TOKEN_SLOTS);
        assert_eq!(COUPON_ROWS * COUPON_COLS, MIXING_COUPONS);
        assert_eq!(PROBE_WELLS, 8);
        assert_eq!(VISCOSITY_POCKETS, 4);
        assert_eq!(OSMOLALITY_POCKETS, 4);
        assert!(TOKEN_SLOT_X * (TOKENS_PER_RAIL as f64) < TOKEN_X);
    }

    #[test]
    fn closed_transfer_and_traceability_are_present() {
        assert_eq!(CONNECTOR_PORTS, 6);
        assert!(connector_span_y() + CONNECTOR_COLLAR_D < BULKHEAD_Y);
        assert!(CONNECTOR_PORT_D < CONNECTOR_COLLAR_D);
        assert_eq!(BARCODE_LANDS, 10);
        assert_eq!(STATUS_LANES, 4);
        assert_eq!(WASTE_BINS, 3);
        assert_eq!(WASTE_PORTS, 6);
    }

    #[test]
    fn light_cover_and_keepouts_clear_station_envelope() {
        assert_eq!(COVER_INSPECTION_WINDOWS, 4);
        assert_eq!(EVIDENCE_CAMERAS, 4);
        assert_eq!(SERVICE_LIGHTS, 3);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 250.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 220.0);
        assert!(COVER_LIFT_CLEARANCE > COVER_Z + DECK_Z);
    }
}
