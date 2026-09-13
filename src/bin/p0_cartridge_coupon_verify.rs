use clap::Parser;
use laminarforge_cad::p0_cartridge_coupons::{
    descriptors, CouponFamily, Parameters, REVISION, SOURCE_ARTIFACTS, SUITE_ID, TICKET_ID,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "models/p0_cartridge_coupons.toml")]
    config: PathBuf,
    #[arg(long, default_value = "all")]
    stack: String,
    #[arg(long, default_value = "output/p0_cartridge_coupons")]
    output_dir: PathBuf,
}
#[derive(Deserialize)]
struct Output {
    mesh: serde_json::Value,
    preview_sha256: String,
    kind: String,
    family: Option<String>,
    stack: Option<String>,
    path: PathBuf,
    bytes: u64,
    sha256: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (parameters, config_sha256) = Parameters::load(&args.config)?;
    let stacks = parameters.select_stacks(&args.stack)?;
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(args.output_dir.join("manifest.json"))?)?;
    for (key, expected) in [
        ("schema_version", serde_json::json!("2")),
        ("suite_id", serde_json::json!(SUITE_ID)),
        ("revision", serde_json::json!(REVISION)),
        ("ticket", serde_json::json!(TICKET_ID)),
        ("source_artifacts", serde_json::json!(SOURCE_ARTIFACTS)),
        ("parameters", serde_json::to_value(&parameters)?),
        ("material_stacks", serde_json::to_value(&stacks)?),
        ("families", serde_json::to_value(descriptors())?),
        ("stack_selection", serde_json::json!(args.stack)),
        ("config_sha256", serde_json::json!(config_sha256)),
    ] {
        if manifest[key] != expected {
            return Err(format!("P0 manifest {key} does not match requested configuration").into());
        }
    }
    if !manifest["scope"]
        .as_str()
        .is_some_and(|s| s.contains("dry"))
        || !manifest["design_status"]
            .as_str()
            .is_some_and(|s| s.contains("not validated"))
        || manifest["shared_interfaces"].as_array().map(Vec::len) != Some(6)
    {
        return Err("P0 manifest claim/interface contract differs".into());
    }
    let outputs: Vec<Output> = serde_json::from_value(manifest["outputs"].clone())?;
    let mut expected = BTreeMap::new();
    for stack in &stacks {
        for family in CouponFamily::ALL {
            expected.insert(
                format!("{}_{}.stl", family.slug(), stack.slug),
                (Some(family), Some(*stack)),
            );
        }
    }
    expected.insert("shared_3_2_1_alignment_nest.stl".into(), (None, None));
    if outputs.len() != expected.len() {
        return Err("P0 output matrix incomplete".into());
    }
    for output in &outputs {
        let name = output
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid output name")?;
        let (family, stack) = expected
            .remove(name)
            .ok_or("duplicate or unexpected output")?;
        if output.path != args.output_dir.join(name)
            || output.family != family.map(|f| f.slug().to_owned())
            || output.stack != stack.map(|s| s.slug.to_owned())
            || output.kind
                != if family.is_some() {
                    "disposable_coupon"
                } else {
                    "reusable_fixture"
                }
        {
            return Err("P0 output identity differs".into());
        }
        let bytes = fs::read(&output.path)?;
        if bytes.len() < 84
            || bytes.len() as u64 != output.bytes
            || format!("{:x}", Sha256::digest(&bytes)) != output.sha256
        {
            return Err("P0 output hash/size differs".into());
        }
        if laminarforge_cad::runtime_cad::mesh(&bytes)? != output.mesh {
            return Err("mesh evidence differs".into());
        }
        let preview = laminarforge_cad::runtime_cad::preview(&bytes)?;
        if fs::read_to_string(output.path.with_extension("preview.svg"))? != preview
            || laminarforge_cad::runtime_cad::hash(preview.as_bytes()) != output.preview_sha256
        {
            return Err("preview evidence differs".into());
        }
        let part = match (family, stack) {
            (Some(f), Some(s)) => parameters.build_coupon(f, s),
            (None, None) => parameters.build_alignment_nest(),
            _ => unreachable!(),
        };
        if part.to_stl()? != bytes {
            return Err(format!("geometry differs from runtime configuration: {name}").into());
        }
    }
    println!("P0 coupon verification passed: {} outputs, shared runtime configuration, exact geometry and hashes",outputs.len());
    Ok(())
}
