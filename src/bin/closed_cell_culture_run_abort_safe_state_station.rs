use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-culture run abort safe-state station.
//
// Mechanical validation fixture for making an aborted closed culture run visible
// without opening the sterile boundary. It packages emergency-stop evidence,
// pump/valve closed-state tokens, waste diversion, cassette quarantine,
// retained sample split, alarm logging, pressure relief/vent status, backflow
// witnesses, custody lands, disposition gates, camera evidence, and robot/service
// keepouts. It is not a biological SOP, emergency procedure, or pressure-rated
// device.

const OUTPUT_PREFIX: &str = "closed_cell_culture_run_abort_safe_state_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_culture_run_abort_safe_state_station_containment_deck.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_emergency_stop_guard.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_pump_valve_state_token_panel.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_waste_diverter_manifold_surrogate.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_cassette_quarantine_dock.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_retained_sample_vial_nest.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_alarm_event_logger_pocket.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_pressure_relief_vent_holder.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_fluid_backflow_witness.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_barcode_rfid_custody_lands.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_release_hold_reject_gates.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_evidence_camera_bridge_and_keepouts.stl",
    "output/closed_cell_culture_run_abort_safe_state_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "containment_deck",
    "emergency_stop_guard",
    "pump_valve_state_token_panel",
    "waste_diverter_manifold_surrogate",
    "cassette_quarantine_dock",
    "retained_sample_vial_nest",
    "alarm_event_logger_pocket",
    "pressure_relief_vent_holder",
    "fluid_backflow_witness",
    "barcode_rfid_custody_lands",
    "release_hold_reject_gates",
    "evidence_camera_bridge_and_keepouts",
];

const DECK_X: f64 = 1660.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 46.0;
const BASIN_X: f64 = 1450.0;
const BASIN_Y: f64 = 830.0;
const BASIN_DEPTH: f64 = 7.0;
const MOUNT_HOLE_COUNT: usize = 12;
const MOUNT_HOLE_D: f64 = 6.8;
const SPILL_GUTTER_W: f64 = 15.0;
const STERILE_BOUNDARY_POSTS: usize = 6;

const ESTOP_CENTER: (f64, f64) = (-610.0, 350.0);
const ESTOP_PANEL_X: f64 = 250.0;
const ESTOP_PANEL_Y: f64 = 170.0;
const ESTOP_PANEL_Z: f64 = 32.0;
const ESTOP_BUTTON_D: f64 = 76.0;
const ESTOP_GUARD_POSTS: usize = 4;
const ESTOP_TAMPER_SEAL_LANDS: usize = 4;
const ESTOP_WITNESS_FLAGS: usize = 3;

const TOKEN_CENTER: (f64, f64) = (-250.0, 310.0);
const TOKEN_PANEL_X: f64 = 470.0;
const TOKEN_PANEL_Y: f64 = 235.0;
const TOKEN_PANEL_Z: f64 = 28.0;
const PUMP_TOKEN_COUNT: usize = 4;
const VALVE_TOKEN_COUNT: usize = 8;
const TOKEN_COUNT: usize = PUMP_TOKEN_COUNT + VALVE_TOKEN_COUNT;
const TOKEN_SLOT_X: f64 = 44.0;
const TOKEN_SLOT_Y: f64 = 34.0;
const TOKEN_PITCH_X: f64 = 62.0;
const TOKEN_PITCH_Y: f64 = 72.0;

const WASTE_CENTER: (f64, f64) = (300.0, 305.0);
const WASTE_PANEL_X: f64 = 500.0;
const WASTE_PANEL_Y: f64 = 250.0;
const WASTE_PANEL_Z: f64 = 30.0;
const WASTE_BRANCH_COUNT: usize = 4;
const DIVERTER_PORT_D: f64 = 27.0;
const WASTE_LOCKOUT_TABS: usize = 6;
const WASTE_BAG_CLIP_COUNT: usize = 4;

const QUARANTINE_CENTER: (f64, f64) = (-470.0, 30.0);
const QUARANTINE_DOCK_X: f64 = 560.0;
const QUARANTINE_DOCK_Y: f64 = 300.0;
const QUARANTINE_DOCK_Z: f64 = 42.0;
const CASSETTE_SLOT_COUNT: usize = 4;
const CASSETTE_SLOT_X: f64 = REVC_CHIP_LENGTH + 26.0;
const CASSETTE_SLOT_Y: f64 = REVC_CHIP_WIDTH + 26.0;
const CASSETTE_PITCH_X: f64 = 132.0;
const QUARANTINE_LOCK_PINS: usize = 8;
const STERILE_BOUNDARY_SEAL_TABS: usize = 8;

const SAMPLE_CENTER: (f64, f64) = (250.0, 30.0);
const SAMPLE_PANEL_X: f64 = 430.0;
const SAMPLE_PANEL_Y: f64 = 285.0;
const SAMPLE_PANEL_Z: f64 = 30.0;
const RETAINED_SAMPLE_VIALS: usize = 6;
const VIAL_NEST_D: f64 = 25.0;
const VIAL_PITCH_X: f64 = 58.0;
const VIAL_PITCH_Y: f64 = 80.0;
const SPLIT_CHANNELS: usize = 3;
const SPLIT_CHANNEL_W: f64 = 12.0;

