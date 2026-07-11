use laminarforge_cad::stl_to_step;
use laminarforge_cad::swab_integrated_sealed_diagnostic_cartridge::{
    build_publication_components, composite_stl_bytes, design_manifest, verify_design,
    CartridgeParams, PUBLICATION_STEM, REVISION, TICKET_ID,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
struct RuntimePublicationManifest {
    design: laminarforge_cad::swab_integrated_sealed_diagnostic_cartridge::DesignManifest,
    outputs: Vec<OutputEvidence>,
    verification: [&'static str; 7],
}

#[derive(Debug, Serialize)]
struct OutputEvidence {
    path: String,
    bytes: u64,
    sha256: String,
}

fn main() {
    let params = CartridgeParams::default();
    verify_design(params).expect("integrated cartridge architecture verification failed");
    fs::create_dir_all("output").expect("failed to create root output directory");

    let stl_path = PathBuf::from(format!("output/{PUBLICATION_STEM}.stl"));
    let stp_path = PathBuf::from(format!("output/{PUBLICATION_STEM}.stp"));
    let manifest_path = PathBuf::from(format!("output/{PUBLICATION_STEM}.manifest.json"));
    remove_stale(&stl_path);
    remove_stale(&stp_path);
    remove_stale(&manifest_path);

    let components = build_publication_components(params);
    let stl = composite_stl_bytes(&components)
        .unwrap_or_else(|error| panic!("failed to encode integrated STL: {error}"));
    fs::write(&stl_path, stl)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", stl_path.display()));
    require_nonempty(&stl_path);

    let stl_text = stl_path.to_string_lossy();
    stl_to_step(stl_text.as_ref());
    require_nonempty(&stp_path);

    let runtime = RuntimePublicationManifest {
        design: design_manifest(params),
        outputs: vec![evidence(&stl_path), evidence(&stp_path)],
        verification: [
            "single acyclic irreversible liquid topology reaches terminal waste",
            "vent exterior receives only the physically separated gas-only path",
            "two dry plungers actuate one wash pouch and paired independent reaction-fill lobes",
            "wash and reaction-fill idealized pouch volumes match proposed exact-dose envelopes",
            "waste headspace is at least 20 percent and proposed pad capacity exceeds 2x delivery",
            "all wet plumbing remains in the sealed disposable",
            "prohibited alternate, gel, evaporation, waste-heat, and reusable-pump features are absent",
        ],
    };
    let mut bytes = serde_json::to_vec_pretty(&runtime)
        .expect("runtime publication manifest serialization must succeed");
    bytes.push(b'\n');
    fs::write(&manifest_path, bytes)
        .unwrap_or_else(|error| panic!("failed to write {}: {error}", manifest_path.display()));

    println!("{PUBLICATION_STEM} {REVISION}");
    println!("  Ticket:                 {TICKET_ID}");
    println!("  Architecture:           single sealed two-plunger path");
    println!("  STL:                    {}", stl_path.display());
    println!("  STP:                    {}", stp_path.display());
    println!("  Runtime manifest:       {}", manifest_path.display());
    println!("  Validation status:      proposed engineering geometry only");
}

fn remove_stale(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove stale {}: {error}", path.display()),
    }
}

fn require_nonempty(path: &Path) {
    let bytes = fs::metadata(path)
        .unwrap_or_else(|error| panic!("required output {} is missing: {error}", path.display()))
        .len();
    assert!(bytes > 0, "required output {} is empty", path.display());
}

fn evidence(path: &Path) -> OutputEvidence {
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("failed to read generated {}: {error}", path.display()));
    OutputEvidence {
        path: path.to_string_lossy().into_owned(),
        bytes: bytes.len() as u64,
        sha256: format!("{:x}", Sha256::digest(&bytes)),
    }
}
