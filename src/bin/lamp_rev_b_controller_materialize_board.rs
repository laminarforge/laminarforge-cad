#![allow(dead_code)]

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const CONTRACT_PATH: &str = "pcb/lamp_rev_b_controller/contract.toml";
const PARTS_PATH: &str = "pcb/lamp_rev_b_controller/parts.toml";
const PLACEMENT_PATH: &str = "pcb/lamp_rev_b_controller/placement.toml";
const PIN_NETS_PATH: &str = "pcb/lamp_rev_b_controller/pin_nets.toml";
const ROUTING_SEED_PATH: &str = "pcb/lamp_rev_b_controller/routing_seed.toml";
const COPPER_ZONES_PATH: &str = "pcb/lamp_rev_b_controller/copper_zones.toml";
const BOARD_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_pcb";

#[derive(Debug, Deserialize)]
struct Contract {
    board: Board,
    stackup: Stackup,
    zones: Vec<Zone>,
    nets: Vec<Net>,
    #[serde(default)]
    net_groups: Vec<NetGroup>,
}

#[derive(Debug, Deserialize)]
struct Board {
    width_mm: f64,
    height_mm: f64,
    thickness_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Stackup {
    copper_layers: Vec<String>,
    ground_plane_layer: String,
    power_plane_layer: String,
}

#[derive(Debug, Deserialize)]
struct Zone {
    name: String,
    purpose: String,
    x_min_mm: f64,
    x_max_mm: f64,
    y_min_mm: f64,
    y_max_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Net {
    name: String,
}

#[derive(Debug, Deserialize)]
struct NetGroup {
    prefix: String,
    count: u32,
}

#[derive(Debug, Deserialize)]
struct PartsManifest {
    schematic: SchematicSource,
    selected_parts: Vec<SelectedPart>,
}

#[derive(Debug, Deserialize)]
struct SchematicSource {
    footprint_library: String,
}

#[derive(Debug, Deserialize)]
struct SelectedPart {
    id: String,
    module: String,
    value: String,
    footprint: String,
    lcsc_part: String,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    placements: Vec<FootprintPlacement>,
    test_points: Vec<TestPointPlacement>,
    #[serde(default)]
    optical_slots: Vec<OpticalSlotPlacement>,
}

#[derive(Debug, Deserialize)]
struct FootprintPlacement {
    reference: String,
    part_id: String,
    zone: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct TestPointPlacement {
    name: String,
    net: String,
    x_mm: f64,
    y_mm: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct OpticalSlotPlacement {
    slot: u32,
    x_mm: f64,
    y_mm: f64,
    emitter_ref: String,
    detector_ref: String,
    driver_ref: String,
    led_resistor_ref: String,
    base_resistor_ref: String,
}

#[derive(Debug, Deserialize)]
struct PinNetManifest {
    assignments: Vec<PinNetAssignment>,
}

#[derive(Debug, Deserialize)]
struct PinNetAssignment {
    reference: String,
    pins: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct RoutingSeed {
    #[serde(default)]
    segments: Vec<RouteSegment>,
}

#[derive(Debug, Deserialize)]
struct RouteSegment {
    net: String,
    layer: String,
    #[serde(default)]
    via_at_ends: bool,
    #[serde(default)]
    via_at_start: Option<bool>,
    #[serde(default)]
    via_at_end: Option<bool>,
    width_mm: f64,
    start_x_mm: f64,
    start_y_mm: f64,
    end_x_mm: f64,
    end_y_mm: f64,
}

#[derive(Debug, Deserialize)]
struct CopperZonePlan {
    zones: Vec<CopperZone>,
}

#[derive(Debug, Deserialize)]
struct CopperZone {
    name: String,
    net: String,
    layer: String,
    clearance_mm: f64,
    min_thickness_mm: f64,
    thermal_gap_mm: f64,
    thermal_bridge_width_mm: f64,
    remove_islands: bool,
    points: Vec<CopperZonePoint>,
}

#[derive(Debug, Deserialize)]
struct CopperZonePoint {
    x_mm: f64,
    y_mm: f64,
}

struct BoardSources<'a> {
    root: &'a Path,
    contract: &'a Contract,
    parts: &'a PartsManifest,
    placement: &'a PlacementPlan,
    pin_nets: &'a PinNetManifest,
    routing_seed: &'a RoutingSeed,
    copper_zones: &'a CopperZonePlan,
    nets: &'a [String],
}

struct RouteEndpoint {
    x_mm: f64,
    y_mm: f64,
    net_id: usize,
}

struct RouteViaPolicy<'a> {
    via_usage: &'a BTreeMap<String, usize>,
    via_layers: &'a BTreeMap<String, BTreeSet<String>>,
    outer_points: &'a BTreeSet<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let contract = read_toml::<Contract>(&root.join(CONTRACT_PATH))?;
    let parts = read_toml::<PartsManifest>(&root.join(PARTS_PATH))?;
    let placement = read_toml::<PlacementPlan>(&root.join(PLACEMENT_PATH))?;
    let pin_nets = read_toml::<PinNetManifest>(&root.join(PIN_NETS_PATH))?;
    let routing_seed = read_toml::<RoutingSeed>(&root.join(ROUTING_SEED_PATH))?;
    let copper_zones = read_toml::<CopperZonePlan>(&root.join(COPPER_ZONES_PATH))?;
    let nets = expand_nets(&contract);

    let board = render_board(BoardSources {
        root: &root,
        contract: &contract,
        parts: &parts,
        placement: &placement,
        pin_nets: &pin_nets,
        routing_seed: &routing_seed,
        copper_zones: &copper_zones,
        nets: &nets,
    })?;
    fs::write(root.join(BOARD_PATH), board)?;

    println!("Materialized LAMP Rev B controller KiCad board:");
    println!("  {BOARD_PATH}");
    println!("  footprints: {}", placement.placements.len());
    println!("  test points: {}", placement.test_points.len());
    println!("  nets: {}", nets.len());
    println!("  assigned pad nets: {}", assigned_pad_count(&pin_nets));
    println!("  starter route segments: {}", routing_seed.segments.len());
    println!("  copper zones: {}", copper_zones.zones.len());
    Ok(())
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn expand_nets(contract: &Contract) -> Vec<String> {
    let mut names = contract
        .nets
        .iter()
        .map(|net| net.name.clone())
        .collect::<Vec<_>>();
    for group in &contract.net_groups {
        for index in 0..group.count {
            names.push(format!("{}{}", group.prefix, index));
        }
    }
    names
}

fn render_board(sources: BoardSources<'_>) -> Result<String, Box<dyn Error>> {
    let mut counter = UuidCounter::default();
    let net_ids = net_ids(sources.nets);
    let part_by_id = sources
        .parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    let pin_nets_by_ref = sources
        .pin_nets
        .assignments
        .iter()
        .map(|assignment| (assignment.reference.as_str(), assignment))
        .collect::<BTreeMap<_, _>>();

    let mut board = String::new();
    write_header(&mut board, sources.contract, sources.nets)?;
    write_board_geometry(&mut board, sources.contract, &mut counter)?;
    write_zone_guides(&mut board, sources.contract, &mut counter)?;
    write_optical_guides(&mut board, sources.placement, &mut counter)?;

    let footprint_dir =
        resolve_footprint_dir(sources.root, &sources.parts.schematic.footprint_library)?;
    for item in &sources.placement.placements {
        let part = part_by_id.get(item.part_id.as_str()).ok_or_else(|| {
            format!(
                "placement {} references unknown part {}",
                item.reference, item.part_id
            )
        })?;
        write_placed_footprint(
            &mut board,
            &footprint_dir,
            item,
            part,
            pin_nets_by_ref.get(item.reference.as_str()).copied(),
            &net_ids,
            &mut counter,
        )?;
    }

    for point in &sources.placement.test_points {
        write_test_point(&mut board, point, &net_ids, &mut counter)?;
    }

    write_route_segments(
        &mut board,
        sources.contract,
        sources.routing_seed,
        &net_ids,
        &mut counter,
    )?;
    write_copper_zones(
        &mut board,
        sources.contract,
        sources.copper_zones,
        &net_ids,
        &mut counter,
    )?;

    board.push_str(")\n");
    Ok(board)
}

fn write_header(
    board: &mut String,
    contract: &Contract,
    nets: &[String],
) -> Result<(), Box<dyn Error>> {
    writeln!(
        board,
        r#"(kicad_pcb
  (version 20240108)
  (generator "laminarforge_lamp_rev_b_controller_materializer")
  (generator_version "1.0")
  (general
    (thickness {})
    (legacy_teardrops no)
  )
  (paper "A4")
  (layers"#,
        fmt(contract.board.thickness_mm)
    )?;

    for layer in &contract.stackup.copper_layers {
        match layer.as_str() {
            "F.Cu" => writeln!(board, r#"    (0 "F.Cu" signal)"#)?,
            "In1.Cu" => writeln!(
                board,
                r#"    (1 "In1.Cu" power "{}")"#,
                contract.stackup.ground_plane_layer
            )?,
            "In2.Cu" => writeln!(
                board,
                r#"    (2 "In2.Cu" power "{}")"#,
                contract.stackup.power_plane_layer
            )?,
            "B.Cu" => writeln!(board, r#"    (31 "B.Cu" signal)"#)?,
            other => return Err(format!("unsupported copper layer {other}").into()),
        }
    }

    board.push_str(
        r#"    (32 "B.Adhes" user "B.Adhesive")
    (33 "F.Adhes" user "F.Adhesive")
    (34 "B.Paste" user)
    (35 "F.Paste" user)
    (36 "B.SilkS" user "B.Silkscreen")
    (37 "F.SilkS" user "F.Silkscreen")
    (38 "B.Mask" user)
    (39 "F.Mask" user)
    (40 "Dwgs.User" user "User.Drawings")
    (41 "Cmts.User" user "User.Comments")
    (42 "Eco1.User" user "User.Eco1")
    (43 "Eco2.User" user "User.Eco2")
    (44 "Edge.Cuts" user)
    (45 "Margin" user)
    (46 "B.CrtYd" user "B.Courtyard")
    (47 "F.CrtYd" user "F.Courtyard")
    (48 "B.Fab" user "B.Fabrication")
    (49 "F.Fab" user "F.Fabrication")
  )
  (setup
    (pad_to_mask_clearance 0)
    (allow_soldermask_bridges_in_footprints yes)
    (pcbplotparams
      (layerselection 0x00010fc_ffffffff)
      (plot_on_all_layers_selection 0x0000000_00000000)
      (disableapertmacros no)
      (usegerberextensions yes)
      (usegerberattributes yes)
      (usegerberadvancedattributes yes)
      (creategerberjobfile yes)
      (plotframeref no)
      (viasonmask no)
      (mode 1)
      (useauxorigin no)
      (dxf_units mm)
      (dxfpolygonmode yes)
      (dxfimperialunits no)
      (dxfusepcbnewfont yes)
      (plotreference yes)
      (plotvalue yes)
      (plotfptext yes)
      (subtractmaskfromsilk yes)
      (outputformat 1)
      (mirror no)
      (drillshape 0)
      (scaleselection 1)
      (outputdirectory "fab/gerbers/")
    )
  )
  (net 0 "")
"#,
    );

    for (index, net) in nets.iter().enumerate() {
        writeln!(board, r#"  (net {} "{}")"#, index + 1, escape(net))?;
    }
    Ok(())
}

fn write_board_geometry(
    board: &mut String,
    contract: &Contract,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        board,
        r#"  (gr_rect
    (start 0 0)
    (end {} {})
    (stroke (width 0.1) (type solid))
    (fill none)
    (layer "Edge.Cuts")
    (uuid "{}")
  )"#,
        fmt(contract.board.width_mm),
        fmt(contract.board.height_mm),
        counter.next()
    )?;
    write_text(
        board,
        "LaminarForge LAMP Rev B Controller",
        contract.board.width_mm / 2.0,
        contract.board.height_mm - 6.0,
        "Dwgs.User",
        1.0,
        counter,
    )?;
    write_text(
        board,
        "CERN-OHL-S v2",
        contract.board.width_mm / 2.0,
        contract.board.height_mm - 2.0,
        "Dwgs.User",
        1.0,
        counter,
    )?;
    write_controller_fixture_labels(board, counter)?;
    Ok(())
}

fn write_controller_fixture_labels(
    board: &mut String,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    let labels = [
        ("USB", 8.0, 2.2, 0.8),
        ("UART", 14.0, 18.0, 0.8),
        ("ESP32-S3 N16", 58.0, 3.0, 0.8),
        ("ANT KEEP CLEAR", 58.0, 8.0, 0.8),
        ("VIN 12/24", 104.0, 2.2, 0.8),
        ("5V/3V3", 110.0, 31.0, 0.8),
        ("THERM ADC/MUX", 18.0, 34.0, 0.8),
        ("NTC0-7", 23.0, 80.5, 0.8),
        ("LED/CAMERA", 61.0, 39.0, 0.8),
        ("INTERLOCK", 50.0, 78.0, 0.8),
        ("HEATER0/1", 103.0, 42.0, 0.8),
    ];

    for (text, x_mm, y_mm, size_mm) in labels {
        write_text(board, text, x_mm, y_mm, "F.SilkS", size_mm, counter)?;
    }
    Ok(())
}

fn write_zone_guides(
    board: &mut String,
    contract: &Contract,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    for zone in &contract.zones {
        writeln!(
            board,
            r#"  (gr_rect
    (start {} {})
    (end {} {})
    (stroke (width 0.05) (type dash))
    (fill none)
    (layer "Dwgs.User")
    (uuid "{}")
  )"#,
            fmt(zone.x_min_mm),
            fmt(zone.y_min_mm),
            fmt(zone.x_max_mm),
            fmt(zone.y_max_mm),
            counter.next()
        )?;
        write_text(
            board,
            &format!("{}: {}", zone.name, zone.purpose),
            zone.x_min_mm + 1.0,
            zone.y_min_mm + 2.0,
            "Dwgs.User",
            0.75,
            counter,
        )?;
    }
    Ok(())
}

fn write_optical_guides(
    board: &mut String,
    placement: &PlacementPlan,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    for slot in &placement.optical_slots {
        writeln!(
            board,
            r#"  (gr_circle
    (center {} {})
    (end {} {})
    (stroke (width 0.05) (type solid))
    (fill none)
    (layer "Dwgs.User")
    (uuid "{}")
  )"#,
            fmt(slot.x_mm),
            fmt(slot.y_mm),
            fmt(slot.x_mm + 2.5),
            fmt(slot.y_mm),
            counter.next()
        )?;
        write_text(
            board,
            &format!("S{}", slot.slot),
            slot.x_mm - 1.2,
            slot.y_mm + 0.35,
            "Dwgs.User",
            0.6,
            counter,
        )?;
    }
    Ok(())
}

fn write_placed_footprint(
    board: &mut String,
    footprint_dir: &Path,
    item: &FootprintPlacement,
    part: &SelectedPart,
    pin_nets: Option<&PinNetAssignment>,
    net_ids: &BTreeMap<&str, usize>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    if item.side != "top" {
        return Err(format!("{} is on unsupported side {}", item.reference, item.side).into());
    }

    let footprint_name = footprint_name(&part.footprint)?;
    let path = footprint_dir.join(format!("{footprint_name}.kicad_mod"));
    let source = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read footprint {}: {error}", path.display()))?;
    if source.contains("(module ") {
        return Err(format!(
            "{} is still in legacy module syntax; run kicad-cli fp upgrade on pcb/lib/lcsc.pretty",
            path.display()
        )
        .into());
    }

    let mut footprint = rewrite_footprint_name(&source, &part.footprint)?;
    footprint = replace_property_value(&footprint, "Reference", &item.reference)?;
    footprint = replace_property_value(&footprint, "Value", &part.value)?;
    footprint = rewrite_uuids(&footprint, counter);
    footprint = insert_placement(&footprint, item, counter)?;
    if let Some(pin_nets) = pin_nets {
        footprint = apply_pin_nets(&footprint, pin_nets, net_ids)?;
    }
    footprint = demote_footprint_silkscreen(&footprint);

    board.push('\n');
    for line in footprint.lines() {
        writeln!(board, "  {line}")?;
    }
    Ok(())
}

fn write_test_point(
    board: &mut String,
    point: &TestPointPlacement,
    net_ids: &BTreeMap<&str, usize>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    if point.side != "top" {
        return Err(format!("{} is on unsupported side {}", point.name, point.side).into());
    }
    let net_id = *net_ids.get(point.net.as_str()).ok_or_else(|| {
        format!(
            "test point {} references unknown net {}",
            point.name, point.net
        )
    })?;
    writeln!(
        board,
        r#"
  (footprint "lcsc:TESTPOINT_SMD_1.5MM"
    (layer "F.Cu")
    (at {} {} 0)
    (tstamp "{}")
    (property "Reference" "{}" (at 0 -1.8 0) (layer "F.Fab") (uuid "{}") (effects (font (size 0.8 0.8) (thickness 0.1))))
    (property "Value" "{}" (at 0 1.8 0) (layer "F.Fab") (uuid "{}") (effects (font (size 0.8 0.8) (thickness 0.1))))
    (attr smd exclude_from_pos_files)
    (pad "1" smd circle (at 0 0) (size 1.5 1.5) (layers "F.Cu" "F.Mask") (net {} "{}") (uuid "{}"))
  )"#,
        fmt(point.x_mm),
        fmt(point.y_mm),
        counter.next(),
        escape(&point.name),
        counter.next(),
        escape(&point.net),
        counter.next(),
        net_id,
        escape(&point.net),
        counter.next()
    )?;
    Ok(())
}

fn write_route_segments(
    board: &mut String,
    contract: &Contract,
    routing_seed: &RoutingSeed,
    net_ids: &BTreeMap<&str, usize>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    let via_usage = route_via_usage(routing_seed, net_ids)?;
    let via_layers = route_via_layers(routing_seed, net_ids)?;
    let outer_points = route_outer_points(routing_seed, net_ids)?;
    let via_policy = RouteViaPolicy {
        via_usage: &via_usage,
        via_layers: &via_layers,
        outer_points: &outer_points,
    };
    let mut emitted_vias = BTreeSet::new();

    for segment in &routing_seed.segments {
        if segment.layer != "F.Cu"
            && segment.layer != "B.Cu"
            && segment.layer != "In1.Cu"
            && segment.layer != "In2.Cu"
        {
            return Err(format!(
                "route segment for {} uses unsupported layer {}",
                segment.net, segment.layer
            )
            .into());
        }
        if segment.layer != "F.Cu"
            && !segment.via_at_ends
            && segment.via_at_start.is_none()
            && segment.via_at_end.is_none()
        {
            return Err(format!(
                "{} route segment for {} must set via_at_ends or explicit via_at_start/via_at_end",
                segment.layer, segment.net
            )
            .into());
        }
        if segment.width_mm <= 0.0 {
            return Err(format!("route segment for {} has non-positive width", segment.net).into());
        }
        validate_board_point(
            segment.start_x_mm,
            segment.start_y_mm,
            contract,
            &segment.net,
        )?;
        validate_board_point(segment.end_x_mm, segment.end_y_mm, contract, &segment.net)?;
        let net_id = *net_ids
            .get(segment.net.as_str())
            .ok_or_else(|| format!("route segment references unknown net {}", segment.net))?;
        if route_segment_via_at_start(segment) {
            write_route_endpoint_via(
                board,
                RouteEndpoint {
                    x_mm: segment.start_x_mm,
                    y_mm: segment.start_y_mm,
                    net_id,
                },
                &via_policy,
                &mut emitted_vias,
                counter,
            )?;
        }
        if route_segment_via_at_end(segment) {
            write_route_endpoint_via(
                board,
                RouteEndpoint {
                    x_mm: segment.end_x_mm,
                    y_mm: segment.end_y_mm,
                    net_id,
                },
                &via_policy,
                &mut emitted_vias,
                counter,
            )?;
        }
        writeln!(
            board,
            r#"
  (segment
    (start {} {})
    (end {} {})
    (width {})
    (layer "{}")
    (net {})
    (uuid "{}")
  )"#,
            fmt(segment.start_x_mm),
            fmt(segment.start_y_mm),
            fmt(segment.end_x_mm),
            fmt(segment.end_y_mm),
            fmt(segment.width_mm),
            escape(&segment.layer),
            net_id,
            counter.next()
        )?;
    }
    Ok(())
}

fn route_via_usage(
    routing_seed: &RoutingSeed,
    net_ids: &BTreeMap<&str, usize>,
) -> Result<BTreeMap<String, usize>, Box<dyn Error>> {
    let mut usage = BTreeMap::new();
    for segment in &routing_seed.segments {
        let net_id = *net_ids
            .get(segment.net.as_str())
            .ok_or_else(|| format!("route segment references unknown net {}", segment.net))?;
        for (wants_via, x_mm, y_mm) in [
            (
                route_segment_via_at_start(segment),
                segment.start_x_mm,
                segment.start_y_mm,
            ),
            (
                route_segment_via_at_end(segment),
                segment.end_x_mm,
                segment.end_y_mm,
            ),
        ] {
            if !wants_via {
                continue;
            }
            *usage
                .entry(route_point_key(net_id, x_mm, y_mm))
                .or_insert(0) += 1;
        }
    }
    Ok(usage)
}

fn route_via_layers(
    routing_seed: &RoutingSeed,
    net_ids: &BTreeMap<&str, usize>,
) -> Result<BTreeMap<String, BTreeSet<String>>, Box<dyn Error>> {
    let mut layers = BTreeMap::new();
    for segment in &routing_seed.segments {
        let net_id = *net_ids
            .get(segment.net.as_str())
            .ok_or_else(|| format!("route segment references unknown net {}", segment.net))?;
        for (wants_via, x_mm, y_mm) in [
            (
                route_segment_via_at_start(segment),
                segment.start_x_mm,
                segment.start_y_mm,
            ),
            (
                route_segment_via_at_end(segment),
                segment.end_x_mm,
                segment.end_y_mm,
            ),
        ] {
            if !wants_via {
                continue;
            }
            layers
                .entry(route_point_key(net_id, x_mm, y_mm))
                .or_insert_with(BTreeSet::new)
                .insert(segment.layer.clone());
        }
    }
    Ok(layers)
}

fn route_segment_via_at_start(segment: &RouteSegment) -> bool {
    segment.via_at_start.unwrap_or(segment.via_at_ends)
}

fn route_segment_via_at_end(segment: &RouteSegment) -> bool {
    segment.via_at_end.unwrap_or(segment.via_at_ends)
}

fn route_outer_points(
    routing_seed: &RoutingSeed,
    net_ids: &BTreeMap<&str, usize>,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut points = BTreeSet::new();
    for segment in &routing_seed.segments {
        if segment.layer != "F.Cu" {
            continue;
        }
        let net_id = *net_ids
            .get(segment.net.as_str())
            .ok_or_else(|| format!("route segment references unknown net {}", segment.net))?;
        points.insert(route_point_key(
            net_id,
            segment.start_x_mm,
            segment.start_y_mm,
        ));
        points.insert(route_point_key(net_id, segment.end_x_mm, segment.end_y_mm));
    }
    Ok(points)
}

fn write_route_endpoint_via(
    board: &mut String,
    endpoint: RouteEndpoint,
    policy: &RouteViaPolicy<'_>,
    emitted_vias: &mut BTreeSet<String>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    let key = route_point_key(endpoint.net_id, endpoint.x_mm, endpoint.y_mm);
    let route_layers = policy.via_layers.get(&key);
    let is_route_endpoint = policy.via_usage.get(&key).copied().unwrap_or(0) == 1;
    let changes_route_layer = route_layers.map(BTreeSet::len).unwrap_or(0) > 1;
    let connects_outer_copper = policy.outer_points.contains(&key);
    if is_route_endpoint || changes_route_layer || connects_outer_copper {
        write_via_once(
            board,
            endpoint.x_mm,
            endpoint.y_mm,
            endpoint.net_id,
            emitted_vias,
            counter,
        )?;
    }
    Ok(())
}

fn write_via_once(
    board: &mut String,
    x_mm: f64,
    y_mm: f64,
    net_id: usize,
    emitted_vias: &mut BTreeSet<String>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    let key = route_point_key(net_id, x_mm, y_mm);
    if emitted_vias.insert(key) {
        write_via(board, x_mm, y_mm, net_id, counter)?;
    }
    Ok(())
}

fn route_point_key(net_id: usize, x_mm: f64, y_mm: f64) -> String {
    format!("{net_id}:{x_mm:.3}:{y_mm:.3}")
}

fn validate_board_point(
    x_mm: f64,
    y_mm: f64,
    contract: &Contract,
    net: &str,
) -> Result<(), Box<dyn Error>> {
    if x_mm < 0.0 || y_mm < 0.0 || x_mm > contract.board.width_mm || y_mm > contract.board.height_mm
    {
        return Err(format!("route segment for {net} has off-board point {x_mm},{y_mm}").into());
    }
    Ok(())
}

fn write_via(
    board: &mut String,
    x_mm: f64,
    y_mm: f64,
    net_id: usize,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        board,
        r#"
  (via
    (at {} {})
    (size 0.6)
    (drill 0.3)
    (layers "F.Cu" "B.Cu")
    (net {})
    (uuid "{}")
  )"#,
        fmt(x_mm),
        fmt(y_mm),
        net_id,
        counter.next()
    )?;
    Ok(())
}

fn write_copper_zones(
    board: &mut String,
    contract: &Contract,
    copper_zones: &CopperZonePlan,
    net_ids: &BTreeMap<&str, usize>,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    for zone in &copper_zones.zones {
        if zone.layer != "F.Cu"
            && zone.layer != "B.Cu"
            && zone.layer != "In1.Cu"
            && zone.layer != "In2.Cu"
        {
            return Err(format!(
                "copper zone {} uses unsupported layer {}",
                zone.name, zone.layer
            )
            .into());
        }
        if zone.points.len() < 3 {
            return Err(format!("copper zone {} needs at least three points", zone.name).into());
        }
        if zone.clearance_mm < 0.15 {
            return Err(
                format!("copper zone {} clearance is below Rev B minimum", zone.name).into(),
            );
        }
        if zone.min_thickness_mm <= 0.0 {
            return Err(format!("copper zone {} min thickness must be positive", zone.name).into());
        }
        let net_id = *net_ids.get(zone.net.as_str()).ok_or_else(|| {
            format!(
                "copper zone {} references unknown net {}",
                zone.name, zone.net
            )
        })?;
        for point in &zone.points {
            validate_board_point(point.x_mm, point.y_mm, contract, &zone.net)?;
        }

        let island_removal_mode = if zone.remove_islands { 0 } else { 1 };

        writeln!(
            board,
            r#"
  (zone
    (net {})
    (net_name "{}")
    (layer "{}")
    (uuid "{}")
    (hatch edge 0.5)
    (connect_pads yes (clearance {}))
    (min_thickness {})
    (fill yes (thermal_gap {}) (thermal_bridge_width {}) (island_removal_mode {}))
    (polygon
      (pts"#,
            net_id,
            escape(&zone.net),
            escape(&zone.layer),
            counter.next(),
            fmt(zone.clearance_mm),
            fmt(zone.min_thickness_mm),
            fmt(zone.thermal_gap_mm),
            fmt(zone.thermal_bridge_width_mm),
            island_removal_mode
        )?;
        for point in &zone.points {
            writeln!(
                board,
                r#"        (xy {} {})"#,
                fmt(point.x_mm),
                fmt(point.y_mm)
            )?;
        }
        board.push_str(
            r#"      )
    )
  )"#,
        );
        board.push('\n');
    }
    Ok(())
}

fn write_text(
    board: &mut String,
    text: &str,
    x: f64,
    y: f64,
    layer: &str,
    size: f64,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    writeln!(
        board,
        r#"  (gr_text "{}"
    (at {} {} 0)
    (layer "{}")
    (uuid "{}")
    (effects (font (size {} {}) (thickness 0.1)))
  )"#,
        escape(text),
        fmt(x),
        fmt(y),
        layer,
        counter.next(),
        fmt(size),
        fmt(size)
    )?;
    Ok(())
}

fn resolve_footprint_dir(root: &Path, manifest_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let dir = root.join("pcb/lamp_rev_b_controller").join(manifest_path);
    if !dir.is_dir() {
        return Err(format!("footprint library path does not exist: {}", dir.display()).into());
    }
    Ok(dir)
}

fn footprint_name(footprint: &str) -> Result<&str, Box<dyn Error>> {
    footprint
        .split_once(':')
        .map(|(_, name)| name)
        .ok_or_else(|| format!("footprint {footprint} must include a library prefix").into())
}

fn rewrite_footprint_name(source: &str, footprint: &str) -> Result<String, Box<dyn Error>> {
    let rest = source
        .strip_prefix("(footprint ")
        .ok_or_else(|| "footprint file must start with (footprint".to_string())?;
    let quote_start = rest
        .find('"')
        .ok_or_else(|| "footprint name is missing opening quote".to_string())?;
    let name_start = quote_start + 1;
    let name_end = rest[name_start..]
        .find('"')
        .map(|index| name_start + index)
        .ok_or_else(|| "footprint name is missing closing quote".to_string())?;
    let mut out = String::new();
    out.push_str("(footprint \"");
    out.push_str(&escape(footprint));
    out.push('"');
    out.push_str(&rest[name_end + 1..]);
    Ok(out)
}

fn replace_property_value(
    source: &str,
    property: &str,
    value: &str,
) -> Result<String, Box<dyn Error>> {
    let needle = format!("(property \"{}\" \"", escape(property));
    let start = source
        .find(&needle)
        .ok_or_else(|| format!("footprint is missing property {property}"))?;
    let value_start = start + needle.len();
    let value_end = source[value_start..]
        .find('"')
        .map(|index| value_start + index)
        .ok_or_else(|| format!("footprint property {property} has no closing quote"))?;
    let mut out = String::new();
    out.push_str(&source[..value_start]);
    out.push_str(&escape(value));
    out.push_str(&source[value_end..]);
    Ok(out)
}

fn rewrite_uuids(source: &str, counter: &mut UuidCounter) -> String {
    let needle = "(uuid \"";
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;
    while let Some(relative) = source[cursor..].find(needle) {
        let start = cursor + relative;
        let value_start = start + needle.len();
        let Some(value_end_relative) = source[value_start..].find('"') else {
            break;
        };
        let value_end = value_start + value_end_relative;
        out.push_str(&source[cursor..value_start]);
        out.push_str(&counter.next());
        cursor = value_end;
    }
    out.push_str(&source[cursor..]);
    out
}

fn insert_placement(
    source: &str,
    item: &FootprintPlacement,
    counter: &mut UuidCounter,
) -> Result<String, Box<dyn Error>> {
    let layer_marker = "(layer \"F.Cu\")";
    let layer_start = source
        .find(layer_marker)
        .ok_or_else(|| "footprint is missing F.Cu layer marker".to_string())?;
    let insert_at = source[layer_start..]
        .find('\n')
        .map(|index| layer_start + index + 1)
        .ok_or_else(|| "footprint layer marker is not line terminated".to_string())?;

    let metadata = format!(
        "\t(at {} {} {})\n\t(tstamp \"{}\")\n",
        fmt(item.x_mm),
        fmt(item.y_mm),
        fmt(item.rotation_deg),
        counter.next()
    );

    let mut out = String::new();
    out.push_str(&source[..insert_at]);
    out.push_str(&metadata);
    out.push_str(&source[insert_at..]);
    Ok(out)
}

fn apply_pin_nets(
    source: &str,
    assignment: &PinNetAssignment,
    net_ids: &BTreeMap<&str, usize>,
) -> Result<String, Box<dyn Error>> {
    let mut footprint = source.to_string();
    for (pin, net) in &assignment.pins {
        let net_id = *net_ids.get(net.as_str()).ok_or_else(|| {
            format!(
                "pin-net assignment {} pad {} references unknown net {}",
                assignment.reference, pin, net
            )
        })?;
        let (updated, count) = assign_pad_net(&footprint, pin, net_id, net);
        if count == 0 {
            return Err(format!(
                "pin-net assignment {} pad {} did not match any pad in footprint",
                assignment.reference, pin
            )
            .into());
        }
        footprint = updated;
    }
    Ok(footprint)
}

fn assign_pad_net(source: &str, pad: &str, net_id: usize, net: &str) -> (String, usize) {
    let marker = format!("(pad \"{}\"", escape(pad));
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;
    let mut count = 0;

    while let Some(relative) = source[cursor..].find(&marker) {
        let pad_start = cursor + relative;
        let Some(pad_end) = find_matching_sexpr_end(source, pad_start) else {
            break;
        };
        let pad_block = &source[pad_start..pad_end];
        out.push_str(&source[cursor..pad_start]);

        let updated_pad = if pad_block.contains("(net ") {
            replace_pad_net(pad_block, net_id, net)
        } else {
            insert_pad_net(pad_block, net_id, net)
        };
        out.push_str(&updated_pad);
        cursor = pad_end;
        count += 1;
    }

    out.push_str(&source[cursor..]);
    (out, count)
}

fn replace_pad_net(pad_block: &str, net_id: usize, net: &str) -> String {
    let Some(net_start) = pad_block.find("(net ") else {
        return insert_pad_net(pad_block, net_id, net);
    };
    let Some(net_end) = find_matching_sexpr_end(pad_block, net_start) else {
        return insert_pad_net(pad_block, net_id, net);
    };
    let mut out = String::new();
    out.push_str(&pad_block[..net_start]);
    out.push_str(&format!(r#"(net {} "{}")"#, net_id, escape(net)));
    out.push_str(&pad_block[net_end..]);
    out
}

fn insert_pad_net(pad_block: &str, net_id: usize, net: &str) -> String {
    let insert_at = pad_block
        .find("\n\t\t(uuid ")
        .or_else(|| pad_block.rfind('\n'))
        .unwrap_or_else(|| pad_block.len().saturating_sub(1));
    let mut out = String::new();
    out.push_str(&pad_block[..insert_at]);
    out.push_str(&format!("\n\t\t(net {} \"{}\")", net_id, escape(net)));
    out.push_str(&pad_block[insert_at..]);
    out
}

fn find_matching_sexpr_end(source: &str, start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (relative, ch) in source[start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(start + relative + ch.len_utf8());
                }
            }
            _ => {}
        }
    }

    None
}

fn demote_footprint_silkscreen(source: &str) -> String {
    source.replace(r#""F.SilkS""#, r#""F.Fab""#)
}

fn assigned_pad_count(pin_nets: &PinNetManifest) -> usize {
    pin_nets
        .assignments
        .iter()
        .map(|assignment| assignment.pins.len())
        .sum()
}

fn net_ids(nets: &[String]) -> BTreeMap<&str, usize> {
    nets.iter()
        .enumerate()
        .map(|(index, net)| (net.as_str(), index + 1))
        .collect()
}

fn fmt(value: f64) -> String {
    if (value.fract()).abs() < 1e-9 {
        format!("{value:.0}")
    } else {
        format!("{value:.3}")
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Default)]
struct UuidCounter {
    next_value: usize,
}

impl UuidCounter {
    fn next(&mut self) -> String {
        self.next_value += 1;
        format!("00000000-0000-4000-8000-{:012x}", self.next_value)
    }
}
