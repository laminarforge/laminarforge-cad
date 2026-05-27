use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed TEER/impedance phantom verification station for sensor backplane release.
//
// Intent:
// - Verify the sensor backplane TEER/impedance contacts against dry and humid
//   reference phantom cartridges before release into the closed culture workcell.
// - Keep phantom cartridges, pogo/contact alignment, resistor/capacitor
//   references, cable handling, evidence capture, and release disposition in one
//   repeatable station with explicit robot and service keepouts.
//
// This is packaging and interface CAD only. Reference values, metrology
// uncertainty, humidity setpoints, firmware stimulus, and release criteria are
// validation-system decisions.

const OUTPUTS: &[&str] = &[
    "output/closed_teer_impedance_phantom_verification_station_base_tray.stl",
    "output/closed_teer_impedance_phantom_verification_station_phantom_cartridge_nests.stl",
    "output/closed_teer_impedance_phantom_verification_station_pogo_contact_alignment_gauge.stl",
    "output/closed_teer_impedance_phantom_verification_station_humid_shield_placeholder.stl",
    "output/closed_teer_impedance_phantom_verification_station_reference_certificate_lands.stl",
    "output/closed_teer_impedance_phantom_verification_station_cable_strain_relief.stl",
    "output/closed_teer_impedance_phantom_verification_station_release_hold_reject_lanes.stl",
    "output/closed_teer_impedance_phantom_verification_station_evidence_camera_bridge.stl",
    "output/closed_teer_impedance_phantom_verification_station_robot_service_keepouts.stl",
    "output/closed_teer_impedance_phantom_verification_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_leak_tray",
    "phantom_cartridge_nests",
    "pogo_contact_alignment_gauge",
    "humid_shield_placeholder",
    "reference_resistor_capacitor_certificate_lands",
    "cable_strain_relief",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 720.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;

const PHANTOM_ROWS: usize = 3;
const PHANTOM_COLS: usize = 4;
const PHANTOM_CARTRIDGE_COUNT: usize = PHANTOM_ROWS * PHANTOM_COLS;
const PHANTOM_NEST_X: f64 = 636.0;
const PHANTOM_NEST_Y: f64 = 258.0;
const PHANTOM_NEST_Z: f64 = 54.0;
const PHANTOM_NEST_POS: (f64, f64) = (-210.0, 160.0);
const PHANTOM_SLOT_X: f64 = 112.0;
const PHANTOM_SLOT_Y: f64 = 46.0;
const PHANTOM_SLOT_Z: f64 = 28.0;
const PHANTOM_PITCH_X: f64 = 144.0;
const PHANTOM_PITCH_Y: f64 = 72.0;

const POGO_GAUGE_X: f64 = 382.0;
const POGO_GAUGE_Y: f64 = 226.0;
const POGO_GAUGE_Z: f64 = 46.0;
const POGO_GAUGE_POS: (f64, f64) = (318.0, 160.0);
const POGO_COLUMNS: usize = 4;
const POGO_ROWS: usize = 25;
const POGO_CONTACT_COUNT: usize = POGO_COLUMNS * POGO_ROWS;
const POGO_PITCH_X: f64 = 12.0;
const POGO_PITCH_Y: f64 = 7.2;
const BACKPLANE_WINDOW_X: f64 = 156.0;
const BACKPLANE_WINDOW_Y: f64 = 198.0;

const HUMID_SHIELD_X: f64 = 980.0;
const HUMID_SHIELD_Y: f64 = 330.0;
const HUMID_SHIELD_Z: f64 = 192.0;
const HUMID_SHIELD_POS: (f64, f64) = (-25.0, 160.0);

const REFERENCE_X: f64 = 374.0;
const REFERENCE_Y: f64 = 150.0;
const REFERENCE_Z: f64 = 18.0;
const REFERENCE_POS: (f64, f64) = (326.0, -72.0);
const REFERENCE_RESISTOR_LANDS: usize = 6;
const REFERENCE_CAPACITOR_LANDS: usize = 6;
const CERTIFICATE_LANDS: usize = 8;

const CABLE_X: f64 = 270.0;
const CABLE_Y: f64 = 190.0;
const CABLE_Z: f64 = 54.0;
const CABLE_POS: (f64, f64) = (385.0, -230.0);
const CABLE_CLIPS: usize = 10;
const CABLE_BORE_D: f64 = 7.0;

const DISPOSITION_X: f64 = 620.0;
const DISPOSITION_Y: f64 = 190.0;
const DISPOSITION_Z: f64 = 42.0;
const DISPOSITION_POS: (f64, f64) = (-220.0, -194.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_LANE_PITCH_X: f64 = 190.0;
const STATUS_SLOT_PITCH_Y: f64 = 38.0;

const CAMERA_BRIDGE_X: f64 = 680.0;
const CAMERA_BRIDGE_Y: f64 = 220.0;
const CAMERA_BRIDGE_Z: f64 = 224.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (-190.0, -194.0);
const CAMERA_FIELD_X: f64 = 500.0;
const CAMERA_FIELD_Y: f64 = 116.0;

const LEAK_CHANNELS: usize = 6;
const KEEP_OUT_ZONE_COUNT: usize = 6;
const FRONT_ROBOT_SWEEP: f64 = 420.0;
const REAR_CABLE_SERVICE: f64 = 260.0;
const SIDE_SERVICE_CLEARANCE: f64 = 180.0;
const SHIELD_LIFT_CLEARANCE: f64 = 250.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let nests = phantom_cartridge_nests();
    export(OUTPUTS[1], &nests);

    let pogo = pogo_contact_alignment_gauge();
    export(OUTPUTS[2], &pogo);

    let shield = humid_shield_placeholder();
    export(OUTPUTS[3], &shield);

    let references = reference_resistor_capacitor_certificate_lands();
    export(OUTPUTS[4], &references);

    let cable = cable_strain_relief();
    export(OUTPUTS[5], &cable);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[6], &disposition);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[7], &camera);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[8], &keepouts);

    let assembly =
        base + nests.translate(
            PHANTOM_NEST_POS.0,
            PHANTOM_NEST_POS.1,
            insert_z(PHANTOM_NEST_Z),
        ) + pogo.translate(POGO_GAUGE_POS.0, POGO_GAUGE_POS.1, insert_z(POGO_GAUGE_Z))
            + shield.translate(
                HUMID_SHIELD_POS.0,
                HUMID_SHIELD_POS.1,
                insert_z(HUMID_SHIELD_Z),
            )
            + references.translate(REFERENCE_POS.0, REFERENCE_POS.1, insert_z(REFERENCE_Z))
            + cable.translate(CABLE_POS.0, CABLE_POS.1, insert_z(CABLE_Z))
            + disposition.translate(
                DISPOSITION_POS.0,
                DISPOSITION_POS.1,
                insert_z(DISPOSITION_Z),
            )
            + camera.translate(
                CAMERA_BRIDGE_POS.0,
                CAMERA_BRIDGE_POS.1,
                insert_z(CAMERA_BRIDGE_Z),
            )
            + keepouts.translate(0.0, 0.0, insert_z(6.0));
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed TEER/impedance phantom verification station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained tray");
    println!(
        "  Phantom throughput:     {PHANTOM_CARTRIDGE_COUNT} cartridge nests in {PHANTOM_ROWS}x{PHANTOM_COLS} dry/humid/reference positions"
    );
    println!(
        "  Backplane release:      {POGO_CONTACT_COUNT} pogo/contact alignment witnesses over {POGO_COLUMNS}x{POGO_ROWS} TEER/impedance contact grid"
    );
    println!(
        "  Reference traceability: {REFERENCE_RESISTOR_LANDS} resistor lands, {REFERENCE_CAPACITOR_LANDS} capacitor lands, {CERTIFICATE_LANDS} certificate/barcode lands"
    );
    println!(
        "  Disposition:            {} lanes ({}) with {} total cartridge slots",
        STATUS_LANES,
        STATUS_LANE_NAMES.join(", "),
        STATUS_LANES * STATUS_SLOTS_PER_LANE
    );
    println!(
        "  Evidence and service:   humid shield placeholder, evidence camera bridge, {CABLE_CLIPS} cable strain-relief clips, and {KEEP_OUT_ZONE_COUNT} robot/service keepout zones"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
    assert_eq!(PHANTOM_CARTRIDGE_COUNT, PHANTOM_ROWS * PHANTOM_COLS);
    assert_eq!(POGO_CONTACT_COUNT, POGO_COLUMNS * POGO_ROWS);
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 7] {
    [
        (
            "phantom_cartridge_nests",
            PHANTOM_NEST_POS,
            PHANTOM_NEST_X,
            PHANTOM_NEST_Y,
        ),
        (
            "pogo_contact_alignment_gauge",
            POGO_GAUGE_POS,
            POGO_GAUGE_X,
            POGO_GAUGE_Y,
        ),
        (
            "humid_shield_placeholder",
            HUMID_SHIELD_POS,
            HUMID_SHIELD_X,
            HUMID_SHIELD_Y,
        ),
        (
            "reference_certificate_lands",
            REFERENCE_POS,
            REFERENCE_X,
            REFERENCE_Y,
        ),
        ("cable_strain_relief", CABLE_POS, CABLE_X, CABLE_Y),
        (
            "release_hold_reject_lanes",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        (
            "evidence_camera_bridge",
            CAMERA_BRIDGE_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 6.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 6.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_teer_impedance_phantom_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_teer_impedance_phantom_station_washdown_recess",
        STATION_X - 116.0,
        STATION_Y - 112.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.5);
    let electrical_dry_moat = centered_cube(
        "closed_teer_impedance_phantom_station_electrical_dry_moat",
        476.0,
        250.0,
        5.0,
    )
    .translate(302.0, 155.0, BASE_Z / 2.0 - 2.5);
    let release_lane_sump = centered_cube(
        "closed_teer_impedance_phantom_station_disposition_sump",
        690.0,
        150.0,
        7.0,
    )
    .translate(-190.0, -205.0, BASE_Z / 2.0 - 3.5);
    let drain = centered_cylinder(
        "closed_teer_impedance_phantom_station_front_drain",
        9.0 / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 80.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess
        - electrical_dry_moat
        - release_lane_sump
        - drain
        - insert_sockets()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_channels()
        + robot_fiducial_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_teer_impedance_phantom_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_teer_impedance_phantom_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_teer_impedance_phantom_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_teer_impedance_phantom_station_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_teer_impedance_phantom_station_m6_slot_relief_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_teer_impedance_phantom_station_datum_pin_holes");
    for (i, (x, y)) in [(-506.0, 312.0), (506.0, 312.0), (-506.0, -312.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_teer_impedance_phantom_station_datum_pin_clearance_{i}"),
                5.0 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_teer_impedance_phantom_station_left_containment_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_teer_impedance_phantom_station_right_containment_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_teer_impedance_phantom_station_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_lip = centered_cube(
        "closed_teer_impedance_phantom_station_front_low_drain_lip",
        STATION_X - 170.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 25.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front_lip
}

fn zone_dividers() -> Part {
    let wet_to_dry = centered_cube(
        "closed_teer_impedance_phantom_station_wet_humid_to_electrical_divider",
        STATION_X - 150.0,
        12.0,
        30.0,
    )
    .translate(0.0, 6.0, BASE_Z / 2.0 + 15.0);
    let electrical_to_disposition = centered_cube(
        "closed_teer_impedance_phantom_station_electrical_to_disposition_divider",
        STATION_X - 176.0,
        10.0,
        28.0,
    )
    .translate(0.0, -122.0, BASE_Z / 2.0 + 14.0);
    let phantom_to_backplane = centered_cube(
        "closed_teer_impedance_phantom_station_phantom_to_backplane_divider",
        10.0,
        268.0,
        28.0,
    )
    .translate(74.0, 160.0, BASE_Z / 2.0 + 14.0);
    wet_to_dry + electrical_to_disposition + phantom_to_backplane
}

fn leak_witness_channels() -> Part {
    let mut channels = Part::empty("closed_teer_impedance_phantom_station_leak_witness_channels");
    for i in 0..LEAK_CHANNELS {
        let x = centered_index(i, LEAK_CHANNELS, 174.0);
        channels = channels
            + centered_cube(
                format!("closed_teer_impedance_phantom_station_leak_witness_rib_{i}"),
                112.0,
                6.0,
                7.0,
            )
            .translate(x, -314.0, BASE_Z / 2.0 + 3.5);
    }
    channels
}

fn robot_fiducial_targets() -> Part {
    let mut targets = Part::empty("closed_teer_impedance_phantom_station_robot_fiducials");
    for (i, (x, y)) in [(-488.0, 292.0), (488.0, 292.0), (-488.0, -292.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_teer_impedance_phantom_station_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    targets
}

fn phantom_cartridge_nests() -> Part {
    let body = centered_cube(
        "closed_teer_impedance_phantom_cartridge_nest_block",
        PHANTOM_NEST_X,
        PHANTOM_NEST_Y,
        PHANTOM_NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_teer_impedance_phantom_cartridge_rear_fence",
        PHANTOM_NEST_X,
        14.0,
        PHANTOM_NEST_Z + 30.0,
    )
    .translate(0.0, PHANTOM_NEST_Y / 2.0 - 7.0, 15.0);
    let front_barcode_rail = centered_cube(
        "closed_teer_impedance_phantom_front_barcode_rail",
        PHANTOM_NEST_X - 42.0,
        12.0,
        10.0,
    )
    .translate(
        0.0,
        -(PHANTOM_NEST_Y / 2.0 - 18.0),
        PHANTOM_NEST_Z / 2.0 + 5.0,
    );

    let mut cuts = Part::empty("closed_teer_impedance_phantom_nest_cutouts");
    let mut features = Part::empty("closed_teer_impedance_phantom_nest_features");
    for row in 0..PHANTOM_ROWS {
        for col in 0..PHANTOM_COLS {
            let index = row * PHANTOM_COLS + col;
            let x = centered_index(col, PHANTOM_COLS, PHANTOM_PITCH_X);
            let y = centered_index(row, PHANTOM_ROWS, PHANTOM_PITCH_Y);
            let slot = centered_cube(
                format!("closed_teer_impedance_phantom_slot_{index}"),
                PHANTOM_SLOT_X,
                PHANTOM_SLOT_Y,
                PHANTOM_SLOT_Z,
            )
            .translate(x, y, PHANTOM_NEST_Z / 2.0 - PHANTOM_SLOT_Z / 2.0 + 1.0);
            let finger_relief = centered_cube(
                format!("closed_teer_impedance_phantom_finger_relief_{index}"),
                20.0,
                PHANTOM_SLOT_Y + 10.0,
                20.0,
            )
            .translate(x, y - 2.0, PHANTOM_NEST_Z / 2.0 - 4.0);
            let contact_window = centered_cube(
                format!("closed_teer_impedance_phantom_kelvin_contact_window_{index}"),
                62.0,
                8.0,
                8.0,
            )
            .translate(
                x,
                y + PHANTOM_SLOT_Y / 2.0 - 5.0,
                PHANTOM_NEST_Z / 2.0 + 4.0,
            );
            let latch_land = centered_cube(
                format!("closed_teer_impedance_phantom_spring_latch_land_{index}"),
                48.0,
                8.0,
                6.0,
            )
            .translate(
                x,
                y - PHANTOM_SLOT_Y / 2.0 - 9.0,
                PHANTOM_NEST_Z / 2.0 + 3.0,
            );
            let type_key = centered_cube(
                format!("closed_teer_impedance_phantom_reference_type_key_{index}"),
                18.0 + row as f64 * 5.0,
                6.0,
                6.0,
            )
            .translate(
                x - PHANTOM_SLOT_X / 2.0 + 18.0,
                y,
                PHANTOM_NEST_Z / 2.0 + 3.0,
            );

            cuts = cuts + slot + finger_relief;
            features = features + contact_window + latch_land + type_key;
        }
    }

    body + rear_fence + front_barcode_rail - cuts + features + phantom_row_labels()
}

fn phantom_row_labels() -> Part {
    let mut labels = Part::empty("closed_teer_impedance_phantom_row_label_lands");
    for row in 0..PHANTOM_ROWS {
        let y = centered_index(row, PHANTOM_ROWS, PHANTOM_PITCH_Y);
        labels = labels
            + centered_cube(
                format!("closed_teer_impedance_phantom_row_{row}_range_label_land"),
                64.0,
                18.0,
                5.0,
            )
            .translate(
                -(PHANTOM_NEST_X / 2.0 - 42.0),
                y,
                PHANTOM_NEST_Z / 2.0 + 2.5,
            );
    }
    labels
}

fn pogo_contact_alignment_gauge() -> Part {
    let body = centered_cube(
        "closed_teer_impedance_pogo_alignment_gauge_body",
        POGO_GAUGE_X,
        POGO_GAUGE_Y,
        POGO_GAUGE_Z,
    );
    let backplane_window = centered_cube(
        "closed_teer_impedance_backplane_release_window",
        BACKPLANE_WINDOW_X,
        BACKPLANE_WINDOW_Y,
        POGO_GAUGE_Z + 8.0,
    )
    .translate(-78.0, 0.0, 0.0);
    let pogo_bores = pogo_contact_bores();
    let datum_bores = pogo_datum_bores();
    let stops = pogo_backplane_stops();
    let witness = pogo_witness_lands();
    let wipe_check = centered_cube(
        "closed_teer_impedance_contact_wipe_witness_land",
        108.0,
        BACKPLANE_WINDOW_Y - 26.0,
        6.0,
    )
    .translate(116.0, 0.0, POGO_GAUGE_Z / 2.0 + 3.0);

    body - backplane_window - pogo_bores - datum_bores + stops + witness + wipe_check
}

fn pogo_contact_bores() -> Part {
    let mut bores = Part::empty("closed_teer_impedance_pogo_contact_bores");
    for col in 0..POGO_COLUMNS {
        for row in 0..POGO_ROWS {
            let x = centered_index(col, POGO_COLUMNS, POGO_PITCH_X) - 78.0;
            let y = centered_index(row, POGO_ROWS, POGO_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("closed_teer_impedance_pogo_contact_bore_c{col}_r{row}"),
                    1.4,
                    POGO_GAUGE_Z + 10.0,
                    12,
                )
                .translate(x, y, 0.0);
        }
    }
    bores
}

fn pogo_datum_bores() -> Part {
    let mut bores = Part::empty("closed_teer_impedance_backplane_datum_bores");
    for (i, (x, y)) in [(-154.0, -86.0), (-154.0, 86.0), (0.0, -86.0), (0.0, 86.0)]
        .iter()
        .enumerate()
    {
        bores = bores
            + centered_cylinder(
                format!("closed_teer_impedance_backplane_datum_bore_{i}"),
                2.5,
                POGO_GAUGE_Z + 10.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    bores
}

fn pogo_backplane_stops() -> Part {
    let left_stop = centered_cube(
        "closed_teer_impedance_backplane_left_hard_stop",
        12.0,
        BACKPLANE_WINDOW_Y + 16.0,
        32.0,
    )
    .translate(
        -78.0 - BACKPLANE_WINDOW_X / 2.0 - 12.0,
        0.0,
        POGO_GAUGE_Z / 2.0 + 16.0,
    );
    let rear_stop = centered_cube(
        "closed_teer_impedance_backplane_rear_hard_stop",
        BACKPLANE_WINDOW_X + 34.0,
        12.0,
        32.0,
    )
    .translate(
        -78.0,
        BACKPLANE_WINDOW_Y / 2.0 + 10.0,
        POGO_GAUGE_Z / 2.0 + 16.0,
    );
    let spring_clip = centered_cube(
        "closed_teer_impedance_backplane_spring_clip_land",
        48.0,
        18.0,
        14.0,
    )
    .translate(
        -78.0 + BACKPLANE_WINDOW_X / 2.0 + 18.0,
        -BACKPLANE_WINDOW_Y / 2.0 + 24.0,
        POGO_GAUGE_Z / 2.0 + 7.0,
    );
    left_stop + rear_stop + spring_clip
}

fn pogo_witness_lands() -> Part {
    let mut lands = Part::empty("closed_teer_impedance_pogo_witness_lands");
    for col in 0..POGO_COLUMNS {
        let x = centered_index(col, POGO_COLUMNS, POGO_PITCH_X) - 78.0;
        lands = lands
            + centered_cube(
                format!("closed_teer_impedance_pogo_column_{col}_continuity_land"),
                8.0,
                BACKPLANE_WINDOW_Y + 22.0,
                4.0,
            )
            .translate(x, 0.0, POGO_GAUGE_Z / 2.0 + 2.0);
    }
    for row in [0, 6, 12, 18, 24] {
        let y = centered_index(row, POGO_ROWS, POGO_PITCH_Y);
        lands = lands
            + centered_cube(
                format!("closed_teer_impedance_pogo_row_{row}_range_tick"),
                BACKPLANE_WINDOW_X + 42.0,
                3.0,
                4.0,
            )
            .translate(-78.0, y, POGO_GAUGE_Z / 2.0 + 2.0);
    }
    lands
}

fn humid_shield_placeholder() -> Part {
    let roof = centered_cube(
        "closed_teer_impedance_humid_shield_roof_placeholder",
        HUMID_SHIELD_X,
        18.0,
        16.0,
    )
    .translate(0.0, 0.0, HUMID_SHIELD_Z / 2.0 - 8.0);
    let front_rail = centered_cube(
        "closed_teer_impedance_humid_shield_front_clear_rail",
        HUMID_SHIELD_X,
        12.0,
        HUMID_SHIELD_Z,
    )
    .translate(0.0, -HUMID_SHIELD_Y / 2.0, 0.0);
    let rear_rail = centered_cube(
        "closed_teer_impedance_humid_shield_rear_clear_rail",
        HUMID_SHIELD_X,
        12.0,
        HUMID_SHIELD_Z,
    )
    .translate(0.0, HUMID_SHIELD_Y / 2.0, 0.0);
    let left_rail = centered_cube(
        "closed_teer_impedance_humid_shield_left_clear_rail",
        12.0,
        HUMID_SHIELD_Y,
        HUMID_SHIELD_Z,
    )
    .translate(-HUMID_SHIELD_X / 2.0, 0.0, 0.0);
    let right_rail = centered_cube(
        "closed_teer_impedance_humid_shield_right_clear_rail",
        12.0,
        HUMID_SHIELD_Y,
        HUMID_SHIELD_Z,
    )
    .translate(HUMID_SHIELD_X / 2.0, 0.0, 0.0);
    let gasket_land = centered_cube(
        "closed_teer_impedance_humid_shield_gasket_land",
        HUMID_SHIELD_X - 52.0,
        HUMID_SHIELD_Y - 52.0,
        8.0,
    )
    .translate(0.0, 0.0, -(HUMID_SHIELD_Z / 2.0 - 4.0));
    let access_window = centered_cube(
        "closed_teer_impedance_humid_shield_robot_front_access_window",
        HUMID_SHIELD_X - 160.0,
        22.0,
        HUMID_SHIELD_Z - 70.0,
    )
    .translate(0.0, -HUMID_SHIELD_Y / 2.0, 10.0);
    let cable_passage = centered_cube(
        "closed_teer_impedance_humid_shield_rear_cable_passage",
        164.0,
        26.0,
        60.0,
    )
    .translate(326.0, HUMID_SHIELD_Y / 2.0, -38.0);
    let humidity_sensor_boss = centered_cylinder(
        "closed_teer_impedance_humid_shield_sensor_boss",
        18.0,
        12.0,
        32,
    )
    .translate(
        -330.0,
        HUMID_SHIELD_Y / 2.0 - 30.0,
        HUMID_SHIELD_Z / 2.0 - 20.0,
    );
    let lift_tabs = centered_cube(
        "closed_teer_impedance_humid_shield_left_lift_tab",
        64.0,
        18.0,
        24.0,
    )
    .translate(-410.0, 0.0, HUMID_SHIELD_Z / 2.0 + 12.0)
        + centered_cube(
            "closed_teer_impedance_humid_shield_right_lift_tab",
            64.0,
            18.0,
            24.0,
        )
        .translate(410.0, 0.0, HUMID_SHIELD_Z / 2.0 + 12.0);

    roof + front_rail
        + rear_rail
        + left_rail
        + right_rail
        + gasket_land
        + humidity_sensor_boss
        + lift_tabs
        - access_window
        - cable_passage
}

fn reference_resistor_capacitor_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_teer_impedance_reference_certificate_plate",
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    );
    let resistor_row = reference_component_lands(
        "closed_teer_impedance_reference_resistor",
        REFERENCE_RESISTOR_LANDS,
        -38.0,
        48.0,
        12.0,
    );
    let capacitor_row = reference_component_lands(
        "closed_teer_impedance_reference_capacitor",
        REFERENCE_CAPACITOR_LANDS,
        18.0,
        42.0,
        16.0,
    );
    let mut certificates = Part::empty("closed_teer_impedance_reference_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 42.0);
        certificates = certificates
            + centered_cube(
                format!("closed_teer_impedance_reference_certificate_qr_land_{i}"),
                32.0,
                26.0,
                5.0,
            )
            .translate(x, 58.0, REFERENCE_Z / 2.0 + 2.5);
    }
    let calibration_barcode = centered_cube(
        "closed_teer_impedance_release_run_barcode_strip",
        REFERENCE_X - 52.0,
        16.0,
        5.0,
    )
    .translate(0.0, -(REFERENCE_Y / 2.0 - 20.0), REFERENCE_Z / 2.0 + 2.5);

    plate + resistor_row + capacitor_row + certificates + calibration_barcode
}

fn reference_component_lands(prefix: &str, count: usize, y: f64, land_x: f64, land_y: f64) -> Part {
    let mut lands = Part::empty(format!("{prefix}_lands"));
    for i in 0..count {
        let x = centered_index(i, count, 54.0);
        lands = lands
            + centered_cube(format!("{prefix}_body_land_{i}"), land_x, land_y, 6.0).translate(
                x,
                y,
                REFERENCE_Z / 2.0 + 3.0,
            )
            + centered_cube(
                format!("{prefix}_certificate_tick_{i}"),
                8.0,
                land_y + 14.0,
                4.0,
            )
            .translate(x + land_x / 2.0 + 8.0, y, REFERENCE_Z / 2.0 + 2.0);
    }
    lands
}

fn cable_strain_relief() -> Part {
    let base = centered_cube(
        "closed_teer_impedance_cable_strain_relief_base",
        CABLE_X,
        CABLE_Y,
        CABLE_Z,
    );
    let rear_bulkhead = centered_cube(
        "closed_teer_impedance_cable_rear_bulkhead",
        CABLE_X,
        18.0,
        CABLE_Z + 32.0,
    )
    .translate(0.0, CABLE_Y / 2.0 - 9.0, 16.0);
    let bend_radius_wall = centered_cube(
        "closed_teer_impedance_cable_bend_radius_wall",
        24.0,
        CABLE_Y - 36.0,
        CABLE_Z + 12.0,
    )
    .translate(CABLE_X / 2.0 - 34.0, 0.0, 6.0);
    let mut clips = Part::empty("closed_teer_impedance_cable_comb_clips");
    let mut bores = Part::empty("closed_teer_impedance_cable_clip_bores");
    for i in 0..CABLE_CLIPS {
        let x = centered_index(i, CABLE_CLIPS, 24.0);
        let clip = centered_cube(
            format!("closed_teer_impedance_cable_clip_{i}"),
            18.0,
            42.0,
            26.0,
        )
        .translate(x, -20.0, CABLE_Z / 2.0 + 13.0);
        let bore = centered_cylinder(
            format!("closed_teer_impedance_cable_clip_bore_{i}"),
            CABLE_BORE_D / 2.0,
            48.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -20.0, CABLE_Z / 2.0 + 13.0);
        let tie_slot = centered_cube(
            format!("closed_teer_impedance_cable_tie_slot_{i}"),
            6.0,
            62.0,
            9.0,
        )
        .translate(x, 42.0, CABLE_Z / 2.0 + 4.5);
        clips = clips + clip + tie_slot;
        bores = bores + bore;
    }
    base + rear_bulkhead + bend_radius_wall + clips - bores + cable_id_flags()
}

fn cable_id_flags() -> Part {
    let mut flags = Part::empty("closed_teer_impedance_cable_id_flags");
    for i in 0..4 {
        flags = flags
            + centered_cube(
                format!("closed_teer_impedance_cable_harness_id_flag_{i}"),
                44.0,
                16.0,
                5.0,
            )
            .translate(
                centered_index(i, 4, 58.0),
                -(CABLE_Y / 2.0 - 24.0),
                CABLE_Z / 2.0 + 2.5,
            );
    }
    flags
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "closed_teer_impedance_disposition_lane_plate",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut cuts = Part::empty("closed_teer_impedance_disposition_slot_cuts");
    let mut features = Part::empty("closed_teer_impedance_disposition_lane_features");
    for (lane, name) in STATUS_LANE_NAMES.iter().enumerate() {
        let x = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_X);
        let trough = centered_cube(
            format!("closed_teer_impedance_{name}_lane_trough"),
            156.0,
            DISPOSITION_Y - 34.0,
            18.0,
        )
        .translate(x, 0.0, DISPOSITION_Z / 2.0 - 8.0);
        cuts = cuts + trough;
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = centered_index(slot, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y);
            let slot_cut = centered_cube(
                format!("closed_teer_impedance_{name}_lane_cartridge_slot_{slot}"),
                108.0,
                24.0,
                22.0,
            )
            .translate(x, y, DISPOSITION_Z / 2.0 - 4.0);
            let latch = centered_cube(
                format!("closed_teer_impedance_{name}_lane_latch_land_{slot}"),
                58.0,
                7.0,
                5.0,
            )
            .translate(x, y + 17.0, DISPOSITION_Z / 2.0 + 2.5);
            cuts = cuts + slot_cut;
            features = features + latch;
        }
        let lane_label = centered_cube(
            format!("closed_teer_impedance_{name}_lane_status_label_land"),
            118.0,
            22.0,
            5.0,
        )
        .translate(x, DISPOSITION_Y / 2.0 - 22.0, DISPOSITION_Z / 2.0 + 2.5);
        features = features + lane_label;
    }
    plate - cuts + features + disposition_lane_separators() + reject_quarantine_wall()
}

fn disposition_lane_separators() -> Part {
    let mut separators = Part::empty("closed_teer_impedance_disposition_lane_separators");
    for i in 0..=STATUS_LANES {
        let x =
            -((STATUS_LANES as f64) * STATUS_LANE_PITCH_X) / 2.0 + i as f64 * STATUS_LANE_PITCH_X;
        separators = separators
            + centered_cube(
                format!("closed_teer_impedance_disposition_lane_separator_{i}"),
                8.0,
                DISPOSITION_Y - 16.0,
                34.0,
            )
            .translate(x, -6.0, DISPOSITION_Z / 2.0 + 17.0);
    }
    separators
}

fn reject_quarantine_wall() -> Part {
    let reject_x = centered_index(2, STATUS_LANES, STATUS_LANE_PITCH_X);
    let wall = centered_cube(
        "closed_teer_impedance_reject_lane_quarantine_wall",
        174.0,
        10.0,
        58.0,
    )
    .translate(
        reject_x,
        -(DISPOSITION_Y / 2.0 - 14.0),
        DISPOSITION_Z / 2.0 + 29.0,
    );
    let hold_flag = centered_cube(
        "closed_teer_impedance_hold_lane_retest_flag_land",
        126.0,
        10.0,
        18.0,
    )
    .translate(
        centered_index(1, STATUS_LANES, STATUS_LANE_PITCH_X),
        -(DISPOSITION_Y / 2.0 - 14.0),
        DISPOSITION_Z / 2.0 + 9.0,
    );
    wall + hold_flag
}

fn evidence_camera_bridge() -> Part {
    let left_upright = centered_cube(
        "closed_teer_impedance_evidence_camera_left_upright",
        22.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 34.0, 0.0, 0.0);
    let right_upright = centered_cube(
        "closed_teer_impedance_evidence_camera_right_upright",
        22.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 34.0, 0.0, 0.0);
    let top_beam = centered_cube(
        "closed_teer_impedance_evidence_camera_top_bridge_beam",
        CAMERA_BRIDGE_X - 48.0,
        24.0,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 14.0);
    let camera_sled = centered_cube(
        "closed_teer_impedance_evidence_camera_adjustable_sled",
        118.0,
        54.0,
        24.0,
    )
    .translate(36.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 44.0);
    let lens = centered_cylinder(
        "closed_teer_impedance_evidence_camera_lens_placeholder",
        22.0,
        34.0,
        36,
    )
    .translate(36.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 78.0);
    let field_frame = centered_cube(
        "closed_teer_impedance_evidence_camera_field_of_view_front_edge",
        CAMERA_FIELD_X,
        6.0,
        6.0,
    )
    .translate(0.0, -CAMERA_FIELD_Y / 2.0, -(CAMERA_BRIDGE_Z / 2.0 - 5.0))
        + centered_cube(
            "closed_teer_impedance_evidence_camera_field_of_view_rear_edge",
            CAMERA_FIELD_X,
            6.0,
            6.0,
        )
        .translate(0.0, CAMERA_FIELD_Y / 2.0, -(CAMERA_BRIDGE_Z / 2.0 - 5.0))
        + centered_cube(
            "closed_teer_impedance_evidence_camera_field_of_view_left_edge",
            6.0,
            CAMERA_FIELD_Y,
            6.0,
        )
        .translate(-CAMERA_FIELD_X / 2.0, 0.0, -(CAMERA_BRIDGE_Z / 2.0 - 5.0))
        + centered_cube(
            "closed_teer_impedance_evidence_camera_field_of_view_right_edge",
            6.0,
            CAMERA_FIELD_Y,
            6.0,
        )
        .translate(CAMERA_FIELD_X / 2.0, 0.0, -(CAMERA_BRIDGE_Z / 2.0 - 5.0));
    let light_bars = centered_cube(
        "closed_teer_impedance_evidence_camera_left_light_bar",
        240.0,
        12.0,
        16.0,
    )
    .translate(
        -150.0,
        -CAMERA_BRIDGE_Y / 2.0 + 24.0,
        CAMERA_BRIDGE_Z / 2.0 - 64.0,
    ) + centered_cube(
        "closed_teer_impedance_evidence_camera_right_light_bar",
        240.0,
        12.0,
        16.0,
    )
    .translate(
        150.0,
        CAMERA_BRIDGE_Y / 2.0 - 24.0,
        CAMERA_BRIDGE_Z / 2.0 - 64.0,
    );

    left_upright
        + right_upright
        + top_beam
        + camera_sled
        + lens
        + field_frame
        + light_bars
        + camera_calibration_targets()
}

fn camera_calibration_targets() -> Part {
    let mut targets = Part::empty("closed_teer_impedance_camera_calibration_targets");
    for (i, x) in [-246.0, -82.0, 82.0, 246.0].iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "closed_teer_impedance_camera_evidence_fiducial_target_{i}"
            ))
            .translate(*x, -72.0, -(CAMERA_BRIDGE_Z / 2.0 - 2.0));
    }
    targets
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_teer_impedance_keepout_front_robot_pick_sweep",
        STATION_X - 160.0,
        10.0,
        6.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 34.0), 0.0);
    let front_depth_marker = centered_cube(
        "closed_teer_impedance_keepout_front_robot_depth_marker",
        STATION_X - 180.0,
        FRONT_ROBOT_SWEEP,
        4.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 + FRONT_ROBOT_SWEEP / 2.0), 0.0);
    let rear_service = centered_cube(
        "closed_teer_impedance_keepout_rear_cable_service_marker",
        STATION_X - 150.0,
        10.0,
        6.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 34.0, 0.0);
    let rear_depth_marker = centered_cube(
        "closed_teer_impedance_keepout_rear_service_depth_marker",
        STATION_X - 180.0,
        REAR_CABLE_SERVICE,
        4.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_CABLE_SERVICE / 2.0, 0.0);
    let left_service = centered_cube(
        "closed_teer_impedance_keepout_left_phantom_load_service_lane",
        10.0,
        STATION_Y - 140.0,
        6.0,
    )
    .translate(-(STATION_X / 2.0 - 40.0), 0.0, 0.0);
    let left_depth_marker = centered_cube(
        "closed_teer_impedance_keepout_left_service_depth_marker",
        SIDE_SERVICE_CLEARANCE,
        STATION_Y - 180.0,
        4.0,
    )
    .translate(-(STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0), 0.0, 0.0);
    let right_service = centered_cube(
        "closed_teer_impedance_keepout_right_electrical_service_lane",
        10.0,
        STATION_Y - 140.0,
        6.0,
    )
    .translate(STATION_X / 2.0 - 40.0, 0.0, 0.0);
    let shield_lift = centered_cube(
        "closed_teer_impedance_keepout_humid_shield_lift_envelope",
        HUMID_SHIELD_X,
        10.0,
        6.0,
    )
    .translate(HUMID_SHIELD_POS.0, HUMID_SHIELD_POS.1, 0.0);
    let camera_service = centered_cube(
        "closed_teer_impedance_keepout_camera_bridge_service_line",
        CAMERA_BRIDGE_X,
        10.0,
        6.0,
    )
    .translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, 0.0);
    let top_lift_height_flag = centered_cube(
        "closed_teer_impedance_keepout_shield_lift_height_flag",
        112.0,
        14.0,
        SHIELD_LIFT_CLEARANCE / 8.0,
    )
    .translate(
        HUMID_SHIELD_POS.0 + HUMID_SHIELD_X / 2.0 - 74.0,
        HUMID_SHIELD_POS.1,
        SHIELD_LIFT_CLEARANCE / 16.0,
    );

    front_robot
        + front_depth_marker
        + rear_service
        + rear_depth_marker
        + left_service
        + left_depth_marker
        + right_service
        + shield_lift
        + camera_service
        + top_lift_height_flag
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center"), 1.2, 3.0, 18);
    disc - center
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_teer_impedance_phantom_verification_station_"),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn required_feature_list_covers_release_station_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 9);
        assert!(REQUIRED_FEATURES.contains(&"phantom_cartridge_nests"));
        assert!(REQUIRED_FEATURES.contains(&"pogo_contact_alignment_gauge"));
        assert!(REQUIRED_FEATURES.contains(&"humid_shield_placeholder"));
        assert!(REQUIRED_FEATURES.contains(&"reference_resistor_capacitor_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"cable_strain_relief"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn all_insert_modules_fit_inside_containment_rims() {
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
    }

    #[test]
    fn phantom_nests_and_contact_grid_match_backplane_release_batch() {
        assert_eq!(PHANTOM_CARTRIDGE_COUNT, 12);
        assert_eq!(PHANTOM_ROWS, 3);
        assert_eq!(PHANTOM_COLS, 4);
        assert_eq!(POGO_CONTACT_COUNT, 100);
        assert_eq!(POGO_COLUMNS, 4);
        assert_eq!(POGO_ROWS, 25);
        assert!(BACKPLANE_WINDOW_Y > POGO_PITCH_Y * (POGO_ROWS as f64 - 1.0));
        assert!(BACKPLANE_WINDOW_X > POGO_PITCH_X * (POGO_COLUMNS as f64 - 1.0));
    }

    #[test]
    fn humid_shield_covers_phantom_nests_and_pogo_gauge() {
        let shield_left = HUMID_SHIELD_POS.0 - HUMID_SHIELD_X / 2.0;
        let shield_right = HUMID_SHIELD_POS.0 + HUMID_SHIELD_X / 2.0;
        let nest_left = PHANTOM_NEST_POS.0 - PHANTOM_NEST_X / 2.0;
        let pogo_right = POGO_GAUGE_POS.0 + POGO_GAUGE_X / 2.0;
        assert!(shield_left <= nest_left + 50.0);
        assert!(shield_right >= pogo_right - 50.0);
        assert!(HUMID_SHIELD_Z >= 180.0);
        assert!(SHIELD_LIFT_CLEARANCE > HUMID_SHIELD_Z);
    }

    #[test]
    fn reference_lands_include_resistors_capacitors_and_certificates() {
        assert_eq!(REFERENCE_RESISTOR_LANDS, 6);
        assert_eq!(REFERENCE_CAPACITOR_LANDS, 6);
        assert_eq!(CERTIFICATE_LANDS, 8);
        assert!(REFERENCE_X >= 360.0);
        assert!(REFERENCE_Y >= 140.0);
    }

    #[test]
    fn disposition_lanes_are_release_hold_reject_with_capacity() {
        assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert_eq!(
            STATUS_LANES * STATUS_SLOTS_PER_LANE,
            PHANTOM_CARTRIDGE_COUNT
        );
        assert!(DISPOSITION_X > STATUS_LANE_PITCH_X * STATUS_LANES as f64);
    }

    #[test]
    fn cable_camera_and_service_clearances_are_explicit() {
        assert_eq!(CABLE_CLIPS, 10);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 6);
        assert!(CABLE_BORE_D >= 6.0);
        assert!(CAMERA_FIELD_X >= 500.0);
        assert!(CAMERA_BRIDGE_Z > HUMID_SHIELD_Z);
        assert!(FRONT_ROBOT_SWEEP >= 400.0);
        assert!(REAR_CABLE_SERVICE >= 240.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 160.0);
    }
}
