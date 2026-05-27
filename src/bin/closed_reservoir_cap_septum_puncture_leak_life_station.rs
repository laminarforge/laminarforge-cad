use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reservoir cap/septum puncture leak-life validation station.
//
// Intent:
// - Exercise reservoir cap coupons and septa through repeated puncture cycles
//   while preserving a closed-system validation layout.
// - Keep cap nests, puncture guides, septum compression witnesses, pressure
//   hold/leak ports, dye ingress wells, needle/connector parking, traceability,
//   disposition lanes, evidence capture, and robot/service keepouts physically
//   legible in separate fixture zones.
// - This is concept/interface CAD only. It is not a sterile barrier design,
//   a septum material acceptance criterion, a pressure-rated device, or a
//   validated leak-life procedure.

const OUTPUTS: [&str; 12] = [
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_base_containment_deck.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_reservoir_cap_coupon_nests.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_repeated_puncture_guide_bridge.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_septum_compression_witness_pockets.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_pressure_hold_leak_ports.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_dye_ingress_wells.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_needle_connector_parking.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_barcode_certificate_lands.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_release_hold_reject_lanes.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_evidence_bridge.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_robot_service_keepout_gauges.stl",
    "output/closed_reservoir_cap_septum_puncture_leak_life_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "reservoir_cap_coupon_nests",
    "repeated_puncture_guide_bridge",
    "septum_compression_witness_pockets",
    "pressure_hold_leak_ports",
    "dye_ingress_wells",
    "needle_connector_parking",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 860.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 14.0;

const CAP_POS: (f64, f64) = (-390.0, 198.0);
const CAP_X: f64 = 400.0;
const CAP_Y: f64 = 250.0;
const CAP_Z: f64 = 50.0;
const CAP_ROWS: usize = 3;
const CAP_COLS: usize = 4;
const CAP_COUNT: usize = CAP_ROWS * CAP_COLS;
const CAP_PITCH_X: f64 = 88.0;
const CAP_PITCH_Y: f64 = 72.0;
const CAP_POCKET_D: f64 = 62.0;
const CAP_THREAD_D: f64 = 42.0;
const CAP_POCKET_DEPTH: f64 = 28.0;
const CAP_ORIENTATION_KEYS: usize = CAP_COUNT;

const GUIDE_POS: (f64, f64) = (70.0, 198.0);
const GUIDE_X: f64 = 420.0;
const GUIDE_Y: f64 = 250.0;
const GUIDE_BASE_Z: f64 = 18.0;
const GUIDE_POST_Z: f64 = 174.0;
const GUIDE_BEAM_Z: f64 = 22.0;
const GUIDE_BUSHING_OD: f64 = 20.0;
const GUIDE_BUSHING_ID: f64 = 7.5;
const GUIDE_ROW_RAIL_Y: f64 = 30.0;
const PUNCTURE_CYCLE_TICKS: usize = 9;

const COMP_POS: (f64, f64) = (455.0, 198.0);
const COMP_X: f64 = 300.0;
const COMP_Y: f64 = 250.0;
const COMP_Z: f64 = 52.0;
const COMP_ROWS: usize = 3;
const COMP_COLS: usize = 4;
const COMP_COUNT: usize = COMP_ROWS * COMP_COLS;
const COMP_PITCH_X: f64 = 60.0;
const COMP_PITCH_Y: f64 = 66.0;
const COMP_WITNESS_D: f64 = 34.0;
const COMP_PLATEN_D: f64 = 48.0;
const COMP_STEP_COUNT: usize = 6;

const PRESSURE_POS: (f64, f64) = (-390.0, -78.0);
const PRESSURE_X: f64 = 420.0;
const PRESSURE_Y: f64 = 150.0;
const PRESSURE_Z: f64 = 62.0;
const PRESSURE_CHANNELS: usize = CAP_COUNT;
const PRESSURE_COLS: usize = 6;
const PRESSURE_PORT_D: f64 = 9.0;
const PRESSURE_COLLAR_D: f64 = 24.0;
const LEAK_WITNESS_GROOVES: usize = 8;

const DYE_POS: (f64, f64) = (-40.0, -96.0);
const DYE_X: f64 = 280.0;
const DYE_Y: f64 = 150.0;
const DYE_Z: f64 = 46.0;
const DYE_ROWS: usize = 3;
const DYE_COLS: usize = 4;
const DYE_WELLS: usize = DYE_ROWS * DYE_COLS;
const DYE_WELL_D: f64 = 22.0;
const DYE_OVERFLOW_SLOTS: usize = 5;

const PARK_POS: (f64, f64) = (350.0, -88.0);
const PARK_X: f64 = 410.0;
const PARK_Y: f64 = 150.0;
const PARK_Z: f64 = 42.0;
const NEEDLE_PARKS: usize = CAP_COUNT;
const CONNECTOR_PARKS: usize = 8;
const NEEDLE_SLOT_X: f64 = 18.0;
const NEEDLE_SLOT_Y: f64 = 76.0;
const CONNECTOR_SOCKET_D: f64 = 18.0;
const SHARPS_GUARD_Z: f64 = 82.0;

const TRACE_POS: (f64, f64) = (-390.0, -305.0);
const TRACE_X: f64 = 390.0;
const TRACE_Y: f64 = 110.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 6;
const CERTIFICATE_LANDS: usize = 3;
const SIGNOFF_LANDS: usize = 2;

const STATUS_POS: (f64, f64) = (50.0, -305.0);
const STATUS_X: f64 = 380.0;
const STATUS_Y: f64 = 110.0;
const STATUS_Z: f64 = 40.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;

const EVIDENCE_POS: (f64, f64) = (430.0, -300.0);
const EVIDENCE_X: f64 = 280.0;
const EVIDENCE_Y: f64 = 120.0;
const EVIDENCE_POST_Z: f64 = 210.0;
const EVIDENCE_BEAM_Z: f64 = 22.0;
const CAMERA_CLEARANCE_Z: f64 = 150.0;
const EVIDENCE_TARGETS: usize = 5;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 210.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 146.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 110.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 280.0;
const PRESSURE_SERVICE_CLEARANCE_Z: f64 = 238.0;
const KEEP_OUT_GAUGES: usize = 6;
const KEEP_OUT_RAIL_Z: f64 = 8.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; STATUS_LANES] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 8.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 8.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let cap_nests = reservoir_cap_coupon_nests();
    export(OUTPUTS[1], &cap_nests);

    let puncture_bridge = repeated_puncture_guide_bridge();
    export(OUTPUTS[2], &puncture_bridge);

    let compression = septum_compression_witness_pockets();
    export(OUTPUTS[3], &compression);

    let pressure = pressure_hold_leak_ports();
    export(OUTPUTS[4], &pressure);

    let dye = dye_ingress_wells();
    export(OUTPUTS[5], &dye);

    let parking = needle_connector_parking();
    export(OUTPUTS[6], &parking);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[8], &status);

    let evidence = evidence_bridge();
    export(OUTPUTS[9], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + cap_nests.translate(CAP_POS.0, CAP_POS.1, deck_top_z())
        + puncture_bridge.translate(GUIDE_POS.0, GUIDE_POS.1, deck_top_z())
        + compression.translate(COMP_POS.0, COMP_POS.1, deck_top_z())
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, deck_top_z())
        + dye.translate(DYE_POS.0, DYE_POS.1, deck_top_z())
        + parking.translate(PARK_POS.0, PARK_POS.1, deck_top_z())
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, deck_top_z() + 1.0)
        + status.translate(STATUS_POS.0, STATUS_POS.1, deck_top_z())
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, deck_top_z())
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed reservoir cap/septum puncture leak-life validation station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Cap coupon capacity:       {CAP_COUNT} reservoir cap coupon nests in a {CAP_ROWS}x{CAP_COLS} grid"
    );
    println!(
        "  Puncture cycling:          bridge-aligned {CAP_COUNT} guide bushings, {PUNCTURE_CYCLE_TICKS} cycle witness ticks"
    );
    println!(
        "  Septum compression:        {COMP_COUNT} compression witness pockets and {COMP_STEP_COUNT} reference step coupons"
    );
    println!(
        "  Leak challenge:            {PRESSURE_CHANNELS} pressure-hold/leak ports and {DYE_WELLS} dye ingress wells"
    );
    println!(
        "  Handling and evidence:     {NEEDLE_PARKS} needle parks, {CONNECTOR_PARKS} connector parks, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Disposition and keepout:   release/hold/reject lanes, evidence bridge, {KEEP_OUT_GAUGES} robot/service keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    DECK_Z
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "reservoir_cap_septum_leak_life_base_containment_deck_floor",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "reservoir_cap_septum_leak_life_recessed_secondary_basin",
        STATION_X - 128.0,
        STATION_Y - 112.0,
        SOCKET_DEPTH + 2.0,
    )
    .translate(0.0, -8.0, DECK_Z - SOCKET_DEPTH / 2.0 + 0.5);
    let front_drain = centered_cylinder(
        "reservoir_cap_septum_leak_life_front_low_point_drain_cut",
        DRAIN_D / 2.0,
        RIM_W + 38.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 10.0, DECK_Z - 7.0);

    deck - basin - front_drain - mounting_slots() - module_socket_recesses()
        + perimeter_curbs()
        + deck_flow_ribs()
        + deck_fiducials()
        + zone_floor_markers()
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("reservoir_cap_septum_leak_life_mounting_slots");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("reservoir_cap_septum_leak_life_m6_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("reservoir_cap_septum_leak_life_m6_slot_relief_{index}"),
            30.0,
            MOUNT_HOLE_D + 0.6,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("reservoir_cap_septum_leak_life_module_socket_recesses");
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("reservoir_cap_septum_leak_life_{}_socket_recess", rect.name),
                rect.x + 16.0,
                rect.y + 16.0,
                SOCKET_DEPTH + 1.0,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.5,
            );
    }
    sockets
}

