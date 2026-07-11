use laminarforge_cad::p0_cartridge_coupons::{
    CouponFamily, MATERIAL_STACKS, REVISION, SOURCE_ARTIFACTS, SUITE_ID, TICKET_ID,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const MANIFEST_PATH: &str = "output/p0_cartridge_coupons/manifest.json";

#[derive(Debug, Deserialize)]
struct RuntimeManifest {
    schema_version: String,
    suite_id: String,
    revision: String,
    ticket: String,
    source_artifacts: Vec<String>,
    scope: String,
    design_status: String,
    material_stacks: Vec<RuntimeStack>,
    families: Vec<RuntimeFamily>,
    shared_interfaces: Vec<String>,
    outputs: Vec<RuntimeOutput>,
}

#[derive(Debug, Deserialize)]
struct RuntimeStack {
    slug: String,
    base_thickness_mm: f64,
    spacer_thickness_mm: f64,
    cover_thickness_mm: f64,
}

#[derive(Debug, Deserialize)]
struct RuntimeFamily {
    family: String,
    coupon_id: String,
    purpose: String,
    conditional: bool,
}

#[derive(Debug, Deserialize)]
struct RuntimeOutput {
    kind: String,
    family: Option<String>,
    stack: Option<String>,
    path: String,
    bytes: u64,
    sha256: String,
}

fn main() {
    let raw = fs::read_to_string(MANIFEST_PATH).unwrap_or_else(|error| {
        panic!(
            "missing generated P0 coupon manifest {MANIFEST_PATH}: {error}; run p0_cartridge_coupon_suite first"
        )
    });
    let manifest: RuntimeManifest = serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("invalid generated P0 coupon manifest: {error}"));

    assert_eq!(manifest.schema_version, "1");
    assert_eq!(manifest.suite_id, SUITE_ID);
    assert_eq!(manifest.revision, REVISION);
    assert_eq!(manifest.ticket, TICKET_ID);
    assert_eq!(manifest.source_artifacts, SOURCE_ARTIFACTS);
    assert!(manifest.scope.contains("dry"));
    assert!(manifest.design_status.contains("not validated"));
    assert_eq!(manifest.material_stacks.len(), MATERIAL_STACKS.len());
    assert_eq!(manifest.families.len(), CouponFamily::ALL.len());
    assert_eq!(manifest.shared_interfaces.len(), 6);

    let stack_slugs: BTreeSet<_> = manifest
        .material_stacks
        .iter()
        .map(|stack| stack.slug.as_str())
        .collect();
    assert_eq!(stack_slugs.len(), MATERIAL_STACKS.len());
    for stack in &manifest.material_stacks {
        assert!(stack.base_thickness_mm > 0.0);
        assert!(stack.spacer_thickness_mm > 0.0);
        assert!(stack.cover_thickness_mm > 0.0);
    }

    let family_slugs: BTreeSet<_> = manifest
        .families
        .iter()
        .map(|family| family.family.as_str())
        .collect();
    assert_eq!(family_slugs.len(), CouponFamily::ALL.len());
    for family in &manifest.families {
        assert!(!family.coupon_id.is_empty());
        assert!(!family.purpose.is_empty());
        assert_eq!(
            family.conditional,
            family.family == "conditional_blister",
            "only the blister family is conditional"
        );
    }

    let expected_output_count = MATERIAL_STACKS.len() * CouponFamily::ALL.len() + 1;
    assert_eq!(manifest.outputs.len(), expected_output_count);
    let mut paths = BTreeSet::new();
    let mut disposable_count = 0;
    let mut fixture_count = 0;
    for output in &manifest.outputs {
        assert!(paths.insert(output.path.as_str()), "duplicate output path");
        let path = Path::new(&output.path);
        assert!(path.exists(), "missing generated output {}", path.display());
        let bytes = fs::read(path)
            .unwrap_or_else(|error| panic!("failed reading {}: {error}", path.display()));
        assert!(bytes.len() > 84, "STL output is unexpectedly small");
        assert_eq!(bytes.len() as u64, output.bytes);
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), output.sha256);
        assert!(output.path.ends_with(".stl"));
        match output.kind.as_str() {
            "disposable_coupon" => {
                disposable_count += 1;
                assert!(output.family.is_some());
                assert!(output.stack.is_some());
            }
            "reusable_fixture" => {
                fixture_count += 1;
                assert!(output.family.is_none());
                assert!(output.stack.is_none());
            }
            other => panic!("unexpected output kind {other}"),
        }
    }
    assert_eq!(
        disposable_count,
        MATERIAL_STACKS.len() * CouponFamily::ALL.len()
    );
    assert_eq!(fixture_count, 1);

    println!("P0 cartridge coupon verification passed");
    println!("  Manifest:             {MANIFEST_PATH}");
    println!("  Material stacks:      {}", manifest.material_stacks.len());
    println!("  Coupon families:      {}", manifest.families.len());
    println!("  Verified STL hashes:  {}", manifest.outputs.len());
    println!("  Reusable fixtures:    {fixture_count}");
    println!("  Scope:                dry engineering geometry only");
}
