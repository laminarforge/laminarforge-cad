use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-conditioning recirculation dead-zone tracer validation station.
//
// This generator models a contained validation fixture for checking whether a
// conditioned-media recirculation loop hides stagnant zones before media is fed
// to tissue-chip lanes. The CAD captures the mechanical evidence surfaces:
// loop surrogate geometry, tracer injection, timed sample wells, witness
// coupons, temperature/pH/osmolality pockets, bubble/degas windows, custody
// lands, disposition gates, camera bridge, and robot/service keepouts.
//
// This is architecture CAD only. It is not a sterile barrier claim, pressure
// vessel, media recipe, sensor calibration package, process limit, or
// biological release criterion.

const OUTPUT_PREFIX: &str =
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_";

const OUTPUTS: [&str; 14] = [
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_containment_deck.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_recirculation_loop_surrogate.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_tracer_injection_manifold.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_timed_sample_well_array.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_baffle_dead_zone_witness_coupon_bank.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_temperature_ph_osmolality_pocket_panel.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_bubble_degas_witness_window_panel.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_chip_feed_handoff_bulkhead.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_barcode_coa_custody_lands.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_release_hold_reject_gate_bank.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_camera_evidence_bridge.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_robot_service_keepout_gauges.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_tracer_route_dead_zone_overlay.stl",
    "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 24] = [
    "containment_deck",
    "closed_recirculation_loop_surrogate",
    "loop_inlet_outlet_ports",
    "tracer_injection_ports",
    "tracer_standard_cups",
    "timed_sample_wells",
    "sample_time_token_rail",
    "baffle_witness_coupons",
    "dead_zone_witness_coupons",
    "temperature_pockets",
    "ph_pockets",
    "osmolality_pockets",
    "bubble_witness_windows",
    "degas_witness_windows",
    "chip_feed_handoff_bulkhead",
    "barcode_custody_lands",
    "coa_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "robot_keepouts",
    "service_keepouts",
    "tracer_route_overlay",
];

const RESEARCH_BASIS: [&str; 6] = [
    "residence_time_distribution_tracer_studies_reveal_dead_zones",
    "recirculation_loop_mixing_controls_media_homogeneity_before_chip_feed",
    "inline_ph_and_temperature_sensing_support_upstream_process_control",
    "osmolality_and_sample_timing_require_custody_before_release",
    "bubble_degas_visual_witnesses_reduce_air_ingress_before_perfusion",
    "release_hold_reject_gates_preserve_process_validation_evidence",
];

const LIMITATIONS: [&str; 7] = [
    "architecture_cad_only",
    "not_a_sterile_barrier_claim",
    "not_a_pressure_rated_recirculation_loop",
    "no_media_recipe_or_process_limits",
    "no_clinical_release_thresholds",
    "no_sensor_metrology_claim",
    "purchased_pumps_sensors_connectors_and_windows_are_surrogates",
];

const PARAMETRIC_REVISION: &str =
    "closed_media_conditioning_recirculation_dead_zone_tracer_station_v1";
const UNITS: &str = "millimeters";
const GRID_STEP_MM: f64 = 2.0;
const CYLINDER_SEGMENTS: u32 = 32;
const FIDUCIAL_SEGMENTS: u32 = 36;

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 960.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const SUMP_DEPTH: f64 = 7.0;
const MODULE_CLEARANCE_MM: f64 = 16.0;
const MODULE_MARGIN_MM: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 18.0;

const LOOP_POS: (f64, f64) = (-465.0, 235.0);
const LOOP_X: f64 = 500.0;
const LOOP_Y: f64 = 220.0;
const LOOP_Z: f64 = 66.0;
const LOOP_LANES: usize = 6;
const LOOP_PORTS: usize = LOOP_LANES * 2;
const LOOP_CHANNEL_D: f64 = 8.4;
const LOOP_LANE_PITCH: f64 = 30.0;
const RECIRC_PUMP_SURROGATES: usize = 2;
const LOOP_DIRECTION_MARKERS: usize = LOOP_LANES * 2;

const TRACER_POS: (f64, f64) = (90.0, 235.0);
const TRACER_X: f64 = 410.0;
const TRACER_Y: f64 = 220.0;
const TRACER_Z: f64 = 56.0;
const TRACER_INJECTION_PORTS: usize = 8;
const TRACER_STANDARD_CUPS: usize = 4;
const TRACER_SEPTUM_POCKETS: usize = 8;
const TRACER_PORT_D: f64 = 12.0;
const TRACER_PORT_PITCH: f64 = 42.0;

const SAMPLE_POS: (f64, f64) = (520.0, 235.0);
const SAMPLE_X: f64 = 330.0;
const SAMPLE_Y: f64 = 220.0;
const SAMPLE_Z: f64 = 54.0;
const SAMPLE_TIMEPOINTS: usize = 8;
const SAMPLE_REPLICATES: usize = 3;
const TIMED_SAMPLE_WELLS: usize = SAMPLE_TIMEPOINTS * SAMPLE_REPLICATES;
const SAMPLE_WELL_D: f64 = 15.0;
const SAMPLE_COLS: usize = 6;
const SAMPLE_ROWS: usize = 4;
const SAMPLE_PITCH_X: f64 = 42.0;
const SAMPLE_PITCH_Y: f64 = 36.0;
const SAMPLE_TIME_TOKENS: usize = SAMPLE_TIMEPOINTS;

const COUPON_POS: (f64, f64) = (-465.0, -55.0);
const COUPON_X: f64 = 500.0;
const COUPON_Y: f64 = 230.0;
const COUPON_Z: f64 = 50.0;
const BAFFLE_WITNESS_COUPONS: usize = 6;
const DEAD_ZONE_WITNESS_COUPONS: usize = 6;
const TOTAL_WITNESS_COUPONS: usize = BAFFLE_WITNESS_COUPONS + DEAD_ZONE_WITNESS_COUPONS;
const COUPON_SLOT_X: f64 = 32.0;
const COUPON_SLOT_Y: f64 = 92.0;
const COUPON_PITCH_X: f64 = 72.0;
const BAFFLE_RIBS_PER_COUPON: usize = 3;

const SENSOR_POS: (f64, f64) = (90.0, -55.0);
const SENSOR_X: f64 = 410.0;
const SENSOR_Y: f64 = 230.0;
const SENSOR_Z: f64 = 54.0;
const TEMPERATURE_POCKETS: usize = 4;
const PH_POCKETS: usize = 4;
const OSMOLALITY_POCKETS: usize = 4;
const SENSOR_POCKET_ROWS: usize = 3;
const SENSOR_POCKET_COLS: usize = 4;
const SENSOR_POCKET_D: f64 = 19.0;
const SENSOR_POCKET_PITCH_X: f64 = 62.0;
const SENSOR_POCKET_PITCH_Y: f64 = 58.0;
const SENSOR_REFERENCE_LANDS: usize = 6;

const BUBBLE_POS: (f64, f64) = (520.0, -55.0);
const BUBBLE_X: f64 = 330.0;
const BUBBLE_Y: f64 = 230.0;
const BUBBLE_Z: f64 = 52.0;
const BUBBLE_WINDOWS: usize = 6;
const DEGAS_WITNESS_WINDOWS: usize = 6;
const DEGAS_VENT_CAPTURE_CUPS: usize = 6;
const WITNESS_WINDOW_X: f64 = 30.0;
const WITNESS_WINDOW_Y: f64 = 62.0;
const WITNESS_WINDOW_PITCH_X: f64 = 46.0;

