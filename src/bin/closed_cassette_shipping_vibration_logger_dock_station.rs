use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette shipping vibration logger dock station.
//
// Design intent:
// - Validate sealed chip cassette transport exposure using a cassette dock,
//   shock/vibration logger cradle, witness coupon rail, alignment stops,
//   tamper/seal inspection pockets, foam compression gauges, and CSG label
//   plaques.
// - Keep the station limited to packaging and evidence geometry. It does not
//   encode shipping acceptance limits, vibration qualification criteria, or
//   release instructions.

const OUTPUTS: [&str; 10] = [
    "output/closed_cassette_shipping_vibration_logger_dock_station_base_deck.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_cassette_dock.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_shock_vibration_data_logger_cradle.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_rail.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_alignment_stops.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_tamper_seal_inspection_pockets.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_foam_compression_gauges.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_csg_label_lands.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_service_keepout_and_cable_gauges.stl",
    "output/closed_cassette_shipping_vibration_logger_dock_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 7] = [
    "cassette dock",
    "shock/vibration data-logger cradle",
    "witness coupon rail",
    "alignment stops",
    "tamper/seal inspection pockets",
    "foam compression gauges",
    "CSG labels",
];

const DECK_X: f64 = 1340.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.8;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_PORT_D: f64 = 10.0;

const DOCK_POS: (f64, f64) = (-410.0, 140.0);
const DOCK_X: f64 = 430.0;
const DOCK_Y: f64 = 300.0;
const DOCK_Z: f64 = 58.0;
const CASSETTE_CLEARANCE_X: f64 = REVC_CHIP_LENGTH + 190.0;
const CASSETTE_CLEARANCE_Y: f64 = REVC_CHIP_WIDTH + 138.0;
const CASSETTE_CLEARANCE_Z: f64 = REVC_TOTAL_HEIGHT + 34.0;
const DOCK_CORNER_PAD_COUNT: usize = 4;
const SHIP_STRAP_BRIDGE_COUNT: usize = 3;

const LOGGER_POS: (f64, f64) = (70.0, 160.0);
const LOGGER_X: f64 = 380.0;
const LOGGER_Y: f64 = 260.0;
const LOGGER_Z: f64 = 48.0;
const LOGGER_POCKET_COUNT: usize = 3;
const LOGGER_PITCH_Y: f64 = 74.0;
const LOGGER_POCKET_X: f64 = 138.0;
const LOGGER_POCKET_Y: f64 = 52.0;
const LOGGER_POCKET_DEPTH: f64 = 18.0;
const AXIS_MARKER_COUNT: usize = 3;
const CABLE_EXIT_COUNT: usize = 4;

const COUPON_POS: (f64, f64) = (470.0, 160.0);
const COUPON_X: f64 = 260.0;
const COUPON_Y: f64 = 280.0;
const COUPON_Z: f64 = 42.0;
const COUPON_SLOT_COUNT: usize = 8;
const COUPON_SLOT_X: f64 = 36.0;
const COUPON_SLOT_Y: f64 = 96.0;
const COUPON_SLOT_PITCH_X: f64 = 28.0;
const WITNESS_STRIP_COUNT: usize = 6;

const TAMPER_POS: (f64, f64) = (-420.0, -220.0);
const TAMPER_X: f64 = 420.0;
const TAMPER_Y: f64 = 220.0;
const TAMPER_Z: f64 = 44.0;
const SEAL_POCKET_ROWS: usize = 2;
const SEAL_POCKET_COLS: usize = 5;
const SEAL_POCKET_COUNT: usize = SEAL_POCKET_ROWS * SEAL_POCKET_COLS;
const TAMPER_POCKET_D: f64 = 42.0;
const SEAL_POCKET_PITCH_X: f64 = 72.0;
const SEAL_POCKET_PITCH_Y: f64 = 68.0;

const FOAM_POS: (f64, f64) = (35.0, -235.0);
const FOAM_X: f64 = 400.0;
const FOAM_Y: f64 = 200.0;
const FOAM_Z: f64 = 42.0;
const FOAM_WINDOW_COUNT: usize = 6;
const FOAM_STEP_COUNT: usize = 7;
const FOAM_STEP_PITCH_X: f64 = 46.0;
const FOAM_STEP_BASE_Z: f64 = 5.0;

const LABEL_POS: (f64, f64) = (440.0, -235.0);
const LABEL_X: f64 = 280.0;
const LABEL_Y: f64 = 200.0;
const LABEL_Z: f64 = 14.0;
const LABEL_PLAQUE_COUNT: usize = 6;
const LABEL_BAR_COUNT: usize = 5;

const CABLE_POS: (f64, f64) = (0.0, 360.0);
const CABLE_X: f64 = 980.0;
const CABLE_Y: f64 = 70.0;
const CABLE_Z: f64 = 38.0;
const CABLE_CLIP_COUNT: usize = 10;
const SERVICE_KEEP_OUT_Z: f64 = 130.0;

const ALIGNMENT_STOP_Z: f64 = 34.0;
const DATUM_PIN_D: f64 = 9.5;
const HARD_STOP_W: f64 = 18.0;
const GO_NOGO_PIN_COUNT: usize = 6;

#[derive(Clone, Copy)]
struct ComponentSpec {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let dock = cassette_dock();
    export(OUTPUTS[1], &dock);

    let logger = shock_vibration_data_logger_cradle();
    export(OUTPUTS[2], &logger);

    let coupons = witness_coupon_rail();
    export(OUTPUTS[3], &coupons);