fn perimeter_curbs() -> Part {
    let front = centered_cube(
        "reservoir_cap_septum_leak_life_front_low_containment_lip",
        STATION_X - 164.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 28.0, DECK_Z + 12.0);
    let rear = centered_cube(
        "reservoir_cap_septum_leak_life_rear_containment_curb",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "reservoir_cap_septum_leak_life_left_containment_curb",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "reservoir_cap_septum_leak_life_right_containment_curb",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn deck_flow_ribs() -> Part {
    let mut ribs = Part::empty("reservoir_cap_septum_leak_life_deck_leak_flow_ribs");
    for i in 0..LEAK_WITNESS_GROOVES {
        let y = centered_index(i, LEAK_WITNESS_GROOVES, 48.0) - 28.0;
        ribs = ribs
            + centered_cube(
                format!("reservoir_cap_septum_leak_life_deck_flow_witness_rib_{i}"),
                STATION_X - 250.0,
                5.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn deck_fiducials() -> Part {
    let mut targets = Part::empty("reservoir_cap_septum_leak_life_deck_fiducials");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 100.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 100.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 100.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 100.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("reservoir_cap_septum_leak_life_deck_fiducial_boss_{i}"),
            18.0,
            6.0,
            36,
        )
        .translate(*x, *y, DECK_Z + 3.0);
        let center = centered_cylinder(
            format!("reservoir_cap_septum_leak_life_deck_fiducial_center_cut_{i}"),
            4.0,
            7.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        targets = targets + (boss - center);
    }
    targets
}

fn zone_floor_markers() -> Part {
    let mut markers = Part::empty("reservoir_cap_septum_leak_life_floor_zone_markers");
    for rect in module_rects() {
        markers = markers
            + centered_cube(
                format!("reservoir_cap_septum_leak_life_{}_floor_marker", rect.name),
                rect.x + 30.0,
                rect.y + 24.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z + 1.5);
    }
    markers
}

fn reservoir_cap_coupon_nests() -> Part {
    let body = centered_cube("reservoir_cap_septum_coupon_nest_body", CAP_X, CAP_Y, CAP_Z)
        .translate(0.0, 0.0, CAP_Z / 2.0);
    let rear_fence = centered_cube(
        "reservoir_cap_septum_coupon_nest_rear_label_fence",
        CAP_X,
        14.0,
        CAP_Z + 26.0,
    )
    .translate(0.0, CAP_Y / 2.0 - 7.0, CAP_Z / 2.0 + 13.0);
    let left_fence = centered_cube(
        "reservoir_cap_septum_coupon_nest_left_clean_side_fence",
        14.0,
        CAP_Y - 18.0,
        CAP_Z + 18.0,
    )
    .translate(-CAP_X / 2.0 + 7.0, 0.0, CAP_Z / 2.0 + 9.0);

    let mut cuts = Part::empty("reservoir_cap_septum_coupon_nest_pocket_cuts");
    let mut rims = Part::empty("reservoir_cap_septum_coupon_nest_capture_rims");
    let mut keys = Part::empty("reservoir_cap_septum_coupon_nest_orientation_keys");
    for row in 0..CAP_ROWS {
        for col in 0..CAP_COLS {
            let index = cap_index(row, col);
            let (x, y) = cap_xy(row, col);
            cuts = cuts
                + cap_pocket_cut(index, x, y)
                + centered_cube(
                    format!("reservoir_cap_septum_coupon_nest_{index}_finger_relief"),
                    22.0,
                    CAP_POCKET_D + 16.0,
                    CAP_POCKET_DEPTH + 2.0,
                )
                .translate(x, y - 4.0, CAP_Z - CAP_POCKET_DEPTH / 2.0 + 0.5);
            rims = rims + cap_retaining_ring(index, x, y);
            keys = keys
                + centered_cube(
                    format!("reservoir_cap_septum_coupon_nest_{index}_orientation_key_land"),
                    20.0,
                    6.0,
                    7.0,
                )
                .translate(x, y + CAP_POCKET_D / 2.0 + 12.0, CAP_Z + 3.5);
        }
    }

    body + rear_fence + left_fence + rims + keys - cuts + cap_nest_index_tabs()
}

fn cap_pocket_cut(index: usize, x: f64, y: f64) -> Part {
    let cap_pocket = centered_cylinder(
        format!("reservoir_cap_septum_coupon_nest_{index}_cap_pocket"),
        CAP_POCKET_D / 2.0,
        CAP_POCKET_DEPTH + 1.0,
        64,
    )
    .translate(x, y, CAP_Z - CAP_POCKET_DEPTH / 2.0 + 0.5);
    let thread_relief = centered_cylinder(
        format!("reservoir_cap_septum_coupon_nest_{index}_thread_relief"),
        CAP_THREAD_D / 2.0,
        CAP_POCKET_DEPTH + 9.0,
        48,
    )
    .translate(x, y, CAP_Z - CAP_POCKET_DEPTH / 2.0 - 3.0);
    cap_pocket + thread_relief
}

fn cap_retaining_ring(index: usize, x: f64, y: f64) -> Part {
    let outer = centered_cylinder(
        format!("reservoir_cap_septum_coupon_nest_{index}_retaining_ring_outer"),
        CAP_POCKET_D / 2.0 + 5.0,
        5.0,
        64,
    )
    .translate(x, y, CAP_Z + 2.5);
    let inner = centered_cylinder(
        format!("reservoir_cap_septum_coupon_nest_{index}_retaining_ring_inner_cut"),
        CAP_POCKET_D / 2.0 + 0.8,
        6.0,
        64,
    )
    .translate(x, y, CAP_Z + 3.0);
    outer - inner
}

fn cap_nest_index_tabs() -> Part {
    let mut tabs = Part::empty("reservoir_cap_septum_coupon_nest_index_tabs");
    for row in 0..CAP_ROWS {
        for col in 0..CAP_COLS {
            let index = cap_index(row, col);
            let (x, y) = cap_xy(row, col);
            tabs = tabs
                + centered_cube(
                    format!("reservoir_cap_septum_coupon_nest_{index}_cycle_id_tab"),
                    24.0,
                    7.0,
                    5.0,
                )
                .translate(x, y - CAP_POCKET_D / 2.0 - 14.0, CAP_Z + 2.5);
        }
    }
    tabs
}

fn repeated_puncture_guide_bridge() -> Part {
    let base = centered_cube(
        "reservoir_cap_septum_repeated_puncture_guide_base_plate",
        GUIDE_X,
        GUIDE_Y,
        GUIDE_BASE_Z,
    )
    .translate(0.0, 0.0, GUIDE_BASE_Z / 2.0);
    let mut guide_hole_shadows =
        Part::empty("reservoir_cap_septum_repeated_puncture_guide_hole_shadows");
    for row in 0..CAP_ROWS {
        for col in 0..CAP_COLS {
            let index = cap_index(row, col);
            let (x, y) = cap_xy(row, col);
            guide_hole_shadows = guide_hole_shadows
                + centered_cylinder(
                    format!("reservoir_cap_septum_puncture_guide_{index}_base_alignment_cut"),
                    GUIDE_BUSHING_ID / 2.0 + 2.0,
                    GUIDE_BASE_Z + 2.0,
                    28,
                )
                .translate(x, y, GUIDE_BASE_Z / 2.0);
        }
    }

    (base - guide_hole_shadows)
        + guide_bridge_posts()
        + guide_bridge_frame()
        + guide_bushing_array()
        + cycle_witness_scale()
}

fn guide_bridge_posts() -> Part {
    let mut posts = Part::empty("reservoir_cap_septum_puncture_bridge_posts");
    for (i, (x, y)) in [
        (-GUIDE_X / 2.0 + 28.0, -GUIDE_Y / 2.0 + 26.0),
        (GUIDE_X / 2.0 - 28.0, -GUIDE_Y / 2.0 + 26.0),
        (-GUIDE_X / 2.0 + 28.0, GUIDE_Y / 2.0 - 26.0),
        (GUIDE_X / 2.0 - 28.0, GUIDE_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("reservoir_cap_septum_puncture_bridge_post_{i}"),
                24.0,
                24.0,
                GUIDE_POST_Z,
            )
            .translate(*x, *y, GUIDE_BASE_Z + GUIDE_POST_Z / 2.0);
    }
    posts
}

fn guide_bridge_frame() -> Part {
    let front = centered_cube(
        "reservoir_cap_septum_puncture_bridge_front_beam",
        GUIDE_X - 34.0,
        22.0,
        GUIDE_BEAM_Z,
    )
    .translate(
        0.0,
        -GUIDE_Y / 2.0 + 26.0,
        GUIDE_BASE_Z + GUIDE_POST_Z - GUIDE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "reservoir_cap_septum_puncture_bridge_rear_beam",
        GUIDE_X - 34.0,
        22.0,
        GUIDE_BEAM_Z,
    )
    .translate(
        0.0,
        GUIDE_Y / 2.0 - 26.0,
        GUIDE_BASE_Z + GUIDE_POST_Z - GUIDE_BEAM_Z / 2.0,
    );
    let mut row_rails = Part::empty("reservoir_cap_septum_puncture_bridge_row_rails");
    for row in 0..CAP_ROWS {
        let y = centered_index(row, CAP_ROWS, CAP_PITCH_Y);
        row_rails = row_rails
            + centered_cube(
                format!("reservoir_cap_septum_puncture_bridge_row_{row}_guide_rail"),
                GUIDE_X - 90.0,
                GUIDE_ROW_RAIL_Y,
                GUIDE_BEAM_Z,
            )
            .translate(0.0, y, GUIDE_BASE_Z + GUIDE_POST_Z - GUIDE_BEAM_Z / 2.0);
    }
    front + rear + row_rails
}

fn guide_bushing_array() -> Part {
    let mut bushings = Part::empty("reservoir_cap_septum_puncture_guide_bushing_array");
    for row in 0..CAP_ROWS {
        for col in 0..CAP_COLS {
            let index = cap_index(row, col);
            let (x, y) = cap_xy(row, col);
            let outer = centered_cylinder(
                format!("reservoir_cap_septum_puncture_guide_bushing_{index}_outer"),
                GUIDE_BUSHING_OD / 2.0,
                GUIDE_BEAM_Z + 8.0,
                44,
            )
            .translate(x, y, GUIDE_BASE_Z + GUIDE_POST_Z - GUIDE_BEAM_Z / 2.0);
            let inner = centered_cylinder(
                format!("reservoir_cap_septum_puncture_guide_bushing_{index}_needle_cut"),
                GUIDE_BUSHING_ID / 2.0,
                GUIDE_BEAM_Z + 10.0,
                32,
            )
            .translate(x, y, GUIDE_BASE_Z + GUIDE_POST_Z - GUIDE_BEAM_Z / 2.0);
            bushings = bushings + (outer - inner);
        }
    }
    bushings
}

fn cycle_witness_scale() -> Part {
    let mut ticks = Part::empty("reservoir_cap_septum_puncture_cycle_witness_scale");
    for i in 0..PUNCTURE_CYCLE_TICKS {
        let x = centered_index(i, PUNCTURE_CYCLE_TICKS, 32.0);
        let tick_x = if i % 3 == 0 { 22.0 } else { 12.0 };
        ticks = ticks
            + centered_cube(
                format!("reservoir_cap_septum_puncture_cycle_witness_tick_{i}"),
                tick_x,
                5.0,
                7.0,
            )
            .translate(x, -GUIDE_Y / 2.0 + 58.0, GUIDE_BASE_Z + 3.5);
    }
    ticks
}

fn septum_compression_witness_pockets() -> Part {
    let body = centered_cube(
        "reservoir_cap_septum_compression_witness_block",
        COMP_X,
        COMP_Y,
        COMP_Z,
    )
    .translate(0.0, 0.0, COMP_Z / 2.0);
    let mut cuts = Part::empty("reservoir_cap_septum_compression_witness_pocket_cuts");
    let mut platen_rims = Part::empty("reservoir_cap_septum_compression_witness_platen_rims");
    for row in 0..COMP_ROWS {
        for col in 0..COMP_COLS {
            let index = row * COMP_COLS + col;
            let (x, y) = comp_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("reservoir_cap_septum_compression_witness_{index}_pocket_cut"),
                    COMP_WITNESS_D / 2.0,
                    22.0,
                    44,
                )
                .translate(x, y, COMP_Z - 10.5);
            let outer = centered_cylinder(
                format!("reservoir_cap_septum_compression_witness_{index}_platen_outer"),
                COMP_PLATEN_D / 2.0,
                5.0,
                52,
            )
            .translate(x, y, COMP_Z + 2.5);
            let inner = centered_cylinder(
                format!("reservoir_cap_septum_compression_witness_{index}_platen_inner_cut"),
                COMP_WITNESS_D / 2.0 + 1.0,
                6.0,
                44,
            )
            .translate(x, y, COMP_Z + 3.0);
            platen_rims = platen_rims + (outer - inner);
        }
    }

    body - cuts + platen_rims + compression_reference_steps() + compression_witness_gauge_tabs()
}

fn compression_reference_steps() -> Part {
    let mut steps = Part::empty("reservoir_cap_septum_compression_reference_steps");
    for i in 0..COMP_STEP_COUNT {
        steps = steps
            + centered_cube(
                format!("reservoir_cap_septum_compression_reference_step_{i}"),
                22.0,
                34.0,
                2.0 + i as f64,
            )
            .translate(
                -COMP_X / 2.0 + 42.0 + i as f64 * 30.0,
                -COMP_Y / 2.0 + 28.0,
                COMP_Z + (2.0 + i as f64) / 2.0,
            );
    }
    steps
}

fn compression_witness_gauge_tabs() -> Part {
    let mut tabs = Part::empty("reservoir_cap_septum_compression_witness_gauge_tabs");
    for i in 0..COMP_ROWS {
        tabs = tabs
            + centered_cube(
                format!("reservoir_cap_septum_compression_witness_row_{i}_gauge_tab"),
                56.0,
                6.0,
                7.0,
            )
            .translate(
                COMP_X / 2.0 - 45.0,
                centered_index(i, COMP_ROWS, COMP_PITCH_Y),
                COMP_Z + 3.5,
            );
    }
    tabs
}

fn pressure_hold_leak_ports() -> Part {
    let body = centered_cube(
        "reservoir_cap_septum_pressure_hold_leak_port_block",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);
    let rear_bulkhead = centered_cube(
        "reservoir_cap_septum_pressure_hold_rear_bulkhead_plate",
        PRESSURE_X,
        18.0,
        PRESSURE_Z + 46.0,
    )
    .translate(0.0, PRESSURE_Y / 2.0 - 9.0, PRESSURE_Z / 2.0 + 23.0);
    let mut cuts = Part::empty("reservoir_cap_septum_pressure_hold_port_cuts");
    let mut collars = Part::empty("reservoir_cap_septum_pressure_hold_port_collars");
    for channel in 0..PRESSURE_CHANNELS {
        let (x, y) = pressure_port_xy(channel);
        cuts = cuts
            + centered_cylinder(
                format!("reservoir_cap_septum_pressure_hold_port_{channel}_through_cut"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_Z + 48.0,
                32,
            )
            .translate(x, y, PRESSURE_Z / 2.0 + 8.0);
        let outer = centered_cylinder(
            format!("reservoir_cap_septum_pressure_hold_port_{channel}_collar_outer"),
            PRESSURE_COLLAR_D / 2.0,
            8.0,
            40,
        )
        .translate(x, y, PRESSURE_Z + 4.0);
        let inner = centered_cylinder(
            format!("reservoir_cap_septum_pressure_hold_port_{channel}_collar_inner_cut"),
            PRESSURE_PORT_D / 2.0 + 0.8,
            9.0,
            32,
        )
        .translate(x, y, PRESSURE_Z + 4.5);
        collars = collars + (outer - inner);
    }

    body + rear_bulkhead + collars - cuts + pressure_leak_witness_lane() + pressure_channel_keying()
}

fn pressure_leak_witness_lane() -> Part {
    let mut lane = Part::empty("reservoir_cap_septum_pressure_leak_witness_lane");
    for i in 0..PRESSURE_CHANNELS {
        let (x, _) = pressure_port_xy(i);
        lane = lane
            + centered_cube(
                format!("reservoir_cap_septum_pressure_leak_witness_channel_{i}"),
                28.0,
                6.0,
                6.0,
            )
            .translate(x, -PRESSURE_Y / 2.0 + 24.0, PRESSURE_Z + 3.0);
    }
    lane
}

fn pressure_channel_keying() -> Part {
    let mut keys = Part::empty("reservoir_cap_septum_pressure_channel_keying_tabs");
    for row in 0..2 {
        keys = keys
            + centered_cube(
                format!("reservoir_cap_septum_pressure_row_{row}_manifold_key_land"),
                PRESSURE_X - 70.0,
                6.0,
                8.0,
            )
            .translate(0.0, centered_index(row, 2, 52.0), PRESSURE_Z / 2.0 + 4.0);
    }
    keys
}

fn dye_ingress_wells() -> Part {
    let body = centered_cube(
        "reservoir_cap_septum_dye_ingress_well_block",
        DYE_X,
        DYE_Y,
        DYE_Z,
    )
    .translate(0.0, 0.0, DYE_Z / 2.0);
    let front_lip = centered_cube(
        "reservoir_cap_septum_dye_ingress_front_spill_lip",
        DYE_X,
        12.0,
        DYE_Z + 18.0,
    )
    .translate(0.0, -DYE_Y / 2.0 + 6.0, DYE_Z / 2.0 + 9.0);

    let mut cuts = Part::empty("reservoir_cap_septum_dye_ingress_well_cuts");
    let mut well_rims = Part::empty("reservoir_cap_septum_dye_ingress_well_rims");
    for row in 0..DYE_ROWS {
        for col in 0..DYE_COLS {
            let index = row * DYE_COLS + col;
            let (x, y) = dye_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("reservoir_cap_septum_dye_ingress_well_{index}_cut"),
                    DYE_WELL_D / 2.0,
                    28.0,
                    40,
                )
                .translate(x, y, DYE_Z - 13.5);
            let outer = centered_cylinder(
                format!("reservoir_cap_septum_dye_ingress_well_{index}_rim_outer"),
                DYE_WELL_D / 2.0 + 3.0,
                4.0,
                40,
            )
            .translate(x, y, DYE_Z + 2.0);
            let inner = centered_cylinder(
                format!("reservoir_cap_septum_dye_ingress_well_{index}_rim_inner_cut"),
                DYE_WELL_D / 2.0 + 0.6,
                5.0,
                40,
            )
            .translate(x, y, DYE_Z + 2.5);
            well_rims = well_rims + (outer - inner);
        }
    }

    body + front_lip + well_rims - cuts + dye_overflow_slots() + dye_lot_witness_tabs()
}

fn dye_overflow_slots() -> Part {
    let mut slots = Part::empty("reservoir_cap_septum_dye_ingress_overflow_slots");
    for i in 0..DYE_OVERFLOW_SLOTS {
        slots = slots
            + centered_cube(
                format!("reservoir_cap_septum_dye_ingress_overflow_slot_{i}"),
                28.0,
                6.0,
                6.0,
            )
            .translate(
                centered_index(i, DYE_OVERFLOW_SLOTS, 42.0),
                DYE_Y / 2.0 - 24.0,
                DYE_Z + 3.0,
            );
    }
    slots
}

fn dye_lot_witness_tabs() -> Part {
    let mut tabs = Part::empty("reservoir_cap_septum_dye_lot_witness_tabs");
    for i in 0..DYE_ROWS {
        tabs = tabs
            + centered_cube(
                format!("reservoir_cap_septum_dye_lot_row_{i}_witness_tab"),
                46.0,
                5.0,
                5.0,
            )
            .translate(
                -DYE_X / 2.0 + 36.0,
                centered_index(i, DYE_ROWS, 42.0),
                DYE_Z + 2.5,
            );
    }
    tabs
}

fn needle_connector_parking() -> Part {
    let body = centered_cube(
        "reservoir_cap_septum_needle_connector_parking_block",
        PARK_X,
        PARK_Y,
        PARK_Z,
    )
    .translate(0.0, 0.0, PARK_Z / 2.0);
    let rear_sharps_guard = centered_cube(
        "reservoir_cap_septum_needle_parking_rear_sharps_guard",
        PARK_X,
        14.0,
        SHARPS_GUARD_Z,
    )
    .translate(0.0, PARK_Y / 2.0 - 7.0, PARK_Z + SHARPS_GUARD_Z / 2.0);
    let mut cuts = Part::empty("reservoir_cap_septum_needle_connector_parking_cuts");
    for i in 0..NEEDLE_PARKS {
        cuts = cuts
            + centered_cube(
                format!("reservoir_cap_septum_needle_park_{i}_slot_cut"),
                NEEDLE_SLOT_X,
                NEEDLE_SLOT_Y,
                18.0,
            )
            .translate(needle_slot_x(i), -22.0, PARK_Z - 8.5);
    }
    for i in 0..CONNECTOR_PARKS {
        cuts = cuts
            + centered_cylinder(
                format!("reservoir_cap_septum_connector_park_{i}_socket_cut"),
                CONNECTOR_SOCKET_D / 2.0,
                20.0,
                36,
            )
            .translate(connector_socket_x(i), PARK_Y / 2.0 - 36.0, PARK_Z - 9.5);
    }

    body + rear_sharps_guard - cuts + needle_parking_clip_lands() + connector_parking_collars()
}

fn needle_parking_clip_lands() -> Part {
    let mut clips = Part::empty("reservoir_cap_septum_needle_parking_clip_lands");
    for i in 0..NEEDLE_PARKS {
        clips = clips
            + centered_cube(
                format!("reservoir_cap_septum_needle_park_{i}_front_clip_land"),
                NEEDLE_SLOT_X + 10.0,
                7.0,
                7.0,
            )
            .translate(needle_slot_x(i), -PARK_Y / 2.0 + 24.0, PARK_Z + 3.5);
    }
    clips
}

fn connector_parking_collars() -> Part {
    let mut collars = Part::empty("reservoir_cap_septum_connector_parking_collars");
    for i in 0..CONNECTOR_PARKS {
        let x = connector_socket_x(i);
        let y = PARK_Y / 2.0 - 36.0;
        let outer = centered_cylinder(
            format!("reservoir_cap_septum_connector_park_{i}_collar_outer"),
            CONNECTOR_SOCKET_D / 2.0 + 4.0,
            5.0,
            40,
        )
        .translate(x, y, PARK_Z + 2.5);
        let inner = centered_cylinder(
            format!("reservoir_cap_septum_connector_park_{i}_collar_inner_cut"),
            CONNECTOR_SOCKET_D / 2.0 + 0.6,
            6.0,
            36,
        )
        .translate(x, y, PARK_Z + 3.0);
        collars = collars + (outer - inner);
    }
    collars
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "reservoir_cap_septum_barcode_certificate_land_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);
    plate + barcode_lands() + certificate_lands() + signoff_lands() + traceability_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("reservoir_cap_septum_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i, BARCODE_LANDS, 54.0) - 8.0;
        lands = lands
            + centered_cube(
                format!("reservoir_cap_septum_barcode_land_{i}"),
                38.0,
                14.0,
                3.0,
            )
            .translate(x, TRACE_Y / 2.0 - 24.0, TRACE_Z + 1.5)
            + barcode_stripes(i, x);
    }
    lands
}

fn barcode_stripes(index: usize, x: f64) -> Part {
    let mut stripes = Part::empty(format!("reservoir_cap_septum_barcode_{index}_stripes"));
    for stripe in 0..4 {
        stripes = stripes
            + centered_cube(
                format!("reservoir_cap_septum_barcode_{index}_stripe_{stripe}"),
                2.0 + stripe as f64,
                16.0,
                3.8,
            )
            .translate(
                x - 12.0 + stripe as f64 * 8.0,
                TRACE_Y / 2.0 - 24.0,
                TRACE_Z + 1.9,
            );
    }
    stripes
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("reservoir_cap_septum_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("reservoir_cap_septum_certificate_land_{i}"),
                92.0,
                24.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LANDS, 112.0),
                4.0,
                TRACE_Z + 1.5,
            );
    }
    lands
}

fn signoff_lands() -> Part {
    let mut lands = Part::empty("reservoir_cap_septum_operator_signoff_lands");
    for i in 0..SIGNOFF_LANDS {
        lands = lands
            + centered_cube(
                format!("reservoir_cap_septum_operator_signoff_land_{i}"),
                126.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(i, SIGNOFF_LANDS, 156.0),
                -TRACE_Y / 2.0 + 24.0,
                TRACE_Z + 1.5,
            );
    }
    lands
}

fn traceability_fiducials() -> Part {
    let mut fiducials = Part::empty("reservoir_cap_septum_traceability_fiducials");
    for (i, (x, y)) in [
        (-TRACE_X / 2.0 + 24.0, -TRACE_Y / 2.0 + 22.0),
        (TRACE_X / 2.0 - 24.0, -TRACE_Y / 2.0 + 22.0),
        (-TRACE_X / 2.0 + 24.0, TRACE_Y / 2.0 - 22.0),
        (TRACE_X / 2.0 - 24.0, TRACE_Y / 2.0 - 22.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("reservoir_cap_septum_traceability_fiducial_{i}_boss"),
            8.0,
            3.0,
            28,
        )
        .translate(*x, *y, TRACE_Z + 1.5);
        let cut = centered_cylinder(
            format!("reservoir_cap_septum_traceability_fiducial_{i}_center_cut"),
            2.0,
            4.0,
            20,
        )
        .translate(*x, *y, TRACE_Z + 2.0);
        fiducials = fiducials + (boss - cut);
    }
    fiducials
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "reservoir_cap_septum_release_hold_reject_lane_base",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let mut cuts = Part::empty("reservoir_cap_septum_disposition_lane_slot_cuts");
    let mut lane_rails = Part::empty("reservoir_cap_septum_disposition_lane_rails");
    for lane in DispositionLane::all() {
        let y = status_lane_y(lane);
        lane_rails = lane_rails
            + centered_cube(
                format!("reservoir_cap_septum_{}_lane_front_rail", lane.label()),
                STATUS_X - 30.0,
                5.0,
                8.0,
            )
            .translate(0.0, y - 16.0, STATUS_Z + 4.0)
            + centered_cube(
                format!("reservoir_cap_septum_{}_lane_rear_rail", lane.label()),
                STATUS_X - 30.0,
                5.0,
                8.0,
            )
            .translate(0.0, y + 16.0, STATUS_Z + 4.0);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 72.0);
            cuts = cuts
                + centered_cube(
                    format!("reservoir_cap_septum_{}_lane_slot_{slot}_cut", lane.label()),
                    42.0,
                    20.0,
                    20.0,
                )
                .translate(x, y, STATUS_Z - 9.5);
        }
    }

    base + lane_rails - cuts + status_lane_header_tabs()
}

