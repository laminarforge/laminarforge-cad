use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed microfluidic chip lot dimensional metrology and fit station.
//
// Intent:
// - Qualify incoming lots of closed microfluidic chips before loading them into
//   scaled multi-chip cassettes.
// - Make chip datum fit, port pitch, gasket land compression, channel/warpage
//   visibility, lot custody, quarantine/release disposition, camera evidence,
//   and robot service datums physically inspectable.
// - Model a mechanical qualification station only. This does not set biological
//   release criteria, certificate acceptance limits, or vendor quality claims.

const BIN_PREFIX: &str = "closed_microfluidic_chip_lot_dimensional_metrology_fit_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_containment_deck.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_scaled_cassette_datum_nests.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_lot_sample_chip_surrogates.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_go_no_go_gauge_rails.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_port_pitch_alignment_combs.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_gasket_land_compression_witnesses.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_transparent_channel_warpage_windows.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_barcode_coa_custody_lands.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_quarantine_release_gate_lanes.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_camera_evidence_bridge.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_robotic_service_datums.stl",
    "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "chip_datum_nests",
    "scaled_multi_chip_cassette_fit_envelope",
    "go_no_go_gauge_rails",
    "port_pitch_alignment_combs",
    "gasket_land_compression_witnesses",
    "transparent_channel_warpage_inspection_windows",
    "barcode_coa_custody_lands",
    "quarantine_release_gates",
    "camera_evidence_bridge",
    "robotic_service_datums",
    "multi_chip_lot_sample_surrogates",
];

const STATION_X: f64 = 1220.0;
const STATION_Y: f64 = 820.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;
const MOUNT_HOLE_COUNT: usize = 8;

const CHIP_X: f64 = 76.0;
const CHIP_Y: f64 = 26.0;
const CHIP_Z: f64 = 1.8;
const CHIP_SLOT_ROWS: usize = 3;
const CHIP_SLOT_COLS: usize = 4;
const CHIP_SLOT_COUNT: usize = CHIP_SLOT_ROWS * CHIP_SLOT_COLS;
const CHIP_SLOT_PITCH_X: f64 = 106.0;
const CHIP_SLOT_PITCH_Y: f64 = 62.0;
const CHIP_NEST_CLEARANCE: f64 = 0.35;
const CHIP_NEST_DEPTH: f64 = 10.0;
const CHIP_DATUMS_PER_SLOT: usize = 3;
const CHIP_DATUM_COUNT: usize = CHIP_SLOT_COUNT * CHIP_DATUMS_PER_SLOT;

const PORTS_PER_CHIP: usize = 2;
const PORT_PITCH_X: f64 = 42.0;
const PORT_D: f64 = 2.4;
const PORT_TINE_COUNT: usize = CHIP_SLOT_COUNT * PORTS_PER_CHIP;

const NEST_CENTER: (f64, f64) = (-310.0, 126.0);
const NEST_BLOCK_X: f64 = 500.0;
const NEST_BLOCK_Y: f64 = 270.0;
const NEST_BLOCK_Z: f64 = 38.0;

const GAUGE_CENTER: (f64, f64) = (275.0, 210.0);
const GAUGE_PANEL_X: f64 = 430.0;
const GAUGE_PANEL_Y: f64 = 230.0;
const GAUGE_PANEL_Z: f64 = 24.0;
const GO_NO_GO_RAIL_PAIR_COUNT: usize = 3;
const GAUGE_LANE_PITCH_Y: f64 = 62.0;
const GAUGE_RAIL_W: f64 = 8.0;
const GAUGE_RAIL_Z: f64 = 24.0;

const COMB_CENTER: (f64, f64) = (305.0, -50.0);
const COMB_PANEL_X: f64 = 400.0;
const COMB_PANEL_Y: f64 = 190.0;
const COMB_PANEL_Z: f64 = 18.0;
const COMB_SLOT_PITCH_X: f64 = 80.0;
const COMB_SLOT_PITCH_Y: f64 = 46.0;
const COMB_TINE_Z: f64 = 22.0;

const GASKET_CENTER: (f64, f64) = (-318.0, -152.0);
const GASKET_PANEL_X: f64 = 480.0;
const GASKET_PANEL_Y: f64 = 160.0;
const GASKET_PANEL_Z: f64 = 20.0;
const COMPRESSION_STEP_COUNT: usize = 5;
const GASKET_RING_COUNT: usize = CHIP_SLOT_COUNT;

const WINDOW_CENTER: (f64, f64) = (120.0, -260.0);
const WINDOW_PANEL_X: f64 = 370.0;
const WINDOW_PANEL_Y: f64 = 160.0;
const WINDOW_PANEL_Z: f64 = 16.0;
const INSPECTION_WINDOW_ROWS: usize = 2;
const INSPECTION_WINDOW_COLS: usize = 3;
const INSPECTION_WINDOW_COUNT: usize = INSPECTION_WINDOW_ROWS * INSPECTION_WINDOW_COLS;
const WARPAGE_RAIL_COUNT: usize = 4;

const CUSTODY_CENTER: (f64, f64) = (-390.0, -310.0);
const CUSTODY_PANEL_X: f64 = 330.0;
const CUSTODY_PANEL_Y: f64 = 120.0;
const CUSTODY_PANEL_Z: f64 = 16.0;
const BARCODE_LAND_COUNT: usize = 6;
const COA_LAND_COUNT: usize = 3;

const GATE_CENTER: (f64, f64) = (432.0, -255.0);
const GATE_PANEL_X: f64 = 250.0;
const GATE_PANEL_Y: f64 = 210.0;
const GATE_PANEL_Z: f64 = 22.0;
const STATUS_LANE_COUNT: usize = 3;
const STATUS_TOKEN_SLOTS_PER_LANE: usize = 4;

