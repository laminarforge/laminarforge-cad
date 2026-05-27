use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed module sterile-air purge duration validation station.
//
// Intent:
// - Package a closed-system validation fixture around module transfer docks,
//   service ports, and closed culture boxes.
// - Make the purge-flow path, purge timing, filtered vent envelope, stagnant
//   pockets, pressure cascade state, and release/hold/reject evidence visible
//   as mechanical validation hardware.
// - Keep custody lands, evidence camera geometry, and robot/service keepouts
//   explicit for parent integration.
//
// This is mechanical CAD packaging and validation hardware only. It is not a
// biosafety claim, sterile-process specification, purge duration release
// criterion, or operating SOP.

const PREFIX: &str = "closed_module_sterile_air_purge_duration_validation_station";
const OUTPUT_PREFIX: &str = "output/closed_module_sterile_air_purge_duration_validation_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_module_sterile_air_purge_duration_validation_station_containment_deck.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_module_surrogate_dock.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_sterile_filter_holder.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_purge_inlet_manifold.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_exhaust_return_witness_channel.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_flow_vane_grid.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_pressure_rh_particle_probe_pockets.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_timer_token_rail.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_stagnant_pocket_tracer_coupons.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_barcode_coa_custody_lands.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_release_hold_reject_gates.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_evidence_camera_robot_keepout_bridge.stl",
    "output/closed_module_sterile_air_purge_duration_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "containment_deck",
    "module_surrogate_dock",
    "sterile_filter_holder",
    "purge_inlet_manifold",
    "exhaust_return_witness_channel",
    "flow_vane_grid",
    "pressure_rh_particle_probe_pockets",
    "timer_token_rail",
    "stagnant_pocket_tracer_coupons",
    "barcode_coa_custody_lands",
    "release_hold_reject_gates",
    "evidence_camera_robot_keepout_bridge",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_packaging_only",
    "not_a_biosafety_claim",
    "not_a_purge_sop",
    "not_a_release_specification",
    "filter_media_and_sensors_are_placeholder_envelopes",
];

const STATION_X: f64 = 1480.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const CLEARANCE: f64 = 14.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;

const MODULES: usize = 4;
const SERVICE_PORTS_PER_MODULE: usize = 3;
const PURGE_STATES: usize = 4;
const DISPOSITION_STATES: usize = 3;

const DOCK_CENTER: (f64, f64) = (-420.0, 235.0);
const DOCK_X: f64 = 500.0;
const DOCK_Y: f64 = 250.0;
const DOCK_Z: f64 = 70.0;
const MODULE_PITCH_X: f64 = 112.0;
const MODULE_NEST_X: f64 = 84.0;
const MODULE_NEST_Y: f64 = 128.0;
const MODULE_NEST_Z: f64 = 28.0;

const FILTER_CENTER: (f64, f64) = (110.0, 235.0);
const FILTER_X: f64 = 390.0;
const FILTER_Y: f64 = 250.0;
const FILTER_Z: f64 = 86.0;
const FILTER_COUNT: usize = MODULES;
const FILTER_D: f64 = 46.0;
const FILTER_PITCH_X: f64 = 82.0;

const INLET_CENTER: (f64, f64) = (520.0, 235.0);
const INLET_X: f64 = 340.0;
const INLET_Y: f64 = 250.0;
const INLET_Z: f64 = 66.0;
const INLET_BRANCHES: usize = MODULES;
const BRANCH_BORE_D: f64 = 7.0;

const WITNESS_CENTER: (f64, f64) = (-420.0, -40.0);
const WITNESS_X: f64 = 500.0;
const WITNESS_Y: f64 = 230.0;
const WITNESS_Z: f64 = 54.0;
const WITNESS_CHANNELS: usize = MODULES;

const VANE_CENTER: (f64, f64) = (110.0, -40.0);
const VANE_X: f64 = 390.0;
const VANE_Y: f64 = 230.0;
const VANE_Z: f64 = 56.0;
const VANE_ROWS: usize = 3;
const VANE_COLS: usize = 6;
const VANE_COUNT: usize = VANE_ROWS * VANE_COLS;

const PROBE_CENTER: (f64, f64) = (520.0, -40.0);
const PROBE_X: f64 = 340.0;
const PROBE_Y: f64 = 230.0;
const PROBE_Z: f64 = 62.0;
const PRESSURE_PROBES: usize = MODULES + 1;
const RH_PROBES: usize = 3;
const PARTICLE_PROBES: usize = 3;

const TIMER_CENTER: (f64, f64) = (-520.0, -310.0);
const TIMER_X: f64 = 300.0;
const TIMER_Y: f64 = 150.0;
const TIMER_Z: f64 = 38.0;
const TIMER_TOKENS: usize = MODULES * PURGE_STATES;

const COUPON_CENTER: (f64, f64) = (-165.0, -310.0);
const COUPON_X: f64 = 330.0;
const COUPON_Y: f64 = 150.0;
const COUPON_Z: f64 = 42.0;
const TRACER_COUPONS: usize = 12;

const CUSTODY_CENTER: (f64, f64) = (190.0, -310.0);
const CUSTODY_X: f64 = 330.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 16.0;
const BARCODE_LANDS: usize = MODULES;
const COA_LANDS: usize = 3;

const GATE_CENTER: (f64, f64) = (545.0, -310.0);
const GATE_X: f64 = 300.0;
const GATE_Y: f64 = 150.0;
const GATE_Z: f64 = 46.0;
const GATE_SLOTS_PER_STATE: usize = MODULES;