fn status_lane_header_tabs() -> Part {
    let mut tabs = Part::empty("reservoir_cap_septum_status_lane_header_tabs");
    for lane in DispositionLane::all() {
        tabs = tabs
            + centered_cube(
                format!("reservoir_cap_septum_{}_lane_header_tab", lane.label()),
                64.0,
                8.0,
                6.0,
            )
            .translate(-STATUS_X / 2.0 + 48.0, status_lane_y(lane), STATUS_Z + 3.0);
    }
    tabs
}

fn evidence_bridge() -> Part {
    let base = centered_cube(
        "reservoir_cap_septum_evidence_bridge_base_plate",
        EVIDENCE_X,
        EVIDENCE_Y,
        12.0,
    )
    .translate(0.0, 0.0, 6.0);
    let front_post = centered_cube(
        "reservoir_cap_septum_evidence_bridge_front_post",
        22.0,
        22.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        -EVIDENCE_X / 2.0 + 30.0,
        -EVIDENCE_Y / 2.0 + 24.0,
        12.0 + EVIDENCE_POST_Z / 2.0,
    );
    let rear_post = centered_cube(
        "reservoir_cap_septum_evidence_bridge_rear_post",
        22.0,
        22.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_X / 2.0 - 30.0,
        EVIDENCE_Y / 2.0 - 24.0,
        12.0 + EVIDENCE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "reservoir_cap_septum_evidence_bridge_camera_beam",
        EVIDENCE_X - 46.0,
        26.0,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, 12.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z / 2.0);
    let camera_window = centered_cube(
        "reservoir_cap_septum_evidence_bridge_camera_window",
        72.0,
        18.0,
        EVIDENCE_BEAM_Z + 2.0,
    )
    .translate(0.0, 0.0, 12.0 + EVIDENCE_POST_Z - EVIDENCE_BEAM_Z / 2.0);

    base + front_post + rear_post + (beam - camera_window) + evidence_target_lands()
}

