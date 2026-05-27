use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Automated cell seeding and ECM/coating station for the scalable tissue-chip cassette.
//
// Intent:
// - Replace manual pipetting with a deck-level fluid path for cell suspension,
//   ECM/coating, media, prime, and waste routing.
// - Keep cassette datum geometry compatible with the 4x5 Rev C culture cassette.
// - Show mixer/reservoir, degassing/bubble trap, metering pumps, valves/manifold,
//   pressure/flow/bubble sensing, and front/service clearances as CAD envelopes.
// - Preserve sterile-fluid-path decisions as placeholders; this is mechanical
//   architecture for automation planning, not a validated biological process.

const COLS: usize = 4;
const ROWS: usize = 5;
const METERED_PUMP_LANES: usize = ROWS + 2; // five cassette rows plus coating and prime lanes.
const VALVE_FLUID_PATHS: usize = 4; // cell, coating/media, prime, waste.
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.6;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const ROW_TRUNK_D: f64 = 6.0;

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const NEST_X: f64 = CASSETTE_X + 150.0;
const NEST_Y: f64 = CASSETTE_Y + 120.0;
const NEST_Z: f64 = 12.0;
const DECK_Z: f64 = 18.0;

const STATION_X: f64 = NEST_X + 530.0;
const STATION_Y: f64 = NEST_Y + 250.0;
const CASSETTE_ORIGIN_X: f64 = STATION_X / 2.0 - NEST_X / 2.0 - 56.0;
const CASSETTE_ORIGIN_Y: f64 = -24.0;

const FLUID_COLUMN_X: f64 = -STATION_X / 2.0 + 178.0;
const DEGASSER_X: f64 = -STATION_X / 2.0 + 330.0;
const PUMP_BANK_X: f64 = -STATION_X / 2.0 + 324.0;
const VALVE_MANIFOLD_X: f64 = -STATION_X / 2.0 + 454.0;
const SENSOR_BRIDGE_X: f64 = CASSETTE_ORIGIN_X - NEST_X / 2.0 - 36.0;
const RESERVOIR_Y: f64 = STATION_Y / 2.0 - 142.0;
const DEGASSER_Y: f64 = 128.0;
const PUMP_BANK_Y: f64 = -72.0;
const VALVE_MANIFOLD_Y: f64 = -250.0;
const SENSOR_BRIDGE_Y: f64 = -250.0;
const WASTE_Y: f64 = -STATION_Y / 2.0 + 96.0;

const RESERVOIR_TRAY_X: f64 = 340.0;
const RESERVOIR_TRAY_Y: f64 = 282.0;
const RESERVOIR_TRAY_Z: f64 = 24.0;
const DEGASSER_BLOCK_X: f64 = 382.0;
const DEGASSER_BLOCK_Y: f64 = 122.0;
const DEGASSER_BLOCK_Z: f64 = 32.0;
const PUMP_BANK_LEN_X: f64 = 380.0;
const PUMP_BANK_LEN_Y: f64 = 236.0;
const PUMP_BANK_Z: f64 = 32.0;
const VALVE_BLOCK_X: f64 = 430.0;
const VALVE_BLOCK_Y: f64 = 156.0;
const VALVE_BLOCK_Z: f64 = 46.0;
const SENSOR_BRIDGE_LEN_X: f64 = 366.0;
const SENSOR_BRIDGE_LEN_Y: f64 = 104.0;
const SENSOR_BRIDGE_Z: f64 = 28.0;
const WASTE_TRAY_X: f64 = 368.0;
const WASTE_TRAY_Y: f64 = 142.0;
const WASTE_TRAY_Z: f64 = 26.0;

const SERVICE_CLEARANCE_X: f64 = CASSETTE_X + 188.0;
const SERVICE_CLEARANCE_Y: f64 = CASSETTE_Y + 166.0;
const SERVICE_CLEARANCE_Z: f64 = 158.0;
const FRONT_SERVICE_CLEARANCE: f64 = 92.0;

fn main() {
    let baseplate = station_baseplate();
    baseplate
        .write_stl("output/automated_seeding_coating_station_baseplate.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_baseplate.stl");

    let cassette_nest = cassette_nest();
    cassette_nest
        .write_stl("output/automated_seeding_coating_station_cassette_nest.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_cassette_nest.stl");

    let reservoir_mixer = reservoir_mixer_tray();
    reservoir_mixer
        .write_stl("output/automated_seeding_coating_station_reservoir_mixer_tray.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_reservoir_mixer_tray.stl");

    let degasser = degassing_bubble_trap_path();
    degasser
        .write_stl("output/automated_seeding_coating_station_degassing_bubble_trap.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_degassing_bubble_trap.stl");

    let waste = priming_waste_routing_tray();
    waste
        .write_stl("output/automated_seeding_coating_station_priming_waste_routing.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_priming_waste_routing.stl");

    let pump_bank = metering_pump_bank();
    pump_bank
        .write_stl("output/automated_seeding_coating_station_metering_pump_bank.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_metering_pump_bank.stl");

    let valve_manifold = valve_manifold_region();
    valve_manifold
        .write_stl("output/automated_seeding_coating_station_valve_manifold.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_valve_manifold.stl");

    let sensors = sensor_bridge();
    sensors
        .write_stl("output/automated_seeding_coating_station_sensor_bridge.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_sensor_bridge.stl");

    let clearance = service_clearance_frame();
    clearance
        .write_stl("output/automated_seeding_coating_station_service_clearance_frame.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_service_clearance_frame.stl");

    let assembly = baseplate
        + cassette_nest.translate(
            CASSETTE_ORIGIN_X,
            CASSETTE_ORIGIN_Y,
            DECK_Z / 2.0 + NEST_Z / 2.0,
        )
        + reservoir_mixer.translate(
            FLUID_COLUMN_X,
            RESERVOIR_Y,
            DECK_Z / 2.0 + RESERVOIR_TRAY_Z / 2.0,
        )
        + degasser.translate(
            DEGASSER_X,
            DEGASSER_Y,
            DECK_Z / 2.0 + DEGASSER_BLOCK_Z / 2.0,
        )
        + pump_bank.translate(PUMP_BANK_X, PUMP_BANK_Y, DECK_Z / 2.0 + PUMP_BANK_Z / 2.0)
        + valve_manifold.translate(
            VALVE_MANIFOLD_X,
            VALVE_MANIFOLD_Y,
            DECK_Z / 2.0 + VALVE_BLOCK_Z / 2.0,
        )
        + sensors.translate(
            SENSOR_BRIDGE_X,
            SENSOR_BRIDGE_Y,
            DECK_Z / 2.0 + SENSOR_BRIDGE_Z / 2.0,
        )
        + waste.translate(FLUID_COLUMN_X, WASTE_Y, DECK_Z / 2.0 + WASTE_TRAY_Z / 2.0)
        + clearance.translate(CASSETTE_ORIGIN_X, CASSETTE_ORIGIN_Y, DECK_Z / 2.0);

    assembly
        .write_stl("output/automated_seeding_coating_station_assembly.stl")
        .unwrap();
    println!("Exported: output/automated_seeding_coating_station_assembly.stl");

    println!(
        "Automated seeding/coating station: {:.0}mm x {:.0}mm deck, 4x5 Rev C cassette nest ({:.0}mm x {:.0}mm cassette envelope), {} metered pump lanes, {} valve fluid paths, {:.0}mm cassette service clearance height, and {:.0}mm front service approach.",
        STATION_X,
        STATION_Y,
        CASSETTE_X,
        CASSETTE_Y,
        METERED_PUMP_LANES,
        VALVE_FLUID_PATHS,
        SERVICE_CLEARANCE_Z,
        FRONT_SERVICE_CLEARANCE
    );
}