const BRIDGE_CENTER: (f64, f64) = (0.0, 0.0);
const BRIDGE_X: f64 = 1280.0;
const BRIDGE_Y: f64 = 720.0;
const BRIDGE_Z: f64 = 170.0;
const CAMERA_WINDOWS: usize = 4;
const ROBOT_KEEP_OUT_GAUGES: usize = 6;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - CLEARANCE;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
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

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let dock = module_surrogate_dock().translate(DOCK_CENTER.0, DOCK_CENTER.1, insert_z(DOCK_Z));
    export(&dock, OUTPUTS[1]);

    let filters =
        sterile_filter_holder().translate(FILTER_CENTER.0, FILTER_CENTER.1, insert_z(FILTER_Z));
    export(&filters, OUTPUTS[2]);

    let inlet = purge_inlet_manifold().translate(INLET_CENTER.0, INLET_CENTER.1, insert_z(INLET_Z));
    export(&inlet, OUTPUTS[3]);

    let witness = exhaust_return_witness_channel().translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        insert_z(WITNESS_Z),
    );
    export(&witness, OUTPUTS[4]);

    let vanes = flow_vane_grid().translate(VANE_CENTER.0, VANE_CENTER.1, insert_z(VANE_Z));
    export(&vanes, OUTPUTS[5]);

    let probes = pressure_rh_particle_probe_pockets().translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1,
        insert_z(PROBE_Z),
    );
    export(&probes, OUTPUTS[6]);

    let timer = timer_token_rail().translate(TIMER_CENTER.0, TIMER_CENTER.1, insert_z(TIMER_Z));
    export(&timer, OUTPUTS[7]);

    let coupons = stagnant_pocket_tracer_coupons().translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        insert_z(COUPON_Z),
    );
    export(&coupons, OUTPUTS[8]);

    let custody = barcode_coa_custody_lands().translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_Z),
    );
    export(&custody, OUTPUTS[9]);

    let gates =
        release_hold_reject_gates().translate(GATE_CENTER.0, GATE_CENTER.1, insert_z(GATE_Z));
    export(&gates, OUTPUTS[10]);

    let bridge = evidence_camera_robot_keepout_bridge().translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        BASE_Z / 2.0 + BRIDGE_Z / 2.0,
    );
    export(&bridge, OUTPUTS[11]);

    let assembly = deck
        + dock
        + filters
        + inlet
        + witness
        + vanes
        + probes
        + timer
        + coupons
        + custody
        + gates
        + bridge;
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed module sterile-air purge duration validation station:");
    println!(
        "  Footprint:         {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with raised rim and drain witness gutters"
    );
    println!(
        "  Module transfer:  {MODULES} surrogate module nests, {} service-port witness pockets, and keyed return-to-culture custody lands",
        MODULES * SERVICE_PORTS_PER_MODULE
    );
    println!(
        "  Purge envelope:   {FILTER_COUNT} sterile filter holder envelopes, {INLET_BRANCHES} inlet branches, {WITNESS_CHANNELS} exhaust/return witness channels"
    );
    println!(
        "  Evidence:         {PRESSURE_PROBES} pressure, {RH_PROBES} RH, {PARTICLE_PROBES} particle probe pockets, {TIMER_TOKENS} timer tokens, {TRACER_COUPONS} stagnant-pocket coupons"
    );
    println!(
        "  Disposition:      {DISPOSITION_STATES} release/hold/reject gates with {GATE_SLOTS_PER_STATE} module slots per state"
    );
    println!("  Output prefix:    {OUTPUT_PREFIX}");
    println!("  Limitations:      {}", LIMITATIONS.join(", "));
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

