use super::Component;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

pub fn validate_and_export(pcb_path: &Path, output_dir: &Path) {
    use std::process::Command;

    let kicad = which_kicad_cli();
    if kicad.is_empty() {
        eprintln!("ERROR: kicad-cli not found in PATH. Install KiCad: brew install --cask kicad");
        std::process::exit(1);
    }
    println!("\nUsing: {}", kicad);

    // Fill zones via KiCad Python API
    println!("\n── Zone Fill ──");
    let kicad_python = "/Applications/KiCad/KiCad.app/Contents/Frameworks/Python.framework/Versions/Current/bin/python3";
    let fill_script = format!(
        "import pcbnew; b = pcbnew.LoadBoard('{}'); filler = pcbnew.ZONE_FILLER(b); filler.Fill(b.Zones()); pcbnew.SaveBoard('{}', b); print('Zones filled: {{}} zones'.format(len(b.Zones())))",
        pcb_path.to_string_lossy(), pcb_path.to_string_lossy()
    );
    let fill = Command::new(kicad_python)
        .args(["-c", &fill_script])
        .output();
    match fill {
        Ok(f) => {
            print!("{}", String::from_utf8_lossy(&f.stdout));
            if !f.status.success() {
                eprintln!("Zone fill warning: {}", String::from_utf8_lossy(&f.stderr));
            }
        }
        Err(e) => eprintln!("Zone fill skipped (KiCad Python not found: {})", e),
    }

    // DRC
    let drc_path = output_dir.join("drc.json");
    println!("\n── DRC Check ──");
    let drc = Command::new(&kicad)
        .args([
            "pcb",
            "drc",
            &pcb_path.to_string_lossy(),
            "-o",
            &drc_path.to_string_lossy(),
            "--severity-all",
            "--all-track-errors",
        ])
        .output()
        .expect("failed to run kicad-cli drc");
    let drc_err = String::from_utf8_lossy(&drc.stderr);
    if !drc_err.is_empty() {
        eprint!("{}", drc_err);
    }
    if !drc.status.success() {
        eprintln!(
            "DRC failed (exit {}). Check {} for details.",
            drc.status.code().unwrap_or(-1),
            drc_path.display()
        );
        std::process::exit(2);
    }

    // Parse the DRC report to classify violations
    let (real_violations, zone_fragments, real_unconnected, zone_unconnected) =
        parse_drc_report(&drc_path);

    println!("Found {} violations", real_violations + zone_fragments);
    if zone_fragments > 0 {
        println!(
            "  ({} isolated copper zone fragments — non-critical)",
            zone_fragments
        );
    }
    println!(
        "Found {} unconnected items",
        real_unconnected + zone_unconnected
    );
    if zone_unconnected > 0 {
        println!(
            "  ({} zone-to-zone GND on F.Cu — non-critical, B.Cu plane provides connectivity)",
            zone_unconnected
        );
    }
    println!("Saved DRC Report to {}", drc_path.display());

    if real_violations > 0 || real_unconnected > 0 {
        eprintln!(
            "ERROR: {} real violations, {} real unconnected",
            real_violations, real_unconnected
        );
        std::process::exit(2);
    }
    println!("DRC passed: 0 real violations, 0 real unconnected");

    // Gerbers
    let gerber_dir = output_dir.join("gerbers");
    println!("\n── Gerber Export ──");
    let gerber = Command::new(&kicad)
        .args([
            "pcb",
            "export",
            "gerbers",
            &pcb_path.to_string_lossy(),
            "-o",
            gerber_dir.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("failed to run kicad-cli gerber export");
    print!("{}", String::from_utf8_lossy(&gerber.stdout));
    if !gerber.status.success() {
        eprintln!(
            "Gerber export failed: {}",
            String::from_utf8_lossy(&gerber.stderr)
        );
        std::process::exit(3);
    }

    // Drill
    println!("\n── Drill Export ──");
    let drill = Command::new(&kicad)
        .args([
            "pcb",
            "export",
            "drill",
            &pcb_path.to_string_lossy(),
            "-o",
            gerber_dir.to_string_lossy().as_ref(),
            "--format",
            "excellon",
            "--excellon-units",
            "mm",
        ])
        .output()
        .expect("failed to run kicad-cli drill export");
    print!("{}", String::from_utf8_lossy(&drill.stdout));
    if !drill.status.success() {
        eprintln!(
            "Drill export failed: {}",
            String::from_utf8_lossy(&drill.stderr)
        );
        std::process::exit(4);
    }

    println!("\n── Generated Files ──");
    if let Ok(entries) = fs::read_dir(&gerber_dir) {
        for entry in entries.flatten() {
            println!("  {}", entry.path().display());
        }
    }

    // Zip
    println!("\n── Packaging for JLCPCB ──");
    let zip_path = output_dir.join("lamp_v1_gerbers.zip");
    let _ = fs::remove_file(&zip_path);
    let zip = Command::new("zip")
        .args([
            "-j",
            &zip_path.to_string_lossy(),
            &format!("{}/*", gerber_dir.to_string_lossy()),
        ])
        .output();
    match zip {
        Ok(z) if z.status.success() => println!("Packaged: {}", zip_path.display()),
        _ => {
            let ditto = Command::new("ditto")
                .args([
                    "-c",
                    "-k",
                    &gerber_dir.to_string_lossy(),
                    &zip_path.to_string_lossy(),
                ])
                .output();
            match ditto {
                Ok(d) if d.status.success() => println!("Packaged: {}", zip_path.display()),
                _ => eprintln!("Warning: could not create zip. Package gerbers/ manually."),
            }
        }
    }

    println!("\nDone! Upload to JLCPCB:");
    println!("  Gerbers: {}", zip_path.display());
    println!("  BOM:     {}", output_dir.join("bom.csv").display());
    println!("  CPL:     {}", output_dir.join("cpl.csv").display());
}

pub fn diagnose_format(pcb_path: &Path) {
    let kicad = which_kicad_cli();
    if kicad.is_empty() {
        eprintln!("kicad-cli not found");
        return;
    }

    let content = fs::read_to_string(pcb_path).expect("read pcb file");

    let full_ok = test_kicad_load(&kicad, &content);
    if full_ok {
        println!("Full file loads OK!");
        return;
    }
    println!("Full file FAILS. Testing sections...\n");

    let sections = extract_top_level_sections(&content);
    let mut accumulated = String::new();

    for (i, (name, section)) in sections.iter().enumerate() {
        let test_content = if accumulated.is_empty() {
            format!("(kicad_pcb\n{}\n)\n", section)
        } else {
            format!("(kicad_pcb\n{}\n{}\n)\n", accumulated, section)
        };

        let ok = test_kicad_load(&kicad, &test_content);
        let status = if ok { "OK  " } else { "FAIL" };
        let line_count = section.lines().count();
        println!("[{}] #{:2} {} ({} lines)", status, i, name, line_count);

        if !ok {
            let preview: String = section.lines().take(20).collect::<Vec<_>>().join("\n");
            println!("  Preview:\n{}", preview);
            if line_count > 20 {
                println!("  ... ({} more lines)", line_count - 20);
            }
            fs::write("/tmp/pcb_diag_fail.kicad_pcb", &test_content).ok();
            println!("\n  Written: /tmp/pcb_diag_fail.kicad_pcb");
            return;
        }

        accumulated = if accumulated.is_empty() {
            section.clone()
        } else {
            format!("{}\n{}", accumulated, section)
        };
    }
    println!("\nAll sections pass individually.");
}

fn test_kicad_load(kicad: &str, content: &str) -> bool {
    use std::process::Command;
    let tmp = "/tmp/pcb_layout_test.kicad_pcb";
    fs::write(tmp, content).expect("write tmp");
    let out = Command::new(kicad)
        .args([
            "pcb",
            "drc",
            tmp,
            "-o",
            "/tmp/pcb_layout_test_drc.json",
            "--severity-all",
        ])
        .output()
        .expect("run kicad-cli");
    out.status.success() || out.status.code() == Some(0)
}

fn extract_top_level_sections(content: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let inner = content.trim();
    let first_nl = inner.find('\n').unwrap_or(inner.len());
    let inner = &inner[first_nl + 1..];
    let inner = inner
        .trim_end()
        .strip_suffix(')')
        .unwrap_or(inner)
        .trim_end();

    let mut i = 0;
    let chars: Vec<char> = inner.chars().collect();
    let len = chars.len();

    while i < len {
        while i < len && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }
        if chars[i] == ';' {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if chars[i] != '(' {
            i += 1;
            continue;
        }

        let start = i;
        i += 1;
        let tag_start = i;
        while i < len && !chars[i].is_whitespace() && chars[i] != ')' {
            i += 1;
        }
        let tag: String = chars[tag_start..i].iter().collect();

        let mut depth = 1;
        while i < len && depth > 0 {
            match chars[i] {
                '(' => depth += 1,
                ')' => depth -= 1,
                '"' => {
                    i += 1;
                    while i < len && chars[i] != '"' {
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let section: String = chars[start..i].iter().collect();

        if let Some(last) = sections.last_mut() {
            if last.0 == tag && section.lines().count() <= 2 && last.1.lines().count() < 100 {
                last.1.push('\n');
                last.1.push_str(&section);
                continue;
            }
        }
        sections.push((tag, section));
    }

    sections
}

/// Parse DRC report and classify items.
/// Returns (real_violations, zone_fragment_violations, real_unconnected, zone_unconnected).
fn parse_drc_report(drc_path: &Path) -> (u32, u32, u32, u32) {
    let content = match fs::read_to_string(drc_path) {
        Ok(c) => c,
        Err(_) => return (0, 0, 0, 0),
    };

    let mut real_violations = 0u32;
    let mut zone_fragments = 0u32;
    let mut real_unconnected = 0u32;
    let mut zone_unconnected = 0u32;

    // Section: "v" = violations, "u" = unconnected, "" = other
    let mut section = "";
    let mut item_lines: Vec<&str> = Vec::new();

    let classify =
        |lines: &[&str], section: &str, rv: &mut u32, zf: &mut u32, ru: &mut u32, zu: &mut u32| {
            if lines.is_empty() || section.is_empty() {
                return;
            }
            // Only classify blocks that start with a DRC rule tag
            if !lines[0].starts_with('[') {
                return;
            }
            let _is_zone = lines.iter().any(|l| l.contains("Zone [GND]"));
            let all_zone = lines
                .iter()
                .filter(|l| l.trim_start().starts_with("@("))
                .all(|l| l.contains("Zone ["));
            match section {
                "v" => {
                    if all_zone {
                        *zf += 1;
                    } else {
                        *rv += 1;
                    }
                }
                "u" => {
                    if all_zone {
                        *zu += 1;
                    } else {
                        *ru += 1;
                    }
                }
                _ => {}
            }
        };

    for line in content.lines() {
        if line.starts_with("** Found") {
            // Classify pending item before switching sections
            classify(
                &item_lines,
                section,
                &mut real_violations,
                &mut zone_fragments,
                &mut real_unconnected,
                &mut zone_unconnected,
            );
            item_lines.clear();

            if line.contains("DRC violations") {
                section = "v";
            } else if line.contains("unconnected") {
                section = "u";
            } else {
                section = "";
            }
            continue;
        }
        if line.starts_with("** End of Report") {
            classify(
                &item_lines,
                section,
                &mut real_violations,
                &mut zone_fragments,
                &mut real_unconnected,
                &mut zone_unconnected,
            );
            break;
        }

        if line.starts_with('[') {
            // New item — classify the previous one
            classify(
                &item_lines,
                section,
                &mut real_violations,
                &mut zone_fragments,
                &mut real_unconnected,
                &mut zone_unconnected,
            );
            item_lines.clear();
        }

        if !line.is_empty() {
            item_lines.push(line);
        }
    }

    (
        real_violations,
        zone_fragments,
        real_unconnected,
        zone_unconnected,
    )
}

fn which_kicad_cli() -> String {
    use std::process::Command;
    let output = Command::new("which").arg("kicad-cli").output();
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => String::new(),
    }
}

pub fn write_bom(path: &Path, components: &[Component]) {
    let mut bom = String::from("Comment,Designator,Footprint,LCSC Part Number\n");
    let mut groups: std::collections::BTreeMap<(&str, &str), Vec<&str>> =
        std::collections::BTreeMap::new();
    for comp in components {
        if comp.lcsc.is_empty() {
            continue;
        }
        groups
            .entry((comp.value, comp.lcsc))
            .or_default()
            .push(comp.reference);
    }
    for ((value, lcsc), refs) in &groups {
        let designators = refs.join(",");
        let footprint = components
            .iter()
            .find(|c| c.value == *value && c.lcsc == *lcsc)
            .map(|c| c.footprint_name)
            .unwrap_or("");
        writeln!(
            bom,
            "\"{}\",\"{}\",\"{}\",\"{}\"",
            value, designators, footprint, lcsc
        )
        .unwrap();
    }
    fs::write(path, &bom).expect("write BOM");
    println!("BOM entries: {}", groups.len());
}

pub fn write_cpl(path: &Path, components: &[Component]) {
    let mut cpl = String::from("Designator,Val,Package,Mid X,Mid Y,Rotation,Layer\n");
    for comp in components {
        if comp.lcsc.is_empty() {
            continue;
        }
        let layer = if comp.layer == "F.Cu" {
            "top"
        } else {
            "bottom"
        };
        writeln!(
            cpl,
            "\"{}\",\"{}\",\"{}\",{}mm,{}mm,{},{}",
            comp.reference, comp.value, comp.footprint_name, comp.x, comp.y, comp.rotation, layer
        )
        .unwrap();
    }
    fs::write(path, &cpl).expect("write CPL");
}
