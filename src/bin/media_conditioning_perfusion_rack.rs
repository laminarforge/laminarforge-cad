use vcad::{centered_cube, centered_cylinder, Part};

// Media conditioning and perfusion pump utility rack for the closed culture stack.
//
// Intent:
// - Keep media bags/reservoirs, warming, bubble removal, pumps, valves, sterile
//   filters, relief routing, and waste outside the sterile process volume.
// - Present a compact service rack that docks to the culture module service skid
//   through explicit media/waste/drain bulkheads instead of loose bench tubing.
// - Make consumable access front-facing: pull-out media tray, pump cassette bay,
//   waste tray, removable filter/valve panel, and visible bubble-trap placeholders.
//
// This is an architecture and packaging CAD model. Material selection,
// sterilization, valve/filter selection, flow calibration, and biological
// validation remain separate gates.

const OUTPUTS: &[&str] = &[
    "output/media_conditioning_perfusion_rack_frame.stl",
    "output/media_conditioning_perfusion_rack_reservoir_tray.stl",
    "output/media_conditioning_perfusion_rack_conditioning_block.stl",
    "output/media_conditioning_perfusion_rack_degasser_bank.stl",
    "output/media_conditioning_perfusion_rack_pump_bay.stl",
    "output/media_conditioning_perfusion_rack_valve_filter_manifold.stl",
    "output/media_conditioning_perfusion_rack_waste_tray.stl",
    "output/media_conditioning_perfusion_rack_service_panel.stl",
    "output/media_conditioning_perfusion_rack_assembly.stl",
];

const RACK_X: f64 = 820.0;
const RACK_Y: f64 = 460.0;
const RACK_Z: f64 = 720.0;
const FRAME_W: f64 = 30.0;
const BASE_PAN_Z: f64 = 46.0;
const SHELF_Z: f64 = 16.0;
const DECK_CLEARANCE_Z: f64 = 7.0;

const RESERVOIR_TRAY_X: f64 = 690.0;
const RESERVOIR_TRAY_Y: f64 = 154.0;
const RESERVOIR_TRAY_Z: f64 = 22.0;
const MEDIA_BAG_LAND_X: f64 = 178.0;
const MEDIA_BAG_LAND_Y: f64 = 98.0;
const MEDIA_BAG_COUNT: usize = 3;
const BOTTLE_WELL_D: f64 = 34.0;

const CONDITIONING_BLOCK_X: f64 = 510.0;
const CONDITIONING_BLOCK_Y: f64 = 92.0;
const CONDITIONING_BLOCK_Z: f64 = 48.0;
const MEDIA_CHANNELS: usize = 6;
const CHANNEL_PITCH_X: f64 = 78.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.7;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const DEGASSER_X: f64 = 560.0;
const DEGASSER_Y: f64 = 112.0;
const DEGASSER_Z: f64 = 138.0;
const BUBBLE_TRAPS: usize = 4;

const PUMP_BAY_X: f64 = 690.0;
const PUMP_BAY_Y: f64 = 182.0;
const PUMP_BAY_Z: f64 = 34.0;
const PERISTALTIC_HEADS: usize = 4;
const SYRINGE_BAYS: usize = 2;

const MANIFOLD_X: f64 = 700.0;
const MANIFOLD_Y: f64 = 36.0;
const MANIFOLD_Z: f64 = 212.0;
const VALVE_COUNT: usize = 12;
const FILTER_COUNT: usize = 6;

const WASTE_TRAY_X: f64 = 430.0;
const WASTE_TRAY_Y: f64 = 168.0;
const WASTE_TRAY_Z: f64 = 76.0;

const SERVICE_PANEL_X: f64 = 720.0;
const SERVICE_PANEL_Y: f64 = 22.0;
const SERVICE_PANEL_Z: f64 = 118.0;

const RESERVOIR_Z: f64 = 590.0;
const CONDITIONING_Z: f64 = 456.0;
const PUMP_Z: f64 = 308.0;
const WASTE_Z: f64 = 136.0;

