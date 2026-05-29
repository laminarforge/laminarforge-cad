use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Contained robot collision and force-limit validation station.
//
// Intent:
// - Provide repeatable force-gauge, cassette-surrogate, staged collision, and
//   stop-coupon targets for robot collision/force-limit verification in a
//   contained workcell fixture.
// - Keep calibration masses, barcode/certificate traceability, disposition
//   lanes, evidence cameras, and robot/service keepout gauges physically tied
//   to the validation station instead of an improvised bench setup.
// - Export modular STL groups so a parent integration pass can include only
//   the needed station components without changing shared manifests.
//
// This is fixture packaging geometry only. It does not define robot safety
// limits, acceptance criteria, or a validated method.

const OUTPUTS: [&str; 11] = [
    "output/closed_robot_collision_force_limit_validation_station_base_tray.stl",
    "output/closed_robot_collision_force_limit_validation_station_force_gauge_targets.stl",
    "output/closed_robot_collision_force_limit_validation_station_compliant_cassette_surrogate.stl",
    "output/closed_robot_collision_force_limit_validation_station_staged_collision_posts.stl",
    "output/closed_robot_collision_force_limit_validation_station_soft_hard_stop_coupons.stl",
    "output/closed_robot_collision_force_limit_validation_station_calibration_mass_pockets.stl",
    "output/closed_robot_collision_force_limit_validation_station_barcode_certificate_lands.stl",
    "output/closed_robot_collision_force_limit_validation_station_release_hold_reject_lanes.stl",
    "output/closed_robot_collision_force_limit_validation_station_evidence_camera_bridge.stl",
    "output/closed_robot_collision_force_limit_validation_station_robot_service_keepout_gauges.stl",
    "output/closed_robot_collision_force_limit_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 9] = [
    "force_gauge_target_blocks",
    "compliant_cassette_surrogate",
    "staged_collision_posts",
    "soft_hard_stop_coupons",
    "calibration_mass_pockets",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const DECK_X: f64 = 820.0;
const DECK_Y: f64 = 560.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 4.0;
const MODULE_CLEARANCE: f64 = 12.0;
const DATUM_PIN_D: f64 = 8.0;

const FORCE_PANEL_X: f64 = 248.0;
const FORCE_PANEL_Y: f64 = 136.0;
const FORCE_PANEL_Z: f64 = 22.0;
const FORCE_CENTER: (f64, f64) = (-235.0, 132.0);

const COLLISION_PANEL_X: f64 = 230.0;
const COLLISION_PANEL_Y: f64 = 136.0;
const COLLISION_PANEL_Z: f64 = 18.0;
const COLLISION_CENTER: (f64, f64) = (40.0, 132.0);

const STOP_PANEL_X: f64 = 170.0;
const STOP_PANEL_Y: f64 = 136.0;
const STOP_PANEL_Z: f64 = 20.0;
const STOP_CENTER: (f64, f64) = (285.0, 132.0);

const CASSETTE_PANEL_X: f64 = 250.0;
const CASSETTE_PANEL_Y: f64 = 140.0;
const CASSETTE_PANEL_Z: f64 = 18.0;
const CASSETTE_CENTER: (f64, f64) = (-235.0, -58.0);
const SURROGATE_X: f64 = 198.0;
const SURROGATE_Y: f64 = 92.0;
const SURROGATE_Z: f64 = 24.0;

const MASS_PANEL_X: f64 = 235.0;
const MASS_PANEL_Y: f64 = 150.0;
const MASS_PANEL_Z: f64 = 24.0;
const MASS_CENTER: (f64, f64) = (35.0, -60.0);

const LANE_PANEL_X: f64 = 180.0;
const LANE_PANEL_Y: f64 = 160.0;
const LANE_PANEL_Z: f64 = 20.0;
const LANES_CENTER: (f64, f64) = (287.0, -62.0);

const LABEL_PANEL_X: f64 = 340.0;
const LABEL_PANEL_Y: f64 = 44.0;
const LABEL_PANEL_Z: f64 = 6.0;
const LABEL_CENTER: (f64, f64) = (-167.0, -224.0);

const BRIDGE_POST_SPAN_X: f64 = 690.0;
const BRIDGE_POST_SPAN_Y: f64 = 430.0;
const BRIDGE_UNDERSIDE_Z: f64 = 168.0;
const BRIDGE_BEAM_Z: f64 = 28.0;

#[derive(Copy, Clone)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let force_targets = force_gauge_target_blocks();
    export(OUTPUTS[1], &force_targets);

    let cassette = compliant_cassette_surrogate();
    export(OUTPUTS[2], &cassette);

    let collision_posts = staged_collision_posts();
    export(OUTPUTS[3], &collision_posts);

    let stop_coupons = soft_hard_stop_coupons();
    export(OUTPUTS[4], &stop_coupons);

    let mass_pockets = calibration_mass_pockets();
    export(OUTPUTS[5], &mass_pockets);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let camera_bridge = evidence_camera_bridge();
    export(OUTPUTS[8], &camera_bridge);

    let keepout = robot_service_keepout_gauges();
    export(OUTPUTS[9], &keepout);

    let assembly = base
        + force_targets.translate(FORCE_CENTER.0, FORCE_CENTER.1, deck_insert_z(FORCE_PANEL_Z))
        + cassette.translate(
            CASSETTE_CENTER.0,
            CASSETTE_CENTER.1,
            deck_insert_z(CASSETTE_PANEL_Z),
        )
        + collision_posts.translate(
            COLLISION_CENTER.0,
            COLLISION_CENTER.1,
            deck_insert_z(COLLISION_PANEL_Z),
        )
        + stop_coupons.translate(STOP_CENTER.0, STOP_CENTER.1, deck_insert_z(STOP_PANEL_Z))
        + mass_pockets.translate(MASS_CENTER.0, MASS_CENTER.1, deck_insert_z(MASS_PANEL_Z))
        + traceability.translate(LABEL_CENTER.0, LABEL_CENTER.1, deck_insert_z(LABEL_PANEL_Z))
        + lanes.translate(LANES_CENTER.0, LANES_CENTER.1, deck_insert_z(LANE_PANEL_Z))
        + camera_bridge.translate(0.0, 0.0, DECK_Z / 2.0)
        + keepout.translate(0.0, 0.0, DECK_Z / 2.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed robot collision/force-limit validation station:");
    println!("  Contained deck:                {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Force targets:                 3 gauge blocks with witness collars and overtravel references"
    );
    println!(
        "  Collision staging:             4 posts, soft/hard coupon panel, compliant cassette surrogate"
    );
    println!(
        "  Calibration and evidence:      mass pockets, barcode/certificate lands, camera bridge"
    );
    println!(
        "  Disposition controls:          release, hold, and reject lanes with physical gates"
    );
    println!(
        "  Keepout coverage:              robot sweep frame plus rear service and side access gauges"
    );
    println!(
        "  Feature groups covered:        {}",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    let modules = insert_footprints();
    for module in modules {
        assert!(
            fits_on_deck(module),
            "{} exceeds contained validation deck envelope",
            module.name
        );
    }

    for (i, left) in modules.iter().enumerate() {
        for right in modules.iter().skip(i + 1) {
            assert!(
                footprints_clear(*left, *right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }

    assert!(
        BRIDGE_POST_SPAN_X < DECK_X - 2.0 * RIM_W && BRIDGE_POST_SPAN_Y < DECK_Y - 2.0 * RIM_W,
        "camera bridge posts must land inside the rim"
    );
    assert!(
        BRIDGE_UNDERSIDE_Z > 2.0 * RIM_Z,
        "camera bridge underside must clear contained rim and validation loads"
    );
}

fn insert_footprints() -> [Footprint; 7] {
    [
        Footprint {
            name: "force gauge target blocks",
            center: FORCE_CENTER,
            x: FORCE_PANEL_X,
            y: FORCE_PANEL_Y,
        },
        Footprint {
            name: "staged collision posts",
            center: COLLISION_CENTER,
            x: COLLISION_PANEL_X,
            y: COLLISION_PANEL_Y,
        },
        Footprint {
            name: "soft and hard stop coupons",
            center: STOP_CENTER,
            x: STOP_PANEL_X,
            y: STOP_PANEL_Y,
        },
        Footprint {
            name: "compliant cassette surrogate",
            center: CASSETTE_CENTER,
            x: CASSETTE_PANEL_X,
            y: CASSETTE_PANEL_Y,
        },
        Footprint {
            name: "calibration mass pockets",
            center: MASS_CENTER,
            x: MASS_PANEL_X,
            y: MASS_PANEL_Y,
        },
        Footprint {
            name: "release hold reject lanes",
            center: LANES_CENTER,
            x: LANE_PANEL_X,
            y: LANE_PANEL_Y,
        },
        Footprint {
            name: "barcode certificate lands",
            center: LABEL_CENTER,
            x: LABEL_PANEL_X,
            y: LABEL_PANEL_Y,
        },
    ]
}

fn fits_on_deck(module: Footprint) -> bool {
    let usable_x = DECK_X / 2.0 - RIM_W - MODULE_CLEARANCE / 2.0;
    let usable_y = DECK_Y / 2.0 - RIM_W - MODULE_CLEARANCE / 2.0;
    module.center.0.abs() + module.x / 2.0 <= usable_x
        && module.center.1.abs() + module.y / 2.0 <= usable_y
}

fn footprints_clear(a: Footprint, b: Footprint) -> bool {
    let x_clear = (a.center.0 - b.center.0).abs() >= a.x / 2.0 + b.x / 2.0 + MODULE_CLEARANCE;
    let y_clear = (a.center.1 - b.center.1).abs() >= a.y / 2.0 + b.y / 2.0 + MODULE_CLEARANCE;
    x_clear || y_clear
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "robot_collision_validation_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let shallow_sump = centered_cube(
        "robot_collision_validation_shallow_witness_sump",
        DECK_X - 122.0,
        DECK_Y - 104.0,
        5.0,
    )
    .translate(0.0, -10.0, DECK_Z - 2.5);
    let drain_slot = centered_cube(
        "robot_collision_validation_front_drain_slot",
        96.0,
        14.0,
        8.0,
    )
    .translate(250.0, -DECK_Y / 2.0 + 28.0, DECK_Z - 4.0);

    deck - shallow_sump - drain_slot - insert_sockets() - base_datum_holes()
        + containment_rims()
        + insert_datum_bosses()
        + deck_handling_features()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("robot_collision_validation_insert_sockets");
    for module in insert_footprints() {
        sockets = sockets
            + centered_cube(
                format!("robot_collision_validation_{}_socket", slug(module.name)),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.3,
            )
            .translate(
                module.center.0,
                module.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn base_datum_holes() -> Part {
    let mut holes = Part::empty("robot_collision_validation_base_datum_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
        (0.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("robot_collision_validation_base_datum_hole_{i}"),
                DATUM_PIN_D / 2.0,
                DECK_Z + 5.0,
                32,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "robot_collision_validation_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "robot_collision_validation_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "robot_collision_validation_left_containment_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "robot_collision_validation_right_containment_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn insert_datum_bosses() -> Part {
    let mut bosses = Part::empty("robot_collision_validation_insert_datum_bosses");
    for module in insert_footprints() {
        for (i, (dx, dy)) in [
            (-module.x / 2.0 + 18.0, -module.y / 2.0 + 18.0),
            (module.x / 2.0 - 18.0, module.y / 2.0 - 18.0),
        ]
        .iter()
        .enumerate()
        {
            let boss = centered_cylinder(
                format!(
                    "robot_collision_validation_{}_datum_boss_{i}",
                    slug(module.name)
                ),
                9.0,
                7.0,
                28,
            )
            .translate(module.center.0 + dx, module.center.1 + dy, DECK_Z + 3.5);
            let center = centered_cylinder(
                format!(
                    "robot_collision_validation_{}_datum_pin_mark_{i}",
                    slug(module.name)
                ),
                2.7,
                9.0,
                24,
            )
            .translate(module.center.0 + dx, module.center.1 + dy, DECK_Z + 3.5);
            bosses = bosses + (boss - center);
        }
    }
    bosses
}

fn deck_handling_features() -> Part {
    let left_handle = centered_cube(
        "robot_collision_validation_left_recessed_handhold",
        90.0,
        12.0,
        13.0,
    )
    .translate(-DECK_X / 2.0 + 110.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 5.0);
    let right_handle = centered_cube(
        "robot_collision_validation_right_recessed_handhold",
        90.0,
        12.0,
        13.0,
    )
    .translate(DECK_X / 2.0 - 110.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 5.0);
    let rear_bar = centered_cube(
        "robot_collision_validation_rear_robot_pick_bar",
        210.0,
        16.0,
        16.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 42.0, DECK_Z + 8.0);

    left_handle + right_handle + rear_bar
}

fn force_gauge_target_blocks() -> Part {
    let panel = module_panel(
        "robot_collision_validation_force_target_panel",
        FORCE_PANEL_X,
        FORCE_PANEL_Y,
        FORCE_PANEL_Z,
    );
    let cable_trough = centered_cube(
        "robot_collision_validation_force_gauge_cable_trough",
        FORCE_PANEL_X - 38.0,
        12.0,
        7.0,
    )
    .translate(0.0, FORCE_PANEL_Y / 2.0 - 22.0, FORCE_PANEL_Z / 2.0 - 3.5);

    let mut targets = Part::empty("robot_collision_validation_force_gauge_target_blocks");
    for (i, x) in [-76.0, 0.0, 76.0].iter().enumerate() {
        let block = centered_cube(
            format!("robot_collision_validation_force_gauge_target_block_{i}"),
            54.0,
            36.0,
            68.0,
        )
        .translate(*x, -6.0, FORCE_PANEL_Z / 2.0 + 34.0);
        let load_cell_land = centered_cube(
            format!("robot_collision_validation_force_gauge_load_cell_land_{i}"),
            48.0,
            16.0,
            46.0,
        )
        .translate(*x, 20.0, FORCE_PANEL_Z / 2.0 + 40.0);
        let impact_face = centered_cylinder(
            format!("robot_collision_validation_force_gauge_round_impact_face_{i}"),
            18.0,
            8.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -28.0, FORCE_PANEL_Z / 2.0 + 42.0);
        let witness_ring = centered_cylinder(
            format!("robot_collision_validation_force_gauge_witness_ring_{i}"),
            24.0,
            3.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -33.0, FORCE_PANEL_Z / 2.0 + 42.0);
        let overtravel_pin = centered_cylinder(
            format!("robot_collision_validation_force_gauge_overtravel_pin_{i}"),
            5.0,
            46.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 42.0, FORCE_PANEL_Z / 2.0 + 42.0);
        let rear_reference = centered_cube(
            format!("robot_collision_validation_force_gauge_rear_reference_stop_{i}"),
            44.0,
            9.0,
            34.0,
        )
        .translate(*x, FORCE_PANEL_Y / 2.0 - 18.0, FORCE_PANEL_Z / 2.0 + 30.0);

        targets = targets + block + load_cell_land + witness_ring + overtravel_pin + rear_reference
            - impact_face;
    }

    panel - cable_trough + targets + force_target_label_tabs()
}

fn force_target_label_tabs() -> Part {
    let mut tabs = Part::empty("robot_collision_validation_force_target_label_tabs");
    for (i, x) in [-76.0, 0.0, 76.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("robot_collision_validation_force_target_trace_label_{i}"),
                52.0,
                15.0,
                3.0,
            )
            .translate(*x, -FORCE_PANEL_Y / 2.0 + 15.0, FORCE_PANEL_Z / 2.0 + 1.5);
    }
    tabs
}

fn compliant_cassette_surrogate() -> Part {
    let panel = module_panel(
        "robot_collision_validation_cassette_surrogate_panel",
        CASSETTE_PANEL_X,
        CASSETTE_PANEL_Y,
        CASSETTE_PANEL_Z,
    );
    let surrogate_body = centered_cube(
        "robot_collision_validation_compliant_cassette_surrogate_body",
        SURROGATE_X,
        SURROGATE_Y,
        SURROGATE_Z,
    )
    .translate(0.0, 0.0, CASSETTE_PANEL_Z / 2.0 + SURROGATE_Z / 2.0);
    let soft_core = centered_cube(
        "robot_collision_validation_compliant_cassette_soft_core_relief",
        SURROGATE_X - 34.0,
        SURROGATE_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, CASSETTE_PANEL_Z / 2.0 + SURROGATE_Z / 2.0 + 2.0);

    panel
        + (surrogate_body - soft_core)
        + cassette_bumper_ribs()
        + cassette_datum_receivers()
        + cassette_deflection_flags()
}

fn cassette_bumper_ribs() -> Part {
    let mut ribs = Part::empty("robot_collision_validation_cassette_bumper_ribs");
    for (i, y) in [-34.0, -17.0, 0.0, 17.0, 34.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("robot_collision_validation_cassette_longitudinal_bumper_rib_{i}"),
                SURROGATE_X + 12.0,
                5.0,
                9.0,
            )
            .translate(0.0, *y, CASSETTE_PANEL_Z / 2.0 + SURROGATE_Z + 4.5);
    }
    ribs
}

fn cassette_datum_receivers() -> Part {
    let mut receivers = Part::empty("robot_collision_validation_cassette_datum_receivers");
    for (i, (x, y)) in [
        (-SURROGATE_X / 2.0 + 22.0, -SURROGATE_Y / 2.0 + 18.0),
        (SURROGATE_X / 2.0 - 22.0, -SURROGATE_Y / 2.0 + 18.0),
        (-SURROGATE_X / 2.0 + 22.0, SURROGATE_Y / 2.0 - 18.0),
        (SURROGATE_X / 2.0 - 22.0, SURROGATE_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("robot_collision_validation_cassette_datum_receiver_boss_{i}"),
            11.0,
            7.0,
            32,
        )
        .translate(*x, *y, CASSETTE_PANEL_Z / 2.0 + SURROGATE_Z + 3.5);
        let hole = centered_cylinder(
            format!("robot_collision_validation_cassette_datum_receiver_hole_{i}"),
            4.4,
            9.0,
            28,
        )
        .translate(*x, *y, CASSETTE_PANEL_Z / 2.0 + SURROGATE_Z + 3.5);
        receivers = receivers + (boss - hole);
    }
    receivers
}

fn cassette_deflection_flags() -> Part {
    let mut flags = Part::empty("robot_collision_validation_cassette_deflection_flags");
    for (i, x) in [-72.0, -24.0, 24.0, 72.0].iter().enumerate() {
        flags = flags
            + centered_cube(
                format!("robot_collision_validation_cassette_deflection_witness_flag_{i}"),
                7.0,
                28.0,
                34.0,
            )
            .translate(*x, SURROGATE_Y / 2.0 + 13.0, CASSETTE_PANEL_Z / 2.0 + 17.0);
    }
    flags
}

fn staged_collision_posts() -> Part {
    let panel = module_panel(
        "robot_collision_validation_staged_collision_post_panel",
        COLLISION_PANEL_X,
        COLLISION_PANEL_Y,
        COLLISION_PANEL_Z,
    );
    let mut posts = Part::empty("robot_collision_validation_staged_collision_posts");
    for (i, x) in [-78.0, -26.0, 26.0, 78.0].iter().enumerate() {
        let height = 30.0 + i as f64 * 18.0;
        let post = centered_cylinder(
            format!("robot_collision_validation_collision_post_stage_{i}"),
            11.0 + i as f64 * 1.5,
            height,
            40,
        )
        .translate(*x, -4.0, COLLISION_PANEL_Z / 2.0 + height / 2.0);
        let base_ring = centered_cylinder(
            format!("robot_collision_validation_collision_post_base_ring_{i}"),
            22.0,
            8.0,
            40,
        )
        .translate(*x, -4.0, COLLISION_PANEL_Z / 2.0 + 4.0);
        let witness_collar = centered_cylinder(
            format!("robot_collision_validation_collision_post_witness_collar_{i}"),
            17.0,
            5.0,
            40,
        )
        .translate(*x, -4.0, COLLISION_PANEL_Z / 2.0 + height - 8.0);
        let index_slot = centered_cube(
            format!("robot_collision_validation_collision_post_index_slot_{i}"),
            34.0,
            9.0,
            4.0,
        )
        .translate(
            *x,
            -COLLISION_PANEL_Y / 2.0 + 20.0,
            COLLISION_PANEL_Z / 2.0 + 2.0,
        );
        posts = posts + post + base_ring + witness_collar + index_slot;
    }

    panel + posts + post_guard_rail()
}

fn post_guard_rail() -> Part {
    let front = centered_cube(
        "robot_collision_validation_collision_post_front_guard",
        COLLISION_PANEL_X - 28.0,
        9.0,
        22.0,
    )
    .translate(
        0.0,
        -COLLISION_PANEL_Y / 2.0 + 12.0,
        COLLISION_PANEL_Z / 2.0 + 11.0,
    );
    let rear = centered_cube(
        "robot_collision_validation_collision_post_rear_guard",
        COLLISION_PANEL_X - 28.0,
        9.0,
        38.0,
    )
    .translate(
        0.0,
        COLLISION_PANEL_Y / 2.0 - 14.0,
        COLLISION_PANEL_Z / 2.0 + 19.0,
    );
    let left = centered_cube(
        "robot_collision_validation_collision_post_left_guard",
        9.0,
        COLLISION_PANEL_Y - 44.0,
        26.0,
    )
    .translate(
        -COLLISION_PANEL_X / 2.0 + 18.0,
        0.0,
        COLLISION_PANEL_Z / 2.0 + 13.0,
    );
    let right = centered_cube(
        "robot_collision_validation_collision_post_right_guard",
        9.0,
        COLLISION_PANEL_Y - 44.0,
        26.0,
    )
    .translate(
        COLLISION_PANEL_X / 2.0 - 18.0,
        0.0,
        COLLISION_PANEL_Z / 2.0 + 13.0,
    );

    front + rear + left + right
}

fn soft_hard_stop_coupons() -> Part {
    let panel = module_panel(
        "robot_collision_validation_stop_coupon_panel",
        STOP_PANEL_X,
        STOP_PANEL_Y,
        STOP_PANEL_Z,
    );
    let soft_row = stop_coupon_row("soft", -30.0, 18.0, 38.0);
    let hard_row = stop_coupon_row("hard", 32.0, 30.0, 58.0);
    let center_divider = centered_cube(
        "robot_collision_validation_stop_coupon_soft_hard_divider",
        STOP_PANEL_X - 24.0,
        8.0,
        28.0,
    )
    .translate(0.0, 0.0, STOP_PANEL_Z / 2.0 + 14.0);
    let latch_bar = centered_cube(
        "robot_collision_validation_stop_coupon_retain_latch_bar",
        STOP_PANEL_X - 36.0,
        10.0,
        12.0,
    )
    .translate(0.0, -STOP_PANEL_Y / 2.0 + 16.0, STOP_PANEL_Z / 2.0 + 6.0);

    panel + soft_row + hard_row + center_divider + latch_bar
}

fn stop_coupon_row(kind: &str, y: f64, stop_z: f64, overtravel_z: f64) -> Part {
    let mut row = Part::empty(format!("robot_collision_validation_{kind}_stop_coupon_row"));
    for (i, x) in [-54.0, 0.0, 54.0].iter().enumerate() {
        let coupon = centered_cube(
            format!("robot_collision_validation_{kind}_stop_coupon_body_{i}"),
            40.0,
            22.0,
            stop_z,
        )
        .translate(*x, y, STOP_PANEL_Z / 2.0 + stop_z / 2.0);
        let witness_tip = centered_cube(
            format!("robot_collision_validation_{kind}_stop_coupon_witness_tip_{i}"),
            34.0,
            6.0,
            8.0,
        )
        .translate(*x, y - 14.0, STOP_PANEL_Z / 2.0 + stop_z + 4.0);
        let rear_overtravel = centered_cube(
            format!("robot_collision_validation_{kind}_stop_coupon_overtravel_reference_{i}"),
            36.0,
            7.0,
            overtravel_z,
        )
        .translate(*x, y + 18.0, STOP_PANEL_Z / 2.0 + overtravel_z / 2.0);
        row = row + coupon + witness_tip + rear_overtravel;
    }
    row
}

fn calibration_mass_pockets() -> Part {
    let panel = module_panel(
        "robot_collision_validation_calibration_mass_panel",
        MASS_PANEL_X,
        MASS_PANEL_Y,
        MASS_PANEL_Z,
    );
    let mut pockets = Part::empty("robot_collision_validation_calibration_mass_pocket_cuts");
    let mut rims = Part::empty("robot_collision_validation_calibration_mass_pocket_rims");
    for (i, (x, radius)) in [(-78.0, 16.0), (-26.0, 20.0), (30.0, 24.0), (88.0, 28.0)]
        .iter()
        .enumerate()
    {
        pockets = pockets
            + centered_cylinder(
                format!("robot_collision_validation_calibration_mass_pocket_cut_{i}"),
                *radius,
                18.0,
                48,
            )
            .translate(*x, 18.0, MASS_PANEL_Z / 2.0 + 3.0);
        let rim = centered_cylinder(
            format!("robot_collision_validation_calibration_mass_pocket_rim_{i}"),
            *radius + 5.0,
            5.0,
            48,
        )
        .translate(*x, 18.0, MASS_PANEL_Z / 2.0 + 2.5);
        let inner = centered_cylinder(
            format!("robot_collision_validation_calibration_mass_pocket_rim_inner_{i}"),
            *radius - 1.0,
            7.0,
            48,
        )
        .translate(*x, 18.0, MASS_PANEL_Z / 2.0 + 2.5);
        let retention_clip = centered_cube(
            format!("robot_collision_validation_calibration_mass_retention_clip_{i}"),
            *radius * 1.5,
            7.0,
            9.0,
        )
        .translate(*x, -14.0, MASS_PANEL_Z / 2.0 + 5.0);
        rims = rims + (rim - inner) + retention_clip;
    }

    panel - pockets + rims + calibration_mass_certificate_rail()
}

fn calibration_mass_certificate_rail() -> Part {
    let rail = centered_cube(
        "robot_collision_validation_calibration_mass_certificate_rail",
        MASS_PANEL_X - 30.0,
        14.0,
        13.0,
    )
    .translate(0.0, -MASS_PANEL_Y / 2.0 + 21.0, MASS_PANEL_Z / 2.0 + 6.5);
    let mut witness_dots = Part::empty("robot_collision_validation_calibration_mass_witness_dots");
    for (i, x) in [-92.0, -46.0, 0.0, 46.0, 92.0].iter().enumerate() {
        witness_dots = witness_dots
            + centered_cylinder(
                format!("robot_collision_validation_calibration_mass_witness_dot_{i}"),
                4.0,
                3.0,
                24,
            )
            .translate(*x, -MASS_PANEL_Y / 2.0 + 36.0, MASS_PANEL_Z / 2.0 + 1.5);
    }
    rail + witness_dots
}

fn barcode_certificate_lands() -> Part {
    let panel = module_panel(
        "robot_collision_validation_barcode_certificate_panel",
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    );
    let mut lands = Part::empty("robot_collision_validation_barcode_certificate_lands");
    for (i, x) in [-144.0, -96.0, -48.0, 0.0, 48.0, 96.0].iter().enumerate() {
        let land = centered_cube(
            format!("robot_collision_validation_barcode_scan_land_{i}"),
            38.0,
            22.0,
            3.0,
        )
        .translate(*x, -3.0, LABEL_PANEL_Z / 2.0 + 1.5);
        lands = lands + land + barcode_stripes(i, *x, -3.0, LABEL_PANEL_Z / 2.0 + 3.4);
    }
    let certificate_land = centered_cube(
        "robot_collision_validation_certificate_laminate_land",
        78.0,
        28.0,
        3.0,
    )
    .translate(144.0, -1.0, LABEL_PANEL_Z / 2.0 + 1.5);
    let punch_bank = punch_witness_bank();

    panel + lands + certificate_land + punch_bank
}

fn barcode_stripes(index: usize, x: f64, y: f64, z: f64) -> Part {
    let mut stripes = Part::empty(format!(
        "robot_collision_validation_barcode_stripes_{index}"
    ));
    for (bar, dx) in [-13.0, -8.0, -2.0, 5.0, 11.0].iter().enumerate() {
        let width = if bar % 2 == 0 { 2.4 } else { 1.2 };
        stripes = stripes
            + centered_cube(
                format!("robot_collision_validation_barcode_{index}_stripe_{bar}"),
                width,
                18.0,
                1.2,
            )
            .translate(x + dx, y, z);
    }
    stripes
}

fn punch_witness_bank() -> Part {
    let mut bank = Part::empty("robot_collision_validation_certificate_witness_punch_bank");
    for (i, x) in [114.0, 134.0, 154.0, 174.0].iter().enumerate() {
        bank = bank
            + centered_cylinder(
                format!("robot_collision_validation_certificate_witness_punch_{i}"),
                5.0,
                3.0,
                24,
            )
            .translate(*x, 18.0, LABEL_PANEL_Z / 2.0 + 1.5);
    }
    bank
}

fn release_hold_reject_lanes() -> Part {
    let panel = module_panel(
        "robot_collision_validation_release_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut lanes = Part::empty("robot_collision_validation_disposition_lanes");
    for (i, (x, name)) in [(-58.0, "release"), (0.0, "hold"), (58.0, "reject")]
        .iter()
        .enumerate()
    {
        let trough = centered_cube(
            format!("robot_collision_validation_{name}_lane_recess"),
            44.0,
            LANE_PANEL_Y - 30.0,
            7.0,
        )
        .translate(*x, 0.0, LANE_PANEL_Z / 2.0 - 3.5);
        let lane_floor = centered_cube(
            format!("robot_collision_validation_{name}_lane_floor_witness_pad"),
            36.0,
            LANE_PANEL_Y - 42.0,
            3.0,
        )
        .translate(*x, 0.0, LANE_PANEL_Z / 2.0 + 1.5);
        let gate = centered_cube(
            format!("robot_collision_validation_{name}_lane_gate_{i}"),
            38.0,
            8.0,
            22.0 + i as f64 * 8.0,
        )
        .translate(
            *x,
            LANE_PANEL_Y / 2.0 - 23.0,
            LANE_PANEL_Z / 2.0 + 11.0 + i as f64 * 4.0,
        );
        let front_lip = centered_cube(
            format!("robot_collision_validation_{name}_lane_front_lip"),
            46.0,
            8.0,
            14.0,
        )
        .translate(*x, -LANE_PANEL_Y / 2.0 + 16.0, LANE_PANEL_Z / 2.0 + 7.0);
        lanes = lanes - trough + lane_floor + gate + front_lip;
    }

    let dividers = centered_cube(
        "robot_collision_validation_left_lane_divider",
        6.0,
        LANE_PANEL_Y - 24.0,
        28.0,
    )
    .translate(-29.0, 0.0, LANE_PANEL_Z / 2.0 + 14.0)
        + centered_cube(
            "robot_collision_validation_right_lane_divider",
            6.0,
            LANE_PANEL_Y - 24.0,
            28.0,
        )
        .translate(29.0, 0.0, LANE_PANEL_Z / 2.0 + 14.0);

    panel + lanes + dividers
}

fn evidence_camera_bridge() -> Part {
    let mut posts = Part::empty("robot_collision_validation_evidence_camera_bridge_posts");
    for (i, (x, y)) in [
        (-BRIDGE_POST_SPAN_X / 2.0, -BRIDGE_POST_SPAN_Y / 2.0),
        (BRIDGE_POST_SPAN_X / 2.0, -BRIDGE_POST_SPAN_Y / 2.0),
        (-BRIDGE_POST_SPAN_X / 2.0, BRIDGE_POST_SPAN_Y / 2.0),
        (BRIDGE_POST_SPAN_X / 2.0, BRIDGE_POST_SPAN_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("robot_collision_validation_evidence_bridge_post_{i}"),
                24.0,
                24.0,
                BRIDGE_UNDERSIDE_Z,
            )
            .translate(*x, *y, BRIDGE_UNDERSIDE_Z / 2.0)
            + centered_cube(
                format!("robot_collision_validation_evidence_bridge_foot_{i}"),
                54.0,
                42.0,
                10.0,
            )
            .translate(*x, *y, 5.0);
    }

    let front_beam = centered_cube(
        "robot_collision_validation_evidence_bridge_front_beam",
        BRIDGE_POST_SPAN_X + 34.0,
        24.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_POST_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        "robot_collision_validation_evidence_bridge_rear_beam",
        BRIDGE_POST_SPAN_X + 34.0,
        24.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_POST_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let left_beam = centered_cube(
        "robot_collision_validation_evidence_bridge_left_beam",
        24.0,
        BRIDGE_POST_SPAN_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        -BRIDGE_POST_SPAN_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let right_beam = centered_cube(
        "robot_collision_validation_evidence_bridge_right_beam",
        24.0,
        BRIDGE_POST_SPAN_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POST_SPAN_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    posts + front_beam + rear_beam + left_beam + right_beam + camera_heads() + bridge_scale_ticks()
}

fn camera_heads() -> Part {
    let mut heads = Part::empty("robot_collision_validation_evidence_camera_heads");
    for (i, (x, y)) in [(-230.0, 20.0), (0.0, -6.0), (230.0, 20.0)]
        .iter()
        .enumerate()
    {
        let sled = centered_cube(
            format!("robot_collision_validation_evidence_camera_sled_{i}"),
            78.0,
            34.0,
            16.0,
        )
        .translate(*x, *y, BRIDGE_UNDERSIDE_Z - 8.0);
        let lens = centered_cylinder(
            format!("robot_collision_validation_evidence_camera_lens_cone_{i}"),
            13.0,
            26.0,
            36,
        )
        .translate(*x, *y, BRIDGE_UNDERSIDE_Z - 29.0);
        let light_bar = centered_cube(
            format!("robot_collision_validation_evidence_camera_light_bar_{i}"),
            66.0,
            8.0,
            8.0,
        )
        .translate(*x, *y - 27.0, BRIDGE_UNDERSIDE_Z - 18.0);
        heads = heads + sled + lens + light_bar;
    }
    heads
}

fn bridge_scale_ticks() -> Part {
    let mut ticks = Part::empty("robot_collision_validation_evidence_bridge_scale_ticks");
    for (i, x) in (-5..=5).map(|n| n as f64 * 48.0).enumerate() {
        let tick_height = if i % 2 == 0 { 15.0 } else { 9.0 };
        ticks = ticks
            + centered_cube(
                format!("robot_collision_validation_evidence_bridge_scale_tick_{i}"),
                3.0,
                7.0,
                tick_height,
            )
            .translate(
                x,
                -BRIDGE_POST_SPAN_Y / 2.0 - 16.0,
                BRIDGE_UNDERSIDE_Z - tick_height / 2.0,
            );
    }
    ticks
}

fn robot_service_keepout_gauges() -> Part {
    let robot_sweep = horizontal_frame(
        "robot_collision_validation_robot_sweep_keepout",
        720.0,
        408.0,
        8.0,
        8.0,
        34.0,
    );
    let rear_service = centered_cube(
        "robot_collision_validation_rear_service_keepout_gauge_wall",
        690.0,
        10.0,
        92.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 68.0, 46.0);
    let front_robot = centered_cube(
        "robot_collision_validation_front_robot_approach_keepout_gauge",
        360.0,
        10.0,
        58.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 76.0, 29.0);
    let side_service = centered_cube(
        "robot_collision_validation_right_service_hand_clearance_gauge",
        10.0,
        250.0,
        70.0,
    )
    .translate(DECK_X / 2.0 - 72.0, -30.0, 35.0);

    robot_sweep + rear_service + front_robot + side_service + keepout_height_comb()
}

fn keepout_height_comb() -> Part {
    let mut comb = Part::empty("robot_collision_validation_keepout_height_comb");
    for (i, height) in [28.0, 44.0, 60.0, 76.0, 92.0].iter().enumerate() {
        comb = comb
            + centered_cube(
                format!("robot_collision_validation_keepout_height_tooth_{i}"),
                18.0,
                12.0,
                *height,
            )
            .translate(
                -DECK_X / 2.0 + 76.0 + i as f64 * 26.0,
                DECK_Y / 2.0 - 76.0,
                height / 2.0,
            );
    }
    comb
}

fn module_panel(name: &str, x: f64, y: f64, z: f64) -> Part {
    let plate = centered_cube(format!("{name}_plate"), x, y, z).translate(0.0, 0.0, z / 2.0);
    let front = centered_cube(format!("{name}_front_lip"), x, 8.0, 10.0).translate(
        0.0,
        -y / 2.0 + 4.0,
        z + 5.0,
    );
    let rear = centered_cube(format!("{name}_rear_lip"), x, 8.0, 10.0).translate(
        0.0,
        y / 2.0 - 4.0,
        z + 5.0,
    );
    let left = centered_cube(format!("{name}_left_lip"), 8.0, y - 16.0, 10.0).translate(
        -x / 2.0 + 4.0,
        0.0,
        z + 5.0,
    );
    let right = centered_cube(format!("{name}_right_lip"), 8.0, y - 16.0, 10.0).translate(
        x / 2.0 - 4.0,
        0.0,
        z + 5.0,
    );

    plate + front + rear + left + right
}

fn horizontal_frame(name: &str, outer_x: f64, outer_y: f64, rail: f64, z_t: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), outer_x, rail, z_t).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        z,
    );
    let rear = centered_cube(format!("{name}_rear"), outer_x, rail, z_t).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        z,
    );
    let left = centered_cube(format!("{name}_left"), rail, outer_y, z_t).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        z,
    );
    let right = centered_cube(format!("{name}_right"), rail, outer_y, z_t).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        z,
    );
    front + rear + left + right
}

fn slug(name: &str) -> String {
    name.replace([' ', '/'], "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_station_features_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 9);
        assert!(REQUIRED_FEATURES.contains(&"force_gauge_target_blocks"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepout_gauges"));
    }

    #[test]
    fn validation_inserts_fit_and_do_not_overlap() {
        assert_layout();
    }

    #[test]
    fn expected_outputs_cover_modules_and_assembly() {
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }
}
