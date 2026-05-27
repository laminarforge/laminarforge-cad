use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste-container neutralization/contact-time validation station.
//
// Design intent:
// - Hold closed cell-culture liquid waste modules in a contained cradle while
//   identity, neutralizer placeholder, pH/color evidence, sealed sample ports,
//   filtered vent placeholder, and release disposition are checked.
// - Make contact-time state physical by using a token lane that cannot be
//   confused with the release/hold/reject lanes.
// - Reserve camera, robot, and service envelopes so evidence capture and
//   cartridge/sample service have visible mechanical space.
//
// This is product-concept CAD for mechanical interfaces and bought-in
// neutralizer/filter/sample-port hardware. It is not a waste-treatment protocol,
// disinfectant validation, pH-release method, or bioburden claim.

// Stable STL exports for parent integration.
const OUTPUTS: [&str; 12] = [
    "output/closed_waste_container_neutralization_contact_time_station_secondary_containment_tray.stl",
    "output/closed_waste_container_neutralization_contact_time_station_waste_container_cradle.stl",
    "output/closed_waste_container_neutralization_contact_time_station_neutralizer_cartridge_placeholder.stl",
    "output/closed_waste_container_neutralization_contact_time_station_contact_time_token_lane.stl",
    "output/closed_waste_container_neutralization_contact_time_station_ph_color_indicator_coupon_pockets.stl",
    "output/closed_waste_container_neutralization_contact_time_station_sealed_sample_port_holders.stl",
    "output/closed_waste_container_neutralization_contact_time_station_filtered_vent_placeholder.stl",
    "output/closed_waste_container_neutralization_contact_time_station_barcode_custody_lands.stl",
    "output/closed_waste_container_neutralization_contact_time_station_release_hold_reject_lanes.stl",
    "output/closed_waste_container_neutralization_contact_time_station_evidence_camera_bridge.stl",
    "output/closed_waste_container_neutralization_contact_time_station_robot_service_keepouts.stl",
    "output/closed_waste_container_neutralization_contact_time_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "secondary_containment_tray",
    "waste_container_cradle",
    "neutralizer_cartridge_placeholder",
    "contact_time_token_lane",
    "ph_color_indicator_coupon_pockets",
    "sealed_sample_port_holders",
    "filtered_vent_placeholder",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATUS_LANES: [&str; 3] = ["release", "hold", "reject"];
const CONTACT_TOKENS: [&str; 6] = [
    "dose_start",
    "mix_confirmed",
    "timer_active",
    "contact_met",
    "ph_checked",
    "decision_locked",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 54.0;
const MOUNT_HOLE_D: f64 = 6.6;

const TRAY_RECESS_X: f64 = 1160.0;
const TRAY_RECESS_Y: f64 = 730.0;
const TRAY_RECESS_Z: f64 = 10.0;
const LEAK_SENSOR_WELLS: usize = 6;
const LEAK_SENSOR_WELL_X: f64 = 64.0;
const LEAK_SENSOR_WELL_Y: f64 = 36.0;
const DRAIN_PORT_D: f64 = 16.0;

const CRADLE_X: f64 = 520.0;
const CRADLE_Y: f64 = 330.0;
const CRADLE_Z: f64 = 92.0;
const CONTAINER_D: f64 = 188.0;
const CONTAINER_SADDLES: usize = 2;
const CRADLE_LATCHES: usize = 4;
const CRADLE_DATUM_PINS: usize = 4;

const NEUTRALIZER_DOCK_X: f64 = 370.0;
const NEUTRALIZER_DOCK_Y: f64 = 205.0;
const NEUTRALIZER_DOCK_Z: f64 = 52.0;
const NEUTRALIZER_CARTRIDGES: usize = 3;
const NEUTRALIZER_CARTRIDGE_D: f64 = 44.0;
const NEUTRALIZER_CARTRIDGE_Z: f64 = 138.0;
const NEUTRALIZER_PORTS: usize = 4;

const TOKEN_LANE_X: f64 = 735.0;
const TOKEN_LANE_Y: f64 = 132.0;
const TOKEN_LANE_Z: f64 = 28.0;
const TOKEN_SLOT_X: f64 = 82.0;
const TOKEN_SLOT_Y: f64 = 58.0;
const TOKEN_SLOT_Z: f64 = 12.0;
const TOKEN_PITCH_X: f64 = 108.0;

const COUPON_BLOCK_X: f64 = 430.0;
const COUPON_BLOCK_Y: f64 = 208.0;
const COUPON_BLOCK_Z: f64 = 40.0;
const COUPON_COUNT: usize = 8;
const COUPON_COLS: usize = 4;
const COUPON_POCKET_X: f64 = 76.0;
const COUPON_POCKET_Y: f64 = 38.0;
const COUPON_POCKET_Z: f64 = 14.0;
const COLOR_REFERENCE_TILES: usize = 6;

const SAMPLE_PANEL_X: f64 = 395.0;
const SAMPLE_PANEL_Y: f64 = 178.0;
const SAMPLE_PANEL_Z: f64 = 50.0;
const SAMPLE_PORTS: usize = 6;
const SAMPLE_PORT_D: f64 = 25.0;
const SAMPLE_CAP_POCKETS: usize = 6;

const VENT_PANEL_X: f64 = 270.0;
const VENT_PANEL_Y: f64 = 155.0;
const VENT_PANEL_Z: f64 = 178.0;
const VENT_FILTERS: usize = 2;
const VENT_FILTER_D: f64 = 48.0;
const VENT_FILTER_LEN: f64 = 210.0;
const VENT_BARBS: usize = 4;

const CUSTODY_PANEL_X: f64 = 510.0;
const CUSTODY_PANEL_Y: f64 = 188.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 6;
const CUSTODY_CARD_SLOTS: usize = 5;

const DISPOSITION_LANE_X: f64 = 255.0;
const DISPOSITION_LANE_Y: f64 = 250.0;
const DISPOSITION_LANE_Z: f64 = 42.0;
const DISPOSITION_LANE_PITCH_X: f64 = 300.0;
const DISPOSITION_TOKENS_PER_LANE: usize = 4;

const CAMERA_BRIDGE_SPAN_X: f64 = 1180.0;
const CAMERA_BRIDGE_POST_X: f64 = 30.0;
const CAMERA_BRIDGE_POST_Y: f64 = 40.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 260.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_COUNT: usize = 4;
const LED_STRIPS: usize = 8;

const KEEP_OUT_RAIL_W: f64 = 12.0;
const KEEP_OUT_Z: f64 = 92.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 420.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 250.0;
const LEFT_CARTRIDGE_SERVICE_KEEP_OUT_X: f64 = 230.0;
const RIGHT_SAMPLE_SERVICE_KEEP_OUT_X: f64 = 220.0;
const OVERHEAD_CAMERA_KEEP_OUT_Z: f64 = 335.0;

const CRADLE_CENTER: (f64, f64) = (-340.0, 120.0);
const NEUTRALIZER_CENTER: (f64, f64) = (340.0, 210.0);
const TOKEN_CENTER: (f64, f64) = (150.0, -20.0);
const COUPON_CENTER: (f64, f64) = (-395.0, -250.0);
const SAMPLE_CENTER: (f64, f64) = (-15.0, -268.0);
const VENT_CENTER: (f64, f64) = (495.0, 60.0);
const CUSTODY_CENTER: (f64, f64) = (385.0, -262.0);
const DISPOSITION_CENTER: (f64, f64) = (105.0, -285.0);
const CAMERA_BRIDGE_CENTER: (f64, f64) = (0.0, 12.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = secondary_containment_tray();
    export(&tray, OUTPUTS[0]);

    let cradle = waste_container_cradle();
    export(&cradle, OUTPUTS[1]);

    let neutralizer = neutralizer_cartridge_placeholder();
    export(&neutralizer, OUTPUTS[2]);

    let token_lane = contact_time_token_lane();
    export(&token_lane, OUTPUTS[3]);

    let coupons = ph_color_indicator_coupon_pockets();
    export(&coupons, OUTPUTS[4]);

    let sample_ports = sealed_sample_port_holders();
    export(&sample_ports, OUTPUTS[5]);

    let vent = filtered_vent_placeholder();
    export(&vent, OUTPUTS[6]);

    let custody = barcode_custody_lands();
    export(&custody, OUTPUTS[7]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[8]);

    let camera_bridge = evidence_camera_bridge();
    export(&camera_bridge, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = tray
        + cradle.translate(CRADLE_CENTER.0, CRADLE_CENTER.1, deck_insert_z(CRADLE_Z))
        + neutralizer.translate(
            NEUTRALIZER_CENTER.0,
            NEUTRALIZER_CENTER.1,
            deck_insert_z(NEUTRALIZER_DOCK_Z),
        )
        + token_lane.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_insert_z(TOKEN_LANE_Z))
        + coupons.translate(
            COUPON_CENTER.0,
            COUPON_CENTER.1,
            deck_insert_z(COUPON_BLOCK_Z),
        )
        + sample_ports.translate(
            SAMPLE_CENTER.0,
            SAMPLE_CENTER.1,
            deck_insert_z(SAMPLE_PANEL_Z),
        )
        + vent.translate(VENT_CENTER.0, VENT_CENTER.1, deck_insert_z(VENT_PANEL_Z))
        + custody.translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            deck_insert_z(CUSTODY_PANEL_Z),
        )
        + disposition.translate(
            DISPOSITION_CENTER.0,
            DISPOSITION_CENTER.1,
            deck_insert_z(DISPOSITION_LANE_Z),
        )
        + camera_bridge.translate(
            CAMERA_BRIDGE_CENTER.0,
            CAMERA_BRIDGE_CENTER.1,
            deck_insert_z(CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z),
        )
        + keepouts.translate(0.0, 0.0, deck_insert_z(KEEP_OUT_Z));
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed waste-container neutralization/contact-time validation station:");
    println!(
        "  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm deck with {TRAY_RECESS_X:.0}mm x {TRAY_RECESS_Y:.0}mm secondary containment tray, {LEAK_SENSOR_WELLS} leak wells, and {DRAIN_PORT_D:.0}mm drain interface"
    );
    println!(
        "  Waste module handling:       {CONTAINER_SADDLES} closed-container saddle positions, {CRADLE_LATCHES} latch lands, {CRADLE_DATUM_PINS} datum pins"
    );
    println!(
        "  Neutralization interface:    {NEUTRALIZER_CARTRIDGES} cartridge placeholders, {NEUTRALIZER_PORTS} keyed wet-port lands, {VENT_FILTERS} filtered vent placeholders, {VENT_BARBS} vent barb lands"
    );
    println!(
        "  Evidence workflow:           {} contact-time tokens, {COUPON_COUNT} pH/color coupon pockets, {SAMPLE_PORTS} sealed sample port holders, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands",
        CONTACT_TOKENS.len()
    );
    println!(
        "  Disposition routing:         release/hold/reject lanes with {DISPOSITION_TOKENS_PER_LANE} token positions per lane"
    );
    println!(
        "  Robot/service envelopes:     front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, cartridge side {LEFT_CARTRIDGE_SERVICE_KEEP_OUT_X:.0}mm, sample side {RIGHT_SAMPLE_SERVICE_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_CAMERA_KEEP_OUT_Z:.0}mm"
    );
    println!(
        "  Placeholder warning:         neutralizer, filtered vent, pH/color, and contact-time features are mechanical reservation geometry only."
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(STATUS_LANES.len(), 3);
    assert_eq!(CONTACT_TOKENS.len(), 6);
    assert_eq!(COUPON_COUNT, COUPON_COLS * 2);
    assert_eq!(SAMPLE_PORTS, SAMPLE_CAP_POCKETS);
    assert_eq!(DISPOSITION_LANE_PITCH_X, 300.0);
    assert!(left_edge(CRADLE_CENTER.0, CRADLE_X) > -STATION_X / 2.0 + 50.0);
    assert!(right_edge(VENT_CENTER.0, VENT_PANEL_X) < STATION_X / 2.0 - 20.0);
    assert!(bottom_edge(DISPOSITION_CENTER.1, DISPOSITION_LANE_Y) > -STATION_Y / 2.0 + 20.0);
    assert!(top_edge(NEUTRALIZER_CENTER.1, NEUTRALIZER_DOCK_Y) < STATION_Y / 2.0 - 70.0);
    assert!(right_edge(DISPOSITION_CENTER.0, disposition_lane_span_x()) < STATION_X / 2.0 - 50.0);
    assert!(BARCODE_LANDS >= STATUS_LANES.len() * DISPOSITION_TOKENS_PER_LANE);
    assert!(RFID_LANDS >= STATUS_LANES.len() + VENT_FILTERS);
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z + 10.0 + height / 2.0
}

fn left_edge(center: f64, width: f64) -> f64 {
    center - width / 2.0
}

fn right_edge(center: f64, width: f64) -> f64 {
    center + width / 2.0
}

fn bottom_edge(center: f64, depth: f64) -> f64 {
    center - depth / 2.0
}

fn top_edge(center: f64, depth: f64) -> f64 {
    center + depth / 2.0
}

fn disposition_lane_span_x() -> f64 {
    DISPOSITION_LANE_X + (STATUS_LANES.len() - 1) as f64 * DISPOSITION_LANE_PITCH_X
}

fn secondary_containment_tray() -> Part {
    let deck = centered_cube(
        "closed_waste_neutralization_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let tray_recess = centered_cube(
        "closed_waste_neutralization_secondary_tray_recess",
        TRAY_RECESS_X,
        TRAY_RECESS_Y,
        TRAY_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z - TRAY_RECESS_Z / 2.0);
    let front_drain = centered_cylinder(
        "closed_waste_neutralization_front_drain_port",
        DRAIN_PORT_D / 2.0,
        60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 25.0, BASE_Z - 6.0);

    deck - tray_recess - front_drain - station_mount_holes()
        + containment_rim()
        + tray_flow_ribs()
        + leak_sensor_wells()
        + module_zone_datum_lands()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        "closed_waste_neutralization_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_waste_neutralization_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_waste_neutralization_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_waste_neutralization_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn station_mount_holes() -> Part {
    let mut holes = Part::empty("closed_waste_neutralization_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 52.0),
        (0.0, -STATION_Y / 2.0 + 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_waste_neutralization_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn tray_flow_ribs() -> Part {
    let mut ribs = Part::empty("closed_waste_neutralization_tray_flow_ribs");
    for (i, y) in [-285.0, -170.0, -55.0, 60.0, 175.0, 290.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_waste_neutralization_drip_flow_rib_{i}"),
                TRAY_RECESS_X - 130.0,
                8.0,
                7.0,
            )
            .translate(0.0, *y, BASE_Z + 3.5);
    }
    ribs
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_waste_neutralization_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = -410.0 + (i % 3) as f64 * 410.0;
        let y = -330.0 + (i / 3) as f64 * 660.0;
        wells = wells
            + centered_cube(
                format!("closed_waste_neutralization_leak_sensor_pocket_{i}"),
                LEAK_SENSOR_WELL_X,
                LEAK_SENSOR_WELL_Y,
                7.0,
            )
            .translate(x, y, BASE_Z + 3.5)
            + centered_cube(
                format!("closed_waste_neutralization_leak_sensor_wire_race_{i}"),
                96.0,
                7.0,
                7.0,
            )
            .translate(x, y + 30.0, BASE_Z + 3.5);
    }
    wells
}

fn module_zone_datum_lands() -> Part {
    let cradle_land = centered_cube(
        "closed_waste_neutralization_waste_module_datum_land",
        CRADLE_X + 56.0,
        CRADLE_Y + 46.0,
        6.0,
    )
    .translate(CRADLE_CENTER.0, CRADLE_CENTER.1, BASE_Z + 3.0);
    let neutralizer_land = centered_cube(
        "closed_waste_neutralization_neutralizer_datum_land",
        NEUTRALIZER_DOCK_X + 42.0,
        NEUTRALIZER_DOCK_Y + 42.0,
        6.0,
    )
    .translate(NEUTRALIZER_CENTER.0, NEUTRALIZER_CENTER.1, BASE_Z + 3.0);
    let sample_land = centered_cube(
        "closed_waste_neutralization_sample_panel_datum_land",
        SAMPLE_PANEL_X + 36.0,
        SAMPLE_PANEL_Y + 36.0,
        6.0,
    )
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, BASE_Z + 3.0);
    cradle_land + neutralizer_land + sample_land
}

fn waste_container_cradle() -> Part {
    let base = centered_cube(
        "closed_waste_neutralization_cradle_base",
        CRADLE_X,
        CRADLE_Y,
        CRADLE_Z,
    );
    let service_window = centered_cube(
        "closed_waste_neutralization_cradle_front_service_window",
        CRADLE_X - 90.0,
        36.0,
        CRADLE_Z - 34.0,
    )
    .translate(0.0, -CRADLE_Y / 2.0 + 18.0, 4.0);

    base - container_saddle_cuts() - service_window
        + cradle_side_rails()
        + cradle_latches()
        + cradle_datum_pins()
        + container_id_witness_lands()
}

fn container_saddle_cuts() -> Part {
    let mut cuts = Part::empty("closed_waste_neutralization_container_saddle_cuts");
    for (i, x) in [-118.0, 118.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("closed_waste_neutralization_container_radius_saddle_cut_{i}"),
                CONTAINER_D / 2.0,
                CRADLE_Y + 24.0,
                64,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, CRADLE_Z / 2.0 + 24.0);
    }
    cuts
}

fn cradle_side_rails() -> Part {
    let left = centered_cube(
        "closed_waste_neutralization_cradle_left_retention_rail",
        26.0,
        CRADLE_Y + 36.0,
        52.0,
    )
    .translate(-CRADLE_X / 2.0 + 28.0, 0.0, CRADLE_Z / 2.0 + 16.0);
    let right = centered_cube(
        "closed_waste_neutralization_cradle_right_retention_rail",
        26.0,
        CRADLE_Y + 36.0,
        52.0,
    )
    .translate(CRADLE_X / 2.0 - 28.0, 0.0, CRADLE_Z / 2.0 + 16.0);
    let rear_stop = centered_cube(
        "closed_waste_neutralization_cradle_rear_closed_container_stop",
        CRADLE_X - 72.0,
        24.0,
        68.0,
    )
    .translate(0.0, CRADLE_Y / 2.0 - 12.0, CRADLE_Z / 2.0 + 18.0);
    let front_lip = centered_cube(
        "closed_waste_neutralization_cradle_front_drip_lip",
        CRADLE_X - 86.0,
        20.0,
        34.0,
    )
    .translate(0.0, -CRADLE_Y / 2.0 + 10.0, CRADLE_Z / 2.0 + 5.0);
    left + right + rear_stop + front_lip
}

fn cradle_latches() -> Part {
    let mut latches = Part::empty("closed_waste_neutralization_cradle_latch_lands");
    for (i, (x, y)) in [
        (-210.0, -130.0),
        (210.0, -130.0),
        (-210.0, 130.0),
        (210.0, 130.0),
    ]
    .iter()
    .enumerate()
    {
        latches = latches
            + centered_cube(
                format!("closed_waste_neutralization_cradle_toggle_latch_land_{i}"),
                84.0,
                24.0,
                18.0,
            )
            .translate(*x, *y, CRADLE_Z / 2.0 + 35.0)
            + centered_cylinder(
                format!("closed_waste_neutralization_cradle_latch_pivot_{i}"),
                6.0,
                82.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(*x, *y, CRADLE_Z / 2.0 + 48.0);
    }
    latches
}

fn cradle_datum_pins() -> Part {
    let mut pins = Part::empty("closed_waste_neutralization_cradle_datum_pins");
    for (i, (x, y)) in [
        (-196.0, -96.0),
        (196.0, -96.0),
        (-196.0, 96.0),
        (196.0, 96.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_waste_neutralization_cradle_datum_pin_{i}"),
                7.0,
                24.0,
                24,
            )
            .translate(*x, *y, CRADLE_Z / 2.0 + 12.0);
    }
    pins
}

fn container_id_witness_lands() -> Part {
    let mut lands = Part::empty("closed_waste_neutralization_container_id_witness_lands");
    for (i, x) in [-118.0, 118.0].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("closed_waste_neutralization_container_label_flat_{i}"),
                136.0,
                34.0,
                6.0,
            )
            .translate(*x, -CRADLE_Y / 2.0 - 22.0, CRADLE_Z / 2.0 + 10.0);
    }
    lands
}

