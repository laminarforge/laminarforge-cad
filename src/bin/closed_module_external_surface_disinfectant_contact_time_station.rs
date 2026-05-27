use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module external surface disinfectant contact-time evidence station.
//
// Intent:
// - Hold a sealed culture module in a repeatable cradle while external surfaces
//   are wiped, kept visibly wet, timed, scanned, and dispositioned.
// - Keep removable witness coupons, wetness/contact-time tokens, disinfectant
//   cartridge pockets, wipe path gauges, certificate/barcode lands, and
//   release/hold/reject lanes physically separated and reviewable.
// - Reserve clean/used segregation and robot/service keepouts so the validation
//   station can be reviewed as a mechanical interface, not as a loose bench
//   procedure.
//
// This is product-architecture CAD for fit and evidence layout. It is not a
// disinfectant efficacy claim, contact-time protocol, material compatibility
// certification, or microbiology release method.

const OUTPUT_PREFIX: &str =
    "output/closed_module_external_surface_disinfectant_contact_time_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_module_external_surface_disinfectant_contact_time_station_base_deck.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_module_cradle.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_removable_surface_coupon_carrier.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_wetness_contact_time_token_lanes.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_disinfectant_cartridge_pockets.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_wipe_path_gauges.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_barcode_certificate_lands.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_clean_used_segregation.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_release_hold_reject_lanes.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_evidence_bridge.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_robot_service_keepouts.stl",
    "output/closed_module_external_surface_disinfectant_contact_time_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "module_cradle",
    "removable_surface_coupons",
    "wetness_contact_time_token_lanes",
    "disinfectant_cartridge_pockets",
    "wipe_path_gauges",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "clean_used_segregation",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 960.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CRADLE_CENTER: (f64, f64) = (-430.0, 150.0);
const CRADLE_X: f64 = 620.0;
const CRADLE_Y: f64 = 350.0;
const CRADLE_Z: f64 = 96.0;
const MODULE_ENVELOPE_X: f64 = 510.0;
const MODULE_ENVELOPE_Y: f64 = 250.0;
const MODULE_ENVELOPE_Z: f64 = 68.0;
const MODULE_SADDLES: usize = 2;
const CRADLE_LATCHES: usize = 6;
const CRADLE_DATUM_PINS: usize = 6;

const COUPON_CENTER: (f64, f64) = (-495.0, -225.0);
const COUPON_PANEL_X: f64 = 450.0;
const COUPON_PANEL_Y: f64 = 270.0;
const COUPON_PANEL_Z: f64 = 32.0;
const COUPON_ROWS: usize = 3;
const COUPON_COLS: usize = 6;
const COUPON_SLOTS: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 32.0;
const COUPON_PITCH_X: f64 = 62.0;
const COUPON_PITCH_Y: f64 = 62.0;
const COUPON_HANDLE_TABS: usize = 2;

const TOKEN_CENTER: (f64, f64) = (90.0, -250.0);
const TOKEN_PANEL_X: f64 = 660.0;
const TOKEN_PANEL_Y: f64 = 165.0;
const TOKEN_PANEL_Z: f64 = 30.0;
const WETNESS_TOKENS: [&str; 5] = [
    "dry_confirm",
    "wet_apply",
    "full_coverage",
    "rewet_needed",
    "dry_down",
];
const CONTACT_TOKENS: [&str; 6] = [
    "start",
    "one_min",
    "three_min",
    "five_min",
    "contact_met",
    "locked",
];
const TOKEN_SLOT_X: f64 = 76.0;
const TOKEN_SLOT_Y: f64 = 34.0;
const TOKEN_SLOT_Z: f64 = 10.0;
const TOKEN_PITCH_X: f64 = 94.0;

const WIPE_CENTER: (f64, f64) = (150.0, -5.0);
const WIPE_PANEL_X: f64 = 450.0;
const WIPE_PANEL_Y: f64 = 170.0;
const WIPE_PANEL_Z: f64 = 26.0;
const WIPE_LANES: usize = 4;
const WIPE_LANE_PITCH_Y: f64 = 34.0;
const WIPE_GAUGE_LENGTH: f64 = 370.0;
const WIPE_GAUGE_WIDTH: f64 = 18.0;
const WIPE_OVERLAP_STRIPS: usize = 5;
const WIPE_RADIUS_GAUGES: usize = 3;

const CARTRIDGE_CENTER: (f64, f64) = (500.0, 235.0);
const CARTRIDGE_DOCK_X: f64 = 400.0;
const CARTRIDGE_DOCK_Y: f64 = 240.0;
const CARTRIDGE_DOCK_Z: f64 = 62.0;
const DISINFECTANT_CARTRIDGES: usize = 4;
const CARTRIDGE_D: f64 = 42.0;
const CARTRIDGE_Z: f64 = 128.0;
const CARTRIDGE_PITCH_X: f64 = 82.0;
const WET_PORTS: usize = 6;

const TRACE_CENTER: (f64, f64) = (555.0, -55.0);
const TRACE_PANEL_X: f64 = 360.0;
const TRACE_PANEL_Y: f64 = 200.0;
const TRACE_PANEL_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 12;
const CERTIFICATE_LANDS: usize = 4;
const RFID_LANDS: usize = 4;

const SEGREGATION_WALL_X: f64 = 18.0;
const SEGREGATION_WALL_Y: f64 = 760.0;
const SEGREGATION_WALL_Z: f64 = 88.0;
const SEGREGATION_CENTER_X: f64 = -255.0;
const SEGREGATION_GATE_Y: f64 = 118.0;
const CLEAN_BUFFER_LANDS: usize = 4;
const USED_BUFFER_LANDS: usize = 5;

