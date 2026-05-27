use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell wash/concentrate interface volume-recovery station.
//
// Intent:
// - Validate the fit, traceability, and recovery-volume evidence around a
//   bought closed-cell wash/concentrator before live product is connected.
// - Keep bag inlets/outlets, low-dead-volume adapter coupons, pressure/flow
//   taps, recovery witness wells, cell-loss surrogate capture, waste/retain
//   split, disposition lanes, and automation clearances visible on one deck.
// - Make the station a no-cell interface/recovery fixture. It is not a sterile
//   barrier drawing, cell-processing protocol, biological acceptance method, or
//   operating recipe for any commercial wash/concentrator.

const PREFIX: &str = "closed_cell_wash_concentrate_interface_volume_recovery_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_secondary_containment_deck.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_bought_wash_concentrator_envelope.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_inlet_outlet_bag_nests.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_recovery_volume_witness_wells.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_low_dead_volume_adapter_coupons.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_pressure_flow_taps.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_cell_loss_surrogate_capture_rack.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_tubing_strain_relief.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_waste_retain_split_manifold.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_release_hold_reject_lanes.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_evidence_bridge.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_robot_service_keepouts.stl",
    "output/closed_cell_wash_concentrate_interface_volume_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "secondary_containment_deck",
    "bought_wash_concentrator_envelope",
    "inlet_outlet_bag_nests",
    "recovery_volume_witness_wells",
    "low_dead_volume_adapter_coupons",
    "pressure_flow_taps",
    "cell_loss_surrogate_capture_rack",
    "tubing_strain_relief",
    "waste_retain_split_manifold",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const BAG_ROLES: [&str; 4] = [
    "wash_inlet",
    "buffer_inlet",
    "retain_outlet",
    "waste_outlet",
];
const DECISION_LANES: [&str; 3] = ["release", "hold", "reject"];
const SPLIT_STREAMS: [&str; 2] = ["retain", "waste"];

const STATION_X: f64 = 1380.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const BASIN_X: f64 = 1240.0;
const BASIN_Y: f64 = 740.0;
const BASIN_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_WITNESS_PADS: usize = 6;
const ROBOT_DATUMS: usize = 8;

const PROCESS_LANES: usize = 6;

const WASH_CENTER: (f64, f64) = (-340.0, 175.0);
const WASH_X: f64 = 500.0;
const WASH_Y: f64 = 260.0;
const WASH_Z: f64 = 74.0;
const WASH_FOOTPRINT_X: f64 = 430.0;
const WASH_FOOTPRINT_Y: f64 = 190.0;
const WASH_HANDLE_BAYS: usize = 2;
const WASH_INTERFACE_PORTS: usize = PROCESS_LANES;

const BAG_CENTER: (f64, f64) = (370.0, 175.0);
const BAG_PANEL_X: f64 = 450.0;
const BAG_PANEL_Y: f64 = 260.0;
const BAG_PANEL_Z: f64 = 42.0;
const BAG_NESTS: usize = BAG_ROLES.len();
const BAG_NEST_X: f64 = 168.0;
const BAG_NEST_Y: f64 = 88.0;
const BAG_NEST_Z: f64 = 9.0;
const BAG_PITCH_X: f64 = 198.0;
const BAG_PITCH_Y: f64 = 112.0;
const BAG_NECK_D: f64 = 22.0;
const BAG_SCALE_WINDOWS: usize = BAG_NESTS;

const WITNESS_CENTER: (f64, f64) = (-450.0, -100.0);
const WITNESS_PANEL_X: f64 = 330.0;
const WITNESS_PANEL_Y: f64 = 215.0;
const WITNESS_PANEL_Z: f64 = 36.0;
const WITNESS_ROWS: usize = 3;
const WITNESS_COLS: usize = 4;
const WITNESS_WELLS: usize = WITNESS_ROWS * WITNESS_COLS;
const WITNESS_WELL_D: f64 = 22.0;
const WITNESS_WELL_CLEARANCE_D: f64 = 25.0;
const WITNESS_PITCH_X: f64 = 64.0;
const WITNESS_PITCH_Y: f64 = 54.0;
const WITNESS_VOLUME_UL_PER_STEP: f64 = 50.0;

const ADAPTER_CENTER: (f64, f64) = (-68.0, -100.0);
const ADAPTER_PANEL_X: f64 = 380.0;
const ADAPTER_PANEL_Y: f64 = 215.0;
const ADAPTER_PANEL_Z: f64 = 32.0;
const ADAPTER_COUPONS: usize = PROCESS_LANES * 2;
const ADAPTER_COUPON_X: f64 = 42.0;
const ADAPTER_COUPON_Y: f64 = 32.0;
const ADAPTER_COUPON_Z: f64 = 12.0;
const ADAPTER_PITCH_X: f64 = 54.0;
const ADAPTER_PITCH_Y: f64 = 70.0;
const ADAPTER_BORE_D: f64 = 1.4;
const ADAPTER_TUBE_SEAT_D: f64 = 5.0;
const ADAPTER_HOLDUP_UL: f64 = 7.8;

const TAP_CENTER: (f64, f64) = (350.0, -100.0);
const TAP_PANEL_X: f64 = 340.0;
const TAP_PANEL_Y: f64 = 215.0;
const TAP_PANEL_Z: f64 = 36.0;
const PRESSURE_TAPS_PER_LANE: usize = 2;
const PRESSURE_TAPS: usize = PROCESS_LANES * PRESSURE_TAPS_PER_LANE;
const FLOW_TAPS: usize = PROCESS_LANES;
const TAP_PITCH_X: f64 = 49.0;
const PRESSURE_TAP_D: f64 = 5.2;
const FLOW_WINDOW_X: f64 = 28.0;
const FLOW_WINDOW_Y: f64 = 10.0;

const CAPTURE_CENTER: (f64, f64) = (-405.0, -320.0);
const CAPTURE_RACK_X: f64 = 430.0;
const CAPTURE_RACK_Y: f64 = 150.0;
const CAPTURE_RACK_Z: f64 = 34.0;
const CAPTURE_ROWS: usize = 3;
const CAPTURE_COLS: usize = PROCESS_LANES;
const CAPTURE_POSITIONS: usize = CAPTURE_ROWS * CAPTURE_COLS;
const CAPTURE_DISC_D: f64 = 24.0;
const CAPTURE_PITCH_X: f64 = 58.0;
const CAPTURE_PITCH_Y: f64 = 42.0;

