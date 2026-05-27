use vcad::{centered_cube, centered_cylinder, Part};

// Automated media sampling and analyzer interface module for tissue-chip culture
// monitoring.
//
// Intent:
// - Route sampled media from parallel chip lanes without manual pipetting.
// - Provide per-lane selector placeholders, sample-loop seats, flush/waste
//   routing, bubble/dead-volume control, and a sterile bulkhead panel.
// - Register a small fraction-collector/cold-block plate interface with barcode
//   and position fiducials, plus an optional glucose/lactate analyzer docking
//   envelope for purchased instrumentation.
// - Keep service access front-facing and separate sterile, waste, analyzer, and
//   fraction-collection interfaces.
//
// This is mechanical architecture and packaging CAD. It does not claim sterile
// validation, fluidic calibration, or analyzer compatibility.

const OUTPUTS: &[&str] = &[
    "output/media_sampling_analyzer_interface_baseplate.stl",
    "output/media_sampling_analyzer_interface_selector_manifold.stl",
    "output/media_sampling_analyzer_interface_flush_waste_manifold.stl",
    "output/media_sampling_analyzer_interface_fraction_collector_cold_block_nest.stl",
    "output/media_sampling_analyzer_interface_analyzer_dock.stl",
    "output/media_sampling_analyzer_interface_sterile_bulkhead_panel.stl",
    "output/media_sampling_analyzer_interface_bubble_dead_volume_control.stl",
    "output/media_sampling_analyzer_interface_service_access_cover.stl",
    "output/media_sampling_analyzer_interface_assembly.stl",
];

const LANES: usize = 8;
const SELECTOR_STATES: usize = 4;
const FRACTION_COLS: usize = 12;
const FRACTION_ROWS: usize = 8;
const FRACTION_WELLS: usize = FRACTION_COLS * FRACTION_ROWS;

const BASE_X: f64 = 760.0;
const BASE_Y: f64 = 430.0;
const BASE_Z: f64 = 16.0;
const PANEL_X: f64 = 700.0;
const PANEL_Y: f64 = 24.0;
const PANEL_Z: f64 = 286.0;
const PANEL_BASE_Y: f64 = BASE_Y / 2.0 - 48.0;

const SELECTOR_BLOCK_X: f64 = 620.0;
const SELECTOR_BLOCK_Y: f64 = 76.0;
const SELECTOR_BLOCK_Z: f64 = 44.0;
const LANE_PITCH_X: f64 = 70.0;
const SELECTOR_ROTOR_D: f64 = 25.0;
const SAMPLE_LOOP_LAND_X: f64 = 42.0;
const SAMPLE_LOOP_LAND_Y: f64 = 36.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.7;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const FLUSH_BLOCK_X: f64 = 560.0;
const FLUSH_BLOCK_Y: f64 = 66.0;
const FLUSH_BLOCK_Z: f64 = 46.0;
const WASTE_BORE_D: f64 = 8.0;
const FLUSH_BORE_D: f64 = 5.5;

const COLD_NEST_X: f64 = 236.0;
const COLD_NEST_Y: f64 = 166.0;
const COLD_NEST_Z: f64 = 42.0;
const COLD_BLOCK_POCKET_X: f64 = 154.0;
const COLD_BLOCK_POCKET_Y: f64 = 112.0;
const COLD_BLOCK_POCKET_DEPTH: f64 = 12.0;
const PLATE_X: f64 = 127.76;
const PLATE_Y: f64 = 85.48;
const PLATE_POCKET_DEPTH: f64 = 4.0;
const WELL96_PITCH: f64 = 9.0;
const WELL96_A1_X: f64 = 14.38;
const WELL96_A1_Y: f64 = 11.24;
const BARCODE_LAND_X: f64 = 82.0;
const BARCODE_LAND_Y: f64 = 18.0;

const ANALYZER_DOCK_X: f64 = 238.0;
const ANALYZER_DOCK_Y: f64 = 194.0;
const ANALYZER_DOCK_Z: f64 = 142.0;
const ANALYZER_ENV_X: f64 = 202.0;
const ANALYZER_ENV_Y: f64 = 148.0;
const ANALYZER_ENV_Z: f64 = 118.0;

const BUBBLE_BANK_X: f64 = 600.0;
const BUBBLE_BANK_Y: f64 = 54.0;
const BUBBLE_BANK_Z: f64 = 78.0;
const BUBBLE_CHAMBER_D: f64 = 18.0;
const DEAD_VOLUME_PURGE_D: f64 = 3.2;

const SERVICE_COVER_X: f64 = 704.0;
const SERVICE_COVER_Y: f64 = 96.0;
const SERVICE_COVER_Z: f64 = 92.0;

const SELECTOR_X: f64 = 0.0;
const SELECTOR_Y: f64 = 26.0;
const FLUSH_Y: f64 = 104.0;
const COLD_NEST_X_POS: f64 = -204.0;
const COLD_NEST_Y_POS: f64 = -115.0;
const ANALYZER_X_POS: f64 = 214.0;
const ANALYZER_Y_POS: f64 = -112.0;
const BUBBLE_Y_POS: f64 = 126.0;

