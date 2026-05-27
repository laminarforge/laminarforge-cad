use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system cell-suspension density crosscheck station.
//
// Intent:
// - Validate one sealed cell-suspension sample loop before seeding by comparing
//   optical density/turbidity, impedance/counter, and gravimetric/volume
//   references in a single custody-controlled station.
// - Make dilution ladder coupons, optical path windows, counter dock envelope,
//   gravimetric reference pad, bubble/settling witness windows, barcode custody
//   lands, and release/hold/reject gates visible as deterministic CAD features.
// - Model validation fixture interfaces only. Density calibration curves,
//   biological acceptance thresholds, and batch disposition logic remain
//   external quality-system data.

const OUTPUT_PREFIX: &str = "closed_cell_suspension_optical_density_counter_crosscheck_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_leak_tray_deck.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_sealed_sample_loop_manifold.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_dilution_ladder_coupon_bank.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_optical_density_turbidity_window_lane.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_impedance_counter_dock_envelope.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_gravimetric_volume_reference_pad.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_bubble_settling_witness_window_bank.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_barcode_custody_lands.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_release_hold_reject_gate_array.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_closed_sample_loop_route_harness.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_robot_service_keepouts.stl",
    "output/closed_cell_suspension_optical_density_counter_crosscheck_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "sealed_sample_loop_manifold",
    "closed_sterile_transfer_ports",
    "dilution_ladder_coupon_bank",
    "optical_density_turbidity_window_lane",
    "impedance_counter_dock_envelope",
    "gravimetric_volume_reference_pad",
    "bubble_settling_witness_window_bank",
    "barcode_custody_lands",
    "release_hold_reject_gate_array",
    "closed_sample_loop_route_harness",
    "reference_standard_lands",
    "cross_method_correlation_token_lands",
    "sample_loop_pressure_taps",
    "robot_service_keepouts",
];

const MEASUREMENT_METHODS: [&str; 3] = [
    "optical_density_turbidity",
    "impedance_counter",
    "gravimetric_volume",
];
const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 38.0;
const SOCKET_DEPTH: f64 = 5.0;
const SUMP_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_TARGETS: usize = 4;
const LEAK_WITNESS_WELLS: usize = 12;

const LOOP_CENTER: (f64, f64) = (-420.0, 205.0);
const LOOP_X: f64 = 420.0;
const LOOP_Y: f64 = 250.0;
const LOOP_Z: f64 = 56.0;
const SAMPLE_LOOP_PORTS: usize = 8;
const LOOP_PINCH_VALVES: usize = 8;
const LOOP_PRESSURE_TAPS: usize = 4;
const LOOP_SAMPLE_BRANCHES: usize = 4;
const LOOP_TUBE_D: f64 = 7.2;
const LOOP_BORE_D: f64 = 5.0;

const DILUTION_CENTER: (f64, f64) = (40.0, 205.0);
const DILUTION_X: f64 = 440.0;
const DILUTION_Y: f64 = 250.0;
const DILUTION_Z: f64 = 44.0;
const DILUTION_LEVELS: usize = 5;
const COUPONS_PER_LEVEL: usize = 2;
const DILUTION_COUPON_COUNT: usize = DILUTION_LEVELS * COUPONS_PER_LEVEL;
const DILUTION_COUPON_X: f64 = 56.0;
const DILUTION_COUPON_Y: f64 = 64.0;
const DILUTION_LEVEL_PITCH_X: f64 = 76.0;
const DILUTION_REPLICATE_PITCH_Y: f64 = 74.0;
const DILUTION_MIXING_WELLS: usize = DILUTION_LEVELS;
const DILUTION_BARCODE_LANDS: usize = DILUTION_COUPON_COUNT;

const OPTICAL_CENTER: (f64, f64) = (475.0, 205.0);
const OPTICAL_X: f64 = 350.0;
const OPTICAL_Y: f64 = 250.0;
const OPTICAL_Z: f64 = 60.0;
const OPTICAL_PATH_WINDOWS: usize = 6;
const OPTICAL_PATH_LENGTH_MM: f64 = 10.0;
const TURBIDITY_STANDARD_WELLS: usize = 4;
const LIGHT_PIPE_BARS: usize = 2;
const DARK_REFERENCE_SHUTTERS: usize = 2;

const COUNTER_CENTER: (f64, f64) = (-420.0, -115.0);
const COUNTER_X: f64 = 420.0;
const COUNTER_Y: f64 = 250.0;
const COUNTER_Z: f64 = 70.0;
const COUNTER_DOCKS: usize = 2;
const IMPEDANCE_ELECTRODE_PAIRS: usize = 6;
const COUNTER_FLUID_PORTS: usize = 4;
const COUNTER_CABLE_GLANDS: usize = 2;
const COUNTER_DATUM_PINS: usize = 4;

const GRAV_CENTER: (f64, f64) = (40.0, -115.0);
const GRAV_X: f64 = 420.0;
const GRAV_Y: f64 = 250.0;
const GRAV_Z: f64 = 50.0;
const SCALE_PAD_X: f64 = 300.0;
const SCALE_PAD_Y: f64 = 126.0;
const VOLUME_REFERENCE_WELLS: usize = 6;
const GRAVIMETRIC_TARE_LANDS: usize = 4;
const GRAVIMETRIC_ISOLATION_FEET: usize = 4;
const VOLUME_STANDARD_STOPS: usize = 3;

const WITNESS_CENTER: (f64, f64) = (475.0, -115.0);
const WITNESS_X: f64 = 350.0;
const WITNESS_Y: f64 = 250.0;
const WITNESS_Z: f64 = 46.0;
const BUBBLE_WITNESS_WINDOWS: usize = 4;
const SETTLING_WITNESS_WINDOWS: usize = 5;
const SETTLING_TIME_TOKENS: [usize; 5] = [0, 2, 5, 10, 20];
const WITNESS_LEVEL_TICKS: usize = 9;

const CUSTODY_CENTER: (f64, f64) = (-255.0, -340.0);
const CUSTODY_X: f64 = 540.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 24.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 4;
const CUSTODY_SEAL_LANDS: usize = 6;
const METHOD_TOKEN_LANDS: usize = MEASUREMENT_METHODS.len();

const GATE_CENTER: (f64, f64) = (330.0, -340.0);
const GATE_X: f64 = 500.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 50.0;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SOLENOIDS: usize = DISPOSITION_LANES.len() * 2;
const DECISION_INPUTS: usize = MEASUREMENT_METHODS.len() * 3;
const GATE_LANE_PITCH_X: f64 = 150.0;

const ROUTE_Z: f64 = DECK_Z + 86.0;
const ROUTE_TUBE_D: f64 = 7.0;
const ROUTE_SEGMENTS: usize = 10;
const ROUTE_ELBOWS: usize = 10;
const ROUTE_DIRECTION_MARKERS: usize = 8;
const BYPASS_BRANCHES: usize = 3;

