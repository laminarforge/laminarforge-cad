use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed material-transfer airlock and bagout integrity station.
//
// Intent:
// - Move consumables, microfluidic chips, media bags, and waste across a
//   walk-in clean enclosure boundary without opening culture modules to room
//   air.
// - Package mechanical witness geometry for double-door interlock state,
//   RTP/beta-bag docking, pressure-decay test hookup, HEPA purge flow, UV
//   shutter exposure evidence, barcode custody, leak witness channels, and
//   quarantine parking.
// - Keep sterilization recipes, acceptance criteria, sensors, seals, filters,
//   lamps, and bag assemblies as external validated items.

const OUTPUT_PREFIX: &str = "closed_airlock_material_transfer_bagout_integrity_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_airlock_material_transfer_bagout_integrity_station_boundary_deck.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_double_door_interlock_bay.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_rtp_beta_bag_flange.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_payload_transfer_sleds.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_pressure_decay_test_ports.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_hepa_purge_slot_diffuser.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_uv_shutter_witness_area.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_barcode_custody_lands.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_leak_witness_channel.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_quarantine_parking.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_waste_bagout_clamp_collar.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_robot_service_keepouts.stl",
    "output/closed_airlock_material_transfer_bagout_integrity_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "double_door_interlock",
    "rtp_beta_bag_flange",
    "payload_transfer_sleds",
    "pressure_decay_test_ports",
    "hepa_purge_slot",
    "uv_shutter_witness_area",
    "barcode_custody",
    "leak_witness_channel",
    "quarantine_parking",
    "waste_bagout_clamp",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 1040.0;
const BASE_Z: f64 = 26.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_SLOT_COUNT: usize = 12;

const DOOR_POS: (f64, f64) = (-410.0, 200.0);
const DOOR_ZONE_X: f64 = 420.0;
const DOOR_ZONE_Y: f64 = 300.0;
const DOOR_FRAME_X: f64 = 330.0;
const DOOR_FRAME_Z: f64 = 250.0;
const DOOR_OPENING_X: f64 = 228.0;
const DOOR_OPENING_Z: f64 = 154.0;
const DOOR_FACE_THICKNESS: f64 = 18.0;
const DOOR_GAP_Y: f64 = 210.0;
const DOOR_LATCH_COUNT: usize = 6;
const INTERLOCK_PIN_COUNT: usize = 8;
const DOOR_STATE_TOKEN_COUNT: usize = 4;

const RTP_POS: (f64, f64) = (170.0, 200.0);
const RTP_ZONE_X: f64 = 360.0;
const RTP_ZONE_Y: f64 = 300.0;
const RTP_PLATE_X: f64 = 330.0;
const RTP_PLATE_Y: f64 = 270.0;
const RTP_PLATE_Z: f64 = 24.0;
const RTP_OUTER_R: f64 = 116.0;
const RTP_INNER_R: f64 = 72.0;
const RTP_BOLT_COUNT: usize = 12;
const BETA_BAG_LATCH_COUNT: usize = 6;
const BETA_BAG_FOLD_RIB_COUNT: usize = 5;

const PRESSURE_POS: (f64, f64) = (520.0, 200.0);
const PRESSURE_ZONE_X: f64 = 260.0;
const PRESSURE_ZONE_Y: f64 = 300.0;
const PRESSURE_BLOCK_X: f64 = 230.0;
const PRESSURE_BLOCK_Y: f64 = 260.0;
const PRESSURE_BLOCK_Z: f64 = 54.0;
const PRESSURE_TEST_PORT_COUNT: usize = 4;
const PRESSURE_REFERENCE_VOLUME_COUNT: usize = 3;
const PRESSURE_SENSOR_POCKET_COUNT: usize = 4;

const UV_POS: (f64, f64) = (-470.0, -95.0);
const UV_ZONE_X: f64 = 380.0;
const UV_ZONE_Y: f64 = 210.0;
const UV_PANEL_X: f64 = 350.0;
const UV_PANEL_Y: f64 = 178.0;
const UV_PANEL_Z: f64 = 20.0;
const UV_WITNESS_COUPON_COUNT: usize = 8;
const UV_SHUTTER_BLADE_COUNT: usize = 3;
const UV_DOSIMETER_TOKEN_COUNT: usize = 4;

const CUSTODY_POS: (f64, f64) = (-40.0, -95.0);
const CUSTODY_ZONE_X: f64 = 360.0;
const CUSTODY_ZONE_Y: f64 = 210.0;
const CUSTODY_PANEL_X: f64 = 328.0;
const CUSTODY_PANEL_Y: f64 = 174.0;
const CUSTODY_PANEL_Z: f64 = 16.0;
const BARCODE_LAND_COUNT: usize = 12;
const RFID_PUCK_COUNT: usize = 6;
const CUSTODY_CARD_COUNT: usize = 4;

const LEAK_POS: (f64, f64) = (390.0, -95.0);
const LEAK_ZONE_X: f64 = 340.0;
const LEAK_ZONE_Y: f64 = 210.0;
const LEAK_TRAY_X: f64 = 312.0;
const LEAK_TRAY_Y: f64 = 174.0;
const LEAK_TRAY_Z: f64 = 24.0;
const LEAK_DYE_WELL_COUNT: usize = 6;
const LEAK_CHANNEL_COUNT: usize = 3;
const LEAK_WITNESS_WINDOW_COUNT: usize = 4;

const QUARANTINE_POS: (f64, f64) = (-310.0, -340.0);
const QUARANTINE_ZONE_X: f64 = 520.0;
const QUARANTINE_ZONE_Y: f64 = 180.0;
const QUARANTINE_BAY_COUNT: usize = 4;
const QUARANTINE_LOCK_PIN_COUNT: usize = 8;
const QUARANTINE_TAG_LAND_COUNT: usize = 4;

const WASTE_POS: (f64, f64) = (360.0, -340.0);
const WASTE_ZONE_X: f64 = 420.0;
const WASTE_ZONE_Y: f64 = 180.0;
const WASTE_COLLAR_OUTER_R: f64 = 82.0;
const WASTE_COLLAR_INNER_R: f64 = 48.0;
const WASTE_TIE_SADDLE_COUNT: usize = 6;
const WASTE_SEAL_JAW_COUNT: usize = 2;
const WASTE_BAG_CLIP_COUNT: usize = 8;

const SLED_COUNT: usize = 4;
const CHIP_SLED_COUNT: usize = 2;
const MEDIA_BAG_SLED_COUNT: usize = 1;
const WASTE_SLED_COUNT: usize = 1;
const SLED_X: f64 = 128.0;
const SLED_Y: f64 = 90.0;
const SLED_Z: f64 = 22.0;

