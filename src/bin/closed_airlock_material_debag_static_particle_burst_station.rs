use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed airlock material de-bagging/static particle burst validation station.
//
// Intent:
// - Fixture consumables entering a clean support pod or isolator path so outer
//   tote/bag removal, static-burst challenge points, particle witness coupons,
//   custody evidence, and quarantine/release decisions are mechanically visible.
// - Keep bought instruments and validation consumables as envelope placeholders
//   with datum geometry rather than claiming final metrology or cleanroom release.
// - Preserve robot/service datums for later integration with the broader
//   LaminarForge closed material-flow workcell.

const PREFIX: &str = "closed_airlock_material_debag_static_particle_burst_station";
const OUTPUT_PREFIX: &str = "output/closed_airlock_material_debag_static_particle_burst_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_airlock_material_debag_static_particle_burst_station_containment_deck.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_sealed_tote_bag_receiver.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_peel_debag_force_datum_rail.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_static_charge_probe_pockets.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_particle_witness_coupon_grid.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_airflow_smoke_vane_grid.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_waste_overbag_chute.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_barcode_coa_custody_lands.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_quarantine_release_gates.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_camera_evidence_bridge.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_robotic_service_datums.stl",
    "output/closed_airlock_material_debag_static_particle_burst_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_tote_bag_receiver",
    "peel_debag_force_datum_rail",
    "static_charge_probe_pockets",
    "particle_witness_coupon_grid",
    "airflow_smoke_vane_grid",
    "waste_overbag_chute",
    "barcode_coa_custody_lands",
    "quarantine_release_gates",
    "camera_evidence_bridge",
    "robotic_service_datums",
    "containment_deck",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_packaging_only",
    "not_a_cleanroom_release_specification",
    "not_a_static_discharge_sop",
    "not_a_particle_limit_claim",
    "sensors_and_cameras_are_placeholder_envelopes",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 52.0;
const CLEARANCE: f64 = 14.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 7.0;

const RECEIVER_CENTER: (f64, f64) = (-480.0, 220.0);
const RECEIVER_X: f64 = 460.0;
const RECEIVER_Y: f64 = 250.0;
const RECEIVER_Z: f64 = 94.0;
const TOTE_CLEAR_X: f64 = 360.0;
const TOTE_CLEAR_Y: f64 = 168.0;
const BAG_COLLAR_D: f64 = 116.0;
const RECEIVER_GUIDE_RIBS: usize = 6;

const PEEL_CENTER: (f64, f64) = (60.0, 220.0);
const PEEL_X: f64 = 430.0;
const PEEL_Y: f64 = 160.0;
const PEEL_Z: f64 = 64.0;
const FORCE_DATUM_RAILS: usize = 2;
const PEEL_CLAMP_STATIONS: usize = 6;
const FORCE_TICKS: usize = 9;

const STATIC_CENTER: (f64, f64) = (515.0, 220.0);
const STATIC_X: f64 = 290.0;
const STATIC_Y: f64 = 160.0;
const STATIC_Z: f64 = 62.0;
const STATIC_PROBE_POCKETS: usize = 8;
const GROUND_BUTTONS: usize = 4;
const CABLE_COMB_SLOTS: usize = 7;

const COUPON_CENTER: (f64, f64) = (-495.0, -35.0);
const COUPON_X: f64 = 430.0;
const COUPON_Y: f64 = 230.0;
const COUPON_Z: f64 = 44.0;
const PARTICLE_COUPON_ROWS: usize = 4;
const PARTICLE_COUPON_COLS: usize = 5;
const PARTICLE_COUPONS: usize = PARTICLE_COUPON_ROWS * PARTICLE_COUPON_COLS;

const VANE_CENTER: (f64, f64) = (0.0, -35.0);
const VANE_X: f64 = 430.0;
const VANE_Y: f64 = 230.0;
const VANE_Z: f64 = 56.0;
const VANE_ROWS: usize = 3;
const VANE_COLS: usize = 6;
const VANE_COUNT: usize = VANE_ROWS * VANE_COLS;
const SMOKE_WAND_PORTS: usize = 4;

const WASTE_CENTER: (f64, f64) = (500.0, -35.0);
const WASTE_X: f64 = 340.0;
const WASTE_Y: f64 = 230.0;
const WASTE_Z: f64 = 108.0;
const OVERBAG_RETAINER_SLOTS: usize = 6;

const CUSTODY_CENTER: (f64, f64) = (-500.0, -305.0);
const CUSTODY_X: f64 = 360.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const COA_LANDS: usize = 4;
const CUSTODY_CARD_SLOTS: usize = 6;

const GATE_CENTER: (f64, f64) = (0.0, -305.0);
const GATE_X: f64 = 360.0;
const GATE_Y: f64 = 150.0;
const GATE_Z: f64 = 48.0;
const DISPOSITION_STATES: usize = 3;
const GATE_SLOTS_PER_STATE: usize = 4;

const DATUM_CENTER: (f64, f64) = (500.0, -305.0);
const DATUM_X: f64 = 340.0;
const DATUM_Y: f64 = 150.0;
const DATUM_Z: f64 = 58.0;
const ROBOTIC_SERVICE_DATUMS: usize = 8;
const SERVICE_RAILS: usize = 2;
const TOOL_CLEARANCE_GAUGES: usize = 5;

