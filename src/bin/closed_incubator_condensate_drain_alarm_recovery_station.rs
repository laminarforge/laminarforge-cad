use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator condensate drain alarm recovery validation station.
//
// Design intent:
// - Package a mechanical surrogate of an incubator condensate drain trap, alarm
//   triggers, wetness sensors, recovery flush routing, custody lands, and
//   evidence capture datums on one contained validation deck.
// - Support clog, overflow, wetness, level-reference, alarm-token, and recovery
//   route challenge hardware without making any sterile-process, cleaning SOP,
//   or biological acceptance claim.
// - Model mechanical validation packaging, fixtures, datums, envelopes, and
//   keepout gauges only. Purchased sensors, procedures, custody records, and
//   acceptance criteria stay external to this CAD generator.

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_condensate_drain_alarm_recovery_station_deck_secondary_containment_moat.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_condensate_drain_trap_surrogate.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_clog_overflow_challenge_cartridges.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_wetness_sensor_docks.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_level_reference_wells.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_alarm_token_rail.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_recovery_flush_route.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_biofilm_coupon_retainers.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_barcode_coa_custody_lands.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_release_hold_reject_gates.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_camera_evidence_bridge.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_robot_service_keepouts.stl",
    "output/closed_incubator_condensate_drain_alarm_recovery_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_INTENT_FEATURES: [&str; 17] = [
    "condensate_drain_trap_surrogate",
    "clog_challenge_cartridges",
    "overflow_challenge_cartridges",
    "wetness_sensor_docks",
    "level_reference_wells",
    "alarm_token_rail",
    "recovery_flush_route",
    "biofilm_coupon_retainers",
    "secondary_containment_moat",
    "barcode_custody_lands",
    "coa_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const MECHANICAL_SCOPE_NOTE: &str =
    "mechanical validation packaging only; no sterile-process claim, cleaning SOP, or biological acceptance criterion";

const DECK_X: f64 = 1220.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 54.0;
const MOAT_W: f64 = 18.0;
const MOAT_DEPTH: f64 = 6.5;
const MOAT_INSET_X: f64 = 1040.0;
const MOAT_INSET_Y: f64 = 636.0;
const SUMP_X: f64 = 210.0;
const SUMP_Y: f64 = 76.0;
const SUMP_Z: f64 = 10.0;
const DATUM_BOSSES: usize = 8;

const TRAP_X: f64 = 330.0;
const TRAP_Y: f64 = 162.0;
const TRAP_Z: f64 = 58.0;
const TRAP_POS: (f64, f64) = (-382.0, 126.0);
const TRAP_CHANNEL_W: f64 = 30.0;
const TRAP_WINDOW_COUNT: usize = 4;
const TRAP_PORT_COUNT: usize = 4;
const TRAP_WATER_SEAL_DEPTH: f64 = 42.0;

const CARTRIDGE_BANK_X: f64 = 430.0;
const CARTRIDGE_BANK_Y: f64 = 168.0;
const CARTRIDGE_BANK_Z: f64 = 50.0;
const CARTRIDGE_BANK_POS: (f64, f64) = (-34.0, 158.0);
const CLOG_CARTRIDGE_COUNT: usize = 4;
const OVERFLOW_WEIR_COUNT: usize = 3;
const CARTRIDGE_SLOT_X: f64 = 74.0;
const CARTRIDGE_SLOT_Y: f64 = 118.0;
const CARTRIDGE_PITCH_X: f64 = 94.0;

const WETNESS_DOCK_BANK_X: f64 = 292.0;
const WETNESS_DOCK_BANK_Y: f64 = 184.0;
const WETNESS_DOCK_BANK_Z: f64 = 38.0;
const WETNESS_DOCK_BANK_POS: (f64, f64) = (352.0, 158.0);
const WETNESS_DOCKS: usize = 6;
const WETNESS_DOCK_PITCH_X: f64 = 70.0;
const WETNESS_DOCK_PITCH_Y: f64 = 58.0;
const WETNESS_SENSOR_WIDTH: f64 = 28.0;

const LEVEL_WELL_BLOCK_X: f64 = 240.0;
const LEVEL_WELL_BLOCK_Y: f64 = 230.0;
const LEVEL_WELL_BLOCK_Z: f64 = 44.0;
const LEVEL_WELL_BLOCK_POS: (f64, f64) = (462.0, -42.0);
const LEVEL_WELLS: usize = 5;
const LEVEL_WELL_D: f64 = 31.0;
const LEVEL_WELL_PITCH: f64 = 38.0;
const LEVEL_REFERENCE_STEPS: usize = 5;

const ALARM_RAIL_X: f64 = 390.0;
const ALARM_RAIL_Y: f64 = 74.0;
const ALARM_RAIL_Z: f64 = 34.0;
const ALARM_RAIL_POS: (f64, f64) = (-386.0, -252.0);
const ALARM_TOKEN_SLOTS: usize = 6;
const ALARM_TOKEN_PITCH: f64 = 58.0;

const FLUSH_ROUTE_X: f64 = 486.0;
const FLUSH_ROUTE_Y: f64 = 162.0;
const FLUSH_ROUTE_Z: f64 = 36.0;
const FLUSH_ROUTE_POS: (f64, f64) = (-42.0, -112.0);
const FLUSH_PORTS: usize = 4;
const FLUSH_ROUTE_SEGMENTS: usize = 5;
const FLUSH_CHANNEL_W: f64 = 18.0;

const COUPON_BANK_X: f64 = 336.0;
const COUPON_BANK_Y: f64 = 190.0;
const COUPON_BANK_Z: f64 = 34.0;
const COUPON_BANK_POS: (f64, f64) = (308.0, -240.0);
const BIOFILM_COUPON_RETAINERS: usize = 6;
const COUPON_SLOT_X: f64 = 82.0;
const COUPON_SLOT_Y: f64 = 24.0;