    let stops = alignment_stops();
    export(OUTPUTS[4], &stops);

    let tamper = tamper_seal_inspection_pockets();
    export(OUTPUTS[5], &tamper);

    let foam = foam_compression_gauges();
    export(OUTPUTS[6], &foam);

    let labels = csg_label_lands();
    export(OUTPUTS[7], &labels);

    let service = service_keepout_and_cable_gauges();
    export(OUTPUTS[8], &service);

    let assembly = base
        + dock.translate(DOCK_POS.0, DOCK_POS.1, deck_top_z())
        + logger.translate(LOGGER_POS.0, LOGGER_POS.1, deck_top_z())
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, deck_top_z())
        + stops.translate(DOCK_POS.0, DOCK_POS.1, deck_top_z() + DOCK_Z)
        + tamper.translate(TAMPER_POS.0, TAMPER_POS.1, deck_top_z())
        + foam.translate(FOAM_POS.0, FOAM_POS.1, deck_top_z())
        + labels.translate(LABEL_POS.0, LABEL_POS.1, deck_top_z())
        + service.translate(CABLE_POS.0, CABLE_POS.1, deck_top_z());
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed cassette shipping vibration logger dock station:");
    println!("  Footprint:                {DECK_X:.0}mm x {DECK_Y:.0}mm evidence deck");
    println!(
        "  Cassette dock:            {DOCK_X:.0}mm x {DOCK_Y:.0}mm sealed cassette receiver with {:.1}mm x {:.1}mm x {:.1}mm clearance envelope",
        CASSETTE_CLEARANCE_X, CASSETTE_CLEARANCE_Y, CASSETTE_CLEARANCE_Z
    );
    println!(
        "  Logger cradle:            {LOGGER_POCKET_COUNT} logger pockets, {AXIS_MARKER_COUNT} axis markers, {CABLE_EXIT_COUNT} cable exits"
    );
    println!(
        "  Witness handling:         {COUPON_SLOT_COUNT} witness coupon slots, {WITNESS_STRIP_COUNT} abrasion/color-change strip lands, {SEAL_POCKET_COUNT} tamper/seal pockets"
    );
    println!(
        "  Alignment/compression:    {GO_NOGO_PIN_COUNT} go/no-go datum pins, {FOAM_WINDOW_COUNT} foam windows, {FOAM_STEP_COUNT} compression step gauges"
    );
    println!(
        "  Labels/keepouts:          {LABEL_PLAQUE_COUNT} CSG label plaques with raised bar codes, {CABLE_CLIP_COUNT} cable clips, {SERVICE_KEEP_OUT_Z:.0}mm service clearance gauge"
    );
    println!(
        "  Scope:                    CAD geometry for sealed cassette shipping evidence only; no acceptance threshold is encoded."
    );
    println!("  Required feature groups:  {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn component_specs() -> [ComponentSpec; 7] {
    [
        ComponentSpec {
            name: "cassette_dock",
            center: DOCK_POS,
            width: DOCK_X,
            depth: DOCK_Y,
        },
        ComponentSpec {
            name: "shock_vibration_data_logger_cradle",
            center: LOGGER_POS,
            width: LOGGER_X,
            depth: LOGGER_Y,
        },
        ComponentSpec {
            name: "witness_coupon_rail",
            center: COUPON_POS,
            width: COUPON_X,
            depth: COUPON_Y,
        },
        ComponentSpec {
            name: "tamper_seal_inspection_pockets",
            center: TAMPER_POS,
            width: TAMPER_X,
            depth: TAMPER_Y,
        },
        ComponentSpec {
            name: "foam_compression_gauges",
            center: FOAM_POS,
            width: FOAM_X,
            depth: FOAM_Y,
        },
        ComponentSpec {
            name: "csg_label_lands",
            center: LABEL_POS,
            width: LABEL_X,
            depth: LABEL_Y,
        },
        ComponentSpec {
            name: "service_keepout_and_cable_gauges",
            center: CABLE_POS,
            width: CABLE_X,
            depth: CABLE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 10);
    assert_eq!(REQUIRED_FEATURES.len(), 7);
    assert_eq!(DOCK_CORNER_PAD_COUNT, 4);
    assert_eq!(LOGGER_POCKET_COUNT, 3);
    assert_eq!(AXIS_MARKER_COUNT, 3);
    assert_eq!(SEAL_POCKET_COUNT, 10);
    assert_eq!(FOAM_WINDOW_COUNT, 6);
    assert_eq!(LABEL_PLAQUE_COUNT, 6);
    assert!(CASSETTE_CLEARANCE_X > REVC_CHIP_LENGTH + 170.0);
    assert!(CASSETTE_CLEARANCE_Y > REVC_CHIP_WIDTH + 120.0);
    assert!(CASSETTE_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 30.0);
    assert!(DOCK_Z + ALIGNMENT_STOP_Z < SERVICE_KEEP_OUT_Z);
    assert!(LOGGER_POCKET_DEPTH < LOGGER_Z - 10.0);
    assert!(FOAM_STEP_BASE_Z + FOAM_STEP_COUNT as f64 * 2.0 < FOAM_Z);

    let specs = component_specs();
    for spec in specs {
        assert!(
            fits_on_deck(spec),
            "{} exceeds station deck footprint",
            spec.name
        );
    }
    for left in 0..specs.len() {
        for right in (left + 1)..specs.len() {
            assert!(
                !overlaps(specs[left], specs[right]),
                "{} overlaps {}",
                specs[left].name,
                specs[right].name
            );
        }
    }
}

fn fits_on_deck(spec: ComponentSpec) -> bool {
    spec.center.0.abs() + spec.width / 2.0 <= DECK_X / 2.0 - RIM_W - 8.0
        && spec.center.1.abs() + spec.depth / 2.0 <= DECK_Y / 2.0 - RIM_W - 8.0
}

fn overlaps(a: ComponentSpec, b: ComponentSpec) -> bool {
    let ax_min = a.center.0 - a.width / 2.0;
    let ax_max = a.center.0 + a.width / 2.0;
    let ay_min = a.center.1 - a.depth / 2.0;
    let ay_max = a.center.1 + a.depth / 2.0;
    let bx_min = b.center.0 - b.width / 2.0;
    let bx_max = b.center.0 + b.width / 2.0;
    let by_min = b.center.1 - b.depth / 2.0;
    let by_max = b.center.1 + b.depth / 2.0;

    ax_min < bx_max && ax_max > bx_min && ay_min < by_max && ay_max > by_min
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let evidence_basin = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_recessed_evidence_basin",
        DECK_X - 124.0,
        DECK_Y - 128.0,
        8.0,
    )
    .translate(0.0, -12.0, DECK_Z / 2.0 - 4.0);
    let front_witness_gutter = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_front_witness_gutter",
        DECK_X - 220.0,
        26.0,
        10.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 62.0, DECK_Z / 2.0 - 5.0);
    let drain = centered_cylinder(
        "closed_cassette_shipping_vibration_logger_dock_station_deck_drain_port",
        DRAIN_PORT_D / 2.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 34.0, 0.0);

    deck - evidence_basin - front_witness_gutter - drain - component_socket_cuts() - mount_holes()
        + perimeter_rims()
        + vibration_axis_route_lands()
        + workflow_port_tabs()
}

