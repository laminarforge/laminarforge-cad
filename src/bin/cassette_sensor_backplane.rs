use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Dry electrical/sensor backplane for the 20-chip cassette / sealed module.
//
// Intent:
// - Present a 4x5 grid of dry contact-pad islands matching the cassette chip pitch.
// - Hold compliant spring pins in an insulated carrier above those pads.
// - Reserve a rear TEER/impedance multiplex connector zone and cable exit.
// - Provide keyed alignment, dry gasket lands, insulation standoffs, and fiducials.
// - Record per-chip/cell-type signals within one cassette-level AAV condition;
//   candidate comparisons happen across matched cassettes or sealed modules.
//
// This is mechanical/electrical interface architecture only. It is not a sealed
// culture boundary, validated electrode layout, or final PCB design.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const BACKPLANE_X: f64 = CASSETTE_X + 64.0;
const BACKPLANE_Y: f64 = CASSETTE_Y + 78.0;
const SUBSTRATE_Z: f64 = 4.0;
const PIN_CARRIER_Z: f64 = 10.0;
const GASKET_Z: f64 = 3.0;
const STANDOFF_Z: f64 = 8.0;

const CONTACTS_PER_CHIP: usize = 6;
const CONTACT_PAD_X: f64 = 10.0;
const CONTACT_PAD_Y: f64 = 8.0;
const CONTACT_PAD_Z: f64 = 0.8;
const CONTACT_PAD_GAP_X: f64 = 3.0;
const CONTACT_PAD_GAP_Y: f64 = 3.0;
const CONTACT_ISLAND_X: f64 = 58.0;
const CONTACT_ISLAND_Y: f64 = 30.0;
const CONTACT_ISLAND_Z: f64 = 0.7;

const PIN_BORE_DIAMETER: f64 = 2.1;
const PIN_TIP_DIAMETER: f64 = 1.3;
const PIN_TIP_Z: f64 = STANDOFF_Z;
const PIN_COLLAR_DIAMETER: f64 = 4.4;
const PIN_COLLAR_Z: f64 = 2.0;

const CONNECTOR_ZONE_X: f64 = 280.0;
const CONNECTOR_ZONE_Y: f64 = 78.0;
const CONNECTOR_ZONE_Z: f64 = 30.0;
const CABLE_EXIT_Y: f64 = 44.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let substrate = backplane_substrate();
    write_part(&substrate, "output/cassette_sensor_backplane_substrate.stl");

    let carrier = spring_pin_carrier();
    write_part(
        &carrier,
        "output/cassette_sensor_backplane_spring_pin_carrier.stl",
    );

    let gasket = gasket_and_standoffs();
    write_part(
        &gasket,
        "output/cassette_sensor_backplane_gasket_standoffs.stl",
    );

    let connector = connector_zone();
    write_part(
        &connector,
        "output/cassette_sensor_backplane_connector_zone.stl",
    );

    let assembly = substrate
        + gasket.translate(0.0, 0.0, SUBSTRATE_Z / 2.0 + GASKET_Z / 2.0)
        + carrier.translate(
            0.0,
            0.0,
            SUBSTRATE_Z / 2.0 + STANDOFF_Z + PIN_CARRIER_Z / 2.0,
        )
        + connector.translate(
            0.0,
            BACKPLANE_Y / 2.0 + CONNECTOR_ZONE_Y / 2.0,
            SUBSTRATE_Z / 2.0 + CONNECTOR_ZONE_Z / 2.0,
        );
    write_part(&assembly, "output/cassette_sensor_backplane_assembly.stl");

    println!(
        "Cassette sensor backplane: {COLS}x{ROWS} chip grid for one cassette-level AAV condition, {CONTACTS_PER_CHIP} dry contacts/chip, {:.0}mm x {:.0}mm substrate, {:.0}mm rear TEER/impedance connector zone.",
        BACKPLANE_X, BACKPLANE_Y, CONNECTOR_ZONE_X
    );
}