const STRAIN_CENTER: (f64, f64) = (38.0, -320.0);
const STRAIN_PANEL_X: f64 = 420.0;
const STRAIN_PANEL_Y: f64 = 150.0;
const STRAIN_PANEL_Z: f64 = 30.0;
const STRAIN_RELIEF_SLOTS: usize = PROCESS_LANES * 2;
const STRAIN_SLOT_X: f64 = 20.0;
const STRAIN_SLOT_Y: f64 = 86.0;
const STRAIN_SLOT_PITCH_X: f64 = 30.0;
const MIN_TUBE_BEND_RADIUS: f64 = 42.0;

const SPLIT_CENTER: (f64, f64) = (450.0, -320.0);
const SPLIT_PANEL_X: f64 = 230.0;
const SPLIT_PANEL_Y: f64 = 150.0;
const SPLIT_PANEL_Z: f64 = 42.0;
const SPLIT_PORT_D: f64 = 18.0;
const SPLIT_DIVERTER_TOKENS: usize = 2;
const RETAIN_BAG_VOLUME_ML: f64 = 650.0;
const WASTE_BAG_VOLUME_ML: f64 = 900.0;

const DECISION_CENTER: (f64, f64) = (60.0, 372.0);
const DECISION_PANEL_X: f64 = 640.0;
const DECISION_PANEL_Y: f64 = 100.0;
const DECISION_PANEL_Z: f64 = 24.0;
const DECISION_LANE_X: f64 = 174.0;
const DECISION_LANE_Y: f64 = 62.0;
const DECISION_PITCH_X: f64 = 198.0;
const DECISION_TOKEN_SLOTS: usize = 12;

const BRIDGE_SPAN_X: f64 = 1240.0;
const BRIDGE_Y: f64 = 74.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 46.0;
const BRIDGE_UNDERSIDE_Z: f64 = 238.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;
const EVIDENCE_CLEARANCE_Z: f64 = 330.0;

