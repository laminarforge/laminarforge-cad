use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed TEER/impedance electrode fouling and cleaning recovery validation station.
//
// This generator packages a closed-system mechanical validation module used
// before barrier-tissue TEER/impedance interpretation. It groups electrode
// cartridge nests, fouling coupon progression, cleaning and rinse path
// witnesses, reference phantom pockets, wet/dry storage, time tokens, bubble
// and dead-volume windows, custody surfaces, and disposition gates into one
// auditable station.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_teer_electrode_fouling_cleaning_recovery_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_base_validation_tray.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_electrode_cartridge_nests.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_fouling_coupon_ladder.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_cleaning_rinse_path_witness.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_impedance_phantom_pockets.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_wet_dry_storage_docks.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_timestamp_token_rail.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_bubble_dead_volume_windows.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_barcode_coa_custody_lands.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_release_hold_reject_gates.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_camera_evidence_bridge.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_robot_service_keepouts.stl",
    "output/closed_teer_electrode_fouling_cleaning_recovery_station_assembly.stl",
];

const DESIGN_SCOPE: &str = "mechanical validation packaging only; not a biological acceptance criterion, cleaning SOP, sterile-process claim, or barrier-tissue interpretation rule";

const REQUIRED_FEATURES: [&str; 24] = [
    "closed_system_validation_tray",
    "electrode_cartridge_nests",
    "electrode_contact_wipe_lands",
    "fouling_coupon_ladder",
    "fouling_gradient_steps",
    "cleaning_rinse_path_witness",
    "rinse_recovery_witness_wells",
    "impedance_phantom_pockets",
    "phantom_terminal_lands",
    "wet_storage_docks",
    "dry_storage_docks",
    "timestamp_token_rail",
    "bubble_dead_volume_windows",
    "bubble_ladder_ticks",
    "dead_volume_trap_pockets",
    "barcode_custody_lands",
    "coa_custody_lands",
    "tamper_seal_tabs",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "robot_keepouts",
    "service_keepouts",
];

const DECK_X: f64 = 1400.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 52.0;
const SOCKET_DEPTH: f64 = 4.5;
const DATUM_BOSSES: usize = 10;
const DRAIN_D: f64 = 16.0;

const ELECTRODE_X: f64 = 430.0;
const ELECTRODE_Y: f64 = 244.0;
const ELECTRODE_Z: f64 = 46.0;
const ELECTRODE_POS: (f64, f64) = (-420.0, 300.0);
const ELECTRODE_ROWS: usize = 2;
const ELECTRODE_COLS: usize = 4;
const ELECTRODE_NESTS: usize = ELECTRODE_ROWS * ELECTRODE_COLS;
const ELECTRODE_PITCH_X: f64 = 92.0;
const ELECTRODE_PITCH_Y: f64 = 84.0;
const ELECTRODE_POCKET_X: f64 = 66.0;
const ELECTRODE_POCKET_Y: f64 = 46.0;
const ELECTRODE_POCKET_Z: f64 = 28.0;
const ELECTRODE_CONTACT_WIPE_LANDS: usize = ELECTRODE_NESTS;

const CLEANING_X: f64 = 570.0;
const CLEANING_Y: f64 = 196.0;
const CLEANING_Z: f64 = 38.0;
const CLEANING_POS: (f64, f64) = (270.0, 300.0);
const CLEANING_LANES: usize = 6;
const CLEANING_LANE_PITCH_Y: f64 = 24.0;
const CLEANING_CHANNEL_W: f64 = 8.0;
const RINSE_STAGES: usize = 4;
const RINSE_WELLS: usize = CLEANING_LANES * RINSE_STAGES;

const FOULING_X: f64 = 330.0;
const FOULING_Y: f64 = 210.0;
const FOULING_Z: f64 = 36.0;
const FOULING_POS: (f64, f64) = (-500.0, 55.0);
const FOULING_LEVELS: usize = 7;
const COUPONS_PER_LEVEL: usize = 2;
const FOULING_COUPONS: usize = FOULING_LEVELS * COUPONS_PER_LEVEL;
const FOULING_LEVEL_PITCH_Y: f64 = 26.0;
const FOULING_COUPON_PITCH_X: f64 = 92.0;
const FOULING_COUPON_X: f64 = 58.0;
const FOULING_COUPON_Y: f64 = 17.0;

const PHANTOM_X: f64 = 350.0;
const PHANTOM_Y: f64 = 210.0;
const PHANTOM_Z: f64 = 44.0;
const PHANTOM_POS: (f64, f64) = (-100.0, 55.0);
const PHANTOM_ROWS: usize = 2;
const PHANTOM_COLS: usize = 3;
const PHANTOM_POCKETS: usize = PHANTOM_ROWS * PHANTOM_COLS;
const PHANTOM_PITCH_X: f64 = 94.0;
const PHANTOM_PITCH_Y: f64 = 78.0;
const PHANTOM_POCKET_X: f64 = 62.0;
const PHANTOM_POCKET_Y: f64 = 40.0;
const PHANTOM_TERMINAL_PAIRS_PER_POCKET: usize = 4;
const PHANTOM_TERMINAL_LANDS: usize = PHANTOM_POCKETS * PHANTOM_TERMINAL_PAIRS_PER_POCKET * 2;

const STORAGE_X: f64 = 310.0;
const STORAGE_Y: f64 = 210.0;
const STORAGE_Z: f64 = 34.0;
const STORAGE_POS: (f64, f64) = (250.0, 55.0);
const STORAGE_LANES: usize = 2;
const STORAGE_DOCKS_PER_LANE: usize = 6;
const WET_DOCKS: usize = STORAGE_DOCKS_PER_LANE;
const DRY_DOCKS: usize = STORAGE_DOCKS_PER_LANE;
const STORAGE_DOCK_PITCH_X: f64 = 42.0;
const STORAGE_LANE_PITCH_Y: f64 = 74.0;
const STORAGE_DOCK_D: f64 = 25.0;
const DRY_LANE_INDEX: usize = 0;
const WET_LANE_INDEX: usize = 1;

