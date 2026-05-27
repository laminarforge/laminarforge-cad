use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-conditioning degas membrane wetout/leak validation station.
//
// Design intent:
// - Package a bought membrane degasser cartridge inside a closed-system fixture
//   before scaled tissue-chip perfusion.
// - Validate wetout/prime routing, leak and air-ingress witness paths, pressure
//   drop tap locations, bypass/relief routing, bubble breakthrough visibility,
//   sample custody, and disposition gates without opening the sterile boundary.
// - Model mechanical packaging, datums, envelopes, keepouts, and witness features
//   only. This is not a pressure-rated membrane, wetout protocol, biological
//   acceptance criterion, or sterile validation claim.

const OUTPUTS: [&str; 13] = [
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_containment_deck.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_degasser_cartridge_envelope.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_wetout_prime_manifold.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_pressure_tap_bosses.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_optical_bubble_witness_window.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_bypass_relief_witness_route.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_sample_retain_wells.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_waste_flush_capture.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_filtered_vent_holder.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_barcode_coa_custody_lands.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_release_hold_reject_gates.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_evidence_camera_bridge_keepouts.stl",
    "output/closed_media_conditioning_degas_membrane_wetout_leak_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_INTENT_FEATURES: [&str; 17] = [
    "containment_deck",
    "bought_degasser_cartridge_envelope",
    "wetout_prime_manifold",
    "upstream_pressure_tap_boss",
    "downstream_pressure_tap_boss",
    "optical_bubble_witness_window",
    "bypass_relief_witness_route",
    "sample_retain_wells",
    "waste_flush_capture",
    "filtered_vent_holder",
    "barcode_land",
    "coa_land",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 48.0;
const SUMP_X: f64 = 1010.0;
const SUMP_Y: f64 = 585.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 16.0;
const DATUM_BOSSES: usize = 8;

const CARTRIDGE_ENV_X: f64 = 360.0;
const CARTRIDGE_ENV_Y: f64 = 118.0;
const CARTRIDGE_ENV_Z: f64 = 94.0;
const CARTRIDGE_POS: (f64, f64) = (-230.0, 110.0);
const CARTRIDGE_BODY_D: f64 = 66.0;
const CARTRIDGE_BODY_L: f64 = 276.0;
const CARTRIDGE_STRAPS: usize = 4;
const PORT_COUNT: usize = 4;
const PORT_COLLAR_D: f64 = 28.0;
const PORT_PITCH_X: f64 = 132.0;
const PORT_PITCH_Y: f64 = 74.0;

const MANIFOLD_X: f64 = 520.0;
const MANIFOLD_Y: f64 = 160.0;
const MANIFOLD_Z: f64 = 40.0;
const MANIFOLD_POS: (f64, f64) = (230.0, 160.0);
const PRIME_PORTS: usize = 8;
const PRIME_PORT_D: f64 = 13.0;
const PRIME_PORT_PITCH: f64 = 58.0;
const CHECK_VALVE_POCKETS: usize = 6;
const TUBE_CHANNEL_D: f64 = 8.0;

const PRESSURE_BAR_X: f64 = 610.0;
const PRESSURE_BAR_Y: f64 = 90.0;
const PRESSURE_BAR_Z: f64 = 34.0;
const PRESSURE_BAR_POS: (f64, f64) = (40.0, -18.0);
const PRESSURE_TAP_PAIRS: usize = 2;
const TAP_BOSS_D: f64 = 38.0;
const TAP_BORE_D: f64 = 8.2;
const TAP_SPAN_X: f64 = 420.0;
const DELTA_P_WITNESS_RIBS: usize = 5;

const WINDOW_X: f64 = 500.0;
const WINDOW_Y: f64 = 126.0;
const WINDOW_Z: f64 = 28.0;
const WINDOW_POS: (f64, f64) = (250.0, -125.0);
const BUBBLE_WINDOWS: usize = 7;
const WINDOW_D: f64 = 32.0;
const WINDOW_PITCH: f64 = 58.0;
const BACKLIGHT_SLOTS: usize = 7;

const BYPASS_X: f64 = 540.0;
const BYPASS_Y: f64 = 108.0;
const BYPASS_Z: f64 = 34.0;
const BYPASS_POS: (f64, f64) = (-210.0, -142.0);
const BYPASS_VALVE_STATIONS: usize = 3;
const RELIEF_WITNESS_CUPS: usize = 4;
const BYPASS_CHANNELS: usize = 2;

const SAMPLE_BANK_X: f64 = 355.0;
const SAMPLE_BANK_Y: f64 = 158.0;
const SAMPLE_BANK_Z: f64 = 42.0;
const SAMPLE_BANK_POS: (f64, f64) = (395.0, -268.0);
const SAMPLE_RETAIN_WELLS: usize = 8;
const SAMPLE_WELL_D: f64 = 25.0;
const SAMPLE_WELL_PITCH_X: f64 = 40.0;
const SAMPLE_LANE_Y: f64 = 44.0;

const WASTE_X: f64 = 360.0;
const WASTE_Y: f64 = 176.0;
const WASTE_Z: f64 = 48.0;
const WASTE_POS: (f64, f64) = (-382.0, -270.0);
const WASTE_CELLS: usize = 4;
const FLUSH_CELLS: usize = 4;
const WASTE_CELL_X: f64 = 72.0;
const WASTE_CELL_Y: f64 = 54.0;

const VENT_X: f64 = 170.0;
const VENT_Y: f64 = 150.0;
const VENT_Z: f64 = 62.0;
const VENT_POS: (f64, f64) = (-492.0, 228.0);
const FILTER_DISC_D: f64 = 52.0;
const VENT_LUER_PORTS: usize = 3;
const VENT_GUARD_RIBS: usize = 6;

const CUSTODY_X: f64 = 390.0;
const CUSTODY_Y: f64 = 118.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (280.0, 292.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_PADS: usize = 4;

const GATE_X: f64 = 470.0;
const GATE_Y: f64 = 94.0;
const GATE_Z: f64 = 34.0;
const GATE_POS: (f64, f64) = (-80.0, -305.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 850.0;
const CAMERA_BRIDGE_Y: f64 = 42.0;
const CAMERA_BRIDGE_Z: f64 = 172.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (10.0, -35.0);
const CAMERA_MOUNTS: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 8;
const ROBOT_KEEPOUT_X: f64 = 1080.0;
const ROBOT_KEEPOUT_Y: f64 = 86.0;
const ROBOT_KEEPOUT_Z: f64 = 72.0;
const SERVICE_KEEPOUT_X: f64 = 96.0;
const SERVICE_KEEPOUT_Y: f64 = 610.0;
const SERVICE_KEEPOUT_Z: f64 = 92.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 285.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(degasser_cartridge_envelope(), OUTPUTS[1]);
    write_part(wetout_prime_manifold(), OUTPUTS[2]);
    write_part(pressure_tap_bosses(), OUTPUTS[3]);
    write_part(optical_bubble_witness_window(), OUTPUTS[4]);
    write_part(bypass_relief_witness_route(), OUTPUTS[5]);
    write_part(sample_retain_wells(), OUTPUTS[6]);
    write_part(waste_flush_capture(), OUTPUTS[7]);
    write_part(filtered_vent_holder(), OUTPUTS[8]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[9]);
    write_part(release_hold_reject_gates(), OUTPUTS[10]);
    write_part(evidence_camera_bridge_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed degas wetout/leak station: {:.0}mm x {:.0}mm contained deck, bought cartridge envelope {:.0}mm x {:.0}mm x {:.0}mm, {} wetout/prime ports, {} pressure tap pairs.",
        DECK_X,
        DECK_Y,
        CARTRIDGE_ENV_X,
        CARTRIDGE_ENV_Y,
        CARTRIDGE_ENV_Z,
        PRIME_PORTS,
        PRESSURE_TAP_PAIRS
    );
    println!(
        "Witness features: {} bubble windows, {} bypass channels, {} relief witness cups, {} sample retain wells, {} waste/flush cells, {} filtered vent ports.",
        BUBBLE_WINDOWS,
        BYPASS_CHANNELS,
        RELIEF_WITNESS_CUPS,
        SAMPLE_RETAIN_WELLS,
        WASTE_CELLS + FLUSH_CELLS,
        VENT_LUER_PORTS
    );
    println!(
        "Custody and disposition: {} barcode lands, {} COA lands, {} release/hold/reject gates, {} camera mounts, top service clearance {:.0}mm.",
        BARCODE_LANDS,
        COA_LANDS,
        DISPOSITION_GATES,
        CAMERA_MOUNTS,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_degas_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube("closed_degas_station_sump_relief", SUMP_X, SUMP_Y, SUMP_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0 - 2.2,
    );
    let drain = centered_cylinder(
        "closed_degas_station_captured_drain",
        DRAIN_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 70.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_degas_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_z());
    let right = centered_cube(
        "closed_degas_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());
    let rear = centered_cube(
        "closed_degas_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let front = centered_cube(
        "closed_degas_station_front_low_containment_rim",
        DECK_X - 110.0,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_degas_station_deck_datums");
    for (i, (x, y)) in [
        (-520.0, -320.0),
        (520.0, -320.0),
        (-520.0, 320.0),
        (520.0, 320.0),
        (-160.0, -320.0),
        (160.0, -320.0),
        (-160.0, 320.0),
        (160.0, 320.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(format!("closed_degas_station_datum_pad_{i}"), 13.0, 5.0, 40)
            .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(format!("closed_degas_station_datum_bore_{i}"), 3.2, 8.0, 24)
            .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }
    datums
}

fn station_landing_pockets() -> Part {
    let cartridge = centered_cube(
        "closed_degas_station_cartridge_landing_recess",
        CARTRIDGE_ENV_X + 34.0,
        CARTRIDGE_ENV_Y + 32.0,
        5.0,
    )
    .translate(CARTRIDGE_POS.0, CARTRIDGE_POS.1, DECK_Z / 2.0 - 1.8);
    let manifold = centered_cube(
        "closed_degas_station_manifold_landing_recess",
        MANIFOLD_X + 30.0,
        MANIFOLD_Y + 28.0,
        5.0,
    )
    .translate(MANIFOLD_POS.0, MANIFOLD_POS.1, DECK_Z / 2.0 - 1.8);
    let window = centered_cube(
        "closed_degas_station_window_landing_recess",
        WINDOW_X + 26.0,
        WINDOW_Y + 24.0,
        5.0,
    )
    .translate(WINDOW_POS.0, WINDOW_POS.1, DECK_Z / 2.0 - 1.8);

    Part::empty("closed_degas_station_landing_pockets") - cartridge - manifold - window
}

fn degasser_cartridge_envelope() -> Part {
    let tray = centered_cube(
        "closed_degas_station_cartridge_cradle",
        CARTRIDGE_ENV_X,
        CARTRIDGE_ENV_Y,
        26.0,
    );
    let body_clearance = centered_cylinder(
        "closed_degas_station_bought_degasser_body_clearance",
        CARTRIDGE_BODY_D / 2.0,
        CARTRIDGE_BODY_L,
        56,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 16.0);
    let envelope = centered_cube(
        "closed_degas_station_bought_degasser_keepin_envelope",
        CARTRIDGE_BODY_L + 44.0,
        CARTRIDGE_BODY_D + 30.0,
        CARTRIDGE_ENV_Z,
    )
    .translate(0.0, 0.0, CARTRIDGE_ENV_Z / 2.0);

    tray - body_clearance + envelope_lattice(envelope) + cartridge_straps() + cartridge_ports()
}

fn envelope_lattice(envelope: Part) -> Part {
    let hollow = centered_cube(
        "closed_degas_station_bought_degasser_keepin_hollow",
        CARTRIDGE_BODY_L + 18.0,
        CARTRIDGE_BODY_D + 4.0,
        CARTRIDGE_ENV_Z + 4.0,
    )
    .translate(0.0, 0.0, CARTRIDGE_ENV_Z / 2.0);
    let window_a = centered_cube(
        "closed_degas_station_cartridge_side_read_window_a",
        214.0,
        14.0,
        54.0,
    )
    .translate(
        0.0,
        -(CARTRIDGE_BODY_D / 2.0 + 17.0),
        CARTRIDGE_ENV_Z / 2.0 + 3.0,
    );
    let window_b = centered_cube(
        "closed_degas_station_cartridge_side_read_window_b",
        214.0,
        14.0,
        54.0,
    )
    .translate(
        0.0,
        CARTRIDGE_BODY_D / 2.0 + 17.0,
        CARTRIDGE_ENV_Z / 2.0 + 3.0,
    );

    envelope - hollow - window_a - window_b
}

fn cartridge_straps() -> Part {
    let mut straps = Part::empty("closed_degas_station_cartridge_straps");
    for i in 0..CARTRIDGE_STRAPS {
        let x = -CARTRIDGE_BODY_L / 2.0 + 42.0 + i as f64 * (CARTRIDGE_BODY_L - 84.0) / 3.0;
        let strap = centered_cube(
            format!("closed_degas_station_cartridge_retainer_strap_{i}"),
            18.0,
            CARTRIDGE_BODY_D + 54.0,
            16.0,
        )
        .translate(x, 0.0, 54.0);
        let clearance = centered_cylinder(
            format!("closed_degas_station_cartridge_retainer_strap_clearance_{i}"),
            CARTRIDGE_BODY_D / 2.0 + 3.0,
            24.0,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 0.0, 54.0);
        straps = straps + (strap - clearance);
    }
    straps
}

fn cartridge_ports() -> Part {
    let mut ports = Part::empty("closed_degas_station_cartridge_port_collars");
    for i in 0..PORT_COUNT {
        let side_x = if i < 2 {
            -PORT_PITCH_X / 2.0
        } else {
            PORT_PITCH_X / 2.0
        };
        let side_y = if i % 2 == 0 {
            -PORT_PITCH_Y / 2.0
        } else {
            PORT_PITCH_Y / 2.0
        };
        let collar = centered_cylinder(
            format!("closed_degas_station_cartridge_port_collar_{i}"),
            PORT_COLLAR_D / 2.0,
            14.0,
            36,
        )
        .translate(side_x, side_y, 18.0);
        let bore = centered_cylinder(
            format!("closed_degas_station_cartridge_port_bore_{i}"),
            TUBE_CHANNEL_D / 2.0,
            18.0,
            24,
        )
        .translate(side_x, side_y, 18.0);
        ports = ports + (collar - bore);
    }
    ports
}

fn wetout_prime_manifold() -> Part {
    let block = centered_cube(
        "closed_degas_station_wetout_prime_manifold_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    block - manifold_channels() - prime_port_bores() + prime_port_collars() + check_valve_pockets()
}

fn manifold_channels() -> Part {
    let main = centered_cylinder(
        "closed_degas_station_wetout_prime_main_channel",
        TUBE_CHANNEL_D / 2.0,
        MANIFOLD_X - 58.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 4.0);
    let return_leg = centered_cylinder(
        "closed_degas_station_prime_return_channel",
        TUBE_CHANNEL_D / 2.0,
        MANIFOLD_X - 116.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -48.0, 4.0);
    let wetout_leg = centered_cylinder(
        "closed_degas_station_wetout_branch_channel",
        TUBE_CHANNEL_D / 2.0,
        MANIFOLD_X - 116.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 48.0, 4.0);

    main + return_leg + wetout_leg
}

fn prime_port_bores() -> Part {
    let mut bores = Part::empty("closed_degas_station_prime_port_bores");
    for i in 0..PRIME_PORTS {
        let x = -prime_port_span() / 2.0 + i as f64 * PRIME_PORT_PITCH;
        let bore = centered_cylinder(
            format!("closed_degas_station_prime_port_bore_{i}"),
            PRIME_PORT_D / 2.0,
            MANIFOLD_Z + 8.0,
            28,
        )
        .translate(x, if i % 2 == 0 { 48.0 } else { -48.0 }, 0.0);
        bores = bores + bore;
    }
    bores
}

fn prime_port_collars() -> Part {
    let mut collars = Part::empty("closed_degas_station_prime_port_collars");
    for i in 0..PRIME_PORTS {
        let x = -prime_port_span() / 2.0 + i as f64 * PRIME_PORT_PITCH;
        let collar = centered_cylinder(
            format!("closed_degas_station_prime_port_label_collar_{i}"),
            17.0,
            7.0,
            32,
        )
        .translate(
            x,
            if i % 2 == 0 { 48.0 } else { -48.0 },
            MANIFOLD_Z / 2.0 + 3.5,
        );
        collars = collars + collar;
    }
    collars
}

fn check_valve_pockets() -> Part {
    let mut pockets = Part::empty("closed_degas_station_check_valve_pockets");
    for i in 0..CHECK_VALVE_POCKETS {
        let x = -145.0 + i as f64 * 58.0;
        pockets = pockets
            - centered_cube(
                format!("closed_degas_station_check_valve_pocket_{i}"),
                34.0,
                32.0,
                11.0,
            )
            .translate(x, 0.0, MANIFOLD_Z / 2.0 - 3.5);
    }
    pockets
}

fn pressure_tap_bosses() -> Part {
    let bar = centered_cube(
        "closed_degas_station_pressure_drop_tap_bar",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    );
    bar + tap_bosses() - tap_bores() + delta_p_witness_ribs()
}

fn tap_bosses() -> Part {
    let mut bosses = Part::empty("closed_degas_station_pressure_tap_bosses");
    for i in 0..PRESSURE_TAP_PAIRS * 2 {
        let x = if i < 2 {
            -TAP_SPAN_X / 2.0
        } else {
            TAP_SPAN_X / 2.0
        };
        let y = if i % 2 == 0 { -24.0 } else { 24.0 };
        bosses = bosses
            + centered_cylinder(
                format!("closed_degas_station_pressure_tap_boss_{i}"),
                TAP_BOSS_D / 2.0,
                18.0,
                40,
            )
            .translate(x, y, PRESSURE_BAR_Z / 2.0 + 9.0);
    }
    bosses
}

fn tap_bores() -> Part {
    let mut bores = Part::empty("closed_degas_station_pressure_tap_bores");
    for i in 0..PRESSURE_TAP_PAIRS * 2 {
        let x = if i < 2 {
            -TAP_SPAN_X / 2.0
        } else {
            TAP_SPAN_X / 2.0
        };
        let y = if i % 2 == 0 { -24.0 } else { 24.0 };
        bores = bores
            + centered_cylinder(
                format!("closed_degas_station_pressure_tap_bore_{i}"),
                TAP_BORE_D / 2.0,
                PRESSURE_BAR_Z + 28.0,
                24,
            )
            .translate(x, y, PRESSURE_BAR_Z / 2.0);
    }
    bores
}

fn delta_p_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_degas_station_delta_p_witness_ribs");
    for i in 0..DELTA_P_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("closed_degas_station_delta_p_rib_{i}"),
                16.0,
                PRESSURE_BAR_Y + 16.0,
                13.0,
            )
            .translate(-112.0 + i as f64 * 56.0, 0.0, PRESSURE_BAR_Z / 2.0 + 6.5);
    }
    ribs
}

fn optical_bubble_witness_window() -> Part {
    let frame = centered_cube(
        "closed_degas_station_optical_bubble_window_frame",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    frame - bubble_window_bores() - backlight_slots() + window_bezel_ribs()
}

fn bubble_window_bores() -> Part {
    let mut bores = Part::empty("closed_degas_station_bubble_window_bores");
    for i in 0..BUBBLE_WINDOWS {
        let x = -window_span() / 2.0 + i as f64 * WINDOW_PITCH;
        bores = bores
            + centered_cylinder(
                format!("closed_degas_station_bubble_breakthrough_window_{i}"),
                WINDOW_D / 2.0,
                WINDOW_Z + 8.0,
                44,
            )
            .translate(x, 0.0, 0.0);
    }
    bores
}

fn backlight_slots() -> Part {
    let mut slots = Part::empty("closed_degas_station_backlight_slots");
    for i in 0..BACKLIGHT_SLOTS {
        let x = -window_span() / 2.0 + i as f64 * WINDOW_PITCH;
        slots = slots
            + centered_cube(
                format!("closed_degas_station_backlight_slot_{i}"),
                34.0,
                18.0,
                WINDOW_Z + 10.0,
            )
            .translate(x, -42.0, 0.0);
    }
    slots
}

fn window_bezel_ribs() -> Part {
    let mut ribs = Part::empty("closed_degas_station_window_bezel_ribs");
    for i in 0..BUBBLE_WINDOWS {
        let x = -window_span() / 2.0 + i as f64 * WINDOW_PITCH;
        ribs = ribs
            + centered_cylinder(
                format!("closed_degas_station_bubble_window_bezel_{i}"),
                WINDOW_D / 2.0 + 5.0,
                4.0,
                44,
            )
            .translate(x, 0.0, WINDOW_Z / 2.0 + 2.0);
    }
    ribs
}

fn bypass_relief_witness_route() -> Part {
    let plate = centered_cube(
        "closed_degas_station_bypass_relief_witness_plate",
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    plate - bypass_channels() - relief_cup_recesses() + bypass_valve_stations() + relief_cup_lips()
}

fn bypass_channels() -> Part {
    let mut channels = Part::empty("closed_degas_station_bypass_channels");
    for i in 0..BYPASS_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("closed_degas_station_bypass_witness_channel_{i}"),
                TUBE_CHANNEL_D / 2.0,
                BYPASS_X - 64.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, if i == 0 { -22.0 } else { 22.0 }, 4.0);
    }
    channels
}

fn relief_cup_recesses() -> Part {
    let mut cups = Part::empty("closed_degas_station_relief_cup_recesses");
    for i in 0..RELIEF_WITNESS_CUPS {
        cups = cups
            + centered_cylinder(
                format!("closed_degas_station_relief_witness_cup_recess_{i}"),
                20.0,
                18.0,
                40,
            )
            .translate(-168.0 + i as f64 * 112.0, 0.0, BYPASS_Z / 2.0 - 5.0);
    }
    cups
}

fn bypass_valve_stations() -> Part {
    let mut stations = Part::empty("closed_degas_station_bypass_valve_stations");
    for i in 0..BYPASS_VALVE_STATIONS {
        stations = stations
            + centered_cube(
                format!("closed_degas_station_bypass_valve_station_{i}"),
                56.0,
                62.0,
                18.0,
            )
            .translate(-120.0 + i as f64 * 120.0, 0.0, BYPASS_Z / 2.0 + 9.0);
    }
    stations
}

fn relief_cup_lips() -> Part {
    let mut lips = Part::empty("closed_degas_station_relief_cup_lips");
    for i in 0..RELIEF_WITNESS_CUPS {
        lips = lips
            + centered_cylinder(
                format!("closed_degas_station_relief_witness_cup_lip_{i}"),
                24.0,
                5.0,
                40,
            )
            .translate(-168.0 + i as f64 * 112.0, 0.0, BYPASS_Z / 2.0 + 2.5);
    }
    lips
}

fn sample_retain_wells() -> Part {
    let bank = centered_cube(
        "closed_degas_station_sample_retain_bank",
        SAMPLE_BANK_X,
        SAMPLE_BANK_Y,
        SAMPLE_BANK_Z,
    );
    bank - sample_well_recesses() + sample_well_lips() + sample_custody_ticks()
}

fn sample_well_recesses() -> Part {
    let mut wells = Part::empty("closed_degas_station_sample_retain_well_recesses");
    for i in 0..SAMPLE_RETAIN_WELLS {
        let (x, y) = sample_well_center(i);
        wells = wells
            + centered_cylinder(
                format!("closed_degas_station_sample_retain_well_recess_{i}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_BANK_Z + 6.0,
                36,
            )
            .translate(x, y, SAMPLE_BANK_Z / 2.0 - 8.0);
    }
    wells
}

fn sample_well_lips() -> Part {
    let mut lips = Part::empty("closed_degas_station_sample_retain_well_lips");
    for i in 0..SAMPLE_RETAIN_WELLS {
        let (x, y) = sample_well_center(i);
        lips = lips
            + centered_cylinder(
                format!("closed_degas_station_sample_retain_well_lip_{i}"),
                SAMPLE_WELL_D / 2.0 + 4.0,
                5.0,
                36,
            )
            .translate(x, y, SAMPLE_BANK_Z / 2.0 + 2.5);
    }
    lips
}

fn sample_custody_ticks() -> Part {
    let mut ticks = Part::empty("closed_degas_station_sample_custody_ticks");
    for i in 0..SAMPLE_RETAIN_WELLS {
        let (x, y) = sample_well_center(i);
        ticks = ticks
            + centered_cube(
                format!("closed_degas_station_sample_custody_tick_{i}"),
                18.0,
                4.0,
                4.0,
            )
            .translate(x, y + 24.0, SAMPLE_BANK_Z / 2.0 + 2.0);
    }
    ticks
}

fn waste_flush_capture() -> Part {
    let tray = centered_cube(
        "closed_degas_station_waste_flush_capture_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    tray - waste_flush_recesses() - waste_tray_drain() + waste_cell_lips()
}

fn waste_flush_recesses() -> Part {
    let mut recesses = Part::empty("closed_degas_station_waste_flush_recesses");
    for i in 0..WASTE_CELLS + FLUSH_CELLS {
        let col = i % 4;
        let row = i / 4;
        let x = -126.0 + col as f64 * 84.0;
        let y = if row == 0 { -38.0 } else { 38.0 };
        recesses = recesses
            + centered_cube(
                format!("closed_degas_station_waste_flush_capture_cell_{i}"),
                WASTE_CELL_X,
                WASTE_CELL_Y,
                WASTE_Z - 12.0,
            )
            .translate(x, y, WASTE_Z / 2.0 - 6.0);
    }
    recesses
}

fn waste_cell_lips() -> Part {
    let mut lips = Part::empty("closed_degas_station_waste_flush_cell_lips");
    for i in 0..WASTE_CELLS + FLUSH_CELLS {
        let col = i % 4;
        let row = i / 4;
        let x = -126.0 + col as f64 * 84.0;
        let y = if row == 0 { -38.0 } else { 38.0 };
        lips = lips
            + centered_cube(
                format!("closed_degas_station_waste_flush_cell_lip_{i}"),
                WASTE_CELL_X + 8.0,
                WASTE_CELL_Y + 8.0,
                5.0,
            )
            .translate(x, y, WASTE_Z / 2.0 + 2.5);
    }
    lips
}

fn waste_tray_drain() -> Part {
    centered_cylinder("closed_degas_station_waste_flush_drain", 6.0, 34.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(WASTE_X / 2.0 - 38.0, -(WASTE_Y / 2.0 + 10.0), -2.0)
}

fn filtered_vent_holder() -> Part {
    let base = centered_cube(
        "closed_degas_station_filtered_vent_holder_base",
        VENT_X,
        VENT_Y,
        VENT_Z,
    );
    base - filter_disc_recess() - vent_luer_bores() + vent_guard_ribs()
}

fn filter_disc_recess() -> Part {
    centered_cylinder(
        "closed_degas_station_hydrophobic_filter_disc_recess",
        FILTER_DISC_D / 2.0,
        VENT_Z + 8.0,
        56,
    )
    .translate(0.0, 0.0, VENT_Z / 2.0 - 10.0)
}

fn vent_luer_bores() -> Part {
    let mut bores = Part::empty("closed_degas_station_filtered_vent_luer_bores");
    for i in 0..VENT_LUER_PORTS {
        bores = bores
            + centered_cylinder(
                format!("closed_degas_station_filtered_vent_luer_bore_{i}"),
                5.4,
                VENT_Z + 10.0,
                24,
            )
            .translate(-42.0 + i as f64 * 42.0, -48.0, 0.0);
    }
    bores
}

fn vent_guard_ribs() -> Part {
    let mut ribs = Part::empty("closed_degas_station_filtered_vent_guard_ribs");
    for i in 0..VENT_GUARD_RIBS {
        let angle = i as f64 * 60.0;
        ribs = ribs
            + centered_cube(
                format!("closed_degas_station_filtered_vent_guard_rib_{i}"),
                FILTER_DISC_D + 18.0,
                5.0,
                8.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(0.0, 0.0, VENT_Z / 2.0 + 4.0);
    }
    ribs
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_degas_station_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel + barcode_lands() + coa_lands() + tamper_seal_pads()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_degas_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_degas_station_barcode_land_{i}"),
                48.0,
                22.0,
                3.0,
            )
            .translate(-145.0 + i as f64 * 58.0, -28.0, CUSTODY_Z / 2.0 + 1.5);
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("closed_degas_station_coa_custody_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_degas_station_coa_card_land_{i}"),
                92.0,
                34.0,
                4.0,
            )
            .translate(-104.0 + i as f64 * 104.0, 30.0, CUSTODY_Z / 2.0 + 2.0);
    }
    lands
}

fn tamper_seal_pads() -> Part {
    let mut pads = Part::empty("closed_degas_station_tamper_seal_pads");
    for i in 0..TAMPER_SEAL_PADS {
        pads = pads
            + centered_cylinder(
                format!("closed_degas_station_tamper_seal_pad_{i}"),
                9.0,
                4.0,
                28,
            )
            .translate(-168.0 + i as f64 * 112.0, 55.0, CUSTODY_Z / 2.0 + 2.0);
    }
    pads
}

fn release_hold_reject_gates() -> Part {
    let bank = centered_cube(
        "closed_degas_station_release_hold_reject_gate_bank",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    bank - gate_lane_recesses() + gate_flags() + gate_token_slots()
}

fn gate_lane_recesses() -> Part {
    let mut lanes = Part::empty("closed_degas_station_gate_lane_recesses");
    for i in 0..DISPOSITION_GATES {
        lanes = lanes
            + centered_cube(
                format!("closed_degas_station_gate_lane_recess_{i}"),
                122.0,
                54.0,
                12.0,
            )
            .translate(-154.0 + i as f64 * 154.0, 0.0, GATE_Z / 2.0 - 4.0);
    }
    lanes
}

fn gate_flags() -> Part {
    let mut flags = Part::empty("closed_degas_station_disposition_gate_flags");
    for i in 0..DISPOSITION_GATES {
        flags = flags
            + centered_cube(
                format!("closed_degas_station_disposition_gate_flag_{i}"),
                26.0,
                66.0,
                38.0,
            )
            .translate(-154.0 + i as f64 * 154.0, 0.0, GATE_Z / 2.0 + 19.0);
    }
    flags
}

fn gate_token_slots() -> Part {
    let mut slots = Part::empty("closed_degas_station_disposition_gate_token_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("closed_degas_station_disposition_token_slot_{i}"),
                34.0,
                18.0,
                6.0,
            )
            .translate(-190.0 + i as f64 * 76.0, 39.0, GATE_Z / 2.0 + 3.0);
    }
    slots
}

fn evidence_camera_bridge_keepouts() -> Part {
    camera_bridge() + evidence_fiducials() + robot_keepout_gauges()
}

fn camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_degas_station_camera_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_degas_station_camera_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "closed_degas_station_camera_bridge_beam",
        CAMERA_BRIDGE_X + 34.0,
        CAMERA_BRIDGE_Y,
        24.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z + 12.0);

    left_post + right_post + beam + camera_mounts()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_degas_station_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("closed_degas_station_camera_mount_plate_{i}"),
                64.0,
                10.0,
                36.0,
            )
            .translate(
                -300.0 + i as f64 * 200.0,
                -CAMERA_BRIDGE_Y / 2.0 - 6.0,
                122.0,
            );
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_degas_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = if i < 4 {
            -450.0 + i as f64 * 300.0
        } else {
            -450.0 + (i - 4) as f64 * 300.0
        };
        let y = if i < 4 { -342.0 } else { 342.0 };
        let disk = centered_cylinder(
            format!("closed_degas_station_evidence_fiducial_disk_{i}"),
            9.0,
            3.0,
            36,
        )
        .translate(x, y, 3.0);
        let center = centered_cylinder(
            format!("closed_degas_station_evidence_fiducial_bore_{i}"),
            2.2,
            5.0,
            20,
        )
        .translate(x, y, 3.0);
        fiducials = fiducials + (disk - center);
    }
    fiducials
}

fn robot_keepout_gauges() -> Part {
    let robot = centered_cube(
        "closed_degas_station_robot_keepout_gauge_front",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + ROBOT_KEEPOUT_Y / 2.0),
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let service_left = centered_cube(
        "closed_degas_station_service_keepout_left",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -(DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0),
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "closed_degas_station_service_keepout_right",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top = centered_cube(
        "closed_degas_station_top_service_clearance_gauge",
        640.0,
        420.0,
        8.0,
    )
    .translate(30.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    robot + service_left + service_right + top
}

fn station_assembly() -> Part {
    containment_deck()
        + degasser_cartridge_envelope().translate(
            CARTRIDGE_POS.0,
            CARTRIDGE_POS.1,
            top_z(CARTRIDGE_ENV_Z),
        )
        + wetout_prime_manifold().translate(MANIFOLD_POS.0, MANIFOLD_POS.1, top_z(MANIFOLD_Z))
        + pressure_tap_bosses().translate(
            PRESSURE_BAR_POS.0,
            PRESSURE_BAR_POS.1,
            top_z(PRESSURE_BAR_Z),
        )
        + optical_bubble_witness_window().translate(WINDOW_POS.0, WINDOW_POS.1, top_z(WINDOW_Z))
        + bypass_relief_witness_route().translate(BYPASS_POS.0, BYPASS_POS.1, top_z(BYPASS_Z))
        + sample_retain_wells().translate(
            SAMPLE_BANK_POS.0,
            SAMPLE_BANK_POS.1,
            top_z(SAMPLE_BANK_Z),
        )
        + waste_flush_capture().translate(WASTE_POS.0, WASTE_POS.1, top_z(WASTE_Z))
        + filtered_vent_holder().translate(VENT_POS.0, VENT_POS.1, top_z(VENT_Z))
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + evidence_camera_bridge_keepouts().translate(
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0,
        )
}

fn sample_well_center(index: usize) -> (f64, f64) {
    let col = index % 4;
    let row = index / 4;
    (
        -1.5 * SAMPLE_WELL_PITCH_X + col as f64 * SAMPLE_WELL_PITCH_X,
        -SAMPLE_LANE_Y / 2.0 + row as f64 * SAMPLE_LANE_Y,
    )
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn prime_port_span() -> f64 {
    (PRIME_PORTS - 1) as f64 * PRIME_PORT_PITCH
}

fn window_span() -> f64 {
    (BUBBLE_WINDOWS - 1) as f64 * WINDOW_PITCH
}

fn assert_layout() {
    assert_eq!(DATUM_BOSSES, 8);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert_eq!(SAMPLE_RETAIN_WELLS, 8);
    assert!(CARTRIDGE_POS.0 - CARTRIDGE_ENV_X / 2.0 > -DECK_X / 2.0 + RIM_W);
    assert!(MANIFOLD_POS.0 + MANIFOLD_X / 2.0 < DECK_X / 2.0 - RIM_W);
    assert!(CUSTODY_POS.1 + CUSTODY_Y / 2.0 < DECK_Y / 2.0 - RIM_W);
    assert!(SAMPLE_BANK_POS.1 - SAMPLE_BANK_Y / 2.0 > -DECK_Y / 2.0 + RIM_W);
    assert!(WASTE_POS.0 - WASTE_X / 2.0 > -DECK_X / 2.0 + RIM_W);
    assert!(prime_port_span() + PRIME_PORT_D < MANIFOLD_X - 40.0);
    assert!(window_span() + WINDOW_D < WINDOW_X - 40.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_media_conditioning_degas_membrane_wetout_leak_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_validation_intent_is_explicit() {
        assert!(REQUIRED_INTENT_FEATURES.contains(&"bought_degasser_cartridge_envelope"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"wetout_prime_manifold"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"upstream_pressure_tap_boss"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"downstream_pressure_tap_boss"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"optical_bubble_witness_window"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_INTENT_FEATURES.contains(&"reject_gate"));
    }

    #[test]
    fn wetout_leak_station_has_redundant_witness_capacity() {
        assert!(PRIME_PORTS >= PORT_COUNT * 2);
        assert_eq!(PRESSURE_TAP_PAIRS * 2, 4);
        assert!(BUBBLE_WINDOWS > PRESSURE_TAP_PAIRS * 2);
        assert!(RELIEF_WITNESS_CUPS >= BYPASS_CHANNELS * 2);
    }

    #[test]
    fn custody_and_disposition_counts_are_physical() {
        assert!(BARCODE_LANDS >= SAMPLE_RETAIN_WELLS / 2);
        assert_eq!(COA_LANDS, DISPOSITION_GATES);
        assert_eq!(DISPOSITION_GATES, 3);
        assert!(GATE_TOKEN_SLOTS >= DISPOSITION_GATES * 2);
    }

    #[test]
    fn helper_geometry_is_centered_and_in_bounds() {
        assert!((sample_well_center(0).0 + sample_well_center(3).0).abs() < 0.001);
        assert!((sample_well_center(0).1 + sample_well_center(4).1).abs() < 0.001);
        assert!(prime_port_span() < MANIFOLD_X - 60.0);
        assert!(window_span() < WINDOW_X - 60.0);
        assert_layout();
    }
}
