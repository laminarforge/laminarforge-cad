use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-source transport shock preseed recovery validation station.
//
// This generator packages mechanical validation geometry for transport shock,
// custody, evidence capture, and recovery dwell handling around sealed
// cell-source vial and bag surrogates. It is mechanical validation packaging
// only. It is not a biological SOP, sterile-process claim, GMP release
// decision, clinical acceptance criterion, or viability acceptance criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_cell_source_transport_shock_preseed_recovery_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_source_transport_shock_preseed_recovery_station_containment_deck.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_sealed_cell_source_vial_bag_nests.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_vibration_tilt_logger_pockets.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_thermal_shock_coupon_blocks.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_recovery_dwell_token_rail.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_gentle_mix_witness_cradle.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_viability_sample_loop_surrogate.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_bubble_dead_volume_windows.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_identity_barcode_rfid_custody_lands.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_release_hold_reject_gates.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_camera_evidence_bridge.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_robot_service_keepouts.stl",
    "output/closed_cell_source_transport_shock_preseed_recovery_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 28] = [
    "mechanical_validation_packaging_only",
    "sealed_cell_source_bag_nests",
    "sealed_cell_source_vial_nests",
    "transport_shock_index_lands",
    "vibration_logger_pockets",
    "tilt_logger_pockets",
    "transport_vibration_axis_ticks",
    "thermal_shock_coupon_blocks",
    "thermal_shock_cold_coupons",
    "thermal_shock_warm_coupons",
    "recovery_dwell_token_rail",
    "preseed_recovery_dwell_tokens",
    "gentle_mix_witness_cradle",
    "mix_angle_witness_stops",
    "viability_sample_loop_surrogate",
    "sample_loop_port_bosses",
    "bubble_witness_windows",
    "dead_volume_windows",
    "identity_barcode_lands",
    "identity_rfid_custody_lands",
    "custody_tamper_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 5] = [
    "biological_sop",
    "sterile_process_claim",
    "gmp_release_decision",
    "clinical_acceptance_criterion",
    "viability_acceptance_criterion",
];

const DECK_X: f64 = 1380.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1190.0;
const SUMP_Y: f64 = 720.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 20.0;
const DATUM_BOSSES: usize = 10;

const SOURCE_NEST_X: f64 = 420.0;
const SOURCE_NEST_Y: f64 = 240.0;
const SOURCE_NEST_Z: f64 = 50.0;
const SOURCE_NEST_POS: (f64, f64) = (-430.0, 270.0);
const BAG_NESTS: usize = 3;
const VIAL_NESTS: usize = 6;
const BAG_NEST_X: f64 = 92.0;
const BAG_NEST_Y: f64 = 130.0;
const BAG_PITCH_X: f64 = 112.0;
const BAG_NECK_D: f64 = 22.0;
const VIAL_NEST_D: f64 = 26.0;
const VIAL_PITCH_X: f64 = 52.0;
const SHOCK_INDEX_LANDS: usize = BAG_NESTS + VIAL_NESTS;

const LOGGER_BANK_X: f64 = 360.0;
const LOGGER_BANK_Y: f64 = 150.0;
const LOGGER_BANK_Z: f64 = 38.0;
const LOGGER_BANK_POS: (f64, f64) = (70.0, 270.0);
const VIBRATION_LOGGERS: usize = 4;
const TILT_LOGGERS: usize = 3;
const LOGGER_POCKET_X: f64 = 55.0;
const LOGGER_POCKET_Y: f64 = 78.0;
const LOGGER_PITCH_X: f64 = 72.0;
const TILT_POCKET_D: f64 = 42.0;
const AXIS_TICKS: usize = VIBRATION_LOGGERS * 3;

const THERMAL_BANK_X: f64 = 300.0;
const THERMAL_BANK_Y: f64 = 210.0;
const THERMAL_BANK_Z: f64 = 46.0;
const THERMAL_BANK_POS: (f64, f64) = (470.0, 250.0);
const COLD_COUPONS: usize = 4;
const WARM_COUPONS: usize = 4;
const THERMAL_COUPON_X: f64 = 44.0;
const THERMAL_COUPON_Y: f64 = 58.0;
const THERMAL_COUPON_PITCH_X: f64 = 54.0;
const THERMAL_COUPON_PITCH_Y: f64 = 74.0;

const DWELL_RAIL_X: f64 = 390.0;
const DWELL_RAIL_Y: f64 = 130.0;
const DWELL_RAIL_Z: f64 = 36.0;
const DWELL_RAIL_POS: (f64, f64) = (-430.0, 55.0);
const DWELL_TOKENS: usize = 8;
const DWELL_TOKEN_D: f64 = 28.0;
const DWELL_TOKEN_PITCH_X: f64 = 43.0;

const MIX_CRADLE_X: f64 = 470.0;
const MIX_CRADLE_Y: f64 = 180.0;
const MIX_CRADLE_Z: f64 = 44.0;
const MIX_CRADLE_POS: (f64, f64) = (40.0, 50.0);
const MIX_ROLLERS: usize = 4;
const MIX_ANGLE_STOPS: usize = 5;
const MIX_WITNESS_BEADS: usize = 8;
const MIX_ROLLER_D: f64 = 22.0;
const MIX_ROLLER_PITCH_X: f64 = 96.0;

const LOOP_BANK_X: f64 = 310.0;
const LOOP_BANK_Y: f64 = 190.0;
const LOOP_BANK_Z: f64 = 44.0;
const LOOP_BANK_POS: (f64, f64) = (455.0, 35.0);
const SAMPLE_LOOPS: usize = 4;
const SAMPLE_LOOP_PITCH_X: f64 = 62.0;
const SAMPLE_LOOP_D: f64 = 8.0;
const SAMPLE_PORT_D: f64 = 22.0;
const LOOP_WITNESS_CUPS: usize = SAMPLE_LOOPS * 2;

