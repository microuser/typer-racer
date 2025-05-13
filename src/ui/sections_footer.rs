// --- Footer Section UI Encapsulation ---
use eframe::egui;

use crate::keyboard::event::{KeyboardEvent, KeyboardEventHandler};

#[derive(Default, Debug, Clone)]
pub struct FooterSectionState {
    pub wpm_history: Vec<f32>,
    pub accuracy: f32,
    pub current_mode: String,
}

impl FooterSectionState {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Footer");
            ui.separator();
            ui.label(format!("Accuracy: {:.1}%", self.accuracy * 100.0));
            ui.separator();
            ui.label(format!("Mode: {}", self.current_mode));
            ui.separator();
            ui.label("Lorem ipsum dolor sit amet.");
        });
    }
}