const CUSTODY_X: f64 = 220.0;
const CUSTODY_Y: f64 = 200.0;
const CUSTODY_Z: f64 = 18.0;
const CUSTODY_POS: (f64, f64) = (535.0, 55.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_TABS: usize = 6;
const CUSTODY_FIDUCIALS: usize = 4;

const TOKEN_X: f64 = 360.0;
const TOKEN_Y: f64 = 120.0;
const TOKEN_Z: f64 = 32.0;
const TOKEN_POS: (f64, f64) = (-470.0, -285.0);
const TIMESTAMP_TOKENS: usize = 10;
const TOKEN_PITCH_X: f64 = 32.0;
const TOKEN_D: f64 = 22.0;

const WINDOW_X: f64 = 430.0;
const WINDOW_Y: f64 = 170.0;
const WINDOW_Z: f64 = 30.0;
const WINDOW_POS: (f64, f64) = (-40.0, -285.0);
const WINDOW_ROWS: usize = 2;
const WINDOW_COLS: usize = 3;
const BUBBLE_WINDOWS: usize = WINDOW_ROWS * WINDOW_COLS;
const WINDOW_PITCH_X: f64 = 112.0;
const WINDOW_PITCH_Y: f64 = 72.0;
const WINDOW_APERTURE_X: f64 = 72.0;
const WINDOW_APERTURE_Y: f64 = 32.0;
const BUBBLE_TICKS_PER_WINDOW: usize = 5;
const DEAD_VOLUME_TRAP_POCKETS: usize = BUBBLE_WINDOWS;

const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 170.0;
const GATE_Z: f64 = 38.0;
const GATE_POS: (f64, f64) = (420.0, -285.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 4;
const GATE_PITCH_X: f64 = 124.0;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_X: f64 = 1180.0;
const CAMERA_Y: f64 = 520.0;
const CAMERA_CLEARANCE_Z: f64 = 168.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 48.0;
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;

const FRONT_ROBOT_SWEEP: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const SIDE_SERVICE_CLEARANCE: f64 = 180.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 310.0;
const KEEP_OUT_ZONE_COUNT: usize = 7;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(base_validation_tray(), OUTPUTS[0]);
    write_part(electrode_cartridge_nests(), OUTPUTS[1]);
    write_part(fouling_coupon_ladder(), OUTPUTS[2]);
    write_part(cleaning_rinse_path_witness(), OUTPUTS[3]);
    write_part(impedance_phantom_pockets(), OUTPUTS[4]);
    write_part(wet_dry_storage_docks(), OUTPUTS[5]);
    write_part(timestamp_token_rail(), OUTPUTS[6]);
    write_part(bubble_dead_volume_windows(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed TEER electrode fouling/cleaning recovery station: {:.0}mm x {:.0}mm closed validation tray.",
        DECK_X, DECK_Y
    );
    println!(
        "Electrode path: {ELECTRODE_NESTS} cartridge nests, {FOULING_COUPONS} fouling coupons across {FOULING_LEVELS} fouling levels, {RINSE_WELLS} cleaning/rinse witness wells."
    );
    println!(
        "Reference path: {PHANTOM_POCKETS} impedance phantom pockets with {PHANTOM_TERMINAL_LANDS} terminal lands, {WET_DOCKS} wet docks, {DRY_DOCKS} dry docks, and {TIMESTAMP_TOKENS} timestamp tokens."
    );
    println!(
        "Evidence and release: {BUBBLE_WINDOWS} bubble/dead-volume windows, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {TAMPER_SEAL_TABS} tamper tabs, and release/hold/reject gates."
    );
    println!("Scope: {DESIGN_SCOPE}.");
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_validation_tray()
        + electrode_cartridge_nests()
        + fouling_coupon_ladder()
        + cleaning_rinse_path_witness()
        + impedance_phantom_pockets()
        + wet_dry_storage_docks()
        + timestamp_token_rail()
        + bubble_dead_volume_windows()
        + barcode_coa_custody_lands()
        + release_hold_reject_gates()
        + camera_evidence_bridge()
        + robot_service_keepouts()
}

fn base_validation_tray() -> Part {
    let deck = centered_cube(
        "closed_teer_electrode_recovery_station_closed_system_validation_tray",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let wet_zone_recess = centered_cube(
        "closed_teer_electrode_recovery_station_wet_zone_capture_recess",
        DECK_X - 168.0,
        400.0,
        6.0,
    )
    .translate(-54.0, 244.0, DECK_Z / 2.0 - 3.0);
    let lower_witness_recess = centered_cube(
        "closed_teer_electrode_recovery_station_lower_witness_sump",
        DECK_X - 160.0,
        204.0,
        5.0,
    )
    .translate(0.0, -286.0, DECK_Z / 2.0 - 2.5);
    let front_drain = centered_cylinder(
        "closed_teer_electrode_recovery_station_front_drain_cut",
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 - 2.0, 0.0);

    deck - wet_zone_recess
        - lower_witness_recess
        - front_drain
        - module_socket_cuts()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + closed_zone_dividers()
        + module_landing_pads()
        + tray_datum_bosses()
        + spill_witness_ticks()
}

fn module_socket_cuts() -> Part {
    let mut sockets = Part::empty("closed_teer_electrode_recovery_station_module_socket_cuts");
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_{}_socket",
                    footprint.name
                ),
                footprint.x + 10.0,
                footprint.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_teer_electrode_recovery_station_mounting_slots");
    for (idx, (x, y)) in [
        (-(DECK_X / 2.0 - 60.0), -(DECK_Y / 2.0 - 54.0)),
        (DECK_X / 2.0 - 60.0, -(DECK_Y / 2.0 - 54.0)),
        (-(DECK_X / 2.0 - 60.0), DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 60.0, DECK_Y / 2.0 - 54.0),
        (0.0, DECK_Y / 2.0 - 54.0),
        (0.0, -(DECK_Y / 2.0 - 54.0)),
        (-(DECK_X / 2.0 - 60.0), 0.0),
        (DECK_X / 2.0 - 60.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_m6_clearance_{idx}"),
                6.6 / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_m6_slot_relief_{idx}"),
                26.0,
                7.0,
                DECK_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_teer_electrode_recovery_station_datum_pin_holes");
    for (idx, (x, y)) in [(-638.0, 408.0), (638.0, 408.0), (-638.0, -408.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_datum_pin_clearance_{idx}"),
                5.0 / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    centered_cube(
        "closed_teer_electrode_recovery_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            "closed_teer_electrode_recovery_station_rear_containment_rim",
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            "closed_teer_electrode_recovery_station_left_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            "closed_teer_electrode_recovery_station_right_containment_rim",
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn closed_zone_dividers() -> Part {
    let wet_to_dry = centered_cube(
        "closed_teer_electrode_recovery_station_wet_cleaning_to_dry_reference_divider",
        DECK_X - 160.0,
        12.0,
        34.0,
    )
    .translate(0.0, 178.0, DECK_Z / 2.0 + 17.0);
    let reference_to_evidence = centered_cube(
        "closed_teer_electrode_recovery_station_reference_to_evidence_divider",
        DECK_X - 180.0,
        10.0,
        30.0,
    )
    .translate(0.0, -132.0, DECK_Z / 2.0 + 15.0);
    let fouling_to_phantom = centered_cube(
        "closed_teer_electrode_recovery_station_fouling_to_phantom_divider",
        10.0,
        214.0,
        28.0,
    )
    .translate(-300.0, 55.0, DECK_Z / 2.0 + 14.0);
    let phantom_to_storage = centered_cube(
        "closed_teer_electrode_recovery_station_phantom_to_storage_divider",
        10.0,
        214.0,
        28.0,
    )
    .translate(76.0, 55.0, DECK_Z / 2.0 + 14.0);

    wet_to_dry + reference_to_evidence + fouling_to_phantom + phantom_to_storage
}

fn module_landing_pads() -> Part {
    let mut pads = Part::empty("closed_teer_electrode_recovery_station_module_landing_pads");
    for footprint in module_footprints() {
        pads = pads
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_{}_landing_pad",
                    footprint.name
                ),
                footprint.x + 12.0,
                footprint.y + 12.0,
                3.0,
            )
            .translate(footprint.center.0, footprint.center.1, DECK_Z / 2.0 + 1.5);
    }
    pads
}

fn tray_datum_bosses() -> Part {
    let mut bosses = Part::empty("closed_teer_electrode_recovery_station_tray_datum_bosses");
    for idx in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 86.0 + idx as f64 * ((DECK_X - 172.0) / 9.0);
        let y = if idx % 2 == 0 {
            DECK_Y / 2.0 - 82.0
        } else {
            -DECK_Y / 2.0 + 82.0
        };
        bosses = bosses
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_robot_datum_boss_{idx}"),
                8.0,
                6.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0 + 3.0);
    }
    bosses
}

fn spill_witness_ticks() -> Part {
    let mut ticks = Part::empty("closed_teer_electrode_recovery_station_spill_witness_ticks");
    for idx in 0..12 {
        let x = centered_index(idx, 12, 94.0);
        ticks = ticks
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_spill_tick_{idx}"),
                54.0,
                5.0,
                5.0,
            )
            .translate(x, -408.0, DECK_Z / 2.0 + 2.5);
    }
    ticks
}

fn electrode_cartridge_nests() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_electrode_cartridge_nest_plate",
        ELECTRODE_X,
        ELECTRODE_Y,
        ELECTRODE_Z,
    )
    .translate(ELECTRODE_POS.0, ELECTRODE_POS.1, top_z(ELECTRODE_Z));
    let rear_fence = centered_cube(
        "closed_teer_electrode_recovery_station_electrode_rear_reference_fence",
        ELECTRODE_X - 22.0,
        12.0,
        ELECTRODE_Z + 20.0,
    )
    .translate(
        ELECTRODE_POS.0,
        ELECTRODE_POS.1 + ELECTRODE_Y / 2.0 - 10.0,
        top_z(ELECTRODE_Z) + 10.0,
    );

    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_electrode_nest_cuts");
    let mut features =
        Part::empty("closed_teer_electrode_recovery_station_electrode_nest_features");
    for idx in 0..ELECTRODE_NESTS {
        let (x, y) = electrode_center(idx);
        let pocket = centered_cube(
            format!("closed_teer_electrode_recovery_station_electrode_pocket_{idx}"),
            ELECTRODE_POCKET_X,
            ELECTRODE_POCKET_Y,
            ELECTRODE_POCKET_Z,
        )
        .translate(
            x,
            y,
            DECK_Z / 2.0 + ELECTRODE_Z - ELECTRODE_POCKET_Z / 2.0 + 1.0,
        );
        let thumb_relief = centered_cylinder(
            format!("closed_teer_electrode_recovery_station_electrode_thumb_relief_{idx}"),
            12.0,
            ELECTRODE_Z + 3.0,
            28,
        )
        .translate(
            x - ELECTRODE_POCKET_X / 2.0 + 8.0,
            y - ELECTRODE_POCKET_Y / 2.0,
            top_z(ELECTRODE_Z),
        );
        let lead_exit = centered_cube(
            format!("closed_teer_electrode_recovery_station_electrode_lead_exit_{idx}"),
            20.0,
            24.0,
            ELECTRODE_Z + 2.0,
        )
        .translate(x + ELECTRODE_POCKET_X / 2.0, y, top_z(ELECTRODE_Z));

        cuts = cuts + pocket + thumb_relief + lead_exit;
        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_electrode_contact_wipe_land_{idx}"),
                48.0,
                7.0,
                5.0,
            )
            .translate(
                x,
                y + ELECTRODE_POCKET_Y / 2.0 + 11.0,
                cap_z(ELECTRODE_Z, 5.0),
            )
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_electrode_orientation_key_{idx}"),
                8.0,
                28.0,
                8.0,
            )
            .translate(
                x - ELECTRODE_POCKET_X / 2.0 - 7.0,
                y,
                cap_z(ELECTRODE_Z, 8.0),
            )
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_electrode_latch_land_{idx}"),
                38.0,
                8.0,
                7.0,
            )
            .translate(
                x,
                y - ELECTRODE_POCKET_Y / 2.0 - 11.0,
                cap_z(ELECTRODE_Z, 7.0),
            );
    }

    plate - cuts + rear_fence + features + electrode_datum_saddles()
}