const BRIDGE_CENTER: (f64, f64) = (0.0, 20.0);
const BRIDGE_SPAN_X: f64 = 1040.0;
const BRIDGE_FOOTPRINT_Y: f64 = 90.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 56.0;
const BRIDGE_UNDERSIDE_Z: f64 = 210.0;
const BRIDGE_BEAM_Z: f64 = 36.0;
const CAMERA_COUNT: usize = 3;
const LIGHT_RAIL_COUNT: usize = 2;

const SERVICE_CENTER: (f64, f64) = (0.0, -352.0);
const SERVICE_STRIP_X: f64 = 980.0;
const SERVICE_STRIP_Y: f64 = 54.0;
const SERVICE_STRIP_Z: f64 = 20.0;
const ROBOTIC_SERVICE_DATUM_COUNT: usize = 6;
const ROBOT_FORK_GAUGE_COUNT: usize = 4;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let nests = scaled_cassette_datum_nests();
    export(OUTPUTS[1], &nests);

    let samples = lot_sample_chip_surrogates();
    export(OUTPUTS[2], &samples);

    let gauges = go_no_go_gauge_rails();
    export(OUTPUTS[3], &gauges);

    let combs = port_pitch_alignment_combs();
    export(OUTPUTS[4], &combs);

    let gasket = gasket_land_compression_witnesses();
    export(OUTPUTS[5], &gasket);

    let windows = transparent_channel_warpage_windows();
    export(OUTPUTS[6], &windows);

    let custody = barcode_coa_custody_lands();
    export(OUTPUTS[7], &custody);

    let gates = quarantine_release_gate_lanes();
    export(OUTPUTS[8], &gates);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let datums = robotic_service_datums();
    export(OUTPUTS[10], &datums);

    let assembly = deck
        + nests
        + samples
        + gauges
        + combs
        + gasket
        + windows
        + custody
        + gates
        + bridge
        + datums;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed microfluidic chip lot dimensional metrology and fit station:");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm deck");
    println!(
        "  Cassette capacity:          {CHIP_SLOT_ROWS} x {CHIP_SLOT_COLS} chip datum nests, {CHIP_SLOT_COUNT} lot sample chips, {CHIP_DATUM_COUNT} datum contacts"
    );
    println!(
        "  Metrology gates:            {GO_NO_GO_RAIL_PAIR_COUNT} go/no-go rail pairs, {PORT_TINE_COUNT} port pitch comb tines at {PORT_PITCH_X:.1}mm chip port pitch"
    );
    println!(
        "  Fit witnesses:              {GASKET_RING_COUNT} gasket land rings, {COMPRESSION_STEP_COUNT} compression steps, {INSPECTION_WINDOW_COUNT} channel/warpage windows, {WARPAGE_RAIL_COUNT} straightedge rails"
    );
    println!(
        "  Custody and release:        {BARCODE_LAND_COUNT} barcode lands, {COA_LAND_COUNT} COA lands, {STATUS_LANE_COUNT} disposition gates, {ROBOTIC_SERVICE_DATUM_COUNT} robotic service datums"
    );
    println!(
        "  Evidence bridge:            {CAMERA_COUNT} camera pods, {LIGHT_RAIL_COUNT} light rails, bridge underside {BRIDGE_UNDERSIDE_Z:.0}mm over deck"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{BIN_PREFIX}_containment_deck_floor"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let washdown_recess = centered_cube(
        format!("{BIN_PREFIX}_shallow_metrology_washdown_recess"),
        STATION_X - 116.0,
        STATION_Y - 112.0,
        7.0,
    )
    .translate(0.0, -4.0, DECK_Z - 3.4);
    let drain_gutter = rectangular_outline(
        format!("{BIN_PREFIX}_perimeter_liquid_gutter"),
        STATION_X - 82.0,
        STATION_Y - 86.0,
        12.0,
        4.0,
    )
    .translate(0.0, 0.0, DECK_Z + 2.0);
    let origin_cross = centered_cube(
        format!("{BIN_PREFIX}_centerline_x_metrology_tick"),
        STATION_X - 190.0,
        3.0,
        4.0,
    )
    .translate(0.0, 0.0, DECK_Z + 2.0)
        + centered_cube(
            format!("{BIN_PREFIX}_centerline_y_metrology_tick"),
            3.0,
            STATION_Y - 170.0,
            4.0,
        )
        .translate(0.0, 0.0, DECK_Z + 2.0);

    deck - washdown_recess - insert_sockets() - mount_holes()
        + perimeter_rims()
        + drain_gutter
        + origin_cross
}