const HANDOFF_POS: (f64, f64) = (-465.0, -330.0);
const HANDOFF_X: f64 = 360.0;
const HANDOFF_Y: f64 = 170.0;
const HANDOFF_Z: f64 = 72.0;
const CHIP_FEED_CONNECTORS: usize = 8;
const CHIP_FEED_GROUPS: usize = 4;
const HANDOFF_CONNECTOR_D: f64 = 17.0;
const HANDOFF_CONNECTOR_PITCH: f64 = 42.0;

const CUSTODY_POS: (f64, f64) = (-80.0, -330.0);
const CUSTODY_X: f64 = 350.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 4;
const TAMPER_SEAL_PADS: usize = 6;
const CUSTODY_TOKEN_SLOTS: usize = 8;

const GATE_POS: (f64, f64) = (315.0, -330.0);
const GATE_X: f64 = 330.0;
const GATE_Y: f64 = 170.0;
const GATE_Z: f64 = 36.0;
const DISPOSITION_NAMES: [&str; 3] = ["release", "hold", "reject"];
const DISPOSITION_STATES: usize = DISPOSITION_NAMES.len();
const GATE_TOKEN_SLOTS: usize = DISPOSITION_STATES * 4;

const CAMERA_POS: (f64, f64) = (0.0, -426.0);
const CAMERA_BRIDGE_X: f64 = 1240.0;
const CAMERA_BRIDGE_Y: f64 = 42.0;
const CAMERA_BRIDGE_Z: f64 = 210.0;
const CAMERA_MOUNTS: usize = 6;
const EVIDENCE_FIDUCIALS: usize = 12;
const CAMERA_CLEARANCE_Z: f64 = 245.0;

const ROBOT_FRONT_CLEARANCE: f64 = 430.0;
const LEFT_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 220.0;
const REAR_SAMPLE_SERVICE_CLEARANCE: f64 = 245.0;
const TOP_LOOP_SERVICE_CLEARANCE: f64 = 295.0;

const ROUTE_SEGMENTS: usize = 12;
const ROUTE_MARKERS: usize = 14;

#[derive(Clone, Copy, Debug)]
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

    fn overlaps_with_clearance(self, other: Footprint, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug)]
struct RouteSegment {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let loop_surrogate = recirculation_loop_surrogate();
    export(OUTPUTS[1], &loop_surrogate);

    let tracer = tracer_injection_manifold();
    export(OUTPUTS[2], &tracer);

    let samples = timed_sample_well_array();
    export(OUTPUTS[3], &samples);

    let coupons = baffle_dead_zone_witness_coupon_bank();
    export(OUTPUTS[4], &coupons);

    let sensors = temperature_ph_osmolality_pocket_panel();
    export(OUTPUTS[5], &sensors);

    let bubble = bubble_degas_witness_window_panel();
    export(OUTPUTS[6], &bubble);

    let handoff = chip_feed_handoff_bulkhead();
    export(OUTPUTS[7], &handoff);