fn main() {
    let baseplate = baseplate();
    export(&baseplate, OUTPUTS[0]);

    let selector = selector_manifold();
    export(&selector, OUTPUTS[1]);

    let flush_waste = flush_waste_manifold();
    export(&flush_waste, OUTPUTS[2]);

    let cold_nest = fraction_collector_cold_block_nest();
    export(&cold_nest, OUTPUTS[3]);

    let analyzer = analyzer_dock();
    export(&analyzer, OUTPUTS[4]);

    let bulkhead = sterile_bulkhead_panel();
    export(&bulkhead, OUTPUTS[5]);

    let bubble_bank = bubble_dead_volume_control_bank();
    export(&bubble_bank, OUTPUTS[6]);

    let service_cover = service_access_cover();
    export(&service_cover, OUTPUTS[7]);

    let assembly = baseplate
        + selector.translate(
            SELECTOR_X,
            SELECTOR_Y,
            BASE_Z / 2.0 + SELECTOR_BLOCK_Z / 2.0,
        )
        + flush_waste.translate(0.0, FLUSH_Y, BASE_Z / 2.0 + FLUSH_BLOCK_Z / 2.0)
        + cold_nest.translate(
            COLD_NEST_X_POS,
            COLD_NEST_Y_POS,
            BASE_Z / 2.0 + COLD_NEST_Z / 2.0,
        )
        + analyzer.translate(
            ANALYZER_X_POS,
            ANALYZER_Y_POS,
            BASE_Z / 2.0 + ANALYZER_DOCK_Z / 2.0,
        )
        + bulkhead.translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 + PANEL_Z / 2.0)
        + bubble_bank.translate(0.0, BUBBLE_Y_POS, BASE_Z / 2.0 + BUBBLE_BANK_Z / 2.0)
        + service_cover.translate(
            0.0,
            -(BASE_Y / 2.0 - 52.0),
            BASE_Z / 2.0 + SERVICE_COVER_Z / 2.0,
        );

    export(&assembly, OUTPUTS[8]);

    println!(
        "Media sampling/analyzer interface: {:.0}mm x {:.0}mm bench module, {} tissue-chip lanes, {} selector states per lane, {}-well fraction collector/cold-block pocket, optional {:.0}mm x {:.0}mm x {:.0}mm analyzer envelope, sterile bulkhead panel, bubble traps, purge/dead-volume controls, and front service access.",
        BASE_X,
        BASE_Y,
        LANES,
        SELECTOR_STATES,
        FRACTION_WELLS,
        ANALYZER_ENV_X,
        ANALYZER_ENV_Y,
        ANALYZER_ENV_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "media_sampling_interface_baseplate_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );

    let spill_sump = centered_cube(
        "media_sampling_interface_front_spill_sump",
        BASE_X - 92.0,
        74.0,
        8.0,
    )
    .translate(0.0, -(BASE_Y / 2.0 - 70.0), BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder("media_sampling_interface_sump_drain", 6.5 / 2.0, 34.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(BASE_X / 2.0 - 58.0, -(BASE_Y / 2.0 - 30.0), 0.0);
    let panel_socket = centered_cube(
        "media_sampling_interface_bulkhead_panel_socket",
        PANEL_X + 24.0,
        18.0,
        8.0,
    )
    .translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 - 2.0);
    let analyzer_cable_trough = centered_cube(
        "media_sampling_interface_analyzer_cable_trough",
        176.0,
        26.0,
        BASE_Z + 2.0,
    )
    .translate(
        ANALYZER_X_POS,
        ANALYZER_Y_POS + ANALYZER_DOCK_Y / 2.0 - 18.0,
        0.0,
    );
    let collector_pull_pocket = centered_cube(
        "media_sampling_interface_collector_pull_pocket",
        134.0,
        28.0,
        BASE_Z + 2.0,
    )
    .translate(COLD_NEST_X_POS, -(BASE_Y / 2.0 - 22.0), 0.0);

    let mut mount_holes = Part::empty("media_sampling_interface_base_mount_holes");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let slot = centered_cube(
            format!("media_sampling_interface_base_m5_slot_{i}"),
            21.0,
            5.6,
            BASE_Z + 2.0,
        )
        .translate(*x, *y, 0.0);
        let hole = centered_cylinder(
            format!("media_sampling_interface_base_m5_clearance_{i}"),
            5.6 / 2.0,
            BASE_Z + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        mount_holes = mount_holes + slot + hole;
    }

    deck - spill_sump
        - drain
        - panel_socket
        - analyzer_cable_trough
        - collector_pull_pocket
        - mount_holes
        + base_perimeter_rails()
        + module_locator_bosses()
        + collector_slide_rails()
        + selector_mount_rails()
        + analyzer_dock_alignment_rails()
        + base_leveling_feet()
}

fn base_perimeter_rails() -> Part {
    let left = centered_cube(
        "media_sampling_interface_left_base_guard",
        18.0,
        BASE_Y - 54.0,
        24.0,
    )
    .translate(-(BASE_X / 2.0 - 26.0), 0.0, BASE_Z / 2.0 + 12.0);
    let right = centered_cube(
        "media_sampling_interface_right_base_guard",
        18.0,
        BASE_Y - 54.0,
        24.0,
    )
    .translate(BASE_X / 2.0 - 26.0, 0.0, BASE_Z / 2.0 + 12.0);
    let rear = centered_cube(
        "media_sampling_interface_rear_base_guard",
        BASE_X - 64.0,
        18.0,
        24.0,
    )
    .translate(0.0, BASE_Y / 2.0 - 28.0, BASE_Z / 2.0 + 12.0);
    let front_lip = centered_cube(
        "media_sampling_interface_front_service_lip",
        BASE_X - 150.0,
        12.0,
        18.0,
    )
    .translate(0.0, -(BASE_Y / 2.0 - 20.0), BASE_Z / 2.0 + 9.0);

    left + right + rear + front_lip
}

fn module_locator_bosses() -> Part {
    let mut bosses = Part::empty("media_sampling_interface_module_locator_bosses");
    for (i, (x, y)) in [
        (SELECTOR_X - SELECTOR_BLOCK_X / 2.0 + 36.0, SELECTOR_Y),
        (SELECTOR_X + SELECTOR_BLOCK_X / 2.0 - 36.0, SELECTOR_Y),
        (COLD_NEST_X_POS - COLD_NEST_X / 2.0 + 24.0, COLD_NEST_Y_POS),
        (COLD_NEST_X_POS + COLD_NEST_X / 2.0 - 24.0, COLD_NEST_Y_POS),
        (
            ANALYZER_X_POS - ANALYZER_DOCK_X / 2.0 + 30.0,
            ANALYZER_Y_POS,
        ),
        (
            ANALYZER_X_POS + ANALYZER_DOCK_X / 2.0 - 30.0,
            ANALYZER_Y_POS,
        ),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("media_sampling_interface_locator_boss_{i}"),
            10.0,
            8.0,
            28,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        let socket = centered_cylinder(
            format!("media_sampling_interface_locator_socket_{i}"),
            3.2 / 2.0,
            10.0,
            20,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        bosses = bosses + (boss - socket);
    }
    bosses
}

fn collector_slide_rails() -> Part {
    let left = centered_cube(
        "media_sampling_interface_collector_left_slide_rail",
        COLD_NEST_X + 36.0,
        10.0,
        12.0,
    )
    .translate(
        COLD_NEST_X_POS,
        COLD_NEST_Y_POS - COLD_NEST_Y / 2.0 - 10.0,
        BASE_Z / 2.0 + 6.0,
    );
    let right = centered_cube(
        "media_sampling_interface_collector_right_slide_rail",
        COLD_NEST_X + 36.0,
        10.0,
        12.0,
    )
    .translate(
        COLD_NEST_X_POS,
        COLD_NEST_Y_POS + COLD_NEST_Y / 2.0 + 10.0,
        BASE_Z / 2.0 + 6.0,
    );
    let rear_stop = centered_cube(
        "media_sampling_interface_collector_rear_stop",
        COLD_NEST_X + 20.0,
        12.0,
        18.0,
    )
    .translate(
        COLD_NEST_X_POS,
        COLD_NEST_Y_POS + COLD_NEST_Y / 2.0 + 24.0,
        BASE_Z / 2.0 + 9.0,
    );

    left + right + rear_stop
}

fn selector_mount_rails() -> Part {
    let front = centered_cube(
        "media_sampling_interface_selector_front_mount_rail",
        SELECTOR_BLOCK_X + 44.0,
        12.0,
        14.0,
    )
    .translate(
        SELECTOR_X,
        SELECTOR_Y - SELECTOR_BLOCK_Y / 2.0 - 12.0,
        BASE_Z / 2.0 + 7.0,
    );
    let rear = centered_cube(
        "media_sampling_interface_selector_rear_mount_rail",
        SELECTOR_BLOCK_X + 44.0,
        12.0,
        14.0,
    )
    .translate(
        SELECTOR_X,
        SELECTOR_Y + SELECTOR_BLOCK_Y / 2.0 + 12.0,
        BASE_Z / 2.0 + 7.0,
    );
    front + rear
}

fn analyzer_dock_alignment_rails() -> Part {
    let left = centered_cube(
        "media_sampling_interface_analyzer_left_alignment_rail",
        12.0,
        ANALYZER_DOCK_Y + 28.0,
        12.0,
    )
    .translate(
        ANALYZER_X_POS - ANALYZER_DOCK_X / 2.0 - 10.0,
        ANALYZER_Y_POS,
        BASE_Z / 2.0 + 6.0,
    );
    let right = centered_cube(
        "media_sampling_interface_analyzer_right_alignment_rail",
        12.0,
        ANALYZER_DOCK_Y + 28.0,
        12.0,
    )
    .translate(
        ANALYZER_X_POS + ANALYZER_DOCK_X / 2.0 + 10.0,
        ANALYZER_Y_POS,
        BASE_Z / 2.0 + 6.0,
    );
    let rear_stop = centered_cube(
        "media_sampling_interface_analyzer_rear_alignment_stop",
        ANALYZER_DOCK_X + 20.0,
        12.0,
        16.0,
    )
    .translate(
        ANALYZER_X_POS,
        ANALYZER_Y_POS + ANALYZER_DOCK_Y / 2.0 + 14.0,
        BASE_Z / 2.0 + 8.0,
    );

    left + right + rear_stop
}

fn base_leveling_feet() -> Part {
    let mut feet = Part::empty("media_sampling_interface_leveling_feet");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let pad = centered_cylinder(
            format!("media_sampling_interface_leveling_pad_{i}"),
            18.0,
            8.0,
            36,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 4.0));
        let stem_clearance = centered_cylinder(
            format!("media_sampling_interface_leveling_stem_clearance_{i}"),
            7.0 / 2.0,
            12.0,
            20,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 4.0));
        feet = feet + (pad - stem_clearance);
    }
    feet
}

