use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/reagent quarantine and staged-release pod.
//
// This module packages incoming media and reagent lots outside the sterile
// workcell. It models receiving, segregated quarantine/release storage,
// temperature-zone placeholders, lot scanning, QC pass-through drawers, thermal
// buffers, spill/waste capture, environmental clearance placeholders, and
// service keepouts. It is an envelope and interface CAD model, not a substitute
// for cold-chain qualification, pressure testing, or biological release SOPs.

const OUTPUTS: &[&str] = &[
    "output/media_reagent_quarantine_pod_shell.stl",
    "output/media_reagent_quarantine_pod_incoming_shelf.stl",
    "output/media_reagent_quarantine_pod_temperature_zones.stl",
    "output/media_reagent_quarantine_pod_segregation_bays.stl",
    "output/media_reagent_quarantine_pod_barcode_qc_station.stl",
    "output/media_reagent_quarantine_pod_sampling_drawers.stl",
    "output/media_reagent_quarantine_pod_thermal_buffers.stl",
    "output/media_reagent_quarantine_pod_spill_waste_capture.stl",
    "output/media_reagent_quarantine_pod_pressure_hepa_vhp.stl",
    "output/media_reagent_quarantine_pod_service_keepouts.stl",
    "output/media_reagent_quarantine_pod_assembly.stl",
];

const POD_X: f64 = 1240.0;
const POD_Y: f64 = 760.0;
const POD_Z: f64 = 1380.0;
const WALL_T: f64 = 30.0;
const BASE_PAN_Z: f64 = 86.0;
const ROOF_PLENUM_Z: f64 = 124.0;
const INNER_X: f64 = POD_X - 2.0 * WALL_T;
const INNER_Y: f64 = POD_Y - 2.0 * WALL_T;
const INNER_Z: f64 = POD_Z - BASE_PAN_Z - ROOF_PLENUM_Z;

const DOOR_T: f64 = 24.0;
const DOOR_GASKET_T: f64 = 8.0;
const FRONT_SERVICE_CLEARANCE: f64 = 760.0;
const REAR_SERVICE_CLEARANCE: f64 = 560.0;
const SIDE_SERVICE_CLEARANCE: f64 = 420.0;

const INCOMING_SHELF_X: f64 = 980.0;
const INCOMING_SHELF_Y: f64 = 248.0;
const INCOMING_SHELF_Z: f64 = 32.0;
const INCOMING_CARTON_LANES: usize = 4;
const INCOMING_CARTONS_PER_LANE: usize = 2;

const QUARANTINE_BAY_X: f64 = 520.0;
const RELEASED_BAY_X: f64 = 430.0;
const SEGREGATION_DIVIDER_W: f64 = 32.0;
const SEGREGATION_AIR_GAP: f64 = 46.0;
const QUARANTINE_BIN_COUNT: usize = 8;
const RELEASED_BIN_COUNT: usize = 6;
const BOTTLES_PER_BIN: usize = 2;
const BIN_X: f64 = 116.0;
const BIN_Y: f64 = 132.0;
const BIN_Z: f64 = 92.0;

const TEMP_ZONE_COUNT: usize = 3;
const TEMP_ZONE_X: f64 = 304.0;
const TEMP_ZONE_Y: f64 = 190.0;
const TEMP_ZONE_Z: f64 = 142.0;
const TEMP_ZONE_PITCH_X: f64 = 338.0;

const BARCODE_STATION_X: f64 = 310.0;
const BARCODE_STATION_Y: f64 = 210.0;
const BARCODE_STATION_Z: f64 = 246.0;
const QC_DRAWER_COUNT: usize = 2;
const DRAWER_X: f64 = 398.0;
const DRAWER_Y: f64 = 286.0;
const DRAWER_Z: f64 = 58.0;
const QC_WELLS_PER_DRAWER: usize = 12;

const COLD_PACK_COUNT: usize = 8;
const THERMAL_BUFFER_COUNT: usize = 6;
const PCM_PACK_X: f64 = 132.0;
const PCM_PACK_Y: f64 = 82.0;
const PCM_PACK_Z: f64 = 26.0;

const SPILL_TRAY_X: f64 = 1080.0;
const SPILL_TRAY_Y: f64 = 620.0;
const SPILL_TRAY_Z: f64 = 52.0;
const SPILL_TRAY_CURB: f64 = 28.0;
const WASTE_BOTTLE_COUNT: usize = 2;
const WASTE_BOTTLE_D: f64 = 86.0;
const DRAIN_PORT_D: f64 = 24.0;

