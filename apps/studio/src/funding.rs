use eframe::egui::{self, RichText};
use serde::{Deserialize, Serialize};

const KEY: &str = "funding_workspace_v1";

#[derive(Serialize, Deserialize)]
pub struct BudgetItem {
    label: String,
    amount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Funding {
    title: String,
    application_url: String,
    summary: String,
    proposal: String,
    budget: Vec<BudgetItem>,
    budget_note: String,
    location: String,
    location_url: String,
    availability: String,
    workspace_costs: String,
    notes: String,
    milestones: String,
    reviewed: bool,
    submitted: bool,
    submitted_date: String,
}

impl Funding {
    pub fn load(storage: Option<&dyn eframe::Storage>) -> Result<Self, String> {
        let saved = storage.and_then(|s| s.get_string(KEY));
        serde_json::from_str(
            saved
                .as_deref()
                .unwrap_or(include_str!("../assets/funding.json")),
        )
        .map_err(|e| format!("Could not load funding workspace: {e}"))
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
                egui::CollapsingHeader::new("Location and remaining details").default_open(true).show(ui, |ui| {
                    edit(ui, "Planned build / test location", &mut self.location, 3);
                    ui.hyperlink_to("Fitzsimons shared-lab information ↗", &self.location_url);
                    ui.label("Published half- and full-bench options. Confirm identity, price, availability and permitted work before claiming access.");
                    edit(ui, "Weekly availability", &mut self.availability, 1);
                    edit(ui, "Workspace and working-time costs to include", &mut self.workspace_costs, 2);
                });
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
        assert!(f.location.contains("Prospective"));
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
    }
    #[test]
    fn corrupt_saved_work_is_not_silently_reset() {
        let mut s = Memory::default();
        s.set_string(KEY, "invalid json".into());
        assert!(Funding::load(Some(&s)).is_err());
    }
}