const STATUS_CENTER: (f64, f64) = (310.0, -397.0);
const STATUS_PANEL_X: f64 = 540.0;
const STATUS_PANEL_Y: f64 = 120.0;
const STATUS_PANEL_Z: f64 = 30.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_SLOT_X: f64 = 94.0;
const STATUS_SLOT_Y: f64 = 30.0;
const STATUS_LANE_PITCH_Y: f64 = 38.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 60.0);
const BRIDGE_SPAN_X: f64 = 1330.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 48.0;
const BRIDGE_UNDERSIDE_Z: f64 = 270.0;
const BRIDGE_BEAM_Z: f64 = 32.0;
const CAMERA_PODS: usize = 5;
const LED_BARS: usize = 4;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 360.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 270.0;
const LEFT_COUPON_SERVICE_KEEP_OUT_X: f64 = 220.0;
const RIGHT_CARTRIDGE_SERVICE_KEEP_OUT_X: f64 = 260.0;
const OVERHEAD_EVIDENCE_KEEP_OUT_Z: f64 = 380.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;

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

    let base = base_deck();
    write_part(OUTPUTS[0], &base);

    let cradle = module_cradle();
    write_part(OUTPUTS[1], &cradle);

    let coupons = removable_surface_coupon_carrier();
    write_part(OUTPUTS[2], &coupons);

    let tokens = wetness_contact_time_token_lanes();
    write_part(OUTPUTS[3], &tokens);

    let cartridges = disinfectant_cartridge_pockets();
    write_part(OUTPUTS[4], &cartridges);

    let gauges = wipe_path_gauges();
    write_part(OUTPUTS[5], &gauges);

    let traceability = barcode_certificate_lands();
    write_part(OUTPUTS[6], &traceability);

    let segregation = clean_used_segregation();
    write_part(OUTPUTS[7], &segregation);

    let status_lanes = release_hold_reject_lanes();
    write_part(OUTPUTS[8], &status_lanes);

    let bridge = evidence_bridge();
    write_part(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    write_part(OUTPUTS[10], &keepouts);

    let assembly = base
        + cradle
        + coupons
        + tokens
        + cartridges
        + gauges
        + traceability
        + segregation
        + status_lanes
        + bridge
        + keepouts;
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed-module external surface disinfectant contact-time evidence station:");
    println!(
        "  Station deck:                {DECK_X:.0}mm x {DECK_Y:.0}mm with contained rim, datum sockets, fiducials, drain gutters, and {OUTPUTS_LEN} STL outputs.",
        OUTPUTS_LEN = OUTPUTS.len()
    );
    println!(
        "  Module handling:             {MODULE_SADDLES} saddle positions, {CRADLE_LATCHES} latch lands, {CRADLE_DATUM_PINS} datum pins, {MODULE_ENVELOPE_X:.0}mm x {MODULE_ENVELOPE_Y:.0}mm sealed-module envelope."
    );
    println!(
        "  Evidence coupons/tokens:     {COUPON_ROWS} x {COUPON_COLS} removable surface coupon grid ({COUPON_SLOTS} coupons), {} wetness tokens, {} contact-time tokens, {WIPE_LANES} wipe path gauge lanes.",
        WETNESS_TOKENS.len(),
        CONTACT_TOKENS.len()
    );
    println!(
        "  Disinfectant and records:    {DISINFECTANT_CARTRIDGES} cartridge pockets, {WET_PORTS} keyed wet-port lands, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {RFID_LANDS} RFID/certificate witness pads."
    );
    println!(
        "  Disposition/segregation:     {STATUS_LANES} release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} token positions each, clean/used wall with {CLEAN_BUFFER_LANDS} clean buffer lands and {USED_BUFFER_LANDS} used evidence lands."
    );
    println!(
        "  Evidence and keepouts:       {CAMERA_PODS} camera pods, {LED_BARS} LED bars, front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, left coupon service {LEFT_COUPON_SERVICE_KEEP_OUT_X:.0}mm, right cartridge service {RIGHT_CARTRIDGE_SERVICE_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_EVIDENCE_KEEP_OUT_Z:.0}mm."
    );
    println!("  Required feature groups:     {}", REQUIRED_FEATURES.len());
    println!("  Output prefix:               {OUTPUT_PREFIX}");
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(STATUS_NAMES.len(), STATUS_LANES);
    assert_eq!(COUPON_SLOTS, COUPON_ROWS * COUPON_COLS);
    assert_eq!(COUPON_HANDLE_TABS, 2);
    assert!(WETNESS_TOKENS.len() >= 5);
    assert!(CONTACT_TOKENS.len() >= 6);
    assert!(BARCODE_LANDS >= STATUS_LANES * STATUS_SLOTS_PER_LANE);
    assert!(CERTIFICATE_LANDS >= DISINFECTANT_CARTRIDGES);
    assert!(RFID_LANDS >= STATUS_LANES);
    assert!(SEGREGATION_CENTER_X < COUPON_CENTER.0 + COUPON_PANEL_X / 2.0 + 22.0);
    assert!(SEGREGATION_CENTER_X > TOKEN_CENTER.0 - TOKEN_PANEL_X / 2.0 - 22.0);

    let modules = layout_envelopes();
    for module in modules {
        assert!(
            module.fits_on_deck(),
            "{} does not fit on deck",
            module.name
        );
    }
    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            assert!(
                !modules[i].overlaps(modules[j]),
                "{} overlaps {}",
                modules[i].name,
                modules[j].name
            );
        }
    }
}