const KEEP_OUT_RAIL_W: f64 = 12.0;
const KEEP_OUT_Z: f64 = 96.0;
const ROBOT_FRONT_CLEARANCE: f64 = 380.0;
const SERVICE_REAR_CLEARANCE: f64 = 235.0;
const LEFT_CONCENTRATOR_SERVICE_CLEARANCE: f64 = 220.0;
const RIGHT_BAG_SERVICE_CLEARANCE: f64 = 210.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self, margin: f64) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - margin
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - margin
    }

    fn overlaps(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(&deck, OUTPUTS[0]);

    let envelope = bought_wash_concentrator_envelope();
    export(&envelope, OUTPUTS[1]);

    let bags = inlet_outlet_bag_nests();
    export(&bags, OUTPUTS[2]);

    let witnesses = recovery_volume_witness_wells();
    export(&witnesses, OUTPUTS[3]);

    let adapters = low_dead_volume_adapter_coupons();
    export(&adapters, OUTPUTS[4]);

    let taps = pressure_flow_taps();
    export(&taps, OUTPUTS[5]);

    let capture = cell_loss_surrogate_capture_rack();
    export(&capture, OUTPUTS[6]);

    let strain = tubing_strain_relief();
    export(&strain, OUTPUTS[7]);

    let split = waste_retain_split_manifold();
    export(&split, OUTPUTS[8]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[9]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = deck
        + envelope
        + bags
        + witnesses
        + adapters
        + taps
        + capture
        + strain
        + split
        + lanes
        + bridge
        + keepouts
        + closed_fluid_route_overlays();
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed cell wash/concentrate interface volume-recovery station:");
    println!(
        "  Deck:                 {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray with {LEAK_WITNESS_PADS} leak witness pads and {ROBOT_DATUMS} robot datums"
    );
    println!(
        "  Bought envelope:      {WASH_FOOTPRINT_X:.0}mm x {WASH_FOOTPRINT_Y:.0}mm removable wash/concentrator shadow with {WASH_INTERFACE_PORTS} interface port witnesses"
    );
    println!(
        "  Bags and recovery:    {BAG_NESTS} inlet/outlet bag nests, {WITNESS_WELLS} recovery witness wells at {WITNESS_VOLUME_UL_PER_STEP:.0}uL ladder steps"
    );
    println!(
        "  Interface evidence:   {ADAPTER_COUPONS} low-dead-volume coupons ({ADAPTER_HOLDUP_UL:.1}uL target), {PRESSURE_TAPS} pressure taps, {FLOW_TAPS} flow windows"
    );
    println!(
        "  Cell-loss check:      {CAPTURE_POSITIONS} surrogate capture positions feeding {SPLIT_STREAMS:?} split and {DECISION_LANES:?} disposition lanes"
    );
    println!(
        "  Automation clearance: front robot {ROBOT_FRONT_CLEARANCE:.0}mm, rear service {SERVICE_REAR_CLEARANCE:.0}mm, left concentrator service {LEFT_CONCENTRATOR_SERVICE_CLEARANCE:.0}mm, right bag service {RIGHT_BAG_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(BAG_NESTS, 4);
    assert_eq!(BAG_SCALE_WINDOWS, BAG_NESTS);
    assert_eq!(ADAPTER_COUPONS, PROCESS_LANES * 2);
    assert_eq!(PRESSURE_TAPS, PROCESS_LANES * PRESSURE_TAPS_PER_LANE);
    assert_eq!(FLOW_TAPS, PROCESS_LANES);
    assert_eq!(CAPTURE_POSITIONS, PROCESS_LANES * CAPTURE_ROWS);
    assert_eq!(STRAIN_RELIEF_SLOTS, PROCESS_LANES * 2);
    assert_eq!(DECISION_TOKEN_SLOTS, DECISION_LANES.len() * 4);
    assert!(ADAPTER_HOLDUP_UL <= 8.0);
    assert!(ADAPTER_BORE_D < ADAPTER_TUBE_SEAT_D);
    assert!(WITNESS_WELL_CLEARANCE_D > WITNESS_WELL_D);
    assert!(RETAIN_BAG_VOLUME_ML >= 600.0);
    assert!(WASTE_BAG_VOLUME_ML > RETAIN_BAG_VOLUME_ML);
    assert!(BRIDGE_UNDERSIDE_Z > BASE_Z + WASH_Z + 100.0);
    assert!(EVIDENCE_CLEARANCE_Z > BRIDGE_UNDERSIDE_Z);
    assert!(MIN_TUBE_BEND_RADIUS >= 40.0);

    let rects = module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(24.0),
            "{} exceeds station envelope",
            rect.name
        );
    }

    for (left_index, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(left_index + 1) {
            assert!(
                !left.overlaps(*right, 10.0),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }

    for path in OUTPUTS {
        assert!(path.starts_with("output/"));
        assert!(path.contains(PREFIX));
        assert!(path.ends_with(".stl"));
    }
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "bought_wash_concentrator_envelope",
            WASH_CENTER,
            WASH_X,
            WASH_Y,
        ),
        rect(
            "inlet_outlet_bag_nests",
            BAG_CENTER,
            BAG_PANEL_X,
            BAG_PANEL_Y,
        ),
        rect(
            "recovery_volume_witness_wells",
            WITNESS_CENTER,
            WITNESS_PANEL_X,
            WITNESS_PANEL_Y,
        ),
        rect(
            "low_dead_volume_adapter_coupons",
            ADAPTER_CENTER,
            ADAPTER_PANEL_X,
            ADAPTER_PANEL_Y,
        ),
        rect("pressure_flow_taps", TAP_CENTER, TAP_PANEL_X, TAP_PANEL_Y),
        rect(
            "cell_loss_surrogate_capture_rack",
            CAPTURE_CENTER,
            CAPTURE_RACK_X,
            CAPTURE_RACK_Y,
        ),
        rect(
            "tubing_strain_relief",
            STRAIN_CENTER,
            STRAIN_PANEL_X,
            STRAIN_PANEL_Y,
        ),
        rect(
            "waste_retain_split_manifold",
            SPLIT_CENTER,
            SPLIT_PANEL_X,
            SPLIT_PANEL_Y,
        ),
        rect(
            "release_hold_reject_lanes",
            DECISION_CENTER,
            DECISION_PANEL_X,
            DECISION_PANEL_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_recessed_recovery_spill_basin"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        format!("{PREFIX}_low_point_recovery_drain"),
        DRAIN_D / 2.0,
        86.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 90.0,
        -STATION_Y / 2.0 + 14.0,
        BASE_Z - 5.0,
    );

    deck - basin - drain - module_socket_reliefs()
        + perimeter_rims()
        + mount_bosses()
        + wet_dry_divider_rails()
        + leak_witness_pads()
        + robot_datum_fiducials()
}

fn module_socket_reliefs() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_socket_reliefs"));
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket_relief", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                6.8,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z - 3.0);
    }
    sockets
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_spill_lip_with_robot_access_gap"),
        STATION_X - 150.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z_on_deck(RIM_Z));
    let rear = centered_cube(
        format!("{PREFIX}_rear_service_tubing_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z_on_deck(RIM_Z));
    let left = centered_cube(
        format!("{PREFIX}_left_bought_device_service_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z_on_deck(RIM_Z));
    let right = centered_cube(
        format!("{PREFIX}_right_bag_service_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z_on_deck(RIM_Z));

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PREFIX}_deck_mount_bosses"));
    for (index, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_mount_boss_{index}"), 58.0, 30.0, 7.0)
            .translate(*x, *y, BASE_Z + 3.5);
        let hole = centered_cylinder(
            format!("{PREFIX}_m6_clearance_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 9.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        bosses = bosses + boss - hole;
    }
    bosses
}

fn wet_dry_divider_rails() -> Part {
    let top_divider = centered_cube(
        format!("{PREFIX}_bought_unit_to_decision_lane_splash_divider"),
        STATION_X - 170.0,
        8.0,
        26.0,
    )
    .translate(0.0, 318.0, z_on_deck(26.0));
    let lower_divider = centered_cube(
        format!("{PREFIX}_recovery_to_lower_evidence_divider"),
        STATION_X - 180.0,
        8.0,
        24.0,
    )
    .translate(0.0, -220.0, z_on_deck(24.0));
    let waste_guard = centered_cube(
        format!("{PREFIX}_waste_retain_split_splash_guard"),
        10.0,
        SPLIT_PANEL_Y + 52.0,
        34.0,
    )
    .translate(
        SPLIT_CENTER.0 - SPLIT_PANEL_X / 2.0 - 22.0,
        SPLIT_CENTER.1,
        z_on_deck(34.0),
    );
    top_divider + lower_divider + waste_guard
}

fn leak_witness_pads() -> Part {
    let mut pads = Part::empty(format!("{PREFIX}_leak_witness_pads"));
    for index in 0..LEAK_WITNESS_PADS {
        let x = centered_index(index, LEAK_WITNESS_PADS, 166.0);
        let pad = centered_cylinder(format!("{PREFIX}_leak_witness_pad_{index}"), 14.0, 5.0, 32)
            .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z + 2.5);
        let recess = centered_cylinder(
            format!("{PREFIX}_leak_witness_wetness_recess_{index}"),
            6.0,
            6.0,
            24,
        )
        .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z + 2.5);
        pads = pads + (pad - recess);
    }
    pads
}

fn robot_datum_fiducials() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_robot_datum_fiducials"));
    for (index, (x, y)) in datum_points().iter().enumerate() {
        datums = datums
            + fiducial(&format!("{PREFIX}_robot_datum_{index}")).translate(*x, *y, BASE_Z + 2.0);
    }
    datums
}

fn bought_wash_concentrator_envelope() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_bought_wash_concentrator_envelope_base_plate"),
        WASH_X,
        WASH_Y,
        WASH_Z,
    )
    .translate(WASH_CENTER.0, WASH_CENTER.1, z_on_deck(WASH_Z));
    let recess = centered_cube(
        format!("{PREFIX}_bought_device_removable_shadow_recess"),
        WASH_FOOTPRINT_X,
        WASH_FOOTPRINT_Y,
        WASH_Z + 2.0,
    )
    .translate(WASH_CENTER.0, WASH_CENTER.1, z_on_deck(WASH_Z) + 8.0);
    let raised_outline = envelope_outline_rails();

    base - recess + raised_outline + bought_device_stop_blocks() + wash_interface_port_witnesses()
}

fn envelope_outline_rails() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_bought_device_front_footprint_rail"),
        WASH_FOOTPRINT_X + 28.0,
        12.0,
        24.0,
    )
    .translate(
        WASH_CENTER.0,
        WASH_CENTER.1 - WASH_FOOTPRINT_Y / 2.0 - 12.0,
        BASE_Z + WASH_Z + 12.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_bought_device_rear_footprint_rail"),
        WASH_FOOTPRINT_X + 28.0,
        12.0,
        24.0,
    )
    .translate(
        WASH_CENTER.0,
        WASH_CENTER.1 + WASH_FOOTPRINT_Y / 2.0 + 12.0,
        BASE_Z + WASH_Z + 12.0,
    );
    let left = centered_cube(
        format!("{PREFIX}_bought_device_left_footprint_rail"),
        12.0,
        WASH_FOOTPRINT_Y + 28.0,
        24.0,
    )
    .translate(
        WASH_CENTER.0 - WASH_FOOTPRINT_X / 2.0 - 12.0,
        WASH_CENTER.1,
        BASE_Z + WASH_Z + 12.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_bought_device_right_footprint_rail"),
        12.0,
        WASH_FOOTPRINT_Y + 28.0,
        24.0,
    )
    .translate(
        WASH_CENTER.0 + WASH_FOOTPRINT_X / 2.0 + 12.0,
        WASH_CENTER.1,
        BASE_Z + WASH_Z + 12.0,
    );

    front + rear + left + right
}

