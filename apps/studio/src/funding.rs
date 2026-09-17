use eframe::egui::{self, RichText};
use serde::{Deserialize, Serialize};

#[path = "sourcing.rs"]
mod sourcing;

const KEY: &str = "funding_workspace_v1";

#[derive(Serialize, Deserialize)]
pub struct BudgetItem {
    label: String,
    amount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Funding {
    #[serde(skip)]
    sourcing: Option<sourcing::Sourcing>,
    title: String,
    application_url: String,
    summary: String,
    proposal: String,
    budget: Vec<BudgetItem>,
    budget_note: String,
    availability: String,
    #[serde(default)]
    scope_revision: u32,
    notes: String,
    milestones: String,
    reviewed: bool,
    submitted: bool,
    submitted_date: String,
}

impl Funding {
    pub fn load(storage: Option<&dyn eframe::Storage>) -> Result<Self, String> {
        let saved = storage.and_then(|s| s.get_string(KEY));
        let mut funding: Self = serde_json::from_str(
            saved
                .as_deref()
                .unwrap_or(include_str!("../assets/funding.json")),
        )
        .map_err(|e| format!("Could not load funding workspace: {e}"))?;
        if funding.scope_revision == 0 {
            funding.proposal = funding.proposal.replace(
                "twelve weeks from funding and access to a suitable workspace, subject to fabrication and instrument lead times",
                "twelve weeks from funding, subject to fabrication and instrument lead times",
            ).replace(
                "I will run the validation internally, starting with water-filled plates.",
                "I will assemble and validate the cassette internally using water-filled plates in an ordinary workspace with suitable electrical safety and measurement equipment. This initial round requires no biological lab, shared-lab membership or cell culture work.",
            );
            funding.budget_note = funding.budget_note.replace(
                "Facility costs must be established before this is a complete project budget.",
                "No laboratory access or facility rental is required for this round. The deliverable is an assembled cassette validated with water-filled plates.",
            );
            funding.scope_revision = 1;
            funding.reviewed = false;
        }
        funding.sourcing = Some(sourcing::Sourcing::load()?);
        Ok(funding)
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        match serde_json::to_string(self) {
            Ok(value) => storage.set_string(KEY, value),
            Err(e) => tracing::error!(component="studio_funding", error=%e, "Funding save failed"),
        }
    }

    fn total(&self) -> u64 {
        self.budget.iter().map(|row| u64::from(row.amount)).sum()
    }

    fn budget_text(&self) -> String {
        let rows = self
            .budget
            .iter()
            .map(|row| format!("{}: ${}", row.label, row.amount))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{rows}\nTotal requested: ${}\n\n{}",
            self.total(),
            self.budget_note
        )
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut save = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().id_salt("funding_scroll").show(ui, |ui| {
                ui.set_max_width(1060.0);
                ui.add_space(12.0);
                ui.heading("Funding");
                ui.label("Application, budget and next steps for the first cassette prototype.");
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("${} proposed request", self.total())).strong().size(20.0));
                    ui.separator();
                    ui.label(if self.submitted { "Marked submitted by you" } else { "Draft · not submitted" });
                    save = ui.button("Save changes").clicked();
                    ui.hyperlink_to("Open application ↗", &self.application_url);
                });
                ui.label(RichText::new("Edits are saved locally on this Mac. Opening the application does not submit it.").small().weak());
                ui.separator();
                ui.label(RichText::new(&self.title).strong());
                ui.label("Organizational structure: undecided. No personal cash match assumed.");
                if let Some(sourcing) = &self.sourcing { sourcing.show(ui); }
                ui.label(RichText::new("Initial round: manufacture → assemble → validate temperatures with water-filled plates.").strong());
                ui.label("An ordinary workspace with suitable electrical safety and measurement equipment is sufficient. Lab access, facility rent and cell experiments are outside this round.");
                edit(ui, "Weekly availability", &mut self.availability, 1);
                egui::CollapsingHeader::new("Application text").default_open(true).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Short description");
                        if ui.button("Copy description").clicked() { ctx.copy_text(self.summary.clone()); }
                    });
                    ui.add(egui::TextEdit::multiline(&mut self.summary).desired_rows(2).desired_width(f32::INFINITY));
                    let words = self.proposal.split_whitespace().count();
                    ui.horizontal(|ui| {
                        ui.label(format!("Proposal · {words} / 1,500 words"));
                        if ui.button("Copy proposal").clicked() { ctx.copy_text(self.proposal.clone()); }
                    });
                    if words > 1500 { ui.colored_label(egui::Color32::LIGHT_RED, "Shorten the proposal before submitting."); }
                    ui.add(egui::TextEdit::multiline(&mut self.proposal).desired_rows(18).desired_width(f32::INFINITY));
                });
                egui::CollapsingHeader::new("Budget · editable planning allowances").default_open(true).show(ui, |ui| {
                    egui::Grid::new("funding_budget").striped(true).show(ui, |ui| {
                        for row in &mut self.budget {
                            ui.text_edit_singleline(&mut row.label);
                            ui.add(egui::DragValue::new(&mut row.amount).range(0..=1_000_000).prefix("$"));
                            ui.end_row();
                        }
                    });
                    if ui.button("Add expense").clicked() { self.budget.push(BudgetItem { label: "New expense".into(), amount: 0 }); }
                    edit(ui, "Budget assumptions", &mut self.budget_note, 3);
                    ui.horizontal(|ui| {
                        if ui.button("Copy requested amount").clicked() { ctx.copy_text(self.total().to_string()); }
                        if ui.button("Copy expense breakdown").clicked() { ctx.copy_text(self.budget_text()); }
                    });
                    ui.label("The requested amount and copied breakdown use the current expense total.");
                });
                egui::CollapsingHeader::new("Milestones and submission checklist").default_open(true).show(ui, |ui| {
                    edit(ui, "Milestones", &mut self.milestones, 5);
                    edit(ui, "Working notes", &mut self.notes, 3);
                    ui.checkbox(&mut self.reviewed, "I reviewed the personal statements, budget and proposed timeline");
                    ui.checkbox(&mut self.submitted, "I submitted the application through the provider's form");
                    edit(ui, "Submission date / confirmation reference", &mut self.submitted_date, 1);
                    ui.label("Complete any applicant certifications and CAPTCHA on the provider's site. This app tracks your work; it does not send the application.");
                });
            });
        });
        save
    }
}

