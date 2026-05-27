use std::collections::BTreeSet;
use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system perfusion-line air-ingress and microleak challenge station.
//
// Intent:
// - Package closed tubing lanes with combed strain relief, calibrated
//   microleak coupon placeholders, optical bubble witness windows,
//   pressure-decay/reference ports, a degas-trap witness pocket, and a sterile
//   connector bulkhead into one validation fixture.
// - Provide tray containment, barcode/certificate traceability, disposition
//   lanes, and visible robot/service keepout gauges.
// - Represent mechanical validation packaging only. Wetted components,
//   pressure-rated design, leak acceptance thresholds, optics, sensors, and
//   sterility protocols remain external controlled items.

const OUTPUT_PREFIX: &str = "output/closed_perfusion_line_air_ingress_microleak_challenge_station";
const OUTPUTS: [&str; 11] = [
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_base_leak_tray.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_closed_tubing_lane_combs.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_microleak_coupon_placeholders.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_optical_bubble_witness_windows.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_pressure_decay_reference_ports.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_degas_trap_witness_pocket.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_sterile_connector_bulkhead.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_barcode_certificate_lands.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_robot_service_keepout_gauges.stl",
    "output/closed_perfusion_line_air_ingress_microleak_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "leak_tray",
    "closed_tubing_lane_combs",
    "microleak_coupon_placeholders",
    "optical_bubble_witness_windows",
    "pressure_decay_reference_ports",
    "degas_trap_witness_pocket",
    "sterile_connector_bulkhead",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "robot_service_keepout_gauges",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_packaging_only",
    "not_pressure_rated_system_design",
    "not_leak_acceptance_protocol",
    "purchased_sensors_connectors_coupons_as_placeholders",
    "sterility_and_wetted_materials_external",
];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_RECESS_Z: f64 = 7.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_RING_D: f64 = 18.0;

const LANES: usize = 8;
const LANE_PITCH_X: f64 = 92.0;
const TUBE_OD_MAX: f64 = 6.4;
const TUBE_CLEARANCE: f64 = 1.2;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const INLET_OUTLET_PORTS: usize = LANES * 2;

const COMB_CENTER: (f64, f64) = (0.0, 76.0);
const COMB_DECK_X: f64 = 880.0;
const COMB_DECK_Y: f64 = 190.0;
const COMB_DECK_Z: f64 = 28.0;
const COMB_BAR_Y: f64 = 28.0;
const COMB_TOOTH_X: f64 = 15.0;
const COMB_TOOTH_Y: f64 = 46.0;
const COMB_TOOTH_Z: f64 = 38.0;
const LANE_DIVIDER_Z: f64 = 58.0;

const COUPON_BANK_CENTER: (f64, f64) = (-398.0, -118.0);
const COUPON_BANK_X: f64 = 350.0;
const COUPON_BANK_Y: f64 = 176.0;
const COUPON_BANK_Z: f64 = 34.0;
const MICROLEAK_COUPONS: usize = LANES;
const COUPON_PITCH_X: f64 = 40.0;
const COUPON_SLOT_X: f64 = 28.0;
const COUPON_SLOT_Y: f64 = 118.0;
const COUPON_SLOT_Z: f64 = 18.0;
const MICROLEAK_ORIFICE_D: f64 = 5.0;
const COUPON_WITNESS_RING_D: f64 = 20.0;

const OPTICAL_BRIDGE_CENTER: (f64, f64) = (0.0, -42.0);
const OPTICAL_BRIDGE_X: f64 = 920.0;
const OPTICAL_BRIDGE_Y: f64 = 94.0;
const OPTICAL_BRIDGE_Z: f64 = 20.0;
const OPTICAL_UNDERSIDE_Z: f64 = 108.0;
const BUBBLE_WINDOWS: usize = LANES;
const WINDOW_X: f64 = 44.0;
const WINDOW_Y: f64 = 20.0;
const WINDOW_Z: f64 = 28.0;
const ILLUMINATOR_LANDS: usize = BUBBLE_WINDOWS;

const PRESSURE_PANEL_CENTER: (f64, f64) = (366.0, -118.0);
const PRESSURE_PANEL_X: f64 = 366.0;
const PRESSURE_PANEL_Y: f64 = 164.0;
const PRESSURE_PANEL_Z: f64 = 48.0;
const PRESSURE_PORTS: usize = LANES;
const PRESSURE_PORT_PITCH_X: f64 = 39.0;
const PRESSURE_PORT_D: f64 = 8.0;
const REFERENCE_PORTS: usize = 4;
const REFERENCE_PORT_D: f64 = 12.0;
const REFERENCE_VOLUME_BLOCKS: usize = REFERENCE_PORTS;
const PRESSURE_TRANSDUCER_DOCKS: usize = LANES;

const DEGAS_CENTER: (f64, f64) = (386.0, 134.0);
const DEGAS_POCKET_X: f64 = 300.0;
const DEGAS_POCKET_Y: f64 = 178.0;
const DEGAS_POCKET_Z: f64 = 58.0;
const DEGAS_TRAPS: usize = 4;
const DEGAS_TRAP_PITCH_X: f64 = 56.0;
const DEGAS_TRAP_D: f64 = 38.0;
const DEGAS_TRAP_WITNESS_WINDOWS: usize = DEGAS_TRAPS;
const DEGAS_HIGH_POINT_MARKERS: usize = DEGAS_TRAPS;

const BULKHEAD_CENTER: (f64, f64) = (0.0, 284.0);
const BULKHEAD_X: f64 = 720.0;
const BULKHEAD_Y: f64 = 72.0;
const BULKHEAD_Z: f64 = 64.0;
const STERILE_CONNECTOR_PORTS: usize = INLET_OUTLET_PORTS;
const CONNECTOR_PITCH_X: f64 = 39.0;
const CONNECTOR_BORE_D: f64 = 10.0;
const CONNECTOR_GASKET_D: f64 = 22.0;
const CONNECTOR_POLARITY_KEYS: usize = LANES;

