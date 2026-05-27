use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic probe clean-storage and sterile rehydration station.
//
// Intent:
// - Validate reusable robotic probe storage between closed-system runs across
//   clean, dry, and wet hold states without mixing custody lanes.
// - Make sterile rehydration, cleaning-verification coupons, carryover witness
//   wells, barcode/COA custody, disposition gates, camera evidence capture, and
//   robot/service keepouts explicit as deterministic CSG geometry.
// - Model validation-station architecture only. This is not a cleaning method,
//   sterility claim, COA authority, acceptance-limit definition, or cell-run
//   release procedure.

const BIN_NAME: &str = "closed_robotic_probe_clean_storage_rehydration_station";
const OUTPUT_PREFIX: &str = "output/closed_robotic_probe_clean_storage_rehydration_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_robotic_probe_clean_storage_rehydration_station_closed_containment_deck.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_clean_dry_wet_storage_bank.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_sterile_rehydration_path.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_cleaning_verification_coupon_bridge.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_carryover_witness_well_plate.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_barcode_coa_custody_panel.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_release_hold_reject_gate.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_camera_evidence_bridge.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_closed_waste_quarantine_tray.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_state_transfer_token_rail.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_lid_seal_chimney_guard.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_robot_service_keepout_gauge.stl",
    "output/closed_robotic_probe_clean_storage_rehydration_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "clean_storage_state",
    "dry_storage_state",
    "wet_storage_state",
    "sterile_rehydration_path",
    "cleaning_verification_coupons",
    "carryover_witness_wells",
    "barcode_coa_custody",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "closed_waste_quarantine",
    "robot_service_keepouts",
    "deterministic_stl_exports",
];

const LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "no_cleaning_protocol",
    "no_sterility_assurance_claim",
    "no_acceptance_limits",
    "no_coa_authority",
    "no_cell_run_release_procedure",
];

const PARAMETER_SET_REV: &str = "robotic-probe-clean-storage-rehydration-station-rev-a";
const OUTPUT_MANIFEST_REV: &str = "source-only-stl-manifest-rev-a";
const USES_RANDOMNESS: bool = false;
const RANDOM_SEED: u64 = 0;

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const MODULE_MARGIN_MM: f64 = 16.0;
const MODULE_GAP_MM: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_PORT_D: f64 = 15.0;

const PROBE_TYPES: usize = 8;
const STORAGE_STATES: usize = 3;
const STATE_NAMES: [&str; STORAGE_STATES] = ["clean", "dry", "wet"];
const PROBE_NAMES: [&str; PROBE_TYPES] = [
    "aspirate",
    "dispense",
    "ph",
    "do",
    "temperature",
    "conductivity",
    "pressure",
    "sampling",
];
const PROBE_SLOTS: usize = PROBE_TYPES * STORAGE_STATES;
const STORAGE_BANK_X: f64 = 500.0;
const STORAGE_BANK_Y: f64 = 260.0;
const STORAGE_BANK_Z: f64 = 72.0;
const STORAGE_BANK_POS: (f64, f64) = (-420.0, 250.0);
const STATE_LANE_PITCH_Y: f64 = 76.0;
const PROBE_PITCH_X: f64 = 54.0;
const PROBE_SLOT_D: f64 = 15.8;
const PROBE_SLOT_Y: f64 = 98.0;
const PROBE_TIP_WELL_D: f64 = 21.0;
const WET_BUFFER_DEPTH_MM: f64 = 38.0;
const DRY_DESICCANT_CHANNELS: usize = 6;
const CLEAN_CAP_LANDS: usize = PROBE_TYPES;

const REHYDRATION_X: f64 = 380.0;
const REHYDRATION_Y: f64 = 260.0;
const REHYDRATION_Z: f64 = 68.0;
const REHYDRATION_POS: (f64, f64) = (30.0, 250.0);
const STERILE_BAG_PORTS: usize = 4;
const HYDRATION_CHIMNEYS: usize = PROBE_TYPES;
const REHYDRATION_FLUSH_BRANCHES: usize = 4;
const STERILE_FILTER_D: f64 = 34.0;
const PATH_BORE_D: f64 = 6.0;
const CHIMNEY_D: f64 = 24.0;
const CHIMNEY_DEPTH: f64 = 42.0;

const COUPON_X: f64 = 300.0;
const COUPON_Y: f64 = 260.0;
const COUPON_Z: f64 = 38.0;
const COUPON_POS: (f64, f64) = (470.0, 250.0);
const COUPON_COUNT: usize = 12;
const COUPON_COLS: usize = 6;
const COUPON_ROWS: usize = 2;
const COUPON_SLOT_X: f64 = 34.0;
const COUPON_SLOT_Y: f64 = 84.0;
const ATP_SWAB_LANDS: usize = 6;
const RESIDUE_CARD_LANDS: usize = 4;

const WITNESS_X: f64 = 500.0;
const WITNESS_Y: f64 = 190.0;
const WITNESS_Z: f64 = 44.0;
const WITNESS_POS: (f64, f64) = (-420.0, -40.0);
const CARRYOVER_WELLS: usize = PROBE_TYPES * 2;
const BLANK_WITNESS_WELLS: usize = 4;
const POSITIVE_CONTROL_WELLS: usize = 4;
const TOTAL_WITNESS_WELLS: usize = CARRYOVER_WELLS + BLANK_WITNESS_WELLS + POSITIVE_CONTROL_WELLS;
const WITNESS_COLS: usize = 8;
const WITNESS_ROWS: usize = 3;
const WITNESS_WELL_D: f64 = 20.0;
const WITNESS_PITCH_X: f64 = 48.0;
const WITNESS_PITCH_Y: f64 = 50.0;
const DYE_GRADIENT_LANDS: usize = 6;

const CUSTODY_X: f64 = 380.0;
const CUSTODY_Y: f64 = 190.0;
const CUSTODY_Z: f64 = 18.0;
const CUSTODY_POS: (f64, f64) = (40.0, -40.0);
const BARCODE_LANDS: usize = PROBE_SLOTS;
const COA_CARD_LANDS: usize = 6;
const RUN_RECORD_LANDS: usize = 4;
const CUSTODY_STATE_TABS: usize = 5;

