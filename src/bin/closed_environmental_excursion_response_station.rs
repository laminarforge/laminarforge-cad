use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental excursion response station for automated tissue-chip runs.
//
// Intent:
// - Receive a suspect sealed cassette/tote without exposing it to the open room.
// - Preserve environmental logger evidence, barcode/status identity, and run-record
//   handoff datums as physical features.
// - Provide closed sample/fraction handoff positions, quarantine cover envelope,
//   temperature recovery buffering, leak/condensate capture, and released/hold/reject
//   segregation lanes.
//
// This is product concept CAD for mechanical layout and automation planning. It is
// not a biological rescue protocol, release criterion, or GMP validation claim.

const OUTPUTS: &[&str] = &[
    "output/closed_environmental_excursion_response_station_leak_condensate_tray.stl",
    "output/closed_environmental_excursion_response_station_suspect_cassette_tote_dock.stl",
    "output/closed_environmental_excursion_response_station_environmental_logger_evidence_pocket.stl",
    "output/closed_environmental_excursion_response_station_deviation_barcode_status_lands.stl",
    "output/closed_environmental_excursion_response_station_sealed_sample_fraction_handoff.stl",
    "output/closed_environmental_excursion_response_station_quarantine_cover_envelope.stl",
    "output/closed_environmental_excursion_response_station_temperature_recovery_buffer.stl",
    "output/closed_environmental_excursion_response_station_released_hold_reject_lanes.stl",
    "output/closed_environmental_excursion_response_station_run_record_deviation_handoff_panel.stl",
    "output/closed_environmental_excursion_response_station_robot_service_keepouts.stl",
    "output/closed_environmental_excursion_response_station_assembly.stl",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 840.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 16.0;
const TRAY_RIM_Z: f64 = 32.0;
const LEAK_SUMP_DEPTH: f64 = 9.0;
const DRAIN_PORT_D: f64 = 12.0;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.0;

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITIONS: usize = CASSETTE_COLS * CASSETTE_ROWS;
const CASSETTE_GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 32.0;
const CASSETTE_MARGIN_Y: f64 = 30.0;
const CASSETTE_X: f64 = CASSETTE_COLS as f64 * REVC_CHIP_LENGTH
    + (CASSETTE_COLS as f64 - 1.0) * CASSETTE_GUTTER
    + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH
    + (CASSETTE_ROWS as f64 - 1.0) * CASSETTE_GUTTER
    + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = 52.0;

const TOTE_X: f64 = CASSETTE_X + 94.0;
const TOTE_Y: f64 = CASSETTE_Y + 84.0;
const TOTE_Z: f64 = 86.0;
const TOTE_GUIDE_CLEARANCE: f64 = 18.0;
const DOCK_X: f64 = TOTE_X + 2.0 * TOTE_GUIDE_CLEARANCE + 42.0;
const DOCK_Y: f64 = TOTE_Y + 2.0 * TOTE_GUIDE_CLEARANCE + 42.0;
const DOCK_Z: f64 = 52.0;
const DOCK_CENTER: (f64, f64) = (-235.0, 43.0);
const DOCK_RAIL_W: f64 = 14.0;
const DOCK_RAIL_Z: f64 = 38.0;
const DOCK_STOP_Y: f64 = 28.0;
const DOCK_CLAMP_COUNT: usize = 6;

const LOGGER_CENTER: (f64, f64) = (-430.0, -304.0);
const LOGGER_PANEL_X: f64 = 304.0;
const LOGGER_PANEL_Y: f64 = 164.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_SLOTS: usize = 4;
const LOGGER_SLOT_X: f64 = 54.0;
const LOGGER_SLOT_Y: f64 = 88.0;
const LOGGER_SLOT_DEPTH: f64 = 9.0;
const EVIDENCE_SEAL_LANDS: usize = 6;

const STATUS_CENTER: (f64, f64) = (-462.0, 350.0);
const STATUS_PANEL_X: f64 = 310.0;
const STATUS_PANEL_Y: f64 = 110.0;
const STATUS_PANEL_Z: f64 = 20.0;
const DEVIATION_BARCODE_LANDS: usize = 10;
const STATUS_TOKEN_LANDS: usize = 6;

const SAMPLE_CENTER: (f64, f64) = (370.0, -300.0);
const SAMPLE_PANEL_X: f64 = 372.0;
const SAMPLE_PANEL_Y: f64 = 174.0;
const SAMPLE_PANEL_Z: f64 = 44.0;
const SAMPLE_LOOP_LANES: usize = 8;
const FRACTION_HANDOFF_POSITIONS: usize = 4;
const SAMPLE_LANE_PITCH: f64 = 40.0;
const SAMPLE_PORT_D: f64 = 5.4;
const FRACTION_DOCK_D: f64 = 22.0;

const COVER_X: f64 = DOCK_X + 76.0;
const COVER_Y: f64 = DOCK_Y + 54.0;
const COVER_Z: f64 = 252.0;
const COVER_WALL_T: f64 = 10.0;
const COVER_GASKET_T: f64 = 8.0;
const COVER_FRONT_SLOT_X: f64 = 420.0;
const COVER_FRONT_SLOT_Z: f64 = 96.0;
const COVER_LATCH_COUNT: usize = 8;

const TEMP_CENTER: (f64, f64) = (-50.0, -306.0);
const TEMP_BUFFER_X: f64 = 402.0;
const TEMP_BUFFER_Y: f64 = 154.0;
const TEMP_BUFFER_Z: f64 = 58.0;
const TEMP_ZONE_COUNT: usize = 4;
const PCM_PACK_COUNT: usize = 6;
const AIR_DIFFUSER_SLOTS: usize = 9;
const TEMP_SENSOR_POCKETS: usize = 6;

const LANE_CENTER: (f64, f64) = (392.0, 126.0);
const LANE_PANEL_X: f64 = 402.0;
const LANE_PANEL_Y: f64 = 380.0;
const LANE_PANEL_Z: f64 = 30.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 86.0;
const STATUS_SLOT_Y: f64 = 54.0;
const STATUS_SLOT_DEPTH: f64 = 8.0;
const STATUS_LANE_PITCH_X: f64 = 122.0;
const STATUS_SLOT_PITCH_Y: f64 = 76.0;
const STATUS_LANE_SEGREGATION_MIN: f64 = 32.0;

const RUN_RECORD_CENTER: (f64, f64) = (430.0, 346.0);
const RUN_RECORD_PANEL_X: f64 = 338.0;
const RUN_RECORD_PANEL_Y: f64 = 106.0;
const RUN_RECORD_PANEL_Z: f64 = 36.0;
const DEVIATION_HANDOFF_PORTS: usize = 6;
const NETWORK_DOCKS: usize = 2;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 470.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 230.0;
const LEFT_COVER_SWING_KEEP_OUT_X: f64 = 310.0;
const RIGHT_DEVIATION_SERVICE_KEEP_OUT_X: f64 = 250.0;
const TOP_COVER_LIFT_CLEARANCE_Z: f64 = 320.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = leak_condensate_tray();
    export(OUTPUTS[0], &tray);

    let dock = suspect_cassette_tote_dock();
    export(OUTPUTS[1], &dock);

    let logger = environmental_logger_evidence_pocket();
    export(OUTPUTS[2], &logger);

    let status = deviation_barcode_status_lands();
    export(OUTPUTS[3], &status);

    let sample = sealed_sample_fraction_handoff();
    export(OUTPUTS[4], &sample);

    let cover = quarantine_cover_envelope();
    export(OUTPUTS[5], &cover);

    let temp = temperature_recovery_buffer();
    export(OUTPUTS[6], &temp);

    let lanes = released_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let run_record = run_record_deviation_handoff_panel();
    export(OUTPUTS[8], &run_record);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = tray
        + dock.translate(DOCK_CENTER.0, DOCK_CENTER.1, DECK_Z)
        + logger.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, DECK_Z)
        + status.translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z)
        + sample.translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, DECK_Z)
        + cover.translate(DOCK_CENTER.0, DOCK_CENTER.1, DECK_Z)
        + temp.translate(TEMP_CENTER.0, TEMP_CENTER.1, DECK_Z)
        + lanes.translate(LANE_CENTER.0, LANE_CENTER.1, DECK_Z)
        + run_record.translate(RUN_RECORD_CENTER.0, RUN_RECORD_CENTER.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed environmental excursion response station:");
    println!("  Deck/tray:                  {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm with leak/condensate sump and drain");
    println!(
        "  Suspect dock:               {DOCK_X:.1}mm x {DOCK_Y:.1}mm dock, sized for {CASSETTE_POSITIONS}-position cassette ({CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm x {CASSETTE_Z:.1}mm) inside a sealed tote"
    );
    println!("  Suspect tote envelope:      {TOTE_X:.1}mm x {TOTE_Y:.1}mm x {TOTE_Z:.1}mm with {TOTE_GUIDE_CLEARANCE:.0}mm guide clearance");
    println!(
        "  Evidence capture:           {LOGGER_SLOTS} logger pockets, {EVIDENCE_SEAL_LANDS} seal lands, {DEVIATION_BARCODE_LANDS} barcode lands, {STATUS_TOKEN_LANDS} status-token lands"
    );
    println!(
        "  Closed handoff:             {SAMPLE_LOOP_LANES} sample-loop lanes and {FRACTION_HANDOFF_POSITIONS} sealed fraction positions"
    );
    println!(
        "  Quarantine envelope:        {COVER_X:.1}mm x {COVER_Y:.1}mm x {COVER_Z:.0}mm cover with {COVER_LATCH_COUNT} latch pads and visible gasket path"
    );
    println!(
        "  Temperature buffer:         {TEMP_ZONE_COUNT} recovery zones, {PCM_PACK_COUNT} PCM pack lands, {TEMP_SENSOR_POCKETS} sensor pockets"
    );
    println!(
        "  Status segregation:         {STATUS_LANES} lanes x {STATUS_SLOTS_PER_LANE} slots for released/hold/reject physical status"
    );
    println!(
        "  Run-record handoff:         {DEVIATION_HANDOFF_PORTS} deviation ports, {NETWORK_DOCKS} data-dock envelopes, explicit barcode/status datums"
    );
    println!(
        "  Service keepouts:           front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, top cover lift {TOP_COVER_LIFT_CLEARANCE_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 11, "unexpected output part count");
    assert_eq!(
        CASSETTE_POSITIONS, 20,
        "station expects a 20-position cassette/tote"
    );
    assert!(fits_on_deck(DOCK_CENTER, COVER_X, COVER_Y, 5.0));
    assert!(fits_on_deck(
        LOGGER_CENTER,
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        10.0
    ));
    assert!(fits_on_deck(
        STATUS_CENTER,
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        10.0
    ));
    assert!(fits_on_deck(
        SAMPLE_CENTER,
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Y,
        10.0
    ));
    assert!(fits_on_deck(
        TEMP_CENTER,
        TEMP_BUFFER_X,
        TEMP_BUFFER_Y,
        10.0
    ));
    assert!(fits_on_deck(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y, 10.0));
    assert!(fits_on_deck(
        RUN_RECORD_CENTER,
        RUN_RECORD_PANEL_X,
        RUN_RECORD_PANEL_Y,
        10.0
    ));
    assert!(DOCK_X > TOTE_X + 2.0 * TOTE_GUIDE_CLEARANCE);
    assert!(DOCK_Y > TOTE_Y + 2.0 * TOTE_GUIDE_CLEARANCE);
    assert!(COVER_X > DOCK_X + 2.0 * COVER_GASKET_T);
    assert!(COVER_Y > DOCK_Y + 2.0 * COVER_GASKET_T);
    assert!(status_lane_gap() >= STATUS_LANE_SEGREGATION_MIN);
    assert!(
        horizontal_gap(
            rect(LOGGER_CENTER, LOGGER_PANEL_X, LOGGER_PANEL_Y),
            rect(TEMP_CENTER, TEMP_BUFFER_X, TEMP_BUFFER_Y)
        ) >= 20.0,
        "logger evidence pocket and temperature buffer are not physically separated"
    );
    assert!(
        horizontal_gap(
            rect(TEMP_CENTER, TEMP_BUFFER_X, TEMP_BUFFER_Y),
            rect(SAMPLE_CENTER, SAMPLE_PANEL_X, SAMPLE_PANEL_Y)
        ) >= 18.0,
        "temperature buffer and closed sample handoff are too close"
    );
}

fn leak_condensate_tray() -> Part {
    let deck = centered_cube(
        "closed_excursion_response_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let sump = centered_cube(
        "closed_excursion_response_recessed_leak_sump",
        DECK_X - 116.0,
        DECK_Y - 110.0,
        LEAK_SUMP_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0);

    let front_drain = centered_cylinder(
        "closed_excursion_response_front_condensate_drain_cut",
        DRAIN_PORT_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 18.0, DECK_Z - 7.0);

    deck - sump - front_drain
        + tray_rim()
        + drain_ribs()
        + module_datum_bosses()
        + wipe_gutter_arrows()
}

fn tray_rim() -> Part {
    let left = centered_cube(
        "closed_excursion_response_left_raised_tray_lip",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        -(DECK_X / 2.0 - TRAY_RIM_W / 2.0),
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_excursion_response_right_raised_tray_lip",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_excursion_response_front_raised_tray_lip",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - TRAY_RIM_W / 2.0),
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_excursion_response_rear_raised_tray_lip",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );

    left + right + front + rear
}

fn drain_ribs() -> Part {
    let mut ribs = Part::empty("closed_excursion_response_sloped_drain_ribs");
    for i in 0..9 {
        let x = -420.0 + i as f64 * 105.0;
        ribs = ribs
            + centered_cube(
                format!("closed_excursion_response_drain_rib_{i}"),
                64.0,
                7.0,
                5.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
            .translate(x, -340.0 + (i % 3) as f64 * 34.0, DECK_Z + 2.5);
    }
    ribs
}

fn module_datum_bosses() -> Part {
    let mut bosses = Part::empty("closed_excursion_response_module_datum_bosses");
    for (i, (x, y)) in [
        (-570.0, -352.0),
        (-570.0, 352.0),
        (570.0, -352.0),
        (570.0, 352.0),
        (-92.0, -352.0),
        (92.0, 352.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_excursion_response_mount_boss_{i}"),
            MOUNT_BOSS_D / 2.0,
            8.0,
            36,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        let hole = centered_cylinder(
            format!("closed_excursion_response_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            12.0,
            28,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn wipe_gutter_arrows() -> Part {
    let mut markers = Part::empty("closed_excursion_response_wipe_gutter_direction_markers");
    for i in 0..6 {
        markers = markers
            + centered_cube(
                format!("closed_excursion_response_wipe_marker_{i}"),
                38.0,
                4.0,
                1.8,
            )
            .rotate(0.0, 0.0, 35.0)
            .translate(-480.0 + i as f64 * 190.0, -382.0, DECK_Z + 1.0);
    }
    markers
}

fn suspect_cassette_tote_dock() -> Part {
    let base = centered_cube(
        "closed_excursion_suspect_tote_dock_base",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0);

    let tote_recess = centered_cube(
        "closed_excursion_suspect_tote_recess_cut",
        TOTE_X + 16.0,
        TOTE_Y + 16.0,
        17.0,
    )
    .translate(0.0, 0.0, DOCK_Z - 7.5);

    let cassette_recess = centered_cube(
        "closed_excursion_cassette_shadow_recess_cut",
        CASSETTE_X + 16.0,
        CASSETTE_Y + 16.0,
        11.0,
    )
    .translate(0.0, 0.0, DOCK_Z - 5.0);

    base - tote_recess - cassette_recess
        + tote_guide_rails()
        + cassette_position_grid_witness()
        + dock_end_stops()
        + dock_clamp_lands()
        + tote_handle_clearance_gauges()
}

fn tote_guide_rails() -> Part {
    let left = centered_cube(
        "closed_excursion_tote_left_datum_rail",
        DOCK_RAIL_W,
        TOTE_Y + 46.0,
        DOCK_RAIL_Z,
    )
    .translate(
        -(TOTE_X / 2.0 + TOTE_GUIDE_CLEARANCE / 2.0),
        0.0,
        DOCK_Z + DOCK_RAIL_Z / 2.0,
    );
    let right = centered_cube(
        "closed_excursion_tote_right_datum_rail",
        DOCK_RAIL_W,
        TOTE_Y + 46.0,
        DOCK_RAIL_Z,
    )
    .translate(
        TOTE_X / 2.0 + TOTE_GUIDE_CLEARANCE / 2.0,
        0.0,
        DOCK_Z + DOCK_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_excursion_tote_rear_datum_stop",
        TOTE_X + 2.0 * TOTE_GUIDE_CLEARANCE,
        DOCK_STOP_Y,
        DOCK_RAIL_Z,
    )
    .translate(
        0.0,
        TOTE_Y / 2.0 + DOCK_STOP_Y / 2.0,
        DOCK_Z + DOCK_RAIL_Z / 2.0,
    );
    let front_gasket_land = centered_cube(
        "closed_excursion_tote_front_transfer_gasket_land",
        TOTE_X + 38.0,
        18.0,
        18.0,
    )
    .translate(0.0, -(TOTE_Y / 2.0 + 22.0), DOCK_Z + 9.0);

    left + right + rear + front_gasket_land
}

fn cassette_position_grid_witness() -> Part {
    let mut witnesses = Part::empty("closed_excursion_cassette_position_grid_witness");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let x = cassette_grid_x(col);
            let y = cassette_grid_y(row);
            witnesses = witnesses
                + centered_cube(
                    format!("closed_excursion_cassette_witness_land_{col}_{row}"),
                    74.0,
                    36.0,
                    2.0,
                )
                .translate(x, y, DOCK_Z + 1.0);
        }
    }
    witnesses
}

fn dock_end_stops() -> Part {
    let left_front = centered_cylinder(
        "closed_excursion_left_front_tote_corner_stop",
        13.0,
        22.0,
        36,
    )
    .translate(-(TOTE_X / 2.0), -(TOTE_Y / 2.0), DOCK_Z + 11.0);
    let right_front = centered_cylinder(
        "closed_excursion_right_front_tote_corner_stop",
        13.0,
        22.0,
        36,
    )
    .translate(TOTE_X / 2.0, -(TOTE_Y / 2.0), DOCK_Z + 11.0);
    let left_rear = centered_cylinder(
        "closed_excursion_left_rear_tote_corner_stop",
        13.0,
        22.0,
        36,
    )
    .translate(-(TOTE_X / 2.0), TOTE_Y / 2.0, DOCK_Z + 11.0);
    let right_rear = centered_cylinder(
        "closed_excursion_right_rear_tote_corner_stop",
        13.0,
        22.0,
        36,
    )
    .translate(TOTE_X / 2.0, TOTE_Y / 2.0, DOCK_Z + 11.0);

    left_front + right_front + left_rear + right_rear
}

fn dock_clamp_lands() -> Part {
    let mut clamps = Part::empty("closed_excursion_tote_clamp_lands");
    for i in 0..DOCK_CLAMP_COUNT {
        let x =
            -DOCK_X / 2.0 + 122.0 + i as f64 * ((DOCK_X - 244.0) / (DOCK_CLAMP_COUNT - 1) as f64);
        let y = if i % 2 == 0 {
            -(DOCK_Y / 2.0 - 32.0)
        } else {
            DOCK_Y / 2.0 - 32.0
        };
        clamps = clamps
            + centered_cube(
                format!("closed_excursion_quarantine_cover_clamp_land_{i}"),
                58.0,
                28.0,
                9.0,
            )
            .translate(x, y, DOCK_Z + 4.5);
    }
    clamps
}

fn tote_handle_clearance_gauges() -> Part {
    let front = centered_cube(
        "closed_excursion_front_tote_handle_clearance_gauge",
        180.0,
        18.0,
        44.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 + 52.0, DOCK_Z + 22.0);
    let rear = centered_cube(
        "closed_excursion_rear_tote_handle_clearance_gauge",
        180.0,
        18.0,
        44.0,
    )
    .translate(0.0, DOCK_Y / 2.0 - 52.0, DOCK_Z + 22.0);
    front + rear
}

fn environmental_logger_evidence_pocket() -> Part {
    let panel = centered_cube(
        "closed_excursion_logger_evidence_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);

    let mut cuts = Part::empty("closed_excursion_logger_evidence_recess_cuts");
    for i in 0..LOGGER_SLOTS {
        let x = -((LOGGER_SLOTS - 1) as f64 * 64.0) / 2.0 + i as f64 * 64.0;
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_logger_recess_cut_{i}"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH,
            )
            .translate(x, -8.0, LOGGER_PANEL_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.2);
    }

    panel - cuts + logger_retention_bridges() + evidence_seal_lands() + witness_card_clip()
}

fn logger_retention_bridges() -> Part {
    let mut bridges = Part::empty("closed_excursion_logger_retention_bridges");
    for i in 0..LOGGER_SLOTS {
        let x = -((LOGGER_SLOTS - 1) as f64 * 64.0) / 2.0 + i as f64 * 64.0;
        bridges = bridges
            + centered_cube(
                format!("closed_excursion_logger_retention_bridge_{i}"),
                LOGGER_SLOT_X + 16.0,
                8.0,
                12.0,
            )
            .translate(x, 45.0, LOGGER_PANEL_Z + 6.0);
    }
    bridges
}

fn evidence_seal_lands() -> Part {
    let mut lands = Part::empty("closed_excursion_evidence_seal_lands");
    for i in 0..EVIDENCE_SEAL_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_excursion_tamper_seal_land_{i}"),
                38.0,
                12.0,
                2.5,
            )
            .translate(-120.0 + i as f64 * 48.0, -70.0, LOGGER_PANEL_Z + 1.25);
    }
    lands
}

fn witness_card_clip() -> Part {
    let clip = centered_cube(
        "closed_excursion_logger_witness_card_clip",
        250.0,
        15.0,
        14.0,
    )
    .translate(0.0, 70.0, LOGGER_PANEL_Z + 7.0);
    let finger_cut = centered_cube(
        "closed_excursion_logger_witness_card_finger_cut",
        62.0,
        18.0,
        16.0,
    )
    .translate(0.0, 70.0, LOGGER_PANEL_Z + 7.0);
    clip - finger_cut
}

fn deviation_barcode_status_lands() -> Part {
    let panel = centered_cube(
        "closed_excursion_deviation_barcode_status_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(0.0, 0.0, STATUS_PANEL_Z / 2.0);

    panel + barcode_lands() + status_token_lands() + deviation_flag_rail()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_excursion_deviation_barcode_lands");
    for i in 0..DEVIATION_BARCODE_LANDS {
        let row = i / 5;
        let col = i % 5;
        lands = lands
            + centered_cube(
                format!("closed_excursion_deviation_barcode_land_{i}"),
                48.0,
                14.0,
                2.0,
            )
            .translate(
                -104.0 + col as f64 * 52.0,
                -28.0 + row as f64 * 28.0,
                STATUS_PANEL_Z + 1.0,
            );
    }
    lands
}

fn status_token_lands() -> Part {
    let mut lands = Part::empty("closed_excursion_status_token_lands");
    for i in 0..STATUS_TOKEN_LANDS {
        lands = lands
            + centered_cylinder(
                format!("closed_excursion_status_token_recess_{i}"),
                12.0,
                5.0,
                36,
            )
            .translate(-125.0 + i as f64 * 50.0, 40.0, STATUS_PANEL_Z + 2.5);
    }
    lands
}

fn deviation_flag_rail() -> Part {
    centered_cube(
        "closed_excursion_deviation_hard_status_flag_rail",
        STATUS_PANEL_X - 44.0,
        9.0,
        11.0,
    )
    .translate(0.0, STATUS_PANEL_Y / 2.0 - 18.0, STATUS_PANEL_Z + 5.5)
}

fn sealed_sample_fraction_handoff() -> Part {
    let panel = centered_cube(
        "closed_excursion_sealed_sample_handoff_panel",
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Y,
        SAMPLE_PANEL_Z,
    )
    .translate(0.0, 0.0, SAMPLE_PANEL_Z / 2.0);

    let mut port_cuts = Part::empty("closed_excursion_closed_sample_port_cuts");
    for i in 0..SAMPLE_LOOP_LANES {
        let x = -((SAMPLE_LOOP_LANES - 1) as f64 * SAMPLE_LANE_PITCH) / 2.0
            + i as f64 * SAMPLE_LANE_PITCH;
        port_cuts = port_cuts
            + centered_cylinder(
                format!("closed_excursion_sample_loop_bore_{i}"),
                SAMPLE_PORT_D / 2.0,
                SAMPLE_PANEL_Z + 8.0,
                28,
            )
            .translate(x, 42.0, SAMPLE_PANEL_Z / 2.0);
    }

    panel - port_cuts + sample_loop_lands() + fraction_docks() + flush_waste_minifold()
}

fn sample_loop_lands() -> Part {
    let mut lands = Part::empty("closed_excursion_sample_loop_lands");
    for i in 0..SAMPLE_LOOP_LANES {
        let x = -((SAMPLE_LOOP_LANES - 1) as f64 * SAMPLE_LANE_PITCH) / 2.0
            + i as f64 * SAMPLE_LANE_PITCH;
        lands = lands
            + centered_cube(
                format!("closed_excursion_closed_sample_loop_land_{i}"),
                28.0,
                42.0,
                9.0,
            )
            .translate(x, 42.0, SAMPLE_PANEL_Z + 4.5);
    }
    lands
}

fn fraction_docks() -> Part {
    let mut docks = Part::empty("closed_excursion_fraction_handoff_docks");
    for i in 0..FRACTION_HANDOFF_POSITIONS {
        let x = -90.0 + i as f64 * 60.0;
        let base = centered_cylinder(
            format!("closed_excursion_fraction_dock_cup_{i}"),
            FRACTION_DOCK_D / 2.0,
            18.0,
            40,
        )
        .translate(x, -42.0, SAMPLE_PANEL_Z + 9.0);
        let bore = centered_cylinder(
            format!("closed_excursion_fraction_dock_bore_{i}"),
            6.0,
            22.0,
            28,
        )
        .translate(x, -42.0, SAMPLE_PANEL_Z + 9.0);
        docks = docks + (base - bore);
    }
    docks
}

fn flush_waste_minifold() -> Part {
    let bus = centered_cube(
        "closed_excursion_sample_flush_waste_minifold_bus",
        SAMPLE_PANEL_X - 72.0,
        14.0,
        18.0,
    )
    .translate(0.0, -72.0, SAMPLE_PANEL_Z + 9.0);
    let mut ports = Part::empty("closed_excursion_sample_flush_waste_ports");
    for i in 0..4 {
        ports = ports
            + centered_cylinder(
                format!("closed_excursion_sample_waste_drop_port_{i}"),
                6.5,
                18.0,
                28,
            )
            .translate(-120.0 + i as f64 * 80.0, -72.0, SAMPLE_PANEL_Z + 9.0);
    }
    bus + ports
}

fn quarantine_cover_envelope() -> Part {
    let outer = centered_cube(
        "closed_excursion_quarantine_cover_outer_envelope",
        COVER_X,
        COVER_Y,
        COVER_Z,
    )
    .translate(0.0, 0.0, COVER_Z / 2.0);
    let inner = centered_cube(
        "closed_excursion_quarantine_cover_inner_clearance",
        COVER_X - 2.0 * COVER_WALL_T,
        COVER_Y - 2.0 * COVER_WALL_T,
        COVER_Z - COVER_WALL_T,
    )
    .translate(0.0, 0.0, COVER_Z / 2.0 + COVER_WALL_T);
    let front_transfer_slot = centered_cube(
        "closed_excursion_quarantine_cover_front_transfer_slot",
        COVER_FRONT_SLOT_X,
        COVER_WALL_T + 6.0,
        COVER_FRONT_SLOT_Z,
    )
    .translate(0.0, -COVER_Y / 2.0, 86.0);
    let logger_window = centered_cube(
        "closed_excursion_quarantine_cover_logger_evidence_window",
        180.0,
        COVER_WALL_T + 6.0,
        56.0,
    )
    .translate(-220.0, COVER_Y / 2.0, 134.0);

    outer - inner - front_transfer_slot - logger_window
        + cover_gasket_frame()
        + cover_latch_pads()
        + cover_lift_handles()
        + cover_pressure_indicator_land()
}

fn cover_gasket_frame() -> Part {
    let front = centered_cube(
        "closed_excursion_quarantine_cover_front_gasket_land",
        COVER_X - 48.0,
        COVER_GASKET_T,
        COVER_GASKET_T,
    )
    .translate(
        0.0,
        -COVER_Y / 2.0 + COVER_GASKET_T / 2.0,
        COVER_GASKET_T / 2.0,
    );
    let rear = centered_cube(
        "closed_excursion_quarantine_cover_rear_gasket_land",
        COVER_X - 48.0,
        COVER_GASKET_T,
        COVER_GASKET_T,
    )
    .translate(
        0.0,
        COVER_Y / 2.0 - COVER_GASKET_T / 2.0,
        COVER_GASKET_T / 2.0,
    );
    let left = centered_cube(
        "closed_excursion_quarantine_cover_left_gasket_land",
        COVER_GASKET_T,
        COVER_Y - 48.0,
        COVER_GASKET_T,
    )
    .translate(
        -COVER_X / 2.0 + COVER_GASKET_T / 2.0,
        0.0,
        COVER_GASKET_T / 2.0,
    );
    let right = centered_cube(
        "closed_excursion_quarantine_cover_right_gasket_land",
        COVER_GASKET_T,
        COVER_Y - 48.0,
        COVER_GASKET_T,
    )
    .translate(
        COVER_X / 2.0 - COVER_GASKET_T / 2.0,
        0.0,
        COVER_GASKET_T / 2.0,
    );
    front + rear + left + right
}

fn cover_latch_pads() -> Part {
    let mut pads = Part::empty("closed_excursion_quarantine_cover_latch_pads");
    for i in 0..COVER_LATCH_COUNT {
        let side_index = i % 4;
        let pair_index = i / 4;
        let (x, y) = match side_index {
            0 => (
                -COVER_X / 2.0 + 78.0 + pair_index as f64 * 140.0,
                -COVER_Y / 2.0 + 18.0,
            ),
            1 => (
                COVER_X / 2.0 - 78.0 - pair_index as f64 * 140.0,
                -COVER_Y / 2.0 + 18.0,
            ),
            2 => (
                -COVER_X / 2.0 + 78.0 + pair_index as f64 * 140.0,
                COVER_Y / 2.0 - 18.0,
            ),
            _ => (
                COVER_X / 2.0 - 78.0 - pair_index as f64 * 140.0,
                COVER_Y / 2.0 - 18.0,
            ),
        };
        pads = pads
            + centered_cube(
                format!("closed_excursion_cover_latch_pad_{i}"),
                58.0,
                22.0,
                12.0,
            )
            .translate(x, y, 30.0);
    }
    pads
}

fn cover_lift_handles() -> Part {
    let left = handle_loop("closed_excursion_cover_left_lift_handle").translate(
        -220.0,
        0.0,
        COVER_Z + 18.0,
    );
    let right = handle_loop("closed_excursion_cover_right_lift_handle").translate(
        220.0,
        0.0,
        COVER_Z + 18.0,
    );
    left + right
}

fn handle_loop(name: &str) -> Part {
    let base = centered_cube(format!("{name}_base"), 132.0, 32.0, 22.0).translate(0.0, 0.0, 0.0);
    let cut = centered_cube(format!("{name}_grip_cut"), 86.0, 36.0, 12.0).translate(0.0, 0.0, 0.0);
    base - cut
}

fn cover_pressure_indicator_land() -> Part {
    centered_cylinder(
        "closed_excursion_cover_pressure_indicator_land",
        22.0,
        8.0,
        40,
    )
    .translate(COVER_X / 2.0 - 72.0, COVER_Y / 2.0 - 72.0, COVER_Z + 4.0)
}

fn temperature_recovery_buffer() -> Part {
    let block = centered_cube(
        "closed_excursion_temperature_recovery_buffer_block",
        TEMP_BUFFER_X,
        TEMP_BUFFER_Y,
        TEMP_BUFFER_Z,
    )
    .translate(0.0, 0.0, TEMP_BUFFER_Z / 2.0);

    let mut cuts = Part::empty("closed_excursion_temperature_buffer_air_slot_cuts");
    for i in 0..AIR_DIFFUSER_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_temperature_air_diffuser_slot_{i}"),
                18.0,
                TEMP_BUFFER_Y + 4.0,
                8.0,
            )
            .translate(
                -((AIR_DIFFUSER_SLOTS - 1) as f64 * 36.0) / 2.0 + i as f64 * 36.0,
                0.0,
                TEMP_BUFFER_Z - 8.0,
            );
    }

    block - cuts + thermal_zone_lands() + pcm_pack_lands() + temp_sensor_pockets()
}

fn thermal_zone_lands() -> Part {
    let mut zones = Part::empty("closed_excursion_temperature_recovery_zone_lands");
    for i in 0..TEMP_ZONE_COUNT {
        zones = zones
            + centered_cube(
                format!("closed_excursion_temperature_zone_land_{i}"),
                78.0,
                62.0,
                6.0,
            )
            .translate(-132.0 + i as f64 * 88.0, 36.0, TEMP_BUFFER_Z + 3.0);
    }
    zones
}

fn pcm_pack_lands() -> Part {
    let mut lands = Part::empty("closed_excursion_pcm_pack_lands");
    for i in 0..PCM_PACK_COUNT {
        let row = i / 3;
        let col = i % 3;
        lands = lands
            + centered_cube(
                format!("closed_excursion_pcm_pack_land_{i}"),
                88.0,
                30.0,
                5.0,
            )
            .translate(
                -104.0 + col as f64 * 104.0,
                -42.0 + row as f64 * 42.0,
                TEMP_BUFFER_Z + 2.5,
            );
    }
    lands
}

fn temp_sensor_pockets() -> Part {
    let mut pockets = Part::empty("closed_excursion_temperature_sensor_pockets");
    for i in 0..TEMP_SENSOR_POCKETS {
        pockets = pockets
            + centered_cylinder(
                format!("closed_excursion_temperature_sensor_pocket_{i}"),
                4.0,
                12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(-150.0 + i as f64 * 60.0, TEMP_BUFFER_Y / 2.0 - 12.0, 30.0);
    }
    pockets
}

fn released_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_excursion_released_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0);

    let mut slot_cuts = Part::empty("closed_excursion_status_lane_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_excursion_status_lane_{lane}_slot_{slot}_cut"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_SLOT_DEPTH,
                )
                .translate(
                    lane_x(lane),
                    lane_slot_y(slot),
                    LANE_PANEL_Z - STATUS_SLOT_DEPTH / 2.0 + 0.2,
                );
        }
    }

    panel - slot_cuts + lane_dividers() + lane_status_header_lands() + lane_front_stop_bar()
}

