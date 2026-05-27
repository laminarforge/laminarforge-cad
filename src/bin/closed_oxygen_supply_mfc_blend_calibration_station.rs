use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed oxygen supply MFC blend calibration station.
//
// Intent:
// - Provide a benchtop validation fixture for checking purchased O2/N2/air/CO2
//   mass-flow-controller blend behavior before gas enters a closed incubator or
//   culture-module cabinet.
// - Keep cylinder/regulator interfaces, MFC cartridges, surrogate manifold,
//   calibration standards, dewpoint/O2/CO2 probe ports, relief routing,
//   leak-test coupons, keyed tubing, labels, and secondary containment visible
//   as auditable mechanical features.
// - Model pressure-rated regulators, MFCs, probes, bottles, and relief hardware
//   as purchased equipment envelopes only. Printed geometry is fixture,
//   containment, routing, labeling, and service-interface CAD.

const OUTPUT_PREFIX: &str = "output/closed_oxygen_supply_mfc_blend_calibration_station_";
const BASE_OUTPUT: &str = "output/closed_oxygen_supply_mfc_blend_calibration_station_base.stl";
const CORE_OUTPUT: &str = "output/closed_oxygen_supply_mfc_blend_calibration_station_core.stl";
const ASSEMBLY_OUTPUT: &str =
    "output/closed_oxygen_supply_mfc_blend_calibration_station_assembly.stl";

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["o2", "n2", "air", "co2"];
const REGULATOR_INLET_BULKHEADS: usize = GAS_CHANNELS;
const MFC_CARTRIDGE_NESTS: usize = GAS_CHANNELS;
const CALIBRATION_BOTTLE_POCKETS: usize = 4;
const PROBE_PORT_TYPES: usize = 3;
const PROBE_PORTS: usize = PROBE_PORT_TYPES * 2;
const LEAK_COUPON_WELLS: usize = 8;
const KEYED_ROUTE_CHANNELS: usize = GAS_CHANNELS + 2;
const STATUS_LANES: usize = 3;
const BARCODE_LANDS: usize = GAS_CHANNELS * 3;
const RELIEF_VENT_PATHS: usize = 2;
const GUARD_RAIL_SEGMENTS: usize = 6;

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;

const BULKHEAD_X: f64 = 1010.0;
const BULKHEAD_Y: f64 = 86.0;
const BULKHEAD_Z: f64 = 138.0;
const BULKHEAD_POS: (f64, f64) = (0.0, 272.0);
const BULKHEAD_PITCH_X: f64 = 208.0;
const INLET_PORT_D: f64 = 18.0;
const REGULATOR_ENVELOPE_X: f64 = 126.0;
const REGULATOR_ENVELOPE_Y: f64 = 50.0;
const REGULATOR_ENVELOPE_Z: f64 = 78.0;

const MFC_BANK_X: f64 = 640.0;
const MFC_BANK_Y: f64 = 184.0;
const MFC_BANK_Z: f64 = 58.0;
const MFC_BANK_POS: (f64, f64) = (-268.0, 82.0);
const MFC_PITCH_X: f64 = 142.0;
const MFC_ENVELOPE_X: f64 = 100.0;
const MFC_ENVELOPE_Y: f64 = 116.0;
const MFC_ENVELOPE_Z: f64 = 40.0;
const TUBE_CLEARANCE_D: f64 = 6.4;

const MANIFOLD_X: f64 = 420.0;
const MANIFOLD_Y: f64 = 184.0;
const MANIFOLD_Z: f64 = 68.0;
const MANIFOLD_POS: (f64, f64) = (330.0, 82.0);
const MIXING_TUBE_D: f64 = 26.0;
const MANIFOLD_PORT_PITCH_X: f64 = 70.0;

const CAL_BOTTLE_X: f64 = 432.0;
const CAL_BOTTLE_Y: f64 = 168.0;
const CAL_BOTTLE_Z: f64 = 50.0;
const CAL_BOTTLE_POS: (f64, f64) = (-382.0, -138.0);
const CAL_BOTTLE_PITCH_X: f64 = 94.0;
const CAL_BOTTLE_D: f64 = 58.0;
const CAL_BOTTLE_DEPTH: f64 = 40.0;

const PROBE_BLOCK_X: f64 = 330.0;
const PROBE_BLOCK_Y: f64 = 168.0;
const PROBE_BLOCK_Z: f64 = 72.0;
const PROBE_BLOCK_POS: (f64, f64) = (68.0, -138.0);
const PROBE_PORT_D: f64 = 20.0;
const PROBE_PITCH_X: f64 = 86.0;
const PROBE_PITCH_Y: f64 = 54.0;