fn selector_manifold() -> Part {
    let body = centered_cube(
        "media_sampling_selector_manifold_body",
        SELECTOR_BLOCK_X,
        SELECTOR_BLOCK_Y,
        SELECTOR_BLOCK_Z,
    );

    let mut cuts = Part::empty("media_sampling_selector_manifold_cuts");
    let mut overlays = Part::empty("media_sampling_selector_position_features");
    for lane in 0..LANES {
        let x = lane_x(lane);
        let main_bore = centered_cylinder(
            format!("media_sampling_lane_{lane}_through_loop_bore"),
            FLUID_BORE_D / 2.0,
            SELECTOR_BLOCK_Y + 10.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, -2.0);
        let sample_branch = centered_cylinder(
            format!("media_sampling_lane_{lane}_sample_branch_bore"),
            FLUID_BORE_D / 2.0,
            SAMPLE_LOOP_LAND_Y + 18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -10.0, 12.0);
        let rotor_pocket = centered_cylinder(
            format!("media_sampling_lane_{lane}_selector_rotor_placeholder"),
            SELECTOR_ROTOR_D / 2.0,
            12.0,
            36,
        )
        .translate(x, -2.0, SELECTOR_BLOCK_Z / 2.0 - 5.0);
        let handle_clearance = centered_cube(
            format!("media_sampling_lane_{lane}_selector_handle_slot"),
            SELECTOR_ROTOR_D + 12.0,
            7.0,
            12.0,
        )
        .translate(x, -2.0, SELECTOR_BLOCK_Z / 2.0 - 5.0);
        let loop_land_recess = centered_cube(
            format!("media_sampling_lane_{lane}_sample_loop_land_recess"),
            SAMPLE_LOOP_LAND_X,
            SAMPLE_LOOP_LAND_Y,
            7.0,
        )
        .translate(
            x,
            -(SELECTOR_BLOCK_Y / 2.0 - 18.0),
            SELECTOR_BLOCK_Z / 2.0 - 3.5,
        );
        cuts =
            cuts + main_bore + sample_branch + rotor_pocket + handle_clearance + loop_land_recess;

        let loop_clip_left = centered_cube(
            format!("media_sampling_lane_{lane}_sample_loop_left_clip"),
            5.0,
            SAMPLE_LOOP_LAND_Y + 8.0,
            10.0,
        )
        .translate(
            x - SAMPLE_LOOP_LAND_X / 2.0 - 4.0,
            -(SELECTOR_BLOCK_Y / 2.0 - 18.0),
            SELECTOR_BLOCK_Z / 2.0 + 5.0,
        );
        let loop_clip_right = centered_cube(
            format!("media_sampling_lane_{lane}_sample_loop_right_clip"),
            5.0,
            SAMPLE_LOOP_LAND_Y + 8.0,
            10.0,
        )
        .translate(
            x + SAMPLE_LOOP_LAND_X / 2.0 + 4.0,
            -(SELECTOR_BLOCK_Y / 2.0 - 18.0),
            SELECTOR_BLOCK_Z / 2.0 + 5.0,
        );
        let position_detents =
            selector_detents(lane).translate(x, -2.0, SELECTOR_BLOCK_Z / 2.0 + 2.0);
        overlays = overlays + loop_clip_left + loop_clip_right + position_detents;
    }

    let sample_bus = centered_cylinder(
        "media_sampling_selector_common_sample_bus",
        FLUID_BORE_D / 2.0,
        SELECTOR_BLOCK_X - 48.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(SELECTOR_BLOCK_Y / 2.0 - 16.0), 12.0);
    let flush_bus = centered_cylinder(
        "media_sampling_selector_common_flush_bus",
        FLUID_BORE_D / 2.0,
        SELECTOR_BLOCK_X - 48.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, SELECTOR_BLOCK_Y / 2.0 - 16.0, 12.0);
    cuts = cuts
        + sample_bus
        + flush_bus
        + selector_mount_holes(
            "selector_manifold",
            SELECTOR_BLOCK_X,
            SELECTOR_BLOCK_Y,
            SELECTOR_BLOCK_Z,
        );

    body - cuts + overlays + selector_label_lands()
}

