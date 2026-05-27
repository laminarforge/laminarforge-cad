use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-media osmolality/evaporation correlation validation station.
//
// This standalone generator models the mechanical witness station used to
// correlate sealed reservoir osmolality drift against exposure to evaporation,
// condensation, humidity, temperature, and mass loss conditions. It is concept
// CAD only: it provides nests, pockets, witness coupon holders, probe mounts,
// return channels, calibration vial storage, and physical CSG label geometry.
// It does not define sample acceptance limits, media recipes, sensor selection,
// or biological release criteria.

const OUTPUT_PREFIX: &str = "closed_media_osmolality_evaporation_correlation_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_osmolality_evaporation_correlation_station_secondary_containment_deck.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_sealed_reservoir_nests.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_mass_osmolality_sample_pockets.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_evaporation_witness_coupon_holders.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_humidity_temperature_probe_mounts.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_condensate_return_channels.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_calibration_vial_rack.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_exposure_conditioning_manifold.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_correlation_label_grid.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_evidence_camera_bridge.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_robot_service_keepout_gauges.stl",
    "output/closed_media_osmolality_evaporation_correlation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_reservoir_nests",
    "mass_osmolality_sample_pockets",
    "evaporation_witness_coupon_holders",
    "humidity_temperature_probe_mounts",
    "condensate_return_channels",
    "calibration_vial_rack",
    "exposure_conditioning_manifold",
    "correlation_label_grid",
    "csg_label_geometry",
    "named_stl_outputs",
];

const STATION_X: f64 = 1600.0;
const STATION_Y: f64 = 1000.0;
const DECK_Z: f64 = 24.0;
const CURB_W: f64 = 24.0;
const CURB_Z: f64 = 54.0;
const BASIN_DEPTH: f64 = 8.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 7.0;
const DRAIN_PORT_D: f64 = 22.0;
const DATUM_TARGETS: usize = 4;
const MOUNT_HOLES: usize = 8;

const RESERVOIR_POS: (f64, f64) = (-410.0, 155.0);
const RESERVOIR_X: f64 = 610.0;
const RESERVOIR_Y: f64 = 360.0;
const RESERVOIR_Z: f64 = 46.0;
const RESERVOIR_COLS: usize = 2;
const RESERVOIR_ROWS: usize = 2;
const RESERVOIR_COUNT: usize = RESERVOIR_COLS * RESERVOIR_ROWS;
const RESERVOIR_PITCH_X: f64 = 255.0;
const RESERVOIR_PITCH_Y: f64 = 150.0;
const RESERVOIR_CELL_X: f64 = 208.0;
const RESERVOIR_CELL_Y: f64 = 104.0;
const RESERVOIR_RECESS_DEPTH: f64 = 22.0;
const RESERVOIR_GASKET_W: f64 = 8.0;
const RESERVOIR_LATCHES_PER_NEST: usize = 4;
const RESERVOIR_SAMPLE_FLAGS_PER_NEST: usize = 3;

const SAMPLE_POS: (f64, f64) = (330.0, 255.0);
const SAMPLE_X: f64 = 470.0;
const SAMPLE_Y: f64 = 245.0;
const SAMPLE_Z: f64 = 42.0;
const SAMPLE_PAIR_COUNT: usize = 8;
const SAMPLE_COLS: usize = 4;
const SAMPLE_PITCH_X: f64 = 102.0;
const SAMPLE_PITCH_Y: f64 = 84.0;
const MASS_POCKET_D: f64 = 38.0;
const OSMO_POCKET_D: f64 = 24.0;
const SAMPLE_POCKET_DEPTH: f64 = 26.0;
const SAMPLE_LABEL_BARS: usize = 5;

const COUPON_POS: (f64, f64) = (-410.0, -255.0);
const COUPON_X: f64 = 610.0;
const COUPON_Y: f64 = 250.0;
const COUPON_Z: f64 = 38.0;
const COUPON_COUNT: usize = 12;
const COUPON_COLS: usize = 4;
const COUPON_PITCH_X: f64 = 132.0;
const COUPON_PITCH_Y: f64 = 70.0;
const WITNESS_COUPON_X: f64 = 92.0;
const WITNESS_COUPON_Y: f64 = 42.0;
const WITNESS_COUPON_Z: f64 = 8.0;
const COUPON_SLOT_DEPTH: f64 = 14.0;
const EVAPORATION_LEVELS: usize = 3;

const PROBE_POS: (f64, f64) = (330.0, 20.0);
const PROBE_X: f64 = 470.0;
const PROBE_Y: f64 = 160.0;
const PROBE_Z: f64 = 58.0;
const PROBE_MOUNTS: usize = 6;
const PROBE_PITCH_X: f64 = 70.0;
const PROBE_BODY_D: f64 = 18.0;
const PROBE_CLAMP_W: f64 = 48.0;
const CABLE_CHANNEL_W: f64 = 16.0;

const CONDENSATE_POS: (f64, f64) = (330.0, -180.0);
const CONDENSATE_X: f64 = 470.0;
const CONDENSATE_Y: f64 = 160.0;
const CONDENSATE_Z: f64 = 32.0;
const RETURN_CHANNELS: usize = 5;
const RETURN_CHANNEL_W: f64 = 34.0;
const RETURN_CHANNEL_DEPTH: f64 = 14.0;
const RETURN_CUP_COUNT: usize = 4;
const FLOW_ARROW_COUNT: usize = 10;

const CAL_POS: (f64, f64) = (330.0, -350.0);
const CAL_X: f64 = 470.0;
const CAL_Y: f64 = 120.0;
const CAL_Z: f64 = 44.0;
const CAL_VIALS: usize = 10;
const CAL_VIAL_D: f64 = 24.0;
const CAL_VIAL_PITCH_X: f64 = 42.0;
const CAL_LABEL_WINDOWS: usize = 5;

const EXPOSURE_POS: (f64, f64) = (-410.0, 400.0);
const EXPOSURE_X: f64 = 610.0;
const EXPOSURE_Y: f64 = 60.0;
const EXPOSURE_Z: f64 = 96.0;
const EXPOSURE_LANES: usize = 4;
const EXPOSURE_WINDOW_COUNT: usize = RESERVOIR_COUNT;
const CONDENSATION_BAFFLES: usize = 6;
const VAPOR_PORT_D: f64 = 20.0;

const LABEL_POS: (f64, f64) = (-410.0, -425.0);
const LABEL_X: f64 = 610.0;
const LABEL_Y: f64 = 70.0;
const LABEL_Z: f64 = 14.0;
const AXIS_LABELS: usize = 6;
const CORRELATION_TOKEN_COUNT: usize = 12;
const LABEL_CODE_BARS: usize = 6;