fn scaled_cassette_datum_nests() -> Part {
    let body = centered_cube(
        format!("{BIN_PREFIX}_scaled_multi_chip_cassette_fit_envelope"),
        NEST_BLOCK_X,
        NEST_BLOCK_Y,
        NEST_BLOCK_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, insert_z(NEST_BLOCK_Z));
    let outer_lip = rectangular_outline(
        format!("{BIN_PREFIX}_cassette_outer_fit_lip"),
        NEST_BLOCK_X,
        NEST_BLOCK_Y,
        8.0,
        16.0,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_BLOCK_Z + 8.0);

    let mut pockets = Part::empty(format!("{BIN_PREFIX}_chip_pocket_negative_array"));
    let mut finger_reliefs = Part::empty(format!("{BIN_PREFIX}_chip_front_finger_reliefs"));
    let mut datums = Part::empty(format!("{BIN_PREFIX}_chip_datum_button_array"));
    let mut row_col_flags = Part::empty(format!("{BIN_PREFIX}_cassette_row_column_flags"));

    for row in 0..CHIP_SLOT_ROWS {
        for col in 0..CHIP_SLOT_COLS {
            let slot = slot_xy(row, col);
            let x = NEST_CENTER.0 + slot.0;
            let y = NEST_CENTER.1 + slot.1;
            let slot_index = row * CHIP_SLOT_COLS + col;

            pockets = pockets
                + centered_cube(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_go_fit_pocket"),
                    CHIP_X + CHIP_NEST_CLEARANCE * 2.0,
                    CHIP_Y + CHIP_NEST_CLEARANCE * 2.0,
                    CHIP_NEST_DEPTH + 0.4,
                )
                .translate(
                    x,
                    y,
                    DECK_Z + NEST_BLOCK_Z - CHIP_NEST_DEPTH / 2.0 + 0.2,
                );

            finger_reliefs = finger_reliefs
                + centered_cube(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_front_lift_relief"),
                    18.0,
                    12.0,
                    CHIP_NEST_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y - CHIP_Y / 2.0 - 1.0,
                    DECK_Z + NEST_BLOCK_Z - CHIP_NEST_DEPTH / 2.0 + 0.4,
                );

            datums = datums
                + datum_button(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_datum_a"),
                    x - CHIP_X / 2.0 + 7.0,
                    y - CHIP_Y / 2.0 + 6.0,
                )
                + datum_button(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_datum_b"),
                    x + CHIP_X / 2.0 - 7.0,
                    y - CHIP_Y / 2.0 + 6.0,
                )
                + datum_button(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_datum_c"),
                    x - CHIP_X / 2.0 + 7.0,
                    y + CHIP_Y / 2.0 - 6.0,
                );

            row_col_flags = row_col_flags
                + centered_cube(
                    format!("{BIN_PREFIX}_chip_slot_{slot_index}_lot_id_tab"),
                    9.0,
                    4.0,
                    7.0,
                )
                .translate(
                    x + CHIP_X / 2.0 + 8.0,
                    y + CHIP_Y / 2.0 - 3.0,
                    DECK_Z + NEST_BLOCK_Z + 3.5,
                );
        }
    }

    body + outer_lip + datums + row_col_flags - pockets - finger_reliefs
}

fn lot_sample_chip_surrogates() -> Part {
    let mut chips = Part::empty(format!("{BIN_PREFIX}_incoming_lot_sample_chip_substrates"));
    let mut ports = Part::empty(format!("{BIN_PREFIX}_incoming_lot_sample_chip_port_holes"));
    let mut channels = Part::empty(format!(
        "{BIN_PREFIX}_incoming_lot_sample_channel_witnesses"
    ));
    let mut corner_marks = Part::empty(format!("{BIN_PREFIX}_incoming_lot_sample_corner_marks"));

    for row in 0..CHIP_SLOT_ROWS {
        for col in 0..CHIP_SLOT_COLS {
            let slot = slot_xy(row, col);
            let x = NEST_CENTER.0 + slot.0;
            let y = NEST_CENTER.1 + slot.1;
            let slot_index = row * CHIP_SLOT_COLS + col;
            let chip_z = DECK_Z + NEST_BLOCK_Z + CHIP_Z / 2.0 + 0.65;

            chips = chips
                + centered_cube(
                    format!("{BIN_PREFIX}_lot_sample_chip_{slot_index}_closed_substrate"),
                    CHIP_X,
                    CHIP_Y,
                    CHIP_Z,
                )
                .translate(x, y, chip_z);

            for port in 0..PORTS_PER_CHIP {
                let px = x + port_x(port);
                ports = ports
                    + centered_cylinder(
                        format!("{BIN_PREFIX}_lot_sample_chip_{slot_index}_port_{port}"),
                        PORT_D / 2.0,
                        CHIP_Z + 2.0,
                        24,
                    )
                    .translate(px, y, chip_z);
            }

            channels = channels
                + centered_cube(
                    format!("{BIN_PREFIX}_lot_sample_chip_{slot_index}_transparent_channel_axis"),
                    PORT_PITCH_X,
                    1.0,
                    0.55,
                )
                .translate(x, y, chip_z + CHIP_Z / 2.0 + 0.25);

            for (mark, (mx, my)) in [
                (-CHIP_X / 2.0 + 6.0, -CHIP_Y / 2.0 + 5.0),
                (CHIP_X / 2.0 - 6.0, -CHIP_Y / 2.0 + 5.0),
                (-CHIP_X / 2.0 + 6.0, CHIP_Y / 2.0 - 5.0),
                (CHIP_X / 2.0 - 6.0, CHIP_Y / 2.0 - 5.0),
            ]
            .iter()
            .enumerate()
            {
                corner_marks = corner_marks
                    + centered_cube(
                        format!("{BIN_PREFIX}_lot_sample_chip_{slot_index}_corner_datum_{mark}"),
                        4.0,
                        1.2,
                        0.6,
                    )
                    .translate(x + mx, y + my, chip_z + CHIP_Z / 2.0 + 0.3);
            }
        }
    }

    chips + channels + corner_marks - ports
}