fn write_part(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn backplane_substrate() -> Part {
    let body = centered_cube(
        "cassette_sensor_backplane_substrate_body",
        BACKPLANE_X,
        BACKPLANE_Y,
        SUBSTRATE_Z,
    );

    body - mounting_holes() - alignment_receiver_cuts() - rear_connector_clearance()
        + contact_pad_array()
        + substrate_fiducials()
        + substrate_edge_datum_marks()
}

fn contact_pad_array() -> Part {
    let mut pads = Part::empty("cassette_sensor_backplane_contact_pad_array");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let island = centered_cube(
                format!("cassette_backplane_contact_island_{col}_{row}"),
                CONTACT_ISLAND_X,
                CONTACT_ISLAND_Y,
                CONTACT_ISLAND_Z,
            )
            .translate(x, y, SUBSTRATE_Z / 2.0 + CONTACT_ISLAND_Z / 2.0);
            pads = pads + island;

            for pin in 0..CONTACTS_PER_CHIP {
                let (dx, dy) = contact_offset(pin);
                let pad = centered_cube(
                    format!("cassette_backplane_contact_pad_{col}_{row}_{pin}"),
                    CONTACT_PAD_X,
                    CONTACT_PAD_Y,
                    CONTACT_PAD_Z,
                )
                .translate(
                    x + dx,
                    y + dy,
                    SUBSTRATE_Z / 2.0 + CONTACT_ISLAND_Z + CONTACT_PAD_Z / 2.0,
                );
                pads = pads + pad;
            }
        }
    }

    pads
}

fn spring_pin_carrier() -> Part {
    let body = centered_cube(
        "cassette_sensor_backplane_spring_pin_carrier_body",
        BACKPLANE_X - 20.0,
        BACKPLANE_Y - 22.0,
        PIN_CARRIER_Z,
    );

    body - pin_bores() - carrier_lightening_windows() - carrier_mount_relief()
        + pin_collars_and_tips()
        + carrier_alignment_posts()
        + carrier_edge_stiffeners()
}

fn pin_bores() -> Part {
    let mut bores = Part::empty("cassette_sensor_backplane_pin_bores");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            for pin in 0..CONTACTS_PER_CHIP {
                let (dx, dy) = contact_offset(pin);
                bores = bores
                    + centered_cylinder(
                        format!("cassette_backplane_pin_bore_{col}_{row}_{pin}"),
                        PIN_BORE_DIAMETER / 2.0,
                        PIN_CARRIER_Z + 2.0,
                        24,
                    )
                    .translate(x + dx, y + dy, 0.0);
            }
        }
    }

    bores
}

fn pin_collars_and_tips() -> Part {
    let mut hardware = Part::empty("cassette_sensor_backplane_pin_collars_and_tips");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            for pin in 0..CONTACTS_PER_CHIP {
                let (dx, dy) = contact_offset(pin);
                let px = x + dx;
                let py = y + dy;

                let collar = centered_cylinder(
                    format!("cassette_backplane_pin_collar_{col}_{row}_{pin}"),
                    PIN_COLLAR_DIAMETER / 2.0,
                    PIN_COLLAR_Z,
                    24,
                )
                .translate(px, py, PIN_CARRIER_Z / 2.0 + PIN_COLLAR_Z / 2.0)
                    - centered_cylinder(
                        format!("cassette_backplane_pin_collar_bore_{col}_{row}_{pin}"),
                        PIN_BORE_DIAMETER / 2.0,
                        PIN_COLLAR_Z + 0.4,
                        24,
                    )
                    .translate(
                        px,
                        py,
                        PIN_CARRIER_Z / 2.0 + PIN_COLLAR_Z / 2.0,
                    );

                let tip = centered_cylinder(
                    format!("cassette_backplane_spring_pin_tip_{col}_{row}_{pin}"),
                    PIN_TIP_DIAMETER / 2.0,
                    PIN_TIP_Z,
                    20,
                )
                .translate(px, py, -(PIN_CARRIER_Z / 2.0 + PIN_TIP_Z / 2.0));

                hardware = hardware + collar + tip;
            }
        }
    }

    hardware
}

fn carrier_lightening_windows() -> Part {
    let mut windows = Part::empty("cassette_sensor_backplane_carrier_lightening_windows");

    for row in 0..ROWS - 1 {
        let (_, y_a) = chip_center(0, row);
        let (_, y_b) = chip_center(0, row + 1);
        windows = windows
            + centered_cube(
                format!("cassette_backplane_carrier_row_service_slot_{row}"),
                CASSETTE_X - 36.0,
                9.0,
                PIN_CARRIER_Z + 2.0,
            )
            .translate(0.0, (y_a + y_b) / 2.0, 0.0);
    }

    for col in 0..COLS - 1 {
        let (x_a, _) = chip_center(col, 0);
        let (x_b, _) = chip_center(col + 1, 0);
        windows = windows
            + centered_cube(
                format!("cassette_backplane_carrier_column_service_slot_{col}"),
                9.0,
                CASSETTE_Y - 34.0,
                PIN_CARRIER_Z + 2.0,
            )
            .translate((x_a + x_b) / 2.0, 0.0, 0.0);
    }

    windows
}

