use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const PARTS_PATH: &str = "pcb/lamp_rev_b_controller/parts.toml";
const PLACEMENT_PATH: &str = "pcb/lamp_rev_b_controller/placement.toml";
const PIN_NETS_PATH: &str = "pcb/lamp_rev_b_controller/pin_nets.toml";
const FAB_CONFIG_PATH: &str = "pcb/lamp_rev_b_controller/fab_release.toml";
const SCHEMATIC_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_sch";

const GRID_MM: f64 = 1.27;
const SYMBOL_WIDTH_MM: f64 = 14.0 * GRID_MM;
const PIN_LENGTH_MM: f64 = 2.54;
const PIN_PITCH_MM: f64 = 2.54;
const COMPONENT_ORIGIN_X_MM: f64 = 24.0 * GRID_MM;
const COMPONENT_ORIGIN_Y_MM: f64 = 40.0 * GRID_MM;
const COMPONENT_X_PITCH_MM: f64 = 54.0 * GRID_MM;
const COMPONENT_Y_PITCH_MM: f64 = 60.0 * GRID_MM;
const COMPONENTS_PER_ROW: usize = 16;

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
    value: String,
    symbol: String,
    footprint: String,
    lcsc_part: String,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    placements: Vec<FootprintPlacement>,
    test_points: Vec<TestPointPlacement>,
}

#[derive(Debug, Deserialize)]
struct FootprintPlacement {
    reference: String,
    part_id: String,
}

#[derive(Debug, Deserialize)]
struct TestPointPlacement {
    name: String,
    net: String,
}

#[derive(Debug, Deserialize)]
struct PinNetManifest {
    assignments: Vec<PinNetAssignment>,
}

#[derive(Debug, Deserialize)]
struct PinNetAssignment {
    reference: String,
    #[serde(default)]
    notes: String,
    pins: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct FabConfig {
    assembly: AssemblyConfig,
}

#[derive(Debug, Default, Deserialize)]
struct AssemblyConfig {
    #[serde(default)]
    dnp_part_ids: BTreeSet<String>,
}

#[derive(Clone)]
struct CaptureComponent {
    reference: String,
    value: String,
    source_symbol: String,
    footprint: String,
    lcsc_part: String,
    notes: String,
    in_bom: bool,
    dnp: bool,
    pins: Vec<CapturePin>,
}

#[derive(Clone)]
struct CapturePin {
    number: String,
    name: String,
    net: Option<String>,
}

struct SymbolPinGeometry {
    number: String,
    name: String,
    x_mm: f64,
    y_mm: f64,
    orientation_deg: i32,
    label_x_mm: f64,
    label_y_mm: f64,
    label_orientation_deg: i32,
}

#[derive(Default)]
struct UuidCounter {
    next_id: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let parts = read_toml::<PartsManifest>(&root.join(PARTS_PATH))?;
    let placement = read_toml::<PlacementPlan>(&root.join(PLACEMENT_PATH))?;
    let pin_nets = read_toml::<PinNetManifest>(&root.join(PIN_NETS_PATH))?;
    let fab_config = read_toml::<FabConfig>(&root.join(FAB_CONFIG_PATH))?;
    let capture = build_capture(&root, &parts, &placement, &pin_nets, &fab_config)?;
    let schematic = render_schematic(&capture)?;
    fs::write(root.join(SCHEMATIC_PATH), schematic)?;

    let placed = placement.placements.len();
    let test_points = placement.test_points.len();
    let connected_pins = capture
        .iter()
        .flat_map(|component| component.pins.iter())
        .filter(|pin| pin.net.is_some())
        .count();
    let no_connect_pins = capture
        .iter()
        .flat_map(|component| component.pins.iter())
        .filter(|pin| pin.net.is_none())
        .count();

