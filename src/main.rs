use eframe::egui;
use serde::{Deserialize, Serialize};

// --- Data Models ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeditationQuote {
    pub original_quotes: Vec<String>,
    pub expanded_meditation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub milliseconds_since_epoch_utc: u64,
    pub quote_index: usize,
    pub character: char,
}

// --- Game State ---
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    NotStarted,
    Running,
    Finished,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::NotStarted
    }
}

#[derive(Default)]
pub struct GameState {
    pub quotes: Vec<MeditationQuote>,
    pub current_quote: usize,
    pub current_char: usize,
    pub input_buffer: String,
    pub status: GameStatus,
    pub errors: usize,
    pub start_time: Option<std::time::Instant>,
    pub elapsed: f32,
    pub replay: Vec<ReplayEvent>,
    pub ghost_mode: bool,
    pub seed: String,
}

impl GameState {
    pub fn new(quotes: Vec<MeditationQuote>) -> Self {
        Self {
            quotes,
            ..Default::default()
        }
    }
}

// --- Main App ---
pub struct TyperRacerApp {
    pub game: GameState,
}

impl Default for TyperRacerApp {
    fn default() -> Self {
        // Load meditation quotes from JSON
        let quotes = load_expanded_meditations();
        let mut game = GameState::new(quotes);
        game.seed = "default-seed".to_string();
        Self { game }
    }
}

// --- WASM Timer Global ---
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub static mut TYPER_RACER_ELAPSED: f32 = 0.0;

