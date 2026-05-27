use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed ECM lot viscosity, temperature, and dispense-window validation station.
//
// Intent:
// - Validate ECM/coating reagent lot handling before automated coating without
//   opening the fluid path or encoding protocol acceptance thresholds.
// - Keep lot vial/bag custody, temperature equilibration, low-shear agitation,
//   viscosity/reference-flow coupons, timed dispense-window tokens, light
//   protection, bubble/wetness witnesses, traceability surfaces, closed
//   connector handoff, and robot/service keepouts mechanically explicit.
// - This is validation fixture CAD only. It does not define clinical
//   acceptance criteria, ECM release limits, sterility claims, or cell outcomes.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_ecm_lot_viscosity_temperature_dispense_window_station";
const OUTPUTS: [&str; 13] = [
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_base_leak_tray_deck.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_ecm_lot_vial_bag_nests.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_temperature_equilibration_blocks.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_low_shear_agitation_witness.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_viscosity_reference_flow_coupon.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_dispense_window_token_lanes.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_light_protection_cover.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_bubble_wetness_witness_pockets.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_barcode_coa_status_surfaces.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_closed_connector_handoff_bulkhead.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_coating_station_handoff_bridge.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_robot_service_keepout_gauges.stl",
    "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "ecm_lot_vial_bag_nests",
    "temperature_equilibration_blocks",
    "low_shear_agitation_witness",
    "viscosity_reference_flow_coupon",
    "dispense_window_token_lanes",
    "light_protection_cover",
    "bubble_wetness_witness_pockets",
    "barcode_coa_status_surfaces",
    "closed_connector_handoff_bulkhead",
    "coating_station_handoff_bridge",
    "robot_service_keepout_gauges",
    "validation_fixture_intent_only",
];

#[cfg(test)]
const LIMITATIONS: [&str; 5] = [
    "no_clinical_acceptance_thresholds",
    "no_ecm_release_limits",
    "no_sterility_validation_claim",
    "no_cell_outcome_acceptance_claim",
    "no_coating_protocol_thresholds",
];

const PARAMETER_SET_REV: &str =
    "closed-ecm-lot-viscosity-temperature-dispense-window-parametric-rev-a";
const OUTPUT_MANIFEST_REV: &str = "stl-manifest-rev-a";
const UNITS: &str = "millimeters";
const USES_RANDOMNESS: bool = false;
const GEOMETRY_SEED: u64 = 0;
const CYLINDER_SEGMENTS: u32 = 32;
const FIDUCIAL_SEGMENTS: u32 = 36;
const FACET_TOLERANCE_MM: f64 = 0.25;
const CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED: bool = false;

const STATION_X: f64 = 1380.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MODULE_MARGIN_MM: f64 = 12.0;
const MAJOR_MODULE_GAP_MM: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_D: f64 = 16.0;

const NEST_POS: (f64, f64) = (-470.0, 275.0);
const NEST_X: f64 = 330.0;
const NEST_Y: f64 = 220.0;
const NEST_Z: f64 = 48.0;
const ECM_VIAL_NESTS: usize = 12;
const VIAL_COLS: usize = 4;
const VIAL_ROWS: usize = 3;
const VIAL_WELL_D: f64 = 22.0;
const VIAL_PITCH_X: f64 = 43.0;
const VIAL_PITCH_Y: f64 = 48.0;
const ECM_BAG_NESTS: usize = 3;
const BAG_NEST_X: f64 = 82.0;
const BAG_NEST_Y: f64 = 50.0;
const BAG_RECESS_Z: f64 = 13.0;
const BAG_PITCH_Y: f64 = 58.0;

const TEMP_POS: (f64, f64) = (-105.0, 275.0);
const TEMP_X: f64 = 320.0;
const TEMP_Y: f64 = 220.0;
const TEMP_BASE_Z: f64 = 18.0;
const TEMP_BLOCK_Z: f64 = 58.0;
const TEMP_BLOCKS: usize = 2;
const TEMP_POCKETS_PER_BLOCK: usize = 6;
const TEMP_POCKETS: usize = TEMP_BLOCKS * TEMP_POCKETS_PER_BLOCK;
const TEMP_BLOCK_X: f64 = 128.0;
const TEMP_BLOCK_Y: f64 = 172.0;
const TEMP_POCKET_D: f64 = 19.0;
const TEMP_POCKET_PITCH_X: f64 = 39.0;
const TEMP_POCKET_PITCH_Y: f64 = 50.0;
const TEMP_PROBE_LANDS: usize = 4;

const AGITATION_POS: (f64, f64) = (335.0, 275.0);
const AGITATION_X: f64 = 420.0;
const AGITATION_Y: f64 = 220.0;
const AGITATION_Z: f64 = 52.0;
const AGITATION_WITNESS_ROLLERS: usize = 5;
const AGITATION_INDEX_TICKS: usize = 9;
const MIXING_WITNESS_WINDOWS: usize = 6;
const LOW_SHEAR_BAFFLES: usize = 7;

const VISCOSITY_POS: (f64, f64) = (-430.0, 25.0);
const VISCOSITY_X: f64 = 400.0;
const VISCOSITY_Y: f64 = 220.0;
const VISCOSITY_Z: f64 = 54.0;
const VISCOSITY_COUPON_LANES: usize = 6;
const REFERENCE_FLOW_CHANNELS: usize = VISCOSITY_COUPON_LANES;
const FLOW_COUPON_SLOT_X: f64 = 40.0;
const FLOW_COUPON_SLOT_Y: f64 = 128.0;
const FLOW_COUPON_PITCH_X: f64 = 54.0;
const PRESSURE_TAP_PORTS: usize = 12;
const REFERENCE_FLOW_PORT_D: f64 = 6.0;

const WINDOW_POS: (f64, f64) = (-430.0, -275.0);
const WINDOW_X: f64 = 420.0;
const WINDOW_Y: f64 = 200.0;
const WINDOW_Z: f64 = 38.0;
const DISPENSE_WINDOW_LANES: usize = 4;
const WINDOW_TOKENS_PER_LANE: usize = 5;
const DISPENSE_WINDOW_TOKENS: usize = DISPENSE_WINDOW_LANES * WINDOW_TOKENS_PER_LANE;
const WINDOW_TOKEN_SLOT_X: f64 = 34.0;
const WINDOW_TOKEN_SLOT_Y: f64 = 28.0;
const WINDOW_LANE_PITCH_Y: f64 = 42.0;
const WINDOW_TOKEN_PITCH_X: f64 = 55.0;

const COVER_POS: (f64, f64) = (470.0, -275.0);
const COVER_X: f64 = 320.0;
const COVER_Y: f64 = 200.0;
const COVER_Z: f64 = 122.0;
const COVER_WALL_W: f64 = 12.0;
const LIGHT_BAFFLES: usize = 6;
const COVER_LATCH_TABS: usize = 4;
const AMBIENT_LIGHT_WITNESS_FLAGS: usize = 4;

const BUBBLE_POS: (f64, f64) = (20.0, 25.0);
const BUBBLE_X: f64 = 390.0;
const BUBBLE_Y: f64 = 220.0;
const BUBBLE_Z: f64 = 42.0;
const BUBBLE_WITNESS_POCKETS: usize = 8;
const WETNESS_WITNESS_POCKETS: usize = 8;
const WITNESS_POCKET_D: f64 = 24.0;
const WITNESS_POCKET_PITCH_X: f64 = 42.0;
const WITNESS_ROW_Y: f64 = 44.0;
const WETNESS_PAD_X: f64 = 32.0;
const WETNESS_PAD_Y: f64 = 22.0;