fn electrode_datum_saddles() -> Part {
    let mut saddles = Part::empty("closed_teer_electrode_recovery_station_electrode_datum_saddles");
    for idx in 0..4 {
        saddles = saddles
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_electrode_datum_saddle_{idx}"),
                9.0,
                ELECTRODE_Y - 64.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                ELECTRODE_POS.0 - 144.0 + idx as f64 * 96.0,
                ELECTRODE_POS.1,
                DECK_Z / 2.0 + ELECTRODE_Z + 10.0,
            );
    }
    saddles
}

fn fouling_coupon_ladder() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_fouling_coupon_ladder_plate",
        FOULING_X,
        FOULING_Y,
        FOULING_Z,
    )
    .translate(FOULING_POS.0, FOULING_POS.1, top_z(FOULING_Z));

    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_fouling_coupon_recesses");
    let mut features = Part::empty("closed_teer_electrode_recovery_station_fouling_gradient_steps");
    for level in 0..FOULING_LEVELS {
        let y = FOULING_POS.1 + fouling_level_y(level);
        let step_h = 3.0 + level as f64 * 1.5;
        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_fouling_level_{level}_step"),
                FOULING_X - 42.0,
                8.0,
                step_h,
            )
            .translate(
                FOULING_POS.0,
                y - FOULING_COUPON_Y / 2.0 - 12.0,
                DECK_Z / 2.0 + FOULING_Z + step_h / 2.0,
            )
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_fouling_level_{level}_dose_land"),
                34.0,
                16.0,
                4.0,
            )
            .translate(
                FOULING_POS.0 - FOULING_X / 2.0 + 26.0,
                y,
                cap_z(FOULING_Z, 4.0),
            );

        for coupon in 0..COUPONS_PER_LEVEL {
            let x = FOULING_POS.0 + coupon_x(coupon, COUPONS_PER_LEVEL, FOULING_COUPON_PITCH_X);
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_fouling_level_{level}_coupon_{coupon}_recess"
                    ),
                    FOULING_COUPON_X,
                    FOULING_COUPON_Y,
                    FOULING_Z + 3.0,
                )
                .translate(x, y, top_z(FOULING_Z));
            features = features
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_fouling_level_{level}_coupon_{coupon}_front_stop"
                    ),
                    FOULING_COUPON_X,
                    4.0,
                    6.0,
                )
                .translate(x, y - FOULING_COUPON_Y / 2.0 - 5.0, cap_z(FOULING_Z, 6.0))
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_fouling_level_{level}_coupon_{coupon}_identity_tick"
                    ),
                    5.0,
                    FOULING_COUPON_Y + 8.0,
                    5.0,
                )
                .translate(
                    x + FOULING_COUPON_X / 2.0 + 7.0,
                    y,
                    cap_z(FOULING_Z, 5.0),
                );
        }
    }

    plate - cuts + features + fouling_clean_baseline_rail()
}

fn fouling_clean_baseline_rail() -> Part {
    let rail = centered_cube(
        "closed_teer_electrode_recovery_station_fouling_clean_baseline_reference_rail",
        14.0,
        FOULING_Y - 38.0,
        10.0,
    )
    .translate(
        FOULING_POS.0 + FOULING_X / 2.0 - 22.0,
        FOULING_POS.1,
        cap_z(FOULING_Z, 10.0),
    );
    let mut ticks = Part::empty("closed_teer_electrode_recovery_station_fouling_baseline_ticks");
    for level in 0..FOULING_LEVELS {
        ticks = ticks
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_fouling_baseline_tick_{level}"),
                22.0,
                3.0,
                5.0,
            )
            .translate(
                FOULING_POS.0 + FOULING_X / 2.0 - 22.0,
                FOULING_POS.1 + fouling_level_y(level),
                cap_z(FOULING_Z, 5.0),
            );
    }
    rail + ticks
}