const GATE_X: f64 = 300.0;
const GATE_Y: f64 = 190.0;
const GATE_Z: f64 = 62.0;
const GATE_POS: (f64, f64) = (470.0, -40.0);
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 5;
const GATE_TEETH_PER_LANE: usize = 6;
const LOCKOUT_FLAG_SLOTS: usize = 6;

const CAMERA_X: f64 = 1160.0;
const CAMERA_Y: f64 = 76.0;
const CAMERA_Z: f64 = 220.0;
const CAMERA_POS: (f64, f64) = (0.0, -210.0);
const CAMERA_PODS: usize = 4;
const EVIDENCE_LIGHT_BARS: usize = 8;
const FOCUS_TARGETS: usize = 6;
const CAMERA_UNDERSIDE_CLEARANCE_Z: f64 = 176.0;

const WASTE_X: f64 = 420.0;
const WASTE_Y: f64 = 150.0;
const WASTE_Z: f64 = 58.0;
const WASTE_POS: (f64, f64) = (-450.0, -335.0);
const WASTE_QUARANTINE_CUPS: usize = 6;
const WASTE_STREAM_PORTS: usize = 4;
const LEAK_SENSOR_WELLS: usize = 4;
const WASTE_CUP_D: f64 = 32.0;

const TOKEN_X: f64 = 390.0;
const TOKEN_Y: f64 = 150.0;
const TOKEN_Z: f64 = 28.0;
const TOKEN_POS: (f64, f64) = (10.0, -335.0);
const STATE_TRANSFER_TOKENS: usize = PROBE_TYPES;
const STEP_TOKEN_STATES: usize = 4;
const TOKEN_COLS: usize = 4;
const TOKEN_ROWS: usize = 2;
const TOKEN_SLOT_X: f64 = 50.0;
const TOKEN_SLOT_Y: f64 = 46.0;

const LID_X: f64 = 300.0;
const LID_Y: f64 = 150.0;
const LID_Z: f64 = 112.0;
const LID_POS: (f64, f64) = (460.0, -335.0);
const LID_SEAL_RIBS: usize = 5;
const CHIMNEY_GUARDS: usize = 4;
const SEAL_WITNESS_WINDOWS: usize = 3;