const KEEP_OUT_X: f64 = 1440.0;
const KEEP_OUT_Y: f64 = 840.0;
const FRONT_ROBOT_CLEARANCE: f64 = 350.0;
const REAR_OPTICAL_SERVICE_CLEARANCE: f64 = 230.0;
const LEFT_COUNTER_SERVICE_CLEARANCE: f64 = 240.0;
const RIGHT_WINDOW_SERVICE_CLEARANCE: f64 = 210.0;
const TOP_SAMPLE_LOOP_LIFT_CLEARANCE: f64 = 300.0;
const KEEP_OUT_ZONES: usize = 5;

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

    let deck = leak_tray_deck();
    write_part(OUTPUTS[0], &deck);

    let loop_manifold = sealed_sample_loop_manifold();
    write_part(OUTPUTS[1], &loop_manifold);

    let dilution = dilution_ladder_coupon_bank();
    write_part(OUTPUTS[2], &dilution);

    let optical = optical_density_turbidity_window_lane();
    write_part(OUTPUTS[3], &optical);

    let counter = impedance_counter_dock_envelope();
    write_part(OUTPUTS[4], &counter);

    let gravimetric = gravimetric_volume_reference_pad();
    write_part(OUTPUTS[5], &gravimetric);

    let witnesses = bubble_settling_witness_window_bank();
    write_part(OUTPUTS[6], &witnesses);

    let custody = barcode_custody_lands();
    write_part(OUTPUTS[7], &custody);

    let gates = release_hold_reject_gate_array();
    write_part(OUTPUTS[8], &gates);

    let routes = closed_sample_loop_route_harness();
    write_part(OUTPUTS[9], &routes);

    let keepouts = robot_service_keepouts();
    write_part(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cell-suspension density crosscheck station:");
    println!(
        "  Footprint: {DECK_X:.0}mm x {DECK_Y:.0}mm contained leak tray with {LEAK_WITNESS_WELLS} witness wells"
    );
    println!(
        "  Sample loop: {SAMPLE_LOOP_PORTS} sterile ports, {LOOP_PINCH_VALVES} pinch valves, {LOOP_PRESSURE_TAPS} pressure taps, {ROUTE_SEGMENTS} routed loop segments"
    );
    println!(
        "  Crosscheck methods: optical OD/turbidity ({OPTICAL_PATH_WINDOWS} windows, {OPTICAL_PATH_LENGTH_MM:.0}mm path), impedance/counter ({COUNTER_DOCKS} docks, {IMPEDANCE_ELECTRODE_PAIRS} electrode pairs), gravimetric/volume ({VOLUME_REFERENCE_WELLS} wells)"
    );
    println!(
        "  References: {DILUTION_COUPON_COUNT} dilution coupons across {DILUTION_LEVELS} levels, {BUBBLE_WITNESS_WINDOWS} bubble windows, {SETTLING_WITNESS_WINDOWS} settling windows"
    );
    println!(
        "  Custody/disposition: {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {} lanes ({}) with {} gate slots",
        DISPOSITION_LANES.len(),
        DISPOSITION_LANES.join(", "),
        DISPOSITION_LANES.len() * GATE_SLOTS_PER_LANE
    );
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_deck()
        + sealed_sample_loop_manifold().translate(LOOP_CENTER.0, LOOP_CENTER.1, on_deck_z(LOOP_Z))
        + dilution_ladder_coupon_bank().translate(
            DILUTION_CENTER.0,
            DILUTION_CENTER.1,
            on_deck_z(DILUTION_Z),
        )
        + optical_density_turbidity_window_lane().translate(
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
            on_deck_z(OPTICAL_Z),
        )
        + impedance_counter_dock_envelope().translate(
            COUNTER_CENTER.0,
            COUNTER_CENTER.1,
            on_deck_z(COUNTER_Z),
        )
        + gravimetric_volume_reference_pad().translate(
            GRAV_CENTER.0,
            GRAV_CENTER.1,
            on_deck_z(GRAV_Z),
        )
        + bubble_settling_witness_window_bank().translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            on_deck_z(WITNESS_Z),
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
        + closed_sample_loop_route_harness()
        + robot_service_keepouts()
}

fn on_deck_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn leak_tray_deck() -> Part {
    let floor = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_floor"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_containment_sump_cut"),
        DECK_X - 148.0,
        DECK_Y - 150.0,
        SUMP_DEPTH + 0.8,
    )
    .translate(0.0, -6.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.4);
    let front_drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_slot_cut"),
        DECK_X - 260.0,
        14.0,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 70.0, DECK_Z - SUMP_DEPTH / 2.0);

    floor - sump - front_drain - insert_socket_cuts() - deck_mount_slots()
        + perimeter_rims()
        + method_zone_dividers()
        + module_floor_markers()
        + deck_datum_targets()
        + leak_witness_well_bank()
}

fn insert_socket_cuts() -> Part {
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

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_slots"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_m6_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_m6_mount_slot_relief_{i}"),
            26.0,
            7.2,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
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
    let front_lip = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_drain_lip"),
        DECK_X - 180.0,
        14.0,
        22.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 11.0);

    left + right + rear + front_lip
}

fn method_zone_dividers() -> Part {
    let top_to_middle = centered_cube(
        format!("{OUTPUT_PREFIX}_sample_loop_to_counter_reference_divider"),
        DECK_X - 164.0,
        10.0,
        28.0,
    )
    .translate(0.0, 45.0, DECK_Z + 14.0);
    let middle_to_custody = centered_cube(
        format!("{OUTPUT_PREFIX}_counter_reference_to_custody_divider"),
        DECK_X - 210.0,
        10.0,
        26.0,
    )
    .translate(0.0, -252.0, DECK_Z + 13.0);
    let optical_to_dilution = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_to_dilution_light_baffle_divider"),
        10.0,
        256.0,
        28.0,
    )
    .translate(282.0, 205.0, DECK_Z + 14.0);
    let gravimetric_to_witness = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_to_witness_splash_divider"),
        10.0,
        248.0,
        26.0,
    )
    .translate(282.0, -115.0, DECK_Z + 13.0);

    top_to_middle + middle_to_custody + optical_to_dilution + gravimetric_to_witness
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for module in module_specs() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", module.name),
                module.x + 12.0,
                module.y + 12.0,
                2.4,
            )
            .translate(module.center.0, module.center.1, DECK_Z + 1.2);
    }
    markers
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_vision_datum_targets"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 102.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 102.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 102.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 102.0, DECK_Y / 2.0 - 104.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("{OUTPUT_PREFIX}_datum_target_{i}")).translate(
                *x,
                *y,
                DECK_Z + 2.0,
            );
    }
    targets
}

fn leak_witness_well_bank() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_witness_well_bank"));
    for i in 0..LEAK_WITNESS_WELLS {
        let x = centered_index(i % 6, 6, 72.0);
        let y = -DECK_Y / 2.0 + 106.0 + (i / 6) as f64 * 34.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_loop_leak_witness_well_{i}"),
                10.0,
                6.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.0);
    }
    wells
}

fn sealed_sample_loop_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_sample_loop_body"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let central_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_loop_service_recess_cut"),
        LOOP_X - 92.0,
        LOOP_Y - 86.0,
        14.0,
    )
    .translate(0.0, 2.0, LOOP_Z / 2.0 - 7.0);

    body - central_recess
        + closed_sterile_transfer_ports()
        + sample_loop_tube_race()
        + sample_loop_pinch_valves()
        + sample_loop_pressure_taps()
        + sample_branch_takeoffs()
        + loop_flow_direction_markers()
}

fn closed_sterile_transfer_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_closed_sterile_transfer_ports"));
    for i in 0..SAMPLE_LOOP_PORTS {
        let top_side = i < SAMPLE_LOOP_PORTS / 2;
        let x = centered_index(i % (SAMPLE_LOOP_PORTS / 2), SAMPLE_LOOP_PORTS / 2, 74.0);
        let y = if top_side {
            LOOP_Y / 2.0 - 28.0
        } else {
            -LOOP_Y / 2.0 + 28.0
        };
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sterile_transfer_port_boss_{i}"),
            16.0,
            16.0,
            32,
        )
        .translate(x, y, LOOP_Z / 2.0 + 8.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sterile_transfer_port_bore_{i}"),
            LOOP_BORE_D / 2.0,
            18.0,
            20,
        )
        .translate(x, y, LOOP_Z / 2.0 + 9.0);
        let cap_land = centered_cube(
            format!("{OUTPUT_PREFIX}_sterile_transfer_port_cap_land_{i}"),
            34.0,
            18.0,
            7.0,
        )
        .translate(
            x,
            y + if top_side { -28.0 } else { 28.0 },
            LOOP_Z / 2.0 + 5.0,
        );
        ports = ports + (boss - bore) + cap_land;
    }
    ports
}

