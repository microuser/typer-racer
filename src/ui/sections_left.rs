// --- Left Section UI Encapsulation ---
use eframe::egui;

#[derive(Default, Debug, Clone)]
pub struct LeftSectionState {
    pub car_position: f32,  // 0.0 to 1.0 for progress along the road
    pub speed: f32,         // Current typing speed
    pub errors: usize,      // Number of typing errors
    pub boosts: usize,      // Number of speed boosts earned
}

use crate::keyboard::event::KeyboardEventHandler;

// KeyboardEventHandler implementation for PlayerViewState
#[derive(Default, Debug, Clone)]
pub struct PlayerViewState {
    pub car_position: f32,  // 0.0 to 1.0 for progress along the road
    pub speed: f32,         // Current typing speed
    pub errors: usize,      // Number of typing errors
    pub boosts: usize,      // Number of speed boosts earned
}

impl KeyboardEventHandler for PlayerViewState {
    fn handle_keyboard_event(&mut self, _event: &crate::keyboard::event::KeyboardEvent) -> bool {
        // Player view does not handle keyboard events by default
        false
    }
}
impl LeftSectionState {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Left Section");
            ui.label(format!("Car Position: {:.2}", self.car_position));
            ui.label(format!("Speed: {:.1}", self.speed));
            ui.label(format!("Errors: {}", self.errors));
            ui.label(format!("Boosts: {}", self.boosts));
        });
    }
}