fn evidence_target_lands() -> Part {
    let mut targets = Part::empty("reservoir_cap_septum_evidence_target_lands");
    for i in 0..EVIDENCE_TARGETS {
        targets = targets
            + centered_cube(
                format!("reservoir_cap_septum_evidence_target_land_{i}"),
                34.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, EVIDENCE_TARGETS, 44.0),
                -EVIDENCE_Y / 2.0 + 26.0,
                14.0,
            );
    }
    targets
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot_rail = centered_cube(
        "reservoir_cap_septum_robot_front_keepout_rail",
        STATION_X - 220.0,
        KEEP_OUT_RAIL_Z,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, DECK_Z + 9.0);
    let rear_service_rail = centered_cube(
        "reservoir_cap_septum_pressure_service_rear_keepout_rail",
        STATION_X - 260.0,
        KEEP_OUT_RAIL_Z,
        18.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y, DECK_Z + 9.0);
    let left_service_rail = centered_cube(
        "reservoir_cap_septum_left_robot_service_keepout_rail",
        KEEP_OUT_RAIL_Z,
        STATION_Y - 260.0,
        18.0,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X,
        0.0,
        DECK_Z + 9.0,
    );
    let right_service_rail = centered_cube(
        "reservoir_cap_septum_right_robot_service_keepout_rail",
        KEEP_OUT_RAIL_Z,
        STATION_Y - 260.0,
        18.0,
    )
    .translate(STATION_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X, 0.0, DECK_Z + 9.0);
    let pick_clearance_bridge = centered_cube(
        "reservoir_cap_septum_robot_pick_clearance_bridge",
        STATION_X - 250.0,
        18.0,
        24.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_PICK_CLEARANCE_Z,
    );
    let pressure_service_bridge = centered_cube(
        "reservoir_cap_septum_pressure_service_clearance_bridge",
        470.0,
        18.0,
        24.0,
    )
    .translate(
        PRESSURE_POS.0 + 85.0,
        PRESSURE_POS.1 + PRESSURE_Y / 2.0 + 42.0,
        PRESSURE_SERVICE_CLEARANCE_Z,
    );

    front_robot_rail
        + rear_service_rail
        + left_service_rail
        + right_service_rail
        + pick_clearance_bridge
        + pressure_service_bridge
        + keepout_gauge_posts()
}

