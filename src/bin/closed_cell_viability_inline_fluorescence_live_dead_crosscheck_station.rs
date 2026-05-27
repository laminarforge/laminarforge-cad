use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline live/dead viability crosscheck station.
//
// Intent:
// - Verify one sealed cell-suspension sample loop immediately before scaled
//   tissue-chip cassette seeding.
// - Package fluorescence and brightfield checks, live/dead standard wells,
//   dilution and flush routing, waste/retain disposition, custody lands, and
//   evidence capture in one closed pre-seed module.
// - Model fixture interfaces and evidence geometry only. Viability thresholds,
//   staining recipes, exposure timing, and release authority remain external
//   quality-system data.

const OUTPUT_PREFIX: &str =
    "closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_secondary_containment_deck.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_sealed_sample_loop_cartridge_bank.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_fluorescence_brightfield_window_banks.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_live_dead_standard_well_rack.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_dilution_flush_route_manifold.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_light_shielded_imaging_hood.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_bubble_dead_volume_witness_windows.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_waste_retain_split_manifold.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_barcode_custody_lands.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_release_hold_reject_gate_array.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_camera_evidence_bridge_robotic_service_datums.stl",
    "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 13] = [
    "sealed_sample_loop_cartridges",
    "fluorescence_window_bank",
    "brightfield_window_bank",
    "live_dead_standard_wells",
    "dilution_flush_routes",
    "light_shielded_imaging_hood",
    "bubble_witness_windows",
    "dead_volume_witness_windows",
    "waste_retain_split",
    "barcode_custody_lands",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robotic_service_datums",
];

const VIABILITY_CHANNELS: [&str; 3] = ["calcein_live", "ethidium_dead", "brightfield"];
const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 40.0;
const SUMP_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_SLOT_W: f64 = 16.0;
const LEAK_WITNESS_WELLS: usize = 12;

const LOOP_CENTER: (f64, f64) = (-470.0, 205.0);
const LOOP_X: f64 = 430.0;
const LOOP_Y: f64 = 230.0;
const LOOP_Z: f64 = 58.0;
const SAMPLE_LOOP_CARTRIDGES: usize = 6;
const SAMPLE_LOOP_PORTS: usize = SAMPLE_LOOP_CARTRIDGES * 2;
const LOOP_PINCH_VALVES: usize = 8;
const LOOP_PRESSURE_TAPS: usize = 4;
const LOOP_CARTRIDGE_X: f64 = 48.0;
const LOOP_CARTRIDGE_Y: f64 = 136.0;
const LOOP_CARTRIDGE_PITCH: f64 = 60.0;
const LOOP_TUBE_D: f64 = 7.0;
const LOOP_BORE_D: f64 = 4.8;

const WINDOW_CENTER: (f64, f64) = (10.0, 205.0);
const WINDOW_X: f64 = 430.0;
const WINDOW_Y: f64 = 230.0;
const WINDOW_Z: f64 = 52.0;
const FLUORESCENCE_WINDOWS: usize = 6;
const BRIGHTFIELD_WINDOWS: usize = 6;
const TOTAL_OPTICAL_WINDOWS: usize = FLUORESCENCE_WINDOWS + BRIGHTFIELD_WINDOWS;
const WINDOW_PITCH_X: f64 = 58.0;
const WINDOW_APERTURE_X: f64 = 34.0;
const WINDOW_APERTURE_Y: f64 = 25.0;
const EXCITATION_BAFFLES: usize = 6;
const EMISSION_FILTER_SLOTS: usize = 6;
const BRIGHTFIELD_DIFFUSER_BARS: usize = 4;

const STANDARD_CENTER: (f64, f64) = (465.0, 205.0);
const STANDARD_X: f64 = 350.0;
const STANDARD_Y: f64 = 230.0;
const STANDARD_Z: f64 = 54.0;
const LIVE_DEAD_CONTROL_FAMILIES: usize = 3;
const LIVE_DEAD_CONTROL_LEVELS: usize = 4;
const LIVE_DEAD_STANDARD_WELLS: usize = LIVE_DEAD_CONTROL_FAMILIES * LIVE_DEAD_CONTROL_LEVELS;
const STANDARD_WELL_D: f64 = 18.0;
const STANDARD_PITCH_X: f64 = 66.0;
const STANDARD_PITCH_Y: f64 = 54.0;
const STANDARD_COA_SLOTS: usize = 3;

const ROUTE_CENTER: (f64, f64) = (-455.0, -115.0);
const ROUTE_X: f64 = 430.0;
const ROUTE_Y: f64 = 250.0;
const ROUTE_Z: f64 = 50.0;
const DILUTION_LEVELS: usize = 5;
const DILUTION_REPLICATES: usize = 2;
const DILUTION_COUPONS: usize = DILUTION_LEVELS * DILUTION_REPLICATES;
const FLUSH_ROUTES: usize = 6;
const FLUSH_VALVES: usize = 8;
const DILUTION_WELL_D: f64 = 17.0;
const ROUTE_BORE_D: f64 = 5.4;
const ROUTE_PORT_D: f64 = 9.0;

const WITNESS_CENTER: (f64, f64) = (0.0, -115.0);
const WITNESS_X: f64 = 400.0;
const WITNESS_Y: f64 = 250.0;
const WITNESS_Z: f64 = 42.0;
const BUBBLE_WITNESS_WINDOWS: usize = 6;
const DEAD_VOLUME_WITNESS_WINDOWS: usize = 6;
const WITNESS_LEVEL_TICKS: usize = 9;
const WITNESS_WINDOW_X: f64 = 40.0;
const WITNESS_WINDOW_Y: f64 = 34.0;
const WITNESS_PITCH_X: f64 = 54.0;

const SPLIT_CENTER: (f64, f64) = (445.0, -115.0);
const SPLIT_X: f64 = 350.0;
const SPLIT_Y: f64 = 250.0;
const SPLIT_Z: f64 = 56.0;
const RETAIN_SPLIT_LANES: usize = 2;
const RETAIN_VIALS: usize = 4;
const WASTE_PORTS: usize = 3;
const SPLIT_VALVES: usize = 6;
const RETAIN_VIAL_D: f64 = 28.0;
const WASTE_SOCKET_D: f64 = 42.0;

const CUSTODY_CENTER: (f64, f64) = (-315.0, -340.0);
const CUSTODY_X: f64 = 560.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 22.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 4;
const CUSTODY_SEAL_LANDS: usize = 6;
const EVIDENCE_CARD_LANDS: usize = 6;

const GATE_CENTER: (f64, f64) = (325.0, -340.0);
const GATE_X: f64 = 500.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 50.0;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SOLENOIDS: usize = DISPOSITION_LANES.len() * 2;
const DECISION_INPUT_TOKENS: usize = VIABILITY_CHANNELS.len() * DISPOSITION_LANES.len();
const GATE_LANE_PITCH_X: f64 = 150.0;

const HOOD_X: f64 = 520.0;
const HOOD_Y: f64 = 315.0;
const HOOD_Z: f64 = 170.0;
const HOOD_WALL: f64 = 16.0;
const HOOD_SHUTTER_SLOTS: usize = 3;
const HOOD_FILTER_CASSETTES: usize = 3;
const HOOD_SERVICE_DOORS: usize = 2;

const BRIDGE_SPAN_X: f64 = 1320.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 46.0;
const BRIDGE_UNDERSIDE_Z: f64 = 240.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_COUNT: usize = 4;
const LED_PANEL_COUNT: usize = 6;
const ROBOT_SERVICE_DATUMS: usize = 8;
const ROBOT_GRIPPER_DATUMS: usize = 4;
const SERVICE_CLEARANCE_ZONES: usize = 4;
const FRONT_ROBOT_CLEARANCE: f64 = 350.0;
const REAR_OPTICS_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_LOOP_SERVICE_CLEARANCE: f64 = 235.0;
const RIGHT_SPLIT_SERVICE_CLEARANCE: f64 = 235.0;

