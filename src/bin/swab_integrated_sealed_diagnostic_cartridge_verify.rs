use laminarforge_cad::swab_integrated_sealed_diagnostic_cartridge::{
    build_publication_components, composite_stl_bytes, verify_design, CartridgeParams,
    PUBLICATION_STEM, REQUIRED_FEATURES, SOURCE_ARTIFACTS, TICKET_ID,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const TRACKED_MANIFEST: &str = "docs/swab_integrated_sealed_diagnostic_cartridge_manifest.json";

#[derive(Debug, Deserialize)]
struct TrackedManifest {
    publication_stem: String,
    ticket: String,
    source_artifacts: Vec<String>,
    architecture_count: usize,
    required_features: Vec<String>,
    excluded_features: Vec<String>,
    output_contract: Vec<String>,
    claim_boundary: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeManifest {
    design: RuntimeDesign,
    outputs: Vec<RuntimeOutput>,
    verification: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RuntimeDesign {
    publication_stem: String,
    ticket: String,
    source_artifacts: Vec<String>,
    architecture: String,
    design_status: String,
    reusable_reader_wet_interfaces: usize,
    disposable_liquid_outlets: usize,
    derived: RuntimeDerived,
}

#[derive(Debug, Deserialize)]
struct RuntimeDerived {
    waste_reserved_headspace_fraction: f64,
    pad_capacity_to_maximum_delivery_ratio: f64,
}

#[derive(Debug, Deserialize)]
struct RuntimeOutput {
    path: String,
    bytes: u64,
    sha256: String,
}

fn main() {
    let params = CartridgeParams::default();
    verify_design(params).expect("default architecture invariants failed");
    verify_tracked_manifest();

    let runtime_path = PathBuf::from(format!("output/{PUBLICATION_STEM}.manifest.json"));
    let runtime: RuntimeManifest = read_json(&runtime_path);
    assert_eq!(runtime.design.publication_stem, PUBLICATION_STEM);
    assert_eq!(runtime.design.ticket, TICKET_ID);
    assert_eq!(runtime.design.source_artifacts, SOURCE_ARTIFACTS);
    assert!(runtime.design.architecture.contains("one irreversible"));
    assert!(runtime.design.design_status.contains("no manufacturing"));
    assert_eq!(runtime.design.reusable_reader_wet_interfaces, 0);
    assert_eq!(runtime.design.disposable_liquid_outlets, 0);
    assert!(runtime.design.derived.waste_reserved_headspace_fraction >= 0.20);
    assert!(
        runtime
            .design
            .derived
            .pad_capacity_to_maximum_delivery_ratio
            >= 2.0
    );
    assert_eq!(runtime.verification.len(), 7);

    let expected_paths = [
        format!("output/{PUBLICATION_STEM}.stl"),
        format!("output/{PUBLICATION_STEM}.stp"),
    ];
    assert_eq!(runtime.outputs.len(), expected_paths.len());
    for (output, expected_path) in runtime.outputs.iter().zip(&expected_paths) {
        assert_eq!(&output.path, expected_path);
        verify_output(output);
    }

    verify_repeatable_stl(&expected_paths[0]);

    println!("Integrated sealed swab cartridge verification passed");
    println!("  Publication stem:      {PUBLICATION_STEM}");
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
    println!("  Verified root outputs: {}", runtime.outputs.len());
    println!("  Deterministic STL:     byte-identical rebuild");
    println!("  Wet reader interfaces: 0");
    println!("  Claim boundary:        proposed engineering geometry only");
}

fn verify_tracked_manifest() {
    let manifest: TrackedManifest = read_json(Path::new(TRACKED_MANIFEST));
    assert_eq!(manifest.publication_stem, PUBLICATION_STEM);
    assert_eq!(manifest.ticket, TICKET_ID);
    assert_eq!(manifest.source_artifacts, SOURCE_ARTIFACTS);
    assert_eq!(manifest.architecture_count, 1);
    assert_eq!(
        manifest
            .required_features
            .into_iter()
            .collect::<BTreeSet<_>>(),
        REQUIRED_FEATURES.into_iter().map(str::to_owned).collect()
    );
    assert_eq!(manifest.excluded_features.len(), 7);
    assert_eq!(manifest.output_contract.len(), 3);
    assert!(manifest.claim_boundary.contains("not validated"));
}

fn verify_output(output: &RuntimeOutput) {
    let path = Path::new(&output.path);
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("missing required root output {}: {error}", path.display()));
    assert!(
        bytes.len() > 84,
        "output {} is unexpectedly small",
        path.display()
    );
    assert_eq!(bytes.len() as u64, output.bytes);
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), output.sha256);
}

fn verify_repeatable_stl(expected_path: &str) {
    let expected = fs::read(expected_path).expect("failed reading publication STL");
    let repeated = composite_stl_bytes(&build_publication_components(CartridgeParams::default()))
        .expect("failed deterministic STL rebuild");
    assert_eq!(expected, repeated, "repeated geometry export changed bytes");
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed reading {}: {error}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("invalid JSON in {}: {error}", path.display()))
}
