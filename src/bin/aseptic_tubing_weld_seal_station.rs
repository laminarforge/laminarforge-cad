use vcad::{centered_cube, centered_cylinder, Part};

// Aseptic tubing weld/seal and connector-prep station for closed cell-culture
// automation.
//
// Research assumptions used for placeholder geometry:
// - Commercial sterile tube welders/sealers support closed processing for wet,
//   dry, or liquid-filled tubing and common TPE/PVC bioprocess tubing.
// - Published Biosealer TC data lists 1/4 in to 1 in OD tubing, 2 to 4 minute
//   sealing cycles, and a 391 mm x 115 mm x 147 mm portable sealer envelope.
// - Terumo TSCD-II material describes heated wafers, closed-system welding,
//   automatic wafer disposal, and weld integrity checks for pressure/tensile
//   qualification. This CAD keeps the purchased process equipment as envelope
//   placeholders and models the automation datums around it.

const OUTPUTS: [&str; 10] = [
    "output/aseptic_tubing_weld_seal_station_deck.stl",
    "output/aseptic_tubing_weld_seal_station_equipment_envelopes.stl",
    "output/aseptic_tubing_weld_seal_station_spool_cassette_datum.stl",
    "output/aseptic_tubing_weld_seal_station_cut_weld_seal_lanes.stl",
    "output/aseptic_tubing_weld_seal_station_connector_cap_staging.stl",
    "output/aseptic_tubing_weld_seal_station_leak_test_handoff_ports.stl",
    "output/aseptic_tubing_weld_seal_station_segregation_waste_trays.stl",
    "output/aseptic_tubing_weld_seal_station_barcode_lot_lands.stl",
    "output/aseptic_tubing_weld_seal_station_cleanability_robot_keepouts.stl",
    "output/aseptic_tubing_weld_seal_station_assembly.stl",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;

const BIOSEALER_DATASHEET_X: f64 = 391.0;
const BIOSEALER_DATASHEET_Y: f64 = 115.0;
const BIOSEALER_DATASHEET_Z: f64 = 147.0;
const TUBE_OD_MIN: f64 = 6.35;
const TUBE_OD_MAX: f64 = 25.4;
const TUBE_CHANNEL_CLEARANCE: f64 = 2.0;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CHANNEL_CLEARANCE;

const WELDER_X: f64 = 530.0;
const WELDER_Y: f64 = 300.0;
const WELDER_Z: f64 = 280.0;
const WELDER_CENTER_X: f64 = -55.0;
const WELDER_CENTER_Y: f64 = 115.0;

const SEALER_X: f64 = 410.0;
const SEALER_Y: f64 = 150.0;
const SEALER_Z: f64 = 172.0;
const SEALER_CENTER_X: f64 = 245.0;
const SEALER_CENTER_Y: f64 = 112.0;

const SPOOL_CASSETTE_X: f64 = 250.0;
const SPOOL_CASSETTE_Y: f64 = 300.0;
const SPOOL_CASSETTE_Z: f64 = 38.0;
const SPOOL_CENTER_X: f64 = -500.0;
const SPOOL_CENTER_Y: f64 = 132.0;
const SPOOL_WELL_D: f64 = 126.0;
const CASSETTE_POCKET_X: f64 = 136.0;
const CASSETTE_POCKET_Y: f64 = 86.0;

const STAGE_RAIL_X: f64 = 890.0;
const STAGE_RAIL_Y: f64 = 240.0;
const STAGE_RAIL_Z: f64 = 28.0;
const STAGE_CENTER_X: f64 = -75.0;
const STAGE_CENTER_Y: f64 = -145.0;
const STAGE_LANES: usize = 4;
const STAGE_LANE_PITCH_Y: f64 = 44.0;
const CUT_WINDOW_X: f64 = -265.0;
const WELD_WINDOW_X: f64 = -70.0;
const SEAL_WINDOW_X: f64 = 170.0;

const CAP_STAGE_X: f64 = 250.0;
const CAP_STAGE_Y: f64 = 190.0;
const CAP_STAGE_Z: f64 = 24.0;
const CAP_STAGE_CENTER_X: f64 = 500.0;
const CAP_STAGE_CENTER_Y: f64 = -235.0;
const CAP_COLS: usize = 6;
const CAP_ROWS_PER_BANK: usize = 2;
const CAP_BANKS: usize = 2;
const CAP_WELL_D: f64 = 18.0;
const CAP_PITCH_X: f64 = 32.0;
const CAP_PITCH_Y: f64 = 32.0;

const LEAK_PORT_BAR_X: f64 = 400.0;
const LEAK_PORT_BAR_Y: f64 = 96.0;
const LEAK_PORT_BAR_Z: f64 = 28.0;
const LEAK_PORT_CENTER_X: f64 = 410.0;
const LEAK_PORT_CENTER_Y: f64 = 285.0;
const LEAK_PORT_COUNT: usize = 8;
const LEAK_PORT_D: f64 = 8.0;
const LEAK_PORT_PITCH_X: f64 = 42.0;

const SEGREGATION_DIVIDER_X: f64 = 352.0;
const SEGREGATION_DIVIDER_Y: f64 = 676.0;
const SEGREGATION_DIVIDER_W: f64 = 26.0;
const SEGREGATION_DIVIDER_Z: f64 = 64.0;
const SEGREGATION_AIR_GAP: f64 = 52.0;

const WASTE_TRAY_X: f64 = 245.0;
const WASTE_TRAY_Y: f64 = 186.0;
const WASTE_TRAY_Z: f64 = 46.0;
const WASTE_TRAY_CENTER_X: f64 = 500.0;
const WASTE_TRAY_CENTER_Y: f64 = -38.0;

const BARCODE_PANEL_X: f64 = 360.0;
const BARCODE_PANEL_Y: f64 = 96.0;
const BARCODE_PANEL_Z: f64 = 12.0;
const BARCODE_CENTER_X: f64 = -455.0;
const BARCODE_CENTER_Y: f64 = -320.0;
const LABEL_LAND_COUNT: usize = 8;

const VHP_CLEARANCE_Y: f64 = 70.0;
const VHP_CLEARANCE_CENTER_Y: f64 = 350.0;
const VHP_CLEARANCE_Z: f64 = 42.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 90.0;
const FRONT_ROBOT_KEEP_OUT_CENTER_Y: f64 = -365.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 60.0;
const SIDE_SERVICE_KEEP_OUT_CENTER_X: f64 = 610.0;
const ROBOT_APPROACH_CLEARANCE_X: f64 = 440.0;
const ROBOT_APPROACH_CLEARANCE_Z: f64 = 125.0;

fn main() {
    write_part(station_deck(), OUTPUTS[0]);
    write_part(tube_welder_sealer_equipment_envelopes(), OUTPUTS[1]);
    write_part(sterile_tubing_spool_cassette_datum(), OUTPUTS[2]);
    write_part(cut_weld_seal_staging_lanes(), OUTPUTS[3]);
    write_part(connector_cap_staging(), OUTPUTS[4]);
    write_part(leak_test_handoff_ports(), OUTPUTS[5]);
    write_part(clean_used_segregation_and_waste_trays(), OUTPUTS[6]);
    write_part(barcode_and_lot_lands(), OUTPUTS[7]);
    write_part(vhp_cleanability_and_robot_keepouts(), OUTPUTS[8]);
    write_part(station_assembly(), OUTPUTS[9]);

    println!(
        "Aseptic tubing weld/seal station: {:.0}mm x {:.0}mm deck, {:.0}-{:.0}mm OD tubing placeholders, {:.0}mm x {:.0}mm x {:.0}mm welder envelope, {:.0}mm x {:.0}mm x {:.0}mm sealer envelope, {} staging lanes, {} cap wells, {} leak-test ports, and {} barcode/lot lands.",
        DECK_X,
        DECK_Y,
        TUBE_OD_MIN,
        TUBE_OD_MAX,
        WELDER_X,
        WELDER_Y,
        WELDER_Z,
        SEALER_X,
        SEALER_Y,
        SEALER_Z,
        STAGE_LANES,
        connector_cap_well_count(),
        LEAK_PORT_COUNT,
        LABEL_LAND_COUNT
    );
    println!(
        "Research baseline: portable tube sealer datasheet envelope {:.0}mm x {:.0}mm x {:.0}mm; station sealer placeholder includes integration clearance around that purchased unit.",
        BIOSEALER_DATASHEET_X, BIOSEALER_DATASHEET_Y, BIOSEALER_DATASHEET_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    station_deck()
        + tube_welder_sealer_equipment_envelopes()
        + sterile_tubing_spool_cassette_datum()
        + cut_weld_seal_staging_lanes()
        + connector_cap_staging()
        + leak_test_handoff_ports()
        + clean_used_segregation_and_waste_trays()
        + barcode_and_lot_lands()
        + vhp_cleanability_and_robot_keepouts()
}

fn station_deck() -> Part {
    let deck = centered_cube("aseptic_tubing_station_deck", DECK_X, DECK_Y, DECK_Z);

    let mut recesses = Part::empty("aseptic_tubing_station_deck_recesses");
    for (i, (x, y, sx, sy)) in [
        (
            WELDER_CENTER_X,
            WELDER_CENTER_Y,
            WELDER_X + 36.0,
            WELDER_Y + 34.0,
        ),
        (
            SEALER_CENTER_X,
            SEALER_CENTER_Y,
            SEALER_X + 32.0,
            SEALER_Y + 28.0,
        ),
        (
            SPOOL_CENTER_X,
            SPOOL_CENTER_Y,
            SPOOL_CASSETTE_X + 26.0,
            SPOOL_CASSETTE_Y + 26.0,
        ),
        (
            STAGE_CENTER_X,
            STAGE_CENTER_Y,
            STAGE_RAIL_X + 28.0,
            STAGE_RAIL_Y + 24.0,
        ),
        (
            CAP_STAGE_CENTER_X,
            CAP_STAGE_CENTER_Y,
            CAP_STAGE_X + 24.0,
            CAP_STAGE_Y + 24.0,
        ),
        (
            LEAK_PORT_CENTER_X,
            LEAK_PORT_CENTER_Y,
            LEAK_PORT_BAR_X + 22.0,
            LEAK_PORT_BAR_Y + 22.0,
        ),
        (
            WASTE_TRAY_CENTER_X,
            WASTE_TRAY_CENTER_Y,
            WASTE_TRAY_X + 24.0,
            WASTE_TRAY_Y + 24.0,
        ),
        (
            BARCODE_CENTER_X,
            BARCODE_CENTER_Y,
            BARCODE_PANEL_X + 20.0,
            BARCODE_PANEL_Y + 18.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        recesses = recesses
            + centered_cube(
                format!("aseptic_tubing_station_deck_recess_{i}"),
                *sx,
                *sy,
                6.0,
            )
            .translate(*x, *y, DECK_Z / 2.0 - 2.8);
    }

    let mut mount_holes = Part::empty("aseptic_tubing_station_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("aseptic_tubing_station_m6_mount_{i}"),
                6.6 / 2.0,
                DECK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    deck - recesses - mount_holes + deck_perimeter_lips()
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "aseptic_tubing_station_rear_wipe_lip",
        DECK_X - 92.0,
        14.0,
        24.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 28.0, DECK_Z / 2.0 + 12.0);
    let left = centered_cube(
        "aseptic_tubing_station_left_wipe_lip",
        14.0,
        DECK_Y - 112.0,
        24.0,
    )
    .translate(-DECK_X / 2.0 + 28.0, 0.0, DECK_Z / 2.0 + 12.0);
    let low_front = centered_cube(
        "aseptic_tubing_station_front_low_robot_lip",
        DECK_X - 360.0,
        10.0,
        12.0,
    )
    .translate(-130.0, -DECK_Y / 2.0 + 24.0, DECK_Z / 2.0 + 6.0);

    rear + left + low_front
}

fn tube_welder_sealer_equipment_envelopes() -> Part {
    let welder = equipment_envelope(
        "aseptic_tubing_welder",
        WELDER_X,
        WELDER_Y,
        WELDER_Z,
        86.0,
        66.0,
    )
    .translate(
        WELDER_CENTER_X,
        WELDER_CENTER_Y,
        DECK_Z / 2.0 + WELDER_Z / 2.0,
    );

    let sealer = equipment_envelope(
        "aseptic_tubing_sealer",
        SEALER_X,
        SEALER_Y,
        SEALER_Z,
        68.0,
        44.0,
    )
    .translate(
        SEALER_CENTER_X,
        SEALER_CENTER_Y,
        DECK_Z / 2.0 + SEALER_Z / 2.0,
    );

    let welder_tube_throat = centered_cube(
        "aseptic_tubing_welder_robot_loading_throat_gauge",
        WELDER_X + 44.0,
        30.0,
        48.0,
    )
    .translate(
        WELDER_CENTER_X,
        WELDER_CENTER_Y - WELDER_Y / 2.0 - 18.0,
        DECK_Z / 2.0 + 90.0,
    );
    let sealer_tube_throat = centered_cube(
        "aseptic_tubing_sealer_robot_loading_throat_gauge",
        SEALER_X + 28.0,
        24.0,
        38.0,
    )
    .translate(
        SEALER_CENTER_X,
        SEALER_CENTER_Y - SEALER_Y / 2.0 - 15.0,
        DECK_Z / 2.0 + 68.0,
    );

    welder + sealer + welder_tube_throat + sealer_tube_throat
}

fn equipment_envelope(name: &str, x: f64, y: f64, z: f64, screen_x: f64, screen_z: f64) -> Part {
    let housing = centered_cube(format!("{name}_placeholder_housing"), x, y, z);
    let process_window = centered_cube(
        format!("{name}_front_tube_access_window"),
        x * 0.52,
        y + 4.0,
        z * 0.32,
    )
    .translate(0.0, -y / 2.0 + 22.0, -z * 0.10);
    let service_panel = centered_cube(
        format!("{name}_rear_service_panel_land"),
        x - 64.0,
        8.0,
        z * 0.44,
    )
    .translate(0.0, y / 2.0 + 4.0, 0.0);
    let screen = centered_cube(
        format!("{name}_touchscreen_placeholder"),
        screen_x,
        7.0,
        screen_z,
    )
    .translate(
        x / 2.0 - screen_x / 2.0 - 34.0,
        -y / 2.0 - 3.5,
        z / 2.0 - screen_z / 2.0 - 28.0,
    );

    let mut feet = Part::empty(format!("{name}_cleanability_feet"));
    for (i, (fx, fy)) in [
        (-(x / 2.0 - 42.0), -(y / 2.0 - 34.0)),
        (x / 2.0 - 42.0, -(y / 2.0 - 34.0)),
        (-(x / 2.0 - 42.0), y / 2.0 - 34.0),
        (x / 2.0 - 42.0, y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        feet = feet
            + centered_cylinder(format!("{name}_wipeable_foot_{i}"), 18.0 / 2.0, 16.0, 28)
                .translate(*fx, *fy, -z / 2.0 - 8.0);
    }

    housing - process_window + service_panel + screen + feet
}

fn sterile_tubing_spool_cassette_datum() -> Part {
    let tray = centered_cube(
        "aseptic_tubing_spool_cassette_datum_tray",
        SPOOL_CASSETTE_X,
        SPOOL_CASSETTE_Y,
        SPOOL_CASSETTE_Z,
    );
    let spool_well = centered_cylinder(
        "aseptic_tubing_spool_recess",
        SPOOL_WELL_D / 2.0,
        SPOOL_CASSETTE_Z + 2.0,
        64,
    )
    .translate(0.0, 52.0, 8.0);
    let spool_hub = centered_cylinder(
        "aseptic_tubing_spool_hub_post",
        38.0 / 2.0,
        SPOOL_CASSETTE_Z + 18.0,
        48,
    )
    .translate(0.0, 52.0, 8.0);
    let cassette_pocket = centered_cube(
        "aseptic_tubing_preloaded_cassette_pocket",
        CASSETTE_POCKET_X,
        CASSETTE_POCKET_Y,
        18.0,
    )
    .translate(0.0, -82.0, SPOOL_CASSETTE_Z / 2.0 - 7.0);
    let cassette_key = centered_cube(
        "aseptic_tubing_asymmetric_cassette_key",
        28.0,
        16.0,
        SPOOL_CASSETTE_Z + 2.0,
    )
    .translate(-(CASSETTE_POCKET_X / 2.0 - 18.0), -128.0, 0.0);

    let mut datum_pins = Part::empty("aseptic_tubing_spool_cassette_datum_pins");
    for (i, (x, y)) in [
        (-92.0, -124.0),
        (92.0, -124.0),
        (-92.0, 124.0),
        (92.0, 124.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("aseptic_tubing_spool_cassette_pin_boss_{i}"),
            16.0 / 2.0,
            10.0,
            32,
        )
        .translate(*x, *y, SPOOL_CASSETTE_Z / 2.0 + 5.0);
        let clearance = centered_cylinder(
            format!("aseptic_tubing_spool_cassette_pin_clearance_{i}"),
            4.2 / 2.0,
            14.0,
            24,
        )
        .translate(*x, *y, SPOOL_CASSETTE_Z / 2.0 + 5.0);
        datum_pins = datum_pins + (boss - clearance);
    }

    let tube_exit_comb = spool_tube_exit_comb().translate(0.0, -SPOOL_CASSETTE_Y / 2.0 - 14.0, 6.0);

    (tray - spool_well - cassette_pocket - cassette_key + spool_hub + datum_pins + tube_exit_comb)
        .translate(
            SPOOL_CENTER_X,
            SPOOL_CENTER_Y,
            DECK_Z / 2.0 + SPOOL_CASSETTE_Z / 2.0,
        )
}

fn spool_tube_exit_comb() -> Part {
    let body = centered_cube("aseptic_tubing_spool_exit_comb_body", 184.0, 26.0, 20.0);
    let mut channels = Part::empty("aseptic_tubing_spool_exit_comb_channels");
    for (i, x) in [-63.0, -21.0, 21.0, 63.0].iter().enumerate() {
        let channel = centered_cylinder(
            format!("aseptic_tubing_spool_exit_channel_{i}"),
            TUBE_CHANNEL_D / 2.0,
            30.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, 0.0);
        let top_slot = centered_cube(
            format!("aseptic_tubing_spool_exit_slot_{i}"),
            TUBE_CHANNEL_D + 2.0,
            30.0,
            20.0,
        )
        .translate(*x, 0.0, 7.0);
        channels = channels + channel + top_slot;
    }
    body - channels
}

fn cut_weld_seal_staging_lanes() -> Part {
    let rail = centered_cube(
        "aseptic_tubing_cut_weld_seal_lane_body",
        STAGE_RAIL_X,
        STAGE_RAIL_Y,
        STAGE_RAIL_Z,
    );

    let mut lane_cuts = Part::empty("aseptic_tubing_cut_weld_seal_lane_cuts");
    let mut lane_ridges = Part::empty("aseptic_tubing_cut_weld_seal_lane_witness_ridges");
    for lane in 0..STAGE_LANES {
        let y = stage_lane_y(lane);
        let channel = centered_cylinder(
            format!("aseptic_tubing_stage_lane_{lane}_max_od_channel"),
            TUBE_CHANNEL_D / 2.0,
            STAGE_RAIL_X + 8.0,
            36,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 2.0);
        let top_slot = centered_cube(
            format!("aseptic_tubing_stage_lane_{lane}_open_top_slot"),
            STAGE_RAIL_X + 10.0,
            TUBE_CHANNEL_D + 3.0,
            STAGE_RAIL_Z,
        )
        .translate(0.0, y, STAGE_RAIL_Z / 2.0 - 4.0);
        let witness_ridge = centered_cube(
            format!("aseptic_tubing_stage_lane_{lane}_tube_shadow_ridge"),
            STAGE_RAIL_X - 46.0,
            2.0,
            3.0,
        )
        .translate(
            0.0,
            y - TUBE_CHANNEL_D / 2.0 - 6.0,
            STAGE_RAIL_Z / 2.0 + 1.5,
        );
        lane_cuts = lane_cuts + channel + top_slot;
        lane_ridges = lane_ridges + witness_ridge;
    }

    let cut_window = process_window("cut_blade", CUT_WINDOW_X, 38.0);
    let weld_window = process_window("weld_fusion", WELD_WINDOW_X, 58.0);
    let seal_window = process_window("seal_anvil", SEAL_WINDOW_X, 48.0);
    let lane_stops = staging_lane_end_stops();
    let lane_labels = staging_lane_label_tabs();

    (rail - lane_cuts - cut_window - weld_window - seal_window
        + lane_ridges
        + lane_stops
        + lane_labels)
        .translate(
            STAGE_CENTER_X,
            STAGE_CENTER_Y,
            DECK_Z / 2.0 + STAGE_RAIL_Z / 2.0,
        )
}

fn process_window(name: &str, x: f64, width: f64) -> Part {
    centered_cube(
        format!("aseptic_tubing_{name}_service_window"),
        width,
        STAGE_RAIL_Y + 8.0,
        STAGE_RAIL_Z + 4.0,
    )
    .translate(x, 0.0, 0.0)
}

fn staging_lane_end_stops() -> Part {
    let clean_stop = centered_cube(
        "aseptic_tubing_clean_side_lane_hard_stop",
        18.0,
        STAGE_RAIL_Y - 24.0,
        STAGE_RAIL_Z + 16.0,
    )
    .translate(-STAGE_RAIL_X / 2.0 + 28.0, 0.0, 8.0);
    let used_stop = centered_cube(
        "aseptic_tubing_used_side_lane_hard_stop",
        18.0,
        STAGE_RAIL_Y - 24.0,
        STAGE_RAIL_Z + 12.0,
    )
    .translate(STAGE_RAIL_X / 2.0 - 28.0, 0.0, 6.0);

    clean_stop + used_stop
}

fn staging_lane_label_tabs() -> Part {
    let mut tabs = Part::empty("aseptic_tubing_stage_lane_label_tabs");
    for lane in 0..STAGE_LANES {
        tabs = tabs
            + centered_cube(
                format!("aseptic_tubing_stage_lane_{lane}_barcode_tab"),
                76.0,
                18.0,
                4.0,
            )
            .translate(
                -STAGE_RAIL_X / 2.0 + 74.0,
                stage_lane_y(lane),
                STAGE_RAIL_Z / 2.0 + 2.0,
            );
    }
    tabs
}

fn connector_cap_staging() -> Part {
    let tray = centered_cube(
        "aseptic_tubing_connector_cap_staging_tray",
        CAP_STAGE_X,
        CAP_STAGE_Y,
        CAP_STAGE_Z,
    );
    let center_divider = centered_cube(
        "aseptic_tubing_connector_cap_clean_used_divider",
        CAP_STAGE_X - 30.0,
        9.0,
        CAP_STAGE_Z + 16.0,
    )
    .translate(0.0, 0.0, 8.0);

    let mut wells = Part::empty("aseptic_tubing_connector_cap_staging_wells");
    let mut orientation_tabs = Part::empty("aseptic_tubing_connector_cap_orientation_tabs");
    for bank in 0..CAP_BANKS {
        for row in 0..CAP_ROWS_PER_BANK {
            for col in 0..CAP_COLS {
                let index = bank * CAP_ROWS_PER_BANK * CAP_COLS + row * CAP_COLS + col;
                let (x, y) = cap_well_center(bank, row, col);
                wells = wells
                    + centered_cylinder(
                        format!("aseptic_tubing_connector_cap_well_{index}"),
                        CAP_WELL_D / 2.0,
                        CAP_STAGE_Z + 3.0,
                        32,
                    )
                    .translate(x, y, 2.0);
                orientation_tabs = orientation_tabs
                    + centered_cube(
                        format!("aseptic_tubing_connector_cap_orientation_tick_{index}"),
                        4.0,
                        11.0,
                        3.0,
                    )
                    .translate(
                        x + CAP_WELL_D / 2.0 + 4.0,
                        y,
                        CAP_STAGE_Z / 2.0 + 1.5,
                    );
            }
        }
    }

    let clean_lot_land = centered_cube(
        "aseptic_tubing_clean_cap_lot_land",
        CAP_STAGE_X / 2.0 - 26.0,
        24.0,
        4.0,
    )
    .translate(
        -CAP_STAGE_X / 4.0,
        -CAP_STAGE_Y / 2.0 + 18.0,
        CAP_STAGE_Z / 2.0 + 2.0,
    );
    let used_lot_land = centered_cube(
        "aseptic_tubing_used_cap_lot_land",
        CAP_STAGE_X / 2.0 - 26.0,
        24.0,
        4.0,
    )
    .translate(
        CAP_STAGE_X / 4.0,
        -CAP_STAGE_Y / 2.0 + 18.0,
        CAP_STAGE_Z / 2.0 + 2.0,
    );

    (tray - wells + center_divider + orientation_tabs + clean_lot_land + used_lot_land).translate(
        CAP_STAGE_CENTER_X,
        CAP_STAGE_CENTER_Y,
        DECK_Z / 2.0 + CAP_STAGE_Z / 2.0,
    )
}

fn leak_test_handoff_ports() -> Part {
    let bar = centered_cube(
        "aseptic_tubing_leak_test_handoff_port_bar",
        LEAK_PORT_BAR_X,
        LEAK_PORT_BAR_Y,
        LEAK_PORT_BAR_Z,
    );
    let mut ports = Part::empty("aseptic_tubing_leak_test_port_cuts");
    let mut bosses = Part::empty("aseptic_tubing_leak_test_port_bosses");
    for port in 0..LEAK_PORT_COUNT {
        let x = leak_port_x(port);
        let y = if port % 2 == 0 { -18.0 } else { 18.0 };
        let boss = centered_cylinder(
            format!("aseptic_tubing_leak_test_port_boss_{port}"),
            22.0 / 2.0,
            9.0,
            32,
        )
        .translate(x, y, LEAK_PORT_BAR_Z / 2.0 + 4.5);
        let cut = centered_cylinder(
            format!("aseptic_tubing_leak_test_port_clearance_{port}"),
            LEAK_PORT_D / 2.0,
            LEAK_PORT_BAR_Z + 12.0,
            24,
        )
        .translate(x, y, 2.0);
        bosses = bosses + boss;
        ports = ports + cut;
    }

    let pressure_header = centered_cube(
        "aseptic_tubing_leak_test_pressure_header_keepout",
        LEAK_PORT_BAR_X - 54.0,
        12.0,
        16.0,
    )
    .translate(
        0.0,
        LEAK_PORT_BAR_Y / 2.0 + 6.0,
        LEAK_PORT_BAR_Z / 2.0 + 8.0,
    );
    let drain_gutter = centered_cube(
        "aseptic_tubing_leak_test_condensate_gutter",
        LEAK_PORT_BAR_X - 46.0,
        10.0,
        6.0,
    )
    .translate(
        0.0,
        -LEAK_PORT_BAR_Y / 2.0 + 13.0,
        LEAK_PORT_BAR_Z / 2.0 + 3.0,
    );

    (bar + bosses + pressure_header + drain_gutter - ports).translate(
        LEAK_PORT_CENTER_X,
        LEAK_PORT_CENTER_Y,
        DECK_Z / 2.0 + LEAK_PORT_BAR_Z / 2.0,
    )
}

fn clean_used_segregation_and_waste_trays() -> Part {
    let divider = centered_cube(
        "aseptic_tubing_clean_used_segregation_wall",
        SEGREGATION_DIVIDER_W,
        SEGREGATION_DIVIDER_Y,
        SEGREGATION_DIVIDER_Z,
    )
    .translate(
        SEGREGATION_DIVIDER_X,
        0.0,
        DECK_Z / 2.0 + SEGREGATION_DIVIDER_Z / 2.0,
    );
    let air_gap_gauge = centered_cube(
        "aseptic_tubing_segregation_air_gap_gauge",
        SEGREGATION_AIR_GAP,
        SEGREGATION_DIVIDER_Y - 82.0,
        10.0,
    )
    .translate(
        SEGREGATION_DIVIDER_X - SEGREGATION_DIVIDER_W / 2.0 - SEGREGATION_AIR_GAP / 2.0,
        0.0,
        DECK_Z / 2.0 + 5.0,
    );

    divider + air_gap_gauge + waste_sharps_offcut_tray()
}

fn waste_sharps_offcut_tray() -> Part {
    let tray = centered_cube(
        "aseptic_tubing_waste_sharps_offcut_outer_tray",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let sharps_basin = centered_cube(
        "aseptic_tubing_hot_blade_sharps_basin",
        WASTE_TRAY_X / 2.0 - 22.0,
        WASTE_TRAY_Y - 30.0,
        WASTE_TRAY_Z - 12.0,
    )
    .translate(-WASTE_TRAY_X / 4.0, 0.0, 8.0);
    let offcut_basin = centered_cube(
        "aseptic_tubing_tube_offcut_basin",
        WASTE_TRAY_X / 2.0 - 22.0,
        WASTE_TRAY_Y - 30.0,
        WASTE_TRAY_Z - 12.0,
    )
    .translate(WASTE_TRAY_X / 4.0, 0.0, 8.0);
    let divider = centered_cube(
        "aseptic_tubing_waste_tray_center_divider",
        10.0,
        WASTE_TRAY_Y - 22.0,
        WASTE_TRAY_Z + 10.0,
    )
    .translate(0.0, 0.0, 5.0);
    let removable_liner_tab = centered_cube(
        "aseptic_tubing_waste_tray_removable_liner_tab",
        WASTE_TRAY_X - 38.0,
        16.0,
        4.0,
    )
    .translate(0.0, -WASTE_TRAY_Y / 2.0 + 18.0, WASTE_TRAY_Z / 2.0 + 2.0);

    (tray - sharps_basin - offcut_basin + divider + removable_liner_tab).translate(
        WASTE_TRAY_CENTER_X,
        WASTE_TRAY_CENTER_Y,
        DECK_Z / 2.0 + WASTE_TRAY_Z / 2.0,
    )
}

fn barcode_and_lot_lands() -> Part {
    let plate = centered_cube(
        "aseptic_tubing_barcode_lot_land_plate",
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    );
    let title_strip = centered_cube(
        "aseptic_tubing_barcode_lot_title_strip",
        BARCODE_PANEL_X - 30.0,
        14.0,
        4.0,
    )
    .translate(
        0.0,
        BARCODE_PANEL_Y / 2.0 - 16.0,
        BARCODE_PANEL_Z / 2.0 + 2.0,
    );

    let mut lands = Part::empty("aseptic_tubing_barcode_lot_lands");
    for land in 0..LABEL_LAND_COUNT {
        let row = land / 4;
        let col = land % 4;
        let x = -((4.0 - 1.0) * 78.0) / 2.0 + col as f64 * 78.0;
        let y = -18.0 + row as f64 * 36.0;
        lands = lands
            + centered_cube(
                format!("aseptic_tubing_barcode_lot_land_{land}"),
                64.0,
                24.0,
                3.0,
            )
            .translate(x, y, BARCODE_PANEL_Z / 2.0 + 1.5);
    }

    let scanner_fiducials = barcode_panel_fiducials();

    (plate + title_strip + lands + scanner_fiducials).translate(
        BARCODE_CENTER_X,
        BARCODE_CENTER_Y,
        DECK_Z / 2.0 + BARCODE_PANEL_Z / 2.0,
    )
}

fn barcode_panel_fiducials() -> Part {
    let mut fiducials = Part::empty("aseptic_tubing_barcode_panel_fiducials");
    for (i, (x, y)) in [
        (
            -(BARCODE_PANEL_X / 2.0 - 22.0),
            -(BARCODE_PANEL_Y / 2.0 - 18.0),
        ),
        (
            BARCODE_PANEL_X / 2.0 - 22.0,
            -(BARCODE_PANEL_Y / 2.0 - 18.0),
        ),
        (
            -(BARCODE_PANEL_X / 2.0 - 22.0),
            BARCODE_PANEL_Y / 2.0 - 18.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("aseptic_tubing_barcode_fiducial_target_{i}"),
            7.0,
            2.0,
            36,
        )
        .translate(*x, *y, BARCODE_PANEL_Z / 2.0 + 1.0);
        let dot = centered_cylinder(
            format!("aseptic_tubing_barcode_fiducial_dot_{i}"),
            1.5,
            3.0,
            18,
        )
        .translate(*x, *y, BARCODE_PANEL_Z / 2.0 + 1.0);
        fiducials = fiducials + (target - dot);
    }
    fiducials
}

fn vhp_cleanability_and_robot_keepouts() -> Part {
    let rear_vhp_sweep = centered_cube(
        "aseptic_tubing_rear_vhp_wipe_sweep_clearance_gauge",
        DECK_X - 130.0,
        VHP_CLEARANCE_Y,
        VHP_CLEARANCE_Z,
    )
    .translate(
        -18.0,
        VHP_CLEARANCE_CENTER_Y,
        DECK_Z / 2.0 + VHP_CLEARANCE_Z / 2.0,
    );
    let front_robot_keepout = centered_cube(
        "aseptic_tubing_front_robot_service_keepout_gauge",
        DECK_X - 280.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_APPROACH_CLEARANCE_Z,
    )
    .translate(
        -120.0,
        FRONT_ROBOT_KEEP_OUT_CENTER_Y,
        DECK_Z / 2.0 + ROBOT_APPROACH_CLEARANCE_Z / 2.0,
    );
    let side_service_keepout = centered_cube(
        "aseptic_tubing_right_service_swing_keepout_gauge",
        SIDE_SERVICE_KEEP_OUT_X,
        DECK_Y - 190.0,
        94.0,
    )
    .translate(SIDE_SERVICE_KEEP_OUT_CENTER_X, 12.0, DECK_Z / 2.0 + 47.0);
    let robot_tcp_window = centered_cube(
        "aseptic_tubing_robot_tcp_handoff_window_gauge",
        ROBOT_APPROACH_CLEARANCE_X,
        42.0,
        82.0,
    )
    .translate(
        STAGE_CENTER_X + STAGE_RAIL_X / 2.0 - ROBOT_APPROACH_CLEARANCE_X / 2.0 - 42.0,
        STAGE_CENTER_Y - STAGE_RAIL_Y / 2.0 - 40.0,
        DECK_Z / 2.0 + 68.0,
    );
    let drip_shadow = centered_cube(
        "aseptic_tubing_cleanability_drip_shadow_under_equipment",
        WELDER_X + SEALER_X + 72.0,
        14.0,
        8.0,
    )
    .translate(
        (WELDER_CENTER_X + SEALER_CENTER_X) / 2.0,
        16.0,
        DECK_Z / 2.0 + 4.0,
    );

    rear_vhp_sweep + front_robot_keepout + side_service_keepout + robot_tcp_window + drip_shadow
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-585.0, -365.0),
        (-300.0, -365.0),
        (0.0, -365.0),
        (300.0, -365.0),
        (585.0, -365.0),
        (-585.0, 365.0),
        (0.0, 365.0),
        (585.0, 365.0),
    ]
}

fn stage_lane_y(lane: usize) -> f64 {
    -((STAGE_LANES as f64 - 1.0) * STAGE_LANE_PITCH_Y) / 2.0 + lane as f64 * STAGE_LANE_PITCH_Y
}

fn cap_well_center(bank: usize, row: usize, col: usize) -> (f64, f64) {
    let bank_offset_y = if bank == 0 { -46.0 } else { 46.0 };
    let x = -((CAP_COLS as f64 - 1.0) * CAP_PITCH_X) / 2.0 + col as f64 * CAP_PITCH_X;
    let y = bank_offset_y
        + -((CAP_ROWS_PER_BANK as f64 - 1.0) * CAP_PITCH_Y) / 2.0
        + row as f64 * CAP_PITCH_Y;
    (x, y)
}

fn leak_port_x(port: usize) -> f64 {
    -((LEAK_PORT_COUNT as f64 - 1.0) * LEAK_PORT_PITCH_X) / 2.0 + port as f64 * LEAK_PORT_PITCH_X
}

fn connector_cap_well_count() -> usize {
    CAP_BANKS * CAP_ROWS_PER_BANK * CAP_COLS
}

#[cfg(test)]
fn rect_fits_deck(center_x: f64, center_y: f64, x: f64, y: f64, margin: f64) -> bool {
    center_x.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center_y.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/aseptic_tubing_weld_seal_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn researched_equipment_and_tube_envelopes_are_represented() {
        assert!(TUBE_OD_MIN <= 6.35);
        assert!(TUBE_OD_MAX >= 25.4);
        assert!(SEALER_X >= BIOSEALER_DATASHEET_X);
        assert!(SEALER_Y >= BIOSEALER_DATASHEET_Y);
        assert!(SEALER_Z >= BIOSEALER_DATASHEET_Z);
        assert!(WELDER_X > SEALER_X);
        assert!(WELDER_Z > SEALER_Z);
    }

    #[test]
    fn repeated_feature_counts_match_closed_process_workflow() {
        assert_eq!(STAGE_LANES, 4);
        assert_eq!(connector_cap_well_count(), 24);
        assert_eq!(LEAK_PORT_COUNT, 8);
        assert_eq!(LABEL_LAND_COUNT, 8);
        assert_eq!(deck_mount_points().len(), 8);
    }

    #[test]
    fn placed_modules_fit_within_deck_footprint() {
        assert!(rect_fits_deck(
            WELDER_CENTER_X,
            WELDER_CENTER_Y,
            WELDER_X,
            WELDER_Y,
            20.0
        ));
        assert!(rect_fits_deck(
            SEALER_CENTER_X,
            SEALER_CENTER_Y,
            SEALER_X,
            SEALER_Y,
            20.0
        ));
        assert!(rect_fits_deck(
            SPOOL_CENTER_X,
            SPOOL_CENTER_Y,
            SPOOL_CASSETTE_X,
            SPOOL_CASSETTE_Y,
            10.0
        ));
        assert!(rect_fits_deck(
            STAGE_CENTER_X,
            STAGE_CENTER_Y,
            STAGE_RAIL_X,
            STAGE_RAIL_Y,
            15.0
        ));
        assert!(rect_fits_deck(
            CAP_STAGE_CENTER_X,
            CAP_STAGE_CENTER_Y,
            CAP_STAGE_X,
            CAP_STAGE_Y,
            10.0
        ));
        assert!(rect_fits_deck(
            LEAK_PORT_CENTER_X,
            LEAK_PORT_CENTER_Y,
            LEAK_PORT_BAR_X,
            LEAK_PORT_BAR_Y,
            10.0
        ));
        assert!(rect_fits_deck(
            WASTE_TRAY_CENTER_X,
            WASTE_TRAY_CENTER_Y,
            WASTE_TRAY_X,
            WASTE_TRAY_Y,
            10.0
        ));
        assert!(rect_fits_deck(
            BARCODE_CENTER_X,
            BARCODE_CENTER_Y,
            BARCODE_PANEL_X,
            BARCODE_PANEL_Y,
            4.0
        ));
    }

    #[test]
    fn lane_and_port_arrays_stay_inside_their_hardware() {
        assert!(stage_lane_y(0).abs() + TUBE_CHANNEL_D / 2.0 + 8.0 < STAGE_RAIL_Y / 2.0);
        assert!(
            stage_lane_y(STAGE_LANES - 1).abs() + TUBE_CHANNEL_D / 2.0 + 8.0 < STAGE_RAIL_Y / 2.0
        );
        assert!(leak_port_x(0).abs() + 22.0 < LEAK_PORT_BAR_X / 2.0);
        assert!(leak_port_x(LEAK_PORT_COUNT - 1).abs() + 22.0 < LEAK_PORT_BAR_X / 2.0);
        let (cap_x, cap_y) = cap_well_center(CAP_BANKS - 1, CAP_ROWS_PER_BANK - 1, CAP_COLS - 1);
        assert!(cap_x.abs() + CAP_WELL_D / 2.0 + 14.0 < CAP_STAGE_X / 2.0);
        assert!(cap_y.abs() + CAP_WELL_D / 2.0 + 14.0 < CAP_STAGE_Y / 2.0);
    }

    #[test]
    fn clean_used_segregation_and_service_keepouts_are_explicit() {
        assert!(SPOOL_CENTER_X + SPOOL_CASSETTE_X / 2.0 < SEGREGATION_DIVIDER_X);
        assert!(BARCODE_CENTER_X + BARCODE_PANEL_X / 2.0 < SEGREGATION_DIVIDER_X);
        assert!(WASTE_TRAY_CENTER_X - WASTE_TRAY_X / 2.0 > SEGREGATION_DIVIDER_X);
        assert!(SEGREGATION_DIVIDER_W >= 24.0);
        assert!(SEGREGATION_AIR_GAP >= 50.0);
        assert!(VHP_CLEARANCE_Y >= 60.0);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 80.0);
        assert!(ROBOT_APPROACH_CLEARANCE_X >= 400.0);
    }
}