fn lane_dividers() -> Part {
    let mut dividers = Part::empty("closed_excursion_status_lane_dividers");
    for i in 0..(STATUS_LANES - 1) {
        let x = (lane_x(i) + lane_x(i + 1)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("closed_excursion_status_lane_divider_{i}"),
                10.0,
                LANE_PANEL_Y - 42.0,
                24.0,
            )
            .translate(x, 0.0, LANE_PANEL_Z + 12.0);
    }
    dividers
}

fn lane_status_header_lands() -> Part {
    let mut headers = Part::empty("closed_excursion_status_lane_header_lands");
    for lane in 0..STATUS_LANES {
        headers = headers
            + centered_cube(
                format!("closed_excursion_status_lane_header_{lane}"),
                STATUS_SLOT_X,
                20.0,
                4.0,
            )
            .translate(lane_x(lane), LANE_PANEL_Y / 2.0 - 26.0, LANE_PANEL_Z + 2.0);
    }
    headers
}

fn lane_front_stop_bar() -> Part {
    centered_cube(
        "closed_excursion_status_lane_front_stop_bar",
        LANE_PANEL_X - 44.0,
        12.0,
        18.0,
    )
    .translate(0.0, -LANE_PANEL_Y / 2.0 + 18.0, LANE_PANEL_Z + 9.0)
}