fn edit(ui: &mut egui::Ui, label: &str, value: &mut String, rows: usize) {
    ui.label(label);
    ui.add(
        egui::TextEdit::multiline(value)
            .desired_rows(rows)
            .desired_width(f32::INFINITY),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::Storage;
    #[derive(Default)]
    struct Memory(std::collections::HashMap<String, String>);
    impl eframe::Storage for Memory {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }
        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.into(), value);
        }
        fn flush(&mut self) {}
    }
    #[test]
    fn seed_is_draft_and_budget_is_consistent() {
        let f = Funding::load(None).unwrap();
        assert_eq!(f.total(), 15000);
        assert!(!f.submitted);
        assert!(f.proposal.split_whitespace().count() <= 1500);
        assert_eq!(f.scope_revision, 1);
    }
    #[test]
    fn edited_application_survives_restart() {
        let mut f = Funding::load(None).unwrap();
        f.proposal = "My revised proposal".into();
        f.budget[0].amount = 123;
        f.availability = "10 hours/week".into();
        let mut s = Memory::default();
        f.save(&mut s);
        let restored = Funding::load(Some(&s)).unwrap();
        assert_eq!(restored.proposal, f.proposal);
        assert_eq!(restored.total(), f.total());
        assert_eq!(restored.availability, f.availability);
        assert!(restored.sourcing.is_some());
    }
    #[test]
    fn previous_draft_loses_lab_prerequisite_without_losing_edits() {
        let mut value = serde_json::to_value(Funding::load(None).unwrap()).unwrap();
        value.as_object_mut().unwrap().remove("scope_revision");
        value["location"] = "Prospective Fitzsimons".into();
        value["workspace_costs"] = "".into();
        value["proposal"] = "Custom intro. twelve weeks from funding and access to a suitable workspace, subject to fabrication and instrument lead times".into();
        value["budget_note"] = "Custom costs. Facility costs must be established before this is a complete project budget.".into();
        value["availability"] = "8 hours/week".into();
        value["budget"][0]["amount"] = 123.into();
        let mut storage = Memory::default();
        storage.set_string(KEY, value.to_string());
        let migrated = Funding::load(Some(&storage)).unwrap();
        assert!(migrated.proposal.starts_with("Custom intro."));
        assert!(!migrated.proposal.contains("access to a suitable workspace"));
        assert!(migrated.budget_note.starts_with("Custom costs."));
        assert!(!migrated.budget_note.contains("Facility costs must"));
        assert_eq!(migrated.budget[0].amount, 123);
        assert_eq!(migrated.availability, "8 hours/week");
        migrated.save(&mut storage);
        let saved = storage.get_string(KEY).unwrap();
        assert!(!saved.contains("Fitzsimons"));
        assert!(!saved.contains("workspace_costs"));
        assert_eq!(
            Funding::load(Some(&storage)).unwrap().proposal,
            migrated.proposal
        );
    }
    #[test]
    fn corrupt_saved_work_is_not_silently_reset() {
        let mut s = Memory::default();
        s.set_string(KEY, "invalid json".into());
        assert!(Funding::load(Some(&s)).is_err());
    }
}