fn bought_device_stop_blocks() -> Part {
    let mut stops = Part::empty(format!("{PREFIX}_bought_device_stop_blocks"));
    for (index, (x_sign, y_sign)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let stop = centered_cube(
            format!("{PREFIX}_bought_device_adjustable_stop_block_{index}"),
            46.0,
            28.0,
            28.0,
        )
        .translate(
            WASH_CENTER.0 + x_sign * (WASH_FOOTPRINT_X / 2.0 + 38.0),
            WASH_CENTER.1 + y_sign * (WASH_FOOTPRINT_Y / 2.0 + 30.0),
            BASE_Z + WASH_Z + 14.0,
        );
        let screw = centered_cylinder(
            format!("{PREFIX}_bought_device_stop_block_screw_relief_{index}"),
            4.0,
            32.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            WASH_CENTER.0 + x_sign * (WASH_FOOTPRINT_X / 2.0 + 38.0),
            WASH_CENTER.1 + y_sign * (WASH_FOOTPRINT_Y / 2.0 + 30.0),
            BASE_Z + WASH_Z + 14.0,
        );
        stops = stops + (stop - screw);
    }

    for bay in 0..WASH_HANDLE_BAYS {
        let x = WASH_CENTER.0 + centered_index(bay, WASH_HANDLE_BAYS, 174.0);
        stops = stops
            + centered_cube(
                format!("{PREFIX}_bought_device_lift_handle_keepclear_bay_{bay}"),
                120.0,
                14.0,
                8.0,
            )
            .translate(
                x,
                WASH_CENTER.1 + WASH_Y / 2.0 - 22.0,
                BASE_Z + WASH_Z + 4.0,
            );
    }

    stops
}

fn wash_interface_port_witnesses() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_wash_interface_port_witnesses"));
    for lane in 0..WASH_INTERFACE_PORTS {
        let y = WASH_CENTER.1 + centered_index(lane, WASH_INTERFACE_PORTS, 30.0);
        let inlet = centered_cylinder(
            format!("{PREFIX}_lane_{lane}_bought_unit_inlet_port_shadow"),
            9.0,
            14.0,
            32,
        )
        .translate(
            WASH_CENTER.0 + WASH_FOOTPRINT_X / 2.0 + 26.0,
            y,
            BASE_Z + WASH_Z + 7.0,
        );
        let outlet = centered_cylinder(
            format!("{PREFIX}_lane_{lane}_bought_unit_outlet_port_shadow"),
            9.0,
            14.0,
            32,
        )
        .translate(
            WASH_CENTER.0 - WASH_FOOTPRINT_X / 2.0 - 26.0,
            y,
            BASE_Z + WASH_Z + 7.0,
        );
        ports = ports + inlet + outlet;
    }
    ports
}

fn inlet_outlet_bag_nests() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_inlet_outlet_bag_nest_panel"),
        BAG_PANEL_X,
        BAG_PANEL_Y,
        BAG_PANEL_Z,
    )
    .translate(BAG_CENTER.0, BAG_CENTER.1, z_on_deck(BAG_PANEL_Z));
    let mut cuts = Part::empty(format!("{PREFIX}_bag_nest_recess_cuts"));
    let mut features = Part::empty(format!("{PREFIX}_bag_nest_features"));

    for (index, role) in BAG_ROLES.iter().enumerate() {
        let (x, y) = bag_nest_center(index);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_{role}_bag_body_recess"),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_NEST_Z + 2.0,
            )
            .translate(x, y, BASE_Z + BAG_PANEL_Z - BAG_NEST_Z / 2.0 + 0.3)
            + centered_cylinder(
                format!("{PREFIX}_{role}_bag_neck_clearance"),
                BAG_NECK_D / 2.0,
                BAG_PANEL_Z + 8.0,
                36,
            )
            .translate(x - BAG_NEST_X / 2.0 + 26.0, y, z_on_deck(BAG_PANEL_Z));

        features = features
            + centered_cube(format!("{PREFIX}_{role}_bag_hanger_tab"), 54.0, 12.0, 12.0).translate(
                x,
                y + BAG_NEST_Y / 2.0 + 16.0,
                BASE_Z + BAG_PANEL_Z + 6.0,
            )
            + centered_cube(
                format!("{PREFIX}_{role}_bag_barcode_label_land"),
                86.0,
                18.0,
                4.0,
            )
            .translate(x, y - BAG_NEST_Y / 2.0 - 14.0, BASE_Z + BAG_PANEL_Z + 2.0)
            + centered_cube(format!("{PREFIX}_{role}_load_cell_window"), 92.0, 20.0, 3.0)
                .translate(x + 18.0, y, BASE_Z + BAG_PANEL_Z + 1.5);
    }

    panel - cuts + features + bag_nest_dividers()
}

fn bag_nest_dividers() -> Part {
    let vertical = centered_cube(
        format!("{PREFIX}_bag_nest_left_right_stream_divider"),
        10.0,
        BAG_PANEL_Y - 34.0,
        24.0,
    )
    .translate(BAG_CENTER.0, BAG_CENTER.1, BASE_Z + BAG_PANEL_Z + 12.0);
    let horizontal = centered_cube(
        format!("{PREFIX}_bag_nest_inlet_outlet_stream_divider"),
        BAG_PANEL_X - 34.0,
        10.0,
        24.0,
    )
    .translate(BAG_CENTER.0, BAG_CENTER.1, BASE_Z + BAG_PANEL_Z + 12.0);
    vertical + horizontal
}