fn layout_envelopes() -> [ModuleEnvelope; 7] {
    [
        ModuleEnvelope {
            name: "module_cradle",
            center: CRADLE_CENTER,
            x: CRADLE_X,
            y: CRADLE_Y,
        },
        ModuleEnvelope {
            name: "coupon_carrier",
            center: COUPON_CENTER,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        ModuleEnvelope {
            name: "token_lanes",
            center: TOKEN_CENTER,
            x: TOKEN_PANEL_X,
            y: TOKEN_PANEL_Y,
        },
        ModuleEnvelope {
            name: "wipe_path_gauges",
            center: WIPE_CENTER,
            x: WIPE_PANEL_X,
            y: WIPE_PANEL_Y,
        },
        ModuleEnvelope {
            name: "disinfectant_cartridges",
            center: CARTRIDGE_CENTER,
            x: CARTRIDGE_DOCK_X,
            y: CARTRIDGE_DOCK_Y,
        },
        ModuleEnvelope {
            name: "barcode_certificate_lands",
            center: TRACE_CENTER,
            x: TRACE_PANEL_X,
            y: TRACE_PANEL_Y,
        },
        ModuleEnvelope {
            name: "release_hold_reject_lanes",
            center: STATUS_CENTER,
            x: STATUS_PANEL_X,
            y: STATUS_PANEL_Y,
        },
    ]
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "external_surface_disinfectant_contact_time_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - module_sockets() - mounting_holes() - deck_drain_gutters()
        + perimeter_rim()
        + workflow_lane_datum_bars()
        + station_fiducials()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("external_surface_contact_time_module_sockets");
    for module in layout_envelopes() {
        sockets = sockets
            + top_recess(
                format!("external_surface_contact_time_{}_socket", module.name),
                module.center,
                module.x + 18.0,
                module.y + 18.0,
                4.0,
            );
    }
    sockets
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "external_surface_contact_time_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "external_surface_contact_time_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "external_surface_contact_time_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "external_surface_contact_time_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("external_surface_contact_time_mounting_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (SEGREGATION_CENTER_X, -DECK_Y / 2.0 + 58.0),
        (SEGREGATION_CENTER_X, DECK_Y / 2.0 - 58.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("external_surface_contact_time_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_drain_gutters() -> Part {
    let front_gutter = centered_cube(
        "external_surface_contact_time_front_wet_runoff_gutter",
        DECK_X - 180.0,
        12.0,
        6.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 92.0, DECK_Z - 3.0);
    let used_side_gutter = centered_cube(
        "external_surface_contact_time_used_side_wet_gutter",
        12.0,
        DECK_Y - 210.0,
        6.0,
    )
    .translate(SEGREGATION_CENTER_X + 54.0, 0.0, DECK_Z - 3.0);
    let drain = centered_cylinder(
        "external_surface_contact_time_low_point_drain_placeholder",
        8.0,
        DECK_Z + 4.0,
        32,
    )
    .translate(DECK_X / 2.0 - 112.0, -DECK_Y / 2.0 + 112.0, DECK_Z / 2.0);
    front_gutter + used_side_gutter + drain
}

fn workflow_lane_datum_bars() -> Part {
    let clean_lane = centered_cube(
        "external_surface_contact_time_clean_coupon_lane_datum_bar",
        10.0,
        DECK_Y - 150.0,
        6.0,
    )
    .translate(SEGREGATION_CENTER_X - 54.0, 0.0, DECK_Z + 3.0);
    let used_lane = centered_cube(
        "external_surface_contact_time_used_evidence_lane_datum_bar",
        10.0,
        DECK_Y - 150.0,
        6.0,
    )
    .translate(SEGREGATION_CENTER_X + 54.0, 0.0, DECK_Z + 3.0);
    let contact_lane = centered_cube(
        "external_surface_contact_time_timer_workflow_datum_bar",
        DECK_X - 260.0,
        8.0,
        6.0,
    )
    .translate(
        72.0,
        TOKEN_CENTER.1 + TOKEN_PANEL_Y / 2.0 + 24.0,
        DECK_Z + 3.0,
    );
    clean_lane + used_lane + contact_lane
}

fn station_fiducials() -> Part {
    let mut fiducials = Part::empty("external_surface_contact_time_robot_fiducials");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 104.0, -DECK_Y / 2.0 + 106.0),
        (DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 106.0),
        (-DECK_X / 2.0 + 104.0, DECK_Y / 2.0 - 106.0),
        (DECK_X / 2.0 - 104.0, DECK_Y / 2.0 - 106.0),
        (SEGREGATION_CENTER_X - 52.0, DECK_Y / 2.0 - 106.0),
        (SEGREGATION_CENTER_X + 52.0, -DECK_Y / 2.0 + 106.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(format!("external_surface_contact_time_fiducial_{i}")).translate(
                x,
                y,
                DECK_Z + 2.0,
            );
    }
    fiducials
}

fn module_cradle() -> Part {
    let base = centered_cube(
        "external_surface_contact_time_closed_module_cradle_base",
        CRADLE_X,
        CRADLE_Y,
        CRADLE_Z,
    )
    .translate(CRADLE_CENTER.0, CRADLE_CENTER.1, DECK_Z + CRADLE_Z / 2.0);
    let module_pocket = centered_cube(
        "external_surface_contact_time_closed_module_clearance_pocket",
        MODULE_ENVELOPE_X,
        MODULE_ENVELOPE_Y,
        MODULE_ENVELOPE_Z,
    )
    .translate(
        CRADLE_CENTER.0,
        CRADLE_CENTER.1,
        DECK_Z + CRADLE_Z - MODULE_ENVELOPE_Z / 2.0 + 5.0,
    );
    let window = centered_cube(
        "external_surface_contact_time_front_wipe_access_window",
        CRADLE_X - 96.0,
        34.0,
        CRADLE_Z - 34.0,
    )
    .translate(
        CRADLE_CENTER.0,
        CRADLE_CENTER.1 - CRADLE_Y / 2.0 + 17.0,
        DECK_Z + CRADLE_Z / 2.0 + 6.0,
    );

    base - module_pocket - module_saddle_cuts() - window
        + cradle_side_rails()
        + cradle_latch_lands()
        + cradle_datum_pins()
        + external_surface_witness_windows()
}

fn module_saddle_cuts() -> Part {
    let mut cuts = Part::empty("external_surface_contact_time_module_saddle_cuts");
    for (i, x) in [-132.0, 132.0].into_iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("external_surface_contact_time_module_radius_saddle_cut_{i}"),
                74.0,
                CRADLE_Y + 24.0,
                56,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                CRADLE_CENTER.0 + x,
                CRADLE_CENTER.1,
                DECK_Z + CRADLE_Z / 2.0 + 36.0,
            );
    }
    cuts
}

fn cradle_side_rails() -> Part {
    let left = centered_cube(
        "external_surface_contact_time_cradle_left_retention_rail",
        24.0,
        CRADLE_Y + 34.0,
        56.0,
    )
    .translate(
        CRADLE_CENTER.0 - CRADLE_X / 2.0 + 30.0,
        CRADLE_CENTER.1,
        DECK_Z + CRADLE_Z + 18.0,
    );
    let right = centered_cube(
        "external_surface_contact_time_cradle_right_retention_rail",
        24.0,
        CRADLE_Y + 34.0,
        56.0,
    )
    .translate(
        CRADLE_CENTER.0 + CRADLE_X / 2.0 - 30.0,
        CRADLE_CENTER.1,
        DECK_Z + CRADLE_Z + 18.0,
    );
    let rear_stop = centered_cube(
        "external_surface_contact_time_cradle_rear_module_stop",
        CRADLE_X - 78.0,
        24.0,
        70.0,
    )
    .translate(
        CRADLE_CENTER.0,
        CRADLE_CENTER.1 + CRADLE_Y / 2.0 - 12.0,
        DECK_Z + CRADLE_Z + 20.0,
    );
    let front_lip = centered_cube(
        "external_surface_contact_time_cradle_front_drip_lip",
        CRADLE_X - 112.0,
        20.0,
        32.0,
    )
    .translate(
        CRADLE_CENTER.0,
        CRADLE_CENTER.1 - CRADLE_Y / 2.0 + 10.0,
        DECK_Z + CRADLE_Z / 2.0 + 28.0,
    );
    left + right + rear_stop + front_lip
}

fn cradle_latch_lands() -> Part {
    let mut latches = Part::empty("external_surface_contact_time_cradle_latch_lands");
    for (i, (x, y)) in [
        (-236.0, -145.0),
        (0.0, -145.0),
        (236.0, -145.0),
        (-236.0, 145.0),
        (0.0, 145.0),
        (236.0, 145.0),
    ]
    .into_iter()
    .enumerate()
    {
        latches = latches
            + centered_cube(
                format!("external_surface_contact_time_cradle_toggle_latch_land_{i}"),
                78.0,
                24.0,
                18.0,
            )
            .translate(
                CRADLE_CENTER.0 + x,
                CRADLE_CENTER.1 + y,
                DECK_Z + CRADLE_Z + 9.0,
            )
            + centered_cylinder(
                format!("external_surface_contact_time_cradle_latch_pivot_{i}"),
                6.0,
                76.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                CRADLE_CENTER.0 + x,
                CRADLE_CENTER.1 + y,
                DECK_Z + CRADLE_Z + 22.0,
            );
    }
    latches
}

fn cradle_datum_pins() -> Part {
    let mut pins = Part::empty("external_surface_contact_time_cradle_datum_pins");
    for (i, (x, y, d)) in [
        (-240.0, -104.0, 12.0),
        (0.0, -104.0, 10.0),
        (240.0, -104.0, 12.0),
        (-240.0, 104.0, 10.0),
        (0.0, 104.0, 12.0),
        (240.0, 104.0, 10.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("external_surface_contact_time_cradle_keyed_datum_pin_{i}"),
                d / 2.0,
                24.0,
                28,
            )
            .translate(
                CRADLE_CENTER.0 + x,
                CRADLE_CENTER.1 + y,
                DECK_Z + CRADLE_Z + 12.0,
            );
    }
    pins
}

fn external_surface_witness_windows() -> Part {
    let mut windows = Part::empty("external_surface_contact_time_module_witness_windows");
    for (i, (x, y)) in [
        (-192.0, -88.0),
        (0.0, -88.0),
        (192.0, -88.0),
        (-192.0, 88.0),
        (0.0, 88.0),
        (192.0, 88.0),
    ]
    .into_iter()
    .enumerate()
    {
        windows = windows
            + centered_cube(
                format!("external_surface_contact_time_surface_wetness_view_window_{i}"),
                112.0,
                28.0,
                8.0,
            )
            .translate(
                CRADLE_CENTER.0 + x,
                CRADLE_CENTER.1 + y,
                DECK_Z + CRADLE_Z + 4.0,
            );
    }
    windows
}

fn removable_surface_coupon_carrier() -> Part {
    let mut carrier = centered_cube(
        "external_surface_contact_time_removable_coupon_carrier_plate",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    )
    .translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        DECK_Z + COUPON_PANEL_Z / 2.0,
    );

    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let (x, y) = coupon_slot_center(row, col);
            let slot = centered_cube(
                format!("external_surface_contact_time_coupon_slot_r{row}_c{col}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                10.0,
            )
            .translate(
                COUPON_CENTER.0 + x,
                COUPON_CENTER.1 + y,
                DECK_Z + COUPON_PANEL_Z - 4.5,
            );
            let spring_clip = centered_cube(
                format!("external_surface_contact_time_coupon_retention_clip_r{row}_c{col}"),
                COUPON_SLOT_X + 12.0,
                5.0,
                8.0,
            )
            .translate(
                COUPON_CENTER.0 + x,
                COUPON_CENTER.1 + y + COUPON_SLOT_Y / 2.0 + 5.0,
                DECK_Z + COUPON_PANEL_Z + 4.0,
            );
            carrier = carrier - slot + spring_clip;
        }
    }

    carrier + coupon_row_column_rails() + coupon_carrier_handles() + coupon_material_index_lands()
}

