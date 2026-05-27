use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Waste/decontamination service pod for a closed automated tissue-chip workcell.
//
// Intent:
// - Package level-sensed liquid waste, secondary containment, filtered venting,
//   placeholder contact-time/thermal treatment envelopes, sample/neutralization
//   access, solid waste pass-out staging, overflow interlocks, drain routing,
//   and front/rear service access in one dockable service pod.
// - Keep contaminated services outside the closed isolator work volume and make
//   every wet/service interface explicit at a gasketed bulkhead or drawer.
//
// This is architecture packaging CAD. Geometry marked as contact-time, thermal,
// disinfectant, or filter hardware is a placeholder for future selection and
// validation; it does not imply pathogen-grade kill validation.

const OUTPUTS: &[&str] = &[
    "output/waste_decon_service_pod_frame.stl",
    "output/waste_decon_service_pod_secondary_containment.stl",
    "output/waste_decon_service_pod_liquid_waste_cassette.stl",
    "output/waste_decon_service_pod_filtered_vent_stack.stl",
    "output/waste_decon_service_pod_kill_contact_placeholder.stl",
    "output/waste_decon_service_pod_neutralization_sample_panel.stl",
    "output/waste_decon_service_pod_solid_passout_drawer.stl",
    "output/waste_decon_service_pod_overflow_interlocks.stl",
    "output/waste_decon_service_pod_drain_routing.stl",
    "output/waste_decon_service_pod_service_access.stl",
    "output/waste_decon_service_pod_assembly.stl",
];

const POD_X: f64 = 920.0;
const POD_Y: f64 = 560.0;
const POD_Z: f64 = 900.0;
const FRAME_W: f64 = 30.0;
const BASE_PAN_Z: f64 = 64.0;
const SERVICE_CLEARANCE_FRONT: f64 = 480.0;
const SERVICE_CLEARANCE_REAR: f64 = 220.0;

const LIQUID_CASSETTE_X: f64 = 560.0;
const LIQUID_CASSETTE_Y: f64 = 262.0;
const LIQUID_CASSETTE_Z: f64 = 318.0;
const PRIMARY_BOTTLE_D: f64 = 154.0;
const SECONDARY_BOTTLE_D: f64 = 116.0;
const BOTTLE_CLEARANCE_D: f64 = 10.0;
const NOMINAL_LIQUID_FILL_Z: f64 = 248.0;
const HIGH_LEVEL_SENSOR_Z: f64 = 270.0;
const HIGH_HIGH_SENSOR_Z: f64 = 296.0;

const VENT_STACK_X: f64 = 470.0;
const VENT_STACK_Y: f64 = 74.0;
const VENT_STACK_Z: f64 = 162.0;
const VENT_FILTERS: usize = 3;

const KILL_MODULE_X: f64 = 430.0;
const KILL_MODULE_Y: f64 = 142.0;
const KILL_MODULE_Z: f64 = 118.0;
const DWELL_CHANNELS: usize = 5;

const SAMPLE_PANEL_X: f64 = 310.0;
const SAMPLE_PANEL_Y: f64 = 42.0;
const SAMPLE_PANEL_Z: f64 = 246.0;

const SOLID_DRAWER_X: f64 = 360.0;
const SOLID_DRAWER_Y: f64 = 252.0;
const SOLID_DRAWER_Z: f64 = 186.0;

const INTERLOCK_PANEL_X: f64 = 274.0;
const INTERLOCK_PANEL_Y: f64 = 34.0;
const INTERLOCK_PANEL_Z: f64 = 310.0;

const DRAIN_MANIFOLD_X: f64 = 610.0;
const DRAIN_MANIFOLD_Y: f64 = 54.0;
const DRAIN_MANIFOLD_Z: f64 = 82.0;
const DRAIN_PORTS: usize = 5;

const FRONT_PANEL_X: f64 = 760.0;
const FRONT_PANEL_Y: f64 = 28.0;
const FRONT_PANEL_Z: f64 = 540.0;
const REAR_PANEL_X: f64 = 620.0;
const REAR_PANEL_Y: f64 = 26.0;
const REAR_PANEL_Z: f64 = 420.0;