fn recovery_volume_witness_wells() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_recovery_volume_witness_well_panel"),
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    )
    .translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        z_on_deck(WITNESS_PANEL_Z),
    );
    let mut cuts = Part::empty(format!("{PREFIX}_recovery_witness_well_cuts"));
    let mut features = Part::empty(format!("{PREFIX}_recovery_witness_well_features"));

    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let index = row * WITNESS_COLS + col;
            let x = WITNESS_CENTER.0 + centered_index(col, WITNESS_COLS, WITNESS_PITCH_X);
            let y = WITNESS_CENTER.1 + centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_recovery_volume_witness_well_{index}_cut"),
                    WITNESS_WELL_CLEARANCE_D / 2.0,
                    WITNESS_PANEL_Z + 8.0,
                    40,
                )
                .translate(x, y, z_on_deck(WITNESS_PANEL_Z));
            features = features
                + centered_cylinder(
                    format!("{PREFIX}_recovery_volume_witness_well_{index}_raised_rim"),
                    WITNESS_WELL_CLEARANCE_D / 2.0 + 4.0,
                    5.0,
                    40,
                )
                .translate(x, y, BASE_Z + WITNESS_PANEL_Z + 2.5)
                - centered_cylinder(
                    format!("{PREFIX}_recovery_volume_witness_well_{index}_rim_opening"),
                    WITNESS_WELL_CLEARANCE_D / 2.0,
                    6.0,
                    40,
                )
                .translate(x, y, BASE_Z + WITNESS_PANEL_Z + 2.5);

            for tick in 0..3 {
                features = features
                    + centered_cube(
                        format!("{PREFIX}_well_{index}_volume_tick_{tick}"),
                        16.0 + tick as f64 * 7.0,
                        2.4,
                        3.0,
                    )
                    .translate(
                        x + WITNESS_WELL_CLEARANCE_D / 2.0 + 12.0,
                        y - 10.0 + tick as f64 * 9.0,
                        BASE_Z + WITNESS_PANEL_Z + 1.5,
                    );
            }
        }
    }

    panel - cuts + features
}

fn low_dead_volume_adapter_coupons() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_low_dead_volume_adapter_coupon_panel"),
        ADAPTER_PANEL_X,
        ADAPTER_PANEL_Y,
        ADAPTER_PANEL_Z,
    )
    .translate(
        ADAPTER_CENTER.0,
        ADAPTER_CENTER.1,
        z_on_deck(ADAPTER_PANEL_Z),
    );
    let mut cuts = Part::empty(format!("{PREFIX}_low_dead_volume_coupon_cuts"));
    let mut coupons = Part::empty(format!("{PREFIX}_low_dead_volume_coupon_features"));

    for lane in 0..PROCESS_LANES {
        let x = ADAPTER_CENTER.0 + centered_index(lane, PROCESS_LANES, ADAPTER_PITCH_X);
        for side in 0..2 {
            let y = ADAPTER_CENTER.1 + centered_index(side, 2, ADAPTER_PITCH_Y);
            let coupon_index = lane * 2 + side;
            let coupon = centered_cube(
                format!("{PREFIX}_lane_{lane}_adapter_coupon_{side}"),
                ADAPTER_COUPON_X,
                ADAPTER_COUPON_Y,
                ADAPTER_COUPON_Z,
            )
            .translate(x, y, BASE_Z + ADAPTER_PANEL_Z + ADAPTER_COUPON_Z / 2.0);
            let bore = centered_cylinder(
                format!("{PREFIX}_lane_{lane}_adapter_coupon_{side}_microbore"),
                ADAPTER_BORE_D / 2.0,
                ADAPTER_COUPON_X + 8.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, BASE_Z + ADAPTER_PANEL_Z + ADAPTER_COUPON_Z / 2.0);
            let tube_seat = centered_cylinder(
                format!("{PREFIX}_lane_{lane}_adapter_coupon_{side}_tube_seat"),
                ADAPTER_TUBE_SEAT_D / 2.0,
                ADAPTER_COUPON_Z + 8.0,
                24,
            )
            .translate(
                x - ADAPTER_COUPON_X / 2.0 + 8.0,
                y,
                BASE_Z + ADAPTER_PANEL_Z,
            );
            let inspection_slot = centered_cube(
                format!("{PREFIX}_adapter_coupon_{coupon_index}_meniscus_inspection_slot"),
                24.0,
                5.0,
                4.0,
            )
            .translate(
                x,
                y + ADAPTER_COUPON_Y / 2.0 + 5.0,
                BASE_Z + ADAPTER_PANEL_Z + 2.0,
            );
            coupons = coupons + (coupon - bore - tube_seat) + inspection_slot;
        }
    }

    for lane in 0..PROCESS_LANES {
        let x = ADAPTER_CENTER.0 + centered_index(lane, PROCESS_LANES, ADAPTER_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_adapter_coupon_pull_recess"),
                38.0,
                ADAPTER_PANEL_Y - 30.0,
                5.8,
            )
            .translate(x, ADAPTER_CENTER.1, BASE_Z + ADAPTER_PANEL_Z - 2.5);
    }

    panel - cuts + coupons + adapter_lane_flow_traces()
}

fn adapter_lane_flow_traces() -> Part {
    let mut traces = Part::empty(format!("{PREFIX}_adapter_lane_flow_traces"));
    for lane in 0..PROCESS_LANES {
        let x = ADAPTER_CENTER.0 + centered_index(lane, PROCESS_LANES, ADAPTER_PITCH_X);
        traces = traces
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_adapter_low_volume_bridge_trace"),
                8.0,
                ADAPTER_PITCH_Y - 22.0,
                4.0,
            )
            .translate(x, ADAPTER_CENTER.1, BASE_Z + ADAPTER_PANEL_Z + 2.0);
    }
    traces
}

fn pressure_flow_taps() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_pressure_flow_tap_panel"),
        TAP_PANEL_X,
        TAP_PANEL_Y,
        TAP_PANEL_Z,
    )
    .translate(TAP_CENTER.0, TAP_CENTER.1, z_on_deck(TAP_PANEL_Z));
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_flow_tap_cuts"));
    let mut features = Part::empty(format!("{PREFIX}_pressure_flow_tap_features"));

    for lane in 0..PROCESS_LANES {
        let x = TAP_CENTER.0 + centered_index(lane, PROCESS_LANES, TAP_PITCH_X);
        let flow_window = centered_cube(
            format!("{PREFIX}_lane_{lane}_flow_sight_window"),
            FLOW_WINDOW_X,
            FLOW_WINDOW_Y,
            TAP_PANEL_Z + 8.0,
        )
        .translate(x, TAP_CENTER.1, z_on_deck(TAP_PANEL_Z));
        cuts = cuts + flow_window;

        for tap in 0..PRESSURE_TAPS_PER_LANE {
            let y = TAP_CENTER.1 + centered_index(tap, PRESSURE_TAPS_PER_LANE, 72.0);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_pressure_tap_{tap}_cut"),
                    PRESSURE_TAP_D / 2.0,
                    TAP_PANEL_Z + 8.0,
                    24,
                )
                .translate(x, y, z_on_deck(TAP_PANEL_Z));
            features = features
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_pressure_tap_{tap}_luer_boss"),
                    PRESSURE_TAP_D / 2.0 + 6.0,
                    6.0,
                    28,
                )
                .translate(x, y, BASE_Z + TAP_PANEL_Z + 3.0);
        }

        features = features
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_flow_arrow_token"),
                34.0,
                7.0,
                4.0,
            )
            .translate(x, TAP_CENTER.1 - 30.0, BASE_Z + TAP_PANEL_Z + 2.0);
    }

    panel - cuts + features
}

