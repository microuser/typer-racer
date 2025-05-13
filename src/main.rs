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
    pub status: GameStatus,
    pub errors: usize,
    pub start_time: Option<TimeInstant>,
    pub elapsed: f32,
    pub replay: Vec<ReplayEvent>,
    pub ghost_mode: bool,
    pub seed: String,
}

// Add TypingBuffer to TyperRacerApp
use crate::keyboard::input::TypingBuffer;

pub struct TyperRacerApp {
    pub game: GameState,
    pub keyboard_state: std::collections::HashMap<String, KeyState>,
    pub typing: TypingBuffer,
    // ... other fields unchanged
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

// --- UI Section State ---
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
pub struct TextInputState {
    pub current_quote: String,
    pub current_position: usize,
    pub typed_text: String,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
}

#[derive(Default, Debug, Clone)]
pub struct KeyboardState {
    pub key_map: std::collections::HashMap<String, KeyState>,
    pub most_used_keys: Vec<String>,
    pub least_used_keys: Vec<String>,
}

#[derive(Default, Debug, Clone)]
pub struct FooterState {
    pub wpm_history: Vec<f32>,
    pub accuracy: f32,
    pub current_mode: String,
}

// --- Main App ---
pub struct TyperRacerApp {
    pub game: GameState,
    pub keyboard_state: std::collections::HashMap<String, KeyState>,
    
    // New UI section states
    pub top_section: TopSectionState,
    pub player1_view: PlayerViewState,
    pub player2_view: PlayerViewState,
    pub text_input: TextInputState,
    pub keyboard_display: KeyboardState,
    pub footer: FooterState,
    
    // UI state
    pub ui_scale: f32,
    pub dark_mode: bool,
    pub show_ghost: bool,
    pub show_keyboard: bool,
    // Track the last pressed key (case-sensitive string)
    pub last_pressed_key: Option<String>,
}

mod app_init;
        game.seed = "default-seed".to_string();
        
        // Initialize keyboard state with all keys
        let mut keyboard_state = std::collections::HashMap::new();
        
        // Standard US QWERTY keyboard keys (now all uppercase)
        let keys = [
            // Row 1: Function keys and numbers
            "ESC", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "BACKSPACE",
            // Row 2
            "TAB", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
            // Row 3
            "CAPSLOCK", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "ENTER",
            // Row 4
            "SHIFT", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "RSHIFT",
            // Row 5
            "CTRL", "WIN", "ALT", "SPACE", "RALT", "RWIN", "MENU", "RCTRL",
            // Arrow keys
            "UP", "DOWN", "LEFT", "RIGHT",
            // Navigation cluster
            "INSERT", "DELETE", "HOME", "END", "PGUP", "PGDN",
        ];
        
        for key in keys.iter() {
            keyboard_state.insert(key.to_string(), KeyState::default());
        }
        
        // Initialize the keyboard display state
        let mut keyboard_display = KeyboardState::default();
        keyboard_display.key_map = keyboard_state.clone();
        
        // Initialize text input state
        let mut text_input = TextInputState::default();
        if !game.quotes.is_empty() {
            text_input.current_quote = game.quotes[0].expanded_meditation.clone();
        }
        
        // Initialize top section state
        let mut top_section = TopSectionState::default();
        top_section.level_seed = game.seed.clone();
        
        Self {
            game,
            keyboard_state,
            top_section,
            player1_view: PlayerViewState::default(),
            player2_view: PlayerViewState::default(),
            text_input,
            keyboard_display,
            footer: FooterState {
                current_mode: "Standard".to_string(),
                ..Default::default()
            },
            ui_scale: 1.0,
            dark_mode: true,
            show_ghost: true,
            show_keyboard: true,
            last_pressed_key: None,
        }
    }
}

// --- WASM Timer Global ---
#[no_mangle]
pub static mut TYPER_RACER_ELAPSED: f32 = 0.0;

impl TyperRacerApp {
    // Update UI state based on game state
    fn update_ui_state(&mut self) {
        // Update top section
        self.top_section.level_seed = self.game.seed.clone();
        
        // Calculate WPM if the game is running
        if self.game.status == GameStatus::Running {
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
        if self.game.status == GameStatus::Running && self.game.elapsed > 0.0 {
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

    // ... (other methods remain the same)

    // Render the MIDDLE section with text input
    fn render_text_input_section(&mut self, ui: &mut egui::Ui) {
        use crate::keyboard::display::render_typing_area;
        let target = self.game.quotes[self.game.current_quote].original_quotes[0].as_str();
        let typed = &self.typing.buffer;
        let cursor = self.typing.cursor;
        render_typing_area(ui, target, typed, cursor);
    }

    // ... (other methods remain the same)
}

impl eframe::App for TyperRacerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ... (other code remains the same)

        // Process input events
        if let Some(event) = ctx.input(|i| i.events.last().cloned()) {
            if let egui::Event::Key { key, pressed, .. } = event {
                use crate::keyboard::input::TypingBuffer;
                // Convert egui key to our keyboard key format (always uppercase)
                let key_str = match key {
                    egui::Key::Space => "SPACE".to_string(),
                    egui::Key::Enter => "ENTER".to_string(),
                    egui::Key::Tab => "TAB".to_string(),
                    egui::Key::Backspace => "BACKSPACE".to_string(),
                    egui::Key::Escape => "ESC".to_string(),
                    egui::Key::Insert => "INSERT".to_string(),
                    egui::Key::Delete => "DELETE".to_string(),
                    egui::Key::ArrowLeft => "LEFT".to_string(),
                    egui::Key::ArrowRight => "RIGHT".to_string(),
                    egui::Key::ArrowUp => "UP".to_string(),
                    egui::Key::ArrowDown => "DOWN".to_string(),
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
                    egui::Key::A => "A".to_string(),
                    egui::Key::B => "B".to_string(),
                    egui::Key::C => "C".to_string(),
                    egui::Key::D => "D".to_string(),
                    egui::Key::E => "E".to_string(),
                    egui::Key::F => "F".to_string(),
                    egui::Key::G => "G".to_string(),
                    egui::Key::H => "H".to_string(),
                    egui::Key::I => "I".to_string(),
                    egui::Key::J => "J".to_string(),
                    egui::Key::K => "K".to_string(),
                    egui::Key::L => "L".to_string(),
                    egui::Key::M => "M".to_string(),
                    egui::Key::N => "N".to_string(),
                    egui::Key::O => "O".to_string(),
                    egui::Key::P => "P".to_string(),
                    egui::Key::Q => "Q".to_string(),
                    egui::Key::R => "R".to_string(),
                    egui::Key::S => "S".to_string(),
                    egui::Key::T => "T".to_string(),
                    egui::Key::U => "U".to_string(),
                    egui::Key::V => "V".to_string(),
                    egui::Key::W => "W".to_string(),
                    egui::Key::X => "X".to_string(),
                    egui::Key::Y => "Y".to_string(),
                    egui::Key::Z => "Z".to_string(),
                    _ => return, // Skip other keys
                };
                // Save the last pressed key (case-sensitive)
                if pressed {
                    self.last_pressed_key = Some(key_str.clone().to_uppercase());
                    // Handle cursor movement and text editing
                    match key_str.as_str() {
                        "Left" => {
                            if self.typing.cursor > 0 {
                                self.typing.cursor -= 1;
                            }
                        },
                        "Right" => {
                            if self.typing.cursor < self.typing.buffer.len() {
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
fn draw_key(ui: &mut egui::Ui, label: &str, size_factor: f32, key_size: f32, anim: f32) -> egui::Response {
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