#[derive(Clone, Copy, Debug)]
struct ModuleSpec {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
    z: f64,
}

impl ModuleSpec {
    fn fits_on_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= DECK_X / 2.0 - RIM_W - 12.0
            && self.center.1.abs() + self.y / 2.0 <= DECK_Y / 2.0 - RIM_W - 12.0
    }

    fn overlaps(self, other: ModuleSpec) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    write_part(OUTPUTS[0], &deck);

    let loop_bank = sealed_sample_loop_cartridge_bank();
    write_part(OUTPUTS[1], &loop_bank);

    let windows = fluorescence_brightfield_window_banks();
    write_part(OUTPUTS[2], &windows);

    let standards = live_dead_standard_well_rack();
    write_part(OUTPUTS[3], &standards);

    let routes = dilution_flush_route_manifold();
    write_part(OUTPUTS[4], &routes);

    let hood = light_shielded_imaging_hood();
    write_part(OUTPUTS[5], &hood);

    let witnesses = bubble_dead_volume_witness_windows();
    write_part(OUTPUTS[6], &witnesses);

    let split = waste_retain_split_manifold();
    write_part(OUTPUTS[7], &split);

    let custody = barcode_custody_lands();
    write_part(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gate_array();
    write_part(OUTPUTS[9], &gates);

    let bridge = camera_evidence_bridge_robotic_service_datums();
    write_part(OUTPUTS[10], &bridge);

    let assembly = station_assembly();
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed inline live/dead viability crosscheck station:");
    println!("  Footprint: {DECK_X:.0}mm x {DECK_Y:.0}mm leak tray before scaled cassette seeding");
    println!(
        "  Closed sample loop: {SAMPLE_LOOP_CARTRIDGES} sealed cartridges, {SAMPLE_LOOP_PORTS} ports, {LOOP_PINCH_VALVES} pinch valves, {LOOP_PRESSURE_TAPS} pressure taps"
    );
    println!(
        "  Optical crosscheck: {FLUORESCENCE_WINDOWS} fluorescence windows, {BRIGHTFIELD_WINDOWS} brightfield windows, channels: {}",
        VIABILITY_CHANNELS.join(", ")
    );
    println!(
        "  Standards/routes: {LIVE_DEAD_STANDARD_WELLS} live/dead standard wells, {DILUTION_COUPONS} dilution coupons, {FLUSH_ROUTES} flush routes"
    );
    println!(
        "  Witness/disposition: {BUBBLE_WITNESS_WINDOWS} bubble windows, {DEAD_VOLUME_WITNESS_WINDOWS} dead-volume windows, {} lanes ({})",
        DISPOSITION_LANES.len(),
        DISPOSITION_LANES.join(", ")
    );
    println!(
        "  Custody/evidence: {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {CAMERA_COUNT} cameras, {ROBOT_SERVICE_DATUMS} robot service datums"
    );
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    secondary_containment_deck()
        + sealed_sample_loop_cartridge_bank().translate(
            LOOP_CENTER.0,
            LOOP_CENTER.1,
            on_deck_z(LOOP_Z),
        )
        + fluorescence_brightfield_window_banks().translate(
            WINDOW_CENTER.0,
            WINDOW_CENTER.1,
            on_deck_z(WINDOW_Z),
        )
        + live_dead_standard_well_rack().translate(
            STANDARD_CENTER.0,
            STANDARD_CENTER.1,
            on_deck_z(STANDARD_Z),
        )
        + dilution_flush_route_manifold().translate(
            ROUTE_CENTER.0,
            ROUTE_CENTER.1,
            on_deck_z(ROUTE_Z),
        )
        + light_shielded_imaging_hood().translate(
            WINDOW_CENTER.0,
            WINDOW_CENTER.1,
            on_deck_z(HOOD_Z),
        )
        + bubble_dead_volume_witness_windows().translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            on_deck_z(WITNESS_Z),
        )
        + waste_retain_split_manifold().translate(
            SPLIT_CENTER.0,
            SPLIT_CENTER.1,
            on_deck_z(SPLIT_Z),
        )
        + barcode_custody_lands().translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            on_deck_z(CUSTODY_Z),
        )
        + release_hold_reject_gate_array().translate(
            GATE_CENTER.0,
            GATE_CENTER.1,
            on_deck_z(GATE_Z),
        )
        + camera_evidence_bridge_robotic_service_datums().translate(0.0, 0.0, DECK_Z)
}

fn on_deck_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn secondary_containment_deck() -> Part {
    let floor = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_floor"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        format!("{OUTPUT_PREFIX}_recessed_secondary_containment_sump_cut"),
        DECK_X - 152.0,
        DECK_Y - 148.0,
        SUMP_DEPTH + 0.8,
    )
    .translate(0.0, -4.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.4);
    let drain_slot = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_slot_cut"),
        DECK_X - 270.0,
        DRAIN_SLOT_W,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 64.0, DECK_Z - SUMP_DEPTH / 2.0);

    floor - sump - drain_slot - module_socket_cuts() - deck_mount_clearances()
        + perimeter_rims()
        + process_zone_dividers()
        + scaled_cassette_handoff_datums()
        + deck_fiducial_targets()
        + leak_witness_well_bank()
}

fn module_socket_cuts() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_module_socket_cuts"));
    for module in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_socket_cut", module.name),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_clearances() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_clearances"));
    for (i, &(x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_m6_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(x, y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_m6_mount_slot_relief_{i}"),
            26.0,
            7.4,
            DECK_Z + 4.0,
        )
        .translate(x, y, DECK_Z / 2.0);
        cuts = cuts + hole + slot;
    }
    cuts
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_robot_access_lip"),
        DECK_X - 190.0,
        14.0,
        22.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 11.0);

    left + right + rear + front
}

fn process_zone_dividers() -> Part {
    let upper_lower = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_and_routing_zone_divider"),
        DECK_X - 188.0,
        10.0,
        28.0,
    )
    .translate(0.0, 45.0, DECK_Z + 14.0);
    let custody_split = centered_cube(
        format!("{OUTPUT_PREFIX}_custody_disposition_zone_divider"),
        DECK_X - 230.0,
        10.0,
        26.0,
    )
    .translate(0.0, -252.0, DECK_Z + 13.0);
    let loop_optics_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_sample_loop_to_optics_baffle"),
        10.0,
        250.0,
        28.0,
    )
    .translate(-230.0, 205.0, DECK_Z + 14.0);
    let optics_standards_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_optics_to_live_dead_standard_baffle"),
        10.0,
        250.0,
        28.0,
    )
    .translate(235.0, 205.0, DECK_Z + 14.0);
    let witness_split_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_witness_to_retain_split_baffle"),
        10.0,
        250.0,
        26.0,
    )
    .translate(220.0, -115.0, DECK_Z + 13.0);

    upper_lower
        + custody_split
        + loop_optics_baffle
        + optics_standards_baffle
        + witness_split_baffle
}

fn scaled_cassette_handoff_datums() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_scaled_tissue_chip_cassette_preseed_handoff_rail"),
        430.0,
        18.0,
        20.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 122.0, DECK_Z + 10.0);
    let mut datum_blocks = Part::empty(format!("{OUTPUT_PREFIX}_scaled_cassette_handoff_datums"));
    for i in 0..ROBOT_GRIPPER_DATUMS {
        let x = centered_index(i, ROBOT_GRIPPER_DATUMS, 108.0);
        datum_blocks = datum_blocks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_scaled_cassette_handoff_datum_block_{i}"),
                42.0,
                28.0,
                16.0,
            )
            .translate(x, -DECK_Y / 2.0 + 154.0, DECK_Z + 8.0);
    }
    rail + datum_blocks
}