const WINDOW_BANK_X: f64 = 410.0;
const WINDOW_BANK_Y: f64 = 140.0;
const WINDOW_BANK_Z: f64 = 32.0;
const WINDOW_BANK_POS: (f64, f64) = (-420.0, -200.0);
const BUBBLE_WINDOWS: usize = 6;
const DEAD_VOLUME_WINDOWS: usize = 5;
const BUBBLE_WINDOW_D: f64 = 30.0;
const DEAD_VOLUME_WINDOW_X: f64 = 44.0;
const DEAD_VOLUME_WINDOW_Y: f64 = 24.0;
const WINDOW_PITCH_X: f64 = 52.0;

const CUSTODY_X: f64 = 390.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (60.0, -205.0);
const BARCODE_LANDS: usize = 9;
const RFID_LANDS: usize = 6;
const TAMPER_LANDS: usize = 5;

const GATE_BANK_X: f64 = 320.0;
const GATE_BANK_Y: f64 = 180.0;
const GATE_BANK_Z: f64 = 38.0;
const GATE_BANK_POS: (f64, f64) = (440.0, -230.0);
const DISPOSITION_GATES: usize = 3;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;
const GATE_TOKEN_SLOTS: usize = 9;
const GATE_PITCH_Y: f64 = 54.0;

const CAMERA_BRIDGE_X: f64 = 1020.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 220.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -35.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;

const ROBOT_KEEPOUT_X: f64 = 1190.0;
const ROBOT_KEEPOUT_Y: f64 = 96.0;
const ROBOT_KEEPOUT_Z: f64 = 82.0;
const SERVICE_KEEPOUT_X: f64 = 104.0;
const SERVICE_KEEPOUT_Y: f64 = 710.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 315.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(sealed_cell_source_vial_bag_nests(), OUTPUTS[1]);
    write_part(vibration_tilt_logger_pockets(), OUTPUTS[2]);
    write_part(thermal_shock_coupon_blocks(), OUTPUTS[3]);
    write_part(recovery_dwell_token_rail(), OUTPUTS[4]);
    write_part(gentle_mix_witness_cradle(), OUTPUTS[5]);
    write_part(viability_sample_loop_surrogate(), OUTPUTS[6]);
    write_part(bubble_dead_volume_windows(), OUTPUTS[7]);
    write_part(identity_barcode_rfid_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed cell-source transport shock preseed recovery station: {:.0}mm x {:.0}mm contained deck, {} sealed bag nests, {} sealed vial nests.",
        DECK_X, DECK_Y, BAG_NESTS, VIAL_NESTS
    );
    println!(
        "Mechanical validation packaging only: {} vibration loggers, {} tilt loggers, {} cold/warm thermal shock coupons, {} recovery dwell tokens.",
        VIBRATION_LOGGERS,
        TILT_LOGGERS,
        COLD_COUPONS + WARM_COUPONS,
        DWELL_TOKENS
    );
    println!(
        "Evidence features: {} sample loop surrogates, {} bubble windows, {} dead-volume windows, release/hold/reject gates, {} camera mounts.",
        SAMPLE_LOOPS, BUBBLE_WINDOWS, DEAD_VOLUME_WINDOWS, CAMERA_MOUNTS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "cell_source_transport_shock_preseed_recovery_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "cell_source_transport_shock_preseed_recovery_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, -4.0, DECK_Z / 2.0 - SUMP_Z / 2.0);
    let drain = centered_cylinder(
        "cell_source_transport_shock_preseed_recovery_captured_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 36.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 12.0, 0.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        "cell_source_transport_shock_preseed_recovery_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, rim_center_z());
    let rear = centered_cube(
        "cell_source_transport_shock_preseed_recovery_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let left = centered_cube(
        "cell_source_transport_shock_preseed_recovery_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_center_z());
    let right = centered_cube(
        "cell_source_transport_shock_preseed_recovery_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());

    front + rear + left + right
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("cell_source_transport_shock_preseed_recovery_deck_datums");
    for i in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 84.0 + i as f64 * ((DECK_X - 168.0) / 9.0);
        let y = if i % 2 == 0 {
            DECK_Y / 2.0 - 72.0
        } else {
            -DECK_Y / 2.0 + 72.0
        };
        let pad = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_datum_pad_{i}"),
            12.0,
            5.0,
            36,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_datum_bore_{i}"),
            3.0,
            8.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }
    datums
}