const LOGGER_CENTER: (f64, f64) = (615.0, 20.0);
const LOGGER_POCKET_X: f64 = 260.0;
const LOGGER_POCKET_Y: f64 = 300.0;
const LOGGER_POCKET_Z: f64 = 38.0;
const EVENT_CARD_SLOTS: usize = 5;
const LOGGER_CABLE_STRAIN_RELIEFS: usize = 4;

const VENT_CENTER: (f64, f64) = (-595.0, -250.0);
const VENT_PANEL_X: f64 = 280.0;
const VENT_PANEL_Y: f64 = 220.0;
const VENT_PANEL_Z: f64 = 30.0;
const RELIEF_HOLDER_COUNT: usize = 3;
const VENT_FILTER_PUCKS: usize = 3;
const VENT_STATUS_FLAGS: usize = 4;

const BACKFLOW_CENTER: (f64, f64) = (-190.0, -285.0);
const BACKFLOW_PANEL_X: f64 = 430.0;
const BACKFLOW_PANEL_Y: f64 = 210.0;
const BACKFLOW_PANEL_Z: f64 = 25.0;
const BACKFLOW_WITNESS_WINDOWS: usize = 8;
const BACKFLOW_LANE_COUNT: usize = 4;

const CUSTODY_CENTER: (f64, f64) = (260.0, -285.0);
const CUSTODY_PANEL_X: f64 = 390.0;
const CUSTODY_PANEL_Y: f64 = 210.0;
const CUSTODY_PANEL_Z: f64 = 20.0;
const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 6;
const CUSTODY_TIMESTAMP_TICKS: usize = 8;

const GATE_CENTER: (f64, f64) = (630.0, -285.0);
const GATE_PANEL_X: f64 = 250.0;
const GATE_PANEL_Y: f64 = 210.0;
const GATE_PANEL_Z: f64 = 28.0;
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_COUNT: usize = 9;
const GATE_SLOT_X: f64 = 62.0;
const GATE_SLOT_Y: f64 = 42.0;

const BRIDGE_SPAN_X: f64 = 1510.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 54.0;
const BRIDGE_BEAM_Y: f64 = 70.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_UNDERSIDE_Z: f64 = 235.0;
const CAMERA_PODS: usize = 4;
const CAMERA_POD_X: f64 = 92.0;
const CAMERA_POD_Y: f64 = 60.0;
const CAMERA_POD_Z: f64 = 44.0;
const CAMERA_PITCH_X: f64 = 315.0;
const EVIDENCE_LIGHT_BARS: usize = 6;