fn deck_fiducial_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_deck_fiducial_targets"));
    for (i, &(x, y)) in deck_fiducial_points().iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("{OUTPUT_PREFIX}_deck_fiducial_{i}")).translate(
                x,
                y,
                DECK_Z + 2.0,
            );
    }
    targets
}

fn leak_witness_well_bank() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_witness_well_bank"));
    for i in 0..LEAK_WITNESS_WELLS {
        let x = centered_index(i % 6, 6, 72.0);
        let y = -DECK_Y / 2.0 + 94.0 + (i / 6) as f64 * 34.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_viability_leak_witness_well_{i}"),
                10.0,
                6.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.0);
    }
    wells
}

fn sealed_sample_loop_cartridge_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_sample_loop_cartridge_bank_body"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let lid_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_loop_lid_gasket_recess_cut"),
        LOOP_X - 36.0,
        LOOP_Y - 34.0,
        9.0,
    )
    .translate(0.0, 0.0, LOOP_Z / 2.0 - 4.5);

    body - lid_recess - cartridge_socket_cuts() - sample_loop_header_bores()
        + cartridge_seal_rims()
        + sterile_loop_port_bosses()
        + sample_loop_tube_race_retainers()
        + sample_loop_pinch_valve_pads()
        + sample_loop_pressure_tap_lands()
        + cartridge_latch_and_barcode_lands()
        + loop_flow_direction_markers()
}

fn cartridge_socket_cuts() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_cartridge_socket_cuts"));
    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = centered_index(i, SAMPLE_LOOP_CARTRIDGES, LOOP_CARTRIDGE_PITCH);
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_sealed_sample_loop_cartridge_socket_{i}"),
                LOOP_CARTRIDGE_X,
                LOOP_CARTRIDGE_Y,
                18.0,
            )
            .translate(x, 0.0, LOOP_Z / 2.0 - 9.0);
    }
    sockets
}

fn sample_loop_header_bores() -> Part {
    let upper = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_upper_closed_header_bore"),
        LOOP_BORE_D / 2.0,
        LOOP_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOOP_Y / 2.0 - 44.0, 4.0);
    let lower = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_lower_closed_header_bore"),
        LOOP_BORE_D / 2.0,
        LOOP_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -LOOP_Y / 2.0 + 44.0, -4.0);
    upper + lower
}

fn cartridge_seal_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_cartridge_seal_rims"));
    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = centered_index(i, SAMPLE_LOOP_CARTRIDGES, LOOP_CARTRIDGE_PITCH);
        let rim = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_cartridge_seal_rim_{i}"),
            LOOP_CARTRIDGE_X + 12.0,
            LOOP_CARTRIDGE_Y + 12.0,
            5.0,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 2.5);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_cartridge_seal_opening_{i}"),
            LOOP_CARTRIDGE_X - 4.0,
            LOOP_CARTRIDGE_Y - 4.0,
            5.6,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 2.5);
        rims = rims + (rim - opening);
    }
    rims
}

fn sterile_loop_port_bosses() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_sterile_loop_port_bosses"));
    for i in 0..SAMPLE_LOOP_PORTS {
        let cartridge = i / 2;
        let side = i % 2;
        let x = centered_index(cartridge, SAMPLE_LOOP_CARTRIDGES, LOOP_CARTRIDGE_PITCH);
        let y = if side == 0 {
            LOOP_Y / 2.0 - 28.0
        } else {
            -LOOP_Y / 2.0 + 28.0
        };
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sealed_sample_loop_port_boss_{i}"),
            14.0,
            14.0,
            32,
        )
        .translate(x, y, LOOP_Z / 2.0 + 7.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sealed_sample_loop_port_bore_{i}"),
            LOOP_BORE_D / 2.0,
            16.0,
            20,
        )
        .translate(x, y, LOOP_Z / 2.0 + 7.0);
        let cap_land = centered_cube(
            format!("{OUTPUT_PREFIX}_sealed_sample_loop_port_cap_land_{i}"),
            32.0,
            18.0,
            6.0,
        )
        .translate(
            x,
            y + if side == 0 { -24.0 } else { 24.0 },
            LOOP_Z / 2.0 + 3.0,
        );
        ports = ports + (boss - bore) + cap_land;
    }
    ports
}

fn sample_loop_tube_race_retainers() -> Part {
    let upper = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_upper_tube_race_retainer"),
        LOOP_TUBE_D / 2.0,
        LOOP_X - 74.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOOP_Y / 2.0 - 58.0, LOOP_Z / 2.0 + 12.0);
    let lower = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_lower_tube_race_retainer"),
        LOOP_TUBE_D / 2.0,
        LOOP_X - 74.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -LOOP_Y / 2.0 + 58.0, LOOP_Z / 2.0 + 12.0);
    let left = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_left_tube_race_retainer"),
        LOOP_TUBE_D / 2.0,
        LOOP_Y - 116.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-LOOP_X / 2.0 + 42.0, 0.0, LOOP_Z / 2.0 + 12.0);
    let right = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_right_tube_race_retainer"),
        LOOP_TUBE_D / 2.0,
        LOOP_Y - 116.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LOOP_X / 2.0 - 42.0, 0.0, LOOP_Z / 2.0 + 12.0);

    upper + lower + left + right
}

fn sample_loop_pinch_valve_pads() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_pads"));
    for i in 0..LOOP_PINCH_VALVES {
        let x = centered_index(i % 4, 4, 82.0);
        let y = if i < 4 { 52.0 } else { -52.0 };
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_pad_{i}"),
                34.0,
                24.0,
                10.0,
            )
            .translate(x, y, LOOP_Z / 2.0 + 5.0);
    }
    valves
}

fn sample_loop_pressure_tap_lands() -> Part {
    let mut taps = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pressure_tap_lands"));
    for i in 0..LOOP_PRESSURE_TAPS {
        let x = centered_index(i, LOOP_PRESSURE_TAPS, 96.0);
        let land = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_loop_pressure_tap_land_{i}"),
            10.0,
            6.0,
            24,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_loop_pressure_tap_bore_{i}"),
            2.2,
            7.0,
            16,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 3.0);
        taps = taps + (land - bore);
    }
    taps
}

fn cartridge_latch_and_barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_latch_barcode_lands"));
    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = centered_index(i, SAMPLE_LOOP_CARTRIDGES, LOOP_CARTRIDGE_PITCH);
        let latch = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_cartridge_latch_pair_{i}"),
            36.0,
            12.0,
            8.0,
        )
        .translate(x, -8.0, LOOP_Z / 2.0 + 4.0);
        let barcode = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_cartridge_barcode_land_{i}"),
            42.0,
            12.0,
            3.0,
        )
        .translate(x, -LOOP_Y / 2.0 + 14.0, LOOP_Z / 2.0 + 1.5);
        lands = lands + latch + barcode;
    }
    lands
}

fn loop_flow_direction_markers() -> Part {
    let mut markers = Part::empty(format!(
        "{OUTPUT_PREFIX}_sample_loop_flow_direction_markers"
    ));
    for i in 0..8 {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_sample_loop_flow_marker_{i}"),
                34.0,
                5.0,
                4.0,
            )
            .translate(
                centered_index(i % 4, 4, 82.0),
                if i < 4 { 82.0 } else { -82.0 },
                LOOP_Z / 2.0 + 2.0,
            );
    }
    markers
}

fn fluorescence_brightfield_window_banks() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_fluorescence_brightfield_window_bank_body"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    let light_gasket = centered_cube(
        format!("{OUTPUT_PREFIX}_window_bank_hood_gasket_recess_cut"),
        WINDOW_X - 34.0,
        WINDOW_Y - 34.0,
        6.0,
    )
    .translate(0.0, 0.0, WINDOW_Z / 2.0 - 3.0);

    body - light_gasket - optical_window_aperture_cuts()
        + optical_window_frames()
        + fluorescence_filter_slots()
        + brightfield_diffuser_bars()
        + excitation_emission_baffles()
        + window_bank_flow_cell_rails()
}