fn station_landing_pockets() -> Part {
    landing_pocket(
        "sealed_cell_source_nest_land",
        SOURCE_NEST_X,
        SOURCE_NEST_Y,
        SOURCE_NEST_POS,
    ) + landing_pocket(
        "vibration_tilt_logger_land",
        LOGGER_BANK_X,
        LOGGER_BANK_Y,
        LOGGER_BANK_POS,
    ) + landing_pocket(
        "thermal_shock_coupon_land",
        THERMAL_BANK_X,
        THERMAL_BANK_Y,
        THERMAL_BANK_POS,
    ) + landing_pocket(
        "recovery_dwell_token_land",
        DWELL_RAIL_X,
        DWELL_RAIL_Y,
        DWELL_RAIL_POS,
    ) + landing_pocket(
        "gentle_mix_witness_cradle_land",
        MIX_CRADLE_X,
        MIX_CRADLE_Y,
        MIX_CRADLE_POS,
    ) + landing_pocket(
        "viability_sample_loop_surrogate_land",
        LOOP_BANK_X,
        LOOP_BANK_Y,
        LOOP_BANK_POS,
    ) + landing_pocket(
        "bubble_dead_volume_window_land",
        WINDOW_BANK_X,
        WINDOW_BANK_Y,
        WINDOW_BANK_POS,
    ) + landing_pocket("identity_custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pocket(
            "release_hold_reject_gate_land",
            GATE_BANK_X,
            GATE_BANK_Y,
            GATE_BANK_POS,
        )
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("cell_source_transport_shock_preseed_recovery_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn sealed_cell_source_vial_bag_nests() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_sealed_cell_source_nest_base",
        SOURCE_NEST_X,
        SOURCE_NEST_Y,
        SOURCE_NEST_Z,
    );
    let bag_lane = centered_cube(
        "cell_source_transport_shock_preseed_recovery_sealed_bag_lane_land",
        SOURCE_NEST_X - 38.0,
        18.0,
        5.0,
    )
    .translate(0.0, SOURCE_NEST_Y / 2.0 - 22.0, SOURCE_NEST_Z / 2.0 + 4.0);
    let vial_lane = centered_cube(
        "cell_source_transport_shock_preseed_recovery_sealed_vial_lane_land",
        SOURCE_NEST_X - 48.0,
        18.0,
        5.0,
    )
    .translate(0.0, -SOURCE_NEST_Y / 2.0 + 22.0, SOURCE_NEST_Z / 2.0 + 4.0);
    let shock_strip = centered_cube(
        "cell_source_transport_shock_preseed_recovery_transport_shock_index_strip",
        SOURCE_NEST_X - 58.0,
        9.0,
        6.0,
    )
    .translate(0.0, 0.0, SOURCE_NEST_Z / 2.0 + 5.0);

    let mut nests = base + bag_lane + vial_lane + shock_strip;

    for i in 0..BAG_NESTS {
        let x = port_x(i, BAG_NESTS, BAG_PITCH_X);
        let bag_recess = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_sealed_cell_source_bag_nest_recess_{i}"),
            BAG_NEST_X,
            BAG_NEST_Y,
            24.0,
        )
        .translate(x, 34.0, SOURCE_NEST_Z / 2.0 - 6.0);
        let left_saddle = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_bag_transport_saddle_left_{i}"),
            10.0,
            BAG_NEST_Y - 18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 25.0, 34.0, SOURCE_NEST_Z / 2.0 + 12.0);
        let right_saddle = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_bag_transport_saddle_right_{i}"),
            10.0,
            BAG_NEST_Y - 18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 25.0, 34.0, SOURCE_NEST_Z / 2.0 + 12.0);
        let neck_land = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_sealed_bag_neck_land_{i}"),
            BAG_NECK_D / 2.0 + 5.0,
            8.0,
            32,
        )
        .translate(x, -37.0, SOURCE_NEST_Z / 2.0 + 5.0);
        let neck_bore = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_sealed_bag_neck_bore_{i}"),
            BAG_NECK_D / 2.0,
            14.0,
            28,
        )
        .translate(x, -37.0, SOURCE_NEST_Z / 2.0 + 5.0);
        let clamp = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_bag_shock_clamp_land_{i}"),
            BAG_NEST_X - 16.0,
            10.0,
            8.0,
        )
        .translate(x, 97.0, SOURCE_NEST_Z / 2.0 + 8.0);
        nests = nests - bag_recess + left_saddle + right_saddle + neck_land - neck_bore + clamp;
    }

    for i in 0..VIAL_NESTS {
        let x = port_x(i, VIAL_NESTS, VIAL_PITCH_X);
        let vial_bore = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_sealed_cell_source_vial_nest_bore_{i}"),
            VIAL_NEST_D / 2.0,
            SOURCE_NEST_Z + 8.0,
            36,
        )
        .translate(x, -76.0, 0.0);
        let vial_lip = centered_cylinder(
            format!(
                "cell_source_transport_shock_preseed_recovery_sealed_cell_source_vial_nest_lip_{i}"
            ),
            VIAL_NEST_D / 2.0 + 5.0,
            6.0,
            36,
        )
        .translate(x, -76.0, SOURCE_NEST_Z / 2.0 + 4.0);
        let shock_land = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_transport_shock_index_land_{i}"),
            22.0,
            7.0,
            5.0,
        )
        .translate(x, -43.0, SOURCE_NEST_Z / 2.0 + 5.0);
        nests = nests - vial_bore + vial_lip + shock_land;
    }

    nests
}

fn vibration_tilt_logger_pockets() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_vibration_tilt_logger_bank",
        LOGGER_BANK_X,
        LOGGER_BANK_Y,
        LOGGER_BANK_Z,
    );
    let strap_rail = centered_cube(
        "cell_source_transport_shock_preseed_recovery_logger_retention_strap_rail",
        LOGGER_BANK_X - 36.0,
        10.0,
        10.0,
    )
    .translate(0.0, 0.0, LOGGER_BANK_Z / 2.0 + 7.0);

    let mut bank = base + strap_rail;

    for i in 0..VIBRATION_LOGGERS {
        let x = port_x(i, VIBRATION_LOGGERS, LOGGER_PITCH_X);
        let pocket = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_vibration_logger_pocket_cut_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            18.0,
        )
        .translate(x, 35.0, LOGGER_BANK_Z / 2.0 - 5.0);
        let rim = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_vibration_logger_pocket_rim_{i}"),
            LOGGER_POCKET_X + 12.0,
            LOGGER_POCKET_Y + 10.0,
            5.0,
        )
        .translate(x, 35.0, LOGGER_BANK_Z / 2.0 + 4.0);
        bank = bank - pocket + rim;

        for axis in 0..3 {
            let tick = centered_cube(
                format!("cell_source_transport_shock_preseed_recovery_transport_vibration_axis_tick_{i}_{axis}"),
                7.0 + axis as f64 * 8.0,
                4.0,
                4.0,
            )
            .translate(
                x,
                4.0 + axis as f64 * 10.0,
                LOGGER_BANK_Z / 2.0 + 7.0 + axis as f64 * 1.0,
            );
            bank = bank + tick;
        }
    }

    for i in 0..TILT_LOGGERS {
        let x = port_x(i, TILT_LOGGERS, LOGGER_PITCH_X);
        let pocket = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_tilt_logger_pocket_cut_{i}"),
            TILT_POCKET_D / 2.0,
            20.0,
            36,
        )
        .translate(x, -42.0, LOGGER_BANK_Z / 2.0 - 4.0);
        let collar = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_tilt_logger_pocket_collar_{i}"),
            TILT_POCKET_D / 2.0 + 5.0,
            6.0,
            36,
        )
        .translate(x, -42.0, LOGGER_BANK_Z / 2.0 + 4.0);
        let reference = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_tilt_reference_saddle_{i}"),
            38.0,
            6.0,
            5.0,
        )
        .translate(x, -8.0, LOGGER_BANK_Z / 2.0 + 5.0);
        bank = bank - pocket + collar + reference;
    }

    bank
}