fn go_no_go_gauge_rails() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_go_no_go_gauge_base_plate"),
        GAUGE_PANEL_X,
        GAUGE_PANEL_Y,
        GAUGE_PANEL_Z,
    )
    .translate(GAUGE_CENTER.0, GAUGE_CENTER.1, insert_z(GAUGE_PANEL_Z));
    let mut rails = Part::empty(format!("{BIN_PREFIX}_go_no_go_width_and_length_rails"));
    let mut labels = Part::empty(format!("{BIN_PREFIX}_go_no_go_lane_notch_labels"));

    for lane in 0..GO_NO_GO_RAIL_PAIR_COUNT {
        let lane_y = centered_index(lane, GO_NO_GO_RAIL_PAIR_COUNT, GAUGE_LANE_PITCH_Y);
        let gap = gauge_rail_gap(lane);
        let lane_name = match lane {
            0 => "go_clearance",
            1 => "nominal_width",
            _ => "no_go_tight",
        };

        rails = rails
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_left_side_rail"),
                CHIP_X + 70.0,
                GAUGE_RAIL_W,
                GAUGE_RAIL_Z,
            )
            .translate(
                GAUGE_CENTER.0,
                GAUGE_CENTER.1 + lane_y - gap / 2.0,
                DECK_Z + GAUGE_PANEL_Z + GAUGE_RAIL_Z / 2.0,
            )
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_right_side_rail"),
                CHIP_X + 70.0,
                GAUGE_RAIL_W,
                GAUGE_RAIL_Z,
            )
            .translate(
                GAUGE_CENTER.0,
                GAUGE_CENTER.1 + lane_y + gap / 2.0,
                DECK_Z + GAUGE_PANEL_Z + GAUGE_RAIL_Z / 2.0,
            )
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_length_end_stop_a"),
                8.0,
                gap + GAUGE_RAIL_W * 2.0,
                22.0,
            )
            .translate(
                GAUGE_CENTER.0 - CHIP_X / 2.0 - 13.0,
                GAUGE_CENTER.1 + lane_y,
                DECK_Z + GAUGE_PANEL_Z + 11.0,
            )
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_length_end_stop_b"),
                8.0,
                gap + GAUGE_RAIL_W * 2.0,
                22.0,
            )
            .translate(
                GAUGE_CENTER.0 + CHIP_X / 2.0 + 13.0,
                GAUGE_CENTER.1 + lane_y,
                DECK_Z + GAUGE_PANEL_Z + 11.0,
            );

        labels = labels
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_operator_sight_notch"),
                32.0,
                5.0,
                4.0,
            )
            .translate(
                GAUGE_CENTER.0 - GAUGE_PANEL_X / 2.0 + 34.0,
                GAUGE_CENTER.1 + lane_y,
                DECK_Z + GAUGE_PANEL_Z + 2.0,
            );
    }

    let length_bar = centered_cube(
        format!("{BIN_PREFIX}_chip_length_reference_bar"),
        CHIP_X,
        10.0,
        18.0,
    )
    .translate(
        GAUGE_CENTER.0,
        GAUGE_CENTER.1 + GAUGE_PANEL_Y / 2.0 - 28.0,
        DECK_Z + GAUGE_PANEL_Z + 9.0,
    );

    base + rails + labels + length_bar
}

fn port_pitch_alignment_combs() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_port_pitch_alignment_comb_base"),
        COMB_PANEL_X,
        COMB_PANEL_Y,
        COMB_PANEL_Z,
    )
    .translate(COMB_CENTER.0, COMB_CENTER.1, insert_z(COMB_PANEL_Z));
    let mut tines = Part::empty(format!("{BIN_PREFIX}_port_pitch_comb_tines"));
    let mut bridges = Part::empty(format!("{BIN_PREFIX}_port_pitch_comb_bridge_bars"));

    for row in 0..CHIP_SLOT_ROWS {
        let local_y = centered_index(row, CHIP_SLOT_ROWS, COMB_SLOT_PITCH_Y);
        bridges = bridges
            + centered_cube(
                format!("{BIN_PREFIX}_port_comb_row_{row}_datum_bridge"),
                COMB_PANEL_X - 48.0,
                4.0,
                8.0,
            )
            .translate(
                COMB_CENTER.0,
                COMB_CENTER.1 + local_y,
                DECK_Z + COMB_PANEL_Z + 4.0,
            );

        for col in 0..CHIP_SLOT_COLS {
            let slot_x = centered_index(col, CHIP_SLOT_COLS, COMB_SLOT_PITCH_X);
            let slot_index = row * CHIP_SLOT_COLS + col;
            for port in 0..PORTS_PER_CHIP {
                let x = COMB_CENTER.0 + slot_x + port_x(port);
                let y = COMB_CENTER.1 + local_y;
                tines = tines
                    + centered_cylinder(
                        format!("{BIN_PREFIX}_slot_{slot_index}_port_{port}_pitch_tine"),
                        1.6,
                        COMB_TINE_Z,
                        20,
                    )
                    .translate(x, y, DECK_Z + COMB_PANEL_Z + COMB_TINE_Z / 2.0);
            }
        }
    }

    let datum_comb_backstop = centered_cube(
        format!("{BIN_PREFIX}_port_comb_cassette_datum_backstop"),
        COMB_PANEL_X - 60.0,
        10.0,
        20.0,
    )
    .translate(
        COMB_CENTER.0,
        COMB_CENTER.1 + COMB_PANEL_Y / 2.0 - 20.0,
        DECK_Z + COMB_PANEL_Z + 10.0,
    );

    base + tines + bridges + datum_comb_backstop
}

fn gasket_land_compression_witnesses() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_gasket_land_witness_base_plate"),
        GASKET_PANEL_X,
        GASKET_PANEL_Y,
        GASKET_PANEL_Z,
    )
    .translate(GASKET_CENTER.0, GASKET_CENTER.1, insert_z(GASKET_PANEL_Z));
    let mut gasket_rings = Part::empty(format!("{BIN_PREFIX}_gasket_land_outline_array"));

    for row in 0..CHIP_SLOT_ROWS {
        for col in 0..CHIP_SLOT_COLS {
            let local_x = centered_index(col, CHIP_SLOT_COLS, CHIP_SLOT_PITCH_X);
            let local_y = centered_index(row, CHIP_SLOT_ROWS, 42.0);
            let slot_index = row * CHIP_SLOT_COLS + col;
            gasket_rings = gasket_rings
                + rectangular_outline(
                    format!("{BIN_PREFIX}_slot_{slot_index}_gasket_land_witness_ring"),
                    CHIP_X + 11.0,
                    CHIP_Y + 10.0,
                    3.0,
                    4.0,
                )
                .translate(
                    GASKET_CENTER.0 + local_x,
                    GASKET_CENTER.1 + local_y,
                    DECK_Z + GASKET_PANEL_Z + 2.0,
                );
        }
    }

    let mut compression_steps =
        Part::empty(format!("{BIN_PREFIX}_compression_witness_step_ladder"));
    for step in 0..COMPRESSION_STEP_COUNT {
        let height = 1.0 + step as f64 * 0.45;
        compression_steps = compression_steps
            + centered_cube(
                format!("{BIN_PREFIX}_compression_witness_step_{step}"),
                28.0,
                34.0,
                height,
            )
            .translate(
                GASKET_CENTER.0 + GASKET_PANEL_X / 2.0 - 34.0,
                GASKET_CENTER.1 + centered_index(step, COMPRESSION_STEP_COUNT, 25.0),
                DECK_Z + GASKET_PANEL_Z + height / 2.0,
            );
    }

    let gasket_pressure_scribe = centered_cube(
        format!("{BIN_PREFIX}_gasket_land_nominal_pressure_scribe"),
        GASKET_PANEL_X - 46.0,
        2.2,
        4.0,
    )
    .translate(
        GASKET_CENTER.0,
        GASKET_CENTER.1,
        DECK_Z + GASKET_PANEL_Z + 2.0,
    );

    base + gasket_rings + compression_steps + gasket_pressure_scribe
}