fn optical_window_aperture_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_optical_window_aperture_cuts"));
    for i in 0..FLUORESCENCE_WINDOWS {
        let x = centered_index(i, FLUORESCENCE_WINDOWS, WINDOW_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_fluorescence_window_clear_aperture_{i}"),
                WINDOW_APERTURE_X,
                WINDOW_APERTURE_Y,
                WINDOW_Z + 4.0,
            )
            .translate(x, 44.0, 0.0);
    }
    for i in 0..BRIGHTFIELD_WINDOWS {
        let x = centered_index(i, BRIGHTFIELD_WINDOWS, WINDOW_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_brightfield_window_clear_aperture_{i}"),
                WINDOW_APERTURE_X,
                WINDOW_APERTURE_Y,
                WINDOW_Z + 4.0,
            )
            .translate(x, -58.0, 0.0);
    }
    cuts
}

fn optical_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_optical_window_frames"));
    for i in 0..TOTAL_OPTICAL_WINDOWS {
        let fluorescence = i < FLUORESCENCE_WINDOWS;
        let local = if fluorescence {
            i
        } else {
            i - FLUORESCENCE_WINDOWS
        };
        let x = centered_index(local, FLUORESCENCE_WINDOWS, WINDOW_PITCH_X);
        let y = if fluorescence { 44.0 } else { -58.0 };
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_window_frame_{i}"),
            WINDOW_APERTURE_X + 12.0,
            WINDOW_APERTURE_Y + 12.0,
            5.0,
        )
        .translate(x, y, WINDOW_Z / 2.0 + 2.5);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_window_frame_opening_{i}"),
            WINDOW_APERTURE_X + 2.0,
            WINDOW_APERTURE_Y + 2.0,
            5.6,
        )
        .translate(x, y, WINDOW_Z / 2.0 + 2.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn fluorescence_filter_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_fluorescence_filter_slots"));
    for i in 0..EMISSION_FILTER_SLOTS {
        let x = centered_index(i, EMISSION_FILTER_SLOTS, WINDOW_PITCH_X);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_emission_filter_slide_slot_{i}"),
            42.0,
            8.0,
            12.0,
        )
        .translate(x, 80.0, WINDOW_Z / 2.0 + 6.0);
        let finger_notch = centered_cylinder(
            format!("{OUTPUT_PREFIX}_emission_filter_finger_notch_{i}"),
            4.0,
            12.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 86.0, WINDOW_Z / 2.0 + 6.0);
        slots = slots + (slot - finger_notch);
    }
    slots
}

fn brightfield_diffuser_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_brightfield_diffuser_bars"));
    for i in 0..BRIGHTFIELD_DIFFUSER_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_brightfield_diffuser_bar_{i}"),
                WINDOW_X - 78.0,
                6.0,
                8.0,
            )
            .translate(0.0, -106.0 + i as f64 * 14.0, WINDOW_Z / 2.0 + 4.0);
    }
    bars
}

fn excitation_emission_baffles() -> Part {
    let mut baffles = Part::empty(format!("{OUTPUT_PREFIX}_excitation_emission_baffles"));
    for i in 0..EXCITATION_BAFFLES {
        let x = centered_index(i, EXCITATION_BAFFLES, WINDOW_PITCH_X);
        baffles = baffles
            + centered_cube(
                format!("{OUTPUT_PREFIX}_fluorescence_cross_talk_baffle_{i}"),
                8.0,
                70.0,
                18.0,
            )
            .translate(x + WINDOW_PITCH_X / 2.0, 44.0, WINDOW_Z / 2.0 + 9.0);
    }
    baffles
}

fn window_bank_flow_cell_rails() -> Part {
    let upper = centered_cube(
        format!("{OUTPUT_PREFIX}_upper_flow_cell_retainer_rail"),
        WINDOW_X - 58.0,
        9.0,
        12.0,
    )
    .translate(0.0, 18.0, WINDOW_Z / 2.0 + 6.0);
    let lower = centered_cube(
        format!("{OUTPUT_PREFIX}_lower_flow_cell_retainer_rail"),
        WINDOW_X - 58.0,
        9.0,
        12.0,
    )
    .translate(0.0, -18.0, WINDOW_Z / 2.0 + 6.0);
    upper + lower
}

fn live_dead_standard_well_rack() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_live_dead_standard_well_rack_body"),
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    );
    let lid_gasket = centered_cube(
        format!("{OUTPUT_PREFIX}_live_dead_standard_lid_gasket_recess_cut"),
        STANDARD_X - 34.0,
        STANDARD_Y - 32.0,
        8.0,
    )
    .translate(0.0, 0.0, STANDARD_Z / 2.0 - 4.0);

    body - lid_gasket - live_dead_standard_well_cuts()
        + live_dead_standard_well_rims()
        + live_dead_family_partition_rails()
        + standard_coa_card_slots()
        + viability_threshold_token_lands()
}

fn live_dead_standard_well_cuts() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_live_dead_standard_well_cuts"));
    for i in 0..LIVE_DEAD_STANDARD_WELLS {
        let x = centered_index(
            i % LIVE_DEAD_CONTROL_LEVELS,
            LIVE_DEAD_CONTROL_LEVELS,
            STANDARD_PITCH_X,
        );
        let y = centered_index(
            i / LIVE_DEAD_CONTROL_LEVELS,
            LIVE_DEAD_CONTROL_FAMILIES,
            STANDARD_PITCH_Y,
        );
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_live_dead_standard_well_cut_{i}"),
                STANDARD_WELL_D / 2.0,
                STANDARD_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn live_dead_standard_well_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_live_dead_standard_well_rims"));
    for i in 0..LIVE_DEAD_STANDARD_WELLS {
        let x = centered_index(
            i % LIVE_DEAD_CONTROL_LEVELS,
            LIVE_DEAD_CONTROL_LEVELS,
            STANDARD_PITCH_X,
        );
        let y = centered_index(
            i / LIVE_DEAD_CONTROL_LEVELS,
            LIVE_DEAD_CONTROL_FAMILIES,
            STANDARD_PITCH_Y,
        );
        let rim = centered_cylinder(
            format!("{OUTPUT_PREFIX}_live_dead_standard_well_rim_{i}"),
            STANDARD_WELL_D / 2.0 + 4.0,
            4.0,
            30,
        )
        .translate(x, y, STANDARD_Z / 2.0 + 2.0);
        let opening = centered_cylinder(
            format!("{OUTPUT_PREFIX}_live_dead_standard_well_rim_opening_{i}"),
            STANDARD_WELL_D / 2.0 + 0.8,
            4.5,
            30,
        )
        .translate(x, y, STANDARD_Z / 2.0 + 2.0);
        rims = rims + (rim - opening);
    }
    rims
}

fn live_dead_family_partition_rails() -> Part {
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_live_dead_family_partition_rails"));
    for i in 0..(LIVE_DEAD_CONTROL_FAMILIES - 1) {
        let y0 = centered_index(i, LIVE_DEAD_CONTROL_FAMILIES, STANDARD_PITCH_Y);
        let y1 = centered_index(i + 1, LIVE_DEAD_CONTROL_FAMILIES, STANDARD_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_live_dead_control_family_separator_{i}"),
                STANDARD_X - 54.0,
                5.0,
                16.0,
            )
            .translate(0.0, (y0 + y1) / 2.0, STANDARD_Z / 2.0 + 8.0);
    }
    rails
}

fn standard_coa_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_live_dead_standard_coa_card_slots"));
    for i in 0..STANDARD_COA_SLOTS {
        let x = centered_index(i, STANDARD_COA_SLOTS, 86.0);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_live_dead_standard_coa_card_slot_{i}"),
            62.0,
            22.0,
            7.0,
        )
        .translate(x, -STANDARD_Y / 2.0 + 22.0, STANDARD_Z / 2.0 + 3.5);
        let relief = centered_cube(
            format!("{OUTPUT_PREFIX}_live_dead_standard_coa_card_relief_{i}"),
            50.0,
            13.0,
            7.6,
        )
        .translate(x, -STANDARD_Y / 2.0 + 22.0, STANDARD_Z / 2.0 + 3.6);
        slots = slots + (slot - relief);
    }
    slots
}

