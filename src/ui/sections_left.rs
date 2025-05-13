// --- Left Section UI Encapsulation ---
use eframe::egui;

#[derive(Default, Debug, Clone)]
pub struct LeftSectionState {
    pub car_position: f32,  // 0.0 to 1.0 for progress along the road
    pub speed: f32,         // Current typing speed
    pub errors: usize,      // Number of typing errors
    pub boosts: usize,      // Number of speed boosts earned
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
