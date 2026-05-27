use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed additive/growth-factor/cytokine addition QC station for media preparation.
//
// Intent:
// - Keep thawed additives, growth factors, and cytokines in chilled indexed nests
//   while a closed septum-port manifold meters them into prepared media.
// - Provide physical witness geometry for micro-dose metering, mix verification,
//   lot/label scanning, cold-chain evidence, released/hold/reject segregation,
//   waste capture, and robot/service keepouts.
// - Package purchased chillers, septa, loggers, balances, scanners, and optical
//   sensors without defining clinical release criteria or sterile-barrier claims.
//
// Product concept CAD only. This file intentionally stays standalone so parent
// integration can decide when to add the bin to manifests and docs.

const OUTPUTS: [&str; 11] = [
    "output/closed_growth_factor_cytokine_addition_qc_station_chilled_spill_deck.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_chilled_additive_nests.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_sterile_septum_transfer_ports.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_micro_dose_metering_witness.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_mix_verification_pocket.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_label_lot_scan_lands.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_cold_chain_logger_pocket.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_release_hold_reject_lanes.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_waste_capture.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_robot_service_keepouts.stl",
    "output/closed_growth_factor_cytokine_addition_qc_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "chilled_additive_nests",
    "sterile_septum_transfer_ports",
    "micro_dose_metering_witness",
    "mix_verification_pocket",
    "label_lot_scan_lands",
    "cold_chain_logger_pocket",
    "release_hold_reject_lanes",
    "waste_capture",
    "robot_service_keepouts",
    "assembly_export",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 880.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const DRAIN_PORT_D: f64 = 16.0;

const NEST_POS: (f64, f64) = (-410.0, 222.0);
const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 290.0;
const NEST_Z: f64 = 58.0;
const ADDITIVE_ROWS: usize = 2;
const ADDITIVE_COLS: usize = 4;
const ADDITIVE_NEST_COUNT: usize = ADDITIVE_ROWS * ADDITIVE_COLS;
const VIAL_SEAT_D: f64 = 23.0;
const VIAL_CLEARANCE_D: f64 = 27.0;
const ADDITIVE_PITCH_X: f64 = 86.0;
const ADDITIVE_PITCH_Y: f64 = 104.0;
const COLD_WELL_DEPTH: f64 = 44.0;
const CHILLER_SLUG_COUNT: usize = 4;
const CHILLER_SLUG_X: f64 = 76.0;
const CHILLER_SLUG_Y: f64 = 54.0;

const PORT_POS: (f64, f64) = (30.0, 260.0);
const PORT_PANEL_X: f64 = 370.0;
const PORT_PANEL_Y: f64 = 220.0;
const PORT_PANEL_Z: f64 = 64.0;
const SEPTUM_PORT_COUNT: usize = ADDITIVE_NEST_COUNT;
const SEPTUM_PORT_D: f64 = 13.0;
const SEPTUM_COLLAR_D: f64 = 31.0;
const SEPTUM_PORT_PITCH: f64 = 42.0;
const CLOSED_TRANSFER_THROAT_X: f64 = 286.0;
const CLOSED_TRANSFER_THROAT_Y: f64 = 36.0;
const PURGE_MANIFOLD_D: f64 = 12.0;

const METER_POS: (f64, f64) = (430.0, 250.0);
const METER_X: f64 = 300.0;
const METER_Y: f64 = 220.0;
const METER_Z: f64 = 42.0;
const MICRO_BALANCE_PAD_X: f64 = 112.0;
const MICRO_BALANCE_PAD_Y: f64 = 92.0;
const WITNESS_WELL_COUNT: usize = ADDITIVE_NEST_COUNT;
const WITNESS_WELL_D: f64 = 14.0;
const WITNESS_PITCH_X: f64 = 32.0;
const MICRO_DOSE_CHANNEL_COUNT: usize = ADDITIVE_NEST_COUNT;
const MICRO_DOSE_MIN_UL: f64 = 5.0;
const MICRO_DOSE_MAX_UL: f64 = 250.0;

const MIX_POS: (f64, f64) = (420.0, -40.0);
const MIX_X: f64 = 310.0;
const MIX_Y: f64 = 260.0;
const MIX_Z: f64 = 52.0;
const MIX_CUVETTE_X: f64 = 38.0;
const MIX_CUVETTE_Y: f64 = 88.0;
const MIX_SAMPLE_LOOP_COUNT: usize = 4;
const MIX_SENSOR_WINDOW_COUNT: usize = 2;
const MIX_STATIC_MIXER_CHANNELS: usize = 6;

const SCAN_POS: (f64, f64) = (25.0, -30.0);
const SCAN_X: f64 = 350.0;
const SCAN_Y: f64 = 180.0;
const SCAN_Z: f64 = 12.0;
const LABEL_LAND_COUNT: usize = ADDITIVE_NEST_COUNT;
const LOT_SCAN_LAND_COUNT: usize = ADDITIVE_NEST_COUNT;
const RUN_RECORD_LAND_COUNT: usize = 4;
const LABEL_LAND_X: f64 = 68.0;
const LABEL_LAND_Y: f64 = 24.0;
const SCANNER_STANCHION_Z: f64 = 120.0;

const LOGGER_POS: (f64, f64) = (-420.0, -60.0);
const LOGGER_X: f64 = 330.0;
const LOGGER_Y: f64 = 180.0;
const LOGGER_Z: f64 = 38.0;
const LOGGER_BAY_COUNT: usize = 2;
const LOGGER_BAY_X: f64 = 104.0;
const LOGGER_BAY_Y: f64 = 54.0;
const LOGGER_PROBE_CLIP_COUNT: usize = 4;
const COLD_CHAIN_TOKEN_COUNT: usize = 6;

const STATUS_POS: (f64, f64) = (370.0, -310.0);
const STATUS_X: f64 = 430.0;
const STATUS_Y: f64 = 200.0;
const STATUS_Z: f64 = 40.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 82.0;
const STATUS_SLOT_Y: f64 = 42.0;
const STATUS_LANE_PITCH_Y: f64 = 62.0;

const WASTE_POS: (f64, f64) = (-420.0, -310.0);
const WASTE_X: f64 = 360.0;
const WASTE_Y: f64 = 180.0;
const WASTE_Z: f64 = 48.0;
const WASTE_WELL_COUNT: usize = ADDITIVE_NEST_COUNT;
const WASTE_WELL_D: f64 = 20.0;
const WASTE_BAG_CRADLE_X: f64 = 182.0;
const WASTE_BAG_CRADLE_Y: f64 = 76.0;
const WASTE_DRAIN_D: f64 = 14.0;

const FRONT_ROBOT_APPROACH: f64 = 430.0;
const REAR_SERVICE_ACCESS: f64 = 300.0;
const LEFT_CHILLER_SERVICE_ACCESS: f64 = 240.0;
const SEPTUM_LID_SWING_CLEARANCE: f64 = 190.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;
const KEEP_OUT_BEAM: f64 = 12.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let half_x = DECK_X / 2.0 - RIM_W;
        let half_y = DECK_Y / 2.0 - RIM_W;
        self.center.0 - self.x / 2.0 >= -half_x
            && self.center.0 + self.x / 2.0 <= half_x
            && self.center.1 - self.y / 2.0 >= -half_y
            && self.center.1 + self.y / 2.0 <= half_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let deck = chilled_spill_deck();
    export(&deck, OUTPUTS[0]);

    let nests = chilled_additive_nests();
    export(&nests, OUTPUTS[1]);

    let ports = sterile_septum_transfer_ports();
    export(&ports, OUTPUTS[2]);

    let witness = micro_dose_metering_witness();
    export(&witness, OUTPUTS[3]);

    let mix = mix_verification_pocket();
    export(&mix, OUTPUTS[4]);

    let scans = label_lot_scan_lands();
    export(&scans, OUTPUTS[5]);

    let logger = cold_chain_logger_pocket();
    export(&logger, OUTPUTS[6]);

    let status = release_hold_reject_lanes();
    export(&status, OUTPUTS[7]);

    let waste = waste_capture();
    export(&waste, OUTPUTS[8]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[9]);

    let assembly = deck
        + nests.translate(NEST_POS.0, NEST_POS.1, insert_z(NEST_Z))
        + ports.translate(PORT_POS.0, PORT_POS.1, insert_z(PORT_PANEL_Z))
        + witness.translate(METER_POS.0, METER_POS.1, insert_z(METER_Z))
        + mix.translate(MIX_POS.0, MIX_POS.1, insert_z(MIX_Z))
        + scans.translate(SCAN_POS.0, SCAN_POS.1, insert_z(SCAN_Z))
        + logger.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_Z))
        + keepouts;
    export(&assembly, OUTPUTS[10]);

    println!();
    println!("Closed growth-factor/cytokine addition QC station:");
    println!("  Footprint:          {DECK_X:.0}mm x {DECK_Y:.0}mm chilled spill-control deck");
    println!(
        "  Chilled additives:  {ADDITIVE_NEST_COUNT} indexed nests, {CHILLER_SLUG_COUNT} removable cold slugs, {LOGGER_BAY_COUNT} logger bays, {LOGGER_PROBE_CLIP_COUNT} probe clips"
    );
    println!(
        "  Closed addition:    {SEPTUM_PORT_COUNT} sterile septum transfer ports, {MICRO_DOSE_CHANNEL_COUNT} micro-dose witness channels, {MICRO_DOSE_MIN_UL:.0}-{MICRO_DOSE_MAX_UL:.0} uL metering range"
    );
    println!(
        "  QC verification:    {MIX_SAMPLE_LOOP_COUNT} mix-loop pockets, {MIX_SENSOR_WINDOW_COUNT} optical windows, {MIX_STATIC_MIXER_CHANNELS} static mixer witness channels"
    );
    println!(
        "  Traceability:       {LABEL_LAND_COUNT} label lands, {LOT_SCAN_LAND_COUNT} lot scan lands, {RUN_RECORD_LAND_COUNT} run-record lands, {STATUS_LANES} release/hold/reject lanes"
    );
    println!(
        "  Keepouts:           {FRONT_ROBOT_APPROACH:.0}mm front robot approach, {REAR_SERVICE_ACCESS:.0}mm rear service, {LEFT_CHILLER_SERVICE_ACCESS:.0}mm chiller service, {SEPTUM_LID_SWING_CLEARANCE:.0}mm septum lid swing, {ROBOT_Z_CLEARANCE:.0}mm robot Z clearance"
    );
    println!("  Feature groups:     {}", REQUIRED_FEATURES.len());
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z - SOCKET_DEPTH + height / 2.0
}