fn viability_threshold_token_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_viability_threshold_token_lands"));
    for (i, _) in VIABILITY_CHANNELS.iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_viability_channel_threshold_token_land_{i}"),
                76.0,
                15.0,
                5.0,
            )
            .translate(
                centered_index(i, VIABILITY_CHANNELS.len(), 90.0),
                STANDARD_Y / 2.0 - 24.0,
                STANDARD_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn dilution_flush_route_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_dilution_flush_route_manifold_body"),
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let service_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_dilution_flush_service_recess_cut"),
        ROUTE_X - 72.0,
        ROUTE_Y - 70.0,
        10.0,
    )
    .translate(0.0, 0.0, ROUTE_Z / 2.0 - 5.0);

    body - service_recess - dilution_well_cuts() - flush_route_bores()
        + dilution_coupon_rims()
        + flush_port_bosses()
        + flush_valve_pads()
        + route_direction_arrows()
        + diluent_reservoir_socket()
}

fn dilution_well_cuts() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_dilution_well_cuts"));
    for i in 0..DILUTION_COUPONS {
        let x = centered_index(i % DILUTION_LEVELS, DILUTION_LEVELS, 68.0) - 34.0;
        let y = if i < DILUTION_LEVELS { 44.0 } else { -24.0 };
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dilution_coupon_well_cut_{i}"),
                DILUTION_WELL_D / 2.0,
                ROUTE_Z + 4.0,
                28,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn dilution_coupon_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_dilution_coupon_rims"));
    for i in 0..DILUTION_COUPONS {
        let x = centered_index(i % DILUTION_LEVELS, DILUTION_LEVELS, 68.0) - 34.0;
        let y = if i < DILUTION_LEVELS { 44.0 } else { -24.0 };
        rims = rims
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dilution_coupon_rim_{i}"),
                DILUTION_WELL_D / 2.0 + 4.2,
                4.0,
                28,
            )
            .translate(x, y, ROUTE_Z / 2.0 + 2.0);
    }
    rims
}

fn flush_route_bores() -> Part {
    let mut bores = Part::empty(format!("{OUTPUT_PREFIX}_flush_route_bores"));
    for i in 0..FLUSH_ROUTES {
        let y = centered_index(i, FLUSH_ROUTES, 30.0);
        bores = bores
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_flush_route_bore_{i}"),
                ROUTE_BORE_D / 2.0,
                ROUTE_X + 16.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -2.0);
    }
    bores
}

fn flush_port_bosses() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_flush_port_bosses"));
    for i in 0..FLUSH_ROUTES {
        let y = centered_index(i, FLUSH_ROUTES, 30.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_flush_route_port_boss_{i}"),
            ROUTE_PORT_D,
            9.0,
            28,
        )
        .translate(ROUTE_X / 2.0 - 34.0, y, ROUTE_Z / 2.0 + 4.5);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_flush_route_port_bore_{i}"),
            ROUTE_BORE_D / 2.0,
            10.0,
            20,
        )
        .translate(ROUTE_X / 2.0 - 34.0, y, ROUTE_Z / 2.0 + 4.5);
        ports = ports + (boss - bore);
    }
    ports
}

fn flush_valve_pads() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_flush_valve_pads"));
    for i in 0..FLUSH_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_flush_valve_pad_{i}"),
                30.0,
                22.0,
                10.0,
            )
            .translate(
                centered_index(i % 4, 4, 66.0),
                if i < 4 {
                    ROUTE_Y / 2.0 - 38.0
                } else {
                    -ROUTE_Y / 2.0 + 38.0
                },
                ROUTE_Z / 2.0 + 5.0,
            );
    }
    valves
}

fn route_direction_arrows() -> Part {
    let mut arrows = Part::empty(format!(
        "{OUTPUT_PREFIX}_dilution_flush_route_direction_arrows"
    ));
    for i in 0..8 {
        arrows = arrows
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_flush_direction_bar_{i}"),
                36.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(i % 4, 4, 72.0),
                -98.0 + (i / 4) as f64 * 196.0,
                ROUTE_Z / 2.0 + 2.5,
            );
    }
    arrows
}

fn diluent_reservoir_socket() -> Part {
    let socket = centered_cylinder(
        format!("{OUTPUT_PREFIX}_diluent_reservoir_socket"),
        28.0,
        18.0,
        36,
    )
    .translate(
        -ROUTE_X / 2.0 + 52.0,
        -ROUTE_Y / 2.0 + 48.0,
        ROUTE_Z / 2.0 + 9.0,
    );
    let opening = centered_cylinder(
        format!("{OUTPUT_PREFIX}_diluent_reservoir_socket_opening"),
        20.0,
        19.0,
        36,
    )
    .translate(
        -ROUTE_X / 2.0 + 52.0,
        -ROUTE_Y / 2.0 + 48.0,
        ROUTE_Z / 2.0 + 9.0,
    );
    socket - opening
}

fn light_shielded_imaging_hood() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_hood_left_light_shield_wall"),
        HOOD_WALL,
        HOOD_Y,
        HOOD_Z,
    )
    .translate(-HOOD_X / 2.0 + HOOD_WALL / 2.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_hood_right_light_shield_wall"),
        HOOD_WALL,
        HOOD_Y,
        HOOD_Z,
    )
    .translate(HOOD_X / 2.0 - HOOD_WALL / 2.0, 0.0, 0.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_hood_rear_light_trap_wall"),
        HOOD_X,
        HOOD_WALL,
        HOOD_Z,
    )
    .translate(0.0, HOOD_Y / 2.0 - HOOD_WALL / 2.0, 0.0);
    let front_apron = centered_cube(
        format!("{OUTPUT_PREFIX}_hood_front_shutter_apron"),
        HOOD_X - 84.0,
        HOOD_WALL,
        HOOD_Z - 48.0,
    )
    .translate(0.0, -HOOD_Y / 2.0 + HOOD_WALL / 2.0, -24.0);
    let top = centered_cube(
        format!("{OUTPUT_PREFIX}_hood_removable_blackout_top"),
        HOOD_X,
        HOOD_Y,
        HOOD_WALL,
    )
    .translate(0.0, 0.0, HOOD_Z / 2.0 - HOOD_WALL / 2.0);

    left + right
        + rear
        + front_apron
        + top
        + hood_shutter_slots()
        + hood_filter_cassette_lands()
        + hood_service_door_handles()
        + hood_internal_baffle_ribs()
}

fn hood_shutter_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_hood_shutter_slots"));
    for i in 0..HOOD_SHUTTER_SLOTS {
        slots = slots
            + centered_cube(
                format!("{OUTPUT_PREFIX}_hood_slide_shutter_slot_{i}"),
                110.0,
                8.0,
                12.0,
            )
            .translate(
                centered_index(i, HOOD_SHUTTER_SLOTS, 130.0),
                -HOOD_Y / 2.0 + 28.0,
                -HOOD_Z / 2.0 + 36.0,
            );
    }
    slots
}

fn hood_filter_cassette_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_hood_filter_cassette_lands"));
    for i in 0..HOOD_FILTER_CASSETTES {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_hood_filter_cassette_land_{i}"),
                86.0,
                46.0,
                7.0,
            )
            .translate(
                centered_index(i, HOOD_FILTER_CASSETTES, 116.0),
                HOOD_Y / 2.0 - 54.0,
                HOOD_Z / 2.0 + 3.5,
            );
    }
    lands
}