fn neutralizer_cartridge_placeholder() -> Part {
    let dock = centered_cube(
        "closed_waste_neutralization_neutralizer_cartridge_dock",
        NEUTRALIZER_DOCK_X,
        NEUTRALIZER_DOCK_Y,
        NEUTRALIZER_DOCK_Z,
    );
    let socket_cuts = neutralizer_socket_cuts();

    dock - socket_cuts
        + neutralizer_cartridges()
        + neutralizer_wet_port_lands()
        + neutralizer_keyed_cover_tabs()
}

fn neutralizer_socket_cuts() -> Part {
    let mut cuts = Part::empty("closed_waste_neutralization_neutralizer_socket_cuts");
    for i in 0..NEUTRALIZER_CARTRIDGES {
        let x = -112.0 + i as f64 * 112.0;
        cuts = cuts
            + centered_cylinder(
                format!("closed_waste_neutralization_neutralizer_socket_cut_{i}"),
                NEUTRALIZER_CARTRIDGE_D / 2.0 + 6.0,
                20.0,
                40,
            )
            .translate(x, 12.0, NEUTRALIZER_DOCK_Z / 2.0 - 5.0);
    }
    cuts
}

fn neutralizer_cartridges() -> Part {
    let mut cartridges = Part::empty("closed_waste_neutralization_neutralizer_cartridge_bodies");
    for i in 0..NEUTRALIZER_CARTRIDGES {
        let x = -112.0 + i as f64 * 112.0;
        cartridges = cartridges
            + centered_cylinder(
                format!("closed_waste_neutralization_neutralizer_placeholder_cartridge_{i}"),
                NEUTRALIZER_CARTRIDGE_D / 2.0,
                NEUTRALIZER_CARTRIDGE_Z,
                48,
            )
            .translate(
                x,
                12.0,
                NEUTRALIZER_DOCK_Z / 2.0 + NEUTRALIZER_CARTRIDGE_Z / 2.0 - 4.0,
            )
            + centered_cylinder(
                format!("closed_waste_neutralization_neutralizer_top_cap_{i}"),
                NEUTRALIZER_CARTRIDGE_D / 2.0 + 5.0,
                14.0,
                48,
            )
            .translate(
                x,
                12.0,
                NEUTRALIZER_DOCK_Z / 2.0 + NEUTRALIZER_CARTRIDGE_Z + 8.0,
            );
    }
    cartridges
}