fn assert_layout_constraints() {
    for rect in layout_rects() {
        assert!(rect.fits_inside(), "{} exceeds deck footprint", rect.name);
    }

    for (a, b) in non_overlap_pairs() {
        assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
    }

    assert_eq!(ADDITIVE_NEST_COUNT, SEPTUM_PORT_COUNT);
    assert_eq!(ADDITIVE_NEST_COUNT, WITNESS_WELL_COUNT);
    assert_eq!(ADDITIVE_NEST_COUNT, WASTE_WELL_COUNT);
    assert!(VIAL_CLEARANCE_D > VIAL_SEAT_D + 3.0);
    assert!(COLD_WELL_DEPTH < NEST_Z);
    assert!(MICRO_DOSE_MIN_UL < MICRO_DOSE_MAX_UL);
    assert_eq!(STATUS_LANES, 3);
    assert!(ROBOT_Z_CLEARANCE > SCANNER_STANCHION_Z + DECK_Z);
}

fn layout_rects() -> [Rect; 8] {
    [
        rect("chilled_additive_nests", NEST_POS, NEST_X, NEST_Y),
        rect(
            "sterile_septum_transfer_ports",
            PORT_POS,
            PORT_PANEL_X,
            PORT_PANEL_Y,
        ),
        rect("micro_dose_metering_witness", METER_POS, METER_X, METER_Y),
        rect("mix_verification_pocket", MIX_POS, MIX_X, MIX_Y),
        rect("label_lot_scan_lands", SCAN_POS, SCAN_X, SCAN_Y),
        rect("cold_chain_logger_pocket", LOGGER_POS, LOGGER_X, LOGGER_Y),
        rect("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        rect("waste_capture", WASTE_POS, WASTE_X, WASTE_Y),
    ]
}

fn non_overlap_pairs() -> [(Rect, Rect); 13] {
    let rects = layout_rects();
    [
        (rects[0], rects[1]),
        (rects[0], rects[4]),
        (rects[0], rects[5]),
        (rects[1], rects[2]),
        (rects[1], rects[3]),
        (rects[1], rects[4]),
        (rects[2], rects[3]),
        (rects[2], rects[6]),
        (rects[3], rects[4]),
        (rects[3], rects[6]),
        (rects[4], rects[5]),
        (rects[5], rects[7]),
        (rects[6], rects[7]),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn chilled_spill_deck() -> Part {
    let deck = centered_cube(
        "growth_factor_cytokine_qc_chilled_spill_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let pan_recess = centered_cube(
        "growth_factor_cytokine_qc_recessed_chilled_spill_pan",
        DECK_X - 2.0 * (RIM_W + 42.0),
        DECK_Y - 2.0 * (RIM_W + 46.0),
        8.0,
    )
    .translate(0.0, -12.0, DECK_Z - 4.0);
    let front_trough = centered_cube(
        "growth_factor_cytokine_qc_front_waste_trough",
        DECK_X - 150.0,
        24.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 66.0, DECK_Z - 4.0);
    let drain = centered_cylinder(
        "growth_factor_cytokine_qc_closed_drain_bulkhead",
        DRAIN_PORT_D / 2.0,
        52.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 7.0);

    deck - pan_recess - front_trough - drain - insert_sockets() - deck_mount_holes()
        + perimeter_rim()
        + station_zone_lands()
        + cold_side_condensate_gutters()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("growth_factor_cytokine_qc_insert_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("growth_factor_cytokine_qc_socket_{}", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("growth_factor_cytokine_qc_deck_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -DECK_Y / 2.0 + 56.0),
        (0.0, DECK_Y / 2.0 - 56.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("growth_factor_cytokine_qc_m6_mount_hole_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "growth_factor_cytokine_qc_front_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "growth_factor_cytokine_qc_rear_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "growth_factor_cytokine_qc_left_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "growth_factor_cytokine_qc_right_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn station_zone_lands() -> Part {
    let cold = centered_cube(
        "growth_factor_cytokine_qc_cold_incoming_zone_land",
        254.0,
        28.0,
        2.0,
    )
    .translate(-430.0, 72.0, DECK_Z + 1.0);
    let closed_addition = centered_cube(
        "growth_factor_cytokine_qc_closed_addition_zone_land",
        268.0,
        28.0,
        2.0,
    )
    .translate(120.0, 96.0, DECK_Z + 1.0);
    let release = centered_cube(
        "growth_factor_cytokine_qc_release_decision_zone_land",
        252.0,
        28.0,
        2.0,
    )
    .translate(398.0, -185.0, DECK_Z + 1.0);
    cold + closed_addition + release
}

fn cold_side_condensate_gutters() -> Part {
    let upper = centered_cube(
        "growth_factor_cytokine_qc_cold_side_upper_condensate_gutter",
        486.0,
        12.0,
        7.0,
    )
    .translate(-410.0, 390.0, DECK_Z + 3.5);
    let lower = centered_cube(
        "growth_factor_cytokine_qc_cold_side_lower_condensate_gutter",
        486.0,
        12.0,
        7.0,
    )
    .translate(-410.0, 58.0, DECK_Z + 3.5);
    upper + lower
}

fn chilled_additive_nests() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_chilled_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cold_wells = additive_vial_well_cuts();
    let glycol_channels = chilled_serpentine_channels();
    let slug_pockets = chiller_slug_pocket_cuts();

    body - cold_wells - glycol_channels - slug_pockets
        + additive_vial_collars()
        + chiller_slug_retainers()
        + cold_lid_parking_rails()
        + robot_pick_fiducials(
            "growth_factor_cytokine_chilled_nest",
            NEST_X,
            NEST_Y,
            NEST_Z,
        )
}

fn additive_vial_well_cuts() -> Part {
    let mut wells = Part::empty("growth_factor_cytokine_additive_vial_well_cuts");
    for row in 0..ADDITIVE_ROWS {
        for col in 0..ADDITIVE_COLS {
            let i = row * ADDITIVE_COLS + col;
            let (x, y) = additive_grid_xy(row, col);
            wells = wells
                + centered_cylinder(
                    format!("growth_factor_cytokine_cold_well_cut_{i}"),
                    VIAL_CLEARANCE_D / 2.0,
                    COLD_WELL_DEPTH,
                    36,
                )
                .translate(x, y, NEST_Z / 2.0 - COLD_WELL_DEPTH / 2.0 + 1.0);
        }
    }
    wells
}

fn additive_vial_collars() -> Part {
    let mut collars = Part::empty("growth_factor_cytokine_additive_vial_collars");
    for row in 0..ADDITIVE_ROWS {
        for col in 0..ADDITIVE_COLS {
            let i = row * ADDITIVE_COLS + col;
            let (x, y) = additive_grid_xy(row, col);
            let outer = centered_cylinder(
                format!("growth_factor_cytokine_vial_collar_outer_{i}"),
                (VIAL_CLEARANCE_D + 12.0) / 2.0,
                8.0,
                36,
            )
            .translate(x, y, NEST_Z / 2.0 + 4.0);
            let inner = centered_cylinder(
                format!("growth_factor_cytokine_vial_collar_inner_clearance_{i}"),
                VIAL_SEAT_D / 2.0,
                10.0,
                36,
            )
            .translate(x, y, NEST_Z / 2.0 + 4.0);
            collars = collars + (outer - inner);
        }
    }
    collars
}

fn chilled_serpentine_channels() -> Part {
    let mut channels = Part::empty("growth_factor_cytokine_chilled_glycol_serpentine_channels");
    for row in 0..ADDITIVE_ROWS {
        let y = row_y(row, ADDITIVE_ROWS, ADDITIVE_PITCH_Y);
        channels = channels
            + centered_cube(
                format!("growth_factor_cytokine_glycol_row_channel_{row}"),
                340.0,
                12.0,
                12.0,
            )
            .translate(0.0, y, -NEST_Z / 2.0 + 18.0);
    }
    for col in 0..(ADDITIVE_COLS - 1) {
        let x = -((ADDITIVE_COLS as f64 - 2.0) * ADDITIVE_PITCH_X) / 2.0
            + col as f64 * ADDITIVE_PITCH_X;
        channels = channels
            + centered_cube(
                format!("growth_factor_cytokine_glycol_turn_channel_{col}"),
                12.0,
                ADDITIVE_PITCH_Y,
                12.0,
            )
            .translate(x, 0.0, -NEST_Z / 2.0 + 18.0);
    }
    channels
}

fn chiller_slug_pocket_cuts() -> Part {
    let mut pockets = Part::empty("growth_factor_cytokine_chiller_slug_pocket_cuts");
    for i in 0..CHILLER_SLUG_COUNT {
        let x = lane_x(i, CHILLER_SLUG_COUNT, 92.0);
        pockets = pockets
            + centered_cube(
                format!("growth_factor_cytokine_cold_slug_pocket_cut_{i}"),
                CHILLER_SLUG_X,
                CHILLER_SLUG_Y,
                16.0,
            )
            .translate(x, -NEST_Y / 2.0 + 48.0, NEST_Z / 2.0 - 8.0);
    }
    pockets
}

fn chiller_slug_retainers() -> Part {
    let mut retainers = Part::empty("growth_factor_cytokine_chiller_slug_retainers");
    for i in 0..CHILLER_SLUG_COUNT {
        let x = lane_x(i, CHILLER_SLUG_COUNT, 92.0);
        let front = centered_cube(
            format!("growth_factor_cytokine_cold_slug_front_lip_{i}"),
            CHILLER_SLUG_X + 14.0,
            8.0,
            16.0,
        )
        .translate(x, -NEST_Y / 2.0 + 15.0, NEST_Z / 2.0 + 8.0);
        let rear = centered_cube(
            format!("growth_factor_cytokine_cold_slug_rear_lip_{i}"),
            CHILLER_SLUG_X + 14.0,
            8.0,
            16.0,
        )
        .translate(x, -NEST_Y / 2.0 + 80.0, NEST_Z / 2.0 + 8.0);
        retainers = retainers + front + rear;
    }
    retainers
}

fn cold_lid_parking_rails() -> Part {
    let left = centered_cube(
        "growth_factor_cytokine_cold_lid_left_parking_rail",
        14.0,
        NEST_Y - 42.0,
        24.0,
    )
    .translate(-NEST_X / 2.0 + 30.0, 0.0, NEST_Z / 2.0 + 12.0);
    let right = centered_cube(
        "growth_factor_cytokine_cold_lid_right_parking_rail",
        14.0,
        NEST_Y - 42.0,
        24.0,
    )
    .translate(NEST_X / 2.0 - 30.0, 0.0, NEST_Z / 2.0 + 12.0);
    left + right
}

fn sterile_septum_transfer_ports() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_septum_transfer_panel_body",
        PORT_PANEL_X,
        PORT_PANEL_Y,
        PORT_PANEL_Z,
    );
    let bores = septum_port_bores();
    let throat = centered_cube(
        "growth_factor_cytokine_closed_transfer_throat_cut",
        CLOSED_TRANSFER_THROAT_X,
        CLOSED_TRANSFER_THROAT_Y,
        PORT_PANEL_Z + 2.0,
    )
    .translate(0.0, -42.0, 0.0);
    let purge_bore = centered_cylinder(
        "growth_factor_cytokine_purge_manifold_bore",
        PURGE_MANIFOLD_D / 2.0,
        PORT_PANEL_X - 54.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 58.0, 12.0);

    body - bores - throat - purge_bore
        + septum_port_collars()
        + septum_lid_parking_fence()
        + transfer_port_tube_combs()
        + robot_pick_fiducials(
            "growth_factor_cytokine_septum_panel",
            PORT_PANEL_X,
            PORT_PANEL_Y,
            PORT_PANEL_Z,
        )
}

fn septum_port_bores() -> Part {
    let mut bores = Part::empty("growth_factor_cytokine_septum_port_bores");
    for i in 0..SEPTUM_PORT_COUNT {
        bores = bores
            + centered_cylinder(
                format!("growth_factor_cytokine_septum_port_bore_{i}"),
                SEPTUM_PORT_D / 2.0,
                PORT_PANEL_Z + 8.0,
                36,
            )
            .translate(port_x(i), 20.0, 0.0);
    }
    bores
}

fn septum_port_collars() -> Part {
    let mut collars = Part::empty("growth_factor_cytokine_septum_port_collars");
    for i in 0..SEPTUM_PORT_COUNT {
        let outer = centered_cylinder(
            format!("growth_factor_cytokine_septum_port_collar_outer_{i}"),
            SEPTUM_COLLAR_D / 2.0,
            12.0,
            36,
        )
        .translate(port_x(i), 20.0, PORT_PANEL_Z / 2.0 + 6.0);
        let inner = centered_cylinder(
            format!("growth_factor_cytokine_septum_port_collar_inner_{i}"),
            SEPTUM_PORT_D / 2.0,
            14.0,
            36,
        )
        .translate(port_x(i), 20.0, PORT_PANEL_Z / 2.0 + 6.0);
        let id_land = centered_cube(
            format!("growth_factor_cytokine_septum_port_id_land_{i}"),
            32.0,
            12.0,
            3.0,
        )
        .translate(port_x(i), 58.0, PORT_PANEL_Z / 2.0 + 2.0);
        collars = collars + (outer - inner) + id_land;
    }
    collars
}

fn septum_lid_parking_fence() -> Part {
    let rear = centered_cube(
        "growth_factor_cytokine_septum_lid_rear_parking_fence",
        PORT_PANEL_X - 38.0,
        12.0,
        72.0,
    )
    .translate(0.0, PORT_PANEL_Y / 2.0 - 24.0, PORT_PANEL_Z / 2.0 + 36.0);
    let left = centered_cube(
        "growth_factor_cytokine_septum_lid_left_stop",
        12.0,
        74.0,
        52.0,
    )
    .translate(
        -PORT_PANEL_X / 2.0 + 24.0,
        PORT_PANEL_Y / 2.0 - 58.0,
        PORT_PANEL_Z / 2.0 + 26.0,
    );
    let right = centered_cube(
        "growth_factor_cytokine_septum_lid_right_stop",
        12.0,
        74.0,
        52.0,
    )
    .translate(
        PORT_PANEL_X / 2.0 - 24.0,
        PORT_PANEL_Y / 2.0 - 58.0,
        PORT_PANEL_Z / 2.0 + 26.0,
    );
    rear + left + right
}

fn transfer_port_tube_combs() -> Part {
    let mut comb = Part::empty("growth_factor_cytokine_transfer_tube_combs");
    for i in 0..SEPTUM_PORT_COUNT {
        let x = port_x(i);
        comb = comb
            + centered_cube(
                format!("growth_factor_cytokine_closed_tube_comb_slot_{i}"),
                18.0,
                74.0,
                18.0,
            )
            .translate(x, -PORT_PANEL_Y / 2.0 + 32.0, PORT_PANEL_Z / 2.0 + 9.0);
    }
    comb
}

fn micro_dose_metering_witness() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_micro_dose_witness_body",
        METER_X,
        METER_Y,
        METER_Z,
    );
    let balance_recess = centered_cube(
        "growth_factor_cytokine_micro_balance_recess",
        MICRO_BALANCE_PAD_X,
        MICRO_BALANCE_PAD_Y,
        10.0,
    )
    .translate(-70.0, 48.0, METER_Z / 2.0 - 5.0);

    body - balance_recess - witness_well_cuts() - micro_dose_channel_cuts()
        + witness_well_collars()
        + micro_balance_edge_rails()
        + metering_camera_fiducials()
}