const TRACE_CENTER: (f64, f64) = (-404.0, -294.0);
const TRACE_PANEL_X: f64 = 320.0;
const TRACE_PANEL_Y: f64 = 96.0;
const TRACE_PANEL_Z: f64 = 11.0;
const BARCODE_LANDS: usize = LANES;
const CERTIFICATE_LANDS: usize = 4;
const RUN_RECORD_LANDS: usize = 3;

const DISPOSITION_CENTER: (f64, f64) = (34.0, -294.0);
const DISPOSITION_X: f64 = 420.0;
const DISPOSITION_Y: f64 = 118.0;
const DISPOSITION_Z: f64 = 32.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 42.0;
const STATUS_SLOT_Y: f64 = 26.0;

const KEEP_OUT_GAUGES: usize = 6;
const ROBOT_SWEEP_X: f64 = 1060.0;
const ROBOT_SWEEP_Y: f64 = 660.0;
const ROBOT_SWEEP_Z: f64 = 172.0;
const FRONT_SERVICE_CLEARANCE: f64 = 245.0;
const REAR_CONNECTOR_SERVICE_CLEARANCE: f64 = 192.0;
const COUPON_TWEEZER_CLEARANCE: f64 = 150.0;
const DEGAS_LIFT_CLEARANCE: f64 = 188.0;
const OPTICAL_CAMERA_CLEARANCE: f64 = 230.0;
const PRESSURE_PANEL_SERVICE_CLEARANCE: f64 = 160.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let combs = closed_tubing_lane_combs();
    export(&combs, OUTPUTS[1]);

    let coupons = microleak_coupon_placeholders();
    export(&coupons, OUTPUTS[2]);

    let optical = optical_bubble_witness_windows();
    export(&optical, OUTPUTS[3]);

    let pressure = pressure_decay_reference_ports();
    export(&pressure, OUTPUTS[4]);

    let degas = degas_trap_witness_pocket();
    export(&degas, OUTPUTS[5]);

    let bulkhead = sterile_connector_bulkhead();
    export(&bulkhead, OUTPUTS[6]);

    let traceability = barcode_certificate_lands();
    export(&traceability, OUTPUTS[7]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[8]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[9]);

    let assembly = base
        + combs
        + coupons
        + optical
        + pressure
        + degas
        + bulkhead
        + traceability
        + disposition
        + keepouts;
    export(&assembly, OUTPUTS[10]);

    println!(
        "Closed perfusion-line air-ingress and microleak challenge station: {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray, {LANES} closed tubing lanes, {MICROLEAK_COUPONS} microleak coupon placeholders, {BUBBLE_WINDOWS} optical bubble windows, {PRESSURE_PORTS} pressure-decay ports, and {REFERENCE_PORTS} reference ports."
    );
    println!(
        "Closed-path interfaces: {STERILE_CONNECTOR_PORTS} sterile connector bulkhead ports, {DEGAS_TRAPS} degas trap witness pockets, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {STATUS_LANES} release/hold/reject lanes, {KEEP_OUT_GAUGES} keepout gauges, {} limitation markers, and {} required feature groups.",
        LIMITATIONS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_perfusion_air_microleak_base_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin_recess = centered_cube(
        "closed_perfusion_air_microleak_base_sumped_recess",
        STATION_X - RIM_W * 2.0 - 54.0,
        STATION_Y - RIM_W * 2.0 - 54.0,
        BASIN_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.2);
    let front_drain = centered_cylinder(
        "closed_perfusion_air_microleak_front_leak_tray_drain",
        DRAIN_D / 2.0,
        66.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -(STATION_Y / 2.0 - 18.0),
        BASE_Z - 5.0,
    );
    let rear_witness_sump = centered_cylinder(
        "closed_perfusion_air_microleak_rear_witness_sump_land",
        13.0,
        3.0,
        32,
    )
    .translate(
        -(STATION_X / 2.0 - 92.0),
        STATION_Y / 2.0 - 74.0,
        BASE_Z + 1.5,
    );

    deck - basin_recess - front_drain - mounting_hole_cuts()
        + leak_tray_rims()
        + mounting_bosses()
        + datum_fiducials()
        + flow_direction_lands()
        + rear_witness_sump
}

fn leak_tray_rims() -> Part {
    let z = BASE_Z + RIM_Z / 2.0;
    let left = centered_cube(
        "closed_perfusion_air_microleak_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        "closed_perfusion_air_microleak_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        "closed_perfusion_air_microleak_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front_low_lip = centered_cube(
        "closed_perfusion_air_microleak_front_low_service_lip",
        STATION_X - 124.0,
        RIM_W,
        RIM_Z * 0.58,
    )
    .translate(0.0, -(STATION_Y / 2.0 - RIM_W / 2.0), BASE_Z + RIM_Z * 0.29);

    left + right + rear + front_low_lip
}