fn transparent_channel_warpage_windows() -> Part {
    let frame = centered_cube(
        format!("{BIN_PREFIX}_transparent_channel_warpage_window_frame"),
        WINDOW_PANEL_X,
        WINDOW_PANEL_Y,
        WINDOW_PANEL_Z,
    )
    .translate(WINDOW_CENTER.0, WINDOW_CENTER.1, insert_z(WINDOW_PANEL_Z));
    let mut openings = Part::empty(format!("{BIN_PREFIX}_transparent_window_cutouts"));
    let mut ledges = Part::empty(format!("{BIN_PREFIX}_transparent_window_retention_ledges"));

    for row in 0..INSPECTION_WINDOW_ROWS {
        for col in 0..INSPECTION_WINDOW_COLS {
            let x = WINDOW_CENTER.0 + centered_index(col, INSPECTION_WINDOW_COLS, 104.0);
            let y = WINDOW_CENTER.1 + centered_index(row, INSPECTION_WINDOW_ROWS, 60.0);
            let index = row * INSPECTION_WINDOW_COLS + col;
            openings = openings
                + centered_cube(
                    format!("{BIN_PREFIX}_channel_visibility_window_{index}_clear_aperture"),
                    78.0,
                    36.0,
                    WINDOW_PANEL_Z + 2.0,
                )
                .translate(x, y, insert_z(WINDOW_PANEL_Z));
            ledges = ledges
                + rectangular_outline(
                    format!(
                        "{BIN_PREFIX}_channel_visibility_window_{index}_transparent_insert_lip"
                    ),
                    92.0,
                    50.0,
                    4.0,
                    5.0,
                )
                .translate(x, y, DECK_Z + WINDOW_PANEL_Z + 2.5);
        }
    }

    let mut warpage_rails = Part::empty(format!("{BIN_PREFIX}_warpage_straightedge_rail_set"));
    for rail in 0..WARPAGE_RAIL_COUNT {
        let x = WINDOW_CENTER.0 + centered_index(rail, WARPAGE_RAIL_COUNT, 82.0);
        warpage_rails = warpage_rails
            + centered_cube(
                format!("{BIN_PREFIX}_warpage_straightedge_rail_{rail}"),
                4.0,
                WINDOW_PANEL_Y - 30.0,
                14.0,
            )
            .translate(x, WINDOW_CENTER.1, DECK_Z + WINDOW_PANEL_Z + 7.0);
    }

    let sag_feeler = centered_cube(
        format!("{BIN_PREFIX}_warpage_feeler_gap_comb"),
        WINDOW_PANEL_X - 34.0,
        7.0,
        10.0,
    )
    .translate(
        WINDOW_CENTER.0,
        WINDOW_CENTER.1 - WINDOW_PANEL_Y / 2.0 + 16.0,
        DECK_Z + WINDOW_PANEL_Z + 5.0,
    );

    frame + ledges + warpage_rails + sag_feeler - openings
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_barcode_coa_custody_panel"),
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_PANEL_Z),
    );
    let mut barcode_lands = Part::empty(format!("{BIN_PREFIX}_incoming_lot_barcode_lands"));
    for index in 0..BARCODE_LAND_COUNT {
        barcode_lands = barcode_lands
            + centered_cube(
                format!("{BIN_PREFIX}_barcode_land_{index}"),
                42.0,
                15.0,
                3.0,
            )
            .translate(
                CUSTODY_CENTER.0 + centered_index(index % 3, 3, 76.0) - 38.0,
                CUSTODY_CENTER.1 + if index < 3 { -24.0 } else { 24.0 },
                DECK_Z + CUSTODY_PANEL_Z + 1.5,
            );
    }

    let mut coa_lands = Part::empty(format!("{BIN_PREFIX}_certificate_of_analysis_lands"));
    for index in 0..COA_LAND_COUNT {
        coa_lands = coa_lands
            + centered_cube(
                format!("{BIN_PREFIX}_coa_card_land_{index}"),
                50.0,
                32.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER.0 + CUSTODY_PANEL_X / 2.0 - 78.0,
                CUSTODY_CENTER.1 + centered_index(index, COA_LAND_COUNT, 36.0),
                DECK_Z + CUSTODY_PANEL_Z + 2.0,
            );
    }

    let tamper_seal_slots = centered_cube(
        format!("{BIN_PREFIX}_custody_tamper_seal_slot_left"),
        72.0,
        5.0,
        CUSTODY_PANEL_Z + 2.0,
    )
    .translate(
        CUSTODY_CENTER.0 - CUSTODY_PANEL_X / 2.0 + 52.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_PANEL_Z),
    ) + centered_cube(
        format!("{BIN_PREFIX}_custody_tamper_seal_slot_right"),
        72.0,
        5.0,
        CUSTODY_PANEL_Z + 2.0,
    )
    .translate(
        CUSTODY_CENTER.0 + 24.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_PANEL_Z),
    );

    panel + barcode_lands + coa_lands - tamper_seal_slots
}

