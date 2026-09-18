//! Bundle a generated cassette into shop/assembly PDFs and a hash-manifested handoff.
use clap::Parser;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
#[derive(Parser)]
struct Args {
    #[arg(long)]
    input_dir: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    #[arg(long)]
    source_dir: PathBuf,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cmd(program: &str, args: &[String]) -> Result<()> {
    let out = Command::new(program).args(args).output()?;
    if !out.status.success() {
        return Err(format!("{program} failed: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(())
}
fn files(p: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut v = fs::read_dir(p)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    v.retain(|p| p.extension().and_then(|e| e.to_str()) == Some(extension));
    v.sort();
    Ok(v)
}
fn combine(input: &[PathBuf], out: &Path, scratch: &Path) -> Result<()> {
    fs::create_dir_all(scratch)?;
    let mut pdfs = Vec::new();
    for (j, p) in input.iter().enumerate() {
        let pdf = scratch.join(format!("page-{j:03}.pdf"));
        cmd(
            "rsvg-convert",
            &[
                "-f".into(),
                "pdf".into(),
                "-o".into(),
                pdf.display().to_string(),
                p.display().to_string(),
            ],
        )?;
        pdfs.push(pdf.display().to_string());
    }
    pdfs.push(out.display().to_string());
    cmd("pdfunite", &pdfs)?;
    Ok(())
}
fn qa(pdf: &Path, dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?;
    cmd(
        "pdftoppm",
        &[
            "-scale-to".into(),
            "1300".into(),
            "-png".into(),
            pdf.display().to_string(),
            dir.join("page").display().to_string(),
        ],
    )?;
    for (group, paths) in files(dir, "png")?.chunks(6).enumerate() {
        let mut sheet = image::RgbImage::from_pixel(1800, 1800, image::Rgb([230, 235, 239]));
        for (j, p) in paths.iter().enumerate() {
            let page = image::open(p)?.thumbnail(590, 880).to_rgb8();
            image::imageops::overlay(
                &mut sheet,
                &page,
                (j % 3 * 600 + 5) as i64,
                (j / 3 * 900 + 5) as i64,
            );
        }
        sheet.save(dir.join(format!("contact-{group:02}.png")))?;
    }
    Ok(())
}
fn walk(
    root: &Path,
    p: &Path,
    hashes: &mut std::collections::BTreeMap<String, String>,
) -> Result<()> {
    for e in fs::read_dir(p)? {
        let p = e?.path();
        if p.is_dir() {
            walk(root, &p, hashes)?;
        } else {
            hashes.insert(
                p.strip_prefix(root)?.display().to_string(),
                format!("{:x}", Sha256::digest(fs::read(&p)?)),
            );
        }
    }
    Ok(())
}
// Validate the files that will actually be shipped using a separately installed importer.
fn exchange_check(folder: &Path, evidence: &Path) -> Result<()> {
    fs::create_dir_all(evidence)?;
    let version = Command::new("DRAWEXE")
        .args(["-b", "-c", "pload ALL; puts [dversion]; exit"])
        .output()?;
    if !version.status.success() {
        return Err("independent CAD checker unavailable".into());
    }
    fs::write(evidence.join("checker-version.txt"), &version.stdout)?;
    for path in files(folder, "step")? {
        let raw = path.to_str().ok_or("non UTF-8 CAD path")?;
        if raw.contains(['{', '}', '\n', '\r']) {
            return Err("unsupported CAD path characters".into());
        }
        let script=format!("pload ALL; if {{[catch {{ReadStep D {{{raw}}}; XGetOneShape s D; set result [checkshape s -exact]; puts $result; if {{[string first {{This shape seems to be valid}} $result] < 0}} {{error {{invalid imported shape}}}}; puts [nbshapes s]; puts [vprops s]; puts {{LF_IMPORT_VALID}}}} err]}} {{puts stderr $err; exit 1}}; exit 0");
        let result = Command::new("DRAWEXE")
            .args(["-b", "-c", &script])
            .output()?;
        let log = format!(
            "{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        fs::write(
            evidence.join(format!(
                "{}.txt",
                path.file_stem().unwrap().to_string_lossy()
            )),
            &log,
        )?;
        if !result.status.success() || !log.contains("LF_IMPORT_VALID") {
            return Err(
                format!("Independent STEP validity check failed: {}", path.display()).into(),
            );
        }
        let solid_line = log
            .lines()
            .find(|l| l.trim_start().starts_with("SOLID "))
            .ok_or("missing independent solid count")?;
        if solid_line.split(':').nth(1).map(str::trim) != Some("1") {
            return Err(format!("not one solid: {}", path.display()).into());
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    let a = Args::parse();
    if a.output_dir.exists() {
        return Err("output directory already exists; preserve release candidates".into());
    }
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(a.input_dir.join("verification.json"))?)?;
    if report["status"] != "V0_WATER_TEST_PROTOTYPE_FABRICATION_PACKAGE" {
        return Err("input is not a verified fabrication generation".into());
    }
    for (name, expected) in report["source_dependencies_sha256"]
        .as_object()
        .ok_or("missing source hashes")?
    {
        let actual = format!("{:x}", Sha256::digest(fs::read(a.source_dir.join(name))?));
        if Some(actual.as_str()) != expected.as_str() {
            return Err(format!("source mismatch: {name}").into());
        }
    }
    let m = a.output_dir.join("manufacturing");
    let assembly = a.output_dir.join("assembly");
    let evidence = a.output_dir.join("verification");
    let source = a.output_dir.join("source");
    for p in [&m, &assembly, &evidence, &source] {
        fs::create_dir_all(p)?;
    }
    let optional = a.output_dir.join("optional-materials");
    fs::create_dir_all(&optional)?;
    let index: Vec<serde_json::Value> =
        serde_json::from_slice(&fs::read(a.input_dir.join("manufacturing-index.json"))?)?;
    for row in &index {
        let name = row["name"].as_str().ok_or("missing part name")?;
        let filename = format!("{name}.step");
        let target = if row["optional"] == true {
            &optional
        } else {
            &m
        };
        fs::copy(a.input_dir.join(&filename), target.join(&filename))?;
    }
    for p in files(&a.input_dir, "dxf")? {
        fs::copy(&p, optional.join(p.file_name().unwrap()))?;
    }
    for p in files(&a.input_dir, "csv")? {
        fs::copy(
            &p,
            if p.file_name().unwrap() == "manufacturing-bom.csv" {
                m.join(p.file_name().unwrap())
            } else if p
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("thermal-")
            {
                evidence.join(p.file_name().unwrap())
            } else {
                assembly.join(p.file_name().unwrap())
            },
        )?;
    }
    for name in ["verification.json", "thermal-sizing.json", "config.toml"] {
        fs::copy(a.input_dir.join(name), evidence.join(name))?;
    }
    for name in [
        "assembly_closed.stl",
        "assembly_open.stl",
        "design-review.svg",
    ] {
        fs::copy(a.input_dir.join(name), assembly.join(name))?;
    }
    cmd(
        "rsvg-convert",
        &[
            "-o".into(),
            assembly.join("assembly-overview.png").display().to_string(),
            a.input_dir.join("design-review.svg").display().to_string(),
        ],
    )?;
    for p in files(&a.source_dir.join("docs/heated-cassette-v0"), "md")? {
        fs::copy(&p, assembly.join(p.file_name().unwrap()))?;
    }
    fs::copy(
        a.source_dir
            .join("docs/heated-cassette-v0/manufacturing-notes.md"),
        m.join("manufacturing-notes.md"),
    )?;
    let mut drawings = files(&a.input_dir.join("manufacturing-pages"), "svg")?;
    drawings.extend(files(&a.input_dir.join("drawing-pages"), "svg")?);
    let qa_dir = a.output_dir.with_extension("qa");
    fs::create_dir_all(&qa_dir)?;
    combine(
        &files(&a.input_dir.join("assembly-pages"), "svg")?,
        &a.output_dir.join("assembly-drawings.pdf"),
        &qa_dir.join("assembly-pdf-pages"),
    )?;
    qa(
        &a.output_dir.join("assembly-drawings.pdf"),
        &qa_dir.join("assembly"),
    )?;
    exchange_check(&m, &evidence.join("independent-import"))?;
    exchange_check(&optional, &evidence.join("independent-import-optional"))?;
    combine(
        &drawings,
        &a.output_dir.join("shop-drawings.pdf"),
        &qa_dir.join("shop-pdf-pages"),
    )?;
    combine(
        &files(&a.input_dir.join("manual-pages"), "svg")?,
        &a.output_dir.join("assembly-handbook.pdf"),
        &qa_dir.join("manual-pdf-pages"),
    )?;
    qa(
        &a.output_dir.join("shop-drawings.pdf"),
        &qa_dir.join("shop"),
    )?;
    qa(
        &a.output_dir.join("assembly-handbook.pdf"),
        &qa_dir.join("manual"),
    )?;
    for p in [
        "src/bin/heated_microplate_cassette_v0.rs",
        "src/bin/cassette_release.rs",
        "src/cassette/solid.rs",
        "src/cassette/drawings.rs",
        "src/cassette/assembly_drawings.rs",
        "src/cassette/feature_drawings.rs",
        "src/cassette/thermal.rs",
        "src/cassette/package.rs",
        "models/heated_microplate_cassette_v0.toml",
        "Cargo.toml",
        "Cargo.lock",
    ] {
        let dest = source.join(p);
        fs::create_dir_all(dest.parent().unwrap())?;
        fs::copy(a.source_dir.join(p), dest)?;
    }
    fs::write(source.join("README.txt"),format!("This is a relevant-source snapshot, not a standalone copy of the full CAD monorepo.\nCanonical repository: https://github.com/laminarforge/laminarforge-cad\nCommit: {}\nUse the full checkout at that commit to reproduce.\nRun the local laminarforge_build MCP tool for bin heated_microplate_cassette_v0, profile dev, features [step], with --config models/heated_microplate_cassette_v0.toml and an empty --output-dir.\nThen run bin cassette_release with --input-dir, a new --output-dir, and --source-dir pointing to that checkout.\nPDF tools required: rsvg-convert, pdfunite, pdftoppm. No cloud build.\n",report["git_head"].as_str().unwrap_or("unknown")))?;
    fs::write(a.output_dir.join("START-HERE.txt"),"LaminarForge LF-CAS-V0 Rev C - one manual water-test prototype\n\nSend supplier-RFQ.zip to the CNC shop. It includes manufacturing/, optional-materials/, all three PDFs, the numbered assembly BOM and the procurement BOM.\nUse assembly-handbook.pdf and assembly/procurement-bom.csv to purchase and assemble.\nSTEP + explicit drawing notes control fabrication; STL is a viewing aid.\nverification/ records geometry, source and reduced thermal calculations.\nThermal performance, electrical protection and cable behavior require the specified physical commissioning.\nThis is not a released biological culture system.\n")?;
    fs::copy(
        a.source_dir.join("docs/heated-cassette-v0/RFQ.md"),
        a.output_dir.join("RFQ.md"),
    )?;
    let zip = Command::new("zip")
        .current_dir(&a.output_dir)
        .args([
            "-qr",
            "supplier-RFQ.zip",
            "manufacturing",
            "optional-materials",
            "shop-drawings.pdf",
            "assembly-handbook.pdf",
            "assembly-drawings.pdf",
            "assembly/assembly-bom.csv",
            "assembly/assembly-overview.png",
            "assembly/procurement-bom.csv",
            "verification/independent-import",
            "RFQ.md",
        ])
        .output()?;
    if !zip.status.success() {
        return Err("supplier ZIP creation failed".into());
    }
    let mut hashes = std::collections::BTreeMap::new();
    walk(&a.output_dir, &a.output_dir, &mut hashes)?;
    fs::write(
        a.output_dir.join("manifest-sha256.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"source_git_head":report["git_head"],"source_git_status":report["git_status"],"files":hashes}),
        )?,
    )?;
    println!(
        "{}",
        serde_json::json!({"output_dir":a.output_dir,"qa_dir":qa_dir,"files":hashes.len(),"status":"packaged_requires_visual_review"})
    );
    Ok(())
}
