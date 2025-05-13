// --- UI Section State and Rendering ---
#[derive(Default, Debug, Clone)]
pub struct TopSectionState {
    pub player1_wpm: f32,
    pub player2_wpm: f32,
    pub level_seed: String,
    pub race_progress_percent: f32,
}

#[derive(Default, Debug, Clone)]
pub struct PlayerViewState {
    pub car_position: f32,  // 0.0 to 1.0 for progress along the road
    pub speed: f32,         // Current typing speed
    pub errors: usize,      // Number of typing errors
    pub boosts: usize,      // Number of speed boosts earned
}

#[derive(Default, Debug, Clone)]
pub struct FooterState {
    pub wpm_history: Vec<f32>,
    pub accuracy: f32,
    pub current_mode: String,
}

use eframe::egui;

impl TopSectionState {
    pub fn render_top_section(&self, ui: &mut egui::Ui) {
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

impl PlayerViewState {
    pub fn render_left_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Left Section");
            ui.label(format!("Car Position: {:.2}", self.car_position));
            ui.label(format!("Speed: {:.1}", self.speed));
            ui.label(format!("Errors: {}", self.errors));
            ui.label(format!("Boosts: {}", self.boosts));
        });
    }
    pub fn render_right_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Right Section");
            ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ac.");
        });
    }
}

impl FooterState {
    pub fn render_footer_section(&self, ui: &mut egui::Ui) {
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