const RELIEF_BLOCK_X: f64 = 300.0;
const RELIEF_BLOCK_Y: f64 = 168.0;
const RELIEF_BLOCK_Z: f64 = 66.0;
const RELIEF_BLOCK_POS: (f64, f64) = (452.0, -138.0);
const RELIEF_BORE_D: f64 = 16.0;

const LEAK_COUPON_X: f64 = 430.0;
const LEAK_COUPON_Y: f64 = 116.0;
const LEAK_COUPON_Z: f64 = 38.0;
const LEAK_COUPON_POS: (f64, f64) = (-380.0, -292.0);
const COUPON_WELL_X: f64 = 38.0;
const COUPON_WELL_Y: f64 = 54.0;
const COUPON_PITCH_X: f64 = 46.0;

const ROUTE_CHANNEL_X: f64 = 492.0;
const ROUTE_CHANNEL_Y: f64 = 116.0;
const ROUTE_CHANNEL_Z: f64 = 34.0;
const ROUTE_CHANNEL_POS: (f64, f64) = (92.0, -292.0);
const ROUTE_PITCH_Y: f64 = 15.0;

const STATUS_PANEL_X: f64 = 260.0;
const STATUS_PANEL_Y: f64 = 116.0;
const STATUS_PANEL_Z: f64 = 22.0;
const STATUS_PANEL_POS: (f64, f64) = (468.0, -292.0);
const STATUS_SLOT_X: f64 = 70.0;
const STATUS_SLOT_Y: f64 = 24.0;

const FOOT_CLEARANCE_X: f64 = 62.0;
const FOOT_CLEARANCE_Y: f64 = 58.0;

#[derive(Clone, Copy, Debug)]
struct ModuleFootprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl ModuleFootprint {
    fn fits_on_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: ModuleFootprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base();
    export(BASE_OUTPUT, &base);

    let core = core();
    export(CORE_OUTPUT, &core);

    let assembly = base + core;
    export(ASSEMBLY_OUTPUT, &assembly);