fn selector_detents(lane: usize) -> Part {
    let mut detents = Part::empty(format!("media_sampling_lane_{lane}_selector_detents"));
    for state in 0..SELECTOR_STATES {
        let angle_index = state as f64 - (SELECTOR_STATES as f64 - 1.0) / 2.0;
        let x = angle_index * 6.8;
        let dot = centered_cylinder(
            format!("media_sampling_lane_{lane}_selector_state_{state}_detent"),
            1.3,
            2.0,
            16,
        )
        .translate(x, SELECTOR_ROTOR_D / 2.0 + 4.0, 0.0);
        detents = detents + dot;
    }
    detents
}

fn selector_label_lands() -> Part {
    let lane_strip = centered_cube(
        "media_sampling_selector_lane_label_strip",
        SELECTOR_BLOCK_X - 44.0,
        4.0,
        8.0,
    )
    .translate(
        0.0,
        -(SELECTOR_BLOCK_Y / 2.0 + 2.0),
        SELECTOR_BLOCK_Z / 2.0 - 12.0,
    );
    let state_strip = centered_cube(
        "media_sampling_selector_state_label_strip",
        SELECTOR_BLOCK_X - 44.0,
        4.0,
        8.0,
    )
    .translate(
        0.0,
        SELECTOR_BLOCK_Y / 2.0 + 2.0,
        SELECTOR_BLOCK_Z / 2.0 - 12.0,
    );
    lane_strip + state_strip
}

