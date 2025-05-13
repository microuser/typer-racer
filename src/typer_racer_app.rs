use crate::ui::sections_top::TopSectionState;
use crate::ui::sections_left::PlayerViewState;
use crate::ui::sections_keyboard::TextInputState;
use crate::ui::sections_footer::FooterState;
use crate::keyboard::KeyboardState;
use crate::keyboard::KeyState;
use crate::keyboard::input::TypingBuffer;
use crate::keyboard::simulator::KeyboardSimulator;
use crate::game::GameState;

pub struct TyperRacerApp {
    pub game: GameState,
    pub keyboard_state: std::collections::HashMap<String, KeyState>,
    pub typing: TypingBuffer,
    pub top_section: TopSectionState,
    pub player1_view: PlayerViewState,
    pub player2_view: PlayerViewState,
    pub text_input: TextInputState,
    pub keyboard_display: KeyboardState,
    pub footer: FooterState,
    pub ui_scale: f32,
    pub dark_mode: bool,
    pub show_ghost: bool,
    pub show_keyboard: bool,
    pub last_pressed_key: Option<String>,
    pub keyboard_simulator: KeyboardSimulator,
}
