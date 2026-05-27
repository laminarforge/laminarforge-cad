use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed ECM/coating uniformity witness coupon station.
//
// Intent:
// - Validate repeatable ECM/coating application across a 20-position multi-chip
//   cassette without opening the cassette workflow.
// - Keep edge/center coating controls, matched witness coupons, rinse recovery
//   capture, barcode identity, closed reagent connections, and evidence
//   imaging datums mechanically explicit.
// - This is fixture/interface CAD only. It does not define ECM chemistry,
//   coating acceptance limits, rinse analytical methods, or sterility claims.

const OUTPUTS: [&str; 13] = [
    "output/closed_ecm_coating_uniformity_witness_coupon_station_base_leak_tray_deck.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_multi_chip_cassette_datum_nest.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_edge_center_coating_control_manifold.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_witness_coupon_gradient_grid.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_rinse_recovery_capture_wells.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_barcode_identity_status_lanes.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_edge_center_reference_coupon_bank.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_closed_reagent_connector_bulkhead.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_applicator_height_sweep_gauge.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_imaging_fiducial_light_frame.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_rinse_waste_quarantine_tray.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_robot_service_keepouts.stl",
    "output/closed_ecm_coating_uniformity_witness_coupon_station_assembly.stl",
];

const FEATURE_NAMES: [&str; 12] = [
    "base_leak_tray_deck",
    "multi_chip_cassette_datum_nest",
    "edge_center_coating_control_manifold",
    "witness_coupon_gradient_grid",
    "rinse_recovery_capture_wells",
    "barcode_identity_status_lanes",
    "edge_center_reference_coupon_bank",
    "closed_reagent_connector_bulkhead",
    "applicator_height_sweep_gauge",
    "imaging_fiducial_light_frame",
    "rinse_waste_quarantine_tray",
    "robot_service_keepouts",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITIONS: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITIONS: usize = 14;
const CENTER_POSITIONS: usize = CASSETTE_POSITIONS - EDGE_POSITIONS;

const WITNESS_COUPONS: usize = CASSETTE_POSITIONS;
const EDGE_WITNESS_COUPONS: usize = EDGE_POSITIONS;
const CENTER_WITNESS_COUPONS: usize = CENTER_POSITIONS;
const RINSE_RECOVERY_WELLS: usize = CASSETTE_POSITIONS;
const EDGE_CONTROL_CHANNELS: usize = 4;
const CENTER_CONTROL_CHANNELS: usize = 2;
const METERING_NEEDLES: usize = 12;
const REFERENCE_COUPONS: usize = 10;
const BARCODE_ID_LANDS: usize = 8;
const STATUS_LANES: usize = 4;
const BULKHEAD_PORTS: usize = 6;
const WASTE_QUARANTINE_CUPS: usize = 4;

const CHIP_GAP_X: f64 = 8.0;
const CHIP_GAP_Y: f64 = 6.0;
const CASSETTE_MARGIN_X: f64 = 36.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP_X;
const ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP_Y;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GAP_X;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GAP_Y;

const DECK_X: f64 = 1680.0;
const DECK_Y: f64 = 1120.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 34.0;
const SOCKET_DEPTH: f64 = 6.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;

const NEST_CENTER: (f64, f64) = (-80.0, 100.0);
const NEST_X: f64 = CASSETTE_X + 78.0;
const NEST_Y: f64 = CASSETTE_Y + 78.0;
const NEST_Z: f64 = 34.0;
const NEST_SOCKET_DEPTH: f64 = 9.0;
const NEST_RAIL_W: f64 = 16.0;
const NEST_RAIL_Z: f64 = 28.0;
const CHIP_RELIEF_Z: f64 = 8.0;
const DATUM_PIN_D: f64 = 7.2;

const CONTROL_CENTER: (f64, f64) = (540.0, 235.0);
const CONTROL_X: f64 = 445.0;
const CONTROL_Y: f64 = 290.0;
const CONTROL_Z: f64 = 52.0;
const EDGE_CHANNEL_D: f64 = 7.0;
const CENTER_CHANNEL_D: f64 = 5.0;
const NEEDLE_BOSS_D: f64 = 24.0;
const NEEDLE_BORE_D: f64 = 4.0;

const WITNESS_CENTER: (f64, f64) = (-580.0, -360.0);
const WITNESS_BANK_X: f64 = 430.0;
const WITNESS_BANK_Y: f64 = 300.0;
const WITNESS_BANK_Z: f64 = 28.0;
const COUPON_SLOT_X: f64 = 62.0;
const COUPON_SLOT_Y: f64 = 34.0;
const COUPON_SLOT_DEPTH: f64 = 13.0;
const COUPON_PITCH_X: f64 = 88.0;
const COUPON_PITCH_Y: f64 = 55.0;

const RINSE_CENTER: (f64, f64) = (540.0, -185.0);
const RINSE_BANK_X: f64 = 460.0;
const RINSE_BANK_Y: f64 = 285.0;
const RINSE_BANK_Z: f64 = 42.0;
const RINSE_WELL_D: f64 = 24.0;
const RINSE_WELL_DEPTH: f64 = 25.0;
const RINSE_PITCH_X: f64 = 92.0;
const RINSE_PITCH_Y: f64 = 52.0;

const STATUS_CENTER: (f64, f64) = (140.0, -445.0);
const STATUS_X: f64 = 600.0;
const STATUS_Y: f64 = 116.0;
const STATUS_Z: f64 = 14.0;
const BARCODE_LAND_X: f64 = 118.0;
const BARCODE_LAND_Y: f64 = 22.0;
const STATUS_LANE_X: f64 = 128.0;
const STATUS_LANE_Y: f64 = 52.0;

const REFERENCE_CENTER: (f64, f64) = (-655.0, -50.0);
const REFERENCE_X: f64 = 245.0;
const REFERENCE_Y: f64 = 210.0;
const REFERENCE_Z: f64 = 26.0;
const REFERENCE_COUPON_D: f64 = 25.0;

const BULKHEAD_CENTER: (f64, f64) = (-690.0, 235.0);
const BULKHEAD_X: f64 = 220.0;
const BULKHEAD_Y: f64 = 330.0;
const BULKHEAD_Z: f64 = 150.0;
const BULKHEAD_PORT_D: f64 = 24.0;
const BULKHEAD_COLLAR_D: f64 = 42.0;
const BULKHEAD_PORT_PITCH_Y: f64 = 47.0;

const APPLICATOR_CENTER: (f64, f64) = (-80.0, 460.0);
const APPLICATOR_X: f64 = 930.0;
const APPLICATOR_Y: f64 = 90.0;
const APPLICATOR_Z: f64 = 32.0;
const SWEEP_GAUGE_COUNT: usize = 7;
const EDGE_SHIM_COUNT: usize = 4;
const CENTER_SHIM_COUNT: usize = 3;

const IMAGING_FRAME_X: f64 = NEST_X + 126.0;
const IMAGING_FRAME_Y: f64 = 176.0;
const IMAGING_FRAME_Z: f64 = 62.0;
const IMAGING_RAIL_W: f64 = 16.0;
const FIDUCIAL_TARGETS: usize = 6;
const LIGHT_BARS: usize = 3;

const WASTE_CENTER: (f64, f64) = (110.0, -310.0);
const WASTE_X: f64 = 300.0;
const WASTE_Y: f64 = 136.0;
const WASTE_Z: f64 = 38.0;
const WASTE_CUP_D: f64 = 32.0;
const WASTE_CUP_DEPTH: f64 = 24.0;

const KEEP_OUT_X: f64 = 1584.0;
const KEEP_OUT_Y: f64 = 1032.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;
const ROBOT_APPROACH_CLEARANCE: f64 = 330.0;
const BULKHEAD_SERVICE_CLEARANCE: f64 = 220.0;
const CAMERA_CLEARANCE_Z: f64 = 172.0;
const APPLICATOR_LIFT_CLEARANCE: f64 = 238.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = base_leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let nest = multi_chip_cassette_datum_nest();
    export(OUTPUTS[1], &nest);

    let control = edge_center_coating_control_manifold();
    export(OUTPUTS[2], &control);

    let witness = witness_coupon_gradient_grid();
    export(OUTPUTS[3], &witness);

    let rinse = rinse_recovery_capture_wells();
    export(OUTPUTS[4], &rinse);

    let status = barcode_identity_status_lanes();
    export(OUTPUTS[5], &status);

    let reference = edge_center_reference_coupon_bank();
    export(OUTPUTS[6], &reference);

    let bulkhead = closed_reagent_connector_bulkhead();
    export(OUTPUTS[7], &bulkhead);

    let applicator = applicator_height_sweep_gauge();
    export(OUTPUTS[8], &applicator);

    let imaging = imaging_fiducial_light_frame();
    export(OUTPUTS[9], &imaging);

    let waste = rinse_waste_quarantine_tray();
    export(OUTPUTS[10], &waste);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_Z / 2.0)
        + control.translate(CONTROL_CENTER.0, CONTROL_CENTER.1, DECK_Z + CONTROL_Z / 2.0)
        + witness.translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            DECK_Z + WITNESS_BANK_Z / 2.0,
        )
        + rinse.translate(RINSE_CENTER.0, RINSE_CENTER.1, DECK_Z + RINSE_BANK_Z / 2.0)
        + status.translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z + STATUS_Z / 2.0)
        + reference.translate(
            REFERENCE_CENTER.0,
            REFERENCE_CENTER.1,
            DECK_Z + REFERENCE_Z / 2.0,
        )
        + bulkhead.translate(
            BULKHEAD_CENTER.0,
            BULKHEAD_CENTER.1,
            DECK_Z + BULKHEAD_Z / 2.0,
        )
        + applicator.translate(
            APPLICATOR_CENTER.0,
            APPLICATOR_CENTER.1,
            DECK_Z + APPLICATOR_Z / 2.0,
        )
        + imaging.translate(
            NEST_CENTER.0,
            NEST_CENTER.1,
            DECK_Z + NEST_Z + IMAGING_FRAME_Z / 2.0 + 8.0,
        )
        + waste.translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z / 2.0)
        + keepouts.translate(0.0, 0.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);

    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed ECM/coating uniformity witness coupon station:");
    println!(
        "  Cassette map:               {CASSETTE_ROWS} rows x {CASSETTE_COLS} columns = {CASSETTE_POSITIONS} Rev C chip positions"
    );
    println!(
        "  Coating controls:           {EDGE_CONTROL_CHANNELS} edge channels, {CENTER_CONTROL_CHANNELS} center channels, {METERING_NEEDLES} metering needle bosses"
    );
    println!(
        "  Witness coupons:            {WITNESS_COUPONS} matched coupons ({EDGE_WITNESS_COUPONS} edge, {CENTER_WITNESS_COUPONS} center) plus {REFERENCE_COUPONS} reference controls"
    );
    println!(
        "  Rinse recovery:             {RINSE_RECOVERY_WELLS} indexed recovery wells and {WASTE_QUARANTINE_CUPS} segregated quarantine cups"
    );
    println!(
        "  Identity/evidence:          {BARCODE_ID_LANDS} barcode identity lands, {STATUS_LANES} status lanes, {BULKHEAD_PORTS} closed reagent ports, {FIDUCIAL_TARGETS} fiducials"
    );
    println!(
        "  Service clearances:         robot approach {ROBOT_APPROACH_CLEARANCE:.0}mm, bulkhead pull {BULKHEAD_SERVICE_CLEARANCE:.0}mm, camera {CAMERA_CLEARANCE_Z:.0}mm, applicator lift {APPLICATOR_LIFT_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
    assert_eq!(CASSETTE_POSITIONS, 20);
    assert_eq!(EDGE_POSITIONS + CENTER_POSITIONS, CASSETTE_POSITIONS);
    assert_eq!(WITNESS_COUPONS, CASSETTE_POSITIONS);
    assert_eq!(RINSE_RECOVERY_WELLS, CASSETTE_POSITIONS);
    assert_eq!(EDGE_WITNESS_COUPONS, EDGE_POSITIONS);
    assert_eq!(CENTER_WITNESS_COUPONS, CENTER_POSITIONS);
    assert!(NEST_Z > REVC_TOTAL_HEIGHT + 12.0);
    assert!(COUPON_SLOT_X < COUPON_PITCH_X);
    assert!(RINSE_WELL_D + 18.0 < RINSE_PITCH_X);
    assert!(camera_clearance_above_cassette() > 110.0);
    assert!(bulkhead_port_span_y() + BULKHEAD_COLLAR_D < BULKHEAD_Y);

    let rects = layout_rects();
    for rect in rects {
        assert!(rect.fits_inside(), "{} does not fit on deck", rect.name);
    }

    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn layout_rects() -> [Rect; 9] {
    [
        Rect {
            name: "multi_chip_cassette_datum_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "edge_center_coating_control_manifold",
            center: CONTROL_CENTER,
            x: CONTROL_X,
            y: CONTROL_Y,
        },
        Rect {
            name: "witness_coupon_gradient_grid",
            center: WITNESS_CENTER,
            x: WITNESS_BANK_X,
            y: WITNESS_BANK_Y,
        },
        Rect {
            name: "rinse_recovery_capture_wells",
            center: RINSE_CENTER,
            x: RINSE_BANK_X,
            y: RINSE_BANK_Y,
        },
        Rect {
            name: "barcode_identity_status_lanes",
            center: STATUS_CENTER,
            x: STATUS_X,
            y: STATUS_Y,
        },
        Rect {
            name: "edge_center_reference_coupon_bank",
            center: REFERENCE_CENTER,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Rect {
            name: "closed_reagent_connector_bulkhead",
            center: BULKHEAD_CENTER,
            x: BULKHEAD_X,
            y: BULKHEAD_Y,
        },
        Rect {
            name: "applicator_height_sweep_gauge",
            center: APPLICATOR_CENTER,
            x: APPLICATOR_X,
            y: APPLICATOR_Y,
        },
        Rect {
            name: "rinse_waste_quarantine_tray",
            center: WASTE_CENTER,
            x: WASTE_X,
            y: WASTE_Y,
        },
    ]
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "ecm_uniformity_witness_base_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let nest_socket = deck_socket(
        "ecm_uniformity_witness_cassette_nest_socket",
        NEST_CENTER,
        NEST_X + 24.0,
        NEST_Y + 24.0,
    );
    let control_socket = deck_socket(
        "ecm_uniformity_witness_control_manifold_socket",
        CONTROL_CENTER,
        CONTROL_X + 20.0,
        CONTROL_Y + 20.0,
    );
    let witness_socket = deck_socket(
        "ecm_uniformity_witness_coupon_grid_socket",
        WITNESS_CENTER,
        WITNESS_BANK_X + 22.0,
        WITNESS_BANK_Y + 22.0,
    );
    let rinse_socket = deck_socket(
        "ecm_uniformity_witness_rinse_capture_socket",
        RINSE_CENTER,
        RINSE_BANK_X + 20.0,
        RINSE_BANK_Y + 20.0,
    );
    let wet_gutter = centered_cube(
        "ecm_uniformity_witness_wet_side_gutter",
        DECK_X - 160.0,
        18.0,
        8.0,
    )
    .translate(0.0, WASTE_CENTER.1 - WASTE_Y / 2.0 - 30.0, DECK_Z - 3.0);
    let drain = centered_cylinder("ecm_uniformity_witness_tray_drain", DRAIN_D / 2.0, 46.0, 36)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0);

    deck - nest_socket
        - control_socket
        - witness_socket
        - rinse_socket
        - wet_gutter
        - drain
        - deck_mounting_holes()
        + perimeter_rims()
        + deck_datum_targets()
        + zone_separator_lands()
}

