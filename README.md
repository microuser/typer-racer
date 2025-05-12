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