fn quarantine_release_gate_lanes() -> Part {
    let base = centered_cube(
        format!("{BIN_PREFIX}_quarantine_release_gate_base"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, insert_z(GATE_PANEL_Z));
    let mut lanes = Part::empty(format!("{BIN_PREFIX}_release_hold_quarantine_lanes"));
    let mut shutters = Part::empty(format!("{BIN_PREFIX}_release_hold_quarantine_shutters"));
    let mut token_pockets = Part::empty(format!("{BIN_PREFIX}_release_gate_token_pockets"));

    for lane in 0..STATUS_LANE_COUNT {
        let lane_y = GATE_CENTER.1 + centered_index(lane, STATUS_LANE_COUNT, 58.0);
        let lane_name = match lane {
            0 => "release",
            1 => "hold",
            _ => "quarantine",
        };
        lanes = lanes
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_lane_raised_curb"),
                GATE_PANEL_X - 42.0,
                34.0,
                10.0,
            )
            .translate(GATE_CENTER.0, lane_y, DECK_Z + GATE_PANEL_Z + 5.0);
        shutters = shutters
            + centered_cube(
                format!("{BIN_PREFIX}_{lane_name}_sliding_gate_shutter"),
                12.0,
                44.0,
                42.0,
            )
            .translate(
                GATE_CENTER.0 - GATE_PANEL_X / 2.0 + 42.0 + lane as f64 * 18.0,
                lane_y,
                DECK_Z + GATE_PANEL_Z + 21.0,
            );

        for slot in 0..STATUS_TOKEN_SLOTS_PER_LANE {
            token_pockets = token_pockets
                + centered_cylinder(
                    format!("{BIN_PREFIX}_{lane_name}_token_socket_{slot}"),
                    7.0,
                    GATE_PANEL_Z + 3.0,
                    24,
                )
                .translate(
                    GATE_CENTER.0 + centered_index(slot, STATUS_TOKEN_SLOTS_PER_LANE, 31.0),
                    lane_y,
                    insert_z(GATE_PANEL_Z),
                );
        }
    }

    base + lanes + shutters - token_pockets
}

