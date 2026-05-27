use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media reservoir temperature stratification and recirculation validation
// station.
//
// Intent:
// - Package a no-cell engineering validation fixture for checking whether a
//   closed media reservoir surrogate is thermally uniform under recirculation.
// - Keep the insulated bag/bottle nest, top/middle/bottom temperature probe
//   wells, recirculation inlet/outlet witness loops, mixing baffle surrogate,
//   flow sensor coupon dock, dead-zone dye witness pockets, load-cell pad,
//   leak/condensate tray, barcode/status lands, and robot/service keepouts
//   visible in one deterministic CAD envelope.
// - Model mechanical interfaces only. This is not a biological protocol,
//   acceptance threshold, cell-culture procedure, or validated process design.

const OUTPUTS: [&str; 13] = [
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_leak_condensate_tray.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_insulated_reservoir_nest.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_temperature_probe_well_stack.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_recirculation_inlet_witness_loop.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_recirculation_outlet_witness_loop.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_mixing_baffle_surrogate.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_flow_sensor_coupon_dock.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_dead_zone_dye_witness_pockets.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_load_cell_pad_placeholder.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_barcode_status_lands.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_robot_service_keepouts.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_service_tool_datum_lands.stl",
    "output/closed_media_reservoir_temperature_stratification_recirculation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "leak_condensate_tray",
    "insulated_bag_bottle_nest",
    "top_temperature_probe_well",
    "middle_temperature_probe_well",
    "bottom_temperature_probe_well",
    "recirculation_inlet_witness_loop",
    "recirculation_outlet_witness_loop",
    "mixing_baffle_surrogate",
    "flow_sensor_coupon_dock",
    "dead_zone_dye_witness_pockets",
    "load_cell_pad_placeholder",
    "barcode_status_lands",
    "robot_keepout",
    "service_keepout",
];

const DECK_X: f64 = 1040.0;
const DECK_Y: f64 = 720.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 18.0;
const TRAY_RIM_Z: f64 = 38.0;
const CONDENSATE_CHANNEL_DEPTH: f64 = 7.0;
const DRAIN_PORT_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.6;

const NEST_X: f64 = 380.0;
const NEST_Y: f64 = 282.0;
const NEST_Z: f64 = 92.0;
const NEST_POS: (f64, f64) = (-218.0, 78.0);
const RESERVOIR_X: f64 = 292.0;
const RESERVOIR_Y: f64 = 194.0;
const RESERVOIR_Z: f64 = 138.0;
const INSULATION_WALL: f64 = 28.0;
const BAG_CRADLE_RADIUS: f64 = 32.0;
const NEST_DATUM_PINS: usize = 6;

const PROBE_STACK_X: f64 = 340.0;
const PROBE_STACK_Y: f64 = 92.0;
const PROBE_STACK_Z: f64 = 184.0;
const PROBE_STACK_POS: (f64, f64) = (-218.0, 290.0);
const PROBE_WELL_COUNT: usize = 3;
const PROBE_WELL_D: f64 = 7.0;
const PROBE_WELL_LENGTH: f64 = 268.0;
const PROBE_LEVEL_Z: [f64; PROBE_WELL_COUNT] = [40.0, 92.0, 144.0];
const PROBE_STRIKE_LANDS: usize = PROBE_WELL_COUNT;

const LOOP_PANEL_X: f64 = 360.0;
const LOOP_PANEL_Y: f64 = 86.0;
const LOOP_PANEL_Z: f64 = 42.0;
const INLET_LOOP_POS: (f64, f64) = (274.0, 186.0);
const OUTLET_LOOP_POS: (f64, f64) = (274.0, 74.0);
const LOOP_TUBE_D: f64 = 6.4;
const LOOP_WINDOW_X: f64 = 118.0;
const LOOP_WINDOW_Y: f64 = 34.0;
const LOOP_CLIPS_PER_PANEL: usize = 4;
const LOOP_SEPTUM_PORTS_PER_PANEL: usize = 2;

const BAFFLE_X: f64 = 246.0;
const BAFFLE_Y: f64 = 154.0;
const BAFFLE_Z: f64 = 118.0;
const BAFFLE_PLATES: usize = 5;
const BAFFLE_PLATE_W: f64 = 6.0;
const BAFFLE_FLOW_GAP: f64 = 22.0;

const FLOW_DOCK_X: f64 = 256.0;
const FLOW_DOCK_Y: f64 = 152.0;
const FLOW_DOCK_Z: f64 = 46.0;
const FLOW_DOCK_POS: (f64, f64) = (310.0, -98.0);
const FLOW_COUPON_X: f64 = 86.0;
const FLOW_COUPON_Y: f64 = 44.0;
const FLOW_COUPON_SLOTS: usize = 3;
const FLOW_COUPON_CHANNEL_D: f64 = 8.0;

