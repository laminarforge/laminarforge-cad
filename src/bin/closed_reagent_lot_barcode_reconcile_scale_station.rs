use std::collections::BTreeSet;
use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent/media lot barcode reconciliation and gravimetric verification station.
//
// Design intent:
// - Reconcile sealed reagent/media bag and vial lot identity through barcode/RFID
//   evidence before automated culture runs can consume the material.
// - Pair lot identity checks with a mechanically isolated load-cell/reference-mass
//   pocket so gross-fill or dispensing-kit mass discrepancies are visible at the
//   station instead of being discovered in the workcell.
// - Physically segregate released, hold, reject, expired, and quarantine outcomes
//   while preserving COA/certificate, temperature logger, cap/connector custody,
//   sealed transfer, evidence camera, and robot/service keepout cues.
//
// Mechanical concept CAD only. This file does not define a release rule, GMP
// disposition procedure, barcode parser, electronic record system, sterility claim,
// or metrology acceptance criterion.

const OUTPUT_PREFIX: &str = "output/closed_reagent_lot_barcode_reconcile_scale_station";
const PART_PREFIX: &str = "closed_reagent_lot_barcode_reconcile_scale_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_reagent_lot_barcode_reconcile_scale_station_containment_deck.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_media_bag_vial_nests.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_barcode_rfid_scanner_bridge.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_coa_certificate_land.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_load_cell_reference_mass_pocket.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_released_hold_reject_lanes.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_expired_quarantine_pocket.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_temperature_logger_pocket.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_sealed_transfer_bulkhead.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_cap_connector_custody.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_evidence_camera_bridge.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_robot_service_keepouts.stl",
    "output/closed_reagent_lot_barcode_reconcile_scale_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "media_bag_vial_nests",
    "barcode_rfid_scanner_bridge",
    "coa_certificate_land",
    "load_cell_reference_mass_pocket",
    "released_hold_reject_lanes",
    "expired_quarantine_pocket",
    "temperature_logger_pocket",
    "sealed_transfer_bulkhead",
    "cap_connector_custody",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "no_release_rule",
    "no_barcode_parser",
    "no_electronic_record_claim",
    "purchased_wetted_components_external",
];

const STATION_X: f64 = 1600.0;
const STATION_Y: f64 = 1000.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 44.0;
const SUMP_X: f64 = 1390.0;
const SUMP_Y: f64 = 812.0;
const SUMP_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_BOSSES: usize = 8;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.8;

const NEST_POS: (f64, f64) = (-470.0, 150.0);
const NEST_PANEL_X: f64 = 500.0;
const NEST_PANEL_Y: f64 = 310.0;
const NEST_PANEL_Z: f64 = 48.0;
const MEDIA_BAG_NESTS: usize = 2;
const MEDIA_BAG_NEST_X: f64 = 180.0;
const MEDIA_BAG_NEST_Y: f64 = 205.0;
const MEDIA_BAG_NEST_DEPTH: f64 = 15.0;
const BAG_NEST_PITCH_X: f64 = 220.0;
const VIAL_NESTS: usize = 12;
const VIAL_NEST_COLS: usize = 6;
const VIAL_WELL_D: f64 = 27.0;
const VIAL_WELL_DEPTH: f64 = 16.0;
const VIAL_PITCH_X: f64 = 54.0;
const VIAL_PITCH_Y: f64 = 44.0;
const BAG_DATUM_RAIL_Z: f64 = 28.0;
const LOT_FLAG_LANDS: usize = 8;
const NEST_ORIENTATION_DOTS: usize = MEDIA_BAG_NESTS + VIAL_NESTS;

const SCANNER_POS: (f64, f64) = NEST_POS;
const SCANNER_SPAN_X: f64 = 575.0;
const SCANNER_POST_X: f64 = 26.0;
const SCANNER_POST_Y: f64 = 34.0;
const SCANNER_POST_Z: f64 = 190.0;
const SCANNER_BEAM_Y: f64 = 52.0;
const SCANNER_BEAM_Z: f64 = 28.0;
const BARCODE_CAMERA_PODS: usize = 3;
const RFID_ANTENNA_LANDS: usize = 4;
const SCANNER_CAL_TAGS: usize = 4;
const SCANNER_UNDERSIDE_CLEARANCE_Z: f64 = 162.0;

const SCALE_POS: (f64, f64) = (-20.0, 150.0);
const SCALE_PANEL_X: f64 = 360.0;
const SCALE_PANEL_Y: f64 = 310.0;
const SCALE_PANEL_Z: f64 = 42.0;
const LOAD_CELL_PAD_COUNT: usize = 4;
const LOAD_CELL_PAD_X: f64 = 74.0;
const LOAD_CELL_PAD_Y: f64 = 52.0;
const LOAD_CELL_PAD_Z: f64 = 12.0;
const SCALE_PLATTER_X: f64 = 190.0;
const SCALE_PLATTER_Y: f64 = 132.0;
const SCALE_PLATTER_Z: f64 = 16.0;
const FLEXURE_RELIEF_SLOTS: usize = 6;
const REFERENCE_MASS_WELLS: usize = 8;
const REFERENCE_MASS_COLS: usize = 4;
const MASS_WELL_D: f64 = 38.0;
const MASS_WELL_DEPTH: f64 = 16.0;
const MASS_PITCH_X: f64 = 62.0;
const MASS_PITCH_Y: f64 = 58.0;
const FINE_MASS_WELLS: usize = 6;

const LANE_POS: (f64, f64) = (520.0, 150.0);
const LANE_PANEL_X: f64 = 360.0;
const LANE_PANEL_Y: f64 = 330.0;
const LANE_PANEL_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["released", "hold", "reject"];
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 92.0;
const STATUS_SLOT_Y: f64 = 44.0;
const STATUS_SLOT_DEPTH: f64 = 9.0;
const STATUS_LANE_PITCH_X: f64 = 128.0;
const STATUS_SLOT_PITCH_Y: f64 = 64.0;
const LANE_DIVIDER_W: f64 = 10.0;
const LANE_SHUTTERS: usize = STATUS_LANES;
const LANE_MIN_GAP: f64 = STATUS_LANE_PITCH_X - STATUS_SLOT_X;

const QUARANTINE_POS: (f64, f64) = (-540.0, -285.0);
const QUARANTINE_X: f64 = 360.0;
const QUARANTINE_Y: f64 = 210.0;
const QUARANTINE_Z: f64 = 52.0;
const QUARANTINE_WALL: f64 = 14.0;
const EXPIRED_BINS: usize = 2;
const QUARANTINE_BINS: usize = 2;
const QUARANTINE_TOKEN_WELLS: usize = 8;
const TAMPER_SEAL_POSTS: usize = 6;

const LOGGER_POS: (f64, f64) = (-170.0, -285.0);
const LOGGER_PANEL_X: f64 = 250.0;
const LOGGER_PANEL_Y: f64 = 210.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_POCKETS: usize = 6;
const LOGGER_SLOT_X: f64 = 38.0;
const LOGGER_SLOT_Y: f64 = 86.0;
const LOGGER_SLOT_DEPTH: f64 = 11.0;
const LOGGER_SLOT_PITCH_X: f64 = 40.0;
const LOGGER_SEAL_WELLS: usize = 6;
const LOGGER_CONTACT_PINS_PER_POCKET: usize = 2;
const LOGGER_CONTACT_PINS: usize = LOGGER_POCKETS * LOGGER_CONTACT_PINS_PER_POCKET;

const COA_POS: (f64, f64) = (165.0, -285.0);
const COA_PLATE_X: f64 = 340.0;
const COA_PLATE_Y: f64 = 210.0;
const COA_PLATE_Z: f64 = 18.0;
const COA_CARD_LANDS: usize = 4;
const CERTIFICATE_LANDS: usize = 4;
const BARCODE_LANDS: usize = 8;
const RFID_LABEL_WELLS: usize = 4;
const COA_CLIP_RAILS: usize = 3;