fn carrier_mount_relief() -> Part {
    mounting_holes().scale(1.05, 1.05, 1.0)
}

fn carrier_edge_stiffeners() -> Part {
    let front = centered_cube(
        "cassette_sensor_backplane_front_edge_stiffener",
        BACKPLANE_X - 46.0,
        12.0,
        8.0,
    )
    .translate(0.0, -(BACKPLANE_Y / 2.0 - 22.0), 1.0);
    let rear = centered_cube(
        "cassette_sensor_backplane_rear_edge_stiffener",
        BACKPLANE_X - 46.0,
        12.0,
        8.0,
    )
    .translate(0.0, BACKPLANE_Y / 2.0 - 22.0, 1.0);
    let left = centered_cube(
        "cassette_sensor_backplane_left_edge_stiffener",
        12.0,
        BACKPLANE_Y - 70.0,
        8.0,
    )
    .translate(-(BACKPLANE_X / 2.0 - 22.0), 0.0, 1.0);
    let right = centered_cube(
        "cassette_sensor_backplane_right_edge_stiffener",
        12.0,
        BACKPLANE_Y - 70.0,
        8.0,
    )
    .translate(BACKPLANE_X / 2.0 - 22.0, 0.0, 1.0);

    front + rear + left + right
}

fn gasket_and_standoffs() -> Part {
    let mut features = perimeter_gasket();

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let gasket = rectangular_frame(
                &format!("cassette_backplane_chip_gasket_{col}_{row}"),
                CONTACT_ISLAND_X + 18.0,
                CONTACT_ISLAND_Y + 18.0,
                3.2,
                GASKET_Z,
            )
            .translate(x, y, 0.0);

            let standoff = insulation_standoff(&format!(
                "cassette_backplane_insulation_standoff_{col}_{row}"
            ))
            .translate(
                x - CONTACT_ISLAND_X / 2.0 - 13.0,
                y,
                (STANDOFF_Z - GASKET_Z) / 2.0,
            );

            features = features + gasket + standoff;
        }
    }

    features + row_insulation_ribs()
}

fn perimeter_gasket() -> Part {
    rectangular_frame(
        "cassette_sensor_backplane_perimeter_gasket",
        CASSETTE_X + 32.0,
        CASSETTE_Y + 30.0,
        5.0,
        GASKET_Z,
    )
}

fn row_insulation_ribs() -> Part {
    let mut ribs = Part::empty("cassette_sensor_backplane_row_insulation_ribs");

    for row in 0..ROWS - 1 {
        let (_, y_a) = chip_center(0, row);
        let (_, y_b) = chip_center(0, row + 1);
        ribs = ribs
            + centered_cube(
                format!("cassette_backplane_row_insulation_rib_{row}"),
                CASSETTE_X + 18.0,
                4.0,
                GASKET_Z,
            )
            .translate(0.0, (y_a + y_b) / 2.0, 0.0);
    }

    for col in 0..COLS - 1 {
        let (x_a, _) = chip_center(col, 0);
        let (x_b, _) = chip_center(col + 1, 0);
        ribs = ribs
            + centered_cube(
                format!("cassette_backplane_col_insulation_rib_{col}"),
                4.0,
                CASSETTE_Y + 16.0,
                GASKET_Z,
            )
            .translate((x_a + x_b) / 2.0, 0.0, 0.0);
    }

    ribs
}

fn insulation_standoff(name: &str) -> Part {
    centered_cylinder(format!("{name}_body"), 4.8, STANDOFF_Z, 28)
        - centered_cylinder(
            format!("{name}_m25_clearance"),
            2.8 / 2.0,
            STANDOFF_Z + 1.0,
            20,
        )
}

fn connector_zone() -> Part {
    let body = centered_cube(
        "cassette_sensor_backplane_connector_zone_body",
        CONNECTOR_ZONE_X,
        CONNECTOR_ZONE_Y,
        CONNECTOR_ZONE_Z,
    );

    body - connector_socket_recesses() - cable_exit_cuts() - connector_mount_holes()
        + connector_shells()
        + mux_heat_spreader_land()
        + cable_exit_shroud()
        + connector_alignment_tabs()
}