fn mounting_hole_cuts() -> Part {
    let mut holes = Part::empty("closed_perfusion_air_microleak_mounting_hole_cuts");
    for (i, (x, y)) in mount_points().iter().copied().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_m6_mount_hole_cut_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 10.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn mounting_bosses() -> Part {
    let mut bosses = Part::empty("closed_perfusion_air_microleak_mounting_bosses");
    for (i, (x, y)) in mount_points().iter().copied().enumerate() {
        let boss = centered_cylinder(
            format!("closed_perfusion_air_microleak_mount_boss_{i}"),
            18.0,
            8.0,
            32,
        )
        .translate(x, y, BASE_Z + 4.0);
        let bore = centered_cylinder(
            format!("closed_perfusion_air_microleak_mount_boss_bore_{i}"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            28,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn datum_fiducials() -> Part {
    let mut datums = Part::empty("closed_perfusion_air_microleak_datum_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 72.0), -(STATION_Y / 2.0 - 64.0)),
        (STATION_X / 2.0 - 72.0, -(STATION_Y / 2.0 - 64.0)),
        (-(STATION_X / 2.0 - 72.0), STATION_Y / 2.0 - 64.0),
        (STATION_X / 2.0 - 72.0, STATION_Y / 2.0 - 64.0),
        (0.0, STATION_Y / 2.0 - 64.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_perfusion_air_microleak_datum_ring_{i}"),
            DATUM_RING_D / 2.0,
            3.0,
            32,
        )
        .translate(x, y, BASE_Z + 1.5);
        let center = centered_cylinder(
            format!("closed_perfusion_air_microleak_datum_center_pip_{i}"),
            2.2,
            4.0,
            18,
        )
        .translate(x, y, BASE_Z + 1.5);
        datums = datums + (ring - center);
    }
    datums
}

fn flow_direction_lands() -> Part {
    let inlet_rail = centered_cube(
        "closed_perfusion_air_microleak_inlet_flow_direction_land",
        710.0,
        7.0,
        6.0,
    )
    .translate(0.0, BULKHEAD_CENTER.1 - 58.0, BASE_Z + 3.0);
    let challenge_rail = centered_cube(
        "closed_perfusion_air_microleak_microleak_challenge_direction_land",
        810.0,
        7.0,
        6.0,
    )
    .translate(0.0, COMB_CENTER.1 - 42.0, BASE_Z + 3.0);
    let disposition_rail = centered_cube(
        "closed_perfusion_air_microleak_disposition_direction_land",
        530.0,
        7.0,
        6.0,
    )
    .translate(-176.0, DISPOSITION_CENTER.1 + 78.0, BASE_Z + 3.0);

    inlet_rail + challenge_rail + disposition_rail
}

fn closed_tubing_lane_combs() -> Part {
    let deck = centered_cube(
        "closed_perfusion_air_microleak_tubing_lane_comb_deck",
        COMB_DECK_X,
        COMB_DECK_Y,
        COMB_DECK_Z,
    )
    .translate(COMB_CENTER.0, COMB_CENTER.1, BASE_Z + COMB_DECK_Z / 2.0);
    let inlet_comb = comb_bar("inlet", COMB_CENTER.1 + COMB_DECK_Y / 2.0 - 30.0);
    let outlet_comb = comb_bar("outlet", COMB_CENTER.1 - COMB_DECK_Y / 2.0 + 30.0);
    let mut lane_troughs = Part::empty("closed_perfusion_air_microleak_lane_troughs");
    let mut lane_dividers = Part::empty("closed_perfusion_air_microleak_lane_dividers");

    for lane in 0..LANES {
        let x = lane_center(lane);
        lane_troughs = lane_troughs
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_lane_{lane}_tube_trough"),
                TUBE_CHANNEL_D / 2.0,
                COMB_DECK_Y + 26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, COMB_CENTER.1, BASE_Z + COMB_DECK_Z + 2.0);

        if lane + 1 < LANES {
            lane_dividers = lane_dividers
                + centered_cube(
                    format!("closed_perfusion_air_microleak_lane_{lane}_separation_wall"),
                    4.5,
                    COMB_DECK_Y - 22.0,
                    LANE_DIVIDER_Z,
                )
                .translate(
                    x + LANE_PITCH_X / 2.0,
                    COMB_CENTER.1,
                    BASE_Z + COMB_DECK_Z + LANE_DIVIDER_Z / 2.0,
                );
        }
    }

    deck - lane_troughs + inlet_comb + outlet_comb + lane_dividers + lane_id_lands()
}

fn comb_bar(kind: &str, y: f64) -> Part {
    let bar = centered_cube(
        format!("closed_perfusion_air_microleak_{kind}_comb_bar"),
        COMB_DECK_X - 42.0,
        COMB_BAR_Y,
        34.0,
    )
    .translate(COMB_CENTER.0, y, BASE_Z + COMB_DECK_Z + 17.0);
    let mut notches = Part::empty(format!(
        "closed_perfusion_air_microleak_{kind}_comb_notches"
    ));
    let mut teeth = Part::empty(format!("closed_perfusion_air_microleak_{kind}_comb_teeth"));

    for lane in 0..LANES {
        let x = lane_center(lane);
        notches = notches
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_{kind}_lane_{lane}_comb_tube_notch"),
                TUBE_CHANNEL_D / 2.0,
                COMB_BAR_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, BASE_Z + COMB_DECK_Z + 17.0);

        for (side, dx) in [("left", -13.0), ("right", 13.0)] {
            teeth = teeth
                + centered_cube(
                    format!("closed_perfusion_air_microleak_{kind}_lane_{lane}_{side}_comb_tooth"),
                    COMB_TOOTH_X,
                    COMB_TOOTH_Y,
                    COMB_TOOTH_Z,
                )
                .translate(x + dx, y, BASE_Z + COMB_DECK_Z + COMB_TOOTH_Z / 2.0);
        }
    }

    bar - notches + teeth
}

fn lane_id_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_air_microleak_lane_id_lands");
    for lane in 0..LANES {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_id_land"),
                46.0,
                16.0,
                3.0,
            )
            .translate(lane_center(lane), COMB_CENTER.1, BASE_Z + COMB_DECK_Z + 1.5);
    }
    lands
}

fn microleak_coupon_placeholders() -> Part {
    let body = centered_cube(
        "closed_perfusion_air_microleak_coupon_bank_body",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(
        COUPON_BANK_CENTER.0,
        COUPON_BANK_CENTER.1,
        BASE_Z + COUPON_BANK_Z / 2.0,
    );
    let clean_lip = centered_cube(
        "closed_perfusion_air_microleak_coupon_bank_clean_lip_witness",
        COUPON_BANK_X - 24.0,
        COUPON_BANK_Y - 24.0,
        5.0,
    )
    .translate(
        COUPON_BANK_CENTER.0,
        COUPON_BANK_CENTER.1,
        BASE_Z + COUPON_BANK_Z + 2.5,
    );
    let mut slot_cuts = Part::empty("closed_perfusion_air_microleak_coupon_slot_cuts");
    let mut witness_features =
        Part::empty("closed_perfusion_air_microleak_coupon_witness_features");

    for coupon in 0..MICROLEAK_COUPONS {
        let x = coupon_x(coupon);
        slot_cuts = slot_cuts
            + centered_cube(
                format!(
                    "closed_perfusion_air_microleak_coupon_{coupon}_rectangular_placeholder_slot"
                ),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_SLOT_Z,
            )
            .translate(
                x,
                COUPON_BANK_CENTER.1,
                BASE_Z + COUPON_BANK_Z - COUPON_SLOT_Z / 2.0 + 0.2,
            )
            + centered_cylinder(
                format!(
                    "closed_perfusion_air_microleak_coupon_{coupon}_micro_orifice_witness_bore"
                ),
                MICROLEAK_ORIFICE_D / 2.0,
                COUPON_BANK_Z + 8.0,
                20,
            )
            .translate(x, COUPON_BANK_CENTER.1, BASE_Z + COUPON_BANK_Z / 2.0);

        witness_features = witness_features
            + centered_cylinder(
                format!(
                    "closed_perfusion_air_microleak_coupon_{coupon}_calibrated_leak_witness_ring"
                ),
                COUPON_WITNESS_RING_D / 2.0,
                3.0,
                28,
            )
            .translate(x, COUPON_BANK_CENTER.1, BASE_Z + COUPON_BANK_Z + 1.5)
            + centered_cube(
                format!("closed_perfusion_air_microleak_coupon_{coupon}_serial_land"),
                28.0,
                16.0,
                3.0,
            )
            .translate(
                x,
                COUPON_BANK_CENTER.1 - COUPON_BANK_Y / 2.0 + 20.0,
                BASE_Z + COUPON_BANK_Z + 1.5,
            )
            + centered_cube(
                format!("closed_perfusion_air_microleak_coupon_{coupon}_inlet_outlet_witness_land"),
                26.0,
                14.0,
                3.0,
            )
            .translate(
                x,
                COUPON_BANK_CENTER.1 + COUPON_BANK_Y / 2.0 - 22.0,
                BASE_Z + COUPON_BANK_Z + 1.5,
            );
    }

    body - slot_cuts + clean_lip + witness_features + coupon_bypass_header()
}

fn coupon_bypass_header() -> Part {
    let header = centered_cube(
        "closed_perfusion_air_microleak_coupon_bypass_header",
        COUPON_BANK_X - 40.0,
        24.0,
        28.0,
    )
    .translate(
        COUPON_BANK_CENTER.0,
        COUPON_BANK_CENTER.1 + COUPON_BANK_Y / 2.0 + 18.0,
        BASE_Z + COUPON_BANK_Z / 2.0 + 14.0,
    );
    let mut bores = Part::empty("closed_perfusion_air_microleak_coupon_bypass_bores");
    for coupon in 0..MICROLEAK_COUPONS {
        bores = bores
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_coupon_{coupon}_bypass_tube_bore"),
                TUBE_CHANNEL_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                coupon_x(coupon),
                COUPON_BANK_CENTER.1 + COUPON_BANK_Y / 2.0 + 18.0,
                BASE_Z + COUPON_BANK_Z / 2.0 + 14.0,
            );
    }
    header - bores
}

fn optical_bubble_witness_windows() -> Part {
    let bridge = centered_cube(
        "closed_perfusion_air_microleak_optical_bubble_witness_bridge",
        OPTICAL_BRIDGE_X,
        OPTICAL_BRIDGE_Y,
        OPTICAL_BRIDGE_Z,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0,
        OPTICAL_BRIDGE_CENTER.1,
        BASE_Z + OPTICAL_UNDERSIDE_Z + OPTICAL_BRIDGE_Z / 2.0,
    );
    let left_column = centered_cube(
        "closed_perfusion_air_microleak_optical_bridge_left_column",
        18.0,
        OPTICAL_BRIDGE_Y,
        OPTICAL_UNDERSIDE_Z,
    )
    .translate(
        -(OPTICAL_BRIDGE_X / 2.0 - 26.0),
        OPTICAL_BRIDGE_CENTER.1,
        BASE_Z + OPTICAL_UNDERSIDE_Z / 2.0,
    );
    let right_column = centered_cube(
        "closed_perfusion_air_microleak_optical_bridge_right_column",
        18.0,
        OPTICAL_BRIDGE_Y,
        OPTICAL_UNDERSIDE_Z,
    )
    .translate(
        OPTICAL_BRIDGE_X / 2.0 - 26.0,
        OPTICAL_BRIDGE_CENTER.1,
        BASE_Z + OPTICAL_UNDERSIDE_Z / 2.0,
    );

    let mut window_cuts = Part::empty("closed_perfusion_air_microleak_optical_window_cuts");
    let mut frames = Part::empty("closed_perfusion_air_microleak_optical_window_frames");
    let mut illuminators = Part::empty("closed_perfusion_air_microleak_illuminator_lands");
    for lane in 0..BUBBLE_WINDOWS {
        let x = lane_center(lane);
        window_cuts = window_cuts
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_bubble_witness_window_cut"),
                WINDOW_X,
                WINDOW_Y,
                WINDOW_Z,
            )
            .translate(
                x,
                OPTICAL_BRIDGE_CENTER.1,
                BASE_Z + OPTICAL_UNDERSIDE_Z + OPTICAL_BRIDGE_Z / 2.0,
            );
        frames = frames
            + optical_window_frame(lane, x)
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_bubble_scale_land"),
                38.0,
                5.0,
                4.0,
            )
            .translate(
                x,
                OPTICAL_BRIDGE_CENTER.1 - OPTICAL_BRIDGE_Y / 2.0 - 8.0,
                BASE_Z + OPTICAL_UNDERSIDE_Z + 6.0,
            );
        illuminators = illuminators
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_backlight_mount_land"),
                34.0,
                12.0,
                5.0,
            )
            .translate(
                x,
                OPTICAL_BRIDGE_CENTER.1 + OPTICAL_BRIDGE_Y / 2.0 + 10.0,
                BASE_Z + OPTICAL_UNDERSIDE_Z + OPTICAL_BRIDGE_Z - 2.5,
            );
    }

    bridge - window_cuts + left_column + right_column + frames + illuminators
}