const TRACE_POS: (f64, f64) = (40.0, -275.0);
const TRACE_X: f64 = 420.0;
const TRACE_Y: f64 = 200.0;
const TRACE_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 12;
const COA_CARD_SURFACES: usize = 4;
const STATUS_LANES: usize = 4;
const STATUS_TOKENS_PER_LANE: usize = 4;
const STATUS_TOKEN_SLOTS: usize = STATUS_LANES * STATUS_TOKENS_PER_LANE;

const HANDOFF_POS: (f64, f64) = (460.0, 25.0);
const HANDOFF_X: f64 = 340.0;
const HANDOFF_Y: f64 = 220.0;
const HANDOFF_Z: f64 = 118.0;
const CLOSED_CONNECTOR_PORTS: usize = 8;
const CONNECTOR_ROWS: usize = 2;
const CONNECTOR_COLS: usize = 4;
const CONNECTOR_PITCH_X: f64 = 64.0;
const CONNECTOR_PITCH_Z: f64 = 36.0;
const CONNECTOR_PORT_D: f64 = 10.0;
const CONNECTOR_COLLAR_D: f64 = 30.0;
const CAP_PARKS: usize = CLOSED_CONNECTOR_PORTS;
const STRAIN_RELIEF_CLAMPS: usize = 6;

const BRIDGE_POS: (f64, f64) = (500.0, -130.0);
const BRIDGE_X: f64 = 290.0;
const BRIDGE_Y: f64 = 60.0;
const BRIDGE_Z: f64 = 72.0;
const COATING_HANDOFF_DOCKS: usize = 2;
const GUIDE_TONGUES: usize = 3;
const ROUTE_SEGMENTS: usize = 8;

const ROBOT_SERVICE_KEEPOUT_ZONES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 380.0;
const REAR_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_ECM_LOT_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_CONNECTOR_SERVICE_CLEARANCE: f64 = 220.0;
const TOP_COVER_LIFT_CLEARANCE: f64 = 275.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - MODULE_MARGIN_MM
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - MODULE_MARGIN_MM
    }

    fn overlaps(self, other: Footprint, margin: f64) -> bool {
        let a = rect(self.center, self.x, self.y);
        let b = rect(other.center, other.x, other.y);
        a.0 < b.1 + margin && a.1 + margin > b.0 && a.2 < b.3 + margin && a.3 + margin > b.2
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let nests = ecm_lot_vial_bag_nests();
    export(OUTPUTS[1], &nests);

    let temp = temperature_equilibration_blocks();
    export(OUTPUTS[2], &temp);

    let agitation = low_shear_agitation_witness();
    export(OUTPUTS[3], &agitation);

    let viscosity = viscosity_reference_flow_coupon();
    export(OUTPUTS[4], &viscosity);

    let windows = dispense_window_token_lanes();
    export(OUTPUTS[5], &windows);

    let cover = light_protection_cover();
    export(OUTPUTS[6], &cover);

    let bubble = bubble_wetness_witness_pockets();
    export(OUTPUTS[7], &bubble);

    let trace = barcode_coa_status_surfaces();
    export(OUTPUTS[8], &trace);

    let handoff = closed_connector_handoff_bulkhead();
    export(OUTPUTS[9], &handoff);

    let bridge = coating_station_handoff_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + nests
        + temp
        + agitation
        + viscosity
        + windows
        + cover
        + bubble
        + trace
        + handoff
        + bridge
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed ECM lot viscosity/temperature/dispense-window validation station:");
    println!(
        "  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck, units={UNITS}"
    );
    println!(
        "  Lot custody:                {ECM_VIAL_NESTS} vial nests, {ECM_BAG_NESTS} bag nests, {BARCODE_LANDS} barcode lands, {COA_CARD_SURFACES} COA surfaces"
    );
    println!(
        "  Equilibration/agitation:    {TEMP_BLOCKS} temperature blocks, {TEMP_POCKETS} pockets, {AGITATION_WITNESS_ROLLERS} low-shear rollers, {MIXING_WITNESS_WINDOWS} witness windows"
    );
    println!(
        "  Flow/dispense witness:      {VISCOSITY_COUPON_LANES} viscosity coupons, {REFERENCE_FLOW_CHANNELS} reference-flow channels, {DISPENSE_WINDOW_TOKENS} timed dispense-window tokens"
    );
    println!(
        "  Closed handoff/keepouts:    {CLOSED_CONNECTOR_PORTS} connector ports, {COATING_HANDOFF_DOCKS} coating-station docks, {ROBOT_SERVICE_KEEPOUT_ZONES} keepout gauges, thresholds_encoded={CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED}"
    );
    println!(
        "  Reproducibility controls:   {PARAMETER_SET_REV}, {OUTPUT_MANIFEST_REV}, randomness={USES_RANDOMNESS}, seed={GEOMETRY_SEED}, cylinder_segments={CYLINDER_SEGMENTS}, facet_tolerance={FACET_TOLERANCE_MM:.2}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_ecm_lot_window_station_base_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "closed_ecm_lot_window_station_recessed_spill_sump",
        STATION_X - 132.0,
        STATION_Y - 118.0,
        SOCKET_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SOCKET_DEPTH / 2.0);
    let drain = centered_cylinder(
        "closed_ecm_lot_window_station_sump_drain_bore",
        DRAIN_D / 2.0,
        RIM_W + 32.0,
        CYLINDER_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 76.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 7.0);

    deck - sump - drain - module_socket_recesses() - deck_mount_holes()
        + perimeter_rims()
        + row_dividers()
        + leak_witness_ribs()
        + robot_datum_targets()
}

fn module_socket_recesses() -> Part {
    let mut recesses = Part::empty("closed_ecm_lot_window_station_module_socket_recesses");
    for module in module_footprints() {
        recesses = recesses
            + centered_cube(
                format!(
                    "closed_ecm_lot_window_station_{}_socket_recess",
                    module.name
                ),
                module.x + 16.0,
                module.y + 16.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0,
            );
    }
    recesses
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_ecm_lot_window_station_deck_mount_holes");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_m6_mount_bore_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("closed_ecm_lot_window_station_m6_mount_slot_{index}"),
                24.0,
                7.0,
                BASE_Z + 6.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_ecm_lot_window_station_front_low_leak_lip",
        STATION_X - 180.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z + 10.0);
    let rear = centered_cube(
        "closed_ecm_lot_window_station_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_ecm_lot_window_station_left_containment_rim",
        RIM_W,
        STATION_Y - 72.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_ecm_lot_window_station_right_containment_rim",
        RIM_W,
        STATION_Y - 72.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn row_dividers() -> Part {
    let top_mid = centered_cube(
        "closed_ecm_lot_window_station_top_middle_row_divider",
        STATION_X - 158.0,
        8.0,
        22.0,
    )
    .translate(0.0, 150.0, BASE_Z + 11.0);
    let mid_bottom = centered_cube(
        "closed_ecm_lot_window_station_middle_bottom_row_divider",
        STATION_X - 180.0,
        8.0,
        22.0,
    )
    .translate(0.0, -150.0, BASE_Z + 11.0);
    let left_mid_split = centered_cube(
        "closed_ecm_lot_window_station_viscosity_witness_split",
        8.0,
        214.0,
        20.0,
    )
    .translate(-205.0, 25.0, BASE_Z + 10.0);
    let right_mid_split = centered_cube(
        "closed_ecm_lot_window_station_witness_handoff_split",
        8.0,
        214.0,
        20.0,
    )
    .translate(235.0, 25.0, BASE_Z + 10.0);

    top_mid + mid_bottom + left_mid_split + right_mid_split
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_ecm_lot_window_station_leak_witness_ribs");
    for (index, x) in [-560.0, -380.0, -190.0, 0.0, 190.0, 380.0, 560.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_ecm_lot_window_station_leak_witness_rib_{index}"),
                5.0,
                STATION_Y - 166.0,
                5.0,
            )
            .translate(x, -6.0, BASE_Z + 2.5);
    }
    ribs
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_ecm_lot_window_station_robot_datum_targets");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 84.0, STATION_Y / 2.0 - 84.0),
        (STATION_X / 2.0 - 84.0, STATION_Y / 2.0 - 84.0),
        (-STATION_X / 2.0 + 84.0, -STATION_Y / 2.0 + 84.0),
        (STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 + 84.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_ecm_lot_window_station_robot_datum_{index}"
            ))
            .translate(x, y, BASE_Z + 2.0);
    }
    targets
}