fn hood_service_door_handles() -> Part {
    let mut handles = Part::empty(format!("{OUTPUT_PREFIX}_hood_service_door_handles"));
    for i in 0..HOOD_SERVICE_DOORS {
        handles = handles
            + centered_cube(
                format!("{OUTPUT_PREFIX}_hood_side_service_door_handle_{i}"),
                10.0,
                62.0,
                12.0,
            )
            .translate(
                if i == 0 {
                    -HOOD_X / 2.0 - 5.0
                } else {
                    HOOD_X / 2.0 + 5.0
                },
                0.0,
                HOOD_Z / 2.0 - 58.0,
            );
    }
    handles
}

fn hood_internal_baffle_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_hood_internal_baffle_ribs"));
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_hood_internal_light_baffle_rib_{i}"),
                8.0,
                HOOD_Y - 64.0,
                28.0,
            )
            .translate(centered_index(i, 5, 88.0), 0.0, HOOD_Z / 2.0 - 48.0);
    }
    ribs
}

fn bubble_dead_volume_witness_windows() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_witness_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let cover_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_witness_window_cover_gasket_recess_cut"),
        WITNESS_X - 34.0,
        WITNESS_Y - 34.0,
        7.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0 - 3.5);

    body - cover_recess - witness_window_cuts()
        + witness_window_frames()
        + witness_level_ticks()
        + dead_volume_sweep_grooves()
        + bubble_challenge_injection_ports()
}

fn witness_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_cuts"));
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_witness_window_cut_{i}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(
                centered_index(i, BUBBLE_WITNESS_WINDOWS, WITNESS_PITCH_X),
                48.0,
                0.0,
            );
    }
    for i in 0..DEAD_VOLUME_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_witness_window_cut_{i}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_WITNESS_WINDOWS, WITNESS_PITCH_X),
                -52.0,
                0.0,
            );
    }
    cuts
}

fn witness_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_frames"));
    for i in 0..(BUBBLE_WITNESS_WINDOWS + DEAD_VOLUME_WITNESS_WINDOWS) {
        let bubble = i < BUBBLE_WITNESS_WINDOWS;
        let local = if bubble {
            i
        } else {
            i - BUBBLE_WITNESS_WINDOWS
        };
        let x = centered_index(local, BUBBLE_WITNESS_WINDOWS, WITNESS_PITCH_X);
        let y = if bubble { 48.0 } else { -52.0 };
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_witness_window_frame_{i}"),
            WITNESS_WINDOW_X + 10.0,
            WITNESS_WINDOW_Y + 10.0,
            4.0,
        )
        .translate(x, y, WITNESS_Z / 2.0 + 2.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_witness_window_frame_opening_{i}"),
            WITNESS_WINDOW_X + 1.0,
            WITNESS_WINDOW_Y + 1.0,
            4.6,
        )
        .translate(x, y, WITNESS_Z / 2.0 + 2.0);
        frames = frames + (frame - opening);
    }
    frames
}

fn witness_level_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_witness_level_ticks"));
    for window in 0..BUBBLE_WITNESS_WINDOWS {
        let x0 = centered_index(window, BUBBLE_WITNESS_WINDOWS, WITNESS_PITCH_X);
        for tick in 0..WITNESS_LEVEL_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_bubble_window_{window}_level_tick_{tick}"),
                    8.0,
                    2.0,
                    3.0,
                )
                .translate(
                    x0 - WITNESS_WINDOW_X / 2.0 + 5.0,
                    48.0 - WITNESS_WINDOW_Y / 2.0 + 4.0 + tick as f64 * 3.2,
                    WITNESS_Z / 2.0 + 1.5,
                );
        }
    }
    ticks
}

fn dead_volume_sweep_grooves() -> Part {
    let mut grooves = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_sweep_groove_marks"));
    for i in 0..DEAD_VOLUME_WITNESS_WINDOWS {
        grooves = grooves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_sweep_marker_{i}"),
                32.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_WITNESS_WINDOWS, WITNESS_PITCH_X),
                -92.0,
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    grooves
}

fn bubble_challenge_injection_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_bubble_challenge_injection_ports"));
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bubble_challenge_injection_port_boss_{i}"),
            8.0,
            6.0,
            24,
        )
        .translate(
            centered_index(i, BUBBLE_WITNESS_WINDOWS, WITNESS_PITCH_X),
            88.0,
            WITNESS_Z / 2.0 + 3.0,
        );
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bubble_challenge_injection_port_bore_{i}"),
            2.2,
            7.0,
            16,
        )
        .translate(
            centered_index(i, BUBBLE_WITNESS_WINDOWS, WITNESS_PITCH_X),
            88.0,
            WITNESS_Z / 2.0 + 3.0,
        );
        ports = ports + (boss - bore);
    }
    ports
}

fn waste_retain_split_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_waste_retain_split_manifold_body"),
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    );
    let service_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_waste_retain_split_service_recess_cut"),
        SPLIT_X - 58.0,
        SPLIT_Y - 64.0,
        10.0,
    )
    .translate(0.0, 0.0, SPLIT_Z / 2.0 - 5.0);

    body - service_recess - split_lane_bores() - retain_vial_well_cuts() - waste_socket_cuts()
        + split_valve_pads()
        + retain_vial_rims()
        + waste_socket_rims()
        + split_ratio_witness_tokens()
}

fn split_lane_bores() -> Part {
    let mut bores = Part::empty(format!("{OUTPUT_PREFIX}_waste_retain_split_lane_bores"));
    for i in 0..RETAIN_SPLIT_LANES {
        let y = centered_index(i, RETAIN_SPLIT_LANES, 70.0);
        bores = bores
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_waste_retain_split_lane_bore_{i}"),
                ROUTE_BORE_D / 2.0,
                SPLIT_X + 12.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -2.0);
    }
    bores
}

fn retain_vial_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_retain_vial_well_cuts"));
    for i in 0..RETAIN_VIALS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_retain_vial_well_cut_{i}"),
                RETAIN_VIAL_D / 2.0,
                SPLIT_Z + 4.0,
                34,
            )
            .translate(centered_index(i, RETAIN_VIALS, 52.0) - 44.0, -76.0, 0.0);
    }
    cuts
}

fn retain_vial_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_retain_vial_rims"));
    for i in 0..RETAIN_VIALS {
        let x = centered_index(i, RETAIN_VIALS, 52.0) - 44.0;
        let rim = centered_cylinder(
            format!("{OUTPUT_PREFIX}_retain_vial_rim_{i}"),
            RETAIN_VIAL_D / 2.0 + 4.0,
            4.0,
            34,
        )
        .translate(x, -76.0, SPLIT_Z / 2.0 + 2.0);
        let opening = centered_cylinder(
            format!("{OUTPUT_PREFIX}_retain_vial_rim_opening_{i}"),
            RETAIN_VIAL_D / 2.0 + 0.8,
            4.5,
            34,
        )
        .translate(x, -76.0, SPLIT_Z / 2.0 + 2.0);
        rims = rims + (rim - opening);
    }
    rims
}

fn waste_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_waste_socket_cuts"));
    for i in 0..WASTE_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_waste_socket_cut_{i}"),
                WASTE_SOCKET_D / 2.0,
                SPLIT_Z + 4.0,
                36,
            )
            .translate(centered_index(i, WASTE_PORTS, 72.0) + 58.0, 78.0, 0.0);
    }
    cuts
}

fn waste_socket_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_waste_socket_rims"));
    for i in 0..WASTE_PORTS {
        let x = centered_index(i, WASTE_PORTS, 72.0) + 58.0;
        let rim = centered_cylinder(
            format!("{OUTPUT_PREFIX}_waste_socket_rim_{i}"),
            WASTE_SOCKET_D / 2.0 + 5.0,
            4.0,
            36,
        )
        .translate(x, 78.0, SPLIT_Z / 2.0 + 2.0);
        let opening = centered_cylinder(
            format!("{OUTPUT_PREFIX}_waste_socket_rim_opening_{i}"),
            WASTE_SOCKET_D / 2.0 + 1.0,
            4.5,
            36,
        )
        .translate(x, 78.0, SPLIT_Z / 2.0 + 2.0);
        rims = rims + (rim - opening);
    }
    rims
}

