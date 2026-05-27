use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterility/mycoplasma sample custody split station.
//
// This is fixture and automation-datum CAD only. It packages a sealed inlet,
// split-manifold surrogate, sterility and mycoplasma vial nests, control
// segregation, cold-block pocket, traceability lands, cap/seal staging,
// waste/flush routing, suspect isolation, evidence imaging, and robot/service
// keepouts for closed cell-culture validation work. It does not define a
// sterility test, mycoplasma assay, acceptance criterion, sampling plan, or
// release rule.

const OUTPUTS: [&str; 12] = [
    "output/closed_myco_sterility_sample_custody_split_station_base_tray.stl",
    "output/closed_myco_sterility_sample_custody_split_station_sealed_sample_inlet_dock.stl",
    "output/closed_myco_sterility_sample_custody_split_station_split_manifold_surrogate.stl",
    "output/closed_myco_sterility_sample_custody_split_station_sterility_myco_vial_nests.stl",
    "output/closed_myco_sterility_sample_custody_split_station_control_segregation_lanes.stl",
    "output/closed_myco_sterility_sample_custody_split_station_cold_block_pocket.stl",
    "output/closed_myco_sterility_sample_custody_split_station_barcode_coa_lands.stl",
    "output/closed_myco_sterility_sample_custody_split_station_cap_seal_staging.stl",
    "output/closed_myco_sterility_sample_custody_split_station_waste_flush_route.stl",
    "output/closed_myco_sterility_sample_custody_split_station_contamination_suspect_isolation_cover.stl",
    "output/closed_myco_sterility_sample_custody_split_station_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_myco_sterility_sample_custody_split_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_sample_inlet_dock",
    "split_manifold_surrogate",
    "sterility_myco_vial_nests",
    "positive_negative_control_segregation",
    "cold_block_pocket",
    "barcode_coa_lands",
    "cap_seal_staging",
    "waste_flush_route",
    "contamination_suspect_isolation_cover",
    "evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 5.0;

const INLET_X: f64 = 330.0;
const INLET_Y: f64 = 224.0;
const INLET_Z: f64 = 64.0;
const INLET_POS: (f64, f64) = (-462.0, 248.0);
const INLET_CLAMP_COUNT: usize = 4;
const INLET_DOCK_PORTS: usize = 2;
const SEPTUM_D: f64 = 28.0;

const MANIFOLD_X: f64 = 514.0;
const MANIFOLD_Y: f64 = 168.0;
const MANIFOLD_Z: f64 = 70.0;
const MANIFOLD_POS: (f64, f64) = (-10.0, 248.0);
const SPLIT_BRANCHES: usize = 6;
const BRANCH_CHANNEL_W: f64 = 18.0;
const BRANCH_PITCH_X: f64 = 72.0;
const HEADER_D: f64 = 24.0;

const VIAL_NEST_X: f64 = 560.0;
const VIAL_NEST_Y: f64 = 286.0;
const VIAL_NEST_Z: f64 = 48.0;
const VIAL_POS: (f64, f64) = (-10.0, -54.0);
const STERILITY_VIALS: usize = 6;
const MYCO_VIALS: usize = 6;
const TOTAL_SAMPLE_VIALS: usize = STERILITY_VIALS + MYCO_VIALS;
const VIAL_D: f64 = 22.0;
const VIAL_CLEARANCE_D: f64 = 25.0;
const VIAL_DEPTH: f64 = 32.0;
const VIAL_PITCH_X: f64 = 56.0;
const VIAL_PITCH_Y: f64 = 98.0;

const CONTROL_X: f64 = 344.0;
const CONTROL_Y: f64 = 236.0;
const CONTROL_Z: f64 = 52.0;
const CONTROL_POS: (f64, f64) = (458.0, 192.0);
const CONTROL_BANKS: usize = 2;
const CONTROL_WELLS_PER_BANK: usize = 4;
const CONTROL_WELL_D: f64 = 21.0;
const CONTROL_BANK_GAP: f64 = 44.0;

const COLD_BLOCK_X: f64 = 354.0;
const COLD_BLOCK_Y: f64 = 182.0;
const COLD_BLOCK_Z: f64 = 58.0;
const COLD_POS: (f64, f64) = (456.0, -86.0);
const COLD_WELLS: usize = 8;
const COLD_WELL_D: f64 = 18.0;
const COLD_WELL_DEPTH: f64 = 34.0;
const COLD_WELL_PITCH: f64 = 38.0;

const TRACE_X: f64 = 430.0;
const TRACE_Y: f64 = 140.0;
const TRACE_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (-415.0, -282.0);
const BARCODE_LANDS: usize = 12;
const COA_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 82.0;
const BARCODE_LAND_Y: f64 = 24.0;
const COA_LAND_X: f64 = 98.0;
const COA_LAND_Y: f64 = 38.0;

const CAP_STAGE_X: f64 = 376.0;
const CAP_STAGE_Y: f64 = 150.0;
const CAP_STAGE_Z: f64 = 34.0;
const CAP_POS: (f64, f64) = (44.0, -300.0);
const CAP_WELLS: usize = TOTAL_SAMPLE_VIALS + CONTROL_BANKS * CONTROL_WELLS_PER_BANK;
const SEAL_TOKEN_SLOTS: usize = 8;
const CAP_WELL_D: f64 = 16.0;
const CAP_PITCH_X: f64 = 34.0;

const WASTE_X: f64 = 354.0;
const WASTE_Y: f64 = 118.0;
const WASTE_Z: f64 = 42.0;
const WASTE_POS: (f64, f64) = (456.0, -300.0);
const FLUSH_PORTS: usize = 6;
const WASTE_SUMP_D: f64 = 42.0;

const ISOLATION_X: f64 = 344.0;
const ISOLATION_Y: f64 = 232.0;
const ISOLATION_Z: f64 = 205.0;
const ISOLATION_POS: (f64, f64) = (-470.0, 16.0);
const ISOLATION_VIALS: usize = 4;
const ISOLATION_GASKET_W: f64 = 9.0;

const BRIDGE_X: f64 = 1060.0;
const BRIDGE_Y: f64 = 86.0;
const BRIDGE_Z: f64 = 210.0;
const BRIDGE_POS: (f64, f64) = (22.0, 354.0);
const CAMERA_PODS: usize = 4;
const EVIDENCE_LANDS: usize = 5;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 132.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 116.0;
const LEFT_SERVICE_KEEP_OUT_X: f64 = 84.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 96.0;
const ROBOT_Z_CLEARANCE: f64 = 320.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = DECK_X / 2.0 - RIM_W;
        let half_y = DECK_Y / 2.0 - RIM_W;
        self.center.0 - self.x / 2.0 >= -half_x
            && self.center.0 + self.x / 2.0 <= half_x
            && self.center.1 - self.y / 2.0 >= -half_y
            && self.center.1 + self.y / 2.0 <= half_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let inlet = sealed_sample_inlet_dock();
    export(OUTPUTS[1], &inlet);

    let manifold = split_manifold_surrogate();
    export(OUTPUTS[2], &manifold);

    let vial_nests = sterility_myco_vial_nests();
    export(OUTPUTS[3], &vial_nests);

    let controls = control_segregation_lanes();
    export(OUTPUTS[4], &controls);

    let cold = cold_block_pocket();
    export(OUTPUTS[5], &cold);

    let traceability = barcode_coa_lands();
    export(OUTPUTS[6], &traceability);

    let caps = cap_seal_staging();
    export(OUTPUTS[7], &caps);

    let waste = waste_flush_route();
    export(OUTPUTS[8], &waste);

    let isolation = contamination_suspect_isolation_cover();
    export(OUTPUTS[9], &isolation);

    let bridge_keepouts = evidence_bridge_robot_service_keepouts();
    export(OUTPUTS[10], &bridge_keepouts);

    let assembly = base
        + inlet.translate(INLET_POS.0, INLET_POS.1, on_deck_z(INLET_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_deck_z(MANIFOLD_Z))
        + vial_nests.translate(VIAL_POS.0, VIAL_POS.1, on_deck_z(VIAL_NEST_Z))
        + controls.translate(CONTROL_POS.0, CONTROL_POS.1, on_deck_z(CONTROL_Z))
        + cold.translate(COLD_POS.0, COLD_POS.1, on_deck_z(COLD_BLOCK_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + caps.translate(CAP_POS.0, CAP_POS.1, on_deck_z(CAP_STAGE_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_Z))
        + isolation.translate(ISOLATION_POS.0, ISOLATION_POS.1, on_deck_z(ISOLATION_Z))
        + bridge_keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed sterility/mycoplasma sample custody split station:");
    println!(
        "  Footprint:             {DECK_X:.0}mm x {DECK_Y:.0}mm deck with raised containment rim"
    );
    println!(
        "  Closed sample path:    {INLET_DOCK_PORTS} sealed inlet ports, {SPLIT_BRANCHES} split-manifold surrogate branches, and {FLUSH_PORTS} waste/flush indexed ports"
    );
    println!(
        "  Sample custody:        {STERILITY_VIALS} sterility vial nests, {MYCO_VIALS} mycoplasma vial nests, {ISOLATION_VIALS} suspect-isolation nests, and {COLD_WELLS} cold-block positions"
    );
    println!(
        "  Controls/traceability: {CONTROL_BANKS} segregated positive/negative banks with {CONTROL_WELLS_PER_BANK} wells each, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {CAP_WELLS} cap wells, and {SEAL_TOKEN_SLOTS} seal-token slots"
    );
    println!(
        "  Evidence/keepouts:     {CAMERA_PODS} camera pods, {EVIDENCE_LANDS} evidence lands, {FRONT_ROBOT_KEEP_OUT_Y:.0}mm front robot keepout, {REAR_SERVICE_KEEP_OUT_Y:.0}mm rear service keepout, and {ROBOT_Z_CLEARANCE:.0}mm Z clearance gauge"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
    println!("  Limitation:            Mechanical CAD only; no sterility/mycoplasma assay protocol or release rule is encoded.");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    DECK_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(TOTAL_SAMPLE_VIALS, 12);
    assert_eq!(CAP_WELLS, 20);
    assert_eq!(CONTROL_BANKS, 2);
    assert!(closed_fraction_count() >= TOTAL_SAMPLE_VIALS);
    assert!(cold_chain_capacity() >= STERILITY_VIALS);
    assert!(contamination_isolation_capacity() >= ISOLATION_VIALS);
    assert!(flush_route_capacity_ml() > closed_manifold_hold_up_ml());

    let modules = module_rects();
    for module in modules {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds deck containment boundary",
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

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "sealed_sample_inlet_dock",
            center: INLET_POS,
            x: INLET_X,
            y: INLET_Y,
        },
        Rect {
            name: "split_manifold_surrogate",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Rect {
            name: "sterility_myco_vial_nests",
            center: VIAL_POS,
            x: VIAL_NEST_X,
            y: VIAL_NEST_Y,
        },
        Rect {
            name: "control_segregation_lanes",
            center: CONTROL_POS,
            x: CONTROL_X,
            y: CONTROL_Y,
        },
        Rect {
            name: "cold_block_pocket",
            center: COLD_POS,
            x: COLD_BLOCK_X,
            y: COLD_BLOCK_Y,
        },
        Rect {
            name: "barcode_coa_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "cap_seal_staging",
            center: CAP_POS,
            x: CAP_STAGE_X,
            y: CAP_STAGE_Y,
        },
        Rect {
            name: "waste_flush_route",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "contamination_suspect_isolation_cover",
            center: ISOLATION_POS,
            x: ISOLATION_X,
            y: ISOLATION_Y,
        },
    ]
}

fn closed_fraction_count() -> usize {
    SPLIT_BRANCHES * 2
}

fn cold_chain_capacity() -> usize {
    COLD_WELLS
}

fn contamination_isolation_capacity() -> usize {
    ISOLATION_VIALS
}

fn closed_manifold_hold_up_ml() -> f64 {
    SPLIT_BRANCHES as f64 * 1.8 + 6.0
}

fn flush_route_capacity_ml() -> f64 {
    FLUSH_PORTS as f64 * 6.0
}

fn base_tray() -> Part {
    let deck = centered_cube("closed_myco_custody_base_deck", DECK_X, DECK_Y, DECK_Z);

    let front_rim = centered_cube("closed_myco_custody_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        on_deck_z(RIM_Z),
    );
    let rear_rim = centered_cube("closed_myco_custody_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        on_deck_z(RIM_Z),
    );
    let left_rim = centered_cube("closed_myco_custody_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        on_deck_z(RIM_Z),
    );
    let right_rim = centered_cube("closed_myco_custody_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        on_deck_z(RIM_Z),
    );

    let mut sockets = Part::empty("closed_myco_custody_module_socket_recesses");
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("closed_myco_custody_{}_socket", module.name),
                module.x + 18.0,
                module.y + 18.0,
                SOCKET_DEPTH,
            )
            .translate(module.center.0, module.center.1, DECK_Z / 2.0);
    }

    let mut datums = Part::empty("closed_myco_custody_deck_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_myco_custody_datum_target_{i}"),
                10.0,
                4.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0 + 2.0);
    }

    deck - sockets + front_rim + rear_rim + left_rim + right_rim + datums
}

