use std::f64::consts::TAU;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Robotic end effector for moving sealed 20-chip cassette modules.
//
// Intent:
// - Grip cassette side datums with sterile-compatible parallel fingers and replaceable pads.
// - Keep force/torque sensing, barcode/RFID reading, cable/tube relief, and quick-change wrist
//   features explicit in the CAD architecture.
// - Reserve collision and leak-tray clearance envelopes for parent workcell integration.
//
// This is a parametric packaging model. It is not a process-validation or sterile-material claim.

const OUTPUTS: &[&str] = &[
    "output/robotic_cassette_gripper_end_effector_wrist_plate.stl",
    "output/robotic_cassette_gripper_end_effector_force_torque_placeholder.stl",
    "output/robotic_cassette_gripper_end_effector_finger_bodies.stl",
    "output/robotic_cassette_gripper_end_effector_compliant_pads.stl",
    "output/robotic_cassette_gripper_end_effector_datum_receivers.stl",
    "output/robotic_cassette_gripper_end_effector_scan_window.stl",
    "output/robotic_cassette_gripper_end_effector_strain_relief.stl",
    "output/robotic_cassette_gripper_end_effector_cleanable_covers.stl",
    "output/robotic_cassette_gripper_end_effector_leak_tray_clearance.stl",
    "output/robotic_cassette_gripper_end_effector_collision_keepout.stl",
    "output/robotic_cassette_gripper_end_effector_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;
const SEALED_MODULE_Z: f64 = 78.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const PAD_FACE_CLEARANCE_X: f64 = 3.0;
const PAD_INNER_SPAN_X: f64 = CASSETTE_X + 2.0 * PAD_FACE_CLEARANCE_X;
const PAD_X: f64 = 14.0;
const PAD_Y: f64 = 96.0;
const PAD_Z: f64 = 22.0;
const PAD_COUNT: usize = 3;
const PAD_RIB_Z: f64 = 2.0;

const FINGER_X: f64 = 28.0;
const FINGER_Y: f64 = CASSETTE_Y + 34.0;
const FINGER_Z: f64 = 88.0;
const FINGER_Z_CENTER: f64 = 38.0;
const FINGER_TIP_Y_RELIEF: f64 = 36.0;

const BRIDGE_X: f64 = PAD_INNER_SPAN_X + 2.0 * (PAD_X + FINGER_X + 52.0);
const BRIDGE_Y: f64 = 156.0;
const BRIDGE_Z: f64 = 34.0;
const BRIDGE_Z_CENTER: f64 = 114.0;

const WRIST_PLATE_RADIUS: f64 = 64.0;
const WRIST_PLATE_Z: f64 = 14.0;
const WRIST_Z_CENTER: f64 = 196.0;
const WRIST_BOLT_COUNT: usize = 6;
const WRIST_BOLT_CIRCLE_DIA: f64 = 100.0;
const WRIST_BOLT_HOLE_DIA: f64 = 6.6;
const WRIST_CENTER_BORE_DIA: f64 = 32.0;

const SENSOR_X: f64 = 92.0;
const SENSOR_Y: f64 = 92.0;
const SENSOR_Z: f64 = 48.0;
const SENSOR_Z_CENTER: f64 = 162.0;
const SENSOR_BOLT_PITCH: f64 = 62.0;

const DATUM_PIN_DIA: f64 = 6.0;
const DATUM_RECEIVER_CLEARANCE: f64 = 0.45;
const DATUM_RECEIVER_DIA: f64 = DATUM_PIN_DIA + 2.0 * DATUM_RECEIVER_CLEARANCE;
const DATUM_BOSS_DIA: f64 = 18.0;
const DATUM_BOSS_Z: f64 = 14.0;

const SCAN_WINDOW_X: f64 = 118.0;
const SCAN_WINDOW_Y: f64 = 8.0;
const SCAN_WINDOW_Z: f64 = 38.0;
const SCAN_WINDOW_CLEAR_X: f64 = 86.0;
const SCAN_WINDOW_CLEAR_Z: f64 = 20.0;
const RFID_COIL_X: f64 = 146.0;
const RFID_COIL_Z: f64 = 58.0;

const STRAIN_RELIEF_X: f64 = 198.0;
const STRAIN_RELIEF_Y: f64 = 44.0;
const STRAIN_RELIEF_Z: f64 = 34.0;
const STRAIN_RELIEF_CHANNELS: usize = 5;
const STRAIN_RELIEF_CHANNEL_DIA: f64 = 8.0;
const MIN_TUBE_BEND_RADIUS: f64 = 42.0;

const CLEAN_COVER_Z: f64 = 16.0;
const CLEAN_COVER_EDGE_R: f64 = 8.0;

const LEAK_TRAY_CLEARANCE_Z: f64 = 32.0;
const LEAK_TRAY_ENVELOPE_X: f64 = CASSETTE_X + 110.0;
const LEAK_TRAY_ENVELOPE_Y: f64 = CASSETTE_Y + 96.0;
const LEAK_TRAY_ENVELOPE_Z: f64 = 24.0;

const COLLISION_KEEP_OUT_X: f64 = BRIDGE_X + 80.0;
const COLLISION_KEEP_OUT_Y: f64 = CASSETTE_Y + 140.0;
const COLLISION_KEEP_OUT_Z: f64 = 236.0;
const KEEP_OUT_RAIL_W: f64 = 8.0;

fn main() {
    let wrist = quick_change_wrist_plate();
    export(&wrist, OUTPUTS[0]);

    let sensor = force_torque_sensor_placeholder();
    export(&sensor, OUTPUTS[1]);

    let fingers = sterile_gripper_finger_bodies();
    export(&fingers, OUTPUTS[2]);

    let pads = compliant_pad_set();
    export(&pads, OUTPUTS[3]);

    let datum = datum_pin_receivers();
    export(&datum, OUTPUTS[4]);

    let scan = barcode_rfid_scan_window();
    export(&scan, OUTPUTS[5]);

    let strain_relief = cable_tube_strain_relief();
    export(&strain_relief, OUTPUTS[6]);

    let covers = cleanable_cover_surfaces();
    export(&covers, OUTPUTS[7]);

    let leak_clearance = leak_tray_clearance_gauge();
    export(&leak_clearance, OUTPUTS[8]);

    let keepout = collision_keepout_envelope();
    export(&keepout, OUTPUTS[9]);

    let assembly = wrist
        + sensor
        + fingers
        + pads
        + datum
        + scan
        + strain_relief
        + covers
        + leak_clearance
        + keepout;
    export(&assembly, OUTPUTS[10]);

    println!(
        "Robotic cassette gripper end effector: {:.0}mm cassette X span, {:.0}mm cassette Y span, {:.0}mm sealed-module height envelope, {:.1}mm pad working clearance per side, {:.0}mm wrist bolt circle, and {:.0}mm leak-tray Z clearance.",
        CASSETTE_X,
        CASSETTE_Y,
        SEALED_MODULE_Z,
        PAD_FACE_CLEARANCE_X,
        WRIST_BOLT_CIRCLE_DIA,
        LEAK_TRAY_CLEARANCE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn quick_change_wrist_plate() -> Part {
    let plate = centered_cylinder(
        "robotic_cassette_gripper_wrist_plate",
        WRIST_PLATE_RADIUS,
        WRIST_PLATE_Z,
        96,
    )
    .translate(0.0, 0.0, WRIST_Z_CENTER);
    let center_bore = centered_cylinder(
        "robotic_cassette_gripper_wrist_center_bore",
        WRIST_CENTER_BORE_DIA / 2.0,
        WRIST_PLATE_Z + 4.0,
        64,
    )
    .translate(0.0, 0.0, WRIST_Z_CENTER);

    let mut bolt_holes = Part::empty("robotic_cassette_gripper_wrist_bolt_holes");
    let mut bolt_boss_washers = Part::empty("robotic_cassette_gripper_wrist_boss_washers");
    for (i, (x, y)) in wrist_bolt_points().iter().enumerate() {
        bolt_holes = bolt_holes
            + centered_cylinder(
                format!("robotic_cassette_gripper_wrist_m6_hole_{i}"),
                WRIST_BOLT_HOLE_DIA / 2.0,
                WRIST_PLATE_Z + 6.0,
                32,
            )
            .translate(*x, *y, WRIST_Z_CENTER);
        bolt_boss_washers = bolt_boss_washers
            + centered_cylinder(
                format!("robotic_cassette_gripper_wrist_washer_land_{i}"),
                10.5,
                2.0,
                32,
            )
            .translate(*x, *y, WRIST_Z_CENTER + WRIST_PLATE_Z / 2.0 + 1.0);
    }

    let anti_rotation_key = centered_cube(
        "robotic_cassette_gripper_wrist_anti_rotation_key",
        18.0,
        78.0,
        8.0,
    )
    .translate(0.0, 0.0, WRIST_Z_CENTER + WRIST_PLATE_Z / 2.0 + 4.0);
    let dog_tab_a = centered_cube(
        "robotic_cassette_gripper_wrist_quick_change_dog_a",
        26.0,
        16.0,
        12.0,
    )
    .translate(-46.0, 0.0, WRIST_Z_CENTER - 2.0);
    let dog_tab_b = centered_cube(
        "robotic_cassette_gripper_wrist_quick_change_dog_b",
        26.0,
        16.0,
        12.0,
    )
    .translate(46.0, 0.0, WRIST_Z_CENTER - 2.0);

    plate + bolt_boss_washers + anti_rotation_key + dog_tab_a + dog_tab_b - center_bore - bolt_holes
}

fn force_torque_sensor_placeholder() -> Part {
    let block = centered_cube(
        "robotic_cassette_gripper_force_torque_sensor_placeholder",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z_CENTER);
    let top_land = centered_cube(
        "robotic_cassette_gripper_force_torque_top_adapter_land",
        SENSOR_X + 24.0,
        SENSOR_Y + 24.0,
        8.0,
    )
    .translate(0.0, 0.0, SENSOR_Z_CENTER + SENSOR_Z / 2.0 + 4.0);
    let lower_land = centered_cube(
        "robotic_cassette_gripper_force_torque_lower_adapter_land",
        SENSOR_X + 42.0,
        SENSOR_Y + 18.0,
        10.0,
    )
    .translate(0.0, 0.0, SENSOR_Z_CENTER - SENSOR_Z / 2.0 - 5.0);

    let mut holes = Part::empty("robotic_cassette_gripper_force_torque_bolt_holes");
    for (i, (x, y)) in sensor_bolt_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("robotic_cassette_gripper_sensor_m4_clearance_{i}"),
                4.3 / 2.0,
                SENSOR_Z + 24.0,
                24,
            )
            .translate(*x, *y, SENSOR_Z_CENTER);
    }

    let service_port = centered_cube(
        "robotic_cassette_gripper_sensor_service_connector",
        34.0,
        12.0,
        20.0,
    )
    .translate(0.0, SENSOR_Y / 2.0 + 6.0, SENSOR_Z_CENTER);
    let cable_key = centered_cube(
        "robotic_cassette_gripper_sensor_cable_key_cut",
        14.0,
        10.0,
        10.0,
    )
    .translate(0.0, SENSOR_Y / 2.0 + 8.0, SENSOR_Z_CENTER);

    block + top_land + lower_land + service_port - holes - cable_key
}

fn sterile_gripper_finger_bodies() -> Part {
    one_finger_body("left", -1.0) + one_finger_body("right", 1.0) + cross_bridge()
}

fn one_finger_body(name: &str, sign: f64) -> Part {
    let x = finger_body_x(sign);
    let main_beam = centered_cube(
        format!("robotic_cassette_gripper_{name}_sterile_finger_beam"),
        FINGER_X,
        FINGER_Y,
        FINGER_Z,
    )
    .translate(x, 0.0, FINGER_Z_CENTER);
    let distal_round_end = centered_cylinder(
        format!("robotic_cassette_gripper_{name}_rounded_distal_end"),
        FINGER_X / 2.0,
        FINGER_Z,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, -FINGER_Y / 2.0, FINGER_Z_CENTER);
    let proximal_round_end = centered_cylinder(
        format!("robotic_cassette_gripper_{name}_rounded_proximal_end"),
        FINGER_X / 2.0,
        FINGER_Z,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, FINGER_Y / 2.0, FINGER_Z_CENTER);

    let inner_pad_land = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_mount_land"),
        8.0,
        FINGER_Y - 58.0,
        FINGER_Z - 22.0,
    )
    .translate(
        sign * (PAD_INNER_SPAN_X / 2.0 + PAD_X + 4.0),
        0.0,
        FINGER_Z_CENTER - 2.0,
    );
    let outer_washdown_lip = centered_cube(
        format!("robotic_cassette_gripper_{name}_outer_washdown_lip"),
        8.0,
        FINGER_Y - 32.0,
        10.0,
    )
    .translate(
        sign * (finger_body_x(sign).abs() + FINGER_X / 2.0 + 4.0),
        0.0,
        FINGER_Z_CENTER + FINGER_Z / 2.0 - 8.0,
    );
    let rear_triage_stop = centered_cube(
        format!("robotic_cassette_gripper_{name}_rear_cassette_soft_stop"),
        FINGER_X + 10.0,
        18.0,
        28.0,
    )
    .translate(x, CASSETTE_Y / 2.0 + 18.0, 18.0);

    let side_relief = centered_cube(
        format!("robotic_cassette_gripper_{name}_side_drain_relief"),
        FINGER_X + 4.0,
        FINGER_Y - 2.0 * FINGER_TIP_Y_RELIEF,
        18.0,
    )
    .translate(x, 0.0, FINGER_Z_CENTER - FINGER_Z / 2.0 + 14.0);
    let cable_shadow_relief = centered_cube(
        format!("robotic_cassette_gripper_{name}_rear_harness_shadow_relief"),
        FINGER_X + 4.0,
        72.0,
        28.0,
    )
    .translate(x, CASSETTE_Y / 2.0 + 42.0, FINGER_Z_CENTER + 8.0);

    main_beam
        + distal_round_end
        + proximal_round_end
        + inner_pad_land
        + outer_washdown_lip
        + rear_triage_stop
        - side_relief
        - cable_shadow_relief
}