    println!("Materialized LAMP Rev B controller KiCad schematic:");
    println!("  {SCHEMATIC_PATH}");
    println!("  placed schematic symbols: {}", capture.len());
    println!("  manifest placements: {placed}");
    println!("  test point symbols: {test_points}");
    println!("  connected pins: {connected_pins}");
    println!("  explicit no-connect pins: {no_connect_pins}");
    Ok(())
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn build_capture(
    root: &Path,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    pin_nets: &PinNetManifest,
    fab_config: &FabConfig,
) -> Result<Vec<CaptureComponent>, Box<dyn Error>> {
    let part_by_id = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    let assignment_by_ref = pin_nets
        .assignments
        .iter()
        .map(|assignment| (assignment.reference.as_str(), assignment))
        .collect::<BTreeMap<_, _>>();
    let footprint_dir = resolve_footprint_dir(root, &parts.schematic.footprint_library)?;
    for part_id in &fab_config.assembly.dnp_part_ids {
        if !part_by_id.contains_key(part_id.as_str()) {
            return Err(format!("DNP part id {part_id} is not in selected_parts").into());
        }
    }

    let mut capture = Vec::new();
    for item in &placement.placements {
        let part = part_by_id.get(item.part_id.as_str()).ok_or_else(|| {
            format!(
                "placement {} references unknown part {}",
                item.reference, item.part_id
            )
        })?;
        let pads = footprint_pads(&footprint_dir, &part.footprint)?;
        let assignment = assignment_by_ref.get(item.reference.as_str()).copied();
        let assigned_pins = assignment
            .map(|assignment| assignment.pins.clone())
            .unwrap_or_default();
        for pin in assigned_pins.keys() {
            if !pads.contains(pin) {
                return Err(format!(
                    "{} assigns pad {} but footprint {} does not contain it",
                    item.reference, pin, part.footprint
                )
                .into());
            }
        }

        let mut pins = Vec::new();
        for pad in pads {
            let net = assigned_pins.get(&pad).cloned();
            let name = net.clone().unwrap_or_else(|| format!("NC_{pad}"));
            pins.push(CapturePin {
                number: pad,
                name,
                net,
            });
        }
        capture.push(CaptureComponent {
            reference: item.reference.clone(),
            value: part.value.clone(),
            source_symbol: part.symbol.clone(),
            footprint: part.footprint.clone(),
            lcsc_part: part.lcsc_part.clone(),
            notes: assignment
                .map(|assignment| assignment.notes.clone())
                .unwrap_or_default(),
            in_bom: true,
            dnp: fab_config
                .assembly
                .dnp_part_ids
                .contains(item.part_id.as_str()),
            pins,
        });
    }

    for point in &placement.test_points {
        capture.push(CaptureComponent {
            reference: point.name.clone(),
            value: format!("test point {}", point.net),
            source_symbol: "TESTPOINT_SMD_1.5MM".to_string(),
            footprint: "lcsc:TESTPOINT_SMD_1.5MM".to_string(),
            lcsc_part: "VIRTUAL_TESTPOINT".to_string(),
            notes: "Virtual PCB test pad generated from placement.toml; excluded from fabrication BOM and CPL"
                .to_string(),
            in_bom: false,
            dnp: false,
            pins: vec![CapturePin {
                number: "1".to_string(),
                name: point.net.clone(),
                net: Some(point.net.clone()),
            }],
        });
    }

    capture.sort_by(|a, b| reference_sort_key(&a.reference).cmp(&reference_sort_key(&b.reference)));
    Ok(capture)
}

fn resolve_footprint_dir(root: &Path, manifest_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let dir = root.join("pcb/lamp_rev_b_controller").join(manifest_path);
    if !dir.is_dir() {
        return Err(format!("footprint library path does not exist: {}", dir.display()).into());
    }
    Ok(dir)
}

fn footprint_pads(
    footprint_dir: &Path,
    footprint: &str,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let footprint_name = footprint
        .split_once(':')
        .map(|(_, name)| name)
        .ok_or_else(|| format!("footprint {footprint} must include a library prefix"))?;
    let path = footprint_dir.join(format!("{footprint_name}.kicad_mod"));
    let source = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read footprint {}: {error}", path.display()))?;
    let mut pads = BTreeSet::new();
    let mut cursor = 0usize;
    let needle = "(pad \"";
    while let Some(relative) = source[cursor..].find(needle) {
        let start = cursor + relative + needle.len();
        let Some(end_relative) = source[start..].find('"') else {
            return Err(format!("unterminated pad name in {}", path.display()).into());
        };
        pads.insert(source[start..start + end_relative].to_string());
        cursor = start + end_relative + 1;
    }
    if pads.is_empty() {
        return Err(format!("footprint {} has no pads", path.display()).into());
    }
    Ok(pads)
}

fn render_schematic(capture: &[CaptureComponent]) -> Result<String, Box<dyn Error>> {
    let mut counter = UuidCounter::default();
    let mut schematic = String::new();
    writeln!(
        schematic,
        r#"(kicad_sch
  (version 20230121)
  (generator "laminarforge_lamp_rev_b_controller_capture")
  (uuid "{}")
  (paper "A0")
  (title_block
    (title "LaminarForge LAMP Rev B Controller Carrier")
    (date "2026-07-01")
    (rev "B-proto")
    (company "LaminarForge")
    (comment 1 "Captured connectivity generated from parts, placement, pin_nets, and test point manifests.")
    (comment 2 "Selected P7805/AP63203/LDD-700H source package preserved; manual/DNP semantics come from fab_release.toml.")
    (comment 3 "Heater pad, optics, camera module, cartridge wet path, and enclosure are connectorized.")
  )"#,
        counter.next()
    )?;
    write_lib_symbols(&mut schematic, capture, &mut counter)?;
    write_source_notes(&mut schematic, &mut counter)?;
    write_symbols_and_connectivity(&mut schematic, capture, &mut counter)?;
    schematic.push_str(
        r#"  (sheet_instances
    (path "/" (page "1"))
  )
"#,
    );
    write_symbol_instances(&mut schematic, capture)?;
    schematic.push_str(")\n");
    Ok(schematic)
}

fn write_lib_symbols(
    schematic: &mut String,
    capture: &[CaptureComponent],
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    schematic.push_str("  (lib_symbols\n");
    for component in capture {
        let geometry = pin_geometry(&component.pins);
        let height = symbol_height(&component.pins);
        let half_height = height / 2.0;
        let half_width = SYMBOL_WIDTH_MM / 2.0;
        writeln!(
            schematic,
            r#"    (symbol "{}"
      (pin_names (offset 0.508))
      (exclude_from_sim no)
      (in_bom {})
      (on_board yes)
      (property "Reference" "{}" (at 0 {} 0) (effects (font (size 1.27 1.27))))
      (property "Value" "{}" (at 0 {} 0) (effects (font (size 1.27 1.27))))
      (property "Footprint" "{}" (at 0 {} 0) (effects (font (size 1.27 1.27)) hide))
      (property "LCSC Part" "{}" (at 0 {} 0) (effects (font (size 1.27 1.27)) hide))
      (property "Source Symbol" "{}" (at 0 {} 0) (effects (font (size 1.27 1.27)) hide))
      (symbol "{}_0_1"
        (rectangle
          (start {} {})
          (end {} {})
          (stroke (width 0.254) (type default))
          (fill (type background))
        )"#,
            symbol_name(&component.reference),
            yes_no(component.in_bom),
            reference_prefix(&component.reference),
            fmt(-(half_height + 5.08)),
            escape(&component.value),
            fmt(-(half_height + 7.62)),
            escape(&component.footprint),
            fmt(-(half_height + 10.16)),
            escape(&component.lcsc_part),
            fmt(-(half_height + 12.70)),
            escape(&component.source_symbol),
            fmt(-(half_height + 15.24)),
            symbol_name(&component.reference),
            fmt(-half_width),
            fmt(half_height),
            fmt(half_width),
            fmt(-half_height)
        )?;
        for pin in &geometry {
            writeln!(
                schematic,
                r#"        (pin passive line
          (at {} {} {})
          (length {})
          (name "{}" (effects (font (size 1.0 1.0))))
          (number "{}" (effects (font (size 1.0 1.0))))
        )"#,
                fmt(pin.x_mm),
                fmt(pin.y_mm),
                pin.orientation_deg,
                fmt(PIN_LENGTH_MM),
                escape(&pin.name),
                escape(&pin.number)
            )?;
        }
        schematic.push_str("      )\n    )\n");
        counter.reserve(1);
    }
    schematic.push_str("  )\n");
    Ok(())
}

fn write_source_notes(
    schematic: &mut String,
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    let notes = [
        (
            "Connectivity source: parts.toml + placement.toml + pin_nets.toml + placement test points.",
            20.0,
            18.0,
        ),
        (
            "Every assigned footprint pad is tied to a global net label; unassigned footprint pads are explicit no-connects.",
            20.0,
            23.0,
        ),
        (
            "External high-power LED remains off-board on J15; LDD-700H is manual/THT no-substitution for first articles.",
            20.0,
            28.0,
        ),
    ];
    for (text, x, y) in notes {
        writeln!(
            schematic,
            r#"  (text "{}"
    (at {} {} 0)
    (effects (font (size 1.27 1.27)) (justify left))
    (uuid "{}")
  )"#,
            escape(text),
            fmt(x),
            fmt(y),
            counter.next()
        )?;
    }
    Ok(())
}

