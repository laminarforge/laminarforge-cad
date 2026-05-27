use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system perfusion media temperature step-response station.
//
// Intent:
// - Validate warm/cold media transitions through a no-live-chip perfusion path
//   before exposing tissue-chip cassettes to a thermal step.
// - Make thermal source selection, exchanger surrogate response, inline sensor
//   timing, residence-time hold-up, bubble/degas visibility, cassette inlet and
//   outlet references, waste/retain disposition, and release/hold/reject gates
//   mechanically visible on one closed validation deck.
// - This is architecture/fit CAD for validation planning. It is not a sterile
//   barrier drawing, calibrated thermal standard, biological acceptance method,
//   or operating recipe.

const PREFIX: &str = "closed_perfusion_media_temperature_step_response_station";
const OUTPUT_PREFIX: &str = "output/closed_perfusion_media_temperature_step_response_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_media_temperature_step_response_station_secondary_containment_deck.stl",
    "output/closed_perfusion_media_temperature_step_response_station_warm_cold_media_cartridge_nests.stl",
    "output/closed_perfusion_media_temperature_step_response_station_heat_exchanger_surrogate_block.stl",
    "output/closed_perfusion_media_temperature_step_response_station_inline_temperature_sensor_ladder.stl",
    "output/closed_perfusion_media_temperature_step_response_station_residence_time_loop.stl",
    "output/closed_perfusion_media_temperature_step_response_station_bubble_degas_witness_window.stl",
    "output/closed_perfusion_media_temperature_step_response_station_cassette_inlet_outlet_reference_pockets.stl",
    "output/closed_perfusion_media_temperature_step_response_station_waste_retain_split_manifold.stl",
    "output/closed_perfusion_media_temperature_step_response_station_release_hold_reject_gates.stl",
    "output/closed_perfusion_media_temperature_step_response_station_run_record_custody_lands.stl",
    "output/closed_perfusion_media_temperature_step_response_station_robot_service_keepouts.stl",
    "output/closed_perfusion_media_temperature_step_response_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "secondary_containment_deck",
    "warm_cold_media_cartridge_nests",
    "heat_exchanger_surrogate_block",
    "inline_temperature_sensor_ladder",
    "residence_time_loop",
    "bubble_degas_witness_window",
    "cassette_inlet_outlet_reference_pockets",
    "waste_retain_split_manifold",
    "release_hold_reject_gates",
    "run_record_custody_lands",
    "robot_service_keepouts",
];

const MEDIA_STATES: [&str; 2] = ["warm_media", "cold_media"];
const REFERENCE_ROLES: [&str; 2] = ["cassette_inlet_reference", "cassette_outlet_reference"];
const DISPOSITION_GATES: [&str; 3] = ["release", "hold", "reject"];

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const BASIN_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const SOCKET_DEPTH: f64 = 5.0;
const LEAK_WITNESS_PADS: usize = 8;
const DATUM_BOSSES: usize = 10;

const CARTRIDGE_CENTER: (f64, f64) = (-435.0, 230.0);
const CARTRIDGE_X: f64 = 410.0;
const CARTRIDGE_Y: f64 = 250.0;
const CARTRIDGE_Z: f64 = 50.0;
const CARTRIDGE_NESTS: usize = MEDIA_STATES.len();
const CARTRIDGE_NEST_X: f64 = 158.0;
const CARTRIDGE_NEST_Y: f64 = 128.0;
const CARTRIDGE_NEST_Z: f64 = 11.0;
const CARTRIDGE_PITCH_X: f64 = 190.0;
const CARTRIDGE_NECK_D: f64 = 24.0;
const SOURCE_VALVE_PADS: usize = 4;

const EXCHANGER_CENTER: (f64, f64) = (0.0, 240.0);
const EXCHANGER_X: f64 = 360.0;
const EXCHANGER_Y: f64 = 230.0;
const EXCHANGER_Z: f64 = 74.0;
const EXCHANGER_CHANNELS: usize = 6;
const EXCHANGER_CHANNEL_D: f64 = 7.2;
const EXCHANGER_FINS: usize = 9;
const THERMAL_MASS_COUPONS: usize = 4;
const EXCHANGER_REFERENCE_WELLS: usize = 3;

const SENSOR_CENTER: (f64, f64) = (415.0, 180.0);
const SENSOR_X: f64 = 360.0;
const SENSOR_Y: f64 = 260.0;
const SENSOR_Z: f64 = 44.0;
const INLINE_SENSOR_COUNT: usize = 8;
const SENSOR_PITCH_Y: f64 = 27.5;
const SENSOR_WELL_D: f64 = 18.0;
const SENSOR_WINDOW_X: f64 = 94.0;
const SENSOR_WINDOW_Y: f64 = 13.0;
const SENSOR_LADDER_RUNG_COUNT: usize = INLINE_SENSOR_COUNT - 1;
const SENSOR_SPACING_MM: f64 = 34.0;

const LOOP_CENTER: (f64, f64) = (-410.0, -80.0);
const LOOP_X: f64 = 420.0;
const LOOP_Y: f64 = 330.0;
const LOOP_Z: f64 = 38.0;
const RESIDENCE_SEGMENTS: usize = 7;
const RESIDENCE_TURNS: usize = RESIDENCE_SEGMENTS - 1;
const LOOP_STRAIGHT_X: f64 = 334.0;
const LOOP_TRACE_W: f64 = 10.0;
const LOOP_TRACE_Z: f64 = 8.0;
const LOOP_PITCH_Y: f64 = 39.0;
const RESIDENCE_LOOP_VOLUME_UL: f64 = 820.0;
const RESIDENCE_TARGET_SECONDS: f64 = 75.0;
const SAMPLE_TAPS: usize = 5;