fn cleaning_rinse_path_witness() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_cleaning_rinse_path_witness_plate",
        CLEANING_X,
        CLEANING_Y,
        CLEANING_Z,
    )
    .translate(CLEANING_POS.0, CLEANING_POS.1, top_z(CLEANING_Z));

    let mut channel_cuts =
        Part::empty("closed_teer_electrode_recovery_station_cleaning_path_channel_cuts");
    let mut features =
        Part::empty("closed_teer_electrode_recovery_station_cleaning_rinse_witness_features");
    for lane in 0..CLEANING_LANES {
        let y = CLEANING_POS.1 + cleaning_lane_y(lane);
        channel_cuts = channel_cuts
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_cleaning_lane_{lane}_witness_channel"
                ),
                CLEANING_X - 92.0,
                CLEANING_CHANNEL_W,
                CLEANING_Z + 3.0,
            )
            .translate(CLEANING_POS.0, y, top_z(CLEANING_Z));

        if lane + 1 < CLEANING_LANES {
            let turn_x = if lane % 2 == 0 {
                CLEANING_POS.0 + CLEANING_X / 2.0 - 56.0
            } else {
                CLEANING_POS.0 - CLEANING_X / 2.0 + 56.0
            };
            channel_cuts = channel_cuts
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_cleaning_lane_{lane}_serpentine_turn"
                    ),
                    CLEANING_CHANNEL_W,
                    CLEANING_LANE_PITCH_Y + CLEANING_CHANNEL_W,
                    CLEANING_Z + 3.0,
                )
                .translate(
                    turn_x,
                    CLEANING_POS.1
                        + (cleaning_lane_y(lane) + cleaning_lane_y(lane + 1)) / 2.0,
                    top_z(CLEANING_Z),
                );
        }

        for stage in 0..RINSE_STAGES {
            let x = CLEANING_POS.0 + rinse_stage_x(stage);
            features = features
                + centered_cylinder(
                    format!(
                        "closed_teer_electrode_recovery_station_lane_{lane}_rinse_recovery_witness_well_{stage}"
                    ),
                    8.0,
                    5.0,
                    28,
                )
                .translate(x, y + CLEANING_CHANNEL_W + 12.0, cap_z(CLEANING_Z, 5.0))
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_lane_{lane}_rinse_stage_{stage}_barcode_tick"
                    ),
                    4.0,
                    22.0,
                    4.0,
                )
                .translate(x, y - CLEANING_CHANNEL_W - 13.0, cap_z(CLEANING_Z, 4.0));
        }
    }

    plate - channel_cuts + features + cleaning_inlet_outlet_cups() + rinse_stage_dividers()
}

fn cleaning_inlet_outlet_cups() -> Part {
    let mut cups = Part::empty("closed_teer_electrode_recovery_station_cleaning_inlet_outlet_cups");
    for lane in 0..CLEANING_LANES {
        let y = CLEANING_POS.1 + cleaning_lane_y(lane);
        cups = cups
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_cleaning_lane_{lane}_dirty_inlet_cup"),
                12.0,
                6.0,
                32,
            )
            .translate(
                CLEANING_POS.0 - CLEANING_X / 2.0 + 30.0,
                y,
                cap_z(CLEANING_Z, 6.0),
            )
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_cleaning_lane_{lane}_clean_rinse_outlet_cup"),
                12.0,
                6.0,
                32,
            )
            .translate(
                CLEANING_POS.0 + CLEANING_X / 2.0 - 30.0,
                y,
                cap_z(CLEANING_Z, 6.0),
            );
    }
    cups
}

fn rinse_stage_dividers() -> Part {
    let mut dividers = Part::empty("closed_teer_electrode_recovery_station_rinse_stage_dividers");
    for stage in 1..RINSE_STAGES {
        dividers = dividers
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_rinse_stage_{stage}_divider"),
                5.0,
                CLEANING_Y - 42.0,
                9.0,
            )
            .translate(
                CLEANING_POS.0 + (rinse_stage_x(stage - 1) + rinse_stage_x(stage)) / 2.0,
                CLEANING_POS.1,
                cap_z(CLEANING_Z, 9.0),
            );
    }
    dividers
}

fn impedance_phantom_pockets() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_impedance_phantom_pocket_plate",
        PHANTOM_X,
        PHANTOM_Y,
        PHANTOM_Z,
    )
    .translate(PHANTOM_POS.0, PHANTOM_POS.1, top_z(PHANTOM_Z));

    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_impedance_phantom_cuts");
    let mut features =
        Part::empty("closed_teer_electrode_recovery_station_impedance_phantom_features");
    for idx in 0..PHANTOM_POCKETS {
        let (x, y) = phantom_center(idx);
        cuts = cuts
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_impedance_phantom_pocket_{idx}"),
                PHANTOM_POCKET_X,
                PHANTOM_POCKET_Y,
                PHANTOM_Z + 3.0,
            )
            .translate(x, y, top_z(PHANTOM_Z))
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_impedance_phantom_finger_relief_{idx}"
                ),
                16.0,
                PHANTOM_POCKET_Y + 14.0,
                PHANTOM_Z + 4.0,
            )
            .translate(x - PHANTOM_POCKET_X / 2.0 + 5.0, y, top_z(PHANTOM_Z));

        for pair in 0..PHANTOM_TERMINAL_PAIRS_PER_POCKET {
            let terminal_x = x - 22.0 + pair as f64 * 14.5;
            features = features
                + centered_cylinder(
                    format!(
                        "closed_teer_electrode_recovery_station_phantom_{idx}_terminal_pair_{pair}_low_land"
                    ),
                    3.6,
                    4.0,
                    18,
                )
                .translate(
                    terminal_x,
                    y + PHANTOM_POCKET_Y / 2.0 + 10.0,
                    cap_z(PHANTOM_Z, 4.0),
                )
                + centered_cylinder(
                    format!(
                        "closed_teer_electrode_recovery_station_phantom_{idx}_terminal_pair_{pair}_high_land"
                    ),
                    3.6,
                    4.0,
                    18,
                )
                .translate(
                    terminal_x,
                    y - PHANTOM_POCKET_Y / 2.0 - 10.0,
                    cap_z(PHANTOM_Z, 4.0),
                );
        }

        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_impedance_phantom_type_key_{idx}"),
                8.0 + (idx % PHANTOM_COLS) as f64 * 4.0,
                7.0,
                6.0,
            )
            .translate(x + PHANTOM_POCKET_X / 2.0 + 10.0, y, cap_z(PHANTOM_Z, 6.0));
    }

    plate - cuts + features + phantom_reference_bus_lands()
}

fn phantom_reference_bus_lands() -> Part {
    centered_cube(
        "closed_teer_electrode_recovery_station_impedance_phantom_reference_bus_low",
        PHANTOM_X - 34.0,
        6.0,
        6.0,
    )
    .translate(
        PHANTOM_POS.0,
        PHANTOM_POS.1 + PHANTOM_Y / 2.0 - 18.0,
        cap_z(PHANTOM_Z, 6.0),
    ) + centered_cube(
        "closed_teer_electrode_recovery_station_impedance_phantom_reference_bus_high",
        PHANTOM_X - 34.0,
        6.0,
        6.0,
    )
    .translate(
        PHANTOM_POS.0,
        PHANTOM_POS.1 - PHANTOM_Y / 2.0 + 18.0,
        cap_z(PHANTOM_Z, 6.0),
    )
}