const BULKHEAD_POS: (f64, f64) = (590.0, -285.0);
const BULKHEAD_PANEL_X: f64 = 260.0;
const BULKHEAD_PANEL_Y: f64 = 210.0;
const BULKHEAD_BASE_Z: f64 = 28.0;
const BULKHEAD_GATE_X: f64 = 224.0;
const BULKHEAD_GATE_Y: f64 = 26.0;
const BULKHEAD_GATE_Z: f64 = 168.0;
const TRANSFER_WINDOW_X: f64 = 132.0;
const TRANSFER_WINDOW_Z: f64 = 78.0;
const BULKHEAD_LATCHES: usize = 6;
const PRESSURE_EQUALIZATION_PORTS: usize = 4;
const SEALED_CONNECTOR_GLANDS: usize = 5;

const CUSTODY_POS: (f64, f64) = (100.0, 382.0);
const CUSTODY_RAIL_X: f64 = 520.0;
const CUSTODY_RAIL_Y: f64 = 96.0;
const CUSTODY_RAIL_Z: f64 = 30.0;
const CAP_WELLS: usize = 12;
const CAP_WELL_COLS: usize = 6;
const CAP_WELL_D: f64 = 18.0;
const CONNECTOR_SADDLES: usize = 8;
const CUSTODY_TOKEN_WELLS: usize = 6;

const EVIDENCE_POS: (f64, f64) = (30.0, 410.0);
const EVIDENCE_SPAN_X: f64 = 1320.0;
const EVIDENCE_POST_X: f64 = 28.0;
const EVIDENCE_POST_Y: f64 = 38.0;
const EVIDENCE_POST_Z: f64 = 218.0;
const EVIDENCE_BEAM_Y: f64 = 46.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CAMERA_PODS: usize = 5;
const EVIDENCE_LIGHT_BARS: usize = 4;
const EVIDENCE_CARD_RAILS: usize = 2;
const EVIDENCE_UNDERSIDE_CLEARANCE_Z: f64 = 190.0;

const ROBOT_FRONT_APPROACH_Y: f64 = 430.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 270.0;
const LEFT_NEST_SERVICE_X: f64 = 210.0;
const RIGHT_BULKHEAD_SERVICE_X: f64 = 230.0;
const SCALE_NO_TOUCH_CLEARANCE_Z: f64 = 128.0;
const TOP_CAMERA_SERVICE_Z: f64 = 360.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    export(&containment_deck(), OUTPUTS[0]);
    export(&media_bag_vial_nests(), OUTPUTS[1]);
    export(&barcode_rfid_scanner_bridge(), OUTPUTS[2]);
    export(&coa_certificate_land(), OUTPUTS[3]);
    export(&load_cell_reference_mass_pocket(), OUTPUTS[4]);
    export(&released_hold_reject_lanes(), OUTPUTS[5]);
    export(&expired_quarantine_pocket(), OUTPUTS[6]);
    export(&temperature_logger_pocket(), OUTPUTS[7]);
    export(&sealed_transfer_bulkhead(), OUTPUTS[8]);
    export(&cap_connector_custody(), OUTPUTS[9]);
    export(&evidence_camera_bridge(), OUTPUTS[10]);
    export(&robot_service_keepouts(), OUTPUTS[11]);
    export(&station_assembly(), OUTPUTS[12]);

    println!();
    println!("Closed reagent lot barcode reconcile scale station:");
    println!(
        "  Containment deck:        {STATION_X:.0}mm x {STATION_Y:.0}mm deck with {MOUNT_BOSSES} mount bosses and {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Material nests:          {MEDIA_BAG_NESTS} media-bag nests, {VIAL_NESTS} vial wells, {LOT_FLAG_LANDS} lot flag lands, {NEST_ORIENTATION_DOTS} orientation dots"
    );
    println!(
        "  Barcode/RFID bridge:     {BARCODE_CAMERA_PODS} barcode camera pods, {RFID_ANTENNA_LANDS} RFID antenna lands, {SCANNER_CAL_TAGS} calibration tags, clearance {:.0}mm",
        scanner_bridge_clearance()
    );
    println!(
        "  Gravimetric pocket:      {LOAD_CELL_PAD_COUNT} load-cell pads, {REFERENCE_MASS_WELLS} reference mass wells, {FINE_MASS_WELLS} fine-mass wells"
    );
    println!(
        "  COA/cert evidence:       {COA_CARD_LANDS} COA lands, {CERTIFICATE_LANDS} certificate lands, {BARCODE_LANDS} barcode lands, {RFID_LABEL_WELLS} RFID label wells"
    );
    println!(
        "  Disposition lanes:       {:?} lanes with {STATUS_SLOTS_PER_LANE} sealed-material slots per lane and {LANE_SHUTTERS} shutter lands",
        STATUS_LANE_NAMES
    );
    println!(
        "  Exception handling:      {EXPIRED_BINS} expired bins, {QUARANTINE_BINS} quarantine bins, {QUARANTINE_TOKEN_WELLS} quarantine token wells"
    );
    println!(
        "  Transfer/custody:        {LOGGER_POCKETS} temperature logger pockets, {SEALED_CONNECTOR_GLANDS} sealed transfer glands, {CAP_WELLS} cap wells, {CONNECTOR_SADDLES} connector saddles"
    );
    println!(
        "  Evidence/service:        {EVIDENCE_CAMERA_PODS} evidence cameras, {EVIDENCE_LIGHT_BARS} light bars, front robot {ROBOT_FRONT_APPROACH_Y:.0}mm, rear service {REAR_SERVICE_CLEARANCE_Y:.0}mm"
    );
    println!("  Labeled STL outputs:     {} files", OUTPUTS.len());
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + media_bag_vial_nests().translate(NEST_POS.0, NEST_POS.1, BASE_Z)
        + barcode_rfid_scanner_bridge().translate(SCANNER_POS.0, SCANNER_POS.1, BASE_Z)
        + load_cell_reference_mass_pocket().translate(SCALE_POS.0, SCALE_POS.1, BASE_Z)
        + released_hold_reject_lanes().translate(LANE_POS.0, LANE_POS.1, BASE_Z)
        + expired_quarantine_pocket().translate(QUARANTINE_POS.0, QUARANTINE_POS.1, BASE_Z)
        + temperature_logger_pocket().translate(LOGGER_POS.0, LOGGER_POS.1, BASE_Z)
        + coa_certificate_land().translate(COA_POS.0, COA_POS.1, BASE_Z)
        + sealed_transfer_bulkhead().translate(BULKHEAD_POS.0, BULKHEAD_POS.1, BASE_Z)
        + cap_connector_custody().translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z)
        + evidence_camera_bridge().translate(EVIDENCE_POS.0, EVIDENCE_POS.1, BASE_Z)
        + robot_service_keepouts()
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PART_PREFIX}_containment_deck_floor"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        format!("{PART_PREFIX}_identity_scale_washdown_sump_cut"),
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{PART_PREFIX}_front_right_deck_drain_port"),
        DRAIN_D / 2.0,
        CURB_W + 38.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 14.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain - module_socket_recesses()
        + containment_curbs()
        + deck_mount_bosses()
        + deck_zone_lands()
        + wipe_flow_ribs()
}