fn witness_well_cuts() -> Part {
    let mut wells = Part::empty("growth_factor_cytokine_witness_well_cuts");
    for i in 0..WITNESS_WELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("growth_factor_cytokine_micro_dose_witness_well_cut_{i}"),
                WITNESS_WELL_D / 2.0,
                METER_Z + 4.0,
                28,
            )
            .translate(witness_x(i), -54.0, 2.0);
    }
    wells
}

fn witness_well_collars() -> Part {
    let mut collars = Part::empty("growth_factor_cytokine_witness_well_collars");
    for i in 0..WITNESS_WELL_COUNT {
        let collar = centered_cylinder(
            format!("growth_factor_cytokine_micro_dose_witness_collar_{i}"),
            (WITNESS_WELL_D + 8.0) / 2.0,
            7.0,
            28,
        )
        .translate(witness_x(i), -54.0, METER_Z / 2.0 + 3.5);
        let bore = centered_cylinder(
            format!("growth_factor_cytokine_micro_dose_witness_collar_bore_{i}"),
            WITNESS_WELL_D / 2.0,
            8.0,
            28,
        )
        .translate(witness_x(i), -54.0, METER_Z / 2.0 + 3.5);
        collars = collars + (collar - bore);
    }
    collars
}

fn micro_dose_channel_cuts() -> Part {
    let mut channels = Part::empty("growth_factor_cytokine_micro_dose_channel_cuts");
    for i in 0..MICRO_DOSE_CHANNEL_COUNT {
        channels = channels
            + centered_cube(
                format!("growth_factor_cytokine_micro_dose_gravimetric_channel_{i}"),
                16.0,
                92.0,
                6.0,
            )
            .translate(witness_x(i), -2.0, METER_Z / 2.0 - 3.0);
    }
    channels
}