fn coupon_slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X,
        (row as f64 - (COUPON_ROWS as f64 - 1.0) / 2.0) * COUPON_PITCH_Y,
    )
}

fn coupon_row_column_rails() -> Part {
    let mut rails = Part::empty("external_surface_contact_time_coupon_grid_rails");
    for row in 0..=COUPON_ROWS {
        let y = COUPON_CENTER.1 + (row as f64 - COUPON_ROWS as f64 / 2.0) * COUPON_PITCH_Y
            - COUPON_PITCH_Y / 2.0;
        rails = rails
            + centered_cube(
                format!("external_surface_contact_time_coupon_row_separator_{row}"),
                COUPON_PANEL_X - 58.0,
                4.0,
                8.0,
            )
            .translate(COUPON_CENTER.0, y, DECK_Z + COUPON_PANEL_Z + 4.0);
    }
    for col in 0..=COUPON_COLS {
        let x = COUPON_CENTER.0 + (col as f64 - COUPON_COLS as f64 / 2.0) * COUPON_PITCH_X
            - COUPON_PITCH_X / 2.0;
        rails = rails
            + centered_cube(
                format!("external_surface_contact_time_coupon_column_separator_{col}"),
                4.0,
                COUPON_PANEL_Y - 52.0,
                8.0,
            )
            .translate(x, COUPON_CENTER.1, DECK_Z + COUPON_PANEL_Z + 4.0);
    }
    rails
}

fn coupon_carrier_handles() -> Part {
    let left = centered_cube(
        "external_surface_contact_time_coupon_carrier_left_pull_tab",
        22.0,
        100.0,
        26.0,
    )
    .translate(
        COUPON_CENTER.0 - COUPON_PANEL_X / 2.0 - 11.0,
        COUPON_CENTER.1,
        DECK_Z + COUPON_PANEL_Z / 2.0,
    );
    let right = centered_cube(
        "external_surface_contact_time_coupon_carrier_right_pull_tab",
        22.0,
        100.0,
        26.0,
    )
    .translate(
        COUPON_CENTER.0 + COUPON_PANEL_X / 2.0 + 11.0,
        COUPON_CENTER.1,
        DECK_Z + COUPON_PANEL_Z / 2.0,
    );
    left + right
}

fn coupon_material_index_lands() -> Part {
    let mut lands = Part::empty("external_surface_contact_time_coupon_material_index_lands");
    for i in 0..COUPON_COLS {
        let x = COUPON_CENTER.0 + slot_x(COUPON_COLS, COUPON_PITCH_X, i);
        lands = lands
            + centered_cube(
                format!("external_surface_contact_time_coupon_material_code_land_{i}"),
                44.0,
                13.0,
                5.0,
            )
            .translate(
                x,
                COUPON_CENTER.1 + COUPON_PANEL_Y / 2.0 - 20.0,
                DECK_Z + COUPON_PANEL_Z + 2.5,
            );
    }
    lands
}