fn thermal_shock_coupon_blocks() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_thermal_shock_coupon_bank",
        THERMAL_BANK_X,
        THERMAL_BANK_Y,
        THERMAL_BANK_Z,
    );
    let divider = centered_cube(
        "cell_source_transport_shock_preseed_recovery_thermal_shock_lane_divider",
        THERMAL_BANK_X - 34.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, THERMAL_BANK_Z / 2.0 + 7.0);

    let mut bank = base + divider;

    for i in 0..COLD_COUPONS {
        let coupon = thermal_coupon("cold", i, THERMAL_COUPON_PITCH_Y / 2.0, COLD_COUPONS);
        bank = bank - coupon.0 + coupon.1 + coupon.2;
    }
    for i in 0..WARM_COUPONS {
        let coupon = thermal_coupon("warm", i, -THERMAL_COUPON_PITCH_Y / 2.0, WARM_COUPONS);
        bank = bank - coupon.0 + coupon.1 + coupon.2;
    }

    let gradient_comb = thermal_gradient_comb();
    let logger_land = centered_cube(
        "cell_source_transport_shock_preseed_recovery_thermal_logger_custody_land",
        72.0,
        22.0,
        6.0,
    )
    .translate(
        -THERMAL_BANK_X / 2.0 + 58.0,
        0.0,
        THERMAL_BANK_Z / 2.0 + 5.0,
    );

    bank + gradient_comb + logger_land
}

fn thermal_coupon(prefix: &str, index: usize, y: f64, count: usize) -> (Part, Part, Part) {
    let x = port_x(index, count, THERMAL_COUPON_PITCH_X);
    let cut = centered_cube(
        format!("cell_source_transport_shock_preseed_recovery_thermal_shock_{prefix}_coupon_slot_{index}"),
        THERMAL_COUPON_X,
        THERMAL_COUPON_Y,
        THERMAL_BANK_Z + 8.0,
    )
    .translate(x, y, 0.0);
    let block = centered_cube(
        format!("cell_source_transport_shock_preseed_recovery_thermal_shock_{prefix}_coupon_block_{index}"),
        THERMAL_COUPON_X - 10.0,
        THERMAL_COUPON_Y - 14.0,
        12.0,
    )
    .translate(x, y, THERMAL_BANK_Z / 2.0 + 6.0);
    let witness_tick = centered_cube(
        format!("cell_source_transport_shock_preseed_recovery_thermal_shock_{prefix}_witness_tick_{index}"),
        30.0,
        5.0,
        5.0,
    )
    .translate(x, y - THERMAL_COUPON_Y / 2.0 - 10.0, THERMAL_BANK_Z / 2.0 + 5.0);
    (cut, block, witness_tick)
}

fn thermal_gradient_comb() -> Part {
    let mut comb =
        Part::empty("cell_source_transport_shock_preseed_recovery_thermal_gradient_comb");
    for i in 0..5 {
        let step = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_thermal_gradient_step_{i}"),
            15.0,
            48.0,
            6.0 + i as f64 * 3.0,
        )
        .translate(
            THERMAL_BANK_X / 2.0 - 72.0 + i as f64 * 15.0,
            0.0,
            THERMAL_BANK_Z / 2.0 + 3.0 + i as f64 * 1.5,
        );
        comb = comb + step;
    }
    comb
}

fn recovery_dwell_token_rail() -> Part {
    let rail = centered_cube(
        "cell_source_transport_shock_preseed_recovery_dwell_token_rail",
        DWELL_RAIL_X,
        DWELL_RAIL_Y,
        DWELL_RAIL_Z,
    );
    let sight_slot = centered_cube(
        "cell_source_transport_shock_preseed_recovery_dwell_token_sight_slot",
        DWELL_RAIL_X - 54.0,
        18.0,
        16.0,
    )
    .translate(0.0, -8.0, DWELL_RAIL_Z / 2.0);

    let mut part = rail - sight_slot;
    for i in 0..DWELL_TOKENS {
        let x = token_x(i);
        let token_well = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_preseed_recovery_dwell_token_well_{i}"),
            DWELL_TOKEN_D / 2.0,
            DWELL_RAIL_Z + 10.0,
            36,
        )
        .translate(x, 28.0, 6.0);
        let token = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_preseed_recovery_dwell_token_gauge_{i}"),
            DWELL_TOKEN_D / 2.0 - 3.0,
            6.0,
            36,
        )
        .translate(x, 28.0, DWELL_RAIL_Z / 2.0 + 5.0);
        let dwell_step = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_recovery_dwell_step_gauge_{i}"),
            12.0,
            38.0,
            5.0 + i as f64 * 1.8,
        )
        .translate(
            x,
            -DWELL_RAIL_Y / 2.0 + 23.0,
            DWELL_RAIL_Z / 2.0 + 2.5 + i as f64 * 0.9,
        );
        part = part - token_well + token + dwell_step;
    }

    part
}