fn optical_window_frame(lane: usize, x: f64) -> Part {
    let z = BASE_Z + OPTICAL_UNDERSIDE_Z + OPTICAL_BRIDGE_Z / 2.0;
    let y = OPTICAL_BRIDGE_CENTER.1;
    let top = centered_cube(
        format!("closed_perfusion_air_microleak_lane_{lane}_window_top_frame"),
        WINDOW_X + 10.0,
        4.0,
        5.0,
    )
    .translate(x, y + WINDOW_Y / 2.0 + 3.0, z);
    let bottom = centered_cube(
        format!("closed_perfusion_air_microleak_lane_{lane}_window_bottom_frame"),
        WINDOW_X + 10.0,
        4.0,
        5.0,
    )
    .translate(x, y - WINDOW_Y / 2.0 - 3.0, z);
    let left = centered_cube(
        format!("closed_perfusion_air_microleak_lane_{lane}_window_left_frame"),
        4.0,
        WINDOW_Y + 10.0,
        5.0,
    )
    .translate(x - WINDOW_X / 2.0 - 3.0, y, z);
    let right = centered_cube(
        format!("closed_perfusion_air_microleak_lane_{lane}_window_right_frame"),
        4.0,
        WINDOW_Y + 10.0,
        5.0,
    )
    .translate(x + WINDOW_X / 2.0 + 3.0, y, z);

    top + bottom + left + right
}

