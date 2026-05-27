use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Disposition {
    Buy,
    Build,
    Access,
    BuyOrAccess,
    PrototypeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Readiness {
    RequiredBeforeCulture,
    EarlyBuild,
    PrototypeGate,
    Defer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CultureUse {
    RealCulture,
    PracticeOnly,
    HardwareValidation,
    Support,
}

#[derive(Debug, Serialize)]
struct Equipment {
    category: &'static str,
    item: &'static str,
    disposition: Disposition,
    readiness: Readiness,
    culture_use: CultureUse,
    cad_bins: &'static [&'static str],
    validation_gate: &'static str,
    notes: &'static str,
}

fn manifest() -> Vec<Equipment> {
    vec![
        Equipment {
            category: "Sterile handling",
            item: "Certified Class II biosafety cabinet or lab-provided sterile hood",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[],
            validation_gate: "Certification sticker/current service record, airflow smoke test per facility procedure, and approved decontamination SOP.",
            notes: "Do not use DIY airflow hardware for real mammalian culture, primary human material, viral work, or iPSC work.",
        },
        Equipment {
            category: "Sterile handling",
            item: "Practice still-air enclosure",
            disposition: Disposition::PrototypeOnly,
            readiness: Readiness::EarlyBuild,
            culture_use: CultureUse::PracticeOnly,
            cad_bins: &[
                "still_air_box_corner",
                "still_air_box_rail",
                "still_air_box_accessories",
                "arm_hole_grommet",
                "workstation_enclosure",
            ],
            validation_gate: "Fit check, cleanability review, surface wipe compatibility, and nonhazardous aseptic-transfer practice only.",
            notes: "Useful for training movements and dry runs; not a substitute for a certified BSC.",
        },
        Equipment {
            category: "Incubation",
            item: "Validated CO2 incubator access",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[],
            validation_gate: "Independent 37 C, 5% CO2, humidity, contamination, and door-recovery logs before culture use.",
            notes: "Use a certified or used commercial incubator for serious mammalian culture until DIY cabinet passes validation.",
        },
        Equipment {
            category: "Incubation",
            item: "Starter DIY CO2 incubator cabinet",
            disposition: Disposition::PrototypeOnly,
            readiness: Readiness::PrototypeGate,
            culture_use: CultureUse::HardwareValidation,
            cad_bins: &["co2_incubator", "co2_incubator_verify", "co2_incubator_sim"],
            validation_gate: "CAD verifier pass, lumped thermal/CO2 simulation pass, empty-chamber commissioning, then physical logger agreement.",
            notes: "Internal engineering prototype; not website-ready or culture-ready yet.",
        },
        Equipment {
            category: "Temperature support",
            item: "37 C media water bath or dry warming block",
            disposition: Disposition::Build,
            readiness: Readiness::EarlyBuild,
            culture_use: CultureUse::Support,
            cad_bins: &["water_bath", "water_bath_safety_kit", "heating_block"],
            validation_gate: "Independent probe shows 37 C setpoint stability and no leak/electrical hazard during a multi-hour run.",
            notes: "Useful for warming media and reagents; keep bottles sealed and disinfected.",
        },
        Equipment {
            category: "Observation",
            item: "Inverted phase-contrast microscope",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &["optical_mount"],
            validation_gate: "Can inspect culture vessels/chips for confluence, morphology, and visible contamination before and after handling.",
            notes: "A normal upright scope is not enough for routine flask and plate culture.",
        },
        Equipment {
            category: "Liquid handling",
            item: "Automation-first closed fluid handling path",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[
                "syringe_pump_standalone",
                "media_reservoir",
                "chip_priming_tubing_fixture",
                "automated_media_exchange_cassette",
            ],
            validation_gate: "Disposable sterile fluid path, calibrated flow rates, no open manual transfers in the intended process, and leak/bubble/dead-volume checks.",
            notes: "The intended LaminarForge workflow should use pumps, reservoirs, tubing, manifolds, and fixtures rather than routine manual pipetting.",
        },
        Equipment {
            category: "Liquid handling",
            item: "Bench pipettes for calibration and debug only",
            disposition: Disposition::Buy,
            readiness: Readiness::Defer,
            culture_use: CultureUse::HardwareValidation,
            cad_bins: &["tube_holder", "column_rack", "pipette_tip_organizer"],
            validation_gate: "If used, calibrated pipettes and sterile filtered tips are limited to bench metrology, setup, rescue, or non-production validation steps.",
            notes: "Not part of the intended cell-culture process. Keep the design pressure on automated closed handling.",
        },
        Equipment {
            category: "Liquid handling",
            item: "Closed aspiration and waste path",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[
                "wash_station",
                "media_reservoir",
                "aspirator_waste_trap_holder",
            ],
            validation_gate: "Vacuum trap, disinfectant contact, splash/aerosol control, overflow protection, and facility-approved biohazard disposal path.",
            notes: "Waste handling can be automated or assisted, but aspirated media is biohazard waste when cultures are live or potentially contaminated.",
        },
        Equipment {
            category: "Centrifugation",
            item: "Benchtop centrifuge with sealed buckets or appropriate rotor",
            disposition: Disposition::Buy,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &["centrifuge_adapter"],
            validation_gate: "Use only manufacturer rotor ratings; printed adapters are for fit/organization, not safety-critical rotor substitution.",
            notes: "Do not 3D print load-bearing high-speed centrifuge rotors.",
        },
        Equipment {
            category: "Cold storage",
            item: "4 C refrigerator and -20 C freezer",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &["sample_cold_block", "peltier_reservoir_block"],
            validation_gate: "Continuous min/max logging, labeled zones, and no food/shared non-lab storage.",
            notes: "Printed cold blocks help bench handling but do not replace validated cold storage.",
        },
        Equipment {
            category: "Cryostorage",
            item: "Liquid nitrogen or controlled cryostorage access",
            disposition: Disposition::Access,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[],
            validation_gate: "Facility storage access, inventory tracking, PPE, and controlled-rate freezing procedure.",
            notes: "Required before banking cells or holding recovery stocks.",
        },
        Equipment {
            category: "Mixing",
            item: "Orbital shaker, rack rocker, and magnetic stirrer",
            disposition: Disposition::Build,
            readiness: Readiness::EarlyBuild,
            culture_use: CultureUse::Support,
            cad_bins: &["orbital_shaker", "rack_rocker", "rack_rocker_2axis", "magnetic_stirrer"],
            validation_gate: "RPM/tilt verification, tip resistance, spill containment, and application-specific sterility plan.",
            notes: "Useful for staining, washes, reagent prep, and chip rocking; not a substitute for closed fluid handling.",
        },
        Equipment {
            category: "Microfluidics",
            item: "Syringe pump, chip fixtures, reservoirs, and tubing management",
            disposition: Disposition::Build,
            readiness: Readiness::EarlyBuild,
            culture_use: CultureUse::HardwareValidation,
            cad_bins: &[
                "syringe_pump_standalone",
                "chip_adapter_plate",
                "chip_stack_rack",
                "chip_priming_tubing_fixture",
                "automated_media_exchange_cassette",
                "media_reservoir",
                "pbmc_flow_cell_mount",
            ],
            validation_gate: "Flow-rate calibration, leak test, bubble management, dead-volume check, and disposable sterile fluid path.",
            notes: "Needed for chip priming/perfusion tests before the larger automated stack.",
        },
        Equipment {
            category: "Automation",
            item: "High-density chip incubator and chip farm assembly",
            disposition: Disposition::Build,
            readiness: Readiness::Defer,
            culture_use: CultureUse::HardwareValidation,
            cad_bins: &["chip_incubator_v3", "chip_farm_assembly", "chip_farm_assembly_v2", "lh_interface"],
            validation_gate: "Only start after starter incubator, sterile handling, and single-chip automated fluid workflow are validated.",
            notes: "Do not scale complexity before the single-cabinet control and contamination questions are resolved.",
        },
        Equipment {
            category: "Safety and waste",
            item: "PPE, disinfectant, biohazard waste, sharps container, spill kit",
            disposition: Disposition::BuyOrAccess,
            readiness: Readiness::RequiredBeforeCulture,
            culture_use: CultureUse::RealCulture,
            cad_bins: &[],
            validation_gate: "Facility-approved SOPs and disposal chain are in place before any live culture work.",
            notes: "This is an operating requirement, not an optional accessory.",
        },
        Equipment {
            category: "Environmental logging",
            item: "Independent temperature, CO2, humidity, and power logging",
            disposition: Disposition::Buy,
            readiness: Readiness::EarlyBuild,
            culture_use: CultureUse::HardwareValidation,
            cad_bins: &[
                "controller_enclosure",
                "cell_culture_logger_enclosure",
                "co2_sensor_service_module",
            ],
            validation_gate: "Independent logger agrees with controller sensors over warmup, steady state, door-open recovery, and overnight hold.",
            notes: "No DIY incubator result is trusted without independent measurement.",
        },
    ]
}

fn main() {
    let equipment = manifest();
    println!("{}", serde_json::to_string_pretty(&equipment).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_covers_required_cell_culture_categories() {
        let items = manifest();
        let required = [
            "Sterile handling",
            "Incubation",
            "Temperature support",
            "Observation",
            "Liquid handling",
            "Centrifugation",
            "Cold storage",
            "Cryostorage",
            "Safety and waste",
            "Environmental logging",
        ];

        for category in required {
            assert!(
                items.iter().any(|item| item.category == category),
                "missing category {category}"
            );
        }
    }

    #[test]
    fn real_culture_items_are_not_assigned_to_build_only_diy_hardware() {
        for item in manifest()
            .into_iter()
            .filter(|item| item.culture_use == CultureUse::RealCulture)
        {
            assert_ne!(
                item.disposition,
                Disposition::Build,
                "{} is real-culture equipment and must not be DIY-only",
                item.item
            );
            assert_eq!(
                item.readiness,
                Readiness::RequiredBeforeCulture,
                "{} must be required before culture",
                item.item
            );
        }
    }

    #[test]
    fn diy_incubator_stays_prototype_only() {
        let incubator = manifest()
            .into_iter()
            .find(|item| item.item == "Starter DIY CO2 incubator cabinet")
            .expect("starter incubator manifest row");

        assert_eq!(incubator.disposition, Disposition::PrototypeOnly);
        assert_eq!(incubator.culture_use, CultureUse::HardwareValidation);
        assert!(incubator.cad_bins.contains(&"co2_incubator_sim"));
    }
}
