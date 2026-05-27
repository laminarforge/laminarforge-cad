use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste-line backpressure / contamination sentinel station.
//
// This standalone generator packages a closed waste-line validation module for
// multi-chip perfusion cassette waste paths. The geometry makes pressure tap
// ladders, sterile vent barrier witness pockets, overflow trays, check-valve
// surrogate nests, split sample ports, leak/condensate witness coupons,
// quarantine/release gates, robotic service datums, barcode custody lands, and
// route-stability witness features visible for design review.
//
// This is mechanical CAD for validation-fixture packaging only. It is not a
// pressure-rated manifold, sterility claim, biohazard-disposal workflow, or
// acceptance procedure.

const PREFIX: &str = "closed_waste_line_backpressure_contamination_sentinel_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_waste_line_backpressure_contamination_sentinel_station_secondary_containment_deck.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_multi_chip_waste_inlet_bulkhead.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_pressure_tap_ladder_manifold.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_hydrophobic_vent_sterile_barrier_witness_pockets.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_overflow_catch_trays.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_check_valve_surrogate_nests.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_split_sample_ports.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_leak_condensate_witness_coupons.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_quarantine_release_gate_panel.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_robotic_service_datums.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_barcode_custody_features.stl",
    "output/closed_waste_line_backpressure_contamination_sentinel_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "multi_chip_waste_inlet_bulkhead",
    "pressure_tap_ladder_manifold",
    "hydrophobic_vent_sterile_barrier_witness_pockets",
    "overflow_catch_trays",
    "check_valve_surrogate_nests",
    "split_sample_ports",
    "leak_condensate_witness_coupons",
    "quarantine_release_gate_panel",
    "robotic_service_datums",
    "barcode_custody_features",
    "waste_routing_stability_spine",
];

const STATION_X: f64 = 1380.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 52.0;
const BASIN_X: f64 = 1220.0;
const BASIN_Y: f64 = 690.0;
const BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 16.0;

const INLET_POS: (f64, f64) = (-510.0, 248.0);
const INLET_X: f64 = 280.0;
const INLET_Y: f64 = 128.0;
const INLET_Z: f64 = 82.0;
const MULTI_CHIP_CASSETTES: usize = 6;
const INLET_PORT_D: f64 = 11.0;
const INLET_PITCH: f64 = 38.0;
const CAP_PARKS: usize = MULTI_CHIP_CASSETTES;

const PRESSURE_POS: (f64, f64) = (-130.0, 230.0);
const PRESSURE_X: f64 = 430.0;
const PRESSURE_Y: f64 = 172.0;
const PRESSURE_Z: f64 = 66.0;
const PRESSURE_TAPS_PER_CASSETTE: usize = 4;
const PRESSURE_TAP_COUNT: usize = MULTI_CHIP_CASSETTES * PRESSURE_TAPS_PER_CASSETTE;
const PRESSURE_TAP_D: f64 = 5.2;
const PRESSURE_LANE_PITCH: f64 = 58.0;
const PRESSURE_LEVEL_PITCH: f64 = 18.0;

const VENT_POS: (f64, f64) = (365.0, 235.0);
const VENT_X: f64 = 350.0;
const VENT_Y: f64 = 170.0;
const VENT_Z: f64 = 94.0;
const HYDROPHOBIC_VENTS: usize = 4;
const STERILE_BARRIER_WITNESS_POCKETS: usize = 4;
const VENT_FILTER_D: f64 = 30.0;
const VENT_FILTER_LENGTH: f64 = 138.0;

const OVERFLOW_POS: (f64, f64) = (-485.0, -20.0);
const OVERFLOW_X: f64 = 340.0;
const OVERFLOW_Y: f64 = 210.0;
const OVERFLOW_Z: f64 = 54.0;
const OVERFLOW_CATCH_TRAYS: usize = 3;
const OVERFLOW_TRAY_X: f64 = 255.0;
const OVERFLOW_TRAY_Y: f64 = 48.0;
const OVERFLOW_TRAY_DEPTH: f64 = 10.0;
const FLOAT_LEVEL_TICKS: usize = 5;

const CHECK_POS: (f64, f64) = (-110.0, -20.0);
const CHECK_X: f64 = 330.0;
const CHECK_Y: f64 = 190.0;
const CHECK_Z: f64 = 60.0;
const CHECK_VALVE_NESTS: usize = MULTI_CHIP_CASSETTES;
const CHECK_VALVE_D: f64 = 23.0;
const CHECK_VALVE_PITCH: f64 = 48.0;

const SAMPLE_POS: (f64, f64) = (325.0, -25.0);
const SAMPLE_X: f64 = 360.0;
const SAMPLE_Y: f64 = 190.0;
const SAMPLE_Z: f64 = 58.0;
const SAMPLE_BRANCHES_PER_CASSETTE: usize = 2;
const SPLIT_SAMPLE_PORTS: usize = MULTI_CHIP_CASSETTES * SAMPLE_BRANCHES_PER_CASSETTE;
const SAMPLE_PORT_D: f64 = 7.0;
const SAMPLE_LANE_PITCH: f64 = 48.0;