fn cross_bridge() -> Part {
    let bridge = centered_cube(
        "robotic_cassette_gripper_sanitary_cross_bridge",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(0.0, 0.0, BRIDGE_Z_CENTER);
    let underside_channel = centered_cube(
        "robotic_cassette_gripper_bridge_underside_wash_channel",
        PAD_INNER_SPAN_X - 26.0,
        BRIDGE_Y - 44.0,
        14.0,
    )
    .translate(0.0, 0.0, BRIDGE_Z_CENTER - BRIDGE_Z / 2.0 + 7.0);
    let scan_cavity = centered_cube(
        "robotic_cassette_gripper_bridge_scan_window_cavity",
        SCAN_WINDOW_X + 10.0,
        18.0,
        SCAN_WINDOW_Z + 10.0,
    )
    .translate(0.0, -(BRIDGE_Y / 2.0 - 10.0), BRIDGE_Z_CENTER - 2.0);
    let lightening_a = centered_cube(
        "robotic_cassette_gripper_bridge_left_lightening_wash_slot",
        92.0,
        BRIDGE_Y + 4.0,
        16.0,
    )
    .translate(-BRIDGE_X / 2.0 + 104.0, 0.0, BRIDGE_Z_CENTER + 2.0);
    let lightening_b = centered_cube(
        "robotic_cassette_gripper_bridge_right_lightening_wash_slot",
        92.0,
        BRIDGE_Y + 4.0,
        16.0,
    )
    .translate(BRIDGE_X / 2.0 - 104.0, 0.0, BRIDGE_Z_CENTER + 2.0);

    bridge - underside_channel - scan_cavity - lightening_a - lightening_b
}

fn compliant_pad_set() -> Part {
    let mut pads = Part::empty("robotic_cassette_gripper_compliant_pad_set");
    for i in 0..PAD_COUNT {
        let y = pad_y(i);
        pads = pads + compliant_pad("left", -1.0, i, y) + compliant_pad("right", 1.0, i, y);
    }
    pads
}

fn compliant_pad(name: &str, sign: f64, index: usize, y: f64) -> Part {
    let x = pad_center_x(sign);
    let body = centered_cube(
        format!("robotic_cassette_gripper_{name}_compliant_pad_{index}"),
        PAD_X,
        PAD_Y,
        PAD_Z,
    )
    .translate(x, y, pad_center_z());
    let pad_groove = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_cleaning_groove_{index}"),
        PAD_X + 2.0,
        6.0,
        PAD_Z + 2.0,
    )
    .translate(x, y, pad_center_z());
    let top_relief = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_top_relief_{index}"),
        PAD_X + 2.0,
        PAD_Y - 18.0,
        5.0,
    )
    .translate(x, y, pad_center_z() + PAD_Z / 2.0 - 2.0);
    let lower_relief = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_lower_relief_{index}"),
        PAD_X + 2.0,
        PAD_Y - 18.0,
        4.0,
    )
    .translate(x, y, pad_center_z() - PAD_Z / 2.0 + 2.0);

    let rib_front = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_front_grip_rib_{index}"),
        PAD_RIB_Z,
        PAD_Y - 12.0,
        4.0,
    )
    .translate(
        sign * (PAD_INNER_SPAN_X / 2.0 - PAD_RIB_Z / 2.0),
        y,
        pad_center_z() + 2.0,
    );
    let rib_lower = centered_cube(
        format!("robotic_cassette_gripper_{name}_pad_lower_grip_rib_{index}"),
        PAD_RIB_Z,
        PAD_Y - 12.0,
        4.0,
    )
    .translate(
        sign * (PAD_INNER_SPAN_X / 2.0 - PAD_RIB_Z / 2.0),
        y,
        pad_center_z() - 6.0,
    );

    body + rib_front + rib_lower - pad_groove - top_relief - lower_relief
}

