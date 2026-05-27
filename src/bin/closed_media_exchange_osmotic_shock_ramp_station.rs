use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-exchange osmotic-shock ramp validation station.
//
// This generator packages mechanical validation hardware for a contained media
// exchange station: surrogate cassette docking, high/low osmolality bag nests,
// staged ramp routing, witness wells, custody lands, disposition gates, camera
// evidence features, and robot/service keepouts. It intentionally models only
// fixture geometry and evidence packaging. It is not a biological SOP, sterile
// process claim, assay method, pressure-rated fluid path, or acceptance
// criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_media_exchange_osmotic_shock_ramp_station_";

const OUTPUTS: [&str; 14] = [
    "output/closed_media_exchange_osmotic_shock_ramp_station_containment_deck.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_cassette_surrogate_dock.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_high_low_osmolality_media_bag_nests.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_staged_ramp_manifold.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_conductivity_osmolality_sample_wells.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_timed_exchange_token_rail.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_pressure_flow_shear_witness_taps.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_bubble_dead_volume_windows.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_waste_retain_split.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_barcode_coa_custody_lands.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_release_hold_reject_gates.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_camera_evidence_bridge.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_robot_service_keepouts.stl",
    "output/closed_media_exchange_osmotic_shock_ramp_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 25] = [
    "mechanical_validation_packaging_only",
    "cassette_surrogate_dock",
    "cassette_datum_pins",
    "cassette_interface_witness_lands",
    "high_osmolality_media_bag_nests",
    "low_osmolality_media_bag_nests",
    "bag_connector_parking",
    "staged_ramp_manifold",
    "ramp_stage_valve_bosses",
    "conductivity_sample_wells",
    "osmolality_sample_wells",
    "high_low_reference_wells",
    "timed_exchange_token_rail",
    "pressure_witness_taps",
    "flow_witness_taps",
    "shear_witness_coupons",
    "bubble_witness_windows",
    "dead_volume_windows",
    "waste_retain_split",
    "barcode_lands",
    "coa_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge_robot_service_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 5] = [
    "biological_sop",
    "sterile_process_claim",
    "assay_method",
    "pressure_rated_fluid_path",
    "acceptance_criterion",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 880.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1150.0;
const SUMP_Y: f64 = 690.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 20.0;
const DATUM_BOSSES: usize = 10;

const CASSETTE_DOCK_X: f64 = 360.0;
const CASSETTE_DOCK_Y: f64 = 230.0;
const CASSETTE_DOCK_Z: f64 = 54.0;
const CASSETTE_DOCK_POS: (f64, f64) = (-430.0, 245.0);
const CASSETTE_DATUM_PINS: usize = 6;
const CASSETTE_LATCHES: usize = 4;
const CASSETTE_INTERFACE_LANDS: usize = 8;
const CASSETTE_CHANNEL_D: f64 = 8.0;

const BAG_BANK_X: f64 = 570.0;
const BAG_BANK_Y: f64 = 230.0;
const BAG_BANK_Z: f64 = 46.0;
const BAG_BANK_POS: (f64, f64) = (170.0, 258.0);
const BAG_NESTS_PER_OSMOLALITY: usize = 3;
const TOTAL_BAG_NESTS: usize = BAG_NESTS_PER_OSMOLALITY * 2;
const BAG_NEST_X: f64 = 78.0;
const BAG_NEST_Y: f64 = 150.0;
const BAG_NEST_RECESS_Z: f64 = 15.0;
const BAG_PITCH_X: f64 = 86.0;
const BAG_LANE_PITCH_Y: f64 = 88.0;
const BAG_CONNECTOR_PARKS: usize = TOTAL_BAG_NESTS;

const MANIFOLD_X: f64 = 650.0;
const MANIFOLD_Y: f64 = 180.0;
const MANIFOLD_Z: f64 = 48.0;
const MANIFOLD_POS: (f64, f64) = (35.0, 42.0);
const RAMP_STAGES: usize = 6;
const RAMP_CHANNEL_D: f64 = 8.0;
const RAMP_PORT_D: f64 = 18.0;
const STAGE_PITCH_X: f64 = 82.0;
const MANIFOLD_VALVE_BOSSES: usize = RAMP_STAGES + 1;
const DWELL_LOOP_CUPS: usize = RAMP_STAGES;

const SAMPLE_BANK_X: f64 = 425.0;
const SAMPLE_BANK_Y: f64 = 174.0;
const SAMPLE_BANK_Z: f64 = 44.0;
const SAMPLE_BANK_POS: (f64, f64) = (420.0, -170.0);
const CONDUCTIVITY_WELLS: usize = 6;
const OSMOLALITY_WELLS: usize = 6;
const REFERENCE_WELLS: usize = 4;
const SAMPLE_WELL_D: f64 = 27.0;
const SAMPLE_WELL_PITCH_X: f64 = 48.0;
const SAMPLE_ROW_PITCH_Y: f64 = 58.0;

const TOKEN_RAIL_X: f64 = 370.0;
const TOKEN_RAIL_Y: f64 = 150.0;
const TOKEN_RAIL_Z: f64 = 36.0;
const TOKEN_RAIL_POS: (f64, f64) = (-430.0, -8.0);
const TIMED_EXCHANGE_TOKENS: usize = 8;
const TOKEN_PITCH_X: f64 = 42.0;
const TOKEN_SLOT_X: f64 = 28.0;
const TOKEN_SLOT_Y: f64 = 34.0;
const TOKEN_SLOT_Z: f64 = 10.0;

const TAP_BANK_X: f64 = 520.0;
const TAP_BANK_Y: f64 = 126.0;
const TAP_BANK_Z: f64 = 42.0;
const TAP_BANK_POS: (f64, f64) = (-75.0, -185.0);
const PRESSURE_TAPS: usize = 4;
const FLOW_TAPS: usize = 4;
const SHEAR_COUPONS: usize = 6;
const TAP_BOSS_D: f64 = 32.0;
const TAP_BORE_D: f64 = 6.2;
const TAP_PITCH_X: f64 = 62.0;
const SHEAR_COUPON_X: f64 = 44.0;
const SHEAR_COUPON_Y: f64 = 28.0;

const WINDOW_BANK_X: f64 = 362.0;
const WINDOW_BANK_Y: f64 = 150.0;
const WINDOW_BANK_Z: f64 = 34.0;
const WINDOW_BANK_POS: (f64, f64) = (-430.0, -286.0);
const BUBBLE_WINDOWS: usize = 6;
const DEAD_VOLUME_WINDOWS: usize = 5;
const WINDOW_D: f64 = 29.0;
const WINDOW_PITCH_X: f64 = 46.0;

const SPLIT_TRAY_X: f64 = 330.0;
const SPLIT_TRAY_Y: f64 = 122.0;
const SPLIT_TRAY_Z: f64 = 48.0;
const SPLIT_TRAY_POS: (f64, f64) = (56.0, -332.0);
const WASTE_CELLS: usize = 4;
const RETAIN_CELLS: usize = 4;
const SPLIT_CELL_X: f64 = 56.0;
const SPLIT_CELL_Y: f64 = 42.0;

const CUSTODY_X: f64 = 382.0;
const CUSTODY_Y: f64 = 116.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (438.0, 332.0);
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 4;
const CUSTODY_SEAL_PADS: usize = 5;

const GATE_BANK_X: f64 = 300.0;
const GATE_BANK_Y: f64 = 170.0;
const GATE_BANK_Z: f64 = 38.0;
const GATE_BANK_POS: (f64, f64) = (432.0, 30.0);
const DISPOSITION_GATES: usize = 3;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;
const GATE_TOKEN_SLOTS: usize = 9;
const GATE_PITCH_Y: f64 = 52.0;

const CAMERA_BRIDGE_X: f64 = 990.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 214.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (-12.0, -42.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;

const ROBOT_KEEPOUT_X: f64 = 1160.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 76.0;
const SERVICE_KEEPOUT_X: f64 = 96.0;
const SERVICE_KEEPOUT_Y: f64 = 690.0;
const SERVICE_KEEPOUT_Z: f64 = 94.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 310.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(cassette_surrogate_dock(), OUTPUTS[1]);
    write_part(high_low_osmolality_media_bag_nests(), OUTPUTS[2]);
    write_part(staged_ramp_manifold(), OUTPUTS[3]);
    write_part(conductivity_osmolality_sample_wells(), OUTPUTS[4]);
    write_part(timed_exchange_token_rail(), OUTPUTS[5]);
    write_part(pressure_flow_shear_witness_taps(), OUTPUTS[6]);
    write_part(bubble_dead_volume_windows(), OUTPUTS[7]);
    write_part(waste_retain_split(), OUTPUTS[8]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[9]);
    write_part(release_hold_reject_gates(), OUTPUTS[10]);
    write_part(camera_evidence_bridge(), OUTPUTS[11]);
    write_part(robot_service_keepouts(), OUTPUTS[12]);
    write_part(station_assembly(), OUTPUTS[13]);

    println!(
        "Closed media-exchange osmotic-shock ramp station: {:.0}mm x {:.0}mm contained deck, {} ramp stages, {} high/low bag nests.",
        DECK_X, DECK_Y, RAMP_STAGES, TOTAL_BAG_NESTS
    );
    println!(
        "Mechanical validation packaging only: {} conductivity wells, {} osmolality wells, {} timed exchange tokens, {} pressure/flow taps, {} shear coupons.",
        CONDUCTIVITY_WELLS,
        OSMOLALITY_WELLS,
        TIMED_EXCHANGE_TOKENS,
        PRESSURE_TAPS + FLOW_TAPS,
        SHEAR_COUPONS
    );
    println!(
        "Evidence and custody: {} bubble windows, {} dead-volume windows, {} barcode lands, {} COA lands, release/hold/reject gates, top service clearance {:.0}mm.",
        BUBBLE_WINDOWS,
        DEAD_VOLUME_WINDOWS,
        BARCODE_LANDS,
        COA_LANDS,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "osmotic_shock_ramp_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "osmotic_shock_ramp_station_recessed_spill_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z + 1.0,
    )
    .translate(0.0, -4.0, DECK_Z / 2.0 - SUMP_Z / 2.0);
    let drain = centered_cylinder(
        "osmotic_shock_ramp_station_sump_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 34.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 12.0, 0.0);

    deck - sump - drain + containment_rim() + deck_datums() + landing_pockets()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        "osmotic_shock_ramp_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, rim_z());
    let rear = centered_cube(
        "osmotic_shock_ramp_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let left = centered_cube(
        "osmotic_shock_ramp_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_z());
    let right = centered_cube(
        "osmotic_shock_ramp_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());

    front + rear + left + right
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("osmotic_shock_ramp_station_deck_datums");
    for i in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 82.0 + i as f64 * ((DECK_X - 164.0) / 9.0);
        let y = if i % 2 == 0 {
            DECK_Y / 2.0 - 72.0
        } else {
            -DECK_Y / 2.0 + 72.0
        };
        let pad = centered_cylinder(
            format!("osmotic_shock_ramp_station_datum_pad_{i}"),
            13.0,
            6.0,
            36,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("osmotic_shock_ramp_station_datum_bore_{i}"),
            4.0,
            8.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        datums = datums + (pad - bore);
    }
    datums
}

fn landing_pockets() -> Part {
    landing_pocket(
        "cassette_surrogate_dock_land",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_POS,
    ) + landing_pocket(
        "high_low_bag_nest_land",
        BAG_BANK_X,
        BAG_BANK_Y,
        BAG_BANK_POS,
    ) + landing_pocket(
        "staged_ramp_manifold_land",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_POS,
    ) + landing_pocket(
        "conductivity_osmolality_well_land",
        SAMPLE_BANK_X,
        SAMPLE_BANK_Y,
        SAMPLE_BANK_POS,
    ) + landing_pocket(
        "timed_exchange_token_rail_land",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_POS,
    ) + landing_pocket(
        "pressure_flow_shear_tap_land",
        TAP_BANK_X,
        TAP_BANK_Y,
        TAP_BANK_POS,
    ) + landing_pocket(
        "bubble_dead_volume_window_land",
        WINDOW_BANK_X,
        WINDOW_BANK_Y,
        WINDOW_BANK_POS,
    ) + landing_pocket(
        "waste_retain_split_land",
        SPLIT_TRAY_X,
        SPLIT_TRAY_Y,
        SPLIT_TRAY_POS,
    ) + landing_pocket("custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pocket(
            "release_hold_reject_gate_land",
            GATE_BANK_X,
            GATE_BANK_Y,
            GATE_BANK_POS,
        )
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("osmotic_shock_ramp_station_{name}"),
        x + 14.0,
        y + 14.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn cassette_surrogate_dock() -> Part {
    let base = centered_cube(
        "osmotic_shock_ramp_station_cassette_surrogate_dock_base",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let cassette_recess = centered_cube(
        "osmotic_shock_ramp_station_cassette_surrogate_recess",
        CASSETTE_DOCK_X - 70.0,
        CASSETTE_DOCK_Y - 62.0,
        22.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 - 7.0);
    let gasket_witness = centered_cube(
        "osmotic_shock_ramp_station_cassette_gasket_witness_land",
        CASSETTE_DOCK_X - 44.0,
        CASSETTE_DOCK_Y - 40.0,
        8.0,
    ) - centered_cube(
        "osmotic_shock_ramp_station_cassette_gasket_inner_clearance",
        CASSETTE_DOCK_X - 86.0,
        CASSETTE_DOCK_Y - 80.0,
        10.0,
    );

    let mut dock = base - cassette_recess
        + gasket_witness.translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 5.0)
        + cassette_datum_pins()
        + cassette_latches()
        + cassette_interface_lands();

    for i in 0..2 {
        let y = -42.0 + i as f64 * 84.0;
        let channel = centered_cylinder(
            format!("osmotic_shock_ramp_station_cassette_surrogate_channel_{i}"),
            CASSETTE_CHANNEL_D / 2.0,
            CASSETTE_DOCK_X - 110.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, CASSETTE_DOCK_Z / 2.0 + 16.0);
        dock = dock + channel;
    }

    dock
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("osmotic_shock_ramp_station_cassette_datum_pins");
    for i in 0..CASSETTE_DATUM_PINS {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let row = i / 2;
        let pin = centered_cylinder(
            format!("osmotic_shock_ramp_station_cassette_datum_pin_{i}"),
            8.0,
            16.0,
            32,
        )
        .translate(
            side * (CASSETTE_DOCK_X / 2.0 - 44.0),
            -62.0 + row as f64 * 62.0,
            CASSETTE_DOCK_Z / 2.0 + 8.0,
        );
        pins = pins + pin;
    }
    pins
}

fn cassette_latches() -> Part {
    let mut latches = Part::empty("osmotic_shock_ramp_station_cassette_latches");
    for i in 0..CASSETTE_LATCHES {
        let x = if i < 2 { -110.0 } else { 110.0 };
        let y = if i % 2 == 0 {
            -CASSETTE_DOCK_Y / 2.0 + 20.0
        } else {
            CASSETTE_DOCK_Y / 2.0 - 20.0
        };
        latches = latches
            + centered_cube(
                format!("osmotic_shock_ramp_station_cassette_latch_witness_{i}"),
                64.0,
                16.0,
                14.0,
            )
            .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 10.0);
    }
    latches
}

fn cassette_interface_lands() -> Part {
    let mut lands = Part::empty("osmotic_shock_ramp_station_cassette_interface_lands");
    for i in 0..CASSETTE_INTERFACE_LANDS {
        let x = port_x(i, CASSETTE_INTERFACE_LANDS, 33.0);
        let land = centered_cube(
            format!("osmotic_shock_ramp_station_cassette_interface_witness_land_{i}"),
            22.0,
            18.0,
            5.0,
        )
        .translate(
            x,
            -CASSETTE_DOCK_Y / 2.0 + 44.0,
            CASSETTE_DOCK_Z / 2.0 + 4.0,
        );
        lands = lands + land;
    }
    lands
}

fn high_low_osmolality_media_bag_nests() -> Part {
    let base = centered_cube(
        "osmotic_shock_ramp_station_high_low_osmolality_media_bag_nest_bank",
        BAG_BANK_X,
        BAG_BANK_Y,
        BAG_BANK_Z,
    );
    let divider = centered_cube(
        "osmotic_shock_ramp_station_high_low_osmolality_divider",
        BAG_BANK_X - 38.0,
        9.0,
        18.0,
    )
    .translate(0.0, 0.0, BAG_BANK_Z / 2.0 + 7.0);

    base - bag_recesses() + divider + bag_clips() + bag_connector_parking()
}

fn bag_recesses() -> Part {
    let mut recesses = Part::empty("osmotic_shock_ramp_station_bag_recesses");
    for i in 0..TOTAL_BAG_NESTS {
        let (x, y) = bag_nest_center(i);
        recesses = recesses
            + centered_cube(
                format!("osmotic_shock_ramp_station_osmolality_bag_recess_{i}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_NEST_RECESS_Z + 2.0,
            )
            .translate(x, y, BAG_BANK_Z / 2.0 - BAG_NEST_RECESS_Z / 2.0);
    }
    recesses
}

fn bag_clips() -> Part {
    let mut clips = Part::empty("osmotic_shock_ramp_station_bag_edge_clips");
    for i in 0..TOTAL_BAG_NESTS {
        let (x, y) = bag_nest_center(i);
        let lane_name = if i < BAG_NESTS_PER_OSMOLALITY {
            "low"
        } else {
            "high"
        };
        clips = clips
            + centered_cube(
                format!("osmotic_shock_ramp_station_{lane_name}_bag_front_clip_{i}"),
                BAG_NEST_X,
                10.0,
                14.0,
            )
            .translate(x, y - BAG_NEST_Y / 2.0 + 8.0, BAG_BANK_Z / 2.0 + 8.0)
            + centered_cube(
                format!("osmotic_shock_ramp_station_{lane_name}_bag_rear_clip_{i}"),
                BAG_NEST_X,
                10.0,
                14.0,
            )
            .translate(x, y + BAG_NEST_Y / 2.0 - 8.0, BAG_BANK_Z / 2.0 + 8.0);
    }
    clips
}

fn bag_connector_parking() -> Part {
    let mut parks = Part::empty("osmotic_shock_ramp_station_bag_connector_parking_lands");
    for i in 0..BAG_CONNECTOR_PARKS {
        let (x, y) = bag_nest_center(i);
        let park = centered_cylinder(
            format!("osmotic_shock_ramp_station_bag_connector_parking_boss_{i}"),
            13.0,
            8.0,
            32,
        )
        .translate(
            x + 27.0,
            y - BAG_NEST_Y / 2.0 - 12.0,
            BAG_BANK_Z / 2.0 + 4.0,
        );
        let bore = centered_cylinder(
            format!("osmotic_shock_ramp_station_bag_connector_parking_bore_{i}"),
            4.2,
            10.0,
            20,
        )
        .translate(
            x + 27.0,
            y - BAG_NEST_Y / 2.0 - 12.0,
            BAG_BANK_Z / 2.0 + 4.0,
        );
        parks = parks + (park - bore);
    }
    parks
}

fn staged_ramp_manifold() -> Part {
    let base = centered_cube(
        "osmotic_shock_ramp_station_staged_ramp_manifold_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let main_channel = centered_cylinder(
        "osmotic_shock_ramp_station_ramp_main_channel_witness",
        RAMP_CHANNEL_D / 2.0,
        MANIFOLD_X - 62.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -18.0, MANIFOLD_Z / 2.0 + 4.0);
    let bypass_channel = centered_cylinder(
        "osmotic_shock_ramp_station_ramp_bypass_channel_witness",
        RAMP_CHANNEL_D / 2.0,
        MANIFOLD_X - 120.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 42.0, MANIFOLD_Z / 2.0 + 4.0);

    base + main_channel
        + bypass_channel
        + ramp_stage_valve_bosses()
        + dwell_loop_cups()
        + ramp_stage_step_comb()
}

fn ramp_stage_valve_bosses() -> Part {
    let mut bosses = Part::empty("osmotic_shock_ramp_station_ramp_stage_valve_bosses");
    for i in 0..MANIFOLD_VALVE_BOSSES {
        let x = valve_stage_x(i);
        let boss = centered_cylinder(
            format!("osmotic_shock_ramp_station_ramp_stage_valve_boss_{i}"),
            RAMP_PORT_D / 2.0,
            12.0,
            36,
        )
        .translate(x, -18.0, MANIFOLD_Z / 2.0 + 7.0);
        let bore = centered_cylinder(
            format!("osmotic_shock_ramp_station_ramp_stage_valve_bore_{i}"),
            3.8,
            14.0,
            22,
        )
        .translate(x, -18.0, MANIFOLD_Z / 2.0 + 7.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn dwell_loop_cups() -> Part {
    let mut cups = Part::empty("osmotic_shock_ramp_station_dwell_loop_cups");
    for i in 0..DWELL_LOOP_CUPS {
        let x = ramp_stage_x(i);
        let cup = centered_cylinder(
            format!("osmotic_shock_ramp_station_ramp_dwell_loop_visible_cup_{i}"),
            18.0 + i as f64 * 1.5,
            6.0,
            36,
        )
        .translate(x, 48.0, MANIFOLD_Z / 2.0 + 4.0);
        let inner = centered_cylinder(
            format!("osmotic_shock_ramp_station_ramp_dwell_loop_inner_mark_{i}"),
            9.0 + i as f64,
            8.0,
            28,
        )
        .translate(x, 48.0, MANIFOLD_Z / 2.0 + 4.0);
        cups = cups + (cup - inner);
    }
    cups
}

fn ramp_stage_step_comb() -> Part {
    let mut comb = Part::empty("osmotic_shock_ramp_station_osmolality_ramp_step_comb");
    for i in 0..RAMP_STAGES {
        comb = comb
            + centered_cube(
                format!("osmotic_shock_ramp_station_ramp_stage_height_token_{i}"),
                18.0,
                50.0,
                6.0 + i as f64 * 3.0,
            )
            .translate(
                ramp_stage_x(i),
                -MANIFOLD_Y / 2.0 + 22.0,
                MANIFOLD_Z / 2.0 + 3.0 + i as f64 * 1.5,
            );
    }
    comb
}

fn conductivity_osmolality_sample_wells() -> Part {
    let base = centered_cube(
        "osmotic_shock_ramp_station_conductivity_osmolality_sample_well_bank",
        SAMPLE_BANK_X,
        SAMPLE_BANK_Y,
        SAMPLE_BANK_Z,
    );
    let divider = centered_cube(
        "osmotic_shock_ramp_station_conductivity_osmolality_row_divider",
        SAMPLE_BANK_X - 36.0,
        7.0,
        16.0,
    )
    .translate(0.0, 0.0, SAMPLE_BANK_Z / 2.0 + 6.0);

    base - sample_well_cuts() + sample_well_collars() + divider + probe_identity_ticks()
}

fn sample_well_cuts() -> Part {
    let mut cuts = Part::empty("osmotic_shock_ramp_station_sample_well_cuts");
    for i in 0..CONDUCTIVITY_WELLS {
        let (x, y) = sample_well_center(i, CONDUCTIVITY_WELLS, SAMPLE_ROW_PITCH_Y / 2.0);
        cuts = cuts
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_conductivity_sample_well_cut_{i}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_BANK_Z + 8.0,
                36,
            )
            .translate(x, y, 0.0);
    }
    for i in 0..OSMOLALITY_WELLS {
        let (x, y) = sample_well_center(i, OSMOLALITY_WELLS, -SAMPLE_ROW_PITCH_Y / 2.0);
        cuts = cuts
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_osmolality_sample_well_cut_{i}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_BANK_Z + 8.0,
                36,
            )
            .translate(x, y, 0.0);
    }
    for i in 0..REFERENCE_WELLS {
        let x = -SAMPLE_BANK_X / 2.0 + 34.0 + i as f64 * 36.0;
        cuts = cuts
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_high_low_reference_well_cut_{i}"),
                10.5,
                SAMPLE_BANK_Z + 8.0,
                28,
            )
            .translate(x, 0.0, 0.0);
    }
    cuts
}

fn sample_well_collars() -> Part {
    let mut collars = Part::empty("osmotic_shock_ramp_station_sample_well_collars");
    for i in 0..CONDUCTIVITY_WELLS {
        let (x, y) = sample_well_center(i, CONDUCTIVITY_WELLS, SAMPLE_ROW_PITCH_Y / 2.0);
        collars = collars
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_conductivity_sample_well_collar_{i}"),
                SAMPLE_WELL_D / 2.0 + 4.0,
                5.0,
                36,
            )
            .translate(x, y, SAMPLE_BANK_Z / 2.0 + 3.0);
    }
    for i in 0..OSMOLALITY_WELLS {
        let (x, y) = sample_well_center(i, OSMOLALITY_WELLS, -SAMPLE_ROW_PITCH_Y / 2.0);
        collars = collars
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_osmolality_sample_well_collar_{i}"),
                SAMPLE_WELL_D / 2.0 + 4.0,
                5.0,
                36,
            )
            .translate(x, y, SAMPLE_BANK_Z / 2.0 + 3.0);
    }
    collars
}

fn probe_identity_ticks() -> Part {
    let mut ticks = Part::empty("osmotic_shock_ramp_station_probe_identity_ticks");
    for i in 0..CONDUCTIVITY_WELLS {
        let (x, y) = sample_well_center(i, CONDUCTIVITY_WELLS, SAMPLE_ROW_PITCH_Y / 2.0);
        ticks = ticks
            + centered_cube(
                format!("osmotic_shock_ramp_station_conductivity_probe_tick_{i}"),
                20.0,
                4.0,
                4.0,
            )
            .translate(x, y + 23.0, SAMPLE_BANK_Z / 2.0 + 3.0);
    }
    for i in 0..OSMOLALITY_WELLS {
        let (x, y) = sample_well_center(i, OSMOLALITY_WELLS, -SAMPLE_ROW_PITCH_Y / 2.0);
        ticks = ticks
            + centered_cube(
                format!("osmotic_shock_ramp_station_osmolality_probe_tick_{i}"),
                12.0,
                4.0,
                4.0,
            )
            .translate(x, y - 23.0, SAMPLE_BANK_Z / 2.0 + 3.0);
    }
    ticks
}

fn timed_exchange_token_rail() -> Part {
    let rail = centered_cube(
        "osmotic_shock_ramp_station_timed_exchange_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let sight_slot = centered_cube(
        "osmotic_shock_ramp_station_timed_exchange_sight_slot",
        TOKEN_RAIL_X - 52.0,
        18.0,
        16.0,
    )
    .translate(0.0, -8.0, TOKEN_RAIL_Z / 2.0);

    rail - token_slot_cuts() - sight_slot + timed_exchange_tokens() + timing_comb()
}

fn token_slot_cuts() -> Part {
    let mut cuts = Part::empty("osmotic_shock_ramp_station_timed_exchange_token_slot_cuts");
    for i in 0..TIMED_EXCHANGE_TOKENS {
        cuts = cuts
            + centered_cube(
                format!("osmotic_shock_ramp_station_timed_exchange_token_slot_{i}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_SLOT_Z,
            )
            .translate(token_x(i), 28.0, TOKEN_RAIL_Z / 2.0 - TOKEN_SLOT_Z / 2.0);
    }
    cuts
}

fn timed_exchange_tokens() -> Part {
    let mut tokens = Part::empty("osmotic_shock_ramp_station_timed_exchange_tokens");
    for i in 0..TIMED_EXCHANGE_TOKENS {
        tokens = tokens
            + centered_cube(
                format!("osmotic_shock_ramp_station_timed_exchange_token_{i}"),
                TOKEN_SLOT_X - 5.0,
                TOKEN_SLOT_Y - 5.0,
                6.0,
            )
            .translate(token_x(i), 28.0, TOKEN_RAIL_Z / 2.0 + 5.0);
    }
    tokens
}

fn timing_comb() -> Part {
    let mut comb = Part::empty("osmotic_shock_ramp_station_timed_exchange_step_comb");
    for i in 0..TIMED_EXCHANGE_TOKENS {
        comb = comb
            + centered_cube(
                format!("osmotic_shock_ramp_station_timed_exchange_duration_tooth_{i}"),
                9.0,
                40.0,
                5.0 + i as f64 * 1.5,
            )
            .translate(
                token_x(i),
                -TOKEN_RAIL_Y / 2.0 + 24.0,
                TOKEN_RAIL_Z / 2.0 + 2.5 + i as f64 * 0.75,
            );
    }
    comb
}

fn pressure_flow_shear_witness_taps() -> Part {
    let base = centered_cube(
        "osmotic_shock_ramp_station_pressure_flow_shear_tap_bank",
        TAP_BANK_X,
        TAP_BANK_Y,
        TAP_BANK_Z,
    );

    base + pressure_flow_tap_bosses() - pressure_flow_tap_bores() + shear_witness_coupons()
}

fn pressure_flow_tap_bosses() -> Part {
    let mut bosses = Part::empty("osmotic_shock_ramp_station_pressure_flow_tap_bosses");
    for i in 0..PRESSURE_TAPS {
        bosses = bosses + tap_boss("pressure", i, PRESSURE_TAPS, 30.0);
    }
    for i in 0..FLOW_TAPS {
        bosses = bosses + tap_boss("flow", i, FLOW_TAPS, -30.0);
    }
    bosses
}

fn pressure_flow_tap_bores() -> Part {
    let mut bores = Part::empty("osmotic_shock_ramp_station_pressure_flow_tap_bores");
    for i in 0..PRESSURE_TAPS {
        bores = bores + tap_bore("pressure", i, PRESSURE_TAPS, 30.0);
    }
    for i in 0..FLOW_TAPS {
        bores = bores + tap_bore("flow", i, FLOW_TAPS, -30.0);
    }
    bores
}

fn tap_boss(prefix: &str, index: usize, count: usize, y: f64) -> Part {
    centered_cylinder(
        format!("osmotic_shock_ramp_station_{prefix}_witness_tap_boss_{index}"),
        TAP_BOSS_D / 2.0,
        10.0,
        36,
    )
    .translate(port_x(index, count, TAP_PITCH_X), y, TAP_BANK_Z / 2.0 + 5.0)
}

fn tap_bore(prefix: &str, index: usize, count: usize, y: f64) -> Part {
    centered_cylinder(
        format!("osmotic_shock_ramp_station_{prefix}_witness_tap_bore_{index}"),
        TAP_BORE_D / 2.0,
        TAP_BANK_Z + 12.0,
        24,
    )
    .translate(port_x(index, count, TAP_PITCH_X), y, 0.0)
}

fn shear_witness_coupons() -> Part {
    let mut coupons = Part::empty("osmotic_shock_ramp_station_shear_witness_coupons");
    for i in 0..SHEAR_COUPONS {
        let x = port_x(i, SHEAR_COUPONS, 58.0);
        let coupon = centered_cube(
            format!("osmotic_shock_ramp_station_shear_witness_coupon_{i}"),
            SHEAR_COUPON_X,
            SHEAR_COUPON_Y,
            8.0,
        )
        .translate(x, -TAP_BANK_Y / 2.0 + 16.0, TAP_BANK_Z / 2.0 + 5.0);
        let groove = centered_cube(
            format!("osmotic_shock_ramp_station_shear_witness_coupon_groove_{i}"),
            SHEAR_COUPON_X - 12.0,
            5.0,
            10.0,
        )
        .translate(x, -TAP_BANK_Y / 2.0 + 16.0, TAP_BANK_Z / 2.0 + 5.0);
        coupons = coupons + (coupon - groove);
    }
    coupons
}

fn bubble_dead_volume_windows() -> Part {
    let frame = centered_cube(
        "osmotic_shock_ramp_station_bubble_dead_volume_window_frame",
        WINDOW_BANK_X,
        WINDOW_BANK_Y,
        WINDOW_BANK_Z,
    );
    let backlight_slot = centered_cube(
        "osmotic_shock_ramp_station_window_backlight_slot",
        WINDOW_BANK_X - 50.0,
        14.0,
        14.0,
    )
    .translate(0.0, 0.0, WINDOW_BANK_Z / 2.0);

    frame - bubble_window_cuts() - dead_volume_window_cuts() - backlight_slot
        + window_bezels()
        + dead_volume_ticks()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty("osmotic_shock_ramp_station_bubble_window_cuts");
    for i in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_bubble_witness_window_cut_{i}"),
                WINDOW_D / 2.0,
                WINDOW_BANK_Z + 8.0,
                36,
            )
            .translate(port_x(i, BUBBLE_WINDOWS, WINDOW_PITCH_X), 34.0, 0.0);
    }
    cuts
}

fn dead_volume_window_cuts() -> Part {
    let mut cuts = Part::empty("osmotic_shock_ramp_station_dead_volume_window_cuts");
    for i in 0..DEAD_VOLUME_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("osmotic_shock_ramp_station_dead_volume_window_cut_{i}"),
                30.0,
                22.0,
                WINDOW_BANK_Z + 8.0,
            )
            .translate(port_x(i, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X), -32.0, 0.0);
    }
    cuts
}