fn deck_socket(name: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(name, x, y, SOCKET_DEPTH + 0.4).translate(
        center.0,
        center.1,
        DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn deck_mounting_holes() -> Part {
    let mut holes = Part::empty("ecm_uniformity_witness_deck_mounting_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 62.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 62.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 62.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 62.0, 0.0),
        (DECK_X / 2.0 - 62.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("ecm_uniformity_witness_deck_m6_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube("ecm_uniformity_witness_rear_leak_rim", DECK_X, RIM_W, RIM_Z)
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube("ecm_uniformity_witness_left_leak_rim", RIM_W, DECK_Y, RIM_Z)
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "ecm_uniformity_witness_right_leak_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let front_left = centered_cube(
        "ecm_uniformity_witness_front_left_low_load_lip",
        520.0,
        RIM_W,
        16.0,
    )
    .translate(
        -DECK_X / 2.0 + 280.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 8.0,
    );
    let front_right = centered_cube(
        "ecm_uniformity_witness_front_right_low_load_lip",
        520.0,
        RIM_W,
        16.0,
    )
    .translate(
        DECK_X / 2.0 - 280.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 8.0,
    );

    rear + left + right + front_left + front_right
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("ecm_uniformity_witness_station_datum_targets");
    for (i, (x, y)) in [
        (
            NEST_CENTER.0 - NEST_X / 2.0 + 42.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 40.0,
        ),
        (
            NEST_CENTER.0 + NEST_X / 2.0 - 42.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 40.0,
        ),
        (
            NEST_CENTER.0 - NEST_X / 2.0 + 42.0,
            NEST_CENTER.1 - NEST_Y / 2.0 + 40.0,
        ),
        (
            CONTROL_CENTER.0 + CONTROL_X / 2.0 - 36.0,
            CONTROL_CENTER.1 + CONTROL_Y / 2.0 - 34.0,
        ),
        (
            WITNESS_CENTER.0 - WITNESS_BANK_X / 2.0 + 34.0,
            WITNESS_CENTER.1 + WITNESS_BANK_Y / 2.0 - 34.0,
        ),
        (
            RINSE_CENTER.0 + RINSE_BANK_X / 2.0 - 34.0,
            RINSE_CENTER.1 - RINSE_BANK_Y / 2.0 + 34.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let puck = centered_cylinder(
            format!("ecm_uniformity_witness_station_datum_puck_{i}"),
            10.0,
            3.0,
            40,
        )
        .translate(*x, *y, DECK_Z + 1.5);
        let center = centered_cylinder(
            format!("ecm_uniformity_witness_station_datum_center_cut_{i}"),
            2.0,
            4.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 1.5);
        targets = targets + (puck - center);
    }
    targets
}

fn zone_separator_lands() -> Part {
    let cassette_zone = centered_cube(
        "ecm_uniformity_witness_cassette_zone_label_land",
        NEST_X,
        10.0,
        3.0,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - NEST_Y / 2.0 - 20.0,
        DECK_Z + 1.5,
    );
    let controls_zone = centered_cube(
        "ecm_uniformity_witness_edge_center_control_zone_land",
        CONTROL_X,
        10.0,
        3.0,
    )
    .translate(
        CONTROL_CENTER.0,
        CONTROL_CENTER.1 - CONTROL_Y / 2.0 - 20.0,
        DECK_Z + 1.5,
    );
    let witness_zone = centered_cube(
        "ecm_uniformity_witness_coupon_zone_label_land",
        WITNESS_BANK_X,
        10.0,
        3.0,
    )
    .translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1 + WITNESS_BANK_Y / 2.0 + 18.0,
        DECK_Z + 1.5,
    );
    let rinse_zone = centered_cube(
        "ecm_uniformity_witness_rinse_recovery_zone_land",
        RINSE_BANK_X,
        10.0,
        3.0,
    )
    .translate(
        RINSE_CENTER.0,
        RINSE_CENTER.1 + RINSE_BANK_Y / 2.0 + 18.0,
        DECK_Z + 1.5,
    );

    cassette_zone + controls_zone + witness_zone + rinse_zone
}

fn multi_chip_cassette_datum_nest() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_multi_chip_cassette_nest_base",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cassette_socket = centered_cube(
        "ecm_uniformity_witness_multi_chip_cassette_socket",
        CASSETTE_X + 12.0,
        CASSETTE_Y + 12.0,
        NEST_SOCKET_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 - NEST_SOCKET_DEPTH / 2.0 + 0.5);
    let side_tube_clearance = centered_cube(
        "ecm_uniformity_witness_cassette_side_tube_clearance",
        58.0,
        CASSETTE_Y + 74.0,
        NEST_Z + 2.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 46.0), 0.0, 2.0);
    let recovery_slot = centered_cube(
        "ecm_uniformity_witness_cassette_front_rinse_recovery_slot",
        CASSETTE_X - 72.0,
        28.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 34.0), 2.0);

    base - cassette_socket - side_tube_clearance - recovery_slot - cassette_chip_reliefs()
        + nest_datum_rails()
        + nest_datum_pins()
        + cassette_edge_center_marks()
        + row_latch_bosses()
}