fn datum_pin_receivers() -> Part {
    let mut receivers = Part::empty("robotic_cassette_gripper_datum_pin_receivers");
    for (i, (x, y)) in datum_receiver_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("robotic_cassette_gripper_datum_receiver_boss_{i}"),
            DATUM_BOSS_DIA / 2.0,
            DATUM_BOSS_Z,
            36,
        )
        .translate(*x, *y, datum_receiver_z());
        let pin_bore = centered_cylinder(
            format!("robotic_cassette_gripper_datum_receiver_pin_bore_{i}"),
            DATUM_RECEIVER_DIA / 2.0,
            DATUM_BOSS_Z + 3.0,
            28,
        )
        .translate(*x, *y, datum_receiver_z());
        let lead_in_slot = centered_cube(
            format!("robotic_cassette_gripper_datum_receiver_lead_in_slot_{i}"),
            DATUM_RECEIVER_DIA + 7.0,
            24.0,
            7.0,
        )
        .translate(*x, *y - 8.0, datum_receiver_z() + DATUM_BOSS_Z / 2.0 - 3.5);

        receivers = receivers + (boss - pin_bore - lead_in_slot);
    }

    let datum_cross_tie = centered_cube(
        "robotic_cassette_gripper_datum_receiver_cross_tie",
        PAD_INNER_SPAN_X - 84.0,
        12.0,
        12.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 14.0), datum_receiver_z());
    receivers + datum_cross_tie
}