fn neutralizer_wet_port_lands() -> Part {
    let mut ports = Part::empty("closed_waste_neutralization_neutralizer_wet_port_lands");
    for i in 0..NEUTRALIZER_PORTS {
        let x = -144.0 + i as f64 * 96.0;
        ports = ports
            + centered_cube(
                format!("closed_waste_neutralization_keyed_neutralizer_wet_port_land_{i}"),
                58.0,
                34.0,
                18.0,
            )
            .translate(
                x,
                -NEUTRALIZER_DOCK_Y / 2.0 - 22.0,
                NEUTRALIZER_DOCK_Z / 2.0 + 9.0,
            )
            + centered_cylinder(
                format!("closed_waste_neutralization_wet_port_boss_{i}"),
                8.0,
                18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                -NEUTRALIZER_DOCK_Y / 2.0 - 40.0,
                NEUTRALIZER_DOCK_Z / 2.0 + 9.0,
            );
    }
    ports
}

fn neutralizer_keyed_cover_tabs() -> Part {
    let left = centered_cube(
        "closed_waste_neutralization_neutralizer_left_keyed_cover_tab",
        28.0,
        NEUTRALIZER_DOCK_Y + 18.0,
        22.0,
    )
    .translate(
        -NEUTRALIZER_DOCK_X / 2.0 + 28.0,
        0.0,
        NEUTRALIZER_DOCK_Z / 2.0 + 18.0,
    );
    let right = centered_cube(
        "closed_waste_neutralization_neutralizer_right_keyed_cover_tab",
        28.0,
        NEUTRALIZER_DOCK_Y + 18.0,
        22.0,
    )
    .translate(
        NEUTRALIZER_DOCK_X / 2.0 - 28.0,
        0.0,
        NEUTRALIZER_DOCK_Z / 2.0 + 18.0,
    );
    let custody_slot = centered_cube(
        "closed_waste_neutralization_neutralizer_lot_card_land",
        190.0,
        28.0,
        8.0,
    )
    .translate(
        0.0,
        NEUTRALIZER_DOCK_Y / 2.0 + 28.0,
        NEUTRALIZER_DOCK_Z / 2.0 + 6.0,
    );
    left + right + custody_slot
}