fn split_valve_pads() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_waste_retain_split_valve_pads"));
    for i in 0..SPLIT_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_waste_retain_split_valve_pad_{i}"),
                34.0,
                22.0,
                10.0,
            )
            .translate(
                centered_index(i % 3, 3, 70.0),
                if i < 3 { 28.0 } else { -28.0 },
                SPLIT_Z / 2.0 + 5.0,
            );
    }
    valves
}

fn split_ratio_witness_tokens() -> Part {
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_split_ratio_witness_tokens"));
    for i in 0..6 {
        tokens = tokens
            + centered_cube(
                format!("{OUTPUT_PREFIX}_split_ratio_witness_token_land_{i}"),
                36.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(i, 6, 44.0),
                -SPLIT_Y / 2.0 + 18.0,
                SPLIT_Z / 2.0 + 2.0,
            );
    }
    tokens
}

fn barcode_custody_lands() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_board"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    board + barcode_land_array() + rfid_land_array() + tamper_seal_lands() + evidence_card_lands()
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_land_array"));
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 6, 6, 82.0);
        let y = 26.0 - (i / 6) as f64 * 52.0;
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_land_{i}"),
                62.0,
                18.0,
                4.0,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn rfid_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_land_array"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rfid_custody_land_{i}"),
                40.0,
                28.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 44.0,
                centered_index(i, RFID_LANDS, 26.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn tamper_seal_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_tamper_seal_lands"));
    for i in 0..CUSTODY_SEAL_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tamper_evident_seal_land_{i}"),
                32.0,
                12.0,
                4.0,
            )
            .translate(
                CUSTODY_X / 2.0 - 48.0,
                centered_index(i, CUSTODY_SEAL_LANDS, 18.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn evidence_card_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_evidence_card_lands"));
    for i in 0..EVIDENCE_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_run_evidence_card_land_{i}"),
                48.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(i, EVIDENCE_CARD_LANDS, 62.0),
                -CUSTODY_Y / 2.0 + 12.0,
                CUSTODY_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn release_hold_reject_gate_array() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_gate_panel"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    panel - gate_slot_cuts()
        + gate_lane_dividers()
        + gate_solenoid_pads()
        + decision_input_token_lands()
}

fn gate_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!(
        "{OUTPUT_PREFIX}_release_hold_reject_gate_slot_cuts"
    ));
    for (lane_index, lane_name) in DISPOSITION_LANES.iter().enumerate() {
        let x = centered_index(lane_index, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        for slot in 0..GATE_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_{lane_name}_gate_slot_cut_{slot}"),
                    70.0,
                    18.0,
                    GATE_Z + 4.0,
                )
                .translate(x, centered_index(slot, GATE_SLOTS_PER_LANE, 24.0), 0.0);
        }
    }
    cuts
}

fn gate_lane_dividers() -> Part {
    let mut dividers = Part::empty(format!("{OUTPUT_PREFIX}_gate_lane_dividers"));
    for i in 0..(DISPOSITION_LANES.len() - 1) {
        let x0 = centered_index(i, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        let x1 = centered_index(i + 1, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        dividers = dividers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_lane_divider_{i}"),
                8.0,
                GATE_Y - 22.0,
                18.0,
            )
            .translate((x0 + x1) / 2.0, 0.0, GATE_Z / 2.0 + 9.0);
    }
    dividers
}

fn gate_solenoid_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_gate_solenoid_pads"));
    for i in 0..GATE_SOLENOIDS {
        let lane = i / 2;
        let x = centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        let y = if i % 2 == 0 {
            GATE_Y / 2.0 - 20.0
        } else {
            -GATE_Y / 2.0 + 20.0
        };
        pads = pads
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_solenoid_pad_{i}"),
                44.0,
                18.0,
                9.0,
            )
            .translate(x, y, GATE_Z / 2.0 + 4.5);
    }
    pads
}

fn decision_input_token_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_decision_input_token_lands"));
    for i in 0..DECISION_INPUT_TOKENS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_decision_input_token_land_{i}"),
                26.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, DECISION_INPUT_TOKENS, 34.0),
                0.0,
                GATE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn camera_evidence_bridge_robotic_service_datums() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_camera_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    left_post
        + right_post
        + beam
        + camera_pods()
        + bridge_led_panels()
        + evidence_card_clamps()
        + robotic_service_datums()
        + service_keepout_gauges()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_camera_pods"));
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 245.0);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_evidence_camera_pod_{i}"),
            72.0,
            50.0,
            38.0,
        )
        .translate(x, -34.0, BRIDGE_UNDERSIDE_Z - 18.0);
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_evidence_camera_lens_clearance_{i}"),
            14.0,
            16.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -60.0, BRIDGE_UNDERSIDE_Z - 18.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn bridge_led_panels() -> Part {
    let mut panels = Part::empty(format!("{OUTPUT_PREFIX}_bridge_led_panels"));
    for i in 0..LED_PANEL_COUNT {
        panels = panels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_bridge_led_panel_{i}"),
                96.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, LED_PANEL_COUNT, 140.0),
                34.0,
                BRIDGE_UNDERSIDE_Z - 12.0,
            );
    }
    panels
}

fn evidence_card_clamps() -> Part {
    let mut clamps = Part::empty(format!("{OUTPUT_PREFIX}_evidence_card_clamps"));
    for i in 0..EVIDENCE_CARD_LANDS {
        clamps = clamps
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bridge_evidence_card_clamp_{i}"),
                48.0,
                8.0,
                10.0,
            )
            .translate(centered_index(i, EVIDENCE_CARD_LANDS, 72.0), -96.0, 36.0);
    }
    clamps
}

fn robotic_service_datums() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_robotic_service_datums"));
    for (i, &(x, y)) in robotic_datum_points().iter().enumerate() {
        datums = datums
            + fiducial_disc(&format!("{OUTPUT_PREFIX}_robotic_service_datum_{i}"))
                .translate(x, y, 3.0);
    }
    for i in 0..ROBOT_GRIPPER_DATUMS {
        datums = datums
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_robotic_gripper_cone_socket_witness_{i}"),
                13.0,
                6.0,
                32,
            )
            .translate(
                centered_index(i, ROBOT_GRIPPER_DATUMS, 116.0),
                -DECK_Y / 2.0 + 156.0,
                3.0,
            );
    }
    datums
}

fn service_keepout_gauges() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_reach_keepout_gauge"),
        DECK_X - 170.0,
        8.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + FRONT_ROBOT_CLEARANCE, 4.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_optics_service_keepout_gauge"),
        DECK_X - 170.0,
        8.0,
        8.0,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_OPTICS_SERVICE_CLEARANCE, 4.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_loop_service_keepout_gauge"),
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(-DECK_X / 2.0 + LEFT_LOOP_SERVICE_CLEARANCE, 0.0, 4.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_split_service_keepout_gauge"),
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(DECK_X / 2.0 - RIGHT_SPLIT_SERVICE_CLEARANCE, 0.0, 4.0);
    front + rear + left + right
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 14.0, 4.0, 36);
    let inner = centered_cylinder(format!("{name}_center_clearance"), 6.0, 4.5, 30);
    let cross_x = centered_cube(format!("{name}_cross_x"), 24.0, 3.0, 4.8);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 24.0, 4.8);
    outer - inner + cross_x + cross_y
}