fn ecm_lot_vial_bag_nests() -> Part {
    let body = centered_cube(
        "closed_ecm_lot_window_station_ecm_lot_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let spill_basin = centered_cube(
        "closed_ecm_lot_window_station_ecm_lot_nest_spill_basin",
        NEST_X - 36.0,
        NEST_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, NEST_Z - 3.5);

    let part = body - spill_basin - ecm_vial_well_cuts() - ecm_bag_recess_cuts()
        + ecm_vial_well_rims()
        + ecm_bag_saddle_edges()
        + custody_latch_tabs("ecm_lot_nest", NEST_X, NEST_Y, NEST_Z);
    place_on_deck(part, NEST_POS)
}

fn ecm_vial_well_cuts() -> Part {
    let mut wells = Part::empty("closed_ecm_lot_window_station_vial_well_cuts");
    for index in 0..ECM_VIAL_NESTS {
        let (x, y) = vial_position(index);
        wells = wells
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_ecm_vial_well_cut_{index}"),
                VIAL_WELL_D / 2.0,
                NEST_Z + 8.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, NEST_Z / 2.0);
    }
    wells
}

fn ecm_vial_well_rims() -> Part {
    let mut rims = Part::empty("closed_ecm_lot_window_station_vial_well_rims");
    for index in 0..ECM_VIAL_NESTS {
        let (x, y) = vial_position(index);
        let outer = centered_cylinder(
            format!("closed_ecm_lot_window_station_ecm_vial_retainer_rim_{index}"),
            VIAL_WELL_D / 2.0 + 3.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, NEST_Z + 2.5);
        let inner = centered_cylinder(
            format!("closed_ecm_lot_window_station_ecm_vial_retainer_open_{index}"),
            VIAL_WELL_D / 2.0 + 0.7,
            5.6,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, NEST_Z + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn ecm_bag_recess_cuts() -> Part {
    let mut cuts = Part::empty("closed_ecm_lot_window_station_ecm_bag_recess_cuts");
    for index in 0..ECM_BAG_NESTS {
        let y = centered_index(index, ECM_BAG_NESTS, BAG_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("closed_ecm_lot_window_station_ecm_bag_shadow_recess_{index}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_RECESS_Z + 0.4,
            )
            .translate(86.0, y, NEST_Z - BAG_RECESS_Z / 2.0);
    }
    cuts
}

fn ecm_bag_saddle_edges() -> Part {
    let mut saddles = Part::empty("closed_ecm_lot_window_station_ecm_bag_saddle_edges");
    for index in 0..ECM_BAG_NESTS {
        let y = centered_index(index, ECM_BAG_NESTS, BAG_PITCH_Y);
        let left = centered_cube(
            format!("closed_ecm_lot_window_station_ecm_bag_left_saddle_{index}"),
            7.0,
            BAG_NEST_Y + 12.0,
            12.0,
        )
        .translate(86.0 - BAG_NEST_X / 2.0 - 7.0, y, NEST_Z + 6.0);
        let right = centered_cube(
            format!("closed_ecm_lot_window_station_ecm_bag_right_saddle_{index}"),
            7.0,
            BAG_NEST_Y + 12.0,
            12.0,
        )
        .translate(86.0 + BAG_NEST_X / 2.0 + 7.0, y, NEST_Z + 6.0);
        let tail_clip = centered_cube(
            format!("closed_ecm_lot_window_station_ecm_bag_tail_clip_{index}"),
            BAG_NEST_X - 12.0,
            7.0,
            12.0,
        )
        .translate(86.0, y - BAG_NEST_Y / 2.0 - 7.0, NEST_Z + 6.0);
        saddles = saddles + left + right + tail_clip;
    }
    saddles
}

fn temperature_equilibration_blocks() -> Part {
    let base = centered_cube(
        "closed_ecm_lot_window_station_temperature_equilibration_base",
        TEMP_X,
        TEMP_Y,
        TEMP_BASE_Z,
    )
    .translate(0.0, 0.0, TEMP_BASE_Z / 2.0);
    let coolant_channel = centered_cube(
        "closed_ecm_lot_window_station_temperature_coolant_route_cut",
        TEMP_X - 48.0,
        12.0,
        16.0,
    )
    .translate(0.0, TEMP_Y / 2.0 - 24.0, TEMP_BASE_Z + 11.0);

    let part = base - coolant_channel
        + temperature_block_bodies()
        + temperature_probe_lands()
        + thermal_contact_fins()
        + custody_latch_tabs("temperature_equilibration", TEMP_X, TEMP_Y, TEMP_BLOCK_Z);
    place_on_deck(part, TEMP_POS)
}

fn temperature_block_bodies() -> Part {
    let mut blocks = Part::empty("closed_ecm_lot_window_station_temperature_block_bodies");
    for block in 0..TEMP_BLOCKS {
        let x = centered_index(block, TEMP_BLOCKS, 150.0);
        let body = centered_cube(
            format!("closed_ecm_lot_window_station_temperature_block_{block}"),
            TEMP_BLOCK_X,
            TEMP_BLOCK_Y,
            TEMP_BLOCK_Z,
        )
        .translate(x, 0.0, TEMP_BLOCK_Z / 2.0);
        blocks = blocks + (body - temperature_pocket_cuts_for_block(block, x));
    }
    blocks
}

fn temperature_pocket_cuts_for_block(block: usize, block_x: f64) -> Part {
    let mut cuts = Part::empty(format!(
        "closed_ecm_lot_window_station_temperature_pockets_block_{block}"
    ));
    for index in 0..TEMP_POCKETS_PER_BLOCK {
        let col = index % 3;
        let row = index / 3;
        let x = block_x + centered_index(col, 3, TEMP_POCKET_PITCH_X);
        let y = centered_index(row, 2, TEMP_POCKET_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_temperature_pocket_{block}_{index}"),
                TEMP_POCKET_D / 2.0,
                TEMP_BLOCK_Z + 8.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, TEMP_BLOCK_Z / 2.0)
            + centered_cube(
                format!("closed_ecm_lot_window_station_temperature_square_cup_{block}_{index}"),
                26.0,
                24.0,
                18.0,
            )
            .translate(x, y + 18.0, TEMP_BLOCK_Z - 9.0);
    }
    cuts
}

fn temperature_probe_lands() -> Part {
    let mut probes = Part::empty("closed_ecm_lot_window_station_temperature_probe_lands");
    for index in 0..TEMP_PROBE_LANDS {
        let x = centered_index(index, TEMP_PROBE_LANDS, 58.0);
        probes = probes
            + centered_cube(
                format!("closed_ecm_lot_window_station_temperature_probe_clip_{index}"),
                34.0,
                12.0,
                9.0,
            )
            .translate(x, -TEMP_Y / 2.0 + 22.0, TEMP_BLOCK_Z + 4.5)
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_temperature_probe_bore_{index}"),
                3.0 / 2.0,
                36.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TEMP_Y / 2.0 + 22.0, TEMP_BLOCK_Z + 4.5);
    }
    probes
}

fn thermal_contact_fins() -> Part {
    let mut fins = Part::empty("closed_ecm_lot_window_station_thermal_contact_fins");
    for index in 0..7 {
        fins = fins
            + centered_cube(
                format!("closed_ecm_lot_window_station_thermal_contact_fin_{index}"),
                5.0,
                TEMP_Y - 66.0,
                5.0,
            )
            .translate(centered_index(index, 7, 42.0), 0.0, TEMP_BLOCK_Z + 2.5);
    }
    fins
}

fn low_shear_agitation_witness() -> Part {
    let body = centered_cube(
        "closed_ecm_lot_window_station_low_shear_agitation_body",
        AGITATION_X,
        AGITATION_Y,
        AGITATION_Z,
    )
    .translate(0.0, 0.0, AGITATION_Z / 2.0);
    let bag_shadow = centered_cube(
        "closed_ecm_lot_window_station_low_shear_bag_motion_shadow",
        AGITATION_X - 72.0,
        82.0,
        12.0,
    )
    .translate(0.0, 24.0, AGITATION_Z - 6.0);

    let part = body - bag_shadow
        + agitation_rollers()
        + agitation_index_ticks()
        + mixing_witness_windows()
        + low_shear_baffle_ribs()
        + custody_latch_tabs("low_shear_agitation", AGITATION_X, AGITATION_Y, AGITATION_Z);
    place_on_deck(part, AGITATION_POS)
}

fn agitation_rollers() -> Part {
    let mut rollers = Part::empty("closed_ecm_lot_window_station_low_shear_rollers");
    for index in 0..AGITATION_WITNESS_ROLLERS {
        let x = centered_index(index, AGITATION_WITNESS_ROLLERS, 66.0);
        rollers = rollers
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_low_shear_roller_{index}"),
                9.0,
                142.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 28.0, AGITATION_Z + 9.0)
            + centered_cube(
                format!("closed_ecm_lot_window_station_low_shear_roller_end_stop_{index}"),
                18.0,
                8.0,
                20.0,
            )
            .translate(x, -54.0, AGITATION_Z + 10.0);
    }
    rollers
}

fn agitation_index_ticks() -> Part {
    let mut ticks = Part::empty("closed_ecm_lot_window_station_agitation_index_ticks");
    for index in 0..AGITATION_INDEX_TICKS {
        let x = centered_index(index, AGITATION_INDEX_TICKS, 36.0);
        ticks = ticks
            + centered_cube(
                format!("closed_ecm_lot_window_station_agitation_motion_tick_{index}"),
                6.0,
                22.0,
                7.0,
            )
            .translate(x, AGITATION_Y / 2.0 - 28.0, AGITATION_Z + 3.5);
    }
    ticks
}

fn mixing_witness_windows() -> Part {
    let mut windows = Part::empty("closed_ecm_lot_window_station_mixing_witness_windows");
    for index in 0..MIXING_WITNESS_WINDOWS {
        let x = centered_index(index, MIXING_WITNESS_WINDOWS, 54.0);
        windows = windows
            + centered_cube(
                format!("closed_ecm_lot_window_station_mixing_witness_window_frame_{index}"),
                36.0,
                26.0,
                8.0,
            )
            .translate(x, -AGITATION_Y / 2.0 + 44.0, AGITATION_Z + 4.0)
            - centered_cube(
                format!("closed_ecm_lot_window_station_mixing_witness_window_clear_{index}"),
                26.0,
                16.0,
                9.0,
            )
            .translate(x, -AGITATION_Y / 2.0 + 44.0, AGITATION_Z + 4.0);
    }
    windows
}

fn low_shear_baffle_ribs() -> Part {
    let mut ribs = Part::empty("closed_ecm_lot_window_station_low_shear_baffle_ribs");
    for index in 0..LOW_SHEAR_BAFFLES {
        let x = centered_index(index, LOW_SHEAR_BAFFLES, 42.0);
        ribs = ribs
            + centered_cube(
                format!("closed_ecm_lot_window_station_low_shear_baffle_rib_{index}"),
                5.0,
                92.0,
                10.0,
            )
            .translate(x, 24.0, AGITATION_Z + 5.0);
    }
    ribs
}

fn viscosity_reference_flow_coupon() -> Part {
    let body = centered_cube(
        "closed_ecm_lot_window_station_viscosity_reference_flow_body",
        VISCOSITY_X,
        VISCOSITY_Y,
        VISCOSITY_Z,
    )
    .translate(0.0, 0.0, VISCOSITY_Z / 2.0);
    let drain_basin = centered_cube(
        "closed_ecm_lot_window_station_viscosity_coupon_spill_basin",
        VISCOSITY_X - 38.0,
        VISCOSITY_Y - 42.0,
        9.0,
    )
    .translate(0.0, 0.0, VISCOSITY_Z - 4.0);

    let part = body - drain_basin - viscosity_coupon_slots()
        + viscosity_coupon_clamps()
        + reference_flow_channels()
        + pressure_tap_ports()
        + custody_latch_tabs(
            "viscosity_reference_flow",
            VISCOSITY_X,
            VISCOSITY_Y,
            VISCOSITY_Z,
        );
    place_on_deck(part, VISCOSITY_POS)
}

fn viscosity_coupon_slots() -> Part {
    let mut slots = Part::empty("closed_ecm_lot_window_station_viscosity_coupon_slots");
    for lane in 0..VISCOSITY_COUPON_LANES {
        let x = centered_index(lane, VISCOSITY_COUPON_LANES, FLOW_COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_ecm_lot_window_station_viscosity_coupon_slot_{lane}"),
                FLOW_COUPON_SLOT_X,
                FLOW_COUPON_SLOT_Y,
                VISCOSITY_Z + 6.0,
            )
            .translate(x, 10.0, VISCOSITY_Z / 2.0);
    }
    slots
}

fn viscosity_coupon_clamps() -> Part {
    let mut clamps = Part::empty("closed_ecm_lot_window_station_viscosity_coupon_clamps");
    for lane in 0..VISCOSITY_COUPON_LANES {
        let x = centered_index(lane, VISCOSITY_COUPON_LANES, FLOW_COUPON_PITCH_X);
        clamps = clamps
            + centered_cube(
                format!("closed_ecm_lot_window_station_viscosity_coupon_front_clamp_{lane}"),
                FLOW_COUPON_SLOT_X + 14.0,
                8.0,
                12.0,
            )
            .translate(x, -58.0, VISCOSITY_Z + 6.0)
            + centered_cube(
                format!("closed_ecm_lot_window_station_viscosity_coupon_rear_clamp_{lane}"),
                FLOW_COUPON_SLOT_X + 14.0,
                8.0,
                12.0,
            )
            .translate(x, 78.0, VISCOSITY_Z + 6.0);
    }
    clamps
}

fn reference_flow_channels() -> Part {
    let mut channels = Part::empty("closed_ecm_lot_window_station_reference_flow_channels");
    for lane in 0..REFERENCE_FLOW_CHANNELS {
        let x = centered_index(lane, REFERENCE_FLOW_CHANNELS, FLOW_COUPON_PITCH_X);
        channels = channels
            + centered_cube(
                format!("closed_ecm_lot_window_station_reference_flow_lane_{lane}"),
                8.0,
                VISCOSITY_Y - 64.0,
                8.0,
            )
            .translate(x, 0.0, VISCOSITY_Z + 4.0);
    }
    channels
}

fn pressure_tap_ports() -> Part {
    let mut ports = Part::empty("closed_ecm_lot_window_station_pressure_tap_ports");
    for index in 0..PRESSURE_TAP_PORTS {
        let lane = index / 2;
        let side = index % 2;
        let x = centered_index(lane, VISCOSITY_COUPON_LANES, FLOW_COUPON_PITCH_X);
        let y = if side == 0 { -82.0 } else { 98.0 };
        ports = ports
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_reference_flow_tap_boss_{index}"),
                10.0,
                7.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, VISCOSITY_Z + 3.5)
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_reference_flow_tap_bore_{index}"),
                REFERENCE_FLOW_PORT_D / 2.0,
                8.0,
                20,
            )
            .translate(x, y, VISCOSITY_Z + 3.5);
    }
    ports
}