const COUPON_POS: (f64, f64) = (-475.0, -275.0);
const COUPON_X: f64 = 340.0;
const COUPON_Y: f64 = 130.0;
const COUPON_Z: f64 = 24.0;
const LEAK_WITNESS_COUPONS: usize = 8;
const CONDENSATE_WITNESS_COUPONS: usize = 4;
const WITNESS_COUPON_TOTAL: usize = LEAK_WITNESS_COUPONS + CONDENSATE_WITNESS_COUPONS;

const GATE_POS: (f64, f64) = (-90.0, -280.0);
const GATE_X: f64 = 340.0;
const GATE_Y: f64 = 128.0;
const GATE_Z: f64 = 44.0;
const DISPOSITION_GATE_LANES: usize = 3;
const GATE_TOKEN_POCKETS_PER_LANE: usize = 4;

const ROBOT_POS: (f64, f64) = (350.0, -280.0);
const ROBOT_X: f64 = 300.0;
const ROBOT_Y: f64 = 128.0;
const ROBOT_Z: f64 = 28.0;
const ROBOT_SERVICE_DATUMS: usize = 4;
const SERVICE_KEEP_OUT_Z: f64 = 116.0;

const BARCODE_POS: (f64, f64) = (0.0, 362.0);
const BARCODE_X: f64 = 900.0;
const BARCODE_Y: f64 = 54.0;
const BARCODE_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 10;
const CUSTODY_SEAL_POINTS: usize = 6;
const RFID_LANDS: usize = 4;