fn sample_loop_tube_race() -> Part {
    let left = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sealed_loop_left_race"),
        LOOP_TUBE_D / 2.0,
        LOOP_Y - 82.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-LOOP_X / 2.0 + 66.0, 0.0, LOOP_Z / 2.0 + 18.0);
    let right = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sealed_loop_right_race"),
        LOOP_TUBE_D / 2.0,
        LOOP_Y - 82.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LOOP_X / 2.0 - 66.0, 0.0, LOOP_Z / 2.0 + 18.0);
    let rear = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sealed_loop_rear_race"),
        LOOP_TUBE_D / 2.0,
        LOOP_X - 132.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOOP_Y / 2.0 - 58.0, LOOP_Z / 2.0 + 18.0);
    let front = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sealed_loop_front_race"),
        LOOP_TUBE_D / 2.0,
        LOOP_X - 132.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -LOOP_Y / 2.0 + 58.0, LOOP_Z / 2.0 + 18.0);

    left + right + rear + front
}

fn sample_loop_pinch_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pinch_valves"));
    for i in 0..LOOP_PINCH_VALVES {
        let x = if i < 4 {
            centered_index(i, 4, 72.0)
        } else if i % 2 == 0 {
            -LOOP_X / 2.0 + 92.0
        } else {
            LOOP_X / 2.0 - 92.0
        };
        let y = if i < 4 {
            0.0
        } else {
            centered_index((i - 4) / 2, 2, 64.0)
        };
        let valve = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_{i}"),
            42.0,
            24.0,
            24.0,
        )
        .translate(x, y, LOOP_Z / 2.0 + 12.0);
        let tube_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_tube_clearance_{i}"),
            LOOP_TUBE_D / 2.0,
            46.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, LOOP_Z / 2.0 + 12.0);
        valves = valves + (valve - tube_cut);
    }
    valves
}

fn sample_loop_pressure_taps() -> Part {
    let mut taps = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pressure_taps"));
    for i in 0..LOOP_PRESSURE_TAPS {
        taps = taps
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sample_loop_pressure_tap_{i}"),
                8.0,
                18.0,
                28,
            )
            .translate(
                centered_index(i, LOOP_PRESSURE_TAPS, 74.0),
                -28.0,
                LOOP_Z / 2.0 + 9.0,
            );
    }
    taps
}

fn sample_branch_takeoffs() -> Part {
    let mut branches = Part::empty(format!("{OUTPUT_PREFIX}_sample_branch_takeoffs"));
    for i in 0..LOOP_SAMPLE_BRANCHES {
        let y = centered_index(i, LOOP_SAMPLE_BRANCHES, 38.0);
        let block = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_branch_takeoff_block_{i}"),
            34.0,
            18.0,
            16.0,
        )
        .translate(0.0, y, LOOP_Z / 2.0 + 8.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_branch_takeoff_bore_{i}"),
            LOOP_BORE_D / 2.0,
            40.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, LOOP_Z / 2.0 + 8.0);
        branches = branches + (block - bore);
    }
    branches
}

fn loop_flow_direction_markers() -> Part {
    flow_arrow_marker(
        format!("{OUTPUT_PREFIX}_sample_loop_forward_arrow"),
        1.0,
        0.0,
    )
    .translate(-98.0, LOOP_Y / 2.0 - 54.0, LOOP_Z / 2.0 + 5.0)
        + flow_arrow_marker(
            format!("{OUTPUT_PREFIX}_sample_loop_return_arrow"),
            -1.0,
            0.0,
        )
        .translate(98.0, -LOOP_Y / 2.0 + 54.0, LOOP_Z / 2.0 + 5.0)
}

fn dilution_ladder_coupon_bank() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_dilution_ladder_tray"),
        DILUTION_X,
        DILUTION_Y,
        DILUTION_Z,
    );
    let wash_basin = centered_cube(
        format!("{OUTPUT_PREFIX}_dilution_ladder_wash_basin_cut"),
        DILUTION_X - 58.0,
        DILUTION_Y - 52.0,
        11.0,
    )
    .translate(0.0, 0.0, DILUTION_Z / 2.0 - 5.5);

    tray - wash_basin - dilution_coupon_pocket_cuts()
        + dilution_coupon_frames()
        + dilution_mixing_wells()
        + dilution_ladder_ratio_ticks()
        + dilution_barcode_lands()
        + dilution_sealed_cap_bridge()
}

fn dilution_coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_dilution_coupon_pocket_cuts"));
    for level in 0..DILUTION_LEVELS {
        for replicate in 0..COUPONS_PER_LEVEL {
            let index = level * COUPONS_PER_LEVEL + replicate;
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_dilution_coupon_pocket_cut_{index}"),
                    DILUTION_COUPON_X,
                    DILUTION_COUPON_Y,
                    16.0,
                )
                .translate(
                    centered_index(level, DILUTION_LEVELS, DILUTION_LEVEL_PITCH_X),
                    centered_index(replicate, COUPONS_PER_LEVEL, DILUTION_REPLICATE_PITCH_Y),
                    DILUTION_Z / 2.0 - 6.0,
                );
        }
    }
    cuts
}

fn dilution_coupon_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_dilution_coupon_frames"));
    for level in 0..DILUTION_LEVELS {
        for replicate in 0..COUPONS_PER_LEVEL {
            let index = level * COUPONS_PER_LEVEL + replicate;
            let x = centered_index(level, DILUTION_LEVELS, DILUTION_LEVEL_PITCH_X);
            let y = centered_index(replicate, COUPONS_PER_LEVEL, DILUTION_REPLICATE_PITCH_Y);
            let frame = centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_coupon_frame_{index}"),
                DILUTION_COUPON_X + 16.0,
                DILUTION_COUPON_Y + 16.0,
                8.0,
            )
            .translate(x, y, DILUTION_Z / 2.0 + 4.0);
            let opening = centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_coupon_window_cut_{index}"),
                DILUTION_COUPON_X,
                DILUTION_COUPON_Y,
                9.0,
            )
            .translate(x, y, DILUTION_Z / 2.0 + 4.5);
            frames = frames + (frame - opening);
        }
    }
    frames
}

fn dilution_mixing_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_dilution_mixing_wells"));
    for i in 0..DILUTION_MIXING_WELLS {
        let x = centered_index(i, DILUTION_MIXING_WELLS, DILUTION_LEVEL_PITCH_X);
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sealed_dilution_mixing_well_{i}"),
                13.0,
                12.0,
                32,
            )
            .translate(x, DILUTION_Y / 2.0 - 30.0, DILUTION_Z / 2.0 + 6.0);
    }
    wells
}

fn dilution_ladder_ratio_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_dilution_ladder_ratio_ticks"));
    for i in 0..DILUTION_LEVELS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_ratio_tick_{i}"),
                36.0,
                6.0,
                5.0,
            )
            .translate(
                centered_index(i, DILUTION_LEVELS, DILUTION_LEVEL_PITCH_X),
                -DILUTION_Y / 2.0 + 22.0,
                DILUTION_Z / 2.0 + 2.5,
            );
    }
    ticks
}