fn run_record_deviation_handoff_panel() -> Part {
    let panel = centered_cube(
        "closed_excursion_run_record_deviation_handoff_panel",
        RUN_RECORD_PANEL_X,
        RUN_RECORD_PANEL_Y,
        RUN_RECORD_PANEL_Z,
    )
    .translate(0.0, 0.0, RUN_RECORD_PANEL_Z / 2.0);

    panel + deviation_handoff_ports() + network_data_docks() + hardcopy_witness_slot()
}

fn deviation_handoff_ports() -> Part {
    let mut ports = Part::empty("closed_excursion_deviation_handoff_ports");
    for i in 0..DEVIATION_HANDOFF_PORTS {
        ports = ports
            + centered_cylinder(
                format!("closed_excursion_deviation_handoff_port_{i}"),
                10.0,
                12.0,
                36,
            )
            .translate(-118.0 + i as f64 * 47.0, 16.0, RUN_RECORD_PANEL_Z + 6.0);
    }
    ports
}

fn network_data_docks() -> Part {
    let mut docks = Part::empty("closed_excursion_run_record_data_docks");
    for i in 0..NETWORK_DOCKS {
        docks = docks
            + centered_cube(
                format!("closed_excursion_run_record_data_dock_{i}"),
                78.0,
                30.0,
                14.0,
            )
            .translate(-54.0 + i as f64 * 108.0, -28.0, RUN_RECORD_PANEL_Z + 7.0);
    }
    docks
}

