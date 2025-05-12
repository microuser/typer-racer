# Makefile for Typer Racer

# Default target: build everything and launch web
all: build test wasm web

# mall: clean, then build, test, wasm, web
mall: clean build test wasm web

# Build native Rust binary
build:
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
	@echo "Checking if port 8080 (trunk serve) is open..."
	@if lsof -i :8080 | grep LISTEN; then \
	  echo "Port 8080 is open. Stopping process..."; \
	  kill -9 $$(lsof -ti:8080); \
	  echo "Process on port 8080 stopped."; \
	else \
	  echo "Port 8080 is not open."; \
	fi
	cargo clean
	wasm-pack clean || true

# echo Use make all to build, test, wasm, and launch the web server.

# Setup instructions (not a make target):
# 1. Install Rust: https://rustup.rs/
# 2. Install wasm-pack: cargo install wasm-pack
# 3. Install trunk: cargo install trunk
# 4. Run make all to start everything for web development.