fn component_socket_cuts() -> Part {
    let mut sockets =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_socket_cuts");
    for spec in component_specs() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_{}_socket",
                    spec.name
                ),
                spec.width + 10.0,
                spec.depth + 10.0,
                SOCKET_DEPTH + 0.8,
            )
            .translate(
                spec.center.0,
                spec.center.1,
                DECK_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("closed_cassette_shipping_vibration_logger_dock_station_mounts");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_shipping_vibration_logger_dock_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 52.0, -DECK_Y / 2.0 + 52.0),
        (DECK_X / 2.0 - 52.0, -DECK_Y / 2.0 + 52.0),
        (-DECK_X / 2.0 + 52.0, DECK_Y / 2.0 - 52.0),
        (DECK_X / 2.0 - 52.0, DECK_Y / 2.0 - 52.0),
        (0.0, -DECK_Y / 2.0 + 52.0),
        (0.0, DECK_Y / 2.0 - 52.0),
        (-DECK_X / 2.0 + 52.0, 0.0),
        (DECK_X / 2.0 - 52.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    let front = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_front_evidence_curb",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_rear_logger_cable_curb",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_left_cassette_curb",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_right_coupon_curb",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn vibration_axis_route_lands() -> Part {
    let inbound = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_inbound_package_route_land",
        370.0,
        7.0,
        5.0,
    )
    .translate(-430.0, 318.0, DECK_Z / 2.0 + 2.5);
    let logger_to_coupon = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_to_coupon_route_land",
        560.0,
        7.0,
        5.0,
    )
    .translate(210.0, 24.0, DECK_Z / 2.0 + 2.5);
    let inspection_to_status = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_inspection_to_label_route_land",
        455.0,
        7.0,
        5.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(260.0, -122.0, DECK_Z / 2.0 + 2.5);
    let x_axis = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_deck_x_axis_witness_land",
        176.0,
        9.0,
        6.0,
    )
    .translate(70.0, 18.0, DECK_Z / 2.0 + 3.0);
    let y_axis = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_deck_y_axis_witness_land",
        9.0,
        136.0,
        6.0,
    )
    .translate(70.0, 18.0, DECK_Z / 2.0 + 3.0);
    inbound + logger_to_coupon + inspection_to_status + x_axis + y_axis
}

fn workflow_port_tabs() -> Part {
    let ports = ["receive", "log", "inspect", "retain", "release"];
    let mut tabs =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_workflow_tabs");
    for (i, port) in ports.iter().enumerate() {
        tabs = tabs
            + csg_label_plaque(
                format!("closed_cassette_shipping_vibration_logger_dock_station_{port}_port_tab"),
                78.0,
                18.0,
                6.0,
                i,
            )
            .translate(
                centered_index(i, ports.len(), 160.0),
                DECK_Y / 2.0 - 74.0,
                DECK_Z / 2.0 + 3.0,
            );
    }
    tabs
}

fn cassette_dock() -> Part {
    let body = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_cassette_dock_body",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0);
    let cassette_clearance = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_cassette_clearance_pocket",
        CASSETTE_CLEARANCE_X,
        CASSETTE_CLEARANCE_Y,
        CASSETTE_CLEARANCE_Z,
    )
    .translate(0.0, 4.0, DOCK_Z - CASSETTE_CLEARANCE_Z / 2.0 + 4.0);
    let front_loading_throat = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_front_loading_throat",
        CASSETTE_CLEARANCE_X - 58.0,
        68.0,
        DOCK_Z + 6.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 + 28.0, DOCK_Z / 2.0 + 1.0);
    let logger_window = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_visual_window_cut",
        126.0,
        44.0,
        DOCK_Z + 6.0,
    )
    .translate(DOCK_X / 2.0 - 76.0, -20.0, DOCK_Z / 2.0 + 1.0);

    body - cassette_clearance - front_loading_throat - logger_window - dock_fastener_holes()
        + cassette_corner_pads()
        + shipping_strap_bridges()
        + dock_floor_witness_grid()
}

