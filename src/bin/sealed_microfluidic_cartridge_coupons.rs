use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// First-pass design coupons for the sealed disposable tissue-chip cartridge.
//
// Architecture:
// - Disposable wet path: rigid thermoplastic channel plate plus thin elastomer
//   membrane over valve/pump regions and a bonded optical cover.
// - Reusable machine: clamp nest, alignment pins, plunger/pneumatic actuator
//   comb, heater/imaging/sensors. It never contacts cell media.
// - Development order: valve coupon, bubble trap coupon, single tissue chamber
//   coupon, dock fixture, then a four-lane manifold coupon before 16 lanes.
//
// Research assumptions captured in the companion design note:
// - Thermoplastic COC/COP/PMMA body for optical clarity and scale-up.
// - eCOC/TPE/silicone-class membrane region, tested before committing material.
// - E-beam or EO-compatible geometry; avoid steam/dry heat for thermoplastics.

const OUTPUTS: &[&str] = &[
    "output/sealed_cartridge_membrane_valve_coupon.stl",
    "output/sealed_cartridge_bubble_trap_coupon.stl",
    "output/sealed_cartridge_single_tissue_chamber_coupon.stl",
    "output/sealed_cartridge_reusable_dock_fixture.stl",
    "output/sealed_cartridge_four_lane_manifold_coupon.stl",
    "output/sealed_cartridge_coupon_assembly.stl",
    "output/sealed_cartridge_coupon_design_note.txt",
];

const CHANNEL_W: f64 = 0.6;
const CHANNEL_D: f64 = 0.30;
const VALVE_SEAT_W: f64 = 1.1;
const MEMBRANE_WINDOW_D: f64 = 8.0;
const PORT_D: f64 = 1.6;
const ALIGN_D: f64 = 2.0;
const M3_CLEARANCE_D: f64 = 3.3;