const CAMERA_POS: (f64, f64) = (0.0, 458.0);
const CAMERA_X: f64 = 1350.0;
const CAMERA_Y: f64 = 46.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const CAMERA_CLEARANCE_Z: f64 = 235.0;
const CAMERA_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1510.0;
const KEEP_OUT_Y: f64 = 930.0;
const KEEP_OUT_Z: f64 = 7.0;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_ENVIRONMENTAL_SERVICE: f64 = 280.0;
const SIDE_BALANCE_SERVICE: f64 = 240.0;
const TOP_PROBE_CLEARANCE: f64 = 330.0;
const KEEP_OUT_RAIL_W: f64 = 8.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self, margin: f64) -> bool {
        self.center.0 - self.x / 2.0 >= -STATION_X / 2.0 + margin
            && self.center.0 + self.x / 2.0 <= STATION_X / 2.0 - margin
            && self.center.1 - self.y / 2.0 >= -STATION_Y / 2.0 + margin
            && self.center.1 + self.y / 2.0 <= STATION_Y / 2.0 - margin
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let nests = sealed_reservoir_nests();
    export(OUTPUTS[1], &nests);

    let sample_pockets = mass_osmolality_sample_pockets();
    export(OUTPUTS[2], &sample_pockets);

    let coupons = evaporation_witness_coupon_holders();
    export(OUTPUTS[3], &coupons);

    let probes = humidity_temperature_probe_mounts();
    export(OUTPUTS[4], &probes);

    let returns = condensate_return_channels();
    export(OUTPUTS[5], &returns);

    let calibration = calibration_vial_rack();
    export(OUTPUTS[6], &calibration);

    let exposure = exposure_conditioning_manifold();
    export(OUTPUTS[7], &exposure);

    let labels = correlation_label_grid();
    export(OUTPUTS[8], &labels);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + nests.translate(RESERVOIR_POS.0, RESERVOIR_POS.1, DECK_Z - SOCKET_DEPTH)
        + sample_pockets.translate(SAMPLE_POS.0, SAMPLE_POS.1, DECK_Z - SOCKET_DEPTH)
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, DECK_Z - SOCKET_DEPTH)
        + probes.translate(PROBE_POS.0, PROBE_POS.1, DECK_Z - SOCKET_DEPTH)
        + returns.translate(CONDENSATE_POS.0, CONDENSATE_POS.1, DECK_Z - SOCKET_DEPTH)
        + calibration.translate(CAL_POS.0, CAL_POS.1, DECK_Z - SOCKET_DEPTH)
        + exposure.translate(EXPOSURE_POS.0, EXPOSURE_POS.1, DECK_Z)
        + labels.translate(LABEL_POS.0, LABEL_POS.1, DECK_Z)
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed-media osmolality evaporation correlation station:");
    println!(
        "  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm secondary containment deck"
    );
    println!(
        "  Reservoir matrix:      {RESERVOIR_COUNT} sealed nests with gasket witness frames, latch tabs, and sample flags"
    );
    println!(
        "  Sampling correlation:  {SAMPLE_PAIR_COUNT} paired mass/osmolality pockets, {COUPON_COUNT} evaporation witness coupons, {CAL_VIALS} calibration vial wells"
    );
    println!(
        "  Environment capture:   {PROBE_MOUNTS} humidity/temperature probe clamps, {RETURN_CHANNELS} condensate return channels, {EXPOSURE_LANES} exposure lanes"
    );
    println!(
        "  Evidence geometry:     {CORRELATION_TOKEN_COUNT} raised correlation tokens, {AXIS_LABELS} axis labels, {CAMERA_COUNT} camera lands, {LIGHT_BAR_COUNT} light bars"
    );
    println!(
        "  Clearance gauges:      front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_ENVIRONMENTAL_SERVICE:.0}mm, side balance {SIDE_BALANCE_SERVICE:.0}mm, top probe {TOP_PROBE_CLEARANCE:.0}mm"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
    println!("  Outputs exported:      {}", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_position(
    index: usize,
    cols: usize,
    count: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    let rows = count.div_ceil(cols);
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(RESERVOIR_COUNT, RESERVOIR_COLS * RESERVOIR_ROWS);
    assert_eq!(EXPOSURE_WINDOW_COUNT, RESERVOIR_COUNT);
    assert_eq!(COUPON_COUNT / COUPON_COLS, EVAPORATION_LEVELS);
    assert_eq!(MOUNT_HOLES, mount_hole_positions().len());
    assert_eq!(DATUM_TARGETS, datum_target_positions().len());
    assert_eq!(RETURN_CUP_COUNT, RESERVOIR_COUNT);
    assert!(RESERVOIR_RECESS_DEPTH < RESERVOIR_Z);
    assert!(SAMPLE_POCKET_DEPTH < SAMPLE_Z);
    assert!(COUPON_SLOT_DEPTH < COUPON_Z);
    assert!(RETURN_CHANNEL_DEPTH < CONDENSATE_Z);
    assert!(TOP_PROBE_CLEARANCE > CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z);
    assert!(condensate_return_capacity_ml() > evaporation_challenge_volume_ml());
    assert!(calibration_capacity_ml() > SAMPLE_PAIR_COUNT as f64 * 0.8);

    let rects = deck_module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(CURB_W + 12.0),
            "{} exceeds station deck",
            rect.name
        );
    }

    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn deck_module_rects() -> [Rect; 8] {
    [
        rect(
            "sealed_reservoir_nests",
            RESERVOIR_POS,
            RESERVOIR_X,
            RESERVOIR_Y,
        ),
        rect(
            "mass_osmolality_sample_pockets",
            SAMPLE_POS,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "evaporation_witness_coupon_holders",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        rect(
            "humidity_temperature_probe_mounts",
            PROBE_POS,
            PROBE_X,
            PROBE_Y,
        ),
        rect(
            "condensate_return_channels",
            CONDENSATE_POS,
            CONDENSATE_X,
            CONDENSATE_Y,
        ),
        rect("calibration_vial_rack", CAL_POS, CAL_X, CAL_Y),
        rect(
            "exposure_conditioning_manifold",
            EXPOSURE_POS,
            EXPOSURE_X,
            EXPOSURE_Y,
        ),
        rect("correlation_label_grid", LABEL_POS, LABEL_X, LABEL_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn condensate_return_capacity_ml() -> f64 {
    RETURN_CHANNELS as f64 * RETURN_CHANNEL_W * CONDENSATE_X * RETURN_CHANNEL_DEPTH / 1000.0
}

fn evaporation_challenge_volume_ml() -> f64 {
    RESERVOIR_COUNT as f64 * 25.0 + COUPON_COUNT as f64 * 3.0
}

fn calibration_capacity_ml() -> f64 {
    CAL_VIALS as f64 * 2.0
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        "osmolality_evap_correlation_secondary_containment_deck_plate",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "osmolality_evap_correlation_shallow_spill_basin_relief",
        STATION_X - 2.0 * CURB_W,
        STATION_Y - 2.0 * CURB_W,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 1.0);
    let front_drain = centered_cylinder(
        "osmolality_evap_correlation_front_basin_drain_port",
        DRAIN_PORT_D / 2.0,
        DECK_Z + 8.0,
        36,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W + 48.0, DECK_Z / 2.0);

    deck - basin - front_drain
        + containment_curbs()
        + deck_sockets()
        + mount_hole_bosses()
        + datum_targets()
        + deck_wet_dry_dividers()
}

fn containment_curbs() -> Part {
    let front = centered_cube(
        "osmolality_evap_correlation_front_secondary_containment_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, CURB_Z / 2.0);
    let rear = centered_cube(
        "osmolality_evap_correlation_rear_secondary_containment_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, CURB_Z / 2.0);
    let left = centered_cube(
        "osmolality_evap_correlation_left_secondary_containment_curb",
        CURB_W,
        STATION_Y - 2.0 * CURB_W,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, CURB_Z / 2.0);
    let right = centered_cube(
        "osmolality_evap_correlation_right_secondary_containment_curb",
        CURB_W,
        STATION_Y - 2.0 * CURB_W,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, CURB_Z / 2.0);
    front + rear + left + right
}

fn deck_sockets() -> Part {
    let mut sockets = Part::empty("osmolality_evap_correlation_deck_socket_relief_set");
    for spec in deck_module_rects() {
        sockets = sockets
            + centered_cube(
                format!("osmolality_evap_correlation_{}_socket_relief", spec.name),
                spec.x + 10.0,
                spec.y + 10.0,
                SOCKET_DEPTH,
            )
            .translate(spec.center.0, spec.center.1, DECK_Z + SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mount_hole_bosses() -> Part {
    let mut bosses = Part::empty("osmolality_evap_correlation_deck_mount_hole_bosses");
    for (index, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("osmolality_evap_correlation_mount_hole_{index}_boss"),
            18.0,
            7.0,
            36,
        )
        .translate(x, y, DECK_Z + 3.5);
        let bore = centered_cylinder(
            format!("osmolality_evap_correlation_mount_hole_{index}_through_bore"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, DECK_Z + 5.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-690.0, -415.0),
        (-230.0, -415.0),
        (230.0, -415.0),
        (690.0, -415.0),
        (-690.0, 415.0),
        (-230.0, 415.0),
        (230.0, 415.0),
        (690.0, 415.0),
    ]
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("osmolality_evap_correlation_deck_datum_targets");
    for (index, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets = targets
            + datum_target(format!("osmolality_evap_correlation_datum_{index}")).translate(
                x,
                y,
                DECK_Z + 3.0,
            );
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGETS] {
    [
        (-730.0, -455.0),
        (730.0, -455.0),
        (-730.0, 455.0),
        (730.0, 455.0),
    ]
}

fn datum_target(name: String) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_disc"), 13.0, 4.0, 40);
    let dot = centered_cylinder(format!("{name}_center_dot"), 2.7, 5.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 22.0, 2.4, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.4, 22.0, 5.0);
    disc + dot + cross_x + cross_y
}

fn deck_wet_dry_dividers() -> Part {
    let vertical = centered_cube(
        "osmolality_evap_correlation_wet_dry_center_divider_raised_bar",
        9.0,
        STATION_Y - 150.0,
        10.0,
    )
    .translate(-65.0, 0.0, DECK_Z + 5.0);
    let lower = centered_cube(
        "osmolality_evap_correlation_witness_to_calibration_divider_raised_bar",
        700.0,
        9.0,
        10.0,
    )
    .translate(220.0, -275.0, DECK_Z + 5.0);
    let upper = centered_cube(
        "osmolality_evap_correlation_sample_to_probe_divider_raised_bar",
        700.0,
        9.0,
        10.0,
    )
    .translate(220.0, 130.0, DECK_Z + 5.0);
    vertical + lower + upper
}

fn sealed_reservoir_nests() -> Part {
    let plate = centered_cube(
        "osmolality_evap_correlation_sealed_reservoir_nest_plate",
        RESERVOIR_X,
        RESERVOIR_Y,
        RESERVOIR_Z,
    )
    .translate(0.0, 0.0, RESERVOIR_Z / 2.0);

    plate - reservoir_recesses()
        + reservoir_frames()
        + reservoir_latches()
        + reservoir_sample_flags()
        + reservoir_condition_labels()
}

fn reservoir_recesses() -> Part {
    let mut recesses = Part::empty("osmolality_evap_correlation_reservoir_recess_cutouts");
    for index in 0..RESERVOIR_COUNT {
        let (x, y) = reservoir_position(index);
        let pocket = centered_cube(
            format!("osmolality_evap_correlation_reservoir_{index}_sealed_bag_pocket_relief"),
            RESERVOIR_CELL_X,
            RESERVOIR_CELL_Y,
            RESERVOIR_RECESS_DEPTH + 2.0,
        )
        .translate(x, y, RESERVOIR_Z - RESERVOIR_RECESS_DEPTH / 2.0 + 1.0);
        let cap_cutout = centered_cylinder(
            format!("osmolality_evap_correlation_reservoir_{index}_cap_clearance_relief"),
            17.0,
            RESERVOIR_RECESS_DEPTH + 4.0,
            36,
        )
        .translate(
            x + RESERVOIR_CELL_X / 2.0 - 34.0,
            y,
            RESERVOIR_Z - RESERVOIR_RECESS_DEPTH / 2.0,
        );
        recesses = recesses + pocket + cap_cutout;
    }
    recesses
}

fn reservoir_frames() -> Part {
    let mut frames = Part::empty("osmolality_evap_correlation_reservoir_gasket_frames");
    for index in 0..RESERVOIR_COUNT {
        let (x, y) = reservoir_position(index);
        frames = frames
            + rectangular_frame(
                format!("osmolality_evap_correlation_reservoir_{index}_gasket_compression_frame"),
                RESERVOIR_CELL_X + 28.0,
                RESERVOIR_CELL_Y + 28.0,
                RESERVOIR_GASKET_W,
                10.0,
            )
            .translate(x, y, RESERVOIR_Z + 5.0);
        frames = frames
            + centered_cube(
                format!("osmolality_evap_correlation_reservoir_{index}_closed_media_barcode_land"),
                78.0,
                16.0,
                6.0,
            )
            .translate(
                x - RESERVOIR_CELL_X / 2.0 + 52.0,
                y + RESERVOIR_CELL_Y / 2.0 - 18.0,
                RESERVOIR_Z + 3.0,
            );
    }
    frames
}

fn reservoir_latches() -> Part {
    let mut latches = Part::empty("osmolality_evap_correlation_reservoir_latch_tabs");
    for index in 0..RESERVOIR_COUNT {
        let (x, y) = reservoir_position(index);
        for latch in 0..RESERVOIR_LATCHES_PER_NEST {
            let side_x = if latch % 2 == 0 { -1.0 } else { 1.0 };
            let side_y = if latch < 2 { -1.0 } else { 1.0 };
            latches = latches
                + centered_cube(
                    format!("osmolality_evap_correlation_reservoir_{index}_latch_tab_{latch}"),
                    34.0,
                    16.0,
                    14.0,
                )
                .translate(
                    x + side_x * (RESERVOIR_CELL_X / 2.0 + 10.0),
                    y + side_y * (RESERVOIR_CELL_Y / 2.0 + 9.0),
                    RESERVOIR_Z + 7.0,
                );
        }
    }
    latches
}

fn reservoir_sample_flags() -> Part {
    let mut flags = Part::empty("osmolality_evap_correlation_reservoir_sample_flags");
    for index in 0..RESERVOIR_COUNT {
        let (x, y) = reservoir_position(index);
        for flag in 0..RESERVOIR_SAMPLE_FLAGS_PER_NEST {
            flags = flags
                + centered_cube(
                    format!(
                        "osmolality_evap_correlation_reservoir_{index}_sample_flag_{}",
                        sample_flag_name(flag)
                    ),
                    42.0,
                    10.0,
                    8.0,
                )
                .translate(
                    x - 58.0 + flag as f64 * 58.0,
                    y - RESERVOIR_CELL_Y / 2.0 - 24.0,
                    RESERVOIR_Z + 4.0,
                );
        }
    }
    flags
}

fn sample_flag_name(index: usize) -> &'static str {
    match index {
        0 => "t0_mass",
        1 => "t1_osmo",
        _ => "final_mass",
    }
}

fn reservoir_condition_labels() -> Part {
    let mut labels = Part::empty("osmolality_evap_correlation_reservoir_condition_csg_labels");
    for index in 0..RESERVOIR_COUNT {
        let (x, y) = reservoir_position(index);
        labels = labels
            + label_code_card(
                format!("osmolality_evap_correlation_reservoir_{index}_condition_label"),
                72.0,
                26.0,
                LABEL_CODE_BARS,
                index,
            )
            .translate(
                x + RESERVOIR_CELL_X / 2.0 - 48.0,
                y - RESERVOIR_CELL_Y / 2.0 + 18.0,
                RESERVOIR_Z + 8.0,
            );
    }
    labels
}

fn reservoir_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        RESERVOIR_COLS,
        RESERVOIR_COUNT,
        RESERVOIR_PITCH_X,
        RESERVOIR_PITCH_Y,
    )
}

fn mass_osmolality_sample_pockets() -> Part {
    let tray = centered_cube(
        "osmolality_evap_correlation_mass_osmolality_sample_tray",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);
    let spill_shelf = centered_cube(
        "osmolality_evap_correlation_sample_pocket_spill_shelf_relief",
        SAMPLE_X - 36.0,
        SAMPLE_Y - 36.0,
        10.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z - 4.0);

    tray - spill_shelf - sample_pocket_cutouts()
        + sample_pocket_rims()
        + sample_pair_identity_labels()
        + balance_reference_pad()
}

fn sample_pocket_cutouts() -> Part {
    let mut cutouts = Part::empty("osmolality_evap_correlation_sample_pocket_cutouts");
    for index in 0..SAMPLE_PAIR_COUNT {
        let (x, y) = sample_pair_position(index);
        let mass = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_mass_cup_cutout"),
            MASS_POCKET_D / 2.0,
            SAMPLE_POCKET_DEPTH + 2.0,
            36,
        )
        .translate(x - 20.0, y, SAMPLE_Z - SAMPLE_POCKET_DEPTH / 2.0 + 1.0);
        let osmo = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_osmolality_vial_cutout"),
            OSMO_POCKET_D / 2.0,
            SAMPLE_POCKET_DEPTH + 2.0,
            36,
        )
        .translate(x + 27.0, y, SAMPLE_Z - SAMPLE_POCKET_DEPTH / 2.0 + 1.0);
        cutouts = cutouts + mass + osmo;
    }
    cutouts
}

fn sample_pocket_rims() -> Part {
    let mut rims = Part::empty("osmolality_evap_correlation_sample_pocket_raised_rims");
    for index in 0..SAMPLE_PAIR_COUNT {
        let (x, y) = sample_pair_position(index);
        let mass_outer = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_mass_pocket_outer_rim"),
            MASS_POCKET_D / 2.0 + 5.0,
            6.0,
            36,
        )
        .translate(x - 20.0, y, SAMPLE_Z + 3.0);
        let mass_inner = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_mass_pocket_inner_void"),
            MASS_POCKET_D / 2.0 - 1.0,
            8.0,
            36,
        )
        .translate(x - 20.0, y, SAMPLE_Z + 4.0);
        let osmo_outer = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_osmo_pocket_outer_rim"),
            OSMO_POCKET_D / 2.0 + 5.0,
            6.0,
            36,
        )
        .translate(x + 27.0, y, SAMPLE_Z + 3.0);
        let osmo_inner = centered_cylinder(
            format!("osmolality_evap_correlation_sample_pair_{index}_osmo_pocket_inner_void"),
            OSMO_POCKET_D / 2.0 - 1.0,
            8.0,
            36,
        )
        .translate(x + 27.0, y, SAMPLE_Z + 4.0);
        rims = rims + (mass_outer - mass_inner) + (osmo_outer - osmo_inner);
    }
    rims
}

fn sample_pair_identity_labels() -> Part {
    let mut labels = Part::empty("osmolality_evap_correlation_sample_pair_identity_labels");
    for index in 0..SAMPLE_PAIR_COUNT {
        let (x, y) = sample_pair_position(index);
        labels = labels
            + label_code_card(
                format!("osmolality_evap_correlation_sample_pair_{index}_mass_osmo_label"),
                76.0,
                18.0,
                SAMPLE_LABEL_BARS,
                index + 5,
            )
            .translate(x + 2.0, y - 36.0, SAMPLE_Z + 5.0);
    }
    labels
}

fn balance_reference_pad() -> Part {
    let pad = centered_cube(
        "osmolality_evap_correlation_sample_balance_reference_mass_pad",
        SAMPLE_X - 90.0,
        20.0,
        9.0,
    )
    .translate(0.0, SAMPLE_Y / 2.0 - 28.0, SAMPLE_Z + 4.5);
    let center_mark = centered_cube(
        "osmolality_evap_correlation_sample_balance_reference_centerline",
        6.0,
        26.0,
        12.0,
    )
    .translate(0.0, SAMPLE_Y / 2.0 - 28.0, SAMPLE_Z + 6.0);
    pad + center_mark
}

fn sample_pair_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        SAMPLE_COLS,
        SAMPLE_PAIR_COUNT,
        SAMPLE_PITCH_X,
        SAMPLE_PITCH_Y,
    )
}