const LIQUID_Z: f64 = 250.0;
const KILL_Z: f64 = 446.0;
const SOLID_Z: f64 = 625.0;
const VENT_Z: f64 = 804.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let frame = pod_frame();
    export(&frame, OUTPUTS[0]);

    let containment = secondary_containment_tray();
    export(&containment, OUTPUTS[1]);

    let liquid = liquid_waste_cassette();
    export(&liquid, OUTPUTS[2]);

    let vent = filtered_vent_stack();
    export(&vent, OUTPUTS[3]);

    let kill = contact_time_thermal_placeholder();
    export(&kill, OUTPUTS[4]);

    let sample = neutralization_sample_panel();
    export(&sample, OUTPUTS[5]);

    let solid = solid_waste_passout_drawer();
    export(&solid, OUTPUTS[6]);

    let interlocks = overflow_interlock_panel();
    export(&interlocks, OUTPUTS[7]);

    let drain = drain_routing_manifold();
    export(&drain, OUTPUTS[8]);

    let service = service_access_panels();
    export(&service, OUTPUTS[9]);

    let assembly = frame
        + containment.translate(0.0, -54.0, BASE_PAN_Z + 40.0)
        + liquid.translate(-108.0, -70.0, LIQUID_Z)
        + vent.translate(92.0, POD_Y / 2.0 - 46.0, VENT_Z)
        + kill.translate(-64.0, POD_Y / 2.0 - 104.0, KILL_Z)
        + sample.translate(POD_X / 2.0 + 18.0, 16.0, 444.0)
        + solid.translate(220.0, -96.0, SOLID_Z)
        + interlocks.translate(-(POD_X / 2.0 + 18.0), -8.0, 452.0)
        + drain.translate(0.0, POD_Y / 2.0 - 34.0, 142.0)
        + service;

    export(&assembly, OUTPUTS[10]);

    println!(
        "Waste/decon service pod: {:.0}mm W x {:.0}mm D x {:.0}mm H, {:.0}mm x {:.0}mm liquid cassette, {:.0}mm secondary containment pan, {} filtered vent positions, {} dwell/thermal placeholder lanes, solid pass-out drawer, overflow interlocks, and rear drain manifold.",
        POD_X,
        POD_Y,
        POD_Z,
        LIQUID_CASSETTE_X,
        LIQUID_CASSETTE_Y,
        BASE_PAN_Z,
        VENT_FILTERS,
        DWELL_CHANNELS
    );
    println!(
        "Service clearances modeled: front {:.0}mm pull-out access, rear {:.0}mm hose/filter access. Kill/filter geometry is packaging placeholder only, not validation.",
        SERVICE_CLEARANCE_FRONT, SERVICE_CLEARANCE_REAR
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn pod_frame() -> Part {
    let base_pan = centered_cube(
        "waste_decon_pod_base_secondary_pan_shell",
        POD_X,
        POD_Y,
        BASE_PAN_Z,
    )
    .translate(0.0, 0.0, BASE_PAN_Z / 2.0);
    let pan_cavity = centered_cube(
        "waste_decon_pod_base_pan_cavity",
        POD_X - 96.0,
        POD_Y - 86.0,
        BASE_PAN_Z - 16.0,
    )
    .translate(0.0, -8.0, BASE_PAN_Z / 2.0 + 18.0);
    let leak_sensor_groove = centered_cube(
        "waste_decon_pod_leak_sensor_groove",
        POD_X - 164.0,
        14.0,
        18.0,
    )
    .translate(0.0, -(POD_Y / 2.0 - 48.0), BASE_PAN_Z - 8.0);

    let mut posts = Part::empty("waste_decon_pod_frame_posts");
    for (i, (x, y)) in frame_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("waste_decon_pod_frame_post_{i}"),
                FRAME_W,
                FRAME_W,
                POD_Z,
            )
            .translate(*x, *y, POD_Z / 2.0);
    }

    let mut rails = Part::empty("waste_decon_pod_frame_rails");
    for (i, z) in [
        BASE_PAN_Z + 74.0,
        LIQUID_Z + LIQUID_CASSETTE_Z / 2.0 + 34.0,
        KILL_Z + 92.0,
        SOLID_Z + 134.0,
        POD_Z - 24.0,
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("waste_decon_rear_cross_rail_{i}"),
                POD_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, POD_Y / 2.0 - FRAME_W / 2.0, *z)
            + centered_cube(
                format!("waste_decon_left_side_rail_{i}"),
                FRAME_W,
                POD_Y,
                FRAME_W,
            )
            .translate(-(POD_X / 2.0 - FRAME_W / 2.0), 0.0, *z)
            + centered_cube(
                format!("waste_decon_right_side_rail_{i}"),
                FRAME_W,
                POD_Y,
                FRAME_W,
            )
            .translate(POD_X / 2.0 - FRAME_W / 2.0, 0.0, *z);
    }

    let front_top_rail = centered_cube(
        "waste_decon_front_top_service_rail",
        POD_X,
        FRAME_W,
        FRAME_W,
    )
    .translate(0.0, -(POD_Y / 2.0 - FRAME_W / 2.0), POD_Z - 24.0);

    let floor_datum = centered_cube(
        "waste_decon_pod_workcell_dock_datum_bar",
        POD_X - 180.0,
        22.0,
        34.0,
    )
    .translate(0.0, POD_Y / 2.0 + 8.0, BASE_PAN_Z + 17.0);

    let tow_slots = tow_fork_slots();
    let leveler_feet = leveling_feet();
    let shelf_lips = shelf_lip_set();

    base_pan - pan_cavity - leak_sensor_groove
        + posts
        + rails
        + front_top_rail
        + floor_datum
        + tow_slots
        + leveler_feet
        + shelf_lips
}

fn secondary_containment_tray() -> Part {
    let tray_x = POD_X - 116.0;
    let tray_y = POD_Y - 126.0;
    let tray_z = 84.0;
    let wall = 24.0;

    let outer = centered_cube(
        "waste_decon_secondary_containment_outer",
        tray_x,
        tray_y,
        tray_z,
    );
    let cavity = centered_cube(
        "waste_decon_secondary_containment_cavity",
        tray_x - wall * 2.0,
        tray_y - wall * 2.0,
        tray_z - 18.0,
    )
    .translate(0.0, 0.0, 16.0);
    let sloped_floor_gutter = centered_cube(
        "waste_decon_secondary_containment_floor_gutter",
        tray_x - 122.0,
        20.0,
        18.0,
    )
    .translate(0.0, -(tray_y / 2.0 - 48.0), tray_z / 2.0 - 7.0);
    let corner_sump = centered_cube(
        "waste_decon_secondary_containment_corner_sump",
        118.0,
        82.0,
        24.0,
    )
    .translate(
        tray_x / 2.0 - 84.0,
        -(tray_y / 2.0 - 68.0),
        tray_z / 2.0 - 8.0,
    );
    let sump_drain = centered_cylinder(
        "waste_decon_secondary_containment_sump_drain_bulkhead",
        12.0 / 2.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        tray_x / 2.0 - 84.0,
        -(tray_y / 2.0 + 2.0),
        tray_z / 2.0 - 8.0,
    );

    let leak_strip = centered_cube(
        "waste_decon_secondary_containment_leak_strip_land",
        tray_x - 146.0,
        18.0,
        7.0,
    )
    .translate(0.0, -(tray_y / 2.0 - 34.0), tray_z / 2.0 + 4.0);

    let mut bottle_datums = Part::empty("waste_decon_secondary_containment_bottle_datums");
    for (i, x) in liquid_bottle_x_positions().iter().enumerate() {
        let datum = centered_cylinder(
            format!("waste_decon_secondary_bottle_datum_ring_{i}"),
            bottle_outer_radius(i),
            8.0,
            72,
        )
        .translate(*x, 24.0, tray_z / 2.0 + 4.0);
        let center_cut = centered_cylinder(
            format!("waste_decon_secondary_bottle_datum_center_cut_{i}"),
            bottle_outer_radius(i) - 10.0,
            10.0,
            72,
        )
        .translate(*x, 24.0, tray_z / 2.0 + 4.0);
        bottle_datums = bottle_datums + (datum - center_cut);
    }

    outer - cavity - sloped_floor_gutter - corner_sump - sump_drain
        + leak_strip
        + bottle_datums
        + tray_lift_handles("secondary_containment", tray_x, tray_y, tray_z)
}