fn camera_evidence_bridge() -> Part {
    let mut posts = Part::empty(format!("{BIN_PREFIX}_camera_bridge_posts"));
    for (index, (x, y)) in [
        (
            BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0 + 44.0,
            BRIDGE_CENTER.1 - BRIDGE_FOOTPRINT_Y / 2.0 + 16.0,
        ),
        (
            BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0 - 44.0,
            BRIDGE_CENTER.1 - BRIDGE_FOOTPRINT_Y / 2.0 + 16.0,
        ),
        (
            BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0 + 44.0,
            BRIDGE_CENTER.1 + BRIDGE_FOOTPRINT_Y / 2.0 - 16.0,
        ),
        (
            BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0 - 44.0,
            BRIDGE_CENTER.1 + BRIDGE_FOOTPRINT_Y / 2.0 - 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{BIN_PREFIX}_camera_bridge_post_{index}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_UNDERSIDE_Z,
            )
            .translate(*x, *y, DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0);
    }

    let beam = centered_cube(
        format!("{BIN_PREFIX}_camera_evidence_bridge_beam"),
        BRIDGE_SPAN_X,
        38.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    let mut camera_pods = Part::empty(format!("{BIN_PREFIX}_camera_evidence_pods"));
    for camera in 0..CAMERA_COUNT {
        let x = BRIDGE_CENTER.0 + centered_index(camera, CAMERA_COUNT, 220.0);
        camera_pods = camera_pods
            + centered_cube(
                format!("{BIN_PREFIX}_camera_pod_{camera}"),
                86.0,
                66.0,
                46.0,
            )
            .translate(x, BRIDGE_CENTER.1, DECK_Z + BRIDGE_UNDERSIDE_Z - 23.0)
            + centered_cylinder(
                format!("{BIN_PREFIX}_camera_pod_{camera}_lens_bore"),
                11.0,
                48.0,
                32,
            )
            .translate(x, BRIDGE_CENTER.1, DECK_Z + BRIDGE_UNDERSIDE_Z - 47.0);
    }

    let mut light_rails = Part::empty(format!("{BIN_PREFIX}_raking_light_rails"));
    for rail in 0..LIGHT_RAIL_COUNT {
        let y = BRIDGE_CENTER.1 + if rail == 0 { -31.0 } else { 31.0 };
        light_rails = light_rails
            + centered_cube(
                format!("{BIN_PREFIX}_raking_light_rail_{rail}"),
                BRIDGE_SPAN_X - 150.0,
                9.0,
                12.0,
            )
            .translate(BRIDGE_CENTER.0, y, DECK_Z + BRIDGE_UNDERSIDE_Z - 18.0);
    }

    let evidence_card_shelf = centered_cube(
        format!("{BIN_PREFIX}_evidence_card_clip_shelf"),
        270.0,
        22.0,
        18.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1 + BRIDGE_FOOTPRINT_Y / 2.0 + 18.0,
        DECK_Z + BRIDGE_UNDERSIDE_Z - 24.0,
    );

    posts + beam + light_rails + evidence_card_shelf + camera_pods
}

fn robotic_service_datums() -> Part {
    let strip = centered_cube(
        format!("{BIN_PREFIX}_robotic_service_datum_strip"),
        SERVICE_STRIP_X,
        SERVICE_STRIP_Y,
        SERVICE_STRIP_Z,
    )
    .translate(
        SERVICE_CENTER.0,
        SERVICE_CENTER.1,
        insert_z(SERVICE_STRIP_Z),
    );
    let mut datums = Part::empty(format!("{BIN_PREFIX}_robotic_kinematic_service_datums"));
    for index in 0..ROBOTIC_SERVICE_DATUM_COUNT {
        let x = SERVICE_CENTER.0 + centered_index(index, ROBOTIC_SERVICE_DATUM_COUNT, 158.0);
        datums = datums
            + centered_cylinder(
                format!("{BIN_PREFIX}_robot_service_datum_socket_{index}"),
                9.0,
                SERVICE_STRIP_Z + 4.0,
                32,
            )
            .translate(x, SERVICE_CENTER.1 - 10.0, insert_z(SERVICE_STRIP_Z))
            + centered_cylinder(
                format!("{BIN_PREFIX}_robot_service_datum_post_{index}"),
                4.0,
                18.0,
                28,
            )
            .translate(x, SERVICE_CENTER.1 + 16.0, DECK_Z + SERVICE_STRIP_Z + 9.0);
    }

    let mut fork_gauges = Part::empty(format!("{BIN_PREFIX}_robot_gripper_fork_gauges"));
    for index in 0..ROBOT_FORK_GAUGE_COUNT {
        let x = SERVICE_CENTER.0 + centered_index(index, ROBOT_FORK_GAUGE_COUNT, 210.0);
        fork_gauges = fork_gauges
            + centered_cube(
                format!("{BIN_PREFIX}_robot_gripper_fork_clearance_gauge_{index}_left"),
                9.0,
                36.0,
                36.0,
            )
            .translate(x - 16.0, SERVICE_CENTER.1, DECK_Z + SERVICE_STRIP_Z + 18.0)
            + centered_cube(
                format!("{BIN_PREFIX}_robot_gripper_fork_clearance_gauge_{index}_right"),
                9.0,
                36.0,
                36.0,
            )
            .translate(x + 16.0, SERVICE_CENTER.1, DECK_Z + SERVICE_STRIP_Z + 18.0);
    }

    let sweep_keepout = wireframe_box(
        format!("{BIN_PREFIX}_front_robot_sweep_keepout_wireframe"),
        SERVICE_STRIP_X - 80.0,
        76.0,
        120.0,
        5.0,
    )
    .translate(
        SERVICE_CENTER.0,
        SERVICE_CENTER.1 + 14.0,
        DECK_Z + SERVICE_STRIP_Z + 60.0,
    );

    strip + fork_gauges + sweep_keepout - datum_socket_cuts() + datums
}

fn datum_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{BIN_PREFIX}_robotic_service_datum_socket_cuts"));
    for index in 0..ROBOTIC_SERVICE_DATUM_COUNT {
        let x = SERVICE_CENTER.0 + centered_index(index, ROBOTIC_SERVICE_DATUM_COUNT, 158.0);
        cuts = cuts
            + centered_cylinder(
                format!("{BIN_PREFIX}_robot_service_datum_socket_cut_{index}"),
                6.0,
                SERVICE_STRIP_Z + 5.0,
                28,
            )
            .translate(x, SERVICE_CENTER.1 - 10.0, insert_z(SERVICE_STRIP_Z));
    }
    cuts
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{BIN_PREFIX}_deck_insert_socket_cuts"));
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("{BIN_PREFIX}_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, DECK_Z - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn perimeter_rims() -> Part {
    centered_cube(
        format!("{BIN_PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0)
        + centered_cube(
            format!("{BIN_PREFIX}_right_containment_rim"),
            RIM_W,
            STATION_Y,
            RIM_Z,
        )
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0)
        + centered_cube(
            format!("{BIN_PREFIX}_rear_containment_rim"),
            STATION_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0)
        + centered_cube(
            format!("{BIN_PREFIX}_front_left_robot_entry_lip"),
            360.0,
            RIM_W,
            20.0,
        )
        .translate(
            -STATION_X / 2.0 + 180.0,
            -STATION_Y / 2.0 + RIM_W / 2.0,
            DECK_Z + 10.0,
        )
        + centered_cube(
            format!("{BIN_PREFIX}_front_right_robot_entry_lip"),
            360.0,
            RIM_W,
            20.0,
        )
        .translate(
            STATION_X / 2.0 - 180.0,
            -STATION_Y / 2.0 + RIM_W / 2.0,
            DECK_Z + 10.0,
        )
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{BIN_PREFIX}_m6_mount_hole_array"));
    for (index, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{BIN_PREFIX}_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 248.0, 0.0),
        (STATION_X / 2.0 - 248.0, 0.0),
        (-86.0, STATION_Y / 2.0 - 58.0),
        (86.0, STATION_Y / 2.0 - 58.0),
    ]
}

fn datum_button(name: impl Into<String>, x: f64, y: f64) -> Part {
    centered_cylinder(name, 2.2, 6.0, 24).translate(x, y, DECK_Z + NEST_BLOCK_Z + 3.0)
}

fn rectangular_outline(name: impl Into<String>, sx: f64, sy: f64, rail: f64, z: f64) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_front"), sx, rail, z).translate(0.0, -sy / 2.0 + rail / 2.0, 0.0)
        + centered_cube(format!("{name}_rear"), sx, rail, z).translate(
            0.0,
            sy / 2.0 - rail / 2.0,
            0.0,
        )
        + centered_cube(format!("{name}_left"), rail, sy, z).translate(
            -sx / 2.0 + rail / 2.0,
            0.0,
            0.0,
        )
        + centered_cube(format!("{name}_right"), rail, sy, z).translate(
            sx / 2.0 - rail / 2.0,
            0.0,
            0.0,
        )
}