const HEPA_FILTER_X: f64 = 360.0;
const HEPA_FILTER_Y: f64 = 220.0;
const HEPA_FILTER_Z: f64 = 44.0;
const VHP_CLEARANCE_X: f64 = 470.0;
const VHP_CLEARANCE_Y: f64 = 178.0;
const VHP_CLEARANCE_Z: f64 = 318.0;
const PRESSURE_PANEL_X: f64 = 250.0;
const PRESSURE_PANEL_Z: f64 = 166.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_clearances();

    let shell = closed_pod_shell();
    export(OUTPUTS[0], &shell);

    let incoming = incoming_receiving_shelf();
    export(OUTPUTS[1], &incoming);

    let temp_zones = temperature_zone_placeholders();
    export(OUTPUTS[2], &temp_zones);

    let segregation = quarantine_release_segregation();
    export(OUTPUTS[3], &segregation);

    let scan_station = barcode_lot_qc_station();
    export(OUTPUTS[4], &scan_station);

    let drawers = sampling_qc_pass_through_drawers();
    export(OUTPUTS[5], &drawers);

    let thermal = cold_pack_thermal_buffer_placeholders();
    export(OUTPUTS[6], &thermal);

    let spill = spill_tray_drain_waste_capture();
    export(OUTPUTS[7], &spill);

    let pressure = pressure_hepa_vhp_clearance_placeholders();
    export(OUTPUTS[8], &pressure);

    let keepouts = service_keepout_envelopes();
    export(OUTPUTS[9], &keepouts);

    let assembly = shell
        + incoming
        + temp_zones
        + segregation
        + scan_station
        + drawers
        + thermal
        + spill
        + pressure
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Media/reagent quarantine pod:");
    println!("  Closed cabinet envelope:    {POD_X:.0}mm W x {POD_Y:.0}mm D x {POD_Z:.0}mm H");
    println!(
        "  Segregated storage:         {QUARANTINE_BAY_X:.0}mm quarantine bay, {RELEASED_BAY_X:.0}mm released bay, {SEGREGATION_AIR_GAP:.0}mm segregation gap"
    );
    println!(
        "  Lot capacity placeholder:   {} quarantine + {} released bin positions, {} bottle/carton equivalents",
        QUARANTINE_BIN_COUNT,
        RELEASED_BIN_COUNT,
        total_lot_capacity()
    );
    println!(
        "  Temperature placeholders:   {TEMP_ZONE_COUNT} zones at {TEMP_ZONE_X:.0}mm x {TEMP_ZONE_Y:.0}mm x {TEMP_ZONE_Z:.0}mm each"
    );
    println!(
        "  Sampling/QC drawers:        {QC_DRAWER_COUNT} drawers, {QC_WELLS_PER_DRAWER} aliquot wells each"
    );
    println!(
        "  Spill containment:          {SPILL_TRAY_X:.0}mm x {SPILL_TRAY_Y:.0}mm tray, {DRAIN_PORT_D:.0}mm drain, {WASTE_BOTTLE_COUNT} waste bottle placeholders"
    );
    println!(
        "  Service keepouts:           front {FRONT_SERVICE_CLEARANCE:.0}mm, rear {REAR_SERVICE_CLEARANCE:.0}mm, sides {SIDE_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_clearances() {
    assert!(
        segregation_span_x() < INNER_X - 96.0,
        "quarantine and released bays do not fit within closed pod width"
    );
    assert!(
        incoming_shelf_front_edge() >= POD_Y / 2.0 + 220.0,
        "incoming shelf does not project outside the sterile-workcell side"
    );
    assert!(
        temp_zone_span_x() < INNER_X - 140.0,
        "temperature zone placeholders exceed available width"
    );
    assert!(
        DRAWER_Y + 92.0 < INNER_Y,
        "sampling drawers collide with rear storage zone"
    );
}