fn evaporation_witness_coupon_holders() -> Part {
    let tray = centered_cube(
        "osmolality_evap_correlation_evaporation_witness_coupon_holder_tray",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0);
    let channel_relief = centered_cube(
        "osmolality_evap_correlation_coupon_tray_drain_shelf_relief",
        COUPON_X - 42.0,
        28.0,
        10.0,
    )
    .translate(0.0, -COUPON_Y / 2.0 + 32.0, COUPON_Z - 4.0);

    tray - coupon_slot_cutouts() - channel_relief
        + coupon_witness_set()
        + coupon_retention_tabs()
        + evaporation_level_rails()
        + coupon_tray_labels()
}

fn coupon_slot_cutouts() -> Part {
    let mut slots = Part::empty("osmolality_evap_correlation_coupon_slot_cutouts");
    for index in 0..COUPON_COUNT {
        let (x, y) = coupon_position(index);
        slots = slots
            + centered_cube(
                format!("osmolality_evap_correlation_coupon_{index}_holder_recess"),
                WITNESS_COUPON_X + 10.0,
                WITNESS_COUPON_Y + 10.0,
                COUPON_SLOT_DEPTH + 2.0,
            )
            .translate(x, y, COUPON_Z - COUPON_SLOT_DEPTH / 2.0 + 1.0);
    }
    slots
}