fn dilution_barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_dilution_coupon_barcode_lands"));
    for i in 0..DILUTION_BARCODE_LANDS {
        let level = i / COUPONS_PER_LEVEL;
        let replicate = i % COUPONS_PER_LEVEL;
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_coupon_barcode_land_{i}"),
                42.0,
                10.0,
                3.0,
            )
            .translate(
                centered_index(level, DILUTION_LEVELS, DILUTION_LEVEL_PITCH_X),
                centered_index(replicate, COUPONS_PER_LEVEL, DILUTION_REPLICATE_PITCH_Y) - 42.0,
                DILUTION_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn dilution_sealed_cap_bridge() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_dilution_ladder_sealed_cap_bridge"),
        DILUTION_X - 62.0,
        18.0,
        22.0,
    )
    .translate(0.0, DILUTION_Y / 2.0 - 56.0, DILUTION_Z / 2.0 + 11.0);
    let mut latches = Part::empty(format!("{OUTPUT_PREFIX}_dilution_ladder_cap_latches"));
    for i in 0..DILUTION_LEVELS {
        latches = latches
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dilution_ladder_cap_latch_{i}"),
                18.0,
                32.0,
                18.0,
            )
            .translate(
                centered_index(i, DILUTION_LEVELS, DILUTION_LEVEL_PITCH_X),
                DILUTION_Y / 2.0 - 56.0,
                DILUTION_Z / 2.0 + 19.0,
            );
    }
    rail + latches
}

fn optical_density_turbidity_window_lane() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_lane_body"),
        OPTICAL_X,
        OPTICAL_Y,
        OPTICAL_Z,
    );
    let optical_tunnel = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_dark_tunnel_cut"),
        OPTICAL_X - 84.0,
        66.0,
        24.0,
    )
    .translate(0.0, 0.0, OPTICAL_Z / 2.0 - 6.0);

    body - optical_tunnel - optical_window_cuts()
        + optical_window_frames()
        + turbidity_standard_wells()
        + optical_light_pipe_bars()
        + dark_reference_shutters()
        + optical_path_length_gauges()
}

fn optical_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_optical_path_window_cuts"));
    for i in 0..OPTICAL_PATH_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_path_window_cut_{i}"),
                26.0,
                42.0,
                OPTICAL_Z + 3.0,
            )
            .translate(centered_index(i, OPTICAL_PATH_WINDOWS, 44.0), -16.0, 0.0);
    }
    cuts
}

fn optical_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_optical_path_window_frames"));
    for i in 0..OPTICAL_PATH_WINDOWS {
        let x = centered_index(i, OPTICAL_PATH_WINDOWS, 44.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_path_window_frame_{i}"),
            40.0,
            56.0,
            8.0,
        )
        .translate(x, -16.0, OPTICAL_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_path_window_frame_opening_{i}"),
            26.0,
            42.0,
            9.0,
        )
        .translate(x, -16.0, OPTICAL_Z / 2.0 + 4.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn turbidity_standard_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_turbidity_standard_wells"));
    for i in 0..TURBIDITY_STANDARD_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sealed_turbidity_standard_well_{i}"),
                14.0,
                14.0,
                32,
            )
            .translate(
                centered_index(i, TURBIDITY_STANDARD_WELLS, 54.0),
                OPTICAL_Y / 2.0 - 42.0,
                OPTICAL_Z / 2.0 + 7.0,
            );
    }
    wells
}

fn optical_light_pipe_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_optical_light_pipe_bars"));
    for i in 0..LIGHT_PIPE_BARS {
        let y = if i == 0 { -70.0 } else { 38.0 };
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_light_pipe_bar_{i}"),
                OPTICAL_X - 96.0,
                14.0,
                18.0,
            )
            .translate(0.0, y, OPTICAL_Z / 2.0 + 9.0);
    }
    bars
}

fn dark_reference_shutters() -> Part {
    let mut shutters = Part::empty(format!("{OUTPUT_PREFIX}_dark_reference_shutters"));
    for i in 0..DARK_REFERENCE_SHUTTERS {
        shutters = shutters
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dark_reference_shutter_{i}"),
                42.0,
                18.0,
                20.0,
            )
            .translate(
                if i == 0 {
                    -OPTICAL_X / 2.0 + 48.0
                } else {
                    OPTICAL_X / 2.0 - 48.0
                },
                -70.0,
                OPTICAL_Z / 2.0 + 10.0,
            );
    }
    shutters
}

fn optical_path_length_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_optical_path_length_gauges"));
    for i in 0..OPTICAL_PATH_WINDOWS {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_path_{OPTICAL_PATH_LENGTH_MM:.0}mm_gauge_{i}"),
                4.0,
                OPTICAL_PATH_LENGTH_MM,
                10.0,
            )
            .translate(
                centered_index(i, OPTICAL_PATH_WINDOWS, 44.0) + 19.0,
                28.0,
                OPTICAL_Z / 2.0 + 5.0,
            );
    }
    gauges
}

fn impedance_counter_dock_envelope() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_impedance_counter_dock_body"),
        COUNTER_X,
        COUNTER_Y,
        COUNTER_Z,
    );
    let dock_relief = centered_cube(
        format!("{OUTPUT_PREFIX}_counter_dock_service_relief_cut"),
        COUNTER_X - 80.0,
        COUNTER_Y - 70.0,
        18.0,
    )
    .translate(0.0, 0.0, COUNTER_Z / 2.0 - 9.0);

    body - dock_relief - counter_dock_socket_cuts()
        + counter_dock_frames()
        + impedance_electrode_pair_lands()
        + counter_fluid_port_saddles()
        + counter_cable_glands()
        + counter_datum_pins()
}

fn counter_dock_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_counter_dock_socket_cuts"));
    for i in 0..COUNTER_DOCKS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_counter_cartridge_socket_cut_{i}"),
                134.0,
                118.0,
                26.0,
            )
            .translate(
                centered_index(i, COUNTER_DOCKS, 176.0),
                12.0,
                COUNTER_Z / 2.0 - 9.0,
            );
    }
    cuts
}

fn counter_dock_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_counter_dock_frames"));
    for i in 0..COUNTER_DOCKS {
        let x = centered_index(i, COUNTER_DOCKS, 176.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_counter_dock_frame_{i}"),
            154.0,
            138.0,
            10.0,
        )
        .translate(x, 12.0, COUNTER_Z / 2.0 + 5.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_counter_dock_frame_opening_cut_{i}"),
            134.0,
            118.0,
            11.0,
        )
        .translate(x, 12.0, COUNTER_Z / 2.0 + 5.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn impedance_electrode_pair_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_impedance_electrode_pair_lands"));
    for dock in 0..COUNTER_DOCKS {
        let dock_x = centered_index(dock, COUNTER_DOCKS, 176.0);
        for pair in 0..IMPEDANCE_ELECTRODE_PAIRS {
            let y = centered_index(pair, IMPEDANCE_ELECTRODE_PAIRS, 18.0) + 12.0;
            lands = lands
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_dock_{dock}_impedance_electrode_pair_{pair}"),
                    42.0,
                    5.0,
                    4.0,
                )
                .translate(dock_x - 48.0, y, COUNTER_Z / 2.0 + 2.0)
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_dock_{dock}_counter_aperture_land_{pair}"),
                    18.0,
                    6.0,
                    4.0,
                )
                .translate(dock_x + 48.0, y, COUNTER_Z / 2.0 + 2.0);
        }
    }
    lands
}

