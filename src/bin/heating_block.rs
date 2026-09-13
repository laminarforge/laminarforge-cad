use clap::Parser;
use laminarforge_cad::heating_platen::{build, preview_svg, publication, verify_mesh, Args};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (parameters, config_hash) = args.load()?;
    let model = build(&parameters)?;
    std::fs::create_dir_all(&args.output_dir)?;
    let manifest = args.output_dir.join("heating_block.manifest.json");
    if manifest.exists() {
        std::fs::remove_file(&manifest)?;
    }
    let stl = args.output_dir.join("heating_block.stl");
    model.write_stl(stl.to_str().ok_or("output path must be UTF-8")?)?;
    let bytes = std::fs::read(&stl)?;
    verify_mesh(&bytes, &parameters)?;
    std::fs::write(
        args.output_dir.join("heating_block.preview.svg"),
        preview_svg(&bytes, &parameters)?,
    )?;
    let evidence = publication(&parameters, &config_hash, &bytes)?;
    std::fs::write(manifest, serde_json::to_vec_pretty(&evidence)?)?;
    println!("Generated and checked {}", stl.display());
    Ok(())
}