fn cassette_chip_reliefs() -> Part {
    let mut cuts = Part::empty("ecm_uniformity_witness_cassette_chip_reliefs");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = cassette_position_xy(row, col, PITCH_X, PITCH_Y);
            cuts = cuts
                + centered_cube(
                    format!("ecm_uniformity_witness_chip_relief_r{row}_c{col}"),
                    REVC_CHIP_LENGTH - 16.0,
                    REVC_CHIP_WIDTH - 14.0,
                    CHIP_RELIEF_Z,
                )
                .translate(x, y, NEST_Z / 2.0 - CHIP_RELIEF_Z / 2.0 + 0.5);
        }
    }
    cuts
}

fn nest_datum_rails() -> Part {
    let left = centered_cube(
        "ecm_uniformity_witness_cassette_left_hard_datum_rail",
        NEST_RAIL_W,
        NEST_Y,
        NEST_RAIL_Z,
    )
    .translate(
        -(NEST_X / 2.0 - NEST_RAIL_W / 2.0),
        0.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let back = centered_cube(
        "ecm_uniformity_witness_cassette_back_hard_datum_rail",
        NEST_X,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        NEST_Y / 2.0 - NEST_RAIL_W / 2.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let front_soft = centered_cube(
        "ecm_uniformity_witness_cassette_front_soft_capture_rail",
        NEST_X * 0.72,
        NEST_RAIL_W,
        NEST_RAIL_Z * 0.64,
    )
    .translate(
        20.0,
        -(NEST_Y / 2.0 - NEST_RAIL_W / 2.0),
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.32,
    );
    let right_soft = centered_cube(
        "ecm_uniformity_witness_cassette_right_soft_capture_rail",
        NEST_RAIL_W,
        NEST_Y * 0.72,
        NEST_RAIL_Z * 0.64,
    )
    .translate(
        NEST_X / 2.0 - NEST_RAIL_W / 2.0,
        -8.0,
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.32,
    );

    left + back + front_soft + right_soft
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("ecm_uniformity_witness_cassette_datum_pins");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 28.0), CASSETTE_Y / 2.0 - 30.0),
        (CASSETTE_X / 2.0 - 42.0, CASSETTE_Y / 2.0 - 30.0),
        (-(CASSETTE_X / 2.0 - 28.0), -(CASSETTE_Y / 2.0 - 34.0)),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("ecm_uniformity_witness_cassette_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0 + 3.0,
            10.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        let pilot = centered_cylinder(
            format!("ecm_uniformity_witness_cassette_datum_pin_pilot_{i}"),
            DATUM_PIN_D / 2.0,
            12.0,
            28,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        pins = pins + (boss - pilot);
    }
    pins
}

fn cassette_edge_center_marks() -> Part {
    let mut marks = Part::empty("ecm_uniformity_witness_cassette_edge_center_marks");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = cassette_position_xy(row, col, PITCH_X, PITCH_Y);
            let idx = row * CASSETTE_COLS + col;
            if is_edge_position(row, col) {
                marks = marks
                    + centered_cube(
                        format!("ecm_uniformity_witness_edge_position_mark_{idx:02}"),
                        22.0,
                        8.0,
                        4.0,
                    )
                    .translate(
                        x - REVC_CHIP_LENGTH / 2.0 + 22.0,
                        y + REVC_CHIP_WIDTH / 2.0 - 16.0,
                        NEST_Z / 2.0 + 2.0,
                    );
            } else {
                marks = marks
                    + centered_cylinder(
                        format!("ecm_uniformity_witness_center_position_mark_{idx:02}"),
                        8.0,
                        4.0,
                        32,
                    )
                    .translate(
                        x - REVC_CHIP_LENGTH / 2.0 + 22.0,
                        y + REVC_CHIP_WIDTH / 2.0 - 16.0,
                        NEST_Z / 2.0 + 2.0,
                    );
            }
        }
    }
    marks
}