    println!();
    println!("Closed oxygen supply MFC blend calibration station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm benchtop secondary-containment tray");
    println!(
        "  Gas interfaces:        {:?} with {REGULATOR_INLET_BULKHEADS} regulator inlet bulkheads and {MFC_CARTRIDGE_NESTS} removable MFC cartridge nests",
        GAS_NAMES
    );
    println!(
        "  Blend verification:    surrogate mixing manifold, {CALIBRATION_BOTTLE_POCKETS} calibration gas bottle pockets, and {PROBE_PORTS} dewpoint/O2/CO2 probe ports"
    );
    println!(
        "  Fault evidence:        {RELIEF_VENT_PATHS} pressure-relief vent paths, {LEAK_COUPON_WELLS} leak-test coupon wells, and keyed tubing route channels"
    );
    println!(
        "  Traceability:          {BARCODE_LANDS} barcode lands plus release/hold/reject status panel lanes"
    );
    println!("  Exports:               {BASE_OUTPUT}, {CORE_OUTPUT}, {ASSEMBLY_OUTPUT}");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn module_footprints() -> [ModuleFootprint; 9] {
    [
        footprint(
            "cylinder_regulator_bulkhead_bank",
            BULKHEAD_POS,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        footprint(
            "mfc_cartridge_nest_bank",
            MFC_BANK_POS,
            MFC_BANK_X,
            MFC_BANK_Y,
        ),
        footprint(
            "mixing_manifold_surrogate",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        footprint(
            "calibration_gas_bottle_pockets",
            CAL_BOTTLE_POS,
            CAL_BOTTLE_X,
            CAL_BOTTLE_Y,
        ),
        footprint(
            "dewpoint_o2_co2_probe_ports",
            PROBE_BLOCK_POS,
            PROBE_BLOCK_X,
            PROBE_BLOCK_Y,
        ),
        footprint(
            "pressure_relief_vent_path",
            RELIEF_BLOCK_POS,
            RELIEF_BLOCK_X,
            RELIEF_BLOCK_Y,
        ),
        footprint(
            "leak_test_coupon_wells",
            LEAK_COUPON_POS,
            LEAK_COUPON_X,
            LEAK_COUPON_Y,
        ),
        footprint(
            "keyed_tubing_route_channels",
            ROUTE_CHANNEL_POS,
            ROUTE_CHANNEL_X,
            ROUTE_CHANNEL_Y,
        ),
        footprint(
            "barcode_status_panel",
            STATUS_PANEL_POS,
            STATUS_PANEL_X,
            STATUS_PANEL_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> ModuleFootprint {
    ModuleFootprint { name, center, x, y }
}

fn assert_design_constraints() {
    assert!(BASE_OUTPUT.starts_with(OUTPUT_PREFIX));
    assert!(CORE_OUTPUT.starts_with(OUTPUT_PREFIX));
    assert!(ASSEMBLY_OUTPUT.starts_with(OUTPUT_PREFIX));
    assert_eq!(GAS_NAMES.len(), GAS_CHANNELS);
    assert_eq!(REGULATOR_INLET_BULKHEADS, GAS_CHANNELS);
    assert_eq!(MFC_CARTRIDGE_NESTS, GAS_CHANNELS);
    assert_eq!(KEYED_ROUTE_CHANNELS, GAS_CHANNELS + 2);
    assert_eq!(PROBE_PORTS, PROBE_PORT_TYPES * 2);
    assert_eq!(STATUS_LANES, 3);
    assert_eq!(RELIEF_VENT_PATHS, 2);
    assert_eq!(GUARD_RAIL_SEGMENTS, 6);
    assert!(TUBE_CLEARANCE_D > 6.0);
    assert!(RELIEF_BORE_D > INLET_PORT_D * 0.75);
    assert!(CAL_BOTTLE_DEPTH < CAL_BOTTLE_Z);
    assert!(FOOT_CLEARANCE_X >= 50.0);
    assert!(FOOT_CLEARANCE_Y >= 50.0);

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_on_deck(),
            "{} exceeds deck envelope",
            module.name
        );
    }
    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base() -> Part {
    let deck = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_sump = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_recessed_spill_sump",
        STATION_X - 2.0 * (RIM_W + 58.0),
        STATION_Y - 2.0 * (RIM_W + 52.0),
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z - 4.0);
    let front_drain = centered_cylinder(
        "closed_oxygen_mfc_blend_cal_station_front_sump_drain",
        7.0,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 88.0,
        -STATION_Y / 2.0 + 34.0,
        BASE_Z - 9.0,
    );

    deck - recessed_sump - front_drain - base_module_sockets() - base_mounting_holes()
        + perimeter_rims()
        + guard_rails()
        + datum_targets()
        + gas_family_zone_labels()
}

fn base_module_sockets() -> Part {
    let mut sockets = Part::empty("closed_oxygen_mfc_blend_cal_station_module_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_{}_socket", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn base_mounting_holes() -> Part {
    let mut holes = Part::empty("closed_oxygen_mfc_blend_cal_station_mounting_holes");
    for (i, (x, y)) in [
        (
            -STATION_X / 2.0 + FOOT_CLEARANCE_X,
            -STATION_Y / 2.0 + FOOT_CLEARANCE_Y,
        ),
        (
            STATION_X / 2.0 - FOOT_CLEARANCE_X,
            -STATION_Y / 2.0 + FOOT_CLEARANCE_Y,
        ),
        (
            -STATION_X / 2.0 + FOOT_CLEARANCE_X,
            STATION_Y / 2.0 - FOOT_CLEARANCE_Y,
        ),
        (
            STATION_X / 2.0 - FOOT_CLEARANCE_X,
            STATION_Y / 2.0 - FOOT_CLEARANCE_Y,
        ),
        (0.0, -STATION_Y / 2.0 + FOOT_CLEARANCE_Y),
        (0.0, STATION_Y / 2.0 - FOOT_CLEARANCE_Y),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_m6_clearance_{i}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_rear_bulkhead_guard_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_left_spill_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_right_spill_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn guard_rails() -> Part {
    let mut rails = Part::empty("closed_oxygen_mfc_blend_cal_station_guard_rails");
    for (i, (x, y, rail_x, rail_y)) in [
        (
            MFC_BANK_POS.0,
            MFC_BANK_POS.1 + MFC_BANK_Y / 2.0 + 20.0,
            MFC_BANK_X + 44.0,
            12.0,
        ),
        (
            MFC_BANK_POS.0,
            MFC_BANK_POS.1 - MFC_BANK_Y / 2.0 - 20.0,
            MFC_BANK_X + 44.0,
            12.0,
        ),
        (
            MANIFOLD_POS.0 + MANIFOLD_X / 2.0 + 20.0,
            MANIFOLD_POS.1,
            12.0,
            MANIFOLD_Y + 58.0,
        ),
        (
            CAL_BOTTLE_POS.0,
            CAL_BOTTLE_POS.1 - CAL_BOTTLE_Y / 2.0 - 20.0,
            CAL_BOTTLE_X + 36.0,
            12.0,
        ),
        (
            RELIEF_BLOCK_POS.0 + RELIEF_BLOCK_X / 2.0 + 20.0,
            RELIEF_BLOCK_POS.1,
            12.0,
            RELIEF_BLOCK_Y + 38.0,
        ),
        (
            STATUS_PANEL_POS.0,
            STATUS_PANEL_POS.1 - STATUS_PANEL_Y / 2.0 - 18.0,
            STATUS_PANEL_X + 34.0,
            12.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_guard_rail_{i}"),
                *rail_x,
                *rail_y,
                34.0,
            )
            .translate(*x, *y, BASE_Z + 17.0);
    }
    rails
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_oxygen_mfc_blend_cal_station_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 116.0, -STATION_Y / 2.0 + 112.0),
        (STATION_X / 2.0 - 116.0, -STATION_Y / 2.0 + 112.0),
        (-STATION_X / 2.0 + 116.0, STATION_Y / 2.0 - 112.0),
        (STATION_X / 2.0 - 116.0, STATION_Y / 2.0 - 112.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_datum_boss_{i}"),
                12.0,
                5.0,
                32,
            )
            .translate(*x, *y, BASE_Z + 2.5)
            - centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_datum_center_mark_{i}"),
                2.0,
                6.0,
                18,
            )
            .translate(*x, *y, BASE_Z + 2.8);
    }
    targets
}

fn gas_family_zone_labels() -> Part {
    let mut labels = Part::empty("closed_oxygen_mfc_blend_cal_station_gas_family_zone_labels");
    for (i, _) in GAS_NAMES.iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_gas_family_label_land_{i}"),
                72.0,
                16.0,
                3.0,
            )
            .translate(
                BULKHEAD_POS.0 + centered_index(i, GAS_CHANNELS, BULKHEAD_PITCH_X),
                BULKHEAD_POS.1 - 54.0,
                BASE_Z + 1.5,
            );
    }
    labels
}

fn core() -> Part {
    cylinder_regulator_inlet_bulkheads().translate(
        BULKHEAD_POS.0,
        BULKHEAD_POS.1,
        on_deck_z(BULKHEAD_Z),
    ) + mfc_cartridge_nests().translate(MFC_BANK_POS.0, MFC_BANK_POS.1, on_deck_z(MFC_BANK_Z))
        + mixing_manifold_surrogate().translate(
            MANIFOLD_POS.0,
            MANIFOLD_POS.1,
            on_deck_z(MANIFOLD_Z),
        )
        + calibration_gas_bottle_pockets().translate(
            CAL_BOTTLE_POS.0,
            CAL_BOTTLE_POS.1,
            on_deck_z(CAL_BOTTLE_Z),
        )
        + dewpoint_o2_co2_probe_ports().translate(
            PROBE_BLOCK_POS.0,
            PROBE_BLOCK_POS.1,
            on_deck_z(PROBE_BLOCK_Z),
        )
        + pressure_relief_vent_path().translate(
            RELIEF_BLOCK_POS.0,
            RELIEF_BLOCK_POS.1,
            on_deck_z(RELIEF_BLOCK_Z),
        )
        + leak_test_coupon_wells().translate(
            LEAK_COUPON_POS.0,
            LEAK_COUPON_POS.1,
            on_deck_z(LEAK_COUPON_Z),
        )
        + keyed_tubing_route_channels().translate(
            ROUTE_CHANNEL_POS.0,
            ROUTE_CHANNEL_POS.1,
            on_deck_z(ROUTE_CHANNEL_Z),
        )
        + barcode_status_panels().translate(
            STATUS_PANEL_POS.0,
            STATUS_PANEL_POS.1,
            on_deck_z(STATUS_PANEL_Z),
        )
}

fn cylinder_regulator_inlet_bulkheads() -> Part {
    let block = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_cylinder_regulator_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let rear_mount_flange = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_bulkhead_rear_mount_flange",
        BULKHEAD_X + 34.0,
        16.0,
        38.0,
    )
    .translate(0.0, BULKHEAD_Y / 2.0 + 8.0, -BULKHEAD_Z / 2.0 + 34.0);

    let mut cutouts = Part::empty("closed_oxygen_mfc_blend_cal_station_bulkhead_cutouts");
    let mut hardware_envelopes =
        Part::empty("closed_oxygen_mfc_blend_cal_station_regulator_hardware_envelopes");
    for (i, _) in GAS_NAMES.iter().enumerate() {
        let x = centered_index(i, GAS_CHANNELS, BULKHEAD_PITCH_X);
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_inlet_bulkhead_bore_{i}"),
                INLET_PORT_D / 2.0,
                BULKHEAD_Y + 4.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 30.0)
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_bulkhead_key_flat_{i}"),
                26.0,
                BULKHEAD_Y + 5.0,
                5.0,
            )
            .translate(x, 0.0, 53.0);
        hardware_envelopes = hardware_envelopes
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_regulator_envelope_{i}"),
                REGULATOR_ENVELOPE_X,
                REGULATOR_ENVELOPE_Y,
                REGULATOR_ENVELOPE_Z,
            )
            .translate(
                x,
                -BULKHEAD_Y / 2.0 - REGULATOR_ENVELOPE_Y / 2.0 - 8.0,
                12.0,
            )
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_regulator_knob_clearance_{i}"),
                22.0,
                14.0,
                32,
            )
            .translate(x, -BULKHEAD_Y - REGULATOR_ENVELOPE_Y - 6.0, 38.0);
    }

    block + rear_mount_flange - cutouts + hardware_envelopes + bulkhead_channel_tags()
}