fn main() {
    let frame = rack_frame();
    export(&frame, OUTPUTS[0]);

    let reservoir_tray = reservoir_bag_tray();
    export(&reservoir_tray, OUTPUTS[1]);

    let conditioning_block = warm_conditioning_block();
    export(&conditioning_block, OUTPUTS[2]);

    let degasser_bank = degasser_bubble_trap_bank();
    export(&degasser_bank, OUTPUTS[3]);

    let pump_bay = pump_bay_module();
    export(&pump_bay, OUTPUTS[4]);

    let valve_manifold = valve_filter_manifold();
    export(&valve_manifold, OUTPUTS[5]);

    let waste_tray = waste_collection_tray();
    export(&waste_tray, OUTPUTS[6]);

    let service_panel = clean_service_access_panel();
    export(&service_panel, OUTPUTS[7]);

    let assembly = frame
        + reservoir_tray.translate(0.0, -126.0, RESERVOIR_Z)
        + conditioning_block.translate(-64.0, -132.0, CONDITIONING_Z)
        + degasser_bank.translate(126.0, -128.0, CONDITIONING_Z + 92.0)
        + pump_bay.translate(0.0, -122.0, PUMP_Z)
        + valve_manifold.translate(0.0, RACK_Y / 2.0 - 44.0, 410.0)
        + waste_tray.translate(144.0, -124.0, WASTE_Z)
        + service_panel.translate(0.0, -(RACK_Y / 2.0 + 18.0), 420.0);

    export(&assembly, OUTPUTS[8]);

    println!(
        "Media conditioning/perfusion rack: {:.0}mm W x {:.0}mm D x {:.0}mm H frame, {} media-bag lands, {:.0}mm warm conditioning block, {} pump heads + {} syringe bays, {} valves, {} sterile filter placeholders, and front pull-out service access.",
        RACK_X,
        RACK_Y,
        RACK_Z,
        MEDIA_BAG_COUNT,
        CONDITIONING_BLOCK_X,
        PERISTALTIC_HEADS,
        SYRINGE_BAYS,
        VALVE_COUNT,
        FILTER_COUNT
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn rack_frame() -> Part {
    let base_pan = centered_cube("media_perfusion_rack_base_pan", RACK_X, RACK_Y, BASE_PAN_Z)
        .translate(0.0, 0.0, BASE_PAN_Z / 2.0);
    let base_sump = centered_cube(
        "media_perfusion_rack_base_sump",
        RACK_X - 86.0,
        RACK_Y - 82.0,
        18.0,
    )
    .translate(0.0, 0.0, BASE_PAN_Z - 9.0);
    let drain_port = centered_cylinder(
        "media_perfusion_rack_base_drain_bulkhead",
        13.0 / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        RACK_X / 2.0 - 76.0,
        -(RACK_Y / 2.0 - 12.0),
        BASE_PAN_Z - 14.0,
    );

    let mut posts = Part::empty("media_perfusion_rack_posts");
    for (i, (x, y)) in frame_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("media_perfusion_rack_post_{i}"),
                FRAME_W,
                FRAME_W,
                RACK_Z,
            )
            .translate(*x, *y, RACK_Z / 2.0);
    }

    let mut rails = Part::empty("media_perfusion_rack_rails");
    for (i, z) in [
        BASE_PAN_Z + 108.0,
        PUMP_Z - 54.0,
        CONDITIONING_Z - 54.0,
        RACK_Z - 22.0,
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("media_perfusion_rack_rear_rail_{i}"),
                RACK_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, RACK_Y / 2.0 - FRAME_W / 2.0, *z)
            + centered_cube(
                format!("media_perfusion_rack_left_side_rail_{i}"),
                FRAME_W,
                RACK_Y,
                FRAME_W,
            )
            .translate(-(RACK_X / 2.0 - FRAME_W / 2.0), 0.0, *z)
            + centered_cube(
                format!("media_perfusion_rack_right_side_rail_{i}"),
                FRAME_W,
                RACK_Y,
                FRAME_W,
            )
            .translate(RACK_X / 2.0 - FRAME_W / 2.0, 0.0, *z);
    }

    let top_front = centered_cube(
        "media_perfusion_rack_top_front_service_rail",
        RACK_X,
        FRAME_W,
        FRAME_W,
    )
    .translate(0.0, -(RACK_Y / 2.0 - FRAME_W / 2.0), RACK_Z - 22.0);
    let front_toe_left = centered_cube(
        "media_perfusion_rack_front_left_toe_rail",
        152.0,
        FRAME_W,
        FRAME_W,
    )
    .translate(
        -(RACK_X / 2.0 - 76.0),
        -(RACK_Y / 2.0 - FRAME_W / 2.0),
        BASE_PAN_Z + 108.0,
    );
    let front_toe_right = centered_cube(
        "media_perfusion_rack_front_right_toe_rail",
        152.0,
        FRAME_W,
        FRAME_W,
    )
    .translate(
        RACK_X / 2.0 - 76.0,
        -(RACK_Y / 2.0 - FRAME_W / 2.0),
        BASE_PAN_Z + 108.0,
    );

    base_pan - base_sump - drain_port
        + posts
        + rails
        + top_front
        + front_toe_left
        + front_toe_right
        + rack_shelf_rails()
        + caster_foot_plates()
        + rear_bulkhead_lands()
}

fn rack_shelf_rails() -> Part {
    let mut rails = Part::empty("media_perfusion_rack_slide_rails");
    for (i, z) in [
        RESERVOIR_Z - RESERVOIR_TRAY_Z / 2.0 - DECK_CLEARANCE_Z,
        CONDITIONING_Z - CONDITIONING_BLOCK_Z / 2.0 - DECK_CLEARANCE_Z,
        PUMP_Z - PUMP_BAY_Z / 2.0 - DECK_CLEARANCE_Z,
        WASTE_Z - WASTE_TRAY_Z / 2.0 - DECK_CLEARANCE_Z,
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("media_perfusion_rack_slide_left_{i}"),
                22.0,
                RACK_Y - 112.0,
                SHELF_Z,
            )
            .translate(-(RACK_X / 2.0 - 62.0), -18.0, *z)
            + centered_cube(
                format!("media_perfusion_rack_slide_right_{i}"),
                22.0,
                RACK_Y - 112.0,
                SHELF_Z,
            )
            .translate(RACK_X / 2.0 - 62.0, -18.0, *z)
            + centered_cube(
                format!("media_perfusion_rack_slide_rear_stop_{i}"),
                RACK_X - 150.0,
                18.0,
                SHELF_Z,
            )
            .translate(0.0, RACK_Y / 2.0 - 72.0, *z);
    }
    rails
}

