use crate::{
    game::GameState,
    keyboard::{input::TypingBuffer, KeyboardState},
    ui::{sections_footer::FooterState, sections_top::TopSectionState, sections_left::PlayerViewState, sections_keyboard::KeyboardDisplayState},
};
use std::collections::HashMap;

use crate::TyperRacerApp;

impl Default for TyperRacerApp {
    fn default() -> Self {
        // Load meditation quotes from JSON
        let quotes = crate::load_expanded_meditations();
        let mut game = GameState::new(quotes);
        game.seed = "default-seed".to_string();

        // Initialize keyboard state with all keys
        let mut keyboard_state = HashMap::new();
        // Standard US QWERTY keyboard keys (now all uppercase)
        let keys = [
            "ESC", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
            "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "BACKSPACE",
            "TAB", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\\",
            "CAPSLOCK", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "ENTER",
            "SHIFT", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "RSHIFT",
            "CTRL", "WIN", "ALT", "SPACE", "RALT", "RWIN", "MENU", "RCTRL"
        ];
        for key in keys.iter() {
            keyboard_state.insert(key.to_string(), KeyboardState::default());
        }

        let mut top_section = TopSectionState::default();
        top_section.level_seed = game.seed.clone();

        Self {
            game,
            keyboard_state,
            top_section,
            player1_view: PlayerViewState::default(),
            player2_view: PlayerViewState::default(),
            text_input: crate::ui::sections_keyboard::TextInputState::default(),
            keyboard_display: KeyboardDisplayState::default(),
            footer: FooterState {
                current_mode: "Standard".to_string(),
                ..Default::default()
            },
            ui_scale: 1.0,
            dark_mode: true,
            show_ghost: true,
            show_keyboard: true,
            last_pressed_key: None,
            typing: TypingBuffer::default(),
        }
    }
}
