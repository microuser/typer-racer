// Renders the on-screen keyboard and animates key presses
use eframe::egui;

pub fn render_keyboard(ui: &mut egui::Ui, on_key: &mut dyn FnMut(&str)) {
    ui.vertical(|ui| {
        ui.heading("Keyboard");
        ui.label("[On-screen keyboard visualization goes here]");
        // TODO: Render real keyboard and handle clicks
    });
}
