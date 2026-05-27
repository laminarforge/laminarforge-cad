use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system media additive precipitation/filter-blockage validation station.
//
// Purpose:
// - Challenge automated media-additive addition through cold/warm mixing,
//   inline filter witness, optical turbidity window, pressure-drop witness path,
//   settled-particle trap, and release/hold/reject evidence gates.
// - Keep all evidence interfaces closed-path friendly: bag/tubing routing,
//   lot ID capture, retained witness coupons, and segregated disposition lanes.
//
// This is interface and packaging CAD for purchased filters, optical sensors,
// pressure sensors, valves, pumps, and disposable closed-path coupons. It is not
// a validated precipitation assay, sterile barrier definition, or acceptance
// criterion.
//
// Exports:
//   output/closed_media_additive_precipitation_filter_blockage_station_deck.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_additive_inlet_bank.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_cold_warm_mixing_coupon.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_inline_filter_witness.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_optical_turbidity_window.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_pressure_drop_witness_path.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_settled_particle_trap.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_release_hold_reject_gates.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_evidence_capture_panel.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_tubing_route_witness.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_robot_service_keepouts.stl
//   output/closed_media_additive_precipitation_filter_blockage_station_assembly.stl

const OUTPUT_PREFIX: &str = "output/closed_media_additive_precipitation_filter_blockage_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_media_additive_precipitation_filter_blockage_station_deck.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_additive_inlet_bank.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_cold_warm_mixing_coupon.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_inline_filter_witness.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_optical_turbidity_window.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_pressure_drop_witness_path.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_settled_particle_trap.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_release_hold_reject_gates.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_evidence_capture_panel.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_tubing_route_witness.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_robot_service_keepouts.stl",
    "output/closed_media_additive_precipitation_filter_blockage_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "closed_additive_inlet_bank",
    "cold_warm_mixing_coupon",
    "thermal_zone_probe_wells",
    "inline_filter_witness",
    "upstream_downstream_filter_ports",
    "optical_turbidity_window",
    "calibration_blank_and_haze_lands",
    "pressure_drop_witness_path",
    "paired_pressure_taps",
    "settled_particle_trap",
    "release_hold_reject_evidence_gates",
    "lot_barcode_rfid_capture",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1160.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_D: f64 = 10.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const TUBE_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const SENSOR_TAP_D: f64 = 3.2;
const HIGH_FLOW_BORE_D: f64 = 7.0;

const ADDITIVE_BANK_X: f64 = 270.0;
const ADDITIVE_BANK_Y: f64 = 186.0;
const ADDITIVE_BANK_Z: f64 = 64.0;
const ADDITIVE_CENTER_X: f64 = -438.0;
const ADDITIVE_CENTER_Y: f64 = 224.0;
const ADDITIVE_PORTS: usize = 5;
const ADDITIVE_PORT_PITCH_X: f64 = 42.0;
const ADDITIVE_BAG_PADS: usize = 3;

const MIX_COUPON_X: f64 = 360.0;
const MIX_COUPON_Y: f64 = 214.0;
const MIX_COUPON_Z: f64 = 52.0;
const MIX_CENTER_X: f64 = -165.0;
const MIX_CENTER_Y: f64 = 130.0;
const THERMAL_ZONES: usize = 2;
const MIXING_LANES: usize = 4;
const MIX_LANE_PITCH_Y: f64 = 34.0;
const MIX_SERPENTINE_SEGMENTS: usize = 7;
const MIX_PROBE_WELLS: usize = 6;

const FILTER_WITNESS_X: f64 = 300.0;
const FILTER_WITNESS_Y: f64 = 154.0;
const FILTER_WITNESS_Z: f64 = 62.0;
const FILTER_CENTER_X: f64 = 160.0;
const FILTER_CENTER_Y: f64 = 194.0;
const FILTER_MEMBRANE_D: f64 = 50.0;
const FILTER_WITNESS_COUNT: usize = 2;
const FILTER_SAMPLE_PORTS: usize = 4;

const TURBIDITY_X: f64 = 268.0;
const TURBIDITY_Y: f64 = 162.0;
const TURBIDITY_Z: f64 = 72.0;
const TURBIDITY_CENTER_X: f64 = 420.0;
const TURBIDITY_CENTER_Y: f64 = 56.0;
const TURBIDITY_WINDOWS: usize = 3;
const OPTICAL_PATH_LENGTH: f64 = 22.0;
const CALIBRATION_LANDS: usize = 3;

const PRESSURE_PATH_X: f64 = 485.0;
const PRESSURE_PATH_Y: f64 = 164.0;
const PRESSURE_PATH_Z: f64 = 50.0;
const PRESSURE_CENTER_X: f64 = -88.0;
const PRESSURE_CENTER_Y: f64 = -68.0;
const PRESSURE_TAPS: usize = 4;
const RESTRICTOR_COUPONS: usize = 3;
const PRESSURE_TAP_PITCH_X: f64 = 96.0;

const PARTICLE_TRAP_X: f64 = 248.0;
const PARTICLE_TRAP_Y: f64 = 204.0;
const PARTICLE_TRAP_Z: f64 = 70.0;
const PARTICLE_CENTER_X: f64 = 292.0;
const PARTICLE_CENTER_Y: f64 = -184.0;
const SETTLE_WELLS: usize = 4;
const SETTLE_WELL_D: f64 = 31.0;
const MICROSCOPY_WINDOWS: usize = 3;

const GATE_PANEL_X: f64 = 350.0;
const GATE_PANEL_Y: f64 = 254.0;
const GATE_PANEL_Z: f64 = 36.0;
const GATE_CENTER_X: f64 = -360.0;
const GATE_CENTER_Y: f64 = -214.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_PITCH_Y: f64 = 74.0;
const RETAIN_WELLS_PER_LANE: usize = 4;

const EVIDENCE_PANEL_X: f64 = 316.0;
const EVIDENCE_PANEL_Y: f64 = 118.0;
const EVIDENCE_PANEL_Z: f64 = 150.0;
const EVIDENCE_CENTER_X: f64 = 416.0;
const EVIDENCE_CENTER_Y: f64 = 250.0;
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;