const ROUTE_STABILITY_CLAMPS: usize = 8;
const ROUTE_CHANNELS: usize = MULTI_CHIP_CASSETTES + 2;
const ROUTE_BORE_D: f64 = 6.4;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_VENT_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_WASTE_CART_CLEARANCE: f64 = 230.0;
const RIGHT_SAMPLE_SERVICE_CLEARANCE: f64 = 220.0;
const TOP_FILTER_SWAP_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let inlet = multi_chip_waste_inlet_bulkhead();
    export(OUTPUTS[1], &inlet);

    let pressure = pressure_tap_ladder_manifold();
    export(OUTPUTS[2], &pressure);

    let vent = hydrophobic_vent_sterile_barrier_witness_pockets();
    export(OUTPUTS[3], &vent);

    let overflow = overflow_catch_trays();
    export(OUTPUTS[4], &overflow);

    let checks = check_valve_surrogate_nests();
    export(OUTPUTS[5], &checks);

    let samples = split_sample_ports();
    export(OUTPUTS[6], &samples);

    let coupons = leak_condensate_witness_coupons();
    export(OUTPUTS[7], &coupons);

    let gates = quarantine_release_gate_panel();
    export(OUTPUTS[8], &gates);

    let datums = robotic_service_datums();
    export(OUTPUTS[9], &datums);

    let custody = barcode_custody_features();
    export(OUTPUTS[10], &custody);

    let assembly = deck
        + inlet.translate(INLET_POS.0, INLET_POS.1, deck_z(INLET_Z))
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, deck_z(PRESSURE_Z))
        + vent.translate(VENT_POS.0, VENT_POS.1, deck_z(VENT_Z))
        + overflow.translate(OVERFLOW_POS.0, OVERFLOW_POS.1, deck_z(OVERFLOW_Z))
        + checks.translate(CHECK_POS.0, CHECK_POS.1, deck_z(CHECK_Z))
        + samples.translate(SAMPLE_POS.0, SAMPLE_POS.1, deck_z(SAMPLE_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, deck_z(COUPON_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, deck_z(GATE_Z))
        + datums.translate(ROBOT_POS.0, ROBOT_POS.1, deck_z(ROBOT_Z))
        + custody.translate(BARCODE_POS.0, BARCODE_POS.1, deck_z(BARCODE_Z))
        + waste_routing_stability_spine();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed waste-line backpressure contamination sentinel station:");
    println!(
        "  Footprint:            {STATION_X:.0}mm x {STATION_Y:.0}mm secondary containment deck, {BASIN_X:.0}mm x {BASIN_Y:.0}mm recessed basin"
    );
    println!(
        "  Multi-chip inlet:     {MULTI_CHIP_CASSETTES} closed waste inlets, {CAP_PARKS} cap parks, {ROUTE_STABILITY_CLAMPS} route-stability clamps"
    );
    println!(
        "  Backpressure ladder:  {PRESSURE_TAP_COUNT} pressure taps across {MULTI_CHIP_CASSETTES} lanes, {PRESSURE_TAPS_PER_CASSETTE} levels per lane"
    );
    println!(
        "  Barrier witnesses:    {HYDROPHOBIC_VENTS} hydrophobic vent filters, {STERILE_BARRIER_WITNESS_POCKETS} sterile-barrier witness pockets"
    );
    println!(
        "  Waste evidence:       {OVERFLOW_CATCH_TRAYS} overflow trays, {CHECK_VALVE_NESTS} check-valve surrogate nests, {SPLIT_SAMPLE_PORTS} split sample ports"
    );
    println!(
        "  Custody/release:      {WITNESS_COUPON_TOTAL} leak/condensate coupons, {DISPOSITION_GATE_LANES} quarantine/release lanes, {BARCODE_LANDS} barcode lands"
    );
    println!(
        "  Service envelopes:    front {FRONT_ROBOT_CLEARANCE:.0}mm, rear {REAR_VENT_SERVICE_CLEARANCE:.0}mm, left {LEFT_WASTE_CART_CLEARANCE:.0}mm, right {RIGHT_SAMPLE_SERVICE_CLEARANCE:.0}mm, top {TOP_FILTER_SWAP_CLEARANCE:.0}mm"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_z(part_height: f64) -> f64 {
    BASE_Z + 4.0 + part_height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(
        PRESSURE_TAP_COUNT,
        MULTI_CHIP_CASSETTES * PRESSURE_TAPS_PER_CASSETTE
    );
    assert_eq!(
        SPLIT_SAMPLE_PORTS,
        MULTI_CHIP_CASSETTES * SAMPLE_BRANCHES_PER_CASSETTE
    );
    assert_eq!(CHECK_VALVE_NESTS, MULTI_CHIP_CASSETTES);
    assert_eq!(CAP_PARKS, MULTI_CHIP_CASSETTES);
    assert_eq!(ROBOT_SERVICE_DATUMS, 4);
    assert_eq!(ROUTE_CHANNELS, MULTI_CHIP_CASSETTES + 2);
    assert!(LEAK_WITNESS_COUPONS > CONDENSATE_WITNESS_COUPONS);
    assert!(VENT_FILTER_LENGTH > VENT_FILTER_D * 3.5);
    assert!(BASIN_X < STATION_X - 2.0 * RIM_W);
    assert!(BASIN_Y < STATION_Y - 2.0 * RIM_W);
    assert!(TOP_FILTER_SWAP_CLEARANCE > VENT_Z + VENT_FILTER_D);

    for rect in layout_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds usable containment deck envelope",
            rect.name
        );
    }

    let rects = layout_rects();
    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                !left.overlaps_with_clearance(*right, 8.0),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn layout_rects() -> [Rect; 10] {
    [
        Rect {
            name: "multi_chip_waste_inlet_bulkhead",
            center: INLET_POS,
            x: INLET_X,
            y: INLET_Y,
        },
        Rect {
            name: "pressure_tap_ladder_manifold",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "hydrophobic_vent_sterile_barrier_witness_pockets",
            center: VENT_POS,
            x: VENT_X,
            y: VENT_Y,
        },
        Rect {
            name: "overflow_catch_trays",
            center: OVERFLOW_POS,
            x: OVERFLOW_X,
            y: OVERFLOW_Y,
        },
        Rect {
            name: "check_valve_surrogate_nests",
            center: CHECK_POS,
            x: CHECK_X,
            y: CHECK_Y,
        },
        Rect {
            name: "split_sample_ports",
            center: SAMPLE_POS,
            x: SAMPLE_X,
            y: SAMPLE_Y,
        },
        Rect {
            name: "leak_condensate_witness_coupons",
            center: COUPON_POS,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "quarantine_release_gate_panel",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Rect {
            name: "robotic_service_datums",
            center: ROBOT_POS,
            x: ROBOT_X,
            y: ROBOT_Y,
        },
        Rect {
            name: "barcode_custody_features",
            center: BARCODE_POS,
            x: BARCODE_X,
            y: BARCODE_Y,
        },
    ]
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_recessed_waste_basin"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, -10.0, BASE_Z - BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        format!("{PREFIX}_low_point_drain_bore"),
        DRAIN_PORT_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 20.0,
        BASE_Z - 8.0,
    );

    deck - basin - drain - fixture_sockets() - mounting_holes()
        + containment_rims()
        + basin_flow_ribs()
        + wet_dry_zone_lands()
}

fn fixture_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_registration_sockets"));
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_hole_cuts"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn containment_rims() -> Part {
    let front = centered_cube(format!("{PREFIX}_front_spill_rim"), STATION_X, RIM_W, RIM_Z)
        .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(format!("{PREFIX}_rear_spill_rim"), STATION_X, RIM_W, RIM_Z)
        .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(format!("{PREFIX}_left_spill_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(format!("{PREFIX}_right_spill_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn basin_flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_basin_route_stability_flow_ribs"));
    for i in 0..ROUTE_CHANNELS {
        let y = centered_index(i, ROUTE_CHANNELS, 72.0) - 28.0;
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_waste_route_witness_rib_{i}"),
                BASIN_X - 140.0,
                7.0,
                7.0,
            )
            .translate(0.0, y, BASE_Z + 3.5);
    }
    ribs
}

fn wet_dry_zone_lands() -> Part {
    let wet_land = centered_cube(
        format!("{PREFIX}_closed_waste_wet_zone_land"),
        455.0,
        34.0,
        5.0,
    )
    .translate(-340.0, STATION_Y / 2.0 - 82.0, BASE_Z + 2.5);
    let pressure_land = centered_cube(
        format!("{PREFIX}_pressure_evidence_zone_land"),
        300.0,
        34.0,
        5.0,
    )
    .translate(72.0, STATION_Y / 2.0 - 82.0, BASE_Z + 2.5);
    let custody_land = centered_cube(format!("{PREFIX}_dry_custody_zone_land"), 270.0, 34.0, 5.0)
        .translate(448.0, STATION_Y / 2.0 - 82.0, BASE_Z + 2.5);
    wet_land + pressure_land + custody_land
}

fn multi_chip_waste_inlet_bulkhead() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_multi_chip_waste_inlet_bulkhead_body"),
        INLET_X,
        INLET_Y,
        INLET_Z,
    );
    let gasket_land = centered_cube(
        format!("{PREFIX}_closed_waste_inlet_compression_gasket_land"),
        INLET_X - 32.0,
        12.0,
        INLET_Z + 8.0,
    )
    .translate(0.0, -INLET_Y / 2.0 + 18.0, 0.0);
    let mut ports = Part::empty(format!("{PREFIX}_multi_chip_inlet_port_cuts"));
    let mut labels = Part::empty(format!("{PREFIX}_multi_chip_lane_label_lands"));
    for lane in 0..MULTI_CHIP_CASSETTES {
        let x = centered_index(lane, MULTI_CHIP_CASSETTES, INLET_PITCH);
        ports = ports
            + centered_cylinder(
                format!("{PREFIX}_cassette_{lane}_closed_waste_inlet_port"),
                INLET_PORT_D / 2.0,
                INLET_Y + 18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 8.0);
        labels = labels
            + centered_cube(
                format!("{PREFIX}_cassette_{lane}_route_identity_land"),
                26.0,
                18.0,
                5.0,
            )
            .translate(x, INLET_Y / 2.0 - 18.0, INLET_Z / 2.0 + 2.5);
    }

    body - ports + gasket_land + labels + cap_park_posts() + inlet_route_strain_relief_comb()
}