fn containment_curbs() -> Part {
    let z = BASE_Z + CURB_Z / 2.0;
    let front = centered_cube(
        format!("{PART_PREFIX}_front_robot_low_curb"),
        STATION_X,
        CURB_W,
        CURB_Z * 0.64,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, BASE_Z + CURB_Z * 0.32);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_evidence_service_curb"),
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, z);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_reagent_load_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_transfer_bulkhead_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn module_socket_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PART_PREFIX}_module_socket_recesses"));
    for footprint in module_footprints() {
        recesses = recesses
            + centered_cube(
                format!("{PART_PREFIX}_{}_socket_recess", footprint.name),
                footprint.x + 12.0,
                footprint.y + 12.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    recesses
}

fn deck_mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PART_PREFIX}_deck_mount_bosses"));
    for index in 0..MOUNT_BOSSES {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let row = index / 2;
        let x = side * (STATION_X / 2.0 - 82.0);
        let y = centered_index(row, MOUNT_BOSSES / 2, 240.0);
        let boss = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_boss_{index}"),
            MOUNT_BOSS_D / 2.0,
            10.0,
            40,
        )
        .translate(x, y, BASE_Z + 5.0);
        let hole = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            12.0,
            28,
        )
        .translate(x, y, BASE_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn deck_zone_lands() -> Part {
    let incoming = centered_cube(
        format!("{PART_PREFIX}_incoming_lot_identity_floor_land"),
        NEST_PANEL_X + 70.0,
        NEST_PANEL_Y + 56.0,
        3.0,
    )
    .translate(NEST_POS.0, NEST_POS.1, BASE_Z + 1.5);
    let scale = centered_cube(
        format!("{PART_PREFIX}_gravimetric_floor_land"),
        SCALE_PANEL_X + 60.0,
        SCALE_PANEL_Y + 56.0,
        3.0,
    )
    .translate(SCALE_POS.0, SCALE_POS.1, BASE_Z + 1.5);
    let disposition = centered_cube(
        format!("{PART_PREFIX}_released_hold_reject_floor_land"),
        LANE_PANEL_X + 56.0,
        LANE_PANEL_Y + 54.0,
        3.0,
    )
    .translate(LANE_POS.0, LANE_POS.1, BASE_Z + 1.5);
    let evidence = centered_cube(
        format!("{PART_PREFIX}_coa_logger_quarantine_floor_land"),
        1185.0,
        266.0,
        3.0,
    )
    .translate(-8.0, -285.0, BASE_Z + 1.5);

    incoming + scale + disposition + evidence
}

fn wipe_flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_wipe_flow_ribs"));
    for index in 0..8 {
        ribs = ribs
            + centered_cube(
                format!("{PART_PREFIX}_deck_wipe_flow_rib_{index}"),
                12.0,
                SUMP_Y - 150.0,
                5.0,
            )
            .translate(centered_index(index, 8, 146.0), -30.0, BASE_Z + 2.5);
    }
    ribs
}

fn media_bag_vial_nests() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_media_bag_vial_nest_panel"),
        NEST_PANEL_X,
        NEST_PANEL_Y,
        NEST_PANEL_Z,
    )
    .translate(0.0, 0.0, NEST_PANEL_Z / 2.0);

    let mut cuts = Part::empty(format!("{PART_PREFIX}_media_bag_vial_nest_cuts"));
    for bag in 0..MEDIA_BAG_NESTS {
        let x = bag_nest_x(bag);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_media_bag_{bag}_sealed_bag_cradle_cut"),
                MEDIA_BAG_NEST_X,
                MEDIA_BAG_NEST_Y,
                MEDIA_BAG_NEST_DEPTH + 1.0,
            )
            .translate(x, 38.0, NEST_PANEL_Z - MEDIA_BAG_NEST_DEPTH / 2.0 + 0.5)
            + centered_cube(
                format!("{PART_PREFIX}_media_bag_{bag}_barcode_face_notch"),
                MEDIA_BAG_NEST_X - 42.0,
                18.0,
                MEDIA_BAG_NEST_DEPTH + 2.0,
            )
            .translate(x, -MEDIA_BAG_NEST_Y / 2.0 - 58.0, NEST_PANEL_Z - 7.0);
    }
    for vial in 0..VIAL_NESTS {
        let (x, y) = vial_xy(vial);
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_vial_{vial:02}_nest_well_cut"),
                VIAL_WELL_D / 2.0,
                VIAL_WELL_DEPTH + 1.0,
                32,
            )
            .translate(x, y, NEST_PANEL_Z - VIAL_WELL_DEPTH / 2.0 + 0.5);
    }

    panel - cuts + bag_datum_rails() + vial_retainer_rings() + lot_flag_lands() + orientation_dots()
}

fn bag_datum_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_bag_datum_rails"));
    for bag in 0..MEDIA_BAG_NESTS {
        let x = bag_nest_x(bag);
        let left = centered_cube(
            format!("{PART_PREFIX}_bag_{bag}_left_soft_datum_rail"),
            12.0,
            MEDIA_BAG_NEST_Y + 42.0,
            BAG_DATUM_RAIL_Z,
        )
        .translate(
            x - MEDIA_BAG_NEST_X / 2.0 - 18.0,
            38.0,
            NEST_PANEL_Z + BAG_DATUM_RAIL_Z / 2.0,
        );
        let right = centered_cube(
            format!("{PART_PREFIX}_bag_{bag}_right_soft_datum_rail"),
            12.0,
            MEDIA_BAG_NEST_Y + 42.0,
            BAG_DATUM_RAIL_Z,
        )
        .translate(
            x + MEDIA_BAG_NEST_X / 2.0 + 18.0,
            38.0,
            NEST_PANEL_Z + BAG_DATUM_RAIL_Z / 2.0,
        );
        let rear = centered_cube(
            format!("{PART_PREFIX}_bag_{bag}_rear_fill_port_stop"),
            MEDIA_BAG_NEST_X + 48.0,
            12.0,
            BAG_DATUM_RAIL_Z,
        )
        .translate(
            x,
            38.0 + MEDIA_BAG_NEST_Y / 2.0 + 24.0,
            NEST_PANEL_Z + BAG_DATUM_RAIL_Z / 2.0,
        );
        rails = rails + left + right + rear;
    }
    rails
}

fn vial_retainer_rings() -> Part {
    let mut rings = Part::empty(format!("{PART_PREFIX}_vial_retainer_rings"));
    for vial in 0..VIAL_NESTS {
        let (x, y) = vial_xy(vial);
        let outer = centered_cylinder(
            format!("{PART_PREFIX}_vial_{vial:02}_retainer_ring_outer"),
            VIAL_WELL_D / 2.0 + 4.0,
            6.0,
            32,
        )
        .translate(x, y, NEST_PANEL_Z + 3.0);
        let inner = centered_cylinder(
            format!("{PART_PREFIX}_vial_{vial:02}_retainer_ring_inner_clearance"),
            VIAL_WELL_D / 2.0,
            8.0,
            32,
        )
        .translate(x, y, NEST_PANEL_Z + 3.0);
        rings = rings + (outer - inner);
    }
    rings
}

fn lot_flag_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_lot_flag_lands"));
    for index in 0..LOT_FLAG_LANDS {
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_lot_flag_land_{index}"),
                42.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(index, LOT_FLAG_LANDS, 52.0),
                -NEST_PANEL_Y / 2.0 + 28.0,
                NEST_PANEL_Z + 2.5,
            );
    }
    lands
}

fn orientation_dots() -> Part {
    let mut dots = Part::empty(format!("{PART_PREFIX}_nest_orientation_dots"));
    for index in 0..NEST_ORIENTATION_DOTS {
        let (x, y) = if index < MEDIA_BAG_NESTS {
            (
                bag_nest_x(index) - MEDIA_BAG_NEST_X / 2.0 + 18.0,
                38.0 + MEDIA_BAG_NEST_Y / 2.0 - 18.0,
            )
        } else {
            let (vx, vy) = vial_xy(index - MEDIA_BAG_NESTS);
            (vx - 15.0, vy + 15.0)
        };
        dots = dots
            + centered_cylinder(
                format!("{PART_PREFIX}_orientation_dot_{index}"),
                4.0,
                4.0,
                20,
            )
            .translate(x, y, NEST_PANEL_Z + 2.0);
    }
    dots
}

fn barcode_rfid_scanner_bridge() -> Part {
    scanner_bridge_frame()
        + barcode_pods()
        + rfid_antenna_lands()
        + scanner_calibration_tags()
        + scanner_cable_comb()
}