const HEPA_SLOT_COUNT: usize = 9;
const HEPA_PLENUM_X: f64 = 710.0;
const HEPA_PLENUM_Y: f64 = 52.0;
const HEPA_PLENUM_Z: f64 = 58.0;
const HEPA_FILTER_CASSETTE_X: f64 = 360.0;
const HEPA_FILTER_CASSETTE_Z: f64 = 108.0;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 950.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_RAIL: f64 = 10.0;
const FRONT_CART_CLEARANCE_Y: f64 = 380.0;
const REAR_ENCLOSURE_CLEARANCE_Y: f64 = 280.0;
const SIDE_BAGOUT_SWING_CLEARANCE_X: f64 = 240.0;
const TOP_HEPA_SERVICE_CLEARANCE_Z: f64 = 360.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 18.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 18.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = boundary_deck();
    export(OUTPUTS[0], &base);

    let doors = double_door_interlock_bay();
    export(OUTPUTS[1], &doors);

    let rtp = rtp_beta_bag_flange();
    export(OUTPUTS[2], &rtp);

    let sleds = payload_transfer_sleds();
    export(OUTPUTS[3], &sleds);

    let pressure = pressure_decay_test_ports();
    export(OUTPUTS[4], &pressure);

    let hepa = hepa_purge_slot_diffuser();
    export(OUTPUTS[5], &hepa);

    let uv = uv_shutter_witness_area();
    export(OUTPUTS[6], &uv);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let leak = leak_witness_channel();
    export(OUTPUTS[8], &leak);

    let quarantine = quarantine_parking();
    export(OUTPUTS[9], &quarantine);

    let waste = waste_bagout_clamp_collar();
    export(OUTPUTS[10], &waste);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + doors
        + rtp
        + sleds
        + pressure
        + hepa
        + uv
        + custody
        + leak
        + quarantine
        + waste
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed airlock material transfer bagout integrity station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm boundary deck");
    println!(
        "  Double-door airlock:    {INTERLOCK_PIN_COUNT} interlock pins, {DOOR_LATCH_COUNT} latch blocks, {DOOR_STATE_TOKEN_COUNT} door-state witness tokens"
    );
    println!(
        "  RTP/beta-bag docking:   {RTP_BOLT_COUNT} flange bolt lands, {BETA_BAG_LATCH_COUNT} beta-bag latch tabs, {BETA_BAG_FOLD_RIB_COUNT} fold ribs"
    );
    println!(
        "  Integrity checks:       {PRESSURE_TEST_PORT_COUNT} pressure-decay ports, {HEPA_SLOT_COUNT} HEPA purge slots, {LEAK_CHANNEL_COUNT} leak witness channels"
    );
    println!(
        "  Custody/evidence:       {BARCODE_LAND_COUNT} barcode lands, {RFID_PUCK_COUNT} RFID pucks, {UV_WITNESS_COUPON_COUNT} UV witness coupons, {QUARANTINE_BAY_COUNT} quarantine bays"
    );
    println!(
        "  Transfer payloads:      {SLED_COUNT} sleds ({CHIP_SLED_COUNT} chip, {MEDIA_BAG_SLED_COUNT} media bag, {WASTE_SLED_COUNT} waste), {WASTE_TIE_SADDLE_COUNT} waste tie saddles"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn boundary_deck() -> Part {
    let deck = centered_cube(
        "airlock_material_transfer_boundary_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sockets = zone_socket(
        "double_door_interlock_socket",
        DOOR_POS,
        DOOR_ZONE_X,
        DOOR_ZONE_Y,
    ) + zone_socket("rtp_beta_bag_socket", RTP_POS, RTP_ZONE_X, RTP_ZONE_Y)
        + zone_socket(
            "pressure_decay_test_socket",
            PRESSURE_POS,
            PRESSURE_ZONE_X,
            PRESSURE_ZONE_Y,
        )
        + zone_socket("uv_shutter_witness_socket", UV_POS, UV_ZONE_X, UV_ZONE_Y)
        + zone_socket(
            "barcode_custody_socket",
            CUSTODY_POS,
            CUSTODY_ZONE_X,
            CUSTODY_ZONE_Y,
        )
        + zone_socket("leak_witness_socket", LEAK_POS, LEAK_ZONE_X, LEAK_ZONE_Y)
        + zone_socket(
            "quarantine_parking_socket",
            QUARANTINE_POS,
            QUARANTINE_ZONE_X,
            QUARANTINE_ZONE_Y,
        )
        + zone_socket("waste_bagout_socket", WASTE_POS, WASTE_ZONE_X, WASTE_ZONE_Y);

    deck - sockets - mounting_slots()
        + perimeter_lips()
        + enclosure_boundary_wall_register()
        + clean_dirty_flow_lands()
        + low_point_leak_gutter()
        + datum_fiducials()
}

fn zone_socket(name: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(
        format!("airlock_material_transfer_{name}"),
        x + 18.0,
        y + 18.0,
        SOCKET_DEPTH + 0.2,
    )
    .translate(center.0, center.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.1)
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        "airlock_material_transfer_rear_clean_enclosure_lip",
        STATION_X - 116.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 38.0, insert_z(RIM_Z));
    let front = centered_cube(
        "airlock_material_transfer_front_cart_stop_lip",
        STATION_X - 230.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 42.0, insert_z(24.0));
    let left = centered_cube(
        "airlock_material_transfer_left_side_lip",
        RIM_W,
        STATION_Y - 148.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + 38.0, 0.0, insert_z(RIM_Z));
    let right = centered_cube(
        "airlock_material_transfer_right_bagout_service_lip",
        RIM_W,
        STATION_Y - 210.0,
        34.0,
    )
    .translate(STATION_X / 2.0 - 38.0, -34.0, insert_z(34.0));

    rear + front + left + right
}

fn enclosure_boundary_wall_register() -> Part {
    let wall = centered_cube(
        "walk_in_clean_enclosure_boundary_wall_register",
        STATION_X - 210.0,
        22.0,
        86.0,
    )
    .translate(0.0, 355.0, insert_z(86.0));
    let transfer_cut = centered_cube(
        "boundary_wall_transfer_opening_keep_clear",
        850.0,
        24.0,
        64.0,
    )
    .translate(0.0, 355.0, insert_z(64.0));
    let gasket_land = rectangular_frame(
        "boundary_wall_gasket_land_frame",
        930.0,
        12.0,
        114.0,
        790.0,
        62.0,
    )
    .translate(0.0, 341.0, BASE_Z + 86.0);

    wall - transfer_cut + gasket_land
}

fn clean_dirty_flow_lands() -> Part {
    let room_side = centered_cube(
        "room_side_material_loading_floor_land",
        STATION_X - 260.0,
        210.0,
        4.0,
    )
    .translate(0.0, -405.0, BASE_Z + 2.0);
    let airlock_core = centered_cube(
        "sealed_airlock_core_floor_land",
        STATION_X - 320.0,
        255.0,
        4.0,
    )
    .translate(0.0, -95.0, BASE_Z + 2.0);
    let clean_side = centered_cube(
        "clean_enclosure_side_floor_land",
        STATION_X - 300.0,
        235.0,
        4.0,
    )
    .translate(0.0, 210.0, BASE_Z + 2.0);

    room_side + airlock_core + clean_side
}