fn coupon_witness_set() -> Part {
    let mut coupons = Part::empty("osmolality_evap_correlation_evaporation_witness_coupons");
    for index in 0..COUPON_COUNT {
        let (x, y) = coupon_position(index);
        let body = centered_cube(
            format!("osmolality_evap_correlation_coupon_{index}_witness_coupon_body"),
            WITNESS_COUPON_X,
            WITNESS_COUPON_Y,
            WITNESS_COUPON_Z,
        )
        .translate(x, y, COUPON_Z + WITNESS_COUPON_Z / 2.0);
        let wet_band = centered_cube(
            format!("osmolality_evap_correlation_coupon_{index}_evaporation_exposure_band"),
            WITNESS_COUPON_X - 18.0,
            8.0,
            WITNESS_COUPON_Z + 3.0,
        )
        .translate(x, y - 7.0, COUPON_Z + WITNESS_COUPON_Z / 2.0 + 1.5);
        let dry_band = centered_cube(
            format!("osmolality_evap_correlation_coupon_{index}_condensation_reference_band"),
            WITNESS_COUPON_X - 18.0,
            8.0,
            WITNESS_COUPON_Z + 3.0,
        )
        .translate(x, y + 9.0, COUPON_Z + WITNESS_COUPON_Z / 2.0 + 1.5);
        coupons = coupons + body + wet_band + dry_band;
    }
    coupons
}