fn scanner_bridge_frame() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_scanner_left_post"),
        SCANNER_POST_X,
        SCANNER_POST_Y,
        SCANNER_POST_Z,
    )
    .translate(
        -SCANNER_SPAN_X / 2.0 + SCANNER_POST_X / 2.0,
        0.0,
        SCANNER_POST_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PART_PREFIX}_scanner_right_post"),
        SCANNER_POST_X,
        SCANNER_POST_Y,
        SCANNER_POST_Z,
    )
    .translate(
        SCANNER_SPAN_X / 2.0 - SCANNER_POST_X / 2.0,
        0.0,
        SCANNER_POST_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PART_PREFIX}_scanner_barcode_rfid_bridge_beam"),
        SCANNER_SPAN_X,
        SCANNER_BEAM_Y,
        SCANNER_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        SCANNER_UNDERSIDE_CLEARANCE_Z + SCANNER_BEAM_Z / 2.0,
    );
    let front_glare_baffle = centered_cube(
        format!("{PART_PREFIX}_scanner_front_anti_glare_baffle"),
        SCANNER_SPAN_X - 80.0,
        8.0,
        42.0,
    )
    .translate(
        0.0,
        -SCANNER_BEAM_Y / 2.0 - 8.0,
        SCANNER_UNDERSIDE_CLEARANCE_Z - 21.0,
    );

    left + right + beam + front_glare_baffle
}

fn barcode_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_barcode_camera_pods"));
    for index in 0..BARCODE_CAMERA_PODS {
        let x = centered_index(index, BARCODE_CAMERA_PODS, 118.0);
        let pod = centered_cube(
            format!("{PART_PREFIX}_barcode_camera_pod_{index}"),
            72.0,
            42.0,
            36.0,
        )
        .translate(
            x,
            -SCANNER_BEAM_Y / 2.0 - 28.0,
            SCANNER_UNDERSIDE_CLEARANCE_Z - 18.0,
        );
        let aperture = centered_cube(
            format!("{PART_PREFIX}_barcode_camera_pod_{index}_view_aperture"),
            42.0,
            9.0,
            20.0,
        )
        .translate(
            x,
            -SCANNER_BEAM_Y / 2.0 - 49.0,
            SCANNER_UNDERSIDE_CLEARANCE_Z - 18.0,
        );
        pods = pods + (pod - aperture);
    }
    pods
}

fn rfid_antenna_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_rfid_antenna_lands"));
    for index in 0..RFID_ANTENNA_LANDS {
        let x = centered_index(index, RFID_ANTENNA_LANDS, 104.0);
        lands = lands
            + rectangular_frame(
                &format!("{PART_PREFIX}_rfid_antenna_loop_{index}"),
                82.0,
                42.0,
                6.0,
                7.0,
            )
            .translate(
                x,
                SCANNER_BEAM_Y / 2.0 + 18.0,
                SCANNER_UNDERSIDE_CLEARANCE_Z - 8.0,
            );
    }
    lands
}

fn scanner_calibration_tags() -> Part {
    let mut tags = Part::empty(format!("{PART_PREFIX}_scanner_calibration_tags"));
    for index in 0..SCANNER_CAL_TAGS {
        let x = centered_index(index, SCANNER_CAL_TAGS, 78.0);
        tags = tags
            + centered_cylinder(
                format!("{PART_PREFIX}_scanner_reference_tag_{index}"),
                8.0,
                5.0,
                28,
            )
            .translate(x, 0.0, SCANNER_UNDERSIDE_CLEARANCE_Z - 46.0);
    }
    tags
}

fn scanner_cable_comb() -> Part {
    let comb = centered_cube(
        format!("{PART_PREFIX}_scanner_rear_cable_comb"),
        SCANNER_SPAN_X - 100.0,
        18.0,
        18.0,
    )
    .translate(
        0.0,
        SCANNER_BEAM_Y / 2.0 + 34.0,
        SCANNER_UNDERSIDE_CLEARANCE_Z + 8.0,
    );
    let mut cuts = Part::empty(format!("{PART_PREFIX}_scanner_rear_cable_comb_cuts"));
    for index in 0..(BARCODE_CAMERA_PODS + RFID_ANTENNA_LANDS) {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_scanner_cable_channel_{index}"),
                3.2,
                34.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, BARCODE_CAMERA_PODS + RFID_ANTENNA_LANDS, 54.0),
                SCANNER_BEAM_Y / 2.0 + 34.0,
                SCANNER_UNDERSIDE_CLEARANCE_Z + 8.0,
            );
    }
    comb - cuts
}

fn load_cell_reference_mass_pocket() -> Part {
    let base = centered_cube(
        format!("{PART_PREFIX}_load_cell_reference_mass_pocket_base"),
        SCALE_PANEL_X,
        SCALE_PANEL_Y,
        SCALE_PANEL_Z,
    )
    .translate(0.0, 0.0, SCALE_PANEL_Z / 2.0);
    let base = base - flexure_relief_cuts() - reference_mass_well_cuts() - fine_mass_well_cuts();

    base + load_cell_pads()
        + scale_platter()
        + reference_mass_custody_posts()
        + load_cell_cable_exit()
}

fn flexure_relief_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_load_cell_flexure_relief_cuts"));
    for index in 0..FLEXURE_RELIEF_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_load_cell_flexure_relief_slot_{index}"),
                12.0,
                112.0,
                10.0,
            )
            .translate(
                centered_index(index, FLEXURE_RELIEF_SLOTS, 34.0),
                40.0,
                SCALE_PANEL_Z - 5.0,
            );
    }
    cuts
}

fn reference_mass_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_reference_mass_well_cuts"));
    for index in 0..REFERENCE_MASS_WELLS {
        let (x, y) = mass_well_xy(index, REFERENCE_MASS_COLS, MASS_PITCH_X, MASS_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_reference_mass_well_cut_{index}"),
                MASS_WELL_D / 2.0,
                MASS_WELL_DEPTH + 1.0,
                36,
            )
            .translate(x, y - 86.0, SCALE_PANEL_Z - MASS_WELL_DEPTH / 2.0 + 0.5);
    }
    cuts
}

fn fine_mass_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_fine_reference_mass_well_cuts"));
    for index in 0..FINE_MASS_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_fine_reference_mass_well_cut_{index}"),
                9.0,
                10.0,
                28,
            )
            .translate(
                centered_index(index, FINE_MASS_WELLS, 34.0),
                116.0,
                SCALE_PANEL_Z - 5.0,
            );
    }
    cuts
}

fn load_cell_pads() -> Part {
    let mut pads = Part::empty(format!("{PART_PREFIX}_load_cell_pads"));
    for index in 0..LOAD_CELL_PAD_COUNT {
        let (x, y) = load_cell_pad_xy(index);
        pads = pads
            + centered_cube(
                format!("{PART_PREFIX}_load_cell_pad_{index}"),
                LOAD_CELL_PAD_X,
                LOAD_CELL_PAD_Y,
                LOAD_CELL_PAD_Z,
            )
            .translate(x, y, SCALE_PANEL_Z + LOAD_CELL_PAD_Z / 2.0);
    }
    pads
}

fn scale_platter() -> Part {
    let platter = centered_cube(
        format!("{PART_PREFIX}_isolated_scale_platter"),
        SCALE_PLATTER_X,
        SCALE_PLATTER_Y,
        SCALE_PLATTER_Z,
    )
    .translate(
        0.0,
        42.0,
        SCALE_PANEL_Z + LOAD_CELL_PAD_Z + SCALE_PLATTER_Z / 2.0,
    );
    let bag_shadow = centered_cube(
        format!("{PART_PREFIX}_sealed_bag_mass_shadow_land"),
        SCALE_PLATTER_X - 34.0,
        SCALE_PLATTER_Y - 34.0,
        4.0,
    )
    .translate(
        0.0,
        42.0,
        SCALE_PANEL_Z + LOAD_CELL_PAD_Z + SCALE_PLATTER_Z + 2.0,
    );
    platter + bag_shadow
}

fn reference_mass_custody_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_reference_mass_custody_posts"));
    for index in 0..REFERENCE_MASS_WELLS {
        let (x, y) = mass_well_xy(index, REFERENCE_MASS_COLS, MASS_PITCH_X, MASS_PITCH_Y);
        posts = posts
            + centered_cylinder(
                format!("{PART_PREFIX}_reference_mass_custody_post_{index}"),
                3.5,
                8.0,
                18,
            )
            .translate(x + 22.0, y - 86.0, SCALE_PANEL_Z + 4.0);
    }
    posts
}