fn low_point_leak_gutter() -> Part {
    let gutter = centered_cube(
        "airlock_material_transfer_low_point_leak_gutter",
        STATION_X - 260.0,
        12.0,
        7.0,
    )
    .translate(0.0, -230.0, BASE_Z - 2.0);
    let sump = centered_cube(
        "airlock_material_transfer_leak_gutter_sump",
        88.0,
        52.0,
        BASE_Z + 4.0,
    )
    .translate(STATION_X / 2.0 - 124.0, -230.0, BASE_Z / 2.0);
    let drain = centered_cylinder(
        "airlock_material_transfer_leak_gutter_drain_port",
        7.0,
        BASE_Z + 8.0,
        32,
    )
    .translate(STATION_X / 2.0 - 124.0, -230.0, BASE_Z / 2.0);

    gutter + sump + drain
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airlock_material_transfer_mounting_slots");
    for (i, (x, y)) in mount_points().into_iter().enumerate() {
        let round = centered_cylinder(
            format!("airlock_material_transfer_m8_mount_round_{i}"),
            4.4,
            BASE_Z + 8.0,
            28,
        )
        .translate(x, y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("airlock_material_transfer_m8_mount_slot_{i}"),
            30.0,
            9.4,
            BASE_Z + 8.0,
        )
        .translate(x, y, BASE_Z / 2.0);
        slots = slots + round + slot;
    }
    slots
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 72.0), -(STATION_Y / 2.0 - 72.0)),
        (STATION_X / 2.0 - 72.0, -(STATION_Y / 2.0 - 72.0)),
        (-(STATION_X / 2.0 - 72.0), STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 72.0, STATION_Y / 2.0 - 76.0),
        (DOOR_POS.0 - DOOR_ZONE_X / 2.0 + 46.0, DOOR_POS.1),
        (DOOR_POS.0 + DOOR_ZONE_X / 2.0 - 46.0, DOOR_POS.1),
        (RTP_POS.0 - RTP_ZONE_X / 2.0 + 46.0, RTP_POS.1),
        (RTP_POS.0 + RTP_ZONE_X / 2.0 - 46.0, RTP_POS.1),
        (PRESSURE_POS.0, PRESSURE_POS.1),
        (UV_POS.0, UV_POS.1),
        (QUARANTINE_POS.0, QUARANTINE_POS.1),
        (WASTE_POS.0, WASTE_POS.1),
    ]
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("airlock_material_transfer_robot_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 108.0), -(STATION_Y / 2.0 - 112.0)),
        (STATION_X / 2.0 - 108.0, -(STATION_Y / 2.0 - 112.0)),
        (-(STATION_X / 2.0 - 108.0), STATION_Y / 2.0 - 112.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 112.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("airlock_material_transfer_fiducial_{i}")).translate(
                x,
                y,
                BASE_Z + 2.0,
            );
    }
    fiducials
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 18.0, 2.0, 44);
    let inner = centered_cylinder(format!("{name}_center_clearance"), 8.0, 3.0, 36);
    let cross_x = centered_cube(format!("{name}_cross_x"), 36.0, 3.5, 2.6);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.5, 36.0, 2.6);

    outer - inner + cross_x + cross_y
}

fn double_door_interlock_bay() -> Part {
    let saddle = centered_cube(
        "double_door_interlock_bay_floor_saddle",
        DOOR_ZONE_X - 36.0,
        DOOR_ZONE_Y - 36.0,
        30.0,
    )
    .translate(DOOR_POS.0, DOOR_POS.1, insert_z(30.0));
    let room_door = airlock_door_frame("room_side_outer_door", -DOOR_GAP_Y / 2.0);
    let clean_door = airlock_door_frame("clean_side_inner_door", DOOR_GAP_Y / 2.0);
    let chamber_side_left = centered_cube(
        "double_door_interlock_left_chamber_wall",
        16.0,
        DOOR_GAP_Y + 64.0,
        116.0,
    )
    .translate(
        DOOR_POS.0 - DOOR_FRAME_X / 2.0 - 22.0,
        DOOR_POS.1,
        BASE_Z + 30.0 + 58.0,
    );
    let chamber_side_right = centered_cube(
        "double_door_interlock_right_chamber_wall",
        16.0,
        DOOR_GAP_Y + 64.0,
        116.0,
    )
    .translate(
        DOOR_POS.0 + DOOR_FRAME_X / 2.0 + 22.0,
        DOOR_POS.1,
        BASE_Z + 30.0 + 58.0,
    );
    let interlock_bar = centered_cube(
        "double_door_interlock_state_bar_between_latches",
        DOOR_FRAME_X + 96.0,
        18.0,
        24.0,
    )
    .translate(DOOR_POS.0, DOOR_POS.1, BASE_Z + 30.0 + DOOR_FRAME_Z + 28.0);

    saddle
        + room_door
        + clean_door
        + chamber_side_left
        + chamber_side_right
        + interlock_bar
        + interlock_pin_bank()
        + door_latch_blocks()
        + door_state_tokens()
}

fn airlock_door_frame(label: &str, y_offset: f64) -> Part {
    let frame = rectangular_frame(
        format!("double_door_interlock_{label}_gasket_frame"),
        DOOR_FRAME_X,
        DOOR_FACE_THICKNESS,
        DOOR_FRAME_Z,
        DOOR_OPENING_X,
        DOOR_OPENING_Z,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + y_offset,
        BASE_Z + 30.0 + DOOR_FRAME_Z / 2.0,
    );
    let hinge_left = centered_cylinder(
        format!("double_door_interlock_{label}_left_hinge_bar"),
        8.0,
        DOOR_FRAME_Z - 36.0,
        24,
    )
    .translate(
        DOOR_POS.0 - DOOR_FRAME_X / 2.0 - 22.0,
        DOOR_POS.1 + y_offset,
        BASE_Z + 30.0 + DOOR_FRAME_Z / 2.0,
    );
    let hinge_right = centered_cylinder(
        format!("double_door_interlock_{label}_right_hinge_bar"),
        8.0,
        DOOR_FRAME_Z - 36.0,
        24,
    )
    .translate(
        DOOR_POS.0 + DOOR_FRAME_X / 2.0 + 22.0,
        DOOR_POS.1 + y_offset,
        BASE_Z + 30.0 + DOOR_FRAME_Z / 2.0,
    );
    let viewing_lane = centered_cube(
        format!("double_door_interlock_{label}_window_witness_land"),
        DOOR_OPENING_X - 38.0,
        4.0,
        20.0,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + y_offset,
        BASE_Z + 30.0 + DOOR_FRAME_Z / 2.0,
    );

    frame + hinge_left + hinge_right + viewing_lane
}