fn row_latch_bosses() -> Part {
    let mut bosses = Part::empty("ecm_uniformity_witness_cassette_row_latch_bosses");
    for row in 0..CASSETTE_ROWS {
        let y = cassette_position_y(row, PITCH_Y);
        for side in [-1.0, 1.0] {
            let x = side * (CASSETTE_X / 2.0 + 20.0);
            let boss = centered_cylinder(
                format!("ecm_uniformity_witness_row_{row}_latch_boss_{side}"),
                9.0,
                18.0,
                32,
            )
            .translate(x, y, NEST_Z / 2.0 + 9.0);
            let screw = centered_cylinder(
                format!("ecm_uniformity_witness_row_{row}_latch_screw_clearance_{side}"),
                2.0,
                20.0,
                20,
            )
            .translate(x, y, NEST_Z / 2.0 + 9.0);
            bosses = bosses + (boss - screw);
        }
    }
    bosses
}

fn edge_center_coating_control_manifold() -> Part {
    let body = centered_cube(
        "ecm_uniformity_witness_edge_center_control_manifold_body",
        CONTROL_X,
        CONTROL_Y,
        CONTROL_Z,
    );
    let mut channels = Part::empty("ecm_uniformity_witness_edge_center_control_channels");

    for channel in 0..EDGE_CONTROL_CHANNELS {
        let y = edge_control_channel_y(channel);
        channels = channels
            + centered_cylinder(
                format!("ecm_uniformity_witness_edge_loop_channel_{channel}"),
                EDGE_CHANNEL_D / 2.0,
                CONTROL_X + 8.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 4.0);
    }

    for channel in 0..CENTER_CONTROL_CHANNELS {
        let y = center_control_channel_y(channel);
        channels = channels
            + centered_cylinder(
                format!("ecm_uniformity_witness_center_loop_channel_{channel}"),
                CENTER_CHANNEL_D / 2.0,
                CONTROL_X + 8.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 13.0);
    }

    body - channels
        + metering_needle_bosses()
        + edge_center_selector_tabs()
        + pressure_gauge_lands()
}