fn hardcopy_witness_slot() -> Part {
    centered_cube(
        "closed_excursion_run_record_witness_card_slot",
        RUN_RECORD_PANEL_X - 56.0,
        9.0,
        16.0,
    )
    .translate(
        0.0,
        -RUN_RECORD_PANEL_Y / 2.0 + 14.0,
        RUN_RECORD_PANEL_Z + 8.0,
    )
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_excursion_front_robot_approach_keepout",
        DECK_X,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0),
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_excursion_rear_service_keepout",
        DECK_X,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left_cover = centered_cube(
        "closed_excursion_left_cover_swing_keepout",
        LEFT_COVER_SWING_KEEP_OUT_X,
        DECK_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -(DECK_X / 2.0 + LEFT_COVER_SWING_KEEP_OUT_X / 2.0),
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right_deviation_service = centered_cube(
        "closed_excursion_right_deviation_service_keepout",
        RIGHT_DEVIATION_SERVICE_KEEP_OUT_X,
        DECK_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_DEVIATION_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top_cover = centered_cube(
        "closed_excursion_top_quarantine_cover_lift_keepout",
        COVER_X,
        COVER_Y,
        TOP_COVER_LIFT_CLEARANCE_Z,
    )
    .translate(
        DOCK_CENTER.0,
        DOCK_CENTER.1,
        COVER_Z + TOP_COVER_LIFT_CLEARANCE_Z / 2.0,
    );

    front_robot + rear_service + left_cover + right_deviation_service + top_cover
}

fn cassette_grid_x(col: usize) -> f64 {
    -((CASSETTE_COLS - 1) as f64 * (REVC_CHIP_LENGTH + CASSETTE_GUTTER)) / 2.0
        + col as f64 * (REVC_CHIP_LENGTH + CASSETTE_GUTTER)
}

fn cassette_grid_y(row: usize) -> f64 {
    -((CASSETTE_ROWS - 1) as f64 * (REVC_CHIP_WIDTH + CASSETTE_GUTTER)) / 2.0
        + row as f64 * (REVC_CHIP_WIDTH + CASSETTE_GUTTER)
}

fn lane_x(lane: usize) -> f64 {
    -STATUS_LANE_PITCH_X + lane as f64 * STATUS_LANE_PITCH_X
}

fn lane_slot_y(slot: usize) -> f64 {
    -((STATUS_SLOTS_PER_LANE - 1) as f64 * STATUS_SLOT_PITCH_Y) / 2.0
        + slot as f64 * STATUS_SLOT_PITCH_Y
}

fn status_lane_gap() -> f64 {
    STATUS_LANE_PITCH_X - STATUS_SLOT_X
}

fn fits_on_deck(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + width / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= DECK_Y / 2.0 - margin
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn horizontal_gap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> f64 {
    if a.1 < b.0 {
        b.0 - a.1
    } else if b.1 < a.0 {
        a.0 - b.1
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_has_expected_parts() {
        assert_eq!(OUTPUTS.len(), 11);
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS[0].contains("leak_condensate_tray"));
        assert!(OUTPUTS[10].ends_with("_assembly.stl"));
    }

    #[test]
    fn suspect_dock_fits_cassette_and_sealed_tote() {
        assert_eq!(CASSETTE_POSITIONS, 20);
        assert!(CASSETTE_X > 580.0);
        assert!(CASSETTE_Y > 500.0);
        assert!(DOCK_X > TOTE_X + 2.0 * TOTE_GUIDE_CLEARANCE);
        assert!(DOCK_Y > TOTE_Y + 2.0 * TOTE_GUIDE_CLEARANCE);
        assert!(COVER_X > DOCK_X + 2.0 * COVER_GASKET_T);
        assert!(COVER_Z > TOTE_Z + 120.0);
    }

    #[test]
    fn evidence_and_traceability_features_are_counted() {
        assert_eq!(LOGGER_SLOTS, 4);
        assert_eq!(EVIDENCE_SEAL_LANDS, 6);
        assert_eq!(DEVIATION_BARCODE_LANDS, 10);
        assert_eq!(STATUS_TOKEN_LANDS, 6);
        assert_eq!(DEVIATION_HANDOFF_PORTS, 6);
    }

    #[test]
    fn closed_sample_handoff_has_enough_positions() {
        assert_eq!(SAMPLE_LOOP_LANES, 8);
        assert_eq!(FRACTION_HANDOFF_POSITIONS, 4);
        assert!(SAMPLE_LANE_PITCH >= 34.0);
        assert!(FRACTION_DOCK_D >= 20.0);
    }

    #[test]
    fn released_hold_reject_lanes_are_physically_segregated() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert!(status_lane_gap() >= STATUS_LANE_SEGREGATION_MIN);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 12);
    }

    #[test]
    fn major_subassemblies_fit_on_cleanable_tray() {
        assert_layout();
    }

    #[test]
    fn service_clearances_are_explicitly_modeled() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 420.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 200.0);
        assert!(LEFT_COVER_SWING_KEEP_OUT_X >= 300.0);
        assert!(TOP_COVER_LIFT_CLEARANCE_Z >= COVER_Z);
    }
}