const ROUTE_WITNESS_Z: f64 = 26.0;
const ROUTE_CLIPS: usize = 11;
const ROBOT_KEEP_OUTS: usize = 4;
const ROBOT_PICK_CLEARANCE_Z: f64 = 156.0;
const SERVICE_CLEARANCE_Z: f64 = 126.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    write_part(station_deck(), OUTPUTS[0]);
    write_part(additive_inlet_bank(), OUTPUTS[1]);
    write_part(cold_warm_mixing_coupon(), OUTPUTS[2]);
    write_part(inline_filter_witness(), OUTPUTS[3]);
    write_part(optical_turbidity_window(), OUTPUTS[4]);
    write_part(pressure_drop_witness_path(), OUTPUTS[5]);
    write_part(settled_particle_trap(), OUTPUTS[6]);
    write_part(release_hold_reject_gates(), OUTPUTS[7]);
    write_part(evidence_capture_panel(), OUTPUTS[8]);
    write_part(tubing_route_witness(), OUTPUTS[9]);
    write_part(robot_service_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed media additive precipitation/filter-blockage station: {:.0}mm x {:.0}mm deck, {} additive ports, {} thermal zones, {} mixing lanes, {} inline filter witnesses, {} turbidity windows, {} pressure taps, {} restrictor coupons, {} settled-particle wells, {} status lanes, {} evidence feature groups, and {} keepout envelopes.",
        DECK_X,
        DECK_Y,
        ADDITIVE_PORTS,
        THERMAL_ZONES,
        MIXING_LANES,
        FILTER_WITNESS_COUNT,
        TURBIDITY_WINDOWS,
        PRESSURE_TAPS,
        RESTRICTOR_COUPONS,
        SETTLE_WELLS,
        STATUS_LANES,
        REQUIRED_FEATURES.len(),
        ROBOT_KEEP_OUTS
    );
    println!(
        "Witness tubing placeholders use {:.1}mm OD with {:.1}mm bore clearance; turbidity path length is {:.0}mm; outputs use prefix {OUTPUT_PREFIX}.",
        TUBE_OD, TUBE_BORE_D, OPTICAL_PATH_LENGTH
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    station_deck()
        + additive_inlet_bank()
        + cold_warm_mixing_coupon()
        + inline_filter_witness()
        + optical_turbidity_window()
        + pressure_drop_witness_path()
        + settled_particle_trap()
        + release_hold_reject_gates()
        + evidence_capture_panel()
        + tubing_route_witness()
        + robot_service_keepouts()
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "media_additive_precip_filter_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let wet_zone_recess = centered_cube(
        "media_additive_precip_filter_station_wet_zone_recess",
        DECK_X - 126.0,
        420.0,
        7.0,
    )
    .translate(18.0, 26.0, DECK_Z / 2.0 - 2.8);
    let evidence_recess = centered_cube(
        "media_additive_precip_filter_station_gate_recess",
        GATE_PANEL_X + 38.0,
        GATE_PANEL_Y + 38.0,
        7.0,
    )
    .translate(GATE_CENTER_X, GATE_CENTER_Y, DECK_Z / 2.0 - 2.8);
    let particle_trap_sump = centered_cube(
        "media_additive_precip_filter_station_particle_trap_sump",
        PARTICLE_TRAP_X + 50.0,
        PARTICLE_TRAP_Y + 48.0,
        8.0,
    )
    .translate(PARTICLE_CENTER_X, PARTICLE_CENTER_Y, DECK_Z / 2.0 - 3.2);
    let drain = centered_cylinder(
        "media_additive_precip_filter_station_sump_drain",
        DRAIN_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 78.0, -DECK_Y / 2.0 - 1.0, -1.0);

    deck - wet_zone_recess - evidence_recess - particle_trap_sump - drain - deck_mount_slots()
        + deck_perimeter_rails()
        + component_registration_bosses()
        + robot_fiducials()
}

