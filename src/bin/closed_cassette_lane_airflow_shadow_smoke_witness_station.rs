use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette lane airflow shadow / smoke witness station.
//
// This standalone generator models a no-cell validation station for checking
// whether cassette lane geometry creates airflow shadows, stagnant pockets, or
// cross-position differences inside a modular clean enclosure/incubator. It
// packages a lane-surrogate rack, smoke/aerosol injection coupon ports, witness
// strip holders, low-profile probe masts, edge/center position tokens, exhaust
// capture, camera evidence support, barcode/custody lands, and robot/service
// keepout gauges. Smoke chemistry, exposure time, probe calibration, acceptance
// thresholds, and study disposition remain protocol controls outside the CAD.

const OUTPUT_PREFIX: &str = "closed_cassette_lane_airflow_shadow_smoke_witness_station";

const OUTPUTS: [&str; 11] = [
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_base_containment_deck.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_cassette_lane_surrogate_rack.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_smoke_aerosol_injection_coupon_ports.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_airflow_shadow_witness_strip_holders.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_low_profile_probe_masts.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_edge_center_position_tokens.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_exhaust_capture_channel.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_camera_evidence_bridge.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_barcode_custody_lands.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_lane_airflow_shadow_smoke_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "cassette_lane_surrogate_rack",
    "smoke_aerosol_injection_coupon_ports",
    "airflow_shadow_witness_strip_holders",
    "low_profile_probe_masts",
    "edge_center_position_tokens",
    "exhaust_capture_channel",
    "camera_evidence_bridge",
    "barcode_custody_lands",
    "robot_service_keepout_gauges",
    "assembly",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 930.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const LANE_COUNT: usize = 4;
const POSITIONS_PER_LANE: usize = 3;
const POSITION_COUNT: usize = LANE_COUNT * POSITIONS_PER_LANE;
const EDGE_POSITION_COUNT: usize = 10;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;
const RACK_X: f64 = 660.0;
const RACK_Y: f64 = 360.0;
const RACK_Z: f64 = 38.0;
const RACK_POS: (f64, f64) = (-360.0, 100.0);
const LANE_PITCH_X: f64 = 140.0;
const POSITION_PITCH_Y: f64 = 96.0;
const CASSETTE_SURROGATE_X: f64 = REVC_CHIP_LENGTH + 44.0;
const CASSETTE_SURROGATE_Y: f64 = REVC_CHIP_WIDTH + 36.0;
const CASSETTE_SURROGATE_Z: f64 = REVC_TOTAL_HEIGHT + 18.0;
const LANE_RELIEF_DEPTH: f64 = 10.0;
const LANE_BYPASS_SLOT_COUNT: usize = 16;
const LANE_DIVIDER_COUNT: usize = LANE_COUNT + 1;

const SMOKE_X: f64 = 420.0;
const SMOKE_Y: f64 = 220.0;
const SMOKE_Z: f64 = 34.0;
const SMOKE_POS: (f64, f64) = (360.0, 190.0);
const SMOKE_PORT_COUNT: usize = POSITION_COUNT;
const SMOKE_PORT_D: f64 = 9.2;
const COUPON_SOCKET_D: f64 = 28.0;
const COUPON_GROUPS: usize = 3;
const COUPON_COLUMNS: usize = 4;
const INJECTION_MANIFOLD_COUNT: usize = 4;
const SMOKE_MIXING_VANE_COUNT: usize = 8;

const WITNESS_X: f64 = 660.0;
const WITNESS_Y: f64 = 190.0;
const WITNESS_Z: f64 = 28.0;
const WITNESS_POS: (f64, f64) = (-360.0, -235.0);
const WITNESS_STRIP_COUNT: usize = POSITION_COUNT;
const STRIP_X: f64 = 96.0;
const STRIP_Y: f64 = 14.0;
const STRIP_RECESS_DEPTH: f64 = 7.0;
const STRIP_CLIP_COUNT: usize = WITNESS_STRIP_COUNT * 2;
const SHADOW_SCALE_TICK_COUNT: usize = 16;

const PROBE_X: f64 = 420.0;
const PROBE_Y: f64 = 180.0;
const PROBE_Z: f64 = 30.0;
const PROBE_POS: (f64, f64) = (360.0, -55.0);
const PROBE_MAST_COUNT: usize = 8;
const PROBE_COLS: usize = 4;
const PROBE_ROWS: usize = 2;
const PROBE_SOCKET_D: f64 = 16.0;
const PROBE_MAST_D: f64 = 10.0;
const PROBE_LOW_Z: f64 = 42.0;
const PROBE_MID_Z: f64 = 66.0;
const PROBE_HIGH_Z: f64 = 88.0;
const PROBE_FLAG_COUNT: usize = PROBE_MAST_COUNT;
const CABLE_COMB_SLOTS: usize = PROBE_MAST_COUNT;

const TOKEN_X: f64 = 250.0;
const TOKEN_Y: f64 = 90.0;
const TOKEN_Z: f64 = 12.0;
const TOKEN_POS: (f64, f64) = (20.0, -385.0);
const POSITION_TOKEN_COUNT: usize = POSITION_COUNT;
const EDGE_TOKEN_D: f64 = 18.0;
const CENTER_TOKEN_D: f64 = 22.0;
const TOKEN_BARCODE_LANDS: usize = POSITION_COUNT;

const EXHAUST_X: f64 = 420.0;
const EXHAUST_Y: f64 = 160.0;
const EXHAUST_Z: f64 = 40.0;
const EXHAUST_POS: (f64, f64) = (410.0, -300.0);
const EXHAUST_SLOT_COUNT: usize = 7;
const EXHAUST_SLOT_X: f64 = 42.0;
const EXHAUST_SLOT_Y: f64 = 18.0;
const EXHAUST_TRAP_CUP_COUNT: usize = 3;
const EXHAUST_HEPA_COUPON_COUNT: usize = 2;

const BRIDGE_X: f64 = 1280.0;
const BRIDGE_Y: f64 = 70.0;
const BRIDGE_ANCHOR_Z: f64 = 16.0;
const BRIDGE_POST_Z: f64 = 188.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const BRIDGE_POS: (f64, f64) = (0.0, 390.0);
const CAMERA_POD_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 174.0;

const TRACE_X: f64 = 260.0;
const TRACE_Y: f64 = 94.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-270.0, -385.0);
const BARCODE_LAND_COUNT: usize = POSITION_COUNT + COUPON_GROUPS;
const CUSTODY_SEAL_LAND_COUNT: usize = 6;
const WITNESS_CARD_LANDS: usize = 4;