fn metering_needle_bosses() -> Part {
    let mut bosses = Part::empty("ecm_uniformity_witness_metering_needle_bosses");
    for i in 0..METERING_NEEDLES {
        let row = i / 6;
        let col = i % 6;
        let x = (col as f64 - 2.5) * 58.0;
        let y = if row == 0 { 62.0 } else { -62.0 };
        let boss = centered_cylinder(
            format!("ecm_uniformity_witness_metering_needle_boss_{i}"),
            NEEDLE_BOSS_D / 2.0,
            16.0,
            36,
        )
        .translate(x, y, CONTROL_Z / 2.0 + 8.0);
        let bore = centered_cylinder(
            format!("ecm_uniformity_witness_metering_needle_bore_{i}"),
            NEEDLE_BORE_D / 2.0,
            18.0,
            24,
        )
        .translate(x, y, CONTROL_Z / 2.0 + 8.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn edge_center_selector_tabs() -> Part {
    let edge_in = centered_cube(
        "ecm_uniformity_witness_edge_loop_selector_land",
        150.0,
        28.0,
        8.0,
    )
    .translate(
        -CONTROL_X / 2.0 + 86.0,
        CONTROL_Y / 2.0 + 18.0,
        CONTROL_Z / 2.0 + 4.0,
    );
    let center_in = centered_cube(
        "ecm_uniformity_witness_center_loop_selector_land",
        150.0,
        28.0,
        8.0,
    )
    .translate(
        -CONTROL_X / 2.0 + 86.0,
        -CONTROL_Y / 2.0 - 18.0,
        CONTROL_Z / 2.0 + 4.0,
    );
    let balanced_out = centered_cube(
        "ecm_uniformity_witness_balanced_output_selector_land",
        180.0,
        28.0,
        8.0,
    )
    .translate(CONTROL_X / 2.0 - 104.0, 0.0, CONTROL_Z / 2.0 + 4.0);

    edge_in + center_in + balanced_out
}

fn pressure_gauge_lands() -> Part {
    let mut lands = Part::empty("ecm_uniformity_witness_pressure_gauge_lands");
    for i in 0..4 {
        let x = -144.0 + i as f64 * 96.0;
        lands = lands
            + centered_cylinder(
                format!("ecm_uniformity_witness_pressure_gauge_land_{i}"),
                14.0,
                5.0,
                36,
            )
            .translate(x, 0.0, CONTROL_Z / 2.0 + 2.5);
    }
    lands
}

fn witness_coupon_gradient_grid() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_coupon_gradient_grid_base",
        WITNESS_BANK_X,
        WITNESS_BANK_Y,
        WITNESS_BANK_Z,
    );
    let mut slots = Part::empty("ecm_uniformity_witness_coupon_slots");
    let mut lands = Part::empty("ecm_uniformity_witness_coupon_identity_lands");

    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = coupon_position_xy(row, col);
            let idx = row * CASSETTE_COLS + col;
            slots = slots
                + centered_cube(
                    format!("ecm_uniformity_witness_coupon_slot_r{row}_c{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    WITNESS_BANK_Z / 2.0 - COUPON_SLOT_DEPTH / 2.0 + 0.5,
                );

            let marker = if is_edge_position(row, col) {
                centered_cube(
                    format!("ecm_uniformity_witness_edge_coupon_barcode_land_{idx:02}"),
                    38.0,
                    8.0,
                    3.0,
                )
            } else {
                centered_cube(
                    format!("ecm_uniformity_witness_center_coupon_barcode_land_{idx:02}"),
                    28.0,
                    14.0,
                    3.0,
                )
            };
            lands = lands
                + marker.translate(
                    x,
                    y - COUPON_SLOT_Y / 2.0 - 10.0,
                    WITNESS_BANK_Z / 2.0 + 1.5,
                );
        }
    }

    base - slots + lands + coupon_retainer_bars() + witness_strip_chain_land()
}

fn coupon_retainer_bars() -> Part {
    let mut bars = Part::empty("ecm_uniformity_witness_coupon_retainer_bars");
    for row in 0..CASSETTE_ROWS {
        let y = coupon_position_y(row);
        bars = bars
            + centered_cube(
                format!("ecm_uniformity_witness_coupon_row_{row}_retainer_bar"),
                WITNESS_BANK_X - 52.0,
                6.0,
                6.0,
            )
            .translate(
                0.0,
                y + COUPON_SLOT_Y / 2.0 + 12.0,
                WITNESS_BANK_Z / 2.0 + 3.0,
            );
    }
    bars
}

fn witness_strip_chain_land() -> Part {
    let run_land = centered_cube(
        "ecm_uniformity_witness_coupon_strip_chain_of_custody_land",
        WITNESS_BANK_X - 70.0,
        18.0,
        4.0,
    )
    .translate(
        0.0,
        -WITNESS_BANK_Y / 2.0 + 22.0,
        WITNESS_BANK_Z / 2.0 + 2.0,
    );
    let seal_land = centered_cube(
        "ecm_uniformity_witness_coupon_strip_tamper_seal_land",
        132.0,
        20.0,
        4.0,
    )
    .translate(
        WITNESS_BANK_X / 2.0 - 92.0,
        WITNESS_BANK_Y / 2.0 - 24.0,
        WITNESS_BANK_Z / 2.0 + 2.0,
    );
    run_land + seal_land
}