fn dock_fastener_holes() -> Part {
    let mut holes =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_dock_fasteners");
    for (i, (x, y)) in [
        (-DOCK_X / 2.0 + 34.0, -DOCK_Y / 2.0 + 34.0),
        (DOCK_X / 2.0 - 34.0, -DOCK_Y / 2.0 + 34.0),
        (-DOCK_X / 2.0 + 34.0, DOCK_Y / 2.0 - 34.0),
        (DOCK_X / 2.0 - 34.0, DOCK_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_shipping_vibration_logger_dock_station_dock_fastener_{i}"),
                3.4,
                DOCK_Z + 6.0,
                28,
            )
            .translate(*x, *y, DOCK_Z / 2.0);
    }
    holes
}

fn cassette_corner_pads() -> Part {
    let mut pads =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_cassette_corner_pads");
    let x = CASSETTE_CLEARANCE_X / 2.0 - 28.0;
    let y = CASSETTE_CLEARANCE_Y / 2.0 - 26.0;
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        pads = pads
            + centered_cube(
                format!(
                "closed_cassette_shipping_vibration_logger_dock_station_corner_elastomer_pad_{i}"
            ),
                52.0,
                42.0,
                6.0,
            )
            .translate(sx * x, sy * y + 4.0, DOCK_Z + 3.0);
    }
    pads
}

fn shipping_strap_bridges() -> Part {
    let mut bridges =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_ship_strap_bridges");
    for i in 0..SHIP_STRAP_BRIDGE_COUNT {
        let y = centered_index(i, SHIP_STRAP_BRIDGE_COUNT, 84.0);
        let bridge = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_strap_bridge_{i}"),
            24.0,
            56.0,
            24.0,
        )
        .translate(DOCK_X / 2.0 - 34.0, y, DOCK_Z + 12.0);
        let tunnel = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_strap_tunnel_{i}"),
            26.0,
            32.0,
            13.0,
        )
        .translate(DOCK_X / 2.0 - 34.0, y, DOCK_Z + 10.0);
        bridges = bridges + (bridge - tunnel);
    }
    bridges
}

fn dock_floor_witness_grid() -> Part {
    let mut grid =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_dock_witness_grid");
    for i in 0..5 {
        grid = grid
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_dock_x_witness_tick_{i}"
                ),
                4.0,
                CASSETTE_CLEARANCE_Y - 46.0,
                2.8,
            )
            .translate(centered_index(i, 5, 56.0), 4.0, DOCK_Z + 1.4);
    }
    for i in 0..4 {
        grid = grid
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_dock_y_witness_tick_{i}"
                ),
                CASSETTE_CLEARANCE_X - 56.0,
                4.0,
                2.8,
            )
            .translate(0.0, centered_index(i, 4, 50.0) + 4.0, DOCK_Z + 1.4);
    }
    grid
}

fn alignment_stops() -> Part {
    let rear_stop = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_rear_alignment_hard_stop",
        CASSETTE_CLEARANCE_X + 52.0,
        HARD_STOP_W,
        ALIGNMENT_STOP_Z,
    )
    .translate(
        0.0,
        CASSETTE_CLEARANCE_Y / 2.0 + 20.0,
        ALIGNMENT_STOP_Z / 2.0,
    );
    let left_stop = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_left_alignment_hard_stop",
        HARD_STOP_W,
        CASSETTE_CLEARANCE_Y + 44.0,
        ALIGNMENT_STOP_Z,
    )
    .translate(
        -CASSETTE_CLEARANCE_X / 2.0 - 20.0,
        4.0,
        ALIGNMENT_STOP_Z / 2.0,
    );
    let right_soft_stop = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_right_soft_capture_stop",
        HARD_STOP_W,
        CASSETTE_CLEARANCE_Y * 0.58,
        ALIGNMENT_STOP_Z * 0.62,
    )
    .translate(
        CASSETTE_CLEARANCE_X / 2.0 + 20.0,
        -26.0,
        ALIGNMENT_STOP_Z * 0.31,
    );
    let front_low_stop = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_front_low_loading_stop",
        CASSETTE_CLEARANCE_X * 0.56,
        HARD_STOP_W,
        ALIGNMENT_STOP_Z * 0.48,
    )
    .translate(
        -28.0,
        -CASSETTE_CLEARANCE_Y / 2.0 - 20.0,
        ALIGNMENT_STOP_Z * 0.24,
    );

    rear_stop + left_stop + right_soft_stop + front_low_stop + datum_pins() + go_nogo_pin_rack()
}