fn wetness_contact_time_token_lanes() -> Part {
    let mut panel = centered_cube(
        "external_surface_contact_time_wetness_timer_token_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z + TOKEN_PANEL_Z / 2.0);

    for (i, token) in WETNESS_TOKENS.into_iter().enumerate() {
        panel =
            panel - token_slot("wetness", token, i, 36.0) + token_chip("wetness", token, i, 36.0);
    }
    for (i, token) in CONTACT_TOKENS.into_iter().enumerate() {
        panel = panel - token_slot("contact_time", token, i, -38.0)
            + token_chip("contact_time", token, i, -38.0);
    }

    panel + token_lane_dividers() + token_lane_stop_blocks() + token_lane_evidence_ruler()
}

fn token_slot(lane: &str, token: &str, index: usize, local_y: f64) -> Part {
    centered_cube(
        format!("external_surface_contact_time_{lane}_{token}_token_slot"),
        TOKEN_SLOT_X,
        TOKEN_SLOT_Y,
        TOKEN_SLOT_Z,
    )
    .translate(
        TOKEN_CENTER.0 + token_lane_x(index),
        TOKEN_CENTER.1 + local_y,
        DECK_Z + TOKEN_PANEL_Z - TOKEN_SLOT_Z / 2.0 + 0.4,
    )
}

fn token_chip(lane: &str, token: &str, index: usize, local_y: f64) -> Part {
    centered_cube(
        format!("external_surface_contact_time_{lane}_{token}_loose_token_witness"),
        TOKEN_SLOT_X - 18.0,
        TOKEN_SLOT_Y - 12.0,
        7.0,
    )
    .translate(
        TOKEN_CENTER.0 + token_lane_x(index),
        TOKEN_CENTER.1 + local_y,
        DECK_Z + TOKEN_PANEL_Z + 3.5,
    )
}

fn token_lane_x(index: usize) -> f64 {
    (index as f64 - (CONTACT_TOKENS.len() as f64 - 1.0) / 2.0) * TOKEN_PITCH_X
}

fn token_lane_dividers() -> Part {
    let center = centered_cube(
        "external_surface_contact_time_token_lane_center_divider",
        TOKEN_PANEL_X - 48.0,
        5.0,
        9.0,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z + TOKEN_PANEL_Z + 4.5);
    let upper = centered_cube(
        "external_surface_contact_time_wetness_token_lane_raised_edge",
        TOKEN_PANEL_X - 42.0,
        4.0,
        8.0,
    )
    .translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1 + TOKEN_PANEL_Y / 2.0 - 22.0,
        DECK_Z + TOKEN_PANEL_Z + 4.0,
    );
    let lower = centered_cube(
        "external_surface_contact_time_contact_timer_lane_raised_edge",
        TOKEN_PANEL_X - 42.0,
        4.0,
        8.0,
    )
    .translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1 - TOKEN_PANEL_Y / 2.0 + 22.0,
        DECK_Z + TOKEN_PANEL_Z + 4.0,
    );
    center + upper + lower
}

fn token_lane_stop_blocks() -> Part {
    let mut stops = Part::empty("external_surface_contact_time_token_lane_stop_blocks");
    for (i, x) in [-TOKEN_PANEL_X / 2.0 + 24.0, TOKEN_PANEL_X / 2.0 - 24.0]
        .into_iter()
        .enumerate()
    {
        stops = stops
            + centered_cube(
                format!("external_surface_contact_time_token_lane_end_stop_{i}"),
                18.0,
                TOKEN_PANEL_Y - 34.0,
                18.0,
            )
            .translate(
                TOKEN_CENTER.0 + x,
                TOKEN_CENTER.1,
                DECK_Z + TOKEN_PANEL_Z + 9.0,
            );
    }
    stops
}

fn token_lane_evidence_ruler() -> Part {
    let mut ruler = Part::empty("external_surface_contact_time_token_lane_evidence_ruler");
    for i in 0..=10 {
        ruler = ruler
            + centered_cube(
                format!("external_surface_contact_time_timer_ruler_tick_{i}"),
                2.0,
                18.0,
                6.0,
            )
            .translate(
                TOKEN_CENTER.0 - 280.0 + i as f64 * 56.0,
                TOKEN_CENTER.1 - 4.0,
                DECK_Z + TOKEN_PANEL_Z + 3.0,
            );
    }
    ruler
}

fn disinfectant_cartridge_pockets() -> Part {
    let dock = centered_cube(
        "external_surface_contact_time_disinfectant_cartridge_dock",
        CARTRIDGE_DOCK_X,
        CARTRIDGE_DOCK_Y,
        CARTRIDGE_DOCK_Z,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1,
        DECK_Z + CARTRIDGE_DOCK_Z / 2.0,
    );

    dock - cartridge_socket_cuts()
        + cartridge_placeholder_bodies()
        + disinfectant_wet_port_lands()
        + cartridge_key_tabs()
        + cartridge_lot_certificate_shelf()
}

fn cartridge_socket_cuts() -> Part {
    let mut cuts = Part::empty("external_surface_contact_time_cartridge_socket_cuts");
    for i in 0..DISINFECTANT_CARTRIDGES {
        cuts = cuts
            + centered_cylinder(
                format!("external_surface_contact_time_disinfectant_cartridge_socket_{i}"),
                CARTRIDGE_D / 2.0 + 4.0,
                CARTRIDGE_DOCK_Z + 10.0,
                44,
            )
            .translate(
                CARTRIDGE_CENTER.0 + slot_x(DISINFECTANT_CARTRIDGES, CARTRIDGE_PITCH_X, i),
                CARTRIDGE_CENTER.1 + 38.0,
                DECK_Z + CARTRIDGE_DOCK_Z / 2.0,
            );
    }
    cuts
}

fn cartridge_placeholder_bodies() -> Part {
    let mut cartridges = Part::empty("external_surface_contact_time_disinfectant_cartridges");
    for i in 0..DISINFECTANT_CARTRIDGES {
        let x = CARTRIDGE_CENTER.0 + slot_x(DISINFECTANT_CARTRIDGES, CARTRIDGE_PITCH_X, i);
        cartridges = cartridges
            + centered_cylinder(
                format!("external_surface_contact_time_disinfectant_cartridge_body_{i}"),
                CARTRIDGE_D / 2.0,
                CARTRIDGE_Z,
                44,
            )
            .translate(
                x,
                CARTRIDGE_CENTER.1 + 38.0,
                DECK_Z + CARTRIDGE_DOCK_Z + CARTRIDGE_Z / 2.0,
            )
            + centered_cube(
                format!("external_surface_contact_time_disinfectant_cartridge_key_flat_{i}"),
                CARTRIDGE_D + 16.0,
                10.0,
                18.0,
            )
            .translate(
                x,
                CARTRIDGE_CENTER.1 - 14.0,
                DECK_Z + CARTRIDGE_DOCK_Z + 24.0,
            );
    }
    cartridges
}

fn disinfectant_wet_port_lands() -> Part {
    let mut ports = Part::empty("external_surface_contact_time_disinfectant_wet_port_lands");
    for i in 0..WET_PORTS {
        ports = ports
            + centered_cylinder(
                format!("external_surface_contact_time_keyed_wet_port_land_{i}"),
                10.0,
                9.0,
                28,
            )
            .translate(
                CARTRIDGE_CENTER.0 - 148.0 + i as f64 * 59.0,
                CARTRIDGE_CENTER.1 - 76.0,
                DECK_Z + CARTRIDGE_DOCK_Z + 4.5,
            )
            + centered_cube(
                format!("external_surface_contact_time_wet_port_orientation_key_{i}"),
                18.0,
                5.0,
                7.0,
            )
            .translate(
                CARTRIDGE_CENTER.0 - 148.0 + i as f64 * 59.0,
                CARTRIDGE_CENTER.1 - 58.0,
                DECK_Z + CARTRIDGE_DOCK_Z + 3.5,
            );
    }
    ports
}