fn contact_time_token_lane() -> Part {
    let lane = centered_cube(
        "closed_waste_neutralization_contact_time_token_lane_base",
        TOKEN_LANE_X,
        TOKEN_LANE_Y,
        TOKEN_LANE_Z,
    );
    lane - contact_token_recesses()
        + contact_token_discs()
        + contact_lane_rails()
        + timer_reader_land()
}

fn contact_token_recesses() -> Part {
    let mut recesses = Part::empty("closed_waste_neutralization_contact_token_recesses");
    for i in 0..CONTACT_TOKENS.len() {
        let x = token_x(i);
        recesses = recesses
            + centered_cube(
                format!(
                    "closed_waste_neutralization_{}_token_recess",
                    CONTACT_TOKENS[i]
                ),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_SLOT_Z,
            )
            .translate(x, 0.0, TOKEN_LANE_Z / 2.0 - TOKEN_SLOT_Z / 2.0 + 2.0);
    }
    recesses
}

fn contact_token_discs() -> Part {
    let mut discs = Part::empty("closed_waste_neutralization_contact_token_discs");
    for i in 0..CONTACT_TOKENS.len() {
        let x = token_x(i);
        discs = discs
            + centered_cylinder(
                format!(
                    "closed_waste_neutralization_{}_physical_token",
                    CONTACT_TOKENS[i]
                ),
                19.0,
                7.0,
                32,
            )
            .translate(x, 0.0, TOKEN_LANE_Z / 2.0 + 3.5);
    }
    discs
}

