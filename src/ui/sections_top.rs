// --- Top Section UI Encapsulation ---
use eframe::egui;

#[derive(Default, Debug, Clone)]
pub struct TopSectionState {
    pub player1_wpm: f32,
    pub player2_wpm: f32,
    pub level_seed: String,
    pub race_progress_percent: f32,
}

use crate::keyboard::event::KeyboardEventHandler;

impl KeyboardEventHandler for TopSectionState {
    fn handle_keyboard_event(&mut self, _event: &crate::keyboard::event::KeyboardEvent) -> bool {
        // Top section does not handle keyboard events by default
        false
    }
}

impl TopSectionState {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Player 1 WPM: {:.1}", self.player1_wpm));
            ui.separator();
            ui.label(format!("Player 2 WPM: {:.1}", self.player2_wpm));
            ui.separator();
            ui.label(format!("Seed: {}", self.level_seed));
            ui.separator();
            ui.label(format!("Progress: {:.0}%", self.race_progress_percent * 100.0));
        });
    }
}