const DEGAS_CENTER: (f64, f64) = (0.0, -95.0);
const DEGAS_X: f64 = 360.0;
const DEGAS_Y: f64 = 260.0;
const DEGAS_Z: f64 = 42.0;
const WITNESS_WINDOWS: usize = 4;
const BUBBLE_RISERS: usize = 5;
const DEGAS_VENT_PORTS: usize = 4;
const WITNESS_TICKS: usize = 14;

const REFERENCE_CENTER: (f64, f64) = (415.0, -95.0);
const REFERENCE_X: f64 = 330.0;
const REFERENCE_Y: f64 = 260.0;
const REFERENCE_Z: f64 = 46.0;
const REFERENCE_POCKETS: usize = REFERENCE_ROLES.len();
const REFERENCE_POCKET_X: f64 = 122.0;
const REFERENCE_POCKET_Y: f64 = 78.0;
const REFERENCE_THERMOWELLS_PER_POCKET: usize = 3;
const REFERENCE_TOKEN_SLOTS: usize = 6;

const SPLIT_CENTER: (f64, f64) = (-480.0, -335.0);
const SPLIT_X: f64 = 300.0;
const SPLIT_Y: f64 = 110.0;
const SPLIT_Z: f64 = 42.0;
const SPLIT_STREAMS: usize = 2;
const SPLIT_PORT_D: f64 = 20.0;
const RETAIN_BAG_VOLUME_ML: f64 = 650.0;
const WASTE_BAG_VOLUME_ML: f64 = 900.0;

const GATE_CENTER: (f64, f64) = (-80.0, -340.0);
const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 100.0;
const GATE_Z: f64 = 34.0;
const GATE_LANES: usize = DISPOSITION_GATES.len();
const GATE_TOKEN_SLOTS_PER_LANE: usize = 5;
const GATE_TOKEN_SLOTS: usize = GATE_LANES * GATE_TOKEN_SLOTS_PER_LANE;
const GATE_LANE_PITCH_X: f64 = 132.0;
const GATE_SLOT_Y: f64 = 15.0;

const CUSTODY_CENTER: (f64, f64) = (390.0, -340.0);
const CUSTODY_X: f64 = 360.0;
const CUSTODY_Y: f64 = 100.0;
const CUSTODY_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 8;
const STEP_DELTA_TOKENS: usize = 6;
const RUN_RECORD_CARDS: usize = 4;