fn wet_dry_storage_docks() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_wet_dry_storage_dock_plate",
        STORAGE_X,
        STORAGE_Y,
        STORAGE_Z,
    )
    .translate(STORAGE_POS.0, STORAGE_POS.1, top_z(STORAGE_Z));
    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_storage_dock_cuts");
    let mut features = Part::empty("closed_teer_electrode_recovery_station_storage_dock_features");
    for lane in 0..STORAGE_LANES {
        let lane_y = STORAGE_POS.1 + storage_lane_y(lane);
        features = features
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_{}_storage_lane_label_land",
                    storage_lane_name(lane)
                ),
                64.0,
                16.0,
                4.0,
            )
            .translate(
                STORAGE_POS.0 - STORAGE_X / 2.0 + 44.0,
                lane_y,
                cap_z(STORAGE_Z, 4.0),
            );

        for dock in 0..STORAGE_DOCKS_PER_LANE {
            let x = STORAGE_POS.0 + storage_dock_x(dock);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_teer_electrode_recovery_station_{}_storage_dock_{dock}",
                        storage_lane_name(lane)
                    ),
                    STORAGE_DOCK_D / 2.0,
                    STORAGE_Z + 3.0,
                    36,
                )
                .translate(x, lane_y, top_z(STORAGE_Z));
            features = features
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_{}_dock_capture_tick_{dock}",
                        storage_lane_name(lane)
                    ),
                    4.0,
                    STORAGE_DOCK_D + 12.0,
                    5.0,
                )
                .translate(
                    x + STORAGE_DOCK_D / 2.0 + 8.0,
                    lane_y,
                    cap_z(STORAGE_Z, 5.0),
                );
        }
    }

    plate - cuts + features + wet_storage_hydration_moat() + dry_storage_desiccant_rail()
}

fn wet_storage_hydration_moat() -> Part {
    centered_cube(
        "closed_teer_electrode_recovery_station_wet_storage_hydration_moat",
        STORAGE_X - 40.0,
        8.0,
        8.0,
    )
    .translate(
        STORAGE_POS.0,
        STORAGE_POS.1 + storage_lane_y(WET_LANE_INDEX) + 29.0,
        cap_z(STORAGE_Z, 8.0),
    )
}

fn dry_storage_desiccant_rail() -> Part {
    centered_cube(
        "closed_teer_electrode_recovery_station_dry_storage_desiccant_indicator_rail",
        STORAGE_X - 40.0,
        8.0,
        8.0,
    )
    .translate(
        STORAGE_POS.0,
        STORAGE_POS.1 + storage_lane_y(DRY_LANE_INDEX) - 29.0,
        cap_z(STORAGE_Z, 8.0),
    )
}

fn timestamp_token_rail() -> Part {
    let rail = centered_cube(
        "closed_teer_electrode_recovery_station_timestamp_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, top_z(TOKEN_Z));
    let sight_slot = centered_cube(
        "closed_teer_electrode_recovery_station_timestamp_sequence_sight_slot",
        TOKEN_X - 50.0,
        14.0,
        TOKEN_Z + 3.0,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1 - 21.0, top_z(TOKEN_Z));

    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_timestamp_token_cuts");
    let mut features =
        Part::empty("closed_teer_electrode_recovery_station_timestamp_token_features");
    for idx in 0..TIMESTAMP_TOKENS {
        let x = TOKEN_POS.0 + timestamp_token_x(idx);
        cuts = cuts
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_timestamp_token_socket_{idx}"),
                TOKEN_D / 2.0,
                TOKEN_Z + 4.0,
                36,
            )
            .translate(x, TOKEN_POS.1 + 18.0, top_z(TOKEN_Z));
        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_timestamp_elapsed_step_{idx}"),
                10.0,
                30.0,
                4.0 + idx as f64,
            )
            .translate(
                x,
                TOKEN_POS.1 - TOKEN_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + TOKEN_Z + 2.0 + idx as f64 / 2.0,
            )
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_timestamp_hash_land_{idx}"),
                18.0,
                5.0,
                4.0,
            )
            .translate(x, TOKEN_POS.1 + TOKEN_Y / 2.0 - 18.0, cap_z(TOKEN_Z, 4.0));
    }
    rail - sight_slot - cuts + features
}

fn bubble_dead_volume_windows() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_bubble_dead_volume_window_plate",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(WINDOW_POS.0, WINDOW_POS.1, top_z(WINDOW_Z));
    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_bubble_window_cuts");
    let mut features = Part::empty("closed_teer_electrode_recovery_station_bubble_window_features");
    for idx in 0..BUBBLE_WINDOWS {
        let (x, y) = window_center(idx);
        cuts = cuts
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_bubble_sight_window_cut_{idx}"),
                WINDOW_APERTURE_X,
                WINDOW_APERTURE_Y,
                WINDOW_Z + 3.0,
            )
            .translate(x, y, top_z(WINDOW_Z))
            + centered_cylinder(
                format!("closed_teer_electrode_recovery_station_dead_volume_trap_pocket_cut_{idx}"),
                8.0,
                WINDOW_Z + 3.0,
                28,
            )
            .translate(x + WINDOW_APERTURE_X / 2.0 + 14.0, y, top_z(WINDOW_Z));
        features = features
            + window_frame(idx, x, y)
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_dead_volume_trap_pocket_rim_{idx}"),
                24.0,
                7.0,
                5.0,
            )
            .translate(
                x + WINDOW_APERTURE_X / 2.0 + 14.0,
                y + 13.0,
                cap_z(WINDOW_Z, 5.0),
            );

        for tick in 0..BUBBLE_TICKS_PER_WINDOW {
            features = features
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_window_{idx}_bubble_ladder_tick_{tick}"
                    ),
                    3.0 + tick as f64 * 2.0,
                    3.0,
                    4.0,
                )
                .translate(
                    x - WINDOW_APERTURE_X / 2.0 - 9.0,
                    y - 14.0 + tick as f64 * 7.0,
                    cap_z(WINDOW_Z, 4.0),
                );
        }
    }
    plate - cuts + features
}

fn window_frame(index: usize, x: f64, y: f64) -> Part {
    let frame_z = cap_z(WINDOW_Z, 5.0);
    centered_cube(
        format!("closed_teer_electrode_recovery_station_bubble_window_{index}_front_frame"),
        WINDOW_APERTURE_X + 18.0,
        5.0,
        5.0,
    )
    .translate(x, y - WINDOW_APERTURE_Y / 2.0 - 5.0, frame_z)
        + centered_cube(
            format!("closed_teer_electrode_recovery_station_bubble_window_{index}_rear_frame"),
            WINDOW_APERTURE_X + 18.0,
            5.0,
            5.0,
        )
        .translate(x, y + WINDOW_APERTURE_Y / 2.0 + 5.0, frame_z)
        + centered_cube(
            format!("closed_teer_electrode_recovery_station_bubble_window_{index}_left_frame"),
            5.0,
            WINDOW_APERTURE_Y + 10.0,
            5.0,
        )
        .translate(x - WINDOW_APERTURE_X / 2.0 - 5.0, y, frame_z)
        + centered_cube(
            format!("closed_teer_electrode_recovery_station_bubble_window_{index}_right_frame"),
            5.0,
            WINDOW_APERTURE_Y + 10.0,
            5.0,
        )
        .translate(x + WINDOW_APERTURE_X / 2.0 + 5.0, y, frame_z)
}