fn load_cell_cable_exit() -> Part {
    let gland = centered_cube(
        format!("{PART_PREFIX}_load_cell_cable_exit_gland"),
        68.0,
        22.0,
        18.0,
    )
    .translate(0.0, SCALE_PANEL_Y / 2.0 + 14.0, SCALE_PANEL_Z - 6.0);
    let cut = centered_cylinder(
        format!("{PART_PREFIX}_load_cell_cable_exit_round_clearance"),
        4.4,
        76.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, SCALE_PANEL_Y / 2.0 + 14.0, SCALE_PANEL_Z - 6.0);
    gland - cut
}

fn released_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_released_hold_reject_lane_panel"),
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_released_hold_reject_slot_cuts"));
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "{PART_PREFIX}_{}_sealed_material_slot_{slot}",
                        STATUS_LANE_NAMES[lane]
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_SLOT_DEPTH + 1.0,
                )
                .translate(
                    status_lane_x(lane),
                    status_slot_y(slot),
                    LANE_PANEL_Z - STATUS_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    panel - cuts + lane_dividers() + lane_shutter_lands() + lane_status_tokens()
}

fn lane_dividers() -> Part {
    let mut dividers = Part::empty(format!("{PART_PREFIX}_lane_dividers"));
    for index in 0..(STATUS_LANES - 1) {
        dividers = dividers
            + centered_cube(
                format!("{PART_PREFIX}_disposition_lane_divider_{index}"),
                LANE_DIVIDER_W,
                LANE_PANEL_Y - 34.0,
                34.0,
            )
            .translate(
                centered_index(index, STATUS_LANES - 1, STATUS_LANE_PITCH_X)
                    + STATUS_LANE_PITCH_X / 2.0,
                0.0,
                LANE_PANEL_Z + 17.0,
            );
    }
    let front_stop = centered_cube(
        format!("{PART_PREFIX}_disposition_front_interlock_stop"),
        LANE_PANEL_X - 32.0,
        12.0,
        26.0,
    )
    .translate(0.0, -LANE_PANEL_Y / 2.0 + 26.0, LANE_PANEL_Z + 13.0);
    let rear_stop = centered_cube(
        format!("{PART_PREFIX}_disposition_rear_release_datum_stop"),
        LANE_PANEL_X - 32.0,
        12.0,
        26.0,
    )
    .translate(0.0, LANE_PANEL_Y / 2.0 - 26.0, LANE_PANEL_Z + 13.0);
    dividers + front_stop + rear_stop
}

fn lane_shutter_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_lane_shutter_lands"));
    for lane in 0..LANE_SHUTTERS {
        lands = lands
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_lane_shutter_land",
                    STATUS_LANE_NAMES[lane]
                ),
                104.0,
                20.0,
                16.0,
            )
            .translate(
                status_lane_x(lane),
                LANE_PANEL_Y / 2.0 + 15.0,
                LANE_PANEL_Z + 8.0,
            );
    }
    lands
}

fn lane_status_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PART_PREFIX}_lane_status_token_posts"));
    for lane in 0..STATUS_LANES {
        for token in 0..3 {
            tokens = tokens
                + centered_cylinder(
                    format!("{PART_PREFIX}_lane_{lane}_status_token_post_{token}"),
                    4.5,
                    8.0,
                    22,
                )
                .translate(
                    status_lane_x(lane) + centered_index(token, 3, 24.0),
                    -LANE_PANEL_Y / 2.0 + 48.0,
                    LANE_PANEL_Z + 4.0,
                );
        }
    }
    tokens
}

fn expired_quarantine_pocket() -> Part {
    let tray = centered_cube(
        format!("{PART_PREFIX}_expired_quarantine_tray_body"),
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0);
    let tray = tray - quarantine_bin_cuts();
    tray + quarantine_walls() + quarantine_token_wells() + tamper_seal_posts()
}

fn quarantine_bin_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_expired_quarantine_bin_cuts"));
    let total_bins = EXPIRED_BINS + QUARANTINE_BINS;
    for index in 0..total_bins {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_expired_quarantine_bin_{index}_pocket_cut"),
                126.0,
                70.0,
                24.0,
            )
            .translate(
                centered_index(index % 2, 2, 154.0),
                centered_index(index / 2, 2, 86.0),
                QUARANTINE_Z - 12.0,
            );
    }
    cuts
}

fn quarantine_walls() -> Part {
    let front = centered_cube(
        format!("{PART_PREFIX}_expired_quarantine_front_high_wall"),
        QUARANTINE_X,
        QUARANTINE_WALL,
        42.0,
    )
    .translate(
        0.0,
        -QUARANTINE_Y / 2.0 + QUARANTINE_WALL / 2.0,
        QUARANTINE_Z + 21.0,
    );
    let rear = centered_cube(
        format!("{PART_PREFIX}_expired_quarantine_rear_high_wall"),
        QUARANTINE_X,
        QUARANTINE_WALL,
        42.0,
    )
    .translate(
        0.0,
        QUARANTINE_Y / 2.0 - QUARANTINE_WALL / 2.0,
        QUARANTINE_Z + 21.0,
    );
    let center_divider = centered_cube(
        format!("{PART_PREFIX}_expired_quarantine_center_divider"),
        QUARANTINE_WALL,
        QUARANTINE_Y - 34.0,
        46.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z + 23.0);
    front + rear + center_divider
}

fn quarantine_token_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_quarantine_token_wells"));
    for index in 0..QUARANTINE_TOKEN_WELLS {
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_quarantine_token_ring_{index}"),
            8.0,
            6.0,
            28,
        )
        .translate(
            centered_index(index, QUARANTINE_TOKEN_WELLS, 34.0),
            QUARANTINE_Y / 2.0 + 18.0,
            QUARANTINE_Z + 3.0,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_quarantine_token_recess_{index}"),
            4.6,
            8.0,
            28,
        )
        .translate(
            centered_index(index, QUARANTINE_TOKEN_WELLS, 34.0),
            QUARANTINE_Y / 2.0 + 18.0,
            QUARANTINE_Z + 3.0,
        );
        wells = wells + (ring - cut);
    }
    wells
}

fn tamper_seal_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_quarantine_tamper_seal_posts"));
    for index in 0..TAMPER_SEAL_POSTS {
        posts = posts
            + centered_cylinder(
                format!("{PART_PREFIX}_quarantine_tamper_seal_post_{index}"),
                5.0,
                12.0,
                24,
            )
            .translate(
                centered_index(index, TAMPER_SEAL_POSTS, 48.0),
                -QUARANTINE_Y / 2.0 - 16.0,
                QUARANTINE_Z + 6.0,
            );
    }
    posts
}

fn temperature_logger_pocket() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_temperature_logger_pocket_panel"),
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);
    let panel = panel - logger_slot_cuts();
    panel + logger_cable_comb() + logger_seal_wells() + logger_contact_pin_lands()
}

fn logger_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_temperature_logger_slot_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_temperature_logger_{index}_slot_cut"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH + 1.0,
            )
            .translate(
                centered_index(index, LOGGER_POCKETS, LOGGER_SLOT_PITCH_X),
                -10.0,
                LOGGER_PANEL_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    cuts
}

fn logger_cable_comb() -> Part {
    let comb = centered_cube(
        format!("{PART_PREFIX}_temperature_logger_cable_comb"),
        LOGGER_PANEL_X - 28.0,
        20.0,
        16.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 + 15.0, LOGGER_PANEL_Z - 6.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_temperature_logger_cable_comb_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_temperature_logger_cable_channel_{index}"),
                2.8,
                40.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, LOGGER_POCKETS, LOGGER_SLOT_PITCH_X),
                LOGGER_PANEL_Y / 2.0 + 15.0,
                LOGGER_PANEL_Z - 6.0,
            );
    }
    comb - cuts
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_temperature_logger_seal_wells"));
    for index in 0..LOGGER_SEAL_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PART_PREFIX}_temperature_logger_seal_well_{index}"),
                5.5,
                6.0,
                24,
            )
            .translate(
                centered_index(index, LOGGER_SEAL_WELLS, 34.0),
                -LOGGER_PANEL_Y / 2.0 + 24.0,
                LOGGER_PANEL_Z + 3.0,
            );
    }
    wells
}