fn micro_balance_edge_rails() -> Part {
    let front = centered_cube(
        "growth_factor_cytokine_micro_balance_front_edge_rail",
        MICRO_BALANCE_PAD_X + 18.0,
        8.0,
        18.0,
    )
    .translate(
        -70.0,
        48.0 - MICRO_BALANCE_PAD_Y / 2.0 - 8.0,
        METER_Z / 2.0 + 9.0,
    );
    let rear = centered_cube(
        "growth_factor_cytokine_micro_balance_rear_edge_rail",
        MICRO_BALANCE_PAD_X + 18.0,
        8.0,
        18.0,
    )
    .translate(
        -70.0,
        48.0 + MICRO_BALANCE_PAD_Y / 2.0 + 8.0,
        METER_Z / 2.0 + 9.0,
    );
    let witness_label_strip = centered_cube(
        "growth_factor_cytokine_micro_dose_witness_label_strip",
        METER_X - 42.0,
        18.0,
        4.0,
    )
    .translate(0.0, -METER_Y / 2.0 + 22.0, METER_Z / 2.0 + 2.0);
    front + rear + witness_label_strip
}

fn metering_camera_fiducials() -> Part {
    let mut fiducials = Part::empty("growth_factor_cytokine_metering_camera_fiducials");
    for (i, (x, y)) in [
        (-METER_X / 2.0 + 28.0, METER_Y / 2.0 - 28.0),
        (METER_X / 2.0 - 28.0, METER_Y / 2.0 - 28.0),
        (-METER_X / 2.0 + 28.0, -METER_Y / 2.0 + 28.0),
        (METER_X / 2.0 - 28.0, -METER_Y / 2.0 + 28.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("growth_factor_cytokine_metering_fiducial_post_{i}"),
                5.0,
                8.0,
                24,
            )
            .translate(*x, *y, METER_Z / 2.0 + 4.0);
    }
    fiducials
}