const DYE_POCKET_PANEL_X: f64 = 436.0;
const DYE_POCKET_PANEL_Y: f64 = 118.0;
const DYE_POCKET_PANEL_Z: f64 = 34.0;
const DYE_POCKET_POS: (f64, f64) = (-196.0, -150.0);
const DYE_POCKET_COUNT: usize = 8;
const DYE_POCKET_D: f64 = 18.0;
const DYE_POCKET_DEPTH: f64 = 22.0;

const LOAD_CELL_PAD_X: f64 = 420.0;
const LOAD_CELL_PAD_Y: f64 = 324.0;
const LOAD_CELL_PAD_Z: f64 = 18.0;
const LOAD_CELL_PAD_POS: (f64, f64) = NEST_POS;
const LOAD_CELL_BOSSES: usize = 4;
const LOAD_CELL_CABLE_SLOT_X: f64 = 84.0;
const LOAD_CELL_CABLE_SLOT_Y: f64 = 18.0;

const STATUS_PANEL_X: f64 = 392.0;
const STATUS_PANEL_Y: f64 = 92.0;
const STATUS_PANEL_Z: f64 = 14.0;
const STATUS_PANEL_POS: (f64, f64) = (258.0, -280.0);
const BARCODE_LANDS: usize = 6;
const STATUS_LANDS: usize = 4;
const RFID_LANDS: usize = 2;

const TOOL_DATUM_X: f64 = 310.0;
const TOOL_DATUM_Y: f64 = 92.0;
const TOOL_DATUM_Z: f64 = 22.0;
const TOOL_DATUM_POS: (f64, f64) = (-340.0, -292.0);
const TOOL_DATUM_LANDS: usize = 5;

const ROBOT_KEEPOUT_X: f64 = 760.0;
const ROBOT_KEEPOUT_Y: f64 = 112.0;
const ROBOT_KEEPOUT_Z: f64 = 156.0;
const SERVICE_KEEPOUT_X: f64 = 128.0;
const SERVICE_KEEPOUT_Y: f64 = 430.0;
const SERVICE_KEEPOUT_Z: f64 = 190.0;
const TOP_PROBE_SERVICE_Z: f64 = 248.0;
const KEEP_OUT_ZONES: usize = 4;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = leak_condensate_tray();
    write_part(&tray, OUTPUTS[0]);

    let nest = insulated_reservoir_nest();
    write_part(&nest, OUTPUTS[1]);

    let probes = temperature_probe_well_stack();
    write_part(&probes, OUTPUTS[2]);

    let inlet = recirculation_witness_loop("inlet");
    write_part(&inlet, OUTPUTS[3]);

    let outlet = recirculation_witness_loop("outlet");
    write_part(&outlet, OUTPUTS[4]);

    let baffle = mixing_baffle_surrogate();
    write_part(&baffle, OUTPUTS[5]);

    let flow = flow_sensor_coupon_dock();
    write_part(&flow, OUTPUTS[6]);

    let dye = dead_zone_dye_witness_pockets();
    write_part(&dye, OUTPUTS[7]);

    let load_cell = load_cell_pad_placeholder();
    write_part(&load_cell, OUTPUTS[8]);

    let status = barcode_status_lands();
    write_part(&status, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    write_part(&keepouts, OUTPUTS[10]);

    let tool_datums = service_tool_datum_lands();
    write_part(&tool_datums, OUTPUTS[11]);

    let assembly =
        tray + load_cell.translate(
            LOAD_CELL_PAD_POS.0,
            LOAD_CELL_PAD_POS.1,
            DECK_Z + LOAD_CELL_PAD_Z / 2.0,
        ) + nest.translate(
            NEST_POS.0,
            NEST_POS.1,
            DECK_Z + LOAD_CELL_PAD_Z + NEST_Z / 2.0,
        ) + baffle.translate(
            NEST_POS.0,
            NEST_POS.1,
            DECK_Z + LOAD_CELL_PAD_Z + NEST_Z + BAFFLE_Z / 2.0,
        ) + probes.translate(
            PROBE_STACK_POS.0,
            PROBE_STACK_POS.1,
            DECK_Z + PROBE_STACK_Z / 2.0,
        ) + inlet.translate(
            INLET_LOOP_POS.0,
            INLET_LOOP_POS.1,
            DECK_Z + LOOP_PANEL_Z / 2.0 + 12.0,
        ) + outlet.translate(
            OUTLET_LOOP_POS.0,
            OUTLET_LOOP_POS.1,
            DECK_Z + LOOP_PANEL_Z / 2.0 + 12.0,
        ) + flow.translate(FLOW_DOCK_POS.0, FLOW_DOCK_POS.1, DECK_Z + FLOW_DOCK_Z / 2.0)
            + dye.translate(
                DYE_POCKET_POS.0,
                DYE_POCKET_POS.1,
                DECK_Z + DYE_POCKET_PANEL_Z / 2.0,
            )
            + status.translate(
                STATUS_PANEL_POS.0,
                STATUS_PANEL_POS.1,
                DECK_Z + STATUS_PANEL_Z / 2.0,
            )
            + tool_datums.translate(
                TOOL_DATUM_POS.0,
                TOOL_DATUM_POS.1,
                DECK_Z + TOOL_DATUM_Z / 2.0,
            )
            + recirculation_span_tubes()
            + keepouts;

    write_part(&assembly, OUTPUTS[12]);

    println!(
        "Closed media reservoir thermal validation station: {:.0}mm x {:.0}mm deck, {:.0}mm x {:.0}mm insulated reservoir nest around a {:.0}mm tall reservoir surrogate, {} top/middle/bottom probe wells with {} strike lands, {} recirculation witness loop panels, {} baffle surrogate plates, {} flow coupon slots, {} dye witness pockets, {} load-cell bosses, {} keepout zones, and {} required feature groups.",
        DECK_X,
        DECK_Y,
        NEST_X,
        NEST_Y,
        RESERVOIR_Z,
        PROBE_WELL_COUNT,
        PROBE_STRIKE_LANDS,
        2,
        BAFFLE_PLATES,
        FLOW_COUPON_SLOTS,
        DYE_POCKET_COUNT,
        LOAD_CELL_BOSSES,
        KEEP_OUT_ZONES,
        REQUIRED_FEATURES.len()
    );
    println!(
        "No-cell validation CAD only: probe service clearance {:.0}mm Z, robot front keepout {:.0}mm, side service keepout {:.0}mm, output STLs {}.",
        TOP_PROBE_SERVICE_Z,
        ROBOT_KEEPOUT_Y,
        SERVICE_KEEPOUT_X,
        OUTPUTS.len()
    );
}