fn cartridge_key_tabs() -> Part {
    let mut tabs = Part::empty("external_surface_contact_time_disinfectant_key_tabs");
    for i in 0..DISINFECTANT_CARTRIDGES {
        tabs = tabs
            + centered_cube(
                format!("external_surface_contact_time_disinfectant_cartridge_key_tab_{i}"),
                30.0,
                12.0,
                16.0,
            )
            .translate(
                CARTRIDGE_CENTER.0 + slot_x(DISINFECTANT_CARTRIDGES, CARTRIDGE_PITCH_X, i),
                CARTRIDGE_CENTER.1 + 86.0,
                DECK_Z + CARTRIDGE_DOCK_Z + 8.0,
            );
    }
    tabs
}

fn cartridge_lot_certificate_shelf() -> Part {
    centered_cube(
        "external_surface_contact_time_disinfectant_lot_certificate_shelf",
        CARTRIDGE_DOCK_X - 58.0,
        28.0,
        10.0,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1 - CARTRIDGE_DOCK_Y / 2.0 + 24.0,
        DECK_Z + CARTRIDGE_DOCK_Z + 5.0,
    )
}

fn wipe_path_gauges() -> Part {
    let panel = centered_cube(
        "external_surface_contact_time_wipe_path_gauge_panel",
        WIPE_PANEL_X,
        WIPE_PANEL_Y,
        WIPE_PANEL_Z,
    )
    .translate(WIPE_CENTER.0, WIPE_CENTER.1, DECK_Z + WIPE_PANEL_Z / 2.0);

    panel - wipe_go_nogo_slots()
        + wipe_lane_rails()
        + wipe_overlap_witness_strips()
        + wipe_radius_gauge_blocks()
        + wipe_direction_arrows()
}

fn wipe_go_nogo_slots() -> Part {
    let mut slots = Part::empty("external_surface_contact_time_wipe_go_nogo_slots");
    for i in 0..WIPE_LANES {
        let y = WIPE_CENTER.1 + (i as f64 - (WIPE_LANES as f64 - 1.0) / 2.0) * WIPE_LANE_PITCH_Y;
        slots = slots
            + centered_cube(
                format!("external_surface_contact_time_wipe_path_go_nogo_slot_{i}"),
                WIPE_GAUGE_LENGTH,
                WIPE_GAUGE_WIDTH,
                9.0,
            )
            .translate(WIPE_CENTER.0, y, DECK_Z + WIPE_PANEL_Z - 4.0);
    }
    slots
}

fn wipe_lane_rails() -> Part {
    let mut rails = Part::empty("external_surface_contact_time_wipe_lane_rails");
    for i in 0..WIPE_LANES {
        let y = WIPE_CENTER.1 + (i as f64 - (WIPE_LANES as f64 - 1.0) / 2.0) * WIPE_LANE_PITCH_Y;
        rails = rails
            + centered_cube(
                format!("external_surface_contact_time_wipe_path_left_edge_rail_{i}"),
                WIPE_GAUGE_LENGTH,
                4.0,
                8.0,
            )
            .translate(
                WIPE_CENTER.0,
                y - WIPE_GAUGE_WIDTH / 2.0 - 7.0,
                DECK_Z + WIPE_PANEL_Z + 4.0,
            )
            + centered_cube(
                format!("external_surface_contact_time_wipe_path_right_edge_rail_{i}"),
                WIPE_GAUGE_LENGTH,
                4.0,
                8.0,
            )
            .translate(
                WIPE_CENTER.0,
                y + WIPE_GAUGE_WIDTH / 2.0 + 7.0,
                DECK_Z + WIPE_PANEL_Z + 4.0,
            );
    }
    rails
}

fn wipe_overlap_witness_strips() -> Part {
    let mut strips = Part::empty("external_surface_contact_time_wipe_overlap_witness_strips");
    for i in 0..WIPE_OVERLAP_STRIPS {
        strips = strips
            + centered_cube(
                format!("external_surface_contact_time_overlap_witness_strip_{i}"),
                6.0,
                WIPE_PANEL_Y - 38.0,
                7.0,
            )
            .translate(
                WIPE_CENTER.0 - 150.0 + i as f64 * 75.0,
                WIPE_CENTER.1,
                DECK_Z + WIPE_PANEL_Z + 3.5,
            );
    }
    strips
}

fn wipe_radius_gauge_blocks() -> Part {
    let mut gauges = Part::empty("external_surface_contact_time_wipe_corner_radius_gauges");
    for i in 0..WIPE_RADIUS_GAUGES {
        let radius = 18.0 + i as f64 * 10.0;
        let ring = centered_cylinder(
            format!("external_surface_contact_time_wipe_corner_radius_gauge_{i}"),
            radius,
            8.0,
            48,
        )
        .translate(
            WIPE_CENTER.0 + WIPE_PANEL_X / 2.0 - 92.0 + i as f64 * 32.0,
            WIPE_CENTER.1 + WIPE_PANEL_Y / 2.0 - 34.0,
            DECK_Z + WIPE_PANEL_Z + 4.0,
        );
        let inner = centered_cylinder(
            format!("external_surface_contact_time_wipe_corner_radius_gauge_cut_{i}"),
            radius - 7.0,
            10.0,
            48,
        )
        .translate(
            WIPE_CENTER.0 + WIPE_PANEL_X / 2.0 - 92.0 + i as f64 * 32.0,
            WIPE_CENTER.1 + WIPE_PANEL_Y / 2.0 - 34.0,
            DECK_Z + WIPE_PANEL_Z + 4.0,
        );
        gauges = gauges + (ring - inner);
    }
    gauges
}