fn station_rects() -> [Rect; 11] {
    [
        Rect {
            name: "module_surrogate_dock",
            center: DOCK_CENTER,
            x: DOCK_X,
            y: DOCK_Y,
        },
        Rect {
            name: "sterile_filter_holder",
            center: FILTER_CENTER,
            x: FILTER_X,
            y: FILTER_Y,
        },
        Rect {
            name: "purge_inlet_manifold",
            center: INLET_CENTER,
            x: INLET_X,
            y: INLET_Y,
        },
        Rect {
            name: "exhaust_return_witness_channel",
            center: WITNESS_CENTER,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "flow_vane_grid",
            center: VANE_CENTER,
            x: VANE_X,
            y: VANE_Y,
        },
        Rect {
            name: "pressure_rh_particle_probe_pockets",
            center: PROBE_CENTER,
            x: PROBE_X,
            y: PROBE_Y,
        },
        Rect {
            name: "timer_token_rail",
            center: TIMER_CENTER,
            x: TIMER_X,
            y: TIMER_Y,
        },
        Rect {
            name: "stagnant_pocket_tracer_coupons",
            center: COUPON_CENTER,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "barcode_coa_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "release_hold_reject_gates",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
        Rect {
            name: "camera_robot_keepout_projection",
            center: BRIDGE_CENTER,
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert_eq!(VANE_COUNT, VANE_ROWS * VANE_COLS);
    assert_eq!(FILTER_COUNT, MODULES);
    assert_eq!(INLET_BRANCHES, MODULES);
    assert_eq!(WITNESS_CHANNELS, MODULES);
    assert_eq!(TIMER_TOKENS, MODULES * PURGE_STATES);
    assert_eq!(DISPOSITION_STATES, 3);
    assert!(TRACER_COUPONS >= MODULES * SERVICE_PORTS_PER_MODULE);
    assert!(BRIDGE_Z > DOCK_Z + 70.0);

    let rects = station_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds deck envelope",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            if rects[i].name == "camera_robot_keepout_projection"
                || rects[j].name == "camera_robot_keepout_projection"
            {
                continue;
            }
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], 6.0),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_base_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let upper_basin = centered_cube(
        format!("{PREFIX}_base_module_purge_basin_recess"),
        1260.0,
        260.0,
        8.0,
    )
    .translate(-10.0, 235.0, BASE_Z / 2.0 - 4.0);
    let witness_basin = centered_cube(
        format!("{PREFIX}_base_witness_probe_basin_recess"),
        1260.0,
        240.0,
        8.0,
    )
    .translate(-10.0, -40.0, BASE_Z / 2.0 - 4.0);
    let evidence_basin = centered_cube(
        format!("{PREFIX}_base_evidence_disposition_basin_recess"),
        1260.0,
        170.0,
        8.0,
    )
    .translate(10.0, -310.0, BASE_Z / 2.0 - 4.0);

    deck - upper_basin
        - witness_basin
        - evidence_basin
        - insert_sockets()
        - mounting_holes()
        - low_point_drain_bores()
        + perimeter_rims()
        + zone_dividers()
        + cascade_state_gutters()
        + datum_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_insert_sockets"));
    for rect in station_rects() {
        if rect.name == "camera_robot_keepout_projection" {
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
    let mut holes = Part::empty(format!("{PREFIX}_mounting_holes"));
    for (i, (x, y)) in [
        (-660.0, 390.0),
        (660.0, 390.0),
        (-660.0, -390.0),
        (660.0, -390.0),
        (0.0, 390.0),
        (0.0, -390.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_clearance_slot_round_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("{PREFIX}_m6_clearance_slot_obround_{i}"),
                24.0,
                MOUNT_HOLE_D + 0.4,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn low_point_drain_bores() -> Part {
    let front = centered_cylinder(
        format!("{PREFIX}_front_evidence_low_point_drain"),
        6.0,
        58.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(615.0, -STATION_Y / 2.0 + 9.0, -1.0);
    let purge = centered_cylinder(
        format!("{PREFIX}_purge_zone_low_point_drain"),
        6.0,
        58.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(615.0, STATION_Y / 2.0 - 135.0, -1.0);
    front + purge
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y - 52.0,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y - 52.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X - 38.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        format!("{PREFIX}_front_evidence_low_lip"),
        STATION_X - 160.0,
        14.0,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 34.0, BASE_Z / 2.0 + 14.0);
    left + right + rear + front
}

fn zone_dividers() -> Part {
    let row_a = centered_cube(
        format!("{PREFIX}_purge_to_witness_zone_divider"),
        1240.0,
        12.0,
        32.0,
    )
    .translate(0.0, 98.0, BASE_Z / 2.0 + 16.0);
    let row_b = centered_cube(
        format!("{PREFIX}_witness_to_evidence_zone_divider"),
        1240.0,
        12.0,
        32.0,
    )
    .translate(0.0, -178.0, BASE_Z / 2.0 + 16.0);
    let dock_filter = centered_cube(
        format!("{PREFIX}_dock_to_filter_zone_divider"),
        10.0,
        238.0,
        30.0,
    )
    .translate(-155.0, 235.0, BASE_Z / 2.0 + 15.0);
    let filter_inlet = centered_cube(
        format!("{PREFIX}_filter_to_inlet_zone_divider"),
        10.0,
        238.0,
        30.0,
    )
    .translate(325.0, 235.0, BASE_Z / 2.0 + 15.0);
    let witness_vane = centered_cube(
        format!("{PREFIX}_witness_to_vane_zone_divider"),
        10.0,
        218.0,
        30.0,
    )
    .translate(-155.0, -40.0, BASE_Z / 2.0 + 15.0);
    let vane_probe = centered_cube(
        format!("{PREFIX}_vane_to_probe_zone_divider"),
        10.0,
        218.0,
        30.0,
    )
    .translate(325.0, -40.0, BASE_Z / 2.0 + 15.0);
    row_a + row_b + dock_filter + filter_inlet + witness_vane + vane_probe
}

fn cascade_state_gutters() -> Part {
    let mut gutters = Part::empty(format!("{PREFIX}_pressure_cascade_state_gutters"));
    for i in 0..MODULES {
        let x = centered_index(i, MODULES, 250.0) - 20.0;
        gutters = gutters
            + centered_cube(
                format!("{PREFIX}_cascade_gutter_clean_to_transfer_{i}"),
                7.0,
                148.0,
                7.0,
            )
            .translate(x, 112.0, BASE_Z / 2.0 + 3.5)
            + centered_cube(
                format!("{PREFIX}_cascade_gutter_transfer_to_return_{i}"),
                7.0,
                120.0,
                7.0,
            )
            .translate(x, -176.0, BASE_Z / 2.0 + 3.5);
    }
    gutters
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_datum_fiducials"));
    for (i, (x, y)) in [
        (-665.0, 390.0),
        (665.0, 390.0),
        (-665.0, -390.0),
        (665.0, -390.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial(format!("{PREFIX}_robot_datum_{i}")).translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    fiducials
}

fn fiducial(name: impl Into<String>) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_disc"), 12.0, 4.0, 36)
        + centered_cube(format!("{name}_bar_x"), 22.0, 3.0, 5.0)
        + centered_cube(format!("{name}_bar_y"), 3.0, 22.0, 5.0)
}

fn module_surrogate_dock() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_module_surrogate_dock_body"),
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    base - module_nest_recesses() - service_port_pockets()
        + module_key_rails()
        + clamp_bridge_tabs()
        + closed_box_shadow_lands()
}

fn module_nest_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_module_nest_recesses"));
    for i in 0..MODULES {
        let x = centered_index(i, MODULES, MODULE_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("{PREFIX}_module_surrogate_rect_recess_{i}"),
                MODULE_NEST_X,
                MODULE_NEST_Y,
                MODULE_NEST_Z,
            )
            .translate(x, -18.0, DOCK_Z / 2.0 - MODULE_NEST_Z / 2.0 + 0.3)
            + centered_cylinder(
                format!("{PREFIX}_module_alignment_pin_bore_a_{i}"),
                3.2,
                DOCK_Z + 4.0,
                20,
            )
            .translate(x - 28.0, 58.0, 0.0)
            + centered_cylinder(
                format!("{PREFIX}_module_alignment_pin_bore_b_{i}"),
                3.2,
                DOCK_Z + 4.0,
                20,
            )
            .translate(x + 28.0, 58.0, 0.0);
    }
    recesses
}

fn service_port_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_service_port_pockets"));
    for module in 0..MODULES {
        let x = centered_index(module, MODULES, MODULE_PITCH_X);
        for port in 0..SERVICE_PORTS_PER_MODULE {
            pockets = pockets
                + centered_cylinder(
                    format!("{PREFIX}_service_port_witness_socket_m{module}_p{port}"),
                    9.0,
                    22.0,
                    28,
                )
                .translate(
                    x - 28.0 + port as f64 * 28.0,
                    -92.0,
                    DOCK_Z / 2.0 - 8.0,
                );
        }
    }
    pockets
}

fn module_key_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_module_key_rails"));
    for i in 0..MODULES {
        let x = centered_index(i, MODULES, MODULE_PITCH_X);
        rails = rails
            + centered_cube(
                format!("{PREFIX}_module_left_key_rail_{i}"),
                7.0,
                MODULE_NEST_Y + 28.0,
                16.0,
            )
            .translate(x - MODULE_NEST_X / 2.0 - 12.0, -18.0, DOCK_Z / 2.0 + 8.0)
            + centered_cube(
                format!("{PREFIX}_module_right_key_rail_{i}"),
                7.0,
                MODULE_NEST_Y + 28.0,
                16.0,
            )
            .translate(x + MODULE_NEST_X / 2.0 + 12.0, -18.0, DOCK_Z / 2.0 + 8.0);
    }
    rails
}

fn clamp_bridge_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_module_clamp_bridge_tabs"));
    for i in 0..MODULES {
        let x = centered_index(i, MODULES, MODULE_PITCH_X);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_module_return_lock_tab_{i}"),
                58.0,
                12.0,
                20.0,
            )
            .translate(x, 72.0, DOCK_Z / 2.0 + 10.0)
            + centered_cube(
                format!("{PREFIX}_module_transfer_lock_tab_{i}"),
                58.0,
                12.0,
                20.0,
            )
            .translate(x, -110.0, DOCK_Z / 2.0 + 10.0);
    }
    tabs
}

fn closed_box_shadow_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_closed_culture_box_shadow_lands"));
    for i in 0..MODULES {
        let x = centered_index(i, MODULES, MODULE_PITCH_X);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_culture_box_outline_land_{i}"),
                96.0,
                4.0,
                5.0,
            )
            .translate(x, 14.0, DOCK_Z / 2.0 + 2.5);
    }
    lands
}