fn pressure_decay_reference_ports() -> Part {
    let panel = centered_cube(
        "closed_perfusion_air_microleak_pressure_decay_reference_panel",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1,
        BASE_Z + PRESSURE_PANEL_Z / 2.0,
    );
    let mut cuts = Part::empty("closed_perfusion_air_microleak_pressure_decay_port_cuts");
    let mut pads = Part::empty("closed_perfusion_air_microleak_pressure_decay_port_pads");
    let mut transducer_docks =
        Part::empty("closed_perfusion_air_microleak_pressure_transducer_docks");

    for lane in 0..PRESSURE_PORTS {
        let x = pressure_lane_x(lane);
        cuts =
            cuts + centered_cylinder(
                format!("closed_perfusion_air_microleak_lane_{lane}_pressure_decay_port_bore"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_PANEL_Z + 10.0,
                24,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 + 24.0,
                BASE_Z + PRESSURE_PANEL_Z / 2.0,
            ) + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_transducer_pocket_cut"),
                28.0,
                20.0,
                16.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - 24.0,
                BASE_Z + PRESSURE_PANEL_Z - 7.5,
            );
        pads = pads
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_lane_{lane}_pressure_port_pad"),
                9.5,
                4.0,
                28,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 + 24.0,
                BASE_Z + PRESSURE_PANEL_Z + 2.0,
            );
        transducer_docks = transducer_docks
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_pressure_transducer_dock_land"),
                32.0,
                18.0,
                4.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - 24.0,
                BASE_Z + PRESSURE_PANEL_Z + 2.0,
            );
    }

    for port in 0..REFERENCE_PORTS {
        let x = reference_port_x(port);
        cuts = cuts
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_reference_port_{port}_bore"),
                REFERENCE_PORT_D / 2.0,
                PRESSURE_PANEL_Z + 12.0,
                28,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 + PRESSURE_PANEL_Y / 2.0 - 26.0,
                BASE_Z + PRESSURE_PANEL_Z / 2.0,
            );
        pads =
            pads + centered_cylinder(
                format!("closed_perfusion_air_microleak_reference_port_{port}_gauge_pad"),
                16.0,
                5.0,
                32,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 + PRESSURE_PANEL_Y / 2.0 - 26.0,
                BASE_Z + PRESSURE_PANEL_Z + 2.5,
            ) + centered_cube(
                format!("closed_perfusion_air_microleak_reference_volume_block_{port}"),
                38.0,
                42.0,
                28.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - PRESSURE_PANEL_Y / 2.0 + 26.0,
                BASE_Z + PRESSURE_PANEL_Z + 14.0,
            );
    }

    panel - cuts + pads + transducer_docks + pressure_decay_route_ribs()
}

fn pressure_decay_route_ribs() -> Part {
    let mut ribs = Part::empty("closed_perfusion_air_microleak_pressure_decay_route_ribs");
    for lane in 0..PRESSURE_PORTS {
        ribs = ribs
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_pressure_decay_route_rib"),
                4.0,
                PRESSURE_PANEL_Y - 36.0,
                5.0,
            )
            .translate(
                pressure_lane_x(lane),
                PRESSURE_PANEL_CENTER.1,
                BASE_Z + PRESSURE_PANEL_Z + 2.5,
            );
    }
    ribs
}

fn degas_trap_witness_pocket() -> Part {
    let body = centered_cube(
        "closed_perfusion_air_microleak_degas_trap_witness_pocket_body",
        DEGAS_POCKET_X,
        DEGAS_POCKET_Y,
        DEGAS_POCKET_Z,
    )
    .translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1,
        BASE_Z + DEGAS_POCKET_Z / 2.0,
    );
    let witness_window_cut = centered_cube(
        "closed_perfusion_air_microleak_degas_trap_front_witness_window_cut",
        DEGAS_POCKET_X - 58.0,
        14.0,
        32.0,
    )
    .translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1 - DEGAS_POCKET_Y / 2.0 - 0.5,
        BASE_Z + DEGAS_POCKET_Z / 2.0,
    );
    let mut trap_cups = Part::empty("closed_perfusion_air_microleak_degas_trap_cup_cuts");
    let mut witness_features = Part::empty("closed_perfusion_air_microleak_degas_trap_witnesses");
    for trap in 0..DEGAS_TRAPS {
        let x = degas_trap_x(trap);
        trap_cups = trap_cups
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_degas_trap_{trap}_pocket_bore"),
                DEGAS_TRAP_D / 2.0,
                DEGAS_POCKET_Z + 8.0,
                36,
            )
            .translate(x, DEGAS_CENTER.1 + 12.0, BASE_Z + DEGAS_POCKET_Z / 2.0);
        witness_features = witness_features
            + centered_cylinder(
                format!("closed_perfusion_air_microleak_degas_trap_{trap}_witness_ring"),
                DEGAS_TRAP_D / 2.0 + 5.0,
                3.0,
                36,
            )
            .translate(x, DEGAS_CENTER.1 + 12.0, BASE_Z + DEGAS_POCKET_Z + 1.5)
            + centered_cube(
                format!("closed_perfusion_air_microleak_degas_trap_{trap}_high_point_marker"),
                10.0,
                46.0,
                5.0,
            )
            .translate(x, DEGAS_CENTER.1 + 56.0, BASE_Z + DEGAS_POCKET_Z + 2.5)
            + centered_cube(
                format!("closed_perfusion_air_microleak_degas_trap_{trap}_front_bubble_scale_land"),
                32.0,
                4.0,
                22.0,
            )
            .translate(
                x,
                DEGAS_CENTER.1 - DEGAS_POCKET_Y / 2.0 - 8.0,
                BASE_Z + DEGAS_POCKET_Z / 2.0,
            );
    }

    let drain_channel = centered_cube(
        "closed_perfusion_air_microleak_degas_trap_drain_witness_channel",
        DEGAS_POCKET_X - 42.0,
        8.0,
        8.0,
    )
    .translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1 - DEGAS_POCKET_Y / 2.0 + 20.0,
        BASE_Z + 10.0,
    );

    body - trap_cups - witness_window_cut + witness_features + drain_channel
}

