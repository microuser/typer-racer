use eframe::egui;

/// Draw a keyboard key with the given label, size, and pressed state
pub fn draw_key(ui: &mut egui::Ui, label: &str, size_factor: f32, key_size: f32, anim: f32) -> egui::Response {
    let key_width = key_size * size_factor;
    let key_height = key_size;
    // Animation: anim in [0,1], 1 means just pressed, 0 means idle
    let bg_color = if anim > 0.0 {
        let t = anim;
        egui::Color32::from_rgb(
            (60.0 + (100.0-60.0)*t) as u8,
            (60.0 + (180.0-60.0)*t) as u8,
            (70.0 + (255.0-70.0)*t) as u8,
        )
    } else {
        egui::Color32::from_rgb(60, 60, 70)
    };
    let text_color = if anim > 0.5 { egui::Color32::BLACK } else { egui::Color32::WHITE };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(key_width, key_height),
        egui::Sense::click()
    );
    ui.painter().rect_filled(rect, 4.0, bg_color);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color
    );
    response
}

/// FNV-1a hash for deterministic seed-to-number
pub fn fnv_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