fn contact_lane_rails() -> Part {
    let front = centered_cube(
        "closed_waste_neutralization_contact_token_front_rail",
        TOKEN_LANE_X,
        12.0,
        34.0,
    )
    .translate(0.0, -TOKEN_LANE_Y / 2.0 + 6.0, 17.0);
    let rear = centered_cube(
        "closed_waste_neutralization_contact_token_rear_rail",
        TOKEN_LANE_X,
        12.0,
        34.0,
    )
    .translate(0.0, TOKEN_LANE_Y / 2.0 - 6.0, 17.0);
    let entry_gate = centered_cube(
        "closed_waste_neutralization_contact_token_one_way_entry_gate",
        24.0,
        TOKEN_LANE_Y,
        44.0,
    )
    .translate(-TOKEN_LANE_X / 2.0 + 26.0, 0.0, 22.0);
    let exit_gate = centered_cube(
        "closed_waste_neutralization_contact_token_release_interlock_gate",
        24.0,
        TOKEN_LANE_Y,
        44.0,
    )
    .translate(TOKEN_LANE_X / 2.0 - 26.0, 0.0, 22.0);
    front + rear + entry_gate + exit_gate
}

fn timer_reader_land() -> Part {
    centered_cube(
        "closed_waste_neutralization_contact_time_timer_reader_land",
        112.0,
        76.0,
        9.0,
    )
    .translate(TOKEN_LANE_X / 2.0 + 72.0, 0.0, TOKEN_LANE_Z / 2.0 + 4.5)
}

fn token_x(index: usize) -> f64 {
    (index as f64 - (CONTACT_TOKENS.len() as f64 - 1.0) / 2.0) * TOKEN_PITCH_X
}

fn ph_color_indicator_coupon_pockets() -> Part {
    let block = centered_cube(
        "closed_waste_neutralization_ph_color_coupon_block",
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
        COUPON_BLOCK_Z,
    );
    block - coupon_pocket_cuts()
        + coupon_retention_lips()
        + color_reference_tiles()
        + coupon_drain_gutter()
}

