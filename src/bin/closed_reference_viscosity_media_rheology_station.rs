use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reference viscosity and media rheology calibration station.
//
// Intent:
// - Validate how reference-fluid and media viscosity shift pressure-flow
//   calibration and derived shear estimates before closed culture runs.
// - Keep traceable standards under physical custody with temperature
//   equilibration, capillary/restriction coupons, pressure taps, flow sensor
//   docks, flush/waste routing, status lanes, and evidence capture.
// - Model packaging/interface geometry only; rheology math, metrology limits,
//   certificate acceptance, and sterile process controls remain outside CAD.

const OUTPUTS: [&str; 13] = [
    "output/closed_reference_viscosity_media_rheology_station_base_leak_tray.stl",
    "output/closed_reference_viscosity_media_rheology_station_reference_fluid_vial_nests.stl",
    "output/closed_reference_viscosity_media_rheology_station_temperature_equilibration_block.stl",
    "output/closed_reference_viscosity_media_rheology_station_capillary_restriction_coupon_holder.stl",
    "output/closed_reference_viscosity_media_rheology_station_pressure_tap_manifold.stl",
    "output/closed_reference_viscosity_media_rheology_station_flow_sensor_dock.stl",
    "output/closed_reference_viscosity_media_rheology_station_barcode_certificate_lands.stl",
    "output/closed_reference_viscosity_media_rheology_station_high_low_viscosity_standard_lanes.stl",
    "output/closed_reference_viscosity_media_rheology_station_flush_waste_routing.stl",
    "output/closed_reference_viscosity_media_rheology_station_released_hold_reject_lanes.stl",
    "output/closed_reference_viscosity_media_rheology_station_evidence_camera_bridge.stl",
    "output/closed_reference_viscosity_media_rheology_station_robot_service_keepouts.stl",
    "output/closed_reference_viscosity_media_rheology_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "reference_fluid_vial_nests",
    "temperature_equilibration_block",
    "capillary_restriction_coupon_holder",
    "pressure_tap_manifold",
    "flow_sensor_dock",
    "barcode_certificate_lands",
    "high_low_viscosity_standard_lanes",
    "flush_waste_routing",
    "released_hold_reject_lanes",
    "leak_tray",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const DECK_RECESS_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 12.0;

const VIAL_CENTER: (f64, f64) = (-415.0, 220.0);
const VIAL_NEST_X: f64 = 280.0;
const VIAL_NEST_Y: f64 = 210.0;
const VIAL_NEST_Z: f64 = 46.0;
const REFERENCE_VIAL_COUNT: usize = 12;
const VIAL_COLS: usize = 4;
const VIAL_PITCH_X: f64 = 54.0;
const VIAL_PITCH_Y: f64 = 54.0;
const VIAL_WELL_D: f64 = 18.0;

const TEMP_CENTER: (f64, f64) = (-115.0, 220.0);
const TEMP_BLOCK_X: f64 = 240.0;
const TEMP_BLOCK_Y: f64 = 210.0;
const TEMP_BLOCK_Z: f64 = 58.0;
const TEMP_POCKET_COUNT: usize = 8;
const TEMP_POCKET_COLS: usize = 4;
const TEMP_POCKET_PITCH_X: f64 = 48.0;
const TEMP_POCKET_PITCH_Y: f64 = 58.0;
const TEMP_PROBE_COUNT: usize = 4;
const TEMP_EQUILIBRATION_CLEARANCE: f64 = 34.0;

const COUPON_CENTER: (f64, f64) = (200.0, 220.0);
const COUPON_HOLDER_X: f64 = 310.0;
const COUPON_HOLDER_Y: f64 = 210.0;
const COUPON_HOLDER_Z: f64 = 40.0;
const COUPON_LANES: usize = 6;
const COUPON_PITCH_X: f64 = 45.0;
const CAPILLARY_SLOT_X: f64 = 34.0;
const CAPILLARY_SLOT_Y: f64 = 118.0;
const CAPILLARY_ID_LANDS: usize = 6;

const FLOW_CENTER: (f64, f64) = (470.0, 220.0);
const FLOW_DOCK_X: f64 = 180.0;
const FLOW_DOCK_Y: f64 = 210.0;
const FLOW_DOCK_Z: f64 = 48.0;
const FLOW_SENSOR_DOCKS: usize = 4;
const FLOW_SENSOR_PITCH_Y: f64 = 42.0;

const PRESSURE_CENTER: (f64, f64) = (-370.0, 10.0);
const PRESSURE_MANIFOLD_X: f64 = 330.0;
const PRESSURE_MANIFOLD_Y: f64 = 190.0;
const PRESSURE_MANIFOLD_Z: f64 = 44.0;
const PRESSURE_TAP_COUNT: usize = 8;
const PRESSURE_TAP_PITCH_X: f64 = 38.0;
const PRESSURE_PORT_D: f64 = 6.0;

const STANDARD_CENTER: (f64, f64) = (0.0, 10.0);
const STANDARD_LANES_X: f64 = 330.0;
const STANDARD_LANES_Y: f64 = 190.0;
const STANDARD_LANES_Z: f64 = 42.0;
const STANDARD_LANE_COUNT: usize = 2;
const LOW_STANDARD_LANE_X: f64 = -82.0;
const HIGH_STANDARD_LANE_X: f64 = 82.0;
const LOW_STANDARD_SLOTS: usize = 4;
const HIGH_STANDARD_SLOTS: usize = 4;
const STANDARD_SLOT_PITCH_Y: f64 = 34.0;
const STANDARD_CUSTODY_GAP_MIN: f64 = 44.0;

const TRACE_CENTER: (f64, f64) = (370.0, 10.0);
const TRACE_PANEL_X: f64 = 330.0;
const TRACE_PANEL_Y: f64 = 190.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 4;
const RUN_RECORD_LANDS: usize = 3;

const ROUTING_CENTER: (f64, f64) = (-380.0, -225.0);
const ROUTING_X: f64 = 350.0;
const ROUTING_Y: f64 = 230.0;
const ROUTING_Z: f64 = 46.0;
const FLUSH_PORT_COUNT: usize = 8;
const WASTE_CHANNEL_COUNT: usize = 8;
const ROUTING_LANE_PITCH_X: f64 = 38.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const TUBE_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const STATUS_CENTER: (f64, f64) = (0.0, -225.0);
const STATUS_PANEL_X: f64 = 330.0;
const STATUS_PANEL_Y: f64 = 230.0;
const STATUS_PANEL_Z: f64 = 44.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_PITCH_X: f64 = 100.0;
const STATUS_SLOT_Y: f64 = 40.0;
const STATUS_SLOT_PITCH_Y: f64 = 44.0;
const STATUS_LANE_MIN_GAP: f64 = 20.0;

const CAMERA_CENTER: (f64, f64) = (52.0, -54.0);
const CAMERA_BRIDGE_SPAN_X: f64 = 900.0;
const CAMERA_BRIDGE_Y: f64 = 72.0;
const CAMERA_BEAM_Z: f64 = 30.0;
const CAMERA_POST_X: f64 = 32.0;
const CAMERA_POST_Y: f64 = 44.0;
const CAMERA_UNDERSIDE_Z: f64 = 188.0;
const EVIDENCE_CAMERA_COUNT: usize = 3;
const EVIDENCE_LED_COUNT: usize = 7;

const ROBOT_SWEEP_X: f64 = 1060.0;
const ROBOT_SWEEP_Y: f64 = 168.0;
const ROBOT_SWEEP_Z: f64 = 142.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 340.0;
const REAR_SERVICE_CLEARANCE: f64 = 250.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 140.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    export(OUTPUTS[0], &base_leak_tray());
    export(OUTPUTS[1], &reference_fluid_vial_nests());
    export(OUTPUTS[2], &temperature_equilibration_block());
    export(OUTPUTS[3], &capillary_restriction_coupon_holder());
    export(OUTPUTS[4], &pressure_tap_manifold());
    export(OUTPUTS[5], &flow_sensor_dock());
    export(OUTPUTS[6], &barcode_certificate_lands());
    export(OUTPUTS[7], &high_low_viscosity_standard_lanes());
    export(OUTPUTS[8], &flush_waste_routing());
    export(OUTPUTS[9], &released_hold_reject_lanes());
    export(OUTPUTS[10], &evidence_camera_bridge());
    export(OUTPUTS[11], &robot_service_keepouts());
    export(OUTPUTS[12], &station_assembly());

    println!();
    println!("Closed reference viscosity and media rheology calibration station:");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm leak-tray deck");
    println!(
        "  Fluid custody:               {REFERENCE_VIAL_COUNT} reference/media vial nests, {TEMP_POCKET_COUNT} temperature-equilibration pockets, and {STANDARD_LANE_COUNT} separated high/low standard lanes"
    );
    println!(
        "  Pressure-flow calibration:   {COUPON_LANES} capillary/restriction coupons, {PRESSURE_TAP_COUNT} pressure taps, and {FLOW_SENSOR_DOCKS} flow sensor docks"
    );
    println!(
        "  Routing and evidence:        {FLUSH_PORT_COUNT} flush ports, {WASTE_CHANNEL_COUNT} waste channels, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {EVIDENCE_CAMERA_COUNT} evidence cameras"
    );
    println!(
        "  Disposition:                 released/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots per lane, leak tray, robot sweep keepout, and service envelopes"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray()
        + reference_fluid_vial_nests()
        + temperature_equilibration_block()
        + capillary_restriction_coupon_holder()
        + pressure_tap_manifold()
        + flow_sensor_dock()
        + barcode_certificate_lands()
        + high_low_viscosity_standard_lanes()
        + flush_waste_routing()
        + released_hold_reject_lanes()
        + evidence_camera_bridge()
        + robot_service_keepouts()
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_reference_viscosity_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let washdown_basin = centered_cube(
        "closed_reference_viscosity_station_washdown_basin",
        STATION_X - 118.0,
        STATION_Y - 108.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z - 3.0);
    let front_drain = centered_cylinder(
        "closed_reference_viscosity_station_leak_tray_front_drain",
        DRAIN_PORT_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 7.0);

    deck - washdown_basin - front_drain - deck_recesses() - deck_mount_holes()
        + perimeter_rims()
        + station_row_dividers()
        + leak_witness_ribs()
        + robot_datum_targets()
}

fn deck_recesses() -> Part {
    let mut recesses = Part::empty("closed_reference_viscosity_station_deck_module_recesses");
    for (name, center, x, y) in deck_module_specs() {
        recesses = recesses
            + centered_cube(
                format!("closed_reference_viscosity_station_{name}_socket_recess"),
                x + 16.0,
                y + 16.0,
                DECK_RECESS_DEPTH + 0.4,
            )
            .translate(center.0, center.1, BASE_Z - DECK_RECESS_DEPTH / 2.0 + 0.2);
    }
    recesses
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_reference_viscosity_station_deck_mount_holes");
    for (index, (x, y)) in [
        (-(STATION_X / 2.0 - 56.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 56.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 56.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
        (0.0, STATION_Y / 2.0 - 50.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_reference_viscosity_station_m6_mount_bore_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("closed_reference_viscosity_station_m6_mount_slot_{index}"),
                24.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "closed_reference_viscosity_station_rear_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_reference_viscosity_station_left_rim",
        RIM_W,
        STATION_Y - 72.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_reference_viscosity_station_right_rim",
        RIM_W,
        STATION_Y - 72.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let front_lip = centered_cube(
        "closed_reference_viscosity_station_front_low_leak_lip",
        STATION_X - 210.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z + 10.0);

    rear + left + right + front_lip
}

fn station_row_dividers() -> Part {
    let top_mid = centered_cube(
        "closed_reference_viscosity_station_top_mid_row_divider",
        STATION_X - 120.0,
        8.0,
        24.0,
    )
    .translate(0.0, 110.0, BASE_Z + 12.0);
    let mid_bottom = centered_cube(
        "closed_reference_viscosity_station_mid_bottom_row_divider",
        STATION_X - 160.0,
        8.0,
        22.0,
    )
    .translate(-20.0, -108.0, BASE_Z + 11.0);
    let custody_pressure_split = centered_cube(
        "closed_reference_viscosity_station_pressure_standard_split",
        8.0,
        192.0,
        22.0,
    )
    .translate(-182.0, 10.0, BASE_Z + 11.0);
    let standard_trace_split = centered_cube(
        "closed_reference_viscosity_station_standard_trace_split",
        8.0,
        192.0,
        22.0,
    )
    .translate(182.0, 10.0, BASE_Z + 11.0);

    top_mid + mid_bottom + custody_pressure_split + standard_trace_split
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_reference_viscosity_station_leak_witness_ribs");
    for (index, x) in [-495.0, -330.0, -165.0, 0.0, 165.0, 330.0, 495.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_reference_viscosity_station_leak_witness_rib_{index}"),
                6.0,
                STATION_Y - 160.0,
                5.0,
            )
            .translate(x, -10.0, BASE_Z + 2.5);
    }
    ribs
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_reference_viscosity_station_robot_datum_targets");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 82.0, STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 82.0),
        (-STATION_X / 2.0 + 82.0, -STATION_Y / 2.0 + 82.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_reference_viscosity_station_robot_fiducial_{index}"
            ))
            .translate(x, y, BASE_Z + 2.0);
    }
    targets
}

fn reference_fluid_vial_nests() -> Part {
    let tray = centered_cube(
        "closed_reference_viscosity_station_reference_vial_nest_tray",
        VIAL_NEST_X,
        VIAL_NEST_Y,
        VIAL_NEST_Z,
    );
    let basin = centered_cube(
        "closed_reference_viscosity_station_reference_vial_spill_basin",
        VIAL_NEST_X - 34.0,
        VIAL_NEST_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, VIAL_NEST_Z / 2.0 - 3.5);

    let part = tray - basin - vial_well_cuts()
        + vial_well_rims()
        + vial_lane_labels()
        + vial_custody_latch_tabs();
    place_on_deck(part, VIAL_CENTER, VIAL_NEST_Z)
}

fn vial_well_cuts() -> Part {
    let mut wells = Part::empty("closed_reference_viscosity_station_reference_vial_well_cuts");
    for index in 0..REFERENCE_VIAL_COUNT {
        let (x, y) = vial_position(index);
        wells = wells
            + centered_cylinder(
                format!("closed_reference_viscosity_station_reference_vial_well_cut_{index}"),
                VIAL_WELL_D / 2.0,
                VIAL_NEST_Z + 6.0,
                36,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn vial_well_rims() -> Part {
    let mut rims = Part::empty("closed_reference_viscosity_station_reference_vial_well_rims");
    for index in 0..REFERENCE_VIAL_COUNT {
        let (x, y) = vial_position(index);
        let outer = centered_cylinder(
            format!("closed_reference_viscosity_station_reference_vial_rim_{index}"),
            VIAL_WELL_D / 2.0 + 3.0,
            4.0,
            36,
        )
        .translate(x, y, VIAL_NEST_Z / 2.0 + 2.0);
        let inner = centered_cylinder(
            format!("closed_reference_viscosity_station_reference_vial_rim_open_{index}"),
            VIAL_WELL_D / 2.0 + 0.5,
            4.4,
            36,
        )
        .translate(x, y, VIAL_NEST_Z / 2.0 + 2.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn vial_lane_labels() -> Part {
    let mut labels = Part::empty("closed_reference_viscosity_station_reference_vial_lane_labels");
    for (index, label_y) in [-54.0, 0.0, 54.0].into_iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("closed_reference_viscosity_station_reference_media_label_land_{index}"),
                VIAL_NEST_X - 44.0,
                12.0,
                3.0,
            )
            .translate(0.0, label_y - 21.0, VIAL_NEST_Z / 2.0 + 1.5);
    }
    labels
}

fn vial_custody_latch_tabs() -> Part {
    let rear = centered_cube(
        "closed_reference_viscosity_station_reference_vial_rear_custody_latch",
        VIAL_NEST_X - 42.0,
        10.0,
        22.0,
    )
    .translate(0.0, VIAL_NEST_Y / 2.0 - 18.0, VIAL_NEST_Z / 2.0 + 11.0);
    let front = centered_cube(
        "closed_reference_viscosity_station_reference_vial_front_robot_pull",
        92.0,
        10.0,
        14.0,
    )
    .translate(0.0, -VIAL_NEST_Y / 2.0 + 18.0, VIAL_NEST_Z / 2.0 + 7.0);
    rear + front
}

fn temperature_equilibration_block() -> Part {
    let block = centered_cube(
        "closed_reference_viscosity_station_temperature_equilibration_block",
        TEMP_BLOCK_X,
        TEMP_BLOCK_Y,
        TEMP_BLOCK_Z,
    );
    let thermal_plate = centered_cube(
        "closed_reference_viscosity_station_temperature_plate_socket",
        TEMP_BLOCK_X - 34.0,
        TEMP_BLOCK_Y - 36.0,
        10.0,
    )
    .translate(0.0, 0.0, -TEMP_BLOCK_Z / 2.0 + 5.0);
    let coolant_channel = centered_cube(
        "closed_reference_viscosity_station_temperature_coolant_channel",
        TEMP_BLOCK_X - 56.0,
        12.0,
        16.0,
    )
    .translate(0.0, TEMP_BLOCK_Y / 2.0 - 22.0, -4.0);

    let part = block - thermal_plate - coolant_channel - temperature_pocket_cuts()
        + temperature_probe_lands()
        + temperature_equalization_ribs()
        + module_latch_tabs("temperature_equilibration");
    place_on_deck(part, TEMP_CENTER, TEMP_BLOCK_Z)
}

fn temperature_pocket_cuts() -> Part {
    let mut pockets =
        Part::empty("closed_reference_viscosity_station_temperature_equilibration_pockets");
    for index in 0..TEMP_POCKET_COUNT {
        let (x, y) = temp_pocket_position(index);
        pockets = pockets
            + centered_cylinder(
                format!("closed_reference_viscosity_station_temperature_vial_pocket_{index}"),
                22.0 / 2.0,
                TEMP_BLOCK_Z + 8.0,
                36,
            )
            .translate(x, y, 8.0)
            + centered_cube(
                format!("closed_reference_viscosity_station_temperature_square_media_cup_{index}"),
                34.0,
                28.0,
                20.0,
            )
            .translate(x, y + 18.0, TEMP_BLOCK_Z / 2.0 - 10.0);
    }
    pockets
}

fn temperature_probe_lands() -> Part {
    let mut probes = Part::empty("closed_reference_viscosity_station_temperature_probe_lands");
    for index in 0..TEMP_PROBE_COUNT {
        let x = centered_index(index, TEMP_PROBE_COUNT, 45.0);
        probes = probes
            + centered_cube(
                format!("closed_reference_viscosity_station_temperature_probe_clip_{index}"),
                32.0,
                12.0,
                9.0,
            )
            .translate(x, -TEMP_BLOCK_Y / 2.0 + 20.0, TEMP_BLOCK_Z / 2.0 + 4.5)
            - centered_cylinder(
                format!("closed_reference_viscosity_station_temperature_probe_bore_{index}"),
                3.0 / 2.0,
                36.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TEMP_BLOCK_Y / 2.0 + 20.0, TEMP_BLOCK_Z / 2.0 + 4.5);
    }
    probes
}

fn temperature_equalization_ribs() -> Part {
    let mut ribs = Part::empty("closed_reference_viscosity_station_temperature_equalization_ribs");
    for index in 0..=TEMP_POCKET_COLS {
        ribs = ribs
            + centered_cube(
                format!("closed_reference_viscosity_station_temperature_equalization_rib_{index}"),
                4.0,
                TEMP_BLOCK_Y - 46.0,
                5.0,
            )
            .translate(
                centered_index(index, TEMP_POCKET_COLS + 1, TEMP_POCKET_PITCH_X),
                0.0,
                TEMP_BLOCK_Z / 2.0 + 2.5,
            );
    }
    ribs
}

fn capillary_restriction_coupon_holder() -> Part {
    let holder = centered_cube(
        "closed_reference_viscosity_station_capillary_coupon_holder_body",
        COUPON_HOLDER_X,
        COUPON_HOLDER_Y,
        COUPON_HOLDER_Z,
    );
    let drain_basin = centered_cube(
        "closed_reference_viscosity_station_capillary_coupon_spill_basin",
        COUPON_HOLDER_X - 36.0,
        COUPON_HOLDER_Y - 42.0,
        8.0,
    )
    .translate(0.0, 0.0, COUPON_HOLDER_Z / 2.0 - 3.5);

    let part = holder - drain_basin - capillary_slot_cuts()
        + coupon_clamp_bridges()
        + capillary_id_lands()
        + module_latch_tabs("capillary_coupon");
    place_on_deck(part, COUPON_CENTER, COUPON_HOLDER_Z)
}

fn capillary_slot_cuts() -> Part {
    let mut slots = Part::empty("closed_reference_viscosity_station_capillary_slot_cuts");
    for lane in 0..COUPON_LANES {
        let x = centered_index(lane, COUPON_LANES, COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_reference_viscosity_station_capillary_restriction_slot_{lane}"),
                CAPILLARY_SLOT_X,
                CAPILLARY_SLOT_Y,
                COUPON_HOLDER_Z + 5.0,
            )
            .translate(x, -10.0, 1.0)
            + centered_cylinder(
                format!("closed_reference_viscosity_station_coupon_inlet_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                34.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -78.0, 3.0)
            + centered_cylinder(
                format!("closed_reference_viscosity_station_coupon_outlet_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                34.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 58.0, 3.0);
    }
    slots
}

fn coupon_clamp_bridges() -> Part {
    let mut clamps = Part::empty("closed_reference_viscosity_station_coupon_clamp_bridges");
    for lane in 0..COUPON_LANES {
        let x = centered_index(lane, COUPON_LANES, COUPON_PITCH_X);
        clamps = clamps
            + centered_cube(
                format!("closed_reference_viscosity_station_coupon_front_clamp_{lane}"),
                CAPILLARY_SLOT_X + 10.0,
                10.0,
                14.0,
            )
            .translate(x, -72.0, COUPON_HOLDER_Z / 2.0 + 7.0)
            + centered_cube(
                format!("closed_reference_viscosity_station_coupon_rear_clamp_{lane}"),
                CAPILLARY_SLOT_X + 10.0,
                10.0,
                14.0,
            )
            .translate(x, 52.0, COUPON_HOLDER_Z / 2.0 + 7.0);
    }
    clamps
}

fn capillary_id_lands() -> Part {
    let mut lands = Part::empty("closed_reference_viscosity_station_capillary_id_lands");
    for lane in 0..CAPILLARY_ID_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_viscosity_station_capillary_certificate_land_{lane}"),
                36.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(lane, CAPILLARY_ID_LANDS, COUPON_PITCH_X),
                COUPON_HOLDER_Y / 2.0 - 20.0,
                COUPON_HOLDER_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn pressure_tap_manifold() -> Part {
    let manifold = centered_cube(
        "closed_reference_viscosity_station_pressure_tap_manifold_body",
        PRESSURE_MANIFOLD_X,
        PRESSURE_MANIFOLD_Y,
        PRESSURE_MANIFOLD_Z,
    );
    let tube_gallery = centered_cube(
        "closed_reference_viscosity_station_pressure_tube_gallery",
        PRESSURE_MANIFOLD_X - 36.0,
        20.0,
        16.0,
    )
    .translate(0.0, -PRESSURE_MANIFOLD_Y / 2.0 + 34.0, 0.0);

    let part = manifold - tube_gallery - pressure_tap_bores()
        + pressure_tap_labels()
        + manifold_valve_witness_buttons()
        + module_latch_tabs("pressure_tap");
    place_on_deck(part, PRESSURE_CENTER, PRESSURE_MANIFOLD_Z)
}

fn pressure_tap_bores() -> Part {
    let mut bores = Part::empty("closed_reference_viscosity_station_pressure_tap_bores");
    for tap in 0..PRESSURE_TAP_COUNT {
        let x = centered_index(tap, PRESSURE_TAP_COUNT, PRESSURE_TAP_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("closed_reference_viscosity_station_pressure_tap_vertical_bore_{tap}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_MANIFOLD_Z + 8.0,
                24,
            )
            .translate(x, 12.0, 2.0)
            + centered_cylinder(
                format!("closed_reference_viscosity_station_pressure_tap_side_port_{tap}"),
                TUBE_BORE_D / 2.0,
                46.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -PRESSURE_MANIFOLD_Y / 2.0 + 24.0, 0.0);
    }
    bores
}

fn pressure_tap_labels() -> Part {
    let mut labels = Part::empty("closed_reference_viscosity_station_pressure_tap_labels");
    for tap in 0..PRESSURE_TAP_COUNT {
        labels = labels
            + centered_cube(
                format!("closed_reference_viscosity_station_pressure_tap_label_land_{tap}"),
                28.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(tap, PRESSURE_TAP_COUNT, PRESSURE_TAP_PITCH_X),
                50.0,
                PRESSURE_MANIFOLD_Z / 2.0 + 1.5,
            );
    }
    labels
}

fn manifold_valve_witness_buttons() -> Part {
    let mut buttons = Part::empty("closed_reference_viscosity_station_pressure_valve_witness");
    for tap in 0..PRESSURE_TAP_COUNT {
        buttons = buttons
            + centered_cylinder(
                format!("closed_reference_viscosity_station_pressure_valve_witness_button_{tap}"),
                7.0,
                5.0,
                24,
            )
            .translate(
                centered_index(tap, PRESSURE_TAP_COUNT, PRESSURE_TAP_PITCH_X),
                -36.0,
                PRESSURE_MANIFOLD_Z / 2.0 + 2.5,
            );
    }
    buttons
}

fn flow_sensor_dock() -> Part {
    let dock = centered_cube(
        "closed_reference_viscosity_station_flow_sensor_dock_body",
        FLOW_DOCK_X,
        FLOW_DOCK_Y,
        FLOW_DOCK_Z,
    );
    let service_channel = centered_cube(
        "closed_reference_viscosity_station_flow_sensor_cable_channel",
        18.0,
        FLOW_DOCK_Y - 34.0,
        18.0,
    )
    .translate(FLOW_DOCK_X / 2.0 - 28.0, 0.0, 0.0);

    let part = dock - service_channel - flow_sensor_pockets()
        + flow_sensor_clip_bosses()
        + flow_direction_arrows()
        + module_latch_tabs("flow_sensor");
    place_on_deck(part, FLOW_CENTER, FLOW_DOCK_Z)
}

fn flow_sensor_pockets() -> Part {
    let mut pockets = Part::empty("closed_reference_viscosity_station_flow_sensor_pockets");
    for index in 0..FLOW_SENSOR_DOCKS {
        let y = centered_index(index, FLOW_SENSOR_DOCKS, FLOW_SENSOR_PITCH_Y);
        pockets = pockets
            + centered_cube(
                format!("closed_reference_viscosity_station_flow_sensor_sled_pocket_{index}"),
                98.0,
                28.0,
                26.0,
            )
            .translate(-18.0, y, FLOW_DOCK_Z / 2.0 - 12.0)
            + centered_cylinder(
                format!("closed_reference_viscosity_station_flow_sensor_inline_bore_{index}"),
                TUBE_BORE_D / 2.0,
                128.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-18.0, y, 4.0);
    }
    pockets
}

fn flow_sensor_clip_bosses() -> Part {
    let mut bosses = Part::empty("closed_reference_viscosity_station_flow_sensor_clip_bosses");
    for index in 0..FLOW_SENSOR_DOCKS {
        let y = centered_index(index, FLOW_SENSOR_DOCKS, FLOW_SENSOR_PITCH_Y);
        bosses = bosses
            + centered_cube(
                format!("closed_reference_viscosity_station_flow_sensor_front_clip_{index}"),
                24.0,
                8.0,
                14.0,
            )
            .translate(-74.0, y - 18.0, FLOW_DOCK_Z / 2.0 + 7.0)
            + centered_cube(
                format!("closed_reference_viscosity_station_flow_sensor_rear_clip_{index}"),
                24.0,
                8.0,
                14.0,
            )
            .translate(38.0, y + 18.0, FLOW_DOCK_Z / 2.0 + 7.0);
    }
    bosses
}

fn flow_direction_arrows() -> Part {
    let mut arrows = Part::empty("closed_reference_viscosity_station_flow_direction_arrows");
    for index in 0..FLOW_SENSOR_DOCKS {
        let y = centered_index(index, FLOW_SENSOR_DOCKS, FLOW_SENSOR_PITCH_Y);
        arrows = arrows
            + centered_cube(
                format!("closed_reference_viscosity_station_flow_direction_arrow_body_{index}"),
                44.0,
                5.0,
                4.0,
            )
            .translate(46.0, y, FLOW_DOCK_Z / 2.0 + 2.0)
            + centered_cube(
                format!("closed_reference_viscosity_station_flow_direction_arrow_head_{index}"),
                10.0,
                15.0,
                4.0,
            )
            .translate(72.0, y, FLOW_DOCK_Z / 2.0 + 2.0);
    }
    arrows
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_reference_viscosity_station_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let part = panel + barcode_lands() + certificate_lands() + run_record_lands();
    place_on_deck(part, TRACE_CENTER, TRACE_PANEL_Z)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_reference_viscosity_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let col = index % 5;
        let row = index / 5;
        lands = lands
            + centered_cube(
                format!("closed_reference_viscosity_station_barcode_land_{index}"),
                46.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(col, 5, 55.0),
                -58.0 + row as f64 * 32.0,
                TRACE_PANEL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("closed_reference_viscosity_station_certificate_lands");
    for index in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_viscosity_station_certificate_card_land_{index}"),
                92.0,
                30.0,
                3.0,
            )
            .translate(
                centered_index(index, CERTIFICATE_LANDS, 76.0),
                18.0,
                TRACE_PANEL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn run_record_lands() -> Part {
    let mut lands = Part::empty("closed_reference_viscosity_station_run_record_lands");
    for index in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_reference_viscosity_station_run_record_land_{index}"),
                78.0,
                20.0,
                3.0,
            )
            .translate(
                centered_index(index, RUN_RECORD_LANDS, 94.0),
                TRACE_PANEL_Y / 2.0 - 30.0,
                TRACE_PANEL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn high_low_viscosity_standard_lanes() -> Part {
    let panel = centered_cube(
        "closed_reference_viscosity_station_high_low_standard_lane_panel",
        STANDARD_LANES_X,
        STANDARD_LANES_Y,
        STANDARD_LANES_Z,
    );
    let divider = centered_cube(
        "closed_reference_viscosity_station_high_low_standard_center_divider",
        12.0,
        STANDARD_LANES_Y - 24.0,
        STANDARD_LANES_Z + 24.0,
    )
    .translate(0.0, 0.0, 12.0);
    let part = panel + divider - standard_lane_pockets()
        + standard_lane_rims()
        + standard_lane_status_keys()
        + module_latch_tabs("high_low_standard");
    place_on_deck(part, STANDARD_CENTER, STANDARD_LANES_Z)
}

fn standard_lane_pockets() -> Part {
    let mut pockets = Part::empty("closed_reference_viscosity_station_standard_lane_pockets");
    for lane in 0..STANDARD_LANE_COUNT {
        let x = standard_lane_x(lane);
        let slots = if lane == 0 {
            LOW_STANDARD_SLOTS
        } else {
            HIGH_STANDARD_SLOTS
        };
        for slot in 0..slots {
            let y = centered_index(slot, slots, STANDARD_SLOT_PITCH_Y);
            pockets = pockets
                + centered_cylinder(
                    format!(
                        "closed_reference_viscosity_station_{}_viscosity_standard_well_{slot}",
                        standard_lane_name(lane)
                    ),
                    18.0 / 2.0,
                    STANDARD_LANES_Z + 6.0,
                    32,
                )
                .translate(x, y, 4.0)
                + centered_cube(
                    format!(
                        "closed_reference_viscosity_station_{}_viscosity_box_socket_{slot}",
                        standard_lane_name(lane)
                    ),
                    30.0,
                    24.0,
                    18.0,
                )
                .translate(x + 34.0, y, STANDARD_LANES_Z / 2.0 - 8.0);
        }
    }
    pockets
}

fn standard_lane_rims() -> Part {
    let mut rims = Part::empty("closed_reference_viscosity_station_standard_lane_well_rims");
    for lane in 0..STANDARD_LANE_COUNT {
        let x = standard_lane_x(lane);
        let slots = if lane == 0 {
            LOW_STANDARD_SLOTS
        } else {
            HIGH_STANDARD_SLOTS
        };
        for slot in 0..slots {
            let y = centered_index(slot, slots, STANDARD_SLOT_PITCH_Y);
            let outer = centered_cylinder(
                format!(
                    "closed_reference_viscosity_station_{}_standard_well_rim_{slot}",
                    standard_lane_name(lane)
                ),
                12.0,
                4.0,
                32,
            )
            .translate(x, y, STANDARD_LANES_Z / 2.0 + 2.0);
            let inner = centered_cylinder(
                format!(
                    "closed_reference_viscosity_station_{}_standard_well_rim_open_{slot}",
                    standard_lane_name(lane)
                ),
                9.3,
                4.4,
                32,
            )
            .translate(x, y, STANDARD_LANES_Z / 2.0 + 2.0);
            rims = rims + (outer - inner);
        }
    }
    rims
}

fn standard_lane_status_keys() -> Part {
    let low = centered_cube(
        "closed_reference_viscosity_station_low_viscosity_lane_key",
        98.0,
        14.0,
        12.0,
    )
    .translate(
        LOW_STANDARD_LANE_X,
        -STANDARD_LANES_Y / 2.0 + 18.0,
        STANDARD_LANES_Z / 2.0 + 6.0,
    );
    let high = centered_cube(
        "closed_reference_viscosity_station_high_viscosity_lane_key",
        98.0,
        14.0,
        12.0,
    )
    .translate(
        HIGH_STANDARD_LANE_X,
        -STANDARD_LANES_Y / 2.0 + 18.0,
        STANDARD_LANES_Z / 2.0 + 6.0,
    );
    low + high
}

fn flush_waste_routing() -> Part {
    let routing = centered_cube(
        "closed_reference_viscosity_station_flush_waste_routing_body",
        ROUTING_X,
        ROUTING_Y,
        ROUTING_Z,
    );
    let waste_sump = centered_cube(
        "closed_reference_viscosity_station_flush_waste_sump",
        ROUTING_X - 44.0,
        54.0,
        12.0,
    )
    .translate(0.0, -ROUTING_Y / 2.0 + 42.0, ROUTING_Z / 2.0 - 6.0);
    let part = routing - waste_sump - flush_port_bores() - waste_channel_cuts()
        + routing_tube_clips()
        + waste_bottle_dock()
        + module_latch_tabs("flush_waste");
    place_on_deck(part, ROUTING_CENTER, ROUTING_Z)
}

fn flush_port_bores() -> Part {
    let mut bores = Part::empty("closed_reference_viscosity_station_flush_port_bores");
    for port in 0..FLUSH_PORT_COUNT {
        bores = bores
            + centered_cylinder(
                format!("closed_reference_viscosity_station_flush_port_bore_{port}"),
                TUBE_BORE_D / 2.0,
                46.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(port, FLUSH_PORT_COUNT, ROUTING_LANE_PITCH_X),
                ROUTING_Y / 2.0 - 28.0,
                0.0,
            );
    }
    bores
}

fn waste_channel_cuts() -> Part {
    let mut cuts = Part::empty("closed_reference_viscosity_station_waste_channel_cuts");
    for channel in 0..WASTE_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("closed_reference_viscosity_station_waste_channel_{channel}"),
                8.0,
                ROUTING_Y - 76.0,
                12.0,
            )
            .translate(
                centered_index(channel, WASTE_CHANNEL_COUNT, ROUTING_LANE_PITCH_X),
                -8.0,
                ROUTING_Z / 2.0 - 5.0,
            );
    }
    let drain = centered_cylinder(
        "closed_reference_viscosity_station_waste_routing_drain",
        DRAIN_PORT_D / 2.0,
        54.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(ROUTING_X / 2.0 - 34.0, -ROUTING_Y / 2.0 + 18.0, -2.0);

    cuts + drain
}

fn routing_tube_clips() -> Part {
    let mut clips = Part::empty("closed_reference_viscosity_station_routing_tube_clips");
    for channel in 0..WASTE_CHANNEL_COUNT {
        clips = clips
            + centered_cube(
                format!("closed_reference_viscosity_station_routing_tube_clip_{channel}"),
                22.0,
                10.0,
                14.0,
            )
            .translate(
                centered_index(channel, WASTE_CHANNEL_COUNT, ROUTING_LANE_PITCH_X),
                18.0,
                ROUTING_Z / 2.0 + 7.0,
            );
    }
    clips
}

fn waste_bottle_dock() -> Part {
    let dock = centered_cube(
        "closed_reference_viscosity_station_waste_bottle_dock",
        86.0,
        46.0,
        18.0,
    )
    .translate(
        ROUTING_X / 2.0 - 58.0,
        -ROUTING_Y / 2.0 + 54.0,
        ROUTING_Z / 2.0 + 9.0,
    );
    let bottle_recess = centered_cylinder(
        "closed_reference_viscosity_station_waste_bottle_neck_recess",
        18.0,
        20.0,
        36,
    )
    .translate(
        ROUTING_X / 2.0 - 58.0,
        -ROUTING_Y / 2.0 + 54.0,
        ROUTING_Z / 2.0 + 9.0,
    );
    dock - bottle_recess
}

fn released_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_reference_viscosity_station_status_lane_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    );
    let rear_gate = centered_cube(
        "closed_reference_viscosity_station_released_hold_reject_gate_rail",
        STATUS_PANEL_X - 30.0,
        12.0,
        STATUS_PANEL_Z + 32.0,
    )
    .translate(0.0, STATUS_PANEL_Y / 2.0 - 18.0, 16.0);
    let part = panel + rear_gate + status_lane_dividers() - status_lane_pockets()
        + status_lane_keys()
        + module_latch_tabs("status_lane");
    place_on_deck(part, STATUS_CENTER, STATUS_PANEL_Z)
}

fn status_lane_dividers() -> Part {
    let left = centered_cube(
        "closed_reference_viscosity_station_released_hold_divider",
        10.0,
        STATUS_PANEL_Y - 34.0,
        STATUS_PANEL_Z + 18.0,
    )
    .translate(-STATUS_LANE_PITCH_X / 2.0, 0.0, 9.0);
    let right = centered_cube(
        "closed_reference_viscosity_station_hold_reject_divider",
        10.0,
        STATUS_PANEL_Y - 34.0,
        STATUS_PANEL_Z + 18.0,
    )
    .translate(STATUS_LANE_PITCH_X / 2.0, 0.0, 9.0);
    left + right
}

fn status_lane_pockets() -> Part {
    let mut pockets = Part::empty("closed_reference_viscosity_station_status_lane_pockets");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = centered_index(slot, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y);
            pockets = pockets
                + centered_cube(
                    format!(
                        "closed_reference_viscosity_station_{}_sample_slot_{slot}",
                        status_lane_name(lane)
                    ),
                    STATUS_SLOT_Y,
                    28.0,
                    20.0,
                )
                .translate(x, y, STATUS_PANEL_Z / 2.0 - 9.0);
        }
    }
    pockets
}

fn status_lane_keys() -> Part {
    let mut keys = Part::empty("closed_reference_viscosity_station_status_lane_keys");
    for lane in 0..STATUS_LANES {
        keys = keys
            + centered_cube(
                format!(
                    "closed_reference_viscosity_station_{}_status_key",
                    status_lane_name(lane)
                ),
                72.0,
                14.0,
                12.0,
            )
            .translate(
                status_lane_x(lane),
                -STATUS_PANEL_Y / 2.0 + 18.0,
                STATUS_PANEL_Z / 2.0 + 6.0,
            );
    }
    keys
}

fn evidence_camera_bridge() -> Part {
    let post_height = CAMERA_UNDERSIDE_Z;
    let post_z = BASE_Z + post_height / 2.0;
    let left_x = CAMERA_CENTER.0 - CAMERA_BRIDGE_SPAN_X / 2.0;
    let right_x = CAMERA_CENTER.0 + CAMERA_BRIDGE_SPAN_X / 2.0;

    let mut posts = Part::empty("closed_reference_viscosity_station_evidence_bridge_posts");
    for (index, (x, y)) in [
        (left_x, CAMERA_CENTER.1 - CAMERA_BRIDGE_Y / 2.0),
        (left_x, CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0),
        (right_x, CAMERA_CENTER.1 - CAMERA_BRIDGE_Y / 2.0),
        (right_x, CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_reference_viscosity_station_evidence_bridge_post_{index}"),
                CAMERA_POST_X,
                CAMERA_POST_Y,
                post_height,
            )
            .translate(x, y, post_z);
    }

    let beam = centered_cube(
        "closed_reference_viscosity_station_evidence_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X + CAMERA_POST_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1,
        BASE_Z + CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );

    posts + beam + evidence_camera_pods() + evidence_led_bars() + certificate_view_ruler()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("closed_reference_viscosity_station_evidence_camera_pods");
    for index in 0..EVIDENCE_CAMERA_COUNT {
        let x = CAMERA_CENTER.0 + centered_index(index, EVIDENCE_CAMERA_COUNT, 210.0);
        let camera = centered_cube(
            format!("closed_reference_viscosity_station_evidence_camera_body_{index}"),
            58.0,
            46.0,
            38.0,
        )
        .translate(x, CAMERA_CENTER.1, BASE_Z + CAMERA_UNDERSIDE_Z - 18.0);
        let lens = centered_cylinder(
            format!("closed_reference_viscosity_station_evidence_camera_lens_{index}"),
            12.0,
            16.0,
            32,
        )
        .translate(x, CAMERA_CENTER.1, BASE_Z + CAMERA_UNDERSIDE_Z - 43.0);
        pods = pods + camera + lens;
    }
    pods
}

fn evidence_led_bars() -> Part {
    let mut bars = Part::empty("closed_reference_viscosity_station_evidence_led_bars");
    for index in 0..EVIDENCE_LED_COUNT {
        bars = bars
            + centered_cube(
                format!("closed_reference_viscosity_station_evidence_led_bar_{index}"),
                72.0,
                8.0,
                9.0,
            )
            .translate(
                CAMERA_CENTER.0 + centered_index(index, EVIDENCE_LED_COUNT, 108.0),
                CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 + 7.0,
                BASE_Z + CAMERA_UNDERSIDE_Z - 7.0,
            );
    }
    bars
}

fn certificate_view_ruler() -> Part {
    let ruler = centered_cube(
        "closed_reference_viscosity_station_certificate_view_scale_ruler",
        420.0,
        8.0,
        6.0,
    )
    .translate(CAMERA_CENTER.0 + 175.0, 88.0, BASE_Z + 34.0);
    let mut ticks = Part::empty("closed_reference_viscosity_station_certificate_view_ticks");
    for index in 0..9 {
        ticks = ticks
            + centered_cube(
                format!("closed_reference_viscosity_station_certificate_view_tick_{index}"),
                3.0,
                16.0,
                8.0,
            )
            .translate(
                CAMERA_CENTER.0 - 25.0 + index as f64 * 50.0,
                88.0,
                BASE_Z + 35.0,
            );
    }
    ruler + ticks
}

fn robot_service_keepouts() -> Part {
    let robot_sweep = centered_cube(
        "closed_reference_viscosity_station_robot_sweep_keepout",
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        ROBOT_SWEEP_Z,
    )
    .translate(12.0, 58.0, BASE_Z + ROBOT_SWEEP_Z / 2.0);
    let front_service = centered_cube(
        "closed_reference_viscosity_station_front_service_keepout",
        STATION_X - 220.0,
        FRONT_SERVICE_CLEARANCE,
        70.0,
    )
    .translate(
        -20.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 35.0,
    );
    let rear_service = centered_cube(
        "closed_reference_viscosity_station_rear_service_keepout",
        STATION_X - 260.0,
        REAR_SERVICE_CLEARANCE,
        76.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 38.0,
    );
    let right_service = centered_cube(
        "closed_reference_viscosity_station_right_sensor_service_keepout",
        RIGHT_SERVICE_CLEARANCE,
        STATION_Y - 150.0,
        76.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SERVICE_CLEARANCE / 2.0,
        4.0,
        BASE_Z + 38.0,
    );

    robot_sweep - robot_sweep_windows() + front_service + rear_service + right_service
}

fn robot_sweep_windows() -> Part {
    let mut windows = Part::empty("closed_reference_viscosity_station_robot_sweep_windows");
    for index in 0..ROBOT_KEEP_OUT_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_reference_viscosity_station_robot_sweep_window_{index}"),
                182.0,
                ROBOT_SWEEP_Y + 8.0,
                72.0,
            )
            .translate(
                12.0 + centered_index(index, ROBOT_KEEP_OUT_WINDOWS, 232.0),
                58.0,
                BASE_Z + ROBOT_SWEEP_Z / 2.0,
            );
    }
    windows
}

fn module_latch_tabs(prefix: &str) -> Part {
    let left = centered_cube(
        format!("closed_reference_viscosity_station_{prefix}_left_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(-34.0, 0.0, 6.0);
    let right = centered_cube(
        format!("closed_reference_viscosity_station_{prefix}_right_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(34.0, 0.0, 6.0);
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 12.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center"), 3.0, 4.0, 24)
}

fn place_on_deck(part: Part, center: (f64, f64), height: f64) -> Part {
    part.translate(center.0, center.1, BASE_Z + height / 2.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn vial_position(index: usize) -> (f64, f64) {
    let col = index % VIAL_COLS;
    let row = index / VIAL_COLS;
    (
        centered_index(col, VIAL_COLS, VIAL_PITCH_X),
        centered_index(row, REFERENCE_VIAL_COUNT / VIAL_COLS, VIAL_PITCH_Y),
    )
}

fn temp_pocket_position(index: usize) -> (f64, f64) {
    let col = index % TEMP_POCKET_COLS;
    let row = index / TEMP_POCKET_COLS;
    (
        centered_index(col, TEMP_POCKET_COLS, TEMP_POCKET_PITCH_X),
        centered_index(
            row,
            TEMP_POCKET_COUNT / TEMP_POCKET_COLS,
            TEMP_POCKET_PITCH_Y,
        ),
    )
}

fn standard_lane_x(lane: usize) -> f64 {
    match lane {
        0 => LOW_STANDARD_LANE_X,
        1 => HIGH_STANDARD_LANE_X,
        _ => unreachable!("only low and high viscosity standard lanes are defined"),
    }
}

fn standard_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "low",
        1 => "high",
        _ => unreachable!("only low and high viscosity standard lanes are defined"),
    }
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_X)
}

fn status_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "released",
        1 => "hold",
        2 => "reject",
        _ => unreachable!("only released, hold, and reject lanes are defined"),
    }
}

fn standard_lane_gap() -> f64 {
    HIGH_STANDARD_LANE_X - LOW_STANDARD_LANE_X - 2.0 * 43.0
}

fn status_lane_gap() -> f64 {
    status_lane_x(1) - status_lane_x(0) - STATUS_SLOT_Y
}

fn bridge_clearance_above_deck() -> f64 {
    CAMERA_UNDERSIDE_Z
}

fn deck_module_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "reference_fluid_vial_nests",
            VIAL_CENTER,
            VIAL_NEST_X,
            VIAL_NEST_Y,
        ),
        (
            "temperature_equilibration_block",
            TEMP_CENTER,
            TEMP_BLOCK_X,
            TEMP_BLOCK_Y,
        ),
        (
            "capillary_restriction_coupon_holder",
            COUPON_CENTER,
            COUPON_HOLDER_X,
            COUPON_HOLDER_Y,
        ),
        ("flow_sensor_dock", FLOW_CENTER, FLOW_DOCK_X, FLOW_DOCK_Y),
        (
            "pressure_tap_manifold",
            PRESSURE_CENTER,
            PRESSURE_MANIFOLD_X,
            PRESSURE_MANIFOLD_Y,
        ),
        (
            "high_low_viscosity_standard_lanes",
            STANDARD_CENTER,
            STANDARD_LANES_X,
            STANDARD_LANES_Y,
        ),
        (
            "barcode_certificate_lands",
            TRACE_CENTER,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        ("flush_waste_routing", ROUTING_CENTER, ROUTING_X, ROUTING_Y),
        (
            "released_hold_reject_status_lanes",
            STATUS_CENTER,
            STATUS_PANEL_X,
            STATUS_PANEL_Y,
        ),
    ]
}

fn assert_layout() {
    for (name, center, x, y) in deck_module_specs() {
        assert!(
            fits_on_deck(center, x, y, RIM_W + 8.0),
            "{name} exceeds station deck envelope"
        );
    }

    let specs = deck_module_specs();
    for (left_index, left) in specs.iter().enumerate() {
        for right in specs.iter().skip(left_index + 1) {
            assert!(
                !rects_overlap(
                    rect(left.1, left.2 + 8.0, left.3 + 8.0),
                    rect(right.1, right.2 + 8.0, right.3 + 8.0),
                ),
                "{} overlaps {}",
                left.0,
                right.0
            );
        }
    }

    assert!(bridge_clearance_above_deck() >= 180.0);
    assert!(standard_lane_gap() >= STANDARD_CUSTODY_GAP_MIN);
    assert!(status_lane_gap() >= STATUS_LANE_MIN_GAP);
    assert!((VIAL_CENTER.1 - TEMP_CENTER.1).abs() < TEMP_EQUILIBRATION_CLEARANCE);
    assert!((COUPON_CENTER.1 - TEMP_CENTER.1).abs() < TEMP_EQUILIBRATION_CLEARANCE);
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0 - x / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + x / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - y / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + y / 2.0 <= STATION_Y / 2.0 - margin
}

#[derive(Clone, Copy)]
struct Rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

fn rect(center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect {
        x0: center.0 - x / 2.0,
        x1: center.0 + x / 2.0,
        y0: center.1 - y / 2.0,
        y1: center.1 + y / 2.0,
    }
}

fn rects_overlap(left: Rect, right: Rect) -> bool {
    left.x0 < right.x1 && left.x1 > right.x0 && left.y0 < right.y1 && left.y1 > right.y0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_reference_viscosity_media_rheology_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS
            .iter()
            .any(|path| path
                .ends_with("closed_reference_viscosity_media_rheology_station_assembly.stl")));
    }

    #[test]
    fn required_features_cover_viscosity_and_rheology_station_scope() {
        for feature in [
            "reference_fluid_vial_nests",
            "temperature_equilibration_block",
            "capillary_restriction_coupon_holder",
            "pressure_tap_manifold",
            "flow_sensor_dock",
            "barcode_certificate_lands",
            "high_low_viscosity_standard_lanes",
            "flush_waste_routing",
            "released_hold_reject_lanes",
            "leak_tray",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 12);
    }

    #[test]
    fn deck_modules_fit_without_footprint_collisions() {
        assert_layout();
        assert!(deck_module_specs()
            .iter()
            .all(|(_, center, x, y)| fits_on_deck(*center, *x, *y, RIM_W + 8.0)));
    }

    #[test]
    fn temperature_and_standard_custody_geometry_is_explicit() {
        assert_eq!(REFERENCE_VIAL_COUNT, 12);
        assert_eq!(TEMP_POCKET_COUNT, LOW_STANDARD_SLOTS + HIGH_STANDARD_SLOTS);
        assert_eq!(STANDARD_LANE_COUNT, 2);
        assert_eq!(LOW_STANDARD_SLOTS, HIGH_STANDARD_SLOTS);
        assert!(TEMP_EQUILIBRATION_CLEARANCE >= 30.0);
        assert!(temp_block_center_is_between_vials_and_coupons());
        assert!(standard_lane_gap() >= STANDARD_CUSTODY_GAP_MIN);

        for index in 0..TEMP_POCKET_COUNT {
            let (x, y) = temp_pocket_position(index);
            assert!(x.abs() + 14.0 < TEMP_BLOCK_X / 2.0);
            assert!(y.abs() + 30.0 < TEMP_BLOCK_Y / 2.0);
        }

        for index in 0..REFERENCE_VIAL_COUNT {
            let (x, y) = vial_position(index);
            assert!(x.abs() + VIAL_WELL_D / 2.0 + 10.0 < VIAL_NEST_X / 2.0);
            assert!(y.abs() + VIAL_WELL_D / 2.0 + 10.0 < VIAL_NEST_Y / 2.0);
        }
    }

    #[test]
    fn pressure_flow_and_restriction_channels_are_balanced() {
        assert_eq!(COUPON_LANES, 6);
        assert_eq!(PRESSURE_TAP_COUNT, FLOW_SENSOR_DOCKS * 2);
        assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
        assert!(CAPILLARY_SLOT_Y > CAPILLARY_SLOT_X * 3.0);
        assert!(PRESSURE_TAP_PITCH_X > PRESSURE_PORT_D * 5.0);
        assert!(TUBE_BORE_D > TUBE_OD);
    }

    #[test]
    fn released_hold_reject_status_lanes_are_separated() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(status_lane_name(0), "released");
        assert_eq!(status_lane_name(1), "hold");
        assert_eq!(status_lane_name(2), "reject");
        assert!(status_lane_x(0) < status_lane_x(1));
        assert!(status_lane_x(1) < status_lane_x(2));
        assert!(status_lane_gap() >= STATUS_LANE_MIN_GAP);

        let released = rect(
            (status_lane_x(0), STATUS_CENTER.1),
            STATUS_SLOT_Y,
            STATUS_PANEL_Y,
        );
        let hold = rect(
            (status_lane_x(1), STATUS_CENTER.1),
            STATUS_SLOT_Y,
            STATUS_PANEL_Y,
        );
        let reject = rect(
            (status_lane_x(2), STATUS_CENTER.1),
            STATUS_SLOT_Y,
            STATUS_PANEL_Y,
        );
        assert!(!rects_overlap(released, hold));
        assert!(!rects_overlap(hold, reject));
        assert!(!rects_overlap(released, reject));
    }

    #[test]
    fn traceability_evidence_and_keepouts_are_sized_for_service() {
        assert!(BARCODE_LANDS >= REFERENCE_VIAL_COUNT - 2);
        assert_eq!(CERTIFICATE_LANDS, 4);
        assert_eq!(RUN_RECORD_LANDS, 3);
        assert_eq!(EVIDENCE_CAMERA_COUNT, 3);
        assert!(bridge_clearance_above_deck() >= 180.0);
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 4);
        assert!(ROBOT_SWEEP_X < STATION_X);
        assert!(FRONT_SERVICE_CLEARANCE >= 320.0);
        assert!(REAR_SERVICE_CLEARANCE >= 240.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 130.0);
    }

    fn temp_block_center_is_between_vials_and_coupons() -> bool {
        VIAL_CENTER.0 < TEMP_CENTER.0
            && TEMP_CENTER.0 < COUPON_CENTER.0
            && (VIAL_CENTER.1 - TEMP_CENTER.1).abs() < TEMP_EQUILIBRATION_CLEARANCE
            && (COUPON_CENTER.1 - TEMP_CENTER.1).abs() < TEMP_EQUILIBRATION_CLEARANCE
    }
}