const CUSTODY_X: f64 = 396.0;
const CUSTODY_Y: f64 = 116.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (360.0, 300.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const CUSTODY_SEAL_TABS: usize = 4;

const GATE_X: f64 = 462.0;
const GATE_Y: f64 = 92.0;
const GATE_Z: f64 = 34.0;
const GATE_POS: (f64, f64) = (-38.0, -322.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 910.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 188.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (22.0, -18.0);
const CAMERA_MOUNTS: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 8;
const BRIDGE_UNDERSIDE_CLEARANCE: f64 = 126.0;

const ROBOT_KEEPOUT_X: f64 = 1120.0;
const ROBOT_KEEPOUT_Y: f64 = 90.0;
const ROBOT_KEEPOUT_Z: f64 = 72.0;
const SERVICE_KEEPOUT_X: f64 = 112.0;
const SERVICE_KEEPOUT_Y: f64 = 648.0;
const SERVICE_KEEPOUT_Z: f64 = 92.0;
const REAR_DRAIN_KEEP_OUT_Y: f64 = 72.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 300.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(station_deck_secondary_containment_moat(), OUTPUTS[0]);
    write_part(condensate_drain_trap_surrogate(), OUTPUTS[1]);
    write_part(clog_overflow_challenge_cartridges(), OUTPUTS[2]);
    write_part(wetness_sensor_docks(), OUTPUTS[3]);
    write_part(level_reference_wells(), OUTPUTS[4]);
    write_part(alarm_token_rail(), OUTPUTS[5]);
    write_part(recovery_flush_route(), OUTPUTS[6]);
    write_part(biofilm_coupon_retainers(), OUTPUTS[7]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed incubator condensate drain alarm recovery validation station: {:.0}mm x {:.0}mm contained deck, {:.0}mm trap surrogate, {} clog/overflow cartridges, {} wetness docks, and {} level wells.",
        DECK_X,
        DECK_Y,
        TRAP_X,
        CLOG_CARTRIDGE_COUNT,
        WETNESS_DOCKS,
        LEVEL_WELLS
    );
    println!(
        "Alarm/recovery features: {} token slots, {} flush ports across {} route segments, {} coupon retainers, {} barcode lands, {} COA lands, and {} disposition gates.",
        ALARM_TOKEN_SLOTS,
        FLUSH_PORTS,
        FLUSH_ROUTE_SEGMENTS,
        BIOFILM_COUPON_RETAINERS,
        BARCODE_LANDS,
        COA_LANDS,
        DISPOSITION_GATES
    );
    println!(
        "Scope: mechanical validation packaging only; no sterile-process claim, cleaning SOP, or biological acceptance criterion."
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_deck_secondary_containment_moat() -> Part {
    let deck = centered_cube(
        "incubator_condensate_alarm_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let shallow_pan = centered_cube(
        "incubator_condensate_alarm_station_shallow_pan_recess",
        DECK_X - 2.0 * RIM_W - 56.0,
        DECK_Y - 2.0 * RIM_W - 62.0,
        5.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain_sump = centered_cube(
        "incubator_condensate_alarm_station_sump_recess",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(
        -DECK_X / 2.0 + 164.0,
        -DECK_Y / 2.0 + 86.0,
        DECK_Z / 2.0 - 3.0,
    );
    let drain_port = centered_cylinder(
        "incubator_condensate_alarm_station_secondary_moat_drain_port",
        16.0 / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-DECK_X / 2.0 + 164.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - shallow_pan - secondary_containment_moat_cuts() - drain_sump - drain_port
        + containment_rim()
        + moat_guard_ribs()
        + deck_datums()
        + station_landing_recesses()
}

fn secondary_containment_moat_cuts() -> Part {
    let z = DECK_Z / 2.0 - MOAT_DEPTH / 2.0 + 0.4;
    let north = centered_cube(
        "incubator_condensate_secondary_moat_north_cut",
        MOAT_INSET_X,
        MOAT_W,
        MOAT_DEPTH,
    )
    .translate(0.0, MOAT_INSET_Y / 2.0, z);
    let south = centered_cube(
        "incubator_condensate_secondary_moat_south_cut",
        MOAT_INSET_X,
        MOAT_W,
        MOAT_DEPTH,
    )
    .translate(0.0, -MOAT_INSET_Y / 2.0, z);
    let west = centered_cube(
        "incubator_condensate_secondary_moat_west_cut",
        MOAT_W,
        MOAT_INSET_Y,
        MOAT_DEPTH,
    )
    .translate(-MOAT_INSET_X / 2.0, 0.0, z);
    let east = centered_cube(
        "incubator_condensate_secondary_moat_east_cut",
        MOAT_W,
        MOAT_INSET_Y,
        MOAT_DEPTH,
    )
    .translate(MOAT_INSET_X / 2.0, 0.0, z);
    let cross = centered_cube(
        "incubator_condensate_secondary_moat_cross_drain_cut",
        DECK_X - 2.0 * RIM_W - 140.0,
        MOAT_W * 0.75,
        MOAT_DEPTH,
    )
    .translate(38.0, -210.0, z);

    north + south + west + east + cross
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "incubator_condensate_alarm_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_z());
    let right = centered_cube(
        "incubator_condensate_alarm_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());
    let rear = centered_cube(
        "incubator_condensate_alarm_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let front = centered_cube(
        "incubator_condensate_alarm_station_front_low_containment_rim",
        DECK_X - 135.0,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn moat_guard_ribs() -> Part {
    let mut ribs = Part::empty("incubator_condensate_alarm_station_moat_guard_ribs");
    for (i, x) in [-426.0, -212.0, 0.0, 212.0, 426.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("incubator_condensate_moat_crossing_rib_{i}"),
                34.0,
                MOAT_W + 16.0,
                8.0,
            )
            .translate(*x, -MOAT_INSET_Y / 2.0, DECK_Z / 2.0 + 4.0);
    }
    ribs
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("incubator_condensate_alarm_station_deck_datums");
    for (i, (x, y)) in [
        (-552.0, -352.0),
        (552.0, -352.0),
        (-552.0, 352.0),
        (552.0, 352.0),
        (-250.0, -352.0),
        (250.0, -352.0),
        (-250.0, 352.0),
        (250.0, 352.0),
    ]
    .iter()
    .take(CUSTODY_SEAL_TABS)
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("incubator_condensate_alarm_datum_pad_{i}"),
            14.0,
            5.0,
            40,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("incubator_condensate_alarm_datum_bore_{i}"),
            3.4,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }
    datums
}

fn station_landing_recesses() -> Part {
    let trap = deck_recess("trap", TRAP_X + 32.0, TRAP_Y + 32.0, TRAP_POS.0, TRAP_POS.1);
    let cartridges = deck_recess(
        "clog_overflow_cartridge_bank",
        CARTRIDGE_BANK_X + 34.0,
        CARTRIDGE_BANK_Y + 30.0,
        CARTRIDGE_BANK_POS.0,
        CARTRIDGE_BANK_POS.1,
    );
    let wetness = deck_recess(
        "wetness_sensor_dock_bank",
        WETNESS_DOCK_BANK_X + 32.0,
        WETNESS_DOCK_BANK_Y + 28.0,
        WETNESS_DOCK_BANK_POS.0,
        WETNESS_DOCK_BANK_POS.1,
    );
    let level = deck_recess(
        "level_reference_well_block",
        LEVEL_WELL_BLOCK_X + 28.0,
        LEVEL_WELL_BLOCK_Y + 26.0,
        LEVEL_WELL_BLOCK_POS.0,
        LEVEL_WELL_BLOCK_POS.1,
    );
    let flush = deck_recess(
        "recovery_flush_route",
        FLUSH_ROUTE_X + 30.0,
        FLUSH_ROUTE_Y + 30.0,
        FLUSH_ROUTE_POS.0,
        FLUSH_ROUTE_POS.1,
    );
    let coupons = deck_recess(
        "biofilm_coupon_bank",
        COUPON_BANK_X + 30.0,
        COUPON_BANK_Y + 28.0,
        COUPON_BANK_POS.0,
        COUPON_BANK_POS.1,
    );

    trap + cartridges + wetness + level + flush + coupons
}

fn deck_recess(name: &str, x: f64, y: f64, cx: f64, cy: f64) -> Part {
    centered_cube(
        format!("incubator_condensate_alarm_station_{name}_landing_recess"),
        x,
        y,
        4.5,
    )
    .translate(cx, cy, DECK_Z / 2.0 - 1.8)
}

fn condensate_drain_trap_surrogate() -> Part {
    let body = centered_cube(
        "incubator_condensate_drain_trap_surrogate_body",
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    );
    let channel = trap_u_channel();
    let observation_windows = trap_observation_windows();
    let cover_recess = centered_cube(
        "incubator_condensate_drain_trap_surrogate_clear_cover_recess",
        TRAP_X - 46.0,
        TRAP_Y - 38.0,
        9.0,
    )
    .translate(0.0, 0.0, TRAP_Z / 2.0 - 4.0);
    let water_seal_gauge = centered_cube(
        "incubator_condensate_drain_trap_water_seal_depth_gauge",
        18.0,
        TRAP_Y - 44.0,
        7.0,
    )
    .translate(36.0, 0.0, TRAP_Z / 2.0 + 3.5);
    let flow_arrow = centered_cube(
        "incubator_condensate_drain_trap_flow_direction_arrow_land",
        96.0,
        10.0,
        5.0,
    )
    .translate(-88.0, -TRAP_Y / 2.0 - 6.0, 8.0);

    body - channel - observation_windows - cover_recess
        + trap_port_collars()
        + trap_strap_bridges()
        + water_seal_gauge
        + flow_arrow
}

fn trap_u_channel() -> Part {
    let inlet = centered_cube(
        "incubator_condensate_drain_trap_inlet_leg_cut",
        118.0,
        TRAP_CHANNEL_W,
        TRAP_Z + 4.0,
    )
    .translate(-88.0, 38.0, 8.0);
    let downcomer = centered_cube(
        "incubator_condensate_drain_trap_downcomer_cut",
        TRAP_CHANNEL_W,
        96.0,
        TRAP_Z + 4.0,
    )
    .translate(-34.0, -10.0, 8.0);
    let return_leg = centered_cube(
        "incubator_condensate_drain_trap_return_leg_cut",
        158.0,
        TRAP_CHANNEL_W,
        TRAP_Z + 4.0,
    )
    .translate(40.0, -58.0, 8.0);
    let outlet = centered_cube(
        "incubator_condensate_drain_trap_outlet_leg_cut",
        TRAP_CHANNEL_W,
        92.0,
        TRAP_Z + 4.0,
    )
    .translate(116.0, -12.0, 8.0);
    let water_seal = centered_cube(
        "incubator_condensate_drain_trap_surrogate_water_seal_cut",
        98.0,
        TRAP_CHANNEL_W + 18.0,
        TRAP_WATER_SEAL_DEPTH,
    )
    .translate(0.0, -58.0, -2.0);

    inlet + downcomer + return_leg + outlet + water_seal
}

fn trap_observation_windows() -> Part {
    let mut windows = Part::empty("incubator_condensate_trap_observation_windows");
    for i in 0..TRAP_WINDOW_COUNT {
        let x = -112.0 + i as f64 * 76.0;
        windows = windows
            + centered_cube(
                format!("incubator_condensate_trap_observation_window_{i}"),
                32.0,
                10.0,
                22.0,
            )
            .translate(x, -TRAP_Y / 2.0 - 1.0, 10.0);
    }
    windows
}

fn trap_port_collars() -> Part {
    let mut collars = Part::empty("incubator_condensate_trap_port_collars");
    for (i, (x, y, rot_z)) in [
        (-154.0, 38.0, 90.0),
        (154.0, 34.0, 90.0),
        (118.0, -78.0, 0.0),
        (-118.0, -78.0, 0.0),
    ]
    .iter()
    .take(TRAP_PORT_COUNT)
    .enumerate()
    {
        let collar = centered_cylinder(
            format!("incubator_condensate_trap_port_collar_{i}"),
            19.0,
            18.0,
            36,
        )
        .rotate(90.0, 0.0, *rot_z)
        .translate(*x, *y, 2.0);
        let bore = centered_cylinder(
            format!("incubator_condensate_trap_port_bore_{i}"),
            7.2,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, *rot_z)
        .translate(*x, *y, 2.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn trap_strap_bridges() -> Part {
    let mut straps = Part::empty("incubator_condensate_trap_cover_strap_bridges");
    for (i, x) in [-116.0, 0.0, 116.0].iter().enumerate() {
        let bridge = centered_cube(
            format!("incubator_condensate_trap_cover_strap_bridge_{i}"),
            32.0,
            TRAP_Y + 24.0,
            11.0,
        )
        .translate(*x, 0.0, TRAP_Z / 2.0 + 5.5);
        let fastener = centered_cylinder(
            format!("incubator_condensate_trap_cover_strap_fastener_{i}"),
            3.2,
            16.0,
            22,
        )
        .translate(*x, 0.0, TRAP_Z / 2.0 + 5.5);
        straps = straps + (bridge - fastener);
    }
    straps
}

fn clog_overflow_challenge_cartridges() -> Part {
    let tray = centered_cube(
        "incubator_condensate_clog_overflow_challenge_cartridge_tray",
        CARTRIDGE_BANK_X,
        CARTRIDGE_BANK_Y,
        CARTRIDGE_BANK_Z,
    );
    let pockets = cartridge_pocket_cuts();
    let overflow_gutter = centered_cube(
        "incubator_condensate_overflow_challenge_common_gutter",
        CARTRIDGE_BANK_X - 48.0,
        18.0,
        20.0,
    )
    .translate(0.0, -CARTRIDGE_BANK_Y / 2.0 + 28.0, 8.0);
    let drain_notch = centered_cube(
        "incubator_condensate_overflow_gutter_drain_notch",
        38.0,
        24.0,
        22.0,
    )
    .translate(
        CARTRIDGE_BANK_X / 2.0 - 36.0,
        -CARTRIDGE_BANK_Y / 2.0 + 18.0,
        8.0,
    );

    tray - pockets - overflow_gutter - drain_notch
        + challenge_cartridge_inserts()
        + overflow_weir_cards()
        + cartridge_keying_tabs()
}

fn cartridge_pocket_cuts() -> Part {
    let mut cuts = Part::empty("incubator_condensate_challenge_cartridge_pocket_cuts");
    for i in 0..CLOG_CARTRIDGE_COUNT {
        let x = cartridge_center_x(i);
        cuts = cuts
            + centered_cube(
                format!("incubator_condensate_challenge_cartridge_pocket_{i}"),
                CARTRIDGE_SLOT_X + 6.0,
                CARTRIDGE_SLOT_Y + 8.0,
                CARTRIDGE_BANK_Z,
            )
            .translate(x, 8.0, 8.0);
    }
    cuts
}

fn challenge_cartridge_inserts() -> Part {
    let mut inserts = Part::empty("incubator_condensate_challenge_cartridge_inserts");
    for i in 0..CLOG_CARTRIDGE_COUNT {
        let x = cartridge_center_x(i);
        let cartridge = centered_cube(
            format!("incubator_condensate_clog_challenge_cartridge_body_{i}"),
            CARTRIDGE_SLOT_X,
            CARTRIDGE_SLOT_Y,
            26.0,
        )
        .translate(x, 8.0, CARTRIDGE_BANK_Z / 2.0 + 13.0);
        let witness_slot = centered_cube(
            format!("incubator_condensate_clog_challenge_cartridge_witness_slot_{i}"),
            CARTRIDGE_SLOT_X - 18.0,
            14.0,
            28.0,
        )
        .translate(x, 8.0, CARTRIDGE_BANK_Z / 2.0 + 13.0);
        let restriction = centered_cube(
            format!("incubator_condensate_clog_challenge_restriction_land_{i}"),
            14.0 + i as f64 * 5.0,
            CARTRIDGE_SLOT_Y + 14.0,
            6.0,
        )
        .translate(x, 8.0, CARTRIDGE_BANK_Z / 2.0 + 29.0);
        inserts = inserts + (cartridge - witness_slot) + restriction;
    }
    inserts
}

fn overflow_weir_cards() -> Part {
    let mut weirs = Part::empty("incubator_condensate_overflow_challenge_weir_cards");
    for i in 0..OVERFLOW_WEIR_COUNT {
        let x = -CARTRIDGE_BANK_X / 2.0 + 108.0 + i as f64 * 124.0;
        weirs = weirs
            + centered_cube(
                format!("incubator_condensate_overflow_weir_card_{i}"),
                54.0,
                10.0,
                32.0 + i as f64 * 6.0,
            )
            .translate(
                x,
                -CARTRIDGE_BANK_Y / 2.0 + 48.0,
                CARTRIDGE_BANK_Z / 2.0 + 16.0,
            );
    }
    weirs
}

fn cartridge_keying_tabs() -> Part {
    let mut tabs = Part::empty("incubator_condensate_cartridge_keying_tabs");
    for i in 0..CLOG_CARTRIDGE_COUNT {
        tabs = tabs
            + centered_cube(
                format!("incubator_condensate_cartridge_asymmetric_key_tab_{i}"),
                20.0,
                14.0,
                10.0,
            )
            .translate(
                cartridge_center_x(i) - 18.0,
                CARTRIDGE_BANK_Y / 2.0 - 12.0,
                CARTRIDGE_BANK_Z / 2.0 + 5.0,
            );
    }
    tabs
}

fn wetness_sensor_docks() -> Part {
    let bank = centered_cube(
        "incubator_condensate_wetness_sensor_dock_bank",
        WETNESS_DOCK_BANK_X,
        WETNESS_DOCK_BANK_Y,
        WETNESS_DOCK_BANK_Z,
    );
    let dock_cuts = wetness_dock_cuts();
    let drip_trough = centered_cube(
        "incubator_condensate_wetness_sensor_common_drip_trough",
        WETNESS_DOCK_BANK_X - 38.0,
        18.0,
        18.0,
    )
    .translate(0.0, -WETNESS_DOCK_BANK_Y / 2.0 + 24.0, 10.0);

    bank - dock_cuts - drip_trough + wetness_sensor_clips() + wetness_cable_comb()
}

fn wetness_dock_cuts() -> Part {
    let mut cuts = Part::empty("incubator_condensate_wetness_sensor_dock_cuts");
    for i in 0..WETNESS_DOCKS {
        let (x, y) = wetness_dock_center(i);
        cuts = cuts
            + centered_cube(
                format!("incubator_condensate_wetness_sensor_dock_recess_{i}"),
                WETNESS_SENSOR_WIDTH + 8.0,
                36.0,
                WETNESS_DOCK_BANK_Z,
            )
            .translate(x, y, 9.0)
            + centered_cube(
                format!("incubator_condensate_wetness_sensor_lead_slot_{i}"),
                9.0,
                74.0,
                18.0,
            )
            .translate(x, y + 24.0, 10.0);
    }
    cuts
}

fn wetness_sensor_clips() -> Part {
    let mut clips = Part::empty("incubator_condensate_wetness_sensor_dock_clips");
    for i in 0..WETNESS_DOCKS {
        let (x, y) = wetness_dock_center(i);
        clips = clips
            + centered_cube(
                format!("incubator_condensate_wetness_sensor_left_clip_{i}"),
                6.0,
                42.0,
                22.0,
            )
            .translate(x - 22.0, y, WETNESS_DOCK_BANK_Z / 2.0 + 11.0)
            + centered_cube(
                format!("incubator_condensate_wetness_sensor_right_clip_{i}"),
                6.0,
                42.0,
                22.0,
            )
            .translate(x + 22.0, y, WETNESS_DOCK_BANK_Z / 2.0 + 11.0);
    }
    clips
}

fn wetness_cable_comb() -> Part {
    let comb = centered_cube(
        "incubator_condensate_wetness_sensor_cable_comb",
        WETNESS_DOCK_BANK_X - 44.0,
        22.0,
        18.0,
    )
    .translate(0.0, WETNESS_DOCK_BANK_Y / 2.0 + 8.0, 14.0);
    let mut slots = Part::empty("incubator_condensate_wetness_sensor_cable_comb_slots");
    for i in 0..WETNESS_DOCKS {
        let (x, _) = wetness_dock_center(i);
        slots = slots
            + centered_cube(
                format!("incubator_condensate_wetness_sensor_cable_slot_{i}"),
                10.0,
                28.0,
                20.0,
            )
            .translate(x, WETNESS_DOCK_BANK_Y / 2.0 + 8.0, 14.0);
    }
    comb - slots
}

fn level_reference_wells() -> Part {
    let block = centered_cube(
        "incubator_condensate_level_reference_well_block",
        LEVEL_WELL_BLOCK_X,
        LEVEL_WELL_BLOCK_Y,
        LEVEL_WELL_BLOCK_Z,
    );
    let wells = level_well_cuts();
    let drain_reference_slot = centered_cube(
        "incubator_condensate_level_reference_drain_alignment_slot",
        28.0,
        LEVEL_WELL_BLOCK_Y - 44.0,
        18.0,
    )
    .translate(LEVEL_WELL_BLOCK_X / 2.0 - 38.0, 0.0, 10.0);

    block - wells - drain_reference_slot + level_step_gauges() + level_label_ticks()
}

fn level_well_cuts() -> Part {
    let mut wells = Part::empty("incubator_condensate_level_reference_well_cuts");
    for i in 0..LEVEL_WELLS {
        wells = wells
            + centered_cylinder(
                format!("incubator_condensate_level_reference_well_{i}"),
                LEVEL_WELL_D / 2.0,
                LEVEL_WELL_BLOCK_Z + 4.0,
                40,
            )
            .translate(
                -((LEVEL_WELLS as f64 - 1.0) * LEVEL_WELL_PITCH) / 2.0
                    + i as f64 * LEVEL_WELL_PITCH,
                -48.0,
                8.0,
            );
    }
    wells
}

fn level_step_gauges() -> Part {
    let mut steps = Part::empty("incubator_condensate_level_reference_step_gauges");
    for i in 0..LEVEL_REFERENCE_STEPS {
        steps = steps
            + centered_cube(
                format!("incubator_condensate_level_reference_step_{i}"),
                28.0,
                22.0,
                8.0 + i as f64 * 5.0,
            )
            .translate(
                -82.0 + i as f64 * 41.0,
                54.0,
                LEVEL_WELL_BLOCK_Z / 2.0 + 4.0 + i as f64 * 2.5,
            );
    }
    steps
}

fn level_label_ticks() -> Part {
    let mut ticks = Part::empty("incubator_condensate_level_reference_label_ticks");
    for i in 0..=LEVEL_REFERENCE_STEPS {
        ticks = ticks
            + centered_cube(
                format!("incubator_condensate_level_reference_tick_{i}"),
                5.0,
                68.0,
                4.0,
            )
            .translate(
                -102.0 + i as f64 * 41.0,
                94.0,
                LEVEL_WELL_BLOCK_Z / 2.0 + 2.0,
            );
    }
    ticks
}

fn alarm_token_rail() -> Part {
    let rail = centered_cube(
        "incubator_condensate_alarm_token_rail_body",
        ALARM_RAIL_X,
        ALARM_RAIL_Y,
        ALARM_RAIL_Z,
    );
    let slot_cuts = alarm_token_slot_cuts();
    let back_stop = centered_cube(
        "incubator_condensate_alarm_token_rail_back_stop",
        ALARM_RAIL_X,
        12.0,
        48.0,
    )
    .translate(0.0, ALARM_RAIL_Y / 2.0 + 6.0, 7.0);
    let detent_bar = centered_cube(
        "incubator_condensate_alarm_token_rail_detent_bar",
        ALARM_RAIL_X - 40.0,
        10.0,
        12.0,
    )
    .translate(0.0, -ALARM_RAIL_Y / 2.0 - 5.0, 13.0);

    rail - slot_cuts + back_stop + detent_bar + alarm_token_marker_tabs()
}

fn alarm_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("incubator_condensate_alarm_token_slot_cuts");
    for i in 0..ALARM_TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("incubator_condensate_alarm_token_slot_{i}"),
                38.0,
                46.0,
                ALARM_RAIL_Z,
            )
            .translate(alarm_slot_x(i), 0.0, 8.0);
    }
    cuts
}

fn alarm_token_marker_tabs() -> Part {
    let mut tabs = Part::empty("incubator_condensate_alarm_token_marker_tabs");
    for i in 0..ALARM_TOKEN_SLOTS {
        tabs = tabs
            + centered_cube(
                format!("incubator_condensate_alarm_token_marker_tab_{i}"),
                28.0,
                10.0,
                6.0,
            )
            .translate(alarm_slot_x(i), -ALARM_RAIL_Y / 2.0 - 12.0, 17.0);
    }
    tabs
}

fn recovery_flush_route() -> Part {
    let base = centered_cube(
        "incubator_condensate_recovery_flush_route_base",
        FLUSH_ROUTE_X,
        FLUSH_ROUTE_Y,
        FLUSH_ROUTE_Z,
    );
    let channels = recovery_flush_channel_cuts();
    let waste_cup = centered_cylinder(
        "incubator_condensate_recovery_flush_waste_capture_cup",
        34.0,
        32.0,
        48,
    )
    .translate(
        FLUSH_ROUTE_X / 2.0 - 54.0,
        -38.0,
        FLUSH_ROUTE_Z / 2.0 + 16.0,
    );
    let waste_cup_cut = centered_cylinder(
        "incubator_condensate_recovery_flush_waste_capture_cavity",
        25.0,
        34.0,
        48,
    )
    .translate(
        FLUSH_ROUTE_X / 2.0 - 54.0,
        -38.0,
        FLUSH_ROUTE_Z / 2.0 + 18.0,
    );

    base - channels + flush_port_collars() + (waste_cup - waste_cup_cut) + flush_route_arrows()
}

fn recovery_flush_channel_cuts() -> Part {
    let trunk = centered_cube(
        "incubator_condensate_recovery_flush_route_trunk_cut",
        FLUSH_ROUTE_X - 78.0,
        FLUSH_CHANNEL_W,
        FLUSH_ROUTE_Z + 4.0,
    )
    .translate(4.0, -38.0, 8.0);
    let trap_leg = centered_cube(
        "incubator_condensate_recovery_flush_route_to_trap_cut",
        FLUSH_CHANNEL_W,
        118.0,
        FLUSH_ROUTE_Z + 4.0,
    )
    .translate(-FLUSH_ROUTE_X / 2.0 + 72.0, 14.0, 8.0);
    let challenge_leg = centered_cube(
        "incubator_condensate_recovery_flush_route_to_challenge_cut",
        FLUSH_CHANNEL_W,
        106.0,
        FLUSH_ROUTE_Z + 4.0,
    )
    .translate(-86.0, 10.0, 8.0);
    let sensor_leg = centered_cube(
        "incubator_condensate_recovery_flush_route_to_sensor_cut",
        FLUSH_CHANNEL_W,
        98.0,
        FLUSH_ROUTE_Z + 4.0,
    )
    .translate(76.0, 8.0, 8.0);
    let waste_leg = centered_cube(
        "incubator_condensate_recovery_flush_route_to_waste_cut",
        92.0,
        FLUSH_CHANNEL_W,
        FLUSH_ROUTE_Z + 4.0,
    )
    .translate(FLUSH_ROUTE_X / 2.0 - 86.0, -38.0, 8.0);

    trunk + trap_leg + challenge_leg + sensor_leg + waste_leg
}

fn flush_port_collars() -> Part {
    let mut collars = Part::empty("incubator_condensate_recovery_flush_port_collars");
    for (i, x) in [-174.0, -58.0, 58.0, 174.0].iter().enumerate() {
        let collar = centered_cylinder(
            format!("incubator_condensate_recovery_flush_port_collar_{i}"),
            17.0,
            18.0,
            36,
        )
        .translate(*x, FLUSH_ROUTE_Y / 2.0 - 28.0, FLUSH_ROUTE_Z / 2.0 + 9.0);
        let bore = centered_cylinder(
            format!("incubator_condensate_recovery_flush_port_bore_{i}"),
            6.5,
            22.0,
            24,
        )
        .translate(*x, FLUSH_ROUTE_Y / 2.0 - 28.0, FLUSH_ROUTE_Z / 2.0 + 9.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn flush_route_arrows() -> Part {
    let mut arrows = Part::empty("incubator_condensate_recovery_flush_route_arrow_lands");
    for (i, x) in [-160.0, -58.0, 48.0, 152.0].iter().enumerate() {
        arrows = arrows
            + centered_cube(
                format!("incubator_condensate_recovery_flush_direction_land_{i}"),
                42.0,
                8.0,
                5.0,
            )
            .translate(*x, -70.0, FLUSH_ROUTE_Z / 2.0 + 2.5);
    }
    arrows
}

fn biofilm_coupon_retainers() -> Part {
    let tray = centered_cube(
        "incubator_condensate_biofilm_coupon_retainer_tray",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    );
    let slots = biofilm_coupon_slot_cuts();
    let drip_gutter = centered_cube(
        "incubator_condensate_biofilm_coupon_retainer_drip_gutter",
        COUPON_BANK_X - 38.0,
        20.0,
        18.0,
    )
    .translate(0.0, -COUPON_BANK_Y / 2.0 + 26.0, 8.0);

    tray - slots - drip_gutter + biofilm_coupon_retainers_clips() + biofilm_coupon_custody_tabs()
}

fn biofilm_coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("incubator_condensate_biofilm_coupon_slot_cuts");
    for i in 0..BIOFILM_COUPON_RETAINERS {
        let row = i / 3;
        let col = i % 3;
        let x = -88.0 + col as f64 * 88.0;
        let y = -36.0 + row as f64 * 72.0;
        slots = slots
            + centered_cube(
                format!("incubator_condensate_biofilm_coupon_retainer_slot_{i}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_BANK_Z + 4.0,
            )
            .translate(x, y, 8.0);
    }
    slots
}

fn biofilm_coupon_retainers_clips() -> Part {
    let mut clips = Part::empty("incubator_condensate_biofilm_coupon_retainer_clips");
    for i in 0..BIOFILM_COUPON_RETAINERS {
        let row = i / 3;
        let col = i % 3;
        let x = -88.0 + col as f64 * 88.0;
        let y = -36.0 + row as f64 * 72.0;
        clips = clips
            + centered_cube(
                format!("incubator_condensate_biofilm_coupon_left_retainer_clip_{i}"),
                9.0,
                COUPON_SLOT_Y + 18.0,
                20.0,
            )
            .translate(x - COUPON_SLOT_X / 2.0 - 7.0, y, COUPON_BANK_Z / 2.0 + 10.0)
            + centered_cube(
                format!("incubator_condensate_biofilm_coupon_right_retainer_clip_{i}"),
                9.0,
                COUPON_SLOT_Y + 18.0,
                20.0,
            )
            .translate(x + COUPON_SLOT_X / 2.0 + 7.0, y, COUPON_BANK_Z / 2.0 + 10.0);
    }
    clips
}

fn biofilm_coupon_custody_tabs() -> Part {
    let mut tabs = Part::empty("incubator_condensate_biofilm_coupon_custody_tabs");
    for i in 0..BIOFILM_COUPON_RETAINERS {
        let col = i % 3;
        let row = i / 3;
        tabs = tabs
            + centered_cube(
                format!("incubator_condensate_biofilm_coupon_custody_tab_{i}"),
                42.0,
                10.0,
                5.0,
            )
            .translate(
                -88.0 + col as f64 * 88.0,
                -70.0 + row as f64 * 140.0,
                COUPON_BANK_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "incubator_condensate_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let barcode_lands = barcode_land_array();
    let coa_lands = coa_land_array();
    let seal_tabs = custody_seal_tabs();
    let document_lip = centered_cube(
        "incubator_condensate_coa_document_lip",
        CUSTODY_X - 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 + 5.0, 6.0);

    panel + barcode_lands + coa_lands + seal_tabs + document_lip
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty("incubator_condensate_barcode_custody_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 3;
        let col = i % 3;
        lands = lands
            + centered_cube(
                format!("incubator_condensate_barcode_custody_land_{i}"),
                92.0,
                24.0,
                5.0,
            )
            .translate(-116.0 + col as f64 * 116.0, -28.0 + row as f64 * 38.0, 10.5);
    }
    lands
}

fn coa_land_array() -> Part {
    let mut lands = Part::empty("incubator_condensate_coa_custody_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("incubator_condensate_coa_custody_land_{i}"),
                92.0,
                28.0,
                5.0,
            )
            .translate(-116.0 + i as f64 * 116.0, 40.0, 10.5);
    }
    lands
}

fn custody_seal_tabs() -> Part {
    let mut tabs = Part::empty("incubator_condensate_custody_tamper_seal_tabs");
    for (i, (x, y)) in [
        (-CUSTODY_X / 2.0 + 26.0, -CUSTODY_Y / 2.0 + 20.0),
        (CUSTODY_X / 2.0 - 26.0, -CUSTODY_Y / 2.0 + 20.0),
        (-CUSTODY_X / 2.0 + 26.0, CUSTODY_Y / 2.0 - 20.0),
        (CUSTODY_X / 2.0 - 26.0, CUSTODY_Y / 2.0 - 20.0),
    ]
    .iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cylinder(
                format!("incubator_condensate_custody_tamper_seal_tab_{i}"),
                12.0,
                5.0,
                28,
            )
            .translate(*x, *y, 10.5);
    }
    tabs
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "incubator_condensate_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let gate_pockets = gate_pocket_cuts();
    let dividers = gate_dividers();
    let front_label = centered_cube(
        "incubator_condensate_release_hold_reject_front_label_land",
        GATE_X - 42.0,
        8.0,
        7.0,
    )
    .translate(0.0, -GATE_Y / 2.0 - 4.0, 12.0);

    base - gate_pockets + dividers + front_label + gate_token_retention_slots()
}

fn gate_pocket_cuts() -> Part {
    let mut cuts = Part::empty("incubator_condensate_release_hold_reject_gate_pockets");
    for i in 0..DISPOSITION_GATES {
        cuts = cuts
            + centered_cube(
                format!("incubator_condensate_disposition_gate_pocket_{i}"),
                118.0,
                GATE_Y - 30.0,
                GATE_Z,
            )
            .translate(gate_center_x(i), 0.0, 8.0);
    }
    cuts
}

fn gate_dividers() -> Part {
    let mut dividers = Part::empty("incubator_condensate_release_hold_reject_gate_dividers");
    for i in 0..=DISPOSITION_GATES {
        dividers = dividers
            + centered_cube(
                format!("incubator_condensate_disposition_gate_divider_{i}"),
                10.0,
                GATE_Y,
                GATE_Z + 16.0,
            )
            .translate(-GATE_X / 2.0 + 62.0 + i as f64 * 112.0, 0.0, 8.0);
    }
    dividers
}

fn gate_token_retention_slots() -> Part {
    let mut slots = Part::empty("incubator_condensate_disposition_gate_token_retention_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("incubator_condensate_disposition_token_retainer_{i}"),
                34.0,
                10.0,
                8.0,
            )
            .translate(-144.0 + i as f64 * 58.0, GATE_Y / 2.0 + 8.0, 14.0);
    }
    slots
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "incubator_condensate_camera_evidence_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "incubator_condensate_camera_evidence_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "incubator_condensate_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X + 34.0,
        CAMERA_BRIDGE_Y,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);
    let underside_gauge = centered_cube(
        "incubator_condensate_camera_evidence_bridge_underside_clearance_gauge",
        CAMERA_BRIDGE_X - 86.0,
        10.0,
        8.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 - 7.0,
        BRIDGE_UNDERSIDE_CLEARANCE,
    );

    left_post + right_post + beam + underside_gauge + camera_mounts() + evidence_fiducials()
        - bridge_lightening_slots()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("incubator_condensate_camera_evidence_mounts");
    for (i, x) in [-330.0, -110.0, 110.0, 330.0].iter().enumerate() {
        if i >= CAMERA_MOUNTS {
            break;
        }
        let plate = centered_cube(
            format!("incubator_condensate_camera_mount_plate_{i}"),
            86.0,
            18.0,
            42.0,
        )
        .translate(*x, -CAMERA_BRIDGE_Y / 2.0 - 9.0, CAMERA_BRIDGE_Z - 54.0);
        let bore = centered_cylinder(
            format!("incubator_condensate_camera_mount_bore_{i}"),
            5.0,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -CAMERA_BRIDGE_Y / 2.0 - 9.0, CAMERA_BRIDGE_Z - 54.0);
        mounts = mounts + (plate - bore);
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("incubator_condensate_evidence_bridge_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -350.0 + (i % 4) as f64 * 232.0;
        let y = if i < 4 { -62.0 } else { 62.0 };
        let disk = centered_cylinder(
            format!("incubator_condensate_evidence_fiducial_outer_{i}"),
            12.0,
            4.0,
            28,
        )
        .translate(x, y, 4.0);
        let center = centered_cylinder(
            format!("incubator_condensate_evidence_fiducial_center_{i}"),
            3.0,
            5.0,
            18,
        )
        .translate(x, y, 4.0);
        fiducials = fiducials + (disk - center);
    }
    fiducials
}

fn bridge_lightening_slots() -> Part {
    let mut slots = Part::empty("incubator_condensate_camera_bridge_lightening_slots");
    for (i, x) in [-290.0, -96.0, 96.0, 290.0].iter().enumerate() {
        slots = slots
            + centered_cube(
                format!("incubator_condensate_camera_bridge_lightening_slot_{i}"),
                108.0,
                CAMERA_BRIDGE_Y + 10.0,
                32.0,
            )
            .translate(*x, 0.0, CAMERA_BRIDGE_Z - 60.0);
    }
    slots
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "incubator_condensate_robot_keepout_front_sweep_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + ROBOT_KEEPOUT_Y / 2.0),
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let service_left = centered_cube(
        "incubator_condensate_service_keepout_left_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -(DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0),
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "incubator_condensate_service_keepout_right_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let rear_drain = centered_cube(
        "incubator_condensate_rear_drain_service_keepout_gauge",
        DECK_X - 180.0,
        REAR_DRAIN_KEEP_OUT_Y,
        82.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_DRAIN_KEEP_OUT_Y / 2.0, 41.0);
    let top = centered_cube(
        "incubator_condensate_top_service_clearance_gauge",
        720.0,
        462.0,
        8.0,
    )
    .translate(28.0, -18.0, TOP_SERVICE_CLEARANCE_Z);

    robot_front + service_left + service_right + rear_drain + top
}

fn station_assembly() -> Part {
    station_deck_secondary_containment_moat()
        + condensate_drain_trap_surrogate().translate(TRAP_POS.0, TRAP_POS.1, top_z(TRAP_Z))
        + clog_overflow_challenge_cartridges().translate(
            CARTRIDGE_BANK_POS.0,
            CARTRIDGE_BANK_POS.1,
            top_z(CARTRIDGE_BANK_Z),
        )
        + wetness_sensor_docks().translate(
            WETNESS_DOCK_BANK_POS.0,
            WETNESS_DOCK_BANK_POS.1,
            top_z(WETNESS_DOCK_BANK_Z),
        )
        + level_reference_wells().translate(
            LEVEL_WELL_BLOCK_POS.0,
            LEVEL_WELL_BLOCK_POS.1,
            top_z(LEVEL_WELL_BLOCK_Z),
        )
        + alarm_token_rail().translate(ALARM_RAIL_POS.0, ALARM_RAIL_POS.1, top_z(ALARM_RAIL_Z))
        + recovery_flush_route().translate(
            FLUSH_ROUTE_POS.0,
            FLUSH_ROUTE_POS.1,
            top_z(FLUSH_ROUTE_Z),
        )
        + biofilm_coupon_retainers().translate(
            COUPON_BANK_POS.0,
            COUPON_BANK_POS.1,
            top_z(COUPON_BANK_Z),
        )
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge().translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, DECK_Z / 2.0)
        + robot_service_keepouts()
}

fn cartridge_center_x(index: usize) -> f64 {
    -((CLOG_CARTRIDGE_COUNT as f64 - 1.0) * CARTRIDGE_PITCH_X) / 2.0
        + index as f64 * CARTRIDGE_PITCH_X
}

fn wetness_dock_center(index: usize) -> (f64, f64) {
    let row = index / 3;
    let col = index % 3;
    (
        -WETNESS_DOCK_PITCH_X + col as f64 * WETNESS_DOCK_PITCH_X,
        -WETNESS_DOCK_PITCH_Y / 2.0 + row as f64 * WETNESS_DOCK_PITCH_Y,
    )
}

fn alarm_slot_x(index: usize) -> f64 {
    -((ALARM_TOKEN_SLOTS as f64 - 1.0) * ALARM_TOKEN_PITCH) / 2.0 + index as f64 * ALARM_TOKEN_PITCH
}

fn gate_center_x(index: usize) -> f64 {
    -112.0 + index as f64 * 112.0
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn assert_layout() {
    assert_eq!(DATUM_BOSSES, 8);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert!(TRAP_WATER_SEAL_DEPTH < TRAP_Z);
    assert!(MOAT_W * 2.0 + MOAT_INSET_X < DECK_X - RIM_W);
    assert!(MOAT_W * 2.0 + MOAT_INSET_Y < DECK_Y - RIM_W);
    assert!(TRAP_POS.0 - TRAP_X / 2.0 > -DECK_X / 2.0 + RIM_W);
    assert!(LEVEL_WELL_BLOCK_POS.0 + LEVEL_WELL_BLOCK_X / 2.0 < DECK_X / 2.0 - RIM_W);
    assert!(CUSTODY_POS.1 + CUSTODY_Y / 2.0 < DECK_Y / 2.0 - RIM_W);
    assert!(GATE_POS.1 - GATE_Y / 2.0 > -DECK_Y / 2.0 + RIM_W);
    assert!(ALARM_RAIL_POS.0 - ALARM_RAIL_X / 2.0 > -DECK_X / 2.0 + RIM_W);
    assert!(cartridge_span() + CARTRIDGE_SLOT_X < CARTRIDGE_BANK_X - 34.0);
    assert!(alarm_token_span() + 38.0 < ALARM_RAIL_X - 40.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > DECK_Z + CAMERA_BRIDGE_Z);
}

fn cartridge_span() -> f64 {
    (CLOG_CARTRIDGE_COUNT - 1) as f64 * CARTRIDGE_PITCH_X
}

fn alarm_token_span() -> f64 {
    (ALARM_TOKEN_SLOTS - 1) as f64 * ALARM_TOKEN_PITCH
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[cfg(test)]
impl Rect {
    fn left(self) -> f64 {
        self.x - self.w / 2.0
    }

    fn right(self) -> f64 {
        self.x + self.w / 2.0
    }

    fn bottom(self) -> f64 {
        self.y - self.h / 2.0
    }

    fn top(self) -> f64 {
        self.y + self.h / 2.0
    }
}

#[cfg(test)]
fn feature_rects() -> Vec<Rect> {
    vec![
        Rect {
            x: TRAP_POS.0,
            y: TRAP_POS.1,
            w: TRAP_X,
            h: TRAP_Y,
        },
        Rect {
            x: CARTRIDGE_BANK_POS.0,
            y: CARTRIDGE_BANK_POS.1,
            w: CARTRIDGE_BANK_X,
            h: CARTRIDGE_BANK_Y,
        },
        Rect {
            x: WETNESS_DOCK_BANK_POS.0,
            y: WETNESS_DOCK_BANK_POS.1,
            w: WETNESS_DOCK_BANK_X,
            h: WETNESS_DOCK_BANK_Y,
        },
        Rect {
            x: LEVEL_WELL_BLOCK_POS.0,
            y: LEVEL_WELL_BLOCK_POS.1,
            w: LEVEL_WELL_BLOCK_X,
            h: LEVEL_WELL_BLOCK_Y,
        },
        Rect {
            x: ALARM_RAIL_POS.0,
            y: ALARM_RAIL_POS.1,
            w: ALARM_RAIL_X,
            h: ALARM_RAIL_Y,
        },
        Rect {
            x: FLUSH_ROUTE_POS.0,
            y: FLUSH_ROUTE_POS.1,
            w: FLUSH_ROUTE_X,
            h: FLUSH_ROUTE_Y,
        },
        Rect {
            x: COUPON_BANK_POS.0,
            y: COUPON_BANK_POS.1,
            w: COUPON_BANK_X,
            h: COUPON_BANK_Y,
        },
        Rect {
            x: CUSTODY_POS.0,
            y: CUSTODY_POS.1,
            w: CUSTODY_X,
            h: CUSTODY_Y,
        },
        Rect {
            x: GATE_POS.0,
            y: GATE_POS.1,
            w: GATE_X,
            h: GATE_Y,
        },
    ]
}

#[cfg(test)]
fn inside_deck(rect: Rect) -> bool {
    rect.left() > -DECK_X / 2.0 + RIM_W
        && rect.right() < DECK_X / 2.0 - RIM_W
        && rect.bottom() > -DECK_Y / 2.0 + RIM_W
        && rect.top() < DECK_Y / 2.0 - RIM_W
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_incubator_condensate_drain_alarm_recovery_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_validation_features_are_explicit() {
        for feature in [
            "condensate_drain_trap_surrogate",
            "clog_challenge_cartridges",
            "overflow_challenge_cartridges",
            "wetness_sensor_docks",
            "level_reference_wells",
            "alarm_token_rail",
            "recovery_flush_route",
            "biofilm_coupon_retainers",
            "secondary_containment_moat",
            "barcode_custody_lands",
            "coa_custody_lands",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "camera_evidence_bridge",
            "robot_keepouts",
            "service_keepouts",
        ] {
            assert!(REQUIRED_INTENT_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn scope_note_excludes_process_or_biology_claims() {
        assert!(MECHANICAL_SCOPE_NOTE.contains("mechanical validation packaging only"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("no sterile-process claim"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("cleaning SOP"));
        assert!(MECHANICAL_SCOPE_NOTE.contains("biological acceptance criterion"));
    }

    #[test]
    fn challenge_and_recovery_counts_cover_alarm_scenarios() {
        assert_eq!(TRAP_PORT_COUNT, 4);
        assert_eq!(TRAP_WINDOW_COUNT, 4);
        assert!(CLOG_CARTRIDGE_COUNT >= OVERFLOW_WEIR_COUNT);
        assert!(WETNESS_DOCKS >= 6);
        assert_eq!(LEVEL_WELLS, LEVEL_REFERENCE_STEPS);
        assert!(ALARM_TOKEN_SLOTS >= LEVEL_WELLS);
        assert!(FLUSH_PORTS >= TRAP_PORT_COUNT);
        assert!(FLUSH_ROUTE_SEGMENTS >= 5);
        assert!(BIOFILM_COUPON_RETAINERS >= WETNESS_DOCKS);
    }

    #[test]
    fn custody_disposition_and_evidence_capacity_is_physical() {
        assert!(BARCODE_LANDS >= BIOFILM_COUPON_RETAINERS);
        assert_eq!(COA_LANDS, DISPOSITION_GATES);
        assert_eq!(DISPOSITION_GATES, 3);
        assert!(GATE_TOKEN_SLOTS >= DISPOSITION_GATES * 2);
        assert_eq!(CAMERA_MOUNTS, 4);
        assert!(EVIDENCE_FIDUCIALS >= CAMERA_MOUNTS * 2);
    }

    #[test]
    fn feature_envelopes_stay_on_contained_deck() {
        for rect in feature_rects() {
            assert!(inside_deck(rect));
        }
        assert!(MOAT_INSET_X < DECK_X - 2.0 * RIM_W);
        assert!(MOAT_INSET_Y < DECK_Y - 2.0 * RIM_W);
        assert_layout();
    }

    #[test]
    fn helper_geometry_is_centered_and_in_bounds() {
        assert!(
            (cartridge_center_x(0) + cartridge_center_x(CLOG_CARTRIDGE_COUNT - 1)).abs() < 0.001
        );
        assert!((alarm_slot_x(0) + alarm_slot_x(ALARM_TOKEN_SLOTS - 1)).abs() < 0.001);
        assert!((wetness_dock_center(0).0 + wetness_dock_center(2).0).abs() < 0.001);
        assert!((wetness_dock_center(0).1 + wetness_dock_center(3).1).abs() < 0.001);
        assert!(cartridge_span() < CARTRIDGE_BANK_X - 80.0);
        assert!(alarm_token_span() < ALARM_RAIL_X - 70.0);
    }
}