fn coupon_pocket_cuts() -> Part {
    let mut pockets = Part::empty("closed_waste_neutralization_ph_coupon_pocket_cuts");
    let rows = COUPON_COUNT / COUPON_COLS;
    for i in 0..COUPON_COUNT {
        let col = i % COUPON_COLS;
        let row = i / COUPON_COLS;
        let x = (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * 92.0;
        let y = (row as f64 - (rows as f64 - 1.0) / 2.0) * 72.0;
        pockets = pockets
            + centered_cube(
                format!("closed_waste_neutralization_ph_color_coupon_pocket_{i}"),
                COUPON_POCKET_X,
                COUPON_POCKET_Y,
                COUPON_POCKET_Z,
            )
            .translate(x, y, COUPON_BLOCK_Z / 2.0 - COUPON_POCKET_Z / 2.0 + 3.0);
    }
    pockets
}

fn coupon_retention_lips() -> Part {
    let mut lips = Part::empty("closed_waste_neutralization_coupon_retention_lips");
    for i in 0..COUPON_COUNT {
        let col = i % COUPON_COLS;
        let row = i / COUPON_COLS;
        let x = (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * 92.0;
        let y = (row as f64 - 0.5) * 72.0;
        lips = lips
            + centered_cube(
                format!("closed_waste_neutralization_coupon_retention_lip_{i}"),
                COUPON_POCKET_X + 10.0,
                5.0,
                6.0,
            )
            .translate(
                x,
                y + COUPON_POCKET_Y / 2.0 + 5.0,
                COUPON_BLOCK_Z / 2.0 + 3.0,
            );
    }
    lips
}

fn color_reference_tiles() -> Part {
    let mut tiles = Part::empty("closed_waste_neutralization_color_reference_tiles");
    for i in 0..COLOR_REFERENCE_TILES {
        tiles = tiles
            + centered_cube(
                format!("closed_waste_neutralization_color_reference_tile_{i}"),
                34.0,
                24.0,
                5.0,
            )
            .translate(
                -COUPON_BLOCK_X / 2.0 + 42.0 + i as f64 * 42.0,
                -COUPON_BLOCK_Y / 2.0 - 26.0,
                COUPON_BLOCK_Z / 2.0 + 2.5,
            );
    }
    tiles
}

fn coupon_drain_gutter() -> Part {
    centered_cube(
        "closed_waste_neutralization_ph_coupon_low_volume_drain_gutter",
        COUPON_BLOCK_X - 60.0,
        12.0,
        8.0,
    )
    .translate(0.0, COUPON_BLOCK_Y / 2.0 + 18.0, COUPON_BLOCK_Z / 2.0 + 4.0)
}

fn sealed_sample_port_holders() -> Part {
    let panel = centered_cube(
        "closed_waste_neutralization_sealed_sample_port_panel",
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Y,
        SAMPLE_PANEL_Z,
    );
    panel - sample_port_socket_cuts()
        + sample_port_cups()
        + sample_cap_pockets()
        + sample_tamper_tabs()
}

fn sample_port_socket_cuts() -> Part {
    let mut cuts = Part::empty("closed_waste_neutralization_sample_port_socket_cuts");
    for i in 0..SAMPLE_PORTS {
        let x = -150.0 + (i % 3) as f64 * 150.0;
        let y = -40.0 + (i / 3) as f64 * 80.0;
        cuts = cuts
            + centered_cylinder(
                format!("closed_waste_neutralization_sample_port_socket_cut_{i}"),
                SAMPLE_PORT_D / 2.0 + 6.0,
                26.0,
                32,
            )
            .translate(x, y, SAMPLE_PANEL_Z / 2.0 - 8.0);
    }
    cuts
}

fn sample_port_cups() -> Part {
    let mut cups = Part::empty("closed_waste_neutralization_sample_port_cups");
    for i in 0..SAMPLE_PORTS {
        let x = -150.0 + (i % 3) as f64 * 150.0;
        let y = -40.0 + (i / 3) as f64 * 80.0;
        let outer = centered_cylinder(
            format!("closed_waste_neutralization_sample_port_outer_cup_{i}"),
            SAMPLE_PORT_D / 2.0 + 10.0,
            24.0,
            36,
        )
        .translate(x, y, SAMPLE_PANEL_Z / 2.0 + 12.0);
        let inner = centered_cylinder(
            format!("closed_waste_neutralization_sample_port_inner_socket_{i}"),
            SAMPLE_PORT_D / 2.0,
            28.0,
            36,
        )
        .translate(x, y, SAMPLE_PANEL_Z / 2.0 + 12.0);
        cups = cups + (outer - inner);
    }
    cups
}

fn sample_cap_pockets() -> Part {
    let mut pockets = Part::empty("closed_waste_neutralization_sample_cap_pockets");
    for i in 0..SAMPLE_CAP_POCKETS {
        let x = -160.0 + i as f64 * 64.0;
        pockets = pockets
            + centered_cylinder(
                format!("closed_waste_neutralization_sample_cap_parking_pocket_{i}"),
                12.0,
                10.0,
                24,
            )
            .translate(x, SAMPLE_PANEL_Y / 2.0 + 26.0, SAMPLE_PANEL_Z / 2.0 + 5.0);
    }
    pockets
}

fn sample_tamper_tabs() -> Part {
    let left = centered_cube(
        "closed_waste_neutralization_sample_left_tamper_witness_tab",
        24.0,
        SAMPLE_PANEL_Y + 26.0,
        18.0,
    )
    .translate(
        -SAMPLE_PANEL_X / 2.0 + 22.0,
        0.0,
        SAMPLE_PANEL_Z / 2.0 + 9.0,
    );
    let right = centered_cube(
        "closed_waste_neutralization_sample_right_tamper_witness_tab",
        24.0,
        SAMPLE_PANEL_Y + 26.0,
        18.0,
    )
    .translate(SAMPLE_PANEL_X / 2.0 - 22.0, 0.0, SAMPLE_PANEL_Z / 2.0 + 9.0);
    left + right
}

fn filtered_vent_placeholder() -> Part {
    let panel = centered_cube(
        "closed_waste_neutralization_filtered_vent_mount_panel",
        VENT_PANEL_X,
        VENT_PANEL_Y,
        VENT_PANEL_Z,
    );
    panel + vent_filter_bodies() + vent_manifold_barbs() + vent_condensate_trap_land()
        - vent_panel_lightening_window()
}

fn vent_filter_bodies() -> Part {
    let mut filters = Part::empty("closed_waste_neutralization_filtered_vent_bodies");
    for i in 0..VENT_FILTERS {
        let z = -34.0 + i as f64 * 82.0;
        filters = filters
            + centered_cylinder(
                format!("closed_waste_neutralization_filtered_vent_placeholder_filter_{i}"),
                VENT_FILTER_D / 2.0,
                VENT_FILTER_LEN,
                48,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, -VENT_PANEL_Y / 2.0 - 16.0, z)
            + centered_cube(
                format!("closed_waste_neutralization_filtered_vent_filter_clamp_{i}"),
                VENT_FILTER_LEN + 24.0,
                16.0,
                18.0,
            )
            .translate(0.0, -VENT_PANEL_Y / 2.0 - 16.0, z);
    }
    filters
}

fn vent_manifold_barbs() -> Part {
    let mut barbs = Part::empty("closed_waste_neutralization_vent_barb_lands");
    for i in 0..VENT_BARBS {
        let x = -92.0 + i as f64 * 61.0;
        barbs = barbs
            + centered_cylinder(
                format!("closed_waste_neutralization_vent_barb_land_{i}"),
                8.0,
                26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, VENT_PANEL_Y / 2.0 + 18.0, -VENT_PANEL_Z / 2.0 + 36.0);
    }
    barbs
}

fn vent_condensate_trap_land() -> Part {
    centered_cube(
        "closed_waste_neutralization_filtered_vent_condensate_trap_land",
        138.0,
        40.0,
        48.0,
    )
    .translate(0.0, VENT_PANEL_Y / 2.0 + 30.0, -VENT_PANEL_Z / 2.0 + 68.0)
}

fn vent_panel_lightening_window() -> Part {
    centered_cube(
        "closed_waste_neutralization_filtered_vent_panel_lightening_window",
        VENT_PANEL_X - 90.0,
        VENT_PANEL_Y + 8.0,
        VENT_PANEL_Z - 76.0,
    )
    .translate(0.0, 0.0, 14.0)
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_waste_neutralization_barcode_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    panel + barcode_lands() + rfid_lands() + custody_card_slots() + custody_tamper_strip_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_waste_neutralization_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = -190.0 + (i % 4) as f64 * 126.0;
        let y = -52.0 + (i / 4) as f64 * 48.0;
        lands = lands
            + centered_cube(
                format!("closed_waste_neutralization_barcode_label_land_{i}"),
                98.0,
                26.0,
                5.0,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + 2.5);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("closed_waste_neutralization_rfid_lands");
    for i in 0..RFID_LANDS {
        let x = -150.0 + (i % 3) as f64 * 150.0;
        let y = 88.0 + (i / 3) as f64 * 42.0;
        lands = lands
            + centered_cube(
                format!("closed_waste_neutralization_rfid_tag_land_{i}"),
                72.0,
                42.0,
                5.0,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + 2.5);
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty("closed_waste_neutralization_custody_card_slots");
    for i in 0..CUSTODY_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_waste_neutralization_custody_card_slot_{i}"),
                70.0,
                12.0,
                20.0,
            )
            .translate(
                -170.0 + i as f64 * 85.0,
                -CUSTODY_PANEL_Y / 2.0 - 24.0,
                CUSTODY_PANEL_Z / 2.0 + 8.0,
            );
    }
    slots
}

fn custody_tamper_strip_lands() -> Part {
    let left = centered_cube(
        "closed_waste_neutralization_custody_left_tamper_strip_land",
        164.0,
        18.0,
        6.0,
    )
    .translate(
        -128.0,
        CUSTODY_PANEL_Y / 2.0 + 22.0,
        CUSTODY_PANEL_Z / 2.0 + 3.0,
    );
    let right = centered_cube(
        "closed_waste_neutralization_custody_right_tamper_strip_land",
        164.0,
        18.0,
        6.0,
    )
    .translate(
        128.0,
        CUSTODY_PANEL_Y / 2.0 + 22.0,
        CUSTODY_PANEL_Z / 2.0 + 3.0,
    );
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("closed_waste_neutralization_release_hold_reject_lanes");
    for (i, name) in STATUS_LANES.iter().enumerate() {
        let x = (i as f64 - 1.0) * DISPOSITION_LANE_PITCH_X;
        lanes = lanes
            + disposition_lane(name).translate(x, 0.0, 0.0)
            + disposition_gate(name).translate(x, DISPOSITION_LANE_Y / 2.0 + 26.0, 34.0);
    }
    lanes + disposition_common_handoff_rail()
}

fn disposition_lane(name: &str) -> Part {
    let lane = centered_cube(
        format!("closed_waste_neutralization_{name}_lane_base"),
        DISPOSITION_LANE_X,
        DISPOSITION_LANE_Y,
        DISPOSITION_LANE_Z,
    );
    let rail_left = centered_cube(
        format!("closed_waste_neutralization_{name}_lane_left_rail"),
        16.0,
        DISPOSITION_LANE_Y,
        58.0,
    )
    .translate(-DISPOSITION_LANE_X / 2.0 + 8.0, 0.0, 20.0);
    let rail_right = centered_cube(
        format!("closed_waste_neutralization_{name}_lane_right_rail"),
        16.0,
        DISPOSITION_LANE_Y,
        58.0,
    )
    .translate(DISPOSITION_LANE_X / 2.0 - 8.0, 0.0, 20.0);
    lane - disposition_token_recesses(name) + rail_left + rail_right + disposition_token_posts(name)
}

fn disposition_token_recesses(name: &str) -> Part {
    let mut recesses = Part::empty(format!(
        "closed_waste_neutralization_{name}_lane_token_recesses"
    ));
    for i in 0..DISPOSITION_TOKENS_PER_LANE {
        let y = -120.0 + i as f64 * 80.0;
        recesses = recesses
            + centered_cube(
                format!("closed_waste_neutralization_{name}_lane_token_recess_{i}"),
                90.0,
                44.0,
                12.0,
            )
            .translate(0.0, y, DISPOSITION_LANE_Z / 2.0 - 6.0);
    }
    recesses
}

fn disposition_token_posts(name: &str) -> Part {
    let mut posts = Part::empty(format!(
        "closed_waste_neutralization_{name}_lane_token_posts"
    ));
    for i in 0..DISPOSITION_TOKENS_PER_LANE {
        let y = -120.0 + i as f64 * 80.0;
        posts = posts
            + centered_cylinder(
                format!("closed_waste_neutralization_{name}_lane_status_token_{i}"),
                15.0,
                8.0,
                28,
            )
            .translate(0.0, y, DISPOSITION_LANE_Z / 2.0 + 4.0);
    }
    posts
}

fn disposition_gate(name: &str) -> Part {
    centered_cube(
        format!("closed_waste_neutralization_{name}_lane_gate_flag"),
        DISPOSITION_LANE_X - 38.0,
        18.0,
        68.0,
    )
}

fn disposition_common_handoff_rail() -> Part {
    centered_cube(
        "closed_waste_neutralization_disposition_common_handoff_rail",
        disposition_lane_span_x() + 82.0,
        28.0,
        52.0,
    )
    .translate(0.0, -DISPOSITION_LANE_Y / 2.0 - 30.0, 26.0)
}

fn evidence_camera_bridge() -> Part {
    camera_bridge_posts()
        + camera_bridge_beam()
        + camera_pods()
        + camera_bridge_led_strips()
        + camera_calibration_fiducials()
}

fn camera_bridge_posts() -> Part {
    let mut posts = Part::empty("closed_waste_neutralization_camera_bridge_posts");
    for (i, x) in [
        -CAMERA_BRIDGE_SPAN_X / 2.0 + 42.0,
        CAMERA_BRIDGE_SPAN_X / 2.0 - 42.0,
        -160.0,
        160.0,
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_waste_neutralization_evidence_camera_bridge_post_{i}"),
                CAMERA_BRIDGE_POST_X,
                CAMERA_BRIDGE_POST_Y,
                CAMERA_BRIDGE_UNDERSIDE_Z,
            )
            .translate(*x, 0.0, -CAMERA_BRIDGE_BEAM_Z / 2.0);
    }
    posts
}