fn barcode_coa_custody_lands() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_barcode_coa_custody_land_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z));
    let mut features = Part::empty("closed_teer_electrode_recovery_station_custody_features");
    for idx in 0..BARCODE_LANDS {
        let y = CUSTODY_POS.1 - CUSTODY_Y / 2.0 + 28.0 + idx as f64 * 20.0;
        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_barcode_custody_land_{idx}"),
                86.0,
                13.0,
                4.0,
            )
            .translate(CUSTODY_POS.0 - 44.0, y, cap_z(CUSTODY_Z, 4.0))
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_barcode_scan_stop_{idx}"),
                4.0,
                13.0,
                7.0,
            )
            .translate(CUSTODY_POS.0 + 3.0, y, cap_z(CUSTODY_Z, 7.0));
    }
    for idx in 0..COA_LANDS {
        features = features
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_coa_custody_land_{idx}"),
                74.0,
                22.0,
                4.0,
            )
            .translate(
                CUSTODY_POS.0 + 57.0,
                CUSTODY_POS.1 - 58.0 + idx as f64 * 58.0,
                cap_z(CUSTODY_Z, 4.0),
            );
    }
    plate + features + tamper_seal_tabs() + custody_fiducials()
}

fn tamper_seal_tabs() -> Part {
    let mut tabs = Part::empty("closed_teer_electrode_recovery_station_tamper_seal_tabs");
    for idx in 0..TAMPER_SEAL_TABS {
        tabs = tabs
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_tamper_seal_tab_{idx}"),
                28.0,
                9.0,
                6.0,
            )
            .translate(
                CUSTODY_POS.0 - CUSTODY_X / 2.0 + 23.0 + idx as f64 * 34.0,
                CUSTODY_POS.1 + CUSTODY_Y / 2.0 - 16.0,
                cap_z(CUSTODY_Z, 6.0),
            );
    }
    tabs
}

fn custody_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_teer_electrode_recovery_station_custody_fiducials");
    for (idx, (x, y)) in [(-86.0, -82.0), (86.0, -82.0), (-86.0, 82.0), (86.0, 82.0)]
        .iter()
        .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_teer_electrode_recovery_station_custody_fiducial_{idx}"
            ))
            .translate(CUSTODY_POS.0 + x, CUSTODY_POS.1 + y, cap_z(CUSTODY_Z, 2.0));
    }
    fiducials
}

fn release_hold_reject_gates() -> Part {
    let plate = centered_cube(
        "closed_teer_electrode_recovery_station_release_hold_reject_gate_plate",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z));
    let mut cuts = Part::empty("closed_teer_electrode_recovery_station_disposition_gate_cuts");
    let mut features =
        Part::empty("closed_teer_electrode_recovery_station_disposition_gate_features");
    for gate in 0..DISPOSITION_GATES {
        let x = GATE_POS.0 + gate_x(gate);
        let gate_height = 10.0 + gate as f64 * 5.0;
        features = features
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_{}_gate",
                    disposition_gate_name(gate)
                ),
                92.0,
                28.0,
                gate_height,
            )
            .translate(
                x,
                GATE_POS.1 + 34.0,
                DECK_Z / 2.0 + GATE_Z + gate_height / 2.0,
            )
            + centered_cube(
                format!(
                    "closed_teer_electrode_recovery_station_{}_gate_backstop",
                    disposition_gate_name(gate)
                ),
                96.0,
                8.0,
                12.0,
            )
            .translate(x, GATE_POS.1 - 54.0, cap_z(GATE_Z, 12.0));
        for slot in 0..GATE_TOKEN_SLOTS {
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_teer_electrode_recovery_station_{}_gate_token_slot_{slot}",
                        disposition_gate_name(gate)
                    ),
                    16.0,
                    28.0,
                    GATE_Z + 3.0,
                )
                .translate(
                    x - 30.0 + slot as f64 * 20.0,
                    GATE_POS.1 - 10.0,
                    top_z(GATE_Z),
                );
        }
    }
    plate - cuts + features + gate_quarantine_lips()
}

fn gate_quarantine_lips() -> Part {
    centered_cube(
        "closed_teer_electrode_recovery_station_hold_reject_quarantine_lip",
        GATE_PITCH_X * 2.0 + 92.0,
        7.0,
        13.0,
    )
    .translate(
        GATE_POS.0 + (gate_x(HOLD_GATE_INDEX) + gate_x(REJECT_GATE_INDEX)) / 2.0,
        GATE_POS.1 + GATE_Y / 2.0 - 20.0,
        cap_z(GATE_Z, 13.0),
    )
}

fn camera_evidence_bridge() -> Part {
    let post_z = DECK_Z / 2.0 + CAMERA_CLEARANCE_Z / 2.0;
    let beam_z = DECK_Z / 2.0 + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0;
    let left_post = centered_cube(
        "closed_teer_electrode_recovery_station_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_CLEARANCE_Z,
    )
    .translate(-CAMERA_X / 2.0, -20.0, post_z);
    let right_post = centered_cube(
        "closed_teer_electrode_recovery_station_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_CLEARANCE_Z,
    )
    .translate(CAMERA_X / 2.0, -20.0, post_z);
    let top_beam = centered_cube(
        "closed_teer_electrode_recovery_station_camera_bridge_top_beam",
        CAMERA_X,
        42.0,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, -20.0, beam_z);
    let field_front = centered_cube(
        "closed_teer_electrode_recovery_station_camera_evidence_field_front_rail",
        CAMERA_X - 170.0,
        5.0,
        5.0,
    )
    .translate(0.0, -20.0 - CAMERA_Y / 2.0, DECK_Z / 2.0 + 2.5);
    let field_rear = centered_cube(
        "closed_teer_electrode_recovery_station_camera_evidence_field_rear_rail",
        CAMERA_X - 170.0,
        5.0,
        5.0,
    )
    .translate(0.0, -20.0 + CAMERA_Y / 2.0, DECK_Z / 2.0 + 2.5);
    let lens = centered_cylinder(
        "closed_teer_electrode_recovery_station_camera_downlook_lens",
        13.0,
        34.0,
        36,
    )
    .translate(120.0, -20.0, DECK_Z / 2.0 + CAMERA_CLEARANCE_Z - 17.0);

    left_post
        + right_post
        + top_beam
        + field_front
        + field_rear
        + lens
        + camera_mounts()
        + evidence_fiducial_targets()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_teer_electrode_recovery_station_camera_mounts");
    for idx in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("closed_teer_electrode_recovery_station_camera_mount_slot_{idx}"),
                34.0,
                6.0,
                5.0,
            )
            .translate(
                centered_index(idx, CAMERA_MOUNTS, 78.0),
                -20.0,
                DECK_Z / 2.0 + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z + 2.5,
            );
    }
    mounts
}