fn bulkhead_channel_tags() -> Part {
    let mut tags = Part::empty("closed_oxygen_mfc_blend_cal_station_bulkhead_channel_tags");
    for (i, _) in GAS_NAMES.iter().enumerate() {
        tags = tags
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_bulkhead_channel_tag_{i}"),
                54.0,
                4.0,
                16.0,
            )
            .translate(
                centered_index(i, GAS_CHANNELS, BULKHEAD_PITCH_X),
                -BULKHEAD_Y / 2.0 - 2.0,
                -BULKHEAD_Z / 2.0 + 16.0,
            );
    }
    tags
}

fn mfc_cartridge_nests() -> Part {
    let tray = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_mfc_cartridge_nest_tray",
        MFC_BANK_X,
        MFC_BANK_Y,
        MFC_BANK_Z,
    );
    let mut pockets = Part::empty("closed_oxygen_mfc_blend_cal_station_mfc_pockets");
    let mut locators = Part::empty("closed_oxygen_mfc_blend_cal_station_mfc_key_locators");
    for (i, _) in GAS_NAMES.iter().enumerate() {
        let x = centered_index(i, GAS_CHANNELS, MFC_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_envelope_pocket_{i}"),
                MFC_ENVELOPE_X,
                MFC_ENVELOPE_Y,
                MFC_ENVELOPE_Z,
            )
            .translate(x, 0.0, MFC_BANK_Z / 2.0 - MFC_ENVELOPE_Z / 2.0 + 1.0)
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_inlet_bore_{i}"),
                TUBE_CLEARANCE_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 36.0, MFC_BANK_Y / 2.0 - 12.0, 2.0)
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_outlet_bore_{i}"),
                TUBE_CLEARANCE_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + 36.0, -MFC_BANK_Y / 2.0 + 12.0, 2.0);
        locators = locators
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_asymmetric_key_{i}"),
                13.0,
                40.0,
                12.0,
            )
            .translate(x - 51.0, -MFC_BANK_Y / 2.0 + 34.0, MFC_BANK_Z / 2.0 + 6.0)
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_retainer_land_{i}"),
                76.0,
                10.0,
                10.0,
            )
            .translate(x, MFC_BANK_Y / 2.0 - 16.0, MFC_BANK_Z / 2.0 + 5.0);
    }
    tray - pockets + locators + mfc_bank_dividers()
}