const ROBOT_KEEP_OUT_Z: f64 = 135.0;
const ROBOT_FRONT_CLEARANCE: f64 = 365.0;
const REAR_THERMAL_SERVICE_CLEARANCE: f64 = 240.0;
const LEFT_CARTRIDGE_SERVICE_CLEARANCE: f64 = 215.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 210.0;
const KEEP_OUT_GAUGES: usize = 7;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_station(self, margin: f64) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - margin
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - margin
    }

    fn overlaps(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(&deck, OUTPUTS[0]);

    let cartridges = warm_cold_media_cartridge_nests();
    export(&cartridges, OUTPUTS[1]);

    let exchanger = heat_exchanger_surrogate_block();
    export(&exchanger, OUTPUTS[2]);

    let ladder = inline_temperature_sensor_ladder();
    export(&ladder, OUTPUTS[3]);

    let loop_path = residence_time_loop();
    export(&loop_path, OUTPUTS[4]);

    let witness = bubble_degas_witness_window();
    export(&witness, OUTPUTS[5]);

    let references = cassette_inlet_outlet_reference_pockets();
    export(&references, OUTPUTS[6]);

    let split = waste_retain_split_manifold();
    export(&split, OUTPUTS[7]);

    let gates = release_hold_reject_gates();
    export(&gates, OUTPUTS[8]);

    let custody = run_record_custody_lands();
    export(&custody, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = deck
        + cartridges
        + exchanger
        + ladder
        + loop_path
        + witness
        + references
        + split
        + gates
        + custody
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed perfusion media temperature step-response station:");
    println!("  Output prefix:               {OUTPUT_PREFIX}");
    println!("  Deck/leak tray:              {STATION_X:.0}mm x {STATION_Y:.0}mm");
    println!(
        "  Thermal sources:             {CARTRIDGE_NESTS} warm/cold cartridge nests, {SOURCE_VALVE_PADS} source valve pads"
    );
    println!(
        "  Exchanger surrogate:         {EXCHANGER_CHANNELS} flow channels, {EXCHANGER_FINS} thermal fins, {THERMAL_MASS_COUPONS} mass coupons"
    );
    println!(
        "  Sensor timing:               {INLINE_SENSOR_COUNT} inline sensor pockets with {SENSOR_SPACING_MM:.0}mm nominal spacing"
    );
    println!(
        "  Residence loop:              {RESIDENCE_SEGMENTS} serpentine segments, {SAMPLE_TAPS} sample taps, {RESIDENCE_LOOP_VOLUME_UL:.0} uL modeled hold-up target"
    );
    println!(
        "  Witness/reference:           {WITNESS_WINDOWS} degas windows, {BUBBLE_RISERS} risers, {REFERENCE_POCKETS} cassette reference pockets"
    );
    println!(
        "  Disposition/evidence:        {SPLIT_STREAMS} retain/waste split streams, {GATE_TOKEN_SLOTS} gate token slots, {} required feature groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_z(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    assert_eq!(CARTRIDGE_NESTS, 2);
    assert_eq!(REFERENCE_POCKETS, 2);
    assert_eq!(GATE_LANES, 3);
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert!(RESIDENCE_TARGET_SECONDS >= 60.0);
    assert!(RESIDENCE_LOOP_VOLUME_UL >= 750.0);
    assert!(INLINE_SENSOR_COUNT >= 6);
    assert!(SENSOR_LADDER_RUNG_COUNT + 1 == INLINE_SENSOR_COUNT);
    assert!(CARTRIDGE_NEST_Z >= 10.0);
    assert!(RETAIN_BAG_VOLUME_ML < WASTE_BAG_VOLUME_ML);
    assert!(ROBOT_FRONT_CLEARANCE >= 360.0);
    assert!(REAR_THERMAL_SERVICE_CLEARANCE >= 220.0);
    assert!(LEFT_CARTRIDGE_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 200.0);

    let modules = module_specs();
    for module in modules {
        assert!(
            module.fits_station(14.0),
            "{} exceeds station envelope",
            module.name
        );
    }

    for a in 0..modules.len() {
        for b in (a + 1)..modules.len() {
            assert!(
                !modules[a].overlaps(modules[b], 8.0),
                "{} overlaps {}",
                modules[a].name,
                modules[b].name
            );
        }
    }
}

fn module_specs() -> [Rect; 9] {
    [
        Rect {
            name: "warm_cold_media_cartridge_nests",
            center: CARTRIDGE_CENTER,
            x: CARTRIDGE_X,
            y: CARTRIDGE_Y,
        },
        Rect {
            name: "heat_exchanger_surrogate_block",
            center: EXCHANGER_CENTER,
            x: EXCHANGER_X,
            y: EXCHANGER_Y,
        },
        Rect {
            name: "inline_temperature_sensor_ladder",
            center: SENSOR_CENTER,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Rect {
            name: "residence_time_loop",
            center: LOOP_CENTER,
            x: LOOP_X,
            y: LOOP_Y,
        },
        Rect {
            name: "bubble_degas_witness_window",
            center: DEGAS_CENTER,
            x: DEGAS_X,
            y: DEGAS_Y,
        },
        Rect {
            name: "cassette_inlet_outlet_reference_pockets",
            center: REFERENCE_CENTER,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Rect {
            name: "waste_retain_split_manifold",
            center: SPLIT_CENTER,
            x: SPLIT_X,
            y: SPLIT_Y,
        },
        Rect {
            name: "release_hold_reject_gates",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
        Rect {
            name: "run_record_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
    ]
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let basin = centered_cube(
        format!("{PREFIX}_recessed_leak_basin"),
        STATION_X - 118.0,
        STATION_Y - 108.0,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_DEPTH / 2.0);

    let drain = centered_cylinder(
        format!("{PREFIX}_front_low_point_drain"),
        DRAIN_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -STATION_Y / 2.0 + 18.0,
        BASE_Z - 5.0,
    );

    deck - basin - drain - module_socket_recesses()
        + perimeter_rims()
        + leak_witness_pads()
        + datum_bosses()
        + thermal_row_boundary_rails()
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_socket_recesses"));
    for module in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket_recess", module.name),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn perimeter_rims() -> Part {
    let left = centered_cube(format!("{PREFIX}_left_raised_lip"), RIM_W, STATION_Y, RIM_Z)
        .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_raised_lip"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_thermal_service_lip"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        format!("{PREFIX}_front_low_robot_lip"),
        STATION_X - 170.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 10.0, BASE_Z + 11.0);

    left + right + rear + front
}

fn leak_witness_pads() -> Part {
    let mut pads = Part::empty(format!("{PREFIX}_leak_witness_pads"));
    for i in 0..LEAK_WITNESS_PADS {
        let x = centered_index(i % 4, 4, 250.0);
        let y = centered_index(i / 4, 2, 520.0) - 12.0;
        pads =
            pads + centered_cylinder(format!("{PREFIX}_leak_witness_pad_{i}"), 14.0, 5.0, 32)
                .translate(x, y, BASE_Z + 2.5);
    }
    pads
}

fn datum_bosses() -> Part {
    let points = [
        (-620.0, 350.0),
        (-420.0, 350.0),
        (-110.0, 350.0),
        (190.0, 350.0),
        (560.0, 350.0),
        (-620.0, -390.0),
        (-330.0, -390.0),
        (-20.0, -390.0),
        (290.0, -390.0),
        (590.0, -390.0),
    ];
    assert_eq!(points.len(), DATUM_BOSSES);

    let mut bosses = Part::empty(format!("{PREFIX}_robot_datum_bosses"));
    for (i, (x, y)) in points.into_iter().enumerate() {
        let boss = centered_cylinder(format!("{PREFIX}_robot_datum_boss_{i}"), 10.5, 8.0, 28)
            .translate(x, y, BASE_Z + 4.0);
        let hole = centered_cylinder(
            format!("{PREFIX}_robot_datum_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn thermal_row_boundary_rails() -> Part {
    let source_to_exchanger = centered_cube(
        format!("{PREFIX}_source_to_exchanger_clean_lane_rail"),
        1110.0,
        8.0,
        22.0,
    )
    .translate(-80.0, 82.0, BASE_Z + 11.0);
    let response_to_disposition = centered_cube(
        format!("{PREFIX}_response_to_disposition_boundary_rail"),
        1110.0,
        8.0,
        20.0,
    )
    .translate(-80.0, -250.0, BASE_Z + 10.0);
    let retain_waste_split = centered_cube(
        format!("{PREFIX}_retain_waste_dirty_side_boundary"),
        8.0,
        300.0,
        22.0,
    )
    .translate(-295.0, -315.0, BASE_Z + 11.0);

    source_to_exchanger + response_to_disposition + retain_waste_split
}

fn warm_cold_media_cartridge_nests() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_warm_cold_media_cartridge_nest_panel"),
        CARTRIDGE_X,
        CARTRIDGE_Y,
        CARTRIDGE_Z,
    );

    let mut pockets = Part::empty(format!("{PREFIX}_media_cartridge_pocket_cuts"));
    let mut rims = Part::empty(format!("{PREFIX}_media_cartridge_retention_rims"));
    let mut necks = Part::empty(format!("{PREFIX}_media_cartridge_neck_saddles"));
    for (i, role) in MEDIA_STATES.into_iter().enumerate() {
        let x = centered_index(i, CARTRIDGE_NESTS, CARTRIDGE_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!("{PREFIX}_{role}_cartridge_body_clearance"),
                CARTRIDGE_NEST_X,
                CARTRIDGE_NEST_Y,
                CARTRIDGE_Z + 4.0,
            )
            .translate(x, 18.0, 0.0);

        rims = rims
            + centered_cube(
                format!("{PREFIX}_{role}_front_retention_lip"),
                CARTRIDGE_NEST_X + 18.0,
                12.0,
                12.0,
            )
            .translate(x, -CARTRIDGE_NEST_Y / 2.0 - 24.0, CARTRIDGE_Z / 2.0 + 6.0)
            + centered_cube(
                format!("{PREFIX}_{role}_rear_retention_lip"),
                CARTRIDGE_NEST_X + 18.0,
                12.0,
                14.0,
            )
            .translate(x, CARTRIDGE_NEST_Y / 2.0 + 22.0, CARTRIDGE_Z / 2.0 + 7.0);

        let neck_boss = centered_cylinder(
            format!("{PREFIX}_{role}_sealed_neck_boss"),
            CARTRIDGE_NECK_D / 2.0 + 7.0,
            12.0,
            36,
        )
        .translate(x, -CARTRIDGE_Y / 2.0 + 34.0, CARTRIDGE_Z / 2.0 + 6.0);
        let neck_clearance = centered_cylinder(
            format!("{PREFIX}_{role}_sealed_neck_clearance"),
            CARTRIDGE_NECK_D / 2.0,
            13.0,
            36,
        )
        .translate(x, -CARTRIDGE_Y / 2.0 + 34.0, CARTRIDGE_Z / 2.0 + 6.0);
        necks = necks + (neck_boss - neck_clearance);
    }

    let thermal_isolator = centered_cube(
        format!("{PREFIX}_warm_cold_thermal_isolator_spine"),
        18.0,
        CARTRIDGE_Y - 44.0,
        28.0,
    )
    .translate(0.0, 20.0, CARTRIDGE_Z / 2.0 + 14.0);

    let mut valves = Part::empty(format!("{PREFIX}_warm_cold_source_valve_pads"));
    for i in 0..SOURCE_VALVE_PADS {
        valves = valves
            + centered_cube(
                format!("{PREFIX}_source_selection_valve_pad_{i}"),
                46.0,
                28.0,
                10.0,
            )
            .translate(
                centered_index(i, SOURCE_VALVE_PADS, 70.0),
                -CARTRIDGE_Y / 2.0 + 72.0,
                CARTRIDGE_Z / 2.0 + 5.0,
            );
    }

    (body - pockets + rims + necks + thermal_isolator + valves).translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1,
        deck_z(CARTRIDGE_Z),
    )
}

fn heat_exchanger_surrogate_block() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_heat_exchanger_surrogate_block_body"),
        EXCHANGER_X,
        EXCHANGER_Y,
        EXCHANGER_Z,
    );

    let mut channel_cuts = Part::empty(format!("{PREFIX}_heat_exchanger_parallel_channel_cuts"));
    for i in 0..EXCHANGER_CHANNELS {
        let y = centered_index(i, EXCHANGER_CHANNELS, 27.0);
        channel_cuts = channel_cuts
            + centered_cylinder(
                format!("{PREFIX}_heat_exchanger_flow_channel_{i}"),
                EXCHANGER_CHANNEL_D / 2.0,
                EXCHANGER_X + 18.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -10.0);
    }

    let mut fins = Part::empty(format!("{PREFIX}_heat_exchanger_response_fins"));
    for i in 0..EXCHANGER_FINS {
        fins = fins
            + centered_cube(
                format!("{PREFIX}_heat_exchanger_response_fin_{i}"),
                9.0,
                EXCHANGER_Y - 34.0,
                28.0,
            )
            .translate(
                centered_index(i, EXCHANGER_FINS, 34.0),
                0.0,
                EXCHANGER_Z / 2.0 + 14.0,
            );
    }

    let mut coupons = Part::empty(format!("{PREFIX}_heat_exchanger_thermal_mass_coupons"));
    for i in 0..THERMAL_MASS_COUPONS {
        coupons = coupons
            + centered_cube(
                format!("{PREFIX}_heat_exchanger_mass_coupon_{i}"),
                54.0,
                38.0,
                12.0,
            )
            .translate(
                centered_index(i, THERMAL_MASS_COUPONS, 72.0),
                EXCHANGER_Y / 2.0 - 34.0,
                EXCHANGER_Z / 2.0 + 6.0,
            );
    }

    let mut wells = Part::empty(format!("{PREFIX}_heat_exchanger_reference_wells"));
    for i in 0..EXCHANGER_REFERENCE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_heat_exchanger_reference_thermowell_{i}"),
                4.2,
                EXCHANGER_Z + 10.0,
                20,
            )
            .translate(
                centered_index(i, EXCHANGER_REFERENCE_WELLS, 96.0),
                -82.0,
                0.0,
            );
    }

    let inlet_outlet_manifold = centered_cube(
        format!("{PREFIX}_heat_exchanger_inlet_outlet_manifold_face"),
        EXCHANGER_X - 36.0,
        16.0,
        18.0,
    )
    .translate(0.0, -EXCHANGER_Y / 2.0 + 22.0, EXCHANGER_Z / 2.0 + 9.0);

    (block - channel_cuts - wells + fins + coupons + inlet_outlet_manifold).translate(
        EXCHANGER_CENTER.0,
        EXCHANGER_CENTER.1,
        deck_z(EXCHANGER_Z),
    )
}

fn inline_temperature_sensor_ladder() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_inline_temperature_sensor_ladder_panel"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );

    let flow_bore = centered_cylinder(
        format!("{PREFIX}_inline_temperature_sensor_ladder_flow_bore"),
        4.1,
        SENSOR_Y + 16.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-92.0, 0.0, -5.0);

    let mut sensor_cuts = Part::empty(format!("{PREFIX}_inline_temperature_sensor_well_cuts"));
    let mut guard_rings = Part::empty(format!("{PREFIX}_inline_temperature_sensor_guard_rings"));
    let mut windows = Part::empty(format!("{PREFIX}_inline_temperature_step_witness_windows"));
    for i in 0..INLINE_SENSOR_COUNT {
        let y = centered_index(i, INLINE_SENSOR_COUNT, SENSOR_PITCH_Y);
        sensor_cuts = sensor_cuts
            + centered_cylinder(
                format!("{PREFIX}_inline_temperature_sensor_well_{i}"),
                SENSOR_WELL_D / 2.0,
                SENSOR_Z + 6.0,
                30,
            )
            .translate(-92.0, y, 0.0);

        let ring = centered_cylinder(
            format!("{PREFIX}_inline_temperature_sensor_guard_ring_{i}"),
            SENSOR_WELL_D / 2.0 + 5.0,
            4.0,
            30,
        )
        .translate(-92.0, y, SENSOR_Z / 2.0 + 2.0);
        let opening = centered_cylinder(
            format!("{PREFIX}_inline_temperature_sensor_guard_opening_{i}"),
            SENSOR_WELL_D / 2.0 + 0.8,
            4.5,
            30,
        )
        .translate(-92.0, y, SENSOR_Z / 2.0 + 2.0);
        guard_rings = guard_rings + (ring - opening);

        windows = windows
            + centered_cube(
                format!("{PREFIX}_inline_temperature_sensor_trace_window_{i}"),
                SENSOR_WINDOW_X,
                SENSOR_WINDOW_Y,
                8.0,
            )
            .translate(52.0, y, SENSOR_Z / 2.0 - 3.0);
    }

    let mut rungs = Part::empty(format!("{PREFIX}_inline_temperature_sensor_ladder_rungs"));
    for i in 0..SENSOR_LADDER_RUNG_COUNT {
        let y = (centered_index(i, INLINE_SENSOR_COUNT, SENSOR_PITCH_Y)
            + centered_index(i + 1, INLINE_SENSOR_COUNT, SENSOR_PITCH_Y))
            / 2.0;
        rungs = rungs
            + centered_cube(
                format!("{PREFIX}_inline_temperature_sensor_ladder_rung_{i}"),
                220.0,
                4.0,
                11.0,
            )
            .translate(8.0, y, SENSOR_Z / 2.0 + 5.5);
    }

    let delta_token_lane = centered_cube(
        format!("{PREFIX}_inline_temperature_delta_t_token_lane"),
        SENSOR_X - 44.0,
        16.0,
        7.0,
    )
    .translate(0.0, -SENSOR_Y / 2.0 + 20.0, SENSOR_Z / 2.0 + 3.5);

    (panel - flow_bore - sensor_cuts - windows + guard_rings + rungs + delta_token_lane).translate(
        SENSOR_CENTER.0,
        SENSOR_CENTER.1,
        deck_z(SENSOR_Z),
    )
}

