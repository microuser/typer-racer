# Plan0: Typing Game Architecture Planning

## Project Summary

- **Purpose:** Fun typing game with real-time stats, visual effects (confetti, car, road), and a "ghost" player replaying previous successful runs.
- **Tech Stack:** Rust (core logic, integration tests, local deployment via dev/debug command, compiled to WebAssembly for browser execution).
- **Key Features:**
  - Words appear on the left; player types them.
  - Right side replays a "ghost" (recording of a previous run).
  - 60 FPS, fast response to input.
  - Visual effects: confetti, car, road, dynamic animations.
  - Road generated from a seed phrase (algorithmic/procedural generation).
  - End-of-game report: speed, stats.
  - **Runs in browser (WebGPU compatible, e.g., latest Chrome/Edge/Firefox).**
- **Non-Functional:** High performance, testable, local deployment.

## Clarifying Questions

1. **Frontend Rendering**
   - Do you want the UI/animations to be rendered in a browser (WebAssembly via Rust), or as a native desktop app (e.g., using egui or another Rust GUI framework)?
2. **Audio/Playback**
   - Will the "ghost" playback be audio, or just a visual replay of the previous typing session?
3. **Persistence**
   - Where/how should player runs be saved for replay? (Local file, in-memory, database?)
4. **Testing**
   - Should integration tests cover both game logic and rendering, or just the logic?
5. **Visuals**
   - Any specific libraries or frameworks for animations/graphics you prefer (e.g., Bevy, egui, macroquad)?
6. **Input**
   - Is multiplayer or online play in scope, or strictly single-player/local?

## Proposed Layered Architecture

1. **Presentation Layer (UI)**
   - Renders game state, animations, effects (confetti, car, road).
   - Handles user input (typing).
2. **Game Logic Layer**
   - Manages game state (words, car position, score, timing).
   - Processes input, updates stats, triggers effects.
   - Handles ghost playback (replay from recording).
3. **Persistence Layer**
   - Saves and loads ghost player data (recordings).
   - Stores player stats and reports.
4. **Procedural Generation Layer**
   - Generates roads from seed phrases.
   - Ensures reproducibility for ghost runs.
5. **Testing/Dev Layer**
   - Integration tests for game logic.
   - Debug/dev commands for local runs.

## Dataflow (Initial)

- On game start: Seed phrase → Road generated → Words placed.
- Player types: Input → Game logic updates state → UI renders changes → Effects triggered.
- Ghost: Loads prior run → Replays actions on right pane.
- On completion: Stats calculated → Report generated → Option to save run for ghost.

## Data Model

### Structure of JSON Meditation Quotes Database
```json
[
  {
    "original_quotes": ["", "", ""],
    "expanded_meditation": ""
  }
]
```
- `original_quotes`: Array of strings for quotes/words to type.
- `expanded_meditation`: Associated meditation/expanded content.

### Structure of JSON Recording/Replay
```json
[
  {
    "milliseconds_since_epoch_utc": 0,
    "quote_index": 0,
    "character": ""
  }
]
```
- `milliseconds_since_epoch_utc`: Timestamp of the character typed.
- `quote_index`: Index of the quote/word being typed.
- `character`: The character typed at that moment.

---

**Next Steps:**
- Answer clarifying questions above.
- Once clarified, proceed with detailed analysis, decision tree, and draft architecture.md.