fn keepout_gauge_posts() -> Part {
    let mut posts = Part::empty("reservoir_cap_septum_robot_service_keepout_gauge_posts");
    for (i, (x, y, z)) in [
        (
            -STATION_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X,
            -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
            ROBOT_PICK_CLEARANCE_Z,
        ),
        (
            STATION_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X,
            -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
            ROBOT_PICK_CLEARANCE_Z,
        ),
        (
            -STATION_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X,
            STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
            PRESSURE_SERVICE_CLEARANCE_Z,
        ),
        (
            STATION_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X,
            STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
            PRESSURE_SERVICE_CLEARANCE_Z,
        ),
        (
            PRESSURE_POS.0 - PRESSURE_X / 2.0 + 34.0,
            PRESSURE_POS.1 + PRESSURE_Y / 2.0 + 42.0,
            PRESSURE_SERVICE_CLEARANCE_Z,
        ),
        (
            PRESSURE_POS.0 + PRESSURE_X / 2.0 - 34.0,
            PRESSURE_POS.1 + PRESSURE_Y / 2.0 + 42.0,
            PRESSURE_SERVICE_CLEARANCE_Z,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("reservoir_cap_septum_keepout_gauge_post_{i}"),
                6.0,
                *z - DECK_Z,
                24,
            )
            .translate(*x, *y, (DECK_Z + *z) / 2.0);
    }
    posts
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "reservoir_cap_coupon_nests",
            center: CAP_POS,
            x: CAP_X,
            y: CAP_Y,
        },
        Rect {
            name: "repeated_puncture_guide_bridge",
            center: GUIDE_POS,
            x: GUIDE_X,
            y: GUIDE_Y,
        },
        Rect {
            name: "septum_compression_witness_pockets",
            center: COMP_POS,
            x: COMP_X,
            y: COMP_Y,
        },
        Rect {
            name: "pressure_hold_leak_ports",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "dye_ingress_wells",
            center: DYE_POS,
            x: DYE_X,
            y: DYE_Y,
        },
        Rect {
            name: "needle_connector_parking",
            center: PARK_POS,
            x: PARK_X,
            y: PARK_Y,
        },
        Rect {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: STATUS_POS,
            x: STATUS_X,
            y: STATUS_Y,
        },
        Rect {
            name: "evidence_bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "export count changed");
    assert_eq!(
        REQUIRED_FEATURES.len(),
        10,
        "required feature count changed"
    );
    assert_eq!(CAP_COUNT, CAP_ROWS * CAP_COLS);
    assert_eq!(CAP_ORIENTATION_KEYS, CAP_COUNT);
    assert_eq!(COMP_COUNT, COMP_ROWS * COMP_COLS);
    assert_eq!(DYE_WELLS, DYE_ROWS * DYE_COLS);
    assert_eq!(PRESSURE_CHANNELS, CAP_COUNT);
    assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, CAP_COUNT);
    assert!(NEEDLE_PARKS >= CAP_COUNT);
    assert!(CONNECTOR_PARKS >= PRESSURE_CHANNELS / 2);
    assert!(GUIDE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(ROBOT_PICK_CLEARANCE_Z > EVIDENCE_POST_Z);
    assert!(PRESSURE_COLLAR_D > PRESSURE_PORT_D + 10.0);

    for rect in module_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station containment footprint",
            rect.name
        );
    }

    let rects = module_rects();
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

