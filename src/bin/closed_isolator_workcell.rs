use vcad::{centered_cube, centered_cylinder, Part};

// Closed isolator workcell for automated tissue-on-chip handling.
//
// Research direction:
// - The isolator, not the walk-in pod, is the primary sterile process boundary.
// - Loading/open aseptic operations happen under ISO 5 / Grade A first air.
// - Pumps/controllers stay outside the Grade A volume where possible.
// - Transfers use RTP/VHP hatches and validated service penetrations, not ad hoc holes.
//
// This is an architecture CAD model for fit/BOM planning, not a certified isolator.

const CELL_X: f64 = 2400.0;
const CELL_Y: f64 = 1120.0;
const CELL_Z: f64 = 1780.0;
const WALL: f64 = 55.0;
const DECK_Z: f64 = 90.0;
const CHAMBER_Z: f64 = 860.0;
const PLENUM_Z: f64 = 260.0;
const BASE_Z: f64 = 560.0;
const SERVICE_Z: f64 = 340.0;

const MODULE_BAY_X: f64 = 1180.0;
const MODULE_BAY_Y: f64 = 520.0;
const MODULE_BAY_Z: f64 = 390.0;
const RTP_SMALL_D: f64 = 190.0;
const RTP_TRAY_D: f64 = 270.0;

fn main() {
    let shell = isolator_shell();
    shell
        .write_stl("output/closed_isolator_workcell_shell.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_shell.stl");

    let deck = work_deck();
    deck.write_stl("output/closed_isolator_workcell_deck.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_deck.stl");

    let plenum = hepa_plenum();
    plenum
        .write_stl("output/closed_isolator_workcell_hepa_plenum.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_hepa_plenum.stl");

    let transfers = transfer_ports_and_hatch();
    transfers
        .write_stl("output/closed_isolator_workcell_transfer_ports.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_transfer_ports.stl");

    let service = service_penetration_panel();
    service
        .write_stl("output/closed_isolator_workcell_service_panel.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_service_panel.stl");

    let module_bay = incubated_module_bay();
    module_bay
        .write_stl("output/closed_isolator_workcell_module_bay.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_module_bay.stl");

    let assembly = shell
        + deck.translate(0.0, 0.0, BASE_Z + DECK_Z / 2.0)
        + plenum.translate(0.0, 0.0, BASE_Z + DECK_Z + CHAMBER_Z + PLENUM_Z / 2.0)
        + transfers.translate(-CELL_X / 2.0 - 26.0, -90.0, BASE_Z + DECK_Z + 410.0)
        + service.translate(CELL_X / 2.0 + 28.0, 0.0, BASE_Z + DECK_Z + 360.0)
        + module_bay.translate(360.0, CELL_Y / 2.0 + 58.0, BASE_Z + DECK_Z + 230.0);

    assembly
        .write_stl("output/closed_isolator_workcell_assembly.stl")
        .unwrap();
    println!("Exported: output/closed_isolator_workcell_assembly.stl");

    println!(
        "Closed isolator workcell: {:.0}mm x {:.0}mm x {:.0}mm envelope, Grade A chamber, HEPA plenum, RTP/VHP transfer features, service panel, and rear incubated module bay.",
        CELL_X, CELL_Y, CELL_Z
    );
}