fn gentle_mix_witness_cradle() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_gentle_mix_witness_cradle_base",
        MIX_CRADLE_X,
        MIX_CRADLE_Y,
        MIX_CRADLE_Z,
    );
    let cradle_recess = centered_cube(
        "cell_source_transport_shock_preseed_recovery_gentle_mix_bag_cradle_recess",
        MIX_CRADLE_X - 74.0,
        MIX_CRADLE_Y - 64.0,
        20.0,
    )
    .translate(0.0, 0.0, MIX_CRADLE_Z / 2.0 - 6.0);
    let perimeter_witness = centered_cube(
        "cell_source_transport_shock_preseed_recovery_gentle_mix_perimeter_witness_land",
        MIX_CRADLE_X - 44.0,
        MIX_CRADLE_Y - 40.0,
        8.0,
    )
    .translate(0.0, 0.0, MIX_CRADLE_Z / 2.0 + 5.0);

    let mut cradle = base - cradle_recess + perimeter_witness;

    for i in 0..MIX_ROLLERS {
        let x = port_x(i, MIX_ROLLERS, MIX_ROLLER_PITCH_X);
        let roller = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_gentle_mix_witness_roller_{i}"),
            MIX_ROLLER_D / 2.0,
            MIX_CRADLE_Y - 42.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, MIX_CRADLE_Z / 2.0 + 13.0);
        cradle = cradle + roller;
    }

    for i in 0..MIX_ANGLE_STOPS {
        let x = port_x(i, MIX_ANGLE_STOPS, 76.0);
        let height = 9.0 + i as f64 * 3.0;
        let stop = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_mix_angle_witness_stop_{i}"),
            34.0,
            10.0,
            height,
        )
        .translate(
            x,
            MIX_CRADLE_Y / 2.0 - 18.0,
            MIX_CRADLE_Z / 2.0 + height / 2.0,
        );
        cradle = cradle + stop;
    }

    for i in 0..MIX_WITNESS_BEADS {
        let bead = centered_cylinder(
            format!(
                "cell_source_transport_shock_preseed_recovery_gentle_mix_witness_bead_land_{i}"
            ),
            7.0,
            5.0,
            24,
        )
        .translate(
            port_x(i, MIX_WITNESS_BEADS, 42.0),
            -66.0,
            MIX_CRADLE_Z / 2.0 + 4.0,
        );
        cradle = cradle + bead;
    }

    cradle
}

fn viability_sample_loop_surrogate() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_viability_sample_loop_surrogate_bank",
        LOOP_BANK_X,
        LOOP_BANK_Y,
        LOOP_BANK_Z,
    );
    let header = centered_cylinder(
        "cell_source_transport_shock_preseed_recovery_sample_loop_header_surrogate",
        SAMPLE_LOOP_D / 2.0,
        LOOP_BANK_X - 56.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, LOOP_BANK_Z / 2.0 + 7.0);

    let mut bank = base + header;

    for i in 0..SAMPLE_LOOPS {
        let x = sample_loop_x(i);
        let inlet = centered_cylinder(
            format!(
                "cell_source_transport_shock_preseed_recovery_viability_sample_loop_inlet_leg_{i}"
            ),
            SAMPLE_LOOP_D / 2.0,
            116.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 12.0, 0.0, LOOP_BANK_Z / 2.0 + 13.0);
        let return_leg = centered_cylinder(
            format!(
                "cell_source_transport_shock_preseed_recovery_viability_sample_loop_return_leg_{i}"
            ),
            SAMPLE_LOOP_D / 2.0,
            116.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 12.0, 0.0, LOOP_BANK_Z / 2.0 + 13.0);
        let turn_cup = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_viability_sample_loop_turn_witness_cup_{i}"),
            22.0,
            7.0,
            36,
        )
        .translate(x, LOOP_BANK_Y / 2.0 - 34.0, LOOP_BANK_Z / 2.0 + 5.0);
        let deadleg_token = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_sample_loop_low_dead_volume_token_{i}"),
            32.0,
            12.0,
            6.0,
        )
        .translate(x, -LOOP_BANK_Y / 2.0 + 34.0, LOOP_BANK_Z / 2.0 + 5.0);

        bank = bank + inlet + return_leg + turn_cup + deadleg_token;

        for side in 0..2 {
            let y = if side == 0 {
                -LOOP_BANK_Y / 2.0 - 4.0
            } else {
                LOOP_BANK_Y / 2.0 + 4.0
            };
            let port = centered_cylinder(
                format!(
                    "cell_source_transport_shock_preseed_recovery_sample_loop_port_boss_{i}_{side}"
                ),
                SAMPLE_PORT_D / 2.0,
                28.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, LOOP_BANK_Z / 2.0 + 8.0);
            let bore = centered_cylinder(
                format!(
                    "cell_source_transport_shock_preseed_recovery_sample_loop_port_bore_{i}_{side}"
                ),
                3.8,
                36.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, LOOP_BANK_Z / 2.0 + 8.0);
            bank = bank + port - bore;
        }
    }

    bank
}