fn barcode_rfid_scan_window() -> Part {
    let front_y = -(BRIDGE_Y / 2.0 + SCAN_WINDOW_Y / 2.0);
    let window_frame = rectangular_frame(
        "robotic_cassette_gripper_barcode_scan_window_frame",
        SCAN_WINDOW_X,
        SCAN_WINDOW_Y,
        SCAN_WINDOW_Z,
        SCAN_WINDOW_CLEAR_X,
        SCAN_WINDOW_CLEAR_Z,
    )
    .translate(0.0, front_y, BRIDGE_Z_CENTER - 2.0);
    let lens_land = centered_cube(
        "robotic_cassette_gripper_recessed_clear_lens_land",
        SCAN_WINDOW_CLEAR_X - 8.0,
        3.0,
        SCAN_WINDOW_CLEAR_Z - 5.0,
    )
    .translate(0.0, front_y - 4.0, BRIDGE_Z_CENTER - 2.0);
    let rfid_outer = rectangular_frame(
        "robotic_cassette_gripper_rfid_antenna_outer_loop",
        RFID_COIL_X,
        3.0,
        RFID_COIL_Z,
        RFID_COIL_X - 12.0,
        RFID_COIL_Z - 12.0,
    )
    .translate(0.0, front_y - 8.0, BRIDGE_Z_CENTER - 2.0);
    let rfid_inner = rectangular_frame(
        "robotic_cassette_gripper_rfid_antenna_inner_loop",
        RFID_COIL_X - 22.0,
        3.0,
        RFID_COIL_Z - 22.0,
        RFID_COIL_X - 34.0,
        RFID_COIL_Z - 34.0,
    )
    .translate(0.0, front_y - 12.0, BRIDGE_Z_CENTER - 2.0);
    let status_led_land = centered_cube(
        "robotic_cassette_gripper_scan_status_led_land",
        18.0,
        4.0,
        10.0,
    )
    .translate(
        SCAN_WINDOW_X / 2.0 + 20.0,
        front_y - 5.0,
        BRIDGE_Z_CENTER - 2.0,
    );

    window_frame + lens_land + rfid_outer + rfid_inner + status_led_land
}