fn deck_perimeter_rails() -> Part {
    let left = centered_cube(
        "media_additive_precip_filter_station_left_rail",
        18.0,
        DECK_Y - 72.0,
        38.0,
    )
    .translate(-(DECK_X / 2.0 - 34.0), 0.0, DECK_Z / 2.0 + 19.0);
    let right = centered_cube(
        "media_additive_precip_filter_station_right_rail",
        18.0,
        DECK_Y - 72.0,
        38.0,
    )
    .translate(DECK_X / 2.0 - 34.0, 0.0, DECK_Z / 2.0 + 19.0);
    let rear = centered_cube(
        "media_additive_precip_filter_station_rear_rail",
        DECK_X - 72.0,
        18.0,
        38.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 34.0, DECK_Z / 2.0 + 19.0);
    let front_lip = centered_cube(
        "media_additive_precip_filter_station_front_containment_lip",
        DECK_X - 126.0,
        14.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 32.0, DECK_Z / 2.0 + 12.0);

    left + right + rear + front_lip
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("media_additive_precip_filter_station_deck_mount_slots");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("media_additive_precip_filter_station_m6_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 5.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("media_additive_precip_filter_station_m6_slot_{i}"),
            28.0,
            MOUNT_HOLE_D + 0.3,
            DECK_Z + 5.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn component_registration_bosses() -> Part {
    let mut bosses = Part::empty("media_additive_precip_filter_station_registration_bosses");
    for (i, (x, y)) in component_centers().iter().enumerate() {
        let boss = centered_cylinder(
            format!("media_additive_precip_filter_station_registration_boss_{i}"),
            10.0,
            8.0,
            32,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        let pin_bore = centered_cylinder(
            format!("media_additive_precip_filter_station_registration_pin_bore_{i}"),
            2.7,
            10.0,
            20,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        bosses = bosses + (boss - pin_bore);
    }
    bosses
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("media_additive_precip_filter_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 72.0), DECK_Y / 2.0 - 72.0),
        (DECK_X / 2.0 - 72.0, DECK_Y / 2.0 - 72.0),
        (-(DECK_X / 2.0 - 72.0), -(DECK_Y / 2.0 - 72.0)),
        (DECK_X / 2.0 - 72.0, -(DECK_Y / 2.0 - 72.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("media_additive_precip_filter_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn additive_inlet_bank() -> Part {
    let body = centered_cube(
        "media_additive_precip_filter_additive_inlet_bank_body",
        ADDITIVE_BANK_X,
        ADDITIVE_BANK_Y,
        ADDITIVE_BANK_Z,
    );
    let top_recess = centered_cube(
        "media_additive_precip_filter_additive_inlet_bank_source_recess",
        ADDITIVE_BANK_X - 34.0,
        ADDITIVE_BANK_Y - 44.0,
        12.0,
    )
    .translate(0.0, 8.0, ADDITIVE_BANK_Z / 2.0 - 5.0);

    let mut port_bores = Part::empty("media_additive_precip_filter_additive_port_bores");
    for port in 0..ADDITIVE_PORTS {
        let x = additive_port_x(port);
        let bore = centered_cylinder(
            format!("media_additive_precip_filter_additive_port_bore_{port}"),
            TUBE_BORE_D / 2.0,
            ADDITIVE_BANK_Z + 4.0,
            24,
        )
        .translate(x, -ADDITIVE_BANK_Y / 2.0 + 34.0, 0.0);
        let luer_socket = centered_cylinder(
            format!("media_additive_precip_filter_additive_luer_socket_{port}"),
            11.0 / 2.0,
            18.0,
            28,
        )
        .translate(
            x,
            -ADDITIVE_BANK_Y / 2.0 + 34.0,
            ADDITIVE_BANK_Z / 2.0 - 7.0,
        );
        port_bores = port_bores + bore + luer_socket;
    }

    let mut bag_pads = Part::empty("media_additive_precip_filter_additive_bag_pads");
    for pad in 0..ADDITIVE_BAG_PADS {
        let x = (pad as f64 - (ADDITIVE_BAG_PADS as f64 - 1.0) / 2.0) * 78.0;
        bag_pads = bag_pads
            + centered_cube(
                format!("media_additive_precip_filter_source_bag_pad_{pad}"),
                58.0,
                78.0,
                8.0,
            )
            .translate(x, 36.0, ADDITIVE_BANK_Z / 2.0 + 4.0)
            + centered_cube(
                format!("media_additive_precip_filter_source_bag_label_land_{pad}"),
                46.0,
                18.0,
                3.0,
            )
            .translate(x, 82.0, ADDITIVE_BANK_Z / 2.0 + 9.5);
    }

    let outlet_header = tube_run_x(
        "media_additive_precip_filter_additive_outlet_header",
        additive_port_x(0),
        additive_port_x(ADDITIVE_PORTS - 1),
        -ADDITIVE_BANK_Y / 2.0 + 34.0,
        0.0,
        HIGH_FLOW_BORE_D,
    );

    (body - top_recess - port_bores - outlet_header
        + bag_pads
        + latch_ears(
            "additive_bank",
            ADDITIVE_BANK_X,
            ADDITIVE_BANK_Y,
            ADDITIVE_BANK_Z / 2.0,
        ))
    .translate(
        ADDITIVE_CENTER_X,
        ADDITIVE_CENTER_Y,
        DECK_Z / 2.0 + ADDITIVE_BANK_Z / 2.0,
    )
}

fn cold_warm_mixing_coupon() -> Part {
    let body = centered_cube(
        "media_additive_precip_filter_cold_warm_mixing_coupon_body",
        MIX_COUPON_X,
        MIX_COUPON_Y,
        MIX_COUPON_Z,
    );
    let isolation_trench = centered_cube(
        "media_additive_precip_filter_cold_warm_thermal_isolation_trench",
        14.0,
        MIX_COUPON_Y - 30.0,
        MIX_COUPON_Z + 3.0,
    );
    let cold_pocket = centered_cube(
        "media_additive_precip_filter_cold_zone_peltier_pocket",
        MIX_COUPON_X / 2.0 - 42.0,
        MIX_COUPON_Y - 48.0,
        12.0,
    )
    .translate(-(MIX_COUPON_X / 4.0 + 10.0), 0.0, -MIX_COUPON_Z / 2.0 + 4.0);
    let warm_pocket = centered_cube(
        "media_additive_precip_filter_warm_zone_heater_pocket",
        MIX_COUPON_X / 2.0 - 42.0,
        MIX_COUPON_Y - 48.0,
        12.0,
    )
    .translate(MIX_COUPON_X / 4.0 + 10.0, 0.0, -MIX_COUPON_Z / 2.0 + 4.0);

    let mut channels = Part::empty("media_additive_precip_filter_mixing_coupon_flow_channels");
    for lane in 0..MIXING_LANES {
        let y = mix_lane_y(lane);
        channels = channels
            + tube_run_x(
                &format!("media_additive_precip_filter_mix_lane_{lane}_cold_leg"),
                -(MIX_COUPON_X / 2.0 - 34.0),
                -24.0,
                y,
                5.0,
                TUBE_BORE_D,
            )
            + tube_run_x(
                &format!("media_additive_precip_filter_mix_lane_{lane}_warm_leg"),
                24.0,
                MIX_COUPON_X / 2.0 - 34.0,
                y,
                5.0,
                TUBE_BORE_D,
            )
            + centered_cylinder(
                format!("media_additive_precip_filter_mix_lane_{lane}_transition_loop"),
                18.0,
                TUBE_BORE_D,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(0.0, y, 5.0);
    }

    let mut serpentine_ridges = Part::empty("media_additive_precip_filter_mixing_coupon_ridges");
    for segment in 0..MIX_SERPENTINE_SEGMENTS {
        let x = -132.0 + segment as f64 * 44.0;
        serpentine_ridges = serpentine_ridges
            + centered_cube(
                format!("media_additive_precip_filter_serpentine_baffle_{segment}"),
                10.0,
                132.0,
                14.0,
            )
            .translate(x, 0.0, MIX_COUPON_Z / 2.0 + 7.0);
    }

    let mut probe_wells = Part::empty("media_additive_precip_filter_mixing_coupon_probe_wells");
    for well in 0..MIX_PROBE_WELLS {
        let x = if well < MIX_PROBE_WELLS / 2 {
            -92.0 + well as f64 * 42.0
        } else {
            8.0 + (well - MIX_PROBE_WELLS / 2) as f64 * 42.0
        };
        let y = if well % 2 == 0 { 80.0 } else { -80.0 };
        probe_wells = probe_wells
            + centered_cylinder(
                format!("media_additive_precip_filter_temperature_probe_well_{well}"),
                5.5,
                MIX_COUPON_Z + 4.0,
                24,
            )
            .translate(x, y, 0.0);
    }

    let zone_labels = centered_cube(
        "media_additive_precip_filter_cold_zone_label_land",
        94.0,
        20.0,
        3.0,
    )
    .translate(-96.0, MIX_COUPON_Y / 2.0 - 22.0, MIX_COUPON_Z / 2.0 + 1.5)
        + centered_cube(
            "media_additive_precip_filter_warm_zone_label_land",
            94.0,
            20.0,
            3.0,
        )
        .translate(96.0, MIX_COUPON_Y / 2.0 - 22.0, MIX_COUPON_Z / 2.0 + 1.5);

    (body - isolation_trench - cold_pocket - warm_pocket - channels - probe_wells
        + serpentine_ridges
        + zone_labels)
        .translate(
            MIX_CENTER_X,
            MIX_CENTER_Y,
            DECK_Z / 2.0 + MIX_COUPON_Z / 2.0,
        )
}

fn inline_filter_witness() -> Part {
    let body = centered_cube(
        "media_additive_precip_filter_inline_filter_witness_body",
        FILTER_WITNESS_X,
        FILTER_WITNESS_Y,
        FILTER_WITNESS_Z,
    );
    let service_window = centered_cube(
        "media_additive_precip_filter_inline_filter_service_window",
        FILTER_WITNESS_X - 46.0,
        FILTER_WITNESS_Y - 44.0,
        16.0,
    )
    .translate(0.0, 0.0, FILTER_WITNESS_Z / 2.0 - 7.0);

    let mut filter_bores = Part::empty("media_additive_precip_filter_filter_witness_bores");
    for filter in 0..FILTER_WITNESS_COUNT {
        let x = filter_witness_x(filter);
        filter_bores = filter_bores
            + centered_cylinder(
                format!("media_additive_precip_filter_membrane_disc_pocket_{filter}"),
                FILTER_MEMBRANE_D / 2.0,
                18.0,
                56,
            )
            .translate(x, 0.0, FILTER_WITNESS_Z / 2.0 - 8.0)
            + centered_cylinder(
                format!("media_additive_precip_filter_membrane_flow_bore_{filter}"),
                HIGH_FLOW_BORE_D / 2.0,
                FILTER_WITNESS_Z + 6.0,
                28,
            )
            .translate(x, 0.0, 0.0);
    }

    let mut sample_ports = Part::empty("media_additive_precip_filter_filter_sample_ports");
    for port in 0..FILTER_SAMPLE_PORTS {
        let x = filter_sample_port_x(port);
        let y = if port % 2 == 0 { -48.0 } else { 48.0 };
        sample_ports = sample_ports
            + centered_cylinder(
                format!("media_additive_precip_filter_up_downstream_sample_port_{port}"),
                4.8,
                FILTER_WITNESS_Z + 6.0,
                24,
            )
            .translate(x, y, 0.0);
    }

    let witness_windows = centered_cube(
        "media_additive_precip_filter_upstream_filter_color_witness_land",
        86.0,
        18.0,
        4.0,
    )
    .translate(
        -74.0,
        FILTER_WITNESS_Y / 2.0 - 22.0,
        FILTER_WITNESS_Z / 2.0 + 2.0,
    ) + centered_cube(
        "media_additive_precip_filter_downstream_filter_color_witness_land",
        86.0,
        18.0,
        4.0,
    )
    .translate(
        74.0,
        FILTER_WITNESS_Y / 2.0 - 22.0,
        FILTER_WITNESS_Z / 2.0 + 2.0,
    );

    let header = tube_run_x(
        "media_additive_precip_filter_inline_filter_flow_header",
        -(FILTER_WITNESS_X / 2.0 - 30.0),
        FILTER_WITNESS_X / 2.0 - 30.0,
        0.0,
        0.0,
        HIGH_FLOW_BORE_D,
    );

    (body - service_window - filter_bores - sample_ports - header
        + witness_windows
        + latch_ears(
            "filter_witness",
            FILTER_WITNESS_X,
            FILTER_WITNESS_Y,
            FILTER_WITNESS_Z / 2.0,
        ))
    .translate(
        FILTER_CENTER_X,
        FILTER_CENTER_Y,
        DECK_Z / 2.0 + FILTER_WITNESS_Z / 2.0,
    )
}

fn optical_turbidity_window() -> Part {
    let base = centered_cube(
        "media_additive_precip_filter_optical_turbidity_base",
        TURBIDITY_X,
        TURBIDITY_Y,
        TURBIDITY_Z,
    );
    let cuvette_slot = centered_cube(
        "media_additive_precip_filter_turbidity_cuvette_slot",
        154.0,
        OPTICAL_PATH_LENGTH,
        TURBIDITY_Z + 4.0,
    )
    .translate(-12.0, 0.0, 0.0);
    let optical_path = centered_cylinder(
        "media_additive_precip_filter_turbidity_optical_path_bore",
        8.0,
        TURBIDITY_Y + 6.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-12.0, 0.0, 10.0);

    let mut windows = Part::empty("media_additive_precip_filter_turbidity_window_lands");
    for window in 0..TURBIDITY_WINDOWS {
        windows = windows
            + centered_cube(
                format!("media_additive_precip_filter_turbidity_window_{window}"),
                34.0,
                5.0,
                28.0,
            )
            .translate(turbidity_window_x(window), -TURBIDITY_Y / 2.0 - 2.0, 12.0)
            + centered_cube(
                format!("media_additive_precip_filter_turbidity_detector_window_{window}"),
                34.0,
                5.0,
                28.0,
            )
            .translate(turbidity_window_x(window), TURBIDITY_Y / 2.0 + 2.0, 12.0);
    }

    let led_tower = sensor_tower(
        "media_additive_precip_filter_turbidity_led_tower",
        42.0,
        32.0,
        96.0,
    )
    .translate(-118.0, -TURBIDITY_Y / 2.0 - 22.0, TURBIDITY_Z / 2.0 + 48.0);
    let detector_tower = sensor_tower(
        "media_additive_precip_filter_turbidity_detector_tower",
        42.0,
        32.0,
        96.0,
    )
    .translate(-118.0, TURBIDITY_Y / 2.0 + 22.0, TURBIDITY_Z / 2.0 + 48.0);

    let mut calibration_lands = Part::empty("media_additive_precip_filter_turbidity_cal_lands");
    for land in 0..CALIBRATION_LANDS {
        calibration_lands = calibration_lands
            + centered_cube(
                format!("media_additive_precip_filter_turbidity_calibration_land_{land}"),
                54.0,
                30.0,
                6.0,
            )
            .translate(82.0, (land as f64 - 1.0) * 42.0, TURBIDITY_Z / 2.0 + 3.0);
    }

    (base - cuvette_slot - optical_path + windows + led_tower + detector_tower + calibration_lands)
        .translate(
            TURBIDITY_CENTER_X,
            TURBIDITY_CENTER_Y,
            DECK_Z / 2.0 + TURBIDITY_Z / 2.0,
        )
}

fn pressure_drop_witness_path() -> Part {
    let body = centered_cube(
        "media_additive_precip_filter_pressure_drop_witness_body",
        PRESSURE_PATH_X,
        PRESSURE_PATH_Y,
        PRESSURE_PATH_Z,
    );
    let route_channel = tube_run_x(
        "media_additive_precip_filter_pressure_drop_main_flow_path",
        -(PRESSURE_PATH_X / 2.0 - 38.0),
        PRESSURE_PATH_X / 2.0 - 38.0,
        0.0,
        0.0,
        HIGH_FLOW_BORE_D,
    );

    let mut pressure_taps = Part::empty("media_additive_precip_filter_pressure_drop_taps");
    for tap in 0..PRESSURE_TAPS {
        pressure_taps = pressure_taps
            + centered_cylinder(
                format!("media_additive_precip_filter_pressure_drop_tap_{tap}"),
                SENSOR_TAP_D / 2.0,
                PRESSURE_PATH_Z + 5.0,
                18,
            )
            .translate(pressure_tap_x(tap), -36.0, 0.0)
            + centered_cube(
                format!("media_additive_precip_filter_pressure_sensor_pocket_{tap}"),
                44.0,
                28.0,
                12.0,
            )
            .translate(
                pressure_tap_x(tap),
                -PRESSURE_PATH_Y / 2.0 + 28.0,
                PRESSURE_PATH_Z / 2.0 - 5.0,
            );
    }

    let mut coupons = Part::empty("media_additive_precip_filter_restrictor_coupon_pockets");
    for coupon in 0..RESTRICTOR_COUPONS {
        coupons = coupons
            + centered_cube(
                format!("media_additive_precip_filter_known_restrictor_coupon_slot_{coupon}"),
                56.0,
                24.0,
                PRESSURE_PATH_Z + 5.0,
            )
            .translate(restrictor_coupon_x(coupon), 36.0, 0.0)
            + centered_cube(
                format!("media_additive_precip_filter_restrictor_coupon_label_land_{coupon}"),
                48.0,
                14.0,
                3.0,
            )
            .translate(
                restrictor_coupon_x(coupon),
                64.0,
                PRESSURE_PATH_Z / 2.0 + 1.5,
            );
    }

    let bypass_lane = tube_run_x(
        "media_additive_precip_filter_pressure_drop_bypass_witness_lane",
        -(PRESSURE_PATH_X / 2.0 - 54.0),
        PRESSURE_PATH_X / 2.0 - 54.0,
        62.0,
        4.0,
        TUBE_BORE_D,
    );

    (body - route_channel - pressure_taps - coupons - bypass_lane
        + latch_ears(
            "pressure_witness",
            PRESSURE_PATH_X,
            PRESSURE_PATH_Y,
            PRESSURE_PATH_Z / 2.0,
        ))
    .translate(
        PRESSURE_CENTER_X,
        PRESSURE_CENTER_Y,
        DECK_Z / 2.0 + PRESSURE_PATH_Z / 2.0,
    )
}

fn settled_particle_trap() -> Part {
    let body = centered_cube(
        "media_additive_precip_filter_settled_particle_trap_body",
        PARTICLE_TRAP_X,
        PARTICLE_TRAP_Y,
        PARTICLE_TRAP_Z,
    );
    let inlet_channel = tube_run_x(
        "media_additive_precip_filter_particle_trap_inlet_channel",
        -(PARTICLE_TRAP_X / 2.0 - 26.0),
        PARTICLE_TRAP_X / 2.0 - 26.0,
        0.0,
        10.0,
        TUBE_BORE_D,
    );

    let mut wells = Part::empty("media_additive_precip_filter_settle_wells");
    for well in 0..SETTLE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("media_additive_precip_filter_settled_particle_well_{well}"),
                SETTLE_WELL_D / 2.0,
                PARTICLE_TRAP_Z + 6.0,
                42,
            )
            .translate(settle_well_x(well), -36.0, 0.0)
            + centered_cylinder(
                format!("media_additive_precip_filter_settled_particle_funnel_{well}"),
                24.0,
                18.0,
                42,
            )
            .translate(settle_well_x(well), -36.0, PARTICLE_TRAP_Z / 2.0 - 7.0);
    }

    let mut microscopy = Part::empty("media_additive_precip_filter_microscopy_windows");
    for window in 0..MICROSCOPY_WINDOWS {
        microscopy = microscopy
            + centered_cube(
                format!("media_additive_precip_filter_microscopy_slide_window_{window}"),
                54.0,
                26.0,
                6.0,
            )
            .translate(
                (window as f64 - 1.0) * 62.0,
                PARTICLE_TRAP_Y / 2.0 - 36.0,
                PARTICLE_TRAP_Z / 2.0 + 3.0,
            );
    }

    let drain_manifold = tube_run_y(
        "media_additive_precip_filter_particle_trap_drain_manifold",
        PARTICLE_TRAP_X / 2.0 - 38.0,
        -(PARTICLE_TRAP_Y / 2.0 - 34.0),
        PARTICLE_TRAP_Y / 2.0 - 34.0,
        -12.0,
        HIGH_FLOW_BORE_D,
    );

    (body - inlet_channel - wells - drain_manifold
        + microscopy
        + latch_ears(
            "particle_trap",
            PARTICLE_TRAP_X,
            PARTICLE_TRAP_Y,
            PARTICLE_TRAP_Z / 2.0,
        ))
    .translate(
        PARTICLE_CENTER_X,
        PARTICLE_CENTER_Y,
        DECK_Z / 2.0 + PARTICLE_TRAP_Z / 2.0,
    )
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "media_additive_precip_filter_release_hold_reject_gate_panel",
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );

    let mut lane_recesses = Part::empty("media_additive_precip_filter_status_lane_recesses");
    let mut tokens = Part::empty("media_additive_precip_filter_status_lane_tokens");
    for lane in 0..STATUS_LANES {
        let y = status_lane_y(lane);
        lane_recesses = lane_recesses
            + centered_cube(
                format!("media_additive_precip_filter_status_lane_recess_{lane}"),
                GATE_PANEL_X - 56.0,
                46.0,
                10.0,
            )
            .translate(0.0, y, GATE_PANEL_Z / 2.0 - 4.0);
        tokens = tokens
            + centered_cube(
                format!("media_additive_precip_filter_status_lane_{lane}_evidence_card_land"),
                76.0,
                30.0,
                4.0,
            )
            .translate(-96.0, y, GATE_PANEL_Z / 2.0 + 2.0)
            + centered_cube(
                format!("media_additive_precip_filter_status_lane_{lane}_operator_scan_land"),
                72.0,
                30.0,
                4.0,
            )
            .translate(6.0, y, GATE_PANEL_Z / 2.0 + 2.0)
            + centered_cube(
                format!("media_additive_precip_filter_status_lane_{lane}_retain_vial_land"),
                68.0,
                30.0,
                4.0,
            )
            .translate(102.0, y, GATE_PANEL_Z / 2.0 + 2.0);
    }

    let mut retain_wells = Part::empty("media_additive_precip_filter_retain_sample_wells");
    for lane in 0..STATUS_LANES {
        for well in 0..RETAIN_WELLS_PER_LANE {
            retain_wells = retain_wells
                + centered_cylinder(
                    format!("media_additive_precip_filter_lane_{lane}_retain_well_{well}"),
                    6.0,
                    GATE_PANEL_Z + 4.0,
                    24,
                )
                .translate(120.0 + well as f64 * 14.0, status_lane_y(lane), 0.0);
        }
    }

    let disposition_labels = centered_cube(
        "media_additive_precip_filter_release_lane_label_land",
        68.0,
        16.0,
        3.0,
    )
    .translate(
        -(GATE_PANEL_X / 2.0 - 52.0),
        status_lane_y(0),
        GATE_PANEL_Z / 2.0 + 1.5,
    ) + centered_cube(
        "media_additive_precip_filter_hold_lane_label_land",
        68.0,
        16.0,
        3.0,
    )
    .translate(
        -(GATE_PANEL_X / 2.0 - 52.0),
        status_lane_y(1),
        GATE_PANEL_Z / 2.0 + 1.5,
    ) + centered_cube(
        "media_additive_precip_filter_reject_lane_label_land",
        68.0,
        16.0,
        3.0,
    )
    .translate(
        -(GATE_PANEL_X / 2.0 - 52.0),
        status_lane_y(2),
        GATE_PANEL_Z / 2.0 + 1.5,
    );

    (base - lane_recesses - retain_wells + tokens + disposition_labels).translate(
        GATE_CENTER_X,
        GATE_CENTER_Y,
        DECK_Z / 2.0 + GATE_PANEL_Z / 2.0,
    )
}

fn evidence_capture_panel() -> Part {
    let panel = centered_cube(
        "media_additive_precip_filter_evidence_capture_upright",
        EVIDENCE_PANEL_X,
        22.0,
        EVIDENCE_PANEL_Z,
    );
    let foot = centered_cube(
        "media_additive_precip_filter_evidence_capture_panel_foot",
        EVIDENCE_PANEL_X + 42.0,
        EVIDENCE_PANEL_Y,
        18.0,
    )
    .translate(0.0, 0.0, -(EVIDENCE_PANEL_Z / 2.0 - 9.0));

    let camera_window = centered_cube(
        "media_additive_precip_filter_evidence_camera_window",
        64.0,
        26.0,
        42.0,
    )
    .translate(-96.0, -2.0, 42.0);
    let scanner_window = centered_cube(
        "media_additive_precip_filter_evidence_barcode_scanner_window",
        92.0,
        26.0,
        28.0,
    )
    .translate(42.0, -2.0, 38.0);

    let mut barcode_lands = Part::empty("media_additive_precip_filter_barcode_lands");
    for land in 0..BARCODE_LANDS {
        barcode_lands = barcode_lands
            + centered_cube(
                format!("media_additive_precip_filter_barcode_scan_land_{land}"),
                40.0,
                3.0,
                18.0,
            )
            .translate(
                -130.0 + land as f64 * 48.0,
                -(EVIDENCE_PANEL_Y / 2.0 - 18.0),
                -16.0,
            );
    }

    let mut rfid_lands = Part::empty("media_additive_precip_filter_rfid_lands");
    for land in 0..RFID_LANDS {
        rfid_lands = rfid_lands
            + centered_cube(
                format!("media_additive_precip_filter_rfid_tag_land_{land}"),
                42.0,
                34.0,
                4.0,
            )
            .translate(
                -96.0 + land as f64 * 64.0,
                28.0,
                -(EVIDENCE_PANEL_Z / 2.0 - 20.0),
            );
    }

    let retain_card_slot = centered_cube(
        "media_additive_precip_filter_run_record_card_slot",
        238.0,
        9.0,
        58.0,
    )
    .translate(16.0, 0.0, -32.0);

    (panel - camera_window - scanner_window - retain_card_slot + foot + barcode_lands + rfid_lands)
        .translate(
            EVIDENCE_CENTER_X,
            EVIDENCE_CENTER_Y,
            DECK_Z / 2.0 + EVIDENCE_PANEL_Z / 2.0,
        )
}

fn tubing_route_witness() -> Part {
    let mut routes = Part::empty("media_additive_precip_filter_tubing_route_witnesses");
    for (i, (x1, y1, x2, y2)) in [
        (
            ADDITIVE_CENTER_X + ADDITIVE_BANK_X / 2.0 - 18.0,
            ADDITIVE_CENTER_Y - 34.0,
            MIX_CENTER_X - MIX_COUPON_X / 2.0 + 24.0,
            MIX_CENTER_Y + 52.0,
        ),
        (
            MIX_CENTER_X + MIX_COUPON_X / 2.0 - 26.0,
            MIX_CENTER_Y + 18.0,
            FILTER_CENTER_X - FILTER_WITNESS_X / 2.0 + 28.0,
            FILTER_CENTER_Y,
        ),
        (
            FILTER_CENTER_X + FILTER_WITNESS_X / 2.0 - 26.0,
            FILTER_CENTER_Y - 18.0,
            TURBIDITY_CENTER_X - TURBIDITY_X / 2.0 + 22.0,
            TURBIDITY_CENTER_Y,
        ),
        (
            TURBIDITY_CENTER_X - 34.0,
            TURBIDITY_CENTER_Y - TURBIDITY_Y / 2.0 + 20.0,
            PRESSURE_CENTER_X + PRESSURE_PATH_X / 2.0 - 36.0,
            PRESSURE_CENTER_Y + 34.0,
        ),
        (
            PRESSURE_CENTER_X + PRESSURE_PATH_X / 2.0 - 28.0,
            PRESSURE_CENTER_Y - 36.0,
            PARTICLE_CENTER_X - PARTICLE_TRAP_X / 2.0 + 24.0,
            PARTICLE_CENTER_Y + 42.0,
        ),
        (
            PARTICLE_CENTER_X - PARTICLE_TRAP_X / 2.0 + 36.0,
            PARTICLE_CENTER_Y - 56.0,
            GATE_CENTER_X + GATE_PANEL_X / 2.0 - 42.0,
            GATE_CENTER_Y + status_lane_y(0),
        ),
    ]
    .iter()
    .enumerate()
    {
        routes = routes + elbow_route_placeholder(i, *x1, *y1, *x2, *y2, ROUTE_WITNESS_Z);
    }

    let mut clips = Part::empty("media_additive_precip_filter_tubing_route_clips");
    for clip in 0..ROUTE_CLIPS {
        let (x, y) = route_clip_center(clip);
        clips = clips
            + centered_cube(
                format!("media_additive_precip_filter_tubing_clip_{clip}"),
                28.0,
                15.0,
                14.0,
            )
            .translate(x, y, DECK_Z / 2.0 + 7.0)
            - centered_cylinder(
                format!("media_additive_precip_filter_tubing_clip_bore_{clip}"),
                TUBE_BORE_D / 2.0,
                30.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, DECK_Z / 2.0 + 7.0);
    }

    routes + clips
}

fn robot_service_keepouts() -> Part {
    let front_loading = keepout_box(
        "media_additive_precip_filter_front_loading_keepout",
        DECK_X - 180.0,
        104.0,
        SERVICE_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 86.0,
        DECK_Z / 2.0 + SERVICE_CLEARANCE_Z / 2.0,
    );
    let optics_service = keepout_box(
        "media_additive_precip_filter_optics_service_keepout",
        TURBIDITY_X + 92.0,
        TURBIDITY_Y + 112.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        TURBIDITY_CENTER_X,
        TURBIDITY_CENTER_Y,
        DECK_Z / 2.0 + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let filter_swap = keepout_box(
        "media_additive_precip_filter_filter_swap_keepout",
        FILTER_WITNESS_X + 96.0,
        FILTER_WITNESS_Y + 82.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        FILTER_CENTER_X,
        FILTER_CENTER_Y,
        DECK_Z / 2.0 + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let gate_scanner = keepout_box(
        "media_additive_precip_filter_gate_scanner_keepout",
        GATE_PANEL_X + 90.0,
        GATE_PANEL_Y + 74.0,
        SERVICE_CLEARANCE_Z,
    )
    .translate(
        GATE_CENTER_X,
        GATE_CENTER_Y,
        DECK_Z / 2.0 + SERVICE_CLEARANCE_Z / 2.0,
    );

    front_loading + optics_service + filter_swap + gate_scanner
}

fn latch_ears(name: &str, width: f64, depth: f64, z: f64) -> Part {
    let mut ears = Part::empty(format!("media_additive_precip_filter_{name}_latch_ears"));
    for (i, x) in [-(width / 2.0 - 22.0), width / 2.0 - 22.0]
        .iter()
        .enumerate()
    {
        let ear = centered_cube(
            format!("media_additive_precip_filter_{name}_latch_ear_{i}"),
            34.0,
            24.0,
            12.0,
        )
        .translate(*x, -(depth / 2.0 + 12.0), z);
        let screw = centered_cylinder(
            format!("media_additive_precip_filter_{name}_latch_screw_{i}"),
            3.4 / 2.0,
            14.0,
            20,
        )
        .translate(*x, -(depth / 2.0 + 12.0), z);
        ears = ears + (ear - screw);
    }
    ears
}

fn sensor_tower(name: &str, x: f64, y: f64, z: f64) -> Part {
    let tower = centered_cube(format!("{name}_body"), x, y, z);
    let bore = centered_cylinder(format!("{name}_optical_bore"), 6.0, y + 4.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 18.0);
    let cable_slot = centered_cube(format!("{name}_cable_exit"), x - 12.0, y + 4.0, 10.0)
        .translate(0.0, 0.0, z / 2.0 - 15.0);
    tower - bore - cable_slot
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let shell = centered_cube(format!("{name}_envelope"), x, y, z);
    let inner = centered_cube(format!("{name}_hollow"), x - 18.0, y - 18.0, z - 12.0);
    let label = centered_cube(format!("{name}_label_land"), x * 0.36, 18.0, 3.0).translate(
        0.0,
        -(y / 2.0 - 13.0),
        z / 2.0 + 1.5,
    );
    shell - inner + label
}

fn fiducial_target(name: &str) -> Part {
    let pad = centered_cylinder(format!("{name}_pad"), 14.0, 3.0, 48);
    let cross_x = centered_cube(format!("{name}_cross_x"), 24.0, 3.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 24.0, 4.0);
    pad - cross_x - cross_y
}

fn elbow_route_placeholder(index: usize, x1: f64, y1: f64, x2: f64, y2: f64, z: f64) -> Part {
    let horizontal = tube_run_x(
        &format!("media_additive_precip_filter_route_{index}_x"),
        x1,
        x2,
        y1,
        z,
        TUBE_OD,
    );
    let vertical = tube_run_y(
        &format!("media_additive_precip_filter_route_{index}_y"),
        x2,
        y1,
        y2,
        z,
        TUBE_OD,
    );
    let bend = centered_cylinder(
        format!("media_additive_precip_filter_route_{index}_bend"),
        TUBE_OD / 2.0,
        10.0,
        24,
    )
    .translate(x2, y1, z);
    horizontal + vertical + bend
}

fn tube_run_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    let len = (x_b - x_a).abs().max(1.0);
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x_a + x_b) / 2.0, y, z)
}

fn tube_run_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    let len = (y_b - y_a).abs().max(1.0);
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y_a + y_b) / 2.0, z)
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 56.0), -(DECK_Y / 2.0 - 56.0)),
        (DECK_X / 2.0 - 56.0, -(DECK_Y / 2.0 - 56.0)),
        (-(DECK_X / 2.0 - 56.0), DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -(DECK_Y / 2.0 - 56.0)),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-(DECK_X / 2.0 - 56.0), 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
    ]
}

