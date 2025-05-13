// Handles all keyboard input, buffer/cursor state, navigation, and editing logic

#[derive(Default, Debug, Clone)]
pub struct TypingBuffer {
    pub buffer: String,
    pub cursor: usize,
}

impl TypingBuffer {
    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
    }
    pub fn insert_str(&mut self, s: &str) {
        self.buffer.insert_str(self.cursor, s);
        self.cursor += s.len();
    }
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }
    pub fn set_buffer(&mut self, s: &str) {
        self.buffer = s.to_string();
        self.cursor = self.buffer.len();
    }
}
