// --- Game State and Data Models ---
use serde::{Deserialize, Serialize};
use std::time::Instant;

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

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    NotStarted,
    Running,
    Finished,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Running
    }
}

#[derive(Default)]
pub struct GameState {
    pub quotes: Vec<MeditationQuote>,
    pub current_quote: usize,
    pub current_char: usize,
    pub input_buffer: String,
    pub cursor_pos: usize, // New: cursor position in input_buffer
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
