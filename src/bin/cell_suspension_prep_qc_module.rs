use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Cell suspension preparation and QC module for the automated seeding/coating station.
//
// Intent:
// - Keep cell suspension preparation in a closed sterile fluid path.
// - Package a sterile bag/reservoir holder, gentle mixing/recirculation,
//   temperature hold, sampling/QC loop, bubble/dead-volume control, sterile
//   connector panel, prime/waste path, and service-skid handoff manifold.
// - Feed the automated_seeding_coating_station row input interface without
//   manual pipetting.
//
// This is a CAD architecture placeholder. Sterile connector selection, tubing
// set validation, cell compatibility, sensor calibration, and process release
// criteria stay as separate gates.
//
// Exports:
//   output/cell_suspension_prep_qc_module_baseplate.stl
//   output/cell_suspension_prep_qc_module_bag_holder.stl
//   output/cell_suspension_prep_qc_module_temperature_hold_zone.stl
//   output/cell_suspension_prep_qc_module_mixing_recirculation.stl
//   output/cell_suspension_prep_qc_module_qc_loop_cartridge.stl
//   output/cell_suspension_prep_qc_module_bubble_dead_volume_block.stl
//   output/cell_suspension_prep_qc_module_sterile_connector_panel.stl
//   output/cell_suspension_prep_qc_module_prime_waste_tray.stl
//   output/cell_suspension_prep_qc_module_handoff_manifold.stl
//   output/cell_suspension_prep_qc_module_assembly.stl

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.6;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const RECIRC_BORE_D: f64 = 6.8;
const ROW_TRUNK_D: f64 = 6.0;
const STERILE_CONNECTOR_COUNT: usize = ROWS + 4; // five row feeds plus bag, recirc, QC, prime/waste.

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const MODULE_X: f64 = 760.0;
const MODULE_Y: f64 = 430.0;
const DECK_Z: f64 = 18.0;
const SERVICE_APPROACH_Y: f64 = 82.0;

const BAG_TRAY_X: f64 = 304.0;
const BAG_TRAY_Y: f64 = 190.0;
const BAG_TRAY_Z: f64 = 28.0;
const BAG_ENVELOPE_X: f64 = 250.0;
const BAG_ENVELOPE_Y: f64 = 150.0;
const BAG_NECK_D: f64 = 19.0;

const TEMP_ZONE_X: f64 = 330.0;
const TEMP_ZONE_Y: f64 = 210.0;
const TEMP_ZONE_Z: f64 = 32.0;
const PELTIER_SIZE: f64 = 40.4;

const MIXER_X: f64 = 250.0;
const MIXER_Y: f64 = 142.0;
const MIXER_Z: f64 = 42.0;

const QC_BLOCK_X: f64 = 282.0;
const QC_BLOCK_Y: f64 = 118.0;
const QC_BLOCK_Z: f64 = 40.0;

const BUBBLE_BLOCK_X: f64 = 242.0;
const BUBBLE_BLOCK_Y: f64 = 112.0;
const BUBBLE_BLOCK_Z: f64 = 36.0;

const CONNECTOR_PANEL_X: f64 = 548.0;
const CONNECTOR_PANEL_Y: f64 = 24.0;
const CONNECTOR_PANEL_Z: f64 = 132.0;
const CONNECTOR_PITCH: f64 = 54.0;

const WASTE_TRAY_X: f64 = 286.0;
const WASTE_TRAY_Y: f64 = 122.0;
const WASTE_TRAY_Z: f64 = 26.0;

const HANDOFF_X: f64 = 336.0;
const HANDOFF_Y: f64 = 132.0;
const HANDOFF_Z: f64 = 34.0;

const BAG_X: f64 = -210.0;
const BAG_Y: f64 = 70.0;
const MIXER_X_POS: f64 = -218.0;
const MIXER_Y_POS: f64 = -114.0;
const QC_X: f64 = 70.0;
const QC_Y: f64 = 70.0;
const BUBBLE_X: f64 = 252.0;
const BUBBLE_Y: f64 = 74.0;
const CONNECTOR_PANEL_X_POS: f64 = 0.0;
const CONNECTOR_PANEL_Y_POS: f64 = MODULE_Y / 2.0 - 22.0;
const WASTE_X: f64 = 56.0;
const WASTE_Y: f64 = -MODULE_Y / 2.0 + 76.0;
const HANDOFF_X_POS: f64 = MODULE_X / 2.0 - 186.0;
const HANDOFF_Y_POS: f64 = -82.0;

fn main() {
    let baseplate = baseplate();
    baseplate
        .write_stl("output/cell_suspension_prep_qc_module_baseplate.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_baseplate.stl");

    let bag_holder = sterile_bag_holder();
    bag_holder
        .write_stl("output/cell_suspension_prep_qc_module_bag_holder.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_bag_holder.stl");

    let temperature_hold = temperature_hold_zone();
    temperature_hold
        .write_stl("output/cell_suspension_prep_qc_module_temperature_hold_zone.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_temperature_hold_zone.stl");

    let mixer = mixing_recirculation_module();
    mixer
        .write_stl("output/cell_suspension_prep_qc_module_mixing_recirculation.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_mixing_recirculation.stl");

    let qc_loop = qc_loop_cartridge();
    qc_loop
        .write_stl("output/cell_suspension_prep_qc_module_qc_loop_cartridge.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_qc_loop_cartridge.stl");

    let bubble_control = bubble_dead_volume_block();
    bubble_control
        .write_stl("output/cell_suspension_prep_qc_module_bubble_dead_volume_block.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_bubble_dead_volume_block.stl");

    let connector_panel = sterile_connector_panel();
    connector_panel
        .write_stl("output/cell_suspension_prep_qc_module_sterile_connector_panel.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_sterile_connector_panel.stl");

    let waste_tray = prime_waste_tray();
    waste_tray
        .write_stl("output/cell_suspension_prep_qc_module_prime_waste_tray.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_prime_waste_tray.stl");

    let handoff = handoff_manifold();
    handoff
        .write_stl("output/cell_suspension_prep_qc_module_handoff_manifold.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_handoff_manifold.stl");

    let assembly = baseplate
        + temperature_hold.translate(BAG_X, BAG_Y, DECK_Z / 2.0 + TEMP_ZONE_Z / 2.0)
        + bag_holder.translate(BAG_X, BAG_Y, DECK_Z / 2.0 + TEMP_ZONE_Z + BAG_TRAY_Z / 2.0)
        + mixer.translate(MIXER_X_POS, MIXER_Y_POS, DECK_Z / 2.0 + MIXER_Z / 2.0)
        + qc_loop.translate(QC_X, QC_Y, DECK_Z / 2.0 + QC_BLOCK_Z / 2.0)
        + bubble_control.translate(BUBBLE_X, BUBBLE_Y, DECK_Z / 2.0 + BUBBLE_BLOCK_Z / 2.0)
        + connector_panel.translate(
            CONNECTOR_PANEL_X_POS,
            CONNECTOR_PANEL_Y_POS,
            DECK_Z / 2.0 + CONNECTOR_PANEL_Z / 2.0,
        )
        + waste_tray.translate(WASTE_X, WASTE_Y, DECK_Z / 2.0 + WASTE_TRAY_Z / 2.0)
        + handoff.translate(HANDOFF_X_POS, HANDOFF_Y_POS, DECK_Z / 2.0 + HANDOFF_Z / 2.0)
        + routed_tube_placeholders();

    assembly
        .write_stl("output/cell_suspension_prep_qc_module_assembly.stl")
        .unwrap();
    println!("Exported: output/cell_suspension_prep_qc_module_assembly.stl");

    println!(
        "Cell suspension prep/QC module: {:.0}mm x {:.0}mm service-skid deck, {:.0}mm x {:.0}mm sterile bag envelope, {:.0}mm x {:.0}mm temperature zone, {:.0}mm QC loop, {} sterile connector positions, {} cassette row handoff ports for a {}x{} Rev C cassette ({:.0}mm x {:.0}mm, {:.1}mm x {:.1}mm pitch), and {:.0}mm front service approach.",
        MODULE_X,
        MODULE_Y,
        BAG_ENVELOPE_X,
        BAG_ENVELOPE_Y,
        TEMP_ZONE_X,
        TEMP_ZONE_Y,
        QC_BLOCK_X,
        STERILE_CONNECTOR_COUNT,
        ROWS,
        COLS,
        ROWS,
        CASSETTE_X,
        CASSETTE_Y,
        PITCH_X,
        PITCH_Y,
        SERVICE_APPROACH_Y
    );
}