fn connector_socket_recesses() -> Part {
    let teer_recess = centered_cube(
        "cassette_backplane_teer_mux_socket_recess",
        92.0,
        18.0,
        10.0,
    )
    .translate(-62.0, -CONNECTOR_ZONE_Y / 2.0 + 10.0, 4.0);
    let impedance_recess = centered_cube(
        "cassette_backplane_impedance_socket_recess",
        78.0,
        18.0,
        10.0,
    )
    .translate(58.0, -CONNECTOR_ZONE_Y / 2.0 + 10.0, 4.0);
    let programming_recess = centered_cube(
        "cassette_backplane_service_programming_recess",
        42.0,
        16.0,
        8.0,
    )
    .translate(
        CONNECTOR_ZONE_X / 2.0 - 34.0,
        -CONNECTOR_ZONE_Y / 2.0 + 9.0,
        -7.0,
    );

    teer_recess + impedance_recess + programming_recess
}

fn connector_shells() -> Part {
    let teer_shell = connector_shell("teer_mux", 92.0, 24.0, 16.0).translate(-62.0, -18.0, -4.0);
    let impedance_shell =
        connector_shell("impedance_afe", 78.0, 24.0, 16.0).translate(58.0, -18.0, -4.0);
    let shield_bus = centered_cube(
        "cassette_backplane_analog_guard_shield_bus",
        208.0,
        8.0,
        4.0,
    )
    .translate(0.0, 13.0, CONNECTOR_ZONE_Z / 2.0 + 2.0);

    teer_shell + impedance_shell + shield_bus + connector_pin_fields()
}

fn connector_shell(name: &str, x: f64, y: f64, z: f64) -> Part {
    let outer = centered_cube(format!("cassette_backplane_{name}_shell_outer"), x, y, z);
    let inner = centered_cube(
        format!("cassette_backplane_{name}_shell_inner"),
        x - 10.0,
        y - 8.0,
        z + 1.0,
    )
    .translate(0.0, -1.0, 1.5);

    outer - inner
}

fn connector_pin_fields() -> Part {
    let mut pins = Part::empty("cassette_sensor_backplane_connector_pin_fields");

    for (field, origin_x, count) in [("teer", -62.0, 32_usize), ("impedance", 58.0, 24_usize)] {
        for i in 0..count {
            let col = i % 16;
            let row = i / 16;
            let x = origin_x - 37.5 + col as f64 * 5.0;
            let y = -28.0 + row as f64 * 5.0;
            pins = pins
                + centered_cube(
                    format!("cassette_backplane_{field}_connector_pin_{i}"),
                    2.0,
                    2.0,
                    3.0,
                )
                .translate(x, y, CONNECTOR_ZONE_Z / 2.0 + 1.5);
        }
    }

    pins
}

fn cable_exit_cuts() -> Part {
    let round_exit = centered_cylinder(
        "cassette_sensor_backplane_cable_exit_round_cut",
        9.0,
        CONNECTOR_ZONE_Y + CABLE_EXIT_Y + 10.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 0.0);
    let flat_key = centered_cube(
        "cassette_sensor_backplane_cable_exit_flat_key",
        36.0,
        CONNECTOR_ZONE_Y + CABLE_EXIT_Y + 12.0,
        8.0,
    )
    .translate(0.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 3.0);

    round_exit + flat_key
}

fn cable_exit_shroud() -> Part {
    let shroud = centered_cube(
        "cassette_sensor_backplane_cable_exit_shroud_body",
        72.0,
        CABLE_EXIT_Y,
        CONNECTOR_ZONE_Z,
    )
    .translate(0.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 0.0);

    let bore = centered_cylinder(
        "cassette_sensor_backplane_cable_exit_shroud_bore",
        9.0,
        CABLE_EXIT_Y + 8.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 0.0);

    let rib_left = centered_cube(
        "cassette_sensor_backplane_cable_strain_relief_left",
        8.0,
        CABLE_EXIT_Y + 8.0,
        22.0,
    )
    .translate(-28.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 0.0);
    let rib_right = centered_cube(
        "cassette_sensor_backplane_cable_strain_relief_right",
        8.0,
        CABLE_EXIT_Y + 8.0,
        22.0,
    )
    .translate(28.0, CONNECTOR_ZONE_Y / 2.0 + CABLE_EXIT_Y / 2.0, 0.0);

    shroud - bore + rib_left + rib_right
}