fn cap_park_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_closed_cap_parking_posts"));
    for i in 0..CAP_PARKS {
        let x = centered_index(i, CAP_PARKS, INLET_PITCH);
        let post = centered_cylinder(format!("{PREFIX}_cap_park_post_{i}"), 9.0, 24.0, 24)
            .translate(x, INLET_Y / 2.0 + 16.0, INLET_Z / 2.0 - 6.0);
        let tether_slot = centered_cube(format!("{PREFIX}_cap_tether_slot_{i}"), 4.0, 18.0, 8.0)
            .translate(x, INLET_Y / 2.0 + 16.0, INLET_Z / 2.0 - 6.0);
        posts = posts + (post - tether_slot);
    }
    posts
}

fn inlet_route_strain_relief_comb() -> Part {
    let spine = centered_cube(
        format!("{PREFIX}_waste_line_route_stability_comb_spine"),
        INLET_X - 34.0,
        10.0,
        16.0,
    )
    .translate(0.0, -INLET_Y / 2.0 - 13.0, INLET_Z / 2.0 - 8.0);
    let mut teeth = Part::empty(format!("{PREFIX}_route_stability_comb_teeth"));
    for i in 0..ROUTE_STABILITY_CLAMPS {
        let x = centered_index(i, ROUTE_STABILITY_CLAMPS, 30.0);
        teeth = teeth
            + centered_cube(
                format!("{PREFIX}_route_stability_clamp_tooth_{i}"),
                8.0,
                22.0,
                18.0,
            )
            .translate(x, -INLET_Y / 2.0 - 24.0, INLET_Z / 2.0 - 6.0);
    }
    spine + teeth
}