fn interlock_pin_bank() -> Part {
    let mut pins = Part::empty("double_door_interlock_mechanical_pin_bank");
    for i in 0..INTERLOCK_PIN_COUNT {
        let x = centered_index(i, INTERLOCK_PIN_COUNT, 42.0);
        let pin = centered_cylinder(format!("interlock_captive_pin_{i}"), 7.0, 34.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(
                DOOR_POS.0 + x,
                DOOR_POS.1,
                BASE_Z + 30.0 + DOOR_FRAME_Z + 10.0,
            );
        let receiver = centered_cube(format!("interlock_pin_receiver_slot_{i}"), 24.0, 10.0, 18.0)
            .translate(
                DOOR_POS.0 + x,
                DOOR_POS.1 + DOOR_GAP_Y / 2.0 + 22.0,
                BASE_Z + 30.0 + DOOR_FRAME_Z + 10.0,
            );
        pins = pins + pin + receiver;
    }
    pins
}

fn door_latch_blocks() -> Part {
    let mut latches = Part::empty("double_door_interlock_latch_blocks");
    for i in 0..DOOR_LATCH_COUNT {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let door_index = i / 2;
        let y = DOOR_POS.1 + centered_index(door_index, 3, DOOR_GAP_Y / 2.0);
        let block = centered_cube(
            format!("door_interlock_latch_strike_block_{i}"),
            34.0,
            18.0,
            28.0,
        )
        .translate(
            DOOR_POS.0 + side * (DOOR_FRAME_X / 2.0 + 54.0),
            y,
            BASE_Z + 30.0 + 126.0,
        );
        latches = latches + block;
    }
    latches
}

fn door_state_tokens() -> Part {
    let mut tokens = Part::empty("double_door_state_witness_tokens");
    for i in 0..DOOR_STATE_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("door_state_witness_token_pocket_{i}"),
                16.0,
                6.0,
                32,
            )
            .translate(
                DOOR_POS.0 - 126.0 + i as f64 * 84.0,
                DOOR_POS.1 - DOOR_ZONE_Y / 2.0 + 34.0,
                BASE_Z + 30.0 + 6.0,
            );
    }
    tokens
}

fn rtp_beta_bag_flange() -> Part {
    let plate = centered_cube(
        "rtp_beta_bag_docking_plate",
        RTP_PLATE_X,
        RTP_PLATE_Y,
        RTP_PLATE_Z,
    )
    .translate(RTP_POS.0, RTP_POS.1, insert_z(RTP_PLATE_Z));
    let outer = centered_cylinder("rtp_alpha_port_outer_flange", RTP_OUTER_R, 26.0, 72).translate(
        RTP_POS.0,
        RTP_POS.1,
        BASE_Z + RTP_PLATE_Z + 13.0,
    );
    let bore = centered_cylinder("rtp_alpha_port_bore_clearance", RTP_INNER_R, 30.0, 72).translate(
        RTP_POS.0,
        RTP_POS.1,
        BASE_Z + RTP_PLATE_Z + 13.0,
    );
    let key_notch = centered_cube("rtp_alpha_port_keyed_notch_clearance", 48.0, 28.0, 32.0)
        .translate(
            RTP_POS.0,
            RTP_POS.1 + RTP_OUTER_R - 10.0,
            BASE_Z + RTP_PLATE_Z + 13.0,
        );
    let beta_ring = centered_cylinder("beta_bag_secondary_clamp_ring", 98.0, 12.0, 72).translate(
        RTP_POS.0,
        RTP_POS.1,
        BASE_Z + RTP_PLATE_Z + 44.0,
    );
    let beta_bore = centered_cylinder("beta_bag_secondary_clamp_bore", 76.0, 14.0, 72).translate(
        RTP_POS.0,
        RTP_POS.1,
        BASE_Z + RTP_PLATE_Z + 44.0,
    );

    plate
        + (outer - bore - key_notch)
        + (beta_ring - beta_bore)
        + rtp_bolt_lands()
        + beta_bag_latches()
        + beta_bag_fold_ribs()
}

fn rtp_bolt_lands() -> Part {
    let mut lands = Part::empty("rtp_beta_bag_flange_bolt_lands");
    for i in 0..RTP_BOLT_COUNT {
        let (x, y) = polar_xy(i as f64 * 360.0 / RTP_BOLT_COUNT as f64, RTP_OUTER_R + 16.0);
        let boss = centered_cylinder(format!("rtp_flange_m6_bolt_boss_{i}"), 10.0, 10.0, 24)
            .translate(RTP_POS.0 + x, RTP_POS.1 + y, BASE_Z + RTP_PLATE_Z + 34.0);
        let hole = centered_cylinder(format!("rtp_flange_m6_bolt_clearance_{i}"), 3.4, 12.0, 18)
            .translate(RTP_POS.0 + x, RTP_POS.1 + y, BASE_Z + RTP_PLATE_Z + 34.0);
        lands = lands + (boss - hole);
    }
    lands
}

fn beta_bag_latches() -> Part {
    let mut latches = Part::empty("beta_bag_latch_tabs");
    for i in 0..BETA_BAG_LATCH_COUNT {
        let angle = i as f64 * 360.0 / BETA_BAG_LATCH_COUNT as f64 + 30.0;
        let (x, y) = polar_xy(angle, RTP_OUTER_R - 2.0);
        latches = latches
            + centered_cube(format!("beta_bag_captive_latch_tab_{i}"), 38.0, 14.0, 18.0)
                .rotate(0.0, 0.0, angle)
                .translate(RTP_POS.0 + x, RTP_POS.1 + y, BASE_Z + RTP_PLATE_Z + 58.0);
    }
    latches
}

fn beta_bag_fold_ribs() -> Part {
    let mut ribs = Part::empty("beta_bag_fold_support_ribs");
    for i in 0..BETA_BAG_FOLD_RIB_COUNT {
        let y = RTP_POS.1 - 102.0 + i as f64 * 51.0;
        ribs =
            ribs + centered_cube(format!("beta_bag_fold_support_rib_{i}"), 190.0, 8.0, 18.0)
                .translate(RTP_POS.0, y, BASE_Z + RTP_PLATE_Z + 70.0);
    }
    ribs
}

fn payload_transfer_sleds() -> Part {
    let mut sleds = Part::empty("material_transfer_payload_sleds");
    for i in 0..SLED_COUNT {
        let x = -144.0 + i as f64 * 96.0;
        let y = -24.0 + if i % 2 == 0 { -36.0 } else { 36.0 };
        let sled =
            payload_sled(i).translate(DOOR_POS.0 + 190.0 + x, DOOR_POS.1 + y, insert_z(SLED_Z));
        sleds = sleds + sled;
    }
    sleds + transfer_rail_pair()
}

fn payload_sled(index: usize) -> Part {
    let tray = centered_cube(
        format!("payload_transfer_sled_{index}_tray"),
        SLED_X,
        SLED_Y,
        SLED_Z,
    );
    let recess = centered_cube(
        format!("payload_transfer_sled_{index}_nested_payload_recess"),
        SLED_X - 28.0,
        SLED_Y - 24.0,
        SLED_Z + 2.0,
    )
    .translate(0.0, 0.0, 5.0);
    let barcode = centered_cube(
        format!("payload_transfer_sled_{index}_barcode_end_land"),
        44.0,
        14.0,
        4.0,
    )
    .translate(0.0, -SLED_Y / 2.0 - 8.0, SLED_Z / 2.0 + 2.0);
    let type_key = centered_cube(
        format!("payload_transfer_sled_{index}_payload_type_key"),
        18.0 + index as f64 * 4.0,
        12.0,
        8.0,
    )
    .translate(SLED_X / 2.0 - 24.0, SLED_Y / 2.0 + 6.0, SLED_Z / 2.0 + 4.0);

    tray - recess + barcode + type_key
}