fn cell_loss_surrogate_capture_rack() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_cell_loss_surrogate_capture_rack_body"),
        CAPTURE_RACK_X,
        CAPTURE_RACK_Y,
        CAPTURE_RACK_Z,
    )
    .translate(
        CAPTURE_CENTER.0,
        CAPTURE_CENTER.1,
        z_on_deck(CAPTURE_RACK_Z),
    );
    let mut cuts = Part::empty(format!("{PREFIX}_surrogate_capture_disc_cuts"));
    let mut features = Part::empty(format!("{PREFIX}_surrogate_capture_features"));

    for row in 0..CAPTURE_ROWS {
        for col in 0..CAPTURE_COLS {
            let index = row * CAPTURE_COLS + col;
            let x = CAPTURE_CENTER.0 + centered_index(col, CAPTURE_COLS, CAPTURE_PITCH_X);
            let y = CAPTURE_CENTER.1 + centered_index(row, CAPTURE_ROWS, CAPTURE_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_capture_position_{index}_mesh_coupon_recess"),
                    CAPTURE_DISC_D / 2.0,
                    CAPTURE_RACK_Z + 8.0,
                    36,
                )
                .translate(x, y, z_on_deck(CAPTURE_RACK_Z));
            features = features
                + centered_cylinder(
                    format!("{PREFIX}_capture_position_{index}_retaining_rim"),
                    CAPTURE_DISC_D / 2.0 + 4.0,
                    5.0,
                    36,
                )
                .translate(x, y, BASE_Z + CAPTURE_RACK_Z + 2.5)
                - centered_cylinder(
                    format!("{PREFIX}_capture_position_{index}_rim_opening"),
                    CAPTURE_DISC_D / 2.0,
                    6.0,
                    36,
                )
                .translate(x, y, BASE_Z + CAPTURE_RACK_Z + 2.5);
        }
    }

    rack - cuts + features + capture_rack_identity_lands()
}

fn capture_rack_identity_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_capture_rack_identity_lands"));
    for lane in 0..PROCESS_LANES {
        let x = CAPTURE_CENTER.0 + centered_index(lane, PROCESS_LANES, CAPTURE_PITCH_X);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_capture_barcode_land"),
                42.0,
                12.0,
                4.0,
            )
            .translate(
                x,
                CAPTURE_CENTER.1 - CAPTURE_RACK_Y / 2.0 - 10.0,
                BASE_Z + 2.0,
            );
    }
    lands
}

fn tubing_strain_relief() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_tubing_strain_relief_panel"),
        STRAIN_PANEL_X,
        STRAIN_PANEL_Y,
        STRAIN_PANEL_Z,
    )
    .translate(STRAIN_CENTER.0, STRAIN_CENTER.1, z_on_deck(STRAIN_PANEL_Z));
    let mut cuts = Part::empty(format!("{PREFIX}_strain_relief_slot_cuts"));
    let mut clamps = Part::empty(format!("{PREFIX}_strain_relief_clamp_features"));

    for slot in 0..STRAIN_RELIEF_SLOTS {
        let x = STRAIN_CENTER.0 + centered_index(slot, STRAIN_RELIEF_SLOTS, STRAIN_SLOT_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_strain_relief_slot_{slot}"),
                STRAIN_SLOT_X,
                STRAIN_SLOT_Y,
                STRAIN_PANEL_Z + 8.0,
            )
            .translate(x, STRAIN_CENTER.1, z_on_deck(STRAIN_PANEL_Z));
        clamps = clamps
            + centered_cube(
                format!("{PREFIX}_strain_relief_slot_{slot}_spring_clip_land"),
                STRAIN_SLOT_X + 12.0,
                8.0,
                8.0,
            )
            .translate(
                x,
                STRAIN_CENTER.1 + STRAIN_SLOT_Y / 2.0 + 10.0,
                BASE_Z + 4.0,
            );
    }

    panel - cuts + clamps + bend_radius_gauges()
}

fn bend_radius_gauges() -> Part {
    let mut gauges = Part::empty(format!("{PREFIX}_tube_bend_radius_gauges"));
    for index in 0..3 {
        let radius = MIN_TUBE_BEND_RADIUS + index as f64 * 12.0;
        let gauge = centered_cylinder(
            format!("{PREFIX}_bend_radius_gauge_{radius:.0}mm"),
            radius,
            5.0,
            64,
        )
        .translate(
            STRAIN_CENTER.0 - STRAIN_PANEL_X / 2.0 + 58.0 + index as f64 * 92.0,
            STRAIN_CENTER.1 - STRAIN_PANEL_Y / 2.0 - 18.0,
            BASE_Z + 2.5,
        );
        let inner = centered_cylinder(
            format!("{PREFIX}_bend_radius_gauge_{radius:.0}mm_inner_opening"),
            radius - 6.0,
            6.0,
            64,
        )
        .translate(
            STRAIN_CENTER.0 - STRAIN_PANEL_X / 2.0 + 58.0 + index as f64 * 92.0,
            STRAIN_CENTER.1 - STRAIN_PANEL_Y / 2.0 - 18.0,
            BASE_Z + 2.5,
        );
        gauges = gauges + (gauge - inner);
    }
    gauges
}

