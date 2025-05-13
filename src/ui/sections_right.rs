// --- Right Section UI Encapsulation ---
use eframe::egui;

#[derive(Default, Debug, Clone)]
pub struct RightSectionState {
    pub placeholder: String, // Extend with actual right section state as needed
}

impl RightSectionState {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Right Section");
            ui.label(&self.placeholder);
        });
    }
}