fn mux_heat_spreader_land() -> Part {
    let analog_mux = centered_cube("cassette_backplane_mux_mezzanine_land", 118.0, 36.0, 3.0)
        .translate(-62.0, 14.0, CONNECTOR_ZONE_Z / 2.0 + 1.5);
    let impedance_afe = centered_cube("cassette_backplane_impedance_afe_land", 92.0, 36.0, 3.0)
        .translate(58.0, 14.0, CONNECTOR_ZONE_Z / 2.0 + 1.5);

    analog_mux + impedance_afe
}

fn connector_alignment_tabs() -> Part {
    let left_tab = centered_cube(
        "cassette_backplane_connector_left_key_tab",
        16.0,
        12.0,
        20.0,
    )
    .translate(
        -CONNECTOR_ZONE_X / 2.0 + 22.0,
        -CONNECTOR_ZONE_Y / 2.0 - 4.0,
        0.0,
    );
    let right_tab = centered_cube(
        "cassette_backplane_connector_right_key_tab",
        28.0,
        12.0,
        20.0,
    )
    .translate(
        CONNECTOR_ZONE_X / 2.0 - 28.0,
        -CONNECTOR_ZONE_Y / 2.0 - 4.0,
        0.0,
    );

    left_tab + right_tab
}

fn connector_mount_holes() -> Part {
    let mut holes = Part::empty("cassette_sensor_backplane_connector_mount_holes");

    for (i, x) in [
        -(CONNECTOR_ZONE_X / 2.0 - 20.0),
        -94.0,
        94.0,
        CONNECTOR_ZONE_X / 2.0 - 20.0,
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cassette_backplane_connector_m3_hole_{i}"),
                3.4 / 2.0,
                CONNECTOR_ZONE_Z + 2.0,
                24,
            )
            .translate(*x, 24.0, 0.0);
    }

    holes
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("cassette_sensor_backplane_mount_holes");

    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("cassette_backplane_m4_mount_hole_{i}"),
                4.3 / 2.0,
                PIN_CARRIER_Z + SUBSTRATE_Z + 6.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }

    holes
}

fn alignment_receiver_cuts() -> Part {
    let round = centered_cylinder(
        "cassette_backplane_alignment_round_receiver",
        3.4,
        SUBSTRATE_Z + 2.0,
        32,
    )
    .translate(-(BACKPLANE_X / 2.0 - 42.0), BACKPLANE_Y / 2.0 - 44.0, 0.0);
    let diamond = centered_cube(
        "cassette_backplane_alignment_key_receiver",
        18.0,
        8.0,
        SUBSTRATE_Z + 2.0,
    )
    .translate(BACKPLANE_X / 2.0 - 44.0, BACKPLANE_Y / 2.0 - 44.0, 0.0);
    let asymmetric_front_key = centered_cube(
        "cassette_backplane_front_asymmetric_key_notch",
        42.0,
        12.0,
        SUBSTRATE_Z + 2.0,
    )
    .translate(-(BACKPLANE_X / 2.0 - 86.0), -(BACKPLANE_Y / 2.0 - 4.0), 0.0);

    round + diamond + asymmetric_front_key
}

fn carrier_alignment_posts() -> Part {
    let post_z = STANDOFF_Z + 2.0;
    let post_center_z = -(PIN_CARRIER_Z / 2.0 + post_z / 2.0);
    let round_post = centered_cylinder("cassette_backplane_round_alignment_post", 3.0, post_z, 32)
        .translate(
            -(BACKPLANE_X / 2.0 - 42.0),
            BACKPLANE_Y / 2.0 - 44.0,
            post_center_z,
        );
    let key_blade = centered_cube(
        "cassette_backplane_keyed_alignment_blade",
        15.0,
        5.8,
        post_z,
    )
    .translate(
        BACKPLANE_X / 2.0 - 44.0,
        BACKPLANE_Y / 2.0 - 44.0,
        post_center_z,
    );
    let front_stop = centered_cube(
        "cassette_backplane_asymmetric_front_key_stop",
        36.0,
        8.0,
        post_z,
    )
    .translate(
        -(BACKPLANE_X / 2.0 - 86.0),
        -(BACKPLANE_Y / 2.0 - 8.0),
        post_center_z,
    );

    round_post + key_blade + front_stop
}