fn write_symbols_and_connectivity(
    schematic: &mut String,
    capture: &[CaptureComponent],
    counter: &mut UuidCounter,
) -> Result<(), Box<dyn Error>> {
    for (index, component) in capture.iter().enumerate() {
        let column = index % COMPONENTS_PER_ROW;
        let row = index / COMPONENTS_PER_ROW;
        let origin_x = COMPONENT_ORIGIN_X_MM + column as f64 * COMPONENT_X_PITCH_MM;
        let origin_y = COMPONENT_ORIGIN_Y_MM + row as f64 * COMPONENT_Y_PITCH_MM;
        let geometry = pin_geometry(&component.pins);
        let symbol_uuid = symbol_path_uuid(index);

        writeln!(
            schematic,
            r#"  (symbol
    (lib_id "{}")
    (at {} {} 0)
    (unit 1)
    (exclude_from_sim no)
    (in_bom {})
    (on_board yes)
    (dnp {})
    (uuid "{}")
    (property "Reference" "{}" (at {} {} 0) (effects (font (size 1.27 1.27))))
    (property "Value" "{}" (at {} {} 0) (effects (font (size 1.27 1.27))))
    (property "Footprint" "{}" (at {} {} 0) (effects (font (size 1.27 1.27)) hide))
    (property "LCSC Part" "{}" (at {} {} 0) (effects (font (size 1.27 1.27)) hide))
    (property "Notes" "{}" (at {} {} 0) (effects (font (size 1.27 1.27)) hide))"#,
            symbol_name(&component.reference),
            fmt(origin_x),
            fmt(origin_y),
            yes_no(component.in_bom),
            yes_no(component.dnp),
            symbol_uuid,
            escape(&component.reference),
            fmt(origin_x),
            fmt(origin_y - symbol_height(&component.pins) / 2.0 - 5.08),
            escape(&component.value),
            fmt(origin_x),
            fmt(origin_y + symbol_height(&component.pins) / 2.0 + 5.08),
            escape(&component.footprint),
            fmt(origin_x),
            fmt(origin_y + symbol_height(&component.pins) / 2.0 + 7.62),
            escape(&component.lcsc_part),
            fmt(origin_x),
            fmt(origin_y + symbol_height(&component.pins) / 2.0 + 10.16),
            escape(&component.notes),
            fmt(origin_x),
            fmt(origin_y + symbol_height(&component.pins) / 2.0 + 12.70)
        )?;
        for pin in &geometry {
            writeln!(
                schematic,
                r#"    (pin "{}" (uuid "{}"))"#,
                escape(&pin.number),
                counter.next()
            )?;
        }
        schematic.push_str("  )\n");

        for pin in &geometry {
            let global_x = origin_x + pin.x_mm;
            // KiCad mirrors library-symbol Y coordinates when instantiating a
            // symbol on the sheet. Emit connectivity at the transformed pin
            // position, not at the raw library-local Y coordinate.
            let global_y = origin_y - pin.y_mm;
            let label_x = origin_x + pin.label_x_mm;
            let label_y = origin_y - pin.label_y_mm;
            let Some(net) = component
                .pins
                .iter()
                .find(|candidate| candidate.number == pin.number)
                .and_then(|candidate| candidate.net.as_ref())
            else {
                writeln!(
                    schematic,
                    r#"  (no_connect
    (at {} {})
    (uuid "{}")
  )"#,
                    fmt(global_x),
                    fmt(global_y),
                    counter.next()
                )?;
                continue;
            };
            writeln!(
                schematic,
                r#"  (wire
    (pts
      (xy {} {})
      (xy {} {})
    )
    (stroke (width 0) (type default))
    (uuid "{}")
  )
  (global_label "{}" (shape passive)
    (at {} {} {})
    (effects (font (size 1.0 1.0)) (justify left))
    (uuid "{}")
  )"#,
                fmt(global_x),
                fmt(global_y),
                fmt(label_x),
                fmt(label_y),
                counter.next(),
                escape(net),
                fmt(label_x),
                fmt(label_y),
                pin.label_orientation_deg,
                counter.next()
            )?;
        }
    }
    Ok(())
}