fn coupon_retention_tabs() -> Part {
    let mut tabs = Part::empty("osmolality_evap_correlation_coupon_retention_tabs");
    for index in 0..COUPON_COUNT {
        let (x, y) = coupon_position(index);
        tabs =
            tabs + centered_cube(
                format!("osmolality_evap_correlation_coupon_{index}_front_retention_tab"),
                22.0,
                8.0,
                12.0,
            )
            .translate(
                x,
                y - WITNESS_COUPON_Y / 2.0 - 7.0,
                COUPON_Z + WITNESS_COUPON_Z / 2.0,
            ) + centered_cube(
                format!("osmolality_evap_correlation_coupon_{index}_rear_retention_tab"),
                22.0,
                8.0,
                12.0,
            )
            .translate(
                x,
                y + WITNESS_COUPON_Y / 2.0 + 7.0,
                COUPON_Z + WITNESS_COUPON_Z / 2.0,
            );
    }
    tabs
}

fn evaporation_level_rails() -> Part {
    let mut rails = Part::empty("osmolality_evap_correlation_evaporation_level_rails");
    for level in 0..EVAPORATION_LEVELS {
        let y = centered_index(level, EVAPORATION_LEVELS, COUPON_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("osmolality_evap_correlation_evaporation_level_{level}_lane_rail"),
                COUPON_X - 58.0,
                6.0,
                9.0,
            )
            .translate(0.0, y + COUPON_PITCH_Y / 2.0 - 28.0, COUPON_Z + 4.5);
    }
    rails
}

fn coupon_tray_labels() -> Part {
    let low = label_code_card(
        "osmolality_evap_correlation_coupon_low_evaporation_csg_label",
        112.0,
        18.0,
        LABEL_CODE_BARS,
        2,
    )
    .translate(
        -COUPON_X / 2.0 + 74.0,
        COUPON_Y / 2.0 - 22.0,
        COUPON_Z + 5.0,
    );
    let high = label_code_card(
        "osmolality_evap_correlation_coupon_high_evaporation_csg_label",
        112.0,
        18.0,
        LABEL_CODE_BARS,
        9,
    )
    .translate(COUPON_X / 2.0 - 74.0, COUPON_Y / 2.0 - 22.0, COUPON_Z + 5.0);
    low + high
}

fn coupon_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        COUPON_COLS,
        COUPON_COUNT,
        COUPON_PITCH_X,
        COUPON_PITCH_Y,
    )
}

fn humidity_temperature_probe_mounts() -> Part {
    let base = centered_cube(
        "osmolality_evap_correlation_humidity_temperature_probe_mount_base",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    )
    .translate(0.0, 0.0, PROBE_Z / 2.0);
    let cable_trench = centered_cube(
        "osmolality_evap_correlation_probe_cable_trench_cutout",
        PROBE_X - 42.0,
        CABLE_CHANNEL_W,
        PROBE_Z + 4.0,
    )
    .translate(0.0, -PROBE_Y / 2.0 + 28.0, PROBE_Z / 2.0);

    base - probe_bore_cutouts() - cable_trench
        + probe_clamp_blocks()
        + probe_cable_strain_reliefs()
        + probe_reference_labels()
}

fn probe_bore_cutouts() -> Part {
    let mut bores = Part::empty("osmolality_evap_correlation_probe_bore_cutouts");
    for index in 0..PROBE_MOUNTS {
        let x = centered_index(index, PROBE_MOUNTS, PROBE_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("osmolality_evap_correlation_probe_mount_{index}_horizontal_probe_bore"),
                PROBE_BODY_D / 2.0,
                PROBE_CLAMP_W + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, PROBE_Z * 0.62);
    }
    bores
}

fn probe_clamp_blocks() -> Part {
    let mut clamps = Part::empty("osmolality_evap_correlation_probe_clamp_blocks");
    for index in 0..PROBE_MOUNTS {
        let x = centered_index(index, PROBE_MOUNTS, PROBE_PITCH_X);
        let body = centered_cube(
            format!("osmolality_evap_correlation_probe_mount_{index}_split_clamp_body"),
            PROBE_CLAMP_W,
            38.0,
            32.0,
        )
        .translate(x, 0.0, PROBE_Z + 16.0);
        let top_slot = centered_cube(
            format!("osmolality_evap_correlation_probe_mount_{index}_split_clamp_gap"),
            PROBE_CLAMP_W - 12.0,
            6.0,
            36.0,
        )
        .translate(x, 0.0, PROBE_Z + 18.0);
        let screw_land = centered_cylinder(
            format!("osmolality_evap_correlation_probe_mount_{index}_thumb_screw_land"),
            8.0,
            8.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x + PROBE_CLAMP_W / 2.0 + 3.0, 0.0, PROBE_Z + 24.0);
        clamps = clamps + (body - top_slot) + screw_land;
    }
    clamps
}