fn sealed_sample_inlet_dock() -> Part {
    let block = centered_cube(
        "closed_myco_custody_inlet_dock_body",
        INLET_X,
        INLET_Y,
        INLET_Z,
    );
    let gasket_land = centered_cube(
        "closed_myco_custody_inlet_dock_compression_gasket_land",
        INLET_X - 36.0,
        INLET_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, INLET_Z / 2.0 + 4.0);

    let mut port_cuts = Part::empty("closed_myco_custody_inlet_port_cuts");
    let mut port_bosses = Part::empty("closed_myco_custody_inlet_port_bosses");
    for port in 0..INLET_DOCK_PORTS {
        let x = centered_index(port, INLET_DOCK_PORTS, 72.0);
        port_cuts = port_cuts
            + centered_cylinder(
                format!("closed_myco_custody_inlet_luer_relief_{port}"),
                SEPTUM_D / 2.0,
                INLET_Z + 14.0,
                40,
            )
            .translate(x, -42.0, 0.0);
        port_bosses = port_bosses
            + centered_cylinder(
                format!("closed_myco_custody_inlet_septum_boss_{port}"),
                SEPTUM_D / 2.0 + 9.0,
                12.0,
                40,
            )
            .translate(x, -42.0, INLET_Z / 2.0 + 6.0);
    }

    let mut clamps = Part::empty("closed_myco_custody_inlet_clamp_saddles");
    for clamp in 0..INLET_CLAMP_COUNT {
        let x = centered_index(clamp, INLET_CLAMP_COUNT, 68.0);
        let saddle = centered_cube(
            format!("closed_myco_custody_inlet_clamp_saddle_{clamp}"),
            42.0,
            28.0,
            18.0,
        )
        .translate(x, 48.0, INLET_Z / 2.0 + 9.0);
        let tube_relief = centered_cylinder(
            format!("closed_myco_custody_inlet_clamp_tube_relief_{clamp}"),
            8.0,
            54.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 48.0, INLET_Z / 2.0 + 10.0);
        clamps = clamps + (saddle - tube_relief);
    }

    block - port_cuts + gasket_land + port_bosses + clamps
}