const KEEP_OUT_X: f64 = 1460.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_Z: f64 = 8.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;
const ROBOT_FRONT_CLEARANCE: f64 = 360.0;
const SERVICE_REAR_CLEARANCE: f64 = 36.0;
const SIDE_SERVICE_CLEARANCE: f64 = 120.0;
const ROBOT_Z_CLEARANCE: f64 = 250.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 14.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 14.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PositionZone {
    Edge,
    Center,
}

impl PositionZone {
    fn name(self) -> &'static str {
        match self {
            PositionZone::Edge => "edge",
            PositionZone::Center => "center",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let rack = cassette_lane_surrogate_rack();
    export(OUTPUTS[1], &rack);

    let smoke = smoke_aerosol_injection_coupon_ports();
    export(OUTPUTS[2], &smoke);

    let witness = airflow_shadow_witness_strip_holders();
    export(OUTPUTS[3], &witness);

    let probes = low_profile_probe_masts();
    export(OUTPUTS[4], &probes);

    let tokens = edge_center_position_tokens();
    export(OUTPUTS[5], &tokens);

    let exhaust = exhaust_capture_channel();
    export(OUTPUTS[6], &exhaust);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[7], &bridge);

    let trace = barcode_custody_lands();
    export(OUTPUTS[8], &trace);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[9], &keepouts);

    let assembly = deck
        + rack.translate(RACK_POS.0, RACK_POS.1, on_deck_z(RACK_Z))
        + smoke.translate(SMOKE_POS.0, SMOKE_POS.1, on_deck_z(SMOKE_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + probes.translate(PROBE_POS.0, PROBE_POS.1, on_deck_z(PROBE_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + exhaust.translate(EXHAUST_POS.0, EXHAUST_POS.1, on_deck_z(EXHAUST_Z))
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, on_deck_z(BRIDGE_ANCHOR_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cassette lane airflow shadow smoke witness station:");
    println!(
        "  Lane rack:             {LANE_COUNT} cassette lanes x {POSITIONS_PER_LANE} positions, {EDGE_POSITION_COUNT} edge and {CENTER_POSITION_COUNT} center comparators"
    );
    println!(
        "  Smoke challenge:       {SMOKE_PORT_COUNT} indexed smoke/aerosol ports, {COUPON_GROUPS} coupon groups, {INJECTION_MANIFOLD_COUNT} manifold lanes, {SMOKE_MIXING_VANE_COUNT} mixing vanes"
    );
    println!(
        "  Witness capture:       {WITNESS_STRIP_COUNT} strip holders, {STRIP_CLIP_COUNT} retaining clips, {SHADOW_SCALE_TICK_COUNT} shadow-scale ticks"
    );
    println!(
        "  Instrumentation:       {PROBE_MAST_COUNT} low-profile probe masts, {PROBE_FLAG_COUNT} probe flags, {:.0}/{:.0}/{:.0}mm height references",
        PROBE_LOW_Z, PROBE_MID_Z, PROBE_HIGH_Z
    );
    println!(
        "  Capture/evidence:      {EXHAUST_SLOT_COUNT} exhaust slots, {EXHAUST_TRAP_CUP_COUNT} trap cups, {CAMERA_POD_COUNT} camera pods, {LIGHT_BAR_COUNT} light bars"
    );
    println!(
        "  Traceability/access:   {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_SEAL_LAND_COUNT} custody seal lands, {KEEP_OUT_GAUGE_COUNT} robot/service keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    DECK_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(POSITION_COUNT, LANE_COUNT * POSITIONS_PER_LANE);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(SMOKE_PORT_COUNT, POSITION_COUNT);
    assert_eq!(WITNESS_STRIP_COUNT, POSITION_COUNT);
    assert_eq!(STRIP_CLIP_COUNT, WITNESS_STRIP_COUNT * 2);
    assert_eq!(PROBE_MAST_COUNT, PROBE_COLS * PROBE_ROWS);
    assert_eq!(POSITION_TOKEN_COUNT, POSITION_COUNT);
    assert_eq!(TOKEN_BARCODE_LANDS, POSITION_COUNT);
    assert!(CASSETTE_SURROGATE_X > REVC_CHIP_LENGTH);
    assert!(CASSETTE_SURROGATE_Y > REVC_CHIP_WIDTH);
    assert!(CASSETTE_SURROGATE_Z > REVC_TOTAL_HEIGHT);
    assert!(LANE_RELIEF_DEPTH < RACK_Z - 16.0);
    assert!(PROBE_HIGH_Z > PROBE_MID_Z && PROBE_MID_Z > PROBE_LOW_Z);
    assert!(CAMERA_CLEARANCE_Z > PROBE_HIGH_Z + 70.0);
    assert!(ROBOT_Z_CLEARANCE > BRIDGE_POST_Z + BRIDGE_BEAM_Z + 30.0);
    assert!(exhaust_capture_open_area_mm2() > smoke_port_open_area_mm2() * 4.0);
    assert!(deck_basin_freeboard_ml() > 650.0);
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
    assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);

    let rects = module_rects();
    for item in rects {
        assert!(item.fits_inside_deck(), "{} exceeds deck rim", item.name);
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
}

fn module_rects() -> [Rect; 8] {
    [
        rect("cassette_lane_surrogate_rack", RACK_POS, RACK_X, RACK_Y),
        rect(
            "smoke_aerosol_injection_coupon_ports",
            SMOKE_POS,
            SMOKE_X,
            SMOKE_Y,
        ),
        rect(
            "airflow_shadow_witness_strip_holders",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect("low_profile_probe_masts", PROBE_POS, PROBE_X, PROBE_Y),
        rect("edge_center_position_tokens", TOKEN_POS, TOKEN_X, TOKEN_Y),
        rect("exhaust_capture_channel", EXHAUST_POS, EXHAUST_X, EXHAUST_Y),
        rect("camera_evidence_bridge", BRIDGE_POS, BRIDGE_X, BRIDGE_Y),
        rect("barcode_custody_lands", TRACE_POS, TRACE_X, TRACE_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "airflow_shadow_smoke_witness_base_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        "airflow_shadow_smoke_witness_wipeable_basin_cut",
        STATION_X - 150.0,
        STATION_Y - 132.0,
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - BASIN_DEPTH / 2.0 + 0.3);
    let front_gutter = centered_cube(
        "airflow_shadow_smoke_witness_front_smoke_condensate_gutter_cut",
        STATION_X - 230.0,
        18.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 56.0, DECK_Z / 2.0 - 4.0);
    let right_sump = centered_cylinder(
        "airflow_shadow_smoke_witness_closed_exhaust_drain_placeholder",
        8.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 + 38.0, 0.0);

    deck - basin - front_gutter - right_sump - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + zone_spines()
        + datum_targets()
        + deck_flow_axis_ticks()
        + bridge_anchor_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("airflow_shadow_smoke_witness_insert_sockets");
    for item in module_rects().into_iter().take(7) {
        sockets = sockets
            + centered_cube(
                format!("airflow_shadow_smoke_witness_{}_socket", item.name),
                item.x + 8.0,
                item.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                item.center.0,
                item.center.1,
                DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airflow_shadow_smoke_witness_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let bore = centered_cylinder(
            format!("airflow_shadow_smoke_witness_m6_mount_bore_{i}"),
            3.4,
            DECK_Z + 4.0,
            28,
        )
        .translate(x, y, 0.0);
        let service_slot = centered_cube(
            format!("airflow_shadow_smoke_witness_m6_service_slot_{i}"),
            30.0,
            7.0,
            DECK_Z + 4.0,
        )
        .translate(x, y, 0.0);
        slots = slots + bore + service_slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 60.0, 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "airflow_shadow_smoke_witness_front_robot_low_lip",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "airflow_shadow_smoke_witness_rear_service_high_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "airflow_shadow_smoke_witness_left_datum_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "airflow_shadow_smoke_witness_right_exhaust_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn zone_spines() -> Part {
    let lane_to_smoke_spine = centered_cube(
        "airflow_shadow_smoke_witness_lane_to_smoke_zone_spine",
        10.0,
        650.0,
        22.0,
    )
    .translate(10.0, 25.0, DECK_Z / 2.0 + 11.0);
    let witness_trace_spine = centered_cube(
        "airflow_shadow_smoke_witness_witness_traceability_zone_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -342.0, DECK_Z / 2.0 + 11.0);
    let exhaust_spine = centered_cube(
        "airflow_shadow_smoke_witness_exhaust_capture_zone_spine",
        10.0,
        260.0,
        22.0,
    )
    .translate(190.0, -300.0, DECK_Z / 2.0 + 11.0);

    lane_to_smoke_spine + witness_trace_spine + exhaust_spine
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("airflow_shadow_smoke_witness_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets =
            targets
                + fiducial_disc(&format!("airflow_shadow_smoke_witness_datum_target_{i}"))
                    .translate(x, y, DECK_Z / 2.0 + 2.0);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 96.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 92.0),
        (-STATION_X / 2.0 + 96.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 96.0, STATION_Y / 2.0 - 92.0),
    ]
}

fn deck_flow_axis_ticks() -> Part {
    let mut ticks = Part::empty("airflow_shadow_smoke_witness_deck_flow_axis_ticks");
    for i in 0..LANE_BYPASS_SLOT_COUNT {
        ticks = ticks
            + centered_cube(
                format!("airflow_shadow_smoke_witness_global_flow_tick_{i}"),
                8.0,
                38.0,
                4.0,
            )
            .translate(
                centered_index(i, LANE_BYPASS_SLOT_COUNT, 76.0),
                -118.0,
                DECK_Z / 2.0 + 2.0,
            );
    }
    ticks
}

fn bridge_anchor_lands() -> Part {
    let left = centered_cube(
        "airflow_shadow_smoke_witness_bridge_left_anchor_land",
        104.0,
        42.0,
        8.0,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0 + 62.0,
        BRIDGE_POS.1,
        DECK_Z / 2.0 + 4.0,
    );
    let right = centered_cube(
        "airflow_shadow_smoke_witness_bridge_right_anchor_land",
        104.0,
        42.0,
        8.0,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0 - 62.0,
        BRIDGE_POS.1,
        DECK_Z / 2.0 + 4.0,
    );
    left + right
}

fn cassette_lane_surrogate_rack() -> Part {
    let plate = centered_cube(
        "airflow_shadow_smoke_witness_cassette_lane_surrogate_rack_plate",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let gasket = rectangular_frame_xy(
        "airflow_shadow_smoke_witness_lane_rack_gasket_land",
        RACK_X - 38.0,
        RACK_Y - 34.0,
        6.0,
        RACK_X - 78.0,
        RACK_Y - 74.0,
    )
    .translate(0.0, 0.0, RACK_Z / 2.0 + 3.0);

    plate - lane_position_reliefs() - lane_bypass_windows()
        + lane_divider_rails()
        + lane_surrogate_cassettes()
        + lane_air_shadow_screens()
        + lane_datum_pins()
        + gasket
}

fn lane_position_reliefs() -> Part {
    let mut reliefs = Part::empty("airflow_shadow_smoke_witness_lane_position_reliefs");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let (x, y) = lane_position_center(lane, position);
            reliefs = reliefs
                + centered_cube(
                    format!("airflow_shadow_smoke_witness_lane_{lane}_position_{position}_relief_{index}"),
                    CASSETTE_SURROGATE_X + 12.0,
                    CASSETTE_SURROGATE_Y + 12.0,
                    LANE_RELIEF_DEPTH + 0.6,
                )
                .translate(x, y, RACK_Z / 2.0 - LANE_RELIEF_DEPTH / 2.0 + 0.3);
        }
    }
    reliefs
}

fn lane_bypass_windows() -> Part {
    let mut windows = Part::empty("airflow_shadow_smoke_witness_lane_bypass_windows");
    for i in 0..LANE_BYPASS_SLOT_COUNT {
        let lane = i % LANE_COUNT;
        let row = i / LANE_COUNT;
        windows = windows
            + centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_bypass_window_{i}"),
                82.0,
                8.0,
                RACK_Z + 2.0,
            )
            .translate(
                centered_index(lane, LANE_COUNT, LANE_PITCH_X),
                centered_index(row, 4, 72.0),
                0.0,
            );
    }
    windows
}

fn lane_divider_rails() -> Part {
    let mut rails = Part::empty("airflow_shadow_smoke_witness_lane_divider_rails");
    for i in 0..LANE_DIVIDER_COUNT {
        let x = -((LANE_COUNT as f64 - 1.0) * LANE_PITCH_X) / 2.0 - LANE_PITCH_X / 2.0
            + i as f64 * LANE_PITCH_X;
        rails = rails
            + centered_cube(
                format!("airflow_shadow_smoke_witness_lane_divider_rail_{i}"),
                9.0,
                RACK_Y - 70.0,
                28.0,
            )
            .translate(x, 0.0, RACK_Z / 2.0 + 14.0);
    }
    rails
}

fn lane_surrogate_cassettes() -> Part {
    let mut cassettes = Part::empty("airflow_shadow_smoke_witness_lane_surrogate_cassettes");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let zone = position_zone(lane, position).name();
            let (x, y) = lane_position_center(lane, position);
            let body = centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_position_{position}_{zone}_cassette_surrogate_{index}"),
                CASSETTE_SURROGATE_X - 18.0,
                CASSETTE_SURROGATE_Y - 16.0,
                CASSETTE_SURROGATE_Z,
            );
            let smoke_slot = centered_cube(
                format!(
                    "airflow_shadow_smoke_witness_lane_{lane}_position_{position}_flow_view_slot"
                ),
                CASSETTE_SURROGATE_X - 58.0,
                11.0,
                CASSETTE_SURROGATE_Z + 1.0,
            );
            let leading_edge_notch = centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_position_{position}_leading_edge_notch"),
                22.0,
                18.0,
                CASSETTE_SURROGATE_Z + 1.0,
            )
            .translate(-CASSETTE_SURROGATE_X / 2.0 + 20.0, 0.0, 0.0);
            cassettes = cassettes
                + (body - smoke_slot - leading_edge_notch).translate(
                    x,
                    y,
                    RACK_Z / 2.0 + CASSETTE_SURROGATE_Z / 2.0,
                );
        }
    }
    cassettes
}

fn lane_air_shadow_screens() -> Part {
    let mut screens = Part::empty("airflow_shadow_smoke_witness_lane_air_shadow_screens");
    for lane in 0..LANE_COUNT {
        let x = centered_index(lane, LANE_COUNT, LANE_PITCH_X);
        screens = screens
            + centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_upstream_shadow_screen"),
                CASSETTE_SURROGATE_X + 22.0,
                5.0,
                44.0,
            )
            .translate(
                x,
                -((POSITIONS_PER_LANE as f64 - 1.0) * POSITION_PITCH_Y) / 2.0 - 46.0,
                RACK_Z / 2.0 + 22.0,
            );
        screens = screens
            + centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_downstream_wake_screen"),
                CASSETTE_SURROGATE_X + 22.0,
                5.0,
                36.0,
            )
            .translate(
                x,
                ((POSITIONS_PER_LANE as f64 - 1.0) * POSITION_PITCH_Y) / 2.0 + 46.0,
                RACK_Z / 2.0 + 18.0,
            );
    }
    screens
}

fn lane_datum_pins() -> Part {
    let mut pins = Part::empty("airflow_shadow_smoke_witness_lane_datum_pins");
    for (i, (x, y)) in [
        (-RACK_X / 2.0 + 34.0, -RACK_Y / 2.0 + 34.0),
        (RACK_X / 2.0 - 34.0, -RACK_Y / 2.0 + 34.0),
        (-RACK_X / 2.0 + 34.0, RACK_Y / 2.0 - 34.0),
        (RACK_X / 2.0 - 34.0, RACK_Y / 2.0 - 34.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins =
            pins + datum_boss(&format!("airflow_shadow_smoke_witness_lane_rack_datum_{i}"))
                .translate(x, y, RACK_Z / 2.0 + 4.0);
    }
    pins
}

fn smoke_aerosol_injection_coupon_ports() -> Part {
    let plate = centered_cube(
        "airflow_shadow_smoke_witness_smoke_aerosol_coupon_port_plate",
        SMOKE_X,
        SMOKE_Y,
        SMOKE_Z,
    );
    let plenum_frame = rectangular_frame_xy(
        "airflow_shadow_smoke_witness_smoke_plenum_gasket_frame",
        SMOKE_X - 34.0,
        SMOKE_Y - 34.0,
        6.0,
        SMOKE_X - 74.0,
        SMOKE_Y - 74.0,
    )
    .translate(0.0, 0.0, SMOKE_Z / 2.0 + 3.0);

    plate - smoke_coupon_socket_cuts() - injection_port_bores()
        + injection_port_rings()
        + aerosol_coupon_tabs()
        + injection_manifold_lanes()
        + smoke_mixing_vanes()
        + plenum_frame
}

fn smoke_coupon_socket_cuts() -> Part {
    let mut cuts = Part::empty("airflow_shadow_smoke_witness_coupon_socket_cuts");
    for group in 0..COUPON_GROUPS {
        for col in 0..COUPON_COLUMNS {
            let index = group * COUPON_COLUMNS + col;
            let (x, y) = coupon_center(group, col);
            cuts = cuts
                + centered_cylinder(
                    format!("airflow_shadow_smoke_witness_coupon_socket_cut_{index}"),
                    COUPON_SOCKET_D / 2.0,
                    SMOKE_Z + 3.0,
                    36,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn injection_port_bores() -> Part {
    let mut bores = Part::empty("airflow_shadow_smoke_witness_injection_port_bores");
    for group in 0..COUPON_GROUPS {
        for col in 0..COUPON_COLUMNS {
            let index = group * COUPON_COLUMNS + col;
            let (x, y) = coupon_center(group, col);
            bores = bores
                + centered_cylinder(
                    format!("airflow_shadow_smoke_witness_indexed_smoke_port_bore_{index}"),
                    SMOKE_PORT_D / 2.0,
                    SMOKE_Z + 8.0,
                    28,
                )
                .translate(x, y, 0.0);
        }
    }
    bores
}

fn injection_port_rings() -> Part {
    let mut rings = Part::empty("airflow_shadow_smoke_witness_injection_port_rings");
    for group in 0..COUPON_GROUPS {
        for col in 0..COUPON_COLUMNS {
            let index = group * COUPON_COLUMNS + col;
            let zone = position_zone(index % LANE_COUNT, index / LANE_COUNT).name();
            let (x, y) = coupon_center(group, col);
            let outer = centered_cylinder(
                format!("airflow_shadow_smoke_witness_{zone}_coupon_port_outer_ring_{index}"),
                COUPON_SOCKET_D / 2.0 + 5.0,
                6.0,
                36,
            );
            let inner = centered_cylinder(
                format!("airflow_shadow_smoke_witness_coupon_port_inner_relief_{index}"),
                COUPON_SOCKET_D / 2.0 - 5.0,
                7.0,
                36,
            );
            rings = rings + (outer - inner).translate(x, y, SMOKE_Z / 2.0 + 3.0);
        }
    }
    rings
}

fn aerosol_coupon_tabs() -> Part {
    let mut tabs = Part::empty("airflow_shadow_smoke_witness_aerosol_coupon_pull_tabs");
    for group in 0..COUPON_GROUPS {
        for col in 0..COUPON_COLUMNS {
            let index = group * COUPON_COLUMNS + col;
            let (x, y) = coupon_center(group, col);
            tabs = tabs
                + centered_cube(
                    format!("airflow_shadow_smoke_witness_coupon_{index}_pull_tab"),
                    22.0,
                    9.0,
                    7.0,
                )
                .translate(x, y - 26.0, SMOKE_Z / 2.0 + 3.5);
        }
    }
    tabs
}

fn injection_manifold_lanes() -> Part {
    let mut lanes = Part::empty("airflow_shadow_smoke_witness_injection_manifold_lanes");
    for i in 0..INJECTION_MANIFOLD_COUNT {
        let x = centered_index(i, INJECTION_MANIFOLD_COUNT, 86.0);
        let rail = centered_cube(
            format!("airflow_shadow_smoke_witness_smoke_manifold_lane_{i}"),
            12.0,
            SMOKE_Y - 74.0,
            18.0,
        )
        .translate(x, 0.0, SMOKE_Z / 2.0 + 9.0);
        let hose_boss = centered_cylinder(
            format!("airflow_shadow_smoke_witness_smoke_manifold_hose_boss_{i}"),
            11.0,
            14.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -SMOKE_Y / 2.0 + 26.0, SMOKE_Z / 2.0 + 16.0);
        lanes = lanes + rail + hose_boss;
    }
    lanes
}

fn smoke_mixing_vanes() -> Part {
    let mut vanes = Part::empty("airflow_shadow_smoke_witness_smoke_mixing_vanes");
    for i in 0..SMOKE_MIXING_VANE_COUNT {
        vanes = vanes
            + centered_cube(
                format!("airflow_shadow_smoke_witness_mixing_vane_{i}"),
                42.0,
                5.0,
                22.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
            .translate(
                centered_index(i, SMOKE_MIXING_VANE_COUNT, 42.0),
                70.0,
                SMOKE_Z / 2.0 + 11.0,
            );
    }
    vanes
}

fn airflow_shadow_witness_strip_holders() -> Part {
    let plate = centered_cube(
        "airflow_shadow_smoke_witness_strip_holder_plate",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );

    plate - witness_strip_recesses()
        + witness_strip_retainer_clips()
        + witness_strip_carriers()
        + shadow_scale_ticks()
        + witness_lane_headers()
}

fn witness_strip_recesses() -> Part {
    let mut recesses = Part::empty("airflow_shadow_smoke_witness_strip_recesses");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let (x, y) = strip_center(lane, position);
            recesses = recesses
                + centered_cube(
                    format!("airflow_shadow_smoke_witness_strip_recess_{index}"),
                    STRIP_X + 12.0,
                    STRIP_Y + 10.0,
                    STRIP_RECESS_DEPTH + 0.6,
                )
                .translate(x, y, WITNESS_Z / 2.0 - STRIP_RECESS_DEPTH / 2.0 + 0.3);
        }
    }
    recesses
}

fn witness_strip_retainer_clips() -> Part {
    let mut clips = Part::empty("airflow_shadow_smoke_witness_strip_retainer_clips");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let (x, y) = strip_center(lane, position);
            for side in [-1.0, 1.0] {
                clips = clips
                    + centered_cube(
                        format!(
                            "airflow_shadow_smoke_witness_strip_{index}_retainer_clip_{}",
                            if side < 0.0 { "left" } else { "right" }
                        ),
                        8.0,
                        STRIP_Y + 18.0,
                        13.0,
                    )
                    .translate(
                        x + side * (STRIP_X / 2.0 + 8.0),
                        y,
                        WITNESS_Z / 2.0 + 6.5,
                    );
            }
        }
    }
    clips
}

fn witness_strip_carriers() -> Part {
    let mut strips = Part::empty("airflow_shadow_smoke_witness_removable_strip_carriers");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let zone = position_zone(lane, position).name();
            let (x, y) = strip_center(lane, position);
            let strip = centered_cube(
                format!("airflow_shadow_smoke_witness_{zone}_strip_carrier_{index}"),
                STRIP_X,
                STRIP_Y,
                4.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 2.0);
            let witness_dot = centered_cylinder(
                format!("airflow_shadow_smoke_witness_strip_{index}_witness_dot"),
                4.5,
                4.6,
                24,
            )
            .translate(x - STRIP_X / 2.0 + 14.0, y, WITNESS_Z / 2.0 + 2.3);
            strips = strips + strip + witness_dot;
        }
    }
    strips
}

fn shadow_scale_ticks() -> Part {
    let mut ticks = Part::empty("airflow_shadow_smoke_witness_shadow_scale_ticks");
    for i in 0..SHADOW_SCALE_TICK_COUNT {
        ticks = ticks
            + centered_cube(
                format!("airflow_shadow_smoke_witness_shadow_scale_tick_{i}"),
                4.0,
                36.0,
                4.0,
            )
            .translate(
                centered_index(i, SHADOW_SCALE_TICK_COUNT, 36.0),
                -WITNESS_Y / 2.0 + 24.0,
                WITNESS_Z / 2.0 + 2.0,
            );
    }
    ticks
}

fn witness_lane_headers() -> Part {
    let mut headers = Part::empty("airflow_shadow_smoke_witness_lane_headers");
    for lane in 0..LANE_COUNT {
        let x = centered_index(lane, LANE_COUNT, LANE_PITCH_X);
        headers = headers
            + centered_cube(
                format!("airflow_shadow_smoke_witness_lane_{lane}_witness_header"),
                STRIP_X + 34.0,
                10.0,
                11.0,
            )
            .translate(x, WITNESS_Y / 2.0 - 24.0, WITNESS_Z / 2.0 + 5.5);
    }
    headers
}

fn low_profile_probe_masts() -> Part {
    let plate = centered_cube(
        "airflow_shadow_smoke_witness_low_profile_probe_mast_plate",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    );

    plate - probe_socket_cuts()
        + probe_mast_bosses()
        + probe_mast_posts()
        + probe_height_flags()
        + probe_cable_comb()
}

fn probe_socket_cuts() -> Part {
    let mut cuts = Part::empty("airflow_shadow_smoke_witness_probe_socket_cuts");
    for row in 0..PROBE_ROWS {
        for col in 0..PROBE_COLS {
            let index = row * PROBE_COLS + col;
            let (x, y) = probe_center(col, row);
            cuts = cuts
                + centered_cylinder(
                    format!("airflow_shadow_smoke_witness_probe_socket_cut_{index}"),
                    PROBE_SOCKET_D / 2.0,
                    PROBE_Z + 2.0,
                    28,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn probe_mast_bosses() -> Part {
    let mut bosses = Part::empty("airflow_shadow_smoke_witness_probe_mast_bosses");
    for row in 0..PROBE_ROWS {
        for col in 0..PROBE_COLS {
            let index = row * PROBE_COLS + col;
            let (x, y) = probe_center(col, row);
            let outer = centered_cylinder(
                format!("airflow_shadow_smoke_witness_probe_mast_boss_{index}"),
                17.0,
                7.0,
                32,
            );
            let inner = centered_cylinder(
                format!("airflow_shadow_smoke_witness_probe_mast_pilot_{index}"),
                PROBE_SOCKET_D / 2.0 - 2.5,
                8.0,
                28,
            );
            bosses = bosses + (outer - inner).translate(x, y, PROBE_Z / 2.0 + 3.5);
        }
    }
    bosses
}

fn probe_mast_posts() -> Part {
    let mut posts = Part::empty("airflow_shadow_smoke_witness_probe_mast_posts");
    for row in 0..PROBE_ROWS {
        for col in 0..PROBE_COLS {
            let index = row * PROBE_COLS + col;
            let (x, y) = probe_center(col, row);
            let height = probe_height_for_index(index);
            posts = posts
                + centered_cylinder(
                    format!("airflow_shadow_smoke_witness_low_profile_probe_mast_{index}"),
                    PROBE_MAST_D / 2.0,
                    height,
                    28,
                )
                .translate(x, y, PROBE_Z / 2.0 + height / 2.0);
        }
    }
    posts
}

fn probe_height_flags() -> Part {
    let mut flags = Part::empty("airflow_shadow_smoke_witness_probe_height_flags");
    for row in 0..PROBE_ROWS {
        for col in 0..PROBE_COLS {
            let index = row * PROBE_COLS + col;
            let (x, y) = probe_center(col, row);
            let height = probe_height_for_index(index);
            flags = flags
                + centered_cube(
                    format!("airflow_shadow_smoke_witness_probe_mast_{index}_height_flag"),
                    24.0,
                    4.0,
                    10.0,
                )
                .translate(x + 18.0, y, PROBE_Z / 2.0 + height - 8.0);
        }
    }
    flags
}

fn probe_cable_comb() -> Part {
    let rail = centered_cube(
        "airflow_shadow_smoke_witness_probe_cable_comb_rail",
        PROBE_X - 70.0,
        10.0,
        20.0,
    )
    .translate(0.0, -PROBE_Y / 2.0 + 22.0, PROBE_Z / 2.0 + 10.0);
    let mut slots = Part::empty("airflow_shadow_smoke_witness_probe_cable_comb_slots");
    for i in 0..CABLE_COMB_SLOTS {
        slots = slots
            + centered_cube(
                format!("airflow_shadow_smoke_witness_probe_cable_comb_slot_{i}"),
                10.0,
                12.0,
                22.0,
            )
            .translate(
                centered_index(i, CABLE_COMB_SLOTS, 38.0),
                -PROBE_Y / 2.0 + 22.0,
                PROBE_Z / 2.0 + 10.0,
            );
    }
    rail - slots
}

fn edge_center_position_tokens() -> Part {
    let board = centered_cube(
        "airflow_shadow_smoke_witness_edge_center_position_token_board",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );

    board + position_tokens() + token_barcode_lands() + edge_center_summary_rails()
}

fn position_tokens() -> Part {
    let mut tokens = Part::empty("airflow_shadow_smoke_witness_position_tokens");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let zone = position_zone(lane, position);
            let (x, y) = token_center(lane, position);
            let radius = match zone {
                PositionZone::Edge => EDGE_TOKEN_D / 2.0,
                PositionZone::Center => CENTER_TOKEN_D / 2.0,
            };
            tokens = tokens
                + centered_cylinder(
                    format!(
                        "airflow_shadow_smoke_witness_{}_position_token_{index}",
                        zone.name()
                    ),
                    radius,
                    5.0,
                    30,
                )
                .translate(x, y, TOKEN_Z / 2.0 + 2.5);
        }
    }
    tokens
}

fn token_barcode_lands() -> Part {
    let mut lands = Part::empty("airflow_shadow_smoke_witness_position_token_barcode_lands");
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            let index = lane_position_index(lane, position);
            let (x, y) = token_center(lane, position);
            lands = lands
                + centered_cube(
                    format!("airflow_shadow_smoke_witness_position_token_barcode_land_{index}"),
                    26.0,
                    8.0,
                    3.0,
                )
                .translate(x, y - 18.0, TOKEN_Z / 2.0 + 1.5);
        }
    }
    lands
}

fn edge_center_summary_rails() -> Part {
    let edge = centered_cube(
        "airflow_shadow_smoke_witness_edge_token_summary_rail",
        TOKEN_X - 36.0,
        6.0,
        8.0,
    )
    .translate(0.0, TOKEN_Y / 2.0 - 14.0, TOKEN_Z / 2.0 + 4.0);
    let center = centered_cube(
        "airflow_shadow_smoke_witness_center_token_summary_rail",
        TOKEN_X - 36.0,
        6.0,
        8.0,
    )
    .translate(0.0, -TOKEN_Y / 2.0 + 14.0, TOKEN_Z / 2.0 + 4.0);

    edge + center
}

fn exhaust_capture_channel() -> Part {
    let body = centered_cube(
        "airflow_shadow_smoke_witness_exhaust_capture_channel_body",
        EXHAUST_X,
        EXHAUST_Y,
        EXHAUST_Z,
    );
    let trough = centered_cube(
        "airflow_shadow_smoke_witness_exhaust_capture_trough_cut",
        EXHAUST_X - 62.0,
        EXHAUST_Y - 64.0,
        18.0,
    )
    .translate(0.0, 6.0, EXHAUST_Z / 2.0 - 9.0);

    body - trough - exhaust_grille_slot_cuts()
        + exhaust_grille_lips()
        + exhaust_trap_cups()
        + hepa_coupon_lands()
        + exhaust_tube_boss()
}

fn exhaust_grille_slot_cuts() -> Part {
    let mut cuts = Part::empty("airflow_shadow_smoke_witness_exhaust_grille_slot_cuts");
    for i in 0..EXHAUST_SLOT_COUNT {
        cuts = cuts
            + centered_cube(
                format!("airflow_shadow_smoke_witness_exhaust_grille_slot_cut_{i}"),
                EXHAUST_SLOT_X,
                EXHAUST_SLOT_Y,
                EXHAUST_Z + 3.0,
            )
            .translate(centered_index(i, EXHAUST_SLOT_COUNT, 48.0), 6.0, 0.0);
    }
    cuts
}

fn exhaust_grille_lips() -> Part {
    let mut lips = Part::empty("airflow_shadow_smoke_witness_exhaust_grille_lips");
    for i in 0..EXHAUST_SLOT_COUNT {
        lips = lips
            + centered_cube(
                format!("airflow_shadow_smoke_witness_exhaust_slot_lip_{i}"),
                EXHAUST_SLOT_X + 10.0,
                4.0,
                12.0,
            )
            .translate(
                centered_index(i, EXHAUST_SLOT_COUNT, 48.0),
                -EXHAUST_SLOT_Y / 2.0 - 8.0,
                EXHAUST_Z / 2.0 + 6.0,
            );
    }
    lips
}

fn exhaust_trap_cups() -> Part {
    let mut cups = Part::empty("airflow_shadow_smoke_witness_exhaust_trap_cups");
    for i in 0..EXHAUST_TRAP_CUP_COUNT {
        let x = centered_index(i, EXHAUST_TRAP_CUP_COUNT, 92.0);
        let cup = centered_cylinder(
            format!("airflow_shadow_smoke_witness_exhaust_smoke_trap_cup_{i}"),
            20.0,
            18.0,
            36,
        );
        let recess = centered_cylinder(
            format!("airflow_shadow_smoke_witness_exhaust_smoke_trap_cup_recess_{i}"),
            13.0,
            19.0,
            36,
        );
        cups = cups + (cup - recess).translate(x, -EXHAUST_Y / 2.0 + 32.0, EXHAUST_Z / 2.0 + 9.0);
    }
    cups
}

fn hepa_coupon_lands() -> Part {
    let mut lands = Part::empty("airflow_shadow_smoke_witness_exhaust_hepa_coupon_lands");
    for i in 0..EXHAUST_HEPA_COUPON_COUNT {
        lands = lands
            + centered_cube(
                format!("airflow_shadow_smoke_witness_hepa_capture_coupon_land_{i}"),
                92.0,
                34.0,
                7.0,
            )
            .translate(
                centered_index(i, EXHAUST_HEPA_COUPON_COUNT, 118.0),
                EXHAUST_Y / 2.0 - 32.0,
                EXHAUST_Z / 2.0 + 3.5,
            );
    }
    lands
}

fn exhaust_tube_boss() -> Part {
    let outer = centered_cylinder(
        "airflow_shadow_smoke_witness_exhaust_capture_tube_boss",
        18.0,
        42.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        EXHAUST_X / 2.0 - 42.0,
        EXHAUST_Y / 2.0 - 10.0,
        EXHAUST_Z / 2.0 + 8.0,
    );
    let inner = centered_cylinder(
        "airflow_shadow_smoke_witness_exhaust_capture_tube_bore",
        10.0,
        46.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        EXHAUST_X / 2.0 - 42.0,
        EXHAUST_Y / 2.0 - 10.0,
        EXHAUST_Z / 2.0 + 8.0,
    );
    outer - inner
}

fn camera_evidence_bridge() -> Part {
    let left_anchor = centered_cube(
        "airflow_shadow_smoke_witness_camera_bridge_left_anchor",
        118.0,
        BRIDGE_Y,
        BRIDGE_ANCHOR_Z,
    )
    .translate(-BRIDGE_X / 2.0 + 62.0, 0.0, 0.0);
    let right_anchor = centered_cube(
        "airflow_shadow_smoke_witness_camera_bridge_right_anchor",
        118.0,
        BRIDGE_Y,
        BRIDGE_ANCHOR_Z,
    )
    .translate(BRIDGE_X / 2.0 - 62.0, 0.0, 0.0);
    let left_post = centered_cube(
        "airflow_shadow_smoke_witness_camera_bridge_left_post",
        28.0,
        28.0,
        BRIDGE_POST_Z,
    )
    .translate(
        -BRIDGE_X / 2.0 + 62.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "airflow_shadow_smoke_witness_camera_bridge_right_post",
        28.0,
        28.0,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_X / 2.0 - 62.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "airflow_shadow_smoke_witness_camera_bridge_crossbeam",
        BRIDGE_X - 92.0,
        32.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_anchor
        + right_anchor
        + left_post
        + right_post
        + beam
        + camera_pods()
        + bridge_light_bars()
        + bridge_target_tabs()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("airflow_shadow_smoke_witness_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let pod = centered_cube(
            format!("airflow_shadow_smoke_witness_camera_pod_{i}"),
            74.0,
            46.0,
            16.0,
        );
        let lens = centered_cylinder(
            format!("airflow_shadow_smoke_witness_camera_lens_clearance_{i}"),
            13.0,
            17.0,
            32,
        );
        pods = pods
            + (pod - lens).translate(
                centered_index(i, CAMERA_POD_COUNT, 190.0),
                0.0,
                BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z - 18.0,
            );
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty("airflow_shadow_smoke_witness_bridge_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        let x = centered_index(i, LIGHT_BAR_COUNT, 250.0);
        bars = bars
            + centered_cube(
                format!("airflow_shadow_smoke_witness_bridge_light_bar_{i}"),
                132.0,
                10.0,
                10.0,
            )
            .translate(
                x,
                -BRIDGE_Y / 2.0 + 14.0,
                BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z - 38.0,
            );
    }
    bars
}

fn bridge_target_tabs() -> Part {
    let mut tabs = Part::empty("airflow_shadow_smoke_witness_bridge_target_tabs");
    for i in 0..POSITION_COUNT {
        let x = centered_index(i % LANE_COUNT, LANE_COUNT, LANE_PITCH_X);
        let y = if i / LANE_COUNT == 0 { -22.0 } else { 22.0 };
        tabs = tabs
            + centered_cube(
                format!("airflow_shadow_smoke_witness_camera_position_target_tab_{i}"),
                20.0,
                5.0,
                8.0,
            )
            .translate(x, y, BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z - 62.0);
    }
    tabs
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "airflow_shadow_smoke_witness_barcode_custody_land_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );

    panel + barcode_lands() + custody_seal_lands() + witness_card_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("airflow_shadow_smoke_witness_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("airflow_shadow_smoke_witness_barcode_land_{i}"),
                44.0,
                11.0,
                3.0,
            )
            .translate(
                centered_index(i % 5, 5, 48.0),
                centered_index(i / 5, 3, 24.0) + 14.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty("airflow_shadow_smoke_witness_custody_seal_lands");
    for i in 0..CUSTODY_SEAL_LAND_COUNT {
        seals = seals
            + centered_cylinder(
                format!("airflow_shadow_smoke_witness_custody_seal_land_{i}"),
                8.5,
                3.0,
                24,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LAND_COUNT, 34.0),
                -TRACE_Y / 2.0 + 18.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }
    seals
}

fn witness_card_lands() -> Part {
    let mut cards = Part::empty("airflow_shadow_smoke_witness_witness_card_lands");
    for i in 0..WITNESS_CARD_LANDS {
        cards = cards
            + centered_cube(
                format!("airflow_shadow_smoke_witness_chain_of_custody_card_land_{i}"),
                48.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, WITNESS_CARD_LANDS, 56.0),
                TRACE_Y / 2.0 - 14.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    cards
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "airflow_shadow_smoke_witness_front_robot_service_keepout_gauge",
        KEEP_OUT_X - 80.0,
        16.0,
        54.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE, 27.0);
    let rear_service = centered_cube(
        "airflow_shadow_smoke_witness_rear_service_keepout_gauge",
        KEEP_OUT_X - 110.0,
        16.0,
        48.0,
    )
    .translate(0.0, STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE, 24.0);
    let left_service = centered_cube(
        "airflow_shadow_smoke_witness_left_probe_service_keepout_gauge",
        16.0,
        KEEP_OUT_Y - 128.0,
        46.0,
    )
    .translate(-STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE, 0.0, 23.0);
    let right_exhaust = centered_cube(
        "airflow_shadow_smoke_witness_right_exhaust_service_keepout_gauge",
        16.0,
        KEEP_OUT_Y - 128.0,
        46.0,
    )
    .translate(STATION_X / 2.0 - SIDE_SERVICE_CLEARANCE, 0.0, 23.0);
    let overhead = centered_cube(
        "airflow_shadow_smoke_witness_overhead_camera_bridge_keepout_gauge",
        160.0,
        120.0,
        10.0,
    )
    .translate(0.0, BRIDGE_POS.1, ROBOT_Z_CLEARANCE);
    let robot_gripper_sweep = centered_cube(
        "airflow_shadow_smoke_witness_robot_gripper_lane_sweep_gauge",
        620.0,
        14.0,
        48.0,
    )
    .translate(RACK_POS.0, -STATION_Y / 2.0 + 178.0, 24.0);

    front_robot + rear_service + left_service + right_exhaust + overhead + robot_gripper_sweep
}

fn rectangular_frame_xy(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let rail_x = (outer_x - inner_x) / 2.0;
    let rail_y = (outer_y - inner_y) / 2.0;
    let front = centered_cube(format!("{name}_front"), outer_x, rail_y, z).translate(
        0.0,
        -outer_y / 2.0 + rail_y / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear"), outer_x, rail_y, z).translate(
        0.0,
        outer_y / 2.0 - rail_y / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left"), rail_x, inner_y, z).translate(
        -outer_x / 2.0 + rail_x / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail_x, inner_y, z).translate(
        outer_x / 2.0 - rail_x / 2.0,
        0.0,
        0.0,
    );

    front + rear + left + right
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 14.0, 4.0, 36);
    let inner = centered_cylinder(format!("{name}_inner_cut"), 6.0, 5.0, 36);
    let cross_x = centered_cube(format!("{name}_cross_x"), 28.0, 3.0, 4.4);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 28.0, 4.4);
    outer - inner + cross_x + cross_y
}

fn datum_boss(name: &str) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), 8.5, 8.0, 32);
    let pilot = centered_cylinder(format!("{name}_pilot_cut"), 2.8, 9.0, 24);
    boss - pilot
}