fn component_centers() -> [(f64, f64); 7] {
    [
        (ADDITIVE_CENTER_X, ADDITIVE_CENTER_Y),
        (MIX_CENTER_X, MIX_CENTER_Y),
        (FILTER_CENTER_X, FILTER_CENTER_Y),
        (TURBIDITY_CENTER_X, TURBIDITY_CENTER_Y),
        (PRESSURE_CENTER_X, PRESSURE_CENTER_Y),
        (PARTICLE_CENTER_X, PARTICLE_CENTER_Y),
        (GATE_CENTER_X, GATE_CENTER_Y),
    ]
}

fn additive_port_x(port: usize) -> f64 {
    (port as f64 - (ADDITIVE_PORTS as f64 - 1.0) / 2.0) * ADDITIVE_PORT_PITCH_X
}

fn mix_lane_y(lane: usize) -> f64 {
    (lane as f64 - (MIXING_LANES as f64 - 1.0) / 2.0) * MIX_LANE_PITCH_Y
}

fn filter_witness_x(filter: usize) -> f64 {
    (filter as f64 - (FILTER_WITNESS_COUNT as f64 - 1.0) / 2.0) * 96.0
}

fn filter_sample_port_x(port: usize) -> f64 {
    (port as f64 - (FILTER_SAMPLE_PORTS as f64 - 1.0) / 2.0) * 48.0
}

