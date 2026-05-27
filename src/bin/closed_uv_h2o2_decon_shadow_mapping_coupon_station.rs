use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed UV/H2O2 decontamination shadow-mapping coupon station.
//
// Intent:
// - Qualify isolator/module UV and vaporized H2O2 exposure coverage by holding
//   repeatable coupon grids, deliberate shadow masks, exposure indicators, and
//   flow witness placeholders in a sealed transfer tray.
// - Keep clean incoming coupons/cards physically separated from used evidence,
//   release/hold/reject decisions, barcode/certificate lands, and camera proof.
// - Model the practical station envelope and interfaces only. This is not a
//   validated decontamination cycle, acceptance criterion, UV dose model, or
//   H2O2/VHP process recipe.
//
// Exports:
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_base_deck.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_sealed_transfer_tray.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_coupon_grid_carrier.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_shadow_mask_test_blocks.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_exposure_indicator_card_lands.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_vhp_h2o2_flow_witness_placeholders.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_barcode_certificate_lands.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_clean_used_segregation.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_release_hold_reject_lanes.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_evidence_camera_bridge.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_robot_service_keepouts.stl
//   output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_assembly.stl

const OUTPUT_PREFIX: &str = "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_base_deck.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_sealed_transfer_tray.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_coupon_grid_carrier.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_shadow_mask_test_blocks.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_exposure_indicator_card_lands.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_vhp_h2o2_flow_witness_placeholders.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_barcode_certificate_lands.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_clean_used_segregation.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_release_hold_reject_lanes.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_evidence_camera_bridge.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_robot_service_keepouts.stl",
    "output/closed_uv_h2o2_decon_shadow_mapping_coupon_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "coupon_grid_carrier",
    "shadow_mask_test_blocks",
    "exposure_indicator_card_lands",
    "vhp_h2o2_flow_witness_placeholders",
    "barcode_certificate_lands",
    "clean_used_segregation",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_keepouts",
    "service_keepouts",
    "sealed_transfer_tray",
];

const DECK_X: f64 = 1340.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 36.0;
const MOUNT_HOLE_D: f64 = 6.6;

const TRAY_X: f64 = 1270.0;
const TRAY_Y: f64 = 760.0;
const TRAY_Z: f64 = 52.0;
const TRAY_WALL: f64 = 26.0;
const TRAY_FLOOR_Z: f64 = 7.0;
const GASKET_W: f64 = 7.0;
const LATCH_COUNT: usize = 8;
const TRANSFER_PORT_D: f64 = 84.0;

const COUPON_CENTER: (f64, f64) = (-365.0, 170.0);
const COUPON_GRID_X: f64 = 455.0;
const COUPON_GRID_Y: f64 = 300.0;
const COUPON_GRID_Z: f64 = 34.0;
const COUPON_ROWS: usize = 4;
const COUPON_COLS: usize = 6;
const COUPON_SLOTS: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 44.0;
const COUPON_SLOT_Y: f64 = 32.0;
const COUPON_PITCH_X: f64 = 62.0;
const COUPON_PITCH_Y: f64 = 58.0;
const COUPON_DATUM_PINS: usize = 4;

const SHADOW_CENTER: (f64, f64) = (165.0, 180.0);
const SHADOW_BLOCK_X: f64 = 420.0;
const SHADOW_BLOCK_Y: f64 = 300.0;
const SHADOW_BLOCK_Z: f64 = 24.0;
const SHADOW_TEST_BLOCKS: usize = 8;
const SHADOW_BLOCK_PITCH_X: f64 = 92.0;
const SHADOW_BLOCK_PITCH_Y: f64 = 112.0;
const SHADOW_BLOCK_BASE_X: f64 = 54.0;
const SHADOW_BLOCK_BASE_Y: f64 = 62.0;
const SHADOW_BLOCK_MIN_Z: f64 = 30.0;
const SHADOW_BLOCK_STEP_Z: f64 = 10.0;
const SHADOW_UNDERCUT_GAP: f64 = 14.0;

const INDICATOR_CENTER: (f64, f64) = (-460.0, -168.0);
const INDICATOR_PANEL_X: f64 = 350.0;
const INDICATOR_PANEL_Y: f64 = 230.0;
const INDICATOR_PANEL_Z: f64 = 22.0;
const INDICATOR_ROWS: usize = 2;
const INDICATOR_COLS: usize = 4;
const INDICATOR_CARD_COUNT: usize = INDICATOR_ROWS * INDICATOR_COLS;
const INDICATOR_LAND_X: f64 = 66.0;
const INDICATOR_LAND_Y: f64 = 40.0;
const INDICATOR_PITCH_X: f64 = 78.0;
const INDICATOR_PITCH_Y: f64 = 76.0;
const UV_DOSE_CHIP_LANDS: usize = 8;

const FLOW_CENTER: (f64, f64) = (-60.0, -120.0);
const FLOW_PANEL_X: f64 = 390.0;
const FLOW_PANEL_Y: f64 = 230.0;
const FLOW_PANEL_Z: f64 = 32.0;
const FLOW_WITNESS_COUNT: usize = 10;
const FLOW_WITNESS_PITCH_X: f64 = 58.0;
const H2O2_WITNESS_D: f64 = 18.0;
const FLOW_VANE_COUNT: usize = 5;
const FLOW_VECTOR_BAR_X: f64 = 300.0;

const TRACE_CENTER: (f64, f64) = (430.0, -110.0);
const TRACE_PANEL_X: f64 = 340.0;
const TRACE_PANEL_Y: f64 = 250.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 12;
const CERTIFICATE_LANDS: usize = 3;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 22.0;
const CERT_LAND_X: f64 = 124.0;
const CERT_LAND_Y: f64 = 62.0;