fn counter_fluid_port_saddles() -> Part {
    let mut saddles = Part::empty(format!("{OUTPUT_PREFIX}_counter_fluid_port_saddles"));
    for i in 0..COUNTER_FLUID_PORTS {
        let x = centered_index(i, COUNTER_FLUID_PORTS, 72.0);
        let saddle = centered_cube(
            format!("{OUTPUT_PREFIX}_counter_fluid_port_saddle_{i}"),
            38.0,
            18.0,
            18.0,
        )
        .translate(x, -COUNTER_Y / 2.0 + 28.0, COUNTER_Z / 2.0 + 9.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_counter_fluid_port_bore_{i}"),
            LOOP_BORE_D / 2.0,
            42.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -COUNTER_Y / 2.0 + 28.0, COUNTER_Z / 2.0 + 9.0);
        saddles = saddles + (saddle - bore);
    }
    saddles
}

fn counter_cable_glands() -> Part {
    let mut glands = Part::empty(format!("{OUTPUT_PREFIX}_counter_cable_glands"));
    for i in 0..COUNTER_CABLE_GLANDS {
        glands = glands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_counter_cable_gland_{i}"),
                11.0,
                18.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                if i == 0 {
                    -COUNTER_X / 2.0 + 30.0
                } else {
                    COUNTER_X / 2.0 - 30.0
                },
                COUNTER_Y / 2.0 - 42.0,
                COUNTER_Z / 2.0 + 14.0,
            );
    }
    glands
}

fn counter_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_counter_datum_pins"));
    for (i, (x, y)) in [
        (-COUNTER_X / 2.0 + 44.0, -COUNTER_Y / 2.0 + 44.0),
        (COUNTER_X / 2.0 - 44.0, -COUNTER_Y / 2.0 + 44.0),
        (-COUNTER_X / 2.0 + 44.0, COUNTER_Y / 2.0 - 44.0),
        (COUNTER_X / 2.0 - 44.0, COUNTER_Y / 2.0 - 44.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_counter_datum_pin_{i}"),
                5.0,
                12.0,
                24,
            )
            .translate(*x, *y, COUNTER_Z / 2.0 + 6.0);
    }
    pins
}

fn gravimetric_volume_reference_pad() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_reference_base"),
        GRAV_X,
        GRAV_Y,
        GRAV_Z,
    );
    let scale_socket = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_scale_pad_socket_cut"),
        SCALE_PAD_X,
        SCALE_PAD_Y,
        12.0,
    )
    .translate(-18.0, 10.0, GRAV_Z / 2.0 - 6.0);
    let spill_moat = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_reference_spill_moat_cut"),
        GRAV_X - 62.0,
        GRAV_Y - 62.0,
        7.0,
    )
    .translate(0.0, 0.0, GRAV_Z / 2.0 - 3.5);

    base - scale_socket - spill_moat
        + gravimetric_scale_pad_frame()
        + volume_reference_wells()
        + tare_weight_lands()
        + gravimetric_isolation_feet()
        + volume_standard_stops()
        + gravimetric_closed_port_comb()
}

fn gravimetric_scale_pad_frame() -> Part {
    let frame = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_scale_pad_frame"),
        SCALE_PAD_X + 22.0,
        SCALE_PAD_Y + 22.0,
        9.0,
    )
    .translate(-18.0, 10.0, GRAV_Z / 2.0 + 4.5);
    let opening = centered_cube(
        format!("{OUTPUT_PREFIX}_gravimetric_scale_pad_opening_cut"),
        SCALE_PAD_X,
        SCALE_PAD_Y,
        10.0,
    )
    .translate(-18.0, 10.0, GRAV_Z / 2.0 + 5.0);
    frame - opening
}

fn volume_reference_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_volume_reference_wells"));
    for i in 0..VOLUME_REFERENCE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sealed_volume_reference_well_{i}"),
                13.0,
                13.0,
                32,
            )
            .translate(
                GRAV_X / 2.0 - 58.0,
                centered_index(i, VOLUME_REFERENCE_WELLS, 30.0),
                GRAV_Z / 2.0 + 6.5,
            );
    }
    wells
}

fn tare_weight_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_gravimetric_tare_weight_lands"));
    for i in 0..GRAVIMETRIC_TARE_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gravimetric_tare_land_{i}"),
                44.0,
                26.0,
                6.0,
            )
            .translate(
                -GRAV_X / 2.0 + 54.0,
                centered_index(i, GRAVIMETRIC_TARE_LANDS, 46.0),
                GRAV_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn gravimetric_isolation_feet() -> Part {
    let mut feet = Part::empty(format!("{OUTPUT_PREFIX}_gravimetric_isolation_feet"));
    for (i, (x, y)) in [
        (-SCALE_PAD_X / 2.0 + 34.0, -SCALE_PAD_Y / 2.0 + 26.0),
        (SCALE_PAD_X / 2.0 - 34.0, -SCALE_PAD_Y / 2.0 + 26.0),
        (-SCALE_PAD_X / 2.0 + 34.0, SCALE_PAD_Y / 2.0 - 26.0),
        (SCALE_PAD_X / 2.0 - 34.0, SCALE_PAD_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        feet = feet
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_gravimetric_vibration_isolation_foot_{i}"),
                11.0,
                8.0,
                28,
            )
            .translate(-18.0 + *x, 10.0 + *y, GRAV_Z / 2.0 + 4.0);
    }
    feet
}

fn volume_standard_stops() -> Part {
    let mut stops = Part::empty(format!("{OUTPUT_PREFIX}_volume_standard_stops"));
    for i in 0..VOLUME_STANDARD_STOPS {
        stops = stops
            + centered_cube(
                format!("{OUTPUT_PREFIX}_volume_standard_stop_{i}"),
                18.0,
                82.0,
                16.0,
            )
            .translate(
                -112.0 + i as f64 * 94.0,
                -GRAV_Y / 2.0 + 36.0,
                GRAV_Z / 2.0 + 8.0,
            );
    }
    stops
}

fn gravimetric_closed_port_comb() -> Part {
    let mut comb = Part::empty(format!("{OUTPUT_PREFIX}_gravimetric_closed_port_comb"));
    for i in 0..4 {
        let x = centered_index(i, 4, 44.0) - 18.0;
        let finger = centered_cube(
            format!("{OUTPUT_PREFIX}_gravimetric_closed_port_comb_finger_{i}"),
            10.0,
            52.0,
            18.0,
        )
        .translate(x, GRAV_Y / 2.0 - 34.0, GRAV_Z / 2.0 + 9.0);
        comb = comb + finger;
    }
    comb
}

fn bubble_settling_witness_window_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_settling_witness_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let rear_light_channel = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_settling_backlight_channel_cut"),
        WITNESS_X - 78.0,
        38.0,
        14.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 52.0, WITNESS_Z / 2.0 - 7.0);

    body - rear_light_channel - bubble_window_cuts() - settling_window_cuts()
        + bubble_window_frames()
        + settling_window_frames()
        + witness_level_ticks()
        + settling_time_token_lands()
        + witness_air_purge_saddles()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_bubble_witness_window_cuts"));
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_witness_window_cut_{i}"),
                34.0,
                86.0,
                WITNESS_Z + 4.0,
            )
            .translate(
                centered_index(i, BUBBLE_WITNESS_WINDOWS, 54.0) - 58.0,
                46.0,
                0.0,
            );
    }
    cuts
}