fn bubble_dead_volume_windows() -> Part {
    let frame = centered_cube(
        "cell_source_transport_shock_preseed_recovery_bubble_dead_volume_window_frame",
        WINDOW_BANK_X,
        WINDOW_BANK_Y,
        WINDOW_BANK_Z,
    );
    let backlight_slot = centered_cube(
        "cell_source_transport_shock_preseed_recovery_window_backlight_slot",
        WINDOW_BANK_X - 58.0,
        17.0,
        16.0,
    )
    .translate(0.0, 0.0, WINDOW_BANK_Z / 2.0);

    let mut windows = frame - backlight_slot;

    for i in 0..BUBBLE_WINDOWS {
        let x = port_x(i, BUBBLE_WINDOWS, WINDOW_PITCH_X);
        let cut = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_bubble_witness_window_cut_{i}"),
            BUBBLE_WINDOW_D / 2.0,
            WINDOW_BANK_Z + 8.0,
            36,
        )
        .translate(x, 36.0, 0.0);
        let rim = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_bubble_witness_window_rim_{i}"),
            BUBBLE_WINDOW_D / 2.0 + 4.0,
            5.0,
            36,
        )
        .translate(x, 36.0, WINDOW_BANK_Z / 2.0 + 4.0);
        windows = windows - cut + rim;
    }

    for i in 0..DEAD_VOLUME_WINDOWS {
        let x = port_x(i, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X + 10.0);
        let cut = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_dead_volume_window_cut_{i}"),
            DEAD_VOLUME_WINDOW_X,
            DEAD_VOLUME_WINDOW_Y,
            WINDOW_BANK_Z + 8.0,
        )
        .translate(x, -38.0, 0.0);
        let channel = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_dead_volume_witness_channel_{i}"),
            DEAD_VOLUME_WINDOW_X + 18.0,
            7.0,
            6.0,
        )
        .translate(x, -7.0, WINDOW_BANK_Z / 2.0 + 5.0);
        windows = windows - cut + channel;
    }

    windows
}