fn waste_retain_split_manifold() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_waste_retain_split_manifold_panel"),
        SPLIT_PANEL_X,
        SPLIT_PANEL_Y,
        SPLIT_PANEL_Z,
    )
    .translate(SPLIT_CENTER.0, SPLIT_CENTER.1, z_on_deck(SPLIT_PANEL_Z));
    let mut cuts = Part::empty(format!("{PREFIX}_split_manifold_port_cuts"));
    let mut features = Part::empty(format!("{PREFIX}_split_manifold_features"));

    for (index, stream) in SPLIT_STREAMS.iter().enumerate() {
        let y = SPLIT_CENTER.1 + centered_index(index, SPLIT_STREAMS.len(), 64.0);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_{stream}_split_port_cut"),
                SPLIT_PORT_D / 2.0,
                SPLIT_PANEL_Z + 8.0,
                36,
            )
            .translate(SPLIT_CENTER.0 - 50.0, y, z_on_deck(SPLIT_PANEL_Z));
        features = features
            + centered_cube(format!("{PREFIX}_{stream}_bag_tail_nest"), 92.0, 34.0, 12.0)
                .translate(SPLIT_CENTER.0 + 36.0, y, BASE_Z + SPLIT_PANEL_Z + 6.0)
            + centered_cube(
                format!("{PREFIX}_{stream}_custody_label_land"),
                78.0,
                16.0,
                4.0,
            )
            .translate(
                SPLIT_CENTER.0 + 36.0,
                y + 30.0,
                BASE_Z + SPLIT_PANEL_Z + 2.0,
            );
    }

    for index in 0..SPLIT_DIVERTER_TOKENS {
        features = features
            + centered_cube(
                format!("{PREFIX}_waste_retain_diverter_token_socket_{index}"),
                38.0,
                24.0,
                5.0,
            )
            .translate(
                SPLIT_CENTER.0 - 54.0 + index as f64 * 52.0,
                SPLIT_CENTER.1,
                BASE_Z + SPLIT_PANEL_Z + 2.5,
            );
    }

    panel - cuts + features + split_y_trace()
}

fn split_y_trace() -> Part {
    let inlet = centered_cube(format!("{PREFIX}_split_common_inlet_trace"), 78.0, 7.0, 5.0)
        .translate(
            SPLIT_CENTER.0 - 88.0,
            SPLIT_CENTER.1,
            BASE_Z + SPLIT_PANEL_Z + 2.5,
        );
    let retain = centered_cube(
        format!("{PREFIX}_split_retain_branch_trace"),
        82.0,
        7.0,
        5.0,
    )
    .rotate(0.0, 0.0, 24.0)
    .translate(
        SPLIT_CENTER.0 - 30.0,
        SPLIT_CENTER.1 + 25.0,
        BASE_Z + SPLIT_PANEL_Z + 2.5,
    );
    let waste = centered_cube(format!("{PREFIX}_split_waste_branch_trace"), 82.0, 7.0, 5.0)
        .rotate(0.0, 0.0, -24.0)
        .translate(
            SPLIT_CENTER.0 - 30.0,
            SPLIT_CENTER.1 - 25.0,
            BASE_Z + SPLIT_PANEL_Z + 2.5,
        );
    inlet + retain + waste
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_panel"),
        DECISION_PANEL_X,
        DECISION_PANEL_Y,
        DECISION_PANEL_Z,
    )
    .translate(
        DECISION_CENTER.0,
        DECISION_CENTER.1,
        z_on_deck(DECISION_PANEL_Z),
    );
    let mut lanes = Part::empty(format!("{PREFIX}_release_hold_reject_lane_features"));
    let mut cuts = Part::empty(format!("{PREFIX}_release_hold_reject_lane_token_cuts"));

    for (lane_index, lane) in DECISION_LANES.iter().enumerate() {
        let x =
            DECISION_CENTER.0 + centered_index(lane_index, DECISION_LANES.len(), DECISION_PITCH_X);
        lanes = lanes
            + centered_cube(
                format!("{PREFIX}_{lane}_decision_lane_raised_border"),
                DECISION_LANE_X,
                DECISION_LANE_Y,
                8.0,
            )
            .translate(x, DECISION_CENTER.1, BASE_Z + DECISION_PANEL_Z + 4.0)
            - centered_cube(
                format!("{PREFIX}_{lane}_decision_lane_open_center"),
                DECISION_LANE_X - 18.0,
                DECISION_LANE_Y - 18.0,
                9.0,
            )
            .translate(x, DECISION_CENTER.1, BASE_Z + DECISION_PANEL_Z + 4.0);

        for slot in 0..4 {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_{lane}_decision_token_slot_{slot}"),
                    30.0,
                    18.0,
                    DECISION_PANEL_Z + 8.0,
                )
                .translate(
                    x - 54.0 + slot as f64 * 36.0,
                    DECISION_CENTER.1,
                    z_on_deck(DECISION_PANEL_Z),
                );
        }
    }

    panel - cuts + lanes
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_evidence_bridge_camera_beam"),
        BRIDGE_SPAN_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut evidence = Part::empty(format!("{PREFIX}_evidence_camera_light_mounts"));
    for camera in 0..CAMERA_PODS {
        let x = centered_index(camera, CAMERA_PODS, 240.0);
        evidence = evidence
            + centered_cube(
                format!("{PREFIX}_evidence_camera_pod_{camera}"),
                84.0,
                30.0,
                18.0,
            )
            .translate(x, -BRIDGE_Y / 2.0 - 18.0, BRIDGE_UNDERSIDE_Z + 12.0)
            + centered_cylinder(
                format!("{PREFIX}_evidence_camera_lens_clearance_{camera}"),
                8.0,
                20.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -BRIDGE_Y / 2.0 - 30.0, BRIDGE_UNDERSIDE_Z + 12.0);
    }

    for bar in 0..LIGHT_BARS {
        let x = centered_index(bar, LIGHT_BARS, 300.0);
        evidence = evidence
            + centered_cube(
                format!("{PREFIX}_evidence_light_bar_{bar}"),
                190.0,
                10.0,
                8.0,
            )
            .translate(x, BRIDGE_Y / 2.0 + 10.0, BRIDGE_UNDERSIDE_Z + 7.0);
    }

    left_post + right_post + beam + evidence
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_keepout_rail"),
        STATION_X - 120.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - ROBOT_FRONT_CLEARANCE / 2.0,
        KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_service_keepout_rail"),
        STATION_X - 120.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + SERVICE_REAR_CLEARANCE / 2.0,
        KEEP_OUT_Z / 2.0,
    );
    let left = centered_cube(
        format!("{PREFIX}_left_concentrator_service_keepout_rail"),
        KEEP_OUT_RAIL_W,
        STATION_Y - 80.0,
        KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CONCENTRATOR_SERVICE_CLEARANCE / 2.0,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_bag_service_keepout_rail"),
        KEEP_OUT_RAIL_W,
        STATION_Y - 80.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_BAG_SERVICE_CLEARANCE / 2.0,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let overhead = centered_cube(
        format!("{PREFIX}_overhead_robot_camera_service_keepout_gauge"),
        STATION_X - 180.0,
        14.0,
        10.0,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z);

    front + rear + left + right + overhead
}