fn logger_contact_pin_lands() -> Part {
    let mut pins = Part::empty(format!(
        "{PART_PREFIX}_temperature_logger_contact_pin_lands"
    ));
    for pocket in 0..LOGGER_POCKETS {
        for pin in 0..LOGGER_CONTACT_PINS_PER_POCKET {
            pins = pins
                + centered_cylinder(
                    format!("{PART_PREFIX}_temperature_logger_{pocket}_contact_pin_{pin}"),
                    2.0,
                    3.0,
                    16,
                )
                .translate(
                    centered_index(pocket, LOGGER_POCKETS, LOGGER_SLOT_PITCH_X)
                        + centered_index(pin, LOGGER_CONTACT_PINS_PER_POCKET, 8.0),
                    52.0,
                    LOGGER_PANEL_Z + 1.5,
                );
        }
    }
    pins
}

fn coa_certificate_land() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_coa_certificate_land_plate"),
        COA_PLATE_X,
        COA_PLATE_Y,
        COA_PLATE_Z,
    )
    .translate(0.0, 0.0, COA_PLATE_Z / 2.0);
    let plate = plate - coa_card_cuts() - certificate_land_cuts() - rfid_label_well_cuts();
    plate + barcode_lands() + coa_clip_rails()
}

fn coa_card_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_coa_card_land_cuts"));
    for index in 0..COA_CARD_LANDS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_coa_card_land_cut_{index}"),
                66.0,
                42.0,
                6.0,
            )
            .translate(
                centered_index(index, COA_CARD_LANDS, 76.0),
                38.0,
                COA_PLATE_Z - 3.0,
            );
    }
    cuts
}

fn certificate_land_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_certificate_land_cuts"));
    for index in 0..CERTIFICATE_LANDS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_certificate_land_cut_{index}"),
                62.0,
                30.0,
                5.0,
            )
            .translate(
                centered_index(index, CERTIFICATE_LANDS, 74.0),
                -4.0,
                COA_PLATE_Z - 2.5,
            );
    }
    cuts
}

fn rfid_label_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_rfid_label_well_cuts"));
    for index in 0..RFID_LABEL_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_rfid_label_well_cut_{index}"),
                7.0,
                7.0,
                24,
            )
            .translate(
                centered_index(index, RFID_LABEL_WELLS, 54.0),
                COA_PLATE_Y / 2.0 - 28.0,
                COA_PLATE_Z - 3.0,
            );
    }
    cuts
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_coa_barcode_lands"));
    for index in 0..BARCODE_LANDS {
        let row = index / 4;
        let col = index % 4;
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_coa_barcode_land_{index}"),
                58.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 70.0),
                -COA_PLATE_Y / 2.0 + 30.0 + row as f64 * 30.0,
                COA_PLATE_Z + 2.0,
            );
    }
    lands
}

fn coa_clip_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_coa_clip_rails"));
    for index in 0..COA_CLIP_RAILS {
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_coa_clip_rail_{index}"),
                COA_PLATE_X - 54.0,
                5.0,
                9.0,
            )
            .translate(
                0.0,
                centered_index(index, COA_CLIP_RAILS, 54.0),
                COA_PLATE_Z + 4.5,
            );
    }
    rails
}

fn sealed_transfer_bulkhead() -> Part {
    let base = centered_cube(
        format!("{PART_PREFIX}_sealed_transfer_bulkhead_base"),
        BULKHEAD_PANEL_X,
        BULKHEAD_PANEL_Y,
        BULKHEAD_BASE_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_BASE_Z / 2.0);
    let gate = centered_cube(
        format!("{PART_PREFIX}_sealed_transfer_bulkhead_gate_plate"),
        BULKHEAD_GATE_X,
        BULKHEAD_GATE_Y,
        BULKHEAD_GATE_Z,
    )
    .translate(0.0, 18.0, BULKHEAD_BASE_Z + BULKHEAD_GATE_Z / 2.0);
    let pass_window = centered_cube(
        format!("{PART_PREFIX}_sealed_transfer_bulkhead_pass_window_cut"),
        TRANSFER_WINDOW_X,
        BULKHEAD_GATE_Y + 2.0,
        TRANSFER_WINDOW_Z,
    )
    .translate(0.0, 18.0, BULKHEAD_BASE_Z + BULKHEAD_GATE_Z * 0.52);

    base + (gate - pass_window)
        + transfer_latch_lands()
        + pressure_equalization_ports()
        + sealed_connector_glands()
        + bulkhead_gasket_lands()
}

fn transfer_latch_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_transfer_latch_lands"));
    for index in 0..BULKHEAD_LATCHES {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let row = index / 2;
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_sealed_transfer_latch_land_{index}"),
                22.0,
                10.0,
                24.0,
            )
            .translate(
                side * (BULKHEAD_GATE_X / 2.0 - 24.0),
                0.0,
                BULKHEAD_BASE_Z + 42.0 + row as f64 * 42.0,
            );
    }
    lands
}

fn pressure_equalization_ports() -> Part {
    let mut ports = Part::empty(format!("{PART_PREFIX}_pressure_equalization_ports"));
    for index in 0..PRESSURE_EQUALIZATION_PORTS {
        ports = ports
            + centered_cylinder(
                format!("{PART_PREFIX}_pressure_equalization_port_{index}"),
                6.0,
                10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, PRESSURE_EQUALIZATION_PORTS, 32.0),
                BULKHEAD_GATE_Y / 2.0 + 24.0,
                BULKHEAD_BASE_Z + BULKHEAD_GATE_Z - 30.0,
            );
    }
    ports
}

fn sealed_connector_glands() -> Part {
    let mut glands = Part::empty(format!("{PART_PREFIX}_sealed_connector_glands"));
    for index in 0..SEALED_CONNECTOR_GLANDS {
        let gland = centered_cylinder(
            format!("{PART_PREFIX}_sealed_connector_gland_outer_{index}"),
            11.0,
            14.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, SEALED_CONNECTOR_GLANDS, 38.0),
            -BULKHEAD_GATE_Y / 2.0 - 30.0,
            BULKHEAD_BASE_Z + 34.0,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_sealed_connector_gland_clearance_{index}"),
            5.2,
            16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, SEALED_CONNECTOR_GLANDS, 38.0),
            -BULKHEAD_GATE_Y / 2.0 - 30.0,
            BULKHEAD_BASE_Z + 34.0,
        );
        glands = glands + (gland - cut);
    }
    glands
}

fn bulkhead_gasket_lands() -> Part {
    let outer = rectangular_frame(
        &format!("{PART_PREFIX}_bulkhead_outer_gasket_land"),
        BULKHEAD_GATE_X - 36.0,
        BULKHEAD_GATE_Z - 30.0,
        7.0,
        6.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        0.0,
        -BULKHEAD_GATE_Y / 2.0 - 4.0,
        BULKHEAD_BASE_Z + BULKHEAD_GATE_Z / 2.0,
    );
    let inner = rectangular_frame(
        &format!("{PART_PREFIX}_bulkhead_inner_gasket_land"),
        TRANSFER_WINDOW_X + 32.0,
        TRANSFER_WINDOW_Z + 28.0,
        5.0,
        5.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        0.0,
        -BULKHEAD_GATE_Y / 2.0 - 8.0,
        BULKHEAD_BASE_Z + BULKHEAD_GATE_Z * 0.52,
    );
    outer + inner
}

fn cap_connector_custody() -> Part {
    let rail = centered_cube(
        format!("{PART_PREFIX}_cap_connector_custody_rail"),
        CUSTODY_RAIL_X,
        CUSTODY_RAIL_Y,
        CUSTODY_RAIL_Z,
    )
    .translate(0.0, 0.0, CUSTODY_RAIL_Z / 2.0);
    let rail = rail - cap_well_cuts();
    rail + connector_saddles() + custody_token_wells() + custody_guard_rails()
}