fn evidence_fiducial_targets() -> Part {
    let mut targets =
        Part::empty("closed_teer_electrode_recovery_station_camera_evidence_fiducials");
    for idx in 0..EVIDENCE_FIDUCIALS {
        let x = centered_index(idx % 5, 5, 180.0);
        let y = -20.0 + if idx < 5 { -224.0 } else { 224.0 };
        targets = targets
            + fiducial_disc(&format!(
                "closed_teer_electrode_recovery_station_camera_evidence_fiducial_{idx}"
            ))
            .translate(x, y, DECK_Z / 2.0 + 2.0);
    }
    targets
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_teer_electrode_recovery_station_robot_keepout_front_pick_sweep",
        DECK_X - 180.0,
        8.0,
        6.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 42.0, DECK_Z / 2.0 + 3.0);
    let front_robot_depth = centered_cube(
        "closed_teer_electrode_recovery_station_robot_keepout_front_depth_marker",
        DECK_X - 210.0,
        FRONT_ROBOT_SWEEP,
        4.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_SWEEP / 2.0,
        DECK_Z / 2.0 + 2.0,
    );
    let rear_service = centered_cube(
        "closed_teer_electrode_recovery_station_service_keepout_rear_rinse_access",
        DECK_X - 190.0,
        8.0,
        6.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 42.0, DECK_Z / 2.0 + 3.0);
    let rear_depth = centered_cube(
        "closed_teer_electrode_recovery_station_service_keepout_rear_depth_marker",
        DECK_X - 220.0,
        REAR_SERVICE_CLEARANCE,
        4.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        DECK_Z / 2.0 + 2.0,
    );
    let left_service = centered_cube(
        "closed_teer_electrode_recovery_station_service_keepout_left_cartridge_load",
        8.0,
        DECK_Y - 160.0,
        6.0,
    )
    .translate(-DECK_X / 2.0 + 42.0, 0.0, DECK_Z / 2.0 + 3.0);
    let left_depth = centered_cube(
        "closed_teer_electrode_recovery_station_service_keepout_left_depth_marker",
        SIDE_SERVICE_CLEARANCE,
        DECK_Y - 180.0,
        4.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE / 2.0,
        0.0,
        DECK_Z / 2.0 + 2.0,
    );
    let top_height_flag = centered_cube(
        "closed_teer_electrode_recovery_station_service_keepout_top_bridge_height_flag",
        120.0,
        14.0,
        TOP_SERVICE_CLEARANCE_Z / 8.0,
    )
    .translate(CAMERA_X / 2.0 - 74.0, -20.0, TOP_SERVICE_CLEARANCE_Z / 16.0);
    front_robot
        + front_robot_depth
        + rear_service
        + rear_depth
        + left_service
        + left_depth
        + top_height_flag
}