fn datum_pins() -> Part {
    let mut pins = Part::empty("closed_cassette_shipping_vibration_logger_dock_station_datum_pins");
    for (i, (x, y)) in [
        (
            -CASSETTE_CLEARANCE_X / 2.0 - 20.0,
            CASSETTE_CLEARANCE_Y / 2.0 + 20.0,
        ),
        (
            CASSETTE_CLEARANCE_X / 2.0 + 20.0,
            CASSETTE_CLEARANCE_Y / 2.0 + 20.0,
        ),
        (
            -CASSETTE_CLEARANCE_X / 2.0 - 20.0,
            -CASSETTE_CLEARANCE_Y / 2.0 - 20.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_cassette_shipping_vibration_logger_dock_station_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0 + 6.0,
            6.0,
            36,
        )
        .translate(*x, *y, 3.0);
        let pin = centered_cylinder(
            format!("closed_cassette_shipping_vibration_logger_dock_station_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            22.0,
            36,
        )
        .translate(*x, *y, 11.0);
        pins = pins + boss + pin;
    }
    pins
}

fn go_nogo_pin_rack() -> Part {
    let mut rack =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_go_nogo_pin_rack");
    for i in 0..GO_NOGO_PIN_COUNT {
        let x = centered_index(i, GO_NOGO_PIN_COUNT, 32.0);
        let height = 12.0 + i as f64 * 2.5;
        rack = rack
            + centered_cylinder(
                format!("closed_cassette_shipping_vibration_logger_dock_station_go_nogo_pin_{i}"),
                4.0 + i as f64 * 0.25,
                height,
                28,
            )
            .translate(x, -CASSETTE_CLEARANCE_Y / 2.0 - 52.0, height / 2.0);
    }
    rack
}

fn shock_vibration_data_logger_cradle() -> Part {
    let block = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_cradle_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);
    let top_basin = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_top_basin",
        LOGGER_X - 38.0,
        LOGGER_Y - 36.0,
        8.0,
    )
    .translate(0.0, 0.0, LOGGER_Z - 4.0);

    block - top_basin - logger_pocket_cuts() - logger_cable_exit_cuts()
        + logger_retaining_lips()
        + sensor_axis_markers()
        + shock_event_token_lands()
        + logger_serial_label_lands()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_logger_pocket_cuts");
    for i in 0..LOGGER_POCKET_COUNT {
        let y = centered_index(i, LOGGER_POCKET_COUNT, LOGGER_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_logger_pocket_cut_{i}"
                ),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH + 1.0,
            )
            .translate(-54.0, y, LOGGER_Z - LOGGER_POCKET_DEPTH / 2.0 + 0.5);
    }
    cuts
}

fn logger_cable_exit_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_logger_cable_exits");
    for i in 0..CABLE_EXIT_COUNT {
        let y = centered_index(i, CABLE_EXIT_COUNT, 50.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_logger_cable_exit_{i}"
                ),
                5.0,
                LOGGER_X + 20.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, LOGGER_Z - 13.0);
    }
    cuts
}

fn logger_retaining_lips() -> Part {
    let mut lips =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_logger_retaining_lips");
    for i in 0..LOGGER_POCKET_COUNT {
        let y = centered_index(i, LOGGER_POCKET_COUNT, LOGGER_PITCH_Y);
        let top_lip = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_logger_top_lip_{i}"),
            LOGGER_POCKET_X + 18.0,
            8.0,
            10.0,
        )
        .translate(-54.0, y + LOGGER_POCKET_Y / 2.0 + 8.0, LOGGER_Z + 5.0);
        let bottom_lip = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_logger_bottom_lip_{i}"),
            LOGGER_POCKET_X + 18.0,
            8.0,
            10.0,
        )
        .translate(-54.0, y - LOGGER_POCKET_Y / 2.0 - 8.0, LOGGER_Z + 5.0);
        lips = lips + top_lip + bottom_lip;
    }
    lips
}

fn sensor_axis_markers() -> Part {
    let x_arrow = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_x_axis_bar",
        112.0,
        8.0,
        6.0,
    )
    .translate(108.0, 58.0, LOGGER_Z + 3.0);
    let x_head = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_x_axis_head",
        18.0,
        18.0,
        6.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(172.0, 58.0, LOGGER_Z + 3.0);
    let y_arrow = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_y_axis_bar",
        8.0,
        112.0,
        6.0,
    )
    .translate(68.0, 16.0, LOGGER_Z + 3.0);
    let y_head = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_y_axis_head",
        18.0,
        18.0,
        6.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(68.0, 80.0, LOGGER_Z + 3.0);
    let z_post = centered_cylinder(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_z_axis_post",
        8.0,
        52.0,
        32,
    )
    .translate(128.0, -60.0, LOGGER_Z + 26.0);
    let z_cap = centered_cylinder(
        "closed_cassette_shipping_vibration_logger_dock_station_logger_z_axis_cap",
        15.0,
        6.0,
        32,
    )
    .translate(128.0, -60.0, LOGGER_Z + 55.0);
    x_arrow + x_head + y_arrow + y_head + z_post + z_cap
}

fn shock_event_token_lands() -> Part {
    let mut lands =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_shock_token_lands");
    for i in 0..6 {
        lands = lands
            + centered_cylinder(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_shock_token_land_{i}"
                ),
                13.0,
                4.0,
                36,
            )
            .translate(
                106.0 + centered_index(i % 3, 3, 44.0),
                -88.0 + (i / 3) as f64 * 42.0,
                LOGGER_Z + 2.0,
            );
    }
    lands
}

fn logger_serial_label_lands() -> Part {
    let mut labels =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_logger_serial_labels");
    for i in 0..LOGGER_POCKET_COUNT {
        labels = labels
            + csg_label_plaque(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_logger_serial_plaque_{i}"
                ),
                58.0,
                18.0,
                4.0,
                i,
            )
            .translate(-150.0, centered_index(i, LOGGER_POCKET_COUNT, LOGGER_PITCH_Y), LOGGER_Z + 2.0);
    }
    labels
}