fn window_bezels() -> Part {
    let mut bezels = Part::empty("osmotic_shock_ramp_station_window_bezels");
    for i in 0..BUBBLE_WINDOWS {
        bezels = bezels
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_bubble_witness_window_bezel_{i}"),
                WINDOW_D / 2.0 + 4.0,
                5.0,
                36,
            )
            .translate(
                port_x(i, BUBBLE_WINDOWS, WINDOW_PITCH_X),
                34.0,
                WINDOW_BANK_Z / 2.0 + 3.0,
            );
    }
    for i in 0..DEAD_VOLUME_WINDOWS {
        bezels = bezels
            + centered_cube(
                format!("osmotic_shock_ramp_station_dead_volume_window_bezel_{i}"),
                38.0,
                30.0,
                5.0,
            )
            .translate(
                port_x(i, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X),
                -32.0,
                WINDOW_BANK_Z / 2.0 + 3.0,
            );
    }
    bezels
}

fn dead_volume_ticks() -> Part {
    let mut ticks = Part::empty("osmotic_shock_ramp_station_dead_volume_tick_marks");
    for i in 0..DEAD_VOLUME_WINDOWS {
        ticks = ticks
            + centered_cube(
                format!("osmotic_shock_ramp_station_dead_volume_scale_tick_{i}"),
                5.0 + i as f64 * 4.0,
                5.0,
                4.0,
            )
            .translate(
                port_x(i, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X),
                -58.0,
                WINDOW_BANK_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn waste_retain_split() -> Part {
    let tray = centered_cube(
        "osmotic_shock_ramp_station_waste_retain_split_tray",
        SPLIT_TRAY_X,
        SPLIT_TRAY_Y,
        SPLIT_TRAY_Z,
    );
    let center_weir = centered_cube(
        "osmotic_shock_ramp_station_waste_retain_split_weir",
        SPLIT_TRAY_X - 26.0,
        8.0,
        28.0,
    )
    .translate(0.0, 0.0, SPLIT_TRAY_Z / 2.0 + 8.0);
    let drain = centered_cylinder(
        "osmotic_shock_ramp_station_waste_retain_split_waste_drain",
        6.0,
        34.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SPLIT_TRAY_X / 2.0 - 36.0, -SPLIT_TRAY_Y / 2.0 - 4.0, 3.0);

    tray - split_cell_cuts() - drain + split_cell_lips() + center_weir + retain_clip_lands()
}

fn split_cell_cuts() -> Part {
    let mut cuts = Part::empty("osmotic_shock_ramp_station_waste_retain_cell_cuts");
    for i in 0..WASTE_CELLS {
        cuts = cuts
            + centered_cube(
                format!("osmotic_shock_ramp_station_waste_cell_cut_{i}"),
                SPLIT_CELL_X,
                SPLIT_CELL_Y,
                SPLIT_TRAY_Z + 8.0,
            )
            .translate(port_x(i, WASTE_CELLS, 64.0), -32.0, 0.0);
    }
    for i in 0..RETAIN_CELLS {
        cuts = cuts
            + centered_cube(
                format!("osmotic_shock_ramp_station_retain_cell_cut_{i}"),
                SPLIT_CELL_X,
                SPLIT_CELL_Y,
                SPLIT_TRAY_Z + 8.0,
            )
            .translate(port_x(i, RETAIN_CELLS, 64.0), 32.0, 0.0);
    }
    cuts
}

fn split_cell_lips() -> Part {
    let mut lips = Part::empty("osmotic_shock_ramp_station_waste_retain_cell_lips");
    for i in 0..WASTE_CELLS {
        lips = lips
            + centered_cube(
                format!("osmotic_shock_ramp_station_waste_cell_label_lip_{i}"),
                SPLIT_CELL_X,
                7.0,
                5.0,
            )
            .translate(
                port_x(i, WASTE_CELLS, 64.0),
                -58.0,
                SPLIT_TRAY_Z / 2.0 + 3.0,
            );
    }
    for i in 0..RETAIN_CELLS {
        lips = lips
            + centered_cube(
                format!("osmotic_shock_ramp_station_retain_cell_label_lip_{i}"),
                SPLIT_CELL_X,
                7.0,
                5.0,
            )
            .translate(
                port_x(i, RETAIN_CELLS, 64.0),
                58.0,
                SPLIT_TRAY_Z / 2.0 + 3.0,
            );
    }
    lips
}

fn retain_clip_lands() -> Part {
    let mut lands = Part::empty("osmotic_shock_ramp_station_retain_clip_lands");
    for i in 0..RETAIN_CELLS {
        lands = lands
            + centered_cube(
                format!("osmotic_shock_ramp_station_retain_split_clip_land_{i}"),
                30.0,
                12.0,
                8.0,
            )
            .translate(port_x(i, RETAIN_CELLS, 64.0), 8.0, SPLIT_TRAY_Z / 2.0 + 5.0);
    }
    lands
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "osmotic_shock_ramp_station_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel + barcode_lands() + coa_custody_lands() + custody_seal_pads()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("osmotic_shock_ramp_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("osmotic_shock_ramp_station_barcode_land_{i}"),
                72.0,
                18.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 50.0 + (i % 4) as f64 * 88.0,
                30.0 - (i / 4) as f64 * 30.0,
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn coa_custody_lands() -> Part {
    let mut lands = Part::empty("osmotic_shock_ramp_station_coa_custody_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("osmotic_shock_ramp_station_coa_custody_land_{i}"),
                58.0,
                30.0,
                5.0,
            )
            .translate(-132.0 + i as f64 * 88.0, -36.0, CUSTODY_Z / 2.0 + 3.5);
    }
    lands
}

fn custody_seal_pads() -> Part {
    let mut pads = Part::empty("osmotic_shock_ramp_station_custody_seal_pads");
    for i in 0..CUSTODY_SEAL_PADS {
        pads = pads
            + centered_cylinder(
                format!("osmotic_shock_ramp_station_custody_seal_pad_{i}"),
                8.0,
                4.0,
                28,
            )
            .translate(
                -150.0 + i as f64 * 75.0,
                CUSTODY_Y / 2.0 - 16.0,
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    pads
}

fn release_hold_reject_gates() -> Part {
    let bank = centered_cube(
        "osmotic_shock_ramp_station_release_hold_reject_gate_bank",
        GATE_BANK_X,
        GATE_BANK_Y,
        GATE_BANK_Z,
    );
    let lane_recesses = disposition_gate_recesses();
    let gate_flags = disposition_gate_flags();
    let slots = disposition_gate_token_slots();

    bank - lane_recesses - slots + gate_flags
}

fn disposition_gate_recesses() -> Part {
    let mut recesses = Part::empty("osmotic_shock_ramp_station_disposition_gate_recesses");
    for gate in 0..DISPOSITION_GATES {
        recesses = recesses
            + centered_cube(
                format!(
                    "osmotic_shock_ramp_station_{}_gate_recess",
                    disposition_gate_name(gate)
                ),
                GATE_BANK_X - 44.0,
                28.0,
                GATE_BANK_Z + 8.0,
            )
            .translate(0.0, gate_y(gate), 0.0);
    }
    recesses
}

fn disposition_gate_flags() -> Part {
    let mut flags = Part::empty("osmotic_shock_ramp_station_disposition_gate_flags");
    for gate in 0..DISPOSITION_GATES {
        flags = flags
            + centered_cube(
                format!(
                    "osmotic_shock_ramp_station_{}_gate_positive_stop",
                    disposition_gate_name(gate)
                ),
                24.0,
                26.0,
                24.0,
            )
            .translate(
                GATE_BANK_X / 2.0 - 34.0,
                gate_y(gate),
                GATE_BANK_Z / 2.0 + 12.0,
            );
    }
    flags
}

fn disposition_gate_token_slots() -> Part {
    let mut slots = Part::empty("osmotic_shock_ramp_station_disposition_gate_token_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        let gate = i % DISPOSITION_GATES;
        let row = i / DISPOSITION_GATES;
        slots = slots
            + centered_cube(
                format!("osmotic_shock_ramp_station_disposition_token_slot_{i}"),
                28.0,
                12.0,
                10.0,
            )
            .translate(
                -GATE_BANK_X / 2.0 + 42.0 + row as f64 * 36.0,
                gate_y(gate),
                GATE_BANK_Z / 2.0 - 5.0,
            );
    }
    slots
}

fn camera_evidence_bridge() -> Part {
    let bridge = centered_cube(
        "osmotic_shock_ramp_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let left_foot = centered_cube(
        "osmotic_shock_ramp_station_camera_evidence_bridge_left_foot",
        44.0,
        72.0,
        34.0,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 40.0, 0.0, 17.0);
    let right_foot = centered_cube(
        "osmotic_shock_ramp_station_camera_evidence_bridge_right_foot",
        44.0,
        72.0,
        34.0,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 40.0, 0.0, 17.0);

    bridge + left_foot + right_foot + camera_mounts() + evidence_fiducials()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("osmotic_shock_ramp_station_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("osmotic_shock_ramp_station_camera_mount_plate_{i}"),
                64.0,
                12.0,
                36.0,
            )
            .translate(
                port_x(i, CAMERA_MOUNTS, 190.0),
                -CAMERA_BRIDGE_Y / 2.0 - 7.0,
                CAMERA_BRIDGE_Z - 30.0,
            );
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("osmotic_shock_ramp_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -CAMERA_BRIDGE_X / 2.0 + 70.0 + i as f64 * ((CAMERA_BRIDGE_X - 140.0) / 9.0);
        let fid = centered_cylinder(
            format!("osmotic_shock_ramp_station_evidence_fiducial_{i}"),
            6.0,
            4.0,
            24,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 18.0);
        let center = centered_cylinder(
            format!("osmotic_shock_ramp_station_evidence_fiducial_center_{i}"),
            2.0,
            6.0,
            18,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 18.0);
        fiducials = fiducials + (fid - center);
    }
    fiducials
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "osmotic_shock_ramp_station_front_robot_approach_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 82.0,
        DECK_Z / 2.0 + ROBOT_KEEPOUT_Z / 2.0,
    );
    let service_left = centered_cube(
        "osmotic_shock_ramp_station_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 78.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "osmotic_shock_ramp_station_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 - 78.0,
        0.0,
        DECK_Z / 2.0 + SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "osmotic_shock_ramp_station_top_service_clearance_gauge",
        700.0,
        430.0,
        10.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 + TOP_SERVICE_CLEARANCE_Z);

    robot_front + service_left + service_right + top_clearance
}

fn station_assembly() -> Part {
    containment_deck()
        + cassette_surrogate_dock().translate(
            CASSETTE_DOCK_POS.0,
            CASSETTE_DOCK_POS.1,
            top_z(CASSETTE_DOCK_Z),
        )
        + high_low_osmolality_media_bag_nests().translate(
            BAG_BANK_POS.0,
            BAG_BANK_POS.1,
            top_z(BAG_BANK_Z),
        )
        + staged_ramp_manifold().translate(MANIFOLD_POS.0, MANIFOLD_POS.1, top_z(MANIFOLD_Z))
        + conductivity_osmolality_sample_wells().translate(
            SAMPLE_BANK_POS.0,
            SAMPLE_BANK_POS.1,
            top_z(SAMPLE_BANK_Z),
        )
        + timed_exchange_token_rail().translate(
            TOKEN_RAIL_POS.0,
            TOKEN_RAIL_POS.1,
            top_z(TOKEN_RAIL_Z),
        )
        + pressure_flow_shear_witness_taps().translate(
            TAP_BANK_POS.0,
            TAP_BANK_POS.1,
            top_z(TAP_BANK_Z),
        )
        + bubble_dead_volume_windows().translate(
            WINDOW_BANK_POS.0,
            WINDOW_BANK_POS.1,
            top_z(WINDOW_BANK_Z),
        )
        + waste_retain_split().translate(SPLIT_TRAY_POS.0, SPLIT_TRAY_POS.1, top_z(SPLIT_TRAY_Z))
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(
            GATE_BANK_POS.0,
            GATE_BANK_POS.1,
            top_z(GATE_BANK_Z),
        )
        + camera_evidence_bridge().translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, DECK_Z / 2.0)
        + robot_service_keepouts()
}

fn bag_nest_center(index: usize) -> (f64, f64) {
    let lane = index / BAG_NESTS_PER_OSMOLALITY;
    let slot = index % BAG_NESTS_PER_OSMOLALITY;
    (
        port_x(slot, BAG_NESTS_PER_OSMOLALITY, BAG_PITCH_X),
        if lane == 0 {
            -BAG_LANE_PITCH_Y / 2.0
        } else {
            BAG_LANE_PITCH_Y / 2.0
        },
    )
}

fn ramp_stage_x(index: usize) -> f64 {
    port_x(index, RAMP_STAGES, STAGE_PITCH_X)
}

fn valve_stage_x(index: usize) -> f64 {
    port_x(index, MANIFOLD_VALVE_BOSSES, STAGE_PITCH_X)
}

fn sample_well_center(index: usize, count: usize, y: f64) -> (f64, f64) {
    (port_x(index, count, SAMPLE_WELL_PITCH_X), y)
}

fn token_x(index: usize) -> f64 {
    port_x(index, TIMED_EXCHANGE_TOKENS, TOKEN_PITCH_X)
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

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn inside_deck(pos: (f64, f64), x: f64, y: f64) -> bool {
    pos.0 - x / 2.0 > -DECK_X / 2.0 + RIM_W
        && pos.0 + x / 2.0 < DECK_X / 2.0 - RIM_W
        && pos.1 - y / 2.0 > -DECK_Y / 2.0 + RIM_W
        && pos.1 + y / 2.0 < DECK_Y / 2.0 - RIM_W
}

fn token_span() -> f64 {
    (TIMED_EXCHANGE_TOKENS as f64 - 1.0) * TOKEN_PITCH_X + TOKEN_SLOT_X
}

fn bag_nest_span_x() -> f64 {
    (BAG_NESTS_PER_OSMOLALITY as f64 - 1.0) * BAG_PITCH_X + BAG_NEST_X
}

fn ramp_span_x() -> f64 {
    (MANIFOLD_VALVE_BOSSES as f64 - 1.0) * STAGE_PITCH_X + RAMP_PORT_D
}

fn sample_well_span(count: usize) -> f64 {
    (count as f64 - 1.0) * SAMPLE_WELL_PITCH_X + SAMPLE_WELL_D
}

fn window_span(count: usize) -> f64 {
    (count as f64 - 1.0) * WINDOW_PITCH_X + WINDOW_D
}

fn tap_span(count: usize) -> f64 {
    (count as f64 - 1.0) * TAP_PITCH_X + TAP_BOSS_D
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(TOTAL_BAG_NESTS, BAG_NESTS_PER_OSMOLALITY * 2);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX, 0);
    assert_eq!(HOLD_GATE_INDEX, 1);
    assert_eq!(REJECT_GATE_INDEX, 2);
    assert!(inside_deck(
        CASSETTE_DOCK_POS,
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y
    ));
    assert!(inside_deck(BAG_BANK_POS, BAG_BANK_X, BAG_BANK_Y));
    assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
    assert!(inside_deck(SAMPLE_BANK_POS, SAMPLE_BANK_X, SAMPLE_BANK_Y));
    assert!(inside_deck(TOKEN_RAIL_POS, TOKEN_RAIL_X, TOKEN_RAIL_Y));
    assert!(inside_deck(TAP_BANK_POS, TAP_BANK_X, TAP_BANK_Y));
    assert!(inside_deck(WINDOW_BANK_POS, WINDOW_BANK_X, WINDOW_BANK_Y));
    assert!(inside_deck(SPLIT_TRAY_POS, SPLIT_TRAY_X, SPLIT_TRAY_Y));
    assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
    assert!(inside_deck(GATE_BANK_POS, GATE_BANK_X, GATE_BANK_Y));
    assert!(token_span() < TOKEN_RAIL_X - 34.0);
    assert!(bag_nest_span_x() < BAG_BANK_X / 2.0);
    assert!(ramp_span_x() < MANIFOLD_X - 76.0);
    assert!(sample_well_span(CONDUCTIVITY_WELLS) < SAMPLE_BANK_X - 112.0);
    assert!(sample_well_span(OSMOLALITY_WELLS) < SAMPLE_BANK_X - 112.0);
    assert!(window_span(BUBBLE_WINDOWS) < WINDOW_BANK_X - 70.0);
    assert!(window_span(DEAD_VOLUME_WINDOWS) < WINDOW_BANK_X - 100.0);
    assert!(tap_span(PRESSURE_TAPS) < TAP_BANK_X - 170.0);
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
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
        assert!(OUTPUTS
            .iter()
            .any(|path| path.contains("staged_ramp_manifold")));
        assert!(OUTPUTS
            .iter()
            .any(|path| path.contains("waste_retain_split")));
    }

    #[test]
    fn requested_feature_metadata_is_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 25);
        assert!(REQUIRED_FEATURES.contains(&"mechanical_validation_packaging_only"));
        assert!(REQUIRED_FEATURES.contains(&"cassette_surrogate_dock"));
        assert!(REQUIRED_FEATURES.contains(&"high_osmolality_media_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"low_osmolality_media_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"staged_ramp_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"conductivity_sample_wells"));
        assert!(REQUIRED_FEATURES.contains(&"osmolality_sample_wells"));
        assert!(REQUIRED_FEATURES.contains(&"timed_exchange_token_rail"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_witness_taps"));
        assert!(REQUIRED_FEATURES.contains(&"flow_witness_taps"));
        assert!(REQUIRED_FEATURES.contains(&"shear_witness_coupons"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"dead_volume_windows"));
        assert!(REQUIRED_FEATURES.contains(&"waste_retain_split"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"coa_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge_robot_service_keepouts"));
    }

    #[test]
    fn scope_excludes_biological_process_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 5);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_sop"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"assay_method"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"pressure_rated_fluid_path"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"acceptance_criterion"));
    }

    #[test]
    fn repeated_features_match_validation_packaging_intent() {
        assert_eq!(CASSETTE_DATUM_PINS, 6);
        assert_eq!(TOTAL_BAG_NESTS, 6);
        assert_eq!(RAMP_STAGES, 6);
        assert_eq!(MANIFOLD_VALVE_BOSSES, RAMP_STAGES + 1);
        assert_eq!(CONDUCTIVITY_WELLS, OSMOLALITY_WELLS);
        assert_eq!(REFERENCE_WELLS, 4);
        assert!(TIMED_EXCHANGE_TOKENS >= RAMP_STAGES);
        assert_eq!(PRESSURE_TAPS, FLOW_TAPS);
        assert!(SHEAR_COUPONS >= RAMP_STAGES);
        assert!(BUBBLE_WINDOWS > DEAD_VOLUME_WINDOWS);
        assert_eq!(WASTE_CELLS, RETAIN_CELLS);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_MOUNTS, 5);
    }

    #[test]
    fn major_modules_fit_inside_contained_deck() {
        assert_layout();
        assert!(inside_deck(
            CASSETTE_DOCK_POS,
            CASSETTE_DOCK_X,
            CASSETTE_DOCK_Y
        ));
        assert!(inside_deck(BAG_BANK_POS, BAG_BANK_X, BAG_BANK_Y));
        assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
        assert!(inside_deck(SAMPLE_BANK_POS, SAMPLE_BANK_X, SAMPLE_BANK_Y));
        assert!(inside_deck(TOKEN_RAIL_POS, TOKEN_RAIL_X, TOKEN_RAIL_Y));
        assert!(inside_deck(TAP_BANK_POS, TAP_BANK_X, TAP_BANK_Y));
        assert!(inside_deck(WINDOW_BANK_POS, WINDOW_BANK_X, WINDOW_BANK_Y));
        assert!(inside_deck(SPLIT_TRAY_POS, SPLIT_TRAY_X, SPLIT_TRAY_Y));
        assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
        assert!(inside_deck(GATE_BANK_POS, GATE_BANK_X, GATE_BANK_Y));
    }

    #[test]
    fn arrays_and_spans_stay_within_printable_banks() {
        assert!(token_span() < TOKEN_RAIL_X - 34.0);
        assert!(bag_nest_span_x() < BAG_BANK_X / 2.0);
        assert!(ramp_span_x() < MANIFOLD_X - 76.0);
        assert!(sample_well_span(CONDUCTIVITY_WELLS) < SAMPLE_BANK_X - 112.0);
        assert!(sample_well_span(OSMOLALITY_WELLS) < SAMPLE_BANK_X - 112.0);
        assert!(window_span(BUBBLE_WINDOWS) < WINDOW_BANK_X - 70.0);
        assert!(window_span(DEAD_VOLUME_WINDOWS) < WINDOW_BANK_X - 100.0);
        assert!(tap_span(PRESSURE_TAPS) < TAP_BANK_X - 170.0);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
    }

    #[test]
    fn high_low_lanes_and_disposition_indices_are_stable() {
        let low_bag = bag_nest_center(0);
        let high_bag = bag_nest_center(BAG_NESTS_PER_OSMOLALITY);
        assert!(low_bag.1 < 0.0);
        assert!(high_bag.1 > 0.0);
        assert!((high_bag.1 - low_bag.1).abs() >= BAG_LANE_PITCH_Y);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
        assert_eq!(GATE_TOKEN_SLOTS % DISPOSITION_GATES, 0);
    }
}
