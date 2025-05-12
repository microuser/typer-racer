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