fn witness_coupon_rail() -> Part {
    let body = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_rail_body",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0);
    let rail_channel = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_rail_channel",
        COUPON_X - 46.0,
        118.0,
        10.0,
    )
    .translate(0.0, 26.0, COUPON_Z - 5.0);

    body - rail_channel - coupon_slot_cuts()
        + coupon_retainer_lips()
        + witness_coupon_tabs()
        + abrasion_strip_lands()
        + coupon_index_ticks()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_coupon_slot_cuts");
    for i in 0..COUPON_SLOT_COUNT {
        let x = centered_index(i, COUPON_SLOT_COUNT, COUPON_SLOT_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_coupon_slot_cut_{i}"
                ),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_Z + 4.0,
            )
            .translate(x, 22.0, COUPON_Z / 2.0 + 1.0);
    }
    cuts
}

fn coupon_retainer_lips() -> Part {
    let rear = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_coupon_rear_retainer_lip",
        COUPON_X - 36.0,
        10.0,
        18.0,
    )
    .translate(0.0, 86.0, COUPON_Z + 9.0);
    let front = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_coupon_front_retainer_lip",
        COUPON_X - 80.0,
        10.0,
        12.0,
    )
    .translate(0.0, -36.0, COUPON_Z + 6.0);
    let left = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_coupon_left_end_stop",
        10.0,
        122.0,
        18.0,
    )
    .translate(-COUPON_X / 2.0 + 22.0, 24.0, COUPON_Z + 9.0);
    let right = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_coupon_right_end_stop",
        10.0,
        122.0,
        18.0,
    )
    .translate(COUPON_X / 2.0 - 22.0, 24.0, COUPON_Z + 9.0);
    rear + front + left + right
}

fn witness_coupon_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_tabs");
    for i in 0..COUPON_SLOT_COUNT {
        let x = centered_index(i, COUPON_SLOT_COUNT, COUPON_SLOT_PITCH_X);
        let coupon = centered_cube(
            format!(
                "closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_card_{i}"
            ),
            COUPON_SLOT_X - 10.0,
            COUPON_SLOT_Y - 14.0,
            3.0,
        )
        .translate(x, 22.0, COUPON_Z + 1.5);
        let pull_tab = centered_cube(
            format!(
                "closed_cassette_shipping_vibration_logger_dock_station_witness_coupon_pull_tab_{i}"
            ),
            COUPON_SLOT_X - 14.0,
            16.0,
            8.0,
        )
        .translate(x, -48.0, COUPON_Z + 4.0);
        tabs = tabs + coupon + pull_tab;
    }
    tabs
}

fn abrasion_strip_lands() -> Part {
    let mut lands =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_abrasion_strip_lands");
    for i in 0..WITNESS_STRIP_COUNT {
        lands = lands
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_abrasion_color_strip_land_{i}"
                ),
                78.0,
                16.0,
                4.0,
            )
            .translate(centered_index(i, WITNESS_STRIP_COUNT, 38.0), -100.0, COUPON_Z + 2.0);
    }
    lands
}

fn coupon_index_ticks() -> Part {
    let mut ticks =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_coupon_index_ticks");
    for i in 0..(COUPON_SLOT_COUNT + 1) {
        ticks = ticks
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_coupon_index_tick_{i}"
                ),
                3.0,
                26.0,
                5.0,
            )
            .translate(
                centered_index(i, COUPON_SLOT_COUNT + 1, COUPON_SLOT_PITCH_X),
                104.0,
                COUPON_Z + 2.5,
            );
    }
    ticks
}

fn tamper_seal_inspection_pockets() -> Part {
    let block = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_tamper_seal_pocket_block",
        TAMPER_X,
        TAMPER_Y,
        TAMPER_Z,
    )
    .translate(0.0, 0.0, TAMPER_Z / 2.0);
    let top_basin = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_tamper_seal_top_basin",
        TAMPER_X - 32.0,
        TAMPER_Y - 32.0,
        8.0,
    )
    .translate(0.0, 0.0, TAMPER_Z - 4.0);

    block - top_basin - seal_pocket_cuts()
        + seal_witness_rings()
        + magnifier_frame()
        + adhesive_lift_tabs()
        + tamper_label_keys()
}

fn seal_pocket_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_seal_pocket_cuts");
    for row in 0..SEAL_POCKET_ROWS {
        for col in 0..SEAL_POCKET_COLS {
            let index = row * SEAL_POCKET_COLS + col;
            let (x, y) = seal_pocket_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_cassette_shipping_vibration_logger_dock_station_seal_pocket_cut_{index}"
                    ),
                    TAMPER_POCKET_D / 2.0,
                    18.0,
                    44,
                )
                .translate(x, y, TAMPER_Z - 8.0);
        }
    }
    cuts
}

fn seal_witness_rings() -> Part {
    let mut rings =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_seal_witness_rings");
    for row in 0..SEAL_POCKET_ROWS {
        for col in 0..SEAL_POCKET_COLS {
            let index = row * SEAL_POCKET_COLS + col;
            let (x, y) = seal_pocket_xy(row, col);
            let outer = centered_cylinder(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_seal_ring_outer_{index}"
                ),
                TAMPER_POCKET_D / 2.0 + 3.0,
                4.0,
                44,
            );
            let inner = centered_cylinder(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_seal_ring_inner_{index}"
                ),
                TAMPER_POCKET_D / 2.0 - 4.0,
                5.0,
                44,
            );
            rings = rings + (outer - inner).translate(x, y, TAMPER_Z + 2.0);
        }
    }
    rings
}

fn seal_pocket_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, SEAL_POCKET_COLS, SEAL_POCKET_PITCH_X),
        centered_index(row, SEAL_POCKET_ROWS, SEAL_POCKET_PITCH_Y) + 16.0,
    )
}