fn station_baseplate() -> Part {
    let deck = centered_cube(
        "automated_seeding_station_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );

    let cassette_leak_basin = centered_cube(
        "automated_seeding_station_cassette_leak_basin",
        NEST_X - 42.0,
        NEST_Y - 42.0,
        8.0,
    )
    .translate(CASSETTE_ORIGIN_X, CASSETTE_ORIGIN_Y, DECK_Z / 2.0 - 3.5);

    let fluid_bay_drip_basin = centered_cube(
        "automated_seeding_station_fluid_bay_drip_basin",
        498.0,
        STATION_Y - 156.0,
        8.0,
    )
    .translate(-STATION_X / 2.0 + 274.0, -10.0, DECK_Z / 2.0 - 3.5);

    let priming_spill_gutter = centered_cube(
        "automated_seeding_station_priming_spill_gutter",
        456.0,
        30.0,
        10.0,
    )
    .translate(FLUID_COLUMN_X, WASTE_Y + 90.0, DECK_Z / 2.0 - 4.0);
    let waste_drain_port = centered_cylinder(
        "automated_seeding_station_waste_drain_port",
        7.0 / 2.0,
        36.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FLUID_COLUMN_X + 136.0, -STATION_Y / 2.0 + 16.0, 0.0);

    let front_service_keepout = centered_cube(
        "automated_seeding_station_front_service_keepout_relief",
        SERVICE_CLEARANCE_X,
        FRONT_SERVICE_CLEARANCE,
        5.0,
    )
    .translate(
        CASSETTE_ORIGIN_X,
        CASSETTE_ORIGIN_Y - SERVICE_CLEARANCE_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        DECK_Z / 2.0 - 2.5,
    );

    let mut mount_slots = Part::empty("automated_seeding_station_mount_slots");
    for (i, (x, y)) in station_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("automated_seeding_station_m6_clearance_{i}"),
            6.6 / 2.0,
            DECK_Z + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("automated_seeding_station_m6_slot_{i}"),
            24.0,
            6.8,
            DECK_Z + 2.0,
        )
        .translate(*x, *y, 0.0);
        mount_slots = mount_slots + hole + slot;
    }

    deck - cassette_leak_basin
        - fluid_bay_drip_basin
        - priming_spill_gutter
        - waste_drain_port
        - front_service_keepout
        - mount_slots
        + deck_perimeter_rails()
        + station_leveling_feet()
        + deck_locator_targets()
        + fluid_path_locator_strip()
}

fn deck_perimeter_rails() -> Part {
    let rear = centered_cube(
        "automated_seeding_station_rear_service_rail",
        STATION_X - 58.0,
        18.0,
        26.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 28.0, DECK_Z / 2.0 + 13.0);
    let left = centered_cube(
        "automated_seeding_station_left_fluid_rail",
        18.0,
        STATION_Y - 70.0,
        26.0,
    )
    .translate(-STATION_X / 2.0 + 28.0, 0.0, DECK_Z / 2.0 + 13.0);
    let right = centered_cube(
        "automated_seeding_station_right_cassette_rail",
        18.0,
        STATION_Y - 70.0,
        26.0,
    )
    .translate(STATION_X / 2.0 - 28.0, 0.0, DECK_Z / 2.0 + 13.0);
    let front_left = centered_cube(
        "automated_seeding_station_front_left_low_lip",
        360.0,
        12.0,
        14.0,
    )
    .translate(
        -STATION_X / 2.0 + 220.0,
        -STATION_Y / 2.0 + 22.0,
        DECK_Z / 2.0 + 7.0,
    );
    let front_right = centered_cube(
        "automated_seeding_station_front_right_low_lip",
        360.0,
        12.0,
        14.0,
    )
    .translate(
        STATION_X / 2.0 - 220.0,
        -STATION_Y / 2.0 + 22.0,
        DECK_Z / 2.0 + 7.0,
    );

    rear + left + right + front_left + front_right
}

fn station_leveling_feet() -> Part {
    let mut feet = Part::empty("automated_seeding_station_leveling_feet");
    for (i, (x, y)) in station_mount_points().iter().enumerate() {
        let pad = centered_cylinder(
            format!("automated_seeding_station_leveling_pad_{i}"),
            21.0,
            9.0,
            36,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 4.5));
        let adjuster = centered_cylinder(
            format!("automated_seeding_station_leveling_adjuster_{i}"),
            8.0 / 2.0,
            18.0,
            24,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 4.5));
        feet = feet + (pad - adjuster);
    }
    feet
}

fn deck_locator_targets() -> Part {
    let mut targets = Part::empty("automated_seeding_station_deck_locator_targets");
    for (i, (x, y)) in [
        (
            CASSETTE_ORIGIN_X - NEST_X / 2.0 + 38.0,
            CASSETTE_ORIGIN_Y + NEST_Y / 2.0 - 36.0,
        ),
        (
            CASSETTE_ORIGIN_X + NEST_X / 2.0 - 38.0,
            CASSETTE_ORIGIN_Y + NEST_Y / 2.0 - 36.0,
        ),
        (
            CASSETTE_ORIGIN_X + NEST_X / 2.0 - 38.0,
            CASSETTE_ORIGIN_Y - NEST_Y / 2.0 + 36.0,
        ),
        (
            CASSETTE_ORIGIN_X - NEST_X / 2.0 + 38.0,
            CASSETTE_ORIGIN_Y - NEST_Y / 2.0 + 36.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("automated_seeding_station_deck_datum_target_{i}"),
            9.0,
            2.4,
            40,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 1.2);
        let bore = centered_cylinder(
            format!("automated_seeding_station_deck_datum_center_{i}"),
            2.0,
            3.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 1.2);
        targets = targets + (target - bore);
    }
    targets
}

fn fluid_path_locator_strip() -> Part {
    let spine = centered_cube(
        "automated_seeding_station_fluid_path_locator_spine",
        24.0,
        STATION_Y - 210.0,
        10.0,
    )
    .translate(SENSOR_BRIDGE_X + 142.0, -22.0, DECK_Z / 2.0 + 5.0);

    let mut row_marks = Part::empty("automated_seeding_station_row_feed_marks");
    for row in 0..ROWS {
        row_marks = row_marks
            + centered_cube(
                format!("automated_seeding_station_row_{row}_feed_mark"),
                72.0,
                6.0,
                8.0,
            )
            .translate(
                SENSOR_BRIDGE_X + 118.0,
                CASSETTE_ORIGIN_Y + row_y(row),
                DECK_Z / 2.0 + 4.0,
            );
    }

    spine + row_marks
}