fn pressure_tap_ladder_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_pressure_tap_ladder_manifold_body"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let trunk = centered_cylinder(
        format!("{PREFIX}_waste_pressure_trunk_bore"),
        6.0,
        PRESSURE_X + 24.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -48.0, -8.0);
    let mut tap_cuts = Part::empty(format!("{PREFIX}_pressure_tap_ladder_cuts"));
    let mut pads = Part::empty(format!("{PREFIX}_pressure_sensor_pad_ladder"));
    for lane in 0..MULTI_CHIP_CASSETTES {
        let x = centered_index(lane, MULTI_CHIP_CASSETTES, PRESSURE_LANE_PITCH);
        for level in 0..PRESSURE_TAPS_PER_CASSETTE {
            let z = -18.0 + level as f64 * PRESSURE_LEVEL_PITCH;
            tap_cuts = tap_cuts
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_pressure_tap_level_{level}"),
                    PRESSURE_TAP_D / 2.0,
                    PRESSURE_Y + 18.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, 0.0, z);
            pads = pads
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_pressure_sensor_pad_{level}"),
                    24.0,
                    16.0,
                    5.0,
                )
                .translate(x, PRESSURE_Y / 2.0 - 18.0, z);
        }
        pads = pads
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_vertical_tap_ladder_backbone"),
                9.0,
                10.0,
                76.0,
            )
            .translate(x, PRESSURE_Y / 2.0 - 34.0, 0.0);
    }

    body - trunk - tap_cuts + pads + pressure_range_steps()
}

fn pressure_range_steps() -> Part {
    let mut steps = Part::empty(format!("{PREFIX}_pressure_range_step_stair"));
    for i in 0..PRESSURE_TAPS_PER_CASSETTE {
        steps = steps
            + centered_cube(
                format!("{PREFIX}_pressure_range_step_{i}"),
                52.0 + i as f64 * 28.0,
                12.0,
                6.0,
            )
            .translate(
                PRESSURE_X / 2.0 - 68.0,
                -PRESSURE_Y / 2.0 + 22.0 + i as f64 * 22.0,
                PRESSURE_Z / 2.0 + 3.0,
            );
    }
    steps
}

fn hydrophobic_vent_sterile_barrier_witness_pockets() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_vent_barrier_witness_pocket_body"),
        VENT_X,
        VENT_Y,
        VENT_Z,
    );
    let mut vent_sockets = Part::empty(format!("{PREFIX}_hydrophobic_vent_socket_cuts"));
    let mut rings = Part::empty(format!("{PREFIX}_hydrophobic_vent_retention_rings"));
    let mut witness = Part::empty(format!("{PREFIX}_sterile_barrier_witness_pockets"));
    for i in 0..HYDROPHOBIC_VENTS {
        let x = centered_index(i, HYDROPHOBIC_VENTS, 74.0);
        vent_sockets = vent_sockets
            + centered_cylinder(
                format!("{PREFIX}_hydrophobic_vent_filter_socket_{i}"),
                VENT_FILTER_D / 2.0,
                VENT_FILTER_LENGTH,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, -34.0, 18.0);
        rings = rings
            + centered_cylinder(
                format!("{PREFIX}_hydrophobic_vent_filter_retention_ring_{i}"),
                VENT_FILTER_D / 2.0 + 5.0,
                8.0,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, -34.0, 18.0);
    }
    for i in 0..STERILE_BARRIER_WITNESS_POCKETS {
        let x = centered_index(i, STERILE_BARRIER_WITNESS_POCKETS, 74.0);
        let cup = centered_cylinder(
            format!("{PREFIX}_sterile_barrier_witness_cup_{i}"),
            16.0,
            10.0,
            36,
        )
        .translate(x, 48.0, VENT_Z / 2.0 + 5.0);
        let well = centered_cylinder(
            format!("{PREFIX}_sterile_barrier_witness_well_{i}"),
            10.0,
            12.0,
            32,
        )
        .translate(x, 48.0, VENT_Z / 2.0 + 5.0);
        witness = witness + (cup - well);
    }

    body - vent_sockets + rings + witness + sterile_barrier_status_tabs()
}

fn sterile_barrier_status_tabs() -> Part {
    let accept = centered_cube(
        format!("{PREFIX}_sterile_barrier_pass_land"),
        92.0,
        18.0,
        6.0,
    )
    .translate(-80.0, VENT_Y / 2.0 - 18.0, VENT_Z / 2.0 + 3.0);
    let suspect = centered_cube(
        format!("{PREFIX}_sterile_barrier_suspect_land"),
        92.0,
        18.0,
        6.0,
    )
    .translate(80.0, VENT_Y / 2.0 - 18.0, VENT_Z / 2.0 + 3.0);
    accept + suspect
}