fn residence_time_loop() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_residence_time_loop_base_panel"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );

    let mut traces = Part::empty(format!("{PREFIX}_residence_time_serpentine_trace"));
    for i in 0..RESIDENCE_SEGMENTS {
        let y = centered_index(i, RESIDENCE_SEGMENTS, LOOP_PITCH_Y);
        traces = traces
            + centered_cube(
                format!("{PREFIX}_residence_time_straight_segment_{i}"),
                LOOP_STRAIGHT_X,
                LOOP_TRACE_W,
                LOOP_TRACE_Z,
            )
            .translate(0.0, y, LOOP_Z / 2.0 + LOOP_TRACE_Z / 2.0);
    }

    let mut turns = Part::empty(format!("{PREFIX}_residence_time_u_turn_witnesses"));
    for i in 0..RESIDENCE_TURNS {
        let y = (centered_index(i, RESIDENCE_SEGMENTS, LOOP_PITCH_Y)
            + centered_index(i + 1, RESIDENCE_SEGMENTS, LOOP_PITCH_Y))
            / 2.0;
        let x = if i % 2 == 0 {
            LOOP_STRAIGHT_X / 2.0
        } else {
            -LOOP_STRAIGHT_X / 2.0
        };
        turns = turns
            + centered_cylinder(
                format!("{PREFIX}_residence_time_u_turn_post_{i}"),
                18.0,
                LOOP_TRACE_Z,
                32,
            )
            .translate(x, y, LOOP_Z / 2.0 + LOOP_TRACE_Z / 2.0);
    }

    let mut taps = Part::empty(format!("{PREFIX}_residence_time_sample_tap_bosses"));
    for i in 0..SAMPLE_TAPS {
        taps = taps
            + centered_cylinder(
                format!("{PREFIX}_residence_time_sample_tap_{i}"),
                8.0,
                10.0,
                24,
            )
            .translate(
                centered_index(i, SAMPLE_TAPS, 74.0),
                -LOOP_Y / 2.0 + 34.0,
                LOOP_Z / 2.0 + 5.0,
            );
    }

    let timer_slot = centered_cube(
        format!("{PREFIX}_residence_time_target_timer_slot"),
        LOOP_X - 54.0,
        20.0,
        9.0,
    )
    .translate(0.0, LOOP_Y / 2.0 - 28.0, LOOP_Z / 2.0 + 4.5);

    (panel + traces + turns + taps + timer_slot).translate(
        LOOP_CENTER.0,
        LOOP_CENTER.1,
        deck_z(LOOP_Z),
    )
}