fn wipe_direction_arrows() -> Part {
    let mut arrows = Part::empty("external_surface_contact_time_wipe_direction_arrows");
    for i in 0..WIPE_LANES {
        let y = WIPE_CENTER.1 + (i as f64 - (WIPE_LANES as f64 - 1.0) / 2.0) * WIPE_LANE_PITCH_Y;
        arrows = arrows
            + centered_cube(
                format!("external_surface_contact_time_wipe_direction_shaft_{i}"),
                44.0,
                5.0,
                6.0,
            )
            .translate(
                WIPE_CENTER.0 - WIPE_GAUGE_LENGTH / 2.0 + 45.0,
                y,
                DECK_Z + WIPE_PANEL_Z + 3.0,
            )
            + centered_cube(
                format!("external_surface_contact_time_wipe_direction_head_{i}"),
                16.0,
                16.0,
                6.0,
            )
            .translate(
                WIPE_CENTER.0 - WIPE_GAUGE_LENGTH / 2.0 + 76.0,
                y,
                DECK_Z + WIPE_PANEL_Z + 3.0,
            );
    }
    arrows
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "external_surface_contact_time_barcode_certificate_trace_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, DECK_Z + TRACE_PANEL_Z / 2.0);

    panel + barcode_lands() + certificate_lands() + rfid_lands() + tamper_seal_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("external_surface_contact_time_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(
                format!("external_surface_contact_time_barcode_land_{i}"),
                68.0,
                22.0,
                4.0,
            )
            .translate(
                TRACE_CENTER.0 - 116.0 + col as f64 * 78.0,
                TRACE_CENTER.1 + 56.0 - row as f64 * 44.0,
                DECK_Z + TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut certs = Part::empty("external_surface_contact_time_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        certs = certs
            + centered_cube(
                format!("external_surface_contact_time_certificate_card_land_{i}"),
                118.0,
                46.0,
                5.0,
            )
            .translate(
                TRACE_CENTER.0 - 78.0 + (i % 2) as f64 * 156.0,
                TRACE_CENTER.1 - 62.0 - (i / 2) as f64 * 54.0,
                DECK_Z + TRACE_PANEL_Z + 2.5,
            );
    }
    certs
}

fn rfid_lands() -> Part {
    let mut rfids = Part::empty("external_surface_contact_time_rfid_witness_lands");
    for i in 0..RFID_LANDS {
        rfids = rfids
            + centered_cube(
                format!("external_surface_contact_time_rfid_witness_pad_{i}"),
                40.0,
                32.0,
                4.0,
            )
            .translate(
                TRACE_CENTER.0 + 132.0,
                TRACE_CENTER.1 + 64.0 - i as f64 * 42.0,
                DECK_Z + TRACE_PANEL_Z + 2.0,
            );
    }
    rfids
}

fn tamper_seal_slots() -> Part {
    let mut seals = Part::empty("external_surface_contact_time_tamper_seal_slots");
    for i in 0..4 {
        seals = seals
            + centered_cube(
                format!("external_surface_contact_time_certificate_tamper_seal_slot_{i}"),
                50.0,
                6.0,
                6.0,
            )
            .translate(
                TRACE_CENTER.0 - 120.0 + i as f64 * 80.0,
                TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0 - 18.0,
                DECK_Z + TRACE_PANEL_Z + 3.0,
            );
    }
    seals
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "external_surface_contact_time_clean_used_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(SEGREGATION_CENTER_X, 0.0, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let gate_cut = centered_cube(
        "external_surface_contact_time_clean_used_pass_gate_clearance",
        SEGREGATION_WALL_X + 4.0,
        SEGREGATION_GATE_Y,
        SEGREGATION_WALL_Z + 4.0,
    )
    .translate(
        SEGREGATION_CENTER_X,
        -48.0,
        DECK_Z + SEGREGATION_WALL_Z / 2.0,
    );

    wall - gate_cut + segregation_gate_frame() + clean_buffer_lands() + used_buffer_lands()
}

fn segregation_gate_frame() -> Part {
    let upper = centered_cube(
        "external_surface_contact_time_clean_used_pass_gate_upper_frame",
        34.0,
        SEGREGATION_GATE_Y + 26.0,
        14.0,
    )
    .translate(
        SEGREGATION_CENTER_X,
        -48.0,
        DECK_Z + SEGREGATION_WALL_Z - 7.0,
    );
    let lower = centered_cube(
        "external_surface_contact_time_clean_used_pass_gate_floor_bridge",
        34.0,
        SEGREGATION_GATE_Y + 26.0,
        8.0,
    )
    .translate(SEGREGATION_CENTER_X, -48.0, DECK_Z + 4.0);
    upper + lower
}

fn clean_buffer_lands() -> Part {
    let mut lands = Part::empty("external_surface_contact_time_clean_coupon_buffer_lands");
    for i in 0..CLEAN_BUFFER_LANDS {
        lands = lands
            + centered_cube(
                format!("external_surface_contact_time_clean_buffer_coupon_land_{i}"),
                86.0,
                46.0,
                8.0,
            )
            .translate(
                SEGREGATION_CENTER_X - 82.0,
                -318.0 + i as f64 * 66.0,
                DECK_Z + 4.0,
            );
    }
    lands
}

fn used_buffer_lands() -> Part {
    let mut lands = Part::empty("external_surface_contact_time_used_evidence_buffer_lands");
    for i in 0..USED_BUFFER_LANDS {
        lands = lands
            + centered_cube(
                format!("external_surface_contact_time_used_evidence_land_{i}"),
                92.0,
                42.0,
                8.0,
            )
            .translate(
                SEGREGATION_CENTER_X + 86.0,
                -330.0 + i as f64 * 70.0,
                DECK_Z + 4.0,
            );
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let mut panel = centered_cube(
        "external_surface_contact_time_release_hold_reject_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(
        STATUS_CENTER.0,
        STATUS_CENTER.1,
        DECK_Z + STATUS_PANEL_Z / 2.0,
    );

    for (lane, name) in STATUS_NAMES.into_iter().enumerate() {
        for pos in 0..STATUS_SLOTS_PER_LANE {
            panel = panel - status_slot(name, lane, pos) + status_token(name, lane, pos);
        }
    }

    panel + status_lane_separators() + disposition_lock_bar()
}

fn status_slot(name: &str, lane: usize, position: usize) -> Part {
    centered_cube(
        format!("external_surface_contact_time_{name}_lane_token_slot_{position}"),
        STATUS_SLOT_X,
        STATUS_SLOT_Y,
        10.0,
    )
    .translate(
        STATUS_CENTER.0 - 160.0 + position as f64 * 106.0,
        STATUS_CENTER.1 + (lane as f64 - 1.0) * STATUS_LANE_PITCH_Y,
        DECK_Z + STATUS_PANEL_Z - 4.6,
    )
}

fn status_token(name: &str, lane: usize, position: usize) -> Part {
    centered_cube(
        format!("external_surface_contact_time_{name}_decision_token_{position}"),
        STATUS_SLOT_X - 18.0,
        STATUS_SLOT_Y - 12.0,
        7.0,
    )
    .translate(
        STATUS_CENTER.0 - 160.0 + position as f64 * 106.0,
        STATUS_CENTER.1 + (lane as f64 - 1.0) * STATUS_LANE_PITCH_Y,
        DECK_Z + STATUS_PANEL_Z + 3.5,
    )
}

fn status_lane_separators() -> Part {
    let upper = centered_cube(
        "external_surface_contact_time_release_hold_lane_separator",
        STATUS_PANEL_X - 34.0,
        4.0,
        8.0,
    )
    .translate(
        STATUS_CENTER.0,
        STATUS_CENTER.1 + STATUS_LANE_PITCH_Y / 2.0,
        DECK_Z + STATUS_PANEL_Z + 4.0,
    );
    let lower = centered_cube(
        "external_surface_contact_time_hold_reject_lane_separator",
        STATUS_PANEL_X - 34.0,
        4.0,
        8.0,
    )
    .translate(
        STATUS_CENTER.0,
        STATUS_CENTER.1 - STATUS_LANE_PITCH_Y / 2.0,
        DECK_Z + STATUS_PANEL_Z + 4.0,
    );
    upper + lower
}

fn disposition_lock_bar() -> Part {
    centered_cube(
        "external_surface_contact_time_disposition_lock_bar",
        STATUS_PANEL_X - 60.0,
        12.0,
        14.0,
    )
    .translate(
        STATUS_CENTER.0,
        STATUS_CENTER.1 + STATUS_PANEL_Y / 2.0 - 18.0,
        DECK_Z + STATUS_PANEL_Z + 7.0,
    )
}

fn evidence_bridge() -> Part {
    let mut bridge = Part::empty("external_surface_contact_time_evidence_bridge");
    for (i, (x, y)) in [
        (-BRIDGE_SPAN_X / 2.0, -128.0),
        (BRIDGE_SPAN_X / 2.0, -128.0),
        (-BRIDGE_SPAN_X / 2.0, 128.0),
        (BRIDGE_SPAN_X / 2.0, 128.0),
    ]
    .into_iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("external_surface_contact_time_evidence_bridge_post_{i}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_UNDERSIDE_Z,
            )
            .translate(
                BRIDGE_CENTER.0 + x,
                BRIDGE_CENTER.1 + y,
                DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0,
            );
    }

    bridge
        + evidence_bridge_beams()
        + evidence_camera_pods()
        + evidence_led_bars()
        + certificate_scan_crossbar()
}

