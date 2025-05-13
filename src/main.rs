mod keyboard;
mod ui;
mod quote_loader;
mod game;

use eframe::egui;
use serde::{Deserialize, Serialize};
use crate::keyboard::*;
use crate::ui::*;
use crate::quote_loader::load_expanded_meditations;
use crate::game::{MeditationQuote, ReplayEvent, GameState, GameStatus};

// Add TypingBuffer to TyperRacerApp
use crate::keyboard::input::TypingBuffer;

mod impl_typer_racer_app;

mod typer_racer_app;
pub use typer_racer_app::TyperRacerApp;

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

// --- Main App ---

mod app_init;

// --- WASM Timer Global ---
#[no_mangle]
pub static mut TYPER_RACER_ELAPSED: f32 = 0.0;

// For wasm32 (web) builds, use the following entrypoint
#[cfg(target_arch = "wasm32")]
pub fn main() {
    use wasm_bindgen::JsCast;
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("failed to initialize logger");

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(TyperRacerApp::default()))),
            )
            .await
            .expect("Failed to start eframe");
    });
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
        Box::new(|_cc| Ok(Box::new(TyperRacerApp::default()))),
    )
    .expect("Failed to start native app");
}