fn overflow_catch_trays() -> Part {
    let tray_block = centered_cube(
        format!("{PREFIX}_overflow_catch_tray_carrier"),
        OVERFLOW_X,
        OVERFLOW_Y,
        OVERFLOW_Z,
    );
    let mut tray_cuts = Part::empty(format!("{PREFIX}_overflow_tray_basin_cuts"));
    let mut weirs = Part::empty(format!("{PREFIX}_overflow_weir_level_witnesses"));
    for i in 0..OVERFLOW_CATCH_TRAYS {
        let y = centered_index(i, OVERFLOW_CATCH_TRAYS, 58.0);
        tray_cuts = tray_cuts
            + centered_cube(
                format!("{PREFIX}_overflow_catch_tray_recess_{i}"),
                OVERFLOW_TRAY_X,
                OVERFLOW_TRAY_Y,
                OVERFLOW_TRAY_DEPTH,
            )
            .translate(0.0, y, OVERFLOW_Z / 2.0 - OVERFLOW_TRAY_DEPTH / 2.0 + 1.0);
        weirs = weirs
            + centered_cube(
                format!("{PREFIX}_overflow_tray_weir_{i}"),
                OVERFLOW_TRAY_X - 28.0,
                5.0,
                22.0,
            )
            .translate(
                0.0,
                y + OVERFLOW_TRAY_Y / 2.0 - 11.0,
                OVERFLOW_Z / 2.0 + 11.0,
            );
    }

    tray_block - tray_cuts + weirs + overflow_float_ticks()
}

fn overflow_float_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_overflow_float_level_ticks"));
    for i in 0..FLOAT_LEVEL_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_overflow_float_tick_{i}"),
                34.0 + i as f64 * 14.0,
                5.0,
                6.0,
            )
            .translate(
                -OVERFLOW_X / 2.0 + 50.0,
                -OVERFLOW_Y / 2.0 + 34.0 + i as f64 * 28.0,
                OVERFLOW_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn check_valve_surrogate_nests() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_check_valve_surrogate_nest_body"),
        CHECK_X,
        CHECK_Y,
        CHECK_Z,
    );
    let mut sockets = Part::empty(format!("{PREFIX}_check_valve_surrogate_socket_cuts"));
    let mut arrows = Part::empty(format!("{PREFIX}_check_valve_direction_witness_arrows"));
    for i in 0..CHECK_VALVE_NESTS {
        let x = centered_index(i, CHECK_VALVE_NESTS, CHECK_VALVE_PITCH);
        sockets = sockets
            + centered_cylinder(
                format!("{PREFIX}_check_valve_surrogate_nest_{i}"),
                CHECK_VALVE_D / 2.0,
                CHECK_Y + 12.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 4.0);
        arrows = arrows
            + centered_cube(
                format!("{PREFIX}_check_valve_flow_arrow_tail_{i}"),
                24.0,
                7.0,
                6.0,
            )
            .translate(x - 8.0, CHECK_Y / 2.0 - 26.0, CHECK_Z / 2.0 + 3.0)
            + centered_cube(
                format!("{PREFIX}_check_valve_flow_arrow_head_{i}"),
                11.0,
                17.0,
                6.0,
            )
            .translate(x + 12.0, CHECK_Y / 2.0 - 26.0, CHECK_Z / 2.0 + 3.0);
    }
    body - sockets + arrows + check_valve_lot_lands()
}

fn check_valve_lot_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_check_valve_lot_lands"));
    for i in 0..CHECK_VALVE_NESTS {
        let x = centered_index(i, CHECK_VALVE_NESTS, CHECK_VALVE_PITCH);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_check_valve_lot_land_{i}"),
                32.0,
                16.0,
                5.0,
            )
            .translate(x, -CHECK_Y / 2.0 + 22.0, CHECK_Z / 2.0 + 2.5);
    }
    lands
}

fn split_sample_ports() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_split_sample_port_panel_body"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let trunk = centered_cylinder(
        format!("{PREFIX}_sample_split_source_trunk_bore"),
        6.0,
        SAMPLE_X + 22.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -54.0, -6.0);
    let mut port_cuts = Part::empty(format!("{PREFIX}_split_sample_port_cuts"));
    let mut branch_pads = Part::empty(format!("{PREFIX}_split_sample_branch_pads"));
    for lane in 0..MULTI_CHIP_CASSETTES {
        let x = centered_index(lane, MULTI_CHIP_CASSETTES, SAMPLE_LANE_PITCH);
        for branch in 0..SAMPLE_BRANCHES_PER_CASSETTE {
            let y = -8.0 + branch as f64 * 42.0;
            port_cuts = port_cuts
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_split_sample_port_{branch}"),
                    SAMPLE_PORT_D / 2.0,
                    SAMPLE_Z + 12.0,
                    24,
                )
                .translate(x, y, 0.0);
            branch_pads = branch_pads
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_sample_branch_{branch}_custody_pad"),
                    24.0,
                    20.0,
                    5.0,
                )
                .translate(x, y, SAMPLE_Z / 2.0 + 2.5);
        }
    }
    body - trunk - port_cuts + branch_pads + sample_pinchoff_gates()
}