impl eframe::App for TyperRacerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Timing ---
        if self.game.status == GameStatus::Running {
            if let Some(start) = self.game.start_time {
                self.game.elapsed = start.elapsed().as_secs_f32();
            }
        }
        // --- Update WASM Timer Global ---
        #[cfg(target_arch = "wasm32")]
        unsafe {
            TYPER_RACER_ELAPSED = self.game.elapsed;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Typer Racer");
            // --- Level Selector (Seed Input) ---
            if self.game.status == GameStatus::NotStarted {
                ui.horizontal(|ui| {
                    ui.label("Level/Seed:");
                    ui.text_edit_singleline(&mut self.game.seed);
                });
            }
            // --- Procedural Road & Car Rendering (Wavy Placeholder) ---
            ui.group(|ui| {
                ui.label(format!("Procedural Road (seed: '{}')", self.game.seed));
                let (rect, _resp) = ui.allocate_exact_size(egui::vec2(300.0, 80.0), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                // Use the seed to create a simple deterministic wavy road
                let hash = fnv_hash(&self.game.seed);
                let mut points = Vec::new();
                let n_points = 20;
                for i in 0..=n_points {
                    let t = i as f32 / n_points as f32;
                    let x = rect.left() + t * rect.width();
                    // Use hash to vary amplitude and frequency
                    let amp = 20.0 + (hash % 13) as f32;
                    let freq = 2.0 + ((hash >> 3) % 3) as f32;
                    let y = rect.center().y + (amp * ((freq * t * std::f32::consts::PI).sin()));
                    points.push(egui::pos2(x, y));
                }
                for w in points.windows(2) {
                    painter.line_segment([w[0], w[1]], (4.0, egui::Color32::DARK_GRAY));
                }
                // Car position based on progress
                let progress = if let Some(quote) = self.game.quotes.get(self.game.current_quote) {
                    self.game.input_buffer.len() as f32 / quote.original_quotes[0].len().max(1) as f32
                } else { 0.0 };
                let car_idx = ((points.len() as f32 - 1.0) * progress).round() as usize;
                let car_idx = car_idx.min(points.len() - 1);
                painter.circle_filled(points[car_idx], 10.0, egui::Color32::LIGHT_BLUE);
            });


            // --- Player Panel ---
            ui.group(|ui| {
                ui.label("Player:");
                if let Some(quote) = self.game.quotes.get(self.game.current_quote) {
                    // Layout: quote and input aligned horizontally, monospace font
                    let quote_text = &quote.original_quotes[0];
                    let input_text = &self.game.input_buffer;
                    let char_count = quote_text.chars().count();
                    let font_id = egui::FontId::monospace(22.0);

                    // Calculate the width of each character block
                    let char_width = ui.fonts(|f| f.glyph_width(&font_id, 'W'));
                    let char_height = ui.fonts(|f| f.row_height(&font_id));

                    // Reserve space for the horizontal layout
                    let (rect, _resp) = ui.allocate_exact_size(
                        egui::vec2(char_width * char_count as f32, char_height * 2.0 + 10.0),
                        egui::Sense::focusable_noninteractive(),
                    );
                    let painter = ui.painter_at(rect);
                    let base_y = rect.top();
                    let quote_y = base_y;
                    let input_y = base_y + char_height + 6.0;

                    // Draw quote text (monospace, color-coded)
                    for (i, ch) in quote_text.chars().enumerate() {
                        let x = rect.left() + i as f32 * char_width;
                        let color = if i < input_text.len() {
                            if input_text.chars().nth(i) == Some(ch) {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::RED
                            }
                        } else {
                            egui::Color32::WHITE
                        };
                        painter.text(
                            egui::pos2(x, quote_y),
                            egui::Align2::LEFT_TOP,
                            ch,
                            font_id.clone(),
                            color,
                        );
                    }

                    // Draw input text (monospace, same kerning)
                    for (i, ch) in input_text.chars().enumerate() {
                        let x = rect.left() + i as f32 * char_width;
                        let color = if i < quote_text.len() && input_text.chars().nth(i) == quote_text.chars().nth(i) {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::WHITE
                        };
                        painter.text(
                            egui::pos2(x, input_y),
                            egui::Align2::LEFT_TOP,
                            ch,
                            font_id.clone(),
                            color,
                        );
                    }

                    // Draw vertical caret/indicator at current character
                    let caret_idx = input_text.chars().count().min(char_count);
                    let caret_x = rect.left() + caret_idx as f32 * char_width;
                    painter.line_segment(
                        [
                            egui::pos2(caret_x, quote_y - 2.0),
                            egui::pos2(caret_x, input_y + char_height + 2.0),
                        ],
                        (2.0, egui::Color32::YELLOW),
                    );

                    // Hidden text edit for input capture (no visible box)
                    let input_resp = egui::TextEdit::singleline(&mut self.game.input_buffer)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(char_width * char_count as f32)
                        .frame(false)
                        .show(ui);
                    if input_resp.response.changed() && self.game.status == GameStatus::Running {
                        let expected = &quote.original_quotes[0];
                        let input = &self.game.input_buffer;
                        let correct = expected.starts_with(input);
                        if !correct && !input.is_empty() {
                            self.game.errors += 1;
                        }
                        // Word complete
                        if input == expected {
                            // Record replay event
                            self.game.replay.push(ReplayEvent {
                                milliseconds_since_epoch_utc: chrono::Utc::now().timestamp_millis() as u64,
                                quote_index: self.game.current_quote,
                                character: '\n',
                            });
                            self.game.current_quote += 1;
                            self.game.input_buffer.clear();
                            if self.game.current_quote >= self.game.quotes.len() {
                                self.game.status = GameStatus::Finished;
                            }
                        } else if let Some(last_char) = input.chars().last() {
                            // Record replay event for each char
                            self.game.replay.push(ReplayEvent {
                                milliseconds_since_epoch_utc: chrono::Utc::now().timestamp_millis() as u64,
                                quote_index: self.game.current_quote,
                                character: last_char,
                            });
                        }
                    }
                }
            });

            // --- Ghost Replay Panel ---
            ui.separator();
            ui.group(|ui| {
                ui.label("Ghost (Replay):");
                if self.game.ghost_mode && !self.game.replay.is_empty() {
                    // Simple ghost: show replayed chars up to elapsed time
                    let mut ghost_buffer = String::new();
                    let now = chrono::Utc::now().timestamp_millis() as u64;
                    for event in &self.game.replay {
                        if event.milliseconds_since_epoch_utc <= now {
                            if event.character == '\n' {
                                break;
                            }
                            ghost_buffer.push(event.character);
                        }
                    }
                    ui.label(ghost_buffer);
                } else {
                    ui.label("[No replay loaded]");
                }
            });

            // --- Stats Panel ---
            ui.separator();
            ui.label(format!("Errors: {}", self.game.errors));
            ui.label(format!("Elapsed: {:.2}s", self.game.elapsed));
            // Controls
            if ui.button("Start Race").clicked() {
                // Use the seed to deterministically select a meditation
                let meditations = load_expanded_meditations();
                let hash = fnv_hash(&self.game.seed);
                let idx = (hash as usize) % meditations.len().max(1);
                let meditation = &meditations[idx].expanded_meditation;
                self.game.quotes = vec![MeditationQuote {
                    original_quotes: vec![meditation.clone()],
                    expanded_meditation: meditation.clone(),
                }];
                self.game.status = GameStatus::Running;
                self.game.start_time = Some(std::time::Instant::now());
                self.game.input_buffer.clear();
                self.game.errors = 0;
                self.game.current_quote = 0;
                self.game.replay.clear();
                // Optionally, you could re-generate the road or other procedural elements here
            }
            if ui.button("Reset").clicked() {
                self.game.status = GameStatus::NotStarted;
                self.game.input_buffer.clear();
                self.game.errors = 0;
                self.game.current_quote = 0;
                self.game.replay.clear();
            }
            if ui.button("Toggle Ghost Mode").clicked() {
                self.game.ghost_mode = !self.game.ghost_mode;
            }
            if ui.button("Save Replay").clicked() {
                save_replay(&self.game.replay);
            }
            if ui.button("Load Replay").clicked() {
                self.game.replay = load_replay();
                self.game.ghost_mode = true;
            }
        });
    }
}