const KEEP_OUT_RAIL_Z: f64 = 6.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 420.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 260.0;
const LEFT_QUARANTINE_KEEP_OUT_X: f64 = 250.0;
const RIGHT_LOGGER_KEEP_OUT_X: f64 = 190.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 18.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 18.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let estop = emergency_stop_guard();
    export(OUTPUTS[1], &estop);

    let tokens = pump_valve_state_token_panel();
    export(OUTPUTS[2], &tokens);

    let waste = waste_diverter_manifold_surrogate();
    export(OUTPUTS[3], &waste);

    let quarantine = cassette_quarantine_dock();
    export(OUTPUTS[4], &quarantine);

    let sample = retained_sample_vial_nest();
    export(OUTPUTS[5], &sample);

    let logger = alarm_event_logger_pocket();
    export(OUTPUTS[6], &logger);

    let vent = pressure_relief_vent_holder();
    export(OUTPUTS[7], &vent);

    let backflow = fluid_backflow_witness();
    export(OUTPUTS[8], &backflow);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[9], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[10], &gates);

    let bridge = evidence_camera_bridge_and_keepouts();
    export(OUTPUTS[11], &bridge);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cell-culture run abort safe-state station:");
    println!("  Deck:                    {DECK_X:.0}mm x {DECK_Y:.0}mm containment deck");
    println!(
        "  Abort evidence:          {ESTOP_WITNESS_FLAGS} e-stop witness flags, {TOKEN_COUNT} pump/valve closed-state token slots"
    );
    println!(
        "  Closed fluid state:      {WASTE_BRANCH_COUNT} waste-diverter branches, {BACKFLOW_WITNESS_WINDOWS} backflow witness windows, {RELIEF_HOLDER_COUNT} relief/vent holders"
    );
    println!(
        "  Quarantine/sample split: {CASSETTE_SLOT_COUNT} cassette quarantine slots, {RETAINED_SAMPLE_VIALS} retained sample vial nests, {SPLIT_CHANNELS} split-channel traces"
    );
    println!(
        "  Audit/custody:           {EVENT_CARD_SLOTS} alarm event card slots, {} custody lands/ticks, {DISPOSITION_GATES} release/hold/reject gates",
        audit_land_count()
    );
    println!(
        "  Evidence capture:        {CAMERA_PODS} camera pods on bridge, underside {:.0}mm above deck, robot/service keepouts included",
        evidence_bridge_clearance_above_deck()
    );
    println!("  Labeled STL outputs:     {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + emergency_stop_guard()
        + pump_valve_state_token_panel()
        + waste_diverter_manifold_surrogate()
        + cassette_quarantine_dock()
        + retained_sample_vial_nest()
        + alarm_event_logger_pocket()
        + pressure_relief_vent_holder()
        + fluid_backflow_witness()
        + barcode_rfid_custody_lands()
        + release_hold_reject_gates()
        + evidence_camera_bridge_and_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "abort_safe_state_containment_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - containment_basin_relief()
        + deck_perimeter_rim()
        + spill_gutter_lands()
        + sterile_boundary_posts()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("abort_safe_state_deck_mount_holes");
    for (index, (x, y)) in mount_hole_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("abort_safe_state_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 8.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn containment_basin_relief() -> Part {
    centered_cube(
        "abort_safe_state_shallow_containment_basin_relief",
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.2)
}

fn deck_perimeter_rim() -> Part {
    let z = DECK_Z + RIM_Z / 2.0;
    let front = centered_cube("abort_safe_state_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        z,
    );
    let rear = centered_cube("abort_safe_state_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        z,
    );
    let left = centered_cube(
        "abort_safe_state_left_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "abort_safe_state_right_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn spill_gutter_lands() -> Part {
    let z = DECK_Z + 2.5;
    let mut gutters = Part::empty("abort_safe_state_spill_gutter_lands");
    for (index, y) in [-430.0, 430.0].into_iter().enumerate() {
        gutters = gutters
            + centered_cube(
                format!("abort_safe_state_long_spill_gutter_land_{index}"),
                BASIN_X,
                SPILL_GUTTER_W,
                5.0,
            )
            .translate(0.0, y, z);
    }
    for (index, x) in [-740.0, 740.0].into_iter().enumerate() {
        gutters = gutters
            + centered_cube(
                format!("abort_safe_state_short_spill_gutter_land_{index}"),
                SPILL_GUTTER_W,
                BASIN_Y,
                5.0,
            )
            .translate(x, 0.0, z);
    }
    gutters
}

fn sterile_boundary_posts() -> Part {
    let mut posts = Part::empty("abort_safe_state_visible_sterile_boundary_posts");
    for (index, (x, y)) in sterile_boundary_post_points().into_iter().enumerate() {
        posts = posts
            + centered_cylinder(
                format!("abort_safe_state_sterile_boundary_status_post_{index}"),
                13.0,
                38.0,
                32,
            )
            .translate(x, y, DECK_Z + 19.0);
    }
    posts
}

fn emergency_stop_guard() -> Part {
    let base = centered_cube(
        "abort_safe_state_estop_evidence_panel",
        ESTOP_PANEL_X,
        ESTOP_PANEL_Y,
        ESTOP_PANEL_Z,
    )
    .translate(ESTOP_CENTER.0, ESTOP_CENTER.1, DECK_Z + ESTOP_PANEL_Z / 2.0);
    let button_well = centered_cylinder(
        "abort_safe_state_estop_button_visible_well",
        ESTOP_BUTTON_D / 2.0,
        ESTOP_PANEL_Z + 8.0,
        48,
    )
    .translate(
        ESTOP_CENTER.0,
        ESTOP_CENTER.1 + 20.0,
        DECK_Z + ESTOP_PANEL_Z / 2.0,
    );
    let guard = estop_guard_posts() + estop_tamper_seal_lands() + estop_witness_flags();

    base - button_well + guard
}

fn estop_guard_posts() -> Part {
    let mut posts = Part::empty("abort_safe_state_estop_guard_posts");
    for (index, (dx, dy)) in [(-82.0, -48.0), (82.0, -48.0), (-82.0, 78.0), (82.0, 78.0)]
        .into_iter()
        .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("abort_safe_state_estop_guard_post_{index}"),
                11.0,
                70.0,
                32,
            )
            .translate(
                ESTOP_CENTER.0 + dx,
                ESTOP_CENTER.1 + dy,
                DECK_Z + ESTOP_PANEL_Z + 35.0,
            );
    }
    posts
}

fn estop_tamper_seal_lands() -> Part {
    let mut lands = Part::empty("abort_safe_state_estop_tamper_seal_lands");
    for index in 0..ESTOP_TAMPER_SEAL_LANDS {
        let x = ESTOP_CENTER.0 - 90.0 + index as f64 * 60.0;
        lands = lands
            + centered_cube(
                format!("abort_safe_state_estop_tamper_seal_land_{index}"),
                38.0,
                14.0,
                5.0,
            )
            .translate(x, ESTOP_CENTER.1 - 72.0, DECK_Z + ESTOP_PANEL_Z + 2.5);
    }
    lands
}

fn estop_witness_flags() -> Part {
    let mut flags = Part::empty("abort_safe_state_estop_witness_flags");
    for index in 0..ESTOP_WITNESS_FLAGS {
        flags = flags
            + centered_cube(
                format!("abort_safe_state_estop_witness_flag_{index}"),
                22.0,
                52.0,
                7.0,
            )
            .translate(
                ESTOP_CENTER.0 - 55.0 + index as f64 * 55.0,
                ESTOP_CENTER.1 + 74.0,
                DECK_Z + ESTOP_PANEL_Z + 3.5,
            );
    }
    flags
}

fn pump_valve_state_token_panel() -> Part {
    let panel = centered_cube(
        "abort_safe_state_pump_valve_state_token_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z + TOKEN_PANEL_Z / 2.0);

    panel - token_slot_cutouts() + token_closed_markers() + token_row_labels()
}

fn token_slot_cutouts() -> Part {
    let mut slots = Part::empty("abort_safe_state_pump_valve_token_slots");
    for index in 0..TOKEN_COUNT {
        let (x, y) = token_position(index);
        slots = slots
            + centered_cube(
                format!("abort_safe_state_token_slot_{index}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_PANEL_Z + 6.0,
            )
            .translate(x, y, DECK_Z + TOKEN_PANEL_Z / 2.0 + 1.0);
    }
    slots
}

fn token_closed_markers() -> Part {
    let mut markers = Part::empty("abort_safe_state_token_closed_markers");
    for index in 0..TOKEN_COUNT {
        let (x, y) = token_position(index);
        markers = markers
            + centered_cube(
                format!("abort_safe_state_closed_state_marker_{index}"),
                18.0,
                10.0,
                6.0,
            )
            .translate(x, y + 24.0, DECK_Z + TOKEN_PANEL_Z + 3.0);
    }
    markers
}

fn token_row_labels() -> Part {
    centered_cube("abort_safe_state_pump_row_label_land", 156.0, 16.0, 4.0).translate(
        TOKEN_CENTER.0 - 150.0,
        TOKEN_CENTER.1 + 86.0,
        DECK_Z + TOKEN_PANEL_Z + 2.0,
    ) + centered_cube("abort_safe_state_valve_row_label_land", 260.0, 16.0, 4.0).translate(
        TOKEN_CENTER.0 + 75.0,
        TOKEN_CENTER.1 - 86.0,
        DECK_Z + TOKEN_PANEL_Z + 2.0,
    )
}

fn waste_diverter_manifold_surrogate() -> Part {
    let panel = centered_cube(
        "abort_safe_state_waste_diverter_panel",
        WASTE_PANEL_X,
        WASTE_PANEL_Y,
        WASTE_PANEL_Z,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_PANEL_Z / 2.0);
    let header = centered_cube(
        "abort_safe_state_waste_diverter_header_surrogate",
        360.0,
        28.0,
        32.0,
    )
    .translate(
        WASTE_CENTER.0,
        WASTE_CENTER.1 + 42.0,
        DECK_Z + WASTE_PANEL_Z + 16.0,
    );

    panel + header + waste_branch_ports() + waste_lockout_tabs() + waste_bag_clips()
}

fn waste_branch_ports() -> Part {
    let mut ports = Part::empty("abort_safe_state_waste_diverter_branch_ports");
    for index in 0..WASTE_BRANCH_COUNT {
        let x = WASTE_CENTER.0 - 150.0 + index as f64 * 100.0;
        ports = ports
            + centered_cylinder(
                format!("abort_safe_state_waste_diverter_port_{index}"),
                DIVERTER_PORT_D / 2.0,
                34.0,
                32,
            )
            .translate(x, WASTE_CENTER.1 + 42.0, DECK_Z + WASTE_PANEL_Z + 43.0)
            + centered_cube(
                format!("abort_safe_state_diverter_arrow_land_{index}"),
                58.0,
                12.0,
                5.0,
            )
            .translate(x, WASTE_CENTER.1 - 36.0, DECK_Z + WASTE_PANEL_Z + 2.5);
    }
    ports
}

fn waste_lockout_tabs() -> Part {
    let mut tabs = Part::empty("abort_safe_state_waste_lockout_tabs");
    for index in 0..WASTE_LOCKOUT_TABS {
        let x = WASTE_CENTER.0 - 190.0 + index as f64 * 76.0;
        tabs = tabs
            + centered_cube(
                format!("abort_safe_state_waste_lockout_tab_{index}"),
                42.0,
                28.0,
                14.0,
            )
            .translate(x, WASTE_CENTER.1 + 100.0, DECK_Z + WASTE_PANEL_Z + 7.0);
    }
    tabs
}

fn waste_bag_clips() -> Part {
    let mut clips = Part::empty("abort_safe_state_waste_bag_clip_saddles");
    for index in 0..WASTE_BAG_CLIP_COUNT {
        let x = WASTE_CENTER.0 - 150.0 + index as f64 * 100.0;
        clips = clips
            + centered_cube(
                format!("abort_safe_state_waste_bag_clip_saddle_{index}"),
                54.0,
                20.0,
                18.0,
            )
            .translate(x, WASTE_CENTER.1 - 96.0, DECK_Z + WASTE_PANEL_Z + 9.0);
    }
    clips
}

fn cassette_quarantine_dock() -> Part {
    let dock = centered_cube(
        "abort_safe_state_cassette_quarantine_dock",
        QUARANTINE_DOCK_X,
        QUARANTINE_DOCK_Y,
        QUARANTINE_DOCK_Z,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        DECK_Z + QUARANTINE_DOCK_Z / 2.0,
    );

    dock - cassette_slot_recesses() + quarantine_lock_pins() + sterile_boundary_seal_tabs()
}

fn cassette_slot_recesses() -> Part {
    let mut slots = Part::empty("abort_safe_state_cassette_quarantine_recesses");
    for index in 0..CASSETTE_SLOT_COUNT {
        let x = QUARANTINE_CENTER.0 - 1.5 * CASSETTE_PITCH_X + index as f64 * CASSETTE_PITCH_X;
        slots = slots
            + centered_cube(
                format!("abort_safe_state_cassette_quarantine_slot_{index}"),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                REVC_TOTAL_HEIGHT + 16.0,
            )
            .translate(x, QUARANTINE_CENTER.1, DECK_Z + QUARANTINE_DOCK_Z - 8.0);
    }
    slots
}

fn quarantine_lock_pins() -> Part {
    let mut pins = Part::empty("abort_safe_state_quarantine_lock_pins");
    for index in 0..QUARANTINE_LOCK_PINS {
        let side = if index < 4 { -1.0 } else { 1.0 };
        let local = index % 4;
        let x = QUARANTINE_CENTER.0 - 1.5 * CASSETTE_PITCH_X + local as f64 * CASSETTE_PITCH_X;
        pins = pins
            + centered_cylinder(
                format!("abort_safe_state_quarantine_lock_pin_{index}"),
                6.5,
                28.0,
                24,
            )
            .translate(
                x,
                QUARANTINE_CENTER.1 + side * 116.0,
                DECK_Z + QUARANTINE_DOCK_Z + 14.0,
            );
    }
    pins
}

fn sterile_boundary_seal_tabs() -> Part {
    let mut tabs = Part::empty("abort_safe_state_sterile_boundary_seal_tabs");
    for index in 0..STERILE_BOUNDARY_SEAL_TABS {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let column = index / 2;
        tabs = tabs
            + centered_cube(
                format!("abort_safe_state_sterile_boundary_seal_tab_{index}"),
                38.0,
                18.0,
                8.0,
            )
            .translate(
                QUARANTINE_CENTER.0 - 198.0 + column as f64 * 132.0,
                QUARANTINE_CENTER.1 + side * 146.0,
                DECK_Z + QUARANTINE_DOCK_Z + 4.0,
            );
    }
    tabs
}

fn retained_sample_vial_nest() -> Part {
    let panel = centered_cube(
        "abort_safe_state_retained_sample_split_panel",
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Y,
        SAMPLE_PANEL_Z,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1,
        DECK_Z + SAMPLE_PANEL_Z / 2.0,
    );

    panel - vial_nest_cutouts() + split_channel_traces() + sample_seal_lands()
}

fn vial_nest_cutouts() -> Part {
    let mut cutouts = Part::empty("abort_safe_state_retained_sample_vial_nest_cutouts");
    for index in 0..RETAINED_SAMPLE_VIALS {
        let col = index % 3;
        let row = index / 3;
        cutouts = cutouts
            + centered_cylinder(
                format!("abort_safe_state_retained_sample_vial_cutout_{index}"),
                VIAL_NEST_D / 2.0,
                SAMPLE_PANEL_Z + 8.0,
                32,
            )
            .translate(
                SAMPLE_CENTER.0 - VIAL_PITCH_X + col as f64 * VIAL_PITCH_X,
                SAMPLE_CENTER.1 - VIAL_PITCH_Y / 2.0 + row as f64 * VIAL_PITCH_Y,
                DECK_Z + SAMPLE_PANEL_Z / 2.0 + 1.0,
            );
    }
    cutouts
}

fn split_channel_traces() -> Part {
    let mut traces = Part::empty("abort_safe_state_retained_sample_split_channel_traces");
    for index in 0..SPLIT_CHANNELS {
        traces = traces
            + centered_cube(
                format!("abort_safe_state_retained_sample_split_channel_{index}"),
                280.0,
                SPLIT_CHANNEL_W,
                5.0,
            )
            .translate(
                SAMPLE_CENTER.0,
                SAMPLE_CENTER.1 - 95.0 + index as f64 * 95.0,
                DECK_Z + SAMPLE_PANEL_Z + 2.5,
            );
    }
    traces
}

fn sample_seal_lands() -> Part {
    let mut lands = Part::empty("abort_safe_state_retained_sample_seal_lands");
    for index in 0..RETAINED_SAMPLE_VIALS {
        let col = index % 3;
        let row = index / 3;
        lands = lands
            + centered_cube(
                format!("abort_safe_state_retained_sample_barcode_seal_land_{index}"),
                48.0,
                14.0,
                4.0,
            )
            .translate(
                SAMPLE_CENTER.0 - VIAL_PITCH_X + col as f64 * VIAL_PITCH_X,
                SAMPLE_CENTER.1 - VIAL_PITCH_Y / 2.0 + row as f64 * VIAL_PITCH_Y + 32.0,
                DECK_Z + SAMPLE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn alarm_event_logger_pocket() -> Part {
    let pocket = centered_cube(
        "abort_safe_state_alarm_event_logger_pocket_body",
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        LOGGER_POCKET_Z,
    )
    .translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1,
        DECK_Z + LOGGER_POCKET_Z / 2.0,
    );

    pocket - logger_device_recess() + event_card_slots() + logger_cable_strain_reliefs()
}

fn logger_device_recess() -> Part {
    centered_cube(
        "abort_safe_state_alarm_logger_device_recess",
        LOGGER_POCKET_X - 52.0,
        LOGGER_POCKET_Y - 70.0,
        LOGGER_POCKET_Z + 8.0,
    )
    .translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1,
        DECK_Z + LOGGER_POCKET_Z - 8.0,
    )
}

fn event_card_slots() -> Part {
    let mut slots = Part::empty("abort_safe_state_alarm_event_card_slots");
    for index in 0..EVENT_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("abort_safe_state_alarm_event_card_slot_{index}"),
                150.0,
                10.0,
                14.0,
            )
            .translate(
                LOGGER_CENTER.0,
                LOGGER_CENTER.1 - 110.0 + index as f64 * 55.0,
                DECK_Z + LOGGER_POCKET_Z + 7.0,
            );
    }
    slots
}

fn logger_cable_strain_reliefs() -> Part {
    let mut reliefs = Part::empty("abort_safe_state_logger_cable_strain_reliefs");
    for index in 0..LOGGER_CABLE_STRAIN_RELIEFS {
        reliefs = reliefs
            + centered_cube(
                format!("abort_safe_state_logger_cable_strain_relief_{index}"),
                42.0,
                16.0,
                16.0,
            )
            .translate(
                LOGGER_CENTER.0 - 78.0 + index as f64 * 52.0,
                LOGGER_CENTER.1 + 138.0,
                DECK_Z + LOGGER_POCKET_Z + 8.0,
            );
    }
    reliefs
}

fn pressure_relief_vent_holder() -> Part {
    let panel = centered_cube(
        "abort_safe_state_pressure_relief_vent_holder_panel",
        VENT_PANEL_X,
        VENT_PANEL_Y,
        VENT_PANEL_Z,
    )
    .translate(VENT_CENTER.0, VENT_CENTER.1, DECK_Z + VENT_PANEL_Z / 2.0);

    panel - relief_holder_cutouts() + vent_filter_pucks() + vent_status_flags()
}

fn relief_holder_cutouts() -> Part {
    let mut cutouts = Part::empty("abort_safe_state_relief_holder_cutouts");
    for index in 0..RELIEF_HOLDER_COUNT {
        cutouts = cutouts
            + centered_cylinder(
                format!("abort_safe_state_relief_holder_cutout_{index}"),
                25.0,
                VENT_PANEL_Z + 8.0,
                40,
            )
            .translate(
                VENT_CENTER.0 - 72.0 + index as f64 * 72.0,
                VENT_CENTER.1 + 25.0,
                DECK_Z + VENT_PANEL_Z / 2.0 + 1.0,
            );
    }
    cutouts
}

fn vent_filter_pucks() -> Part {
    let mut pucks = Part::empty("abort_safe_state_vent_filter_puck_lands");
    for index in 0..VENT_FILTER_PUCKS {
        pucks = pucks
            + centered_cylinder(
                format!("abort_safe_state_vent_filter_puck_land_{index}"),
                33.0,
                8.0,
                40,
            )
            .translate(
                VENT_CENTER.0 - 72.0 + index as f64 * 72.0,
                VENT_CENTER.1 - 60.0,
                DECK_Z + VENT_PANEL_Z + 4.0,
            );
    }
    pucks
}

fn vent_status_flags() -> Part {
    let mut flags = Part::empty("abort_safe_state_vent_status_flags");
    for index in 0..VENT_STATUS_FLAGS {
        flags = flags
            + centered_cube(
                format!("abort_safe_state_vent_status_flag_{index}"),
                42.0,
                15.0,
                6.0,
            )
            .translate(
                VENT_CENTER.0 - 87.0 + index as f64 * 58.0,
                VENT_CENTER.1 + 92.0,
                DECK_Z + VENT_PANEL_Z + 3.0,
            );
    }
    flags
}

fn fluid_backflow_witness() -> Part {
    let panel = centered_cube(
        "abort_safe_state_backflow_witness_panel",
        BACKFLOW_PANEL_X,
        BACKFLOW_PANEL_Y,
        BACKFLOW_PANEL_Z,
    )
    .translate(
        BACKFLOW_CENTER.0,
        BACKFLOW_CENTER.1,
        DECK_Z + BACKFLOW_PANEL_Z / 2.0,
    );

    panel - backflow_window_cutouts() + backflow_lane_traces() + backflow_check_datum_blocks()
}

fn backflow_window_cutouts() -> Part {
    let mut cutouts = Part::empty("abort_safe_state_backflow_witness_window_cutouts");
    for index in 0..BACKFLOW_WITNESS_WINDOWS {
        let col = index % 4;
        let row = index / 4;
        cutouts = cutouts
            + centered_cube(
                format!("abort_safe_state_backflow_window_cutout_{index}"),
                54.0,
                26.0,
                BACKFLOW_PANEL_Z + 8.0,
            )
            .translate(
                BACKFLOW_CENTER.0 - 120.0 + col as f64 * 80.0,
                BACKFLOW_CENTER.1 - 42.0 + row as f64 * 84.0,
                DECK_Z + BACKFLOW_PANEL_Z / 2.0 + 1.0,
            );
    }
    cutouts
}

fn backflow_lane_traces() -> Part {
    let mut traces = Part::empty("abort_safe_state_backflow_lane_traces");
    for index in 0..BACKFLOW_LANE_COUNT {
        traces = traces
            + centered_cube(
                format!("abort_safe_state_backflow_lane_trace_{index}"),
                320.0,
                8.0,
                5.0,
            )
            .translate(
                BACKFLOW_CENTER.0,
                BACKFLOW_CENTER.1 - 78.0 + index as f64 * 52.0,
                DECK_Z + BACKFLOW_PANEL_Z + 2.5,
            );
    }
    traces
}

fn backflow_check_datum_blocks() -> Part {
    centered_cube(
        "abort_safe_state_backflow_upstream_datum_block",
        38.0,
        82.0,
        18.0,
    )
    .translate(
        BACKFLOW_CENTER.0 - 190.0,
        BACKFLOW_CENTER.1,
        DECK_Z + BACKFLOW_PANEL_Z + 9.0,
    ) + centered_cube(
        "abort_safe_state_backflow_downstream_datum_block",
        38.0,
        82.0,
        18.0,
    )
    .translate(
        BACKFLOW_CENTER.0 + 190.0,
        BACKFLOW_CENTER.1,
        DECK_Z + BACKFLOW_PANEL_Z + 9.0,
    )
}

fn barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "abort_safe_state_barcode_rfid_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        DECK_Z + CUSTODY_PANEL_Z / 2.0,
    );

    panel + barcode_lands() + rfid_lands() + custody_timestamp_ticks()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("abort_safe_state_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let col = index % 5;
        let row = index / 5;
        lands = lands
            + centered_cube(
                format!("abort_safe_state_barcode_custody_land_{index}"),
                56.0,
                16.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER.0 - 132.0 + col as f64 * 66.0,
                CUSTODY_CENTER.1 - 55.0 + row as f64 * 48.0,
                DECK_Z + CUSTODY_PANEL_Z + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("abort_safe_state_rfid_lands");
    for index in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("abort_safe_state_rfid_custody_land_{index}"),
                18.0,
                5.0,
                32,
            )
            .translate(
                CUSTODY_CENTER.0 - 125.0 + index as f64 * 50.0,
                CUSTODY_CENTER.1 + 70.0,
                DECK_Z + CUSTODY_PANEL_Z + 2.5,
            );
    }
    lands
}