fn module_specs() -> [ModuleSpec; 8] {
    [
        ModuleSpec {
            name: "sealed_sample_loop_cartridge_bank",
            center: LOOP_CENTER,
            x: LOOP_X,
            y: LOOP_Y,
            z: LOOP_Z,
        },
        ModuleSpec {
            name: "fluorescence_brightfield_window_banks",
            center: WINDOW_CENTER,
            x: WINDOW_X,
            y: WINDOW_Y,
            z: WINDOW_Z,
        },
        ModuleSpec {
            name: "live_dead_standard_well_rack",
            center: STANDARD_CENTER,
            x: STANDARD_X,
            y: STANDARD_Y,
            z: STANDARD_Z,
        },
        ModuleSpec {
            name: "dilution_flush_route_manifold",
            center: ROUTE_CENTER,
            x: ROUTE_X,
            y: ROUTE_Y,
            z: ROUTE_Z,
        },
        ModuleSpec {
            name: "bubble_dead_volume_witness_windows",
            center: WITNESS_CENTER,
            x: WITNESS_X,
            y: WITNESS_Y,
            z: WITNESS_Z,
        },
        ModuleSpec {
            name: "waste_retain_split_manifold",
            center: SPLIT_CENTER,
            x: SPLIT_X,
            y: SPLIT_Y,
            z: SPLIT_Z,
        },
        ModuleSpec {
            name: "barcode_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
            z: CUSTODY_Z,
        },
        ModuleSpec {
            name: "release_hold_reject_gate_array",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
            z: GATE_Z,
        },
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
}

fn deck_fiducial_points() -> [(f64, f64); 4] {
    [
        (-DECK_X / 2.0 + 104.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 104.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 104.0, DECK_Y / 2.0 - 104.0),
    ]
}

fn robotic_datum_points() -> [(f64, f64); ROBOT_SERVICE_DATUMS] {
    [
        (
            LOOP_CENTER.0 - LOOP_X / 2.0 + 40.0,
            LOOP_CENTER.1 + LOOP_Y / 2.0 - 38.0,
        ),
        (
            LOOP_CENTER.0 + LOOP_X / 2.0 - 40.0,
            LOOP_CENTER.1 + LOOP_Y / 2.0 - 38.0,
        ),
        (
            STANDARD_CENTER.0 - STANDARD_X / 2.0 + 36.0,
            STANDARD_CENTER.1 - STANDARD_Y / 2.0 + 36.0,
        ),
        (
            STANDARD_CENTER.0 + STANDARD_X / 2.0 - 36.0,
            STANDARD_CENTER.1 - STANDARD_Y / 2.0 + 36.0,
        ),
        (
            ROUTE_CENTER.0 - ROUTE_X / 2.0 + 40.0,
            ROUTE_CENTER.1 - ROUTE_Y / 2.0 + 38.0,
        ),
        (
            WITNESS_CENTER.0 + WITNESS_X / 2.0 - 40.0,
            WITNESS_CENTER.1 + WITNESS_Y / 2.0 - 38.0,
        ),
        (
            SPLIT_CENTER.0 - SPLIT_X / 2.0 + 38.0,
            SPLIT_CENTER.1 - SPLIT_Y / 2.0 + 38.0,
        ),
        (
            GATE_CENTER.0 + GATE_X / 2.0 - 38.0,
            GATE_CENTER.1 + GATE_Y / 2.0 - 30.0,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(SAMPLE_LOOP_PORTS, 12);
    assert_eq!(TOTAL_OPTICAL_WINDOWS, 12);
    assert_eq!(LIVE_DEAD_STANDARD_WELLS, 12);
    assert_eq!(DILUTION_COUPONS, 10);
    assert_eq!(BUBBLE_WITNESS_WINDOWS, DEAD_VOLUME_WITNESS_WINDOWS);
    assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
    assert_eq!(SERVICE_CLEARANCE_ZONES, 4);

    let modules = module_specs();
    for module in modules {
        assert!(
            module.fits_on_deck(),
            "{} exceeds station deck envelope",
            module.name
        );
        assert!(module.z > 0.0, "{} must have positive height", module.name);
    }
    for left in 0..modules.len() {
        for right in left + 1..modules.len() {
            assert!(
                !modules[left].overlaps(modules[right]),
                "{} overlaps {}",
                modules[left].name,
                modules[right].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_scoped_unique_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();

        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_cell_viability_inline_fluorescence_live_dead_crosscheck_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_secondary_containment_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn feature_manifest_covers_requested_station_intent() {
        for feature in [
            "sealed_sample_loop_cartridges",
            "fluorescence_window_bank",
            "brightfield_window_bank",
            "live_dead_standard_wells",
            "dilution_flush_routes",
            "light_shielded_imaging_hood",
            "bubble_witness_windows",
            "dead_volume_witness_windows",
            "waste_retain_split",
            "barcode_custody_lands",
            "release_hold_reject_gates",
            "camera_evidence_bridge",
            "robotic_service_datums",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }
        assert_eq!(REQUIRED_FEATURES.len(), 13);
    }

    #[test]
    fn deck_modules_fit_without_overlaps() {
        assert!(DECK_X <= 1500.0);
        assert!(DECK_Y <= 920.0);
        assert!(RIM_Z >= 40.0);
        assert_eq!(module_specs().len(), 8);
        assert_design_constraints();
    }

    #[test]
    fn sample_loop_and_optical_counts_are_pinned() {
        assert_eq!(SAMPLE_LOOP_CARTRIDGES, 6);
        assert_eq!(SAMPLE_LOOP_PORTS, SAMPLE_LOOP_CARTRIDGES * 2);
        assert_eq!(LOOP_PINCH_VALVES, 8);
        assert_eq!(LOOP_PRESSURE_TAPS, 4);
        assert_eq!(
            VIABILITY_CHANNELS,
            ["calcein_live", "ethidium_dead", "brightfield"]
        );
        assert_eq!(FLUORESCENCE_WINDOWS, 6);
        assert_eq!(BRIGHTFIELD_WINDOWS, 6);
        assert_eq!(TOTAL_OPTICAL_WINDOWS, 12);
    }

    #[test]
    fn standards_dilution_and_witness_counts_are_explicit() {
        assert_eq!(LIVE_DEAD_CONTROL_FAMILIES, 3);
        assert_eq!(LIVE_DEAD_CONTROL_LEVELS, 4);
        assert_eq!(
            LIVE_DEAD_STANDARD_WELLS,
            LIVE_DEAD_CONTROL_FAMILIES * LIVE_DEAD_CONTROL_LEVELS
        );
        assert_eq!(DILUTION_LEVELS, 5);
        assert_eq!(DILUTION_REPLICATES, 2);
        assert_eq!(DILUTION_COUPONS, 10);
        assert_eq!(FLUSH_ROUTES, 6);
        assert_eq!(BUBBLE_WITNESS_WINDOWS, 6);
        assert_eq!(DEAD_VOLUME_WITNESS_WINDOWS, 6);
        assert_eq!(WITNESS_LEVEL_TICKS, 9);
    }

    #[test]
    fn custody_split_and_release_gate_counts_are_physical() {
        assert_eq!(RETAIN_SPLIT_LANES, 2);
        assert_eq!(RETAIN_VIALS, 4);
        assert_eq!(WASTE_PORTS, 3);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(CUSTODY_SEAL_LANDS, 6);
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(GATE_SLOTS_PER_LANE, 4);
        assert_eq!(GATE_SOLENOIDS, 6);
        assert_eq!(DECISION_INPUT_TOKENS, 9);
    }

    #[test]
    fn evidence_bridge_and_robotic_service_datums_are_pinned() {
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(LED_PANEL_COUNT, 6);
        assert_eq!(ROBOT_SERVICE_DATUMS, robotic_datum_points().len());
        assert_eq!(ROBOT_GRIPPER_DATUMS, 4);
        assert_eq!(SERVICE_CLEARANCE_ZONES, 4);
        assert!(BRIDGE_UNDERSIDE_Z >= 230.0);
        assert!(FRONT_ROBOT_CLEARANCE >= 340.0);
        assert!(REAR_OPTICS_SERVICE_CLEARANCE >= 240.0);
    }
}