fn transfer_rail_pair() -> Part {
    let left = centered_cube("airlock_payload_transfer_left_rail", 585.0, 14.0, 30.0).translate(
        -98.0,
        DOOR_POS.1 - 88.0,
        insert_z(30.0),
    );
    let right = centered_cube("airlock_payload_transfer_right_rail", 585.0, 14.0, 30.0).translate(
        -98.0,
        DOOR_POS.1 + 88.0,
        insert_z(30.0),
    );
    let center_stop = centered_cube(
        "airlock_payload_transfer_center_positive_stop",
        24.0,
        196.0,
        34.0,
    )
    .translate(RTP_POS.0 - 10.0, DOOR_POS.1, insert_z(34.0));

    left + right + center_stop
}

fn pressure_decay_test_ports() -> Part {
    let block = centered_cube(
        "pressure_decay_test_manifold_block",
        PRESSURE_BLOCK_X,
        PRESSURE_BLOCK_Y,
        PRESSURE_BLOCK_Z,
    )
    .translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_BLOCK_Z));
    let header = centered_cylinder(
        "pressure_decay_manifold_header_bore",
        11.0,
        PRESSURE_BLOCK_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_BLOCK_Z));

    block - header
        + pressure_test_port_bosses()
        + pressure_sensor_pockets()
        + pressure_reference_volumes()
}

fn pressure_test_port_bosses() -> Part {
    let mut ports = Part::empty("pressure_decay_test_port_bosses");
    for i in 0..PRESSURE_TEST_PORT_COUNT {
        let x = centered_index(i, PRESSURE_TEST_PORT_COUNT, 48.0);
        let boss = centered_cylinder(
            format!("pressure_decay_luer_test_port_boss_{i}"),
            16.0,
            18.0,
            32,
        )
        .translate(
            PRESSURE_POS.0 + x,
            PRESSURE_POS.1 - PRESSURE_BLOCK_Y / 2.0 - 7.0,
            BASE_Z + PRESSURE_BLOCK_Z + 20.0,
        );
        let bore = centered_cylinder(
            format!("pressure_decay_luer_test_port_bore_{i}"),
            4.0,
            20.0,
            20,
        )
        .translate(
            PRESSURE_POS.0 + x,
            PRESSURE_POS.1 - PRESSURE_BLOCK_Y / 2.0 - 7.0,
            BASE_Z + PRESSURE_BLOCK_Z + 20.0,
        );
        ports = ports + (boss - bore);
    }
    ports
}

fn pressure_sensor_pockets() -> Part {
    let mut pockets = Part::empty("pressure_decay_sensor_pockets");
    for i in 0..PRESSURE_SENSOR_POCKET_COUNT {
        pockets = pockets
            + centered_cube(
                format!("pressure_decay_transducer_pocket_land_{i}"),
                42.0,
                32.0,
                10.0,
            )
            .translate(
                PRESSURE_POS.0 + centered_index(i, PRESSURE_SENSOR_POCKET_COUNT, 48.0),
                PRESSURE_POS.1 + 26.0,
                BASE_Z + PRESSURE_BLOCK_Z + 8.0,
            );
    }
    pockets
}

fn pressure_reference_volumes() -> Part {
    let mut volumes = Part::empty("pressure_decay_reference_volume_nests");
    for i in 0..PRESSURE_REFERENCE_VOLUME_COUNT {
        let x = PRESSURE_POS.0 + centered_index(i, PRESSURE_REFERENCE_VOLUME_COUNT, 58.0);
        let nest = centered_cylinder(
            format!("pressure_decay_reference_volume_nest_{i}"),
            20.0,
            20.0,
            36,
        )
        .translate(
            x,
            PRESSURE_POS.1 + PRESSURE_BLOCK_Y / 2.0 - 42.0,
            BASE_Z + PRESSURE_BLOCK_Z + 10.0,
        );
        let label = centered_cube(
            format!("pressure_decay_reference_volume_label_land_{i}"),
            46.0,
            12.0,
            4.0,
        )
        .translate(
            x,
            PRESSURE_POS.1 + PRESSURE_BLOCK_Y / 2.0 - 8.0,
            BASE_Z + PRESSURE_BLOCK_Z + 4.0,
        );
        volumes = volumes + nest + label;
    }
    volumes
}

fn hepa_purge_slot_diffuser() -> Part {
    let plenum = centered_cube(
        "hepa_purge_slot_plenum_body",
        HEPA_PLENUM_X,
        HEPA_PLENUM_Y,
        HEPA_PLENUM_Z,
    )
    .translate(80.0, 365.0, insert_z(HEPA_PLENUM_Z));
    let slots = hepa_purge_slot_cuts();
    let filter_frame = rectangular_frame(
        "hepa_filter_cassette_service_frame",
        HEPA_FILTER_CASSETTE_X,
        16.0,
        HEPA_FILTER_CASSETTE_Z,
        HEPA_FILTER_CASSETTE_X - 84.0,
        HEPA_FILTER_CASSETTE_Z - 42.0,
    )
    .translate(
        520.0,
        388.0,
        BASE_Z + HEPA_PLENUM_Z + HEPA_FILTER_CASSETTE_Z / 2.0,
    );

    plenum - slots + hepa_diffuser_vanes() + filter_frame + hepa_damper_flags()
}

fn hepa_purge_slot_cuts() -> Part {
    let mut slots = Part::empty("hepa_purge_slot_cuts");
    for i in 0..HEPA_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("hepa_purge_diffuser_slot_{i}"),
                48.0,
                HEPA_PLENUM_Y + 4.0,
                12.0,
            )
            .translate(
                80.0 + centered_index(i, HEPA_SLOT_COUNT, 68.0),
                365.0,
                BASE_Z + HEPA_PLENUM_Z / 2.0 + 8.0,
            );
    }
    slots
}

fn hepa_diffuser_vanes() -> Part {
    let mut vanes = Part::empty("hepa_purge_diffuser_vanes");
    for i in 0..(HEPA_SLOT_COUNT + 1) {
        vanes = vanes
            + centered_cube(
                format!("hepa_purge_diffuser_vane_{i}"),
                8.0,
                HEPA_PLENUM_Y + 22.0,
                24.0,
            )
            .translate(
                80.0 + centered_index(i, HEPA_SLOT_COUNT + 1, 68.0) - 34.0,
                365.0,
                BASE_Z + HEPA_PLENUM_Z + 12.0,
            );
    }
    vanes
}

