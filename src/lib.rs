use wasm_bindgen::prelude::*;

// Import the TYPER_RACER_ELAPSED global from main.rs
extern "C" {
    #[link_name = "TYPER_RACER_ELAPSED"]
    static TYPER_RACER_ELAPSED: f32;
}

/// Returns the elapsed time for the typer racer game
/// This function is exported to JavaScript
#[wasm_bindgen]
pub fn get_elapsed_time() -> f32 {
    unsafe { TYPER_RACER_ELAPSED }
}

// Initialize the panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn start() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging to browser console
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    log::info!("Typer Racer WebAssembly module initialized");
}