fn sterile_connector_bulkhead() -> Part {
    let bulkhead = centered_cube(
        "closed_perfusion_air_microleak_sterile_connector_bulkhead_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1,
        BASE_Z + BULKHEAD_Z / 2.0,
    );
    let mut port_cuts = Part::empty("closed_perfusion_air_microleak_sterile_connector_port_cuts");
    let mut face_features =
        Part::empty("closed_perfusion_air_microleak_sterile_connector_face_features");
    for port in 0..STERILE_CONNECTOR_PORTS {
        let x = connector_x(port);
        port_cuts = port_cuts
            + centered_cylinder(
                format!(
                    "closed_perfusion_air_microleak_sterile_connector_port_{port}_through_bore"
                ),
                CONNECTOR_BORE_D / 2.0,
                BULKHEAD_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER.1, BASE_Z + BULKHEAD_Z / 2.0);

        face_features = face_features
            + connector_gasket_ring(port, x)
            + centered_cube(
                format!("closed_perfusion_air_microleak_sterile_connector_port_{port}_cap_land"),
                26.0,
                6.0,
                12.0,
            )
            .translate(
                x,
                BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 4.0,
                BASE_Z + BULKHEAD_Z / 2.0,
            );
    }

    bulkhead - port_cuts + face_features + sterile_connector_lane_keys()
}

fn connector_gasket_ring(port: usize, x: f64) -> Part {
    let outer = centered_cylinder(
        format!("closed_perfusion_air_microleak_sterile_connector_port_{port}_gasket_ring"),
        CONNECTOR_GASKET_D / 2.0,
        5.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        x,
        BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 2.5,
        BASE_Z + BULKHEAD_Z / 2.0,
    );
    let inner = centered_cylinder(
        format!(
            "closed_perfusion_air_microleak_sterile_connector_port_{port}_gasket_center_clearance"
        ),
        CONNECTOR_BORE_D / 2.0,
        7.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        x,
        BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 2.5,
        BASE_Z + BULKHEAD_Z / 2.0,
    );
    outer - inner
}

fn sterile_connector_lane_keys() -> Part {
    let mut keys = Part::empty("closed_perfusion_air_microleak_sterile_connector_lane_keys");
    for lane in 0..CONNECTOR_POLARITY_KEYS {
        let x = (connector_x(lane * 2) + connector_x(lane * 2 + 1)) / 2.0;
        keys = keys
            + centered_cube(
                format!(
                    "closed_perfusion_air_microleak_sterile_connector_lane_{lane}_polarity_key"
                ),
                8.0,
                BULKHEAD_Y + 8.0,
                20.0,
            )
            .translate(x, BULKHEAD_CENTER.1, BASE_Z + BULKHEAD_Z + 10.0);
    }
    keys
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_perfusion_air_microleak_barcode_certificate_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, BASE_Z + TRACE_PANEL_Z / 2.0);
    let mut lands = Part::empty("closed_perfusion_air_microleak_barcode_certificate_lands");
    for lane in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_air_microleak_lane_{lane}_barcode_land"),
                34.0,
                17.0,
                3.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(lane, BARCODE_LANDS, 36.0),
                TRACE_CENTER.1 - 22.0,
                BASE_Z + TRACE_PANEL_Z + 1.5,
            );
    }
    for cert in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_air_microleak_certificate_land_{cert}"),
                58.0,
                20.0,
                3.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(cert, CERTIFICATE_LANDS, 68.0),
                TRACE_CENTER.1 + 22.0,
                BASE_Z + TRACE_PANEL_Z + 1.5,
            );
    }
    for record in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_air_microleak_run_record_land_{record}"),
                72.0,
                10.0,
                3.0,
            )
            .translate(
                TRACE_CENTER.0 - 86.0 + record as f64 * 86.0,
                TRACE_CENTER.1,
                BASE_Z + TRACE_PANEL_Z + 1.5,
            );
    }

    panel + lands + traceability_guard()
}

fn traceability_guard() -> Part {
    centered_cube(
        "closed_perfusion_air_microleak_traceability_panel_guard_rail",
        TRACE_PANEL_X + 26.0,
        8.0,
        26.0,
    )
    .translate(
        TRACE_CENTER.0,
        TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0 + 8.0,
        BASE_Z + TRACE_PANEL_Z + 13.0,
    )
}