fn evidence_bridge_beams() -> Part {
    let front = centered_cube(
        "external_surface_contact_time_evidence_bridge_front_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1 - 128.0,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "external_surface_contact_time_evidence_bridge_rear_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1 + 128.0,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let center = centered_cube(
        "external_surface_contact_time_evidence_bridge_camera_spine",
        BRIDGE_SPAN_X - 120.0,
        24.0,
        24.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + 18.0,
    );
    front + rear + center
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("external_surface_contact_time_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("external_surface_contact_time_overhead_camera_pod_{i}"),
                60.0,
                44.0,
                28.0,
            )
            .translate(
                BRIDGE_CENTER.0 + slot_x(CAMERA_PODS, 240.0, i),
                BRIDGE_CENTER.1,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 18.0,
            ) + centered_cylinder(
                format!("external_surface_contact_time_camera_lens_clearance_{i}"),
                12.0,
                10.0,
                32,
            )
            .translate(
                BRIDGE_CENTER.0 + slot_x(CAMERA_PODS, 240.0, i),
                BRIDGE_CENTER.1,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 36.0,
            );
    }
    pods
}

fn evidence_led_bars() -> Part {
    let mut bars = Part::empty("external_surface_contact_time_evidence_led_bars");
    for i in 0..LED_BARS {
        let y = if i % 2 == 0 { -86.0 } else { 86.0 };
        let x = if i < 2 { -330.0 } else { 330.0 };
        bars = bars
            + centered_cube(
                format!("external_surface_contact_time_evidence_led_bar_{i}"),
                260.0,
                12.0,
                10.0,
            )
            .translate(
                BRIDGE_CENTER.0 + x,
                BRIDGE_CENTER.1 + y,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 30.0,
            );
    }
    bars
}

fn certificate_scan_crossbar() -> Part {
    centered_cube(
        "external_surface_contact_time_certificate_scan_crossbar",
        320.0,
        20.0,
        20.0,
    )
    .translate(
        TRACE_CENTER.0,
        TRACE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z - 8.0,
    )
}

fn robot_service_keepouts() -> Part {
    front_robot_keepout()
        + rear_service_keepout()
        + left_coupon_service_keepout()
        + right_cartridge_service_keepout()
        + overhead_keepout_gauge()
}

fn front_robot_keepout() -> Part {
    centered_cube(
        "external_surface_contact_time_front_robot_keepout_rail",
        DECK_X - 180.0,
        10.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    )
}

fn rear_service_keepout() -> Part {
    centered_cube(
        "external_surface_contact_time_rear_service_keepout_rail",
        DECK_X - 180.0,
        10.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    )
}

fn left_coupon_service_keepout() -> Part {
    centered_cube(
        "external_surface_contact_time_left_coupon_service_keepout_rail",
        10.0,
        DECK_Y - 190.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_COUPON_SERVICE_KEEP_OUT_X,
        0.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    )
}

fn right_cartridge_service_keepout() -> Part {
    centered_cube(
        "external_surface_contact_time_right_cartridge_service_keepout_rail",
        10.0,
        DECK_Y - 190.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_CARTRIDGE_SERVICE_KEEP_OUT_X,
        0.0,
        DECK_Z + KEEP_OUT_RAIL_Z / 2.0,
    )
}

fn overhead_keepout_gauge() -> Part {
    let left = centered_cube(
        "external_surface_contact_time_overhead_keepout_left_gauge_post",
        18.0,
        18.0,
        OVERHEAD_EVIDENCE_KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + 126.0,
        DECK_Y / 2.0 - 126.0,
        DECK_Z + OVERHEAD_EVIDENCE_KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "external_surface_contact_time_overhead_keepout_right_gauge_post",
        18.0,
        18.0,
        OVERHEAD_EVIDENCE_KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - 126.0,
        DECK_Y / 2.0 - 126.0,
        DECK_Z + OVERHEAD_EVIDENCE_KEEP_OUT_Z / 2.0,
    );
    let beam = centered_cube(
        "external_surface_contact_time_overhead_evidence_keepout_beam",
        DECK_X - 252.0,
        16.0,
        16.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 126.0,
        DECK_Z + OVERHEAD_EVIDENCE_KEEP_OUT_Z,
    );
    left + right + beam
}

fn top_recess(name: impl Into<String>, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth).translate(center.0, center.1, DECK_Z - depth / 2.0 + 0.2)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let outer = centered_cylinder(format!("{name}_outer_ring"), 15.0, 4.0, 40);
    let inner = centered_cylinder(format!("{name}_center_recess"), 7.0, 5.0, 32);
    outer - inner
}

fn slot_x(count: usize, pitch: f64, index: usize) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}