fn hepa_damper_flags() -> Part {
    let mut flags = Part::empty("hepa_purge_damper_position_flags");
    for i in 0..3 {
        flags = flags
            + centered_cube(format!("hepa_purge_damper_flag_{i}"), 42.0, 10.0, 28.0)
                .rotate(0.0, 0.0, -18.0 + i as f64 * 18.0)
                .translate(
                    360.0 + i as f64 * 44.0,
                    325.0,
                    BASE_Z + HEPA_PLENUM_Z + 26.0,
                );
    }
    flags
}

fn uv_shutter_witness_area() -> Part {
    let panel = centered_cube(
        "uv_shutter_witness_panel",
        UV_PANEL_X,
        UV_PANEL_Y,
        UV_PANEL_Z,
    )
    .translate(UV_POS.0, UV_POS.1, insert_z(UV_PANEL_Z));
    let window = rectangular_frame(
        "uv_shutter_witness_window_frame",
        UV_PANEL_X - 54.0,
        12.0,
        76.0,
        UV_PANEL_X - 132.0,
        42.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(UV_POS.0, UV_POS.1 - 6.0, BASE_Z + UV_PANEL_Z + 54.0);

    panel + window + uv_shutter_blades() + uv_witness_coupon_pockets() + uv_dosimeter_tokens()
}

fn uv_shutter_blades() -> Part {
    let mut blades = Part::empty("uv_shutter_blade_position_witnesses");
    for i in 0..UV_SHUTTER_BLADE_COUNT {
        blades = blades
            + centered_cube(
                format!("uv_sliding_shutter_blade_witness_{i}"),
                250.0,
                12.0,
                9.0,
            )
            .translate(
                UV_POS.0,
                UV_POS.1 - 58.0 + i as f64 * 58.0,
                BASE_Z + UV_PANEL_Z + 18.0 + i as f64 * 3.0,
            );
    }
    blades
}

fn uv_witness_coupon_pockets() -> Part {
    let mut coupons = Part::empty("uv_witness_coupon_pockets");
    for i in 0..UV_WITNESS_COUPON_COUNT {
        coupons = coupons
            + centered_cube(format!("uv_witness_coupon_pocket_{i}"), 42.0, 28.0, 8.0).translate(
                UV_POS.0 + centered_index(i % 4, 4, 62.0),
                UV_POS.1 - 58.0 + (i / 4) as f64 * 116.0,
                BASE_Z + UV_PANEL_Z + 7.0,
            );
    }
    coupons
}

fn uv_dosimeter_tokens() -> Part {
    let mut tokens = Part::empty("uv_dosimeter_token_lands");
    for i in 0..UV_DOSIMETER_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(format!("uv_dosimeter_token_land_{i}"), 14.0, 5.0, 28).translate(
                UV_POS.0 - 150.0 + i as f64 * 100.0,
                UV_POS.1 + 84.0,
                BASE_Z + UV_PANEL_Z + 6.0,
            );
    }
    tokens
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "barcode_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_PANEL_Z));
    panel + barcode_lands() + rfid_pucks() + custody_card_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("barcode_custody_scan_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(format!("barcode_custody_label_land_{i}"), 58.0, 18.0, 4.0).translate(
                CUSTODY_POS.0 + centered_index(col, 4, 74.0),
                CUSTODY_POS.1 - 58.0 + row as f64 * 42.0,
                BASE_Z + CUSTODY_PANEL_Z + 4.0,
            );
    }
    lands
}

fn rfid_pucks() -> Part {
    let mut pucks = Part::empty("barcode_custody_rfid_puck_lands");
    for i in 0..RFID_PUCK_COUNT {
        pucks = pucks
            + centered_cylinder(format!("rfid_custody_puck_land_{i}"), 16.0, 5.0, 32).translate(
                CUSTODY_POS.0 - 125.0 + i as f64 * 50.0,
                CUSTODY_POS.1 + 58.0,
                BASE_Z + CUSTODY_PANEL_Z + 5.0,
            );
    }
    pucks
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty("chain_of_custody_card_slots");
    for i in 0..CUSTODY_CARD_COUNT {
        slots = slots
            + centered_cube(format!("chain_of_custody_card_slot_{i}"), 62.0, 8.0, 26.0).translate(
                CUSTODY_POS.0 - 111.0 + i as f64 * 74.0,
                CUSTODY_POS.1 + 92.0,
                BASE_Z + CUSTODY_PANEL_Z + 18.0,
            );
    }
    slots
}

fn leak_witness_channel() -> Part {
    let tray = centered_cube(
        "leak_witness_channel_tray",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    )
    .translate(LEAK_POS.0, LEAK_POS.1, insert_z(LEAK_TRAY_Z));
    let trough = centered_cube(
        "leak_witness_main_channel_clearance",
        LEAK_TRAY_X - 58.0,
        32.0,
        LEAK_TRAY_Z + 4.0,
    )
    .translate(LEAK_POS.0, LEAK_POS.1, insert_z(LEAK_TRAY_Z) + 4.0);

    tray - trough + leak_channel_edges() + leak_dye_wells() + leak_witness_windows()
}

fn leak_channel_edges() -> Part {
    let mut edges = Part::empty("leak_witness_channel_edges");
    for i in 0..LEAK_CHANNEL_COUNT {
        let y = LEAK_POS.1 + centered_index(i, LEAK_CHANNEL_COUNT, 52.0);
        edges = edges
            + centered_cube(
                format!("leak_witness_channel_{i}_left_edge"),
                LEAK_TRAY_X - 44.0,
                7.0,
                20.0,
            )
            .translate(LEAK_POS.0, y - 20.0, BASE_Z + LEAK_TRAY_Z + 10.0)
            + centered_cube(
                format!("leak_witness_channel_{i}_right_edge"),
                LEAK_TRAY_X - 44.0,
                7.0,
                20.0,
            )
            .translate(LEAK_POS.0, y + 20.0, BASE_Z + LEAK_TRAY_Z + 10.0);
    }
    edges
}

fn leak_dye_wells() -> Part {
    let mut wells = Part::empty("leak_witness_dye_wells");
    for i in 0..LEAK_DYE_WELL_COUNT {
        wells = wells
            + centered_cylinder(format!("leak_witness_dye_well_{i}"), 14.0, 9.0, 30).translate(
                LEAK_POS.0 + centered_index(i, LEAK_DYE_WELL_COUNT, 44.0),
                LEAK_POS.1 - LEAK_TRAY_Y / 2.0 + 28.0,
                BASE_Z + LEAK_TRAY_Z + 5.0,
            );
    }
    wells
}

fn leak_witness_windows() -> Part {
    let mut windows = Part::empty("leak_witness_window_lands");
    for i in 0..LEAK_WITNESS_WINDOW_COUNT {
        windows = windows
            + centered_cube(
                format!("clear_leak_witness_window_land_{i}"),
                54.0,
                18.0,
                5.0,
            )
            .translate(
                LEAK_POS.0 + centered_index(i, LEAK_WITNESS_WINDOW_COUNT, 68.0),
                LEAK_POS.1 + LEAK_TRAY_Y / 2.0 - 24.0,
                BASE_Z + LEAK_TRAY_Z + 4.0,
            );
    }
    windows
}