fn bubble_degas_witness_window() -> Part {
    let frame = centered_cube(
        format!("{PREFIX}_bubble_degas_witness_window_frame"),
        DEGAS_X,
        DEGAS_Y,
        DEGAS_Z,
    );
    let central_window = centered_cube(
        format!("{PREFIX}_bubble_degas_transparent_insert_clearance"),
        DEGAS_X - 88.0,
        DEGAS_Y - 86.0,
        DEGAS_Z + 5.0,
    )
    .translate(0.0, 8.0, 0.0);

    let mut risers = Part::empty(format!("{PREFIX}_bubble_degas_riser_wells"));
    for i in 0..BUBBLE_RISERS {
        risers = risers
            + centered_cylinder(
                format!("{PREFIX}_bubble_degas_riser_well_{i}"),
                12.0,
                DEGAS_Z + 8.0,
                30,
            )
            .translate(centered_index(i, BUBBLE_RISERS, 54.0), 64.0, 0.0);
    }

    let mut vents = Part::empty(format!("{PREFIX}_bubble_degas_vent_port_bosses"));
    for i in 0..DEGAS_VENT_PORTS {
        let x = centered_index(i, DEGAS_VENT_PORTS, 72.0);
        let boss = centered_cylinder(
            format!("{PREFIX}_bubble_degas_vent_boss_{i}"),
            11.0,
            8.0,
            24,
        )
        .translate(x, -DEGAS_Y / 2.0 + 30.0, DEGAS_Z / 2.0 + 4.0);
        let cut = centered_cylinder(
            format!("{PREFIX}_bubble_degas_vent_clearance_{i}"),
            4.2,
            9.0,
            20,
        )
        .translate(x, -DEGAS_Y / 2.0 + 30.0, DEGAS_Z / 2.0 + 4.0);
        vents = vents + (boss - cut);
    }

    let mut ticks = Part::empty(format!("{PREFIX}_bubble_degas_witness_tick_marks"));
    for i in 0..WITNESS_TICKS {
        let y = centered_index(i, WITNESS_TICKS, 12.0) + 4.0;
        let tick_x = if i % 2 == 0 { -128.0 } else { 128.0 };
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_bubble_degas_tick_mark_{i}"),
                34.0,
                3.5,
                5.0,
            )
            .translate(tick_x, y, DEGAS_Z / 2.0 + 2.5);
    }

    let lower_flow_lane = centered_cube(
        format!("{PREFIX}_bubble_degas_lower_flow_lane_saddle"),
        DEGAS_X - 54.0,
        14.0,
        11.0,
    )
    .translate(0.0, -72.0, DEGAS_Z / 2.0 + 5.5);

    (frame - central_window - risers + vents + ticks + lower_flow_lane).translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1,
        deck_z(DEGAS_Z),
    )
}

