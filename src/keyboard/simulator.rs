use crate::keyboard::event::KeyboardEvent;
use crate::ReplayEvent;

/// Simulates replaying keyboard events with timing.
pub struct KeyboardSimulator {
    pub events: Vec<ReplayEvent>,
    pub current_index: usize,
    pub start_time: Option<u64>, // milliseconds since epoch
    pub running: bool,
}

impl KeyboardSimulator {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            current_index: 0,
            start_time: None,
            running: false,
        }
    }

    /// Start a new replay
    pub fn start_replay(&mut self, events: Vec<ReplayEvent>, start_time: u64) {
        self.events = events;
        self.current_index = 0;
        self.start_time = Some(start_time);
        self.running = true;
    }

    /// Advance the simulation; returns Some(KeyboardEvent) if it's time to emit one
    pub fn tick(&mut self, now_ms: u64) -> Option<KeyboardEvent> {
        if !self.running || self.start_time.is_none() || self.current_index >= self.events.len() {
            return None;
        }
        let base_time = self.start_time.unwrap();
        while self.current_index < self.events.len() {
            let event = &self.events[self.current_index];
            if event.milliseconds_since_epoch_utc <= now_ms {
                // Convert ReplayEvent to KeyboardEvent (assuming character replay)
                let key_event = KeyboardEvent::KeyPress { key: event.character.to_string() };
                self.current_index += 1;
                return Some(key_event);
            } else {
                break;
            }
        }
        // Stop if done
        if self.current_index >= self.events.len() {
            self.running = false;
        }
        None
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