fn mfc_bank_dividers() -> Part {
    let mut dividers = Part::empty("closed_oxygen_mfc_blend_cal_station_mfc_bank_dividers");
    for i in 0..(GAS_CHANNELS - 1) {
        let x = centered_index(i, GAS_CHANNELS, MFC_PITCH_X) + MFC_PITCH_X / 2.0;
        dividers = dividers
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_mfc_cross_channel_divider_{i}"),
                8.0,
                MFC_BANK_Y - 24.0,
                26.0,
            )
            .translate(x, 0.0, MFC_BANK_Z / 2.0 + 13.0);
    }
    dividers
}

fn mixing_manifold_surrogate() -> Part {
    let block = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_mixing_manifold_surrogate_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mixing_bore = centered_cylinder(
        "closed_oxygen_mfc_blend_cal_station_mixing_manifold_longitudinal_bore",
        MIXING_TUBE_D / 2.0,
        MANIFOLD_X - 70.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 8.0);

    let mut ports = Part::empty("closed_oxygen_mfc_blend_cal_station_mixing_manifold_ports");
    for i in 0..GAS_CHANNELS {
        ports = ports
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_manifold_mfc_port_{i}"),
                TUBE_CLEARANCE_D / 2.0,
                MANIFOLD_Y + 6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, GAS_CHANNELS, MANIFOLD_PORT_PITCH_X),
                0.0,
                8.0,
            );
    }
    let outlet = centered_cylinder(
        "closed_oxygen_mfc_blend_cal_station_blended_outlet_to_closed_cabinet_bore",
        8.0,
        58.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(MANIFOLD_X / 2.0 - 18.0, 0.0, 8.0);

    block - mixing_bore - ports - outlet + manifold_flow_arrow_lands() + manifold_mount_bosses()
}