fn main() {
    fs::create_dir_all("output").unwrap();

    let valve = membrane_valve_coupon();
    export(&valve, OUTPUTS[0]);

    let bubble = bubble_trap_coupon();
    export(&bubble, OUTPUTS[1]);

    let chamber = single_tissue_chamber_coupon();
    export(&chamber, OUTPUTS[2]);

    let dock = reusable_dock_fixture();
    export(&dock, OUTPUTS[3]);

    let manifold = four_lane_manifold_coupon();
    export(&manifold, OUTPUTS[4]);

    let assembly = valve.translate(-115.0, 52.0, 0.0)
        + bubble.translate(0.0, 52.0, 0.0)
        + chamber.translate(115.0, 52.0, 0.0)
        + dock.translate(-78.0, -66.0, 0.0)
        + manifold.translate(88.0, -66.0, 0.0);
    export(&assembly, OUTPUTS[5]);

    fs::write(OUTPUTS[6], design_note()).unwrap();

    println!(
        "Sealed cartridge coupons exported: membrane valve, bubble trap, tissue chamber, reusable dock, four-lane manifold, and assembly."
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn membrane_valve_coupon() -> Part {
    let x = 96.0;
    let y = 38.0;
    let z = 3.0;
    let top = z / 2.0;
    let mut body = centered_cube("valve_coupon_channel_plate", x, y, z);

    // Straight 600 um channel with three valve-seat constrictions for A/B/C tests.
    let channel = centered_cube(
        "valve_coupon_straight_channel",
        x - 24.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(0.0, 0.0, top - CHANNEL_D / 2.0 + 0.1);
    body = body - channel;

    for (i, vx) in [-28.0, 0.0, 28.0].iter().enumerate() {
        let seat = centered_cube(
            format!("valve_coupon_seat_{i}"),
            VALVE_SEAT_W,
            MEMBRANE_WINDOW_D,
            CHANNEL_D + 0.25,
        )
        .translate(*vx, 0.0, top - CHANNEL_D / 2.0 + 0.12);
        body = body - seat;

        let membrane_recess = centered_cylinder(
            format!("valve_coupon_membrane_recess_{i}"),
            MEMBRANE_WINDOW_D / 2.0,
            0.35,
            48,
        )
        .translate(*vx, 0.0, top - 0.15);
        body = body - membrane_recess;
    }

    for (i, px) in [-(x / 2.0 - 8.0), x / 2.0 - 8.0].iter().enumerate() {
        let port = centered_cylinder(
            format!("valve_coupon_edge_port_{i}"),
            PORT_D / 2.0,
            z + 1.0,
            32,
        )
        .translate(*px, 0.0, 0.0);
        body = body - port;
    }

    body = subtract_corner_alignment(body, x, y, z, 5.0);

    // Clamp rails represent where the disposable membrane is compressed by a
    // reusable actuator head. The membrane itself is intentionally not modeled
    // as solid wet-path material in this first manufacturability coupon.
    let rail_left = centered_cube("valve_coupon_membrane_clamp_rail_l", x - 12.0, 2.0, 1.2)
        .translate(0.0, -7.0, top + 0.6);
    let rail_right = centered_cube("valve_coupon_membrane_clamp_rail_r", x - 12.0, 2.0, 1.2)
        .translate(0.0, 7.0, top + 0.6);
    let actuator_datums = actuator_buttons(
        "valve_coupon_actuator_datums",
        &[-28.0, 0.0, 28.0],
        0.0,
        top + 1.45,
    );

    body + rail_left + rail_right + actuator_datums
}

fn bubble_trap_coupon() -> Part {
    let x = 86.0;
    let y = 42.0;
    let z = 5.0;
    let top = z / 2.0;
    let mut body = centered_cube("bubble_trap_channel_plate", x, y, z);

    let chamber = centered_cylinder("bubble_trap_expansion_chamber", 12.0, 2.2, 64).translate(
        0.0,
        0.0,
        top - 1.0,
    );
    let vent_membrane_recess = centered_cylinder("bubble_trap_gas_membrane_window", 9.5, 0.45, 64)
        .translate(0.0, 0.0, top - 0.18);
    let inlet = centered_cube(
        "bubble_trap_inlet_channel",
        33.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(-28.5, -5.0, top - CHANNEL_D / 2.0 + 0.1);
    let outlet = centered_cube(
        "bubble_trap_outlet_channel",
        33.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(28.5, 5.0, top - CHANNEL_D / 2.0 + 0.1);
    let inlet_tangent = centered_cube("bubble_trap_inlet_tangent", 0.8, 10.0, CHANNEL_D + 0.2)
        .translate(-12.0, 0.0, top - CHANNEL_D / 2.0 + 0.1);
    let outlet_tangent = centered_cube("bubble_trap_outlet_tangent", 0.8, 10.0, CHANNEL_D + 0.2)
        .translate(12.0, 0.0, top - CHANNEL_D / 2.0 + 0.1);

    body = body - chamber - vent_membrane_recess - inlet - outlet - inlet_tangent - outlet_tangent;

    for (i, (px, py)) in [(-(x / 2.0 - 7.0), -5.0), (x / 2.0 - 7.0, 5.0)]
        .iter()
        .enumerate()
    {
        let port = centered_cylinder(
            format!("bubble_trap_edge_port_{i}"),
            PORT_D / 2.0,
            z + 1.0,
            32,
        )
        .translate(*px, *py, 0.0);
        body = body - port;
    }

    body = subtract_corner_alignment(body, x, y, z, 5.0);

    let vent_ring = centered_cylinder("bubble_trap_membrane_clamp_outer", 14.5, 1.0, 64).translate(
        0.0,
        0.0,
        top + 0.5,
    ) - centered_cylinder("bubble_trap_membrane_clamp_inner", 10.0, 1.2, 64)
        .translate(0.0, 0.0, top + 0.5);
    let flow_arrow_land = centered_cube("bubble_trap_direction_land", x - 18.0, 2.0, 0.8)
        .translate(0.0, y / 2.0 - 8.0, top + 0.4);

    body + vent_ring + flow_arrow_land
}

fn single_tissue_chamber_coupon() -> Part {
    let x = 75.0;
    let y = 25.0;
    let z = 4.0;
    let top = z / 2.0;
    let mut body = centered_cube("single_tissue_chamber_body", x, y, z);

    let chamber = centered_cube("single_tissue_culture_chamber", 22.0, 9.0, 0.55).translate(
        0.0,
        0.0,
        top - 0.23,
    );
    let imaging_window = centered_cube("single_tissue_imaging_window_recess", 28.0, 13.0, 0.18)
        .translate(0.0, 0.0, top - 0.06);
    body = body - chamber - imaging_window;

    // Media perfusion path, left to right.
    let media_in = centered_cube("single_tissue_media_in", 25.0, CHANNEL_W, CHANNEL_D + 0.2)
        .translate(-23.5, -2.0, top - CHANNEL_D / 2.0 + 0.1);
    let media_out = centered_cube("single_tissue_media_out", 25.0, CHANNEL_W, CHANNEL_D + 0.2)
        .translate(23.5, 2.0, top - CHANNEL_D / 2.0 + 0.1);
    let chamber_bridge = centered_cube(
        "single_tissue_media_bridge",
        22.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(0.0, 0.0, top - CHANNEL_D / 2.0 + 0.1);
    body = body - media_in - media_out - chamber_bridge;

    // Separate gel/cell loading path. On the final cartridge this is fed by
    // internal valve routing; on this coupon it exposes side ports for testing.
    let gel_in = centered_cube(
        "single_tissue_gel_load_in",
        19.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(-17.5, 5.0, top - CHANNEL_D / 2.0 + 0.1);
    let gel_out = centered_cube(
        "single_tissue_gel_load_out",
        19.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(17.5, -5.0, top - CHANNEL_D / 2.0 + 0.1);
    let gel_cross_a = centered_cube("single_tissue_gel_cross_a", CHANNEL_W, 5.0, CHANNEL_D + 0.2)
        .translate(-8.0, 2.5, top - CHANNEL_D / 2.0 + 0.1);
    let gel_cross_b = centered_cube("single_tissue_gel_cross_b", CHANNEL_W, 5.0, CHANNEL_D + 0.2)
        .translate(8.0, -2.5, top - CHANNEL_D / 2.0 + 0.1);
    body = body - gel_in - gel_out - gel_cross_a - gel_cross_b;

    for (i, (px, py)) in [
        (-(x / 2.0 - 5.0), -2.0),
        (x / 2.0 - 5.0, 2.0),
        (-(x / 2.0 - 5.0), 5.0),
        (x / 2.0 - 5.0, -5.0),
    ]
    .iter()
    .enumerate()
    {
        let port = centered_cylinder(
            format!("single_tissue_side_port_{i}"),
            PORT_D / 2.0,
            z + 1.0,
            32,
        )
        .translate(*px, *py, 0.0);
        body = body - port;
    }

    let gasket_groove = centered_cube(
        "single_tissue_perimeter_gasket_groove_a",
        x - 8.0,
        1.0,
        0.35,
    )
    .translate(0.0, y / 2.0 - 4.0, top - 0.12)
        + centered_cube(
            "single_tissue_perimeter_gasket_groove_b",
            x - 8.0,
            1.0,
            0.35,
        )
        .translate(0.0, -(y / 2.0 - 4.0), top - 0.12)
        + centered_cube(
            "single_tissue_perimeter_gasket_groove_c",
            1.0,
            y - 8.0,
            0.35,
        )
        .translate(x / 2.0 - 4.0, 0.0, top - 0.12)
        + centered_cube(
            "single_tissue_perimeter_gasket_groove_d",
            1.0,
            y - 8.0,
            0.35,
        )
        .translate(-(x / 2.0 - 4.0), 0.0, top - 0.12);
    body = body - gasket_groove;

    subtract_corner_alignment(body, x, y, z, 4.0)
}

fn reusable_dock_fixture() -> Part {
    let x = 132.0;
    let y = 72.0;
    let z = 12.0;
    let mut dock = centered_cube("reusable_dock_base", x, y, z);

    let pocket = centered_cube("reusable_dock_cartridge_nest", 104.0, 46.0, 3.2).translate(
        0.0,
        0.0,
        z / 2.0 - 1.2,
    );
    dock = dock - pocket;

    for (i, (px, py)) in [
        (-50.0, -22.0),
        (-50.0, 22.0),
        (50.0, -22.0),
        (50.0, 22.0),
        (-60.0, 0.0),
        (60.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        let mount = centered_cylinder(
            format!("reusable_dock_mount_{i}"),
            M3_CLEARANCE_D / 2.0,
            z + 1.0,
            32,
        )
        .translate(*px, *py, 0.0);
        dock = dock - mount;
    }

    let mut pins = Part::empty("reusable_dock_alignment_pins");
    for (i, (px, py)) in [(-43.0, -17.0), (-43.0, 17.0), (43.0, -17.0), (43.0, 17.0)]
        .iter()
        .enumerate()
    {
        let pin = centered_cylinder(format!("reusable_dock_pin_{i}"), 1.0, 5.5, 32).translate(
            *px,
            *py,
            z / 2.0 + 2.75,
        );
        pins = pins + pin;
    }

    let actuator_bridge = centered_cube("reusable_dock_actuator_bridge", 112.0, 10.0, 18.0)
        .translate(0.0, -30.0, z / 2.0 + 9.0);
    let actuator_comb = actuator_buttons(
        "reusable_dock_lane_actuator_plungers",
        &[-36.0, -12.0, 12.0, 36.0],
        -30.0,
        z / 2.0 + 19.5,
    );
    let side_hardstop_l = centered_cube("reusable_dock_left_hardstop", 6.0, 48.0, 9.0).translate(
        -55.0,
        0.0,
        z / 2.0 + 4.5,
    );
    let side_hardstop_r = centered_cube("reusable_dock_right_hardstop", 6.0, 48.0, 9.0).translate(
        55.0,
        0.0,
        z / 2.0 + 4.5,
    );

    dock + pins + actuator_bridge + actuator_comb + side_hardstop_l + side_hardstop_r
}

fn four_lane_manifold_coupon() -> Part {
    let x = 150.0;
    let y = 82.0;
    let z = 4.0;
    let top = z / 2.0;
    let mut body = centered_cube("four_lane_cartridge_plate", x, y, z);

    let inlet_bus_y = -31.0;
    let outlet_bus_y = 31.0;
    let inlet_bus = centered_cube(
        "four_lane_common_inlet_bus",
        x - 26.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(0.0, inlet_bus_y, top - CHANNEL_D / 2.0 + 0.1);
    let outlet_bus = centered_cube(
        "four_lane_common_outlet_bus",
        x - 26.0,
        CHANNEL_W,
        CHANNEL_D + 0.2,
    )
    .translate(0.0, outlet_bus_y, top - CHANNEL_D / 2.0 + 0.1);
    body = body - inlet_bus - outlet_bus;

    for lane in 0..4 {
        let lx = lane_x(lane);
        let chamber = centered_cube(format!("four_lane_chamber_{lane}"), 18.0, 7.0, 0.55)
            .translate(lx, 0.0, top - 0.23);
        let in_leg = centered_cube(
            format!("four_lane_in_leg_{lane}"),
            CHANNEL_W,
            31.0,
            CHANNEL_D + 0.2,
        )
        .translate(lx - 5.0, -15.5, top - CHANNEL_D / 2.0 + 0.1);
        let out_leg = centered_cube(
            format!("four_lane_out_leg_{lane}"),
            CHANNEL_W,
            31.0,
            CHANNEL_D + 0.2,
        )
        .translate(lx + 5.0, 15.5, top - CHANNEL_D / 2.0 + 0.1);
        let chamber_in = centered_cube(
            format!("four_lane_chamber_in_{lane}"),
            5.0,
            CHANNEL_W,
            CHANNEL_D + 0.2,
        )
        .translate(lx - 2.5, -3.5, top - CHANNEL_D / 2.0 + 0.1);
        let chamber_out = centered_cube(
            format!("four_lane_chamber_out_{lane}"),
            5.0,
            CHANNEL_W,
            CHANNEL_D + 0.2,
        )
        .translate(lx + 2.5, 3.5, top - CHANNEL_D / 2.0 + 0.1);
        body = body - chamber - in_leg - out_leg - chamber_in - chamber_out;

        // Normally closed membrane valve seat before each lane.
        let seat = centered_cube(
            format!("four_lane_valve_seat_{lane}"),
            VALVE_SEAT_W,
            7.0,
            CHANNEL_D + 0.2,
        )
        .translate(lx - 5.0, -23.0, top - CHANNEL_D / 2.0 + 0.1);
        let valve_window =
            centered_cylinder(format!("four_lane_valve_window_{lane}"), 4.0, 0.35, 48).translate(
                lx - 5.0,
                -23.0,
                top - 0.15,
            );
        body = body - seat - valve_window;

        // Small upstream bubble expansion chamber per lane. The 16-lane version
        // can share a large debubbler upstream, but this coupon tests local traps.
        let bubble = centered_cylinder(format!("four_lane_bubble_chamber_{lane}"), 4.8, 0.9, 48)
            .translate(lx - 5.0, -12.0, top - 0.35);
        body = body - bubble;
    }

    for (i, (px, py)) in [
        (-(x / 2.0 - 8.0), inlet_bus_y),
        (x / 2.0 - 8.0, outlet_bus_y),
        (-(x / 2.0 - 8.0), outlet_bus_y),
        (x / 2.0 - 8.0, inlet_bus_y),
    ]
    .iter()
    .enumerate()
    {
        let port = centered_cylinder(
            format!("four_lane_edge_port_{i}"),
            PORT_D / 2.0,
            z + 1.0,
            32,
        )
        .translate(*px, *py, 0.0);
        body = body - port;
    }

    body = subtract_corner_alignment(body, x, y, z, 6.0);

    let membrane_clamp_bar = centered_cube("four_lane_membrane_clamp_bar", x - 22.0, 13.0, 1.1)
        .translate(0.0, -23.0, top + 0.55);
    let actuator_targets = actuator_buttons(
        "four_lane_actuator_targets",
        &[
            lane_x(0) - 5.0,
            lane_x(1) - 5.0,
            lane_x(2) - 5.0,
            lane_x(3) - 5.0,
        ],
        -23.0,
        top + 1.35,
    );

    body + membrane_clamp_bar + actuator_targets
}

fn subtract_corner_alignment(mut body: Part, x: f64, y: f64, z: f64, inset: f64) -> Part {
    for (i, (sx, sy)) in [(-1.0_f64, -1.0_f64), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let hole = centered_cylinder(format!("alignment_hole_{i}"), ALIGN_D / 2.0, z + 1.0, 24)
            .translate(sx * (x / 2.0 - inset), sy * (y / 2.0 - inset), 0.0);
        body = body - hole;
    }
    body
}

fn actuator_buttons(name: &str, xs: &[f64], y: f64, z: f64) -> Part {
    let mut buttons = Part::empty(name);
    for (i, x) in xs.iter().enumerate() {
        let button = centered_cylinder(format!("{name}_{i}"), 2.7, 1.2, 32).translate(*x, y, z);
        buttons = buttons + button;
    }
    buttons
}

fn lane_x(lane: usize) -> f64 {
    -45.0 + lane as f64 * 30.0
}

fn design_note() -> String {
    [
        "Sealed microfluidic cartridge CAD coupon set",
        "",
        "Design choice:",
        "- Build the final product around a sealed disposable cartridge, not loose chips plus lab tubing.",
        "- Put all wetted fluid paths, valve seats, chambers, bubble traps, and waste routing in the disposable.",
        "- Put force, motion, imaging, heating, sensing, and control electronics in the reusable instrument.",
        "",
        "Why this is the move:",
        "- Disposable membrane valves are a known pattern in IVD cartridges: elastic membrane over a rigid channel/seat, actuated externally.",
        "- eCOC/TPE-style membranes are more scalable than PDMS for disposable cartridges; silicone can still be a prototype material.",
        "- COC/COP/PMMA thermoplastic bodies are manufacturable by CNC/laser prototype, then hot embossing or injection molding later.",
        "- Sterilization must be designed early. Steam/dry heat can deform thermoplastics; e-beam, gamma, EO, or H2O2 need material validation.",
        "",
        "Coupon intent:",
        "1. Membrane valve coupon: test valve seat geometry, membrane compression, closing pressure, leak rate, and actuation life.",
        "2. Bubble trap coupon: test expansion chamber, vent membrane window, priming, and bubble capture before cells.",
        "3. Single tissue chamber coupon: test gel/cell loading path plus separate perfusion path in a sealed optical chip.",
        "4. Reusable dock fixture: test cartridge alignment, clamp force, actuator-to-valve registration, and no-contact reusable hardware.",
        "5. Four-lane manifold coupon: prove scaled routing before jumping to a full 16-lane panel.",
        "",
        "Next CAD step:",
        "- Convert the best coupon geometries into a 16-lane sealed cartridge with internal reservoirs/waste and a matching reusable actuator head.",
        "",
    ]
    .join("\n")
}
