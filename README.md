# Typer Racer: Endurance Typing Race Game

Ghost Typer is a head-to-head endurance typing game inspired by classic 2-player battles and racing games. The player races against a ghost opponent (a replay of a previous run or an AI) by typing quotes that appear as words on road signs, navigating a car through the course. The faster and more accurately you type, the faster your car moves!

## Features

- **Head-to-Head Racing**: Left side is the player, right side is the ghost (replay or AI).
- **Typing as Navigation**: Words from a quote are displayed as road signs. Type them in order to progress.
- **Speed Boosts**: Correctly typed words give speed boosts. Mistakes slow you down or stop the car.
- **Ghost Racer**: Replay your best runs or compete against an AI with adjustable speed.
- **Visuals**: 45-degree (isometric) overhead view showing the road, cars, and horizon. Large, clear, color-coded text.
- **Error Feedback**: Immediate, clear error indicators for mistyped words.
- **Replay System**: Records every key and timing for full run replays. Ghost racer is shown on the right.
- **Single Page Web Deployment**: Built for the web using Rust (WebAssembly), leveraging WebGPU for rendering.

## User Interface

The Typer Racer UI is designed for clarity, responsiveness, and a fun racing experience. The interface is organized into five main sections, following the Plan2.md specification:

### UI Layout (5 Sections)

1. **TOP Section**: Displays player WPM (words per minute), level seed, timer, and a race progress bar.
2. **LEFT Section**: Shows the player's car and road visualization, including progress and live feedback.
3. **RIGHT Section**: Mirrors the LEFT section for the ghost (AI or replay), including ghost car visualization and progress.
4. **MIDDLE Section**: Text display and input area. The current quote is shown with word highlighting, color-coded feedback for correct/incorrect typing, and a responsive input box.
5. **KEYBOARD Section**: An on-screen keyboard map that highlights pressed keys and provides visual feedback.
6. **FOOTER Section**: Stats, controls, and display options (e.g., restart, settings, mode selection).

### Design Principles & Features

- **Dark mode** by default for eye comfort
- **Monospaced font** for text clarity and alignment
- **Real-time feedback**: Progress bar, WPM counter, error highlighting, and visual car movement
- **Responsive layout** for different screen sizes
- **Immediate visual feedback** for typing accuracy and game events
- **Keyboard shortcuts** and accessibility features for efficient gameplay
- **Animations** and smooth transitions for engaging gameplay

The UI is implemented using Rust and egui, ensuring a unified codebase for both web (WASM) and native desktop builds.

## Requirements

- **Rust** (latest stable)
- **wasm-pack** or **trunk** for building to WebAssembly
- **WebGPU**-compatible browser
- **Frontend Framework**: [egui](https://github.com/emilk/egui) or [Yew](https://yew.rs/) for UI (preferably egui for simplicity)

## How to Play

1. Start the game in your browser.
2. Type the words as they appear on the road signs in order.
3. Correct typing keeps your car moving fast; mistakes slow you down.
4. Race against your own ghost or an AI to beat your best time!

## Development Plan

- [ ] Game core logic: quote loading, word progression, timing, error handling
- [ ] Car and road rendering (isometric view)
- [ ] Typing input and boost/slowdown mechanics
- [ ] Replay recording and playback (ghost racer)
- [ ] UI for stats, error feedback, and controls
- [ ] WebAssembly build and deployment

## Build & Run

1. Install Rust and wasm-pack (or trunk):
   ```sh
   cargo install wasm-pack
   # or
   cargo install trunk
   ```
2. Build and run locally:
   ```sh
   wasm-pack build --target web
   # or
   trunk serve
   ```
3. Open in your browser (WebGPU compatible, e.g., latest Chrome/Edge/Firefox).

## Structure of JSON Meditation Quotes Database 
  [ 
      {
      "original_quotes": [
        "",
        "",
        ""
      ],
      "expanded_meditation": ""
    }
  ]

## Structure of JSON Recording, Replay 
[
{
   milliseconds_since_epoch_utc:
   quote_index:
   character:
   }

]

## License
MIT

## Changelog

- 2025-05-12: Words appear on the left; player types them (core UI implemented)
- 2025-05-12: Right side replays a "ghost" (recording of a previous run)
- 2025-05-12: 60 FPS, fast response to input
- 2025-05-12: Visual effects: confetti, car, road, dynamic animations
- 2025-05-12: Road generated from a seed phrase (algorithmic/procedural generation)
- 2025-05-12: End-of-game report: speed, stats
- 2025-05-12: Runs in browser (WebGPU compatible, e.g., latest Chrome/Edge/Firefox)
- 2025-05-12: Clipboard functionality: Fixed by configuring RUSTFLAGS and updating Trunk.toml and Cargo.toml
- 2025-05-12: Trunk.toml: Created with correct RUSTFLAGS and dev server configuration
- 2025-05-12: Cargo.toml: Updated to include web-sys features for clipboard
- 2025-05-12: Web clipboard: Application now runs with clipboard functionality enabled in the browser
- 2025-05-12: Trunk serve: Application can be accessed at http://localhost:8080 with the dev server running
- 2025-05-12: Asset loading: Trunk setup fixes asset loading issues for the web target
