use clap::Parser;
use laminarforge_cad::p0_cartridge_coupons::{
    build_alignment_nest, build_coupon, descriptors, MaterialStack, MATERIAL_STACKS, REVISION,
    SOURCE_ARTIFACTS, SUITE_ID, TICKET_ID,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use vcad::Part;

const DEFAULT_OUTPUT_DIR: &str = "output/p0_cartridge_coupons";

#[derive(Debug, Parser)]
#[command(
    name = "p0_cartridge_coupon_suite",
    about = "Generate dry parametric P0 cartridge engineering coupons"
)]
struct Args {
    /// Material stack slug or `all`.
    #[arg(long, default_value = "all")]
    stack: String,

    /// Output directory for generated STL and JSON files.
    #[arg(long, default_value = DEFAULT_OUTPUT_DIR)]
    output_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct RuntimeManifest<'a> {
    schema_version: &'static str,
    suite_id: &'static str,
    revision: &'static str,
    ticket: &'static str,
    source_artifacts: [&'static str; 3],
    scope: &'static str,
    design_status: &'static str,
    material_stacks: Vec<MaterialStack>,
    families: Vec<laminarforge_cad::p0_cartridge_coupons::CouponDescriptor>,
    shared_interfaces: [&'static str; 6],
    outputs: &'a [OutputRecord],
}

#[derive(Debug, Serialize)]
struct OutputRecord {
    kind: &'static str,
    family: Option<String>,
    stack: Option<String>,
    path: String,
    bytes: u64,
    sha256: String,
}

fn main() {
    let args = Args::parse();
    let selected_stacks = select_stacks(&args.stack);
    fs::create_dir_all(&args.output_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create P0 cartridge coupon output directory {}: {error}",
            args.output_dir.display()
        )
    });

    let mut outputs = Vec::new();
    for stack in &selected_stacks {
        for descriptor in descriptors() {
            let filename = format!("{}_{}.stl", descriptor.family.slug(), stack.slug);
            let path = args.output_dir.join(filename);
            let part = build_coupon(descriptor.family, *stack);
            export_and_record(
                &part,
                &path,
                "disposable_coupon",
                Some(descriptor.family.slug()),
                Some(stack.slug),
                &mut outputs,
            );
        }
    }

    let nest_path = args.output_dir.join("shared_3_2_1_alignment_nest.stl");
    export_and_record(
        &build_alignment_nest(),
        &nest_path,
        "reusable_fixture",
        None,
        None,
        &mut outputs,
    );

    let runtime_manifest = RuntimeManifest {
        schema_version: "1",
        suite_id: SUITE_ID,
        revision: REVISION,
        ticket: TICKET_ID,
        source_artifacts: SOURCE_ARTIFACTS,
        scope: "dry, nonsterile engineering coupons and reusable fixture geometry only",
        design_status:
            "proposed parametric CAD envelopes; not validated tolerances or manufacturing release",
        material_stacks: selected_stacks,
        families: descriptors(),
        shared_interfaces: [
            "3-2-1 datum nest: three primary pads, two secondary contacts, one tertiary stop",
            "round locating hole plus same-width relief slot",
            "asymmetric keyed corner notch",
            "three cross fiducials outside protected optical/fluidic fields",
            "font-independent geometric revision, family-ID, and stack-ID witness bars",
            "common 86 x 54 mm coupon frame",
        ],
        outputs: &outputs,
    };
    let manifest_path = args.output_dir.join("manifest.json");
    let mut json = serde_json::to_vec_pretty(&runtime_manifest)
        .expect("runtime P0 coupon manifest serialization must succeed");
    json.push(b'\n');
    fs::write(&manifest_path, json).unwrap_or_else(|error| {
        panic!(
            "failed to write runtime manifest {}: {error}",
            manifest_path.display()
        )
    });

    println!();
    println!("{SUITE_ID} {REVISION}");
    println!("  Ticket:              {TICKET_ID}");
    println!(
        "  Material stacks:     {}",
        runtime_manifest.material_stacks.len()
    );
    println!("  Coupon families:     {}", runtime_manifest.families.len());
    println!("  STL outputs:         {}", outputs.len());
    println!("  Runtime manifest:    {}", manifest_path.display());
    println!("  Scope:               dry engineering geometry only");
    println!("  Validation status:   none claimed");
}

fn select_stacks(requested: &str) -> Vec<MaterialStack> {
    if requested == "all" {
        return MATERIAL_STACKS.to_vec();
    }
    MATERIAL_STACKS
        .into_iter()
        .filter(|stack| stack.slug == requested)
        .collect::<Vec<_>>()
        .tap_assert_nonempty(requested)
}

trait StackSelectionExt {
    fn tap_assert_nonempty(self, requested: &str) -> Self;
}

impl StackSelectionExt for Vec<MaterialStack> {
    fn tap_assert_nonempty(self, requested: &str) -> Self {
        assert!(
            !self.is_empty(),
            "unknown stack `{requested}`; expected all, coc_cop_target, pmma_control, or pet_comparator"
        );
        self
    }
}

fn export_and_record(
    part: &Part,
    path: &Path,
    kind: &'static str,
    family: Option<&str>,
    stack: Option<&str>,
    outputs: &mut Vec<OutputRecord>,
) {
    let path_text = path.to_string_lossy();
    part.write_stl(path_text.as_ref())
        .unwrap_or_else(|error| panic!("failed to write {}: {error:?}", path.display()));
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("failed to read generated {}: {error}", path.display()));
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    outputs.push(OutputRecord {
        kind,
        family: family.map(str::to_owned),
        stack: stack.map(str::to_owned),
        path: path_text.into_owned(),
        bytes: bytes.len() as u64,
        sha256,
    });
    println!("Exported: {}", path.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_selection_is_all_three_stacks() {
        assert_eq!(select_stacks("all").len(), 3);
    }

    #[test]
    fn exact_stack_selection_is_supported() {
        let selected = select_stacks("pmma_control");
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].slug, "pmma_control");
    }

    #[test]
    #[should_panic(expected = "unknown stack")]
    fn unknown_stack_fails_immediately() {
        let _ = select_stacks("generic_plastic");
    }
}