fn closed_fluid_route_overlays() -> Part {
    let mut routes = Part::empty(format!("{PREFIX}_closed_fluid_route_overlays"));
    routes = routes
        + route_segment(
            "bag_to_bought_unit_inlet_route",
            WASH_CENTER.0 + WASH_X / 2.0,
            WASH_CENTER.1,
            BAG_CENTER.0 - BAG_PANEL_X / 2.0,
            BAG_CENTER.1,
        )
        + route_segment(
            "bought_unit_to_adapter_route",
            WASH_CENTER.0,
            WASH_CENTER.1 - WASH_Y / 2.0,
            ADAPTER_CENTER.0,
            ADAPTER_CENTER.1 + ADAPTER_PANEL_Y / 2.0,
        )
        + route_segment(
            "adapter_to_tap_route",
            ADAPTER_CENTER.0 + ADAPTER_PANEL_X / 2.0,
            ADAPTER_CENTER.1,
            TAP_CENTER.0 - TAP_PANEL_X / 2.0,
            TAP_CENTER.1,
        )
        + route_segment(
            "tap_to_witness_route",
            TAP_CENTER.0 - TAP_PANEL_X / 2.0,
            TAP_CENTER.1 - 38.0,
            WITNESS_CENTER.0 + WITNESS_PANEL_X / 2.0,
            WITNESS_CENTER.1 - 38.0,
        )
        + route_segment(
            "witness_to_capture_route",
            WITNESS_CENTER.0,
            WITNESS_CENTER.1 - WITNESS_PANEL_Y / 2.0,
            CAPTURE_CENTER.0,
            CAPTURE_CENTER.1 + CAPTURE_RACK_Y / 2.0,
        )
        + route_segment(
            "capture_to_strain_route",
            CAPTURE_CENTER.0 + CAPTURE_RACK_X / 2.0,
            CAPTURE_CENTER.1,
            STRAIN_CENTER.0 - STRAIN_PANEL_X / 2.0,
            STRAIN_CENTER.1,
        )
        + route_segment(
            "strain_to_split_route",
            STRAIN_CENTER.0 + STRAIN_PANEL_X / 2.0,
            STRAIN_CENTER.1,
            SPLIT_CENTER.0 - SPLIT_PANEL_X / 2.0,
            SPLIT_CENTER.1,
        );
    routes
}

fn route_segment(name: &str, x1: f64, y1: f64, x2: f64, y2: f64) -> Part {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx).to_degrees();
    centered_cube(format!("{PREFIX}_{name}"), length, 6.0, 4.0)
        .rotate(0.0, 0.0, angle)
        .translate((x1 + x2) / 2.0, (y1 + y2) / 2.0, BASE_Z + 5.0)
}

fn bag_nest_center(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (
        BAG_CENTER.0 + centered_index(col, 2, BAG_PITCH_X),
        BAG_CENTER.1 + centered_index(row, 2, BAG_PITCH_Y),
    )
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 72.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 72.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 72.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 72.0),
        (-300.0, -STATION_Y / 2.0 + 58.0),
        (300.0, -STATION_Y / 2.0 + 58.0),
        (-300.0, STATION_Y / 2.0 - 58.0),
        (300.0, STATION_Y / 2.0 - 58.0),
    ]
}

fn datum_points() -> [(f64, f64); ROBOT_DATUMS] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 56.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 56.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 56.0),
        (-250.0, STATION_Y / 2.0 - 58.0),
        (250.0, STATION_Y / 2.0 - 58.0),
        (-250.0, -STATION_Y / 2.0 + 58.0),
        (250.0, -STATION_Y / 2.0 + 58.0),
    ]
}

fn fiducial(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 12.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 25.0, 3.0, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 25.0, 5.0);
    let center = centered_cylinder(format!("{name}_center_bore"), 3.0, 6.0, 20);
    disc + cross_x + cross_y - center
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn z_on_deck(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_paths_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cell_wash_concentrate_interface_volume_recovery_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_interface_features_are_represented() {
        for feature in [
            "bought_wash_concentrator_envelope",
            "inlet_outlet_bag_nests",
            "recovery_volume_witness_wells",
            "low_dead_volume_adapter_coupons",
            "pressure_flow_taps",
            "cell_loss_surrogate_capture_rack",
            "waste_retain_split_manifold",
            "release_hold_reject_lanes",
            "evidence_bridge",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_contract_has_clear_module_footprints() {
        assert_layout();
        for rect in module_rects() {
            assert!(rect.fits_inside_station(24.0), "{}", rect.name);
        }
    }

    #[test]
    fn bag_roles_and_split_streams_cover_closed_process_paths() {
        assert_eq!(
            BAG_ROLES,
            [
                "wash_inlet",
                "buffer_inlet",
                "retain_outlet",
                "waste_outlet"
            ]
        );
        assert_eq!(SPLIT_STREAMS, ["retain", "waste"]);
        assert_eq!(BAG_NESTS, 4);
        assert_eq!(SPLIT_DIVERTER_TOKENS, 2);
        assert!(WASTE_BAG_VOLUME_ML > RETAIN_BAG_VOLUME_ML);
    }

    #[test]
    fn low_dead_volume_and_witness_counts_match_process_lanes() {
        assert_eq!(PROCESS_LANES, 6);
        assert_eq!(ADAPTER_COUPONS, PROCESS_LANES * 2);
        assert!(ADAPTER_HOLDUP_UL <= 8.0);
        assert!(ADAPTER_BORE_D <= 1.4);
        assert_eq!(PRESSURE_TAPS, PROCESS_LANES * 2);
        assert_eq!(FLOW_TAPS, PROCESS_LANES);
        assert_eq!(WITNESS_WELLS, 12);
        assert!(WITNESS_VOLUME_UL_PER_STEP >= 50.0);
    }

    #[test]
    fn surrogate_capture_and_tubing_strain_relief_scale_with_lanes() {
        assert_eq!(CAPTURE_POSITIONS, PROCESS_LANES * CAPTURE_ROWS);
        assert_eq!(CAPTURE_ROWS, 3);
        assert_eq!(STRAIN_RELIEF_SLOTS, PROCESS_LANES * 2);
        assert!(MIN_TUBE_BEND_RADIUS >= 40.0);
    }

    #[test]
    fn disposition_and_automation_clearances_are_explicit() {
        assert_eq!(DECISION_LANES, ["release", "hold", "reject"]);
        assert_eq!(DECISION_TOKEN_SLOTS, 12);
        assert_eq!(CAMERA_PODS, 5);
        assert_eq!(LIGHT_BARS, 4);
        assert!(BRIDGE_UNDERSIDE_Z > BASE_Z + WASH_Z + 100.0);
        assert!(ROBOT_FRONT_CLEARANCE >= 360.0);
        assert!(SERVICE_REAR_CLEARANCE >= 220.0);
        assert!(LEFT_CONCENTRATOR_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_BAG_SERVICE_CLEARANCE >= 200.0);
    }
}