fn selector_mount_holes(name: &str, x_span: f64, y_span: f64, z_span: f64) -> Part {
    let mut holes = Part::empty(format!("{name}_mount_holes"));
    for (i, (x, y)) in [
        (-(x_span / 2.0 - 22.0), -(y_span / 2.0 - 14.0)),
        (x_span / 2.0 - 22.0, -(y_span / 2.0 - 14.0)),
        (-(x_span / 2.0 - 22.0), y_span / 2.0 - 14.0),
        (x_span / 2.0 - 22.0, y_span / 2.0 - 14.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(format!("{name}_m4_mount_{i}"), 4.3 / 2.0, z_span + 2.0, 22)
                .translate(*x, *y, 0.0);
    }
    holes
}

fn flush_waste_manifold() -> Part {
    let body = centered_cube(
        "media_sampling_flush_waste_manifold_body",
        FLUSH_BLOCK_X,
        FLUSH_BLOCK_Y,
        FLUSH_BLOCK_Z,
    );

    let waste_bus = centered_cylinder(
        "media_sampling_flush_waste_common_waste_bus",
        WASTE_BORE_D / 2.0,
        FLUSH_BLOCK_X - 42.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -18.0, -6.0);
    let flush_bus = centered_cylinder(
        "media_sampling_flush_waste_common_flush_bus",
        FLUSH_BORE_D / 2.0,
        FLUSH_BLOCK_X - 42.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, 8.0);

    let mut cuts = waste_bus + flush_bus;
    let mut controls = Part::empty("media_sampling_flush_waste_lane_controls");
    for lane in 0..LANES {
        let x = lane_x(lane);
        let lane_drop = centered_cylinder(
            format!("media_sampling_lane_{lane}_waste_drop_bore"),
            FLUID_BORE_D / 2.0,
            FLUSH_BLOCK_Z + 4.0,
            24,
        )
        .translate(x, -18.0, 0.0);
        let lane_flush_riser = centered_cylinder(
            format!("media_sampling_lane_{lane}_flush_riser_bore"),
            FLUID_BORE_D / 2.0,
            FLUSH_BLOCK_Z + 4.0,
            24,
        )
        .translate(x, 18.0, 0.0);
        let diverter_pocket = centered_cylinder(
            format!("media_sampling_lane_{lane}_flush_waste_diverter_placeholder"),
            10.5,
            9.0,
            28,
        )
        .translate(x, 0.0, FLUSH_BLOCK_Z / 2.0 - 4.0);
        cuts = cuts + lane_drop + lane_flush_riser + diverter_pocket;

        let tee_label_land = centered_cube(
            format!("media_sampling_lane_{lane}_flush_waste_label_land"),
            34.0,
            4.0,
            8.0,
        )
        .translate(x, -(FLUSH_BLOCK_Y / 2.0 + 2.0), FLUSH_BLOCK_Z / 2.0 - 12.0);
        controls = controls + tee_label_land;
    }

    let waste_bottle_port = centered_cylinder(
        "media_sampling_flush_waste_bottle_bulkhead_cutout",
        14.0 / 2.0,
        FLUSH_BLOCK_Y + 8.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FLUSH_BLOCK_X / 2.0 - 32.0, -4.0, -6.0);
    let flush_inlet_port = centered_cylinder(
        "media_sampling_flush_waste_flush_inlet_bulkhead_cutout",
        10.0 / 2.0,
        FLUSH_BLOCK_Y + 8.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(FLUSH_BLOCK_X / 2.0 - 32.0), 4.0, 8.0);
    cuts = cuts
        + waste_bottle_port
        + flush_inlet_port
        + selector_mount_holes(
            "media_sampling_flush_waste",
            FLUSH_BLOCK_X,
            FLUSH_BLOCK_Y,
            FLUSH_BLOCK_Z,
        );

    body - cuts + controls + waste_bulkhead_lugs()
}

fn waste_bulkhead_lugs() -> Part {
    let left = centered_cube(
        "media_sampling_waste_bottle_bulkhead_left_lug",
        12.0,
        22.0,
        18.0,
    )
    .translate(
        FLUSH_BLOCK_X / 2.0 - 54.0,
        -(FLUSH_BLOCK_Y / 2.0 + 10.0),
        -6.0,
    );
    let right = centered_cube(
        "media_sampling_waste_bottle_bulkhead_right_lug",
        12.0,
        22.0,
        18.0,
    )
    .translate(
        FLUSH_BLOCK_X / 2.0 - 10.0,
        -(FLUSH_BLOCK_Y / 2.0 + 10.0),
        -6.0,
    );
    let flush_lug = centered_cube("media_sampling_flush_source_bulkhead_lug", 62.0, 18.0, 16.0)
        .translate(
            -(FLUSH_BLOCK_X / 2.0 - 32.0),
            FLUSH_BLOCK_Y / 2.0 + 9.0,
            8.0,
        );

    left + right + flush_lug
}

fn fraction_collector_cold_block_nest() -> Part {
    let body = centered_cube(
        "media_sampling_fraction_collector_cold_block_nest_body",
        COLD_NEST_X,
        COLD_NEST_Y,
        COLD_NEST_Z,
    );

    let cold_block_pocket = centered_cube(
        "media_sampling_fraction_collector_cold_block_pocket",
        COLD_BLOCK_POCKET_X,
        COLD_BLOCK_POCKET_Y,
        COLD_BLOCK_POCKET_DEPTH + 1.0,
    )
    .translate(0.0, 8.0, COLD_NEST_Z / 2.0 - COLD_BLOCK_POCKET_DEPTH / 2.0);
    let plate_registration = centered_cube(
        "media_sampling_fraction_collector_96_well_plate_registration",
        PLATE_X + 0.8,
        PLATE_Y + 0.8,
        PLATE_POCKET_DEPTH + 1.0,
    )
    .translate(0.0, 8.0, COLD_NEST_Z / 2.0 - PLATE_POCKET_DEPTH / 2.0);
    let pull_handle = centered_cube(
        "media_sampling_fraction_collector_pull_handle_cut",
        96.0,
        16.0,
        COLD_NEST_Z + 2.0,
    )
    .translate(0.0, -(COLD_NEST_Y / 2.0 - 8.0), 0.0);
    let barcode_window = centered_cube(
        "media_sampling_fraction_collector_barcode_reader_window",
        BARCODE_LAND_X,
        8.0,
        16.0,
    )
    .translate(0.0, -(COLD_NEST_Y / 2.0 + 1.0), COLD_NEST_Z / 2.0 - 15.0);

    let well_index_dimples = fraction_well_index_dimples();
    let fiducial_holes = fraction_collector_fiducial_holes();
    let thermistor_bore = centered_cylinder(
        "media_sampling_fraction_collector_cold_block_thermistor_access",
        3.5 / 2.0,
        44.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(COLD_NEST_Y / 2.0 - 18.0), -5.0);

    body - cold_block_pocket
        - plate_registration
        - pull_handle
        - barcode_window
        - well_index_dimples
        - fiducial_holes
        - thermistor_bore
        + fraction_collector_retainer_clips()
        + barcode_and_a1_lands()
        + fraction_collector_needle_parking()
}

fn fraction_well_index_dimples() -> Part {
    let mut dimples = Part::empty("media_sampling_fraction_collector_well_index_dimples");
    for row in 0..FRACTION_ROWS {
        for col in 0..FRACTION_COLS {
            let x = well_x(col);
            let y = well_y(row) + 8.0;
            let dimple = centered_cylinder(
                format!("media_sampling_fraction_well_index_r{row}_c{col}"),
                1.2,
                1.5,
                14,
            )
            .translate(x, y, COLD_NEST_Z / 2.0 - 0.5);
            dimples = dimples + dimple;
        }
    }
    dimples
}

fn fraction_collector_fiducial_holes() -> Part {
    let mut fiducials = Part::empty("media_sampling_fraction_collector_position_fiducials");
    for (i, (x, y, d)) in [
        (
            -(COLD_BLOCK_POCKET_X / 2.0 - 12.0),
            COLD_BLOCK_POCKET_Y / 2.0 + 8.0 - 12.0,
            5.0,
        ),
        (
            COLD_BLOCK_POCKET_X / 2.0 - 12.0,
            COLD_BLOCK_POCKET_Y / 2.0 + 8.0 - 12.0,
            4.0,
        ),
        (
            -(COLD_BLOCK_POCKET_X / 2.0 - 12.0),
            -(COLD_BLOCK_POCKET_Y / 2.0 - 8.0) + 8.0,
            3.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("media_sampling_fraction_collector_fiducial_{i}"),
                *d / 2.0,
                COLD_NEST_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    fiducials
}

fn fraction_collector_retainer_clips() -> Part {
    let mut clips = Part::empty("media_sampling_fraction_collector_retainer_clips");
    for (i, (x, y, sx, sy)) in [
        (
            0.0,
            COLD_BLOCK_POCKET_Y / 2.0 + 8.0 + 8.0,
            COLD_BLOCK_POCKET_X,
            7.0,
        ),
        (
            0.0,
            -(COLD_BLOCK_POCKET_Y / 2.0 - 8.0) - 8.0,
            COLD_BLOCK_POCKET_X,
            7.0,
        ),
        (
            -(COLD_BLOCK_POCKET_X / 2.0 + 8.0),
            8.0,
            7.0,
            COLD_BLOCK_POCKET_Y,
        ),
        (
            COLD_BLOCK_POCKET_X / 2.0 + 8.0,
            8.0,
            7.0,
            COLD_BLOCK_POCKET_Y,
        ),
    ]
    .iter()
    .enumerate()
    {
        clips = clips
            + centered_cube(
                format!("media_sampling_fraction_collector_retainer_clip_{i}"),
                *sx,
                *sy,
                12.0,
            )
            .translate(*x, *y, COLD_NEST_Z / 2.0 + 6.0);
    }
    clips
}

fn barcode_and_a1_lands() -> Part {
    let barcode_land = centered_cube(
        "media_sampling_fraction_collector_barcode_label_land",
        BARCODE_LAND_X,
        BARCODE_LAND_Y,
        2.5,
    )
    .translate(0.0, -(COLD_NEST_Y / 2.0 + 4.0), COLD_NEST_Z / 2.0 - 4.0);
    let a1_land = centered_cube(
        "media_sampling_fraction_collector_a1_position_land",
        22.0,
        14.0,
        3.0,
    )
    .translate(
        well_x(0) - 12.0,
        well_y(0) + 8.0 + 10.0,
        COLD_NEST_Z / 2.0 + 1.5,
    );
    let notch = centered_cylinder(
        "media_sampling_fraction_collector_asymmetric_a1_notch",
        3.2,
        4.0,
        20,
    )
    .translate(
        well_x(0) - 12.0,
        well_y(0) + 8.0 + 10.0,
        COLD_NEST_Z / 2.0 + 1.5,
    );

    barcode_land + (a1_land - notch)
}

fn fraction_collector_needle_parking() -> Part {
    let wash_cup = centered_cylinder(
        "media_sampling_fraction_collector_needle_wash_cup",
        12.0,
        18.0,
        32,
    )
    .translate(
        COLD_NEST_X / 2.0 - 28.0,
        -(COLD_NEST_Y / 2.0 - 30.0),
        COLD_NEST_Z / 2.0 + 9.0,
    );
    let wash_cup_bore = centered_cylinder(
        "media_sampling_fraction_collector_needle_wash_cup_bore",
        8.0,
        20.0,
        32,
    )
    .translate(
        COLD_NEST_X / 2.0 - 28.0,
        -(COLD_NEST_Y / 2.0 - 30.0),
        COLD_NEST_Z / 2.0 + 9.0,
    );
    let needle_comb = centered_cube(
        "media_sampling_fraction_collector_sample_tube_comb",
        80.0,
        9.0,
        12.0,
    )
    .translate(
        -(COLD_NEST_X / 2.0 - 54.0),
        -(COLD_NEST_Y / 2.0 - 30.0),
        COLD_NEST_Z / 2.0 + 6.0,
    );

    (wash_cup - wash_cup_bore) + needle_comb
}

fn analyzer_dock() -> Part {
    let deck = centered_cube(
        "media_sampling_analyzer_dock_deck",
        ANALYZER_DOCK_X,
        ANALYZER_DOCK_Y,
        14.0,
    )
    .translate(0.0, 0.0, -(ANALYZER_DOCK_Z / 2.0 - 7.0));
    let left_rail = centered_cube(
        "media_sampling_analyzer_dock_left_rail",
        14.0,
        ANALYZER_DOCK_Y - 20.0,
        36.0,
    )
    .translate(
        -(ANALYZER_DOCK_X / 2.0 - 16.0),
        -4.0,
        -(ANALYZER_DOCK_Z / 2.0 - 32.0),
    );
    let right_rail = centered_cube(
        "media_sampling_analyzer_dock_right_rail",
        14.0,
        ANALYZER_DOCK_Y - 20.0,
        36.0,
    )
    .translate(
        ANALYZER_DOCK_X / 2.0 - 16.0,
        -4.0,
        -(ANALYZER_DOCK_Z / 2.0 - 32.0),
    );
    let rear_stop = centered_cube(
        "media_sampling_analyzer_dock_rear_stop",
        ANALYZER_DOCK_X - 30.0,
        16.0,
        ANALYZER_DOCK_Z - 18.0,
    )
    .translate(0.0, ANALYZER_DOCK_Y / 2.0 - 10.0, 7.0);

    let envelope = purchased_analyzer_envelope();
    let fluid_connector_block = analyzer_fluid_connector_block().translate(
        0.0,
        -(ANALYZER_DOCK_Y / 2.0 - 18.0),
        -(ANALYZER_DOCK_Z / 2.0 - 38.0),
    );
    let cable_bulkhead = centered_cube(
        "media_sampling_analyzer_dock_cable_bulkhead",
        78.0,
        12.0,
        32.0,
    )
    .translate(0.0, ANALYZER_DOCK_Y / 2.0 + 2.0, -14.0);

    deck + left_rail + right_rail + rear_stop + envelope + fluid_connector_block + cable_bulkhead
}

fn purchased_analyzer_envelope() -> Part {
    let body = centered_cube(
        "media_sampling_glucose_lactate_analyzer_purchased_envelope",
        ANALYZER_ENV_X,
        ANALYZER_ENV_Y,
        ANALYZER_ENV_Z,
    );
    let screen_recess = centered_cube("media_sampling_analyzer_screen_recess", 108.0, 8.0, 36.0)
        .translate(0.0, -(ANALYZER_ENV_Y / 2.0 + 1.0), 20.0);
    let sample_door_clearance = centered_cube(
        "media_sampling_analyzer_sample_drawer_clearance",
        90.0,
        10.0,
        28.0,
    )
    .translate(0.0, -(ANALYZER_ENV_Y / 2.0 + 1.0), -32.0);
    let cable_clearance = centered_cube(
        "media_sampling_analyzer_rear_cable_clearance",
        92.0,
        10.0,
        26.0,
    )
    .translate(0.0, ANALYZER_ENV_Y / 2.0 + 1.0, -38.0);

    body - screen_recess - sample_door_clearance - cable_clearance
}

fn analyzer_fluid_connector_block() -> Part {
    let block = centered_cube(
        "media_sampling_analyzer_fluid_connector_block",
        126.0,
        18.0,
        34.0,
    );
    let mut ports = Part::empty("media_sampling_analyzer_fluid_connector_ports");
    for (i, x) in [-42.0, 0.0, 42.0].iter().enumerate() {
        ports = ports
            + centered_cylinder(
                format!("media_sampling_analyzer_connector_port_{i}"),
                6.2 / 2.0,
                22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0);
    }
    block - ports
}

fn sterile_bulkhead_panel() -> Part {
    let panel = centered_cube(
        "media_sampling_sterile_bulkhead_panel_plate",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    );

    let mut cuts = Part::empty("media_sampling_sterile_bulkhead_panel_cuts");
    let mut fittings = Part::empty("media_sampling_sterile_bulkhead_fitting_lands");
    for lane in 0..LANES {
        let x = lane_x(lane);
        for (row, z, d, label) in [
            (0, 82.0, 10.4, "chip_sample_inlet"),
            (1, 34.0, 10.4, "chip_sample_return"),
            (2, -14.0, 8.0, "sample_loop_out"),
            (3, -62.0, 8.0, "flush_or_waste"),
        ] {
            let hole = centered_cylinder(
                format!("media_sampling_lane_{lane}_{label}_bulkhead_cutout"),
                d / 2.0,
                PANEL_Y + 6.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, z);
            cuts = cuts + hole;

            let flange = centered_cylinder(
                format!("media_sampling_lane_{lane}_{label}_bulkhead_flange"),
                d / 2.0 + 4.0,
                4.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(PANEL_Y / 2.0 + 2.0), z);
            let flange_hole = centered_cylinder(
                format!("media_sampling_lane_{lane}_{label}_bulkhead_flange_hole"),
                d / 2.0,
                6.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(PANEL_Y / 2.0 + 2.0), z);
            fittings = fittings + (flange - flange_hole);

            let index_land = centered_cube(
                format!("media_sampling_lane_{lane}_bulkhead_label_land_{row}"),
                34.0,
                4.0,
                8.0,
            )
            .translate(x, -(PANEL_Y / 2.0 + 3.0), z - 20.0);
            fittings = fittings + index_land;
        }
    }

    for (i, (x, z, d, name)) in [
        (
            -(PANEL_X / 2.0 - 54.0),
            -108.0,
            16.0,
            "sterile_flush_source",
        ),
        (PANEL_X / 2.0 - 54.0, -108.0, 16.0, "waste_out"),
        (-(PANEL_X / 2.0 - 54.0), 118.0, 14.0, "analyzer_sample_send"),
        (PANEL_X / 2.0 - 54.0, 118.0, 14.0, "analyzer_return"),
    ]
    .iter()
    .enumerate()
    {
        let service_hole = centered_cylinder(
            format!("media_sampling_bulkhead_panel_{name}_cutout_{i}"),
            *d / 2.0,
            PANEL_Y + 6.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, *z);
        cuts = cuts + service_hole;

        let service_flange = centered_cylinder(
            format!("media_sampling_bulkhead_panel_{name}_flange_{i}"),
            *d / 2.0 + 5.0,
            5.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -(PANEL_Y / 2.0 + 2.5), *z);
        let flange_hole = centered_cylinder(
            format!("media_sampling_bulkhead_panel_{name}_flange_hole_{i}"),
            *d / 2.0,
            7.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -(PANEL_Y / 2.0 + 2.5), *z);
        fittings = fittings + (service_flange - flange_hole);
    }

    let cable_slot = centered_cube(
        "media_sampling_bulkhead_panel_barcode_sensor_cable_slot",
        88.0,
        PANEL_Y + 6.0,
        16.0,
    )
    .translate(0.0, 0.0, -126.0);
    cuts = cuts + cable_slot + panel_mount_screw_cuts();

    panel - cuts + fittings + bulkhead_panel_stiffeners()
}

fn panel_mount_screw_cuts() -> Part {
    let mut screws = Part::empty("media_sampling_bulkhead_panel_mount_screws");
    for (i, (x, z)) in [
        (-(PANEL_X / 2.0 - 24.0), -(PANEL_Z / 2.0 - 24.0)),
        (PANEL_X / 2.0 - 24.0, -(PANEL_Z / 2.0 - 24.0)),
        (-(PANEL_X / 2.0 - 24.0), PANEL_Z / 2.0 - 24.0),
        (PANEL_X / 2.0 - 24.0, PANEL_Z / 2.0 - 24.0),
        (0.0, -(PANEL_Z / 2.0 - 24.0)),
        (0.0, PANEL_Z / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        screws = screws
            + centered_cylinder(
                format!("media_sampling_bulkhead_panel_m5_mount_{i}"),
                5.4 / 2.0,
                PANEL_Y + 6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, *z);
    }
    screws
}

fn bulkhead_panel_stiffeners() -> Part {
    let top = centered_cube(
        "media_sampling_bulkhead_panel_top_stiffener",
        PANEL_X - 54.0,
        12.0,
        16.0,
    )
    .translate(0.0, PANEL_Y / 2.0 + 6.0, PANEL_Z / 2.0 - 20.0);
    let lower = centered_cube(
        "media_sampling_bulkhead_panel_lower_stiffener",
        PANEL_X - 54.0,
        12.0,
        16.0,
    )
    .translate(0.0, PANEL_Y / 2.0 + 6.0, -(PANEL_Z / 2.0 - 20.0));
    let mut ribs = Part::empty("media_sampling_bulkhead_panel_vertical_ribs");
    for (i, x) in [-280.0, -140.0, 0.0, 140.0, 280.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("media_sampling_bulkhead_panel_vertical_rib_{i}"),
                10.0,
                12.0,
                PANEL_Z - 70.0,
            )
            .translate(*x, PANEL_Y / 2.0 + 6.0, 0.0);
    }
    top + lower + ribs
}

fn bubble_dead_volume_control_bank() -> Part {
    let body = centered_cube(
        "media_sampling_bubble_dead_volume_control_body",
        BUBBLE_BANK_X,
        BUBBLE_BANK_Y,
        BUBBLE_BANK_Z,
    );

    let common_sample_bore = centered_cylinder(
        "media_sampling_bubble_bank_common_sample_bore",
        FLUID_BORE_D / 2.0,
        BUBBLE_BANK_X - 46.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -12.0, -18.0);
    let common_purge_bore = centered_cylinder(
        "media_sampling_bubble_bank_common_low_point_purge_bore",
        DEAD_VOLUME_PURGE_D / 2.0,
        BUBBLE_BANK_X - 46.0,
        22,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 14.0, -28.0);

    let mut cuts = common_sample_bore + common_purge_bore;
    let mut collars = Part::empty("media_sampling_bubble_bank_collars_and_purge_labels");
    for lane in 0..LANES {
        let x = lane_x(lane);
        let bubble_chamber = centered_cylinder(
            format!("media_sampling_lane_{lane}_bubble_sight_chamber"),
            BUBBLE_CHAMBER_D / 2.0,
            BUBBLE_BANK_Z + 4.0,
            36,
        )
        .translate(x, -12.0, 0.0);
        let top_vent = centered_cylinder(
            format!("media_sampling_lane_{lane}_bubble_top_vent_bore"),
            3.2 / 2.0,
            BUBBLE_BANK_Z + 6.0,
            18,
        )
        .translate(x + 9.0, -12.0, 0.0);
        let purge_drop = centered_cylinder(
            format!("media_sampling_lane_{lane}_dead_volume_purge_drop"),
            DEAD_VOLUME_PURGE_D / 2.0,
            BUBBLE_BANK_Z + 4.0,
            18,
        )
        .translate(x, 14.0, 0.0);
        let optical_window = centered_cube(
            format!("media_sampling_lane_{lane}_bubble_optical_window"),
            20.0,
            BUBBLE_BANK_Y + 6.0,
            24.0,
        )
        .translate(x, -12.0, 10.0);
        cuts = cuts + bubble_chamber + top_vent + purge_drop + optical_window;

        let collar = centered_cylinder(
            format!("media_sampling_lane_{lane}_bubble_chamber_collar"),
            BUBBLE_CHAMBER_D / 2.0 + 4.0,
            5.0,
            36,
        )
        .translate(x, -12.0, BUBBLE_BANK_Z / 2.0 + 2.5);
        let collar_hole = centered_cylinder(
            format!("media_sampling_lane_{lane}_bubble_chamber_collar_hole"),
            BUBBLE_CHAMBER_D / 2.0,
            7.0,
            36,
        )
        .translate(x, -12.0, BUBBLE_BANK_Z / 2.0 + 2.5);
        let purge_label = centered_cube(
            format!("media_sampling_lane_{lane}_dead_volume_purge_label_land"),
            32.0,
            4.0,
            8.0,
        )
        .translate(x, BUBBLE_BANK_Y / 2.0 + 2.0, -18.0);
        collars = collars + (collar - collar_hole) + purge_label;
    }

    body - cuts
        - selector_mount_holes(
            "media_sampling_bubble_bank",
            BUBBLE_BANK_X,
            BUBBLE_BANK_Y,
            BUBBLE_BANK_Z,
        )
        + collars
        + bubble_bank_service_tabs()
}

fn bubble_bank_service_tabs() -> Part {
    let purge_out = centered_cube(
        "media_sampling_bubble_bank_purge_outlet_tab",
        56.0,
        18.0,
        22.0,
    )
    .translate(BUBBLE_BANK_X / 2.0 - 28.0, BUBBLE_BANK_Y / 2.0 + 9.0, -28.0);
    let sample_in = centered_cube(
        "media_sampling_bubble_bank_sample_inlet_tab",
        56.0,
        18.0,
        22.0,
    )
    .translate(
        -(BUBBLE_BANK_X / 2.0 - 28.0),
        -(BUBBLE_BANK_Y / 2.0 + 9.0),
        -18.0,
    );
    purge_out + sample_in
}

fn service_access_cover() -> Part {
    let frame = centered_cube(
        "media_sampling_service_access_cover_frame",
        SERVICE_COVER_X,
        SERVICE_COVER_Y,
        SERVICE_COVER_Z,
    );
    let center_window = centered_cube(
        "media_sampling_service_access_cover_center_window",
        SERVICE_COVER_X - 128.0,
        SERVICE_COVER_Y + 4.0,
        SERVICE_COVER_Z - 28.0,
    )
    .translate(0.0, 0.0, 0.0);
    let collector_window = centered_cube(
        "media_sampling_service_access_cover_collector_window",
        168.0,
        SERVICE_COVER_Y + 6.0,
        42.0,
    )
    .translate(COLD_NEST_X_POS, 0.0, -16.0);
    let analyzer_window = centered_cube(
        "media_sampling_service_access_cover_analyzer_sample_window",
        144.0,
        SERVICE_COVER_Y + 6.0,
        42.0,
    )
    .translate(ANALYZER_X_POS, 0.0, -16.0);

    let mut latch_holes = Part::empty("media_sampling_service_access_cover_latch_holes");
    for (i, x) in [
        -(SERVICE_COVER_X / 2.0 - 38.0),
        -120.0,
        120.0,
        SERVICE_COVER_X / 2.0 - 38.0,
    ]
    .iter()
    .enumerate()
    {
        latch_holes = latch_holes
            + centered_cylinder(
                format!("media_sampling_service_cover_quarter_turn_latch_{i}"),
                8.0 / 2.0,
                SERVICE_COVER_Y + 6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -(SERVICE_COVER_Z / 2.0 - 16.0));
    }

    frame - center_window - collector_window - analyzer_window - latch_holes
        + service_cover_hinges()
        + service_cover_handle()
}

fn service_cover_hinges() -> Part {
    let mut hinges = Part::empty("media_sampling_service_access_cover_hinges");
    for (i, x) in [-250.0, 0.0, 250.0].iter().enumerate() {
        let leaf = centered_cube(
            format!("media_sampling_service_cover_hinge_leaf_{i}"),
            64.0,
            10.0,
            16.0,
        )
        .translate(
            *x,
            SERVICE_COVER_Y / 2.0 + 5.0,
            SERVICE_COVER_Z / 2.0 - 12.0,
        );
        let pin = centered_cylinder(
            format!("media_sampling_service_cover_hinge_pin_{i}"),
            4.0,
            70.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            *x,
            SERVICE_COVER_Y / 2.0 + 12.0,
            SERVICE_COVER_Z / 2.0 - 12.0,
        );
        hinges = hinges + leaf + pin;
    }
    hinges
}

fn service_cover_handle() -> Part {
    let left_post = centered_cube(
        "media_sampling_service_cover_handle_left_post",
        12.0,
        14.0,
        36.0,
    )
    .translate(-54.0, -(SERVICE_COVER_Y / 2.0 + 7.0), 2.0);
    let right_post = centered_cube(
        "media_sampling_service_cover_handle_right_post",
        12.0,
        14.0,
        36.0,
    )
    .translate(54.0, -(SERVICE_COVER_Y / 2.0 + 7.0), 2.0);
    let grip = centered_cylinder("media_sampling_service_cover_handle_grip", 7.0, 108.0, 28)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, -(SERVICE_COVER_Y / 2.0 + 16.0), 20.0);

    left_post + right_post + grip
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 44.0), -(BASE_Y / 2.0 - 42.0)),
        (BASE_X / 2.0 - 44.0, -(BASE_Y / 2.0 - 42.0)),
        (-(BASE_X / 2.0 - 44.0), BASE_Y / 2.0 - 42.0),
        (BASE_X / 2.0 - 44.0, BASE_Y / 2.0 - 42.0),
        (0.0, -(BASE_Y / 2.0 - 42.0)),
        (0.0, BASE_Y / 2.0 - 42.0),
        (-(BASE_X / 2.0 - 44.0), 0.0),
        (BASE_X / 2.0 - 44.0, 0.0),
    ]
}

