use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;

const PARTS_PATH: &str = "pcb/lamp_rev_b_controller/parts.toml";
const PLACEMENT_PATH: &str = "pcb/lamp_rev_b_controller/placement.toml";
const FAB_CONFIG_PATH: &str = "pcb/lamp_rev_b_controller/fab_release.toml";
const OUTPUT_DIR: &str = "pcb/lamp_rev_b_controller/fab";

#[derive(Debug, Deserialize)]
struct PartsManifest {
    selected_parts: Vec<SelectedPart>,
}

#[derive(Debug, Deserialize)]
struct SelectedPart {
    id: String,
    quantity: u32,
    value: String,
    footprint: String,
    lcsc_part: String,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    placements: Vec<Placement>,
}

#[derive(Debug, Deserialize)]
struct Placement {
    reference: String,
    part_id: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct FabConfig {
    assembly: AssemblyConfig,
}

#[derive(Debug, Default, Deserialize)]
struct AssemblyConfig {
    #[serde(default)]
    manual_part_ids: BTreeSet<String>,
    #[serde(default)]
    no_substitution_part_ids: BTreeSet<String>,
    #[serde(default)]
    dnp_alternates: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let parts = read_toml::<PartsManifest>(&root.join(PARTS_PATH))?;
    let placement = read_toml::<PlacementPlan>(&root.join(PLACEMENT_PATH))?;
    let fab_config = read_toml::<FabConfig>(&root.join(FAB_CONFIG_PATH))?;
    validate(&parts, &placement, &fab_config)?;

    let output_dir = root.join(OUTPUT_DIR);
    fs::create_dir_all(&output_dir)?;
    write_bom(&parts, &placement, &fab_config, &output_dir.join("bom.csv"))?;
    write_cpl(&parts, &placement, &fab_config, &output_dir.join("cpl.csv"))?;

    println!("Wrote LAMP Rev B controller fab preview:");
    println!("  {}/bom.csv", OUTPUT_DIR);
    println!("  {}/cpl.csv", OUTPUT_DIR);
    println!(
        "  manual/THT groups: {}",
        fab_config.assembly.manual_part_ids.len()
    );
    if !fab_config.assembly.dnp_alternates.is_empty() {
        println!("  DNP alternates:");
        for note in &fab_config.assembly.dnp_alternates {
            println!("    - {note}");
        }
    }
    println!("Preview only: complete routing and pass release ERC/DRC before order release.");
    Ok(())
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn validate(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    fab_config: &FabConfig,
) -> Result<(), Box<dyn Error>> {
    let part_ids: BTreeSet<&str> = parts
        .selected_parts
        .iter()
        .map(|part| part.id.as_str())
        .collect();
    let mut by_part: BTreeMap<&str, u32> = BTreeMap::new();
    let mut refs = BTreeSet::new();
    let mut errors = Vec::new();

    for item in &placement.placements {
        if !refs.insert(item.reference.as_str()) {
            errors.push(format!("duplicate placement reference {}", item.reference));
        }
        if !part_ids.contains(item.part_id.as_str()) {
            errors.push(format!(
                "placement {} references unknown part group {}",
                item.reference, item.part_id
            ));
        }
        if !item.x_mm.is_finite() || !item.y_mm.is_finite() || !item.rotation_deg.is_finite() {
            errors.push(format!(
                "placement {} has non-finite coordinate or rotation",
                item.reference
            ));
        }
        if item.side != "top" {
            errors.push(format!(
                "placement {} is {}, but Rev B controller fab preview supports top-side assembly only",
                item.reference, item.side
            ));
        }
        *by_part.entry(item.part_id.as_str()).or_default() += 1;
    }

    for part in &parts.selected_parts {
        let count = by_part.get(part.id.as_str()).copied().unwrap_or_default();
        if count != part.quantity {
            errors.push(format!(
                "part group {} expects {} placements but found {}",
                part.id, part.quantity, count
            ));
        }
    }
    for manual_id in &fab_config.assembly.manual_part_ids {
        if !part_ids.contains(manual_id.as_str()) {
            errors.push(format!(
                "manual assembly part id {manual_id} is not in selected_parts"
            ));
        }
    }
    for no_sub_id in &fab_config.assembly.no_substitution_part_ids {
        if !part_ids.contains(no_sub_id.as_str()) {
            errors.push(format!(
                "no-substitution part id {no_sub_id} is not in selected_parts"
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn write_bom(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    fab_config: &FabConfig,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record([
        "Comment",
        "Designator",
        "Footprint",
        "LCSC Part #",
        "Assembly",
        "Notes",
    ])?;

    for part in &parts.selected_parts {
        let mut placements = by_part
            .remove(part.id.as_str())
            .unwrap_or_else(|| panic!("missing placement group {}", part.id));
        placements.sort_by_key(|item| reference_order(&item.reference));
        let designators = placements
            .iter()
            .map(|item| item.reference.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let assembly = if fab_config.assembly.manual_part_ids.contains(&part.id) {
            "Manual/THT"
        } else {
            "SMT"
        };
        let mut notes = Vec::new();
        if fab_config
            .assembly
            .no_substitution_part_ids
            .contains(&part.id)
        {
            notes.push("no substitution");
        }
        if fab_config.assembly.manual_part_ids.contains(&part.id) {
            notes.push("hand-place or vendor-confirmed manual assembly");
        }
        writer.write_record([
            part.value.as_str(),
            designators.as_str(),
            part.footprint.as_str(),
            part.lcsc_part.as_str(),
            assembly,
            notes.join("; ").as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_cpl(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    fab_config: &FabConfig,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let part_by_id = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    let mut placements = placement
        .placements
        .iter()
        .filter(|item| !fab_config.assembly.manual_part_ids.contains(&item.part_id))
        .collect::<Vec<_>>();
    placements.sort_by_key(|item| reference_order(&item.reference));

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["Designator", "Mid X", "Mid Y", "Layer", "Rotation"])?;
    for item in placements {
        if !part_by_id.contains_key(item.part_id.as_str()) {
            return Err(format!(
                "placement {} references unknown part group {}",
                item.reference, item.part_id
            )
            .into());
        }
        writer.write_record([
            item.reference.as_str(),
            format!("{:.3}", item.x_mm).as_str(),
            format!("{:.3}", item.y_mm).as_str(),
            "TopLayer",
            format!("{:.3}", item.rotation_deg).as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn reference_order(reference: &str) -> (String, u32, String) {
    let prefix = reference
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .collect::<String>();
    let digits = reference
        .chars()
        .skip_while(|ch| ch.is_ascii_alphabetic())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    let number = digits.parse::<u32>().unwrap_or_default();
    (prefix, number, reference.to_string())
}