fn closed_pod_shell() -> Part {
    let outer = centered_cube(
        "media_reagent_quarantine_pod_closed_outer_shell",
        POD_X,
        POD_Y,
        POD_Z,
    )
    .translate(0.0, 0.0, POD_Z / 2.0);
    let inner = centered_cube(
        "media_reagent_quarantine_pod_inner_service_volume",
        INNER_X,
        INNER_Y,
        INNER_Z,
    )
    .translate(0.0, 0.0, BASE_PAN_Z + INNER_Z / 2.0);

    let front_upper_cut = centered_cube(
        "media_reagent_quarantine_pod_front_upper_access_cut",
        POD_X - 160.0,
        WALL_T + 8.0,
        730.0,
    )
    .translate(0.0, -POD_Y / 2.0, 690.0);
    let front_drawer_cut = centered_cube(
        "media_reagent_quarantine_pod_front_qc_drawer_cut",
        910.0,
        WALL_T + 8.0,
        190.0,
    )
    .translate(0.0, -POD_Y / 2.0, 244.0);
    let rear_service_cut = centered_cube(
        "media_reagent_quarantine_pod_rear_service_panel_cut",
        POD_X - 220.0,
        WALL_T + 8.0,
        410.0,
    )
    .translate(0.0, POD_Y / 2.0, 716.0);
    let drain_bulkhead_cut = centered_cylinder(
        "media_reagent_quarantine_pod_front_drain_bulkhead_cut",
        DRAIN_PORT_D / 2.0,
        WALL_T + 24.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(POD_X / 2.0 - 152.0, -POD_Y / 2.0, BASE_PAN_Z - 24.0);

    let shell =
        outer - inner - front_upper_cut - front_drawer_cut - rear_service_cut - drain_bulkhead_cut;

    shell
        + front_double_doors()
        + rear_service_panel()
        + roof_lift_handles()
        + base_leveling_feet()
        + gasket_frame_xz(
            "media_reagent_quarantine_front_access_gasket",
            POD_X - 132.0,
            DOOR_GASKET_T,
            790.0,
            22.0,
        )
        .translate(0.0, -POD_Y / 2.0 - 6.0, 690.0)
        + gasket_frame_xz(
            "media_reagent_quarantine_drawer_gasket",
            944.0,
            DOOR_GASKET_T,
            226.0,
            18.0,
        )
        .translate(0.0, -POD_Y / 2.0 - 7.0, 244.0)
        + gasket_frame_xz(
            "media_reagent_quarantine_rear_service_gasket",
            POD_X - 190.0,
            DOOR_GASKET_T,
            454.0,
            18.0,
        )
        .translate(0.0, POD_Y / 2.0 + 7.0, 716.0)
}

fn front_double_doors() -> Part {
    let left = door_leaf("quarantine", -265.0, 604.0);
    let right = door_leaf("released", 265.0, 604.0);
    let center_latch = centered_cube(
        "media_reagent_quarantine_center_double_door_latch_spine",
        34.0,
        28.0,
        760.0,
    )
    .translate(0.0, -POD_Y / 2.0 - DOOR_T / 2.0 - 3.0, 690.0);

    left + right + center_latch + door_interlock_blocks()
}

fn door_leaf(name: &str, x: f64, z: f64) -> Part {
    let door = centered_cube(
        format!("media_reagent_{name}_front_door_leaf"),
        514.0,
        DOOR_T,
        760.0,
    )
    .translate(x, -POD_Y / 2.0 - DOOR_T / 2.0, z);
    let view_cut = centered_cube(
        format!("media_reagent_{name}_lot_view_window_cut"),
        310.0,
        DOOR_T + 4.0,
        210.0,
    )
    .translate(x, -POD_Y / 2.0 - DOOR_T / 2.0, z + 72.0);
    let window_frame = gasket_frame_xz(
        format!("media_reagent_{name}_view_window_frame"),
        350.0,
        8.0,
        250.0,
        16.0,
    )
    .translate(x, -POD_Y / 2.0 - DOOR_T - 8.0, z + 72.0);
    let pull = centered_cube(
        format!("media_reagent_{name}_door_vertical_pull"),
        24.0,
        38.0,
        210.0,
    )
    .translate(
        x + if x < 0.0 { 205.0 } else { -205.0 },
        -POD_Y / 2.0 - DOOR_T - 18.0,
        z - 52.0,
    );
    let status_flag = centered_cube(
        format!("media_reagent_{name}_release_status_flag_placeholder"),
        96.0,
        10.0,
        34.0,
    )
    .translate(x, -POD_Y / 2.0 - DOOR_T - 8.0, z + 310.0);

    door - view_cut + window_frame + pull + status_flag
}

fn door_interlock_blocks() -> Part {
    let mut blocks = Part::empty("media_reagent_front_door_interlocks");
    for (i, (x, z)) in [
        (-540.0, 944.0),
        (-540.0, 476.0),
        (540.0, 944.0),
        (540.0, 476.0),
    ]
    .iter()
    .enumerate()
    {
        let block = centered_cube(
            format!("media_reagent_front_interlock_keeper_{i}"),
            52.0,
            26.0,
            42.0,
        )
        .translate(*x, -POD_Y / 2.0 - DOOR_T - 8.0, *z);
        let pin = centered_cylinder(
            format!("media_reagent_front_interlock_pin_hole_{i}"),
            5.4 / 2.0,
            32.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -POD_Y / 2.0 - DOOR_T - 8.0, *z);
        blocks = blocks + (block - pin);
    }
    blocks
}

fn rear_service_panel() -> Part {
    let panel = centered_cube(
        "media_reagent_rear_service_lift_off_panel",
        POD_X - 210.0,
        18.0,
        432.0,
    )
    .translate(0.0, POD_Y / 2.0 + 18.0, 716.0);
    let cable_gland_row = cable_gland_row("rear").translate(-236.0, POD_Y / 2.0 + 30.0, 596.0);
    let drain_label_strip =
        centered_cube("media_reagent_rear_bulkhead_label_strip", 430.0, 6.0, 34.0).translate(
            238.0,
            POD_Y / 2.0 + 30.0,
            550.0,
        );
    panel + cable_gland_row + drain_label_strip
}

fn roof_lift_handles() -> Part {
    let left = centered_cube(
        "media_reagent_roof_left_recessed_lift_handle",
        210.0,
        28.0,
        26.0,
    )
    .translate(-260.0, -POD_Y / 2.0 + 82.0, POD_Z + 12.0);
    let right = centered_cube(
        "media_reagent_roof_right_recessed_lift_handle",
        210.0,
        28.0,
        26.0,
    )
    .translate(260.0, -POD_Y / 2.0 + 82.0, POD_Z + 12.0);
    left + right
}

fn base_leveling_feet() -> Part {
    let mut feet = Part::empty("media_reagent_quarantine_leveling_feet");
    for (i, (x, y)) in [
        (-(POD_X / 2.0 - 86.0), -(POD_Y / 2.0 - 78.0)),
        (POD_X / 2.0 - 86.0, -(POD_Y / 2.0 - 78.0)),
        (-(POD_X / 2.0 - 86.0), POD_Y / 2.0 - 78.0),
        (POD_X / 2.0 - 86.0, POD_Y / 2.0 - 78.0),
        (0.0, -(POD_Y / 2.0 - 78.0)),
        (0.0, POD_Y / 2.0 - 78.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cube(
            format!("media_reagent_quarantine_leveling_foot_pad_{i}"),
            84.0,
            84.0,
            14.0,
        )
        .translate(*x, *y, -7.0);
        let stem = centered_cylinder(
            format!("media_reagent_quarantine_leveling_foot_stem_clearance_{i}"),
            12.0 / 2.0,
            20.0,
            28,
        )
        .translate(*x, *y, -6.0);
        feet = feet + (pad - stem);
    }
    feet
}

fn incoming_receiving_shelf() -> Part {
    let y = -incoming_shelf_front_edge() + INCOMING_SHELF_Y / 2.0;
    let z = 1016.0;
    let deck = centered_cube(
        "media_reagent_incoming_receiving_shelf_deck",
        INCOMING_SHELF_X,
        INCOMING_SHELF_Y,
        INCOMING_SHELF_Z,
    )
    .translate(0.0, y, z);
    let basin_cut = centered_cube(
        "media_reagent_incoming_shelf_recessed_wipeable_basin",
        INCOMING_SHELF_X - 76.0,
        INCOMING_SHELF_Y - 58.0,
        INCOMING_SHELF_Z + 4.0,
    )
    .translate(0.0, y + 8.0, z + 8.0);
    let front_lip = centered_cube(
        "media_reagent_incoming_shelf_front_retaining_lip",
        INCOMING_SHELF_X,
        18.0,
        58.0,
    )
    .translate(0.0, y - INCOMING_SHELF_Y / 2.0 + 9.0, z + 36.0);
    let rear_datum = centered_cube(
        "media_reagent_incoming_shelf_rear_datum_fence",
        INCOMING_SHELF_X - 72.0,
        22.0,
        92.0,
    )
    .translate(0.0, -POD_Y / 2.0 - 24.0, z + 54.0);
    let transfer_slot = centered_cube(
        "media_reagent_incoming_shelf_closed_transfer_slot_placeholder",
        690.0,
        18.0,
        96.0,
    )
    .translate(0.0, -POD_Y / 2.0 - 10.0, z + 74.0);

    let mut lane_rails = Part::empty("media_reagent_incoming_lane_rails");
    let mut carton_recesses = Part::empty("media_reagent_incoming_carton_recesses");
    for lane in 0..INCOMING_CARTON_LANES {
        let x = incoming_lane_x(lane);
        lane_rails = lane_rails
            + centered_cube(
                format!("media_reagent_incoming_lane_{lane}_left_rail"),
                8.0,
                INCOMING_SHELF_Y - 72.0,
                22.0,
            )
            .translate(x - 48.0, y + 10.0, z + INCOMING_SHELF_Z / 2.0 + 11.0)
            + centered_cube(
                format!("media_reagent_incoming_lane_{lane}_right_rail"),
                8.0,
                INCOMING_SHELF_Y - 72.0,
                22.0,
            )
            .translate(x + 48.0, y + 10.0, z + INCOMING_SHELF_Z / 2.0 + 11.0);

        for carton in 0..INCOMING_CARTONS_PER_LANE {
            let carton_y = y - 34.0 + carton as f64 * 82.0;
            carton_recesses = carton_recesses
                + centered_cube(
                    format!("media_reagent_incoming_lane_{lane}_carton_{carton}_land"),
                    78.0,
                    56.0,
                    8.0,
                )
                .translate(x, carton_y, z + INCOMING_SHELF_Z / 2.0 + 3.0);
        }
    }

    let lot_clip = centered_cube(
        "media_reagent_incoming_lot_paperwork_clip",
        150.0,
        18.0,
        44.0,
    )
    .translate(
        INCOMING_SHELF_X / 2.0 - 116.0,
        y + INCOMING_SHELF_Y / 2.0 - 34.0,
        z + 48.0,
    );

    deck - basin_cut - carton_recesses
        + front_lip
        + rear_datum
        + transfer_slot
        + lane_rails
        + lot_clip
}

fn temperature_zone_placeholders() -> Part {
    let mut zones = Part::empty("media_reagent_temperature_zone_placeholders");
    for zone in 0..TEMP_ZONE_COUNT {
        let name = match zone {
            0 => "cold_2_to_8c",
            1 => "controlled_room_temp",
            _ => "release_hold",
        };
        zones = zones + temperature_zone_module(name, temp_zone_x(zone), 152.0, 1030.0);
    }
    zones
}

fn temperature_zone_module(name: &str, x: f64, y: f64, z: f64) -> Part {
    let shell = centered_cube(
        format!("media_reagent_temp_zone_{name}_insulated_shell"),
        TEMP_ZONE_X,
        TEMP_ZONE_Y,
        TEMP_ZONE_Z,
    )
    .translate(x, y, z);
    let pocket = centered_cube(
        format!("media_reagent_temp_zone_{name}_payload_clearance"),
        TEMP_ZONE_X - 56.0,
        TEMP_ZONE_Y - 42.0,
        TEMP_ZONE_Z - 42.0,
    )
    .translate(x, y - 4.0, z + 6.0);
    let door = centered_cube(
        format!("media_reagent_temp_zone_{name}_front_latched_lid"),
        TEMP_ZONE_X - 34.0,
        16.0,
        TEMP_ZONE_Z - 28.0,
    )
    .translate(x, y - TEMP_ZONE_Y / 2.0 - 14.0, z);
    let sensor_boss = centered_cylinder(
        format!("media_reagent_temp_zone_{name}_rtd_probe_boss"),
        11.0,
        22.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        x + TEMP_ZONE_X / 2.0 - 34.0,
        y - TEMP_ZONE_Y / 2.0 - 23.0,
        z + 42.0,
    );
    let label_plate = centered_cube(
        format!("media_reagent_temp_zone_{name}_status_label_plate"),
        136.0,
        6.0,
        28.0,
    )
    .translate(
        x,
        y - TEMP_ZONE_Y / 2.0 - 25.0,
        z + TEMP_ZONE_Z / 2.0 - 28.0,
    );

    shell - pocket + door + sensor_boss + label_plate + zone_slide_rails(name, x, y, z)
}

fn zone_slide_rails(name: &str, x: f64, y: f64, z: f64) -> Part {
    let left = centered_cube(
        format!("media_reagent_temp_zone_{name}_left_slide_rail"),
        16.0,
        TEMP_ZONE_Y + 42.0,
        18.0,
    )
    .translate(
        x - TEMP_ZONE_X / 2.0 - 12.0,
        y,
        z - TEMP_ZONE_Z / 2.0 + 16.0,
    );
    let right = centered_cube(
        format!("media_reagent_temp_zone_{name}_right_slide_rail"),
        16.0,
        TEMP_ZONE_Y + 42.0,
        18.0,
    )
    .translate(
        x + TEMP_ZONE_X / 2.0 + 12.0,
        y,
        z - TEMP_ZONE_Z / 2.0 + 16.0,
    );
    left + right
}

fn quarantine_release_segregation() -> Part {
    let barrier = centered_cube(
        "media_reagent_quarantine_released_hard_segregation_barrier",
        SEGREGATION_DIVIDER_W,
        INNER_Y - 92.0,
        870.0,
    )
    .translate(0.0, 58.0, 650.0);
    let air_gap_marker = centered_cube(
        "media_reagent_quarantine_released_no_cross_stage_air_gap_marker",
        SEGREGATION_AIR_GAP,
        INNER_Y - 120.0,
        18.0,
    )
    .translate(0.0, 54.0, 1116.0);
    let transfer_gate = centered_cube(
        "media_reagent_staged_release_locked_transfer_gate",
        206.0,
        22.0,
        188.0,
    )
    .translate(0.0, -136.0, 706.0);
    let gate_window = centered_cube(
        "media_reagent_staged_release_gate_window_cut",
        148.0,
        26.0,
        92.0,
    )
    .translate(0.0, -136.0, 714.0);
    let quarantine_rack = storage_bin_rack(
        "quarantine",
        -SEGREGATION_DIVIDER_W / 2.0 - SEGREGATION_AIR_GAP / 2.0 - QUARANTINE_BAY_X / 2.0,
        QUARANTINE_BAY_X,
        QUARANTINE_BIN_COUNT,
    );
    let released_rack = storage_bin_rack(
        "released",
        SEGREGATION_DIVIDER_W / 2.0 + SEGREGATION_AIR_GAP / 2.0 + RELEASED_BAY_X / 2.0,
        RELEASED_BAY_X,
        RELEASED_BIN_COUNT,
    );
    let lockout_bars = release_lockout_bars();

    barrier
        + air_gap_marker
        + (transfer_gate - gate_window)
        + quarantine_rack
        + released_rack
        + lockout_bars
}

fn storage_bin_rack(name: &str, x_center: f64, rack_x: f64, bin_count: usize) -> Part {
    let rack_back = centered_cube(
        format!("media_reagent_{name}_bay_rear_backplane"),
        rack_x,
        24.0,
        690.0,
    )
    .translate(x_center, POD_Y / 2.0 - WALL_T - 40.0, 618.0);
    let left_post = centered_cube(
        format!("media_reagent_{name}_bay_left_post"),
        22.0,
        INNER_Y - 126.0,
        720.0,
    )
    .translate(x_center - rack_x / 2.0 + 18.0, 52.0, 630.0);
    let right_post = centered_cube(
        format!("media_reagent_{name}_bay_right_post"),
        22.0,
        INNER_Y - 126.0,
        720.0,
    )
    .translate(x_center + rack_x / 2.0 - 18.0, 52.0, 630.0);

    let mut shelves = Part::empty(format!("media_reagent_{name}_bay_shelves"));
    for shelf in 0..3 {
        let z = 370.0 + shelf as f64 * 190.0;
        shelves = shelves
            + centered_cube(
                format!("media_reagent_{name}_bay_shelf_{shelf}"),
                rack_x - 46.0,
                INNER_Y - 150.0,
                18.0,
            )
            .translate(x_center, 44.0, z)
            + centered_cube(
                format!("media_reagent_{name}_bay_front_retainer_{shelf}"),
                rack_x - 66.0,
                14.0,
                44.0,
            )
            .translate(x_center, -244.0, z + 24.0);
    }

    let mut bins = Part::empty(format!("media_reagent_{name}_lot_bins"));
    for bin in 0..bin_count {
        let col = bin % 2;
        let row = bin / 2;
        let x = x_center + (col as f64 - 0.5) * (BIN_X + 34.0);
        let y = -74.0 + (row % 2) as f64 * 174.0;
        let z = 414.0 + (row / 2) as f64 * 190.0;
        bins = bins + lot_bin(name, bin, x, y, z);
    }

    rack_back + left_post + right_post + shelves + bins
}

fn lot_bin(name: &str, bin: usize, x: f64, y: f64, z: f64) -> Part {
    let outer = centered_cube(
        format!("media_reagent_{name}_lot_bin_{bin}_outer"),
        BIN_X,
        BIN_Y,
        BIN_Z,
    )
    .translate(x, y, z);
    let pocket = centered_cube(
        format!("media_reagent_{name}_lot_bin_{bin}_payload_pocket"),
        BIN_X - 22.0,
        BIN_Y - 20.0,
        BIN_Z - 22.0,
    )
    .translate(x, y, z + 8.0);
    let front_label = centered_cube(
        format!("media_reagent_{name}_lot_bin_{bin}_front_status_label"),
        72.0,
        7.0,
        20.0,
    )
    .translate(x, y - BIN_Y / 2.0 - 4.0, z + BIN_Z / 2.0 - 18.0);
    let two_bottle_divider = centered_cube(
        format!("media_reagent_{name}_lot_bin_{bin}_two_bottle_divider"),
        8.0,
        BIN_Y - 30.0,
        44.0,
    )
    .translate(x, y, z + 5.0);
    outer - pocket + front_label + two_bottle_divider
}

fn release_lockout_bars() -> Part {
    let mut bars = Part::empty("media_reagent_release_lockout_bars");
    for (i, z) in [460.0, 650.0, 840.0].iter().enumerate() {
        bars =
            bars + centered_cube(
                format!("media_reagent_quarantine_only_lockout_bar_{i}"),
                QUARANTINE_BAY_X - 84.0,
                20.0,
                34.0,
            )
            .translate(
                -SEGREGATION_DIVIDER_W / 2.0 - SEGREGATION_AIR_GAP / 2.0 - QUARANTINE_BAY_X / 2.0,
                -276.0,
                *z,
            ) + centered_cube(
                format!("media_reagent_released_lot_tamper_seal_bar_{i}"),
                RELEASED_BAY_X - 82.0,
                18.0,
                28.0,
            )
            .translate(
                SEGREGATION_DIVIDER_W / 2.0 + SEGREGATION_AIR_GAP / 2.0 + RELEASED_BAY_X / 2.0,
                -276.0,
                *z,
            );
    }
    bars
}

fn barcode_lot_qc_station() -> Part {
    let x = -POD_X / 2.0 + 205.0;
    let y = -POD_Y / 2.0 - 148.0;
    let z = 700.0;
    let pedestal = centered_cube(
        "media_reagent_barcode_lot_scan_station_pedestal",
        BARCODE_STATION_X,
        BARCODE_STATION_Y,
        44.0,
    )
    .translate(x, y, z);
    let work_pad = centered_cube(
        "media_reagent_barcode_station_wipeable_scan_pad",
        BARCODE_STATION_X - 54.0,
        BARCODE_STATION_Y - 52.0,
        8.0,
    )
    .translate(x, y, z + 28.0);
    let scanner_stem = centered_cube(
        "media_reagent_barcode_station_scanner_stem",
        30.0,
        34.0,
        BARCODE_STATION_Z - 84.0,
    )
    .translate(x - 92.0, y + 54.0, z + 108.0);
    let scanner_head = centered_cube(
        "media_reagent_barcode_station_scanner_head",
        112.0,
        56.0,
        42.0,
    )
    .rotate(0.0, 0.0, -12.0)
    .translate(x - 56.0, y + 28.0, z + BARCODE_STATION_Z - 48.0);
    let lens = centered_cylinder(
        "media_reagent_barcode_station_scanner_lens_placeholder",
        14.0,
        8.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x - 22.0, y - 4.0, z + BARCODE_STATION_Z - 48.0);
    let lot_screen = centered_cube(
        "media_reagent_lot_scan_status_screen_placeholder",
        156.0,
        18.0,
        96.0,
    )
    .rotate(-9.0, 0.0, 0.0)
    .translate(x + 78.0, y + 74.0, z + BARCODE_STATION_Z - 128.0);
    let label_printer = centered_cube(
        "media_reagent_quarantine_label_printer_placeholder",
        132.0,
        98.0,
        58.0,
    )
    .translate(x + 76.0, y - 54.0, z + 78.0);
    let label_exit = centered_cube("media_reagent_label_printer_exit_slot", 92.0, 8.0, 14.0)
        .translate(x + 76.0, y - 106.0, z + 85.0);
    let scale_plate = centered_cube(
        "media_reagent_receiving_scale_plate_placeholder",
        118.0,
        94.0,
        12.0,
    )
    .translate(x - 86.0, y - 44.0, z + 40.0);

    pedestal
        + work_pad
        + scanner_stem
        + scanner_head
        + lens
        + lot_screen
        + (label_printer - label_exit)
        + scale_plate
}

fn sampling_qc_pass_through_drawers() -> Part {
    let mut drawers = Part::empty("media_reagent_qc_pass_through_drawers");
    for drawer in 0..QC_DRAWER_COUNT {
        let z = 192.0 + drawer as f64 * 86.0;
        drawers = drawers + qc_drawer(drawer, -118.0, -POD_Y / 2.0 - 72.0, z);
    }
    let locked_qc_bulkhead = centered_cube(
        "media_reagent_qc_pass_through_locked_bulkhead",
        910.0,
        22.0,
        212.0,
    )
    .translate(0.0, -POD_Y / 2.0 - 14.0, 236.0);
    let sample_transfer_label = centered_cube(
        "media_reagent_qc_sample_transfer_status_strip",
        360.0,
        6.0,
        30.0,
    )
    .translate(242.0, -POD_Y / 2.0 - 28.0, 342.0);
    drawers + locked_qc_bulkhead + sample_transfer_label
}

fn qc_drawer(index: usize, x: f64, y: f64, z: f64) -> Part {
    let body = centered_cube(
        format!("media_reagent_qc_drawer_{index}_body"),
        DRAWER_X,
        DRAWER_Y,
        DRAWER_Z,
    )
    .translate(x, y, z);
    let pocket = centered_cube(
        format!("media_reagent_qc_drawer_{index}_wipeable_basin"),
        DRAWER_X - 44.0,
        DRAWER_Y - 52.0,
        DRAWER_Z - 20.0,
    )
    .translate(x, y + 8.0, z + 8.0);
    let handle = centered_cube(
        format!("media_reagent_qc_drawer_{index}_front_pull_handle"),
        168.0,
        30.0,
        24.0,
    )
    .translate(x, y - DRAWER_Y / 2.0 - 22.0, z + 8.0);
    let mut wells = Part::empty(format!("media_reagent_qc_drawer_{index}_aliquot_wells"));
    for well in 0..QC_WELLS_PER_DRAWER {
        let col = well % 6;
        let row = well / 6;
        wells = wells
            + centered_cylinder(
                format!("media_reagent_qc_drawer_{index}_aliquot_well_{well}"),
                8.6,
                DRAWER_Z + 4.0,
                24,
            )
            .translate(
                x - 142.0 + col as f64 * 56.0,
                y - 40.0 + row as f64 * 70.0,
                z + 8.0,
            );
    }
    let left_slide = centered_cube(
        format!("media_reagent_qc_drawer_{index}_left_slide_rail"),
        18.0,
        DRAWER_Y + 92.0,
        22.0,
    )
    .translate(x - DRAWER_X / 2.0 - 18.0, y + 16.0, z - 6.0);
    let right_slide = centered_cube(
        format!("media_reagent_qc_drawer_{index}_right_slide_rail"),
        18.0,
        DRAWER_Y + 92.0,
        22.0,
    )
    .translate(x + DRAWER_X / 2.0 + 18.0, y + 16.0, z - 6.0);

    body - pocket - wells + handle + left_slide + right_slide
}

fn cold_pack_thermal_buffer_placeholders() -> Part {
    let cold_pack_shelf =
        centered_cube("media_reagent_cold_pack_recharge_shelf", 468.0, 174.0, 22.0)
            .translate(-338.0, 224.0, 882.0);
    let thermal_mass_shelf = centered_cube(
        "media_reagent_thermal_buffer_mass_shelf",
        436.0,
        174.0,
        22.0,
    )
    .translate(344.0, 224.0, 882.0);

    let mut packs = Part::empty("media_reagent_pcm_cold_pack_placeholders");
    for pack in 0..COLD_PACK_COUNT {
        let col = pack % 4;
        let row = pack / 4;
        packs = packs
            + centered_cube(
                format!("media_reagent_pcm_cold_pack_{pack}"),
                PCM_PACK_X,
                PCM_PACK_Y,
                PCM_PACK_Z,
            )
            .translate(
                -512.0 + col as f64 * 116.0,
                196.0 + row as f64 * 72.0,
                912.0,
            )
            + centered_cube(
                format!("media_reagent_pcm_cold_pack_{pack}_pull_tab"),
                44.0,
                8.0,
                14.0,
            )
            .translate(
                -512.0 + col as f64 * 116.0,
                196.0 + row as f64 * 72.0 - PCM_PACK_Y / 2.0 - 8.0,
                916.0,
            );
    }

    let mut thermal_blocks = Part::empty("media_reagent_thermal_buffer_blocks");
    for block in 0..THERMAL_BUFFER_COUNT {
        let col = block % 3;
        let row = block / 3;
        thermal_blocks = thermal_blocks
            + centered_cube(
                format!("media_reagent_thermal_buffer_block_{block}"),
                98.0,
                84.0,
                42.0,
            )
            .translate(236.0 + col as f64 * 106.0, 200.0 + row as f64 * 78.0, 922.0);
    }

    cold_pack_shelf + thermal_mass_shelf + packs + thermal_blocks + cold_chain_witness_sensors()
}

fn cold_chain_witness_sensors() -> Part {
    let mut sensors = Part::empty("media_reagent_cold_chain_witness_sensors");
    for (i, x) in [-522.0, -292.0, 244.0, 456.0].iter().enumerate() {
        sensors = sensors
            + centered_cube(
                format!("media_reagent_cold_chain_witness_sensor_{i}"),
                54.0,
                18.0,
                34.0,
            )
            .translate(*x, 306.0, 966.0)
            + centered_cylinder(
                format!("media_reagent_cold_chain_sensor_cable_gland_{i}"),
                5.0,
                24.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 326.0, 966.0);
    }
    sensors
}

fn spill_tray_drain_waste_capture() -> Part {
    let tray = centered_cube(
        "media_reagent_spill_tray_outer_pan",
        SPILL_TRAY_X,
        SPILL_TRAY_Y,
        SPILL_TRAY_Z,
    )
    .translate(0.0, 32.0, BASE_PAN_Z + SPILL_TRAY_Z / 2.0);
    let basin = centered_cube(
        "media_reagent_spill_tray_recessed_basin",
        SPILL_TRAY_X - 2.0 * SPILL_TRAY_CURB,
        SPILL_TRAY_Y - 2.0 * SPILL_TRAY_CURB,
        SPILL_TRAY_Z - 14.0,
    )
    .translate(0.0, 32.0, BASE_PAN_Z + SPILL_TRAY_Z / 2.0 + 8.0);
    let gutter = centered_cube(
        "media_reagent_spill_tray_front_drain_gutter",
        SPILL_TRAY_X - 150.0,
        18.0,
        16.0,
    )
    .translate(0.0, -250.0, BASE_PAN_Z + SPILL_TRAY_Z - 6.0);
    let drain_sump = centered_cube(
        "media_reagent_spill_tray_right_drain_sump",
        112.0,
        68.0,
        18.0,
    )
    .translate(POD_X / 2.0 - 158.0, -250.0, BASE_PAN_Z + SPILL_TRAY_Z - 5.0);
    let drain_port = centered_cylinder(
        "media_reagent_spill_tray_drain_port",
        DRAIN_PORT_D / 2.0,
        64.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(POD_X / 2.0 - 158.0, -POD_Y / 2.0 - 8.0, BASE_PAN_Z + 24.0);

    let waste_capture = waste_bottle_placeholders();
    let leak_witness_strip = centered_cube(
        "media_reagent_spill_tray_leak_witness_strip",
        420.0,
        10.0,
        10.0,
    )
    .translate(-210.0, -254.0, BASE_PAN_Z + SPILL_TRAY_Z + 8.0);

    tray - basin - gutter - drain_sump - drain_port + waste_capture + leak_witness_strip
}

fn waste_bottle_placeholders() -> Part {
    let mut bottles = Part::empty("media_reagent_waste_capture_bottles");
    for bottle in 0..WASTE_BOTTLE_COUNT {
        let x = POD_X / 2.0 - 160.0 - bottle as f64 * 110.0;
        let body = centered_cylinder(
            format!("media_reagent_waste_capture_bottle_{bottle}"),
            WASTE_BOTTLE_D / 2.0,
            126.0,
            44,
        )
        .translate(x, -314.0, BASE_PAN_Z + 102.0);
        let cap = centered_cylinder(
            format!("media_reagent_waste_capture_bottle_{bottle}_cap"),
            30.0,
            28.0,
            36,
        )
        .translate(x, -314.0, BASE_PAN_Z + 179.0);
        let tether = centered_cube(
            format!("media_reagent_waste_capture_bottle_{bottle}_tether_land"),
            72.0,
            16.0,
            12.0,
        )
        .translate(x, -374.0, BASE_PAN_Z + 180.0);
        bottles = bottles + body + cap + tether;
    }
    bottles
}

fn pressure_hepa_vhp_clearance_placeholders() -> Part {
    let roof_plenum = centered_cube(
        "media_reagent_roof_pressure_hepa_plenum_placeholder",
        POD_X - 160.0,
        POD_Y - 170.0,
        ROOF_PLENUM_Z - 28.0,
    )
    .translate(0.0, 18.0, POD_Z - ROOF_PLENUM_Z / 2.0 + 10.0);
    let supply_hepa = hepa_filter("supply").translate(-260.0, -92.0, POD_Z - 58.0);
    let return_hepa = hepa_filter("return").translate(260.0, 142.0, POD_Z - 58.0);
    let pressure_panel = pressure_monitor_panel().translate(-POD_X / 2.0 - 26.0, 64.0, 1030.0);
    let vhp = vhp_exposure_clearance_placeholder().translate(386.0, -68.0, 684.0);
    let vent_taps = pressure_vent_taps();

    roof_plenum + supply_hepa + return_hepa + pressure_panel + vhp + vent_taps
}

fn hepa_filter(name: &str) -> Part {
    let frame = centered_cube(
        format!("media_reagent_{name}_hepa_filter_frame"),
        HEPA_FILTER_X,
        HEPA_FILTER_Y,
        HEPA_FILTER_Z,
    );
    let media = centered_cube(
        format!("media_reagent_{name}_hepa_filter_media_recess"),
        HEPA_FILTER_X - 40.0,
        HEPA_FILTER_Y - 34.0,
        HEPA_FILTER_Z - 12.0,
    )
    .translate(0.0, 0.0, 4.0);
    let mut ribs = Part::empty(format!("media_reagent_{name}_hepa_grille_ribs"));
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("media_reagent_{name}_hepa_grille_rib_{i}"),
                12.0,
                HEPA_FILTER_Y - 28.0,
                10.0,
            )
            .translate(-120.0 + i as f64 * 60.0, 0.0, HEPA_FILTER_Z / 2.0 + 5.0);
    }
    frame - media + ribs
}

fn pressure_monitor_panel() -> Part {
    let panel = centered_cube(
        "media_reagent_pressure_dp_monitor_panel",
        22.0,
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Z,
    );
    let mut gauges = Part::empty("media_reagent_pressure_dp_gauges");
    for (i, z) in [-48.0, 0.0, 48.0].iter().enumerate() {
        gauges = gauges
            + centered_cylinder(
                format!("media_reagent_pressure_dp_gauge_{i}"),
                24.0,
                10.0,
                36,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-18.0, -72.0 + i as f64 * 72.0, *z);
    }
    panel + gauges
}

fn vhp_exposure_clearance_placeholder() -> Part {
    let envelope = centered_cube(
        "media_reagent_vhp_exposure_clearance_envelope",
        VHP_CLEARANCE_X,
        VHP_CLEARANCE_Y,
        VHP_CLEARANCE_Z,
    );
    let core = centered_cube(
        "media_reagent_vhp_exposure_keep_clear_core",
        VHP_CLEARANCE_X - 42.0,
        VHP_CLEARANCE_Y - 34.0,
        VHP_CLEARANCE_Z - 44.0,
    );
    let inlet = centered_cylinder("media_reagent_vhp_inlet_port_placeholder", 15.0, 46.0, 36)
        .rotate(90.0, 0.0, 0.0)
        .translate(-150.0, -VHP_CLEARANCE_Y / 2.0 - 18.0, 88.0);
    let exhaust = centered_cylinder("media_reagent_vhp_exhaust_port_placeholder", 17.0, 46.0, 36)
        .rotate(90.0, 0.0, 0.0)
        .translate(150.0, -VHP_CLEARANCE_Y / 2.0 - 18.0, 88.0);
    let catalyst_slot = centered_cube(
        "media_reagent_vhp_catalyst_cartridge_service_slot",
        154.0,
        18.0,
        48.0,
    )
    .translate(0.0, VHP_CLEARANCE_Y / 2.0 + 12.0, -104.0);
    envelope - core + inlet + exhaust + catalyst_slot
}

fn pressure_vent_taps() -> Part {
    let mut taps = Part::empty("media_reagent_pressure_sample_taps");
    for (i, x) in [-430.0, -300.0, 300.0, 430.0].iter().enumerate() {
        taps = taps
            + centered_cylinder(
                format!("media_reagent_pressure_sample_tap_{i}"),
                8.0,
                44.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, POD_Y / 2.0 + 18.0, 1120.0)
            + centered_cube(
                format!("media_reagent_pressure_sample_tap_label_{i}"),
                52.0,
                5.0,
                18.0,
            )
            .translate(*x, POD_Y / 2.0 + 42.0, 1164.0);
    }
    taps
}

fn service_keepout_envelopes() -> Part {
    let front = keepout_frame(
        "front_operator_service_keepout",
        POD_X + 180.0,
        FRONT_SERVICE_CLEARANCE,
        108.0,
    )
    .translate(0.0, -POD_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0, 54.0);
    let rear = keepout_frame(
        "rear_maintenance_service_keepout",
        POD_X + 90.0,
        REAR_SERVICE_CLEARANCE,
        108.0,
    )
    .translate(0.0, POD_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 54.0);
    let left = keepout_frame(
        "left_side_cold_chain_service_keepout",
        SIDE_SERVICE_CLEARANCE,
        POD_Y,
        96.0,
    )
    .translate(-POD_X / 2.0 - SIDE_SERVICE_CLEARANCE / 2.0, 0.0, 48.0);
    let right = keepout_frame(
        "right_side_waste_vhp_service_keepout",
        SIDE_SERVICE_CLEARANCE,
        POD_Y,
        96.0,
    )
    .translate(POD_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0, 0.0, 48.0);
    front + rear + left + right
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let floor = frame_xy(
        format!("media_reagent_{name}_floor_outline"),
        x,
        y,
        12.0,
        8.0,
    )
    .translate(0.0, 0.0, -z / 2.0 + 4.0);
    let top = frame_xy(format!("media_reagent_{name}_top_outline"), x, y, 12.0, 8.0).translate(
        0.0,
        0.0,
        z / 2.0 - 4.0,
    );
    let mut posts = Part::empty(format!("media_reagent_{name}_corner_posts"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        posts = posts
            + centered_cube(format!("media_reagent_{name}_post_{i}"), 12.0, 12.0, z).translate(
                *sx * x / 2.0,
                *sy * y / 2.0,
                0.0,
            );
    }
    floor + top + posts
}

fn cable_gland_row(name: &str) -> Part {
    let rail = centered_cube(
        format!("media_reagent_{name}_cable_gland_rail"),
        238.0,
        18.0,
        42.0,
    );
    let mut glands = Part::empty(format!("media_reagent_{name}_cable_glands"));
    for i in 0..4 {
        glands = glands
            + centered_cylinder(
                format!("media_reagent_{name}_cable_gland_cut_{i}"),
                11.0,
                22.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(-78.0 + i as f64 * 52.0, 0.0, 0.0);
    }
    rail - glands
}

fn gasket_frame_xz(
    name: impl Into<String>,
    outer_x: f64,
    y_t: f64,
    outer_z: f64,
    rail: f64,
) -> Part {
    let base = name.into();
    let top = centered_cube(format!("{base}_top"), outer_x, y_t, rail).translate(
        0.0,
        0.0,
        outer_z / 2.0 - rail / 2.0,
    );
    let bottom = centered_cube(format!("{base}_bottom"), outer_x, y_t, rail).translate(
        0.0,
        0.0,
        -outer_z / 2.0 + rail / 2.0,
    );
    let left = centered_cube(format!("{base}_left"), rail, y_t, outer_z).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{base}_right"), rail, y_t, outer_z).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    top + bottom + left + right
}

fn frame_xy(name: impl Into<String>, outer_x: f64, outer_y: f64, rail: f64, z_t: f64) -> Part {
    let base = name.into();
    let front = centered_cube(format!("{base}_front"), outer_x, rail, z_t).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{base}_rear"), outer_x, rail, z_t).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{base}_left"), rail, outer_y, z_t).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{base}_right"), rail, outer_y, z_t).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn incoming_lane_x(lane: usize) -> f64 {
    -((INCOMING_CARTON_LANES as f64 - 1.0) * 206.0) / 2.0 + lane as f64 * 206.0
}

fn temp_zone_x(zone: usize) -> f64 {
    -((TEMP_ZONE_COUNT as f64 - 1.0) * TEMP_ZONE_PITCH_X) / 2.0 + zone as f64 * TEMP_ZONE_PITCH_X
}

fn temp_zone_span_x() -> f64 {
    (TEMP_ZONE_COUNT as f64 - 1.0) * TEMP_ZONE_PITCH_X + TEMP_ZONE_X
}

fn segregation_span_x() -> f64 {
    QUARANTINE_BAY_X + RELEASED_BAY_X + SEGREGATION_DIVIDER_W + SEGREGATION_AIR_GAP
}

fn incoming_shelf_front_edge() -> f64 {
    POD_Y / 2.0 + INCOMING_SHELF_Y + 8.0
}

fn total_lot_capacity() -> usize {
    (QUARANTINE_BIN_COUNT + RELEASED_BIN_COUNT) * BOTTLES_PER_BIN
        + INCOMING_CARTON_LANES * INCOMING_CARTONS_PER_LANE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pod_dimensions_leave_external_service_access() {
        assert!(POD_X <= 1300.0);
        assert!(POD_Y <= 820.0);
        assert!(POD_Z <= 1450.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 700.0);
        assert!(REAR_SERVICE_CLEARANCE >= 500.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 400.0);
    }

    #[test]
    fn quarantine_and_released_bays_are_physically_segregated() {
        assert!(SEGREGATION_DIVIDER_W >= 28.0);
        assert!(SEGREGATION_AIR_GAP >= 40.0);
        assert!(segregation_span_x() < INNER_X - 96.0);
        assert!(QUARANTINE_BAY_X > RELEASED_BAY_X);
    }

    #[test]
    fn staged_lot_capacity_covers_receiving_quarantine_and_release() {
        assert_eq!(INCOMING_CARTON_LANES * INCOMING_CARTONS_PER_LANE, 8);
        assert_eq!(QUARANTINE_BIN_COUNT * BOTTLES_PER_BIN, 16);
        assert_eq!(RELEASED_BIN_COUNT * BOTTLES_PER_BIN, 12);
        assert!(total_lot_capacity() >= 36);
    }

    #[test]
    fn cold_chain_and_qc_placeholders_have_repeated_positions() {
        assert_eq!(TEMP_ZONE_COUNT, 3);
        assert!(temp_zone_span_x() < INNER_X - 140.0);
        assert_eq!(COLD_PACK_COUNT, 8);
        assert_eq!(THERMAL_BUFFER_COUNT, 6);
        assert_eq!(QC_DRAWER_COUNT * QC_WELLS_PER_DRAWER, 24);
    }
}