fn mix_verification_pocket() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_mix_verification_body",
        MIX_X,
        MIX_Y,
        MIX_Z,
    );
    let cuvette = centered_cube(
        "growth_factor_cytokine_mix_cuvette_pocket_cut",
        MIX_CUVETTE_X,
        MIX_CUVETTE_Y,
        MIX_Z + 4.0,
    )
    .translate(-84.0, 22.0, 0.0);
    let sample_loop_cuts = mix_sample_loop_cuts();
    let sensor_window_cuts = mix_sensor_window_cuts();

    body - cuvette - sample_loop_cuts - sensor_window_cuts
        + static_mixer_witness_ridges()
        + mix_optical_bridge()
        + homogeneity_coupon_nests()
}

fn mix_sample_loop_cuts() -> Part {
    let mut loops = Part::empty("growth_factor_cytokine_mix_sample_loop_cuts");
    for i in 0..MIX_SAMPLE_LOOP_COUNT {
        loops = loops
            + centered_cube(
                format!("growth_factor_cytokine_mix_loop_channel_{i}"),
                188.0,
                10.0,
                9.0,
            )
            .translate(
                36.0,
                lane_x(i, MIX_SAMPLE_LOOP_COUNT, 26.0) - 28.0,
                MIX_Z / 2.0 - 4.5,
            );
    }
    loops
}

fn mix_sensor_window_cuts() -> Part {
    let mut windows = Part::empty("growth_factor_cytokine_mix_sensor_window_cuts");
    for i in 0..MIX_SENSOR_WINDOW_COUNT {
        windows = windows
            + centered_cube(
                format!("growth_factor_cytokine_mix_optical_window_cut_{i}"),
                12.0,
                96.0,
                32.0,
            )
            .translate(-130.0 + i as f64 * 92.0, 22.0, 0.0);
    }
    windows
}

fn static_mixer_witness_ridges() -> Part {
    let mut ridges = Part::empty("growth_factor_cytokine_static_mixer_witness_ridges");
    for i in 0..MIX_STATIC_MIXER_CHANNELS {
        ridges = ridges
            + centered_cube(
                format!("growth_factor_cytokine_static_mixer_chevron_witness_{i}"),
                86.0,
                8.0,
                10.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 22.0 } else { -22.0 })
            .translate(66.0, -84.0 + i as f64 * 22.0, MIX_Z / 2.0 + 5.0);
    }
    ridges
}

fn mix_optical_bridge() -> Part {
    let left_post = centered_cube(
        "growth_factor_cytokine_mix_optical_bridge_left_post",
        16.0,
        34.0,
        108.0,
    )
    .translate(-138.0, 22.0, MIX_Z / 2.0 + 54.0);
    let right_post = centered_cube(
        "growth_factor_cytokine_mix_optical_bridge_right_post",
        16.0,
        34.0,
        108.0,
    )
    .translate(-30.0, 22.0, MIX_Z / 2.0 + 54.0);
    let crossbar = centered_cube(
        "growth_factor_cytokine_mix_optical_bridge_crossbar",
        126.0,
        28.0,
        18.0,
    )
    .translate(-84.0, 22.0, MIX_Z / 2.0 + 108.0);
    left_post + right_post + crossbar
}

fn homogeneity_coupon_nests() -> Part {
    let mut nests = Part::empty("growth_factor_cytokine_homogeneity_coupon_nests");
    for i in 0..4 {
        let x = 104.0 + (i % 2) as f64 * 62.0;
        let y = 72.0 - (i / 2) as f64 * 58.0;
        nests = nests
            + centered_cube(
                format!("growth_factor_cytokine_homogeneity_coupon_retainer_{i}"),
                44.0,
                30.0,
                10.0,
            )
            .translate(x, y, MIX_Z / 2.0 + 5.0);
    }
    nests
}

fn label_lot_scan_lands() -> Part {
    let panel = centered_cube(
        "growth_factor_cytokine_label_lot_scan_panel",
        SCAN_X,
        SCAN_Y,
        SCAN_Z,
    );
    panel
        + label_scan_land_grid()
        + lot_scan_land_grid()
        + run_record_land_row()
        + scan_stanchions()
        + scanner_calibration_tiles()
}

