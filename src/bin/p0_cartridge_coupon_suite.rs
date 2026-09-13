use clap::Parser;
use laminarforge_cad::p0_cartridge_coupons::{
    descriptors, MaterialStack, Parameters, REVISION, SOURCE_ARTIFACTS, SUITE_ID, TICKET_ID,
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
    #[arg(long, default_value = "models/p0_cartridge_coupons.toml")]
    config: PathBuf,
    /// Material stack slug or `all`.
    #[arg(long, default_value = "all")]
    stack: String,

    /// Output directory for generated STL and JSON files.
    #[arg(long, default_value = DEFAULT_OUTPUT_DIR)]
    output_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct RuntimeManifest<'a> {
    generator_sha256: String,
    parameters: &'a Parameters,
    config_sha256: String,
    stack_selection: &'a str,
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
    mesh: serde_json::Value,
    preview_sha256: String,
    kind: &'static str,
    family: Option<String>,
    stack: Option<String>,
    path: String,
    bytes: u64,
    sha256: String,
}

fn main() {
    let args = Args::parse();
    let (parameters, config_sha256) =
        Parameters::load(&args.config).expect("invalid P0 runtime configuration");
    let selected_stacks = parameters
        .select_stacks(&args.stack)
        .expect("invalid stack selection");
    fs::create_dir_all(&args.output_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create P0 cartridge coupon output directory {}: {error}",
            args.output_dir.display()
        )
    });

    let manifest_path = args.output_dir.join("manifest.json");
    match fs::remove_file(&manifest_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => panic!("cannot invalidate old manifest: {e}"),
    }
    let mut outputs = Vec::new();
    for stack in &selected_stacks {
        for descriptor in descriptors() {
            let filename = format!("{}_{}.stl", descriptor.family.slug(), stack.slug);
            let path = args.output_dir.join(filename);
            let part = parameters.build_coupon(descriptor.family, *stack);
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
        &parameters.build_alignment_nest(),
        &nest_path,
        "reusable_fixture",
        None,
        None,
        &mut outputs,
    );

    let runtime_manifest = RuntimeManifest {
        generator_sha256: laminarforge_cad::runtime_cad::hash(
            &fs::read(std::env::current_exe().expect("executable path")).expect("executable bytes"),
        ),
        parameters: &parameters,
        config_sha256,
        stack_selection: &args.stack,
        schema_version: "2",
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
            "runtime-configured common coupon frame; see parameters",
        ],
        outputs: &outputs,
    };
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
    let mesh = laminarforge_cad::runtime_cad::mesh(&bytes).expect("invalid mesh");
    let preview = laminarforge_cad::runtime_cad::preview(&bytes).expect("preview failed");
    fs::write(path.with_extension("preview.svg"), &preview).expect("write preview");
    outputs.push(OutputRecord {
        mesh,
        preview_sha256: laminarforge_cad::runtime_cad::hash(preview.as_bytes()),
        kind,
        family: family.map(str::to_owned),
        stack: stack.map(str::to_owned),
        path: path_text.into_owned(),
        bytes: bytes.len() as u64,
        sha256,
    });
    println!("Exported: {}", path.display());
}