const BRIDGE_CENTER: (f64, f64) = (0.0, 5.0);
const BRIDGE_X: f64 = 1300.0;
const BRIDGE_Y: f64 = 690.0;
const BRIDGE_Z: f64 = 220.0;
const CAMERA_PODS: usize = 5;
const EVIDENCE_LED_BARS: usize = 4;
const BRIDGE_UNDERSIDE_Z: f64 = 236.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn left(self) -> f64 {
        self.center.0 - self.x / 2.0
    }

    fn right(self) -> f64 {
        self.center.0 + self.x / 2.0
    }

    fn bottom(self) -> f64 {
        self.center.1 - self.y / 2.0
    }

    fn top(self) -> f64 {
        self.center.1 + self.y / 2.0
    }

    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - CLEARANCE;
        self.left() >= -usable_x
            && self.right() <= usable_x
            && self.bottom() >= -usable_y
            && self.top() <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        self.left() < other.right() + clearance
            && self.right() > other.left() - clearance
            && self.bottom() < other.top() + clearance
            && self.top() > other.bottom() - clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let receiver = sealed_tote_bag_receiver().translate(
        RECEIVER_CENTER.0,
        RECEIVER_CENTER.1,
        insert_z(RECEIVER_Z),
    );
    export(&receiver, OUTPUTS[1]);

    let peel =
        peel_debag_force_datum_rail().translate(PEEL_CENTER.0, PEEL_CENTER.1, insert_z(PEEL_Z));
    export(&peel, OUTPUTS[2]);

    let static_pockets = static_charge_probe_pockets().translate(
        STATIC_CENTER.0,
        STATIC_CENTER.1,
        insert_z(STATIC_Z),
    );
    export(&static_pockets, OUTPUTS[3]);

    let coupons = particle_witness_coupon_grid().translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        insert_z(COUPON_Z),
    );
    export(&coupons, OUTPUTS[4]);

    let vanes = airflow_smoke_vane_grid().translate(VANE_CENTER.0, VANE_CENTER.1, insert_z(VANE_Z));
    export(&vanes, OUTPUTS[5]);

    let waste = waste_overbag_chute().translate(WASTE_CENTER.0, WASTE_CENTER.1, insert_z(WASTE_Z));
    export(&waste, OUTPUTS[6]);

    let custody = barcode_coa_custody_lands().translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_Z),
    );
    export(&custody, OUTPUTS[7]);

    let gates =
        quarantine_release_gates().translate(GATE_CENTER.0, GATE_CENTER.1, insert_z(GATE_Z));
    export(&gates, OUTPUTS[8]);

    let bridge = camera_evidence_bridge().translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        BASE_Z / 2.0 + BRIDGE_Z / 2.0,
    );
    export(&bridge, OUTPUTS[9]);

    let datums =
        robotic_service_datums().translate(DATUM_CENTER.0, DATUM_CENTER.1, insert_z(DATUM_Z));
    export(&datums, OUTPUTS[10]);

    let assembly = deck
        + receiver
        + peel
        + static_pockets
        + coupons
        + vanes
        + waste
        + custody
        + gates
        + bridge
        + datums;
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed airlock material de-bag/static particle burst validation station:");
    println!(
        "  Footprint:       {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck; {TOTE_CLEAR_X:.0}mm x {TOTE_CLEAR_Y:.0}mm tote/bag receiver clear land"
    );
    println!(
        "  Debag controls:  {FORCE_DATUM_RAILS} peel datum rails, {PEEL_CLAMP_STATIONS} clamp stations, {FORCE_TICKS} force graduation ticks"
    );
    println!(
        "  Burst evidence:  {STATIC_PROBE_POCKETS} static probe pockets, {PARTICLE_COUPONS} particle witness coupons, {VANE_COUNT} smoke vane cells, {SMOKE_WAND_PORTS} smoke wand ports"
    );
    println!(
        "  Custody/status:  {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {CUSTODY_CARD_SLOTS} custody card slots, {DISPOSITION_STATES} quarantine/release states with {GATE_SLOTS_PER_STATE} slots each"
    );
    println!(
        "  Service:         {CAMERA_PODS} camera pods, {EVIDENCE_LED_BARS} evidence LED bars, {ROBOTIC_SERVICE_DATUMS} robot datums, {SERVICE_RAILS} service rails"
    );
    println!("  Output prefix:   {OUTPUT_PREFIX}");
    println!("  Limitations:     {}", LIMITATIONS.join(", "));
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn station_rects() -> [Rect; 10] {
    [
        Rect {
            name: "sealed_tote_bag_receiver",
            center: RECEIVER_CENTER,
            x: RECEIVER_X,
            y: RECEIVER_Y,
        },
        Rect {
            name: "peel_debag_force_datum_rail",
            center: PEEL_CENTER,
            x: PEEL_X,
            y: PEEL_Y,
        },
        Rect {
            name: "static_charge_probe_pockets",
            center: STATIC_CENTER,
            x: STATIC_X,
            y: STATIC_Y,
        },
        Rect {
            name: "particle_witness_coupon_grid",
            center: COUPON_CENTER,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "airflow_smoke_vane_grid",
            center: VANE_CENTER,
            x: VANE_X,
            y: VANE_Y,
        },
        Rect {
            name: "waste_overbag_chute",
            center: WASTE_CENTER,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "barcode_coa_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "quarantine_release_gates",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
        Rect {
            name: "robotic_service_datums",
            center: DATUM_CENTER,
            x: DATUM_X,
            y: DATUM_Y,
        },
        Rect {
            name: "camera_evidence_bridge_projection",
            center: BRIDGE_CENTER,
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert_eq!(
        PARTICLE_COUPONS,
        PARTICLE_COUPON_ROWS * PARTICLE_COUPON_COLS
    );
    assert_eq!(VANE_COUNT, VANE_ROWS * VANE_COLS);
    assert_eq!(DISPOSITION_STATES, 3);
    assert_eq!(FORCE_DATUM_RAILS, 2);
    assert!(STATIC_PROBE_POCKETS >= 2 * GROUND_BUTTONS);
    assert!(PARTICLE_COUPONS >= STATIC_PROBE_POCKETS * 2);
    assert!(BARCODE_LANDS >= GATE_SLOTS_PER_STATE * DISPOSITION_STATES - 2);
    assert!(BRIDGE_UNDERSIDE_Z > RECEIVER_Z + BASE_Z + 85.0);
    assert!(TOTE_CLEAR_X < RECEIVER_X - 60.0);
    assert!(TOTE_CLEAR_Y < RECEIVER_Y - 50.0);

    let rects = station_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station envelope",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            if rects[i].name == "camera_evidence_bridge_projection"
                || rects[j].name == "camera_evidence_bridge_projection"
            {
                continue;
            }
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], CLEARANCE),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_one_piece_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let upper_recess = centered_cube(
        format!("{PREFIX}_upper_receiver_force_static_recess"),
        1320.0,
        270.0,
        8.0,
    )
    .translate(10.0, 220.0, BASE_Z / 2.0 - 4.0);
    let burst_recess = centered_cube(
        format!("{PREFIX}_middle_particle_airflow_waste_recess"),
        1320.0,
        246.0,
        8.0,
    )
    .translate(0.0, -35.0, BASE_Z / 2.0 - 4.0);
    let custody_recess = centered_cube(
        format!("{PREFIX}_lower_custody_gate_robot_recess"),
        1320.0,
        168.0,
        8.0,
    )
    .translate(0.0, -305.0, BASE_Z / 2.0 - 4.0);

    deck - upper_recess
        - burst_recess
        - custody_recess
        - insert_sockets()
        - mounting_holes()
        - low_point_drain_ports()
        + perimeter_rims()
        + row_dividers()
        + static_flow_direction_gutters()
        + airlock_datum_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_drop_in_insert_sockets"));
    for rect in station_rects() {
        if rect.name == "camera_evidence_bridge_projection" {
            continue;
        }
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket", rect.name),
                rect.x + 8.0,
                rect.y + 8.0,
                SOCKET_DEPTH,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_deck_mounting_holes"));
    for (i, (x, y)) in [
        (-690.0, 405.0),
        (0.0, 405.0),
        (690.0, 405.0),
        (-690.0, -405.0),
        (0.0, -405.0),
        (690.0, -405.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_clearance_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                30,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn low_point_drain_ports() -> Part {
    let front = centered_cylinder(
        format!("{PREFIX}_front_quarantine_basin_drain"),
        6.0,
        54.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(666.0, -STATION_Y / 2.0 + 22.0, -1.0);
    let waste = centered_cylinder(format!("{PREFIX}_waste_chute_basin_drain"), 6.0, 54.0, 30)
        .rotate(90.0, 0.0, 0.0)
        .translate(666.0, -154.0, -1.0);
    front + waste
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_left_airlock_containment_rim"),
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_airlock_containment_rim"),
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_airlock_tote_receiver_rim"),
        STATION_X - 44.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        format!("{PREFIX}_front_low_profile_robot_service_lip"),
        STATION_X - 220.0,
        16.0,
        30.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 34.0, BASE_Z / 2.0 + 15.0);
    left + right + rear + front
}

fn row_dividers() -> Part {
    let upper_mid = centered_cube(
        format!("{PREFIX}_debag_to_particle_validation_divider"),
        1310.0,
        12.0,
        34.0,
    )
    .translate(0.0, 83.0, BASE_Z / 2.0 + 17.0);
    let mid_lower = centered_cube(
        format!("{PREFIX}_burst_evidence_to_custody_divider"),
        1310.0,
        12.0,
        34.0,
    )
    .translate(0.0, -188.0, BASE_Z / 2.0 + 17.0);
    let receiver_peel = centered_cube(
        format!("{PREFIX}_receiver_to_peel_force_zone_divider"),
        10.0,
        238.0,
        32.0,
    )
    .translate(-220.0, 220.0, BASE_Z / 2.0 + 16.0);
    let peel_static = centered_cube(
        format!("{PREFIX}_peel_to_static_probe_zone_divider"),
        10.0,
        152.0,
        32.0,
    )
    .translate(295.0, 220.0, BASE_Z / 2.0 + 16.0);
    let coupon_vane = centered_cube(
        format!("{PREFIX}_coupon_grid_to_smoke_vane_divider"),
        10.0,
        218.0,
        32.0,
    )
    .translate(-252.0, -35.0, BASE_Z / 2.0 + 16.0);
    let vane_waste = centered_cube(
        format!("{PREFIX}_smoke_vane_to_waste_chute_divider"),
        10.0,
        218.0,
        32.0,
    )
    .translate(252.0, -35.0, BASE_Z / 2.0 + 16.0);
    upper_mid + mid_lower + receiver_peel + peel_static + coupon_vane + vane_waste
}

fn static_flow_direction_gutters() -> Part {
    let mut gutters = Part::empty(format!("{PREFIX}_airflow_static_burst_direction_gutters"));
    for i in 0..6 {
        gutters = gutters
            + centered_cube(
                format!("{PREFIX}_static_burst_path_gutter_{i}"),
                7.0,
                116.0,
                7.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 8.0 } else { -8.0 })
            .translate(centered_index(i, 6, 205.0), 92.0, BASE_Z / 2.0 + 3.5)
            + centered_cube(
                format!("{PREFIX}_particle_fallout_path_gutter_{i}"),
                7.0,
                128.0,
                7.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { -9.0 } else { 9.0 })
            .translate(centered_index(i, 6, 205.0), -166.0, BASE_Z / 2.0 + 3.5);
    }
    gutters
}

fn airlock_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_deck_airlock_datum_fiducials"));
    for (i, (x, y)) in [
        (-704.0, 398.0),
        (704.0, 398.0),
        (-704.0, -398.0),
        (704.0, -398.0),
        (-18.0, 86.0),
        (-18.0, -188.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("{PREFIX}_engraved_airlock_fiducial_ring_{i}"),
                16.0,
                3.0,
                40,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5)
            - centered_cylinder(
                format!("{PREFIX}_engraved_airlock_fiducial_center_{i}"),
                7.0,
                5.0,
                32,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn sealed_tote_bag_receiver() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_sealed_tote_bag_receiver_body"),
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
    );
    let tote_clearance = centered_cube(
        format!("{PREFIX}_sealed_tote_clearance_pocket"),
        TOTE_CLEAR_X,
        TOTE_CLEAR_Y,
        RECEIVER_Z + 6.0,
    )
    .translate(0.0, 10.0, RECEIVER_Z / 2.0 - 26.0);
    let bag_collar_bore = centered_cylinder(
        format!("{PREFIX}_round_bag_collar_receiver_bore"),
        BAG_COLLAR_D / 2.0,
        RECEIVER_Y + 8.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -RECEIVER_Y / 2.0 + 28.0, RECEIVER_Z / 2.0 - 12.0);
    let floor_drain = centered_cylinder(
        format!("{PREFIX}_receiver_floor_debris_drain"),
        7.0,
        32.0,
        30,
    )
    .translate(RECEIVER_X / 2.0 - 54.0, -RECEIVER_Y / 2.0 + 54.0, 0.0);

    body - tote_clearance - bag_collar_bore - floor_drain
        + receiver_side_rails()
        + receiver_gasket_lips()
        + receiver_guide_ribs()
        + receiver_latch_bosses()
}

fn receiver_side_rails() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_receiver_left_tote_slide_rail"),
        16.0,
        RECEIVER_Y - 42.0,
        22.0,
    )
    .translate(-TOTE_CLEAR_X / 2.0 - 20.0, 0.0, RECEIVER_Z / 2.0 + 11.0);
    let right = centered_cube(
        format!("{PREFIX}_receiver_right_tote_slide_rail"),
        16.0,
        RECEIVER_Y - 42.0,
        22.0,
    )
    .translate(TOTE_CLEAR_X / 2.0 + 20.0, 0.0, RECEIVER_Z / 2.0 + 11.0);
    let rear_stop = centered_cube(
        format!("{PREFIX}_receiver_rear_compression_stop"),
        TOTE_CLEAR_X + 54.0,
        16.0,
        34.0,
    )
    .translate(0.0, RECEIVER_Y / 2.0 - 22.0, RECEIVER_Z / 2.0 + 17.0);
    left + right + rear_stop
}

fn receiver_gasket_lips() -> Part {
    let collar_ring = rectangular_frame(
        format!("{PREFIX}_receiver_bag_gasket_rectangular_lip"),
        190.0,
        12.0,
        148.0,
        132.0,
        94.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -RECEIVER_Y / 2.0 + 14.0, RECEIVER_Z / 2.0 + 8.0);
    let compression_witness_strip = centered_cube(
        format!("{PREFIX}_receiver_gasket_compression_witness_strip"),
        238.0,
        8.0,
        8.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 + 22.0, RECEIVER_Z / 2.0 + 82.0);
    collar_ring + compression_witness_strip
}

fn receiver_guide_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_receiver_tote_guide_ribs"));
    for i in 0..RECEIVER_GUIDE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_receiver_floor_guide_rib_{i}"),
                12.0,
                118.0,
                12.0,
            )
            .translate(
                centered_index(i, RECEIVER_GUIDE_RIBS, 54.0),
                24.0,
                RECEIVER_Z / 2.0 + 6.0,
            );
    }
    ribs
}