fn camera_bridge_beam() -> Part {
    centered_cube(
        "closed_waste_neutralization_evidence_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X,
        46.0,
        CAMERA_BRIDGE_BEAM_Z,
    )
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("closed_waste_neutralization_evidence_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = -420.0 + i as f64 * 280.0;
        pods = pods
            + centered_cube(
                format!("closed_waste_neutralization_evidence_camera_pod_{i}"),
                76.0,
                58.0,
                32.0,
            )
            .translate(x, -42.0, -CAMERA_BRIDGE_BEAM_Z / 2.0 - 24.0)
            + centered_cylinder(
                format!("closed_waste_neutralization_evidence_camera_lens_boss_{i}"),
                13.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -77.0, -CAMERA_BRIDGE_BEAM_Z / 2.0 - 24.0);
    }
    pods
}

fn camera_bridge_led_strips() -> Part {
    let mut strips = Part::empty("closed_waste_neutralization_camera_bridge_led_strips");
    for i in 0..LED_STRIPS {
        let x = -490.0 + i as f64 * 140.0;
        strips = strips
            + centered_cube(
                format!("closed_waste_neutralization_evidence_led_strip_{i}"),
                92.0,
                8.0,
                6.0,
            )
            .translate(x, 33.0, -CAMERA_BRIDGE_BEAM_Z / 2.0 - 8.0);
    }
    strips
}