fn dispense_window_token_lanes() -> Part {
    let body = centered_cube(
        "closed_ecm_lot_window_station_dispense_window_lane_body",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(0.0, 0.0, WINDOW_Z / 2.0);
    let part = body - dispense_window_token_slot_cuts()
        + dispense_window_lane_rails()
        + dispense_window_event_tabs()
        + custody_latch_tabs("dispense_window", WINDOW_X, WINDOW_Y, WINDOW_Z);
    place_on_deck(part, WINDOW_POS)
}

fn dispense_window_token_slot_cuts() -> Part {
    let mut slots = Part::empty("closed_ecm_lot_window_station_dispense_token_slot_cuts");
    for lane in 0..DISPENSE_WINDOW_LANES {
        for token in 0..WINDOW_TOKENS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!(
                        "closed_ecm_lot_window_station_dispense_window_lane_{lane}_token_slot_{token}"
                    ),
                    WINDOW_TOKEN_SLOT_X,
                    WINDOW_TOKEN_SLOT_Y,
                    12.0,
                )
                .translate(
                    centered_index(token, WINDOW_TOKENS_PER_LANE, WINDOW_TOKEN_PITCH_X),
                    centered_index(lane, DISPENSE_WINDOW_LANES, WINDOW_LANE_PITCH_Y),
                    WINDOW_Z - 5.5,
                );
        }
    }
    slots
}