fn cassette_inlet_outlet_reference_pockets() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_cassette_inlet_outlet_reference_pocket_panel"),
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    );

    let mut pockets = Part::empty(format!("{PREFIX}_cassette_reference_pocket_cuts"));
    let mut pocket_rims = Part::empty(format!("{PREFIX}_cassette_reference_pocket_rims"));
    let mut thermowells = Part::empty(format!("{PREFIX}_cassette_reference_thermowells"));
    for (i, role) in REFERENCE_ROLES.into_iter().enumerate() {
        let y = centered_index(i, REFERENCE_POCKETS, 108.0);
        pockets = pockets
            + centered_cube(
                format!("{PREFIX}_{role}_pocket_clearance"),
                REFERENCE_POCKET_X,
                REFERENCE_POCKET_Y,
                REFERENCE_Z + 5.0,
            )
            .translate(-44.0, y, 0.0);

        pocket_rims = pocket_rims
            + centered_cube(
                format!("{PREFIX}_{role}_pocket_reference_rim"),
                REFERENCE_POCKET_X + 16.0,
                REFERENCE_POCKET_Y + 14.0,
                7.0,
            )
            .translate(-44.0, y, REFERENCE_Z / 2.0 + 3.5)
            - centered_cube(
                format!("{PREFIX}_{role}_pocket_reference_rim_opening"),
                REFERENCE_POCKET_X - 6.0,
                REFERENCE_POCKET_Y - 4.0,
                8.0,
            )
            .translate(-44.0, y, REFERENCE_Z / 2.0 + 3.5);

        for well in 0..REFERENCE_THERMOWELLS_PER_POCKET {
            thermowells = thermowells
                + centered_cylinder(
                    format!("{PREFIX}_{role}_thermowell_{well}"),
                    3.4,
                    REFERENCE_Z + 8.0,
                    20,
                )
                .translate(
                    100.0,
                    y + centered_index(well, REFERENCE_THERMOWELLS_PER_POCKET, 24.0),
                    0.0,
                );
        }
    }

    let mut token_slots = Part::empty(format!("{PREFIX}_cassette_reference_delta_token_slots"));
    for i in 0..REFERENCE_TOKEN_SLOTS {
        token_slots = token_slots
            + centered_cube(
                format!("{PREFIX}_cassette_reference_delta_token_slot_{i}"),
                44.0,
                22.0,
                8.0,
            )
            .translate(
                centered_index(i % 3, 3, 56.0) + 28.0,
                centered_index(i / 3, 2, 60.0),
                REFERENCE_Z / 2.0 + 4.0,
            );
    }

    (body - pockets - thermowells + pocket_rims + token_slots).translate(
        REFERENCE_CENTER.0,
        REFERENCE_CENTER.1,
        deck_z(REFERENCE_Z),
    )
}