const SEGREGATION_WALL_CENTER_X: f64 = -270.0;
const SEGREGATION_WALL_Y: f64 = 708.0;
const SEGREGATION_WALL_X: f64 = 24.0;
const SEGREGATION_WALL_Z: f64 = 82.0;
const CLEAN_BUFFER_X: f64 = 210.0;
const USED_BUFFER_X: f64 = 260.0;
const SEGREGATION_PASS_GATE_Y: f64 = 118.0;

const STATUS_CENTER: (f64, f64) = (360.0, -310.0);
const STATUS_PANEL_X: f64 = 520.0;
const STATUS_PANEL_Y: f64 = 150.0;
const STATUS_PANEL_Z: f64 = 30.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 96.0;
const STATUS_SLOT_Y: f64 = 34.0;
const STATUS_LANE_PITCH_Y: f64 = 46.0;

const CAMERA_BRIDGE_X: f64 = 1040.0;
const CAMERA_BRIDGE_Y: f64 = 178.0;
const CAMERA_POST_X: f64 = 40.0;
const CAMERA_POST_Y: f64 = 56.0;
const CAMERA_UNDERSIDE_Z: f64 = 238.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const CAMERA_PODS: usize = 4;
const LED_BARS: usize = 2;

const ROBOT_FRONT_KEEP_OUT_Y: f64 = 310.0;
const SERVICE_REAR_KEEP_OUT_Y: f64 = 260.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 230.0;
const LEFT_TRANSFER_KEEP_OUT_X: f64 = 240.0;
const ROBOT_Z_CLEARANCE: f64 = 340.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;

const STATUS_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];

#[derive(Clone, Copy)]
struct ModuleEnvelope {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl ModuleEnvelope {
    fn fits_on_deck(self) -> bool {
        let usable_half_x = DECK_X / 2.0 - RIM_W;
        let usable_half_y = DECK_Y / 2.0 - RIM_W;
        self.center.0 - self.x / 2.0 >= -usable_half_x
            && self.center.0 + self.x / 2.0 <= usable_half_x
            && self.center.1 - self.y / 2.0 >= -usable_half_y
            && self.center.1 + self.y / 2.0 <= usable_half_y
    }

    fn overlaps(self, other: Self) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = base_deck();
    write_part(OUTPUTS[0], &deck);

    let transfer_tray = sealed_transfer_tray();
    write_part(OUTPUTS[1], &transfer_tray);

    let coupon_grid = coupon_grid_carrier();
    write_part(OUTPUTS[2], &coupon_grid);

    let shadow_blocks = shadow_mask_test_blocks();
    write_part(OUTPUTS[3], &shadow_blocks);

    let indicator_lands = exposure_indicator_card_lands();
    write_part(OUTPUTS[4], &indicator_lands);

    let flow_witnesses = vhp_h2o2_flow_witness_placeholders();
    write_part(OUTPUTS[5], &flow_witnesses);

    let traceability = barcode_certificate_lands();
    write_part(OUTPUTS[6], &traceability);

    let segregation = clean_used_segregation();
    write_part(OUTPUTS[7], &segregation);

    let status_lanes = release_hold_reject_lanes();
    write_part(OUTPUTS[8], &status_lanes);

    let camera_bridge = evidence_camera_bridge();
    write_part(OUTPUTS[9], &camera_bridge);

    let keepouts = robot_service_keepouts();
    write_part(OUTPUTS[10], &keepouts);