fn liquid_waste_cassette() -> Part {
    let tray = centered_cube(
        "waste_decon_liquid_cassette_pullout_tray",
        LIQUID_CASSETTE_X,
        LIQUID_CASSETTE_Y,
        42.0,
    );
    let tray_cavity = centered_cube(
        "waste_decon_liquid_cassette_tray_cavity",
        LIQUID_CASSETTE_X - 38.0,
        LIQUID_CASSETTE_Y - 34.0,
        34.0,
    )
    .translate(0.0, 0.0, 10.0);
    let pull_handle_cut = centered_cube(
        "waste_decon_liquid_cassette_pull_handle_cut",
        132.0,
        18.0,
        46.0,
    )
    .translate(0.0, -(LIQUID_CASSETTE_Y / 2.0 - 9.0), 2.0);

    let primary =
        liquid_bottle_cradle("primary", PRIMARY_BOTTLE_D, 234.0).translate(-126.0, 16.0, 128.0);
    let secondary =
        liquid_bottle_cradle("secondary", SECONDARY_BOTTLE_D, 206.0).translate(120.0, 16.0, 112.0);

    let sensor_mast = liquid_level_sensor_mast().translate(
        -(LIQUID_CASSETTE_X / 2.0 - 44.0),
        LIQUID_CASSETTE_Y / 2.0 - 34.0,
        154.0,
    );
    let load_cell_pockets = load_cell_pockets();
    let tube_comb = cassette_tube_comb().translate(0.0, LIQUID_CASSETTE_Y / 2.0 + 13.0, 70.0);
    let quick_disconnects =
        liquid_quick_disconnect_bank().translate(0.0, LIQUID_CASSETTE_Y / 2.0 + 20.0, 134.0);

    tray - tray_cavity - pull_handle_cut
        + primary
        + secondary
        + sensor_mast
        + load_cell_pockets
        + tube_comb
        + quick_disconnects
        + cassette_slide_features("liquid_cassette", LIQUID_CASSETTE_X, LIQUID_CASSETTE_Y)
}