fn wireframe_box(name: impl Into<String>, sx: f64, sy: f64, sz: f64, rail: f64) -> Part {
    let name = name.into();
    let bottom = rectangular_outline(format!("{name}_bottom"), sx, sy, rail, rail).translate(
        0.0,
        0.0,
        -sz / 2.0 + rail / 2.0,
    );
    let top = rectangular_outline(format!("{name}_top"), sx, sy, rail, rail).translate(
        0.0,
        0.0,
        sz / 2.0 - rail / 2.0,
    );
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (index, (x, y)) in [
        (-sx / 2.0 + rail / 2.0, -sy / 2.0 + rail / 2.0),
        (sx / 2.0 - rail / 2.0, -sy / 2.0 + rail / 2.0),
        (-sx / 2.0 + rail / 2.0, sy / 2.0 - rail / 2.0),
        (sx / 2.0 - rail / 2.0, sy / 2.0 - rail / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{index}"), rail, rail, sz)
                .translate(*x, *y, 0.0);
    }
    bottom + top + posts
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn slot_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, CHIP_SLOT_COLS, CHIP_SLOT_PITCH_X),
        centered_index(row, CHIP_SLOT_ROWS, CHIP_SLOT_PITCH_Y),
    )
}

fn port_x(port: usize) -> f64 {
    match port {
        0 => -PORT_PITCH_X / 2.0,
        1 => PORT_PITCH_X / 2.0,
        _ => panic!("unsupported port index {port}"),
    }
}

fn gauge_rail_gap(lane: usize) -> f64 {
    match lane {
        0 => CHIP_Y + 0.7,
        1 => CHIP_Y + 0.15,
        2 => CHIP_Y - 0.45,
        _ => panic!("unsupported gauge lane {lane}"),
    }
}

fn bridge_clearance_over_tallest_fixture() -> f64 {
    let tallest_insert = GATE_PANEL_Z + 42.0;
    BRIDGE_UNDERSIDE_Z - tallest_insert
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "scaled_cassette_datum_nests",
            NEST_CENTER,
            NEST_BLOCK_X,
            NEST_BLOCK_Y,
        ),
        (
            "go_no_go_gauge_rails",
            GAUGE_CENTER,
            GAUGE_PANEL_X,
            GAUGE_PANEL_Y,
        ),
        (
            "port_pitch_alignment_combs",
            COMB_CENTER,
            COMB_PANEL_X,
            COMB_PANEL_Y,
        ),
        (
            "gasket_land_compression_witnesses",
            GASKET_CENTER,
            GASKET_PANEL_X,
            GASKET_PANEL_Y,
        ),
        (
            "transparent_channel_warpage_windows",
            WINDOW_CENTER,
            WINDOW_PANEL_X,
            WINDOW_PANEL_Y,
        ),
        (
            "barcode_coa_custody_lands",
            CUSTODY_CENTER,
            CUSTODY_PANEL_X,
            CUSTODY_PANEL_Y,
        ),
        (
            "quarantine_release_gate_lanes",
            GATE_CENTER,
            GATE_PANEL_X,
            GATE_PANEL_Y,
        ),
        (
            "camera_evidence_bridge",
            BRIDGE_CENTER,
            BRIDGE_SPAN_X,
            BRIDGE_FOOTPRINT_Y,
        ),
        (
            "robotic_service_datums",
            SERVICE_CENTER,
            SERVICE_STRIP_X,
            SERVICE_STRIP_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 4.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 4.0
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds the station envelope"
        );
    }
    assert!(
        bridge_clearance_over_tallest_fixture() > 130.0,
        "camera bridge clearance is insufficient"
    );
    assert_eq!(
        CHIP_SLOT_COUNT, 12,
        "scaled cassette must expose twelve chip datum nests"
    );
    assert_eq!(
        PORT_TINE_COUNT, 24,
        "port pitch comb must check two ports on every chip"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_set_is_scoped_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_microfluidic_chip_lot_dimensional_metrology_fit_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_represented() {
        for feature in [
            "chip_datum_nests",
            "go_no_go_gauge_rails",
            "port_pitch_alignment_combs",
            "gasket_land_compression_witnesses",
            "transparent_channel_warpage_inspection_windows",
            "barcode_coa_custody_lands",
            "quarantine_release_gates",
            "camera_evidence_bridge",
            "robotic_service_datums",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), OUTPUTS.len() - 1);
    }

    #[test]
    fn cassette_capacity_and_datum_counts_match_scaled_lot_fixture() {
        assert_eq!(CHIP_SLOT_ROWS, 3);
        assert_eq!(CHIP_SLOT_COLS, 4);
        assert_eq!(CHIP_SLOT_COUNT, 12);
        assert_eq!(CHIP_DATUM_COUNT, 36);
        assert_eq!(GASKET_RING_COUNT, CHIP_SLOT_COUNT);
    }

    #[test]
    fn port_pitch_comb_checks_every_chip_port() {
        assert_eq!(PORTS_PER_CHIP, 2);
        assert_eq!(PORT_TINE_COUNT, 24);
        assert_eq!(port_x(1) - port_x(0), PORT_PITCH_X);
        assert!(PORT_PITCH_X > CHIP_Y);
    }

    #[test]
    fn go_no_go_rails_span_nominal_chip_width() {
        assert!(gauge_rail_gap(0) > CHIP_Y);
        assert!(gauge_rail_gap(1) > CHIP_Y);
        assert!(gauge_rail_gap(2) < CHIP_Y);
        assert_eq!(GO_NO_GO_RAIL_PAIR_COUNT, 3);
    }

    #[test]
    fn inspection_custody_gate_and_robot_counts_are_visible() {
        assert_eq!(INSPECTION_WINDOW_COUNT, 6);
        assert_eq!(WARPAGE_RAIL_COUNT, 4);
        assert_eq!(BARCODE_LAND_COUNT, 6);
        assert_eq!(COA_LAND_COUNT, 3);
        assert_eq!(STATUS_LANE_COUNT, 3);
        assert_eq!(ROBOTIC_SERVICE_DATUM_COUNT, 6);
        assert_eq!(ROBOT_FORK_GAUGE_COUNT, 4);
    }

    #[test]
    fn station_layout_and_camera_clearance_are_bounded() {
        assert_layout();
        assert!(insert_specs()
            .iter()
            .all(|(_, pos, width, depth)| fits_on_station(*pos, *width, *depth)));
        assert!(bridge_clearance_over_tallest_fixture() > 130.0);
    }
}
