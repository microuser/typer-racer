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
    pub start_time: Option<TimeInstant>,
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

// --- Time Handling for Cross-Platform ---
#[cfg(not(target_arch = "wasm32"))]
pub type TimeInstant = std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct TimeInstant(f64); // Timestamp in milliseconds

#[cfg(target_arch = "wasm32")]
impl TimeInstant {
    pub fn now() -> Self {
        let window = web_sys::window().expect("no global window exists");
        let performance = window.performance().expect("performance object not available");
        Self(performance.now())
    }
    
    pub fn elapsed(&self) -> TimeDuration {
        let now = Self::now();
        TimeDuration(((now.0 - self.0) / 1000.0) as f32) // Convert ms to seconds
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct TimeDuration(f32); // Duration in seconds

#[cfg(not(target_arch = "wasm32"))]
impl TimeDuration {
    pub fn as_secs_f32(&self) -> f32 {
        self.0
    }
}

#[cfg(target_arch = "wasm32")]
pub struct TimeDuration(f32); // Duration in seconds

#[cfg(target_arch = "wasm32")]
impl TimeDuration {
    pub fn as_secs_f32(&self) -> f32 {
        self.0
    }
}

// --- Keyboard Visualization ---
#[derive(Default, Debug, Clone)]
pub struct KeyState {
    pub pressed: bool,
    pub last_press_time: Option<TimeInstant>,
}

// --- Main App ---
pub struct TyperRacerApp {
    pub game: GameState,
    pub keyboard_state: std::collections::HashMap<String, KeyState>,
}

impl Default for TyperRacerApp {
    fn default() -> Self {
        // Load meditation quotes from JSON
        let quotes = load_expanded_meditations();
        let mut game = GameState::new(quotes);
        game.seed = "default-seed".to_string();
        
        // Initialize keyboard state with all keys
        let mut keyboard_state = std::collections::HashMap::new();
        
        // Standard US QWERTY keyboard keys
        let keys = [
            // Row 1: Function keys and numbers
            "Esc", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace",
            // Row 2
            "Tab", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\",
            // Row 3
            "CapsLock", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "Enter",
            // Row 4
            "Shift", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/", "RShift",
            // Row 5
            "Ctrl", "Win", "Alt", "Space", "RAlt", "RWin", "Menu", "RCtrl",
            // Arrow keys
            "Up", "Down", "Left", "Right",
            // Navigation cluster
            "Insert", "Delete", "Home", "End", "PgUp", "PgDn",
        ];
        
        for key in keys.iter() {
            keyboard_state.insert(key.to_string(), KeyState::default());
        }
        
        Self { game, keyboard_state }
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
            if let Some(start) = &self.game.start_time {
                self.game.elapsed = start.elapsed().as_secs_f32();
            }
        }
        // --- Update WASM Timer Global ---
        #[cfg(target_arch = "wasm32")]
        unsafe {
            TYPER_RACER_ELAPSED = self.game.elapsed;
        }
        
        // --- Handle keyboard input ---
        // Reset all key states that have been pressed for more than 150ms
        let now = TimeInstant::now();
        for (_key, state) in self.keyboard_state.iter_mut() {
            if state.pressed {
                if let Some(press_time) = &state.last_press_time {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if now.duration_since(*press_time).as_millis() > 150 {
                            state.pressed = false;
                        }
                    }
                    
                    #[cfg(target_arch = "wasm32")]
                    {
                        let duration = press_time.elapsed();
                        if duration.as_secs_f32() * 1000.0 > 150.0 {
                            state.pressed = false;
                        }
                    }
                }
            }
        }
        
        // Process input events
        if let Some(event) = ctx.input(|i| i.events.last().cloned()) {
            if let egui::Event::Key { key, pressed, .. } = event {
                // Convert egui key to our keyboard key format
                let key_str = match key {
                    egui::Key::Space => "Space".to_string(),
                    egui::Key::Enter => "Enter".to_string(),
                    egui::Key::Tab => "Tab".to_string(),
                    egui::Key::Backspace => "Backspace".to_string(),
                    egui::Key::Escape => "Esc".to_string(),
                    egui::Key::Insert => "Insert".to_string(),
                    egui::Key::Delete => "Delete".to_string(),
                    egui::Key::Home => "Home".to_string(),
                    egui::Key::End => "End".to_string(),
                    egui::Key::PageUp => "PgUp".to_string(),
                    egui::Key::PageDown => "PgDn".to_string(),
                    egui::Key::ArrowLeft => "Left".to_string(),
                    egui::Key::ArrowRight => "Right".to_string(),
                    egui::Key::ArrowUp => "Up".to_string(),
                    egui::Key::ArrowDown => "Down".to_string(),
                    egui::Key::Num0 => "0".to_string(),
                    egui::Key::Num1 => "1".to_string(),
                    egui::Key::Num2 => "2".to_string(),
                    egui::Key::Num3 => "3".to_string(),
                    egui::Key::Num4 => "4".to_string(),
                    egui::Key::Num5 => "5".to_string(),
                    egui::Key::Num6 => "6".to_string(),
                    egui::Key::Num7 => "7".to_string(),
                    egui::Key::Num8 => "8".to_string(),
                    egui::Key::Num9 => "9".to_string(),
                    egui::Key::A => "a".to_string(),
                    egui::Key::B => "b".to_string(),
                    egui::Key::C => "c".to_string(),
                    egui::Key::D => "d".to_string(),
                    egui::Key::E => "e".to_string(),
                    egui::Key::F => "f".to_string(),
                    egui::Key::G => "g".to_string(),
                    egui::Key::H => "h".to_string(),
                    egui::Key::I => "i".to_string(),
                    egui::Key::J => "j".to_string(),
                    egui::Key::K => "k".to_string(),
                    egui::Key::L => "l".to_string(),
                    egui::Key::M => "m".to_string(),
                    egui::Key::N => "n".to_string(),
                    egui::Key::O => "o".to_string(),
                    egui::Key::P => "p".to_string(),
                    egui::Key::Q => "q".to_string(),
                    egui::Key::R => "r".to_string(),
                    egui::Key::S => "s".to_string(),
                    egui::Key::T => "t".to_string(),
                    egui::Key::U => "u".to_string(),
                    egui::Key::V => "v".to_string(),
                    egui::Key::W => "w".to_string(),
                    egui::Key::X => "x".to_string(),
                    egui::Key::Y => "y".to_string(),
                    egui::Key::Z => "z".to_string(),
                    _ => return, // Skip other keys
                };
                
                // Update key state
                if let Some(state) = self.keyboard_state.get_mut(&key_str) {
                    state.pressed = pressed;
                    if pressed {
                        state.last_press_time = Some(now);
                    }
                }
            }
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
            
            // --- Keyboard Visualization ---
            ui.add_space(20.0);
            ui.group(|ui| {
                ui.heading("Keyboard Visualization");
                ui.add_space(10.0);
                
                let _available_width = ui.available_width();
                let key_size = 40.0;
                let key_margin = 4.0;
                let key_spacing = key_size + key_margin;
                
                // Define keyboard rows with their keys and relative sizes
                let keyboard_layout = [
                    // Row 1: Function keys and numbers
                    vec![
                        ("Esc", 1.0), ("F1", 1.0), ("F2", 1.0), ("F3", 1.0), 
                        ("F4", 1.0), ("F5", 1.0), ("F6", 1.0), ("F7", 1.0), 
                        ("F8", 1.0), ("F9", 1.0), ("F10", 1.0), ("F11", 1.0), ("F12", 1.0)
                    ],
                    // Row 2: Numbers
                    vec![
                        ("`", 1.0), ("1", 1.0), ("2", 1.0), ("3", 1.0), ("4", 1.0), 
                        ("5", 1.0), ("6", 1.0), ("7", 1.0), ("8", 1.0), ("9", 1.0), 
                        ("0", 1.0), ("-", 1.0), ("=", 1.0), ("Backspace", 2.0)
                    ],
                    // Row 3: QWERTY
                    vec![
                        ("Tab", 1.5), ("q", 1.0), ("w", 1.0), ("e", 1.0), ("r", 1.0), 
                        ("t", 1.0), ("y", 1.0), ("u", 1.0), ("i", 1.0), ("o", 1.0), 
                        ("p", 1.0), ("[", 1.0), ("]", 1.0), ("\\", 1.5)
                    ],
                    // Row 4: ASDF
                    vec![
                        ("CapsLock", 1.75), ("a", 1.0), ("s", 1.0), ("d", 1.0), ("f", 1.0), 
                        ("g", 1.0), ("h", 1.0), ("j", 1.0), ("k", 1.0), ("l", 1.0), 
                        (";", 1.0), ("'", 1.0), ("Enter", 2.25)
                    ],
                    // Row 5: ZXCV
                    vec![
                        ("Shift", 2.25), ("z", 1.0), ("x", 1.0), ("c", 1.0), ("v", 1.0), 
                        ("b", 1.0), ("n", 1.0), ("m", 1.0), (",", 1.0), (".", 1.0), 
                        ("/", 1.0), ("RShift", 2.75)
                    ],
                    // Row 6: Bottom row
                    vec![
                        ("Ctrl", 1.25), ("Win", 1.25), ("Alt", 1.25), ("Space", 6.25), 
                        ("RAlt", 1.25), ("RWin", 1.25), ("Menu", 1.25), ("RCtrl", 1.25)
                    ],
                ];
                
                // Draw each row of the keyboard
                for row in keyboard_layout.iter() {
                    ui.horizontal(|ui| {
                        for (key, size_factor) in row.iter() {
                            let key_width = key_size * size_factor;
                            let key_height = key_size;
                            
                            // Get key state
                            let is_pressed = self.keyboard_state.get(*key)
                                .map(|state| state.pressed)
                                .unwrap_or(false);
                            
                            // Choose colors based on key state
                            let (bg_color, text_color) = if is_pressed {
                                (egui::Color32::from_rgb(100, 180, 255), egui::Color32::BLACK)
                            } else {
                                (egui::Color32::from_rgb(60, 60, 70), egui::Color32::WHITE)
                            };
                            
                            // Draw the key
                            let (rect, _response) = ui.allocate_exact_size(
                                egui::vec2(key_width, key_height),
                                egui::Sense::hover()
                            );
                            
                            ui.painter().rect_filled(rect, 4.0, bg_color);
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                *key,
                                egui::FontId::proportional(14.0),
                                text_color
                            );
                            
                            // Add spacing between keys
                            ui.add_space(key_margin);
                        }
                    });
                    ui.add_space(key_margin);
                }
                
                // Add arrow keys and navigation cluster
                ui.horizontal(|ui| {
                    // Navigation cluster
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            draw_key(ui, "Insert", 1.0, key_size, self.keyboard_state.get("Insert").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "Home", 1.0, key_size, self.keyboard_state.get("Home").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "PgUp", 1.0, key_size, self.keyboard_state.get("PgUp").map(|s| s.pressed).unwrap_or(false));
                        });
                        ui.add_space(key_margin);
                        ui.horizontal(|ui| {
                            draw_key(ui, "Delete", 1.0, key_size, self.keyboard_state.get("Delete").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "End", 1.0, key_size, self.keyboard_state.get("End").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "PgDn", 1.0, key_size, self.keyboard_state.get("PgDn").map(|s| s.pressed).unwrap_or(false));
                        });
                    });
                    
                    ui.add_space(key_spacing);
                    
                    // Arrow keys
                    ui.vertical(|ui| {
                        ui.add_space(key_spacing);
                        ui.horizontal(|ui| {
                            ui.add_space(key_spacing);
                            draw_key(ui, "Up", 1.0, key_size, self.keyboard_state.get("Up").map(|s| s.pressed).unwrap_or(false));
                        });
                        ui.horizontal(|ui| {
                            draw_key(ui, "Left", 1.0, key_size, self.keyboard_state.get("Left").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "Down", 1.0, key_size, self.keyboard_state.get("Down").map(|s| s.pressed).unwrap_or(false));
                            ui.add_space(key_margin);
                            draw_key(ui, "Right", 1.0, key_size, self.keyboard_state.get("Right").map(|s| s.pressed).unwrap_or(false));
                        });
                    });
                });
            });
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
                self.game.start_time = Some(TimeInstant::now());
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
    // This is the correct entry point for eframe 0.27 with wasm-bindgen
    // Set up panic hook and logger for better debugging in browser
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("failed to initialize logger");

    // eframe 0.27 uses a different approach for web initialization
    // The main entry point is handled by wasm-bindgen
    let web_options = eframe::WebOptions::default();
    
    // Use wasm_bindgen_futures to handle the async nature of web initialization
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // HTML Canvas ID
                web_options,
                Box::new(|_cc| Box::new(TyperRacerApp::default())),
            )
            .await
            .expect("Failed to start eframe");
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

// --- UI Helper Functions ---
/// Draw a keyboard key with the given label, size, and pressed state
fn draw_key(ui: &mut egui::Ui, label: &str, size_factor: f32, key_size: f32, is_pressed: bool) -> egui::Response {
    let key_width = key_size * size_factor;
    let key_height = key_size;
    
    // Choose colors based on key state
    let (bg_color, text_color) = if is_pressed {
        (egui::Color32::from_rgb(100, 180, 255), egui::Color32::BLACK)
    } else {
        (egui::Color32::from_rgb(60, 60, 70), egui::Color32::WHITE)
    };
    
    // Draw the key
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(key_width, key_height),
        egui::Sense::hover()
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
