//! Local macOS packaging only. Never builds or restarts services.
use std::{
    path::{Path, PathBuf},
    process::Command,
};
fn run(command: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    let result = command.output()?;
    if !result.status.success() {
        return Err(String::from_utf8_lossy(&result.stderr).into_owned().into());
    }
    Ok(())
}
fn bundle(binary: &Path, app: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if app.extension().is_none_or(|e| e != "app") {
        return Err("Output must end in .app".into());
    }
    if app.exists() {
        return Err("Output app already exists; choose a new build output directory.".into());
    }
    if !binary.is_file() {
        return Err("The built studio executable is missing.".into());
    }
    let macos = app.join("Contents/MacOS");
    let resources = app.join("Contents/Resources");
    std::fs::create_dir_all(&macos)?;
    std::fs::create_dir_all(&resources)?;
    std::fs::copy(binary, macos.join("laminarforge_studio"))?;
    std::fs::write(
        app.join("Contents/Info.plist"),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>laminarforge_studio</string>
<key>CFBundleIdentifier</key><string>com.laminarforge.studio</string>
<key>CFBundleName</key><string>LaminarForge Studio</string>
<key>CFBundleDisplayName</key><string>LaminarForge Studio</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundleVersion</key><string>1</string>
<key>CFBundleIconFile</key><string>Studio.icns</string>
<key>LSMinimumSystemVersion</key><string>12.0</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>"#,
    )?;
    let iconset = resources.join("Studio.iconset");
    std::fs::create_dir(&iconset)?;
    for size in [16u32, 32, 128, 256, 512] {
        for factor in [1u32, 2] {
            let n = size * factor;
            let icon = image::RgbaImage::from_fn(n, n, |x, y| {
                let x = x as f32 / n as f32;
                let y = y as f32 / n as f32;
                let round = ((x - 0.5).abs() - 0.30)
                    .max(0.0)
                    .hypot(((y - 0.5).abs() - 0.30).max(0.0));
                if round > 0.18 {
                    return image::Rgba([0, 0, 0, 0]);
                }
                let dx = x - 0.5;
                let top = (y - 0.34).abs() / 0.17 + dx.abs() / 0.29 <= 1.0;
                let left =
                    (-0.29..=0.0).contains(&dx) && y >= 0.34 - dx * 0.586 && y <= 0.68 - dx * 0.586;
                let right =
                    (0.0..=0.29).contains(&dx) && y >= 0.34 + dx * 0.586 && y <= 0.68 + dx * 0.586;
                image::Rgba(if top {
                    [116, 233, 214, 255]
                } else if left {
                    [30, 144, 144, 255]
                } else if right {
                    [48, 188, 177, 255]
                } else {
                    [18, 29, 38, 255]
                })
            });
            let suffix = if factor == 2 { "@2x" } else { "" };
            icon.save(iconset.join(format!("icon_{size}x{size}{suffix}.png")))?;
        }
    }
    run(Command::new("/usr/bin/iconutil")
        .arg("-c")
        .arg("icns")
        .arg(&iconset)
        .arg("-o")
        .arg(resources.join("Studio.icns")))?;
    std::fs::remove_dir_all(iconset)?;
    run(Command::new("/usr/bin/codesign")
        .args(["--force", "--sign", "-"])
        .arg(app))?;
    run(Command::new("/usr/bin/codesign")
        .args(["--verify", "--strict"])
        .arg(app))?;
    tracing::info!(component="studio_bundle", app=%app.display(), "Signed local macOS bundle ready");
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if args.len() != 2 {
        return Err("Usage: studio_bundle BUILT_EXECUTABLE OUTPUT.app".into());
    }
    bundle(&args[0], &args[1])
}