fn probe_cable_strain_reliefs() -> Part {
    let mut reliefs = Part::empty("osmolality_evap_correlation_probe_cable_strain_reliefs");
    for index in 0..PROBE_MOUNTS {
        let x = centered_index(index, PROBE_MOUNTS, PROBE_PITCH_X);
        reliefs = reliefs
            + centered_cube(
                format!("osmolality_evap_correlation_probe_mount_{index}_cable_keeper_bridge"),
                32.0,
                12.0,
                10.0,
            )
            .translate(x, -PROBE_Y / 2.0 + 28.0, PROBE_Z + 5.0);
    }
    reliefs
}

fn probe_reference_labels() -> Part {
    let rh = label_code_card(
        "osmolality_evap_correlation_relative_humidity_probe_csg_label",
        130.0,
        18.0,
        LABEL_CODE_BARS,
        4,
    )
    .translate(-135.0, PROBE_Y / 2.0 - 22.0, PROBE_Z + 5.0);
    let temp = label_code_card(
        "osmolality_evap_correlation_temperature_probe_csg_label",
        130.0,
        18.0,
        LABEL_CODE_BARS,
        11,
    )
    .translate(135.0, PROBE_Y / 2.0 - 22.0, PROBE_Z + 5.0);
    rh + temp
}

fn condensate_return_channels() -> Part {
    let plate = centered_cube(
        "osmolality_evap_correlation_condensate_return_channel_plate",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    )
    .translate(0.0, 0.0, CONDENSATE_Z / 2.0);
    let return_manifold = centered_cylinder(
        "osmolality_evap_correlation_condensate_common_return_manifold_cutout",
        14.0,
        CONDENSATE_X - 60.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -CONDENSATE_Y / 2.0 + 36.0, CONDENSATE_Z - 8.0);

    plate - condensate_channel_cutouts() - return_manifold
        + condensate_channel_lips()
        + condensate_collection_cups()
        + flow_direction_markers()
        + condensate_return_label()
}

fn condensate_channel_cutouts() -> Part {
    let mut channels = Part::empty("osmolality_evap_correlation_condensate_channel_cutouts");
    for index in 0..RETURN_CHANNELS {
        let x = centered_index(index, RETURN_CHANNELS, 82.0);
        channels = channels
            + centered_cube(
                format!(
                    "osmolality_evap_correlation_condensate_return_channel_{index}_open_gutter"
                ),
                RETURN_CHANNEL_W,
                CONDENSATE_Y - 38.0,
                RETURN_CHANNEL_DEPTH + 2.0,
            )
            .translate(x, 4.0, CONDENSATE_Z - RETURN_CHANNEL_DEPTH / 2.0 + 1.0);
    }
    channels
}

fn condensate_channel_lips() -> Part {
    let mut lips = Part::empty("osmolality_evap_correlation_condensate_channel_raised_lips");
    for index in 0..RETURN_CHANNELS {
        let x = centered_index(index, RETURN_CHANNELS, 82.0);
        lips = lips
            + centered_cube(
                format!("osmolality_evap_correlation_condensate_return_channel_{index}_left_lip"),
                5.0,
                CONDENSATE_Y - 28.0,
                8.0,
            )
            .translate(x - RETURN_CHANNEL_W / 2.0 - 5.0, 0.0, CONDENSATE_Z + 4.0)
            + centered_cube(
                format!("osmolality_evap_correlation_condensate_return_channel_{index}_right_lip"),
                5.0,
                CONDENSATE_Y - 28.0,
                8.0,
            )
            .translate(x + RETURN_CHANNEL_W / 2.0 + 5.0, 0.0, CONDENSATE_Z + 4.0);
    }
    lips
}

fn condensate_collection_cups() -> Part {
    let mut cups = Part::empty("osmolality_evap_correlation_condensate_collection_cups");
    for index in 0..RETURN_CUP_COUNT {
        let x = centered_index(index, RETURN_CUP_COUNT, 95.0);
        let outer = centered_cylinder(
            format!("osmolality_evap_correlation_condensate_return_cup_{index}_outer"),
            20.0,
            14.0,
            36,
        )
        .translate(x, -CONDENSATE_Y / 2.0 + 34.0, CONDENSATE_Z + 7.0);
        let inner = centered_cylinder(
            format!("osmolality_evap_correlation_condensate_return_cup_{index}_inner_void"),
            12.0,
            16.0,
            36,
        )
        .translate(x, -CONDENSATE_Y / 2.0 + 34.0, CONDENSATE_Z + 8.0);
        cups = cups + (outer - inner);
    }
    cups
}

fn flow_direction_markers() -> Part {
    let mut markers = Part::empty("osmolality_evap_correlation_condensate_flow_direction_markers");
    for index in 0..FLOW_ARROW_COUNT {
        let x = centered_index(index % 5, 5, 82.0);
        let y = -38.0 + (index / 5) as f64 * 48.0;
        let shaft = centered_cube(
            format!("osmolality_evap_correlation_condensate_flow_arrow_{index}_shaft"),
            5.0,
            28.0,
            5.0,
        )
        .translate(x, y, CONDENSATE_Z + 2.5);
        let head = centered_cube(
            format!("osmolality_evap_correlation_condensate_flow_arrow_{index}_head"),
            17.0,
            8.0,
            5.0,
        )
        .translate(x, y - 16.0, CONDENSATE_Z + 2.5);
        markers = markers + shaft + head;
    }
    markers
}

fn condensate_return_label() -> Part {
    label_code_card(
        "osmolality_evap_correlation_condensate_return_channel_csg_label",
        150.0,
        18.0,
        LABEL_CODE_BARS,
        14,
    )
    .translate(0.0, CONDENSATE_Y / 2.0 - 22.0, CONDENSATE_Z + 5.0)
}

fn calibration_vial_rack() -> Part {
    let rack = centered_cube(
        "osmolality_evap_correlation_calibration_vial_rack_body",
        CAL_X,
        CAL_Y,
        CAL_Z,
    )
    .translate(0.0, 0.0, CAL_Z / 2.0);

    rack - calibration_vial_cutouts()
        + calibration_vial_rims()
        + calibration_label_windows()
        + calibration_control_token_strip()
}

fn calibration_vial_cutouts() -> Part {
    let mut cutouts = Part::empty("osmolality_evap_correlation_calibration_vial_cutouts");
    for index in 0..CAL_VIALS {
        let x = centered_index(index, CAL_VIALS, CAL_VIAL_PITCH_X);
        cutouts = cutouts
            + centered_cylinder(
                format!("osmolality_evap_correlation_calibration_vial_{index}_well_cutout"),
                CAL_VIAL_D / 2.0,
                CAL_Z + 4.0,
                36,
            )
            .translate(x, 12.0, CAL_Z / 2.0);
    }
    cutouts
}

