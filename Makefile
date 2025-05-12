# Makefile for Typer Racer

# Default target: build everything and launch web
all: build test wasm web

# Build native Rust binaryuild:
	cargo build

# Run tests
test:
	cargo test

# Build WebAssembly package (using wasm-pack by default)
wasm:
	wasm-pack build --target web

# Launch local web server (using trunk)
web:
	trunk serve

# Clean build artifacts
clean:
	cargo clean
	wasm-pack clean || true

echo "Use 'make all' to build, test, wasm, and launch the web server."

# Setup instructions (not a make target):
# 1. Install Rust: https://rustup.rs/
# 2. Install wasm-pack: cargo install wasm-pack
# 3. Install trunk: cargo install trunk
# 4. Run 'make all' to start everything for web development.