fn label_scan_land_grid() -> Part {
    let mut lands = Part::empty("growth_factor_cytokine_label_scan_lands");
    for i in 0..LABEL_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("growth_factor_cytokine_additive_label_scan_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                3.0,
            )
            .translate(
                lane_x(i % 4, 4, 78.0),
                48.0 - (i / 4) as f64 * 34.0,
                SCAN_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn lot_scan_land_grid() -> Part {
    let mut lands = Part::empty("growth_factor_cytokine_lot_scan_lands");
    for i in 0..LOT_SCAN_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("growth_factor_cytokine_additive_lot_scan_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                3.0,
            )
            .translate(
                lane_x(i % 4, 4, 78.0),
                -24.0 - (i / 4) as f64 * 34.0,
                SCAN_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn run_record_land_row() -> Part {
    let mut lands = Part::empty("growth_factor_cytokine_run_record_lands");
    for i in 0..RUN_RECORD_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("growth_factor_cytokine_run_record_scan_land_{i}"),
                68.0,
                20.0,
                3.0,
            )
            .translate(
                lane_x(i, RUN_RECORD_LAND_COUNT, 78.0),
                -SCAN_Y / 2.0 + 16.0,
                SCAN_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn scan_stanchions() -> Part {
    let left = centered_cube(
        "growth_factor_cytokine_scan_bridge_left_stanchion",
        14.0,
        24.0,
        SCANNER_STANCHION_Z,
    )
    .translate(
        -SCAN_X / 2.0 + 30.0,
        0.0,
        SCAN_Z / 2.0 + SCANNER_STANCHION_Z / 2.0,
    );
    let right = centered_cube(
        "growth_factor_cytokine_scan_bridge_right_stanchion",
        14.0,
        24.0,
        SCANNER_STANCHION_Z,
    )
    .translate(
        SCAN_X / 2.0 - 30.0,
        0.0,
        SCAN_Z / 2.0 + SCANNER_STANCHION_Z / 2.0,
    );
    let bridge = centered_cube(
        "growth_factor_cytokine_label_lot_scanner_bridge",
        SCAN_X - 42.0,
        20.0,
        16.0,
    )
    .translate(0.0, 0.0, SCAN_Z / 2.0 + SCANNER_STANCHION_Z);
    left + right + bridge
}

fn scanner_calibration_tiles() -> Part {
    let white = centered_cube(
        "growth_factor_cytokine_scan_white_reference_tile_land",
        54.0,
        32.0,
        4.0,
    )
    .translate(
        -SCAN_X / 2.0 + 48.0,
        SCAN_Y / 2.0 - 32.0,
        SCAN_Z / 2.0 + 2.0,
    );
    let dark = centered_cube(
        "growth_factor_cytokine_scan_dark_reference_tile_land",
        54.0,
        32.0,
        4.0,
    )
    .translate(SCAN_X / 2.0 - 48.0, SCAN_Y / 2.0 - 32.0, SCAN_Z / 2.0 + 2.0);
    white + dark
}

fn cold_chain_logger_pocket() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_cold_chain_logger_body",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    body - logger_bay_cuts() - probe_channel_cuts()
        + logger_retention_lips()
        + cold_chain_token_wells()
        + probe_clip_posts()
}

fn logger_bay_cuts() -> Part {
    let mut cuts = Part::empty("growth_factor_cytokine_logger_bay_cuts");
    for i in 0..LOGGER_BAY_COUNT {
        cuts = cuts
            + centered_cube(
                format!("growth_factor_cytokine_cold_chain_logger_bay_cut_{i}"),
                LOGGER_BAY_X,
                LOGGER_BAY_Y,
                LOGGER_Z + 2.0,
            )
            .translate(lane_x(i, LOGGER_BAY_COUNT, 128.0), 36.0, 3.0);
    }
    cuts
}

fn probe_channel_cuts() -> Part {
    let mut channels = Part::empty("growth_factor_cytokine_logger_probe_channel_cuts");
    for i in 0..LOGGER_PROBE_CLIP_COUNT {
        channels = channels
            + centered_cube(
                format!("growth_factor_cytokine_cold_chain_probe_channel_{i}"),
                14.0,
                116.0,
                7.0,
            )
            .translate(
                lane_x(i, LOGGER_PROBE_CLIP_COUNT, 48.0),
                -28.0,
                LOGGER_Z / 2.0 - 3.5,
            );
    }
    channels
}

fn logger_retention_lips() -> Part {
    let mut lips = Part::empty("growth_factor_cytokine_logger_retention_lips");
    for i in 0..LOGGER_BAY_COUNT {
        let x = lane_x(i, LOGGER_BAY_COUNT, 128.0);
        lips = lips
            + centered_cube(
                format!("growth_factor_cytokine_logger_front_retention_lip_{i}"),
                LOGGER_BAY_X + 18.0,
                8.0,
                16.0,
            )
            .translate(x, 2.0, LOGGER_Z / 2.0 + 8.0)
            + centered_cube(
                format!("growth_factor_cytokine_logger_rear_retention_lip_{i}"),
                LOGGER_BAY_X + 18.0,
                8.0,
                16.0,
            )
            .translate(x, 72.0, LOGGER_Z / 2.0 + 8.0);
    }
    lips
}

fn cold_chain_token_wells() -> Part {
    let mut tokens = Part::empty("growth_factor_cytokine_cold_chain_token_wells");
    for i in 0..COLD_CHAIN_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("growth_factor_cytokine_cold_chain_token_land_{i}"),
                12.0,
                5.0,
                28,
            )
            .translate(
                lane_x(i, COLD_CHAIN_TOKEN_COUNT, 42.0),
                -LOGGER_Y / 2.0 + 34.0,
                LOGGER_Z / 2.0 + 2.5,
            );
    }
    tokens
}

fn probe_clip_posts() -> Part {
    let mut clips = Part::empty("growth_factor_cytokine_probe_clip_posts");
    for i in 0..LOGGER_PROBE_CLIP_COUNT {
        let x = lane_x(i, LOGGER_PROBE_CLIP_COUNT, 48.0);
        clips = clips
            + centered_cylinder(
                format!("growth_factor_cytokine_probe_clip_left_post_{i}"),
                4.0,
                20.0,
                20,
            )
            .translate(x - 10.0, -20.0, LOGGER_Z / 2.0 + 10.0)
            + centered_cylinder(
                format!("growth_factor_cytokine_probe_clip_right_post_{i}"),
                4.0,
                20.0,
                20,
            )
            .translate(x + 10.0, -20.0, LOGGER_Z / 2.0 + 10.0);
    }
    clips
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_release_hold_reject_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    body - status_slot_cuts()
        + status_lane_dividers()
        + decision_gate_tabs()
        + released_hold_reject_label_lands()
}

fn status_slot_cuts() -> Part {
    let mut slots = Part::empty("growth_factor_cytokine_status_lane_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let index = lane * STATUS_SLOTS_PER_LANE + slot;
            slots = slots
                + centered_cube(
                    format!("growth_factor_cytokine_status_lane_slot_cut_{index}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_Z + 2.0,
                )
                .translate(
                    lane_x(slot, STATUS_SLOTS_PER_LANE, 94.0),
                    lane_x(lane, STATUS_LANES, STATUS_LANE_PITCH_Y),
                    2.0,
                );
        }
    }
    slots
}

fn status_lane_dividers() -> Part {
    let upper = centered_cube(
        "growth_factor_cytokine_release_hold_divider",
        STATUS_X - 42.0,
        8.0,
        42.0,
    )
    .translate(0.0, STATUS_LANE_PITCH_Y / 2.0, STATUS_Z / 2.0 + 21.0);
    let lower = centered_cube(
        "growth_factor_cytokine_hold_reject_divider",
        STATUS_X - 42.0,
        8.0,
        42.0,
    )
    .translate(0.0, -STATUS_LANE_PITCH_Y / 2.0, STATUS_Z / 2.0 + 21.0);
    upper + lower
}

fn decision_gate_tabs() -> Part {
    let mut tabs = Part::empty("growth_factor_cytokine_decision_gate_tabs");
    for lane in 0..STATUS_LANES {
        tabs = tabs
            + centered_cube(
                format!("growth_factor_cytokine_decision_gate_pull_tab_{lane}"),
                32.0,
                44.0,
                14.0,
            )
            .translate(
                STATUS_X / 2.0 - 38.0,
                lane_x(lane, STATUS_LANES, STATUS_LANE_PITCH_Y),
                STATUS_Z / 2.0 + 7.0,
            );
    }
    tabs
}

fn released_hold_reject_label_lands() -> Part {
    let release = centered_cube(
        "growth_factor_cytokine_released_lane_label_land",
        78.0,
        20.0,
        3.0,
    )
    .translate(
        -STATUS_X / 2.0 + 56.0,
        STATUS_LANE_PITCH_Y,
        STATUS_Z / 2.0 + 1.5,
    );
    let hold = centered_cube(
        "growth_factor_cytokine_hold_lane_label_land",
        78.0,
        20.0,
        3.0,
    )
    .translate(-STATUS_X / 2.0 + 56.0, 0.0, STATUS_Z / 2.0 + 1.5);
    let reject = centered_cube(
        "growth_factor_cytokine_reject_lane_label_land",
        78.0,
        20.0,
        3.0,
    )
    .translate(
        -STATUS_X / 2.0 + 56.0,
        -STATUS_LANE_PITCH_Y,
        STATUS_Z / 2.0 + 1.5,
    );
    release + hold + reject
}

fn waste_capture() -> Part {
    let body = centered_cube(
        "growth_factor_cytokine_waste_capture_tray_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let tray_recess = centered_cube(
        "growth_factor_cytokine_waste_capture_recess_cut",
        WASTE_X - 44.0,
        WASTE_Y - 42.0,
        18.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 9.0);
    let drain = centered_cylinder(
        "growth_factor_cytokine_waste_drain_cut",
        WASTE_DRAIN_D / 2.0,
        WASTE_X - 72.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -WASTE_Y / 2.0 + 28.0, WASTE_Z / 2.0 - 11.0);

    body - tray_recess - drain - waste_well_cuts()
        + waste_well_rims()
        + waste_bag_cradle()
        + septum_wipe_disposal_slots()
}

fn waste_well_cuts() -> Part {
    let mut wells = Part::empty("growth_factor_cytokine_waste_well_cuts");
    for i in 0..WASTE_WELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("growth_factor_cytokine_waste_drip_well_cut_{i}"),
                WASTE_WELL_D / 2.0,
                WASTE_Z + 4.0,
                28,
            )
            .translate(
                lane_x(i % 4, 4, 42.0) - 58.0,
                44.0 - (i / 4) as f64 * 42.0,
                0.0,
            );
    }
    wells
}