fn lane_position_index(lane: usize, position: usize) -> usize {
    position * LANE_COUNT + lane
}

fn lane_position_center(lane: usize, position: usize) -> (f64, f64) {
    (
        centered_index(lane, LANE_COUNT, LANE_PITCH_X),
        centered_index(position, POSITIONS_PER_LANE, POSITION_PITCH_Y),
    )
}

fn strip_center(lane: usize, position: usize) -> (f64, f64) {
    (
        centered_index(lane, LANE_COUNT, LANE_PITCH_X),
        centered_index(position, POSITIONS_PER_LANE, 46.0),
    )
}

fn coupon_center(group: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COUPON_COLUMNS, 76.0),
        centered_index(group, COUPON_GROUPS, 54.0),
    )
}

fn probe_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, PROBE_COLS, 82.0),
        centered_index(row, PROBE_ROWS, 72.0),
    )
}

fn token_center(lane: usize, position: usize) -> (f64, f64) {
    (
        centered_index(lane, LANE_COUNT, 52.0),
        centered_index(position, POSITIONS_PER_LANE, 27.0),
    )
}

fn position_zone(lane: usize, position: usize) -> PositionZone {
    if lane == 0 || lane + 1 == LANE_COUNT || position == 0 || position + 1 == POSITIONS_PER_LANE {
        PositionZone::Edge
    } else {
        PositionZone::Center
    }
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for lane in 0..LANE_COUNT {
        for position in 0..POSITIONS_PER_LANE {
            if position_zone(lane, position) == PositionZone::Edge {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    POSITION_COUNT - edge_position_count()
}

fn probe_height_for_index(index: usize) -> f64 {
    match index % 3 {
        0 => PROBE_LOW_Z,
        1 => PROBE_MID_Z,
        _ => PROBE_HIGH_Z,
    }
}

fn exhaust_capture_open_area_mm2() -> f64 {
    EXHAUST_SLOT_COUNT as f64 * EXHAUST_SLOT_X * EXHAUST_SLOT_Y
}

fn smoke_port_open_area_mm2() -> f64 {
    let radius = SMOKE_PORT_D / 2.0;
    SMOKE_PORT_COUNT as f64 * std::f64::consts::PI * radius * radius
}

fn deck_basin_freeboard_ml() -> f64 {
    (STATION_X - 150.0) * (STATION_Y - 132.0) * BASIN_DEPTH / 1000.0
}

fn front_robot_clearance() -> f64 {
    (RACK_POS.1 - RACK_Y / 2.0) - (-STATION_Y / 2.0 + RIM_W)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (BRIDGE_POS.1 + BRIDGE_Y / 2.0)
}

fn side_service_clearance() -> f64 {
    (STATION_X / 2.0 - RIM_W) - (EXHAUST_POS.0 + EXHAUST_X / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_stable_unique_and_station_scoped() {
        assert_eq!(OUTPUTS.len(), 11);
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());

        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_cassette_lane_airflow_shadow_smoke_witness_station_"));
            assert!(path.ends_with(".stl"));
        }

        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing required output feature: {feature}"
            );
        }
    }

    #[test]
    fn geometry_is_partitioned_for_airflow_shadow_witnessing() {
        assert_design_constraints();
        assert_eq!(POSITION_COUNT, 12);
        assert_eq!(EDGE_POSITION_COUNT, 10);
        assert_eq!(CENTER_POSITION_COUNT, 2);
        assert_eq!(SMOKE_PORT_COUNT, WITNESS_STRIP_COUNT);
        assert_eq!(SMOKE_PORT_COUNT, POSITION_TOKEN_COUNT);
        assert_eq!(PROBE_FLAG_COUNT, PROBE_MAST_COUNT);
        assert_eq!(EXHAUST_HEPA_COUPON_COUNT, 2);
        assert!(exhaust_capture_open_area_mm2() > smoke_port_open_area_mm2() * 4.0);
    }

    #[test]
    fn rack_and_access_clearances_preserve_robot_service_envelope() {
        assert!(CASSETTE_SURROGATE_X > REVC_CHIP_LENGTH + 30.0);
        assert!(CASSETTE_SURROGATE_Y > REVC_CHIP_WIDTH + 24.0);
        assert!(CASSETTE_SURROGATE_Z > REVC_TOTAL_HEIGHT + 12.0);
        assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
        assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
        assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);
        assert!(CAMERA_CLEARANCE_Z > PROBE_HIGH_Z + 70.0);
    }
}