fn sample_pinchoff_gates() -> Part {
    let mut gates = Part::empty(format!("{PREFIX}_sample_pinchoff_gate_tabs"));
    for lane in 0..MULTI_CHIP_CASSETTES {
        let x = centered_index(lane, MULTI_CHIP_CASSETTES, SAMPLE_LANE_PITCH);
        gates = gates
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_sample_pinchoff_gate_land"),
                28.0,
                16.0,
                8.0,
            )
            .translate(x, SAMPLE_Y / 2.0 - 22.0, SAMPLE_Z / 2.0 + 4.0);
    }
    gates
}

fn leak_condensate_witness_coupons() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_leak_condensate_witness_coupon_panel"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let gutter = centered_cube(
        format!("{PREFIX}_condensate_gutter_recess"),
        COUPON_X - 34.0,
        20.0,
        8.0,
    )
    .translate(0.0, -COUPON_Y / 2.0 + 24.0, COUPON_Z / 2.0 - 3.0);
    let mut coupons = Part::empty(format!("{PREFIX}_removable_witness_coupons"));
    for i in 0..WITNESS_COUPON_TOTAL {
        let row = i / 6;
        let col = i % 6;
        let x = centered_index(col, 6, 48.0);
        let y = 2.0 + row as f64 * 42.0;
        let coupon_x = if i < LEAK_WITNESS_COUPONS { 34.0 } else { 28.0 };
        coupons =
            coupons
                + centered_cube(format!("{PREFIX}_witness_coupon_{i}"), coupon_x, 28.0, 5.0)
                    .translate(x, y, COUPON_Z / 2.0 + 2.5);
    }
    panel - gutter + coupons
}

fn quarantine_release_gate_panel() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_quarantine_release_gate_panel_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = Part::empty(format!("{PREFIX}_quarantine_release_gate_slides"));
    let mut token_pockets = Part::empty(format!("{PREFIX}_quarantine_release_token_pockets"));
    for lane in 0..DISPOSITION_GATE_LANES {
        let x = centered_index(lane, DISPOSITION_GATE_LANES, 96.0);
        gates = gates
            + centered_cube(
                format!("{PREFIX}_disposition_gate_lane_{lane}"),
                72.0,
                20.0,
                8.0,
            )
            .translate(x, -22.0, GATE_Z / 2.0 + 4.0)
            + centered_cube(
                format!("{PREFIX}_disposition_gate_handle_{lane}"),
                18.0,
                32.0,
                16.0,
            )
            .translate(x, 8.0, GATE_Z / 2.0 + 8.0);
        for token in 0..GATE_TOKEN_POCKETS_PER_LANE {
            token_pockets = token_pockets
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_gate_token_pocket_{token}"),
                    15.0,
                    12.0,
                    6.0,
                )
                .translate(
                    x - 26.0 + token as f64 * 17.0,
                    GATE_Y / 2.0 - 20.0,
                    GATE_Z / 2.0 + 3.0,
                );
        }
    }

    body + gates + token_pockets + disposition_lane_bars()
}

fn disposition_lane_bars() -> Part {
    let release = centered_cube(format!("{PREFIX}_release_lane_bar"), 96.0, 8.0, 7.0).translate(
        -96.0,
        -GATE_Y / 2.0 + 18.0,
        GATE_Z / 2.0 + 3.5,
    );
    let hold = centered_cube(format!("{PREFIX}_hold_lane_bar"), 96.0, 8.0, 7.0).translate(
        0.0,
        -GATE_Y / 2.0 + 18.0,
        GATE_Z / 2.0 + 3.5,
    );
    let quarantine = centered_cube(format!("{PREFIX}_quarantine_lane_bar"), 96.0, 8.0, 7.0)
        .translate(96.0, -GATE_Y / 2.0 + 18.0, GATE_Z / 2.0 + 3.5);
    release + hold + quarantine
}

fn robotic_service_datums() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_robotic_service_datum_plate"),
        ROBOT_X,
        ROBOT_Y,
        ROBOT_Z,
    );
    let mut datums = Part::empty(format!("{PREFIX}_robotic_service_datum_targets"));
    for (i, (x, y)) in [
        (-ROBOT_X / 2.0 + 48.0, -ROBOT_Y / 2.0 + 34.0),
        (ROBOT_X / 2.0 - 48.0, -ROBOT_Y / 2.0 + 34.0),
        (-ROBOT_X / 2.0 + 48.0, ROBOT_Y / 2.0 - 34.0),
        (ROBOT_X / 2.0 - 48.0, ROBOT_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("{PREFIX}_robot_service_datum_ring_{i}"),
            17.0,
            5.0,
            36,
        )
        .translate(*x, *y, ROBOT_Z / 2.0 + 2.5)
            - centered_cylinder(
                format!("{PREFIX}_robot_service_datum_bore_{i}"),
                5.0,
                7.0,
                24,
            )
            .translate(*x, *y, ROBOT_Z / 2.0 + 2.5);
        datums = datums + target;
    }
    plate + datums + service_keepout_posts()
}

