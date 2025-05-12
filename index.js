// Main entry point for Typer Racer WASM application
// Import the auto-generated JavaScript module from Trunk's output
// The path will be determined by Trunk at build time
import init, { get_elapsed_time } from './typer_racer.js';

// Global state
let wasmReady = false;

// Initialize the WASM module
async function initWasm() {
  try {
    await init();
    console.log("WASM module initialized successfully");
    
    // Update UI elements
    document.getElementById('loading').style.display = 'none';
    document.getElementById('score').style.display = 'block';
    document.getElementById('instructions').style.display = 'block';
    
    // Set up timer update
    wasmReady = true;
    setInterval(updateTimer, 100);
  } catch (e) {
    console.error("Failed to initialize WASM module:", e);
  }
}

// Update the timer display
function updateTimer() {
  if (wasmReady && typeof get_elapsed_time === 'function') {
    try {
      let t = get_elapsed_time();
      if (typeof t === 'number' && !isNaN(t)) {
        document.getElementById('timer_value').textContent = t.toFixed(2);
      }
    } catch (e) {
      console.warn("Error updating timer:", e);
    }
  }
}

// Start initialization
window.addEventListener('DOMContentLoaded', () => {
  initWasm();
});