fn settling_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_settling_witness_window_cuts"));
    for i in 0..SETTLING_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settling_witness_window_cut_{i}"),
                28.0,
                104.0,
                WITNESS_Z + 4.0,
            )
            .translate(
                centered_index(i, SETTLING_WITNESS_WINDOWS, 42.0) + 54.0,
                -48.0,
                0.0,
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_bubble_window_frames"));
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        let x = centered_index(i, BUBBLE_WITNESS_WINDOWS, 54.0) - 58.0;
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_bubble_witness_window_frame_{i}"),
            46.0,
            98.0,
            8.0,
        )
        .translate(x, 46.0, WITNESS_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_bubble_witness_frame_opening_{i}"),
            34.0,
            86.0,
            9.0,
        )
        .translate(x, 46.0, WITNESS_Z / 2.0 + 4.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn settling_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_settling_window_frames"));
    for i in 0..SETTLING_WITNESS_WINDOWS {
        let x = centered_index(i, SETTLING_WITNESS_WINDOWS, 42.0) + 54.0;
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_settling_witness_window_frame_{i}"),
            40.0,
            116.0,
            8.0,
        )
        .translate(x, -48.0, WITNESS_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_settling_witness_frame_opening_{i}"),
            28.0,
            104.0,
            9.0,
        )
        .translate(x, -48.0, WITNESS_Z / 2.0 + 4.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn witness_level_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_witness_level_ticks"));
    for i in 0..WITNESS_LEVEL_TICKS {
        let y = -100.0 + i as f64 * 24.0;
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_witness_level_tick_{i}"),
                42.0,
                3.0,
                4.0,
            )
            .translate(WITNESS_X / 2.0 - 46.0, y, WITNESS_Z / 2.0 + 2.0);
    }
    ticks
}

fn settling_time_token_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_settling_time_token_lands"));
    for (i, minutes) in SETTLING_TIME_TOKENS.iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settling_time_{minutes}_minute_token_land"),
                34.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(i, SETTLING_TIME_TOKENS.len(), 48.0),
                -WITNESS_Y / 2.0 + 22.0,
                WITNESS_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn witness_air_purge_saddles() -> Part {
    let mut saddles = Part::empty(format!("{OUTPUT_PREFIX}_witness_air_purge_saddles"));
    for i in 0..BYPASS_BRANCHES {
        let saddle = centered_cube(
            format!("{OUTPUT_PREFIX}_witness_air_purge_saddle_{i}"),
            34.0,
            18.0,
            14.0,
        )
        .translate(
            -WITNESS_X / 2.0 + 44.0,
            centered_index(i, BYPASS_BRANCHES, 54.0),
            WITNESS_Z / 2.0 + 7.0,
        );
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_witness_air_purge_bore_{i}"),
            LOOP_BORE_D / 2.0,
            36.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            -WITNESS_X / 2.0 + 44.0,
            centered_index(i, BYPASS_BRANCHES, 54.0),
            WITNESS_Z / 2.0 + 7.0,
        );
        saddles = saddles + (saddle - bore);
    }
    saddles
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_custody_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let card_slot_cut = centered_cube(
        format!("{OUTPUT_PREFIX}_custody_record_card_slot_cut"),
        118.0,
        76.0,
        11.0,
    )
    .translate(CUSTODY_X / 2.0 - 82.0, 0.0, CUSTODY_Z / 2.0 - 5.5);

    panel - card_slot_cut
        + barcode_land_array()
        + rfid_land_array()
        + custody_seal_lands()
        + cross_method_correlation_token_lands()
        + custody_camera_witness_rail()
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_land_array"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_land_{i}"),
                42.0,
                15.0,
                3.0,
            )
            .translate(
                centered_index(i % 6, 6, 70.0) - 56.0,
                if i < 6 { -30.0 } else { 30.0 },
                CUSTODY_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn rfid_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_land_array"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_rfid_custody_land_{i}"),
                13.0,
                4.0,
                28,
            )
            .translate(
                -CUSTODY_X / 2.0 + 40.0,
                centered_index(i, RFID_LANDS, 28.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_custody_seal_lands"));
    for i in 0..CUSTODY_SEAL_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_custody_tamper_seal_land_{i}"),
                34.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LANDS, 48.0) + 82.0,
                -CUSTODY_Y / 2.0 + 18.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn cross_method_correlation_token_lands() -> Part {
    let mut lands = Part::empty(format!(
        "{OUTPUT_PREFIX}_cross_method_correlation_token_lands"
    ));
    for (i, method) in MEASUREMENT_METHODS.iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{method}_correlation_token_land"),
                72.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, METHOD_TOKEN_LANDS, 86.0) + 66.0,
                CUSTODY_Y / 2.0 - 24.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn custody_camera_witness_rail() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_custody_camera_witness_rail"),
        CUSTODY_X - 62.0,
        10.0,
        12.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0 + 6.0)
}

fn release_hold_reject_gate_array() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_gate_base"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let service_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_gate_service_recess_cut"),
        GATE_X - 78.0,
        GATE_Y - 42.0,
        14.0,
    )
    .translate(0.0, 0.0, GATE_Z / 2.0 - 7.0);

    base - service_recess
        + disposition_lane_slots()
        + release_gate_solenoids()
        + decision_input_lands()
        + lane_lockout_flags()
        + gate_flow_arrows()
}

fn disposition_lane_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_disposition_lane_slots"));
    for lane in 0..DISPOSITION_LANES.len() {
        let lane_x = centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        let lane_bar = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_lane_bar", DISPOSITION_LANES[lane]),
            126.0,
            18.0,
            12.0,
        )
        .translate(lane_x, GATE_Y / 2.0 - 24.0, GATE_Z / 2.0 + 6.0);
        slots = slots + lane_bar;
        for slot in 0..GATE_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_{}_lane_sample_decision_slot_{slot}",
                        DISPOSITION_LANES[lane]
                    ),
                    72.0,
                    12.0,
                    8.0,
                )
                .translate(
                    lane_x,
                    centered_index(slot, GATE_SLOTS_PER_LANE, 22.0) - 16.0,
                    GATE_Z / 2.0 + 4.0,
                );
        }
    }
    slots
}

fn release_gate_solenoids() -> Part {
    let mut solenoids = Part::empty(format!("{OUTPUT_PREFIX}_gate_solenoids"));
    for i in 0..GATE_SOLENOIDS {
        let lane = i / 2;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        solenoids = solenoids
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_solenoid_{i}"),
                34.0,
                18.0,
                22.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X) + side * 44.0,
                -GATE_Y / 2.0 + 22.0,
                GATE_Z / 2.0 + 11.0,
            );
    }
    solenoids
}

fn decision_input_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_decision_input_lands"));
    for i in 0..DECISION_INPUTS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_decision_input_land_{i}"),
                36.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i % 9, 9, 42.0),
                -GATE_Y / 2.0 + 46.0,
                GATE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn lane_lockout_flags() -> Part {
    let mut flags = Part::empty(format!("{OUTPUT_PREFIX}_lane_lockout_flags"));
    for (lane, name) in DISPOSITION_LANES.iter().enumerate() {
        flags = flags
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{name}_lane_lockout_flag"),
                18.0,
                42.0,
                30.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X) + 58.0,
                0.0,
                GATE_Z / 2.0 + 15.0,
            );
    }
    flags
}

fn gate_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{OUTPUT_PREFIX}_gate_flow_arrows"));
    for lane in 0..DISPOSITION_LANES.len() {
        arrows = arrows
            + flow_arrow_marker(
                format!(
                    "{OUTPUT_PREFIX}_{}_lane_flow_arrow",
                    DISPOSITION_LANES[lane]
                ),
                0.0,
                1.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X),
                -16.0,
                GATE_Z / 2.0 + 7.0,
            );
    }
    arrows
}