fn sterile_filter_holder() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_sterile_filter_holder_body"),
        FILTER_X,
        FILTER_Y,
        FILTER_Z,
    );
    base - filter_cradles() - vent_performance_windows()
        + filter_retainer_rings()
        + upstream_downstream_label_bars()
        + vent_envelope_height_flags()
}

fn filter_cradles() -> Part {
    let mut cradles = Part::empty(format!("{PREFIX}_filter_cradles"));
    for i in 0..FILTER_COUNT {
        let x = centered_index(i, FILTER_COUNT, FILTER_PITCH_X);
        cradles = cradles
            + centered_cylinder(
                format!("{PREFIX}_filter_round_cradle_{i}"),
                FILTER_D / 2.0,
                92.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, FILTER_Z / 2.0 - 28.0)
            + centered_cylinder(
                format!("{PREFIX}_filter_luer_bore_upstream_{i}"),
                4.0,
                70.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 62.0, FILTER_Z / 2.0 - 28.0)
            + centered_cylinder(
                format!("{PREFIX}_filter_luer_bore_downstream_{i}"),
                4.0,
                70.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -62.0, FILTER_Z / 2.0 - 28.0);
    }
    cradles
}

fn vent_performance_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_vent_performance_windows"));
    for i in 0..FILTER_COUNT {
        let x = centered_index(i, FILTER_COUNT, FILTER_PITCH_X);
        windows = windows
            + centered_cube(
                format!("{PREFIX}_filter_delta_p_window_{i}"),
                54.0,
                12.0,
                14.0,
            )
            .translate(x, 92.0, FILTER_Z / 2.0 - 7.0)
            + centered_cube(
                format!("{PREFIX}_filter_particle_challenge_window_{i}"),
                54.0,
                12.0,
                14.0,
            )
            .translate(x, -92.0, FILTER_Z / 2.0 - 7.0);
    }
    windows
}