fn dispense_window_lane_rails() -> Part {
    let mut rails = Part::empty("closed_ecm_lot_window_station_dispense_window_lane_rails");
    for lane in 0..DISPENSE_WINDOW_LANES {
        let y = centered_index(lane, DISPENSE_WINDOW_LANES, WINDOW_LANE_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("closed_ecm_lot_window_station_dispense_window_lane_rail_a_{lane}"),
                WINDOW_X - 70.0,
                4.0,
                8.0,
            )
            .translate(0.0, y - WINDOW_TOKEN_SLOT_Y / 2.0 - 6.0, WINDOW_Z + 4.0)
            + centered_cube(
                format!("closed_ecm_lot_window_station_dispense_window_lane_rail_b_{lane}"),
                WINDOW_X - 70.0,
                4.0,
                8.0,
            )
            .translate(0.0, y + WINDOW_TOKEN_SLOT_Y / 2.0 + 6.0, WINDOW_Z + 4.0);
    }
    rails
}

fn dispense_window_event_tabs() -> Part {
    let mut tabs = Part::empty("closed_ecm_lot_window_station_dispense_window_event_tabs");
    for (index, x) in [-154.0, -77.0, 0.0, 77.0, 154.0].into_iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("closed_ecm_lot_window_station_dispense_window_event_index_{index}"),
                28.0,
                12.0,
                8.0,
            )
            .translate(x, WINDOW_Y / 2.0 - 24.0, WINDOW_Z + 4.0);
    }
    tabs
}

fn light_protection_cover() -> Part {
    let floor = centered_cube(
        "closed_ecm_lot_window_station_light_cover_storage_floor",
        COVER_X,
        COVER_Y,
        10.0,
    )
    .translate(0.0, 0.0, 5.0);
    let rear_wall = centered_cube(
        "closed_ecm_lot_window_station_light_cover_rear_wall",
        COVER_X,
        COVER_WALL_W,
        COVER_Z,
    )
    .translate(0.0, COVER_Y / 2.0 - COVER_WALL_W / 2.0, COVER_Z / 2.0);
    let left_wall = centered_cube(
        "closed_ecm_lot_window_station_light_cover_left_wall",
        COVER_WALL_W,
        COVER_Y,
        COVER_Z,
    )
    .translate(-COVER_X / 2.0 + COVER_WALL_W / 2.0, 0.0, COVER_Z / 2.0);
    let right_wall = centered_cube(
        "closed_ecm_lot_window_station_light_cover_right_wall",
        COVER_WALL_W,
        COVER_Y,
        COVER_Z,
    )
    .translate(COVER_X / 2.0 - COVER_WALL_W / 2.0, 0.0, COVER_Z / 2.0);
    let roof = centered_cube(
        "closed_ecm_lot_window_station_light_cover_top_panel",
        COVER_X,
        COVER_Y,
        12.0,
    )
    .translate(0.0, 0.0, COVER_Z + 6.0);

    let part = floor
        + rear_wall
        + left_wall
        + right_wall
        + roof
        + light_baffles()
        + light_cover_latch_tabs()
        + ambient_light_witness_flags();
    place_on_deck(part, COVER_POS)
}

fn light_baffles() -> Part {
    let mut baffles = Part::empty("closed_ecm_lot_window_station_light_cover_baffles");
    for index in 0..LIGHT_BAFFLES {
        baffles = baffles
            + centered_cube(
                format!("closed_ecm_lot_window_station_light_baffle_{index}"),
                COVER_X - 52.0,
                6.0,
                42.0,
            )
            .translate(
                0.0,
                centered_index(index, LIGHT_BAFFLES, 28.0),
                COVER_Z - 22.0,
            );
    }
    baffles
}

fn light_cover_latch_tabs() -> Part {
    let mut tabs = Part::empty("closed_ecm_lot_window_station_light_cover_latch_tabs");
    for index in 0..COVER_LATCH_TABS {
        let x = if index % 2 == 0 { -1.0 } else { 1.0 } * (COVER_X / 2.0 - 38.0);
        let y = if index / 2 == 0 { -1.0 } else { 1.0 } * (COVER_Y / 2.0 - 28.0);
        tabs = tabs
            + centered_cube(
                format!("closed_ecm_lot_window_station_light_cover_latch_tab_{index}"),
                34.0,
                16.0,
                12.0,
            )
            .translate(x, y, 16.0);
    }
    tabs
}

fn ambient_light_witness_flags() -> Part {
    let mut flags = Part::empty("closed_ecm_lot_window_station_ambient_light_witness_flags");
    for index in 0..AMBIENT_LIGHT_WITNESS_FLAGS {
        flags = flags
            + centered_cube(
                format!("closed_ecm_lot_window_station_ambient_light_flag_{index}"),
                34.0,
                10.0,
                18.0,
            )
            .translate(
                centered_index(index, AMBIENT_LIGHT_WITNESS_FLAGS, 48.0),
                -72.0,
                22.0,
            );
    }
    flags
}