    let assembly = deck
        + transfer_tray
        + coupon_grid
        + shadow_blocks
        + indicator_lands
        + flow_witnesses
        + traceability
        + segregation
        + status_lanes
        + camera_bridge
        + keepouts;
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed UV/H2O2 decontamination shadow-mapping coupon station:");
    println!(
        "  Sealed tray/deck:            {DECK_X:.0}mm x {DECK_Y:.0}mm deck with {TRAY_X:.0}mm x {TRAY_Y:.0}mm sealed transfer tray and {TRANSFER_PORT_D:.0}mm port placeholder"
    );
    println!(
        "  Coupon mapping:              {COUPON_ROWS} x {COUPON_COLS} coupon grid ({COUPON_SLOTS} coupons), {COUPON_DATUM_PINS} datum pins, {SHADOW_TEST_BLOCKS} shadow-mask blocks, {INDICATOR_CARD_COUNT} exposure indicator card lands, {UV_DOSE_CHIP_LANDS} UV dose-chip lands"
    );
    println!(
        "  H2O2/VHP witness interfaces: {FLOW_WITNESS_COUNT} witness placeholders, {FLOW_VANE_COUNT} flow vanes, {FLOW_VECTOR_BAR_X:.0}mm vector bar"
    );
    println!(
        "  Evidence and traceability:   {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {CAMERA_PODS} camera pods, {LED_BARS} LED evidence bars"
    );
    println!(
        "  Disposition controls:        clean/used wall, {STATUS_LANES} release/hold/reject lanes, {} status pockets",
        STATUS_LANES * STATUS_SLOTS_PER_LANE
    );
    println!(
        "  Keepouts:                    {ROBOT_FRONT_KEEP_OUT_Y:.0}mm front robot, {SERVICE_REAR_KEEP_OUT_Y:.0}mm rear service, {LEFT_TRANSFER_KEEP_OUT_X:.0}mm transfer, {RIGHT_SERVICE_KEEP_OUT_X:.0}mm right service, {ROBOT_Z_CLEARANCE:.0}mm Z clearance"
    );
    println!("  Required feature groups:     {}", REQUIRED_FEATURES.len());
    println!("  Output prefix:               {OUTPUT_PREFIX}");
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_deck() -> Part {
    let deck = centered_cube("uv_h2o2_station_base_deck", DECK_X, DECK_Y, DECK_Z);
    deck - module_sockets() - mounting_holes() - drain_gutters()
        + perimeter_rim()
        + station_fiducials()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("uv_h2o2_station_module_sockets");
    for module in layout_envelopes() {
        sockets = sockets
            + top_recess(
                format!("uv_h2o2_station_{}_socket", module.name),
                module.center,
                module.x + 16.0,
                module.y + 16.0,
                4.0,
            );
    }
    sockets
}

fn perimeter_rim() -> Part {
    let front = centered_cube("uv_h2o2_station_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube("uv_h2o2_station_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube("uv_h2o2_station_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube("uv_h2o2_station_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("uv_h2o2_station_mounting_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("uv_h2o2_station_mounting_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                32,
            )
            .translate(x, y, 0.0);
    }
    holes
}

fn drain_gutters() -> Part {
    let front = centered_cube(
        "uv_h2o2_station_front_condensate_gutter",
        DECK_X - 160.0,
        12.0,
        6.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 88.0, DECK_Z / 2.0 - 2.0);
    let rear = centered_cube(
        "uv_h2o2_station_rear_condensate_gutter",
        DECK_X - 160.0,
        12.0,
        6.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 88.0, DECK_Z / 2.0 - 2.0);
    let port = centered_cylinder(
        "uv_h2o2_station_condensate_drain_port_placeholder",
        8.0,
        DECK_Z + 2.0,
        32,
    )
    .translate(DECK_X / 2.0 - 106.0, -DECK_Y / 2.0 + 112.0, 0.0);
    front + rear + port
}

fn station_fiducials() -> Part {
    let mut fiducials = Part::empty("uv_h2o2_station_robot_fiducials");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 104.0, -DECK_Y / 2.0 + 108.0),
        (DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 108.0),
        (-DECK_X / 2.0 + 104.0, DECK_Y / 2.0 - 108.0),
        (DECK_X / 2.0 - 104.0, DECK_Y / 2.0 - 108.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(format!("uv_h2o2_station_fiducial_{i}")).translate(
                x,
                y,
                DECK_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn sealed_transfer_tray() -> Part {
    let outer = centered_cube(
        "uv_h2o2_sealed_transfer_tray_outer_shell",
        TRAY_X,
        TRAY_Y,
        TRAY_Z,
    )
    .translate(0.0, 0.0, DECK_Z + TRAY_Z / 2.0);
    let cavity = centered_cube(
        "uv_h2o2_sealed_transfer_tray_open_cavity",
        TRAY_X - 2.0 * TRAY_WALL,
        TRAY_Y - 2.0 * TRAY_WALL,
        TRAY_Z - TRAY_FLOOR_Z + 0.4,
    )
    .translate(
        0.0,
        0.0,
        DECK_Z + TRAY_FLOOR_Z + (TRAY_Z - TRAY_FLOOR_Z) / 2.0 + 0.2,
    );
    let transfer_port = centered_cylinder(
        "uv_h2o2_sealed_transfer_tray_alpha_beta_port_placeholder",
        TRANSFER_PORT_D / 2.0,
        TRAY_WALL + 8.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -TRAY_X / 2.0 + 92.0,
        -TRAY_Y / 2.0 + 5.0,
        DECK_Z + TRAY_Z / 2.0,
    );

    outer - cavity - transfer_port + tray_gasket_lands() + tray_latches() + tray_handles()
}

fn tray_gasket_lands() -> Part {
    let z = DECK_Z + TRAY_Z + 3.0;
    let front = centered_cube(
        "uv_h2o2_transfer_tray_front_gasket_land",
        TRAY_X - 78.0,
        GASKET_W,
        6.0,
    )
    .translate(0.0, -TRAY_Y / 2.0 + TRAY_WALL / 2.0, z);
    let rear = centered_cube(
        "uv_h2o2_transfer_tray_rear_gasket_land",
        TRAY_X - 78.0,
        GASKET_W,
        6.0,
    )
    .translate(0.0, TRAY_Y / 2.0 - TRAY_WALL / 2.0, z);
    let left = centered_cube(
        "uv_h2o2_transfer_tray_left_gasket_land",
        GASKET_W,
        TRAY_Y - 78.0,
        6.0,
    )
    .translate(-TRAY_X / 2.0 + TRAY_WALL / 2.0, 0.0, z);
    let right = centered_cube(
        "uv_h2o2_transfer_tray_right_gasket_land",
        GASKET_W,
        TRAY_Y - 78.0,
        6.0,
    )
    .translate(TRAY_X / 2.0 - TRAY_WALL / 2.0, 0.0, z);
    front + rear + left + right
}

fn tray_latches() -> Part {
    let mut latches = Part::empty("uv_h2o2_transfer_tray_latch_bosses");
    for i in 0..LATCH_COUNT {
        let along_x = i < LATCH_COUNT / 2;
        let pair = if along_x { i } else { i - LATCH_COUNT / 2 };
        let x = if along_x {
            -TRAY_X / 2.0 + 245.0 + pair as f64 * 260.0
        } else if pair < 2 {
            -TRAY_X / 2.0 + 90.0
        } else {
            TRAY_X / 2.0 - 90.0
        };
        let y = if along_x {
            if pair % 2 == 0 {
                -TRAY_Y / 2.0 + 22.0
            } else {
                TRAY_Y / 2.0 - 22.0
            }
        } else {
            -TRAY_Y / 2.0 + 210.0 + (pair % 2) as f64 * 340.0
        };
        latches = latches
            + centered_cube(
                format!("uv_h2o2_transfer_tray_over_center_latch_land_{i}"),
                76.0,
                24.0,
                18.0,
            )
            .translate(x, y, DECK_Z + TRAY_Z + 9.0);
    }
    latches
}

fn tray_handles() -> Part {
    let left = centered_cube(
        "uv_h2o2_transfer_tray_left_gloved_handle",
        22.0,
        138.0,
        34.0,
    )
    .translate(-TRAY_X / 2.0 - 11.0, 0.0, DECK_Z + TRAY_Z / 2.0);
    let right = centered_cube(
        "uv_h2o2_transfer_tray_right_gloved_handle",
        22.0,
        138.0,
        34.0,
    )
    .translate(TRAY_X / 2.0 + 11.0, 0.0, DECK_Z + TRAY_Z / 2.0);
    left + right
}

fn coupon_grid_carrier() -> Part {
    let z_mid = DECK_Z + COUPON_GRID_Z / 2.0;
    let mut carrier = centered_cube(
        "uv_h2o2_coupon_grid_carrier_plate",
        COUPON_GRID_X,
        COUPON_GRID_Y,
        COUPON_GRID_Z,
    )
    .translate(COUPON_CENTER.0, COUPON_CENTER.1, z_mid);

    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let (x, y) = coupon_slot_center(row, col);
            let slot = centered_cube(
                format!("uv_h2o2_coupon_grid_slot_r{row}_c{col}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                10.0,
            )
            .translate(
                COUPON_CENTER.0 + x,
                COUPON_CENTER.1 + y,
                DECK_Z + COUPON_GRID_Z - 4.8,
            );
            let clip = centered_cube(
                format!("uv_h2o2_coupon_grid_spring_clip_r{row}_c{col}"),
                COUPON_SLOT_X + 12.0,
                5.0,
                8.0,
            )
            .translate(
                COUPON_CENTER.0 + x,
                COUPON_CENTER.1 + y + COUPON_SLOT_Y / 2.0 + 5.0,
                DECK_Z + COUPON_GRID_Z + 4.0,
            );
            carrier = carrier - slot + clip;
        }
    }

    carrier + coupon_datum_pins() + coupon_grid_row_column_rails()
}

fn coupon_datum_pins() -> Part {
    let mut pins = Part::empty("uv_h2o2_coupon_grid_datum_pins");
    for (i, (x, y)) in [
        (-COUPON_GRID_X / 2.0 + 32.0, -COUPON_GRID_Y / 2.0 + 32.0),
        (COUPON_GRID_X / 2.0 - 32.0, -COUPON_GRID_Y / 2.0 + 32.0),
        (-COUPON_GRID_X / 2.0 + 32.0, COUPON_GRID_Y / 2.0 - 32.0),
        (COUPON_GRID_X / 2.0 - 32.0, COUPON_GRID_Y / 2.0 - 32.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(format!("uv_h2o2_coupon_grid_datum_pin_{i}"), 7.0, 22.0, 32)
                .translate(
                    COUPON_CENTER.0 + x,
                    COUPON_CENTER.1 + y,
                    DECK_Z + COUPON_GRID_Z + 11.0,
                );
    }
    pins
}

fn coupon_grid_row_column_rails() -> Part {
    let mut rails = Part::empty("uv_h2o2_coupon_grid_row_column_rails");
    for row in 0..=COUPON_ROWS {
        let y = COUPON_CENTER.1 + (row as f64 - COUPON_ROWS as f64 / 2.0) * COUPON_PITCH_Y
            - COUPON_PITCH_Y / 2.0;
        rails = rails
            + centered_cube(
                format!("uv_h2o2_coupon_grid_row_separation_rail_{row}"),
                COUPON_GRID_X - 54.0,
                4.0,
                8.0,
            )
            .translate(COUPON_CENTER.0, y, DECK_Z + COUPON_GRID_Z + 4.0);
    }
    for col in 0..=COUPON_COLS {
        let x = COUPON_CENTER.0 + (col as f64 - COUPON_COLS as f64 / 2.0) * COUPON_PITCH_X
            - COUPON_PITCH_X / 2.0;
        rails = rails
            + centered_cube(
                format!("uv_h2o2_coupon_grid_column_separation_rail_{col}"),
                4.0,
                COUPON_GRID_Y - 58.0,
                8.0,
            )
            .translate(x, COUPON_CENTER.1, DECK_Z + COUPON_GRID_Z + 4.0);
    }
    rails
}

fn shadow_mask_test_blocks() -> Part {
    let panel = centered_cube(
        "uv_h2o2_shadow_mask_block_index_panel",
        SHADOW_BLOCK_X,
        SHADOW_BLOCK_Y,
        SHADOW_BLOCK_Z,
    )
    .translate(
        SHADOW_CENTER.0,
        SHADOW_CENTER.1,
        DECK_Z + SHADOW_BLOCK_Z / 2.0,
    );
    let mut blocks = Part::empty("uv_h2o2_shadow_mask_blocks");
    for i in 0..SHADOW_TEST_BLOCKS {
        let row = i / 4;
        let col = i % 4;
        let local_x = (col as f64 - 1.5) * SHADOW_BLOCK_PITCH_X;
        let local_y = (0.5 - row as f64) * SHADOW_BLOCK_PITCH_Y;
        let block_z = SHADOW_BLOCK_MIN_Z + i as f64 * SHADOW_BLOCK_STEP_Z;
        let base = centered_cube(
            format!("uv_h2o2_shadow_mask_block_{i}_riser"),
            SHADOW_BLOCK_BASE_X,
            SHADOW_BLOCK_BASE_Y,
            block_z,
        )
        .translate(
            SHADOW_CENTER.0 + local_x,
            SHADOW_CENTER.1 + local_y,
            DECK_Z + SHADOW_BLOCK_Z + block_z / 2.0,
        );
        let overhang = centered_cube(
            format!("uv_h2o2_shadow_mask_block_{i}_occluding_overhang"),
            SHADOW_BLOCK_BASE_X + 34.0,
            SHADOW_BLOCK_BASE_Y / 2.0,
            8.0,
        )
        .translate(
            SHADOW_CENTER.0 + local_x + 10.0,
            SHADOW_CENTER.1 + local_y + SHADOW_UNDERCUT_GAP,
            DECK_Z + SHADOW_BLOCK_Z + block_z + 4.0,
        );
        let coupon_shadow_slot = centered_cube(
            format!("uv_h2o2_shadow_mask_block_{i}_coupon_shadow_gap"),
            SHADOW_BLOCK_BASE_X - 16.0,
            SHADOW_UNDERCUT_GAP,
            9.0,
        )
        .translate(
            SHADOW_CENTER.0 + local_x,
            SHADOW_CENTER.1 + local_y - SHADOW_BLOCK_BASE_Y / 2.0 + 13.0,
            DECK_Z + SHADOW_BLOCK_Z + 6.0,
        );
        blocks = blocks + base + overhang - coupon_shadow_slot;
    }
    panel + blocks + shadow_mask_reference_steps()
}

fn shadow_mask_reference_steps() -> Part {
    let mut steps = Part::empty("uv_h2o2_shadow_mask_reference_step_gauges");
    for i in 0..4 {
        steps = steps
            + centered_cube(
                format!("uv_h2o2_shadow_mask_reference_step_{i}"),
                46.0,
                18.0,
                8.0 + i as f64 * 6.0,
            )
            .translate(
                SHADOW_CENTER.0 - SHADOW_BLOCK_X / 2.0 + 46.0 + i as f64 * 54.0,
                SHADOW_CENTER.1 - SHADOW_BLOCK_Y / 2.0 + 24.0,
                DECK_Z + SHADOW_BLOCK_Z + (8.0 + i as f64 * 6.0) / 2.0,
            );
    }
    steps
}

fn exposure_indicator_card_lands() -> Part {
    let mut panel = centered_cube(
        "uv_h2o2_exposure_indicator_card_land_panel",
        INDICATOR_PANEL_X,
        INDICATOR_PANEL_Y,
        INDICATOR_PANEL_Z,
    )
    .translate(
        INDICATOR_CENTER.0,
        INDICATOR_CENTER.1,
        DECK_Z + INDICATOR_PANEL_Z / 2.0,
    );

    for row in 0..INDICATOR_ROWS {
        for col in 0..INDICATOR_COLS {
            let x = INDICATOR_CENTER.0
                + (col as f64 - (INDICATOR_COLS as f64 - 1.0) / 2.0) * INDICATOR_PITCH_X;
            let y = INDICATOR_CENTER.1
                + ((INDICATOR_ROWS as f64 - 1.0) / 2.0 - row as f64) * INDICATOR_PITCH_Y;
            panel = panel
                - top_recess(
                    format!("uv_h2o2_exposure_indicator_card_recess_r{row}_c{col}"),
                    (x, y),
                    INDICATOR_LAND_X,
                    INDICATOR_LAND_Y,
                    4.0,
                )
                + centered_cube(
                    format!("uv_h2o2_exposure_indicator_card_retainer_r{row}_c{col}"),
                    INDICATOR_LAND_X + 8.0,
                    4.0,
                    7.0,
                )
                .translate(
                    x,
                    y - INDICATOR_LAND_Y / 2.0 - 5.0,
                    DECK_Z + INDICATOR_PANEL_Z + 3.5,
                );
        }
    }

    panel + uv_dose_chip_lands()
}

fn uv_dose_chip_lands() -> Part {
    let mut lands = Part::empty("uv_h2o2_uv_dose_chip_lands");
    for i in 0..UV_DOSE_CHIP_LANDS {
        let x = INDICATOR_CENTER.0 - INDICATOR_PANEL_X / 2.0
            + 38.0
            + i as f64 * ((INDICATOR_PANEL_X - 76.0) / (UV_DOSE_CHIP_LANDS as f64 - 1.0));
        lands = lands
            + centered_cube(format!("uv_h2o2_uv_dose_chip_land_{i}"), 20.0, 18.0, 5.0).translate(
                x,
                INDICATOR_CENTER.1 - INDICATOR_PANEL_Y / 2.0 + 22.0,
                DECK_Z + INDICATOR_PANEL_Z + 2.5,
            );
    }
    lands
}

fn vhp_h2o2_flow_witness_placeholders() -> Part {
    let panel = centered_cube(
        "uv_h2o2_vhp_flow_witness_panel",
        FLOW_PANEL_X,
        FLOW_PANEL_Y,
        FLOW_PANEL_Z,
    )
    .translate(FLOW_CENTER.0, FLOW_CENTER.1, DECK_Z + FLOW_PANEL_Z / 2.0);
    panel
        + h2o2_witness_discs()
        + flow_vanes()
        + flow_vector_bar()
        + humidity_temperature_stub_lands()
}

fn h2o2_witness_discs() -> Part {
    let mut discs = Part::empty("uv_h2o2_h2o2_witness_disc_placeholders");
    for i in 0..FLOW_WITNESS_COUNT {
        let x = FLOW_CENTER.0
            + (i as f64 - (FLOW_WITNESS_COUNT as f64 - 1.0) / 2.0) * FLOW_WITNESS_PITCH_X;
        let y = FLOW_CENTER.1 + if i % 2 == 0 { 36.0 } else { -6.0 };
        discs = discs
            + centered_cylinder(
                format!("uv_h2o2_h2o2_witness_disc_placeholder_{i}"),
                H2O2_WITNESS_D / 2.0,
                7.0,
                36,
            )
            .translate(x, y, DECK_Z + FLOW_PANEL_Z + 3.5)
            + centered_cube(
                format!("uv_h2o2_h2o2_witness_barcode_tab_{i}"),
                30.0,
                10.0,
                4.0,
            )
            .translate(x, y - 23.0, DECK_Z + FLOW_PANEL_Z + 2.0);
    }
    discs
}

fn flow_vanes() -> Part {
    let mut vanes = Part::empty("uv_h2o2_flow_witness_vane_placeholders");
    for i in 0..FLOW_VANE_COUNT {
        let x = FLOW_CENTER.0 + (i as f64 - (FLOW_VANE_COUNT as f64 - 1.0) / 2.0) * 68.0;
        vanes = vanes
            + centered_cube(format!("uv_h2o2_flow_vane_flag_{i}"), 8.0, 76.0, 38.0)
                .rotate(0.0, 0.0, -22.0 + i as f64 * 11.0)
                .translate(x, FLOW_CENTER.1 - 62.0, DECK_Z + FLOW_PANEL_Z + 19.0);
    }
    vanes
}

fn flow_vector_bar() -> Part {
    let bar = centered_cube(
        "uv_h2o2_flow_vector_reference_bar",
        FLOW_VECTOR_BAR_X,
        12.0,
        10.0,
    )
    .translate(
        FLOW_CENTER.0,
        FLOW_CENTER.1 + FLOW_PANEL_Y / 2.0 - 28.0,
        DECK_Z + FLOW_PANEL_Z + 5.0,
    );
    let arrow = centered_cube(
        "uv_h2o2_flow_vector_arrow_head_placeholder",
        32.0,
        32.0,
        10.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        FLOW_CENTER.0 + FLOW_VECTOR_BAR_X / 2.0 + 16.0,
        FLOW_CENTER.1 + FLOW_PANEL_Y / 2.0 - 28.0,
        DECK_Z + FLOW_PANEL_Z + 5.0,
    );
    bar + arrow
}

fn humidity_temperature_stub_lands() -> Part {
    let left = centered_cube(
        "uv_h2o2_flow_witness_humidity_probe_placeholder",
        54.0,
        24.0,
        14.0,
    )
    .translate(
        FLOW_CENTER.0 - FLOW_PANEL_X / 2.0 + 48.0,
        FLOW_CENTER.1 - FLOW_PANEL_Y / 2.0 + 34.0,
        DECK_Z + FLOW_PANEL_Z + 7.0,
    );
    let right = centered_cube(
        "uv_h2o2_flow_witness_temperature_probe_placeholder",
        54.0,
        24.0,
        14.0,
    )
    .translate(
        FLOW_CENTER.0 + FLOW_PANEL_X / 2.0 - 48.0,
        FLOW_CENTER.1 - FLOW_PANEL_Y / 2.0 + 34.0,
        DECK_Z + FLOW_PANEL_Z + 7.0,
    );
    left + right
}

fn barcode_certificate_lands() -> Part {
    let mut panel = centered_cube(
        "uv_h2o2_barcode_certificate_land_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, DECK_Z + TRACE_PANEL_Z / 2.0);

    for i in 0..BARCODE_LANDS {
        let row = i / 3;
        let col = i % 3;
        let x = TRACE_CENTER.0 - TRACE_PANEL_X / 2.0 + 62.0 + col as f64 * 92.0;
        let y = TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0 - 34.0 - row as f64 * 34.0;
        panel = panel
            - top_recess(
                format!("uv_h2o2_traceability_barcode_land_recess_{i}"),
                (x, y),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                2.8,
            )
            + barcode_stripes(i, x, y);
    }

    for i in 0..CERTIFICATE_LANDS {
        let x = TRACE_CENTER.0 - 112.0 + i as f64 * 112.0;
        let y = TRACE_CENTER.1 - TRACE_PANEL_Y / 2.0 + 44.0;
        panel = panel
            - top_recess(
                format!("uv_h2o2_certificate_land_recess_{i}"),
                (x, y),
                CERT_LAND_X,
                CERT_LAND_Y,
                3.0,
            )
            + centered_cube(
                format!("uv_h2o2_certificate_corner_clip_{i}"),
                CERT_LAND_X + 10.0,
                5.0,
                5.0,
            )
            .translate(x, y + CERT_LAND_Y / 2.0 + 5.0, DECK_Z + TRACE_PANEL_Z + 2.5);
    }

    panel
}

fn barcode_stripes(index: usize, x: f64, y: f64) -> Part {
    let mut stripes = Part::empty(format!("uv_h2o2_traceability_barcode_stripes_{index}"));
    for stripe in 0..6 {
        let stripe_w = if stripe % 2 == 0 { 2.0 } else { 4.0 };
        stripes = stripes
            + centered_cube(
                format!("uv_h2o2_traceability_barcode_{index}_stripe_{stripe}"),
                stripe_w,
                BARCODE_LAND_Y - 6.0,
                2.0,
            )
            .translate(
                x - BARCODE_LAND_X / 2.0 + 12.0 + stripe as f64 * 8.0,
                y,
                DECK_Z + TRACE_PANEL_Z + 1.0,
            );
    }
    stripes
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "uv_h2o2_clean_used_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X,
        0.0,
        DECK_Z + SEGREGATION_WALL_Z / 2.0,
    );
    let pass_gate = centered_cube(
        "uv_h2o2_clean_used_controlled_pass_gate_cutout",
        SEGREGATION_WALL_X + 4.0,
        SEGREGATION_PASS_GATE_Y,
        46.0,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X,
        -330.0,
        DECK_Z + SEGREGATION_WALL_Z / 2.0,
    );
    let clean_buffer = centered_cube(
        "uv_h2o2_clean_incoming_coupon_buffer_lane",
        CLEAN_BUFFER_X,
        14.0,
        20.0,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X - SEGREGATION_WALL_X / 2.0 - CLEAN_BUFFER_X / 2.0 - 10.0,
        330.0,
        DECK_Z + 10.0,
    );
    let used_buffer = centered_cube(
        "uv_h2o2_used_evidence_buffer_lane",
        USED_BUFFER_X,
        14.0,
        20.0,
    )
    .translate(
        SEGREGATION_WALL_CENTER_X + SEGREGATION_WALL_X / 2.0 + USED_BUFFER_X / 2.0 + 10.0,
        -330.0,
        DECK_Z + 10.0,
    );
    let wipe_gap_gauge = centered_cube("uv_h2o2_clean_used_wipe_gap_gauge", 56.0, 12.0, 8.0)
        .translate(
            SEGREGATION_WALL_CENTER_X + 44.0,
            0.0,
            DECK_Z + SEGREGATION_WALL_Z + 4.0,
        );

    wall - pass_gate + clean_buffer + used_buffer + wipe_gap_gauge
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "uv_h2o2_release_hold_reject_lane_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(
        STATUS_CENTER.0,
        STATUS_CENTER.1,
        DECK_Z + STATUS_PANEL_Z / 2.0,
    );
    let mut lanes = Part::empty("uv_h2o2_release_hold_reject_lane_pockets");
    for (lane, lane_name) in STATUS_NAMES.iter().enumerate() {
        let y = STATUS_CENTER.1
            + ((STATUS_LANES as f64 - 1.0) / 2.0 - lane as f64) * STATUS_LANE_PITCH_Y;
        let lane_rail = centered_cube(
            format!("uv_h2o2_{lane_name}_lane_front_rail"),
            STATUS_PANEL_X - 40.0,
            5.0,
            12.0,
        )
        .translate(
            STATUS_CENTER.0,
            y - STATUS_SLOT_Y / 2.0 - 9.0,
            DECK_Z + STATUS_PANEL_Z + 6.0,
        );
        lanes = lanes + lane_rail;
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = STATUS_CENTER.0
                + (slot as f64 - (STATUS_SLOTS_PER_LANE as f64 - 1.0) / 2.0) * 116.0;
            lanes = lanes
                - top_recess(
                    format!("uv_h2o2_{lane_name}_status_ticket_recess_{slot}"),
                    (x, y),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    5.0,
                )
                + centered_cube(
                    format!("uv_h2o2_{lane_name}_status_ticket_stop_{slot}"),
                    STATUS_SLOT_X,
                    5.0,
                    8.0,
                )
                .translate(
                    x,
                    y + STATUS_SLOT_Y / 2.0 + 5.0,
                    DECK_Z + STATUS_PANEL_Z + 4.0,
                );
        }
    }
    base + lanes
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "uv_h2o2_evidence_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_UNDERSIDE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + CAMERA_POST_X / 2.0,
        DECK_Y / 2.0 - 150.0,
        DECK_Z + CAMERA_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "uv_h2o2_evidence_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_UNDERSIDE_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0 - CAMERA_POST_X / 2.0,
        DECK_Y / 2.0 - 150.0,
        DECK_Z + CAMERA_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "uv_h2o2_evidence_camera_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 150.0,
        DECK_Z + CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    left_post
        + right_post
        + beam
        + camera_pods()
        + led_evidence_bars()
        + calibration_card_overlook()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("uv_h2o2_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = (i as f64 - (CAMERA_PODS as f64 - 1.0) / 2.0) * 250.0;
        let y = DECK_Y / 2.0 - 150.0;
        let z = DECK_Z + CAMERA_UNDERSIDE_Z - 22.0;
        pods = pods
            + centered_cube(format!("uv_h2o2_evidence_camera_pod_{i}"), 58.0, 42.0, 34.0)
                .translate(x, y, z)
            + centered_cylinder(format!("uv_h2o2_evidence_camera_lens_{i}"), 10.0, 16.0, 32)
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y - CAMERA_BRIDGE_Y / 2.0 - 8.0, z)
            + centered_cube(
                format!("uv_h2o2_evidence_camera_barcode_illumination_shroud_{i}"),
                44.0,
                8.0,
                16.0,
            )
            .translate(x, y - CAMERA_BRIDGE_Y / 2.0 - 18.0, z - 10.0);
    }
    pods
}

fn led_evidence_bars() -> Part {
    let mut bars = Part::empty("uv_h2o2_evidence_bridge_led_bars");
    for side in 0..LED_BARS {
        let y_offset = if side == 0 { -36.0 } else { 36.0 };
        bars = bars
            + centered_cube(
                format!("uv_h2o2_evidence_bridge_led_bar_{side}"),
                CAMERA_BRIDGE_X - 160.0,
                10.0,
                10.0,
            )
            .translate(
                0.0,
                DECK_Y / 2.0 - 150.0 + y_offset,
                DECK_Z + CAMERA_UNDERSIDE_Z - 42.0,
            );
    }
    bars
}

fn calibration_card_overlook() -> Part {
    centered_cube(
        "uv_h2o2_evidence_bridge_calibration_card_overlook_land",
        112.0,
        24.0,
        8.0,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + 128.0,
        DECK_Y / 2.0 - 150.0 - CAMERA_BRIDGE_Y / 2.0 - 20.0,
        DECK_Z + CAMERA_UNDERSIDE_Z - 28.0,
    )
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        "uv_h2o2_front_robot_pick_sweep_keepout",
        DECK_X - 210.0,
        42.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_FRONT_KEEP_OUT_Y / 2.0,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    let rear_service = keepout_box(
        "uv_h2o2_rear_service_filter_lamp_keepout",
        DECK_X - 240.0,
        42.0,
        180.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + SERVICE_REAR_KEEP_OUT_Y / 2.0,
        DECK_Z + 90.0,
    );
    let left_transfer = keepout_box(
        "uv_h2o2_left_sealed_transfer_cart_keepout",
        42.0,
        DECK_Y - 180.0,
        160.0,
    )
    .translate(
        -DECK_X / 2.0 - LEFT_TRANSFER_KEEP_OUT_X / 2.0,
        0.0,
        DECK_Z + 80.0,
    );
    let right_service = keepout_box(
        "uv_h2o2_right_vhp_service_hose_keepout",
        42.0,
        DECK_Y - 220.0,
        160.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        DECK_Z + 80.0,
    );
    let z_gauge = centered_cube(
        "uv_h2o2_robot_z_clearance_gauge",
        72.0,
        72.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 74.0,
        DECK_Y / 2.0 - 78.0,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    front_robot + rear_service + left_transfer + right_service + z_gauge
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let shell = centered_cube(format!("{name}_envelope"), x, y, z);
    let hollow = centered_cube(format!("{name}_hollow"), x - 12.0, y - 12.0, z - 16.0);
    let rail = centered_cube(
        format!("{name}_label_land"),
        x * 0.45,
        14.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -y / 2.0 + 12.0, z / 2.0 + KEEP_OUT_RAIL_Z / 2.0);
    shell - hollow + rail
}

fn top_recess(name: impl Into<String>, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(center.0, center.1, DECK_Z - depth / 2.0 + 0.1)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let pad = centered_cylinder(format!("{name}_pad"), 15.0, 3.0, 48);
    let cross_x = centered_cube(format!("{name}_cross_x"), 24.0, 3.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 24.0, 4.0);
    pad - cross_x - cross_y
}

fn coupon_slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X,
        ((COUPON_ROWS as f64 - 1.0) / 2.0 - row as f64) * COUPON_PITCH_Y,
    )
}

fn layout_envelopes() -> [ModuleEnvelope; 6] {
    [
        ModuleEnvelope {
            name: "coupon_grid_carrier",
            center: COUPON_CENTER,
            x: COUPON_GRID_X,
            y: COUPON_GRID_Y,
        },
        ModuleEnvelope {
            name: "shadow_mask_blocks",
            center: SHADOW_CENTER,
            x: SHADOW_BLOCK_X,
            y: SHADOW_BLOCK_Y,
        },
        ModuleEnvelope {
            name: "indicator_card_lands",
            center: INDICATOR_CENTER,
            x: INDICATOR_PANEL_X,
            y: INDICATOR_PANEL_Y,
        },
        ModuleEnvelope {
            name: "flow_witnesses",
            center: FLOW_CENTER,
            x: FLOW_PANEL_X,
            y: FLOW_PANEL_Y,
        },
        ModuleEnvelope {
            name: "barcode_certificates",
            center: TRACE_CENTER,
            x: TRACE_PANEL_X,
            y: TRACE_PANEL_Y,
        },
        ModuleEnvelope {
            name: "status_lanes",
            center: STATUS_CENTER,
            x: STATUS_PANEL_X,
            y: STATUS_PANEL_Y,
        },
    ]
}

fn assert_layout() {
    let envelopes = layout_envelopes();
    for envelope in envelopes {
        assert!(
            envelope.fits_on_deck(),
            "{} does not fit on deck",
            envelope.name
        );
    }
    for i in 0..envelopes.len() {
        for j in i + 1..envelopes.len() {
            assert!(
                !envelopes[i].overlaps(envelopes[j]),
                "{} overlaps {}",
                envelopes[i].name,
                envelopes[j].name
            );
        }
    }
    assert!(TRAY_X < DECK_X - 2.0 * RIM_W);
    assert!(TRAY_Y < DECK_Y - 2.0 * RIM_W);
    assert!(CAMERA_UNDERSIDE_Z > SHADOW_BLOCK_Z + SHADOW_BLOCK_MIN_Z + SHADOW_BLOCK_STEP_Z * 7.0);
    assert!(ROBOT_Z_CLEARANCE > CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z);
    assert!(
        SEGREGATION_WALL_CENTER_X + SEGREGATION_WALL_X / 2.0 < FLOW_CENTER.0 - FLOW_PANEL_X / 2.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_matches_exports() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_requested_station_scope() {
        for feature in [
            "coupon_grid_carrier",
            "shadow_mask_test_blocks",
            "exposure_indicator_card_lands",
            "vhp_h2o2_flow_witness_placeholders",
            "barcode_certificate_lands",
            "clean_used_segregation",
            "release_hold_reject_lanes",
            "evidence_camera_bridge",
            "robot_keepouts",
            "service_keepouts",
            "sealed_transfer_tray",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 11);
    }

    #[test]
    fn layout_constraints_hold() {
        assert_layout();
    }

    #[test]
    fn coupon_and_evidence_capacity_is_traceable() {
        assert_eq!(COUPON_SLOTS, COUPON_ROWS * COUPON_COLS);
        assert_eq!(COUPON_SLOTS, 24);
        assert_eq!(INDICATOR_CARD_COUNT, INDICATOR_ROWS * INDICATOR_COLS);
        assert!(BARCODE_LANDS >= INDICATOR_CARD_COUNT);
        assert_eq!(CERTIFICATE_LANDS, STATUS_LANES);
    }

    #[test]
    fn h2o2_and_shadow_mapping_geometry_has_clearance() {
        assert!(FLOW_WITNESS_COUNT >= SHADOW_TEST_BLOCKS);
        assert!(H2O2_WITNESS_D < FLOW_WITNESS_PITCH_X / 2.0);
        assert!(SHADOW_UNDERCUT_GAP > 10.0);
        assert!(CAMERA_PODS >= 4);
        assert_eq!(COUPON_DATUM_PINS, 4);
    }

    #[test]
    fn station_assembly_builds_all_part_groups() {
        let parts = [
            base_deck(),
            sealed_transfer_tray(),
            coupon_grid_carrier(),
            shadow_mask_test_blocks(),
            exposure_indicator_card_lands(),
            vhp_h2o2_flow_witness_placeholders(),
            barcode_certificate_lands(),
            clean_used_segregation(),
            release_hold_reject_lanes(),
            evidence_camera_bridge(),
            robot_service_keepouts(),
        ];
        assert_eq!(parts.len(), OUTPUTS.len() - 1);
    }
}