fn filter_retainer_rings() -> Part {
    let mut rings = Part::empty(format!("{PREFIX}_filter_retainer_rings"));
    for i in 0..FILTER_COUNT {
        let x = centered_index(i, FILTER_COUNT, FILTER_PITCH_X);
        rings = rings
            + centered_cylinder(format!("{PREFIX}_filter_retainer_left_{i}"), 28.0, 9.0, 44)
                .rotate(90.0, 0.0, 0.0)
                .translate(x, -52.0, FILTER_Z / 2.0 + 4.5)
            + centered_cylinder(format!("{PREFIX}_filter_retainer_right_{i}"), 28.0, 9.0, 44)
                .rotate(90.0, 0.0, 0.0)
                .translate(x, 52.0, FILTER_Z / 2.0 + 4.5);
    }
    rings
}

fn upstream_downstream_label_bars() -> Part {
    centered_cube(
        format!("{PREFIX}_filter_upstream_label_bar"),
        FILTER_X - 50.0,
        6.0,
        6.0,
    )
    .translate(0.0, 108.0, FILTER_Z / 2.0 + 3.0)
        + centered_cube(
            format!("{PREFIX}_filter_downstream_label_bar"),
            FILTER_X - 50.0,
            6.0,
            6.0,
        )
        .translate(0.0, -108.0, FILTER_Z / 2.0 + 3.0)
}

fn vent_envelope_height_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_vent_envelope_height_flags"));
    for i in 0..3 {
        flags = flags
            + centered_cube(
                format!("{PREFIX}_vent_envelope_flag_{i}"),
                14.0,
                42.0,
                18.0 + i as f64 * 12.0,
            )
            .translate(
                -FILTER_X / 2.0 + 34.0 + i as f64 * 24.0,
                0.0,
                FILTER_Z / 2.0 + (18.0 + i as f64 * 12.0) / 2.0,
            );
    }
    flags
}

fn purge_inlet_manifold() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_purge_inlet_manifold_body"),
        INLET_X,
        INLET_Y,
        INLET_Z,
    );
    base - inlet_branch_bores()
        + branch_hose_bosses()
        + flow_direction_arrows()
        + pressure_cascade_state_tokens()
}

fn inlet_branch_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_purge_inlet_branch_bores"));
    for i in 0..INLET_BRANCHES {
        let x = centered_index(i, INLET_BRANCHES, 66.0);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_inlet_branch_bore_{i}"),
                BRANCH_BORE_D / 2.0,
                INLET_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, INLET_Z / 2.0 - 22.0);
    }
    bores
        + centered_cylinder(
            format!("{PREFIX}_main_purge_supply_header_bore"),
            9.0,
            INLET_X + 20.0,
            30,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 86.0, INLET_Z / 2.0 - 22.0)
}

fn branch_hose_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PREFIX}_purge_inlet_hose_bosses"));
    for i in 0..INLET_BRANCHES {
        let x = centered_index(i, INLET_BRANCHES, 66.0);
        bosses = bosses
            + centered_cylinder(format!("{PREFIX}_inlet_branch_boss_{i}"), 15.0, 18.0, 32)
                .rotate(90.0, 0.0, 0.0)
                .translate(x, 112.0, INLET_Z / 2.0 - 22.0);
    }
    bosses
}

fn flow_direction_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_purge_flow_direction_arrows"));
    for i in 0..INLET_BRANCHES {
        let x = centered_index(i, INLET_BRANCHES, 66.0);
        arrows =
            arrows
                + centered_cube(format!("{PREFIX}_flow_arrow_stem_{i}"), 7.0, 46.0, 6.0).translate(
                    x,
                    20.0,
                    INLET_Z / 2.0 + 3.0,
                )
                + centered_cube(format!("{PREFIX}_flow_arrow_head_{i}"), 22.0, 16.0, 7.0)
                    .translate(x, -8.0, INLET_Z / 2.0 + 3.5);
    }
    arrows
}

fn pressure_cascade_state_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_pressure_cascade_state_tokens"));
    for state in 0..PURGE_STATES {
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_cascade_token_state_{state}"),
                48.0,
                18.0,
                8.0,
            )
            .translate(-126.0 + state as f64 * 84.0, -104.0, INLET_Z / 2.0 + 4.0);
    }
    tokens
}

fn exhaust_return_witness_channel() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_exhaust_return_witness_channel_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    base - witness_channel_recesses() - return_sight_windows()
        + exhaust_bubble_witness_ribs()
        + return_path_sample_cups()
}

fn witness_channel_recesses() -> Part {
    let mut channels = Part::empty(format!("{PREFIX}_exhaust_witness_channel_recesses"));
    for i in 0..WITNESS_CHANNELS {
        let y = centered_index(i, WITNESS_CHANNELS, 42.0);
        channels = channels
            + centered_cube(
                format!("{PREFIX}_exhaust_return_serpentine_long_{i}"),
                WITNESS_X - 72.0,
                12.0,
                18.0,
            )
            .translate(0.0, y, WITNESS_Z / 2.0 - 9.0)
            + centered_cylinder(
                format!("{PREFIX}_exhaust_return_turnaround_left_{i}"),
                13.0,
                18.0,
                28,
            )
            .translate(-WITNESS_X / 2.0 + 52.0, y, WITNESS_Z / 2.0 - 9.0)
            + centered_cylinder(
                format!("{PREFIX}_exhaust_return_turnaround_right_{i}"),
                13.0,
                18.0,
                28,
            )
            .translate(WITNESS_X / 2.0 - 52.0, y, WITNESS_Z / 2.0 - 9.0);
    }
    channels
}