fn cable_tube_strain_relief() -> Part {
    let y = CASSETTE_Y / 2.0 + 92.0;
    let body = centered_cube(
        "robotic_cassette_gripper_rear_cable_tube_strain_relief",
        STRAIN_RELIEF_X,
        STRAIN_RELIEF_Y,
        STRAIN_RELIEF_Z,
    )
    .translate(0.0, y, BRIDGE_Z_CENTER - 6.0);

    let mut cuts = Part::empty("robotic_cassette_gripper_strain_relief_channel_cuts");
    for i in 0..STRAIN_RELIEF_CHANNELS {
        let x = strain_relief_channel_x(i);
        let channel = centered_cylinder(
            format!("robotic_cassette_gripper_strain_relief_tube_channel_{i}"),
            STRAIN_RELIEF_CHANNEL_DIA / 2.0,
            STRAIN_RELIEF_Y + 4.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, BRIDGE_Z_CENTER - 8.0);
        let top_slot = centered_cube(
            format!("robotic_cassette_gripper_strain_relief_top_service_slot_{i}"),
            STRAIN_RELIEF_CHANNEL_DIA + 4.0,
            STRAIN_RELIEF_Y + 6.0,
            9.0,
        )
        .translate(x, y, BRIDGE_Z_CENTER + 1.0);
        cuts = cuts + channel + top_slot;
    }

    let zip_tie_a = centered_cube(
        "robotic_cassette_gripper_strain_relief_zip_tie_slot_a",
        STRAIN_RELIEF_X - 26.0,
        6.0,
        8.0,
    )
    .translate(0.0, y - 12.0, BRIDGE_Z_CENTER + 4.0);
    let zip_tie_b = centered_cube(
        "robotic_cassette_gripper_strain_relief_zip_tie_slot_b",
        STRAIN_RELIEF_X - 26.0,
        6.0,
        8.0,
    )
    .translate(0.0, y + 12.0, BRIDGE_Z_CENTER + 4.0);
    let bend_radius_keeper = centered_cube(
        "robotic_cassette_gripper_minimum_bend_radius_keeper",
        STRAIN_RELIEF_X + 42.0,
        12.0,
        12.0,
    )
    .translate(
        0.0,
        y + STRAIN_RELIEF_Y / 2.0 + MIN_TUBE_BEND_RADIUS,
        BRIDGE_Z_CENTER - 6.0,
    );

    body + bend_radius_keeper - cuts - zip_tie_a - zip_tie_b
}