fn identity_barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "cell_source_transport_shock_preseed_recovery_identity_barcode_rfid_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut custody = panel;

    for i in 0..BARCODE_LANDS {
        let land = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_identity_barcode_land_{i}"),
            34.0,
            18.0,
            4.0,
        )
        .translate(port_x(i, BARCODE_LANDS, 35.0), 34.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..RFID_LANDS {
        let land = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_identity_rfid_custody_land_{i}"),
            40.0,
            30.0,
            4.0,
        )
        .translate(port_x(i, RFID_LANDS, 46.0), -16.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..TAMPER_LANDS {
        let seal = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_custody_tamper_land_{i}"),
            8.0,
            4.0,
            24,
        )
        .translate(-124.0 + i as f64 * 62.0, -49.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + seal;
    }

    custody
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "cell_source_transport_shock_preseed_recovery_release_hold_reject_gate_bank",
        GATE_BANK_X,
        GATE_BANK_Y,
        GATE_BANK_Z,
    );
    let mut gates = base;

    for i in 0..DISPOSITION_GATES {
        let y = gate_y(i);
        let name = disposition_gate_name(i);
        let lane = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_{name}_gate_lane"),
            GATE_BANK_X - 42.0,
            34.0,
            10.0,
        )
        .translate(0.0, y, GATE_BANK_Z / 2.0 + 5.0);
        let latch = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_{name}_gate_latch_witness"),
            46.0,
            24.0,
            18.0,
        )
        .translate(GATE_BANK_X / 2.0 - 46.0, y, GATE_BANK_Z / 2.0 + 10.0);
        let indicator = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_{name}_gate_indicator_well"),
            12.0,
            GATE_BANK_Z + 8.0,
            32,
        )
        .translate(-GATE_BANK_X / 2.0 + 48.0, y, 0.0);
        let indicator_rim = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_{name}_gate_indicator_rim"),
            16.0,
            5.0,
            32,
        )
        .translate(-GATE_BANK_X / 2.0 + 48.0, y, GATE_BANK_Z / 2.0 + 4.0);
        gates = gates + lane + latch - indicator + indicator_rim;
    }

    for i in 0..GATE_TOKEN_SLOTS {
        let gate = i % DISPOSITION_GATES;
        let column = i / DISPOSITION_GATES;
        let slot = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_gate_token_slot_{i}"),
            26.0,
            18.0,
            8.0,
        )
        .translate(
            -46.0 + column as f64 * 42.0,
            gate_y(gate),
            GATE_BANK_Z / 2.0 + 5.0,
        );
        gates = gates - slot;
    }

    gates
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "cell_source_transport_shock_preseed_recovery_camera_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 42.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "cell_source_transport_shock_preseed_recovery_camera_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 42.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "cell_source_transport_shock_preseed_recovery_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        32.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 16.0);

    let mut bridge = left_post + right_post + beam;
    for i in 0..CAMERA_MOUNTS {
        let x = port_x(i, CAMERA_MOUNTS, 196.0);
        let mount = centered_cube(
            format!("cell_source_transport_shock_preseed_recovery_camera_mount_plate_{i}"),
            72.0,
            12.0,
            48.0,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 8.0, CAMERA_BRIDGE_Z - 42.0);
        let lens_bore = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_camera_lens_axis_bore_{i}"),
            11.0,
            18.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 9.0, CAMERA_BRIDGE_Z - 42.0);
        bridge = bridge + mount - lens_bore;
    }

    for i in 0..EVIDENCE_FIDUCIALS {
        let x = port_x(i, EVIDENCE_FIDUCIALS, 86.0);
        let fiducial = centered_cylinder(
            format!("cell_source_transport_shock_preseed_recovery_evidence_fiducial_{i}"),
            6.0,
            4.0,
            24,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 16.0);
        bridge = bridge + fiducial;
    }

    bridge
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "cell_source_transport_shock_preseed_recovery_robot_front_sweep_keepout",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let service_left = centered_cube(
        "cell_source_transport_shock_preseed_recovery_left_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "cell_source_transport_shock_preseed_recovery_right_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_service = centered_cube(
        "cell_source_transport_shock_preseed_recovery_top_service_clearance_gauge",
        720.0,
        430.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    robot_front + service_left + service_right + top_service
}

fn station_assembly() -> Part {
    containment_deck()
        + sealed_cell_source_vial_bag_nests().translate(
            SOURCE_NEST_POS.0,
            SOURCE_NEST_POS.1,
            top_z(SOURCE_NEST_Z),
        )
        + vibration_tilt_logger_pockets().translate(
            LOGGER_BANK_POS.0,
            LOGGER_BANK_POS.1,
            top_z(LOGGER_BANK_Z),
        )
        + thermal_shock_coupon_blocks().translate(
            THERMAL_BANK_POS.0,
            THERMAL_BANK_POS.1,
            top_z(THERMAL_BANK_Z),
        )
        + recovery_dwell_token_rail().translate(
            DWELL_RAIL_POS.0,
            DWELL_RAIL_POS.1,
            top_z(DWELL_RAIL_Z),
        )
        + gentle_mix_witness_cradle().translate(
            MIX_CRADLE_POS.0,
            MIX_CRADLE_POS.1,
            top_z(MIX_CRADLE_Z),
        )
        + viability_sample_loop_surrogate().translate(
            LOOP_BANK_POS.0,
            LOOP_BANK_POS.1,
            top_z(LOOP_BANK_Z),
        )
        + bubble_dead_volume_windows().translate(
            WINDOW_BANK_POS.0,
            WINDOW_BANK_POS.1,
            top_z(WINDOW_BANK_Z),
        )
        + identity_barcode_rfid_custody_lands().translate(
            CUSTODY_POS.0,
            CUSTODY_POS.1,
            top_z(CUSTODY_Z),
        )
        + release_hold_reject_gates().translate(
            GATE_BANK_POS.0,
            GATE_BANK_POS.1,
            top_z(GATE_BANK_Z),
        )
        + camera_evidence_bridge().translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, DECK_Z / 2.0)
        + robot_service_keepouts().translate(0.0, 0.0, DECK_Z / 2.0)
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn token_x(index: usize) -> f64 {
    port_x(index, DWELL_TOKENS, DWELL_TOKEN_PITCH_X)
}

fn sample_loop_x(index: usize) -> f64 {
    port_x(index, SAMPLE_LOOPS, SAMPLE_LOOP_PITCH_X)
}

fn gate_y(index: usize) -> f64 {
    (index as f64 - (DISPOSITION_GATES as f64 - 1.0) / 2.0) * GATE_PITCH_Y
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

fn inside_deck(pos: (f64, f64), x: f64, y: f64) -> bool {
    pos.0 - x / 2.0 > -DECK_X / 2.0 + RIM_W
        && pos.0 + x / 2.0 < DECK_X / 2.0 - RIM_W
        && pos.1 - y / 2.0 > -DECK_Y / 2.0 + RIM_W
        && pos.1 + y / 2.0 < DECK_Y / 2.0 - RIM_W
}

fn source_bag_span() -> f64 {
    (BAG_NESTS as f64 - 1.0) * BAG_PITCH_X + BAG_NEST_X
}

fn source_vial_span() -> f64 {
    (VIAL_NESTS as f64 - 1.0) * VIAL_PITCH_X + VIAL_NEST_D
}

fn logger_span(count: usize) -> f64 {
    (count as f64 - 1.0) * LOGGER_PITCH_X + LOGGER_POCKET_X
}

fn thermal_coupon_span(count: usize) -> f64 {
    (count as f64 - 1.0) * THERMAL_COUPON_PITCH_X + THERMAL_COUPON_X
}

fn dwell_token_span() -> f64 {
    (DWELL_TOKENS as f64 - 1.0) * DWELL_TOKEN_PITCH_X + DWELL_TOKEN_D
}

fn mix_roller_span() -> f64 {
    (MIX_ROLLERS as f64 - 1.0) * MIX_ROLLER_PITCH_X + MIX_ROLLER_D
}

fn sample_loop_span() -> f64 {
    (SAMPLE_LOOPS as f64 - 1.0) * SAMPLE_LOOP_PITCH_X + SAMPLE_PORT_D
}

fn bubble_window_span() -> f64 {
    (BUBBLE_WINDOWS as f64 - 1.0) * WINDOW_PITCH_X + BUBBLE_WINDOW_D
}

fn dead_volume_window_span() -> f64 {
    (DEAD_VOLUME_WINDOWS as f64 - 1.0) * (WINDOW_PITCH_X + 10.0) + DEAD_VOLUME_WINDOW_X
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert_eq!(COLD_COUPONS, WARM_COUPONS);
    assert_eq!(SHOCK_INDEX_LANDS, BAG_NESTS + VIAL_NESTS);
    assert_eq!(LOOP_WITNESS_CUPS, SAMPLE_LOOPS * 2);
    assert_eq!(AXIS_TICKS, VIBRATION_LOGGERS * 3);
    assert!(inside_deck(SOURCE_NEST_POS, SOURCE_NEST_X, SOURCE_NEST_Y));
    assert!(inside_deck(LOGGER_BANK_POS, LOGGER_BANK_X, LOGGER_BANK_Y));
    assert!(inside_deck(
        THERMAL_BANK_POS,
        THERMAL_BANK_X,
        THERMAL_BANK_Y
    ));
    assert!(inside_deck(DWELL_RAIL_POS, DWELL_RAIL_X, DWELL_RAIL_Y));
    assert!(inside_deck(MIX_CRADLE_POS, MIX_CRADLE_X, MIX_CRADLE_Y));
    assert!(inside_deck(LOOP_BANK_POS, LOOP_BANK_X, LOOP_BANK_Y));
    assert!(inside_deck(WINDOW_BANK_POS, WINDOW_BANK_X, WINDOW_BANK_Y));
    assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
    assert!(inside_deck(GATE_BANK_POS, GATE_BANK_X, GATE_BANK_Y));
    assert!(source_bag_span() < SOURCE_NEST_X - 72.0);
    assert!(source_vial_span() < SOURCE_NEST_X - 72.0);
    assert!(logger_span(VIBRATION_LOGGERS) < LOGGER_BANK_X - 58.0);
    assert!(logger_span(TILT_LOGGERS) < LOGGER_BANK_X - 100.0);
    assert!(thermal_coupon_span(COLD_COUPONS) < THERMAL_BANK_X - 64.0);
    assert!(thermal_coupon_span(WARM_COUPONS) < THERMAL_BANK_X - 64.0);
    assert!(dwell_token_span() < DWELL_RAIL_X - 46.0);
    assert!(mix_roller_span() < MIX_CRADLE_X - 90.0);
    assert!(sample_loop_span() < LOOP_BANK_X - 62.0);
    assert!(bubble_window_span() < WINDOW_BANK_X - 70.0);
    assert!(dead_volume_window_span() < WINDOW_BANK_X - 76.0);
    assert!(ROBOT_KEEPOUT_X < DECK_X);
    assert!(SERVICE_KEEPOUT_Y < DECK_Y);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
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
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
        assert!(OUTPUTS
            .iter()
            .any(|path| path.contains("sealed_cell_source_vial_bag_nests")));
        assert!(OUTPUTS
            .iter()
            .any(|path| path.contains("robot_service_keepouts")));
    }

    #[test]
    fn requested_feature_metadata_is_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 28);
        assert!(REQUIRED_FEATURES.contains(&"mechanical_validation_packaging_only"));
        assert!(REQUIRED_FEATURES.contains(&"sealed_cell_source_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"sealed_cell_source_vial_nests"));
        assert!(REQUIRED_FEATURES.contains(&"vibration_logger_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"tilt_logger_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"thermal_shock_coupon_blocks"));
        assert!(REQUIRED_FEATURES.contains(&"recovery_dwell_token_rail"));
        assert!(REQUIRED_FEATURES.contains(&"gentle_mix_witness_cradle"));
        assert!(REQUIRED_FEATURES.contains(&"viability_sample_loop_surrogate"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"dead_volume_windows"));
        assert!(REQUIRED_FEATURES.contains(&"identity_barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"identity_rfid_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn scope_excludes_process_release_and_clinical_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 5);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_sop"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"gmp_release_decision"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"clinical_acceptance_criterion"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"viability_acceptance_criterion"));
    }

    #[test]
    fn counts_match_transport_shock_recovery_packaging_intent() {
        assert_eq!(BAG_NESTS, 3);
        assert_eq!(VIAL_NESTS, 6);
        assert_eq!(SHOCK_INDEX_LANDS, BAG_NESTS + VIAL_NESTS);
        assert_eq!(COLD_COUPONS, WARM_COUPONS);
        assert!(DWELL_TOKENS >= COLD_COUPONS + WARM_COUPONS);
        assert!(MIX_WITNESS_BEADS >= SAMPLE_LOOPS * 2);
        assert_eq!(LOOP_WITNESS_CUPS, SAMPLE_LOOPS * 2);
        assert_eq!(BARCODE_LANDS, SHOCK_INDEX_LANDS);
        assert_eq!(RFID_LANDS, VIAL_NESTS);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_MOUNTS, 5);
    }

    #[test]
    fn layout_fits_contained_robotic_station() {
        assert_layout();
        assert!(inside_deck(SOURCE_NEST_POS, SOURCE_NEST_X, SOURCE_NEST_Y));
        assert!(inside_deck(
            THERMAL_BANK_POS,
            THERMAL_BANK_X,
            THERMAL_BANK_Y
        ));
        assert!(inside_deck(MIX_CRADLE_POS, MIX_CRADLE_X, MIX_CRADLE_Y));
        assert!(inside_deck(LOOP_BANK_POS, LOOP_BANK_X, LOOP_BANK_Y));
        assert!(inside_deck(WINDOW_BANK_POS, WINDOW_BANK_X, WINDOW_BANK_Y));
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
    }

    #[test]
    fn geometry_spans_stay_within_module_envelopes() {
        assert!(source_bag_span() < SOURCE_NEST_X - 72.0);
        assert!(source_vial_span() < SOURCE_NEST_X - 72.0);
        assert!(logger_span(VIBRATION_LOGGERS) < LOGGER_BANK_X - 58.0);
        assert!(thermal_coupon_span(COLD_COUPONS) < THERMAL_BANK_X - 64.0);
        assert!(dwell_token_span() < DWELL_RAIL_X - 46.0);
        assert!(mix_roller_span() < MIX_CRADLE_X - 90.0);
        assert!(sample_loop_span() < LOOP_BANK_X - 62.0);
        assert!(bubble_window_span() < WINDOW_BANK_X - 70.0);
        assert!(dead_volume_window_span() < WINDOW_BANK_X - 76.0);
    }

    #[test]
    fn disposition_and_evidence_indices_are_stable() {
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert_eq!(GATE_TOKEN_SLOTS % DISPOSITION_GATES, 0);
        assert!(EVIDENCE_FIDUCIALS >= CAMERA_MOUNTS * 2);
        assert_eq!(AXIS_TICKS, VIBRATION_LOGGERS * 3);
    }
}
