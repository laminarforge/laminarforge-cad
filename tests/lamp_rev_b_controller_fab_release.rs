use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const ROOT: &str = "pcb/lamp_rev_b_controller";

fn read_toml(name: &str) -> toml::Value {
    toml::from_str(
        &fs::read_to_string(Path::new(ROOT).join(name))
            .unwrap_or_else(|error| panic!("read {name}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {name}: {error}"))
}

fn strings(value: &toml::Value, path: &[&str]) -> Vec<String> {
    let mut current = value;
    for key in path {
        current = current
            .get(*key)
            .unwrap_or_else(|| panic!("missing {}", path.join(".")));
    }
    current
        .as_array()
        .unwrap_or_else(|| panic!("{} is not an array", path.join(".")))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("{} contains a non-string", path.join(".")))
                .to_string()
        })
        .collect()
}

#[test]
fn first_article_safety_population_is_locked() {
    let release = read_toml("fab_release.toml");
    assert_eq!(release["package"]["ticket"].as_str(), Some("T-E36FA2C2"));
    assert_eq!(
        release["variant"]["name"].as_str(),
        Some("rev_b_12v_first_article")
    );
    assert_eq!(release["variant"]["input_voltage_v"].as_float(), Some(12.0));
    assert!(release["variant"]["r55_population"]
        .as_str()
        .unwrap()
        .contains("POPULATE"));
    assert!(release["variant"]["r25_r26_population"]
        .as_str()
        .unwrap()
        .contains("MANDATORY DNP"));
    assert!(release["variant"]["j24_requirement"]
        .as_str()
        .unwrap()
        .contains("normally-closed thermal cutoff"));
    assert_eq!(
        release["variant"]["panelization_owner"].as_str(),
        Some("fabricator")
    );
}

#[test]
fn no_substitution_policy_covers_critical_paths() {
    let release = read_toml("fab_release.toml");
    let protected = strings(&release, &["assembly", "no_substitution_part_ids"])
        .into_iter()
        .collect::<BTreeSet<_>>();
    for required in [
        "mcu_module",
        "usb_c_connector",
        "five_v_buck_module",
        "three_v_three_regulator",
        "adc",
        "thermistor_mux",
        "heater_gate_driver",
        "heater_mosfets",
        "heater_output_terminals",
        "heater_cutoff_loop_terminal",
        "led_driver_module",
        "led_current_shunt",
        "led_current_sense_amp",
        "camera_spi_header",
    ] {
        assert!(protected.contains(required), "missing {required}");
    }
}

#[test]
fn archive_allowlists_are_unique_and_path_safe() {
    let manifest = read_toml("release_manifest.toml");
    let groups = [
        strings(&manifest, &["gerbers", "files"]),
        strings(&manifest, &["paste", "files"]),
        strings(&manifest, &["review_package", "files"]),
        strings(&manifest, &["release_only", "files"]),
    ];
    let mut seen = BTreeSet::new();
    for path in groups.into_iter().flatten() {
        assert!(!path.starts_with('/'));
        assert!(!path.contains(".."));
        assert!(!path.contains('\\'));
        assert!(seen.insert(path.clone()), "duplicate allowlist path {path}");
    }
    for required in [
        "MANIFEST.json",
        "SHA256SUMS",
        "assembly/lamp_rev_b_controller-dnp.csv",
        "assembly/lamp_rev_b_controller-no-substitution.csv",
        "evidence/drc.json",
        "evidence/erc.json",
    ] {
        assert!(seen.contains(required), "missing {required}");
    }
}

#[test]
fn unresolved_external_choices_are_explicit_not_invented() {
    let release = read_toml("fab_release.toml");
    for field in [
        "manufacturer",
        "physical_stackup",
        "finished_thickness_tolerance",
        "surface_finish",
        "surface_finish_thickness",
        "soldermask_color",
        "silkscreen_color",
        "minimum_annular_ring",
        "finished_hole_tolerance",
        "acceptance_class",
        "impedance_policy",
    ] {
        assert_eq!(
            release["fabrication_order_profile"][field].as_str(),
            Some(""),
            "{field} must remain an explicit release blocker until approved"
        );
    }
    assert_eq!(
        release["portal_gate"]["preview_approved"].as_bool(),
        Some(false)
    );
    assert_eq!(release["portal_gate"]["approval_record"].as_str(), Some(""));
}

#[test]
fn obsolete_parallel_packagers_are_removed() {
    assert!(!Path::new(ROOT).join("kibot.yaml").exists());
    assert!(!Path::new("src/bin/lamp_rev_b_controller_fab_preview.rs").exists());
    let cargo = fs::read_to_string("Cargo.toml").unwrap();
    assert!(cargo.contains("name = \"lamp_rev_b_controller_fab_release\""));
    assert!(!cargo.contains("name = \"lamp_rev_b_controller_fab_preview\""));
}