fn write_symbol_instances(
    schematic: &mut String,
    capture: &[CaptureComponent],
) -> Result<(), Box<dyn Error>> {
    schematic.push_str("  (symbol_instances\n");
    for (index, component) in capture.iter().enumerate() {
        writeln!(
            schematic,
            r#"    (path "/{}"
      (reference "{}")
      (unit 1)
      (value "{}")
      (footprint "{}")
    )"#,
            symbol_path_uuid(index),
            escape(&component.reference),
            escape(&component.value),
            escape(&component.footprint)
        )?;
    }
    schematic.push_str("  )\n");
    Ok(())
}

fn pin_geometry(pins: &[CapturePin]) -> Vec<SymbolPinGeometry> {
    let split = pins.len().div_ceil(2);
    let half_width = SYMBOL_WIDTH_MM / 2.0;
    let height = symbol_height(pins);
    let top_y = height / 2.0 - PIN_PITCH_MM;
    let mut geometry = Vec::new();

    for (index, pin) in pins.iter().enumerate() {
        let left_side = index < split;
        let side_index = if left_side { index } else { index - split };
        let y = top_y - side_index as f64 * PIN_PITCH_MM;
        if left_side {
            geometry.push(SymbolPinGeometry {
                number: pin.number.clone(),
                name: pin.name.clone(),
                x_mm: -half_width - PIN_LENGTH_MM,
                y_mm: y,
                orientation_deg: 0,
                label_x_mm: -half_width - PIN_LENGTH_MM * 2.0,
                label_y_mm: y,
                label_orientation_deg: 180,
            });
        } else {
            geometry.push(SymbolPinGeometry {
                number: pin.number.clone(),
                name: pin.name.clone(),
                x_mm: half_width + PIN_LENGTH_MM,
                y_mm: y,
                orientation_deg: 180,
                label_x_mm: half_width + PIN_LENGTH_MM * 2.0,
                label_y_mm: y,
                label_orientation_deg: 0,
            });
        }
    }
    geometry
}