fn waste_retain_split_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_waste_retain_split_manifold_body"),
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    );

    let incoming = centered_cylinder(
        format!("{PREFIX}_waste_retain_incoming_flow_bore"),
        4.4,
        SPLIT_X + 14.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -4.0);

    let mut port_cuts = Part::empty(format!("{PREFIX}_waste_retain_split_port_cuts"));
    let mut port_bosses = Part::empty(format!("{PREFIX}_waste_retain_split_port_bosses"));
    for i in 0..SPLIT_STREAMS {
        let y = centered_index(i, SPLIT_STREAMS, 52.0);
        port_cuts = port_cuts
            + centered_cylinder(
                format!("{PREFIX}_waste_retain_stream_port_clearance_{i}"),
                SPLIT_PORT_D / 2.0,
                SPLIT_Z + 6.0,
                32,
            )
            .translate(76.0, y, 0.0);
        port_bosses = port_bosses
            + centered_cylinder(
                format!("{PREFIX}_waste_retain_stream_port_boss_{i}"),
                SPLIT_PORT_D / 2.0 + 8.0,
                7.0,
                32,
            )
            .translate(76.0, y, SPLIT_Z / 2.0 + 3.5);
    }

    let diverter_token = centered_cube(
        format!("{PREFIX}_waste_retain_diverter_state_token_slot"),
        86.0,
        30.0,
        8.0,
    )
    .translate(-70.0, 0.0, SPLIT_Z / 2.0 + 4.0);

    let volume_badge = centered_cube(
        format!("{PREFIX}_waste_retain_volume_capacity_badge"),
        SPLIT_X - 58.0,
        12.0,
        6.0,
    )
    .translate(0.0, SPLIT_Y / 2.0 - 18.0, SPLIT_Z / 2.0 + 3.0);

    (body - incoming - port_cuts + port_bosses + diverter_token + volume_badge).translate(
        SPLIT_CENTER.0,
        SPLIT_CENTER.1,
        deck_z(SPLIT_Z),
    )
}

fn release_hold_reject_gates() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_panel"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );

    let mut lane_slots = Part::empty(format!("{PREFIX}_release_hold_reject_gate_slot_cuts"));
    let mut lane_labels = Part::empty(format!("{PREFIX}_release_hold_reject_gate_lane_labels"));
    for (lane, label) in DISPOSITION_GATES.into_iter().enumerate() {
        let x = centered_index(lane, GATE_LANES, GATE_LANE_PITCH_X);
        for slot in 0..GATE_TOKEN_SLOTS_PER_LANE {
            lane_slots = lane_slots
                + centered_cube(
                    format!("{PREFIX}_{label}_gate_token_slot_{slot}"),
                    58.0,
                    GATE_SLOT_Y,
                    GATE_Z + 4.0,
                )
                .translate(
                    x,
                    centered_index(slot, GATE_TOKEN_SLOTS_PER_LANE, 17.0),
                    0.0,
                );
        }

        lane_labels =
            lane_labels
                + centered_cube(
                    format!("{PREFIX}_{label}_gate_front_label_land"),
                    72.0,
                    13.0,
                    6.0,
                )
                .translate(x, -GATE_Y / 2.0 + 14.0, GATE_Z / 2.0 + 3.0)
                + centered_cube(format!("{PREFIX}_{label}_gate_backstop"), 86.0, 8.0, 18.0)
                    .translate(x, GATE_Y / 2.0 - 12.0, GATE_Z / 2.0 + 9.0);
    }

    (panel - lane_slots + lane_labels).translate(GATE_CENTER.0, GATE_CENTER.1, deck_z(GATE_Z))
}

