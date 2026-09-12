//! Small, dependency-free CI measurement driver. Compile once with rustc.
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn required(name: &str) -> Result<String> {
    Ok(env::var(name).map_err(|_| format!("{name} is required"))?)
}
fn token(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn phase(name: &str, executable: &str, args: &[String]) -> Result<()> {
    let start = Instant::now();
    let stdout = fs::File::create(format!("artifacts/{name}.stdout.log"))?;
    let stderr = fs::File::create(format!("artifacts/{name}.stderr.log"))?;
    let status = Command::new("/usr/bin/time")
        .args([
            "-v",
            "-o",
            &format!("artifacts/{name}.resources.txt"),
            "--",
            executable,
        ])
        .args(args)
        .env("CARGO_BUILD_JOBS", "2")
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .status()?;
    let elapsed = start.elapsed().as_secs_f64();
    use std::io::Write;
    writeln!(
        fs::OpenOptions::new()
            .append(true)
            .open("artifacts/phases.csv")?,
        "{name},{elapsed:.6},{}",
        status.success()
    )?;
    if !status.success() {
        return Err(
            format!("phase {name} failed: {status}; inspect artifacts/{name}.stderr.log").into(),
        );
    }
    Ok(())
}

fn cargo_args(action: &str, bin: &str, profile: &str, features: &str) -> Vec<String> {
    let mut args = vec![
        action.into(),
        "--locked".into(),
        "--bin".into(),
        bin.into(),
        "--profile".into(),
        profile.into(),
        "--jobs=2".into(),
        "--timings".into(),
    ];
    if !features.is_empty() {
        args.extend(["--features".into(), features.into()]);
    }
    args
}

fn validate_stl(path: &Path) -> Result<[f32; 6]> {
    let data = fs::read(path)?;
    if data.len() < 84 {
        return Err("truncated binary STL".into());
    }
    let triangles = u32::from_le_bytes(data[80..84].try_into()?) as usize;
    if triangles == 0 || data.len() != 84 + triangles * 50 {
        return Err("invalid STL triangle count/length".into());
    }
    let mut bounds = [f32::INFINITY; 6];
    bounds[3..].fill(f32::NEG_INFINITY);
    for triangle in data[84..].chunks_exact(50) {
        for vertex in triangle[12..48].chunks_exact(12) {
            for axis in 0..3 {
                let v = f32::from_le_bytes(vertex[axis * 4..axis * 4 + 4].try_into()?);
                if !v.is_finite() {
                    return Err("nonfinite mesh coordinate".into());
                }
                bounds[axis] = bounds[axis].min(v);
                bounds[axis + 3] = bounds[axis + 3].max(v);
            }
        }
    }
    if (0..3).any(|a| bounds[a] >= bounds[a + 3]) {
        return Err("degenerate mesh bounds".into());
    }
    Ok(bounds)
}

fn fixture_run(name: &str, binary: &str, config: &str) -> Result<[f32; 6]> {
    let out = format!("artifacts/{name}");
    phase(
        name,
        binary,
        &[
            "--config".into(),
            config.into(),
            "--output-dir".into(),
            out.clone(),
        ],
    )?;
    let mut base = [0.0; 6];
    for part in ["base", "tubing_comb", "luer_clip", "assembly"] {
        let bounds =
            validate_stl(&PathBuf::from(&out).join(format!("chip_priming_fixture_{part}.stl")))?;
        if part == "base" {
            base = bounds;
        }
    }
    Ok(base)
}

fn main() -> Result<()> {
    let action = required("CAD_ACTION")?;
    let bin = required("CAD_BIN")?;
    let profile = required("CAD_PROFILE")?;
    let features = required("CAD_FEATURES")?;
    if !matches!(action.as_str(), "check" | "build" | "run" | "benchmark")
        || !token(&bin)
        || !matches!(profile.as_str(), "dev" | "release")
        || (!features.is_empty() && features.split(',').any(|f| !token(f)))
    {
        return Err("invalid CAD action, binary, profile, or feature".into());
    }
    if !PathBuf::from(format!("src/bin/{bin}.rs")).is_file() {
        return Err("binary source missing".into());
    }
    fs::create_dir_all("artifacts")?;
    fs::write("artifacts/phases.csv", "phase,wall_seconds,success\n")?;
    let source = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !source.status.success() {
        return Err("source identity unavailable".into());
    }
    let sha = String::from_utf8(source.stdout)?;
    if sha.trim() != required("CAD_SOURCE_SHA")? {
        return Err("source SHA does not match dispatched identity".into());
    }
    fs::write(
        "artifacts/source.txt",
        format!(
            "sha={}\nbin={bin}\nprofile={profile}\nfeatures={features}\njobs=2\n",
            sha.trim()
        ),
    )?;
    phase(
        "toolchain",
        "rustc",
        &["--version".into(), "--verbose".into()],
    )?;
    if action == "check" {
        return phase(
            "check",
            "cargo",
            &cargo_args("check", &bin, &profile, &features),
        );
    }
    if action == "benchmark" {
        phase(
            "check",
            "cargo",
            &cargo_args("check", &bin, &profile, &features),
        )?;
    }
    phase(
        "build",
        "cargo",
        &cargo_args("build", &bin, &profile, &features),
    )?;
    if action == "build" {
        return Ok(());
    }
    let directory = if profile == "dev" { "debug" } else { "release" };
    let binary = format!("target/{directory}/{bin}");
    if action == "benchmark" {
        phase(
            "warm_build",
            "cargo",
            &cargo_args("build", &bin, &profile, &features),
        )?;
    }
    if bin == "chip_priming_tubing_fixture" {
        let original = fs::read_to_string("models/chip_priming_tubing_fixture.toml")?;
        let supplied = required("CAD_MODEL_CONFIG")?;
        if !supplied.is_empty() && action != "run" {
            return Err("custom config requires run action".into());
        }
        fs::write(
            "artifacts/model.toml",
            if supplied.is_empty() {
                &original
            } else {
                &supplied
            },
        )?;
        let before_binary = fs::read(&binary)?;
        let before = fixture_run("generate", &binary, "artifacts/model.toml")?;
        if action == "benchmark" {
            // Increase only the X margin. This must widen the base by exactly 2 mm.
            let mut found = false;
            let modified: Vec<String> = original
                .lines()
                .map(|line| {
                    if let Some((key, value)) = line.split_once('=') {
                        if key.trim() == "margin_x_mm" {
                            found = true;
                            let old: f64 = value
                                .trim()
                                .parse()
                                .expect("committed margin must be numeric");
                            return format!("margin_x_mm = {}", old + 1.0);
                        }
                    }
                    line.to_owned()
                })
                .collect();
            if !found {
                return Err("margin_x_mm missing".into());
            }
            fs::write("artifacts/variant.toml", modified.join("\n"))?;
            let after = fixture_run("data_only_generate", &binary, "artifacts/variant.toml")?;
            if fs::read(&binary)? != before_binary {
                return Err("binary changed during data-only iteration".into());
            }
            if ((after[3] - after[0]) - (before[3] - before[0]) - 2.0).abs() > 0.01 {
                return Err("parameter change did not widen mesh by 2 mm".into());
            }
            fs::write("artifacts/data-only-verification.txt", "PASS: executable bytes unchanged; no Cargo invocation between runs; valid STL outputs; base width increased 2 mm\n")?;
        }
    } else {
        if !required("CAD_MODEL_CONFIG")?.is_empty() {
            return Err("custom config unsupported for this model".into());
        }
        fs::create_dir_all("output")?;
        phase("generate", &binary, &[])?;
    }
    Ok(())
}