fn return_sight_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_return_sight_windows"));
    for i in 0..WITNESS_CHANNELS {
        windows = windows
            + centered_cube(
                format!("{PREFIX}_return_witness_sight_window_{i}"),
                72.0,
                16.0,
                12.0,
            )
            .translate(
                centered_index(i, WITNESS_CHANNELS, 96.0),
                84.0,
                WITNESS_Z / 2.0 - 6.0,
            );
    }
    windows
}

fn exhaust_bubble_witness_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_exhaust_bubble_witness_ribs"));
    for i in 0..10 {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_exhaust_witness_tick_{i}"),
                4.0,
                180.0,
                8.0,
            )
            .translate(-210.0 + i as f64 * 46.0, 0.0, WITNESS_Z / 2.0 + 4.0);
    }
    ribs
}

fn return_path_sample_cups() -> Part {
    let mut cups = Part::empty(format!("{PREFIX}_return_path_sample_cups"));
    for i in 0..WITNESS_CHANNELS {
        cups = cups
            + centered_cylinder(format!("{PREFIX}_return_path_cup_ring_{i}"), 18.0, 8.0, 32)
                .translate(
                    centered_index(i, WITNESS_CHANNELS, 96.0),
                    -86.0,
                    WITNESS_Z / 2.0 + 4.0,
                );
    }
    cups
}

fn flow_vane_grid() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_flow_vane_grid_frame"),
        VANE_X,
        VANE_Y,
        VANE_Z,
    );
    base - vane_window_cutouts() + directional_vanes() + grid_reference_ticks()
}

fn vane_window_cutouts() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_flow_vane_window_cutouts"));
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let idx = row * VANE_COLS + col;
            windows = windows
                + centered_cube(format!("{PREFIX}_vane_cell_window_{idx}"), 42.0, 38.0, 22.0)
                    .translate(
                        centered_index(col, VANE_COLS, 52.0),
                        centered_index(row, VANE_ROWS, 58.0),
                        VANE_Z / 2.0 - 11.0,
                    );
        }
    }
    windows
}

fn directional_vanes() -> Part {
    let mut vanes = Part::empty(format!("{PREFIX}_directional_vanes"));
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let idx = row * VANE_COLS + col;
            let angle = if (row + col) % 2 == 0 { 12.0 } else { -12.0 };
            vanes = vanes
                + centered_cube(format!("{PREFIX}_flow_vane_blade_{idx}"), 6.0, 36.0, 34.0)
                    .rotate(0.0, 0.0, angle)
                    .translate(
                        centered_index(col, VANE_COLS, 52.0),
                        centered_index(row, VANE_ROWS, 58.0),
                        VANE_Z / 2.0 + 17.0,
                    );
        }
    }
    vanes
}

fn grid_reference_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_flow_vane_reference_ticks"));
    for i in 0..VANE_COLS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_vane_column_reference_tick_{i}"),
                32.0,
                5.0,
                6.0,
            )
            .translate(
                centered_index(i, VANE_COLS, 52.0),
                -VANE_Y / 2.0 + 20.0,
                VANE_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn pressure_rh_particle_probe_pockets() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_pressure_rh_particle_probe_pocket_body"),
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    );
    base - pressure_probe_pockets() - rh_probe_pockets() - particle_probe_pockets()
        + probe_cable_strain_relief_combs()
        + pressure_cascade_status_ladder()
}

fn pressure_probe_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_pressure_probe_pockets"));
    for i in 0..PRESSURE_PROBES {
        pockets = pockets
            + centered_cylinder(
                format!("{PREFIX}_pressure_probe_socket_{i}"),
                10.0,
                24.0,
                28,
            )
            .translate(
                centered_index(i, PRESSURE_PROBES, 54.0),
                66.0,
                PROBE_Z / 2.0 - 12.0,
            );
    }
    pockets
}

fn rh_probe_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_rh_probe_pockets"));
    for i in 0..RH_PROBES {
        pockets = pockets
            + centered_cube(
                format!("{PREFIX}_rh_probe_rect_socket_{i}"),
                34.0,
                24.0,
                16.0,
            )
            .translate(centered_index(i, RH_PROBES, 70.0), 0.0, PROBE_Z / 2.0 - 8.0);
    }
    pockets
}

fn particle_probe_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_particle_probe_pockets"));
    for i in 0..PARTICLE_PROBES {
        pockets = pockets
            + centered_cylinder(
                format!("{PREFIX}_particle_probe_tube_socket_{i}"),
                12.0,
                24.0,
                30,
            )
            .translate(
                centered_index(i, PARTICLE_PROBES, 74.0),
                -66.0,
                PROBE_Z / 2.0 - 12.0,
            );
    }
    pockets
}

fn probe_cable_strain_relief_combs() -> Part {
    let mut combs = Part::empty(format!("{PREFIX}_probe_cable_strain_relief_combs"));
    for i in 0..9 {
        combs = combs
            + centered_cube(
                format!("{PREFIX}_probe_cable_comb_tooth_{i}"),
                6.0,
                38.0,
                14.0,
            )
            .translate(-150.0 + i as f64 * 37.5, -104.0, PROBE_Z / 2.0 + 7.0);
    }
    combs
}