fn bubble_wetness_witness_pockets() -> Part {
    let body = centered_cube(
        "closed_ecm_lot_window_station_bubble_wetness_witness_body",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);
    let part = body - bubble_pocket_cuts() - wetness_pocket_cuts()
        + bubble_pocket_rims()
        + wetness_pad_frames()
        + witness_route_channels()
        + custody_latch_tabs("bubble_wetness_witness", BUBBLE_X, BUBBLE_Y, BUBBLE_Z);
    place_on_deck(part, BUBBLE_POS)
}

fn bubble_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_ecm_lot_window_station_bubble_witness_pocket_cuts");
    for index in 0..BUBBLE_WITNESS_POCKETS {
        let x = centered_index(index, BUBBLE_WITNESS_POCKETS, WITNESS_POCKET_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_bubble_witness_pocket_cut_{index}"),
                WITNESS_POCKET_D / 2.0,
                BUBBLE_Z + 6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, WITNESS_ROW_Y, BUBBLE_Z / 2.0);
    }
    cuts
}

fn wetness_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_ecm_lot_window_station_wetness_witness_pocket_cuts");
    for index in 0..WETNESS_WITNESS_POCKETS {
        let x = centered_index(index, WETNESS_WITNESS_POCKETS, WITNESS_POCKET_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("closed_ecm_lot_window_station_wetness_pad_recess_{index}"),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                12.0,
            )
            .translate(x, -WITNESS_ROW_Y, BUBBLE_Z - 5.5);
    }
    cuts
}

fn bubble_pocket_rims() -> Part {
    let mut rims = Part::empty("closed_ecm_lot_window_station_bubble_witness_pocket_rims");
    for index in 0..BUBBLE_WITNESS_POCKETS {
        let x = centered_index(index, BUBBLE_WITNESS_POCKETS, WITNESS_POCKET_PITCH_X);
        rims = rims
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_bubble_witness_rim_{index}"),
                WITNESS_POCKET_D / 2.0 + 2.5,
                4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, WITNESS_ROW_Y, BUBBLE_Z + 2.0)
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_bubble_witness_rim_open_{index}"),
                WITNESS_POCKET_D / 2.0 + 0.4,
                4.4,
                CYLINDER_SEGMENTS,
            )
            .translate(x, WITNESS_ROW_Y, BUBBLE_Z + 2.0);
    }
    rims
}

fn wetness_pad_frames() -> Part {
    let mut frames = Part::empty("closed_ecm_lot_window_station_wetness_pad_frames");
    for index in 0..WETNESS_WITNESS_POCKETS {
        let x = centered_index(index, WETNESS_WITNESS_POCKETS, WITNESS_POCKET_PITCH_X);
        frames = frames
            + centered_cube(
                format!("closed_ecm_lot_window_station_wetness_pad_frame_{index}"),
                WETNESS_PAD_X + 8.0,
                WETNESS_PAD_Y + 8.0,
                5.0,
            )
            .translate(x, -WITNESS_ROW_Y, BUBBLE_Z + 2.5)
            - centered_cube(
                format!("closed_ecm_lot_window_station_wetness_pad_open_{index}"),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                5.4,
            )
            .translate(x, -WITNESS_ROW_Y, BUBBLE_Z + 2.5);
    }
    frames
}

fn witness_route_channels() -> Part {
    let mut routes = Part::empty("closed_ecm_lot_window_station_bubble_wetness_route_channels");
    for index in 0..BUBBLE_WITNESS_POCKETS {
        let x = centered_index(index, BUBBLE_WITNESS_POCKETS, WITNESS_POCKET_PITCH_X);
        routes = routes
            + centered_cube(
                format!("closed_ecm_lot_window_station_bubble_to_wetness_route_{index}"),
                6.0,
                WITNESS_ROW_Y * 2.0 - 26.0,
                6.0,
            )
            .translate(x, 0.0, BUBBLE_Z + 3.0);
    }
    routes
}

fn barcode_coa_status_surfaces() -> Part {
    let panel = centered_cube(
        "closed_ecm_lot_window_station_barcode_coa_status_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);
    let part = panel + barcode_lands() + coa_card_surfaces() + status_token_lanes()
        - status_token_slot_cuts()
        + custody_latch_tabs("barcode_coa_status", TRACE_X, TRACE_Y, TRACE_Z);
    place_on_deck(part, TRACE_POS)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_ecm_lot_window_station_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let row = index / 6;
        let col = index % 6;
        lands = lands
            + centered_cube(
                format!("closed_ecm_lot_window_station_barcode_land_{index}"),
                44.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(col, 6, 54.0) - 34.0,
                58.0 - row as f64 * 32.0,
                TRACE_Z + 2.0,
            );
    }
    lands
}

fn coa_card_surfaces() -> Part {
    let mut cards = Part::empty("closed_ecm_lot_window_station_coa_card_surfaces");
    for index in 0..COA_CARD_SURFACES {
        cards = cards
            + centered_cube(
                format!("closed_ecm_lot_window_station_coa_card_surface_{index}"),
                66.0,
                34.0,
                4.0,
            )
            .translate(
                138.0,
                centered_index(index, COA_CARD_SURFACES, 44.0),
                TRACE_Z + 2.0,
            );
    }
    cards
}

fn status_token_lanes() -> Part {
    let mut lanes = Part::empty("closed_ecm_lot_window_station_status_token_lanes");
    for lane in 0..STATUS_LANES {
        lanes = lanes
            + centered_cube(
                format!("closed_ecm_lot_window_station_status_lane_rail_{lane}"),
                180.0,
                8.0,
                8.0,
            )
            .translate(-72.0, -70.0 + lane as f64 * 28.0, TRACE_Z + 4.0);
    }
    lanes
}

fn status_token_slot_cuts() -> Part {
    let mut slots = Part::empty("closed_ecm_lot_window_station_status_token_slot_cuts");
    for lane in 0..STATUS_LANES {
        for token in 0..STATUS_TOKENS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("closed_ecm_lot_window_station_status_lane_{lane}_token_{token}"),
                    28.0,
                    18.0,
                    9.0,
                )
                .translate(
                    -144.0 + token as f64 * 46.0,
                    -70.0 + lane as f64 * 28.0,
                    TRACE_Z - 4.0,
                );
        }
    }
    slots
}

fn closed_connector_handoff_bulkhead() -> Part {
    let base = centered_cube(
        "closed_ecm_lot_window_station_closed_handoff_base",
        HANDOFF_X,
        HANDOFF_Y,
        18.0,
    )
    .translate(0.0, 0.0, 9.0);
    let wall = centered_cube(
        "closed_ecm_lot_window_station_closed_connector_vertical_bulkhead",
        HANDOFF_X - 42.0,
        20.0,
        HANDOFF_Z,
    )
    .translate(0.0, 28.0, HANDOFF_Z / 2.0);
    let part = base
        + (wall - connector_port_cuts())
        + connector_collars()
        + cap_parks()
        + strain_relief_clamps()
        + custody_latch_tabs("closed_connector_handoff", HANDOFF_X, HANDOFF_Y, HANDOFF_Z);
    place_on_deck(part, HANDOFF_POS)
}

fn connector_port_cuts() -> Part {
    let mut cuts = Part::empty("closed_ecm_lot_window_station_connector_port_cuts");
    for index in 0..CLOSED_CONNECTOR_PORTS {
        let (x, z) = connector_position(index);
        cuts = cuts
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_closed_connector_bore_{index}"),
                CONNECTOR_PORT_D / 2.0,
                28.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 28.0, z);
    }
    cuts
}