fn write_part(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn leak_condensate_tray() -> Part {
    let deck = centered_cube(
        "closed_reservoir_stratification_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        "closed_reservoir_stratification_condensate_sump",
        DECK_X - 110.0,
        DECK_Y - 112.0,
        CONDENSATE_CHANNEL_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - CONDENSATE_CHANNEL_DEPTH / 2.0);
    let front_channel = centered_cube(
        "closed_reservoir_stratification_front_condensate_channel",
        DECK_X - 160.0,
        18.0,
        CONDENSATE_CHANNEL_DEPTH + 2.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 78.0,
        DECK_Z - CONDENSATE_CHANNEL_DEPTH / 2.0,
    );
    let rear_channel = centered_cube(
        "closed_reservoir_stratification_rear_condensate_channel",
        DECK_X - 160.0,
        18.0,
        CONDENSATE_CHANNEL_DEPTH + 2.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 78.0,
        DECK_Z - CONDENSATE_CHANNEL_DEPTH / 2.0,
    );
    let drain = centered_cylinder(
        "closed_reservoir_stratification_condensate_drain_port",
        DRAIN_PORT_D / 2.0,
        46.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 118.0, -DECK_Y / 2.0 + 56.0, DECK_Z / 2.0);
    let mut mount_holes = Part::empty("closed_reservoir_stratification_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("closed_reservoir_stratification_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }

    let rim_north = centered_cube(
        "closed_reservoir_stratification_tray_rim_north",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rim_south = centered_cube(
        "closed_reservoir_stratification_tray_rim_south",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - TRAY_RIM_W / 2.0),
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rim_west = centered_cube(
        "closed_reservoir_stratification_tray_rim_west",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        -(DECK_X / 2.0 - TRAY_RIM_W / 2.0),
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rim_east = centered_cube(
        "closed_reservoir_stratification_tray_rim_east",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );

    deck - sump - front_channel - rear_channel - drain - mount_holes
        + rim_north
        + rim_south
        + rim_west
        + rim_east
}

fn insulated_reservoir_nest() -> Part {
    let shell = centered_cube(
        "closed_reservoir_stratification_insulated_nest_shell",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let reservoir_pocket = centered_cube(
        "closed_reservoir_stratification_bag_bottle_reservoir_pocket",
        RESERVOIR_X,
        RESERVOIR_Y,
        NEST_Z - 16.0,
    )
    .translate(0.0, 0.0, 12.0);
    let bag_cradle = centered_cylinder(
        "closed_reservoir_stratification_flexible_bag_cradle_radius",
        BAG_CRADLE_RADIUS,
        RESERVOIR_X + 18.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -NEST_Z / 2.0 + 30.0);
    let insulation_band = centered_cube(
        "closed_reservoir_stratification_removable_insulation_band",
        NEST_X + 18.0,
        INSULATION_WALL,
        24.0,
    )
    .translate(0.0, NEST_Y / 2.0 + INSULATION_WALL / 2.0 - 6.0, 12.0);
    let mut datum_pins = Part::empty("closed_reservoir_stratification_nest_datum_pins");
    for i in 0..NEST_DATUM_PINS {
        let side = if i < 3 { -1.0 } else { 1.0 };
        let local = i % 3;
        datum_pins = datum_pins
            + centered_cylinder(
                format!("closed_reservoir_stratification_nest_datum_pin_{i}"),
                4.0,
                16.0,
                28,
            )
            .translate(
                -110.0 + local as f64 * 110.0,
                side * (NEST_Y / 2.0 - 22.0),
                NEST_Z / 2.0 + 8.0,
            );
    }
    let label_flat = centered_cube(
        "closed_reservoir_stratification_nest_orientation_flat",
        116.0,
        8.0,
        16.0,
    )
    .translate(0.0, -NEST_Y / 2.0 - 1.0, NEST_Z / 2.0 - 16.0);

    shell - reservoir_pocket - bag_cradle + insulation_band + datum_pins + label_flat
}

fn temperature_probe_well_stack() -> Part {
    let backplane = centered_cube(
        "closed_reservoir_stratification_probe_stack_backplane",
        PROBE_STACK_X,
        PROBE_STACK_Y,
        PROBE_STACK_Z,
    );
    let mut wells = Part::empty("closed_reservoir_stratification_probe_well_bores");
    let mut strike_lands = Part::empty("closed_reservoir_stratification_probe_strike_lands");
    for i in 0..PROBE_WELL_COUNT {
        let z = -PROBE_STACK_Z / 2.0 + PROBE_LEVEL_Z[i];
        wells = wells
            + centered_cylinder(
                format!("closed_reservoir_stratification_probe_well_bore_{i}"),
                PROBE_WELL_D / 2.0,
                PROBE_WELL_LENGTH,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, -8.0, z);
        strike_lands = strike_lands
            + centered_cube(
                format!("closed_reservoir_stratification_probe_level_strike_land_{i}"),
                74.0,
                12.0,
                14.0,
            )
            .translate(PROBE_STACK_X / 2.0 - 54.0, PROBE_STACK_Y / 2.0 + 8.0, z);
    }
    let service_handle = centered_cube(
        "closed_reservoir_stratification_probe_stack_service_handle",
        142.0,
        18.0,
        28.0,
    )
    .translate(0.0, -PROBE_STACK_Y / 2.0 - 10.0, PROBE_STACK_Z / 2.0 - 28.0);

    backplane - wells + strike_lands + service_handle
}

fn recirculation_witness_loop(kind: &str) -> Part {
    let panel = centered_cube(
        format!("closed_reservoir_stratification_{kind}_loop_panel"),
        LOOP_PANEL_X,
        LOOP_PANEL_Y,
        LOOP_PANEL_Z,
    );
    let window = centered_cube(
        format!("closed_reservoir_stratification_{kind}_loop_clear_witness_window"),
        LOOP_WINDOW_X,
        LOOP_WINDOW_Y,
        LOOP_PANEL_Z + 2.0,
    )
    .translate(0.0, 0.0, 4.0);
    let tube_left = centered_cylinder(
        format!("closed_reservoir_stratification_{kind}_loop_left_leg"),
        LOOP_TUBE_D / 2.0,
        LOOP_WINDOW_Y + 28.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-LOOP_WINDOW_X / 2.0, 0.0, LOOP_PANEL_Z / 2.0 + 8.0);
    let tube_right = centered_cylinder(
        format!("closed_reservoir_stratification_{kind}_loop_right_leg"),
        LOOP_TUBE_D / 2.0,
        LOOP_WINDOW_Y + 28.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LOOP_WINDOW_X / 2.0, 0.0, LOOP_PANEL_Z / 2.0 + 8.0);
    let tube_top = centered_cylinder(
        format!("closed_reservoir_stratification_{kind}_loop_top_span"),
        LOOP_TUBE_D / 2.0,
        LOOP_WINDOW_X,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOOP_WINDOW_Y / 2.0, LOOP_PANEL_Z / 2.0 + 8.0);
    let tube_bottom = centered_cylinder(
        format!("closed_reservoir_stratification_{kind}_loop_bottom_span"),
        LOOP_TUBE_D / 2.0,
        LOOP_WINDOW_X,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -LOOP_WINDOW_Y / 2.0, LOOP_PANEL_Z / 2.0 + 8.0);
    let mut clips = Part::empty(format!("closed_reservoir_stratification_{kind}_loop_clips"));
    for i in 0..LOOP_CLIPS_PER_PANEL {
        let x = if i % 2 == 0 { -92.0 } else { 92.0 };
        let y = if i < 2 { -28.0 } else { 28.0 };
        clips = clips
            + centered_cube(
                format!("closed_reservoir_stratification_{kind}_loop_clip_{i}"),
                30.0,
                12.0,
                16.0,
            )
            .translate(x, y, LOOP_PANEL_Z / 2.0 + 8.0);
    }
    let mut septa = Part::empty(format!(
        "closed_reservoir_stratification_{kind}_septum_ports"
    ));
    for i in 0..LOOP_SEPTUM_PORTS_PER_PANEL {
        septa = septa
            + centered_cylinder(
                format!("closed_reservoir_stratification_{kind}_septum_port_{i}"),
                8.0,
                14.0,
                28,
            )
            .translate(
                -LOOP_PANEL_X / 2.0 + 34.0 + i as f64 * 38.0,
                LOOP_PANEL_Y / 2.0 - 22.0,
                LOOP_PANEL_Z / 2.0 + 7.0,
            );
    }

    panel - window + tube_left + tube_right + tube_top + tube_bottom + clips + septa
}

fn mixing_baffle_surrogate() -> Part {
    let frame = centered_cube(
        "closed_reservoir_stratification_baffle_surrogate_outer_frame",
        BAFFLE_X,
        BAFFLE_Y,
        12.0,
    )
    .translate(0.0, 0.0, -BAFFLE_Z / 2.0 + 6.0);
    let mut plates = Part::empty("closed_reservoir_stratification_baffle_surrogate_plates");
    for i in 0..BAFFLE_PLATES {
        let x = -((BAFFLE_PLATES as f64 - 1.0) * 42.0) / 2.0 + i as f64 * 42.0;
        let y_shift = if i % 2 == 0 {
            -BAFFLE_FLOW_GAP
        } else {
            BAFFLE_FLOW_GAP
        };
        plates = plates
            + centered_cube(
                format!("closed_reservoir_stratification_baffle_plate_{i}"),
                BAFFLE_PLATE_W,
                BAFFLE_Y - 34.0,
                BAFFLE_Z,
            )
            .translate(x, y_shift, 0.0);
    }
    let lifting_land = centered_cube(
        "closed_reservoir_stratification_baffle_lifting_land",
        96.0,
        18.0,
        14.0,
    )
    .translate(0.0, BAFFLE_Y / 2.0 + 8.0, BAFFLE_Z / 2.0 - 12.0);

    frame + plates + lifting_land
}

fn flow_sensor_coupon_dock() -> Part {
    let dock = centered_cube(
        "closed_reservoir_stratification_flow_sensor_coupon_dock",
        FLOW_DOCK_X,
        FLOW_DOCK_Y,
        FLOW_DOCK_Z,
    );
    let mut coupon_slots = Part::empty("closed_reservoir_stratification_flow_coupon_slots");
    let mut channels = Part::empty("closed_reservoir_stratification_flow_coupon_channels");
    for i in 0..FLOW_COUPON_SLOTS {
        let y = -((FLOW_COUPON_SLOTS as f64 - 1.0) * 42.0) / 2.0 + i as f64 * 42.0;
        coupon_slots = coupon_slots
            + centered_cube(
                format!("closed_reservoir_stratification_flow_coupon_slot_{i}"),
                FLOW_COUPON_X,
                FLOW_COUPON_Y,
                FLOW_DOCK_Z / 2.0,
            )
            .translate(-28.0, y, FLOW_DOCK_Z / 4.0 + 2.0);
        channels = channels
            + centered_cylinder(
                format!("closed_reservoir_stratification_flow_coupon_channel_{i}"),
                FLOW_COUPON_CHANNEL_D / 2.0,
                FLOW_DOCK_X - 58.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, FLOW_DOCK_Z / 2.0 + 2.0);
    }
    let sensor_cable_relief = centered_cube(
        "closed_reservoir_stratification_flow_sensor_cable_relief",
        26.0,
        FLOW_DOCK_Y - 28.0,
        16.0,
    )
    .translate(FLOW_DOCK_X / 2.0 - 22.0, 0.0, FLOW_DOCK_Z / 2.0 + 4.0);

    dock - coupon_slots + channels + sensor_cable_relief
}

fn dead_zone_dye_witness_pockets() -> Part {
    let panel = centered_cube(
        "closed_reservoir_stratification_dead_zone_dye_pocket_panel",
        DYE_POCKET_PANEL_X,
        DYE_POCKET_PANEL_Y,
        DYE_POCKET_PANEL_Z,
    );
    let mut pockets = Part::empty("closed_reservoir_stratification_dead_zone_dye_pocket_cuts");
    let mut lands = Part::empty("closed_reservoir_stratification_dead_zone_dye_pocket_lands");
    for i in 0..DYE_POCKET_COUNT {
        let (x, y) = dye_pocket_xy(i);
        pockets = pockets
            + centered_cylinder(
                format!("closed_reservoir_stratification_dead_zone_dye_pocket_{i}"),
                DYE_POCKET_D / 2.0,
                DYE_POCKET_DEPTH,
                32,
            )
            .translate(
                x,
                y,
                DYE_POCKET_PANEL_Z / 2.0 - DYE_POCKET_DEPTH / 2.0 + 0.2,
            );
        lands = lands
            + centered_cube(
                format!("closed_reservoir_stratification_dead_zone_dye_label_land_{i}"),
                38.0,
                12.0,
                4.0,
            )
            .translate(x, y + 22.0, DYE_POCKET_PANEL_Z / 2.0 + 2.0);
    }

    panel - pockets + lands
}

fn load_cell_pad_placeholder() -> Part {
    let pad = centered_cube(
        "closed_reservoir_stratification_load_cell_pad_placeholder",
        LOAD_CELL_PAD_X,
        LOAD_CELL_PAD_Y,
        LOAD_CELL_PAD_Z,
    );
    let cable_slot = centered_cube(
        "closed_reservoir_stratification_load_cell_cable_slot",
        LOAD_CELL_CABLE_SLOT_X,
        LOAD_CELL_CABLE_SLOT_Y,
        LOAD_CELL_PAD_Z + 2.0,
    )
    .translate(
        LOAD_CELL_PAD_X / 2.0 - LOAD_CELL_CABLE_SLOT_X / 2.0 - 16.0,
        0.0,
        0.0,
    );
    let mut bosses = Part::empty("closed_reservoir_stratification_load_cell_mount_bosses");
    for i in 0..LOAD_CELL_BOSSES {
        let sx = if i % 2 == 0 { -1.0 } else { 1.0 };
        let sy = if i < 2 { -1.0 } else { 1.0 };
        bosses = bosses
            + centered_cylinder(
                format!("closed_reservoir_stratification_load_cell_boss_{i}"),
                10.0,
                12.0,
                32,
            )
            .translate(
                sx * (LOAD_CELL_PAD_X / 2.0 - 42.0),
                sy * (LOAD_CELL_PAD_Y / 2.0 - 38.0),
                LOAD_CELL_PAD_Z / 2.0 + 6.0,
            );
    }

    pad - cable_slot + bosses
}

fn barcode_status_lands() -> Part {
    let panel = centered_cube(
        "closed_reservoir_stratification_barcode_status_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    );
    let mut lands = Part::empty("closed_reservoir_stratification_barcode_status_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reservoir_stratification_barcode_land_{i}"),
                96.0,
                12.0,
                4.0,
            )
            .translate(-130.0, barcode_y(i), STATUS_PANEL_Z / 2.0 + 2.0);
    }
    for i in 0..STATUS_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reservoir_stratification_status_land_{i}"),
                64.0,
                18.0,
                4.0,
            )
            .translate(6.0, status_y(i), STATUS_PANEL_Z / 2.0 + 2.0);
    }
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("closed_reservoir_stratification_rfid_land_{i}"),
                18.0,
                4.0,
                36,
            )
            .translate(132.0, -22.0 + i as f64 * 44.0, STATUS_PANEL_Z / 2.0 + 2.0);
    }

    panel + lands
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        "closed_reservoir_stratification_front_robot_keepout",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 96.0, DECK_Z + ROBOT_KEEPOUT_Z / 2.0);
    let rear_probe_service = keepout_box(
        "closed_reservoir_stratification_rear_probe_service_keepout",
        ROBOT_KEEPOUT_X,
        92.0,
        TOP_PROBE_SERVICE_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 68.0, DECK_Z + TOP_PROBE_SERVICE_Z / 2.0);
    let left_service = keepout_box(
        "closed_reservoir_stratification_left_reservoir_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(-DECK_X / 2.0 + 74.0, 0.0, DECK_Z + SERVICE_KEEPOUT_Z / 2.0);
    let right_flow_service = keepout_box(
        "closed_reservoir_stratification_right_flow_sensor_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(DECK_X / 2.0 - 74.0, 0.0, DECK_Z + SERVICE_KEEPOUT_Z / 2.0);

    front_robot + rear_probe_service + left_service + right_flow_service
}

fn service_tool_datum_lands() -> Part {
    let rail = centered_cube(
        "closed_reservoir_stratification_service_tool_datum_rail",
        TOOL_DATUM_X,
        TOOL_DATUM_Y,
        TOOL_DATUM_Z,
    );
    let mut datums = Part::empty("closed_reservoir_stratification_service_tool_datum_lands");
    for i in 0..TOOL_DATUM_LANDS {
        datums = datums
            + centered_cylinder(
                format!("closed_reservoir_stratification_service_tool_datum_land_{i}"),
                9.0,
                8.0,
                32,
            )
            .translate(tool_datum_x(i), 0.0, TOOL_DATUM_Z / 2.0 + 4.0);
    }

    rail + datums
}

fn recirculation_span_tubes() -> Part {
    let inlet_to_nest = centered_cylinder(
        "closed_reservoir_stratification_inlet_loop_to_reservoir_span",
        LOOP_TUBE_D / 2.0,
        324.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(18.0, INLET_LOOP_POS.1, DECK_Z + LOOP_PANEL_Z + 24.0);
    let outlet_from_nest = centered_cylinder(
        "closed_reservoir_stratification_outlet_reservoir_to_loop_span",
        LOOP_TUBE_D / 2.0,
        324.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(18.0, OUTLET_LOOP_POS.1, DECK_Z + LOOP_PANEL_Z + 24.0);
    let flow_dock_drop = centered_cylinder(
        "closed_reservoir_stratification_loop_to_flow_coupon_drop",
        LOOP_TUBE_D / 2.0,
        166.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FLOW_DOCK_POS.0, -10.0, DECK_Z + FLOW_DOCK_Z + 18.0);

    inlet_to_nest + outlet_from_nest + flow_dock_drop
}

fn primary_footprints() -> [Footprint; 8] {
    [
        Footprint {
            name: "load_cell_pad_and_reservoir_nest",
            center: LOAD_CELL_PAD_POS,
            x: LOAD_CELL_PAD_X,
            y: LOAD_CELL_PAD_Y,
        },
        Footprint {
            name: "temperature_probe_well_stack",
            center: PROBE_STACK_POS,
            x: PROBE_STACK_X,
            y: PROBE_STACK_Y,
        },
        Footprint {
            name: "recirculation_inlet_witness_loop",
            center: INLET_LOOP_POS,
            x: LOOP_PANEL_X,
            y: LOOP_PANEL_Y,
        },
        Footprint {
            name: "recirculation_outlet_witness_loop",
            center: OUTLET_LOOP_POS,
            x: LOOP_PANEL_X,
            y: LOOP_PANEL_Y,
        },
        Footprint {
            name: "flow_sensor_coupon_dock",
            center: FLOW_DOCK_POS,
            x: FLOW_DOCK_X,
            y: FLOW_DOCK_Y,
        },
        Footprint {
            name: "dead_zone_dye_witness_pockets",
            center: DYE_POCKET_POS,
            x: DYE_POCKET_PANEL_X,
            y: DYE_POCKET_PANEL_Y,
        },
        Footprint {
            name: "barcode_status_lands",
            center: STATUS_PANEL_POS,
            x: STATUS_PANEL_X,
            y: STATUS_PANEL_Y,
        },
        Footprint {
            name: "service_tool_datum_lands",
            center: TOOL_DATUM_POS,
            x: TOOL_DATUM_X,
            y: TOOL_DATUM_Y,
        },
    ]
}

fn assert_layout() {
    let footprints = primary_footprints();
    for footprint in footprints {
        assert!(
            fits_on_deck(footprint.center, footprint.x, footprint.y, 12.0),
            "{} must fit inside the leak tray",
            footprint.name
        );
    }

    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            let a = footprints[left];
            let b = footprints[right];
            assert!(
                !rects_overlap(a, b),
                "{} must not overlap {}",
                a.name,
                b.name
            );
        }
    }
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

fn rects_overlap(a: Footprint, b: Footprint) -> bool {
    let ax0 = a.center.0 - a.x / 2.0;
    let ax1 = a.center.0 + a.x / 2.0;
    let ay0 = a.center.1 - a.y / 2.0;
    let ay1 = a.center.1 + a.y / 2.0;
    let bx0 = b.center.0 - b.x / 2.0;
    let bx1 = b.center.0 + b.x / 2.0;
    let by0 = b.center.1 - b.y / 2.0;
    let by1 = b.center.1 + b.y / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 38.0), -(DECK_Y / 2.0 - 38.0)),
        (DECK_X / 2.0 - 38.0, -(DECK_Y / 2.0 - 38.0)),
        (-(DECK_X / 2.0 - 38.0), DECK_Y / 2.0 - 38.0),
        (DECK_X / 2.0 - 38.0, DECK_Y / 2.0 - 38.0),
        (0.0, -(DECK_Y / 2.0 - 38.0)),
        (0.0, DECK_Y / 2.0 - 38.0),
        (-(DECK_X / 2.0 - 38.0), 0.0),
        (DECK_X / 2.0 - 38.0, 0.0),
    ]
}