fn run_record_custody_lands() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_run_record_custody_land_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    let mut barcode_lands = Part::empty(format!("{PREFIX}_run_record_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        barcode_lands = barcode_lands
            + centered_cube(
                format!("{PREFIX}_run_record_barcode_land_{i}"),
                70.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(i % 4, 4, 82.0),
                centered_index(i / 4, 2, 28.0) + 18.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    let mut delta_tokens = Part::empty(format!("{PREFIX}_run_record_step_delta_token_lands"));
    for i in 0..STEP_DELTA_TOKENS {
        delta_tokens = delta_tokens
            + centered_cylinder(
                format!("{PREFIX}_run_record_step_delta_token_land_{i}"),
                12.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, STEP_DELTA_TOKENS, 42.0),
                -CUSTODY_Y / 2.0 + 22.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    let mut cards = Part::empty(format!("{PREFIX}_run_record_card_slots"));
    for i in 0..RUN_RECORD_CARDS {
        let land = centered_cube(
            format!("{PREFIX}_run_record_card_slot_{i}"),
            60.0,
            24.0,
            7.0,
        )
        .translate(
            centered_index(i, RUN_RECORD_CARDS, 78.0),
            CUSTODY_Y / 2.0 - 20.0,
            CUSTODY_Z / 2.0 + 3.5,
        );
        let relief = centered_cube(
            format!("{PREFIX}_run_record_card_grip_relief_{i}"),
            48.0,
            12.0,
            8.0,
        )
        .translate(
            centered_index(i, RUN_RECORD_CARDS, 78.0),
            CUSTODY_Y / 2.0 - 20.0,
            CUSTODY_Z / 2.0 + 3.5,
        );
        cards = cards + (land - relief);
    }

    (panel + barcode_lands + delta_tokens + cards).translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        deck_z(CUSTODY_Z),
    )
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_sweep_keepout_gauge"),
        STATION_X - 210.0,
        10.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + 50.0,
        BASE_Z + ROBOT_KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_thermal_service_keepout_gauge"),
        STATION_X - 220.0,
        10.0,
        90.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 52.0, BASE_Z + 45.0);
    let left = centered_cube(
        format!("{PREFIX}_left_cartridge_service_keepout_gauge"),
        10.0,
        STATION_Y - 180.0,
        96.0,
    )
    .translate(-STATION_X / 2.0 + 58.0, 0.0, BASE_Z + 48.0);
    let right = centered_cube(
        format!("{PREFIX}_right_sensor_service_keepout_gauge"),
        10.0,
        STATION_Y - 180.0,
        96.0,
    )
    .translate(STATION_X / 2.0 - 58.0, 0.0, BASE_Z + 48.0);

    let mut gauges = front + rear + left + right;
    for i in 0..KEEP_OUT_GAUGES {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_vertical_keepout_height_flag_{i}"),
                24.0,
                18.0,
                110.0,
            )
            .translate(
                centered_index(i, KEEP_OUT_GAUGES, 150.0),
                0.0,
                BASE_Z + 55.0,
            );
    }
    gauges
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_scoped_unique_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn dimensions_keep_modules_inside_containment_deck() {
        assert_layout();
        assert_eq!(STATION_X, 1420.0);
        assert_eq!(STATION_Y, 860.0);
        assert!(BASE_Z >= 22.0);
        assert!(BASIN_DEPTH >= 8.0);
        assert_eq!(module_specs().len(), 9);

        for module in module_specs() {
            assert!(
                module.fits_station(14.0),
                "{} should fit inside the raised containment rim",
                module.name
            );
        }
    }

    #[test]
    fn feature_contract_covers_step_response_path() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "{feature} missing output STL"
            );
        }

        assert!(REQUIRED_FEATURES.contains(&"warm_cold_media_cartridge_nests"));
        assert!(REQUIRED_FEATURES.contains(&"heat_exchanger_surrogate_block"));
        assert!(REQUIRED_FEATURES.contains(&"inline_temperature_sensor_ladder"));
        assert!(REQUIRED_FEATURES.contains(&"residence_time_loop"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_degas_witness_window"));
        assert!(REQUIRED_FEATURES.contains(&"cassette_inlet_outlet_reference_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"waste_retain_split_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_gates"));
    }

    #[test]
    fn validation_counts_are_sized_for_step_response_evidence() {
        assert_eq!(MEDIA_STATES, ["warm_media", "cold_media"]);
        assert_eq!(REFERENCE_ROLES.len(), 2);
        assert_eq!(DISPOSITION_GATES, ["release", "hold", "reject"]);
        assert_eq!(INLINE_SENSOR_COUNT, 8);
        assert_eq!(REFERENCE_THERMOWELLS_PER_POCKET * REFERENCE_POCKETS, 6);
        assert_eq!(GATE_TOKEN_SLOTS, 15);
        assert!(EXCHANGER_CHANNELS >= 6);
        assert!(RESIDENCE_SEGMENTS >= 7);
        assert!(WITNESS_TICKS >= 12);
        assert!(SPLIT_STREAMS == 2);
    }
}