fn connector_collars() -> Part {
    let mut collars = Part::empty("closed_ecm_lot_window_station_connector_collars");
    for index in 0..CLOSED_CONNECTOR_PORTS {
        let (x, z) = connector_position(index);
        collars = collars
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_closed_connector_collar_{index}"),
                CONNECTOR_COLLAR_D / 2.0,
                8.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 14.0, z)
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_closed_connector_collar_open_{index}"),
                CONNECTOR_PORT_D / 2.0,
                9.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 14.0, z);
    }
    collars
}

fn cap_parks() -> Part {
    let mut parks = Part::empty("closed_ecm_lot_window_station_closed_connector_cap_parks");
    for index in 0..CAP_PARKS {
        let x = centered_index(index % 4, 4, 64.0);
        let y = -68.0 + (index / 4) as f64 * 34.0;
        parks = parks
            + centered_cylinder(
                format!("closed_ecm_lot_window_station_closed_connector_cap_park_{index}"),
                12.0,
                8.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, 22.0)
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_closed_connector_cap_park_open_{index}"),
                7.0,
                9.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, 22.0);
    }
    parks
}

fn strain_relief_clamps() -> Part {
    let mut clamps = Part::empty("closed_ecm_lot_window_station_strain_relief_clamps");
    for index in 0..STRAIN_RELIEF_CLAMPS {
        clamps = clamps
            + centered_cube(
                format!("closed_ecm_lot_window_station_strain_relief_clamp_{index}"),
                34.0,
                14.0,
                12.0,
            )
            .translate(
                centered_index(index, STRAIN_RELIEF_CLAMPS, 46.0),
                -88.0,
                32.0,
            )
            - centered_cylinder(
                format!("closed_ecm_lot_window_station_strain_relief_tube_clearance_{index}"),
                5.0,
                38.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, STRAIN_RELIEF_CLAMPS, 46.0),
                -88.0,
                32.0,
            );
    }
    clamps
}