fn manifold_flow_arrow_lands() -> Part {
    let mut arrows = Part::empty("closed_oxygen_mfc_blend_cal_station_manifold_flow_arrow_lands");
    for i in 0..GAS_CHANNELS {
        arrows = arrows
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_manifold_flow_land_{i}"),
                46.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(i, GAS_CHANNELS, MANIFOLD_PORT_PITCH_X),
                MANIFOLD_Y / 2.0 - 22.0,
                MANIFOLD_Z / 2.0 + 2.0,
            );
    }
    arrows
}

fn manifold_mount_bosses() -> Part {
    let mut bosses = Part::empty("closed_oxygen_mfc_blend_cal_station_manifold_mount_bosses");
    for (i, x) in [-MANIFOLD_X / 2.0 + 34.0, MANIFOLD_X / 2.0 - 34.0]
        .iter()
        .enumerate()
    {
        bosses = bosses
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_manifold_mount_boss_{i}"),
                12.0,
                10.0,
                28,
            )
            .translate(*x, -MANIFOLD_Y / 2.0 + 28.0, MANIFOLD_Z / 2.0 + 5.0);
    }
    bosses
}

fn calibration_gas_bottle_pockets() -> Part {
    let tray = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_calibration_gas_bottle_tray",
        CAL_BOTTLE_X,
        CAL_BOTTLE_Y,
        CAL_BOTTLE_Z,
    );
    let mut wells = Part::empty("closed_oxygen_mfc_blend_cal_station_calibration_bottle_wells");
    let mut neck_clips =
        Part::empty("closed_oxygen_mfc_blend_cal_station_calibration_bottle_neck_clips");
    for i in 0..CALIBRATION_BOTTLE_POCKETS {
        let x = centered_index(i, CALIBRATION_BOTTLE_POCKETS, CAL_BOTTLE_PITCH_X);
        wells = wells
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_cal_bottle_pocket_{i}"),
                CAL_BOTTLE_D / 2.0,
                CAL_BOTTLE_DEPTH,
                40,
            )
            .translate(x, 0.0, CAL_BOTTLE_Z / 2.0 - CAL_BOTTLE_DEPTH / 2.0 + 2.0);
        neck_clips = neck_clips
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_cal_bottle_neck_clip_{i}"),
                42.0,
                10.0,
                18.0,
            )
            .translate(x, CAL_BOTTLE_Y / 2.0 - 18.0, CAL_BOTTLE_Z / 2.0 + 9.0);
    }
    tray - wells + neck_clips + calibration_bottle_label_lands()
}

fn calibration_bottle_label_lands() -> Part {
    let mut lands = Part::empty("closed_oxygen_mfc_blend_cal_station_cal_bottle_label_lands");
    for i in 0..CALIBRATION_BOTTLE_POCKETS {
        lands = lands
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_cal_bottle_label_land_{i}"),
                54.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(i, CALIBRATION_BOTTLE_POCKETS, CAL_BOTTLE_PITCH_X),
                -CAL_BOTTLE_Y / 2.0 + 20.0,
                CAL_BOTTLE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn dewpoint_o2_co2_probe_ports() -> Part {
    let block = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_dewpoint_o2_co2_probe_block",
        PROBE_BLOCK_X,
        PROBE_BLOCK_Y,
        PROBE_BLOCK_Z,
    );
    let mut bores = Part::empty("closed_oxygen_mfc_blend_cal_station_probe_port_bores");
    let mut collars = Part::empty("closed_oxygen_mfc_blend_cal_station_probe_port_collars");
    for row in 0..2 {
        for col in 0..PROBE_PORT_TYPES {
            let idx = row * PROBE_PORT_TYPES + col;
            let x = centered_index(col, PROBE_PORT_TYPES, PROBE_PITCH_X);
            let y = centered_index(row, 2, PROBE_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("closed_oxygen_mfc_blend_cal_station_probe_bore_{idx}"),
                    PROBE_PORT_D / 2.0,
                    PROBE_BLOCK_Z + 4.0,
                    36,
                )
                .translate(x, y, 0.0);
            collars = collars
                + centered_cylinder(
                    format!("closed_oxygen_mfc_blend_cal_station_probe_collar_{idx}"),
                    PROBE_PORT_D / 2.0 + 8.0,
                    8.0,
                    36,
                )
                .translate(x, y, PROBE_BLOCK_Z / 2.0 + 4.0);
        }
    }
    block - bores + collars + probe_type_label_lands()
}

fn probe_type_label_lands() -> Part {
    let mut labels = Part::empty("closed_oxygen_mfc_blend_cal_station_probe_type_labels");
    for col in 0..PROBE_PORT_TYPES {
        labels = labels
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_probe_type_label_{col}"),
                58.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(col, PROBE_PORT_TYPES, PROBE_PITCH_X),
                -PROBE_BLOCK_Y / 2.0 + 18.0,
                PROBE_BLOCK_Z / 2.0 + 1.5,
            );
    }
    labels
}