fn cassette_nest() -> Part {
    let tray = centered_cube(
        "automated_seeding_cassette_nest_tray",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cassette_recess = centered_cube(
        "automated_seeding_cassette_nest_recess",
        CASSETTE_X + 10.0,
        CASSETTE_Y + 10.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, 0.0, 3.6);

    let left_tube_trench = centered_cube(
        "automated_seeding_cassette_nest_left_tube_trench",
        42.0,
        NEST_Y + 2.0,
        NEST_Z + 2.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 42.0), 0.0, 1.8);
    let right_tube_trench = centered_cube(
        "automated_seeding_cassette_nest_right_tube_trench",
        42.0,
        NEST_Y + 2.0,
        NEST_Z + 2.0,
    )
    .translate(CASSETTE_X / 2.0 + 42.0, 0.0, 1.8);
    let rear_manifold_clearance = centered_cube(
        "automated_seeding_cassette_nest_rear_manifold_clearance",
        CASSETTE_X + 86.0,
        46.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 46.0), 1.8);

    let drain_sump = centered_cube(
        "automated_seeding_cassette_nest_drain_sump",
        80.0,
        46.0,
        NEST_Z + 2.0,
    )
    .translate(NEST_X / 2.0 - 55.0, -NEST_Y / 2.0 + 42.0, 1.8);
    let drain_port = centered_cylinder("automated_seeding_cassette_nest_drain_port", 5.5, 28.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(NEST_X / 2.0 - 55.0, -NEST_Y / 2.0 + 16.0, 0.0);

    tray - cassette_recess
        - left_tube_trench
        - right_tube_trench
        - rear_manifold_clearance
        - drain_sump
        - drain_port
        + cassette_datum_rails()
        + cassette_latch_posts()
        + cassette_row_alignment_comb()
        + cassette_robot_fiducials()
}

fn cassette_datum_rails() -> Part {
    let z = NEST_Z / 2.0 + 11.0;
    let rear = centered_cube(
        "automated_seeding_cassette_rear_y_datum_rail",
        CASSETTE_X + 38.0,
        16.0,
        22.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 18.0, z);
    let left = centered_cube(
        "automated_seeding_cassette_left_x_datum_rail",
        16.0,
        CASSETTE_Y + 34.0,
        22.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 18.0), 0.0, z);
    let right_soft = centered_cube(
        "automated_seeding_cassette_right_soft_guide_rail",
        14.0,
        CASSETTE_Y + 34.0,
        15.0,
    )
    .translate(CASSETTE_X / 2.0 + 18.0, 0.0, z - 3.5);
    let front_low = centered_cube(
        "automated_seeding_cassette_front_low_load_lip",
        CASSETTE_X - 76.0,
        10.0,
        12.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 8.0), z - 5.0);

    rear + left + right_soft + front_low
}

fn cassette_latch_posts() -> Part {
    let mut posts = Part::empty("automated_seeding_cassette_latch_posts");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 40.0), -(CASSETTE_Y / 2.0 - 36.0)),
        (CASSETTE_X / 2.0 - 40.0, -(CASSETTE_Y / 2.0 - 36.0)),
        (-(CASSETTE_X / 2.0 - 40.0), CASSETTE_Y / 2.0 - 36.0),
        (CASSETTE_X / 2.0 - 40.0, CASSETTE_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cylinder(
            format!("automated_seeding_cassette_latch_post_{i}"),
            8.0,
            22.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 11.0);
        let screw = centered_cylinder(
            format!("automated_seeding_cassette_latch_screw_{i}"),
            3.4 / 2.0,
            24.0,
            20,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 11.0);
        posts = posts + (post - screw);
    }
    posts
}

fn cassette_row_alignment_comb() -> Part {
    let mut comb = Part::empty("automated_seeding_cassette_row_alignment_comb");
    for row in 0..ROWS {
        let y = row_y(row);
        let row_bar = centered_cube(
            format!("automated_seeding_cassette_row_{row}_alignment_bar"),
            CASSETTE_X + 62.0,
            5.0,
            7.0,
        )
        .translate(0.0, y, NEST_Z / 2.0 + 3.5);
        let row_tube_relief = centered_cylinder(
            format!("automated_seeding_cassette_row_{row}_tube_relief"),
            ROW_TRUNK_D / 2.0,
            CASSETTE_X + 82.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, NEST_Z / 2.0 + 8.0);
        comb = comb + (row_bar - row_tube_relief);
    }
    for col in 0..COLS {
        let x = chip_x(col);
        comb = comb
            + centered_cube(
                format!("automated_seeding_cassette_col_{col}_alignment_bar"),
                5.0,
                CASSETTE_Y + 44.0,
                5.0,
            )
            .translate(x, 0.0, NEST_Z / 2.0 + 2.5);
    }
    comb
}

fn cassette_robot_fiducials() -> Part {
    let mut targets = Part::empty("automated_seeding_cassette_robot_fiducials");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 32.0), CASSETTE_Y / 2.0 - 26.0),
        (CASSETTE_X / 2.0 - 32.0, CASSETTE_Y / 2.0 - 26.0),
        (CASSETTE_X / 2.0 - 32.0, -(CASSETTE_Y / 2.0 - 26.0)),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("automated_seeding_cassette_fiducial_target_{i}"),
            7.5,
            2.2,
            40,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 1.1);
        let center = centered_cylinder(
            format!("automated_seeding_cassette_fiducial_center_{i}"),
            1.6,
            3.0,
            20,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 1.1);
        targets = targets + (target - center);
    }
    targets
}