fn isolator_shell() -> Part {
    let outer = centered_cube("isolator_outer_shell", CELL_X, CELL_Y, CELL_Z).translate(
        0.0,
        0.0,
        CELL_Z / 2.0,
    );
    let chamber = centered_cube(
        "isolator_grade_a_chamber",
        CELL_X - WALL * 2.0,
        CELL_Y - WALL * 2.0,
        CHAMBER_Z,
    )
    .translate(0.0, 0.0, BASE_Z + DECK_Z + CHAMBER_Z / 2.0);
    let service_base = centered_cube(
        "isolator_service_base_opening",
        CELL_X - WALL * 2.0,
        CELL_Y - WALL * 2.0,
        BASE_Z - 70.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let front_window = centered_cube(
        "isolator_front_window_cut",
        CELL_X - 360.0,
        WALL + 8.0,
        520.0,
    )
    .translate(0.0, -CELL_Y / 2.0, BASE_Z + DECK_Z + 480.0);
    let rear_module_cut = centered_cube(
        "isolator_rear_module_bay_cut",
        MODULE_BAY_X + 50.0,
        WALL + 8.0,
        MODULE_BAY_Z + 40.0,
    )
    .translate(360.0, CELL_Y / 2.0, BASE_Z + DECK_Z + 230.0);

    let body = outer - chamber - service_base - front_window - rear_module_cut;
    body + coved_corner_standins() + support_frame()
}

fn coved_corner_standins() -> Part {
    let mut coves = Part::empty("isolator_coved_corner_standins");
    for (i, (x, y)) in [
        (-(CELL_X / 2.0 - WALL - 18.0), -(CELL_Y / 2.0 - WALL - 18.0)),
        (CELL_X / 2.0 - WALL - 18.0, -(CELL_Y / 2.0 - WALL - 18.0)),
        (-(CELL_X / 2.0 - WALL - 18.0), CELL_Y / 2.0 - WALL - 18.0),
        (CELL_X / 2.0 - WALL - 18.0, CELL_Y / 2.0 - WALL - 18.0),
    ]
    .iter()
    .enumerate()
    {
        coves = coves
            + centered_cylinder(format!("isolator_vertical_cove_{i}"), 18.0, CHAMBER_Z, 32)
                .translate(*x, *y, BASE_Z + DECK_Z + CHAMBER_Z / 2.0);
    }
    coves
}

fn support_frame() -> Part {
    let post_w = 55.0;
    let mut frame = Part::empty("isolator_base_support_frame");
    for (i, (x, y)) in [
        (-(CELL_X / 2.0 - 70.0), -(CELL_Y / 2.0 - 70.0)),
        (CELL_X / 2.0 - 70.0, -(CELL_Y / 2.0 - 70.0)),
        (-(CELL_X / 2.0 - 70.0), CELL_Y / 2.0 - 70.0),
        (CELL_X / 2.0 - 70.0, CELL_Y / 2.0 - 70.0),
        (0.0, -(CELL_Y / 2.0 - 70.0)),
        (0.0, CELL_Y / 2.0 - 70.0),
    ]
    .iter()
    .enumerate()
    {
        frame =
            frame
                + centered_cube(format!("isolator_base_post_{i}"), post_w, post_w, BASE_Z)
                    .translate(*x, *y, BASE_Z / 2.0);
    }

    let front_rail = centered_cube("isolator_front_base_rail", CELL_X - 140.0, post_w, post_w)
        .translate(0.0, -(CELL_Y / 2.0 - 70.0), BASE_Z - post_w / 2.0);
    let rear_rail = centered_cube("isolator_rear_base_rail", CELL_X - 140.0, post_w, post_w)
        .translate(0.0, CELL_Y / 2.0 - 70.0, BASE_Z - post_w / 2.0);
    frame + front_rail + rear_rail
}

fn work_deck() -> Part {
    let deck = centered_cube(
        "isolator_316l_work_deck",
        CELL_X - 140.0,
        CELL_Y - 150.0,
        DECK_Z,
    );
    let recessed_pan = centered_cube(
        "isolator_work_deck_recessed_pan",
        CELL_X - 300.0,
        CELL_Y - 300.0,
        18.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 9.0);
    let drain_slot = centered_cube("isolator_deck_drain_slot", 850.0, 18.0, 16.0).translate(
        280.0,
        -(CELL_Y / 2.0 - 260.0),
        DECK_Z / 2.0 - 8.0,
    );
    let drain_port = centered_cylinder("isolator_deck_drain_port", 18.0 / 2.0, 42.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(840.0, -(CELL_Y / 2.0 - 230.0), 0.0);

    deck - recessed_pan - drain_slot - drain_port + robot_datums() + module_transfer_rails()
}

fn robot_datums() -> Part {
    let mut datums = Part::empty("isolator_robot_datums");
    for (i, (x, y)) in [
        (-900.0, -315.0),
        (900.0, -315.0),
        (-900.0, 315.0),
        (900.0, 315.0),
        (0.0, -315.0),
        (0.0, 315.0),
    ]
    .iter()
    .enumerate()
    {
        let fid = centered_cylinder(format!("isolator_deck_fiducial_{i}"), 12.0, 3.0, 40)
            .translate(*x, *y, DECK_Z / 2.0 + 1.5);
        let center = centered_cylinder(format!("isolator_deck_fiducial_center_{i}"), 2.0, 4.0, 20)
            .translate(*x, *y, DECK_Z / 2.0 + 1.5);
        datums = datums + (fid - center);
    }
    datums
}

fn module_transfer_rails() -> Part {
    let left = centered_cube(
        "isolator_module_transfer_left_rail",
        MODULE_BAY_X,
        24.0,
        22.0,
    )
    .translate(360.0, 260.0, DECK_Z / 2.0 + 11.0);
    let right = centered_cube(
        "isolator_module_transfer_right_rail",
        MODULE_BAY_X,
        24.0,
        22.0,
    )
    .translate(360.0, 340.0, DECK_Z / 2.0 + 11.0);
    left + right
}

fn hepa_plenum() -> Part {
    let body = centered_cube(
        "isolator_hepa_plenum_body",
        CELL_X - 120.0,
        CELL_Y - 120.0,
        PLENUM_Z,
    );
    let mut filter_cuts = Part::empty("isolator_hepa_filter_cuts");
    for (i, x) in [-720.0, 0.0, 720.0].iter().enumerate() {
        filter_cuts = filter_cuts
            + centered_cube(
                format!("isolator_h14_filter_cut_{i}"),
                610.0,
                610.0,
                PLENUM_Z + 2.0,
            )
            .translate(*x, -80.0, 0.0);
    }
    let supply_manifold =
        centered_cube("isolator_supply_manifold_stub", CELL_X - 260.0, 95.0, 78.0).translate(
            0.0,
            CELL_Y / 2.0 - 150.0,
            PLENUM_Z / 2.0 + 39.0,
        );
    let smoke_access = centered_cube(
        "isolator_airflow_smoke_access_land",
        CELL_X - 420.0,
        8.0,
        38.0,
    )
    .translate(0.0, -(CELL_Y / 2.0 - 140.0), -PLENUM_Z / 2.0 + 38.0);
    body - filter_cuts + supply_manifold + smoke_access + particle_sample_ports()
}

fn particle_sample_ports() -> Part {
    let mut ports = Part::empty("isolator_particle_sample_ports");
    for (i, x) in [-950.0, -475.0, 0.0, 475.0, 950.0].iter().enumerate() {
        ports = ports
            + centered_cylinder(
                format!("isolator_particle_sample_nozzle_{i}"),
                7.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -(CELL_Y / 2.0 - 88.0), -48.0);
    }
    ports
}

fn transfer_ports_and_hatch() -> Part {
    let panel = centered_cube("isolator_transfer_panel", 54.0, 650.0, 720.0);
    let rtp_small = rtp_port("small", RTP_SMALL_D).translate(0.0, -185.0, 120.0);
    let rtp_tray = rtp_port("tray", RTP_TRAY_D).translate(0.0, 160.0, 110.0);
    let vhp_hatch = vhp_transfer_hatch().translate(0.0, 0.0, -230.0);
    panel + rtp_small + rtp_tray + vhp_hatch
}

fn rtp_port(name: &str, diameter: f64) -> Part {
    let flange = centered_cylinder(
        format!("isolator_rtp_{name}_flange"),
        diameter / 2.0 + 22.0,
        20.0,
        72,
    )
    .rotate(0.0, 90.0, 0.0);
    let bore = centered_cylinder(
        format!("isolator_rtp_{name}_bore"),
        diameter / 2.0,
        24.0,
        72,
    )
    .rotate(0.0, 90.0, 0.0);
    let door = centered_cylinder(
        format!("isolator_rtp_{name}_door_placeholder"),
        diameter / 2.0 - 12.0,
        10.0,
        72,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-8.0, 0.0, 0.0);
    flange - bore + door
}

fn vhp_transfer_hatch() -> Part {
    let outer = centered_cube("isolator_vhp_hatch_outer", 70.0, 420.0, 230.0);
    let inner = centered_cube("isolator_vhp_hatch_door_clearance", 74.0, 360.0, 176.0);
    let interlock_lands = centered_cube("isolator_vhp_hatch_interlock_top", 76.0, 390.0, 18.0)
        .translate(0.0, 0.0, 122.0)
        + centered_cube("isolator_vhp_hatch_interlock_bottom", 76.0, 390.0, 18.0)
            .translate(0.0, 0.0, -122.0);
    outer - inner + interlock_lands
}

fn service_penetration_panel() -> Part {
    let body = centered_cube("isolator_hygienic_service_panel", 62.0, 760.0, SERVICE_Z);
    let mut cuts = Part::empty("isolator_service_panel_cuts");

    for (i, z) in [110.0, 70.0, 30.0, -10.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(format!("isolator_gas_bulkhead_{i}"), 12.0 / 2.0, 72.0, 28)
                .rotate(0.0, 90.0, 0.0)
                .translate(0.0, -280.0, *z);
    }

    for (i, y) in [-160.0, -120.0, -80.0, -40.0, 0.0, 40.0, 80.0, 120.0]
        .iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cylinder(format!("isolator_fluid_bulkhead_{i}"), 8.0 / 2.0, 72.0, 28)
                .rotate(0.0, 90.0, 0.0)
                .translate(0.0, *y, 44.0);
    }

    let cable_transit = centered_cube("isolator_sealed_cable_transit", 68.0, 210.0, 74.0)
        .translate(0.0, 245.0, 44.0);
    let dp_ports = centered_cylinder("isolator_dp_sample_port_high", 6.0 / 2.0, 72.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 260.0, -70.0)
        + centered_cylinder("isolator_dp_sample_port_low", 6.0 / 2.0, 72.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 300.0, -70.0);

    body - cuts - cable_transit - dp_ports + service_label_lands()
}

fn service_label_lands() -> Part {
    centered_cube("isolator_service_label_land_upper", 68.0, 650.0, 10.0).translate(0.0, 0.0, 150.0)
        + centered_cube("isolator_service_label_land_lower", 68.0, 650.0, 10.0)
            .translate(0.0, 0.0, -135.0)
}

fn incubated_module_bay() -> Part {
    let shell = centered_cube(
        "isolator_rear_incubated_module_bay_shell",
        MODULE_BAY_X,
        MODULE_BAY_Y,
        MODULE_BAY_Z,
    );
    let cavity = centered_cube(
        "isolator_rear_module_bay_cavity",
        MODULE_BAY_X - 70.0,
        MODULE_BAY_Y - 70.0,
        MODULE_BAY_Z - 58.0,
    )
    .translate(0.0, 0.0, 12.0);
    let front_gate = centered_cube(
        "isolator_module_bay_transfer_gate_cut",
        MODULE_BAY_X - 180.0,
        78.0,
        MODULE_BAY_Z - 110.0,
    )
    .translate(0.0, -(MODULE_BAY_Y / 2.0 - 20.0), 10.0);

    let left_dock = module_dock_placeholder("left").translate(-285.0, 0.0, -80.0);
    let right_dock = module_dock_placeholder("right").translate(285.0, 0.0, -80.0);
    let gas_mix_plenum = centered_cube(
        "isolator_module_bay_gas_mix_plenum",
        MODULE_BAY_X - 120.0,
        62.0,
        55.0,
    )
    .translate(0.0, MODULE_BAY_Y / 2.0 - 62.0, MODULE_BAY_Z / 2.0 - 45.0);
    let sensor_strip = centered_cube(
        "isolator_module_bay_temp_co2_rh_sensor_strip",
        MODULE_BAY_X - 180.0,
        20.0,
        28.0,
    )
    .translate(0.0, -(MODULE_BAY_Y / 2.0 - 64.0), MODULE_BAY_Z / 2.0 - 54.0);

    shell - cavity - front_gate + left_dock + right_dock + gas_mix_plenum + sensor_strip
}

fn module_dock_placeholder(name: &str) -> Part {
    let tray = centered_cube(
        format!("isolator_module_bay_{name}_dock_tray"),
        430.0,
        330.0,
        34.0,
    );
    let rails = centered_cube(
        format!("isolator_module_bay_{name}_dock_left_rail"),
        24.0,
        300.0,
        38.0,
    )
    .translate(-190.0, 0.0, 36.0)
        + centered_cube(
            format!("isolator_module_bay_{name}_dock_right_rail"),
            24.0,
            300.0,
            38.0,
        )
        .translate(190.0, 0.0, 36.0);
    let service_receiver = centered_cube(
        format!("isolator_module_bay_{name}_service_receiver"),
        350.0,
        28.0,
        82.0,
    )
    .translate(0.0, 165.0, 58.0);
    tray + rails + service_receiver
}