fn service_keepout_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_robot_service_keepout_gauge_posts"));
    for (i, x) in [-96.0, 0.0, 96.0].iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_robot_service_keepout_gauge_{i}"),
                18.0,
                18.0,
                SERVICE_KEEP_OUT_Z,
            )
            .translate(*x, 0.0, ROBOT_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0);
    }
    posts
}

fn barcode_custody_features() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_custody_feature_plate"),
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    let mut barcode_lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i, BARCODE_LANDS, 78.0);
        barcode_lands = barcode_lands
            + centered_cube(format!("{PREFIX}_barcode_land_{i}"), 58.0, 18.0, 4.0).translate(
                x,
                -10.0,
                BARCODE_Z / 2.0 + 2.0,
            );
    }

    let mut custody = Part::empty(format!("{PREFIX}_custody_seal_and_rfid_lands"));
    for i in 0..CUSTODY_SEAL_POINTS {
        let x = centered_index(i, CUSTODY_SEAL_POINTS, 64.0);
        custody = custody
            + centered_cylinder(
                format!("{PREFIX}_custody_tamper_seal_point_{i}"),
                7.5,
                5.0,
                24,
            )
            .translate(x, 17.0, BARCODE_Z / 2.0 + 2.5);
    }
    for i in 0..RFID_LANDS {
        let x = -BARCODE_X / 2.0 + 72.0 + i as f64 * 54.0;
        custody = custody
            + centered_cube(format!("{PREFIX}_rfid_land_{i}"), 36.0, 18.0, 4.0).translate(
                x,
                17.0,
                BARCODE_Z / 2.0 + 2.0,
            );
    }

    plate + barcode_lands + custody + custody_direction_tabs()
}

fn custody_direction_tabs() -> Part {
    let inbound = centered_cube(
        format!("{PREFIX}_custody_inbound_direction_tab"),
        120.0,
        10.0,
        5.0,
    )
    .translate(
        -BARCODE_X / 2.0 + 120.0,
        -BARCODE_Y / 2.0 - 8.0,
        BARCODE_Z / 2.0 + 2.5,
    );
    let release = centered_cube(
        format!("{PREFIX}_custody_release_direction_tab"),
        120.0,
        10.0,
        5.0,
    )
    .translate(
        BARCODE_X / 2.0 - 120.0,
        -BARCODE_Y / 2.0 - 8.0,
        BARCODE_Z / 2.0 + 2.5,
    );
    inbound + release
}

fn waste_routing_stability_spine() -> Part {
    let mut routes = Part::empty(format!("{PREFIX}_waste_routing_stability_spine"));
    for lane in 0..MULTI_CHIP_CASSETTES {
        let y = 184.0 - lane as f64 * 48.0;
        routes = routes
            + centered_cylinder(
                format!("{PREFIX}_cassette_{lane}_waste_route_witness_channel"),
                ROUTE_BORE_D / 2.0,
                730.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-130.0, y, BASE_Z + 18.0);
    }
    for i in 0..ROUTE_STABILITY_CLAMPS {
        routes = routes
            + centered_cube(
                format!("{PREFIX}_assembly_route_stability_clamp_{i}"),
                22.0,
                38.0,
                16.0,
            )
            .translate(-420.0 + i as f64 * 110.0, -148.0, BASE_Z + 20.0);
    }
    routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_count_and_prefix_are_stable() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(&format!("output/{PREFIX}_"))));
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn key_dimension_counts_match_multi_chip_layout() {
        assert_eq!(MULTI_CHIP_CASSETTES, 6);
        assert_eq!(PRESSURE_TAP_COUNT, 24);
        assert_eq!(SPLIT_SAMPLE_PORTS, 12);
        assert_eq!(CHECK_VALVE_NESTS, MULTI_CHIP_CASSETTES);
        assert_eq!(ROUTE_CHANNELS, MULTI_CHIP_CASSETTES + 2);
    }

    #[test]
    fn contamination_witness_features_are_present() {
        assert_eq!(HYDROPHOBIC_VENTS, STERILE_BARRIER_WITNESS_POCKETS);
        assert_eq!(WITNESS_COUPON_TOTAL, 12);
        assert!(OVERFLOW_CATCH_TRAYS >= DISPOSITION_GATE_LANES);
        assert!(LEAK_WITNESS_COUPONS >= MULTI_CHIP_CASSETTES);
        assert!(BARCODE_LANDS > CUSTODY_SEAL_POINTS);
    }

    #[test]
    fn modules_fit_without_unplanned_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn required_features_have_named_exports_or_assembly_routes() {
        for feature in REQUIRED_FEATURES {
            let represented = OUTPUTS.iter().any(|path| path.contains(feature))
                || feature == "waste_routing_stability_spine";
            assert!(represented, "{feature} is not represented");
        }
    }
}