fn reservoir_mixer_tray() -> Part {
    let tray = centered_cube(
        "automated_seeding_reservoir_tray_body",
        RESERVOIR_TRAY_X,
        RESERVOIR_TRAY_Y,
        RESERVOIR_TRAY_Z,
    );

    let cell = reservoir_well_cut(
        "cell_suspension_mixer",
        43.0,
        -88.0,
        48.0,
        RESERVOIR_TRAY_Z + 2.0,
    );
    let coating_a = reservoir_well_cut("ecm_coating_a", 31.0, 62.0, 66.0, RESERVOIR_TRAY_Z + 2.0);
    let coating_b = reservoir_well_cut("ecm_coating_b", 31.0, 62.0, -18.0, RESERVOIR_TRAY_Z + 2.0);
    let media = reservoir_well_cut("media", 35.0, -72.0, -72.0, RESERVOIR_TRAY_Z + 2.0);
    let prime = reservoir_well_cut("prime_wash", 25.0, 130.0, -78.0, RESERVOIR_TRAY_Z + 2.0);

    let mixer_motor_pocket = centered_cylinder(
        "automated_seeding_cell_suspension_mixer_motor_pocket",
        34.0,
        10.0,
        48,
    )
    .translate(-88.0, 48.0, -RESERVOIR_TRAY_Z / 2.0 + 4.0);
    let stir_bar_view_slot = centered_cube(
        "automated_seeding_cell_suspension_stir_bar_view_slot",
        72.0,
        14.0,
        8.0,
    )
    .translate(-88.0, 48.0, RESERVOIR_TRAY_Z / 2.0 - 4.0);

    let moat = centered_cube(
        "automated_seeding_reservoir_tray_drip_moat",
        RESERVOIR_TRAY_X - 42.0,
        16.0,
        8.0,
    )
    .translate(
        0.0,
        -RESERVOIR_TRAY_Y / 2.0 + 24.0,
        RESERVOIR_TRAY_Z / 2.0 - 3.5,
    );

    let mut screw_holes = Part::empty("automated_seeding_reservoir_tray_screw_holes");
    for (i, (x, y)) in [
        (
            -(RESERVOIR_TRAY_X / 2.0 - 18.0),
            -(RESERVOIR_TRAY_Y / 2.0 - 18.0),
        ),
        (
            RESERVOIR_TRAY_X / 2.0 - 18.0,
            -(RESERVOIR_TRAY_Y / 2.0 - 18.0),
        ),
        (
            -(RESERVOIR_TRAY_X / 2.0 - 18.0),
            RESERVOIR_TRAY_Y / 2.0 - 18.0,
        ),
        (RESERVOIR_TRAY_X / 2.0 - 18.0, RESERVOIR_TRAY_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        screw_holes = screw_holes
            + centered_cylinder(
                format!("automated_seeding_reservoir_m4_hole_{i}"),
                4.3 / 2.0,
                RESERVOIR_TRAY_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    tray - cell
        - coating_a
        - coating_b
        - media
        - prime
        - mixer_motor_pocket
        - stir_bar_view_slot
        - moat
        - screw_holes
        + reservoir_retaining_rings()
        + reservoir_pickup_manifold()
}

fn reservoir_retaining_rings() -> Part {
    reservoir_ring("cell_suspension_mixer", 43.0, -88.0, 48.0)
        + reservoir_ring("ecm_coating_a", 31.0, 62.0, 66.0)
        + reservoir_ring("ecm_coating_b", 31.0, 62.0, -18.0)
        + reservoir_ring("media", 35.0, -72.0, -72.0)
        + reservoir_ring("prime_wash", 25.0, 130.0, -78.0)
}

fn reservoir_pickup_manifold() -> Part {
    let pickup_bar = centered_cube(
        "automated_seeding_reservoir_pickup_manifold_bar",
        RESERVOIR_TRAY_X - 60.0,
        18.0,
        18.0,
    )
    .translate(
        0.0,
        -RESERVOIR_TRAY_Y / 2.0 + 46.0,
        RESERVOIR_TRAY_Z / 2.0 + 9.0,
    );
    let mut cuts = Part::empty("automated_seeding_reservoir_pickup_tube_cuts");
    for (i, x) in [-122.0, -58.0, 6.0, 70.0, 134.0].iter().enumerate() {
        let tube = centered_cylinder(
            format!("automated_seeding_reservoir_pickup_tube_{i}"),
            FLUID_BORE_D / 2.0,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            *x,
            -RESERVOIR_TRAY_Y / 2.0 + 46.0,
            RESERVOIR_TRAY_Z / 2.0 + 9.0,
        );
        let slot = centered_cube(
            format!("automated_seeding_reservoir_pickup_slot_{i}"),
            FLUID_BORE_D + 1.0,
            24.0,
            10.0,
        )
        .translate(
            *x,
            -RESERVOIR_TRAY_Y / 2.0 + 46.0,
            RESERVOIR_TRAY_Z / 2.0 + 13.0,
        );
        cuts = cuts + tube + slot;
    }
    pickup_bar - cuts
}

fn reservoir_well_cut(name: &str, radius: f64, x: f64, y: f64, height: f64) -> Part {
    centered_cylinder(
        format!("automated_seeding_{name}_reservoir_socket"),
        radius,
        height,
        48,
    )
    .translate(x, y, 0.0)
}

fn reservoir_ring(name: &str, radius: f64, x: f64, y: f64) -> Part {
    let outer = centered_cylinder(
        format!("automated_seeding_{name}_retaining_ring_outer"),
        radius + 6.0,
        9.0,
        56,
    )
    .translate(x, y, RESERVOIR_TRAY_Z / 2.0 + 4.5);
    let inner = centered_cylinder(
        format!("automated_seeding_{name}_retaining_ring_inner"),
        radius + 0.6,
        10.0,
        56,
    )
    .translate(x, y, RESERVOIR_TRAY_Z / 2.0 + 4.5);
    let key = centered_cube(
        format!("automated_seeding_{name}_asymmetric_key_flat"),
        18.0,
        12.0,
        10.0,
    )
    .translate(
        x + radius + 1.0,
        y + radius * 0.45,
        RESERVOIR_TRAY_Z / 2.0 + 4.5,
    );

    outer - inner - key
}

fn degassing_bubble_trap_path() -> Part {
    let body = centered_cube(
        "automated_seeding_degassing_path_body",
        DEGASSER_BLOCK_X,
        DEGASSER_BLOCK_Y,
        DEGASSER_BLOCK_Z,
    );

    let membrane_pocket = centered_cube(
        "automated_seeding_membrane_degasser_cartridge_pocket",
        138.0,
        58.0,
        18.0,
    )
    .translate(-96.0, 0.0, DEGASSER_BLOCK_Z / 2.0 - 8.0);
    let view_slot = centered_cube(
        "automated_seeding_degasser_clear_view_slot",
        310.0,
        14.0,
        10.0,
    )
    .translate(16.0, 0.0, DEGASSER_BLOCK_Z / 2.0 - 4.0);

    body - serpentine_degas_channels() - membrane_pocket - view_slot - degasser_mount_holes()
        + bubble_trap_towers()
        + vacuum_port_placeholder()
        + degasser_connector_tabs()
}

fn serpentine_degas_channels() -> Part {
    let mut channels = Part::empty("automated_seeding_serpentine_degas_channels");
    let lane_ys = [-42.0, -21.0, 0.0, 21.0, 42.0];
    for (i, y) in lane_ys.iter().enumerate() {
        let straight = centered_cylinder(
            format!("automated_seeding_degas_straight_bore_{i}"),
            FLUID_BORE_D / 2.0,
            DEGASSER_BLOCK_X - 44.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, *y, 0.0);
        let top_slot = centered_cube(
            format!("automated_seeding_degas_open_slot_{i}"),
            DEGASSER_BLOCK_X - 48.0,
            FLUID_BORE_D + 1.0,
            12.0,
        )
        .translate(0.0, *y, 9.0);
        channels = channels + straight + top_slot;
    }
    for (i, (x, y)) in [
        (-164.0, -31.5),
        (164.0, -10.5),
        (-164.0, 10.5),
        (164.0, 31.5),
    ]
    .iter()
    .enumerate()
    {
        channels = channels
            + centered_cylinder(
                format!("automated_seeding_degas_u_turn_{i}"),
                FLUID_BORE_D / 2.0,
                28.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, *y, 0.0);
    }
    channels
}

fn degasser_mount_holes() -> Part {
    let mut holes = Part::empty("automated_seeding_degasser_mount_holes");
    for (i, (x, y)) in [
        (
            -(DEGASSER_BLOCK_X / 2.0 - 18.0),
            -(DEGASSER_BLOCK_Y / 2.0 - 16.0),
        ),
        (
            DEGASSER_BLOCK_X / 2.0 - 18.0,
            -(DEGASSER_BLOCK_Y / 2.0 - 16.0),
        ),
        (
            -(DEGASSER_BLOCK_X / 2.0 - 18.0),
            DEGASSER_BLOCK_Y / 2.0 - 16.0,
        ),
        (DEGASSER_BLOCK_X / 2.0 - 18.0, DEGASSER_BLOCK_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("automated_seeding_degasser_m4_hole_{i}"),
                4.3 / 2.0,
                DEGASSER_BLOCK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn bubble_trap_towers() -> Part {
    let mut towers = Part::empty("automated_seeding_bubble_trap_towers");
    for (i, x) in [-6.0, 82.0, 170.0].iter().enumerate() {
        let chamber = centered_cylinder(
            format!("automated_seeding_bubble_trap_chamber_{i}"),
            16.0,
            62.0,
            44,
        )
        .translate(*x, 42.0, DEGASSER_BLOCK_Z / 2.0 + 31.0);
        let vent_core = centered_cylinder(
            format!("automated_seeding_bubble_trap_vent_core_{i}"),
            5.0,
            66.0,
            28,
        )
        .translate(*x, 42.0, DEGASSER_BLOCK_Z / 2.0 + 31.0);
        let optical_flat = centered_cube(
            format!("automated_seeding_bubble_trap_optical_flat_{i}"),
            12.0,
            35.0,
            42.0,
        )
        .translate(*x + 12.0, 42.0, DEGASSER_BLOCK_Z / 2.0 + 36.0);
        towers = towers + (chamber - vent_core - optical_flat);
    }
    towers
}

fn vacuum_port_placeholder() -> Part {
    let body = centered_cube(
        "automated_seeding_degasser_vacuum_port_block",
        62.0,
        36.0,
        26.0,
    )
    .translate(-168.0, 43.0, DEGASSER_BLOCK_Z / 2.0 + 13.0);
    let port = centered_cylinder(
        "automated_seeding_degasser_vacuum_port_clearance",
        8.0 / 2.0,
        66.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-168.0, 43.0, DEGASSER_BLOCK_Z / 2.0 + 13.0);
    body - port
}

fn degasser_connector_tabs() -> Part {
    let inlet =
        fluid_connector_tab("degasser_inlet").translate(-DEGASSER_BLOCK_X / 2.0 - 18.0, -42.0, 0.0);
    let outlet =
        fluid_connector_tab("degasser_outlet").translate(DEGASSER_BLOCK_X / 2.0 + 18.0, 42.0, 0.0);
    inlet + outlet
}

fn priming_waste_routing_tray() -> Part {
    let tray = centered_cube(
        "automated_seeding_priming_waste_tray_body",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let waste_bottle_socket = centered_cylinder(
        "automated_seeding_waste_bottle_socket",
        43.0,
        WASTE_TRAY_Z + 2.0,
        52,
    )
    .translate(112.0, -14.0, 0.0);
    let prime_trough = centered_cube(
        "automated_seeding_prime_waste_flush_trough",
        WASTE_TRAY_X - 82.0,
        26.0,
        12.0,
    )
    .translate(-24.0, 42.0, WASTE_TRAY_Z / 2.0 - 5.0);
    let overflow_sump = centered_cube("automated_seeding_waste_overflow_sump", 120.0, 32.0, 12.0)
        .translate(86.0, -52.0, WASTE_TRAY_Z / 2.0 - 5.0);

    let line_in = centered_cylinder(
        "automated_seeding_prime_waste_line_in",
        FLUID_BORE_D / 2.0,
        WASTE_TRAY_X - 66.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-12.0, 41.0, 0.0);
    let waste_drop = centered_cylinder(
        "automated_seeding_waste_bottle_drop_bore",
        9.0 / 2.0,
        56.0,
        28,
    )
    .translate(112.0, -14.0, 0.0);

    tray - waste_bottle_socket - prime_trough - overflow_sump - line_in - waste_drop
        + waste_luer_bulkhead()
        + waste_tray_tube_comb()
}

fn waste_luer_bulkhead() -> Part {
    let bulkhead = centered_cube(
        "automated_seeding_waste_luer_bulkhead_block",
        72.0,
        38.0,
        24.0,
    )
    .translate(-WASTE_TRAY_X / 2.0 + 44.0, 42.0, WASTE_TRAY_Z / 2.0 + 12.0);
    let tube = centered_cylinder(
        "automated_seeding_waste_luer_bulkhead_channel",
        8.0 / 2.0,
        76.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-WASTE_TRAY_X / 2.0 + 44.0, 42.0, WASTE_TRAY_Z / 2.0 + 12.0);
    let clamp_screw = centered_cylinder(
        "automated_seeding_waste_luer_clamp_screw",
        3.2 / 2.0,
        32.0,
        20,
    )
    .translate(-WASTE_TRAY_X / 2.0 + 44.0, 53.0, WASTE_TRAY_Z / 2.0 + 12.0);
    bulkhead - tube - clamp_screw
}

fn waste_tray_tube_comb() -> Part {
    let comb = centered_cube(
        "automated_seeding_waste_tube_strain_relief_comb",
        186.0,
        18.0,
        16.0,
    )
    .translate(-42.0, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 8.0);
    let mut cuts = Part::empty("automated_seeding_waste_tube_comb_cuts");
    for (i, x) in [-108.0, -72.0, -36.0, 0.0, 36.0, 72.0].iter().enumerate() {
        let channel = centered_cylinder(
            format!("automated_seeding_waste_tube_comb_channel_{i}"),
            FLUID_BORE_D / 2.0,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 8.0);
        let slot = centered_cube(
            format!("automated_seeding_waste_tube_comb_slot_{i}"),
            FLUID_BORE_D + 1.0,
            24.0,
            10.0,
        )
        .translate(*x, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 12.0);
        cuts = cuts + channel + slot;
    }
    comb - cuts
}

fn metering_pump_bank() -> Part {
    let body = centered_cube(
        "automated_seeding_metering_pump_bank_body",
        PUMP_BANK_LEN_X,
        PUMP_BANK_LEN_Y,
        PUMP_BANK_Z,
    );

    body - pump_lane_cuts() - pump_bank_mount_holes()
        + pump_lane_rotor_placeholders()
        + pump_bank_cover_lands()
        + pump_bank_connector_tabs()
}

fn pump_lane_cuts() -> Part {
    let mut cuts = Part::empty("automated_seeding_pump_lane_cuts");
    for lane in 0..METERED_PUMP_LANES {
        let y = pump_lane_y(lane);
        let tube = centered_cylinder(
            format!("automated_seeding_pump_lane_{lane}_tube_path"),
            ROW_TRUNK_D / 2.0,
            PUMP_BANK_LEN_X - 30.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 0.0);
        let cartridge = centered_cube(
            format!("automated_seeding_pump_lane_{lane}_cartridge_pocket"),
            246.0,
            17.0,
            18.0,
        )
        .translate(8.0, y, PUMP_BANK_Z / 2.0 - 8.0);
        let top_slot = centered_cube(
            format!("automated_seeding_pump_lane_{lane}_tube_top_slot"),
            PUMP_BANK_LEN_X - 34.0,
            ROW_TRUNK_D + 1.4,
            14.0,
        )
        .translate(0.0, y, 9.0);
        cuts = cuts + tube + cartridge + top_slot;
    }
    cuts
}

fn pump_lane_rotor_placeholders() -> Part {
    let mut rotors = Part::empty("automated_seeding_pump_rotor_placeholders");
    for lane in 0..METERED_PUMP_LANES {
        let y = pump_lane_y(lane);
        let rotor = centered_cylinder(
            format!("automated_seeding_pump_lane_{lane}_rotor_placeholder"),
            12.5,
            7.0,
            44,
        )
        .translate(-136.0, y, PUMP_BANK_Z / 2.0 + 3.5);
        let hub = centered_cylinder(
            format!("automated_seeding_pump_lane_{lane}_encoder_hub"),
            4.0,
            8.0,
            28,
        )
        .translate(-136.0, y, PUMP_BANK_Z / 2.0 + 4.0);
        let motor_keepout = centered_cube(
            format!("automated_seeding_pump_lane_{lane}_motor_keepout"),
            42.0,
            18.0,
            18.0,
        )
        .translate(-178.0, y, PUMP_BANK_Z / 2.0 + 9.0);
        rotors = rotors + rotor + hub + motor_keepout;
    }
    rotors
}

fn pump_bank_cover_lands() -> Part {
    let front_land = centered_cube(
        "automated_seeding_pump_bank_front_cover_land",
        PUMP_BANK_LEN_X - 28.0,
        10.0,
        6.0,
    )
    .translate(0.0, -PUMP_BANK_LEN_Y / 2.0 + 12.0, PUMP_BANK_Z / 2.0 + 3.0);
    let rear_land = centered_cube(
        "automated_seeding_pump_bank_rear_cover_land",
        PUMP_BANK_LEN_X - 28.0,
        10.0,
        6.0,
    )
    .translate(0.0, PUMP_BANK_LEN_Y / 2.0 - 12.0, PUMP_BANK_Z / 2.0 + 3.0);
    front_land + rear_land
}

fn pump_bank_connector_tabs() -> Part {
    let inlet =
        fluid_connector_tab("pump_bank_inlet").translate(-PUMP_BANK_LEN_X / 2.0 - 18.0, 0.0, 0.0);
    let outlet =
        fluid_connector_tab("pump_bank_outlet").translate(PUMP_BANK_LEN_X / 2.0 + 18.0, 0.0, 0.0);
    inlet + outlet
}

fn pump_bank_mount_holes() -> Part {
    let mut holes = Part::empty("automated_seeding_pump_bank_mount_holes");
    for (i, (x, y)) in [
        (
            -(PUMP_BANK_LEN_X / 2.0 - 18.0),
            -(PUMP_BANK_LEN_Y / 2.0 - 16.0),
        ),
        (
            PUMP_BANK_LEN_X / 2.0 - 18.0,
            -(PUMP_BANK_LEN_Y / 2.0 - 16.0),
        ),
        (
            -(PUMP_BANK_LEN_X / 2.0 - 18.0),
            PUMP_BANK_LEN_Y / 2.0 - 16.0,
        ),
        (PUMP_BANK_LEN_X / 2.0 - 18.0, PUMP_BANK_LEN_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("automated_seeding_pump_bank_m4_hole_{i}"),
                4.3 / 2.0,
                PUMP_BANK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn valve_manifold_region() -> Part {
    let body = centered_cube(
        "automated_seeding_valve_manifold_body",
        VALVE_BLOCK_X,
        VALVE_BLOCK_Y,
        VALVE_BLOCK_Z,
    );
    let top_label_land = centered_cube(
        "automated_seeding_valve_manifold_mode_label_land",
        VALVE_BLOCK_X - 32.0,
        6.0,
        5.0,
    )
    .translate(0.0, -VALVE_BLOCK_Y / 2.0 - 3.0, VALVE_BLOCK_Z / 2.0 - 5.0);

    body + top_label_land + valve_cap_bosses()
        - valve_manifold_channels()
        - valve_seat_cuts()
        - valve_block_mount_holes()
        + cassette_row_bulkhead_tabs()
}

fn valve_manifold_channels() -> Part {
    let mut channels = Part::empty("automated_seeding_valve_manifold_channels");
    for lane in 0..VALVE_FLUID_PATHS {
        let y = valve_path_y(lane);
        let bore = centered_cylinder(
            format!("automated_seeding_valve_path_{lane}_main_bore"),
            FLUID_BORE_D / 2.0,
            VALVE_BLOCK_X + 18.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 0.0);
        let top_slot = centered_cube(
            format!("automated_seeding_valve_path_{lane}_service_slot"),
            VALVE_BLOCK_X - 30.0,
            FLUID_BORE_D + 1.4,
            16.0,
        )
        .translate(0.0, y, 11.0);
        channels = channels + bore + top_slot;
    }

    for (i, x) in [-128.0, -32.0, 64.0, 148.0].iter().enumerate() {
        let cross = centered_cylinder(
            format!("automated_seeding_valve_cross_bore_{i}"),
            FLUID_BORE_D / 2.0,
            VALVE_BLOCK_Y - 28.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, 0.0);
        channels = channels + cross;
    }
    channels
}

fn valve_seat_cuts() -> Part {
    let mut cuts = Part::empty("automated_seeding_valve_seat_cuts");
    for lane in 0..VALVE_FLUID_PATHS {
        let y = valve_path_y(lane);
        for (i, x) in [-144.0, -48.0, 48.0, 144.0].iter().enumerate() {
            let stem = centered_cylinder(
                format!("automated_seeding_valve_path_{lane}_stem_bore_{i}"),
                4.0 / 2.0,
                VALVE_BLOCK_Z + 12.0,
                24,
            )
            .translate(*x, y, 6.0);
            let actuator_pocket = centered_cube(
                format!("automated_seeding_valve_path_{lane}_actuator_pocket_{i}"),
                24.0,
                24.0,
                13.0,
            )
            .translate(*x, y, VALVE_BLOCK_Z / 2.0 - 5.5);
            cuts = cuts + stem + actuator_pocket;
        }
    }
    cuts
}

fn valve_cap_bosses() -> Part {
    let mut bosses = Part::empty("automated_seeding_valve_cap_bosses");
    for lane in 0..VALVE_FLUID_PATHS {
        let y = valve_path_y(lane);
        for (i, x) in [-144.0, -48.0, 48.0, 144.0].iter().enumerate() {
            bosses = bosses
                + centered_cylinder(
                    format!("automated_seeding_valve_path_{lane}_cap_boss_{i}"),
                    11.0,
                    8.0,
                    32,
                )
                .translate(*x, y, VALVE_BLOCK_Z / 2.0 + 4.0);
        }
    }
    bosses
}

fn valve_block_mount_holes() -> Part {
    let mut holes = Part::empty("automated_seeding_valve_block_mount_holes");
    for (i, (x, y)) in [
        (-(VALVE_BLOCK_X / 2.0 - 20.0), -(VALVE_BLOCK_Y / 2.0 - 18.0)),
        (VALVE_BLOCK_X / 2.0 - 20.0, -(VALVE_BLOCK_Y / 2.0 - 18.0)),
        (-(VALVE_BLOCK_X / 2.0 - 20.0), VALVE_BLOCK_Y / 2.0 - 18.0),
        (VALVE_BLOCK_X / 2.0 - 20.0, VALVE_BLOCK_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("automated_seeding_valve_block_m5_hole_{i}"),
                5.3 / 2.0,
                VALVE_BLOCK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn cassette_row_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("automated_seeding_cassette_row_bulkhead_tabs");
    for row in 0..ROWS {
        let y = row_y(row) * 110.0 / ARRAY_Y;
        let tab = centered_cube(
            format!("automated_seeding_cassette_row_{row}_bulkhead_tab"),
            34.0,
            22.0,
            24.0,
        )
        .translate(VALVE_BLOCK_X / 2.0 + 17.0, y, 0.0);
        let tube = centered_cylinder(
            format!("automated_seeding_cassette_row_{row}_bulkhead_tube"),
            ROW_TRUNK_D / 2.0,
            38.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(VALVE_BLOCK_X / 2.0 + 17.0, y, 0.0);
        tabs = tabs + (tab - tube);
    }
    tabs
}

fn sensor_bridge() -> Part {
    let body = centered_cube(
        "automated_seeding_sensor_bridge_body",
        SENSOR_BRIDGE_LEN_X,
        SENSOR_BRIDGE_LEN_Y,
        SENSOR_BRIDGE_Z,
    );
    let main_bore = centered_cylinder(
        "automated_seeding_sensor_bridge_main_bore",
        FLUID_BORE_D / 2.0,
        SENSOR_BRIDGE_LEN_X + 16.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    let row_return_bore = centered_cylinder(
        "automated_seeding_sensor_bridge_waste_return_bore",
        FLUID_BORE_D / 2.0,
        SENSOR_BRIDGE_LEN_X - 68.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(20.0, -32.0, 0.0);
    let slide_clearance = centered_cube(
        "automated_seeding_sensor_bridge_flow_cartridge_slide_clearance",
        122.0,
        42.0,
        16.0,
    )
    .translate(-70.0, 30.0, SENSOR_BRIDGE_Z / 2.0 - 7.0);

    body + pressure_transducer_bosses()
        + flow_meter_cartridge()
        + bubble_sensor_fork().translate(88.0, 30.0, SENSOR_BRIDGE_Z / 2.0 + 20.0)
        - main_bore
        - row_return_bore
        - slide_clearance
        - sensor_bridge_mount_holes()
}

fn pressure_transducer_bosses() -> Part {
    let mut bosses = Part::empty("automated_seeding_pressure_transducer_bosses");
    for (i, x) in [-142.0, 4.0, 142.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("automated_seeding_pressure_transducer_boss_{i}"),
            14.0,
            8.0,
            40,
        )
        .translate(*x, -22.0, SENSOR_BRIDGE_Z / 2.0 + 4.0);
        let tap = centered_cylinder(
            format!("automated_seeding_pressure_transducer_tap_{i}"),
            2.4 / 2.0,
            SENSOR_BRIDGE_Z + 12.0,
            20,
        )
        .translate(*x, -22.0, 0.0);
        let o_ring = centered_cylinder(
            format!("automated_seeding_pressure_transducer_o_ring_{i}"),
            7.5,
            9.0,
            32,
        )
        .translate(*x, -22.0, SENSOR_BRIDGE_Z / 2.0 + 4.0);
        bosses = bosses + (boss - tap - o_ring);
    }
    bosses
}

fn flow_meter_cartridge() -> Part {
    let cartridge = centered_cube(
        "automated_seeding_flow_meter_cartridge_body",
        104.0,
        46.0,
        26.0,
    )
    .translate(-70.0, 30.0, SENSOR_BRIDGE_Z / 2.0 + 13.0);
    let channel = centered_cylinder(
        "automated_seeding_flow_meter_tube_channel",
        FLUID_BORE_D / 2.0,
        110.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-70.0, 30.0, SENSOR_BRIDGE_Z / 2.0 + 13.0);
    let view_window = centered_cube("automated_seeding_flow_meter_view_window", 46.0, 48.0, 12.0)
        .translate(-70.0, 30.0, SENSOR_BRIDGE_Z / 2.0 + 19.0);
    let latch_left = latch_ear("automated_seeding_flow_meter_latch_left").translate(
        -112.0,
        1.0,
        SENSOR_BRIDGE_Z / 2.0 + 11.0,
    );
    let latch_right = latch_ear("automated_seeding_flow_meter_latch_right").translate(
        -28.0,
        1.0,
        SENSOR_BRIDGE_Z / 2.0 + 11.0,
    );

    cartridge + latch_left + latch_right - channel - view_window
}

fn bubble_sensor_fork() -> Part {
    let base = centered_cube("automated_seeding_bubble_sensor_base", 58.0, 42.0, 10.0);
    let led_arm = centered_cube("automated_seeding_bubble_sensor_led_arm", 14.0, 12.0, 42.0)
        .translate(0.0, -17.0, 21.0);
    let detector_arm = centered_cube(
        "automated_seeding_bubble_sensor_detector_arm",
        14.0,
        12.0,
        42.0,
    )
    .translate(0.0, 17.0, 21.0);
    let tube_gap = centered_cylinder(
        "automated_seeding_bubble_sensor_tube_gap",
        7.2 / 2.0,
        64.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 18.0);
    let optical_path = centered_cylinder(
        "automated_seeding_bubble_sensor_optical_path",
        3.0 / 2.0,
        42.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 18.0);
    let wire_slots = centered_cube(
        "automated_seeding_bubble_sensor_led_wire_slot",
        8.0,
        16.0,
        8.0,
    )
    .translate(0.0, -21.0, 38.0)
        + centered_cube(
            "automated_seeding_bubble_sensor_detector_wire_slot",
            8.0,
            16.0,
            8.0,
        )
        .translate(0.0, 21.0, 38.0);

    base + led_arm + detector_arm - tube_gap - optical_path - wire_slots
}

fn sensor_bridge_mount_holes() -> Part {
    let mut holes = Part::empty("automated_seeding_sensor_bridge_mount_holes");
    for (i, (x, y)) in [
        (
            -(SENSOR_BRIDGE_LEN_X / 2.0 - 18.0),
            -(SENSOR_BRIDGE_LEN_Y / 2.0 - 16.0),
        ),
        (
            SENSOR_BRIDGE_LEN_X / 2.0 - 18.0,
            -(SENSOR_BRIDGE_LEN_Y / 2.0 - 16.0),
        ),
        (
            -(SENSOR_BRIDGE_LEN_X / 2.0 - 18.0),
            SENSOR_BRIDGE_LEN_Y / 2.0 - 16.0,
        ),
        (
            SENSOR_BRIDGE_LEN_X / 2.0 - 18.0,
            SENSOR_BRIDGE_LEN_Y / 2.0 - 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("automated_seeding_sensor_bridge_m4_hole_{i}"),
                4.3 / 2.0,
                SENSOR_BRIDGE_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn service_clearance_frame() -> Part {
    let rear_rail = centered_cube(
        "automated_seeding_service_clearance_rear_rail",
        SERVICE_CLEARANCE_X,
        16.0,
        18.0,
    )
    .translate(0.0, SERVICE_CLEARANCE_Y / 2.0, SERVICE_CLEARANCE_Z);
    let left_rail = centered_cube(
        "automated_seeding_service_clearance_left_rail",
        16.0,
        SERVICE_CLEARANCE_Y,
        18.0,
    )
    .translate(-SERVICE_CLEARANCE_X / 2.0, 0.0, SERVICE_CLEARANCE_Z);
    let right_rail = centered_cube(
        "automated_seeding_service_clearance_right_rail",
        16.0,
        SERVICE_CLEARANCE_Y,
        18.0,
    )
    .translate(SERVICE_CLEARANCE_X / 2.0, 0.0, SERVICE_CLEARANCE_Z);

    let mut posts = Part::empty("automated_seeding_service_clearance_posts");
    for (i, (x, y)) in [
        (-SERVICE_CLEARANCE_X / 2.0, -SERVICE_CLEARANCE_Y / 2.0),
        (SERVICE_CLEARANCE_X / 2.0, -SERVICE_CLEARANCE_Y / 2.0),
        (-SERVICE_CLEARANCE_X / 2.0, SERVICE_CLEARANCE_Y / 2.0),
        (SERVICE_CLEARANCE_X / 2.0, SERVICE_CLEARANCE_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("automated_seeding_service_clearance_post_{i}"),
            18.0,
            18.0,
            SERVICE_CLEARANCE_Z,
        )
        .translate(*x, *y, SERVICE_CLEARANCE_Z / 2.0);
        let foot = centered_cube(
            format!("automated_seeding_service_clearance_foot_{i}"),
            42.0,
            42.0,
            8.0,
        )
        .translate(*x, *y, 4.0);
        let screw = centered_cylinder(
            format!("automated_seeding_service_clearance_foot_screw_{i}"),
            5.3 / 2.0,
            10.0,
            24,
        )
        .translate(*x, *y, 4.0);
        posts = posts + post + (foot - screw);
    }

    let front_service_edge = centered_cube(
        "automated_seeding_front_service_approach_edge_marker",
        SERVICE_CLEARANCE_X,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        -SERVICE_CLEARANCE_Y / 2.0 - FRONT_SERVICE_CLEARANCE,
        4.0,
    );
    let cassette_lift_window = centered_cube(
        "automated_seeding_cassette_vertical_lift_window",
        CASSETTE_X + 50.0,
        CASSETTE_Y + 42.0,
        4.0,
    )
    .translate(0.0, 0.0, SERVICE_CLEARANCE_Z + 15.0);

    rear_rail + left_rail + right_rail + posts + front_service_edge + cassette_lift_window
}

fn fluid_connector_tab(name: &str) -> Part {
    let body = centered_cube(
        format!("automated_seeding_{name}_connector_tab"),
        36.0,
        34.0,
        22.0,
    );
    let tube = centered_cylinder(
        format!("automated_seeding_{name}_connector_tube_clearance"),
        FLUID_BORE_D / 2.0,
        40.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0);
    let screw = centered_cylinder(
        format!("automated_seeding_{name}_connector_screw"),
        3.4 / 2.0,
        24.0,
        20,
    )
    .translate(0.0, 10.0, 0.0);
    body - tube - screw
}

fn latch_ear(name: &str) -> Part {
    let ear = centered_cube(format!("{name}_ear"), 24.0, 20.0, 10.0);
    let screw = centered_cylinder(format!("{name}_m3_clearance"), 3.4 / 2.0, 12.0, 20);
    ear - screw
}

fn station_mount_points() -> [(f64, f64); 8] {
    [
        (-(STATION_X / 2.0 - 42.0), -(STATION_Y / 2.0 - 42.0)),
        (STATION_X / 2.0 - 42.0, -(STATION_Y / 2.0 - 42.0)),
        (-(STATION_X / 2.0 - 42.0), STATION_Y / 2.0 - 42.0),
        (STATION_X / 2.0 - 42.0, STATION_Y / 2.0 - 42.0),
        (0.0, -(STATION_Y / 2.0 - 42.0)),
        (0.0, STATION_Y / 2.0 - 42.0),
        (-(STATION_X / 2.0 - 42.0), 0.0),
        (STATION_X / 2.0 - 42.0, 0.0),
    ]
}

fn pump_lane_y(lane: usize) -> f64 {
    let lane_pitch = (PUMP_BANK_LEN_Y - 58.0) / (METERED_PUMP_LANES as f64 - 1.0);
    -(PUMP_BANK_LEN_Y - 58.0) / 2.0 + lane as f64 * lane_pitch
}

fn valve_path_y(lane: usize) -> f64 {
    let lane_pitch = 31.0;
    -((VALVE_FLUID_PATHS as f64 - 1.0) * lane_pitch) / 2.0 + lane as f64 * lane_pitch
}

fn row_y(row: usize) -> f64 {
    -((ROWS as f64 - 1.0) * PITCH_Y) / 2.0 + row as f64 * PITCH_Y
}

fn chip_x(col: usize) -> f64 {
    -((COLS as f64 - 1.0) * PITCH_X) / 2.0 + col as f64 * PITCH_X
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_deck_encloses_fluid_column_and_cassette_nest() {
        assert!(STATION_X > NEST_X + RESERVOIR_TRAY_X);
        assert!(STATION_Y > NEST_Y + FRONT_SERVICE_CLEARANCE);
        assert!(CASSETTE_ORIGIN_X + NEST_X / 2.0 < STATION_X / 2.0);
        assert!(FLUID_COLUMN_X - RESERVOIR_TRAY_X / 2.0 > -STATION_X / 2.0);
    }

    #[test]
    fn cassette_array_matches_scalable_twenty_chip_layout() {
        assert_eq!(COLS * ROWS, 20);
        assert_eq!(PITCH_X, REVC_CHIP_LENGTH + GUTTER);
        assert_eq!(PITCH_Y, REVC_CHIP_WIDTH + GUTTER);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);
    }

    #[test]
    fn pump_and_valve_counts_cover_seeding_coating_prime_and_waste() {
        assert_eq!(METERED_PUMP_LANES, ROWS + 2);
        assert_eq!(VALVE_FLUID_PATHS, 4);
        assert!(pump_lane_y(0) < pump_lane_y(METERED_PUMP_LANES - 1));
        assert!(valve_path_y(0) < valve_path_y(VALVE_FLUID_PATHS - 1));
    }

    #[test]
    fn service_clearance_preserves_cassette_access() {
        assert!(SERVICE_CLEARANCE_X > CASSETTE_X + 150.0);
        assert!(SERVICE_CLEARANCE_Y > CASSETTE_Y + 130.0);
        assert!(SERVICE_CLEARANCE_Z >= 150.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 90.0);
    }
}