fn pressure_relief_vent_path() -> Part {
    let block = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_pressure_relief_vent_block",
        RELIEF_BLOCK_X,
        RELIEF_BLOCK_Y,
        RELIEF_BLOCK_Z,
    );
    let mut bores = Part::empty("closed_oxygen_mfc_blend_cal_station_relief_vent_bores");
    let mut stacks = Part::empty("closed_oxygen_mfc_blend_cal_station_relief_vent_stack_envelopes");
    for i in 0..RELIEF_VENT_PATHS {
        let y = centered_index(i, RELIEF_VENT_PATHS, 58.0);
        bores = bores
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_relief_cross_bore_{i}"),
                RELIEF_BORE_D / 2.0,
                RELIEF_BLOCK_X + 4.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 8.0)
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_relief_vertical_bore_{i}"),
                RELIEF_BORE_D / 2.0,
                RELIEF_BLOCK_Z + 24.0,
                32,
            )
            .translate(RELIEF_BLOCK_X / 2.0 - 52.0, y, 12.0);
        stacks = stacks
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_relief_valve_envelope_{i}"),
                22.0,
                42.0,
                36,
            )
            .translate(RELIEF_BLOCK_X / 2.0 - 52.0, y, RELIEF_BLOCK_Z / 2.0 + 21.0);
    }
    block - bores + stacks + relief_flow_label() + relief_splash_deflector()
}

fn relief_flow_label() -> Part {
    centered_cube(
        "closed_oxygen_mfc_blend_cal_station_relief_path_direction_label_land",
        RELIEF_BLOCK_X - 54.0,
        16.0,
        3.0,
    )
    .translate(
        -10.0,
        -RELIEF_BLOCK_Y / 2.0 + 20.0,
        RELIEF_BLOCK_Z / 2.0 + 1.5,
    )
}

fn relief_splash_deflector() -> Part {
    centered_cube(
        "closed_oxygen_mfc_blend_cal_station_relief_splash_deflector_wall",
        12.0,
        RELIEF_BLOCK_Y - 24.0,
        58.0,
    )
    .translate(
        RELIEF_BLOCK_X / 2.0 - 18.0,
        0.0,
        RELIEF_BLOCK_Z / 2.0 + 29.0,
    )
}

fn leak_test_coupon_wells() -> Part {
    let tray = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_leak_test_coupon_tray",
        LEAK_COUPON_X,
        LEAK_COUPON_Y,
        LEAK_COUPON_Z,
    );
    let mut wells = Part::empty("closed_oxygen_mfc_blend_cal_station_leak_coupon_wells");
    for i in 0..LEAK_COUPON_WELLS {
        wells = wells
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_coupon_well_{i}"),
                COUPON_WELL_X,
                COUPON_WELL_Y,
                26.0,
            )
            .translate(
                centered_index(i, LEAK_COUPON_WELLS, COUPON_PITCH_X),
                0.0,
                LEAK_COUPON_Z / 2.0 - 12.0,
            )
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_coupon_pin_bore_{i}"),
                3.0,
                COUPON_WELL_Y + 4.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, LEAK_COUPON_WELLS, COUPON_PITCH_X),
                0.0,
                LEAK_COUPON_Z / 2.0 + 1.0,
            );
    }
    tray - wells + leak_coupon_index_lands()
}