fn receiver_latch_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PREFIX}_receiver_sealed_latch_bosses"));
    for (i, x) in [-160.0, -80.0, 80.0, 160.0].iter().enumerate() {
        bosses = bosses
            + centered_cylinder(
                format!("{PREFIX}_receiver_toggle_latch_boss_{i}"),
                13.0,
                18.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -RECEIVER_Y / 2.0 + 18.0, RECEIVER_Z / 2.0 + 38.0);
    }
    bosses
}

fn peel_debag_force_datum_rail() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_peel_force_datum_rail_base"),
        PEEL_X,
        PEEL_Y,
        PEEL_Z,
    );
    base - peel_grip_relief_slots() - peel_load_cell_pockets()
        + peel_parallel_datum_rails()
        + peel_clamp_stations()
        + force_tick_ladder()
        + film_tail_capture_teeth()
}

fn peel_parallel_datum_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_parallel_peel_force_datum_rails"));
    for i in 0..FORCE_DATUM_RAILS {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_peel_force_datum_rail_{i}"),
                PEEL_X - 48.0,
                13.0,
                32.0,
            )
            .translate(
                0.0,
                centered_index(i, FORCE_DATUM_RAILS, 74.0),
                PEEL_Z / 2.0 + 16.0,
            );
    }
    rails
}

