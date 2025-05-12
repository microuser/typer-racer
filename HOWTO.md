# HOWTO: Set and Hit a Breakpoint in Typer Racer (Rust, VS Code)

This guide demonstrates how to set and hit a breakpoint in your Rust code using VS Code, with the CodeLLDB extension.

## Prerequisites
- VS Code installed
- [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) installed
- Project built successfully (`cargo build`)

## Step-by-Step Demo

1. **Open the Project in VS Code**
   - Open the root folder (`typer-racer`) in VS Code.

2. **Open `src/main.rs`**

3. **Insert a Breakpoint**
   - Find the line in your `update` function in `TyperRacerApp`:
     ```rust
     ui.label("Welcome! The typing race game will appear here.");
     ```
   - Click to the left of this line number to set a red breakpoint dot.

4. **(Optional) Add a Debug Print**
   - For demonstration, you can add:
     ```rust
     dbg!("Breakpoint hit!");
     ```
     just before the `ui.label` line, to see output in the debug console.

5. **Start Debugging**
   - Go to the Run and Debug panel in VS Code (left sidebar, play icon).
   - Select `Debug Typer Racer (native)` and click the green play button.

6. **Watch the Debugger Hit Your Breakpoint**
   - The debugger will start and pause execution at your breakpoint.
   - You can inspect variables, step through code, or continue execution.

## Example Code Snippet
```rust
impl eframe::App for TyperRacerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            dbg!("Breakpoint hit!"); // Add this for demo
            ui.heading("Typer Racer");
            ui.label("Welcome! The typing race game will appear here."); // Set breakpoint here
        });
    }
}
```

## Notes
- You can set breakpoints on any line of Rust code.
- For WebAssembly/browser debugging, use browser DevTools instead.

---
Happy debugging!


Here’s how I’ll proceed to address your requests:

1. Expand input logic for word progression, timing, and error handling
Advance to the next word when the current word is typed correctly.
Track timing from start to finish.
Increment error count on incorrect input.
2. Implement ghost replay logic
Record player keystrokes and timings.
On replay, “ghost” types according to the recorded events.
3. Add procedural road and car rendering (UI placeholder)
Display a simple visual placeholder for the road and car, using egui drawing primitives.
4. Add persistence for replay data
Save and load replay data using localStorage (for wasm) or file (for native).
5. Add integration tests
Add basic tests for core game logic (progression, error counting, replay).
I will begin by updating main.rs to handle word progression, timing, error handling, and basic ghost replay.
Then, I’ll add a placeholder for the road/car and basic persistence, followed by integration tests.

Let’s start with the main game logic and UI update.

Edited
main.rs
+171
-8
 133
Open diff
Here’s what I’ve just implemented based on your requests:

1. Expanded Input Logic
Progresses to the next word when the current word is typed correctly.
Tracks timing from start to finish.
Increments error count on incorrect input.
2. Ghost Replay Logic
Records player keystrokes and timings as replay events.
Can toggle ghost mode and display a simple ghost replay (shows ghost buffer up to the current time).
Supports saving/loading replays (using localStorage in wasm, file in native).
3. Procedural Road & Car Rendering (UI Placeholder)
Renders a simple “road” line and a “car” (circle) whose position reflects typing progress.
4. Persistence for Replay Data
Functions to save/load replays, supporting both web and native environments.
5. Integration Tests
Added basic tests for word progression and error counting in the core game logic.
You can now:

Play a basic round, see progress, errors, and elapsed time.
Save/load a replay and view a simple ghost.
See a placeholder road/car UI.
Run integration tests with cargo test.
Would you like to see a demo of a specific feature, polish the UI, or add more advanced game/replay logic next?


