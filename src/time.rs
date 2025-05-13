// --- Time Handling for Cross-Platform ---

/// Returns the current time in seconds since the Unix epoch as f64, cross-platform.
#[cfg(target_arch = "wasm32")]
pub fn now_seconds() -> f64 {
    js_sys::Date::now() / 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_seconds() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs_f64()
}

#[cfg(not(target_arch = "wasm32"))]
pub type TimeInstant = std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct TimeInstant(pub f64); // Timestamp in milliseconds

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
pub struct TimeDuration(pub f32); // Duration in seconds

#[cfg(not(target_arch = "wasm32"))]
impl TimeDuration {
    pub fn as_secs_f32(&self) -> f32 {
        self.0
    }
}

#[cfg(target_arch = "wasm32")]
pub struct TimeDuration(pub f32); // Duration in seconds

#[cfg(target_arch = "wasm32")]
impl TimeDuration {
    pub fn as_secs_f32(&self) -> f32 {
        self.0
    }
}