fn peel_grip_relief_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_peel_grip_relief_slots"));
    for i in 0..PEEL_CLAMP_STATIONS {
        slots = slots
            + centered_cube(
                format!("{PREFIX}_peel_film_tail_relief_slot_{i}"),
                42.0,
                28.0,
                20.0,
            )
            .translate(
                centered_index(i, PEEL_CLAMP_STATIONS, 64.0),
                0.0,
                PEEL_Z / 2.0 - 10.0,
            );
    }
    slots
}

fn peel_load_cell_pockets() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_left_peel_load_cell_pocket"),
        72.0,
        54.0,
        22.0,
    )
    .translate(-PEEL_X / 2.0 + 62.0, 0.0, PEEL_Z / 2.0 - 11.0);
    let right = centered_cube(
        format!("{PREFIX}_right_peel_load_cell_pocket"),
        72.0,
        54.0,
        22.0,
    )
    .translate(PEEL_X / 2.0 - 62.0, 0.0, PEEL_Z / 2.0 - 11.0);
    left + right
}

fn peel_clamp_stations() -> Part {
    let mut clamps = Part::empty(format!("{PREFIX}_peel_debag_clamp_stations"));
    for i in 0..PEEL_CLAMP_STATIONS {
        clamps = clamps
            + centered_cube(
                format!("{PREFIX}_peel_debag_clamp_pad_{i}"),
                36.0,
                18.0,
                16.0,
            )
            .translate(
                centered_index(i, PEEL_CLAMP_STATIONS, 64.0),
                -46.0,
                PEEL_Z / 2.0 + 8.0,
            )
            + centered_cylinder(format!("{PREFIX}_peel_clamp_pivot_pin_{i}"), 5.0, 36.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(i, PEEL_CLAMP_STATIONS, 64.0),
                    -46.0,
                    PEEL_Z / 2.0 + 24.0,
                );
    }
    clamps
}