fn caster_foot_plates() -> Part {
    let mut feet = Part::empty("media_perfusion_rack_leveling_feet");
    for (i, (x, y)) in [
        (-(RACK_X / 2.0 - 58.0), -(RACK_Y / 2.0 - 58.0)),
        (RACK_X / 2.0 - 58.0, -(RACK_Y / 2.0 - 58.0)),
        (-(RACK_X / 2.0 - 58.0), RACK_Y / 2.0 - 58.0),
        (RACK_X / 2.0 - 58.0, RACK_Y / 2.0 - 58.0),
        (0.0, RACK_Y / 2.0 - 58.0),
        (0.0, -(RACK_Y / 2.0 - 58.0)),
    ]
    .iter()
    .enumerate()
    {
        let plate = centered_cube(
            format!("media_perfusion_rack_foot_plate_{i}"),
            86.0,
            86.0,
            10.0,
        )
        .translate(*x, *y, -5.0);
        let stem = centered_cylinder(
            format!("media_perfusion_rack_foot_stem_clearance_{i}"),
            11.0 / 2.0,
            14.0,
            24,
        )
        .translate(*x, *y, -5.0);
        feet = feet + (plate - stem);
    }
    feet
}

fn rear_bulkhead_lands() -> Part {
    let lower = centered_cube(
        "media_perfusion_rack_rear_lower_bulkhead_land",
        RACK_X - 180.0,
        18.0,
        76.0,
    )
    .translate(0.0, RACK_Y / 2.0 + 2.0, 226.0);
    let upper = centered_cube(
        "media_perfusion_rack_rear_upper_bulkhead_land",
        RACK_X - 180.0,
        18.0,
        76.0,
    )
    .translate(0.0, RACK_Y / 2.0 + 2.0, 492.0);
    lower + upper
}

fn reservoir_bag_tray() -> Part {
    let tray = centered_cube(
        "media_perfusion_reservoir_tray_body",
        RESERVOIR_TRAY_X,
        RESERVOIR_TRAY_Y,
        RESERVOIR_TRAY_Z,
    );
    let pocket = centered_cube(
        "media_perfusion_reservoir_tray_floor_pocket",
        RESERVOIR_TRAY_X - 36.0,
        RESERVOIR_TRAY_Y - 30.0,
        RESERVOIR_TRAY_Z - 7.0,
    )
    .translate(0.0, 0.0, 5.0);
    let pull_handle = centered_cube(
        "media_perfusion_reservoir_pull_handle_cut",
        122.0,
        16.0,
        RESERVOIR_TRAY_Z + 2.0,
    )
    .translate(0.0, -(RESERVOIR_TRAY_Y / 2.0 - 8.0), 0.0);
    let drain_gutter = centered_cube(
        "media_perfusion_reservoir_tray_drain_gutter",
        RESERVOIR_TRAY_X - 92.0,
        12.0,
        RESERVOIR_TRAY_Z + 2.0,
    )
    .translate(0.0, -(RESERVOIR_TRAY_Y / 2.0 - 34.0), 2.0);
    let drain_port = centered_cylinder(
        "media_perfusion_reservoir_tray_drain_port",
        6.0 / 2.0,
        28.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        RESERVOIR_TRAY_X / 2.0 - 34.0,
        -(RESERVOIR_TRAY_Y / 2.0 - 10.0),
        0.0,
    );

    let mut bag_recesses = Part::empty("media_perfusion_bag_recesses");
    let mut bag_rails = Part::empty("media_perfusion_bag_clip_rails");
    for i in 0..MEDIA_BAG_COUNT {
        let x = media_bag_x(i);
        bag_recesses = bag_recesses
            + centered_cube(
                format!("media_perfusion_bag_land_recess_{i}"),
                MEDIA_BAG_LAND_X,
                MEDIA_BAG_LAND_Y,
                7.0,
            )
            .translate(x, 8.0, RESERVOIR_TRAY_Z / 2.0 - 3.5);

        bag_rails = bag_rails
            + centered_cube(
                format!("media_perfusion_bag_left_retainer_{i}"),
                8.0,
                MEDIA_BAG_LAND_Y + 18.0,
                14.0,
            )
            .translate(
                x - MEDIA_BAG_LAND_X / 2.0 - 8.0,
                8.0,
                RESERVOIR_TRAY_Z / 2.0 + 7.0,
            )
            + centered_cube(
                format!("media_perfusion_bag_right_retainer_{i}"),
                8.0,
                MEDIA_BAG_LAND_Y + 18.0,
                14.0,
            )
            .translate(
                x + MEDIA_BAG_LAND_X / 2.0 + 8.0,
                8.0,
                RESERVOIR_TRAY_Z / 2.0 + 7.0,
            )
            + centered_cube(
                format!("media_perfusion_bag_spike_guard_{i}"),
                MEDIA_BAG_LAND_X - 30.0,
                10.0,
                18.0,
            )
            .translate(
                x,
                -(MEDIA_BAG_LAND_Y / 2.0 - 2.0),
                RESERVOIR_TRAY_Z / 2.0 + 9.0,
            );
    }

    let mut bottle_wells = Part::empty("media_perfusion_reservoir_bottle_wells");
    for (i, x) in [-286.0, 286.0].iter().enumerate() {
        bottle_wells = bottle_wells
            + centered_cylinder(
                format!("media_perfusion_reservoir_bottle_well_{i}"),
                BOTTLE_WELL_D / 2.0,
                RESERVOIR_TRAY_Z + 2.0,
                40,
            )
            .translate(*x, -38.0, 4.0);
    }

    let mut latch_bosses = Part::empty("media_perfusion_reservoir_latch_bosses");
    for (i, x) in [
        -(RESERVOIR_TRAY_X / 2.0 - 28.0),
        RESERVOIR_TRAY_X / 2.0 - 28.0,
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("media_perfusion_reservoir_latch_boss_{i}"),
            10.0,
            8.0,
            24,
        )
        .translate(
            *x,
            -(RESERVOIR_TRAY_Y / 2.0 - 26.0),
            RESERVOIR_TRAY_Z / 2.0 + 4.0,
        );
        let hole = centered_cylinder(
            format!("media_perfusion_reservoir_latch_hole_{i}"),
            3.3 / 2.0,
            10.0,
            20,
        )
        .translate(
            *x,
            -(RESERVOIR_TRAY_Y / 2.0 - 26.0),
            RESERVOIR_TRAY_Z / 2.0 + 4.0,
        );
        latch_bosses = latch_bosses + (boss - hole);
    }

    tray - pocket - pull_handle - drain_gutter - drain_port - bag_recesses - bottle_wells
        + bag_rails
        + latch_bosses
        + asymmetric_tray_key("media_perfusion_reservoir")
}

