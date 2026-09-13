use clap::Parser;
use laminarforge_cad::heating_platen::{build, preview_svg, verify_mesh, Args};
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (parameters, config_hash) = args.load()?;
    let bytes = std::fs::read(args.output_dir.join("heating_block.stl"))?;
    let stats = verify_mesh(&bytes, &parameters)?;
    if std::fs::read_to_string(args.output_dir.join("heating_block.preview.svg"))?
        != preview_svg(&bytes, &parameters)?
    {
        return Err("preview does not match the generated mesh".into());
    }
    let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(
        args.output_dir.join("heating_block.manifest.json"),
    )?)?;
    if manifest["config_sha256"] != config_hash
        || manifest["parameters"] != serde_json::to_value(&parameters)?
        || manifest["stl_sha256"] != format!("{:x}", Sha256::digest(&bytes))
        || manifest["mesh"] != stats
    {
        return Err("publication does not match configuration or mesh".into());
    }
    let temporary = args
        .output_dir
        .join(format!("heating_block.verify.{}.stl", std::process::id()));
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        build(&parameters)?.write_stl(temporary.to_str().ok_or("output path must be UTF-8")?)?;
        if bytes != std::fs::read(&temporary)? {
            return Err("STL differs from geometry regenerated with this configuration".into());
        }
        Ok(())
    })();
    if temporary.exists() {
        std::fs::remove_file(temporary)?;
    }
    result?;
    println!(
        "Heating platen verified: configuration, mesh envelope, hash and deterministic geometry"
    );
    Ok(())
}