fn cap_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_cap_well_cuts"));
    for index in 0..CAP_WELLS {
        let row = index / CAP_WELL_COLS;
        let col = index % CAP_WELL_COLS;
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_cap_well_cut_{index}"),
                CAP_WELL_D / 2.0,
                10.0,
                28,
            )
            .translate(
                centered_index(col, CAP_WELL_COLS, 36.0) - 126.0,
                centered_index(row, 2, 36.0),
                CUSTODY_RAIL_Z - 5.0,
            );
    }
    cuts
}

fn connector_saddles() -> Part {
    let mut saddles = Part::empty(format!("{PART_PREFIX}_connector_custody_saddles"));
    for index in 0..CONNECTOR_SADDLES {
        let saddle = centered_cube(
            format!("{PART_PREFIX}_connector_saddle_{index}"),
            30.0,
            18.0,
            14.0,
        )
        .translate(
            70.0 + centered_index(index, CONNECTOR_SADDLES, 40.0),
            0.0,
            CUSTODY_RAIL_Z + 7.0,
        );
        let tube_cut = centered_cylinder(
            format!("{PART_PREFIX}_connector_saddle_{index}_tube_cut"),
            4.0,
            36.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            70.0 + centered_index(index, CONNECTOR_SADDLES, 40.0),
            0.0,
            CUSTODY_RAIL_Z + 8.0,
        );
        saddles = saddles + (saddle - tube_cut);
    }
    saddles
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_cap_connector_custody_token_wells"));
    for index in 0..CUSTODY_TOKEN_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{PART_PREFIX}_custody_token_well_{index}"),
                6.0,
                6.0,
                24,
            )
            .translate(
                centered_index(index, CUSTODY_TOKEN_WELLS, 34.0),
                CUSTODY_RAIL_Y / 2.0 + 15.0,
                CUSTODY_RAIL_Z + 3.0,
            );
    }
    wells
}

fn custody_guard_rails() -> Part {
    let front = centered_cube(
        format!("{PART_PREFIX}_cap_connector_front_guard_rail"),
        CUSTODY_RAIL_X - 30.0,
        8.0,
        16.0,
    )
    .translate(0.0, -CUSTODY_RAIL_Y / 2.0 + 8.0, CUSTODY_RAIL_Z + 8.0);
    let rear = centered_cube(
        format!("{PART_PREFIX}_cap_connector_rear_guard_rail"),
        CUSTODY_RAIL_X - 30.0,
        8.0,
        16.0,
    )
    .translate(0.0, CUSTODY_RAIL_Y / 2.0 - 8.0, CUSTODY_RAIL_Z + 8.0);
    front + rear
}

fn evidence_camera_bridge() -> Part {
    evidence_bridge_frame() + evidence_camera_pods() + evidence_light_bars() + evidence_card_rails()
}

fn evidence_bridge_frame() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_left_post"),
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        -EVIDENCE_SPAN_X / 2.0 + EVIDENCE_POST_X / 2.0,
        0.0,
        EVIDENCE_POST_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_right_post"),
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_SPAN_X / 2.0 - EVIDENCE_POST_X / 2.0,
        0.0,
        EVIDENCE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_beam"),
        EVIDENCE_SPAN_X,
        EVIDENCE_BEAM_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        EVIDENCE_UNDERSIDE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    left + right + beam
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_evidence_camera_pods"));
    for index in 0..EVIDENCE_CAMERA_PODS {
        let x = centered_index(index, EVIDENCE_CAMERA_PODS, 230.0);
        let pod = centered_cube(
            format!("{PART_PREFIX}_evidence_camera_pod_{index}"),
            78.0,
            42.0,
            38.0,
        )
        .translate(
            x,
            -EVIDENCE_BEAM_Y / 2.0 - 24.0,
            EVIDENCE_UNDERSIDE_CLEARANCE_Z - 19.0,
        );
        let aperture = centered_cube(
            format!("{PART_PREFIX}_evidence_camera_pod_{index}_aperture"),
            36.0,
            8.0,
            20.0,
        )
        .translate(
            x,
            -EVIDENCE_BEAM_Y / 2.0 - 45.0,
            EVIDENCE_UNDERSIDE_CLEARANCE_Z - 19.0,
        );
        pods = pods + (pod - aperture);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty(format!("{PART_PREFIX}_evidence_light_bars"));
    for index in 0..EVIDENCE_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{PART_PREFIX}_evidence_light_bar_{index}"),
                180.0,
                10.0,
                12.0,
            )
            .translate(
                centered_index(index, EVIDENCE_LIGHT_BARS, 270.0),
                EVIDENCE_BEAM_Y / 2.0 + 16.0,
                EVIDENCE_UNDERSIDE_CLEARANCE_Z - 10.0,
            );
    }
    bars
}