fn filtered_vent_stack() -> Part {
    let plenum = centered_cube(
        "waste_decon_vent_negative_pressure_plenum",
        VENT_STACK_X,
        VENT_STACK_Y,
        VENT_STACK_Z,
    );
    let plenum_cavity = centered_cube(
        "waste_decon_vent_plenum_cavity",
        VENT_STACK_X - 42.0,
        VENT_STACK_Y + 8.0,
        VENT_STACK_Z - 44.0,
    )
    .translate(0.0, 0.0, 0.0);
    let inlet_header = centered_cylinder("waste_decon_vent_waste_bottle_header", 13.0, 118.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-(VENT_STACK_X / 2.0 - 64.0), -38.0, -40.0);
    let outlet_stub = centered_cylinder("waste_decon_vent_exhaust_stub", 15.0, 98.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(VENT_STACK_X / 2.0 - 58.0, 44.0, 42.0);

    let mut filter_bays = Part::empty("waste_decon_vent_filter_bays");
    for i in 0..VENT_FILTERS {
        let x = vent_filter_x(i);
        let cartridge = centered_cylinder(
            format!("waste_decon_hydrophobic_hepa_filter_placeholder_{i}"),
            34.0,
            96.0,
            56,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -8.0, 20.0);
        let socket = centered_cylinder(
            format!("waste_decon_filter_socket_clearance_{i}"),
            22.0,
            110.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -8.0, 20.0);
        let bay_label = centered_cube(
            format!("waste_decon_vent_filter_label_land_{i}"),
            54.0,
            5.0,
            18.0,
        )
        .translate(x, -(VENT_STACK_Y / 2.0 + 3.0), 76.0);
        filter_bays = filter_bays + (cartridge - socket) + bay_label;
    }

    let condenser_trap = centered_cylinder("waste_decon_vent_condensate_trap_cup", 30.0, 58.0, 48)
        .translate(
            -(VENT_STACK_X / 2.0 - 48.0),
            0.0,
            -(VENT_STACK_Z / 2.0 + 29.0),
        );
    let condensate_drain = centered_cylinder("waste_decon_vent_trap_drain_stub", 5.0, 52.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -(VENT_STACK_X / 2.0 - 48.0),
            -44.0,
            -(VENT_STACK_Z / 2.0 + 44.0),
        );

    plenum - plenum_cavity - inlet_header - outlet_stub
        + filter_bays
        + condenser_trap
        + condensate_drain
        + vent_pressure_sensor_block()
}

fn contact_time_thermal_placeholder() -> Part {
    let enclosure = centered_cube(
        "waste_decon_kill_placeholder_enclosure",
        KILL_MODULE_X,
        KILL_MODULE_Y,
        KILL_MODULE_Z,
    );
    let service_window = centered_cube(
        "waste_decon_kill_placeholder_service_window",
        KILL_MODULE_X - 78.0,
        KILL_MODULE_Y + 8.0,
        KILL_MODULE_Z - 44.0,
    )
    .translate(0.0, 0.0, 4.0);

    let dwell_coil = dwell_serpentine_placeholder();
    let thermal_block = centered_cube(
        "waste_decon_thermal_kill_placeholder_heater_block",
        126.0,
        54.0,
        76.0,
    )
    .translate(KILL_MODULE_X / 2.0 - 96.0, -12.0, 0.0);
    let heater_bore = centered_cylinder(
        "waste_decon_thermal_placeholder_tube_bore",
        8.0 / 2.0,
        142.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(KILL_MODULE_X / 2.0 - 96.0, -12.0, 0.0);

    let contact_time_sensor_pockets = centered_cube(
        "waste_decon_contact_time_temp_sensor_pocket_a",
        18.0,
        KILL_MODULE_Y + 10.0,
        26.0,
    )
    .translate(-(KILL_MODULE_X / 2.0 - 62.0), 0.0, 34.0)
        + centered_cube(
            "waste_decon_contact_time_temp_sensor_pocket_b",
            18.0,
            KILL_MODULE_Y + 10.0,
            26.0,
        )
        .translate(38.0, 0.0, 34.0);

    enclosure - service_window - heater_bore - contact_time_sensor_pockets
        + dwell_coil
        + thermal_block
        + placeholder_label_lands("kill_placeholder", KILL_MODULE_X - 42.0, KILL_MODULE_Z)
}

fn neutralization_sample_panel() -> Part {
    let panel = centered_cube(
        "waste_decon_neutralization_sample_panel_body",
        SAMPLE_PANEL_Y,
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Z,
    );
    let sample_door_cut = centered_cube(
        "waste_decon_sample_panel_access_door_cut",
        SAMPLE_PANEL_Y + 8.0,
        SAMPLE_PANEL_X - 92.0,
        112.0,
    )
    .translate(0.0, -16.0, 12.0);
    let neutralization_cup = centered_cylinder(
        "waste_decon_neutralization_reagent_cup_placeholder",
        32.0,
        46.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -88.0, 76.0);
    let sample_port = centered_cylinder("waste_decon_sample_luer_port_clearance", 6.0, 52.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 2.0, 36.0);
    let neutralization_port = centered_cylinder(
        "waste_decon_neutralization_dose_port_clearance",
        8.0,
        52.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -56.0, -42.0);
    let qc_well = centered_cylinder("waste_decon_qc_sample_vial_well", 18.0, 46.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 76.0, -46.0);

    let drip_lip = centered_cube(
        "waste_decon_sample_panel_drip_lip",
        SAMPLE_PANEL_Y + 16.0,
        SAMPLE_PANEL_X - 72.0,
        14.0,
    )
    .translate(0.0, 0.0, -(SAMPLE_PANEL_Z / 2.0 + 7.0));
    let hinge_knuckles =
        vertical_hinge_knuckles("sample_panel", SAMPLE_PANEL_Z, -SAMPLE_PANEL_X / 2.0 + 24.0);
    let latch_bosses = panel_latch_bosses("sample_panel", SAMPLE_PANEL_X, SAMPLE_PANEL_Z);

    panel - sample_door_cut - sample_port - neutralization_port - qc_well
        + neutralization_cup
        + drip_lip
        + hinge_knuckles
        + latch_bosses
}

fn solid_waste_passout_drawer() -> Part {
    let drawer = centered_cube(
        "waste_decon_solid_passout_drawer_shell",
        SOLID_DRAWER_X,
        SOLID_DRAWER_Y,
        SOLID_DRAWER_Z,
    );
    let cavity = centered_cube(
        "waste_decon_solid_passout_drawer_liner_cavity",
        SOLID_DRAWER_X - 42.0,
        SOLID_DRAWER_Y - 44.0,
        SOLID_DRAWER_Z - 36.0,
    )
    .translate(0.0, 0.0, 18.0);
    let front_pull = centered_cube(
        "waste_decon_solid_passout_front_pull_cut",
        112.0,
        18.0,
        34.0,
    )
    .translate(0.0, -(SOLID_DRAWER_Y / 2.0 - 9.0), 22.0);
    let passout_lid = centered_cube(
        "waste_decon_solid_passout_drop_lid_placeholder",
        SOLID_DRAWER_X - 86.0,
        SOLID_DRAWER_Y - 84.0,
        18.0,
    )
    .translate(0.0, 0.0, SOLID_DRAWER_Z / 2.0 + 9.0);
    let bag_ring = centered_cube(
        "waste_decon_solid_waste_bag_ring",
        SOLID_DRAWER_X - 118.0,
        SOLID_DRAWER_Y - 112.0,
        14.0,
    )
    .translate(0.0, 0.0, SOLID_DRAWER_Z / 2.0 + 26.0);
    let bag_ring_cut = centered_cube(
        "waste_decon_solid_waste_bag_ring_opening",
        SOLID_DRAWER_X - 170.0,
        SOLID_DRAWER_Y - 164.0,
        18.0,
    )
    .translate(0.0, 0.0, SOLID_DRAWER_Z / 2.0 + 26.0);

    let rtp_datum_ring =
        centered_cylinder("waste_decon_solid_passout_rtp_datum_ring", 84.0, 16.0, 72)
            .rotate(90.0, 0.0, 0.0)
            .translate(0.0, SOLID_DRAWER_Y / 2.0 + 8.0, 24.0);
    let rtp_clearance =
        centered_cylinder("waste_decon_solid_passout_rtp_clearance", 58.0, 20.0, 64)
            .rotate(90.0, 0.0, 0.0)
            .translate(0.0, SOLID_DRAWER_Y / 2.0 + 8.0, 24.0);

    let lockout_tabs = centered_cube(
        "waste_decon_solid_drawer_door_interlock_tab_left",
        28.0,
        18.0,
        44.0,
    )
    .translate(
        -(SOLID_DRAWER_X / 2.0 - 34.0),
        -(SOLID_DRAWER_Y / 2.0 + 8.0),
        48.0,
    ) + centered_cube(
        "waste_decon_solid_drawer_door_interlock_tab_right",
        28.0,
        18.0,
        44.0,
    )
    .translate(
        SOLID_DRAWER_X / 2.0 - 34.0,
        -(SOLID_DRAWER_Y / 2.0 + 8.0),
        48.0,
    );

    drawer - cavity - front_pull
        + passout_lid
        + (bag_ring - bag_ring_cut)
        + (rtp_datum_ring - rtp_clearance)
        + lockout_tabs
        + cassette_slide_features("solid_passout", SOLID_DRAWER_X, SOLID_DRAWER_Y)
}

fn overflow_interlock_panel() -> Part {
    let panel = centered_cube(
        "waste_decon_overflow_interlock_panel_body",
        INTERLOCK_PANEL_Y,
        INTERLOCK_PANEL_X,
        INTERLOCK_PANEL_Z,
    );
    let service_cut = centered_cube(
        "waste_decon_overflow_interlock_panel_service_window",
        INTERLOCK_PANEL_Y + 8.0,
        INTERLOCK_PANEL_X - 72.0,
        INTERLOCK_PANEL_Z - 78.0,
    )
    .translate(0.0, 0.0, 6.0);

    let mut sensor_blocks = Part::empty("waste_decon_overflow_sensor_blocks");
    for (i, (name, y, z)) in [
        ("nominal", -72.0, NOMINAL_LIQUID_FILL_Z - 210.0),
        ("high", 0.0, HIGH_LEVEL_SENSOR_Z - 210.0),
        ("high_high", 72.0, HIGH_HIGH_SENSOR_Z - 210.0),
    ]
    .iter()
    .enumerate()
    {
        let block = centered_cube(
            format!("waste_decon_{name}_level_sensor_block_{i}"),
            22.0,
            42.0,
            30.0,
        )
        .translate(0.0, *y, *z);
        let cable_gland = centered_cylinder(
            format!("waste_decon_{name}_sensor_cable_gland_{i}"),
            4.0,
            36.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, *y, *z + 18.0);
        sensor_blocks = sensor_blocks + block - cable_gland;
    }

    let leak_relay = centered_cube(
        "waste_decon_leak_detect_pump_disable_relay_placeholder",
        26.0,
        92.0,
        58.0,
    )
    .translate(0.0, -4.0, -(INTERLOCK_PANEL_Z / 2.0 - 58.0));
    let door_switch = centered_cube(
        "waste_decon_front_door_service_interlock_switch",
        28.0,
        54.0,
        44.0,
    )
    .translate(
        0.0,
        INTERLOCK_PANEL_X / 2.0 - 42.0,
        INTERLOCK_PANEL_Z / 2.0 - 42.0,
    );
    let e_stop_land = centered_cylinder("waste_decon_overflow_e_stop_land", 24.0, 12.0, 40)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            0.0,
            -(INTERLOCK_PANEL_X / 2.0 - 46.0),
            INTERLOCK_PANEL_Z / 2.0 - 52.0,
        );

    panel - service_cut
        + sensor_blocks
        + leak_relay
        + door_switch
        + e_stop_land
        + panel_latch_bosses("overflow_interlock", INTERLOCK_PANEL_X, INTERLOCK_PANEL_Z)
}

fn drain_routing_manifold() -> Part {
    let manifold = centered_cube(
        "waste_decon_drain_routing_manifold_body",
        DRAIN_MANIFOLD_X,
        DRAIN_MANIFOLD_Y,
        DRAIN_MANIFOLD_Z,
    );
    let main_bore = centered_cylinder(
        "waste_decon_drain_manifold_main_bore",
        10.0,
        DRAIN_MANIFOLD_X + 8.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let mut ports = Part::empty("waste_decon_drain_routing_ports");
    for i in 0..DRAIN_PORTS {
        let x = drain_port_x(i);
        let port_cut = centered_cylinder(
            format!("waste_decon_drain_port_clearance_{i}"),
            7.0,
            DRAIN_MANIFOLD_Y + 10.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 15.0);
        let hose_barb = centered_cylinder(
            format!("waste_decon_drain_hose_barb_placeholder_{i}"),
            11.0,
            36.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(DRAIN_MANIFOLD_Y / 2.0 + 18.0), 15.0);
        let label_land = centered_cube(
            format!("waste_decon_drain_port_label_land_{i}"),
            48.0,
            5.0,
            12.0,
        )
        .translate(
            x,
            -(DRAIN_MANIFOLD_Y / 2.0 + 4.0),
            DRAIN_MANIFOLD_Z / 2.0 + 6.0,
        );
        ports = ports + hose_barb + label_land - port_cut;
    }

    let trap_bowl = centered_cylinder("waste_decon_drain_trap_bowl_placeholder", 42.0, 70.0, 48)
        .translate(
            -(DRAIN_MANIFOLD_X / 2.0 - 70.0),
            0.0,
            -(DRAIN_MANIFOLD_Z / 2.0 + 35.0),
        );
    let trap_drain = centered_cylinder("waste_decon_drain_trap_cleanout", 8.0, 52.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -(DRAIN_MANIFOLD_X / 2.0 - 70.0),
            -(DRAIN_MANIFOLD_Y / 2.0 + 14.0),
            -(DRAIN_MANIFOLD_Z / 2.0 + 52.0),
        );
    let isolation_valve_land =
        centered_cube("waste_decon_drain_isolation_valve_land", 84.0, 24.0, 42.0).translate(
            DRAIN_MANIFOLD_X / 2.0 - 72.0,
            0.0,
            38.0,
        );

    manifold - main_bore + ports + trap_bowl + trap_drain + isolation_valve_land
}

fn service_access_panels() -> Part {
    let front_panel = service_panel("front", FRONT_PANEL_X, FRONT_PANEL_Y, FRONT_PANEL_Z, true)
        .translate(0.0, -(POD_Y / 2.0 + FRONT_PANEL_Y / 2.0), 430.0);
    let rear_panel = service_panel("rear", REAR_PANEL_X, REAR_PANEL_Y, REAR_PANEL_Z, false)
        .translate(0.0, POD_Y / 2.0 + REAR_PANEL_Y / 2.0, 454.0);

    let front_clearance = centered_cube(
        "waste_decon_front_service_pullout_clearance_envelope",
        FRONT_PANEL_X - 120.0,
        16.0,
        126.0,
    )
    .translate(0.0, -(POD_Y / 2.0 + 56.0), 176.0);
    let rear_filter_clearance = centered_cube(
        "waste_decon_rear_filter_service_clearance_envelope",
        VENT_STACK_X + 80.0,
        16.0,
        116.0,
    )
    .translate(74.0, POD_Y / 2.0 + 48.0, 770.0);

    front_panel + rear_panel + front_clearance + rear_filter_clearance
}

fn liquid_bottle_cradle(name: &str, bottle_dia: f64, height: f64) -> Part {
    let ring_od = bottle_dia + BOTTLE_CLEARANCE_D * 2.0;
    let base = centered_cube(
        format!("waste_decon_{name}_bottle_cradle_base"),
        ring_od + 36.0,
        ring_od + 26.0,
        14.0,
    )
    .translate(0.0, 0.0, -(height / 2.0 - 7.0));
    let bottom_socket = centered_cylinder(
        format!("waste_decon_{name}_bottle_socket"),
        ring_od / 2.0,
        12.0,
        80,
    )
    .translate(0.0, 0.0, -(height / 2.0 - 20.0));
    let bottle_clearance = centered_cylinder(
        format!("waste_decon_{name}_bottle_clearance"),
        bottle_dia / 2.0,
        16.0,
        80,
    )
    .translate(0.0, 0.0, -(height / 2.0 - 20.0));

    let back_spine = centered_cube(
        format!("waste_decon_{name}_bottle_back_spine"),
        ring_od,
        16.0,
        height,
    )
    .translate(0.0, ring_od / 2.0 - 8.0, 0.0);
    let left_spine = centered_cube(
        format!("waste_decon_{name}_bottle_left_spine"),
        16.0,
        ring_od * 0.78,
        height,
    )
    .translate(-(ring_od / 2.0 - 8.0), 0.0, 0.0);
    let right_spine = centered_cube(
        format!("waste_decon_{name}_bottle_right_spine"),
        16.0,
        ring_od * 0.78,
        height,
    )
    .translate(ring_od / 2.0 - 8.0, 0.0, 0.0);

    let mut strap_slots = Part::empty(format!("waste_decon_{name}_bottle_strap_slots"));
    for (i, z) in [-height * 0.18, height * 0.18].iter().enumerate() {
        strap_slots = strap_slots
            + centered_cube(
                format!("waste_decon_{name}_bottle_strap_slot_{i}"),
                ring_od + 22.0,
                8.0,
                18.0,
            )
            .translate(0.0, ring_od / 2.0 - 10.0, *z);
    }

    base + (bottom_socket - bottle_clearance) + back_spine + left_spine + right_spine - strap_slots
}

fn liquid_level_sensor_mast() -> Part {
    let mast = centered_cube("waste_decon_liquid_level_sensor_mast", 22.0, 18.0, 292.0);
    let scale_land = centered_cube("waste_decon_liquid_level_scale_land", 6.0, 6.0, 270.0)
        .translate(14.0, -10.0, 0.0);

    let nominal = centered_cube("waste_decon_nominal_level_sensor_pocket", 34.0, 16.0, 18.0)
        .translate(0.0, -12.0, NOMINAL_LIQUID_FILL_Z - 170.0);
    let high = centered_cube("waste_decon_high_level_sensor_pocket", 34.0, 16.0, 18.0).translate(
        0.0,
        -12.0,
        HIGH_LEVEL_SENSOR_Z - 170.0,
    );
    let high_high = centered_cube(
        "waste_decon_high_high_level_sensor_pocket",
        34.0,
        16.0,
        18.0,
    )
    .translate(0.0, -12.0, HIGH_HIGH_SENSOR_Z - 170.0);

    let top_cable_gland =
        centered_cylinder("waste_decon_liquid_level_mast_cable_gland", 5.0, 30.0, 18)
            .rotate(90.0, 0.0, 0.0)
            .translate(0.0, -17.0, 134.0);

    mast + scale_land - nominal - high - high_high - top_cable_gland
}

fn load_cell_pockets() -> Part {
    let mut pockets = Part::empty("waste_decon_liquid_load_cell_pockets");
    for (i, x) in liquid_bottle_x_positions().iter().enumerate() {
        let pad = centered_cube(
            format!("waste_decon_bottle_load_cell_pad_{i}"),
            104.0,
            104.0,
            10.0,
        )
        .translate(*x, 16.0, 28.0);
        let pocket = centered_cube(
            format!("waste_decon_bottle_load_cell_relief_{i}"),
            76.0,
            76.0,
            12.0,
        )
        .translate(*x, 16.0, 29.0);
        pockets = pockets + (pad - pocket);
    }
    pockets
}

fn cassette_tube_comb() -> Part {
    let body = centered_cube(
        "waste_decon_liquid_cassette_tube_comb_body",
        220.0,
        20.0,
        24.0,
    );
    let mut tube_channels = Part::empty("waste_decon_liquid_cassette_tube_channels");
    for (i, x) in [-84.0, -42.0, 0.0, 42.0, 84.0].iter().enumerate() {
        tube_channels = tube_channels
            + centered_cylinder(
                format!("waste_decon_liquid_cassette_tube_channel_{i}"),
                6.2 / 2.0,
                26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0)
            + centered_cube(
                format!("waste_decon_liquid_cassette_tube_slot_{i}"),
                7.2,
                28.0,
                16.0,
            )
            .translate(*x, 0.0, 9.0);
    }
    body - tube_channels
}

fn liquid_quick_disconnect_bank() -> Part {
    let manifold = centered_cube(
        "waste_decon_liquid_quick_disconnect_bank",
        244.0,
        24.0,
        70.0,
    );
    let mut ports = Part::empty("waste_decon_liquid_quick_disconnect_ports");
    for (i, x) in [-96.0, -48.0, 0.0, 48.0, 96.0].iter().enumerate() {
        let port = centered_cylinder(
            format!("waste_decon_liquid_qd_port_clearance_{i}"),
            7.0,
            34.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, 8.0);
        let label = centered_cube(
            format!("waste_decon_liquid_qd_label_land_{i}"),
            34.0,
            4.0,
            10.0,
        )
        .translate(*x, -14.0, 36.0);
        ports = ports + label - port;
    }
    manifold + ports
}

fn dwell_serpentine_placeholder() -> Part {
    let mut lanes = Part::empty("waste_decon_contact_time_serpentine_placeholder");
    for i in 0..DWELL_CHANNELS {
        let y = -((DWELL_CHANNELS as f64 - 1.0) * 22.0) / 2.0 + i as f64 * 22.0;
        let channel_land = centered_cube(
            format!("waste_decon_contact_time_channel_land_{i}"),
            KILL_MODULE_X - 184.0,
            10.0,
            14.0,
        )
        .translate(-50.0, y, -24.0);
        let channel_bore = centered_cylinder(
            format!("waste_decon_contact_time_channel_bore_{i}"),
            3.6,
            KILL_MODULE_X - 174.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-50.0, y, -24.0);
        lanes = lanes + (channel_land - channel_bore);
    }

    let mut u_bends = Part::empty("waste_decon_contact_time_u_bend_placeholders");
    for i in 0..(DWELL_CHANNELS - 1) {
        let y = -((DWELL_CHANNELS as f64 - 1.0) * 22.0) / 2.0 + i as f64 * 22.0 + 11.0;
        let x = if i % 2 == 0 {
            -(KILL_MODULE_X / 2.0 - 112.0)
        } else {
            KILL_MODULE_X / 2.0 - 214.0
        };
        u_bends = u_bends
            + centered_cylinder(
                format!("waste_decon_contact_time_u_bend_land_{i}"),
                16.0,
                11.0,
                24,
            )
            .translate(x, y, -24.0);
    }

    lanes + u_bends
}

fn vent_pressure_sensor_block() -> Part {
    let body = centered_cube("waste_decon_vent_pressure_sensor_block", 78.0, 24.0, 46.0).translate(
        VENT_STACK_X / 2.0 - 56.0,
        -(VENT_STACK_Y / 2.0 + 15.0),
        -48.0,
    );
    let tap = centered_cylinder("waste_decon_vent_pressure_tap_clearance", 3.5, 30.0, 18)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            VENT_STACK_X / 2.0 - 56.0,
            -(VENT_STACK_Y / 2.0 + 15.0),
            -48.0,
        );
    body - tap
}

fn service_panel(name: &str, width: f64, depth: f64, height: f64, window: bool) -> Part {
    let panel = centered_cube(
        format!("waste_decon_{name}_service_panel_body"),
        width,
        depth,
        height,
    );
    let pull = centered_cube(
        format!("waste_decon_{name}_service_panel_pull_cut"),
        124.0,
        depth + 8.0,
        24.0,
    )
    .translate(0.0, 0.0, -(height / 2.0 - 36.0));
    let window_cut = if window {
        centered_cube(
            format!("waste_decon_{name}_service_panel_view_window"),
            width - 190.0,
            depth + 8.0,
            height - 186.0,
        )
        .translate(0.0, 0.0, 28.0)
    } else {
        centered_cube(
            format!("waste_decon_{name}_service_panel_filter_access_cut"),
            width - 240.0,
            depth + 8.0,
            height - 260.0,
        )
        .translate(0.0, 0.0, 56.0)
    };

    let mut latch_holes = Part::empty(format!("waste_decon_{name}_service_panel_latch_holes"));
    for (i, x) in [
        -(width / 2.0 - 56.0),
        -(width / 6.0),
        width / 6.0,
        width / 2.0 - 56.0,
    ]
    .iter()
    .enumerate()
    {
        latch_holes = latch_holes
            + centered_cylinder(
                format!("waste_decon_{name}_service_panel_latch_hole_{i}"),
                4.4 / 2.0,
                depth + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, height / 2.0 - 36.0);
    }

    panel - pull - window_cut - latch_holes + horizontal_hinge_knuckles(name, width, depth, height)
}

fn horizontal_hinge_knuckles(name: &str, width: f64, depth: f64, height: f64) -> Part {
    let mut hinges = Part::empty(format!("waste_decon_{name}_horizontal_hinge_knuckles"));
    for (i, x) in [
        -(width / 2.0 - 74.0),
        -(width / 4.0),
        0.0,
        width / 4.0,
        width / 2.0 - 74.0,
    ]
    .iter()
    .enumerate()
    {
        let knuckle = centered_cylinder(
            format!("waste_decon_{name}_hinge_knuckle_{i}"),
            8.0,
            42.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, -(depth / 2.0 + 8.0), height / 2.0 + 8.0);
        hinges = hinges + knuckle;
    }
    hinges
}

fn vertical_hinge_knuckles(name: &str, height: f64, y: f64) -> Part {
    let mut hinges = Part::empty(format!("waste_decon_{name}_vertical_hinge_knuckles"));
    for (i, z) in [-(height / 2.0 - 42.0), 0.0, height / 2.0 - 42.0]
        .iter()
        .enumerate()
    {
        hinges = hinges
            + centered_cylinder(
                format!("waste_decon_{name}_vertical_hinge_knuckle_{i}"),
                7.0,
                42.0,
                22,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, *z);
    }
    hinges
}

fn panel_latch_bosses(name: &str, width_like: f64, height: f64) -> Part {
    let mut bosses = Part::empty(format!("waste_decon_{name}_panel_latch_bosses"));
    for (i, y) in [-(width_like / 2.0 - 30.0), width_like / 2.0 - 30.0]
        .iter()
        .enumerate()
    {
        let boss = centered_cylinder(
            format!("waste_decon_{name}_panel_latch_boss_{i}"),
            10.0,
            12.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, *y, height / 2.0 - 36.0);
        let hole = centered_cylinder(
            format!("waste_decon_{name}_panel_latch_hole_{i}"),
            3.2 / 2.0,
            14.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, *y, height / 2.0 - 36.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn tray_lift_handles(name: &str, width: f64, depth: f64, height: f64) -> Part {
    let left = centered_cube(
        format!("waste_decon_{name}_left_lift_handle"),
        82.0,
        18.0,
        24.0,
    )
    .translate(
        -(width / 2.0 - 86.0),
        -(depth / 2.0 + 8.0),
        height / 2.0 - 6.0,
    );
    let right = centered_cube(
        format!("waste_decon_{name}_right_lift_handle"),
        82.0,
        18.0,
        24.0,
    )
    .translate(width / 2.0 - 86.0, -(depth / 2.0 + 8.0), height / 2.0 - 6.0);
    left + right
}

fn cassette_slide_features(name: &str, width: f64, depth: f64) -> Part {
    let left_slide = centered_cube(
        format!("waste_decon_{name}_left_slide_runner"),
        width - 54.0,
        12.0,
        12.0,
    )
    .translate(0.0, -(depth / 2.0 + 9.0), 25.0);
    let right_slide = centered_cube(
        format!("waste_decon_{name}_right_slide_runner"),
        width - 54.0,
        12.0,
        12.0,
    )
    .translate(0.0, depth / 2.0 + 9.0, 25.0);
    let rear_key = centered_cube(
        format!("waste_decon_{name}_asymmetric_rear_key"),
        54.0,
        18.0,
        20.0,
    )
    .translate(-(width / 2.0 - 64.0), depth / 2.0 + 12.0, 38.0);
    left_slide + right_slide + rear_key
}

fn placeholder_label_lands(name: &str, width: f64, height: f64) -> Part {
    centered_cube(
        format!("waste_decon_{name}_upper_label_land"),
        width,
        5.0,
        12.0,
    )
    .translate(0.0, -(KILL_MODULE_Y / 2.0 + 3.0), height / 2.0 - 18.0)
        + centered_cube(
            format!("waste_decon_{name}_lower_label_land"),
            width,
            5.0,
            12.0,
        )
        .translate(0.0, -(KILL_MODULE_Y / 2.0 + 3.0), -(height / 2.0 - 18.0))
}

fn tow_fork_slots() -> Part {
    let slot_left = centered_cube(
        "waste_decon_left_fork_slot_guard",
        POD_X - 164.0,
        24.0,
        24.0,
    )
    .translate(0.0, -(POD_Y / 2.0 - 64.0), BASE_PAN_Z + 12.0);
    let slot_right = centered_cube(
        "waste_decon_right_fork_slot_guard",
        POD_X - 164.0,
        24.0,
        24.0,
    )
    .translate(0.0, POD_Y / 2.0 - 64.0, BASE_PAN_Z + 12.0);
    slot_left + slot_right
}

fn leveling_feet() -> Part {
    let mut feet = Part::empty("waste_decon_pod_leveling_feet");
    for (i, (x, y)) in [
        (-(POD_X / 2.0 - 54.0), -(POD_Y / 2.0 - 54.0)),
        (POD_X / 2.0 - 54.0, -(POD_Y / 2.0 - 54.0)),
        (-(POD_X / 2.0 - 54.0), POD_Y / 2.0 - 54.0),
        (POD_X / 2.0 - 54.0, POD_Y / 2.0 - 54.0),
        (0.0, -(POD_Y / 2.0 - 54.0)),
        (0.0, POD_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(format!("waste_decon_pod_leveling_pad_{i}"), 24.0, 12.0, 40)
            .translate(*x, *y, -6.0);
        let screw = centered_cylinder(
            format!("waste_decon_pod_leveling_screw_clearance_{i}"),
            8.0 / 2.0,
            18.0,
            24,
        )
        .translate(*x, *y, -6.0);
        feet = feet + (pad - screw);
    }
    feet
}

fn shelf_lip_set() -> Part {
    let liquid_lip = shelf_lip("liquid", LIQUID_CASSETTE_X + 84.0, LIQUID_CASSETTE_Y + 46.0)
        .translate(-84.0, -70.0, LIQUID_Z - LIQUID_CASSETTE_Z / 2.0 + 16.0);
    let kill_lip = shelf_lip(
        "kill_placeholder",
        KILL_MODULE_X + 78.0,
        KILL_MODULE_Y + 44.0,
    )
    .translate(
        -64.0,
        POD_Y / 2.0 - 104.0,
        KILL_Z - KILL_MODULE_Z / 2.0 + 14.0,
    );
    let solid_lip = shelf_lip(
        "solid_passout",
        SOLID_DRAWER_X + 72.0,
        SOLID_DRAWER_Y + 56.0,
    )
    .translate(220.0, -96.0, SOLID_Z - SOLID_DRAWER_Z / 2.0 + 14.0);
    liquid_lip + kill_lip + solid_lip
}

fn shelf_lip(name: &str, width: f64, depth: f64) -> Part {
    let rear = centered_cube(
        format!("waste_decon_{name}_shelf_rear_lip"),
        width,
        16.0,
        28.0,
    )
    .translate(0.0, depth / 2.0 - 8.0, 0.0);
    let left = centered_cube(
        format!("waste_decon_{name}_shelf_left_lip"),
        16.0,
        depth,
        28.0,
    )
    .translate(-(width / 2.0 - 8.0), 0.0, 0.0);
    let right = centered_cube(
        format!("waste_decon_{name}_shelf_right_lip"),
        16.0,
        depth,
        28.0,
    )
    .translate(width / 2.0 - 8.0, 0.0, 0.0);
    rear + left + right
}

fn frame_post_points() -> [(f64, f64); 6] {
    [
        (
            -(POD_X / 2.0 - FRAME_W / 2.0),
            -(POD_Y / 2.0 - FRAME_W / 2.0),
        ),
        (POD_X / 2.0 - FRAME_W / 2.0, -(POD_Y / 2.0 - FRAME_W / 2.0)),
        (-(POD_X / 2.0 - FRAME_W / 2.0), POD_Y / 2.0 - FRAME_W / 2.0),
        (POD_X / 2.0 - FRAME_W / 2.0, POD_Y / 2.0 - FRAME_W / 2.0),
        (0.0, -(POD_Y / 2.0 - FRAME_W / 2.0)),
        (0.0, POD_Y / 2.0 - FRAME_W / 2.0),
    ]
}

fn liquid_bottle_x_positions() -> [f64; 2] {
    [-126.0, 120.0]
}

fn bottle_outer_radius(index: usize) -> f64 {
    let dia = if index == 0 {
        PRIMARY_BOTTLE_D
    } else {
        SECONDARY_BOTTLE_D
    };
    dia / 2.0 + BOTTLE_CLEARANCE_D
}

fn vent_filter_x(index: usize) -> f64 {
    let pitch = 128.0;
    -((VENT_FILTERS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn drain_port_x(index: usize) -> f64 {
    let pitch = 104.0;
    -((DRAIN_PORTS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/waste_decon_service_pod_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn liquid_cassette_fits_inside_pod_and_containment() {
        assert!(LIQUID_CASSETTE_X < POD_X - 180.0);
        assert!(LIQUID_CASSETTE_Y < POD_Y - 190.0);
        assert!(PRIMARY_BOTTLE_D + SECONDARY_BOTTLE_D + 150.0 < LIQUID_CASSETTE_X);
        for x in liquid_bottle_x_positions() {
            assert!(
                x.abs() + PRIMARY_BOTTLE_D / 2.0 + BOTTLE_CLEARANCE_D < LIQUID_CASSETTE_X / 2.0
            );
        }
    }

    #[test]
    fn overflow_sensors_order_above_nominal_fill() {
        assert!(NOMINAL_LIQUID_FILL_Z < HIGH_LEVEL_SENSOR_Z);
        assert!(HIGH_LEVEL_SENSOR_Z < HIGH_HIGH_SENSOR_Z);
        assert!(HIGH_HIGH_SENSOR_Z < LIQUID_CASSETTE_Z);
    }

    #[test]
    fn vent_and_drain_arrays_stay_inside_their_panels() {
        assert!(vent_filter_x(0).abs() + 46.0 < VENT_STACK_X / 2.0);
        assert!(vent_filter_x(VENT_FILTERS - 1).abs() + 46.0 < VENT_STACK_X / 2.0);
        assert!(drain_port_x(0).abs() + 42.0 < DRAIN_MANIFOLD_X / 2.0);
        assert!(drain_port_x(DRAIN_PORTS - 1).abs() + 42.0 < DRAIN_MANIFOLD_X / 2.0);
    }

    #[test]
    fn placeholder_modules_fit_service_envelope() {
        assert!(KILL_MODULE_X < POD_X - 260.0);
        assert!(SOLID_DRAWER_X < POD_X - 340.0);
        assert!(VENT_STACK_X < POD_X - 260.0);
        assert!(SERVICE_CLEARANCE_FRONT >= 450.0);
        assert!(SERVICE_CLEARANCE_REAR >= 200.0);
    }
}