fn cleanable_cover_surfaces() -> Part {
    let top_cover = centered_cube(
        "robotic_cassette_gripper_cleanable_bridge_cover",
        BRIDGE_X + 28.0,
        BRIDGE_Y + 24.0,
        CLEAN_COVER_Z,
    )
    .translate(
        0.0,
        0.0,
        BRIDGE_Z_CENTER + BRIDGE_Z / 2.0 + CLEAN_COVER_Z / 2.0,
    );
    let front_radius = centered_cylinder(
        "robotic_cassette_gripper_clean_cover_front_radius",
        CLEAN_COVER_EDGE_R,
        BRIDGE_X + 28.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        0.0,
        -(BRIDGE_Y / 2.0 + 12.0),
        BRIDGE_Z_CENTER + BRIDGE_Z / 2.0,
    );
    let rear_radius = centered_cylinder(
        "robotic_cassette_gripper_clean_cover_rear_radius",
        CLEAN_COVER_EDGE_R,
        BRIDGE_X + 28.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, BRIDGE_Y / 2.0 + 12.0, BRIDGE_Z_CENTER + BRIDGE_Z / 2.0);

    let service_seam = centered_cube(
        "robotic_cassette_gripper_cover_single_service_seam",
        2.0,
        BRIDGE_Y + 20.0,
        CLEAN_COVER_Z + 4.0,
    )
    .translate(
        0.0,
        0.0,
        BRIDGE_Z_CENTER + BRIDGE_Z / 2.0 + CLEAN_COVER_Z / 2.0,
    );
    let washdown_label_recess = centered_cube(
        "robotic_cassette_gripper_cover_washdown_label_recess",
        84.0,
        2.0,
        8.0,
    )
    .translate(
        -BRIDGE_X / 2.0 + 92.0,
        -(BRIDGE_Y / 2.0 + 20.0),
        BRIDGE_Z_CENTER + BRIDGE_Z / 2.0 + 6.0,
    );

    top_cover + front_radius + rear_radius - service_seam - washdown_label_recess
}

fn leak_tray_clearance_gauge() -> Part {
    let z = -LEAK_TRAY_CLEARANCE_Z - LEAK_TRAY_ENVELOPE_Z / 2.0;
    let front_bar = centered_cube(
        "robotic_cassette_gripper_leak_tray_clearance_front_gauge",
        LEAK_TRAY_ENVELOPE_X,
        KEEP_OUT_RAIL_W,
        LEAK_TRAY_ENVELOPE_Z,
    )
    .translate(0.0, -LEAK_TRAY_ENVELOPE_Y / 2.0, z);
    let rear_bar = centered_cube(
        "robotic_cassette_gripper_leak_tray_clearance_rear_gauge",
        LEAK_TRAY_ENVELOPE_X,
        KEEP_OUT_RAIL_W,
        LEAK_TRAY_ENVELOPE_Z,
    )
    .translate(0.0, LEAK_TRAY_ENVELOPE_Y / 2.0, z);
    let left_bar = centered_cube(
        "robotic_cassette_gripper_leak_tray_clearance_left_gauge",
        KEEP_OUT_RAIL_W,
        LEAK_TRAY_ENVELOPE_Y,
        LEAK_TRAY_ENVELOPE_Z,
    )
    .translate(-LEAK_TRAY_ENVELOPE_X / 2.0, 0.0, z);
    let right_bar = centered_cube(
        "robotic_cassette_gripper_leak_tray_clearance_right_gauge",
        KEEP_OUT_RAIL_W,
        LEAK_TRAY_ENVELOPE_Y,
        LEAK_TRAY_ENVELOPE_Z,
    )
    .translate(LEAK_TRAY_ENVELOPE_X / 2.0, 0.0, z);
    let drain_corner_flag = centered_cylinder(
        "robotic_cassette_gripper_leak_tray_drain_corner_flag",
        10.0,
        LEAK_TRAY_ENVELOPE_Z,
        36,
    )
    .translate(
        LEAK_TRAY_ENVELOPE_X / 2.0 - 28.0,
        -LEAK_TRAY_ENVELOPE_Y / 2.0 + 28.0,
        z,
    );

    front_bar + rear_bar + left_bar + right_bar + drain_corner_flag
}