fn custody_timestamp_ticks() -> Part {
    let mut ticks = Part::empty("abort_safe_state_custody_timestamp_ticks");
    for index in 0..CUSTODY_TIMESTAMP_TICKS {
        ticks = ticks
            + centered_cube(
                format!("abort_safe_state_custody_timestamp_tick_{index}"),
                8.0,
                26.0,
                5.0,
            )
            .translate(
                CUSTODY_CENTER.0 - 84.0 + index as f64 * 24.0,
                CUSTODY_CENTER.1 + 18.0,
                DECK_Z + CUSTODY_PANEL_Z + 2.5,
            );
    }
    ticks
}

fn release_hold_reject_gates() -> Part {
    let panel = centered_cube(
        "abort_safe_state_release_hold_reject_gate_panel",
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, DECK_Z + GATE_PANEL_Z / 2.0);

    panel - gate_token_recesses() + disposition_gate_blocks()
}

fn gate_token_recesses() -> Part {
    let mut recesses = Part::empty("abort_safe_state_gate_token_recesses");
    for index in 0..GATE_TOKEN_COUNT {
        let col = index % 3;
        let row = index / 3;
        recesses = recesses
            + centered_cube(
                format!("abort_safe_state_gate_token_recess_{index}"),
                GATE_SLOT_X,
                GATE_SLOT_Y,
                GATE_PANEL_Z + 8.0,
            )
            .translate(
                GATE_CENTER.0 - 72.0 + col as f64 * 72.0,
                GATE_CENTER.1 - 62.0 + row as f64 * 62.0,
                DECK_Z + GATE_PANEL_Z / 2.0 + 1.0,
            );
    }
    recesses
}

