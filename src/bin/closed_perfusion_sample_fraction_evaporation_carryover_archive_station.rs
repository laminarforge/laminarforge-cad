use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion sample fraction archive validation station.
//
// Intent:
// - Retain closed media/analyte fractions from multi-chip perfusion runs.
// - Challenge the archive workflow for evaporation and high-to-low carryover.
// - Make cold-block nests, sealed receivers, timed fraction tokens, carryover
//   standards, evaporation mass witnesses, seal coupons, custody lands,
//   flush/waste routing, release/quarantine gates, camera evidence geometry,
//   and robot service datums physically explicit in the CAD.
//
// This is mechanical validation-station concept CAD only. It is not an
// analytical method, acceptance criterion, storage-life claim, or sterility
// assurance claim.

const OUTPUT_PREFIX: &str =
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_base_containment_deck.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_cold_block_archive_nests.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_sealed_fraction_receivers.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_timed_fraction_token_rail.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_high_low_carryover_standard_wells.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_evaporation_mass_witness_pads.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_cap_seal_integrity_coupons.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_barcode_rfid_custody_lands.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_waste_flush_routing_manifold.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_quarantine_release_gates.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_camera_evidence_robot_datums_bridge.stl",
    "output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "cold_block_archive_nests",
    "sealed_fraction_receivers",
    "timed_fraction_token_rail",
    "high_low_carryover_standard_wells",
    "evaporation_mass_witness_pads",
    "cap_seal_integrity_coupons",
    "barcode_rfid_custody_lands",
    "waste_flush_routing",
    "quarantine_release_gates",
    "camera_evidence_bridge",
    "robotic_service_datums",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 48.0;
const BASIN_DEPTH: f64 = 8.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 16.0;
const MODULE_MARGIN_MM: f64 = 32.0;
const MODULE_GAP_MM: f64 = 4.0;

const CHIP_COUNT: usize = 6;
const FRACTION_TIMEPOINTS: usize = 8;
const FRACTIONS_PER_CHIP: usize = FRACTION_TIMEPOINTS;
const FRACTION_RECEIVERS: usize = CHIP_COUNT * FRACTIONS_PER_CHIP;
const FRACTION_COLS: usize = FRACTION_TIMEPOINTS;
const FRACTION_ROWS: usize = CHIP_COUNT;
const FRACTION_PITCH_X: f64 = 43.0;
const FRACTION_PITCH_Y: f64 = 39.0;

const COLD_BLOCK_POS: (f64, f64) = (-450.0, 185.0);
const COLD_BLOCK_X: f64 = 430.0;
const COLD_BLOCK_Y: f64 = 300.0;
const COLD_BLOCK_Z: f64 = 52.0;
const COLD_BLOCK_STANDOFF_Z: f64 = 6.0;
const COLD_NEST_D: f64 = 20.5;
const COOLANT_BORE_D: f64 = 9.5;
const THERMISTOR_BORE_D: f64 = 3.8;
const COLD_DATUMS: usize = 4;

const RECEIVER_RACK_X: f64 = 398.0;
const RECEIVER_RACK_Y: f64 = 260.0;
const RECEIVER_RACK_Z: f64 = 28.0;
const RECEIVER_TUBE_D: f64 = 14.8;
const RECEIVER_SEAL_RING_D: f64 = 20.0;
const RECEIVER_CAP_Z: f64 = 8.0;
const RECEIVER_SEPTUM_D: f64 = 5.2;

const TOKEN_POS: (f64, f64) = (0.0, 405.0);
const TOKEN_RAIL_X: f64 = 530.0;
const TOKEN_RAIL_Y: f64 = 86.0;
const TOKEN_RAIL_Z: f64 = 28.0;
const TOKEN_SLOTS: usize = FRACTION_TIMEPOINTS;
const TOKEN_PITCH_X: f64 = 58.0;
const TOKEN_CARD_X: f64 = 42.0;
const TOKEN_CARD_Y: f64 = 52.0;

const STANDARD_POS: (f64, f64) = (410.0, 190.0);
const STANDARD_BLOCK_X: f64 = 430.0;
const STANDARD_BLOCK_Y: f64 = 170.0;
const STANDARD_BLOCK_Z: f64 = 48.0;
const CARRYOVER_STANDARD_WELLS: usize = 16;
const STANDARD_PITCH_X: f64 = 24.0;
const HIGH_STANDARD_D: f64 = 19.0;
const LOW_STANDARD_D: f64 = 15.5;
const STANDARD_SEAL_LANDS: usize = 4;

const MASS_WITNESS_POS: (f64, f64) = (-455.0, -205.0);
const MASS_WITNESS_X: f64 = 400.0;
const MASS_WITNESS_Y: f64 = 230.0;
const MASS_WITNESS_Z: f64 = 34.0;
const MASS_WITNESS_PADS: usize = 12;
const MASS_WITNESS_COLS: usize = 4;
const MASS_WITNESS_PITCH_X: f64 = 86.0;
const MASS_WITNESS_PITCH_Y: f64 = 62.0;
const MASS_PAD_X: f64 = 62.0;
const MASS_PAD_Y: f64 = 38.0;
const MASS_PAD_Z: f64 = 7.0;
const BALANCE_PIN_D: f64 = 3.0;

const COUPON_POS: (f64, f64) = (-20.0, -210.0);
const COUPON_BLOCK_X: f64 = 360.0;
const COUPON_BLOCK_Y: f64 = 230.0;
const COUPON_BLOCK_Z: f64 = 36.0;
const CAP_SEAL_COUPONS: usize = 12;
const COUPON_COLS: usize = 4;
const COUPON_PITCH_X: f64 = 76.0;
const COUPON_PITCH_Y: f64 = 62.0;
const CAP_TORQUE_DISC_D: f64 = 28.0;
const SEAL_PULL_TAB_X: f64 = 46.0;
const SEAL_PULL_TAB_Y: f64 = 24.0;

const CUSTODY_POS: (f64, f64) = (405.0, -210.0);
const CUSTODY_BLOCK_X: f64 = 380.0;
const CUSTODY_BLOCK_Y: f64 = 230.0;
const CUSTODY_BLOCK_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 6;
const RUN_RECORD_CARD_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 70.0;
const BARCODE_LAND_Y: f64 = 22.0;
const RFID_LAND_X: f64 = 42.0;
const RFID_LAND_Y: f64 = 34.0;

const ROUTING_POS: (f64, f64) = (60.0, -35.0);
const ROUTING_X: f64 = 830.0;
const ROUTING_Y: f64 = 100.0;
const ROUTING_Z: f64 = 54.0;
const FLUSH_PORTS: usize = CHIP_COUNT;
const WASTE_PORTS: usize = CHIP_COUNT;
const BLANK_BRANCH_PORTS: usize = 2;
const ROUTING_PORT_PITCH_X: f64 = 104.0;
const FLUSH_BORE_D: f64 = 5.6;
const WASTE_BORE_D: f64 = 9.0;
const WASTE_DROP_D: f64 = 22.0;

const GATE_POS: (f64, f64) = (485.0, 405.0);
const GATE_BLOCK_X: f64 = 360.0;
const GATE_BLOCK_Y: f64 = 86.0;
const GATE_BLOCK_Z: f64 = 32.0;
const DISPOSITION_GATES: usize = 3;
const GATE_PITCH_X: f64 = 108.0;
const GATE_SLOT_X: f64 = 78.0;
const GATE_SLOT_Y: f64 = 38.0;

const BRIDGE_POS: (f64, f64) = (0.0, 20.0);
const BRIDGE_X: f64 = 1260.0;
const BRIDGE_Y: f64 = 72.0;
const BRIDGE_Z: f64 = 230.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 4;
const ROBOT_SERVICE_DATUMS: usize = 6;
const CAMERA_CLEARANCE_Z: f64 = 178.0;
const ROBOT_Z_CLEARANCE: f64 = 245.0;