fn magnifier_frame() -> Part {
    let frame = rectangular_frame_xy(
        "closed_cassette_shipping_vibration_logger_dock_station_magnifier_window_frame",
        136.0,
        58.0,
        8.0,
        8.0,
    )
    .translate(
        -TAMPER_X / 2.0 + 94.0,
        -TAMPER_Y / 2.0 + 48.0,
        TAMPER_Z + 4.0,
    );
    let fiducial_left = centered_cylinder(
        "closed_cassette_shipping_vibration_logger_dock_station_magnifier_left_fiducial",
        8.0,
        4.0,
        32,
    )
    .translate(
        -TAMPER_X / 2.0 + 44.0,
        -TAMPER_Y / 2.0 + 48.0,
        TAMPER_Z + 2.0,
    );
    let fiducial_right = centered_cylinder(
        "closed_cassette_shipping_vibration_logger_dock_station_magnifier_right_fiducial",
        8.0,
        4.0,
        32,
    )
    .translate(
        -TAMPER_X / 2.0 + 144.0,
        -TAMPER_Y / 2.0 + 48.0,
        TAMPER_Z + 2.0,
    );
    frame + fiducial_left + fiducial_right
}

fn adhesive_lift_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_adhesive_lift_tabs");
    for i in 0..4 {
        tabs = tabs
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_adhesive_lift_tab_{i}"
                ),
                58.0,
                14.0,
                6.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 8.0 } else { -8.0 })
            .translate(
                TAMPER_X / 2.0 - 84.0,
                centered_index(i, 4, 38.0),
                TAMPER_Z + 3.0,
            );
    }
    tabs
}

fn tamper_label_keys() -> Part {
    let mut keys =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_tamper_label_keys");
    for i in 0..3 {
        keys = keys
            + csg_label_plaque(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_tamper_label_key_{i}"
                ),
                70.0,
                18.0,
                4.0,
                i + 1,
            )
            .translate(
                -40.0 + i as f64 * 82.0,
                -TAMPER_Y / 2.0 + 32.0,
                TAMPER_Z + 2.0,
            );
    }
    keys
}

fn foam_compression_gauges() -> Part {
    let block = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_compression_gauge_block",
        FOAM_X,
        FOAM_Y,
        FOAM_Z,
    )
    .translate(0.0, 0.0, FOAM_Z / 2.0);
    let gauge_tray = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_compression_gauge_tray",
        FOAM_X - 34.0,
        FOAM_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, FOAM_Z - 4.0);

    block - gauge_tray - foam_window_cuts()
        + compression_step_blocks()
        + min_max_flags()
        + foam_sample_retention_rails()
}

fn foam_window_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_foam_window_cuts");
    for i in 0..FOAM_WINDOW_COUNT {
        let x = centered_index(i % 3, 3, 104.0);
        let y = -38.0 + (i / 3) as f64 * 76.0;
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_foam_compression_window_{i}"
                ),
                74.0,
                46.0,
                FOAM_Z + 4.0,
            )
            .translate(x, y, FOAM_Z / 2.0 + 1.0);
    }
    cuts
}

fn compression_step_blocks() -> Part {
    let mut steps =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_compression_steps");
    for i in 0..FOAM_STEP_COUNT {
        let height = FOAM_STEP_BASE_Z + i as f64 * 2.0;
        steps = steps
            + centered_cube(
                format!("closed_cassette_shipping_vibration_logger_dock_station_foam_step_{i}"),
                34.0,
                88.0,
                height,
            )
            .translate(
                centered_index(i, FOAM_STEP_COUNT, FOAM_STEP_PITCH_X),
                FOAM_Y / 2.0 - 62.0,
                FOAM_Z + height / 2.0,
            );
    }
    steps
}

fn min_max_flags() -> Part {
    let min_flag = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_min_flag_land",
        48.0,
        16.0,
        7.0,
    )
    .translate(-FOAM_X / 2.0 + 58.0, FOAM_Y / 2.0 - 32.0, FOAM_Z + 3.5);
    let max_flag = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_max_flag_land",
        48.0,
        16.0,
        7.0,
    )
    .translate(FOAM_X / 2.0 - 58.0, FOAM_Y / 2.0 - 32.0, FOAM_Z + 3.5);
    let center_flag = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_nominal_flag_land",
        58.0,
        16.0,
        7.0,
    )
    .translate(0.0, FOAM_Y / 2.0 - 32.0, FOAM_Z + 3.5);
    min_flag + max_flag + center_flag
}

fn foam_sample_retention_rails() -> Part {
    let front = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_front_sample_rail",
        FOAM_X - 58.0,
        8.0,
        18.0,
    )
    .translate(0.0, -FOAM_Y / 2.0 + 26.0, FOAM_Z + 9.0);
    let rear = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_foam_rear_sample_rail",
        FOAM_X - 58.0,
        8.0,
        18.0,
    )
    .translate(0.0, 12.0, FOAM_Z + 9.0);
    front + rear
}

fn csg_label_lands() -> Part {
    let base = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_csg_label_panel",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    )
    .translate(0.0, 0.0, LABEL_Z / 2.0);
    let bevel_relief = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_csg_label_panel_relief",
        LABEL_X - 28.0,
        LABEL_Y - 26.0,
        5.0,
    )
    .translate(0.0, 0.0, LABEL_Z - 2.0);

    base - bevel_relief + label_plaques() + label_arrow_icons() + label_status_tokens()
}