fn release_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "closed_perfusion_air_microleak_release_hold_reject_tray",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1,
        BASE_Z + DISPOSITION_Z / 2.0,
    );
    let mut cuts = Part::empty("closed_perfusion_air_microleak_release_hold_reject_slot_cuts");
    let mut labels = Part::empty("closed_perfusion_air_microleak_release_hold_reject_labels");
    for (lane, name) in STATUS_LANE_NAMES.iter().enumerate() {
        let y = disposition_lane_y(lane);
        labels = labels
            + centered_cube(
                format!("closed_perfusion_air_microleak_{name}_lane_header_land"),
                78.0,
                20.0,
                4.0,
            )
            .translate(
                DISPOSITION_CENTER.0 - DISPOSITION_X / 2.0 + 54.0,
                y,
                BASE_Z + DISPOSITION_Z + 2.0,
            );
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("closed_perfusion_air_microleak_{name}_lane_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    14.0,
                )
                .translate(
                    DISPOSITION_CENTER.0 - 84.0 + slot as f64 * 58.0,
                    y,
                    BASE_Z + DISPOSITION_Z - 6.5,
                );
        }
    }
    let hold_divider = centered_cube(
        "closed_perfusion_air_microleak_hold_lane_physical_divider",
        DISPOSITION_X - 34.0,
        5.0,
        30.0,
    )
    .translate(
        DISPOSITION_CENTER.0,
        (disposition_lane_y(0) + disposition_lane_y(1)) / 2.0,
        BASE_Z + DISPOSITION_Z + 15.0,
    );
    let reject_divider = centered_cube(
        "closed_perfusion_air_microleak_reject_lane_physical_divider",
        DISPOSITION_X - 34.0,
        5.0,
        30.0,
    )
    .translate(
        DISPOSITION_CENTER.0,
        (disposition_lane_y(1) + disposition_lane_y(2)) / 2.0,
        BASE_Z + DISPOSITION_Z + 15.0,
    );

    tray - cuts + labels + hold_divider + reject_divider
}

fn robot_service_keepout_gauges() -> Part {
    let robot_sweep = keepout_frame(
        "closed_perfusion_air_microleak_robot_sweep_keepout_gauge",
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z + ROBOT_SWEEP_Z);
    let front_service = centered_cube(
        "closed_perfusion_air_microleak_front_service_keepout_gauge",
        ROBOT_SWEEP_X,
        8.0,
        64.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        BASE_Z + 32.0,
    );
    let rear_connector_service = centered_cube(
        "closed_perfusion_air_microleak_rear_connector_service_keepout_gauge",
        BULKHEAD_X + 80.0,
        8.0,
        74.0,
    )
    .translate(
        BULKHEAD_CENTER.0,
        STATION_Y / 2.0 + REAR_CONNECTOR_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 37.0,
    );
    let coupon_tweezer_sweep = keepout_frame(
        "closed_perfusion_air_microleak_coupon_tweezer_sweep_keepout",
        COUPON_BANK_X + 62.0,
        COUPON_BANK_Y + 72.0,
        7.0,
    )
    .translate(
        COUPON_BANK_CENTER.0,
        COUPON_BANK_CENTER.1,
        BASE_Z + COUPON_BANK_Z + COUPON_TWEEZER_CLEARANCE,
    );
    let degas_lift = keepout_frame(
        "closed_perfusion_air_microleak_degas_trap_lift_keepout",
        DEGAS_POCKET_X + 54.0,
        DEGAS_POCKET_Y + 54.0,
        7.0,
    )
    .translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1,
        BASE_Z + DEGAS_POCKET_Z + DEGAS_LIFT_CLEARANCE,
    );
    let optical_camera = keepout_frame(
        "closed_perfusion_air_microleak_optical_camera_keepout",
        OPTICAL_BRIDGE_X - 84.0,
        OPTICAL_BRIDGE_Y + 52.0,
        7.0,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0,
        OPTICAL_BRIDGE_CENTER.1,
        BASE_Z + OPTICAL_CAMERA_CLEARANCE,
    );
    let pressure_service = centered_cube(
        "closed_perfusion_air_microleak_pressure_panel_service_height_gauge",
        PRESSURE_PANEL_X + 54.0,
        PRESSURE_PANEL_Y + 44.0,
        8.0,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1,
        BASE_Z + PRESSURE_PANEL_SERVICE_CLEARANCE,
    );

    robot_sweep
        + front_service
        + rear_connector_service
        + coupon_tweezer_sweep
        + degas_lift
        + optical_camera
        + pressure_service
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);

    front + rear + left + right
}