fn rinse_recovery_capture_wells() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_rinse_recovery_capture_bank",
        RINSE_BANK_X,
        RINSE_BANK_Y,
        RINSE_BANK_Z,
    );
    let mut wells = Part::empty("ecm_uniformity_witness_rinse_recovery_wells");
    let mut overflows = Part::empty("ecm_uniformity_witness_rinse_recovery_overflow_lands");

    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = rinse_position_xy(row, col);
            let idx = row * CASSETTE_COLS + col;
            wells = wells
                + centered_cylinder(
                    format!("ecm_uniformity_witness_rinse_recovery_well_{idx:02}"),
                    RINSE_WELL_D / 2.0,
                    RINSE_WELL_DEPTH,
                    40,
                )
                .translate(x, y, RINSE_BANK_Z / 2.0 - RINSE_WELL_DEPTH / 2.0 + 0.5);
            overflows = overflows
                + centered_cube(
                    format!("ecm_uniformity_witness_rinse_recovery_index_land_{idx:02}"),
                    34.0,
                    8.0,
                    3.0,
                )
                .translate(
                    x,
                    y - RINSE_WELL_D / 2.0 - 10.0,
                    RINSE_BANK_Z / 2.0 + 1.5,
                );
        }
    }

    base - wells + overflows + edge_center_rinse_splitters() + rinse_pool_troughs()
}

fn edge_center_rinse_splitters() -> Part {
    let edge = centered_cube(
        "ecm_uniformity_witness_edge_rinse_splitter_land",
        RINSE_BANK_X - 56.0,
        14.0,
        5.0,
    )
    .translate(0.0, RINSE_BANK_Y / 2.0 - 38.0, RINSE_BANK_Z / 2.0 + 2.5);
    let center = centered_cube(
        "ecm_uniformity_witness_center_rinse_splitter_land",
        RINSE_BANK_X - 104.0,
        14.0,
        5.0,
    )
    .translate(0.0, -RINSE_BANK_Y / 2.0 + 38.0, RINSE_BANK_Z / 2.0 + 2.5);
    let mass_balance = centered_cube(
        "ecm_uniformity_witness_rinse_mass_balance_label_land",
        156.0,
        22.0,
        4.0,
    )
    .translate(RINSE_BANK_X / 2.0 - 104.0, 0.0, RINSE_BANK_Z / 2.0 + 2.0);

    edge + center + mass_balance
}

fn rinse_pool_troughs() -> Part {
    let mut troughs = Part::empty("ecm_uniformity_witness_rinse_pool_troughs");
    for i in 0..2 {
        let y = if i == 0 {
            RINSE_BANK_Y / 2.0 - 18.0
        } else {
            -RINSE_BANK_Y / 2.0 + 18.0
        };
        troughs = troughs
            + centered_cube(
                format!("ecm_uniformity_witness_rinse_pool_trough_{i}"),
                RINSE_BANK_X - 74.0,
                12.0,
                10.0,
            )
            .translate(0.0, y, RINSE_BANK_Z / 2.0 - 3.0);
    }
    troughs
}