fn label_plaques() -> Part {
    let mut plaques =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_label_plaques");
    for i in 0..LABEL_PLAQUE_COUNT {
        let x = centered_index(i % 2, 2, 132.0);
        let y = centered_index(i / 2, 3, 56.0);
        plaques = plaques
            + csg_label_plaque(
                format!(
                    "closed_cassette_shipping_vibration_logger_dock_station_csg_label_plaque_{i}"
                ),
                106.0,
                34.0,
                5.0,
                i,
            )
            .translate(x, y + 18.0, LABEL_Z + 2.5);
    }
    plaques
}

fn label_arrow_icons() -> Part {
    let mut icons =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_label_arrow_icons");
    for i in 0..3 {
        let y = -LABEL_Y / 2.0 + 26.0 + i as f64 * 18.0;
        let shaft = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_label_arrow_shaft_{i}"),
            82.0,
            5.0,
            5.0,
        )
        .translate(-40.0, y, LABEL_Z + 2.5);
        let head = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_label_arrow_head_{i}"),
            15.0,
            15.0,
            5.0,
        )
        .rotate(0.0, 0.0, 45.0)
        .translate(10.0, y, LABEL_Z + 2.5);
        icons = icons + shaft + head;
    }
    icons
}

fn label_status_tokens() -> Part {
    let mut tokens =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_label_status_tokens");
    for i in 0..3 {
        let token = centered_cylinder(
            format!("closed_cassette_shipping_vibration_logger_dock_station_status_token_{i}"),
            12.0,
            5.0,
            36,
        )
        .translate(76.0 + i as f64 * 30.0, -LABEL_Y / 2.0 + 36.0, LABEL_Z + 2.5);
        let notch = centered_cube(
            format!(
                "closed_cassette_shipping_vibration_logger_dock_station_status_token_notch_{i}"
            ),
            5.0,
            16.0,
            6.0,
        )
        .rotate(0.0, 0.0, i as f64 * 45.0)
        .translate(76.0 + i as f64 * 30.0, -LABEL_Y / 2.0 + 36.0, LABEL_Z + 2.5);
        tokens = tokens + (token - notch);
    }
    tokens
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 3.0 + ((seed + i) % 3) as f64 * 2.0;
        let height = y - 10.0 - (i % 2) as f64 * 4.0;
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 2.0).translate(
                -x / 2.0 + 14.0 + i as f64 * 12.0,
                0.0,
                z / 2.0 + 1.0,
            );
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 18.0, 6.0, z + 2.0)
        .translate(x / 2.0 - 18.0, y / 2.0 - 7.0, z / 2.0 + 1.0);
    base + bars + orientation_tab
}

fn service_keepout_and_cable_gauges() -> Part {
    let trough = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_rear_cable_trough",
        CABLE_X,
        CABLE_Y,
        CABLE_Z,
    )
    .translate(0.0, 0.0, CABLE_Z / 2.0);
    let trough_channel = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_rear_cable_trough_channel",
        CABLE_X - 48.0,
        CABLE_Y - 26.0,
        14.0,
    )
    .translate(0.0, 0.0, CABLE_Z - 7.0);

    trough - trough_channel - cable_passage_cuts()
        + cable_comb_clips()
        + service_clearance_goalposts()
        + connector_bulkhead_blocks()
}

fn cable_passage_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_cable_passage_cuts");
    for i in 0..CABLE_CLIP_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("closed_cassette_shipping_vibration_logger_dock_station_cable_passage_{i}"),
                5.5,
                CABLE_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CABLE_CLIP_COUNT, 78.0),
                0.0,
                CABLE_Z - 13.0,
            );
    }
    cuts
}

fn cable_comb_clips() -> Part {
    let mut clips =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_cable_comb_clips");
    for i in 0..CABLE_CLIP_COUNT {
        let clip = rectangular_frame_xy(
            format!("closed_cassette_shipping_vibration_logger_dock_station_cable_clip_{i}"),
            28.0,
            30.0,
            5.0,
            15.0,
        )
        .translate(
            centered_index(i, CABLE_CLIP_COUNT, 78.0),
            0.0,
            CABLE_Z + 7.5,
        );
        clips = clips + clip;
    }
    clips
}

fn service_clearance_goalposts() -> Part {
    let left = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_left_service_clearance_goalpost",
        18.0,
        20.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        -CABLE_X / 2.0 + 36.0,
        0.0,
        CABLE_Z + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_right_service_clearance_goalpost",
        18.0,
        20.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        CABLE_X / 2.0 - 36.0,
        0.0,
        CABLE_Z + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let top_bar = centered_cube(
        "closed_cassette_shipping_vibration_logger_dock_station_service_clearance_top_bar",
        CABLE_X - 72.0,
        14.0,
        12.0,
    )
    .translate(0.0, 0.0, CABLE_Z + SERVICE_KEEP_OUT_Z + 6.0);
    left + right + top_bar
}

fn connector_bulkhead_blocks() -> Part {
    let mut blocks =
        Part::empty("closed_cassette_shipping_vibration_logger_dock_station_bulkhead_blocks");
    for i in 0..4 {
        let x = centered_index(i, 4, 94.0);
        let block = centered_cube(
            format!("closed_cassette_shipping_vibration_logger_dock_station_bulkhead_block_{i}"),
            56.0,
            18.0,
            34.0,
        )
        .translate(x, CABLE_Y / 2.0 + 11.0, CABLE_Z + 17.0);
        let port = centered_cylinder(
            format!("closed_cassette_shipping_vibration_logger_dock_station_bulkhead_port_{i}"),
            8.0,
            24.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CABLE_Y / 2.0 + 11.0, CABLE_Z + 17.0);
        blocks = blocks + (block - port);
    }
    blocks
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    wall: f64,
    z: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}