fn mount_points() -> [(f64, f64); 6] {
    [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
}

fn module_footprints() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        (
            "closed_tubing_lane_combs",
            COMB_CENTER,
            COMB_DECK_X,
            COMB_DECK_Y,
        ),
        (
            "microleak_coupon_placeholders",
            COUPON_BANK_CENTER,
            COUPON_BANK_X,
            COUPON_BANK_Y + 42.0,
        ),
        (
            "optical_bubble_witness_windows",
            OPTICAL_BRIDGE_CENTER,
            OPTICAL_BRIDGE_X,
            OPTICAL_BRIDGE_Y,
        ),
        (
            "pressure_decay_reference_ports",
            PRESSURE_PANEL_CENTER,
            PRESSURE_PANEL_X,
            PRESSURE_PANEL_Y,
        ),
        (
            "degas_trap_witness_pocket",
            DEGAS_CENTER,
            DEGAS_POCKET_X,
            DEGAS_POCKET_Y,
        ),
        (
            "sterile_connector_bulkhead",
            BULKHEAD_CENTER,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        (
            "barcode_certificate_lands",
            TRACE_CENTER,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            DISPOSITION_CENTER,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
    ]
}

fn fits_inside_tray(center: (f64, f64), width: f64, depth: f64) -> bool {
    let usable_half_x = STATION_X / 2.0 - RIM_W - 14.0;
    let usable_half_y = STATION_Y / 2.0 - RIM_W - 14.0;
    center.0.abs() + width / 2.0 <= usable_half_x && center.1.abs() + depth / 2.0 <= usable_half_y
}

fn lane_center(lane: usize) -> f64 {
    centered_index(lane, LANES, LANE_PITCH_X)
}

fn coupon_x(coupon: usize) -> f64 {
    COUPON_BANK_CENTER.0 + centered_index(coupon, MICROLEAK_COUPONS, COUPON_PITCH_X)
}

fn pressure_lane_x(lane: usize) -> f64 {
    PRESSURE_PANEL_CENTER.0 + centered_index(lane, PRESSURE_PORTS, PRESSURE_PORT_PITCH_X)
}

fn reference_port_x(port: usize) -> f64 {
    PRESSURE_PANEL_CENTER.0 + centered_index(port, REFERENCE_PORTS, 70.0)
}

fn degas_trap_x(trap: usize) -> f64 {
    DEGAS_CENTER.0 + centered_index(trap, DEGAS_TRAPS, DEGAS_TRAP_PITCH_X)
}

fn connector_x(port: usize) -> f64 {
    BULKHEAD_CENTER.0 + centered_index(port, STERILE_CONNECTOR_PORTS, CONNECTOR_PITCH_X)
}

fn disposition_lane_y(lane: usize) -> f64 {
    DISPOSITION_CENTER.1 + centered_index(lane, STATUS_LANES, 36.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(INLET_OUTLET_PORTS, LANES * 2);
    assert_eq!(MICROLEAK_COUPONS, LANES);
    assert_eq!(BUBBLE_WINDOWS, LANES);
    assert_eq!(ILLUMINATOR_LANDS, BUBBLE_WINDOWS);
    assert_eq!(PRESSURE_PORTS, LANES);
    assert_eq!(PRESSURE_TRANSDUCER_DOCKS, LANES);
    assert_eq!(REFERENCE_VOLUME_BLOCKS, REFERENCE_PORTS);
    assert_eq!(DEGAS_TRAP_WITNESS_WINDOWS, DEGAS_TRAPS);
    assert_eq!(DEGAS_HIGH_POINT_MARKERS, DEGAS_TRAPS);
    assert_eq!(STERILE_CONNECTOR_PORTS, LANES * 2);
    assert_eq!(CONNECTOR_POLARITY_KEYS, LANES);
    assert_eq!(BARCODE_LANDS, LANES);
    assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= MICROLEAK_COUPONS);
    assert!(TUBE_CHANNEL_D > TUBE_OD_MAX);
    assert!(FRONT_SERVICE_CLEARANCE >= 220.0);
    assert!(REAR_CONNECTOR_SERVICE_CLEARANCE >= 180.0);
    assert!(COUPON_TWEEZER_CLEARANCE >= 140.0);
    assert!(DEGAS_LIFT_CLEARANCE >= 180.0);
    assert!(OPTICAL_CAMERA_CLEARANCE >= 220.0);
    assert!(PRESSURE_PANEL_SERVICE_CLEARANCE >= 150.0);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.ends_with(".stl"), "{path}");
    }

    for (name, center, width, depth) in module_footprints() {
        assert!(
            fits_inside_tray(center, width, depth),
            "{name} exceeds containment tray"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_unique_scoped_and_complete() {
        assert_layout();
        assert!(OUTPUTS[0].contains("base_leak_tray"));
        assert!(OUTPUTS[1].contains("closed_tubing_lane_combs"));
        assert!(OUTPUTS[2].contains("microleak_coupon_placeholders"));
        assert!(OUTPUTS[3].contains("optical_bubble_witness_windows"));
        assert!(OUTPUTS[4].contains("pressure_decay_reference_ports"));
        assert!(OUTPUTS[10].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_groups_match_worker_scope() {
        for feature in [
            "leak_tray",
            "closed_tubing_lane_combs",
            "microleak_coupon_placeholders",
            "optical_bubble_witness_windows",
            "pressure_decay_reference_ports",
            "degas_trap_witness_pocket",
            "sterile_connector_bulkhead",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn closed_tubing_lane_topology_is_consistent() {
        assert_eq!(LANES, 8);
        assert_eq!(INLET_OUTLET_PORTS, 16);
        assert_eq!(STERILE_CONNECTOR_PORTS, INLET_OUTLET_PORTS);
        assert_eq!(MICROLEAK_COUPONS, LANES);
        assert!(TUBE_CHANNEL_D > TUBE_OD_MAX);
        assert!(LANE_PITCH_X > WINDOW_X);
    }

    #[test]
    fn microleak_pressure_and_disposition_capacity_match_lanes() {
        assert_eq!(PRESSURE_PORTS, LANES);
        assert_eq!(PRESSURE_TRANSDUCER_DOCKS, LANES);
        assert_eq!(REFERENCE_PORTS, 4);
        assert_eq!(REFERENCE_VOLUME_BLOCKS, REFERENCE_PORTS);
        assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= MICROLEAK_COUPONS);
    }

    #[test]
    fn optical_and_degas_witness_counts_are_explicit() {
        assert_eq!(BUBBLE_WINDOWS, LANES);
        assert_eq!(ILLUMINATOR_LANDS, BUBBLE_WINDOWS);
        assert_eq!(DEGAS_TRAPS, 4);
        assert_eq!(DEGAS_TRAP_WITNESS_WINDOWS, DEGAS_TRAPS);
        assert_eq!(DEGAS_HIGH_POINT_MARKERS, DEGAS_TRAPS);
        assert!(DEGAS_TRAP_D > CONNECTOR_BORE_D);
    }

    #[test]
    fn all_fixture_modules_fit_inside_leak_tray() {
        for (_name, center, width, depth) in module_footprints() {
            assert!(fits_inside_tray(center, width, depth));
        }
        assert!(COMB_DECK_X < STATION_X - 2.0 * RIM_W);
        assert!(BULKHEAD_X < STATION_X - 2.0 * RIM_W);
    }

    #[test]
    fn traceability_and_keepout_intent_are_declared() {
        assert_eq!(BARCODE_LANDS, LANES);
        assert_eq!(CERTIFICATE_LANDS, 4);
        assert_eq!(RUN_RECORD_LANDS, STATUS_LANES);
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(FRONT_SERVICE_CLEARANCE >= 220.0);
        assert!(OPTICAL_CAMERA_CLEARANCE >= 220.0);
    }

    #[test]
    fn limitation_markers_prevent_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
        assert!(LIMITATIONS.contains(&"not_pressure_rated_system_design"));
        assert!(LIMITATIONS.contains(&"not_leak_acceptance_protocol"));
        assert!(LIMITATIONS.contains(&"purchased_sensors_connectors_coupons_as_placeholders"));
        assert!(LIMITATIONS.contains(&"sterility_and_wetted_materials_external"));
    }
}