fn force_tick_ladder() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_peel_force_graduation_ticks"));
    for i in 0..FORCE_TICKS {
        let tick_h = if i % 2 == 0 { 22.0 } else { 13.0 };
        ticks = ticks
            + centered_cube(format!("{PREFIX}_peel_force_tick_{i}"), 5.0, 32.0, tick_h).translate(
                centered_index(i, FORCE_TICKS, 42.0),
                PEEL_Y / 2.0 - 28.0,
                PEEL_Z / 2.0 + tick_h / 2.0,
            );
    }
    ticks
}

fn film_tail_capture_teeth() -> Part {
    let mut teeth = Part::empty(format!("{PREFIX}_outer_film_tail_capture_teeth"));
    for i in 0..8 {
        teeth = teeth
            + centered_cube(
                format!("{PREFIX}_film_tail_capture_tooth_{i}"),
                10.0,
                42.0,
                18.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 10.0 } else { -10.0 })
            .translate(
                centered_index(i, 8, 42.0),
                -PEEL_Y / 2.0 + 28.0,
                PEEL_Z / 2.0 + 9.0,
            );
    }
    teeth
}

fn static_charge_probe_pockets() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_static_charge_probe_pocket_body"),
        STATIC_X,
        STATIC_Y,
        STATIC_Z,
    );
    body - static_probe_socket_cutouts() - static_ground_button_reliefs()
        + esd_isolation_islands()
        + cable_strain_relief_comb()
        + static_reference_scale()
}

fn static_probe_socket_cutouts() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_static_charge_probe_socket_cutouts"));
    for i in 0..STATIC_PROBE_POCKETS {
        sockets = sockets
            + centered_cylinder(format!("{PREFIX}_static_probe_socket_{i}"), 9.0, 28.0, 30)
                .translate(
                    centered_index(i % 4, 4, 54.0),
                    centered_index(i / 4, 2, 58.0),
                    STATIC_Z / 2.0 - 14.0,
                );
    }
    sockets
}

fn static_ground_button_reliefs() -> Part {
    let mut reliefs = Part::empty(format!("{PREFIX}_static_ground_button_reliefs"));
    for i in 0..GROUND_BUTTONS {
        reliefs = reliefs
            + centered_cylinder(
                format!("{PREFIX}_ground_reference_button_relief_{i}"),
                11.0,
                18.0,
                30,
            )
            .translate(
                centered_index(i, GROUND_BUTTONS, 58.0),
                -STATIC_Y / 2.0 + 26.0,
                STATIC_Z / 2.0 - 9.0,
            );
    }
    reliefs
}

fn esd_isolation_islands() -> Part {
    let mut islands = Part::empty(format!("{PREFIX}_esd_probe_isolation_islands"));
    for i in 0..STATIC_PROBE_POCKETS {
        islands = islands
            + centered_cube(
                format!("{PREFIX}_esd_isolation_island_{i}"),
                34.0,
                34.0,
                7.0,
            )
            .translate(
                centered_index(i % 4, 4, 54.0),
                centered_index(i / 4, 2, 58.0),
                STATIC_Z / 2.0 + 3.5,
            );
    }
    islands
}

fn cable_strain_relief_comb() -> Part {
    let mut comb = Part::empty(format!("{PREFIX}_static_probe_cable_strain_relief_comb"));
    for i in 0..CABLE_COMB_SLOTS {
        comb = comb
            + centered_cube(
                format!("{PREFIX}_static_probe_cable_comb_tooth_{i}"),
                8.0,
                32.0,
                20.0,
            )
            .translate(
                centered_index(i, CABLE_COMB_SLOTS, 32.0),
                STATIC_Y / 2.0 - 26.0,
                STATIC_Z / 2.0 + 10.0,
            );
    }
    comb
}

fn static_reference_scale() -> Part {
    let mut scale = Part::empty(format!("{PREFIX}_static_charge_reference_scale"));
    for i in 0..6 {
        scale = scale
            + centered_cube(
                format!("{PREFIX}_static_charge_reference_bar_{i}"),
                28.0,
                4.0,
                5.0,
            )
            .translate(
                -STATIC_X / 2.0 + 38.0 + i as f64 * 28.0,
                0.0,
                STATIC_Z / 2.0 + 2.5,
            );
    }
    scale
}

fn particle_witness_coupon_grid() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_particle_witness_coupon_grid_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    body - particle_coupon_recesses()
        + coupon_pull_tabs()
        + coupon_row_column_index_ticks()
        + coupon_corner_fiducials()
}

fn particle_coupon_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_particle_coupon_recesses"));
    for row in 0..PARTICLE_COUPON_ROWS {
        for col in 0..PARTICLE_COUPON_COLS {
            let idx = row * PARTICLE_COUPON_COLS + col;
            recesses = recesses
                + centered_cube(
                    format!("{PREFIX}_particle_coupon_recess_{idx}"),
                    52.0,
                    34.0,
                    14.0,
                )
                .translate(
                    centered_index(col, PARTICLE_COUPON_COLS, 70.0),
                    centered_index(row, PARTICLE_COUPON_ROWS, 46.0),
                    COUPON_Z / 2.0 - 7.0,
                );
        }
    }
    recesses
}

