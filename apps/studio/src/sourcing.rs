use eframe::egui::{self, RichText};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Sourcing {
    checked_on: String,
    follow_up_on: String,
    package: String,
    quotes: Vec<Quote>,
    items: Vec<Item>,
    open_items: Vec<String>,
}

#[derive(Deserialize)]
struct Quote {
    name: String,
    email: String,
    url: String,
    scope: String,
    message_id: String,
    thread_url: String,
    status: String,
}

#[derive(Deserialize)]
struct Item {
    name: String,
    quantity: u32,
    unit_cents: Option<u64>,
    unit: String,
    included: bool,
    url: String,
    note: String,
}

impl Sourcing {
    pub fn load() -> Result<Self, String> {
        serde_json::from_str(include_str!("../assets/sourcing.json"))
            .map_err(|e| format!("Could not load supplier research: {e}"))
    }

    fn catalog_subtotal(&self) -> u64 {
        self.items
            .iter()
            .filter(|i| i.included)
            .filter_map(|i| i.unit_cents.map(|price| price * u64::from(i.quantity)))
            .sum()
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Supplier quotes and equipment prices")
            .default_open(true).show(ui, |ui| {
                ui.label(RichText::new(format!("{} quote requests sent · research checked {}", self.quotes.len(), self.checked_on)).strong());
                ui.label(format!("Reply review date: {}. This snapshot does not monitor your inbox.", self.follow_up_on));
                ui.label(&self.package);
                ui.label(format!("Priced catalog subset: ${:.2} USD. This is not the complete prototype cost.", self.catalog_subtotal() as f64 / 100.0));
                ui.label("Catalog prices exclude shipping, taxes and tariffs. Unpriced items and optional candidates are excluded from this subtotal; the application budget remains provisional.");
                egui::CollapsingHeader::new("Sent quote requests").show(ui, |ui| {
                    for quote in &self.quotes {
                        ui.push_id(&quote.message_id, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.hyperlink_to(&quote.name, &quote.url);
                                ui.label(&quote.status);
                                ui.hyperlink_to("Open sent email ↗", &quote.thread_url);
                            });
                            ui.label(&quote.email);
                            ui.label(&quote.scope);
                            ui.separator();
                        });
                    }
                });
                egui::CollapsingHeader::new("Equipment shortlist and prices").show(ui, |ui| {
                    for item in &self.items {
                        ui.hyperlink_to(RichText::new(&item.name).strong(), &item.url);
                        let price = match item.unit_cents {
                            Some(cents) => format!("{} × ${:.2} / {} = ${:.2} USD{}", item.quantity, cents as f64 / 100.0, item.unit, cents as f64 * f64::from(item.quantity) / 100.0, if item.included { "" } else { " · candidate, excluded from subtotal" }),
                            None => format!("{} × {} · price pending", item.quantity, item.unit),
                        };
                        ui.label(price);
                        ui.label(&item.note);
                        ui.separator();
                    }
                });
                egui::CollapsingHeader::new("What still needs a price or confirmation").show(ui, |ui| {
                    for item in &self.open_items { ui.label(format!("• {item}")); }
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtotal_counts_purchase_packs_and_excludes_candidates_and_unknowns() {
        let s = Sourcing::load().unwrap();
        assert_eq!(s.catalog_subtotal(), 55437);
        assert_eq!(s.quotes.len(), 7);
        assert!(s.items.iter().any(|i| i.unit_cents.is_none()));
        assert!(s
            .items
            .iter()
            .any(|i| !i.included && i.unit_cents.is_some()));
        for q in &s.quotes {
            assert!(!q.message_id.is_empty());
            assert!(q.url.starts_with("https://"));
        }
    }
}