fn disposition_gate_blocks() -> Part {
    let mut blocks = Part::empty("abort_safe_state_disposition_gate_blocks");
    for index in 0..DISPOSITION_GATES {
        blocks = blocks
            + centered_cube(
                format!("abort_safe_state_release_hold_reject_gate_block_{index}"),
                66.0,
                18.0,
                22.0,
            )
            .translate(
                GATE_CENTER.0 - 72.0 + index as f64 * 72.0,
                GATE_CENTER.1 + 86.0,
                DECK_Z + GATE_PANEL_Z + 11.0,
            );
    }
    blocks
}

fn evidence_camera_bridge_and_keepouts() -> Part {
    evidence_camera_bridge() + robot_service_keepouts()
}

fn evidence_camera_bridge() -> Part {
    let post_z = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
    let beam_z = DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0;
    let y = 0.0;
    let left_post = centered_cube(
        "abort_safe_state_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, y, DECK_Z + post_z / 2.0);
    let right_post = centered_cube(
        "abort_safe_state_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, y, DECK_Z + post_z / 2.0);
    let beam = centered_cube(
        "abort_safe_state_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, y, beam_z);

    left_post + right_post + beam + camera_pods() + evidence_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("abort_safe_state_camera_pods");
    for index in 0..CAMERA_PODS {
        pods = pods
            + centered_cube(
                format!("abort_safe_state_evidence_camera_pod_{index}"),
                CAMERA_POD_X,
                CAMERA_POD_Y,
                CAMERA_POD_Z,
            )
            .translate(
                -1.5 * CAMERA_PITCH_X + index as f64 * CAMERA_PITCH_X,
                0.0,
                DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
            );
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("abort_safe_state_evidence_light_bars");
    for index in 0..EVIDENCE_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("abort_safe_state_evidence_light_bar_{index}"),
                150.0,
                10.0,
                8.0,
            )
            .translate(
                -375.0 + index as f64 * 150.0,
                44.0,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 18.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let z = KEEP_OUT_RAIL_Z / 2.0;
    let front = centered_cube(
        "abort_safe_state_front_robot_keepout_rail",
        DECK_X - 120.0,
        12.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, z);
    let rear = centered_cube(
        "abort_safe_state_rear_service_keepout_rail",
        DECK_X - 120.0,
        12.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y, z);
    let left = centered_cube(
        "abort_safe_state_left_quarantine_service_keepout",
        12.0,
        DECK_Y - 140.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-DECK_X / 2.0 + LEFT_QUARANTINE_KEEP_OUT_X, 0.0, z);
    let right = centered_cube(
        "abort_safe_state_right_logger_service_keepout",
        12.0,
        DECK_Y - 140.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(DECK_X / 2.0 - RIGHT_LOGGER_KEEP_OUT_X, 0.0, z);

    front + rear + left + right
}

fn mount_hole_points() -> Vec<(f64, f64)> {
    vec![
        (-750.0, -455.0),
        (-450.0, -455.0),
        (-150.0, -455.0),
        (150.0, -455.0),
        (450.0, -455.0),
        (750.0, -455.0),
        (-750.0, 455.0),
        (-450.0, 455.0),
        (-150.0, 455.0),
        (150.0, 455.0),
        (450.0, 455.0),
        (750.0, 455.0),
    ]
}

fn sterile_boundary_post_points() -> Vec<(f64, f64)> {
    vec![
        (-690.0, -390.0),
        (-140.0, -390.0),
        (410.0, -390.0),
        (-690.0, 390.0),
        (-140.0, 390.0),
        (410.0, 390.0),
    ]
}

fn token_position(index: usize) -> (f64, f64) {
    let row = index / 6;
    let col = index % 6;
    (
        TOKEN_CENTER.0 - 2.5 * TOKEN_PITCH_X + col as f64 * TOKEN_PITCH_X,
        TOKEN_CENTER.1 + 0.5 * TOKEN_PITCH_Y - row as f64 * TOKEN_PITCH_Y,
    )
}

fn layout_rects() -> Vec<Rect> {
    vec![
        Rect {
            name: "emergency_stop_guard",
            center: ESTOP_CENTER,
            x: ESTOP_PANEL_X,
            y: ESTOP_PANEL_Y,
        },
        Rect {
            name: "pump_valve_state_token_panel",
            center: TOKEN_CENTER,
            x: TOKEN_PANEL_X,
            y: TOKEN_PANEL_Y,
        },
        Rect {
            name: "waste_diverter_manifold_surrogate",
            center: WASTE_CENTER,
            x: WASTE_PANEL_X,
            y: WASTE_PANEL_Y,
        },
        Rect {
            name: "cassette_quarantine_dock",
            center: QUARANTINE_CENTER,
            x: QUARANTINE_DOCK_X,
            y: QUARANTINE_DOCK_Y,
        },
        Rect {
            name: "retained_sample_vial_nest",
            center: SAMPLE_CENTER,
            x: SAMPLE_PANEL_X,
            y: SAMPLE_PANEL_Y,
        },
        Rect {
            name: "alarm_event_logger_pocket",
            center: LOGGER_CENTER,
            x: LOGGER_POCKET_X,
            y: LOGGER_POCKET_Y,
        },
        Rect {
            name: "pressure_relief_vent_holder",
            center: VENT_CENTER,
            x: VENT_PANEL_X,
            y: VENT_PANEL_Y,
        },
        Rect {
            name: "fluid_backflow_witness",
            center: BACKFLOW_CENTER,
            x: BACKFLOW_PANEL_X,
            y: BACKFLOW_PANEL_Y,
        },
        Rect {
            name: "barcode_rfid_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Rect {
            name: "release_hold_reject_gates",
            center: GATE_CENTER,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
    ]
}

fn evidence_bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

fn token_state_count() -> usize {
    PUMP_TOKEN_COUNT + VALVE_TOKEN_COUNT
}

fn audit_land_count() -> usize {
    BARCODE_LANDS + RFID_LANDS + CUSTODY_TIMESTAMP_TICKS
}

fn assert_design_constraints() {
    assert_eq!(
        OUTPUT_PREFIX,
        "closed_cell_culture_run_abort_safe_state_station"
    );
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(MOUNT_HOLE_COUNT, mount_hole_points().len());
    assert_eq!(STERILE_BOUNDARY_POSTS, sterile_boundary_post_points().len());
    assert_eq!(ESTOP_GUARD_POSTS, 4);
    assert_eq!(token_state_count(), TOKEN_COUNT);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(CASSETTE_SLOT_COUNT, 4);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with("output/closed_cell_culture_run_abort_safe_state_station_")));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(evidence_bridge_clearance_above_deck() > REVC_TOTAL_HEIGHT + QUARANTINE_DOCK_Z + 120.0);

    let rects = layout_rects();
    for rect in &rects {
        assert!(
            rect.fits_inside_deck(),
            "{} must fit inside the containment deck",
            rect.name
        );
    }

    for (index, rect) in rects.iter().enumerate() {
        for other in rects.iter().skip(index + 1) {
            assert!(
                !rect.overlaps(*other),
                "{} overlaps {}",
                rect.name,
                other.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_cover_named_safe_state_feature_groups() {
        assert_design_constraints();
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    }

    #[test]
    fn visible_abort_state_counts_are_complete() {
        assert_eq!(token_state_count(), 12);
        assert_eq!(PUMP_TOKEN_COUNT, 4);
        assert_eq!(VALVE_TOKEN_COUNT, 8);
        assert_eq!(BACKFLOW_WITNESS_WINDOWS, BACKFLOW_LANE_COUNT * 2);
        assert_eq!(audit_land_count(), 24);
    }

    #[test]
    fn quarantine_and_bridge_clearances_match_closed_cassette_use() {
        assert_eq!(CASSETTE_SLOT_COUNT * 2, QUARANTINE_LOCK_PINS);
        assert!(CASSETTE_SLOT_X > REVC_CHIP_LENGTH);
        assert!(CASSETTE_SLOT_Y > REVC_CHIP_WIDTH);
        assert!(evidence_bridge_clearance_above_deck() > REVC_TOTAL_HEIGHT + 150.0);
    }
}