fn collision_keepout_envelope() -> Part {
    let z = COLLISION_KEEP_OUT_Z / 2.0 - LEAK_TRAY_CLEARANCE_Z;
    let top_front = centered_cube(
        "robotic_cassette_gripper_collision_keepout_top_front",
        COLLISION_KEEP_OUT_X,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(
        0.0,
        -COLLISION_KEEP_OUT_Y / 2.0,
        z + COLLISION_KEEP_OUT_Z / 2.0,
    );
    let top_rear = centered_cube(
        "robotic_cassette_gripper_collision_keepout_top_rear",
        COLLISION_KEEP_OUT_X,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(
        0.0,
        COLLISION_KEEP_OUT_Y / 2.0,
        z + COLLISION_KEEP_OUT_Z / 2.0,
    );
    let top_left = centered_cube(
        "robotic_cassette_gripper_collision_keepout_top_left",
        KEEP_OUT_RAIL_W,
        COLLISION_KEEP_OUT_Y,
        KEEP_OUT_RAIL_W,
    )
    .translate(
        -COLLISION_KEEP_OUT_X / 2.0,
        0.0,
        z + COLLISION_KEEP_OUT_Z / 2.0,
    );
    let top_right = centered_cube(
        "robotic_cassette_gripper_collision_keepout_top_right",
        KEEP_OUT_RAIL_W,
        COLLISION_KEEP_OUT_Y,
        KEEP_OUT_RAIL_W,
    )
    .translate(
        COLLISION_KEEP_OUT_X / 2.0,
        0.0,
        z + COLLISION_KEEP_OUT_Z / 2.0,
    );

    let mut posts = Part::empty("robotic_cassette_gripper_collision_keepout_posts");
    for (i, (x, y)) in [
        (-COLLISION_KEEP_OUT_X / 2.0, -COLLISION_KEEP_OUT_Y / 2.0),
        (COLLISION_KEEP_OUT_X / 2.0, -COLLISION_KEEP_OUT_Y / 2.0),
        (-COLLISION_KEEP_OUT_X / 2.0, COLLISION_KEEP_OUT_Y / 2.0),
        (COLLISION_KEEP_OUT_X / 2.0, COLLISION_KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("robotic_cassette_gripper_collision_keepout_post_{i}"),
                KEEP_OUT_RAIL_W,
                KEEP_OUT_RAIL_W,
                COLLISION_KEEP_OUT_Z,
            )
            .translate(*x, *y, z);
    }

    let front_approach_notch = centered_cube(
        "robotic_cassette_gripper_collision_keepout_robot_approach_notch",
        160.0,
        KEEP_OUT_RAIL_W,
        30.0,
    )
    .translate(0.0, -COLLISION_KEEP_OUT_Y / 2.0, FINGER_Z_CENTER);

    top_front + top_rear + top_left + top_right + posts + front_approach_notch
}

fn rectangular_frame(
    name: &str,
    outer_x: f64,
    y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, y, outer_z);
    let inner = centered_cube(format!("{name}_inner_cut"), inner_x, y + 2.0, inner_z);
    outer - inner
}

fn wrist_bolt_points() -> Vec<(f64, f64)> {
    (0..WRIST_BOLT_COUNT)
        .map(|i| {
            let angle = i as f64 * TAU / WRIST_BOLT_COUNT as f64;
            let radius = WRIST_BOLT_CIRCLE_DIA / 2.0;
            (angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

fn sensor_bolt_points() -> Vec<(f64, f64)> {
    let s = SENSOR_BOLT_PITCH / 2.0;
    vec![(-s, -s), (s, -s), (-s, s), (s, s)]
}

fn datum_receiver_points() -> Vec<(f64, f64)> {
    vec![
        (-(CASSETTE_X / 2.0 - 42.0), -(CASSETTE_Y / 2.0 + 16.0)),
        (CASSETTE_X / 2.0 - 42.0, -(CASSETTE_Y / 2.0 + 16.0)),
        (-(CASSETTE_X / 2.0 - 42.0), CASSETTE_Y / 2.0 - 58.0),
    ]
}

fn pad_y(index: usize) -> f64 {
    let pitch = (CASSETTE_Y - 132.0) / (PAD_COUNT as f64 - 1.0);
    -(CASSETTE_Y - 132.0) / 2.0 + index as f64 * pitch
}

fn pad_center_x(sign: f64) -> f64 {
    sign * (PAD_INNER_SPAN_X / 2.0 + PAD_X / 2.0)
}

fn pad_center_z() -> f64 {
    FINGER_Z_CENTER - 12.0
}

fn finger_body_x(sign: f64) -> f64 {
    sign * (PAD_INNER_SPAN_X / 2.0 + PAD_X + FINGER_X / 2.0)
}

fn datum_receiver_z() -> f64 {
    pad_center_z() - PAD_Z / 2.0 - DATUM_BOSS_Z / 2.0 - 4.0
}

fn strain_relief_channel_x(index: usize) -> f64 {
    let pitch = 32.0;
    -((STRAIN_RELIEF_CHANNELS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-9;

    #[test]
    fn cassette_envelope_has_required_grip_and_leak_tray_clearance() {
        assert_eq!(COLS * ROWS, 20);
        assert!(PAD_INNER_SPAN_X >= CASSETTE_X + 6.0);
        assert!(FINGER_Y >= CASSETTE_Y + 30.0);
        assert!(SEALED_MODULE_Z < FINGER_Z + LEAK_TRAY_CLEARANCE_Z);
        assert!(LEAK_TRAY_ENVELOPE_X > CASSETTE_X + 90.0);
        assert!(LEAK_TRAY_ENVELOPE_Y > CASSETTE_Y + 80.0);
    }

    #[test]
    fn compliant_pads_are_symmetric_about_cassette_centerline() {
        for i in 0..PAD_COUNT {
            let left = (-pad_center_x(-1.0), pad_y(i), pad_center_z());
            let right = (pad_center_x(1.0), pad_y(i), pad_center_z());
            assert!((left.0 - right.0).abs() < EPS);
            assert!((left.1 - right.1).abs() < EPS);
            assert!((left.2 - right.2).abs() < EPS);
        }
        assert!(pad_y(0) < 0.0);
        assert!((pad_y(0) + pad_y(PAD_COUNT - 1)).abs() < EPS);
    }

    #[test]
    fn quick_change_wrist_pattern_is_centered_and_inside_plate() {
        let points = wrist_bolt_points();
        assert_eq!(points.len(), WRIST_BOLT_COUNT);
        for (i, (x, y)) in points.iter().enumerate() {
            let radius = (x.powi(2) + y.powi(2)).sqrt();
            assert!((radius - WRIST_BOLT_CIRCLE_DIA / 2.0).abs() < EPS);
            let opposite = points[(i + WRIST_BOLT_COUNT / 2) % WRIST_BOLT_COUNT];
            assert!((x + opposite.0).abs() < EPS);
            assert!((y + opposite.1).abs() < EPS);
            assert!(radius + WRIST_BOLT_HOLE_DIA / 2.0 < WRIST_PLATE_RADIUS - 6.0);
        }
    }

    #[test]
    fn cable_tube_keepout_stays_behind_cassette_and_respects_bend_radius() {
        let rear_of_cassette = CASSETTE_Y / 2.0;
        let strain_relief_front = CASSETTE_Y / 2.0 + 92.0 - STRAIN_RELIEF_Y / 2.0;
        assert!(strain_relief_front > rear_of_cassette + 50.0);
        assert!(MIN_TUBE_BEND_RADIUS >= 5.0 * STRAIN_RELIEF_CHANNEL_DIA);
        assert!(strain_relief_channel_x(0).abs() < STRAIN_RELIEF_X / 2.0 - 18.0);
        assert!(
            strain_relief_channel_x(STRAIN_RELIEF_CHANNELS - 1).abs()
                < STRAIN_RELIEF_X / 2.0 - 18.0
        );
    }

    #[test]
    fn datum_pin_engagement_is_clearanced_and_kinematic() {
        let points = datum_receiver_points();
        assert_eq!(points.len(), 3);
        assert!(DATUM_RECEIVER_DIA > DATUM_PIN_DIA);
        assert!((DATUM_RECEIVER_DIA - DATUM_PIN_DIA) >= 2.0 * DATUM_RECEIVER_CLEARANCE);

        let front_span = (points[0].0 - points[1].0).abs();
        assert!(front_span > CASSETTE_X - 96.0);
        assert!((points[0].1 - points[1].1).abs() < EPS);
        assert!(points[2].1 > points[0].1 + CASSETTE_Y * 0.55);

        for (x, y) in points {
            assert!(x.abs() < PAD_INNER_SPAN_X / 2.0 - 30.0);
            assert!(y.abs() < FINGER_Y / 2.0 + 6.0);
        }
    }
}