fn barcode_identity_status_lanes() -> Part {
    let board = centered_cube(
        "ecm_uniformity_witness_barcode_identity_status_board",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut lands = Part::empty("ecm_uniformity_witness_barcode_identity_lands");

    for i in 0..BARCODE_ID_LANDS {
        let x = -STATUS_X / 2.0 + 58.0 + (i % 4) as f64 * 158.0;
        let y = if i < 4 { 25.0 } else { -29.0 };
        lands = lands
            + centered_cube(
                format!("ecm_uniformity_witness_barcode_identity_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, y, STATUS_Z / 2.0 + 2.0);
    }

    for lane in 0..STATUS_LANES {
        let x = -STATUS_X / 2.0 + 88.0 + lane as f64 * 140.0;
        lands = lands
            + centered_cube(
                format!("ecm_uniformity_witness_status_lane_{lane}"),
                STATUS_LANE_X,
                STATUS_LANE_Y,
                3.0,
            )
            .translate(x, 0.0, STATUS_Z / 2.0 + 1.5);
    }

    board + lands
}

fn edge_center_reference_coupon_bank() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_edge_center_reference_coupon_bank",
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    );
    let mut coupons = Part::empty("ecm_uniformity_witness_reference_coupon_lands");

    for i in 0..REFERENCE_COUPONS {
        let col = i % 2;
        let row = i / 2;
        let x = (col as f64 - 0.5) * 76.0;
        let y = REFERENCE_Y / 2.0 - 32.0 - row as f64 * 36.0;
        let coupon = centered_cylinder(
            format!("ecm_uniformity_witness_reference_coupon_land_{i}"),
            REFERENCE_COUPON_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y, REFERENCE_Z / 2.0 + 2.5);
        let id = centered_cube(
            format!("ecm_uniformity_witness_reference_coupon_id_land_{i}"),
            38.0,
            8.0,
            3.0,
        )
        .translate(x + 34.0, y, REFERENCE_Z / 2.0 + 1.5);
        coupons = coupons + coupon + id;
    }

    base + coupons + reference_pair_labels()
}

fn reference_pair_labels() -> Part {
    let edge_label = centered_cube(
        "ecm_uniformity_witness_reference_edge_control_label",
        REFERENCE_X - 36.0,
        10.0,
        3.0,
    )
    .translate(0.0, REFERENCE_Y / 2.0 - 8.0, REFERENCE_Z / 2.0 + 1.5);
    let center_label = centered_cube(
        "ecm_uniformity_witness_reference_center_control_label",
        REFERENCE_X - 36.0,
        10.0,
        3.0,
    )
    .translate(0.0, -REFERENCE_Y / 2.0 + 8.0, REFERENCE_Z / 2.0 + 1.5);

    edge_label + center_label
}

fn closed_reagent_connector_bulkhead() -> Part {
    let body = centered_cube(
        "ecm_uniformity_witness_closed_reagent_connector_bulkhead_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let mut ports = Part::empty("ecm_uniformity_witness_bulkhead_port_cuts");
    let mut collars = Part::empty("ecm_uniformity_witness_bulkhead_connector_collars");

    for i in 0..BULKHEAD_PORTS {
        let y = bulkhead_port_y(i);
        let z = -BULKHEAD_Z / 2.0 + 35.0 + (i % 2) as f64 * 32.0;
        ports = ports
            + centered_cylinder(
                format!("ecm_uniformity_witness_bulkhead_reagent_port_cut_{i}"),
                BULKHEAD_PORT_D / 2.0,
                BULKHEAD_X + 8.0,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, z);
        collars = collars
            + centered_cylinder(
                format!("ecm_uniformity_witness_bulkhead_reagent_port_collar_{i}"),
                BULKHEAD_COLLAR_D / 2.0,
                10.0,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-BULKHEAD_X / 2.0 - 5.0, y, z);
    }

    body - ports + collars + bulkhead_label_lands() + drip_shelf()
}

fn bulkhead_label_lands() -> Part {
    let mut labels = Part::empty("ecm_uniformity_witness_bulkhead_label_lands");
    for i in 0..BULKHEAD_PORTS {
        labels = labels
            + centered_cube(
                format!("ecm_uniformity_witness_bulkhead_port_label_{i}"),
                76.0,
                12.0,
                4.0,
            )
            .translate(
                BULKHEAD_X / 2.0 - 42.0,
                bulkhead_port_y(i),
                BULKHEAD_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn drip_shelf() -> Part {
    centered_cube(
        "ecm_uniformity_witness_bulkhead_closed_drip_shelf",
        BULKHEAD_X + 34.0,
        BULKHEAD_Y - 26.0,
        10.0,
    )
    .translate(0.0, 0.0, -BULKHEAD_Z / 2.0 - 5.0)
}

fn applicator_height_sweep_gauge() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_applicator_height_sweep_gauge_base",
        APPLICATOR_X,
        APPLICATOR_Y,
        APPLICATOR_Z,
    );
    let mut gauges = Part::empty("ecm_uniformity_witness_applicator_sweep_gauges");

    for i in 0..SWEEP_GAUGE_COUNT {
        let x = -APPLICATOR_X / 2.0 + 78.0 + i as f64 * 130.0;
        let height = 10.0 + i as f64 * 2.0;
        gauges = gauges
            + centered_cube(
                format!("ecm_uniformity_witness_applicator_height_step_{i}"),
                52.0,
                24.0,
                height,
            )
            .translate(x, 0.0, APPLICATOR_Z / 2.0 + height / 2.0);
    }

    for i in 0..EDGE_SHIM_COUNT {
        gauges = gauges
            + centered_cube(
                format!("ecm_uniformity_witness_edge_coating_shim_land_{i}"),
                72.0,
                12.0,
                5.0,
            )
            .translate(
                -APPLICATOR_X / 2.0 + 105.0 + i as f64 * 92.0,
                34.0,
                APPLICATOR_Z / 2.0 + 2.5,
            );
    }

    for i in 0..CENTER_SHIM_COUNT {
        gauges = gauges
            + centered_cube(
                format!("ecm_uniformity_witness_center_coating_shim_land_{i}"),
                72.0,
                12.0,
                5.0,
            )
            .translate(
                APPLICATOR_X / 2.0 - 289.0 + i as f64 * 92.0,
                -34.0,
                APPLICATOR_Z / 2.0 + 2.5,
            );
    }

    base + gauges + applicator_travel_slot()
}

fn applicator_travel_slot() -> Part {
    centered_cube(
        "ecm_uniformity_witness_applicator_travel_slot_cut",
        APPLICATOR_X - 104.0,
        16.0,
        APPLICATOR_Z + 2.0,
    )
    .translate(0.0, 0.0, 1.0)
}

fn imaging_fiducial_light_frame() -> Part {
    let front = centered_cube(
        "ecm_uniformity_witness_imaging_front_light_bar",
        IMAGING_FRAME_X,
        IMAGING_RAIL_W,
        IMAGING_FRAME_Z,
    )
    .translate(0.0, -IMAGING_FRAME_Y / 2.0 + IMAGING_RAIL_W / 2.0, 0.0);
    let rear = centered_cube(
        "ecm_uniformity_witness_imaging_rear_light_bar",
        IMAGING_FRAME_X,
        IMAGING_RAIL_W,
        IMAGING_FRAME_Z,
    )
    .translate(0.0, IMAGING_FRAME_Y / 2.0 - IMAGING_RAIL_W / 2.0, 0.0);
    let left = centered_cube(
        "ecm_uniformity_witness_imaging_left_fiducial_rail",
        IMAGING_RAIL_W,
        IMAGING_FRAME_Y,
        IMAGING_FRAME_Z,
    )
    .translate(-IMAGING_FRAME_X / 2.0 + IMAGING_RAIL_W / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "ecm_uniformity_witness_imaging_right_fiducial_rail",
        IMAGING_RAIL_W,
        IMAGING_FRAME_Y,
        IMAGING_FRAME_Z,
    )
    .translate(IMAGING_FRAME_X / 2.0 - IMAGING_RAIL_W / 2.0, 0.0, 0.0);

    front + rear + left + right + imaging_fiducials() + light_bar_lands()
}

fn imaging_fiducials() -> Part {
    let mut fiducials = Part::empty("ecm_uniformity_witness_imaging_fiducials");
    for (i, (x, y)) in [
        (-IMAGING_FRAME_X / 2.0 + 44.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (IMAGING_FRAME_X / 2.0 - 44.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (-IMAGING_FRAME_X / 2.0 + 44.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (IMAGING_FRAME_X / 2.0 - 44.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (-120.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (120.0, IMAGING_FRAME_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("ecm_uniformity_witness_imaging_fiducial_target_{i}"),
            10.0,
            4.0,
            36,
        )
        .translate(*x, *y, IMAGING_FRAME_Z / 2.0 + 2.0);
        let center = centered_cylinder(
            format!("ecm_uniformity_witness_imaging_fiducial_center_{i}"),
            2.0,
            5.0,
            24,
        )
        .translate(*x, *y, IMAGING_FRAME_Z / 2.0 + 2.0);
        fiducials = fiducials + (target - center);
    }
    fiducials
}

fn light_bar_lands() -> Part {
    let mut bars = Part::empty("ecm_uniformity_witness_imaging_light_bar_lands");
    for i in 0..LIGHT_BARS {
        let x = (i as f64 - 1.0) * 210.0;
        bars = bars
            + centered_cube(
                format!("ecm_uniformity_witness_imaging_light_bar_land_{i}"),
                126.0,
                14.0,
                5.0,
            )
            .translate(
                x,
                -IMAGING_FRAME_Y / 2.0 + 38.0,
                IMAGING_FRAME_Z / 2.0 + 2.5,
            );
    }
    bars
}

fn rinse_waste_quarantine_tray() -> Part {
    let base = centered_cube(
        "ecm_uniformity_witness_rinse_waste_quarantine_tray_base",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let mut cups = Part::empty("ecm_uniformity_witness_waste_quarantine_cups");
    for i in 0..WASTE_QUARANTINE_CUPS {
        let x = (i as f64 - 1.5) * 62.0;
        cups = cups
            + centered_cylinder(
                format!("ecm_uniformity_witness_waste_quarantine_cup_{i}"),
                WASTE_CUP_D / 2.0,
                WASTE_CUP_DEPTH,
                36,
            )
            .translate(x, 8.0, WASTE_Z / 2.0 - WASTE_CUP_DEPTH / 2.0 + 0.5);
    }
    let quarantine_land = centered_cube(
        "ecm_uniformity_witness_waste_quarantine_barcode_land",
        WASTE_X - 64.0,
        18.0,
        4.0,
    )
    .translate(0.0, -WASTE_Y / 2.0 + 24.0, WASTE_Z / 2.0 + 2.0);

    base - cups + quarantine_land + waste_overflow_notch()
}

fn waste_overflow_notch() -> Part {
    centered_cube(
        "ecm_uniformity_witness_waste_overflow_notch",
        WASTE_X - 70.0,
        12.0,
        10.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 20.0, WASTE_Z / 2.0 - 3.0)
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "ecm_uniformity_witness_robot_front_approach_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        "ecm_uniformity_witness_rear_service_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        "ecm_uniformity_witness_left_service_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "ecm_uniformity_witness_right_service_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    let camera = centered_cube(
        "ecm_uniformity_witness_camera_vertical_clearance_gauge",
        36.0,
        36.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, CAMERA_CLEARANCE_Z / 2.0);
    let applicator_lift = centered_cube(
        "ecm_uniformity_witness_applicator_lift_clearance_gauge",
        APPLICATOR_X,
        10.0,
        8.0,
    )
    .translate(
        APPLICATOR_CENTER.0,
        APPLICATOR_CENTER.1,
        APPLICATOR_LIFT_CLEARANCE,
    );
    let bulkhead_pull = centered_cube(
        "ecm_uniformity_witness_bulkhead_connector_pull_clearance_gauge",
        BULKHEAD_SERVICE_CLEARANCE,
        18.0,
        42.0,
    )
    .translate(
        BULKHEAD_CENTER.0 + BULKHEAD_X / 2.0 + BULKHEAD_SERVICE_CLEARANCE / 2.0,
        BULKHEAD_CENTER.1,
        21.0,
    );

    front + rear + left + right + camera + applicator_lift + bulkhead_pull
}

fn cassette_position_xy(row: usize, col: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    (
        cassette_position_x(col, pitch_x),
        cassette_position_y(row, pitch_y),
    )
}

fn cassette_position_x(col: usize, pitch_x: f64) -> f64 {
    (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * pitch_x
}

fn cassette_position_y(row: usize, pitch_y: f64) -> f64 {
    ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * pitch_y
}

fn coupon_position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X,
        coupon_position_y(row),
    )
}

fn coupon_position_y(row: usize) -> f64 {
    ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * COUPON_PITCH_Y
}

fn rinse_position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * RINSE_PITCH_X,
        ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * RINSE_PITCH_Y,
    )
}

fn is_edge_position(row: usize, col: usize) -> bool {
    row == 0 || row == CASSETTE_ROWS - 1 || col == 0 || col == CASSETTE_COLS - 1
}

fn edge_control_channel_y(channel: usize) -> f64 {
    match channel {
        0 => CONTROL_Y / 2.0 - 58.0,
        1 => CONTROL_Y / 2.0 - 92.0,
        2 => -CONTROL_Y / 2.0 + 58.0,
        _ => -CONTROL_Y / 2.0 + 92.0,
    }
}

fn center_control_channel_y(channel: usize) -> f64 {
    if channel == 0 {
        -28.0
    } else {
        28.0
    }
}

fn bulkhead_port_y(index: usize) -> f64 {
    -bulkhead_port_span_y() / 2.0 + index as f64 * BULKHEAD_PORT_PITCH_Y
}

fn bulkhead_port_span_y() -> f64 {
    BULKHEAD_PORT_PITCH_Y * (BULKHEAD_PORTS as f64 - 1.0)
}

fn camera_clearance_above_cassette() -> f64 {
    CAMERA_CLEARANCE_Z - (NEST_Z + REVC_TOTAL_HEIGHT)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_ecm_coating_uniformity_witness_coupon_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cassette_edge_center_map_matches_twenty_chip_workflow() {
        assert_eq!(CASSETTE_ROWS * CASSETTE_COLS, CASSETTE_POSITIONS);
        assert_eq!(CASSETTE_POSITIONS, 20);

        let mut edge_count = 0;
        let mut center_count = 0;
        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                if is_edge_position(row, col) {
                    edge_count += 1;
                } else {
                    center_count += 1;
                }
            }
        }

        assert_eq!(edge_count, EDGE_POSITIONS);
        assert_eq!(center_count, CENTER_POSITIONS);
    }

    #[test]
    fn witness_and_rinse_capacity_cover_every_cassette_position() {
        assert_eq!(WITNESS_COUPONS, CASSETTE_POSITIONS);
        assert_eq!(RINSE_RECOVERY_WELLS, CASSETTE_POSITIONS);
        assert_eq!(EDGE_WITNESS_COUPONS, EDGE_POSITIONS);
        assert_eq!(CENTER_WITNESS_COUPONS, CENTER_POSITIONS);
        assert!(COUPON_SLOT_X < COUPON_PITCH_X);
        assert!(RINSE_WELL_D < RINSE_PITCH_Y);
    }

    #[test]
    fn edge_center_controls_and_traceability_are_explicit() {
        assert_eq!(EDGE_CONTROL_CHANNELS, 4);
        assert_eq!(CENTER_CONTROL_CHANNELS, 2);
        assert_eq!(METERING_NEEDLES, 12);
        assert_eq!(REFERENCE_COUPONS, 10);
        assert_eq!(BARCODE_ID_LANDS, 8);
        assert_eq!(BULKHEAD_PORTS, 6);
        assert!(bulkhead_port_span_y() + BULKHEAD_COLLAR_D < BULKHEAD_Y);
    }

    #[test]
    fn major_modules_fit_without_overlaps() {
        assert_layout();
    }

    #[test]
    fn service_keepouts_clear_closed_fixture() {
        assert!(ROBOT_APPROACH_CLEARANCE >= 300.0);
        assert!(BULKHEAD_SERVICE_CLEARANCE >= 200.0);
        assert!(camera_clearance_above_cassette() > 110.0);
        assert!(APPLICATOR_LIFT_CLEARANCE > APPLICATOR_Z + NEST_Z);
        assert_eq!(SWEEP_GAUGE_COUNT, 7);
        assert_eq!(EDGE_SHIM_COUNT + CENTER_SHIM_COUNT, 7);
    }
}