fn pressure_cascade_status_ladder() -> Part {
    let mut ladder = Part::empty(format!("{PREFIX}_pressure_cascade_status_ladder"));
    for i in 0..PURGE_STATES {
        ladder = ladder
            + centered_cube(
                format!("{PREFIX}_cascade_status_rung_{i}"),
                70.0 + i as f64 * 24.0,
                6.0,
                7.0,
            )
            .translate(0.0, 100.0 - i as f64 * 18.0, PROBE_Z / 2.0 + 3.5);
    }
    ladder
}

fn timer_token_rail() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_timer_token_rail_body"),
        TIMER_X,
        TIMER_Y,
        TIMER_Z,
    );
    base - token_recesses() + timer_graduation_ticks() + elapsed_time_hold_tabs()
}

fn token_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_timer_token_recesses"));
    for i in 0..TIMER_TOKENS {
        let col = i % 8;
        let row = i / 8;
        recesses = recesses
            + centered_cube(format!("{PREFIX}_timer_token_recess_{i}"), 25.0, 22.0, 12.0)
                .translate(
                    centered_index(col, 8, 32.0),
                    -28.0 + row as f64 * 42.0,
                    TIMER_Z / 2.0 - 6.0,
                );
    }
    recesses
}

fn timer_graduation_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_timer_graduation_ticks"));
    for i in 0..PURGE_STATES {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_purge_duration_graduation_{i}"),
                5.0,
                TIMER_Y - 24.0,
                7.0,
            )
            .translate(-108.0 + i as f64 * 72.0, 0.0, TIMER_Z / 2.0 + 3.5);
    }
    ticks
}

fn elapsed_time_hold_tabs() -> Part {
    centered_cube(
        format!("{PREFIX}_elapsed_timer_token_hold_tab_left"),
        18.0,
        TIMER_Y - 28.0,
        20.0,
    )
    .translate(-TIMER_X / 2.0 + 20.0, 0.0, TIMER_Z / 2.0 + 10.0)
        + centered_cube(
            format!("{PREFIX}_elapsed_timer_token_hold_tab_right"),
            18.0,
            TIMER_Y - 28.0,
            20.0,
        )
        .translate(TIMER_X / 2.0 - 20.0, 0.0, TIMER_Z / 2.0 + 10.0)
}

fn stagnant_pocket_tracer_coupons() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_stagnant_pocket_coupon_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    base - stagnant_coupon_recesses() + pocket_geometry_shadow_blocks() + tracer_pull_tab_rack()
}

fn stagnant_coupon_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_stagnant_coupon_recesses"));
    for i in 0..TRACER_COUPONS {
        let col = i % 6;
        let row = i / 6;
        recesses = recesses
            + centered_cube(
                format!("{PREFIX}_stagnant_pocket_coupon_recess_{i}"),
                38.0,
                30.0,
                14.0,
            )
            .translate(
                centered_index(col, 6, 48.0),
                -28.0 + row as f64 * 58.0,
                COUPON_Z / 2.0 - 7.0,
            );
    }
    recesses
}

fn pocket_geometry_shadow_blocks() -> Part {
    let mut blocks = Part::empty(format!("{PREFIX}_stagnant_pocket_shadow_geometry_blocks"));
    for i in 0..TRACER_COUPONS {
        let col = i % 6;
        let row = i / 6;
        let h = 8.0 + (i % 3) as f64 * 5.0;
        blocks = blocks
            + centered_cube(
                format!("{PREFIX}_stagnant_pocket_shadow_step_{i}"),
                18.0,
                9.0,
                h,
            )
            .translate(
                centered_index(col, 6, 48.0),
                -51.0 + row as f64 * 58.0,
                COUPON_Z / 2.0 + h / 2.0,
            );
    }
    blocks
}

fn tracer_pull_tab_rack() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_tracer_pull_tab_rack"));
    for i in 0..TRACER_COUPONS {
        let col = i % 6;
        let row = i / 6;
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_tracer_coupon_pull_tab_{i}"),
                24.0,
                6.0,
                8.0,
            )
            .translate(
                centered_index(col, 6, 48.0),
                -8.0 + row as f64 * 58.0,
                COUPON_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn barcode_coa_custody_lands() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_barcode_coa_custody_land_plate"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    base + barcode_lands() + coa_custody_lands() + custody_chain_hash_ticks()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_module_barcode_land_{i}"), 58.0, 26.0, 4.0)
                .translate(
                    centered_index(i, BARCODE_LANDS, 70.0),
                    34.0,
                    CUSTODY_Z / 2.0 + 2.0,
                );
    }
    lands
}

fn coa_custody_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_coa_custody_lands"));
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_coa_custody_land_{i}"), 82.0, 28.0, 4.0).translate(
                centered_index(i, COA_LANDS, 96.0),
                -24.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_chain_hash_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_custody_chain_hash_ticks"));
    for i in 0..14 {
        ticks =
            ticks
                + centered_cube(format!("{PREFIX}_custody_hash_tick_{i}"), 5.0, 18.0, 5.0)
                    .translate(-148.0 + i as f64 * 22.0, -62.0, CUSTODY_Z / 2.0 + 2.5);
    }
    ticks
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    base - disposition_gate_slots() + gate_state_fences() + disposition_token_stops()
}

