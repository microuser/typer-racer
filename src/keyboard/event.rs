// Keyboard event abstraction and handler trait
use eframe::egui;
use eframe::egui::Key;

#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    KeyPress { key: String },
    KeyRelease { key: String },
}

pub fn egui_key_to_string(key: &Key) -> Option<String> {

    Some(match key {
        Key::Space => "SPACE",
        Key::Enter => "ENTER",
        Key::Tab => "TAB",
        Key::Backspace => "BACKSPACE",
        Key::Escape => "ESC",
        Key::Insert => "INSERT",
        Key::Delete => "DELETE",
        Key::ArrowLeft => "LEFT",
        Key::ArrowRight => "RIGHT",
        Key::ArrowUp => "UP",
        Key::ArrowDown => "DOWN",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
        _ => return None,
    }.to_string())
}

pub trait KeyboardEventHandler {
    fn handle_keyboard_event(&mut self, event: &KeyboardEvent) -> bool;
}