fn dye_pocket_xy(index: usize) -> (f64, f64) {
    let col = index % 4;
    let row = index / 4;
    let x = -150.0 + col as f64 * 100.0;
    let y = -28.0 + row as f64 * 56.0;
    (x, y)
}

fn barcode_y(index: usize) -> f64 {
    -32.5 + index as f64 * 13.0
}

fn status_y(index: usize) -> f64 {
    -27.0 + index as f64 * 18.0
}

fn tool_datum_x(index: usize) -> f64 {
    -96.0 + index as f64 * 48.0
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let envelope = centered_cube(format!("{name}_envelope"), x, y, z);
    let hollow = centered_cube(format!("{name}_hollow"), x - 18.0, y - 18.0, z - 18.0);
    envelope - hollow
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_media_reservoir_temperature_stratification_recirculation_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_reservoir_validation_intent() {
        for feature in [
            "leak_condensate_tray",
            "insulated_bag_bottle_nest",
            "top_temperature_probe_well",
            "middle_temperature_probe_well",
            "bottom_temperature_probe_well",
            "recirculation_inlet_witness_loop",
            "recirculation_outlet_witness_loop",
            "mixing_baffle_surrogate",
            "flow_sensor_coupon_dock",
            "dead_zone_dye_witness_pockets",
            "load_cell_pad_placeholder",
            "barcode_status_lands",
            "robot_keepout",
            "service_keepout",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 14);
    }

    #[test]
    fn reservoir_nest_and_probe_levels_are_defined() {
        assert!(NEST_X >= RESERVOIR_X + INSULATION_WALL * 2.0);
        assert!(NEST_Y >= RESERVOIR_Y + INSULATION_WALL * 2.0);
        assert_eq!(PROBE_WELL_COUNT, 3);
        assert_eq!(PROBE_STRIKE_LANDS, PROBE_WELL_COUNT);
        assert!(PROBE_LEVEL_Z[0] < PROBE_LEVEL_Z[1]);
        assert!(PROBE_LEVEL_Z[1] < PROBE_LEVEL_Z[2]);
        assert!(PROBE_LEVEL_Z[2] < PROBE_STACK_Z - 24.0);
    }

    #[test]
    fn recirculation_and_flow_witness_counts_are_explicit() {
        assert_eq!(LOOP_CLIPS_PER_PANEL * 2, 8);
        assert_eq!(LOOP_SEPTUM_PORTS_PER_PANEL * 2, 4);
        assert_eq!(FLOW_COUPON_SLOTS, 3);
        assert!(FLOW_COUPON_CHANNEL_D > LOOP_TUBE_D);
        assert!(LOOP_WINDOW_X > 100.0);
        assert!(INLET_LOOP_POS.1 > OUTLET_LOOP_POS.1);
    }

    #[test]
    fn dye_baffle_and_load_cell_surrogates_are_counted() {
        assert_eq!(DYE_POCKET_COUNT, 8);
        assert_eq!(BAFFLE_PLATES, 5);
        assert!(BAFFLE_FLOW_GAP >= 20.0);
        assert_eq!(LOAD_CELL_BOSSES, 4);
        assert!(LOAD_CELL_PAD_X > NEST_X);
        assert!(LOAD_CELL_PAD_Y > NEST_Y);
    }

    #[test]
    fn evidence_lands_and_service_datums_are_present() {
        assert_eq!(BARCODE_LANDS, 6);
        assert_eq!(STATUS_LANDS, 4);
        assert_eq!(RFID_LANDS, 2);
        assert_eq!(TOOL_DATUM_LANDS, 5);
        assert!(STATUS_PANEL_X > BARCODE_LANDS as f64 * 50.0);
        assert!(tool_datum_x(0) < tool_datum_x(TOOL_DATUM_LANDS - 1));
    }

    #[test]
    fn station_fits_defined_benchtop_envelope() {
        assert!(DECK_X <= 1100.0);
        assert!(DECK_Y <= 760.0);
        assert!(TRAY_RIM_Z >= 34.0);
        assert!(CONDENSATE_CHANNEL_DEPTH >= 6.0);
        assert_eq!(mount_points().len(), 8);
        assert_layout();
    }

    #[test]
    fn primary_modules_stay_inside_tray_margin() {
        for footprint in primary_footprints() {
            assert!(
                fits_on_deck(footprint.center, footprint.x, footprint.y, 12.0),
                "{} should stay inside the tray",
                footprint.name
            );
        }
    }

    #[test]
    fn keepouts_cover_robot_and_service_access() {
        assert_eq!(KEEP_OUT_ZONES, 4);
        assert!(ROBOT_KEEPOUT_X >= 700.0);
        assert!(ROBOT_KEEPOUT_Y >= 100.0);
        assert!(SERVICE_KEEPOUT_Y >= 400.0);
        assert!(TOP_PROBE_SERVICE_Z > PROBE_STACK_Z + 50.0);
    }

    #[test]
    fn pocket_arrays_remain_inside_their_panels() {
        let (first_x, first_y) = dye_pocket_xy(0);
        let (last_x, last_y) = dye_pocket_xy(DYE_POCKET_COUNT - 1);
        assert!(first_x.abs() + DYE_POCKET_D < DYE_POCKET_PANEL_X / 2.0);
        assert!(last_x.abs() + DYE_POCKET_D < DYE_POCKET_PANEL_X / 2.0);
        assert!(first_y.abs() + DYE_POCKET_D < DYE_POCKET_PANEL_Y / 2.0);
        assert!(last_y.abs() + DYE_POCKET_D < DYE_POCKET_PANEL_Y / 2.0);
        assert!(barcode_y(0).abs() + 8.0 < STATUS_PANEL_Y / 2.0);
        assert!(barcode_y(BARCODE_LANDS - 1).abs() + 8.0 < STATUS_PANEL_Y / 2.0);
    }
}