    let custody = barcode_coa_custody_lands();
    export(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gate_bank();
    export(OUTPUTS[9], &gates);

    let camera = camera_evidence_bridge();
    export(OUTPUTS[10], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let overlay = tracer_route_dead_zone_overlay();
    export(OUTPUTS[12], &overlay);

    let assembly = deck
        + loop_surrogate
        + tracer
        + samples
        + coupons
        + sensors
        + bubble
        + handoff
        + custody
        + gates
        + camera
        + keepouts
        + overlay;
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed media-conditioning recirculation dead-zone tracer station:");
    println!("  Revision/units:              {PARAMETRIC_REVISION} / {UNITS}");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Recirculation loop:          {LOOP_LANES} lanes, {LOOP_PORTS} loop ports, {RECIRC_PUMP_SURROGATES} pump surrogates"
    );
    println!(
        "  Tracer and samples:          {TRACER_INJECTION_PORTS} injection ports, {TIMED_SAMPLE_WELLS} timed sample wells, {SAMPLE_TIME_TOKENS} time tokens"
    );
    println!(
        "  Dead-zone evidence:          {BAFFLE_WITNESS_COUPONS} baffle coupons, {DEAD_ZONE_WITNESS_COUPONS} dead-zone coupons, {BUBBLE_WINDOWS} bubble windows, {DEGAS_WITNESS_WINDOWS} degas windows"
    );
    println!(
        "  Chemistry/custody/release:   {} sensor pockets, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, release/hold/reject gates",
        TEMPERATURE_POCKETS + PH_POCKETS + OSMOLALITY_POCKETS
    );
    println!(
        "  Reproducibility controls:    fixed manifest, static feature counts, no random inputs, {GRID_STEP_MM:.0}mm layout grid"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z
}

fn place_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn module_footprints() -> [Footprint; 9] {
    [
        footprint("recirculation_loop_surrogate", LOOP_POS, LOOP_X, LOOP_Y),
        footprint("tracer_injection_manifold", TRACER_POS, TRACER_X, TRACER_Y),
        footprint("timed_sample_well_array", SAMPLE_POS, SAMPLE_X, SAMPLE_Y),
        footprint(
            "baffle_dead_zone_witness_coupon_bank",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        footprint(
            "temperature_ph_osmolality_pocket_panel",
            SENSOR_POS,
            SENSOR_X,
            SENSOR_Y,
        ),
        footprint(
            "bubble_degas_witness_window_panel",
            BUBBLE_POS,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        footprint(
            "chip_feed_handoff_bulkhead",
            HANDOFF_POS,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        footprint(
            "barcode_coa_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        footprint("release_hold_reject_gate_bank", GATE_POS, GATE_X, GATE_Y),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "recirculation_dead_zone_tracer_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "recirculation_dead_zone_tracer_shallow_sump_recess",
        STATION_X - 142.0,
        STATION_Y - 136.0,
        SUMP_DEPTH,
    )
    .translate(0.0, -10.0, deck_top_z() - SUMP_DEPTH / 2.0 + 0.2);
    let drain = centered_cylinder(
        "recirculation_dead_zone_tracer_front_drain",
        DRAIN_D / 2.0,
        68.0,
        CYLINDER_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 82.0,
        -STATION_Y / 2.0 + 8.0,
        deck_top_z() - 5.0,
    );

    deck - sump - drain - module_insert_sockets() - mounting_holes()
        + perimeter_rims()
        + datum_targets()
        + zone_locator_rails()
}

fn module_insert_sockets() -> Part {
    let mut sockets = Part::empty("recirculation_dead_zone_tracer_module_insert_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("recirculation_dead_zone_tracer_{}_socket", module.name),
                module.x + 12.0,
                module.y + 12.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                deck_top_z() - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("recirculation_dead_zone_tracer_mounting_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-245.0, 86.0),
        (245.0, 86.0),
        (-245.0, -230.0),
        (245.0, -230.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "recirculation_dead_zone_tracer_left_retention_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "recirculation_dead_zone_tracer_right_retention_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "recirculation_dead_zone_tracer_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "recirculation_dead_zone_tracer_low_front_robot_lip",
        STATION_X - 220.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 16.0, deck_top_z() + 10.0);

    left + right + rear + front
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("recirculation_dead_zone_tracer_robot_datum_targets");
    for (i, (x, y)) in [
        (-670.0, -392.0),
        (670.0, -392.0),
        (-670.0, 392.0),
        (670.0, 392.0),
        (-235.0, 392.0),
        (235.0, 392.0),
    ]
    .iter()
    .enumerate()
    {
        let outer = centered_cylinder(
            format!("recirculation_dead_zone_tracer_datum_outer_{i}"),
            9.0,
            4.0,
            FIDUCIAL_SEGMENTS,
        )
        .translate(*x, *y, deck_top_z() + 2.0);
        let inner = centered_cylinder(
            format!("recirculation_dead_zone_tracer_datum_inner_{i}"),
            3.6,
            6.0,
            24,
        )
        .translate(*x, *y, deck_top_z() + 2.0);
        targets = targets + (outer - inner);
    }
    targets
}

fn zone_locator_rails() -> Part {
    let rear = centered_cube(
        "recirculation_dead_zone_tracer_loop_tracer_sample_locator_rail",
        STATION_X - 270.0,
        9.0,
        16.0,
    )
    .translate(0.0, 85.0, deck_top_z() + 8.0);
    let middle = centered_cube(
        "recirculation_dead_zone_tracer_coupon_sensor_window_locator_rail",
        STATION_X - 310.0,
        9.0,
        16.0,
    )
    .translate(0.0, -178.0, deck_top_z() + 8.0);
    let front = centered_cube(
        "recirculation_dead_zone_tracer_handoff_custody_gate_locator_rail",
        1180.0,
        9.0,
        16.0,
    )
    .translate(-75.0, -430.0, deck_top_z() + 8.0);
    rear + middle + front
}

fn recirculation_loop_surrogate() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_loop_surrogate_body",
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    )
    .translate(LOOP_POS.0, LOOP_POS.1, place_z(LOOP_Z));
    body - loop_channel_bores()
        + loop_port_gasket_lands()
        + recirculation_pump_surrogates()
        + loop_flow_direction_markers()
        + loop_return_u_turn_windows()
}

fn loop_channel_bores() -> Part {
    let mut bores = Part::empty("recirculation_dead_zone_tracer_loop_channel_bores");
    for lane in 0..LOOP_LANES {
        let y = LOOP_POS.1 + centered_index(lane, LOOP_LANES, LOOP_LANE_PITCH);
        let main = centered_cylinder(
            format!("recirculation_dead_zone_tracer_lane_{lane}_main_bore"),
            LOOP_CHANNEL_D / 2.0,
            LOOP_X - 68.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(LOOP_POS.0, y, deck_top_z() + LOOP_Z * 0.58);
        let return_leg = centered_cylinder(
            format!("recirculation_dead_zone_tracer_lane_{lane}_return_bore"),
            LOOP_CHANNEL_D / 2.0,
            LOOP_X - 120.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(LOOP_POS.0, y + 10.0, deck_top_z() + LOOP_Z * 0.32);
        bores = bores + main + return_leg;
    }
    bores
}

fn loop_port_gasket_lands() -> Part {
    let mut lands = Part::empty("recirculation_dead_zone_tracer_loop_port_gasket_lands");
    for lane in 0..LOOP_LANES {
        let y = LOOP_POS.1 + centered_index(lane, LOOP_LANES, LOOP_LANE_PITCH);
        for side in 0..2 {
            let x = LOOP_POS.0
                + if side == 0 {
                    -LOOP_X / 2.0 + 34.0
                } else {
                    LOOP_X / 2.0 - 34.0
                };
            let land = centered_cylinder(
                format!("recirculation_dead_zone_tracer_loop_lane_{lane}_port_land_{side}"),
                16.0,
                6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + LOOP_Z + 3.0);
            let bore = centered_cylinder(
                format!("recirculation_dead_zone_tracer_loop_lane_{lane}_port_bore_{side}"),
                LOOP_CHANNEL_D / 2.0 + 0.8,
                9.0,
                24,
            )
            .translate(x, y, deck_top_z() + LOOP_Z + 3.0);
            lands = lands + (land - bore);
        }
    }
    lands
}

fn recirculation_pump_surrogates() -> Part {
    let mut pumps = Part::empty("recirculation_dead_zone_tracer_pump_surrogates");
    for i in 0..RECIRC_PUMP_SURROGATES {
        let x = LOOP_POS.0 + centered_index(i, RECIRC_PUMP_SURROGATES, 175.0);
        let housing = centered_cube(
            format!("recirculation_dead_zone_tracer_pump_surrogate_housing_{i}"),
            92.0,
            46.0,
            22.0,
        )
        .translate(
            x,
            LOOP_POS.1 - LOOP_Y / 2.0 + 38.0,
            deck_top_z() + LOOP_Z + 11.0,
        );
        let roller = centered_cylinder(
            format!("recirculation_dead_zone_tracer_pump_surrogate_roller_{i}"),
            18.0,
            18.0,
            CYLINDER_SEGMENTS,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            LOOP_POS.1 - LOOP_Y / 2.0 + 38.0,
            deck_top_z() + LOOP_Z + 12.0,
        );
        pumps = pumps + housing + roller;
    }
    pumps
}

fn loop_flow_direction_markers() -> Part {
    let mut markers = Part::empty("recirculation_dead_zone_tracer_loop_flow_direction_markers");
    for marker in 0..LOOP_DIRECTION_MARKERS {
        let lane = marker / 2;
        let phase = marker % 2;
        let x = LOOP_POS.0 + if phase == 0 { -108.0 } else { 108.0 };
        let y = LOOP_POS.1 + centered_index(lane, LOOP_LANES, LOOP_LANE_PITCH);
        markers = markers
            + centered_cube(
                format!("recirculation_dead_zone_tracer_loop_flow_arrow_marker_{marker}"),
                26.0,
                8.0,
                5.0,
            )
            .translate(x, y, deck_top_z() + LOOP_Z + 4.0)
            + centered_cube(
                format!("recirculation_dead_zone_tracer_loop_flow_arrow_tip_{marker}"),
                8.0,
                14.0,
                5.0,
            )
            .translate(
                x + if phase == 0 { 16.0 } else { -16.0 },
                y,
                deck_top_z() + LOOP_Z + 4.0,
            );
    }
    markers
}

fn loop_return_u_turn_windows() -> Part {
    let mut windows = Part::empty("recirculation_dead_zone_tracer_loop_return_u_turn_windows");
    for lane in 0..LOOP_LANES {
        let y = LOOP_POS.1 + centered_index(lane, LOOP_LANES, LOOP_LANE_PITCH);
        for side in 0..2 {
            let x = LOOP_POS.0
                + if side == 0 {
                    -LOOP_X / 2.0 + 72.0
                } else {
                    LOOP_X / 2.0 - 72.0
                };
            windows = windows
                + centered_cube(
                    format!("recirculation_dead_zone_tracer_visible_u_turn_window_{lane}_{side}"),
                    42.0,
                    12.0,
                    8.0,
                )
                .translate(x, y + 9.0, deck_top_z() + LOOP_Z + 4.0);
        }
    }
    windows
}

fn tracer_injection_manifold() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_injection_manifold_body",
        TRACER_X,
        TRACER_Y,
        TRACER_Z,
    )
    .translate(TRACER_POS.0, TRACER_POS.1, place_z(TRACER_Z));
    body - tracer_manifold_channels() - tracer_injection_bores()
        + tracer_injection_collars()
        + tracer_standard_cups()
        + tracer_septum_pockets()
}

fn tracer_manifold_channels() -> Part {
    let main = centered_cylinder(
        "recirculation_dead_zone_tracer_injection_main_channel",
        4.2,
        TRACER_X - 52.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        TRACER_POS.0,
        TRACER_POS.1 + 26.0,
        deck_top_z() + TRACER_Z * 0.52,
    );
    let branch = centered_cylinder(
        "recirculation_dead_zone_tracer_injection_branch_channel",
        3.6,
        TRACER_Y - 68.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TRACER_POS.0, TRACER_POS.1, deck_top_z() + TRACER_Z * 0.52);
    main + branch
}

fn tracer_injection_bores() -> Part {
    let mut bores = Part::empty("recirculation_dead_zone_tracer_injection_port_bores");
    for i in 0..TRACER_INJECTION_PORTS {
        let x = TRACER_POS.0 + centered_index(i, TRACER_INJECTION_PORTS, TRACER_PORT_PITCH);
        let y = TRACER_POS.1 + if i % 2 == 0 { 48.0 } else { 12.0 };
        bores = bores
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_injection_port_bore_{i}"),
                TRACER_PORT_D / 2.0,
                TRACER_Z + 12.0,
                24,
            )
            .translate(x, y, place_z(TRACER_Z));
    }
    bores
}

fn tracer_injection_collars() -> Part {
    let mut collars = Part::empty("recirculation_dead_zone_tracer_injection_port_collars");
    for i in 0..TRACER_INJECTION_PORTS {
        let x = TRACER_POS.0 + centered_index(i, TRACER_INJECTION_PORTS, TRACER_PORT_PITCH);
        let y = TRACER_POS.1 + if i % 2 == 0 { 48.0 } else { 12.0 };
        let collar = centered_cylinder(
            format!("recirculation_dead_zone_tracer_injection_port_collar_{i}"),
            15.0,
            7.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + TRACER_Z + 3.5);
        let clearance = centered_cylinder(
            format!("recirculation_dead_zone_tracer_injection_port_collar_clearance_{i}"),
            TRACER_PORT_D / 2.0 + 0.6,
            9.0,
            24,
        )
        .translate(x, y, deck_top_z() + TRACER_Z + 3.5);
        collars = collars + (collar - clearance);
    }
    collars
}

fn tracer_standard_cups() -> Part {
    let mut cups = Part::empty("recirculation_dead_zone_tracer_standard_cups");
    for i in 0..TRACER_STANDARD_CUPS {
        let x = TRACER_POS.0 + centered_index(i, TRACER_STANDARD_CUPS, 62.0);
        let y = TRACER_POS.1 - TRACER_Y / 2.0 + 44.0;
        let cup = centered_cylinder(
            format!("recirculation_dead_zone_tracer_standard_cup_{i}"),
            17.0,
            10.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + TRACER_Z + 5.0);
        let well = centered_cylinder(
            format!("recirculation_dead_zone_tracer_standard_cup_well_{i}"),
            10.0,
            14.0,
            24,
        )
        .translate(x, y, deck_top_z() + TRACER_Z + 5.0);
        cups = cups + (cup - well);
    }
    cups
}

fn tracer_septum_pockets() -> Part {
    let mut pockets = Part::empty("recirculation_dead_zone_tracer_septum_pockets");
    for i in 0..TRACER_SEPTUM_POCKETS {
        let x = TRACER_POS.0 + centered_index(i, TRACER_SEPTUM_POCKETS, 40.0);
        let pocket = centered_cube(
            format!("recirculation_dead_zone_tracer_septum_pocket_{i}"),
            28.0,
            22.0,
            8.0,
        )
        .translate(x, TRACER_POS.1 - 28.0, deck_top_z() + TRACER_Z + 4.0);
        pockets = pockets + pocket;
    }
    pockets
}

fn timed_sample_well_array() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_timed_sample_array_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_POS.0, SAMPLE_POS.1, place_z(SAMPLE_Z));
    body - timed_sample_well_bores() + timed_sample_well_collars() + sample_time_token_rail()
}

fn timed_sample_well_bores() -> Part {
    let mut bores = Part::empty("recirculation_dead_zone_tracer_timed_sample_well_bores");
    for i in 0..TIMED_SAMPLE_WELLS {
        let (x, y) = sample_well_center(i);
        bores = bores
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_timed_sample_well_bore_{i}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_Z + 10.0,
                24,
            )
            .translate(SAMPLE_POS.0 + x, SAMPLE_POS.1 + y, place_z(SAMPLE_Z));
    }
    bores
}

fn timed_sample_well_collars() -> Part {
    let mut collars = Part::empty("recirculation_dead_zone_tracer_timed_sample_well_collars");
    for i in 0..TIMED_SAMPLE_WELLS {
        let (x, y) = sample_well_center(i);
        let collar = centered_cylinder(
            format!("recirculation_dead_zone_tracer_timed_sample_well_collar_{i}"),
            SAMPLE_WELL_D / 2.0 + 3.5,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(
            SAMPLE_POS.0 + x,
            SAMPLE_POS.1 + y,
            deck_top_z() + SAMPLE_Z + 2.5,
        );
        let clearance = centered_cylinder(
            format!("recirculation_dead_zone_tracer_timed_sample_well_clearance_{i}"),
            SAMPLE_WELL_D / 2.0,
            7.0,
            24,
        )
        .translate(
            SAMPLE_POS.0 + x,
            SAMPLE_POS.1 + y,
            deck_top_z() + SAMPLE_Z + 2.5,
        );
        collars = collars + (collar - clearance);
    }
    collars
}

fn sample_time_token_rail() -> Part {
    let rail = centered_cube(
        "recirculation_dead_zone_tracer_sample_time_token_rail",
        SAMPLE_X - 44.0,
        28.0,
        12.0,
    )
    .translate(
        SAMPLE_POS.0,
        SAMPLE_POS.1 - SAMPLE_Y / 2.0 + 24.0,
        deck_top_z() + SAMPLE_Z + 6.0,
    );
    let mut tokens = Part::empty("recirculation_dead_zone_tracer_sample_time_tokens");
    for i in 0..SAMPLE_TIME_TOKENS {
        tokens = tokens
            + centered_cube(
                format!("recirculation_dead_zone_tracer_sample_time_token_{i}"),
                18.0,
                18.0,
                4.0 + i as f64,
            )
            .translate(
                SAMPLE_POS.0 + centered_index(i, SAMPLE_TIME_TOKENS, 34.0),
                SAMPLE_POS.1 - SAMPLE_Y / 2.0 + 24.0,
                deck_top_z() + SAMPLE_Z + 14.0 + i as f64 / 2.0,
            );
    }
    rail + tokens
}

fn sample_well_center(index: usize) -> (f64, f64) {
    let col = index % SAMPLE_COLS;
    let row = index / SAMPLE_COLS;
    (
        centered_index(col, SAMPLE_COLS, SAMPLE_PITCH_X),
        centered_index(row, SAMPLE_ROWS, SAMPLE_PITCH_Y) + 24.0,
    )
}

fn baffle_dead_zone_witness_coupon_bank() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_coupon_bank_body",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(COUPON_POS.0, COUPON_POS.1, place_z(COUPON_Z));
    body - coupon_slots() + baffle_witness_coupons() + dead_zone_witness_coupons()
}

fn coupon_slots() -> Part {
    let mut slots = Part::empty("recirculation_dead_zone_tracer_coupon_slots");
    for i in 0..TOTAL_WITNESS_COUPONS {
        let row = i / BAFFLE_WITNESS_COUPONS;
        let col = i % BAFFLE_WITNESS_COUPONS;
        let x = COUPON_POS.0 + centered_index(col, BAFFLE_WITNESS_COUPONS, COUPON_PITCH_X);
        let y = COUPON_POS.1 + if row == 0 { 42.0 } else { -42.0 };
        slots = slots
            + centered_cube(
                format!("recirculation_dead_zone_tracer_coupon_slot_{i}"),
                COUPON_SLOT_X + 4.0,
                COUPON_SLOT_Y + 8.0,
                16.0,
            )
            .translate(x, y, deck_top_z() + COUPON_Z - 4.0);
    }
    slots
}

fn baffle_witness_coupons() -> Part {
    let mut coupons = Part::empty("recirculation_dead_zone_tracer_baffle_witness_coupons");
    for i in 0..BAFFLE_WITNESS_COUPONS {
        let x = COUPON_POS.0 + centered_index(i, BAFFLE_WITNESS_COUPONS, COUPON_PITCH_X);
        let base = centered_cube(
            format!("recirculation_dead_zone_tracer_baffle_coupon_card_{i}"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            8.0,
        )
        .translate(x, COUPON_POS.1 + 42.0, deck_top_z() + COUPON_Z + 4.0);
        coupons = coupons + base;
        for rib in 0..BAFFLE_RIBS_PER_COUPON {
            coupons = coupons
                + centered_cube(
                    format!("recirculation_dead_zone_tracer_baffle_coupon_{i}_rib_{rib}"),
                    24.0,
                    6.0,
                    18.0,
                )
                .translate(
                    x,
                    COUPON_POS.1 + 12.0 + rib as f64 * 24.0,
                    deck_top_z() + COUPON_Z + 17.0,
                );
        }
    }
    coupons
}

fn dead_zone_witness_coupons() -> Part {
    let mut coupons = Part::empty("recirculation_dead_zone_tracer_dead_zone_witness_coupons");
    for i in 0..DEAD_ZONE_WITNESS_COUPONS {
        let x = COUPON_POS.0 + centered_index(i, DEAD_ZONE_WITNESS_COUPONS, COUPON_PITCH_X);
        let card = centered_cube(
            format!("recirculation_dead_zone_tracer_dead_zone_coupon_card_{i}"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            8.0,
        )
        .translate(x, COUPON_POS.1 - 42.0, deck_top_z() + COUPON_Z + 4.0);
        let pocket = centered_cylinder(
            format!("recirculation_dead_zone_tracer_dead_zone_coupon_stagnant_pocket_{i}"),
            11.0 + i as f64,
            7.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, COUPON_POS.1 - 42.0, deck_top_z() + COUPON_Z + 12.0);
        coupons = coupons + card + pocket;
    }
    coupons
}

fn temperature_ph_osmolality_pocket_panel() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_temperature_ph_osmolality_panel_body",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, place_z(SENSOR_Z));
    body - sensor_pocket_bores() + sensor_pocket_collars() + sensor_reference_lands()
}

fn sensor_pocket_bores() -> Part {
    let mut bores = Part::empty("recirculation_dead_zone_tracer_sensor_pocket_bores");
    for row in 0..SENSOR_POCKET_ROWS {
        for col in 0..SENSOR_POCKET_COLS {
            let index = row * SENSOR_POCKET_COLS + col;
            let x = SENSOR_POS.0 + centered_index(col, SENSOR_POCKET_COLS, SENSOR_POCKET_PITCH_X);
            let y = SENSOR_POS.1 + centered_index(row, SENSOR_POCKET_ROWS, SENSOR_POCKET_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("recirculation_dead_zone_tracer_sensor_pocket_bore_{index}"),
                    SENSOR_POCKET_D / 2.0,
                    SENSOR_Z + 10.0,
                    24,
                )
                .translate(x, y, place_z(SENSOR_Z));
        }
    }
    bores
}

fn sensor_pocket_collars() -> Part {
    let mut collars = Part::empty("recirculation_dead_zone_tracer_sensor_pocket_collars");
    for row in 0..SENSOR_POCKET_ROWS {
        let prefix = match row {
            0 => "temperature",
            1 => "ph",
            _ => "osmolality",
        };
        for col in 0..SENSOR_POCKET_COLS {
            let x = SENSOR_POS.0 + centered_index(col, SENSOR_POCKET_COLS, SENSOR_POCKET_PITCH_X);
            let y = SENSOR_POS.1 + centered_index(row, SENSOR_POCKET_ROWS, SENSOR_POCKET_PITCH_Y);
            let collar = centered_cylinder(
                format!("recirculation_dead_zone_tracer_{prefix}_pocket_collar_{col}"),
                SENSOR_POCKET_D / 2.0 + 4.0,
                6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + SENSOR_Z + 3.0);
            let clearance = centered_cylinder(
                format!("recirculation_dead_zone_tracer_{prefix}_pocket_clearance_{col}"),
                SENSOR_POCKET_D / 2.0,
                8.0,
                24,
            )
            .translate(x, y, deck_top_z() + SENSOR_Z + 3.0);
            collars = collars + (collar - clearance);
        }
    }
    collars
}

fn sensor_reference_lands() -> Part {
    let mut lands = Part::empty("recirculation_dead_zone_tracer_sensor_reference_lands");
    for i in 0..SENSOR_REFERENCE_LANDS {
        lands = lands
            + centered_cube(
                format!("recirculation_dead_zone_tracer_sensor_reference_land_{i}"),
                38.0,
                18.0,
                5.0,
            )
            .translate(
                SENSOR_POS.0 + centered_index(i, SENSOR_REFERENCE_LANDS, 48.0),
                SENSOR_POS.1 - SENSOR_Y / 2.0 + 26.0,
                deck_top_z() + SENSOR_Z + 2.5,
            );
    }
    lands
}

fn bubble_degas_witness_window_panel() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_bubble_degas_window_panel_body",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(BUBBLE_POS.0, BUBBLE_POS.1, place_z(BUBBLE_Z));
    body - bubble_window_cutouts() - degas_window_cutouts()
        + bubble_window_frames()
        + degas_window_frames()
        + degas_vent_capture_cups()
}

fn bubble_window_cutouts() -> Part {
    let mut cutouts = Part::empty("recirculation_dead_zone_tracer_bubble_window_cutouts");
    for i in 0..BUBBLE_WINDOWS {
        cutouts = cutouts
            + centered_cube(
                format!("recirculation_dead_zone_tracer_bubble_window_clear_{i}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                BUBBLE_Z + 8.0,
            )
            .translate(
                BUBBLE_POS.0 + centered_index(i, BUBBLE_WINDOWS, WITNESS_WINDOW_PITCH_X),
                BUBBLE_POS.1 + 44.0,
                place_z(BUBBLE_Z),
            );
    }
    cutouts
}

fn degas_window_cutouts() -> Part {
    let mut cutouts = Part::empty("recirculation_dead_zone_tracer_degas_window_cutouts");
    for i in 0..DEGAS_WITNESS_WINDOWS {
        cutouts = cutouts
            + centered_cube(
                format!("recirculation_dead_zone_tracer_degas_window_clear_{i}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                BUBBLE_Z + 8.0,
            )
            .translate(
                BUBBLE_POS.0 + centered_index(i, DEGAS_WITNESS_WINDOWS, WITNESS_WINDOW_PITCH_X),
                BUBBLE_POS.1 - 20.0,
                place_z(BUBBLE_Z),
            );
    }
    cutouts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty("recirculation_dead_zone_tracer_bubble_window_frames");
    for i in 0..BUBBLE_WINDOWS {
        frames = frames
            + centered_cube(
                format!("recirculation_dead_zone_tracer_bubble_window_frame_{i}"),
                WITNESS_WINDOW_X + 8.0,
                WITNESS_WINDOW_Y + 8.0,
                6.0,
            )
            .translate(
                BUBBLE_POS.0 + centered_index(i, BUBBLE_WINDOWS, WITNESS_WINDOW_PITCH_X),
                BUBBLE_POS.1 + 44.0,
                deck_top_z() + BUBBLE_Z + 3.0,
            );
    }
    frames
}

fn degas_window_frames() -> Part {
    let mut frames = Part::empty("recirculation_dead_zone_tracer_degas_window_frames");
    for i in 0..DEGAS_WITNESS_WINDOWS {
        frames = frames
            + centered_cube(
                format!("recirculation_dead_zone_tracer_degas_window_frame_{i}"),
                WITNESS_WINDOW_X + 8.0,
                WITNESS_WINDOW_Y + 8.0,
                6.0,
            )
            .translate(
                BUBBLE_POS.0 + centered_index(i, DEGAS_WITNESS_WINDOWS, WITNESS_WINDOW_PITCH_X),
                BUBBLE_POS.1 - 20.0,
                deck_top_z() + BUBBLE_Z + 3.0,
            );
    }
    frames
}

fn degas_vent_capture_cups() -> Part {
    let mut cups = Part::empty("recirculation_dead_zone_tracer_degas_vent_capture_cups");
    for i in 0..DEGAS_VENT_CAPTURE_CUPS {
        let x = BUBBLE_POS.0 + centered_index(i, DEGAS_VENT_CAPTURE_CUPS, WITNESS_WINDOW_PITCH_X);
        let y = BUBBLE_POS.1 - BUBBLE_Y / 2.0 + 28.0;
        let cup = centered_cylinder(
            format!("recirculation_dead_zone_tracer_degas_vent_capture_cup_{i}"),
            14.0,
            10.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + BUBBLE_Z + 5.0);
        let hollow = centered_cylinder(
            format!("recirculation_dead_zone_tracer_degas_vent_capture_hollow_{i}"),
            8.0,
            12.0,
            24,
        )
        .translate(x, y, deck_top_z() + BUBBLE_Z + 5.0);
        cups = cups + (cup - hollow);
    }
    cups
}

fn chip_feed_handoff_bulkhead() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_chip_feed_handoff_bulkhead_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    )
    .translate(HANDOFF_POS.0, HANDOFF_POS.1, place_z(HANDOFF_Z));
    body - handoff_connector_bores() + handoff_connector_collars() + chip_feed_group_map()
}

fn handoff_connector_bores() -> Part {
    let mut bores = Part::empty("recirculation_dead_zone_tracer_handoff_connector_bores");
    for i in 0..CHIP_FEED_CONNECTORS {
        let x = HANDOFF_POS.0 + centered_index(i, CHIP_FEED_CONNECTORS, HANDOFF_CONNECTOR_PITCH);
        bores = bores
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_chip_feed_connector_bore_{i}"),
                HANDOFF_CONNECTOR_D / 2.0,
                HANDOFF_Z + 12.0,
                24,
            )
            .translate(x, HANDOFF_POS.1 + 34.0, place_z(HANDOFF_Z));
    }
    bores
}

fn handoff_connector_collars() -> Part {
    let mut collars = Part::empty("recirculation_dead_zone_tracer_handoff_connector_collars");
    for i in 0..CHIP_FEED_CONNECTORS {
        let x = HANDOFF_POS.0 + centered_index(i, CHIP_FEED_CONNECTORS, HANDOFF_CONNECTOR_PITCH);
        let collar = centered_cylinder(
            format!("recirculation_dead_zone_tracer_chip_feed_connector_collar_{i}"),
            HANDOFF_CONNECTOR_D / 2.0 + 5.0,
            8.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, HANDOFF_POS.1 + 34.0, deck_top_z() + HANDOFF_Z + 4.0);
        let clearance = centered_cylinder(
            format!("recirculation_dead_zone_tracer_chip_feed_connector_clearance_{i}"),
            HANDOFF_CONNECTOR_D / 2.0,
            10.0,
            24,
        )
        .translate(x, HANDOFF_POS.1 + 34.0, deck_top_z() + HANDOFF_Z + 4.0);
        collars = collars + (collar - clearance);
    }
    collars
}

fn chip_feed_group_map() -> Part {
    let mut map = Part::empty("recirculation_dead_zone_tracer_chip_feed_group_map");
    for group in 0..CHIP_FEED_GROUPS {
        map = map
            + centered_cube(
                format!("recirculation_dead_zone_tracer_chip_feed_group_lane_{group}"),
                62.0,
                24.0,
                6.0,
            )
            .translate(
                HANDOFF_POS.0 + centered_index(group, CHIP_FEED_GROUPS, 76.0),
                HANDOFF_POS.1 - 42.0,
                deck_top_z() + HANDOFF_Z + 3.0,
            );
    }
    map
}

fn barcode_coa_custody_lands() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_barcode_coa_custody_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, place_z(CUSTODY_Z));
    body + barcode_lands() + coa_lands() + tamper_seal_pads() - custody_token_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("recirculation_dead_zone_tracer_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("recirculation_dead_zone_tracer_barcode_land_{i}"),
                58.0,
                18.0,
                4.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i % 4, 4, 70.0),
                CUSTODY_POS.1 + if i < 4 { 46.0 } else { 18.0 },
                deck_top_z() + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("recirculation_dead_zone_tracer_coa_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("recirculation_dead_zone_tracer_coa_land_{i}"),
                54.0,
                26.0,
                4.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i, COA_LANDS, 72.0),
                CUSTODY_POS.1 - 18.0,
                deck_top_z() + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn tamper_seal_pads() -> Part {
    let mut pads = Part::empty("recirculation_dead_zone_tracer_tamper_seal_pads");
    for i in 0..TAMPER_SEAL_PADS {
        pads = pads
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_tamper_seal_pad_{i}"),
                9.0,
                4.0,
                24,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i, TAMPER_SEAL_PADS, 42.0),
                CUSTODY_POS.1 - CUSTODY_Y / 2.0 + 24.0,
                deck_top_z() + CUSTODY_Z + 2.0,
            );
    }
    pads
}