const KEEP_OUT_X: f64 = 1340.0;
const KEEP_OUT_Y: f64 = 820.0;
const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_FRONT_APPROACH_CLEARANCE_MM: f64 = 430.0;
const REAR_SERVICE_CLEARANCE_MM: f64 = 290.0;
const LEFT_PROBE_LOAD_CLEARANCE_MM: f64 = 240.0;
const RIGHT_WASTE_SERVICE_CLEARANCE_MM: f64 = 220.0;
const OVERHEAD_CAMERA_SERVICE_CLEARANCE_MM: f64 = 260.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - MODULE_MARGIN_MM
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - MODULE_MARGIN_MM
    }

    fn overlaps(self, other: Footprint, margin: f64) -> bool {
        let left_a = self.center.0 - self.x / 2.0;
        let right_a = self.center.0 + self.x / 2.0;
        let bottom_a = self.center.1 - self.y / 2.0;
        let top_a = self.center.1 + self.y / 2.0;

        let left_b = other.center.0 - other.x / 2.0;
        let right_b = other.center.0 + other.x / 2.0;
        let bottom_b = other.center.1 - other.y / 2.0;
        let top_b = other.center.1 + other.y / 2.0;

        left_a < right_b + margin
            && right_a + margin > left_b
            && bottom_a < top_b + margin
            && top_a + margin > bottom_b
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = closed_containment_deck();
    export(OUTPUTS[0], &deck);

    let storage = clean_dry_wet_storage_bank();
    export(OUTPUTS[1], &storage);

    let rehydration = sterile_rehydration_path();
    export(OUTPUTS[2], &rehydration);

    let coupons = cleaning_verification_coupon_bridge();
    export(OUTPUTS[3], &coupons);

    let witness = carryover_witness_well_plate();
    export(OUTPUTS[4], &witness);

    let custody = barcode_coa_custody_panel();
    export(OUTPUTS[5], &custody);

    let gates = release_hold_reject_gate();
    export(OUTPUTS[6], &gates);

    let camera = camera_evidence_bridge();
    export(OUTPUTS[7], &camera);

    let waste = closed_waste_quarantine_tray();
    export(OUTPUTS[8], &waste);

    let tokens = state_transfer_token_rail();
    export(OUTPUTS[9], &tokens);

    let lid_guard = lid_seal_chimney_guard();
    export(OUTPUTS[10], &lid_guard);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        deck + storage.translate(
            STORAGE_BANK_POS.0,
            STORAGE_BANK_POS.1,
            on_deck_z(STORAGE_BANK_Z),
        ) + rehydration.translate(
            REHYDRATION_POS.0,
            REHYDRATION_POS.1,
            on_deck_z(REHYDRATION_Z),
        ) + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_Z))
            + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
            + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
            + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
            + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
            + waste.translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_Z))
            + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
            + lid_guard.translate(LID_POS.0, LID_POS.1, on_deck_z(LID_Z))
            + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed robotic probe clean-storage and rehydration station:");
    println!(
        "  Parameter set:             {PARAMETER_SET_REV}; manifest {OUTPUT_MANIFEST_REV}; deterministic={}",
        !USES_RANDOMNESS
    );
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm closed containment deck"
    );
    println!(
        "  Probe storage states:      {STORAGE_STATES} lanes ({}) x {PROBE_TYPES} probe families = {PROBE_SLOTS} state slots",
        STATE_NAMES.join(", ")
    );
    println!(
        "  Sterile rehydration path:  {STERILE_BAG_PORTS} bag ports, {HYDRATION_CHIMNEYS} probe chimneys, {REHYDRATION_FLUSH_BRANCHES} closed flush branches"
    );
    println!(
        "  Evidence coupons/wells:    {COUPON_COUNT} cleaning coupons, {TOTAL_WITNESS_WELLS} carryover witness wells"
    );
    println!(
        "  Custody/disposition:       {BARCODE_LANDS} barcode lands, {COA_CARD_LANDS} COA lands, release/hold/reject gates with {DISPOSITION_SLOTS_PER_LANE} slots each"
    );
    println!(
        "  Camera/keepouts:           {CAMERA_PODS} evidence camera pods, {KEEP_OUT_ZONE_COUNT} robot/service keepout zones"
    );
    println!("  STL outputs:               {}", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn name(suffix: &str) -> String {
    format!("{BIN_NAME}_{suffix}")
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn module_footprints() -> [Footprint; 10] {
    [
        footprint(
            "clean_dry_wet_storage_bank",
            STORAGE_BANK_POS,
            STORAGE_BANK_X,
            STORAGE_BANK_Y,
        ),
        footprint(
            "sterile_rehydration_path",
            REHYDRATION_POS,
            REHYDRATION_X,
            REHYDRATION_Y,
        ),
        footprint(
            "cleaning_verification_coupon_bridge",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        footprint(
            "carryover_witness_well_plate",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        footprint(
            "barcode_coa_custody_panel",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        footprint("release_hold_reject_gate", GATE_POS, GATE_X, GATE_Y),
        footprint("closed_waste_quarantine_tray", WASTE_POS, WASTE_X, WASTE_Y),
        footprint("state_transfer_token_rail", TOKEN_POS, TOKEN_X, TOKEN_Y),
        footprint("lid_seal_chimney_guard", LID_POS, LID_X, LID_Y),
        footprint("camera_evidence_bridge", CAMERA_POS, CAMERA_X, CAMERA_Y),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert!(!USES_RANDOMNESS);
    assert_eq!(RANDOM_SEED, 0);

    for feature in [
        "clean_storage_state",
        "dry_storage_state",
        "wet_storage_state",
        "sterile_rehydration_path",
        "cleaning_verification_coupons",
        "carryover_witness_wells",
        "barcode_coa_custody",
        "release_hold_reject_gates",
        "camera_evidence_bridge",
        "robot_service_keepouts",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }
    for limitation in [
        "validation_fixture_only",
        "no_cleaning_protocol",
        "no_sterility_assurance_claim",
        "no_acceptance_limits",
        "no_coa_authority",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    assert_eq!(PROBE_SLOTS, PROBE_TYPES * STORAGE_STATES);
    assert_eq!(PROBE_NAMES.len(), PROBE_TYPES);
    assert_eq!(STATE_NAMES.len(), STORAGE_STATES);
    assert_eq!(HYDRATION_CHIMNEYS, PROBE_TYPES);
    assert_eq!(COUPON_COUNT, COUPON_COLS * COUPON_ROWS);
    assert_eq!(TOTAL_WITNESS_WELLS, WITNESS_COLS * WITNESS_ROWS);
    assert_eq!(STATE_TRANSFER_TOKENS, TOKEN_COLS * TOKEN_ROWS);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(WET_BUFFER_DEPTH_MM > CHIMNEY_DEPTH - 8.0);
    assert!(PROBE_SLOT_Y > WET_BUFFER_DEPTH_MM * 2.0);
    assert!(CAMERA_UNDERSIDE_CLEARANCE_Z > STORAGE_BANK_Z + 80.0);
    assert!(OVERHEAD_CAMERA_SERVICE_CLEARANCE_MM > CAMERA_UNDERSIDE_CLEARANCE_Z);
    assert!(closed_waste_capacity_ml() > rehydration_challenge_volume_ml());

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_station(),
            "{} exceeds containment deck",
            module.name
        );
    }
    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(
                !a.overlaps(*b, MODULE_GAP_MM),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }
}

fn closed_waste_capacity_ml() -> f64 {
    WASTE_QUARANTINE_CUPS as f64 * 42.0
}

fn rehydration_challenge_volume_ml() -> f64 {
    HYDRATION_CHIMNEYS as f64 * 12.0 + REHYDRATION_FLUSH_BRANCHES as f64 * 8.0
}

fn closed_containment_deck() -> Part {
    let deck = centered_cube(
        name("closed_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let recessed_pan = centered_cube(
        name("closed_containment_recessed_leak_pan"),
        STATION_X - 140.0,
        STATION_Y - 126.0,
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z / 2.0 - 4.0);
    let front_gutter = centered_cube(
        name("closed_containment_front_probe_drip_gutter"),
        STATION_X - 180.0,
        28.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 - 4.5);
    let drain = centered_cylinder(
        name("closed_containment_quarantine_drain_port"),
        DRAIN_PORT_D / 2.0,
        56.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-STATION_X / 2.0 + 130.0, -STATION_Y / 2.0 + 54.0, 0.0);

    deck - recessed_pan - front_gutter - drain - deck_locator_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + deck_zone_dividers()
        + datum_targets()
        + leak_witness_ribs()
}

fn deck_locator_sockets() -> Part {
    let mut sockets = Part::empty(name("deck_locator_sockets"));
    for module in module_footprints().iter().take(9) {
        sockets = sockets
            + centered_cube(
                name(&format!("{}_locator_socket", module.name)),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.8,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn deck_mounting_holes() -> Part {
    let mut holes = Part::empty(name("deck_mounting_holes"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 64.0),
        (STATION_X / 2.0 - 64.0, -STATION_Y / 2.0 + 64.0),
        (-STATION_X / 2.0 + 64.0, STATION_Y / 2.0 - 64.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 64.0),
        (0.0, -STATION_Y / 2.0 + 64.0),
        (0.0, STATION_Y / 2.0 - 64.0),
        (-STATION_X / 2.0 + 64.0, 0.0),
        (STATION_X / 2.0 - 64.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                name(&format!("deck_m6_mount_clearance_{i}")),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                name(&format!("deck_m6_mount_slot_relief_{i}")),
                28.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(name("front_low_drip_lip"), STATION_X - 150.0, RIM_W, 24.0)
        .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 12.0);
    let rear = centered_cube(name("rear_service_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(name("left_closed_side_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(name("right_closed_side_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn deck_zone_dividers() -> Part {
    let upper_to_middle = centered_cube(
        name("storage_rehydration_to_evidence_zone_divider"),
        STATION_X - 190.0,
        10.0,
        26.0,
    )
    .translate(0.0, 105.0, BASE_Z / 2.0 + 13.0);
    let middle_to_lower = centered_cube(
        name("evidence_to_disposition_zone_divider"),
        STATION_X - 190.0,
        10.0,
        24.0,
    )
    .translate(0.0, -170.0, BASE_Z / 2.0 + 12.0);
    let storage_to_rehydration = centered_cube(
        name("storage_to_sterile_rehydration_divider"),
        10.0,
        250.0,
        26.0,
    )
    .translate(-190.0, 250.0, BASE_Z / 2.0 + 13.0);
    let rehydration_to_coupon =
        centered_cube(name("rehydration_to_coupon_divider"), 10.0, 250.0, 26.0).translate(
            245.0,
            250.0,
            BASE_Z / 2.0 + 13.0,
        );
    let custody_to_gate = centered_cube(
        name("custody_to_disposition_gate_divider"),
        10.0,
        190.0,
        24.0,
    )
    .translate(255.0, -40.0, BASE_Z / 2.0 + 12.0);
    upper_to_middle
        + middle_to_lower
        + storage_to_rehydration
        + rehydration_to_coupon
        + custody_to_gate
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(name("datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 98.0, -STATION_Y / 2.0 + 98.0),
        (STATION_X / 2.0 - 98.0, -STATION_Y / 2.0 + 98.0),
        (-STATION_X / 2.0 + 98.0, STATION_Y / 2.0 - 98.0),
        (STATION_X / 2.0 - 98.0, STATION_Y / 2.0 - 98.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("deck_datum_{i}")).translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty(name("deck_leak_witness_ribs"));
    for i in 0..7 {
        ribs = ribs
            + centered_cube(
                name(&format!("deck_leak_witness_rib_{i}")),
                STATION_X - 220.0,
                4.0,
                5.0,
            )
            .translate(0.0, centered_index(i, 7, 92.0) - 30.0, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn clean_dry_wet_storage_bank() -> Part {
    let body = centered_cube(
        name("clean_dry_wet_storage_bank_body"),
        STORAGE_BANK_X,
        STORAGE_BANK_Y,
        STORAGE_BANK_Z,
    );
    let rear_cable_fence = centered_cube(
        name("storage_bank_rear_cable_fence"),
        STORAGE_BANK_X,
        14.0,
        STORAGE_BANK_Z + 36.0,
    )
    .translate(0.0, STORAGE_BANK_Y / 2.0 - 7.0, 18.0);
    let front_state_lip = centered_cube(
        name("storage_bank_front_state_lip"),
        STORAGE_BANK_X,
        12.0,
        30.0,
    )
    .translate(
        0.0,
        -STORAGE_BANK_Y / 2.0 + 6.0,
        STORAGE_BANK_Z / 2.0 + 15.0,
    );

    let mut cuts = Part::empty(name("storage_bank_probe_socket_cuts"));
    let mut state_features = Part::empty(name("storage_bank_state_features"));
    let mut labels = Part::empty(name("storage_bank_state_label_lands"));

    for state in 0..STORAGE_STATES {
        let y = centered_index(state, STORAGE_STATES, STATE_LANE_PITCH_Y);
        state_features = state_features
            + centered_cube(
                name(&format!("{}_state_lane_raised_rail", STATE_NAMES[state])),
                STORAGE_BANK_X - 38.0,
                5.0,
                10.0,
            )
            .translate(0.0, y + 32.0, STORAGE_BANK_Z / 2.0 + 5.0)
            + centered_cube(
                name(&format!("{}_state_lane_barrier_rail", STATE_NAMES[state])),
                STORAGE_BANK_X - 38.0,
                5.0,
                10.0,
            )
            .translate(0.0, y - 32.0, STORAGE_BANK_Z / 2.0 + 5.0);

        for probe in 0..PROBE_TYPES {
            let x = centered_index(probe, PROBE_TYPES, PROBE_PITCH_X);
            let slot_name = format!("{}_{}_probe", STATE_NAMES[state], PROBE_NAMES[probe]);
            cuts = cuts
                + probe_socket_cut(&format!("{slot_name}_storage_socket")).translate(x, y, 5.0)
                + centered_cube(
                    name(&format!("{slot_name}_robot_pick_window")),
                    23.0,
                    PROBE_SLOT_Y,
                    24.0,
                )
                .translate(x, y, STORAGE_BANK_Z / 2.0 - 7.0)
                + centered_cylinder(
                    name(&format!("{slot_name}_tip_well")),
                    PROBE_TIP_WELL_D / 2.0,
                    if state == 2 {
                        WET_BUFFER_DEPTH_MM
                    } else {
                        22.0
                    },
                    32,
                )
                .translate(
                    x + 20.0,
                    y,
                    STORAGE_BANK_Z / 2.0
                        - if state == 2 {
                            WET_BUFFER_DEPTH_MM / 2.0
                        } else {
                            11.0
                        }
                        + 2.0,
                );

            labels = labels
                + centered_cube(
                    name(&format!("{slot_name}_identity_label_land")),
                    42.0,
                    8.0,
                    3.0,
                )
                .translate(x, y - 28.0, STORAGE_BANK_Z / 2.0 + 1.5);
        }
    }

    let mut dry_channels = Part::empty(name("dry_lane_desiccant_channel_cuts"));
    for i in 0..DRY_DESICCANT_CHANNELS {
        dry_channels = dry_channels
            + centered_cube(
                name(&format!("dry_lane_desiccant_air_channel_{i}")),
                STORAGE_BANK_X - 60.0,
                4.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(1, STORAGE_STATES, STATE_LANE_PITCH_Y)
                    + centered_index(i, DRY_DESICCANT_CHANNELS, 10.0),
                -12.0,
            );
    }

    let mut clean_caps = Part::empty(name("clean_lane_cap_lands"));
    for i in 0..CLEAN_CAP_LANDS {
        clean_caps = clean_caps
            + centered_cube(name(&format!("clean_lane_cap_land_{i}")), 34.0, 7.0, 8.0).translate(
                centered_index(i, CLEAN_CAP_LANDS, PROBE_PITCH_X),
                centered_index(0, STORAGE_STATES, STATE_LANE_PITCH_Y) + 38.0,
                STORAGE_BANK_Z / 2.0 + 4.0,
            );
    }

    body + rear_cable_fence + front_state_lip + state_features + labels + clean_caps
        - cuts
        - dry_channels
        + gripper_fiducials("storage_bank", 210.0)
}

fn sterile_rehydration_path() -> Part {
    let body = centered_cube(
        name("sterile_rehydration_path_body"),
        REHYDRATION_X,
        REHYDRATION_Y,
        REHYDRATION_Z,
    );
    let splash_wall = centered_cube(
        name("sterile_rehydration_path_splash_wall"),
        REHYDRATION_X,
        14.0,
        REHYDRATION_Z + 42.0,
    )
    .translate(0.0, REHYDRATION_Y / 2.0 - 7.0, 21.0);
    let sterile_filter_boss = centered_cylinder(
        name("sterile_rehydration_filter_boss"),
        STERILE_FILTER_D / 2.0,
        30.0,
        44,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-REHYDRATION_X / 2.0 + 52.0, 0.0, REHYDRATION_Z / 2.0 + 15.0);

    let mut cuts = Part::empty(name("sterile_rehydration_path_cuts"));
    let mut connectors = Part::empty(name("sterile_rehydration_path_connectors"));
    let mut ribs = Part::empty(name("sterile_rehydration_flow_arrow_ribs"));

    for i in 0..STERILE_BAG_PORTS {
        connectors = connectors
            + centered_cylinder(
                name(&format!("sterile_bag_port_luer_guard_{i}")),
                17.0,
                18.0,
                32,
            )
            .translate(
                -REHYDRATION_X / 2.0 + 48.0,
                centered_index(i, STERILE_BAG_PORTS, 42.0),
                REHYDRATION_Z / 2.0 + 9.0,
            );
        cuts = cuts
            + centered_cylinder(
                name(&format!("sterile_bag_port_bore_{i}")),
                PATH_BORE_D / 2.0,
                50.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                -REHYDRATION_X / 2.0 + 48.0,
                centered_index(i, STERILE_BAG_PORTS, 42.0),
                REHYDRATION_Z / 2.0,
            );
    }

    for i in 0..HYDRATION_CHIMNEYS {
        let x = centered_index(i, HYDRATION_CHIMNEYS, 38.0) + 40.0;
        cuts = cuts
            + centered_cylinder(
                name(&format!("rehydration_probe_chimney_well_{i}")),
                CHIMNEY_D / 2.0,
                CHIMNEY_DEPTH,
                40,
            )
            .translate(x, -34.0, REHYDRATION_Z / 2.0 - CHIMNEY_DEPTH / 2.0 + 2.0)
            + centered_cylinder(
                name(&format!("rehydration_probe_chimney_drain_{i}")),
                PATH_BORE_D / 2.0,
                80.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -34.0, 0.0);
        connectors = connectors
            + centered_cube(
                name(&format!("rehydration_chimney_cap_land_{i}")),
                30.0,
                9.0,
                8.0,
            )
            .translate(x, -66.0, REHYDRATION_Z / 2.0 + 4.0);
    }

    for i in 0..REHYDRATION_FLUSH_BRANCHES {
        ribs = ribs
            + centered_cube(
                name(&format!("closed_flush_branch_visible_rib_{i}")),
                REHYDRATION_X - 90.0,
                4.0,
                6.0,
            )
            .translate(
                32.0,
                centered_index(i, REHYDRATION_FLUSH_BRANCHES, 28.0) + 58.0,
                REHYDRATION_Z / 2.0 + 3.0,
            );
    }

    body + splash_wall + sterile_filter_boss + connectors + ribs - cuts
        + gripper_fiducials("rehydration_path", 150.0)
}

fn cleaning_verification_coupon_bridge() -> Part {
    let body = centered_cube(
        name("cleaning_verification_coupon_bridge_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let rear_backer = centered_cube(
        name("cleaning_coupon_rear_evidence_backer"),
        COUPON_X,
        14.0,
        COUPON_Z + 32.0,
    )
    .translate(0.0, COUPON_Y / 2.0 - 7.0, 16.0);

    let mut cuts = Part::empty(name("cleaning_coupon_slot_cuts"));
    let mut clips = Part::empty(name("cleaning_coupon_retention_clips"));
    for i in 0..COUPON_COUNT {
        let row = i / COUPON_COLS;
        let col = i % COUPON_COLS;
        let x = centered_index(col, COUPON_COLS, 46.0);
        let y = centered_index(row, COUPON_ROWS, 70.0);
        cuts = cuts
            + centered_cube(
                name(&format!("cleaning_verification_coupon_slot_{i}")),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_Z + 4.0,
            )
            .translate(x, y, 5.0);
        clips = clips
            + centered_cube(
                name(&format!("cleaning_coupon_clip_{i}")),
                COUPON_SLOT_X + 10.0,
                5.0,
                12.0,
            )
            .translate(x, y + COUPON_SLOT_Y / 2.0 - 6.0, COUPON_Z / 2.0 + 6.0);
    }

    let mut swab_lands = Part::empty(name("atp_swab_land_geometry"));
    for i in 0..ATP_SWAB_LANDS {
        swab_lands = swab_lands
            + centered_cube(name(&format!("atp_swab_land_{i}")), 36.0, 12.0, 4.0).translate(
                centered_index(i, ATP_SWAB_LANDS, 42.0),
                -112.0,
                COUPON_Z / 2.0 + 2.0,
            );
    }

    let mut residue_cards = Part::empty(name("residue_card_lands"));
    for i in 0..RESIDUE_CARD_LANDS {
        residue_cards = residue_cards
            + centered_cube(name(&format!("residue_card_land_{i}")), 44.0, 18.0, 4.0).translate(
                centered_index(i, RESIDUE_CARD_LANDS, 58.0),
                112.0,
                COUPON_Z / 2.0 + 2.0,
            );
    }

    body + rear_backer + clips + swab_lands + residue_cards - cuts
        + gripper_fiducials("cleaning_coupon_bridge", 128.0)
}

fn carryover_witness_well_plate() -> Part {
    let body = centered_cube(
        name("carryover_witness_well_plate_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut wells = Part::empty(name("carryover_witness_well_cuts"));
    let mut rims = Part::empty(name("carryover_witness_well_rims"));
    for i in 0..TOTAL_WITNESS_WELLS {
        let row = i / WITNESS_COLS;
        let col = i % WITNESS_COLS;
        let x = centered_index(col, WITNESS_COLS, WITNESS_PITCH_X);
        let y = centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y);
        wells = wells
            + centered_cylinder(
                name(&format!("carryover_witness_well_cut_{i}")),
                WITNESS_WELL_D / 2.0,
                WITNESS_Z + 2.0,
                36,
            )
            .translate(x, y, 4.0);
        rims = rims
            + centered_cylinder(
                name(&format!("carryover_witness_well_raised_rim_{i}")),
                WITNESS_WELL_D / 2.0 + 3.0,
                4.0,
                36,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 2.0);
    }

    let mut gradients = Part::empty(name("carryover_dye_gradient_reference_lands"));
    for i in 0..DYE_GRADIENT_LANDS {
        gradients = gradients
            + centered_cube(
                name(&format!("carryover_dye_gradient_land_{i}")),
                34.0 + i as f64 * 2.0,
                8.0,
                4.0,
            )
            .translate(
                WITNESS_X / 2.0 - 64.0,
                centered_index(i, DYE_GRADIENT_LANDS, 18.0),
                WITNESS_Z / 2.0 + 2.0,
            );
    }

    body + rims + gradients - wells + gripper_fiducials("carryover_witness_plate", 220.0)
}

fn barcode_coa_custody_panel() -> Part {
    let panel = centered_cube(
        name("barcode_coa_custody_panel_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty(name("barcode_coa_custody_lands"));
    for i in 0..BARCODE_LANDS {
        let row = i / 8;
        let col = i % 8;
        lands = lands
            + barcode_land(&format!("probe_storage_barcode_land_{i}"), 34.0, 12.0, i).translate(
                centered_index(col, 8, 42.0),
                centered_index(row, 3, 38.0) + 28.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    for i in 0..COA_CARD_LANDS {
        lands = lands
            + centered_cube(
                name(&format!("coa_certificate_card_land_{i}")),
                48.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(i, COA_CARD_LANDS, 54.0),
                -58.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + barcode_land(&format!("run_record_card_land_{i}"), 52.0, 14.0, i + 31).translate(
                centered_index(i, RUN_RECORD_LANDS, 72.0),
                -88.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    let mut tabs = Part::empty(name("custody_state_tabs"));
    for i in 0..CUSTODY_STATE_TABS {
        tabs = tabs
            + centered_cube(name(&format!("custody_state_tab_{i}")), 48.0, 8.0, 10.0).translate(
                centered_index(i, CUSTODY_STATE_TABS, 64.0),
                82.0,
                CUSTODY_Z / 2.0 + 5.0,
            );
    }

    panel + lands + tabs
}

fn release_hold_reject_gate() -> Part {
    let body = centered_cube(
        name("release_hold_reject_gate_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut lane_cuts = Part::empty(name("release_hold_reject_slot_cuts"));
    let mut gate_teeth = Part::empty(name("release_hold_reject_gate_teeth"));
    let lane_names = ["release", "hold", "reject"];
    for lane in 0..DISPOSITION_LANES {
        let x = centered_index(lane, DISPOSITION_LANES, 92.0);
        gate_teeth = gate_teeth
            + centered_cube(
                name(&format!("{}_lane_status_header", lane_names[lane])),
                78.0,
                14.0,
                12.0,
            )
            .translate(x, GATE_Y / 2.0 - 24.0, GATE_Z / 2.0 + 6.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            lane_cuts = lane_cuts
                + centered_cube(
                    name(&format!(
                        "{}_lane_probe_ticket_slot_{slot}",
                        lane_names[lane]
                    )),
                    56.0,
                    16.0,
                    GATE_Z + 4.0,
                )
                .translate(
                    x,
                    centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 28.0) - 10.0,
                    6.0,
                );
        }
        for tooth in 0..GATE_TEETH_PER_LANE {
            gate_teeth = gate_teeth
                + centered_cube(
                    name(&format!("{}_lane_ratchet_tooth_{tooth}", lane_names[lane])),
                    8.0,
                    12.0,
                    10.0,
                )
                .translate(
                    x - 36.0 + tooth as f64 * 14.0,
                    -GATE_Y / 2.0 + 22.0,
                    GATE_Z / 2.0 + 5.0,
                );
        }
    }

    let mut lockout_flags = Part::empty(name("release_hold_reject_lockout_flags"));
    for i in 0..LOCKOUT_FLAG_SLOTS {
        lockout_flags = lockout_flags
            + centered_cube(name(&format!("lockout_flag_slot_{i}")), 20.0, 42.0, 7.0).translate(
                centered_index(i, LOCKOUT_FLAG_SLOTS, 36.0),
                0.0,
                GATE_Z / 2.0 + 3.5,
            );
    }

    body + gate_teeth + lockout_flags - lane_cuts + gripper_fiducials("disposition_gate", 120.0)
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(name("camera_bridge_left_post"), 34.0, CAMERA_Y, CAMERA_Z)
        .translate(-CAMERA_X / 2.0 + 17.0, 0.0, 0.0);
    let right_post = centered_cube(name("camera_bridge_right_post"), 34.0, CAMERA_Y, CAMERA_Z)
        .translate(CAMERA_X / 2.0 - 17.0, 0.0, 0.0);
    let beam = centered_cube(
        name("camera_bridge_overhead_beam"),
        CAMERA_X,
        CAMERA_Y,
        36.0,
    )
    .translate(0.0, 0.0, CAMERA_Z / 2.0 - 18.0);
    let underside_clearance = centered_cube(
        name("camera_bridge_clearance_shadow_cut"),
        CAMERA_X - 110.0,
        CAMERA_Y + 6.0,
        CAMERA_UNDERSIDE_CLEARANCE_Z,
    )
    .translate(
        0.0,
        0.0,
        -CAMERA_Z / 2.0 + CAMERA_UNDERSIDE_CLEARANCE_Z / 2.0,
    );

    let mut pods = Part::empty(name("camera_evidence_pods"));
    for i in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(name(&format!("camera_pod_{i}")), 76.0, 46.0, 28.0).translate(
                centered_index(i, CAMERA_PODS, 245.0),
                0.0,
                CAMERA_Z / 2.0 - 58.0,
            ) - centered_cylinder(name(&format!("camera_lens_bore_{i}")), 12.0, 50.0, 36)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(i, CAMERA_PODS, 245.0),
                    0.0,
                    CAMERA_Z / 2.0 - 58.0,
                );
    }

    let mut lights = Part::empty(name("camera_evidence_light_bars"));
    for i in 0..EVIDENCE_LIGHT_BARS {
        lights = lights
            + centered_cube(name(&format!("evidence_light_bar_{i}")), 82.0, 8.0, 8.0).translate(
                centered_index(i, EVIDENCE_LIGHT_BARS, 126.0),
                -CAMERA_Y / 2.0 + 10.0,
                CAMERA_Z / 2.0 - 34.0,
            );
    }

    let mut targets = Part::empty(name("camera_focus_targets"));
    for i in 0..FOCUS_TARGETS {
        targets = targets
            + fiducial_disc(&format!("camera_focus_target_{i}")).translate(
                centered_index(i, FOCUS_TARGETS, 185.0),
                CAMERA_Y / 2.0 - 12.0,
                CAMERA_Z / 2.0 - 36.0,
            );
    }

    left_post + right_post + beam + pods + lights + targets - underside_clearance
}

fn closed_waste_quarantine_tray() -> Part {
    let tray = centered_cube(
        name("closed_waste_quarantine_tray_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let basin = centered_cube(
        name("closed_waste_quarantine_basin_cut"),
        WASTE_X - 56.0,
        WASTE_Y - 42.0,
        WASTE_Z - 16.0,
    )
    .translate(0.0, 0.0, 8.0);

    let mut cups = Part::empty(name("closed_waste_quarantine_cups"));
    for i in 0..WASTE_QUARANTINE_CUPS {
        cups = cups
            + centered_cylinder(
                name(&format!("closed_waste_quarantine_cup_{i}")),
                WASTE_CUP_D / 2.0,
                WASTE_Z + 4.0,
                36,
            )
            .translate(centered_index(i, WASTE_QUARANTINE_CUPS, 52.0), -22.0, 6.0);
    }

    let mut ports = Part::empty(name("closed_waste_stream_ports"));
    for i in 0..WASTE_STREAM_PORTS {
        ports = ports
            + centered_cylinder(
                name(&format!("waste_stream_closed_connector_port_{i}")),
                8.0,
                40.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, WASTE_STREAM_PORTS, 62.0),
                WASTE_Y / 2.0 - 20.0,
                0.0,
            );
    }

    let mut leak_wells = Part::empty(name("closed_waste_leak_sensor_wells"));
    for i in 0..LEAK_SENSOR_WELLS {
        leak_wells = leak_wells
            + centered_cylinder(
                name(&format!("closed_waste_leak_sensor_well_{i}")),
                10.0,
                18.0,
                28,
            )
            .translate(
                centered_index(i, LEAK_SENSOR_WELLS, 52.0),
                WASTE_Y / 2.0 - 42.0,
                14.0,
            );
    }

    tray - basin - cups - ports - leak_wells
        + centered_cube(
            name("closed_waste_red_quarantine_card_land"),
            WASTE_X - 80.0,
            14.0,
            4.0,
        )
        .translate(0.0, -WASTE_Y / 2.0 + 18.0, WASTE_Z / 2.0 + 2.0)
}

fn state_transfer_token_rail() -> Part {
    let rail = centered_cube(
        name("state_transfer_token_rail_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts = Part::empty(name("state_transfer_token_slot_cuts"));
    let mut step_ticks = Part::empty(name("state_transfer_step_ticks"));
    for i in 0..STATE_TRANSFER_TOKENS {
        let row = i / TOKEN_COLS;
        let col = i % TOKEN_COLS;
        let x = centered_index(col, TOKEN_COLS, 82.0);
        let y = centered_index(row, TOKEN_ROWS, 64.0);
        cuts = cuts
            + centered_cube(
                name(&format!("state_transfer_token_slot_{i}")),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_Z + 2.0,
            )
            .translate(x, y, 5.0);
        for step in 0..STEP_TOKEN_STATES {
            step_ticks = step_ticks
                + centered_cube(
                    name(&format!("state_transfer_token_{i}_step_tick_{step}")),
                    6.0,
                    8.0,
                    5.0,
                )
                .translate(
                    x - TOKEN_SLOT_X / 2.0 + 10.0 + step as f64 * 10.0,
                    y + TOKEN_SLOT_Y / 2.0 + 8.0,
                    TOKEN_Z / 2.0 + 2.5,
                );
        }
    }
    rail + step_ticks - cuts
}

fn lid_seal_chimney_guard() -> Part {
    let left_wall = centered_cube(name("lid_guard_left_wall"), 14.0, LID_Y, LID_Z).translate(
        -LID_X / 2.0 + 7.0,
        0.0,
        0.0,
    );
    let right_wall = centered_cube(name("lid_guard_right_wall"), 14.0, LID_Y, LID_Z).translate(
        LID_X / 2.0 - 7.0,
        0.0,
        0.0,
    );
    let rear_wall = centered_cube(name("lid_guard_rear_wall"), LID_X, 14.0, LID_Z).translate(
        0.0,
        LID_Y / 2.0 - 7.0,
        0.0,
    );
    let roof = centered_cube(name("lid_guard_roof"), LID_X, LID_Y, 16.0).translate(
        0.0,
        0.0,
        LID_Z / 2.0 - 8.0,
    );
    let front_lip = centered_cube(name("lid_guard_front_low_lip"), LID_X, 12.0, 34.0).translate(
        0.0,
        -LID_Y / 2.0 + 6.0,
        -LID_Z / 2.0 + 17.0,
    );

    let mut seal_ribs = Part::empty(name("lid_guard_seal_ribs"));
    for i in 0..LID_SEAL_RIBS {
        seal_ribs = seal_ribs
            + centered_cube(
                name(&format!("lid_guard_seal_compression_rib_{i}")),
                LID_X - 42.0,
                4.0,
                6.0,
            )
            .translate(
                0.0,
                centered_index(i, LID_SEAL_RIBS, 22.0),
                -LID_Z / 2.0 + 3.0,
            );
    }

    let mut chimney_guards = Part::empty(name("lid_guard_chimney_guard_windows"));
    for i in 0..CHIMNEY_GUARDS {
        chimney_guards = chimney_guards
            + centered_cube(
                name(&format!("lid_guard_chimney_guard_window_{i}")),
                42.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(i, CHIMNEY_GUARDS, 58.0),
                12.0,
                LID_Z / 2.0 + 2.0,
            );
    }

    let mut witness_windows = Part::empty(name("lid_guard_seal_witness_windows"));
    for i in 0..SEAL_WITNESS_WINDOWS {
        witness_windows = witness_windows
            + centered_cube(
                name(&format!("lid_guard_seal_witness_window_{i}")),
                46.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, SEAL_WITNESS_WINDOWS, 70.0),
                -46.0,
                LID_Z / 2.0 + 2.0,
            );
    }

    left_wall
        + right_wall
        + rear_wall
        + roof
        + front_lip
        + seal_ribs
        + chimney_guards
        + witness_windows
}

fn robot_service_keepout_gauge() -> Part {
    let outline = centered_cube(
        name("robot_service_keepout_outline_plate"),
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let front_robot = centered_cube(
        name(&format!(
            "front_robot_{ROBOT_FRONT_APPROACH_CLEARANCE_MM:.0}mm_approach_keepout"
        )),
        KEEP_OUT_X - 160.0,
        34.0,
        5.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 52.0, KEEP_OUT_Z / 2.0 + 2.5);
    let rear_service = centered_cube(
        name(&format!(
            "rear_service_{REAR_SERVICE_CLEARANCE_MM:.0}mm_keepout"
        )),
        KEEP_OUT_X - 180.0,
        34.0,
        5.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 52.0, KEEP_OUT_Z / 2.0 + 2.5);
    let left_load = centered_cube(
        name(&format!(
            "left_probe_load_{LEFT_PROBE_LOAD_CLEARANCE_MM:.0}mm_keepout"
        )),
        38.0,
        KEEP_OUT_Y - 170.0,
        5.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 70.0, 0.0, KEEP_OUT_Z / 2.0 + 2.5);
    let right_waste = centered_cube(
        name(&format!(
            "right_waste_service_{RIGHT_WASTE_SERVICE_CLEARANCE_MM:.0}mm_keepout"
        )),
        38.0,
        KEEP_OUT_Y - 170.0,
        5.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 70.0, 0.0, KEEP_OUT_Z / 2.0 + 2.5);
    let overhead_camera = centered_cube(
        name(&format!(
            "overhead_camera_{OVERHEAD_CAMERA_SERVICE_CLEARANCE_MM:.0}mm_service_keepout"
        )),
        CAMERA_X,
        CAMERA_Y + 72.0,
        5.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1, KEEP_OUT_Z / 2.0 + 2.5);

    outline + front_robot + rear_service + left_load + right_waste + overhead_camera
}

fn probe_socket_cut(label: &str) -> Part {
    centered_cylinder(name(label), PROBE_SLOT_D / 2.0, PROBE_SLOT_Y + 8.0, 36)
        .rotate(90.0, 0.0, 0.0)
}

fn barcode_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(name(label), x, y, 2.0);
    let mut bars = Part::empty(name(&format!("{label}_raised_bars")));
    for bit in 0..7 {
        let bar_h = if (code + bit) % 3 == 0 {
            y - 3.0
        } else {
            (y - 5.0) / 2.0
        };
        bars = bars
            + centered_cube(name(&format!("{label}_bar_{bit}")), 2.0, bar_h, 2.2).translate(
                -x / 2.0 + 7.0 + bit as f64 * 4.0,
                0.0,
                2.1,
            );
    }
    land + bars
}

fn fiducial_disc(label: &str) -> Part {
    let outer = centered_cylinder(name(&format!("{label}_outer_disc")), 11.0, 3.0, 36);
    let center = centered_cylinder(name(&format!("{label}_center_dot")), 3.0, 4.0, 24);
    let cross_x = centered_cube(name(&format!("{label}_cross_x")), 24.0, 2.0, 4.0);
    let cross_y = centered_cube(name(&format!("{label}_cross_y")), 2.0, 24.0, 4.0);
    outer + center + cross_x + cross_y
}

fn gripper_fiducials(prefix: &str, span_x: f64) -> Part {
    fiducial_disc(&format!("{prefix}_left_fiducial")).translate(-span_x / 2.0, 0.0, 2.0)
        + fiducial_disc(&format!("{prefix}_right_fiducial")).translate(span_x / 2.0, 0.0, 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_scoped_and_deterministic() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
        assert!(!USES_RANDOMNESS);
        assert_eq!(RANDOM_SEED, 0);
    }

    #[test]
    fn requested_feature_groups_are_declared() {
        for feature in [
            "clean_storage_state",
            "dry_storage_state",
            "wet_storage_state",
            "sterile_rehydration_path",
            "cleaning_verification_coupons",
            "carryover_witness_wells",
            "barcode_coa_custody",
            "release_hold_reject_gates",
            "camera_evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn storage_and_rehydration_counts_match_probe_set() {
        assert_eq!(PROBE_SLOTS, PROBE_TYPES * STORAGE_STATES);
        assert_eq!(HYDRATION_CHIMNEYS, PROBE_TYPES);
        assert_eq!(CLEAN_CAP_LANDS, PROBE_TYPES);
        assert_eq!(STATE_TRANSFER_TOKENS, PROBE_TYPES);
        assert_eq!(PROBE_NAMES.len(), PROBE_TYPES);
    }

    #[test]
    fn witness_coupon_and_custody_counts_are_complete() {
        assert_eq!(COUPON_COUNT, COUPON_ROWS * COUPON_COLS);
        assert_eq!(TOTAL_WITNESS_WELLS, WITNESS_ROWS * WITNESS_COLS);
        assert_eq!(BARCODE_LANDS, PROBE_SLOTS);
        assert!(COA_CARD_LANDS >= STORAGE_STATES * 2);
        assert_eq!(DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE, 15);
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
        let modules = module_footprints();
        for module in modules {
            assert!(
                module.fits_inside_station(),
                "{} is outside the station deck",
                module.name
            );
        }
        for (index, a) in modules.iter().enumerate() {
            for b in modules.iter().skip(index + 1) {
                assert!(
                    !a.overlaps(*b, MODULE_GAP_MM),
                    "{} overlaps {}",
                    a.name,
                    b.name
                );
            }
        }
    }

    #[test]
    fn closed_waste_and_camera_clearances_cover_challenge_flow() {
        assert!(closed_waste_capacity_ml() > rehydration_challenge_volume_ml());
        assert!(CAMERA_UNDERSIDE_CLEARANCE_Z > STORAGE_BANK_Z + 80.0);
        assert!(ROBOT_FRONT_APPROACH_CLEARANCE_MM > 400.0);
        assert!(REAR_SERVICE_CLEARANCE_MM > 250.0);
    }
}