fn closed_sample_loop_route_harness() -> Part {
    let mut routes = Part::empty(format!("{OUTPUT_PREFIX}_closed_sample_loop_route_harness"));
    let segments = route_segments();
    for (i, (x0, y0, x1, y1)) in segments.iter().enumerate() {
        routes = routes
            + route_segment(
                format!("{OUTPUT_PREFIX}_sample_loop_route_segment_{i}"),
                *x0,
                *y0,
                *x1,
                *y1,
            );
    }

    routes + route_elbow_nodes() + route_direction_markers() + bypass_branch_harness()
}

fn route_segments() -> [(f64, f64, f64, f64); ROUTE_SEGMENTS] {
    [
        (LOOP_CENTER.0, LOOP_CENTER.1, LOOP_CENTER.0, 330.0),
        (LOOP_CENTER.0, 330.0, DILUTION_CENTER.0, 330.0),
        (
            DILUTION_CENTER.0,
            330.0,
            DILUTION_CENTER.0,
            DILUTION_CENTER.1,
        ),
        (
            DILUTION_CENTER.0,
            DILUTION_CENTER.1,
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
        ),
        (
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
            OPTICAL_CENTER.0,
            COUNTER_CENTER.1,
        ),
        (
            OPTICAL_CENTER.0,
            COUNTER_CENTER.1,
            GRAV_CENTER.0,
            GRAV_CENTER.1,
        ),
        (GRAV_CENTER.0, GRAV_CENTER.1, GRAV_CENTER.0, GATE_CENTER.1),
        (GRAV_CENTER.0, GATE_CENTER.1, GATE_CENTER.0, GATE_CENTER.1),
        (
            GRAV_CENTER.0,
            GRAV_CENTER.1,
            COUNTER_CENTER.0,
            COUNTER_CENTER.1,
        ),
        (
            COUNTER_CENTER.0,
            COUNTER_CENTER.1,
            LOOP_CENTER.0,
            LOOP_CENTER.1,
        ),
    ]
}

fn route_segment(name: String, x0: f64, y0: f64, x1: f64, y1: f64) -> Part {
    if (x0 - x1).abs() >= (y0 - y1).abs() {
        centered_cylinder(name, ROUTE_TUBE_D / 2.0, (x1 - x0).abs(), 24)
            .rotate(0.0, 90.0, 0.0)
            .translate((x0 + x1) / 2.0, y0, ROUTE_Z)
    } else {
        centered_cylinder(name, ROUTE_TUBE_D / 2.0, (y1 - y0).abs(), 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(x0, (y0 + y1) / 2.0, ROUTE_Z)
    }
}

fn route_elbow_nodes() -> Part {
    let mut elbows = Part::empty(format!("{OUTPUT_PREFIX}_route_elbow_nodes"));
    for (i, (x, y)) in [
        (LOOP_CENTER.0, LOOP_CENTER.1),
        (LOOP_CENTER.0, 330.0),
        (DILUTION_CENTER.0, 330.0),
        (DILUTION_CENTER.0, DILUTION_CENTER.1),
        (OPTICAL_CENTER.0, OPTICAL_CENTER.1),
        (OPTICAL_CENTER.0, COUNTER_CENTER.1),
        (GRAV_CENTER.0, GRAV_CENTER.1),
        (GRAV_CENTER.0, GATE_CENTER.1),
        (GATE_CENTER.0, GATE_CENTER.1),
        (COUNTER_CENTER.0, COUNTER_CENTER.1),
    ]
    .iter()
    .enumerate()
    {
        elbows = elbows
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_route_elbow_node_{i}"),
                ROUTE_TUBE_D / 2.0 + 2.5,
                10.0,
                24,
            )
            .translate(*x, *y, ROUTE_Z);
    }
    elbows
}

fn route_direction_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_route_direction_markers"));
    for (i, (x, y, dx, dy)) in [
        (-236.0, 330.0, 1.0, 0.0),
        (40.0, 270.0, 0.0, -1.0),
        (256.0, 205.0, 1.0, 0.0),
        (475.0, 42.0, 0.0, -1.0),
        (256.0, -115.0, -1.0, 0.0),
        (40.0, -228.0, 0.0, -1.0),
        (194.0, -340.0, 1.0, 0.0),
        (-420.0, 42.0, 0.0, 1.0),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + flow_arrow_marker(
                format!("{OUTPUT_PREFIX}_route_direction_marker_{i}"),
                *dx,
                *dy,
            )
            .translate(*x, *y, ROUTE_Z + 8.0);
    }
    markers
}

fn bypass_branch_harness() -> Part {
    let mut branches = Part::empty(format!("{OUTPUT_PREFIX}_method_bypass_branch_harness"));
    for (i, (x, y)) in [
        (OPTICAL_CENTER.0 + 108.0, OPTICAL_CENTER.1 - 74.0),
        (COUNTER_CENTER.0 + 106.0, COUNTER_CENTER.1 + 82.0),
        (GRAV_CENTER.0 + 94.0, GRAV_CENTER.1 + 82.0),
    ]
    .iter()
    .enumerate()
    {
        branches = branches
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_method_bypass_branch_{i}"),
                ROUTE_TUBE_D / 2.0,
                82.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, *y, ROUTE_Z - 14.0);
    }
    branches
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_bar(
        format!("{OUTPUT_PREFIX}_keepout_front_robot_sweep"),
        KEEP_OUT_X,
        12.0,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + FRONT_ROBOT_CLEARANCE, 4.0);
    let rear_optical = keepout_bar(
        format!("{OUTPUT_PREFIX}_keepout_rear_optical_service"),
        KEEP_OUT_X - 180.0,
        12.0,
        7.0,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_OPTICAL_SERVICE_CLEARANCE, 4.0);
    let left_counter = keepout_bar(
        format!("{OUTPUT_PREFIX}_keepout_left_counter_service"),
        12.0,
        KEEP_OUT_Y - 144.0,
        7.0,
    )
    .translate(-DECK_X / 2.0 + LEFT_COUNTER_SERVICE_CLEARANCE, 0.0, 4.0);
    let right_witness = keepout_bar(
        format!("{OUTPUT_PREFIX}_keepout_right_window_service"),
        12.0,
        KEEP_OUT_Y - 144.0,
        7.0,
    )
    .translate(DECK_X / 2.0 - RIGHT_WINDOW_SERVICE_CLEARANCE, 0.0, 4.0);
    let top_lift = centered_cube(
        format!("{OUTPUT_PREFIX}_keepout_sample_loop_lift_height_flag"),
        130.0,
        16.0,
        TOP_SAMPLE_LOOP_LIFT_CLEARANCE / 8.0,
    )
    .translate(
        LOOP_CENTER.0 - LOOP_X / 2.0 + 88.0,
        LOOP_CENTER.1,
        TOP_SAMPLE_LOOP_LIFT_CLEARANCE / 16.0,
    );

    front_robot + rear_optical + left_counter + right_witness + top_lift
}

fn keepout_bar(name: String, x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y, z)
}

fn flow_arrow_marker(name: String, dx: f64, dy: f64) -> Part {
    let horizontal = dx.abs() >= dy.abs();
    if horizontal {
        let stem = centered_cube(format!("{name}_stem"), 28.0, 5.0, 4.0);
        let head = centered_cube(format!("{name}_head"), 10.0, 14.0, 4.0).translate(
            dx.signum() * 19.0,
            0.0,
            0.0,
        );
        stem + head
    } else {
        let stem = centered_cube(format!("{name}_stem"), 5.0, 28.0, 4.0);
        let head = centered_cube(format!("{name}_head"), 14.0, 10.0, 4.0).translate(
            0.0,
            dy.signum() * 19.0,
            0.0,
        );
        stem + head
    }
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 8.0, 3.0, 32);
    let center = centered_cylinder(format!("{name}_center_cut"), 2.0, 4.0, 20);
    disc - center
}