fn camera_calibration_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_waste_neutralization_camera_calibration_fiducials");
    for (i, x) in [-520.0, -260.0, 0.0, 260.0, 520.0].iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_waste_neutralization_camera_fiducial_disc_{i}"),
                12.0,
                5.0,
                28,
            )
            .translate(*x, 70.0, -CAMERA_BRIDGE_BEAM_Z / 2.0 - 12.0);
    }
    fiducials
}

fn robot_service_keepouts() -> Part {
    keepout_box_frame(
        "closed_waste_neutralization_front_robot_keepout",
        STATION_X,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0, 0.0)
        + keepout_box_frame(
            "closed_waste_neutralization_rear_filter_cartridge_service_keepout",
            STATION_X,
            REAR_SERVICE_KEEP_OUT_Y,
            KEEP_OUT_Z,
        )
        .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0, 0.0)
        + keepout_box_frame(
            "closed_waste_neutralization_left_neutralizer_service_keepout",
            LEFT_CARTRIDGE_SERVICE_KEEP_OUT_X,
            STATION_Y,
            KEEP_OUT_Z,
        )
        .translate(
            -STATION_X / 2.0 - LEFT_CARTRIDGE_SERVICE_KEEP_OUT_X / 2.0,
            0.0,
            0.0,
        )
        + keepout_box_frame(
            "closed_waste_neutralization_right_sample_service_keepout",
            RIGHT_SAMPLE_SERVICE_KEEP_OUT_X,
            STATION_Y,
            KEEP_OUT_Z,
        )
        .translate(
            STATION_X / 2.0 + RIGHT_SAMPLE_SERVICE_KEEP_OUT_X / 2.0,
            0.0,
            0.0,
        )
        + keepout_box_frame(
            "closed_waste_neutralization_overhead_evidence_camera_keepout",
            STATION_X - 160.0,
            STATION_Y - 120.0,
            64.0,
        )
        .translate(0.0, 0.0, OVERHEAD_CAMERA_KEEP_OUT_Z)
}

fn keepout_box_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let bottom_front = centered_cube(
        format!("{name}_bottom_front_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(0.0, -y / 2.0, -z / 2.0);
    let bottom_rear = centered_cube(
        format!("{name}_bottom_rear_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(0.0, y / 2.0, -z / 2.0);
    let top_front = centered_cube(
        format!("{name}_top_front_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(0.0, -y / 2.0, z / 2.0);
    let top_rear = centered_cube(
        format!("{name}_top_rear_rail"),
        x,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_RAIL_W,
    )
    .translate(0.0, y / 2.0, z / 2.0);
    let bottom_left = centered_cube(
        format!("{name}_bottom_left_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_W,
    )
    .translate(-x / 2.0, 0.0, -z / 2.0);
    let bottom_right = centered_cube(
        format!("{name}_bottom_right_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_W,
    )
    .translate(x / 2.0, 0.0, -z / 2.0);
    let top_left = centered_cube(
        format!("{name}_top_left_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_W,
    )
    .translate(-x / 2.0, 0.0, z / 2.0);
    let top_right = centered_cube(
        format!("{name}_top_right_rail"),
        KEEP_OUT_RAIL_W,
        y,
        KEEP_OUT_RAIL_W,
    )
    .translate(x / 2.0, 0.0, z / 2.0);

    bottom_front
        + bottom_rear
        + top_front
        + top_rear
        + bottom_left
        + bottom_right
        + top_left
        + top_right
        + keepout_corner_posts(name, x, y, z)
}

fn keepout_corner_posts(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (px, py)) in [
        (-x / 2.0, -y / 2.0),
        (x / 2.0, -y / 2.0),
        (-x / 2.0, y / 2.0),
        (x / 2.0, y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{name}_corner_post_{i}"),
                KEEP_OUT_RAIL_W,
                KEEP_OUT_RAIL_W,
                z,
            )
            .translate(*px, *py, 0.0);
    }
    posts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_are_stable_and_feature_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with("output/"));
            assert!(path.ends_with(".stl"));
            assert!(path.contains("closed_waste_container_neutralization_contact_time_station"));
        }
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing feature output for {feature}"
            );
        }
    }

    #[test]
    fn validation_workflow_counts_are_explicit() {
        assert_eq!(STATUS_LANES, ["release", "hold", "reject"]);
        assert_eq!(CONTACT_TOKENS.len(), 6);
        assert_eq!(COUPON_COUNT, 8);
        assert_eq!(SAMPLE_PORTS, 6);
        assert_eq!(VENT_FILTERS, 2);
        assert_eq!(NEUTRALIZER_CARTRIDGES, 3);
        assert!(BARCODE_LANDS >= CONTACT_TOKENS.len() + SAMPLE_PORTS);
    }

    #[test]
    fn layout_stays_inside_station_deck() {
        assert_layout();
    }

    #[test]
    fn robot_and_service_keepouts_are_nonzero() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 400.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 240.0);
        assert!(LEFT_CARTRIDGE_SERVICE_KEEP_OUT_X >= 220.0);
        assert!(RIGHT_SAMPLE_SERVICE_KEEP_OUT_X >= 200.0);
        assert!(OVERHEAD_CAMERA_KEEP_OUT_Z > CAMERA_BRIDGE_UNDERSIDE_Z);
    }
}