fn rear_connector_clearance() -> Part {
    centered_cube(
        "cassette_sensor_backplane_rear_connector_clearance",
        CONNECTOR_ZONE_X + 18.0,
        14.0,
        SUBSTRATE_Z + 2.0,
    )
    .translate(0.0, BACKPLANE_Y / 2.0 - 5.0, 0.0)
}

fn substrate_fiducials() -> Part {
    let z = SUBSTRATE_Z / 2.0 + 0.7;
    fiducial_target("front_left").translate(
        -(BACKPLANE_X / 2.0 - 36.0),
        -(BACKPLANE_Y / 2.0 - 36.0),
        z,
    ) + fiducial_target("front_right").translate(
        BACKPLANE_X / 2.0 - 36.0,
        -(BACKPLANE_Y / 2.0 - 36.0),
        z,
    ) + fiducial_target("rear_left").translate(
        -(BACKPLANE_X / 2.0 - 36.0),
        BACKPLANE_Y / 2.0 - 74.0,
        z,
    )
}

fn substrate_edge_datum_marks() -> Part {
    let x_datum = centered_cube("cassette_backplane_x_datum_land", 54.0, 4.0, 1.0).translate(
        -(BACKPLANE_X / 2.0 - 110.0),
        BACKPLANE_Y / 2.0 - 22.0,
        SUBSTRATE_Z / 2.0 + 0.5,
    );
    let y_datum = centered_cube("cassette_backplane_y_datum_land", 4.0, 54.0, 1.0).translate(
        -(BACKPLANE_X / 2.0 - 22.0),
        BACKPLANE_Y / 2.0 - 110.0,
        SUBSTRATE_Z / 2.0 + 0.5,
    );

    x_datum + y_datum
}

fn fiducial_target(name: &str) -> Part {
    let disk = centered_cylinder(format!("cassette_backplane_fiducial_{name}"), 6.0, 1.4, 40);
    let center = centered_cylinder(
        format!("cassette_backplane_fiducial_{name}_center"),
        1.6,
        1.8,
        24,
    );
    let x_groove = centered_cube(
        format!("cassette_backplane_fiducial_{name}_x_groove"),
        14.0,
        1.1,
        1.8,
    );
    let y_groove = centered_cube(
        format!("cassette_backplane_fiducial_{name}_y_groove"),
        1.1,
        14.0,
        1.8,
    );

    disk - center - x_groove - y_groove
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, wall: f64, z: f64) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner"),
            outer_x - wall * 2.0,
            outer_y - wall * 2.0,
            z + 0.2,
        )
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    let x = -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * PITCH_X;
    let y = -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * PITCH_Y;
    (x, y)
}

fn contact_offset(pin: usize) -> (f64, f64) {
    let col = (pin % 3) as f64;
    let row = (pin / 3) as f64;
    let x = (col - 1.0) * (CONTACT_PAD_X + CONTACT_PAD_GAP_X);
    let y = (row - 0.5) * (CONTACT_PAD_Y + CONTACT_PAD_GAP_Y);
    (x, y)
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-(BACKPLANE_X / 2.0 - 24.0), -(BACKPLANE_Y / 2.0 - 24.0)),
        (BACKPLANE_X / 2.0 - 24.0, -(BACKPLANE_Y / 2.0 - 24.0)),
        (-(BACKPLANE_X / 2.0 - 24.0), BACKPLANE_Y / 2.0 - 24.0),
        (BACKPLANE_X / 2.0 - 24.0, BACKPLANE_Y / 2.0 - 24.0),
        (0.0, -(BACKPLANE_Y / 2.0 - 24.0)),
        (0.0, BACKPLANE_Y / 2.0 - 24.0),
        (-(BACKPLANE_X / 2.0 - 24.0), 0.0),
        (BACKPLANE_X / 2.0 - 24.0, 0.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cassette_grid_has_twenty_chip_positions() {
        assert_eq!(COLS * ROWS, 20);
        assert_eq!(PITCH_X, REVC_CHIP_LENGTH + GUTTER);
        assert_eq!(PITCH_Y, REVC_CHIP_WIDTH + GUTTER);
    }

    #[test]
    fn contact_array_has_expected_dry_pins() {
        assert_eq!(CONTACTS_PER_CHIP * COLS * ROWS, 120);
    }

    #[test]
    fn backplane_covers_existing_cassette_envelope() {
        assert!(BACKPLANE_X > CASSETTE_X);
        assert!(BACKPLANE_Y > CASSETTE_Y);
    }
}