fn coupon_pull_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_particle_coupon_pull_tabs"));
    for row in 0..PARTICLE_COUPON_ROWS {
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_particle_coupon_row_pull_tab_{row}"),
                28.0,
                18.0,
                8.0,
            )
            .translate(
                COUPON_X / 2.0 - 30.0,
                centered_index(row, PARTICLE_COUPON_ROWS, 46.0),
                COUPON_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn coupon_row_column_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_particle_coupon_row_column_index_ticks"));
    for col in 0..PARTICLE_COUPON_COLS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_particle_coupon_column_tick_{col}"),
                36.0,
                5.0,
                6.0,
            )
            .translate(
                centered_index(col, PARTICLE_COUPON_COLS, 70.0),
                -COUPON_Y / 2.0 + 18.0,
                COUPON_Z / 2.0 + 3.0,
            );
    }
    for row in 0..PARTICLE_COUPON_ROWS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_particle_coupon_row_tick_{row}"),
                5.0,
                28.0,
                6.0,
            )
            .translate(
                -COUPON_X / 2.0 + 18.0,
                centered_index(row, PARTICLE_COUPON_ROWS, 46.0),
                COUPON_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn coupon_corner_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_particle_coupon_grid_fiducials"));
    for (i, (x, y)) in [
        (-COUPON_X / 2.0 + 34.0, -COUPON_Y / 2.0 + 34.0),
        (COUPON_X / 2.0 - 34.0, -COUPON_Y / 2.0 + 34.0),
        (-COUPON_X / 2.0 + 34.0, COUPON_Y / 2.0 - 34.0),
        (COUPON_X / 2.0 - 34.0, COUPON_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("{PREFIX}_particle_coupon_grid_fiducial_{i}"),
                10.0,
                5.0,
                30,
            )
            .translate(*x, *y, COUPON_Z / 2.0 + 2.5);
    }
    fiducials
}

fn airflow_smoke_vane_grid() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_airflow_smoke_vane_grid_frame"),
        VANE_X,
        VANE_Y,
        VANE_Z,
    );
    body - vane_window_cutouts() - smoke_wand_port_cutouts()
        + directional_smoke_vanes()
        + smoke_path_reference_ticks()
}

fn vane_window_cutouts() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_smoke_vane_window_cutouts"));
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let idx = row * VANE_COLS + col;
            windows = windows
                + centered_cube(
                    format!("{PREFIX}_smoke_vane_cell_window_{idx}"),
                    46.0,
                    42.0,
                    24.0,
                )
                .translate(
                    centered_index(col, VANE_COLS, 58.0),
                    centered_index(row, VANE_ROWS, 60.0),
                    VANE_Z / 2.0 - 12.0,
                );
        }
    }
    windows
}

fn smoke_wand_port_cutouts() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_smoke_wand_port_cutouts"));
    for i in 0..SMOKE_WAND_PORTS {
        ports = ports
            + centered_cylinder(format!("{PREFIX}_smoke_wand_port_{i}"), 8.0, 38.0, 28)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(i, SMOKE_WAND_PORTS, 84.0),
                    -VANE_Y / 2.0 + 14.0,
                    VANE_Z / 2.0 + 2.0,
                );
    }
    ports
}

fn directional_smoke_vanes() -> Part {
    let mut vanes = Part::empty(format!("{PREFIX}_directional_smoke_vanes"));
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let idx = row * VANE_COLS + col;
            let angle = if (row + col) % 2 == 0 { 14.0 } else { -14.0 };
            vanes = vanes
                + centered_cube(format!("{PREFIX}_smoke_vane_blade_{idx}"), 6.0, 40.0, 36.0)
                    .rotate(0.0, 0.0, angle)
                    .translate(
                        centered_index(col, VANE_COLS, 58.0),
                        centered_index(row, VANE_ROWS, 60.0),
                        VANE_Z / 2.0 + 18.0,
                    );
        }
    }
    vanes
}

fn smoke_path_reference_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_smoke_path_reference_ticks"));
    for i in 0..VANE_COLS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_smoke_path_column_tick_{i}"),
                34.0,
                5.0,
                7.0,
            )
            .translate(
                centered_index(i, VANE_COLS, 58.0),
                VANE_Y / 2.0 - 20.0,
                VANE_Z / 2.0 + 3.5,
            );
    }
    ticks
}

fn waste_overbag_chute() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_waste_overbag_chute_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let cavity = centered_cube(
        format!("{PREFIX}_waste_overbag_open_cavity"),
        WASTE_X - 54.0,
        WASTE_Y - 54.0,
        WASTE_Z + 4.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 20.0);
    let front_chute = centered_cube(
        format!("{PREFIX}_sloped_outer_overbag_chute_tongue"),
        WASTE_X - 86.0,
        62.0,
        50.0,
    )
    .rotate(-12.0, 0.0, 0.0)
    .translate(0.0, -WASTE_Y / 2.0 - 18.0, -8.0);
    body - cavity
        + front_chute
        + waste_collar_ring()
        + overbag_retainer_slots()
        + waste_full_witness_tabs()
}

fn waste_collar_ring() -> Part {
    rectangular_frame(
        format!("{PREFIX}_waste_overbag_collapsible_liner_ring"),
        WASTE_X - 44.0,
        12.0,
        122.0,
        WASTE_X - 116.0,
        70.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, WASTE_Y / 2.0 + 8.0, WASTE_Z / 2.0 + 4.0)
}

fn overbag_retainer_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_overbag_retainer_slots"));
    for i in 0..OVERBAG_RETAINER_SLOTS {
        slots = slots
            + centered_cube(
                format!("{PREFIX}_overbag_retain_clip_slot_{i}"),
                12.0,
                58.0,
                28.0,
            )
            .translate(
                centered_index(i, OVERBAG_RETAINER_SLOTS, 42.0),
                WASTE_Y / 2.0 - 28.0,
                WASTE_Z / 2.0 + 14.0,
            );
    }
    slots
}

fn waste_full_witness_tabs() -> Part {
    let low = centered_cube(
        format!("{PREFIX}_waste_bag_low_fill_witness_tab"),
        52.0,
        8.0,
        7.0,
    )
    .translate(-96.0, -WASTE_Y / 2.0 + 22.0, WASTE_Z / 2.0 + 18.0);
    let high = centered_cube(
        format!("{PREFIX}_waste_bag_high_fill_witness_tab"),
        52.0,
        8.0,
        7.0,
    )
    .translate(96.0, -WASTE_Y / 2.0 + 22.0, WASTE_Z / 2.0 + 58.0);
    low + high
}