fn calibration_vial_rims() -> Part {
    let mut rims = Part::empty("osmolality_evap_correlation_calibration_vial_rims");
    for index in 0..CAL_VIALS {
        let x = centered_index(index, CAL_VIALS, CAL_VIAL_PITCH_X);
        let outer = centered_cylinder(
            format!("osmolality_evap_correlation_calibration_vial_{index}_cap_clearance_rim"),
            CAL_VIAL_D / 2.0 + 5.0,
            7.0,
            36,
        )
        .translate(x, 12.0, CAL_Z + 3.5);
        let inner = centered_cylinder(
            format!("osmolality_evap_correlation_calibration_vial_{index}_open_label_void"),
            CAL_VIAL_D / 2.0 - 1.0,
            8.0,
            36,
        )
        .translate(x, 12.0, CAL_Z + 4.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn calibration_label_windows() -> Part {
    let mut windows = Part::empty("osmolality_evap_correlation_calibration_label_windows");
    for index in 0..CAL_LABEL_WINDOWS {
        let x = centered_index(index, CAL_LABEL_WINDOWS, 78.0);
        windows = windows
            + centered_cube(
                format!("osmolality_evap_correlation_calibration_label_window_{index}"),
                58.0,
                16.0,
                6.0,
            )
            .translate(x, -CAL_Y / 2.0 + 24.0, CAL_Z + 3.0);
    }
    windows
}

fn calibration_control_token_strip() -> Part {
    label_code_card(
        "osmolality_evap_correlation_calibration_control_vial_csg_label",
        170.0,
        18.0,
        LABEL_CODE_BARS,
        22,
    )
    .translate(0.0, CAL_Y / 2.0 - 22.0, CAL_Z + 5.0)
}

fn exposure_conditioning_manifold() -> Part {
    let rear_header = centered_cube(
        "osmolality_evap_correlation_exposure_conditioning_rear_header",
        EXPOSURE_X,
        20.0,
        EXPOSURE_Z,
    )
    .translate(0.0, EXPOSURE_Y / 2.0 - 10.0, EXPOSURE_Z / 2.0);
    let front_header = centered_cube(
        "osmolality_evap_correlation_exposure_conditioning_front_header",
        EXPOSURE_X,
        18.0,
        46.0,
    )
    .translate(0.0, -EXPOSURE_Y / 2.0 + 9.0, 23.0);
    let overhead_beam = centered_cube(
        "osmolality_evap_correlation_exposure_conditioning_overhead_beam",
        EXPOSURE_X,
        16.0,
        24.0,
    )
    .translate(0.0, 0.0, EXPOSURE_Z - 12.0);

    rear_header
        + front_header
        + overhead_beam
        + exposure_vapor_ports()
        + exposure_windows()
        + condensation_baffles()
        + exposure_lane_labels()
}

fn exposure_vapor_ports() -> Part {
    let mut ports = Part::empty("osmolality_evap_correlation_exposure_vapor_ports");
    for index in 0..EXPOSURE_LANES {
        let x = centered_index(index, EXPOSURE_LANES, 145.0);
        let collar = centered_cylinder(
            format!("osmolality_evap_correlation_exposure_lane_{index}_vapor_port_collar"),
            VAPOR_PORT_D / 2.0 + 7.0,
            12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, EXPOSURE_Y / 2.0 - 20.0, EXPOSURE_Z * 0.68);
        let bore = centered_cylinder(
            format!("osmolality_evap_correlation_exposure_lane_{index}_vapor_port_bore"),
            VAPOR_PORT_D / 2.0,
            14.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, EXPOSURE_Y / 2.0 - 20.0, EXPOSURE_Z * 0.68);
        ports = ports + (collar - bore);
    }
    ports
}

fn exposure_windows() -> Part {
    let mut windows = Part::empty("osmolality_evap_correlation_reservoir_exposure_windows");
    for index in 0..EXPOSURE_WINDOW_COUNT {
        let x = centered_index(index, EXPOSURE_WINDOW_COUNT, 130.0);
        let open_frame = rectangular_frame(
            format!("osmolality_evap_correlation_exposure_window_{index}_open_frame"),
            94.0,
            36.0,
            7.0,
            8.0,
        )
        .translate(x, -4.0, EXPOSURE_Z - 18.0);
        let shutter_flag = centered_cube(
            format!("osmolality_evap_correlation_exposure_window_{index}_shutter_position_flag"),
            34.0,
            7.0,
            12.0,
        )
        .translate(x, -EXPOSURE_Y / 2.0 + 18.0, EXPOSURE_Z - 18.0);
        windows = windows + open_frame + shutter_flag;
    }
    windows
}

fn condensation_baffles() -> Part {
    let mut baffles = Part::empty("osmolality_evap_correlation_condensation_baffles");
    for index in 0..CONDENSATION_BAFFLES {
        let x = centered_index(index, CONDENSATION_BAFFLES, 92.0);
        baffles = baffles
            + centered_cube(
                format!("osmolality_evap_correlation_condensation_baffle_{index}"),
                9.0,
                EXPOSURE_Y - 16.0,
                40.0,
            )
            .translate(x, 0.0, EXPOSURE_Z / 2.0);
    }
    baffles
}

fn exposure_lane_labels() -> Part {
    let mut labels = Part::empty("osmolality_evap_correlation_exposure_lane_csg_labels");
    for index in 0..EXPOSURE_LANES {
        labels = labels
            + label_code_card(
                format!("osmolality_evap_correlation_exposure_lane_{index}_csg_label"),
                88.0,
                16.0,
                LABEL_CODE_BARS,
                index + 30,
            )
            .translate(
                centered_index(index, EXPOSURE_LANES, 145.0),
                EXPOSURE_Y / 2.0 - 12.0,
                EXPOSURE_Z + 5.0,
            );
    }
    labels
}

fn correlation_label_grid() -> Part {
    let spine = centered_cube(
        "osmolality_evap_correlation_csg_label_grid_spine",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    )
    .translate(0.0, 0.0, LABEL_Z / 2.0);
    spine + axis_labels() + correlation_tokens() + run_card_label()
}

fn axis_labels() -> Part {
    let mut labels = Part::empty("osmolality_evap_correlation_axis_csg_labels");
    for index in 0..AXIS_LABELS {
        labels = labels
            + label_code_card(
                format!("osmolality_evap_correlation_axis_label_{index}"),
                78.0,
                20.0,
                LABEL_CODE_BARS,
                index + 40,
            )
            .translate(
                centered_index(index, AXIS_LABELS, 94.0),
                18.0,
                LABEL_Z + 5.0,
            );
    }
    labels
}

fn correlation_tokens() -> Part {
    let mut tokens = Part::empty("osmolality_evap_correlation_raised_correlation_tokens");
    for index in 0..CORRELATION_TOKEN_COUNT {
        let col = index % 6;
        let row = index / 6;
        let x = centered_index(col, 6, 88.0);
        let y = -22.0 + row as f64 * 18.0;
        let token = centered_cube(
            format!("osmolality_evap_correlation_token_{index}_mass_osmo_evap_link"),
            54.0,
            10.0,
            6.0,
        )
        .translate(x, y, LABEL_Z + 3.0);
        let dot = centered_cylinder(
            format!("osmolality_evap_correlation_token_{index}_paired_measurement_dot"),
            4.5,
            7.0,
            18,
        )
        .translate(x + 22.0, y, LABEL_Z + 3.5);
        tokens = tokens + token + dot;
    }
    tokens
}

fn run_card_label() -> Part {
    label_code_card(
        "osmolality_evap_correlation_run_card_csg_label_plate",
        150.0,
        22.0,
        LABEL_CODE_BARS,
        51,
    )
    .translate(-LABEL_X / 2.0 + 92.0, 0.0, LABEL_Z + 5.0)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "osmolality_evap_correlation_evidence_bridge_left_post",
        36.0,
        CAMERA_Y,
        CAMERA_CLEARANCE_Z,
    )
    .translate(-CAMERA_X / 2.0 + 18.0, 0.0, CAMERA_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "osmolality_evap_correlation_evidence_bridge_right_post",
        36.0,
        CAMERA_Y,
        CAMERA_CLEARANCE_Z,
    )
    .translate(CAMERA_X / 2.0 - 18.0, 0.0, CAMERA_CLEARANCE_Z / 2.0);
    let beam = centered_cube(
        "osmolality_evap_correlation_evidence_bridge_camera_beam",
        CAMERA_X,
        CAMERA_Y,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0);

    left_post + right_post + beam + camera_lands() + evidence_light_bars()
}

fn camera_lands() -> Part {
    let mut lands = Part::empty("osmolality_evap_correlation_evidence_camera_lands");
    for index in 0..CAMERA_COUNT {
        let x = centered_index(index, CAMERA_COUNT, 245.0);
        let body = centered_cube(
            format!("osmolality_evap_correlation_evidence_camera_{index}_mount_land"),
            70.0,
            32.0,
            14.0,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z + 7.0);
        let lens = centered_cylinder(
            format!("osmolality_evap_correlation_evidence_camera_{index}_lens_axis_marker"),
            11.0,
            8.0,
            32,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z + 18.0);
        lands = lands + body + lens;
    }
    lands
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("osmolality_evap_correlation_evidence_light_bars");
    for index in 0..LIGHT_BAR_COUNT {
        let y = if index == 0 {
            -CAMERA_Y / 2.0 - 12.0
        } else {
            CAMERA_Y / 2.0 + 12.0
        };
        bars = bars
            + centered_cube(
                format!("osmolality_evap_correlation_evidence_light_bar_{index}"),
                CAMERA_X - 210.0,
                12.0,
                10.0,
            )
            .translate(0.0, y, CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z + 5.0);
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    rectangular_frame(
        "osmolality_evap_correlation_overall_robot_service_keepout_frame",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    ) + keepout_zone_markers()
}

fn keepout_zone_markers() -> Part {
    let front = centered_cube(
        "osmolality_evap_correlation_front_robot_approach_keepout_gauge",
        KEEP_OUT_X,
        FRONT_ROBOT_CLEARANCE / 8.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -KEEP_OUT_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 16.0,
        KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "osmolality_evap_correlation_rear_environmental_service_keepout_gauge",
        KEEP_OUT_X,
        REAR_ENVIRONMENTAL_SERVICE / 8.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        KEEP_OUT_Y / 2.0 - REAR_ENVIRONMENTAL_SERVICE / 16.0,
        KEEP_OUT_Z / 2.0,
    );
    let left = centered_cube(
        "osmolality_evap_correlation_left_balance_service_keepout_gauge",
        SIDE_BALANCE_SERVICE / 8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -KEEP_OUT_X / 2.0 + SIDE_BALANCE_SERVICE / 16.0,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let top = centered_cube(
        "osmolality_evap_correlation_top_probe_clearance_gauge",
        130.0,
        36.0,
        18.0,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 100.0,
        KEEP_OUT_Y / 2.0 - 44.0,
        TOP_PROBE_CLEARANCE,
    );
    front + rear + left + top
}

fn rectangular_frame(name: impl Into<String>, x: f64, y: f64, rail: f64, z: f64) -> Part {
    let name = name.into();
    let front = centered_cube(format!("{name}_front_rail"), x, rail, z).translate(
        0.0,
        -y / 2.0 + rail / 2.0,
        z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail, z).translate(
        0.0,
        y / 2.0 - rail / 2.0,
        z / 2.0,
    );
    let left = centered_cube(format!("{name}_left_rail"), rail, y, z).translate(
        -x / 2.0 + rail / 2.0,
        0.0,
        z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, y, z).translate(
        x / 2.0 - rail / 2.0,
        0.0,
        z / 2.0,
    );
    front + rear + left + right
}

fn label_code_card(
    name: impl Into<String>,
    width: f64,
    depth: f64,
    bars: usize,
    code: usize,
) -> Part {
    let name = name.into();
    let plaque =
        centered_cube(format!("{name}_plaque"), width, depth, 3.0).translate(0.0, 0.0, 1.5);
    let mut code_bars = Part::empty(format!("{name}_raised_code_bars"));
    let spacing = (width - 18.0) / bars as f64;
    for bar in 0..bars {
        let bit = (code + bar * 3) % 5;
        let bar_h = 4.0 + bit as f64 * 1.2;
        code_bars = code_bars
            + centered_cube(
                format!("{name}_raised_code_bar_{bar}"),
                4.0,
                depth - 8.0,
                bar_h,
            )
            .translate(
                -width / 2.0 + 9.0 + bar as f64 * spacing,
                0.0,
                3.0 + bar_h / 2.0,
            );
    }
    let start_dot = centered_cylinder(format!("{name}_start_dot"), 3.0, 4.0, 18).translate(
        width / 2.0 - 10.0,
        -depth / 2.0 + 8.0,
        5.0,
    );
    plaque + code_bars + start_dot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_named_for_the_station() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    }

    #[test]
    fn required_validation_features_are_represented() {
        for required in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|output| output.contains(required))
                    || required == "csg_label_geometry"
                    || required == "named_stl_outputs",
                "{required} is not represented by an output path"
            );
        }
    }

    #[test]
    fn surface_modules_fit_without_xy_conflicts() {
        let rects = deck_module_rects();
        for rect in rects {
            assert!(rect.fits_inside_station(CURB_W + 12.0));
        }
        for a in 0..rects.len() {
            for b in (a + 1)..rects.len() {
                assert!(!rects[a].overlaps(rects[b]));
            }
        }
    }

    #[test]
    fn correlation_station_has_more_capture_than_challenge_volume() {
        assert!(condensate_return_capacity_ml() > evaporation_challenge_volume_ml());
        assert!(calibration_capacity_ml() > SAMPLE_PAIR_COUNT as f64 * 0.8);
    }
}