fn waste_well_rims() -> Part {
    let mut rims = Part::empty("growth_factor_cytokine_waste_well_rims");
    for i in 0..WASTE_WELL_COUNT {
        let x = lane_x(i % 4, 4, 42.0) - 58.0;
        let y = 44.0 - (i / 4) as f64 * 42.0;
        let outer = centered_cylinder(
            format!("growth_factor_cytokine_waste_drip_well_rim_outer_{i}"),
            (WASTE_WELL_D + 8.0) / 2.0,
            7.0,
            28,
        )
        .translate(x, y, WASTE_Z / 2.0 + 3.5);
        let inner = centered_cylinder(
            format!("growth_factor_cytokine_waste_drip_well_rim_inner_{i}"),
            WASTE_WELL_D / 2.0,
            8.0,
            28,
        )
        .translate(x, y, WASTE_Z / 2.0 + 3.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn waste_bag_cradle() -> Part {
    let cradle = centered_cube(
        "growth_factor_cytokine_waste_bag_cradle_saddle",
        WASTE_BAG_CRADLE_X,
        WASTE_BAG_CRADLE_Y,
        28.0,
    )
    .translate(72.0, -24.0, WASTE_Z / 2.0 + 14.0);
    let strap_front = centered_cube(
        "growth_factor_cytokine_waste_bag_front_strap_land",
        WASTE_BAG_CRADLE_X + 18.0,
        8.0,
        18.0,
    )
    .translate(72.0, -WASTE_Y / 2.0 + 36.0, WASTE_Z / 2.0 + 9.0);
    let strap_rear = centered_cube(
        "growth_factor_cytokine_waste_bag_rear_strap_land",
        WASTE_BAG_CRADLE_X + 18.0,
        8.0,
        18.0,
    )
    .translate(72.0, 26.0, WASTE_Z / 2.0 + 9.0);
    cradle + strap_front + strap_rear
}

fn septum_wipe_disposal_slots() -> Part {
    let mut slots = Part::empty("growth_factor_cytokine_septum_wipe_disposal_slots");
    for i in 0..4 {
        slots = slots
            + centered_cube(
                format!("growth_factor_cytokine_septum_wipe_slot_{i}"),
                24.0,
                54.0,
                18.0,
            )
            .translate(
                -WASTE_X / 2.0 + 34.0 + i as f64 * 32.0,
                -WASTE_Y / 2.0 + 42.0,
                WASTE_Z / 2.0 + 9.0,
            );
    }
    slots
}

fn robot_service_keepouts() -> Part {
    keepout_box(
        "growth_factor_cytokine_front_robot_approach_keepout",
        760.0,
        FRONT_ROBOT_APPROACH,
        ROBOT_Z_CLEARANCE,
        (
            130.0,
            -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0 + 48.0,
            DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
        ),
    ) + keepout_box(
        "growth_factor_cytokine_rear_service_keepout",
        1040.0,
        REAR_SERVICE_ACCESS,
        210.0,
        (
            0.0,
            DECK_Y / 2.0 + REAR_SERVICE_ACCESS / 2.0 - 52.0,
            DECK_Z + 105.0,
        ),
    ) + keepout_box(
        "growth_factor_cytokine_left_chiller_service_keepout",
        LEFT_CHILLER_SERVICE_ACCESS,
        420.0,
        180.0,
        (
            -DECK_X / 2.0 - LEFT_CHILLER_SERVICE_ACCESS / 2.0 + 44.0,
            36.0,
            DECK_Z + 90.0,
        ),
    ) + keepout_box(
        "growth_factor_cytokine_septum_lid_swing_keepout",
        430.0,
        SEPTUM_LID_SWING_CLEARANCE,
        180.0,
        (
            PORT_POS.0,
            PORT_POS.1 + PORT_PANEL_Y / 2.0 + SEPTUM_LID_SWING_CLEARANCE / 2.0 - 20.0,
            DECK_Z + 90.0,
        ),
    )
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64, center: (f64, f64, f64)) -> Part {
    let bottom_front = centered_cube(
        format!("{name}_bottom_front_beam"),
        x,
        KEEP_OUT_BEAM,
        KEEP_OUT_BEAM,
    )
    .translate(center.0, center.1 - y / 2.0, center.2 - z / 2.0);
    let bottom_rear = centered_cube(
        format!("{name}_bottom_rear_beam"),
        x,
        KEEP_OUT_BEAM,
        KEEP_OUT_BEAM,
    )
    .translate(center.0, center.1 + y / 2.0, center.2 - z / 2.0);
    let top_front = centered_cube(
        format!("{name}_top_front_beam"),
        x,
        KEEP_OUT_BEAM,
        KEEP_OUT_BEAM,
    )
    .translate(center.0, center.1 - y / 2.0, center.2 + z / 2.0);
    let top_rear = centered_cube(
        format!("{name}_top_rear_beam"),
        x,
        KEEP_OUT_BEAM,
        KEEP_OUT_BEAM,
    )
    .translate(center.0, center.1 + y / 2.0, center.2 + z / 2.0);
    let left_bottom = centered_cube(
        format!("{name}_left_bottom_beam"),
        KEEP_OUT_BEAM,
        y,
        KEEP_OUT_BEAM,
    )
    .translate(center.0 - x / 2.0, center.1, center.2 - z / 2.0);
    let right_bottom = centered_cube(
        format!("{name}_right_bottom_beam"),
        KEEP_OUT_BEAM,
        y,
        KEEP_OUT_BEAM,
    )
    .translate(center.0 + x / 2.0, center.1, center.2 - z / 2.0);
    let left_top = centered_cube(
        format!("{name}_left_top_beam"),
        KEEP_OUT_BEAM,
        y,
        KEEP_OUT_BEAM,
    )
    .translate(center.0 - x / 2.0, center.1, center.2 + z / 2.0);
    let right_top = centered_cube(
        format!("{name}_right_top_beam"),
        KEEP_OUT_BEAM,
        y,
        KEEP_OUT_BEAM,
    )
    .translate(center.0 + x / 2.0, center.1, center.2 + z / 2.0);
    let mut posts = Part::empty(format!("{name}_vertical_posts"));
    for (i, (px, py)) in [
        (center.0 - x / 2.0, center.1 - y / 2.0),
        (center.0 + x / 2.0, center.1 - y / 2.0),
        (center.0 - x / 2.0, center.1 + y / 2.0),
        (center.0 + x / 2.0, center.1 + y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{name}_vertical_corner_post_{i}"),
                KEEP_OUT_BEAM,
                KEEP_OUT_BEAM,
                z,
            )
            .translate(*px, *py, center.2);
    }

    bottom_front
        + bottom_rear
        + top_front
        + top_rear
        + left_bottom
        + right_bottom
        + left_top
        + right_top
        + posts
}

fn robot_pick_fiducials(prefix: &str, x_span: f64, y_span: f64, z_height: f64) -> Part {
    let mut fiducials = Part::empty(format!("{prefix}_robot_pick_fiducials"));
    for (i, (x, y)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(format!("{prefix}_fiducial_{i}"), 4.0, 5.0, 24).translate(
                x * 0.5 * 0.78 * x_span,
                y * 0.5 * 0.72 * y_span,
                z_height / 2.0 + 2.5,
            );
    }
    fiducials
}

fn additive_grid_xy(row: usize, col: usize) -> (f64, f64) {
    (
        lane_x(col, ADDITIVE_COLS, ADDITIVE_PITCH_X),
        row_y(row, ADDITIVE_ROWS, ADDITIVE_PITCH_Y) + 30.0,
    )
}

fn port_x(index: usize) -> f64 {
    lane_x(index, SEPTUM_PORT_COUNT, SEPTUM_PORT_PITCH)
}

fn witness_x(index: usize) -> f64 {
    lane_x(index, WITNESS_WELL_COUNT, WITNESS_PITCH_X)
}

fn row_y(row: usize, rows: usize, pitch: f64) -> f64 {
    -((rows as f64 - 1.0) * pitch) / 2.0 + row as f64 * pitch
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_scoped_unique_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_growth_factor_cytokine_addition_qc_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn closed_addition_interfaces_match_additive_capacity() {
        assert_eq!(ADDITIVE_NEST_COUNT, 8);
        assert_eq!(ADDITIVE_NEST_COUNT, SEPTUM_PORT_COUNT);
        assert_eq!(SEPTUM_PORT_COUNT, WITNESS_WELL_COUNT);
        assert_eq!(WITNESS_WELL_COUNT, WASTE_WELL_COUNT);
        assert_eq!(MICRO_DOSE_CHANNEL_COUNT, ADDITIVE_NEST_COUNT);
        assert!(SEPTUM_COLLAR_D > SEPTUM_PORT_D * 2.0);
    }

    #[test]
    fn layout_modules_fit_without_claiming_shared_space() {
        assert_layout_constraints();
        let rects = layout_rects();
        assert_eq!(rects.len(), 8);
        for rect in rects {
            assert!(rect.fits_inside(), "{} should fit deck", rect.name);
        }
    }

    #[test]
    fn traceability_and_release_decision_features_are_explicit() {
        assert!(REQUIRED_FEATURES.contains(&"label_lot_scan_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert_eq!(LABEL_LAND_COUNT, LOT_SCAN_LAND_COUNT);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE * STATUS_LANES, 12);
        assert!(RUN_RECORD_LAND_COUNT >= 4);
    }

    #[test]
    fn cold_chain_waste_and_keepout_clearances_are_dimensioned() {
        assert!(REQUIRED_FEATURES.contains(&"cold_chain_logger_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"waste_capture"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
        assert_eq!(LOGGER_BAY_COUNT, 2);
        assert!(LOGGER_BAY_X * (LOGGER_BAY_COUNT as f64) < LOGGER_X);
        assert!(WASTE_BAG_CRADLE_X < WASTE_X);
        assert!(FRONT_ROBOT_APPROACH > REAR_SERVICE_ACCESS);
        assert!(ROBOT_Z_CLEARANCE > SCANNER_STANCHION_Z + MIX_Z);
    }
}