fn leak_coupon_index_lands() -> Part {
    let mut lands = Part::empty("closed_oxygen_mfc_blend_cal_station_leak_coupon_index_lands");
    for i in 0..LEAK_COUPON_WELLS {
        lands = lands
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_coupon_index_land_{i}"),
                30.0,
                8.0,
                3.0,
            )
            .translate(
                centered_index(i, LEAK_COUPON_WELLS, COUPON_PITCH_X),
                -LEAK_COUPON_Y / 2.0 + 14.0,
                LEAK_COUPON_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn keyed_tubing_route_channels() -> Part {
    let block = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_keyed_tubing_route_block",
        ROUTE_CHANNEL_X,
        ROUTE_CHANNEL_Y,
        ROUTE_CHANNEL_Z,
    );
    let mut channels = Part::empty("closed_oxygen_mfc_blend_cal_station_keyed_tube_channels");
    let mut keys = Part::empty("closed_oxygen_mfc_blend_cal_station_route_key_teeth");
    for i in 0..KEYED_ROUTE_CHANNELS {
        let y = centered_index(i, KEYED_ROUTE_CHANNELS, ROUTE_PITCH_Y);
        channels = channels
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_route_channel_{i}"),
                TUBE_CLEARANCE_D / 2.0,
                ROUTE_CHANNEL_X - 44.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, ROUTE_CHANNEL_Z / 2.0 - 8.0);
        keys = keys
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_route_key_tooth_{i}"),
                18.0 + i as f64 * 2.0,
                4.0,
                10.0,
            )
            .translate(
                -ROUTE_CHANNEL_X / 2.0 + 36.0,
                y,
                ROUTE_CHANNEL_Z / 2.0 + 5.0,
            );
    }
    block - channels + keys + route_cover_bosses()
}

fn route_cover_bosses() -> Part {
    let mut bosses = Part::empty("closed_oxygen_mfc_blend_cal_station_route_cover_bosses");
    for (i, x) in [
        -ROUTE_CHANNEL_X / 2.0 + 38.0,
        -ROUTE_CHANNEL_X / 6.0,
        ROUTE_CHANNEL_X / 6.0,
        ROUTE_CHANNEL_X / 2.0 - 38.0,
    ]
    .iter()
    .enumerate()
    {
        bosses = bosses
            + centered_cylinder(
                format!("closed_oxygen_mfc_blend_cal_station_route_cover_boss_{i}"),
                8.0,
                8.0,
                24,
            )
            .translate(
                *x,
                ROUTE_CHANNEL_Y / 2.0 - 18.0,
                ROUTE_CHANNEL_Z / 2.0 + 4.0,
            );
    }
    bosses
}

fn barcode_status_panels() -> Part {
    let panel = centered_cube(
        "closed_oxygen_mfc_blend_cal_station_barcode_status_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    );
    panel + barcode_lands() + status_lane_flags()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_oxygen_mfc_blend_cal_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 4;
        let row = i / 4;
        lands = lands
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_barcode_land_{i}"),
                50.0,
                13.0,
                3.0,
            )
            .translate(
                centered_index(col, 4, 58.0),
                -STATUS_PANEL_Y / 2.0 + 24.0 + row as f64 * 22.0,
                STATUS_PANEL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn status_lane_flags() -> Part {
    let mut flags = Part::empty("closed_oxygen_mfc_blend_cal_station_status_lane_flags");
    for i in 0..STATUS_LANES {
        flags = flags
            + centered_cube(
                format!("closed_oxygen_mfc_blend_cal_station_status_lane_flag_{i}"),
                STATUS_SLOT_X,
                STATUS_SLOT_Y,
                8.0,
            )
            .translate(
                centered_index(i, STATUS_LANES, 80.0),
                STATUS_PANEL_Y / 2.0 - 24.0,
                STATUS_PANEL_Z / 2.0 + 4.0,
            );
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_deterministic_and_scoped() {
        for output in [BASE_OUTPUT, CORE_OUTPUT, ASSEMBLY_OUTPUT] {
            assert!(output.starts_with(OUTPUT_PREFIX));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_counts_are_represented() {
        assert_eq!(REGULATOR_INLET_BULKHEADS, 4);
        assert_eq!(MFC_CARTRIDGE_NESTS, 4);
        assert_eq!(CALIBRATION_BOTTLE_POCKETS, 4);
        assert_eq!(PROBE_PORTS, 6);
        assert_eq!(RELIEF_VENT_PATHS, 2);
        assert_eq!(LEAK_COUPON_WELLS, 8);
        assert_eq!(KEYED_ROUTE_CHANNELS, 6);
        assert!(BARCODE_LANDS >= 12);
        assert!(GUARD_RAIL_SEGMENTS >= 6);
    }

    #[test]
    fn layout_is_inside_deck_and_non_overlapping() {
        assert_design_constraints();
    }
}