const TUBE_ROUTE_COUNT: usize = CHIP_COUNT * 2 + BLANK_BRANCH_PORTS;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self, margin: f64) -> bool {
        self.center.0 - self.x / 2.0 >= -STATION_X / 2.0 + margin
            && self.center.0 + self.x / 2.0 <= STATION_X / 2.0 - margin
            && self.center.1 - self.y / 2.0 >= -STATION_Y / 2.0 + margin
            && self.center.1 + self.y / 2.0 <= STATION_Y / 2.0 - margin
    }

    fn overlaps(self, other: Footprint, margin: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + margin && dy < (self.y + other.y) / 2.0 + margin
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let cold_block = cold_block_archive_nests();
    export(OUTPUTS[1], &cold_block);

    let receivers = sealed_fraction_receivers();
    export(OUTPUTS[2], &receivers);

    let token_rail = timed_fraction_token_rail();
    export(OUTPUTS[3], &token_rail);

    let standards = high_low_carryover_standard_wells();
    export(OUTPUTS[4], &standards);

    let mass_witness = evaporation_mass_witness_pads();
    export(OUTPUTS[5], &mass_witness);

    let coupons = cap_seal_integrity_coupons();
    export(OUTPUTS[6], &coupons);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[7], &custody);

    let routing = waste_flush_routing_manifold();
    export(OUTPUTS[8], &routing);

    let gates = quarantine_release_gates();
    export(OUTPUTS[9], &gates);

    let bridge = camera_evidence_robot_datums_bridge();
    export(OUTPUTS[10], &bridge);

    let assembly = deck
        + cold_block.translate(COLD_BLOCK_POS.0, COLD_BLOCK_POS.1, on_deck_z(COLD_BLOCK_Z))
        + receivers.translate(
            COLD_BLOCK_POS.0,
            COLD_BLOCK_POS.1,
            BASE_Z + COLD_BLOCK_Z + COLD_BLOCK_STANDOFF_Z + RECEIVER_RACK_Z / 2.0,
        )
        + token_rail.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_RAIL_Z))
        + standards.translate(STANDARD_POS.0, STANDARD_POS.1, on_deck_z(STANDARD_BLOCK_Z))
        + mass_witness.translate(
            MASS_WITNESS_POS.0,
            MASS_WITNESS_POS.1,
            on_deck_z(MASS_WITNESS_Z),
        )
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_BLOCK_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_BLOCK_Z))
        + routing.translate(ROUTING_POS.0, ROUTING_POS.1, on_deck_z(ROUTING_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_BLOCK_Z))
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, on_deck_z(BRIDGE_Z))
        + closed_fraction_route_placeholders();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed perfusion sample fraction evaporation/carryover archive station:");
    println!("  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Multi-chip archive:        {CHIP_COUNT} chips x {FRACTIONS_PER_CHIP} timed fractions = {FRACTION_RECEIVERS} sealed receivers on a cold block"
    );
    println!(
        "  Carryover challenge:       {CARRYOVER_STANDARD_WELLS} alternating high/low standard wells, {FLUSH_PORTS} flush ports, {WASTE_PORTS} waste drops, {BLANK_BRANCH_PORTS} blank challenge branches"
    );
    println!(
        "  Evaporation evidence:      {MASS_WITNESS_PADS} mass witness pads and {CAP_SEAL_COUPONS} cap/seal integrity coupons"
    );
    println!(
        "  Custody and disposition:   {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {RUN_RECORD_CARD_LANDS} run-record cards, {DISPOSITION_GATES} release/quarantine gates"
    );
    println!(
        "  Evidence and service:      {CAMERA_COUNT} camera lands, {LIGHT_BAR_COUNT} light bars, {ROBOT_SERVICE_DATUMS} robotic service datums, camera clearance {CAMERA_CLEARANCE_Z:.0}mm"
    );
    println!("  Outputs exported:          {}", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z + part_z / 2.0
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
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert_eq!(FRACTION_RECEIVERS, CHIP_COUNT * FRACTIONS_PER_CHIP);
    assert_eq!(FRACTION_RECEIVERS, FRACTION_COLS * FRACTION_ROWS);
    assert_eq!(FRACTION_TIMEPOINTS, TOKEN_SLOTS);
    assert_eq!(high_standard_count(), low_standard_count());
    assert_eq!(FLUSH_PORTS, CHIP_COUNT);
    assert_eq!(WASTE_PORTS, CHIP_COUNT);
    assert_eq!(COLD_DATUMS, 4);
    assert_eq!(TUBE_ROUTE_COUNT, CHIP_COUNT * 2 + BLANK_BRANCH_PORTS);
    assert!(COLD_NEST_D > RECEIVER_TUBE_D + 2.0);
    assert!(RECEIVER_SEAL_RING_D > RECEIVER_TUBE_D);
    assert!(CAMERA_CLEARANCE_Z > COLD_BLOCK_Z + RECEIVER_RACK_Z + RECEIVER_CAP_Z);
    assert!(ROBOT_Z_CLEARANCE > BRIDGE_Z);

    let rects = deck_module_footprints();
    for rect in rects {
        assert!(
            rect.fits_inside_station(MODULE_MARGIN_MM),
            "{} exceeds station envelope",
            rect.name
        );
    }

    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b], MODULE_GAP_MM),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn deck_module_footprints() -> [Footprint; 8] {
    [
        footprint(
            "cold_block_archive_stack",
            COLD_BLOCK_POS,
            COLD_BLOCK_X,
            COLD_BLOCK_Y,
        ),
        footprint(
            "timed_fraction_token_rail",
            TOKEN_POS,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
        footprint(
            "high_low_carryover_standard_wells",
            STANDARD_POS,
            STANDARD_BLOCK_X,
            STANDARD_BLOCK_Y,
        ),
        footprint(
            "evaporation_mass_witness_pads",
            MASS_WITNESS_POS,
            MASS_WITNESS_X,
            MASS_WITNESS_Y,
        ),
        footprint(
            "cap_seal_integrity_coupons",
            COUPON_POS,
            COUPON_BLOCK_X,
            COUPON_BLOCK_Y,
        ),
        footprint(
            "barcode_rfid_custody_lands",
            CUSTODY_POS,
            CUSTODY_BLOCK_X,
            CUSTODY_BLOCK_Y,
        ),
        footprint(
            "waste_flush_routing_manifold",
            ROUTING_POS,
            ROUTING_X,
            ROUTING_Y,
        ),
        footprint(
            "quarantine_release_gates",
            GATE_POS,
            GATE_BLOCK_X,
            GATE_BLOCK_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "evap_carryover_archive_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "evap_carryover_archive_recessed_secondary_containment_basin",
        STATION_X - 118.0,
        STATION_Y - 112.0,
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -6.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.3);
    let drain = centered_cylinder(
        "evap_carryover_archive_front_drain_to_waste_sensor",
        DRAIN_D / 2.0,
        72.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 8.0,
        BASE_Z * 0.48,
    );

    deck - basin - drain - module_socket_cutouts() - mounting_holes()
        + containment_rims()
        + leak_flow_gutters()
        + deck_zone_dividers()
        + deck_datum_targets()
}

fn module_socket_cutouts() -> Part {
    let mut sockets = Part::empty("evap_carryover_archive_module_socket_cutouts");
    for rect in deck_module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("evap_carryover_archive_{}_socket", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("evap_carryover_archive_mounting_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 50.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 50.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 50.0),
        (0.0, -STATION_Y / 2.0 + 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("evap_carryover_archive_m6_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 6.0,
            28,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("evap_carryover_archive_mount_slot_relief_{i}"),
            26.0,
            7.4,
            BASE_Z + 6.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        holes = holes + hole + slot;
    }
    holes
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "evap_carryover_archive_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "evap_carryover_archive_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "evap_carryover_archive_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "evap_carryover_archive_front_low_service_lip",
        STATION_X - 180.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z + 12.0);

    left + right + rear + front
}

fn leak_flow_gutters() -> Part {
    let mut gutters = Part::empty("evap_carryover_archive_leak_flow_gutters");
    for i in 0..6 {
        let x = centered_index(i, 6, 210.0);
        gutters = gutters
            + centered_cube(
                format!("evap_carryover_archive_leak_flow_rib_{i}"),
                8.0,
                STATION_Y - 170.0,
                6.0,
            )
            .translate(x, -18.0, BASE_Z + 3.0);
    }

    let drain_weir = centered_cube("evap_carryover_archive_drain_weir", 160.0, 10.0, 8.0)
        .translate(
            STATION_X / 2.0 - 150.0,
            -STATION_Y / 2.0 + 64.0,
            BASE_Z + 4.0,
        );

    gutters + drain_weir
}

fn deck_zone_dividers() -> Part {
    let wet_dry = centered_cube(
        "evap_carryover_archive_wet_routing_to_dry_custody_divider",
        STATION_X - 154.0,
        10.0,
        28.0,
    )
    .translate(0.0, -88.0, BASE_Z + 14.0);
    let archive_to_standards = centered_cube(
        "evap_carryover_archive_archive_to_standards_divider",
        10.0,
        350.0,
        28.0,
    )
    .translate(-10.0, 210.0, BASE_Z + 14.0);
    let disposition_split = centered_cube(
        "evap_carryover_archive_disposition_gate_dry_lane_divider",
        10.0,
        116.0,
        34.0,
    )
    .translate(270.0, 405.0, BASE_Z + 17.0);

    wet_dry + archive_to_standards + disposition_split
}

fn deck_datum_targets() -> Part {
    let mut datums = Part::empty("evap_carryover_archive_deck_robot_datum_targets");
    for (i, (x, y)) in [
        (-690.0, -432.0),
        (690.0, -432.0),
        (-690.0, 432.0),
        (690.0, 432.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_target(format!("evap_carryover_archive_deck_datum_{i}")).translate(
                *x,
                *y,
                BASE_Z + 2.0,
            );
    }
    datums
}

fn cold_block_archive_nests() -> Part {
    let block = centered_cube(
        "evap_carryover_archive_chilled_aluminum_block",
        COLD_BLOCK_X,
        COLD_BLOCK_Y,
        COLD_BLOCK_Z,
    );
    let receiver_shelf = centered_cube(
        "evap_carryover_archive_receiver_rack_shallow_shelf",
        RECEIVER_RACK_X + 16.0,
        RECEIVER_RACK_Y + 16.0,
        8.0,
    )
    .translate(0.0, 0.0, COLD_BLOCK_Z / 2.0 - 4.0);

    block - cold_receiver_well_cutouts() - coolant_bores() - thermistor_bores() - receiver_shelf
        + cold_block_perimeter_curb()
        + cold_block_datum_posts()
        + cold_block_channel_bosses()
        + archive_grid_label_cards()
}

fn cold_receiver_well_cutouts() -> Part {
    let mut wells = Part::empty("evap_carryover_archive_cold_block_receiver_well_cutouts");
    for index in 0..FRACTION_RECEIVERS {
        let (x, y) = fraction_position(index);
        wells = wells
            + centered_cylinder(
                format!("evap_carryover_archive_cold_receiver_nest_{index}"),
                COLD_NEST_D / 2.0,
                COLD_BLOCK_Z + 8.0,
                34,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn coolant_bores() -> Part {
    let mut bores = Part::empty("evap_carryover_archive_cold_block_coolant_bores");
    for (i, y) in [-102.0, 102.0].iter().enumerate() {
        bores = bores
            + centered_cylinder(
                format!("evap_carryover_archive_coolant_loop_bore_{i}"),
                COOLANT_BORE_D / 2.0,
                COLD_BLOCK_X + 24.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, *y, 2.0);
    }
    bores
}

fn thermistor_bores() -> Part {
    let mut bores = Part::empty("evap_carryover_archive_cold_block_thermistor_bores");
    for (i, (x, y)) in [(-150.0, 0.0), (150.0, 0.0), (0.0, -118.0), (0.0, 118.0)]
        .iter()
        .enumerate()
    {
        bores = bores
            + centered_cylinder(
                format!("evap_carryover_archive_thermistor_probe_bore_{i}"),
                THERMISTOR_BORE_D / 2.0,
                46.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, *y, 8.0);
    }
    bores
}

fn cold_block_perimeter_curb() -> Part {
    let front = centered_cube(
        "evap_carryover_archive_cold_block_front_receiver_curb",
        COLD_BLOCK_X - 36.0,
        8.0,
        16.0,
    )
    .translate(0.0, -COLD_BLOCK_Y / 2.0 + 20.0, COLD_BLOCK_Z / 2.0 + 8.0);
    let rear = centered_cube(
        "evap_carryover_archive_cold_block_rear_receiver_curb",
        COLD_BLOCK_X - 36.0,
        8.0,
        16.0,
    )
    .translate(0.0, COLD_BLOCK_Y / 2.0 - 20.0, COLD_BLOCK_Z / 2.0 + 8.0);
    let left = centered_cube(
        "evap_carryover_archive_cold_block_left_receiver_curb",
        8.0,
        COLD_BLOCK_Y - 36.0,
        16.0,
    )
    .translate(-COLD_BLOCK_X / 2.0 + 20.0, 0.0, COLD_BLOCK_Z / 2.0 + 8.0);
    let right = centered_cube(
        "evap_carryover_archive_cold_block_right_receiver_curb",
        8.0,
        COLD_BLOCK_Y - 36.0,
        16.0,
    )
    .translate(COLD_BLOCK_X / 2.0 - 20.0, 0.0, COLD_BLOCK_Z / 2.0 + 8.0);

    front + rear + left + right
}

fn cold_block_datum_posts() -> Part {
    let mut posts = Part::empty("evap_carryover_archive_cold_block_receiver_datums");
    for (i, (x, y)) in [
        (-COLD_BLOCK_X / 2.0 + 36.0, -COLD_BLOCK_Y / 2.0 + 34.0),
        (COLD_BLOCK_X / 2.0 - 36.0, -COLD_BLOCK_Y / 2.0 + 34.0),
        (-COLD_BLOCK_X / 2.0 + 36.0, COLD_BLOCK_Y / 2.0 - 34.0),
        (COLD_BLOCK_X / 2.0 - 36.0, COLD_BLOCK_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("evap_carryover_archive_cold_block_datum_post_{i}"),
                9.0,
                16.0,
                28,
            )
            .translate(*x, *y, COLD_BLOCK_Z / 2.0 + 8.0);
    }
    posts
}

fn cold_block_channel_bosses() -> Part {
    let inlet = centered_cylinder(
        "evap_carryover_archive_cold_block_coolant_inlet_boss",
        14.0,
        14.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-COLD_BLOCK_X / 2.0 - 5.0, -102.0, 2.0);
    let outlet = centered_cylinder(
        "evap_carryover_archive_cold_block_coolant_outlet_boss",
        14.0,
        14.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(COLD_BLOCK_X / 2.0 + 5.0, 102.0, 2.0);
    inlet + outlet
}

fn archive_grid_label_cards() -> Part {
    let chips = label_code_card(
        "evap_carryover_archive_cold_block_chip_axis_label",
        138.0,
        18.0,
        7,
        3,
    )
    .translate(-128.0, COLD_BLOCK_Y / 2.0 - 24.0, COLD_BLOCK_Z / 2.0 + 5.0);
    let fractions = label_code_card(
        "evap_carryover_archive_cold_block_fraction_time_axis_label",
        164.0,
        18.0,
        8,
        6,
    )
    .translate(120.0, COLD_BLOCK_Y / 2.0 - 24.0, COLD_BLOCK_Z / 2.0 + 5.0);
    chips + fractions
}

fn sealed_fraction_receivers() -> Part {
    let rack = centered_cube(
        "evap_carryover_archive_sealed_fraction_receiver_rack",
        RECEIVER_RACK_X,
        RECEIVER_RACK_Y,
        RECEIVER_RACK_Z,
    );
    let underside_key = centered_cube(
        "evap_carryover_archive_receiver_rack_cold_block_key",
        RECEIVER_RACK_X - 42.0,
        18.0,
        8.0,
    )
    .translate(
        0.0,
        -RECEIVER_RACK_Y / 2.0 + 30.0,
        -RECEIVER_RACK_Z / 2.0 - 4.0,
    );

    rack - receiver_socket_cutouts()
        + receiver_tube_bodies()
        + receiver_seal_rings()
        + receiver_cap_latch_bars()
        + receiver_run_axis_labels()
        + underside_key
}

fn receiver_socket_cutouts() -> Part {
    let mut cutouts = Part::empty("evap_carryover_archive_receiver_socket_cutouts");
    for index in 0..FRACTION_RECEIVERS {
        let (x, y) = fraction_position(index);
        cutouts = cutouts
            + centered_cylinder(
                format!("evap_carryover_archive_receiver_socket_{index}"),
                (RECEIVER_TUBE_D + 2.0) / 2.0,
                RECEIVER_RACK_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);
    }
    cutouts
}

fn receiver_tube_bodies() -> Part {
    let mut tubes = Part::empty("evap_carryover_archive_sealed_receiver_tube_bodies");
    for index in 0..FRACTION_RECEIVERS {
        let (x, y) = fraction_position(index);
        tubes = tubes
            + centered_cylinder(
                format!("evap_carryover_archive_sealed_receiver_body_{index}"),
                RECEIVER_TUBE_D / 2.0,
                RECEIVER_RACK_Z + 22.0,
                30,
            )
            .translate(x, y, 7.0);
    }
    tubes
}

fn receiver_seal_rings() -> Part {
    let mut rings = Part::empty("evap_carryover_archive_receiver_seal_rings");
    for index in 0..FRACTION_RECEIVERS {
        let (x, y) = fraction_position(index);
        let outer = centered_cylinder(
            format!("evap_carryover_archive_receiver_seal_ring_outer_{index}"),
            RECEIVER_SEAL_RING_D / 2.0,
            RECEIVER_CAP_Z,
            34,
        )
        .translate(x, y, RECEIVER_RACK_Z / 2.0 + RECEIVER_CAP_Z / 2.0);
        let inner = centered_cylinder(
            format!("evap_carryover_archive_receiver_septum_access_{index}"),
            RECEIVER_SEPTUM_D / 2.0,
            RECEIVER_CAP_Z + 2.0,
            18,
        )
        .translate(x, y, RECEIVER_RACK_Z / 2.0 + RECEIVER_CAP_Z / 2.0);
        rings = rings + (outer - inner);
    }
    rings
}

fn receiver_cap_latch_bars() -> Part {
    let mut bars = Part::empty("evap_carryover_archive_receiver_cap_latch_bars");
    for chip in 0..CHIP_COUNT {
        let y = centered_index(chip, CHIP_COUNT, FRACTION_PITCH_Y);
        bars = bars
            + centered_cube(
                format!("evap_carryover_archive_chip_{chip}_cap_latch_bar"),
                FRACTION_PITCH_X * (FRACTION_COLS as f64 - 1.0) + 58.0,
                5.0,
                10.0,
            )
            .translate(0.0, y + 16.0, RECEIVER_RACK_Z / 2.0 + 5.0);
    }
    bars
}

fn receiver_run_axis_labels() -> Part {
    let chip_axis = label_code_card(
        "evap_carryover_archive_receiver_chip_axis_barcode",
        128.0,
        18.0,
        6,
        8,
    )
    .translate(
        -112.0,
        -RECEIVER_RACK_Y / 2.0 + 22.0,
        RECEIVER_RACK_Z / 2.0 + 3.0,
    );
    let time_axis = label_code_card(
        "evap_carryover_archive_receiver_fraction_time_barcode",
        148.0,
        18.0,
        7,
        11,
    )
    .translate(
        112.0,
        -RECEIVER_RACK_Y / 2.0 + 22.0,
        RECEIVER_RACK_Z / 2.0 + 3.0,
    );
    chip_axis + time_axis
}

fn timed_fraction_token_rail() -> Part {
    let rail = centered_cube(
        "evap_carryover_archive_timed_fraction_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let back_stop = centered_cube(
        "evap_carryover_archive_timed_fraction_token_back_stop",
        TOKEN_RAIL_X,
        10.0,
        30.0,
    )
    .translate(0.0, TOKEN_RAIL_Y / 2.0 - 5.0, TOKEN_RAIL_Z / 2.0 + 15.0);

    rail - token_card_recesses()
        + back_stop
        + token_cards()
        + token_detent_buttons()
        + token_time_axis_label()
}

fn token_card_recesses() -> Part {
    let mut recesses = Part::empty("evap_carryover_archive_timed_fraction_token_recesses");
    for index in 0..TOKEN_SLOTS {
        let x = centered_index(index, TOKEN_SLOTS, TOKEN_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("evap_carryover_archive_fraction_time_token_recess_{index}"),
                TOKEN_CARD_X + 8.0,
                TOKEN_CARD_Y + 8.0,
                10.0,
            )
            .translate(x, 0.0, TOKEN_RAIL_Z / 2.0 - 5.0);
    }
    recesses
}

fn token_cards() -> Part {
    let mut cards = Part::empty("evap_carryover_archive_timed_fraction_token_cards");
    for index in 0..TOKEN_SLOTS {
        let x = centered_index(index, TOKEN_SLOTS, TOKEN_PITCH_X);
        cards = cards
            + label_code_card(
                format!("evap_carryover_archive_fraction_timepoint_token_{index}"),
                TOKEN_CARD_X,
                TOKEN_CARD_Y,
                4,
                index + 1,
            )
            .translate(x, 0.0, TOKEN_RAIL_Z / 2.0 + 2.0);
    }
    cards
}

fn token_detent_buttons() -> Part {
    let mut buttons = Part::empty("evap_carryover_archive_timed_token_detent_buttons");
    for index in 0..TOKEN_SLOTS {
        let x = centered_index(index, TOKEN_SLOTS, TOKEN_PITCH_X);
        buttons = buttons
            + centered_cylinder(
                format!("evap_carryover_archive_token_detent_button_{index}"),
                4.5,
                4.0,
                20,
            )
            .translate(x, -TOKEN_RAIL_Y / 2.0 + 12.0, TOKEN_RAIL_Z / 2.0 + 2.0);
    }
    buttons
}

fn token_time_axis_label() -> Part {
    label_code_card(
        "evap_carryover_archive_timed_fraction_sequence_label",
        172.0,
        18.0,
        8,
        14,
    )
    .translate(0.0, TOKEN_RAIL_Y / 2.0 + 14.0, TOKEN_RAIL_Z / 2.0 + 3.0)
}

fn high_low_carryover_standard_wells() -> Part {
    let block = centered_cube(
        "evap_carryover_archive_high_low_standard_block",
        STANDARD_BLOCK_X,
        STANDARD_BLOCK_Y,
        STANDARD_BLOCK_Z,
    );
    let split_fence = centered_cube(
        "evap_carryover_archive_high_low_standard_split_fence",
        STANDARD_BLOCK_X - 42.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, STANDARD_BLOCK_Z / 2.0 + 12.0);
    let rear_certificate_clip = centered_cube(
        "evap_carryover_archive_standard_certificate_clip_rail",
        STANDARD_BLOCK_X - 54.0,
        10.0,
        18.0,
    )
    .translate(
        0.0,
        STANDARD_BLOCK_Y / 2.0 - 9.0,
        STANDARD_BLOCK_Z / 2.0 + 9.0,
    );

    block - standard_well_cutouts()
        + split_fence
        + rear_certificate_clip
        + high_low_standard_lands()
        + standard_seal_lands()
}

fn standard_well_cutouts() -> Part {
    let mut wells = Part::empty("evap_carryover_archive_high_low_standard_well_cutouts");
    for index in 0..CARRYOVER_STANDARD_WELLS {
        let x = centered_index(index, CARRYOVER_STANDARD_WELLS, STANDARD_PITCH_X);
        let y = standard_y(index);
        let diameter = if standard_is_high(index) {
            HIGH_STANDARD_D
        } else {
            LOW_STANDARD_D
        };
        wells = wells
            + centered_cylinder(
                format!("evap_carryover_archive_standard_well_{index}"),
                diameter / 2.0,
                STANDARD_BLOCK_Z + 6.0,
                32,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn high_low_standard_lands() -> Part {
    let mut lands = Part::empty("evap_carryover_archive_high_low_standard_lands");
    for index in 0..CARRYOVER_STANDARD_WELLS {
        let x = centered_index(index, CARRYOVER_STANDARD_WELLS, STANDARD_PITCH_X);
        let y = if standard_is_high(index) { 58.0 } else { -58.0 };
        let label = if standard_is_high(index) {
            "high_carryover_standard"
        } else {
            "low_carryover_standard"
        };
        lands = lands
            + centered_cube(
                format!("evap_carryover_archive_{label}_identity_land_{index}"),
                22.0,
                10.0,
                5.0,
            )
            .translate(x, y, STANDARD_BLOCK_Z / 2.0 + 2.5);
    }
    lands
}

fn standard_seal_lands() -> Part {
    let mut lands = Part::empty("evap_carryover_archive_standard_seal_lands");
    for index in 0..STANDARD_SEAL_LANDS {
        lands = lands
            + label_code_card(
                format!("evap_carryover_archive_standard_seal_land_{index}"),
                70.0,
                18.0,
                5,
                index + 7,
            )
            .translate(
                centered_index(index, STANDARD_SEAL_LANDS, 92.0),
                STANDARD_BLOCK_Y / 2.0 + 16.0,
                STANDARD_BLOCK_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn standard_is_high(index: usize) -> bool {
    index % 2 == 0
}

fn standard_y(index: usize) -> f64 {
    if standard_is_high(index) {
        31.0
    } else {
        -31.0
    }
}

fn high_standard_count() -> usize {
    (0..CARRYOVER_STANDARD_WELLS)
        .filter(|index| standard_is_high(*index))
        .count()
}

fn low_standard_count() -> usize {
    CARRYOVER_STANDARD_WELLS - high_standard_count()
}

fn evaporation_mass_witness_pads() -> Part {
    let tray = centered_cube(
        "evap_carryover_archive_evaporation_mass_witness_tray",
        MASS_WITNESS_X,
        MASS_WITNESS_Y,
        MASS_WITNESS_Z,
    );
    let balance_reference_slot = centered_cube(
        "evap_carryover_archive_balance_reference_mass_slot",
        MASS_WITNESS_X - 72.0,
        20.0,
        10.0,
    )
    .translate(0.0, MASS_WITNESS_Y / 2.0 - 28.0, MASS_WITNESS_Z / 2.0 - 5.0);

    tray - mass_pad_recesses() - balance_reference_slot
        + mass_witness_pad_bodies()
        + mass_witness_pin_pairs()
        + evaporation_gradient_rails()
        + mass_witness_label_cards()
}

fn mass_pad_recesses() -> Part {
    let mut recesses = Part::empty("evap_carryover_archive_mass_pad_recesses");
    for index in 0..MASS_WITNESS_PADS {
        let (x, y) = mass_witness_position(index);
        recesses = recesses
            + centered_cube(
                format!("evap_carryover_archive_mass_witness_recess_{index}"),
                MASS_PAD_X + 10.0,
                MASS_PAD_Y + 10.0,
                10.0,
            )
            .translate(x, y, MASS_WITNESS_Z / 2.0 - 5.0);
    }
    recesses
}

fn mass_witness_pad_bodies() -> Part {
    let mut pads = Part::empty("evap_carryover_archive_mass_witness_pad_bodies");
    for index in 0..MASS_WITNESS_PADS {
        let (x, y) = mass_witness_position(index);
        let pad = centered_cube(
            format!("evap_carryover_archive_evaporation_mass_pad_{index}"),
            MASS_PAD_X,
            MASS_PAD_Y,
            MASS_PAD_Z,
        )
        .translate(x, y, MASS_WITNESS_Z / 2.0 + MASS_PAD_Z / 2.0);
        let wet_band = centered_cube(
            format!("evap_carryover_archive_mass_pad_{index}_wet_edge_band"),
            MASS_PAD_X - 10.0,
            6.0,
            MASS_PAD_Z + 3.0,
        )
        .translate(x, y - 8.0, MASS_WITNESS_Z / 2.0 + MASS_PAD_Z / 2.0 + 1.5);
        let dry_band = centered_cube(
            format!("evap_carryover_archive_mass_pad_{index}_dry_control_band"),
            MASS_PAD_X - 10.0,
            6.0,
            MASS_PAD_Z + 3.0,
        )
        .translate(x, y + 8.0, MASS_WITNESS_Z / 2.0 + MASS_PAD_Z / 2.0 + 1.5);
        pads = pads + pad + wet_band + dry_band;
    }
    pads
}

fn mass_witness_pin_pairs() -> Part {
    let mut pins = Part::empty("evap_carryover_archive_mass_witness_datum_pin_pairs");
    for index in 0..MASS_WITNESS_PADS {
        let (x, y) = mass_witness_position(index);
        for offset in [-22.0, 22.0] {
            pins = pins
                + centered_cylinder(
                    format!("evap_carryover_archive_mass_pad_{index}_balance_pin_{offset}"),
                    BALANCE_PIN_D / 2.0,
                    7.0,
                    16,
                )
                .translate(
                    x + offset,
                    y + MASS_PAD_Y / 2.0 + 6.0,
                    MASS_WITNESS_Z / 2.0 + 3.5,
                );
        }
    }
    pins
}

fn evaporation_gradient_rails() -> Part {
    let mut rails = Part::empty("evap_carryover_archive_evaporation_gradient_rails");
    for row in 0..MASS_WITNESS_PADS.div_ceil(MASS_WITNESS_COLS) {
        let y = centered_index(
            row,
            MASS_WITNESS_PADS.div_ceil(MASS_WITNESS_COLS),
            MASS_WITNESS_PITCH_Y,
        );
        rails = rails
            + centered_cube(
                format!("evap_carryover_archive_evaporation_gradient_row_{row}_rail"),
                MASS_WITNESS_X - 48.0,
                6.0,
                8.0,
            )
            .translate(
                0.0,
                y - MASS_WITNESS_PITCH_Y / 2.0 + 16.0,
                MASS_WITNESS_Z / 2.0 + 4.0,
            );
    }
    rails
}

fn mass_witness_label_cards() -> Part {
    let low = label_code_card(
        "evap_carryover_archive_low_evaporation_mass_label",
        126.0,
        18.0,
        6,
        4,
    )
    .translate(
        -114.0,
        -MASS_WITNESS_Y / 2.0 + 20.0,
        MASS_WITNESS_Z / 2.0 + 3.0,
    );
    let high = label_code_card(
        "evap_carryover_archive_high_evaporation_mass_label",
        126.0,
        18.0,
        6,
        10,
    )
    .translate(
        114.0,
        -MASS_WITNESS_Y / 2.0 + 20.0,
        MASS_WITNESS_Z / 2.0 + 3.0,
    );
    low + high
}

fn mass_witness_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        MASS_WITNESS_COLS,
        MASS_WITNESS_PADS,
        MASS_WITNESS_PITCH_X,
        MASS_WITNESS_PITCH_Y,
    )
}

fn cap_seal_integrity_coupons() -> Part {
    let block = centered_cube(
        "evap_carryover_archive_cap_seal_integrity_coupon_block",
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
        COUPON_BLOCK_Z,
    );
    let seal_pull_trough = centered_cube(
        "evap_carryover_archive_seal_pull_test_trough",
        COUPON_BLOCK_X - 60.0,
        18.0,
        10.0,
    )
    .translate(
        0.0,
        -COUPON_BLOCK_Y / 2.0 + 32.0,
        COUPON_BLOCK_Z / 2.0 - 5.0,
    );

    block - coupon_recesses() - seal_pull_trough
        + cap_torque_coupons()
        + seal_pull_tabs()
        + coupon_load_direction_arrows()
        + coupon_label_cards()
}

fn coupon_recesses() -> Part {
    let mut recesses = Part::empty("evap_carryover_archive_cap_seal_coupon_recesses");
    for index in 0..CAP_SEAL_COUPONS {
        let (x, y) = coupon_position(index);
        recesses = recesses
            + centered_cube(
                format!("evap_carryover_archive_cap_seal_coupon_recess_{index}"),
                62.0,
                44.0,
                9.0,
            )
            .translate(x, y, COUPON_BLOCK_Z / 2.0 - 4.5);
    }
    recesses
}

fn cap_torque_coupons() -> Part {
    let mut caps = Part::empty("evap_carryover_archive_cap_torque_coupons");
    for index in 0..CAP_SEAL_COUPONS {
        let (x, y) = coupon_position(index);
        let cap = centered_cylinder(
            format!("evap_carryover_archive_cap_torque_disc_{index}"),
            CAP_TORQUE_DISC_D / 2.0,
            9.0,
            34,
        )
        .translate(x - 15.0, y, COUPON_BLOCK_Z / 2.0 + 4.5);
        let witness_slot = centered_cube(
            format!("evap_carryover_archive_cap_torque_witness_mark_{index}"),
            4.0,
            CAP_TORQUE_DISC_D,
            11.0,
        )
        .translate(x - 15.0, y, COUPON_BLOCK_Z / 2.0 + 5.5);
        caps = caps + cap + witness_slot;
    }
    caps
}

fn seal_pull_tabs() -> Part {
    let mut tabs = Part::empty("evap_carryover_archive_seal_pull_tabs");
    for index in 0..CAP_SEAL_COUPONS {
        let (x, y) = coupon_position(index);
        tabs = tabs
            + centered_cube(
                format!("evap_carryover_archive_seal_integrity_pull_tab_{index}"),
                SEAL_PULL_TAB_X,
                SEAL_PULL_TAB_Y,
                7.0,
            )
            .translate(x + 18.0, y, COUPON_BLOCK_Z / 2.0 + 3.5);
    }
    tabs
}

fn coupon_load_direction_arrows() -> Part {
    let mut arrows = Part::empty("evap_carryover_archive_coupon_load_direction_arrows");
    for row in 0..CAP_SEAL_COUPONS.div_ceil(COUPON_COLS) {
        let y = centered_index(row, CAP_SEAL_COUPONS.div_ceil(COUPON_COLS), COUPON_PITCH_Y);
        arrows = arrows
            + centered_cube(
                format!("evap_carryover_archive_coupon_row_{row}_pull_direction_arrow"),
                COUPON_BLOCK_X - 58.0,
                4.0,
                6.0,
            )
            .translate(0.0, y - 24.0, COUPON_BLOCK_Z / 2.0 + 3.0);
    }
    arrows
}

fn coupon_label_cards() -> Part {
    let cap = label_code_card(
        "evap_carryover_archive_cap_torque_coupon_label",
        118.0,
        18.0,
        6,
        5,
    )
    .translate(
        -98.0,
        COUPON_BLOCK_Y / 2.0 - 20.0,
        COUPON_BLOCK_Z / 2.0 + 3.0,
    );
    let seal = label_code_card(
        "evap_carryover_archive_seal_pull_coupon_label",
        118.0,
        18.0,
        6,
        12,
    )
    .translate(
        98.0,
        COUPON_BLOCK_Y / 2.0 - 20.0,
        COUPON_BLOCK_Z / 2.0 + 3.0,
    );
    cap + seal
}

fn coupon_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        COUPON_COLS,
        CAP_SEAL_COUPONS,
        COUPON_PITCH_X,
        COUPON_PITCH_Y,
    )
}

fn barcode_rfid_custody_lands() -> Part {
    let deck = centered_cube(
        "evap_carryover_archive_barcode_rfid_custody_deck",
        CUSTODY_BLOCK_X,
        CUSTODY_BLOCK_Y,
        CUSTODY_BLOCK_Z,
    );
    let dry_lip = centered_cube(
        "evap_carryover_archive_custody_dry_lip",
        CUSTODY_BLOCK_X,
        8.0,
        22.0,
    )
    .translate(
        0.0,
        CUSTODY_BLOCK_Y / 2.0 - 4.0,
        CUSTODY_BLOCK_Z / 2.0 + 11.0,
    );

    deck + dry_lip + barcode_lands() + rfid_lands() + run_record_cards() + custody_status_tokens()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("evap_carryover_archive_barcode_custody_lands");
    for index in 0..BARCODE_LANDS {
        let col = index % 5;
        let row = index / 5;
        let x = centered_index(col, 5, 74.0);
        let y = 46.0 - row as f64 * 42.0;
        lands = lands
            + label_code_card(
                format!("evap_carryover_archive_barcode_custody_land_{index}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                7,
                index + 2,
            )
            .translate(x, y, CUSTODY_BLOCK_Z / 2.0 + 3.0);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("evap_carryover_archive_rfid_custody_lands");
    for index in 0..RFID_LANDS {
        let x = centered_index(index, RFID_LANDS, 54.0);
        let antenna = centered_cube(
            format!("evap_carryover_archive_rfid_antenna_land_{index}"),
            RFID_LAND_X,
            RFID_LAND_Y,
            4.0,
        )
        .translate(x, -38.0, CUSTODY_BLOCK_Z / 2.0 + 2.0);
        let keep_clear = centered_cube(
            format!("evap_carryover_archive_rfid_keep_clear_window_{index}"),
            RFID_LAND_X - 12.0,
            RFID_LAND_Y - 12.0,
            5.0,
        )
        .translate(x, -38.0, CUSTODY_BLOCK_Z / 2.0 + 2.5);
        lands = lands + (antenna - keep_clear);
    }
    lands
}

fn run_record_cards() -> Part {
    let mut cards = Part::empty("evap_carryover_archive_run_record_card_lands");
    for index in 0..RUN_RECORD_CARD_LANDS {
        cards = cards
            + label_code_card(
                format!("evap_carryover_archive_run_record_card_land_{index}"),
                76.0,
                28.0,
                5,
                index + 9,
            )
            .translate(
                centered_index(index, RUN_RECORD_CARD_LANDS, 88.0),
                -CUSTODY_BLOCK_Y / 2.0 + 24.0,
                CUSTODY_BLOCK_Z / 2.0 + 3.0,
            );
    }
    cards
}

fn custody_status_tokens() -> Part {
    let retained = centered_cube(
        "evap_carryover_archive_custody_retained_status_token",
        86.0,
        16.0,
        6.0,
    )
    .translate(
        -110.0,
        CUSTODY_BLOCK_Y / 2.0 + 12.0,
        CUSTODY_BLOCK_Z / 2.0 + 3.0,
    );
    let evidence = centered_cube(
        "evap_carryover_archive_custody_evidence_status_token",
        86.0,
        16.0,
        6.0,
    )
    .translate(
        0.0,
        CUSTODY_BLOCK_Y / 2.0 + 12.0,
        CUSTODY_BLOCK_Z / 2.0 + 3.0,
    );
    let released = centered_cube(
        "evap_carryover_archive_custody_released_status_token",
        86.0,
        16.0,
        6.0,
    )
    .translate(
        110.0,
        CUSTODY_BLOCK_Y / 2.0 + 12.0,
        CUSTODY_BLOCK_Z / 2.0 + 3.0,
    );
    retained + evidence + released
}

fn waste_flush_routing_manifold() -> Part {
    let block = centered_cube(
        "evap_carryover_archive_waste_flush_routing_manifold_body",
        ROUTING_X,
        ROUTING_Y,
        ROUTING_Z,
    );
    let splash_lip = centered_cube(
        "evap_carryover_archive_waste_flush_splash_lip",
        ROUTING_X - 54.0,
        8.0,
        22.0,
    )
    .translate(0.0, -ROUTING_Y / 2.0 + 8.0, ROUTING_Z / 2.0 + 11.0);

    block - routing_bore_cutouts() - waste_drop_cutouts()
        + splash_lip
        + routing_hose_bosses()
        + blank_branch_caps()
        + routing_identity_labels()
}

fn routing_bore_cutouts() -> Part {
    let mut bores = Part::empty("evap_carryover_archive_waste_flush_bore_cutouts");
    for index in 0..FLUSH_PORTS {
        let x = centered_index(index, FLUSH_PORTS, ROUTING_PORT_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{index}_flush_supply_bore"),
                FLUSH_BORE_D / 2.0,
                ROUTING_Y + 18.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 14.0, 0.0, 2.0)
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{index}_waste_return_bore"),
                WASTE_BORE_D / 2.0,
                ROUTING_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 14.0, 0.0, -2.0);
    }
    bores
}

fn waste_drop_cutouts() -> Part {
    let mut drops = Part::empty("evap_carryover_archive_waste_drop_cutouts");
    for index in 0..WASTE_PORTS {
        let x = centered_index(index, WASTE_PORTS, ROUTING_PORT_PITCH_X);
        drops = drops
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{index}_waste_drop_well"),
                WASTE_DROP_D / 2.0,
                ROUTING_Z + 8.0,
                32,
            )
            .translate(x + 14.0, 20.0, 0.0);
    }
    drops
}

fn routing_hose_bosses() -> Part {
    let mut bosses = Part::empty("evap_carryover_archive_routing_hose_bosses");
    for index in 0..FLUSH_PORTS {
        let x = centered_index(index, FLUSH_PORTS, ROUTING_PORT_PITCH_X);
        bosses = bosses
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{index}_flush_hose_boss"),
                10.0,
                10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 14.0, ROUTING_Y / 2.0 + 3.0, 2.0)
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{index}_waste_hose_boss"),
                13.0,
                10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 14.0, -ROUTING_Y / 2.0 - 3.0, -2.0);
    }
    bosses
}

fn blank_branch_caps() -> Part {
    let mut caps = Part::empty("evap_carryover_archive_blank_branch_caps");
    for index in 0..BLANK_BRANCH_PORTS {
        caps = caps
            + centered_cylinder(
                format!("evap_carryover_archive_blank_branch_cap_{index}"),
                12.0,
                12.0,
                28,
            )
            .translate(
                centered_index(index, BLANK_BRANCH_PORTS, 42.0),
                -ROUTING_Y / 2.0 - 10.0,
                ROUTING_Z / 2.0 + 6.0,
            );
    }
    caps
}

fn routing_identity_labels() -> Part {
    let flush = label_code_card(
        "evap_carryover_archive_flush_route_identity_label",
        142.0,
        18.0,
        7,
        6,
    )
    .translate(-210.0, ROUTING_Y / 2.0 + 14.0, ROUTING_Z / 2.0 + 3.0);
    let waste = label_code_card(
        "evap_carryover_archive_waste_route_identity_label",
        142.0,
        18.0,
        7,
        13,
    )
    .translate(210.0, ROUTING_Y / 2.0 + 14.0, ROUTING_Z / 2.0 + 3.0);
    flush + waste
}

fn quarantine_release_gates() -> Part {
    let block = centered_cube(
        "evap_carryover_archive_quarantine_release_gate_block",
        GATE_BLOCK_X,
        GATE_BLOCK_Y,
        GATE_BLOCK_Z,
    );
    let rear_guard = centered_cube(
        "evap_carryover_archive_gate_rear_status_guard",
        GATE_BLOCK_X,
        8.0,
        28.0,
    )
    .translate(0.0, GATE_BLOCK_Y / 2.0 - 4.0, GATE_BLOCK_Z / 2.0 + 14.0);

    block - gate_slot_cutouts()
        + rear_guard
        + sliding_gate_flags()
        + gate_stop_posts()
        + gate_custody_labels()
}

fn gate_slot_cutouts() -> Part {
    let mut slots = Part::empty("evap_carryover_archive_quarantine_release_gate_slots");
    for index in 0..DISPOSITION_GATES {
        slots = slots
            + centered_cube(
                format!("evap_carryover_archive_disposition_gate_slot_{index}"),
                GATE_SLOT_X,
                GATE_SLOT_Y,
                10.0,
            )
            .translate(
                centered_index(index, DISPOSITION_GATES, GATE_PITCH_X),
                0.0,
                GATE_BLOCK_Z / 2.0 - 5.0,
            );
    }
    slots
}

fn sliding_gate_flags() -> Part {
    let mut flags = Part::empty("evap_carryover_archive_sliding_gate_flags");
    for (index, state) in ["release", "quarantine", "hold"].iter().enumerate() {
        flags = flags
            + centered_cube(
                format!("evap_carryover_archive_{state}_sliding_gate_flag"),
                68.0,
                28.0,
                8.0,
            )
            .translate(
                centered_index(index, DISPOSITION_GATES, GATE_PITCH_X),
                0.0,
                GATE_BLOCK_Z / 2.0 + 4.0,
            );
    }
    flags
}

fn gate_stop_posts() -> Part {
    let mut posts = Part::empty("evap_carryover_archive_gate_stop_posts");
    for index in 0..DISPOSITION_GATES {
        let x = centered_index(index, DISPOSITION_GATES, GATE_PITCH_X);
        posts = posts
            + centered_cylinder(
                format!("evap_carryover_archive_gate_{index}_left_stop_post"),
                5.0,
                12.0,
                20,
            )
            .translate(x - GATE_SLOT_X / 2.0 - 8.0, 0.0, GATE_BLOCK_Z / 2.0 + 6.0)
            + centered_cylinder(
                format!("evap_carryover_archive_gate_{index}_right_stop_post"),
                5.0,
                12.0,
                20,
            )
            .translate(x + GATE_SLOT_X / 2.0 + 8.0, 0.0, GATE_BLOCK_Z / 2.0 + 6.0);
    }
    posts
}

fn gate_custody_labels() -> Part {
    let release = label_code_card(
        "evap_carryover_archive_release_gate_label",
        86.0,
        18.0,
        5,
        2,
    )
    .translate(
        centered_index(0, DISPOSITION_GATES, GATE_PITCH_X),
        -GATE_BLOCK_Y / 2.0 + 12.0,
        GATE_BLOCK_Z / 2.0 + 3.0,
    );
    let quarantine = label_code_card(
        "evap_carryover_archive_quarantine_gate_label",
        86.0,
        18.0,
        5,
        9,
    )
    .translate(
        centered_index(1, DISPOSITION_GATES, GATE_PITCH_X),
        -GATE_BLOCK_Y / 2.0 + 12.0,
        GATE_BLOCK_Z / 2.0 + 3.0,
    );
    let hold = label_code_card("evap_carryover_archive_hold_gate_label", 86.0, 18.0, 5, 15)
        .translate(
            centered_index(2, DISPOSITION_GATES, GATE_PITCH_X),
            -GATE_BLOCK_Y / 2.0 + 12.0,
            GATE_BLOCK_Z / 2.0 + 3.0,
        );
    release + quarantine + hold
}

fn camera_evidence_robot_datums_bridge() -> Part {
    let beam = centered_cube(
        "evap_carryover_archive_camera_evidence_bridge_beam",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_Z / 2.0 - BRIDGE_BEAM_Z / 2.0);
    let left_leg = centered_cube(
        "evap_carryover_archive_camera_bridge_left_leg",
        28.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0 + 28.0, 0.0, 0.0);
    let right_leg = centered_cube(
        "evap_carryover_archive_camera_bridge_right_leg",
        28.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 28.0, 0.0, 0.0);

    beam + left_leg
        + right_leg
        + camera_mount_lands()
        + evidence_light_bars()
        + evidence_timestamp_cards()
        + robot_service_datum_towers()
}

fn camera_mount_lands() -> Part {
    let mut mounts = Part::empty("evap_carryover_archive_camera_mount_lands");
    for index in 0..CAMERA_COUNT {
        let x = centered_index(index, CAMERA_COUNT, 265.0);
        let mount = centered_cube(
            format!("evap_carryover_archive_evidence_camera_mount_land_{index}"),
            64.0,
            48.0,
            12.0,
        )
        .translate(x, -BRIDGE_Y / 2.0 - 20.0, BRIDGE_Z / 2.0 - 44.0);
        let lens_bore = centered_cylinder(
            format!("evap_carryover_archive_evidence_camera_lens_clearance_{index}"),
            14.0,
            16.0,
            32,
        )
        .translate(x, -BRIDGE_Y / 2.0 - 20.0, BRIDGE_Z / 2.0 - 44.0);
        mounts = mounts + (mount - lens_bore);
    }
    mounts
}

fn evidence_light_bars() -> Part {
    let mut lights = Part::empty("evap_carryover_archive_evidence_light_bars");
    for index in 0..LIGHT_BAR_COUNT {
        let x = centered_index(index, LIGHT_BAR_COUNT, 280.0);
        lights = lights
            + centered_cube(
                format!("evap_carryover_archive_evidence_light_bar_{index}"),
                190.0,
                10.0,
                12.0,
            )
            .translate(x, BRIDGE_Y / 2.0 + 10.0, BRIDGE_Z / 2.0 - 40.0);
    }
    lights
}

fn evidence_timestamp_cards() -> Part {
    let run_card = label_code_card(
        "evap_carryover_archive_evidence_run_timestamp_card",
        170.0,
        24.0,
        8,
        17,
    )
    .translate(-210.0, 0.0, BRIDGE_Z / 2.0 - 16.0);
    let custody_card = label_code_card(
        "evap_carryover_archive_evidence_custody_snapshot_card",
        170.0,
        24.0,
        8,
        21,
    )
    .translate(210.0, 0.0, BRIDGE_Z / 2.0 - 16.0);
    run_card + custody_card
}

fn robot_service_datum_towers() -> Part {
    let mut towers = Part::empty("evap_carryover_archive_robotic_service_datum_towers");
    for index in 0..ROBOT_SERVICE_DATUMS {
        let x = centered_index(index, ROBOT_SERVICE_DATUMS, 205.0);
        let tower = centered_cylinder(
            format!("evap_carryover_archive_robot_service_datum_tower_{index}"),
            10.0,
            40.0,
            28,
        )
        .translate(x, BRIDGE_Y / 2.0 + 34.0, BRIDGE_Z / 2.0 - 20.0);
        let target = fiducial_target(format!(
            "evap_carryover_archive_robot_service_datum_target_{index}"
        ))
        .translate(x, BRIDGE_Y / 2.0 + 34.0, BRIDGE_Z / 2.0 + 3.0);
        towers = towers + tower + target;
    }
    towers
}

fn closed_fraction_route_placeholders() -> Part {
    let mut routes = Part::empty("evap_carryover_archive_closed_fraction_route_placeholders");
    for chip in 0..CHIP_COUNT {
        let y = centered_index(chip, CHIP_COUNT, 28.0) + 22.0;
        routes = routes
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{chip}_fraction_route_to_cold_archive"),
                2.4,
                470.0,
                12,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-210.0, y + 70.0, BASE_Z + 18.0)
            + centered_cylinder(
                format!("evap_carryover_archive_chip_{chip}_flush_route_to_manifold"),
                2.0,
                365.0,
                12,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(45.0, y - 60.0, BASE_Z + 16.0);
    }

    for branch in 0..BLANK_BRANCH_PORTS {
        routes = routes
            + centered_cylinder(
                format!("evap_carryover_archive_blank_branch_{branch}_route_placeholder"),
                2.2,
                300.0,
                12,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(branch, BLANK_BRANCH_PORTS, 66.0) + 220.0,
                38.0,
                BASE_Z + 20.0,
            );
    }
    routes
}

fn fraction_position(index: usize) -> (f64, f64) {
    grid_position(
        index,
        FRACTION_COLS,
        FRACTION_RECEIVERS,
        FRACTION_PITCH_X,
        FRACTION_PITCH_Y,
    )
}

fn label_code_card(
    name: impl Into<String>,
    width: f64,
    depth: f64,
    bars: usize,
    seed: usize,
) -> Part {
    let name = name.into();
    let plate = centered_cube(format!("{name}_plate"), width, depth, 3.0);
    let mut code = Part::empty(format!("{name}_barcode_geometry"));
    for index in 0..bars {
        let x = centered_index(index, bars, width / (bars as f64 + 1.0));
        let bar_width = if (index + seed) % 3 == 0 { 3.8 } else { 1.8 };
        code = code
            + centered_cube(
                format!("{name}_barcode_bar_{index}"),
                bar_width,
                depth - 6.0,
                4.0,
            )
            .translate(x, 0.0, 2.0);
    }
    plate + code
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let disk = centered_cylinder(format!("{name}_outer_disk"), 12.0, 4.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot_relief"), 3.0, 5.0, 20);
    let x_bar = centered_cube(format!("{name}_x_crosshair"), 28.0, 3.0, 5.0);
    let y_bar = centered_cube(format!("{name}_y_crosshair"), 3.0, 28.0, 5.0);
    disk - center + x_bar + y_bar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_scoped_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert_eq!(
            OUTPUTS.last().copied(),
            Some("output/closed_perfusion_sample_fraction_evaporation_carryover_archive_station_assembly.stl")
        );
        assert_eq!(REQUIRED_FEATURES.len(), 11);
    }

    #[test]
    fn multi_chip_fraction_archive_counts_match_grid() {
        assert_eq!(CHIP_COUNT, 6);
        assert_eq!(FRACTION_TIMEPOINTS, 8);
        assert_eq!(FRACTION_RECEIVERS, 48);
        assert_eq!(FRACTION_RECEIVERS, FRACTION_COLS * FRACTION_ROWS);
        assert_eq!(TOKEN_SLOTS, FRACTION_TIMEPOINTS);
        assert!(COLD_BLOCK_X > FRACTION_PITCH_X * (FRACTION_COLS as f64 - 1.0) + COLD_NEST_D);
        assert!(COLD_BLOCK_Y > FRACTION_PITCH_Y * (FRACTION_ROWS as f64 - 1.0) + COLD_NEST_D);
    }

    #[test]
    fn carryover_and_evaporation_witness_counts_are_balanced() {
        assert_eq!(high_standard_count(), 8);
        assert_eq!(low_standard_count(), 8);
        assert_eq!(MASS_WITNESS_PADS, 12);
        assert_eq!(CAP_SEAL_COUPONS, 12);
        assert_eq!(FLUSH_PORTS, CHIP_COUNT);
        assert_eq!(WASTE_PORTS, CHIP_COUNT);
        assert_eq!(TUBE_ROUTE_COUNT, 14);
    }

    #[test]
    fn custody_evidence_and_robot_service_counts_are_explicit() {
        assert_eq!(BARCODE_LANDS, 10);
        assert_eq!(RFID_LANDS, 6);
        assert_eq!(RUN_RECORD_CARD_LANDS, 4);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(ROBOT_SERVICE_DATUMS, 6);
        assert_eq!(COLD_DATUMS, 4);
    }

    #[test]
    fn station_modules_fit_with_service_gaps() {
        let rects = deck_module_footprints();
        for rect in rects {
            assert!(
                rect.fits_inside_station(MODULE_MARGIN_MM),
                "{} exceeds station envelope",
                rect.name
            );
        }

        for a in 0..rects.len() {
            for b in (a + 1)..rects.len() {
                assert!(
                    !rects[a].overlaps(rects[b], MODULE_GAP_MM),
                    "{} overlaps {}",
                    rects[a].name,
                    rects[b].name
                );
            }
        }
    }
}