fn cap_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, CAP_COLS, CAP_PITCH_X),
        centered_index(row, CAP_ROWS, CAP_PITCH_Y),
    )
}

fn cap_index(row: usize, col: usize) -> usize {
    row * CAP_COLS + col
}

fn comp_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COMP_COLS, COMP_PITCH_X),
        centered_index(row, COMP_ROWS, COMP_PITCH_Y),
    )
}

fn pressure_port_xy(index: usize) -> (f64, f64) {
    let row = index / PRESSURE_COLS;
    let col = index % PRESSURE_COLS;
    (
        centered_index(col, PRESSURE_COLS, 56.0),
        centered_index(row, PRESSURE_CHANNELS / PRESSURE_COLS, 46.0),
    )
}

fn dye_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, DYE_COLS, 50.0),
        centered_index(row, DYE_ROWS, 42.0),
    )
}

fn needle_slot_x(index: usize) -> f64 {
    centered_index(index, NEEDLE_PARKS, 28.0)
}

fn connector_socket_x(index: usize) -> f64 {
    centered_index(index, CONNECTOR_PARKS, 38.0)
}

fn status_lane_y(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), STATUS_LANES, 32.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_reservoir_cap_septum_puncture_leak_life_station_"),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_groups_cover_requested_scope() {
        for feature in [
            "reservoir_cap_coupon_nests",
            "repeated_puncture_guide_bridge",
            "septum_compression_witness_pockets",
            "pressure_hold_leak_ports",
            "dye_ingress_wells",
            "needle_connector_parking",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_fits_and_has_separate_validation_zones() {
        assert_design_constraints();
        assert_eq!(module_rects().len(), 9);
    }

    #[test]
    fn cap_puncture_pressure_and_dye_counts_are_matched() {
        assert_eq!(CAP_COUNT, 12);
        assert_eq!(PRESSURE_CHANNELS, CAP_COUNT);
        assert_eq!(DYE_WELLS, CAP_COUNT);
        assert_eq!(COMP_COUNT, CAP_COUNT);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, CAP_COUNT);
    }

    #[test]
    fn handling_traceability_and_keepouts_are_explicit() {
        assert!(NEEDLE_PARKS >= CAP_COUNT);
        assert!(CONNECTOR_PARKS >= 8);
        assert_eq!(BARCODE_LANDS, 6);
        assert_eq!(CERTIFICATE_LANDS, 3);
        assert_eq!(SIGNOFF_LANDS, 2);
        assert_eq!(KEEP_OUT_GAUGES, 6);
    }

    #[test]
    fn guide_and_evidence_clearances_are_visible() {
        assert!(GUIDE_POST_Z > CAMERA_CLEARANCE_Z);
        assert!(ROBOT_PICK_CLEARANCE_Z > EVIDENCE_POST_Z);
        assert!(PRESSURE_SERVICE_CLEARANCE_Z > GUIDE_POST_Z);
        assert!(GUIDE_BUSHING_OD > GUIDE_BUSHING_ID + 8.0);
    }
}