fn quarantine_parking() -> Part {
    let deck = centered_cube(
        "quarantine_material_parking_deck",
        QUARANTINE_ZONE_X - 34.0,
        QUARANTINE_ZONE_Y - 28.0,
        22.0,
    )
    .translate(QUARANTINE_POS.0, QUARANTINE_POS.1, insert_z(22.0));
    deck + quarantine_bays() + quarantine_lock_pins() + quarantine_tag_lands()
}

fn quarantine_bays() -> Part {
    let mut bays = Part::empty("quarantine_material_parking_bays");
    for i in 0..QUARANTINE_BAY_COUNT {
        let x = QUARANTINE_POS.0 + centered_index(i, QUARANTINE_BAY_COUNT, 118.0);
        let floor = centered_cube(
            format!("quarantine_bay_{i}_floor_socket"),
            92.0,
            104.0,
            10.0,
        )
        .translate(x, QUARANTINE_POS.1, BASE_Z + 22.0 + 5.0);
        let rear_gate = centered_cube(
            format!("quarantine_bay_{i}_rear_locked_gate"),
            92.0,
            12.0,
            46.0,
        )
        .translate(x, QUARANTINE_POS.1 + 62.0, BASE_Z + 22.0 + 23.0);
        let side_rail = centered_cube(format!("quarantine_bay_{i}_side_rail"), 10.0, 118.0, 34.0)
            .translate(x - 52.0, QUARANTINE_POS.1, BASE_Z + 22.0 + 17.0);
        bays = bays + floor + rear_gate + side_rail;
    }
    bays
}

fn quarantine_lock_pins() -> Part {
    let mut pins = Part::empty("quarantine_lock_pin_board");
    for i in 0..QUARANTINE_LOCK_PIN_COUNT {
        pins = pins
            + centered_cylinder(format!("quarantine_lock_pin_{i}"), 8.0, 30.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    QUARANTINE_POS.0 - 210.0 + i as f64 * 60.0,
                    QUARANTINE_POS.1 - QUARANTINE_ZONE_Y / 2.0 + 20.0,
                    BASE_Z + 64.0,
                );
    }
    pins
}

fn quarantine_tag_lands() -> Part {
    let mut tags = Part::empty("quarantine_status_tag_lands");
    for i in 0..QUARANTINE_TAG_LAND_COUNT {
        tags = tags
            + centered_cube(format!("quarantine_status_tag_land_{i}"), 74.0, 20.0, 5.0).translate(
                QUARANTINE_POS.0 + centered_index(i, QUARANTINE_TAG_LAND_COUNT, 118.0),
                QUARANTINE_POS.1 - 70.0,
                BASE_Z + 30.0,
            );
    }
    tags
}

fn waste_bagout_clamp_collar() -> Part {
    let plate = centered_cube(
        "waste_bagout_clamp_plate",
        WASTE_ZONE_X - 34.0,
        WASTE_ZONE_Y - 30.0,
        22.0,
    )
    .translate(WASTE_POS.0, WASTE_POS.1, insert_z(22.0));
    let collar = centered_cylinder(
        "waste_bagout_clamp_outer_collar",
        WASTE_COLLAR_OUTER_R,
        24.0,
        64,
    )
    .translate(WASTE_POS.0 - 112.0, WASTE_POS.1, BASE_Z + 22.0 + 12.0);
    let bore = centered_cylinder(
        "waste_bagout_bag_neck_clearance",
        WASTE_COLLAR_INNER_R,
        28.0,
        64,
    )
    .translate(WASTE_POS.0 - 112.0, WASTE_POS.1, BASE_Z + 22.0 + 12.0);

    plate + (collar - bore) + waste_tie_saddles() + waste_seal_jaws() + waste_bag_clips()
}

fn waste_tie_saddles() -> Part {
    let mut saddles = Part::empty("waste_bagout_tie_saddles");
    for i in 0..WASTE_TIE_SADDLE_COUNT {
        saddles = saddles
            + centered_cube(format!("waste_bagout_tie_saddle_{i}"), 34.0, 14.0, 18.0).translate(
                WASTE_POS.0 + 28.0 + centered_index(i, WASTE_TIE_SADDLE_COUNT, 46.0),
                WASTE_POS.1 - 58.0,
                BASE_Z + 22.0 + 22.0,
            );
    }
    saddles
}

fn waste_seal_jaws() -> Part {
    let mut jaws = Part::empty("waste_bagout_heat_seal_jaws");
    for i in 0..WASTE_SEAL_JAW_COUNT {
        jaws = jaws
            + centered_cube(
                format!("waste_bagout_parallel_seal_jaw_{i}"),
                238.0,
                14.0,
                34.0,
            )
            .translate(
                WASTE_POS.0 + 104.0,
                WASTE_POS.1 + centered_index(i, WASTE_SEAL_JAW_COUNT, 46.0),
                BASE_Z + 22.0 + 28.0,
            );
    }
    jaws
}

fn waste_bag_clips() -> Part {
    let mut clips = Part::empty("waste_bagout_bag_clip_lands");
    for i in 0..WASTE_BAG_CLIP_COUNT {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = WASTE_POS.0 - 112.0 + side * (WASTE_COLLAR_OUTER_R + 28.0);
        let y = WASTE_POS.1 + centered_index(i / 2, WASTE_BAG_CLIP_COUNT / 2, 38.0);
        clips =
            clips
                + centered_cube(format!("waste_bagout_bag_clip_land_{i}"), 28.0, 12.0, 16.0)
                    .translate(x, y, BASE_Z + 22.0 + 28.0);
    }
    clips
}

