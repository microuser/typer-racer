use crate::TyperRacerApp;
use crate::keyboard::event::KeyboardEventHandler;
use eframe::egui;

impl eframe::App for TyperRacerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use crate::keyboard::simulator::KeyboardSimulator;
        use std::time::{SystemTime, UNIX_EPOCH};
        // ... (other code remains the same)

        // Process input events

        // --- Keyboard Replay Simulation ---
        let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        if let Some(event) = self.keyboard_simulator.tick(now_ms) {
            // For now, send replayed event to player1_view
            let _ = self.player1_view.handle_keyboard_event(&event);
        }
        // ... (rest of update logic from main.rs)
    }
}

impl TyperRacerApp {
    // Update UI state based on game state
    pub fn update_ui_state(&mut self) {
        // Update top section
        self.top_section.level_seed = self.game.seed.clone();
        // Calculate WPM if the game is running
        if self.game.status == crate::GameStatus::Running {
            if self.game.elapsed > 0.0 {
                // Words per minute calculation (assuming 5 chars per word)
                let chars_typed = self.game.current_char as f32;
                let minutes = self.game.elapsed / 60.0;
                if minutes > 0.0 {
                    self.top_section.player1_wpm = (chars_typed / 5.0) / minutes;
                }
            }
            // Update race progress percentage
            if !self.game.quotes.is_empty() && self.game.current_quote < self.game.quotes.len() {
                let quote = &self.game.quotes[self.game.current_quote];
                let total_chars = quote.expanded_meditation.len() as f32;
                if total_chars > 0.0 {
                    self.top_section.race_progress_percent = (self.game.current_char as f32 / total_chars) * 100.0;
                    // Update player view state
                    self.player1_view.car_position = self.game.current_char as f32 / total_chars;
                    self.player1_view.errors = self.game.errors;
                }
            }
        }
        // Update text input state
        if !self.game.quotes.is_empty() && self.game.current_quote < self.game.quotes.len() {
            let quote = &self.game.quotes[self.game.current_quote];
            self.text_input.current_quote = quote.expanded_meditation.clone();
            self.text_input.current_position = self.game.current_char;
            self.text_input.typed_text = self.typing.buffer.clone();
        }
        // Update footer
        if self.game.status == crate::GameStatus::Running && self.game.elapsed > 0.0 {
            // Calculate accuracy
            let total_chars = self.game.current_char + self.game.errors;
            if total_chars > 0 {
                self.footer.accuracy = (self.game.current_char as f32 / total_chars as f32) * 100.0;
            }
            // Add current WPM to history every second
            if self.game.elapsed as usize > self.footer.wpm_history.len() {
                self.footer.wpm_history.push(self.top_section.player1_wpm);
            }
        }
    }

}



impl TyperRacerApp {
}