fn baseplate() -> Part {
    let deck = centered_cube("cell_prep_qc_module_deck", MODULE_X, MODULE_Y, DECK_Z);

    let bag_spill_sump = centered_cube(
        "cell_prep_qc_bag_zone_spill_sump",
        TEMP_ZONE_X + 24.0,
        TEMP_ZONE_Y + 20.0,
        8.0,
    )
    .translate(BAG_X, BAG_Y, DECK_Z / 2.0 - 3.5);
    let qc_drip_basin = centered_cube(
        "cell_prep_qc_sensor_loop_drip_basin",
        QC_BLOCK_X + BUBBLE_BLOCK_X - 32.0,
        48.0,
        8.0,
    )
    .translate((QC_X + BUBBLE_X) / 2.0, QC_Y - 74.0, DECK_Z / 2.0 - 3.5);
    let prime_gutter = centered_cube(
        "cell_prep_qc_prime_waste_gutter",
        WASTE_TRAY_X + 86.0,
        24.0,
        9.0,
    )
    .translate(WASTE_X + 38.0, WASTE_Y + 78.0, DECK_Z / 2.0 - 4.0);
    let service_approach_relief = centered_cube(
        "cell_prep_qc_front_service_approach_relief",
        MODULE_X - 96.0,
        SERVICE_APPROACH_Y,
        5.0,
    )
    .translate(
        0.0,
        -MODULE_Y / 2.0 + SERVICE_APPROACH_Y / 2.0,
        DECK_Z / 2.0 - 2.5,
    );
    let waste_drain = centered_cylinder("cell_prep_qc_deck_waste_drain", 8.0 / 2.0, 36.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            WASTE_X + WASTE_TRAY_X / 2.0 - 42.0,
            -MODULE_Y / 2.0 + 15.0,
            0.0,
        );

    deck - bag_spill_sump
        - qc_drip_basin
        - prime_gutter
        - service_approach_relief
        - waste_drain
        - deck_mount_slots()
        - deck_route_trenches()
        + deck_perimeter_rails()
        + skid_locator_bosses()
        + module_zone_markers()
        + tube_bridge_standoffs()
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("cell_prep_qc_deck_mount_slots");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("cell_prep_qc_deck_m6_clearance_{i}"),
            6.6 / 2.0,
            DECK_Z + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("cell_prep_qc_deck_m6_slot_relief_{i}"),
            26.0,
            6.8,
            DECK_Z + 2.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn deck_route_trenches() -> Part {
    let bag_to_mixer = centered_cube(
        "cell_prep_qc_deck_bag_to_mixer_tube_trench",
        34.0,
        142.0,
        8.0,
    )
    .translate(BAG_X + 106.0, -18.0, DECK_Z / 2.0 - 3.5);
    let mixer_to_qc = centered_cube(
        "cell_prep_qc_deck_mixer_to_qc_tube_trench",
        QC_X - MIXER_X_POS + 60.0,
        24.0,
        8.0,
    )
    .translate((MIXER_X_POS + QC_X) / 2.0 + 20.0, -34.0, DECK_Z / 2.0 - 3.5);
    let qc_to_handoff = centered_cube(
        "cell_prep_qc_deck_qc_to_handoff_tube_trench",
        HANDOFF_X_POS - QC_X + 86.0,
        24.0,
        8.0,
    )
    .translate(
        (QC_X + HANDOFF_X_POS) / 2.0 + 12.0,
        -24.0,
        DECK_Z / 2.0 - 3.5,
    );
    let waste_route = centered_cube("cell_prep_qc_deck_waste_route_trench", 38.0, 132.0, 8.0)
        .translate(
            WASTE_X - WASTE_TRAY_X / 2.0 + 54.0,
            WASTE_Y + 34.0,
            DECK_Z / 2.0 - 3.5,
        );

    bag_to_mixer + mixer_to_qc + qc_to_handoff + waste_route
}

fn deck_perimeter_rails() -> Part {
    let rear = centered_cube(
        "cell_prep_qc_rear_service_rail",
        MODULE_X - 60.0,
        18.0,
        28.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 28.0, DECK_Z / 2.0 + 14.0);
    let left = centered_cube("cell_prep_qc_left_module_rail", 18.0, MODULE_Y - 76.0, 28.0)
        .translate(-MODULE_X / 2.0 + 28.0, 0.0, DECK_Z / 2.0 + 14.0);
    let right = centered_cube(
        "cell_prep_qc_right_handoff_rail",
        18.0,
        MODULE_Y - 76.0,
        28.0,
    )
    .translate(MODULE_X / 2.0 - 28.0, 0.0, DECK_Z / 2.0 + 14.0);
    let front_left = centered_cube("cell_prep_qc_front_left_service_lip", 270.0, 12.0, 14.0)
        .translate(
            -MODULE_X / 2.0 + 185.0,
            -MODULE_Y / 2.0 + 22.0,
            DECK_Z / 2.0 + 7.0,
        );
    let front_right = centered_cube("cell_prep_qc_front_right_service_lip", 270.0, 12.0, 14.0)
        .translate(
            MODULE_X / 2.0 - 185.0,
            -MODULE_Y / 2.0 + 22.0,
            DECK_Z / 2.0 + 7.0,
        );

    rear + left + right + front_left + front_right
}

fn skid_locator_bosses() -> Part {
    let mut bosses = Part::empty("cell_prep_qc_skid_locator_bosses");
    for (i, (x, y)) in skid_locator_points().iter().enumerate() {
        let boss = centered_cylinder(format!("cell_prep_qc_skid_locator_boss_{i}"), 11.0, 8.0, 36)
            .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        let socket = centered_cylinder(
            format!("cell_prep_qc_skid_locator_socket_{i}"),
            4.0 / 2.0,
            10.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        bosses = bosses + (boss - socket);
    }
    bosses
}

fn module_zone_markers() -> Part {
    let bag_zone = centered_cube("cell_prep_qc_bag_zone_locator_land", 86.0, 6.0, 5.0).translate(
        BAG_X,
        BAG_Y - TEMP_ZONE_Y / 2.0 - 8.0,
        DECK_Z / 2.0 + 2.5,
    );
    let qc_zone = centered_cube("cell_prep_qc_sensor_zone_locator_land", 96.0, 6.0, 5.0).translate(
        QC_X,
        QC_Y - QC_BLOCK_Y / 2.0 - 10.0,
        DECK_Z / 2.0 + 2.5,
    );
    let handoff_zone = centered_cube("cell_prep_qc_handoff_zone_locator_land", 110.0, 6.0, 5.0)
        .translate(
            HANDOFF_X_POS,
            HANDOFF_Y_POS - HANDOFF_Y / 2.0 - 10.0,
            DECK_Z / 2.0 + 2.5,
        );
    let cassette_reference = centered_cube(
        "cell_prep_qc_cassette_row_pitch_reference",
        6.0,
        CASSETTE_Y.min(MODULE_Y - 118.0),
        5.0,
    )
    .translate(MODULE_X / 2.0 - 64.0, 4.0, DECK_Z / 2.0 + 2.5);

    bag_zone + qc_zone + handoff_zone + cassette_reference
}

fn tube_bridge_standoffs() -> Part {
    let mut standoffs = Part::empty("cell_prep_qc_tube_bridge_standoffs");
    for (i, (x, y)) in [
        (BAG_X + 132.0, -6.0),
        (MIXER_X_POS + 84.0, -34.0),
        (QC_X + 118.0, -24.0),
        (HANDOFF_X_POS - 118.0, -24.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cylinder(format!("cell_prep_qc_tube_bridge_post_{i}"), 9.0, 14.0, 28)
            .translate(*x, *y, DECK_Z / 2.0 + 7.0);
        let tie_slot = centered_cube(
            format!("cell_prep_qc_tube_bridge_tie_slot_{i}"),
            12.0,
            4.0,
            16.0,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 7.0);
        standoffs = standoffs + (post - tie_slot);
    }
    standoffs
}

fn sterile_bag_holder() -> Part {
    let tray = centered_cube(
        "cell_prep_qc_sterile_bag_holder_tray",
        BAG_TRAY_X,
        BAG_TRAY_Y,
        BAG_TRAY_Z,
    );
    let bag_recess = centered_cube(
        "cell_prep_qc_sterile_bag_envelope_recess",
        BAG_ENVELOPE_X,
        BAG_ENVELOPE_Y,
        BAG_TRAY_Z + 2.0,
    )
    .translate(-14.0, 0.0, 4.8);
    let soft_wall_relief = centered_cube(
        "cell_prep_qc_bag_soft_wall_relief",
        BAG_ENVELOPE_X - 36.0,
        BAG_ENVELOPE_Y - 34.0,
        8.0,
    )
    .translate(-14.0, 0.0, BAG_TRAY_Z / 2.0 - 3.5);
    let neck_channel =
        centered_cylinder("cell_prep_qc_bag_neck_channel", BAG_NECK_D / 2.0, 86.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(BAG_TRAY_X / 2.0 - 32.0, 0.0, 4.0);
    let drain_slot = centered_cube(
        "cell_prep_qc_bag_tray_low_point_drain_slot",
        72.0,
        14.0,
        8.0,
    )
    .translate(
        BAG_TRAY_X / 2.0 - 70.0,
        -BAG_TRAY_Y / 2.0 + 24.0,
        BAG_TRAY_Z / 2.0 - 4.0,
    );

    tray - bag_recess - soft_wall_relief - neck_channel - drain_slot - bag_tray_mount_holes()
        + bag_retaining_rails()
        + bag_hanger_frame()
        + sterile_bag_neck_clamp()
        + bag_pickup_lance_manifold()
        + bag_load_cell_pads()
}

fn bag_retaining_rails() -> Part {
    let left = centered_cube(
        "cell_prep_qc_bag_left_retaining_rail",
        12.0,
        BAG_TRAY_Y - 32.0,
        24.0,
    )
    .translate(-(BAG_TRAY_X / 2.0 - 20.0), 0.0, BAG_TRAY_Z / 2.0 + 12.0);
    let right = centered_cube(
        "cell_prep_qc_bag_right_retaining_rail",
        12.0,
        BAG_TRAY_Y - 32.0,
        24.0,
    )
    .translate(BAG_TRAY_X / 2.0 - 20.0, 0.0, BAG_TRAY_Z / 2.0 + 12.0);
    let rear = centered_cube(
        "cell_prep_qc_bag_rear_retaining_rail",
        BAG_TRAY_X - 46.0,
        12.0,
        24.0,
    )
    .translate(0.0, BAG_TRAY_Y / 2.0 - 20.0, BAG_TRAY_Z / 2.0 + 12.0);
    let front_low = centered_cube(
        "cell_prep_qc_bag_front_low_loading_lip",
        BAG_TRAY_X - 84.0,
        10.0,
        14.0,
    )
    .translate(-22.0, -(BAG_TRAY_Y / 2.0 - 18.0), BAG_TRAY_Z / 2.0 + 7.0);

    left + right + rear + front_low
}

fn bag_hanger_frame() -> Part {
    let left_upright = centered_cube("cell_prep_qc_bag_hanger_left_upright", 14.0, 16.0, 138.0)
        .translate(
            -BAG_TRAY_X / 2.0 + 42.0,
            BAG_TRAY_Y / 2.0 + 10.0,
            BAG_TRAY_Z / 2.0 + 69.0,
        );
    let right_upright = centered_cube("cell_prep_qc_bag_hanger_right_upright", 14.0, 16.0, 138.0)
        .translate(
            BAG_TRAY_X / 2.0 - 42.0,
            BAG_TRAY_Y / 2.0 + 10.0,
            BAG_TRAY_Z / 2.0 + 69.0,
        );
    let crossbar = centered_cube(
        "cell_prep_qc_bag_hanger_crossbar",
        BAG_TRAY_X - 72.0,
        14.0,
        16.0,
    )
    .translate(0.0, BAG_TRAY_Y / 2.0 + 10.0, BAG_TRAY_Z / 2.0 + 132.0);

    let mut hanger_slots = Part::empty("cell_prep_qc_bag_hanger_slots");
    for (i, x) in [-74.0, 0.0, 74.0].iter().enumerate() {
        let loop_slot = centered_cube(
            format!("cell_prep_qc_bag_hanger_loop_slot_{i}"),
            28.0,
            18.0,
            8.0,
        )
        .translate(*x, BAG_TRAY_Y / 2.0 + 10.0, BAG_TRAY_Z / 2.0 + 132.0);
        hanger_slots = hanger_slots + loop_slot;
    }

    left_upright + right_upright + (crossbar - hanger_slots)
}

fn sterile_bag_neck_clamp() -> Part {
    let body = centered_cube(
        "cell_prep_qc_bag_neck_sterile_connector_clamp",
        78.0,
        42.0,
        30.0,
    )
    .translate(BAG_TRAY_X / 2.0 - 32.0, 0.0, BAG_TRAY_Z / 2.0 + 15.0);
    let tube_bore = centered_cylinder(
        "cell_prep_qc_bag_neck_connector_bore",
        BAG_NECK_D / 2.0,
        86.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(BAG_TRAY_X / 2.0 - 32.0, 0.0, BAG_TRAY_Z / 2.0 + 15.0);
    let clamp_screw_top =
        centered_cylinder("cell_prep_qc_bag_neck_top_clamp_screw", 3.4 / 2.0, 42.0, 20).translate(
            BAG_TRAY_X / 2.0 - 32.0,
            13.0,
            BAG_TRAY_Z / 2.0 + 15.0,
        );
    let clamp_screw_bottom = centered_cylinder(
        "cell_prep_qc_bag_neck_bottom_clamp_screw",
        3.4 / 2.0,
        42.0,
        20,
    )
    .translate(BAG_TRAY_X / 2.0 - 32.0, -13.0, BAG_TRAY_Z / 2.0 + 15.0);

    body - tube_bore - clamp_screw_top - clamp_screw_bottom
}

fn bag_pickup_lance_manifold() -> Part {
    let pickup_bar = centered_cube("cell_prep_qc_bag_pickup_lance_bar", 142.0, 18.0, 20.0)
        .translate(
            BAG_TRAY_X / 2.0 - 102.0,
            -BAG_TRAY_Y / 2.0 + 42.0,
            BAG_TRAY_Z / 2.0 + 10.0,
        );
    let mut cuts = Part::empty("cell_prep_qc_bag_pickup_lance_cuts");
    for (i, x) in [-50.0, 0.0, 50.0].iter().enumerate() {
        let tube = centered_cylinder(
            format!("cell_prep_qc_bag_lance_channel_{i}"),
            FLUID_BORE_D / 2.0,
            24.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            BAG_TRAY_X / 2.0 - 102.0 + *x,
            -BAG_TRAY_Y / 2.0 + 42.0,
            BAG_TRAY_Z / 2.0 + 10.0,
        );
        let top_slot = centered_cube(
            format!("cell_prep_qc_bag_lance_top_slot_{i}"),
            FLUID_BORE_D + 1.0,
            26.0,
            10.0,
        )
        .translate(
            BAG_TRAY_X / 2.0 - 102.0 + *x,
            -BAG_TRAY_Y / 2.0 + 42.0,
            BAG_TRAY_Z / 2.0 + 14.0,
        );
        cuts = cuts + tube + top_slot;
    }
    pickup_bar - cuts
}

fn bag_load_cell_pads() -> Part {
    let mut pads = Part::empty("cell_prep_qc_bag_load_cell_pads");
    for (i, (x, y)) in [(-96.0, -58.0), (72.0, -58.0), (-96.0, 58.0), (72.0, 58.0)]
        .iter()
        .enumerate()
    {
        let pad = centered_cube(
            format!("cell_prep_qc_bag_load_cell_pad_{i}"),
            38.0,
            28.0,
            6.0,
        )
        .translate(*x, *y, BAG_TRAY_Z / 2.0 + 3.0);
        let relief = centered_cylinder(
            format!("cell_prep_qc_bag_load_cell_screw_relief_{i}"),
            3.4 / 2.0,
            8.0,
            20,
        )
        .translate(*x, *y, BAG_TRAY_Z / 2.0 + 3.0);
        pads = pads + (pad - relief);
    }
    pads
}

fn bag_tray_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_bag_tray_mount_holes");
    for (i, (x, y)) in [
        (-(BAG_TRAY_X / 2.0 - 18.0), -(BAG_TRAY_Y / 2.0 - 18.0)),
        (BAG_TRAY_X / 2.0 - 18.0, -(BAG_TRAY_Y / 2.0 - 18.0)),
        (-(BAG_TRAY_X / 2.0 - 18.0), BAG_TRAY_Y / 2.0 - 18.0),
        (BAG_TRAY_X / 2.0 - 18.0, BAG_TRAY_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_bag_tray_m4_mount_{i}"),
                4.3 / 2.0,
                BAG_TRAY_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn temperature_hold_zone() -> Part {
    let block = centered_cube(
        "cell_prep_qc_temperature_hold_block",
        TEMP_ZONE_X,
        TEMP_ZONE_Y,
        TEMP_ZONE_Z,
    );
    let bag_contact_pocket = centered_cube(
        "cell_prep_qc_temperature_bag_contact_pocket",
        BAG_ENVELOPE_X + 18.0,
        BAG_ENVELOPE_Y + 20.0,
        12.0,
    )
    .translate(-10.0, 0.0, TEMP_ZONE_Z / 2.0 - 5.5);
    let insulation_moat = centered_cube(
        "cell_prep_qc_temperature_insulation_moat",
        TEMP_ZONE_X - 32.0,
        14.0,
        9.0,
    )
    .translate(0.0, -TEMP_ZONE_Y / 2.0 + 24.0, TEMP_ZONE_Z / 2.0 - 4.0);
    let condensate_drain = centered_cylinder(
        "cell_prep_qc_temperature_condensate_drain",
        5.5 / 2.0,
        42.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TEMP_ZONE_X / 2.0 - 42.0, -TEMP_ZONE_Y / 2.0 + 10.0, 0.0);

    block
        - bag_contact_pocket
        - insulation_moat
        - condensate_drain
        - peltier_recesses()
        - temperature_sensor_pockets()
        - temperature_hold_mount_holes()
        + insulation_lips()
        + thermal_spreader_ribs()
}

fn peltier_recesses() -> Part {
    let mut recesses = Part::empty("cell_prep_qc_temperature_peltier_recesses");
    for (i, x) in [-82.0, 0.0, 82.0].iter().enumerate() {
        let recess = centered_cube(
            format!("cell_prep_qc_temperature_peltier_recess_{i}"),
            PELTIER_SIZE,
            PELTIER_SIZE,
            7.0,
        )
        .translate(*x, 0.0, -TEMP_ZONE_Z / 2.0 + 2.5);
        let cable_exit = centered_cube(
            format!("cell_prep_qc_temperature_peltier_cable_exit_{i}"),
            12.0,
            TEMP_ZONE_Y / 2.0,
            7.0,
        )
        .translate(*x, -TEMP_ZONE_Y / 4.0, -TEMP_ZONE_Z / 2.0 + 2.5);
        recesses = recesses + recess + cable_exit;
    }
    recesses
}

fn temperature_sensor_pockets() -> Part {
    let therm_front = centered_cylinder(
        "cell_prep_qc_temperature_front_thermistor_pocket",
        3.5 / 2.0,
        46.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-70.0, -TEMP_ZONE_Y / 2.0 + 22.0, 5.0);
    let therm_rear = centered_cylinder(
        "cell_prep_qc_temperature_rear_thermistor_pocket",
        3.5 / 2.0,
        46.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(70.0, TEMP_ZONE_Y / 2.0 - 22.0, 5.0);
    let rtd_slot = centered_cube("cell_prep_qc_temperature_rtd_flex_slot", 210.0, 7.0, 6.0)
        .translate(-10.0, TEMP_ZONE_Y / 2.0 - 46.0, TEMP_ZONE_Z / 2.0 - 3.0);

    therm_front + therm_rear + rtd_slot
}

fn insulation_lips() -> Part {
    let rear = centered_cube(
        "cell_prep_qc_temperature_rear_insulation_lip",
        TEMP_ZONE_X - 34.0,
        12.0,
        18.0,
    )
    .translate(0.0, TEMP_ZONE_Y / 2.0 - 16.0, TEMP_ZONE_Z / 2.0 + 9.0);
    let left = centered_cube(
        "cell_prep_qc_temperature_left_insulation_lip",
        12.0,
        TEMP_ZONE_Y - 42.0,
        18.0,
    )
    .translate(-TEMP_ZONE_X / 2.0 + 16.0, 0.0, TEMP_ZONE_Z / 2.0 + 9.0);
    let right = centered_cube(
        "cell_prep_qc_temperature_right_insulation_lip",
        12.0,
        TEMP_ZONE_Y - 42.0,
        18.0,
    )
    .translate(TEMP_ZONE_X / 2.0 - 16.0, 0.0, TEMP_ZONE_Z / 2.0 + 9.0);

    rear + left + right
}

fn thermal_spreader_ribs() -> Part {
    let mut ribs = Part::empty("cell_prep_qc_temperature_spreader_ribs");
    for (i, y) in [-54.0, 0.0, 54.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("cell_prep_qc_temperature_spreader_rib_{i}"),
                TEMP_ZONE_X - 76.0,
                7.0,
                6.0,
            )
            .translate(-8.0, *y, TEMP_ZONE_Z / 2.0 + 3.0);
    }
    ribs
}

fn temperature_hold_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_temperature_hold_mount_holes");
    for (i, (x, y)) in [
        (-(TEMP_ZONE_X / 2.0 - 20.0), -(TEMP_ZONE_Y / 2.0 - 18.0)),
        (TEMP_ZONE_X / 2.0 - 20.0, -(TEMP_ZONE_Y / 2.0 - 18.0)),
        (-(TEMP_ZONE_X / 2.0 - 20.0), TEMP_ZONE_Y / 2.0 - 18.0),
        (TEMP_ZONE_X / 2.0 - 20.0, TEMP_ZONE_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_temperature_m5_mount_{i}"),
                5.3 / 2.0,
                TEMP_ZONE_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn mixing_recirculation_module() -> Part {
    let body = centered_cube(
        "cell_prep_qc_mixing_recirculation_body",
        MIXER_X,
        MIXER_Y,
        MIXER_Z,
    );
    let rocker_clearance = centered_cube(
        "cell_prep_qc_rocking_platform_clearance",
        MIXER_X - 64.0,
        MIXER_Y - 42.0,
        18.0,
    )
    .translate(-8.0, 0.0, MIXER_Z / 2.0 - 8.0);
    let pivot_bore = centered_cylinder(
        "cell_prep_qc_rocker_pivot_shaft_bore",
        8.0 / 2.0,
        MIXER_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 2.0);
    let pump_tube_path = centered_cylinder(
        "cell_prep_qc_recirc_pump_tube_path",
        RECIRC_BORE_D / 2.0,
        MIXER_X - 36.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -MIXER_Y / 2.0 + 28.0, -4.0);
    let pump_top_slot = centered_cube(
        "cell_prep_qc_recirc_pump_tube_top_slot",
        MIXER_X - 38.0,
        RECIRC_BORE_D + 1.2,
        16.0,
    )
    .translate(0.0, -MIXER_Y / 2.0 + 28.0, 8.0);
    let motor_pocket = centered_cube("cell_prep_qc_recirc_pump_motor_keepout", 52.0, 36.0, 28.0)
        .translate(
            -MIXER_X / 2.0 + 42.0,
            -MIXER_Y / 2.0 + 28.0,
            MIXER_Z / 2.0 - 12.0,
        );

    body - rocker_clearance
        - pivot_bore
        - pump_tube_path
        - pump_top_slot
        - motor_pocket
        - mixer_mount_holes()
        + rocking_platform_placeholder()
        + recirc_rotor_placeholders()
        + recirc_connector_tabs()
        + rocker_limit_stops()
}

fn rocking_platform_placeholder() -> Part {
    let platform = centered_cube(
        "cell_prep_qc_rocking_bag_platform_placeholder",
        MIXER_X - 78.0,
        MIXER_Y - 58.0,
        10.0,
    )
    .rotate(4.0, 0.0, 0.0)
    .translate(-8.0, 0.0, MIXER_Z / 2.0 + 7.0);
    let left_trunnion = centered_cylinder("cell_prep_qc_rocker_left_trunnion", 10.0, 18.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(-(MIXER_X / 2.0 - 40.0), 0.0, MIXER_Z / 2.0 + 4.0);
    let right_trunnion = centered_cylinder("cell_prep_qc_rocker_right_trunnion", 10.0, 18.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(MIXER_X / 2.0 - 40.0, 0.0, MIXER_Z / 2.0 + 4.0);
    let flexure_bridge = centered_cube(
        "cell_prep_qc_rocker_soft_bag_contact_bridge",
        MIXER_X - 122.0,
        16.0,
        7.0,
    )
    .translate(-8.0, 0.0, MIXER_Z / 2.0 + 14.0);

    platform + left_trunnion + right_trunnion + flexure_bridge
}

fn recirc_rotor_placeholders() -> Part {
    let rotor = centered_cylinder("cell_prep_qc_recirc_peristaltic_rotor", 15.0, 8.0, 44)
        .translate(
            -MIXER_X / 2.0 + 84.0,
            -MIXER_Y / 2.0 + 28.0,
            MIXER_Z / 2.0 + 4.0,
        );
    let hub = centered_cylinder("cell_prep_qc_recirc_encoder_hub", 5.0, 9.0, 32).translate(
        -MIXER_X / 2.0 + 84.0,
        -MIXER_Y / 2.0 + 28.0,
        MIXER_Z / 2.0 + 5.0,
    );
    let pinch_anvil = centered_cube("cell_prep_qc_recirc_pump_pinch_anvil", 66.0, 12.0, 18.0)
        .translate(
            -MIXER_X / 2.0 + 84.0,
            -MIXER_Y / 2.0 + 49.0,
            MIXER_Z / 2.0 + 9.0,
        );

    rotor + hub + pinch_anvil
}

fn recirc_connector_tabs() -> Part {
    let inlet = fluid_connector_tab("recirc_inlet").translate(
        -MIXER_X / 2.0 - 18.0,
        -MIXER_Y / 2.0 + 28.0,
        -4.0,
    );
    let outlet = fluid_connector_tab("recirc_outlet").translate(
        MIXER_X / 2.0 + 18.0,
        -MIXER_Y / 2.0 + 28.0,
        -4.0,
    );
    inlet + outlet
}

fn rocker_limit_stops() -> Part {
    let left_stop = centered_cube("cell_prep_qc_rocker_left_limit_stop", 18.0, 22.0, 22.0)
        .translate(
            -MIXER_X / 2.0 + 34.0,
            MIXER_Y / 2.0 - 32.0,
            MIXER_Z / 2.0 + 11.0,
        );
    let right_stop = centered_cube("cell_prep_qc_rocker_right_limit_stop", 18.0, 22.0, 22.0)
        .translate(
            MIXER_X / 2.0 - 34.0,
            MIXER_Y / 2.0 - 32.0,
            MIXER_Z / 2.0 + 11.0,
        );
    left_stop + right_stop
}

fn mixer_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_mixer_mount_holes");
    for (i, (x, y)) in [
        (-(MIXER_X / 2.0 - 18.0), -(MIXER_Y / 2.0 - 16.0)),
        (MIXER_X / 2.0 - 18.0, -(MIXER_Y / 2.0 - 16.0)),
        (-(MIXER_X / 2.0 - 18.0), MIXER_Y / 2.0 - 16.0),
        (MIXER_X / 2.0 - 18.0, MIXER_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_mixer_m4_mount_{i}"),
                4.3 / 2.0,
                MIXER_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn qc_loop_cartridge() -> Part {
    let body = centered_cube(
        "cell_prep_qc_loop_cartridge_body",
        QC_BLOCK_X,
        QC_BLOCK_Y,
        QC_BLOCK_Z,
    );
    let main_channel = centered_cylinder(
        "cell_prep_qc_loop_main_sample_channel",
        FLUID_BORE_D / 2.0,
        QC_BLOCK_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 16.0, 0.0);
    let bypass_channel = centered_cylinder(
        "cell_prep_qc_loop_bypass_channel",
        FLUID_BORE_D / 2.0,
        QC_BLOCK_X - 58.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(8.0, -24.0, 0.0);
    let bypass_bridge_in = centered_cylinder(
        "cell_prep_qc_loop_bypass_bridge_in",
        FLUID_BORE_D / 2.0,
        48.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-94.0, -4.0, 0.0);
    let bypass_bridge_out = centered_cylinder(
        "cell_prep_qc_loop_bypass_bridge_out",
        FLUID_BORE_D / 2.0,
        48.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(112.0, -4.0, 0.0);
    let cuvette_pocket = centered_cube("cell_prep_qc_imaging_cuvette_pocket", 94.0, 38.0, 24.0)
        .translate(-52.0, 16.0, QC_BLOCK_Z / 2.0 - 10.0);
    let imaging_window = centered_cube(
        "cell_prep_qc_imaging_cuvette_clear_window",
        78.0,
        QC_BLOCK_Y + 2.0,
        18.0,
    )
    .translate(-52.0, 16.0, QC_BLOCK_Z / 2.0 - 3.0);
    let viability_sensor_pocket = centered_cube(
        "cell_prep_qc_viability_sensor_puck_pocket",
        62.0,
        42.0,
        18.0,
    )
    .translate(62.0, 16.0, QC_BLOCK_Z / 2.0 - 8.0);
    let sample_takeoff = centered_cylinder(
        "cell_prep_qc_sample_takeoff_port",
        3.2 / 2.0,
        QC_BLOCK_Z + 10.0,
        20,
    )
    .translate(-118.0, 16.0, 0.0);
    let reagent_spike = centered_cylinder(
        "cell_prep_qc_viability_reagent_spike_port",
        3.2 / 2.0,
        QC_BLOCK_Z + 10.0,
        20,
    )
    .translate(16.0, 16.0, 0.0);

    body + qc_optical_fork() + qc_latch_ears() + qc_cover_lands() + qc_valve_cap_bosses()
        - main_channel
        - bypass_channel
        - bypass_bridge_in
        - bypass_bridge_out
        - cuvette_pocket
        - imaging_window
        - viability_sensor_pocket
        - sample_takeoff
        - reagent_spike
        - qc_valve_seat_cuts()
        - qc_mount_holes()
}

fn qc_optical_fork() -> Part {
    let base = centered_cube("cell_prep_qc_cuvette_optical_fork_base", 72.0, 48.0, 10.0).translate(
        -52.0,
        16.0,
        QC_BLOCK_Z / 2.0 + 5.0,
    );
    let led_arm = centered_cube("cell_prep_qc_cuvette_led_arm", 16.0, 12.0, 48.0).translate(
        -52.0,
        -9.0,
        QC_BLOCK_Z / 2.0 + 24.0,
    );
    let camera_arm = centered_cube("cell_prep_qc_cuvette_camera_arm", 16.0, 12.0, 48.0).translate(
        -52.0,
        41.0,
        QC_BLOCK_Z / 2.0 + 24.0,
    );
    let cuvette_gap = centered_cube("cell_prep_qc_cuvette_optical_gap", 32.0, 62.0, 32.0)
        .translate(-52.0, 16.0, QC_BLOCK_Z / 2.0 + 23.0);
    let optical_axis = centered_cylinder(
        "cell_prep_qc_cuvette_optical_axis_clearance",
        4.0 / 2.0,
        58.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-52.0, 16.0, QC_BLOCK_Z / 2.0 + 24.0);

    base + led_arm + camera_arm - cuvette_gap - optical_axis
}

fn qc_latch_ears() -> Part {
    let left = latch_ear("cell_prep_qc_loop_cartridge_left").translate(
        -QC_BLOCK_X / 2.0 + 32.0,
        -QC_BLOCK_Y / 2.0 - 14.0,
        0.0,
    );
    let right = latch_ear("cell_prep_qc_loop_cartridge_right").translate(
        QC_BLOCK_X / 2.0 - 32.0,
        -QC_BLOCK_Y / 2.0 - 14.0,
        0.0,
    );
    left + right
}

fn qc_cover_lands() -> Part {
    let rear = centered_cube(
        "cell_prep_qc_loop_cover_rear_land",
        QC_BLOCK_X - 32.0,
        8.0,
        6.0,
    )
    .translate(0.0, QC_BLOCK_Y / 2.0 - 12.0, QC_BLOCK_Z / 2.0 + 3.0);
    let front = centered_cube(
        "cell_prep_qc_loop_cover_front_land",
        QC_BLOCK_X - 32.0,
        8.0,
        6.0,
    )
    .translate(0.0, -QC_BLOCK_Y / 2.0 + 12.0, QC_BLOCK_Z / 2.0 + 3.0);
    rear + front
}

fn qc_valve_cap_bosses() -> Part {
    let mut bosses = Part::empty("cell_prep_qc_loop_valve_cap_bosses");
    for (i, (x, y)) in [(-94.0, 16.0), (-12.0, -24.0), (112.0, 16.0)]
        .iter()
        .enumerate()
    {
        bosses = bosses
            + centered_cylinder(
                format!("cell_prep_qc_loop_valve_cap_boss_{i}"),
                12.0,
                8.0,
                32,
            )
            .translate(*x, *y, QC_BLOCK_Z / 2.0 + 4.0);
    }
    bosses
}

fn qc_valve_seat_cuts() -> Part {
    let mut cuts = Part::empty("cell_prep_qc_loop_valve_seat_cuts");
    for (i, (x, y)) in [(-94.0, 16.0), (-12.0, -24.0), (112.0, 16.0)]
        .iter()
        .enumerate()
    {
        let stem = centered_cylinder(
            format!("cell_prep_qc_loop_valve_stem_bore_{i}"),
            4.0 / 2.0,
            QC_BLOCK_Z + 10.0,
            24,
        )
        .translate(*x, *y, 4.0);
        let actuator_pocket = centered_cube(
            format!("cell_prep_qc_loop_valve_actuator_pocket_{i}"),
            24.0,
            24.0,
            12.0,
        )
        .translate(*x, *y, QC_BLOCK_Z / 2.0 - 5.0);
        cuts = cuts + stem + actuator_pocket;
    }
    cuts
}

fn qc_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_loop_mount_holes");
    for (i, (x, y)) in [
        (-(QC_BLOCK_X / 2.0 - 18.0), -(QC_BLOCK_Y / 2.0 - 16.0)),
        (QC_BLOCK_X / 2.0 - 18.0, -(QC_BLOCK_Y / 2.0 - 16.0)),
        (-(QC_BLOCK_X / 2.0 - 18.0), QC_BLOCK_Y / 2.0 - 16.0),
        (QC_BLOCK_X / 2.0 - 18.0, QC_BLOCK_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_loop_m4_mount_{i}"),
                4.3 / 2.0,
                QC_BLOCK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn bubble_dead_volume_block() -> Part {
    let body = centered_cube(
        "cell_prep_qc_bubble_dead_volume_body",
        BUBBLE_BLOCK_X,
        BUBBLE_BLOCK_Y,
        BUBBLE_BLOCK_Z,
    );
    let main_channel = centered_cylinder(
        "cell_prep_qc_bubble_control_main_channel",
        FLUID_BORE_D / 2.0,
        BUBBLE_BLOCK_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -20.0, 0.0);
    let low_dead_volume_slot = centered_cube(
        "cell_prep_qc_low_dead_volume_swept_slot",
        BUBBLE_BLOCK_X - 42.0,
        FLUID_BORE_D + 1.4,
        12.0,
    )
    .translate(0.0, -20.0, 8.0);
    let drain_channel = centered_cylinder(
        "cell_prep_qc_low_point_prime_drain_channel",
        FLUID_BORE_D / 2.0,
        BUBBLE_BLOCK_Y - 28.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BUBBLE_BLOCK_X / 2.0 - 42.0, -2.0, 0.0);
    let drain_cup = centered_cube("cell_prep_qc_low_point_prime_drain_cup", 44.0, 24.0, 12.0)
        .translate(
            BUBBLE_BLOCK_X / 2.0 - 42.0,
            -BUBBLE_BLOCK_Y / 2.0 + 22.0,
            -BUBBLE_BLOCK_Z / 2.0 + 5.0,
        );

    body + bubble_trap_towers() + vent_filter_placeholders() + dead_volume_sweep_ribs()
        - main_channel
        - low_dead_volume_slot
        - drain_channel
        - drain_cup
        - bubble_block_mount_holes()
}

fn bubble_trap_towers() -> Part {
    let mut towers = Part::empty("cell_prep_qc_bubble_trap_towers");
    for (i, x) in [-72.0, 0.0, 72.0].iter().enumerate() {
        let chamber = centered_cylinder(
            format!("cell_prep_qc_bubble_trap_chamber_{i}"),
            16.0,
            64.0,
            48,
        )
        .translate(*x, 22.0, BUBBLE_BLOCK_Z / 2.0 + 32.0);
        let vent_core = centered_cylinder(
            format!("cell_prep_qc_bubble_trap_vent_core_{i}"),
            5.2,
            70.0,
            28,
        )
        .translate(*x, 22.0, BUBBLE_BLOCK_Z / 2.0 + 32.0);
        let optical_flat = centered_cube(
            format!("cell_prep_qc_bubble_trap_optical_flat_{i}"),
            13.0,
            36.0,
            42.0,
        )
        .translate(*x + 12.0, 22.0, BUBBLE_BLOCK_Z / 2.0 + 34.0);
        towers = towers + (chamber - vent_core - optical_flat);
    }
    towers
}

fn vent_filter_placeholders() -> Part {
    let mut vents = Part::empty("cell_prep_qc_bubble_trap_vent_filters");
    for (i, x) in [-72.0, 0.0, 72.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("cell_prep_qc_bubble_trap_vent_filter_boss_{i}"),
            12.0,
            8.0,
            36,
        )
        .translate(*x, 22.0, BUBBLE_BLOCK_Z + 68.0);
        let bore = centered_cylinder(
            format!("cell_prep_qc_bubble_trap_vent_filter_bore_{i}"),
            5.2 / 2.0,
            10.0,
            24,
        )
        .translate(*x, 22.0, BUBBLE_BLOCK_Z + 68.0);
        vents = vents + (boss - bore);
    }
    vents
}

fn dead_volume_sweep_ribs() -> Part {
    let rib_a = centered_cube(
        "cell_prep_qc_dead_volume_sweep_rib_a",
        BUBBLE_BLOCK_X - 58.0,
        6.0,
        6.0,
    )
    .translate(0.0, -38.0, BUBBLE_BLOCK_Z / 2.0 + 3.0);
    let rib_b = centered_cube(
        "cell_prep_qc_dead_volume_sweep_rib_b",
        BUBBLE_BLOCK_X - 58.0,
        6.0,
        6.0,
    )
    .translate(0.0, -2.0, BUBBLE_BLOCK_Z / 2.0 + 3.0);
    rib_a + rib_b
}

fn bubble_block_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_bubble_block_mount_holes");
    for (i, (x, y)) in [
        (
            -(BUBBLE_BLOCK_X / 2.0 - 18.0),
            -(BUBBLE_BLOCK_Y / 2.0 - 16.0),
        ),
        (BUBBLE_BLOCK_X / 2.0 - 18.0, -(BUBBLE_BLOCK_Y / 2.0 - 16.0)),
        (-(BUBBLE_BLOCK_X / 2.0 - 18.0), BUBBLE_BLOCK_Y / 2.0 - 16.0),
        (BUBBLE_BLOCK_X / 2.0 - 18.0, BUBBLE_BLOCK_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_bubble_block_m4_mount_{i}"),
                4.3 / 2.0,
                BUBBLE_BLOCK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn sterile_connector_panel() -> Part {
    let panel = centered_cube(
        "cell_prep_qc_sterile_connector_panel_body",
        CONNECTOR_PANEL_X,
        CONNECTOR_PANEL_Y,
        CONNECTOR_PANEL_Z,
    );
    let top_land = centered_cube(
        "cell_prep_qc_connector_panel_top_label_land",
        CONNECTOR_PANEL_X - 56.0,
        6.0,
        8.0,
    )
    .translate(
        0.0,
        -CONNECTOR_PANEL_Y / 2.0 - 3.0,
        CONNECTOR_PANEL_Z / 2.0 - 10.0,
    );
    let lower_cable_slot = centered_cube(
        "cell_prep_qc_connector_panel_cable_pass_slot",
        CONNECTOR_PANEL_X - 120.0,
        CONNECTOR_PANEL_Y + 2.0,
        16.0,
    )
    .translate(0.0, 0.0, -CONNECTOR_PANEL_Z / 2.0 + 24.0);

    panel + top_land + connector_collars() + sterile_cap_parking_posts() + panel_mount_flanges()
        - connector_panel_bores()
        - lower_cable_slot
        - panel_mount_holes()
}

fn connector_collars() -> Part {
    let mut collars = Part::empty("cell_prep_qc_connector_panel_collars");
    for i in 0..STERILE_CONNECTOR_COUNT {
        let x = connector_x(i);
        let y = -CONNECTOR_PANEL_Y / 2.0 - 5.0;
        let z = connector_z(i);
        let outer = centered_cylinder(
            format!("cell_prep_qc_sterile_connector_collar_{i}"),
            connector_outer_radius(i),
            10.0,
            44,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z);
        let inner = centered_cylinder(
            format!("cell_prep_qc_sterile_connector_collar_bore_{i}"),
            connector_bore_radius(i),
            12.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z);
        collars = collars + (outer - inner);
    }
    collars
}

fn connector_panel_bores() -> Part {
    let mut bores = Part::empty("cell_prep_qc_connector_panel_bores");
    for i in 0..STERILE_CONNECTOR_COUNT {
        let x = connector_x(i);
        let z = connector_z(i);
        let bore = centered_cylinder(
            format!("cell_prep_qc_sterile_connector_through_bore_{i}"),
            connector_bore_radius(i),
            CONNECTOR_PANEL_Y + 16.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, z);
        let clamp_screw = centered_cylinder(
            format!("cell_prep_qc_sterile_connector_clamp_screw_{i}"),
            3.4 / 2.0,
            CONNECTOR_PANEL_Y + 16.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, z + 20.0);
        bores = bores + bore + clamp_screw;
    }
    bores
}

fn sterile_cap_parking_posts() -> Part {
    let mut posts = Part::empty("cell_prep_qc_sterile_cap_parking_posts");
    for (i, x) in [-222.0, -168.0, 168.0, 222.0].iter().enumerate() {
        let post = centered_cylinder(
            format!("cell_prep_qc_sterile_cap_parking_post_{i}"),
            8.0,
            16.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            *x,
            -CONNECTOR_PANEL_Y / 2.0 - 8.0,
            -CONNECTOR_PANEL_Z / 2.0 + 54.0,
        );
        let tether_slot = centered_cube(
            format!("cell_prep_qc_sterile_cap_tether_slot_{i}"),
            4.0,
            18.0,
            12.0,
        )
        .translate(
            *x,
            -CONNECTOR_PANEL_Y / 2.0 - 8.0,
            -CONNECTOR_PANEL_Z / 2.0 + 54.0,
        );
        posts = posts + (post - tether_slot);
    }
    posts
}

fn panel_mount_flanges() -> Part {
    let left = centered_cube(
        "cell_prep_qc_connector_panel_left_flange",
        26.0,
        42.0,
        CONNECTOR_PANEL_Z + 10.0,
    )
    .translate(-CONNECTOR_PANEL_X / 2.0 - 13.0, 0.0, 0.0);
    let right = centered_cube(
        "cell_prep_qc_connector_panel_right_flange",
        26.0,
        42.0,
        CONNECTOR_PANEL_Z + 10.0,
    )
    .translate(CONNECTOR_PANEL_X / 2.0 + 13.0, 0.0, 0.0);
    left + right
}

fn panel_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_connector_panel_mount_holes");
    for (i, (x, z)) in [
        (
            -(CONNECTOR_PANEL_X / 2.0 + 13.0),
            -(CONNECTOR_PANEL_Z / 2.0 - 20.0),
        ),
        (
            CONNECTOR_PANEL_X / 2.0 + 13.0,
            -(CONNECTOR_PANEL_Z / 2.0 - 20.0),
        ),
        (
            -(CONNECTOR_PANEL_X / 2.0 + 13.0),
            CONNECTOR_PANEL_Z / 2.0 - 20.0,
        ),
        (
            CONNECTOR_PANEL_X / 2.0 + 13.0,
            CONNECTOR_PANEL_Z / 2.0 - 20.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_connector_panel_m5_mount_{i}"),
                5.3 / 2.0,
                48.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, *z);
    }
    holes
}

fn prime_waste_tray() -> Part {
    let tray = centered_cube(
        "cell_prep_qc_prime_waste_tray_body",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let waste_bottle_socket = centered_cylinder(
        "cell_prep_qc_waste_bottle_socket",
        43.0,
        WASTE_TRAY_Z + 2.0,
        52,
    )
    .translate(WASTE_TRAY_X / 2.0 - 76.0, -12.0, 0.0);
    let prime_trough = centered_cube(
        "cell_prep_qc_prime_flush_trough",
        WASTE_TRAY_X - 82.0,
        26.0,
        12.0,
    )
    .translate(-26.0, 38.0, WASTE_TRAY_Z / 2.0 - 5.0);
    let overflow_sump = centered_cube("cell_prep_qc_waste_overflow_sump", 116.0, 32.0, 12.0)
        .translate(WASTE_TRAY_X / 2.0 - 92.0, -50.0, WASTE_TRAY_Z / 2.0 - 5.0);
    let waste_drop = centered_cylinder("cell_prep_qc_waste_bottle_drop_bore", 9.0 / 2.0, 56.0, 28)
        .translate(WASTE_TRAY_X / 2.0 - 76.0, -12.0, 0.0);
    let line_in = centered_cylinder(
        "cell_prep_qc_prime_waste_line_in",
        FLUID_BORE_D / 2.0,
        WASTE_TRAY_X - 68.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-12.0, 38.0, 0.0);

    tray - waste_bottle_socket - prime_trough - overflow_sump - waste_drop - line_in
        + waste_luer_bulkhead()
        + waste_tube_comb()
        + waste_bottle_retainer()
}

fn waste_luer_bulkhead() -> Part {
    let bulkhead = centered_cube("cell_prep_qc_waste_luer_bulkhead_block", 72.0, 38.0, 24.0)
        .translate(-WASTE_TRAY_X / 2.0 + 44.0, 38.0, WASTE_TRAY_Z / 2.0 + 12.0);
    let tube = centered_cylinder(
        "cell_prep_qc_waste_luer_bulkhead_channel",
        8.0 / 2.0,
        76.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-WASTE_TRAY_X / 2.0 + 44.0, 38.0, WASTE_TRAY_Z / 2.0 + 12.0);
    let clamp_screw = centered_cylinder("cell_prep_qc_waste_luer_clamp_screw", 3.2 / 2.0, 32.0, 20)
        .translate(-WASTE_TRAY_X / 2.0 + 44.0, 49.0, WASTE_TRAY_Z / 2.0 + 12.0);
    bulkhead - tube - clamp_screw
}

fn waste_tube_comb() -> Part {
    let comb = centered_cube(
        "cell_prep_qc_waste_tube_strain_relief_comb",
        190.0,
        18.0,
        16.0,
    )
    .translate(-42.0, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 8.0);
    let mut cuts = Part::empty("cell_prep_qc_waste_tube_comb_cuts");
    for (i, x) in [-72.0, -36.0, 0.0, 36.0, 72.0].iter().enumerate() {
        let channel = centered_cylinder(
            format!("cell_prep_qc_waste_tube_comb_channel_{i}"),
            FLUID_BORE_D / 2.0,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 8.0);
        let slot = centered_cube(
            format!("cell_prep_qc_waste_tube_comb_slot_{i}"),
            FLUID_BORE_D + 1.0,
            24.0,
            10.0,
        )
        .translate(*x, -WASTE_TRAY_Y / 2.0 - 10.0, WASTE_TRAY_Z / 2.0 + 12.0);
        cuts = cuts + channel + slot;
    }
    comb - cuts
}

fn waste_bottle_retainer() -> Part {
    let outer = centered_cylinder("cell_prep_qc_waste_bottle_retainer_outer", 49.0, 8.0, 56)
        .translate(WASTE_TRAY_X / 2.0 - 76.0, -12.0, WASTE_TRAY_Z / 2.0 + 4.0);
    let inner = centered_cylinder("cell_prep_qc_waste_bottle_retainer_inner", 43.8, 9.0, 56)
        .translate(WASTE_TRAY_X / 2.0 - 76.0, -12.0, WASTE_TRAY_Z / 2.0 + 4.0);
    outer - inner
}

fn handoff_manifold() -> Part {
    let body = centered_cube(
        "cell_prep_qc_handoff_manifold_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let inlet_bore = centered_cylinder(
        "cell_prep_qc_handoff_conditioned_cell_inlet_bore",
        ROW_TRUNK_D / 2.0,
        HANDOFF_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 0.0);
    let purge_bore = centered_cylinder(
        "cell_prep_qc_handoff_prime_waste_bore",
        FLUID_BORE_D / 2.0,
        HANDOFF_X - 44.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -HANDOFF_Y / 2.0 + 26.0, 0.0);
    let top_slot = centered_cube(
        "cell_prep_qc_handoff_low_dead_volume_open_slot",
        HANDOFF_X - 44.0,
        ROW_TRUNK_D + 1.4,
        14.0,
    )
    .translate(0.0, 0.0, 9.0);

    body + row_handoff_bulkhead_tabs()
        + seeding_station_dock_tongue()
        + handoff_locator_bosses()
        + handoff_valve_cap_bosses()
        - inlet_bore
        - purge_bore
        - top_slot
        - row_handoff_channels()
        - handoff_valve_seat_cuts()
        - handoff_mount_holes()
}

fn row_handoff_channels() -> Part {
    let mut channels = Part::empty("cell_prep_qc_row_handoff_channels");
    for row in 0..ROWS {
        let y = row_handoff_y(row);
        let cross = centered_cylinder(
            format!("cell_prep_qc_row_{row}_handoff_cross_bore"),
            ROW_TRUNK_D / 2.0,
            HANDOFF_Y - 22.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(HANDOFF_X / 2.0 - 68.0, y / 2.0, 0.0);
        let port = centered_cylinder(
            format!("cell_prep_qc_row_{row}_handoff_output_port"),
            ROW_TRUNK_D / 2.0,
            52.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(HANDOFF_X / 2.0 - 20.0, y, 0.0);
        channels = channels + cross + port;
    }
    channels
}

fn row_handoff_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("cell_prep_qc_row_handoff_bulkhead_tabs");
    for row in 0..ROWS {
        let y = row_handoff_y(row);
        let tab = centered_cube(
            format!("cell_prep_qc_row_{row}_handoff_bulkhead_tab"),
            36.0,
            22.0,
            24.0,
        )
        .translate(HANDOFF_X / 2.0 + 18.0, y, 0.0);
        let tube = centered_cylinder(
            format!("cell_prep_qc_row_{row}_handoff_bulkhead_tube"),
            ROW_TRUNK_D / 2.0,
            40.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(HANDOFF_X / 2.0 + 18.0, y, 0.0);
        let screw = centered_cylinder(
            format!("cell_prep_qc_row_{row}_handoff_bulkhead_screw"),
            3.4 / 2.0,
            26.0,
            20,
        )
        .translate(HANDOFF_X / 2.0 + 18.0, y + 7.0, 0.0);
        tabs = tabs + (tab - tube - screw);
    }
    tabs
}

fn seeding_station_dock_tongue() -> Part {
    let tongue = centered_cube(
        "cell_prep_qc_seeding_station_dock_tongue",
        102.0,
        HANDOFF_Y - 18.0,
        12.0,
    )
    .translate(HANDOFF_X / 2.0 + 48.0, 0.0, -HANDOFF_Z / 2.0 + 6.0);
    let key_notch = centered_cube(
        "cell_prep_qc_seeding_station_dock_asymmetric_key_notch",
        24.0,
        22.0,
        14.0,
    )
    .translate(
        HANDOFF_X / 2.0 + 86.0,
        HANDOFF_Y / 2.0 - 24.0,
        -HANDOFF_Z / 2.0 + 6.0,
    );
    tongue - key_notch
}

fn handoff_locator_bosses() -> Part {
    let mut bosses = Part::empty("cell_prep_qc_handoff_locator_bosses");
    for (i, (x, y)) in [
        (-(HANDOFF_X / 2.0 - 28.0), -(HANDOFF_Y / 2.0 - 22.0)),
        (HANDOFF_X / 2.0 - 28.0, -(HANDOFF_Y / 2.0 - 22.0)),
        (-(HANDOFF_X / 2.0 - 28.0), HANDOFF_Y / 2.0 - 22.0),
        (HANDOFF_X / 2.0 - 28.0, HANDOFF_Y / 2.0 - 22.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("cell_prep_qc_handoff_locator_boss_{i}"),
            9.0,
            7.0,
            32,
        )
        .translate(*x, *y, HANDOFF_Z / 2.0 + 3.5);
        let bore = centered_cylinder(
            format!("cell_prep_qc_handoff_locator_socket_{i}"),
            3.2 / 2.0,
            8.0,
            20,
        )
        .translate(*x, *y, HANDOFF_Z / 2.0 + 3.5);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn handoff_valve_cap_bosses() -> Part {
    let mut bosses = Part::empty("cell_prep_qc_handoff_valve_cap_bosses");
    for (i, x) in [-112.0, -42.0, 42.0, 112.0].iter().enumerate() {
        bosses = bosses
            + centered_cylinder(
                format!("cell_prep_qc_handoff_valve_cap_boss_{i}"),
                11.0,
                8.0,
                32,
            )
            .translate(*x, 0.0, HANDOFF_Z / 2.0 + 4.0);
    }
    bosses
}

fn handoff_valve_seat_cuts() -> Part {
    let mut cuts = Part::empty("cell_prep_qc_handoff_valve_seat_cuts");
    for (i, x) in [-112.0, -42.0, 42.0, 112.0].iter().enumerate() {
        let stem = centered_cylinder(
            format!("cell_prep_qc_handoff_valve_stem_bore_{i}"),
            4.0 / 2.0,
            HANDOFF_Z + 10.0,
            24,
        )
        .translate(*x, 0.0, 4.0);
        let actuator = centered_cube(
            format!("cell_prep_qc_handoff_valve_actuator_pocket_{i}"),
            24.0,
            24.0,
            12.0,
        )
        .translate(*x, 0.0, HANDOFF_Z / 2.0 - 5.0);
        cuts = cuts + stem + actuator;
    }
    cuts
}

fn handoff_mount_holes() -> Part {
    let mut holes = Part::empty("cell_prep_qc_handoff_mount_holes");
    for (i, (x, y)) in [
        (-(HANDOFF_X / 2.0 - 18.0), -(HANDOFF_Y / 2.0 - 16.0)),
        (HANDOFF_X / 2.0 - 18.0, -(HANDOFF_Y / 2.0 - 16.0)),
        (-(HANDOFF_X / 2.0 - 18.0), HANDOFF_Y / 2.0 - 16.0),
        (HANDOFF_X / 2.0 - 18.0, HANDOFF_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cell_prep_qc_handoff_m5_mount_{i}"),
                5.3 / 2.0,
                HANDOFF_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn routed_tube_placeholders() -> Part {
    let bag_to_mixer = tube_run_x(
        "cell_prep_qc_route_bag_to_mixer",
        BAG_X + BAG_TRAY_X / 2.0 - 32.0,
        MIXER_X_POS - MIXER_X / 2.0 - 18.0,
        -38.0,
        DECK_Z + 60.0,
        RECIRC_BORE_D,
    );
    let mixer_to_qc = tube_run_x(
        "cell_prep_qc_route_mixer_to_qc",
        MIXER_X_POS + MIXER_X / 2.0 + 18.0,
        QC_X - QC_BLOCK_X / 2.0,
        -34.0,
        DECK_Z + 38.0,
        RECIRC_BORE_D,
    );
    let qc_to_bubble = tube_run_x(
        "cell_prep_qc_route_qc_to_bubble",
        QC_X + QC_BLOCK_X / 2.0,
        BUBBLE_X - BUBBLE_BLOCK_X / 2.0,
        54.0,
        DECK_Z + 42.0,
        FLUID_BORE_D,
    );
    let bubble_to_handoff = tube_run_x(
        "cell_prep_qc_route_bubble_to_handoff",
        BUBBLE_X + BUBBLE_BLOCK_X / 2.0,
        HANDOFF_X_POS - HANDOFF_X / 2.0,
        -20.0,
        DECK_Z + 40.0,
        FLUID_BORE_D,
    );
    let prime_waste = tube_run_y(
        "cell_prep_qc_route_prime_to_waste",
        WASTE_X - WASTE_TRAY_X / 2.0 + 44.0,
        BUBBLE_Y - BUBBLE_BLOCK_Y / 2.0,
        WASTE_Y + WASTE_TRAY_Y / 2.0,
        DECK_Z + 38.0,
        FLUID_BORE_D,
    );

    bag_to_mixer + mixer_to_qc + qc_to_bubble + bubble_to_handoff + prime_waste
}

fn tube_run_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    let len = (x_b - x_a).abs();
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x_a + x_b) / 2.0, y, z)
}

fn tube_run_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    let len = (y_b - y_a).abs();
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y_a + y_b) / 2.0, z)
}

fn fluid_connector_tab(name: &str) -> Part {
    let body = centered_cube(
        format!("cell_prep_qc_{name}_connector_tab"),
        36.0,
        34.0,
        22.0,
    );
    let tube = centered_cylinder(
        format!("cell_prep_qc_{name}_connector_tube_clearance"),
        FLUID_BORE_D / 2.0,
        40.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0);
    let screw = centered_cylinder(
        format!("cell_prep_qc_{name}_connector_screw"),
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

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(MODULE_X / 2.0 - 42.0), -(MODULE_Y / 2.0 - 42.0)),
        (MODULE_X / 2.0 - 42.0, -(MODULE_Y / 2.0 - 42.0)),
        (-(MODULE_X / 2.0 - 42.0), MODULE_Y / 2.0 - 42.0),
        (MODULE_X / 2.0 - 42.0, MODULE_Y / 2.0 - 42.0),
        (0.0, -(MODULE_Y / 2.0 - 42.0)),
        (0.0, MODULE_Y / 2.0 - 42.0),
        (-(MODULE_X / 2.0 - 42.0), 0.0),
        (MODULE_X / 2.0 - 42.0, 0.0),
    ]
}

fn skid_locator_points() -> [(f64, f64); 4] {
    [
        (-(MODULE_X / 2.0 - 88.0), -(MODULE_Y / 2.0 - 86.0)),
        (MODULE_X / 2.0 - 88.0, -(MODULE_Y / 2.0 - 86.0)),
        (-(MODULE_X / 2.0 - 88.0), MODULE_Y / 2.0 - 86.0),
        (MODULE_X / 2.0 - 88.0, MODULE_Y / 2.0 - 86.0),
    ]
}

fn row_y(row: usize) -> f64 {
    -((ROWS as f64 - 1.0) * PITCH_Y) / 2.0 + row as f64 * PITCH_Y
}

fn row_handoff_y(row: usize) -> f64 {
    row_y(row) * (HANDOFF_Y - 44.0) / ARRAY_Y
}

fn connector_x(i: usize) -> f64 {
    -((STERILE_CONNECTOR_COUNT as f64 - 1.0) * CONNECTOR_PITCH) / 2.0 + i as f64 * CONNECTOR_PITCH
}

fn connector_z(i: usize) -> f64 {
    if i < ROWS {
        10.0
    } else {
        -CONNECTOR_PANEL_Z / 2.0 + 42.0
    }
}

fn connector_outer_radius(i: usize) -> f64 {
    if i < ROWS {
        15.0
    } else {
        12.0
    }
}

fn connector_bore_radius(i: usize) -> f64 {
    if i < ROWS {
        ROW_TRUNK_D / 2.0
    } else {
        FLUID_BORE_D / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_deck_contains_all_major_zones() {
        assert!(BAG_X - TEMP_ZONE_X / 2.0 > -MODULE_X / 2.0);
        assert!(BUBBLE_X + BUBBLE_BLOCK_X / 2.0 < MODULE_X / 2.0);
        assert!(CONNECTOR_PANEL_Y_POS + CONNECTOR_PANEL_Y / 2.0 <= MODULE_Y / 2.0 + 2.0);
        assert!(WASTE_Y - WASTE_TRAY_Y / 2.0 > -MODULE_Y / 2.0);
    }

    #[test]
    fn cassette_reference_matches_twenty_chip_station() {
        assert_eq!(COLS * ROWS, 20);
        assert_eq!(PITCH_X, REVC_CHIP_LENGTH + GUTTER);
        assert_eq!(PITCH_Y, REVC_CHIP_WIDTH + GUTTER);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);
    }

    #[test]
    fn sterile_connector_panel_has_rows_plus_service_ports() {
        assert_eq!(STERILE_CONNECTOR_COUNT, ROWS + 4);
        assert!(connector_x(0) < connector_x(STERILE_CONNECTOR_COUNT - 1));
        assert!(connector_x(0) > -CONNECTOR_PANEL_X / 2.0 + 20.0);
        assert!(connector_x(STERILE_CONNECTOR_COUNT - 1) < CONNECTOR_PANEL_X / 2.0 - 20.0);
    }

    #[test]
    fn handoff_rows_fit_scaled_cassette_pitch() {
        assert!(row_handoff_y(0) < row_handoff_y(ROWS - 1));
        assert!(row_handoff_y(0) > -HANDOFF_Y / 2.0 + 12.0);
        assert!(row_handoff_y(ROWS - 1) < HANDOFF_Y / 2.0 - 12.0);
    }

    #[test]
    fn temperature_hold_has_expected_thermal_clearance() {
        assert!(TEMP_ZONE_X > BAG_ENVELOPE_X + 60.0);
        assert!(TEMP_ZONE_Y > BAG_ENVELOPE_Y + 50.0);
        assert!(PELTIER_SIZE > 40.0);
    }
}