fn evidence_card_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_evidence_card_rails"));
    for index in 0..EVIDENCE_CARD_RAILS {
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_evidence_card_rail_{index}"),
                420.0,
                8.0,
                16.0,
            )
            .translate(
                0.0,
                EVIDENCE_BEAM_Y / 2.0 + 42.0 + index as f64 * 20.0,
                EVIDENCE_UNDERSIDE_CLEARANCE_Z + 8.0,
            );
    }
    rails
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        format!("{PART_PREFIX}_front_robot_approach_keepout_gauge"),
        STATION_X - 180.0,
        ROBOT_FRONT_APPROACH_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - ROBOT_FRONT_APPROACH_Y / 2.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_service = centered_cube(
        format!("{PART_PREFIX}_rear_evidence_service_keepout_gauge"),
        STATION_X - 200.0,
        REAR_SERVICE_CLEARANCE_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE_Y / 2.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left_service = centered_cube(
        format!("{PART_PREFIX}_left_reagent_load_service_keepout_gauge"),
        LEFT_NEST_SERVICE_X,
        STATION_Y - 210.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_NEST_SERVICE_X / 2.0,
        0.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right_service = centered_cube(
        format!("{PART_PREFIX}_right_bulkhead_service_keepout_gauge"),
        RIGHT_BULKHEAD_SERVICE_X,
        STATION_Y - 220.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_BULKHEAD_SERVICE_X / 2.0,
        0.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let scale_no_touch = centered_cube(
        format!("{PART_PREFIX}_scale_no_touch_vertical_keepout_gauge"),
        SCALE_PANEL_X - 60.0,
        SCALE_PANEL_Y - 60.0,
        SCALE_NO_TOUCH_CLEARANCE_Z,
    )
    .translate(
        SCALE_POS.0,
        SCALE_POS.1,
        BASE_Z + SCALE_PANEL_Z + SCALE_NO_TOUCH_CLEARANCE_Z / 2.0,
    );
    let top_camera_service = centered_cube(
        format!("{PART_PREFIX}_top_camera_bridge_service_keepout_gauge"),
        EVIDENCE_SPAN_X,
        110.0,
        10.0,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        BASE_Z + TOP_CAMERA_SERVICE_Z,
    );

    front_robot + rear_service + left_service + right_service + scale_no_touch + top_camera_service
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn module_footprints() -> [Footprint; 8] {
    [
        footprint("media_bag_vial_nests", NEST_POS, NEST_PANEL_X, NEST_PANEL_Y),
        footprint(
            "load_cell_reference_mass_pocket",
            SCALE_POS,
            SCALE_PANEL_X,
            SCALE_PANEL_Y,
        ),
        footprint(
            "released_hold_reject_lanes",
            LANE_POS,
            LANE_PANEL_X,
            LANE_PANEL_Y,
        ),
        footprint(
            "expired_quarantine_pocket",
            QUARANTINE_POS,
            QUARANTINE_X,
            QUARANTINE_Y,
        ),
        footprint(
            "temperature_logger_pocket",
            LOGGER_POS,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        ),
        footprint("coa_certificate_land", COA_POS, COA_PLATE_X, COA_PLATE_Y),
        footprint(
            "sealed_transfer_bulkhead",
            BULKHEAD_POS,
            BULKHEAD_PANEL_X,
            BULKHEAD_PANEL_Y,
        ),
        footprint(
            "cap_connector_custody",
            CUSTODY_POS,
            CUSTODY_RAIL_X,
            CUSTODY_RAIL_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_inside_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - CURB_W - 24.0;
    let usable_y = STATION_Y / 2.0 - CURB_W - 24.0;
    footprint.center.0 - footprint.x / 2.0 >= -usable_x
        && footprint.center.0 + footprint.x / 2.0 <= usable_x
        && footprint.center.1 - footprint.y / 2.0 >= -usable_y
        && footprint.center.1 + footprint.y / 2.0 <= usable_y
}

fn overlaps(first: Footprint, second: Footprint) -> bool {
    let dx = (first.center.0 - second.center.0).abs();
    let dy = (first.center.1 - second.center.1).abs();
    dx < (first.x + second.x) / 2.0 && dy < (first.y + second.y) / 2.0
}

fn bag_nest_x(index: usize) -> f64 {
    centered_index(index, MEDIA_BAG_NESTS, BAG_NEST_PITCH_X)
}

fn vial_xy(index: usize) -> (f64, f64) {
    let row = index / VIAL_NEST_COLS;
    let col = index % VIAL_NEST_COLS;
    (
        centered_index(col, VIAL_NEST_COLS, VIAL_PITCH_X),
        -NEST_PANEL_Y / 2.0 + 76.0 + row as f64 * VIAL_PITCH_Y,
    )
}

fn mass_well_xy(index: usize, cols: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let rows = REFERENCE_MASS_WELLS.div_ceil(cols);
    let row = index / cols;
    let col = index % cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn load_cell_pad_xy(index: usize) -> (f64, f64) {
    let side_x = if index % 2 == 0 { -1.0 } else { 1.0 };
    let side_y = if index < 2 { -1.0 } else { 1.0 };
    (side_x * 98.0, 42.0 + side_y * 58.0)
}

fn status_lane_x(index: usize) -> f64 {
    centered_index(index, STATUS_LANES, STATUS_LANE_PITCH_X)
}

fn status_slot_y(index: usize) -> f64 {
    centered_index(index, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y)
}

fn scanner_bridge_clearance() -> f64 {
    SCANNER_UNDERSIDE_CLEARANCE_Z - NEST_PANEL_Z
}

fn evidence_bridge_clearance() -> f64 {
    EVIDENCE_UNDERSIDE_CLEARANCE_Z - CUSTODY_RAIL_Z
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(MEDIA_BAG_NESTS, 2);
    assert_eq!(VIAL_NESTS, 12);
    assert_eq!(NEST_ORIENTATION_DOTS, MEDIA_BAG_NESTS + VIAL_NESTS);
    assert_eq!(STATUS_LANE_NAMES, ["released", "hold", "reject"]);
    assert_eq!(STATUS_LANES, 3);
    assert_eq!(
        LOGGER_CONTACT_PINS,
        LOGGER_POCKETS * LOGGER_CONTACT_PINS_PER_POCKET
    );
    assert_eq!(CAP_WELLS, CAP_WELL_COLS * 2);
    assert_eq!(EXPIRED_BINS + QUARANTINE_BINS, 4);
    assert!(LANE_MIN_GAP >= 34.0);
    assert!(TRANSFER_WINDOW_X < BULKHEAD_GATE_X);
    assert!(TRANSFER_WINDOW_Z < BULKHEAD_GATE_Z);
    assert!(scanner_bridge_clearance() >= 100.0);
    assert!(evidence_bridge_clearance() >= 150.0);
    assert!(TOP_CAMERA_SERVICE_Z > EVIDENCE_UNDERSIDE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 120.0);
    assert!(SCALE_NO_TOUCH_CLEARANCE_Z > SCALE_PLATTER_Z + LOAD_CELL_PAD_Z + 80.0);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.ends_with(".stl"), "{path}");
    }

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            fits_inside_station(footprint),
            "{} exceeds station footprint",
            footprint.name
        );
    }
    for (index, first) in footprints.iter().enumerate() {
        for second in footprints.iter().skip(index + 1) {
            assert!(
                !overlaps(*first, *second),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        assert_layout();
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS[1].contains("media_bag_vial_nests"));
        assert!(OUTPUTS[2].contains("barcode_rfid_scanner_bridge"));
        assert!(OUTPUTS[4].contains("load_cell_reference_mass_pocket"));
        assert!(OUTPUTS[5].contains("released_hold_reject_lanes"));
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_user_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for feature in [
            "media_bag_vial_nests",
            "barcode_rfid_scanner_bridge",
            "coa_certificate_land",
            "load_cell_reference_mass_pocket",
            "released_hold_reject_lanes",
            "expired_quarantine_pocket",
            "temperature_logger_pocket",
            "sealed_transfer_bulkhead",
            "cap_connector_custody",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn deck_modules_fit_without_plan_overlap() {
        let footprints = module_footprints();
        for footprint in footprints {
            assert!(fits_inside_station(footprint), "{}", footprint.name);
        }
        for (index, first) in footprints.iter().enumerate() {
            for second in footprints.iter().skip(index + 1) {
                assert!(
                    !overlaps(*first, *second),
                    "{} overlaps {}",
                    first.name,
                    second.name
                );
            }
        }
    }

    #[test]
    fn lot_identity_and_evidence_counts_are_explicit() {
        assert_eq!(MEDIA_BAG_NESTS, 2);
        assert_eq!(VIAL_NESTS, 12);
        assert_eq!(BARCODE_CAMERA_PODS, 3);
        assert_eq!(RFID_ANTENNA_LANDS, 4);
        assert_eq!(SCANNER_CAL_TAGS, 4);
        assert_eq!(COA_CARD_LANDS, 4);
        assert_eq!(CERTIFICATE_LANDS, 4);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(RFID_LABEL_WELLS, 4);
    }

    #[test]
    fn gravimetric_and_logger_custody_is_parametric() {
        assert_eq!(LOAD_CELL_PAD_COUNT, 4);
        assert_eq!(REFERENCE_MASS_WELLS, 8);
        assert_eq!(FINE_MASS_WELLS, 6);
        assert_eq!(LOGGER_POCKETS, 6);
        assert_eq!(LOGGER_CONTACT_PINS, 12);
        assert!(MASS_WELL_D + 18.0 < MASS_PITCH_X);
        assert!(SCALE_NO_TOUCH_CLEARANCE_Z > SCALE_PLATTER_Z + LOAD_CELL_PAD_Z);
    }

    #[test]
    fn disposition_quarantine_and_transfer_are_segregated() {
        assert_eq!(STATUS_LANE_NAMES, ["released", "hold", "reject"]);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 12);
        assert_eq!(EXPIRED_BINS, 2);
        assert_eq!(QUARANTINE_BINS, 2);
        assert_eq!(CAP_WELLS, 12);
        assert_eq!(CONNECTOR_SADDLES, 8);
        assert_eq!(SEALED_CONNECTOR_GLANDS, 5);
        assert_eq!(PRESSURE_EQUALIZATION_PORTS, 4);
        assert!(TRANSFER_WINDOW_X < BULKHEAD_GATE_X);
        assert!(TRANSFER_WINDOW_Z < BULKHEAD_GATE_Z);
    }

    #[test]
    fn bridge_and_keepout_clearances_are_declared() {
        assert!(scanner_bridge_clearance() >= 100.0);
        assert!(evidence_bridge_clearance() >= 150.0);
        assert_eq!(EVIDENCE_CAMERA_PODS, 5);
        assert_eq!(EVIDENCE_LIGHT_BARS, 4);
        assert!(ROBOT_FRONT_APPROACH_Y >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE_Y >= 250.0);
        assert!(TOP_CAMERA_SERVICE_Z > EVIDENCE_POST_Z);
    }

    #[test]
    fn limitation_markers_prevent_protocol_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"no_release_rule"));
        assert!(LIMITATIONS.contains(&"no_barcode_parser"));
        assert!(LIMITATIONS.contains(&"no_electronic_record_claim"));
        assert!(LIMITATIONS.contains(&"purchased_wetted_components_external"));
    }
}
