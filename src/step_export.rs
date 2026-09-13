use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Convert a mesh to a faceted STEP file. This is not analytic solid modeling.
/// Missing tools, failed conversions and malformed output are errors.
pub fn stl_to_step(stl_path: &str) -> io::Result<PathBuf> {
    let converter = match std::env::var_os("STLTOSTEP_BIN") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(std::env::var_os("HOME").ok_or_else(|| {
            io::Error::other("HOME is missing; configure STLTOSTEP_BIN explicitly")
        })?)
        .join(".local/bin/stltostp"),
    };
    convert(Path::new(stl_path), &converter)
}

fn convert(input: &Path, converter: &Path) -> io::Result<PathBuf> {
    if input.extension().and_then(|v| v.to_str()) != Some("stl") {
        return Err(io::Error::other("STEP conversion requires an .stl input"));
    }
    let output = input.with_extension("stp");
    // A failed conversion must never leave an earlier STEP looking current.
    match std::fs::remove_file(&output) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    if !converter.is_file() {
        return Err(io::Error::other(format!(
            "required STEP converter missing at {}; set STLTOSTEP_BIN to the installed executable",
            converter.display()
        )));
    }
    if std::fs::metadata(input)?.len() <= 84 {
        return Err(io::Error::other("STL input is empty or too small"));
    }
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let temporary = output.with_extension(format!(
        "{}.{}.tmp.stp",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        let status = std::process::Command::new(converter)
            .arg(input)
            .arg(&temporary)
            .output()?;
        if !status.status.success() {
            return Err(io::Error::other(format!(
                "STEP conversion failed for {} ({}): {}",
                input.display(),
                status.status,
                String::from_utf8_lossy(&status.stderr)
                    .chars()
                    .take(2000)
                    .collect::<String>()
            )));
        }
        let text = std::fs::read_to_string(&temporary)?;
        if !text.trim_start().starts_with("ISO-10303-21;")
            || !text.trim_end().ends_with("END-ISO-10303-21;")
            || !text.contains("DATA;")
            || !text.contains("#1")
        {
            return Err(io::Error::other(
                "converter produced incomplete STEP output",
            ));
        }
        std::fs::rename(&temporary, &output)?;
        Ok(output)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(temporary);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn fixture(script: &str) -> (PathBuf, PathBuf, PathBuf) {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "step-test-{}-{}",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let input = dir.join("part.stl");
        std::fs::write(&input, [0u8; 134]).unwrap();
        let tool = dir.join("converter");
        std::fs::write(&tool, format!("#!/bin/sh\n{script}\n")).unwrap();
        std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o700)).unwrap();
        (dir, input, tool)
    }

    #[test]
    fn step_export_rejects_missing_failed_and_incomplete_converters() {
        for script in ["exit 7", "printf 'incomplete' > \"$2\"", "exit 0"] {
            let (dir, input, tool) = fixture(script);
            std::fs::write(input.with_extension("stp"), "stale").unwrap();
            assert!(convert(&input, &tool).is_err());
            assert!(!input.with_extension("stp").exists());
            assert!(convert(&input, &dir.join("missing")).is_err());
            assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
            std::fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn step_export_publishes_complete_output() {
        let (dir, input, tool) = fixture("printf 'ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n#1=EXAMPLE();\nENDSEC;\nEND-ISO-10303-21;\n' > \"$2\"");
        let output = convert(&input, &tool).unwrap();
        assert!(output.is_file());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 3);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