fn barcode_coa_custody_lands() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_barcode_coa_custody_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    base + barcode_land_tiles() + coa_land_tiles() + custody_card_slots() + custody_hash_ticks()
}

fn barcode_land_tiles() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_barcode_land_{i}"), 70.0, 22.0, 5.0).translate(
                centered_index(i % 5, 5, 68.0),
                38.0 - (i / 5) as f64 * 32.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn coa_land_tiles() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_coa_custody_lands"));
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_coa_land_{i}"), 72.0, 32.0, 5.0).translate(
                -CUSTODY_X / 2.0 + 54.0,
                centered_index(i, COA_LANDS, 34.0),
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_custody_card_slots"));
    for i in 0..CUSTODY_CARD_SLOTS {
        slots = slots
            + centered_cube(format!("{PREFIX}_custody_card_slot_{i}"), 44.0, 16.0, 12.0).translate(
                CUSTODY_X / 2.0 - 54.0,
                centered_index(i, CUSTODY_CARD_SLOTS, 22.0),
                CUSTODY_Z / 2.0 + 6.0,
            );
    }
    slots
}

fn custody_hash_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_custody_hash_chain_ticks"));
    for i in 0..12 {
        ticks = ticks
            + centered_cube(format!("{PREFIX}_custody_hash_tick_{i}"), 4.0, 22.0, 6.0)
                .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
                .translate(
                    -92.0 + i as f64 * 16.0,
                    -CUSTODY_Y / 2.0 + 18.0,
                    CUSTODY_Z / 2.0 + 3.0,
                );
    }
    ticks
}

fn quarantine_release_gates() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_quarantine_release_gate_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    base - disposition_gate_recesses() + gate_state_fences() + gate_status_tokens()
}

fn disposition_gate_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_quarantine_release_gate_recesses"));
    for state in 0..DISPOSITION_STATES {
        for slot in 0..GATE_SLOTS_PER_STATE {
            let idx = state * GATE_SLOTS_PER_STATE + slot;
            recesses = recesses
                + centered_cube(
                    format!("{PREFIX}_disposition_gate_recess_{idx}"),
                    54.0,
                    26.0,
                    18.0,
                )
                .translate(
                    centered_index(slot, GATE_SLOTS_PER_STATE, 70.0),
                    centered_index(state, DISPOSITION_STATES, 42.0),
                    GATE_Z / 2.0 - 9.0,
                );
        }
    }
    recesses
}

fn gate_state_fences() -> Part {
    let mut fences = Part::empty(format!("{PREFIX}_quarantine_release_gate_state_fences"));
    for state in 0..DISPOSITION_STATES {
        fences = fences
            + centered_cube(
                format!("{PREFIX}_quarantine_release_state_fence_{state}"),
                GATE_X - 50.0,
                5.0,
                18.0,
            )
            .translate(
                0.0,
                centered_index(state, DISPOSITION_STATES, 42.0) + 20.0,
                GATE_Z / 2.0 + 9.0,
            );
    }
    fences
}

fn gate_status_tokens() -> Part {
    let release = centered_cube(
        format!("{PREFIX}_release_gate_square_token"),
        34.0,
        24.0,
        12.0,
    )
    .translate(-GATE_X / 2.0 + 36.0, 42.0, GATE_Z / 2.0 + 6.0);
    let quarantine = centered_cube(
        format!("{PREFIX}_quarantine_gate_diamond_token"),
        30.0,
        30.0,
        12.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-GATE_X / 2.0 + 36.0, 0.0, GATE_Z / 2.0 + 6.0);
    let reject = centered_cube(
        format!("{PREFIX}_reject_gate_cross_token_a"),
        36.0,
        8.0,
        12.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-GATE_X / 2.0 + 36.0, -42.0, GATE_Z / 2.0 + 6.0)
        + centered_cube(
            format!("{PREFIX}_reject_gate_cross_token_b"),
            36.0,
            8.0,
            12.0,
        )
        .rotate(0.0, 0.0, -45.0)
        .translate(-GATE_X / 2.0 + 36.0, -42.0, GATE_Z / 2.0 + 6.0);
    release + quarantine + reject
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_camera_bridge_left_post"),
        36.0,
        70.0,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0 + 70.0, 0.0, 0.0);
    let right_post = centered_cube(
        format!("{PREFIX}_camera_bridge_right_post"),
        36.0,
        70.0,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 70.0, 0.0, 0.0);
    let rear_post = centered_cube(
        format!("{PREFIX}_camera_bridge_rear_center_post"),
        34.0,
        70.0,
        BRIDGE_Z - 30.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0 - 64.0, -15.0);
    let beam = centered_cube(
        format!("{PREFIX}_evidence_camera_bridge_beam"),
        BRIDGE_X - 108.0,
        34.0,
        30.0,
    )
    .translate(0.0, 0.0, BRIDGE_Z / 2.0 - 15.0);
    left_post
        + right_post
        + rear_post
        + beam
        + camera_pod_markers()
        + evidence_led_bars()
        + camera_mount_bores()
}

fn camera_pod_markers() -> Part {
    let mut pods = Part::empty(format!("{PREFIX}_camera_evidence_pod_markers"));
    for i in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("{PREFIX}_camera_pod_window_marker_{i}"),
                64.0,
                9.0,
                8.0,
            )
            .translate(
                centered_index(i, CAMERA_PODS, 220.0),
                0.0,
                BRIDGE_Z / 2.0 + 4.0,
            ) + centered_cylinder(
                format!("{PREFIX}_camera_pod_lens_envelope_{i}"),
                16.0,
                16.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CAMERA_PODS, 220.0),
                -12.0,
                BRIDGE_Z / 2.0 - 18.0,
            );
    }
    pods
}

fn evidence_led_bars() -> Part {
    let mut bars = Part::empty(format!("{PREFIX}_evidence_led_bars"));
    for i in 0..EVIDENCE_LED_BARS {
        bars = bars
            + centered_cube(format!("{PREFIX}_evidence_led_bar_{i}"), 150.0, 7.0, 7.0).translate(
                centered_index(i, EVIDENCE_LED_BARS, 250.0),
                26.0,
                BRIDGE_Z / 2.0 - 30.0,
            );
    }
    bars
}

