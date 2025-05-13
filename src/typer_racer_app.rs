use crate::ui::sections_top::TopSectionState;
use crate::ui::sections_left::PlayerViewState;
use crate::ui::sections_keyboard::TextInputState;
use crate::sections::FooterState;
// use crate::keyboard::KeyboardState; (removed, not found)
// use crate::keyboard::KeyState; (removed, not found)
use crate::keyboard::input::TypingBuffer;
use crate::keyboard::simulator::KeyboardSimulator;
use crate::game::GameState;

pub struct TyperRacerApp {
    pub game: GameState,
    // pub keyboard_state: std::collections::HashMap<String, KeyState>, (removed)
    pub typing: TypingBuffer,
    pub top_section: TopSectionState,
    pub player1_view: PlayerViewState,
    pub player2_view: PlayerViewState,
    pub text_input: TextInputState,
    // pub keyboard_display: KeyboardState, (removed)
    pub footer: FooterState,
    pub ui_scale: f32,
    pub dark_mode: bool,
    pub show_ghost: bool,
    pub show_keyboard: bool,
    pub last_pressed_key: Option<String>,
    pub keyboard_simulator: KeyboardSimulator,
}




impl Default for TyperRacerApp {
    fn default() -> Self {
        TyperRacerApp {
            game: GameState::default(),
            typing: TypingBuffer::default(),
            top_section: TopSectionState::default(),
            player1_view: PlayerViewState::default(),
            player2_view: PlayerViewState::default(),
            text_input: TextInputState {
                current_quote: String::new(),
                current_position: 0,
                typed_text: String::new(),
            },
            footer: FooterState::default(),
            ui_scale: 1.0,
            dark_mode: false,
            show_ghost: false,
            show_keyboard: true,
            last_pressed_key: None,
            keyboard_simulator: KeyboardSimulator::new(),
        }
    }
}