fn coating_station_handoff_bridge() -> Part {
    let base = centered_cube(
        "closed_ecm_lot_window_station_coating_handoff_bridge_base",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(0.0, 0.0, BRIDGE_Z / 2.0);
    let part =
        base - handoff_dock_window_cuts()
            + guide_tongues()
            + closed_route_segments()
            + fiducial_disc("closed_ecm_lot_window_station_coating_handoff_bridge_fiducial")
                .translate(0.0, 0.0, BRIDGE_Z + 2.0);
    place_on_deck(part, BRIDGE_POS)
}

fn handoff_dock_window_cuts() -> Part {
    let mut cuts = Part::empty("closed_ecm_lot_window_station_handoff_dock_window_cuts");
    for index in 0..COATING_HANDOFF_DOCKS {
        cuts = cuts
            + centered_cube(
                format!("closed_ecm_lot_window_station_coating_handoff_window_{index}"),
                80.0,
                38.0,
                BRIDGE_Z + 6.0,
            )
            .translate(
                centered_index(index, COATING_HANDOFF_DOCKS, 122.0),
                0.0,
                BRIDGE_Z / 2.0,
            );
    }
    cuts
}

fn guide_tongues() -> Part {
    let mut tongues = Part::empty("closed_ecm_lot_window_station_coating_handoff_guide_tongues");
    for index in 0..GUIDE_TONGUES {
        tongues = tongues
            + centered_cube(
                format!("closed_ecm_lot_window_station_coating_handoff_guide_tongue_{index}"),
                36.0,
                84.0,
                12.0,
            )
            .translate(centered_index(index, GUIDE_TONGUES, 92.0), -76.0, 28.0);
    }
    tongues
}

fn closed_route_segments() -> Part {
    let mut routes = Part::empty("closed_ecm_lot_window_station_handoff_closed_route_segments");
    for index in 0..ROUTE_SEGMENTS {
        routes = routes
            + centered_cube(
                format!("closed_ecm_lot_window_station_handoff_closed_route_segment_{index}"),
                8.0,
                BRIDGE_Y + 48.0,
                8.0,
            )
            .translate(
                centered_index(index, ROUTE_SEGMENTS, 32.0),
                0.0,
                BRIDGE_Z + 4.0,
            );
    }
    routes
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "closed_ecm_lot_window_station_front_robot_approach_keepout",
        STATION_X - 190.0,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_ecm_lot_window_station_rear_service_keepout",
        STATION_X - 220.0,
        KEEP_OUT_GAUGE_Z,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_ecm_lot_window_station_left_ecm_lot_service_keepout",
        KEEP_OUT_GAUGE_Z,
        STATION_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_ECM_LOT_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right_service = centered_cube(
        "closed_ecm_lot_window_station_right_connector_service_keepout",
        KEEP_OUT_GAUGE_Z,
        STATION_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_CONNECTOR_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top_cover_lift = centered_cube(
        "closed_ecm_lot_window_station_light_cover_lift_keepout",
        COVER_X + 80.0,
        COVER_Y + 60.0,
        12.0,
    )
    .translate(COVER_POS.0, COVER_POS.1, BASE_Z + TOP_COVER_LIFT_CLEARANCE);

    front_robot + rear_service + left_service + right_service + top_cover_lift
}

fn custody_latch_tabs(name: &str, x: f64, y: f64, z: f64) -> Part {
    let rear = centered_cube(
        format!("closed_ecm_lot_window_station_{name}_rear_custody_latch"),
        x - 54.0,
        8.0,
        16.0,
    )
    .translate(0.0, y / 2.0 - 18.0, z + 8.0);
    let front = centered_cube(
        format!("closed_ecm_lot_window_station_{name}_front_robot_pull_tab"),
        88.0,
        8.0,
        12.0,
    )
    .translate(0.0, -y / 2.0 + 18.0, z + 6.0);
    rear + front
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 8.0, 4.0, FIDUCIAL_SEGMENTS)
        - centered_cylinder(format!("{name}_center_dot"), 1.8, 5.0, 20)
        - centered_cube(format!("{name}_cross_x"), 13.0, 2.0, 5.0)
        - centered_cube(format!("{name}_cross_y"), 2.0, 13.0, 5.0)
}

fn place_on_deck(part: Part, center: (f64, f64)) -> Part {
    part.translate(center.0, center.1, BASE_Z)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn vial_position(index: usize) -> (f64, f64) {
    let col = index % VIAL_COLS;
    let row = index / VIAL_COLS;
    (
        -78.0 + centered_index(col, VIAL_COLS, VIAL_PITCH_X),
        centered_index(row, VIAL_ROWS, VIAL_PITCH_Y),
    )
}

fn connector_position(index: usize) -> (f64, f64) {
    let col = index % CONNECTOR_COLS;
    let row = index / CONNECTOR_COLS;
    (
        centered_index(col, CONNECTOR_COLS, CONNECTOR_PITCH_X),
        58.0 + row as f64 * CONNECTOR_PITCH_Z,
    )
}

fn module_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "ecm_lot_vial_bag_nests",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Footprint {
            name: "temperature_equilibration_blocks",
            center: TEMP_POS,
            x: TEMP_X,
            y: TEMP_Y,
        },
        Footprint {
            name: "low_shear_agitation_witness",
            center: AGITATION_POS,
            x: AGITATION_X,
            y: AGITATION_Y,
        },
        Footprint {
            name: "viscosity_reference_flow_coupon",
            center: VISCOSITY_POS,
            x: VISCOSITY_X,
            y: VISCOSITY_Y,
        },
        Footprint {
            name: "bubble_wetness_witness_pockets",
            center: BUBBLE_POS,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Footprint {
            name: "closed_connector_handoff_bulkhead",
            center: HANDOFF_POS,
            x: HANDOFF_X,
            y: HANDOFF_Y,
        },
        Footprint {
            name: "dispense_window_token_lanes",
            center: WINDOW_POS,
            x: WINDOW_X,
            y: WINDOW_Y,
        },
        Footprint {
            name: "barcode_coa_status_surfaces",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Footprint {
            name: "light_protection_cover",
            center: COVER_POS,
            x: COVER_X,
            y: COVER_Y,
        },
        Footprint {
            name: "coating_station_handoff_bridge",
            center: BRIDGE_POS,
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
    ]
}

fn assert_layout() {
    assert_eq!(ECM_VIAL_NESTS, VIAL_COLS * VIAL_ROWS);
    assert_eq!(TEMP_POCKETS, TEMP_BLOCKS * TEMP_POCKETS_PER_BLOCK);
    assert_eq!(CLOSED_CONNECTOR_PORTS, CONNECTOR_COLS * CONNECTOR_ROWS);
    assert_eq!(
        DISPENSE_WINDOW_TOKENS,
        DISPENSE_WINDOW_LANES * WINDOW_TOKENS_PER_LANE
    );
    assert_eq!(STATUS_TOKEN_SLOTS, STATUS_LANES * STATUS_TOKENS_PER_LANE);
    assert!(!CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED);

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station envelope",
            module.name
        );
    }
    for left_index in 0..footprints.len() {
        for right_index in (left_index + 1)..footprints.len() {
            assert!(
                !footprints[left_index].overlaps(footprints[right_index], MAJOR_MODULE_GAP_MM),
                "{} overlaps {}",
                footprints[left_index].name,
                footprints[right_index].name
            );
        }
    }

    assert!(connector_span_x() + CONNECTOR_COLLAR_D < HANDOFF_X - 54.0);
    assert!(TOP_COVER_LIFT_CLEARANCE > COVER_Z + BASE_Z + 90.0);
}

fn connector_span_x() -> f64 {
    (CONNECTOR_COLS - 1) as f64 * CONNECTOR_PITCH_X
}

fn rect(center: (f64, f64), x: f64, y: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - x / 2.0,
        center.0 + x / 2.0,
        center.1 - y / 2.0,
        center.1 + y / 2.0,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let expected = [
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_base_leak_tray_deck.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_ecm_lot_vial_bag_nests.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_temperature_equilibration_blocks.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_low_shear_agitation_witness.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_viscosity_reference_flow_coupon.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_dispense_window_token_lanes.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_light_protection_cover.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_bubble_wetness_witness_pockets.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_barcode_coa_status_surfaces.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_closed_connector_handoff_bulkhead.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_coating_station_handoff_bridge.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_robot_service_keepout_gauges.stl",
            "output/closed_ecm_lot_viscosity_temperature_dispense_window_station_assembly.stl",
        ];
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS, expected);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn required_features_cover_requested_validation_fixture_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for feature in [
            "ecm_lot_vial_bag_nests",
            "temperature_equilibration_blocks",
            "low_shear_agitation_witness",
            "viscosity_reference_flow_coupon",
            "dispense_window_token_lanes",
            "light_protection_cover",
            "bubble_wetness_witness_pockets",
            "barcode_coa_status_surfaces",
            "closed_connector_handoff_bulkhead",
            "coating_station_handoff_bridge",
            "robot_service_keepout_gauges",
            "validation_fixture_intent_only",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }

        for limitation in [
            "no_clinical_acceptance_thresholds",
            "no_ecm_release_limits",
            "no_sterility_validation_claim",
            "no_cell_outcome_acceptance_claim",
            "no_coating_protocol_thresholds",
        ] {
            assert!(LIMITATIONS.contains(&limitation), "{limitation}");
        }
    }

    #[test]
    fn feature_counts_match_closed_ecm_lot_validation_scope() {
        assert_eq!(ECM_VIAL_NESTS, 12);
        assert_eq!(ECM_BAG_NESTS, 3);
        assert_eq!(TEMP_BLOCKS, 2);
        assert_eq!(TEMP_POCKETS, 12);
        assert_eq!(AGITATION_WITNESS_ROLLERS, 5);
        assert_eq!(AGITATION_INDEX_TICKS, 9);
        assert_eq!(MIXING_WITNESS_WINDOWS, 6);
        assert_eq!(VISCOSITY_COUPON_LANES, 6);
        assert_eq!(REFERENCE_FLOW_CHANNELS, 6);
        assert_eq!(PRESSURE_TAP_PORTS, 12);
        assert_eq!(DISPENSE_WINDOW_LANES, 4);
        assert_eq!(DISPENSE_WINDOW_TOKENS, 20);
        assert_eq!(LIGHT_BAFFLES, 6);
        assert_eq!(BUBBLE_WITNESS_POCKETS, 8);
        assert_eq!(WETNESS_WITNESS_POCKETS, 8);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(COA_CARD_SURFACES, 4);
        assert_eq!(STATUS_TOKEN_SLOTS, 16);
        assert_eq!(CLOSED_CONNECTOR_PORTS, 8);
        assert_eq!(COATING_HANDOFF_DOCKS, 2);
    }

    #[test]
    fn station_bounds_and_major_modules_do_not_overlap() {
        assert_layout();
        let footprints = module_footprints();
        for module in footprints {
            assert!(module.fits_inside_station(), "{} does not fit", module.name);
        }
        for a in 0..footprints.len() {
            for b in (a + 1)..footprints.len() {
                assert!(
                    !footprints[a].overlaps(footprints[b], MAJOR_MODULE_GAP_MM),
                    "{} overlaps {}",
                    footprints[a].name,
                    footprints[b].name
                );
            }
        }
        assert!(STATION_X <= 1380.0);
        assert!(STATION_Y <= 900.0);
    }

    #[test]
    fn explicit_reproducibility_controls_are_pinned() {
        assert_eq!(
            PARAMETER_SET_REV,
            "closed-ecm-lot-viscosity-temperature-dispense-window-parametric-rev-a"
        );
        assert_eq!(OUTPUT_MANIFEST_REV, "stl-manifest-rev-a");
        assert_eq!(UNITS, "millimeters");
        assert!(!USES_RANDOMNESS);
        assert_eq!(GEOMETRY_SEED, 0);
        assert_eq!(CYLINDER_SEGMENTS, 32);
        assert_eq!(FIDUCIAL_SEGMENTS, 36);
        assert_eq!(FACET_TOLERANCE_MM, 0.25);
        assert!(!CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED);
    }

    #[test]
    fn closed_handoff_and_robot_service_keepouts_are_explicit() {
        assert_eq!(ROBOT_SERVICE_KEEPOUT_ZONES, 5);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
        assert!(REAR_SERVICE_CLEARANCE >= 240.0);
        assert!(LEFT_ECM_LOT_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_CONNECTOR_SERVICE_CLEARANCE >= 200.0);
        assert!(TOP_COVER_LIFT_CLEARANCE > COVER_Z + BASE_Z + 90.0);
        assert_eq!(CAP_PARKS, CLOSED_CONNECTOR_PORTS);
        assert_eq!(CONNECTOR_COLS * CONNECTOR_ROWS, CLOSED_CONNECTOR_PORTS);
        assert!(connector_span_x() + CONNECTOR_COLLAR_D < HANDOFF_X - 54.0);
        assert_eq!(GUIDE_TONGUES, 3);
        assert_eq!(ROUTE_SEGMENTS, CLOSED_CONNECTOR_PORTS);
    }
}