fn lane_x(lane: usize) -> f64 {
    -((LANES as f64 - 1.0) * LANE_PITCH_X) / 2.0 + lane as f64 * LANE_PITCH_X
}

fn well_x(col: usize) -> f64 {
    let plate_center_x = PLATE_X / 2.0;
    let first_col_x = WELL96_A1_X - plate_center_x;
    first_col_x + col as f64 * WELL96_PITCH
}

fn well_y(row: usize) -> f64 {
    let plate_center_y = PLATE_Y / 2.0;
    let first_row_y = plate_center_y - WELL96_A1_Y;
    first_row_y - row as f64 * WELL96_PITCH
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 9);
        for path in OUTPUTS {
            assert!(path.starts_with("output/media_sampling_analyzer_interface_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn lane_selector_array_fits_inside_manifold_and_panel() {
        assert_eq!(LANES, 8);
        assert_eq!(SELECTOR_STATES, 4);
        assert_eq!(lane_x(0), -lane_x(LANES - 1));
        assert!(lane_x(0).abs() < SELECTOR_BLOCK_X / 2.0 - 50.0);
        assert!(lane_x(LANES - 1).abs() < PANEL_X / 2.0 - 70.0);
        assert!(LANE_PITCH_X > SELECTOR_ROTOR_D + SAMPLE_LOOP_LAND_X);
    }

    #[test]
    fn fraction_collector_registers_standard_96_well_plate() {
        assert_eq!(FRACTION_WELLS, 96);
        assert!(COLD_BLOCK_POCKET_X > PLATE_X + 20.0);
        assert!(COLD_BLOCK_POCKET_Y > PLATE_Y + 20.0);
        assert!(well_x(0) > -PLATE_X / 2.0);
        assert!(well_x(FRACTION_COLS - 1) < PLATE_X / 2.0);
        assert!(well_y(0) < PLATE_Y / 2.0);
        assert!(well_y(FRACTION_ROWS - 1) > -PLATE_Y / 2.0);
    }

    #[test]
    fn analyzer_and_collector_footprints_stay_on_base() {
        assert!(ANALYZER_X_POS + ANALYZER_DOCK_X / 2.0 < BASE_X / 2.0 - 36.0);
        assert!(ANALYZER_X_POS - ANALYZER_DOCK_X / 2.0 > -BASE_X / 2.0 + 36.0);
        assert!(COLD_NEST_X_POS - COLD_NEST_X / 2.0 > -BASE_X / 2.0 + 36.0);
        assert!(COLD_NEST_Y_POS - COLD_NEST_Y / 2.0 > -BASE_Y / 2.0 + 16.0);
        assert!(ANALYZER_ENV_X < ANALYZER_DOCK_X);
        assert!(ANALYZER_ENV_Y < ANALYZER_DOCK_Y);
    }

    #[test]
    fn bubble_and_waste_controls_cover_every_lane() {
        assert!(BUBBLE_BANK_X > (LANES as f64 - 1.0) * LANE_PITCH_X + 80.0);
        assert!(FLUSH_BLOCK_X > (LANES as f64 - 1.0) * LANE_PITCH_X + 64.0);
        assert!(WASTE_BORE_D > FLUID_BORE_D);
        assert!(DEAD_VOLUME_PURGE_D < FLUID_BORE_D);
    }
}