fn camera_mount_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_camera_bridge_mount_bores"));
    for i in 0..CAMERA_PODS {
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_camera_mount_bore_marker_{i}"),
                4.0,
                40.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CAMERA_PODS, 220.0),
                0.0,
                BRIDGE_Z / 2.0 - 15.0,
            );
    }
    bores
}

fn robotic_service_datums() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_robotic_service_datum_plate"),
        DATUM_X,
        DATUM_Y,
        DATUM_Z,
    );
    base - datum_cone_pockets() + service_rails() + robot_teach_pad_grid() + tool_clearance_gauges()
}

fn datum_cone_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_robotic_service_datum_cone_pockets"));
    for i in 0..ROBOTIC_SERVICE_DATUMS {
        pockets = pockets
            + centered_cylinder(format!("{PREFIX}_robotic_datum_pocket_{i}"), 10.0, 22.0, 32)
                .translate(
                    centered_index(i % 4, 4, 70.0),
                    centered_index(i / 4, 2, 58.0),
                    DATUM_Z / 2.0 - 11.0,
                );
    }
    pockets
}

fn service_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_robotic_service_rails"));
    for i in 0..SERVICE_RAILS {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_robotic_service_rail_{i}"),
                DATUM_X - 52.0,
                12.0,
                26.0,
            )
            .translate(
                0.0,
                centered_index(i, SERVICE_RAILS, 92.0),
                DATUM_Z / 2.0 + 13.0,
            );
    }
    rails
}

fn robot_teach_pad_grid() -> Part {
    let mut pads = Part::empty(format!("{PREFIX}_robot_teach_pad_grid"));
    for i in 0..ROBOTIC_SERVICE_DATUMS {
        pads = pads
            + centered_cube(format!("{PREFIX}_robot_teach_pad_{i}"), 30.0, 18.0, 6.0).translate(
                centered_index(i % 4, 4, 70.0),
                centered_index(i / 4, 2, 58.0),
                DATUM_Z / 2.0 + 3.0,
            );
    }
    pads
}

fn tool_clearance_gauges() -> Part {
    let mut gauges = Part::empty(format!("{PREFIX}_robot_tool_clearance_gauges"));
    for i in 0..TOOL_CLEARANCE_GAUGES {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_robot_tool_clearance_gauge_{i}"),
                16.0,
                72.0,
                8.0,
            )
            .translate(
                centered_index(i, TOOL_CLEARANCE_GAUGES, 54.0),
                DATUM_Y / 2.0 - 28.0,
                DATUM_Z / 2.0 + 4.0,
            );
    }
    gauges
}

fn rectangular_frame(
    name: impl Into<String>,
    outer_x: f64,
    y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let top = centered_cube(format!("{name}_top"), outer_x, y, (outer_z - inner_z) / 2.0)
        .translate(0.0, 0.0, inner_z / 2.0 + (outer_z - inner_z) / 4.0);
    let bottom = centered_cube(
        format!("{name}_bottom"),
        outer_x,
        y,
        (outer_z - inner_z) / 2.0,
    )
    .translate(0.0, 0.0, -inner_z / 2.0 - (outer_z - inner_z) / 4.0);
    let left = centered_cube(
        format!("{name}_left"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(-inner_x / 2.0 - (outer_x - inner_x) / 4.0, 0.0, 0.0);
    let right = centered_cube(
        format!("{name}_right"),
        (outer_x - inner_x) / 2.0,
        y,
        inner_z,
    )
    .translate(inner_x / 2.0 + (outer_x - inner_x) / 4.0, 0.0, 0.0);

    top + bottom + left + right
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn layout_regions_fit_without_core_collisions() {
        assert_design_constraints();
    }

    #[test]
    fn receiver_and_debag_force_controls_are_dimensioned_for_closed_material_flow() {
        assert!(STATION_X <= 1600.0);
        assert!(STATION_Y <= 1000.0);
        assert!(TOTE_CLEAR_X >= 340.0);
        assert!(TOTE_CLEAR_Y >= 160.0);
        assert_eq!(FORCE_DATUM_RAILS, 2);
        assert!(PEEL_CLAMP_STATIONS >= 6);
        assert!(FORCE_TICKS >= PEEL_CLAMP_STATIONS);
    }

    #[test]
    fn static_particle_and_airflow_counts_track_validation_intent() {
        assert_eq!(STATIC_PROBE_POCKETS, 8);
        assert_eq!(GROUND_BUTTONS, 4);
        assert_eq!(
            PARTICLE_COUPONS,
            PARTICLE_COUPON_ROWS * PARTICLE_COUPON_COLS
        );
        assert_eq!(PARTICLE_COUPONS, 20);
        assert_eq!(VANE_COUNT, VANE_ROWS * VANE_COLS);
        assert_eq!(VANE_COUNT, 18);
        assert!(SMOKE_WAND_PORTS >= 4);
    }

    #[test]
    fn custody_quarantine_release_and_robot_features_are_explicit() {
        assert!(BARCODE_LANDS >= 10);
        assert!(COA_LANDS >= 4);
        assert_eq!(DISPOSITION_STATES, 3);
        assert_eq!(GATE_SLOTS_PER_STATE, 4);
        assert_eq!(ROBOTIC_SERVICE_DATUMS, 8);
        assert_eq!(SERVICE_RAILS, 2);
        assert!(BRIDGE_UNDERSIDE_Z > RECEIVER_Z + BASE_Z + 85.0);
    }

    #[test]
    fn limitations_do_not_claim_process_release_or_sop_status() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
        assert!(LIMITATIONS.contains(&"not_a_cleanroom_release_specification"));
        assert!(LIMITATIONS.contains(&"not_a_static_discharge_sop"));
        assert!(LIMITATIONS.contains(&"not_a_particle_limit_claim"));
    }
}