fn symbol_height(pins: &[CapturePin]) -> f64 {
    let pins_per_side = pins.len().div_ceil(2).max(1);
    (pins_per_side as f64 + 1.0) * PIN_PITCH_MM
}

fn symbol_name(reference: &str) -> String {
    format!("LF_CAPTURE_{}", sanitize_symbol_id(reference))
}

fn sanitize_symbol_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn reference_prefix(reference: &str) -> String {
    reference
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic() || *ch == '_')
        .collect::<String>()
}

fn reference_sort_key(reference: &str) -> (String, u32, String) {
    let prefix = reference
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic() || *ch == '_')
        .collect::<String>();
    let digits = reference
        .chars()
        .skip_while(|ch| ch.is_ascii_alphabetic() || *ch == '_')
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    let number = digits.parse::<u32>().unwrap_or(u32::MAX);
    (prefix, number, reference.to_string())
}

fn symbol_path_uuid(index: usize) -> String {
    format!("b1000000-0000-4000-8000-{:012x}", index + 1)
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn fmt(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    if (rounded.fract()).abs() < 0.000_001 {
        format!("{rounded:.0}")
    } else {
        format!("{rounded:.3}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

impl UuidCounter {
    fn next(&mut self) -> String {
        self.next_id += 1;
        format!("b2000000-0000-4000-8000-{:012x}", self.next_id)
    }

    fn reserve(&mut self, count: u64) {
        self.next_id += count;
    }
}