fn disposition_gate_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_release_hold_reject_slots"));
    for state in 0..DISPOSITION_STATES {
        for module in 0..GATE_SLOTS_PER_STATE {
            slots = slots
                + centered_cube(
                    format!("{PREFIX}_disposition_slot_s{state}_m{module}"),
                    32.0,
                    19.0,
                    14.0,
                )
                .translate(
                    centered_index(module, GATE_SLOTS_PER_STATE, 46.0),
                    42.0 - state as f64 * 42.0,
                    GATE_Z / 2.0 - 7.0,
                );
        }
    }
    slots
}

fn gate_state_fences() -> Part {
    let mut fences = Part::empty(format!("{PREFIX}_gate_state_fences"));
    for state in 0..DISPOSITION_STATES {
        fences = fences
            + centered_cube(
                format!("{PREFIX}_gate_state_fence_{state}"),
                GATE_X - 42.0,
                5.0,
                18.0,
            )
            .translate(0.0, 63.0 - state as f64 * 42.0, GATE_Z / 2.0 + 9.0);
    }
    fences
}

fn disposition_token_stops() -> Part {
    let release = centered_cube(
        format!("{PREFIX}_release_gate_green_geometry_token"),
        36.0,
        24.0,
        12.0,
    )
    .translate(-GATE_X / 2.0 + 32.0, 42.0, GATE_Z / 2.0 + 6.0);
    let hold = centered_cube(
        format!("{PREFIX}_hold_gate_triangle_geometry_token"),
        30.0,
        30.0,
        12.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-GATE_X / 2.0 + 32.0, 0.0, GATE_Z / 2.0 + 6.0);
    let reject = centered_cube(
        format!("{PREFIX}_reject_gate_red_geometry_token"),
        38.0,
        8.0,
        12.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-GATE_X / 2.0 + 32.0, -42.0, GATE_Z / 2.0 + 6.0)
        + centered_cube(
            format!("{PREFIX}_reject_gate_cross_geometry_token"),
            38.0,
            8.0,
            12.0,
        )
        .rotate(0.0, 0.0, -45.0)
        .translate(-GATE_X / 2.0 + 32.0, -42.0, GATE_Z / 2.0 + 6.0);
    release + hold + reject
}

fn evidence_camera_robot_keepout_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_camera_bridge_left_post"),
        34.0,
        60.0,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0 + 68.0, 0.0, 0.0);
    let right_post = centered_cube(
        format!("{PREFIX}_camera_bridge_right_post"),
        34.0,
        60.0,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 68.0, 0.0, 0.0);
    let beam = centered_cube(
        format!("{PREFIX}_evidence_camera_bridge_beam"),
        BRIDGE_X - 110.0,
        32.0,
        28.0,
    )
    .translate(0.0, 0.0, BRIDGE_Z / 2.0 - 14.0);
    let bridge =
        left_post + right_post + beam + camera_window_markers() + robot_service_keepout_gauges();
    bridge - camera_mount_bores()
}

fn camera_window_markers() -> Part {
    let mut markers = Part::empty(format!("{PREFIX}_evidence_camera_window_markers"));
    for i in 0..CAMERA_WINDOWS {
        markers = markers
            + centered_cube(
                format!("{PREFIX}_camera_evidence_window_marker_{i}"),
                88.0,
                8.0,
                7.0,
            )
            .translate(
                centered_index(i, CAMERA_WINDOWS, 230.0),
                0.0,
                BRIDGE_Z / 2.0 + 3.5,
            );
    }
    markers
}

fn robot_service_keepout_gauges() -> Part {
    let mut gauges = Part::empty(format!("{PREFIX}_robot_service_keepout_gauges"));
    for i in 0..ROBOT_KEEP_OUT_GAUGES {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_robot_service_keepout_gauge_{i}"),
                20.0,
                90.0,
                8.0,
            )
            .translate(
                centered_index(i, ROBOT_KEEP_OUT_GAUGES, 190.0),
                52.0,
                -BRIDGE_Z / 2.0 + 4.0,
            );
    }
    gauges
}

fn camera_mount_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_camera_mount_bores"));
    for i in 0..CAMERA_WINDOWS {
        bores = bores
            + centered_cylinder(format!("{PREFIX}_camera_mount_bore_{i}"), 4.0, 40.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    centered_index(i, CAMERA_WINDOWS, 230.0),
                    0.0,
                    BRIDGE_Z / 2.0 - 14.0,
                );
    }
    bores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_matches_feature_count() {
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_regions_fit_without_collisions() {
        assert_design_constraints();
    }

    #[test]
    fn purge_feature_counts_track_module_count() {
        assert_eq!(FILTER_COUNT, MODULES);
        assert_eq!(WITNESS_CHANNELS, MODULES);
        assert_eq!(INLET_BRANCHES, MODULES);
        assert_eq!(PRESSURE_PROBES, MODULES + 1);
        assert_eq!(TIMER_TOKENS, MODULES * PURGE_STATES);
        assert_eq!(GATE_SLOTS_PER_STATE, MODULES);
    }

    #[test]
    fn centered_index_is_symmetric() {
        assert_eq!(centered_index(0, 4, 10.0), -15.0);
        assert_eq!(centered_index(3, 4, 10.0), 15.0);
        assert_eq!(centered_index(1, 3, 12.0), 0.0);
    }

    #[test]
    fn limitations_are_explicitly_non_sop() {
        assert!(LIMITATIONS.contains(&"not_a_biosafety_claim"));
        assert!(LIMITATIONS.contains(&"not_a_purge_sop"));
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
    }
}