fn warm_conditioning_block() -> Part {
    let body = centered_cube(
        "media_perfusion_warm_conditioning_block_body",
        CONDITIONING_BLOCK_X,
        CONDITIONING_BLOCK_Y,
        CONDITIONING_BLOCK_Z,
    );

    let mut cuts = Part::empty("media_perfusion_warm_conditioning_block_cuts");
    for i in 0..MEDIA_CHANNELS {
        let x = channel_x(i);
        let tube_bore = centered_cylinder(
            format!("media_perfusion_warm_block_tube_bore_{i}"),
            FLUID_BORE_D / 2.0,
            CONDITIONING_BLOCK_Y + 8.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 8.0);
        let top_slot = centered_cube(
            format!("media_perfusion_warm_block_tube_slot_{i}"),
            FLUID_BORE_D + 1.4,
            CONDITIONING_BLOCK_Y + 8.0,
            CONDITIONING_BLOCK_Z,
        )
        .translate(x, 0.0, CONDITIONING_BLOCK_Z / 2.0 - 8.0);
        cuts = cuts + tube_bore + top_slot;
    }

    for (i, y) in [-26.0, 26.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("media_perfusion_warm_block_heater_bore_{i}"),
                6.4 / 2.0,
                CONDITIONING_BLOCK_X - 70.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, *y, -13.0);
    }

    cuts = cuts
        + centered_cylinder(
            "media_perfusion_warm_block_thermistor_bore",
            3.5 / 2.0,
            46.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -(CONDITIONING_BLOCK_Y / 2.0 - 20.0), -4.0);

    let mut mount_holes = Part::empty("media_perfusion_warm_block_mount_holes");
    for (i, (x, y)) in [
        (
            -(CONDITIONING_BLOCK_X / 2.0 - 24.0),
            -(CONDITIONING_BLOCK_Y / 2.0 - 16.0),
        ),
        (
            CONDITIONING_BLOCK_X / 2.0 - 24.0,
            -(CONDITIONING_BLOCK_Y / 2.0 - 16.0),
        ),
        (
            -(CONDITIONING_BLOCK_X / 2.0 - 24.0),
            CONDITIONING_BLOCK_Y / 2.0 - 16.0,
        ),
        (
            CONDITIONING_BLOCK_X / 2.0 - 24.0,
            CONDITIONING_BLOCK_Y / 2.0 - 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("media_perfusion_warm_block_m4_mount_{i}"),
                4.3 / 2.0,
                CONDITIONING_BLOCK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    body - cuts - mount_holes + conditioning_block_insulation_lips()
}

fn conditioning_block_insulation_lips() -> Part {
    centered_cube(
        "media_perfusion_warm_block_front_insulation_lip",
        CONDITIONING_BLOCK_X + 36.0,
        10.0,
        18.0,
    )
    .translate(0.0, -(CONDITIONING_BLOCK_Y / 2.0 + 5.0), 4.0)
        + centered_cube(
            "media_perfusion_warm_block_rear_insulation_lip",
            CONDITIONING_BLOCK_X + 36.0,
            10.0,
            18.0,
        )
        .translate(0.0, CONDITIONING_BLOCK_Y / 2.0 + 5.0, 4.0)
}

fn degasser_bubble_trap_bank() -> Part {
    let rail = centered_cube(
        "media_perfusion_degasser_bank_base_rail",
        DEGASSER_X,
        DEGASSER_Y,
        22.0,
    )
    .translate(0.0, 0.0, -DEGASSER_Z / 2.0 + 11.0);
    let rear_plate = centered_cube(
        "media_perfusion_degasser_bank_rear_plate",
        DEGASSER_X,
        18.0,
        DEGASSER_Z,
    )
    .translate(0.0, DEGASSER_Y / 2.0 - 9.0, 0.0);
    let degasser_cassette = centered_cube(
        "media_perfusion_membrane_degasser_placeholder",
        210.0,
        62.0,
        92.0,
    )
    .translate(-(DEGASSER_X / 2.0 - 132.0), -10.0, 8.0);
    let cassette_window = centered_cube(
        "media_perfusion_membrane_degasser_service_window",
        172.0,
        66.0,
        48.0,
    )
    .translate(-(DEGASSER_X / 2.0 - 132.0), -10.0, 18.0);

    let mut traps = Part::empty("media_perfusion_bubble_trap_placeholders");
    for i in 0..BUBBLE_TRAPS {
        let x = -42.0 + i as f64 * 82.0;
        let trap_body = centered_cylinder(
            format!("media_perfusion_bubble_trap_body_{i}"),
            19.0,
            112.0,
            44,
        )
        .translate(x, -16.0, 8.0);
        let clear_core = centered_cylinder(
            format!("media_perfusion_bubble_trap_visual_core_{i}"),
            14.0,
            116.0,
            44,
        )
        .translate(x, -16.0, 8.0);
        let clamp_top = centered_cube(
            format!("media_perfusion_bubble_trap_top_clamp_{i}"),
            54.0,
            14.0,
            12.0,
        )
        .translate(x, -42.0, 54.0);
        let clamp_bottom = centered_cube(
            format!("media_perfusion_bubble_trap_bottom_clamp_{i}"),
            54.0,
            14.0,
            12.0,
        )
        .translate(x, -42.0, -38.0);
        let vent_stub = centered_cylinder(
            format!("media_perfusion_bubble_trap_vent_stub_{i}"),
            4.0 / 2.0,
            30.0,
            20,
        )
        .translate(x, -16.0, 76.0);
        traps = traps + (trap_body - clear_core) + clamp_top + clamp_bottom + vent_stub;
    }

    let mut ports = Part::empty("media_perfusion_degasser_bank_ports");
    for i in 0..MEDIA_CHANNELS {
        let x = channel_x(i) * (DEGASSER_X - 90.0) / CONDITIONING_BLOCK_X;
        ports = ports
            + centered_cylinder(
                format!("media_perfusion_degasser_bank_bulkhead_{i}"),
                7.0 / 2.0,
                26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, DEGASSER_Y / 2.0 - 8.0, -48.0);
    }

    rail + rear_plate + (degasser_cassette - cassette_window) + traps - ports
}

fn pump_bay_module() -> Part {
    let base = centered_cube(
        "media_perfusion_pump_bay_baseplate",
        PUMP_BAY_X,
        PUMP_BAY_Y,
        PUMP_BAY_Z,
    );
    let base_pocket = centered_cube(
        "media_perfusion_pump_bay_base_pocket",
        PUMP_BAY_X - 38.0,
        PUMP_BAY_Y - 34.0,
        PUMP_BAY_Z - 8.0,
    )
    .translate(0.0, 0.0, 5.0);
    let pull_handle = centered_cube(
        "media_perfusion_pump_bay_pull_handle_cut",
        120.0,
        16.0,
        PUMP_BAY_Z + 2.0,
    )
    .translate(0.0, -(PUMP_BAY_Y / 2.0 - 8.0), 0.0);

    let mut peristaltic = Part::empty("media_perfusion_peristaltic_heads");
    for i in 0..PERISTALTIC_HEADS {
        let x = -252.0 + i as f64 * 84.0;
        peristaltic = peristaltic + peristaltic_head_placeholder(i, x, 24.0);
    }

    let mut syringe = Part::empty("media_perfusion_syringe_bays");
    for i in 0..SYRINGE_BAYS {
        let y = -54.0 + i as f64 * 54.0;
        syringe = syringe + syringe_bay_placeholder(i, 214.0, y);
    }

    let tube_comb = pump_bay_tube_comb().translate(-54.0, -(PUMP_BAY_Y / 2.0 + 14.0), 23.0);
    let latch = cartridge_latch_bosses(
        "media_perfusion_pump_bay",
        PUMP_BAY_X,
        PUMP_BAY_Y,
        PUMP_BAY_Z,
    );

    base - base_pocket - pull_handle
        + peristaltic
        + syringe
        + tube_comb
        + latch
        + asymmetric_tray_key("media_perfusion_pump_bay")
}

fn peristaltic_head_placeholder(index: usize, x: f64, y: f64) -> Part {
    let ring = centered_cylinder(
        format!("media_perfusion_peristaltic_head_ring_{index}"),
        31.0,
        12.0,
        48,
    )
    .translate(x, y, PUMP_BAY_Z / 2.0 + 6.0);
    let rotor_void = centered_cylinder(
        format!("media_perfusion_peristaltic_head_service_void_{index}"),
        20.0,
        14.0,
        48,
    )
    .translate(x, y, PUMP_BAY_Z / 2.0 + 6.0);
    let motor_pocket = centered_cube(
        format!("media_perfusion_peristaltic_motor_pocket_{index}"),
        62.0,
        52.0,
        30.0,
    )
    .translate(x, y + 54.0, PUMP_BAY_Z / 2.0 + 7.0);
    let tube_slot = centered_cube(
        format!("media_perfusion_peristaltic_tube_slot_{index}"),
        76.0,
        8.0,
        24.0,
    )
    .translate(x, y - 34.0, PUMP_BAY_Z / 2.0 + 10.0);
    (ring - rotor_void) + motor_pocket - tube_slot
}

fn syringe_bay_placeholder(index: usize, x: f64, y: f64) -> Part {
    let rail_left = centered_cube(
        format!("media_perfusion_syringe_bay_left_rail_{index}"),
        222.0,
        8.0,
        18.0,
    )
    .translate(x, y - 17.0, PUMP_BAY_Z / 2.0 + 9.0);
    let rail_right = centered_cube(
        format!("media_perfusion_syringe_bay_right_rail_{index}"),
        222.0,
        8.0,
        18.0,
    )
    .translate(x, y + 17.0, PUMP_BAY_Z / 2.0 + 9.0);
    let barrel_saddle = centered_cylinder(
        format!("media_perfusion_syringe_bay_barrel_saddle_{index}"),
        12.0,
        150.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x + 18.0, y, PUMP_BAY_Z / 2.0 + 15.0);
    let pusher_clearance = centered_cube(
        format!("media_perfusion_syringe_pusher_clearance_{index}"),
        44.0,
        52.0,
        24.0,
    )
    .translate(x - 116.0, y, PUMP_BAY_Z / 2.0 + 9.0);
    rail_left + rail_right + barrel_saddle - pusher_clearance
}

fn pump_bay_tube_comb() -> Part {
    let body = centered_cube("media_perfusion_pump_bay_tube_comb", 440.0, 18.0, 18.0);
    let mut cuts = Part::empty("media_perfusion_pump_bay_tube_comb_cuts");
    for i in 0..MEDIA_CHANNELS {
        let x = -185.0 + i as f64 * 74.0;
        cuts = cuts
            + centered_cylinder(
                format!("media_perfusion_pump_bay_tube_clip_{i}"),
                FLUID_BORE_D / 2.0,
                22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0)
            + centered_cube(
                format!("media_perfusion_pump_bay_tube_clip_slot_{i}"),
                FLUID_BORE_D + 1.0,
                22.0,
                14.0,
            )
            .translate(x, 0.0, 6.0);
    }
    body - cuts
}

fn valve_filter_manifold() -> Part {
    let panel = centered_cube(
        "media_perfusion_valve_filter_panel_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let valve_block = centered_cube(
        "media_perfusion_valve_manifold_bar",
        MANIFOLD_X - 110.0,
        48.0,
        54.0,
    )
    .translate(-18.0, -(MANIFOLD_Y / 2.0 + 20.0), 16.0);
    let drain_header = centered_cube(
        "media_perfusion_relief_drain_header",
        MANIFOLD_X - 170.0,
        34.0,
        34.0,
    )
    .translate(12.0, -(MANIFOLD_Y / 2.0 + 14.0), -72.0);

    let mut cuts = Part::empty("media_perfusion_valve_filter_panel_cuts");
    for i in 0..VALVE_COUNT {
        let x = valve_x(i);
        cuts = cuts
            + centered_cylinder(
                format!("media_perfusion_valve_pocket_{i}"),
                13.0,
                MANIFOLD_Y + 70.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 45.0)
            + centered_cylinder(
                format!("media_perfusion_valve_port_a_{i}"),
                3.2 / 2.0,
                MANIFOLD_Y + 78.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 8.0, 0.0, 17.0)
            + centered_cylinder(
                format!("media_perfusion_valve_port_b_{i}"),
                3.2 / 2.0,
                MANIFOLD_Y + 78.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 8.0, 0.0, 17.0);
    }

    for i in 0..FILTER_COUNT {
        let x = filter_x(i);
        cuts = cuts
            + centered_cylinder(
                format!("media_perfusion_filter_bulkhead_in_{i}"),
                7.0 / 2.0,
                MANIFOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 14.0, 0.0, -18.0)
            + centered_cylinder(
                format!("media_perfusion_filter_bulkhead_out_{i}"),
                7.0 / 2.0,
                MANIFOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 14.0, 0.0, -18.0);
    }

    let relief_port = centered_cylinder(
        "media_perfusion_pressure_relief_bulkhead",
        15.0 / 2.0,
        MANIFOLD_Y + 90.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(MANIFOLD_X / 2.0 - 44.0), 0.0, -72.0);
    let drain_port = centered_cylinder(
        "media_perfusion_manifold_drain_bulkhead",
        11.0 / 2.0,
        MANIFOLD_Y + 90.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MANIFOLD_X / 2.0 - 48.0, 0.0, -72.0);
    let cable_slot = centered_cube(
        "media_perfusion_valve_panel_cable_slot",
        138.0,
        MANIFOLD_Y + 8.0,
        34.0,
    )
    .translate(MANIFOLD_X / 2.0 - 116.0, 0.0, 78.0);

    (panel + valve_block + drain_header - cuts - relief_port - drain_port - cable_slot)
        + sterile_filter_placeholders()
        + pressure_sensor_pockets()
        + panel_mount_tabs("media_perfusion_valve_filter_panel", MANIFOLD_X, MANIFOLD_Z)
        + manifold_label_lands()
}

fn sterile_filter_placeholders() -> Part {
    let mut filters = Part::empty("media_perfusion_sterile_filter_placeholders");
    for i in 0..FILTER_COUNT {
        let x = filter_x(i);
        let capsule = centered_cylinder(
            format!("media_perfusion_inline_sterile_filter_{i}"),
            18.0,
            74.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(MANIFOLD_Y / 2.0 + 36.0), -20.0);
        let clamp = centered_cube(
            format!("media_perfusion_inline_filter_clamp_{i}"),
            48.0,
            12.0,
            18.0,
        )
        .translate(x, -(MANIFOLD_Y / 2.0 + 76.0), -20.0);
        filters = filters + capsule + clamp;
    }
    filters
}

fn pressure_sensor_pockets() -> Part {
    let mut pockets = Part::empty("media_perfusion_pressure_sensor_pockets");
    for (i, x) in [-274.0, -206.0, 206.0, 274.0].iter().enumerate() {
        pockets = pockets
            + centered_cube(
                format!("media_perfusion_pressure_sensor_body_{i}"),
                48.0,
                38.0,
                58.0,
            )
            .translate(*x, -(MANIFOLD_Y / 2.0 + 24.0), 88.0)
            - centered_cube(
                format!("media_perfusion_pressure_sensor_display_recess_{i}"),
                34.0,
                12.0,
                30.0,
            )
            .translate(*x, -(MANIFOLD_Y / 2.0 + 44.0), 88.0);
    }
    pockets
}

fn manifold_label_lands() -> Part {
    centered_cube(
        "media_perfusion_valve_filter_label_land_upper",
        MANIFOLD_X - 90.0,
        4.0,
        9.0,
    )
    .translate(0.0, -(MANIFOLD_Y / 2.0 + 2.0), 101.0)
        + centered_cube(
            "media_perfusion_valve_filter_label_land_lower",
            MANIFOLD_X - 90.0,
            4.0,
            9.0,
        )
        .translate(0.0, -(MANIFOLD_Y / 2.0 + 2.0), -101.0)
}

fn waste_collection_tray() -> Part {
    let tray = centered_cube(
        "media_perfusion_waste_collection_tray_body",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let cavity = centered_cube(
        "media_perfusion_waste_collection_tray_cavity",
        WASTE_TRAY_X - 32.0,
        WASTE_TRAY_Y - 30.0,
        WASTE_TRAY_Z - 14.0,
    )
    .translate(0.0, 0.0, 9.0);
    let pull_handle = centered_cube(
        "media_perfusion_waste_tray_pull_handle_cut",
        118.0,
        16.0,
        WASTE_TRAY_Z + 2.0,
    )
    .translate(0.0, -(WASTE_TRAY_Y / 2.0 - 8.0), 0.0);

    let mut bottle_wells = Part::empty("media_perfusion_waste_bottle_wells");
    for (i, x) in [-128.0, -46.0].iter().enumerate() {
        bottle_wells = bottle_wells
            + centered_cylinder(
                format!("media_perfusion_waste_bottle_well_{i}"),
                34.0,
                WASTE_TRAY_Z + 2.0,
                48,
            )
            .translate(*x, 8.0, 12.0);
    }

    let waste_bag_bay = centered_cube(
        "media_perfusion_waste_bag_bay",
        150.0,
        104.0,
        WASTE_TRAY_Z + 2.0,
    )
    .translate(112.0, 6.0, 12.0);
    let drain_slope = centered_cube(
        "media_perfusion_waste_tray_drain_slope_gutter",
        WASTE_TRAY_X - 92.0,
        14.0,
        WASTE_TRAY_Z + 2.0,
    )
    .translate(0.0, -(WASTE_TRAY_Y / 2.0 - 36.0), 18.0);
    let drain_port = centered_cylinder(
        "media_perfusion_waste_tray_drain_bulkhead",
        9.0 / 2.0,
        34.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WASTE_TRAY_X / 2.0 - 36.0,
        -(WASTE_TRAY_Y / 2.0 - 10.0),
        -6.0,
    );

    let overflow_sensor = centered_cube(
        "media_perfusion_waste_overflow_sensor_pocket",
        38.0,
        16.0,
        48.0,
    )
    .translate(
        -(WASTE_TRAY_X / 2.0 - 36.0),
        WASTE_TRAY_Y / 2.0 - 18.0,
        10.0,
    );

    tray - cavity
        - pull_handle
        - bottle_wells
        - waste_bag_bay
        - drain_slope
        - drain_port
        - overflow_sensor
        + cartridge_latch_bosses(
            "media_perfusion_waste_tray",
            WASTE_TRAY_X,
            WASTE_TRAY_Y,
            WASTE_TRAY_Z,
        )
        + asymmetric_tray_key("media_perfusion_waste")
}

fn clean_service_access_panel() -> Part {
    let panel = centered_cube(
        "media_perfusion_clean_service_panel_body",
        SERVICE_PANEL_X,
        SERVICE_PANEL_Y,
        SERVICE_PANEL_Z,
    );
    let window = centered_cube(
        "media_perfusion_clean_service_panel_view_window",
        SERVICE_PANEL_X - 150.0,
        SERVICE_PANEL_Y + 8.0,
        SERVICE_PANEL_Z - 42.0,
    )
    .translate(0.0, 0.0, 8.0);
    let pull = centered_cube(
        "media_perfusion_clean_service_panel_pull_cut",
        112.0,
        SERVICE_PANEL_Y + 8.0,
        18.0,
    )
    .translate(0.0, 0.0, -(SERVICE_PANEL_Z / 2.0 - 14.0));

    let mut tube_clips = Part::empty("media_perfusion_clean_service_panel_tube_clips");
    for i in 0..MEDIA_CHANNELS {
        let x = channel_x(i) * (SERVICE_PANEL_X - 160.0) / CONDITIONING_BLOCK_X;
        tube_clips = tube_clips
            + centered_cylinder(
                format!("media_perfusion_service_panel_tube_clip_{i}"),
                FLUID_BORE_D / 2.0,
                SERVICE_PANEL_Y + 14.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -(SERVICE_PANEL_Z / 2.0 - 28.0))
            + centered_cube(
                format!("media_perfusion_service_panel_tube_slot_{i}"),
                FLUID_BORE_D + 1.2,
                SERVICE_PANEL_Y + 14.0,
                18.0,
            )
            .translate(x, 0.0, -(SERVICE_PANEL_Z / 2.0 - 20.0));
    }

    let mut latch_holes = Part::empty("media_perfusion_service_panel_latch_holes");
    for (i, x) in [
        -(SERVICE_PANEL_X / 2.0 - 42.0),
        SERVICE_PANEL_X / 2.0 - 42.0,
        -86.0,
        86.0,
    ]
    .iter()
    .enumerate()
    {
        latch_holes = latch_holes
            + centered_cylinder(
                format!("media_perfusion_service_panel_latch_hole_{i}"),
                4.4 / 2.0,
                SERVICE_PANEL_Y + 12.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, SERVICE_PANEL_Z / 2.0 - 16.0);
    }

    panel - window - pull - tube_clips - latch_holes
}

fn panel_mount_tabs(name: &str, width: f64, height: f64) -> Part {
    let mut tabs = Part::empty(format!("{name}_mount_tabs"));
    for (i, x) in [
        -(width / 2.0 - 28.0),
        -(width / 4.0),
        width / 4.0,
        width / 2.0 - 28.0,
    ]
    .iter()
    .enumerate()
    {
        let tab = centered_cube(format!("{name}_mount_tab_{i}"), 38.0, 28.0, 16.0).translate(
            *x,
            4.0,
            -(height / 2.0 + 8.0),
        );
        let hole = centered_cylinder(format!("{name}_mount_hole_{i}"), 5.4 / 2.0, 32.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 4.0, -(height / 2.0 + 8.0));
        tabs = tabs + (tab - hole);
    }
    tabs
}

fn cartridge_latch_bosses(name: &str, width: f64, depth: f64, height: f64) -> Part {
    let mut bosses = Part::empty(format!("{name}_latch_bosses"));
    for (i, x) in [-(width / 2.0 - 28.0), width / 2.0 - 28.0]
        .iter()
        .enumerate()
    {
        let boss = centered_cylinder(format!("{name}_latch_boss_{i}"), 10.0, 8.0, 24).translate(
            *x,
            -(depth / 2.0 - 26.0),
            height / 2.0 + 4.0,
        );
        let hole = centered_cylinder(format!("{name}_latch_hole_{i}"), 3.3 / 2.0, 10.0, 20)
            .translate(*x, -(depth / 2.0 - 26.0), height / 2.0 + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn asymmetric_tray_key(name: &str) -> Part {
    centered_cube(format!("{name}_asymmetric_key"), 44.0, 18.0, 10.0).translate(-40.0, 82.0, -2.0)
}

fn frame_post_points() -> [(f64, f64); 6] {
    [
        (
            -(RACK_X / 2.0 - FRAME_W / 2.0),
            -(RACK_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            RACK_X / 2.0 - FRAME_W / 2.0,
            -(RACK_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            -(RACK_X / 2.0 - FRAME_W / 2.0),
            RACK_Y / 2.0 - FRAME_W / 2.0,
        ),
        (RACK_X / 2.0 - FRAME_W / 2.0, RACK_Y / 2.0 - FRAME_W / 2.0),
        (0.0, RACK_Y / 2.0 - FRAME_W / 2.0),
        (0.0, -(RACK_Y / 2.0 - FRAME_W / 2.0)),
    ]
}

fn media_bag_x(index: usize) -> f64 {
    let pitch = MEDIA_BAG_LAND_X + 38.0;
    -((MEDIA_BAG_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn channel_x(index: usize) -> f64 {
    -((MEDIA_CHANNELS as f64 - 1.0) * CHANNEL_PITCH_X) / 2.0 + index as f64 * CHANNEL_PITCH_X
}

fn valve_x(index: usize) -> f64 {
    let pitch = 48.0;
    -((VALVE_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn filter_x(index: usize) -> f64 {
    let pitch = 92.0;
    -((FILTER_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 9);
        for path in OUTPUTS {
            assert!(path.starts_with("output/media_conditioning_perfusion_rack_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn rack_accepts_all_service_cartridges() {
        assert!(RESERVOIR_TRAY_X <= RACK_X - 100.0);
        assert!(PUMP_BAY_X <= RACK_X - 100.0);
        assert!(MANIFOLD_X <= RACK_X - 90.0);
        assert!(WASTE_TRAY_X <= RACK_X - 260.0);
        assert!(SERVICE_PANEL_X <= RACK_X - 80.0);
    }

    #[test]
    fn media_channels_are_symmetric_and_inside_warm_block() {
        assert_eq!(channel_x(0), -channel_x(MEDIA_CHANNELS - 1));
        assert_eq!(channel_x(1), -channel_x(MEDIA_CHANNELS - 2));
        assert!(channel_x(MEDIA_CHANNELS - 1).abs() < CONDITIONING_BLOCK_X / 2.0 - 40.0);
    }

    #[test]
    fn valve_and_filter_arrays_stay_inside_panel() {
        assert!(valve_x(0).abs() < MANIFOLD_X / 2.0 - 42.0);
        assert!(valve_x(VALVE_COUNT - 1).abs() < MANIFOLD_X / 2.0 - 42.0);
        assert!(filter_x(0).abs() < MANIFOLD_X / 2.0 - 48.0);
        assert!(filter_x(FILTER_COUNT - 1).abs() < MANIFOLD_X / 2.0 - 48.0);
    }
}
