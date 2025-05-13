// Draws the target text, typed text, blinking cursor, and error highlighting
use eframe::egui;

pub fn render_typing_area(ui: &mut egui::Ui, target: &str, typed: &str, cursor: usize) {
    ui.label("Target Text:");
    ui.monospace(target);
    ui.add_space(10.0);
    ui.label("Typed Text:");
    // Draw typed text with cursor
    let (before, after) = typed.split_at(cursor.min(typed.len()));
    ui.horizontal(|ui| {
        ui.monospace(before);
        // Blinking cursor
        if (ui.input(|i| i.time) * 2.0).sin() > 0.0 {
            ui.monospace("|");
        }
        ui.monospace(after);
    });
}