fn module_specs() -> [ModuleSpec; 8] {
    [
        ModuleSpec {
            name: "sealed_sample_loop_manifold",
            center: LOOP_CENTER,
            x: LOOP_X,
            y: LOOP_Y,
            z: LOOP_Z,
        },
        ModuleSpec {
            name: "dilution_ladder_coupon_bank",
            center: DILUTION_CENTER,
            x: DILUTION_X,
            y: DILUTION_Y,
            z: DILUTION_Z,
        },
        ModuleSpec {
            name: "optical_density_turbidity_window_lane",
            center: OPTICAL_CENTER,
            x: OPTICAL_X,
            y: OPTICAL_Y,
            z: OPTICAL_Z,
        },
        ModuleSpec {
            name: "impedance_counter_dock_envelope",
            center: COUNTER_CENTER,
            x: COUNTER_X,
            y: COUNTER_Y,
            z: COUNTER_Z,
        },
        ModuleSpec {
            name: "gravimetric_volume_reference_pad",
            center: GRAV_CENTER,
            x: GRAV_X,
            y: GRAV_Y,
            z: GRAV_Z,
        },
        ModuleSpec {
            name: "bubble_settling_witness_window_bank",
            center: WITNESS_CENTER,
            x: WITNESS_X,
            y: WITNESS_Y,
            z: WITNESS_Z,
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

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "expected stable STL export count");
    assert_eq!(REQUIRED_FEATURES.len(), 14, "required feature list changed");
    assert_eq!(
        DILUTION_COUPON_COUNT,
        DILUTION_LEVELS * COUPONS_PER_LEVEL,
        "dilution ladder coupon grid changed"
    );
    assert_eq!(
        GATE_SOLENOIDS,
        DISPOSITION_LANES.len() * 2,
        "each disposition lane needs two physical gate solenoids"
    );
    assert_eq!(
        DECISION_INPUTS,
        MEASUREMENT_METHODS.len() * DISPOSITION_LANES.len(),
        "decision input lands must map every method into every lane"
    );
    assert_eq!(
        SETTLING_TIME_TOKENS.len(),
        SETTLING_WITNESS_WINDOWS,
        "settling witness windows must match the time-token count"
    );
    assert!(
        SETTLING_TIME_TOKENS
            .windows(2)
            .all(|window| window[0] < window[1]),
        "settling time tokens must increase monotonically"
    );
    assert_eq!(
        ROUTE_SEGMENTS,
        route_segments().len(),
        "closed route segment count changed"
    );
    assert_eq!(ROUTE_ELBOWS, 10, "closed route elbow count changed");
    assert_eq!(
        ROUTE_DIRECTION_MARKERS, 8,
        "route direction marker count changed"
    );
    assert_eq!(DATUM_TARGETS, 4, "datum target count changed");
    assert_eq!(
        COUNTER_DATUM_PINS, 4,
        "counter dock datum pin count changed"
    );
    assert_eq!(
        GRAVIMETRIC_ISOLATION_FEET, 4,
        "gravimetric pad isolation foot count changed"
    );
    assert_eq!(
        KEEP_OUT_ZONES, 5,
        "robot/service keepout zone count changed"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 340.0
            && REAR_OPTICAL_SERVICE_CLEARANCE >= 220.0
            && LEFT_COUNTER_SERVICE_CLEARANCE >= 230.0
            && RIGHT_WINDOW_SERVICE_CLEARANCE >= 200.0,
        "service clearances below station target"
    );

    let modules = module_specs();
    for module in modules {
        assert!(
            module.fits_on_deck(),
            "{} exceeds deck envelope",
            module.name
        );
        assert!(
            module.z > 0.0,
            "{} must have positive module height",
            module.name
        );
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
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();

        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_cell_suspension_optical_density_counter_crosscheck_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_density_crosscheck_intent() {
        for feature in [
            "sealed_sample_loop_manifold",
            "dilution_ladder_coupon_bank",
            "optical_density_turbidity_window_lane",
            "impedance_counter_dock_envelope",
            "gravimetric_volume_reference_pad",
            "bubble_settling_witness_window_bank",
            "barcode_custody_lands",
            "release_hold_reject_gate_array",
            "closed_sample_loop_route_harness",
            "cross_method_correlation_token_lands",
            "sample_loop_pressure_taps",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }
        assert_eq!(REQUIRED_FEATURES.len(), 14);
    }

    #[test]
    fn station_dimensions_and_modules_fit_without_overlap() {
        assert!(DECK_X <= 1500.0);
        assert!(DECK_Y <= 900.0);
        assert!(RIM_Z >= 36.0);
        assert_eq!(module_specs().len(), 8);
        assert_design_constraints();
    }

    #[test]
    fn crosscheck_methods_and_reference_counts_are_explicit() {
        assert_eq!(MEASUREMENT_METHODS.len(), 3);
        assert_eq!(
            MEASUREMENT_METHODS,
            [
                "optical_density_turbidity",
                "impedance_counter",
                "gravimetric_volume"
            ]
        );
        assert_eq!(DILUTION_LEVELS, 5);
        assert_eq!(COUPONS_PER_LEVEL, 2);
        assert_eq!(DILUTION_COUPON_COUNT, 10);
        assert_eq!(OPTICAL_PATH_WINDOWS, 6);
        assert_eq!(COUNTER_DOCKS, 2);
        assert_eq!(IMPEDANCE_ELECTRODE_PAIRS, 6);
        assert_eq!(VOLUME_REFERENCE_WELLS, 6);
    }

    #[test]
    fn witness_and_custody_surfaces_are_sized_for_release_evidence() {
        assert_eq!(BUBBLE_WITNESS_WINDOWS, 4);
        assert_eq!(SETTLING_WITNESS_WINDOWS, SETTLING_TIME_TOKENS.len());
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(METHOD_TOKEN_LANDS, MEASUREMENT_METHODS.len());
        assert!(CUSTODY_X >= 520.0);
        assert!(WITNESS_Y >= 240.0);
    }

    #[test]
    fn release_hold_reject_gates_are_physical_and_method_mapped() {
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(GATE_SLOTS_PER_LANE, 4);
        assert_eq!(GATE_SOLENOIDS, 6);
        assert_eq!(DECISION_INPUTS, 9);
        assert_eq!(
            DECISION_INPUTS,
            MEASUREMENT_METHODS.len() * DISPOSITION_LANES.len()
        );
        assert!(GATE_X > GATE_LANE_PITCH_X * DISPOSITION_LANES.len() as f64);
    }

    #[test]
    fn closed_route_harness_count_and_service_keepouts_are_pinned() {
        assert_eq!(ROUTE_SEGMENTS, route_segments().len());
        assert_eq!(ROUTE_ELBOWS, 10);
        assert_eq!(ROUTE_DIRECTION_MARKERS, 8);
        assert_eq!(BYPASS_BRANCHES, MEASUREMENT_METHODS.len());
        assert_eq!(KEEP_OUT_ZONES, 5);
        assert!(KEEP_OUT_X <= DECK_X);
        assert!(KEEP_OUT_Y <= DECK_Y);
        assert!(TOP_SAMPLE_LOOP_LIFT_CLEARANCE >= 300.0);
    }
}