fn split_manifold_surrogate() -> Part {
    let base = centered_cube(
        "closed_myco_custody_split_manifold_base",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let header = centered_cylinder(
        "closed_myco_custody_split_manifold_closed_header",
        HEADER_D / 2.0,
        MANIFOLD_X - 52.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, MANIFOLD_Z / 2.0 + 12.0);

    let mut branch_channels = Part::empty("closed_myco_custody_split_branch_channel_reliefs");
    let mut branch_bosses = Part::empty("closed_myco_custody_split_branch_transfer_bosses");
    for branch in 0..SPLIT_BRANCHES {
        let x = centered_index(branch, SPLIT_BRANCHES, BRANCH_PITCH_X);
        branch_channels = branch_channels
            + centered_cube(
                format!("closed_myco_custody_split_branch_{branch}_channel_shadow"),
                BRANCH_CHANNEL_W,
                MANIFOLD_Y + 10.0,
                9.0,
            )
            .translate(x, -16.0, MANIFOLD_Z / 2.0 + 5.0);
        branch_bosses = branch_bosses
            + centered_cylinder(
                format!("closed_myco_custody_split_branch_{branch}_sealed_takeoff"),
                13.0,
                16.0,
                32,
            )
            .translate(x, -MANIFOLD_Y / 2.0 + 26.0, MANIFOLD_Z / 2.0 + 8.0)
            + centered_cylinder(
                format!("closed_myco_custody_split_branch_{branch}_check_valve_placeholder"),
                10.0,
                34.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 38.0, MANIFOLD_Z / 2.0 + 15.0);
    }

    let inlet_boss = centered_cylinder(
        "closed_myco_custody_split_manifold_inlet_bulkhead",
        18.0,
        24.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-MANIFOLD_X / 2.0 + 44.0, MANIFOLD_Y / 2.0 - 26.0, 12.0);

    base - branch_channels + header + branch_bosses + inlet_boss
}

fn sterility_myco_vial_nests() -> Part {
    let plate = centered_cube(
        "closed_myco_custody_sterility_myco_vial_nest_plate",
        VIAL_NEST_X,
        VIAL_NEST_Y,
        VIAL_NEST_Z,
    );
    let divider = centered_cube(
        "closed_myco_custody_sterility_myco_center_divider",
        VIAL_NEST_X - 42.0,
        10.0,
        VIAL_NEST_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);

    let mut wells = Part::empty("closed_myco_custody_sample_vial_well_cuts");
    let mut collars = Part::empty("closed_myco_custody_sample_vial_custody_collars");
    for row in 0..2 {
        let y = centered_index(row, 2, VIAL_PITCH_Y);
        for col in 0..STERILITY_VIALS {
            let index = row * STERILITY_VIALS + col;
            let x = centered_index(col, STERILITY_VIALS, VIAL_PITCH_X);
            wells = wells
                + centered_cylinder(
                    format!("closed_myco_custody_sample_vial_well_{index}"),
                    VIAL_CLEARANCE_D / 2.0,
                    VIAL_DEPTH + 2.0,
                    40,
                )
                .translate(x, y, VIAL_NEST_Z / 2.0 - VIAL_DEPTH / 2.0 + 1.0);
            collars = collars
                + centered_cylinder(
                    format!("closed_myco_custody_sample_vial_collar_{index}"),
                    VIAL_D / 2.0 + 8.0,
                    8.0,
                    40,
                )
                .translate(x, y, VIAL_NEST_Z / 2.0 + 4.0);
        }
    }

    let mut row_tags = Part::empty("closed_myco_custody_sterility_myco_row_tags");
    for (row, y) in [(0, -VIAL_PITCH_Y / 2.0), (1, VIAL_PITCH_Y / 2.0)] {
        row_tags = row_tags
            + centered_cube(
                format!("closed_myco_custody_row_{row}_scan_tab"),
                510.0,
                18.0,
                6.0,
            )
            .translate(0.0, y + 36.0, VIAL_NEST_Z / 2.0 + 3.0);
    }

    plate - wells + divider + collars + row_tags
}

fn control_segregation_lanes() -> Part {
    let tray = centered_cube(
        "closed_myco_custody_control_segregation_tray",
        CONTROL_X,
        CONTROL_Y,
        CONTROL_Z,
    );
    let bulkhead = centered_cube(
        "closed_myco_custody_positive_negative_control_bulkhead",
        CONTROL_X - 28.0,
        16.0,
        CONTROL_Z + 32.0,
    )
    .translate(0.0, 0.0, 16.0);

    let mut wells = Part::empty("closed_myco_custody_control_well_cuts");
    let mut guard_rails = Part::empty("closed_myco_custody_control_guard_rails");
    for bank in 0..CONTROL_BANKS {
        let y = if bank == 0 {
            -CONTROL_BANK_GAP
        } else {
            CONTROL_BANK_GAP
        };
        guard_rails = guard_rails
            + centered_cube(
                format!("closed_myco_custody_control_bank_{bank}_custody_rail"),
                CONTROL_X - 44.0,
                9.0,
                CONTROL_Z + 18.0,
            )
            .translate(0.0, y + 34.0, 9.0);
        for well in 0..CONTROL_WELLS_PER_BANK {
            let x = centered_index(well, CONTROL_WELLS_PER_BANK, 54.0);
            wells = wells
                + centered_cylinder(
                    format!("closed_myco_custody_control_bank_{bank}_well_{well}"),
                    CONTROL_WELL_D / 2.0,
                    34.0,
                    32,
                )
                .translate(x, y, CONTROL_Z / 2.0 - 17.0);
        }
    }

    tray - wells + bulkhead + guard_rails
}

fn cold_block_pocket() -> Part {
    let tray = centered_cube(
        "closed_myco_custody_cold_block_pocket_tray",
        COLD_BLOCK_X,
        COLD_BLOCK_Y,
        COLD_BLOCK_Z,
    );
    let block_shadow = centered_cube(
        "closed_myco_custody_cold_block_removable_insert_shadow",
        COLD_BLOCK_X - 44.0,
        COLD_BLOCK_Y - 44.0,
        COLD_BLOCK_Z - 16.0,
    )
    .translate(0.0, 0.0, -6.0);
    let drain_slot = centered_cube(
        "closed_myco_custody_cold_block_condensate_drain_slot",
        COLD_BLOCK_X - 72.0,
        9.0,
        12.0,
    )
    .translate(0.0, -COLD_BLOCK_Y / 2.0 + 24.0, -COLD_BLOCK_Z / 2.0 + 10.0);

    let mut wells = Part::empty("closed_myco_custody_cold_block_well_cuts");
    for i in 0..COLD_WELLS {
        let x = centered_index(i, COLD_WELLS, COLD_WELL_PITCH);
        wells = wells
            + centered_cylinder(
                format!("closed_myco_custody_cold_block_well_{i}"),
                COLD_WELL_D / 2.0,
                COLD_WELL_DEPTH + 1.0,
                32,
            )
            .translate(x, 0.0, COLD_BLOCK_Z / 2.0 - COLD_WELL_DEPTH / 2.0 + 0.5);
    }

    tray - block_shadow - drain_slot - wells
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_myco_custody_barcode_coa_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let title_strip = centered_cube(
        "closed_myco_custody_barcode_coa_title_strip",
        TRACE_X - 34.0,
        16.0,
        6.0,
    )
    .translate(0.0, TRACE_Y / 2.0 - 20.0, TRACE_Z / 2.0 + 3.0);

    let mut lands = Part::empty("closed_myco_custody_barcode_and_coa_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        let x = centered_index(col, 4, 96.0);
        let y = 28.0 - row as f64 * 32.0;
        lands = lands + barcode_land(format!("closed_myco_custody_barcode_land_{i}"), x, y);
    }
    for i in 0..COA_LANDS {
        let x = centered_index(i, COA_LANDS, 102.0);
        lands = lands
            + centered_cube(
                format!("closed_myco_custody_coa_land_{i}"),
                COA_LAND_X,
                COA_LAND_Y,
                4.0,
            )
            .translate(x, -TRACE_Y / 2.0 + 22.0, TRACE_Z / 2.0 + 2.0);
    }

    panel + title_strip + lands + scanner_fiducials()
}

fn barcode_land(name: String, x: f64, y: f64) -> Part {
    let land = centered_cube(name.clone(), BARCODE_LAND_X, BARCODE_LAND_Y, 4.0).translate(
        x,
        y,
        TRACE_Z / 2.0 + 2.0,
    );
    let scan_relief = centered_cube(
        format!("{name}_scan_line_relief"),
        BARCODE_LAND_X - 14.0,
        2.0,
        5.0,
    )
    .translate(x, y, TRACE_Z / 2.0 + 2.5);
    land - scan_relief
}

fn scanner_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_myco_custody_traceability_fiducials");
    for (i, (x, y)) in [
        (-TRACE_X / 2.0 + 22.0, TRACE_Y / 2.0 - 22.0),
        (TRACE_X / 2.0 - 22.0, TRACE_Y / 2.0 - 22.0),
        (-TRACE_X / 2.0 + 22.0, -TRACE_Y / 2.0 + 22.0),
        (TRACE_X / 2.0 - 22.0, -TRACE_Y / 2.0 + 22.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_myco_custody_traceability_fiducial_{i}"),
                4.0,
                4.0,
                20,
            )
            .translate(x, y, TRACE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn cap_seal_staging() -> Part {
    let tray = centered_cube(
        "closed_myco_custody_cap_seal_staging_tray",
        CAP_STAGE_X,
        CAP_STAGE_Y,
        CAP_STAGE_Z,
    );
    let clean_used_divider = centered_cube(
        "closed_myco_custody_cap_seal_clean_used_divider",
        12.0,
        CAP_STAGE_Y - 22.0,
        CAP_STAGE_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);

    let mut cap_wells = Part::empty("closed_myco_custody_cap_staging_wells");
    for cap in 0..CAP_WELLS {
        let row = cap / 10;
        let col = cap % 10;
        let x = centered_index(col, 10, CAP_PITCH_X);
        let y = if row == 0 { -32.0 } else { 32.0 };
        cap_wells = cap_wells
            + centered_cylinder(
                format!("closed_myco_custody_cap_well_{cap}"),
                CAP_WELL_D / 2.0,
                22.0,
                28,
            )
            .translate(x, y, CAP_STAGE_Z / 2.0 - 11.0);
    }

    let mut seal_slots = Part::empty("closed_myco_custody_seal_token_slots");
    for token in 0..SEAL_TOKEN_SLOTS {
        let x = centered_index(token, SEAL_TOKEN_SLOTS, 42.0);
        seal_slots = seal_slots
            + centered_cube(
                format!("closed_myco_custody_seal_token_slot_{token}"),
                24.0,
                12.0,
                10.0,
            )
            .translate(x, 0.0, CAP_STAGE_Z / 2.0 - 5.0);
    }

    tray - cap_wells - seal_slots + clean_used_divider
}

fn waste_flush_route() -> Part {
    let tray = centered_cube(
        "closed_myco_custody_waste_flush_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let channel = centered_cube(
        "closed_myco_custody_flush_channel_shadow",
        WASTE_X - 62.0,
        16.0,
        12.0,
    )
    .translate(0.0, 18.0, WASTE_Z / 2.0 - 6.0);
    let sump = centered_cylinder(
        "closed_myco_custody_waste_sump_relief",
        WASTE_SUMP_D / 2.0,
        WASTE_Z + 2.0,
        40,
    )
    .translate(WASTE_X / 2.0 - 54.0, -22.0, 0.0);

    let mut ports = Part::empty("closed_myco_custody_flush_ports");
    for port in 0..FLUSH_PORTS {
        let x = centered_index(port, FLUSH_PORTS, 42.0) - 28.0;
        ports = ports
            + centered_cylinder(
                format!("closed_myco_custody_flush_port_{port}"),
                7.0,
                18.0,
                28,
            )
            .translate(x, 44.0, WASTE_Z / 2.0 + 9.0);
    }

    tray - channel - sump + ports
}

fn contamination_suspect_isolation_cover() -> Part {
    let base = centered_cube(
        "closed_myco_custody_suspect_isolation_base",
        ISOLATION_X,
        ISOLATION_Y,
        34.0,
    );
    let cover_roof = centered_cube(
        "closed_myco_custody_suspect_isolation_clear_cover_roof",
        ISOLATION_X - 28.0,
        ISOLATION_Y - 28.0,
        18.0,
    )
    .translate(0.0, 0.0, ISOLATION_Z / 2.0 - 9.0);
    let rear_wall = centered_cube(
        "closed_myco_custody_suspect_isolation_rear_wall",
        ISOLATION_X - 28.0,
        14.0,
        ISOLATION_Z,
    )
    .translate(0.0, ISOLATION_Y / 2.0 - 21.0, ISOLATION_Z / 2.0 - 17.0);
    let left_wall = centered_cube(
        "closed_myco_custody_suspect_isolation_left_wall",
        14.0,
        ISOLATION_Y - 28.0,
        ISOLATION_Z,
    )
    .translate(-ISOLATION_X / 2.0 + 21.0, 0.0, ISOLATION_Z / 2.0 - 17.0);
    let right_wall = centered_cube(
        "closed_myco_custody_suspect_isolation_right_wall",
        14.0,
        ISOLATION_Y - 28.0,
        ISOLATION_Z,
    )
    .translate(ISOLATION_X / 2.0 - 21.0, 0.0, ISOLATION_Z / 2.0 - 17.0);
    let gasket = centered_cube(
        "closed_myco_custody_suspect_isolation_gasket_land",
        ISOLATION_X - 46.0,
        ISOLATION_GASKET_W,
        8.0,
    )
    .translate(0.0, -ISOLATION_Y / 2.0 + 32.0, 21.0);

    let mut wells = Part::empty("closed_myco_custody_suspect_isolation_well_cuts");
    for i in 0..ISOLATION_VIALS {
        let x = centered_index(i, ISOLATION_VIALS, 58.0);
        wells = wells
            + centered_cylinder(
                format!("closed_myco_custody_suspect_isolation_vial_well_{i}"),
                14.0,
                30.0,
                32,
            )
            .translate(x, -34.0, 3.0);
    }

    base - wells + cover_roof + rear_wall + left_wall + right_wall + gasket
}

fn evidence_bridge_robot_service_keepouts() -> Part {
    let bridge = evidence_bridge();
    let keepouts = robot_service_keepouts();
    bridge + keepouts
}

fn evidence_bridge() -> Part {
    let rail = centered_cube(
        "closed_myco_custody_evidence_bridge_rail",
        BRIDGE_X,
        BRIDGE_Y,
        28.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1, ROBOT_Z_CLEARANCE - 28.0);

    let mut posts = Part::empty("closed_myco_custody_evidence_bridge_posts");
    for (i, x) in [
        -BRIDGE_X / 2.0 + 36.0,
        -BRIDGE_X / 6.0,
        BRIDGE_X / 6.0,
        BRIDGE_X / 2.0 - 36.0,
    ]
    .iter()
    .copied()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_myco_custody_evidence_bridge_post_{i}"),
                24.0,
                24.0,
                BRIDGE_Z,
            )
            .translate(BRIDGE_POS.0 + x, BRIDGE_POS.1, BRIDGE_Z / 2.0 + DECK_Z);
    }

    let mut pods = Part::empty("closed_myco_custody_evidence_camera_pods");
    for pod in 0..CAMERA_PODS {
        let x = centered_index(pod, CAMERA_PODS, 245.0);
        pods = pods
            + centered_cube(
                format!("closed_myco_custody_evidence_camera_pod_{pod}"),
                64.0,
                50.0,
                26.0,
            )
            .translate(
                BRIDGE_POS.0 + x,
                BRIDGE_POS.1 - 10.0,
                ROBOT_Z_CLEARANCE - 52.0,
            );
    }

    let mut evidence_lands = Part::empty("closed_myco_custody_bridge_evidence_lands");
    for land in 0..EVIDENCE_LANDS {
        let x = centered_index(land, EVIDENCE_LANDS, 176.0);
        evidence_lands = evidence_lands
            + centered_cube(
                format!("closed_myco_custody_evidence_land_{land}"),
                112.0,
                24.0,
                5.0,
            )
            .translate(
                BRIDGE_POS.0 + x,
                BRIDGE_POS.1 - 46.0,
                ROBOT_Z_CLEARANCE - 38.0,
            );
    }

    rail + posts + pods + evidence_lands
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_myco_custody_front_robot_approach_keepout",
        DECK_X - 140.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + RIM_W,
        DECK_Z / 2.0 + 4.0,
    );
    let rear_service = centered_cube(
        "closed_myco_custody_rear_service_sweep_keepout",
        DECK_X - 120.0,
        REAR_SERVICE_KEEP_OUT_Y,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0 - RIM_W,
        DECK_Z / 2.0 + 4.0,
    );
    let left_service = centered_cube(
        "closed_myco_custody_left_inlet_service_keepout",
        LEFT_SERVICE_KEEP_OUT_X,
        DECK_Y - 190.0,
        7.0,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_SERVICE_KEEP_OUT_X / 2.0 + RIM_W,
        0.0,
        DECK_Z / 2.0 + 3.5,
    );
    let right_service = centered_cube(
        "closed_myco_custody_right_cold_block_service_keepout",
        RIGHT_SERVICE_KEEP_OUT_X,
        DECK_Y - 220.0,
        7.0,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_KEEP_OUT_X / 2.0 - RIM_W,
        0.0,
        DECK_Z / 2.0 + 3.5,
    );
    let z_gauge = centered_cube(
        "closed_myco_custody_robot_z_clearance_gauge",
        34.0,
        34.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 72.0,
        -DECK_Y / 2.0 + 72.0,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );

    front_robot + rear_service + left_service + right_service + z_gauge
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_paths_are_unique_and_deterministic() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS[0].ends_with("_base_tray.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_myco_sterility_sample_custody_split_station_"));
        }
    }

    #[test]
    fn required_scope_features_are_represented() {
        for feature in [
            "sealed_sample_inlet_dock",
            "split_manifold_surrogate",
            "sterility_myco_vial_nests",
            "positive_negative_control_segregation",
            "cold_block_pocket",
            "barcode_coa_lands",
            "cap_seal_staging",
            "waste_flush_route",
            "contamination_suspect_isolation_cover",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn custody_counts_match_closed_split_station_intent() {
        assert_eq!(STERILITY_VIALS, 6);
        assert_eq!(MYCO_VIALS, 6);
        assert_eq!(TOTAL_SAMPLE_VIALS, 12);
        assert_eq!(CONTROL_BANKS * CONTROL_WELLS_PER_BANK, 8);
        assert_eq!(
            CAP_WELLS,
            TOTAL_SAMPLE_VIALS + CONTROL_BANKS * CONTROL_WELLS_PER_BANK
        );
        assert!(closed_fraction_count() >= TOTAL_SAMPLE_VIALS);
        assert!(flush_route_capacity_ml() > closed_manifold_hold_up_ml());
    }

    #[test]
    fn modules_fit_without_top_level_overlap() {
        assert_layout_constraints();
    }
}