fn module_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "electrode_cartridge_nests",
            center: ELECTRODE_POS,
            x: ELECTRODE_X,
            y: ELECTRODE_Y,
        },
        Footprint {
            name: "cleaning_rinse_path_witness",
            center: CLEANING_POS,
            x: CLEANING_X,
            y: CLEANING_Y,
        },
        Footprint {
            name: "fouling_coupon_ladder",
            center: FOULING_POS,
            x: FOULING_X,
            y: FOULING_Y,
        },
        Footprint {
            name: "impedance_phantom_pockets",
            center: PHANTOM_POS,
            x: PHANTOM_X,
            y: PHANTOM_Y,
        },
        Footprint {
            name: "wet_dry_storage_docks",
            center: STORAGE_POS,
            x: STORAGE_X,
            y: STORAGE_Y,
        },
        Footprint {
            name: "barcode_coa_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "timestamp_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "bubble_dead_volume_windows",
            center: WINDOW_POS,
            x: WINDOW_X,
            y: WINDOW_Y,
        },
        Footprint {
            name: "release_hold_reject_gates",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn electrode_center(index: usize) -> (f64, f64) {
    let row = index / ELECTRODE_COLS;
    let col = index % ELECTRODE_COLS;
    let (x, y) = grid_center(
        col,
        row,
        ELECTRODE_COLS,
        ELECTRODE_ROWS,
        ELECTRODE_PITCH_X,
        ELECTRODE_PITCH_Y,
    );
    (ELECTRODE_POS.0 + x, ELECTRODE_POS.1 + y)
}

fn phantom_center(index: usize) -> (f64, f64) {
    let row = index / PHANTOM_COLS;
    let col = index % PHANTOM_COLS;
    let (x, y) = grid_center(
        col,
        row,
        PHANTOM_COLS,
        PHANTOM_ROWS,
        PHANTOM_PITCH_X,
        PHANTOM_PITCH_Y,
    );
    (PHANTOM_POS.0 + x, PHANTOM_POS.1 + y)
}

fn window_center(index: usize) -> (f64, f64) {
    let row = index / WINDOW_COLS;
    let col = index % WINDOW_COLS;
    let (x, y) = grid_center(
        col,
        row,
        WINDOW_COLS,
        WINDOW_ROWS,
        WINDOW_PITCH_X,
        WINDOW_PITCH_Y,
    );
    (WINDOW_POS.0 + x, WINDOW_POS.1 + y)
}

fn grid_center(
    col: usize,
    row: usize,
    cols: usize,
    rows: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    (
        (col as f64 - (cols as f64 - 1.0) / 2.0) * pitch_x,
        (row as f64 - (rows as f64 - 1.0) / 2.0) * pitch_y,
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn coupon_x(index: usize, count: usize, pitch: f64) -> f64 {
    centered_index(index, count, pitch)
}

fn fouling_level_y(level: usize) -> f64 {
    centered_index(level, FOULING_LEVELS, FOULING_LEVEL_PITCH_Y)
}

fn cleaning_lane_y(lane: usize) -> f64 {
    centered_index(lane, CLEANING_LANES, CLEANING_LANE_PITCH_Y)
}

fn rinse_stage_x(stage: usize) -> f64 {
    centered_index(
        stage,
        RINSE_STAGES,
        (CLEANING_X - 156.0) / (RINSE_STAGES as f64 - 1.0),
    )
}

fn storage_lane_y(lane: usize) -> f64 {
    centered_index(lane, STORAGE_LANES, STORAGE_LANE_PITCH_Y)
}

fn storage_dock_x(dock: usize) -> f64 {
    centered_index(dock, STORAGE_DOCKS_PER_LANE, STORAGE_DOCK_PITCH_X)
}

fn timestamp_token_x(index: usize) -> f64 {
    centered_index(index, TIMESTAMP_TOKENS, TOKEN_PITCH_X)
}

fn gate_x(index: usize) -> f64 {
    centered_index(index, DISPOSITION_GATES, GATE_PITCH_X)
}

fn storage_lane_name(lane: usize) -> &'static str {
    match lane {
        DRY_LANE_INDEX => "dry",
        WET_LANE_INDEX => "wet",
        _ => panic!("unknown storage lane"),
    }
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate"),
    }
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn cap_z(base_height: f64, cap_height: f64) -> f64 {
    DECK_Z / 2.0 + base_height + cap_height / 2.0
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center_cut"), 1.2, 3.0, 18);
    disc - center
}

fn electrode_grid_span_x() -> f64 {
    (ELECTRODE_COLS as f64 - 1.0) * ELECTRODE_PITCH_X + ELECTRODE_POCKET_X
}

fn electrode_grid_span_y() -> f64 {
    (ELECTRODE_ROWS as f64 - 1.0) * ELECTRODE_PITCH_Y + ELECTRODE_POCKET_Y
}

fn cleaning_channel_span_y() -> f64 {
    (CLEANING_LANES as f64 - 1.0) * CLEANING_LANE_PITCH_Y + CLEANING_CHANNEL_W
}

fn fouling_ladder_span_y() -> f64 {
    (FOULING_LEVELS as f64 - 1.0) * FOULING_LEVEL_PITCH_Y + FOULING_COUPON_Y
}

fn phantom_grid_span_x() -> f64 {
    (PHANTOM_COLS as f64 - 1.0) * PHANTOM_PITCH_X + PHANTOM_POCKET_X
}

fn storage_dock_span_x() -> f64 {
    (STORAGE_DOCKS_PER_LANE as f64 - 1.0) * STORAGE_DOCK_PITCH_X + STORAGE_DOCK_D
}

fn token_span_x() -> f64 {
    (TIMESTAMP_TOKENS as f64 - 1.0) * TOKEN_PITCH_X + TOKEN_D
}

fn window_span_x() -> f64 {
    (WINDOW_COLS as f64 - 1.0) * WINDOW_PITCH_X + WINDOW_APERTURE_X
}

fn footprint_fits_inner_deck(footprint: Footprint) -> bool {
    let usable_x = DECK_X / 2.0 - RIM_W - 10.0;
    let usable_y = DECK_Y / 2.0 - RIM_W - 10.0;
    footprint.center.0.abs() + footprint.x / 2.0 <= usable_x
        && footprint.center.1.abs() + footprint.y / 2.0 <= usable_y
}

fn overlaps(left: Footprint, right: Footprint) -> bool {
    let dx = (left.center.0 - right.center.0).abs();
    let dy = (left.center.1 - right.center.1).abs();
    dx < (left.x + right.x) / 2.0 && dy < (left.y + right.y) / 2.0
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 24);
    assert_eq!(ELECTRODE_NESTS, ELECTRODE_ROWS * ELECTRODE_COLS);
    assert_eq!(FOULING_COUPONS, FOULING_LEVELS * COUPONS_PER_LEVEL);
    assert_eq!(PHANTOM_POCKETS, PHANTOM_ROWS * PHANTOM_COLS);
    assert_eq!(
        PHANTOM_TERMINAL_LANDS,
        PHANTOM_POCKETS * PHANTOM_TERMINAL_PAIRS_PER_POCKET * 2
    );
    assert_eq!(RINSE_WELLS, CLEANING_LANES * RINSE_STAGES);
    assert_eq!(WET_DOCKS, STORAGE_DOCKS_PER_LANE);
    assert_eq!(DRY_DOCKS, STORAGE_DOCKS_PER_LANE);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert_eq!(BUBBLE_WINDOWS, WINDOW_ROWS * WINDOW_COLS);
    assert_eq!(DEAD_VOLUME_TRAP_POCKETS, BUBBLE_WINDOWS);
    assert_eq!(CUSTODY_FIDUCIALS, 4);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 7);
    assert!(ELECTRODE_CONTACT_WIPE_LANDS >= ELECTRODE_NESTS);
    assert!(CAMERA_CLEARANCE_Z > ELECTRODE_Z.max(CLEANING_Z).max(PHANTOM_Z) + 96.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z);

    assert!(electrode_grid_span_x() < ELECTRODE_X - 48.0);
    assert!(electrode_grid_span_y() < ELECTRODE_Y - 48.0);
    assert!(cleaning_channel_span_y() < CLEANING_Y - 52.0);
    assert!(fouling_ladder_span_y() < FOULING_Y - 30.0);
    assert!(phantom_grid_span_x() < PHANTOM_X - 72.0);
    assert!(storage_dock_span_x() < STORAGE_X - 48.0);
    assert!(token_span_x() < TOKEN_X - 34.0);
    assert!(window_span_x() < WINDOW_X - 74.0);

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            footprint_fits_inner_deck(footprint),
            "{} exceeds inner deck",
            footprint.name
        );
    }
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            assert!(
                !overlaps(footprints[left], footprints[right]),
                "{} overlaps {}",
                footprints[left].name,
                footprints[right].name
            );
        }
    }
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
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert_eq!(OUTPUTS[12], format!("{OUTPUT_PREFIX}assembly.stl"));

        let joined = OUTPUTS.join("\n");
        for requested_output in [
            "electrode_cartridge_nests",
            "fouling_coupon_ladder",
            "cleaning_rinse_path_witness",
            "impedance_phantom_pockets",
            "wet_dry_storage_docks",
            "timestamp_token_rail",
            "bubble_dead_volume_windows",
            "custody_lands",
            "release_hold_reject_gates",
        ] {
            assert!(joined.contains(requested_output), "{requested_output}");
        }
    }

    #[test]
    fn required_feature_list_covers_requested_validation_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 24);
        for feature in [
            "electrode_cartridge_nests",
            "fouling_coupon_ladder",
            "cleaning_rinse_path_witness",
            "impedance_phantom_pockets",
            "wet_storage_docks",
            "dry_storage_docks",
            "timestamp_token_rail",
            "bubble_dead_volume_windows",
            "barcode_custody_lands",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "robot_keepouts",
            "service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }
    }

    #[test]
    fn dimensions_match_closed_station_packaging() {
        assert_eq!(DECK_X, 1400.0);
        assert_eq!(DECK_Y, 940.0);
        assert!(RIM_Z >= 50.0);
        assert!(SOCKET_DEPTH >= 4.0);
        assert_layout();
    }

    #[test]
    fn feature_counts_cover_fouling_cleaning_and_recovery_paths() {
        assert_eq!(ELECTRODE_NESTS, 8);
        assert_eq!(ELECTRODE_CONTACT_WIPE_LANDS, ELECTRODE_NESTS);
        assert_eq!(FOULING_LEVELS, 7);
        assert_eq!(FOULING_COUPONS, 14);
        assert_eq!(CLEANING_LANES, 6);
        assert_eq!(RINSE_WELLS, 24);
        assert_eq!(PHANTOM_POCKETS, 6);
        assert_eq!(WET_DOCKS, 6);
        assert_eq!(DRY_DOCKS, 6);
        assert_eq!(TIMESTAMP_TOKENS, 10);
        assert_eq!(BUBBLE_WINDOWS, 6);
        assert_eq!(DEAD_VOLUME_TRAP_POCKETS, 6);
        assert_eq!(DISPOSITION_GATES, 3);
    }

    #[test]
    fn repeated_features_stay_inside_their_modules() {
        assert!(electrode_grid_span_x() < ELECTRODE_X - 48.0);
        assert!(electrode_grid_span_y() < ELECTRODE_Y - 48.0);
        assert!(cleaning_channel_span_y() < CLEANING_Y - 52.0);
        assert!(fouling_ladder_span_y() < FOULING_Y - 30.0);
        assert!(phantom_grid_span_x() < PHANTOM_X - 72.0);
        assert!(storage_dock_span_x() < STORAGE_X - 48.0);
        assert!(token_span_x() < TOKEN_X - 34.0);
        assert!(window_span_x() < WINDOW_X - 74.0);
    }

    #[test]
    fn disposition_and_storage_names_are_stable() {
        assert_eq!(storage_lane_name(DRY_LANE_INDEX), "dry");
        assert_eq!(storage_lane_name(WET_LANE_INDEX), "wet");
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert!(storage_lane_y(WET_LANE_INDEX) > storage_lane_y(DRY_LANE_INDEX));
    }

    #[test]
    fn scope_avoids_process_and_biology_claims() {
        assert!(DESIGN_SCOPE.contains("mechanical validation packaging only"));
        assert!(DESIGN_SCOPE.contains("not a biological acceptance criterion"));
        assert!(DESIGN_SCOPE.contains("cleaning SOP"));
        assert!(DESIGN_SCOPE.contains("sterile-process claim"));
        assert!(DESIGN_SCOPE.contains("barrier-tissue interpretation rule"));
    }
}