fn custody_token_slots() -> Part {
    let mut slots = Part::empty("recirculation_dead_zone_tracer_custody_token_slots");
    for i in 0..CUSTODY_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("recirculation_dead_zone_tracer_custody_token_slot_{i}"),
                22.0,
                14.0,
                CUSTODY_Z + 6.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i % 4, 4, 48.0),
                CUSTODY_POS.1 + if i < 4 { -52.0 } else { -72.0 },
                place_z(CUSTODY_Z),
            );
    }
    slots
}

fn release_hold_reject_gate_bank() -> Part {
    let body = centered_cube(
        "recirculation_dead_zone_tracer_release_hold_reject_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, place_z(GATE_Z));
    body - disposition_token_slots() + disposition_gate_blocks() + disposition_card_lands()
}

fn disposition_token_slots() -> Part {
    let mut slots = Part::empty("recirculation_dead_zone_tracer_disposition_token_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        slots = slots
            + centered_cylinder(
                format!("recirculation_dead_zone_tracer_disposition_token_slot_{i}"),
                7.0,
                GATE_Z + 8.0,
                20,
            )
            .translate(
                GATE_POS.0 + centered_index(i % 4, 4, 42.0),
                GATE_POS.1 + centered_index(i / 4, DISPOSITION_STATES, 44.0),
                place_z(GATE_Z),
            );
    }
    slots
}

fn disposition_gate_blocks() -> Part {
    let mut gates = Part::empty("recirculation_dead_zone_tracer_disposition_gate_blocks");
    for (i, name) in DISPOSITION_NAMES.iter().enumerate() {
        gates = gates
            + centered_cube(
                format!("recirculation_dead_zone_tracer_{name}_gate_block"),
                76.0,
                32.0,
                16.0,
            )
            .translate(
                GATE_POS.0 - GATE_X / 2.0 + 62.0,
                GATE_POS.1 + centered_index(i, DISPOSITION_STATES, 44.0),
                deck_top_z() + GATE_Z + 8.0,
            );
    }
    gates
}

fn disposition_card_lands() -> Part {
    let mut lands = Part::empty("recirculation_dead_zone_tracer_disposition_card_lands");
    for (i, name) in DISPOSITION_NAMES.iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("recirculation_dead_zone_tracer_{name}_decision_card_land"),
                108.0,
                24.0,
                4.0,
            )
            .translate(
                GATE_POS.0 + 74.0,
                GATE_POS.1 + centered_index(i, DISPOSITION_STATES, 44.0),
                deck_top_z() + GATE_Z + 2.0,
            );
    }
    lands
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "recirculation_dead_zone_tracer_camera_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + 50.0,
        CAMERA_POS.1,
        deck_top_z() + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "recirculation_dead_zone_tracer_camera_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0 - 50.0,
        CAMERA_POS.1,
        deck_top_z() + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "recirculation_dead_zone_tracer_camera_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        30.0,
    )
    .translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        deck_top_z() + CAMERA_BRIDGE_Z - 15.0,
    );

    left_post + right_post + beam + camera_mounts() + evidence_fiducials()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("recirculation_dead_zone_tracer_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        let x = centered_index(i, CAMERA_MOUNTS, 196.0);
        let boss = centered_cylinder(
            format!("recirculation_dead_zone_tracer_camera_mount_boss_{i}"),
            16.0,
            16.0,
            CYLINDER_SEGMENTS,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CAMERA_POS.1 - 26.0, CAMERA_CLEARANCE_Z);
        let bore = centered_cylinder(
            format!("recirculation_dead_zone_tracer_camera_mount_bore_{i}"),
            5.0,
            20.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CAMERA_POS.1 - 26.0, CAMERA_CLEARANCE_Z);
        mounts = mounts + (boss - bore);
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("recirculation_dead_zone_tracer_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = centered_index(i % 6, 6, 160.0);
        let y = if i < 6 { -365.0 } else { 365.0 };
        let outer = centered_cylinder(
            format!("recirculation_dead_zone_tracer_evidence_fiducial_outer_{i}"),
            8.0,
            3.0,
            FIDUCIAL_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + 3.0);
        let inner = centered_cylinder(
            format!("recirculation_dead_zone_tracer_evidence_fiducial_inner_{i}"),
            3.0,
            5.0,
            20,
        )
        .translate(x, y, deck_top_z() + 3.0);
        fiducials = fiducials + (outer - inner);
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let robot = centered_cube(
        "recirculation_dead_zone_tracer_robot_front_keepout_gauge",
        1240.0,
        84.0,
        76.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - 42.0, 38.0);
    let left = centered_cube(
        "recirculation_dead_zone_tracer_left_service_keepout_gauge",
        84.0,
        660.0,
        92.0,
    )
    .translate(-STATION_X / 2.0 - 42.0, 20.0, 46.0);
    let right = centered_cube(
        "recirculation_dead_zone_tracer_right_service_keepout_gauge",
        84.0,
        660.0,
        92.0,
    )
    .translate(STATION_X / 2.0 + 42.0, 20.0, 46.0);
    let rear = centered_cube(
        "recirculation_dead_zone_tracer_rear_sample_service_keepout_gauge",
        1160.0,
        58.0,
        74.0,
    )
    .translate(0.0, STATION_Y / 2.0 + 29.0, 37.0);
    let top = centered_cube(
        "recirculation_dead_zone_tracer_top_loop_service_clearance_gauge",
        980.0,
        420.0,
        8.0,
    )
    .translate(-100.0, 65.0, TOP_LOOP_SERVICE_CLEARANCE);
    robot + left + right + rear + top
}

fn tracer_route_dead_zone_overlay() -> Part {
    let mut overlay = Part::empty("recirculation_dead_zone_tracer_route_overlay");
    for segment in route_segments() {
        overlay = overlay
            + centered_cube(
                format!(
                    "recirculation_dead_zone_tracer_route_segment_{}",
                    segment.name
                ),
                segment.x,
                segment.y,
                7.0,
            )
            .translate(segment.center.0, segment.center.1, deck_top_z() + 6.0);
    }
    overlay + route_flow_markers()
}

fn route_flow_markers() -> Part {
    let mut markers = Part::empty("recirculation_dead_zone_tracer_route_flow_markers");
    for i in 0..ROUTE_MARKERS {
        let segment = route_segments()[i % ROUTE_SEGMENTS];
        let x = segment.center.0 + centered_index(i % 2, 2, segment.x.min(42.0));
        let y = segment.center.1;
        markers = markers
            + centered_cube(
                format!("recirculation_dead_zone_tracer_route_flow_marker_{i}"),
                16.0,
                8.0,
                5.0,
            )
            .translate(x, y, deck_top_z() + 13.0);
    }
    markers
}

fn route_segments() -> [RouteSegment; ROUTE_SEGMENTS] {
    [
        route("loop_to_tracer", (-187.0, 270.0), 236.0, 8.0),
        route("tracer_to_sample", (305.0, 270.0), 216.0, 8.0),
        route("loop_to_coupon", (-465.0, 86.0), 8.0, 146.0),
        route("coupon_to_sensor", (-187.0, -32.0), 236.0, 8.0),
        route("sensor_to_bubble", (305.0, -32.0), 216.0, 8.0),
        route("sample_to_bubble", (520.0, 90.0), 8.0, 120.0),
        route("coupon_to_handoff", (-465.0, -205.0), 8.0, 142.0),
        route("sensor_to_custody", (5.0, -205.0), 8.0, 142.0),
        route("bubble_to_gate", (415.0, -205.0), 8.0, 142.0),
        route("handoff_to_custody", (-272.0, -330.0), 178.0, 8.0),
        route("custody_to_gate", (118.0, -330.0), 188.0, 8.0),
        route("gate_to_handoff_return", (-75.0, -392.0), 776.0, 8.0),
    ]
}

fn route(name: &'static str, center: (f64, f64), x: f64, y: f64) -> RouteSegment {
    RouteSegment { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX));
        assert!(path.ends_with(".stl"));
    }
    assert_eq!(RESEARCH_BASIS.len(), 6);
    assert!(RESEARCH_BASIS.iter().all(|basis| basis.is_ascii()));
    assert_eq!(LOOP_PORTS, LOOP_LANES * 2);
    assert!(TRACER_INJECTION_PORTS >= LOOP_LANES);
    assert_eq!(TIMED_SAMPLE_WELLS, SAMPLE_TIMEPOINTS * SAMPLE_REPLICATES);
    assert_eq!(SAMPLE_COLS * SAMPLE_ROWS, TIMED_SAMPLE_WELLS);
    assert_eq!(
        TOTAL_WITNESS_COUPONS,
        BAFFLE_WITNESS_COUPONS + DEAD_ZONE_WITNESS_COUPONS
    );
    assert_eq!(TEMPERATURE_POCKETS, SENSOR_POCKET_COLS);
    assert_eq!(PH_POCKETS, SENSOR_POCKET_COLS);
    assert_eq!(OSMOLALITY_POCKETS, SENSOR_POCKET_COLS);
    assert_eq!(
        TEMPERATURE_POCKETS + PH_POCKETS + OSMOLALITY_POCKETS,
        SENSOR_POCKET_ROWS * SENSOR_POCKET_COLS
    );
    assert_eq!(BUBBLE_WINDOWS, LOOP_LANES);
    assert_eq!(DEGAS_WITNESS_WINDOWS, LOOP_LANES);
    assert_eq!(DEGAS_VENT_CAPTURE_CUPS, LOOP_LANES);
    assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
    assert_eq!(GATE_TOKEN_SLOTS, DISPOSITION_STATES * 4);
    assert!(ROBOT_FRONT_CLEARANCE >= 420.0);
    assert!(LEFT_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_SERVICE_CLEARANCE >= 210.0);
    assert!(REAR_SAMPLE_SERVICE_CLEARANCE >= 240.0);
    assert!(TOP_LOOP_SERVICE_CLEARANCE > CAMERA_CLEARANCE_Z);
    assert!(CAMERA_CLEARANCE_Z > deck_top_z() + GATE_Z + 120.0);

    for feature in [
        "closed_recirculation_loop_surrogate",
        "tracer_injection_ports",
        "timed_sample_wells",
        "baffle_witness_coupons",
        "dead_zone_witness_coupons",
        "temperature_pockets",
        "ph_pockets",
        "osmolality_pockets",
        "bubble_witness_windows",
        "degas_witness_windows",
        "chip_feed_handoff_bulkhead",
        "barcode_custody_lands",
        "coa_custody_lands",
        "release_gate",
        "hold_gate",
        "reject_gate",
        "camera_evidence_bridge",
        "robot_keepouts",
        "service_keepouts",
        "tracer_route_overlay",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "architecture_cad_only",
        "not_a_sterile_barrier_claim",
        "not_a_pressure_rated_recirculation_loop",
        "no_media_recipe_or_process_limits",
        "no_clinical_release_thresholds",
        "no_sensor_metrology_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    for module in module_footprints() {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station envelope",
            module.name
        );
    }

    let modules = module_footprints();
    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            assert!(
                !modules[i].overlaps_with_clearance(modules[j], MODULE_CLEARANCE_MM),
                "{} overlaps {}",
                modules[i].name,
                modules[j].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_containment_deck.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_recirculation_loop_surrogate.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_tracer_injection_manifold.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_timed_sample_well_array.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_baffle_dead_zone_witness_coupon_bank.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_temperature_ph_osmolality_pocket_panel.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_bubble_degas_witness_window_panel.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_chip_feed_handoff_bulkhead.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_barcode_coa_custody_lands.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_release_hold_reject_gate_bank.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_camera_evidence_bridge.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_robot_service_keepout_gauges.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_tracer_route_dead_zone_overlay.stl",
                "output/closed_media_conditioning_recirculation_dead_zone_tracer_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "closed_recirculation_loop_surrogate",
            "loop_inlet_outlet_ports",
            "tracer_injection_ports",
            "tracer_standard_cups",
            "timed_sample_wells",
            "sample_time_token_rail",
            "baffle_witness_coupons",
            "dead_zone_witness_coupons",
            "temperature_pockets",
            "ph_pockets",
            "osmolality_pockets",
            "bubble_witness_windows",
            "degas_witness_windows",
            "chip_feed_handoff_bulkhead",
            "barcode_custody_lands",
            "coa_custody_lands",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "camera_evidence_bridge",
            "robot_keepouts",
            "service_keepouts",
            "tracer_route_overlay",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing feature {feature}"
            );
        }
    }

    #[test]
    fn counts_cover_loop_tracer_samples_sensors_windows_and_release() {
        assert_eq!(LOOP_LANES, 6);
        assert_eq!(LOOP_PORTS, 12);
        assert_eq!(TRACER_INJECTION_PORTS, 8);
        assert_eq!(TRACER_STANDARD_CUPS, 4);
        assert_eq!(TIMED_SAMPLE_WELLS, 24);
        assert_eq!(BAFFLE_WITNESS_COUPONS, 6);
        assert_eq!(DEAD_ZONE_WITNESS_COUPONS, 6);
        assert_eq!(TEMPERATURE_POCKETS, 4);
        assert_eq!(PH_POCKETS, 4);
        assert_eq!(OSMOLALITY_POCKETS, 4);
        assert_eq!(BUBBLE_WINDOWS, LOOP_LANES);
        assert_eq!(DEGAS_WITNESS_WINDOWS, LOOP_LANES);
        assert_eq!(CHIP_FEED_CONNECTORS, 8);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_LANDS, 4);
        assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
    }

    #[test]
    fn station_modules_fit_inside_bounds_without_major_overlap() {
        assert_design_constraints();
        for module in module_footprints() {
            assert!(module.fits_inside_station(), "{module:?} outside deck");
        }
        let modules = module_footprints();
        for (i, left) in modules.iter().enumerate() {
            for right in modules.iter().skip(i + 1) {
                assert!(
                    !left.overlaps_with_clearance(*right, MODULE_CLEARANCE_MM),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn research_basis_and_limitations_are_explicit() {
        for basis in [
            "residence_time_distribution_tracer_studies_reveal_dead_zones",
            "recirculation_loop_mixing_controls_media_homogeneity_before_chip_feed",
            "inline_ph_and_temperature_sensing_support_upstream_process_control",
            "osmolality_and_sample_timing_require_custody_before_release",
            "bubble_degas_visual_witnesses_reduce_air_ingress_before_perfusion",
            "release_hold_reject_gates_preserve_process_validation_evidence",
        ] {
            assert!(RESEARCH_BASIS.contains(&basis));
        }
        assert!(LIMITATIONS.contains(&"architecture_cad_only"));
        assert!(LIMITATIONS.contains(&"not_a_pressure_rated_recirculation_loop"));
        assert!(LIMITATIONS.contains(&"no_clinical_release_thresholds"));
        assert!(LIMITATIONS.contains(&"no_media_recipe_or_process_limits"));
    }

    #[test]
    fn workflow_routes_from_loop_to_tracer_dead_zone_release_and_chip_feed() {
        assert!(LOOP_POS.0 < TRACER_POS.0);
        assert!(TRACER_POS.0 < SAMPLE_POS.0);
        assert!(COUPON_POS.0 < SENSOR_POS.0);
        assert!(SENSOR_POS.0 < BUBBLE_POS.0);
        assert!(COUPON_POS.1 > HANDOFF_POS.1);
        assert!(HANDOFF_POS.0 < CUSTODY_POS.0);
        assert!(CUSTODY_POS.0 < GATE_POS.0);
        assert_eq!(route_segments().len(), ROUTE_SEGMENTS);
    }

    #[test]
    fn keepouts_and_evidence_clearances_are_physical() {
        assert!(ROBOT_FRONT_CLEARANCE >= 420.0);
        assert!(LEFT_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 210.0);
        assert!(REAR_SAMPLE_SERVICE_CLEARANCE >= 240.0);
        assert!(TOP_LOOP_SERVICE_CLEARANCE > CAMERA_CLEARANCE_Z);
        assert_eq!(CAMERA_MOUNTS, 6);
        assert_eq!(EVIDENCE_FIDUCIALS, 12);
        assert!(CAMERA_POS.1 < GATE_POS.1);
    }
}