fn robot_service_keepouts() -> Part {
    keepout_frame(
        "material_transfer_robot_cart_keepout",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        72.0,
    ) + cart_approach_gauges()
        + door_swing_reference_arcs()
        + hepa_service_lift_post()
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let base = centered_cube(format!("{name}_floor_reference"), x, y, KEEP_OUT_Z);
    let top_front = centered_cube(
        format!("{name}_top_front_rail"),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -y / 2.0 + KEEP_OUT_RAIL / 2.0, z);
    let top_rear = centered_cube(
        format!("{name}_top_rear_rail"),
        x,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, y / 2.0 - KEEP_OUT_RAIL / 2.0, z);
    let top_left = centered_cube(
        format!("{name}_top_left_rail"),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(-x / 2.0 + KEEP_OUT_RAIL / 2.0, 0.0, z);
    let top_right = centered_cube(
        format!("{name}_top_right_rail"),
        KEEP_OUT_RAIL,
        y,
        KEEP_OUT_RAIL,
    )
    .translate(x / 2.0 - KEEP_OUT_RAIL / 2.0, 0.0, z);

    base + top_front + top_rear + top_left + top_right
}

fn cart_approach_gauges() -> Part {
    let mut gauges = Part::empty("cart_approach_and_service_clearance_gauges");
    for (i, (x, y, z)) in [
        (0.0, -FRONT_CART_CLEARANCE_Y, 54.0),
        (0.0, REAR_ENCLOSURE_CLEARANCE_Y, 64.0),
        (-SIDE_BAGOUT_SWING_CLEARANCE_X, 0.0, 58.0),
        (SIDE_BAGOUT_SWING_CLEARANCE_X, 0.0, 58.0),
    ]
    .into_iter()
    .enumerate()
    {
        gauges =
            gauges
                + centered_cube(format!("service_clearance_gauge_post_{i}"), 22.0, 22.0, z)
                    .translate(x, y, KEEP_OUT_Z / 2.0 + z / 2.0);
    }
    gauges
}

fn door_swing_reference_arcs() -> Part {
    let outer = centered_cylinder("double_door_swing_reference_outer_arc", 228.0, 5.0, 72)
        .translate(DOOR_POS.0, DOOR_POS.1, KEEP_OUT_Z + 6.0);
    let inner = centered_cylinder(
        "double_door_swing_reference_inner_clearance",
        202.0,
        6.0,
        72,
    )
    .translate(DOOR_POS.0, DOOR_POS.1, KEEP_OUT_Z + 6.0);
    let front_trim = centered_cube("double_door_swing_reference_arc_trim", 520.0, 260.0, 8.0)
        .translate(DOOR_POS.0, DOOR_POS.1 - 174.0, KEEP_OUT_Z + 6.0);

    outer - inner - front_trim
}

fn hepa_service_lift_post() -> Part {
    centered_cube(
        "top_hepa_service_lift_clearance_post",
        26.0,
        26.0,
        TOP_HEPA_SERVICE_CLEARANCE_Z,
    )
    .translate(620.0, 388.0, TOP_HEPA_SERVICE_CLEARANCE_Z / 2.0)
}

fn rectangular_frame(
    name: impl Into<String>,
    outer_x: f64,
    y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let top = centered_cube(format!("{name}_top"), outer_x, y, (outer_z - inner_z) / 2.0)
        .translate(0.0, 0.0, inner_z / 2.0 + (outer_z - inner_z) / 4.0);
    let bottom = centered_cube(
        format!("{name}_bottom"),
        outer_x,
        y,
        (outer_z - inner_z) / 2.0,
    )
    .translate(0.0, 0.0, -inner_z / 2.0 - (outer_z - inner_z) / 4.0);
    let left = centered_cube(
        format!("{name}_left"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(-inner_x / 2.0 - (outer_x - inner_x) / 4.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{name}_right"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(inner_x / 2.0 + (outer_x - inner_x) / 4.0, 0.0, 0.0);

    top + bottom + left + right
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn insert_z(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn polar_xy(theta_degrees: f64, radius: f64) -> (f64, f64) {
    let radians = theta_degrees.to_radians();
    (radius * radians.cos(), radius * radians.sin())
}

fn layout_rects() -> [Rect; 8] {
    [
        Rect {
            name: "double door interlock",
            center: DOOR_POS,
            x: DOOR_ZONE_X,
            y: DOOR_ZONE_Y,
        },
        Rect {
            name: "RTP beta bag flange",
            center: RTP_POS,
            x: RTP_ZONE_X,
            y: RTP_ZONE_Y,
        },
        Rect {
            name: "pressure decay test ports",
            center: PRESSURE_POS,
            x: PRESSURE_ZONE_X,
            y: PRESSURE_ZONE_Y,
        },
        Rect {
            name: "UV shutter witness",
            center: UV_POS,
            x: UV_ZONE_X,
            y: UV_ZONE_Y,
        },
        Rect {
            name: "barcode custody",
            center: CUSTODY_POS,
            x: CUSTODY_ZONE_X,
            y: CUSTODY_ZONE_Y,
        },
        Rect {
            name: "leak witness",
            center: LEAK_POS,
            x: LEAK_ZONE_X,
            y: LEAK_ZONE_Y,
        },
        Rect {
            name: "quarantine parking",
            center: QUARANTINE_POS,
            x: QUARANTINE_ZONE_X,
            y: QUARANTINE_ZONE_Y,
        },
        Rect {
            name: "waste bagout",
            center: WASTE_POS,
            x: WASTE_ZONE_X,
            y: WASTE_ZONE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 11);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds usable station deck",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert_eq!(
        SLED_COUNT,
        CHIP_SLED_COUNT + MEDIA_BAG_SLED_COUNT + WASTE_SLED_COUNT
    );
    assert!(DOOR_GAP_Y > DOOR_FACE_THICKNESS * 6.0);
    assert!(RTP_OUTER_R - RTP_INNER_R >= 40.0);
    assert!(WASTE_COLLAR_OUTER_R > WASTE_COLLAR_INNER_R + 25.0);
    assert!(HEPA_SLOT_COUNT >= 7);
    assert!(TOP_HEPA_SERVICE_CLEARANCE_Z >= 340.0);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/"));
            assert!(path.contains(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn requested_design_cues_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for expected in [
            "double_door_interlock",
            "rtp_beta_bag_flange",
            "pressure_decay_test_ports",
            "hepa_purge_slot",
            "uv_shutter_witness_area",
            "barcode_custody",
            "leak_witness_channel",
            "quarantine_parking",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn station_zones_fit_without_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn transfer_and_bagout_capacity_is_counted() {
        assert_eq!(SLED_COUNT, 4);
        assert_eq!(CHIP_SLED_COUNT, 2);
        assert_eq!(MEDIA_BAG_SLED_COUNT, 1);
        assert_eq!(WASTE_SLED_COUNT, 1);
        assert_eq!(QUARANTINE_BAY_COUNT, 4);
        assert_eq!(WASTE_SEAL_JAW_COUNT, 2);
        assert_eq!(WASTE_BAG_CLIP_COUNT, 8);
    }

    #[test]
    fn integrity_features_cover_pressure_purge_uv_and_leak_paths() {
        assert_eq!(PRESSURE_TEST_PORT_COUNT, 4);
        assert_eq!(PRESSURE_SENSOR_POCKET_COUNT, PRESSURE_TEST_PORT_COUNT);
        assert_eq!(PRESSURE_REFERENCE_VOLUME_COUNT, 3);
        assert_eq!(HEPA_SLOT_COUNT, 9);
        assert_eq!(UV_WITNESS_COUPON_COUNT, 8);
        assert_eq!(UV_DOSIMETER_TOKEN_COUNT, 4);
        assert_eq!(LEAK_DYE_WELL_COUNT, 6);
        assert_eq!(LEAK_CHANNEL_COUNT, 3);
    }

    #[test]
    fn interlock_and_custody_controls_are_plausible() {
        assert_eq!(INTERLOCK_PIN_COUNT, 8);
        assert_eq!(DOOR_LATCH_COUNT, 6);
        assert_eq!(DOOR_STATE_TOKEN_COUNT, 4);
        assert_eq!(RTP_BOLT_COUNT, 12);
        assert_eq!(BETA_BAG_LATCH_COUNT, 6);
        assert!(BARCODE_LAND_COUNT >= SLED_COUNT + QUARANTINE_BAY_COUNT);
        assert!(RFID_PUCK_COUNT >= 6);
        assert_eq!(CUSTODY_CARD_COUNT, 4);
    }
}