// --- Persistence (localStorage for wasm, file for native) ---
#[cfg(target_arch = "wasm32")]
fn save_replay(replay: &[ReplayEvent]) {

    use web_sys::window;
    if let Ok(json) = serde_json::to_string(replay) {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("typer_racer_replay", &json);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_replay(replay: &[ReplayEvent]) {
    if let Ok(json) = serde_json::to_string(replay) {
        let _ = std::fs::write("replay.json", json);
    }
}

#[cfg(target_arch = "wasm32")]
fn load_replay() -> Vec<ReplayEvent> {

    use web_sys::window;
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(json)) = storage.get_item("typer_racer_replay") {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_replay() -> Vec<ReplayEvent> {
    if let Ok(json) = std::fs::read_to_string("replay.json") {
        serde_json::from_str(&json).unwrap_or_default()
    } else {
        Vec::new()
    }
}

// --- Integration Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_progression() {
        let quote = MeditationQuote {
            original_quotes: vec!["abc".to_string()],
            expanded_meditation: "".to_string(),
        };
        let mut game = GameState::new(vec![quote]);
        game.status = GameStatus::Running;
        game.input_buffer = "abc".to_string();
        assert_eq!(game.input_buffer, "abc");
    }
    #[test]
    fn test_error_counting() {
        let quote = MeditationQuote {
            original_quotes: vec!["abc".to_string()],
            expanded_meditation: "".to_string(),
        };
        let mut game = GameState::new(vec![quote]);
        game.status = GameStatus::Running;
        game.input_buffer = "abd".to_string();
        // Simulate error logic
        let expected = &game.quotes[0].original_quotes[0];
        let input = &game.input_buffer;
        let correct = expected.starts_with(input);
        if !correct && !input.is_empty() {
            game.errors += 1;
        }
        assert_eq!(game.errors, 1);
    }
}

// For wasm32 (web) builds, use the following entrypoint
#[cfg(target_arch = "wasm32")]
pub fn main() {
    use eframe::WebOptions;
    use eframe::web;
    wasm_bindgen_futures::spawn_local(async {
        web::start_web(
            "the_canvas_id",
            WebOptions::default(),
            Box::new(|_cc| Box::new(TyperRacerApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

// Utility: FNV-1a hash for deterministic seed-to-number
fn fnv_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// Load expanded meditations from JSON file (native only)
#[cfg(not(target_arch = "wasm32"))]
fn load_expanded_meditations() -> Vec<MeditationQuote> {
    let path = "level/expanded-meditations.json";
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let mut contents = String::new();
    use std::io::Read;
    if file.read_to_string(&mut contents).is_err() {
        return vec![];
    }
    let parsed: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(val) => val,
        Err(_) => return vec![],
    };
    let Some(array) = parsed.get("expanded_meditations").and_then(|v| v.as_array()) else {
        return vec![];
    };
    array.iter().filter_map(|item| {
        let meditation = item.get("expanded_meditation")?.as_str()?.to_string();
        Some(MeditationQuote {
            original_quotes: vec![meditation.clone()],
            expanded_meditation: meditation,
        })
    }).collect()
}

// For wasm: fallback to a default quote
#[cfg(target_arch = "wasm32")]
fn load_expanded_meditations() -> Vec<MeditationQuote> {
    vec![MeditationQuote {
        original_quotes: vec!["Practice makes perfect.".to_string()],
        expanded_meditation: "Focus on steady improvement.".to_string(),
    }]
}

// --- WASM Timer Export ---
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_elapsed_time() -> f32 {
    // Use the global singleton if available
    // This is a hack: in eframe/egui, the App is managed by the framework,
    // so we need a way to access the latest elapsed time.
    // We'll use a static mut with unsafe for demonstration (not thread safe, but ok for wasm single-thread)
    extern "C" {
        static mut TYPER_RACER_ELAPSED: f32;
    }
    unsafe { TYPER_RACER_ELAPSED }
}

// In the App update, set TYPER_RACER_ELAPSED = self.game.elapsed;


// Next steps:
// - Expand input logic to update game state and stats
// - Implement ghost replay logic
// - Add procedural road/car rendering (placeholder UI for now)
// - Add persistence for replay data
// - Add integration tests

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "Typer Racer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(TyperRacerApp::default())),
    ).expect("Failed to start native app");
}