fn turbidity_window_x(window: usize) -> f64 {
    (window as f64 - (TURBIDITY_WINDOWS as f64 - 1.0) / 2.0) * 44.0 - 12.0
}

fn pressure_tap_x(tap: usize) -> f64 {
    (tap as f64 - (PRESSURE_TAPS as f64 - 1.0) / 2.0) * PRESSURE_TAP_PITCH_X
}

fn restrictor_coupon_x(coupon: usize) -> f64 {
    (coupon as f64 - (RESTRICTOR_COUPONS as f64 - 1.0) / 2.0) * 116.0
}

fn settle_well_x(well: usize) -> f64 {
    (well as f64 - (SETTLE_WELLS as f64 - 1.0) / 2.0) * 46.0
}

fn status_lane_y(lane: usize) -> f64 {
    ((STATUS_LANES as f64 - 1.0) / 2.0 - lane as f64) * STATUS_LANE_PITCH_Y
}

fn route_clip_center(clip: usize) -> (f64, f64) {
    let points = [
        (-382.0, 184.0),
        (-282.0, 184.0),
        (-74.0, 156.0),
        (46.0, 190.0),
        (274.0, 162.0),
        (384.0, 82.0),
        (302.0, -26.0),
        (108.0, -66.0),
        (198.0, -154.0),
        (-42.0, -178.0),
        (-252.0, -214.0),
    ];
    points[clip]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn includes_required_precipitation_blockage_features() {
        for feature in [
            "closed_additive_inlet_bank",
            "cold_warm_mixing_coupon",
            "inline_filter_witness",
            "optical_turbidity_window",
            "pressure_drop_witness_path",
            "paired_pressure_taps",
            "settled_particle_trap",
            "release_hold_reject_evidence_gates",
            "lot_barcode_rfid_capture",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 13);
    }

    #[test]
    fn major_modules_fit_on_deck() {
        for (x, y, module_x, module_y) in [
            (
                ADDITIVE_CENTER_X,
                ADDITIVE_CENTER_Y,
                ADDITIVE_BANK_X,
                ADDITIVE_BANK_Y,
            ),
            (MIX_CENTER_X, MIX_CENTER_Y, MIX_COUPON_X, MIX_COUPON_Y),
            (
                FILTER_CENTER_X,
                FILTER_CENTER_Y,
                FILTER_WITNESS_X,
                FILTER_WITNESS_Y,
            ),
            (
                TURBIDITY_CENTER_X,
                TURBIDITY_CENTER_Y,
                TURBIDITY_X,
                TURBIDITY_Y,
            ),
            (
                PRESSURE_CENTER_X,
                PRESSURE_CENTER_Y,
                PRESSURE_PATH_X,
                PRESSURE_PATH_Y,
            ),
            (
                PARTICLE_CENTER_X,
                PARTICLE_CENTER_Y,
                PARTICLE_TRAP_X,
                PARTICLE_TRAP_Y,
            ),
            (GATE_CENTER_X, GATE_CENTER_Y, GATE_PANEL_X, GATE_PANEL_Y),
            (
                EVIDENCE_CENTER_X,
                EVIDENCE_CENTER_Y,
                EVIDENCE_PANEL_X,
                EVIDENCE_PANEL_Y,
            ),
        ] {
            assert!(x.abs() + module_x / 2.0 < DECK_X / 2.0);
            assert!(y.abs() + module_y / 2.0 < DECK_Y / 2.0);
        }
    }

    #[test]
    fn witness_counts_match_validation_intent() {
        assert_eq!(THERMAL_ZONES, 2);
        assert_eq!(MIXING_LANES, 4);
        assert_eq!(MIX_PROBE_WELLS, THERMAL_ZONES * 3);
        assert_eq!(FILTER_WITNESS_COUNT, 2);
        assert_eq!(FILTER_SAMPLE_PORTS, FILTER_WITNESS_COUNT * 2);
        assert_eq!(TURBIDITY_WINDOWS, 3);
        assert_eq!(PRESSURE_TAPS, 4);
        assert_eq!(RESTRICTOR_COUPONS, 3);
        assert_eq!(SETTLE_WELLS, 4);
        assert_eq!(STATUS_LANES, 3);
    }

    #[test]
    fn tubing_and_sensor_clearances_are_sane() {
        assert!(TUBE_BORE_D > TUBE_OD);
        assert!(HIGH_FLOW_BORE_D > TUBE_BORE_D);
        assert!(SENSOR_TAP_D < TUBE_BORE_D);
        assert!(FILTER_MEMBRANE_D > 6.0 * HIGH_FLOW_BORE_D);
        assert!(OPTICAL_PATH_LENGTH > TUBE_BORE_D * 3.0);
        assert!(SETTLE_WELL_D > HIGH_FLOW_BORE_D * 4.0);
    }

    #[test]
    fn arrays_are_centered_and_inside_their_carriers() {
        assert!((additive_port_x(0) + additive_port_x(ADDITIVE_PORTS - 1)).abs() < 0.001);
        assert!(additive_port_x(0).abs() < ADDITIVE_BANK_X / 2.0 - 42.0);
        assert!((mix_lane_y(0) + mix_lane_y(MIXING_LANES - 1)).abs() < 0.001);
        assert!(mix_lane_y(0).abs() < MIX_COUPON_Y / 2.0 - 42.0);
        assert!((pressure_tap_x(0) + pressure_tap_x(PRESSURE_TAPS - 1)).abs() < 0.001);
        assert!(pressure_tap_x(0).abs() < PRESSURE_PATH_X / 2.0 - 78.0);
        assert!((settle_well_x(0) + settle_well_x(SETTLE_WELLS - 1)).abs() < 0.001);
        assert!(settle_well_x(0).abs() < PARTICLE_TRAP_X / 2.0 - 40.0);
    }

    #[test]
    fn status_gate_order_is_release_hold_reject() {
        assert!(status_lane_y(0) > status_lane_y(1));
        assert!(status_lane_y(1) > status_lane_y(2));
        assert_eq!(RETAIN_WELLS_PER_LANE * STATUS_LANES, 12);
        assert_eq!(BARCODE_LANDS, 6);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(ROBOT_KEEP_OUTS, 4);
    }

    #[test]
    fn generated_parts_construct_without_panics() {
        let parts = [
            station_deck(),
            additive_inlet_bank(),
            cold_warm_mixing_coupon(),
            inline_filter_witness(),
            optical_turbidity_window(),
            pressure_drop_witness_path(),
            settled_particle_trap(),
            release_hold_reject_gates(),
            evidence_capture_panel(),
            tubing_route_witness(),
            robot_service_keepouts(),
            station_assembly(),
        ];
        assert_eq!(parts.len(), OUTPUTS.len());
    }
}
